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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, Adt6, Adt6, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(u64, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt9, Adt10, Adt10, Adt11, u64, u64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt9 {
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
pub enum Adt10 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1(Adt13),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt7> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, v6) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt7::C0(f1(v2, v6)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt7::C0(Adt8::C1))
                    }
                    Some(v7) => {
                        let a5 = v1;
                        let a6 = 78u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt7::C0(f2(v2, v7, v4, v5, v6)?))
                        } else {
                            let a7 = v1;
                            let a8 = 23u64;
                            if if if nat_eq(a7, a8) {
                                true
                            } else {
                                let a9 = v1;
                                let a10 = 26u64;
                                nat_eq(a9, a10)
                            } {
                                true
                            } else {
                                let a11 = v1;
                                let a12 = 25u64;
                                nat_eq(a11, a12)
                            } {
                                Ok(Adt7::C0(Adt8::C5(64u64)))
                            } else {
                                let a13 = v1;
                                let a14 = 24u64;
                                if nat_eq(a13, a14) {
                                    Ok(Adt7::C0(f3(v2, v7, v6)?))
                                } else {
                                    let a15 = v1;
                                    let a16 = 27u64;
                                    if nat_eq(a15, a16) {
                                        Ok(Adt7::C0(f4(v2, v7, v6)?))
                                    } else {
                                        let a17 = v1;
                                        let a18 = 3u64;
                                        if nat_eq(a17, a18) {
                                            Ok(Adt7::C0(f5(v7, v4, v5, v6)?))
                                        } else {
                                            f6(v1, v2, v7, v4, v6)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v8, v9, v10, v11, v12) => {
            f7(v8, v9, v10, v11, v12)
        }
        Adt0::C2(v13) => {
            f8(v13)
        }
        Adt0::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) => {
            Ok(f9(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt8> {
    let v2: u64 = f10(v1.clone(), 1u64)?;
    let a20 = match v1.clone() {
        Adt2::C0(h19, _, _, _, _, _, _, _, _, _, _) => {
            h19
        }
    };
    let a21 = 0u64;
    if if if if if if if if nat_eq(a20, a21) {
        let a23 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h22, _, _, _, _, _) => {
                h22
            }
        };
        let a24 = 4u64;
        nat_eq(a23, a24)
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(h25, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        })
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _) => {
                h26
            }
        })
    } else {
        false
    } {
        let a28 = 0u64;
        let a29 = match v0.clone() {
            Adt1::C0(_, _, h27, _, _, _, _, _, _, _) => {
                h27
            }
        };
        nat_lt(a28, a29)
    } else {
        false
    } {
        let a31 = match v0.clone() {
            Adt1::C0(_, _, h30, _, _, _, _, _, _, _) => {
                h30
            }
        };
        let a32 = 2139095040u64;
        nat_lt(a31, a32)
    } else {
        false
    } {
        let a33 = v2;
        let a34 = 9223372036854775808u64;
        nat_lt(a33, a34)
    } else {
        false
    } {
        let a36 = f12(v2)?;
        let a37 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h35, _) => {
                h35
            }
        };
        nat_le(a36, a37)
    } else {
        false
    } {
        Ok(Adt8::C4(f13(v2)?, 4u64, f14(v1.clone(), 1u64), f14(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }, match v0.clone() {
            Adt1::C0(_, h39, _, _, _, _, _, _, _, _) => {
                h39
            }
        }, match v0 {
            Adt1::C0(_, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        }, v2)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m41 = v2;
    match m41 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v5) => {
            let m42 = v3;
            match m42 {
                None => {
                    Ok(Adt8::C1)
                }
                Some(v6) => {
                    let a44 = match v1.clone() {
                        Adt2::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                            h43
                        }
                    };
                    let a45 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a44, a45) {
                        let a47 = match v5.clone() {
                            Adt2::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                                h46
                            }
                        };
                        let a48 = 0u64;
                        nat_eq(a47, a48)
                    } else {
                        false
                    } {
                        let a50 = match v6.clone() {
                            Adt2::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
                                h49
                            }
                        };
                        let a51 = 0u64;
                        nat_eq(a50, a51)
                    } else {
                        false
                    } {
                        let a53 = match v4.clone() {
                            Adt2::C0(h52, _, _, _, _, _, _, _, _, _, _) => {
                                h52
                            }
                        };
                        let a54 = 0u64;
                        nat_eq(a53, a54)
                    } else {
                        false
                    } {
                        f15(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f15(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f16(v1.clone())?
                    } else {
                        false
                    } {
                        f16(v5.clone())?
                    } else {
                        false
                    } {
                        f16(v6)?
                    } else {
                        false
                    } {
                        f16(v4.clone())?
                    } else {
                        false
                    } {
                        let a60 = match v1.clone() {
                            Adt2::C0(_, h55, _, _, _, _, _, _, _, _, _) => {
                                h55
                            }
                        };
                        let a61 = {
                            let a58 = match v5.clone() {
                                Adt2::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
                                    h56
                                }
                            };
                            let a59 = match v5.clone() {
                                Adt2::C0(_, h57, _, _, _, _, _, _, _, _, _) => {
                                    h57
                                }
                            };
                            nat_mul(a58, a59)?
                        };
                        nat_eq(a60, a61)
                    } else {
                        false
                    } {
                        let a67 = match v1.clone() {
                            Adt2::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
                                h62
                            }
                        };
                        let a68 = {
                            let a65 = match v5.clone() {
                                Adt2::C0(_, _, h63, _, _, _, _, _, _, _, _) => {
                                    h63
                                }
                            };
                            let a66 = match v5.clone() {
                                Adt2::C0(_, _, _, h64, _, _, _, _, _, _, _) => {
                                    h64
                                }
                            };
                            nat_mul(a65, a66)?
                        };
                        nat_eq(a67, a68)
                    } else {
                        false
                    } {
                        let a71 = match v1.clone() {
                            Adt2::C0(_, _, _, h69, _, _, _, _, _, _, _) => {
                                h69
                            }
                        };
                        let a72 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h70, _, _, _, _, _, _) => {
                                h70
                            }
                        };
                        nat_eq(a71, a72)
                    } else {
                        false
                    } {
                        let a74 = match v0.clone() {
                            Adt1::C0(h73, _, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        };
                        let a75 = 4294967296u64;
                        nat_lt(a74, a75)
                    } else {
                        false
                    } {
                        let a77 = match v0.clone() {
                            Adt1::C0(h76, _, _, _, _, _, _, _, _, _) => {
                                h76
                            }
                        };
                        let a78 = 0u64;
                        if nat_eq(a77, a78) {
                            if f17(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a81 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h79) => {
                                        h79
                                    }
                                };
                                let a82 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h80) => {
                                        h80
                                    }
                                };
                                nat_eq(a81, a82)
                            }
                        } else {
                            let a85 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h83) => {
                                    h83
                                }
                            };
                            let a86 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h84) => {
                                    h84
                                }
                            };
                            nat_eq(a85, a86)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a88 = match v4.clone() {
                                Adt2::C0(_, h87, _, _, _, _, _, _, _, _, _) => {
                                    h87
                                }
                            };
                            let a89 = 0u64;
                            if if if nat_eq(a88, a89) {
                                true
                            } else {
                                let a91 = match v4.clone() {
                                    Adt2::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                                        h90
                                    }
                                };
                                let a92 = 0u64;
                                nat_eq(a91, a92)
                            } {
                                true
                            } else {
                                let a94 = match v4.clone() {
                                    Adt2::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
                                        h93
                                    }
                                };
                                let a95 = 0u64;
                                nat_eq(a94, a95)
                            } {
                                true
                            } else {
                                let a97 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h96, _, _, _, _, _, _) => {
                                        h96
                                    }
                                };
                                let a98 = 0u64;
                                nat_eq(a97, a98)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                                h99
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a101 = match v0 {
                                Adt1::C0(h100, _, _, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            };
                            let a102 = 0u64;
                            if nat_eq(a101, a102) {
                                f10(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f18(v8, match v5.clone() {
                                Adt2::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                                    h103
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                                    h104
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                                    h105
                                }
                            })?
                        };
                        Ok(Adt8::C4(f19(v9, v8, v10)?, 4u64, f14(v1, 1u64), f14(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt8::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
            h106
        }
    };
    let v4: u64 = f20(v3);
    let a108 = match v1.clone() {
        Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
            h107
        }
    };
    let a109 = 0u64;
    if if if if if if if if nat_eq(a108, a109) {
        true
    } else {
        let a111 = match v1.clone() {
            Adt2::C0(h110, _, _, _, _, _, _, _, _, _, _) => {
                h110
            }
        };
        let a112 = 30u64;
        nat_eq(a111, a112)
    } {
        let a115 = match v1.clone() {
            Adt2::C0(h113, _, _, _, _, _, _, _, _, _, _) => {
                h113
            }
        };
        let a116 = match v2.clone() {
            Adt2::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                h114
            }
        };
        nat_eq(a115, a116)
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f21(match v0.clone() {
            Adt1::C0(h117, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        })
    } else {
        false
    } {
        f22(v1.clone(), v4)
    } else {
        false
    } {
        f22(v2.clone(), v4)
    } else {
        false
    } {
        if f17(v1.clone(), v2.clone())? {
            true
        } else {
            f23(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f24(match v1.clone() {
            Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                h118
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h119, _, _, _, _, _, _, _, _) => {
                h119
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h120, _, _, _, _, _, _, _) => {
                h120
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h121, _, _, _, _, _, _) => {
                h121
            }
        }) {
            0u64
        } else {
            let a127 = match v1.clone() {
                Adt2::C0(_, _, h122, _, _, _, _, _, _, _, _) => {
                    h122
                }
            };
            let a128 = {
                let a125 = match v1.clone() {
                    Adt2::C0(_, _, _, h123, _, _, _, _, _, _, _) => {
                        h123
                    }
                };
                let a126 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                        h124
                    }
                };
                nat_mul(a125, a126)?
            };
            nat_mul(a127, a128)?
        };
        let v6: u64 = {
            let a129 = v5;
            let a130 = 0u64;
            if if nat_eq(a129, a130) {
                true
            } else {
                f25(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a132 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h131, _) => {
                        h131
                    }
                };
                let a133 = v4;
                let a134 = 0u64;
                nat_quot(a132, a133, a134)
            }
        };
        Ok(Adt8::C4(f26(v3, match v1.clone() {
            Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, v5, v6)?, v4, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, v5, f27(match v1 {
            Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a139 = match v1.clone() {
        Adt2::C0(h138, _, _, _, _, _, _, _, _, _, _) => {
            h138
        }
    };
    let a140 = 0u64;
    if if if if if nat_eq(a139, a140) {
        let a142 = match v2.clone() {
            Adt2::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                h141
            }
        };
        let a143 = 0u64;
        nat_eq(a142, a143)
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f28(match v0.clone() {
            Adt1::C0(h144, _, _, _, _, _, _, _, _, _) => {
                h144
            }
        })
    } else {
        false
    } {
        if f17(v1.clone(), v2.clone())? {
            true
        } else {
            f23(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f14(v1, 1u64);
        let v4: u64 = {
            let a150 = match v3.clone() {
                Adt3::C0(_, h145, _, _, _, _, _, _) => {
                    h145
                }
            };
            let a151 = {
                let a148 = match v3.clone() {
                    Adt3::C0(_, _, h146, _, _, _, _, _) => {
                        h146
                    }
                };
                let a149 = match v3.clone() {
                    Adt3::C0(_, _, _, h147, _, _, _, _) => {
                        h147
                    }
                };
                nat_mul(a148, a149)?
            };
            nat_mul(a150, a151)?
        };
        Ok(Adt8::C4(f29(match v3.clone() {
            Adt3::C0(h152, _, _, _, _, _, _, _) => {
                h152
            }
        }, v4)?, 4u64, v3, f14(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h153, _, _, _, _, _, _, _, _, _) => {
                h153
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m154 = v1;
    match m154 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let m155 = v2;
            match m155 {
                None => {
                    Ok(Adt8::C1)
                }
                Some(v5) => {
                    let a157 = match v0.clone() {
                        Adt2::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                            h156
                        }
                    };
                    let a158 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a157, a158) {
                        let a160 = match v4.clone() {
                            Adt2::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                                h159
                            }
                        };
                        let a161 = 0u64;
                        nat_eq(a160, a161)
                    } else {
                        false
                    } {
                        let a163 = match v5.clone() {
                            Adt2::C0(h162, _, _, _, _, _, _, _, _, _, _) => {
                                h162
                            }
                        };
                        let a164 = 26u64;
                        nat_eq(a163, a164)
                    } else {
                        false
                    } {
                        let a166 = match v3.clone() {
                            Adt2::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                                h165
                            }
                        };
                        let a167 = 0u64;
                        nat_eq(a166, a167)
                    } else {
                        false
                    } {
                        f15(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a170 = match v0.clone() {
                            Adt2::C0(_, h168, _, _, _, _, _, _, _, _, _) => {
                                h168
                            }
                        };
                        let a171 = match v4.clone() {
                            Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        nat_eq(a170, a171)
                    } else {
                        false
                    } {
                        let a174 = match v0.clone() {
                            Adt2::C0(_, _, h172, _, _, _, _, _, _, _, _) => {
                                h172
                            }
                        };
                        let a175 = match v5.clone() {
                            Adt2::C0(_, h173, _, _, _, _, _, _, _, _, _) => {
                                h173
                            }
                        };
                        nat_eq(a174, a175)
                    } else {
                        false
                    } {
                        let a178 = match v0.clone() {
                            Adt2::C0(_, _, _, h176, _, _, _, _, _, _, _) => {
                                h176
                            }
                        };
                        let a179 = match v5.clone() {
                            Adt2::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        nat_eq(a178, a179)
                    } else {
                        false
                    } {
                        let a181 = 0u64;
                        let a182 = match v5.clone() {
                            Adt2::C0(_, _, _, h180, _, _, _, _, _, _, _) => {
                                h180
                            }
                        };
                        nat_lt(a181, a182)
                    } else {
                        false
                    } {
                        let a184 = 0u64;
                        let a185 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h183, _, _, _, _, _, _) => {
                                h183
                            }
                        };
                        nat_lt(a184, a185)
                    } else {
                        false
                    } {
                        let a187 = 4u64;
                        let a188 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h186, _, _, _, _, _) => {
                                h186
                            }
                        };
                        nat_le(a187, a188)
                    } else {
                        false
                    } {
                        let a190 = 4u64;
                        let a191 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                                h189
                            }
                        };
                        nat_le(a190, a191)
                    } else {
                        false
                    } {
                        let a193 = 4u64;
                        let a194 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h192, _, _, _, _, _) => {
                                h192
                            }
                        };
                        nat_le(a193, a194)
                    } else {
                        false
                    } {
                        let a196 = 4u64;
                        let a197 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h195, _, _, _, _, _) => {
                                h195
                            }
                        };
                        nat_le(a196, a197)
                    } else {
                        false
                    } {
                        f17(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f17(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f17(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt8::C4(f30(match v0.clone() {
                            Adt2::C0(_, h198, _, _, _, _, _, _, _, _, _) => {
                                h198
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h199, _, _, _, _, _, _, _, _) => {
                                h199
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h200, _, _, _, _, _, _, _) => {
                                h200
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h201, _, _, _, _, _, _) => {
                                h201
                            }
                        })?, 4u64, f14(v0, 1u64), f14(v3, 1u64), Adt4::C22(f14(v4, 1u64), f14(v5, 1u64))))
                    } else {
                        Ok(Adt8::C1)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt7> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h202, _, _, _, _, _, _, _, _, _) => {
            h202
        }
    };
    Ok(Adt7::C0({
        let a203 = v0;
        let a204 = 51u64;
        if nat_eq(a203, a204) {
            f31(v1, v2, v3, v4)?
        } else {
            let a205 = v0;
            let a206 = 39u64;
            if nat_eq(a205, a206) {
                f32(v2, v3, v4)?
            } else {
                let a207 = v0;
                let a208 = 77u64;
                if nat_eq(a207, a208) {
                    f33(v2, v4)?
                } else {
                    let a209 = v0;
                    let a210 = 40u64;
                    if nat_eq(a209, a210) {
                        f34(v2, v3, v4)?
                    } else {
                        if f35(v0) {
                            f36(v0, v2, {
                                let m211 = v3;
                                match m211 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v4)?
                        } else {
                            let a212 = v0;
                            let a213 = 19u64;
                            if nat_eq(a212, a213) {
                                f37(v2, v4)?
                            } else {
                                let a214 = v0;
                                let a215 = 21u64;
                                if nat_eq(a214, a215) {
                                    f38(v5, v2, v3, v4)?
                                } else {
                                    let a216 = v0;
                                    let a217 = 61u64;
                                    if if if nat_eq(a216, a217) {
                                        true
                                    } else {
                                        let a218 = v0;
                                        let a219 = 62u64;
                                        nat_eq(a218, a219)
                                    } {
                                        true
                                    } else {
                                        let a220 = v0;
                                        let a221 = 63u64;
                                        nat_eq(a220, a221)
                                    } {
                                        f39(v0, v1, v2, v4)?
                                    } else {
                                        let a222 = v0;
                                        let a223 = 4u64;
                                        if if nat_eq(a222, a223) {
                                            true
                                        } else {
                                            let a224 = v0;
                                            let a225 = 32u64;
                                            nat_eq(a224, a225)
                                        } {
                                            f40(v0, v1, v2, v3, v4)?
                                        } else {
                                            let a226 = v0;
                                            let a227 = 20u64;
                                            if nat_eq(a226, a227) {
                                                f41(v2, v4)?
                                            } else {
                                                let a228 = v0;
                                                let a229 = 42u64;
                                                if nat_eq(a228, a229) {
                                                    f42(4u64, v2, v4)?
                                                } else {
                                                    let a230 = v0;
                                                    let a231 = 69u64;
                                                    if nat_eq(a230, a231) {
                                                        let a232 = v5;
                                                        let a233 = 4u64;
                                                        if nat_lt(a232, a233) {
                                                            f42(v5, v2, v4)?
                                                        } else {
                                                            Adt8::C1
                                                        }
                                                    } else {
                                                        let a234 = v0;
                                                        let a235 = 70u64;
                                                        if nat_eq(a234, a235) {
                                                            f43(v5, v2, v4)?
                                                        } else {
                                                            let a236 = v0;
                                                            let a237 = 75u64;
                                                            if if nat_eq(a236, a237) {
                                                                true
                                                            } else {
                                                                let a238 = v0;
                                                                let a239 = 76u64;
                                                                nat_eq(a238, a239)
                                                            } {
                                                                f44({
                                                                    let a240 = v0;
                                                                    let a241 = 76u64;
                                                                    nat_eq(a240, a241)
                                                                }, v1, v2, v4)?
                                                            } else {
                                                                let a242 = v0;
                                                                let a243 = 31u64;
                                                                if if if nat_eq(a242, a243) {
                                                                    true
                                                                } else {
                                                                    let a244 = v0;
                                                                    let a245 = 47u64;
                                                                    nat_eq(a244, a245)
                                                                } {
                                                                    true
                                                                } else {
                                                                    let a246 = v0;
                                                                    let a247 = 68u64;
                                                                    nat_eq(a246, a247)
                                                                } {
                                                                    f45(v0, v1, v2, v4)?
                                                                } else {
                                                                    let a248 = v0;
                                                                    let a249 = 15u64;
                                                                    if nat_eq(a248, a249) {
                                                                        f46(v2, v4)?
                                                                    } else {
                                                                        let a250 = v0;
                                                                        let a251 = 17u64;
                                                                        if nat_eq(a250, a251) {
                                                                            f47(v2, v4)?
                                                                        } else {
                                                                            let a252 = v0;
                                                                            let a253 = 18u64;
                                                                            if nat_eq(a252, a253) {
                                                                                f48(v2, v3, v4)?
                                                                            } else {
                                                                                let a254 = v0;
                                                                                let a255 = 13u64;
                                                                                if if if nat_eq(a254, a255) {
                                                                                    true
                                                                                } else {
                                                                                    let a256 = v0;
                                                                                    let a257 = 14u64;
                                                                                    nat_eq(a256, a257)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a258 = v0;
                                                                                    let a259 = 16u64;
                                                                                    nat_eq(a258, a259)
                                                                                } {
                                                                                    f49(v0, v2, v4)?
                                                                                } else {
                                                                                    let m260 = f50(v0, v5)?;
                                                                                    match m260 {
                                                                                        None => {
                                                                                            f51(v0, v5, v2, v3, v4)?
                                                                                        }
                                                                                        Some(v7) => {
                                                                                            f52(v7, v2, v4)?
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt7> {
    Ok(f53(v3.clone(), {
        let m261 = v3.clone();
        match m261 {
            Adt4::C0 => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f55(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f56(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f57(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f58(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v58, v59, v60, v61) => {
                f61(v0, v61, v58, v59, v60, v4)?
            }
            Adt4::C24(_) => {
                Adt7::C0(Adt8::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f54(v0, v1, v2, v3)?
            }
            Adt4::C26(v53, v54, v55, v56) => {
                let a262 = v53;
                let a263 = 64u64;
                if if if nat_eq(a262, a263) {
                    true
                } else {
                    let a264 = v53;
                    let a265 = 65u64;
                    nat_eq(a264, a265)
                } {
                    true
                } else {
                    let a266 = v53;
                    let a267 = 66u64;
                    nat_eq(a266, a267)
                } {
                    Adt7::C0(Adt8::C5(64u64))
                } else {
                    let a268 = v53;
                    let a269 = 0u64;
                    if if nat_eq(a268, a269) {
                        true
                    } else {
                        let a270 = v53;
                        let a271 = 30u64;
                        nat_eq(a270, a271)
                    } {
                        f60(v0, v1, v2, v53, v54, v55, v56, v4)?
                    } else {
                        Adt7::C0(Adt8::C1)
                    }
                }
            }
            Adt4::C27(v52) => {
                f59(v0, v1, v2, v52)?
            }
        }
    }))
}

pub fn f8(v0: Adt4) -> R<Adt7> {
    let m272 = v0;
    match m272 {
        Adt4::C0 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt7::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt7::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt7::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt7::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt7::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt7::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v45, _, v47) => {
            Ok(Adt7::C6({
                let a277 = {
                    let a273 = 0u64;
                    let a274 = v45;
                    if nat_lt(a273, a274) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a278 = {
                    let a275 = 0u64;
                    let a276 = v47;
                    if nat_lt(a275, a276) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a277, a278)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v49, _, _, _) => {
            Ok(Adt7::C6({
                let a279 = v49;
                let a280 = 66u64;
                if nat_eq(a279, a280) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false))
        }
        Adt4::C27(_) => {
            Ok(Adt7::C6(2u64, 0u64, 0u64, false))
        }
    }
}

pub fn f9(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt7 {
    let a281 = v1;
    let a282 = 5u64;
    if if if if nat_lt(a281, a282) {
        let a283 = v5;
        let a284 = 5u64;
        nat_lt(a283, a284)
    } else {
        false
    } {
        let a285 = v7;
        let a286 = 5u64;
        nat_lt(a285, a286)
    } else {
        false
    } {
        let a287 = v9;
        let a288 = 2u64;
        if nat_lt(a287, a288) {
            true
        } else {
            let a289 = v9;
            let a290 = 3u64;
            nat_eq(a289, a290)
        }
    } else {
        false
    } {
        Adt7::C7(f62(v0, v1, v2, v3, v4), f63(v5, v6), f63(v7, v8), f64({
            let a291 = v9;
            let a292 = 0u64;
            if nat_eq(a291, a292) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt7::C0(Adt8::C1)
    }
}

pub fn f10(v0: Adt2, v1: u64) -> R<u64> {
    let a304 = {
        let a298 = {
            let a294 = match v0.clone() {
                Adt2::C0(_, h293, _, _, _, _, _, _, _, _, _) => {
                    h293
                }
            };
            let a295 = v1;
            let a296 = 0u64;
            nat_quot(a294, a295, a296)
        };
        let a299 = match v0.clone() {
            Adt2::C0(_, _, h297, _, _, _, _, _, _, _, _) => {
                h297
            }
        };
        nat_mul(a298, a299)?
    };
    let a305 = {
        let a302 = match v0.clone() {
            Adt2::C0(_, _, _, h300, _, _, _, _, _, _, _) => {
                h300
            }
        };
        let a303 = match v0 {
            Adt2::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                h301
            }
        };
        nat_mul(a302, a303)?
    };
    nat_mul(a304, a305)
}

pub fn f11(v0: u64) -> bool {
    let a306 = v0;
    let a307 = 4294967296u64;
    if nat_lt(a306, a307) {
        let a311 = {
            let a308 = v0;
            let a309 = 2147483648u64;
            let a310 = 0u64;
            nat_rem(a308, a309, a310)
        };
        let a312 = 2139095040u64;
        nat_lt(a311, a312)
    } else {
        false
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a313 = 4u64;
    let a314 = v0;
    nat_mul(a313, a314)
}

pub fn f13(v0: u64) -> R<u64> {
    let a315 = v0;
    let a316 = 0u64;
    if nat_eq(a315, a316) {
        Ok(0u64)
    } else {
        let a319 = 4u64;
        let a320 = {
            let a317 = 3u64;
            let a318 = v0;
            nat_mul(a317, a318)?
        };
        nat_add(a319, a320)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a322 = match v0.clone() {
            Adt2::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
                h321
            }
        };
        let a323 = v1;
        let a324 = 0u64;
        nat_quot(a322, a323, a324)
    }, match v0.clone() {
        Adt2::C0(_, _, h325, _, _, _, _, _, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h326, _, _, _, _, _, _, _) => {
            h326
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h327, _, _, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h328, _, _, _, _, _) => {
            h328
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h329, _, _, _, _) => {
            h329
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h330, _, _, _) => {
            h330
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h331, _, _) => {
            h331
        }
    })
}

pub fn f15(v0: Adt2, v1: Adt2) -> bool {
    let a334 = match v0.clone() {
        Adt2::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
            h332
        }
    };
    let a335 = match v1.clone() {
        Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
            h333
        }
    };
    if if if nat_eq(a334, a335) {
        let a338 = match v0.clone() {
            Adt2::C0(_, _, h336, _, _, _, _, _, _, _, _) => {
                h336
            }
        };
        let a339 = match v1.clone() {
            Adt2::C0(_, _, h337, _, _, _, _, _, _, _, _) => {
                h337
            }
        };
        nat_eq(a338, a339)
    } else {
        false
    } {
        let a342 = match v0.clone() {
            Adt2::C0(_, _, _, h340, _, _, _, _, _, _, _) => {
                h340
            }
        };
        let a343 = match v1.clone() {
            Adt2::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                h341
            }
        };
        nat_eq(a342, a343)
    } else {
        false
    } {
        let a346 = match v0 {
            Adt2::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                h344
            }
        };
        let a347 = match v1 {
            Adt2::C0(_, _, _, _, h345, _, _, _, _, _, _) => {
                h345
            }
        };
        nat_eq(a346, a347)
    } else {
        false
    }
}

pub fn f16(v0: Adt2) -> R<bool> {
    let a349 = match v0.clone() {
        Adt2::C0(_, h348, _, _, _, _, _, _, _, _, _) => {
            h348
        }
    };
    let a350 = 1u64;
    if if if if nat_eq(a349, a350) {
        true
    } else {
        let a352 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h351, _, _, _, _, _) => {
                h351
            }
        };
        let a353 = 4u64;
        nat_eq(a352, a353)
    } {
        let a355 = match v0.clone() {
            Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                h354
            }
        };
        let a356 = 1u64;
        if nat_eq(a355, a356) {
            true
        } else {
            let a361 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h357, _, _, _, _) => {
                    h357
                }
            };
            let a362 = {
                let a359 = match v0.clone() {
                    Adt2::C0(_, h358, _, _, _, _, _, _, _, _, _) => {
                        h358
                    }
                };
                let a360 = 4u64;
                nat_mul(a359, a360)?
            };
            nat_eq(a361, a362)
        }
    } else {
        false
    } {
        let a364 = match v0.clone() {
            Adt2::C0(_, _, _, h363, _, _, _, _, _, _, _) => {
                h363
            }
        };
        let a365 = 1u64;
        if nat_eq(a364, a365) {
            true
        } else {
            let a373 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h366, _, _, _) => {
                    h366
                }
            };
            let a374 = {
                let a371 = {
                    let a369 = match v0.clone() {
                        Adt2::C0(_, h367, _, _, _, _, _, _, _, _, _) => {
                            h367
                        }
                    };
                    let a370 = match v0.clone() {
                        Adt2::C0(_, _, h368, _, _, _, _, _, _, _, _) => {
                            h368
                        }
                    };
                    nat_mul(a369, a370)?
                };
                let a372 = 4u64;
                nat_mul(a371, a372)?
            };
            nat_eq(a373, a374)
        }
    } else {
        false
    } {
        let a376 = match v0.clone() {
            Adt2::C0(_, _, _, _, h375, _, _, _, _, _, _) => {
                h375
            }
        };
        let a377 = 1u64;
        if nat_eq(a376, a377) {
            Ok(true)
        } else {
            let a388 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h378, _, _) => {
                    h378
                }
            };
            let a389 = {
                let a386 = {
                    let a384 = {
                        let a381 = match v0.clone() {
                            Adt2::C0(_, h379, _, _, _, _, _, _, _, _, _) => {
                                h379
                            }
                        };
                        let a382 = match v0.clone() {
                            Adt2::C0(_, _, h380, _, _, _, _, _, _, _, _) => {
                                h380
                            }
                        };
                        nat_mul(a381, a382)?
                    };
                    let a385 = match v0 {
                        Adt2::C0(_, _, _, h383, _, _, _, _, _, _, _) => {
                            h383
                        }
                    };
                    nat_mul(a384, a385)?
                };
                let a387 = 4u64;
                nat_mul(a386, a387)?
            };
            Ok(nat_eq(a388, a389))
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: Adt2, v1: Adt2) -> R<bool> {
    let a391 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h390, _) => {
            h390
        }
    };
    let a392 = 0u64;
    if nat_eq(a391, a392) {
        Ok(true)
    } else {
        let a394 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h393, _) => {
                h393
            }
        };
        let a395 = 0u64;
        if nat_eq(a394, a395) {
            Ok(true)
        } else {
            let a397 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h396) => {
                    h396
                }
            };
            let a398 = 0u64;
            if nat_eq(a397, a398) {
                Ok(true)
            } else {
                let a400 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h399) => {
                        h399
                    }
                };
                let a401 = 0u64;
                if nat_eq(a400, a401) {
                    Ok(true)
                } else {
                    let a407 = {
                        let a404 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h402) => {
                                h402
                            }
                        };
                        let a405 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h403, _) => {
                                h403
                            }
                        };
                        nat_add(a404, a405)?
                    };
                    let a408 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h406) => {
                            h406
                        }
                    };
                    if nat_le(a407, a408) {
                        Ok(true)
                    } else {
                        let a414 = {
                            let a411 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h409) => {
                                    h409
                                }
                            };
                            let a412 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h410, _) => {
                                    h410
                                }
                            };
                            nat_add(a411, a412)?
                        };
                        let a415 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h413) => {
                                h413
                            }
                        };
                        Ok(nat_le(a414, a415))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a420 = {
        let a416 = v0;
        let a417 = v1;
        nat_mul(a416, a417)?
    };
    let a421 = {
        let a418 = v2;
        let a419 = f65(v3);
        nat_mul(a418, a419)?
    };
    nat_mul(a420, a421)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a424 = v0;
    let a425 = {
        let a422 = f66(v1)?;
        let a423 = v2;
        nat_mul(a422, a423)?
    };
    nat_add(a424, a425)
}

pub fn f20(v0: u64) -> u64 {
    let a426 = v0;
    let a427 = 30u64;
    if nat_eq(a426, a427) {
        2u64
    } else {
        4u64
    }
}

pub fn f21(v0: u64) -> bool {
    let a428 = v0;
    let a429 = 2139095040u64;
    if nat_le(a428, a429) {
        true
    } else {
        let a430 = v0;
        let a431 = 2147483648u64;
        nat_eq(a430, a431)
    }
}

pub fn f22(v0: Adt2, v1: u64) -> bool {
    let a433 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h432, _, _, _, _, _) => {
            h432
        }
    };
    let a434 = v1;
    if if nat_eq(a433, a434) {
        let a439 = {
            let a436 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h435) => {
                    h435
                }
            };
            let a437 = v1;
            let a438 = 0u64;
            nat_rem(a436, a437, a438)
        };
        let a440 = 0u64;
        nat_eq(a439, a440)
    } else {
        false
    } {
        f67(match v0.clone() {
            Adt2::C0(_, _, h441, _, _, _, _, _, _, _, _) => {
                h441
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h442, _, _, _, _, _, _, _) => {
                h442
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h443, _, _, _, _, _, _) => {
                h443
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h444, _, _, _, _) => {
                h444
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h445, _, _, _) => {
                h445
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h446, _, _) => {
                h446
            }
        }, v1)
    } else {
        false
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a449 = match v0.clone() {
            Adt2::C0(h447, _, _, _, _, _, _, _, _, _, _) => {
                h447
            }
        };
        let a450 = match v1.clone() {
            Adt2::C0(h448, _, _, _, _, _, _, _, _, _, _) => {
                h448
            }
        };
        nat_eq(a449, a450)
    } else {
        false
    } {
        let a453 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h451) => {
                h451
            }
        };
        let a454 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h452) => {
                h452
            }
        };
        nat_eq(a453, a454)
    } else {
        false
    } {
        let a457 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h455, _, _, _, _, _) => {
                h455
            }
        };
        let a458 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h456, _, _, _, _, _) => {
                h456
            }
        };
        nat_eq(a457, a458)
    } else {
        false
    } {
        let a461 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h459, _, _, _, _) => {
                h459
            }
        };
        let a462 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h460, _, _, _, _) => {
                h460
            }
        };
        nat_eq(a461, a462)
    } else {
        false
    } {
        let a465 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h463, _, _, _) => {
                h463
            }
        };
        let a466 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h464, _, _, _) => {
                h464
            }
        };
        nat_eq(a465, a466)
    } else {
        false
    } {
        let a469 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h467, _, _) => {
                h467
            }
        };
        let a470 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h468, _, _) => {
                h468
            }
        };
        nat_eq(a469, a470)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a471 = v0;
    let a472 = 0u64;
    if if if nat_eq(a471, a472) {
        true
    } else {
        let a473 = v1;
        let a474 = 0u64;
        nat_eq(a473, a474)
    } {
        true
    } else {
        let a475 = v2;
        let a476 = 0u64;
        nat_eq(a475, a476)
    } {
        true
    } else {
        let a477 = v3;
        let a478 = 0u64;
        nat_eq(a477, a478)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> R<bool> {
    f68({
        let a480 = match v0.clone() {
            Adt2::C0(_, h479, _, _, _, _, _, _, _, _, _) => {
                h479
            }
        };
        let a481 = v1;
        nat_mul(a480, a481)?
    }, match v0.clone() {
        Adt2::C0(_, _, h482, _, _, _, _, _, _, _, _) => {
            h482
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h483, _, _, _, _, _, _, _) => {
            h483
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h484, _, _, _, _, _, _) => {
            h484
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h485, _, _, _, _) => {
            h485
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h486, _, _, _) => {
            h486
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h487, _, _) => {
            h487
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a488 = v1;
    let a489 = 0u64;
    if if nat_eq(a488, a489) {
        true
    } else {
        let a490 = v2;
        let a491 = 0u64;
        nat_eq(a490, a491)
    } {
        Ok(0u64)
    } else {
        let a498 = {
            let a492 = f69(v1, v2, v3)?;
            let a493 = f70(v0, v1, v2)?;
            nat_add(a492, a493)?
        };
        let a499 = {
            let a496 = f71(v0);
            let a497 = {
                let a494 = v1;
                let a495 = v2;
                nat_mul(a494, a495)?
            };
            nat_mul(a496, a497)?
        };
        nat_add(a498, a499)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a500 = v0;
    let a501 = 0u64;
    if if nat_eq(a500, a501) {
        true
    } else {
        let a502 = v1;
        let a503 = 0u64;
        nat_eq(a502, a503)
    } {
        Ok(3u64)
    } else {
        let a506 = {
            let a504 = 3u64;
            let a505 = v1;
            nat_add(a504, a505)?
        };
        let a507 = v2;
        nat_add(a506, a507)
    }
}

pub fn f28(v0: u64) -> bool {
    let a508 = v0;
    let a509 = 2139095040u64;
    if nat_le(a508, a509) {
        true
    } else {
        let a510 = v0;
        let a511 = 2147483648u64;
        nat_eq(a510, a511)
    }
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a512 = v0;
    let a513 = 0u64;
    if nat_eq(a512, a513) {
        Ok(0u64)
    } else {
        let a514 = v1;
        let a515 = f72(v0)?;
        nat_mul(a514, a515)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a516 = v0;
    let a517 = 0u64;
    if if if if nat_eq(a516, a517) {
        true
    } else {
        let a518 = v1;
        let a519 = 0u64;
        nat_eq(a518, a519)
    } {
        true
    } else {
        let a520 = v2;
        let a521 = 0u64;
        nat_eq(a520, a521)
    } {
        true
    } else {
        let a522 = v3;
        let a523 = 0u64;
        nat_eq(a522, a523)
    } {
        Ok(0u64)
    } else {
        let a536 = {
            let a526 = 2u64;
            let a527 = {
                let a524 = v1;
                let a525 = v2;
                nat_mul(a524, a525)?
            };
            nat_mul(a526, a527)?
        };
        let a537 = {
            let a534 = {
                let a530 = {
                    let a528 = v1;
                    let a529 = v2;
                    nat_mul(a528, a529)?
                };
                let a531 = v3;
                nat_mul(a530, a531)?
            };
            let a535 = {
                let a532 = 1u64;
                let a533 = v0;
                nat_add(a532, a533)?
            };
            nat_mul(a534, a535)?
        };
        nat_add(a536, a537)
    }
}

pub fn f31(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m538 = v2;
    match m538 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h539, _, _, _, _, _, _, _, _, _) => {
                    h539
                }
            }, match v0.clone() {
                Adt1::C0(h540, _, _, _, _, _, _, _, _, _) => {
                    h540
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h541, _, _, _, _, _, _) => {
                    h541
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h542, _, _, _) => {
                    h542
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h543, _, _, _, _, _, _, _, _) => {
                    h543
                }
            }, match v0.clone() {
                Adt1::C0(_, h544, _, _, _, _, _, _, _, _) => {
                    h544
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h545, _, _, _, _, _) => {
                    h545
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h546, _, _) => {
                    h546
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h547, _, _, _, _, _, _, _) => {
                    h547
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h548, _, _, _, _, _, _, _) => {
                    h548
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h549, _, _, _, _) => {
                    h549
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h550, _) => {
                    h550
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h551) => {
                    h551
                }
            };
            let v9: u64 = f73(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                    h552
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h553, _, _, _, _, _, _, _, _, _, _) => {
                    h553
                }
            };
            let v11: u64 = {
                let a561 = v8;
                let a562 = {
                    let a559 = match v5.clone() {
                        Adt6::C0(h554, _, _, _) => {
                            h554
                        }
                    };
                    let a560 = {
                        let a557 = match v6.clone() {
                            Adt6::C0(h555, _, _, _) => {
                                h555
                            }
                        };
                        let a558 = match v7.clone() {
                            Adt6::C0(h556, _, _, _) => {
                                h556
                            }
                        };
                        nat_mul(a557, a558)?
                    };
                    nat_mul(a559, a560)?
                };
                nat_mul(a561, a562)?
            };
            let a564 = match v4.clone() {
                Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                    h563
                }
            };
            let a565 = 0u64;
            if if if if if if if if if if if if if nat_eq(a564, a565) {
                let a566 = v10;
                let a567 = 0u64;
                if nat_eq(a566, a567) {
                    true
                } else {
                    let a568 = v10;
                    let a569 = 1u64;
                    nat_eq(a568, a569)
                }
            } else {
                false
            } {
                let a570 = 0u64;
                let a571 = v8;
                nat_lt(a570, a571)
            } else {
                false
            } {
                let a572 = v8;
                let a573 = 2147483648u64;
                nat_lt(a572, a573)
            } else {
                false
            } {
                let a575 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h574, _, _, _, _, _) => {
                        h574
                    }
                };
                let a576 = 4u64;
                nat_eq(a575, a576)
            } else {
                false
            } {
                let a577 = 0u64;
                let a578 = v9;
                nat_lt(a577, a578)
            } else {
                false
            } {
                let a580 = 0u64;
                let a581 = match v3.clone() {
                    Adt2::C0(_, _, h579, _, _, _, _, _, _, _, _) => {
                        h579
                    }
                };
                nat_lt(a580, a581)
            } else {
                false
            } {
                let a583 = 0u64;
                let a584 = match v3.clone() {
                    Adt2::C0(_, _, _, h582, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                nat_lt(a583, a584)
            } else {
                false
            } {
                let a586 = match v3.clone() {
                    Adt2::C0(_, h585, _, _, _, _, _, _, _, _, _) => {
                        h585
                    }
                };
                let a587 = v11;
                nat_eq(a586, a587)
            } else {
                false
            } {
                let a590 = match v3.clone() {
                    Adt2::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                        h588
                    }
                };
                let a591 = f73(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                        h589
                    }
                })?;
                nat_eq(a590, a591)
            } else {
                false
            } {
                let a594 = match v3.clone() {
                    Adt2::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
                        h592
                    }
                };
                let a595 = f73(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
                        h593
                    }
                })?;
                nat_eq(a594, a595)
            } else {
                false
            } {
                let a603 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h596, _, _, _, _, _, _) => {
                        h596
                    }
                };
                let a604 = {
                    let a601 = v9;
                    let a602 = {
                        let a598 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h597, _, _, _, _, _, _) => {
                                h597
                            }
                        };
                        let a599 = v8;
                        let a600 = 0u64;
                        nat_quot(a598, a599, a600)
                    };
                    nat_mul(a601, a602)?
                };
                nat_eq(a603, a604)
            } else {
                false
            } {
                f17(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt8::C4(f10(v3.clone(), 1u64)?, {
                    let a605 = v10;
                    let a606 = 0u64;
                    if nat_eq(a605, a606) {
                        4u64
                    } else {
                        2u64
                    }
                }, f14(v4, 1u64), f14(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, v8, v9, v10))))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f32(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m607 = v1;
    match m607 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            f74(v0, v3, v2)
        }
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a609 = match v0.clone() {
        Adt2::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
            h608
        }
    };
    let a610 = 1u64;
    if if nat_eq(a609, a610) {
        true
    } else {
        let a612 = match v0.clone() {
            Adt2::C0(h611, _, _, _, _, _, _, _, _, _, _) => {
                h611
            }
        };
        let a613 = 30u64;
        nat_eq(a612, a613)
    } {
        let a615 = match v1.clone() {
            Adt2::C0(h614, _, _, _, _, _, _, _, _, _, _) => {
                h614
            }
        };
        let a616 = 1u64;
        if if if if if if if if if if if nat_eq(a615, a616) {
            let a619 = match v0.clone() {
                Adt2::C0(_, h617, _, _, _, _, _, _, _, _, _) => {
                    h617
                }
            };
            let a620 = match v1.clone() {
                Adt2::C0(_, h618, _, _, _, _, _, _, _, _, _) => {
                    h618
                }
            };
            nat_eq(a619, a620)
        } else {
            false
        } {
            let a622 = 0u64;
            let a623 = match v1.clone() {
                Adt2::C0(_, _, h621, _, _, _, _, _, _, _, _) => {
                    h621
                }
            };
            nat_lt(a622, a623)
        } else {
            false
        } {
            let a626 = match v1.clone() {
                Adt2::C0(_, _, h624, _, _, _, _, _, _, _, _) => {
                    h624
                }
            };
            let a627 = match v1.clone() {
                Adt2::C0(_, _, _, h625, _, _, _, _, _, _, _) => {
                    h625
                }
            };
            nat_eq(a626, a627)
        } else {
            false
        } {
            let a629 = match v1.clone() {
                Adt2::C0(_, _, _, _, h628, _, _, _, _, _, _) => {
                    h628
                }
            };
            let a630 = 1u64;
            nat_eq(a629, a630)
        } else {
            false
        } {
            let a638 = match v0.clone() {
                Adt2::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                    h631
                }
            };
            let a639 = {
                let a636 = {
                    let a634 = match v1.clone() {
                        Adt2::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
                            h632
                        }
                    };
                    let a635 = match v1.clone() {
                        Adt2::C0(_, _, h633, _, _, _, _, _, _, _, _) => {
                            h633
                        }
                    };
                    nat_add(a634, a635)?
                };
                let a637 = 1u64;
                nat_sub(a636, a637)
            };
            nat_eq(a638, a639)
        } else {
            false
        } {
            let a641 = 0u64;
            let a642 = match v0.clone() {
                Adt2::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                    h640
                }
            };
            nat_lt(a641, a642)
        } else {
            false
        } {
            let a644 = 0u64;
            let a645 = match v0.clone() {
                Adt2::C0(_, _, _, _, h643, _, _, _, _, _, _) => {
                    h643
                }
            };
            nat_lt(a644, a645)
        } else {
            false
        } {
            let a647 = 2u64;
            let a648 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h646, _, _, _, _, _) => {
                    h646
                }
            };
            nat_le(a647, a648)
        } else {
            false
        } {
            let a650 = 2u64;
            let a651 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h649, _, _, _, _, _) => {
                    h649
                }
            };
            nat_le(a650, a651)
        } else {
            false
        } {
            f17(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt8::C4(f10(v1.clone(), 1u64)?, 2u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f34(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m652 = v1;
    match m652 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            f75(v0, v3, v2)
        }
    }
}

pub fn f35(v0: u64) -> bool {
    let a653 = v0;
    let a654 = 1u64;
    if if nat_eq(a653, a654) {
        true
    } else {
        let a655 = v0;
        let a656 = 33u64;
        nat_eq(a655, a656)
    } {
        true
    } else {
        let a657 = v0;
        let a658 = 34u64;
        nat_eq(a657, a658)
    }
}

pub fn f36(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt8> {
    let a661 = match v1.clone() {
        Adt2::C0(h659, _, _, _, _, _, _, _, _, _, _) => {
            h659
        }
    };
    let a662 = match v3.clone() {
        Adt2::C0(h660, _, _, _, _, _, _, _, _, _, _) => {
            h660
        }
    };
    if nat_eq(a661, a662) {
        f76(v0, v1, v2, v3)
    } else {
        if if if if f77(match v1.clone() {
            Adt2::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
                h663
            }
        }) {
            f77(match v3.clone() {
                Adt2::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
                    h664
                }
            })
        } else {
            false
        } {
            let a665 = v0;
            let a666 = 33u64;
            if nat_eq(a665, a666) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a667 = f10(v1.clone(), 1u64)?;
            let a668 = f10(v3.clone(), 1u64)?;
            nat_eq(a667, a668)
        } else {
            false
        } {
            let m670 = f78(match v3.clone() {
                Adt2::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                    h669
                }
            });
            match m670 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v4) => {
                    Ok(Adt8::C4(f10(v3.clone(), 1u64)?, match v4 {
                        Adt13::C0(_, _, h671) => {
                            h671
                        }
                    }, f14(v1.clone(), 1u64), f14(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h672, _, _, _, _, _, _, _, _, _, _) => {
                            h672
                        }
                    }, match v3 {
                        Adt2::C0(h673, _, _, _, _, _, _, _, _, _, _) => {
                            h673
                        }
                    })))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a675 = match v0.clone() {
        Adt2::C0(h674, _, _, _, _, _, _, _, _, _, _) => {
            h674
        }
    };
    let a676 = 0u64;
    if if if if nat_eq(a675, a676) {
        true
    } else {
        let a678 = match v0.clone() {
            Adt2::C0(h677, _, _, _, _, _, _, _, _, _, _) => {
                h677
            }
        };
        let a679 = 1u64;
        nat_eq(a678, a679)
    } {
        true
    } else {
        let a681 = match v0.clone() {
            Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                h680
            }
        };
        let a682 = 25u64;
        if nat_eq(a681, a682) {
            true
        } else {
            let a684 = match v0.clone() {
                Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                    h683
                }
            };
            let a685 = 26u64;
            nat_eq(a684, a685)
        }
    } {
        true
    } else {
        let a687 = match v0.clone() {
            Adt2::C0(h686, _, _, _, _, _, _, _, _, _, _) => {
                h686
            }
        };
        let a688 = 30u64;
        nat_eq(a687, a688)
    } {
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
        if if nat_eq(a691, a692) {
            f79(match v1.clone() {
                Adt2::C0(_, h693, _, _, _, _, _, _, _, _, _) => {
                    h693
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h694, _, _, _, _, _, _, _, _) => {
                    h694
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h695, _, _, _, _, _, _, _) => {
                    h695
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h696, _, _, _, _, _, _) => {
                    h696
                }
            }, match v0.clone() {
                Adt2::C0(_, h697, _, _, _, _, _, _, _, _, _) => {
                    h697
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h698, _, _, _, _, _, _, _, _) => {
                    h698
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h699, _, _, _, _, _, _, _) => {
                    h699
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h700, _, _, _, _, _, _) => {
                    h700
                }
            })
        } else {
            false
        } {
            let m702 = f78(match v0.clone() {
                Adt2::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                    h701
                }
            });
            match m702 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v2) => {
                    Ok(Adt8::C4(f10(v1.clone(), 1u64)?, match v2 {
                        Adt13::C0(_, _, h703) => {
                            h703
                        }
                    }, f14(v0, 1u64), f14(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f38(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt8> {
    let m704 = v2;
    match m704 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v4) => {
            let a705 = v0;
            let a706 = 4u64;
            if if if nat_lt(a705, a706) {
                let a709 = match v1.clone() {
                    Adt2::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                        h707
                    }
                };
                let a710 = match v4.clone() {
                    Adt2::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                        h708
                    }
                };
                if nat_eq(a709, a710) {
                    let a713 = match v1.clone() {
                        Adt2::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
                            h711
                        }
                    };
                    let a714 = match v3.clone() {
                        Adt2::C0(h712, _, _, _, _, _, _, _, _, _, _) => {
                            h712
                        }
                    };
                    nat_eq(a713, a714)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f80(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h715, _, _, _, _, _, _, _, _, _) => {
                        h715
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h716, _, _, _, _, _, _, _, _, _) => {
                        h716
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h717, _, _, _, _, _, _, _, _, _) => {
                        h717
                    }
                })? {
                    f80(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h718, _, _, _, _, _, _, _, _) => {
                            h718
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h719, _, _, _, _, _, _, _, _) => {
                            h719
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
                            h720
                        }
                    })?
                } else {
                    false
                } {
                    f80(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h721, _, _, _, _, _, _, _) => {
                            h721
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h722, _, _, _, _, _, _, _) => {
                            h722
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h723, _, _, _, _, _, _, _) => {
                            h723
                        }
                    })?
                } else {
                    false
                } {
                    f80(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h724, _, _, _, _, _, _) => {
                            h724
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h725, _, _, _, _, _, _) => {
                            h725
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h726, _, _, _, _, _, _) => {
                            h726
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m728 = f78(match v1.clone() {
                    Adt2::C0(h727, _, _, _, _, _, _, _, _, _, _) => {
                        h727
                    }
                });
                match m728 {
                    Adt12::C0 => {
                        Ok(Adt8::C0)
                    }
                    Adt12::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt13::C0(_, h729, _) => {
                                h729
                            }
                        };
                        let a734 = {
                            let a731 = match v1.clone() {
                                Adt2::C0(_, h730, _, _, _, _, _, _, _, _, _) => {
                                    h730
                                }
                            };
                            let a732 = v6;
                            let a733 = 1u64;
                            nat_rem(a731, a732, a733)
                        };
                        let a735 = 0u64;
                        if if if nat_eq(a734, a735) {
                            let a740 = {
                                let a737 = match v4.clone() {
                                    Adt2::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
                                        h736
                                    }
                                };
                                let a738 = v6;
                                let a739 = 1u64;
                                nat_rem(a737, a738, a739)
                            };
                            let a741 = 0u64;
                            nat_eq(a740, a741)
                        } else {
                            false
                        } {
                            let a746 = {
                                let a743 = match v3.clone() {
                                    Adt2::C0(_, h742, _, _, _, _, _, _, _, _, _) => {
                                        h742
                                    }
                                };
                                let a744 = v6;
                                let a745 = 1u64;
                                nat_rem(a743, a744, a745)
                            };
                            let a747 = 0u64;
                            nat_eq(a746, a747)
                        } else {
                            false
                        } {
                            Ok(Adt8::C4(f10(v3.clone(), v6)?, match v5 {
                                Adt13::C0(_, _, h748) => {
                                    h748
                                }
                            }, f14(v1, v6), f14(v3, v6), Adt4::C2(v0, f14(v4, v6))))
                        } else {
                            Ok(Adt8::C1)
                        }
                    }
                }
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f39(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let a750 = match v2.clone() {
        Adt2::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
            h749
        }
    };
    let a751 = 0u64;
    if if nat_eq(a750, a751) {
        let a753 = match v3.clone() {
            Adt2::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                h752
            }
        };
        let a754 = 0u64;
        nat_eq(a753, a754)
    } else {
        false
    } {
        let a755 = v0;
        let a756 = 61u64;
        if nat_eq(a755, a756) {
            let v4: bool = {
                let a758 = 0u64;
                let a759 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h757, _) => {
                        h757
                    }
                };
                nat_lt(a758, a759)
            };
            if if if if if if f81(match v2.clone() {
                Adt2::C0(_, h760, _, _, _, _, _, _, _, _, _) => {
                    h760
                }
            }, match v1.clone() {
                Adt1::C0(h761, _, _, _, _, _, _, _, _, _) => {
                    h761
                }
            }, match v1.clone() {
                Adt1::C0(_, h762, _, _, _, _, _, _, _, _) => {
                    h762
                }
            }, match v3.clone() {
                Adt2::C0(_, h763, _, _, _, _, _, _, _, _, _) => {
                    h763
                }
            })? {
                f81(match v2.clone() {
                    Adt2::C0(_, _, h764, _, _, _, _, _, _, _, _) => {
                        h764
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h765, _, _, _, _, _, _, _) => {
                        h765
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h766, _, _, _, _, _, _) => {
                        h766
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h767, _, _, _, _, _, _, _, _) => {
                        h767
                    }
                })?
            } else {
                false
            } {
                f81(match v2.clone() {
                    Adt2::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                        h768
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h769, _, _, _, _, _) => {
                        h769
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h770, _, _, _, _) => {
                        h770
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h771, _, _, _, _, _, _, _) => {
                        h771
                    }
                })?
            } else {
                false
            } {
                f81(match v2.clone() {
                    Adt2::C0(_, _, _, _, h772, _, _, _, _, _, _) => {
                        h772
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h773, _, _, _) => {
                        h773
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h774, _, _) => {
                        h774
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h775, _, _, _, _, _, _) => {
                        h775
                    }
                })?
            } else {
                false
            } {
                let a777 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h776, _) => {
                        h776
                    }
                };
                let a778 = 4294967295u64;
                nat_le(a777, a778)
            } else {
                false
            } {
                if v4 {
                    !f24(match v2.clone() {
                        Adt2::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                            h779
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h780, _, _, _, _, _, _, _, _) => {
                            h780
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h781, _, _, _, _, _, _, _) => {
                            h781
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
                            h782
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h783, _, _, _, _, _, _, _, _, _) => {
                        h783
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h784, _, _, _, _, _, _, _) => {
                        h784
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h785, _, _, _, _, _) => {
                        h785
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h786, _, _, _) => {
                        h786
                    }
                }, v4)))
            } else {
                Ok(Adt8::C1)
            }
        } else {
            let a787 = v0;
            let a788 = 62u64;
            if nat_eq(a787, a788) {
                let a790 = match v1.clone() {
                    Adt1::C0(h789, _, _, _, _, _, _, _, _, _) => {
                        h789
                    }
                };
                let a791 = 2147483648u64;
                if if if if if nat_lt(a790, a791) {
                    let a793 = match v1.clone() {
                        Adt1::C0(_, h792, _, _, _, _, _, _, _, _) => {
                            h792
                        }
                    };
                    let a794 = 2147483648u64;
                    nat_lt(a793, a794)
                } else {
                    false
                } {
                    let a797 = match v1.clone() {
                        Adt1::C0(h795, _, _, _, _, _, _, _, _, _) => {
                            h795
                        }
                    };
                    let a798 = match v2.clone() {
                        Adt2::C0(_, h796, _, _, _, _, _, _, _, _, _) => {
                            h796
                        }
                    };
                    nat_lt(a797, a798)
                } else {
                    false
                } {
                    let a801 = match v1.clone() {
                        Adt1::C0(_, h799, _, _, _, _, _, _, _, _) => {
                            h799
                        }
                    };
                    let a802 = match v2.clone() {
                        Adt2::C0(_, h800, _, _, _, _, _, _, _, _, _) => {
                            h800
                        }
                    };
                    nat_lt(a801, a802)
                } else {
                    false
                } {
                    let a805 = match v2.clone() {
                        Adt2::C0(_, _, h803, _, _, _, _, _, _, _, _) => {
                            h803
                        }
                    };
                    let a806 = match v3.clone() {
                        Adt2::C0(_, _, h804, _, _, _, _, _, _, _, _) => {
                            h804
                        }
                    };
                    if if if nat_eq(a805, a806) {
                        let a809 = match v2.clone() {
                            Adt2::C0(_, _, _, h807, _, _, _, _, _, _, _) => {
                                h807
                            }
                        };
                        let a810 = match v3.clone() {
                            Adt2::C0(_, _, _, h808, _, _, _, _, _, _, _) => {
                                h808
                            }
                        };
                        nat_eq(a809, a810)
                    } else {
                        false
                    } {
                        let a813 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h811, _, _, _, _, _, _) => {
                                h811
                            }
                        };
                        let a814 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h812, _, _, _, _, _, _) => {
                                h812
                            }
                        };
                        nat_eq(a813, a814)
                    } else {
                        false
                    } {
                        let a823 = match v3.clone() {
                            Adt2::C0(_, h815, _, _, _, _, _, _, _, _, _) => {
                                h815
                            }
                        };
                        let a824 = {
                            let a821 = {
                                let a818 = match v2.clone() {
                                    Adt2::C0(_, h816, _, _, _, _, _, _, _, _, _) => {
                                        h816
                                    }
                                };
                                let a819 = match v1.clone() {
                                    Adt1::C0(h817, _, _, _, _, _, _, _, _, _) => {
                                        h817
                                    }
                                };
                                nat_add(a818, a819)?
                            };
                            let a822 = match v1.clone() {
                                Adt1::C0(_, h820, _, _, _, _, _, _, _, _) => {
                                    h820
                                }
                            };
                            nat_add(a821, a822)?
                        };
                        nat_eq(a823, a824)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h825, _, _, _, _, _, _, _, _, _) => {
                            h825
                        }
                    })))
                } else {
                    Ok(Adt8::C1)
                }
            } else {
                let a826 = v0;
                let a827 = 63u64;
                if nat_eq(a826, a827) {
                    if if if if if f82(match v2.clone() {
                        Adt2::C0(_, h828, _, _, _, _, _, _, _, _, _) => {
                            h828
                        }
                    }, match v1.clone() {
                        Adt1::C0(h829, _, _, _, _, _, _, _, _, _) => {
                            h829
                        }
                    })? {
                        f82(match v2.clone() {
                            Adt2::C0(_, _, h830, _, _, _, _, _, _, _, _) => {
                                h830
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h831, _, _, _, _, _, _, _, _) => {
                                h831
                            }
                        })?
                    } else {
                        false
                    } {
                        f82(match v2.clone() {
                            Adt2::C0(_, _, _, h832, _, _, _, _, _, _, _) => {
                                h832
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h833, _, _, _, _, _, _, _) => {
                                h833
                            }
                        })?
                    } else {
                        false
                    } {
                        f82(match v2.clone() {
                            Adt2::C0(_, _, _, _, h834, _, _, _, _, _, _) => {
                                h834
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h835, _, _, _, _, _, _) => {
                                h835
                            }
                        })?
                    } else {
                        false
                    } {
                        let a838 = match v2.clone() {
                            Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                                h836
                            }
                        };
                        let a839 = match v3.clone() {
                            Adt2::C0(_, h837, _, _, _, _, _, _, _, _, _) => {
                                h837
                            }
                        };
                        if if if nat_eq(a838, a839) {
                            let a842 = match v2.clone() {
                                Adt2::C0(_, _, h840, _, _, _, _, _, _, _, _) => {
                                    h840
                                }
                            };
                            let a843 = match v3.clone() {
                                Adt2::C0(_, _, h841, _, _, _, _, _, _, _, _) => {
                                    h841
                                }
                            };
                            nat_eq(a842, a843)
                        } else {
                            false
                        } {
                            let a846 = match v2.clone() {
                                Adt2::C0(_, _, _, h844, _, _, _, _, _, _, _) => {
                                    h844
                                }
                            };
                            let a847 = match v3.clone() {
                                Adt2::C0(_, _, _, h845, _, _, _, _, _, _, _) => {
                                    h845
                                }
                            };
                            nat_eq(a846, a847)
                        } else {
                            false
                        } {
                            let a850 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h848, _, _, _, _, _, _) => {
                                    h848
                                }
                            };
                            let a851 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h849, _, _, _, _, _, _) => {
                                    h849
                                }
                            };
                            nat_eq(a850, a851)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h852, _, _, _, _, _, _, _, _, _) => {
                                h852
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h853, _, _, _, _, _, _, _, _) => {
                                h853
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h854, _, _, _, _, _, _, _) => {
                                h854
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h855, _, _, _, _, _, _) => {
                                h855
                            }
                        })))
                    } else {
                        Ok(Adt8::C1)
                    }
                } else {
                    Ok(Adt8::C0)
                }
            }
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f40(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m856 = v3;
    match m856 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a857 = v0;
                let a858 = 4u64;
                nat_eq(a857, a858)
            };
            let v7: bool = {
                let a860 = 0u64;
                let a861 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h859, _, _, _, _, _) => {
                        h859
                    }
                };
                nat_lt(a860, a861)
            };
            let v8: u64 = f10(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h862, _, _, _, _, _, _, _, _, _) => {
                    h862
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h863, _, _, _, _, _, _, _, _) => {
                    h863
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h864, _, _, _, _, _, _, _) => {
                    h864
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h865, _, _, _, _, _, _) => {
                    h865
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h866, _, _, _, _, _, _, _, _, _) => {
                    h866
                }
            }, match v1.clone() {
                Adt1::C0(_, h867, _, _, _, _, _, _, _, _) => {
                    h867
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h868, _, _, _, _, _, _, _) => {
                    h868
                }
            });
            if if if if if if if if if if if if v6 {
                let a870 = match v2.clone() {
                    Adt2::C0(h869, _, _, _, _, _, _, _, _, _, _) => {
                        h869
                    }
                };
                let a871 = 0u64;
                if nat_eq(a870, a871) {
                    let a873 = match v5.clone() {
                        Adt2::C0(h872, _, _, _, _, _, _, _, _, _, _) => {
                            h872
                        }
                    };
                    let a874 = 0u64;
                    nat_eq(a873, a874)
                } else {
                    false
                }
            } else {
                let a876 = match v2.clone() {
                    Adt2::C0(h875, _, _, _, _, _, _, _, _, _, _) => {
                        h875
                    }
                };
                let a877 = 0u64;
                if if nat_eq(a876, a877) {
                    true
                } else {
                    let a879 = match v2.clone() {
                        Adt2::C0(h878, _, _, _, _, _, _, _, _, _, _) => {
                            h878
                        }
                    };
                    let a880 = 26u64;
                    nat_eq(a879, a880)
                } {
                    let a882 = match v5.clone() {
                        Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                            h881
                        }
                    };
                    let a883 = 0u64;
                    if nat_eq(a882, a883) {
                        true
                    } else {
                        let a885 = match v5.clone() {
                            Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                                h884
                            }
                        };
                        let a886 = 26u64;
                        nat_eq(a885, a886)
                    }
                } else {
                    false
                }
            } {
                let a889 = match v2.clone() {
                    Adt2::C0(h887, _, _, _, _, _, _, _, _, _, _) => {
                        h887
                    }
                };
                let a890 = match v4.clone() {
                    Adt2::C0(h888, _, _, _, _, _, _, _, _, _, _) => {
                        h888
                    }
                };
                nat_eq(a889, a890)
            } else {
                false
            } {
                f15(v2.clone(), v4.clone())
            } else {
                false
            } {
                f16(v2.clone())?
            } else {
                false
            } {
                f16(v4.clone())?
            } else {
                false
            } {
                let a892 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h891, _, _, _, _, _) => {
                        h891
                    }
                };
                let a893 = 4u64;
                nat_eq(a892, a893)
            } else {
                false
            } {
                let a894 = v8;
                let a895 = f10(v2.clone(), 1u64)?;
                nat_le(a894, a895)
            } else {
                false
            } {
                let a897 = match v1.clone() {
                    Adt1::C0(_, _, _, h896, _, _, _, _, _, _) => {
                        h896
                    }
                };
                let a898 = 1073741824u64;
                nat_lt(a897, a898)
            } else {
                false
            } {
                let a900 = match v1.clone() {
                    Adt1::C0(h899, _, _, _, _, _, _, _, _, _) => {
                        h899
                    }
                };
                let a901 = 4294967295u64;
                if if if if nat_le(a900, a901) {
                    let a903 = match v1.clone() {
                        Adt1::C0(_, h902, _, _, _, _, _, _, _, _) => {
                            h902
                        }
                    };
                    let a904 = 4294967295u64;
                    nat_le(a903, a904)
                } else {
                    false
                } {
                    let a906 = match v1.clone() {
                        Adt1::C0(_, _, h905, _, _, _, _, _, _, _) => {
                            h905
                        }
                    };
                    let a907 = 4294967295u64;
                    nat_le(a906, a907)
                } else {
                    false
                } {
                    let a909 = match v1.clone() {
                        Adt1::C0(_, _, _, h908, _, _, _, _, _, _) => {
                            h908
                        }
                    };
                    let a910 = 4294967295u64;
                    nat_le(a909, a910)
                } else {
                    false
                } {
                    let a912 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h911, _, _, _, _, _) => {
                            h911
                        }
                    };
                    let a913 = 4294967295u64;
                    nat_le(a912, a913)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a916 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h914) => {
                            h914
                        }
                    };
                    let a917 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h915) => {
                            h915
                        }
                    };
                    nat_eq(a916, a917)
                } else {
                    true
                }
            } else {
                false
            } {
                f83(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h918, _, _, _, _, _, _) => {
                        h918
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h919, _) => {
                        h919
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f10(v2, 1u64)?
                };
                Ok(Adt8::C4({
                    let a920 = v10;
                    let a921 = v8;
                    nat_add(a920, a921)?
                }, 4u64, f14(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h922, _, _, _, _, _, _) => {
                        h922
                    }
                }, v6)))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f41(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a924 = match v0.clone() {
        Adt2::C0(h923, _, _, _, _, _, _, _, _, _, _) => {
            h923
        }
    };
    let a925 = 0u64;
    if if if nat_eq(a924, a925) {
        let a927 = match v1.clone() {
            Adt2::C0(h926, _, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        };
        let a928 = 0u64;
        nat_eq(a927, a928)
    } else {
        false
    } {
        f79(match v0.clone() {
            Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h930, _, _, _, _, _, _, _, _) => {
                h930
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h931, _, _, _, _, _, _, _) => {
                h931
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h932, _, _, _, _, _, _) => {
                h932
            }
        }, match v1.clone() {
            Adt2::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
                h933
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h934, _, _, _, _, _, _, _, _) => {
                h934
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h936, _, _, _, _, _, _) => {
                h936
            }
        })
    } else {
        false
    } {
        let v2: u64 = f10(v1.clone(), 1u64)?;
        Ok(Adt8::C4({
            let a937 = v2;
            let a938 = f10(v0.clone(), 1u64)?;
            nat_add(a937, a938)?
        }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f42(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a940 = match v1.clone() {
        Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
            h939
        }
    };
    let a941 = 0u64;
    if if if nat_eq(a940, a941) {
        true
    } else {
        let a943 = match v1.clone() {
            Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a944 = 1u64;
        nat_eq(a943, a944)
    } {
        true
    } else {
        let a946 = match v1.clone() {
            Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a947 = 30u64;
        nat_eq(a946, a947)
    } {
        let a950 = match v1.clone() {
            Adt2::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a951 = match v2.clone() {
            Adt2::C0(h949, _, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        };
        if if if if if nat_eq(a950, a951) {
            let a954 = match v1.clone() {
                Adt2::C0(_, h952, _, _, _, _, _, _, _, _, _) => {
                    h952
                }
            };
            let a955 = match v2.clone() {
                Adt2::C0(_, h953, _, _, _, _, _, _, _, _, _) => {
                    h953
                }
            };
            nat_eq(a954, a955)
        } else {
            false
        } {
            let a958 = match v1.clone() {
                Adt2::C0(_, _, _, h956, _, _, _, _, _, _, _) => {
                    h956
                }
            };
            let a959 = match v2.clone() {
                Adt2::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                    h957
                }
            };
            nat_eq(a958, a959)
        } else {
            false
        } {
            let a962 = match v1.clone() {
                Adt2::C0(_, _, _, _, h960, _, _, _, _, _, _) => {
                    h960
                }
            };
            let a963 = match v2.clone() {
                Adt2::C0(_, _, _, _, h961, _, _, _, _, _, _) => {
                    h961
                }
            };
            nat_eq(a962, a963)
        } else {
            false
        } {
            let a964 = v0;
            let a965 = 4u64;
            if nat_eq(a964, a965) {
                let a967 = match v1.clone() {
                    Adt2::C0(_, _, h966, _, _, _, _, _, _, _, _) => {
                        h966
                    }
                };
                let a968 = 1u64;
                if nat_eq(a967, a968) {
                    let a971 = match v1.clone() {
                        Adt2::C0(_, h969, _, _, _, _, _, _, _, _, _) => {
                            h969
                        }
                    };
                    let a972 = match v2.clone() {
                        Adt2::C0(_, _, h970, _, _, _, _, _, _, _, _) => {
                            h970
                        }
                    };
                    nat_eq(a971, a972)
                } else {
                    false
                }
            } else {
                let a973 = v0;
                let a974 = 4u64;
                if if nat_lt(a973, a974) {
                    let a977 = match v1.clone() {
                        Adt2::C0(_, h975, _, _, _, _, _, _, _, _, _) => {
                            h975
                        }
                    };
                    let a978 = match v1.clone() {
                        Adt2::C0(_, _, h976, _, _, _, _, _, _, _, _) => {
                            h976
                        }
                    };
                    nat_eq(a977, a978)
                } else {
                    false
                } {
                    let a981 = match v1.clone() {
                        Adt2::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
                            h979
                        }
                    };
                    let a982 = match v2.clone() {
                        Adt2::C0(_, _, h980, _, _, _, _, _, _, _, _) => {
                            h980
                        }
                    };
                    nat_eq(a981, a982)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m984 = f78(match v1.clone() {
                Adt2::C0(h983, _, _, _, _, _, _, _, _, _, _) => {
                    h983
                }
            });
            match m984 {
                Adt12::C0 => {
                    Ok(Adt8::C0)
                }
                Adt12::C1(v3) => {
                    Ok(Adt8::C4(f10(v2.clone(), 1u64)?, match v3 {
                        Adt13::C0(_, _, h985) => {
                            h985
                        }
                    }, f14(v1, 1u64), f14(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f43(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if if if f84(match v1.clone() {
        Adt2::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
            h986
        }
    }, v0) {
        let a989 = match v1.clone() {
            Adt2::C0(h987, _, _, _, _, _, _, _, _, _, _) => {
                h987
            }
        };
        let a990 = match v2.clone() {
            Adt2::C0(h988, _, _, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        nat_eq(a989, a990)
    } else {
        false
    } {
        let a993 = match v1.clone() {
            Adt2::C0(_, h991, _, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a994 = match v2.clone() {
            Adt2::C0(_, h992, _, _, _, _, _, _, _, _, _) => {
                h992
            }
        };
        if if if nat_eq(a993, a994) {
            let a997 = match v1.clone() {
                Adt2::C0(_, _, h995, _, _, _, _, _, _, _, _) => {
                    h995
                }
            };
            let a998 = match v2.clone() {
                Adt2::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                    h996
                }
            };
            nat_eq(a997, a998)
        } else {
            false
        } {
            let a1001 = match v1.clone() {
                Adt2::C0(_, _, _, h999, _, _, _, _, _, _, _) => {
                    h999
                }
            };
            let a1002 = match v2.clone() {
                Adt2::C0(_, _, _, h1000, _, _, _, _, _, _, _) => {
                    h1000
                }
            };
            nat_eq(a1001, a1002)
        } else {
            false
        } {
            let a1005 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1003, _, _, _, _, _, _) => {
                    h1003
                }
            };
            let a1006 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1004, _, _, _, _, _, _) => {
                    h1004
                }
            };
            nat_eq(a1005, a1006)
        } else {
            false
        }
    } else {
        false
    } {
        let m1008 = f78(match v1.clone() {
            Adt2::C0(h1007, _, _, _, _, _, _, _, _, _, _) => {
                h1007
            }
        });
        match m1008 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v3) => {
                Ok(Adt8::C4(f10(v2.clone(), 1u64)?, match v3 {
                    Adt13::C0(_, _, h1009) => {
                        h1009
                    }
                }, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1010, _, _, _, _, _, _, _, _, _, _) => {
                        h1010
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f44(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1011, _, _, _, _, _, _, _, _, _) => {
                h1011
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1012, _, _, _, _, _, _, _) => {
                h1012
            }
        }
    };
    let a1013 = 0u64;
    let a1014 = v4;
    if if nat_lt(a1013, a1014) {
        let a1015 = v4;
        let a1016 = 2147483648u64;
        nat_lt(a1015, a1016)
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
        let v7: u64 = f85(match v5.clone() {
            Adt2::C0(_, _, h1017, _, _, _, _, _, _, _, _) => {
                h1017
            }
        }, v4)?;
        let v8: u64 = f85(match v5.clone() {
            Adt2::C0(_, _, _, h1018, _, _, _, _, _, _, _) => {
                h1018
            }
        }, v4)?;
        let a1020 = match v5.clone() {
            Adt2::C0(h1019, _, _, _, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1021 = 0u64;
        if if if if if nat_eq(a1020, a1021) {
            let a1023 = match v6.clone() {
                Adt2::C0(h1022, _, _, _, _, _, _, _, _, _, _) => {
                    h1022
                }
            };
            let a1024 = 0u64;
            nat_eq(a1023, a1024)
        } else {
            false
        } {
            let a1026 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1025, _, _, _, _, _, _) => {
                    h1025
                }
            };
            let a1027 = 1u64;
            nat_eq(a1026, a1027)
        } else {
            false
        } {
            let a1030 = match v5.clone() {
                Adt2::C0(_, h1028, _, _, _, _, _, _, _, _, _) => {
                    h1028
                }
            };
            let a1031 = match v6.clone() {
                Adt2::C0(_, h1029, _, _, _, _, _, _, _, _, _) => {
                    h1029
                }
            };
            nat_eq(a1030, a1031)
        } else {
            false
        } {
            if v0 {
                let a1033 = match v5.clone() {
                    Adt2::C0(_, _, h1032, _, _, _, _, _, _, _, _) => {
                        h1032
                    }
                };
                let a1034 = 2147483648u64;
                if if if nat_lt(a1033, a1034) {
                    let a1036 = match v5.clone() {
                        Adt2::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                            h1035
                        }
                    };
                    let a1037 = 2147483648u64;
                    nat_lt(a1036, a1037)
                } else {
                    false
                } {
                    let a1040 = {
                        let a1038 = v7;
                        let a1039 = v8;
                        nat_mul(a1038, a1039)?
                    };
                    let a1041 = 2147483648u64;
                    nat_le(a1040, a1041)
                } else {
                    false
                } {
                    f86(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1043 = match v1.clone() {
                    Adt1::C0(h1042, _, _, _, _, _, _, _, _, _) => {
                        h1042
                    }
                };
                let a1044 = v7;
                if if if if nat_eq(a1043, a1044) {
                    let a1046 = match v1 {
                        Adt1::C0(_, h1045, _, _, _, _, _, _, _, _) => {
                            h1045
                        }
                    };
                    let a1047 = v8;
                    nat_eq(a1046, a1047)
                } else {
                    false
                } {
                    f87(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1049 = match v6.clone() {
                        Adt2::C0(_, _, h1048, _, _, _, _, _, _, _, _) => {
                            h1048
                        }
                    };
                    let a1050 = v4;
                    if nat_eq(a1049, a1050) {
                        let a1052 = match v6.clone() {
                            Adt2::C0(_, _, _, h1051, _, _, _, _, _, _, _) => {
                                h1051
                            }
                        };
                        let a1053 = v4;
                        nat_eq(a1052, a1053)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1057 = match v6 {
                        Adt2::C0(_, _, _, _, h1054, _, _, _, _, _, _) => {
                            h1054
                        }
                    };
                    let a1058 = {
                        let a1055 = v7;
                        let a1056 = v8;
                        nat_mul(a1055, a1056)?
                    };
                    nat_eq(a1057, a1058)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt8::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f45(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt8> {
    let a1060 = match v2.clone() {
        Adt2::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
            h1059
        }
    };
    let a1061 = 0u64;
    if if if if if nat_eq(a1060, a1061) {
        true
    } else {
        let a1063 = match v2.clone() {
            Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        let a1064 = 1u64;
        nat_eq(a1063, a1064)
    } {
        true
    } else {
        let a1066 = match v2.clone() {
            Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1067 = 30u64;
        nat_eq(a1066, a1067)
    } {
        let a1070 = match v2.clone() {
            Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
                h1068
            }
        };
        let a1071 = match v3.clone() {
            Adt2::C0(h1069, _, _, _, _, _, _, _, _, _, _) => {
                h1069
            }
        };
        nat_eq(a1070, a1071)
    } else {
        false
    } {
        if if f15(v2.clone(), v3.clone()) {
            let a1073 = match v1.clone() {
                Adt1::C0(h1072, _, _, _, _, _, _, _, _, _) => {
                    h1072
                }
            };
            let a1074 = 4294967296u64;
            nat_lt(a1073, a1074)
        } else {
            false
        } {
            let a1076 = match v1.clone() {
                Adt1::C0(_, h1075, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1077 = 4294967296u64;
            nat_lt(a1076, a1077)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v3.clone(), 1u64)?, {
            let a1079 = match v2.clone() {
                Adt2::C0(h1078, _, _, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            };
            let a1080 = 0u64;
            if nat_eq(a1079, a1080) {
                4u64
            } else {
                2u64
            }
        }, f14(v2.clone(), 1u64), f14(v3, 1u64), Adt4::C13({
            let a1081 = v0;
            let a1082 = 31u64;
            if nat_eq(a1081, a1082) {
                0u64
            } else {
                let a1083 = v0;
                let a1084 = 47u64;
                if nat_eq(a1083, a1084) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1085, _, _, _, _, _, _, _, _, _, _) => {
                h1085
            }
        }, match v1.clone() {
            Adt1::C0(h1086, _, _, _, _, _, _, _, _, _) => {
                h1086
            }
        }, match v1 {
            Adt1::C0(_, h1087, _, _, _, _, _, _, _, _) => {
                h1087
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a1089 = match v0.clone() {
        Adt2::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
            h1088
        }
    };
    let a1090 = 0u64;
    if if if nat_eq(a1089, a1090) {
        let a1092 = match v1.clone() {
            Adt2::C0(h1091, _, _, _, _, _, _, _, _, _, _) => {
                h1091
            }
        };
        let a1093 = 0u64;
        nat_eq(a1092, a1093)
    } else {
        false
    } {
        f15(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v1.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> R<Adt8> {
    let a1095 = match v0.clone() {
        Adt2::C0(h1094, _, _, _, _, _, _, _, _, _, _) => {
            h1094
        }
    };
    let a1096 = 0u64;
    if if nat_eq(a1095, a1096) {
        let a1098 = match v1.clone() {
            Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1099 = 26u64;
        nat_eq(a1098, a1099)
    } else {
        false
    } {
        let a1101 = match v0.clone() {
            Adt2::C0(_, _, _, h1100, _, _, _, _, _, _, _) => {
                h1100
            }
        };
        let a1102 = 1u64;
        if if if if if if if nat_eq(a1101, a1102) {
            let a1104 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1103, _, _, _, _, _, _) => {
                    h1103
                }
            };
            let a1105 = 1u64;
            nat_eq(a1104, a1105)
        } else {
            false
        } {
            let a1107 = match v0.clone() {
                Adt2::C0(_, h1106, _, _, _, _, _, _, _, _, _) => {
                    h1106
                }
            };
            let a1108 = 2147483647u64;
            nat_le(a1107, a1108)
        } else {
            false
        } {
            let a1111 = match v1.clone() {
                Adt2::C0(_, h1109, _, _, _, _, _, _, _, _, _) => {
                    h1109
                }
            };
            let a1112 = match v0.clone() {
                Adt2::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                    h1110
                }
            };
            nat_eq(a1111, a1112)
        } else {
            false
        } {
            let a1114 = match v1.clone() {
                Adt2::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
                    h1113
                }
            };
            let a1115 = 1u64;
            nat_eq(a1114, a1115)
        } else {
            false
        } {
            let a1117 = match v1.clone() {
                Adt2::C0(_, _, _, h1116, _, _, _, _, _, _, _) => {
                    h1116
                }
            };
            let a1118 = 1u64;
            nat_eq(a1117, a1118)
        } else {
            false
        } {
            let a1120 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1119, _, _, _, _, _, _) => {
                    h1119
                }
            };
            let a1121 = 1u64;
            nat_eq(a1120, a1121)
        } else {
            false
        } {
            Ok(Adt8::C4({
                let a1124 = match v0.clone() {
                    Adt2::C0(_, _, h1122, _, _, _, _, _, _, _, _) => {
                        h1122
                    }
                };
                let a1125 = f88(match v0.clone() {
                    Adt2::C0(_, h1123, _, _, _, _, _, _, _, _, _) => {
                        h1123
                    }
                })?;
                nat_mul(a1124, a1125)?
            }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f48(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt8> {
    let m1126 = v1;
    match m1126 {
        None => {
            Ok(Adt8::C1)
        }
        Some(v3) => {
            let v4: u64 = f89(f14(v0.clone(), 1u64))?;
            let a1128 = match v0.clone() {
                Adt2::C0(h1127, _, _, _, _, _, _, _, _, _, _) => {
                    h1127
                }
            };
            let a1129 = 26u64;
            if if if nat_eq(a1128, a1129) {
                let a1131 = match v3.clone() {
                    Adt2::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                };
                let a1132 = 26u64;
                nat_eq(a1131, a1132)
            } else {
                false
            } {
                let a1134 = match v2.clone() {
                    Adt2::C0(h1133, _, _, _, _, _, _, _, _, _, _) => {
                        h1133
                    }
                };
                let a1135 = 27u64;
                nat_eq(a1134, a1135)
            } else {
                false
            } {
                if if if if if if f15(v0.clone(), v3.clone()) {
                    let a1137 = match v2.clone() {
                        Adt2::C0(_, h1136, _, _, _, _, _, _, _, _, _) => {
                            h1136
                        }
                    };
                    let a1138 = 1u64;
                    nat_eq(a1137, a1138)
                } else {
                    false
                } {
                    let a1140 = match v2.clone() {
                        Adt2::C0(_, _, h1139, _, _, _, _, _, _, _, _) => {
                            h1139
                        }
                    };
                    let a1141 = 1u64;
                    nat_eq(a1140, a1141)
                } else {
                    false
                } {
                    let a1143 = match v2.clone() {
                        Adt2::C0(_, _, _, h1142, _, _, _, _, _, _, _) => {
                            h1142
                        }
                    };
                    let a1144 = 1u64;
                    nat_eq(a1143, a1144)
                } else {
                    false
                } {
                    let a1146 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1145, _, _, _, _, _, _) => {
                            h1145
                        }
                    };
                    let a1147 = 1u64;
                    nat_eq(a1146, a1147)
                } else {
                    false
                } {
                    let a1148 = v4;
                    let a1149 = 9223372036854775808u64;
                    nat_lt(a1148, a1149)
                } else {
                    false
                } {
                    Ok(Adt8::C4({
                        let a1152 = 2u64;
                        let a1153 = {
                            let a1150 = v4;
                            let a1151 = 2u64;
                            nat_mul(a1150, a1151)?
                        };
                        nat_add(a1152, a1153)?
                    }, 8u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C18(v4, f14(v3, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            } else {
                Ok(Adt8::C0)
            }
        }
    }
}

pub fn f49(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1154, _, _, _, _, _, _, _, _, _, _) => {
            h1154
        }
    };
    let a1155 = v3;
    let a1156 = 0u64;
    if if nat_eq(a1155, a1156) {
        true
    } else {
        let a1157 = v0;
        let a1158 = 13u64;
        if nat_eq(a1157, a1158) {
            let a1159 = v3;
            let a1160 = 1u64;
            if nat_eq(a1159, a1160) {
                true
            } else {
                let a1161 = v3;
                let a1162 = 30u64;
                nat_eq(a1161, a1162)
            }
        } else {
            false
        }
    } {
        let a1164 = v3;
        let a1165 = match v2.clone() {
            Adt2::C0(h1163, _, _, _, _, _, _, _, _, _, _) => {
                h1163
            }
        };
        if if nat_eq(a1164, a1165) {
            let a1166 = v0;
            let a1167 = 13u64;
            if nat_eq(a1166, a1167) {
                let a1169 = match v2.clone() {
                    Adt2::C0(_, h1168, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                };
                let a1170 = 1u64;
                if if if nat_eq(a1169, a1170) {
                    let a1172 = match v2.clone() {
                        Adt2::C0(_, _, h1171, _, _, _, _, _, _, _, _) => {
                            h1171
                        }
                    };
                    let a1173 = 1u64;
                    nat_eq(a1172, a1173)
                } else {
                    false
                } {
                    let a1175 = match v2.clone() {
                        Adt2::C0(_, _, _, h1174, _, _, _, _, _, _, _) => {
                            h1174
                        }
                    };
                    let a1176 = 1u64;
                    nat_eq(a1175, a1176)
                } else {
                    false
                } {
                    let a1178 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1177, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    let a1179 = 1u64;
                    nat_eq(a1178, a1179)
                } else {
                    false
                }
            } else {
                let a1181 = match v2.clone() {
                    Adt2::C0(_, h1180, _, _, _, _, _, _, _, _, _) => {
                        h1180
                    }
                };
                let a1182 = 1u64;
                if if if nat_eq(a1181, a1182) {
                    let a1185 = match v1.clone() {
                        Adt2::C0(_, _, h1183, _, _, _, _, _, _, _, _) => {
                            h1183
                        }
                    };
                    let a1186 = match v2.clone() {
                        Adt2::C0(_, _, h1184, _, _, _, _, _, _, _, _) => {
                            h1184
                        }
                    };
                    nat_eq(a1185, a1186)
                } else {
                    false
                } {
                    let a1189 = match v1.clone() {
                        Adt2::C0(_, _, _, h1187, _, _, _, _, _, _, _) => {
                            h1187
                        }
                    };
                    let a1190 = match v2.clone() {
                        Adt2::C0(_, _, _, h1188, _, _, _, _, _, _, _) => {
                            h1188
                        }
                    };
                    nat_eq(a1189, a1190)
                } else {
                    false
                } {
                    let a1193 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1191, _, _, _, _, _, _) => {
                            h1191
                        }
                    };
                    let a1194 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1192, _, _, _, _, _, _) => {
                            h1192
                        }
                    };
                    nat_eq(a1193, a1194)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f14(v1, 1u64);
            Ok(Adt8::C4(f90(v4.clone(), v0, v3)?, {
                let a1195 = v3;
                let a1196 = 0u64;
                if nat_eq(a1195, a1196) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f14(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt8::C1)
        }
    } else {
        Ok(Adt8::C0)
    }
}

pub fn f50(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1197 = v0;
    let a1198 = 89u64;
    if if nat_eq(a1197, a1198) {
        let a1199 = v1;
        let a1200 = 4u64;
        nat_eq(a1199, a1200)
    } else {
        false
    } {
        Ok(Some(19u64))
    } else {
        let a1201 = 10u64;
        let a1202 = v0;
        if if nat_le(a1201, a1202) {
            let a1203 = v0;
            let a1204 = 12u64;
            nat_le(a1203, a1204)
        } else {
            false
        } {
            Ok(Some({
                let a1205 = v0;
                let a1206 = 3u64;
                nat_add(a1205, a1206)?
            }))
        } else {
            let a1207 = v0;
            let a1208 = 89u64;
            if if nat_eq(a1207, a1208) {
                let a1209 = v1;
                let a1210 = 7u64;
                nat_eq(a1209, a1210)
            } else {
                false
            } {
                Ok(Some(17u64))
            } else {
                let a1211 = v0;
                let a1212 = 89u64;
                if if nat_eq(a1211, a1212) {
                    let a1213 = v1;
                    let a1214 = 10u64;
                    nat_eq(a1213, a1214)
                } else {
                    false
                } {
                    Ok(Some(18u64))
                } else {
                    let a1215 = v0;
                    let a1216 = 89u64;
                    if if nat_eq(a1215, a1216) {
                        let a1217 = v1;
                        let a1218 = 13u64;
                        nat_eq(a1217, a1218)
                    } else {
                        false
                    } {
                        Ok(Some(16u64))
                    } else {
                        let a1219 = v0;
                        let a1220 = 8u64;
                        if nat_eq(a1219, a1220) {
                            Ok(Some(5u64))
                        } else {
                            let a1221 = v0;
                            let a1222 = 9u64;
                            if nat_eq(a1221, a1222) {
                                Ok(Some(6u64))
                            } else {
                                let a1223 = v0;
                                let a1224 = 89u64;
                                if nat_eq(a1223, a1224) {
                                    let a1225 = v1;
                                    let a1226 = 0u64;
                                    if nat_eq(a1225, a1226) {
                                        Ok(Some(0u64))
                                    } else {
                                        let a1227 = v1;
                                        let a1228 = 1u64;
                                        if nat_eq(a1227, a1228) {
                                            Ok(Some(1u64))
                                        } else {
                                            let a1229 = v1;
                                            let a1230 = 2u64;
                                            if nat_eq(a1229, a1230) {
                                                Ok(Some(2u64))
                                            } else {
                                                let a1231 = v1;
                                                let a1232 = 3u64;
                                                if nat_eq(a1231, a1232) {
                                                    Ok(Some(3u64))
                                                } else {
                                                    let a1233 = v1;
                                                    let a1234 = 6u64;
                                                    if nat_eq(a1233, a1234) {
                                                        Ok(Some(4u64))
                                                    } else {
                                                        let a1235 = v1;
                                                        let a1236 = 12u64;
                                                        if nat_eq(a1235, a1236) {
                                                            Ok(Some(7u64))
                                                        } else {
                                                            let a1237 = v1;
                                                            let a1238 = 11u64;
                                                            if nat_eq(a1237, a1238) {
                                                                Ok(Some(8u64))
                                                            } else {
                                                                let a1239 = v1;
                                                                let a1240 = 18u64;
                                                                if nat_eq(a1239, a1240) {
                                                                    Ok(Some(9u64))
                                                                } else {
                                                                    let a1241 = v1;
                                                                    let a1242 = 19u64;
                                                                    if nat_eq(a1241, a1242) {
                                                                        Ok(Some(10u64))
                                                                    } else {
                                                                        let a1243 = v1;
                                                                        let a1244 = 20u64;
                                                                        if nat_eq(a1243, a1244) {
                                                                            Ok(Some(11u64))
                                                                        } else {
                                                                            let a1245 = v1;
                                                                            let a1246 = 21u64;
                                                                            if nat_eq(a1245, a1246) {
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

pub fn f51(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let m1247 = f91(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1247 {
        Adt8::C0 => {
            Ok(Adt8::C0)
        }
        Adt8::C1 => {
            let a1248 = v0;
            let a1249 = 2u64;
            if if if if nat_eq(a1248, a1249) {
                true
            } else {
                let a1250 = v0;
                let a1251 = 5u64;
                nat_eq(a1250, a1251)
            } {
                true
            } else {
                let a1252 = v0;
                let a1253 = 6u64;
                nat_eq(a1252, a1253)
            } {
                true
            } else {
                let a1254 = v0;
                let a1255 = 7u64;
                nat_eq(a1254, a1255)
            } {
                Ok(Adt8::C5(67u64))
            } else {
                Ok(Adt8::C1)
            }
        }
        Adt8::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt8::C4(v12, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1256 = v3;
                    match m1256 {
                        None => {
                            f14(v2, 1u64)
                        }
                        Some(v13) => {
                            f14(v13, 1u64)
                        }
                    }
                })))
            } else {
                Ok(Adt8::C5(67u64))
            }
        }
        Adt8::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            Ok(Adt8::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23))
        }
        Adt8::C4(v24, v25, v26, v27, v28) => {
            Ok(Adt8::C4(v24, v25, v26, v27, v28))
        }
        Adt8::C5(v5) => {
            Ok(Adt8::C5(v5))
        }
    }
}

pub fn f52(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt8> {
    let a1258 = match v1.clone() {
        Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
            h1257
        }
    };
    let a1259 = 0u64;
    if if if if if nat_eq(a1258, a1259) {
        true
    } else {
        let a1261 = match v1.clone() {
            Adt2::C0(h1260, _, _, _, _, _, _, _, _, _, _) => {
                h1260
            }
        };
        let a1262 = 1u64;
        nat_eq(a1261, a1262)
    } {
        true
    } else {
        let a1264 = match v1.clone() {
            Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
                h1263
            }
        };
        let a1265 = 30u64;
        nat_eq(a1264, a1265)
    } {
        let a1268 = match v1.clone() {
            Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                h1266
            }
        };
        let a1269 = match v2.clone() {
            Adt2::C0(h1267, _, _, _, _, _, _, _, _, _, _) => {
                h1267
            }
        };
        if nat_eq(a1268, a1269) {
            true
        } else {
            let a1271 = match v2.clone() {
                Adt2::C0(h1270, _, _, _, _, _, _, _, _, _, _) => {
                    h1270
                }
            };
            let a1272 = 0u64;
            nat_eq(a1271, a1272)
        }
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt8::C4(f10(v2.clone(), 1u64)?, {
            let a1274 = match v2.clone() {
                Adt2::C0(h1273, _, _, _, _, _, _, _, _, _, _) => {
                    h1273
                }
            };
            let a1275 = 0u64;
            if nat_eq(a1274, a1275) {
                4u64
            } else {
                2u64
            }
        }, f14(v1.clone(), 1u64), f14(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1276, _, _, _, _, _, _, _, _, _, _) => {
                h1276
            }
        }, match v2 {
            Adt2::C0(h1277, _, _, _, _, _, _, _, _, _, _) => {
                h1277
            }
        })))
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f53(v0: Adt4, v1: Adt7) -> Adt7 {
    let m1278 = v1;
    match m1278 {
        Adt7::C0(v2) => {
            Adt7::C0(v2)
        }
        Adt7::C1(v3, v4, v5) => {
            Adt7::C1(v3, v4, v5)
        }
        Adt7::C2(v6, v7, v8, v9, v10, v11) => {
            Adt7::C2(v6, v7, v8, v9, v10, v11)
        }
        Adt7::C3(v12, v13, v14, v15) => {
            f92(f93(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f94(v0), 3u64, v15)
        }
        Adt7::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f95(v0.clone(), v16);
            f92(f96(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
        }
        Adt7::C5(v24, v25, v26, v27, v28, v29) => {
            Adt7::C5(v24, v25, v26, v27, v28, v29)
        }
        Adt7::C6(v30, v31, v32, v33) => {
            Adt7::C6(v30, v31, v32, v33)
        }
        Adt7::C7(v34, v35, v36, v37, v38, v39) => {
            Adt7::C7(v34, v35, v36, v37, v38, v39)
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt7> {
    let m1279 = v3;
    match m1279 {
        Adt4::C0 => {
            Ok(Adt7::C1(false, f97(v0, v1)?, f97(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt7::C1(false, f99(v0, v1, v2.clone())?, f97(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f100(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f101(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f102(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f101(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f103(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f104(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f105(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt7::C1(true, 0u64, f97(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f106(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt7::C3(v28, v29, f97(v0, v1)?, f97(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f107(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt7::C3(v34, v34, f97(v0, v1)?, f97(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt7::C3(v38, v39, f97(v0, v1)?, f97(v0, v2)?))
        }
        Adt4::C15 => {
            f108(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f109(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C18(v42, v43) => {
            f110(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C21 => {
            Ok(Adt7::C1(false, f97(f98(v0, match v2.clone() {
                Adt3::C0(h1280, _, _, _, _, _, _, _) => {
                    h1280
                }
            }, match v2.clone() {
                Adt3::C0(_, h1281, _, _, _, _, _, _) => {
                    h1281
                }
            })?, v1)?, f97(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt7::C0(Adt8::C5(65u64)))
        }
        Adt4::C25(v48, v49, v50) => {
            f111(v0, v48, v49, v50)
        }
        Adt4::C26(v52, _, _, _) => {
            let a1282 = v52;
            let a1283 = 64u64;
            if if if nat_eq(a1282, a1283) {
                true
            } else {
                let a1284 = v52;
                let a1285 = 65u64;
                nat_eq(a1284, a1285)
            } {
                true
            } else {
                let a1286 = v52;
                let a1287 = 66u64;
                nat_eq(a1286, a1287)
            } {
                Ok(Adt7::C0(Adt8::C5(64u64)))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
        Adt4::C27(v51) => {
            f59(v0, v1, v2, v51)
        }
    }
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = f88(match v1.clone() {
        Adt3::C0(h1288, _, _, _, _, _, _, _) => {
            h1288
        }
    })?;
    let v5: u64 = {
        let a1289 = v0;
        let a1290 = v4;
        let a1291 = 0u64;
        nat_quot(a1289, a1290, a1291)
    };
    let v6: u64 = {
        let a1292 = v0;
        let a1293 = v4;
        let a1294 = 0u64;
        nat_rem(a1292, a1293, a1294)
    };
    let a1295 = v6;
    let a1296 = 0u64;
    if nat_eq(a1295, a1296) {
        Ok(Adt7::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1297 = v6;
        let a1298 = 1u64;
        if nat_eq(a1297, a1298) {
            Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1301 = {
                let a1299 = v6;
                let a1300 = 1u64;
                nat_add(a1299, a1300)?
            };
            let a1302 = v4;
            if nat_eq(a1301, a1302) {
                Ok(Adt7::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f97(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1303 = v6;
                    let a1304 = 2u64;
                    nat_sub(a1303, a1304)
                };
                let v8: u64 = {
                    let a1305 = v7;
                    let a1306 = 3u64;
                    let a1307 = 0u64;
                    nat_quot(a1305, a1306, a1307)
                };
                let v9: u64 = {
                    let a1308 = v7;
                    let a1309 = 3u64;
                    let a1310 = 0u64;
                    nat_rem(a1308, a1309, a1310)
                };
                let v10: u64 = f97({
                    let a1314 = {
                        let a1312 = v5;
                        let a1313 = match v1.clone() {
                            Adt3::C0(h1311, _, _, _, _, _, _, _) => {
                                h1311
                            }
                        };
                        nat_mul(a1312, a1313)?
                    };
                    let a1315 = v8;
                    nat_add(a1314, a1315)?
                }, v1)?;
                let a1316 = v9;
                let a1317 = 0u64;
                if nat_eq(a1316, a1317) {
                    Ok(Adt7::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1318 = v9;
                    let a1319 = 1u64;
                    if nat_eq(a1318, a1319) {
                        let a1320 = v3;
                        let a1321 = 2u64;
                        if nat_eq(a1320, a1321) {
                            Ok(Adt7::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt7::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1322 = v3;
                        let a1323 = 2u64;
                        if if nat_eq(a1322, a1323) {
                            true
                        } else {
                            let a1324 = v3;
                            let a1325 = 3u64;
                            nat_eq(a1324, a1325)
                        } {
                            Ok(Adt7::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt7::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a1327 = 2u64;
        let a1328 = match v1.clone() {
            Adt3::C0(_, h1326, _, _, _, _, _, _) => {
                h1326
            }
        };
        nat_mul(a1327, a1328)?
    };
    let a1329 = v0;
    let a1330 = v5;
    if nat_lt(a1329, a1330) {
        let a1334 = {
            let a1331 = v0;
            let a1332 = 2u64;
            let a1333 = 0u64;
            nat_rem(a1331, a1332, a1333)
        };
        let a1335 = 0u64;
        if nat_eq(a1334, a1335) {
            Ok(Adt7::C4(19u64, 4u64, f97({
                let a1336 = v0;
                let a1337 = 2u64;
                let a1338 = 0u64;
                nat_quot(a1336, a1337, a1338)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1339 = v4;
            let a1340 = 2147483648u64;
            if if nat_lt(a1339, a1340) {
                let a1342 = v4;
                let a1343 = match v2 {
                    Adt3::C0(_, h1341, _, _, _, _, _, _) => {
                        h1341
                    }
                };
                nat_lt(a1342, a1343)
            } else {
                false
            } {
                Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1346 = match v2.clone() {
                Adt3::C0(h1344, _, _, _, _, _, _, _) => {
                    h1344
                }
            };
            let a1347 = match v2.clone() {
                Adt3::C0(_, h1345, _, _, _, _, _, _) => {
                    h1345
                }
            };
            nat_mul(a1346, a1347)?
        };
        let a1350 = {
            let a1348 = v0;
            let a1349 = v5;
            nat_sub(a1348, a1349)
        };
        let a1351 = v6;
        if nat_lt(a1350, a1351) {
            Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f97({
                let a1352 = v0;
                let a1353 = v5;
                nat_sub(a1352, a1353)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1356 = {
                    let a1354 = v0;
                    let a1355 = v5;
                    nat_sub(a1354, a1355)
                };
                let a1357 = v6;
                nat_sub(a1356, a1357)
            };
            let v8: u64 = {
                let a1361 = 1u64;
                let a1362 = {
                    let a1359 = 2u64;
                    let a1360 = match v1.clone() {
                        Adt3::C0(h1358, _, _, _, _, _, _, _) => {
                            h1358
                        }
                    };
                    nat_mul(a1359, a1360)?
                };
                nat_add(a1361, a1362)?
            };
            let v9: u64 = {
                let a1363 = v7;
                let a1364 = v8;
                let a1365 = 0u64;
                nat_quot(a1363, a1364, a1365)
            };
            let v10: u64 = {
                let a1366 = v7;
                let a1367 = v8;
                let a1368 = 0u64;
                nat_rem(a1366, a1367, a1368)
            };
            let a1369 = v10;
            let a1370 = 0u64;
            if nat_eq(a1369, a1370) {
                Ok(Adt7::C4(19u64, 4u64, f97(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1374 = {
                    let a1371 = v10;
                    let a1372 = 2u64;
                    let a1373 = 0u64;
                    nat_rem(a1371, a1372, a1373)
                };
                let a1375 = 1u64;
                if nat_eq(a1374, a1375) {
                    Ok(Adt7::C4(6u64, 0u64, f97({
                        let a1384 = {
                            let a1377 = v9;
                            let a1378 = match v1.clone() {
                                Adt3::C0(h1376, _, _, _, _, _, _, _) => {
                                    h1376
                                }
                            };
                            nat_mul(a1377, a1378)?
                        };
                        let a1385 = {
                            let a1381 = {
                                let a1379 = v10;
                                let a1380 = 1u64;
                                nat_sub(a1379, a1380)
                            };
                            let a1382 = 2u64;
                            let a1383 = 0u64;
                            nat_quot(a1381, a1382, a1383)
                        };
                        nat_add(a1384, a1385)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt7::C4(4u64, 3u64, f97({
                        let a1394 = {
                            let a1387 = v4;
                            let a1388 = match v1.clone() {
                                Adt3::C0(h1386, _, _, _, _, _, _, _) => {
                                    h1386
                                }
                            };
                            nat_mul(a1387, a1388)?
                        };
                        let a1395 = {
                            let a1391 = {
                                let a1389 = v10;
                                let a1390 = 1u64;
                                nat_sub(a1389, a1390)
                            };
                            let a1392 = 2u64;
                            let a1393 = 0u64;
                            nat_quot(a1391, a1392, a1393)
                        };
                        nat_add(a1394, a1395)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f97({
                        let a1404 = {
                            let a1397 = v4;
                            let a1398 = match v1 {
                                Adt3::C0(h1396, _, _, _, _, _, _, _) => {
                                    h1396
                                }
                            };
                            nat_mul(a1397, a1398)?
                        };
                        let a1405 = {
                            let a1401 = {
                                let a1399 = v10;
                                let a1400 = 1u64;
                                nat_sub(a1399, a1400)
                            };
                            let a1402 = 2u64;
                            let a1403 = 0u64;
                            nat_quot(a1401, a1402, a1403)
                        };
                        nat_add(a1404, a1405)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f57(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt7> {
    let a1406 = v1;
    let a1407 = f112(v2.clone(), v4.clone())?;
    if nat_lt(a1406, a1407) {
        let a1410 = v1;
        let a1411 = {
            let a1408 = 2u64;
            let a1409 = f89(v4.clone())?;
            nat_mul(a1408, a1409)?
        };
        if nat_lt(a1410, a1411) {
            let a1415 = {
                let a1412 = v1;
                let a1413 = 2u64;
                let a1414 = 0u64;
                nat_rem(a1412, a1413, a1414)
            };
            let a1416 = 0u64;
            if nat_eq(a1415, a1416) {
                f113({
                    let a1417 = v1;
                    let a1418 = 2u64;
                    let a1419 = 0u64;
                    nat_quot(a1417, a1418, a1419)
                }, v4)
            } else {
                if f114(match v2 {
                    Adt3::C0(_, h1420, _, _, _, _, _, _) => {
                        h1420
                    }
                }, v5) {
                    Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt7::C0(Adt8::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1423 = v1;
                let a1424 = {
                    let a1421 = 2u64;
                    let a1422 = f89(v4.clone())?;
                    nat_mul(a1421, a1422)?
                };
                nat_sub(a1423, a1424)
            };
            let v7: u64 = {
                let a1428 = v6;
                let a1429 = {
                    let a1426 = 1u64;
                    let a1427 = match v2.clone() {
                        Adt3::C0(h1425, _, _, _, _, _, _, _) => {
                            h1425
                        }
                    };
                    nat_add(a1426, a1427)?
                };
                let a1430 = 0u64;
                nat_quot(a1428, a1429, a1430)
            };
            let v8: u64 = {
                let a1434 = v6;
                let a1435 = {
                    let a1432 = 1u64;
                    let a1433 = match v2.clone() {
                        Adt3::C0(h1431, _, _, _, _, _, _, _) => {
                            h1431
                        }
                    };
                    nat_add(a1432, a1433)?
                };
                let a1436 = 0u64;
                nat_rem(a1434, a1435, a1436)
            };
            let a1437 = v8;
            let a1438 = 0u64;
            if nat_eq(a1437, a1438) {
                f113(v7, v4)
            } else {
                f115(v0, v7, {
                    let a1439 = v8;
                    let a1440 = 1u64;
                    nat_sub(a1439, a1440)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt7> {
    let v6: u64 = {
        let a1443 = match v4.clone() {
            Adt3::C0(h1441, _, _, _, _, _, _, _) => {
                h1441
            }
        };
        let a1444 = match v4.clone() {
            Adt3::C0(_, h1442, _, _, _, _, _, _) => {
                h1442
            }
        };
        nat_mul(a1443, a1444)?
    };
    let v7: u64 = {
        let a1445 = 2u64;
        let a1446 = v6;
        nat_mul(a1445, a1446)?
    };
    let v8: u64 = f116(match v3.clone() {
        Adt3::C0(_, h1447, _, _, _, _, _, _) => {
            h1447
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1448, _, _, _, _, _) => {
            h1448
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1449, _, _, _, _) => {
            h1449
        }
    });
    let a1450 = v0;
    let a1451 = v7;
    if nat_lt(a1450, a1451) {
        let a1455 = {
            let a1452 = v0;
            let a1453 = 2u64;
            let a1454 = 0u64;
            nat_rem(a1452, a1453, a1454)
        };
        let a1456 = 0u64;
        if nat_eq(a1455, a1456) {
            Ok(Adt7::C4(19u64, 5u64, f97({
                let a1457 = v0;
                let a1458 = 2u64;
                let a1459 = 0u64;
                nat_quot(a1457, a1458, a1459)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f117(v5, v8) {
                Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1460 = v0;
            let a1461 = v7;
            nat_sub(a1460, a1461)
        };
        let v10: u64 = {
            let a1465 = v9;
            let a1466 = {
                let a1463 = 1u64;
                let a1464 = match v1.clone() {
                    Adt3::C0(h1462, _, _, _, _, _, _, _) => {
                        h1462
                    }
                };
                nat_add(a1463, a1464)?
            };
            let a1467 = 0u64;
            nat_quot(a1465, a1466, a1467)
        };
        let v11: u64 = {
            let a1471 = v9;
            let a1472 = {
                let a1469 = 1u64;
                let a1470 = match v1.clone() {
                    Adt3::C0(h1468, _, _, _, _, _, _, _) => {
                        h1468
                    }
                };
                nat_add(a1469, a1470)?
            };
            let a1473 = 0u64;
            nat_rem(a1471, a1472, a1473)
        };
        let a1474 = v11;
        let a1475 = 0u64;
        if nat_eq(a1474, a1475) {
            Ok(Adt7::C4(19u64, 5u64, f97({
                let a1476 = v10;
                let a1477 = v6;
                let a1478 = 0u64;
                nat_rem(a1476, a1477, a1478)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f117(v5, v8) {
                let v12: u64 = {
                    let a1484 = {
                        let a1480 = v10;
                        let a1481 = match v1.clone() {
                            Adt3::C0(h1479, _, _, _, _, _, _, _) => {
                                h1479
                            }
                        };
                        nat_mul(a1480, a1481)?
                    };
                    let a1485 = {
                        let a1482 = v11;
                        let a1483 = 1u64;
                        nat_sub(a1482, a1483)
                    };
                    nat_add(a1484, a1485)?
                };
                let a1486 = v5;
                let a1487 = 4294967295u64;
                if nat_eq(a1486, a1487) {
                    Ok(Adt7::C1(false, f97(v12, v1)?, f97(v12, v2)?))
                } else {
                    Ok(Adt7::C4(8u64, 0u64, f97(v12, v1)?, 4u64, f97({
                        let a1493 = {
                            let a1489 = v5;
                            let a1490 = match v3.clone() {
                                Adt3::C0(h1488, _, _, _, _, _, _, _) => {
                                    h1488
                                }
                            };
                            nat_mul(a1489, a1490)?
                        };
                        let a1494 = {
                            let a1491 = v11;
                            let a1492 = 1u64;
                            nat_sub(a1491, a1492)
                        };
                        nat_add(a1493, a1494)?
                    }, v3)?, 3u64, f97(v12, v2)?))
                }
            } else {
                Ok(Adt7::C0(Adt8::C1))
            }
        }
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt7> {
    let v4: u64 = f118(v0, v2.clone(), 0u64);
    let v5: u64 = f118(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a1496 = v5;
        let a1497 = match v3.clone() {
            Adt5::C0(_, _, _, _, h1495, _) => {
                h1495
            }
        };
        let a1498 = 0u64;
        nat_rem(a1496, a1497, a1498)
    };
    let v7: u64 = {
        let a1500 = v5;
        let a1501 = match v3.clone() {
            Adt5::C0(_, _, _, _, h1499, _) => {
                h1499
            }
        };
        let a1502 = 0u64;
        nat_quot(a1500, a1501, a1502)
    };
    let v8: Option<u64> = f119(match v3.clone() {
        Adt5::C0(h1503, _, _, _, _, _) => {
            h1503
        }
    }, match v1.clone() {
        Adt3::C0(h1504, _, _, _, _, _, _, _) => {
            h1504
        }
    }, f118(v0, v2.clone(), 1u64), {
        let a1508 = v4;
        let a1509 = {
            let m1507 = match v3.clone() {
                Adt5::C0(h1506, _, _, _, _, _) => {
                    h1506
                }
            };
            match m1507 {
                Adt6::C0(h1505, _, _, _) => {
                    h1505
                }
            }
        };
        let a1510 = 0u64;
        nat_rem(a1508, a1509, a1510)
    })?;
    let v9: u64 = {
        let a1514 = v4;
        let a1515 = {
            let m1513 = match v3.clone() {
                Adt5::C0(h1512, _, _, _, _, _) => {
                    h1512
                }
            };
            match m1513 {
                Adt6::C0(h1511, _, _, _) => {
                    h1511
                }
            }
        };
        let a1516 = 0u64;
        nat_quot(a1514, a1515, a1516)
    };
    let v10: Option<u64> = f119(match v3.clone() {
        Adt5::C0(_, h1517, _, _, _, _) => {
            h1517
        }
    }, match v1.clone() {
        Adt3::C0(_, h1518, _, _, _, _, _, _) => {
            h1518
        }
    }, f118(v0, v2.clone(), 2u64), {
        let a1522 = v9;
        let a1523 = {
            let m1521 = match v3.clone() {
                Adt5::C0(_, h1520, _, _, _, _) => {
                    h1520
                }
            };
            match m1521 {
                Adt6::C0(h1519, _, _, _) => {
                    h1519
                }
            }
        };
        let a1524 = 0u64;
        nat_rem(a1522, a1523, a1524)
    })?;
    let v11: u64 = {
        let a1528 = v9;
        let a1529 = {
            let m1527 = match v3.clone() {
                Adt5::C0(_, h1526, _, _, _, _) => {
                    h1526
                }
            };
            match m1527 {
                Adt6::C0(h1525, _, _, _) => {
                    h1525
                }
            }
        };
        let a1530 = 0u64;
        nat_quot(a1528, a1529, a1530)
    };
    let v12: Option<u64> = f119(match v3.clone() {
        Adt5::C0(_, _, h1531, _, _, _) => {
            h1531
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1532, _, _, _, _, _) => {
            h1532
        }
    }, v6, {
        let a1536 = v11;
        let a1537 = {
            let m1535 = match v3.clone() {
                Adt5::C0(_, _, h1534, _, _, _) => {
                    h1534
                }
            };
            match m1535 {
                Adt6::C0(h1533, _, _, _) => {
                    h1533
                }
            }
        };
        let a1538 = 0u64;
        nat_rem(a1536, a1537, a1538)
    })?;
    let v13: u64 = {
        let a1542 = v11;
        let a1543 = {
            let m1541 = match v3.clone() {
                Adt5::C0(_, _, h1540, _, _, _) => {
                    h1540
                }
            };
            match m1541 {
                Adt6::C0(h1539, _, _, _) => {
                    h1539
                }
            }
        };
        let a1544 = 0u64;
        nat_quot(a1542, a1543, a1544)
    };
    let v17: Adt10 = {
        let m1545 = v8;
        match m1545 {
            None => {
                Adt10::C2(0u64)
            }
            Some(v14) => {
                let m1546 = v10;
                match m1546 {
                    None => {
                        Adt10::C2(0u64)
                    }
                    Some(v15) => {
                        let m1547 = v12;
                        match m1547 {
                            None => {
                                Adt10::C2(0u64)
                            }
                            Some(v16) => {
                                Adt10::C0(1u64, f120(v1, v14, v15, v16, {
                                    let a1551 = {
                                        let a1549 = v7;
                                        let a1550 = match v3.clone() {
                                            Adt5::C0(_, _, _, h1548, _, _) => {
                                                h1548
                                            }
                                        };
                                        nat_mul(a1549, a1550)?
                                    };
                                    let a1552 = v13;
                                    nat_add(a1551, a1552)?
                                })?)
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt7::C7(Adt9::C2(0u64, match v3.clone() {
        Adt5::C0(_, _, _, _, _, h1553) => {
            h1553
        }
    }, true), v17, Adt10::C2(0u64), Adt11::C1(f97(v0, v2)?), 4u64, {
        let a1555 = match v3 {
            Adt5::C0(_, _, _, _, _, h1554) => {
                h1554
            }
        };
        let a1556 = 0u64;
        if nat_eq(a1555, a1556) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt7> {
    let v8: u64 = {
        let a1559 = v6;
        let a1560 = {
            let a1557 = 3u64;
            let a1558 = v5;
            nat_add(a1557, a1558)?
        };
        nat_sub(a1559, a1560)
    };
    let a1562 = v0;
    let a1563 = f26(v3, match v1.clone() {
        Adt3::C0(h1561, _, _, _, _, _, _, _) => {
            h1561
        }
    }, v5, v8)?;
    if nat_lt(a1562, a1563) {
        let v9: u64 = f69(match v1.clone() {
            Adt3::C0(h1564, _, _, _, _, _, _, _) => {
                h1564
            }
        }, v5, v8)?;
        let v10: u64 = f70(v3, match v1.clone() {
            Adt3::C0(h1565, _, _, _, _, _, _, _) => {
                h1565
            }
        }, v5)?;
        let a1566 = v0;
        let a1567 = v9;
        if nat_lt(a1566, a1567) {
            f121(v0, v2, f20(v3), {
                let a1568 = 3u64;
                let a1569 = v5;
                nat_add(a1568, a1569)?
            }, v7)
        } else {
            let a1572 = v0;
            let a1573 = {
                let a1570 = v9;
                let a1571 = v10;
                nat_add(a1570, a1571)?
            };
            if nat_lt(a1572, a1573) {
                let v11: u64 = {
                    let a1574 = v0;
                    let a1575 = v9;
                    nat_sub(a1574, a1575)
                };
                let v12: u64 = f122(v3, match v1.clone() {
                    Adt3::C0(h1576, _, _, _, _, _, _, _) => {
                        h1576
                    }
                })?;
                f123({
                    let a1577 = v11;
                    let a1578 = v12;
                    let a1579 = 0u64;
                    nat_rem(a1577, a1578, a1579)
                }, {
                    let a1580 = v11;
                    let a1581 = v12;
                    let a1582 = 0u64;
                    nat_quot(a1580, a1581, a1582)
                }, v1, v3, v4, v7)
            } else {
                f124({
                    let a1585 = v0;
                    let a1586 = {
                        let a1583 = v9;
                        let a1584 = v10;
                        nat_add(a1583, a1584)?
                    };
                    nat_sub(a1585, a1586)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt7> {
    let a1587 = v0;
    let a1588 = f13(v1)?;
    if nat_lt(a1587, a1588) {
        let a1589 = v0;
        let a1590 = 4u64;
        if nat_lt(a1589, a1590) {
            let a1591 = v0;
            let a1592 = 0u64;
            if nat_eq(a1591, a1592) {
                Ok(Adt7::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1593 = v0;
                let a1594 = 1u64;
                if nat_eq(a1593, a1594) {
                    let a1595 = v5;
                    let a1596 = 0u64;
                    if nat_eq(a1595, a1596) {
                        Ok(Adt7::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C0(Adt8::C1))
                    }
                } else {
                    let a1597 = v0;
                    let a1598 = 2u64;
                    if nat_eq(a1597, a1598) {
                        Ok(Adt7::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1599 = v0;
            let a1600 = 4u64;
            if if nat_eq(a1599, a1600) {
                let a1601 = f125(v5)?;
                let a1602 = v1;
                !nat_eq(a1601, a1602)
            } else {
                false
            } {
                Ok(Adt7::C0(Adt8::C1))
            } else {
                let v6: u64 = {
                    let a1605 = {
                        let a1603 = v0;
                        let a1604 = 4u64;
                        nat_sub(a1603, a1604)
                    };
                    let a1606 = 3u64;
                    let a1607 = 0u64;
                    nat_quot(a1605, a1606, a1607)
                };
                let v7: u64 = {
                    let a1610 = {
                        let a1608 = v0;
                        let a1609 = 4u64;
                        nat_sub(a1608, a1609)
                    };
                    let a1611 = 3u64;
                    let a1612 = 0u64;
                    nat_rem(a1610, a1611, a1612)
                };
                let a1613 = v7;
                let a1614 = 0u64;
                if nat_eq(a1613, a1614) {
                    Ok(Adt7::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1615 = v7;
                    let a1616 = 1u64;
                    if nat_eq(a1615, a1616) {
                        Ok(Adt7::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt7::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f12(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f62(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt9 {
    let a1617 = v1;
    let a1618 = 0u64;
    if nat_eq(a1617, a1618) {
        if v0 {
            Adt9::C2(v3, v4, false)
        } else {
            Adt9::C0(v2)
        }
    } else {
        let a1619 = v1;
        let a1620 = 1u64;
        if nat_eq(a1619, a1620) {
            Adt9::C1(v2)
        } else {
            let a1621 = v1;
            let a1622 = 2u64;
            if nat_eq(a1621, a1622) {
                Adt9::C5(v2)
            } else {
                let a1623 = v1;
                let a1624 = 3u64;
                if nat_eq(a1623, a1624) {
                    Adt9::C3(v2, v2, false)
                } else {
                    let a1625 = v1;
                    let a1626 = 4u64;
                    if nat_eq(a1625, a1626) {
                        Adt9::C2(v3, v4, false)
                    } else {
                        Adt9::C6
                    }
                }
            }
        }
    }
}

pub fn f63(v0: u64, v1: u64) -> Adt10 {
    let a1627 = v0;
    let a1628 = 0u64;
    if nat_eq(a1627, a1628) {
        Adt10::C0(0u64, v1)
    } else {
        let a1629 = v0;
        let a1630 = 1u64;
        if nat_eq(a1629, a1630) {
            Adt10::C1(v1)
        } else {
            let a1631 = v0;
            let a1632 = 2u64;
            if nat_eq(a1631, a1632) {
                Adt10::C2(v1)
            } else {
                let a1633 = v0;
                let a1634 = 3u64;
                if nat_eq(a1633, a1634) {
                    Adt10::C3(v1)
                } else {
                    let a1635 = v0;
                    let a1636 = 13u64;
                    if nat_lt(a1635, a1636) {
                        Adt10::C0({
                            let a1637 = v0;
                            let a1638 = 3u64;
                            nat_sub(a1637, a1638)
                        }, v1)
                    } else {
                        Adt10::C4
                    }
                }
            }
        }
    }
}

pub fn f64(v0: u64, v1: u64) -> Adt11 {
    let a1639 = v0;
    let a1640 = 1u64;
    if nat_eq(a1639, a1640) {
        Adt11::C0(v1)
    } else {
        let a1641 = v0;
        let a1642 = 3u64;
        if if nat_eq(a1641, a1642) {
            true
        } else {
            let a1643 = v0;
            let a1644 = 6u64;
            nat_eq(a1643, a1644)
        } {
            Adt11::C1(v1)
        } else {
            Adt11::C2
        }
    }
}

pub fn f65(v0: u64) -> u64 {
    let v1: u64 = {
        let a1645 = v0;
        let a1646 = 4294967296u64;
        let a1647 = 0u64;
        nat_rem(a1645, a1646, a1647)
    };
    let a1648 = v1;
    let a1649 = 2147483648u64;
    if nat_lt(a1648, a1649) {
        v1
    } else {
        0u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1652 = 2u64;
    let a1653 = {
        let a1650 = 2u64;
        let a1651 = v0;
        nat_mul(a1650, a1651)?
    };
    nat_add(a1652, a1653)
}

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f126(v0, v3, v6) {
        f126(v1, v4, v6)
    } else {
        false
    } {
        f126(v2, v5, v6)
    } else {
        false
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f127(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f127(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f127(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f127(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f127(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f127(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1654 = v2;
    let a1655 = 0u64;
    if nat_eq(a1654, a1655) {
        Ok(0u64)
    } else {
        let a1658 = 2u64;
        let a1659 = {
            let a1656 = v0;
            let a1657 = v1;
            nat_mul(a1656, a1657)?
        };
        nat_mul(a1658, a1659)
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1660 = v2;
    let a1661 = f122(v0, v1)?;
    nat_mul(a1660, a1661)
}

pub fn f71(v0: u64) -> u64 {
    let a1662 = v0;
    let a1663 = 30u64;
    if nat_eq(a1662, a1663) {
        3u64
    } else {
        1u64
    }
}

pub fn f72(v0: u64) -> R<u64> {
    let a1666 = 5u64;
    let a1667 = {
        let a1664 = 4u64;
        let a1665 = v0;
        nat_mul(a1664, a1665)?
    };
    nat_add(a1666, a1667)
}

pub fn f73(v0: Adt6, v1: u64) -> R<u64> {
    let a1669 = 0u64;
    let a1670 = match v0.clone() {
        Adt6::C0(h1668, _, _, _) => {
            h1668
        }
    };
    if if if if if nat_lt(a1669, a1670) {
        let a1672 = 0u64;
        let a1673 = match v0.clone() {
            Adt6::C0(_, h1671, _, _) => {
                h1671
            }
        };
        nat_lt(a1672, a1673)
    } else {
        false
    } {
        let a1675 = 0u64;
        let a1676 = match v0.clone() {
            Adt6::C0(_, _, _, h1674) => {
                h1674
            }
        };
        nat_lt(a1675, a1676)
    } else {
        false
    } {
        let a1678 = match v0.clone() {
            Adt6::C0(_, h1677, _, _) => {
                h1677
            }
        };
        let a1679 = 2147483648u64;
        nat_lt(a1678, a1679)
    } else {
        false
    } {
        let a1681 = match v0.clone() {
            Adt6::C0(_, _, _, h1680) => {
                h1680
            }
        };
        let a1682 = 2147483648u64;
        nat_lt(a1681, a1682)
    } else {
        false
    } {
        let v2: u64 = {
            let a1686 = v1;
            let a1687 = {
                let a1684 = 2u64;
                let a1685 = f128(match v0.clone() {
                    Adt6::C0(_, _, h1683, _) => {
                        h1683
                    }
                });
                nat_mul(a1684, a1685)?
            };
            nat_add(a1686, a1687)?
        };
        let v3: u64 = {
            let a1699 = {
                let a1694 = 1u64;
                let a1695 = {
                    let a1692 = match v0.clone() {
                        Adt6::C0(_, _, _, h1688) => {
                            h1688
                        }
                    };
                    let a1693 = {
                        let a1690 = match v0.clone() {
                            Adt6::C0(h1689, _, _, _) => {
                                h1689
                            }
                        };
                        let a1691 = 1u64;
                        nat_sub(a1690, a1691)
                    };
                    nat_mul(a1692, a1693)?
                };
                nat_add(a1694, a1695)?
            };
            let a1700 = {
                let a1697 = 2u64;
                let a1698 = f129(match v0.clone() {
                    Adt6::C0(_, _, h1696, _) => {
                        h1696
                    }
                });
                nat_mul(a1697, a1698)?
            };
            nat_add(a1699, a1700)?
        };
        let a1701 = v3;
        let a1702 = v2;
        if nat_le(a1701, a1702) {
            let a1709 = 1u64;
            let a1710 = {
                let a1706 = {
                    let a1703 = v2;
                    let a1704 = v3;
                    nat_sub(a1703, a1704)
                };
                let a1707 = match v0 {
                    Adt6::C0(_, h1705, _, _) => {
                        h1705
                    }
                };
                let a1708 = 0u64;
                nat_quot(a1706, a1707, a1708)
            };
            nat_add(a1709, a1710)
        } else {
            let a1714 = {
                let a1711 = v3;
                let a1712 = v2;
                nat_sub(a1711, a1712)
            };
            let a1715 = match v0 {
                Adt6::C0(_, h1713, _, _) => {
                    h1713
                }
            };
            if nat_lt(a1714, a1715) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f74(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if f130(match v0.clone() {
        Adt2::C0(h1716, _, _, _, _, _, _, _, _, _, _) => {
            h1716
        }
    }, match v0.clone() {
        Adt2::C0(_, h1717, _, _, _, _, _, _, _, _, _) => {
            h1717
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1718, _, _, _, _, _, _, _, _) => {
            h1718
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1719, _, _, _, _, _, _, _) => {
            h1719
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1720, _, _, _, _, _, _) => {
            h1720
        }
    }, match v1.clone() {
        Adt2::C0(h1721, _, _, _, _, _, _, _, _, _, _) => {
            h1721
        }
    }, match v1.clone() {
        Adt2::C0(_, h1722, _, _, _, _, _, _, _, _, _) => {
            h1722
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1723, _, _, _, _, _, _, _, _) => {
            h1723
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1724, _, _, _, _, _, _, _) => {
            h1724
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1725, _, _, _, _, _, _) => {
            h1725
        }
    }, match v2.clone() {
        Adt2::C0(h1726, _, _, _, _, _, _, _, _, _, _) => {
            h1726
        }
    }, match v2.clone() {
        Adt2::C0(_, h1727, _, _, _, _, _, _, _, _, _) => {
            h1727
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1728, _, _, _, _, _, _, _, _) => {
            h1728
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1729, _, _, _, _, _, _, _) => {
            h1729
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1730, _, _, _, _, _, _) => {
            h1730
        }
    }) {
        let m1732 = f78(match v0.clone() {
            Adt2::C0(h1731, _, _, _, _, _, _, _, _, _, _) => {
                h1731
            }
        });
        match m1732 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v3) => {
                let a1738 = {
                    let a1735 = match v0.clone() {
                        Adt2::C0(_, h1733, _, _, _, _, _, _, _, _, _) => {
                            h1733
                        }
                    };
                    let a1736 = match v3.clone() {
                        Adt13::C0(_, h1734, _) => {
                            h1734
                        }
                    };
                    let a1737 = 0u64;
                    nat_rem(a1735, a1736, a1737)
                };
                let a1739 = 0u64;
                if if nat_eq(a1738, a1739) {
                    if f131(match v0.clone() {
                        Adt2::C0(_, h1740, _, _, _, _, _, _, _, _, _) => {
                            h1740
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1741, _, _, _, _, _, _, _, _, _) => {
                            h1741
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1742, _, _, _, _, _, _, _, _) => {
                            h1742
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1743, _, _, _, _, _, _, _) => {
                            h1743
                        }
                    }) {
                        true
                    } else {
                        let a1745 = 4u64;
                        let a1746 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1744, _, _, _, _, _) => {
                                h1744
                            }
                        };
                        if if nat_le(a1745, a1746) {
                            let a1748 = match v3.clone() {
                                Adt13::C0(_, h1747, _) => {
                                    h1747
                                }
                            };
                            let a1749 = 1u64;
                            if nat_eq(a1748, a1749) {
                                true
                            } else {
                                let a1752 = match v3.clone() {
                                    Adt13::C0(_, _, h1750) => {
                                        h1750
                                    }
                                };
                                let a1753 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1751, _, _, _, _, _) => {
                                        h1751
                                    }
                                };
                                nat_le(a1752, a1753)
                            }
                        } else {
                            false
                        } {
                            if f17(v0.clone(), v2.clone())? {
                                f17(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f14(v0.clone(), match v3 {
                        Adt13::C0(_, h1754, _) => {
                            h1754
                        }
                    });
                    Ok(Adt8::C4(if f131(match v0.clone() {
                        Adt2::C0(_, h1755, _, _, _, _, _, _, _, _, _) => {
                            h1755
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1756, _, _, _, _, _, _, _, _, _) => {
                            h1756
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1757, _, _, _, _, _, _, _, _) => {
                            h1757
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1758, _, _, _, _, _, _, _) => {
                            h1758
                        }
                    }) {
                        0u64
                    } else {
                        f112(v4.clone(), f14(v1.clone(), 1u64))?
                    }, 4u64, v4, f14(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1759, _, _, _, _, _, _, _, _, _, _) => {
                            h1759
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f75(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt8> {
    if if if if if if if if if if if if f77(match v0.clone() {
        Adt2::C0(h1760, _, _, _, _, _, _, _, _, _, _) => {
            h1760
        }
    }) {
        let a1762 = match v1.clone() {
            Adt2::C0(h1761, _, _, _, _, _, _, _, _, _, _) => {
                h1761
            }
        };
        let a1763 = 26u64;
        nat_eq(a1762, a1763)
    } else {
        false
    } {
        let a1765 = match v2.clone() {
            Adt2::C0(h1764, _, _, _, _, _, _, _, _, _, _) => {
                h1764
            }
        };
        let a1766 = 0u64;
        nat_eq(a1765, a1766)
    } else {
        false
    } {
        let a1769 = match v0.clone() {
            Adt2::C0(_, h1767, _, _, _, _, _, _, _, _, _) => {
                h1767
            }
        };
        let a1770 = match v2.clone() {
            Adt2::C0(_, h1768, _, _, _, _, _, _, _, _, _) => {
                h1768
            }
        };
        nat_eq(a1769, a1770)
    } else {
        false
    } {
        let a1773 = match v1.clone() {
            Adt2::C0(_, h1771, _, _, _, _, _, _, _, _, _) => {
                h1771
            }
        };
        let a1774 = match v0.clone() {
            Adt2::C0(_, _, h1772, _, _, _, _, _, _, _, _) => {
                h1772
            }
        };
        nat_le(a1773, a1774)
    } else {
        false
    } {
        let a1776 = match v0.clone() {
            Adt2::C0(_, _, _, h1775, _, _, _, _, _, _, _) => {
                h1775
            }
        };
        let a1777 = 1u64;
        nat_eq(a1776, a1777)
    } else {
        false
    } {
        let a1779 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1778, _, _, _, _, _, _) => {
                h1778
            }
        };
        let a1780 = 1u64;
        nat_eq(a1779, a1780)
    } else {
        false
    } {
        let a1782 = match v2.clone() {
            Adt2::C0(_, _, _, h1781, _, _, _, _, _, _, _) => {
                h1781
            }
        };
        let a1783 = 1u64;
        nat_eq(a1782, a1783)
    } else {
        false
    } {
        let a1785 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1784, _, _, _, _, _, _) => {
                h1784
            }
        };
        let a1786 = 1u64;
        nat_eq(a1785, a1786)
    } else {
        false
    } {
        let a1788 = match v1.clone() {
            Adt2::C0(_, _, h1787, _, _, _, _, _, _, _, _) => {
                h1787
            }
        };
        let a1789 = 1u64;
        nat_eq(a1788, a1789)
    } else {
        false
    } {
        let a1791 = match v1.clone() {
            Adt2::C0(_, _, _, h1790, _, _, _, _, _, _, _) => {
                h1790
            }
        };
        let a1792 = 1u64;
        nat_eq(a1791, a1792)
    } else {
        false
    } {
        let a1794 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1793, _, _, _, _, _, _) => {
                h1793
            }
        };
        let a1795 = 1u64;
        nat_eq(a1794, a1795)
    } else {
        false
    } {
        if f24(match v2.clone() {
            Adt2::C0(_, h1796, _, _, _, _, _, _, _, _, _) => {
                h1796
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h1797, _, _, _, _, _, _, _, _) => {
                h1797
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h1798, _, _, _, _, _, _, _) => {
                h1798
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h1799, _, _, _, _, _, _) => {
                h1799
            }
        }) {
            Ok(Adt8::C4(0u64, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a1801 = match v1.clone() {
                Adt2::C0(_, h1800, _, _, _, _, _, _, _, _, _) => {
                    h1800
                }
            };
            let a1802 = 0u64;
            if nat_eq(a1801, a1802) {
                Ok(Adt8::C4(f10(v2.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a1804 = match v0.clone() {
                    Adt2::C0(_, h1803, _, _, _, _, _, _, _, _, _) => {
                        h1803
                    }
                };
                let a1805 = 2147483647u64;
                if if if if if if nat_le(a1804, a1805) {
                    let a1807 = match v1.clone() {
                        Adt2::C0(_, h1806, _, _, _, _, _, _, _, _, _) => {
                            h1806
                        }
                    };
                    let a1808 = 2147483647u64;
                    nat_le(a1807, a1808)
                } else {
                    false
                } {
                    let a1810 = 4u64;
                    let a1811 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h1809, _, _, _, _, _) => {
                            h1809
                        }
                    };
                    nat_le(a1810, a1811)
                } else {
                    false
                } {
                    let a1817 = {
                        let a1814 = match v2.clone() {
                            Adt2::C0(_, h1812, _, _, _, _, _, _, _, _, _) => {
                                h1812
                            }
                        };
                        let a1815 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1813, _, _, _, _, _) => {
                                h1813
                            }
                        };
                        nat_mul(a1814, a1815)?
                    };
                    let a1818 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1816, _, _, _, _) => {
                            h1816
                        }
                    };
                    nat_le(a1817, a1818)
                } else {
                    false
                } {
                    f17(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f17(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt8::C4({
                        let a1832 = {
                            let a1822 = {
                                let a1820 = 2u64;
                                let a1821 = match v1.clone() {
                                    Adt2::C0(_, h1819, _, _, _, _, _, _, _, _, _) => {
                                        h1819
                                    }
                                };
                                nat_mul(a1820, a1821)?
                            };
                            let a1823 = f10(v2.clone(), 1u64)?;
                            nat_add(a1822, a1823)?
                        };
                        let a1833 = {
                            let a1830 = match v1.clone() {
                                Adt2::C0(_, h1824, _, _, _, _, _, _, _, _, _) => {
                                    h1824
                                }
                            };
                            let a1831 = {
                                let a1828 = 1u64;
                                let a1829 = {
                                    let a1826 = 2u64;
                                    let a1827 = match v0.clone() {
                                        Adt2::C0(_, h1825, _, _, _, _, _, _, _, _, _) => {
                                            h1825
                                        }
                                    };
                                    nat_mul(a1826, a1827)?
                                };
                                nat_add(a1828, a1829)?
                            };
                            nat_mul(a1830, a1831)?
                        };
                        nat_add(a1832, a1833)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h1834, _, _, _, _, _, _, _, _, _) => {
                            h1834
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1835, _, _, _, _, _, _, _, _, _) => {
                            h1835
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h1836, _, _, _, _, _) => {
                            h1836
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1837, _, _, _, _) => {
                            h1837
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h1838, _, _, _) => {
                            h1838
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1839, _, _) => {
                            h1839
                        }
                    }), f14(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h1840, _, _, _, _, _, _, _, _, _, _) => {
                            h1840
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f76(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt8> {
    let a1843 = match v1.clone() {
        Adt2::C0(h1841, _, _, _, _, _, _, _, _, _, _) => {
            h1841
        }
    };
    let a1844 = match v3.clone() {
        Adt2::C0(h1842, _, _, _, _, _, _, _, _, _, _) => {
            h1842
        }
    };
    if if nat_eq(a1843, a1844) {
        let a1845 = v0;
        let a1846 = 33u64;
        if nat_eq(a1845, a1846) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1848 = f78(match v1.clone() {
            Adt2::C0(h1847, _, _, _, _, _, _, _, _, _, _) => {
                h1847
            }
        });
        match m1848 {
            Adt12::C0 => {
                Ok(Adt8::C0)
            }
            Adt12::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt13::C0(_, h1849, _) => {
                        h1849
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a1854 = {
                    let a1851 = match v1.clone() {
                        Adt2::C0(_, h1850, _, _, _, _, _, _, _, _, _) => {
                            h1850
                        }
                    };
                    let a1852 = v5;
                    let a1853 = 1u64;
                    nat_rem(a1851, a1852, a1853)
                };
                let a1855 = 0u64;
                if if if nat_eq(a1854, a1855) {
                    let a1860 = {
                        let a1857 = match v3.clone() {
                            Adt2::C0(_, h1856, _, _, _, _, _, _, _, _, _) => {
                                h1856
                            }
                        };
                        let a1858 = v5;
                        let a1859 = 1u64;
                        nat_rem(a1857, a1858, a1859)
                    };
                    let a1861 = 0u64;
                    nat_eq(a1860, a1861)
                } else {
                    false
                } {
                    let a1862 = v6;
                    let a1863 = f10(v3.clone(), v5)?;
                    nat_eq(a1862, a1863)
                } else {
                    false
                } {
                    Ok(Adt8::C4(v6, match v4 {
                        Adt13::C0(_, _, h1864) => {
                            h1864
                        }
                    }, f14(v1, v5), f14(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt8::C1)
                }
            }
        }
    } else {
        Ok(Adt8::C1)
    }
}

pub fn f77(v0: u64) -> bool {
    let a1865 = v0;
    let a1866 = 0u64;
    if if nat_eq(a1865, a1866) {
        true
    } else {
        let a1867 = v0;
        let a1868 = 1u64;
        nat_eq(a1867, a1868)
    } {
        true
    } else {
        let a1869 = v0;
        let a1870 = 30u64;
        nat_eq(a1869, a1870)
    }
}

pub fn f78(v0: u64) -> Adt12 {
    let a1871 = v0;
    let a1872 = 0u64;
    if nat_eq(a1871, a1872) {
        Adt12::C1(Adt13::C0(0u64, 1u64, 4u64))
    } else {
        let a1873 = v0;
        let a1874 = 1u64;
        if nat_eq(a1873, a1874) {
            Adt12::C1(Adt13::C0(1u64, 1u64, 2u64))
        } else {
            let a1875 = v0;
            let a1876 = 2u64;
            if nat_eq(a1875, a1876) {
                Adt12::C1(Adt13::C0(2u64, 32u64, 18u64))
            } else {
                let a1877 = v0;
                let a1878 = 3u64;
                if nat_eq(a1877, a1878) {
                    Adt12::C1(Adt13::C0(3u64, 32u64, 20u64))
                } else {
                    let a1879 = v0;
                    let a1880 = 6u64;
                    if nat_eq(a1879, a1880) {
                        Adt12::C1(Adt13::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1881 = v0;
                        let a1882 = 7u64;
                        if nat_eq(a1881, a1882) {
                            Adt12::C1(Adt13::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1883 = v0;
                            let a1884 = 8u64;
                            if nat_eq(a1883, a1884) {
                                Adt12::C1(Adt13::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1885 = v0;
                                let a1886 = 9u64;
                                if nat_eq(a1885, a1886) {
                                    Adt12::C1(Adt13::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1887 = v0;
                                    let a1888 = 10u64;
                                    if nat_eq(a1887, a1888) {
                                        Adt12::C1(Adt13::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1889 = v0;
                                        let a1890 = 11u64;
                                        if nat_eq(a1889, a1890) {
                                            Adt12::C1(Adt13::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1891 = v0;
                                            let a1892 = 12u64;
                                            if nat_eq(a1891, a1892) {
                                                Adt12::C1(Adt13::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1893 = v0;
                                                let a1894 = 13u64;
                                                if nat_eq(a1893, a1894) {
                                                    Adt12::C1(Adt13::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1895 = v0;
                                                    let a1896 = 14u64;
                                                    if nat_eq(a1895, a1896) {
                                                        Adt12::C1(Adt13::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1897 = v0;
                                                        let a1898 = 15u64;
                                                        if nat_eq(a1897, a1898) {
                                                            Adt12::C1(Adt13::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1899 = v0;
                                                            let a1900 = 16u64;
                                                            if nat_eq(a1899, a1900) {
                                                                Adt12::C1(Adt13::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1901 = v0;
                                                                let a1902 = 17u64;
                                                                if nat_eq(a1901, a1902) {
                                                                    Adt12::C1(Adt13::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1903 = v0;
                                                                    let a1904 = 18u64;
                                                                    if nat_eq(a1903, a1904) {
                                                                        Adt12::C1(Adt13::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1905 = v0;
                                                                        let a1906 = 19u64;
                                                                        if nat_eq(a1905, a1906) {
                                                                            Adt12::C1(Adt13::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1907 = v0;
                                                                            let a1908 = 20u64;
                                                                            if nat_eq(a1907, a1908) {
                                                                                Adt12::C1(Adt13::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1909 = v0;
                                                                                let a1910 = 21u64;
                                                                                if nat_eq(a1909, a1910) {
                                                                                    Adt12::C1(Adt13::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1911 = v0;
                                                                                    let a1912 = 22u64;
                                                                                    if nat_eq(a1911, a1912) {
                                                                                        Adt12::C1(Adt13::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1913 = v0;
                                                                                        let a1914 = 23u64;
                                                                                        if nat_eq(a1913, a1914) {
                                                                                            Adt12::C1(Adt13::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1915 = v0;
                                                                                            let a1916 = 24u64;
                                                                                            if nat_eq(a1915, a1916) {
                                                                                                Adt12::C1(Adt13::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1917 = v0;
                                                                                                let a1918 = 25u64;
                                                                                                if nat_eq(a1917, a1918) {
                                                                                                    Adt12::C1(Adt13::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1919 = v0;
                                                                                                    let a1920 = 26u64;
                                                                                                    if nat_eq(a1919, a1920) {
                                                                                                        Adt12::C1(Adt13::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1921 = v0;
                                                                                                        let a1922 = 27u64;
                                                                                                        if nat_eq(a1921, a1922) {
                                                                                                            Adt12::C1(Adt13::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1923 = v0;
                                                                                                            let a1924 = 28u64;
                                                                                                            if nat_eq(a1923, a1924) {
                                                                                                                Adt12::C1(Adt13::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1925 = v0;
                                                                                                                let a1926 = 29u64;
                                                                                                                if nat_eq(a1925, a1926) {
                                                                                                                    Adt12::C1(Adt13::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1927 = v0;
                                                                                                                    let a1928 = 30u64;
                                                                                                                    if nat_eq(a1927, a1928) {
                                                                                                                        Adt12::C1(Adt13::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1929 = v0;
                                                                                                                        let a1930 = 34u64;
                                                                                                                        if nat_eq(a1929, a1930) {
                                                                                                                            Adt12::C1(Adt13::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1931 = v0;
                                                                                                                            let a1932 = 35u64;
                                                                                                                            if nat_eq(a1931, a1932) {
                                                                                                                                Adt12::C1(Adt13::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1933 = v0;
                                                                                                                                let a1934 = 39u64;
                                                                                                                                if nat_eq(a1933, a1934) {
                                                                                                                                    Adt12::C1(Adt13::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1935 = v0;
                                                                                                                                    let a1936 = 40u64;
                                                                                                                                    if nat_eq(a1935, a1936) {
                                                                                                                                        Adt12::C1(Adt13::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1937 = v0;
                                                                                                                                        let a1938 = 41u64;
                                                                                                                                        if nat_eq(a1937, a1938) {
                                                                                                                                            Adt12::C1(Adt13::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1939 = v0;
                                                                                                                                            let a1940 = 42u64;
                                                                                                                                            if nat_eq(a1939, a1940) {
                                                                                                                                                Adt12::C1(Adt13::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1941 = v0;
                                                                                                                                                let a1942 = 43u64;
                                                                                                                                                if nat_eq(a1941, a1942) {
                                                                                                                                                    Adt12::C1(Adt13::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt12::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f24(v4, v5, v6, v7) {
        f24(v0, v1, v2, v3)
    } else {
        let a1946 = {
            let a1943 = v0;
            let a1944 = v4;
            let a1945 = 1u64;
            nat_rem(a1943, a1944, a1945)
        };
        let a1947 = 0u64;
        if if if nat_eq(a1946, a1947) {
            let a1951 = {
                let a1948 = v1;
                let a1949 = v5;
                let a1950 = 1u64;
                nat_rem(a1948, a1949, a1950)
            };
            let a1952 = 0u64;
            nat_eq(a1951, a1952)
        } else {
            false
        } {
            let a1956 = {
                let a1953 = v2;
                let a1954 = v6;
                let a1955 = 1u64;
                nat_rem(a1953, a1954, a1955)
            };
            let a1957 = 0u64;
            nat_eq(a1956, a1957)
        } else {
            false
        } {
            let a1961 = {
                let a1958 = v3;
                let a1959 = v7;
                let a1960 = 1u64;
                nat_rem(a1958, a1959, a1960)
            };
            let a1962 = 0u64;
            nat_eq(a1961, a1962)
        } else {
            false
        }
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1963 = v0;
    let a1964 = v1;
    if nat_eq(a1963, a1964) {
        let a1967 = v4;
        let a1968 = {
            let a1965 = v2;
            let a1966 = v3;
            nat_add(a1965, a1966)?
        };
        Ok(nat_eq(a1967, a1968))
    } else {
        let a1969 = v2;
        let a1970 = v3;
        if nat_eq(a1969, a1970) {
            let a1971 = v4;
            let a1972 = v2;
            Ok(nat_eq(a1971, a1972))
        } else {
            Ok(false)
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1975 = {
            let a1973 = v0;
            let a1974 = f128(v1);
            nat_add(a1973, a1974)?
        };
        let a1976 = f128(v2);
        nat_add(a1975, a1976)?
    };
    let v5: u64 = {
        let a1977 = f129(v1);
        let a1978 = f129(v2);
        nat_add(a1977, a1978)?
    };
    let a1979 = v1;
    let a1980 = 4294967295u64;
    if if nat_le(a1979, a1980) {
        let a1981 = v2;
        let a1982 = 4294967295u64;
        nat_le(a1981, a1982)
    } else {
        false
    } {
        let a1983 = v5;
        let a1984 = v4;
        if nat_le(a1983, a1984) {
            let a1987 = v3;
            let a1988 = {
                let a1985 = v4;
                let a1986 = v5;
                nat_sub(a1985, a1986)
            };
            Ok(nat_eq(a1987, a1988))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f82(v0: u64, v1: u64) -> R<bool> {
    let a1989 = v1;
    let a1990 = 4294967295u64;
    if nat_le(a1989, a1990) {
        let a1993 = {
            let a1991 = f128(v1);
            let a1992 = f129(v1);
            nat_add(a1991, a1992)?
        };
        let a1994 = v0;
        Ok(nat_lt(a1993, a1994))
    } else {
        Ok(false)
    }
}

pub fn f83(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f132(v0.clone(), false)?;
    let v6: u64 = f132(v0, true)?;
    let a1995 = v6;
    let a1996 = v1;
    if if nat_le(a1995, a1996) {
        let a2003 = {
            let a2001 = {
                let a1997 = v1;
                let a1998 = v5;
                nat_add(a1997, a1998)?
            };
            let a2002 = {
                let a1999 = v3;
                let a2000 = 0u64;
                if nat_eq(a1999, a2000) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2001, a2002)?
        };
        let a2004 = v2;
        nat_le(a2003, a2004)
    } else {
        false
    } {
        if v4 {
            let a2009 = {
                let a2007 = {
                    let a2005 = v1;
                    let a2006 = v5;
                    nat_add(a2005, a2006)?
                };
                let a2008 = v6;
                nat_sub(a2007, a2008)
            };
            let a2010 = v2;
            Ok(nat_lt(a2009, a2010))
        } else {
            let a2015 = {
                let a2013 = {
                    let a2011 = v1;
                    let a2012 = v5;
                    nat_add(a2011, a2012)?
                };
                let a2014 = v6;
                nat_sub(a2013, a2014)
            };
            let a2016 = v2;
            Ok(nat_le(a2015, a2016))
        }
    } else {
        Ok(false)
    }
}

pub fn f84(v0: u64, v1: u64) -> bool {
    let a2017 = v0;
    let a2018 = 0u64;
    if if nat_eq(a2017, a2018) {
        true
    } else {
        let a2019 = v0;
        let a2020 = 1u64;
        nat_eq(a2019, a2020)
    } {
        let a2021 = v1;
        let a2022 = 4294967296u64;
        nat_lt(a2021, a2022)
    } else {
        false
    }
}

pub fn f85(v0: u64, v1: u64) -> R<u64> {
    let a2027 = {
        let a2025 = v0;
        let a2026 = {
            let a2023 = v1;
            let a2024 = 1u64;
            nat_sub(a2023, a2024)
        };
        nat_add(a2025, a2026)?
    };
    let a2028 = v1;
    let a2029 = 0u64;
    Ok(nat_quot(a2027, a2028, a2029))
}

pub fn f86(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f24(match v0.clone() {
        Adt2::C0(_, h2030, _, _, _, _, _, _, _, _, _) => {
            h2030
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2031, _, _, _, _, _, _, _, _) => {
            h2031
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2032, _, _, _, _, _, _, _) => {
            h2032
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2033, _, _, _, _, _, _) => {
            h2033
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2036 = match v1.clone() {
                Adt2::C0(_, h2034, _, _, _, _, _, _, _, _, _) => {
                    h2034
                }
            };
            let a2037 = match v1.clone() {
                Adt2::C0(_, _, h2035, _, _, _, _, _, _, _, _) => {
                    h2035
                }
            };
            nat_mul(a2036, a2037)?
        };
        let v5: u64 = {
            let a2039 = v4;
            let a2040 = match v1.clone() {
                Adt2::C0(_, _, _, h2038, _, _, _, _, _, _, _) => {
                    h2038
                }
            };
            nat_mul(a2039, a2040)?
        };
        let a2053 = {
            let a2051 = {
                let a2042 = match v0.clone() {
                    Adt2::C0(_, h2041, _, _, _, _, _, _, _, _, _) => {
                        h2041
                    }
                };
                let a2043 = 1u64;
                nat_sub(a2042, a2043)
            };
            let a2052 = {
                let a2049 = f133(match v0.clone() {
                    Adt2::C0(_, _, h2044, _, _, _, _, _, _, _, _) => {
                        h2044
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2045, _, _, _, _, _, _, _, _, _) => {
                        h2045
                    }
                })?;
                let a2050 = f133(match v0 {
                    Adt2::C0(_, _, _, h2046, _, _, _, _, _, _, _) => {
                        h2046
                    }
                }, v2, {
                    let a2047 = v3;
                    let a2048 = v5;
                    nat_mul(a2047, a2048)?
                }, v4)?;
                nat_add(a2049, a2050)?
            };
            nat_add(a2051, a2052)?
        };
        let a2054 = f10(v1, 1u64)?;
        Ok(nat_lt(a2053, a2054))
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2055 = v0;
    let a2056 = 2147483648u64;
    if if if if nat_lt(a2055, a2056) {
        let a2057 = v1;
        let a2058 = 2147483648u64;
        nat_lt(a2057, a2058)
    } else {
        false
    } {
        let a2061 = {
            let a2059 = v0;
            let a2060 = v1;
            nat_mul(a2059, a2060)?
        };
        let a2062 = 2147483648u64;
        nat_lt(a2061, a2062)
    } else {
        false
    } {
        let a2067 = {
            let a2065 = {
                let a2063 = v0;
                let a2064 = 1u64;
                nat_sub(a2063, a2064)
            };
            let a2066 = v2;
            nat_mul(a2065, a2066)?
        };
        let a2068 = 2147483648u64;
        nat_lt(a2067, a2068)
    } else {
        false
    } {
        let a2073 = {
            let a2071 = {
                let a2069 = v1;
                let a2070 = 1u64;
                nat_sub(a2069, a2070)
            };
            let a2072 = v2;
            nat_mul(a2071, a2072)?
        };
        let a2074 = 2147483648u64;
        Ok(nat_lt(a2073, a2074))
    } else {
        Ok(false)
    }
}

pub fn f88(v0: u64) -> R<u64> {
    let a2077 = 3u64;
    let a2078 = {
        let a2075 = v0;
        let a2076 = 3u64;
        nat_mul(a2075, a2076)?
    };
    nat_add(a2077, a2078)
}

pub fn f89(v0: Adt3) -> R<u64> {
    if f24(match v0.clone() {
        Adt3::C0(h2079, _, _, _, _, _, _, _) => {
            h2079
        }
    }, match v0.clone() {
        Adt3::C0(_, h2080, _, _, _, _, _, _) => {
            h2080
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2081, _, _, _, _, _) => {
            h2081
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2082, _, _, _, _) => {
            h2082
        }
    }) {
        Ok(0u64)
    } else {
        let a2091 = match v0.clone() {
            Adt3::C0(h2083, _, _, _, _, _, _, _) => {
                h2083
            }
        };
        let a2092 = {
            let a2089 = match v0.clone() {
                Adt3::C0(_, h2084, _, _, _, _, _, _) => {
                    h2084
                }
            };
            let a2090 = {
                let a2087 = match v0.clone() {
                    Adt3::C0(_, _, h2085, _, _, _, _, _) => {
                        h2085
                    }
                };
                let a2088 = match v0 {
                    Adt3::C0(_, _, _, h2086, _, _, _, _) => {
                        h2086
                    }
                };
                nat_mul(a2087, a2088)?
            };
            nat_mul(a2089, a2090)?
        };
        nat_mul(a2091, a2092)
    }
}

pub fn f90(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2093 = v1;
    let a2094 = 13u64;
    if if nat_eq(a2093, a2094) {
        f24(match v0.clone() {
            Adt3::C0(h2095, _, _, _, _, _, _, _) => {
                h2095
            }
        }, match v0.clone() {
            Adt3::C0(_, h2096, _, _, _, _, _, _) => {
                h2096
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2097, _, _, _, _, _) => {
                h2097
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2098, _, _, _, _) => {
                h2098
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2104 = match v0.clone() {
                Adt3::C0(_, h2099, _, _, _, _, _, _) => {
                    h2099
                }
            };
            let a2105 = {
                let a2102 = match v0.clone() {
                    Adt3::C0(_, _, h2100, _, _, _, _, _) => {
                        h2100
                    }
                };
                let a2103 = match v0.clone() {
                    Adt3::C0(_, _, _, h2101, _, _, _, _) => {
                        h2101
                    }
                };
                nat_mul(a2102, a2103)?
            };
            nat_mul(a2104, a2105)?
        };
        let a2106 = v1;
        let a2107 = 13u64;
        if nat_eq(a2106, a2107) {
            let a2116 = 2u64;
            let a2117 = if f134(v0.clone(), v2) {
                let a2109 = v3;
                let a2110 = f135(match v0 {
                    Adt3::C0(h2108, _, _, _, _, _, _, _) => {
                        h2108
                    }
                }, false)?;
                nat_mul(a2109, a2110)?
            } else {
                let a2114 = v3;
                let a2115 = {
                    let a2112 = match v0 {
                        Adt3::C0(h2111, _, _, _, _, _, _, _) => {
                            h2111
                        }
                    };
                    let a2113 = 2u64;
                    nat_mul(a2112, a2113)?
                };
                nat_mul(a2114, a2115)?
            };
            nat_add(a2116, a2117)
        } else {
            let a2121 = v3;
            let a2122 = f135(match v0 {
                Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                    h2118
                }
            }, {
                let a2119 = v1;
                let a2120 = 16u64;
                nat_eq(a2119, a2120)
            })?;
            nat_mul(a2121, a2122)
        }
    }
}

pub fn f91(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt8> {
    let a2123 = v0;
    let a2124 = 30u64;
    if if nat_eq(a2123, a2124) {
        let a2127 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2125, _, _, _, _) => {
                h2125
            }
        };
        let a2128 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2126, _, _, _, _, _) => {
                h2126
            }
        };
        nat_lt(a2127, a2128)
    } else {
        false
    } {
        Ok(Adt8::C1)
    } else {
        let m2129 = v3;
        match m2129 {
            None => {
                f136(v0, v1, match v2.clone() {
                    Adt2::C0(h2130, _, _, _, _, _, _, _, _, _, _) => {
                        h2130
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2131, _, _, _, _, _, _, _, _, _, _) => {
                        h2131
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2132, _, _, _, _, _, _, _, _, _) => {
                        h2132
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2133, _, _, _, _, _, _, _, _) => {
                        h2133
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2134, _, _, _, _, _, _, _) => {
                        h2134
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2135, _, _, _, _, _, _) => {
                        h2135
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2136, _, _, _, _, _, _, _, _, _) => {
                        h2136
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2137, _, _, _, _, _, _, _, _) => {
                        h2137
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2138, _, _, _, _, _, _, _) => {
                        h2138
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2139, _, _, _, _, _, _) => {
                        h2139
                    }
                })
            }
            Some(v5) => {
                f136(v0, v1, match v2.clone() {
                    Adt2::C0(h2140, _, _, _, _, _, _, _, _, _, _) => {
                        h2140
                    }
                }, match v5.clone() {
                    Adt2::C0(h2141, _, _, _, _, _, _, _, _, _, _) => {
                        h2141
                    }
                }, match v4.clone() {
                    Adt2::C0(h2142, _, _, _, _, _, _, _, _, _, _) => {
                        h2142
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2143, _, _, _, _, _, _, _, _, _) => {
                        h2143
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2144, _, _, _, _, _, _, _, _) => {
                        h2144
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2145, _, _, _, _, _, _, _) => {
                        h2145
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2146, _, _, _, _, _, _) => {
                        h2146
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2147, _, _, _, _, _, _, _, _, _) => {
                        h2147
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2148, _, _, _, _, _, _, _, _) => {
                        h2148
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2149, _, _, _, _, _, _, _) => {
                        h2149
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2150, _, _, _, _, _, _) => {
                        h2150
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2151, _, _, _, _, _, _, _, _, _) => {
                        h2151
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2152, _, _, _, _, _, _, _, _) => {
                        h2152
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2153, _, _, _, _, _, _, _) => {
                        h2153
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2154, _, _, _, _, _, _) => {
                        h2154
                    }
                })
            }
        }
    }
}

pub fn f92(v0: Adt9, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt7 {
    let v9: u64 = {
        let a2155 = v7;
        let a2156 = 6u64;
        if nat_eq(a2155, a2156) {
            27u64
        } else {
            v2
        }
    };
    Adt7::C7(v0, f63(v3, v4), f63(v5, v6), f64(v7, v8), f137(v1), f137(v9))
}

pub fn f93(v0: Adt4, v1: u64, v2: u64) -> Adt9 {
    let m2157 = v0;
    match m2157 {
        Adt4::C0 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C13(v30, v31, v32, v33) => {
            Adt9::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a2158 = 13u64;
            let a2159 = v34;
            if nat_le(a2158, a2159) {
                Adt9::C7(v34, v1, v2)
            } else {
                Adt9::C3(v1, v2, true)
            }
        }
        Adt4::C15 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C24(_) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C25(_, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C26(_, _, _, _) => {
            Adt9::C2(v1, v2, true)
        }
        Adt4::C27(v50) => {
            Adt9::C2(0u64, match v50 {
                Adt5::C0(_, _, _, _, _, h2160) => {
                    h2160
                }
            }, true)
        }
    }
}

pub fn f94(v0: Adt4) -> u64 {
    let m2161 = v0;
    match m2161 {
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
    }
}

pub fn f95(v0: Adt4, v1: u64) -> u64 {
    let m2162 = v0;
    match m2162 {
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
            let a2163 = v1;
            let a2164 = 6u64;
            if nat_eq(a2163, a2164) {
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
        Adt4::C26(v50, _, _, _) => {
            let a2166 = v1;
            let a2167 = 6u64;
            if if if nat_eq(a2166, a2167) {
                true
            } else {
                let a2168 = v1;
                let a2169 = 7u64;
                nat_eq(a2168, a2169)
            } {
                let a2170 = v50;
                let a2171 = 30u64;
                nat_eq(a2170, a2171)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v49) => {
            match v49 {
                Adt5::C0(_, _, _, _, _, h2165) => {
                    h2165
                }
            }
        }
    }
}

pub fn f96(v0: Adt4, v1: u64) -> Adt9 {
    let a2172 = v1;
    let a2173 = 4u64;
    if nat_lt(a2172, a2173) {
        Adt9::C0(v1)
    } else {
        let a2174 = v1;
        let a2175 = 4u64;
        if nat_eq(a2174, a2175) {
            Adt9::C1(0u64)
        } else {
            let a2176 = v1;
            let a2177 = 5u64;
            if nat_eq(a2176, a2177) {
                Adt9::C1(3u64)
            } else {
                let a2178 = v1;
                let a2179 = 6u64;
                if nat_eq(a2178, a2179) {
                    Adt9::C2(f95(v0, v1), 0u64, false)
                } else {
                    let a2180 = v1;
                    let a2181 = 7u64;
                    if nat_eq(a2180, a2181) {
                        Adt9::C2(0u64, f95(v0, v1), false)
                    } else {
                        let a2182 = v1;
                        let a2183 = 16u64;
                        if nat_lt(a2182, a2183) {
                            Adt9::C1({
                                let a2184 = v1;
                                let a2185 = 8u64;
                                nat_sub(a2184, a2185)
                            })
                        } else {
                            let a2186 = v1;
                            let a2187 = 20u64;
                            if nat_lt(a2186, a2187) {
                                Adt9::C5({
                                    let a2188 = v1;
                                    let a2189 = 16u64;
                                    nat_sub(a2188, a2189)
                                })
                            } else {
                                let a2190 = v1;
                                let a2191 = 20u64;
                                if nat_eq(a2190, a2191) {
                                    Adt9::C3(0u64, 0u64, false)
                                } else {
                                    let a2192 = v1;
                                    let a2193 = 21u64;
                                    if nat_eq(a2192, a2193) {
                                        Adt9::C0(4u64)
                                    } else {
                                        let a2194 = v1;
                                        let a2195 = 22u64;
                                        if nat_eq(a2194, a2195) {
                                            Adt9::C0(5u64)
                                        } else {
                                            Adt9::C6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f97(v0: u64, v1: Adt3) -> R<u64> {
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
    f138({
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

pub fn f98(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2241 = {
        let a2236 = f139(v2, {
            let a2227 = {
                let a2224 = v0;
                let a2225 = v1;
                let a2226 = 0u64;
                nat_quot(a2224, a2225, a2226)
            };
            let a2228 = v2;
            let a2229 = 0u64;
            nat_rem(a2227, a2228, a2229)
        }, {
            let a2233 = {
                let a2230 = v0;
                let a2231 = v1;
                let a2232 = 0u64;
                nat_quot(a2230, a2231, a2232)
            };
            let a2234 = v2;
            let a2235 = 0u64;
            nat_quot(a2233, a2234, a2235)
        })?;
        let a2237 = v1;
        nat_mul(a2236, a2237)?
    };
    let a2242 = {
        let a2238 = v0;
        let a2239 = v1;
        let a2240 = 0u64;
        nat_rem(a2238, a2239, a2240)
    };
    nat_add(a2241, a2242)
}

pub fn f99(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2244 = v0;
        let a2245 = match v2.clone() {
            Adt3::C0(h2243, _, _, _, _, _, _, _) => {
                h2243
            }
        };
        let a2246 = 0u64;
        nat_quot(a2244, a2245, a2246)
    };
    let v4: u64 = {
        let a2248 = v3;
        let a2249 = match v2.clone() {
            Adt3::C0(_, h2247, _, _, _, _, _, _) => {
                h2247
            }
        };
        let a2250 = 0u64;
        nat_quot(a2248, a2249, a2250)
    };
    f140({
        let a2252 = v0;
        let a2253 = match v2.clone() {
            Adt3::C0(h2251, _, _, _, _, _, _, _) => {
                h2251
            }
        };
        let a2254 = 0u64;
        nat_rem(a2252, a2253, a2254)
    }, {
        let a2256 = v3;
        let a2257 = match v2.clone() {
            Adt3::C0(_, h2255, _, _, _, _, _, _) => {
                h2255
            }
        };
        let a2258 = 0u64;
        nat_rem(a2256, a2257, a2258)
    }, {
        let a2260 = v4;
        let a2261 = match v2.clone() {
            Adt3::C0(_, _, h2259, _, _, _, _, _) => {
                h2259
            }
        };
        let a2262 = 0u64;
        nat_rem(a2260, a2261, a2262)
    }, {
        let a2264 = v4;
        let a2265 = match v2 {
            Adt3::C0(_, _, h2263, _, _, _, _, _) => {
                h2263
            }
        };
        let a2266 = 0u64;
        nat_quot(a2264, a2265, a2266)
    }, match v1.clone() {
        Adt3::C0(h2267, _, _, _, _, _, _, _) => {
            h2267
        }
    }, match v1.clone() {
        Adt3::C0(_, h2268, _, _, _, _, _, _) => {
            h2268
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2269, _, _, _, _, _) => {
            h2269
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2270, _, _, _, _) => {
            h2270
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2271, _, _, _) => {
            h2271
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2272, _, _) => {
            h2272
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2273, _) => {
            h2273
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2274) => {
            h2274
        }
    })
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a2276 = v0;
        let a2277 = match v3.clone() {
            Adt3::C0(h2275, _, _, _, _, _, _, _) => {
                h2275
            }
        };
        let a2278 = 0u64;
        nat_quot(a2276, a2277, a2278)
    };
    let v6: u64 = {
        let a2280 = v5;
        let a2281 = match v3.clone() {
            Adt3::C0(_, h2279, _, _, _, _, _, _) => {
                h2279
            }
        };
        let a2282 = 0u64;
        nat_quot(a2280, a2281, a2282)
    };
    let v7: u64 = {
        let a2284 = v0;
        let a2285 = match v3.clone() {
            Adt3::C0(h2283, _, _, _, _, _, _, _) => {
                h2283
            }
        };
        let a2286 = 0u64;
        nat_rem(a2284, a2285, a2286)
    };
    let v8: u64 = {
        let a2288 = v5;
        let a2289 = match v3.clone() {
            Adt3::C0(_, h2287, _, _, _, _, _, _) => {
                h2287
            }
        };
        let a2290 = 0u64;
        nat_rem(a2288, a2289, a2290)
    };
    let v9: u64 = {
        let a2292 = v6;
        let a2293 = match v3.clone() {
            Adt3::C0(_, _, h2291, _, _, _, _, _) => {
                h2291
            }
        };
        let a2294 = 0u64;
        nat_rem(a2292, a2293, a2294)
    };
    let v10: u64 = {
        let a2296 = v6;
        let a2297 = match v3.clone() {
            Adt3::C0(_, _, h2295, _, _, _, _, _) => {
                h2295
            }
        };
        let a2298 = 0u64;
        nat_quot(a2296, a2297, a2298)
    };
    let v11: u64 = {
        let a2299 = v4;
        let a2300 = 0u64;
        if nat_eq(a2299, a2300) {
            match v1.clone() {
                Adt3::C0(h2301, _, _, _, _, _, _, _) => {
                    h2301
                }
            }
        } else {
            let a2302 = v4;
            let a2303 = 1u64;
            if nat_eq(a2302, a2303) {
                match v1.clone() {
                    Adt3::C0(_, h2304, _, _, _, _, _, _) => {
                        h2304
                    }
                }
            } else {
                let a2305 = v4;
                let a2306 = 2u64;
                if nat_eq(a2305, a2306) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2307, _, _, _, _, _) => {
                            h2307
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2308, _, _, _, _) => {
                            h2308
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2315 = v11;
        let a2316 = {
            let a2309 = v4;
            let a2310 = 0u64;
            if nat_eq(a2309, a2310) {
                v7
            } else {
                let a2311 = v4;
                let a2312 = 1u64;
                if nat_eq(a2311, a2312) {
                    v8
                } else {
                    let a2313 = v4;
                    let a2314 = 2u64;
                    if nat_eq(a2313, a2314) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2315, a2316)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt7::C1(v12, f138(if if v12 {
        let a2317 = v4;
        let a2318 = 0u64;
        nat_eq(a2317, a2318)
    } else {
        false
    } {
        let a2319 = v7;
        let a2320 = v11;
        nat_sub(a2319, a2320)
    } else {
        v7
    }, if if v12 {
        let a2321 = v4;
        let a2322 = 1u64;
        nat_eq(a2321, a2322)
    } else {
        false
    } {
        let a2323 = v8;
        let a2324 = v11;
        nat_sub(a2323, a2324)
    } else {
        v8
    }, if if v12 {
        let a2325 = v4;
        let a2326 = 2u64;
        nat_eq(a2325, a2326)
    } else {
        false
    } {
        let a2327 = v9;
        let a2328 = v11;
        nat_sub(a2327, a2328)
    } else {
        v9
    }, if if v12 {
        let a2329 = v4;
        let a2330 = 3u64;
        nat_eq(a2329, a2330)
    } else {
        false
    } {
        let a2331 = v10;
        let a2332 = v11;
        nat_sub(a2331, a2332)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2333, _, _, _) => {
            h2333
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2334, _, _) => {
            h2334
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2335, _) => {
            h2335
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2336) => {
            h2336
        }
    })?, f97(v0, v3)?))
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt7> {
    let v8: u64 = f118(v0, v2.clone(), 0u64);
    let v9: u64 = f118(v0, v2.clone(), 1u64);
    let v10: u64 = f118(v0, v2.clone(), 2u64);
    let v11: u64 = f118(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt7::C1(false, f120(v1.clone(), f141(v8, v3, match v1.clone() {
            Adt3::C0(h2337, _, _, _, _, _, _, _) => {
                h2337
            }
        })?, f141(v9, v4, match v1.clone() {
            Adt3::C0(_, h2338, _, _, _, _, _, _) => {
                h2338
            }
        })?, f141(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2339, _, _, _, _, _) => {
                h2339
            }
        })?, f141(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2340, _, _, _, _) => {
                h2340
            }
        })?)?, f97(v0, v2)?))
    } else {
        if if if if f142(v8, v3, match v1.clone() {
            Adt3::C0(h2341, _, _, _, _, _, _, _) => {
                h2341
            }
        })? {
            f142(v9, v4, match v1.clone() {
                Adt3::C0(_, h2342, _, _, _, _, _, _) => {
                    h2342
                }
            })?
        } else {
            false
        } {
            f142(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2343, _, _, _, _, _) => {
                    h2343
                }
            })?
        } else {
            false
        } {
            f142(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2344, _, _, _, _) => {
                    h2344
                }
            })?
        } else {
            false
        } {
            Ok(Adt7::C1(false, f120(v1, f143(v8, v3)?, f143(v9, v4)?, f143(v10, v5)?, f143(v11, v6)?)?, f97(v0, v2)?))
        } else {
            Ok(Adt7::C1(true, 0u64, f97(v0, v2)?))
        }
    }
}

pub fn f102(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = f118(v0, v2.clone(), 0u64);
    let v5: u64 = f118(v0, v2.clone(), 1u64);
    let v6: u64 = f118(v0, v2.clone(), 2u64);
    let v7: u64 = f118(v0, v2.clone(), 3u64);
    Ok(Adt7::C1(false, f120(v1.clone(), f144(v4, v3, match v1 {
        Adt3::C0(h2345, _, _, _, _, _, _, _) => {
            h2345
        }
    })?, v5, v6, v7)?, f97(v0, v2)?))
}

pub fn f103(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt7> {
    let a2346 = v0;
    let a2347 = v3;
    if nat_lt(a2346, a2347) {
        Ok(Adt7::C1(false, {
            let a2348 = v0;
            let a2349 = 4u64;
            nat_mul(a2348, a2349)?
        }, {
            let a2350 = v0;
            let a2351 = 4u64;
            nat_mul(a2350, a2351)?
        }))
    } else {
        let v6: u64 = {
            let a2352 = v0;
            let a2353 = v3;
            nat_sub(a2352, a2353)
        };
        let v7: u64 = f97(v6, v1)?;
        let v8: u64 = f145(v6, v2, v4)?;
        if v5 {
            Ok(Adt7::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt7::C1(true, v7, v8))
        }
    }
}

pub fn f104(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let a2354 = v0;
    let a2355 = v3;
    if nat_lt(a2354, a2355) {
        Ok(Adt7::C1(true, 0u64, f97(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2356 = v0;
            let a2357 = v3;
            nat_sub(a2356, a2357)
        };
        let v5: u64 = f99(v4, v2, v1.clone())?;
        Ok(Adt7::C4(8u64, 3u64, v5, 0u64, f97(v4, v1)?, 3u64, v5))
    }
}

pub fn f105(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt7> {
    let v4: u64 = {
        let a2359 = v1;
        let a2360 = match v3.clone() {
            Adt3::C0(h2358, _, _, _, _, _, _, _) => {
                h2358
            }
        };
        let a2361 = 0u64;
        nat_rem(a2359, a2360, a2361)
    };
    let v5: u64 = {
        let a2367 = {
            let a2363 = v1;
            let a2364 = match v3.clone() {
                Adt3::C0(h2362, _, _, _, _, _, _, _) => {
                    h2362
                }
            };
            let a2365 = 0u64;
            nat_quot(a2363, a2364, a2365)
        };
        let a2368 = match v3.clone() {
            Adt3::C0(_, h2366, _, _, _, _, _, _) => {
                h2366
            }
        };
        let a2369 = 0u64;
        nat_rem(a2367, a2368, a2369)
    };
    let v6: bool = f146(v0, v4, v5);
    Ok(Adt7::C1(!v6, if v6 {
        f99(v1, v2, v3.clone())?
    } else {
        0u64
    }, f97(v1, v3)?))
}

pub fn f106(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt7> {
    Ok(Adt7::C4({
        let a2370 = v3;
        let a2371 = 8u64;
        nat_add(a2370, a2371)?
    }, 0u64, f97(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f99(v0, v5, v2.clone())?
    }, 3u64, f97(v0, v2)?))
}

pub fn f107(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt7> {
    let v6: u64 = {
        let a2373 = v0;
        let a2374 = match v2.clone() {
            Adt3::C0(h2372, _, _, _, _, _, _, _) => {
                h2372
            }
        };
        let a2375 = 0u64;
        nat_rem(a2373, a2374, a2375)
    };
    let v7: u64 = {
        let a2377 = v0;
        let a2378 = match v2.clone() {
            Adt3::C0(h2376, _, _, _, _, _, _, _) => {
                h2376
            }
        };
        let a2379 = 0u64;
        nat_quot(a2377, a2378, a2379)
    };
    let v8: u64 = {
        let a2381 = v7;
        let a2382 = match v2.clone() {
            Adt3::C0(_, h2380, _, _, _, _, _, _) => {
                h2380
            }
        };
        let a2383 = 0u64;
        nat_rem(a2381, a2382, a2383)
    };
    let v9: u64 = {
        let a2385 = v7;
        let a2386 = match v2.clone() {
            Adt3::C0(_, h2384, _, _, _, _, _, _) => {
                h2384
            }
        };
        let a2387 = 0u64;
        nat_quot(a2385, a2386, a2387)
    };
    let v10: u64 = {
        let a2389 = v9;
        let a2390 = match v2.clone() {
            Adt3::C0(_, _, h2388, _, _, _, _, _) => {
                h2388
            }
        };
        let a2391 = 0u64;
        nat_rem(a2389, a2390, a2391)
    };
    let v11: u64 = {
        let a2393 = v9;
        let a2394 = match v2.clone() {
            Adt3::C0(_, _, h2392, _, _, _, _, _) => {
                h2392
            }
        };
        let a2395 = 0u64;
        nat_quot(a2393, a2394, a2395)
    };
    let v12: u64 = f97(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2404 = {
                let a2399 = {
                    let a2396 = v10;
                    let a2397 = v4;
                    let a2398 = 0u64;
                    nat_quot(a2396, a2397, a2398)
                };
                let a2400 = v5;
                nat_mul(a2399, a2400)?
            };
            let a2405 = {
                let a2401 = v8;
                let a2402 = v4;
                let a2403 = 0u64;
                nat_quot(a2401, a2402, a2403)
            };
            nat_add(a2404, a2405)?
        };
        let v14: u64 = {
            let a2425 = v6;
            let a2426 = {
                let a2423 = match v1.clone() {
                    Adt3::C0(h2406, _, _, _, _, _, _, _) => {
                        h2406
                    }
                };
                let a2424 = {
                    let a2421 = {
                        let a2407 = v8;
                        let a2408 = v4;
                        let a2409 = 0u64;
                        nat_rem(a2407, a2408, a2409)
                    };
                    let a2422 = {
                        let a2419 = match v1.clone() {
                            Adt3::C0(_, h2410, _, _, _, _, _, _) => {
                                h2410
                            }
                        };
                        let a2420 = {
                            let a2417 = {
                                let a2411 = v10;
                                let a2412 = v4;
                                let a2413 = 0u64;
                                nat_rem(a2411, a2412, a2413)
                            };
                            let a2418 = {
                                let a2415 = match v1.clone() {
                                    Adt3::C0(_, _, h2414, _, _, _, _, _) => {
                                        h2414
                                    }
                                };
                                let a2416 = v13;
                                nat_mul(a2415, a2416)?
                            };
                            nat_add(a2417, a2418)?
                        };
                        nat_mul(a2419, a2420)?
                    };
                    nat_add(a2421, a2422)?
                };
                nat_mul(a2423, a2424)?
            };
            nat_add(a2425, a2426)?
        };
        Ok(Adt7::C1(false, f97(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2432 = {
                let a2430 = {
                    let a2427 = v11;
                    let a2428 = v5;
                    let a2429 = 0u64;
                    nat_rem(a2427, a2428, a2429)
                };
                let a2431 = v4;
                nat_mul(a2430, a2431)?
            };
            let a2433 = v8;
            nat_add(a2432, a2433)?
        };
        let v16: u64 = {
            let a2439 = {
                let a2437 = {
                    let a2434 = v11;
                    let a2435 = v5;
                    let a2436 = 0u64;
                    nat_quot(a2434, a2435, a2436)
                };
                let a2438 = v4;
                nat_mul(a2437, a2438)?
            };
            let a2440 = v10;
            nat_add(a2439, a2440)?
        };
        let a2442 = v15;
        let a2443 = match v1.clone() {
            Adt3::C0(_, h2441, _, _, _, _, _, _) => {
                h2441
            }
        };
        if if nat_lt(a2442, a2443) {
            let a2445 = v16;
            let a2446 = match v1.clone() {
                Adt3::C0(_, _, h2444, _, _, _, _, _) => {
                    h2444
                }
            };
            nat_lt(a2445, a2446)
        } else {
            false
        } {
            Ok(Adt7::C1(false, f97({
                let a2455 = v6;
                let a2456 = {
                    let a2453 = match v1.clone() {
                        Adt3::C0(h2447, _, _, _, _, _, _, _) => {
                            h2447
                        }
                    };
                    let a2454 = {
                        let a2451 = v15;
                        let a2452 = {
                            let a2449 = match v1.clone() {
                                Adt3::C0(_, h2448, _, _, _, _, _, _) => {
                                    h2448
                                }
                            };
                            let a2450 = v16;
                            nat_mul(a2449, a2450)?
                        };
                        nat_add(a2451, a2452)?
                    };
                    nat_mul(a2453, a2454)?
                };
                nat_add(a2455, a2456)?
            }, v1)?, v12))
        } else {
            Ok(Adt7::C1(true, 0u64, v12))
        }
    }
}

pub fn f108(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt7> {
    let v3: u64 = f97(v0, v2.clone())?;
    let v4: u64 = f97(v0, v1.clone())?;
    let a2461 = {
        let a2458 = v0;
        let a2459 = match v1 {
            Adt3::C0(h2457, _, _, _, _, _, _, _) => {
                h2457
            }
        };
        let a2460 = 0u64;
        nat_rem(a2458, a2459, a2460)
    };
    let a2462 = 0u64;
    if nat_eq(a2461, a2462) {
        Ok(Adt7::C1(false, v4, v3))
    } else {
        Ok(Adt7::C4(8u64, 3u64, f97({
            let a2463 = v0;
            let a2464 = 1u64;
            nat_sub(a2463, a2464)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f109(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt7> {
    let a2465 = v3;
    let a2466 = 13u64;
    if nat_eq(a2465, a2466) {
        f147(v0, v1, v4)
    } else {
        let v5: u64 = f135(match v1.clone() {
            Adt3::C0(h2467, _, _, _, _, _, _, _) => {
                h2467
            }
        }, {
            let a2468 = v3;
            let a2469 = 16u64;
            nat_eq(a2468, a2469)
        })?;
        let v6: u64 = {
            let a2470 = v0;
            let a2471 = v5;
            let a2472 = 0u64;
            nat_quot(a2470, a2471, a2472)
        };
        let v7: u64 = {
            let a2473 = v0;
            let a2474 = v5;
            let a2475 = 0u64;
            nat_rem(a2473, a2474, a2475)
        };
        let v8: u64 = {
            let a2479 = 1u64;
            let a2480 = {
                let a2477 = match v1.clone() {
                    Adt3::C0(h2476, _, _, _, _, _, _, _) => {
                        h2476
                    }
                };
                let a2478 = 2u64;
                nat_mul(a2477, a2478)?
            };
            nat_add(a2479, a2480)?
        };
        let a2481 = v7;
        let a2482 = v8;
        if nat_lt(a2481, a2482) {
            f148(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f97(v6, v2)?;
            let a2483 = v3;
            let a2484 = 14u64;
            if nat_eq(a2483, a2484) {
                Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2485 = v7;
                let a2486 = v8;
                if nat_eq(a2485, a2486) {
                    Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2489 = v7;
                    let a2490 = {
                        let a2487 = v8;
                        let a2488 = 1u64;
                        nat_add(a2487, a2488)?
                    };
                    if nat_eq(a2489, a2490) {
                        Ok(Adt7::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2491, _, _, _, _, _, _, _) => {
                                h2491
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt7::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let a2492 = v0;
    let a2493 = 0u64;
    if nat_eq(a2492, a2493) {
        Ok(Adt7::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2500 = {
            let a2494 = v0;
            let a2495 = 1u64;
            nat_add(a2494, a2495)?
        };
        let a2501 = {
            let a2498 = 2u64;
            let a2499 = {
                let a2496 = v3;
                let a2497 = 2u64;
                nat_mul(a2496, a2497)?
            };
            nat_add(a2498, a2499)?
        };
        if nat_eq(a2500, a2501) {
            Ok(Adt7::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2502 = v0;
                let a2503 = 1u64;
                nat_sub(a2502, a2503)
            };
            let v5: u64 = {
                let a2504 = v4;
                let a2505 = 2u64;
                let a2506 = 0u64;
                nat_quot(a2504, a2505, a2506)
            };
            let a2510 = {
                let a2507 = v4;
                let a2508 = 2u64;
                let a2509 = 0u64;
                nat_rem(a2507, a2508, a2509)
            };
            let a2511 = 0u64;
            if nat_eq(a2510, a2511) {
                Ok(Adt7::C4(17u64, 0u64, f97(v5, v1)?, 4u64, f97(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt7::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f111(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt7> {
    let a2512 = v0;
    let a2513 = f19(v1, v2, v3)?;
    if nat_lt(a2512, a2513) {
        let a2514 = v0;
        let a2515 = v1;
        if nat_lt(a2514, a2515) {
            Ok(Adt7::C1(false, {
                let a2516 = 4u64;
                let a2517 = v0;
                nat_mul(a2516, a2517)?
            }, {
                let a2518 = 4u64;
                let a2519 = v0;
                nat_mul(a2518, a2519)?
            }))
        } else {
            let v4: u64 = {
                let a2520 = v0;
                let a2521 = v1;
                nat_sub(a2520, a2521)
            };
            let v5: u64 = f149(v4, v2)?;
            let a2522 = v5;
            let a2523 = 2u64;
            if nat_lt(a2522, a2523) {
                Ok(Adt7::C4(6u64, {
                    let a2524 = v5;
                    let a2525 = 0u64;
                    if nat_eq(a2524, a2525) {
                        4u64
                    } else {
                        5u64
                    }
                }, f150(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f151(v4, v2)?;
                Ok(Adt7::C4(8u64, 3u64, v6, 1u64, if f152(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt7::C0(Adt8::C1))
    }
}

pub fn f112(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2533 = {
        let a2526 = 2u64;
        let a2527 = f89(v1.clone())?;
        nat_mul(a2526, a2527)?
    };
    let a2534 = {
        let a2531 = f89(v1)?;
        let a2532 = {
            let a2529 = 1u64;
            let a2530 = match v0 {
                Adt3::C0(h2528, _, _, _, _, _, _, _) => {
                    h2528
                }
            };
            nat_add(a2529, a2530)?
        };
        nat_mul(a2531, a2532)?
    };
    nat_add(a2533, a2534)
}

pub fn f113(v0: u64, v1: Adt3) -> R<Adt7> {
    Ok(Adt7::C4(19u64, 4u64, f97(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f114(v0: u64, v1: u64) -> bool {
    let a2535 = v1;
    let a2536 = 2147483647u64;
    if nat_le(a2535, a2536) {
        let a2537 = v1;
        let a2538 = v0;
        nat_lt(a2537, a2538)
    } else {
        false
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt7> {
    let m2539 = f78(v0);
    match m2539 {
        Adt12::C0 => {
            Ok(Adt7::C0(Adt8::C1))
        }
        Adt12::C1(v7) => {
            let v8: u64 = {
                let a2541 = v2;
                let a2542 = match v7.clone() {
                    Adt13::C0(_, h2540, _) => {
                        h2540
                    }
                };
                nat_mul(a2541, a2542)?
            };
            let m2555 = f153(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2543, _, _, _, _, _, _, _) => {
                    h2543
                }
            }, match v5.clone() {
                Adt3::C0(_, h2544, _, _, _, _, _, _) => {
                    h2544
                }
            }, match v5 {
                Adt3::C0(_, _, h2545, _, _, _, _, _) => {
                    h2545
                }
            }, {
                let a2548 = match v3.clone() {
                    Adt3::C0(h2546, _, _, _, _, _, _, _) => {
                        h2546
                    }
                };
                let a2549 = match v7.clone() {
                    Adt13::C0(_, h2547, _) => {
                        h2547
                    }
                };
                nat_mul(a2548, a2549)?
            }, match v3.clone() {
                Adt3::C0(_, h2550, _, _, _, _, _, _) => {
                    h2550
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2551, _, _, _) => {
                    h2551
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2552, _, _) => {
                    h2552
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2553, _) => {
                    h2553
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2554) => {
                    h2554
                }
            })?;
            match m2555 {
                None => {
                    Ok(Adt7::C0(Adt8::C1))
                }
                Some(v9) => {
                    let a2556 = v0;
                    let a2557 = 0u64;
                    if if nat_eq(a2556, a2557) {
                        true
                    } else {
                        let a2558 = v0;
                        let a2559 = 26u64;
                        nat_eq(a2558, a2559)
                    } {
                        Ok(Adt7::C1(false, v9, f97({
                            let a2563 = {
                                let a2561 = v1;
                                let a2562 = match v4.clone() {
                                    Adt3::C0(h2560, _, _, _, _, _, _, _) => {
                                        h2560
                                    }
                                };
                                nat_mul(a2561, a2562)?
                            };
                            let a2564 = v8;
                            nat_add(a2563, a2564)?
                        }, v4)?))
                    } else {
                        Ok(Adt7::C5(v0, v9, match v7.clone() {
                            Adt13::C0(_, _, h2565) => {
                                h2565
                            }
                        }, f97({
                            let a2569 = {
                                let a2567 = v1;
                                let a2568 = match v4.clone() {
                                    Adt3::C0(h2566, _, _, _, _, _, _, _) => {
                                        h2566
                                    }
                                };
                                nat_mul(a2567, a2568)?
                            };
                            let a2570 = v8;
                            nat_add(a2569, a2570)?
                        }, v4.clone())?, {
                            let a2577 = {
                                let a2575 = {
                                    let a2572 = match v7 {
                                        Adt13::C0(_, h2571, _) => {
                                            h2571
                                        }
                                    };
                                    let a2573 = 1u64;
                                    nat_sub(a2572, a2573)
                                };
                                let a2576 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2574, _, _, _) => {
                                        h2574
                                    }
                                };
                                nat_mul(a2575, a2576)?
                            };
                            let a2578 = 4u64;
                            nat_add(a2577, a2578)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2579, _, _, _) => {
                                h2579
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2580 = v1;
    let a2581 = 0u64;
    if if nat_eq(a2580, a2581) {
        true
    } else {
        let a2582 = v2;
        let a2583 = 0u64;
        nat_eq(a2582, a2583)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f117(v0: u64, v1: u64) -> bool {
    let a2584 = v0;
    let a2585 = 4294967295u64;
    if nat_eq(a2584, a2585) {
        true
    } else {
        let a2586 = v0;
        let a2587 = 2147483648u64;
        if nat_lt(a2586, a2587) {
            let a2588 = v0;
            let a2589 = v1;
            nat_lt(a2588, a2589)
        } else {
            false
        }
    }
}

pub fn f118(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2591 = v0;
        let a2592 = match v1.clone() {
            Adt3::C0(h2590, _, _, _, _, _, _, _) => {
                h2590
            }
        };
        let a2593 = 0u64;
        nat_quot(a2591, a2592, a2593)
    };
    let v4: u64 = {
        let a2595 = v3;
        let a2596 = match v1.clone() {
            Adt3::C0(_, h2594, _, _, _, _, _, _) => {
                h2594
            }
        };
        let a2597 = 0u64;
        nat_quot(a2595, a2596, a2597)
    };
    let a2598 = v2;
    let a2599 = 0u64;
    if nat_eq(a2598, a2599) {
        let a2601 = v0;
        let a2602 = match v1 {
            Adt3::C0(h2600, _, _, _, _, _, _, _) => {
                h2600
            }
        };
        let a2603 = 0u64;
        nat_rem(a2601, a2602, a2603)
    } else {
        let a2604 = v2;
        let a2605 = 1u64;
        if nat_eq(a2604, a2605) {
            let a2607 = v3;
            let a2608 = match v1 {
                Adt3::C0(_, h2606, _, _, _, _, _, _) => {
                    h2606
                }
            };
            let a2609 = 0u64;
            nat_rem(a2607, a2608, a2609)
        } else {
            let a2610 = v2;
            let a2611 = 2u64;
            if nat_eq(a2610, a2611) {
                let a2613 = v4;
                let a2614 = match v1 {
                    Adt3::C0(_, _, h2612, _, _, _, _, _) => {
                        h2612
                    }
                };
                let a2615 = 0u64;
                nat_rem(a2613, a2614, a2615)
            } else {
                let a2617 = v4;
                let a2618 = match v1 {
                    Adt3::C0(_, _, h2616, _, _, _, _, _) => {
                        h2616
                    }
                };
                let a2619 = 0u64;
                nat_quot(a2617, a2618, a2619)
            }
        }
    }
}

pub fn f119(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2629 = {
            let a2626 = {
                let a2621 = v2;
                let a2622 = match v0.clone() {
                    Adt6::C0(_, h2620, _, _) => {
                        h2620
                    }
                };
                nat_mul(a2621, a2622)?
            };
            let a2627 = {
                let a2624 = v3;
                let a2625 = match v0.clone() {
                    Adt6::C0(_, _, _, h2623) => {
                        h2623
                    }
                };
                nat_mul(a2624, a2625)?
            };
            nat_add(a2626, a2627)?
        };
        let a2630 = f129(match v0.clone() {
            Adt6::C0(_, _, h2628, _) => {
                h2628
            }
        });
        nat_add(a2629, a2630)?
    };
    let v5: u64 = f128(match v0 {
        Adt6::C0(_, _, h2631, _) => {
            h2631
        }
    });
    let a2632 = v5;
    let a2633 = v4;
    if if nat_le(a2632, a2633) {
        let a2636 = {
            let a2634 = v4;
            let a2635 = v5;
            nat_sub(a2634, a2635)
        };
        let a2637 = v1;
        nat_lt(a2636, a2637)
    } else {
        false
    } {
        Ok(Some({
            let a2638 = v4;
            let a2639 = v5;
            nat_sub(a2638, a2639)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f120(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f138(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2640, _, _, _) => {
            h2640
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2641, _, _) => {
            h2641
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2642, _) => {
            h2642
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2643) => {
            h2643
        }
    })
}

pub fn f121(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt7> {
    let v5: u64 = {
        let a2650 = v3;
        let a2651 = {
            let a2647 = f97({
                let a2644 = v0;
                let a2645 = 2u64;
                let a2646 = 0u64;
                nat_quot(a2644, a2645, a2646)
            }, v1)?;
            let a2648 = v2;
            let a2649 = 0u64;
            nat_quot(a2647, a2648, a2649)
        };
        nat_add(a2650, a2651)?
    };
    let a2655 = {
        let a2652 = v0;
        let a2653 = 2u64;
        let a2654 = 0u64;
        nat_rem(a2652, a2653, a2654)
    };
    let a2656 = 0u64;
    if nat_eq(a2655, a2656) {
        Ok(Adt7::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2657 = v4;
        let a2658 = 0u64;
        if nat_eq(a2657, a2658) {
            Ok(Adt7::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt7::C0(Adt8::C1))
        }
    }
}

pub fn f122(v0: u64, v1: u64) -> R<u64> {
    let a2661 = 8u64;
    let a2662 = {
        let a2659 = f154(v0);
        let a2660 = v1;
        nat_mul(a2659, a2660)?
    };
    nat_add(a2661, a2662)
}

pub fn f123(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt7> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2663, _, _, _, _, _, _, _) => {
            h2663
        }
    };
    let v7: u64 = f154(v3);
    let v8: u64 = {
        let a2666 = 1u64;
        let a2667 = {
            let a2664 = v6;
            let a2665 = v7;
            nat_mul(a2664, a2665)?
        };
        nat_add(a2666, a2667)?
    };
    let a2668 = v0;
    let a2669 = 0u64;
    if nat_eq(a2668, a2669) {
        Ok(f155(0u64))
    } else {
        let a2670 = v0;
        let a2671 = v8;
        if nat_lt(a2670, a2671) {
            let v9: u64 = {
                let a2672 = v0;
                let a2673 = 1u64;
                nat_sub(a2672, a2673)
            };
            let v10: u64 = {
                let a2674 = v9;
                let a2675 = v7;
                let a2676 = 0u64;
                nat_rem(a2674, a2675, a2676)
            };
            let v11: u64 = f97({
                let a2682 = {
                    let a2677 = v1;
                    let a2678 = v6;
                    nat_mul(a2677, a2678)?
                };
                let a2683 = {
                    let a2679 = v9;
                    let a2680 = v7;
                    let a2681 = 0u64;
                    nat_quot(a2679, a2680, a2681)
                };
                nat_add(a2682, a2683)?
            }, v2)?;
            let a2684 = v3;
            let a2685 = 30u64;
            if nat_eq(a2684, a2685) {
                let a2686 = v10;
                let a2687 = 0u64;
                if nat_eq(a2686, a2687) {
                    Ok(Adt7::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2688 = v10;
                    let a2689 = 1u64;
                    if nat_eq(a2688, a2689) {
                        Ok(Adt7::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2690 = v10;
                        let a2691 = 2u64;
                        if nat_eq(a2690, a2691) {
                            Ok(Adt7::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt7::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2692 = v10;
                let a2693 = 0u64;
                if nat_eq(a2692, a2693) {
                    Ok(Adt7::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2694 = v10;
                    let a2695 = 1u64;
                    if nat_eq(a2694, a2695) {
                        Ok(Adt7::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt7::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2696 = v0;
                let a2697 = v8;
                nat_sub(a2696, a2697)
            };
            let a2698 = v12;
            let a2699 = 0u64;
            if nat_eq(a2698, a2699) {
                Ok(Adt7::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2700 = v12;
                let a2701 = 1u64;
                if nat_eq(a2700, a2701) {
                    Ok(Adt7::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2702 = v12;
                    let a2703 = 2u64;
                    if nat_eq(a2702, a2703) {
                        Ok(Adt7::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2704 = v12;
                        let a2705 = 3u64;
                        if nat_eq(a2704, a2705) {
                            Ok(Adt7::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2706 = v12;
                            let a2707 = 4u64;
                            if nat_eq(a2706, a2707) {
                                Ok(Adt7::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2708 = v12;
                                let a2709 = 5u64;
                                if nat_eq(a2708, a2709) {
                                    Ok(Adt7::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2710 = v12;
                                    let a2711 = 6u64;
                                    if nat_eq(a2710, a2711) {
                                        if f156(v5) {
                                            Ok(Adt7::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2712 = 3u64;
                                                let a2713 = v1;
                                                nat_add(a2712, a2713)?
                                            }))
                                        } else {
                                            Ok(Adt7::C0(Adt8::C1))
                                        }
                                    } else {
                                        Ok(Adt7::C0(Adt8::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f124(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt7> {
    let v4: u64 = {
        let a2714 = v0;
        let a2715 = f71(v3);
        let a2716 = 0u64;
        nat_quot(a2714, a2715, a2716)
    };
    let v5: u64 = {
        let a2721 = 3u64;
        let a2722 = {
            let a2718 = v4;
            let a2719 = match v1.clone() {
                Adt3::C0(h2717, _, _, _, _, _, _, _) => {
                    h2717
                }
            };
            let a2720 = 0u64;
            nat_quot(a2718, a2719, a2720)
        };
        nat_add(a2721, a2722)?
    };
    let v6: u64 = f97(v4, v1)?;
    let v7: u64 = f97(v4, v2)?;
    let a2723 = v3;
    let a2724 = 30u64;
    if nat_eq(a2723, a2724) {
        let a2728 = {
            let a2725 = v0;
            let a2726 = 3u64;
            let a2727 = 0u64;
            nat_rem(a2725, a2726, a2727)
        };
        let a2729 = 0u64;
        if nat_eq(a2728, a2729) {
            Ok(Adt7::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2733 = {
                let a2730 = v0;
                let a2731 = 3u64;
                let a2732 = 0u64;
                nat_rem(a2730, a2731, a2732)
            };
            let a2734 = 1u64;
            if nat_eq(a2733, a2734) {
                Ok(Adt7::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt7::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt7::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f125(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2735 = v0;
        let a2736 = 8388608u64;
        let a2737 = 0u64;
        nat_quot(a2735, a2736, a2737)
    };
    let a2738 = v0;
    let a2739 = 2147483648u64;
    if if if nat_lt(a2738, a2739) {
        let a2740 = 127u64;
        let a2741 = v1;
        nat_le(a2740, a2741)
    } else {
        false
    } {
        let a2742 = v1;
        let a2743 = 190u64;
        nat_lt(a2742, a2743)
    } else {
        false
    } {
        let v2: u64 = {
            let a2747 = 8388608u64;
            let a2748 = {
                let a2744 = v0;
                let a2745 = 8388608u64;
                let a2746 = 0u64;
                nat_rem(a2744, a2745, a2746)
            };
            nat_add(a2747, a2748)?
        };
        let a2749 = v1;
        let a2750 = 150u64;
        if nat_lt(a2749, a2750) {
            let v3: u64 = f157({
                let a2751 = 150u64;
                let a2752 = v1;
                nat_sub(a2751, a2752)
            });
            let a2756 = {
                let a2753 = v2;
                let a2754 = v3;
                let a2755 = 0u64;
                nat_rem(a2753, a2754, a2755)
            };
            let a2757 = 0u64;
            if nat_eq(a2756, a2757) {
                let a2758 = v2;
                let a2759 = v3;
                let a2760 = 0u64;
                Ok(nat_quot(a2758, a2759, a2760))
            } else {
                Ok(0u64)
            }
        } else {
            let a2763 = f157({
                let a2761 = v1;
                let a2762 = 150u64;
                nat_sub(a2761, a2762)
            });
            let a2764 = v2;
            nat_mul(a2763, a2764)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64) -> bool {
    let a2765 = v0;
    let a2766 = 1u64;
    if nat_le(a2765, a2766) {
        true
    } else {
        let a2770 = {
            let a2767 = v1;
            let a2768 = v2;
            let a2769 = 1u64;
            nat_rem(a2767, a2768, a2769)
        };
        let a2771 = 0u64;
        nat_eq(a2770, a2771)
    }
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2772 = v1;
    let a2773 = 1u64;
    if if if nat_le(a2772, a2773) {
        true
    } else {
        let a2774 = v0;
        let a2775 = v2;
        nat_le(a2774, a2775)
    } {
        let a2776 = v3;
        let a2777 = 1u64;
        if nat_le(a2776, a2777) {
            true
        } else {
            let a2784 = {
                let a2782 = v0;
                let a2783 = {
                    let a2780 = {
                        let a2778 = v1;
                        let a2779 = 1u64;
                        nat_sub(a2778, a2779)
                    };
                    let a2781 = v2;
                    nat_mul(a2780, a2781)?
                };
                nat_add(a2782, a2783)?
            };
            let a2785 = v4;
            nat_le(a2784, a2785)
        }
    } else {
        false
    } {
        let a2786 = v5;
        let a2787 = 1u64;
        if nat_le(a2786, a2787) {
            Ok(true)
        } else {
            let a2800 = {
                let a2798 = {
                    let a2792 = v0;
                    let a2793 = {
                        let a2790 = {
                            let a2788 = v1;
                            let a2789 = 1u64;
                            nat_sub(a2788, a2789)
                        };
                        let a2791 = v2;
                        nat_mul(a2790, a2791)?
                    };
                    nat_add(a2792, a2793)?
                };
                let a2799 = {
                    let a2796 = {
                        let a2794 = v3;
                        let a2795 = 1u64;
                        nat_sub(a2794, a2795)
                    };
                    let a2797 = v4;
                    nat_mul(a2796, a2797)?
                };
                nat_add(a2798, a2799)?
            };
            let a2801 = v6;
            Ok(nat_le(a2800, a2801))
        }
    } else {
        Ok(false)
    }
}

pub fn f128(v0: u64) -> u64 {
    let a2802 = v0;
    let a2803 = 2147483648u64;
    if nat_lt(a2802, a2803) {
        v0
    } else {
        0u64
    }
}

pub fn f129(v0: u64) -> u64 {
    let a2804 = v0;
    let a2805 = 2147483648u64;
    if nat_lt(a2804, a2805) {
        0u64
    } else {
        let a2806 = 4294967296u64;
        let a2807 = v0;
        nat_sub(a2806, a2807)
    }
}

pub fn f130(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f158(v0, v1, v6, v7, v8) {
        let a2808 = v5;
        let a2809 = 26u64;
        nat_eq(a2808, a2809)
    } else {
        false
    } {
        let a2810 = v3;
        let a2811 = v7;
        nat_eq(a2810, a2811)
    } else {
        false
    } {
        let a2812 = v4;
        let a2813 = v8;
        nat_eq(a2812, a2813)
    } else {
        false
    } {
        let a2814 = v9;
        let a2815 = 1u64;
        nat_eq(a2814, a2815)
    } else {
        false
    } {
        let a2816 = v10;
        let a2817 = f159(v0);
        nat_eq(a2816, a2817)
    } else {
        false
    } {
        let a2818 = v11;
        let a2819 = v1;
        nat_eq(a2818, a2819)
    } else {
        false
    } {
        let a2820 = v12;
        let a2821 = v6;
        nat_eq(a2820, a2821)
    } else {
        false
    } {
        let a2822 = v13;
        let a2823 = v7;
        nat_eq(a2822, a2823)
    } else {
        false
    } {
        let a2824 = v14;
        let a2825 = v8;
        nat_eq(a2824, a2825)
    } else {
        false
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2826 = v0;
    let a2827 = 0u64;
    if if if nat_eq(a2826, a2827) {
        true
    } else {
        let a2828 = v1;
        let a2829 = 0u64;
        nat_eq(a2828, a2829)
    } {
        true
    } else {
        let a2830 = v2;
        let a2831 = 0u64;
        nat_eq(a2830, a2831)
    } {
        true
    } else {
        let a2832 = v3;
        let a2833 = 0u64;
        nat_eq(a2832, a2833)
    }
}

pub fn f132(v0: Adt3, v1: bool) -> R<u64> {
    f160(v0.clone(), {
        let a2835 = match v0.clone() {
            Adt3::C0(h2834, _, _, _, _, _, _, _) => {
                h2834
            }
        };
        let a2836 = 1u64;
        nat_sub(a2835, a2836)
    }, {
        let a2838 = match v0.clone() {
            Adt3::C0(_, h2837, _, _, _, _, _, _) => {
                h2837
            }
        };
        let a2839 = 1u64;
        nat_sub(a2838, a2839)
    }, {
        let a2841 = match v0.clone() {
            Adt3::C0(_, _, h2840, _, _, _, _, _) => {
                h2840
            }
        };
        let a2842 = 1u64;
        nat_sub(a2841, a2842)
    }, {
        let a2844 = match v0 {
            Adt3::C0(_, _, _, h2843, _, _, _, _) => {
                h2843
            }
        };
        let a2845 = 1u64;
        nat_sub(a2844, a2845)
    }, v1)
}

pub fn f133(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2846 = v0;
    let a2847 = 0u64;
    if nat_eq(a2846, a2847) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2848 = v0;
            let a2849 = 1u64;
            nat_sub(a2848, a2849)
        };
        let v5: u64 = {
            let a2850 = v4;
            let a2851 = v1;
            let a2852 = 0u64;
            nat_quot(a2850, a2851, a2852)
        };
        let v6: u64 = {
            let a2860 = {
                let a2853 = v5;
                let a2854 = v2;
                nat_mul(a2853, a2854)?
            };
            let a2861 = {
                let a2858 = {
                    let a2855 = v4;
                    let a2856 = v1;
                    let a2857 = 0u64;
                    nat_rem(a2855, a2856, a2857)
                };
                let a2859 = v3;
                nat_mul(a2858, a2859)?
            };
            nat_add(a2860, a2861)?
        };
        let a2862 = v5;
        let a2863 = 0u64;
        if nat_eq(a2862, a2863) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2872 = {
                    let a2866 = {
                        let a2864 = v5;
                        let a2865 = 1u64;
                        nat_sub(a2864, a2865)
                    };
                    let a2867 = v2;
                    nat_mul(a2866, a2867)?
                };
                let a2873 = {
                    let a2870 = {
                        let a2868 = v1;
                        let a2869 = 1u64;
                        nat_sub(a2868, a2869)
                    };
                    let a2871 = v3;
                    nat_mul(a2870, a2871)?
                };
                nat_add(a2872, a2873)?
            };
            let a2874 = v6;
            let a2875 = v7;
            if nat_lt(a2874, a2875) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f134(v0: Adt3, v1: u64) -> bool {
    let a2876 = v1;
    let a2877 = 0u64;
    if !nat_eq(a2876, a2877) {
        true
    } else {
        let a2879 = match v0 {
            Adt3::C0(_, _, _, _, h2878, _, _, _) => {
                h2878
            }
        };
        let a2880 = 4u64;
        nat_eq(a2879, a2880)
    }
}

pub fn f135(v0: u64, v1: bool) -> R<u64> {
    let a2883 = {
        let a2881 = v0;
        let a2882 = 2u64;
        nat_mul(a2881, a2882)?
    };
    let a2884 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2883, a2884)
}

pub fn f136(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt8> {
    let m2885 = f161(v0, v1);
    match m2885 {
        None => {
            if v5 {
                let m2886 = f162(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2886 {
                    Adt14::C0 => {
                        Ok(Adt8::C0)
                    }
                    Adt14::C1 => {
                        Ok(Adt8::C1)
                    }
                    Adt14::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt8::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt8::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a2887 = 4u64;
                let a2888 = v28;
                nat_le(a2887, a2888)
            };
            let a2889 = v2;
            let a2890 = 0u64;
            if if if if if nat_eq(a2889, a2890) {
                let a2891 = v4;
                let a2892 = 0u64;
                nat_eq(a2891, a2892)
            } else {
                false
            } {
                let a2893 = v6;
                let a2894 = v14;
                if nat_eq(a2893, a2894) {
                    let a2895 = v7;
                    let a2896 = v15;
                    nat_eq(a2895, a2896)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2897 = v8;
                let a2898 = v16;
                if nat_eq(a2897, a2898) {
                    let a2899 = v9;
                    let a2900 = v17;
                    nat_eq(a2899, a2900)
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
                        let a2901 = v3;
                        let a2902 = 0u64;
                        nat_eq(a2901, a2902)
                    } else {
                        false
                    } {
                        f79(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt8::C2(v28, v29, v6, v7, v8, v9, {
                    let a2907 = {
                        let a2903 = v6;
                        let a2904 = v7;
                        nat_mul(a2903, a2904)?
                    };
                    let a2908 = {
                        let a2905 = v8;
                        let a2906 = v9;
                        nat_mul(a2905, a2906)?
                    };
                    nat_mul(a2907, a2908)?
                }))
            } else {
                Ok(Adt8::C1)
            }
        }
    }
}

pub fn f137(v0: u64) -> u64 {
    let m2909 = f78(v0);
    match m2909 {
        Adt12::C0 => {
            0u64
        }
        Adt12::C1(v1) => {
            let a2911 = match v1.clone() {
                Adt13::C0(_, h2910, _) => {
                    h2910
                }
            };
            let a2912 = 1u64;
            if if nat_eq(a2911, a2912) {
                let a2914 = match v1.clone() {
                    Adt13::C0(_, _, h2913) => {
                        h2913
                    }
                };
                let a2915 = 1u64;
                if if if nat_eq(a2914, a2915) {
                    true
                } else {
                    let a2917 = match v1.clone() {
                        Adt13::C0(_, _, h2916) => {
                            h2916
                        }
                    };
                    let a2918 = 2u64;
                    nat_eq(a2917, a2918)
                } {
                    true
                } else {
                    let a2920 = match v1.clone() {
                        Adt13::C0(_, _, h2919) => {
                            h2919
                        }
                    };
                    let a2921 = 4u64;
                    nat_eq(a2920, a2921)
                } {
                    true
                } else {
                    let a2923 = match v1.clone() {
                        Adt13::C0(_, _, h2922) => {
                            h2922
                        }
                    };
                    let a2924 = 8u64;
                    nat_eq(a2923, a2924)
                }
            } else {
                false
            } {
                match v1 {
                    Adt13::C0(_, _, h2925) => {
                        h2925
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2938 = {
        let a2934 = {
            let a2930 = {
                let a2926 = v0;
                let a2927 = v4;
                nat_mul(a2926, a2927)?
            };
            let a2931 = {
                let a2928 = v1;
                let a2929 = v5;
                nat_mul(a2928, a2929)?
            };
            nat_add(a2930, a2931)?
        };
        let a2935 = {
            let a2932 = v2;
            let a2933 = v6;
            nat_mul(a2932, a2933)?
        };
        nat_add(a2934, a2935)?
    };
    let a2939 = {
        let a2936 = v3;
        let a2937 = v7;
        nat_mul(a2936, a2937)?
    };
    nat_add(a2938, a2939)
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2944 = {
        let a2942 = {
            let a2940 = v0;
            let a2941 = v1;
            nat_sub(a2940, a2941)
        };
        let a2943 = 1u64;
        nat_sub(a2942, a2943)
    };
    let a2945 = v2;
    nat_add(a2944, a2945)
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f138({
        let a2946 = v0;
        let a2947 = v4;
        let a2948 = 0u64;
        nat_rem(a2946, a2947, a2948)
    }, {
        let a2949 = v1;
        let a2950 = v5;
        let a2951 = 0u64;
        nat_rem(a2949, a2950, a2951)
    }, {
        let a2952 = v2;
        let a2953 = v6;
        let a2954 = 0u64;
        nat_rem(a2952, a2953, a2954)
    }, {
        let a2955 = v3;
        let a2956 = v7;
        let a2957 = 0u64;
        nat_rem(a2955, a2956, a2957)
    }, v8, v9, v10, v11)
}

pub fn f141(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2958 = v1;
    let a2959 = 2147483648u64;
    if nat_lt(a2958, a2959) {
        let a2970 = {
            let a2968 = {
                let a2963 = {
                    let a2960 = v0;
                    let a2961 = v2;
                    let a2962 = 0u64;
                    nat_rem(a2960, a2961, a2962)
                };
                let a2964 = v2;
                nat_add(a2963, a2964)?
            };
            let a2969 = {
                let a2965 = v1;
                let a2966 = v2;
                let a2967 = 0u64;
                nat_rem(a2965, a2966, a2967)
            };
            nat_sub(a2968, a2969)
        };
        let a2971 = v2;
        let a2972 = 0u64;
        Ok(nat_rem(a2970, a2971, a2972))
    } else {
        let a2975 = {
            let a2973 = v0;
            let a2974 = f129(v1);
            nat_add(a2973, a2974)?
        };
        let a2976 = v2;
        let a2977 = 0u64;
        Ok(nat_rem(a2975, a2976, a2977))
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2980 = f128(v1);
    let a2981 = {
        let a2978 = v0;
        let a2979 = f129(v1);
        nat_add(a2978, a2979)?
    };
    if nat_le(a2980, a2981) {
        let a2982 = f143(v0, v1)?;
        let a2983 = v2;
        Ok(nat_lt(a2982, a2983))
    } else {
        Ok(false)
    }
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2986 = {
        let a2984 = v0;
        let a2985 = f129(v1);
        nat_add(a2984, a2985)?
    };
    let a2987 = f128(v1);
    Ok(nat_sub(a2986, a2987))
}

pub fn f144(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2988 = v0;
    let a2989 = v1;
    if nat_lt(a2988, a2989) {
        let a2990 = v1;
        let a2991 = v0;
        Ok(nat_sub(a2990, a2991))
    } else {
        let a2994 = v0;
        let a2995 = {
            let a2992 = v1;
            let a2993 = v2;
            nat_add(a2992, a2993)?
        };
        if nat_lt(a2994, a2995) {
            let a2996 = v0;
            let a2997 = v1;
            Ok(nat_sub(a2996, a2997))
        } else {
            let a3004 = {
                let a3000 = 2u64;
                let a3001 = {
                    let a2998 = v2;
                    let a2999 = 1u64;
                    nat_sub(a2998, a2999)
                };
                nat_mul(a3000, a3001)?
            };
            let a3005 = {
                let a3002 = v0;
                let a3003 = v1;
                nat_sub(a3002, a3003)
            };
            Ok(nat_sub(a3004, a3005))
        }
    }
}

pub fn f145(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3008 = {
        let a3006 = v2;
        let a3007 = f160(v1.clone(), f118(v0, v1.clone(), 0u64), f118(v0, v1.clone(), 1u64), f118(v0, v1.clone(), 2u64), f118(v0, v1.clone(), 3u64), false)?;
        nat_add(a3006, a3007)?
    };
    let a3009 = f160(v1.clone(), f118(v0, v1.clone(), 0u64), f118(v0, v1.clone(), 1u64), f118(v0, v1.clone(), 2u64), f118(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3008, a3009))
}

pub fn f146(v0: u64, v1: u64, v2: u64) -> bool {
    let a3010 = v0;
    let a3011 = 0u64;
    if nat_eq(a3010, a3011) {
        let a3012 = v2;
        let a3013 = v1;
        nat_le(a3012, a3013)
    } else {
        let a3014 = v0;
        let a3015 = 1u64;
        if nat_eq(a3014, a3015) {
            let a3016 = v2;
            let a3017 = v1;
            nat_lt(a3016, a3017)
        } else {
            let a3018 = v0;
            let a3019 = 2u64;
            if nat_eq(a3018, a3019) {
                let a3020 = v1;
                let a3021 = v2;
                nat_le(a3020, a3021)
            } else {
                let a3022 = v0;
                let a3023 = 3u64;
                if nat_eq(a3022, a3023) {
                    let a3024 = v1;
                    let a3025 = v2;
                    nat_lt(a3024, a3025)
                } else {
                    let a3026 = v1;
                    let a3027 = v2;
                    nat_eq(a3026, a3027)
                }
            }
        }
    }
}

pub fn f147(v0: u64, v1: Adt3, v2: u64) -> R<Adt7> {
    let a3028 = v0;
    let a3029 = 0u64;
    if nat_eq(a3028, a3029) {
        Ok(f155(0u64))
    } else {
        let a3032 = {
            let a3030 = v0;
            let a3031 = 1u64;
            nat_add(a3030, a3031)?
        };
        let a3033 = f90(v1.clone(), 13u64, v2)?;
        if nat_eq(a3032, a3033) {
            Ok(Adt7::C4({
                let a3034 = v2;
                let a3035 = 0u64;
                if nat_eq(a3034, a3035) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3036 = v0;
                let a3037 = 1u64;
                nat_sub(a3036, a3037)
            };
            if f134(v1.clone(), v2) {
                let v4: u64 = f135(match v1.clone() {
                    Adt3::C0(h3038, _, _, _, _, _, _, _) => {
                        h3038
                    }
                }, false)?;
                let v5: u64 = {
                    let a3039 = v3;
                    let a3040 = v4;
                    let a3041 = 0u64;
                    nat_quot(a3039, a3040, a3041)
                };
                let v6: u64 = {
                    let a3042 = v3;
                    let a3043 = v4;
                    let a3044 = 0u64;
                    nat_rem(a3042, a3043, a3044)
                };
                let a3047 = {
                    let a3045 = v6;
                    let a3046 = 1u64;
                    nat_add(a3045, a3046)?
                };
                let a3048 = v4;
                if nat_eq(a3047, a3048) {
                    Ok(Adt7::C4({
                        let a3049 = v2;
                        let a3050 = 0u64;
                        if nat_eq(a3049, a3050) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f148(v6, v5, v1, v2)
                }
            } else {
                f163({
                    let a3051 = v3;
                    let a3052 = 2u64;
                    let a3053 = 0u64;
                    nat_quot(a3051, a3052, a3053)
                }, {
                    let a3054 = v3;
                    let a3055 = 2u64;
                    let a3056 = 0u64;
                    nat_rem(a3054, a3055, a3056)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f148(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt7> {
    let a3057 = v0;
    let a3058 = 0u64;
    if nat_eq(a3057, a3058) {
        Ok(f155(1u64))
    } else {
        f163({
            let a3067 = {
                let a3060 = v1;
                let a3061 = match v2.clone() {
                    Adt3::C0(h3059, _, _, _, _, _, _, _) => {
                        h3059
                    }
                };
                nat_mul(a3060, a3061)?
            };
            let a3068 = {
                let a3064 = {
                    let a3062 = v0;
                    let a3063 = 1u64;
                    nat_sub(a3062, a3063)
                };
                let a3065 = 2u64;
                let a3066 = 0u64;
                nat_quot(a3064, a3065, a3066)
            };
            nat_add(a3067, a3068)?
        }, {
            let a3071 = {
                let a3069 = v0;
                let a3070 = 1u64;
                nat_sub(a3069, a3070)
            };
            let a3072 = 2u64;
            let a3073 = 0u64;
            nat_rem(a3071, a3072, a3073)
        }, v2, v3, 1u64)
    }
}

pub fn f149(v0: u64, v1: u64) -> R<u64> {
    let a3074 = v0;
    let a3075 = f66(v1)?;
    let a3076 = 0u64;
    Ok(nat_rem(a3074, a3075, a3076))
}

pub fn f150(v0: u64, v1: u64) -> R<u64> {
    let a3077 = 4u64;
    let a3078 = f164(v0, v1)?;
    nat_mul(a3077, a3078)
}

pub fn f151(v0: u64, v1: u64) -> R<u64> {
    let a3079 = 4u64;
    let a3080 = f165(v0, v1)?;
    nat_mul(a3079, a3080)
}

pub fn f152(v0: u64, v1: u64) -> R<bool> {
    let a3086 = {
        let a3083 = {
            let a3081 = f149(v0, v1)?;
            let a3082 = 2u64;
            nat_sub(a3081, a3082)
        };
        let a3084 = 2u64;
        let a3085 = 0u64;
        nat_rem(a3083, a3084, a3085)
    };
    let a3087 = 0u64;
    Ok(nat_eq(a3086, a3087))
}

pub fn f153(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3088 = f78(v0);
    match m3088 {
        Adt12::C0 => {
            Ok(None::<u64>)
        }
        Adt12::C1(v13) => {
            if if if if if f166(v0) {
                let a3089 = v1;
                let a3090 = v7;
                nat_lt(a3089, a3090)
            } else {
                false
            } {
                f114(v8, v2)
            } else {
                false
            } {
                let a3091 = v3;
                let a3092 = f167(v4, v5, v6)?;
                nat_lt(a3091, a3092)
            } else {
                false
            } {
                let a3097 = {
                    let a3094 = v7;
                    let a3095 = match v13.clone() {
                        Adt13::C0(_, h3093, _) => {
                            h3093
                        }
                    };
                    let a3096 = 0u64;
                    nat_rem(a3094, a3095, a3096)
                };
                let a3098 = 0u64;
                nat_eq(a3097, a3098)
            } else {
                false
            } {
                Ok(Some({
                    let a3127 = {
                        let a3117 = {
                            let a3107 = {
                                let a3103 = {
                                    let a3100 = v1;
                                    let a3101 = match v13 {
                                        Adt13::C0(_, h3099, _) => {
                                            h3099
                                        }
                                    };
                                    let a3102 = 0u64;
                                    nat_quot(a3100, a3101, a3102)
                                };
                                let a3104 = v9;
                                nat_mul(a3103, a3104)?
                            };
                            let a3108 = {
                                let a3105 = v2;
                                let a3106 = v10;
                                nat_mul(a3105, a3106)?
                            };
                            nat_add(a3107, a3108)?
                        };
                        let a3118 = {
                            let a3115 = {
                                let a3112 = {
                                    let a3109 = v3;
                                    let a3110 = v4;
                                    let a3111 = 0u64;
                                    nat_quot(a3109, a3110, a3111)
                                };
                                let a3113 = v5;
                                let a3114 = 0u64;
                                nat_rem(a3112, a3113, a3114)
                            };
                            let a3116 = v11;
                            nat_mul(a3115, a3116)?
                        };
                        nat_add(a3117, a3118)?
                    };
                    let a3128 = {
                        let a3125 = {
                            let a3122 = {
                                let a3119 = v3;
                                let a3120 = v4;
                                let a3121 = 0u64;
                                nat_quot(a3119, a3120, a3121)
                            };
                            let a3123 = v5;
                            let a3124 = 0u64;
                            nat_quot(a3122, a3123, a3124)
                        };
                        let a3126 = v12;
                        nat_mul(a3125, a3126)?
                    };
                    nat_add(a3127, a3128)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f154(v0: u64) -> u64 {
    let a3129 = v0;
    let a3130 = 30u64;
    if nat_eq(a3129, a3130) {
        4u64
    } else {
        3u64
    }
}

pub fn f155(v0: u64) -> Adt7 {
    Adt7::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f156(v0: u64) -> bool {
    let a3131 = 0u64;
    let a3132 = v0;
    if nat_lt(a3131, a3132) {
        let a3133 = v0;
        let a3134 = 2139095040u64;
        nat_le(a3133, a3134)
    } else {
        false
    }
}

pub fn f157(v0: u64) -> u64 {
    let a3135 = v0;
    let a3136 = 40u64;
    if nat_lt(a3135, a3136) {
        let a3137 = v0;
        let a3138 = 20u64;
        if nat_lt(a3137, a3138) {
            let a3139 = v0;
            let a3140 = 10u64;
            if nat_lt(a3139, a3140) {
                let a3141 = v0;
                let a3142 = 5u64;
                if nat_lt(a3141, a3142) {
                    let a3143 = v0;
                    let a3144 = 2u64;
                    if nat_lt(a3143, a3144) {
                        let a3145 = v0;
                        let a3146 = 1u64;
                        if nat_lt(a3145, a3146) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3147 = v0;
                        let a3148 = 3u64;
                        if nat_lt(a3147, a3148) {
                            4u64
                        } else {
                            let a3149 = v0;
                            let a3150 = 4u64;
                            if nat_lt(a3149, a3150) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3151 = v0;
                    let a3152 = 7u64;
                    if nat_lt(a3151, a3152) {
                        let a3153 = v0;
                        let a3154 = 6u64;
                        if nat_lt(a3153, a3154) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3155 = v0;
                        let a3156 = 8u64;
                        if nat_lt(a3155, a3156) {
                            128u64
                        } else {
                            let a3157 = v0;
                            let a3158 = 9u64;
                            if nat_lt(a3157, a3158) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3159 = v0;
                let a3160 = 15u64;
                if nat_lt(a3159, a3160) {
                    let a3161 = v0;
                    let a3162 = 12u64;
                    if nat_lt(a3161, a3162) {
                        let a3163 = v0;
                        let a3164 = 11u64;
                        if nat_lt(a3163, a3164) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3165 = v0;
                        let a3166 = 13u64;
                        if nat_lt(a3165, a3166) {
                            4096u64
                        } else {
                            let a3167 = v0;
                            let a3168 = 14u64;
                            if nat_lt(a3167, a3168) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3169 = v0;
                    let a3170 = 17u64;
                    if nat_lt(a3169, a3170) {
                        let a3171 = v0;
                        let a3172 = 16u64;
                        if nat_lt(a3171, a3172) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3173 = v0;
                        let a3174 = 18u64;
                        if nat_lt(a3173, a3174) {
                            131072u64
                        } else {
                            let a3175 = v0;
                            let a3176 = 19u64;
                            if nat_lt(a3175, a3176) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3177 = v0;
            let a3178 = 30u64;
            if nat_lt(a3177, a3178) {
                let a3179 = v0;
                let a3180 = 25u64;
                if nat_lt(a3179, a3180) {
                    let a3181 = v0;
                    let a3182 = 22u64;
                    if nat_lt(a3181, a3182) {
                        let a3183 = v0;
                        let a3184 = 21u64;
                        if nat_lt(a3183, a3184) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3185 = v0;
                        let a3186 = 23u64;
                        if nat_lt(a3185, a3186) {
                            4194304u64
                        } else {
                            let a3187 = v0;
                            let a3188 = 24u64;
                            if nat_lt(a3187, a3188) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3189 = v0;
                    let a3190 = 27u64;
                    if nat_lt(a3189, a3190) {
                        let a3191 = v0;
                        let a3192 = 26u64;
                        if nat_lt(a3191, a3192) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3193 = v0;
                        let a3194 = 28u64;
                        if nat_lt(a3193, a3194) {
                            134217728u64
                        } else {
                            let a3195 = v0;
                            let a3196 = 29u64;
                            if nat_lt(a3195, a3196) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3197 = v0;
                let a3198 = 35u64;
                if nat_lt(a3197, a3198) {
                    let a3199 = v0;
                    let a3200 = 32u64;
                    if nat_lt(a3199, a3200) {
                        let a3201 = v0;
                        let a3202 = 31u64;
                        if nat_lt(a3201, a3202) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3203 = v0;
                        let a3204 = 33u64;
                        if nat_lt(a3203, a3204) {
                            4294967296u64
                        } else {
                            let a3205 = v0;
                            let a3206 = 34u64;
                            if nat_lt(a3205, a3206) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3207 = v0;
                    let a3208 = 37u64;
                    if nat_lt(a3207, a3208) {
                        let a3209 = v0;
                        let a3210 = 36u64;
                        if nat_lt(a3209, a3210) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3211 = v0;
                        let a3212 = 38u64;
                        if nat_lt(a3211, a3212) {
                            137438953472u64
                        } else {
                            let a3213 = v0;
                            let a3214 = 39u64;
                            if nat_lt(a3213, a3214) {
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

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f131(v1, v2, v3, v4) {
        f168(v0)
    } else {
        f166(v0)
    }
}

pub fn f159(v0: u64) -> u64 {
    let a3215 = v0;
    let a3216 = 26u64;
    if nat_eq(a3215, a3216) {
        26u64
    } else {
        0u64
    }
}

pub fn f160(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3237 = {
        let a3231 = {
            let a3225 = {
                let a3219 = v1;
                let a3220 = if v5 {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3217, _, _, _) => {
                            h3217
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3218, _, _, _) => {
                            h3218
                        }
                    })
                };
                nat_mul(a3219, a3220)?
            };
            let a3226 = {
                let a3223 = v2;
                let a3224 = if v5 {
                    f129(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3221, _, _) => {
                            h3221
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3222, _, _) => {
                            h3222
                        }
                    })
                };
                nat_mul(a3223, a3224)?
            };
            nat_add(a3225, a3226)?
        };
        let a3232 = {
            let a3229 = v3;
            let a3230 = if v5 {
                f129(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3227, _) => {
                        h3227
                    }
                })
            } else {
                f128(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3228, _) => {
                        h3228
                    }
                })
            };
            nat_mul(a3229, a3230)?
        };
        nat_add(a3231, a3232)?
    };
    let a3238 = {
        let a3235 = v4;
        let a3236 = if v5 {
            f129(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3233) => {
                    h3233
                }
            })
        } else {
            f128(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3234) => {
                    h3234
                }
            })
        };
        nat_mul(a3235, a3236)?
    };
    nat_add(a3237, a3238)
}

pub fn f161(v0: u64, v1: u64) -> Option<u64> {
    let a3239 = v0;
    let a3240 = 2u64;
    if nat_eq(a3239, a3240) {
        Some(0u64)
    } else {
        let a3241 = v0;
        let a3242 = 5u64;
        if nat_eq(a3241, a3242) {
            Some(1u64)
        } else {
            let a3243 = v0;
            let a3244 = 6u64;
            if nat_eq(a3243, a3244) {
                Some(2u64)
            } else {
                let a3245 = v0;
                let a3246 = 7u64;
                if nat_eq(a3245, a3246) {
                    Some(3u64)
                } else {
                    let a3247 = v0;
                    let a3248 = 9u64;
                    if nat_eq(a3247, a3248) {
                        Some(4u64)
                    } else {
                        let a3249 = v0;
                        let a3250 = 8u64;
                        if nat_eq(a3249, a3250) {
                            Some(5u64)
                        } else {
                            let a3251 = v0;
                            let a3252 = 89u64;
                            if nat_eq(a3251, a3252) {
                                let a3253 = v1;
                                let a3254 = 2u64;
                                if nat_eq(a3253, a3254) {
                                    Some(6u64)
                                } else {
                                    let a3255 = v1;
                                    let a3256 = 0u64;
                                    if nat_eq(a3255, a3256) {
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

pub fn f162(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt14> {
    let a3257 = v0;
    let a3258 = 30u64;
    if nat_eq(a3257, a3258) {
        let a3259 = v2;
        let a3260 = 0u64;
        if if nat_eq(a3259, a3260) {
            let a3261 = v1;
            let a3262 = 0u64;
            if nat_eq(a3261, a3262) {
                true
            } else {
                if if f169(v1, v4) {
                    let a3263 = v6;
                    let a3264 = v10;
                    nat_eq(a3263, a3264)
                } else {
                    false
                } {
                    let a3265 = v7;
                    let a3266 = v11;
                    nat_eq(a3265, a3266)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            f170(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt14::C0)
        }
    } else {
        if if if f171(v1) {
            f171(v2)
        } else {
            false
        } {
            let a3267 = v1;
            let a3268 = 0u64;
            if if nat_eq(a3267, a3268) {
                true
            } else {
                let a3269 = v2;
                let a3270 = 0u64;
                nat_eq(a3269, a3270)
            } {
                true
            } else {
                let a3271 = v1;
                let a3272 = v2;
                nat_eq(a3271, a3272)
            }
        } else {
            false
        } {
            f170(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt14::C0)
        }
    }
}

pub fn f163(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt7> {
    let a3273 = v1;
    let a3274 = 0u64;
    if nat_eq(a3273, a3274) {
        Ok(Adt7::C4({
            let a3275 = v3;
            let a3276 = 0u64;
            if nat_eq(a3275, a3276) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f97(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt7::C4({
            let a3277 = v3;
            let a3278 = 0u64;
            if nat_eq(a3277, a3278) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f164(v0: u64, v1: u64) -> R<u64> {
    let a3279 = v0;
    let a3280 = f66(v1)?;
    let a3281 = 0u64;
    Ok(nat_quot(a3279, a3280, a3281))
}

pub fn f165(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f164(v0, v1)?;
    let v3: u64 = {
        let a3284 = {
            let a3282 = f149(v0, v1)?;
            let a3283 = 2u64;
            nat_sub(a3282, a3283)
        };
        let a3285 = 2u64;
        let a3286 = 0u64;
        nat_quot(a3284, a3285, a3286)
    };
    let v4: u64 = {
        let a3287 = v2;
        let a3288 = v1;
        nat_mul(a3287, a3288)?
    };
    if f152(v0, v1)? {
        let a3289 = v4;
        let a3290 = v3;
        nat_add(a3289, a3290)
    } else {
        let a3302 = {
            let a3298 = v4;
            let a3299 = {
                let a3296 = {
                    let a3291 = v1;
                    let a3292 = 1u64;
                    nat_sub(a3291, a3292)
                };
                let a3297 = {
                    let a3293 = v2;
                    let a3294 = v1;
                    let a3295 = 0u64;
                    nat_rem(a3293, a3294, a3295)
                };
                nat_mul(a3296, a3297)?
            };
            nat_sub(a3298, a3299)
        };
        let a3303 = {
            let a3300 = v3;
            let a3301 = v1;
            nat_mul(a3300, a3301)?
        };
        nat_add(a3302, a3303)
    }
}

pub fn f166(v0: u64) -> bool {
    let a3304 = v0;
    let a3305 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3304, a3305) {
        true
    } else {
        let a3306 = v0;
        let a3307 = 1u64;
        nat_eq(a3306, a3307)
    } {
        true
    } else {
        let a3308 = v0;
        let a3309 = 2u64;
        nat_eq(a3308, a3309)
    } {
        true
    } else {
        let a3310 = v0;
        let a3311 = 3u64;
        nat_eq(a3310, a3311)
    } {
        true
    } else {
        let a3312 = v0;
        let a3313 = 6u64;
        nat_eq(a3312, a3313)
    } {
        true
    } else {
        let a3314 = v0;
        let a3315 = 7u64;
        nat_eq(a3314, a3315)
    } {
        true
    } else {
        let a3316 = v0;
        let a3317 = 8u64;
        nat_eq(a3316, a3317)
    } {
        true
    } else {
        let a3318 = v0;
        let a3319 = 10u64;
        nat_eq(a3318, a3319)
    } {
        true
    } else {
        let a3320 = v0;
        let a3321 = 11u64;
        nat_eq(a3320, a3321)
    } {
        true
    } else {
        let a3322 = v0;
        let a3323 = 12u64;
        nat_eq(a3322, a3323)
    } {
        true
    } else {
        let a3324 = v0;
        let a3325 = 13u64;
        nat_eq(a3324, a3325)
    } {
        true
    } else {
        let a3326 = v0;
        let a3327 = 14u64;
        nat_eq(a3326, a3327)
    } {
        true
    } else {
        let a3328 = v0;
        let a3329 = 16u64;
        nat_eq(a3328, a3329)
    } {
        true
    } else {
        let a3330 = v0;
        let a3331 = 17u64;
        nat_eq(a3330, a3331)
    } {
        true
    } else {
        let a3332 = v0;
        let a3333 = 18u64;
        nat_eq(a3332, a3333)
    } {
        true
    } else {
        let a3334 = v0;
        let a3335 = 19u64;
        nat_eq(a3334, a3335)
    } {
        true
    } else {
        let a3336 = v0;
        let a3337 = 20u64;
        nat_eq(a3336, a3337)
    } {
        true
    } else {
        let a3338 = v0;
        let a3339 = 21u64;
        nat_eq(a3338, a3339)
    } {
        true
    } else {
        let a3340 = v0;
        let a3341 = 22u64;
        nat_eq(a3340, a3341)
    } {
        true
    } else {
        let a3342 = v0;
        let a3343 = 23u64;
        nat_eq(a3342, a3343)
    } {
        true
    } else {
        let a3344 = v0;
        let a3345 = 26u64;
        nat_eq(a3344, a3345)
    } {
        true
    } else {
        let a3346 = v0;
        let a3347 = 29u64;
        nat_eq(a3346, a3347)
    } {
        true
    } else {
        let a3348 = v0;
        let a3349 = 30u64;
        nat_eq(a3348, a3349)
    } {
        true
    } else {
        let a3350 = v0;
        let a3351 = 34u64;
        nat_eq(a3350, a3351)
    } {
        true
    } else {
        let a3352 = v0;
        let a3353 = 35u64;
        nat_eq(a3352, a3353)
    } {
        true
    } else {
        let a3354 = v0;
        let a3355 = 39u64;
        nat_eq(a3354, a3355)
    } {
        true
    } else {
        let a3356 = v0;
        let a3357 = 40u64;
        nat_eq(a3356, a3357)
    } {
        true
    } else {
        let a3358 = v0;
        let a3359 = 41u64;
        nat_eq(a3358, a3359)
    } {
        true
    } else {
        let a3360 = v0;
        let a3361 = 42u64;
        nat_eq(a3360, a3361)
    } {
        true
    } else {
        let a3362 = v0;
        let a3363 = 43u64;
        nat_eq(a3362, a3363)
    }
}

pub fn f167(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3364 = v0;
    let a3365 = 0u64;
    if if if nat_eq(a3364, a3365) {
        true
    } else {
        let a3366 = v1;
        let a3367 = 0u64;
        nat_eq(a3366, a3367)
    } {
        true
    } else {
        let a3368 = v2;
        let a3369 = 0u64;
        nat_eq(a3368, a3369)
    } {
        Ok(0u64)
    } else {
        let a3372 = {
            let a3370 = v0;
            let a3371 = v1;
            nat_mul(a3370, a3371)?
        };
        let a3373 = v2;
        nat_mul(a3372, a3373)
    }
}

pub fn f168(v0: u64) -> bool {
    let m3374 = f78(v0);
    match m3374 {
        Adt12::C0 => {
            false
        }
        Adt12::C1(_) => {
            true
        }
    }
}

pub fn f169(v0: u64, v1: u64) -> bool {
    let a3375 = v0;
    let a3376 = 1u64;
    if nat_eq(a3375, a3376) {
        true
    } else {
        let v2: u64 = f172(v0);
        let a3377 = 0u64;
        let a3378 = v2;
        if nat_lt(a3377, a3378) {
            let a3382 = {
                let a3379 = v1;
                let a3380 = v2;
                let a3381 = 1u64;
                nat_rem(a3379, a3380, a3381)
            };
            let a3383 = 0u64;
            nat_eq(a3382, a3383)
        } else {
            false
        }
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt14> {
    let a3384 = v0;
    let a3385 = 28u64;
    if if if if nat_eq(a3384, a3385) {
        let a3386 = v1;
        let a3387 = 0u64;
        nat_eq(a3386, a3387)
    } else {
        false
    } {
        let a3388 = v2;
        let a3389 = 0u64;
        nat_eq(a3388, a3389)
    } else {
        false
    } {
        let a3390 = v3;
        let a3391 = 0u64;
        nat_eq(a3390, a3391)
    } else {
        false
    } {
        let a3392 = v4;
        let a3393 = v8;
        if if if if if if if if nat_eq(a3392, a3393) {
            let a3394 = v5;
            let a3395 = v12;
            nat_eq(a3394, a3395)
        } else {
            false
        } {
            let a3396 = v9;
            let a3397 = v13;
            nat_eq(a3396, a3397)
        } else {
            false
        } {
            let a3398 = v10;
            let a3399 = v14;
            nat_eq(a3398, a3399)
        } else {
            false
        } {
            let a3400 = v11;
            let a3401 = v15;
            nat_eq(a3400, a3401)
        } else {
            false
        } {
            let a3402 = 0u64;
            let a3403 = v6;
            nat_lt(a3402, a3403)
        } else {
            false
        } {
            let a3404 = 0u64;
            let a3405 = v7;
            nat_lt(a3404, a3405)
        } else {
            false
        } {
            let a3409 = {
                let a3406 = v10;
                let a3407 = v6;
                let a3408 = 1u64;
                nat_rem(a3406, a3407, a3408)
            };
            let a3410 = 0u64;
            if nat_eq(a3409, a3410) {
                let a3414 = {
                    let a3411 = v11;
                    let a3412 = v7;
                    let a3413 = 1u64;
                    nat_rem(a3411, a3412, a3413)
                };
                let a3415 = 0u64;
                nat_eq(a3414, a3415)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt14::C2(v5, v4, v9, v10, v11, {
                let a3416 = v10;
                let a3417 = v6;
                let a3418 = 0u64;
                nat_quot(a3416, a3417, a3418)
            }, {
                let a3419 = v11;
                let a3420 = v7;
                let a3421 = 0u64;
                nat_quot(a3419, a3420, a3421)
            }, {
                let a3422 = v5;
                let a3423 = v4;
                nat_mul(a3422, a3423)?
            }, {
                let a3424 = v4;
                let a3425 = v9;
                nat_mul(a3424, a3425)?
            }, {
                let a3426 = v5;
                let a3427 = v9;
                nat_mul(a3426, a3427)?
            }))
        } else {
            Ok(Adt14::C1)
        }
    } else {
        Ok(Adt14::C0)
    }
}

pub fn f171(v0: u64) -> bool {
    let a3428 = v0;
    let a3429 = 0u64;
    if if nat_eq(a3428, a3429) {
        true
    } else {
        let a3430 = v0;
        let a3431 = 1u64;
        nat_eq(a3430, a3431)
    } {
        true
    } else {
        let a3432 = v0;
        let a3433 = 30u64;
        nat_eq(a3432, a3433)
    }
}

pub fn f172(v0: u64) -> u64 {
    if f166(v0) {
        let m3434 = f78(v0);
        match m3434 {
            Adt12::C0 => {
                0u64
            }
            Adt12::C1(v1) => {
                let a3436 = 1u64;
                let a3437 = match v1.clone() {
                    Adt13::C0(_, h3435, _) => {
                        h3435
                    }
                };
                if nat_lt(a3436, a3437) {
                    match v1 {
                        Adt13::C0(_, h3438, _) => {
                            h3438
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

pub fn run(p0: Adt0) -> R<Adt7> {
    f0(p0)
}
