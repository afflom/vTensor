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
        let a327 = 52u64;
        if nat_eq(a326, a327) {
            f34(v1, v2, v4)?
        } else {
            let a328 = v0;
            let a329 = 73u64;
            if nat_eq(a328, a329) {
                f35(v2, v3, v4)?
            } else {
                let a330 = v0;
                let a331 = 57u64;
                if if nat_eq(a330, a331) {
                    true
                } else {
                    let a332 = v0;
                    let a333 = 58u64;
                    nat_eq(a332, a333)
                } {
                    let m334 = v3;
                    match m334 {
                        None => {
                            f36({
                                let a335 = v0;
                                let a336 = 58u64;
                                nat_eq(a335, a336)
                            }, v1, v2, v4)?
                        }
                        Some(_) => {
                            Adt10::C1
                        }
                    }
                } else {
                    let a337 = v0;
                    let a338 = 49u64;
                    if nat_eq(a337, a338) {
                        f37(v1, v2, v3, v4)?
                    } else {
                        let a339 = v0;
                        let a340 = 51u64;
                        if nat_eq(a339, a340) {
                            f38(v1, v2, v3, v4)?
                        } else {
                            let a341 = v0;
                            let a342 = 39u64;
                            if nat_eq(a341, a342) {
                                f39(v2, v3, v4)?
                            } else {
                                let a343 = v0;
                                let a344 = 77u64;
                                if nat_eq(a343, a344) {
                                    f40(v2, v4)?
                                } else {
                                    let a345 = v0;
                                    let a346 = 40u64;
                                    if nat_eq(a345, a346) {
                                        f41(v2, v3, v4)?
                                    } else {
                                        if f42(v0) {
                                            f43(v0, v2, {
                                                let m347 = v3;
                                                match m347 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(_) => {
                                                        true
                                                    }
                                                }
                                            }, v4)?
                                        } else {
                                            let a348 = v0;
                                            let a349 = 19u64;
                                            if nat_eq(a348, a349) {
                                                f44(v2, v4)?
                                            } else {
                                                let a350 = v0;
                                                let a351 = 21u64;
                                                if nat_eq(a350, a351) {
                                                    f45(v5, v2, v3, v4)?
                                                } else {
                                                    let a352 = v0;
                                                    let a353 = 61u64;
                                                    if if if nat_eq(a352, a353) {
                                                        true
                                                    } else {
                                                        let a354 = v0;
                                                        let a355 = 62u64;
                                                        nat_eq(a354, a355)
                                                    } {
                                                        true
                                                    } else {
                                                        let a356 = v0;
                                                        let a357 = 63u64;
                                                        nat_eq(a356, a357)
                                                    } {
                                                        f46(v0, v1, v2, v4)?
                                                    } else {
                                                        let a358 = v0;
                                                        let a359 = 4u64;
                                                        if if nat_eq(a358, a359) {
                                                            true
                                                        } else {
                                                            let a360 = v0;
                                                            let a361 = 32u64;
                                                            nat_eq(a360, a361)
                                                        } {
                                                            f47(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a362 = v0;
                                                            let a363 = 20u64;
                                                            if nat_eq(a362, a363) {
                                                                f48(v2, v4)?
                                                            } else {
                                                                let a364 = v0;
                                                                let a365 = 42u64;
                                                                if nat_eq(a364, a365) {
                                                                    f49(4u64, v2, v4)?
                                                                } else {
                                                                    let a366 = v0;
                                                                    let a367 = 69u64;
                                                                    if nat_eq(a366, a367) {
                                                                        let a368 = v5;
                                                                        let a369 = 4u64;
                                                                        if nat_lt(a368, a369) {
                                                                            f49(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a370 = v0;
                                                                        let a371 = 70u64;
                                                                        if nat_eq(a370, a371) {
                                                                            f50(v5, v2, v4)?
                                                                        } else {
                                                                            let a372 = v0;
                                                                            let a373 = 75u64;
                                                                            if if nat_eq(a372, a373) {
                                                                                true
                                                                            } else {
                                                                                let a374 = v0;
                                                                                let a375 = 76u64;
                                                                                nat_eq(a374, a375)
                                                                            } {
                                                                                f51({
                                                                                    let a376 = v0;
                                                                                    let a377 = 76u64;
                                                                                    nat_eq(a376, a377)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a378 = v0;
                                                                                let a379 = 31u64;
                                                                                if if if nat_eq(a378, a379) {
                                                                                    true
                                                                                } else {
                                                                                    let a380 = v0;
                                                                                    let a381 = 47u64;
                                                                                    nat_eq(a380, a381)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a382 = v0;
                                                                                    let a383 = 68u64;
                                                                                    nat_eq(a382, a383)
                                                                                } {
                                                                                    f52(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a384 = v0;
                                                                                    let a385 = 15u64;
                                                                                    if nat_eq(a384, a385) {
                                                                                        f53(v2, v4)?
                                                                                    } else {
                                                                                        let a386 = v0;
                                                                                        let a387 = 17u64;
                                                                                        if nat_eq(a386, a387) {
                                                                                            f54(v2, v4)?
                                                                                        } else {
                                                                                            let a388 = v0;
                                                                                            let a389 = 18u64;
                                                                                            if nat_eq(a388, a389) {
                                                                                                f55(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a390 = v0;
                                                                                                let a391 = 13u64;
                                                                                                if if if nat_eq(a390, a391) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a392 = v0;
                                                                                                    let a393 = 14u64;
                                                                                                    nat_eq(a392, a393)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a394 = v0;
                                                                                                    let a395 = 16u64;
                                                                                                    nat_eq(a394, a395)
                                                                                                } {
                                                                                                    f56(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m396 = f57(v0, v5)?;
                                                                                                    match m396 {
                                                                                                        None => {
                                                                                                            f58(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f59(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    let a397 = v1;
    let a398 = 0u64;
    if nat_eq(a397, a398) {
        let a400 = v3;
        let a401 = match v0.clone() {
            Adt8::C0(_, _, _, h399, _, _, _, _, _, _, _, _, _, _, _) => {
                h399
            }
        };
        if nat_lt(a400, a401) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h402, _, _, _, _, _, _) => {
                    h402
                }
            };
            let v5: u64 = {
                let a404 = v3;
                let a405 = match v0.clone() {
                    Adt8::C0(_, _, h403, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h403
                    }
                };
                let a406 = 0u64;
                nat_quot(a404, a405, a406)
            };
            Ok(Adt9::C8(Some(f60(v4.clone(), f61(v5, v4.clone(), 0u64), f61(v5, v4.clone(), 1u64), f61(v5, v4, 2u64), f62(v0.clone(), v5, {
                let a408 = v3;
                let a409 = match v0 {
                    Adt8::C0(_, _, h407, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h407
                    }
                };
                let a410 = 0u64;
                nat_rem(a408, a409, a410)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h411, _, _, _, _) => {
                h411
            }
        };
        let v7: u64 = {
            let a412 = v1;
            let a413 = 1u64;
            if nat_eq(a412, a413) {
                match v0.clone() {
                    Adt8::C0(_, h414, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h415, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h415
                    }
                }
            }
        };
        let v8: u64 = {
            let a419 = v2;
            let a420 = {
                let a416 = v3;
                let a417 = v7;
                let a418 = 0u64;
                nat_quot(a416, a417, a418)
            };
            nat_add(a419, a420)?
        };
        let v9: u64 = {
            let a421 = v3;
            let a422 = v7;
            let a423 = 0u64;
            nat_rem(a421, a422, a423)
        };
        let v10: u64 = {
            let a425 = v8;
            let a426 = match v6.clone() {
                Adt3::C0(h424, _, _, _, _, _, _, _) => {
                    h424
                }
            };
            let a427 = 0u64;
            nat_rem(a425, a426, a427)
        };
        let v11: u64 = {
            let a429 = v8;
            let a430 = match v6.clone() {
                Adt3::C0(h428, _, _, _, _, _, _, _) => {
                    h428
                }
            };
            let a431 = 0u64;
            nat_quot(a429, a430, a431)
        };
        let v12: u64 = {
            let a433 = v11;
            let a434 = match v6.clone() {
                Adt3::C0(_, h432, _, _, _, _, _, _) => {
                    h432
                }
            };
            let a435 = 0u64;
            nat_rem(a433, a434, a435)
        };
        let v13: u64 = {
            let a437 = v11;
            let a438 = match v6.clone() {
                Adt3::C0(_, h436, _, _, _, _, _, _) => {
                    h436
                }
            };
            let a439 = 0u64;
            nat_quot(a437, a438, a439)
        };
        let a441 = v8;
        let a442 = match v0.clone() {
            Adt8::C0(h440, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h440
            }
        };
        if nat_lt(a441, a442) {
            let a443 = v1;
            let a444 = 1u64;
            if nat_eq(a443, a444) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h445, _, _, _, _, _, _) => {
                        h445
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h446, _, _, _, _, _) => {
                        h446
                    }
                };
                let m449 = f63(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h447, _, _, _) => {
                        h447
                    }
                }, match v15.clone() {
                    Adt3::C0(h448, _, _, _, _, _, _, _) => {
                        h448
                    }
                }, v10, f61(v9, v14.clone(), 0u64))?;
                match m449 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m452 = f63(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h450, _, _) => {
                                h450
                            }
                        }, match v15 {
                            Adt3::C0(_, h451, _, _, _, _, _, _) => {
                                h451
                            }
                        }, v12, f61(v9, v14, 1u64))?;
                        match m452 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f64(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a453 = v1;
                let a454 = 2u64;
                if nat_eq(a453, a454) {
                    Ok(Adt9::C8(Some({
                        let a456 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h455) => {
                                h455
                            }
                        };
                        let a457 = 0u64;
                        if nat_eq(a456, a457) {
                            f60(v6, v10, v12, v9, v13)?
                        } else {
                            f60(v6.clone(), v10, v12, {
                                let a459 = v13;
                                let a460 = match v6.clone() {
                                    Adt3::C0(_, _, h458, _, _, _, _, _) => {
                                        h458
                                    }
                                };
                                let a461 = 0u64;
                                nat_rem(a459, a460, a461)
                            }, {
                                let a469 = {
                                    let a467 = {
                                        let a463 = v13;
                                        let a464 = match v6 {
                                            Adt3::C0(_, _, h462, _, _, _, _, _) => {
                                                h462
                                            }
                                        };
                                        let a465 = 0u64;
                                        nat_quot(a463, a464, a465)
                                    };
                                    let a468 = match v0 {
                                        Adt8::C0(_, _, h466, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h466
                                        }
                                    };
                                    nat_mul(a467, a468)?
                                };
                                let a470 = v9;
                                nat_add(a469, a470)?
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
    Ok(f65(v3.clone(), {
        let m471 = v3.clone();
        match m471 {
            Adt4::C0 => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f67(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f68(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f69(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f70(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v62, v63, v64, v65) => {
                f76(v0, v65, v62, v63, v64, v4)?
            }
            Adt4::C24(_) => {
                Adt9::C0(Adt10::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f66(v0, v1, v2, v3)?
            }
            Adt4::C26(v57, v58, v59, v60) => {
                let a472 = v57;
                let a473 = 64u64;
                if if if nat_eq(a472, a473) {
                    true
                } else {
                    let a474 = v57;
                    let a475 = 65u64;
                    nat_eq(a474, a475)
                } {
                    true
                } else {
                    let a476 = v57;
                    let a477 = 66u64;
                    nat_eq(a476, a477)
                } {
                    Adt9::C0(Adt10::C5(64u64))
                } else {
                    let a478 = v57;
                    let a479 = 0u64;
                    if if nat_eq(a478, a479) {
                        true
                    } else {
                        let a480 = v57;
                        let a481 = 30u64;
                        nat_eq(a480, a481)
                    } {
                        f75(v0, v1, v2, v57, v58, v59, v60, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v56) => {
                f74(v0, v1, v2, v56)?
            }
            Adt4::C28(v52) => {
                f71(v0, v1, v2, v52, v4)?
            }
            Adt4::C29(v53) => {
                f72(v0, v1, v2, v53)?
            }
            Adt4::C30(v54, _) => {
                f73(v0, v1, v2, v54)?
            }
        }
    }))
}

pub fn f10(v0: Adt4) -> R<Adt9> {
    let m482 = v0;
    match m482 {
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
                let a487 = {
                    let a483 = 0u64;
                    let a484 = v45;
                    if nat_lt(a483, a484) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a488 = {
                    let a485 = 0u64;
                    let a486 = v47;
                    if nat_lt(a485, a486) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a487, a488)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v53, _, _, _) => {
            Ok(Adt9::C6({
                let a489 = v53;
                let a490 = 66u64;
                if nat_eq(a489, a490) {
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

pub fn f11(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a491 = v1;
    let a492 = 5u64;
    if if if if nat_lt(a491, a492) {
        let a493 = v5;
        let a494 = 5u64;
        nat_lt(a493, a494)
    } else {
        false
    } {
        let a495 = v7;
        let a496 = 5u64;
        nat_lt(a495, a496)
    } else {
        false
    } {
        let a497 = v9;
        let a498 = 2u64;
        if nat_lt(a497, a498) {
            true
        } else {
            let a499 = v9;
            let a500 = 3u64;
            nat_eq(a499, a500)
        }
    } else {
        false
    } {
        Adt9::C7(f77(v0, v1, v2, v3, v4), f78(v5, v6), f78(v7, v8), f79({
            let a501 = v9;
            let a502 = 0u64;
            if nat_eq(a501, a502) {
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
    let a514 = {
        let a508 = {
            let a504 = match v0.clone() {
                Adt2::C0(_, h503, _, _, _, _, _, _, _, _, _) => {
                    h503
                }
            };
            let a505 = v1;
            let a506 = 0u64;
            nat_quot(a504, a505, a506)
        };
        let a509 = match v0.clone() {
            Adt2::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
                h507
            }
        };
        nat_mul(a508, a509)?
    };
    let a515 = {
        let a512 = match v0.clone() {
            Adt2::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                h510
            }
        };
        let a513 = match v0 {
            Adt2::C0(_, _, _, _, h511, _, _, _, _, _, _) => {
                h511
            }
        };
        nat_mul(a512, a513)?
    };
    nat_mul(a514, a515)
}

pub fn f13(v0: u64) -> bool {
    let a516 = v0;
    let a517 = 4294967296u64;
    if nat_lt(a516, a517) {
        let a521 = {
            let a518 = v0;
            let a519 = 2147483648u64;
            let a520 = 0u64;
            nat_rem(a518, a519, a520)
        };
        let a522 = 2139095040u64;
        nat_lt(a521, a522)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a523 = 4u64;
    let a524 = v0;
    nat_mul(a523, a524)
}

pub fn f15(v0: u64) -> R<u64> {
    let a525 = v0;
    let a526 = 0u64;
    if nat_eq(a525, a526) {
        Ok(0u64)
    } else {
        let a529 = 4u64;
        let a530 = {
            let a527 = 3u64;
            let a528 = v0;
            nat_mul(a527, a528)?
        };
        nat_add(a529, a530)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a532 = match v0.clone() {
            Adt2::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
                h531
            }
        };
        let a533 = v1;
        let a534 = 0u64;
        nat_quot(a532, a533, a534)
    }, match v0.clone() {
        Adt2::C0(_, _, h535, _, _, _, _, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h536, _, _, _, _, _, _, _) => {
            h536
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h537, _, _, _, _, _, _) => {
            h537
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h538, _, _, _, _, _) => {
            h538
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h539, _, _, _, _) => {
            h539
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h540, _, _, _) => {
            h540
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h541, _, _) => {
            h541
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a543 = match v0.clone() {
        Adt6::C0(h542, _, _, _) => {
            h542
        }
    };
    let a544 = 0u64;
    if nat_eq(a543, a544) {
        f80(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h545, _, _) => {
                h545
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h546, _) => {
                h546
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h547) => {
                h547
            }
        }), {
            let a549 = v1;
            let a550 = match v0 {
                Adt6::C0(_, _, _, h548) => {
                    h548
                }
            };
            nat_add(a549, a550)?
        })
    } else {
        f80(v0, v1)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a552 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h551, _) => {
            h551
        }
    };
    let a553 = 0u64;
    if nat_eq(a552, a553) {
        Ok(true)
    } else {
        let a555 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h554, _) => {
                h554
            }
        };
        let a556 = 0u64;
        if nat_eq(a555, a556) {
            Ok(true)
        } else {
            let a558 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h557) => {
                    h557
                }
            };
            let a559 = 0u64;
            if nat_eq(a558, a559) {
                Ok(true)
            } else {
                let a561 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h560) => {
                        h560
                    }
                };
                let a562 = 0u64;
                if nat_eq(a561, a562) {
                    Ok(true)
                } else {
                    let a568 = {
                        let a565 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h563) => {
                                h563
                            }
                        };
                        let a566 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h564, _) => {
                                h564
                            }
                        };
                        nat_add(a565, a566)?
                    };
                    let a569 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h567) => {
                            h567
                        }
                    };
                    if nat_le(a568, a569) {
                        Ok(true)
                    } else {
                        let a575 = {
                            let a572 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h570) => {
                                    h570
                                }
                            };
                            let a573 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h571, _) => {
                                    h571
                                }
                            };
                            nat_add(a572, a573)?
                        };
                        let a576 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h574) => {
                                h574
                            }
                        };
                        Ok(nat_le(a575, a576))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a579 = match v0.clone() {
        Adt2::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
            h577
        }
    };
    let a580 = match v1.clone() {
        Adt2::C0(_, h578, _, _, _, _, _, _, _, _, _) => {
            h578
        }
    };
    if if if nat_eq(a579, a580) {
        let a583 = match v0.clone() {
            Adt2::C0(_, _, h581, _, _, _, _, _, _, _, _) => {
                h581
            }
        };
        let a584 = match v1.clone() {
            Adt2::C0(_, _, h582, _, _, _, _, _, _, _, _) => {
                h582
            }
        };
        nat_eq(a583, a584)
    } else {
        false
    } {
        let a587 = match v0.clone() {
            Adt2::C0(_, _, _, h585, _, _, _, _, _, _, _) => {
                h585
            }
        };
        let a588 = match v1.clone() {
            Adt2::C0(_, _, _, h586, _, _, _, _, _, _, _) => {
                h586
            }
        };
        nat_eq(a587, a588)
    } else {
        false
    } {
        let a591 = match v0 {
            Adt2::C0(_, _, _, _, h589, _, _, _, _, _, _) => {
                h589
            }
        };
        let a592 = match v1 {
            Adt2::C0(_, _, _, _, h590, _, _, _, _, _, _) => {
                h590
            }
        };
        nat_eq(a591, a592)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a594 = match v0.clone() {
        Adt2::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
            h593
        }
    };
    let a595 = 1u64;
    if if if if nat_eq(a594, a595) {
        true
    } else {
        let a597 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h596, _, _, _, _, _) => {
                h596
            }
        };
        let a598 = 4u64;
        nat_eq(a597, a598)
    } {
        let a600 = match v0.clone() {
            Adt2::C0(_, _, h599, _, _, _, _, _, _, _, _) => {
                h599
            }
        };
        let a601 = 1u64;
        if nat_eq(a600, a601) {
            true
        } else {
            let a606 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h602, _, _, _, _) => {
                    h602
                }
            };
            let a607 = {
                let a604 = match v0.clone() {
                    Adt2::C0(_, h603, _, _, _, _, _, _, _, _, _) => {
                        h603
                    }
                };
                let a605 = 4u64;
                nat_mul(a604, a605)?
            };
            nat_eq(a606, a607)
        }
    } else {
        false
    } {
        let a609 = match v0.clone() {
            Adt2::C0(_, _, _, h608, _, _, _, _, _, _, _) => {
                h608
            }
        };
        let a610 = 1u64;
        if nat_eq(a609, a610) {
            true
        } else {
            let a618 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h611, _, _, _) => {
                    h611
                }
            };
            let a619 = {
                let a616 = {
                    let a614 = match v0.clone() {
                        Adt2::C0(_, h612, _, _, _, _, _, _, _, _, _) => {
                            h612
                        }
                    };
                    let a615 = match v0.clone() {
                        Adt2::C0(_, _, h613, _, _, _, _, _, _, _, _) => {
                            h613
                        }
                    };
                    nat_mul(a614, a615)?
                };
                let a617 = 4u64;
                nat_mul(a616, a617)?
            };
            nat_eq(a618, a619)
        }
    } else {
        false
    } {
        let a621 = match v0.clone() {
            Adt2::C0(_, _, _, _, h620, _, _, _, _, _, _) => {
                h620
            }
        };
        let a622 = 1u64;
        if nat_eq(a621, a622) {
            Ok(true)
        } else {
            let a633 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h623, _, _) => {
                    h623
                }
            };
            let a634 = {
                let a631 = {
                    let a629 = {
                        let a626 = match v0.clone() {
                            Adt2::C0(_, h624, _, _, _, _, _, _, _, _, _) => {
                                h624
                            }
                        };
                        let a627 = match v0.clone() {
                            Adt2::C0(_, _, h625, _, _, _, _, _, _, _, _) => {
                                h625
                            }
                        };
                        nat_mul(a626, a627)?
                    };
                    let a630 = match v0 {
                        Adt2::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                            h628
                        }
                    };
                    nat_mul(a629, a630)?
                };
                let a632 = 4u64;
                nat_mul(a631, a632)?
            };
            Ok(nat_eq(a633, a634))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a639 = {
        let a635 = v0;
        let a636 = v1;
        nat_mul(a635, a636)?
    };
    let a640 = {
        let a637 = v2;
        let a638 = f81(v3);
        nat_mul(a637, a638)?
    };
    nat_mul(a639, a640)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a643 = v0;
    let a644 = {
        let a641 = f82(v1)?;
        let a642 = v2;
        nat_mul(a641, a642)?
    };
    nat_add(a643, a644)
}

pub fn f23(v0: u64) -> u64 {
    let a645 = v0;
    let a646 = 30u64;
    if nat_eq(a645, a646) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a647 = v0;
    let a648 = 2139095040u64;
    if nat_le(a647, a648) {
        true
    } else {
        let a649 = v0;
        let a650 = 2147483648u64;
        nat_eq(a649, a650)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a652 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h651, _, _, _, _, _) => {
            h651
        }
    };
    let a653 = v1;
    if if nat_eq(a652, a653) {
        let a658 = {
            let a655 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h654) => {
                    h654
                }
            };
            let a656 = v1;
            let a657 = 0u64;
            nat_rem(a655, a656, a657)
        };
        let a659 = 0u64;
        nat_eq(a658, a659)
    } else {
        false
    } {
        f83(match v0.clone() {
            Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                h660
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
                h661
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h662, _, _, _, _, _, _) => {
                h662
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h663, _, _, _, _) => {
                h663
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h664, _, _, _) => {
                h664
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h665, _, _) => {
                h665
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a668 = match v0.clone() {
            Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                h666
            }
        };
        let a669 = match v1.clone() {
            Adt2::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
                h667
            }
        };
        nat_eq(a668, a669)
    } else {
        false
    } {
        let a672 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h670) => {
                h670
            }
        };
        let a673 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h671) => {
                h671
            }
        };
        nat_eq(a672, a673)
    } else {
        false
    } {
        let a676 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
                h674
            }
        };
        let a677 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h675, _, _, _, _, _) => {
                h675
            }
        };
        nat_eq(a676, a677)
    } else {
        false
    } {
        let a680 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h678, _, _, _, _) => {
                h678
            }
        };
        let a681 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h679, _, _, _, _) => {
                h679
            }
        };
        nat_eq(a680, a681)
    } else {
        false
    } {
        let a684 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h682, _, _, _) => {
                h682
            }
        };
        let a685 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h683, _, _, _) => {
                h683
            }
        };
        nat_eq(a684, a685)
    } else {
        false
    } {
        let a688 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h686, _, _) => {
                h686
            }
        };
        let a689 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h687, _, _) => {
                h687
            }
        };
        nat_eq(a688, a689)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a690 = v0;
    let a691 = 0u64;
    if if if nat_eq(a690, a691) {
        true
    } else {
        let a692 = v1;
        let a693 = 0u64;
        nat_eq(a692, a693)
    } {
        true
    } else {
        let a694 = v2;
        let a695 = 0u64;
        nat_eq(a694, a695)
    } {
        true
    } else {
        let a696 = v3;
        let a697 = 0u64;
        nat_eq(a696, a697)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f84({
        let a699 = match v0.clone() {
            Adt2::C0(_, h698, _, _, _, _, _, _, _, _, _) => {
                h698
            }
        };
        let a700 = v1;
        nat_mul(a699, a700)?
    }, match v0.clone() {
        Adt2::C0(_, _, h701, _, _, _, _, _, _, _, _) => {
            h701
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h702, _, _, _, _, _, _, _) => {
            h702
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h703, _, _, _, _, _, _) => {
            h703
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h704, _, _, _, _) => {
            h704
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h705, _, _, _) => {
            h705
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h706, _, _) => {
            h706
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a707 = v1;
    let a708 = 0u64;
    if if nat_eq(a707, a708) {
        true
    } else {
        let a709 = v2;
        let a710 = 0u64;
        nat_eq(a709, a710)
    } {
        Ok(0u64)
    } else {
        let a717 = {
            let a711 = f85(v1, v2, v3)?;
            let a712 = f86(v0, v1, v2)?;
            nat_add(a711, a712)?
        };
        let a718 = {
            let a715 = f87(v0);
            let a716 = {
                let a713 = v1;
                let a714 = v2;
                nat_mul(a713, a714)?
            };
            nat_mul(a715, a716)?
        };
        nat_add(a717, a718)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a719 = v0;
    let a720 = 0u64;
    if if nat_eq(a719, a720) {
        true
    } else {
        let a721 = v1;
        let a722 = 0u64;
        nat_eq(a721, a722)
    } {
        Ok(3u64)
    } else {
        let a725 = {
            let a723 = 3u64;
            let a724 = v1;
            nat_add(a723, a724)?
        };
        let a726 = v2;
        nat_add(a725, a726)
    }
}

pub fn f31(v0: u64) -> bool {
    let a727 = v0;
    let a728 = 2139095040u64;
    if nat_le(a727, a728) {
        true
    } else {
        let a729 = v0;
        let a730 = 2147483648u64;
        nat_eq(a729, a730)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a731 = v0;
    let a732 = 0u64;
    if nat_eq(a731, a732) {
        Ok(0u64)
    } else {
        let a733 = v1;
        let a734 = f88(v0)?;
        nat_mul(a733, a734)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a735 = v0;
    let a736 = 0u64;
    if if if if nat_eq(a735, a736) {
        true
    } else {
        let a737 = v1;
        let a738 = 0u64;
        nat_eq(a737, a738)
    } {
        true
    } else {
        let a739 = v2;
        let a740 = 0u64;
        nat_eq(a739, a740)
    } {
        true
    } else {
        let a741 = v3;
        let a742 = 0u64;
        nat_eq(a741, a742)
    } {
        Ok(0u64)
    } else {
        let a755 = {
            let a745 = 2u64;
            let a746 = {
                let a743 = v1;
                let a744 = v2;
                nat_mul(a743, a744)?
            };
            nat_mul(a745, a746)?
        };
        let a756 = {
            let a753 = {
                let a749 = {
                    let a747 = v1;
                    let a748 = v2;
                    nat_mul(a747, a748)?
                };
                let a750 = v3;
                nat_mul(a749, a750)?
            };
            let a754 = {
                let a751 = 1u64;
                let a752 = v0;
                nat_add(a751, a752)?
            };
            nat_mul(a753, a754)?
        };
        nat_add(a755, a756)
    }
}

pub fn f34(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h757, _, _, _, _, _, _, _, _, _, _, _) => {
            h757
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h758, _, _, _, _, _, _, _, _, _, _) => {
            h758
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h759, _, _, _, _, _, _, _, _, _) => {
            h759
        }
    };
    let v6: u64 = {
        let a761 = match v1.clone() {
            Adt2::C0(_, h760, _, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        let a762 = v4;
        let a763 = 0u64;
        nat_quot(a761, a762, a763)
    };
    let v7: u64 = {
        let a765 = match v1.clone() {
            Adt2::C0(h764, _, _, _, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a766 = 0u64;
        if nat_eq(a765, a766) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f89(match v1.clone() {
        Adt2::C0(h767, _, _, _, _, _, _, _, _, _, _) => {
            h767
        }
    }) {
        let a770 = match v1.clone() {
            Adt2::C0(h768, _, _, _, _, _, _, _, _, _, _) => {
                h768
            }
        };
        let a771 = match v2.clone() {
            Adt2::C0(h769, _, _, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        nat_eq(a770, a771)
    } else {
        false
    } {
        let a772 = 0u64;
        let a773 = v3;
        nat_lt(a772, a773)
    } else {
        false
    } {
        let a774 = v3;
        let a775 = 2147483648u64;
        nat_lt(a774, a775)
    } else {
        false
    } {
        let a776 = 0u64;
        let a777 = v4;
        nat_lt(a776, a777)
    } else {
        false
    } {
        let a778 = v4;
        let a779 = 2147483648u64;
        nat_lt(a778, a779)
    } else {
        false
    } {
        let a780 = v5;
        let a781 = 2147483648u64;
        nat_lt(a780, a781)
    } else {
        false
    } {
        let a782 = 0u64;
        let a783 = v6;
        nat_lt(a782, a783)
    } else {
        false
    } {
        let a785 = 0u64;
        let a786 = match v2.clone() {
            Adt2::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        };
        nat_lt(a785, a786)
    } else {
        false
    } {
        let a790 = match v1.clone() {
            Adt2::C0(_, h787, _, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a791 = {
            let a788 = v6;
            let a789 = v4;
            nat_mul(a788, a789)?
        };
        nat_eq(a790, a791)
    } else {
        false
    } {
        let a793 = match v2.clone() {
            Adt2::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                h792
            }
        };
        let a794 = v4;
        nat_eq(a793, a794)
    } else {
        false
    } {
        let a807 = {
            let a800 = {
                let a798 = match v2.clone() {
                    Adt2::C0(_, h795, _, _, _, _, _, _, _, _, _) => {
                        h795
                    }
                };
                let a799 = {
                    let a796 = v5;
                    let a797 = 2u64;
                    nat_mul(a796, a797)?
                };
                nat_add(a798, a799)?
            };
            let a801 = v3;
            nat_add(a800, a801)?
        };
        let a808 = {
            let a805 = {
                let a803 = match v1.clone() {
                    Adt2::C0(_, _, h802, _, _, _, _, _, _, _, _) => {
                        h802
                    }
                };
                let a804 = v3;
                nat_mul(a803, a804)?
            };
            let a806 = v6;
            nat_add(a805, a806)?
        };
        nat_eq(a807, a808)
    } else {
        false
    } {
        f90(v1.clone(), v7)?
    } else {
        false
    } {
        f90(v2.clone(), v7)?
    } else {
        false
    } {
        f18(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a809 = f12(v2.clone(), 1u64)?;
            let a810 = f91(v8.clone())?;
            nat_mul(a809, a810)?
        }, v7, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h811, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f35(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m812 = v1;
    match m812 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a814 = match v0.clone() {
                Adt2::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                    h813
                }
            };
            let a815 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a814, a815) {
                let a817 = match v3.clone() {
                    Adt2::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                        h816
                    }
                };
                let a818 = 0u64;
                nat_eq(a817, a818)
            } else {
                false
            } {
                let a820 = match v2.clone() {
                    Adt2::C0(h819, _, _, _, _, _, _, _, _, _, _) => {
                        h819
                    }
                };
                let a821 = 0u64;
                nat_eq(a820, a821)
            } else {
                false
            } {
                let a823 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h822, _, _, _, _, _, _) => {
                        h822
                    }
                };
                let a824 = 1u64;
                nat_eq(a823, a824)
            } else {
                false
            } {
                let a826 = match v3.clone() {
                    Adt2::C0(_, _, _, h825, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = 1u64;
                nat_eq(a826, a827)
            } else {
                false
            } {
                let a829 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h828, _, _, _, _, _, _) => {
                        h828
                    }
                };
                let a830 = 1u64;
                nat_eq(a829, a830)
            } else {
                false
            } {
                let a833 = match v0.clone() {
                    Adt2::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                        h831
                    }
                };
                let a834 = match v3.clone() {
                    Adt2::C0(_, _, h832, _, _, _, _, _, _, _, _) => {
                        h832
                    }
                };
                nat_eq(a833, a834)
            } else {
                false
            } {
                let a839 = match v3.clone() {
                    Adt2::C0(_, h835, _, _, _, _, _, _, _, _, _) => {
                        h835
                    }
                };
                let a840 = {
                    let a837 = match v0.clone() {
                        Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                            h836
                        }
                    };
                    let a838 = 1u64;
                    nat_add(a837, a838)?
                };
                nat_le(a839, a840)
            } else {
                false
            } {
                let a843 = match v2.clone() {
                    Adt2::C0(_, h841, _, _, _, _, _, _, _, _, _) => {
                        h841
                    }
                };
                let a844 = match v0.clone() {
                    Adt2::C0(_, _, h842, _, _, _, _, _, _, _, _) => {
                        h842
                    }
                };
                nat_eq(a843, a844)
            } else {
                false
            } {
                let a852 = match v2.clone() {
                    Adt2::C0(_, _, h845, _, _, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a853 = {
                    let a850 = {
                        let a847 = match v0.clone() {
                            Adt2::C0(_, h846, _, _, _, _, _, _, _, _, _) => {
                                h846
                            }
                        };
                        let a848 = 1u64;
                        nat_add(a847, a848)?
                    };
                    let a851 = match v3.clone() {
                        Adt2::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                            h849
                        }
                    };
                    nat_sub(a850, a851)
                };
                nat_eq(a852, a853)
            } else {
                false
            } {
                let a856 = match v2.clone() {
                    Adt2::C0(_, _, _, h854, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a857 = match v0.clone() {
                    Adt2::C0(_, _, _, h855, _, _, _, _, _, _, _) => {
                        h855
                    }
                };
                nat_eq(a856, a857)
            } else {
                false
            } {
                let a859 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h858, _, _, _, _, _, _) => {
                        h858
                    }
                };
                let a860 = 1u64;
                nat_eq(a859, a860)
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                let a862 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h861, _, _, _, _, _) => {
                        h861
                    }
                };
                let a863 = 4u64;
                nat_eq(a862, a863)
            } else {
                false
            } {
                let a865 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h864, _, _, _, _, _) => {
                        h864
                    }
                };
                let a866 = 4u64;
                nat_eq(a865, a866)
            } else {
                false
            } {
                let a871 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h867, _, _, _, _) => {
                        h867
                    }
                };
                let a872 = {
                    let a869 = match v0.clone() {
                        Adt2::C0(_, h868, _, _, _, _, _, _, _, _, _) => {
                            h868
                        }
                    };
                    let a870 = 4u64;
                    nat_mul(a869, a870)?
                };
                nat_eq(a871, a872)
            } else {
                false
            } {
                let a877 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h873, _, _, _, _) => {
                        h873
                    }
                };
                let a878 = {
                    let a875 = match v3.clone() {
                        Adt2::C0(_, h874, _, _, _, _, _, _, _, _, _) => {
                            h874
                        }
                    };
                    let a876 = 4u64;
                    nat_mul(a875, a876)?
                };
                nat_eq(a877, a878)
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
                    let a880 = f12(v2.clone(), 1u64)?;
                    let a881 = f92(match v3.clone() {
                        Adt2::C0(_, h879, _, _, _, _, _, _, _, _, _) => {
                            h879
                        }
                    })?;
                    nat_mul(a880, a881)?
                }, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C29(f16(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f36(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h882, _, _, _, _, _, _, _, _, _, _) => {
            h882
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h883, _, _, _, _, _, _, _, _) => {
                h883
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h884, _, _, _, _, _, _, _, _, _) => {
                h884
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h885, _, _, _, _, _, _) => {
                h885
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h886, _, _, _, _, _, _, _, _) => {
                h886
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h887, _, _, _, _, _, _, _, _, _) => {
                h887
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h888, _, _, _, _, _, _, _) => {
                h888
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h889, _, _, _, _, _) => {
                h889
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a891 = match v1.clone() {
        Adt1::C0(h890, _, _, _, _, _, _, _, _, _, _, _) => {
            h890
        }
    };
    let a892 = 1u64;
    if if if if if if if if if if if nat_le(a891, a892) {
        let a894 = match v2.clone() {
            Adt2::C0(h893, _, _, _, _, _, _, _, _, _, _) => {
                h893
            }
        };
        let a895 = 1u64;
        nat_le(a894, a895)
    } else {
        false
    } {
        let a897 = match v3.clone() {
            Adt2::C0(h896, _, _, _, _, _, _, _, _, _, _) => {
                h896
            }
        };
        let a898 = 0u64;
        nat_eq(a897, a898)
    } else {
        false
    } {
        let a903 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h899, _, _, _, _, _) => {
                h899
            }
        };
        let a904 = {
            let a901 = match v2.clone() {
                Adt2::C0(h900, _, _, _, _, _, _, _, _, _, _) => {
                    h900
                }
            };
            let a902 = 0u64;
            if nat_eq(a901, a902) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a903, a904)
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
        f93(v4.clone(), match v2.clone() {
            Adt2::C0(_, h905, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        }, match v3.clone() {
            Adt2::C0(_, h906, _, _, _, _, _, _, _, _, _) => {
                h906
            }
        })?
    } else {
        false
    } {
        if v0 {
            f93(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h907, _, _, _, _, _, _, _, _) => {
                    h907
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h908, _, _, _, _, _, _, _, _) => {
                    h908
                }
            })?
        } else {
            let a911 = match v2.clone() {
                Adt2::C0(_, _, h909, _, _, _, _, _, _, _, _) => {
                    h909
                }
            };
            let a912 = match v3.clone() {
                Adt2::C0(_, _, h910, _, _, _, _, _, _, _, _) => {
                    h910
                }
            };
            nat_eq(a911, a912)
        }
    } else {
        false
    } {
        let a915 = match v2.clone() {
            Adt2::C0(_, _, _, h913, _, _, _, _, _, _, _) => {
                h913
            }
        };
        let a916 = match v3.clone() {
            Adt2::C0(_, _, _, h914, _, _, _, _, _, _, _) => {
                h914
            }
        };
        nat_eq(a915, a916)
    } else {
        false
    } {
        let a919 = match v2.clone() {
            Adt2::C0(_, _, _, _, h917, _, _, _, _, _, _) => {
                h917
            }
        };
        let a920 = match v3.clone() {
            Adt2::C0(_, _, _, _, h918, _, _, _, _, _, _) => {
                h918
            }
        };
        nat_eq(a919, a920)
    } else {
        false
    } {
        let a925 = {
            let a923 = match v4.clone() {
                Adt6::C0(h921, _, _, _) => {
                    h921
                }
            };
            let a924 = match v5.clone() {
                Adt6::C0(h922, _, _, _) => {
                    h922
                }
            };
            nat_mul(a923, a924)?
        };
        let a926 = 2147483648u64;
        nat_lt(a925, a926)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a928 = match v1 {
                Adt1::C0(h927, _, _, _, _, _, _, _, _, _, _, _) => {
                    h927
                }
            };
            let a929 = 0u64;
            nat_eq(a928, a929)
        }, match v2.clone() {
            Adt2::C0(h930, _, _, _, _, _, _, _, _, _, _) => {
                h930
            }
        });
        Ok(Adt10::C4({
            let a931 = f12(v3.clone(), 1u64)?;
            let a932 = f94(v6.clone())?;
            nat_mul(a931, a932)?
        }, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f37(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m933 = v2;
    match m933 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a935 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h934, _, _, _, _, _) => {
                        h934
                    }
                };
                let a936 = 1u64;
                nat_eq(a935, a936)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h937, _, _, _, _, _, _, _, _, _) => {
                    h937
                }
            }, match v0.clone() {
                Adt1::C0(h938, _, _, _, _, _, _, _, _, _, _, _) => {
                    h938
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h939, _, _, _, _, _, _, _, _, _) => {
                    h939
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h940, _, _, _, _, _, _, _) => {
                    h940
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h941, _, _, _, _, _, _, _, _) => {
                        h941
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h942, _, _, _, _, _, _, _, _, _, _) => {
                        h942
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h943, _, _, _, _, _, _, _, _) => {
                        h943
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h944, _, _, _, _, _, _) => {
                        h944
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
                        h945
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h946, _, _, _, _, _, _, _, _) => {
                        h946
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h947, _, _, _, _, _, _) => {
                        h947
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h948, _, _, _, _, _, _, _) => {
                        h948
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
                        h949
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h950, _, _, _, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h951, _, _, _, _, _, _, _, _, _, _) => {
                    h951
                }
            };
            let v14: u64 = {
                let a952 = v12;
                let a953 = 0u64;
                if nat_eq(a952, a953) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a955 = match v4.clone() {
                    Adt2::C0(_, h954, _, _, _, _, _, _, _, _, _) => {
                        h954
                    }
                };
                let a956 = v14;
                nat_mul(a955, a956)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h957, _, _, _) => {
                        h957
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h958, _, _, _, _) => {
                        h958
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h959, _, _) => {
                        h959
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h960, _, _, _) => {
                        h960
                    }
                }
            };
            let a962 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h961, _, _, _, _, _) => {
                    h961
                }
            };
            let a963 = 1u64;
            if if if if if if if if if if nat_le(a962, a963) {
                let a964 = v12;
                let a965 = 0u64;
                if nat_eq(a964, a965) {
                    true
                } else {
                    let a966 = v12;
                    let a967 = 1u64;
                    if nat_eq(a966, a967) {
                        let a968 = v13;
                        let a969 = 1u64;
                        nat_eq(a968, a969)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a970 = v13;
                let a971 = 0u64;
                if nat_eq(a970, a971) {
                    true
                } else {
                    let a972 = v13;
                    let a973 = 1u64;
                    nat_eq(a972, a973)
                }
            } else {
                false
            } {
                let a975 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h974, _, _, _, _, _) => {
                        h974
                    }
                };
                let a976 = v14;
                nat_eq(a975, a976)
            } else {
                false
            } {
                let a984 = match v3.clone() {
                    Adt2::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a985 = {
                    let a982 = v9;
                    let a983 = {
                        let a980 = match v6.clone() {
                            Adt6::C0(h978, _, _, _) => {
                                h978
                            }
                        };
                        let a981 = match v7.clone() {
                            Adt6::C0(h979, _, _, _) => {
                                h979
                            }
                        };
                        nat_mul(a980, a981)?
                    };
                    nat_mul(a982, a983)?
                };
                nat_eq(a984, a985)
            } else {
                false
            } {
                let a988 = match v3.clone() {
                    Adt2::C0(_, _, h986, _, _, _, _, _, _, _, _) => {
                        h986
                    }
                };
                let a989 = f80(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h987, _, _, _, _, _, _, _, _, _) => {
                        h987
                    }
                })?;
                nat_eq(a988, a989)
            } else {
                false
            } {
                let a991 = 0u64;
                let a992 = match v3.clone() {
                    Adt2::C0(_, _, h990, _, _, _, _, _, _, _, _) => {
                        h990
                    }
                };
                nat_lt(a991, a992)
            } else {
                false
            } {
                if v5 {
                    let a994 = match v1 {
                        Adt2::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                            h993
                        }
                    };
                    let a995 = v9;
                    if if if nat_eq(a994, a995) {
                        let a997 = match v3.clone() {
                            Adt2::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                                h996
                            }
                        };
                        let a998 = f80(v7.clone(), v11)?;
                        nat_eq(a997, a998)
                    } else {
                        false
                    } {
                        let a1000 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h999, _, _, _, _, _, _) => {
                                h999
                            }
                        };
                        let a1001 = v10;
                        nat_eq(a1000, a1001)
                    } else {
                        false
                    } {
                        let a1003 = 0u64;
                        let a1004 = match v3.clone() {
                            Adt2::C0(_, _, _, h1002, _, _, _, _, _, _, _) => {
                                h1002
                            }
                        };
                        nat_lt(a1003, a1004)
                    } else {
                        false
                    }
                } else {
                    let a1006 = match v1 {
                        Adt2::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
                            h1005
                        }
                    };
                    let a1007 = v9;
                    if if if nat_eq(a1006, a1007) {
                        let a1009 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1008, _, _, _, _, _, _) => {
                                h1008
                            }
                        };
                        let a1010 = 1u64;
                        nat_eq(a1009, a1010)
                    } else {
                        false
                    } {
                        let a1012 = match v3.clone() {
                            Adt2::C0(_, _, _, h1011, _, _, _, _, _, _, _) => {
                                h1011
                            }
                        };
                        let a1013 = v10;
                        nat_eq(a1012, a1013)
                    } else {
                        false
                    } {
                        let a1015 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1014, _, _, _, _, _, _) => {
                                h1014
                            }
                        };
                        let a1016 = 1u64;
                        nat_eq(a1015, a1016)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h1017, _, _, _, _, _, _, _, _, _) => {
                        h1017
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1037 = {
                        let a1034 = {
                            let a1032 = {
                                let a1026 = {
                                    let a1020 = {
                                        let a1018 = v10;
                                        let a1019 = 1u64;
                                        nat_sub(a1018, a1019)
                                    };
                                    let a1021 = v17;
                                    nat_mul(a1020, a1021)?
                                };
                                let a1027 = {
                                    let a1024 = {
                                        let a1022 = v9;
                                        let a1023 = 1u64;
                                        nat_sub(a1022, a1023)
                                    };
                                    let a1025 = v16;
                                    nat_mul(a1024, a1025)?
                                };
                                nat_add(a1026, a1027)?
                            };
                            let a1033 = {
                                let a1030 = {
                                    let a1028 = v11;
                                    let a1029 = 1u64;
                                    nat_sub(a1028, a1029)
                                };
                                let a1031 = v15;
                                nat_mul(a1030, a1031)?
                            };
                            nat_add(a1032, a1033)?
                        };
                        let a1035 = v15;
                        nat_add(a1034, a1035)?
                    };
                    let a1038 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1036, _) => {
                            h1036
                        }
                    };
                    nat_le(a1037, a1038)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a1039 = v13;
                    let a1040 = 0u64;
                    if nat_eq(a1039, a1040) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1041, _, _, _, _, _, _, _, _, _) => {
                        h1041
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1042, _, _, _, _, _, _, _, _, _) => {
                            h1042
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1043, _, _, _, _, _, _, _, _) => {
                            h1043
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1044, _, _, _, _, _, _, _) => {
                            h1044
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1045, _, _, _, _, _) => {
                            h1045
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1046, _, _, _, _) => {
                            h1046
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1047, _, _, _) => {
                            h1047
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f38(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1048 = v2;
    match m1048 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1049, _, _, _, _, _, _, _, _, _) => {
                    h1049
                }
            }, match v0.clone() {
                Adt1::C0(h1050, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1050
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1051, _, _, _, _, _, _, _, _) => {
                    h1051
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1052, _, _, _, _, _) => {
                    h1052
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1053, _, _, _, _, _, _, _, _) => {
                    h1053
                }
            }, match v0.clone() {
                Adt1::C0(_, h1054, _, _, _, _, _, _, _, _, _, _) => {
                    h1054
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1055, _, _, _, _, _, _, _) => {
                    h1055
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1056, _, _, _, _) => {
                    h1056
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1057, _, _, _, _, _, _, _) => {
                    h1057
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1058, _, _, _, _, _, _, _, _, _) => {
                    h1058
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1059, _, _, _, _, _, _) => {
                    h1059
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1060, _, _, _) => {
                    h1060
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1061, _, _) => {
                    h1061
                }
            };
            let v9: u64 = f80(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                    h1062
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1063, _, _, _, _, _, _, _, _, _, _) => {
                    h1063
                }
            };
            let v11: u64 = {
                let a1071 = v8;
                let a1072 = {
                    let a1069 = match v5.clone() {
                        Adt6::C0(h1064, _, _, _) => {
                            h1064
                        }
                    };
                    let a1070 = {
                        let a1067 = match v6.clone() {
                            Adt6::C0(h1065, _, _, _) => {
                                h1065
                            }
                        };
                        let a1068 = match v7.clone() {
                            Adt6::C0(h1066, _, _, _) => {
                                h1066
                            }
                        };
                        nat_mul(a1067, a1068)?
                    };
                    nat_mul(a1069, a1070)?
                };
                nat_mul(a1071, a1072)?
            };
            let a1074 = match v4.clone() {
                Adt2::C0(h1073, _, _, _, _, _, _, _, _, _, _) => {
                    h1073
                }
            };
            let a1075 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1074, a1075) {
                let a1076 = v10;
                let a1077 = 0u64;
                if nat_eq(a1076, a1077) {
                    true
                } else {
                    let a1078 = v10;
                    let a1079 = 1u64;
                    nat_eq(a1078, a1079)
                }
            } else {
                false
            } {
                let a1080 = 0u64;
                let a1081 = v8;
                nat_lt(a1080, a1081)
            } else {
                false
            } {
                let a1082 = v8;
                let a1083 = 2147483648u64;
                nat_lt(a1082, a1083)
            } else {
                false
            } {
                let a1085 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1084, _, _, _, _, _) => {
                        h1084
                    }
                };
                let a1086 = 4u64;
                nat_eq(a1085, a1086)
            } else {
                false
            } {
                let a1087 = 0u64;
                let a1088 = v9;
                nat_lt(a1087, a1088)
            } else {
                false
            } {
                let a1090 = 0u64;
                let a1091 = match v3.clone() {
                    Adt2::C0(_, _, h1089, _, _, _, _, _, _, _, _) => {
                        h1089
                    }
                };
                nat_lt(a1090, a1091)
            } else {
                false
            } {
                let a1093 = 0u64;
                let a1094 = match v3.clone() {
                    Adt2::C0(_, _, _, h1092, _, _, _, _, _, _, _) => {
                        h1092
                    }
                };
                nat_lt(a1093, a1094)
            } else {
                false
            } {
                let a1096 = match v3.clone() {
                    Adt2::C0(_, h1095, _, _, _, _, _, _, _, _, _) => {
                        h1095
                    }
                };
                let a1097 = v11;
                nat_eq(a1096, a1097)
            } else {
                false
            } {
                let a1100 = match v3.clone() {
                    Adt2::C0(_, _, h1098, _, _, _, _, _, _, _, _) => {
                        h1098
                    }
                };
                let a1101 = f80(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1099, _, _, _, _, _, _, _, _, _) => {
                        h1099
                    }
                })?;
                nat_eq(a1100, a1101)
            } else {
                false
            } {
                let a1104 = match v3.clone() {
                    Adt2::C0(_, _, _, h1102, _, _, _, _, _, _, _) => {
                        h1102
                    }
                };
                let a1105 = f80(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                        h1103
                    }
                })?;
                nat_eq(a1104, a1105)
            } else {
                false
            } {
                let a1113 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1106, _, _, _, _, _, _) => {
                        h1106
                    }
                };
                let a1114 = {
                    let a1111 = v9;
                    let a1112 = {
                        let a1108 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1107, _, _, _, _, _, _) => {
                                h1107
                            }
                        };
                        let a1109 = v8;
                        let a1110 = 0u64;
                        nat_quot(a1108, a1109, a1110)
                    };
                    nat_mul(a1111, a1112)?
                };
                nat_eq(a1113, a1114)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a1115 = v10;
                    let a1116 = 0u64;
                    if nat_eq(a1115, a1116) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1117, _, _) => {
                        h1117
                    }
                }, {
                    let a1119 = v8;
                    let a1120 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1118, _, _) => {
                            h1118
                        }
                    };
                    nat_mul(a1119, a1120)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f39(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1121 = v1;
    match m1121 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f95(v0, v3, v2)
        }
    }
}

pub fn f40(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1123 = match v0.clone() {
        Adt2::C0(h1122, _, _, _, _, _, _, _, _, _, _) => {
            h1122
        }
    };
    let a1124 = 1u64;
    if if nat_eq(a1123, a1124) {
        true
    } else {
        let a1126 = match v0.clone() {
            Adt2::C0(h1125, _, _, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        let a1127 = 30u64;
        nat_eq(a1126, a1127)
    } {
        let a1129 = match v1.clone() {
            Adt2::C0(h1128, _, _, _, _, _, _, _, _, _, _) => {
                h1128
            }
        };
        let a1130 = 1u64;
        if if if if if if if if if if if nat_eq(a1129, a1130) {
            let a1133 = match v0.clone() {
                Adt2::C0(_, h1131, _, _, _, _, _, _, _, _, _) => {
                    h1131
                }
            };
            let a1134 = match v1.clone() {
                Adt2::C0(_, h1132, _, _, _, _, _, _, _, _, _) => {
                    h1132
                }
            };
            nat_eq(a1133, a1134)
        } else {
            false
        } {
            let a1136 = 0u64;
            let a1137 = match v1.clone() {
                Adt2::C0(_, _, h1135, _, _, _, _, _, _, _, _) => {
                    h1135
                }
            };
            nat_lt(a1136, a1137)
        } else {
            false
        } {
            let a1140 = match v1.clone() {
                Adt2::C0(_, _, h1138, _, _, _, _, _, _, _, _) => {
                    h1138
                }
            };
            let a1141 = match v1.clone() {
                Adt2::C0(_, _, _, h1139, _, _, _, _, _, _, _) => {
                    h1139
                }
            };
            nat_eq(a1140, a1141)
        } else {
            false
        } {
            let a1143 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1142, _, _, _, _, _, _) => {
                    h1142
                }
            };
            let a1144 = 1u64;
            nat_eq(a1143, a1144)
        } else {
            false
        } {
            let a1152 = match v0.clone() {
                Adt2::C0(_, _, h1145, _, _, _, _, _, _, _, _) => {
                    h1145
                }
            };
            let a1153 = {
                let a1150 = {
                    let a1148 = match v1.clone() {
                        Adt2::C0(_, _, h1146, _, _, _, _, _, _, _, _) => {
                            h1146
                        }
                    };
                    let a1149 = match v1.clone() {
                        Adt2::C0(_, _, h1147, _, _, _, _, _, _, _, _) => {
                            h1147
                        }
                    };
                    nat_add(a1148, a1149)?
                };
                let a1151 = 1u64;
                nat_sub(a1150, a1151)
            };
            nat_eq(a1152, a1153)
        } else {
            false
        } {
            let a1155 = 0u64;
            let a1156 = match v0.clone() {
                Adt2::C0(_, _, _, h1154, _, _, _, _, _, _, _) => {
                    h1154
                }
            };
            nat_lt(a1155, a1156)
        } else {
            false
        } {
            let a1158 = 0u64;
            let a1159 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1157, _, _, _, _, _, _) => {
                    h1157
                }
            };
            nat_lt(a1158, a1159)
        } else {
            false
        } {
            let a1161 = 2u64;
            let a1162 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1160, _, _, _, _, _) => {
                    h1160
                }
            };
            nat_le(a1161, a1162)
        } else {
            false
        } {
            let a1164 = 2u64;
            let a1165 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1163, _, _, _, _, _) => {
                    h1163
                }
            };
            nat_le(a1164, a1165)
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

pub fn f41(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1166 = v1;
    match m1166 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f96(v0, v3, v2)
        }
    }
}

pub fn f42(v0: u64) -> bool {
    let a1167 = v0;
    let a1168 = 1u64;
    if if nat_eq(a1167, a1168) {
        true
    } else {
        let a1169 = v0;
        let a1170 = 33u64;
        nat_eq(a1169, a1170)
    } {
        true
    } else {
        let a1171 = v0;
        let a1172 = 34u64;
        nat_eq(a1171, a1172)
    }
}

pub fn f43(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1175 = match v1.clone() {
        Adt2::C0(h1173, _, _, _, _, _, _, _, _, _, _) => {
            h1173
        }
    };
    let a1176 = match v3.clone() {
        Adt2::C0(h1174, _, _, _, _, _, _, _, _, _, _) => {
            h1174
        }
    };
    if nat_eq(a1175, a1176) {
        f97(v0, v1, v2, v3)
    } else {
        if if if if f89(match v1.clone() {
            Adt2::C0(h1177, _, _, _, _, _, _, _, _, _, _) => {
                h1177
            }
        }) {
            f89(match v3.clone() {
                Adt2::C0(h1178, _, _, _, _, _, _, _, _, _, _) => {
                    h1178
                }
            })
        } else {
            false
        } {
            let a1179 = v0;
            let a1180 = 33u64;
            if nat_eq(a1179, a1180) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1181 = f12(v1.clone(), 1u64)?;
            let a1182 = f12(v3.clone(), 1u64)?;
            nat_eq(a1181, a1182)
        } else {
            false
        } {
            let m1184 = f98(match v3.clone() {
                Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
                    h1183
                }
            });
            match m1184 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v4) => {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt15::C0(_, _, h1185) => {
                            h1185
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                            h1186
                        }
                    }, match v3 {
                        Adt2::C0(h1187, _, _, _, _, _, _, _, _, _, _) => {
                            h1187
                        }
                    })))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1189 = match v0.clone() {
        Adt2::C0(h1188, _, _, _, _, _, _, _, _, _, _) => {
            h1188
        }
    };
    let a1190 = 0u64;
    if if if if nat_eq(a1189, a1190) {
        true
    } else {
        let a1192 = match v0.clone() {
            Adt2::C0(h1191, _, _, _, _, _, _, _, _, _, _) => {
                h1191
            }
        };
        let a1193 = 1u64;
        nat_eq(a1192, a1193)
    } {
        true
    } else {
        let a1195 = match v0.clone() {
            Adt2::C0(h1194, _, _, _, _, _, _, _, _, _, _) => {
                h1194
            }
        };
        let a1196 = 25u64;
        if nat_eq(a1195, a1196) {
            true
        } else {
            let a1198 = match v0.clone() {
                Adt2::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
                    h1197
                }
            };
            let a1199 = 26u64;
            nat_eq(a1198, a1199)
        }
    } {
        true
    } else {
        let a1201 = match v0.clone() {
            Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
                h1200
            }
        };
        let a1202 = 30u64;
        nat_eq(a1201, a1202)
    } {
        let a1205 = match v0.clone() {
            Adt2::C0(h1203, _, _, _, _, _, _, _, _, _, _) => {
                h1203
            }
        };
        let a1206 = match v1.clone() {
            Adt2::C0(h1204, _, _, _, _, _, _, _, _, _, _) => {
                h1204
            }
        };
        if if nat_eq(a1205, a1206) {
            f99(match v1.clone() {
                Adt2::C0(_, h1207, _, _, _, _, _, _, _, _, _) => {
                    h1207
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1208, _, _, _, _, _, _, _, _) => {
                    h1208
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1209, _, _, _, _, _, _, _) => {
                    h1209
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1210, _, _, _, _, _, _) => {
                    h1210
                }
            }, match v0.clone() {
                Adt2::C0(_, h1211, _, _, _, _, _, _, _, _, _) => {
                    h1211
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1212, _, _, _, _, _, _, _, _) => {
                    h1212
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1213, _, _, _, _, _, _, _) => {
                    h1213
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1214, _, _, _, _, _, _) => {
                    h1214
                }
            })
        } else {
            false
        } {
            let m1216 = f98(match v0.clone() {
                Adt2::C0(h1215, _, _, _, _, _, _, _, _, _, _) => {
                    h1215
                }
            });
            match m1216 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v2) => {
                    Ok(Adt10::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt15::C0(_, _, h1217) => {
                            h1217
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

pub fn f45(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1218 = v2;
    match m1218 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1219 = v0;
            let a1220 = 4u64;
            if if if nat_lt(a1219, a1220) {
                let a1223 = match v1.clone() {
                    Adt2::C0(h1221, _, _, _, _, _, _, _, _, _, _) => {
                        h1221
                    }
                };
                let a1224 = match v4.clone() {
                    Adt2::C0(h1222, _, _, _, _, _, _, _, _, _, _) => {
                        h1222
                    }
                };
                if nat_eq(a1223, a1224) {
                    let a1227 = match v1.clone() {
                        Adt2::C0(h1225, _, _, _, _, _, _, _, _, _, _) => {
                            h1225
                        }
                    };
                    let a1228 = match v3.clone() {
                        Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                            h1226
                        }
                    };
                    nat_eq(a1227, a1228)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f100(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1229, _, _, _, _, _, _, _, _, _) => {
                        h1229
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                        h1230
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1231, _, _, _, _, _, _, _, _, _) => {
                        h1231
                    }
                })? {
                    f100(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1232, _, _, _, _, _, _, _, _) => {
                            h1232
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1233, _, _, _, _, _, _, _, _) => {
                            h1233
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1234, _, _, _, _, _, _, _, _) => {
                            h1234
                        }
                    })?
                } else {
                    false
                } {
                    f100(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1235, _, _, _, _, _, _, _) => {
                            h1235
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1236, _, _, _, _, _, _, _) => {
                            h1236
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1237, _, _, _, _, _, _, _) => {
                            h1237
                        }
                    })?
                } else {
                    false
                } {
                    f100(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1238, _, _, _, _, _, _) => {
                            h1238
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1239, _, _, _, _, _, _) => {
                            h1239
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1240, _, _, _, _, _, _) => {
                            h1240
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1242 = f98(match v1.clone() {
                    Adt2::C0(h1241, _, _, _, _, _, _, _, _, _, _) => {
                        h1241
                    }
                });
                match m1242 {
                    Adt14::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt14::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt15::C0(_, h1243, _) => {
                                h1243
                            }
                        };
                        let a1248 = {
                            let a1245 = match v1.clone() {
                                Adt2::C0(_, h1244, _, _, _, _, _, _, _, _, _) => {
                                    h1244
                                }
                            };
                            let a1246 = v6;
                            let a1247 = 1u64;
                            nat_rem(a1245, a1246, a1247)
                        };
                        let a1249 = 0u64;
                        if if if nat_eq(a1248, a1249) {
                            let a1254 = {
                                let a1251 = match v4.clone() {
                                    Adt2::C0(_, h1250, _, _, _, _, _, _, _, _, _) => {
                                        h1250
                                    }
                                };
                                let a1252 = v6;
                                let a1253 = 1u64;
                                nat_rem(a1251, a1252, a1253)
                            };
                            let a1255 = 0u64;
                            nat_eq(a1254, a1255)
                        } else {
                            false
                        } {
                            let a1260 = {
                                let a1257 = match v3.clone() {
                                    Adt2::C0(_, h1256, _, _, _, _, _, _, _, _, _) => {
                                        h1256
                                    }
                                };
                                let a1258 = v6;
                                let a1259 = 1u64;
                                nat_rem(a1257, a1258, a1259)
                            };
                            let a1261 = 0u64;
                            nat_eq(a1260, a1261)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt15::C0(_, _, h1262) => {
                                    h1262
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

pub fn f46(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1264 = match v2.clone() {
        Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
            h1263
        }
    };
    let a1265 = 0u64;
    if if nat_eq(a1264, a1265) {
        let a1267 = match v3.clone() {
            Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                h1266
            }
        };
        let a1268 = 0u64;
        nat_eq(a1267, a1268)
    } else {
        false
    } {
        let a1269 = v0;
        let a1270 = 61u64;
        if nat_eq(a1269, a1270) {
            let v4: bool = {
                let a1272 = 0u64;
                let a1273 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1271, _, _, _) => {
                        h1271
                    }
                };
                nat_lt(a1272, a1273)
            };
            if if if if if if f101(match v2.clone() {
                Adt2::C0(_, h1274, _, _, _, _, _, _, _, _, _) => {
                    h1274
                }
            }, match v1.clone() {
                Adt1::C0(h1275, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1275
                }
            }, match v1.clone() {
                Adt1::C0(_, h1276, _, _, _, _, _, _, _, _, _, _) => {
                    h1276
                }
            }, match v3.clone() {
                Adt2::C0(_, h1277, _, _, _, _, _, _, _, _, _) => {
                    h1277
                }
            })? {
                f101(match v2.clone() {
                    Adt2::C0(_, _, h1278, _, _, _, _, _, _, _, _) => {
                        h1278
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1279, _, _, _, _, _, _, _, _, _) => {
                        h1279
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1280, _, _, _, _, _, _, _, _) => {
                        h1280
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1281, _, _, _, _, _, _, _, _) => {
                        h1281
                    }
                })?
            } else {
                false
            } {
                f101(match v2.clone() {
                    Adt2::C0(_, _, _, h1282, _, _, _, _, _, _, _) => {
                        h1282
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1283, _, _, _, _, _, _, _) => {
                        h1283
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1284, _, _, _, _, _, _) => {
                        h1284
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1285, _, _, _, _, _, _, _) => {
                        h1285
                    }
                })?
            } else {
                false
            } {
                f101(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1286, _, _, _, _, _, _) => {
                        h1286
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1287, _, _, _, _, _) => {
                        h1287
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1288, _, _, _, _) => {
                        h1288
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1289, _, _, _, _, _, _) => {
                        h1289
                    }
                })?
            } else {
                false
            } {
                let a1291 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1290, _, _, _) => {
                        h1290
                    }
                };
                let a1292 = 4294967295u64;
                nat_le(a1291, a1292)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1293, _, _, _, _, _, _, _, _, _) => {
                            h1293
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1294, _, _, _, _, _, _, _, _) => {
                            h1294
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1295, _, _, _, _, _, _, _) => {
                            h1295
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1296, _, _, _, _, _, _) => {
                            h1296
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1297, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1297
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1298, _, _, _, _, _, _, _, _, _) => {
                        h1298
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1299, _, _, _, _, _, _, _) => {
                        h1299
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1300, _, _, _, _, _) => {
                        h1300
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1301 = v0;
            let a1302 = 62u64;
            if nat_eq(a1301, a1302) {
                let a1304 = match v1.clone() {
                    Adt1::C0(h1303, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1303
                    }
                };
                let a1305 = 2147483648u64;
                if if if if if nat_lt(a1304, a1305) {
                    let a1307 = match v1.clone() {
                        Adt1::C0(_, h1306, _, _, _, _, _, _, _, _, _, _) => {
                            h1306
                        }
                    };
                    let a1308 = 2147483648u64;
                    nat_lt(a1307, a1308)
                } else {
                    false
                } {
                    let a1311 = match v1.clone() {
                        Adt1::C0(h1309, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1309
                        }
                    };
                    let a1312 = match v2.clone() {
                        Adt2::C0(_, h1310, _, _, _, _, _, _, _, _, _) => {
                            h1310
                        }
                    };
                    nat_lt(a1311, a1312)
                } else {
                    false
                } {
                    let a1315 = match v1.clone() {
                        Adt1::C0(_, h1313, _, _, _, _, _, _, _, _, _, _) => {
                            h1313
                        }
                    };
                    let a1316 = match v2.clone() {
                        Adt2::C0(_, h1314, _, _, _, _, _, _, _, _, _) => {
                            h1314
                        }
                    };
                    nat_lt(a1315, a1316)
                } else {
                    false
                } {
                    let a1319 = match v2.clone() {
                        Adt2::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                            h1317
                        }
                    };
                    let a1320 = match v3.clone() {
                        Adt2::C0(_, _, h1318, _, _, _, _, _, _, _, _) => {
                            h1318
                        }
                    };
                    if if if nat_eq(a1319, a1320) {
                        let a1323 = match v2.clone() {
                            Adt2::C0(_, _, _, h1321, _, _, _, _, _, _, _) => {
                                h1321
                            }
                        };
                        let a1324 = match v3.clone() {
                            Adt2::C0(_, _, _, h1322, _, _, _, _, _, _, _) => {
                                h1322
                            }
                        };
                        nat_eq(a1323, a1324)
                    } else {
                        false
                    } {
                        let a1327 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1325, _, _, _, _, _, _) => {
                                h1325
                            }
                        };
                        let a1328 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1326, _, _, _, _, _, _) => {
                                h1326
                            }
                        };
                        nat_eq(a1327, a1328)
                    } else {
                        false
                    } {
                        let a1337 = match v3.clone() {
                            Adt2::C0(_, h1329, _, _, _, _, _, _, _, _, _) => {
                                h1329
                            }
                        };
                        let a1338 = {
                            let a1335 = {
                                let a1332 = match v2.clone() {
                                    Adt2::C0(_, h1330, _, _, _, _, _, _, _, _, _) => {
                                        h1330
                                    }
                                };
                                let a1333 = match v1.clone() {
                                    Adt1::C0(h1331, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1331
                                    }
                                };
                                nat_add(a1332, a1333)?
                            };
                            let a1336 = match v1.clone() {
                                Adt1::C0(_, h1334, _, _, _, _, _, _, _, _, _, _) => {
                                    h1334
                                }
                            };
                            nat_add(a1335, a1336)?
                        };
                        nat_eq(a1337, a1338)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1339, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1339
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1340 = v0;
                let a1341 = 63u64;
                if nat_eq(a1340, a1341) {
                    if if if if if f102(match v2.clone() {
                        Adt2::C0(_, h1342, _, _, _, _, _, _, _, _, _) => {
                            h1342
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1343, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1343
                        }
                    })? {
                        f102(match v2.clone() {
                            Adt2::C0(_, _, h1344, _, _, _, _, _, _, _, _) => {
                                h1344
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1345, _, _, _, _, _, _, _, _, _, _) => {
                                h1345
                            }
                        })?
                    } else {
                        false
                    } {
                        f102(match v2.clone() {
                            Adt2::C0(_, _, _, h1346, _, _, _, _, _, _, _) => {
                                h1346
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1347, _, _, _, _, _, _, _, _, _) => {
                                h1347
                            }
                        })?
                    } else {
                        false
                    } {
                        f102(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1348, _, _, _, _, _, _) => {
                                h1348
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1349, _, _, _, _, _, _, _, _) => {
                                h1349
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1352 = match v2.clone() {
                            Adt2::C0(_, h1350, _, _, _, _, _, _, _, _, _) => {
                                h1350
                            }
                        };
                        let a1353 = match v3.clone() {
                            Adt2::C0(_, h1351, _, _, _, _, _, _, _, _, _) => {
                                h1351
                            }
                        };
                        if if if nat_eq(a1352, a1353) {
                            let a1356 = match v2.clone() {
                                Adt2::C0(_, _, h1354, _, _, _, _, _, _, _, _) => {
                                    h1354
                                }
                            };
                            let a1357 = match v3.clone() {
                                Adt2::C0(_, _, h1355, _, _, _, _, _, _, _, _) => {
                                    h1355
                                }
                            };
                            nat_eq(a1356, a1357)
                        } else {
                            false
                        } {
                            let a1360 = match v2.clone() {
                                Adt2::C0(_, _, _, h1358, _, _, _, _, _, _, _) => {
                                    h1358
                                }
                            };
                            let a1361 = match v3.clone() {
                                Adt2::C0(_, _, _, h1359, _, _, _, _, _, _, _) => {
                                    h1359
                                }
                            };
                            nat_eq(a1360, a1361)
                        } else {
                            false
                        } {
                            let a1364 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1362, _, _, _, _, _, _) => {
                                    h1362
                                }
                            };
                            let a1365 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1363, _, _, _, _, _, _) => {
                                    h1363
                                }
                            };
                            nat_eq(a1364, a1365)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1366, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1366
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1367, _, _, _, _, _, _, _, _, _, _) => {
                                h1367
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1368, _, _, _, _, _, _, _, _, _) => {
                                h1368
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1369, _, _, _, _, _, _, _, _) => {
                                h1369
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

pub fn f47(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1370 = v3;
    match m1370 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1371 = v0;
                let a1372 = 4u64;
                nat_eq(a1371, a1372)
            };
            let v7: bool = {
                let a1374 = 0u64;
                let a1375 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1373, _, _, _, _, _, _, _) => {
                        h1373
                    }
                };
                nat_lt(a1374, a1375)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1376, _, _, _, _, _, _, _, _, _) => {
                    h1376
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1377, _, _, _, _, _, _, _, _) => {
                    h1377
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1378, _, _, _, _, _, _, _) => {
                    h1378
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1379, _, _, _, _, _, _) => {
                    h1379
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1380, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1380
                }
            }, match v1.clone() {
                Adt1::C0(_, h1381, _, _, _, _, _, _, _, _, _, _) => {
                    h1381
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1382, _, _, _, _, _, _, _, _, _) => {
                    h1382
                }
            });
            if if if if if if if if if if if if v6 {
                let a1384 = match v2.clone() {
                    Adt2::C0(h1383, _, _, _, _, _, _, _, _, _, _) => {
                        h1383
                    }
                };
                let a1385 = 0u64;
                if nat_eq(a1384, a1385) {
                    let a1387 = match v5.clone() {
                        Adt2::C0(h1386, _, _, _, _, _, _, _, _, _, _) => {
                            h1386
                        }
                    };
                    let a1388 = 0u64;
                    nat_eq(a1387, a1388)
                } else {
                    false
                }
            } else {
                let a1390 = match v2.clone() {
                    Adt2::C0(h1389, _, _, _, _, _, _, _, _, _, _) => {
                        h1389
                    }
                };
                let a1391 = 0u64;
                if if nat_eq(a1390, a1391) {
                    true
                } else {
                    let a1393 = match v2.clone() {
                        Adt2::C0(h1392, _, _, _, _, _, _, _, _, _, _) => {
                            h1392
                        }
                    };
                    let a1394 = 26u64;
                    nat_eq(a1393, a1394)
                } {
                    let a1396 = match v5.clone() {
                        Adt2::C0(h1395, _, _, _, _, _, _, _, _, _, _) => {
                            h1395
                        }
                    };
                    let a1397 = 0u64;
                    if nat_eq(a1396, a1397) {
                        true
                    } else {
                        let a1399 = match v5.clone() {
                            Adt2::C0(h1398, _, _, _, _, _, _, _, _, _, _) => {
                                h1398
                            }
                        };
                        let a1400 = 26u64;
                        nat_eq(a1399, a1400)
                    }
                } else {
                    false
                }
            } {
                let a1403 = match v2.clone() {
                    Adt2::C0(h1401, _, _, _, _, _, _, _, _, _, _) => {
                        h1401
                    }
                };
                let a1404 = match v4.clone() {
                    Adt2::C0(h1402, _, _, _, _, _, _, _, _, _, _) => {
                        h1402
                    }
                };
                nat_eq(a1403, a1404)
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
                let a1406 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1405, _, _, _, _, _) => {
                        h1405
                    }
                };
                let a1407 = 4u64;
                nat_eq(a1406, a1407)
            } else {
                false
            } {
                let a1408 = v8;
                let a1409 = f12(v2.clone(), 1u64)?;
                nat_le(a1408, a1409)
            } else {
                false
            } {
                let a1411 = match v1.clone() {
                    Adt1::C0(_, _, _, h1410, _, _, _, _, _, _, _, _) => {
                        h1410
                    }
                };
                let a1412 = 1073741824u64;
                nat_lt(a1411, a1412)
            } else {
                false
            } {
                let a1414 = match v1.clone() {
                    Adt1::C0(h1413, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1413
                    }
                };
                let a1415 = 4294967295u64;
                if if if if nat_le(a1414, a1415) {
                    let a1417 = match v1.clone() {
                        Adt1::C0(_, h1416, _, _, _, _, _, _, _, _, _, _) => {
                            h1416
                        }
                    };
                    let a1418 = 4294967295u64;
                    nat_le(a1417, a1418)
                } else {
                    false
                } {
                    let a1420 = match v1.clone() {
                        Adt1::C0(_, _, h1419, _, _, _, _, _, _, _, _, _) => {
                            h1419
                        }
                    };
                    let a1421 = 4294967295u64;
                    nat_le(a1420, a1421)
                } else {
                    false
                } {
                    let a1423 = match v1.clone() {
                        Adt1::C0(_, _, _, h1422, _, _, _, _, _, _, _, _) => {
                            h1422
                        }
                    };
                    let a1424 = 4294967295u64;
                    nat_le(a1423, a1424)
                } else {
                    false
                } {
                    let a1426 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1425, _, _, _, _, _, _, _) => {
                            h1425
                        }
                    };
                    let a1427 = 4294967295u64;
                    nat_le(a1426, a1427)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1430 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1428) => {
                            h1428
                        }
                    };
                    let a1431 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1429) => {
                            h1429
                        }
                    };
                    nat_eq(a1430, a1431)
                } else {
                    true
                }
            } else {
                false
            } {
                f103(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1432, _, _, _, _, _, _, _, _) => {
                        h1432
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1433, _) => {
                        h1433
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
                    let a1434 = v10;
                    let a1435 = v8;
                    nat_add(a1434, a1435)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1436, _, _, _, _, _, _, _, _) => {
                        h1436
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f48(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1438 = match v0.clone() {
        Adt2::C0(h1437, _, _, _, _, _, _, _, _, _, _) => {
            h1437
        }
    };
    let a1439 = 0u64;
    if if if nat_eq(a1438, a1439) {
        let a1441 = match v1.clone() {
            Adt2::C0(h1440, _, _, _, _, _, _, _, _, _, _) => {
                h1440
            }
        };
        let a1442 = 0u64;
        nat_eq(a1441, a1442)
    } else {
        false
    } {
        f99(match v0.clone() {
            Adt2::C0(_, h1443, _, _, _, _, _, _, _, _, _) => {
                h1443
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1444, _, _, _, _, _, _, _, _) => {
                h1444
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1445, _, _, _, _, _, _, _) => {
                h1445
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1446, _, _, _, _, _, _) => {
                h1446
            }
        }, match v1.clone() {
            Adt2::C0(_, h1447, _, _, _, _, _, _, _, _, _) => {
                h1447
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1448, _, _, _, _, _, _, _, _) => {
                h1448
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1449, _, _, _, _, _, _, _) => {
                h1449
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1450, _, _, _, _, _, _) => {
                h1450
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1451 = v2;
            let a1452 = f12(v0.clone(), 1u64)?;
            nat_add(a1451, a1452)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f49(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1454 = match v1.clone() {
        Adt2::C0(h1453, _, _, _, _, _, _, _, _, _, _) => {
            h1453
        }
    };
    let a1455 = 0u64;
    if if if nat_eq(a1454, a1455) {
        true
    } else {
        let a1457 = match v1.clone() {
            Adt2::C0(h1456, _, _, _, _, _, _, _, _, _, _) => {
                h1456
            }
        };
        let a1458 = 1u64;
        nat_eq(a1457, a1458)
    } {
        true
    } else {
        let a1460 = match v1.clone() {
            Adt2::C0(h1459, _, _, _, _, _, _, _, _, _, _) => {
                h1459
            }
        };
        let a1461 = 30u64;
        nat_eq(a1460, a1461)
    } {
        let a1464 = match v1.clone() {
            Adt2::C0(h1462, _, _, _, _, _, _, _, _, _, _) => {
                h1462
            }
        };
        let a1465 = match v2.clone() {
            Adt2::C0(h1463, _, _, _, _, _, _, _, _, _, _) => {
                h1463
            }
        };
        if if if if if nat_eq(a1464, a1465) {
            let a1468 = match v1.clone() {
                Adt2::C0(_, h1466, _, _, _, _, _, _, _, _, _) => {
                    h1466
                }
            };
            let a1469 = match v2.clone() {
                Adt2::C0(_, h1467, _, _, _, _, _, _, _, _, _) => {
                    h1467
                }
            };
            nat_eq(a1468, a1469)
        } else {
            false
        } {
            let a1472 = match v1.clone() {
                Adt2::C0(_, _, _, h1470, _, _, _, _, _, _, _) => {
                    h1470
                }
            };
            let a1473 = match v2.clone() {
                Adt2::C0(_, _, _, h1471, _, _, _, _, _, _, _) => {
                    h1471
                }
            };
            nat_eq(a1472, a1473)
        } else {
            false
        } {
            let a1476 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1474, _, _, _, _, _, _) => {
                    h1474
                }
            };
            let a1477 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1475, _, _, _, _, _, _) => {
                    h1475
                }
            };
            nat_eq(a1476, a1477)
        } else {
            false
        } {
            let a1478 = v0;
            let a1479 = 4u64;
            if nat_eq(a1478, a1479) {
                let a1481 = match v1.clone() {
                    Adt2::C0(_, _, h1480, _, _, _, _, _, _, _, _) => {
                        h1480
                    }
                };
                let a1482 = 1u64;
                if nat_eq(a1481, a1482) {
                    let a1485 = match v1.clone() {
                        Adt2::C0(_, h1483, _, _, _, _, _, _, _, _, _) => {
                            h1483
                        }
                    };
                    let a1486 = match v2.clone() {
                        Adt2::C0(_, _, h1484, _, _, _, _, _, _, _, _) => {
                            h1484
                        }
                    };
                    nat_eq(a1485, a1486)
                } else {
                    false
                }
            } else {
                let a1487 = v0;
                let a1488 = 4u64;
                if if nat_lt(a1487, a1488) {
                    let a1491 = match v1.clone() {
                        Adt2::C0(_, h1489, _, _, _, _, _, _, _, _, _) => {
                            h1489
                        }
                    };
                    let a1492 = match v1.clone() {
                        Adt2::C0(_, _, h1490, _, _, _, _, _, _, _, _) => {
                            h1490
                        }
                    };
                    nat_eq(a1491, a1492)
                } else {
                    false
                } {
                    let a1495 = match v1.clone() {
                        Adt2::C0(_, _, h1493, _, _, _, _, _, _, _, _) => {
                            h1493
                        }
                    };
                    let a1496 = match v2.clone() {
                        Adt2::C0(_, _, h1494, _, _, _, _, _, _, _, _) => {
                            h1494
                        }
                    };
                    nat_eq(a1495, a1496)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1498 = f98(match v1.clone() {
                Adt2::C0(h1497, _, _, _, _, _, _, _, _, _, _) => {
                    h1497
                }
            });
            match m1498 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v3) => {
                    Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt15::C0(_, _, h1499) => {
                            h1499
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

pub fn f50(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f104(match v1.clone() {
        Adt2::C0(h1500, _, _, _, _, _, _, _, _, _, _) => {
            h1500
        }
    }, v0) {
        let a1503 = match v1.clone() {
            Adt2::C0(h1501, _, _, _, _, _, _, _, _, _, _) => {
                h1501
            }
        };
        let a1504 = match v2.clone() {
            Adt2::C0(h1502, _, _, _, _, _, _, _, _, _, _) => {
                h1502
            }
        };
        nat_eq(a1503, a1504)
    } else {
        false
    } {
        let a1507 = match v1.clone() {
            Adt2::C0(_, h1505, _, _, _, _, _, _, _, _, _) => {
                h1505
            }
        };
        let a1508 = match v2.clone() {
            Adt2::C0(_, h1506, _, _, _, _, _, _, _, _, _) => {
                h1506
            }
        };
        if if if nat_eq(a1507, a1508) {
            let a1511 = match v1.clone() {
                Adt2::C0(_, _, h1509, _, _, _, _, _, _, _, _) => {
                    h1509
                }
            };
            let a1512 = match v2.clone() {
                Adt2::C0(_, _, h1510, _, _, _, _, _, _, _, _) => {
                    h1510
                }
            };
            nat_eq(a1511, a1512)
        } else {
            false
        } {
            let a1515 = match v1.clone() {
                Adt2::C0(_, _, _, h1513, _, _, _, _, _, _, _) => {
                    h1513
                }
            };
            let a1516 = match v2.clone() {
                Adt2::C0(_, _, _, h1514, _, _, _, _, _, _, _) => {
                    h1514
                }
            };
            nat_eq(a1515, a1516)
        } else {
            false
        } {
            let a1519 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1517, _, _, _, _, _, _) => {
                    h1517
                }
            };
            let a1520 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1518, _, _, _, _, _, _) => {
                    h1518
                }
            };
            nat_eq(a1519, a1520)
        } else {
            false
        }
    } else {
        false
    } {
        let m1522 = f98(match v1.clone() {
            Adt2::C0(h1521, _, _, _, _, _, _, _, _, _, _) => {
                h1521
            }
        });
        match m1522 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt15::C0(_, _, h1523) => {
                        h1523
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1524, _, _, _, _, _, _, _, _, _, _) => {
                        h1524
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f51(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1525, _, _, _, _, _, _, _, _, _, _, _) => {
                h1525
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1526, _, _, _, _, _, _, _, _, _) => {
                h1526
            }
        }
    };
    let a1527 = 0u64;
    let a1528 = v4;
    if if nat_lt(a1527, a1528) {
        let a1529 = v4;
        let a1530 = 2147483648u64;
        nat_lt(a1529, a1530)
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
        let v7: u64 = f105(match v5.clone() {
            Adt2::C0(_, _, h1531, _, _, _, _, _, _, _, _) => {
                h1531
            }
        }, v4)?;
        let v8: u64 = f105(match v5.clone() {
            Adt2::C0(_, _, _, h1532, _, _, _, _, _, _, _) => {
                h1532
            }
        }, v4)?;
        let a1534 = match v5.clone() {
            Adt2::C0(h1533, _, _, _, _, _, _, _, _, _, _) => {
                h1533
            }
        };
        let a1535 = 0u64;
        if if if if if nat_eq(a1534, a1535) {
            let a1537 = match v6.clone() {
                Adt2::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1538 = 0u64;
            nat_eq(a1537, a1538)
        } else {
            false
        } {
            let a1540 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1539, _, _, _, _, _, _) => {
                    h1539
                }
            };
            let a1541 = 1u64;
            nat_eq(a1540, a1541)
        } else {
            false
        } {
            let a1544 = match v5.clone() {
                Adt2::C0(_, h1542, _, _, _, _, _, _, _, _, _) => {
                    h1542
                }
            };
            let a1545 = match v6.clone() {
                Adt2::C0(_, h1543, _, _, _, _, _, _, _, _, _) => {
                    h1543
                }
            };
            nat_eq(a1544, a1545)
        } else {
            false
        } {
            if v0 {
                let a1547 = match v5.clone() {
                    Adt2::C0(_, _, h1546, _, _, _, _, _, _, _, _) => {
                        h1546
                    }
                };
                let a1548 = 2147483648u64;
                if if if nat_lt(a1547, a1548) {
                    let a1550 = match v5.clone() {
                        Adt2::C0(_, _, _, h1549, _, _, _, _, _, _, _) => {
                            h1549
                        }
                    };
                    let a1551 = 2147483648u64;
                    nat_lt(a1550, a1551)
                } else {
                    false
                } {
                    let a1554 = {
                        let a1552 = v7;
                        let a1553 = v8;
                        nat_mul(a1552, a1553)?
                    };
                    let a1555 = 2147483648u64;
                    nat_le(a1554, a1555)
                } else {
                    false
                } {
                    f106(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1557 = match v1.clone() {
                    Adt1::C0(h1556, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1556
                    }
                };
                let a1558 = v7;
                if if if if nat_eq(a1557, a1558) {
                    let a1560 = match v1 {
                        Adt1::C0(_, h1559, _, _, _, _, _, _, _, _, _, _) => {
                            h1559
                        }
                    };
                    let a1561 = v8;
                    nat_eq(a1560, a1561)
                } else {
                    false
                } {
                    f107(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1563 = match v6.clone() {
                        Adt2::C0(_, _, h1562, _, _, _, _, _, _, _, _) => {
                            h1562
                        }
                    };
                    let a1564 = v4;
                    if nat_eq(a1563, a1564) {
                        let a1566 = match v6.clone() {
                            Adt2::C0(_, _, _, h1565, _, _, _, _, _, _, _) => {
                                h1565
                            }
                        };
                        let a1567 = v4;
                        nat_eq(a1566, a1567)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1571 = match v6 {
                        Adt2::C0(_, _, _, _, h1568, _, _, _, _, _, _) => {
                            h1568
                        }
                    };
                    let a1572 = {
                        let a1569 = v7;
                        let a1570 = v8;
                        nat_mul(a1569, a1570)?
                    };
                    nat_eq(a1571, a1572)
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

pub fn f52(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1574 = match v2.clone() {
        Adt2::C0(h1573, _, _, _, _, _, _, _, _, _, _) => {
            h1573
        }
    };
    let a1575 = 0u64;
    if if if if if nat_eq(a1574, a1575) {
        true
    } else {
        let a1577 = match v2.clone() {
            Adt2::C0(h1576, _, _, _, _, _, _, _, _, _, _) => {
                h1576
            }
        };
        let a1578 = 1u64;
        nat_eq(a1577, a1578)
    } {
        true
    } else {
        let a1580 = match v2.clone() {
            Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
                h1579
            }
        };
        let a1581 = 30u64;
        nat_eq(a1580, a1581)
    } {
        let a1584 = match v2.clone() {
            Adt2::C0(h1582, _, _, _, _, _, _, _, _, _, _) => {
                h1582
            }
        };
        let a1585 = match v3.clone() {
            Adt2::C0(h1583, _, _, _, _, _, _, _, _, _, _) => {
                h1583
            }
        };
        nat_eq(a1584, a1585)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1587 = match v1.clone() {
                Adt1::C0(h1586, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1586
                }
            };
            let a1588 = 4294967296u64;
            nat_lt(a1587, a1588)
        } else {
            false
        } {
            let a1590 = match v1.clone() {
                Adt1::C0(_, h1589, _, _, _, _, _, _, _, _, _, _) => {
                    h1589
                }
            };
            let a1591 = 4294967296u64;
            nat_lt(a1590, a1591)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
            let a1593 = match v2.clone() {
                Adt2::C0(h1592, _, _, _, _, _, _, _, _, _, _) => {
                    h1592
                }
            };
            let a1594 = 0u64;
            if nat_eq(a1593, a1594) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1595 = v0;
            let a1596 = 31u64;
            if nat_eq(a1595, a1596) {
                0u64
            } else {
                let a1597 = v0;
                let a1598 = 47u64;
                if nat_eq(a1597, a1598) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1599, _, _, _, _, _, _, _, _, _, _) => {
                h1599
            }
        }, match v1.clone() {
            Adt1::C0(h1600, _, _, _, _, _, _, _, _, _, _, _) => {
                h1600
            }
        }, match v1 {
            Adt1::C0(_, h1601, _, _, _, _, _, _, _, _, _, _) => {
                h1601
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f53(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1603 = match v0.clone() {
        Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
            h1602
        }
    };
    let a1604 = 0u64;
    if if if nat_eq(a1603, a1604) {
        let a1606 = match v1.clone() {
            Adt2::C0(h1605, _, _, _, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1607 = 0u64;
        nat_eq(a1606, a1607)
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

pub fn f54(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1609 = match v0.clone() {
        Adt2::C0(h1608, _, _, _, _, _, _, _, _, _, _) => {
            h1608
        }
    };
    let a1610 = 0u64;
    if if nat_eq(a1609, a1610) {
        let a1612 = match v1.clone() {
            Adt2::C0(h1611, _, _, _, _, _, _, _, _, _, _) => {
                h1611
            }
        };
        let a1613 = 26u64;
        nat_eq(a1612, a1613)
    } else {
        false
    } {
        let a1615 = match v0.clone() {
            Adt2::C0(_, _, _, h1614, _, _, _, _, _, _, _) => {
                h1614
            }
        };
        let a1616 = 1u64;
        if if if if if if if nat_eq(a1615, a1616) {
            let a1618 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1617, _, _, _, _, _, _) => {
                    h1617
                }
            };
            let a1619 = 1u64;
            nat_eq(a1618, a1619)
        } else {
            false
        } {
            let a1621 = match v0.clone() {
                Adt2::C0(_, h1620, _, _, _, _, _, _, _, _, _) => {
                    h1620
                }
            };
            let a1622 = 2147483647u64;
            nat_le(a1621, a1622)
        } else {
            false
        } {
            let a1625 = match v1.clone() {
                Adt2::C0(_, h1623, _, _, _, _, _, _, _, _, _) => {
                    h1623
                }
            };
            let a1626 = match v0.clone() {
                Adt2::C0(_, _, h1624, _, _, _, _, _, _, _, _) => {
                    h1624
                }
            };
            nat_eq(a1625, a1626)
        } else {
            false
        } {
            let a1628 = match v1.clone() {
                Adt2::C0(_, _, h1627, _, _, _, _, _, _, _, _) => {
                    h1627
                }
            };
            let a1629 = 1u64;
            nat_eq(a1628, a1629)
        } else {
            false
        } {
            let a1631 = match v1.clone() {
                Adt2::C0(_, _, _, h1630, _, _, _, _, _, _, _) => {
                    h1630
                }
            };
            let a1632 = 1u64;
            nat_eq(a1631, a1632)
        } else {
            false
        } {
            let a1634 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1633, _, _, _, _, _, _) => {
                    h1633
                }
            };
            let a1635 = 1u64;
            nat_eq(a1634, a1635)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1638 = match v0.clone() {
                    Adt2::C0(_, _, h1636, _, _, _, _, _, _, _, _) => {
                        h1636
                    }
                };
                let a1639 = f108(match v0.clone() {
                    Adt2::C0(_, h1637, _, _, _, _, _, _, _, _, _) => {
                        h1637
                    }
                })?;
                nat_mul(a1638, a1639)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f55(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1640 = v1;
    match m1640 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f109(f16(v0.clone(), 1u64))?;
            let a1642 = match v0.clone() {
                Adt2::C0(h1641, _, _, _, _, _, _, _, _, _, _) => {
                    h1641
                }
            };
            let a1643 = 26u64;
            if if if nat_eq(a1642, a1643) {
                let a1645 = match v3.clone() {
                    Adt2::C0(h1644, _, _, _, _, _, _, _, _, _, _) => {
                        h1644
                    }
                };
                let a1646 = 26u64;
                nat_eq(a1645, a1646)
            } else {
                false
            } {
                let a1648 = match v2.clone() {
                    Adt2::C0(h1647, _, _, _, _, _, _, _, _, _, _) => {
                        h1647
                    }
                };
                let a1649 = 27u64;
                nat_eq(a1648, a1649)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1651 = match v2.clone() {
                        Adt2::C0(_, h1650, _, _, _, _, _, _, _, _, _) => {
                            h1650
                        }
                    };
                    let a1652 = 1u64;
                    nat_eq(a1651, a1652)
                } else {
                    false
                } {
                    let a1654 = match v2.clone() {
                        Adt2::C0(_, _, h1653, _, _, _, _, _, _, _, _) => {
                            h1653
                        }
                    };
                    let a1655 = 1u64;
                    nat_eq(a1654, a1655)
                } else {
                    false
                } {
                    let a1657 = match v2.clone() {
                        Adt2::C0(_, _, _, h1656, _, _, _, _, _, _, _) => {
                            h1656
                        }
                    };
                    let a1658 = 1u64;
                    nat_eq(a1657, a1658)
                } else {
                    false
                } {
                    let a1660 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1659, _, _, _, _, _, _) => {
                            h1659
                        }
                    };
                    let a1661 = 1u64;
                    nat_eq(a1660, a1661)
                } else {
                    false
                } {
                    let a1662 = v4;
                    let a1663 = 9223372036854775808u64;
                    nat_lt(a1662, a1663)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1666 = 2u64;
                        let a1667 = {
                            let a1664 = v4;
                            let a1665 = 2u64;
                            nat_mul(a1664, a1665)?
                        };
                        nat_add(a1666, a1667)?
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

pub fn f56(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1668, _, _, _, _, _, _, _, _, _, _) => {
            h1668
        }
    };
    let a1669 = v3;
    let a1670 = 0u64;
    if if nat_eq(a1669, a1670) {
        true
    } else {
        let a1671 = v0;
        let a1672 = 13u64;
        if nat_eq(a1671, a1672) {
            let a1673 = v3;
            let a1674 = 1u64;
            if nat_eq(a1673, a1674) {
                true
            } else {
                let a1675 = v3;
                let a1676 = 30u64;
                nat_eq(a1675, a1676)
            }
        } else {
            false
        }
    } {
        let a1678 = v3;
        let a1679 = match v2.clone() {
            Adt2::C0(h1677, _, _, _, _, _, _, _, _, _, _) => {
                h1677
            }
        };
        if if nat_eq(a1678, a1679) {
            let a1680 = v0;
            let a1681 = 13u64;
            if nat_eq(a1680, a1681) {
                let a1683 = match v2.clone() {
                    Adt2::C0(_, h1682, _, _, _, _, _, _, _, _, _) => {
                        h1682
                    }
                };
                let a1684 = 1u64;
                if if if nat_eq(a1683, a1684) {
                    let a1686 = match v2.clone() {
                        Adt2::C0(_, _, h1685, _, _, _, _, _, _, _, _) => {
                            h1685
                        }
                    };
                    let a1687 = 1u64;
                    nat_eq(a1686, a1687)
                } else {
                    false
                } {
                    let a1689 = match v2.clone() {
                        Adt2::C0(_, _, _, h1688, _, _, _, _, _, _, _) => {
                            h1688
                        }
                    };
                    let a1690 = 1u64;
                    nat_eq(a1689, a1690)
                } else {
                    false
                } {
                    let a1692 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1691, _, _, _, _, _, _) => {
                            h1691
                        }
                    };
                    let a1693 = 1u64;
                    nat_eq(a1692, a1693)
                } else {
                    false
                }
            } else {
                let a1695 = match v2.clone() {
                    Adt2::C0(_, h1694, _, _, _, _, _, _, _, _, _) => {
                        h1694
                    }
                };
                let a1696 = 1u64;
                if if if nat_eq(a1695, a1696) {
                    let a1699 = match v1.clone() {
                        Adt2::C0(_, _, h1697, _, _, _, _, _, _, _, _) => {
                            h1697
                        }
                    };
                    let a1700 = match v2.clone() {
                        Adt2::C0(_, _, h1698, _, _, _, _, _, _, _, _) => {
                            h1698
                        }
                    };
                    nat_eq(a1699, a1700)
                } else {
                    false
                } {
                    let a1703 = match v1.clone() {
                        Adt2::C0(_, _, _, h1701, _, _, _, _, _, _, _) => {
                            h1701
                        }
                    };
                    let a1704 = match v2.clone() {
                        Adt2::C0(_, _, _, h1702, _, _, _, _, _, _, _) => {
                            h1702
                        }
                    };
                    nat_eq(a1703, a1704)
                } else {
                    false
                } {
                    let a1707 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1705, _, _, _, _, _, _) => {
                            h1705
                        }
                    };
                    let a1708 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1706, _, _, _, _, _, _) => {
                            h1706
                        }
                    };
                    nat_eq(a1707, a1708)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt10::C4(f110(v4.clone(), v0, v3)?, {
                let a1709 = v3;
                let a1710 = 0u64;
                if nat_eq(a1709, a1710) {
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

pub fn f57(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1711 = v0;
    let a1712 = 89u64;
    if if nat_eq(a1711, a1712) {
        let a1713 = 8u64;
        let a1714 = v1;
        if nat_le(a1713, a1714) {
            let a1715 = v1;
            let a1716 = 9u64;
            nat_le(a1715, a1716)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1717 = v1;
            let a1718 = 12u64;
            nat_add(a1717, a1718)?
        }))
    } else {
        let a1719 = v0;
        let a1720 = 89u64;
        if if nat_eq(a1719, a1720) {
            let a1721 = v1;
            let a1722 = 4u64;
            nat_eq(a1721, a1722)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1723 = 10u64;
            let a1724 = v0;
            if if nat_le(a1723, a1724) {
                let a1725 = v0;
                let a1726 = 12u64;
                nat_le(a1725, a1726)
            } else {
                false
            } {
                Ok(Some({
                    let a1727 = v0;
                    let a1728 = 3u64;
                    nat_add(a1727, a1728)?
                }))
            } else {
                let a1729 = v0;
                let a1730 = 89u64;
                if if nat_eq(a1729, a1730) {
                    let a1731 = v1;
                    let a1732 = 7u64;
                    nat_eq(a1731, a1732)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1733 = v0;
                    let a1734 = 89u64;
                    if if nat_eq(a1733, a1734) {
                        let a1735 = v1;
                        let a1736 = 10u64;
                        nat_eq(a1735, a1736)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1737 = v0;
                        let a1738 = 89u64;
                        if if nat_eq(a1737, a1738) {
                            let a1739 = v1;
                            let a1740 = 13u64;
                            nat_eq(a1739, a1740)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1741 = v0;
                            let a1742 = 8u64;
                            if nat_eq(a1741, a1742) {
                                Ok(Some(5u64))
                            } else {
                                let a1743 = v0;
                                let a1744 = 9u64;
                                if nat_eq(a1743, a1744) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1745 = v0;
                                    let a1746 = 89u64;
                                    if nat_eq(a1745, a1746) {
                                        let a1747 = v1;
                                        let a1748 = 0u64;
                                        if nat_eq(a1747, a1748) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1749 = v1;
                                            let a1750 = 1u64;
                                            if nat_eq(a1749, a1750) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1751 = v1;
                                                let a1752 = 2u64;
                                                if nat_eq(a1751, a1752) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1753 = v1;
                                                    let a1754 = 3u64;
                                                    if nat_eq(a1753, a1754) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1755 = v1;
                                                        let a1756 = 6u64;
                                                        if nat_eq(a1755, a1756) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1757 = v1;
                                                            let a1758 = 12u64;
                                                            if nat_eq(a1757, a1758) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1759 = v1;
                                                                let a1760 = 11u64;
                                                                if nat_eq(a1759, a1760) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1761 = v1;
                                                                    let a1762 = 18u64;
                                                                    if nat_eq(a1761, a1762) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1763 = v1;
                                                                        let a1764 = 19u64;
                                                                        if nat_eq(a1763, a1764) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1765 = v1;
                                                                            let a1766 = 20u64;
                                                                            if nat_eq(a1765, a1766) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1767 = v1;
                                                                                let a1768 = 21u64;
                                                                                if nat_eq(a1767, a1768) {
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

pub fn f58(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1769 = f111(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1769 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1770 = v0;
            let a1771 = 2u64;
            if if if if nat_eq(a1770, a1771) {
                true
            } else {
                let a1772 = v0;
                let a1773 = 5u64;
                nat_eq(a1772, a1773)
            } {
                true
            } else {
                let a1774 = v0;
                let a1775 = 6u64;
                nat_eq(a1774, a1775)
            } {
                true
            } else {
                let a1776 = v0;
                let a1777 = 7u64;
                nat_eq(a1776, a1777)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1778 = v3;
                    match m1778 {
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

pub fn f59(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1780 = match v1.clone() {
        Adt2::C0(h1779, _, _, _, _, _, _, _, _, _, _) => {
            h1779
        }
    };
    let a1781 = 0u64;
    if if if if if nat_eq(a1780, a1781) {
        true
    } else {
        let a1783 = match v1.clone() {
            Adt2::C0(h1782, _, _, _, _, _, _, _, _, _, _) => {
                h1782
            }
        };
        let a1784 = 1u64;
        nat_eq(a1783, a1784)
    } {
        true
    } else {
        let a1786 = match v1.clone() {
            Adt2::C0(h1785, _, _, _, _, _, _, _, _, _, _) => {
                h1785
            }
        };
        let a1787 = 30u64;
        nat_eq(a1786, a1787)
    } {
        let a1790 = match v1.clone() {
            Adt2::C0(h1788, _, _, _, _, _, _, _, _, _, _) => {
                h1788
            }
        };
        let a1791 = match v2.clone() {
            Adt2::C0(h1789, _, _, _, _, _, _, _, _, _, _) => {
                h1789
            }
        };
        if nat_eq(a1790, a1791) {
            true
        } else {
            let a1793 = match v2.clone() {
                Adt2::C0(h1792, _, _, _, _, _, _, _, _, _, _) => {
                    h1792
                }
            };
            let a1794 = 0u64;
            nat_eq(a1793, a1794)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v2.clone(), 1u64)?, {
            let a1796 = match v2.clone() {
                Adt2::C0(h1795, _, _, _, _, _, _, _, _, _, _) => {
                    h1795
                }
            };
            let a1797 = 0u64;
            if nat_eq(a1796, a1797) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1798, _, _, _, _, _, _, _, _, _, _) => {
                h1798
            }
        }, match v2 {
            Adt2::C0(h1799, _, _, _, _, _, _, _, _, _, _) => {
                h1799
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f60(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f112(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1800, _, _, _) => {
            h1800
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1801, _, _) => {
            h1801
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1802, _) => {
            h1802
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1803) => {
            h1803
        }
    })
}

pub fn f61(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1805 = v0;
        let a1806 = match v1.clone() {
            Adt3::C0(h1804, _, _, _, _, _, _, _) => {
                h1804
            }
        };
        let a1807 = 0u64;
        nat_quot(a1805, a1806, a1807)
    };
    let v4: u64 = {
        let a1809 = v3;
        let a1810 = match v1.clone() {
            Adt3::C0(_, h1808, _, _, _, _, _, _) => {
                h1808
            }
        };
        let a1811 = 0u64;
        nat_quot(a1809, a1810, a1811)
    };
    let a1812 = v2;
    let a1813 = 0u64;
    if nat_eq(a1812, a1813) {
        let a1815 = v0;
        let a1816 = match v1 {
            Adt3::C0(h1814, _, _, _, _, _, _, _) => {
                h1814
            }
        };
        let a1817 = 0u64;
        nat_rem(a1815, a1816, a1817)
    } else {
        let a1818 = v2;
        let a1819 = 1u64;
        if nat_eq(a1818, a1819) {
            let a1821 = v3;
            let a1822 = match v1 {
                Adt3::C0(_, h1820, _, _, _, _, _, _) => {
                    h1820
                }
            };
            let a1823 = 0u64;
            nat_rem(a1821, a1822, a1823)
        } else {
            let a1824 = v2;
            let a1825 = 2u64;
            if nat_eq(a1824, a1825) {
                let a1827 = v4;
                let a1828 = match v1 {
                    Adt3::C0(_, _, h1826, _, _, _, _, _) => {
                        h1826
                    }
                };
                let a1829 = 0u64;
                nat_rem(a1827, a1828, a1829)
            } else {
                let a1831 = v4;
                let a1832 = match v1 {
                    Adt3::C0(_, _, h1830, _, _, _, _, _) => {
                        h1830
                    }
                };
                let a1833 = 0u64;
                nat_quot(a1831, a1832, a1833)
            }
        }
    }
}

pub fn f62(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1835 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1834) => {
            h1834
        }
    };
    let a1836 = 0u64;
    if nat_eq(a1835, a1836) {
        Ok(v2)
    } else {
        let a1841 = {
            let a1838 = v2;
            let a1839 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1837) => {
                    h1837
                }
            };
            nat_mul(a1838, a1839)?
        };
        let a1842 = f61(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1840, _, _, _, _, _, _) => {
                h1840
            }
        }, 3u64);
        nat_add(a1841, a1842)
    }
}

pub fn f63(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1852 = {
            let a1849 = {
                let a1844 = v2;
                let a1845 = match v0.clone() {
                    Adt6::C0(_, h1843, _, _) => {
                        h1843
                    }
                };
                nat_mul(a1844, a1845)?
            };
            let a1850 = {
                let a1847 = v3;
                let a1848 = match v0.clone() {
                    Adt6::C0(_, _, _, h1846) => {
                        h1846
                    }
                };
                nat_mul(a1847, a1848)?
            };
            nat_add(a1849, a1850)?
        };
        let a1853 = f113(match v0.clone() {
            Adt6::C0(_, _, h1851, _) => {
                h1851
            }
        });
        nat_add(a1852, a1853)?
    };
    let v5: u64 = f114(match v0 {
        Adt6::C0(_, _, h1854, _) => {
            h1854
        }
    });
    let a1855 = v5;
    let a1856 = v4;
    if if nat_le(a1855, a1856) {
        let a1859 = {
            let a1857 = v4;
            let a1858 = v5;
            nat_sub(a1857, a1858)
        };
        let a1860 = v1;
        nat_lt(a1859, a1860)
    } else {
        false
    } {
        Ok(Some({
            let a1861 = v4;
            let a1862 = v5;
            nat_sub(a1861, a1862)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f64(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1863, _, _, _, _, _) => {
            h1863
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1864, _, _, _, _, _, _) => {
            h1864
        }
    };
    let v7: u64 = f61(v1, v6.clone(), 2u64);
    let a1866 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1865) => {
            h1865
        }
    };
    let a1867 = 0u64;
    if nat_eq(a1866, a1867) {
        Ok(Adt9::C8(Some(f60(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1870 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1869, _, _, _, _) => {
                    h1869
                }
            };
            match m1870 {
                Adt3::C0(_, _, h1868, _, _, _, _, _) => {
                    h1868
                }
            }
        };
        let m1876 = f63(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1871, _) => {
                h1871
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1872, _, _, _, _, _) => {
                h1872
            }
        }, {
            let a1873 = v2;
            let a1874 = v8;
            let a1875 = 0u64;
            nat_rem(a1873, a1874, a1875)
        }, v7)?;
        match m1876 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1883 = {
                        let a1881 = {
                            let a1877 = v2;
                            let a1878 = v8;
                            let a1879 = 0u64;
                            nat_quot(a1877, a1878, a1879)
                        };
                        let a1882 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1880) => {
                                h1880
                            }
                        };
                        nat_mul(a1881, a1882)?
                    };
                    let a1884 = f61(v1, v6, 3u64);
                    nat_add(a1883, a1884)?
                };
                Ok(Adt9::C8(Some(f60(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f65(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1885 = v1;
    match m1885 {
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
            f115(f116(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f117(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f118(v0.clone(), v18);
            f115(f119(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
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

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1886 = v3;
    match m1886 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f120(v0, v1)?, f120(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f122(v0, v1, v2.clone())?, f120(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f123(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f124(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f125(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f124(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f126(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f127(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f128(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f120(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f129(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f120(v0, v1)?, f120(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f130(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f120(v0, v1)?, f120(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f120(v0, v1)?, f120(v0, v2)?))
        }
        Adt4::C15 => {
            f131(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f132(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f133(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f120(f121(v0, match v2.clone() {
                Adt3::C0(h1887, _, _, _, _, _, _, _) => {
                    h1887
                }
            }, match v2.clone() {
                Adt3::C0(_, h1888, _, _, _, _, _, _) => {
                    h1888
                }
            })?, v1)?, f120(v0, v2)?))
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
            f134(v0, v48, v49, v50)
        }
        Adt4::C26(v56, _, _, _) => {
            let a1889 = v56;
            let a1890 = 64u64;
            if if if nat_eq(a1889, a1890) {
                true
            } else {
                let a1891 = v56;
                let a1892 = 65u64;
                nat_eq(a1891, a1892)
            } {
                true
            } else {
                let a1893 = v56;
                let a1894 = 66u64;
                nat_eq(a1893, a1894)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v55) => {
            f74(v0, v1, v2, v55)
        }
        Adt4::C28(v51) => {
            f71(v0, v1, v2, v51, 0u64)
        }
        Adt4::C29(v52) => {
            f72(v0, v1, v2, v52)
        }
        Adt4::C30(v53, _) => {
            f73(v0, v1, v2, v53)
        }
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f108(match v1.clone() {
        Adt3::C0(h1895, _, _, _, _, _, _, _) => {
            h1895
        }
    })?;
    let v5: u64 = {
        let a1896 = v0;
        let a1897 = v4;
        let a1898 = 0u64;
        nat_quot(a1896, a1897, a1898)
    };
    let v6: u64 = {
        let a1899 = v0;
        let a1900 = v4;
        let a1901 = 0u64;
        nat_rem(a1899, a1900, a1901)
    };
    let a1902 = v6;
    let a1903 = 0u64;
    if nat_eq(a1902, a1903) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1904 = v6;
        let a1905 = 1u64;
        if nat_eq(a1904, a1905) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1908 = {
                let a1906 = v6;
                let a1907 = 1u64;
                nat_add(a1906, a1907)?
            };
            let a1909 = v4;
            if nat_eq(a1908, a1909) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f120(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1910 = v6;
                    let a1911 = 2u64;
                    nat_sub(a1910, a1911)
                };
                let v8: u64 = {
                    let a1912 = v7;
                    let a1913 = 3u64;
                    let a1914 = 0u64;
                    nat_quot(a1912, a1913, a1914)
                };
                let v9: u64 = {
                    let a1915 = v7;
                    let a1916 = 3u64;
                    let a1917 = 0u64;
                    nat_rem(a1915, a1916, a1917)
                };
                let v10: u64 = f120({
                    let a1921 = {
                        let a1919 = v5;
                        let a1920 = match v1.clone() {
                            Adt3::C0(h1918, _, _, _, _, _, _, _) => {
                                h1918
                            }
                        };
                        nat_mul(a1919, a1920)?
                    };
                    let a1922 = v8;
                    nat_add(a1921, a1922)?
                }, v1)?;
                let a1923 = v9;
                let a1924 = 0u64;
                if nat_eq(a1923, a1924) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1925 = v9;
                    let a1926 = 1u64;
                    if nat_eq(a1925, a1926) {
                        let a1927 = v3;
                        let a1928 = 2u64;
                        if nat_eq(a1927, a1928) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1929 = v3;
                        let a1930 = 2u64;
                        if if nat_eq(a1929, a1930) {
                            true
                        } else {
                            let a1931 = v3;
                            let a1932 = 3u64;
                            nat_eq(a1931, a1932)
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

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1934 = 2u64;
        let a1935 = match v1.clone() {
            Adt3::C0(_, h1933, _, _, _, _, _, _) => {
                h1933
            }
        };
        nat_mul(a1934, a1935)?
    };
    let a1936 = v0;
    let a1937 = v5;
    if nat_lt(a1936, a1937) {
        let a1941 = {
            let a1938 = v0;
            let a1939 = 2u64;
            let a1940 = 0u64;
            nat_rem(a1938, a1939, a1940)
        };
        let a1942 = 0u64;
        if nat_eq(a1941, a1942) {
            Ok(Adt9::C4(19u64, 4u64, f120({
                let a1943 = v0;
                let a1944 = 2u64;
                let a1945 = 0u64;
                nat_quot(a1943, a1944, a1945)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1946 = v4;
            let a1947 = 2147483648u64;
            if if nat_lt(a1946, a1947) {
                let a1949 = v4;
                let a1950 = match v2 {
                    Adt3::C0(_, h1948, _, _, _, _, _, _) => {
                        h1948
                    }
                };
                nat_lt(a1949, a1950)
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
            let a1953 = match v2.clone() {
                Adt3::C0(h1951, _, _, _, _, _, _, _) => {
                    h1951
                }
            };
            let a1954 = match v2.clone() {
                Adt3::C0(_, h1952, _, _, _, _, _, _) => {
                    h1952
                }
            };
            nat_mul(a1953, a1954)?
        };
        let a1957 = {
            let a1955 = v0;
            let a1956 = v5;
            nat_sub(a1955, a1956)
        };
        let a1958 = v6;
        if nat_lt(a1957, a1958) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f120({
                let a1959 = v0;
                let a1960 = v5;
                nat_sub(a1959, a1960)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1963 = {
                    let a1961 = v0;
                    let a1962 = v5;
                    nat_sub(a1961, a1962)
                };
                let a1964 = v6;
                nat_sub(a1963, a1964)
            };
            let v8: u64 = {
                let a1968 = 1u64;
                let a1969 = {
                    let a1966 = 2u64;
                    let a1967 = match v1.clone() {
                        Adt3::C0(h1965, _, _, _, _, _, _, _) => {
                            h1965
                        }
                    };
                    nat_mul(a1966, a1967)?
                };
                nat_add(a1968, a1969)?
            };
            let v9: u64 = {
                let a1970 = v7;
                let a1971 = v8;
                let a1972 = 0u64;
                nat_quot(a1970, a1971, a1972)
            };
            let v10: u64 = {
                let a1973 = v7;
                let a1974 = v8;
                let a1975 = 0u64;
                nat_rem(a1973, a1974, a1975)
            };
            let a1976 = v10;
            let a1977 = 0u64;
            if nat_eq(a1976, a1977) {
                Ok(Adt9::C4(19u64, 4u64, f120(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1981 = {
                    let a1978 = v10;
                    let a1979 = 2u64;
                    let a1980 = 0u64;
                    nat_rem(a1978, a1979, a1980)
                };
                let a1982 = 1u64;
                if nat_eq(a1981, a1982) {
                    Ok(Adt9::C4(6u64, 0u64, f120({
                        let a1991 = {
                            let a1984 = v9;
                            let a1985 = match v1.clone() {
                                Adt3::C0(h1983, _, _, _, _, _, _, _) => {
                                    h1983
                                }
                            };
                            nat_mul(a1984, a1985)?
                        };
                        let a1992 = {
                            let a1988 = {
                                let a1986 = v10;
                                let a1987 = 1u64;
                                nat_sub(a1986, a1987)
                            };
                            let a1989 = 2u64;
                            let a1990 = 0u64;
                            nat_quot(a1988, a1989, a1990)
                        };
                        nat_add(a1991, a1992)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f120({
                        let a2001 = {
                            let a1994 = v4;
                            let a1995 = match v1.clone() {
                                Adt3::C0(h1993, _, _, _, _, _, _, _) => {
                                    h1993
                                }
                            };
                            nat_mul(a1994, a1995)?
                        };
                        let a2002 = {
                            let a1998 = {
                                let a1996 = v10;
                                let a1997 = 1u64;
                                nat_sub(a1996, a1997)
                            };
                            let a1999 = 2u64;
                            let a2000 = 0u64;
                            nat_quot(a1998, a1999, a2000)
                        };
                        nat_add(a2001, a2002)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f120({
                        let a2011 = {
                            let a2004 = v4;
                            let a2005 = match v1 {
                                Adt3::C0(h2003, _, _, _, _, _, _, _) => {
                                    h2003
                                }
                            };
                            nat_mul(a2004, a2005)?
                        };
                        let a2012 = {
                            let a2008 = {
                                let a2006 = v10;
                                let a2007 = 1u64;
                                nat_sub(a2006, a2007)
                            };
                            let a2009 = 2u64;
                            let a2010 = 0u64;
                            nat_quot(a2008, a2009, a2010)
                        };
                        nat_add(a2011, a2012)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f69(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2013 = v1;
    let a2014 = f135(v2.clone(), v4.clone())?;
    if nat_lt(a2013, a2014) {
        let a2017 = v1;
        let a2018 = {
            let a2015 = 2u64;
            let a2016 = f109(v4.clone())?;
            nat_mul(a2015, a2016)?
        };
        if nat_lt(a2017, a2018) {
            let a2022 = {
                let a2019 = v1;
                let a2020 = 2u64;
                let a2021 = 0u64;
                nat_rem(a2019, a2020, a2021)
            };
            let a2023 = 0u64;
            if nat_eq(a2022, a2023) {
                f136({
                    let a2024 = v1;
                    let a2025 = 2u64;
                    let a2026 = 0u64;
                    nat_quot(a2024, a2025, a2026)
                }, v4)
            } else {
                if f137(match v2 {
                    Adt3::C0(_, h2027, _, _, _, _, _, _) => {
                        h2027
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2030 = v1;
                let a2031 = {
                    let a2028 = 2u64;
                    let a2029 = f109(v4.clone())?;
                    nat_mul(a2028, a2029)?
                };
                nat_sub(a2030, a2031)
            };
            let v7: u64 = {
                let a2035 = v6;
                let a2036 = {
                    let a2033 = 1u64;
                    let a2034 = match v2.clone() {
                        Adt3::C0(h2032, _, _, _, _, _, _, _) => {
                            h2032
                        }
                    };
                    nat_add(a2033, a2034)?
                };
                let a2037 = 0u64;
                nat_quot(a2035, a2036, a2037)
            };
            let v8: u64 = {
                let a2041 = v6;
                let a2042 = {
                    let a2039 = 1u64;
                    let a2040 = match v2.clone() {
                        Adt3::C0(h2038, _, _, _, _, _, _, _) => {
                            h2038
                        }
                    };
                    nat_add(a2039, a2040)?
                };
                let a2043 = 0u64;
                nat_rem(a2041, a2042, a2043)
            };
            let a2044 = v8;
            let a2045 = 0u64;
            if nat_eq(a2044, a2045) {
                f136(v7, v4)
            } else {
                f138(v0, v7, {
                    let a2046 = v8;
                    let a2047 = 1u64;
                    nat_sub(a2046, a2047)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2050 = match v4.clone() {
            Adt3::C0(h2048, _, _, _, _, _, _, _) => {
                h2048
            }
        };
        let a2051 = match v4.clone() {
            Adt3::C0(_, h2049, _, _, _, _, _, _) => {
                h2049
            }
        };
        nat_mul(a2050, a2051)?
    };
    let v7: u64 = {
        let a2052 = 2u64;
        let a2053 = v6;
        nat_mul(a2052, a2053)?
    };
    let v8: u64 = f139(match v3.clone() {
        Adt3::C0(_, h2054, _, _, _, _, _, _) => {
            h2054
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2055, _, _, _, _, _) => {
            h2055
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2056, _, _, _, _) => {
            h2056
        }
    });
    let a2057 = v0;
    let a2058 = v7;
    if nat_lt(a2057, a2058) {
        let a2062 = {
            let a2059 = v0;
            let a2060 = 2u64;
            let a2061 = 0u64;
            nat_rem(a2059, a2060, a2061)
        };
        let a2063 = 0u64;
        if nat_eq(a2062, a2063) {
            Ok(Adt9::C4(19u64, 5u64, f120({
                let a2064 = v0;
                let a2065 = 2u64;
                let a2066 = 0u64;
                nat_quot(a2064, a2065, a2066)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f140(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2067 = v0;
            let a2068 = v7;
            nat_sub(a2067, a2068)
        };
        let v10: u64 = {
            let a2072 = v9;
            let a2073 = {
                let a2070 = 1u64;
                let a2071 = match v1.clone() {
                    Adt3::C0(h2069, _, _, _, _, _, _, _) => {
                        h2069
                    }
                };
                nat_add(a2070, a2071)?
            };
            let a2074 = 0u64;
            nat_quot(a2072, a2073, a2074)
        };
        let v11: u64 = {
            let a2078 = v9;
            let a2079 = {
                let a2076 = 1u64;
                let a2077 = match v1.clone() {
                    Adt3::C0(h2075, _, _, _, _, _, _, _) => {
                        h2075
                    }
                };
                nat_add(a2076, a2077)?
            };
            let a2080 = 0u64;
            nat_rem(a2078, a2079, a2080)
        };
        let a2081 = v11;
        let a2082 = 0u64;
        if nat_eq(a2081, a2082) {
            Ok(Adt9::C4(19u64, 5u64, f120({
                let a2083 = v10;
                let a2084 = v6;
                let a2085 = 0u64;
                nat_rem(a2083, a2084, a2085)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f140(v5, v8) {
                let v12: u64 = {
                    let a2091 = {
                        let a2087 = v10;
                        let a2088 = match v1.clone() {
                            Adt3::C0(h2086, _, _, _, _, _, _, _) => {
                                h2086
                            }
                        };
                        nat_mul(a2087, a2088)?
                    };
                    let a2092 = {
                        let a2089 = v11;
                        let a2090 = 1u64;
                        nat_sub(a2089, a2090)
                    };
                    nat_add(a2091, a2092)?
                };
                let a2093 = v5;
                let a2094 = 4294967295u64;
                if nat_eq(a2093, a2094) {
                    Ok(Adt9::C1(false, f120(v12, v1)?, f120(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f120(v12, v1)?, 4u64, f120({
                        let a2100 = {
                            let a2096 = v5;
                            let a2097 = match v3.clone() {
                                Adt3::C0(h2095, _, _, _, _, _, _, _) => {
                                    h2095
                                }
                            };
                            nat_mul(a2096, a2097)?
                        };
                        let a2101 = {
                            let a2098 = v11;
                            let a2099 = 1u64;
                            nat_sub(a2098, a2099)
                        };
                        nat_add(a2100, a2101)?
                    }, v3)?, 3u64, f120(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f94(v3.clone())?;
    let v6: u64 = {
        let a2102 = v0;
        let a2103 = v5;
        let a2104 = 0u64;
        nat_quot(a2102, a2103, a2104)
    };
    let v7: u64 = {
        let a2105 = v0;
        let a2106 = v5;
        let a2107 = 0u64;
        nat_rem(a2105, a2106, a2107)
    };
    let v8: u64 = f141(v3.clone())?;
    let a2108 = v7;
    let a2109 = 0u64;
    if nat_eq(a2108, a2109) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2110, _) => {
                h2110
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2114 = v5;
        let a2115 = {
            let a2112 = v7;
            let a2113 = if match v3.clone() {
                Adt7::C0(_, _, _, h2111, _) => {
                    h2111
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2112, a2113)?
        };
        if nat_le(a2114, a2115) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2116, _) => {
                    h2116
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f120(v6, v2)?))
            } else {
                let a2119 = {
                    let a2117 = v7;
                    let a2118 = 2u64;
                    nat_add(a2117, a2118)?
                };
                let a2120 = v5;
                if nat_eq(a2119, a2120) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2121, _, _) => {
                            h2121
                        }
                    } {
                        let a2128 = {
                            let m2124 = match v3.clone() {
                                Adt7::C0(h2123, _, _, _, _) => {
                                    h2123
                                }
                            };
                            match m2124 {
                                Adt6::C0(h2122, _, _, _) => {
                                    h2122
                                }
                            }
                        };
                        let a2129 = {
                            let m2127 = match v3 {
                                Adt7::C0(_, h2126, _, _, _) => {
                                    h2126
                                }
                            };
                            match m2127 {
                                Adt6::C0(h2125, _, _, _) => {
                                    h2125
                                }
                            }
                        };
                        nat_mul(a2128, a2129)?
                    } else {
                        f142(match v3 {
                            Adt7::C0(h2130, _, _, _, _) => {
                                h2130
                            }
                        }, match v1 {
                            Adt3::C0(h2131, _, _, _, _, _, _, _) => {
                                h2131
                            }
                        }, f61(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2132 = v9;
                        let a2133 = 0u64;
                        if nat_eq(a2132, a2133) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f120(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2136 = {
                    let a2134 = v7;
                    let a2135 = 1u64;
                    nat_sub(a2134, a2135)
                };
                let a2137 = v8;
                let a2138 = 0u64;
                nat_rem(a2136, a2137, a2138)
            };
            let m2144 = f143(v3.clone(), v1, v2, v6, {
                let a2141 = {
                    let a2139 = v7;
                    let a2140 = 1u64;
                    nat_sub(a2139, a2140)
                };
                let a2142 = v8;
                let a2143 = 0u64;
                nat_quot(a2141, a2142, a2143)
            })?;
            match m2144 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2146 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2145) => {
                            h2145
                        }
                    };
                    let a2147 = 1u64;
                    if if nat_eq(a2146, a2147) {
                        let a2148 = v10;
                        let a2149 = 0u64;
                        nat_eq(a2148, a2149)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2150, _) => {
                                h2150
                            }
                        } {
                            let a2153 = {
                                let a2151 = v10;
                                let a2152 = 1u64;
                                nat_add(a2151, a2152)?
                            };
                            let a2154 = v8;
                            if nat_eq(a2153, a2154) {
                                let a2155 = v4;
                                let a2156 = 2u64;
                                if nat_eq(a2155, a2156) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2158 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2157) => {
                                                h2157
                                            }
                                        };
                                        let a2159 = 1u64;
                                        if nat_eq(a2158, a2159) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2161 = match v3 {
                                            Adt7::C0(_, _, _, _, h2160) => {
                                                h2160
                                            }
                                        };
                                        let a2162 = 1u64;
                                        if nat_eq(a2161, a2162) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2164 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2163) => {
                                            h2163
                                        }
                                    };
                                    let a2165 = 1u64;
                                    if nat_eq(a2164, a2165) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2167 = match v3 {
                                        Adt7::C0(_, _, _, _, h2166) => {
                                            h2166
                                        }
                                    };
                                    let a2168 = 1u64;
                                    if nat_eq(a2167, a2168) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2170 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2169) => {
                                        h2169
                                    }
                                };
                                let a2171 = 1u64;
                                if nat_eq(a2170, a2171) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2173 = match v3 {
                                    Adt7::C0(_, _, _, _, h2172) => {
                                        h2172
                                    }
                                };
                                let a2174 = 1u64;
                                if nat_eq(a2173, a2174) {
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

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f92(match v3.clone() {
        Adt3::C0(h2175, _, _, _, _, _, _, _) => {
            h2175
        }
    })?;
    let v5: u64 = {
        let a2176 = v0;
        let a2177 = v4;
        let a2178 = 0u64;
        nat_rem(a2176, a2177, a2178)
    };
    let v6: u64 = {
        let a2179 = v0;
        let a2180 = v4;
        let a2181 = 0u64;
        nat_quot(a2179, a2180, a2181)
    };
    let a2182 = v5;
    let a2183 = 0u64;
    if nat_eq(a2182, a2183) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2186 = {
            let a2184 = v5;
            let a2185 = 1u64;
            nat_add(a2184, a2185)?
        };
        let a2187 = v4;
        if nat_eq(a2186, a2187) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f120(v6, v2)?))
        } else {
            let a2191 = {
                let a2188 = v5;
                let a2189 = 2u64;
                let a2190 = 0u64;
                nat_rem(a2188, a2189, a2190)
            };
            let a2192 = 0u64;
            if nat_eq(a2191, a2192) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2195 = {
                        let a2193 = v5;
                        let a2194 = 1u64;
                        nat_sub(a2193, a2194)
                    };
                    let a2196 = 2u64;
                    let a2197 = 0u64;
                    nat_quot(a2195, a2196, a2197)
                };
                let v8: u64 = f61(v6, v2.clone(), 0u64);
                let v9: u64 = f61(v6, v2.clone(), 1u64);
                let v10: u64 = f61(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f60(v1, {
                    let a2198 = v9;
                    let a2199 = v7;
                    nat_add(a2198, a2199)?
                }, v8, v10, 0u64)?, 4u64, f60(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f91(v3.clone())?;
    let v5: u64 = {
        let a2200 = v0;
        let a2201 = v4;
        let a2202 = 0u64;
        nat_rem(a2200, a2201, a2202)
    };
    let v6: u64 = {
        let a2203 = v0;
        let a2204 = v4;
        let a2205 = 0u64;
        nat_quot(a2203, a2204, a2205)
    };
    let a2206 = v5;
    let a2207 = 0u64;
    if nat_eq(a2206, a2207) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2210 = {
            let a2208 = v5;
            let a2209 = 1u64;
            nat_add(a2208, a2209)?
        };
        let a2211 = v4;
        if nat_eq(a2210, a2211) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f120(v6, v2)?))
        } else {
            let a2215 = {
                let a2212 = v5;
                let a2213 = 2u64;
                let a2214 = 0u64;
                nat_rem(a2212, a2213, a2214)
            };
            let a2216 = 0u64;
            if nat_eq(a2215, a2216) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2218 = f61(v6, v2.clone(), 0u64);
                    let a2219 = match v3.clone() {
                        Adt6::C0(_, _, h2217, _) => {
                            h2217
                        }
                    };
                    nat_add(a2218, a2219)?
                };
                let v8: u64 = {
                    let a2227 = {
                        let a2224 = {
                            let a2221 = v7;
                            let a2222 = match v3.clone() {
                                Adt6::C0(_, h2220, _, _) => {
                                    h2220
                                }
                            };
                            nat_add(a2221, a2222)?
                        };
                        let a2225 = match v3.clone() {
                            Adt6::C0(h2223, _, _, _) => {
                                h2223
                            }
                        };
                        nat_sub(a2224, a2225)
                    };
                    let a2228 = match v3.clone() {
                        Adt6::C0(_, h2226, _, _) => {
                            h2226
                        }
                    };
                    let a2229 = 0u64;
                    nat_quot(a2227, a2228, a2229)
                };
                let v9: u64 = {
                    let a2235 = v8;
                    let a2236 = {
                        let a2232 = {
                            let a2230 = v5;
                            let a2231 = 1u64;
                            nat_sub(a2230, a2231)
                        };
                        let a2233 = 2u64;
                        let a2234 = 0u64;
                        nat_quot(a2232, a2233, a2234)
                    };
                    nat_add(a2235, a2236)?
                };
                let v10: u64 = {
                    let a2238 = v9;
                    let a2239 = match v3.clone() {
                        Adt6::C0(_, h2237, _, _) => {
                            h2237
                        }
                    };
                    nat_mul(a2238, a2239)?
                };
                let v11: u64 = {
                    let a2240 = v7;
                    let a2241 = v10;
                    nat_sub(a2240, a2241)
                };
                let a2243 = v9;
                let a2244 = match v1.clone() {
                    Adt3::C0(_, h2242, _, _, _, _, _, _) => {
                        h2242
                    }
                };
                if if if nat_lt(a2243, a2244) {
                    let a2245 = v10;
                    let a2246 = v7;
                    nat_le(a2245, a2246)
                } else {
                    false
                } {
                    let a2248 = v11;
                    let a2249 = match v3.clone() {
                        Adt6::C0(h2247, _, _, _) => {
                            h2247
                        }
                    };
                    nat_lt(a2248, a2249)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f60(v1, {
                        let a2253 = {
                            let a2251 = f61(v6, v2, 1u64);
                            let a2252 = match v3 {
                                Adt6::C0(h2250, _, _, _) => {
                                    h2250
                                }
                            };
                            nat_mul(a2251, a2252)?
                        };
                        let a2254 = v11;
                        nat_add(a2253, a2254)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f61(v0, v2.clone(), 0u64);
    let v5: u64 = f61(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2256 = v5;
        let a2257 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2255, _) => {
                h2255
            }
        };
        let a2258 = 0u64;
        nat_rem(a2256, a2257, a2258)
    };
    let v7: u64 = {
        let a2260 = v5;
        let a2261 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2259, _) => {
                h2259
            }
        };
        let a2262 = 0u64;
        nat_quot(a2260, a2261, a2262)
    };
    let v8: Option<u64> = f63(match v3.clone() {
        Adt5::C0(h2263, _, _, _, _, _, _, _) => {
            h2263
        }
    }, match v1.clone() {
        Adt3::C0(h2264, _, _, _, _, _, _, _) => {
            h2264
        }
    }, f61(v0, v2.clone(), 1u64), {
        let a2268 = v4;
        let a2269 = {
            let m2267 = match v3.clone() {
                Adt5::C0(h2266, _, _, _, _, _, _, _) => {
                    h2266
                }
            };
            match m2267 {
                Adt6::C0(h2265, _, _, _) => {
                    h2265
                }
            }
        };
        let a2270 = 0u64;
        nat_rem(a2268, a2269, a2270)
    })?;
    let v9: u64 = {
        let a2274 = v4;
        let a2275 = {
            let m2273 = match v3.clone() {
                Adt5::C0(h2272, _, _, _, _, _, _, _) => {
                    h2272
                }
            };
            match m2273 {
                Adt6::C0(h2271, _, _, _) => {
                    h2271
                }
            }
        };
        let a2276 = 0u64;
        nat_quot(a2274, a2275, a2276)
    };
    let v10: Option<u64> = f63(match v3.clone() {
        Adt5::C0(_, h2277, _, _, _, _, _, _) => {
            h2277
        }
    }, match v1.clone() {
        Adt3::C0(_, h2278, _, _, _, _, _, _) => {
            h2278
        }
    }, f61(v0, v2.clone(), 2u64), {
        let a2282 = v9;
        let a2283 = {
            let m2281 = match v3.clone() {
                Adt5::C0(_, h2280, _, _, _, _, _, _) => {
                    h2280
                }
            };
            match m2281 {
                Adt6::C0(h2279, _, _, _) => {
                    h2279
                }
            }
        };
        let a2284 = 0u64;
        nat_rem(a2282, a2283, a2284)
    })?;
    let v11: u64 = {
        let a2288 = v9;
        let a2289 = {
            let m2287 = match v3.clone() {
                Adt5::C0(_, h2286, _, _, _, _, _, _) => {
                    h2286
                }
            };
            match m2287 {
                Adt6::C0(h2285, _, _, _) => {
                    h2285
                }
            }
        };
        let a2290 = 0u64;
        nat_quot(a2288, a2289, a2290)
    };
    let v12: Option<u64> = f63(match v3.clone() {
        Adt5::C0(_, _, h2291, _, _, _, _, _) => {
            h2291
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2292, _, _, _, _, _) => {
            h2292
        }
    }, v6, {
        let a2296 = v11;
        let a2297 = {
            let m2295 = match v3.clone() {
                Adt5::C0(_, _, h2294, _, _, _, _, _) => {
                    h2294
                }
            };
            match m2295 {
                Adt6::C0(h2293, _, _, _) => {
                    h2293
                }
            }
        };
        let a2298 = 0u64;
        nat_rem(a2296, a2297, a2298)
    })?;
    let v13: u64 = {
        let a2302 = v11;
        let a2303 = {
            let m2301 = match v3.clone() {
                Adt5::C0(_, _, h2300, _, _, _, _, _) => {
                    h2300
                }
            };
            match m2301 {
                Adt6::C0(h2299, _, _, _) => {
                    h2299
                }
            }
        };
        let a2304 = 0u64;
        nat_quot(a2302, a2303, a2304)
    };
    let v17: Adt12 = {
        let m2305 = v8;
        match m2305 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2306 = v10;
                match m2306 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2307 = v12;
                        match m2307 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2316 = {
                                        let a2311 = f60(v1, v14, v15, v16, 0u64)?;
                                        let a2312 = {
                                            let a2309 = v7;
                                            let a2310 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2308, _, _, _) => {
                                                    h2308
                                                }
                                            };
                                            nat_mul(a2309, a2310)?
                                        };
                                        nat_add(a2311, a2312)?
                                    };
                                    let a2317 = {
                                        let a2314 = v13;
                                        let a2315 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2313, _, _, _, _) => {
                                                h2313
                                            }
                                        };
                                        nat_mul(a2314, a2315)?
                                    };
                                    nat_add(a2316, a2317)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2318, _, _) => {
            h2318
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2319) => {
            h2319
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f120(v0, v2)?), {
        let a2321 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2320, _, _) => {
                h2320
            }
        };
        let a2322 = 0u64;
        if nat_eq(a2321, a2322) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2324 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2323) => {
                h2323
            }
        };
        let a2325 = 0u64;
        if nat_eq(a2324, a2325) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2328 = v6;
        let a2329 = {
            let a2326 = 3u64;
            let a2327 = v5;
            nat_add(a2326, a2327)?
        };
        nat_sub(a2328, a2329)
    };
    let a2331 = v0;
    let a2332 = f29(v3, match v1.clone() {
        Adt3::C0(h2330, _, _, _, _, _, _, _) => {
            h2330
        }
    }, v5, v8)?;
    if nat_lt(a2331, a2332) {
        let v9: u64 = f85(match v1.clone() {
            Adt3::C0(h2333, _, _, _, _, _, _, _) => {
                h2333
            }
        }, v5, v8)?;
        let v10: u64 = f86(v3, match v1.clone() {
            Adt3::C0(h2334, _, _, _, _, _, _, _) => {
                h2334
            }
        }, v5)?;
        let a2335 = v0;
        let a2336 = v9;
        if nat_lt(a2335, a2336) {
            f144(v0, v2, f23(v3), {
                let a2337 = 3u64;
                let a2338 = v5;
                nat_add(a2337, a2338)?
            }, v7)
        } else {
            let a2341 = v0;
            let a2342 = {
                let a2339 = v9;
                let a2340 = v10;
                nat_add(a2339, a2340)?
            };
            if nat_lt(a2341, a2342) {
                let v11: u64 = {
                    let a2343 = v0;
                    let a2344 = v9;
                    nat_sub(a2343, a2344)
                };
                let v12: u64 = f145(v3, match v1.clone() {
                    Adt3::C0(h2345, _, _, _, _, _, _, _) => {
                        h2345
                    }
                })?;
                f146({
                    let a2346 = v11;
                    let a2347 = v12;
                    let a2348 = 0u64;
                    nat_rem(a2346, a2347, a2348)
                }, {
                    let a2349 = v11;
                    let a2350 = v12;
                    let a2351 = 0u64;
                    nat_quot(a2349, a2350, a2351)
                }, v1, v3, v4, v7)
            } else {
                f147({
                    let a2354 = v0;
                    let a2355 = {
                        let a2352 = v9;
                        let a2353 = v10;
                        nat_add(a2352, a2353)?
                    };
                    nat_sub(a2354, a2355)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2356 = v0;
    let a2357 = f15(v1)?;
    if nat_lt(a2356, a2357) {
        let a2358 = v0;
        let a2359 = 4u64;
        if nat_lt(a2358, a2359) {
            let a2360 = v0;
            let a2361 = 0u64;
            if nat_eq(a2360, a2361) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2362 = v0;
                let a2363 = 1u64;
                if nat_eq(a2362, a2363) {
                    let a2364 = v5;
                    let a2365 = 0u64;
                    if nat_eq(a2364, a2365) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2366 = v0;
                    let a2367 = 2u64;
                    if nat_eq(a2366, a2367) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2368 = v0;
            let a2369 = 4u64;
            if if nat_eq(a2368, a2369) {
                let a2370 = f148(v5)?;
                let a2371 = v1;
                !nat_eq(a2370, a2371)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2374 = {
                        let a2372 = v0;
                        let a2373 = 4u64;
                        nat_sub(a2372, a2373)
                    };
                    let a2375 = 3u64;
                    let a2376 = 0u64;
                    nat_quot(a2374, a2375, a2376)
                };
                let v7: u64 = {
                    let a2379 = {
                        let a2377 = v0;
                        let a2378 = 4u64;
                        nat_sub(a2377, a2378)
                    };
                    let a2380 = 3u64;
                    let a2381 = 0u64;
                    nat_rem(a2379, a2380, a2381)
                };
                let a2382 = v7;
                let a2383 = 0u64;
                if nat_eq(a2382, a2383) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2384 = v7;
                    let a2385 = 1u64;
                    if nat_eq(a2384, a2385) {
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

pub fn f77(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2386 = v1;
    let a2387 = 0u64;
    if nat_eq(a2386, a2387) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2388 = v1;
        let a2389 = 1u64;
        if nat_eq(a2388, a2389) {
            Adt11::C1(v2)
        } else {
            let a2390 = v1;
            let a2391 = 2u64;
            if nat_eq(a2390, a2391) {
                Adt11::C5(v2)
            } else {
                let a2392 = v1;
                let a2393 = 3u64;
                if nat_eq(a2392, a2393) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2394 = v1;
                    let a2395 = 4u64;
                    if nat_eq(a2394, a2395) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64) -> Adt12 {
    let a2396 = v0;
    let a2397 = 0u64;
    if nat_eq(a2396, a2397) {
        Adt12::C0(0u64, v1)
    } else {
        let a2398 = v0;
        let a2399 = 1u64;
        if nat_eq(a2398, a2399) {
            Adt12::C1(v1)
        } else {
            let a2400 = v0;
            let a2401 = 2u64;
            if nat_eq(a2400, a2401) {
                Adt12::C2(v1)
            } else {
                let a2402 = v0;
                let a2403 = 3u64;
                if nat_eq(a2402, a2403) {
                    Adt12::C3(v1)
                } else {
                    let a2404 = v0;
                    let a2405 = 13u64;
                    if nat_lt(a2404, a2405) {
                        Adt12::C0({
                            let a2406 = v0;
                            let a2407 = 3u64;
                            nat_sub(a2406, a2407)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: u64) -> Adt13 {
    let a2408 = v0;
    let a2409 = 1u64;
    if nat_eq(a2408, a2409) {
        Adt13::C0(v1)
    } else {
        let a2410 = v0;
        let a2411 = 3u64;
        if if nat_eq(a2410, a2411) {
            true
        } else {
            let a2412 = v0;
            let a2413 = 6u64;
            nat_eq(a2412, a2413)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f80(v0: Adt6, v1: u64) -> R<u64> {
    let a2415 = 0u64;
    let a2416 = match v0.clone() {
        Adt6::C0(h2414, _, _, _) => {
            h2414
        }
    };
    if if if if if nat_lt(a2415, a2416) {
        let a2418 = 0u64;
        let a2419 = match v0.clone() {
            Adt6::C0(_, h2417, _, _) => {
                h2417
            }
        };
        nat_lt(a2418, a2419)
    } else {
        false
    } {
        let a2421 = 0u64;
        let a2422 = match v0.clone() {
            Adt6::C0(_, _, _, h2420) => {
                h2420
            }
        };
        nat_lt(a2421, a2422)
    } else {
        false
    } {
        let a2424 = match v0.clone() {
            Adt6::C0(_, h2423, _, _) => {
                h2423
            }
        };
        let a2425 = 2147483648u64;
        nat_lt(a2424, a2425)
    } else {
        false
    } {
        let a2427 = match v0.clone() {
            Adt6::C0(_, _, _, h2426) => {
                h2426
            }
        };
        let a2428 = 2147483648u64;
        nat_lt(a2427, a2428)
    } else {
        false
    } {
        let v2: u64 = {
            let a2432 = v1;
            let a2433 = {
                let a2430 = 2u64;
                let a2431 = f114(match v0.clone() {
                    Adt6::C0(_, _, h2429, _) => {
                        h2429
                    }
                });
                nat_mul(a2430, a2431)?
            };
            nat_add(a2432, a2433)?
        };
        let v3: u64 = {
            let a2445 = {
                let a2440 = 1u64;
                let a2441 = {
                    let a2438 = match v0.clone() {
                        Adt6::C0(_, _, _, h2434) => {
                            h2434
                        }
                    };
                    let a2439 = {
                        let a2436 = match v0.clone() {
                            Adt6::C0(h2435, _, _, _) => {
                                h2435
                            }
                        };
                        let a2437 = 1u64;
                        nat_sub(a2436, a2437)
                    };
                    nat_mul(a2438, a2439)?
                };
                nat_add(a2440, a2441)?
            };
            let a2446 = {
                let a2443 = 2u64;
                let a2444 = f113(match v0.clone() {
                    Adt6::C0(_, _, h2442, _) => {
                        h2442
                    }
                });
                nat_mul(a2443, a2444)?
            };
            nat_add(a2445, a2446)?
        };
        let a2447 = v3;
        let a2448 = v2;
        if nat_le(a2447, a2448) {
            let a2455 = 1u64;
            let a2456 = {
                let a2452 = {
                    let a2449 = v2;
                    let a2450 = v3;
                    nat_sub(a2449, a2450)
                };
                let a2453 = match v0 {
                    Adt6::C0(_, h2451, _, _) => {
                        h2451
                    }
                };
                let a2454 = 0u64;
                nat_quot(a2452, a2453, a2454)
            };
            nat_add(a2455, a2456)
        } else {
            let a2460 = {
                let a2457 = v3;
                let a2458 = v2;
                nat_sub(a2457, a2458)
            };
            let a2461 = match v0 {
                Adt6::C0(_, h2459, _, _) => {
                    h2459
                }
            };
            if nat_lt(a2460, a2461) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f81(v0: u64) -> u64 {
    let v1: u64 = {
        let a2462 = v0;
        let a2463 = 4294967296u64;
        let a2464 = 0u64;
        nat_rem(a2462, a2463, a2464)
    };
    let a2465 = v1;
    let a2466 = 2147483648u64;
    if nat_lt(a2465, a2466) {
        v1
    } else {
        0u64
    }
}

pub fn f82(v0: u64) -> R<u64> {
    let a2469 = 2u64;
    let a2470 = {
        let a2467 = 2u64;
        let a2468 = v0;
        nat_mul(a2467, a2468)?
    };
    nat_add(a2469, a2470)
}

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f149(v0, v3, v6) {
        f149(v1, v4, v6)
    } else {
        false
    } {
        f149(v2, v5, v6)
    } else {
        false
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f150(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f150(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f150(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f150(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f150(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f150(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2471 = v2;
    let a2472 = 0u64;
    if nat_eq(a2471, a2472) {
        Ok(0u64)
    } else {
        let a2475 = 2u64;
        let a2476 = {
            let a2473 = v0;
            let a2474 = v1;
            nat_mul(a2473, a2474)?
        };
        nat_mul(a2475, a2476)
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2477 = v2;
    let a2478 = f145(v0, v1)?;
    nat_mul(a2477, a2478)
}

pub fn f87(v0: u64) -> u64 {
    let a2479 = v0;
    let a2480 = 30u64;
    if nat_eq(a2479, a2480) {
        3u64
    } else {
        1u64
    }
}

pub fn f88(v0: u64) -> R<u64> {
    let a2483 = 5u64;
    let a2484 = {
        let a2481 = 4u64;
        let a2482 = v0;
        nat_mul(a2481, a2482)?
    };
    nat_add(a2483, a2484)
}

pub fn f89(v0: u64) -> bool {
    let a2485 = v0;
    let a2486 = 0u64;
    if if nat_eq(a2485, a2486) {
        true
    } else {
        let a2487 = v0;
        let a2488 = 1u64;
        nat_eq(a2487, a2488)
    } {
        true
    } else {
        let a2489 = v0;
        let a2490 = 30u64;
        nat_eq(a2489, a2490)
    }
}

pub fn f90(v0: Adt2, v1: u64) -> R<bool> {
    let a2492 = match v0.clone() {
        Adt2::C0(_, _, _, h2491, _, _, _, _, _, _, _) => {
            h2491
        }
    };
    let a2493 = 1u64;
    if if if nat_eq(a2492, a2493) {
        let a2495 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2494, _, _, _, _, _, _) => {
                h2494
            }
        };
        let a2496 = 1u64;
        nat_eq(a2495, a2496)
    } else {
        false
    } {
        let a2498 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2497, _, _, _, _, _) => {
                h2497
            }
        };
        let a2499 = v1;
        nat_eq(a2498, a2499)
    } else {
        false
    } {
        let a2501 = match v0.clone() {
            Adt2::C0(_, _, h2500, _, _, _, _, _, _, _, _) => {
                h2500
            }
        };
        let a2502 = 1u64;
        if nat_eq(a2501, a2502) {
            Ok(true)
        } else {
            let a2507 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2503, _, _, _, _) => {
                    h2503
                }
            };
            let a2508 = {
                let a2505 = match v0 {
                    Adt2::C0(_, h2504, _, _, _, _, _, _, _, _, _) => {
                        h2504
                    }
                };
                let a2506 = v1;
                nat_mul(a2505, a2506)?
            };
            Ok(nat_eq(a2507, a2508))
        }
    } else {
        Ok(false)
    }
}

pub fn f91(v0: Adt6) -> R<u64> {
    let a2521 = 2u64;
    let a2522 = {
        let a2519 = 2u64;
        let a2520 = {
            let a2516 = {
                let a2513 = match v0.clone() {
                    Adt6::C0(h2509, _, _, _) => {
                        h2509
                    }
                };
                let a2514 = {
                    let a2511 = match v0.clone() {
                        Adt6::C0(_, h2510, _, _) => {
                            h2510
                        }
                    };
                    let a2512 = 1u64;
                    nat_sub(a2511, a2512)
                };
                nat_add(a2513, a2514)?
            };
            let a2517 = match v0 {
                Adt6::C0(_, h2515, _, _) => {
                    h2515
                }
            };
            let a2518 = 0u64;
            nat_quot(a2516, a2517, a2518)
        };
        nat_mul(a2519, a2520)?
    };
    nat_add(a2521, a2522)
}

pub fn f92(v0: u64) -> R<u64> {
    let a2525 = 2u64;
    let a2526 = {
        let a2523 = v0;
        let a2524 = 2u64;
        nat_mul(a2523, a2524)?
    };
    nat_add(a2525, a2526)
}

pub fn f93(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2530 = {
            let a2527 = v2;
            let a2528 = 1u64;
            nat_sub(a2527, a2528)
        };
        let a2531 = match v0.clone() {
            Adt6::C0(_, h2529, _, _) => {
                h2529
            }
        };
        nat_mul(a2530, a2531)?
    };
    let a2533 = 0u64;
    let a2534 = match v0.clone() {
        Adt6::C0(h2532, _, _, _) => {
            h2532
        }
    };
    if if if if if if if if nat_lt(a2533, a2534) {
        let a2536 = match v0.clone() {
            Adt6::C0(h2535, _, _, _) => {
                h2535
            }
        };
        let a2537 = 2147483648u64;
        nat_lt(a2536, a2537)
    } else {
        false
    } {
        let a2539 = 0u64;
        let a2540 = match v0.clone() {
            Adt6::C0(_, h2538, _, _) => {
                h2538
            }
        };
        nat_lt(a2539, a2540)
    } else {
        false
    } {
        let a2542 = match v0.clone() {
            Adt6::C0(_, h2541, _, _) => {
                h2541
            }
        };
        let a2543 = 2147483648u64;
        nat_lt(a2542, a2543)
    } else {
        false
    } {
        let a2544 = v1;
        let a2545 = 2147483648u64;
        nat_lt(a2544, a2545)
    } else {
        false
    } {
        let a2546 = 0u64;
        let a2547 = v2;
        nat_lt(a2546, a2547)
    } else {
        false
    } {
        let a2548 = v3;
        let a2549 = 2147483648u64;
        nat_lt(a2548, a2549)
    } else {
        false
    } {
        let a2561 = {
            let a2556 = {
                let a2553 = v3;
                let a2554 = {
                    let a2551 = match v0.clone() {
                        Adt6::C0(h2550, _, _, _) => {
                            h2550
                        }
                    };
                    let a2552 = 1u64;
                    nat_sub(a2551, a2552)
                };
                nat_add(a2553, a2554)?
            };
            let a2557 = f113(match v0.clone() {
                Adt6::C0(_, _, h2555, _) => {
                    h2555
                }
            });
            nat_add(a2556, a2557)?
        };
        let a2562 = {
            let a2559 = 2147483647u64;
            let a2560 = f114(match v0.clone() {
                Adt6::C0(_, _, h2558, _) => {
                    h2558
                }
            });
            nat_add(a2559, a2560)?
        };
        nat_le(a2561, a2562)
    } else {
        false
    } {
        let a2564 = match v0 {
            Adt6::C0(_, _, h2563, _) => {
                h2563
            }
        };
        let a2565 = 2147483648u64;
        Ok(!nat_eq(a2564, a2565))
    } else {
        Ok(false)
    }
}

pub fn f94(v0: Adt7) -> R<u64> {
    let a2577 = if match v0.clone() {
        Adt7::C0(_, _, _, h2566, _) => {
            h2566
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2578 = {
        let a2575 = {
            let a2573 = {
                let m2569 = match v0.clone() {
                    Adt7::C0(h2568, _, _, _, _) => {
                        h2568
                    }
                };
                match m2569 {
                    Adt6::C0(h2567, _, _, _) => {
                        h2567
                    }
                }
            };
            let a2574 = {
                let m2572 = match v0.clone() {
                    Adt7::C0(_, h2571, _, _, _) => {
                        h2571
                    }
                };
                match m2572 {
                    Adt6::C0(h2570, _, _, _) => {
                        h2570
                    }
                }
            };
            nat_mul(a2573, a2574)?
        };
        let a2576 = f141(v0)?;
        nat_mul(a2575, a2576)?
    };
    nat_add(a2577, a2578)
}

pub fn f95(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f151(match v0.clone() {
        Adt2::C0(h2579, _, _, _, _, _, _, _, _, _, _) => {
            h2579
        }
    }, match v0.clone() {
        Adt2::C0(_, h2580, _, _, _, _, _, _, _, _, _) => {
            h2580
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2581, _, _, _, _, _, _, _, _) => {
            h2581
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2582, _, _, _, _, _, _, _) => {
            h2582
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2583, _, _, _, _, _, _) => {
            h2583
        }
    }, match v1.clone() {
        Adt2::C0(h2584, _, _, _, _, _, _, _, _, _, _) => {
            h2584
        }
    }, match v1.clone() {
        Adt2::C0(_, h2585, _, _, _, _, _, _, _, _, _) => {
            h2585
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2586, _, _, _, _, _, _, _, _) => {
            h2586
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2587, _, _, _, _, _, _, _) => {
            h2587
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2588, _, _, _, _, _, _) => {
            h2588
        }
    }, match v2.clone() {
        Adt2::C0(h2589, _, _, _, _, _, _, _, _, _, _) => {
            h2589
        }
    }, match v2.clone() {
        Adt2::C0(_, h2590, _, _, _, _, _, _, _, _, _) => {
            h2590
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2591, _, _, _, _, _, _, _, _) => {
            h2591
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2592, _, _, _, _, _, _, _) => {
            h2592
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2593, _, _, _, _, _, _) => {
            h2593
        }
    }) {
        let m2595 = f98(match v0.clone() {
            Adt2::C0(h2594, _, _, _, _, _, _, _, _, _, _) => {
                h2594
            }
        });
        match m2595 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                let a2601 = {
                    let a2598 = match v0.clone() {
                        Adt2::C0(_, h2596, _, _, _, _, _, _, _, _, _) => {
                            h2596
                        }
                    };
                    let a2599 = match v3.clone() {
                        Adt15::C0(_, h2597, _) => {
                            h2597
                        }
                    };
                    let a2600 = 0u64;
                    nat_rem(a2598, a2599, a2600)
                };
                let a2602 = 0u64;
                if if nat_eq(a2601, a2602) {
                    if f152(match v0.clone() {
                        Adt2::C0(_, h2603, _, _, _, _, _, _, _, _, _) => {
                            h2603
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2604, _, _, _, _, _, _, _, _, _) => {
                            h2604
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2605, _, _, _, _, _, _, _, _) => {
                            h2605
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2606, _, _, _, _, _, _, _) => {
                            h2606
                        }
                    }) {
                        true
                    } else {
                        let a2608 = 4u64;
                        let a2609 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2607, _, _, _, _, _) => {
                                h2607
                            }
                        };
                        if if nat_le(a2608, a2609) {
                            let a2611 = match v3.clone() {
                                Adt15::C0(_, h2610, _) => {
                                    h2610
                                }
                            };
                            let a2612 = 1u64;
                            if nat_eq(a2611, a2612) {
                                true
                            } else {
                                let a2615 = match v3.clone() {
                                    Adt15::C0(_, _, h2613) => {
                                        h2613
                                    }
                                };
                                let a2616 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2614, _, _, _, _, _) => {
                                        h2614
                                    }
                                };
                                nat_le(a2615, a2616)
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
                        Adt15::C0(_, h2617, _) => {
                            h2617
                        }
                    });
                    Ok(Adt10::C4(if f152(match v0.clone() {
                        Adt2::C0(_, h2618, _, _, _, _, _, _, _, _, _) => {
                            h2618
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2619, _, _, _, _, _, _, _, _, _) => {
                            h2619
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2620, _, _, _, _, _, _, _, _) => {
                            h2620
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2621, _, _, _, _, _, _, _) => {
                            h2621
                        }
                    }) {
                        0u64
                    } else {
                        f135(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2622, _, _, _, _, _, _, _, _, _, _) => {
                            h2622
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

pub fn f96(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f89(match v0.clone() {
        Adt2::C0(h2623, _, _, _, _, _, _, _, _, _, _) => {
            h2623
        }
    }) {
        let a2625 = match v1.clone() {
            Adt2::C0(h2624, _, _, _, _, _, _, _, _, _, _) => {
                h2624
            }
        };
        let a2626 = 26u64;
        nat_eq(a2625, a2626)
    } else {
        false
    } {
        let a2628 = match v2.clone() {
            Adt2::C0(h2627, _, _, _, _, _, _, _, _, _, _) => {
                h2627
            }
        };
        let a2629 = 0u64;
        nat_eq(a2628, a2629)
    } else {
        false
    } {
        let a2632 = match v0.clone() {
            Adt2::C0(_, h2630, _, _, _, _, _, _, _, _, _) => {
                h2630
            }
        };
        let a2633 = match v2.clone() {
            Adt2::C0(_, h2631, _, _, _, _, _, _, _, _, _) => {
                h2631
            }
        };
        nat_eq(a2632, a2633)
    } else {
        false
    } {
        let a2636 = match v1.clone() {
            Adt2::C0(_, h2634, _, _, _, _, _, _, _, _, _) => {
                h2634
            }
        };
        let a2637 = match v0.clone() {
            Adt2::C0(_, _, h2635, _, _, _, _, _, _, _, _) => {
                h2635
            }
        };
        nat_le(a2636, a2637)
    } else {
        false
    } {
        let a2639 = match v0.clone() {
            Adt2::C0(_, _, _, h2638, _, _, _, _, _, _, _) => {
                h2638
            }
        };
        let a2640 = 1u64;
        nat_eq(a2639, a2640)
    } else {
        false
    } {
        let a2642 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2641, _, _, _, _, _, _) => {
                h2641
            }
        };
        let a2643 = 1u64;
        nat_eq(a2642, a2643)
    } else {
        false
    } {
        let a2645 = match v2.clone() {
            Adt2::C0(_, _, _, h2644, _, _, _, _, _, _, _) => {
                h2644
            }
        };
        let a2646 = 1u64;
        nat_eq(a2645, a2646)
    } else {
        false
    } {
        let a2648 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2647, _, _, _, _, _, _) => {
                h2647
            }
        };
        let a2649 = 1u64;
        nat_eq(a2648, a2649)
    } else {
        false
    } {
        let a2651 = match v1.clone() {
            Adt2::C0(_, _, h2650, _, _, _, _, _, _, _, _) => {
                h2650
            }
        };
        let a2652 = 1u64;
        nat_eq(a2651, a2652)
    } else {
        false
    } {
        let a2654 = match v1.clone() {
            Adt2::C0(_, _, _, h2653, _, _, _, _, _, _, _) => {
                h2653
            }
        };
        let a2655 = 1u64;
        nat_eq(a2654, a2655)
    } else {
        false
    } {
        let a2657 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2656, _, _, _, _, _, _) => {
                h2656
            }
        };
        let a2658 = 1u64;
        nat_eq(a2657, a2658)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2659, _, _, _, _, _, _, _, _, _) => {
                h2659
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2660, _, _, _, _, _, _, _, _) => {
                h2660
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2661, _, _, _, _, _, _, _) => {
                h2661
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2662, _, _, _, _, _, _) => {
                h2662
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2664 = match v1.clone() {
                Adt2::C0(_, h2663, _, _, _, _, _, _, _, _, _) => {
                    h2663
                }
            };
            let a2665 = 0u64;
            if nat_eq(a2664, a2665) {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2667 = match v0.clone() {
                    Adt2::C0(_, h2666, _, _, _, _, _, _, _, _, _) => {
                        h2666
                    }
                };
                let a2668 = 2147483647u64;
                if if if if if if nat_le(a2667, a2668) {
                    let a2670 = match v1.clone() {
                        Adt2::C0(_, h2669, _, _, _, _, _, _, _, _, _) => {
                            h2669
                        }
                    };
                    let a2671 = 2147483647u64;
                    nat_le(a2670, a2671)
                } else {
                    false
                } {
                    let a2673 = 4u64;
                    let a2674 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2672, _, _, _, _, _) => {
                            h2672
                        }
                    };
                    nat_le(a2673, a2674)
                } else {
                    false
                } {
                    let a2680 = {
                        let a2677 = match v2.clone() {
                            Adt2::C0(_, h2675, _, _, _, _, _, _, _, _, _) => {
                                h2675
                            }
                        };
                        let a2678 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2676, _, _, _, _, _) => {
                                h2676
                            }
                        };
                        nat_mul(a2677, a2678)?
                    };
                    let a2681 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2679, _, _, _, _) => {
                            h2679
                        }
                    };
                    nat_le(a2680, a2681)
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
                        let a2695 = {
                            let a2685 = {
                                let a2683 = 2u64;
                                let a2684 = match v1.clone() {
                                    Adt2::C0(_, h2682, _, _, _, _, _, _, _, _, _) => {
                                        h2682
                                    }
                                };
                                nat_mul(a2683, a2684)?
                            };
                            let a2686 = f12(v2.clone(), 1u64)?;
                            nat_add(a2685, a2686)?
                        };
                        let a2696 = {
                            let a2693 = match v1.clone() {
                                Adt2::C0(_, h2687, _, _, _, _, _, _, _, _, _) => {
                                    h2687
                                }
                            };
                            let a2694 = {
                                let a2691 = 1u64;
                                let a2692 = {
                                    let a2689 = 2u64;
                                    let a2690 = match v0.clone() {
                                        Adt2::C0(_, h2688, _, _, _, _, _, _, _, _, _) => {
                                            h2688
                                        }
                                    };
                                    nat_mul(a2689, a2690)?
                                };
                                nat_add(a2691, a2692)?
                            };
                            nat_mul(a2693, a2694)?
                        };
                        nat_add(a2695, a2696)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2697, _, _, _, _, _, _, _, _, _) => {
                            h2697
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2698, _, _, _, _, _, _, _, _, _) => {
                            h2698
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2699, _, _, _, _, _) => {
                            h2699
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2700, _, _, _, _) => {
                            h2700
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2701, _, _, _) => {
                            h2701
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2702, _, _) => {
                            h2702
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2703, _, _, _, _, _, _, _, _, _, _) => {
                            h2703
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

pub fn f97(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2706 = match v1.clone() {
        Adt2::C0(h2704, _, _, _, _, _, _, _, _, _, _) => {
            h2704
        }
    };
    let a2707 = match v3.clone() {
        Adt2::C0(h2705, _, _, _, _, _, _, _, _, _, _) => {
            h2705
        }
    };
    if if nat_eq(a2706, a2707) {
        let a2708 = v0;
        let a2709 = 33u64;
        if nat_eq(a2708, a2709) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2711 = f98(match v1.clone() {
            Adt2::C0(h2710, _, _, _, _, _, _, _, _, _, _) => {
                h2710
            }
        });
        match m2711 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt15::C0(_, h2712, _) => {
                        h2712
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2717 = {
                    let a2714 = match v1.clone() {
                        Adt2::C0(_, h2713, _, _, _, _, _, _, _, _, _) => {
                            h2713
                        }
                    };
                    let a2715 = v5;
                    let a2716 = 1u64;
                    nat_rem(a2714, a2715, a2716)
                };
                let a2718 = 0u64;
                if if if nat_eq(a2717, a2718) {
                    let a2723 = {
                        let a2720 = match v3.clone() {
                            Adt2::C0(_, h2719, _, _, _, _, _, _, _, _, _) => {
                                h2719
                            }
                        };
                        let a2721 = v5;
                        let a2722 = 1u64;
                        nat_rem(a2720, a2721, a2722)
                    };
                    let a2724 = 0u64;
                    nat_eq(a2723, a2724)
                } else {
                    false
                } {
                    let a2725 = v6;
                    let a2726 = f12(v3.clone(), v5)?;
                    nat_eq(a2725, a2726)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt15::C0(_, _, h2727) => {
                            h2727
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

pub fn f98(v0: u64) -> Adt14 {
    let a2728 = v0;
    let a2729 = 0u64;
    if nat_eq(a2728, a2729) {
        Adt14::C1(Adt15::C0(0u64, 1u64, 4u64))
    } else {
        let a2730 = v0;
        let a2731 = 1u64;
        if nat_eq(a2730, a2731) {
            Adt14::C1(Adt15::C0(1u64, 1u64, 2u64))
        } else {
            let a2732 = v0;
            let a2733 = 2u64;
            if nat_eq(a2732, a2733) {
                Adt14::C1(Adt15::C0(2u64, 32u64, 18u64))
            } else {
                let a2734 = v0;
                let a2735 = 3u64;
                if nat_eq(a2734, a2735) {
                    Adt14::C1(Adt15::C0(3u64, 32u64, 20u64))
                } else {
                    let a2736 = v0;
                    let a2737 = 6u64;
                    if nat_eq(a2736, a2737) {
                        Adt14::C1(Adt15::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2738 = v0;
                        let a2739 = 7u64;
                        if nat_eq(a2738, a2739) {
                            Adt14::C1(Adt15::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2740 = v0;
                            let a2741 = 8u64;
                            if nat_eq(a2740, a2741) {
                                Adt14::C1(Adt15::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2742 = v0;
                                let a2743 = 9u64;
                                if nat_eq(a2742, a2743) {
                                    Adt14::C1(Adt15::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2744 = v0;
                                    let a2745 = 10u64;
                                    if nat_eq(a2744, a2745) {
                                        Adt14::C1(Adt15::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2746 = v0;
                                        let a2747 = 11u64;
                                        if nat_eq(a2746, a2747) {
                                            Adt14::C1(Adt15::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2748 = v0;
                                            let a2749 = 12u64;
                                            if nat_eq(a2748, a2749) {
                                                Adt14::C1(Adt15::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2750 = v0;
                                                let a2751 = 13u64;
                                                if nat_eq(a2750, a2751) {
                                                    Adt14::C1(Adt15::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2752 = v0;
                                                    let a2753 = 14u64;
                                                    if nat_eq(a2752, a2753) {
                                                        Adt14::C1(Adt15::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2754 = v0;
                                                        let a2755 = 15u64;
                                                        if nat_eq(a2754, a2755) {
                                                            Adt14::C1(Adt15::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2756 = v0;
                                                            let a2757 = 16u64;
                                                            if nat_eq(a2756, a2757) {
                                                                Adt14::C1(Adt15::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2758 = v0;
                                                                let a2759 = 17u64;
                                                                if nat_eq(a2758, a2759) {
                                                                    Adt14::C1(Adt15::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2760 = v0;
                                                                    let a2761 = 18u64;
                                                                    if nat_eq(a2760, a2761) {
                                                                        Adt14::C1(Adt15::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2762 = v0;
                                                                        let a2763 = 19u64;
                                                                        if nat_eq(a2762, a2763) {
                                                                            Adt14::C1(Adt15::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2764 = v0;
                                                                            let a2765 = 20u64;
                                                                            if nat_eq(a2764, a2765) {
                                                                                Adt14::C1(Adt15::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2766 = v0;
                                                                                let a2767 = 21u64;
                                                                                if nat_eq(a2766, a2767) {
                                                                                    Adt14::C1(Adt15::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2768 = v0;
                                                                                    let a2769 = 22u64;
                                                                                    if nat_eq(a2768, a2769) {
                                                                                        Adt14::C1(Adt15::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2770 = v0;
                                                                                        let a2771 = 23u64;
                                                                                        if nat_eq(a2770, a2771) {
                                                                                            Adt14::C1(Adt15::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2772 = v0;
                                                                                            let a2773 = 24u64;
                                                                                            if nat_eq(a2772, a2773) {
                                                                                                Adt14::C1(Adt15::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2774 = v0;
                                                                                                let a2775 = 25u64;
                                                                                                if nat_eq(a2774, a2775) {
                                                                                                    Adt14::C1(Adt15::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2776 = v0;
                                                                                                    let a2777 = 26u64;
                                                                                                    if nat_eq(a2776, a2777) {
                                                                                                        Adt14::C1(Adt15::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2778 = v0;
                                                                                                        let a2779 = 27u64;
                                                                                                        if nat_eq(a2778, a2779) {
                                                                                                            Adt14::C1(Adt15::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2780 = v0;
                                                                                                            let a2781 = 28u64;
                                                                                                            if nat_eq(a2780, a2781) {
                                                                                                                Adt14::C1(Adt15::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2782 = v0;
                                                                                                                let a2783 = 29u64;
                                                                                                                if nat_eq(a2782, a2783) {
                                                                                                                    Adt14::C1(Adt15::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2784 = v0;
                                                                                                                    let a2785 = 30u64;
                                                                                                                    if nat_eq(a2784, a2785) {
                                                                                                                        Adt14::C1(Adt15::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2786 = v0;
                                                                                                                        let a2787 = 34u64;
                                                                                                                        if nat_eq(a2786, a2787) {
                                                                                                                            Adt14::C1(Adt15::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2788 = v0;
                                                                                                                            let a2789 = 35u64;
                                                                                                                            if nat_eq(a2788, a2789) {
                                                                                                                                Adt14::C1(Adt15::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2790 = v0;
                                                                                                                                let a2791 = 39u64;
                                                                                                                                if nat_eq(a2790, a2791) {
                                                                                                                                    Adt14::C1(Adt15::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2792 = v0;
                                                                                                                                    let a2793 = 40u64;
                                                                                                                                    if nat_eq(a2792, a2793) {
                                                                                                                                        Adt14::C1(Adt15::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2794 = v0;
                                                                                                                                        let a2795 = 41u64;
                                                                                                                                        if nat_eq(a2794, a2795) {
                                                                                                                                            Adt14::C1(Adt15::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2796 = v0;
                                                                                                                                            let a2797 = 42u64;
                                                                                                                                            if nat_eq(a2796, a2797) {
                                                                                                                                                Adt14::C1(Adt15::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2798 = v0;
                                                                                                                                                let a2799 = 43u64;
                                                                                                                                                if nat_eq(a2798, a2799) {
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

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f27(v4, v5, v6, v7) {
        f27(v0, v1, v2, v3)
    } else {
        let a2803 = {
            let a2800 = v0;
            let a2801 = v4;
            let a2802 = 1u64;
            nat_rem(a2800, a2801, a2802)
        };
        let a2804 = 0u64;
        if if if nat_eq(a2803, a2804) {
            let a2808 = {
                let a2805 = v1;
                let a2806 = v5;
                let a2807 = 1u64;
                nat_rem(a2805, a2806, a2807)
            };
            let a2809 = 0u64;
            nat_eq(a2808, a2809)
        } else {
            false
        } {
            let a2813 = {
                let a2810 = v2;
                let a2811 = v6;
                let a2812 = 1u64;
                nat_rem(a2810, a2811, a2812)
            };
            let a2814 = 0u64;
            nat_eq(a2813, a2814)
        } else {
            false
        } {
            let a2818 = {
                let a2815 = v3;
                let a2816 = v7;
                let a2817 = 1u64;
                nat_rem(a2815, a2816, a2817)
            };
            let a2819 = 0u64;
            nat_eq(a2818, a2819)
        } else {
            false
        }
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2820 = v0;
    let a2821 = v1;
    if nat_eq(a2820, a2821) {
        let a2824 = v4;
        let a2825 = {
            let a2822 = v2;
            let a2823 = v3;
            nat_add(a2822, a2823)?
        };
        Ok(nat_eq(a2824, a2825))
    } else {
        let a2826 = v2;
        let a2827 = v3;
        if nat_eq(a2826, a2827) {
            let a2828 = v4;
            let a2829 = v2;
            Ok(nat_eq(a2828, a2829))
        } else {
            Ok(false)
        }
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2832 = {
            let a2830 = v0;
            let a2831 = f114(v1);
            nat_add(a2830, a2831)?
        };
        let a2833 = f114(v2);
        nat_add(a2832, a2833)?
    };
    let v5: u64 = {
        let a2834 = f113(v1);
        let a2835 = f113(v2);
        nat_add(a2834, a2835)?
    };
    let a2836 = v1;
    let a2837 = 4294967295u64;
    if if nat_le(a2836, a2837) {
        let a2838 = v2;
        let a2839 = 4294967295u64;
        nat_le(a2838, a2839)
    } else {
        false
    } {
        let a2840 = v5;
        let a2841 = v4;
        if nat_le(a2840, a2841) {
            let a2844 = v3;
            let a2845 = {
                let a2842 = v4;
                let a2843 = v5;
                nat_sub(a2842, a2843)
            };
            Ok(nat_eq(a2844, a2845))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f102(v0: u64, v1: u64) -> R<bool> {
    let a2846 = v1;
    let a2847 = 4294967295u64;
    if nat_le(a2846, a2847) {
        let a2850 = {
            let a2848 = f114(v1);
            let a2849 = f113(v1);
            nat_add(a2848, a2849)?
        };
        let a2851 = v0;
        Ok(nat_lt(a2850, a2851))
    } else {
        Ok(false)
    }
}

pub fn f103(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f153(v0.clone(), false)?;
    let v6: u64 = f153(v0, true)?;
    let a2852 = v6;
    let a2853 = v1;
    if if nat_le(a2852, a2853) {
        let a2860 = {
            let a2858 = {
                let a2854 = v1;
                let a2855 = v5;
                nat_add(a2854, a2855)?
            };
            let a2859 = {
                let a2856 = v3;
                let a2857 = 0u64;
                if nat_eq(a2856, a2857) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2858, a2859)?
        };
        let a2861 = v2;
        nat_le(a2860, a2861)
    } else {
        false
    } {
        if v4 {
            let a2866 = {
                let a2864 = {
                    let a2862 = v1;
                    let a2863 = v5;
                    nat_add(a2862, a2863)?
                };
                let a2865 = v6;
                nat_sub(a2864, a2865)
            };
            let a2867 = v2;
            Ok(nat_lt(a2866, a2867))
        } else {
            let a2872 = {
                let a2870 = {
                    let a2868 = v1;
                    let a2869 = v5;
                    nat_add(a2868, a2869)?
                };
                let a2871 = v6;
                nat_sub(a2870, a2871)
            };
            let a2873 = v2;
            Ok(nat_le(a2872, a2873))
        }
    } else {
        Ok(false)
    }
}

pub fn f104(v0: u64, v1: u64) -> bool {
    let a2874 = v0;
    let a2875 = 0u64;
    if if nat_eq(a2874, a2875) {
        true
    } else {
        let a2876 = v0;
        let a2877 = 1u64;
        nat_eq(a2876, a2877)
    } {
        let a2878 = v1;
        let a2879 = 4294967296u64;
        nat_lt(a2878, a2879)
    } else {
        false
    }
}

pub fn f105(v0: u64, v1: u64) -> R<u64> {
    let a2884 = {
        let a2882 = v0;
        let a2883 = {
            let a2880 = v1;
            let a2881 = 1u64;
            nat_sub(a2880, a2881)
        };
        nat_add(a2882, a2883)?
    };
    let a2885 = v1;
    let a2886 = 0u64;
    Ok(nat_quot(a2884, a2885, a2886))
}

pub fn f106(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2887, _, _, _, _, _, _, _, _, _) => {
            h2887
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2888, _, _, _, _, _, _, _, _) => {
            h2888
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2889, _, _, _, _, _, _, _) => {
            h2889
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2890, _, _, _, _, _, _) => {
            h2890
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2893 = match v1.clone() {
                Adt2::C0(_, h2891, _, _, _, _, _, _, _, _, _) => {
                    h2891
                }
            };
            let a2894 = match v1.clone() {
                Adt2::C0(_, _, h2892, _, _, _, _, _, _, _, _) => {
                    h2892
                }
            };
            nat_mul(a2893, a2894)?
        };
        let v5: u64 = {
            let a2896 = v4;
            let a2897 = match v1.clone() {
                Adt2::C0(_, _, _, h2895, _, _, _, _, _, _, _) => {
                    h2895
                }
            };
            nat_mul(a2896, a2897)?
        };
        let a2910 = {
            let a2908 = {
                let a2899 = match v0.clone() {
                    Adt2::C0(_, h2898, _, _, _, _, _, _, _, _, _) => {
                        h2898
                    }
                };
                let a2900 = 1u64;
                nat_sub(a2899, a2900)
            };
            let a2909 = {
                let a2906 = f154(match v0.clone() {
                    Adt2::C0(_, _, h2901, _, _, _, _, _, _, _, _) => {
                        h2901
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2902, _, _, _, _, _, _, _, _, _) => {
                        h2902
                    }
                })?;
                let a2907 = f154(match v0 {
                    Adt2::C0(_, _, _, h2903, _, _, _, _, _, _, _) => {
                        h2903
                    }
                }, v2, {
                    let a2904 = v3;
                    let a2905 = v5;
                    nat_mul(a2904, a2905)?
                }, v4)?;
                nat_add(a2906, a2907)?
            };
            nat_add(a2908, a2909)?
        };
        let a2911 = f12(v1, 1u64)?;
        Ok(nat_lt(a2910, a2911))
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2912 = v0;
    let a2913 = 2147483648u64;
    if if if if nat_lt(a2912, a2913) {
        let a2914 = v1;
        let a2915 = 2147483648u64;
        nat_lt(a2914, a2915)
    } else {
        false
    } {
        let a2918 = {
            let a2916 = v0;
            let a2917 = v1;
            nat_mul(a2916, a2917)?
        };
        let a2919 = 2147483648u64;
        nat_lt(a2918, a2919)
    } else {
        false
    } {
        let a2924 = {
            let a2922 = {
                let a2920 = v0;
                let a2921 = 1u64;
                nat_sub(a2920, a2921)
            };
            let a2923 = v2;
            nat_mul(a2922, a2923)?
        };
        let a2925 = 2147483648u64;
        nat_lt(a2924, a2925)
    } else {
        false
    } {
        let a2930 = {
            let a2928 = {
                let a2926 = v1;
                let a2927 = 1u64;
                nat_sub(a2926, a2927)
            };
            let a2929 = v2;
            nat_mul(a2928, a2929)?
        };
        let a2931 = 2147483648u64;
        Ok(nat_lt(a2930, a2931))
    } else {
        Ok(false)
    }
}

pub fn f108(v0: u64) -> R<u64> {
    let a2934 = 3u64;
    let a2935 = {
        let a2932 = v0;
        let a2933 = 3u64;
        nat_mul(a2932, a2933)?
    };
    nat_add(a2934, a2935)
}

pub fn f109(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
        Adt3::C0(h2936, _, _, _, _, _, _, _) => {
            h2936
        }
    }, match v0.clone() {
        Adt3::C0(_, h2937, _, _, _, _, _, _) => {
            h2937
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2938, _, _, _, _, _) => {
            h2938
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2939, _, _, _, _) => {
            h2939
        }
    }) {
        Ok(0u64)
    } else {
        let a2948 = match v0.clone() {
            Adt3::C0(h2940, _, _, _, _, _, _, _) => {
                h2940
            }
        };
        let a2949 = {
            let a2946 = match v0.clone() {
                Adt3::C0(_, h2941, _, _, _, _, _, _) => {
                    h2941
                }
            };
            let a2947 = {
                let a2944 = match v0.clone() {
                    Adt3::C0(_, _, h2942, _, _, _, _, _) => {
                        h2942
                    }
                };
                let a2945 = match v0 {
                    Adt3::C0(_, _, _, h2943, _, _, _, _) => {
                        h2943
                    }
                };
                nat_mul(a2944, a2945)?
            };
            nat_mul(a2946, a2947)?
        };
        nat_mul(a2948, a2949)
    }
}

pub fn f110(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2950 = v1;
    let a2951 = 13u64;
    if if nat_eq(a2950, a2951) {
        f27(match v0.clone() {
            Adt3::C0(h2952, _, _, _, _, _, _, _) => {
                h2952
            }
        }, match v0.clone() {
            Adt3::C0(_, h2953, _, _, _, _, _, _) => {
                h2953
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2954, _, _, _, _, _) => {
                h2954
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2955, _, _, _, _) => {
                h2955
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2961 = match v0.clone() {
                Adt3::C0(_, h2956, _, _, _, _, _, _) => {
                    h2956
                }
            };
            let a2962 = {
                let a2959 = match v0.clone() {
                    Adt3::C0(_, _, h2957, _, _, _, _, _) => {
                        h2957
                    }
                };
                let a2960 = match v0.clone() {
                    Adt3::C0(_, _, _, h2958, _, _, _, _) => {
                        h2958
                    }
                };
                nat_mul(a2959, a2960)?
            };
            nat_mul(a2961, a2962)?
        };
        let a2963 = v1;
        let a2964 = 13u64;
        if nat_eq(a2963, a2964) {
            let a2973 = 2u64;
            let a2974 = if f155(v0.clone(), v2) {
                let a2966 = v3;
                let a2967 = f156(match v0 {
                    Adt3::C0(h2965, _, _, _, _, _, _, _) => {
                        h2965
                    }
                }, false)?;
                nat_mul(a2966, a2967)?
            } else {
                let a2971 = v3;
                let a2972 = {
                    let a2969 = match v0 {
                        Adt3::C0(h2968, _, _, _, _, _, _, _) => {
                            h2968
                        }
                    };
                    let a2970 = 2u64;
                    nat_mul(a2969, a2970)?
                };
                nat_mul(a2971, a2972)?
            };
            nat_add(a2973, a2974)
        } else {
            let a2978 = v3;
            let a2979 = f156(match v0 {
                Adt3::C0(h2975, _, _, _, _, _, _, _) => {
                    h2975
                }
            }, {
                let a2976 = v1;
                let a2977 = 16u64;
                nat_eq(a2976, a2977)
            })?;
            nat_mul(a2978, a2979)
        }
    }
}

pub fn f111(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a2980 = v0;
    let a2981 = 30u64;
    if if nat_eq(a2980, a2981) {
        let a2984 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2982, _, _, _, _) => {
                h2982
            }
        };
        let a2985 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2983, _, _, _, _, _) => {
                h2983
            }
        };
        nat_lt(a2984, a2985)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m2986 = v3;
        match m2986 {
            None => {
                f157(v0, v1, match v2.clone() {
                    Adt2::C0(h2987, _, _, _, _, _, _, _, _, _, _) => {
                        h2987
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2988, _, _, _, _, _, _, _, _, _, _) => {
                        h2988
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2989, _, _, _, _, _, _, _, _, _) => {
                        h2989
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2990, _, _, _, _, _, _, _, _) => {
                        h2990
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2991, _, _, _, _, _, _, _) => {
                        h2991
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2992, _, _, _, _, _, _) => {
                        h2992
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2993, _, _, _, _, _, _, _, _, _) => {
                        h2993
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2994, _, _, _, _, _, _, _, _) => {
                        h2994
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2995, _, _, _, _, _, _, _) => {
                        h2995
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2996, _, _, _, _, _, _) => {
                        h2996
                    }
                })
            }
            Some(v5) => {
                f157(v0, v1, match v2.clone() {
                    Adt2::C0(h2997, _, _, _, _, _, _, _, _, _, _) => {
                        h2997
                    }
                }, match v5.clone() {
                    Adt2::C0(h2998, _, _, _, _, _, _, _, _, _, _) => {
                        h2998
                    }
                }, match v4.clone() {
                    Adt2::C0(h2999, _, _, _, _, _, _, _, _, _, _) => {
                        h2999
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3000, _, _, _, _, _, _, _, _, _) => {
                        h3000
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3001, _, _, _, _, _, _, _, _) => {
                        h3001
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3002, _, _, _, _, _, _, _) => {
                        h3002
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3003, _, _, _, _, _, _) => {
                        h3003
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3004, _, _, _, _, _, _, _, _, _) => {
                        h3004
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3005, _, _, _, _, _, _, _, _) => {
                        h3005
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3006, _, _, _, _, _, _, _) => {
                        h3006
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3007, _, _, _, _, _, _) => {
                        h3007
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3008, _, _, _, _, _, _, _, _, _) => {
                        h3008
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3009, _, _, _, _, _, _, _, _) => {
                        h3009
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3010, _, _, _, _, _, _, _) => {
                        h3010
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3011, _, _, _, _, _, _) => {
                        h3011
                    }
                })
            }
        }
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3024 = {
        let a3020 = {
            let a3016 = {
                let a3012 = v0;
                let a3013 = v4;
                nat_mul(a3012, a3013)?
            };
            let a3017 = {
                let a3014 = v1;
                let a3015 = v5;
                nat_mul(a3014, a3015)?
            };
            nat_add(a3016, a3017)?
        };
        let a3021 = {
            let a3018 = v2;
            let a3019 = v6;
            nat_mul(a3018, a3019)?
        };
        nat_add(a3020, a3021)?
    };
    let a3025 = {
        let a3022 = v3;
        let a3023 = v7;
        nat_mul(a3022, a3023)?
    };
    nat_add(a3024, a3025)
}

pub fn f113(v0: u64) -> u64 {
    let a3026 = v0;
    let a3027 = 2147483648u64;
    if nat_lt(a3026, a3027) {
        0u64
    } else {
        let a3028 = 4294967296u64;
        let a3029 = v0;
        nat_sub(a3028, a3029)
    }
}

pub fn f114(v0: u64) -> u64 {
    let a3030 = v0;
    let a3031 = 2147483648u64;
    if nat_lt(a3030, a3031) {
        v0
    } else {
        0u64
    }
}

pub fn f115(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3032 = v7;
        let a3033 = 6u64;
        if nat_eq(a3032, a3033) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f78(v3, v4), f78(v5, v6), f79(v7, v8), f158(v1), f158(v9))
}

pub fn f116(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3034 = v0;
    match m3034 {
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
            let a3035 = 13u64;
            let a3036 = v34;
            if nat_le(a3035, a3036) {
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
                Adt5::C0(_, _, _, _, _, h3037, _, _) => {
                    h3037
                }
            }, match v54 {
                Adt5::C0(_, _, _, _, _, _, _, h3038) => {
                    h3038
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

pub fn f117(v0: Adt4) -> u64 {
    let m3039 = v0;
    match m3039 {
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

pub fn f118(v0: Adt4, v1: u64) -> u64 {
    let m3040 = v0;
    match m3040 {
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
            let a3041 = v1;
            let a3042 = 6u64;
            if nat_eq(a3041, a3042) {
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
            let a3051 = v1;
            let a3052 = 6u64;
            if if if nat_eq(a3051, a3052) {
                true
            } else {
                let a3053 = v1;
                let a3054 = 7u64;
                nat_eq(a3053, a3054)
            } {
                let a3055 = v54;
                let a3056 = 30u64;
                nat_eq(a3055, a3056)
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
                Adt5::C0(_, _, _, _, _, _, _, h3050) => {
                    h3050
                }
            }
        }
        Adt4::C28(v49) => {
            let a3043 = v1;
            let a3044 = 6u64;
            if nat_eq(a3043, a3044) {
                match v49 {
                    Adt7::C0(_, _, _, _, h3045) => {
                        h3045
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
            let a3046 = v1;
            let a3047 = 6u64;
            if if nat_eq(a3046, a3047) {
                true
            } else {
                let a3048 = v1;
                let a3049 = 7u64;
                nat_eq(a3048, a3049)
            } {
                v52
            } else {
                0u64
            }
        }
    }
}

pub fn f119(v0: Adt4, v1: u64) -> Adt11 {
    let a3057 = v1;
    let a3058 = 4u64;
    if nat_lt(a3057, a3058) {
        Adt11::C0(v1)
    } else {
        let a3059 = v1;
        let a3060 = 4u64;
        if nat_eq(a3059, a3060) {
            Adt11::C1(0u64)
        } else {
            let a3061 = v1;
            let a3062 = 5u64;
            if nat_eq(a3061, a3062) {
                Adt11::C1(3u64)
            } else {
                let a3063 = v1;
                let a3064 = 6u64;
                if nat_eq(a3063, a3064) {
                    Adt11::C2(f118(v0, v1), 0u64, false)
                } else {
                    let a3065 = v1;
                    let a3066 = 7u64;
                    if nat_eq(a3065, a3066) {
                        Adt11::C2(0u64, f118(v0, v1), false)
                    } else {
                        let a3067 = v1;
                        let a3068 = 16u64;
                        if nat_lt(a3067, a3068) {
                            Adt11::C1({
                                let a3069 = v1;
                                let a3070 = 8u64;
                                nat_sub(a3069, a3070)
                            })
                        } else {
                            let a3071 = v1;
                            let a3072 = 20u64;
                            if nat_lt(a3071, a3072) {
                                Adt11::C5({
                                    let a3073 = v1;
                                    let a3074 = 16u64;
                                    nat_sub(a3073, a3074)
                                })
                            } else {
                                let a3075 = v1;
                                let a3076 = 20u64;
                                if nat_eq(a3075, a3076) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3077 = v1;
                                    let a3078 = 21u64;
                                    if nat_eq(a3077, a3078) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3079 = v1;
                                        let a3080 = 22u64;
                                        if nat_eq(a3079, a3080) {
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

pub fn f120(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3082 = v0;
        let a3083 = match v1.clone() {
            Adt3::C0(h3081, _, _, _, _, _, _, _) => {
                h3081
            }
        };
        let a3084 = 0u64;
        nat_quot(a3082, a3083, a3084)
    };
    let v3: u64 = {
        let a3086 = v2;
        let a3087 = match v1.clone() {
            Adt3::C0(_, h3085, _, _, _, _, _, _) => {
                h3085
            }
        };
        let a3088 = 0u64;
        nat_quot(a3086, a3087, a3088)
    };
    f112({
        let a3090 = v0;
        let a3091 = match v1.clone() {
            Adt3::C0(h3089, _, _, _, _, _, _, _) => {
                h3089
            }
        };
        let a3092 = 0u64;
        nat_rem(a3090, a3091, a3092)
    }, {
        let a3094 = v2;
        let a3095 = match v1.clone() {
            Adt3::C0(_, h3093, _, _, _, _, _, _) => {
                h3093
            }
        };
        let a3096 = 0u64;
        nat_rem(a3094, a3095, a3096)
    }, {
        let a3098 = v3;
        let a3099 = match v1.clone() {
            Adt3::C0(_, _, h3097, _, _, _, _, _) => {
                h3097
            }
        };
        let a3100 = 0u64;
        nat_rem(a3098, a3099, a3100)
    }, {
        let a3102 = v3;
        let a3103 = match v1.clone() {
            Adt3::C0(_, _, h3101, _, _, _, _, _) => {
                h3101
            }
        };
        let a3104 = 0u64;
        nat_quot(a3102, a3103, a3104)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3105, _, _, _) => {
            h3105
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3106, _, _) => {
            h3106
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3107, _) => {
            h3107
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3108) => {
            h3108
        }
    })
}

pub fn f121(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3126 = {
        let a3121 = f159(v2, {
            let a3112 = {
                let a3109 = v0;
                let a3110 = v1;
                let a3111 = 0u64;
                nat_quot(a3109, a3110, a3111)
            };
            let a3113 = v2;
            let a3114 = 0u64;
            nat_rem(a3112, a3113, a3114)
        }, {
            let a3118 = {
                let a3115 = v0;
                let a3116 = v1;
                let a3117 = 0u64;
                nat_quot(a3115, a3116, a3117)
            };
            let a3119 = v2;
            let a3120 = 0u64;
            nat_quot(a3118, a3119, a3120)
        })?;
        let a3122 = v1;
        nat_mul(a3121, a3122)?
    };
    let a3127 = {
        let a3123 = v0;
        let a3124 = v1;
        let a3125 = 0u64;
        nat_rem(a3123, a3124, a3125)
    };
    nat_add(a3126, a3127)
}

pub fn f122(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3129 = v0;
        let a3130 = match v2.clone() {
            Adt3::C0(h3128, _, _, _, _, _, _, _) => {
                h3128
            }
        };
        let a3131 = 0u64;
        nat_quot(a3129, a3130, a3131)
    };
    let v4: u64 = {
        let a3133 = v3;
        let a3134 = match v2.clone() {
            Adt3::C0(_, h3132, _, _, _, _, _, _) => {
                h3132
            }
        };
        let a3135 = 0u64;
        nat_quot(a3133, a3134, a3135)
    };
    f160({
        let a3137 = v0;
        let a3138 = match v2.clone() {
            Adt3::C0(h3136, _, _, _, _, _, _, _) => {
                h3136
            }
        };
        let a3139 = 0u64;
        nat_rem(a3137, a3138, a3139)
    }, {
        let a3141 = v3;
        let a3142 = match v2.clone() {
            Adt3::C0(_, h3140, _, _, _, _, _, _) => {
                h3140
            }
        };
        let a3143 = 0u64;
        nat_rem(a3141, a3142, a3143)
    }, {
        let a3145 = v4;
        let a3146 = match v2.clone() {
            Adt3::C0(_, _, h3144, _, _, _, _, _) => {
                h3144
            }
        };
        let a3147 = 0u64;
        nat_rem(a3145, a3146, a3147)
    }, {
        let a3149 = v4;
        let a3150 = match v2 {
            Adt3::C0(_, _, h3148, _, _, _, _, _) => {
                h3148
            }
        };
        let a3151 = 0u64;
        nat_quot(a3149, a3150, a3151)
    }, match v1.clone() {
        Adt3::C0(h3152, _, _, _, _, _, _, _) => {
            h3152
        }
    }, match v1.clone() {
        Adt3::C0(_, h3153, _, _, _, _, _, _) => {
            h3153
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3154, _, _, _, _, _) => {
            h3154
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3155, _, _, _, _) => {
            h3155
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3156, _, _, _) => {
            h3156
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3157, _, _) => {
            h3157
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3158, _) => {
            h3158
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3159) => {
            h3159
        }
    })
}

pub fn f123(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3161 = v0;
        let a3162 = match v3.clone() {
            Adt3::C0(h3160, _, _, _, _, _, _, _) => {
                h3160
            }
        };
        let a3163 = 0u64;
        nat_quot(a3161, a3162, a3163)
    };
    let v6: u64 = {
        let a3165 = v5;
        let a3166 = match v3.clone() {
            Adt3::C0(_, h3164, _, _, _, _, _, _) => {
                h3164
            }
        };
        let a3167 = 0u64;
        nat_quot(a3165, a3166, a3167)
    };
    let v7: u64 = {
        let a3169 = v0;
        let a3170 = match v3.clone() {
            Adt3::C0(h3168, _, _, _, _, _, _, _) => {
                h3168
            }
        };
        let a3171 = 0u64;
        nat_rem(a3169, a3170, a3171)
    };
    let v8: u64 = {
        let a3173 = v5;
        let a3174 = match v3.clone() {
            Adt3::C0(_, h3172, _, _, _, _, _, _) => {
                h3172
            }
        };
        let a3175 = 0u64;
        nat_rem(a3173, a3174, a3175)
    };
    let v9: u64 = {
        let a3177 = v6;
        let a3178 = match v3.clone() {
            Adt3::C0(_, _, h3176, _, _, _, _, _) => {
                h3176
            }
        };
        let a3179 = 0u64;
        nat_rem(a3177, a3178, a3179)
    };
    let v10: u64 = {
        let a3181 = v6;
        let a3182 = match v3.clone() {
            Adt3::C0(_, _, h3180, _, _, _, _, _) => {
                h3180
            }
        };
        let a3183 = 0u64;
        nat_quot(a3181, a3182, a3183)
    };
    let v11: u64 = {
        let a3184 = v4;
        let a3185 = 0u64;
        if nat_eq(a3184, a3185) {
            match v1.clone() {
                Adt3::C0(h3186, _, _, _, _, _, _, _) => {
                    h3186
                }
            }
        } else {
            let a3187 = v4;
            let a3188 = 1u64;
            if nat_eq(a3187, a3188) {
                match v1.clone() {
                    Adt3::C0(_, h3189, _, _, _, _, _, _) => {
                        h3189
                    }
                }
            } else {
                let a3190 = v4;
                let a3191 = 2u64;
                if nat_eq(a3190, a3191) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3192, _, _, _, _, _) => {
                            h3192
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3193, _, _, _, _) => {
                            h3193
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3200 = v11;
        let a3201 = {
            let a3194 = v4;
            let a3195 = 0u64;
            if nat_eq(a3194, a3195) {
                v7
            } else {
                let a3196 = v4;
                let a3197 = 1u64;
                if nat_eq(a3196, a3197) {
                    v8
                } else {
                    let a3198 = v4;
                    let a3199 = 2u64;
                    if nat_eq(a3198, a3199) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3200, a3201)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f112(if if v12 {
        let a3202 = v4;
        let a3203 = 0u64;
        nat_eq(a3202, a3203)
    } else {
        false
    } {
        let a3204 = v7;
        let a3205 = v11;
        nat_sub(a3204, a3205)
    } else {
        v7
    }, if if v12 {
        let a3206 = v4;
        let a3207 = 1u64;
        nat_eq(a3206, a3207)
    } else {
        false
    } {
        let a3208 = v8;
        let a3209 = v11;
        nat_sub(a3208, a3209)
    } else {
        v8
    }, if if v12 {
        let a3210 = v4;
        let a3211 = 2u64;
        nat_eq(a3210, a3211)
    } else {
        false
    } {
        let a3212 = v9;
        let a3213 = v11;
        nat_sub(a3212, a3213)
    } else {
        v9
    }, if if v12 {
        let a3214 = v4;
        let a3215 = 3u64;
        nat_eq(a3214, a3215)
    } else {
        false
    } {
        let a3216 = v10;
        let a3217 = v11;
        nat_sub(a3216, a3217)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3218, _, _, _) => {
            h3218
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3219, _, _) => {
            h3219
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3220, _) => {
            h3220
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3221) => {
            h3221
        }
    })?, f120(v0, v3)?))
}

pub fn f124(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f61(v0, v2.clone(), 0u64);
    let v9: u64 = f61(v0, v2.clone(), 1u64);
    let v10: u64 = f61(v0, v2.clone(), 2u64);
    let v11: u64 = f61(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f60(v1.clone(), f161(v8, v3, match v1.clone() {
            Adt3::C0(h3222, _, _, _, _, _, _, _) => {
                h3222
            }
        })?, f161(v9, v4, match v1.clone() {
            Adt3::C0(_, h3223, _, _, _, _, _, _) => {
                h3223
            }
        })?, f161(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3224, _, _, _, _, _) => {
                h3224
            }
        })?, f161(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3225, _, _, _, _) => {
                h3225
            }
        })?)?, f120(v0, v2)?))
    } else {
        if if if if f162(v8, v3, match v1.clone() {
            Adt3::C0(h3226, _, _, _, _, _, _, _) => {
                h3226
            }
        })? {
            f162(v9, v4, match v1.clone() {
                Adt3::C0(_, h3227, _, _, _, _, _, _) => {
                    h3227
                }
            })?
        } else {
            false
        } {
            f162(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3228, _, _, _, _, _) => {
                    h3228
                }
            })?
        } else {
            false
        } {
            f162(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3229, _, _, _, _) => {
                    h3229
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f60(v1, f163(v8, v3)?, f163(v9, v4)?, f163(v10, v5)?, f163(v11, v6)?)?, f120(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f120(v0, v2)?))
        }
    }
}

pub fn f125(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f61(v0, v2.clone(), 0u64);
    let v5: u64 = f61(v0, v2.clone(), 1u64);
    let v6: u64 = f61(v0, v2.clone(), 2u64);
    let v7: u64 = f61(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f60(v1.clone(), f164(v4, v3, match v1 {
        Adt3::C0(h3230, _, _, _, _, _, _, _) => {
            h3230
        }
    })?, v5, v6, v7)?, f120(v0, v2)?))
}

pub fn f126(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3231 = v0;
    let a3232 = v3;
    if nat_lt(a3231, a3232) {
        Ok(Adt9::C1(false, {
            let a3233 = v0;
            let a3234 = 4u64;
            nat_mul(a3233, a3234)?
        }, {
            let a3235 = v0;
            let a3236 = 4u64;
            nat_mul(a3235, a3236)?
        }))
    } else {
        let v6: u64 = {
            let a3237 = v0;
            let a3238 = v3;
            nat_sub(a3237, a3238)
        };
        let v7: u64 = f120(v6, v1)?;
        let v8: u64 = f165(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f127(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3239 = v0;
    let a3240 = v3;
    if nat_lt(a3239, a3240) {
        Ok(Adt9::C1(true, 0u64, f120(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3241 = v0;
            let a3242 = v3;
            nat_sub(a3241, a3242)
        };
        let v5: u64 = f122(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f120(v4, v1)?, 3u64, v5))
    }
}

pub fn f128(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3244 = v1;
        let a3245 = match v3.clone() {
            Adt3::C0(h3243, _, _, _, _, _, _, _) => {
                h3243
            }
        };
        let a3246 = 0u64;
        nat_rem(a3244, a3245, a3246)
    };
    let v5: u64 = {
        let a3252 = {
            let a3248 = v1;
            let a3249 = match v3.clone() {
                Adt3::C0(h3247, _, _, _, _, _, _, _) => {
                    h3247
                }
            };
            let a3250 = 0u64;
            nat_quot(a3248, a3249, a3250)
        };
        let a3253 = match v3.clone() {
            Adt3::C0(_, h3251, _, _, _, _, _, _) => {
                h3251
            }
        };
        let a3254 = 0u64;
        nat_rem(a3252, a3253, a3254)
    };
    let v6: bool = f166(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f122(v1, v2, v3.clone())?
    } else {
        0u64
    }, f120(v1, v3)?))
}

pub fn f129(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3255 = v3;
        let a3256 = 8u64;
        nat_add(a3255, a3256)?
    }, 0u64, f120(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f122(v0, v5, v2.clone())?
    }, 3u64, f120(v0, v2)?))
}

pub fn f130(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3258 = v0;
        let a3259 = match v2.clone() {
            Adt3::C0(h3257, _, _, _, _, _, _, _) => {
                h3257
            }
        };
        let a3260 = 0u64;
        nat_rem(a3258, a3259, a3260)
    };
    let v7: u64 = {
        let a3262 = v0;
        let a3263 = match v2.clone() {
            Adt3::C0(h3261, _, _, _, _, _, _, _) => {
                h3261
            }
        };
        let a3264 = 0u64;
        nat_quot(a3262, a3263, a3264)
    };
    let v8: u64 = {
        let a3266 = v7;
        let a3267 = match v2.clone() {
            Adt3::C0(_, h3265, _, _, _, _, _, _) => {
                h3265
            }
        };
        let a3268 = 0u64;
        nat_rem(a3266, a3267, a3268)
    };
    let v9: u64 = {
        let a3270 = v7;
        let a3271 = match v2.clone() {
            Adt3::C0(_, h3269, _, _, _, _, _, _) => {
                h3269
            }
        };
        let a3272 = 0u64;
        nat_quot(a3270, a3271, a3272)
    };
    let v10: u64 = {
        let a3274 = v9;
        let a3275 = match v2.clone() {
            Adt3::C0(_, _, h3273, _, _, _, _, _) => {
                h3273
            }
        };
        let a3276 = 0u64;
        nat_rem(a3274, a3275, a3276)
    };
    let v11: u64 = {
        let a3278 = v9;
        let a3279 = match v2.clone() {
            Adt3::C0(_, _, h3277, _, _, _, _, _) => {
                h3277
            }
        };
        let a3280 = 0u64;
        nat_quot(a3278, a3279, a3280)
    };
    let v12: u64 = f120(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3289 = {
                let a3284 = {
                    let a3281 = v10;
                    let a3282 = v4;
                    let a3283 = 0u64;
                    nat_quot(a3281, a3282, a3283)
                };
                let a3285 = v5;
                nat_mul(a3284, a3285)?
            };
            let a3290 = {
                let a3286 = v8;
                let a3287 = v4;
                let a3288 = 0u64;
                nat_quot(a3286, a3287, a3288)
            };
            nat_add(a3289, a3290)?
        };
        let v14: u64 = {
            let a3310 = v6;
            let a3311 = {
                let a3308 = match v1.clone() {
                    Adt3::C0(h3291, _, _, _, _, _, _, _) => {
                        h3291
                    }
                };
                let a3309 = {
                    let a3306 = {
                        let a3292 = v8;
                        let a3293 = v4;
                        let a3294 = 0u64;
                        nat_rem(a3292, a3293, a3294)
                    };
                    let a3307 = {
                        let a3304 = match v1.clone() {
                            Adt3::C0(_, h3295, _, _, _, _, _, _) => {
                                h3295
                            }
                        };
                        let a3305 = {
                            let a3302 = {
                                let a3296 = v10;
                                let a3297 = v4;
                                let a3298 = 0u64;
                                nat_rem(a3296, a3297, a3298)
                            };
                            let a3303 = {
                                let a3300 = match v1.clone() {
                                    Adt3::C0(_, _, h3299, _, _, _, _, _) => {
                                        h3299
                                    }
                                };
                                let a3301 = v13;
                                nat_mul(a3300, a3301)?
                            };
                            nat_add(a3302, a3303)?
                        };
                        nat_mul(a3304, a3305)?
                    };
                    nat_add(a3306, a3307)?
                };
                nat_mul(a3308, a3309)?
            };
            nat_add(a3310, a3311)?
        };
        Ok(Adt9::C1(false, f120(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3317 = {
                let a3315 = {
                    let a3312 = v11;
                    let a3313 = v5;
                    let a3314 = 0u64;
                    nat_rem(a3312, a3313, a3314)
                };
                let a3316 = v4;
                nat_mul(a3315, a3316)?
            };
            let a3318 = v8;
            nat_add(a3317, a3318)?
        };
        let v16: u64 = {
            let a3324 = {
                let a3322 = {
                    let a3319 = v11;
                    let a3320 = v5;
                    let a3321 = 0u64;
                    nat_quot(a3319, a3320, a3321)
                };
                let a3323 = v4;
                nat_mul(a3322, a3323)?
            };
            let a3325 = v10;
            nat_add(a3324, a3325)?
        };
        let a3327 = v15;
        let a3328 = match v1.clone() {
            Adt3::C0(_, h3326, _, _, _, _, _, _) => {
                h3326
            }
        };
        if if nat_lt(a3327, a3328) {
            let a3330 = v16;
            let a3331 = match v1.clone() {
                Adt3::C0(_, _, h3329, _, _, _, _, _) => {
                    h3329
                }
            };
            nat_lt(a3330, a3331)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f120({
                let a3340 = v6;
                let a3341 = {
                    let a3338 = match v1.clone() {
                        Adt3::C0(h3332, _, _, _, _, _, _, _) => {
                            h3332
                        }
                    };
                    let a3339 = {
                        let a3336 = v15;
                        let a3337 = {
                            let a3334 = match v1.clone() {
                                Adt3::C0(_, h3333, _, _, _, _, _, _) => {
                                    h3333
                                }
                            };
                            let a3335 = v16;
                            nat_mul(a3334, a3335)?
                        };
                        nat_add(a3336, a3337)?
                    };
                    nat_mul(a3338, a3339)?
                };
                nat_add(a3340, a3341)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f131(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f120(v0, v2.clone())?;
    let v4: u64 = f120(v0, v1.clone())?;
    let a3346 = {
        let a3343 = v0;
        let a3344 = match v1 {
            Adt3::C0(h3342, _, _, _, _, _, _, _) => {
                h3342
            }
        };
        let a3345 = 0u64;
        nat_rem(a3343, a3344, a3345)
    };
    let a3347 = 0u64;
    if nat_eq(a3346, a3347) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f120({
            let a3348 = v0;
            let a3349 = 1u64;
            nat_sub(a3348, a3349)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f132(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3350 = v3;
    let a3351 = 13u64;
    if nat_eq(a3350, a3351) {
        f167(v0, v1, v4)
    } else {
        let v5: u64 = f156(match v1.clone() {
            Adt3::C0(h3352, _, _, _, _, _, _, _) => {
                h3352
            }
        }, {
            let a3353 = v3;
            let a3354 = 16u64;
            nat_eq(a3353, a3354)
        })?;
        let v6: u64 = {
            let a3355 = v0;
            let a3356 = v5;
            let a3357 = 0u64;
            nat_quot(a3355, a3356, a3357)
        };
        let v7: u64 = {
            let a3358 = v0;
            let a3359 = v5;
            let a3360 = 0u64;
            nat_rem(a3358, a3359, a3360)
        };
        let v8: u64 = {
            let a3364 = 1u64;
            let a3365 = {
                let a3362 = match v1.clone() {
                    Adt3::C0(h3361, _, _, _, _, _, _, _) => {
                        h3361
                    }
                };
                let a3363 = 2u64;
                nat_mul(a3362, a3363)?
            };
            nat_add(a3364, a3365)?
        };
        let a3366 = v7;
        let a3367 = v8;
        if nat_lt(a3366, a3367) {
            f168(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f120(v6, v2)?;
            let a3368 = v3;
            let a3369 = 14u64;
            if nat_eq(a3368, a3369) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3370 = v7;
                let a3371 = v8;
                if nat_eq(a3370, a3371) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3374 = v7;
                    let a3375 = {
                        let a3372 = v8;
                        let a3373 = 1u64;
                        nat_add(a3372, a3373)?
                    };
                    if nat_eq(a3374, a3375) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3376, _, _, _, _, _, _, _) => {
                                h3376
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

pub fn f133(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3377 = v0;
    let a3378 = 0u64;
    if nat_eq(a3377, a3378) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3385 = {
            let a3379 = v0;
            let a3380 = 1u64;
            nat_add(a3379, a3380)?
        };
        let a3386 = {
            let a3383 = 2u64;
            let a3384 = {
                let a3381 = v3;
                let a3382 = 2u64;
                nat_mul(a3381, a3382)?
            };
            nat_add(a3383, a3384)?
        };
        if nat_eq(a3385, a3386) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3387 = v0;
                let a3388 = 1u64;
                nat_sub(a3387, a3388)
            };
            let v5: u64 = {
                let a3389 = v4;
                let a3390 = 2u64;
                let a3391 = 0u64;
                nat_quot(a3389, a3390, a3391)
            };
            let a3395 = {
                let a3392 = v4;
                let a3393 = 2u64;
                let a3394 = 0u64;
                nat_rem(a3392, a3393, a3394)
            };
            let a3396 = 0u64;
            if nat_eq(a3395, a3396) {
                Ok(Adt9::C4(17u64, 0u64, f120(v5, v1)?, 4u64, f120(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3397 = v0;
    let a3398 = f22(v1, v2, v3)?;
    if nat_lt(a3397, a3398) {
        let a3399 = v0;
        let a3400 = v1;
        if nat_lt(a3399, a3400) {
            Ok(Adt9::C1(false, {
                let a3401 = 4u64;
                let a3402 = v0;
                nat_mul(a3401, a3402)?
            }, {
                let a3403 = 4u64;
                let a3404 = v0;
                nat_mul(a3403, a3404)?
            }))
        } else {
            let v4: u64 = {
                let a3405 = v0;
                let a3406 = v1;
                nat_sub(a3405, a3406)
            };
            let v5: u64 = f169(v4, v2)?;
            let a3407 = v5;
            let a3408 = 2u64;
            if nat_lt(a3407, a3408) {
                Ok(Adt9::C4(6u64, {
                    let a3409 = v5;
                    let a3410 = 0u64;
                    if nat_eq(a3409, a3410) {
                        4u64
                    } else {
                        5u64
                    }
                }, f170(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f171(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f172(v4, v2)? {
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

pub fn f135(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3418 = {
        let a3411 = 2u64;
        let a3412 = f109(v1.clone())?;
        nat_mul(a3411, a3412)?
    };
    let a3419 = {
        let a3416 = f109(v1)?;
        let a3417 = {
            let a3414 = 1u64;
            let a3415 = match v0 {
                Adt3::C0(h3413, _, _, _, _, _, _, _) => {
                    h3413
                }
            };
            nat_add(a3414, a3415)?
        };
        nat_mul(a3416, a3417)?
    };
    nat_add(a3418, a3419)
}

pub fn f136(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f120(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f137(v0: u64, v1: u64) -> bool {
    let a3420 = v1;
    let a3421 = 2147483647u64;
    if nat_le(a3420, a3421) {
        let a3422 = v1;
        let a3423 = v0;
        nat_lt(a3422, a3423)
    } else {
        false
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3424 = f98(v0);
    match m3424 {
        Adt14::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt14::C1(v7) => {
            let v8: u64 = {
                let a3426 = v2;
                let a3427 = match v7.clone() {
                    Adt15::C0(_, h3425, _) => {
                        h3425
                    }
                };
                nat_mul(a3426, a3427)?
            };
            let m3440 = f173(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3428, _, _, _, _, _, _, _) => {
                    h3428
                }
            }, match v5.clone() {
                Adt3::C0(_, h3429, _, _, _, _, _, _) => {
                    h3429
                }
            }, match v5 {
                Adt3::C0(_, _, h3430, _, _, _, _, _) => {
                    h3430
                }
            }, {
                let a3433 = match v3.clone() {
                    Adt3::C0(h3431, _, _, _, _, _, _, _) => {
                        h3431
                    }
                };
                let a3434 = match v7.clone() {
                    Adt15::C0(_, h3432, _) => {
                        h3432
                    }
                };
                nat_mul(a3433, a3434)?
            }, match v3.clone() {
                Adt3::C0(_, h3435, _, _, _, _, _, _) => {
                    h3435
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3436, _, _, _) => {
                    h3436
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3437, _, _) => {
                    h3437
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3438, _) => {
                    h3438
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3439) => {
                    h3439
                }
            })?;
            match m3440 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3441 = v0;
                    let a3442 = 0u64;
                    if if nat_eq(a3441, a3442) {
                        true
                    } else {
                        let a3443 = v0;
                        let a3444 = 26u64;
                        nat_eq(a3443, a3444)
                    } {
                        Ok(Adt9::C1(false, v9, f120({
                            let a3448 = {
                                let a3446 = v1;
                                let a3447 = match v4.clone() {
                                    Adt3::C0(h3445, _, _, _, _, _, _, _) => {
                                        h3445
                                    }
                                };
                                nat_mul(a3446, a3447)?
                            };
                            let a3449 = v8;
                            nat_add(a3448, a3449)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt15::C0(_, _, h3450) => {
                                h3450
                            }
                        }, f120({
                            let a3454 = {
                                let a3452 = v1;
                                let a3453 = match v4.clone() {
                                    Adt3::C0(h3451, _, _, _, _, _, _, _) => {
                                        h3451
                                    }
                                };
                                nat_mul(a3452, a3453)?
                            };
                            let a3455 = v8;
                            nat_add(a3454, a3455)?
                        }, v4.clone())?, {
                            let a3462 = {
                                let a3460 = {
                                    let a3457 = match v7 {
                                        Adt15::C0(_, h3456, _) => {
                                            h3456
                                        }
                                    };
                                    let a3458 = 1u64;
                                    nat_sub(a3457, a3458)
                                };
                                let a3461 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3459, _, _, _) => {
                                        h3459
                                    }
                                };
                                nat_mul(a3460, a3461)?
                            };
                            let a3463 = 4u64;
                            nat_add(a3462, a3463)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3464, _, _, _) => {
                                h3464
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3465 = v1;
    let a3466 = 0u64;
    if if nat_eq(a3465, a3466) {
        true
    } else {
        let a3467 = v2;
        let a3468 = 0u64;
        nat_eq(a3467, a3468)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f140(v0: u64, v1: u64) -> bool {
    let a3469 = v0;
    let a3470 = 4294967295u64;
    if nat_eq(a3469, a3470) {
        true
    } else {
        let a3471 = v0;
        let a3472 = 2147483648u64;
        if nat_lt(a3471, a3472) {
            let a3473 = v0;
            let a3474 = v1;
            nat_lt(a3473, a3474)
        } else {
            false
        }
    }
}

pub fn f141(v0: Adt7) -> R<u64> {
    let a3479 = if match v0.clone() {
        Adt7::C0(_, _, _, h3475, _) => {
            h3475
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3480 = {
        let a3477 = match v0 {
            Adt7::C0(_, _, _, _, h3476) => {
                h3476
            }
        };
        let a3478 = 1u64;
        if nat_eq(a3477, a3478) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3479, a3480)
}

pub fn f142(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3485 = {
            let a3482 = v2;
            let a3483 = match v0.clone() {
                Adt6::C0(_, h3481, _, _) => {
                    h3481
                }
            };
            nat_mul(a3482, a3483)?
        };
        let a3486 = f113(match v0.clone() {
            Adt6::C0(_, _, h3484, _) => {
                h3484
            }
        });
        nat_add(a3485, a3486)?
    };
    let v4: u64 = f114(match v0.clone() {
        Adt6::C0(_, _, h3487, _) => {
            h3487
        }
    });
    let v5: u64 = {
        let a3491 = {
            let a3489 = v3;
            let a3490 = match v0 {
                Adt6::C0(h3488, _, _, _) => {
                    h3488
                }
            };
            nat_add(a3489, a3490)?
        };
        let a3492 = v4;
        nat_sub(a3491, a3492)
    };
    let a3497 = {
        let a3493 = v5;
        let a3494 = v1;
        if nat_lt(a3493, a3494) {
            v5
        } else {
            v1
        }
    };
    let a3498 = {
        let a3495 = v3;
        let a3496 = v4;
        nat_sub(a3495, a3496)
    };
    Ok(nat_sub(a3497, a3498))
}

pub fn f143(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3507 = f63(match v0.clone() {
        Adt7::C0(h3499, _, _, _, _) => {
            h3499
        }
    }, match v1.clone() {
        Adt3::C0(h3500, _, _, _, _, _, _, _) => {
            h3500
        }
    }, f61(v3, v2.clone(), 0u64), {
        let a3504 = v4;
        let a3505 = {
            let m3503 = match v0.clone() {
                Adt7::C0(h3502, _, _, _, _) => {
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
        nat_rem(a3504, a3505, a3506)
    })?;
    match m3507 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3516 = f63(match v0.clone() {
                Adt7::C0(_, h3508, _, _, _) => {
                    h3508
                }
            }, match v1.clone() {
                Adt3::C0(_, h3509, _, _, _, _, _, _) => {
                    h3509
                }
            }, f61(v3, v2.clone(), 1u64), {
                let a3513 = v4;
                let a3514 = {
                    let m3512 = match v0 {
                        Adt7::C0(h3511, _, _, _, _) => {
                            h3511
                        }
                    };
                    match m3512 {
                        Adt6::C0(h3510, _, _, _) => {
                            h3510
                        }
                    }
                };
                let a3515 = 0u64;
                nat_quot(a3513, a3514, a3515)
            })?;
            match m3516 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f60(v1, v5, v6, f61(v3, v2.clone(), 2u64), f61(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f144(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3523 = v3;
        let a3524 = {
            let a3520 = f120({
                let a3517 = v0;
                let a3518 = 2u64;
                let a3519 = 0u64;
                nat_quot(a3517, a3518, a3519)
            }, v1)?;
            let a3521 = v2;
            let a3522 = 0u64;
            nat_quot(a3520, a3521, a3522)
        };
        nat_add(a3523, a3524)?
    };
    let a3528 = {
        let a3525 = v0;
        let a3526 = 2u64;
        let a3527 = 0u64;
        nat_rem(a3525, a3526, a3527)
    };
    let a3529 = 0u64;
    if nat_eq(a3528, a3529) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3530 = v4;
        let a3531 = 0u64;
        if nat_eq(a3530, a3531) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f145(v0: u64, v1: u64) -> R<u64> {
    let a3534 = 8u64;
    let a3535 = {
        let a3532 = f174(v0);
        let a3533 = v1;
        nat_mul(a3532, a3533)?
    };
    nat_add(a3534, a3535)
}

pub fn f146(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3536, _, _, _, _, _, _, _) => {
            h3536
        }
    };
    let v7: u64 = f174(v3);
    let v8: u64 = {
        let a3539 = 1u64;
        let a3540 = {
            let a3537 = v6;
            let a3538 = v7;
            nat_mul(a3537, a3538)?
        };
        nat_add(a3539, a3540)?
    };
    let a3541 = v0;
    let a3542 = 0u64;
    if nat_eq(a3541, a3542) {
        Ok(f175(0u64))
    } else {
        let a3543 = v0;
        let a3544 = v8;
        if nat_lt(a3543, a3544) {
            let v9: u64 = {
                let a3545 = v0;
                let a3546 = 1u64;
                nat_sub(a3545, a3546)
            };
            let v10: u64 = {
                let a3547 = v9;
                let a3548 = v7;
                let a3549 = 0u64;
                nat_rem(a3547, a3548, a3549)
            };
            let v11: u64 = f120({
                let a3555 = {
                    let a3550 = v1;
                    let a3551 = v6;
                    nat_mul(a3550, a3551)?
                };
                let a3556 = {
                    let a3552 = v9;
                    let a3553 = v7;
                    let a3554 = 0u64;
                    nat_quot(a3552, a3553, a3554)
                };
                nat_add(a3555, a3556)?
            }, v2)?;
            let a3557 = v3;
            let a3558 = 30u64;
            if nat_eq(a3557, a3558) {
                let a3559 = v10;
                let a3560 = 0u64;
                if nat_eq(a3559, a3560) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3561 = v10;
                    let a3562 = 1u64;
                    if nat_eq(a3561, a3562) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3563 = v10;
                        let a3564 = 2u64;
                        if nat_eq(a3563, a3564) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3565 = v10;
                let a3566 = 0u64;
                if nat_eq(a3565, a3566) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3567 = v10;
                    let a3568 = 1u64;
                    if nat_eq(a3567, a3568) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3569 = v0;
                let a3570 = v8;
                nat_sub(a3569, a3570)
            };
            let a3571 = v12;
            let a3572 = 0u64;
            if nat_eq(a3571, a3572) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3573 = v12;
                let a3574 = 1u64;
                if nat_eq(a3573, a3574) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3575 = v12;
                    let a3576 = 2u64;
                    if nat_eq(a3575, a3576) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3577 = v12;
                        let a3578 = 3u64;
                        if nat_eq(a3577, a3578) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3579 = v12;
                            let a3580 = 4u64;
                            if nat_eq(a3579, a3580) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3581 = v12;
                                let a3582 = 5u64;
                                if nat_eq(a3581, a3582) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3583 = v12;
                                    let a3584 = 6u64;
                                    if nat_eq(a3583, a3584) {
                                        if f176(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3585 = 3u64;
                                                let a3586 = v1;
                                                nat_add(a3585, a3586)?
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

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3587 = v0;
        let a3588 = f87(v3);
        let a3589 = 0u64;
        nat_quot(a3587, a3588, a3589)
    };
    let v5: u64 = {
        let a3594 = 3u64;
        let a3595 = {
            let a3591 = v4;
            let a3592 = match v1.clone() {
                Adt3::C0(h3590, _, _, _, _, _, _, _) => {
                    h3590
                }
            };
            let a3593 = 0u64;
            nat_quot(a3591, a3592, a3593)
        };
        nat_add(a3594, a3595)?
    };
    let v6: u64 = f120(v4, v1)?;
    let v7: u64 = f120(v4, v2)?;
    let a3596 = v3;
    let a3597 = 30u64;
    if nat_eq(a3596, a3597) {
        let a3601 = {
            let a3598 = v0;
            let a3599 = 3u64;
            let a3600 = 0u64;
            nat_rem(a3598, a3599, a3600)
        };
        let a3602 = 0u64;
        if nat_eq(a3601, a3602) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3606 = {
                let a3603 = v0;
                let a3604 = 3u64;
                let a3605 = 0u64;
                nat_rem(a3603, a3604, a3605)
            };
            let a3607 = 1u64;
            if nat_eq(a3606, a3607) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f148(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3608 = v0;
        let a3609 = 8388608u64;
        let a3610 = 0u64;
        nat_quot(a3608, a3609, a3610)
    };
    let a3611 = v0;
    let a3612 = 2147483648u64;
    if if if nat_lt(a3611, a3612) {
        let a3613 = 127u64;
        let a3614 = v1;
        nat_le(a3613, a3614)
    } else {
        false
    } {
        let a3615 = v1;
        let a3616 = 190u64;
        nat_lt(a3615, a3616)
    } else {
        false
    } {
        let v2: u64 = {
            let a3620 = 8388608u64;
            let a3621 = {
                let a3617 = v0;
                let a3618 = 8388608u64;
                let a3619 = 0u64;
                nat_rem(a3617, a3618, a3619)
            };
            nat_add(a3620, a3621)?
        };
        let a3622 = v1;
        let a3623 = 150u64;
        if nat_lt(a3622, a3623) {
            let v3: u64 = f177({
                let a3624 = 150u64;
                let a3625 = v1;
                nat_sub(a3624, a3625)
            });
            let a3629 = {
                let a3626 = v2;
                let a3627 = v3;
                let a3628 = 0u64;
                nat_rem(a3626, a3627, a3628)
            };
            let a3630 = 0u64;
            if nat_eq(a3629, a3630) {
                let a3631 = v2;
                let a3632 = v3;
                let a3633 = 0u64;
                Ok(nat_quot(a3631, a3632, a3633))
            } else {
                Ok(0u64)
            }
        } else {
            let a3636 = f177({
                let a3634 = v1;
                let a3635 = 150u64;
                nat_sub(a3634, a3635)
            });
            let a3637 = v2;
            nat_mul(a3636, a3637)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f149(v0: u64, v1: u64, v2: u64) -> bool {
    let a3638 = v0;
    let a3639 = 1u64;
    if nat_le(a3638, a3639) {
        true
    } else {
        let a3643 = {
            let a3640 = v1;
            let a3641 = v2;
            let a3642 = 1u64;
            nat_rem(a3640, a3641, a3642)
        };
        let a3644 = 0u64;
        nat_eq(a3643, a3644)
    }
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3645 = v1;
    let a3646 = 1u64;
    if if if nat_le(a3645, a3646) {
        true
    } else {
        let a3647 = v0;
        let a3648 = v2;
        nat_le(a3647, a3648)
    } {
        let a3649 = v3;
        let a3650 = 1u64;
        if nat_le(a3649, a3650) {
            true
        } else {
            let a3657 = {
                let a3655 = v0;
                let a3656 = {
                    let a3653 = {
                        let a3651 = v1;
                        let a3652 = 1u64;
                        nat_sub(a3651, a3652)
                    };
                    let a3654 = v2;
                    nat_mul(a3653, a3654)?
                };
                nat_add(a3655, a3656)?
            };
            let a3658 = v4;
            nat_le(a3657, a3658)
        }
    } else {
        false
    } {
        let a3659 = v5;
        let a3660 = 1u64;
        if nat_le(a3659, a3660) {
            Ok(true)
        } else {
            let a3673 = {
                let a3671 = {
                    let a3665 = v0;
                    let a3666 = {
                        let a3663 = {
                            let a3661 = v1;
                            let a3662 = 1u64;
                            nat_sub(a3661, a3662)
                        };
                        let a3664 = v2;
                        nat_mul(a3663, a3664)?
                    };
                    nat_add(a3665, a3666)?
                };
                let a3672 = {
                    let a3669 = {
                        let a3667 = v3;
                        let a3668 = 1u64;
                        nat_sub(a3667, a3668)
                    };
                    let a3670 = v4;
                    nat_mul(a3669, a3670)?
                };
                nat_add(a3671, a3672)?
            };
            let a3674 = v6;
            Ok(nat_le(a3673, a3674))
        }
    } else {
        Ok(false)
    }
}

pub fn f151(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f178(v0, v1, v6, v7, v8) {
        let a3675 = v5;
        let a3676 = 26u64;
        nat_eq(a3675, a3676)
    } else {
        false
    } {
        let a3677 = v3;
        let a3678 = v7;
        nat_eq(a3677, a3678)
    } else {
        false
    } {
        let a3679 = v4;
        let a3680 = v8;
        nat_eq(a3679, a3680)
    } else {
        false
    } {
        let a3681 = v9;
        let a3682 = 1u64;
        nat_eq(a3681, a3682)
    } else {
        false
    } {
        let a3683 = v10;
        let a3684 = f179(v0);
        nat_eq(a3683, a3684)
    } else {
        false
    } {
        let a3685 = v11;
        let a3686 = v1;
        nat_eq(a3685, a3686)
    } else {
        false
    } {
        let a3687 = v12;
        let a3688 = v6;
        nat_eq(a3687, a3688)
    } else {
        false
    } {
        let a3689 = v13;
        let a3690 = v7;
        nat_eq(a3689, a3690)
    } else {
        false
    } {
        let a3691 = v14;
        let a3692 = v8;
        nat_eq(a3691, a3692)
    } else {
        false
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3693 = v0;
    let a3694 = 0u64;
    if if if nat_eq(a3693, a3694) {
        true
    } else {
        let a3695 = v1;
        let a3696 = 0u64;
        nat_eq(a3695, a3696)
    } {
        true
    } else {
        let a3697 = v2;
        let a3698 = 0u64;
        nat_eq(a3697, a3698)
    } {
        true
    } else {
        let a3699 = v3;
        let a3700 = 0u64;
        nat_eq(a3699, a3700)
    }
}

pub fn f153(v0: Adt3, v1: bool) -> R<u64> {
    f180(v0.clone(), {
        let a3702 = match v0.clone() {
            Adt3::C0(h3701, _, _, _, _, _, _, _) => {
                h3701
            }
        };
        let a3703 = 1u64;
        nat_sub(a3702, a3703)
    }, {
        let a3705 = match v0.clone() {
            Adt3::C0(_, h3704, _, _, _, _, _, _) => {
                h3704
            }
        };
        let a3706 = 1u64;
        nat_sub(a3705, a3706)
    }, {
        let a3708 = match v0.clone() {
            Adt3::C0(_, _, h3707, _, _, _, _, _) => {
                h3707
            }
        };
        let a3709 = 1u64;
        nat_sub(a3708, a3709)
    }, {
        let a3711 = match v0 {
            Adt3::C0(_, _, _, h3710, _, _, _, _) => {
                h3710
            }
        };
        let a3712 = 1u64;
        nat_sub(a3711, a3712)
    }, v1)
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3713 = v0;
    let a3714 = 0u64;
    if nat_eq(a3713, a3714) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3715 = v0;
            let a3716 = 1u64;
            nat_sub(a3715, a3716)
        };
        let v5: u64 = {
            let a3717 = v4;
            let a3718 = v1;
            let a3719 = 0u64;
            nat_quot(a3717, a3718, a3719)
        };
        let v6: u64 = {
            let a3727 = {
                let a3720 = v5;
                let a3721 = v2;
                nat_mul(a3720, a3721)?
            };
            let a3728 = {
                let a3725 = {
                    let a3722 = v4;
                    let a3723 = v1;
                    let a3724 = 0u64;
                    nat_rem(a3722, a3723, a3724)
                };
                let a3726 = v3;
                nat_mul(a3725, a3726)?
            };
            nat_add(a3727, a3728)?
        };
        let a3729 = v5;
        let a3730 = 0u64;
        if nat_eq(a3729, a3730) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3739 = {
                    let a3733 = {
                        let a3731 = v5;
                        let a3732 = 1u64;
                        nat_sub(a3731, a3732)
                    };
                    let a3734 = v2;
                    nat_mul(a3733, a3734)?
                };
                let a3740 = {
                    let a3737 = {
                        let a3735 = v1;
                        let a3736 = 1u64;
                        nat_sub(a3735, a3736)
                    };
                    let a3738 = v3;
                    nat_mul(a3737, a3738)?
                };
                nat_add(a3739, a3740)?
            };
            let a3741 = v6;
            let a3742 = v7;
            if nat_lt(a3741, a3742) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f155(v0: Adt3, v1: u64) -> bool {
    let a3743 = v1;
    let a3744 = 0u64;
    if !nat_eq(a3743, a3744) {
        true
    } else {
        let a3746 = match v0 {
            Adt3::C0(_, _, _, _, h3745, _, _, _) => {
                h3745
            }
        };
        let a3747 = 4u64;
        nat_eq(a3746, a3747)
    }
}

pub fn f156(v0: u64, v1: bool) -> R<u64> {
    let a3750 = {
        let a3748 = v0;
        let a3749 = 2u64;
        nat_mul(a3748, a3749)?
    };
    let a3751 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3750, a3751)
}

pub fn f157(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m3752 = f181(v0, v1);
    match m3752 {
        None => {
            if v5 {
                let m3753 = f182(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3753 {
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
                let a3754 = 4u64;
                let a3755 = v28;
                nat_le(a3754, a3755)
            };
            let a3756 = v2;
            let a3757 = 0u64;
            if if if if if nat_eq(a3756, a3757) {
                let a3758 = v4;
                let a3759 = 0u64;
                nat_eq(a3758, a3759)
            } else {
                false
            } {
                let a3760 = v6;
                let a3761 = v14;
                if nat_eq(a3760, a3761) {
                    let a3762 = v7;
                    let a3763 = v15;
                    nat_eq(a3762, a3763)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3764 = v8;
                let a3765 = v16;
                if nat_eq(a3764, a3765) {
                    let a3766 = v9;
                    let a3767 = v17;
                    nat_eq(a3766, a3767)
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
                        let a3768 = v3;
                        let a3769 = 0u64;
                        nat_eq(a3768, a3769)
                    } else {
                        false
                    } {
                        f99(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a3774 = {
                        let a3770 = v6;
                        let a3771 = v7;
                        nat_mul(a3770, a3771)?
                    };
                    let a3775 = {
                        let a3772 = v8;
                        let a3773 = v9;
                        nat_mul(a3772, a3773)?
                    };
                    nat_mul(a3774, a3775)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f158(v0: u64) -> u64 {
    let m3776 = f98(v0);
    match m3776 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1(v1) => {
            let a3778 = match v1.clone() {
                Adt15::C0(_, h3777, _) => {
                    h3777
                }
            };
            let a3779 = 1u64;
            if if nat_eq(a3778, a3779) {
                let a3781 = match v1.clone() {
                    Adt15::C0(_, _, h3780) => {
                        h3780
                    }
                };
                let a3782 = 1u64;
                if if if nat_eq(a3781, a3782) {
                    true
                } else {
                    let a3784 = match v1.clone() {
                        Adt15::C0(_, _, h3783) => {
                            h3783
                        }
                    };
                    let a3785 = 2u64;
                    nat_eq(a3784, a3785)
                } {
                    true
                } else {
                    let a3787 = match v1.clone() {
                        Adt15::C0(_, _, h3786) => {
                            h3786
                        }
                    };
                    let a3788 = 4u64;
                    nat_eq(a3787, a3788)
                } {
                    true
                } else {
                    let a3790 = match v1.clone() {
                        Adt15::C0(_, _, h3789) => {
                            h3789
                        }
                    };
                    let a3791 = 8u64;
                    nat_eq(a3790, a3791)
                }
            } else {
                false
            } {
                match v1 {
                    Adt15::C0(_, _, h3792) => {
                        h3792
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f159(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3797 = {
        let a3795 = {
            let a3793 = v0;
            let a3794 = v1;
            nat_sub(a3793, a3794)
        };
        let a3796 = 1u64;
        nat_sub(a3795, a3796)
    };
    let a3798 = v2;
    nat_add(a3797, a3798)
}

pub fn f160(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f112({
        let a3799 = v0;
        let a3800 = v4;
        let a3801 = 0u64;
        nat_rem(a3799, a3800, a3801)
    }, {
        let a3802 = v1;
        let a3803 = v5;
        let a3804 = 0u64;
        nat_rem(a3802, a3803, a3804)
    }, {
        let a3805 = v2;
        let a3806 = v6;
        let a3807 = 0u64;
        nat_rem(a3805, a3806, a3807)
    }, {
        let a3808 = v3;
        let a3809 = v7;
        let a3810 = 0u64;
        nat_rem(a3808, a3809, a3810)
    }, v8, v9, v10, v11)
}

pub fn f161(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3811 = v1;
    let a3812 = 2147483648u64;
    if nat_lt(a3811, a3812) {
        let a3823 = {
            let a3821 = {
                let a3816 = {
                    let a3813 = v0;
                    let a3814 = v2;
                    let a3815 = 0u64;
                    nat_rem(a3813, a3814, a3815)
                };
                let a3817 = v2;
                nat_add(a3816, a3817)?
            };
            let a3822 = {
                let a3818 = v1;
                let a3819 = v2;
                let a3820 = 0u64;
                nat_rem(a3818, a3819, a3820)
            };
            nat_sub(a3821, a3822)
        };
        let a3824 = v2;
        let a3825 = 0u64;
        Ok(nat_rem(a3823, a3824, a3825))
    } else {
        let a3828 = {
            let a3826 = v0;
            let a3827 = f113(v1);
            nat_add(a3826, a3827)?
        };
        let a3829 = v2;
        let a3830 = 0u64;
        Ok(nat_rem(a3828, a3829, a3830))
    }
}

pub fn f162(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3833 = f114(v1);
    let a3834 = {
        let a3831 = v0;
        let a3832 = f113(v1);
        nat_add(a3831, a3832)?
    };
    if nat_le(a3833, a3834) {
        let a3835 = f163(v0, v1)?;
        let a3836 = v2;
        Ok(nat_lt(a3835, a3836))
    } else {
        Ok(false)
    }
}

pub fn f163(v0: u64, v1: u64) -> R<u64> {
    let a3839 = {
        let a3837 = v0;
        let a3838 = f113(v1);
        nat_add(a3837, a3838)?
    };
    let a3840 = f114(v1);
    Ok(nat_sub(a3839, a3840))
}

pub fn f164(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3841 = v0;
    let a3842 = v1;
    if nat_lt(a3841, a3842) {
        let a3843 = v1;
        let a3844 = v0;
        Ok(nat_sub(a3843, a3844))
    } else {
        let a3847 = v0;
        let a3848 = {
            let a3845 = v1;
            let a3846 = v2;
            nat_add(a3845, a3846)?
        };
        if nat_lt(a3847, a3848) {
            let a3849 = v0;
            let a3850 = v1;
            Ok(nat_sub(a3849, a3850))
        } else {
            let a3857 = {
                let a3853 = 2u64;
                let a3854 = {
                    let a3851 = v2;
                    let a3852 = 1u64;
                    nat_sub(a3851, a3852)
                };
                nat_mul(a3853, a3854)?
            };
            let a3858 = {
                let a3855 = v0;
                let a3856 = v1;
                nat_sub(a3855, a3856)
            };
            Ok(nat_sub(a3857, a3858))
        }
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3861 = {
        let a3859 = v2;
        let a3860 = f180(v1.clone(), f61(v0, v1.clone(), 0u64), f61(v0, v1.clone(), 1u64), f61(v0, v1.clone(), 2u64), f61(v0, v1.clone(), 3u64), false)?;
        nat_add(a3859, a3860)?
    };
    let a3862 = f180(v1.clone(), f61(v0, v1.clone(), 0u64), f61(v0, v1.clone(), 1u64), f61(v0, v1.clone(), 2u64), f61(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3861, a3862))
}

pub fn f166(v0: u64, v1: u64, v2: u64) -> bool {
    let a3863 = v0;
    let a3864 = 0u64;
    if nat_eq(a3863, a3864) {
        let a3865 = v2;
        let a3866 = v1;
        nat_le(a3865, a3866)
    } else {
        let a3867 = v0;
        let a3868 = 1u64;
        if nat_eq(a3867, a3868) {
            let a3869 = v2;
            let a3870 = v1;
            nat_lt(a3869, a3870)
        } else {
            let a3871 = v0;
            let a3872 = 2u64;
            if nat_eq(a3871, a3872) {
                let a3873 = v1;
                let a3874 = v2;
                nat_le(a3873, a3874)
            } else {
                let a3875 = v0;
                let a3876 = 3u64;
                if nat_eq(a3875, a3876) {
                    let a3877 = v1;
                    let a3878 = v2;
                    nat_lt(a3877, a3878)
                } else {
                    let a3879 = v1;
                    let a3880 = v2;
                    nat_eq(a3879, a3880)
                }
            }
        }
    }
}

pub fn f167(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a3881 = v0;
    let a3882 = 0u64;
    if nat_eq(a3881, a3882) {
        Ok(f175(0u64))
    } else {
        let a3885 = {
            let a3883 = v0;
            let a3884 = 1u64;
            nat_add(a3883, a3884)?
        };
        let a3886 = f110(v1.clone(), 13u64, v2)?;
        if nat_eq(a3885, a3886) {
            Ok(Adt9::C4({
                let a3887 = v2;
                let a3888 = 0u64;
                if nat_eq(a3887, a3888) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3889 = v0;
                let a3890 = 1u64;
                nat_sub(a3889, a3890)
            };
            if f155(v1.clone(), v2) {
                let v4: u64 = f156(match v1.clone() {
                    Adt3::C0(h3891, _, _, _, _, _, _, _) => {
                        h3891
                    }
                }, false)?;
                let v5: u64 = {
                    let a3892 = v3;
                    let a3893 = v4;
                    let a3894 = 0u64;
                    nat_quot(a3892, a3893, a3894)
                };
                let v6: u64 = {
                    let a3895 = v3;
                    let a3896 = v4;
                    let a3897 = 0u64;
                    nat_rem(a3895, a3896, a3897)
                };
                let a3900 = {
                    let a3898 = v6;
                    let a3899 = 1u64;
                    nat_add(a3898, a3899)?
                };
                let a3901 = v4;
                if nat_eq(a3900, a3901) {
                    Ok(Adt9::C4({
                        let a3902 = v2;
                        let a3903 = 0u64;
                        if nat_eq(a3902, a3903) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f168(v6, v5, v1, v2)
                }
            } else {
                f183({
                    let a3904 = v3;
                    let a3905 = 2u64;
                    let a3906 = 0u64;
                    nat_quot(a3904, a3905, a3906)
                }, {
                    let a3907 = v3;
                    let a3908 = 2u64;
                    let a3909 = 0u64;
                    nat_rem(a3907, a3908, a3909)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f168(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3910 = v0;
    let a3911 = 0u64;
    if nat_eq(a3910, a3911) {
        Ok(f175(1u64))
    } else {
        f183({
            let a3920 = {
                let a3913 = v1;
                let a3914 = match v2.clone() {
                    Adt3::C0(h3912, _, _, _, _, _, _, _) => {
                        h3912
                    }
                };
                nat_mul(a3913, a3914)?
            };
            let a3921 = {
                let a3917 = {
                    let a3915 = v0;
                    let a3916 = 1u64;
                    nat_sub(a3915, a3916)
                };
                let a3918 = 2u64;
                let a3919 = 0u64;
                nat_quot(a3917, a3918, a3919)
            };
            nat_add(a3920, a3921)?
        }, {
            let a3924 = {
                let a3922 = v0;
                let a3923 = 1u64;
                nat_sub(a3922, a3923)
            };
            let a3925 = 2u64;
            let a3926 = 0u64;
            nat_rem(a3924, a3925, a3926)
        }, v2, v3, 1u64)
    }
}

pub fn f169(v0: u64, v1: u64) -> R<u64> {
    let a3927 = v0;
    let a3928 = f82(v1)?;
    let a3929 = 0u64;
    Ok(nat_rem(a3927, a3928, a3929))
}

pub fn f170(v0: u64, v1: u64) -> R<u64> {
    let a3930 = 4u64;
    let a3931 = f184(v0, v1)?;
    nat_mul(a3930, a3931)
}

pub fn f171(v0: u64, v1: u64) -> R<u64> {
    let a3932 = 4u64;
    let a3933 = f185(v0, v1)?;
    nat_mul(a3932, a3933)
}

pub fn f172(v0: u64, v1: u64) -> R<bool> {
    let a3939 = {
        let a3936 = {
            let a3934 = f169(v0, v1)?;
            let a3935 = 2u64;
            nat_sub(a3934, a3935)
        };
        let a3937 = 2u64;
        let a3938 = 0u64;
        nat_rem(a3936, a3937, a3938)
    };
    let a3940 = 0u64;
    Ok(nat_eq(a3939, a3940))
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3941 = f98(v0);
    match m3941 {
        Adt14::C0 => {
            Ok(None::<u64>)
        }
        Adt14::C1(v13) => {
            if if if if if f186(v0) {
                let a3942 = v1;
                let a3943 = v7;
                nat_lt(a3942, a3943)
            } else {
                false
            } {
                f137(v8, v2)
            } else {
                false
            } {
                let a3944 = v3;
                let a3945 = f187(v4, v5, v6)?;
                nat_lt(a3944, a3945)
            } else {
                false
            } {
                let a3950 = {
                    let a3947 = v7;
                    let a3948 = match v13.clone() {
                        Adt15::C0(_, h3946, _) => {
                            h3946
                        }
                    };
                    let a3949 = 0u64;
                    nat_rem(a3947, a3948, a3949)
                };
                let a3951 = 0u64;
                nat_eq(a3950, a3951)
            } else {
                false
            } {
                Ok(Some({
                    let a3980 = {
                        let a3970 = {
                            let a3960 = {
                                let a3956 = {
                                    let a3953 = v1;
                                    let a3954 = match v13 {
                                        Adt15::C0(_, h3952, _) => {
                                            h3952
                                        }
                                    };
                                    let a3955 = 0u64;
                                    nat_quot(a3953, a3954, a3955)
                                };
                                let a3957 = v9;
                                nat_mul(a3956, a3957)?
                            };
                            let a3961 = {
                                let a3958 = v2;
                                let a3959 = v10;
                                nat_mul(a3958, a3959)?
                            };
                            nat_add(a3960, a3961)?
                        };
                        let a3971 = {
                            let a3968 = {
                                let a3965 = {
                                    let a3962 = v3;
                                    let a3963 = v4;
                                    let a3964 = 0u64;
                                    nat_quot(a3962, a3963, a3964)
                                };
                                let a3966 = v5;
                                let a3967 = 0u64;
                                nat_rem(a3965, a3966, a3967)
                            };
                            let a3969 = v11;
                            nat_mul(a3968, a3969)?
                        };
                        nat_add(a3970, a3971)?
                    };
                    let a3981 = {
                        let a3978 = {
                            let a3975 = {
                                let a3972 = v3;
                                let a3973 = v4;
                                let a3974 = 0u64;
                                nat_quot(a3972, a3973, a3974)
                            };
                            let a3976 = v5;
                            let a3977 = 0u64;
                            nat_quot(a3975, a3976, a3977)
                        };
                        let a3979 = v12;
                        nat_mul(a3978, a3979)?
                    };
                    nat_add(a3980, a3981)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f174(v0: u64) -> u64 {
    let a3982 = v0;
    let a3983 = 30u64;
    if nat_eq(a3982, a3983) {
        4u64
    } else {
        3u64
    }
}

pub fn f175(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f176(v0: u64) -> bool {
    let a3984 = 0u64;
    let a3985 = v0;
    if nat_lt(a3984, a3985) {
        let a3986 = v0;
        let a3987 = 2139095040u64;
        nat_le(a3986, a3987)
    } else {
        false
    }
}

pub fn f177(v0: u64) -> u64 {
    let a3988 = v0;
    let a3989 = 40u64;
    if nat_lt(a3988, a3989) {
        let a3990 = v0;
        let a3991 = 20u64;
        if nat_lt(a3990, a3991) {
            let a3992 = v0;
            let a3993 = 10u64;
            if nat_lt(a3992, a3993) {
                let a3994 = v0;
                let a3995 = 5u64;
                if nat_lt(a3994, a3995) {
                    let a3996 = v0;
                    let a3997 = 2u64;
                    if nat_lt(a3996, a3997) {
                        let a3998 = v0;
                        let a3999 = 1u64;
                        if nat_lt(a3998, a3999) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4000 = v0;
                        let a4001 = 3u64;
                        if nat_lt(a4000, a4001) {
                            4u64
                        } else {
                            let a4002 = v0;
                            let a4003 = 4u64;
                            if nat_lt(a4002, a4003) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4004 = v0;
                    let a4005 = 7u64;
                    if nat_lt(a4004, a4005) {
                        let a4006 = v0;
                        let a4007 = 6u64;
                        if nat_lt(a4006, a4007) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4008 = v0;
                        let a4009 = 8u64;
                        if nat_lt(a4008, a4009) {
                            128u64
                        } else {
                            let a4010 = v0;
                            let a4011 = 9u64;
                            if nat_lt(a4010, a4011) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4012 = v0;
                let a4013 = 15u64;
                if nat_lt(a4012, a4013) {
                    let a4014 = v0;
                    let a4015 = 12u64;
                    if nat_lt(a4014, a4015) {
                        let a4016 = v0;
                        let a4017 = 11u64;
                        if nat_lt(a4016, a4017) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4018 = v0;
                        let a4019 = 13u64;
                        if nat_lt(a4018, a4019) {
                            4096u64
                        } else {
                            let a4020 = v0;
                            let a4021 = 14u64;
                            if nat_lt(a4020, a4021) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4022 = v0;
                    let a4023 = 17u64;
                    if nat_lt(a4022, a4023) {
                        let a4024 = v0;
                        let a4025 = 16u64;
                        if nat_lt(a4024, a4025) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4026 = v0;
                        let a4027 = 18u64;
                        if nat_lt(a4026, a4027) {
                            131072u64
                        } else {
                            let a4028 = v0;
                            let a4029 = 19u64;
                            if nat_lt(a4028, a4029) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4030 = v0;
            let a4031 = 30u64;
            if nat_lt(a4030, a4031) {
                let a4032 = v0;
                let a4033 = 25u64;
                if nat_lt(a4032, a4033) {
                    let a4034 = v0;
                    let a4035 = 22u64;
                    if nat_lt(a4034, a4035) {
                        let a4036 = v0;
                        let a4037 = 21u64;
                        if nat_lt(a4036, a4037) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4038 = v0;
                        let a4039 = 23u64;
                        if nat_lt(a4038, a4039) {
                            4194304u64
                        } else {
                            let a4040 = v0;
                            let a4041 = 24u64;
                            if nat_lt(a4040, a4041) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4042 = v0;
                    let a4043 = 27u64;
                    if nat_lt(a4042, a4043) {
                        let a4044 = v0;
                        let a4045 = 26u64;
                        if nat_lt(a4044, a4045) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4046 = v0;
                        let a4047 = 28u64;
                        if nat_lt(a4046, a4047) {
                            134217728u64
                        } else {
                            let a4048 = v0;
                            let a4049 = 29u64;
                            if nat_lt(a4048, a4049) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4050 = v0;
                let a4051 = 35u64;
                if nat_lt(a4050, a4051) {
                    let a4052 = v0;
                    let a4053 = 32u64;
                    if nat_lt(a4052, a4053) {
                        let a4054 = v0;
                        let a4055 = 31u64;
                        if nat_lt(a4054, a4055) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4056 = v0;
                        let a4057 = 33u64;
                        if nat_lt(a4056, a4057) {
                            4294967296u64
                        } else {
                            let a4058 = v0;
                            let a4059 = 34u64;
                            if nat_lt(a4058, a4059) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4060 = v0;
                    let a4061 = 37u64;
                    if nat_lt(a4060, a4061) {
                        let a4062 = v0;
                        let a4063 = 36u64;
                        if nat_lt(a4062, a4063) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4064 = v0;
                        let a4065 = 38u64;
                        if nat_lt(a4064, a4065) {
                            137438953472u64
                        } else {
                            let a4066 = v0;
                            let a4067 = 39u64;
                            if nat_lt(a4066, a4067) {
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

pub fn f178(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f152(v1, v2, v3, v4) {
        f188(v0)
    } else {
        f186(v0)
    }
}

pub fn f179(v0: u64) -> u64 {
    let a4068 = v0;
    let a4069 = 26u64;
    if nat_eq(a4068, a4069) {
        26u64
    } else {
        0u64
    }
}

pub fn f180(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4090 = {
        let a4084 = {
            let a4078 = {
                let a4072 = v1;
                let a4073 = if v5 {
                    f113(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4070, _, _, _) => {
                            h4070
                        }
                    })
                } else {
                    f114(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4071, _, _, _) => {
                            h4071
                        }
                    })
                };
                nat_mul(a4072, a4073)?
            };
            let a4079 = {
                let a4076 = v2;
                let a4077 = if v5 {
                    f113(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4074, _, _) => {
                            h4074
                        }
                    })
                } else {
                    f114(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4075, _, _) => {
                            h4075
                        }
                    })
                };
                nat_mul(a4076, a4077)?
            };
            nat_add(a4078, a4079)?
        };
        let a4085 = {
            let a4082 = v3;
            let a4083 = if v5 {
                f113(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4080, _) => {
                        h4080
                    }
                })
            } else {
                f114(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4081, _) => {
                        h4081
                    }
                })
            };
            nat_mul(a4082, a4083)?
        };
        nat_add(a4084, a4085)?
    };
    let a4091 = {
        let a4088 = v4;
        let a4089 = if v5 {
            f113(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4086) => {
                    h4086
                }
            })
        } else {
            f114(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4087) => {
                    h4087
                }
            })
        };
        nat_mul(a4088, a4089)?
    };
    nat_add(a4090, a4091)
}

pub fn f181(v0: u64, v1: u64) -> Option<u64> {
    let a4092 = v0;
    let a4093 = 2u64;
    if nat_eq(a4092, a4093) {
        Some(0u64)
    } else {
        let a4094 = v0;
        let a4095 = 5u64;
        if nat_eq(a4094, a4095) {
            Some(1u64)
        } else {
            let a4096 = v0;
            let a4097 = 6u64;
            if nat_eq(a4096, a4097) {
                Some(2u64)
            } else {
                let a4098 = v0;
                let a4099 = 7u64;
                if nat_eq(a4098, a4099) {
                    Some(3u64)
                } else {
                    let a4100 = v0;
                    let a4101 = 9u64;
                    if nat_eq(a4100, a4101) {
                        Some(4u64)
                    } else {
                        let a4102 = v0;
                        let a4103 = 8u64;
                        if nat_eq(a4102, a4103) {
                            Some(5u64)
                        } else {
                            let a4104 = v0;
                            let a4105 = 89u64;
                            if nat_eq(a4104, a4105) {
                                let a4106 = v1;
                                let a4107 = 2u64;
                                if nat_eq(a4106, a4107) {
                                    Some(6u64)
                                } else {
                                    let a4108 = v1;
                                    let a4109 = 0u64;
                                    if nat_eq(a4108, a4109) {
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

pub fn f182(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a4110 = v0;
    let a4111 = 30u64;
    if nat_eq(a4110, a4111) {
        let a4112 = v2;
        let a4113 = 0u64;
        if if if nat_eq(a4112, a4113) {
            true
        } else {
            let a4114 = v2;
            let a4115 = 1u64;
            nat_eq(a4114, a4115)
        } {
            let a4116 = v1;
            let a4117 = 0u64;
            if nat_eq(a4116, a4117) {
                true
            } else {
                f189(v1, v4)
            }
        } else {
            false
        } {
            f190(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    } else {
        if if if f191(v1) {
            f191(v2)
        } else {
            false
        } {
            let a4118 = v1;
            let a4119 = 0u64;
            if if nat_eq(a4118, a4119) {
                true
            } else {
                let a4120 = v2;
                let a4121 = 0u64;
                nat_eq(a4120, a4121)
            } {
                true
            } else {
                let a4122 = v1;
                let a4123 = v2;
                nat_eq(a4122, a4123)
            }
        } else {
            false
        } {
            f190(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    }
}

pub fn f183(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4124 = v1;
    let a4125 = 0u64;
    if nat_eq(a4124, a4125) {
        Ok(Adt9::C4({
            let a4126 = v3;
            let a4127 = 0u64;
            if nat_eq(a4126, a4127) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f120(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4128 = v3;
            let a4129 = 0u64;
            if nat_eq(a4128, a4129) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f184(v0: u64, v1: u64) -> R<u64> {
    let a4130 = v0;
    let a4131 = f82(v1)?;
    let a4132 = 0u64;
    Ok(nat_quot(a4130, a4131, a4132))
}

pub fn f185(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f184(v0, v1)?;
    let v3: u64 = {
        let a4135 = {
            let a4133 = f169(v0, v1)?;
            let a4134 = 2u64;
            nat_sub(a4133, a4134)
        };
        let a4136 = 2u64;
        let a4137 = 0u64;
        nat_quot(a4135, a4136, a4137)
    };
    let v4: u64 = {
        let a4138 = v2;
        let a4139 = v1;
        nat_mul(a4138, a4139)?
    };
    if f172(v0, v1)? {
        let a4140 = v4;
        let a4141 = v3;
        nat_add(a4140, a4141)
    } else {
        let a4153 = {
            let a4149 = v4;
            let a4150 = {
                let a4147 = {
                    let a4142 = v1;
                    let a4143 = 1u64;
                    nat_sub(a4142, a4143)
                };
                let a4148 = {
                    let a4144 = v2;
                    let a4145 = v1;
                    let a4146 = 0u64;
                    nat_rem(a4144, a4145, a4146)
                };
                nat_mul(a4147, a4148)?
            };
            nat_sub(a4149, a4150)
        };
        let a4154 = {
            let a4151 = v3;
            let a4152 = v1;
            nat_mul(a4151, a4152)?
        };
        nat_add(a4153, a4154)
    }
}

pub fn f186(v0: u64) -> bool {
    let a4155 = v0;
    let a4156 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4155, a4156) {
        true
    } else {
        let a4157 = v0;
        let a4158 = 1u64;
        nat_eq(a4157, a4158)
    } {
        true
    } else {
        let a4159 = v0;
        let a4160 = 2u64;
        nat_eq(a4159, a4160)
    } {
        true
    } else {
        let a4161 = v0;
        let a4162 = 3u64;
        nat_eq(a4161, a4162)
    } {
        true
    } else {
        let a4163 = v0;
        let a4164 = 6u64;
        nat_eq(a4163, a4164)
    } {
        true
    } else {
        let a4165 = v0;
        let a4166 = 7u64;
        nat_eq(a4165, a4166)
    } {
        true
    } else {
        let a4167 = v0;
        let a4168 = 8u64;
        nat_eq(a4167, a4168)
    } {
        true
    } else {
        let a4169 = v0;
        let a4170 = 10u64;
        nat_eq(a4169, a4170)
    } {
        true
    } else {
        let a4171 = v0;
        let a4172 = 11u64;
        nat_eq(a4171, a4172)
    } {
        true
    } else {
        let a4173 = v0;
        let a4174 = 12u64;
        nat_eq(a4173, a4174)
    } {
        true
    } else {
        let a4175 = v0;
        let a4176 = 13u64;
        nat_eq(a4175, a4176)
    } {
        true
    } else {
        let a4177 = v0;
        let a4178 = 14u64;
        nat_eq(a4177, a4178)
    } {
        true
    } else {
        let a4179 = v0;
        let a4180 = 16u64;
        nat_eq(a4179, a4180)
    } {
        true
    } else {
        let a4181 = v0;
        let a4182 = 17u64;
        nat_eq(a4181, a4182)
    } {
        true
    } else {
        let a4183 = v0;
        let a4184 = 18u64;
        nat_eq(a4183, a4184)
    } {
        true
    } else {
        let a4185 = v0;
        let a4186 = 19u64;
        nat_eq(a4185, a4186)
    } {
        true
    } else {
        let a4187 = v0;
        let a4188 = 20u64;
        nat_eq(a4187, a4188)
    } {
        true
    } else {
        let a4189 = v0;
        let a4190 = 21u64;
        nat_eq(a4189, a4190)
    } {
        true
    } else {
        let a4191 = v0;
        let a4192 = 22u64;
        nat_eq(a4191, a4192)
    } {
        true
    } else {
        let a4193 = v0;
        let a4194 = 23u64;
        nat_eq(a4193, a4194)
    } {
        true
    } else {
        let a4195 = v0;
        let a4196 = 26u64;
        nat_eq(a4195, a4196)
    } {
        true
    } else {
        let a4197 = v0;
        let a4198 = 29u64;
        nat_eq(a4197, a4198)
    } {
        true
    } else {
        let a4199 = v0;
        let a4200 = 30u64;
        nat_eq(a4199, a4200)
    } {
        true
    } else {
        let a4201 = v0;
        let a4202 = 34u64;
        nat_eq(a4201, a4202)
    } {
        true
    } else {
        let a4203 = v0;
        let a4204 = 35u64;
        nat_eq(a4203, a4204)
    } {
        true
    } else {
        let a4205 = v0;
        let a4206 = 39u64;
        nat_eq(a4205, a4206)
    } {
        true
    } else {
        let a4207 = v0;
        let a4208 = 40u64;
        nat_eq(a4207, a4208)
    } {
        true
    } else {
        let a4209 = v0;
        let a4210 = 41u64;
        nat_eq(a4209, a4210)
    } {
        true
    } else {
        let a4211 = v0;
        let a4212 = 42u64;
        nat_eq(a4211, a4212)
    } {
        true
    } else {
        let a4213 = v0;
        let a4214 = 43u64;
        nat_eq(a4213, a4214)
    }
}

pub fn f187(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4215 = v0;
    let a4216 = 0u64;
    if if if nat_eq(a4215, a4216) {
        true
    } else {
        let a4217 = v1;
        let a4218 = 0u64;
        nat_eq(a4217, a4218)
    } {
        true
    } else {
        let a4219 = v2;
        let a4220 = 0u64;
        nat_eq(a4219, a4220)
    } {
        Ok(0u64)
    } else {
        let a4223 = {
            let a4221 = v0;
            let a4222 = v1;
            nat_mul(a4221, a4222)?
        };
        let a4224 = v2;
        nat_mul(a4223, a4224)
    }
}

pub fn f188(v0: u64) -> bool {
    let m4225 = f98(v0);
    match m4225 {
        Adt14::C0 => {
            false
        }
        Adt14::C1(_) => {
            true
        }
    }
}

pub fn f189(v0: u64, v1: u64) -> bool {
    let a4226 = v0;
    let a4227 = 1u64;
    if nat_eq(a4226, a4227) {
        true
    } else {
        let v2: u64 = f192(v0);
        let a4228 = 0u64;
        let a4229 = v2;
        if nat_lt(a4228, a4229) {
            let a4233 = {
                let a4230 = v1;
                let a4231 = v2;
                let a4232 = 1u64;
                nat_rem(a4230, a4231, a4232)
            };
            let a4234 = 0u64;
            nat_eq(a4233, a4234)
        } else {
            false
        }
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a4235 = v0;
    let a4236 = 28u64;
    if if if if nat_eq(a4235, a4236) {
        let a4237 = v1;
        let a4238 = 0u64;
        nat_eq(a4237, a4238)
    } else {
        false
    } {
        let a4239 = v2;
        let a4240 = 0u64;
        nat_eq(a4239, a4240)
    } else {
        false
    } {
        let a4241 = v3;
        let a4242 = 0u64;
        nat_eq(a4241, a4242)
    } else {
        false
    } {
        let a4243 = v4;
        let a4244 = v8;
        if if if if if if if if nat_eq(a4243, a4244) {
            let a4245 = v5;
            let a4246 = v12;
            nat_eq(a4245, a4246)
        } else {
            false
        } {
            let a4247 = v9;
            let a4248 = v13;
            nat_eq(a4247, a4248)
        } else {
            false
        } {
            let a4249 = v10;
            let a4250 = v14;
            nat_eq(a4249, a4250)
        } else {
            false
        } {
            let a4251 = v11;
            let a4252 = v15;
            nat_eq(a4251, a4252)
        } else {
            false
        } {
            let a4253 = 0u64;
            let a4254 = v6;
            nat_lt(a4253, a4254)
        } else {
            false
        } {
            let a4255 = 0u64;
            let a4256 = v7;
            nat_lt(a4255, a4256)
        } else {
            false
        } {
            let a4260 = {
                let a4257 = v10;
                let a4258 = v6;
                let a4259 = 1u64;
                nat_rem(a4257, a4258, a4259)
            };
            let a4261 = 0u64;
            if nat_eq(a4260, a4261) {
                let a4265 = {
                    let a4262 = v11;
                    let a4263 = v7;
                    let a4264 = 1u64;
                    nat_rem(a4262, a4263, a4264)
                };
                let a4266 = 0u64;
                nat_eq(a4265, a4266)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt16::C2(v5, v4, v9, v10, v11, {
                let a4267 = v10;
                let a4268 = v6;
                let a4269 = 0u64;
                nat_quot(a4267, a4268, a4269)
            }, {
                let a4270 = v11;
                let a4271 = v7;
                let a4272 = 0u64;
                nat_quot(a4270, a4271, a4272)
            }, {
                let a4273 = v5;
                let a4274 = v4;
                nat_mul(a4273, a4274)?
            }, {
                let a4275 = v4;
                let a4276 = v9;
                nat_mul(a4275, a4276)?
            }, {
                let a4277 = v5;
                let a4278 = v9;
                nat_mul(a4277, a4278)?
            }))
        } else {
            Ok(Adt16::C1)
        }
    } else {
        Ok(Adt16::C0)
    }
}

pub fn f191(v0: u64) -> bool {
    let a4279 = v0;
    let a4280 = 0u64;
    if if nat_eq(a4279, a4280) {
        true
    } else {
        let a4281 = v0;
        let a4282 = 1u64;
        nat_eq(a4281, a4282)
    } {
        true
    } else {
        let a4283 = v0;
        let a4284 = 30u64;
        nat_eq(a4283, a4284)
    }
}

pub fn f192(v0: u64) -> u64 {
    if f186(v0) {
        let m4285 = f98(v0);
        match m4285 {
            Adt14::C0 => {
                0u64
            }
            Adt14::C1(v1) => {
                let a4287 = 1u64;
                let a4288 = match v1.clone() {
                    Adt15::C0(_, h4286, _) => {
                        h4286
                    }
                };
                if if nat_lt(a4287, a4288) {
                    true
                } else {
                    let a4289 = v0;
                    let a4290 = 43u64;
                    nat_eq(a4289, a4290)
                } {
                    match v1 {
                        Adt15::C0(_, h4291, _) => {
                            h4291
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
