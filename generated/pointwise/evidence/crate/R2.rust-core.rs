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
                            if if nat_eq(a7, a8) {
                                true
                            } else {
                                let a9 = v1;
                                let a10 = 26u64;
                                nat_eq(a9, a10)
                            } {
                                Ok(Adt5::C0(Adt6::C5(64u64)))
                            } else {
                                let a11 = v1;
                                let a12 = 24u64;
                                if nat_eq(a11, a12) {
                                    Ok(Adt5::C0(f3(v2, v14, v13)?))
                                } else {
                                    let a13 = v1;
                                    let a14 = 27u64;
                                    if nat_eq(a13, a14) {
                                        Ok(Adt5::C0(f4(v2, v14, v13)?))
                                    } else {
                                        let a15 = v1;
                                        let a16 = 3u64;
                                        if nat_eq(a15, a16) {
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
    let a18 = match v1.clone() {
        Adt2::C0(h17, _, _, _, _, _, _, _, _, _, _) => {
            h17
        }
    };
    let a19 = 0u64;
    if if if if if if if if nat_eq(a18, a19) {
        let a21 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h20, _, _, _, _, _) => {
                h20
            }
        };
        let a22 = 4u64;
        nat_eq(a21, a22)
    } else {
        false
    } {
        f10(match v0.clone() {
            Adt1::C0(h23, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h23
            }
        })
    } else {
        false
    } {
        f10(match v0.clone() {
            Adt1::C0(_, h24, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h24
            }
        })
    } else {
        false
    } {
        let a26 = 0u64;
        let a27 = match v0.clone() {
            Adt1::C0(_, _, h25, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        };
        nat_lt(a26, a27)
    } else {
        false
    } {
        let a29 = match v0.clone() {
            Adt1::C0(_, _, h28, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        };
        let a30 = 2139095040u64;
        nat_lt(a29, a30)
    } else {
        false
    } {
        let a31 = v2;
        let a32 = 9223372036854775808u64;
        nat_lt(a31, a32)
    } else {
        false
    } {
        let a34 = f11(v2)?;
        let a35 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h33, _) => {
                h33
            }
        };
        nat_le(a34, a35)
    } else {
        false
    } {
        Ok(Adt6::C4(f12(v2)?, 4u64, f13(v1.clone(), 1u64), f13(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h36, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h36
            }
        }, match v0.clone() {
            Adt1::C0(_, h37, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h37
            }
        }, match v0 {
            Adt1::C0(_, _, h38, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h38
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m39 = v2;
    match m39 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let m40 = v3;
            match m40 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v6) => {
                    let a42 = match v1.clone() {
                        Adt2::C0(h41, _, _, _, _, _, _, _, _, _, _) => {
                            h41
                        }
                    };
                    let a43 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a42, a43) {
                        let a45 = match v5.clone() {
                            Adt2::C0(h44, _, _, _, _, _, _, _, _, _, _) => {
                                h44
                            }
                        };
                        let a46 = 0u64;
                        nat_eq(a45, a46)
                    } else {
                        false
                    } {
                        let a48 = match v6.clone() {
                            Adt2::C0(h47, _, _, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a49 = 0u64;
                        nat_eq(a48, a49)
                    } else {
                        false
                    } {
                        let a51 = match v4.clone() {
                            Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                                h50
                            }
                        };
                        let a52 = 0u64;
                        nat_eq(a51, a52)
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
                        let a58 = match v1.clone() {
                            Adt2::C0(_, h53, _, _, _, _, _, _, _, _, _) => {
                                h53
                            }
                        };
                        let a59 = {
                            let a56 = match v5.clone() {
                                Adt2::C0(_, h54, _, _, _, _, _, _, _, _, _) => {
                                    h54
                                }
                            };
                            let a57 = match v5.clone() {
                                Adt2::C0(_, h55, _, _, _, _, _, _, _, _, _) => {
                                    h55
                                }
                            };
                            nat_mul(a56, a57)?
                        };
                        nat_eq(a58, a59)
                    } else {
                        false
                    } {
                        let a65 = match v1.clone() {
                            Adt2::C0(_, _, h60, _, _, _, _, _, _, _, _) => {
                                h60
                            }
                        };
                        let a66 = {
                            let a63 = match v5.clone() {
                                Adt2::C0(_, _, h61, _, _, _, _, _, _, _, _) => {
                                    h61
                                }
                            };
                            let a64 = match v5.clone() {
                                Adt2::C0(_, _, _, h62, _, _, _, _, _, _, _) => {
                                    h62
                                }
                            };
                            nat_mul(a63, a64)?
                        };
                        nat_eq(a65, a66)
                    } else {
                        false
                    } {
                        let a69 = match v1.clone() {
                            Adt2::C0(_, _, _, h67, _, _, _, _, _, _, _) => {
                                h67
                            }
                        };
                        let a70 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h68, _, _, _, _, _, _) => {
                                h68
                            }
                        };
                        nat_eq(a69, a70)
                    } else {
                        false
                    } {
                        let a72 = match v0.clone() {
                            Adt1::C0(h71, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h71
                            }
                        };
                        let a73 = 4294967296u64;
                        nat_lt(a72, a73)
                    } else {
                        false
                    } {
                        let a75 = match v0.clone() {
                            Adt1::C0(h74, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h74
                            }
                        };
                        let a76 = 0u64;
                        if nat_eq(a75, a76) {
                            if f16(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a79 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h77) => {
                                        h77
                                    }
                                };
                                let a80 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h78) => {
                                        h78
                                    }
                                };
                                nat_eq(a79, a80)
                            }
                        } else {
                            let a83 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h81) => {
                                    h81
                                }
                            };
                            let a84 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h82) => {
                                    h82
                                }
                            };
                            nat_eq(a83, a84)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a86 = match v4.clone() {
                                Adt2::C0(_, h85, _, _, _, _, _, _, _, _, _) => {
                                    h85
                                }
                            };
                            let a87 = 0u64;
                            if if if nat_eq(a86, a87) {
                                true
                            } else {
                                let a89 = match v4.clone() {
                                    Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                                        h88
                                    }
                                };
                                let a90 = 0u64;
                                nat_eq(a89, a90)
                            } {
                                true
                            } else {
                                let a92 = match v4.clone() {
                                    Adt2::C0(_, _, _, h91, _, _, _, _, _, _, _) => {
                                        h91
                                    }
                                };
                                let a93 = 0u64;
                                nat_eq(a92, a93)
                            } {
                                true
                            } else {
                                let a95 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h94, _, _, _, _, _, _) => {
                                        h94
                                    }
                                };
                                let a96 = 0u64;
                                nat_eq(a95, a96)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                                h97
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a99 = match v0 {
                                Adt1::C0(h98, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h98
                                }
                            };
                            let a100 = 0u64;
                            if nat_eq(a99, a100) {
                                f9(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f17(v8, match v5.clone() {
                                Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                                    h101
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h102, _, _, _, _, _, _, _) => {
                                    h102
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                                    h103
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
        Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
            h104
        }
    };
    let v4: u64 = f19(v3);
    let a106 = match v1.clone() {
        Adt2::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
            h105
        }
    };
    let a107 = 0u64;
    if if if if if if if if nat_eq(a106, a107) {
        true
    } else {
        let a109 = match v1.clone() {
            Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                h108
            }
        };
        let a110 = 30u64;
        nat_eq(a109, a110)
    } {
        let a113 = match v1.clone() {
            Adt2::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                h111
            }
        };
        let a114 = match v2.clone() {
            Adt2::C0(h112, _, _, _, _, _, _, _, _, _, _) => {
                h112
            }
        };
        nat_eq(a113, a114)
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f20(match v0.clone() {
            Adt1::C0(h115, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h115
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
            Adt2::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                h116
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h117, _, _, _, _, _, _, _, _) => {
                h117
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h118, _, _, _, _, _, _, _) => {
                h118
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h119, _, _, _, _, _, _) => {
                h119
            }
        }) {
            0u64
        } else {
            let a125 = match v1.clone() {
                Adt2::C0(_, _, h120, _, _, _, _, _, _, _, _) => {
                    h120
                }
            };
            let a126 = {
                let a123 = match v1.clone() {
                    Adt2::C0(_, _, _, h121, _, _, _, _, _, _, _) => {
                        h121
                    }
                };
                let a124 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h122, _, _, _, _, _, _) => {
                        h122
                    }
                };
                nat_mul(a123, a124)?
            };
            nat_mul(a125, a126)?
        };
        let v6: u64 = {
            let a127 = v5;
            let a128 = 0u64;
            if if nat_eq(a127, a128) {
                true
            } else {
                f24(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a130 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h129, _) => {
                        h129
                    }
                };
                let a131 = v4;
                let a132 = 0u64;
                nat_quot(a130, a131, a132)
            }
        };
        Ok(Adt6::C4(f25(v3, match v1.clone() {
            Adt2::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                h133
            }
        }, v5, v6)?, v4, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h134, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h134
            }
        }, v5, f26(match v1 {
            Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a137 = match v1.clone() {
        Adt2::C0(h136, _, _, _, _, _, _, _, _, _, _) => {
            h136
        }
    };
    let a138 = 0u64;
    if if if if if nat_eq(a137, a138) {
        let a140 = match v2.clone() {
            Adt2::C0(h139, _, _, _, _, _, _, _, _, _, _) => {
                h139
            }
        };
        let a141 = 0u64;
        nat_eq(a140, a141)
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        f27(match v0.clone() {
            Adt1::C0(h142, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h142
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
            let a148 = match v3.clone() {
                Adt3::C0(_, h143, _, _, _, _, _, _) => {
                    h143
                }
            };
            let a149 = {
                let a146 = match v3.clone() {
                    Adt3::C0(_, _, h144, _, _, _, _, _) => {
                        h144
                    }
                };
                let a147 = match v3.clone() {
                    Adt3::C0(_, _, _, h145, _, _, _, _) => {
                        h145
                    }
                };
                nat_mul(a146, a147)?
            };
            nat_mul(a148, a149)?
        };
        Ok(Adt6::C4(f28(match v3.clone() {
            Adt3::C0(h150, _, _, _, _, _, _, _) => {
                h150
            }
        }, v4)?, 4u64, v3, f13(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h151, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h151
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m152 = v1;
    match m152 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m153 = v2;
            match m153 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a155 = match v0.clone() {
                        Adt2::C0(h154, _, _, _, _, _, _, _, _, _, _) => {
                            h154
                        }
                    };
                    let a156 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a155, a156) {
                        let a158 = match v4.clone() {
                            Adt2::C0(h157, _, _, _, _, _, _, _, _, _, _) => {
                                h157
                            }
                        };
                        let a159 = 0u64;
                        nat_eq(a158, a159)
                    } else {
                        false
                    } {
                        let a161 = match v5.clone() {
                            Adt2::C0(h160, _, _, _, _, _, _, _, _, _, _) => {
                                h160
                            }
                        };
                        let a162 = 26u64;
                        nat_eq(a161, a162)
                    } else {
                        false
                    } {
                        let a164 = match v3.clone() {
                            Adt2::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                                h163
                            }
                        };
                        let a165 = 0u64;
                        nat_eq(a164, a165)
                    } else {
                        false
                    } {
                        f14(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a168 = match v0.clone() {
                            Adt2::C0(_, h166, _, _, _, _, _, _, _, _, _) => {
                                h166
                            }
                        };
                        let a169 = match v4.clone() {
                            Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                                h167
                            }
                        };
                        nat_eq(a168, a169)
                    } else {
                        false
                    } {
                        let a172 = match v0.clone() {
                            Adt2::C0(_, _, h170, _, _, _, _, _, _, _, _) => {
                                h170
                            }
                        };
                        let a173 = match v5.clone() {
                            Adt2::C0(_, h171, _, _, _, _, _, _, _, _, _) => {
                                h171
                            }
                        };
                        nat_eq(a172, a173)
                    } else {
                        false
                    } {
                        let a176 = match v0.clone() {
                            Adt2::C0(_, _, _, h174, _, _, _, _, _, _, _) => {
                                h174
                            }
                        };
                        let a177 = match v5.clone() {
                            Adt2::C0(_, _, h175, _, _, _, _, _, _, _, _) => {
                                h175
                            }
                        };
                        nat_eq(a176, a177)
                    } else {
                        false
                    } {
                        let a179 = 0u64;
                        let a180 = match v5.clone() {
                            Adt2::C0(_, _, _, h178, _, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        nat_lt(a179, a180)
                    } else {
                        false
                    } {
                        let a182 = 0u64;
                        let a183 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h181, _, _, _, _, _, _) => {
                                h181
                            }
                        };
                        nat_lt(a182, a183)
                    } else {
                        false
                    } {
                        let a185 = 4u64;
                        let a186 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h184, _, _, _, _, _) => {
                                h184
                            }
                        };
                        nat_le(a185, a186)
                    } else {
                        false
                    } {
                        let a188 = 4u64;
                        let a189 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h187, _, _, _, _, _) => {
                                h187
                            }
                        };
                        nat_le(a188, a189)
                    } else {
                        false
                    } {
                        let a191 = 4u64;
                        let a192 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h190, _, _, _, _, _) => {
                                h190
                            }
                        };
                        nat_le(a191, a192)
                    } else {
                        false
                    } {
                        let a194 = 4u64;
                        let a195 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h193, _, _, _, _, _) => {
                                h193
                            }
                        };
                        nat_le(a194, a195)
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
                            Adt2::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                                h196
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h197, _, _, _, _, _, _, _, _) => {
                                h197
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                                h198
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                                h199
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
        Adt1::C0(h200, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h200
        }
    };
    Ok(Adt5::C0({
        let a201 = v0;
        let a202 = 39u64;
        if nat_eq(a201, a202) {
            f30(v2, v3, v4)?
        } else {
            let a203 = v0;
            let a204 = 77u64;
            if nat_eq(a203, a204) {
                f31(v2, v4)?
            } else {
                let a205 = v0;
                let a206 = 40u64;
                if nat_eq(a205, a206) {
                    f32(v2, v3, v4)?
                } else {
                    if f33(v0) {
                        f34(v0, v2, {
                            let m207 = v3;
                            match m207 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a208 = v0;
                        let a209 = 19u64;
                        if nat_eq(a208, a209) {
                            f35(v2, v4)?
                        } else {
                            let a210 = v0;
                            let a211 = 21u64;
                            if nat_eq(a210, a211) {
                                f36(v5, v2, v3, v4)?
                            } else {
                                let a212 = v0;
                                let a213 = 61u64;
                                if if if nat_eq(a212, a213) {
                                    true
                                } else {
                                    let a214 = v0;
                                    let a215 = 62u64;
                                    nat_eq(a214, a215)
                                } {
                                    true
                                } else {
                                    let a216 = v0;
                                    let a217 = 63u64;
                                    nat_eq(a216, a217)
                                } {
                                    f37(v0, v1, v2, v4)?
                                } else {
                                    let a218 = v0;
                                    let a219 = 4u64;
                                    if if nat_eq(a218, a219) {
                                        true
                                    } else {
                                        let a220 = v0;
                                        let a221 = 32u64;
                                        nat_eq(a220, a221)
                                    } {
                                        f38(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a222 = v0;
                                        let a223 = 20u64;
                                        if nat_eq(a222, a223) {
                                            f39(v2, v4)?
                                        } else {
                                            let a224 = v0;
                                            let a225 = 42u64;
                                            if nat_eq(a224, a225) {
                                                f40(4u64, v2, v4)?
                                            } else {
                                                let a226 = v0;
                                                let a227 = 69u64;
                                                if nat_eq(a226, a227) {
                                                    let a228 = v5;
                                                    let a229 = 4u64;
                                                    if nat_lt(a228, a229) {
                                                        f40(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a230 = v0;
                                                    let a231 = 70u64;
                                                    if nat_eq(a230, a231) {
                                                        f41(v5, v2, v4)?
                                                    } else {
                                                        let a232 = v0;
                                                        let a233 = 75u64;
                                                        if if nat_eq(a232, a233) {
                                                            true
                                                        } else {
                                                            let a234 = v0;
                                                            let a235 = 76u64;
                                                            nat_eq(a234, a235)
                                                        } {
                                                            f42({
                                                                let a236 = v0;
                                                                let a237 = 76u64;
                                                                nat_eq(a236, a237)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a238 = v0;
                                                            let a239 = 31u64;
                                                            if if if nat_eq(a238, a239) {
                                                                true
                                                            } else {
                                                                let a240 = v0;
                                                                let a241 = 47u64;
                                                                nat_eq(a240, a241)
                                                            } {
                                                                true
                                                            } else {
                                                                let a242 = v0;
                                                                let a243 = 68u64;
                                                                nat_eq(a242, a243)
                                                            } {
                                                                f43(v0, v1, v2, v4)?
                                                            } else {
                                                                let a244 = v0;
                                                                let a245 = 15u64;
                                                                if nat_eq(a244, a245) {
                                                                    f44(v2, v4)?
                                                                } else {
                                                                    let a246 = v0;
                                                                    let a247 = 17u64;
                                                                    if nat_eq(a246, a247) {
                                                                        f45(v2, v4)?
                                                                    } else {
                                                                        let a248 = v0;
                                                                        let a249 = 18u64;
                                                                        if nat_eq(a248, a249) {
                                                                            f46(v2, v3, v4)?
                                                                        } else {
                                                                            let a250 = v0;
                                                                            let a251 = 13u64;
                                                                            if if if nat_eq(a250, a251) {
                                                                                true
                                                                            } else {
                                                                                let a252 = v0;
                                                                                let a253 = 14u64;
                                                                                nat_eq(a252, a253)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a254 = v0;
                                                                                let a255 = 16u64;
                                                                                nat_eq(a254, a255)
                                                                            } {
                                                                                f47(v0, v2, v4)?
                                                                            } else {
                                                                                let m256 = f48(v0, v5);
                                                                                match m256 {
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
        let m257 = v3.clone();
        match m257 {
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
                let a258 = v52;
                let a259 = 64u64;
                if if nat_eq(a258, a259) {
                    true
                } else {
                    let a260 = v52;
                    let a261 = 65u64;
                    nat_eq(a260, a261)
                } {
                    Adt5::C0(Adt6::C5(64u64))
                } else {
                    let a262 = v52;
                    let a263 = 0u64;
                    if if nat_eq(a262, a263) {
                        true
                    } else {
                        let a264 = v52;
                        let a265 = 30u64;
                        nat_eq(a264, a265)
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
    let m266 = v0;
    match m266 {
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
                let a271 = {
                    let a267 = 0u64;
                    let a268 = v45;
                    if nat_lt(a267, a268) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a272 = {
                    let a269 = 0u64;
                    let a270 = v47;
                    if nat_lt(a269, a270) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a271, a272)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
    }
}

pub fn f9(v0: Adt2, v1: u64) -> R<u64> {
    let a284 = {
        let a278 = {
            let a274 = match v0.clone() {
                Adt2::C0(_, h273, _, _, _, _, _, _, _, _, _) => {
                    h273
                }
            };
            let a275 = v1;
            let a276 = 0u64;
            nat_quot(a274, a275, a276)
        };
        let a279 = match v0.clone() {
            Adt2::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                h277
            }
        };
        nat_mul(a278, a279)?
    };
    let a285 = {
        let a282 = match v0.clone() {
            Adt2::C0(_, _, _, h280, _, _, _, _, _, _, _) => {
                h280
            }
        };
        let a283 = match v0 {
            Adt2::C0(_, _, _, _, h281, _, _, _, _, _, _) => {
                h281
            }
        };
        nat_mul(a282, a283)?
    };
    nat_mul(a284, a285)
}

pub fn f10(v0: u64) -> bool {
    let a286 = v0;
    let a287 = 4294967296u64;
    if nat_lt(a286, a287) {
        let a291 = {
            let a288 = v0;
            let a289 = 2147483648u64;
            let a290 = 0u64;
            nat_rem(a288, a289, a290)
        };
        let a292 = 2139095040u64;
        nat_lt(a291, a292)
    } else {
        false
    }
}

pub fn f11(v0: u64) -> R<u64> {
    let a293 = 4u64;
    let a294 = v0;
    nat_mul(a293, a294)
}

pub fn f12(v0: u64) -> R<u64> {
    let a295 = v0;
    let a296 = 0u64;
    if nat_eq(a295, a296) {
        Ok(0u64)
    } else {
        let a299 = 4u64;
        let a300 = {
            let a297 = 3u64;
            let a298 = v0;
            nat_mul(a297, a298)?
        };
        nat_add(a299, a300)
    }
}

pub fn f13(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a302 = match v0.clone() {
            Adt2::C0(_, h301, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        };
        let a303 = v1;
        let a304 = 0u64;
        nat_quot(a302, a303, a304)
    }, match v0.clone() {
        Adt2::C0(_, _, h305, _, _, _, _, _, _, _, _) => {
            h305
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
            h306
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h307, _, _, _, _, _, _) => {
            h307
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h308, _, _, _, _, _) => {
            h308
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h309, _, _, _, _) => {
            h309
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h310, _, _, _) => {
            h310
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h311, _, _) => {
            h311
        }
    })
}

pub fn f14(v0: Adt2, v1: Adt2) -> bool {
    let a314 = match v0.clone() {
        Adt2::C0(_, h312, _, _, _, _, _, _, _, _, _) => {
            h312
        }
    };
    let a315 = match v1.clone() {
        Adt2::C0(_, h313, _, _, _, _, _, _, _, _, _) => {
            h313
        }
    };
    if if if nat_eq(a314, a315) {
        let a318 = match v0.clone() {
            Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                h316
            }
        };
        let a319 = match v1.clone() {
            Adt2::C0(_, _, h317, _, _, _, _, _, _, _, _) => {
                h317
            }
        };
        nat_eq(a318, a319)
    } else {
        false
    } {
        let a322 = match v0.clone() {
            Adt2::C0(_, _, _, h320, _, _, _, _, _, _, _) => {
                h320
            }
        };
        let a323 = match v1.clone() {
            Adt2::C0(_, _, _, h321, _, _, _, _, _, _, _) => {
                h321
            }
        };
        nat_eq(a322, a323)
    } else {
        false
    } {
        let a326 = match v0 {
            Adt2::C0(_, _, _, _, h324, _, _, _, _, _, _) => {
                h324
            }
        };
        let a327 = match v1 {
            Adt2::C0(_, _, _, _, h325, _, _, _, _, _, _) => {
                h325
            }
        };
        nat_eq(a326, a327)
    } else {
        false
    }
}

pub fn f15(v0: Adt2) -> R<bool> {
    let a329 = match v0.clone() {
        Adt2::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
            h328
        }
    };
    let a330 = 1u64;
    if if if if nat_eq(a329, a330) {
        true
    } else {
        let a332 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h331, _, _, _, _, _) => {
                h331
            }
        };
        let a333 = 4u64;
        nat_eq(a332, a333)
    } {
        let a335 = match v0.clone() {
            Adt2::C0(_, _, h334, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a336 = 1u64;
        if nat_eq(a335, a336) {
            true
        } else {
            let a341 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h337, _, _, _, _) => {
                    h337
                }
            };
            let a342 = {
                let a339 = match v0.clone() {
                    Adt2::C0(_, h338, _, _, _, _, _, _, _, _, _) => {
                        h338
                    }
                };
                let a340 = 4u64;
                nat_mul(a339, a340)?
            };
            nat_eq(a341, a342)
        }
    } else {
        false
    } {
        let a344 = match v0.clone() {
            Adt2::C0(_, _, _, h343, _, _, _, _, _, _, _) => {
                h343
            }
        };
        let a345 = 1u64;
        if nat_eq(a344, a345) {
            true
        } else {
            let a353 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h346, _, _, _) => {
                    h346
                }
            };
            let a354 = {
                let a351 = {
                    let a349 = match v0.clone() {
                        Adt2::C0(_, h347, _, _, _, _, _, _, _, _, _) => {
                            h347
                        }
                    };
                    let a350 = match v0.clone() {
                        Adt2::C0(_, _, h348, _, _, _, _, _, _, _, _) => {
                            h348
                        }
                    };
                    nat_mul(a349, a350)?
                };
                let a352 = 4u64;
                nat_mul(a351, a352)?
            };
            nat_eq(a353, a354)
        }
    } else {
        false
    } {
        let a356 = match v0.clone() {
            Adt2::C0(_, _, _, _, h355, _, _, _, _, _, _) => {
                h355
            }
        };
        let a357 = 1u64;
        if nat_eq(a356, a357) {
            Ok(true)
        } else {
            let a368 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h358, _, _) => {
                    h358
                }
            };
            let a369 = {
                let a366 = {
                    let a364 = {
                        let a361 = match v0.clone() {
                            Adt2::C0(_, h359, _, _, _, _, _, _, _, _, _) => {
                                h359
                            }
                        };
                        let a362 = match v0.clone() {
                            Adt2::C0(_, _, h360, _, _, _, _, _, _, _, _) => {
                                h360
                            }
                        };
                        nat_mul(a361, a362)?
                    };
                    let a365 = match v0 {
                        Adt2::C0(_, _, _, h363, _, _, _, _, _, _, _) => {
                            h363
                        }
                    };
                    nat_mul(a364, a365)?
                };
                let a367 = 4u64;
                nat_mul(a366, a367)?
            };
            Ok(nat_eq(a368, a369))
        }
    } else {
        Ok(false)
    }
}

pub fn f16(v0: Adt2, v1: Adt2) -> R<bool> {
    let a371 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h370, _) => {
            h370
        }
    };
    let a372 = 0u64;
    if nat_eq(a371, a372) {
        Ok(true)
    } else {
        let a374 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h373, _) => {
                h373
            }
        };
        let a375 = 0u64;
        if nat_eq(a374, a375) {
            Ok(true)
        } else {
            let a377 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h376) => {
                    h376
                }
            };
            let a378 = 0u64;
            if nat_eq(a377, a378) {
                Ok(true)
            } else {
                let a380 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h379) => {
                        h379
                    }
                };
                let a381 = 0u64;
                if nat_eq(a380, a381) {
                    Ok(true)
                } else {
                    let a387 = {
                        let a384 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h382) => {
                                h382
                            }
                        };
                        let a385 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h383, _) => {
                                h383
                            }
                        };
                        nat_add(a384, a385)?
                    };
                    let a388 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h386) => {
                            h386
                        }
                    };
                    if nat_le(a387, a388) {
                        Ok(true)
                    } else {
                        let a394 = {
                            let a391 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h389) => {
                                    h389
                                }
                            };
                            let a392 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h390, _) => {
                                    h390
                                }
                            };
                            nat_add(a391, a392)?
                        };
                        let a395 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h393) => {
                                h393
                            }
                        };
                        Ok(nat_le(a394, a395))
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a400 = {
        let a396 = v0;
        let a397 = v1;
        nat_mul(a396, a397)?
    };
    let a401 = {
        let a398 = v2;
        let a399 = f59(v3);
        nat_mul(a398, a399)?
    };
    nat_mul(a400, a401)
}

pub fn f18(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a404 = v0;
    let a405 = {
        let a402 = f60(v1)?;
        let a403 = v2;
        nat_mul(a402, a403)?
    };
    nat_add(a404, a405)
}

pub fn f19(v0: u64) -> u64 {
    let a406 = v0;
    let a407 = 30u64;
    if nat_eq(a406, a407) {
        2u64
    } else {
        4u64
    }
}

pub fn f20(v0: u64) -> bool {
    let a408 = v0;
    let a409 = 2139095040u64;
    if nat_le(a408, a409) {
        true
    } else {
        let a410 = v0;
        let a411 = 2147483648u64;
        nat_eq(a410, a411)
    }
}

pub fn f21(v0: Adt2, v1: u64) -> bool {
    let a413 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h412, _, _, _, _, _) => {
            h412
        }
    };
    let a414 = v1;
    if if nat_eq(a413, a414) {
        let a419 = {
            let a416 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h415) => {
                    h415
                }
            };
            let a417 = v1;
            let a418 = 0u64;
            nat_rem(a416, a417, a418)
        };
        let a420 = 0u64;
        nat_eq(a419, a420)
    } else {
        false
    } {
        f61(match v0.clone() {
            Adt2::C0(_, _, h421, _, _, _, _, _, _, _, _) => {
                h421
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                h422
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h423, _, _, _, _, _, _) => {
                h423
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h424, _, _, _, _) => {
                h424
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h425, _, _, _) => {
                h425
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h426, _, _) => {
                h426
            }
        }, v1)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f14(v0.clone(), v1.clone()) {
        let a429 = match v0.clone() {
            Adt2::C0(h427, _, _, _, _, _, _, _, _, _, _) => {
                h427
            }
        };
        let a430 = match v1.clone() {
            Adt2::C0(h428, _, _, _, _, _, _, _, _, _, _) => {
                h428
            }
        };
        nat_eq(a429, a430)
    } else {
        false
    } {
        let a433 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h431) => {
                h431
            }
        };
        let a434 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h432) => {
                h432
            }
        };
        nat_eq(a433, a434)
    } else {
        false
    } {
        let a437 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h435, _, _, _, _, _) => {
                h435
            }
        };
        let a438 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h436, _, _, _, _, _) => {
                h436
            }
        };
        nat_eq(a437, a438)
    } else {
        false
    } {
        let a441 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h439, _, _, _, _) => {
                h439
            }
        };
        let a442 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h440, _, _, _, _) => {
                h440
            }
        };
        nat_eq(a441, a442)
    } else {
        false
    } {
        let a445 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h443, _, _, _) => {
                h443
            }
        };
        let a446 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h444, _, _, _) => {
                h444
            }
        };
        nat_eq(a445, a446)
    } else {
        false
    } {
        let a449 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h447, _, _) => {
                h447
            }
        };
        let a450 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h448, _, _) => {
                h448
            }
        };
        nat_eq(a449, a450)
    } else {
        false
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a451 = v0;
    let a452 = 0u64;
    if if if nat_eq(a451, a452) {
        true
    } else {
        let a453 = v1;
        let a454 = 0u64;
        nat_eq(a453, a454)
    } {
        true
    } else {
        let a455 = v2;
        let a456 = 0u64;
        nat_eq(a455, a456)
    } {
        true
    } else {
        let a457 = v3;
        let a458 = 0u64;
        nat_eq(a457, a458)
    }
}

pub fn f24(v0: Adt2, v1: u64) -> R<bool> {
    f62({
        let a460 = match v0.clone() {
            Adt2::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        };
        let a461 = v1;
        nat_mul(a460, a461)?
    }, match v0.clone() {
        Adt2::C0(_, _, h462, _, _, _, _, _, _, _, _) => {
            h462
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h463, _, _, _, _, _, _, _) => {
            h463
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h464, _, _, _, _, _, _) => {
            h464
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h465, _, _, _, _) => {
            h465
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h466, _, _, _) => {
            h466
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h467, _, _) => {
            h467
        }
    })
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a468 = v1;
    let a469 = 0u64;
    if if nat_eq(a468, a469) {
        true
    } else {
        let a470 = v2;
        let a471 = 0u64;
        nat_eq(a470, a471)
    } {
        Ok(0u64)
    } else {
        let a478 = {
            let a472 = f63(v1, v2, v3)?;
            let a473 = f64(v0, v1, v2)?;
            nat_add(a472, a473)?
        };
        let a479 = {
            let a476 = f65(v0);
            let a477 = {
                let a474 = v1;
                let a475 = v2;
                nat_mul(a474, a475)?
            };
            nat_mul(a476, a477)?
        };
        nat_add(a478, a479)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a480 = v0;
    let a481 = 0u64;
    if if nat_eq(a480, a481) {
        true
    } else {
        let a482 = v1;
        let a483 = 0u64;
        nat_eq(a482, a483)
    } {
        Ok(3u64)
    } else {
        let a486 = {
            let a484 = 3u64;
            let a485 = v1;
            nat_add(a484, a485)?
        };
        let a487 = v2;
        nat_add(a486, a487)
    }
}

pub fn f27(v0: u64) -> bool {
    let a488 = v0;
    let a489 = 2139095040u64;
    if nat_le(a488, a489) {
        true
    } else {
        let a490 = v0;
        let a491 = 2147483648u64;
        nat_eq(a490, a491)
    }
}

pub fn f28(v0: u64, v1: u64) -> R<u64> {
    let a492 = v0;
    let a493 = 0u64;
    if nat_eq(a492, a493) {
        Ok(0u64)
    } else {
        let a494 = v1;
        let a495 = f66(v0)?;
        nat_mul(a494, a495)
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a496 = v0;
    let a497 = 0u64;
    if if if if nat_eq(a496, a497) {
        true
    } else {
        let a498 = v1;
        let a499 = 0u64;
        nat_eq(a498, a499)
    } {
        true
    } else {
        let a500 = v2;
        let a501 = 0u64;
        nat_eq(a500, a501)
    } {
        true
    } else {
        let a502 = v3;
        let a503 = 0u64;
        nat_eq(a502, a503)
    } {
        Ok(0u64)
    } else {
        let a516 = {
            let a506 = 2u64;
            let a507 = {
                let a504 = v1;
                let a505 = v2;
                nat_mul(a504, a505)?
            };
            nat_mul(a506, a507)?
        };
        let a517 = {
            let a514 = {
                let a510 = {
                    let a508 = v1;
                    let a509 = v2;
                    nat_mul(a508, a509)?
                };
                let a511 = v3;
                nat_mul(a510, a511)?
            };
            let a515 = {
                let a512 = 1u64;
                let a513 = v0;
                nat_add(a512, a513)?
            };
            nat_mul(a514, a515)?
        };
        nat_add(a516, a517)
    }
}

pub fn f30(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m518 = v1;
    match m518 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f67(v0, v3, v2)
        }
    }
}

pub fn f31(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a520 = match v0.clone() {
        Adt2::C0(h519, _, _, _, _, _, _, _, _, _, _) => {
            h519
        }
    };
    let a521 = 1u64;
    if if nat_eq(a520, a521) {
        true
    } else {
        let a523 = match v0.clone() {
            Adt2::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                h522
            }
        };
        let a524 = 30u64;
        nat_eq(a523, a524)
    } {
        let a526 = match v1.clone() {
            Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                h525
            }
        };
        let a527 = 1u64;
        if if if if if if if if if if if nat_eq(a526, a527) {
            let a530 = match v0.clone() {
                Adt2::C0(_, h528, _, _, _, _, _, _, _, _, _) => {
                    h528
                }
            };
            let a531 = match v1.clone() {
                Adt2::C0(_, h529, _, _, _, _, _, _, _, _, _) => {
                    h529
                }
            };
            nat_eq(a530, a531)
        } else {
            false
        } {
            let a533 = 0u64;
            let a534 = match v1.clone() {
                Adt2::C0(_, _, h532, _, _, _, _, _, _, _, _) => {
                    h532
                }
            };
            nat_lt(a533, a534)
        } else {
            false
        } {
            let a537 = match v1.clone() {
                Adt2::C0(_, _, h535, _, _, _, _, _, _, _, _) => {
                    h535
                }
            };
            let a538 = match v1.clone() {
                Adt2::C0(_, _, _, h536, _, _, _, _, _, _, _) => {
                    h536
                }
            };
            nat_eq(a537, a538)
        } else {
            false
        } {
            let a540 = match v1.clone() {
                Adt2::C0(_, _, _, _, h539, _, _, _, _, _, _) => {
                    h539
                }
            };
            let a541 = 1u64;
            nat_eq(a540, a541)
        } else {
            false
        } {
            let a549 = match v0.clone() {
                Adt2::C0(_, _, h542, _, _, _, _, _, _, _, _) => {
                    h542
                }
            };
            let a550 = {
                let a547 = {
                    let a545 = match v1.clone() {
                        Adt2::C0(_, _, h543, _, _, _, _, _, _, _, _) => {
                            h543
                        }
                    };
                    let a546 = match v1.clone() {
                        Adt2::C0(_, _, h544, _, _, _, _, _, _, _, _) => {
                            h544
                        }
                    };
                    nat_add(a545, a546)?
                };
                let a548 = 1u64;
                nat_sub(a547, a548)
            };
            nat_eq(a549, a550)
        } else {
            false
        } {
            let a552 = 0u64;
            let a553 = match v0.clone() {
                Adt2::C0(_, _, _, h551, _, _, _, _, _, _, _) => {
                    h551
                }
            };
            nat_lt(a552, a553)
        } else {
            false
        } {
            let a555 = 0u64;
            let a556 = match v0.clone() {
                Adt2::C0(_, _, _, _, h554, _, _, _, _, _, _) => {
                    h554
                }
            };
            nat_lt(a555, a556)
        } else {
            false
        } {
            let a558 = 2u64;
            let a559 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h557, _, _, _, _, _) => {
                    h557
                }
            };
            nat_le(a558, a559)
        } else {
            false
        } {
            let a561 = 2u64;
            let a562 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h560, _, _, _, _, _) => {
                    h560
                }
            };
            nat_le(a561, a562)
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
    let m563 = v1;
    match m563 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a565 = match v0.clone() {
                Adt2::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                    h564
                }
            };
            let a566 = 0u64;
            if if if if if if if if if if if if if nat_eq(a565, a566) {
                true
            } else {
                let a568 = match v0.clone() {
                    Adt2::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                        h567
                    }
                };
                let a569 = 1u64;
                nat_eq(a568, a569)
            } {
                let a571 = match v3.clone() {
                    Adt2::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                        h570
                    }
                };
                let a572 = 26u64;
                nat_eq(a571, a572)
            } else {
                false
            } {
                let a574 = match v2.clone() {
                    Adt2::C0(h573, _, _, _, _, _, _, _, _, _, _) => {
                        h573
                    }
                };
                let a575 = 0u64;
                nat_eq(a574, a575)
            } else {
                false
            } {
                let a578 = match v0.clone() {
                    Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a579 = match v2.clone() {
                    Adt2::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                nat_eq(a578, a579)
            } else {
                false
            } {
                let a582 = match v3.clone() {
                    Adt2::C0(_, h580, _, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a583 = match v0.clone() {
                    Adt2::C0(_, _, h581, _, _, _, _, _, _, _, _) => {
                        h581
                    }
                };
                nat_le(a582, a583)
            } else {
                false
            } {
                let a585 = match v0.clone() {
                    Adt2::C0(_, _, _, h584, _, _, _, _, _, _, _) => {
                        h584
                    }
                };
                let a586 = 1u64;
                nat_eq(a585, a586)
            } else {
                false
            } {
                let a588 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                        h587
                    }
                };
                let a589 = 1u64;
                nat_eq(a588, a589)
            } else {
                false
            } {
                let a591 = match v2.clone() {
                    Adt2::C0(_, _, _, h590, _, _, _, _, _, _, _) => {
                        h590
                    }
                };
                let a592 = 1u64;
                nat_eq(a591, a592)
            } else {
                false
            } {
                let a594 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h593, _, _, _, _, _, _) => {
                        h593
                    }
                };
                let a595 = 1u64;
                nat_eq(a594, a595)
            } else {
                false
            } {
                let a597 = match v3.clone() {
                    Adt2::C0(_, _, h596, _, _, _, _, _, _, _, _) => {
                        h596
                    }
                };
                let a598 = 1u64;
                nat_eq(a597, a598)
            } else {
                false
            } {
                let a600 = match v3.clone() {
                    Adt2::C0(_, _, _, h599, _, _, _, _, _, _, _) => {
                        h599
                    }
                };
                let a601 = 1u64;
                nat_eq(a600, a601)
            } else {
                false
            } {
                let a603 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                        h602
                    }
                };
                let a604 = 1u64;
                nat_eq(a603, a604)
            } else {
                false
            } {
                if f23(match v2.clone() {
                    Adt2::C0(_, h605, _, _, _, _, _, _, _, _, _) => {
                        h605
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h606, _, _, _, _, _, _, _, _) => {
                        h606
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h607, _, _, _, _, _, _, _) => {
                        h607
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h608, _, _, _, _, _, _) => {
                        h608
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a610 = match v3.clone() {
                        Adt2::C0(_, h609, _, _, _, _, _, _, _, _, _) => {
                            h609
                        }
                    };
                    let a611 = 0u64;
                    if nat_eq(a610, a611) {
                        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a613 = match v0.clone() {
                            Adt2::C0(_, h612, _, _, _, _, _, _, _, _, _) => {
                                h612
                            }
                        };
                        let a614 = 2147483647u64;
                        if if if if if if nat_le(a613, a614) {
                            let a616 = match v3.clone() {
                                Adt2::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                                    h615
                                }
                            };
                            let a617 = 2147483647u64;
                            nat_le(a616, a617)
                        } else {
                            false
                        } {
                            let a619 = 4u64;
                            let a620 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h618, _, _, _, _, _) => {
                                    h618
                                }
                            };
                            nat_le(a619, a620)
                        } else {
                            false
                        } {
                            let a626 = {
                                let a623 = match v2.clone() {
                                    Adt2::C0(_, h621, _, _, _, _, _, _, _, _, _) => {
                                        h621
                                    }
                                };
                                let a624 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h622, _, _, _, _, _) => {
                                        h622
                                    }
                                };
                                nat_mul(a623, a624)?
                            };
                            let a627 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h625, _, _, _, _) => {
                                    h625
                                }
                            };
                            nat_le(a626, a627)
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
                                let a641 = {
                                    let a631 = {
                                        let a629 = 2u64;
                                        let a630 = match v3.clone() {
                                            Adt2::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                                                h628
                                            }
                                        };
                                        nat_mul(a629, a630)?
                                    };
                                    let a632 = f9(v2.clone(), 1u64)?;
                                    nat_add(a631, a632)?
                                };
                                let a642 = {
                                    let a639 = match v3.clone() {
                                        Adt2::C0(_, h633, _, _, _, _, _, _, _, _, _) => {
                                            h633
                                        }
                                    };
                                    let a640 = {
                                        let a637 = 1u64;
                                        let a638 = {
                                            let a635 = 2u64;
                                            let a636 = match v0.clone() {
                                                Adt2::C0(_, h634, _, _, _, _, _, _, _, _, _) => {
                                                    h634
                                                }
                                            };
                                            nat_mul(a635, a636)?
                                        };
                                        nat_add(a637, a638)?
                                    };
                                    nat_mul(a639, a640)?
                                };
                                nat_add(a641, a642)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h643, _, _, _, _, _, _, _, _, _) => {
                                    h643
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                                    h644
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h645, _, _, _, _, _) => {
                                    h645
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h646, _, _, _, _) => {
                                    h646
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h647, _, _, _) => {
                                    h647
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h648, _, _) => {
                                    h648
                                }
                            }), f13(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h649, _, _, _, _, _, _, _, _, _, _) => {
                                    h649
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
    let a650 = v0;
    let a651 = 1u64;
    if if nat_eq(a650, a651) {
        true
    } else {
        let a652 = v0;
        let a653 = 33u64;
        nat_eq(a652, a653)
    } {
        true
    } else {
        let a654 = v0;
        let a655 = 34u64;
        nat_eq(a654, a655)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a658 = match v1.clone() {
        Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _) => {
            h656
        }
    };
    let a659 = match v3.clone() {
        Adt2::C0(h657, _, _, _, _, _, _, _, _, _, _) => {
            h657
        }
    };
    if nat_eq(a658, a659) {
        f68(v0, v1, v2, v3)
    } else {
        if if if if f69(match v1.clone() {
            Adt2::C0(h660, _, _, _, _, _, _, _, _, _, _) => {
                h660
            }
        }) {
            f69(match v3.clone() {
                Adt2::C0(h661, _, _, _, _, _, _, _, _, _, _) => {
                    h661
                }
            })
        } else {
            false
        } {
            let a662 = v0;
            let a663 = 33u64;
            if nat_eq(a662, a663) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a664 = f9(v1.clone(), 1u64)?;
            let a665 = f9(v3.clone(), 1u64)?;
            nat_eq(a664, a665)
        } else {
            false
        } {
            let m667 = f70(match v3.clone() {
                Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
                    h666
                }
            });
            match m667 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h668) => {
                            h668
                        }
                    }, f13(v1.clone(), 1u64), f13(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                            h669
                        }
                    }, match v3 {
                        Adt2::C0(h670, _, _, _, _, _, _, _, _, _, _) => {
                            h670
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
    let a672 = match v0.clone() {
        Adt2::C0(h671, _, _, _, _, _, _, _, _, _, _) => {
            h671
        }
    };
    let a673 = 0u64;
    if if if if nat_eq(a672, a673) {
        true
    } else {
        let a675 = match v0.clone() {
            Adt2::C0(h674, _, _, _, _, _, _, _, _, _, _) => {
                h674
            }
        };
        let a676 = 1u64;
        nat_eq(a675, a676)
    } {
        true
    } else {
        let a678 = match v0.clone() {
            Adt2::C0(h677, _, _, _, _, _, _, _, _, _, _) => {
                h677
            }
        };
        let a679 = 25u64;
        if nat_eq(a678, a679) {
            true
        } else {
            let a681 = match v0.clone() {
                Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
                    h680
                }
            };
            let a682 = 26u64;
            nat_eq(a681, a682)
        }
    } {
        true
    } else {
        let a684 = match v0.clone() {
            Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                h683
            }
        };
        let a685 = 30u64;
        nat_eq(a684, a685)
    } {
        let a688 = match v0.clone() {
            Adt2::C0(h686, _, _, _, _, _, _, _, _, _, _) => {
                h686
            }
        };
        let a689 = match v1.clone() {
            Adt2::C0(h687, _, _, _, _, _, _, _, _, _, _) => {
                h687
            }
        };
        if if nat_eq(a688, a689) {
            f71(match v1.clone() {
                Adt2::C0(_, h690, _, _, _, _, _, _, _, _, _) => {
                    h690
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h691, _, _, _, _, _, _, _, _) => {
                    h691
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h692, _, _, _, _, _, _, _) => {
                    h692
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h693, _, _, _, _, _, _) => {
                    h693
                }
            }, match v0.clone() {
                Adt2::C0(_, h694, _, _, _, _, _, _, _, _, _) => {
                    h694
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h695, _, _, _, _, _, _, _, _) => {
                    h695
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h696, _, _, _, _, _, _, _) => {
                    h696
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h697, _, _, _, _, _, _) => {
                    h697
                }
            })
        } else {
            false
        } {
            let m699 = f70(match v0.clone() {
                Adt2::C0(h698, _, _, _, _, _, _, _, _, _, _) => {
                    h698
                }
            });
            match m699 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f9(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h700) => {
                            h700
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
    let m701 = v2;
    match m701 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a702 = v0;
            let a703 = 4u64;
            if if if nat_lt(a702, a703) {
                let a706 = match v1.clone() {
                    Adt2::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                        h704
                    }
                };
                let a707 = match v4.clone() {
                    Adt2::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
                        h705
                    }
                };
                if nat_eq(a706, a707) {
                    let a710 = match v1.clone() {
                        Adt2::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                            h708
                        }
                    };
                    let a711 = match v3.clone() {
                        Adt2::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                            h709
                        }
                    };
                    nat_eq(a710, a711)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f72(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h712, _, _, _, _, _, _, _, _, _) => {
                        h712
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                        h713
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h714, _, _, _, _, _, _, _, _, _) => {
                        h714
                    }
                })? {
                    f72(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                            h715
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h716, _, _, _, _, _, _, _, _) => {
                            h716
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h717, _, _, _, _, _, _, _, _) => {
                            h717
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h718, _, _, _, _, _, _, _) => {
                            h718
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h719, _, _, _, _, _, _, _) => {
                            h719
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h720, _, _, _, _, _, _, _) => {
                            h720
                        }
                    })?
                } else {
                    false
                } {
                    f72(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h721, _, _, _, _, _, _) => {
                            h721
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h722, _, _, _, _, _, _) => {
                            h722
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                            h723
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m725 = f70(match v1.clone() {
                    Adt2::C0(h724, _, _, _, _, _, _, _, _, _, _) => {
                        h724
                    }
                });
                match m725 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h726, _) => {
                                h726
                            }
                        };
                        let a731 = {
                            let a728 = match v1.clone() {
                                Adt2::C0(_, h727, _, _, _, _, _, _, _, _, _) => {
                                    h727
                                }
                            };
                            let a729 = v6;
                            let a730 = 1u64;
                            nat_rem(a728, a729, a730)
                        };
                        let a732 = 0u64;
                        if if if nat_eq(a731, a732) {
                            let a737 = {
                                let a734 = match v4.clone() {
                                    Adt2::C0(_, h733, _, _, _, _, _, _, _, _, _) => {
                                        h733
                                    }
                                };
                                let a735 = v6;
                                let a736 = 1u64;
                                nat_rem(a734, a735, a736)
                            };
                            let a738 = 0u64;
                            nat_eq(a737, a738)
                        } else {
                            false
                        } {
                            let a743 = {
                                let a740 = match v3.clone() {
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
                            Ok(Adt6::C4(f9(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h745) => {
                                    h745
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
    let a747 = match v2.clone() {
        Adt2::C0(h746, _, _, _, _, _, _, _, _, _, _) => {
            h746
        }
    };
    let a748 = 0u64;
    if if nat_eq(a747, a748) {
        let a750 = match v3.clone() {
            Adt2::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                h749
            }
        };
        let a751 = 0u64;
        nat_eq(a750, a751)
    } else {
        false
    } {
        let a752 = v0;
        let a753 = 61u64;
        if nat_eq(a752, a753) {
            let v4: bool = {
                let a755 = 0u64;
                let a756 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h754, _, _, _, _, _, _, _) => {
                        h754
                    }
                };
                nat_lt(a755, a756)
            };
            if if if if if if f73(match v2.clone() {
                Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                    h757
                }
            }, match v1.clone() {
                Adt1::C0(h758, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h758
                }
            }, match v1.clone() {
                Adt1::C0(_, h759, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h759
                }
            }, match v3.clone() {
                Adt2::C0(_, h760, _, _, _, _, _, _, _, _, _) => {
                    h760
                }
            })? {
                f73(match v2.clone() {
                    Adt2::C0(_, _, h761, _, _, _, _, _, _, _, _) => {
                        h761
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h762, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h762
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h763, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h763
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h764, _, _, _, _, _, _, _, _) => {
                        h764
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, h765, _, _, _, _, _, _, _) => {
                        h765
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h766, _, _, _, _, _, _, _, _, _, _, _) => {
                        h766
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h767, _, _, _, _, _, _, _, _, _, _) => {
                        h767
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                        h768
                    }
                })?
            } else {
                false
            } {
                f73(match v2.clone() {
                    Adt2::C0(_, _, _, _, h769, _, _, _, _, _, _) => {
                        h769
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h770, _, _, _, _, _, _, _, _, _) => {
                        h770
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h771, _, _, _, _, _, _, _, _) => {
                        h771
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h772, _, _, _, _, _, _) => {
                        h772
                    }
                })?
            } else {
                false
            } {
                let a774 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h773, _, _, _, _, _, _, _) => {
                        h773
                    }
                };
                let a775 = 4294967295u64;
                nat_le(a774, a775)
            } else {
                false
            } {
                if v4 {
                    !f23(match v2.clone() {
                        Adt2::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                            h776
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h777, _, _, _, _, _, _, _, _) => {
                            h777
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h778, _, _, _, _, _, _, _) => {
                            h778
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h779, _, _, _, _, _, _) => {
                            h779
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h780, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h780
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h781, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h781
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h782, _, _, _, _, _, _, _, _, _, _, _) => {
                        h782
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h783, _, _, _, _, _, _, _, _, _) => {
                        h783
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a784 = v0;
            let a785 = 62u64;
            if nat_eq(a784, a785) {
                let a787 = match v1.clone() {
                    Adt1::C0(h786, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h786
                    }
                };
                let a788 = 2147483648u64;
                if if if if if nat_lt(a787, a788) {
                    let a790 = match v1.clone() {
                        Adt1::C0(_, h789, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h789
                        }
                    };
                    let a791 = 2147483648u64;
                    nat_lt(a790, a791)
                } else {
                    false
                } {
                    let a794 = match v1.clone() {
                        Adt1::C0(h792, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h792
                        }
                    };
                    let a795 = match v2.clone() {
                        Adt2::C0(_, h793, _, _, _, _, _, _, _, _, _) => {
                            h793
                        }
                    };
                    nat_lt(a794, a795)
                } else {
                    false
                } {
                    let a798 = match v1.clone() {
                        Adt1::C0(_, h796, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h796
                        }
                    };
                    let a799 = match v2.clone() {
                        Adt2::C0(_, h797, _, _, _, _, _, _, _, _, _) => {
                            h797
                        }
                    };
                    nat_lt(a798, a799)
                } else {
                    false
                } {
                    let a802 = match v2.clone() {
                        Adt2::C0(_, _, h800, _, _, _, _, _, _, _, _) => {
                            h800
                        }
                    };
                    let a803 = match v3.clone() {
                        Adt2::C0(_, _, h801, _, _, _, _, _, _, _, _) => {
                            h801
                        }
                    };
                    if if if nat_eq(a802, a803) {
                        let a806 = match v2.clone() {
                            Adt2::C0(_, _, _, h804, _, _, _, _, _, _, _) => {
                                h804
                            }
                        };
                        let a807 = match v3.clone() {
                            Adt2::C0(_, _, _, h805, _, _, _, _, _, _, _) => {
                                h805
                            }
                        };
                        nat_eq(a806, a807)
                    } else {
                        false
                    } {
                        let a810 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h808, _, _, _, _, _, _) => {
                                h808
                            }
                        };
                        let a811 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h809, _, _, _, _, _, _) => {
                                h809
                            }
                        };
                        nat_eq(a810, a811)
                    } else {
                        false
                    } {
                        let a820 = match v3.clone() {
                            Adt2::C0(_, h812, _, _, _, _, _, _, _, _, _) => {
                                h812
                            }
                        };
                        let a821 = {
                            let a818 = {
                                let a815 = match v2.clone() {
                                    Adt2::C0(_, h813, _, _, _, _, _, _, _, _, _) => {
                                        h813
                                    }
                                };
                                let a816 = match v1.clone() {
                                    Adt1::C0(h814, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h814
                                    }
                                };
                                nat_add(a815, a816)?
                            };
                            let a819 = match v1.clone() {
                                Adt1::C0(_, h817, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h817
                                }
                            };
                            nat_add(a818, a819)?
                        };
                        nat_eq(a820, a821)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h822, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h822
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a823 = v0;
                let a824 = 63u64;
                if nat_eq(a823, a824) {
                    if if if if if f74(match v2.clone() {
                        Adt2::C0(_, h825, _, _, _, _, _, _, _, _, _) => {
                            h825
                        }
                    }, match v1.clone() {
                        Adt1::C0(h826, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h826
                        }
                    })? {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, h827, _, _, _, _, _, _, _, _) => {
                                h827
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h828, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h828
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, h829, _, _, _, _, _, _, _) => {
                                h829
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h830, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h830
                            }
                        })?
                    } else {
                        false
                    } {
                        f74(match v2.clone() {
                            Adt2::C0(_, _, _, _, h831, _, _, _, _, _, _) => {
                                h831
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h832, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h832
                            }
                        })?
                    } else {
                        false
                    } {
                        let a835 = match v2.clone() {
                            Adt2::C0(_, h833, _, _, _, _, _, _, _, _, _) => {
                                h833
                            }
                        };
                        let a836 = match v3.clone() {
                            Adt2::C0(_, h834, _, _, _, _, _, _, _, _, _) => {
                                h834
                            }
                        };
                        if if if nat_eq(a835, a836) {
                            let a839 = match v2.clone() {
                                Adt2::C0(_, _, h837, _, _, _, _, _, _, _, _) => {
                                    h837
                                }
                            };
                            let a840 = match v3.clone() {
                                Adt2::C0(_, _, h838, _, _, _, _, _, _, _, _) => {
                                    h838
                                }
                            };
                            nat_eq(a839, a840)
                        } else {
                            false
                        } {
                            let a843 = match v2.clone() {
                                Adt2::C0(_, _, _, h841, _, _, _, _, _, _, _) => {
                                    h841
                                }
                            };
                            let a844 = match v3.clone() {
                                Adt2::C0(_, _, _, h842, _, _, _, _, _, _, _) => {
                                    h842
                                }
                            };
                            nat_eq(a843, a844)
                        } else {
                            false
                        } {
                            let a847 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h845, _, _, _, _, _, _) => {
                                    h845
                                }
                            };
                            let a848 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h846, _, _, _, _, _, _) => {
                                    h846
                                }
                            };
                            nat_eq(a847, a848)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h849, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h849
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h850, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h850
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h851, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h851
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h852, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h852
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
    let m853 = v3;
    match m853 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a854 = v0;
                let a855 = 4u64;
                nat_eq(a854, a855)
            };
            let v7: bool = {
                let a857 = 0u64;
                let a858 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h856, _, _, _, _, _, _, _, _, _, _, _) => {
                        h856
                    }
                };
                nat_lt(a857, a858)
            };
            let v8: u64 = f9(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h859, _, _, _, _, _, _, _, _, _) => {
                    h859
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h860, _, _, _, _, _, _, _, _) => {
                    h860
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h861, _, _, _, _, _, _, _) => {
                    h861
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h862, _, _, _, _, _, _) => {
                    h862
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h863, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h863
                }
            }, match v1.clone() {
                Adt1::C0(_, h864, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h864
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h865, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h865
                }
            });
            if if if if if if if if if if if if v6 {
                let a867 = match v2.clone() {
                    Adt2::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
                        h866
                    }
                };
                let a868 = 0u64;
                if nat_eq(a867, a868) {
                    let a870 = match v5.clone() {
                        Adt2::C0(h869, _, _, _, _, _, _, _, _, _, _) => {
                            h869
                        }
                    };
                    let a871 = 0u64;
                    nat_eq(a870, a871)
                } else {
                    false
                }
            } else {
                let a873 = match v2.clone() {
                    Adt2::C0(h872, _, _, _, _, _, _, _, _, _, _) => {
                        h872
                    }
                };
                let a874 = 0u64;
                if if nat_eq(a873, a874) {
                    true
                } else {
                    let a876 = match v2.clone() {
                        Adt2::C0(h875, _, _, _, _, _, _, _, _, _, _) => {
                            h875
                        }
                    };
                    let a877 = 26u64;
                    nat_eq(a876, a877)
                } {
                    let a879 = match v5.clone() {
                        Adt2::C0(h878, _, _, _, _, _, _, _, _, _, _) => {
                            h878
                        }
                    };
                    let a880 = 0u64;
                    if nat_eq(a879, a880) {
                        true
                    } else {
                        let a882 = match v5.clone() {
                            Adt2::C0(h881, _, _, _, _, _, _, _, _, _, _) => {
                                h881
                            }
                        };
                        let a883 = 26u64;
                        nat_eq(a882, a883)
                    }
                } else {
                    false
                }
            } {
                let a886 = match v2.clone() {
                    Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
                        h884
                    }
                };
                let a887 = match v4.clone() {
                    Adt2::C0(h885, _, _, _, _, _, _, _, _, _, _) => {
                        h885
                    }
                };
                nat_eq(a886, a887)
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
                let a889 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h888, _, _, _, _, _) => {
                        h888
                    }
                };
                let a890 = 4u64;
                nat_eq(a889, a890)
            } else {
                false
            } {
                let a891 = v8;
                let a892 = f9(v2.clone(), 1u64)?;
                nat_le(a891, a892)
            } else {
                false
            } {
                let a894 = match v1.clone() {
                    Adt1::C0(_, _, _, h893, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h893
                    }
                };
                let a895 = 1073741824u64;
                nat_lt(a894, a895)
            } else {
                false
            } {
                let a897 = match v1.clone() {
                    Adt1::C0(h896, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h896
                    }
                };
                let a898 = 4294967295u64;
                if if if if nat_le(a897, a898) {
                    let a900 = match v1.clone() {
                        Adt1::C0(_, h899, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h899
                        }
                    };
                    let a901 = 4294967295u64;
                    nat_le(a900, a901)
                } else {
                    false
                } {
                    let a903 = match v1.clone() {
                        Adt1::C0(_, _, h902, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h902
                        }
                    };
                    let a904 = 4294967295u64;
                    nat_le(a903, a904)
                } else {
                    false
                } {
                    let a906 = match v1.clone() {
                        Adt1::C0(_, _, _, h905, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h905
                        }
                    };
                    let a907 = 4294967295u64;
                    nat_le(a906, a907)
                } else {
                    false
                } {
                    let a909 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h908, _, _, _, _, _, _, _, _, _, _, _) => {
                            h908
                        }
                    };
                    let a910 = 4294967295u64;
                    nat_le(a909, a910)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a913 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h911) => {
                            h911
                        }
                    };
                    let a914 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h912) => {
                            h912
                        }
                    };
                    nat_eq(a913, a914)
                } else {
                    true
                }
            } else {
                false
            } {
                f75(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h915, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h915
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h916, _) => {
                        h916
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
                    let a917 = v10;
                    let a918 = v8;
                    nat_add(a917, a918)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h919, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h919
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a921 = match v0.clone() {
        Adt2::C0(h920, _, _, _, _, _, _, _, _, _, _) => {
            h920
        }
    };
    let a922 = 0u64;
    if if if nat_eq(a921, a922) {
        let a924 = match v1.clone() {
            Adt2::C0(h923, _, _, _, _, _, _, _, _, _, _) => {
                h923
            }
        };
        let a925 = 0u64;
        nat_eq(a924, a925)
    } else {
        false
    } {
        f71(match v0.clone() {
            Adt2::C0(_, h926, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h927, _, _, _, _, _, _, _, _) => {
                h927
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                h928
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h929, _, _, _, _, _, _) => {
                h929
            }
        }, match v1.clone() {
            Adt2::C0(_, h930, _, _, _, _, _, _, _, _, _) => {
                h930
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h931, _, _, _, _, _, _, _, _) => {
                h931
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h932, _, _, _, _, _, _, _) => {
                h932
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
                h933
            }
        })
    } else {
        false
    } {
        let v2: u64 = f9(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a934 = v2;
            let a935 = f9(v0.clone(), 1u64)?;
            nat_add(a934, a935)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a937 = match v1.clone() {
        Adt2::C0(h936, _, _, _, _, _, _, _, _, _, _) => {
            h936
        }
    };
    let a938 = 0u64;
    if if if nat_eq(a937, a938) {
        true
    } else {
        let a940 = match v1.clone() {
            Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
                h939
            }
        };
        let a941 = 1u64;
        nat_eq(a940, a941)
    } {
        true
    } else {
        let a943 = match v1.clone() {
            Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
                h942
            }
        };
        let a944 = 30u64;
        nat_eq(a943, a944)
    } {
        let a947 = match v1.clone() {
            Adt2::C0(h945, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        let a948 = match v2.clone() {
            Adt2::C0(h946, _, _, _, _, _, _, _, _, _, _) => {
                h946
            }
        };
        if if if if if nat_eq(a947, a948) {
            let a951 = match v1.clone() {
                Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                    h949
                }
            };
            let a952 = match v2.clone() {
                Adt2::C0(_, h950, _, _, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            nat_eq(a951, a952)
        } else {
            false
        } {
            let a955 = match v1.clone() {
                Adt2::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
                    h953
                }
            };
            let a956 = match v2.clone() {
                Adt2::C0(_, _, _, h954, _, _, _, _, _, _, _) => {
                    h954
                }
            };
            nat_eq(a955, a956)
        } else {
            false
        } {
            let a959 = match v1.clone() {
                Adt2::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                    h957
                }
            };
            let a960 = match v2.clone() {
                Adt2::C0(_, _, _, _, h958, _, _, _, _, _, _) => {
                    h958
                }
            };
            nat_eq(a959, a960)
        } else {
            false
        } {
            let a961 = v0;
            let a962 = 4u64;
            if nat_eq(a961, a962) {
                let a964 = match v1.clone() {
                    Adt2::C0(_, _, h963, _, _, _, _, _, _, _, _) => {
                        h963
                    }
                };
                let a965 = 1u64;
                if nat_eq(a964, a965) {
                    let a968 = match v1.clone() {
                        Adt2::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
                            h966
                        }
                    };
                    let a969 = match v2.clone() {
                        Adt2::C0(_, _, h967, _, _, _, _, _, _, _, _) => {
                            h967
                        }
                    };
                    nat_eq(a968, a969)
                } else {
                    false
                }
            } else {
                let a970 = v0;
                let a971 = 4u64;
                if if nat_lt(a970, a971) {
                    let a974 = match v1.clone() {
                        Adt2::C0(_, h972, _, _, _, _, _, _, _, _, _) => {
                            h972
                        }
                    };
                    let a975 = match v1.clone() {
                        Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                            h973
                        }
                    };
                    nat_eq(a974, a975)
                } else {
                    false
                } {
                    let a978 = match v1.clone() {
                        Adt2::C0(_, _, h976, _, _, _, _, _, _, _, _) => {
                            h976
                        }
                    };
                    let a979 = match v2.clone() {
                        Adt2::C0(_, _, h977, _, _, _, _, _, _, _, _) => {
                            h977
                        }
                    };
                    nat_eq(a978, a979)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m981 = f70(match v1.clone() {
                Adt2::C0(h980, _, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            });
            match m981 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h982) => {
                            h982
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
        Adt2::C0(h983, _, _, _, _, _, _, _, _, _, _) => {
            h983
        }
    }, v0) {
        let a986 = match v1.clone() {
            Adt2::C0(h984, _, _, _, _, _, _, _, _, _, _) => {
                h984
            }
        };
        let a987 = match v2.clone() {
            Adt2::C0(h985, _, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        nat_eq(a986, a987)
    } else {
        false
    } {
        let a990 = match v1.clone() {
            Adt2::C0(_, h988, _, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v2.clone() {
            Adt2::C0(_, h989, _, _, _, _, _, _, _, _, _) => {
                h989
            }
        };
        if if if nat_eq(a990, a991) {
            let a994 = match v1.clone() {
                Adt2::C0(_, _, h992, _, _, _, _, _, _, _, _) => {
                    h992
                }
            };
            let a995 = match v2.clone() {
                Adt2::C0(_, _, h993, _, _, _, _, _, _, _, _) => {
                    h993
                }
            };
            nat_eq(a994, a995)
        } else {
            false
        } {
            let a998 = match v1.clone() {
                Adt2::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                    h996
                }
            };
            let a999 = match v2.clone() {
                Adt2::C0(_, _, _, h997, _, _, _, _, _, _, _) => {
                    h997
                }
            };
            nat_eq(a998, a999)
        } else {
            false
        } {
            let a1002 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1000, _, _, _, _, _, _) => {
                    h1000
                }
            };
            let a1003 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1001, _, _, _, _, _, _) => {
                    h1001
                }
            };
            nat_eq(a1002, a1003)
        } else {
            false
        }
    } else {
        false
    } {
        let m1005 = f70(match v1.clone() {
            Adt2::C0(h1004, _, _, _, _, _, _, _, _, _, _) => {
                h1004
            }
        });
        match m1005 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1006) => {
                        h1006
                    }
                }, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1007, _, _, _, _, _, _, _, _, _, _) => {
                        h1007
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
            Adt1::C0(h1008, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1008
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1009, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1009
            }
        }
    };
    let a1010 = 0u64;
    let a1011 = v4;
    if if nat_lt(a1010, a1011) {
        let a1012 = v4;
        let a1013 = 2147483648u64;
        nat_lt(a1012, a1013)
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
            Adt2::C0(_, _, h1014, _, _, _, _, _, _, _, _) => {
                h1014
            }
        }, v4)?;
        let v8: u64 = f77(match v5.clone() {
            Adt2::C0(_, _, _, h1015, _, _, _, _, _, _, _) => {
                h1015
            }
        }, v4)?;
        let a1017 = match v5.clone() {
            Adt2::C0(h1016, _, _, _, _, _, _, _, _, _, _) => {
                h1016
            }
        };
        let a1018 = 0u64;
        if if if if if nat_eq(a1017, a1018) {
            let a1020 = match v6.clone() {
                Adt2::C0(h1019, _, _, _, _, _, _, _, _, _, _) => {
                    h1019
                }
            };
            let a1021 = 0u64;
            nat_eq(a1020, a1021)
        } else {
            false
        } {
            let a1023 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
                    h1022
                }
            };
            let a1024 = 1u64;
            nat_eq(a1023, a1024)
        } else {
            false
        } {
            let a1027 = match v5.clone() {
                Adt2::C0(_, h1025, _, _, _, _, _, _, _, _, _) => {
                    h1025
                }
            };
            let a1028 = match v6.clone() {
                Adt2::C0(_, h1026, _, _, _, _, _, _, _, _, _) => {
                    h1026
                }
            };
            nat_eq(a1027, a1028)
        } else {
            false
        } {
            if v0 {
                let a1030 = match v5.clone() {
                    Adt2::C0(_, _, h1029, _, _, _, _, _, _, _, _) => {
                        h1029
                    }
                };
                let a1031 = 2147483648u64;
                if if if nat_lt(a1030, a1031) {
                    let a1033 = match v5.clone() {
                        Adt2::C0(_, _, _, h1032, _, _, _, _, _, _, _) => {
                            h1032
                        }
                    };
                    let a1034 = 2147483648u64;
                    nat_lt(a1033, a1034)
                } else {
                    false
                } {
                    let a1037 = {
                        let a1035 = v7;
                        let a1036 = v8;
                        nat_mul(a1035, a1036)?
                    };
                    let a1038 = 2147483648u64;
                    nat_le(a1037, a1038)
                } else {
                    false
                } {
                    f78(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1040 = match v1.clone() {
                    Adt1::C0(h1039, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1039
                    }
                };
                let a1041 = v7;
                if if if if nat_eq(a1040, a1041) {
                    let a1043 = match v1 {
                        Adt1::C0(_, h1042, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1042
                        }
                    };
                    let a1044 = v8;
                    nat_eq(a1043, a1044)
                } else {
                    false
                } {
                    f79(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1046 = match v6.clone() {
                        Adt2::C0(_, _, h1045, _, _, _, _, _, _, _, _) => {
                            h1045
                        }
                    };
                    let a1047 = v4;
                    if nat_eq(a1046, a1047) {
                        let a1049 = match v6.clone() {
                            Adt2::C0(_, _, _, h1048, _, _, _, _, _, _, _) => {
                                h1048
                            }
                        };
                        let a1050 = v4;
                        nat_eq(a1049, a1050)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1054 = match v6 {
                        Adt2::C0(_, _, _, _, h1051, _, _, _, _, _, _) => {
                            h1051
                        }
                    };
                    let a1055 = {
                        let a1052 = v7;
                        let a1053 = v8;
                        nat_mul(a1052, a1053)?
                    };
                    nat_eq(a1054, a1055)
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
    let a1057 = match v2.clone() {
        Adt2::C0(h1056, _, _, _, _, _, _, _, _, _, _) => {
            h1056
        }
    };
    let a1058 = 0u64;
    if if if if if nat_eq(a1057, a1058) {
        true
    } else {
        let a1060 = match v2.clone() {
            Adt2::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
                h1059
            }
        };
        let a1061 = 1u64;
        nat_eq(a1060, a1061)
    } {
        true
    } else {
        let a1063 = match v2.clone() {
            Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        let a1064 = 30u64;
        nat_eq(a1063, a1064)
    } {
        let a1067 = match v2.clone() {
            Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1068 = match v3.clone() {
            Adt2::C0(h1066, _, _, _, _, _, _, _, _, _, _) => {
                h1066
            }
        };
        nat_eq(a1067, a1068)
    } else {
        false
    } {
        if if f14(v2.clone(), v3.clone()) {
            let a1070 = match v1.clone() {
                Adt1::C0(h1069, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1069
                }
            };
            let a1071 = 4294967296u64;
            nat_lt(a1070, a1071)
        } else {
            false
        } {
            let a1073 = match v1.clone() {
                Adt1::C0(_, h1072, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1072
                }
            };
            let a1074 = 4294967296u64;
            nat_lt(a1073, a1074)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, {
            let a1076 = match v2.clone() {
                Adt2::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1077 = 0u64;
            if nat_eq(a1076, a1077) {
                4u64
            } else {
                2u64
            }
        }, f13(v2.clone(), 1u64), f13(v3, 1u64), Adt4::C13({
            let a1078 = v0;
            let a1079 = 31u64;
            if nat_eq(a1078, a1079) {
                0u64
            } else {
                let a1080 = v0;
                let a1081 = 47u64;
                if nat_eq(a1080, a1081) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1082, _, _, _, _, _, _, _, _, _, _) => {
                h1082
            }
        }, match v1.clone() {
            Adt1::C0(h1083, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1083
            }
        }, match v1 {
            Adt1::C0(_, h1084, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1084
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1086 = match v0.clone() {
        Adt2::C0(h1085, _, _, _, _, _, _, _, _, _, _) => {
            h1085
        }
    };
    let a1087 = 0u64;
    if if if nat_eq(a1086, a1087) {
        let a1089 = match v1.clone() {
            Adt2::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1090 = 0u64;
        nat_eq(a1089, a1090)
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
    let a1092 = match v0.clone() {
        Adt2::C0(h1091, _, _, _, _, _, _, _, _, _, _) => {
            h1091
        }
    };
    let a1093 = 0u64;
    if if nat_eq(a1092, a1093) {
        let a1095 = match v1.clone() {
            Adt2::C0(h1094, _, _, _, _, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1096 = 26u64;
        nat_eq(a1095, a1096)
    } else {
        false
    } {
        let a1098 = match v0.clone() {
            Adt2::C0(_, _, _, h1097, _, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1099 = 1u64;
        if if if if if if if nat_eq(a1098, a1099) {
            let a1101 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1100, _, _, _, _, _, _) => {
                    h1100
                }
            };
            let a1102 = 1u64;
            nat_eq(a1101, a1102)
        } else {
            false
        } {
            let a1104 = match v0.clone() {
                Adt2::C0(_, h1103, _, _, _, _, _, _, _, _, _) => {
                    h1103
                }
            };
            let a1105 = 2147483647u64;
            nat_le(a1104, a1105)
        } else {
            false
        } {
            let a1108 = match v1.clone() {
                Adt2::C0(_, h1106, _, _, _, _, _, _, _, _, _) => {
                    h1106
                }
            };
            let a1109 = match v0.clone() {
                Adt2::C0(_, _, h1107, _, _, _, _, _, _, _, _) => {
                    h1107
                }
            };
            nat_eq(a1108, a1109)
        } else {
            false
        } {
            let a1111 = match v1.clone() {
                Adt2::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                    h1110
                }
            };
            let a1112 = 1u64;
            nat_eq(a1111, a1112)
        } else {
            false
        } {
            let a1114 = match v1.clone() {
                Adt2::C0(_, _, _, h1113, _, _, _, _, _, _, _) => {
                    h1113
                }
            };
            let a1115 = 1u64;
            nat_eq(a1114, a1115)
        } else {
            false
        } {
            let a1117 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1116, _, _, _, _, _, _) => {
                    h1116
                }
            };
            let a1118 = 1u64;
            nat_eq(a1117, a1118)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1121 = match v0.clone() {
                    Adt2::C0(_, _, h1119, _, _, _, _, _, _, _, _) => {
                        h1119
                    }
                };
                let a1122 = f80(match v0.clone() {
                    Adt2::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                        h1120
                    }
                })?;
                nat_mul(a1121, a1122)?
            }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1123 = v1;
    match m1123 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f81(f13(v0.clone(), 1u64))?;
            let a1125 = match v0.clone() {
                Adt2::C0(h1124, _, _, _, _, _, _, _, _, _, _) => {
                    h1124
                }
            };
            let a1126 = 26u64;
            if if if nat_eq(a1125, a1126) {
                let a1128 = match v3.clone() {
                    Adt2::C0(h1127, _, _, _, _, _, _, _, _, _, _) => {
                        h1127
                    }
                };
                let a1129 = 26u64;
                nat_eq(a1128, a1129)
            } else {
                false
            } {
                let a1131 = match v2.clone() {
                    Adt2::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                };
                let a1132 = 27u64;
                nat_eq(a1131, a1132)
            } else {
                false
            } {
                if if if if if if f14(v0.clone(), v3.clone()) {
                    let a1134 = match v2.clone() {
                        Adt2::C0(_, h1133, _, _, _, _, _, _, _, _, _) => {
                            h1133
                        }
                    };
                    let a1135 = 1u64;
                    nat_eq(a1134, a1135)
                } else {
                    false
                } {
                    let a1137 = match v2.clone() {
                        Adt2::C0(_, _, h1136, _, _, _, _, _, _, _, _) => {
                            h1136
                        }
                    };
                    let a1138 = 1u64;
                    nat_eq(a1137, a1138)
                } else {
                    false
                } {
                    let a1140 = match v2.clone() {
                        Adt2::C0(_, _, _, h1139, _, _, _, _, _, _, _) => {
                            h1139
                        }
                    };
                    let a1141 = 1u64;
                    nat_eq(a1140, a1141)
                } else {
                    false
                } {
                    let a1143 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1142, _, _, _, _, _, _) => {
                            h1142
                        }
                    };
                    let a1144 = 1u64;
                    nat_eq(a1143, a1144)
                } else {
                    false
                } {
                    let a1145 = v4;
                    let a1146 = 9223372036854775808u64;
                    nat_lt(a1145, a1146)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1149 = 2u64;
                        let a1150 = {
                            let a1147 = v4;
                            let a1148 = 2u64;
                            nat_mul(a1147, a1148)?
                        };
                        nat_add(a1149, a1150)?
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
        Adt2::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
            h1151
        }
    };
    let a1152 = v3;
    let a1153 = 0u64;
    if if nat_eq(a1152, a1153) {
        true
    } else {
        let a1154 = v0;
        let a1155 = 13u64;
        if nat_eq(a1154, a1155) {
            let a1156 = v3;
            let a1157 = 1u64;
            if nat_eq(a1156, a1157) {
                true
            } else {
                let a1158 = v3;
                let a1159 = 30u64;
                nat_eq(a1158, a1159)
            }
        } else {
            false
        }
    } {
        let a1161 = v3;
        let a1162 = match v2.clone() {
            Adt2::C0(h1160, _, _, _, _, _, _, _, _, _, _) => {
                h1160
            }
        };
        if if nat_eq(a1161, a1162) {
            let a1163 = v0;
            let a1164 = 13u64;
            if nat_eq(a1163, a1164) {
                let a1166 = match v2.clone() {
                    Adt2::C0(_, h1165, _, _, _, _, _, _, _, _, _) => {
                        h1165
                    }
                };
                let a1167 = 1u64;
                if if if nat_eq(a1166, a1167) {
                    let a1169 = match v2.clone() {
                        Adt2::C0(_, _, h1168, _, _, _, _, _, _, _, _) => {
                            h1168
                        }
                    };
                    let a1170 = 1u64;
                    nat_eq(a1169, a1170)
                } else {
                    false
                } {
                    let a1172 = match v2.clone() {
                        Adt2::C0(_, _, _, h1171, _, _, _, _, _, _, _) => {
                            h1171
                        }
                    };
                    let a1173 = 1u64;
                    nat_eq(a1172, a1173)
                } else {
                    false
                } {
                    let a1175 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1174, _, _, _, _, _, _) => {
                            h1174
                        }
                    };
                    let a1176 = 1u64;
                    nat_eq(a1175, a1176)
                } else {
                    false
                }
            } else {
                let a1178 = match v2.clone() {
                    Adt2::C0(_, h1177, _, _, _, _, _, _, _, _, _) => {
                        h1177
                    }
                };
                let a1179 = 1u64;
                if if if nat_eq(a1178, a1179) {
                    let a1182 = match v1.clone() {
                        Adt2::C0(_, _, h1180, _, _, _, _, _, _, _, _) => {
                            h1180
                        }
                    };
                    let a1183 = match v2.clone() {
                        Adt2::C0(_, _, h1181, _, _, _, _, _, _, _, _) => {
                            h1181
                        }
                    };
                    nat_eq(a1182, a1183)
                } else {
                    false
                } {
                    let a1186 = match v1.clone() {
                        Adt2::C0(_, _, _, h1184, _, _, _, _, _, _, _) => {
                            h1184
                        }
                    };
                    let a1187 = match v2.clone() {
                        Adt2::C0(_, _, _, h1185, _, _, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    nat_eq(a1186, a1187)
                } else {
                    false
                } {
                    let a1190 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1188, _, _, _, _, _, _) => {
                            h1188
                        }
                    };
                    let a1191 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1189, _, _, _, _, _, _) => {
                            h1189
                        }
                    };
                    nat_eq(a1190, a1191)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f13(v1, 1u64);
            Ok(Adt6::C4(f82(v4.clone(), v0, v3)?, {
                let a1192 = v3;
                let a1193 = 0u64;
                if nat_eq(a1192, a1193) {
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

pub fn f48(v0: u64, v1: u64) -> Option<u64> {
    let a1194 = v0;
    let a1195 = 8u64;
    if nat_eq(a1194, a1195) {
        Some(5u64)
    } else {
        let a1196 = v0;
        let a1197 = 9u64;
        if nat_eq(a1196, a1197) {
            Some(6u64)
        } else {
            let a1198 = v0;
            let a1199 = 89u64;
            if nat_eq(a1198, a1199) {
                let a1200 = v1;
                let a1201 = 0u64;
                if nat_eq(a1200, a1201) {
                    Some(0u64)
                } else {
                    let a1202 = v1;
                    let a1203 = 1u64;
                    if nat_eq(a1202, a1203) {
                        Some(1u64)
                    } else {
                        let a1204 = v1;
                        let a1205 = 2u64;
                        if nat_eq(a1204, a1205) {
                            Some(2u64)
                        } else {
                            let a1206 = v1;
                            let a1207 = 3u64;
                            if nat_eq(a1206, a1207) {
                                Some(3u64)
                            } else {
                                let a1208 = v1;
                                let a1209 = 6u64;
                                if nat_eq(a1208, a1209) {
                                    Some(4u64)
                                } else {
                                    let a1210 = v1;
                                    let a1211 = 12u64;
                                    if nat_eq(a1210, a1211) {
                                        Some(7u64)
                                    } else {
                                        let a1212 = v1;
                                        let a1213 = 11u64;
                                        if nat_eq(a1212, a1213) {
                                            Some(8u64)
                                        } else {
                                            let a1214 = v1;
                                            let a1215 = 18u64;
                                            if nat_eq(a1214, a1215) {
                                                Some(9u64)
                                            } else {
                                                let a1216 = v1;
                                                let a1217 = 19u64;
                                                if nat_eq(a1216, a1217) {
                                                    Some(10u64)
                                                } else {
                                                    let a1218 = v1;
                                                    let a1219 = 20u64;
                                                    if nat_eq(a1218, a1219) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1220 = v1;
                                                        let a1221 = 21u64;
                                                        if nat_eq(a1220, a1221) {
                                                            Some(12u64)
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
                        }
                    }
                }
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f49(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1222 = f83(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1222 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                let m1223 = v3;
                match m1223 {
                    None => {
                        f13(v2, 1u64)
                    }
                    Some(v13) => {
                        f13(v13, 1u64)
                    }
                }
            })))
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
    let a1225 = match v1.clone() {
        Adt2::C0(h1224, _, _, _, _, _, _, _, _, _, _) => {
            h1224
        }
    };
    let a1226 = 0u64;
    if if if if if nat_eq(a1225, a1226) {
        true
    } else {
        let a1228 = match v1.clone() {
            Adt2::C0(h1227, _, _, _, _, _, _, _, _, _, _) => {
                h1227
            }
        };
        let a1229 = 1u64;
        nat_eq(a1228, a1229)
    } {
        true
    } else {
        let a1231 = match v1.clone() {
            Adt2::C0(h1230, _, _, _, _, _, _, _, _, _, _) => {
                h1230
            }
        };
        let a1232 = 30u64;
        nat_eq(a1231, a1232)
    } {
        let a1235 = match v1.clone() {
            Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                h1233
            }
        };
        let a1236 = match v2.clone() {
            Adt2::C0(h1234, _, _, _, _, _, _, _, _, _, _) => {
                h1234
            }
        };
        if nat_eq(a1235, a1236) {
            true
        } else {
            let a1238 = match v2.clone() {
                Adt2::C0(h1237, _, _, _, _, _, _, _, _, _, _) => {
                    h1237
                }
            };
            let a1239 = 0u64;
            nat_eq(a1238, a1239)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1241 = match v2.clone() {
                Adt2::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                    h1240
                }
            };
            let a1242 = 0u64;
            if nat_eq(a1241, a1242) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1243, _, _, _, _, _, _, _, _, _, _) => {
                h1243
            }
        }, match v2 {
            Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1245 = v1;
    match m1245 {
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
    let m1246 = v3;
    match m1246 {
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
                Adt3::C0(h1247, _, _, _, _, _, _, _) => {
                    h1247
                }
            }, match v2.clone() {
                Adt3::C0(_, h1248, _, _, _, _, _, _) => {
                    h1248
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
            let a1249 = v51;
            let a1250 = 64u64;
            if if nat_eq(a1249, a1250) {
                true
            } else {
                let a1251 = v51;
                let a1252 = 65u64;
                nat_eq(a1251, a1252)
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
        Adt3::C0(h1253, _, _, _, _, _, _, _) => {
            h1253
        }
    })?;
    let v5: u64 = {
        let a1254 = v0;
        let a1255 = v4;
        let a1256 = 0u64;
        nat_quot(a1254, a1255, a1256)
    };
    let v6: u64 = {
        let a1257 = v0;
        let a1258 = v4;
        let a1259 = 0u64;
        nat_rem(a1257, a1258, a1259)
    };
    let a1260 = v6;
    let a1261 = 0u64;
    if nat_eq(a1260, a1261) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1262 = v6;
        let a1263 = 1u64;
        if nat_eq(a1262, a1263) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1266 = {
                let a1264 = v6;
                let a1265 = 1u64;
                nat_add(a1264, a1265)?
            };
            let a1267 = v4;
            if nat_eq(a1266, a1267) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1268 = v6;
                    let a1269 = 2u64;
                    nat_sub(a1268, a1269)
                };
                let v8: u64 = {
                    let a1270 = v7;
                    let a1271 = 3u64;
                    let a1272 = 0u64;
                    nat_quot(a1270, a1271, a1272)
                };
                let v9: u64 = {
                    let a1273 = v7;
                    let a1274 = 3u64;
                    let a1275 = 0u64;
                    nat_rem(a1273, a1274, a1275)
                };
                let v10: u64 = f89({
                    let a1279 = {
                        let a1277 = v5;
                        let a1278 = match v1.clone() {
                            Adt3::C0(h1276, _, _, _, _, _, _, _) => {
                                h1276
                            }
                        };
                        nat_mul(a1277, a1278)?
                    };
                    let a1280 = v8;
                    nat_add(a1279, a1280)?
                }, v1)?;
                let a1281 = v9;
                let a1282 = 0u64;
                if nat_eq(a1281, a1282) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1283 = v9;
                    let a1284 = 1u64;
                    if nat_eq(a1283, a1284) {
                        let a1285 = v3;
                        let a1286 = 2u64;
                        if nat_eq(a1285, a1286) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1287 = v3;
                        let a1288 = 2u64;
                        if if nat_eq(a1287, a1288) {
                            true
                        } else {
                            let a1289 = v3;
                            let a1290 = 3u64;
                            nat_eq(a1289, a1290)
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
        let a1292 = 2u64;
        let a1293 = match v1.clone() {
            Adt3::C0(_, h1291, _, _, _, _, _, _) => {
                h1291
            }
        };
        nat_mul(a1292, a1293)?
    };
    let a1294 = v0;
    let a1295 = v5;
    if nat_lt(a1294, a1295) {
        let a1299 = {
            let a1296 = v0;
            let a1297 = 2u64;
            let a1298 = 0u64;
            nat_rem(a1296, a1297, a1298)
        };
        let a1300 = 0u64;
        if nat_eq(a1299, a1300) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1301 = v0;
                let a1302 = 2u64;
                let a1303 = 0u64;
                nat_quot(a1301, a1302, a1303)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1304 = v4;
            let a1305 = 2147483648u64;
            if if nat_lt(a1304, a1305) {
                let a1307 = v4;
                let a1308 = match v2 {
                    Adt3::C0(_, h1306, _, _, _, _, _, _) => {
                        h1306
                    }
                };
                nat_lt(a1307, a1308)
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
            let a1311 = match v2.clone() {
                Adt3::C0(h1309, _, _, _, _, _, _, _) => {
                    h1309
                }
            };
            let a1312 = match v2.clone() {
                Adt3::C0(_, h1310, _, _, _, _, _, _) => {
                    h1310
                }
            };
            nat_mul(a1311, a1312)?
        };
        let a1315 = {
            let a1313 = v0;
            let a1314 = v5;
            nat_sub(a1313, a1314)
        };
        let a1316 = v6;
        if nat_lt(a1315, a1316) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1317 = v0;
                let a1318 = v5;
                nat_sub(a1317, a1318)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1321 = {
                    let a1319 = v0;
                    let a1320 = v5;
                    nat_sub(a1319, a1320)
                };
                let a1322 = v6;
                nat_sub(a1321, a1322)
            };
            let v8: u64 = {
                let a1326 = 1u64;
                let a1327 = {
                    let a1324 = 2u64;
                    let a1325 = match v1.clone() {
                        Adt3::C0(h1323, _, _, _, _, _, _, _) => {
                            h1323
                        }
                    };
                    nat_mul(a1324, a1325)?
                };
                nat_add(a1326, a1327)?
            };
            let v9: u64 = {
                let a1328 = v7;
                let a1329 = v8;
                let a1330 = 0u64;
                nat_quot(a1328, a1329, a1330)
            };
            let v10: u64 = {
                let a1331 = v7;
                let a1332 = v8;
                let a1333 = 0u64;
                nat_rem(a1331, a1332, a1333)
            };
            let a1334 = v10;
            let a1335 = 0u64;
            if nat_eq(a1334, a1335) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1339 = {
                    let a1336 = v10;
                    let a1337 = 2u64;
                    let a1338 = 0u64;
                    nat_rem(a1336, a1337, a1338)
                };
                let a1340 = 1u64;
                if nat_eq(a1339, a1340) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1349 = {
                            let a1342 = v9;
                            let a1343 = match v1.clone() {
                                Adt3::C0(h1341, _, _, _, _, _, _, _) => {
                                    h1341
                                }
                            };
                            nat_mul(a1342, a1343)?
                        };
                        let a1350 = {
                            let a1346 = {
                                let a1344 = v10;
                                let a1345 = 1u64;
                                nat_sub(a1344, a1345)
                            };
                            let a1347 = 2u64;
                            let a1348 = 0u64;
                            nat_quot(a1346, a1347, a1348)
                        };
                        nat_add(a1349, a1350)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1359 = {
                            let a1352 = v4;
                            let a1353 = match v1.clone() {
                                Adt3::C0(h1351, _, _, _, _, _, _, _) => {
                                    h1351
                                }
                            };
                            nat_mul(a1352, a1353)?
                        };
                        let a1360 = {
                            let a1356 = {
                                let a1354 = v10;
                                let a1355 = 1u64;
                                nat_sub(a1354, a1355)
                            };
                            let a1357 = 2u64;
                            let a1358 = 0u64;
                            nat_quot(a1356, a1357, a1358)
                        };
                        nat_add(a1359, a1360)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1369 = {
                            let a1362 = v4;
                            let a1363 = match v1 {
                                Adt3::C0(h1361, _, _, _, _, _, _, _) => {
                                    h1361
                                }
                            };
                            nat_mul(a1362, a1363)?
                        };
                        let a1370 = {
                            let a1366 = {
                                let a1364 = v10;
                                let a1365 = 1u64;
                                nat_sub(a1364, a1365)
                            };
                            let a1367 = 2u64;
                            let a1368 = 0u64;
                            nat_quot(a1366, a1367, a1368)
                        };
                        nat_add(a1369, a1370)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1371 = v1;
    let a1372 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1371, a1372) {
        let a1375 = v1;
        let a1376 = {
            let a1373 = 2u64;
            let a1374 = f81(v4.clone())?;
            nat_mul(a1373, a1374)?
        };
        if nat_lt(a1375, a1376) {
            let a1380 = {
                let a1377 = v1;
                let a1378 = 2u64;
                let a1379 = 0u64;
                nat_rem(a1377, a1378, a1379)
            };
            let a1381 = 0u64;
            if nat_eq(a1380, a1381) {
                f105({
                    let a1382 = v1;
                    let a1383 = 2u64;
                    let a1384 = 0u64;
                    nat_quot(a1382, a1383, a1384)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1385, _, _, _, _, _, _) => {
                        h1385
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1388 = v1;
                let a1389 = {
                    let a1386 = 2u64;
                    let a1387 = f81(v4.clone())?;
                    nat_mul(a1386, a1387)?
                };
                nat_sub(a1388, a1389)
            };
            let v7: u64 = {
                let a1393 = v6;
                let a1394 = {
                    let a1391 = 1u64;
                    let a1392 = match v2.clone() {
                        Adt3::C0(h1390, _, _, _, _, _, _, _) => {
                            h1390
                        }
                    };
                    nat_add(a1391, a1392)?
                };
                let a1395 = 0u64;
                nat_quot(a1393, a1394, a1395)
            };
            let v8: u64 = {
                let a1399 = v6;
                let a1400 = {
                    let a1397 = 1u64;
                    let a1398 = match v2.clone() {
                        Adt3::C0(h1396, _, _, _, _, _, _, _) => {
                            h1396
                        }
                    };
                    nat_add(a1397, a1398)?
                };
                let a1401 = 0u64;
                nat_rem(a1399, a1400, a1401)
            };
            let a1402 = v8;
            let a1403 = 0u64;
            if nat_eq(a1402, a1403) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1404 = v8;
                    let a1405 = 1u64;
                    nat_sub(a1404, a1405)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1408 = match v4.clone() {
            Adt3::C0(h1406, _, _, _, _, _, _, _) => {
                h1406
            }
        };
        let a1409 = match v4.clone() {
            Adt3::C0(_, h1407, _, _, _, _, _, _) => {
                h1407
            }
        };
        nat_mul(a1408, a1409)?
    };
    let v7: u64 = {
        let a1410 = 2u64;
        let a1411 = v6;
        nat_mul(a1410, a1411)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1412, _, _, _, _, _, _) => {
            h1412
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1413, _, _, _, _, _) => {
            h1413
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1414, _, _, _, _) => {
            h1414
        }
    });
    let a1415 = v0;
    let a1416 = v7;
    if nat_lt(a1415, a1416) {
        let a1420 = {
            let a1417 = v0;
            let a1418 = 2u64;
            let a1419 = 0u64;
            nat_rem(a1417, a1418, a1419)
        };
        let a1421 = 0u64;
        if nat_eq(a1420, a1421) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1422 = v0;
                let a1423 = 2u64;
                let a1424 = 0u64;
                nat_quot(a1422, a1423, a1424)
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
            let a1425 = v0;
            let a1426 = v7;
            nat_sub(a1425, a1426)
        };
        let v10: u64 = {
            let a1430 = v9;
            let a1431 = {
                let a1428 = 1u64;
                let a1429 = match v1.clone() {
                    Adt3::C0(h1427, _, _, _, _, _, _, _) => {
                        h1427
                    }
                };
                nat_add(a1428, a1429)?
            };
            let a1432 = 0u64;
            nat_quot(a1430, a1431, a1432)
        };
        let v11: u64 = {
            let a1436 = v9;
            let a1437 = {
                let a1434 = 1u64;
                let a1435 = match v1.clone() {
                    Adt3::C0(h1433, _, _, _, _, _, _, _) => {
                        h1433
                    }
                };
                nat_add(a1434, a1435)?
            };
            let a1438 = 0u64;
            nat_rem(a1436, a1437, a1438)
        };
        let a1439 = v11;
        let a1440 = 0u64;
        if nat_eq(a1439, a1440) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1441 = v10;
                let a1442 = v6;
                let a1443 = 0u64;
                nat_rem(a1441, a1442, a1443)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1449 = {
                        let a1445 = v10;
                        let a1446 = match v1.clone() {
                            Adt3::C0(h1444, _, _, _, _, _, _, _) => {
                                h1444
                            }
                        };
                        nat_mul(a1445, a1446)?
                    };
                    let a1450 = {
                        let a1447 = v11;
                        let a1448 = 1u64;
                        nat_sub(a1447, a1448)
                    };
                    nat_add(a1449, a1450)?
                };
                let a1451 = v5;
                let a1452 = 4294967295u64;
                if nat_eq(a1451, a1452) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1458 = {
                            let a1454 = v5;
                            let a1455 = match v3.clone() {
                                Adt3::C0(h1453, _, _, _, _, _, _, _) => {
                                    h1453
                                }
                            };
                            nat_mul(a1454, a1455)?
                        };
                        let a1459 = {
                            let a1456 = v11;
                            let a1457 = 1u64;
                            nat_sub(a1456, a1457)
                        };
                        nat_add(a1458, a1459)?
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
        let a1462 = v6;
        let a1463 = {
            let a1460 = 3u64;
            let a1461 = v5;
            nat_add(a1460, a1461)?
        };
        nat_sub(a1462, a1463)
    };
    let a1465 = v0;
    let a1466 = f25(v3, match v1.clone() {
        Adt3::C0(h1464, _, _, _, _, _, _, _) => {
            h1464
        }
    }, v5, v8)?;
    if nat_lt(a1465, a1466) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1467, _, _, _, _, _, _, _) => {
                h1467
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1468, _, _, _, _, _, _, _) => {
                h1468
            }
        }, v5)?;
        let a1469 = v0;
        let a1470 = v9;
        if nat_lt(a1469, a1470) {
            f110(v0, v2, f19(v3), {
                let a1471 = 3u64;
                let a1472 = v5;
                nat_add(a1471, a1472)?
            }, v7)
        } else {
            let a1475 = v0;
            let a1476 = {
                let a1473 = v9;
                let a1474 = v10;
                nat_add(a1473, a1474)?
            };
            if nat_lt(a1475, a1476) {
                let v11: u64 = {
                    let a1477 = v0;
                    let a1478 = v9;
                    nat_sub(a1477, a1478)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1479, _, _, _, _, _, _, _) => {
                        h1479
                    }
                })?;
                f112({
                    let a1480 = v11;
                    let a1481 = v12;
                    let a1482 = 0u64;
                    nat_rem(a1480, a1481, a1482)
                }, {
                    let a1483 = v11;
                    let a1484 = v12;
                    let a1485 = 0u64;
                    nat_quot(a1483, a1484, a1485)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1488 = v0;
                    let a1489 = {
                        let a1486 = v9;
                        let a1487 = v10;
                        nat_add(a1486, a1487)?
                    };
                    nat_sub(a1488, a1489)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1490 = v0;
    let a1491 = f12(v1)?;
    if nat_lt(a1490, a1491) {
        let a1492 = v0;
        let a1493 = 4u64;
        if nat_lt(a1492, a1493) {
            let a1494 = v0;
            let a1495 = 0u64;
            if nat_eq(a1494, a1495) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1496 = v0;
                let a1497 = 1u64;
                if nat_eq(a1496, a1497) {
                    let a1498 = v5;
                    let a1499 = 0u64;
                    if nat_eq(a1498, a1499) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1500 = v0;
                    let a1501 = 2u64;
                    if nat_eq(a1500, a1501) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1502 = v0;
            let a1503 = 4u64;
            if if nat_eq(a1502, a1503) {
                let a1504 = f114(v5)?;
                let a1505 = v1;
                !nat_eq(a1504, a1505)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1508 = {
                        let a1506 = v0;
                        let a1507 = 4u64;
                        nat_sub(a1506, a1507)
                    };
                    let a1509 = 3u64;
                    let a1510 = 0u64;
                    nat_quot(a1508, a1509, a1510)
                };
                let v7: u64 = {
                    let a1513 = {
                        let a1511 = v0;
                        let a1512 = 4u64;
                        nat_sub(a1511, a1512)
                    };
                    let a1514 = 3u64;
                    let a1515 = 0u64;
                    nat_rem(a1513, a1514, a1515)
                };
                let a1516 = v7;
                let a1517 = 0u64;
                if nat_eq(a1516, a1517) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1518 = v7;
                    let a1519 = 1u64;
                    if nat_eq(a1518, a1519) {
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
        let a1520 = v0;
        let a1521 = 4294967296u64;
        let a1522 = 0u64;
        nat_rem(a1520, a1521, a1522)
    };
    let a1523 = v1;
    let a1524 = 2147483648u64;
    if nat_lt(a1523, a1524) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1527 = 2u64;
    let a1528 = {
        let a1525 = 2u64;
        let a1526 = v0;
        nat_mul(a1525, a1526)?
    };
    nat_add(a1527, a1528)
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
    let a1529 = v2;
    let a1530 = 0u64;
    if nat_eq(a1529, a1530) {
        Ok(0u64)
    } else {
        let a1533 = 2u64;
        let a1534 = {
            let a1531 = v0;
            let a1532 = v1;
            nat_mul(a1531, a1532)?
        };
        nat_mul(a1533, a1534)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1535 = v2;
    let a1536 = f111(v0, v1)?;
    nat_mul(a1535, a1536)
}

pub fn f65(v0: u64) -> u64 {
    let a1537 = v0;
    let a1538 = 30u64;
    if nat_eq(a1537, a1538) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1541 = 5u64;
    let a1542 = {
        let a1539 = 4u64;
        let a1540 = v0;
        nat_mul(a1539, a1540)?
    };
    nat_add(a1541, a1542)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1543, _, _, _, _, _, _, _, _, _, _) => {
            h1543
        }
    }, match v0.clone() {
        Adt2::C0(_, h1544, _, _, _, _, _, _, _, _, _) => {
            h1544
        }
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
    }, match v1.clone() {
        Adt2::C0(h1548, _, _, _, _, _, _, _, _, _, _) => {
            h1548
        }
    }, match v1.clone() {
        Adt2::C0(_, h1549, _, _, _, _, _, _, _, _, _) => {
            h1549
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1550, _, _, _, _, _, _, _, _) => {
            h1550
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1551, _, _, _, _, _, _, _) => {
            h1551
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1552, _, _, _, _, _, _) => {
            h1552
        }
    }, match v2.clone() {
        Adt2::C0(h1553, _, _, _, _, _, _, _, _, _, _) => {
            h1553
        }
    }, match v2.clone() {
        Adt2::C0(_, h1554, _, _, _, _, _, _, _, _, _) => {
            h1554
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1555, _, _, _, _, _, _, _, _) => {
            h1555
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1556, _, _, _, _, _, _, _) => {
            h1556
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1557, _, _, _, _, _, _) => {
            h1557
        }
    }) {
        let m1559 = f70(match v0.clone() {
            Adt2::C0(h1558, _, _, _, _, _, _, _, _, _, _) => {
                h1558
            }
        });
        match m1559 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1565 = {
                    let a1562 = match v0.clone() {
                        Adt2::C0(_, h1560, _, _, _, _, _, _, _, _, _) => {
                            h1560
                        }
                    };
                    let a1563 = match v3.clone() {
                        Adt11::C0(_, h1561, _) => {
                            h1561
                        }
                    };
                    let a1564 = 0u64;
                    nat_rem(a1562, a1563, a1564)
                };
                let a1566 = 0u64;
                if if nat_eq(a1565, a1566) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1567, _, _, _, _, _, _, _, _, _) => {
                            h1567
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1568, _, _, _, _, _, _, _, _, _) => {
                            h1568
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1569, _, _, _, _, _, _, _, _) => {
                            h1569
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1570, _, _, _, _, _, _, _) => {
                            h1570
                        }
                    }) {
                        true
                    } else {
                        let a1572 = 4u64;
                        let a1573 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1571, _, _, _, _, _) => {
                                h1571
                            }
                        };
                        if if nat_le(a1572, a1573) {
                            let a1575 = match v3.clone() {
                                Adt11::C0(_, h1574, _) => {
                                    h1574
                                }
                            };
                            let a1576 = 1u64;
                            if nat_eq(a1575, a1576) {
                                true
                            } else {
                                let a1579 = match v3.clone() {
                                    Adt11::C0(_, _, h1577) => {
                                        h1577
                                    }
                                };
                                let a1580 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1578, _, _, _, _, _) => {
                                        h1578
                                    }
                                };
                                nat_le(a1579, a1580)
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
                        Adt11::C0(_, h1581, _) => {
                            h1581
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1582, _, _, _, _, _, _, _, _, _) => {
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
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1586, _, _, _, _, _, _, _, _, _, _) => {
                            h1586
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
    let a1589 = match v1.clone() {
        Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
            h1587
        }
    };
    let a1590 = match v3.clone() {
        Adt2::C0(h1588, _, _, _, _, _, _, _, _, _, _) => {
            h1588
        }
    };
    if if nat_eq(a1589, a1590) {
        let a1591 = v0;
        let a1592 = 33u64;
        if nat_eq(a1591, a1592) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1594 = f70(match v1.clone() {
            Adt2::C0(h1593, _, _, _, _, _, _, _, _, _, _) => {
                h1593
            }
        });
        match m1594 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1595, _) => {
                        h1595
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1600 = {
                    let a1597 = match v1.clone() {
                        Adt2::C0(_, h1596, _, _, _, _, _, _, _, _, _) => {
                            h1596
                        }
                    };
                    let a1598 = v5;
                    let a1599 = 1u64;
                    nat_rem(a1597, a1598, a1599)
                };
                let a1601 = 0u64;
                if if if nat_eq(a1600, a1601) {
                    let a1606 = {
                        let a1603 = match v3.clone() {
                            Adt2::C0(_, h1602, _, _, _, _, _, _, _, _, _) => {
                                h1602
                            }
                        };
                        let a1604 = v5;
                        let a1605 = 1u64;
                        nat_rem(a1603, a1604, a1605)
                    };
                    let a1607 = 0u64;
                    nat_eq(a1606, a1607)
                } else {
                    false
                } {
                    let a1608 = v6;
                    let a1609 = f9(v3.clone(), v5)?;
                    nat_eq(a1608, a1609)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1610) => {
                            h1610
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
    let a1611 = v0;
    let a1612 = 0u64;
    if if nat_eq(a1611, a1612) {
        true
    } else {
        let a1613 = v0;
        let a1614 = 1u64;
        nat_eq(a1613, a1614)
    } {
        true
    } else {
        let a1615 = v0;
        let a1616 = 30u64;
        nat_eq(a1615, a1616)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1617 = v0;
    let a1618 = 0u64;
    if nat_eq(a1617, a1618) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1619 = v0;
        let a1620 = 1u64;
        if nat_eq(a1619, a1620) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1621 = v0;
            let a1622 = 2u64;
            if nat_eq(a1621, a1622) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1623 = v0;
                let a1624 = 3u64;
                if nat_eq(a1623, a1624) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1625 = v0;
                    let a1626 = 6u64;
                    if nat_eq(a1625, a1626) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1627 = v0;
                        let a1628 = 7u64;
                        if nat_eq(a1627, a1628) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1629 = v0;
                            let a1630 = 8u64;
                            if nat_eq(a1629, a1630) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1631 = v0;
                                let a1632 = 9u64;
                                if nat_eq(a1631, a1632) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1633 = v0;
                                    let a1634 = 10u64;
                                    if nat_eq(a1633, a1634) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1635 = v0;
                                        let a1636 = 11u64;
                                        if nat_eq(a1635, a1636) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1637 = v0;
                                            let a1638 = 12u64;
                                            if nat_eq(a1637, a1638) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1639 = v0;
                                                let a1640 = 13u64;
                                                if nat_eq(a1639, a1640) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1641 = v0;
                                                    let a1642 = 14u64;
                                                    if nat_eq(a1641, a1642) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1643 = v0;
                                                        let a1644 = 15u64;
                                                        if nat_eq(a1643, a1644) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1645 = v0;
                                                            let a1646 = 16u64;
                                                            if nat_eq(a1645, a1646) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1647 = v0;
                                                                let a1648 = 17u64;
                                                                if nat_eq(a1647, a1648) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1649 = v0;
                                                                    let a1650 = 18u64;
                                                                    if nat_eq(a1649, a1650) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1651 = v0;
                                                                        let a1652 = 19u64;
                                                                        if nat_eq(a1651, a1652) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1653 = v0;
                                                                            let a1654 = 20u64;
                                                                            if nat_eq(a1653, a1654) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1655 = v0;
                                                                                let a1656 = 21u64;
                                                                                if nat_eq(a1655, a1656) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1657 = v0;
                                                                                    let a1658 = 22u64;
                                                                                    if nat_eq(a1657, a1658) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1659 = v0;
                                                                                        let a1660 = 23u64;
                                                                                        if nat_eq(a1659, a1660) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1661 = v0;
                                                                                            let a1662 = 24u64;
                                                                                            if nat_eq(a1661, a1662) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1663 = v0;
                                                                                                let a1664 = 25u64;
                                                                                                if nat_eq(a1663, a1664) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1665 = v0;
                                                                                                    let a1666 = 26u64;
                                                                                                    if nat_eq(a1665, a1666) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1667 = v0;
                                                                                                        let a1668 = 27u64;
                                                                                                        if nat_eq(a1667, a1668) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1669 = v0;
                                                                                                            let a1670 = 28u64;
                                                                                                            if nat_eq(a1669, a1670) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1671 = v0;
                                                                                                                let a1672 = 29u64;
                                                                                                                if nat_eq(a1671, a1672) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1673 = v0;
                                                                                                                    let a1674 = 30u64;
                                                                                                                    if nat_eq(a1673, a1674) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1675 = v0;
                                                                                                                        let a1676 = 34u64;
                                                                                                                        if nat_eq(a1675, a1676) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1677 = v0;
                                                                                                                            let a1678 = 35u64;
                                                                                                                            if nat_eq(a1677, a1678) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1679 = v0;
                                                                                                                                let a1680 = 39u64;
                                                                                                                                if nat_eq(a1679, a1680) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1681 = v0;
                                                                                                                                    let a1682 = 40u64;
                                                                                                                                    if nat_eq(a1681, a1682) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1683 = v0;
                                                                                                                                        let a1684 = 41u64;
                                                                                                                                        if nat_eq(a1683, a1684) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1685 = v0;
                                                                                                                                            let a1686 = 42u64;
                                                                                                                                            if nat_eq(a1685, a1686) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1687 = v0;
                                                                                                                                                let a1688 = 43u64;
                                                                                                                                                if nat_eq(a1687, a1688) {
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
        let a1692 = {
            let a1689 = v0;
            let a1690 = v4;
            let a1691 = 1u64;
            nat_rem(a1689, a1690, a1691)
        };
        let a1693 = 0u64;
        if if if nat_eq(a1692, a1693) {
            let a1697 = {
                let a1694 = v1;
                let a1695 = v5;
                let a1696 = 1u64;
                nat_rem(a1694, a1695, a1696)
            };
            let a1698 = 0u64;
            nat_eq(a1697, a1698)
        } else {
            false
        } {
            let a1702 = {
                let a1699 = v2;
                let a1700 = v6;
                let a1701 = 1u64;
                nat_rem(a1699, a1700, a1701)
            };
            let a1703 = 0u64;
            nat_eq(a1702, a1703)
        } else {
            false
        } {
            let a1707 = {
                let a1704 = v3;
                let a1705 = v7;
                let a1706 = 1u64;
                nat_rem(a1704, a1705, a1706)
            };
            let a1708 = 0u64;
            nat_eq(a1707, a1708)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1709 = v0;
    let a1710 = v1;
    if nat_eq(a1709, a1710) {
        let a1713 = v4;
        let a1714 = {
            let a1711 = v2;
            let a1712 = v3;
            nat_add(a1711, a1712)?
        };
        Ok(nat_eq(a1713, a1714))
    } else {
        let a1715 = v2;
        let a1716 = v3;
        if nat_eq(a1715, a1716) {
            let a1717 = v4;
            let a1718 = v2;
            Ok(nat_eq(a1717, a1718))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1721 = {
            let a1719 = v0;
            let a1720 = f119(v1);
            nat_add(a1719, a1720)?
        };
        let a1722 = f119(v2);
        nat_add(a1721, a1722)?
    };
    let v5: u64 = {
        let a1723 = f120(v1);
        let a1724 = f120(v2);
        nat_add(a1723, a1724)?
    };
    let a1725 = v1;
    let a1726 = 4294967295u64;
    if if nat_le(a1725, a1726) {
        let a1727 = v2;
        let a1728 = 4294967295u64;
        nat_le(a1727, a1728)
    } else {
        false
    } {
        let a1729 = v5;
        let a1730 = v4;
        if nat_le(a1729, a1730) {
            let a1733 = v3;
            let a1734 = {
                let a1731 = v4;
                let a1732 = v5;
                nat_sub(a1731, a1732)
            };
            Ok(nat_eq(a1733, a1734))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1735 = v1;
    let a1736 = 4294967295u64;
    if nat_le(a1735, a1736) {
        let a1739 = {
            let a1737 = f119(v1);
            let a1738 = f120(v1);
            nat_add(a1737, a1738)?
        };
        let a1740 = v0;
        Ok(nat_lt(a1739, a1740))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1741 = v6;
    let a1742 = v1;
    if if nat_le(a1741, a1742) {
        let a1749 = {
            let a1747 = {
                let a1743 = v1;
                let a1744 = v5;
                nat_add(a1743, a1744)?
            };
            let a1748 = {
                let a1745 = v3;
                let a1746 = 0u64;
                if nat_eq(a1745, a1746) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1747, a1748)?
        };
        let a1750 = v2;
        nat_le(a1749, a1750)
    } else {
        false
    } {
        if v4 {
            let a1755 = {
                let a1753 = {
                    let a1751 = v1;
                    let a1752 = v5;
                    nat_add(a1751, a1752)?
                };
                let a1754 = v6;
                nat_sub(a1753, a1754)
            };
            let a1756 = v2;
            Ok(nat_lt(a1755, a1756))
        } else {
            let a1761 = {
                let a1759 = {
                    let a1757 = v1;
                    let a1758 = v5;
                    nat_add(a1757, a1758)?
                };
                let a1760 = v6;
                nat_sub(a1759, a1760)
            };
            let a1762 = v2;
            Ok(nat_le(a1761, a1762))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1763 = v0;
    let a1764 = 0u64;
    if if nat_eq(a1763, a1764) {
        true
    } else {
        let a1765 = v0;
        let a1766 = 1u64;
        nat_eq(a1765, a1766)
    } {
        let a1767 = v1;
        let a1768 = 4294967296u64;
        nat_lt(a1767, a1768)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1773 = {
        let a1771 = v0;
        let a1772 = {
            let a1769 = v1;
            let a1770 = 1u64;
            nat_sub(a1769, a1770)
        };
        nat_add(a1771, a1772)?
    };
    let a1774 = v1;
    let a1775 = 0u64;
    Ok(nat_quot(a1773, a1774, a1775))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1776, _, _, _, _, _, _, _, _, _) => {
            h1776
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1777, _, _, _, _, _, _, _, _) => {
            h1777
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1778, _, _, _, _, _, _, _) => {
            h1778
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1779, _, _, _, _, _, _) => {
            h1779
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1782 = match v1.clone() {
                Adt2::C0(_, h1780, _, _, _, _, _, _, _, _, _) => {
                    h1780
                }
            };
            let a1783 = match v1.clone() {
                Adt2::C0(_, _, h1781, _, _, _, _, _, _, _, _) => {
                    h1781
                }
            };
            nat_mul(a1782, a1783)?
        };
        let v5: u64 = {
            let a1785 = v4;
            let a1786 = match v1.clone() {
                Adt2::C0(_, _, _, h1784, _, _, _, _, _, _, _) => {
                    h1784
                }
            };
            nat_mul(a1785, a1786)?
        };
        let a1799 = {
            let a1797 = {
                let a1788 = match v0.clone() {
                    Adt2::C0(_, h1787, _, _, _, _, _, _, _, _, _) => {
                        h1787
                    }
                };
                let a1789 = 1u64;
                nat_sub(a1788, a1789)
            };
            let a1798 = {
                let a1795 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1790, _, _, _, _, _, _, _, _) => {
                        h1790
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1791, _, _, _, _, _, _, _, _, _) => {
                        h1791
                    }
                })?;
                let a1796 = f122(match v0 {
                    Adt2::C0(_, _, _, h1792, _, _, _, _, _, _, _) => {
                        h1792
                    }
                }, v2, {
                    let a1793 = v3;
                    let a1794 = v5;
                    nat_mul(a1793, a1794)?
                }, v4)?;
                nat_add(a1795, a1796)?
            };
            nat_add(a1797, a1798)?
        };
        let a1800 = f9(v1, 1u64)?;
        Ok(nat_lt(a1799, a1800))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1801 = v0;
    let a1802 = 2147483648u64;
    if if if if nat_lt(a1801, a1802) {
        let a1803 = v1;
        let a1804 = 2147483648u64;
        nat_lt(a1803, a1804)
    } else {
        false
    } {
        let a1807 = {
            let a1805 = v0;
            let a1806 = v1;
            nat_mul(a1805, a1806)?
        };
        let a1808 = 2147483648u64;
        nat_lt(a1807, a1808)
    } else {
        false
    } {
        let a1813 = {
            let a1811 = {
                let a1809 = v0;
                let a1810 = 1u64;
                nat_sub(a1809, a1810)
            };
            let a1812 = v2;
            nat_mul(a1811, a1812)?
        };
        let a1814 = 2147483648u64;
        nat_lt(a1813, a1814)
    } else {
        false
    } {
        let a1819 = {
            let a1817 = {
                let a1815 = v1;
                let a1816 = 1u64;
                nat_sub(a1815, a1816)
            };
            let a1818 = v2;
            nat_mul(a1817, a1818)?
        };
        let a1820 = 2147483648u64;
        Ok(nat_lt(a1819, a1820))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1823 = 3u64;
    let a1824 = {
        let a1821 = v0;
        let a1822 = 3u64;
        nat_mul(a1821, a1822)?
    };
    nat_add(a1823, a1824)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1825, _, _, _, _, _, _, _) => {
            h1825
        }
    }, match v0.clone() {
        Adt3::C0(_, h1826, _, _, _, _, _, _) => {
            h1826
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1827, _, _, _, _, _) => {
            h1827
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1828, _, _, _, _) => {
            h1828
        }
    }) {
        Ok(0u64)
    } else {
        let a1837 = match v0.clone() {
            Adt3::C0(h1829, _, _, _, _, _, _, _) => {
                h1829
            }
        };
        let a1838 = {
            let a1835 = match v0.clone() {
                Adt3::C0(_, h1830, _, _, _, _, _, _) => {
                    h1830
                }
            };
            let a1836 = {
                let a1833 = match v0.clone() {
                    Adt3::C0(_, _, h1831, _, _, _, _, _) => {
                        h1831
                    }
                };
                let a1834 = match v0 {
                    Adt3::C0(_, _, _, h1832, _, _, _, _) => {
                        h1832
                    }
                };
                nat_mul(a1833, a1834)?
            };
            nat_mul(a1835, a1836)?
        };
        nat_mul(a1837, a1838)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1839 = v1;
    let a1840 = 13u64;
    if if nat_eq(a1839, a1840) {
        f23(match v0.clone() {
            Adt3::C0(h1841, _, _, _, _, _, _, _) => {
                h1841
            }
        }, match v0.clone() {
            Adt3::C0(_, h1842, _, _, _, _, _, _) => {
                h1842
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1843, _, _, _, _, _) => {
                h1843
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1844, _, _, _, _) => {
                h1844
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1850 = match v0.clone() {
                Adt3::C0(_, h1845, _, _, _, _, _, _) => {
                    h1845
                }
            };
            let a1851 = {
                let a1848 = match v0.clone() {
                    Adt3::C0(_, _, h1846, _, _, _, _, _) => {
                        h1846
                    }
                };
                let a1849 = match v0.clone() {
                    Adt3::C0(_, _, _, h1847, _, _, _, _) => {
                        h1847
                    }
                };
                nat_mul(a1848, a1849)?
            };
            nat_mul(a1850, a1851)?
        };
        let a1852 = v1;
        let a1853 = 13u64;
        if nat_eq(a1852, a1853) {
            let a1862 = 2u64;
            let a1863 = if f123(v0.clone(), v2) {
                let a1855 = v3;
                let a1856 = f124(match v0 {
                    Adt3::C0(h1854, _, _, _, _, _, _, _) => {
                        h1854
                    }
                }, false)?;
                nat_mul(a1855, a1856)?
            } else {
                let a1860 = v3;
                let a1861 = {
                    let a1858 = match v0 {
                        Adt3::C0(h1857, _, _, _, _, _, _, _) => {
                            h1857
                        }
                    };
                    let a1859 = 2u64;
                    nat_mul(a1858, a1859)?
                };
                nat_mul(a1860, a1861)?
            };
            nat_add(a1862, a1863)
        } else {
            let a1867 = v3;
            let a1868 = f124(match v0 {
                Adt3::C0(h1864, _, _, _, _, _, _, _) => {
                    h1864
                }
            }, {
                let a1865 = v1;
                let a1866 = 16u64;
                nat_eq(a1865, a1866)
            })?;
            nat_mul(a1867, a1868)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1869 = v3;
    match m1869 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1870, _, _, _, _, _, _, _, _, _, _) => {
                    h1870
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1871, _, _, _, _, _, _, _, _, _, _) => {
                    h1871
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1872, _, _, _, _, _, _, _, _, _) => {
                    h1872
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1873, _, _, _, _, _, _, _, _) => {
                    h1873
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1874, _, _, _, _, _, _, _) => {
                    h1874
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1875, _, _, _, _, _, _) => {
                    h1875
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1876, _, _, _, _, _, _, _, _, _) => {
                    h1876
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1877, _, _, _, _, _, _, _, _) => {
                    h1877
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1878, _, _, _, _, _, _, _) => {
                    h1878
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1879, _, _, _, _, _, _) => {
                    h1879
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1880, _, _, _, _, _, _, _, _, _, _) => {
                    h1880
                }
            }, match v5.clone() {
                Adt2::C0(h1881, _, _, _, _, _, _, _, _, _, _) => {
                    h1881
                }
            }, match v4.clone() {
                Adt2::C0(h1882, _, _, _, _, _, _, _, _, _, _) => {
                    h1882
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1883, _, _, _, _, _, _, _, _, _) => {
                    h1883
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1884, _, _, _, _, _, _, _, _) => {
                    h1884
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1885, _, _, _, _, _, _, _) => {
                    h1885
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1886, _, _, _, _, _, _) => {
                    h1886
                }
            }, match v5.clone() {
                Adt2::C0(_, h1887, _, _, _, _, _, _, _, _, _) => {
                    h1887
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1888, _, _, _, _, _, _, _, _) => {
                    h1888
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1889, _, _, _, _, _, _, _) => {
                    h1889
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1890, _, _, _, _, _, _) => {
                    h1890
                }
            }, match v4.clone() {
                Adt2::C0(_, h1891, _, _, _, _, _, _, _, _, _) => {
                    h1891
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1892, _, _, _, _, _, _, _, _) => {
                    h1892
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1893, _, _, _, _, _, _, _) => {
                    h1893
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1894, _, _, _, _, _, _) => {
                    h1894
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1895 = v7;
        let a1896 = 6u64;
        if nat_eq(a1895, a1896) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1897 = v0;
    match m1897 {
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
        Adt4::C14(_, _, _) => {
            Adt7::C3(v1, v2, true)
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
    let m1898 = v0;
    match m1898 {
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
    let m1899 = v0;
    match m1899 {
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
            let a1900 = v1;
            let a1901 = 6u64;
            if nat_eq(a1900, a1901) {
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
            let a1902 = v1;
            let a1903 = 6u64;
            if if if nat_eq(a1902, a1903) {
                true
            } else {
                let a1904 = v1;
                let a1905 = 7u64;
                nat_eq(a1904, a1905)
            } {
                let a1906 = v49;
                let a1907 = 30u64;
                nat_eq(a1906, a1907)
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
    let a1908 = v1;
    let a1909 = 4u64;
    if nat_lt(a1908, a1909) {
        Adt7::C0(v1)
    } else {
        let a1910 = v1;
        let a1911 = 4u64;
        if nat_eq(a1910, a1911) {
            Adt7::C1(0u64)
        } else {
            let a1912 = v1;
            let a1913 = 5u64;
            if nat_eq(a1912, a1913) {
                Adt7::C1(3u64)
            } else {
                let a1914 = v1;
                let a1915 = 6u64;
                if nat_eq(a1914, a1915) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1916 = v1;
                    let a1917 = 7u64;
                    if nat_eq(a1916, a1917) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1918 = v1;
                        let a1919 = 16u64;
                        if nat_lt(a1918, a1919) {
                            Adt7::C1({
                                let a1920 = v1;
                                let a1921 = 8u64;
                                nat_sub(a1920, a1921)
                            })
                        } else {
                            let a1922 = v1;
                            let a1923 = 20u64;
                            if nat_lt(a1922, a1923) {
                                Adt7::C5({
                                    let a1924 = v1;
                                    let a1925 = 16u64;
                                    nat_sub(a1924, a1925)
                                })
                            } else {
                                let a1926 = v1;
                                let a1927 = 20u64;
                                if nat_eq(a1926, a1927) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1928 = v1;
                                    let a1929 = 21u64;
                                    if nat_eq(a1928, a1929) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1930 = v1;
                                        let a1931 = 22u64;
                                        if nat_eq(a1930, a1931) {
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
        let a1933 = v0;
        let a1934 = match v1.clone() {
            Adt3::C0(h1932, _, _, _, _, _, _, _) => {
                h1932
            }
        };
        let a1935 = 0u64;
        nat_quot(a1933, a1934, a1935)
    };
    let v3: u64 = {
        let a1937 = v2;
        let a1938 = match v1.clone() {
            Adt3::C0(_, h1936, _, _, _, _, _, _) => {
                h1936
            }
        };
        let a1939 = 0u64;
        nat_quot(a1937, a1938, a1939)
    };
    f129({
        let a1941 = v0;
        let a1942 = match v1.clone() {
            Adt3::C0(h1940, _, _, _, _, _, _, _) => {
                h1940
            }
        };
        let a1943 = 0u64;
        nat_rem(a1941, a1942, a1943)
    }, {
        let a1945 = v2;
        let a1946 = match v1.clone() {
            Adt3::C0(_, h1944, _, _, _, _, _, _) => {
                h1944
            }
        };
        let a1947 = 0u64;
        nat_rem(a1945, a1946, a1947)
    }, {
        let a1949 = v3;
        let a1950 = match v1.clone() {
            Adt3::C0(_, _, h1948, _, _, _, _, _) => {
                h1948
            }
        };
        let a1951 = 0u64;
        nat_rem(a1949, a1950, a1951)
    }, {
        let a1953 = v3;
        let a1954 = match v1.clone() {
            Adt3::C0(_, _, h1952, _, _, _, _, _) => {
                h1952
            }
        };
        let a1955 = 0u64;
        nat_quot(a1953, a1954, a1955)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1956, _, _, _) => {
            h1956
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1957, _, _) => {
            h1957
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1958, _) => {
            h1958
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1959) => {
            h1959
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1977 = {
        let a1972 = f130(v2, {
            let a1963 = {
                let a1960 = v0;
                let a1961 = v1;
                let a1962 = 0u64;
                nat_quot(a1960, a1961, a1962)
            };
            let a1964 = v2;
            let a1965 = 0u64;
            nat_rem(a1963, a1964, a1965)
        }, {
            let a1969 = {
                let a1966 = v0;
                let a1967 = v1;
                let a1968 = 0u64;
                nat_quot(a1966, a1967, a1968)
            };
            let a1970 = v2;
            let a1971 = 0u64;
            nat_quot(a1969, a1970, a1971)
        })?;
        let a1973 = v1;
        nat_mul(a1972, a1973)?
    };
    let a1978 = {
        let a1974 = v0;
        let a1975 = v1;
        let a1976 = 0u64;
        nat_rem(a1974, a1975, a1976)
    };
    nat_add(a1977, a1978)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1980 = v0;
        let a1981 = match v2.clone() {
            Adt3::C0(h1979, _, _, _, _, _, _, _) => {
                h1979
            }
        };
        let a1982 = 0u64;
        nat_quot(a1980, a1981, a1982)
    };
    let v4: u64 = {
        let a1984 = v3;
        let a1985 = match v2.clone() {
            Adt3::C0(_, h1983, _, _, _, _, _, _) => {
                h1983
            }
        };
        let a1986 = 0u64;
        nat_quot(a1984, a1985, a1986)
    };
    f131({
        let a1988 = v0;
        let a1989 = match v2.clone() {
            Adt3::C0(h1987, _, _, _, _, _, _, _) => {
                h1987
            }
        };
        let a1990 = 0u64;
        nat_rem(a1988, a1989, a1990)
    }, {
        let a1992 = v3;
        let a1993 = match v2.clone() {
            Adt3::C0(_, h1991, _, _, _, _, _, _) => {
                h1991
            }
        };
        let a1994 = 0u64;
        nat_rem(a1992, a1993, a1994)
    }, {
        let a1996 = v4;
        let a1997 = match v2.clone() {
            Adt3::C0(_, _, h1995, _, _, _, _, _) => {
                h1995
            }
        };
        let a1998 = 0u64;
        nat_rem(a1996, a1997, a1998)
    }, {
        let a2000 = v4;
        let a2001 = match v2 {
            Adt3::C0(_, _, h1999, _, _, _, _, _) => {
                h1999
            }
        };
        let a2002 = 0u64;
        nat_quot(a2000, a2001, a2002)
    }, match v1.clone() {
        Adt3::C0(h2003, _, _, _, _, _, _, _) => {
            h2003
        }
    }, match v1.clone() {
        Adt3::C0(_, h2004, _, _, _, _, _, _) => {
            h2004
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2005, _, _, _, _, _) => {
            h2005
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2006, _, _, _, _) => {
            h2006
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2007, _, _, _) => {
            h2007
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2008, _, _) => {
            h2008
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2009, _) => {
            h2009
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2010) => {
            h2010
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2012 = v0;
        let a2013 = match v3.clone() {
            Adt3::C0(h2011, _, _, _, _, _, _, _) => {
                h2011
            }
        };
        let a2014 = 0u64;
        nat_quot(a2012, a2013, a2014)
    };
    let v6: u64 = {
        let a2016 = v5;
        let a2017 = match v3.clone() {
            Adt3::C0(_, h2015, _, _, _, _, _, _) => {
                h2015
            }
        };
        let a2018 = 0u64;
        nat_quot(a2016, a2017, a2018)
    };
    let v7: u64 = {
        let a2020 = v0;
        let a2021 = match v3.clone() {
            Adt3::C0(h2019, _, _, _, _, _, _, _) => {
                h2019
            }
        };
        let a2022 = 0u64;
        nat_rem(a2020, a2021, a2022)
    };
    let v8: u64 = {
        let a2024 = v5;
        let a2025 = match v3.clone() {
            Adt3::C0(_, h2023, _, _, _, _, _, _) => {
                h2023
            }
        };
        let a2026 = 0u64;
        nat_rem(a2024, a2025, a2026)
    };
    let v9: u64 = {
        let a2028 = v6;
        let a2029 = match v3.clone() {
            Adt3::C0(_, _, h2027, _, _, _, _, _) => {
                h2027
            }
        };
        let a2030 = 0u64;
        nat_rem(a2028, a2029, a2030)
    };
    let v10: u64 = {
        let a2032 = v6;
        let a2033 = match v3.clone() {
            Adt3::C0(_, _, h2031, _, _, _, _, _) => {
                h2031
            }
        };
        let a2034 = 0u64;
        nat_quot(a2032, a2033, a2034)
    };
    let v11: u64 = {
        let a2035 = v4;
        let a2036 = 0u64;
        if nat_eq(a2035, a2036) {
            match v1.clone() {
                Adt3::C0(h2037, _, _, _, _, _, _, _) => {
                    h2037
                }
            }
        } else {
            let a2038 = v4;
            let a2039 = 1u64;
            if nat_eq(a2038, a2039) {
                match v1.clone() {
                    Adt3::C0(_, h2040, _, _, _, _, _, _) => {
                        h2040
                    }
                }
            } else {
                let a2041 = v4;
                let a2042 = 2u64;
                if nat_eq(a2041, a2042) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2043, _, _, _, _, _) => {
                            h2043
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2044, _, _, _, _) => {
                            h2044
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2051 = v11;
        let a2052 = {
            let a2045 = v4;
            let a2046 = 0u64;
            if nat_eq(a2045, a2046) {
                v7
            } else {
                let a2047 = v4;
                let a2048 = 1u64;
                if nat_eq(a2047, a2048) {
                    v8
                } else {
                    let a2049 = v4;
                    let a2050 = 2u64;
                    if nat_eq(a2049, a2050) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2051, a2052)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2053 = v4;
        let a2054 = 0u64;
        nat_eq(a2053, a2054)
    } else {
        false
    } {
        let a2055 = v7;
        let a2056 = v11;
        nat_sub(a2055, a2056)
    } else {
        v7
    }, if if v12 {
        let a2057 = v4;
        let a2058 = 1u64;
        nat_eq(a2057, a2058)
    } else {
        false
    } {
        let a2059 = v8;
        let a2060 = v11;
        nat_sub(a2059, a2060)
    } else {
        v8
    }, if if v12 {
        let a2061 = v4;
        let a2062 = 2u64;
        nat_eq(a2061, a2062)
    } else {
        false
    } {
        let a2063 = v9;
        let a2064 = v11;
        nat_sub(a2063, a2064)
    } else {
        v9
    }, if if v12 {
        let a2065 = v4;
        let a2066 = 3u64;
        nat_eq(a2065, a2066)
    } else {
        false
    } {
        let a2067 = v10;
        let a2068 = v11;
        nat_sub(a2067, a2068)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2069, _, _, _) => {
            h2069
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2070, _, _) => {
            h2070
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2071, _) => {
            h2071
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2072) => {
            h2072
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
            Adt3::C0(h2073, _, _, _, _, _, _, _) => {
                h2073
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2074, _, _, _, _, _, _) => {
                h2074
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2075, _, _, _, _, _) => {
                h2075
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2076, _, _, _, _) => {
                h2076
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2077, _, _, _, _, _, _, _) => {
                h2077
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2078, _, _, _, _, _, _) => {
                    h2078
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2079, _, _, _, _, _) => {
                    h2079
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2080, _, _, _, _) => {
                    h2080
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
        Adt3::C0(h2081, _, _, _, _, _, _, _) => {
            h2081
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2082 = v0;
    let a2083 = v3;
    if nat_lt(a2082, a2083) {
        Ok(Adt5::C1(false, {
            let a2084 = v0;
            let a2085 = 4u64;
            nat_mul(a2084, a2085)?
        }, {
            let a2086 = v0;
            let a2087 = 4u64;
            nat_mul(a2086, a2087)?
        }))
    } else {
        let v6: u64 = {
            let a2088 = v0;
            let a2089 = v3;
            nat_sub(a2088, a2089)
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
    let a2090 = v0;
    let a2091 = v3;
    if nat_lt(a2090, a2091) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2092 = v0;
            let a2093 = v3;
            nat_sub(a2092, a2093)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2095 = v1;
        let a2096 = match v3.clone() {
            Adt3::C0(h2094, _, _, _, _, _, _, _) => {
                h2094
            }
        };
        let a2097 = 0u64;
        nat_rem(a2095, a2096, a2097)
    };
    let v5: u64 = {
        let a2103 = {
            let a2099 = v1;
            let a2100 = match v3.clone() {
                Adt3::C0(h2098, _, _, _, _, _, _, _) => {
                    h2098
                }
            };
            let a2101 = 0u64;
            nat_quot(a2099, a2100, a2101)
        };
        let a2104 = match v3.clone() {
            Adt3::C0(_, h2102, _, _, _, _, _, _) => {
                h2102
            }
        };
        let a2105 = 0u64;
        nat_rem(a2103, a2104, a2105)
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
        let a2106 = v3;
        let a2107 = 8u64;
        nat_add(a2106, a2107)?
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
        let a2109 = v0;
        let a2110 = match v2.clone() {
            Adt3::C0(h2108, _, _, _, _, _, _, _) => {
                h2108
            }
        };
        let a2111 = 0u64;
        nat_rem(a2109, a2110, a2111)
    };
    let v7: u64 = {
        let a2113 = v0;
        let a2114 = match v2.clone() {
            Adt3::C0(h2112, _, _, _, _, _, _, _) => {
                h2112
            }
        };
        let a2115 = 0u64;
        nat_quot(a2113, a2114, a2115)
    };
    let v8: u64 = {
        let a2117 = v7;
        let a2118 = match v2.clone() {
            Adt3::C0(_, h2116, _, _, _, _, _, _) => {
                h2116
            }
        };
        let a2119 = 0u64;
        nat_rem(a2117, a2118, a2119)
    };
    let v9: u64 = {
        let a2121 = v7;
        let a2122 = match v2.clone() {
            Adt3::C0(_, h2120, _, _, _, _, _, _) => {
                h2120
            }
        };
        let a2123 = 0u64;
        nat_quot(a2121, a2122, a2123)
    };
    let v10: u64 = {
        let a2125 = v9;
        let a2126 = match v2.clone() {
            Adt3::C0(_, _, h2124, _, _, _, _, _) => {
                h2124
            }
        };
        let a2127 = 0u64;
        nat_rem(a2125, a2126, a2127)
    };
    let v11: u64 = {
        let a2129 = v9;
        let a2130 = match v2.clone() {
            Adt3::C0(_, _, h2128, _, _, _, _, _) => {
                h2128
            }
        };
        let a2131 = 0u64;
        nat_quot(a2129, a2130, a2131)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2140 = {
                let a2135 = {
                    let a2132 = v10;
                    let a2133 = v4;
                    let a2134 = 0u64;
                    nat_quot(a2132, a2133, a2134)
                };
                let a2136 = v5;
                nat_mul(a2135, a2136)?
            };
            let a2141 = {
                let a2137 = v8;
                let a2138 = v4;
                let a2139 = 0u64;
                nat_quot(a2137, a2138, a2139)
            };
            nat_add(a2140, a2141)?
        };
        let v14: u64 = {
            let a2161 = v6;
            let a2162 = {
                let a2159 = match v1.clone() {
                    Adt3::C0(h2142, _, _, _, _, _, _, _) => {
                        h2142
                    }
                };
                let a2160 = {
                    let a2157 = {
                        let a2143 = v8;
                        let a2144 = v4;
                        let a2145 = 0u64;
                        nat_rem(a2143, a2144, a2145)
                    };
                    let a2158 = {
                        let a2155 = match v1.clone() {
                            Adt3::C0(_, h2146, _, _, _, _, _, _) => {
                                h2146
                            }
                        };
                        let a2156 = {
                            let a2153 = {
                                let a2147 = v10;
                                let a2148 = v4;
                                let a2149 = 0u64;
                                nat_rem(a2147, a2148, a2149)
                            };
                            let a2154 = {
                                let a2151 = match v1.clone() {
                                    Adt3::C0(_, _, h2150, _, _, _, _, _) => {
                                        h2150
                                    }
                                };
                                let a2152 = v13;
                                nat_mul(a2151, a2152)?
                            };
                            nat_add(a2153, a2154)?
                        };
                        nat_mul(a2155, a2156)?
                    };
                    nat_add(a2157, a2158)?
                };
                nat_mul(a2159, a2160)?
            };
            nat_add(a2161, a2162)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2168 = {
                let a2166 = {
                    let a2163 = v11;
                    let a2164 = v5;
                    let a2165 = 0u64;
                    nat_rem(a2163, a2164, a2165)
                };
                let a2167 = v4;
                nat_mul(a2166, a2167)?
            };
            let a2169 = v8;
            nat_add(a2168, a2169)?
        };
        let v16: u64 = {
            let a2175 = {
                let a2173 = {
                    let a2170 = v11;
                    let a2171 = v5;
                    let a2172 = 0u64;
                    nat_quot(a2170, a2171, a2172)
                };
                let a2174 = v4;
                nat_mul(a2173, a2174)?
            };
            let a2176 = v10;
            nat_add(a2175, a2176)?
        };
        let a2178 = v15;
        let a2179 = match v1.clone() {
            Adt3::C0(_, h2177, _, _, _, _, _, _) => {
                h2177
            }
        };
        if if nat_lt(a2178, a2179) {
            let a2181 = v16;
            let a2182 = match v1.clone() {
                Adt3::C0(_, _, h2180, _, _, _, _, _) => {
                    h2180
                }
            };
            nat_lt(a2181, a2182)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2191 = v6;
                let a2192 = {
                    let a2189 = match v1.clone() {
                        Adt3::C0(h2183, _, _, _, _, _, _, _) => {
                            h2183
                        }
                    };
                    let a2190 = {
                        let a2187 = v15;
                        let a2188 = {
                            let a2185 = match v1.clone() {
                                Adt3::C0(_, h2184, _, _, _, _, _, _) => {
                                    h2184
                                }
                            };
                            let a2186 = v16;
                            nat_mul(a2185, a2186)?
                        };
                        nat_add(a2187, a2188)?
                    };
                    nat_mul(a2189, a2190)?
                };
                nat_add(a2191, a2192)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2197 = {
        let a2194 = v0;
        let a2195 = match v1 {
            Adt3::C0(h2193, _, _, _, _, _, _, _) => {
                h2193
            }
        };
        let a2196 = 0u64;
        nat_rem(a2194, a2195, a2196)
    };
    let a2198 = 0u64;
    if nat_eq(a2197, a2198) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2199 = v0;
            let a2200 = 1u64;
            nat_sub(a2199, a2200)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2201 = v3;
    let a2202 = 13u64;
    if nat_eq(a2201, a2202) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2203, _, _, _, _, _, _, _) => {
                h2203
            }
        }, {
            let a2204 = v3;
            let a2205 = 16u64;
            nat_eq(a2204, a2205)
        })?;
        let v6: u64 = {
            let a2206 = v0;
            let a2207 = v5;
            let a2208 = 0u64;
            nat_quot(a2206, a2207, a2208)
        };
        let v7: u64 = {
            let a2209 = v0;
            let a2210 = v5;
            let a2211 = 0u64;
            nat_rem(a2209, a2210, a2211)
        };
        let v8: u64 = {
            let a2215 = 1u64;
            let a2216 = {
                let a2213 = match v1.clone() {
                    Adt3::C0(h2212, _, _, _, _, _, _, _) => {
                        h2212
                    }
                };
                let a2214 = 2u64;
                nat_mul(a2213, a2214)?
            };
            nat_add(a2215, a2216)?
        };
        let a2217 = v7;
        let a2218 = v8;
        if nat_lt(a2217, a2218) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2219 = v3;
            let a2220 = 14u64;
            if nat_eq(a2219, a2220) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2221 = v7;
                let a2222 = v8;
                if nat_eq(a2221, a2222) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2225 = v7;
                    let a2226 = {
                        let a2223 = v8;
                        let a2224 = 1u64;
                        nat_add(a2223, a2224)?
                    };
                    if nat_eq(a2225, a2226) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2227, _, _, _, _, _, _, _) => {
                                h2227
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
    let a2228 = v0;
    let a2229 = 0u64;
    if nat_eq(a2228, a2229) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2236 = {
            let a2230 = v0;
            let a2231 = 1u64;
            nat_add(a2230, a2231)?
        };
        let a2237 = {
            let a2234 = 2u64;
            let a2235 = {
                let a2232 = v3;
                let a2233 = 2u64;
                nat_mul(a2232, a2233)?
            };
            nat_add(a2234, a2235)?
        };
        if nat_eq(a2236, a2237) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2238 = v0;
                let a2239 = 1u64;
                nat_sub(a2238, a2239)
            };
            let v5: u64 = {
                let a2240 = v4;
                let a2241 = 2u64;
                let a2242 = 0u64;
                nat_quot(a2240, a2241, a2242)
            };
            let a2246 = {
                let a2243 = v4;
                let a2244 = 2u64;
                let a2245 = 0u64;
                nat_rem(a2243, a2244, a2245)
            };
            let a2247 = 0u64;
            if nat_eq(a2246, a2247) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2248 = v0;
    let a2249 = f18(v1, v2, v3)?;
    if nat_lt(a2248, a2249) {
        let a2250 = v0;
        let a2251 = v1;
        if nat_lt(a2250, a2251) {
            Ok(Adt5::C1(false, {
                let a2252 = 4u64;
                let a2253 = v0;
                nat_mul(a2252, a2253)?
            }, {
                let a2254 = 4u64;
                let a2255 = v0;
                nat_mul(a2254, a2255)?
            }))
        } else {
            let v4: u64 = {
                let a2256 = v0;
                let a2257 = v1;
                nat_sub(a2256, a2257)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2258 = v5;
            let a2259 = 2u64;
            if nat_lt(a2258, a2259) {
                Ok(Adt5::C4(6u64, {
                    let a2260 = v5;
                    let a2261 = 0u64;
                    if nat_eq(a2260, a2261) {
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
    let a2269 = {
        let a2262 = 2u64;
        let a2263 = f81(v1.clone())?;
        nat_mul(a2262, a2263)?
    };
    let a2270 = {
        let a2267 = f81(v1)?;
        let a2268 = {
            let a2265 = 1u64;
            let a2266 = match v0 {
                Adt3::C0(h2264, _, _, _, _, _, _, _) => {
                    h2264
                }
            };
            nat_add(a2265, a2266)?
        };
        nat_mul(a2267, a2268)?
    };
    nat_add(a2269, a2270)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2271 = v1;
    let a2272 = 2147483647u64;
    if nat_le(a2271, a2272) {
        let a2273 = v1;
        let a2274 = v0;
        nat_lt(a2273, a2274)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2275 = f70(v0);
    match m2275 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2277 = v2;
                let a2278 = match v7.clone() {
                    Adt11::C0(_, h2276, _) => {
                        h2276
                    }
                };
                nat_mul(a2277, a2278)?
            };
            let m2291 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2279, _, _, _, _, _, _, _) => {
                    h2279
                }
            }, match v5.clone() {
                Adt3::C0(_, h2280, _, _, _, _, _, _) => {
                    h2280
                }
            }, match v5 {
                Adt3::C0(_, _, h2281, _, _, _, _, _) => {
                    h2281
                }
            }, {
                let a2284 = match v3.clone() {
                    Adt3::C0(h2282, _, _, _, _, _, _, _) => {
                        h2282
                    }
                };
                let a2285 = match v7.clone() {
                    Adt11::C0(_, h2283, _) => {
                        h2283
                    }
                };
                nat_mul(a2284, a2285)?
            }, match v3.clone() {
                Adt3::C0(_, h2286, _, _, _, _, _, _) => {
                    h2286
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2287, _, _, _) => {
                    h2287
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2288, _, _) => {
                    h2288
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2289, _) => {
                    h2289
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2290) => {
                    h2290
                }
            })?;
            match m2291 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2292 = v0;
                    let a2293 = 0u64;
                    if if nat_eq(a2292, a2293) {
                        true
                    } else {
                        let a2294 = v0;
                        let a2295 = 26u64;
                        nat_eq(a2294, a2295)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2299 = {
                                let a2297 = v1;
                                let a2298 = match v4.clone() {
                                    Adt3::C0(h2296, _, _, _, _, _, _, _) => {
                                        h2296
                                    }
                                };
                                nat_mul(a2297, a2298)?
                            };
                            let a2300 = v8;
                            nat_add(a2299, a2300)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2301) => {
                                h2301
                            }
                        }, f89({
                            let a2305 = {
                                let a2303 = v1;
                                let a2304 = match v4.clone() {
                                    Adt3::C0(h2302, _, _, _, _, _, _, _) => {
                                        h2302
                                    }
                                };
                                nat_mul(a2303, a2304)?
                            };
                            let a2306 = v8;
                            nat_add(a2305, a2306)?
                        }, v4.clone())?, {
                            let a2313 = {
                                let a2311 = {
                                    let a2308 = match v7 {
                                        Adt11::C0(_, h2307, _) => {
                                            h2307
                                        }
                                    };
                                    let a2309 = 1u64;
                                    nat_sub(a2308, a2309)
                                };
                                let a2312 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2310, _, _, _) => {
                                        h2310
                                    }
                                };
                                nat_mul(a2311, a2312)?
                            };
                            let a2314 = 4u64;
                            nat_add(a2313, a2314)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2315, _, _, _) => {
                                h2315
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2316 = v1;
    let a2317 = 0u64;
    if if nat_eq(a2316, a2317) {
        true
    } else {
        let a2318 = v2;
        let a2319 = 0u64;
        nat_eq(a2318, a2319)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2320 = v0;
    let a2321 = 4294967295u64;
    if nat_eq(a2320, a2321) {
        true
    } else {
        let a2322 = v0;
        let a2323 = 2147483648u64;
        if nat_lt(a2322, a2323) {
            let a2324 = v0;
            let a2325 = v1;
            nat_lt(a2324, a2325)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2332 = v3;
        let a2333 = {
            let a2329 = f89({
                let a2326 = v0;
                let a2327 = 2u64;
                let a2328 = 0u64;
                nat_quot(a2326, a2327, a2328)
            }, v1)?;
            let a2330 = v2;
            let a2331 = 0u64;
            nat_quot(a2329, a2330, a2331)
        };
        nat_add(a2332, a2333)?
    };
    let a2337 = {
        let a2334 = v0;
        let a2335 = 2u64;
        let a2336 = 0u64;
        nat_rem(a2334, a2335, a2336)
    };
    let a2338 = 0u64;
    if nat_eq(a2337, a2338) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2339 = v4;
        let a2340 = 0u64;
        if nat_eq(a2339, a2340) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2343 = 8u64;
    let a2344 = {
        let a2341 = f147(v0);
        let a2342 = v1;
        nat_mul(a2341, a2342)?
    };
    nat_add(a2343, a2344)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2345, _, _, _, _, _, _, _) => {
            h2345
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2348 = 1u64;
        let a2349 = {
            let a2346 = v6;
            let a2347 = v7;
            nat_mul(a2346, a2347)?
        };
        nat_add(a2348, a2349)?
    };
    let a2350 = v0;
    let a2351 = 0u64;
    if nat_eq(a2350, a2351) {
        Ok(f148(0u64))
    } else {
        let a2352 = v0;
        let a2353 = v8;
        if nat_lt(a2352, a2353) {
            let v9: u64 = {
                let a2354 = v0;
                let a2355 = 1u64;
                nat_sub(a2354, a2355)
            };
            let v10: u64 = {
                let a2356 = v9;
                let a2357 = v7;
                let a2358 = 0u64;
                nat_rem(a2356, a2357, a2358)
            };
            let v11: u64 = f89({
                let a2364 = {
                    let a2359 = v1;
                    let a2360 = v6;
                    nat_mul(a2359, a2360)?
                };
                let a2365 = {
                    let a2361 = v9;
                    let a2362 = v7;
                    let a2363 = 0u64;
                    nat_quot(a2361, a2362, a2363)
                };
                nat_add(a2364, a2365)?
            }, v2)?;
            let a2366 = v3;
            let a2367 = 30u64;
            if nat_eq(a2366, a2367) {
                let a2368 = v10;
                let a2369 = 0u64;
                if nat_eq(a2368, a2369) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2370 = v10;
                    let a2371 = 1u64;
                    if nat_eq(a2370, a2371) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2372 = v10;
                        let a2373 = 2u64;
                        if nat_eq(a2372, a2373) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2374 = v10;
                let a2375 = 0u64;
                if nat_eq(a2374, a2375) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2376 = v10;
                    let a2377 = 1u64;
                    if nat_eq(a2376, a2377) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2378 = v0;
                let a2379 = v8;
                nat_sub(a2378, a2379)
            };
            let a2380 = v12;
            let a2381 = 0u64;
            if nat_eq(a2380, a2381) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2382 = v12;
                let a2383 = 1u64;
                if nat_eq(a2382, a2383) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2384 = v12;
                    let a2385 = 2u64;
                    if nat_eq(a2384, a2385) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2386 = v12;
                        let a2387 = 3u64;
                        if nat_eq(a2386, a2387) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2388 = v12;
                            let a2389 = 4u64;
                            if nat_eq(a2388, a2389) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2390 = v12;
                                let a2391 = 5u64;
                                if nat_eq(a2390, a2391) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2392 = v12;
                                    let a2393 = 6u64;
                                    if nat_eq(a2392, a2393) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2394 = 3u64;
                                                let a2395 = v1;
                                                nat_add(a2394, a2395)?
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
        let a2396 = v0;
        let a2397 = f65(v3);
        let a2398 = 0u64;
        nat_quot(a2396, a2397, a2398)
    };
    let v5: u64 = {
        let a2403 = 3u64;
        let a2404 = {
            let a2400 = v4;
            let a2401 = match v1.clone() {
                Adt3::C0(h2399, _, _, _, _, _, _, _) => {
                    h2399
                }
            };
            let a2402 = 0u64;
            nat_quot(a2400, a2401, a2402)
        };
        nat_add(a2403, a2404)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2405 = v3;
    let a2406 = 30u64;
    if nat_eq(a2405, a2406) {
        let a2410 = {
            let a2407 = v0;
            let a2408 = 3u64;
            let a2409 = 0u64;
            nat_rem(a2407, a2408, a2409)
        };
        let a2411 = 0u64;
        if nat_eq(a2410, a2411) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2415 = {
                let a2412 = v0;
                let a2413 = 3u64;
                let a2414 = 0u64;
                nat_rem(a2412, a2413, a2414)
            };
            let a2416 = 1u64;
            if nat_eq(a2415, a2416) {
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
        let a2417 = v0;
        let a2418 = 8388608u64;
        let a2419 = 0u64;
        nat_quot(a2417, a2418, a2419)
    };
    let a2420 = v0;
    let a2421 = 2147483648u64;
    if if if nat_lt(a2420, a2421) {
        let a2422 = 127u64;
        let a2423 = v1;
        nat_le(a2422, a2423)
    } else {
        false
    } {
        let a2424 = v1;
        let a2425 = 190u64;
        nat_lt(a2424, a2425)
    } else {
        false
    } {
        let v2: u64 = {
            let a2429 = 8388608u64;
            let a2430 = {
                let a2426 = v0;
                let a2427 = 8388608u64;
                let a2428 = 0u64;
                nat_rem(a2426, a2427, a2428)
            };
            nat_add(a2429, a2430)?
        };
        let a2431 = v1;
        let a2432 = 150u64;
        if nat_lt(a2431, a2432) {
            let v3: u64 = f150({
                let a2433 = 150u64;
                let a2434 = v1;
                nat_sub(a2433, a2434)
            });
            let a2438 = {
                let a2435 = v2;
                let a2436 = v3;
                let a2437 = 0u64;
                nat_rem(a2435, a2436, a2437)
            };
            let a2439 = 0u64;
            if nat_eq(a2438, a2439) {
                let a2440 = v2;
                let a2441 = v3;
                let a2442 = 0u64;
                Ok(nat_quot(a2440, a2441, a2442))
            } else {
                Ok(0u64)
            }
        } else {
            let a2445 = f150({
                let a2443 = v1;
                let a2444 = 150u64;
                nat_sub(a2443, a2444)
            });
            let a2446 = v2;
            nat_mul(a2445, a2446)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2447 = v0;
    let a2448 = 1u64;
    if nat_le(a2447, a2448) {
        true
    } else {
        let a2452 = {
            let a2449 = v1;
            let a2450 = v2;
            let a2451 = 1u64;
            nat_rem(a2449, a2450, a2451)
        };
        let a2453 = 0u64;
        nat_eq(a2452, a2453)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2454 = v1;
    let a2455 = 1u64;
    if if if nat_le(a2454, a2455) {
        true
    } else {
        let a2456 = v0;
        let a2457 = v2;
        nat_le(a2456, a2457)
    } {
        let a2458 = v3;
        let a2459 = 1u64;
        if nat_le(a2458, a2459) {
            true
        } else {
            let a2466 = {
                let a2464 = v0;
                let a2465 = {
                    let a2462 = {
                        let a2460 = v1;
                        let a2461 = 1u64;
                        nat_sub(a2460, a2461)
                    };
                    let a2463 = v2;
                    nat_mul(a2462, a2463)?
                };
                nat_add(a2464, a2465)?
            };
            let a2467 = v4;
            nat_le(a2466, a2467)
        }
    } else {
        false
    } {
        let a2468 = v5;
        let a2469 = 1u64;
        if nat_le(a2468, a2469) {
            Ok(true)
        } else {
            let a2482 = {
                let a2480 = {
                    let a2474 = v0;
                    let a2475 = {
                        let a2472 = {
                            let a2470 = v1;
                            let a2471 = 1u64;
                            nat_sub(a2470, a2471)
                        };
                        let a2473 = v2;
                        nat_mul(a2472, a2473)?
                    };
                    nat_add(a2474, a2475)?
                };
                let a2481 = {
                    let a2478 = {
                        let a2476 = v3;
                        let a2477 = 1u64;
                        nat_sub(a2476, a2477)
                    };
                    let a2479 = v4;
                    nat_mul(a2478, a2479)?
                };
                nat_add(a2480, a2481)?
            };
            let a2483 = v6;
            Ok(nat_le(a2482, a2483))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2484 = v5;
        let a2485 = 26u64;
        nat_eq(a2484, a2485)
    } else {
        false
    } {
        let a2486 = v3;
        let a2487 = v7;
        nat_eq(a2486, a2487)
    } else {
        false
    } {
        let a2488 = v4;
        let a2489 = v8;
        nat_eq(a2488, a2489)
    } else {
        false
    } {
        let a2490 = v9;
        let a2491 = 1u64;
        nat_eq(a2490, a2491)
    } else {
        false
    } {
        let a2492 = v10;
        let a2493 = f152(v0);
        nat_eq(a2492, a2493)
    } else {
        false
    } {
        let a2494 = v11;
        let a2495 = v1;
        nat_eq(a2494, a2495)
    } else {
        false
    } {
        let a2496 = v12;
        let a2497 = v6;
        nat_eq(a2496, a2497)
    } else {
        false
    } {
        let a2498 = v13;
        let a2499 = v7;
        nat_eq(a2498, a2499)
    } else {
        false
    } {
        let a2500 = v14;
        let a2501 = v8;
        nat_eq(a2500, a2501)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2502 = v0;
    let a2503 = 0u64;
    if if if nat_eq(a2502, a2503) {
        true
    } else {
        let a2504 = v1;
        let a2505 = 0u64;
        nat_eq(a2504, a2505)
    } {
        true
    } else {
        let a2506 = v2;
        let a2507 = 0u64;
        nat_eq(a2506, a2507)
    } {
        true
    } else {
        let a2508 = v3;
        let a2509 = 0u64;
        nat_eq(a2508, a2509)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2510 = v0;
    let a2511 = 2147483648u64;
    if nat_lt(a2510, a2511) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2512 = v0;
    let a2513 = 2147483648u64;
    if nat_lt(a2512, a2513) {
        0u64
    } else {
        let a2514 = 4294967296u64;
        let a2515 = v0;
        nat_sub(a2514, a2515)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2517 = match v0.clone() {
            Adt3::C0(h2516, _, _, _, _, _, _, _) => {
                h2516
            }
        };
        let a2518 = 1u64;
        nat_sub(a2517, a2518)
    }, {
        let a2520 = match v0.clone() {
            Adt3::C0(_, h2519, _, _, _, _, _, _) => {
                h2519
            }
        };
        let a2521 = 1u64;
        nat_sub(a2520, a2521)
    }, {
        let a2523 = match v0.clone() {
            Adt3::C0(_, _, h2522, _, _, _, _, _) => {
                h2522
            }
        };
        let a2524 = 1u64;
        nat_sub(a2523, a2524)
    }, {
        let a2526 = match v0 {
            Adt3::C0(_, _, _, h2525, _, _, _, _) => {
                h2525
            }
        };
        let a2527 = 1u64;
        nat_sub(a2526, a2527)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2528 = v0;
    let a2529 = 0u64;
    if nat_eq(a2528, a2529) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2530 = v0;
            let a2531 = 1u64;
            nat_sub(a2530, a2531)
        };
        let v5: u64 = {
            let a2532 = v4;
            let a2533 = v1;
            let a2534 = 0u64;
            nat_quot(a2532, a2533, a2534)
        };
        let v6: u64 = {
            let a2542 = {
                let a2535 = v5;
                let a2536 = v2;
                nat_mul(a2535, a2536)?
            };
            let a2543 = {
                let a2540 = {
                    let a2537 = v4;
                    let a2538 = v1;
                    let a2539 = 0u64;
                    nat_rem(a2537, a2538, a2539)
                };
                let a2541 = v3;
                nat_mul(a2540, a2541)?
            };
            nat_add(a2542, a2543)?
        };
        let a2544 = v5;
        let a2545 = 0u64;
        if nat_eq(a2544, a2545) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2554 = {
                    let a2548 = {
                        let a2546 = v5;
                        let a2547 = 1u64;
                        nat_sub(a2546, a2547)
                    };
                    let a2549 = v2;
                    nat_mul(a2548, a2549)?
                };
                let a2555 = {
                    let a2552 = {
                        let a2550 = v1;
                        let a2551 = 1u64;
                        nat_sub(a2550, a2551)
                    };
                    let a2553 = v3;
                    nat_mul(a2552, a2553)?
                };
                nat_add(a2554, a2555)?
            };
            let a2556 = v6;
            let a2557 = v7;
            if nat_lt(a2556, a2557) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2558 = v1;
    let a2559 = 0u64;
    if !nat_eq(a2558, a2559) {
        true
    } else {
        let a2561 = match v0 {
            Adt3::C0(_, _, _, _, h2560, _, _, _) => {
                h2560
            }
        };
        let a2562 = 4u64;
        nat_eq(a2561, a2562)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2565 = {
        let a2563 = v0;
        let a2564 = 2u64;
        nat_mul(a2563, a2564)?
    };
    let a2566 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2565, a2566)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2567 = f154(v0, v1);
    match m2567 {
        None => {
            if v5 {
                let m2568 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2568 {
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
                let a2569 = 4u64;
                let a2570 = v28;
                nat_le(a2569, a2570)
            };
            let a2571 = v2;
            let a2572 = 0u64;
            if if if if if nat_eq(a2571, a2572) {
                let a2573 = v4;
                let a2574 = 0u64;
                nat_eq(a2573, a2574)
            } else {
                false
            } {
                let a2575 = v6;
                let a2576 = v14;
                if nat_eq(a2575, a2576) {
                    let a2577 = v7;
                    let a2578 = v15;
                    nat_eq(a2577, a2578)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2579 = v8;
                let a2580 = v16;
                if nat_eq(a2579, a2580) {
                    let a2581 = v9;
                    let a2582 = v17;
                    nat_eq(a2581, a2582)
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
                        let a2583 = v3;
                        let a2584 = 0u64;
                        nat_eq(a2583, a2584)
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
                    let a2589 = {
                        let a2585 = v6;
                        let a2586 = v7;
                        nat_mul(a2585, a2586)?
                    };
                    let a2590 = {
                        let a2587 = v8;
                        let a2588 = v9;
                        nat_mul(a2587, a2588)?
                    };
                    nat_mul(a2589, a2590)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2591 = v0;
    let a2592 = 0u64;
    if nat_eq(a2591, a2592) {
        Adt8::C0(0u64, v1)
    } else {
        let a2593 = v0;
        let a2594 = 1u64;
        if nat_eq(a2593, a2594) {
            Adt8::C1(v1)
        } else {
            let a2595 = v0;
            let a2596 = 2u64;
            if nat_eq(a2595, a2596) {
                Adt8::C2(v1)
            } else {
                let a2597 = v0;
                let a2598 = 3u64;
                if nat_eq(a2597, a2598) {
                    Adt8::C3(v1)
                } else {
                    let a2599 = v0;
                    let a2600 = 13u64;
                    if nat_lt(a2599, a2600) {
                        Adt8::C0({
                            let a2601 = v0;
                            let a2602 = 3u64;
                            nat_sub(a2601, a2602)
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
    let a2603 = v0;
    let a2604 = 1u64;
    if nat_eq(a2603, a2604) {
        Adt9::C0(v1)
    } else {
        let a2605 = v0;
        let a2606 = 3u64;
        if if nat_eq(a2605, a2606) {
            true
        } else {
            let a2607 = v0;
            let a2608 = 6u64;
            nat_eq(a2607, a2608)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2609 = f70(v0);
    match m2609 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2611 = match v1.clone() {
                Adt11::C0(_, h2610, _) => {
                    h2610
                }
            };
            let a2612 = 1u64;
            if if nat_eq(a2611, a2612) {
                let a2614 = match v1.clone() {
                    Adt11::C0(_, _, h2613) => {
                        h2613
                    }
                };
                let a2615 = 1u64;
                if if if nat_eq(a2614, a2615) {
                    true
                } else {
                    let a2617 = match v1.clone() {
                        Adt11::C0(_, _, h2616) => {
                            h2616
                        }
                    };
                    let a2618 = 2u64;
                    nat_eq(a2617, a2618)
                } {
                    true
                } else {
                    let a2620 = match v1.clone() {
                        Adt11::C0(_, _, h2619) => {
                            h2619
                        }
                    };
                    let a2621 = 4u64;
                    nat_eq(a2620, a2621)
                } {
                    true
                } else {
                    let a2623 = match v1.clone() {
                        Adt11::C0(_, _, h2622) => {
                            h2622
                        }
                    };
                    let a2624 = 8u64;
                    nat_eq(a2623, a2624)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2625) => {
                        h2625
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2638 = {
        let a2634 = {
            let a2630 = {
                let a2626 = v0;
                let a2627 = v4;
                nat_mul(a2626, a2627)?
            };
            let a2631 = {
                let a2628 = v1;
                let a2629 = v5;
                nat_mul(a2628, a2629)?
            };
            nat_add(a2630, a2631)?
        };
        let a2635 = {
            let a2632 = v2;
            let a2633 = v6;
            nat_mul(a2632, a2633)?
        };
        nat_add(a2634, a2635)?
    };
    let a2639 = {
        let a2636 = v3;
        let a2637 = v7;
        nat_mul(a2636, a2637)?
    };
    nat_add(a2638, a2639)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2644 = {
        let a2642 = {
            let a2640 = v0;
            let a2641 = v1;
            nat_sub(a2640, a2641)
        };
        let a2643 = 1u64;
        nat_sub(a2642, a2643)
    };
    let a2645 = v2;
    nat_add(a2644, a2645)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2646 = v0;
        let a2647 = v4;
        let a2648 = 0u64;
        nat_rem(a2646, a2647, a2648)
    }, {
        let a2649 = v1;
        let a2650 = v5;
        let a2651 = 0u64;
        nat_rem(a2649, a2650, a2651)
    }, {
        let a2652 = v2;
        let a2653 = v6;
        let a2654 = 0u64;
        nat_rem(a2652, a2653, a2654)
    }, {
        let a2655 = v3;
        let a2656 = v7;
        let a2657 = 0u64;
        nat_rem(a2655, a2656, a2657)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2659 = v0;
        let a2660 = match v1.clone() {
            Adt3::C0(h2658, _, _, _, _, _, _, _) => {
                h2658
            }
        };
        let a2661 = 0u64;
        nat_quot(a2659, a2660, a2661)
    };
    let v4: u64 = {
        let a2663 = v3;
        let a2664 = match v1.clone() {
            Adt3::C0(_, h2662, _, _, _, _, _, _) => {
                h2662
            }
        };
        let a2665 = 0u64;
        nat_quot(a2663, a2664, a2665)
    };
    let a2666 = v2;
    let a2667 = 0u64;
    if nat_eq(a2666, a2667) {
        let a2669 = v0;
        let a2670 = match v1 {
            Adt3::C0(h2668, _, _, _, _, _, _, _) => {
                h2668
            }
        };
        let a2671 = 0u64;
        nat_rem(a2669, a2670, a2671)
    } else {
        let a2672 = v2;
        let a2673 = 1u64;
        if nat_eq(a2672, a2673) {
            let a2675 = v3;
            let a2676 = match v1 {
                Adt3::C0(_, h2674, _, _, _, _, _, _) => {
                    h2674
                }
            };
            let a2677 = 0u64;
            nat_rem(a2675, a2676, a2677)
        } else {
            let a2678 = v2;
            let a2679 = 2u64;
            if nat_eq(a2678, a2679) {
                let a2681 = v4;
                let a2682 = match v1 {
                    Adt3::C0(_, _, h2680, _, _, _, _, _) => {
                        h2680
                    }
                };
                let a2683 = 0u64;
                nat_rem(a2681, a2682, a2683)
            } else {
                let a2685 = v4;
                let a2686 = match v1 {
                    Adt3::C0(_, _, h2684, _, _, _, _, _) => {
                        h2684
                    }
                };
                let a2687 = 0u64;
                nat_quot(a2685, a2686, a2687)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2688, _, _, _) => {
            h2688
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2689, _, _) => {
            h2689
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2690, _) => {
            h2690
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2691) => {
            h2691
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2692 = v1;
    let a2693 = 2147483648u64;
    if nat_lt(a2692, a2693) {
        let a2704 = {
            let a2702 = {
                let a2697 = {
                    let a2694 = v0;
                    let a2695 = v2;
                    let a2696 = 0u64;
                    nat_rem(a2694, a2695, a2696)
                };
                let a2698 = v2;
                nat_add(a2697, a2698)?
            };
            let a2703 = {
                let a2699 = v1;
                let a2700 = v2;
                let a2701 = 0u64;
                nat_rem(a2699, a2700, a2701)
            };
            nat_sub(a2702, a2703)
        };
        let a2705 = v2;
        let a2706 = 0u64;
        Ok(nat_rem(a2704, a2705, a2706))
    } else {
        let a2709 = {
            let a2707 = v0;
            let a2708 = f120(v1);
            nat_add(a2707, a2708)?
        };
        let a2710 = v2;
        let a2711 = 0u64;
        Ok(nat_rem(a2709, a2710, a2711))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2714 = f119(v1);
    let a2715 = {
        let a2712 = v0;
        let a2713 = f120(v1);
        nat_add(a2712, a2713)?
    };
    if nat_le(a2714, a2715) {
        let a2716 = f136(v0, v1)?;
        let a2717 = v2;
        Ok(nat_lt(a2716, a2717))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2720 = {
        let a2718 = v0;
        let a2719 = f120(v1);
        nat_add(a2718, a2719)?
    };
    let a2721 = f119(v1);
    Ok(nat_sub(a2720, a2721))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2722 = v0;
    let a2723 = v1;
    if nat_lt(a2722, a2723) {
        let a2724 = v1;
        let a2725 = v0;
        Ok(nat_sub(a2724, a2725))
    } else {
        let a2728 = v0;
        let a2729 = {
            let a2726 = v1;
            let a2727 = v2;
            nat_add(a2726, a2727)?
        };
        if nat_lt(a2728, a2729) {
            let a2730 = v0;
            let a2731 = v1;
            Ok(nat_sub(a2730, a2731))
        } else {
            let a2738 = {
                let a2734 = 2u64;
                let a2735 = {
                    let a2732 = v2;
                    let a2733 = 1u64;
                    nat_sub(a2732, a2733)
                };
                nat_mul(a2734, a2735)?
            };
            let a2739 = {
                let a2736 = v0;
                let a2737 = v1;
                nat_sub(a2736, a2737)
            };
            Ok(nat_sub(a2738, a2739))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2742 = {
        let a2740 = v2;
        let a2741 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2740, a2741)?
    };
    let a2743 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2742, a2743))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2744 = v0;
    let a2745 = 0u64;
    if nat_eq(a2744, a2745) {
        let a2746 = v2;
        let a2747 = v1;
        nat_le(a2746, a2747)
    } else {
        let a2748 = v0;
        let a2749 = 1u64;
        if nat_eq(a2748, a2749) {
            let a2750 = v2;
            let a2751 = v1;
            nat_lt(a2750, a2751)
        } else {
            let a2752 = v0;
            let a2753 = 2u64;
            if nat_eq(a2752, a2753) {
                let a2754 = v1;
                let a2755 = v2;
                nat_le(a2754, a2755)
            } else {
                let a2756 = v0;
                let a2757 = 3u64;
                if nat_eq(a2756, a2757) {
                    let a2758 = v1;
                    let a2759 = v2;
                    nat_lt(a2758, a2759)
                } else {
                    let a2760 = v1;
                    let a2761 = v2;
                    nat_eq(a2760, a2761)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2762 = v0;
    let a2763 = 0u64;
    if nat_eq(a2762, a2763) {
        Ok(f148(0u64))
    } else {
        let a2766 = {
            let a2764 = v0;
            let a2765 = 1u64;
            nat_add(a2764, a2765)?
        };
        let a2767 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2766, a2767) {
            Ok(Adt5::C4({
                let a2768 = v2;
                let a2769 = 0u64;
                if nat_eq(a2768, a2769) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2770 = v0;
                let a2771 = 1u64;
                nat_sub(a2770, a2771)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2772, _, _, _, _, _, _, _) => {
                        h2772
                    }
                }, false)?;
                let v5: u64 = {
                    let a2773 = v3;
                    let a2774 = v4;
                    let a2775 = 0u64;
                    nat_quot(a2773, a2774, a2775)
                };
                let v6: u64 = {
                    let a2776 = v3;
                    let a2777 = v4;
                    let a2778 = 0u64;
                    nat_rem(a2776, a2777, a2778)
                };
                let a2781 = {
                    let a2779 = v6;
                    let a2780 = 1u64;
                    nat_add(a2779, a2780)?
                };
                let a2782 = v4;
                if nat_eq(a2781, a2782) {
                    Ok(Adt5::C4({
                        let a2783 = v2;
                        let a2784 = 0u64;
                        if nat_eq(a2783, a2784) {
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
                    let a2785 = v3;
                    let a2786 = 2u64;
                    let a2787 = 0u64;
                    nat_quot(a2785, a2786, a2787)
                }, {
                    let a2788 = v3;
                    let a2789 = 2u64;
                    let a2790 = 0u64;
                    nat_rem(a2788, a2789, a2790)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2791 = v0;
    let a2792 = 0u64;
    if nat_eq(a2791, a2792) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2801 = {
                let a2794 = v1;
                let a2795 = match v2.clone() {
                    Adt3::C0(h2793, _, _, _, _, _, _, _) => {
                        h2793
                    }
                };
                nat_mul(a2794, a2795)?
            };
            let a2802 = {
                let a2798 = {
                    let a2796 = v0;
                    let a2797 = 1u64;
                    nat_sub(a2796, a2797)
                };
                let a2799 = 2u64;
                let a2800 = 0u64;
                nat_quot(a2798, a2799, a2800)
            };
            nat_add(a2801, a2802)?
        }, {
            let a2805 = {
                let a2803 = v0;
                let a2804 = 1u64;
                nat_sub(a2803, a2804)
            };
            let a2806 = 2u64;
            let a2807 = 0u64;
            nat_rem(a2805, a2806, a2807)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2808 = v0;
    let a2809 = f60(v1)?;
    let a2810 = 0u64;
    Ok(nat_rem(a2808, a2809, a2810))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2811 = 4u64;
    let a2812 = f157(v0, v1)?;
    nat_mul(a2811, a2812)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2813 = 4u64;
    let a2814 = f158(v0, v1)?;
    nat_mul(a2813, a2814)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2820 = {
        let a2817 = {
            let a2815 = f142(v0, v1)?;
            let a2816 = 2u64;
            nat_sub(a2815, a2816)
        };
        let a2818 = 2u64;
        let a2819 = 0u64;
        nat_rem(a2817, a2818, a2819)
    };
    let a2821 = 0u64;
    Ok(nat_eq(a2820, a2821))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2822 = f70(v0);
    match m2822 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2823 = v1;
                let a2824 = v7;
                nat_lt(a2823, a2824)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2825 = v3;
                let a2826 = f160(v4, v5, v6)?;
                nat_lt(a2825, a2826)
            } else {
                false
            } {
                let a2831 = {
                    let a2828 = v7;
                    let a2829 = match v13.clone() {
                        Adt11::C0(_, h2827, _) => {
                            h2827
                        }
                    };
                    let a2830 = 0u64;
                    nat_rem(a2828, a2829, a2830)
                };
                let a2832 = 0u64;
                nat_eq(a2831, a2832)
            } else {
                false
            } {
                Ok(Some({
                    let a2861 = {
                        let a2851 = {
                            let a2841 = {
                                let a2837 = {
                                    let a2834 = v1;
                                    let a2835 = match v13 {
                                        Adt11::C0(_, h2833, _) => {
                                            h2833
                                        }
                                    };
                                    let a2836 = 0u64;
                                    nat_quot(a2834, a2835, a2836)
                                };
                                let a2838 = v9;
                                nat_mul(a2837, a2838)?
                            };
                            let a2842 = {
                                let a2839 = v2;
                                let a2840 = v10;
                                nat_mul(a2839, a2840)?
                            };
                            nat_add(a2841, a2842)?
                        };
                        let a2852 = {
                            let a2849 = {
                                let a2846 = {
                                    let a2843 = v3;
                                    let a2844 = v4;
                                    let a2845 = 0u64;
                                    nat_quot(a2843, a2844, a2845)
                                };
                                let a2847 = v5;
                                let a2848 = 0u64;
                                nat_rem(a2846, a2847, a2848)
                            };
                            let a2850 = v11;
                            nat_mul(a2849, a2850)?
                        };
                        nat_add(a2851, a2852)?
                    };
                    let a2862 = {
                        let a2859 = {
                            let a2856 = {
                                let a2853 = v3;
                                let a2854 = v4;
                                let a2855 = 0u64;
                                nat_quot(a2853, a2854, a2855)
                            };
                            let a2857 = v5;
                            let a2858 = 0u64;
                            nat_quot(a2856, a2857, a2858)
                        };
                        let a2860 = v12;
                        nat_mul(a2859, a2860)?
                    };
                    nat_add(a2861, a2862)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2863 = v0;
    let a2864 = 30u64;
    if nat_eq(a2863, a2864) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2865 = 0u64;
    let a2866 = v0;
    if nat_lt(a2865, a2866) {
        let a2867 = v0;
        let a2868 = 2139095040u64;
        nat_le(a2867, a2868)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2869 = v0;
    let a2870 = 40u64;
    if nat_lt(a2869, a2870) {
        let a2871 = v0;
        let a2872 = 20u64;
        if nat_lt(a2871, a2872) {
            let a2873 = v0;
            let a2874 = 10u64;
            if nat_lt(a2873, a2874) {
                let a2875 = v0;
                let a2876 = 5u64;
                if nat_lt(a2875, a2876) {
                    let a2877 = v0;
                    let a2878 = 2u64;
                    if nat_lt(a2877, a2878) {
                        let a2879 = v0;
                        let a2880 = 1u64;
                        if nat_lt(a2879, a2880) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2881 = v0;
                        let a2882 = 3u64;
                        if nat_lt(a2881, a2882) {
                            4u64
                        } else {
                            let a2883 = v0;
                            let a2884 = 4u64;
                            if nat_lt(a2883, a2884) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2885 = v0;
                    let a2886 = 7u64;
                    if nat_lt(a2885, a2886) {
                        let a2887 = v0;
                        let a2888 = 6u64;
                        if nat_lt(a2887, a2888) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2889 = v0;
                        let a2890 = 8u64;
                        if nat_lt(a2889, a2890) {
                            128u64
                        } else {
                            let a2891 = v0;
                            let a2892 = 9u64;
                            if nat_lt(a2891, a2892) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2893 = v0;
                let a2894 = 15u64;
                if nat_lt(a2893, a2894) {
                    let a2895 = v0;
                    let a2896 = 12u64;
                    if nat_lt(a2895, a2896) {
                        let a2897 = v0;
                        let a2898 = 11u64;
                        if nat_lt(a2897, a2898) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2899 = v0;
                        let a2900 = 13u64;
                        if nat_lt(a2899, a2900) {
                            4096u64
                        } else {
                            let a2901 = v0;
                            let a2902 = 14u64;
                            if nat_lt(a2901, a2902) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2903 = v0;
                    let a2904 = 17u64;
                    if nat_lt(a2903, a2904) {
                        let a2905 = v0;
                        let a2906 = 16u64;
                        if nat_lt(a2905, a2906) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2907 = v0;
                        let a2908 = 18u64;
                        if nat_lt(a2907, a2908) {
                            131072u64
                        } else {
                            let a2909 = v0;
                            let a2910 = 19u64;
                            if nat_lt(a2909, a2910) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2911 = v0;
            let a2912 = 30u64;
            if nat_lt(a2911, a2912) {
                let a2913 = v0;
                let a2914 = 25u64;
                if nat_lt(a2913, a2914) {
                    let a2915 = v0;
                    let a2916 = 22u64;
                    if nat_lt(a2915, a2916) {
                        let a2917 = v0;
                        let a2918 = 21u64;
                        if nat_lt(a2917, a2918) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2919 = v0;
                        let a2920 = 23u64;
                        if nat_lt(a2919, a2920) {
                            4194304u64
                        } else {
                            let a2921 = v0;
                            let a2922 = 24u64;
                            if nat_lt(a2921, a2922) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2923 = v0;
                    let a2924 = 27u64;
                    if nat_lt(a2923, a2924) {
                        let a2925 = v0;
                        let a2926 = 26u64;
                        if nat_lt(a2925, a2926) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2927 = v0;
                        let a2928 = 28u64;
                        if nat_lt(a2927, a2928) {
                            134217728u64
                        } else {
                            let a2929 = v0;
                            let a2930 = 29u64;
                            if nat_lt(a2929, a2930) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2931 = v0;
                let a2932 = 35u64;
                if nat_lt(a2931, a2932) {
                    let a2933 = v0;
                    let a2934 = 32u64;
                    if nat_lt(a2933, a2934) {
                        let a2935 = v0;
                        let a2936 = 31u64;
                        if nat_lt(a2935, a2936) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2937 = v0;
                        let a2938 = 33u64;
                        if nat_lt(a2937, a2938) {
                            4294967296u64
                        } else {
                            let a2939 = v0;
                            let a2940 = 34u64;
                            if nat_lt(a2939, a2940) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2941 = v0;
                    let a2942 = 37u64;
                    if nat_lt(a2941, a2942) {
                        let a2943 = v0;
                        let a2944 = 36u64;
                        if nat_lt(a2943, a2944) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2945 = v0;
                        let a2946 = 38u64;
                        if nat_lt(a2945, a2946) {
                            137438953472u64
                        } else {
                            let a2947 = v0;
                            let a2948 = 39u64;
                            if nat_lt(a2947, a2948) {
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
    let a2949 = v0;
    let a2950 = 26u64;
    if nat_eq(a2949, a2950) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2971 = {
        let a2965 = {
            let a2959 = {
                let a2953 = v1;
                let a2954 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2951, _, _, _) => {
                            h2951
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2952, _, _, _) => {
                            h2952
                        }
                    })
                };
                nat_mul(a2953, a2954)?
            };
            let a2960 = {
                let a2957 = v2;
                let a2958 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2955, _, _) => {
                            h2955
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2956, _, _) => {
                            h2956
                        }
                    })
                };
                nat_mul(a2957, a2958)?
            };
            nat_add(a2959, a2960)?
        };
        let a2966 = {
            let a2963 = v3;
            let a2964 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2961, _) => {
                        h2961
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2962, _) => {
                        h2962
                    }
                })
            };
            nat_mul(a2963, a2964)?
        };
        nat_add(a2965, a2966)?
    };
    let a2972 = {
        let a2969 = v4;
        let a2970 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2967) => {
                    h2967
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2968) => {
                    h2968
                }
            })
        };
        nat_mul(a2969, a2970)?
    };
    nat_add(a2971, a2972)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a2973 = v0;
    let a2974 = 2u64;
    if nat_eq(a2973, a2974) {
        Some(0u64)
    } else {
        let a2975 = v0;
        let a2976 = 5u64;
        if nat_eq(a2975, a2976) {
            Some(1u64)
        } else {
            let a2977 = v0;
            let a2978 = 6u64;
            if nat_eq(a2977, a2978) {
                Some(2u64)
            } else {
                let a2979 = v0;
                let a2980 = 7u64;
                if nat_eq(a2979, a2980) {
                    Some(3u64)
                } else {
                    let a2981 = v0;
                    let a2982 = 9u64;
                    if nat_eq(a2981, a2982) {
                        Some(4u64)
                    } else {
                        let a2983 = v0;
                        let a2984 = 8u64;
                        if nat_eq(a2983, a2984) {
                            Some(5u64)
                        } else {
                            let a2985 = v0;
                            let a2986 = 89u64;
                            if nat_eq(a2985, a2986) {
                                let a2987 = v1;
                                let a2988 = 2u64;
                                if nat_eq(a2987, a2988) {
                                    Some(6u64)
                                } else {
                                    let a2989 = v1;
                                    let a2990 = 0u64;
                                    if nat_eq(a2989, a2990) {
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
    let a2991 = v0;
    let a2992 = 28u64;
    if if if if nat_eq(a2991, a2992) {
        let a2993 = v1;
        let a2994 = 0u64;
        nat_eq(a2993, a2994)
    } else {
        false
    } {
        let a2995 = v2;
        let a2996 = 0u64;
        nat_eq(a2995, a2996)
    } else {
        false
    } {
        let a2997 = v3;
        let a2998 = 0u64;
        nat_eq(a2997, a2998)
    } else {
        false
    } {
        let a2999 = v4;
        let a3000 = v8;
        if if if if if if if if nat_eq(a2999, a3000) {
            let a3001 = v5;
            let a3002 = v12;
            nat_eq(a3001, a3002)
        } else {
            false
        } {
            let a3003 = v9;
            let a3004 = v13;
            nat_eq(a3003, a3004)
        } else {
            false
        } {
            let a3005 = v10;
            let a3006 = v14;
            nat_eq(a3005, a3006)
        } else {
            false
        } {
            let a3007 = v11;
            let a3008 = v15;
            nat_eq(a3007, a3008)
        } else {
            false
        } {
            let a3009 = 0u64;
            let a3010 = v6;
            nat_lt(a3009, a3010)
        } else {
            false
        } {
            let a3011 = 0u64;
            let a3012 = v7;
            nat_lt(a3011, a3012)
        } else {
            false
        } {
            let a3016 = {
                let a3013 = v10;
                let a3014 = v6;
                let a3015 = 1u64;
                nat_rem(a3013, a3014, a3015)
            };
            let a3017 = 0u64;
            if nat_eq(a3016, a3017) {
                let a3021 = {
                    let a3018 = v11;
                    let a3019 = v7;
                    let a3020 = 1u64;
                    nat_rem(a3018, a3019, a3020)
                };
                let a3022 = 0u64;
                nat_eq(a3021, a3022)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3023 = v10;
                let a3024 = v6;
                let a3025 = 0u64;
                nat_quot(a3023, a3024, a3025)
            }, {
                let a3026 = v11;
                let a3027 = v7;
                let a3028 = 0u64;
                nat_quot(a3026, a3027, a3028)
            }, {
                let a3029 = v5;
                let a3030 = v4;
                nat_mul(a3029, a3030)?
            }, {
                let a3031 = v4;
                let a3032 = v9;
                nat_mul(a3031, a3032)?
            }, {
                let a3033 = v5;
                let a3034 = v9;
                nat_mul(a3033, a3034)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3035 = v1;
    let a3036 = 0u64;
    if nat_eq(a3035, a3036) {
        Ok(Adt5::C4({
            let a3037 = v3;
            let a3038 = 0u64;
            if nat_eq(a3037, a3038) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3039 = v3;
            let a3040 = 0u64;
            if nat_eq(a3039, a3040) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3041 = v0;
    let a3042 = f60(v1)?;
    let a3043 = 0u64;
    Ok(nat_quot(a3041, a3042, a3043))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3046 = {
            let a3044 = f142(v0, v1)?;
            let a3045 = 2u64;
            nat_sub(a3044, a3045)
        };
        let a3047 = 2u64;
        let a3048 = 0u64;
        nat_quot(a3046, a3047, a3048)
    };
    let v4: u64 = {
        let a3049 = v2;
        let a3050 = v1;
        nat_mul(a3049, a3050)?
    };
    if f145(v0, v1)? {
        let a3051 = v4;
        let a3052 = v3;
        nat_add(a3051, a3052)
    } else {
        let a3064 = {
            let a3060 = v4;
            let a3061 = {
                let a3058 = {
                    let a3053 = v1;
                    let a3054 = 1u64;
                    nat_sub(a3053, a3054)
                };
                let a3059 = {
                    let a3055 = v2;
                    let a3056 = v1;
                    let a3057 = 0u64;
                    nat_rem(a3055, a3056, a3057)
                };
                nat_mul(a3058, a3059)?
            };
            nat_sub(a3060, a3061)
        };
        let a3065 = {
            let a3062 = v3;
            let a3063 = v1;
            nat_mul(a3062, a3063)?
        };
        nat_add(a3064, a3065)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3066 = v0;
    let a3067 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3066, a3067) {
        true
    } else {
        let a3068 = v0;
        let a3069 = 1u64;
        nat_eq(a3068, a3069)
    } {
        true
    } else {
        let a3070 = v0;
        let a3071 = 2u64;
        nat_eq(a3070, a3071)
    } {
        true
    } else {
        let a3072 = v0;
        let a3073 = 3u64;
        nat_eq(a3072, a3073)
    } {
        true
    } else {
        let a3074 = v0;
        let a3075 = 6u64;
        nat_eq(a3074, a3075)
    } {
        true
    } else {
        let a3076 = v0;
        let a3077 = 7u64;
        nat_eq(a3076, a3077)
    } {
        true
    } else {
        let a3078 = v0;
        let a3079 = 8u64;
        nat_eq(a3078, a3079)
    } {
        true
    } else {
        let a3080 = v0;
        let a3081 = 10u64;
        nat_eq(a3080, a3081)
    } {
        true
    } else {
        let a3082 = v0;
        let a3083 = 11u64;
        nat_eq(a3082, a3083)
    } {
        true
    } else {
        let a3084 = v0;
        let a3085 = 12u64;
        nat_eq(a3084, a3085)
    } {
        true
    } else {
        let a3086 = v0;
        let a3087 = 13u64;
        nat_eq(a3086, a3087)
    } {
        true
    } else {
        let a3088 = v0;
        let a3089 = 14u64;
        nat_eq(a3088, a3089)
    } {
        true
    } else {
        let a3090 = v0;
        let a3091 = 16u64;
        nat_eq(a3090, a3091)
    } {
        true
    } else {
        let a3092 = v0;
        let a3093 = 17u64;
        nat_eq(a3092, a3093)
    } {
        true
    } else {
        let a3094 = v0;
        let a3095 = 18u64;
        nat_eq(a3094, a3095)
    } {
        true
    } else {
        let a3096 = v0;
        let a3097 = 19u64;
        nat_eq(a3096, a3097)
    } {
        true
    } else {
        let a3098 = v0;
        let a3099 = 20u64;
        nat_eq(a3098, a3099)
    } {
        true
    } else {
        let a3100 = v0;
        let a3101 = 21u64;
        nat_eq(a3100, a3101)
    } {
        true
    } else {
        let a3102 = v0;
        let a3103 = 22u64;
        nat_eq(a3102, a3103)
    } {
        true
    } else {
        let a3104 = v0;
        let a3105 = 23u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 26u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 29u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 30u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 34u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 35u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 39u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 40u64;
        nat_eq(a3118, a3119)
    } {
        true
    } else {
        let a3120 = v0;
        let a3121 = 41u64;
        nat_eq(a3120, a3121)
    } {
        true
    } else {
        let a3122 = v0;
        let a3123 = 42u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v0;
        let a3125 = 43u64;
        nat_eq(a3124, a3125)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3126 = v0;
    let a3127 = 0u64;
    if if if nat_eq(a3126, a3127) {
        true
    } else {
        let a3128 = v1;
        let a3129 = 0u64;
        nat_eq(a3128, a3129)
    } {
        true
    } else {
        let a3130 = v2;
        let a3131 = 0u64;
        nat_eq(a3130, a3131)
    } {
        Ok(0u64)
    } else {
        let a3134 = {
            let a3132 = v0;
            let a3133 = v1;
            nat_mul(a3132, a3133)?
        };
        let a3135 = v2;
        nat_mul(a3134, a3135)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3136 = f70(v0);
    match m3136 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
