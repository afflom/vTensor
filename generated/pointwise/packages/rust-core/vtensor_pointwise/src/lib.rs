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
            let a1209 = 13u64;
            nat_eq(a1208, a1209)
        } else {
            false
        } {
            Ok(Some(16u64))
        } else {
            let a1210 = v0;
            let a1211 = 8u64;
            if nat_eq(a1210, a1211) {
                Ok(Some(5u64))
            } else {
                let a1212 = v0;
                let a1213 = 9u64;
                if nat_eq(a1212, a1213) {
                    Ok(Some(6u64))
                } else {
                    let a1214 = v0;
                    let a1215 = 89u64;
                    if nat_eq(a1214, a1215) {
                        let a1216 = v1;
                        let a1217 = 0u64;
                        if nat_eq(a1216, a1217) {
                            Ok(Some(0u64))
                        } else {
                            let a1218 = v1;
                            let a1219 = 1u64;
                            if nat_eq(a1218, a1219) {
                                Ok(Some(1u64))
                            } else {
                                let a1220 = v1;
                                let a1221 = 2u64;
                                if nat_eq(a1220, a1221) {
                                    Ok(Some(2u64))
                                } else {
                                    let a1222 = v1;
                                    let a1223 = 3u64;
                                    if nat_eq(a1222, a1223) {
                                        Ok(Some(3u64))
                                    } else {
                                        let a1224 = v1;
                                        let a1225 = 6u64;
                                        if nat_eq(a1224, a1225) {
                                            Ok(Some(4u64))
                                        } else {
                                            let a1226 = v1;
                                            let a1227 = 12u64;
                                            if nat_eq(a1226, a1227) {
                                                Ok(Some(7u64))
                                            } else {
                                                let a1228 = v1;
                                                let a1229 = 11u64;
                                                if nat_eq(a1228, a1229) {
                                                    Ok(Some(8u64))
                                                } else {
                                                    let a1230 = v1;
                                                    let a1231 = 18u64;
                                                    if nat_eq(a1230, a1231) {
                                                        Ok(Some(9u64))
                                                    } else {
                                                        let a1232 = v1;
                                                        let a1233 = 19u64;
                                                        if nat_eq(a1232, a1233) {
                                                            Ok(Some(10u64))
                                                        } else {
                                                            let a1234 = v1;
                                                            let a1235 = 20u64;
                                                            if nat_eq(a1234, a1235) {
                                                                Ok(Some(11u64))
                                                            } else {
                                                                let a1236 = v1;
                                                                let a1237 = 21u64;
                                                                if nat_eq(a1236, a1237) {
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

pub fn f49(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1238 = f83(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1238 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            let a1239 = v0;
            let a1240 = 2u64;
            if if if if nat_eq(a1239, a1240) {
                true
            } else {
                let a1241 = v0;
                let a1242 = 5u64;
                nat_eq(a1241, a1242)
            } {
                true
            } else {
                let a1243 = v0;
                let a1244 = 6u64;
                nat_eq(a1243, a1244)
            } {
                true
            } else {
                let a1245 = v0;
                let a1246 = 7u64;
                nat_eq(a1245, a1246)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1247 = v3;
                    match m1247 {
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
    let a1249 = match v1.clone() {
        Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
            h1248
        }
    };
    let a1250 = 0u64;
    if if if if if nat_eq(a1249, a1250) {
        true
    } else {
        let a1252 = match v1.clone() {
            Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                h1251
            }
        };
        let a1253 = 1u64;
        nat_eq(a1252, a1253)
    } {
        true
    } else {
        let a1255 = match v1.clone() {
            Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                h1254
            }
        };
        let a1256 = 30u64;
        nat_eq(a1255, a1256)
    } {
        let a1259 = match v1.clone() {
            Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
                h1257
            }
        };
        let a1260 = match v2.clone() {
            Adt2::C0(h1258, _, _, _, _, _, _, _, _, _, _) => {
                h1258
            }
        };
        if nat_eq(a1259, a1260) {
            true
        } else {
            let a1262 = match v2.clone() {
                Adt2::C0(h1261, _, _, _, _, _, _, _, _, _, _) => {
                    h1261
                }
            };
            let a1263 = 0u64;
            nat_eq(a1262, a1263)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1265 = match v2.clone() {
                Adt2::C0(h1264, _, _, _, _, _, _, _, _, _, _) => {
                    h1264
                }
            };
            let a1266 = 0u64;
            if nat_eq(a1265, a1266) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1267, _, _, _, _, _, _, _, _, _, _) => {
                h1267
            }
        }, match v2 {
            Adt2::C0(h1268, _, _, _, _, _, _, _, _, _, _) => {
                h1268
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1269 = v1;
    match m1269 {
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
    let m1270 = v3;
    match m1270 {
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
                Adt3::C0(h1271, _, _, _, _, _, _, _) => {
                    h1271
                }
            }, match v2.clone() {
                Adt3::C0(_, h1272, _, _, _, _, _, _) => {
                    h1272
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
            let a1273 = v51;
            let a1274 = 64u64;
            if if if nat_eq(a1273, a1274) {
                true
            } else {
                let a1275 = v51;
                let a1276 = 65u64;
                nat_eq(a1275, a1276)
            } {
                true
            } else {
                let a1277 = v51;
                let a1278 = 66u64;
                nat_eq(a1277, a1278)
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
        Adt3::C0(h1279, _, _, _, _, _, _, _) => {
            h1279
        }
    })?;
    let v5: u64 = {
        let a1280 = v0;
        let a1281 = v4;
        let a1282 = 0u64;
        nat_quot(a1280, a1281, a1282)
    };
    let v6: u64 = {
        let a1283 = v0;
        let a1284 = v4;
        let a1285 = 0u64;
        nat_rem(a1283, a1284, a1285)
    };
    let a1286 = v6;
    let a1287 = 0u64;
    if nat_eq(a1286, a1287) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1288 = v6;
        let a1289 = 1u64;
        if nat_eq(a1288, a1289) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1292 = {
                let a1290 = v6;
                let a1291 = 1u64;
                nat_add(a1290, a1291)?
            };
            let a1293 = v4;
            if nat_eq(a1292, a1293) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1294 = v6;
                    let a1295 = 2u64;
                    nat_sub(a1294, a1295)
                };
                let v8: u64 = {
                    let a1296 = v7;
                    let a1297 = 3u64;
                    let a1298 = 0u64;
                    nat_quot(a1296, a1297, a1298)
                };
                let v9: u64 = {
                    let a1299 = v7;
                    let a1300 = 3u64;
                    let a1301 = 0u64;
                    nat_rem(a1299, a1300, a1301)
                };
                let v10: u64 = f89({
                    let a1305 = {
                        let a1303 = v5;
                        let a1304 = match v1.clone() {
                            Adt3::C0(h1302, _, _, _, _, _, _, _) => {
                                h1302
                            }
                        };
                        nat_mul(a1303, a1304)?
                    };
                    let a1306 = v8;
                    nat_add(a1305, a1306)?
                }, v1)?;
                let a1307 = v9;
                let a1308 = 0u64;
                if nat_eq(a1307, a1308) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1309 = v9;
                    let a1310 = 1u64;
                    if nat_eq(a1309, a1310) {
                        let a1311 = v3;
                        let a1312 = 2u64;
                        if nat_eq(a1311, a1312) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1313 = v3;
                        let a1314 = 2u64;
                        if if nat_eq(a1313, a1314) {
                            true
                        } else {
                            let a1315 = v3;
                            let a1316 = 3u64;
                            nat_eq(a1315, a1316)
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
        let a1318 = 2u64;
        let a1319 = match v1.clone() {
            Adt3::C0(_, h1317, _, _, _, _, _, _) => {
                h1317
            }
        };
        nat_mul(a1318, a1319)?
    };
    let a1320 = v0;
    let a1321 = v5;
    if nat_lt(a1320, a1321) {
        let a1325 = {
            let a1322 = v0;
            let a1323 = 2u64;
            let a1324 = 0u64;
            nat_rem(a1322, a1323, a1324)
        };
        let a1326 = 0u64;
        if nat_eq(a1325, a1326) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1327 = v0;
                let a1328 = 2u64;
                let a1329 = 0u64;
                nat_quot(a1327, a1328, a1329)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1330 = v4;
            let a1331 = 2147483648u64;
            if if nat_lt(a1330, a1331) {
                let a1333 = v4;
                let a1334 = match v2 {
                    Adt3::C0(_, h1332, _, _, _, _, _, _) => {
                        h1332
                    }
                };
                nat_lt(a1333, a1334)
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
            let a1337 = match v2.clone() {
                Adt3::C0(h1335, _, _, _, _, _, _, _) => {
                    h1335
                }
            };
            let a1338 = match v2.clone() {
                Adt3::C0(_, h1336, _, _, _, _, _, _) => {
                    h1336
                }
            };
            nat_mul(a1337, a1338)?
        };
        let a1341 = {
            let a1339 = v0;
            let a1340 = v5;
            nat_sub(a1339, a1340)
        };
        let a1342 = v6;
        if nat_lt(a1341, a1342) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1343 = v0;
                let a1344 = v5;
                nat_sub(a1343, a1344)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1347 = {
                    let a1345 = v0;
                    let a1346 = v5;
                    nat_sub(a1345, a1346)
                };
                let a1348 = v6;
                nat_sub(a1347, a1348)
            };
            let v8: u64 = {
                let a1352 = 1u64;
                let a1353 = {
                    let a1350 = 2u64;
                    let a1351 = match v1.clone() {
                        Adt3::C0(h1349, _, _, _, _, _, _, _) => {
                            h1349
                        }
                    };
                    nat_mul(a1350, a1351)?
                };
                nat_add(a1352, a1353)?
            };
            let v9: u64 = {
                let a1354 = v7;
                let a1355 = v8;
                let a1356 = 0u64;
                nat_quot(a1354, a1355, a1356)
            };
            let v10: u64 = {
                let a1357 = v7;
                let a1358 = v8;
                let a1359 = 0u64;
                nat_rem(a1357, a1358, a1359)
            };
            let a1360 = v10;
            let a1361 = 0u64;
            if nat_eq(a1360, a1361) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1365 = {
                    let a1362 = v10;
                    let a1363 = 2u64;
                    let a1364 = 0u64;
                    nat_rem(a1362, a1363, a1364)
                };
                let a1366 = 1u64;
                if nat_eq(a1365, a1366) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1375 = {
                            let a1368 = v9;
                            let a1369 = match v1.clone() {
                                Adt3::C0(h1367, _, _, _, _, _, _, _) => {
                                    h1367
                                }
                            };
                            nat_mul(a1368, a1369)?
                        };
                        let a1376 = {
                            let a1372 = {
                                let a1370 = v10;
                                let a1371 = 1u64;
                                nat_sub(a1370, a1371)
                            };
                            let a1373 = 2u64;
                            let a1374 = 0u64;
                            nat_quot(a1372, a1373, a1374)
                        };
                        nat_add(a1375, a1376)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1385 = {
                            let a1378 = v4;
                            let a1379 = match v1.clone() {
                                Adt3::C0(h1377, _, _, _, _, _, _, _) => {
                                    h1377
                                }
                            };
                            nat_mul(a1378, a1379)?
                        };
                        let a1386 = {
                            let a1382 = {
                                let a1380 = v10;
                                let a1381 = 1u64;
                                nat_sub(a1380, a1381)
                            };
                            let a1383 = 2u64;
                            let a1384 = 0u64;
                            nat_quot(a1382, a1383, a1384)
                        };
                        nat_add(a1385, a1386)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1395 = {
                            let a1388 = v4;
                            let a1389 = match v1 {
                                Adt3::C0(h1387, _, _, _, _, _, _, _) => {
                                    h1387
                                }
                            };
                            nat_mul(a1388, a1389)?
                        };
                        let a1396 = {
                            let a1392 = {
                                let a1390 = v10;
                                let a1391 = 1u64;
                                nat_sub(a1390, a1391)
                            };
                            let a1393 = 2u64;
                            let a1394 = 0u64;
                            nat_quot(a1392, a1393, a1394)
                        };
                        nat_add(a1395, a1396)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1397 = v1;
    let a1398 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1397, a1398) {
        let a1401 = v1;
        let a1402 = {
            let a1399 = 2u64;
            let a1400 = f81(v4.clone())?;
            nat_mul(a1399, a1400)?
        };
        if nat_lt(a1401, a1402) {
            let a1406 = {
                let a1403 = v1;
                let a1404 = 2u64;
                let a1405 = 0u64;
                nat_rem(a1403, a1404, a1405)
            };
            let a1407 = 0u64;
            if nat_eq(a1406, a1407) {
                f105({
                    let a1408 = v1;
                    let a1409 = 2u64;
                    let a1410 = 0u64;
                    nat_quot(a1408, a1409, a1410)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1411, _, _, _, _, _, _) => {
                        h1411
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1414 = v1;
                let a1415 = {
                    let a1412 = 2u64;
                    let a1413 = f81(v4.clone())?;
                    nat_mul(a1412, a1413)?
                };
                nat_sub(a1414, a1415)
            };
            let v7: u64 = {
                let a1419 = v6;
                let a1420 = {
                    let a1417 = 1u64;
                    let a1418 = match v2.clone() {
                        Adt3::C0(h1416, _, _, _, _, _, _, _) => {
                            h1416
                        }
                    };
                    nat_add(a1417, a1418)?
                };
                let a1421 = 0u64;
                nat_quot(a1419, a1420, a1421)
            };
            let v8: u64 = {
                let a1425 = v6;
                let a1426 = {
                    let a1423 = 1u64;
                    let a1424 = match v2.clone() {
                        Adt3::C0(h1422, _, _, _, _, _, _, _) => {
                            h1422
                        }
                    };
                    nat_add(a1423, a1424)?
                };
                let a1427 = 0u64;
                nat_rem(a1425, a1426, a1427)
            };
            let a1428 = v8;
            let a1429 = 0u64;
            if nat_eq(a1428, a1429) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1430 = v8;
                    let a1431 = 1u64;
                    nat_sub(a1430, a1431)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1434 = match v4.clone() {
            Adt3::C0(h1432, _, _, _, _, _, _, _) => {
                h1432
            }
        };
        let a1435 = match v4.clone() {
            Adt3::C0(_, h1433, _, _, _, _, _, _) => {
                h1433
            }
        };
        nat_mul(a1434, a1435)?
    };
    let v7: u64 = {
        let a1436 = 2u64;
        let a1437 = v6;
        nat_mul(a1436, a1437)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1438, _, _, _, _, _, _) => {
            h1438
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1439, _, _, _, _, _) => {
            h1439
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1440, _, _, _, _) => {
            h1440
        }
    });
    let a1441 = v0;
    let a1442 = v7;
    if nat_lt(a1441, a1442) {
        let a1446 = {
            let a1443 = v0;
            let a1444 = 2u64;
            let a1445 = 0u64;
            nat_rem(a1443, a1444, a1445)
        };
        let a1447 = 0u64;
        if nat_eq(a1446, a1447) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1448 = v0;
                let a1449 = 2u64;
                let a1450 = 0u64;
                nat_quot(a1448, a1449, a1450)
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
            let a1451 = v0;
            let a1452 = v7;
            nat_sub(a1451, a1452)
        };
        let v10: u64 = {
            let a1456 = v9;
            let a1457 = {
                let a1454 = 1u64;
                let a1455 = match v1.clone() {
                    Adt3::C0(h1453, _, _, _, _, _, _, _) => {
                        h1453
                    }
                };
                nat_add(a1454, a1455)?
            };
            let a1458 = 0u64;
            nat_quot(a1456, a1457, a1458)
        };
        let v11: u64 = {
            let a1462 = v9;
            let a1463 = {
                let a1460 = 1u64;
                let a1461 = match v1.clone() {
                    Adt3::C0(h1459, _, _, _, _, _, _, _) => {
                        h1459
                    }
                };
                nat_add(a1460, a1461)?
            };
            let a1464 = 0u64;
            nat_rem(a1462, a1463, a1464)
        };
        let a1465 = v11;
        let a1466 = 0u64;
        if nat_eq(a1465, a1466) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1467 = v10;
                let a1468 = v6;
                let a1469 = 0u64;
                nat_rem(a1467, a1468, a1469)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1475 = {
                        let a1471 = v10;
                        let a1472 = match v1.clone() {
                            Adt3::C0(h1470, _, _, _, _, _, _, _) => {
                                h1470
                            }
                        };
                        nat_mul(a1471, a1472)?
                    };
                    let a1476 = {
                        let a1473 = v11;
                        let a1474 = 1u64;
                        nat_sub(a1473, a1474)
                    };
                    nat_add(a1475, a1476)?
                };
                let a1477 = v5;
                let a1478 = 4294967295u64;
                if nat_eq(a1477, a1478) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1484 = {
                            let a1480 = v5;
                            let a1481 = match v3.clone() {
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
        let a1488 = v6;
        let a1489 = {
            let a1486 = 3u64;
            let a1487 = v5;
            nat_add(a1486, a1487)?
        };
        nat_sub(a1488, a1489)
    };
    let a1491 = v0;
    let a1492 = f25(v3, match v1.clone() {
        Adt3::C0(h1490, _, _, _, _, _, _, _) => {
            h1490
        }
    }, v5, v8)?;
    if nat_lt(a1491, a1492) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1493, _, _, _, _, _, _, _) => {
                h1493
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1494, _, _, _, _, _, _, _) => {
                h1494
            }
        }, v5)?;
        let a1495 = v0;
        let a1496 = v9;
        if nat_lt(a1495, a1496) {
            f110(v0, v2, f19(v3), {
                let a1497 = 3u64;
                let a1498 = v5;
                nat_add(a1497, a1498)?
            }, v7)
        } else {
            let a1501 = v0;
            let a1502 = {
                let a1499 = v9;
                let a1500 = v10;
                nat_add(a1499, a1500)?
            };
            if nat_lt(a1501, a1502) {
                let v11: u64 = {
                    let a1503 = v0;
                    let a1504 = v9;
                    nat_sub(a1503, a1504)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1505, _, _, _, _, _, _, _) => {
                        h1505
                    }
                })?;
                f112({
                    let a1506 = v11;
                    let a1507 = v12;
                    let a1508 = 0u64;
                    nat_rem(a1506, a1507, a1508)
                }, {
                    let a1509 = v11;
                    let a1510 = v12;
                    let a1511 = 0u64;
                    nat_quot(a1509, a1510, a1511)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1514 = v0;
                    let a1515 = {
                        let a1512 = v9;
                        let a1513 = v10;
                        nat_add(a1512, a1513)?
                    };
                    nat_sub(a1514, a1515)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1516 = v0;
    let a1517 = f12(v1)?;
    if nat_lt(a1516, a1517) {
        let a1518 = v0;
        let a1519 = 4u64;
        if nat_lt(a1518, a1519) {
            let a1520 = v0;
            let a1521 = 0u64;
            if nat_eq(a1520, a1521) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1522 = v0;
                let a1523 = 1u64;
                if nat_eq(a1522, a1523) {
                    let a1524 = v5;
                    let a1525 = 0u64;
                    if nat_eq(a1524, a1525) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1526 = v0;
                    let a1527 = 2u64;
                    if nat_eq(a1526, a1527) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1528 = v0;
            let a1529 = 4u64;
            if if nat_eq(a1528, a1529) {
                let a1530 = f114(v5)?;
                let a1531 = v1;
                !nat_eq(a1530, a1531)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1534 = {
                        let a1532 = v0;
                        let a1533 = 4u64;
                        nat_sub(a1532, a1533)
                    };
                    let a1535 = 3u64;
                    let a1536 = 0u64;
                    nat_quot(a1534, a1535, a1536)
                };
                let v7: u64 = {
                    let a1539 = {
                        let a1537 = v0;
                        let a1538 = 4u64;
                        nat_sub(a1537, a1538)
                    };
                    let a1540 = 3u64;
                    let a1541 = 0u64;
                    nat_rem(a1539, a1540, a1541)
                };
                let a1542 = v7;
                let a1543 = 0u64;
                if nat_eq(a1542, a1543) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1544 = v7;
                    let a1545 = 1u64;
                    if nat_eq(a1544, a1545) {
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
        let a1546 = v0;
        let a1547 = 4294967296u64;
        let a1548 = 0u64;
        nat_rem(a1546, a1547, a1548)
    };
    let a1549 = v1;
    let a1550 = 2147483648u64;
    if nat_lt(a1549, a1550) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1553 = 2u64;
    let a1554 = {
        let a1551 = 2u64;
        let a1552 = v0;
        nat_mul(a1551, a1552)?
    };
    nat_add(a1553, a1554)
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
    let a1555 = v2;
    let a1556 = 0u64;
    if nat_eq(a1555, a1556) {
        Ok(0u64)
    } else {
        let a1559 = 2u64;
        let a1560 = {
            let a1557 = v0;
            let a1558 = v1;
            nat_mul(a1557, a1558)?
        };
        nat_mul(a1559, a1560)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1561 = v2;
    let a1562 = f111(v0, v1)?;
    nat_mul(a1561, a1562)
}

pub fn f65(v0: u64) -> u64 {
    let a1563 = v0;
    let a1564 = 30u64;
    if nat_eq(a1563, a1564) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1567 = 5u64;
    let a1568 = {
        let a1565 = 4u64;
        let a1566 = v0;
        nat_mul(a1565, a1566)?
    };
    nat_add(a1567, a1568)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1569, _, _, _, _, _, _, _, _, _, _) => {
            h1569
        }
    }, match v0.clone() {
        Adt2::C0(_, h1570, _, _, _, _, _, _, _, _, _) => {
            h1570
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1571, _, _, _, _, _, _, _, _) => {
            h1571
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
            h1572
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1573, _, _, _, _, _, _) => {
            h1573
        }
    }, match v1.clone() {
        Adt2::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
            h1574
        }
    }, match v1.clone() {
        Adt2::C0(_, h1575, _, _, _, _, _, _, _, _, _) => {
            h1575
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1576, _, _, _, _, _, _, _, _) => {
            h1576
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1577, _, _, _, _, _, _, _) => {
            h1577
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1578, _, _, _, _, _, _) => {
            h1578
        }
    }, match v2.clone() {
        Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
            h1579
        }
    }, match v2.clone() {
        Adt2::C0(_, h1580, _, _, _, _, _, _, _, _, _) => {
            h1580
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1581, _, _, _, _, _, _, _, _) => {
            h1581
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1582, _, _, _, _, _, _, _) => {
            h1582
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1583, _, _, _, _, _, _) => {
            h1583
        }
    }) {
        let m1585 = f70(match v0.clone() {
            Adt2::C0(h1584, _, _, _, _, _, _, _, _, _, _) => {
                h1584
            }
        });
        match m1585 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1591 = {
                    let a1588 = match v0.clone() {
                        Adt2::C0(_, h1586, _, _, _, _, _, _, _, _, _) => {
                            h1586
                        }
                    };
                    let a1589 = match v3.clone() {
                        Adt11::C0(_, h1587, _) => {
                            h1587
                        }
                    };
                    let a1590 = 0u64;
                    nat_rem(a1588, a1589, a1590)
                };
                let a1592 = 0u64;
                if if nat_eq(a1591, a1592) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1593, _, _, _, _, _, _, _, _, _) => {
                            h1593
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                            h1594
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1595, _, _, _, _, _, _, _, _) => {
                            h1595
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1596, _, _, _, _, _, _, _) => {
                            h1596
                        }
                    }) {
                        true
                    } else {
                        let a1598 = 4u64;
                        let a1599 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1597, _, _, _, _, _) => {
                                h1597
                            }
                        };
                        if if nat_le(a1598, a1599) {
                            let a1601 = match v3.clone() {
                                Adt11::C0(_, h1600, _) => {
                                    h1600
                                }
                            };
                            let a1602 = 1u64;
                            if nat_eq(a1601, a1602) {
                                true
                            } else {
                                let a1605 = match v3.clone() {
                                    Adt11::C0(_, _, h1603) => {
                                        h1603
                                    }
                                };
                                let a1606 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1604, _, _, _, _, _) => {
                                        h1604
                                    }
                                };
                                nat_le(a1605, a1606)
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
                        Adt11::C0(_, h1607, _) => {
                            h1607
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1608, _, _, _, _, _, _, _, _, _) => {
                            h1608
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1609, _, _, _, _, _, _, _, _, _) => {
                            h1609
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1610, _, _, _, _, _, _, _, _) => {
                            h1610
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1611, _, _, _, _, _, _, _) => {
                            h1611
                        }
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1612, _, _, _, _, _, _, _, _, _, _) => {
                            h1612
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
    let a1615 = match v1.clone() {
        Adt2::C0(h1613, _, _, _, _, _, _, _, _, _, _) => {
            h1613
        }
    };
    let a1616 = match v3.clone() {
        Adt2::C0(h1614, _, _, _, _, _, _, _, _, _, _) => {
            h1614
        }
    };
    if if nat_eq(a1615, a1616) {
        let a1617 = v0;
        let a1618 = 33u64;
        if nat_eq(a1617, a1618) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1620 = f70(match v1.clone() {
            Adt2::C0(h1619, _, _, _, _, _, _, _, _, _, _) => {
                h1619
            }
        });
        match m1620 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1621, _) => {
                        h1621
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1626 = {
                    let a1623 = match v1.clone() {
                        Adt2::C0(_, h1622, _, _, _, _, _, _, _, _, _) => {
                            h1622
                        }
                    };
                    let a1624 = v5;
                    let a1625 = 1u64;
                    nat_rem(a1623, a1624, a1625)
                };
                let a1627 = 0u64;
                if if if nat_eq(a1626, a1627) {
                    let a1632 = {
                        let a1629 = match v3.clone() {
                            Adt2::C0(_, h1628, _, _, _, _, _, _, _, _, _) => {
                                h1628
                            }
                        };
                        let a1630 = v5;
                        let a1631 = 1u64;
                        nat_rem(a1629, a1630, a1631)
                    };
                    let a1633 = 0u64;
                    nat_eq(a1632, a1633)
                } else {
                    false
                } {
                    let a1634 = v6;
                    let a1635 = f9(v3.clone(), v5)?;
                    nat_eq(a1634, a1635)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1636) => {
                            h1636
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
    let a1637 = v0;
    let a1638 = 0u64;
    if if nat_eq(a1637, a1638) {
        true
    } else {
        let a1639 = v0;
        let a1640 = 1u64;
        nat_eq(a1639, a1640)
    } {
        true
    } else {
        let a1641 = v0;
        let a1642 = 30u64;
        nat_eq(a1641, a1642)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1643 = v0;
    let a1644 = 0u64;
    if nat_eq(a1643, a1644) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1645 = v0;
        let a1646 = 1u64;
        if nat_eq(a1645, a1646) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1647 = v0;
            let a1648 = 2u64;
            if nat_eq(a1647, a1648) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1649 = v0;
                let a1650 = 3u64;
                if nat_eq(a1649, a1650) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1651 = v0;
                    let a1652 = 6u64;
                    if nat_eq(a1651, a1652) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1653 = v0;
                        let a1654 = 7u64;
                        if nat_eq(a1653, a1654) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1655 = v0;
                            let a1656 = 8u64;
                            if nat_eq(a1655, a1656) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1657 = v0;
                                let a1658 = 9u64;
                                if nat_eq(a1657, a1658) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1659 = v0;
                                    let a1660 = 10u64;
                                    if nat_eq(a1659, a1660) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1661 = v0;
                                        let a1662 = 11u64;
                                        if nat_eq(a1661, a1662) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1663 = v0;
                                            let a1664 = 12u64;
                                            if nat_eq(a1663, a1664) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1665 = v0;
                                                let a1666 = 13u64;
                                                if nat_eq(a1665, a1666) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1667 = v0;
                                                    let a1668 = 14u64;
                                                    if nat_eq(a1667, a1668) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1669 = v0;
                                                        let a1670 = 15u64;
                                                        if nat_eq(a1669, a1670) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1671 = v0;
                                                            let a1672 = 16u64;
                                                            if nat_eq(a1671, a1672) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1673 = v0;
                                                                let a1674 = 17u64;
                                                                if nat_eq(a1673, a1674) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1675 = v0;
                                                                    let a1676 = 18u64;
                                                                    if nat_eq(a1675, a1676) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1677 = v0;
                                                                        let a1678 = 19u64;
                                                                        if nat_eq(a1677, a1678) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1679 = v0;
                                                                            let a1680 = 20u64;
                                                                            if nat_eq(a1679, a1680) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1681 = v0;
                                                                                let a1682 = 21u64;
                                                                                if nat_eq(a1681, a1682) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1683 = v0;
                                                                                    let a1684 = 22u64;
                                                                                    if nat_eq(a1683, a1684) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1685 = v0;
                                                                                        let a1686 = 23u64;
                                                                                        if nat_eq(a1685, a1686) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1687 = v0;
                                                                                            let a1688 = 24u64;
                                                                                            if nat_eq(a1687, a1688) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1689 = v0;
                                                                                                let a1690 = 25u64;
                                                                                                if nat_eq(a1689, a1690) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1691 = v0;
                                                                                                    let a1692 = 26u64;
                                                                                                    if nat_eq(a1691, a1692) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1693 = v0;
                                                                                                        let a1694 = 27u64;
                                                                                                        if nat_eq(a1693, a1694) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1695 = v0;
                                                                                                            let a1696 = 28u64;
                                                                                                            if nat_eq(a1695, a1696) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1697 = v0;
                                                                                                                let a1698 = 29u64;
                                                                                                                if nat_eq(a1697, a1698) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1699 = v0;
                                                                                                                    let a1700 = 30u64;
                                                                                                                    if nat_eq(a1699, a1700) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1701 = v0;
                                                                                                                        let a1702 = 34u64;
                                                                                                                        if nat_eq(a1701, a1702) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1703 = v0;
                                                                                                                            let a1704 = 35u64;
                                                                                                                            if nat_eq(a1703, a1704) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1705 = v0;
                                                                                                                                let a1706 = 39u64;
                                                                                                                                if nat_eq(a1705, a1706) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1707 = v0;
                                                                                                                                    let a1708 = 40u64;
                                                                                                                                    if nat_eq(a1707, a1708) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1709 = v0;
                                                                                                                                        let a1710 = 41u64;
                                                                                                                                        if nat_eq(a1709, a1710) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1711 = v0;
                                                                                                                                            let a1712 = 42u64;
                                                                                                                                            if nat_eq(a1711, a1712) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1713 = v0;
                                                                                                                                                let a1714 = 43u64;
                                                                                                                                                if nat_eq(a1713, a1714) {
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
        let a1718 = {
            let a1715 = v0;
            let a1716 = v4;
            let a1717 = 1u64;
            nat_rem(a1715, a1716, a1717)
        };
        let a1719 = 0u64;
        if if if nat_eq(a1718, a1719) {
            let a1723 = {
                let a1720 = v1;
                let a1721 = v5;
                let a1722 = 1u64;
                nat_rem(a1720, a1721, a1722)
            };
            let a1724 = 0u64;
            nat_eq(a1723, a1724)
        } else {
            false
        } {
            let a1728 = {
                let a1725 = v2;
                let a1726 = v6;
                let a1727 = 1u64;
                nat_rem(a1725, a1726, a1727)
            };
            let a1729 = 0u64;
            nat_eq(a1728, a1729)
        } else {
            false
        } {
            let a1733 = {
                let a1730 = v3;
                let a1731 = v7;
                let a1732 = 1u64;
                nat_rem(a1730, a1731, a1732)
            };
            let a1734 = 0u64;
            nat_eq(a1733, a1734)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1735 = v0;
    let a1736 = v1;
    if nat_eq(a1735, a1736) {
        let a1739 = v4;
        let a1740 = {
            let a1737 = v2;
            let a1738 = v3;
            nat_add(a1737, a1738)?
        };
        Ok(nat_eq(a1739, a1740))
    } else {
        let a1741 = v2;
        let a1742 = v3;
        if nat_eq(a1741, a1742) {
            let a1743 = v4;
            let a1744 = v2;
            Ok(nat_eq(a1743, a1744))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1747 = {
            let a1745 = v0;
            let a1746 = f119(v1);
            nat_add(a1745, a1746)?
        };
        let a1748 = f119(v2);
        nat_add(a1747, a1748)?
    };
    let v5: u64 = {
        let a1749 = f120(v1);
        let a1750 = f120(v2);
        nat_add(a1749, a1750)?
    };
    let a1751 = v1;
    let a1752 = 4294967295u64;
    if if nat_le(a1751, a1752) {
        let a1753 = v2;
        let a1754 = 4294967295u64;
        nat_le(a1753, a1754)
    } else {
        false
    } {
        let a1755 = v5;
        let a1756 = v4;
        if nat_le(a1755, a1756) {
            let a1759 = v3;
            let a1760 = {
                let a1757 = v4;
                let a1758 = v5;
                nat_sub(a1757, a1758)
            };
            Ok(nat_eq(a1759, a1760))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1761 = v1;
    let a1762 = 4294967295u64;
    if nat_le(a1761, a1762) {
        let a1765 = {
            let a1763 = f119(v1);
            let a1764 = f120(v1);
            nat_add(a1763, a1764)?
        };
        let a1766 = v0;
        Ok(nat_lt(a1765, a1766))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1767 = v6;
    let a1768 = v1;
    if if nat_le(a1767, a1768) {
        let a1775 = {
            let a1773 = {
                let a1769 = v1;
                let a1770 = v5;
                nat_add(a1769, a1770)?
            };
            let a1774 = {
                let a1771 = v3;
                let a1772 = 0u64;
                if nat_eq(a1771, a1772) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1773, a1774)?
        };
        let a1776 = v2;
        nat_le(a1775, a1776)
    } else {
        false
    } {
        if v4 {
            let a1781 = {
                let a1779 = {
                    let a1777 = v1;
                    let a1778 = v5;
                    nat_add(a1777, a1778)?
                };
                let a1780 = v6;
                nat_sub(a1779, a1780)
            };
            let a1782 = v2;
            Ok(nat_lt(a1781, a1782))
        } else {
            let a1787 = {
                let a1785 = {
                    let a1783 = v1;
                    let a1784 = v5;
                    nat_add(a1783, a1784)?
                };
                let a1786 = v6;
                nat_sub(a1785, a1786)
            };
            let a1788 = v2;
            Ok(nat_le(a1787, a1788))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1789 = v0;
    let a1790 = 0u64;
    if if nat_eq(a1789, a1790) {
        true
    } else {
        let a1791 = v0;
        let a1792 = 1u64;
        nat_eq(a1791, a1792)
    } {
        let a1793 = v1;
        let a1794 = 4294967296u64;
        nat_lt(a1793, a1794)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1799 = {
        let a1797 = v0;
        let a1798 = {
            let a1795 = v1;
            let a1796 = 1u64;
            nat_sub(a1795, a1796)
        };
        nat_add(a1797, a1798)?
    };
    let a1800 = v1;
    let a1801 = 0u64;
    Ok(nat_quot(a1799, a1800, a1801))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1802, _, _, _, _, _, _, _, _, _) => {
            h1802
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1803, _, _, _, _, _, _, _, _) => {
            h1803
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1804, _, _, _, _, _, _, _) => {
            h1804
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1805, _, _, _, _, _, _) => {
            h1805
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1808 = match v1.clone() {
                Adt2::C0(_, h1806, _, _, _, _, _, _, _, _, _) => {
                    h1806
                }
            };
            let a1809 = match v1.clone() {
                Adt2::C0(_, _, h1807, _, _, _, _, _, _, _, _) => {
                    h1807
                }
            };
            nat_mul(a1808, a1809)?
        };
        let v5: u64 = {
            let a1811 = v4;
            let a1812 = match v1.clone() {
                Adt2::C0(_, _, _, h1810, _, _, _, _, _, _, _) => {
                    h1810
                }
            };
            nat_mul(a1811, a1812)?
        };
        let a1825 = {
            let a1823 = {
                let a1814 = match v0.clone() {
                    Adt2::C0(_, h1813, _, _, _, _, _, _, _, _, _) => {
                        h1813
                    }
                };
                let a1815 = 1u64;
                nat_sub(a1814, a1815)
            };
            let a1824 = {
                let a1821 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1816, _, _, _, _, _, _, _, _) => {
                        h1816
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1817, _, _, _, _, _, _, _, _, _) => {
                        h1817
                    }
                })?;
                let a1822 = f122(match v0 {
                    Adt2::C0(_, _, _, h1818, _, _, _, _, _, _, _) => {
                        h1818
                    }
                }, v2, {
                    let a1819 = v3;
                    let a1820 = v5;
                    nat_mul(a1819, a1820)?
                }, v4)?;
                nat_add(a1821, a1822)?
            };
            nat_add(a1823, a1824)?
        };
        let a1826 = f9(v1, 1u64)?;
        Ok(nat_lt(a1825, a1826))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1827 = v0;
    let a1828 = 2147483648u64;
    if if if if nat_lt(a1827, a1828) {
        let a1829 = v1;
        let a1830 = 2147483648u64;
        nat_lt(a1829, a1830)
    } else {
        false
    } {
        let a1833 = {
            let a1831 = v0;
            let a1832 = v1;
            nat_mul(a1831, a1832)?
        };
        let a1834 = 2147483648u64;
        nat_lt(a1833, a1834)
    } else {
        false
    } {
        let a1839 = {
            let a1837 = {
                let a1835 = v0;
                let a1836 = 1u64;
                nat_sub(a1835, a1836)
            };
            let a1838 = v2;
            nat_mul(a1837, a1838)?
        };
        let a1840 = 2147483648u64;
        nat_lt(a1839, a1840)
    } else {
        false
    } {
        let a1845 = {
            let a1843 = {
                let a1841 = v1;
                let a1842 = 1u64;
                nat_sub(a1841, a1842)
            };
            let a1844 = v2;
            nat_mul(a1843, a1844)?
        };
        let a1846 = 2147483648u64;
        Ok(nat_lt(a1845, a1846))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1849 = 3u64;
    let a1850 = {
        let a1847 = v0;
        let a1848 = 3u64;
        nat_mul(a1847, a1848)?
    };
    nat_add(a1849, a1850)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1851, _, _, _, _, _, _, _) => {
            h1851
        }
    }, match v0.clone() {
        Adt3::C0(_, h1852, _, _, _, _, _, _) => {
            h1852
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1853, _, _, _, _, _) => {
            h1853
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1854, _, _, _, _) => {
            h1854
        }
    }) {
        Ok(0u64)
    } else {
        let a1863 = match v0.clone() {
            Adt3::C0(h1855, _, _, _, _, _, _, _) => {
                h1855
            }
        };
        let a1864 = {
            let a1861 = match v0.clone() {
                Adt3::C0(_, h1856, _, _, _, _, _, _) => {
                    h1856
                }
            };
            let a1862 = {
                let a1859 = match v0.clone() {
                    Adt3::C0(_, _, h1857, _, _, _, _, _) => {
                        h1857
                    }
                };
                let a1860 = match v0 {
                    Adt3::C0(_, _, _, h1858, _, _, _, _) => {
                        h1858
                    }
                };
                nat_mul(a1859, a1860)?
            };
            nat_mul(a1861, a1862)?
        };
        nat_mul(a1863, a1864)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1865 = v1;
    let a1866 = 13u64;
    if if nat_eq(a1865, a1866) {
        f23(match v0.clone() {
            Adt3::C0(h1867, _, _, _, _, _, _, _) => {
                h1867
            }
        }, match v0.clone() {
            Adt3::C0(_, h1868, _, _, _, _, _, _) => {
                h1868
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1869, _, _, _, _, _) => {
                h1869
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1870, _, _, _, _) => {
                h1870
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1876 = match v0.clone() {
                Adt3::C0(_, h1871, _, _, _, _, _, _) => {
                    h1871
                }
            };
            let a1877 = {
                let a1874 = match v0.clone() {
                    Adt3::C0(_, _, h1872, _, _, _, _, _) => {
                        h1872
                    }
                };
                let a1875 = match v0.clone() {
                    Adt3::C0(_, _, _, h1873, _, _, _, _) => {
                        h1873
                    }
                };
                nat_mul(a1874, a1875)?
            };
            nat_mul(a1876, a1877)?
        };
        let a1878 = v1;
        let a1879 = 13u64;
        if nat_eq(a1878, a1879) {
            let a1888 = 2u64;
            let a1889 = if f123(v0.clone(), v2) {
                let a1881 = v3;
                let a1882 = f124(match v0 {
                    Adt3::C0(h1880, _, _, _, _, _, _, _) => {
                        h1880
                    }
                }, false)?;
                nat_mul(a1881, a1882)?
            } else {
                let a1886 = v3;
                let a1887 = {
                    let a1884 = match v0 {
                        Adt3::C0(h1883, _, _, _, _, _, _, _) => {
                            h1883
                        }
                    };
                    let a1885 = 2u64;
                    nat_mul(a1884, a1885)?
                };
                nat_mul(a1886, a1887)?
            };
            nat_add(a1888, a1889)
        } else {
            let a1893 = v3;
            let a1894 = f124(match v0 {
                Adt3::C0(h1890, _, _, _, _, _, _, _) => {
                    h1890
                }
            }, {
                let a1891 = v1;
                let a1892 = 16u64;
                nat_eq(a1891, a1892)
            })?;
            nat_mul(a1893, a1894)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1895 = v3;
    match m1895 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1896, _, _, _, _, _, _, _, _, _, _) => {
                    h1896
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1897, _, _, _, _, _, _, _, _, _, _) => {
                    h1897
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1898, _, _, _, _, _, _, _, _, _) => {
                    h1898
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1899, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1900, _, _, _, _, _, _, _) => {
                    h1900
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1901, _, _, _, _, _, _) => {
                    h1901
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1902, _, _, _, _, _, _, _, _, _) => {
                    h1902
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1903, _, _, _, _, _, _, _, _) => {
                    h1903
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1904, _, _, _, _, _, _, _) => {
                    h1904
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1905, _, _, _, _, _, _) => {
                    h1905
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1906, _, _, _, _, _, _, _, _, _, _) => {
                    h1906
                }
            }, match v5.clone() {
                Adt2::C0(h1907, _, _, _, _, _, _, _, _, _, _) => {
                    h1907
                }
            }, match v4.clone() {
                Adt2::C0(h1908, _, _, _, _, _, _, _, _, _, _) => {
                    h1908
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1909, _, _, _, _, _, _, _, _, _) => {
                    h1909
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1910, _, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1911, _, _, _, _, _, _, _) => {
                    h1911
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1912, _, _, _, _, _, _) => {
                    h1912
                }
            }, match v5.clone() {
                Adt2::C0(_, h1913, _, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1914, _, _, _, _, _, _, _, _) => {
                    h1914
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1915, _, _, _, _, _, _, _) => {
                    h1915
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1916, _, _, _, _, _, _) => {
                    h1916
                }
            }, match v4.clone() {
                Adt2::C0(_, h1917, _, _, _, _, _, _, _, _, _) => {
                    h1917
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1918, _, _, _, _, _, _, _, _) => {
                    h1918
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1919, _, _, _, _, _, _, _) => {
                    h1919
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1920, _, _, _, _, _, _) => {
                    h1920
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1921 = v7;
        let a1922 = 6u64;
        if nat_eq(a1921, a1922) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1923 = v0;
    match m1923 {
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
            let a1924 = 13u64;
            let a1925 = v34;
            if nat_le(a1924, a1925) {
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
    let m1926 = v0;
    match m1926 {
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
    let m1927 = v0;
    match m1927 {
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
            let a1928 = v1;
            let a1929 = 6u64;
            if nat_eq(a1928, a1929) {
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
            let a1930 = v1;
            let a1931 = 6u64;
            if if if nat_eq(a1930, a1931) {
                true
            } else {
                let a1932 = v1;
                let a1933 = 7u64;
                nat_eq(a1932, a1933)
            } {
                let a1934 = v49;
                let a1935 = 30u64;
                nat_eq(a1934, a1935)
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
    let a1936 = v1;
    let a1937 = 4u64;
    if nat_lt(a1936, a1937) {
        Adt7::C0(v1)
    } else {
        let a1938 = v1;
        let a1939 = 4u64;
        if nat_eq(a1938, a1939) {
            Adt7::C1(0u64)
        } else {
            let a1940 = v1;
            let a1941 = 5u64;
            if nat_eq(a1940, a1941) {
                Adt7::C1(3u64)
            } else {
                let a1942 = v1;
                let a1943 = 6u64;
                if nat_eq(a1942, a1943) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1944 = v1;
                    let a1945 = 7u64;
                    if nat_eq(a1944, a1945) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1946 = v1;
                        let a1947 = 16u64;
                        if nat_lt(a1946, a1947) {
                            Adt7::C1({
                                let a1948 = v1;
                                let a1949 = 8u64;
                                nat_sub(a1948, a1949)
                            })
                        } else {
                            let a1950 = v1;
                            let a1951 = 20u64;
                            if nat_lt(a1950, a1951) {
                                Adt7::C5({
                                    let a1952 = v1;
                                    let a1953 = 16u64;
                                    nat_sub(a1952, a1953)
                                })
                            } else {
                                let a1954 = v1;
                                let a1955 = 20u64;
                                if nat_eq(a1954, a1955) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1956 = v1;
                                    let a1957 = 21u64;
                                    if nat_eq(a1956, a1957) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1958 = v1;
                                        let a1959 = 22u64;
                                        if nat_eq(a1958, a1959) {
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
        let a1961 = v0;
        let a1962 = match v1.clone() {
            Adt3::C0(h1960, _, _, _, _, _, _, _) => {
                h1960
            }
        };
        let a1963 = 0u64;
        nat_quot(a1961, a1962, a1963)
    };
    let v3: u64 = {
        let a1965 = v2;
        let a1966 = match v1.clone() {
            Adt3::C0(_, h1964, _, _, _, _, _, _) => {
                h1964
            }
        };
        let a1967 = 0u64;
        nat_quot(a1965, a1966, a1967)
    };
    f129({
        let a1969 = v0;
        let a1970 = match v1.clone() {
            Adt3::C0(h1968, _, _, _, _, _, _, _) => {
                h1968
            }
        };
        let a1971 = 0u64;
        nat_rem(a1969, a1970, a1971)
    }, {
        let a1973 = v2;
        let a1974 = match v1.clone() {
            Adt3::C0(_, h1972, _, _, _, _, _, _) => {
                h1972
            }
        };
        let a1975 = 0u64;
        nat_rem(a1973, a1974, a1975)
    }, {
        let a1977 = v3;
        let a1978 = match v1.clone() {
            Adt3::C0(_, _, h1976, _, _, _, _, _) => {
                h1976
            }
        };
        let a1979 = 0u64;
        nat_rem(a1977, a1978, a1979)
    }, {
        let a1981 = v3;
        let a1982 = match v1.clone() {
            Adt3::C0(_, _, h1980, _, _, _, _, _) => {
                h1980
            }
        };
        let a1983 = 0u64;
        nat_quot(a1981, a1982, a1983)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1984, _, _, _) => {
            h1984
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1985, _, _) => {
            h1985
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1986, _) => {
            h1986
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1987) => {
            h1987
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2005 = {
        let a2000 = f130(v2, {
            let a1991 = {
                let a1988 = v0;
                let a1989 = v1;
                let a1990 = 0u64;
                nat_quot(a1988, a1989, a1990)
            };
            let a1992 = v2;
            let a1993 = 0u64;
            nat_rem(a1991, a1992, a1993)
        }, {
            let a1997 = {
                let a1994 = v0;
                let a1995 = v1;
                let a1996 = 0u64;
                nat_quot(a1994, a1995, a1996)
            };
            let a1998 = v2;
            let a1999 = 0u64;
            nat_quot(a1997, a1998, a1999)
        })?;
        let a2001 = v1;
        nat_mul(a2000, a2001)?
    };
    let a2006 = {
        let a2002 = v0;
        let a2003 = v1;
        let a2004 = 0u64;
        nat_rem(a2002, a2003, a2004)
    };
    nat_add(a2005, a2006)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2008 = v0;
        let a2009 = match v2.clone() {
            Adt3::C0(h2007, _, _, _, _, _, _, _) => {
                h2007
            }
        };
        let a2010 = 0u64;
        nat_quot(a2008, a2009, a2010)
    };
    let v4: u64 = {
        let a2012 = v3;
        let a2013 = match v2.clone() {
            Adt3::C0(_, h2011, _, _, _, _, _, _) => {
                h2011
            }
        };
        let a2014 = 0u64;
        nat_quot(a2012, a2013, a2014)
    };
    f131({
        let a2016 = v0;
        let a2017 = match v2.clone() {
            Adt3::C0(h2015, _, _, _, _, _, _, _) => {
                h2015
            }
        };
        let a2018 = 0u64;
        nat_rem(a2016, a2017, a2018)
    }, {
        let a2020 = v3;
        let a2021 = match v2.clone() {
            Adt3::C0(_, h2019, _, _, _, _, _, _) => {
                h2019
            }
        };
        let a2022 = 0u64;
        nat_rem(a2020, a2021, a2022)
    }, {
        let a2024 = v4;
        let a2025 = match v2.clone() {
            Adt3::C0(_, _, h2023, _, _, _, _, _) => {
                h2023
            }
        };
        let a2026 = 0u64;
        nat_rem(a2024, a2025, a2026)
    }, {
        let a2028 = v4;
        let a2029 = match v2 {
            Adt3::C0(_, _, h2027, _, _, _, _, _) => {
                h2027
            }
        };
        let a2030 = 0u64;
        nat_quot(a2028, a2029, a2030)
    }, match v1.clone() {
        Adt3::C0(h2031, _, _, _, _, _, _, _) => {
            h2031
        }
    }, match v1.clone() {
        Adt3::C0(_, h2032, _, _, _, _, _, _) => {
            h2032
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2033, _, _, _, _, _) => {
            h2033
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2034, _, _, _, _) => {
            h2034
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2035, _, _, _) => {
            h2035
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2036, _, _) => {
            h2036
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2037, _) => {
            h2037
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2038) => {
            h2038
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2040 = v0;
        let a2041 = match v3.clone() {
            Adt3::C0(h2039, _, _, _, _, _, _, _) => {
                h2039
            }
        };
        let a2042 = 0u64;
        nat_quot(a2040, a2041, a2042)
    };
    let v6: u64 = {
        let a2044 = v5;
        let a2045 = match v3.clone() {
            Adt3::C0(_, h2043, _, _, _, _, _, _) => {
                h2043
            }
        };
        let a2046 = 0u64;
        nat_quot(a2044, a2045, a2046)
    };
    let v7: u64 = {
        let a2048 = v0;
        let a2049 = match v3.clone() {
            Adt3::C0(h2047, _, _, _, _, _, _, _) => {
                h2047
            }
        };
        let a2050 = 0u64;
        nat_rem(a2048, a2049, a2050)
    };
    let v8: u64 = {
        let a2052 = v5;
        let a2053 = match v3.clone() {
            Adt3::C0(_, h2051, _, _, _, _, _, _) => {
                h2051
            }
        };
        let a2054 = 0u64;
        nat_rem(a2052, a2053, a2054)
    };
    let v9: u64 = {
        let a2056 = v6;
        let a2057 = match v3.clone() {
            Adt3::C0(_, _, h2055, _, _, _, _, _) => {
                h2055
            }
        };
        let a2058 = 0u64;
        nat_rem(a2056, a2057, a2058)
    };
    let v10: u64 = {
        let a2060 = v6;
        let a2061 = match v3.clone() {
            Adt3::C0(_, _, h2059, _, _, _, _, _) => {
                h2059
            }
        };
        let a2062 = 0u64;
        nat_quot(a2060, a2061, a2062)
    };
    let v11: u64 = {
        let a2063 = v4;
        let a2064 = 0u64;
        if nat_eq(a2063, a2064) {
            match v1.clone() {
                Adt3::C0(h2065, _, _, _, _, _, _, _) => {
                    h2065
                }
            }
        } else {
            let a2066 = v4;
            let a2067 = 1u64;
            if nat_eq(a2066, a2067) {
                match v1.clone() {
                    Adt3::C0(_, h2068, _, _, _, _, _, _) => {
                        h2068
                    }
                }
            } else {
                let a2069 = v4;
                let a2070 = 2u64;
                if nat_eq(a2069, a2070) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2071, _, _, _, _, _) => {
                            h2071
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2072, _, _, _, _) => {
                            h2072
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2079 = v11;
        let a2080 = {
            let a2073 = v4;
            let a2074 = 0u64;
            if nat_eq(a2073, a2074) {
                v7
            } else {
                let a2075 = v4;
                let a2076 = 1u64;
                if nat_eq(a2075, a2076) {
                    v8
                } else {
                    let a2077 = v4;
                    let a2078 = 2u64;
                    if nat_eq(a2077, a2078) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2079, a2080)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2081 = v4;
        let a2082 = 0u64;
        nat_eq(a2081, a2082)
    } else {
        false
    } {
        let a2083 = v7;
        let a2084 = v11;
        nat_sub(a2083, a2084)
    } else {
        v7
    }, if if v12 {
        let a2085 = v4;
        let a2086 = 1u64;
        nat_eq(a2085, a2086)
    } else {
        false
    } {
        let a2087 = v8;
        let a2088 = v11;
        nat_sub(a2087, a2088)
    } else {
        v8
    }, if if v12 {
        let a2089 = v4;
        let a2090 = 2u64;
        nat_eq(a2089, a2090)
    } else {
        false
    } {
        let a2091 = v9;
        let a2092 = v11;
        nat_sub(a2091, a2092)
    } else {
        v9
    }, if if v12 {
        let a2093 = v4;
        let a2094 = 3u64;
        nat_eq(a2093, a2094)
    } else {
        false
    } {
        let a2095 = v10;
        let a2096 = v11;
        nat_sub(a2095, a2096)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2097, _, _, _) => {
            h2097
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2098, _, _) => {
            h2098
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2099, _) => {
            h2099
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2100) => {
            h2100
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
            Adt3::C0(h2101, _, _, _, _, _, _, _) => {
                h2101
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2102, _, _, _, _, _, _) => {
                h2102
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2103, _, _, _, _, _) => {
                h2103
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2104, _, _, _, _) => {
                h2104
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2105, _, _, _, _, _, _, _) => {
                h2105
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2106, _, _, _, _, _, _) => {
                    h2106
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2107, _, _, _, _, _) => {
                    h2107
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2108, _, _, _, _) => {
                    h2108
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
        Adt3::C0(h2109, _, _, _, _, _, _, _) => {
            h2109
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2110 = v0;
    let a2111 = v3;
    if nat_lt(a2110, a2111) {
        Ok(Adt5::C1(false, {
            let a2112 = v0;
            let a2113 = 4u64;
            nat_mul(a2112, a2113)?
        }, {
            let a2114 = v0;
            let a2115 = 4u64;
            nat_mul(a2114, a2115)?
        }))
    } else {
        let v6: u64 = {
            let a2116 = v0;
            let a2117 = v3;
            nat_sub(a2116, a2117)
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
    let a2118 = v0;
    let a2119 = v3;
    if nat_lt(a2118, a2119) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2120 = v0;
            let a2121 = v3;
            nat_sub(a2120, a2121)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2123 = v1;
        let a2124 = match v3.clone() {
            Adt3::C0(h2122, _, _, _, _, _, _, _) => {
                h2122
            }
        };
        let a2125 = 0u64;
        nat_rem(a2123, a2124, a2125)
    };
    let v5: u64 = {
        let a2131 = {
            let a2127 = v1;
            let a2128 = match v3.clone() {
                Adt3::C0(h2126, _, _, _, _, _, _, _) => {
                    h2126
                }
            };
            let a2129 = 0u64;
            nat_quot(a2127, a2128, a2129)
        };
        let a2132 = match v3.clone() {
            Adt3::C0(_, h2130, _, _, _, _, _, _) => {
                h2130
            }
        };
        let a2133 = 0u64;
        nat_rem(a2131, a2132, a2133)
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
        let a2134 = v3;
        let a2135 = 8u64;
        nat_add(a2134, a2135)?
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
        let a2137 = v0;
        let a2138 = match v2.clone() {
            Adt3::C0(h2136, _, _, _, _, _, _, _) => {
                h2136
            }
        };
        let a2139 = 0u64;
        nat_rem(a2137, a2138, a2139)
    };
    let v7: u64 = {
        let a2141 = v0;
        let a2142 = match v2.clone() {
            Adt3::C0(h2140, _, _, _, _, _, _, _) => {
                h2140
            }
        };
        let a2143 = 0u64;
        nat_quot(a2141, a2142, a2143)
    };
    let v8: u64 = {
        let a2145 = v7;
        let a2146 = match v2.clone() {
            Adt3::C0(_, h2144, _, _, _, _, _, _) => {
                h2144
            }
        };
        let a2147 = 0u64;
        nat_rem(a2145, a2146, a2147)
    };
    let v9: u64 = {
        let a2149 = v7;
        let a2150 = match v2.clone() {
            Adt3::C0(_, h2148, _, _, _, _, _, _) => {
                h2148
            }
        };
        let a2151 = 0u64;
        nat_quot(a2149, a2150, a2151)
    };
    let v10: u64 = {
        let a2153 = v9;
        let a2154 = match v2.clone() {
            Adt3::C0(_, _, h2152, _, _, _, _, _) => {
                h2152
            }
        };
        let a2155 = 0u64;
        nat_rem(a2153, a2154, a2155)
    };
    let v11: u64 = {
        let a2157 = v9;
        let a2158 = match v2.clone() {
            Adt3::C0(_, _, h2156, _, _, _, _, _) => {
                h2156
            }
        };
        let a2159 = 0u64;
        nat_quot(a2157, a2158, a2159)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2168 = {
                let a2163 = {
                    let a2160 = v10;
                    let a2161 = v4;
                    let a2162 = 0u64;
                    nat_quot(a2160, a2161, a2162)
                };
                let a2164 = v5;
                nat_mul(a2163, a2164)?
            };
            let a2169 = {
                let a2165 = v8;
                let a2166 = v4;
                let a2167 = 0u64;
                nat_quot(a2165, a2166, a2167)
            };
            nat_add(a2168, a2169)?
        };
        let v14: u64 = {
            let a2189 = v6;
            let a2190 = {
                let a2187 = match v1.clone() {
                    Adt3::C0(h2170, _, _, _, _, _, _, _) => {
                        h2170
                    }
                };
                let a2188 = {
                    let a2185 = {
                        let a2171 = v8;
                        let a2172 = v4;
                        let a2173 = 0u64;
                        nat_rem(a2171, a2172, a2173)
                    };
                    let a2186 = {
                        let a2183 = match v1.clone() {
                            Adt3::C0(_, h2174, _, _, _, _, _, _) => {
                                h2174
                            }
                        };
                        let a2184 = {
                            let a2181 = {
                                let a2175 = v10;
                                let a2176 = v4;
                                let a2177 = 0u64;
                                nat_rem(a2175, a2176, a2177)
                            };
                            let a2182 = {
                                let a2179 = match v1.clone() {
                                    Adt3::C0(_, _, h2178, _, _, _, _, _) => {
                                        h2178
                                    }
                                };
                                let a2180 = v13;
                                nat_mul(a2179, a2180)?
                            };
                            nat_add(a2181, a2182)?
                        };
                        nat_mul(a2183, a2184)?
                    };
                    nat_add(a2185, a2186)?
                };
                nat_mul(a2187, a2188)?
            };
            nat_add(a2189, a2190)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2196 = {
                let a2194 = {
                    let a2191 = v11;
                    let a2192 = v5;
                    let a2193 = 0u64;
                    nat_rem(a2191, a2192, a2193)
                };
                let a2195 = v4;
                nat_mul(a2194, a2195)?
            };
            let a2197 = v8;
            nat_add(a2196, a2197)?
        };
        let v16: u64 = {
            let a2203 = {
                let a2201 = {
                    let a2198 = v11;
                    let a2199 = v5;
                    let a2200 = 0u64;
                    nat_quot(a2198, a2199, a2200)
                };
                let a2202 = v4;
                nat_mul(a2201, a2202)?
            };
            let a2204 = v10;
            nat_add(a2203, a2204)?
        };
        let a2206 = v15;
        let a2207 = match v1.clone() {
            Adt3::C0(_, h2205, _, _, _, _, _, _) => {
                h2205
            }
        };
        if if nat_lt(a2206, a2207) {
            let a2209 = v16;
            let a2210 = match v1.clone() {
                Adt3::C0(_, _, h2208, _, _, _, _, _) => {
                    h2208
                }
            };
            nat_lt(a2209, a2210)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2219 = v6;
                let a2220 = {
                    let a2217 = match v1.clone() {
                        Adt3::C0(h2211, _, _, _, _, _, _, _) => {
                            h2211
                        }
                    };
                    let a2218 = {
                        let a2215 = v15;
                        let a2216 = {
                            let a2213 = match v1.clone() {
                                Adt3::C0(_, h2212, _, _, _, _, _, _) => {
                                    h2212
                                }
                            };
                            let a2214 = v16;
                            nat_mul(a2213, a2214)?
                        };
                        nat_add(a2215, a2216)?
                    };
                    nat_mul(a2217, a2218)?
                };
                nat_add(a2219, a2220)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2225 = {
        let a2222 = v0;
        let a2223 = match v1 {
            Adt3::C0(h2221, _, _, _, _, _, _, _) => {
                h2221
            }
        };
        let a2224 = 0u64;
        nat_rem(a2222, a2223, a2224)
    };
    let a2226 = 0u64;
    if nat_eq(a2225, a2226) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2227 = v0;
            let a2228 = 1u64;
            nat_sub(a2227, a2228)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2229 = v3;
    let a2230 = 13u64;
    if nat_eq(a2229, a2230) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2231, _, _, _, _, _, _, _) => {
                h2231
            }
        }, {
            let a2232 = v3;
            let a2233 = 16u64;
            nat_eq(a2232, a2233)
        })?;
        let v6: u64 = {
            let a2234 = v0;
            let a2235 = v5;
            let a2236 = 0u64;
            nat_quot(a2234, a2235, a2236)
        };
        let v7: u64 = {
            let a2237 = v0;
            let a2238 = v5;
            let a2239 = 0u64;
            nat_rem(a2237, a2238, a2239)
        };
        let v8: u64 = {
            let a2243 = 1u64;
            let a2244 = {
                let a2241 = match v1.clone() {
                    Adt3::C0(h2240, _, _, _, _, _, _, _) => {
                        h2240
                    }
                };
                let a2242 = 2u64;
                nat_mul(a2241, a2242)?
            };
            nat_add(a2243, a2244)?
        };
        let a2245 = v7;
        let a2246 = v8;
        if nat_lt(a2245, a2246) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2247 = v3;
            let a2248 = 14u64;
            if nat_eq(a2247, a2248) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2249 = v7;
                let a2250 = v8;
                if nat_eq(a2249, a2250) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2253 = v7;
                    let a2254 = {
                        let a2251 = v8;
                        let a2252 = 1u64;
                        nat_add(a2251, a2252)?
                    };
                    if nat_eq(a2253, a2254) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2255, _, _, _, _, _, _, _) => {
                                h2255
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
    let a2256 = v0;
    let a2257 = 0u64;
    if nat_eq(a2256, a2257) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2264 = {
            let a2258 = v0;
            let a2259 = 1u64;
            nat_add(a2258, a2259)?
        };
        let a2265 = {
            let a2262 = 2u64;
            let a2263 = {
                let a2260 = v3;
                let a2261 = 2u64;
                nat_mul(a2260, a2261)?
            };
            nat_add(a2262, a2263)?
        };
        if nat_eq(a2264, a2265) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2266 = v0;
                let a2267 = 1u64;
                nat_sub(a2266, a2267)
            };
            let v5: u64 = {
                let a2268 = v4;
                let a2269 = 2u64;
                let a2270 = 0u64;
                nat_quot(a2268, a2269, a2270)
            };
            let a2274 = {
                let a2271 = v4;
                let a2272 = 2u64;
                let a2273 = 0u64;
                nat_rem(a2271, a2272, a2273)
            };
            let a2275 = 0u64;
            if nat_eq(a2274, a2275) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2276 = v0;
    let a2277 = f18(v1, v2, v3)?;
    if nat_lt(a2276, a2277) {
        let a2278 = v0;
        let a2279 = v1;
        if nat_lt(a2278, a2279) {
            Ok(Adt5::C1(false, {
                let a2280 = 4u64;
                let a2281 = v0;
                nat_mul(a2280, a2281)?
            }, {
                let a2282 = 4u64;
                let a2283 = v0;
                nat_mul(a2282, a2283)?
            }))
        } else {
            let v4: u64 = {
                let a2284 = v0;
                let a2285 = v1;
                nat_sub(a2284, a2285)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2286 = v5;
            let a2287 = 2u64;
            if nat_lt(a2286, a2287) {
                Ok(Adt5::C4(6u64, {
                    let a2288 = v5;
                    let a2289 = 0u64;
                    if nat_eq(a2288, a2289) {
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
    let a2297 = {
        let a2290 = 2u64;
        let a2291 = f81(v1.clone())?;
        nat_mul(a2290, a2291)?
    };
    let a2298 = {
        let a2295 = f81(v1)?;
        let a2296 = {
            let a2293 = 1u64;
            let a2294 = match v0 {
                Adt3::C0(h2292, _, _, _, _, _, _, _) => {
                    h2292
                }
            };
            nat_add(a2293, a2294)?
        };
        nat_mul(a2295, a2296)?
    };
    nat_add(a2297, a2298)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2299 = v1;
    let a2300 = 2147483647u64;
    if nat_le(a2299, a2300) {
        let a2301 = v1;
        let a2302 = v0;
        nat_lt(a2301, a2302)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2303 = f70(v0);
    match m2303 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2305 = v2;
                let a2306 = match v7.clone() {
                    Adt11::C0(_, h2304, _) => {
                        h2304
                    }
                };
                nat_mul(a2305, a2306)?
            };
            let m2319 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2307, _, _, _, _, _, _, _) => {
                    h2307
                }
            }, match v5.clone() {
                Adt3::C0(_, h2308, _, _, _, _, _, _) => {
                    h2308
                }
            }, match v5 {
                Adt3::C0(_, _, h2309, _, _, _, _, _) => {
                    h2309
                }
            }, {
                let a2312 = match v3.clone() {
                    Adt3::C0(h2310, _, _, _, _, _, _, _) => {
                        h2310
                    }
                };
                let a2313 = match v7.clone() {
                    Adt11::C0(_, h2311, _) => {
                        h2311
                    }
                };
                nat_mul(a2312, a2313)?
            }, match v3.clone() {
                Adt3::C0(_, h2314, _, _, _, _, _, _) => {
                    h2314
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2315, _, _, _) => {
                    h2315
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2316, _, _) => {
                    h2316
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2317, _) => {
                    h2317
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2318) => {
                    h2318
                }
            })?;
            match m2319 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2320 = v0;
                    let a2321 = 0u64;
                    if if nat_eq(a2320, a2321) {
                        true
                    } else {
                        let a2322 = v0;
                        let a2323 = 26u64;
                        nat_eq(a2322, a2323)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2327 = {
                                let a2325 = v1;
                                let a2326 = match v4.clone() {
                                    Adt3::C0(h2324, _, _, _, _, _, _, _) => {
                                        h2324
                                    }
                                };
                                nat_mul(a2325, a2326)?
                            };
                            let a2328 = v8;
                            nat_add(a2327, a2328)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2329) => {
                                h2329
                            }
                        }, f89({
                            let a2333 = {
                                let a2331 = v1;
                                let a2332 = match v4.clone() {
                                    Adt3::C0(h2330, _, _, _, _, _, _, _) => {
                                        h2330
                                    }
                                };
                                nat_mul(a2331, a2332)?
                            };
                            let a2334 = v8;
                            nat_add(a2333, a2334)?
                        }, v4.clone())?, {
                            let a2341 = {
                                let a2339 = {
                                    let a2336 = match v7 {
                                        Adt11::C0(_, h2335, _) => {
                                            h2335
                                        }
                                    };
                                    let a2337 = 1u64;
                                    nat_sub(a2336, a2337)
                                };
                                let a2340 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2338, _, _, _) => {
                                        h2338
                                    }
                                };
                                nat_mul(a2339, a2340)?
                            };
                            let a2342 = 4u64;
                            nat_add(a2341, a2342)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2343, _, _, _) => {
                                h2343
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2344 = v1;
    let a2345 = 0u64;
    if if nat_eq(a2344, a2345) {
        true
    } else {
        let a2346 = v2;
        let a2347 = 0u64;
        nat_eq(a2346, a2347)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2348 = v0;
    let a2349 = 4294967295u64;
    if nat_eq(a2348, a2349) {
        true
    } else {
        let a2350 = v0;
        let a2351 = 2147483648u64;
        if nat_lt(a2350, a2351) {
            let a2352 = v0;
            let a2353 = v1;
            nat_lt(a2352, a2353)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2360 = v3;
        let a2361 = {
            let a2357 = f89({
                let a2354 = v0;
                let a2355 = 2u64;
                let a2356 = 0u64;
                nat_quot(a2354, a2355, a2356)
            }, v1)?;
            let a2358 = v2;
            let a2359 = 0u64;
            nat_quot(a2357, a2358, a2359)
        };
        nat_add(a2360, a2361)?
    };
    let a2365 = {
        let a2362 = v0;
        let a2363 = 2u64;
        let a2364 = 0u64;
        nat_rem(a2362, a2363, a2364)
    };
    let a2366 = 0u64;
    if nat_eq(a2365, a2366) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2367 = v4;
        let a2368 = 0u64;
        if nat_eq(a2367, a2368) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2371 = 8u64;
    let a2372 = {
        let a2369 = f147(v0);
        let a2370 = v1;
        nat_mul(a2369, a2370)?
    };
    nat_add(a2371, a2372)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2373, _, _, _, _, _, _, _) => {
            h2373
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2376 = 1u64;
        let a2377 = {
            let a2374 = v6;
            let a2375 = v7;
            nat_mul(a2374, a2375)?
        };
        nat_add(a2376, a2377)?
    };
    let a2378 = v0;
    let a2379 = 0u64;
    if nat_eq(a2378, a2379) {
        Ok(f148(0u64))
    } else {
        let a2380 = v0;
        let a2381 = v8;
        if nat_lt(a2380, a2381) {
            let v9: u64 = {
                let a2382 = v0;
                let a2383 = 1u64;
                nat_sub(a2382, a2383)
            };
            let v10: u64 = {
                let a2384 = v9;
                let a2385 = v7;
                let a2386 = 0u64;
                nat_rem(a2384, a2385, a2386)
            };
            let v11: u64 = f89({
                let a2392 = {
                    let a2387 = v1;
                    let a2388 = v6;
                    nat_mul(a2387, a2388)?
                };
                let a2393 = {
                    let a2389 = v9;
                    let a2390 = v7;
                    let a2391 = 0u64;
                    nat_quot(a2389, a2390, a2391)
                };
                nat_add(a2392, a2393)?
            }, v2)?;
            let a2394 = v3;
            let a2395 = 30u64;
            if nat_eq(a2394, a2395) {
                let a2396 = v10;
                let a2397 = 0u64;
                if nat_eq(a2396, a2397) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2398 = v10;
                    let a2399 = 1u64;
                    if nat_eq(a2398, a2399) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2400 = v10;
                        let a2401 = 2u64;
                        if nat_eq(a2400, a2401) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2402 = v10;
                let a2403 = 0u64;
                if nat_eq(a2402, a2403) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2404 = v10;
                    let a2405 = 1u64;
                    if nat_eq(a2404, a2405) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2406 = v0;
                let a2407 = v8;
                nat_sub(a2406, a2407)
            };
            let a2408 = v12;
            let a2409 = 0u64;
            if nat_eq(a2408, a2409) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2410 = v12;
                let a2411 = 1u64;
                if nat_eq(a2410, a2411) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2412 = v12;
                    let a2413 = 2u64;
                    if nat_eq(a2412, a2413) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2414 = v12;
                        let a2415 = 3u64;
                        if nat_eq(a2414, a2415) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2416 = v12;
                            let a2417 = 4u64;
                            if nat_eq(a2416, a2417) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2418 = v12;
                                let a2419 = 5u64;
                                if nat_eq(a2418, a2419) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2420 = v12;
                                    let a2421 = 6u64;
                                    if nat_eq(a2420, a2421) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2422 = 3u64;
                                                let a2423 = v1;
                                                nat_add(a2422, a2423)?
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
        let a2424 = v0;
        let a2425 = f65(v3);
        let a2426 = 0u64;
        nat_quot(a2424, a2425, a2426)
    };
    let v5: u64 = {
        let a2431 = 3u64;
        let a2432 = {
            let a2428 = v4;
            let a2429 = match v1.clone() {
                Adt3::C0(h2427, _, _, _, _, _, _, _) => {
                    h2427
                }
            };
            let a2430 = 0u64;
            nat_quot(a2428, a2429, a2430)
        };
        nat_add(a2431, a2432)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2433 = v3;
    let a2434 = 30u64;
    if nat_eq(a2433, a2434) {
        let a2438 = {
            let a2435 = v0;
            let a2436 = 3u64;
            let a2437 = 0u64;
            nat_rem(a2435, a2436, a2437)
        };
        let a2439 = 0u64;
        if nat_eq(a2438, a2439) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2443 = {
                let a2440 = v0;
                let a2441 = 3u64;
                let a2442 = 0u64;
                nat_rem(a2440, a2441, a2442)
            };
            let a2444 = 1u64;
            if nat_eq(a2443, a2444) {
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
        let a2445 = v0;
        let a2446 = 8388608u64;
        let a2447 = 0u64;
        nat_quot(a2445, a2446, a2447)
    };
    let a2448 = v0;
    let a2449 = 2147483648u64;
    if if if nat_lt(a2448, a2449) {
        let a2450 = 127u64;
        let a2451 = v1;
        nat_le(a2450, a2451)
    } else {
        false
    } {
        let a2452 = v1;
        let a2453 = 190u64;
        nat_lt(a2452, a2453)
    } else {
        false
    } {
        let v2: u64 = {
            let a2457 = 8388608u64;
            let a2458 = {
                let a2454 = v0;
                let a2455 = 8388608u64;
                let a2456 = 0u64;
                nat_rem(a2454, a2455, a2456)
            };
            nat_add(a2457, a2458)?
        };
        let a2459 = v1;
        let a2460 = 150u64;
        if nat_lt(a2459, a2460) {
            let v3: u64 = f150({
                let a2461 = 150u64;
                let a2462 = v1;
                nat_sub(a2461, a2462)
            });
            let a2466 = {
                let a2463 = v2;
                let a2464 = v3;
                let a2465 = 0u64;
                nat_rem(a2463, a2464, a2465)
            };
            let a2467 = 0u64;
            if nat_eq(a2466, a2467) {
                let a2468 = v2;
                let a2469 = v3;
                let a2470 = 0u64;
                Ok(nat_quot(a2468, a2469, a2470))
            } else {
                Ok(0u64)
            }
        } else {
            let a2473 = f150({
                let a2471 = v1;
                let a2472 = 150u64;
                nat_sub(a2471, a2472)
            });
            let a2474 = v2;
            nat_mul(a2473, a2474)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2475 = v0;
    let a2476 = 1u64;
    if nat_le(a2475, a2476) {
        true
    } else {
        let a2480 = {
            let a2477 = v1;
            let a2478 = v2;
            let a2479 = 1u64;
            nat_rem(a2477, a2478, a2479)
        };
        let a2481 = 0u64;
        nat_eq(a2480, a2481)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2482 = v1;
    let a2483 = 1u64;
    if if if nat_le(a2482, a2483) {
        true
    } else {
        let a2484 = v0;
        let a2485 = v2;
        nat_le(a2484, a2485)
    } {
        let a2486 = v3;
        let a2487 = 1u64;
        if nat_le(a2486, a2487) {
            true
        } else {
            let a2494 = {
                let a2492 = v0;
                let a2493 = {
                    let a2490 = {
                        let a2488 = v1;
                        let a2489 = 1u64;
                        nat_sub(a2488, a2489)
                    };
                    let a2491 = v2;
                    nat_mul(a2490, a2491)?
                };
                nat_add(a2492, a2493)?
            };
            let a2495 = v4;
            nat_le(a2494, a2495)
        }
    } else {
        false
    } {
        let a2496 = v5;
        let a2497 = 1u64;
        if nat_le(a2496, a2497) {
            Ok(true)
        } else {
            let a2510 = {
                let a2508 = {
                    let a2502 = v0;
                    let a2503 = {
                        let a2500 = {
                            let a2498 = v1;
                            let a2499 = 1u64;
                            nat_sub(a2498, a2499)
                        };
                        let a2501 = v2;
                        nat_mul(a2500, a2501)?
                    };
                    nat_add(a2502, a2503)?
                };
                let a2509 = {
                    let a2506 = {
                        let a2504 = v3;
                        let a2505 = 1u64;
                        nat_sub(a2504, a2505)
                    };
                    let a2507 = v4;
                    nat_mul(a2506, a2507)?
                };
                nat_add(a2508, a2509)?
            };
            let a2511 = v6;
            Ok(nat_le(a2510, a2511))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2512 = v5;
        let a2513 = 26u64;
        nat_eq(a2512, a2513)
    } else {
        false
    } {
        let a2514 = v3;
        let a2515 = v7;
        nat_eq(a2514, a2515)
    } else {
        false
    } {
        let a2516 = v4;
        let a2517 = v8;
        nat_eq(a2516, a2517)
    } else {
        false
    } {
        let a2518 = v9;
        let a2519 = 1u64;
        nat_eq(a2518, a2519)
    } else {
        false
    } {
        let a2520 = v10;
        let a2521 = f152(v0);
        nat_eq(a2520, a2521)
    } else {
        false
    } {
        let a2522 = v11;
        let a2523 = v1;
        nat_eq(a2522, a2523)
    } else {
        false
    } {
        let a2524 = v12;
        let a2525 = v6;
        nat_eq(a2524, a2525)
    } else {
        false
    } {
        let a2526 = v13;
        let a2527 = v7;
        nat_eq(a2526, a2527)
    } else {
        false
    } {
        let a2528 = v14;
        let a2529 = v8;
        nat_eq(a2528, a2529)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2530 = v0;
    let a2531 = 0u64;
    if if if nat_eq(a2530, a2531) {
        true
    } else {
        let a2532 = v1;
        let a2533 = 0u64;
        nat_eq(a2532, a2533)
    } {
        true
    } else {
        let a2534 = v2;
        let a2535 = 0u64;
        nat_eq(a2534, a2535)
    } {
        true
    } else {
        let a2536 = v3;
        let a2537 = 0u64;
        nat_eq(a2536, a2537)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2538 = v0;
    let a2539 = 2147483648u64;
    if nat_lt(a2538, a2539) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2540 = v0;
    let a2541 = 2147483648u64;
    if nat_lt(a2540, a2541) {
        0u64
    } else {
        let a2542 = 4294967296u64;
        let a2543 = v0;
        nat_sub(a2542, a2543)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2545 = match v0.clone() {
            Adt3::C0(h2544, _, _, _, _, _, _, _) => {
                h2544
            }
        };
        let a2546 = 1u64;
        nat_sub(a2545, a2546)
    }, {
        let a2548 = match v0.clone() {
            Adt3::C0(_, h2547, _, _, _, _, _, _) => {
                h2547
            }
        };
        let a2549 = 1u64;
        nat_sub(a2548, a2549)
    }, {
        let a2551 = match v0.clone() {
            Adt3::C0(_, _, h2550, _, _, _, _, _) => {
                h2550
            }
        };
        let a2552 = 1u64;
        nat_sub(a2551, a2552)
    }, {
        let a2554 = match v0 {
            Adt3::C0(_, _, _, h2553, _, _, _, _) => {
                h2553
            }
        };
        let a2555 = 1u64;
        nat_sub(a2554, a2555)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2556 = v0;
    let a2557 = 0u64;
    if nat_eq(a2556, a2557) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2558 = v0;
            let a2559 = 1u64;
            nat_sub(a2558, a2559)
        };
        let v5: u64 = {
            let a2560 = v4;
            let a2561 = v1;
            let a2562 = 0u64;
            nat_quot(a2560, a2561, a2562)
        };
        let v6: u64 = {
            let a2570 = {
                let a2563 = v5;
                let a2564 = v2;
                nat_mul(a2563, a2564)?
            };
            let a2571 = {
                let a2568 = {
                    let a2565 = v4;
                    let a2566 = v1;
                    let a2567 = 0u64;
                    nat_rem(a2565, a2566, a2567)
                };
                let a2569 = v3;
                nat_mul(a2568, a2569)?
            };
            nat_add(a2570, a2571)?
        };
        let a2572 = v5;
        let a2573 = 0u64;
        if nat_eq(a2572, a2573) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2582 = {
                    let a2576 = {
                        let a2574 = v5;
                        let a2575 = 1u64;
                        nat_sub(a2574, a2575)
                    };
                    let a2577 = v2;
                    nat_mul(a2576, a2577)?
                };
                let a2583 = {
                    let a2580 = {
                        let a2578 = v1;
                        let a2579 = 1u64;
                        nat_sub(a2578, a2579)
                    };
                    let a2581 = v3;
                    nat_mul(a2580, a2581)?
                };
                nat_add(a2582, a2583)?
            };
            let a2584 = v6;
            let a2585 = v7;
            if nat_lt(a2584, a2585) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2586 = v1;
    let a2587 = 0u64;
    if !nat_eq(a2586, a2587) {
        true
    } else {
        let a2589 = match v0 {
            Adt3::C0(_, _, _, _, h2588, _, _, _) => {
                h2588
            }
        };
        let a2590 = 4u64;
        nat_eq(a2589, a2590)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2593 = {
        let a2591 = v0;
        let a2592 = 2u64;
        nat_mul(a2591, a2592)?
    };
    let a2594 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2593, a2594)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2595 = f154(v0, v1);
    match m2595 {
        None => {
            if v5 {
                let m2596 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2596 {
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
                let a2597 = 4u64;
                let a2598 = v28;
                nat_le(a2597, a2598)
            };
            let a2599 = v2;
            let a2600 = 0u64;
            if if if if if nat_eq(a2599, a2600) {
                let a2601 = v4;
                let a2602 = 0u64;
                nat_eq(a2601, a2602)
            } else {
                false
            } {
                let a2603 = v6;
                let a2604 = v14;
                if nat_eq(a2603, a2604) {
                    let a2605 = v7;
                    let a2606 = v15;
                    nat_eq(a2605, a2606)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2607 = v8;
                let a2608 = v16;
                if nat_eq(a2607, a2608) {
                    let a2609 = v9;
                    let a2610 = v17;
                    nat_eq(a2609, a2610)
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
                        let a2611 = v3;
                        let a2612 = 0u64;
                        nat_eq(a2611, a2612)
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
                    let a2617 = {
                        let a2613 = v6;
                        let a2614 = v7;
                        nat_mul(a2613, a2614)?
                    };
                    let a2618 = {
                        let a2615 = v8;
                        let a2616 = v9;
                        nat_mul(a2615, a2616)?
                    };
                    nat_mul(a2617, a2618)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2619 = v0;
    let a2620 = 0u64;
    if nat_eq(a2619, a2620) {
        Adt8::C0(0u64, v1)
    } else {
        let a2621 = v0;
        let a2622 = 1u64;
        if nat_eq(a2621, a2622) {
            Adt8::C1(v1)
        } else {
            let a2623 = v0;
            let a2624 = 2u64;
            if nat_eq(a2623, a2624) {
                Adt8::C2(v1)
            } else {
                let a2625 = v0;
                let a2626 = 3u64;
                if nat_eq(a2625, a2626) {
                    Adt8::C3(v1)
                } else {
                    let a2627 = v0;
                    let a2628 = 13u64;
                    if nat_lt(a2627, a2628) {
                        Adt8::C0({
                            let a2629 = v0;
                            let a2630 = 3u64;
                            nat_sub(a2629, a2630)
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
    let a2631 = v0;
    let a2632 = 1u64;
    if nat_eq(a2631, a2632) {
        Adt9::C0(v1)
    } else {
        let a2633 = v0;
        let a2634 = 3u64;
        if if nat_eq(a2633, a2634) {
            true
        } else {
            let a2635 = v0;
            let a2636 = 6u64;
            nat_eq(a2635, a2636)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2637 = f70(v0);
    match m2637 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2639 = match v1.clone() {
                Adt11::C0(_, h2638, _) => {
                    h2638
                }
            };
            let a2640 = 1u64;
            if if nat_eq(a2639, a2640) {
                let a2642 = match v1.clone() {
                    Adt11::C0(_, _, h2641) => {
                        h2641
                    }
                };
                let a2643 = 1u64;
                if if if nat_eq(a2642, a2643) {
                    true
                } else {
                    let a2645 = match v1.clone() {
                        Adt11::C0(_, _, h2644) => {
                            h2644
                        }
                    };
                    let a2646 = 2u64;
                    nat_eq(a2645, a2646)
                } {
                    true
                } else {
                    let a2648 = match v1.clone() {
                        Adt11::C0(_, _, h2647) => {
                            h2647
                        }
                    };
                    let a2649 = 4u64;
                    nat_eq(a2648, a2649)
                } {
                    true
                } else {
                    let a2651 = match v1.clone() {
                        Adt11::C0(_, _, h2650) => {
                            h2650
                        }
                    };
                    let a2652 = 8u64;
                    nat_eq(a2651, a2652)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2653) => {
                        h2653
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2666 = {
        let a2662 = {
            let a2658 = {
                let a2654 = v0;
                let a2655 = v4;
                nat_mul(a2654, a2655)?
            };
            let a2659 = {
                let a2656 = v1;
                let a2657 = v5;
                nat_mul(a2656, a2657)?
            };
            nat_add(a2658, a2659)?
        };
        let a2663 = {
            let a2660 = v2;
            let a2661 = v6;
            nat_mul(a2660, a2661)?
        };
        nat_add(a2662, a2663)?
    };
    let a2667 = {
        let a2664 = v3;
        let a2665 = v7;
        nat_mul(a2664, a2665)?
    };
    nat_add(a2666, a2667)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2672 = {
        let a2670 = {
            let a2668 = v0;
            let a2669 = v1;
            nat_sub(a2668, a2669)
        };
        let a2671 = 1u64;
        nat_sub(a2670, a2671)
    };
    let a2673 = v2;
    nat_add(a2672, a2673)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2674 = v0;
        let a2675 = v4;
        let a2676 = 0u64;
        nat_rem(a2674, a2675, a2676)
    }, {
        let a2677 = v1;
        let a2678 = v5;
        let a2679 = 0u64;
        nat_rem(a2677, a2678, a2679)
    }, {
        let a2680 = v2;
        let a2681 = v6;
        let a2682 = 0u64;
        nat_rem(a2680, a2681, a2682)
    }, {
        let a2683 = v3;
        let a2684 = v7;
        let a2685 = 0u64;
        nat_rem(a2683, a2684, a2685)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2687 = v0;
        let a2688 = match v1.clone() {
            Adt3::C0(h2686, _, _, _, _, _, _, _) => {
                h2686
            }
        };
        let a2689 = 0u64;
        nat_quot(a2687, a2688, a2689)
    };
    let v4: u64 = {
        let a2691 = v3;
        let a2692 = match v1.clone() {
            Adt3::C0(_, h2690, _, _, _, _, _, _) => {
                h2690
            }
        };
        let a2693 = 0u64;
        nat_quot(a2691, a2692, a2693)
    };
    let a2694 = v2;
    let a2695 = 0u64;
    if nat_eq(a2694, a2695) {
        let a2697 = v0;
        let a2698 = match v1 {
            Adt3::C0(h2696, _, _, _, _, _, _, _) => {
                h2696
            }
        };
        let a2699 = 0u64;
        nat_rem(a2697, a2698, a2699)
    } else {
        let a2700 = v2;
        let a2701 = 1u64;
        if nat_eq(a2700, a2701) {
            let a2703 = v3;
            let a2704 = match v1 {
                Adt3::C0(_, h2702, _, _, _, _, _, _) => {
                    h2702
                }
            };
            let a2705 = 0u64;
            nat_rem(a2703, a2704, a2705)
        } else {
            let a2706 = v2;
            let a2707 = 2u64;
            if nat_eq(a2706, a2707) {
                let a2709 = v4;
                let a2710 = match v1 {
                    Adt3::C0(_, _, h2708, _, _, _, _, _) => {
                        h2708
                    }
                };
                let a2711 = 0u64;
                nat_rem(a2709, a2710, a2711)
            } else {
                let a2713 = v4;
                let a2714 = match v1 {
                    Adt3::C0(_, _, h2712, _, _, _, _, _) => {
                        h2712
                    }
                };
                let a2715 = 0u64;
                nat_quot(a2713, a2714, a2715)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2716, _, _, _) => {
            h2716
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2717, _, _) => {
            h2717
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2718, _) => {
            h2718
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2719) => {
            h2719
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2720 = v1;
    let a2721 = 2147483648u64;
    if nat_lt(a2720, a2721) {
        let a2732 = {
            let a2730 = {
                let a2725 = {
                    let a2722 = v0;
                    let a2723 = v2;
                    let a2724 = 0u64;
                    nat_rem(a2722, a2723, a2724)
                };
                let a2726 = v2;
                nat_add(a2725, a2726)?
            };
            let a2731 = {
                let a2727 = v1;
                let a2728 = v2;
                let a2729 = 0u64;
                nat_rem(a2727, a2728, a2729)
            };
            nat_sub(a2730, a2731)
        };
        let a2733 = v2;
        let a2734 = 0u64;
        Ok(nat_rem(a2732, a2733, a2734))
    } else {
        let a2737 = {
            let a2735 = v0;
            let a2736 = f120(v1);
            nat_add(a2735, a2736)?
        };
        let a2738 = v2;
        let a2739 = 0u64;
        Ok(nat_rem(a2737, a2738, a2739))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2742 = f119(v1);
    let a2743 = {
        let a2740 = v0;
        let a2741 = f120(v1);
        nat_add(a2740, a2741)?
    };
    if nat_le(a2742, a2743) {
        let a2744 = f136(v0, v1)?;
        let a2745 = v2;
        Ok(nat_lt(a2744, a2745))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2748 = {
        let a2746 = v0;
        let a2747 = f120(v1);
        nat_add(a2746, a2747)?
    };
    let a2749 = f119(v1);
    Ok(nat_sub(a2748, a2749))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2750 = v0;
    let a2751 = v1;
    if nat_lt(a2750, a2751) {
        let a2752 = v1;
        let a2753 = v0;
        Ok(nat_sub(a2752, a2753))
    } else {
        let a2756 = v0;
        let a2757 = {
            let a2754 = v1;
            let a2755 = v2;
            nat_add(a2754, a2755)?
        };
        if nat_lt(a2756, a2757) {
            let a2758 = v0;
            let a2759 = v1;
            Ok(nat_sub(a2758, a2759))
        } else {
            let a2766 = {
                let a2762 = 2u64;
                let a2763 = {
                    let a2760 = v2;
                    let a2761 = 1u64;
                    nat_sub(a2760, a2761)
                };
                nat_mul(a2762, a2763)?
            };
            let a2767 = {
                let a2764 = v0;
                let a2765 = v1;
                nat_sub(a2764, a2765)
            };
            Ok(nat_sub(a2766, a2767))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2770 = {
        let a2768 = v2;
        let a2769 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2768, a2769)?
    };
    let a2771 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2770, a2771))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2772 = v0;
    let a2773 = 0u64;
    if nat_eq(a2772, a2773) {
        let a2774 = v2;
        let a2775 = v1;
        nat_le(a2774, a2775)
    } else {
        let a2776 = v0;
        let a2777 = 1u64;
        if nat_eq(a2776, a2777) {
            let a2778 = v2;
            let a2779 = v1;
            nat_lt(a2778, a2779)
        } else {
            let a2780 = v0;
            let a2781 = 2u64;
            if nat_eq(a2780, a2781) {
                let a2782 = v1;
                let a2783 = v2;
                nat_le(a2782, a2783)
            } else {
                let a2784 = v0;
                let a2785 = 3u64;
                if nat_eq(a2784, a2785) {
                    let a2786 = v1;
                    let a2787 = v2;
                    nat_lt(a2786, a2787)
                } else {
                    let a2788 = v1;
                    let a2789 = v2;
                    nat_eq(a2788, a2789)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2790 = v0;
    let a2791 = 0u64;
    if nat_eq(a2790, a2791) {
        Ok(f148(0u64))
    } else {
        let a2794 = {
            let a2792 = v0;
            let a2793 = 1u64;
            nat_add(a2792, a2793)?
        };
        let a2795 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2794, a2795) {
            Ok(Adt5::C4({
                let a2796 = v2;
                let a2797 = 0u64;
                if nat_eq(a2796, a2797) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2798 = v0;
                let a2799 = 1u64;
                nat_sub(a2798, a2799)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2800, _, _, _, _, _, _, _) => {
                        h2800
                    }
                }, false)?;
                let v5: u64 = {
                    let a2801 = v3;
                    let a2802 = v4;
                    let a2803 = 0u64;
                    nat_quot(a2801, a2802, a2803)
                };
                let v6: u64 = {
                    let a2804 = v3;
                    let a2805 = v4;
                    let a2806 = 0u64;
                    nat_rem(a2804, a2805, a2806)
                };
                let a2809 = {
                    let a2807 = v6;
                    let a2808 = 1u64;
                    nat_add(a2807, a2808)?
                };
                let a2810 = v4;
                if nat_eq(a2809, a2810) {
                    Ok(Adt5::C4({
                        let a2811 = v2;
                        let a2812 = 0u64;
                        if nat_eq(a2811, a2812) {
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
                    let a2813 = v3;
                    let a2814 = 2u64;
                    let a2815 = 0u64;
                    nat_quot(a2813, a2814, a2815)
                }, {
                    let a2816 = v3;
                    let a2817 = 2u64;
                    let a2818 = 0u64;
                    nat_rem(a2816, a2817, a2818)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2819 = v0;
    let a2820 = 0u64;
    if nat_eq(a2819, a2820) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2829 = {
                let a2822 = v1;
                let a2823 = match v2.clone() {
                    Adt3::C0(h2821, _, _, _, _, _, _, _) => {
                        h2821
                    }
                };
                nat_mul(a2822, a2823)?
            };
            let a2830 = {
                let a2826 = {
                    let a2824 = v0;
                    let a2825 = 1u64;
                    nat_sub(a2824, a2825)
                };
                let a2827 = 2u64;
                let a2828 = 0u64;
                nat_quot(a2826, a2827, a2828)
            };
            nat_add(a2829, a2830)?
        }, {
            let a2833 = {
                let a2831 = v0;
                let a2832 = 1u64;
                nat_sub(a2831, a2832)
            };
            let a2834 = 2u64;
            let a2835 = 0u64;
            nat_rem(a2833, a2834, a2835)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2836 = v0;
    let a2837 = f60(v1)?;
    let a2838 = 0u64;
    Ok(nat_rem(a2836, a2837, a2838))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2839 = 4u64;
    let a2840 = f157(v0, v1)?;
    nat_mul(a2839, a2840)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2841 = 4u64;
    let a2842 = f158(v0, v1)?;
    nat_mul(a2841, a2842)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2848 = {
        let a2845 = {
            let a2843 = f142(v0, v1)?;
            let a2844 = 2u64;
            nat_sub(a2843, a2844)
        };
        let a2846 = 2u64;
        let a2847 = 0u64;
        nat_rem(a2845, a2846, a2847)
    };
    let a2849 = 0u64;
    Ok(nat_eq(a2848, a2849))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2850 = f70(v0);
    match m2850 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2851 = v1;
                let a2852 = v7;
                nat_lt(a2851, a2852)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2853 = v3;
                let a2854 = f160(v4, v5, v6)?;
                nat_lt(a2853, a2854)
            } else {
                false
            } {
                let a2859 = {
                    let a2856 = v7;
                    let a2857 = match v13.clone() {
                        Adt11::C0(_, h2855, _) => {
                            h2855
                        }
                    };
                    let a2858 = 0u64;
                    nat_rem(a2856, a2857, a2858)
                };
                let a2860 = 0u64;
                nat_eq(a2859, a2860)
            } else {
                false
            } {
                Ok(Some({
                    let a2889 = {
                        let a2879 = {
                            let a2869 = {
                                let a2865 = {
                                    let a2862 = v1;
                                    let a2863 = match v13 {
                                        Adt11::C0(_, h2861, _) => {
                                            h2861
                                        }
                                    };
                                    let a2864 = 0u64;
                                    nat_quot(a2862, a2863, a2864)
                                };
                                let a2866 = v9;
                                nat_mul(a2865, a2866)?
                            };
                            let a2870 = {
                                let a2867 = v2;
                                let a2868 = v10;
                                nat_mul(a2867, a2868)?
                            };
                            nat_add(a2869, a2870)?
                        };
                        let a2880 = {
                            let a2877 = {
                                let a2874 = {
                                    let a2871 = v3;
                                    let a2872 = v4;
                                    let a2873 = 0u64;
                                    nat_quot(a2871, a2872, a2873)
                                };
                                let a2875 = v5;
                                let a2876 = 0u64;
                                nat_rem(a2874, a2875, a2876)
                            };
                            let a2878 = v11;
                            nat_mul(a2877, a2878)?
                        };
                        nat_add(a2879, a2880)?
                    };
                    let a2890 = {
                        let a2887 = {
                            let a2884 = {
                                let a2881 = v3;
                                let a2882 = v4;
                                let a2883 = 0u64;
                                nat_quot(a2881, a2882, a2883)
                            };
                            let a2885 = v5;
                            let a2886 = 0u64;
                            nat_quot(a2884, a2885, a2886)
                        };
                        let a2888 = v12;
                        nat_mul(a2887, a2888)?
                    };
                    nat_add(a2889, a2890)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2891 = v0;
    let a2892 = 30u64;
    if nat_eq(a2891, a2892) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2893 = 0u64;
    let a2894 = v0;
    if nat_lt(a2893, a2894) {
        let a2895 = v0;
        let a2896 = 2139095040u64;
        nat_le(a2895, a2896)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2897 = v0;
    let a2898 = 40u64;
    if nat_lt(a2897, a2898) {
        let a2899 = v0;
        let a2900 = 20u64;
        if nat_lt(a2899, a2900) {
            let a2901 = v0;
            let a2902 = 10u64;
            if nat_lt(a2901, a2902) {
                let a2903 = v0;
                let a2904 = 5u64;
                if nat_lt(a2903, a2904) {
                    let a2905 = v0;
                    let a2906 = 2u64;
                    if nat_lt(a2905, a2906) {
                        let a2907 = v0;
                        let a2908 = 1u64;
                        if nat_lt(a2907, a2908) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2909 = v0;
                        let a2910 = 3u64;
                        if nat_lt(a2909, a2910) {
                            4u64
                        } else {
                            let a2911 = v0;
                            let a2912 = 4u64;
                            if nat_lt(a2911, a2912) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2913 = v0;
                    let a2914 = 7u64;
                    if nat_lt(a2913, a2914) {
                        let a2915 = v0;
                        let a2916 = 6u64;
                        if nat_lt(a2915, a2916) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2917 = v0;
                        let a2918 = 8u64;
                        if nat_lt(a2917, a2918) {
                            128u64
                        } else {
                            let a2919 = v0;
                            let a2920 = 9u64;
                            if nat_lt(a2919, a2920) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2921 = v0;
                let a2922 = 15u64;
                if nat_lt(a2921, a2922) {
                    let a2923 = v0;
                    let a2924 = 12u64;
                    if nat_lt(a2923, a2924) {
                        let a2925 = v0;
                        let a2926 = 11u64;
                        if nat_lt(a2925, a2926) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2927 = v0;
                        let a2928 = 13u64;
                        if nat_lt(a2927, a2928) {
                            4096u64
                        } else {
                            let a2929 = v0;
                            let a2930 = 14u64;
                            if nat_lt(a2929, a2930) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2931 = v0;
                    let a2932 = 17u64;
                    if nat_lt(a2931, a2932) {
                        let a2933 = v0;
                        let a2934 = 16u64;
                        if nat_lt(a2933, a2934) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2935 = v0;
                        let a2936 = 18u64;
                        if nat_lt(a2935, a2936) {
                            131072u64
                        } else {
                            let a2937 = v0;
                            let a2938 = 19u64;
                            if nat_lt(a2937, a2938) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2939 = v0;
            let a2940 = 30u64;
            if nat_lt(a2939, a2940) {
                let a2941 = v0;
                let a2942 = 25u64;
                if nat_lt(a2941, a2942) {
                    let a2943 = v0;
                    let a2944 = 22u64;
                    if nat_lt(a2943, a2944) {
                        let a2945 = v0;
                        let a2946 = 21u64;
                        if nat_lt(a2945, a2946) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2947 = v0;
                        let a2948 = 23u64;
                        if nat_lt(a2947, a2948) {
                            4194304u64
                        } else {
                            let a2949 = v0;
                            let a2950 = 24u64;
                            if nat_lt(a2949, a2950) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2951 = v0;
                    let a2952 = 27u64;
                    if nat_lt(a2951, a2952) {
                        let a2953 = v0;
                        let a2954 = 26u64;
                        if nat_lt(a2953, a2954) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2955 = v0;
                        let a2956 = 28u64;
                        if nat_lt(a2955, a2956) {
                            134217728u64
                        } else {
                            let a2957 = v0;
                            let a2958 = 29u64;
                            if nat_lt(a2957, a2958) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2959 = v0;
                let a2960 = 35u64;
                if nat_lt(a2959, a2960) {
                    let a2961 = v0;
                    let a2962 = 32u64;
                    if nat_lt(a2961, a2962) {
                        let a2963 = v0;
                        let a2964 = 31u64;
                        if nat_lt(a2963, a2964) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2965 = v0;
                        let a2966 = 33u64;
                        if nat_lt(a2965, a2966) {
                            4294967296u64
                        } else {
                            let a2967 = v0;
                            let a2968 = 34u64;
                            if nat_lt(a2967, a2968) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2969 = v0;
                    let a2970 = 37u64;
                    if nat_lt(a2969, a2970) {
                        let a2971 = v0;
                        let a2972 = 36u64;
                        if nat_lt(a2971, a2972) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2973 = v0;
                        let a2974 = 38u64;
                        if nat_lt(a2973, a2974) {
                            137438953472u64
                        } else {
                            let a2975 = v0;
                            let a2976 = 39u64;
                            if nat_lt(a2975, a2976) {
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
    let a2977 = v0;
    let a2978 = 26u64;
    if nat_eq(a2977, a2978) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2999 = {
        let a2993 = {
            let a2987 = {
                let a2981 = v1;
                let a2982 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2979, _, _, _) => {
                            h2979
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2980, _, _, _) => {
                            h2980
                        }
                    })
                };
                nat_mul(a2981, a2982)?
            };
            let a2988 = {
                let a2985 = v2;
                let a2986 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2983, _, _) => {
                            h2983
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2984, _, _) => {
                            h2984
                        }
                    })
                };
                nat_mul(a2985, a2986)?
            };
            nat_add(a2987, a2988)?
        };
        let a2994 = {
            let a2991 = v3;
            let a2992 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2989, _) => {
                        h2989
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2990, _) => {
                        h2990
                    }
                })
            };
            nat_mul(a2991, a2992)?
        };
        nat_add(a2993, a2994)?
    };
    let a3000 = {
        let a2997 = v4;
        let a2998 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2995) => {
                    h2995
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2996) => {
                    h2996
                }
            })
        };
        nat_mul(a2997, a2998)?
    };
    nat_add(a2999, a3000)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a3001 = v0;
    let a3002 = 2u64;
    if nat_eq(a3001, a3002) {
        Some(0u64)
    } else {
        let a3003 = v0;
        let a3004 = 5u64;
        if nat_eq(a3003, a3004) {
            Some(1u64)
        } else {
            let a3005 = v0;
            let a3006 = 6u64;
            if nat_eq(a3005, a3006) {
                Some(2u64)
            } else {
                let a3007 = v0;
                let a3008 = 7u64;
                if nat_eq(a3007, a3008) {
                    Some(3u64)
                } else {
                    let a3009 = v0;
                    let a3010 = 9u64;
                    if nat_eq(a3009, a3010) {
                        Some(4u64)
                    } else {
                        let a3011 = v0;
                        let a3012 = 8u64;
                        if nat_eq(a3011, a3012) {
                            Some(5u64)
                        } else {
                            let a3013 = v0;
                            let a3014 = 89u64;
                            if nat_eq(a3013, a3014) {
                                let a3015 = v1;
                                let a3016 = 2u64;
                                if nat_eq(a3015, a3016) {
                                    Some(6u64)
                                } else {
                                    let a3017 = v1;
                                    let a3018 = 0u64;
                                    if nat_eq(a3017, a3018) {
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
    let a3019 = v0;
    let a3020 = 28u64;
    if if if if nat_eq(a3019, a3020) {
        let a3021 = v1;
        let a3022 = 0u64;
        nat_eq(a3021, a3022)
    } else {
        false
    } {
        let a3023 = v2;
        let a3024 = 0u64;
        nat_eq(a3023, a3024)
    } else {
        false
    } {
        let a3025 = v3;
        let a3026 = 0u64;
        nat_eq(a3025, a3026)
    } else {
        false
    } {
        let a3027 = v4;
        let a3028 = v8;
        if if if if if if if if nat_eq(a3027, a3028) {
            let a3029 = v5;
            let a3030 = v12;
            nat_eq(a3029, a3030)
        } else {
            false
        } {
            let a3031 = v9;
            let a3032 = v13;
            nat_eq(a3031, a3032)
        } else {
            false
        } {
            let a3033 = v10;
            let a3034 = v14;
            nat_eq(a3033, a3034)
        } else {
            false
        } {
            let a3035 = v11;
            let a3036 = v15;
            nat_eq(a3035, a3036)
        } else {
            false
        } {
            let a3037 = 0u64;
            let a3038 = v6;
            nat_lt(a3037, a3038)
        } else {
            false
        } {
            let a3039 = 0u64;
            let a3040 = v7;
            nat_lt(a3039, a3040)
        } else {
            false
        } {
            let a3044 = {
                let a3041 = v10;
                let a3042 = v6;
                let a3043 = 1u64;
                nat_rem(a3041, a3042, a3043)
            };
            let a3045 = 0u64;
            if nat_eq(a3044, a3045) {
                let a3049 = {
                    let a3046 = v11;
                    let a3047 = v7;
                    let a3048 = 1u64;
                    nat_rem(a3046, a3047, a3048)
                };
                let a3050 = 0u64;
                nat_eq(a3049, a3050)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3051 = v10;
                let a3052 = v6;
                let a3053 = 0u64;
                nat_quot(a3051, a3052, a3053)
            }, {
                let a3054 = v11;
                let a3055 = v7;
                let a3056 = 0u64;
                nat_quot(a3054, a3055, a3056)
            }, {
                let a3057 = v5;
                let a3058 = v4;
                nat_mul(a3057, a3058)?
            }, {
                let a3059 = v4;
                let a3060 = v9;
                nat_mul(a3059, a3060)?
            }, {
                let a3061 = v5;
                let a3062 = v9;
                nat_mul(a3061, a3062)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3063 = v1;
    let a3064 = 0u64;
    if nat_eq(a3063, a3064) {
        Ok(Adt5::C4({
            let a3065 = v3;
            let a3066 = 0u64;
            if nat_eq(a3065, a3066) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3067 = v3;
            let a3068 = 0u64;
            if nat_eq(a3067, a3068) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3069 = v0;
    let a3070 = f60(v1)?;
    let a3071 = 0u64;
    Ok(nat_quot(a3069, a3070, a3071))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3074 = {
            let a3072 = f142(v0, v1)?;
            let a3073 = 2u64;
            nat_sub(a3072, a3073)
        };
        let a3075 = 2u64;
        let a3076 = 0u64;
        nat_quot(a3074, a3075, a3076)
    };
    let v4: u64 = {
        let a3077 = v2;
        let a3078 = v1;
        nat_mul(a3077, a3078)?
    };
    if f145(v0, v1)? {
        let a3079 = v4;
        let a3080 = v3;
        nat_add(a3079, a3080)
    } else {
        let a3092 = {
            let a3088 = v4;
            let a3089 = {
                let a3086 = {
                    let a3081 = v1;
                    let a3082 = 1u64;
                    nat_sub(a3081, a3082)
                };
                let a3087 = {
                    let a3083 = v2;
                    let a3084 = v1;
                    let a3085 = 0u64;
                    nat_rem(a3083, a3084, a3085)
                };
                nat_mul(a3086, a3087)?
            };
            nat_sub(a3088, a3089)
        };
        let a3093 = {
            let a3090 = v3;
            let a3091 = v1;
            nat_mul(a3090, a3091)?
        };
        nat_add(a3092, a3093)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3094 = v0;
    let a3095 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3094, a3095) {
        true
    } else {
        let a3096 = v0;
        let a3097 = 1u64;
        nat_eq(a3096, a3097)
    } {
        true
    } else {
        let a3098 = v0;
        let a3099 = 2u64;
        nat_eq(a3098, a3099)
    } {
        true
    } else {
        let a3100 = v0;
        let a3101 = 3u64;
        nat_eq(a3100, a3101)
    } {
        true
    } else {
        let a3102 = v0;
        let a3103 = 6u64;
        nat_eq(a3102, a3103)
    } {
        true
    } else {
        let a3104 = v0;
        let a3105 = 7u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 8u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 10u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 11u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 12u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 13u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 14u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 16u64;
        nat_eq(a3118, a3119)
    } {
        true
    } else {
        let a3120 = v0;
        let a3121 = 17u64;
        nat_eq(a3120, a3121)
    } {
        true
    } else {
        let a3122 = v0;
        let a3123 = 18u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v0;
        let a3125 = 19u64;
        nat_eq(a3124, a3125)
    } {
        true
    } else {
        let a3126 = v0;
        let a3127 = 20u64;
        nat_eq(a3126, a3127)
    } {
        true
    } else {
        let a3128 = v0;
        let a3129 = 21u64;
        nat_eq(a3128, a3129)
    } {
        true
    } else {
        let a3130 = v0;
        let a3131 = 22u64;
        nat_eq(a3130, a3131)
    } {
        true
    } else {
        let a3132 = v0;
        let a3133 = 23u64;
        nat_eq(a3132, a3133)
    } {
        true
    } else {
        let a3134 = v0;
        let a3135 = 26u64;
        nat_eq(a3134, a3135)
    } {
        true
    } else {
        let a3136 = v0;
        let a3137 = 29u64;
        nat_eq(a3136, a3137)
    } {
        true
    } else {
        let a3138 = v0;
        let a3139 = 30u64;
        nat_eq(a3138, a3139)
    } {
        true
    } else {
        let a3140 = v0;
        let a3141 = 34u64;
        nat_eq(a3140, a3141)
    } {
        true
    } else {
        let a3142 = v0;
        let a3143 = 35u64;
        nat_eq(a3142, a3143)
    } {
        true
    } else {
        let a3144 = v0;
        let a3145 = 39u64;
        nat_eq(a3144, a3145)
    } {
        true
    } else {
        let a3146 = v0;
        let a3147 = 40u64;
        nat_eq(a3146, a3147)
    } {
        true
    } else {
        let a3148 = v0;
        let a3149 = 41u64;
        nat_eq(a3148, a3149)
    } {
        true
    } else {
        let a3150 = v0;
        let a3151 = 42u64;
        nat_eq(a3150, a3151)
    } {
        true
    } else {
        let a3152 = v0;
        let a3153 = 43u64;
        nat_eq(a3152, a3153)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3154 = v0;
    let a3155 = 0u64;
    if if if nat_eq(a3154, a3155) {
        true
    } else {
        let a3156 = v1;
        let a3157 = 0u64;
        nat_eq(a3156, a3157)
    } {
        true
    } else {
        let a3158 = v2;
        let a3159 = 0u64;
        nat_eq(a3158, a3159)
    } {
        Ok(0u64)
    } else {
        let a3162 = {
            let a3160 = v0;
            let a3161 = v1;
            nat_mul(a3160, a3161)?
        };
        let a3163 = v2;
        nat_mul(a3162, a3163)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3164 = f70(v0);
    match m3164 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
