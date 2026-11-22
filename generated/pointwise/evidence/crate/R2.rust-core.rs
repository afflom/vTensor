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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4, u64),
    C2(Adt4),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
    C6(u64, u64, u64, bool),
    C7(Adt7, Adt8, Adt8, Adt9, u64, u64),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2(u64, bool, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
    C4(u64, u64, Adt3, Adt3, Adt4),
    C5(u64),
}

#[derive(Clone)]
pub enum Adt7 {
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
pub enum Adt8 {
    C0(u64, u64),
    C1(u64),
    C2(u64),
    C3(u64),
    C4,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(u64),
    C1(u64),
    C2,
}

#[derive(Clone)]
pub enum Adt10 {
    C0,
    C1(Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt12 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5, _, _, _, _, _, _, _, v13) => {
            let a2 = v1;
            let a3 = 64u64;
            if nat_eq(a2, a3) {
                Ok(Adt5::C0(f1(v2, v13)?))
            } else {
                let m4 = v3;
                match m4 {
                    None => {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                    Some(v14) => {
                        let a5 = v1;
                        let a6 = 78u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt5::C0(f2(v2, v14, v4, v5, v13)?))
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
                                Ok(Adt5::C0(Adt6::C5(64u64)))
                            } else {
                                let a13 = v1;
                                let a14 = 24u64;
                                if nat_eq(a13, a14) {
                                    Ok(Adt5::C0(f3(v2, v14, v13)?))
                                } else {
                                    let a15 = v1;
                                    let a16 = 27u64;
                                    if nat_eq(a15, a16) {
                                        Ok(Adt5::C0(f4(v2, v14, v13)?))
                                    } else {
                                        let a17 = v1;
                                        let a18 = 3u64;
                                        if nat_eq(a17, a18) {
                                            Ok(Adt5::C0(f5(v14, v4, v5, v13)?))
                                        } else {
                                            f6(v1, v2, v14, v4, v13)
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
            f7(v15, v16, v17, v18, v19)
        }
        Adt0::C2(v20) => {
            f8(v20)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f9(v1.clone(), 1u64)?;
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
        f10(match v0.clone() {
            Adt1::C0(h25, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        })
    } else {
        false
    } {
        f10(match v0.clone() {
            Adt1::C0(_, h26, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        })
    } else {
        false
    } {
        let a28 = 0u64;
        let a29 = match v0.clone() {
            Adt1::C0(_, _, h27, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        };
        nat_lt(a28, a29)
    } else {
        false
    } {
        let a31 = match v0.clone() {
            Adt1::C0(_, _, h30, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
        let a36 = f11(v2)?;
        let a37 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h35, _) => {
                h35
            }
        };
        nat_le(a36, a37)
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v2)?, 4u64, f13(v1.clone(), 1u64), f13(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h38, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }, match v0.clone() {
            Adt1::C0(_, h39, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        }, match v0 {
            Adt1::C0(_, _, h40, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m41 = v2;
    match m41 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let m42 = v3;
            match m42 {
                None => {
                    Ok(Adt6::C1)
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
                        f14(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f14(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f15(v1.clone())?
                    } else {
                        false
                    } {
                        f15(v5.clone())?
                    } else {
                        false
                    } {
                        f15(v6)?
                    } else {
                        false
                    } {
                        f15(v4.clone())?
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
                            Adt1::C0(h73, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        };
                        let a75 = 4294967296u64;
                        nat_lt(a74, a75)
                    } else {
                        false
                    } {
                        let a77 = match v0.clone() {
                            Adt1::C0(h76, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h76
                            }
                        };
                        let a78 = 0u64;
                        if nat_eq(a77, a78) {
                            if f16(v1.clone(), v4.clone())? {
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
                                Adt1::C0(h100, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h100
                                }
                            };
                            let a102 = 0u64;
                            if nat_eq(a101, a102) {
                                f9(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f17(v8, match v5.clone() {
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
                        Ok(Adt6::C4(f18(v9, v8, v10)?, 4u64, f13(v1, 1u64), f13(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h106, _, _, _, _, _, _, _, _, _, _) => {
            h106
        }
    };
    let v4: u64 = f19(v3);
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
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(h117, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h117
            }
        })
    } else {
        false
    } {
        f21(v1.clone(), v4)
    } else {
        false
    } {
        f21(v2.clone(), v4)
    } else {
        false
    } {
        if f16(v1.clone(), v2.clone())? {
            true
        } else {
            f22(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f23(match v1.clone() {
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
                f24(v2.clone(), v4)?
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
        Ok(Adt6::C4(f25(v3, match v1.clone() {
            Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, v5, v6)?, v4, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, v5, f26(match v1 {
            Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
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
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f27(match v0.clone() {
            Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h144
            }
        })
    } else {
        false
    } {
        if f16(v1.clone(), v2.clone())? {
            true
        } else {
            f22(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f13(v1, 1u64);
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
        Ok(Adt6::C4(f28(match v3.clone() {
            Adt3::C0(h152, _, _, _, _, _, _, _) => {
                h152
            }
        }, v4)?, 4u64, v3, f13(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h153, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h153
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m154 = v1;
    match m154 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m155 = v2;
            match m155 {
                None => {
                    Ok(Adt6::C1)
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
                        f14(v0.clone(), v3.clone())
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
                        f16(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f16(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f16(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f29(match v0.clone() {
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
                        })?, 4u64, f13(v0, 1u64), f13(v3, 1u64), Adt4::C22(f13(v4, 1u64), f13(v5, 1u64))))
                    } else {
                        Ok(Adt6::C1)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h202, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h202
        }
    };
    Ok(Adt5::C0({
        let a203 = v0;
        let a204 = 39u64;
        if nat_eq(a203, a204) {
            f30(v2, v3, v4)?
        } else {
            let a205 = v0;
            let a206 = 77u64;
            if nat_eq(a205, a206) {
                f31(v2, v4)?
            } else {
                let a207 = v0;
                let a208 = 40u64;
                if nat_eq(a207, a208) {
                    f32(v2, v3, v4)?
                } else {
                    if f33(v0) {
                        f34(v0, v2, {
                            let m209 = v3;
                            match m209 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a210 = v0;
                        let a211 = 19u64;
                        if nat_eq(a210, a211) {
                            f35(v2, v4)?
                        } else {
                            let a212 = v0;
                            let a213 = 21u64;
                            if nat_eq(a212, a213) {
                                f36(v5, v2, v3, v4)?
                            } else {
                                let a214 = v0;
                                let a215 = 61u64;
                                if if if nat_eq(a214, a215) {
                                    true
                                } else {
                                    let a216 = v0;
                                    let a217 = 62u64;
                                    nat_eq(a216, a217)
                                } {
                                    true
                                } else {
                                    let a218 = v0;
                                    let a219 = 63u64;
                                    nat_eq(a218, a219)
                                } {
                                    f37(v0, v1, v2, v4)?
                                } else {
                                    let a220 = v0;
                                    let a221 = 4u64;
                                    if if nat_eq(a220, a221) {
                                        true
                                    } else {
                                        let a222 = v0;
                                        let a223 = 32u64;
                                        nat_eq(a222, a223)
                                    } {
                                        f38(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a224 = v0;
                                        let a225 = 20u64;
                                        if nat_eq(a224, a225) {
                                            f39(v2, v4)?
                                        } else {
                                            let a226 = v0;
                                            let a227 = 42u64;
                                            if nat_eq(a226, a227) {
                                                f40(4u64, v2, v4)?
                                            } else {
                                                let a228 = v0;
                                                let a229 = 69u64;
                                                if nat_eq(a228, a229) {
                                                    let a230 = v5;
                                                    let a231 = 4u64;
                                                    if nat_lt(a230, a231) {
                                                        f40(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a232 = v0;
                                                    let a233 = 70u64;
                                                    if nat_eq(a232, a233) {
                                                        f41(v5, v2, v4)?
                                                    } else {
                                                        let a234 = v0;
                                                        let a235 = 75u64;
                                                        if if nat_eq(a234, a235) {
                                                            true
                                                        } else {
                                                            let a236 = v0;
                                                            let a237 = 76u64;
                                                            nat_eq(a236, a237)
                                                        } {
                                                            f42({
                                                                let a238 = v0;
                                                                let a239 = 76u64;
                                                                nat_eq(a238, a239)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a240 = v0;
                                                            let a241 = 31u64;
                                                            if if if nat_eq(a240, a241) {
                                                                true
                                                            } else {
                                                                let a242 = v0;
                                                                let a243 = 47u64;
                                                                nat_eq(a242, a243)
                                                            } {
                                                                true
                                                            } else {
                                                                let a244 = v0;
                                                                let a245 = 68u64;
                                                                nat_eq(a244, a245)
                                                            } {
                                                                f43(v0, v1, v2, v4)?
                                                            } else {
                                                                let a246 = v0;
                                                                let a247 = 15u64;
                                                                if nat_eq(a246, a247) {
                                                                    f44(v2, v4)?
                                                                } else {
                                                                    let a248 = v0;
                                                                    let a249 = 17u64;
                                                                    if nat_eq(a248, a249) {
                                                                        f45(v2, v4)?
                                                                    } else {
                                                                        let a250 = v0;
                                                                        let a251 = 18u64;
                                                                        if nat_eq(a250, a251) {
                                                                            f46(v2, v3, v4)?
                                                                        } else {
                                                                            let a252 = v0;
                                                                            let a253 = 13u64;
                                                                            if if if nat_eq(a252, a253) {
                                                                                true
                                                                            } else {
                                                                                let a254 = v0;
                                                                                let a255 = 14u64;
                                                                                nat_eq(a254, a255)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a256 = v0;
                                                                                let a257 = 16u64;
                                                                                nat_eq(a256, a257)
                                                                            } {
                                                                                f47(v0, v2, v4)?
                                                                            } else {
                                                                                let m258 = f48(v0, v5)?;
                                                                                match m258 {
                                                                                    None => {
                                                                                        f49(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f50(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f51(v3.clone(), {
        let m259 = v3.clone();
        match m259 {
            Adt4::C0 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f53(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f54(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f55(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f56(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v57, v58, v59, v60) => {
                f58(v0, v60, v57, v58, v59, v4)?
            }
            Adt4::C24(_) => {
                Adt5::C0(Adt6::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f52(v0, v1, v2, v3)?
            }
            Adt4::C26(v52, v53, v54, v55) => {
                let a260 = v52;
                let a261 = 64u64;
                if if if nat_eq(a260, a261) {
                    true
                } else {
                    let a262 = v52;
                    let a263 = 65u64;
                    nat_eq(a262, a263)
                } {
                    true
                } else {
                    let a264 = v52;
                    let a265 = 66u64;
                    nat_eq(a264, a265)
                } {
                    Adt5::C0(Adt6::C5(64u64))
                } else {
                    let a266 = v52;
                    let a267 = 0u64;
                    if if nat_eq(a266, a267) {
                        true
                    } else {
                        let a268 = v52;
                        let a269 = 30u64;
                        nat_eq(a268, a269)
                    } {
                        f57(v0, v1, v2, v52, v53, v54, v55, v4)?
                    } else {
                        Adt5::C0(Adt6::C1)
                    }
                }
            }
        }
    }))
}

pub fn f8(v0: Adt4) -> R<Adt5> {
    let m270 = v0;
    match m270 {
        Adt4::C0 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C1 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C2(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C3(_, _, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C4(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C5(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C6(_, _, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C7(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C8(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C9(v18, v19) => {
            Ok(Adt5::C6(0u64, v18, v19, true))
        }
        Adt4::C10(_, v21, _) => {
            Ok(Adt5::C6(if v21 {
                1u64
            } else {
                3u64
            }, 0u64, 0u64, false))
        }
        Adt4::C11(_, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C12(_, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C13(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C14(_, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C15 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C16(_, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C17 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C18(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C21 => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C6(7u64, 0u64, 0u64, false))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C6(0u64, 0u64, 0u64, false))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C25(v45, _, v47) => {
            Ok(Adt5::C6({
                let a275 = {
                    let a271 = 0u64;
                    let a272 = v45;
                    if nat_lt(a271, a272) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a276 = {
                    let a273 = 0u64;
                    let a274 = v47;
                    if nat_lt(a273, a274) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a275, a276)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v48, _, _, _) => {
            Ok(Adt5::C6({
                let a277 = v48;
                let a278 = 66u64;
                if nat_eq(a277, a278) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false))
        }
    }
}

pub fn f9(v0: Adt2, v1: u64) -> R<u64> {
    let a290 = {
        let a284 = {
            let a280 = match v0.clone() {
                Adt2::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
                    h279
                }
            };
            let a281 = v1;
            let a282 = 0u64;
            nat_quot(a280, a281, a282)
        };
        let a285 = match v0.clone() {
            Adt2::C0(_, _, h283, _, _, _, _, _, _, _, _) => {
                h283
            }
        };
        nat_mul(a284, a285)?
    };
    let a291 = {
        let a288 = match v0.clone() {
            Adt2::C0(_, _, _, h286, _, _, _, _, _, _, _) => {
                h286
            }
        };
        let a289 = match v0 {
            Adt2::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                h287
            }
        };
        nat_mul(a288, a289)?
    };
    nat_mul(a290, a291)
}

pub fn f10(v0: u64) -> bool {
    let a292 = v0;
    let a293 = 4294967296u64;
    if nat_lt(a292, a293) {
        let a297 = {
            let a294 = v0;
            let a295 = 2147483648u64;
            let a296 = 0u64;
            nat_rem(a294, a295, a296)
        };
        let a298 = 2139095040u64;
        nat_lt(a297, a298)
    } else {
        false
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a299 = 4u64;
    let a300 = v0;
    nat_mul(a299, a300)
}

pub fn f12(v0: u64) -> R<u64> {
    let a301 = v0;
    let a302 = 0u64;
    if nat_eq(a301, a302) {
        Ok(0u64)
    } else {
        let a305 = 4u64;
        let a306 = {
            let a303 = 3u64;
            let a304 = v0;
            nat_mul(a303, a304)?
        };
        nat_add(a305, a306)
    }
}

pub fn f13(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a308 = match v0.clone() {
            Adt2::C0(_, h307, _, _, _, _, _, _, _, _, _) => {
                h307
            }
        };
        let a309 = v1;
        let a310 = 0u64;
        nat_quot(a308, a309, a310)
    }, match v0.clone() {
        Adt2::C0(_, _, h311, _, _, _, _, _, _, _, _) => {
            h311
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h312, _, _, _, _, _, _, _) => {
            h312
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h313, _, _, _, _, _, _) => {
            h313
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h314, _, _, _, _, _) => {
            h314
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h315, _, _, _, _) => {
            h315
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h316, _, _, _) => {
            h316
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h317, _, _) => {
            h317
        }
    })
}

pub fn f14(v0: Adt2, v1: Adt2) -> bool {
    let a320 = match v0.clone() {
        Adt2::C0(_, h318, _, _, _, _, _, _, _, _, _) => {
            h318
        }
    };
    let a321 = match v1.clone() {
        Adt2::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    };
    if if if nat_eq(a320, a321) {
        let a324 = match v0.clone() {
            Adt2::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                h322
            }
        };
        let a325 = match v1.clone() {
            Adt2::C0(_, _, h323, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        nat_eq(a324, a325)
    } else {
        false
    } {
        let a328 = match v0.clone() {
            Adt2::C0(_, _, _, h326, _, _, _, _, _, _, _) => {
                h326
            }
        };
        let a329 = match v1.clone() {
            Adt2::C0(_, _, _, h327, _, _, _, _, _, _, _) => {
                h327
            }
        };
        nat_eq(a328, a329)
    } else {
        false
    } {
        let a332 = match v0 {
            Adt2::C0(_, _, _, _, h330, _, _, _, _, _, _) => {
                h330
            }
        };
        let a333 = match v1 {
            Adt2::C0(_, _, _, _, h331, _, _, _, _, _, _) => {
                h331
            }
        };
        nat_eq(a332, a333)
    } else {
        false
    }
}

pub fn f15(v0: Adt2) -> R<bool> {
    let a335 = match v0.clone() {
        Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
            h334
        }
    };
    let a336 = 1u64;
    if if if if nat_eq(a335, a336) {
        true
    } else {
        let a338 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h337, _, _, _, _, _) => {
                h337
            }
        };
        let a339 = 4u64;
        nat_eq(a338, a339)
    } {
        let a341 = match v0.clone() {
            Adt2::C0(_, _, h340, _, _, _, _, _, _, _, _) => {
                h340
            }
        };
        let a342 = 1u64;
        if nat_eq(a341, a342) {
            true
        } else {
            let a347 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h343, _, _, _, _) => {
                    h343
                }
            };
            let a348 = {
                let a345 = match v0.clone() {
                    Adt2::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                        h344
                    }
                };
                let a346 = 4u64;
                nat_mul(a345, a346)?
            };
            nat_eq(a347, a348)
        }
    } else {
        false
    } {
        let a350 = match v0.clone() {
            Adt2::C0(_, _, _, h349, _, _, _, _, _, _, _) => {
                h349
            }
        };
        let a351 = 1u64;
        if nat_eq(a350, a351) {
            true
        } else {
            let a359 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h352, _, _, _) => {
                    h352
                }
            };
            let a360 = {
                let a357 = {
                    let a355 = match v0.clone() {
                        Adt2::C0(_, h353, _, _, _, _, _, _, _, _, _) => {
                            h353
                        }
                    };
                    let a356 = match v0.clone() {
                        Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                            h354
                        }
                    };
                    nat_mul(a355, a356)?
                };
                let a358 = 4u64;
                nat_mul(a357, a358)?
            };
            nat_eq(a359, a360)
        }
    } else {
        false
    } {
        let a362 = match v0.clone() {
            Adt2::C0(_, _, _, _, h361, _, _, _, _, _, _) => {
                h361
            }
        };
        let a363 = 1u64;
        if nat_eq(a362, a363) {
            Ok(true)
        } else {
            let a374 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h364, _, _) => {
                    h364
                }
            };
            let a375 = {
                let a372 = {
                    let a370 = {
                        let a367 = match v0.clone() {
                            Adt2::C0(_, h365, _, _, _, _, _, _, _, _, _) => {
                                h365
                            }
                        };
                        let a368 = match v0.clone() {
                            Adt2::C0(_, _, h366, _, _, _, _, _, _, _, _) => {
                                h366
                            }
                        };
                        nat_mul(a367, a368)?
                    };
                    let a371 = match v0 {
                        Adt2::C0(_, _, _, h369, _, _, _, _, _, _, _) => {
                            h369
                        }
                    };
                    nat_mul(a370, a371)?
                };
                let a373 = 4u64;
                nat_mul(a372, a373)?
            };
            Ok(nat_eq(a374, a375))
        }
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt2, v1: Adt2) -> R<bool> {
    let a377 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h376, _) => {
            h376
        }
    };
    let a378 = 0u64;
    if nat_eq(a377, a378) {
        Ok(true)
    } else {
        let a380 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h379, _) => {
                h379
            }
        };
        let a381 = 0u64;
        if nat_eq(a380, a381) {
            Ok(true)
        } else {
            let a383 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h382) => {
                    h382
                }
            };
            let a384 = 0u64;
            if nat_eq(a383, a384) {
                Ok(true)
            } else {
                let a386 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h385) => {
                        h385
                    }
                };
                let a387 = 0u64;
                if nat_eq(a386, a387) {
                    Ok(true)
                } else {
                    let a393 = {
                        let a390 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h388) => {
                                h388
                            }
                        };
                        let a391 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h389, _) => {
                                h389
                            }
                        };
                        nat_add(a390, a391)?
                    };
                    let a394 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h392) => {
                            h392
                        }
                    };
                    if nat_le(a393, a394) {
                        Ok(true)
                    } else {
                        let a400 = {
                            let a397 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h395) => {
                                    h395
                                }
                            };
                            let a398 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h396, _) => {
                                    h396
                                }
                            };
                            nat_add(a397, a398)?
                        };
                        let a401 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h399) => {
                                h399
                            }
                        };
                        Ok(nat_le(a400, a401))
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a406 = {
        let a402 = v0;
        let a403 = v1;
        nat_mul(a402, a403)?
    };
    let a407 = {
        let a404 = v2;
        let a405 = f59(v3);
        nat_mul(a404, a405)?
    };
    nat_mul(a406, a407)
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a410 = v0;
    let a411 = {
        let a408 = f60(v1)?;
        let a409 = v2;
        nat_mul(a408, a409)?
    };
    nat_add(a410, a411)
}

pub fn f19(v0: u64) -> u64 {
    let a412 = v0;
    let a413 = 30u64;
    if nat_eq(a412, a413) {
        2u64
    } else {
        4u64
    }
}

pub fn f20(v0: u64) -> bool {
    let a414 = v0;
    let a415 = 2139095040u64;
    if nat_le(a414, a415) {
        true
    } else {
        let a416 = v0;
        let a417 = 2147483648u64;
        nat_eq(a416, a417)
    }
}

pub fn f21(v0: Adt2, v1: u64) -> bool {
    let a419 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h418, _, _, _, _, _) => {
            h418
        }
    };
    let a420 = v1;
    if if nat_eq(a419, a420) {
        let a425 = {
            let a422 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h421) => {
                    h421
                }
            };
            let a423 = v1;
            let a424 = 0u64;
            nat_rem(a422, a423, a424)
        };
        let a426 = 0u64;
        nat_eq(a425, a426)
    } else {
        false
    } {
        f61(match v0.clone() {
            Adt2::C0(_, _, h427, _, _, _, _, _, _, _, _) => {
                h427
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h428, _, _, _, _, _, _, _) => {
                h428
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h429, _, _, _, _, _, _) => {
                h429
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h430, _, _, _, _) => {
                h430
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h431, _, _, _) => {
                h431
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h432, _, _) => {
                h432
            }
        }, v1)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a435 = match v0.clone() {
            Adt2::C0(h433, _, _, _, _, _, _, _, _, _, _) => {
                h433
            }
        };
        let a436 = match v1.clone() {
            Adt2::C0(h434, _, _, _, _, _, _, _, _, _, _) => {
                h434
            }
        };
        nat_eq(a435, a436)
    } else {
        false
    } {
        let a439 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h437) => {
                h437
            }
        };
        let a440 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h438) => {
                h438
            }
        };
        nat_eq(a439, a440)
    } else {
        false
    } {
        let a443 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h441, _, _, _, _, _) => {
                h441
            }
        };
        let a444 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h442, _, _, _, _, _) => {
                h442
            }
        };
        nat_eq(a443, a444)
    } else {
        false
    } {
        let a447 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h445, _, _, _, _) => {
                h445
            }
        };
        let a448 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h446, _, _, _, _) => {
                h446
            }
        };
        nat_eq(a447, a448)
    } else {
        false
    } {
        let a451 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h449, _, _, _) => {
                h449
            }
        };
        let a452 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h450, _, _, _) => {
                h450
            }
        };
        nat_eq(a451, a452)
    } else {
        false
    } {
        let a455 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h453, _, _) => {
                h453
            }
        };
        let a456 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h454, _, _) => {
                h454
            }
        };
        nat_eq(a455, a456)
    } else {
        false
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a457 = v0;
    let a458 = 0u64;
    if if if nat_eq(a457, a458) {
        true
    } else {
        let a459 = v1;
        let a460 = 0u64;
        nat_eq(a459, a460)
    } {
        true
    } else {
        let a461 = v2;
        let a462 = 0u64;
        nat_eq(a461, a462)
    } {
        true
    } else {
        let a463 = v3;
        let a464 = 0u64;
        nat_eq(a463, a464)
    }
}

pub fn f24(v0: Adt2, v1: u64) -> R<bool> {
    f62({
        let a466 = match v0.clone() {
            Adt2::C0(_, h465, _, _, _, _, _, _, _, _, _) => {
                h465
            }
        };
        let a467 = v1;
        nat_mul(a466, a467)?
    }, match v0.clone() {
        Adt2::C0(_, _, h468, _, _, _, _, _, _, _, _) => {
            h468
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h469, _, _, _, _, _, _, _) => {
            h469
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h470, _, _, _, _, _, _) => {
            h470
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h471, _, _, _, _) => {
            h471
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h472, _, _, _) => {
            h472
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h473, _, _) => {
            h473
        }
    })
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a474 = v1;
    let a475 = 0u64;
    if if nat_eq(a474, a475) {
        true
    } else {
        let a476 = v2;
        let a477 = 0u64;
        nat_eq(a476, a477)
    } {
        Ok(0u64)
    } else {
        let a484 = {
            let a478 = f63(v1, v2, v3)?;
            let a479 = f64(v0, v1, v2)?;
            nat_add(a478, a479)?
        };
        let a485 = {
            let a482 = f65(v0);
            let a483 = {
                let a480 = v1;
                let a481 = v2;
                nat_mul(a480, a481)?
            };
            nat_mul(a482, a483)?
        };
        nat_add(a484, a485)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a486 = v0;
    let a487 = 0u64;
    if if nat_eq(a486, a487) {
        true
    } else {
        let a488 = v1;
        let a489 = 0u64;
        nat_eq(a488, a489)
    } {
        Ok(3u64)
    } else {
        let a492 = {
            let a490 = 3u64;
            let a491 = v1;
            nat_add(a490, a491)?
        };
        let a493 = v2;
        nat_add(a492, a493)
    }
}

pub fn f27(v0: u64) -> bool {
    let a494 = v0;
    let a495 = 2139095040u64;
    if nat_le(a494, a495) {
        true
    } else {
        let a496 = v0;
        let a497 = 2147483648u64;
        nat_eq(a496, a497)
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a498 = v0;
    let a499 = 0u64;
    if nat_eq(a498, a499) {
        Ok(0u64)
    } else {
        let a500 = v1;
        let a501 = f66(v0)?;
        nat_mul(a500, a501)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a502 = v0;
    let a503 = 0u64;
    if if if if nat_eq(a502, a503) {
        true
    } else {
        let a504 = v1;
        let a505 = 0u64;
        nat_eq(a504, a505)
    } {
        true
    } else {
        let a506 = v2;
        let a507 = 0u64;
        nat_eq(a506, a507)
    } {
        true
    } else {
        let a508 = v3;
        let a509 = 0u64;
        nat_eq(a508, a509)
    } {
        Ok(0u64)
    } else {
        let a522 = {
            let a512 = 2u64;
            let a513 = {
                let a510 = v1;
                let a511 = v2;
                nat_mul(a510, a511)?
            };
            nat_mul(a512, a513)?
        };
        let a523 = {
            let a520 = {
                let a516 = {
                    let a514 = v1;
                    let a515 = v2;
                    nat_mul(a514, a515)?
                };
                let a517 = v3;
                nat_mul(a516, a517)?
            };
            let a521 = {
                let a518 = 1u64;
                let a519 = v0;
                nat_add(a518, a519)?
            };
            nat_mul(a520, a521)?
        };
        nat_add(a522, a523)
    }
}

pub fn f30(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m524 = v1;
    match m524 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f67(v0, v3, v2)
        }
    }
}

pub fn f31(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a526 = match v0.clone() {
        Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
            h525
        }
    };
    let a527 = 1u64;
    if if nat_eq(a526, a527) {
        true
    } else {
        let a529 = match v0.clone() {
            Adt2::C0(h528, _, _, _, _, _, _, _, _, _, _) => {
                h528
            }
        };
        let a530 = 30u64;
        nat_eq(a529, a530)
    } {
        let a532 = match v1.clone() {
            Adt2::C0(h531, _, _, _, _, _, _, _, _, _, _) => {
                h531
            }
        };
        let a533 = 1u64;
        if if if if if if if if if if if nat_eq(a532, a533) {
            let a536 = match v0.clone() {
                Adt2::C0(_, h534, _, _, _, _, _, _, _, _, _) => {
                    h534
                }
            };
            let a537 = match v1.clone() {
                Adt2::C0(_, h535, _, _, _, _, _, _, _, _, _) => {
                    h535
                }
            };
            nat_eq(a536, a537)
        } else {
            false
        } {
            let a539 = 0u64;
            let a540 = match v1.clone() {
                Adt2::C0(_, _, h538, _, _, _, _, _, _, _, _) => {
                    h538
                }
            };
            nat_lt(a539, a540)
        } else {
            false
        } {
            let a543 = match v1.clone() {
                Adt2::C0(_, _, h541, _, _, _, _, _, _, _, _) => {
                    h541
                }
            };
            let a544 = match v1.clone() {
                Adt2::C0(_, _, _, h542, _, _, _, _, _, _, _) => {
                    h542
                }
            };
            nat_eq(a543, a544)
        } else {
            false
        } {
            let a546 = match v1.clone() {
                Adt2::C0(_, _, _, _, h545, _, _, _, _, _, _) => {
                    h545
                }
            };
            let a547 = 1u64;
            nat_eq(a546, a547)
        } else {
            false
        } {
            let a555 = match v0.clone() {
                Adt2::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                    h548
                }
            };
            let a556 = {
                let a553 = {
                    let a551 = match v1.clone() {
                        Adt2::C0(_, _, h549, _, _, _, _, _, _, _, _) => {
                            h549
                        }
                    };
                    let a552 = match v1.clone() {
                        Adt2::C0(_, _, h550, _, _, _, _, _, _, _, _) => {
                            h550
                        }
                    };
                    nat_add(a551, a552)?
                };
                let a554 = 1u64;
                nat_sub(a553, a554)
            };
            nat_eq(a555, a556)
        } else {
            false
        } {
            let a558 = 0u64;
            let a559 = match v0.clone() {
                Adt2::C0(_, _, _, h557, _, _, _, _, _, _, _) => {
                    h557
                }
            };
            nat_lt(a558, a559)
        } else {
            false
        } {
            let a561 = 0u64;
            let a562 = match v0.clone() {
                Adt2::C0(_, _, _, _, h560, _, _, _, _, _, _) => {
                    h560
                }
            };
            nat_lt(a561, a562)
        } else {
            false
        } {
            let a564 = 2u64;
            let a565 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h563, _, _, _, _, _) => {
                    h563
                }
            };
            nat_le(a564, a565)
        } else {
            false
        } {
            let a567 = 2u64;
            let a568 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h566, _, _, _, _, _) => {
                    h566
                }
            };
            nat_le(a567, a568)
        } else {
            false
        } {
            f16(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f9(v1.clone(), 1u64)?, 2u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f32(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m569 = v1;
    match m569 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a571 = match v0.clone() {
                Adt2::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                    h570
                }
            };
            let a572 = 0u64;
            if if if if if if if if if if if if if nat_eq(a571, a572) {
                true
            } else {
                let a574 = match v0.clone() {
                    Adt2::C0(h573, _, _, _, _, _, _, _, _, _, _) => {
                        h573
                    }
                };
                let a575 = 1u64;
                nat_eq(a574, a575)
            } {
                let a577 = match v3.clone() {
                    Adt2::C0(h576, _, _, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a578 = 26u64;
                nat_eq(a577, a578)
            } else {
                false
            } {
                let a580 = match v2.clone() {
                    Adt2::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                        h579
                    }
                };
                let a581 = 0u64;
                nat_eq(a580, a581)
            } else {
                false
            } {
                let a584 = match v0.clone() {
                    Adt2::C0(_, h582, _, _, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                let a585 = match v2.clone() {
                    Adt2::C0(_, h583, _, _, _, _, _, _, _, _, _) => {
                        h583
                    }
                };
                nat_eq(a584, a585)
            } else {
                false
            } {
                let a588 = match v3.clone() {
                    Adt2::C0(_, h586, _, _, _, _, _, _, _, _, _) => {
                        h586
                    }
                };
                let a589 = match v0.clone() {
                    Adt2::C0(_, _, h587, _, _, _, _, _, _, _, _) => {
                        h587
                    }
                };
                nat_le(a588, a589)
            } else {
                false
            } {
                let a591 = match v0.clone() {
                    Adt2::C0(_, _, _, h590, _, _, _, _, _, _, _) => {
                        h590
                    }
                };
                let a592 = 1u64;
                nat_eq(a591, a592)
            } else {
                false
            } {
                let a594 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h593, _, _, _, _, _, _) => {
                        h593
                    }
                };
                let a595 = 1u64;
                nat_eq(a594, a595)
            } else {
                false
            } {
                let a597 = match v2.clone() {
                    Adt2::C0(_, _, _, h596, _, _, _, _, _, _, _) => {
                        h596
                    }
                };
                let a598 = 1u64;
                nat_eq(a597, a598)
            } else {
                false
            } {
                let a600 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
                        h599
                    }
                };
                let a601 = 1u64;
                nat_eq(a600, a601)
            } else {
                false
            } {
                let a603 = match v3.clone() {
                    Adt2::C0(_, _, h602, _, _, _, _, _, _, _, _) => {
                        h602
                    }
                };
                let a604 = 1u64;
                nat_eq(a603, a604)
            } else {
                false
            } {
                let a606 = match v3.clone() {
                    Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                        h605
                    }
                };
                let a607 = 1u64;
                nat_eq(a606, a607)
            } else {
                false
            } {
                let a609 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h608, _, _, _, _, _, _) => {
                        h608
                    }
                };
                let a610 = 1u64;
                nat_eq(a609, a610)
            } else {
                false
            } {
                if f23(match v2.clone() {
                    Adt2::C0(_, h611, _, _, _, _, _, _, _, _, _) => {
                        h611
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h612, _, _, _, _, _, _, _, _) => {
                        h612
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h613, _, _, _, _, _, _, _) => {
                        h613
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h614, _, _, _, _, _, _) => {
                        h614
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a616 = match v3.clone() {
                        Adt2::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                            h615
                        }
                    };
                    let a617 = 0u64;
                    if nat_eq(a616, a617) {
                        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a619 = match v0.clone() {
                            Adt2::C0(_, h618, _, _, _, _, _, _, _, _, _) => {
                                h618
                            }
                        };
                        let a620 = 2147483647u64;
                        if if if if if if nat_le(a619, a620) {
                            let a622 = match v3.clone() {
                                Adt2::C0(_, h621, _, _, _, _, _, _, _, _, _) => {
                                    h621
                                }
                            };
                            let a623 = 2147483647u64;
                            nat_le(a622, a623)
                        } else {
                            false
                        } {
                            let a625 = 4u64;
                            let a626 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h624, _, _, _, _, _) => {
                                    h624
                                }
                            };
                            nat_le(a625, a626)
                        } else {
                            false
                        } {
                            let a632 = {
                                let a629 = match v2.clone() {
                                    Adt2::C0(_, h627, _, _, _, _, _, _, _, _, _) => {
                                        h627
                                    }
                                };
                                let a630 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h628, _, _, _, _, _) => {
                                        h628
                                    }
                                };
                                nat_mul(a629, a630)?
                            };
                            let a633 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h631, _, _, _, _) => {
                                    h631
                                }
                            };
                            nat_le(a632, a633)
                        } else {
                            false
                        } {
                            f16(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f16(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a647 = {
                                    let a637 = {
                                        let a635 = 2u64;
                                        let a636 = match v3.clone() {
                                            Adt2::C0(_, h634, _, _, _, _, _, _, _, _, _) => {
                                                h634
                                            }
                                        };
                                        nat_mul(a635, a636)?
                                    };
                                    let a638 = f9(v2.clone(), 1u64)?;
                                    nat_add(a637, a638)?
                                };
                                let a648 = {
                                    let a645 = match v3.clone() {
                                        Adt2::C0(_, h639, _, _, _, _, _, _, _, _, _) => {
                                            h639
                                        }
                                    };
                                    let a646 = {
                                        let a643 = 1u64;
                                        let a644 = {
                                            let a641 = 2u64;
                                            let a642 = match v0.clone() {
                                                Adt2::C0(_, h640, _, _, _, _, _, _, _, _, _) => {
                                                    h640
                                                }
                                            };
                                            nat_mul(a641, a642)?
                                        };
                                        nat_add(a643, a644)?
                                    };
                                    nat_mul(a645, a646)?
                                };
                                nat_add(a647, a648)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h649, _, _, _, _, _, _, _, _, _) => {
                                    h649
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h650, _, _, _, _, _, _, _, _, _) => {
                                    h650
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h651, _, _, _, _, _) => {
                                    h651
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h652, _, _, _, _) => {
                                    h652
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h653, _, _, _) => {
                                    h653
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h654, _, _) => {
                                    h654
                                }
                            }), f13(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
                                    h655
                                }
                            }, f13(v3, 1u64))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f33(v0: u64) -> bool {
    let a656 = v0;
    let a657 = 1u64;
    if if nat_eq(a656, a657) {
        true
    } else {
        let a658 = v0;
        let a659 = 33u64;
        nat_eq(a658, a659)
    } {
        true
    } else {
        let a660 = v0;
        let a661 = 34u64;
        nat_eq(a660, a661)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a664 = match v1.clone() {
        Adt2::C0(h662, _, _, _, _, _, _, _, _, _, _) => {
            h662
        }
    };
    let a665 = match v3.clone() {
        Adt2::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
            h663
        }
    };
    if nat_eq(a664, a665) {
        f68(v0, v1, v2, v3)
    } else {
        if if if if f69(match v1.clone() {
            Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                h666
            }
        }) {
            f69(match v3.clone() {
                Adt2::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
                    h667
                }
            })
        } else {
            false
        } {
            let a668 = v0;
            let a669 = 33u64;
            if nat_eq(a668, a669) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a670 = f9(v1.clone(), 1u64)?;
            let a671 = f9(v3.clone(), 1u64)?;
            nat_eq(a670, a671)
        } else {
            false
        } {
            let m673 = f70(match v3.clone() {
                Adt2::C0(h672, _, _, _, _, _, _, _, _, _, _) => {
                    h672
                }
            });
            match m673 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h674) => {
                            h674
                        }
                    }, f13(v1.clone(), 1u64), f13(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h675, _, _, _, _, _, _, _, _, _, _) => {
                            h675
                        }
                    }, match v3 {
                        Adt2::C0(h676, _, _, _, _, _, _, _, _, _, _) => {
                            h676
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f35(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a678 = match v0.clone() {
        Adt2::C0(h677, _, _, _, _, _, _, _, _, _, _) => {
            h677
        }
    };
    let a679 = 0u64;
    if if if if nat_eq(a678, a679) {
        true
    } else {
        let a681 = match v0.clone() {
            Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                h680
            }
        };
        let a682 = 1u64;
        nat_eq(a681, a682)
    } {
        true
    } else {
        let a684 = match v0.clone() {
            Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                h683
            }
        };
        let a685 = 25u64;
        if nat_eq(a684, a685) {
            true
        } else {
            let a687 = match v0.clone() {
                Adt2::C0(h686, _, _, _, _, _, _, _, _, _, _) => {
                    h686
                }
            };
            let a688 = 26u64;
            nat_eq(a687, a688)
        }
    } {
        true
    } else {
        let a690 = match v0.clone() {
            Adt2::C0(h689, _, _, _, _, _, _, _, _, _, _) => {
                h689
            }
        };
        let a691 = 30u64;
        nat_eq(a690, a691)
    } {
        let a694 = match v0.clone() {
            Adt2::C0(h692, _, _, _, _, _, _, _, _, _, _) => {
                h692
            }
        };
        let a695 = match v1.clone() {
            Adt2::C0(h693, _, _, _, _, _, _, _, _, _, _) => {
                h693
            }
        };
        if if nat_eq(a694, a695) {
            f71(match v1.clone() {
                Adt2::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
                    h696
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
                    h697
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h698, _, _, _, _, _, _, _) => {
                    h698
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h699, _, _, _, _, _, _) => {
                    h699
                }
            }, match v0.clone() {
                Adt2::C0(_, h700, _, _, _, _, _, _, _, _, _) => {
                    h700
                }
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
            })
        } else {
            false
        } {
            let m705 = f70(match v0.clone() {
                Adt2::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                    h704
                }
            });
            match m705 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f9(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h706) => {
                            h706
                        }
                    }, f13(v0, 1u64), f13(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f36(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m707 = v2;
    match m707 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a708 = v0;
            let a709 = 4u64;
            if if if nat_lt(a708, a709) {
                let a712 = match v1.clone() {
                    Adt2::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
                        h710
                    }
                };
                let a713 = match v4.clone() {
                    Adt2::C0(h711, _, _, _, _, _, _, _, _, _, _) => {
                        h711
                    }
                };
                if nat_eq(a712, a713) {
                    let a716 = match v1.clone() {
                        Adt2::C0(h714, _, _, _, _, _, _, _, _, _, _) => {
                            h714
                        }
                    };
                    let a717 = match v3.clone() {
                        Adt2::C0(h715, _, _, _, _, _, _, _, _, _, _) => {
                            h715
                        }
                    };
                    nat_eq(a716, a717)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f72(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h718, _, _, _, _, _, _, _, _, _) => {
                        h718
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h719, _, _, _, _, _, _, _, _, _) => {
                        h719
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h720, _, _, _, _, _, _, _, _, _) => {
                        h720
                    }
                })? {
                    f72(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h721, _, _, _, _, _, _, _, _) => {
                            h721
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h722, _, _, _, _, _, _, _, _) => {
                            h722
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h723, _, _, _, _, _, _, _, _) => {
                            h723
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
                            h724
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
                            h725
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                            h726
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h727, _, _, _, _, _, _) => {
                            h727
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h728, _, _, _, _, _, _) => {
                            h728
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                            h729
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m731 = f70(match v1.clone() {
                    Adt2::C0(h730, _, _, _, _, _, _, _, _, _, _) => {
                        h730
                    }
                });
                match m731 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h732, _) => {
                                h732
                            }
                        };
                        let a737 = {
                            let a734 = match v1.clone() {
                                Adt2::C0(_, h733, _, _, _, _, _, _, _, _, _) => {
                                    h733
                                }
                            };
                            let a735 = v6;
                            let a736 = 1u64;
                            nat_rem(a734, a735, a736)
                        };
                        let a738 = 0u64;
                        if if if nat_eq(a737, a738) {
                            let a743 = {
                                let a740 = match v4.clone() {
                                    Adt2::C0(_, h739, _, _, _, _, _, _, _, _, _) => {
                                        h739
                                    }
                                };
                                let a741 = v6;
                                let a742 = 1u64;
                                nat_rem(a740, a741, a742)
                            };
                            let a744 = 0u64;
                            nat_eq(a743, a744)
                        } else {
                            false
                        } {
                            let a749 = {
                                let a746 = match v3.clone() {
                                    Adt2::C0(_, h745, _, _, _, _, _, _, _, _, _) => {
                                        h745
                                    }
                                };
                                let a747 = v6;
                                let a748 = 1u64;
                                nat_rem(a746, a747, a748)
                            };
                            let a750 = 0u64;
                            nat_eq(a749, a750)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f9(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h751) => {
                                    h751
                                }
                            }, f13(v1, v6), f13(v3, v6), Adt4::C2(v0, f13(v4, v6))))
                        } else {
                            Ok(Adt6::C1)
                        }
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f37(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a753 = match v2.clone() {
        Adt2::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
            h752
        }
    };
    let a754 = 0u64;
    if if nat_eq(a753, a754) {
        let a756 = match v3.clone() {
            Adt2::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                h755
            }
        };
        let a757 = 0u64;
        nat_eq(a756, a757)
    } else {
        false
    } {
        let a758 = v0;
        let a759 = 61u64;
        if nat_eq(a758, a759) {
            let v4: bool = {
                let a761 = 0u64;
                let a762 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h760, _, _, _, _, _, _, _) => {
                        h760
                    }
                };
                nat_lt(a761, a762)
            };
            if if if if if if f73(match v2.clone() {
                Adt2::C0(_, h763, _, _, _, _, _, _, _, _, _) => {
                    h763
                }
            }, match v1.clone() {
                Adt1::C0(h764, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h764
                }
            }, match v1.clone() {
                Adt1::C0(_, h765, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h765
                }
            }, match v3.clone() {
                Adt2::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                    h766
                }
            })? {
                f73(match v2.clone() {
                    Adt2::C0(_, _, h767, _, _, _, _, _, _, _, _) => {
                        h767
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h768, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h768
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h769, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h769
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                        h770
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, h771, _, _, _, _, _, _, _) => {
                        h771
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h772, _, _, _, _, _, _, _, _, _, _, _) => {
                        h772
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h773, _, _, _, _, _, _, _, _, _, _) => {
                        h773
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h774, _, _, _, _, _, _, _) => {
                        h774
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, _, h775, _, _, _, _, _, _) => {
                        h775
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h776, _, _, _, _, _, _, _, _, _) => {
                        h776
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h777, _, _, _, _, _, _, _, _) => {
                        h777
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h778, _, _, _, _, _, _) => {
                        h778
                    }
                })?
            } else {
                false
            } {
                let a780 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h779, _, _, _, _, _, _, _) => {
                        h779
                    }
                };
                let a781 = 4294967295u64;
                nat_le(a780, a781)
            } else {
                false
            } {
                if v4 {
                    !f23(match v2.clone() {
                        Adt2::C0(_, h782, _, _, _, _, _, _, _, _, _) => {
                            h782
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h783, _, _, _, _, _, _, _, _) => {
                            h783
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h784, _, _, _, _, _, _, _) => {
                            h784
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h785, _, _, _, _, _, _) => {
                            h785
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h786, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h786
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h787, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h787
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h788, _, _, _, _, _, _, _, _, _, _, _) => {
                        h788
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h789, _, _, _, _, _, _, _, _, _) => {
                        h789
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a790 = v0;
            let a791 = 62u64;
            if nat_eq(a790, a791) {
                let a793 = match v1.clone() {
                    Adt1::C0(h792, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h792
                    }
                };
                let a794 = 2147483648u64;
                if if if if if nat_lt(a793, a794) {
                    let a796 = match v1.clone() {
                        Adt1::C0(_, h795, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h795
                        }
                    };
                    let a797 = 2147483648u64;
                    nat_lt(a796, a797)
                } else {
                    false
                } {
                    let a800 = match v1.clone() {
                        Adt1::C0(h798, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h798
                        }
                    };
                    let a801 = match v2.clone() {
                        Adt2::C0(_, h799, _, _, _, _, _, _, _, _, _) => {
                            h799
                        }
                    };
                    nat_lt(a800, a801)
                } else {
                    false
                } {
                    let a804 = match v1.clone() {
                        Adt1::C0(_, h802, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h802
                        }
                    };
                    let a805 = match v2.clone() {
                        Adt2::C0(_, h803, _, _, _, _, _, _, _, _, _) => {
                            h803
                        }
                    };
                    nat_lt(a804, a805)
                } else {
                    false
                } {
                    let a808 = match v2.clone() {
                        Adt2::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                            h806
                        }
                    };
                    let a809 = match v3.clone() {
                        Adt2::C0(_, _, h807, _, _, _, _, _, _, _, _) => {
                            h807
                        }
                    };
                    if if if nat_eq(a808, a809) {
                        let a812 = match v2.clone() {
                            Adt2::C0(_, _, _, h810, _, _, _, _, _, _, _) => {
                                h810
                            }
                        };
                        let a813 = match v3.clone() {
                            Adt2::C0(_, _, _, h811, _, _, _, _, _, _, _) => {
                                h811
                            }
                        };
                        nat_eq(a812, a813)
                    } else {
                        false
                    } {
                        let a816 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h814, _, _, _, _, _, _) => {
                                h814
                            }
                        };
                        let a817 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h815, _, _, _, _, _, _) => {
                                h815
                            }
                        };
                        nat_eq(a816, a817)
                    } else {
                        false
                    } {
                        let a826 = match v3.clone() {
                            Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                                h818
                            }
                        };
                        let a827 = {
                            let a824 = {
                                let a821 = match v2.clone() {
                                    Adt2::C0(_, h819, _, _, _, _, _, _, _, _, _) => {
                                        h819
                                    }
                                };
                                let a822 = match v1.clone() {
                                    Adt1::C0(h820, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h820
                                    }
                                };
                                nat_add(a821, a822)?
                            };
                            let a825 = match v1.clone() {
                                Adt1::C0(_, h823, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h823
                                }
                            };
                            nat_add(a824, a825)?
                        };
                        nat_eq(a826, a827)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h828, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h828
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a829 = v0;
                let a830 = 63u64;
                if nat_eq(a829, a830) {
                    if if if if if f74(match v2.clone() {
                        Adt2::C0(_, h831, _, _, _, _, _, _, _, _, _) => {
                            h831
                        }
                    }, match v1.clone() {
                        Adt1::C0(h832, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h832
                        }
                    })? {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, h833, _, _, _, _, _, _, _, _) => {
                                h833
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h834, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h834
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
                                h835
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h836, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h836
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, _, h837, _, _, _, _, _, _) => {
                                h837
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h838, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h838
                            }
                        })?
                    } else {
                        false
                    } {
                        let a841 = match v2.clone() {
                            Adt2::C0(_, h839, _, _, _, _, _, _, _, _, _) => {
                                h839
                            }
                        };
                        let a842 = match v3.clone() {
                            Adt2::C0(_, h840, _, _, _, _, _, _, _, _, _) => {
                                h840
                            }
                        };
                        if if if nat_eq(a841, a842) {
                            let a845 = match v2.clone() {
                                Adt2::C0(_, _, h843, _, _, _, _, _, _, _, _) => {
                                    h843
                                }
                            };
                            let a846 = match v3.clone() {
                                Adt2::C0(_, _, h844, _, _, _, _, _, _, _, _) => {
                                    h844
                                }
                            };
                            nat_eq(a845, a846)
                        } else {
                            false
                        } {
                            let a849 = match v2.clone() {
                                Adt2::C0(_, _, _, h847, _, _, _, _, _, _, _) => {
                                    h847
                                }
                            };
                            let a850 = match v3.clone() {
                                Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                                    h848
                                }
                            };
                            nat_eq(a849, a850)
                        } else {
                            false
                        } {
                            let a853 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h851, _, _, _, _, _, _) => {
                                    h851
                                }
                            };
                            let a854 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h852, _, _, _, _, _, _) => {
                                    h852
                                }
                            };
                            nat_eq(a853, a854)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h855, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h855
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h856, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h856
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h857, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h857
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h858, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h858
                            }
                        })))
                    } else {
                        Ok(Adt6::C1)
                    }
                } else {
                    Ok(Adt6::C0)
                }
            }
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f38(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m859 = v3;
    match m859 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a860 = v0;
                let a861 = 4u64;
                nat_eq(a860, a861)
            };
            let v7: bool = {
                let a863 = 0u64;
                let a864 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h862, _, _, _, _, _, _, _, _, _, _, _) => {
                        h862
                    }
                };
                nat_lt(a863, a864)
            };
            let v8: u64 = f9(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h865, _, _, _, _, _, _, _, _, _) => {
                    h865
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h866, _, _, _, _, _, _, _, _) => {
                    h866
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h867, _, _, _, _, _, _, _) => {
                    h867
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h868, _, _, _, _, _, _) => {
                    h868
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h869, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h869
                }
            }, match v1.clone() {
                Adt1::C0(_, h870, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h870
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h871, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h871
                }
            });
            if if if if if if if if if if if if v6 {
                let a873 = match v2.clone() {
                    Adt2::C0(h872, _, _, _, _, _, _, _, _, _, _) => {
                        h872
                    }
                };
                let a874 = 0u64;
                if nat_eq(a873, a874) {
                    let a876 = match v5.clone() {
                        Adt2::C0(h875, _, _, _, _, _, _, _, _, _, _) => {
                            h875
                        }
                    };
                    let a877 = 0u64;
                    nat_eq(a876, a877)
                } else {
                    false
                }
            } else {
                let a879 = match v2.clone() {
                    Adt2::C0(h878, _, _, _, _, _, _, _, _, _, _) => {
                        h878
                    }
                };
                let a880 = 0u64;
                if if nat_eq(a879, a880) {
                    true
                } else {
                    let a882 = match v2.clone() {
                        Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                            h881
                        }
                    };
                    let a883 = 26u64;
                    nat_eq(a882, a883)
                } {
                    let a885 = match v5.clone() {
                        Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                            h884
                        }
                    };
                    let a886 = 0u64;
                    if nat_eq(a885, a886) {
                        true
                    } else {
                        let a888 = match v5.clone() {
                            Adt2::C0(h887, _, _, _, _, _, _, _, _, _, _) => {
                                h887
                            }
                        };
                        let a889 = 26u64;
                        nat_eq(a888, a889)
                    }
                } else {
                    false
                }
            } {
                let a892 = match v2.clone() {
                    Adt2::C0(h890, _, _, _, _, _, _, _, _, _, _) => {
                        h890
                    }
                };
                let a893 = match v4.clone() {
                    Adt2::C0(h891, _, _, _, _, _, _, _, _, _, _) => {
                        h891
                    }
                };
                nat_eq(a892, a893)
            } else {
                false
            } {
                f14(v2.clone(), v4.clone())
            } else {
                false
            } {
                f15(v2.clone())?
            } else {
                false
            } {
                f15(v4.clone())?
            } else {
                false
            } {
                let a895 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h894, _, _, _, _, _) => {
                        h894
                    }
                };
                let a896 = 4u64;
                nat_eq(a895, a896)
            } else {
                false
            } {
                let a897 = v8;
                let a898 = f9(v2.clone(), 1u64)?;
                nat_le(a897, a898)
            } else {
                false
            } {
                let a900 = match v1.clone() {
                    Adt1::C0(_, _, _, h899, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h899
                    }
                };
                let a901 = 1073741824u64;
                nat_lt(a900, a901)
            } else {
                false
            } {
                let a903 = match v1.clone() {
                    Adt1::C0(h902, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h902
                    }
                };
                let a904 = 4294967295u64;
                if if if if nat_le(a903, a904) {
                    let a906 = match v1.clone() {
                        Adt1::C0(_, h905, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h905
                        }
                    };
                    let a907 = 4294967295u64;
                    nat_le(a906, a907)
                } else {
                    false
                } {
                    let a909 = match v1.clone() {
                        Adt1::C0(_, _, h908, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h908
                        }
                    };
                    let a910 = 4294967295u64;
                    nat_le(a909, a910)
                } else {
                    false
                } {
                    let a912 = match v1.clone() {
                        Adt1::C0(_, _, _, h911, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h911
                        }
                    };
                    let a913 = 4294967295u64;
                    nat_le(a912, a913)
                } else {
                    false
                } {
                    let a915 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h914, _, _, _, _, _, _, _, _, _, _, _) => {
                            h914
                        }
                    };
                    let a916 = 4294967295u64;
                    nat_le(a915, a916)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a919 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h917) => {
                            h917
                        }
                    };
                    let a920 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h918) => {
                            h918
                        }
                    };
                    nat_eq(a919, a920)
                } else {
                    true
                }
            } else {
                false
            } {
                f75(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h921, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h921
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h922, _) => {
                        h922
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f9(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a923 = v10;
                    let a924 = v8;
                    nat_add(a923, a924)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h925, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h925
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a927 = match v0.clone() {
        Adt2::C0(h926, _, _, _, _, _, _, _, _, _, _) => {
            h926
        }
    };
    let a928 = 0u64;
    if if if nat_eq(a927, a928) {
        let a930 = match v1.clone() {
            Adt2::C0(h929, _, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        };
        let a931 = 0u64;
        nat_eq(a930, a931)
    } else {
        false
    } {
        f71(match v0.clone() {
            Adt2::C0(_, h932, _, _, _, _, _, _, _, _, _) => {
                h932
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                h933
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h934, _, _, _, _, _, _, _) => {
                h934
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h935, _, _, _, _, _, _) => {
                h935
            }
        }, match v1.clone() {
            Adt2::C0(_, h936, _, _, _, _, _, _, _, _, _) => {
                h936
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h937, _, _, _, _, _, _, _, _) => {
                h937
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h938, _, _, _, _, _, _, _) => {
                h938
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h939, _, _, _, _, _, _) => {
                h939
            }
        })
    } else {
        false
    } {
        let v2: u64 = f9(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a940 = v2;
            let a941 = f9(v0.clone(), 1u64)?;
            nat_add(a940, a941)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a943 = match v1.clone() {
        Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
            h942
        }
    };
    let a944 = 0u64;
    if if if nat_eq(a943, a944) {
        true
    } else {
        let a946 = match v1.clone() {
            Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a947 = 1u64;
        nat_eq(a946, a947)
    } {
        true
    } else {
        let a949 = match v1.clone() {
            Adt2::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a950 = 30u64;
        nat_eq(a949, a950)
    } {
        let a953 = match v1.clone() {
            Adt2::C0(h951, _, _, _, _, _, _, _, _, _, _) => {
                h951
            }
        };
        let a954 = match v2.clone() {
            Adt2::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                h952
            }
        };
        if if if if if nat_eq(a953, a954) {
            let a957 = match v1.clone() {
                Adt2::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
                    h955
                }
            };
            let a958 = match v2.clone() {
                Adt2::C0(_, h956, _, _, _, _, _, _, _, _, _) => {
                    h956
                }
            };
            nat_eq(a957, a958)
        } else {
            false
        } {
            let a961 = match v1.clone() {
                Adt2::C0(_, _, _, h959, _, _, _, _, _, _, _) => {
                    h959
                }
            };
            let a962 = match v2.clone() {
                Adt2::C0(_, _, _, h960, _, _, _, _, _, _, _) => {
                    h960
                }
            };
            nat_eq(a961, a962)
        } else {
            false
        } {
            let a965 = match v1.clone() {
                Adt2::C0(_, _, _, _, h963, _, _, _, _, _, _) => {
                    h963
                }
            };
            let a966 = match v2.clone() {
                Adt2::C0(_, _, _, _, h964, _, _, _, _, _, _) => {
                    h964
                }
            };
            nat_eq(a965, a966)
        } else {
            false
        } {
            let a967 = v0;
            let a968 = 4u64;
            if nat_eq(a967, a968) {
                let a970 = match v1.clone() {
                    Adt2::C0(_, _, h969, _, _, _, _, _, _, _, _) => {
                        h969
                    }
                };
                let a971 = 1u64;
                if nat_eq(a970, a971) {
                    let a974 = match v1.clone() {
                        Adt2::C0(_, h972, _, _, _, _, _, _, _, _, _) => {
                            h972
                        }
                    };
                    let a975 = match v2.clone() {
                        Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                            h973
                        }
                    };
                    nat_eq(a974, a975)
                } else {
                    false
                }
            } else {
                let a976 = v0;
                let a977 = 4u64;
                if if nat_lt(a976, a977) {
                    let a980 = match v1.clone() {
                        Adt2::C0(_, h978, _, _, _, _, _, _, _, _, _) => {
                            h978
                        }
                    };
                    let a981 = match v1.clone() {
                        Adt2::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
                            h979
                        }
                    };
                    nat_eq(a980, a981)
                } else {
                    false
                } {
                    let a984 = match v1.clone() {
                        Adt2::C0(_, _, h982, _, _, _, _, _, _, _, _) => {
                            h982
                        }
                    };
                    let a985 = match v2.clone() {
                        Adt2::C0(_, _, h983, _, _, _, _, _, _, _, _) => {
                            h983
                        }
                    };
                    nat_eq(a984, a985)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m987 = f70(match v1.clone() {
                Adt2::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
                    h986
                }
            });
            match m987 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h988) => {
                            h988
                        }
                    }, f13(v1, 1u64), f13(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f76(match v1.clone() {
        Adt2::C0(h989, _, _, _, _, _, _, _, _, _, _) => {
            h989
        }
    }, v0) {
        let a992 = match v1.clone() {
            Adt2::C0(h990, _, _, _, _, _, _, _, _, _, _) => {
                h990
            }
        };
        let a993 = match v2.clone() {
            Adt2::C0(h991, _, _, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        nat_eq(a992, a993)
    } else {
        false
    } {
        let a996 = match v1.clone() {
            Adt2::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a997 = match v2.clone() {
            Adt2::C0(_, h995, _, _, _, _, _, _, _, _, _) => {
                h995
            }
        };
        if if if nat_eq(a996, a997) {
            let a1000 = match v1.clone() {
                Adt2::C0(_, _, h998, _, _, _, _, _, _, _, _) => {
                    h998
                }
            };
            let a1001 = match v2.clone() {
                Adt2::C0(_, _, h999, _, _, _, _, _, _, _, _) => {
                    h999
                }
            };
            nat_eq(a1000, a1001)
        } else {
            false
        } {
            let a1004 = match v1.clone() {
                Adt2::C0(_, _, _, h1002, _, _, _, _, _, _, _) => {
                    h1002
                }
            };
            let a1005 = match v2.clone() {
                Adt2::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
                    h1003
                }
            };
            nat_eq(a1004, a1005)
        } else {
            false
        } {
            let a1008 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1006, _, _, _, _, _, _) => {
                    h1006
                }
            };
            let a1009 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1007, _, _, _, _, _, _) => {
                    h1007
                }
            };
            nat_eq(a1008, a1009)
        } else {
            false
        }
    } else {
        false
    } {
        let m1011 = f70(match v1.clone() {
            Adt2::C0(h1010, _, _, _, _, _, _, _, _, _, _) => {
                h1010
            }
        });
        match m1011 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1012) => {
                        h1012
                    }
                }, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1013, _, _, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f42(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1014, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1015, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        }
    };
    let a1016 = 0u64;
    let a1017 = v4;
    if if nat_lt(a1016, a1017) {
        let a1018 = v4;
        let a1019 = 2147483648u64;
        nat_lt(a1018, a1019)
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
        let v7: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, h1020, _, _, _, _, _, _, _, _) => {
                h1020
            }
        }, v4)?;
        let v8: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, _, h1021, _, _, _, _, _, _, _) => {
                h1021
            }
        }, v4)?;
        let a1023 = match v5.clone() {
            Adt2::C0(h1022, _, _, _, _, _, _, _, _, _, _) => {
                h1022
            }
        };
        let a1024 = 0u64;
        if if if if if nat_eq(a1023, a1024) {
            let a1026 = match v6.clone() {
                Adt2::C0(h1025, _, _, _, _, _, _, _, _, _, _) => {
                    h1025
                }
            };
            let a1027 = 0u64;
            nat_eq(a1026, a1027)
        } else {
            false
        } {
            let a1029 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1028, _, _, _, _, _, _) => {
                    h1028
                }
            };
            let a1030 = 1u64;
            nat_eq(a1029, a1030)
        } else {
            false
        } {
            let a1033 = match v5.clone() {
                Adt2::C0(_, h1031, _, _, _, _, _, _, _, _, _) => {
                    h1031
                }
            };
            let a1034 = match v6.clone() {
                Adt2::C0(_, h1032, _, _, _, _, _, _, _, _, _) => {
                    h1032
                }
            };
            nat_eq(a1033, a1034)
        } else {
            false
        } {
            if v0 {
                let a1036 = match v5.clone() {
                    Adt2::C0(_, _, h1035, _, _, _, _, _, _, _, _) => {
                        h1035
                    }
                };
                let a1037 = 2147483648u64;
                if if if nat_lt(a1036, a1037) {
                    let a1039 = match v5.clone() {
                        Adt2::C0(_, _, _, h1038, _, _, _, _, _, _, _) => {
                            h1038
                        }
                    };
                    let a1040 = 2147483648u64;
                    nat_lt(a1039, a1040)
                } else {
                    false
                } {
                    let a1043 = {
                        let a1041 = v7;
                        let a1042 = v8;
                        nat_mul(a1041, a1042)?
                    };
                    let a1044 = 2147483648u64;
                    nat_le(a1043, a1044)
                } else {
                    false
                } {
                    f78(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1046 = match v1.clone() {
                    Adt1::C0(h1045, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1045
                    }
                };
                let a1047 = v7;
                if if if if nat_eq(a1046, a1047) {
                    let a1049 = match v1 {
                        Adt1::C0(_, h1048, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1048
                        }
                    };
                    let a1050 = v8;
                    nat_eq(a1049, a1050)
                } else {
                    false
                } {
                    f79(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1052 = match v6.clone() {
                        Adt2::C0(_, _, h1051, _, _, _, _, _, _, _, _) => {
                            h1051
                        }
                    };
                    let a1053 = v4;
                    if nat_eq(a1052, a1053) {
                        let a1055 = match v6.clone() {
                            Adt2::C0(_, _, _, h1054, _, _, _, _, _, _, _) => {
                                h1054
                            }
                        };
                        let a1056 = v4;
                        nat_eq(a1055, a1056)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1060 = match v6 {
                        Adt2::C0(_, _, _, _, h1057, _, _, _, _, _, _) => {
                            h1057
                        }
                    };
                    let a1061 = {
                        let a1058 = v7;
                        let a1059 = v8;
                        nat_mul(a1058, a1059)?
                    };
                    nat_eq(a1060, a1061)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f43(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1063 = match v2.clone() {
        Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
            h1062
        }
    };
    let a1064 = 0u64;
    if if if if if nat_eq(a1063, a1064) {
        true
    } else {
        let a1066 = match v2.clone() {
            Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1067 = 1u64;
        nat_eq(a1066, a1067)
    } {
        true
    } else {
        let a1069 = match v2.clone() {
            Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
                h1068
            }
        };
        let a1070 = 30u64;
        nat_eq(a1069, a1070)
    } {
        let a1073 = match v2.clone() {
            Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                h1071
            }
        };
        let a1074 = match v3.clone() {
            Adt2::C0(h1072, _, _, _, _, _, _, _, _, _, _) => {
                h1072
            }
        };
        nat_eq(a1073, a1074)
    } else {
        false
    } {
        if if f14(v2.clone(), v3.clone()) {
            let a1076 = match v1.clone() {
                Adt1::C0(h1075, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1077 = 4294967296u64;
            nat_lt(a1076, a1077)
        } else {
            false
        } {
            let a1079 = match v1.clone() {
                Adt1::C0(_, h1078, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            };
            let a1080 = 4294967296u64;
            nat_lt(a1079, a1080)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, {
            let a1082 = match v2.clone() {
                Adt2::C0(h1081, _, _, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            };
            let a1083 = 0u64;
            if nat_eq(a1082, a1083) {
                4u64
            } else {
                2u64
            }
        }, f13(v2.clone(), 1u64), f13(v3, 1u64), Adt4::C13({
            let a1084 = v0;
            let a1085 = 31u64;
            if nat_eq(a1084, a1085) {
                0u64
            } else {
                let a1086 = v0;
                let a1087 = 47u64;
                if nat_eq(a1086, a1087) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        }, match v1.clone() {
            Adt1::C0(h1089, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1089
            }
        }, match v1 {
            Adt1::C0(_, h1090, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1090
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1092 = match v0.clone() {
        Adt2::C0(h1091, _, _, _, _, _, _, _, _, _, _) => {
            h1091
        }
    };
    let a1093 = 0u64;
    if if if nat_eq(a1092, a1093) {
        let a1095 = match v1.clone() {
            Adt2::C0(h1094, _, _, _, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1096 = 0u64;
        nat_eq(a1095, a1096)
    } else {
        false
    } {
        f14(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v1.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1098 = match v0.clone() {
        Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
            h1097
        }
    };
    let a1099 = 0u64;
    if if nat_eq(a1098, a1099) {
        let a1101 = match v1.clone() {
            Adt2::C0(h1100, _, _, _, _, _, _, _, _, _, _) => {
                h1100
            }
        };
        let a1102 = 26u64;
        nat_eq(a1101, a1102)
    } else {
        false
    } {
        let a1104 = match v0.clone() {
            Adt2::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1105 = 1u64;
        if if if if if if if nat_eq(a1104, a1105) {
            let a1107 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1106, _, _, _, _, _, _) => {
                    h1106
                }
            };
            let a1108 = 1u64;
            nat_eq(a1107, a1108)
        } else {
            false
        } {
            let a1110 = match v0.clone() {
                Adt2::C0(_, h1109, _, _, _, _, _, _, _, _, _) => {
                    h1109
                }
            };
            let a1111 = 2147483647u64;
            nat_le(a1110, a1111)
        } else {
            false
        } {
            let a1114 = match v1.clone() {
                Adt2::C0(_, h1112, _, _, _, _, _, _, _, _, _) => {
                    h1112
                }
            };
            let a1115 = match v0.clone() {
                Adt2::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
                    h1113
                }
            };
            nat_eq(a1114, a1115)
        } else {
            false
        } {
            let a1117 = match v1.clone() {
                Adt2::C0(_, _, h1116, _, _, _, _, _, _, _, _) => {
                    h1116
                }
            };
            let a1118 = 1u64;
            nat_eq(a1117, a1118)
        } else {
            false
        } {
            let a1120 = match v1.clone() {
                Adt2::C0(_, _, _, h1119, _, _, _, _, _, _, _) => {
                    h1119
                }
            };
            let a1121 = 1u64;
            nat_eq(a1120, a1121)
        } else {
            false
        } {
            let a1123 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1122, _, _, _, _, _, _) => {
                    h1122
                }
            };
            let a1124 = 1u64;
            nat_eq(a1123, a1124)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1127 = match v0.clone() {
                    Adt2::C0(_, _, h1125, _, _, _, _, _, _, _, _) => {
                        h1125
                    }
                };
                let a1128 = f80(match v0.clone() {
                    Adt2::C0(_, h1126, _, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                })?;
                nat_mul(a1127, a1128)?
            }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1129 = v1;
    match m1129 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f81(f13(v0.clone(), 1u64))?;
            let a1131 = match v0.clone() {
                Adt2::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                    h1130
                }
            };
            let a1132 = 26u64;
            if if if nat_eq(a1131, a1132) {
                let a1134 = match v3.clone() {
                    Adt2::C0(h1133, _, _, _, _, _, _, _, _, _, _) => {
                        h1133
                    }
                };
                let a1135 = 26u64;
                nat_eq(a1134, a1135)
            } else {
                false
            } {
                let a1137 = match v2.clone() {
                    Adt2::C0(h1136, _, _, _, _, _, _, _, _, _, _) => {
                        h1136
                    }
                };
                let a1138 = 27u64;
                nat_eq(a1137, a1138)
            } else {
                false
            } {
                if if if if if if f14(v0.clone(), v3.clone()) {
                    let a1140 = match v2.clone() {
                        Adt2::C0(_, h1139, _, _, _, _, _, _, _, _, _) => {
                            h1139
                        }
                    };
                    let a1141 = 1u64;
                    nat_eq(a1140, a1141)
                } else {
                    false
                } {
                    let a1143 = match v2.clone() {
                        Adt2::C0(_, _, h1142, _, _, _, _, _, _, _, _) => {
                            h1142
                        }
                    };
                    let a1144 = 1u64;
                    nat_eq(a1143, a1144)
                } else {
                    false
                } {
                    let a1146 = match v2.clone() {
                        Adt2::C0(_, _, _, h1145, _, _, _, _, _, _, _) => {
                            h1145
                        }
                    };
                    let a1147 = 1u64;
                    nat_eq(a1146, a1147)
                } else {
                    false
                } {
                    let a1149 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1148, _, _, _, _, _, _) => {
                            h1148
                        }
                    };
                    let a1150 = 1u64;
                    nat_eq(a1149, a1150)
                } else {
                    false
                } {
                    let a1151 = v4;
                    let a1152 = 9223372036854775808u64;
                    nat_lt(a1151, a1152)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1155 = 2u64;
                        let a1156 = {
                            let a1153 = v4;
                            let a1154 = 2u64;
                            nat_mul(a1153, a1154)?
                        };
                        nat_add(a1155, a1156)?
                    }, 8u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C18(v4, f13(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1157, _, _, _, _, _, _, _, _, _, _) => {
            h1157
        }
    };
    let a1158 = v3;
    let a1159 = 0u64;
    if if nat_eq(a1158, a1159) {
        true
    } else {
        let a1160 = v0;
        let a1161 = 13u64;
        if nat_eq(a1160, a1161) {
            let a1162 = v3;
            let a1163 = 1u64;
            if nat_eq(a1162, a1163) {
                true
            } else {
                let a1164 = v3;
                let a1165 = 30u64;
                nat_eq(a1164, a1165)
            }
        } else {
            false
        }
    } {
        let a1167 = v3;
        let a1168 = match v2.clone() {
            Adt2::C0(h1166, _, _, _, _, _, _, _, _, _, _) => {
                h1166
            }
        };
        if if nat_eq(a1167, a1168) {
            let a1169 = v0;
            let a1170 = 13u64;
            if nat_eq(a1169, a1170) {
                let a1172 = match v2.clone() {
                    Adt2::C0(_, h1171, _, _, _, _, _, _, _, _, _) => {
                        h1171
                    }
                };
                let a1173 = 1u64;
                if if if nat_eq(a1172, a1173) {
                    let a1175 = match v2.clone() {
                        Adt2::C0(_, _, h1174, _, _, _, _, _, _, _, _) => {
                            h1174
                        }
                    };
                    let a1176 = 1u64;
                    nat_eq(a1175, a1176)
                } else {
                    false
                } {
                    let a1178 = match v2.clone() {
                        Adt2::C0(_, _, _, h1177, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    let a1179 = 1u64;
                    nat_eq(a1178, a1179)
                } else {
                    false
                } {
                    let a1181 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1180, _, _, _, _, _, _) => {
                            h1180
                        }
                    };
                    let a1182 = 1u64;
                    nat_eq(a1181, a1182)
                } else {
                    false
                }
            } else {
                let a1184 = match v2.clone() {
                    Adt2::C0(_, h1183, _, _, _, _, _, _, _, _, _) => {
                        h1183
                    }
                };
                let a1185 = 1u64;
                if if if nat_eq(a1184, a1185) {
                    let a1188 = match v1.clone() {
                        Adt2::C0(_, _, h1186, _, _, _, _, _, _, _, _) => {
                            h1186
                        }
                    };
                    let a1189 = match v2.clone() {
                        Adt2::C0(_, _, h1187, _, _, _, _, _, _, _, _) => {
                            h1187
                        }
                    };
                    nat_eq(a1188, a1189)
                } else {
                    false
                } {
                    let a1192 = match v1.clone() {
                        Adt2::C0(_, _, _, h1190, _, _, _, _, _, _, _) => {
                            h1190
                        }
                    };
                    let a1193 = match v2.clone() {
                        Adt2::C0(_, _, _, h1191, _, _, _, _, _, _, _) => {
                            h1191
                        }
                    };
                    nat_eq(a1192, a1193)
                } else {
                    false
                } {
                    let a1196 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1194, _, _, _, _, _, _) => {
                            h1194
                        }
                    };
                    let a1197 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1195, _, _, _, _, _, _) => {
                            h1195
                        }
                    };
                    nat_eq(a1196, a1197)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f13(v1, 1u64);
            Ok(Adt6::C4(f82(v4.clone(), v0, v3)?, {
                let a1198 = v3;
                let a1199 = 0u64;
                if nat_eq(a1198, a1199) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f13(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f48(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1200 = 10u64;
    let a1201 = v0;
    if if nat_le(a1200, a1201) {
        let a1202 = v0;
        let a1203 = 12u64;
        nat_le(a1202, a1203)
    } else {
        false
    } {
        Ok(Some({
            let a1204 = v0;
            let a1205 = 3u64;
            nat_add(a1204, a1205)?
        }))
    } else {
        let a1206 = v0;
        let a1207 = 89u64;
        if if nat_eq(a1206, a1207) {
            let a1208 = v1;
            let a1209 = 7u64;
            nat_eq(a1208, a1209)
        } else {
            false
        } {
            Ok(Some(17u64))
        } else {
            let a1210 = v0;
            let a1211 = 89u64;
            if if nat_eq(a1210, a1211) {
                let a1212 = v1;
                let a1213 = 10u64;
                nat_eq(a1212, a1213)
            } else {
                false
            } {
                Ok(Some(18u64))
            } else {
                let a1214 = v0;
                let a1215 = 89u64;
                if if nat_eq(a1214, a1215) {
                    let a1216 = v1;
                    let a1217 = 13u64;
                    nat_eq(a1216, a1217)
                } else {
                    false
                } {
                    Ok(Some(16u64))
                } else {
                    let a1218 = v0;
                    let a1219 = 8u64;
                    if nat_eq(a1218, a1219) {
                        Ok(Some(5u64))
                    } else {
                        let a1220 = v0;
                        let a1221 = 9u64;
                        if nat_eq(a1220, a1221) {
                            Ok(Some(6u64))
                        } else {
                            let a1222 = v0;
                            let a1223 = 89u64;
                            if nat_eq(a1222, a1223) {
                                let a1224 = v1;
                                let a1225 = 0u64;
                                if nat_eq(a1224, a1225) {
                                    Ok(Some(0u64))
                                } else {
                                    let a1226 = v1;
                                    let a1227 = 1u64;
                                    if nat_eq(a1226, a1227) {
                                        Ok(Some(1u64))
                                    } else {
                                        let a1228 = v1;
                                        let a1229 = 2u64;
                                        if nat_eq(a1228, a1229) {
                                            Ok(Some(2u64))
                                        } else {
                                            let a1230 = v1;
                                            let a1231 = 3u64;
                                            if nat_eq(a1230, a1231) {
                                                Ok(Some(3u64))
                                            } else {
                                                let a1232 = v1;
                                                let a1233 = 6u64;
                                                if nat_eq(a1232, a1233) {
                                                    Ok(Some(4u64))
                                                } else {
                                                    let a1234 = v1;
                                                    let a1235 = 12u64;
                                                    if nat_eq(a1234, a1235) {
                                                        Ok(Some(7u64))
                                                    } else {
                                                        let a1236 = v1;
                                                        let a1237 = 11u64;
                                                        if nat_eq(a1236, a1237) {
                                                            Ok(Some(8u64))
                                                        } else {
                                                            let a1238 = v1;
                                                            let a1239 = 18u64;
                                                            if nat_eq(a1238, a1239) {
                                                                Ok(Some(9u64))
                                                            } else {
                                                                let a1240 = v1;
                                                                let a1241 = 19u64;
                                                                if nat_eq(a1240, a1241) {
                                                                    Ok(Some(10u64))
                                                                } else {
                                                                    let a1242 = v1;
                                                                    let a1243 = 20u64;
                                                                    if nat_eq(a1242, a1243) {
                                                                        Ok(Some(11u64))
                                                                    } else {
                                                                        let a1244 = v1;
                                                                        let a1245 = 21u64;
                                                                        if nat_eq(a1244, a1245) {
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

pub fn f49(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1246 = f83(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1246 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            let a1247 = v0;
            let a1248 = 2u64;
            if if if if nat_eq(a1247, a1248) {
                true
            } else {
                let a1249 = v0;
                let a1250 = 5u64;
                nat_eq(a1249, a1250)
            } {
                true
            } else {
                let a1251 = v0;
                let a1252 = 6u64;
                nat_eq(a1251, a1252)
            } {
                true
            } else {
                let a1253 = v0;
                let a1254 = 7u64;
                nat_eq(a1253, a1254)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1255 = v3;
                    match m1255 {
                        None => {
                            f13(v2, 1u64)
                        }
                        Some(v13) => {
                            f13(v13, 1u64)
                        }
                    }
                })))
            } else {
                Ok(Adt6::C5(67u64))
            }
        }
        Adt6::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) => {
            Ok(Adt6::C3(v14, v15, v16, v17, v18, v19, v20, v21, v22, v23))
        }
        Adt6::C4(v24, v25, v26, v27, v28) => {
            Ok(Adt6::C4(v24, v25, v26, v27, v28))
        }
        Adt6::C5(v5) => {
            Ok(Adt6::C5(v5))
        }
    }
}

pub fn f50(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1257 = match v1.clone() {
        Adt2::C0(h1256, _, _, _, _, _, _, _, _, _, _) => {
            h1256
        }
    };
    let a1258 = 0u64;
    if if if if if nat_eq(a1257, a1258) {
        true
    } else {
        let a1260 = match v1.clone() {
            Adt2::C0(h1259, _, _, _, _, _, _, _, _, _, _) => {
                h1259
            }
        };
        let a1261 = 1u64;
        nat_eq(a1260, a1261)
    } {
        true
    } else {
        let a1263 = match v1.clone() {
            Adt2::C0(h1262, _, _, _, _, _, _, _, _, _, _) => {
                h1262
            }
        };
        let a1264 = 30u64;
        nat_eq(a1263, a1264)
    } {
        let a1267 = match v1.clone() {
            Adt2::C0(h1265, _, _, _, _, _, _, _, _, _, _) => {
                h1265
            }
        };
        let a1268 = match v2.clone() {
            Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                h1266
            }
        };
        if nat_eq(a1267, a1268) {
            true
        } else {
            let a1270 = match v2.clone() {
                Adt2::C0(h1269, _, _, _, _, _, _, _, _, _, _) => {
                    h1269
                }
            };
            let a1271 = 0u64;
            nat_eq(a1270, a1271)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1273 = match v2.clone() {
                Adt2::C0(h1272, _, _, _, _, _, _, _, _, _, _) => {
                    h1272
                }
            };
            let a1274 = 0u64;
            if nat_eq(a1273, a1274) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1275, _, _, _, _, _, _, _, _, _, _) => {
                h1275
            }
        }, match v2 {
            Adt2::C0(h1276, _, _, _, _, _, _, _, _, _, _) => {
                h1276
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1277 = v1;
    match m1277 {
        Adt5::C0(v2) => {
            Adt5::C0(v2)
        }
        Adt5::C1(v3, v4, v5) => {
            Adt5::C1(v3, v4, v5)
        }
        Adt5::C2(v6, v7, v8, v9, v10, v11) => {
            Adt5::C2(v6, v7, v8, v9, v10, v11)
        }
        Adt5::C3(v12, v13, v14, v15) => {
            f84(f85(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f86(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f87(v0.clone(), v16);
            f84(f88(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
        }
        Adt5::C5(v24, v25, v26, v27, v28, v29) => {
            Adt5::C5(v24, v25, v26, v27, v28, v29)
        }
        Adt5::C6(v30, v31, v32, v33) => {
            Adt5::C6(v30, v31, v32, v33)
        }
        Adt5::C7(v34, v35, v36, v37, v38, v39) => {
            Adt5::C7(v34, v35, v36, v37, v38, v39)
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m1278 = v3;
    match m1278 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f91(v0, v1, v2.clone())?, f89(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f92(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f93(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f94(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f93(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f95(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f96(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f97(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f98(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f99(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f89(v0, v1)?, f89(v0, v2)?))
        }
        Adt4::C15 => {
            f100(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f101(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f102(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f89(f90(v0, match v2.clone() {
                Adt3::C0(h1279, _, _, _, _, _, _, _) => {
                    h1279
                }
            }, match v2.clone() {
                Adt3::C0(_, h1280, _, _, _, _, _, _) => {
                    h1280
                }
            })?, v1)?, f89(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C0(Adt6::C5(65u64)))
        }
        Adt4::C25(v48, v49, v50) => {
            f103(v0, v48, v49, v50)
        }
        Adt4::C26(v51, _, _, _) => {
            let a1281 = v51;
            let a1282 = 64u64;
            if if if nat_eq(a1281, a1282) {
                true
            } else {
                let a1283 = v51;
                let a1284 = 65u64;
                nat_eq(a1283, a1284)
            } {
                true
            } else {
                let a1285 = v51;
                let a1286 = 66u64;
                nat_eq(a1285, a1286)
            } {
                Ok(Adt5::C0(Adt6::C5(64u64)))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f80(match v1.clone() {
        Adt3::C0(h1287, _, _, _, _, _, _, _) => {
            h1287
        }
    })?;
    let v5: u64 = {
        let a1288 = v0;
        let a1289 = v4;
        let a1290 = 0u64;
        nat_quot(a1288, a1289, a1290)
    };
    let v6: u64 = {
        let a1291 = v0;
        let a1292 = v4;
        let a1293 = 0u64;
        nat_rem(a1291, a1292, a1293)
    };
    let a1294 = v6;
    let a1295 = 0u64;
    if nat_eq(a1294, a1295) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1296 = v6;
        let a1297 = 1u64;
        if nat_eq(a1296, a1297) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1300 = {
                let a1298 = v6;
                let a1299 = 1u64;
                nat_add(a1298, a1299)?
            };
            let a1301 = v4;
            if nat_eq(a1300, a1301) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1302 = v6;
                    let a1303 = 2u64;
                    nat_sub(a1302, a1303)
                };
                let v8: u64 = {
                    let a1304 = v7;
                    let a1305 = 3u64;
                    let a1306 = 0u64;
                    nat_quot(a1304, a1305, a1306)
                };
                let v9: u64 = {
                    let a1307 = v7;
                    let a1308 = 3u64;
                    let a1309 = 0u64;
                    nat_rem(a1307, a1308, a1309)
                };
                let v10: u64 = f89({
                    let a1313 = {
                        let a1311 = v5;
                        let a1312 = match v1.clone() {
                            Adt3::C0(h1310, _, _, _, _, _, _, _) => {
                                h1310
                            }
                        };
                        nat_mul(a1311, a1312)?
                    };
                    let a1314 = v8;
                    nat_add(a1313, a1314)?
                }, v1)?;
                let a1315 = v9;
                let a1316 = 0u64;
                if nat_eq(a1315, a1316) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1317 = v9;
                    let a1318 = 1u64;
                    if nat_eq(a1317, a1318) {
                        let a1319 = v3;
                        let a1320 = 2u64;
                        if nat_eq(a1319, a1320) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1321 = v3;
                        let a1322 = 2u64;
                        if if nat_eq(a1321, a1322) {
                            true
                        } else {
                            let a1323 = v3;
                            let a1324 = 3u64;
                            nat_eq(a1323, a1324)
                        } {
                            Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(19u64, 2u64, v8, 2u64, 0u64, 1u64, 1u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1326 = 2u64;
        let a1327 = match v1.clone() {
            Adt3::C0(_, h1325, _, _, _, _, _, _) => {
                h1325
            }
        };
        nat_mul(a1326, a1327)?
    };
    let a1328 = v0;
    let a1329 = v5;
    if nat_lt(a1328, a1329) {
        let a1333 = {
            let a1330 = v0;
            let a1331 = 2u64;
            let a1332 = 0u64;
            nat_rem(a1330, a1331, a1332)
        };
        let a1334 = 0u64;
        if nat_eq(a1333, a1334) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1335 = v0;
                let a1336 = 2u64;
                let a1337 = 0u64;
                nat_quot(a1335, a1336, a1337)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1338 = v4;
            let a1339 = 2147483648u64;
            if if nat_lt(a1338, a1339) {
                let a1341 = v4;
                let a1342 = match v2 {
                    Adt3::C0(_, h1340, _, _, _, _, _, _) => {
                        h1340
                    }
                };
                nat_lt(a1341, a1342)
            } else {
                false
            } {
                Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1345 = match v2.clone() {
                Adt3::C0(h1343, _, _, _, _, _, _, _) => {
                    h1343
                }
            };
            let a1346 = match v2.clone() {
                Adt3::C0(_, h1344, _, _, _, _, _, _) => {
                    h1344
                }
            };
            nat_mul(a1345, a1346)?
        };
        let a1349 = {
            let a1347 = v0;
            let a1348 = v5;
            nat_sub(a1347, a1348)
        };
        let a1350 = v6;
        if nat_lt(a1349, a1350) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1351 = v0;
                let a1352 = v5;
                nat_sub(a1351, a1352)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1355 = {
                    let a1353 = v0;
                    let a1354 = v5;
                    nat_sub(a1353, a1354)
                };
                let a1356 = v6;
                nat_sub(a1355, a1356)
            };
            let v8: u64 = {
                let a1360 = 1u64;
                let a1361 = {
                    let a1358 = 2u64;
                    let a1359 = match v1.clone() {
                        Adt3::C0(h1357, _, _, _, _, _, _, _) => {
                            h1357
                        }
                    };
                    nat_mul(a1358, a1359)?
                };
                nat_add(a1360, a1361)?
            };
            let v9: u64 = {
                let a1362 = v7;
                let a1363 = v8;
                let a1364 = 0u64;
                nat_quot(a1362, a1363, a1364)
            };
            let v10: u64 = {
                let a1365 = v7;
                let a1366 = v8;
                let a1367 = 0u64;
                nat_rem(a1365, a1366, a1367)
            };
            let a1368 = v10;
            let a1369 = 0u64;
            if nat_eq(a1368, a1369) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1373 = {
                    let a1370 = v10;
                    let a1371 = 2u64;
                    let a1372 = 0u64;
                    nat_rem(a1370, a1371, a1372)
                };
                let a1374 = 1u64;
                if nat_eq(a1373, a1374) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1383 = {
                            let a1376 = v9;
                            let a1377 = match v1.clone() {
                                Adt3::C0(h1375, _, _, _, _, _, _, _) => {
                                    h1375
                                }
                            };
                            nat_mul(a1376, a1377)?
                        };
                        let a1384 = {
                            let a1380 = {
                                let a1378 = v10;
                                let a1379 = 1u64;
                                nat_sub(a1378, a1379)
                            };
                            let a1381 = 2u64;
                            let a1382 = 0u64;
                            nat_quot(a1380, a1381, a1382)
                        };
                        nat_add(a1383, a1384)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1393 = {
                            let a1386 = v4;
                            let a1387 = match v1.clone() {
                                Adt3::C0(h1385, _, _, _, _, _, _, _) => {
                                    h1385
                                }
                            };
                            nat_mul(a1386, a1387)?
                        };
                        let a1394 = {
                            let a1390 = {
                                let a1388 = v10;
                                let a1389 = 1u64;
                                nat_sub(a1388, a1389)
                            };
                            let a1391 = 2u64;
                            let a1392 = 0u64;
                            nat_quot(a1390, a1391, a1392)
                        };
                        nat_add(a1393, a1394)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1403 = {
                            let a1396 = v4;
                            let a1397 = match v1 {
                                Adt3::C0(h1395, _, _, _, _, _, _, _) => {
                                    h1395
                                }
                            };
                            nat_mul(a1396, a1397)?
                        };
                        let a1404 = {
                            let a1400 = {
                                let a1398 = v10;
                                let a1399 = 1u64;
                                nat_sub(a1398, a1399)
                            };
                            let a1401 = 2u64;
                            let a1402 = 0u64;
                            nat_quot(a1400, a1401, a1402)
                        };
                        nat_add(a1403, a1404)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1405 = v1;
    let a1406 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1405, a1406) {
        let a1409 = v1;
        let a1410 = {
            let a1407 = 2u64;
            let a1408 = f81(v4.clone())?;
            nat_mul(a1407, a1408)?
        };
        if nat_lt(a1409, a1410) {
            let a1414 = {
                let a1411 = v1;
                let a1412 = 2u64;
                let a1413 = 0u64;
                nat_rem(a1411, a1412, a1413)
            };
            let a1415 = 0u64;
            if nat_eq(a1414, a1415) {
                f105({
                    let a1416 = v1;
                    let a1417 = 2u64;
                    let a1418 = 0u64;
                    nat_quot(a1416, a1417, a1418)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1419, _, _, _, _, _, _) => {
                        h1419
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1422 = v1;
                let a1423 = {
                    let a1420 = 2u64;
                    let a1421 = f81(v4.clone())?;
                    nat_mul(a1420, a1421)?
                };
                nat_sub(a1422, a1423)
            };
            let v7: u64 = {
                let a1427 = v6;
                let a1428 = {
                    let a1425 = 1u64;
                    let a1426 = match v2.clone() {
                        Adt3::C0(h1424, _, _, _, _, _, _, _) => {
                            h1424
                        }
                    };
                    nat_add(a1425, a1426)?
                };
                let a1429 = 0u64;
                nat_quot(a1427, a1428, a1429)
            };
            let v8: u64 = {
                let a1433 = v6;
                let a1434 = {
                    let a1431 = 1u64;
                    let a1432 = match v2.clone() {
                        Adt3::C0(h1430, _, _, _, _, _, _, _) => {
                            h1430
                        }
                    };
                    nat_add(a1431, a1432)?
                };
                let a1435 = 0u64;
                nat_rem(a1433, a1434, a1435)
            };
            let a1436 = v8;
            let a1437 = 0u64;
            if nat_eq(a1436, a1437) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1438 = v8;
                    let a1439 = 1u64;
                    nat_sub(a1438, a1439)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1442 = match v4.clone() {
            Adt3::C0(h1440, _, _, _, _, _, _, _) => {
                h1440
            }
        };
        let a1443 = match v4.clone() {
            Adt3::C0(_, h1441, _, _, _, _, _, _) => {
                h1441
            }
        };
        nat_mul(a1442, a1443)?
    };
    let v7: u64 = {
        let a1444 = 2u64;
        let a1445 = v6;
        nat_mul(a1444, a1445)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1446, _, _, _, _, _, _) => {
            h1446
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1447, _, _, _, _, _) => {
            h1447
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1448, _, _, _, _) => {
            h1448
        }
    });
    let a1449 = v0;
    let a1450 = v7;
    if nat_lt(a1449, a1450) {
        let a1454 = {
            let a1451 = v0;
            let a1452 = 2u64;
            let a1453 = 0u64;
            nat_rem(a1451, a1452, a1453)
        };
        let a1455 = 0u64;
        if nat_eq(a1454, a1455) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1456 = v0;
                let a1457 = 2u64;
                let a1458 = 0u64;
                nat_quot(a1456, a1457, a1458)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1459 = v0;
            let a1460 = v7;
            nat_sub(a1459, a1460)
        };
        let v10: u64 = {
            let a1464 = v9;
            let a1465 = {
                let a1462 = 1u64;
                let a1463 = match v1.clone() {
                    Adt3::C0(h1461, _, _, _, _, _, _, _) => {
                        h1461
                    }
                };
                nat_add(a1462, a1463)?
            };
            let a1466 = 0u64;
            nat_quot(a1464, a1465, a1466)
        };
        let v11: u64 = {
            let a1470 = v9;
            let a1471 = {
                let a1468 = 1u64;
                let a1469 = match v1.clone() {
                    Adt3::C0(h1467, _, _, _, _, _, _, _) => {
                        h1467
                    }
                };
                nat_add(a1468, a1469)?
            };
            let a1472 = 0u64;
            nat_rem(a1470, a1471, a1472)
        };
        let a1473 = v11;
        let a1474 = 0u64;
        if nat_eq(a1473, a1474) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1475 = v10;
                let a1476 = v6;
                let a1477 = 0u64;
                nat_rem(a1475, a1476, a1477)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1483 = {
                        let a1479 = v10;
                        let a1480 = match v1.clone() {
                            Adt3::C0(h1478, _, _, _, _, _, _, _) => {
                                h1478
                            }
                        };
                        nat_mul(a1479, a1480)?
                    };
                    let a1484 = {
                        let a1481 = v11;
                        let a1482 = 1u64;
                        nat_sub(a1481, a1482)
                    };
                    nat_add(a1483, a1484)?
                };
                let a1485 = v5;
                let a1486 = 4294967295u64;
                if nat_eq(a1485, a1486) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1492 = {
                            let a1488 = v5;
                            let a1489 = match v3.clone() {
                                Adt3::C0(h1487, _, _, _, _, _, _, _) => {
                                    h1487
                                }
                            };
                            nat_mul(a1488, a1489)?
                        };
                        let a1493 = {
                            let a1490 = v11;
                            let a1491 = 1u64;
                            nat_sub(a1490, a1491)
                        };
                        nat_add(a1492, a1493)?
                    }, v3)?, 3u64, f89(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt5> {
    let v8: u64 = {
        let a1496 = v6;
        let a1497 = {
            let a1494 = 3u64;
            let a1495 = v5;
            nat_add(a1494, a1495)?
        };
        nat_sub(a1496, a1497)
    };
    let a1499 = v0;
    let a1500 = f25(v3, match v1.clone() {
        Adt3::C0(h1498, _, _, _, _, _, _, _) => {
            h1498
        }
    }, v5, v8)?;
    if nat_lt(a1499, a1500) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1501, _, _, _, _, _, _, _) => {
                h1501
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1502, _, _, _, _, _, _, _) => {
                h1502
            }
        }, v5)?;
        let a1503 = v0;
        let a1504 = v9;
        if nat_lt(a1503, a1504) {
            f110(v0, v2, f19(v3), {
                let a1505 = 3u64;
                let a1506 = v5;
                nat_add(a1505, a1506)?
            }, v7)
        } else {
            let a1509 = v0;
            let a1510 = {
                let a1507 = v9;
                let a1508 = v10;
                nat_add(a1507, a1508)?
            };
            if nat_lt(a1509, a1510) {
                let v11: u64 = {
                    let a1511 = v0;
                    let a1512 = v9;
                    nat_sub(a1511, a1512)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1513, _, _, _, _, _, _, _) => {
                        h1513
                    }
                })?;
                f112({
                    let a1514 = v11;
                    let a1515 = v12;
                    let a1516 = 0u64;
                    nat_rem(a1514, a1515, a1516)
                }, {
                    let a1517 = v11;
                    let a1518 = v12;
                    let a1519 = 0u64;
                    nat_quot(a1517, a1518, a1519)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1522 = v0;
                    let a1523 = {
                        let a1520 = v9;
                        let a1521 = v10;
                        nat_add(a1520, a1521)?
                    };
                    nat_sub(a1522, a1523)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1524 = v0;
    let a1525 = f12(v1)?;
    if nat_lt(a1524, a1525) {
        let a1526 = v0;
        let a1527 = 4u64;
        if nat_lt(a1526, a1527) {
            let a1528 = v0;
            let a1529 = 0u64;
            if nat_eq(a1528, a1529) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1530 = v0;
                let a1531 = 1u64;
                if nat_eq(a1530, a1531) {
                    let a1532 = v5;
                    let a1533 = 0u64;
                    if nat_eq(a1532, a1533) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1534 = v0;
                    let a1535 = 2u64;
                    if nat_eq(a1534, a1535) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1536 = v0;
            let a1537 = 4u64;
            if if nat_eq(a1536, a1537) {
                let a1538 = f114(v5)?;
                let a1539 = v1;
                !nat_eq(a1538, a1539)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1542 = {
                        let a1540 = v0;
                        let a1541 = 4u64;
                        nat_sub(a1540, a1541)
                    };
                    let a1543 = 3u64;
                    let a1544 = 0u64;
                    nat_quot(a1542, a1543, a1544)
                };
                let v7: u64 = {
                    let a1547 = {
                        let a1545 = v0;
                        let a1546 = 4u64;
                        nat_sub(a1545, a1546)
                    };
                    let a1548 = 3u64;
                    let a1549 = 0u64;
                    nat_rem(a1547, a1548, a1549)
                };
                let a1550 = v7;
                let a1551 = 0u64;
                if nat_eq(a1550, a1551) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1552 = v7;
                    let a1553 = 1u64;
                    if nat_eq(a1552, a1553) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f11(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f59(v0: u64) -> u64 {
    let v1: u64 = {
        let a1554 = v0;
        let a1555 = 4294967296u64;
        let a1556 = 0u64;
        nat_rem(a1554, a1555, a1556)
    };
    let a1557 = v1;
    let a1558 = 2147483648u64;
    if nat_lt(a1557, a1558) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1561 = 2u64;
    let a1562 = {
        let a1559 = 2u64;
        let a1560 = v0;
        nat_mul(a1559, a1560)?
    };
    nat_add(a1561, a1562)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f115(v0, v3, v6) {
        f115(v1, v4, v6)
    } else {
        false
    } {
        f115(v2, v5, v6)
    } else {
        false
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f116(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f116(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f116(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f116(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f116(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f116(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1563 = v2;
    let a1564 = 0u64;
    if nat_eq(a1563, a1564) {
        Ok(0u64)
    } else {
        let a1567 = 2u64;
        let a1568 = {
            let a1565 = v0;
            let a1566 = v1;
            nat_mul(a1565, a1566)?
        };
        nat_mul(a1567, a1568)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1569 = v2;
    let a1570 = f111(v0, v1)?;
    nat_mul(a1569, a1570)
}

pub fn f65(v0: u64) -> u64 {
    let a1571 = v0;
    let a1572 = 30u64;
    if nat_eq(a1571, a1572) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1575 = 5u64;
    let a1576 = {
        let a1573 = 4u64;
        let a1574 = v0;
        nat_mul(a1573, a1574)?
    };
    nat_add(a1575, a1576)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1577, _, _, _, _, _, _, _, _, _, _) => {
            h1577
        }
    }, match v0.clone() {
        Adt2::C0(_, h1578, _, _, _, _, _, _, _, _, _) => {
            h1578
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1579, _, _, _, _, _, _, _, _) => {
            h1579
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1580, _, _, _, _, _, _, _) => {
            h1580
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1581, _, _, _, _, _, _) => {
            h1581
        }
    }, match v1.clone() {
        Adt2::C0(h1582, _, _, _, _, _, _, _, _, _, _) => {
            h1582
        }
    }, match v1.clone() {
        Adt2::C0(_, h1583, _, _, _, _, _, _, _, _, _) => {
            h1583
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1584, _, _, _, _, _, _, _, _) => {
            h1584
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1585, _, _, _, _, _, _, _) => {
            h1585
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1586, _, _, _, _, _, _) => {
            h1586
        }
    }, match v2.clone() {
        Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
            h1587
        }
    }, match v2.clone() {
        Adt2::C0(_, h1588, _, _, _, _, _, _, _, _, _) => {
            h1588
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1589, _, _, _, _, _, _, _, _) => {
            h1589
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1590, _, _, _, _, _, _, _) => {
            h1590
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1591, _, _, _, _, _, _) => {
            h1591
        }
    }) {
        let m1593 = f70(match v0.clone() {
            Adt2::C0(h1592, _, _, _, _, _, _, _, _, _, _) => {
                h1592
            }
        });
        match m1593 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1599 = {
                    let a1596 = match v0.clone() {
                        Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                            h1594
                        }
                    };
                    let a1597 = match v3.clone() {
                        Adt11::C0(_, h1595, _) => {
                            h1595
                        }
                    };
                    let a1598 = 0u64;
                    nat_rem(a1596, a1597, a1598)
                };
                let a1600 = 0u64;
                if if nat_eq(a1599, a1600) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1601, _, _, _, _, _, _, _, _, _) => {
                            h1601
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1602, _, _, _, _, _, _, _, _, _) => {
                            h1602
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1603, _, _, _, _, _, _, _, _) => {
                            h1603
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1604, _, _, _, _, _, _, _) => {
                            h1604
                        }
                    }) {
                        true
                    } else {
                        let a1606 = 4u64;
                        let a1607 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1605, _, _, _, _, _) => {
                                h1605
                            }
                        };
                        if if nat_le(a1606, a1607) {
                            let a1609 = match v3.clone() {
                                Adt11::C0(_, h1608, _) => {
                                    h1608
                                }
                            };
                            let a1610 = 1u64;
                            if nat_eq(a1609, a1610) {
                                true
                            } else {
                                let a1613 = match v3.clone() {
                                    Adt11::C0(_, _, h1611) => {
                                        h1611
                                    }
                                };
                                let a1614 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1612, _, _, _, _, _) => {
                                        h1612
                                    }
                                };
                                nat_le(a1613, a1614)
                            }
                        } else {
                            false
                        } {
                            if f16(v0.clone(), v2.clone())? {
                                f16(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f13(v0.clone(), match v3 {
                        Adt11::C0(_, h1615, _) => {
                            h1615
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1616, _, _, _, _, _, _, _, _, _) => {
                            h1616
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1617, _, _, _, _, _, _, _, _, _) => {
                            h1617
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1618, _, _, _, _, _, _, _, _) => {
                            h1618
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1619, _, _, _, _, _, _, _) => {
                            h1619
                        }
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1620, _, _, _, _, _, _, _, _, _, _) => {
                            h1620
                        }
                    }, f13(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f68(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1623 = match v1.clone() {
        Adt2::C0(h1621, _, _, _, _, _, _, _, _, _, _) => {
            h1621
        }
    };
    let a1624 = match v3.clone() {
        Adt2::C0(h1622, _, _, _, _, _, _, _, _, _, _) => {
            h1622
        }
    };
    if if nat_eq(a1623, a1624) {
        let a1625 = v0;
        let a1626 = 33u64;
        if nat_eq(a1625, a1626) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1628 = f70(match v1.clone() {
            Adt2::C0(h1627, _, _, _, _, _, _, _, _, _, _) => {
                h1627
            }
        });
        match m1628 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1629, _) => {
                        h1629
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1634 = {
                    let a1631 = match v1.clone() {
                        Adt2::C0(_, h1630, _, _, _, _, _, _, _, _, _) => {
                            h1630
                        }
                    };
                    let a1632 = v5;
                    let a1633 = 1u64;
                    nat_rem(a1631, a1632, a1633)
                };
                let a1635 = 0u64;
                if if if nat_eq(a1634, a1635) {
                    let a1640 = {
                        let a1637 = match v3.clone() {
                            Adt2::C0(_, h1636, _, _, _, _, _, _, _, _, _) => {
                                h1636
                            }
                        };
                        let a1638 = v5;
                        let a1639 = 1u64;
                        nat_rem(a1637, a1638, a1639)
                    };
                    let a1641 = 0u64;
                    nat_eq(a1640, a1641)
                } else {
                    false
                } {
                    let a1642 = v6;
                    let a1643 = f9(v3.clone(), v5)?;
                    nat_eq(a1642, a1643)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1644) => {
                            h1644
                        }
                    }, f13(v1, v5), f13(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f69(v0: u64) -> bool {
    let a1645 = v0;
    let a1646 = 0u64;
    if if nat_eq(a1645, a1646) {
        true
    } else {
        let a1647 = v0;
        let a1648 = 1u64;
        nat_eq(a1647, a1648)
    } {
        true
    } else {
        let a1649 = v0;
        let a1650 = 30u64;
        nat_eq(a1649, a1650)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1651 = v0;
    let a1652 = 0u64;
    if nat_eq(a1651, a1652) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1653 = v0;
        let a1654 = 1u64;
        if nat_eq(a1653, a1654) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1655 = v0;
            let a1656 = 2u64;
            if nat_eq(a1655, a1656) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1657 = v0;
                let a1658 = 3u64;
                if nat_eq(a1657, a1658) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1659 = v0;
                    let a1660 = 6u64;
                    if nat_eq(a1659, a1660) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1661 = v0;
                        let a1662 = 7u64;
                        if nat_eq(a1661, a1662) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1663 = v0;
                            let a1664 = 8u64;
                            if nat_eq(a1663, a1664) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1665 = v0;
                                let a1666 = 9u64;
                                if nat_eq(a1665, a1666) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1667 = v0;
                                    let a1668 = 10u64;
                                    if nat_eq(a1667, a1668) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1669 = v0;
                                        let a1670 = 11u64;
                                        if nat_eq(a1669, a1670) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1671 = v0;
                                            let a1672 = 12u64;
                                            if nat_eq(a1671, a1672) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1673 = v0;
                                                let a1674 = 13u64;
                                                if nat_eq(a1673, a1674) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1675 = v0;
                                                    let a1676 = 14u64;
                                                    if nat_eq(a1675, a1676) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1677 = v0;
                                                        let a1678 = 15u64;
                                                        if nat_eq(a1677, a1678) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1679 = v0;
                                                            let a1680 = 16u64;
                                                            if nat_eq(a1679, a1680) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1681 = v0;
                                                                let a1682 = 17u64;
                                                                if nat_eq(a1681, a1682) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1683 = v0;
                                                                    let a1684 = 18u64;
                                                                    if nat_eq(a1683, a1684) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1685 = v0;
                                                                        let a1686 = 19u64;
                                                                        if nat_eq(a1685, a1686) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1687 = v0;
                                                                            let a1688 = 20u64;
                                                                            if nat_eq(a1687, a1688) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1689 = v0;
                                                                                let a1690 = 21u64;
                                                                                if nat_eq(a1689, a1690) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1691 = v0;
                                                                                    let a1692 = 22u64;
                                                                                    if nat_eq(a1691, a1692) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1693 = v0;
                                                                                        let a1694 = 23u64;
                                                                                        if nat_eq(a1693, a1694) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1695 = v0;
                                                                                            let a1696 = 24u64;
                                                                                            if nat_eq(a1695, a1696) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1697 = v0;
                                                                                                let a1698 = 25u64;
                                                                                                if nat_eq(a1697, a1698) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1699 = v0;
                                                                                                    let a1700 = 26u64;
                                                                                                    if nat_eq(a1699, a1700) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1701 = v0;
                                                                                                        let a1702 = 27u64;
                                                                                                        if nat_eq(a1701, a1702) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1703 = v0;
                                                                                                            let a1704 = 28u64;
                                                                                                            if nat_eq(a1703, a1704) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1705 = v0;
                                                                                                                let a1706 = 29u64;
                                                                                                                if nat_eq(a1705, a1706) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1707 = v0;
                                                                                                                    let a1708 = 30u64;
                                                                                                                    if nat_eq(a1707, a1708) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1709 = v0;
                                                                                                                        let a1710 = 34u64;
                                                                                                                        if nat_eq(a1709, a1710) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1711 = v0;
                                                                                                                            let a1712 = 35u64;
                                                                                                                            if nat_eq(a1711, a1712) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1713 = v0;
                                                                                                                                let a1714 = 39u64;
                                                                                                                                if nat_eq(a1713, a1714) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1715 = v0;
                                                                                                                                    let a1716 = 40u64;
                                                                                                                                    if nat_eq(a1715, a1716) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1717 = v0;
                                                                                                                                        let a1718 = 41u64;
                                                                                                                                        if nat_eq(a1717, a1718) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1719 = v0;
                                                                                                                                            let a1720 = 42u64;
                                                                                                                                            if nat_eq(a1719, a1720) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1721 = v0;
                                                                                                                                                let a1722 = 43u64;
                                                                                                                                                if nat_eq(a1721, a1722) {
                                                                                                                                                    Adt10::C1(Adt11::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt10::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f23(v4, v5, v6, v7) {
        f23(v0, v1, v2, v3)
    } else {
        let a1726 = {
            let a1723 = v0;
            let a1724 = v4;
            let a1725 = 1u64;
            nat_rem(a1723, a1724, a1725)
        };
        let a1727 = 0u64;
        if if if nat_eq(a1726, a1727) {
            let a1731 = {
                let a1728 = v1;
                let a1729 = v5;
                let a1730 = 1u64;
                nat_rem(a1728, a1729, a1730)
            };
            let a1732 = 0u64;
            nat_eq(a1731, a1732)
        } else {
            false
        } {
            let a1736 = {
                let a1733 = v2;
                let a1734 = v6;
                let a1735 = 1u64;
                nat_rem(a1733, a1734, a1735)
            };
            let a1737 = 0u64;
            nat_eq(a1736, a1737)
        } else {
            false
        } {
            let a1741 = {
                let a1738 = v3;
                let a1739 = v7;
                let a1740 = 1u64;
                nat_rem(a1738, a1739, a1740)
            };
            let a1742 = 0u64;
            nat_eq(a1741, a1742)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1743 = v0;
    let a1744 = v1;
    if nat_eq(a1743, a1744) {
        let a1747 = v4;
        let a1748 = {
            let a1745 = v2;
            let a1746 = v3;
            nat_add(a1745, a1746)?
        };
        Ok(nat_eq(a1747, a1748))
    } else {
        let a1749 = v2;
        let a1750 = v3;
        if nat_eq(a1749, a1750) {
            let a1751 = v4;
            let a1752 = v2;
            Ok(nat_eq(a1751, a1752))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1755 = {
            let a1753 = v0;
            let a1754 = f119(v1);
            nat_add(a1753, a1754)?
        };
        let a1756 = f119(v2);
        nat_add(a1755, a1756)?
    };
    let v5: u64 = {
        let a1757 = f120(v1);
        let a1758 = f120(v2);
        nat_add(a1757, a1758)?
    };
    let a1759 = v1;
    let a1760 = 4294967295u64;
    if if nat_le(a1759, a1760) {
        let a1761 = v2;
        let a1762 = 4294967295u64;
        nat_le(a1761, a1762)
    } else {
        false
    } {
        let a1763 = v5;
        let a1764 = v4;
        if nat_le(a1763, a1764) {
            let a1767 = v3;
            let a1768 = {
                let a1765 = v4;
                let a1766 = v5;
                nat_sub(a1765, a1766)
            };
            Ok(nat_eq(a1767, a1768))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1769 = v1;
    let a1770 = 4294967295u64;
    if nat_le(a1769, a1770) {
        let a1773 = {
            let a1771 = f119(v1);
            let a1772 = f120(v1);
            nat_add(a1771, a1772)?
        };
        let a1774 = v0;
        Ok(nat_lt(a1773, a1774))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1775 = v6;
    let a1776 = v1;
    if if nat_le(a1775, a1776) {
        let a1783 = {
            let a1781 = {
                let a1777 = v1;
                let a1778 = v5;
                nat_add(a1777, a1778)?
            };
            let a1782 = {
                let a1779 = v3;
                let a1780 = 0u64;
                if nat_eq(a1779, a1780) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1781, a1782)?
        };
        let a1784 = v2;
        nat_le(a1783, a1784)
    } else {
        false
    } {
        if v4 {
            let a1789 = {
                let a1787 = {
                    let a1785 = v1;
                    let a1786 = v5;
                    nat_add(a1785, a1786)?
                };
                let a1788 = v6;
                nat_sub(a1787, a1788)
            };
            let a1790 = v2;
            Ok(nat_lt(a1789, a1790))
        } else {
            let a1795 = {
                let a1793 = {
                    let a1791 = v1;
                    let a1792 = v5;
                    nat_add(a1791, a1792)?
                };
                let a1794 = v6;
                nat_sub(a1793, a1794)
            };
            let a1796 = v2;
            Ok(nat_le(a1795, a1796))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1797 = v0;
    let a1798 = 0u64;
    if if nat_eq(a1797, a1798) {
        true
    } else {
        let a1799 = v0;
        let a1800 = 1u64;
        nat_eq(a1799, a1800)
    } {
        let a1801 = v1;
        let a1802 = 4294967296u64;
        nat_lt(a1801, a1802)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1807 = {
        let a1805 = v0;
        let a1806 = {
            let a1803 = v1;
            let a1804 = 1u64;
            nat_sub(a1803, a1804)
        };
        nat_add(a1805, a1806)?
    };
    let a1808 = v1;
    let a1809 = 0u64;
    Ok(nat_quot(a1807, a1808, a1809))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1810, _, _, _, _, _, _, _, _, _) => {
            h1810
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1811, _, _, _, _, _, _, _, _) => {
            h1811
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1812, _, _, _, _, _, _, _) => {
            h1812
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1813, _, _, _, _, _, _) => {
            h1813
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1816 = match v1.clone() {
                Adt2::C0(_, h1814, _, _, _, _, _, _, _, _, _) => {
                    h1814
                }
            };
            let a1817 = match v1.clone() {
                Adt2::C0(_, _, h1815, _, _, _, _, _, _, _, _) => {
                    h1815
                }
            };
            nat_mul(a1816, a1817)?
        };
        let v5: u64 = {
            let a1819 = v4;
            let a1820 = match v1.clone() {
                Adt2::C0(_, _, _, h1818, _, _, _, _, _, _, _) => {
                    h1818
                }
            };
            nat_mul(a1819, a1820)?
        };
        let a1833 = {
            let a1831 = {
                let a1822 = match v0.clone() {
                    Adt2::C0(_, h1821, _, _, _, _, _, _, _, _, _) => {
                        h1821
                    }
                };
                let a1823 = 1u64;
                nat_sub(a1822, a1823)
            };
            let a1832 = {
                let a1829 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1824, _, _, _, _, _, _, _, _) => {
                        h1824
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1825, _, _, _, _, _, _, _, _, _) => {
                        h1825
                    }
                })?;
                let a1830 = f122(match v0 {
                    Adt2::C0(_, _, _, h1826, _, _, _, _, _, _, _) => {
                        h1826
                    }
                }, v2, {
                    let a1827 = v3;
                    let a1828 = v5;
                    nat_mul(a1827, a1828)?
                }, v4)?;
                nat_add(a1829, a1830)?
            };
            nat_add(a1831, a1832)?
        };
        let a1834 = f9(v1, 1u64)?;
        Ok(nat_lt(a1833, a1834))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1835 = v0;
    let a1836 = 2147483648u64;
    if if if if nat_lt(a1835, a1836) {
        let a1837 = v1;
        let a1838 = 2147483648u64;
        nat_lt(a1837, a1838)
    } else {
        false
    } {
        let a1841 = {
            let a1839 = v0;
            let a1840 = v1;
            nat_mul(a1839, a1840)?
        };
        let a1842 = 2147483648u64;
        nat_lt(a1841, a1842)
    } else {
        false
    } {
        let a1847 = {
            let a1845 = {
                let a1843 = v0;
                let a1844 = 1u64;
                nat_sub(a1843, a1844)
            };
            let a1846 = v2;
            nat_mul(a1845, a1846)?
        };
        let a1848 = 2147483648u64;
        nat_lt(a1847, a1848)
    } else {
        false
    } {
        let a1853 = {
            let a1851 = {
                let a1849 = v1;
                let a1850 = 1u64;
                nat_sub(a1849, a1850)
            };
            let a1852 = v2;
            nat_mul(a1851, a1852)?
        };
        let a1854 = 2147483648u64;
        Ok(nat_lt(a1853, a1854))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1857 = 3u64;
    let a1858 = {
        let a1855 = v0;
        let a1856 = 3u64;
        nat_mul(a1855, a1856)?
    };
    nat_add(a1857, a1858)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1859, _, _, _, _, _, _, _) => {
            h1859
        }
    }, match v0.clone() {
        Adt3::C0(_, h1860, _, _, _, _, _, _) => {
            h1860
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1861, _, _, _, _, _) => {
            h1861
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1862, _, _, _, _) => {
            h1862
        }
    }) {
        Ok(0u64)
    } else {
        let a1871 = match v0.clone() {
            Adt3::C0(h1863, _, _, _, _, _, _, _) => {
                h1863
            }
        };
        let a1872 = {
            let a1869 = match v0.clone() {
                Adt3::C0(_, h1864, _, _, _, _, _, _) => {
                    h1864
                }
            };
            let a1870 = {
                let a1867 = match v0.clone() {
                    Adt3::C0(_, _, h1865, _, _, _, _, _) => {
                        h1865
                    }
                };
                let a1868 = match v0 {
                    Adt3::C0(_, _, _, h1866, _, _, _, _) => {
                        h1866
                    }
                };
                nat_mul(a1867, a1868)?
            };
            nat_mul(a1869, a1870)?
        };
        nat_mul(a1871, a1872)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1873 = v1;
    let a1874 = 13u64;
    if if nat_eq(a1873, a1874) {
        f23(match v0.clone() {
            Adt3::C0(h1875, _, _, _, _, _, _, _) => {
                h1875
            }
        }, match v0.clone() {
            Adt3::C0(_, h1876, _, _, _, _, _, _) => {
                h1876
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1877, _, _, _, _, _) => {
                h1877
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1878, _, _, _, _) => {
                h1878
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1884 = match v0.clone() {
                Adt3::C0(_, h1879, _, _, _, _, _, _) => {
                    h1879
                }
            };
            let a1885 = {
                let a1882 = match v0.clone() {
                    Adt3::C0(_, _, h1880, _, _, _, _, _) => {
                        h1880
                    }
                };
                let a1883 = match v0.clone() {
                    Adt3::C0(_, _, _, h1881, _, _, _, _) => {
                        h1881
                    }
                };
                nat_mul(a1882, a1883)?
            };
            nat_mul(a1884, a1885)?
        };
        let a1886 = v1;
        let a1887 = 13u64;
        if nat_eq(a1886, a1887) {
            let a1896 = 2u64;
            let a1897 = if f123(v0.clone(), v2) {
                let a1889 = v3;
                let a1890 = f124(match v0 {
                    Adt3::C0(h1888, _, _, _, _, _, _, _) => {
                        h1888
                    }
                }, false)?;
                nat_mul(a1889, a1890)?
            } else {
                let a1894 = v3;
                let a1895 = {
                    let a1892 = match v0 {
                        Adt3::C0(h1891, _, _, _, _, _, _, _) => {
                            h1891
                        }
                    };
                    let a1893 = 2u64;
                    nat_mul(a1892, a1893)?
                };
                nat_mul(a1894, a1895)?
            };
            nat_add(a1896, a1897)
        } else {
            let a1901 = v3;
            let a1902 = f124(match v0 {
                Adt3::C0(h1898, _, _, _, _, _, _, _) => {
                    h1898
                }
            }, {
                let a1899 = v1;
                let a1900 = 16u64;
                nat_eq(a1899, a1900)
            })?;
            nat_mul(a1901, a1902)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1903 = v3;
    match m1903 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1904, _, _, _, _, _, _, _, _, _, _) => {
                    h1904
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1905, _, _, _, _, _, _, _, _, _, _) => {
                    h1905
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1906, _, _, _, _, _, _, _, _, _) => {
                    h1906
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1907, _, _, _, _, _, _, _, _) => {
                    h1907
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1908, _, _, _, _, _, _, _) => {
                    h1908
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1909, _, _, _, _, _, _) => {
                    h1909
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1910, _, _, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1911, _, _, _, _, _, _, _, _) => {
                    h1911
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1912, _, _, _, _, _, _, _) => {
                    h1912
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1913, _, _, _, _, _, _) => {
                    h1913
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1914, _, _, _, _, _, _, _, _, _, _) => {
                    h1914
                }
            }, match v5.clone() {
                Adt2::C0(h1915, _, _, _, _, _, _, _, _, _, _) => {
                    h1915
                }
            }, match v4.clone() {
                Adt2::C0(h1916, _, _, _, _, _, _, _, _, _, _) => {
                    h1916
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1917, _, _, _, _, _, _, _, _, _) => {
                    h1917
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1918, _, _, _, _, _, _, _, _) => {
                    h1918
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1919, _, _, _, _, _, _, _) => {
                    h1919
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1920, _, _, _, _, _, _) => {
                    h1920
                }
            }, match v5.clone() {
                Adt2::C0(_, h1921, _, _, _, _, _, _, _, _, _) => {
                    h1921
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1922, _, _, _, _, _, _, _, _) => {
                    h1922
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1923, _, _, _, _, _, _, _) => {
                    h1923
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1924, _, _, _, _, _, _) => {
                    h1924
                }
            }, match v4.clone() {
                Adt2::C0(_, h1925, _, _, _, _, _, _, _, _, _) => {
                    h1925
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1926, _, _, _, _, _, _, _, _) => {
                    h1926
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1927, _, _, _, _, _, _, _) => {
                    h1927
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1928, _, _, _, _, _, _) => {
                    h1928
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1929 = v7;
        let a1930 = 6u64;
        if nat_eq(a1929, a1930) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1931 = v0;
    match m1931 {
        Adt4::C0 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C1 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C2(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C3(_, _, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C4(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C5(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C6(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C7(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C8(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C9(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C10(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C11(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C12(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C13(v30, v31, v32, v33) => {
            Adt7::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a1932 = 13u64;
            let a1933 = v34;
            if nat_le(a1932, a1933) {
                Adt7::C7(v34, v1, v2)
            } else {
                Adt7::C3(v1, v2, true)
            }
        }
        Adt4::C15 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C16(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C17 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C18(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C19(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C20(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C21 => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C22(_, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C23(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C24(_) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C25(_, _, _) => {
            Adt7::C2(v1, v2, true)
        }
        Adt4::C26(_, _, _, _) => {
            Adt7::C2(v1, v2, true)
        }
    }
}

pub fn f86(v0: Adt4) -> u64 {
    let m1934 = v0;
    match m1934 {
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
    }
}

pub fn f87(v0: Adt4, v1: u64) -> u64 {
    let m1935 = v0;
    match m1935 {
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
            let a1936 = v1;
            let a1937 = 6u64;
            if nat_eq(a1936, a1937) {
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
        Adt4::C26(v49, _, _, _) => {
            let a1938 = v1;
            let a1939 = 6u64;
            if if if nat_eq(a1938, a1939) {
                true
            } else {
                let a1940 = v1;
                let a1941 = 7u64;
                nat_eq(a1940, a1941)
            } {
                let a1942 = v49;
                let a1943 = 30u64;
                nat_eq(a1942, a1943)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
    }
}

pub fn f88(v0: Adt4, v1: u64) -> Adt7 {
    let a1944 = v1;
    let a1945 = 4u64;
    if nat_lt(a1944, a1945) {
        Adt7::C0(v1)
    } else {
        let a1946 = v1;
        let a1947 = 4u64;
        if nat_eq(a1946, a1947) {
            Adt7::C1(0u64)
        } else {
            let a1948 = v1;
            let a1949 = 5u64;
            if nat_eq(a1948, a1949) {
                Adt7::C1(3u64)
            } else {
                let a1950 = v1;
                let a1951 = 6u64;
                if nat_eq(a1950, a1951) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1952 = v1;
                    let a1953 = 7u64;
                    if nat_eq(a1952, a1953) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1954 = v1;
                        let a1955 = 16u64;
                        if nat_lt(a1954, a1955) {
                            Adt7::C1({
                                let a1956 = v1;
                                let a1957 = 8u64;
                                nat_sub(a1956, a1957)
                            })
                        } else {
                            let a1958 = v1;
                            let a1959 = 20u64;
                            if nat_lt(a1958, a1959) {
                                Adt7::C5({
                                    let a1960 = v1;
                                    let a1961 = 16u64;
                                    nat_sub(a1960, a1961)
                                })
                            } else {
                                let a1962 = v1;
                                let a1963 = 20u64;
                                if nat_eq(a1962, a1963) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1964 = v1;
                                    let a1965 = 21u64;
                                    if nat_eq(a1964, a1965) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1966 = v1;
                                        let a1967 = 22u64;
                                        if nat_eq(a1966, a1967) {
                                            Adt7::C0(5u64)
                                        } else {
                                            Adt7::C6
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f89(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1969 = v0;
        let a1970 = match v1.clone() {
            Adt3::C0(h1968, _, _, _, _, _, _, _) => {
                h1968
            }
        };
        let a1971 = 0u64;
        nat_quot(a1969, a1970, a1971)
    };
    let v3: u64 = {
        let a1973 = v2;
        let a1974 = match v1.clone() {
            Adt3::C0(_, h1972, _, _, _, _, _, _) => {
                h1972
            }
        };
        let a1975 = 0u64;
        nat_quot(a1973, a1974, a1975)
    };
    f129({
        let a1977 = v0;
        let a1978 = match v1.clone() {
            Adt3::C0(h1976, _, _, _, _, _, _, _) => {
                h1976
            }
        };
        let a1979 = 0u64;
        nat_rem(a1977, a1978, a1979)
    }, {
        let a1981 = v2;
        let a1982 = match v1.clone() {
            Adt3::C0(_, h1980, _, _, _, _, _, _) => {
                h1980
            }
        };
        let a1983 = 0u64;
        nat_rem(a1981, a1982, a1983)
    }, {
        let a1985 = v3;
        let a1986 = match v1.clone() {
            Adt3::C0(_, _, h1984, _, _, _, _, _) => {
                h1984
            }
        };
        let a1987 = 0u64;
        nat_rem(a1985, a1986, a1987)
    }, {
        let a1989 = v3;
        let a1990 = match v1.clone() {
            Adt3::C0(_, _, h1988, _, _, _, _, _) => {
                h1988
            }
        };
        let a1991 = 0u64;
        nat_quot(a1989, a1990, a1991)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1992, _, _, _) => {
            h1992
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1993, _, _) => {
            h1993
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1994, _) => {
            h1994
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1995) => {
            h1995
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2013 = {
        let a2008 = f130(v2, {
            let a1999 = {
                let a1996 = v0;
                let a1997 = v1;
                let a1998 = 0u64;
                nat_quot(a1996, a1997, a1998)
            };
            let a2000 = v2;
            let a2001 = 0u64;
            nat_rem(a1999, a2000, a2001)
        }, {
            let a2005 = {
                let a2002 = v0;
                let a2003 = v1;
                let a2004 = 0u64;
                nat_quot(a2002, a2003, a2004)
            };
            let a2006 = v2;
            let a2007 = 0u64;
            nat_quot(a2005, a2006, a2007)
        })?;
        let a2009 = v1;
        nat_mul(a2008, a2009)?
    };
    let a2014 = {
        let a2010 = v0;
        let a2011 = v1;
        let a2012 = 0u64;
        nat_rem(a2010, a2011, a2012)
    };
    nat_add(a2013, a2014)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2016 = v0;
        let a2017 = match v2.clone() {
            Adt3::C0(h2015, _, _, _, _, _, _, _) => {
                h2015
            }
        };
        let a2018 = 0u64;
        nat_quot(a2016, a2017, a2018)
    };
    let v4: u64 = {
        let a2020 = v3;
        let a2021 = match v2.clone() {
            Adt3::C0(_, h2019, _, _, _, _, _, _) => {
                h2019
            }
        };
        let a2022 = 0u64;
        nat_quot(a2020, a2021, a2022)
    };
    f131({
        let a2024 = v0;
        let a2025 = match v2.clone() {
            Adt3::C0(h2023, _, _, _, _, _, _, _) => {
                h2023
            }
        };
        let a2026 = 0u64;
        nat_rem(a2024, a2025, a2026)
    }, {
        let a2028 = v3;
        let a2029 = match v2.clone() {
            Adt3::C0(_, h2027, _, _, _, _, _, _) => {
                h2027
            }
        };
        let a2030 = 0u64;
        nat_rem(a2028, a2029, a2030)
    }, {
        let a2032 = v4;
        let a2033 = match v2.clone() {
            Adt3::C0(_, _, h2031, _, _, _, _, _) => {
                h2031
            }
        };
        let a2034 = 0u64;
        nat_rem(a2032, a2033, a2034)
    }, {
        let a2036 = v4;
        let a2037 = match v2 {
            Adt3::C0(_, _, h2035, _, _, _, _, _) => {
                h2035
            }
        };
        let a2038 = 0u64;
        nat_quot(a2036, a2037, a2038)
    }, match v1.clone() {
        Adt3::C0(h2039, _, _, _, _, _, _, _) => {
            h2039
        }
    }, match v1.clone() {
        Adt3::C0(_, h2040, _, _, _, _, _, _) => {
            h2040
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2041, _, _, _, _, _) => {
            h2041
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2042, _, _, _, _) => {
            h2042
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2043, _, _, _) => {
            h2043
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2044, _, _) => {
            h2044
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2045, _) => {
            h2045
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2046) => {
            h2046
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2048 = v0;
        let a2049 = match v3.clone() {
            Adt3::C0(h2047, _, _, _, _, _, _, _) => {
                h2047
            }
        };
        let a2050 = 0u64;
        nat_quot(a2048, a2049, a2050)
    };
    let v6: u64 = {
        let a2052 = v5;
        let a2053 = match v3.clone() {
            Adt3::C0(_, h2051, _, _, _, _, _, _) => {
                h2051
            }
        };
        let a2054 = 0u64;
        nat_quot(a2052, a2053, a2054)
    };
    let v7: u64 = {
        let a2056 = v0;
        let a2057 = match v3.clone() {
            Adt3::C0(h2055, _, _, _, _, _, _, _) => {
                h2055
            }
        };
        let a2058 = 0u64;
        nat_rem(a2056, a2057, a2058)
    };
    let v8: u64 = {
        let a2060 = v5;
        let a2061 = match v3.clone() {
            Adt3::C0(_, h2059, _, _, _, _, _, _) => {
                h2059
            }
        };
        let a2062 = 0u64;
        nat_rem(a2060, a2061, a2062)
    };
    let v9: u64 = {
        let a2064 = v6;
        let a2065 = match v3.clone() {
            Adt3::C0(_, _, h2063, _, _, _, _, _) => {
                h2063
            }
        };
        let a2066 = 0u64;
        nat_rem(a2064, a2065, a2066)
    };
    let v10: u64 = {
        let a2068 = v6;
        let a2069 = match v3.clone() {
            Adt3::C0(_, _, h2067, _, _, _, _, _) => {
                h2067
            }
        };
        let a2070 = 0u64;
        nat_quot(a2068, a2069, a2070)
    };
    let v11: u64 = {
        let a2071 = v4;
        let a2072 = 0u64;
        if nat_eq(a2071, a2072) {
            match v1.clone() {
                Adt3::C0(h2073, _, _, _, _, _, _, _) => {
                    h2073
                }
            }
        } else {
            let a2074 = v4;
            let a2075 = 1u64;
            if nat_eq(a2074, a2075) {
                match v1.clone() {
                    Adt3::C0(_, h2076, _, _, _, _, _, _) => {
                        h2076
                    }
                }
            } else {
                let a2077 = v4;
                let a2078 = 2u64;
                if nat_eq(a2077, a2078) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2079, _, _, _, _, _) => {
                            h2079
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2080, _, _, _, _) => {
                            h2080
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2087 = v11;
        let a2088 = {
            let a2081 = v4;
            let a2082 = 0u64;
            if nat_eq(a2081, a2082) {
                v7
            } else {
                let a2083 = v4;
                let a2084 = 1u64;
                if nat_eq(a2083, a2084) {
                    v8
                } else {
                    let a2085 = v4;
                    let a2086 = 2u64;
                    if nat_eq(a2085, a2086) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2087, a2088)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2089 = v4;
        let a2090 = 0u64;
        nat_eq(a2089, a2090)
    } else {
        false
    } {
        let a2091 = v7;
        let a2092 = v11;
        nat_sub(a2091, a2092)
    } else {
        v7
    }, if if v12 {
        let a2093 = v4;
        let a2094 = 1u64;
        nat_eq(a2093, a2094)
    } else {
        false
    } {
        let a2095 = v8;
        let a2096 = v11;
        nat_sub(a2095, a2096)
    } else {
        v8
    }, if if v12 {
        let a2097 = v4;
        let a2098 = 2u64;
        nat_eq(a2097, a2098)
    } else {
        false
    } {
        let a2099 = v9;
        let a2100 = v11;
        nat_sub(a2099, a2100)
    } else {
        v9
    }, if if v12 {
        let a2101 = v4;
        let a2102 = 3u64;
        nat_eq(a2101, a2102)
    } else {
        false
    } {
        let a2103 = v10;
        let a2104 = v11;
        nat_sub(a2103, a2104)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2105, _, _, _) => {
            h2105
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2106, _, _) => {
            h2106
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2107, _) => {
            h2107
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2108) => {
            h2108
        }
    })?, f89(v0, v3)?))
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f132(v0, v2.clone(), 0u64);
    let v9: u64 = f132(v0, v2.clone(), 1u64);
    let v10: u64 = f132(v0, v2.clone(), 2u64);
    let v11: u64 = f132(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f133(v1.clone(), f134(v8, v3, match v1.clone() {
            Adt3::C0(h2109, _, _, _, _, _, _, _) => {
                h2109
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2110, _, _, _, _, _, _) => {
                h2110
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2111, _, _, _, _, _) => {
                h2111
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2112, _, _, _, _) => {
                h2112
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2113, _, _, _, _, _, _, _) => {
                h2113
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2114, _, _, _, _, _, _) => {
                    h2114
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2115, _, _, _, _, _) => {
                    h2115
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2116, _, _, _, _) => {
                    h2116
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f133(v1, f136(v8, v3)?, f136(v9, v4)?, f136(v10, v5)?, f136(v11, v6)?)?, f89(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
        }
    }
}

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f132(v0, v2.clone(), 0u64);
    let v5: u64 = f132(v0, v2.clone(), 1u64);
    let v6: u64 = f132(v0, v2.clone(), 2u64);
    let v7: u64 = f132(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f133(v1.clone(), f137(v4, v3, match v1 {
        Adt3::C0(h2117, _, _, _, _, _, _, _) => {
            h2117
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2118 = v0;
    let a2119 = v3;
    if nat_lt(a2118, a2119) {
        Ok(Adt5::C1(false, {
            let a2120 = v0;
            let a2121 = 4u64;
            nat_mul(a2120, a2121)?
        }, {
            let a2122 = v0;
            let a2123 = 4u64;
            nat_mul(a2122, a2123)?
        }))
    } else {
        let v6: u64 = {
            let a2124 = v0;
            let a2125 = v3;
            nat_sub(a2124, a2125)
        };
        let v7: u64 = f89(v6, v1)?;
        let v8: u64 = f138(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2126 = v0;
    let a2127 = v3;
    if nat_lt(a2126, a2127) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2128 = v0;
            let a2129 = v3;
            nat_sub(a2128, a2129)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2131 = v1;
        let a2132 = match v3.clone() {
            Adt3::C0(h2130, _, _, _, _, _, _, _) => {
                h2130
            }
        };
        let a2133 = 0u64;
        nat_rem(a2131, a2132, a2133)
    };
    let v5: u64 = {
        let a2139 = {
            let a2135 = v1;
            let a2136 = match v3.clone() {
                Adt3::C0(h2134, _, _, _, _, _, _, _) => {
                    h2134
                }
            };
            let a2137 = 0u64;
            nat_quot(a2135, a2136, a2137)
        };
        let a2140 = match v3.clone() {
            Adt3::C0(_, h2138, _, _, _, _, _, _) => {
                h2138
            }
        };
        let a2141 = 0u64;
        nat_rem(a2139, a2140, a2141)
    };
    let v6: bool = f139(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f91(v1, v2, v3.clone())?
    } else {
        0u64
    }, f89(v1, v3)?))
}

pub fn f98(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a2142 = v3;
        let a2143 = 8u64;
        nat_add(a2142, a2143)?
    }, 0u64, f89(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f91(v0, v5, v2.clone())?
    }, 3u64, f89(v0, v2)?))
}

pub fn f99(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a2145 = v0;
        let a2146 = match v2.clone() {
            Adt3::C0(h2144, _, _, _, _, _, _, _) => {
                h2144
            }
        };
        let a2147 = 0u64;
        nat_rem(a2145, a2146, a2147)
    };
    let v7: u64 = {
        let a2149 = v0;
        let a2150 = match v2.clone() {
            Adt3::C0(h2148, _, _, _, _, _, _, _) => {
                h2148
            }
        };
        let a2151 = 0u64;
        nat_quot(a2149, a2150, a2151)
    };
    let v8: u64 = {
        let a2153 = v7;
        let a2154 = match v2.clone() {
            Adt3::C0(_, h2152, _, _, _, _, _, _) => {
                h2152
            }
        };
        let a2155 = 0u64;
        nat_rem(a2153, a2154, a2155)
    };
    let v9: u64 = {
        let a2157 = v7;
        let a2158 = match v2.clone() {
            Adt3::C0(_, h2156, _, _, _, _, _, _) => {
                h2156
            }
        };
        let a2159 = 0u64;
        nat_quot(a2157, a2158, a2159)
    };
    let v10: u64 = {
        let a2161 = v9;
        let a2162 = match v2.clone() {
            Adt3::C0(_, _, h2160, _, _, _, _, _) => {
                h2160
            }
        };
        let a2163 = 0u64;
        nat_rem(a2161, a2162, a2163)
    };
    let v11: u64 = {
        let a2165 = v9;
        let a2166 = match v2.clone() {
            Adt3::C0(_, _, h2164, _, _, _, _, _) => {
                h2164
            }
        };
        let a2167 = 0u64;
        nat_quot(a2165, a2166, a2167)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2176 = {
                let a2171 = {
                    let a2168 = v10;
                    let a2169 = v4;
                    let a2170 = 0u64;
                    nat_quot(a2168, a2169, a2170)
                };
                let a2172 = v5;
                nat_mul(a2171, a2172)?
            };
            let a2177 = {
                let a2173 = v8;
                let a2174 = v4;
                let a2175 = 0u64;
                nat_quot(a2173, a2174, a2175)
            };
            nat_add(a2176, a2177)?
        };
        let v14: u64 = {
            let a2197 = v6;
            let a2198 = {
                let a2195 = match v1.clone() {
                    Adt3::C0(h2178, _, _, _, _, _, _, _) => {
                        h2178
                    }
                };
                let a2196 = {
                    let a2193 = {
                        let a2179 = v8;
                        let a2180 = v4;
                        let a2181 = 0u64;
                        nat_rem(a2179, a2180, a2181)
                    };
                    let a2194 = {
                        let a2191 = match v1.clone() {
                            Adt3::C0(_, h2182, _, _, _, _, _, _) => {
                                h2182
                            }
                        };
                        let a2192 = {
                            let a2189 = {
                                let a2183 = v10;
                                let a2184 = v4;
                                let a2185 = 0u64;
                                nat_rem(a2183, a2184, a2185)
                            };
                            let a2190 = {
                                let a2187 = match v1.clone() {
                                    Adt3::C0(_, _, h2186, _, _, _, _, _) => {
                                        h2186
                                    }
                                };
                                let a2188 = v13;
                                nat_mul(a2187, a2188)?
                            };
                            nat_add(a2189, a2190)?
                        };
                        nat_mul(a2191, a2192)?
                    };
                    nat_add(a2193, a2194)?
                };
                nat_mul(a2195, a2196)?
            };
            nat_add(a2197, a2198)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2204 = {
                let a2202 = {
                    let a2199 = v11;
                    let a2200 = v5;
                    let a2201 = 0u64;
                    nat_rem(a2199, a2200, a2201)
                };
                let a2203 = v4;
                nat_mul(a2202, a2203)?
            };
            let a2205 = v8;
            nat_add(a2204, a2205)?
        };
        let v16: u64 = {
            let a2211 = {
                let a2209 = {
                    let a2206 = v11;
                    let a2207 = v5;
                    let a2208 = 0u64;
                    nat_quot(a2206, a2207, a2208)
                };
                let a2210 = v4;
                nat_mul(a2209, a2210)?
            };
            let a2212 = v10;
            nat_add(a2211, a2212)?
        };
        let a2214 = v15;
        let a2215 = match v1.clone() {
            Adt3::C0(_, h2213, _, _, _, _, _, _) => {
                h2213
            }
        };
        if if nat_lt(a2214, a2215) {
            let a2217 = v16;
            let a2218 = match v1.clone() {
                Adt3::C0(_, _, h2216, _, _, _, _, _) => {
                    h2216
                }
            };
            nat_lt(a2217, a2218)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2227 = v6;
                let a2228 = {
                    let a2225 = match v1.clone() {
                        Adt3::C0(h2219, _, _, _, _, _, _, _) => {
                            h2219
                        }
                    };
                    let a2226 = {
                        let a2223 = v15;
                        let a2224 = {
                            let a2221 = match v1.clone() {
                                Adt3::C0(_, h2220, _, _, _, _, _, _) => {
                                    h2220
                                }
                            };
                            let a2222 = v16;
                            nat_mul(a2221, a2222)?
                        };
                        nat_add(a2223, a2224)?
                    };
                    nat_mul(a2225, a2226)?
                };
                nat_add(a2227, a2228)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2233 = {
        let a2230 = v0;
        let a2231 = match v1 {
            Adt3::C0(h2229, _, _, _, _, _, _, _) => {
                h2229
            }
        };
        let a2232 = 0u64;
        nat_rem(a2230, a2231, a2232)
    };
    let a2234 = 0u64;
    if nat_eq(a2233, a2234) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2235 = v0;
            let a2236 = 1u64;
            nat_sub(a2235, a2236)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2237 = v3;
    let a2238 = 13u64;
    if nat_eq(a2237, a2238) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2239, _, _, _, _, _, _, _) => {
                h2239
            }
        }, {
            let a2240 = v3;
            let a2241 = 16u64;
            nat_eq(a2240, a2241)
        })?;
        let v6: u64 = {
            let a2242 = v0;
            let a2243 = v5;
            let a2244 = 0u64;
            nat_quot(a2242, a2243, a2244)
        };
        let v7: u64 = {
            let a2245 = v0;
            let a2246 = v5;
            let a2247 = 0u64;
            nat_rem(a2245, a2246, a2247)
        };
        let v8: u64 = {
            let a2251 = 1u64;
            let a2252 = {
                let a2249 = match v1.clone() {
                    Adt3::C0(h2248, _, _, _, _, _, _, _) => {
                        h2248
                    }
                };
                let a2250 = 2u64;
                nat_mul(a2249, a2250)?
            };
            nat_add(a2251, a2252)?
        };
        let a2253 = v7;
        let a2254 = v8;
        if nat_lt(a2253, a2254) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2255 = v3;
            let a2256 = 14u64;
            if nat_eq(a2255, a2256) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2257 = v7;
                let a2258 = v8;
                if nat_eq(a2257, a2258) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2261 = v7;
                    let a2262 = {
                        let a2259 = v8;
                        let a2260 = 1u64;
                        nat_add(a2259, a2260)?
                    };
                    if nat_eq(a2261, a2262) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2263, _, _, _, _, _, _, _) => {
                                h2263
                            }
                        }, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(5u64, 1u64, 1u64, 1u64, 2u64, 3u64, v9))
                    }
                }
            }
        }
    }
}

pub fn f102(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2264 = v0;
    let a2265 = 0u64;
    if nat_eq(a2264, a2265) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2272 = {
            let a2266 = v0;
            let a2267 = 1u64;
            nat_add(a2266, a2267)?
        };
        let a2273 = {
            let a2270 = 2u64;
            let a2271 = {
                let a2268 = v3;
                let a2269 = 2u64;
                nat_mul(a2268, a2269)?
            };
            nat_add(a2270, a2271)?
        };
        if nat_eq(a2272, a2273) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2274 = v0;
                let a2275 = 1u64;
                nat_sub(a2274, a2275)
            };
            let v5: u64 = {
                let a2276 = v4;
                let a2277 = 2u64;
                let a2278 = 0u64;
                nat_quot(a2276, a2277, a2278)
            };
            let a2282 = {
                let a2279 = v4;
                let a2280 = 2u64;
                let a2281 = 0u64;
                nat_rem(a2279, a2280, a2281)
            };
            let a2283 = 0u64;
            if nat_eq(a2282, a2283) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2284 = v0;
    let a2285 = f18(v1, v2, v3)?;
    if nat_lt(a2284, a2285) {
        let a2286 = v0;
        let a2287 = v1;
        if nat_lt(a2286, a2287) {
            Ok(Adt5::C1(false, {
                let a2288 = 4u64;
                let a2289 = v0;
                nat_mul(a2288, a2289)?
            }, {
                let a2290 = 4u64;
                let a2291 = v0;
                nat_mul(a2290, a2291)?
            }))
        } else {
            let v4: u64 = {
                let a2292 = v0;
                let a2293 = v1;
                nat_sub(a2292, a2293)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2294 = v5;
            let a2295 = 2u64;
            if nat_lt(a2294, a2295) {
                Ok(Adt5::C4(6u64, {
                    let a2296 = v5;
                    let a2297 = 0u64;
                    if nat_eq(a2296, a2297) {
                        4u64
                    } else {
                        5u64
                    }
                }, f143(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f144(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f145(v4, v2)? {
                    1u64
                } else {
                    0u64
                }, 3u64, v6))
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f104(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2305 = {
        let a2298 = 2u64;
        let a2299 = f81(v1.clone())?;
        nat_mul(a2298, a2299)?
    };
    let a2306 = {
        let a2303 = f81(v1)?;
        let a2304 = {
            let a2301 = 1u64;
            let a2302 = match v0 {
                Adt3::C0(h2300, _, _, _, _, _, _, _) => {
                    h2300
                }
            };
            nat_add(a2301, a2302)?
        };
        nat_mul(a2303, a2304)?
    };
    nat_add(a2305, a2306)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2307 = v1;
    let a2308 = 2147483647u64;
    if nat_le(a2307, a2308) {
        let a2309 = v1;
        let a2310 = v0;
        nat_lt(a2309, a2310)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2311 = f70(v0);
    match m2311 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2313 = v2;
                let a2314 = match v7.clone() {
                    Adt11::C0(_, h2312, _) => {
                        h2312
                    }
                };
                nat_mul(a2313, a2314)?
            };
            let m2327 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2315, _, _, _, _, _, _, _) => {
                    h2315
                }
            }, match v5.clone() {
                Adt3::C0(_, h2316, _, _, _, _, _, _) => {
                    h2316
                }
            }, match v5 {
                Adt3::C0(_, _, h2317, _, _, _, _, _) => {
                    h2317
                }
            }, {
                let a2320 = match v3.clone() {
                    Adt3::C0(h2318, _, _, _, _, _, _, _) => {
                        h2318
                    }
                };
                let a2321 = match v7.clone() {
                    Adt11::C0(_, h2319, _) => {
                        h2319
                    }
                };
                nat_mul(a2320, a2321)?
            }, match v3.clone() {
                Adt3::C0(_, h2322, _, _, _, _, _, _) => {
                    h2322
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2323, _, _, _) => {
                    h2323
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2324, _, _) => {
                    h2324
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2325, _) => {
                    h2325
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2326) => {
                    h2326
                }
            })?;
            match m2327 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2328 = v0;
                    let a2329 = 0u64;
                    if if nat_eq(a2328, a2329) {
                        true
                    } else {
                        let a2330 = v0;
                        let a2331 = 26u64;
                        nat_eq(a2330, a2331)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2335 = {
                                let a2333 = v1;
                                let a2334 = match v4.clone() {
                                    Adt3::C0(h2332, _, _, _, _, _, _, _) => {
                                        h2332
                                    }
                                };
                                nat_mul(a2333, a2334)?
                            };
                            let a2336 = v8;
                            nat_add(a2335, a2336)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2337) => {
                                h2337
                            }
                        }, f89({
                            let a2341 = {
                                let a2339 = v1;
                                let a2340 = match v4.clone() {
                                    Adt3::C0(h2338, _, _, _, _, _, _, _) => {
                                        h2338
                                    }
                                };
                                nat_mul(a2339, a2340)?
                            };
                            let a2342 = v8;
                            nat_add(a2341, a2342)?
                        }, v4.clone())?, {
                            let a2349 = {
                                let a2347 = {
                                    let a2344 = match v7 {
                                        Adt11::C0(_, h2343, _) => {
                                            h2343
                                        }
                                    };
                                    let a2345 = 1u64;
                                    nat_sub(a2344, a2345)
                                };
                                let a2348 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2346, _, _, _) => {
                                        h2346
                                    }
                                };
                                nat_mul(a2347, a2348)?
                            };
                            let a2350 = 4u64;
                            nat_add(a2349, a2350)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2351, _, _, _) => {
                                h2351
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2352 = v1;
    let a2353 = 0u64;
    if if nat_eq(a2352, a2353) {
        true
    } else {
        let a2354 = v2;
        let a2355 = 0u64;
        nat_eq(a2354, a2355)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2356 = v0;
    let a2357 = 4294967295u64;
    if nat_eq(a2356, a2357) {
        true
    } else {
        let a2358 = v0;
        let a2359 = 2147483648u64;
        if nat_lt(a2358, a2359) {
            let a2360 = v0;
            let a2361 = v1;
            nat_lt(a2360, a2361)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2368 = v3;
        let a2369 = {
            let a2365 = f89({
                let a2362 = v0;
                let a2363 = 2u64;
                let a2364 = 0u64;
                nat_quot(a2362, a2363, a2364)
            }, v1)?;
            let a2366 = v2;
            let a2367 = 0u64;
            nat_quot(a2365, a2366, a2367)
        };
        nat_add(a2368, a2369)?
    };
    let a2373 = {
        let a2370 = v0;
        let a2371 = 2u64;
        let a2372 = 0u64;
        nat_rem(a2370, a2371, a2372)
    };
    let a2374 = 0u64;
    if nat_eq(a2373, a2374) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2375 = v4;
        let a2376 = 0u64;
        if nat_eq(a2375, a2376) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2379 = 8u64;
    let a2380 = {
        let a2377 = f147(v0);
        let a2378 = v1;
        nat_mul(a2377, a2378)?
    };
    nat_add(a2379, a2380)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2381, _, _, _, _, _, _, _) => {
            h2381
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2384 = 1u64;
        let a2385 = {
            let a2382 = v6;
            let a2383 = v7;
            nat_mul(a2382, a2383)?
        };
        nat_add(a2384, a2385)?
    };
    let a2386 = v0;
    let a2387 = 0u64;
    if nat_eq(a2386, a2387) {
        Ok(f148(0u64))
    } else {
        let a2388 = v0;
        let a2389 = v8;
        if nat_lt(a2388, a2389) {
            let v9: u64 = {
                let a2390 = v0;
                let a2391 = 1u64;
                nat_sub(a2390, a2391)
            };
            let v10: u64 = {
                let a2392 = v9;
                let a2393 = v7;
                let a2394 = 0u64;
                nat_rem(a2392, a2393, a2394)
            };
            let v11: u64 = f89({
                let a2400 = {
                    let a2395 = v1;
                    let a2396 = v6;
                    nat_mul(a2395, a2396)?
                };
                let a2401 = {
                    let a2397 = v9;
                    let a2398 = v7;
                    let a2399 = 0u64;
                    nat_quot(a2397, a2398, a2399)
                };
                nat_add(a2400, a2401)?
            }, v2)?;
            let a2402 = v3;
            let a2403 = 30u64;
            if nat_eq(a2402, a2403) {
                let a2404 = v10;
                let a2405 = 0u64;
                if nat_eq(a2404, a2405) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2406 = v10;
                    let a2407 = 1u64;
                    if nat_eq(a2406, a2407) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2408 = v10;
                        let a2409 = 2u64;
                        if nat_eq(a2408, a2409) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2410 = v10;
                let a2411 = 0u64;
                if nat_eq(a2410, a2411) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2412 = v10;
                    let a2413 = 1u64;
                    if nat_eq(a2412, a2413) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2414 = v0;
                let a2415 = v8;
                nat_sub(a2414, a2415)
            };
            let a2416 = v12;
            let a2417 = 0u64;
            if nat_eq(a2416, a2417) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2418 = v12;
                let a2419 = 1u64;
                if nat_eq(a2418, a2419) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2420 = v12;
                    let a2421 = 2u64;
                    if nat_eq(a2420, a2421) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2422 = v12;
                        let a2423 = 3u64;
                        if nat_eq(a2422, a2423) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2424 = v12;
                            let a2425 = 4u64;
                            if nat_eq(a2424, a2425) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2426 = v12;
                                let a2427 = 5u64;
                                if nat_eq(a2426, a2427) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2428 = v12;
                                    let a2429 = 6u64;
                                    if nat_eq(a2428, a2429) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2430 = 3u64;
                                                let a2431 = v1;
                                                nat_add(a2430, a2431)?
                                            }))
                                        } else {
                                            Ok(Adt5::C0(Adt6::C1))
                                        }
                                    } else {
                                        Ok(Adt5::C0(Adt6::C1))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f113(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a2432 = v0;
        let a2433 = f65(v3);
        let a2434 = 0u64;
        nat_quot(a2432, a2433, a2434)
    };
    let v5: u64 = {
        let a2439 = 3u64;
        let a2440 = {
            let a2436 = v4;
            let a2437 = match v1.clone() {
                Adt3::C0(h2435, _, _, _, _, _, _, _) => {
                    h2435
                }
            };
            let a2438 = 0u64;
            nat_quot(a2436, a2437, a2438)
        };
        nat_add(a2439, a2440)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2441 = v3;
    let a2442 = 30u64;
    if nat_eq(a2441, a2442) {
        let a2446 = {
            let a2443 = v0;
            let a2444 = 3u64;
            let a2445 = 0u64;
            nat_rem(a2443, a2444, a2445)
        };
        let a2447 = 0u64;
        if nat_eq(a2446, a2447) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2451 = {
                let a2448 = v0;
                let a2449 = 3u64;
                let a2450 = 0u64;
                nat_rem(a2448, a2449, a2450)
            };
            let a2452 = 1u64;
            if nat_eq(a2451, a2452) {
                Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt5::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt5::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f114(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2453 = v0;
        let a2454 = 8388608u64;
        let a2455 = 0u64;
        nat_quot(a2453, a2454, a2455)
    };
    let a2456 = v0;
    let a2457 = 2147483648u64;
    if if if nat_lt(a2456, a2457) {
        let a2458 = 127u64;
        let a2459 = v1;
        nat_le(a2458, a2459)
    } else {
        false
    } {
        let a2460 = v1;
        let a2461 = 190u64;
        nat_lt(a2460, a2461)
    } else {
        false
    } {
        let v2: u64 = {
            let a2465 = 8388608u64;
            let a2466 = {
                let a2462 = v0;
                let a2463 = 8388608u64;
                let a2464 = 0u64;
                nat_rem(a2462, a2463, a2464)
            };
            nat_add(a2465, a2466)?
        };
        let a2467 = v1;
        let a2468 = 150u64;
        if nat_lt(a2467, a2468) {
            let v3: u64 = f150({
                let a2469 = 150u64;
                let a2470 = v1;
                nat_sub(a2469, a2470)
            });
            let a2474 = {
                let a2471 = v2;
                let a2472 = v3;
                let a2473 = 0u64;
                nat_rem(a2471, a2472, a2473)
            };
            let a2475 = 0u64;
            if nat_eq(a2474, a2475) {
                let a2476 = v2;
                let a2477 = v3;
                let a2478 = 0u64;
                Ok(nat_quot(a2476, a2477, a2478))
            } else {
                Ok(0u64)
            }
        } else {
            let a2481 = f150({
                let a2479 = v1;
                let a2480 = 150u64;
                nat_sub(a2479, a2480)
            });
            let a2482 = v2;
            nat_mul(a2481, a2482)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2483 = v0;
    let a2484 = 1u64;
    if nat_le(a2483, a2484) {
        true
    } else {
        let a2488 = {
            let a2485 = v1;
            let a2486 = v2;
            let a2487 = 1u64;
            nat_rem(a2485, a2486, a2487)
        };
        let a2489 = 0u64;
        nat_eq(a2488, a2489)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2490 = v1;
    let a2491 = 1u64;
    if if if nat_le(a2490, a2491) {
        true
    } else {
        let a2492 = v0;
        let a2493 = v2;
        nat_le(a2492, a2493)
    } {
        let a2494 = v3;
        let a2495 = 1u64;
        if nat_le(a2494, a2495) {
            true
        } else {
            let a2502 = {
                let a2500 = v0;
                let a2501 = {
                    let a2498 = {
                        let a2496 = v1;
                        let a2497 = 1u64;
                        nat_sub(a2496, a2497)
                    };
                    let a2499 = v2;
                    nat_mul(a2498, a2499)?
                };
                nat_add(a2500, a2501)?
            };
            let a2503 = v4;
            nat_le(a2502, a2503)
        }
    } else {
        false
    } {
        let a2504 = v5;
        let a2505 = 1u64;
        if nat_le(a2504, a2505) {
            Ok(true)
        } else {
            let a2518 = {
                let a2516 = {
                    let a2510 = v0;
                    let a2511 = {
                        let a2508 = {
                            let a2506 = v1;
                            let a2507 = 1u64;
                            nat_sub(a2506, a2507)
                        };
                        let a2509 = v2;
                        nat_mul(a2508, a2509)?
                    };
                    nat_add(a2510, a2511)?
                };
                let a2517 = {
                    let a2514 = {
                        let a2512 = v3;
                        let a2513 = 1u64;
                        nat_sub(a2512, a2513)
                    };
                    let a2515 = v4;
                    nat_mul(a2514, a2515)?
                };
                nat_add(a2516, a2517)?
            };
            let a2519 = v6;
            Ok(nat_le(a2518, a2519))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2520 = v5;
        let a2521 = 26u64;
        nat_eq(a2520, a2521)
    } else {
        false
    } {
        let a2522 = v3;
        let a2523 = v7;
        nat_eq(a2522, a2523)
    } else {
        false
    } {
        let a2524 = v4;
        let a2525 = v8;
        nat_eq(a2524, a2525)
    } else {
        false
    } {
        let a2526 = v9;
        let a2527 = 1u64;
        nat_eq(a2526, a2527)
    } else {
        false
    } {
        let a2528 = v10;
        let a2529 = f152(v0);
        nat_eq(a2528, a2529)
    } else {
        false
    } {
        let a2530 = v11;
        let a2531 = v1;
        nat_eq(a2530, a2531)
    } else {
        false
    } {
        let a2532 = v12;
        let a2533 = v6;
        nat_eq(a2532, a2533)
    } else {
        false
    } {
        let a2534 = v13;
        let a2535 = v7;
        nat_eq(a2534, a2535)
    } else {
        false
    } {
        let a2536 = v14;
        let a2537 = v8;
        nat_eq(a2536, a2537)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2538 = v0;
    let a2539 = 0u64;
    if if if nat_eq(a2538, a2539) {
        true
    } else {
        let a2540 = v1;
        let a2541 = 0u64;
        nat_eq(a2540, a2541)
    } {
        true
    } else {
        let a2542 = v2;
        let a2543 = 0u64;
        nat_eq(a2542, a2543)
    } {
        true
    } else {
        let a2544 = v3;
        let a2545 = 0u64;
        nat_eq(a2544, a2545)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2546 = v0;
    let a2547 = 2147483648u64;
    if nat_lt(a2546, a2547) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2548 = v0;
    let a2549 = 2147483648u64;
    if nat_lt(a2548, a2549) {
        0u64
    } else {
        let a2550 = 4294967296u64;
        let a2551 = v0;
        nat_sub(a2550, a2551)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2553 = match v0.clone() {
            Adt3::C0(h2552, _, _, _, _, _, _, _) => {
                h2552
            }
        };
        let a2554 = 1u64;
        nat_sub(a2553, a2554)
    }, {
        let a2556 = match v0.clone() {
            Adt3::C0(_, h2555, _, _, _, _, _, _) => {
                h2555
            }
        };
        let a2557 = 1u64;
        nat_sub(a2556, a2557)
    }, {
        let a2559 = match v0.clone() {
            Adt3::C0(_, _, h2558, _, _, _, _, _) => {
                h2558
            }
        };
        let a2560 = 1u64;
        nat_sub(a2559, a2560)
    }, {
        let a2562 = match v0 {
            Adt3::C0(_, _, _, h2561, _, _, _, _) => {
                h2561
            }
        };
        let a2563 = 1u64;
        nat_sub(a2562, a2563)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2564 = v0;
    let a2565 = 0u64;
    if nat_eq(a2564, a2565) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2566 = v0;
            let a2567 = 1u64;
            nat_sub(a2566, a2567)
        };
        let v5: u64 = {
            let a2568 = v4;
            let a2569 = v1;
            let a2570 = 0u64;
            nat_quot(a2568, a2569, a2570)
        };
        let v6: u64 = {
            let a2578 = {
                let a2571 = v5;
                let a2572 = v2;
                nat_mul(a2571, a2572)?
            };
            let a2579 = {
                let a2576 = {
                    let a2573 = v4;
                    let a2574 = v1;
                    let a2575 = 0u64;
                    nat_rem(a2573, a2574, a2575)
                };
                let a2577 = v3;
                nat_mul(a2576, a2577)?
            };
            nat_add(a2578, a2579)?
        };
        let a2580 = v5;
        let a2581 = 0u64;
        if nat_eq(a2580, a2581) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2590 = {
                    let a2584 = {
                        let a2582 = v5;
                        let a2583 = 1u64;
                        nat_sub(a2582, a2583)
                    };
                    let a2585 = v2;
                    nat_mul(a2584, a2585)?
                };
                let a2591 = {
                    let a2588 = {
                        let a2586 = v1;
                        let a2587 = 1u64;
                        nat_sub(a2586, a2587)
                    };
                    let a2589 = v3;
                    nat_mul(a2588, a2589)?
                };
                nat_add(a2590, a2591)?
            };
            let a2592 = v6;
            let a2593 = v7;
            if nat_lt(a2592, a2593) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2594 = v1;
    let a2595 = 0u64;
    if !nat_eq(a2594, a2595) {
        true
    } else {
        let a2597 = match v0 {
            Adt3::C0(_, _, _, _, h2596, _, _, _) => {
                h2596
            }
        };
        let a2598 = 4u64;
        nat_eq(a2597, a2598)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2601 = {
        let a2599 = v0;
        let a2600 = 2u64;
        nat_mul(a2599, a2600)?
    };
    let a2602 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2601, a2602)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2603 = f154(v0, v1);
    match m2603 {
        None => {
            if v5 {
                let m2604 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2604 {
                    Adt12::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt12::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt12::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a2605 = 4u64;
                let a2606 = v28;
                nat_le(a2605, a2606)
            };
            let a2607 = v2;
            let a2608 = 0u64;
            if if if if if nat_eq(a2607, a2608) {
                let a2609 = v4;
                let a2610 = 0u64;
                nat_eq(a2609, a2610)
            } else {
                false
            } {
                let a2611 = v6;
                let a2612 = v14;
                if nat_eq(a2611, a2612) {
                    let a2613 = v7;
                    let a2614 = v15;
                    nat_eq(a2613, a2614)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2615 = v8;
                let a2616 = v16;
                if nat_eq(a2615, a2616) {
                    let a2617 = v9;
                    let a2618 = v17;
                    nat_eq(a2617, a2618)
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
                        let a2619 = v3;
                        let a2620 = 0u64;
                        nat_eq(a2619, a2620)
                    } else {
                        false
                    } {
                        f71(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2625 = {
                        let a2621 = v6;
                        let a2622 = v7;
                        nat_mul(a2621, a2622)?
                    };
                    let a2626 = {
                        let a2623 = v8;
                        let a2624 = v9;
                        nat_mul(a2623, a2624)?
                    };
                    nat_mul(a2625, a2626)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2627 = v0;
    let a2628 = 0u64;
    if nat_eq(a2627, a2628) {
        Adt8::C0(0u64, v1)
    } else {
        let a2629 = v0;
        let a2630 = 1u64;
        if nat_eq(a2629, a2630) {
            Adt8::C1(v1)
        } else {
            let a2631 = v0;
            let a2632 = 2u64;
            if nat_eq(a2631, a2632) {
                Adt8::C2(v1)
            } else {
                let a2633 = v0;
                let a2634 = 3u64;
                if nat_eq(a2633, a2634) {
                    Adt8::C3(v1)
                } else {
                    let a2635 = v0;
                    let a2636 = 13u64;
                    if nat_lt(a2635, a2636) {
                        Adt8::C0({
                            let a2637 = v0;
                            let a2638 = 3u64;
                            nat_sub(a2637, a2638)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f127(v0: u64, v1: u64) -> Adt9 {
    let a2639 = v0;
    let a2640 = 1u64;
    if nat_eq(a2639, a2640) {
        Adt9::C0(v1)
    } else {
        let a2641 = v0;
        let a2642 = 3u64;
        if if nat_eq(a2641, a2642) {
            true
        } else {
            let a2643 = v0;
            let a2644 = 6u64;
            nat_eq(a2643, a2644)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2645 = f70(v0);
    match m2645 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2647 = match v1.clone() {
                Adt11::C0(_, h2646, _) => {
                    h2646
                }
            };
            let a2648 = 1u64;
            if if nat_eq(a2647, a2648) {
                let a2650 = match v1.clone() {
                    Adt11::C0(_, _, h2649) => {
                        h2649
                    }
                };
                let a2651 = 1u64;
                if if if nat_eq(a2650, a2651) {
                    true
                } else {
                    let a2653 = match v1.clone() {
                        Adt11::C0(_, _, h2652) => {
                            h2652
                        }
                    };
                    let a2654 = 2u64;
                    nat_eq(a2653, a2654)
                } {
                    true
                } else {
                    let a2656 = match v1.clone() {
                        Adt11::C0(_, _, h2655) => {
                            h2655
                        }
                    };
                    let a2657 = 4u64;
                    nat_eq(a2656, a2657)
                } {
                    true
                } else {
                    let a2659 = match v1.clone() {
                        Adt11::C0(_, _, h2658) => {
                            h2658
                        }
                    };
                    let a2660 = 8u64;
                    nat_eq(a2659, a2660)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2661) => {
                        h2661
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2674 = {
        let a2670 = {
            let a2666 = {
                let a2662 = v0;
                let a2663 = v4;
                nat_mul(a2662, a2663)?
            };
            let a2667 = {
                let a2664 = v1;
                let a2665 = v5;
                nat_mul(a2664, a2665)?
            };
            nat_add(a2666, a2667)?
        };
        let a2671 = {
            let a2668 = v2;
            let a2669 = v6;
            nat_mul(a2668, a2669)?
        };
        nat_add(a2670, a2671)?
    };
    let a2675 = {
        let a2672 = v3;
        let a2673 = v7;
        nat_mul(a2672, a2673)?
    };
    nat_add(a2674, a2675)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2680 = {
        let a2678 = {
            let a2676 = v0;
            let a2677 = v1;
            nat_sub(a2676, a2677)
        };
        let a2679 = 1u64;
        nat_sub(a2678, a2679)
    };
    let a2681 = v2;
    nat_add(a2680, a2681)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2682 = v0;
        let a2683 = v4;
        let a2684 = 0u64;
        nat_rem(a2682, a2683, a2684)
    }, {
        let a2685 = v1;
        let a2686 = v5;
        let a2687 = 0u64;
        nat_rem(a2685, a2686, a2687)
    }, {
        let a2688 = v2;
        let a2689 = v6;
        let a2690 = 0u64;
        nat_rem(a2688, a2689, a2690)
    }, {
        let a2691 = v3;
        let a2692 = v7;
        let a2693 = 0u64;
        nat_rem(a2691, a2692, a2693)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2695 = v0;
        let a2696 = match v1.clone() {
            Adt3::C0(h2694, _, _, _, _, _, _, _) => {
                h2694
            }
        };
        let a2697 = 0u64;
        nat_quot(a2695, a2696, a2697)
    };
    let v4: u64 = {
        let a2699 = v3;
        let a2700 = match v1.clone() {
            Adt3::C0(_, h2698, _, _, _, _, _, _) => {
                h2698
            }
        };
        let a2701 = 0u64;
        nat_quot(a2699, a2700, a2701)
    };
    let a2702 = v2;
    let a2703 = 0u64;
    if nat_eq(a2702, a2703) {
        let a2705 = v0;
        let a2706 = match v1 {
            Adt3::C0(h2704, _, _, _, _, _, _, _) => {
                h2704
            }
        };
        let a2707 = 0u64;
        nat_rem(a2705, a2706, a2707)
    } else {
        let a2708 = v2;
        let a2709 = 1u64;
        if nat_eq(a2708, a2709) {
            let a2711 = v3;
            let a2712 = match v1 {
                Adt3::C0(_, h2710, _, _, _, _, _, _) => {
                    h2710
                }
            };
            let a2713 = 0u64;
            nat_rem(a2711, a2712, a2713)
        } else {
            let a2714 = v2;
            let a2715 = 2u64;
            if nat_eq(a2714, a2715) {
                let a2717 = v4;
                let a2718 = match v1 {
                    Adt3::C0(_, _, h2716, _, _, _, _, _) => {
                        h2716
                    }
                };
                let a2719 = 0u64;
                nat_rem(a2717, a2718, a2719)
            } else {
                let a2721 = v4;
                let a2722 = match v1 {
                    Adt3::C0(_, _, h2720, _, _, _, _, _) => {
                        h2720
                    }
                };
                let a2723 = 0u64;
                nat_quot(a2721, a2722, a2723)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2724, _, _, _) => {
            h2724
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2725, _, _) => {
            h2725
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2726, _) => {
            h2726
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2727) => {
            h2727
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2728 = v1;
    let a2729 = 2147483648u64;
    if nat_lt(a2728, a2729) {
        let a2740 = {
            let a2738 = {
                let a2733 = {
                    let a2730 = v0;
                    let a2731 = v2;
                    let a2732 = 0u64;
                    nat_rem(a2730, a2731, a2732)
                };
                let a2734 = v2;
                nat_add(a2733, a2734)?
            };
            let a2739 = {
                let a2735 = v1;
                let a2736 = v2;
                let a2737 = 0u64;
                nat_rem(a2735, a2736, a2737)
            };
            nat_sub(a2738, a2739)
        };
        let a2741 = v2;
        let a2742 = 0u64;
        Ok(nat_rem(a2740, a2741, a2742))
    } else {
        let a2745 = {
            let a2743 = v0;
            let a2744 = f120(v1);
            nat_add(a2743, a2744)?
        };
        let a2746 = v2;
        let a2747 = 0u64;
        Ok(nat_rem(a2745, a2746, a2747))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2750 = f119(v1);
    let a2751 = {
        let a2748 = v0;
        let a2749 = f120(v1);
        nat_add(a2748, a2749)?
    };
    if nat_le(a2750, a2751) {
        let a2752 = f136(v0, v1)?;
        let a2753 = v2;
        Ok(nat_lt(a2752, a2753))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2756 = {
        let a2754 = v0;
        let a2755 = f120(v1);
        nat_add(a2754, a2755)?
    };
    let a2757 = f119(v1);
    Ok(nat_sub(a2756, a2757))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2758 = v0;
    let a2759 = v1;
    if nat_lt(a2758, a2759) {
        let a2760 = v1;
        let a2761 = v0;
        Ok(nat_sub(a2760, a2761))
    } else {
        let a2764 = v0;
        let a2765 = {
            let a2762 = v1;
            let a2763 = v2;
            nat_add(a2762, a2763)?
        };
        if nat_lt(a2764, a2765) {
            let a2766 = v0;
            let a2767 = v1;
            Ok(nat_sub(a2766, a2767))
        } else {
            let a2774 = {
                let a2770 = 2u64;
                let a2771 = {
                    let a2768 = v2;
                    let a2769 = 1u64;
                    nat_sub(a2768, a2769)
                };
                nat_mul(a2770, a2771)?
            };
            let a2775 = {
                let a2772 = v0;
                let a2773 = v1;
                nat_sub(a2772, a2773)
            };
            Ok(nat_sub(a2774, a2775))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2778 = {
        let a2776 = v2;
        let a2777 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2776, a2777)?
    };
    let a2779 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2778, a2779))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2780 = v0;
    let a2781 = 0u64;
    if nat_eq(a2780, a2781) {
        let a2782 = v2;
        let a2783 = v1;
        nat_le(a2782, a2783)
    } else {
        let a2784 = v0;
        let a2785 = 1u64;
        if nat_eq(a2784, a2785) {
            let a2786 = v2;
            let a2787 = v1;
            nat_lt(a2786, a2787)
        } else {
            let a2788 = v0;
            let a2789 = 2u64;
            if nat_eq(a2788, a2789) {
                let a2790 = v1;
                let a2791 = v2;
                nat_le(a2790, a2791)
            } else {
                let a2792 = v0;
                let a2793 = 3u64;
                if nat_eq(a2792, a2793) {
                    let a2794 = v1;
                    let a2795 = v2;
                    nat_lt(a2794, a2795)
                } else {
                    let a2796 = v1;
                    let a2797 = v2;
                    nat_eq(a2796, a2797)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2798 = v0;
    let a2799 = 0u64;
    if nat_eq(a2798, a2799) {
        Ok(f148(0u64))
    } else {
        let a2802 = {
            let a2800 = v0;
            let a2801 = 1u64;
            nat_add(a2800, a2801)?
        };
        let a2803 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2802, a2803) {
            Ok(Adt5::C4({
                let a2804 = v2;
                let a2805 = 0u64;
                if nat_eq(a2804, a2805) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2806 = v0;
                let a2807 = 1u64;
                nat_sub(a2806, a2807)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2808, _, _, _, _, _, _, _) => {
                        h2808
                    }
                }, false)?;
                let v5: u64 = {
                    let a2809 = v3;
                    let a2810 = v4;
                    let a2811 = 0u64;
                    nat_quot(a2809, a2810, a2811)
                };
                let v6: u64 = {
                    let a2812 = v3;
                    let a2813 = v4;
                    let a2814 = 0u64;
                    nat_rem(a2812, a2813, a2814)
                };
                let a2817 = {
                    let a2815 = v6;
                    let a2816 = 1u64;
                    nat_add(a2815, a2816)?
                };
                let a2818 = v4;
                if nat_eq(a2817, a2818) {
                    Ok(Adt5::C4({
                        let a2819 = v2;
                        let a2820 = 0u64;
                        if nat_eq(a2819, a2820) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f141(v6, v5, v1, v2)
                }
            } else {
                f156({
                    let a2821 = v3;
                    let a2822 = 2u64;
                    let a2823 = 0u64;
                    nat_quot(a2821, a2822, a2823)
                }, {
                    let a2824 = v3;
                    let a2825 = 2u64;
                    let a2826 = 0u64;
                    nat_rem(a2824, a2825, a2826)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2827 = v0;
    let a2828 = 0u64;
    if nat_eq(a2827, a2828) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2837 = {
                let a2830 = v1;
                let a2831 = match v2.clone() {
                    Adt3::C0(h2829, _, _, _, _, _, _, _) => {
                        h2829
                    }
                };
                nat_mul(a2830, a2831)?
            };
            let a2838 = {
                let a2834 = {
                    let a2832 = v0;
                    let a2833 = 1u64;
                    nat_sub(a2832, a2833)
                };
                let a2835 = 2u64;
                let a2836 = 0u64;
                nat_quot(a2834, a2835, a2836)
            };
            nat_add(a2837, a2838)?
        }, {
            let a2841 = {
                let a2839 = v0;
                let a2840 = 1u64;
                nat_sub(a2839, a2840)
            };
            let a2842 = 2u64;
            let a2843 = 0u64;
            nat_rem(a2841, a2842, a2843)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2844 = v0;
    let a2845 = f60(v1)?;
    let a2846 = 0u64;
    Ok(nat_rem(a2844, a2845, a2846))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2847 = 4u64;
    let a2848 = f157(v0, v1)?;
    nat_mul(a2847, a2848)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2849 = 4u64;
    let a2850 = f158(v0, v1)?;
    nat_mul(a2849, a2850)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2856 = {
        let a2853 = {
            let a2851 = f142(v0, v1)?;
            let a2852 = 2u64;
            nat_sub(a2851, a2852)
        };
        let a2854 = 2u64;
        let a2855 = 0u64;
        nat_rem(a2853, a2854, a2855)
    };
    let a2857 = 0u64;
    Ok(nat_eq(a2856, a2857))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2858 = f70(v0);
    match m2858 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2859 = v1;
                let a2860 = v7;
                nat_lt(a2859, a2860)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2861 = v3;
                let a2862 = f160(v4, v5, v6)?;
                nat_lt(a2861, a2862)
            } else {
                false
            } {
                let a2867 = {
                    let a2864 = v7;
                    let a2865 = match v13.clone() {
                        Adt11::C0(_, h2863, _) => {
                            h2863
                        }
                    };
                    let a2866 = 0u64;
                    nat_rem(a2864, a2865, a2866)
                };
                let a2868 = 0u64;
                nat_eq(a2867, a2868)
            } else {
                false
            } {
                Ok(Some({
                    let a2897 = {
                        let a2887 = {
                            let a2877 = {
                                let a2873 = {
                                    let a2870 = v1;
                                    let a2871 = match v13 {
                                        Adt11::C0(_, h2869, _) => {
                                            h2869
                                        }
                                    };
                                    let a2872 = 0u64;
                                    nat_quot(a2870, a2871, a2872)
                                };
                                let a2874 = v9;
                                nat_mul(a2873, a2874)?
                            };
                            let a2878 = {
                                let a2875 = v2;
                                let a2876 = v10;
                                nat_mul(a2875, a2876)?
                            };
                            nat_add(a2877, a2878)?
                        };
                        let a2888 = {
                            let a2885 = {
                                let a2882 = {
                                    let a2879 = v3;
                                    let a2880 = v4;
                                    let a2881 = 0u64;
                                    nat_quot(a2879, a2880, a2881)
                                };
                                let a2883 = v5;
                                let a2884 = 0u64;
                                nat_rem(a2882, a2883, a2884)
                            };
                            let a2886 = v11;
                            nat_mul(a2885, a2886)?
                        };
                        nat_add(a2887, a2888)?
                    };
                    let a2898 = {
                        let a2895 = {
                            let a2892 = {
                                let a2889 = v3;
                                let a2890 = v4;
                                let a2891 = 0u64;
                                nat_quot(a2889, a2890, a2891)
                            };
                            let a2893 = v5;
                            let a2894 = 0u64;
                            nat_quot(a2892, a2893, a2894)
                        };
                        let a2896 = v12;
                        nat_mul(a2895, a2896)?
                    };
                    nat_add(a2897, a2898)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2899 = v0;
    let a2900 = 30u64;
    if nat_eq(a2899, a2900) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2901 = 0u64;
    let a2902 = v0;
    if nat_lt(a2901, a2902) {
        let a2903 = v0;
        let a2904 = 2139095040u64;
        nat_le(a2903, a2904)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2905 = v0;
    let a2906 = 40u64;
    if nat_lt(a2905, a2906) {
        let a2907 = v0;
        let a2908 = 20u64;
        if nat_lt(a2907, a2908) {
            let a2909 = v0;
            let a2910 = 10u64;
            if nat_lt(a2909, a2910) {
                let a2911 = v0;
                let a2912 = 5u64;
                if nat_lt(a2911, a2912) {
                    let a2913 = v0;
                    let a2914 = 2u64;
                    if nat_lt(a2913, a2914) {
                        let a2915 = v0;
                        let a2916 = 1u64;
                        if nat_lt(a2915, a2916) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2917 = v0;
                        let a2918 = 3u64;
                        if nat_lt(a2917, a2918) {
                            4u64
                        } else {
                            let a2919 = v0;
                            let a2920 = 4u64;
                            if nat_lt(a2919, a2920) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2921 = v0;
                    let a2922 = 7u64;
                    if nat_lt(a2921, a2922) {
                        let a2923 = v0;
                        let a2924 = 6u64;
                        if nat_lt(a2923, a2924) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2925 = v0;
                        let a2926 = 8u64;
                        if nat_lt(a2925, a2926) {
                            128u64
                        } else {
                            let a2927 = v0;
                            let a2928 = 9u64;
                            if nat_lt(a2927, a2928) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2929 = v0;
                let a2930 = 15u64;
                if nat_lt(a2929, a2930) {
                    let a2931 = v0;
                    let a2932 = 12u64;
                    if nat_lt(a2931, a2932) {
                        let a2933 = v0;
                        let a2934 = 11u64;
                        if nat_lt(a2933, a2934) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2935 = v0;
                        let a2936 = 13u64;
                        if nat_lt(a2935, a2936) {
                            4096u64
                        } else {
                            let a2937 = v0;
                            let a2938 = 14u64;
                            if nat_lt(a2937, a2938) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2939 = v0;
                    let a2940 = 17u64;
                    if nat_lt(a2939, a2940) {
                        let a2941 = v0;
                        let a2942 = 16u64;
                        if nat_lt(a2941, a2942) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2943 = v0;
                        let a2944 = 18u64;
                        if nat_lt(a2943, a2944) {
                            131072u64
                        } else {
                            let a2945 = v0;
                            let a2946 = 19u64;
                            if nat_lt(a2945, a2946) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2947 = v0;
            let a2948 = 30u64;
            if nat_lt(a2947, a2948) {
                let a2949 = v0;
                let a2950 = 25u64;
                if nat_lt(a2949, a2950) {
                    let a2951 = v0;
                    let a2952 = 22u64;
                    if nat_lt(a2951, a2952) {
                        let a2953 = v0;
                        let a2954 = 21u64;
                        if nat_lt(a2953, a2954) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2955 = v0;
                        let a2956 = 23u64;
                        if nat_lt(a2955, a2956) {
                            4194304u64
                        } else {
                            let a2957 = v0;
                            let a2958 = 24u64;
                            if nat_lt(a2957, a2958) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2959 = v0;
                    let a2960 = 27u64;
                    if nat_lt(a2959, a2960) {
                        let a2961 = v0;
                        let a2962 = 26u64;
                        if nat_lt(a2961, a2962) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2963 = v0;
                        let a2964 = 28u64;
                        if nat_lt(a2963, a2964) {
                            134217728u64
                        } else {
                            let a2965 = v0;
                            let a2966 = 29u64;
                            if nat_lt(a2965, a2966) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2967 = v0;
                let a2968 = 35u64;
                if nat_lt(a2967, a2968) {
                    let a2969 = v0;
                    let a2970 = 32u64;
                    if nat_lt(a2969, a2970) {
                        let a2971 = v0;
                        let a2972 = 31u64;
                        if nat_lt(a2971, a2972) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2973 = v0;
                        let a2974 = 33u64;
                        if nat_lt(a2973, a2974) {
                            4294967296u64
                        } else {
                            let a2975 = v0;
                            let a2976 = 34u64;
                            if nat_lt(a2975, a2976) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2977 = v0;
                    let a2978 = 37u64;
                    if nat_lt(a2977, a2978) {
                        let a2979 = v0;
                        let a2980 = 36u64;
                        if nat_lt(a2979, a2980) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2981 = v0;
                        let a2982 = 38u64;
                        if nat_lt(a2981, a2982) {
                            137438953472u64
                        } else {
                            let a2983 = v0;
                            let a2984 = 39u64;
                            if nat_lt(a2983, a2984) {
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

pub fn f151(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f118(v1, v2, v3, v4) {
        f161(v0)
    } else {
        f159(v0)
    }
}

pub fn f152(v0: u64) -> u64 {
    let a2985 = v0;
    let a2986 = 26u64;
    if nat_eq(a2985, a2986) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3007 = {
        let a3001 = {
            let a2995 = {
                let a2989 = v1;
                let a2990 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2987, _, _, _) => {
                            h2987
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2988, _, _, _) => {
                            h2988
                        }
                    })
                };
                nat_mul(a2989, a2990)?
            };
            let a2996 = {
                let a2993 = v2;
                let a2994 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2991, _, _) => {
                            h2991
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2992, _, _) => {
                            h2992
                        }
                    })
                };
                nat_mul(a2993, a2994)?
            };
            nat_add(a2995, a2996)?
        };
        let a3002 = {
            let a2999 = v3;
            let a3000 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2997, _) => {
                        h2997
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2998, _) => {
                        h2998
                    }
                })
            };
            nat_mul(a2999, a3000)?
        };
        nat_add(a3001, a3002)?
    };
    let a3008 = {
        let a3005 = v4;
        let a3006 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3003) => {
                    h3003
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3004) => {
                    h3004
                }
            })
        };
        nat_mul(a3005, a3006)?
    };
    nat_add(a3007, a3008)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a3009 = v0;
    let a3010 = 2u64;
    if nat_eq(a3009, a3010) {
        Some(0u64)
    } else {
        let a3011 = v0;
        let a3012 = 5u64;
        if nat_eq(a3011, a3012) {
            Some(1u64)
        } else {
            let a3013 = v0;
            let a3014 = 6u64;
            if nat_eq(a3013, a3014) {
                Some(2u64)
            } else {
                let a3015 = v0;
                let a3016 = 7u64;
                if nat_eq(a3015, a3016) {
                    Some(3u64)
                } else {
                    let a3017 = v0;
                    let a3018 = 9u64;
                    if nat_eq(a3017, a3018) {
                        Some(4u64)
                    } else {
                        let a3019 = v0;
                        let a3020 = 8u64;
                        if nat_eq(a3019, a3020) {
                            Some(5u64)
                        } else {
                            let a3021 = v0;
                            let a3022 = 89u64;
                            if nat_eq(a3021, a3022) {
                                let a3023 = v1;
                                let a3024 = 2u64;
                                if nat_eq(a3023, a3024) {
                                    Some(6u64)
                                } else {
                                    let a3025 = v1;
                                    let a3026 = 0u64;
                                    if nat_eq(a3025, a3026) {
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

pub fn f155(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a3027 = v0;
    let a3028 = 28u64;
    if if if if nat_eq(a3027, a3028) {
        let a3029 = v1;
        let a3030 = 0u64;
        nat_eq(a3029, a3030)
    } else {
        false
    } {
        let a3031 = v2;
        let a3032 = 0u64;
        nat_eq(a3031, a3032)
    } else {
        false
    } {
        let a3033 = v3;
        let a3034 = 0u64;
        nat_eq(a3033, a3034)
    } else {
        false
    } {
        let a3035 = v4;
        let a3036 = v8;
        if if if if if if if if nat_eq(a3035, a3036) {
            let a3037 = v5;
            let a3038 = v12;
            nat_eq(a3037, a3038)
        } else {
            false
        } {
            let a3039 = v9;
            let a3040 = v13;
            nat_eq(a3039, a3040)
        } else {
            false
        } {
            let a3041 = v10;
            let a3042 = v14;
            nat_eq(a3041, a3042)
        } else {
            false
        } {
            let a3043 = v11;
            let a3044 = v15;
            nat_eq(a3043, a3044)
        } else {
            false
        } {
            let a3045 = 0u64;
            let a3046 = v6;
            nat_lt(a3045, a3046)
        } else {
            false
        } {
            let a3047 = 0u64;
            let a3048 = v7;
            nat_lt(a3047, a3048)
        } else {
            false
        } {
            let a3052 = {
                let a3049 = v10;
                let a3050 = v6;
                let a3051 = 1u64;
                nat_rem(a3049, a3050, a3051)
            };
            let a3053 = 0u64;
            if nat_eq(a3052, a3053) {
                let a3057 = {
                    let a3054 = v11;
                    let a3055 = v7;
                    let a3056 = 1u64;
                    nat_rem(a3054, a3055, a3056)
                };
                let a3058 = 0u64;
                nat_eq(a3057, a3058)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3059 = v10;
                let a3060 = v6;
                let a3061 = 0u64;
                nat_quot(a3059, a3060, a3061)
            }, {
                let a3062 = v11;
                let a3063 = v7;
                let a3064 = 0u64;
                nat_quot(a3062, a3063, a3064)
            }, {
                let a3065 = v5;
                let a3066 = v4;
                nat_mul(a3065, a3066)?
            }, {
                let a3067 = v4;
                let a3068 = v9;
                nat_mul(a3067, a3068)?
            }, {
                let a3069 = v5;
                let a3070 = v9;
                nat_mul(a3069, a3070)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3071 = v1;
    let a3072 = 0u64;
    if nat_eq(a3071, a3072) {
        Ok(Adt5::C4({
            let a3073 = v3;
            let a3074 = 0u64;
            if nat_eq(a3073, a3074) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3075 = v3;
            let a3076 = 0u64;
            if nat_eq(a3075, a3076) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3077 = v0;
    let a3078 = f60(v1)?;
    let a3079 = 0u64;
    Ok(nat_quot(a3077, a3078, a3079))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3082 = {
            let a3080 = f142(v0, v1)?;
            let a3081 = 2u64;
            nat_sub(a3080, a3081)
        };
        let a3083 = 2u64;
        let a3084 = 0u64;
        nat_quot(a3082, a3083, a3084)
    };
    let v4: u64 = {
        let a3085 = v2;
        let a3086 = v1;
        nat_mul(a3085, a3086)?
    };
    if f145(v0, v1)? {
        let a3087 = v4;
        let a3088 = v3;
        nat_add(a3087, a3088)
    } else {
        let a3100 = {
            let a3096 = v4;
            let a3097 = {
                let a3094 = {
                    let a3089 = v1;
                    let a3090 = 1u64;
                    nat_sub(a3089, a3090)
                };
                let a3095 = {
                    let a3091 = v2;
                    let a3092 = v1;
                    let a3093 = 0u64;
                    nat_rem(a3091, a3092, a3093)
                };
                nat_mul(a3094, a3095)?
            };
            nat_sub(a3096, a3097)
        };
        let a3101 = {
            let a3098 = v3;
            let a3099 = v1;
            nat_mul(a3098, a3099)?
        };
        nat_add(a3100, a3101)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3102 = v0;
    let a3103 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3102, a3103) {
        true
    } else {
        let a3104 = v0;
        let a3105 = 1u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 2u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 3u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 6u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 7u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 8u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 10u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 11u64;
        nat_eq(a3118, a3119)
    } {
        true
    } else {
        let a3120 = v0;
        let a3121 = 12u64;
        nat_eq(a3120, a3121)
    } {
        true
    } else {
        let a3122 = v0;
        let a3123 = 13u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v0;
        let a3125 = 14u64;
        nat_eq(a3124, a3125)
    } {
        true
    } else {
        let a3126 = v0;
        let a3127 = 16u64;
        nat_eq(a3126, a3127)
    } {
        true
    } else {
        let a3128 = v0;
        let a3129 = 17u64;
        nat_eq(a3128, a3129)
    } {
        true
    } else {
        let a3130 = v0;
        let a3131 = 18u64;
        nat_eq(a3130, a3131)
    } {
        true
    } else {
        let a3132 = v0;
        let a3133 = 19u64;
        nat_eq(a3132, a3133)
    } {
        true
    } else {
        let a3134 = v0;
        let a3135 = 20u64;
        nat_eq(a3134, a3135)
    } {
        true
    } else {
        let a3136 = v0;
        let a3137 = 21u64;
        nat_eq(a3136, a3137)
    } {
        true
    } else {
        let a3138 = v0;
        let a3139 = 22u64;
        nat_eq(a3138, a3139)
    } {
        true
    } else {
        let a3140 = v0;
        let a3141 = 23u64;
        nat_eq(a3140, a3141)
    } {
        true
    } else {
        let a3142 = v0;
        let a3143 = 26u64;
        nat_eq(a3142, a3143)
    } {
        true
    } else {
        let a3144 = v0;
        let a3145 = 29u64;
        nat_eq(a3144, a3145)
    } {
        true
    } else {
        let a3146 = v0;
        let a3147 = 30u64;
        nat_eq(a3146, a3147)
    } {
        true
    } else {
        let a3148 = v0;
        let a3149 = 34u64;
        nat_eq(a3148, a3149)
    } {
        true
    } else {
        let a3150 = v0;
        let a3151 = 35u64;
        nat_eq(a3150, a3151)
    } {
        true
    } else {
        let a3152 = v0;
        let a3153 = 39u64;
        nat_eq(a3152, a3153)
    } {
        true
    } else {
        let a3154 = v0;
        let a3155 = 40u64;
        nat_eq(a3154, a3155)
    } {
        true
    } else {
        let a3156 = v0;
        let a3157 = 41u64;
        nat_eq(a3156, a3157)
    } {
        true
    } else {
        let a3158 = v0;
        let a3159 = 42u64;
        nat_eq(a3158, a3159)
    } {
        true
    } else {
        let a3160 = v0;
        let a3161 = 43u64;
        nat_eq(a3160, a3161)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3162 = v0;
    let a3163 = 0u64;
    if if if nat_eq(a3162, a3163) {
        true
    } else {
        let a3164 = v1;
        let a3165 = 0u64;
        nat_eq(a3164, a3165)
    } {
        true
    } else {
        let a3166 = v2;
        let a3167 = 0u64;
        nat_eq(a3166, a3167)
    } {
        Ok(0u64)
    } else {
        let a3170 = {
            let a3168 = v0;
            let a3169 = v1;
            nat_mul(a3168, a3169)?
        };
        let a3171 = v2;
        nat_mul(a3170, a3171)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3172 = f70(v0);
    match m3172 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
