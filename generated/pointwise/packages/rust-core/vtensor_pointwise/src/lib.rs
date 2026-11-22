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
            f68(v0, v3, v2)
        }
    }
}

pub fn f33(v0: u64) -> bool {
    let a570 = v0;
    let a571 = 1u64;
    if if nat_eq(a570, a571) {
        true
    } else {
        let a572 = v0;
        let a573 = 33u64;
        nat_eq(a572, a573)
    } {
        true
    } else {
        let a574 = v0;
        let a575 = 34u64;
        nat_eq(a574, a575)
    }
}

pub fn f34(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a578 = match v1.clone() {
        Adt2::C0(h576, _, _, _, _, _, _, _, _, _, _) => {
            h576
        }
    };
    let a579 = match v3.clone() {
        Adt2::C0(h577, _, _, _, _, _, _, _, _, _, _) => {
            h577
        }
    };
    if nat_eq(a578, a579) {
        f69(v0, v1, v2, v3)
    } else {
        if if if if f70(match v1.clone() {
            Adt2::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                h580
            }
        }) {
            f70(match v3.clone() {
                Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                    h581
                }
            })
        } else {
            false
        } {
            let a582 = v0;
            let a583 = 33u64;
            if nat_eq(a582, a583) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a584 = f9(v1.clone(), 1u64)?;
            let a585 = f9(v3.clone(), 1u64)?;
            nat_eq(a584, a585)
        } else {
            false
        } {
            let m587 = f71(match v3.clone() {
                Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                    h586
                }
            });
            match m587 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h588) => {
                            h588
                        }
                    }, f13(v1.clone(), 1u64), f13(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                            h589
                        }
                    }, match v3 {
                        Adt2::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                            h590
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
    let a592 = match v0.clone() {
        Adt2::C0(h591, _, _, _, _, _, _, _, _, _, _) => {
            h591
        }
    };
    let a593 = 0u64;
    if if if if nat_eq(a592, a593) {
        true
    } else {
        let a595 = match v0.clone() {
            Adt2::C0(h594, _, _, _, _, _, _, _, _, _, _) => {
                h594
            }
        };
        let a596 = 1u64;
        nat_eq(a595, a596)
    } {
        true
    } else {
        let a598 = match v0.clone() {
            Adt2::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a599 = 25u64;
        if nat_eq(a598, a599) {
            true
        } else {
            let a601 = match v0.clone() {
                Adt2::C0(h600, _, _, _, _, _, _, _, _, _, _) => {
                    h600
                }
            };
            let a602 = 26u64;
            nat_eq(a601, a602)
        }
    } {
        true
    } else {
        let a604 = match v0.clone() {
            Adt2::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
                h603
            }
        };
        let a605 = 30u64;
        nat_eq(a604, a605)
    } {
        let a608 = match v0.clone() {
            Adt2::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a609 = match v1.clone() {
            Adt2::C0(h607, _, _, _, _, _, _, _, _, _, _) => {
                h607
            }
        };
        if if nat_eq(a608, a609) {
            f72(match v1.clone() {
                Adt2::C0(_, h610, _, _, _, _, _, _, _, _, _) => {
                    h610
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                    h611
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h612, _, _, _, _, _, _, _) => {
                    h612
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h613, _, _, _, _, _, _) => {
                    h613
                }
            }, match v0.clone() {
                Adt2::C0(_, h614, _, _, _, _, _, _, _, _, _) => {
                    h614
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h615, _, _, _, _, _, _, _, _) => {
                    h615
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h616, _, _, _, _, _, _, _) => {
                    h616
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h617, _, _, _, _, _, _) => {
                    h617
                }
            })
        } else {
            false
        } {
            let m619 = f71(match v0.clone() {
                Adt2::C0(h618, _, _, _, _, _, _, _, _, _, _) => {
                    h618
                }
            });
            match m619 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f9(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h620) => {
                            h620
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
    let m621 = v2;
    match m621 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a622 = v0;
            let a623 = 4u64;
            if if if nat_lt(a622, a623) {
                let a626 = match v1.clone() {
                    Adt2::C0(h624, _, _, _, _, _, _, _, _, _, _) => {
                        h624
                    }
                };
                let a627 = match v4.clone() {
                    Adt2::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                        h625
                    }
                };
                if nat_eq(a626, a627) {
                    let a630 = match v1.clone() {
                        Adt2::C0(h628, _, _, _, _, _, _, _, _, _, _) => {
                            h628
                        }
                    };
                    let a631 = match v3.clone() {
                        Adt2::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                            h629
                        }
                    };
                    nat_eq(a630, a631)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f73(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h632, _, _, _, _, _, _, _, _, _) => {
                        h632
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h633, _, _, _, _, _, _, _, _, _) => {
                        h633
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h634, _, _, _, _, _, _, _, _, _) => {
                        h634
                    }
                })? {
                    f73(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h635, _, _, _, _, _, _, _, _) => {
                            h635
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h636, _, _, _, _, _, _, _, _) => {
                            h636
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h637, _, _, _, _, _, _, _, _) => {
                            h637
                        }
                    })?
                } else {
                    false
                } {
                    f73(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h638, _, _, _, _, _, _, _) => {
                            h638
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h639, _, _, _, _, _, _, _) => {
                            h639
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                            h640
                        }
                    })?
                } else {
                    false
                } {
                    f73(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h641, _, _, _, _, _, _) => {
                            h641
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h642, _, _, _, _, _, _) => {
                            h642
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h643, _, _, _, _, _, _) => {
                            h643
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m645 = f71(match v1.clone() {
                    Adt2::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                });
                match m645 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h646, _) => {
                                h646
                            }
                        };
                        let a651 = {
                            let a648 = match v1.clone() {
                                Adt2::C0(_, h647, _, _, _, _, _, _, _, _, _) => {
                                    h647
                                }
                            };
                            let a649 = v6;
                            let a650 = 1u64;
                            nat_rem(a648, a649, a650)
                        };
                        let a652 = 0u64;
                        if if if nat_eq(a651, a652) {
                            let a657 = {
                                let a654 = match v4.clone() {
                                    Adt2::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
                                        h653
                                    }
                                };
                                let a655 = v6;
                                let a656 = 1u64;
                                nat_rem(a654, a655, a656)
                            };
                            let a658 = 0u64;
                            nat_eq(a657, a658)
                        } else {
                            false
                        } {
                            let a663 = {
                                let a660 = match v3.clone() {
                                    Adt2::C0(_, h659, _, _, _, _, _, _, _, _, _) => {
                                        h659
                                    }
                                };
                                let a661 = v6;
                                let a662 = 1u64;
                                nat_rem(a660, a661, a662)
                            };
                            let a664 = 0u64;
                            nat_eq(a663, a664)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f9(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h665) => {
                                    h665
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
    let a667 = match v2.clone() {
        Adt2::C0(h666, _, _, _, _, _, _, _, _, _, _) => {
            h666
        }
    };
    let a668 = 0u64;
    if if nat_eq(a667, a668) {
        let a670 = match v3.clone() {
            Adt2::C0(h669, _, _, _, _, _, _, _, _, _, _) => {
                h669
            }
        };
        let a671 = 0u64;
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a672 = v0;
        let a673 = 61u64;
        if nat_eq(a672, a673) {
            let v4: bool = {
                let a675 = 0u64;
                let a676 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h674, _, _, _, _, _, _, _) => {
                        h674
                    }
                };
                nat_lt(a675, a676)
            };
            if if if if if if f74(match v2.clone() {
                Adt2::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
                    h677
                }
            }, match v1.clone() {
                Adt1::C0(h678, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h678
                }
            }, match v1.clone() {
                Adt1::C0(_, h679, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h679
                }
            }, match v3.clone() {
                Adt2::C0(_, h680, _, _, _, _, _, _, _, _, _) => {
                    h680
                }
            })? {
                f74(match v2.clone() {
                    Adt2::C0(_, _, h681, _, _, _, _, _, _, _, _) => {
                        h681
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h682, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h682
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h683, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h683
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h684, _, _, _, _, _, _, _, _) => {
                        h684
                    }
                })?
            } else {
                false
            } {
                f74(match v2.clone() {
                    Adt2::C0(_, _, _, h685, _, _, _, _, _, _, _) => {
                        h685
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h686, _, _, _, _, _, _, _, _, _, _, _) => {
                        h686
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h687, _, _, _, _, _, _, _, _, _, _) => {
                        h687
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h688, _, _, _, _, _, _, _) => {
                        h688
                    }
                })?
            } else {
                false
            } {
                f74(match v2.clone() {
                    Adt2::C0(_, _, _, _, h689, _, _, _, _, _, _) => {
                        h689
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h690, _, _, _, _, _, _, _, _, _) => {
                        h690
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h691, _, _, _, _, _, _, _, _) => {
                        h691
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h692, _, _, _, _, _, _) => {
                        h692
                    }
                })?
            } else {
                false
            } {
                let a694 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h693, _, _, _, _, _, _, _) => {
                        h693
                    }
                };
                let a695 = 4294967295u64;
                nat_le(a694, a695)
            } else {
                false
            } {
                if v4 {
                    !f23(match v2.clone() {
                        Adt2::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
                            h696
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
                            h697
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h698, _, _, _, _, _, _, _) => {
                            h698
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h699, _, _, _, _, _, _) => {
                            h699
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h700, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h700
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h701, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h701
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h702, _, _, _, _, _, _, _, _, _, _, _) => {
                        h702
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h703, _, _, _, _, _, _, _, _, _) => {
                        h703
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a704 = v0;
            let a705 = 62u64;
            if nat_eq(a704, a705) {
                let a707 = match v1.clone() {
                    Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                };
                let a708 = 2147483648u64;
                if if if if if nat_lt(a707, a708) {
                    let a710 = match v1.clone() {
                        Adt1::C0(_, h709, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h709
                        }
                    };
                    let a711 = 2147483648u64;
                    nat_lt(a710, a711)
                } else {
                    false
                } {
                    let a714 = match v1.clone() {
                        Adt1::C0(h712, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h712
                        }
                    };
                    let a715 = match v2.clone() {
                        Adt2::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                            h713
                        }
                    };
                    nat_lt(a714, a715)
                } else {
                    false
                } {
                    let a718 = match v1.clone() {
                        Adt1::C0(_, h716, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h716
                        }
                    };
                    let a719 = match v2.clone() {
                        Adt2::C0(_, h717, _, _, _, _, _, _, _, _, _) => {
                            h717
                        }
                    };
                    nat_lt(a718, a719)
                } else {
                    false
                } {
                    let a722 = match v2.clone() {
                        Adt2::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
                            h720
                        }
                    };
                    let a723 = match v3.clone() {
                        Adt2::C0(_, _, h721, _, _, _, _, _, _, _, _) => {
                            h721
                        }
                    };
                    if if if nat_eq(a722, a723) {
                        let a726 = match v2.clone() {
                            Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
                                h724
                            }
                        };
                        let a727 = match v3.clone() {
                            Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
                                h725
                            }
                        };
                        nat_eq(a726, a727)
                    } else {
                        false
                    } {
                        let a730 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h728, _, _, _, _, _, _) => {
                                h728
                            }
                        };
                        let a731 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                                h729
                            }
                        };
                        nat_eq(a730, a731)
                    } else {
                        false
                    } {
                        let a740 = match v3.clone() {
                            Adt2::C0(_, h732, _, _, _, _, _, _, _, _, _) => {
                                h732
                            }
                        };
                        let a741 = {
                            let a738 = {
                                let a735 = match v2.clone() {
                                    Adt2::C0(_, h733, _, _, _, _, _, _, _, _, _) => {
                                        h733
                                    }
                                };
                                let a736 = match v1.clone() {
                                    Adt1::C0(h734, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h734
                                    }
                                };
                                nat_add(a735, a736)?
                            };
                            let a739 = match v1.clone() {
                                Adt1::C0(_, h737, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h737
                                }
                            };
                            nat_add(a738, a739)?
                        };
                        nat_eq(a740, a741)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h742, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h742
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a743 = v0;
                let a744 = 63u64;
                if nat_eq(a743, a744) {
                    if if if if if f75(match v2.clone() {
                        Adt2::C0(_, h745, _, _, _, _, _, _, _, _, _) => {
                            h745
                        }
                    }, match v1.clone() {
                        Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h746
                        }
                    })? {
                        f75(match v2.clone() {
                            Adt2::C0(_, _, h747, _, _, _, _, _, _, _, _) => {
                                h747
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h748, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h748
                            }
                        })?
                    } else {
                        false
                    } {
                        f75(match v2.clone() {
                            Adt2::C0(_, _, _, h749, _, _, _, _, _, _, _) => {
                                h749
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h750, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h750
                            }
                        })?
                    } else {
                        false
                    } {
                        f75(match v2.clone() {
                            Adt2::C0(_, _, _, _, h751, _, _, _, _, _, _) => {
                                h751
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h752, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h752
                            }
                        })?
                    } else {
                        false
                    } {
                        let a755 = match v2.clone() {
                            Adt2::C0(_, h753, _, _, _, _, _, _, _, _, _) => {
                                h753
                            }
                        };
                        let a756 = match v3.clone() {
                            Adt2::C0(_, h754, _, _, _, _, _, _, _, _, _) => {
                                h754
                            }
                        };
                        if if if nat_eq(a755, a756) {
                            let a759 = match v2.clone() {
                                Adt2::C0(_, _, h757, _, _, _, _, _, _, _, _) => {
                                    h757
                                }
                            };
                            let a760 = match v3.clone() {
                                Adt2::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                                    h758
                                }
                            };
                            nat_eq(a759, a760)
                        } else {
                            false
                        } {
                            let a763 = match v2.clone() {
                                Adt2::C0(_, _, _, h761, _, _, _, _, _, _, _) => {
                                    h761
                                }
                            };
                            let a764 = match v3.clone() {
                                Adt2::C0(_, _, _, h762, _, _, _, _, _, _, _) => {
                                    h762
                                }
                            };
                            nat_eq(a763, a764)
                        } else {
                            false
                        } {
                            let a767 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
                                    h765
                                }
                            };
                            let a768 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h766, _, _, _, _, _, _) => {
                                    h766
                                }
                            };
                            nat_eq(a767, a768)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, 4u64, f13(v2, 1u64), f13(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h769, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h769
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h770, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h770
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h771, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h771
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h772, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h772
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
    let m773 = v3;
    match m773 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a774 = v0;
                let a775 = 4u64;
                nat_eq(a774, a775)
            };
            let v7: bool = {
                let a777 = 0u64;
                let a778 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h776, _, _, _, _, _, _, _, _, _, _, _) => {
                        h776
                    }
                };
                nat_lt(a777, a778)
            };
            let v8: u64 = f9(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                    h779
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h780, _, _, _, _, _, _, _, _) => {
                    h780
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h781, _, _, _, _, _, _, _) => {
                    h781
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
                    h782
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h783, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h783
                }
            }, match v1.clone() {
                Adt1::C0(_, h784, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h784
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h785, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h785
                }
            });
            if if if if if if if if if if if if v6 {
                let a787 = match v2.clone() {
                    Adt2::C0(h786, _, _, _, _, _, _, _, _, _, _) => {
                        h786
                    }
                };
                let a788 = 0u64;
                if nat_eq(a787, a788) {
                    let a790 = match v5.clone() {
                        Adt2::C0(h789, _, _, _, _, _, _, _, _, _, _) => {
                            h789
                        }
                    };
                    let a791 = 0u64;
                    nat_eq(a790, a791)
                } else {
                    false
                }
            } else {
                let a793 = match v2.clone() {
                    Adt2::C0(h792, _, _, _, _, _, _, _, _, _, _) => {
                        h792
                    }
                };
                let a794 = 0u64;
                if if nat_eq(a793, a794) {
                    true
                } else {
                    let a796 = match v2.clone() {
                        Adt2::C0(h795, _, _, _, _, _, _, _, _, _, _) => {
                            h795
                        }
                    };
                    let a797 = 26u64;
                    nat_eq(a796, a797)
                } {
                    let a799 = match v5.clone() {
                        Adt2::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                            h798
                        }
                    };
                    let a800 = 0u64;
                    if nat_eq(a799, a800) {
                        true
                    } else {
                        let a802 = match v5.clone() {
                            Adt2::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                                h801
                            }
                        };
                        let a803 = 26u64;
                        nat_eq(a802, a803)
                    }
                } else {
                    false
                }
            } {
                let a806 = match v2.clone() {
                    Adt2::C0(h804, _, _, _, _, _, _, _, _, _, _) => {
                        h804
                    }
                };
                let a807 = match v4.clone() {
                    Adt2::C0(h805, _, _, _, _, _, _, _, _, _, _) => {
                        h805
                    }
                };
                nat_eq(a806, a807)
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
                let a809 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h808, _, _, _, _, _) => {
                        h808
                    }
                };
                let a810 = 4u64;
                nat_eq(a809, a810)
            } else {
                false
            } {
                let a811 = v8;
                let a812 = f9(v2.clone(), 1u64)?;
                nat_le(a811, a812)
            } else {
                false
            } {
                let a814 = match v1.clone() {
                    Adt1::C0(_, _, _, h813, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h813
                    }
                };
                let a815 = 1073741824u64;
                nat_lt(a814, a815)
            } else {
                false
            } {
                let a817 = match v1.clone() {
                    Adt1::C0(h816, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h816
                    }
                };
                let a818 = 4294967295u64;
                if if if if nat_le(a817, a818) {
                    let a820 = match v1.clone() {
                        Adt1::C0(_, h819, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h819
                        }
                    };
                    let a821 = 4294967295u64;
                    nat_le(a820, a821)
                } else {
                    false
                } {
                    let a823 = match v1.clone() {
                        Adt1::C0(_, _, h822, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h822
                        }
                    };
                    let a824 = 4294967295u64;
                    nat_le(a823, a824)
                } else {
                    false
                } {
                    let a826 = match v1.clone() {
                        Adt1::C0(_, _, _, h825, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h825
                        }
                    };
                    let a827 = 4294967295u64;
                    nat_le(a826, a827)
                } else {
                    false
                } {
                    let a829 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h828, _, _, _, _, _, _, _, _, _, _, _) => {
                            h828
                        }
                    };
                    let a830 = 4294967295u64;
                    nat_le(a829, a830)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a833 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h831) => {
                            h831
                        }
                    };
                    let a834 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h832) => {
                            h832
                        }
                    };
                    nat_eq(a833, a834)
                } else {
                    true
                }
            } else {
                false
            } {
                f76(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h835, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h835
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h836, _) => {
                        h836
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
                    let a837 = v10;
                    let a838 = v8;
                    nat_add(a837, a838)?
                }, 4u64, f13(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h839, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h839
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a841 = match v0.clone() {
        Adt2::C0(h840, _, _, _, _, _, _, _, _, _, _) => {
            h840
        }
    };
    let a842 = 0u64;
    if if if nat_eq(a841, a842) {
        let a844 = match v1.clone() {
            Adt2::C0(h843, _, _, _, _, _, _, _, _, _, _) => {
                h843
            }
        };
        let a845 = 0u64;
        nat_eq(a844, a845)
    } else {
        false
    } {
        f72(match v0.clone() {
            Adt2::C0(_, h846, _, _, _, _, _, _, _, _, _) => {
                h846
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h847, _, _, _, _, _, _, _, _) => {
                h847
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h849, _, _, _, _, _, _) => {
                h849
            }
        }, match v1.clone() {
            Adt2::C0(_, h850, _, _, _, _, _, _, _, _, _) => {
                h850
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h851, _, _, _, _, _, _, _, _) => {
                h851
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h852, _, _, _, _, _, _, _) => {
                h852
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h853, _, _, _, _, _, _) => {
                h853
            }
        })
    } else {
        false
    } {
        let v2: u64 = f9(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a854 = v2;
            let a855 = f9(v0.clone(), 1u64)?;
            nat_add(a854, a855)?
        }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a857 = match v1.clone() {
        Adt2::C0(h856, _, _, _, _, _, _, _, _, _, _) => {
            h856
        }
    };
    let a858 = 0u64;
    if if if nat_eq(a857, a858) {
        true
    } else {
        let a860 = match v1.clone() {
            Adt2::C0(h859, _, _, _, _, _, _, _, _, _, _) => {
                h859
            }
        };
        let a861 = 1u64;
        nat_eq(a860, a861)
    } {
        true
    } else {
        let a863 = match v1.clone() {
            Adt2::C0(h862, _, _, _, _, _, _, _, _, _, _) => {
                h862
            }
        };
        let a864 = 30u64;
        nat_eq(a863, a864)
    } {
        let a867 = match v1.clone() {
            Adt2::C0(h865, _, _, _, _, _, _, _, _, _, _) => {
                h865
            }
        };
        let a868 = match v2.clone() {
            Adt2::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
                h866
            }
        };
        if if if if if nat_eq(a867, a868) {
            let a871 = match v1.clone() {
                Adt2::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                    h869
                }
            };
            let a872 = match v2.clone() {
                Adt2::C0(_, h870, _, _, _, _, _, _, _, _, _) => {
                    h870
                }
            };
            nat_eq(a871, a872)
        } else {
            false
        } {
            let a875 = match v1.clone() {
                Adt2::C0(_, _, _, h873, _, _, _, _, _, _, _) => {
                    h873
                }
            };
            let a876 = match v2.clone() {
                Adt2::C0(_, _, _, h874, _, _, _, _, _, _, _) => {
                    h874
                }
            };
            nat_eq(a875, a876)
        } else {
            false
        } {
            let a879 = match v1.clone() {
                Adt2::C0(_, _, _, _, h877, _, _, _, _, _, _) => {
                    h877
                }
            };
            let a880 = match v2.clone() {
                Adt2::C0(_, _, _, _, h878, _, _, _, _, _, _) => {
                    h878
                }
            };
            nat_eq(a879, a880)
        } else {
            false
        } {
            let a881 = v0;
            let a882 = 4u64;
            if nat_eq(a881, a882) {
                let a884 = match v1.clone() {
                    Adt2::C0(_, _, h883, _, _, _, _, _, _, _, _) => {
                        h883
                    }
                };
                let a885 = 1u64;
                if nat_eq(a884, a885) {
                    let a888 = match v1.clone() {
                        Adt2::C0(_, h886, _, _, _, _, _, _, _, _, _) => {
                            h886
                        }
                    };
                    let a889 = match v2.clone() {
                        Adt2::C0(_, _, h887, _, _, _, _, _, _, _, _) => {
                            h887
                        }
                    };
                    nat_eq(a888, a889)
                } else {
                    false
                }
            } else {
                let a890 = v0;
                let a891 = 4u64;
                if if nat_lt(a890, a891) {
                    let a894 = match v1.clone() {
                        Adt2::C0(_, h892, _, _, _, _, _, _, _, _, _) => {
                            h892
                        }
                    };
                    let a895 = match v1.clone() {
                        Adt2::C0(_, _, h893, _, _, _, _, _, _, _, _) => {
                            h893
                        }
                    };
                    nat_eq(a894, a895)
                } else {
                    false
                } {
                    let a898 = match v1.clone() {
                        Adt2::C0(_, _, h896, _, _, _, _, _, _, _, _) => {
                            h896
                        }
                    };
                    let a899 = match v2.clone() {
                        Adt2::C0(_, _, h897, _, _, _, _, _, _, _, _) => {
                            h897
                        }
                    };
                    nat_eq(a898, a899)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m901 = f71(match v1.clone() {
                Adt2::C0(h900, _, _, _, _, _, _, _, _, _, _) => {
                    h900
                }
            });
            match m901 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h902) => {
                            h902
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
    if if if f77(match v1.clone() {
        Adt2::C0(h903, _, _, _, _, _, _, _, _, _, _) => {
            h903
        }
    }, v0) {
        let a906 = match v1.clone() {
            Adt2::C0(h904, _, _, _, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a907 = match v2.clone() {
            Adt2::C0(h905, _, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        };
        nat_eq(a906, a907)
    } else {
        false
    } {
        let a910 = match v1.clone() {
            Adt2::C0(_, h908, _, _, _, _, _, _, _, _, _) => {
                h908
            }
        };
        let a911 = match v2.clone() {
            Adt2::C0(_, h909, _, _, _, _, _, _, _, _, _) => {
                h909
            }
        };
        if if if nat_eq(a910, a911) {
            let a914 = match v1.clone() {
                Adt2::C0(_, _, h912, _, _, _, _, _, _, _, _) => {
                    h912
                }
            };
            let a915 = match v2.clone() {
                Adt2::C0(_, _, h913, _, _, _, _, _, _, _, _) => {
                    h913
                }
            };
            nat_eq(a914, a915)
        } else {
            false
        } {
            let a918 = match v1.clone() {
                Adt2::C0(_, _, _, h916, _, _, _, _, _, _, _) => {
                    h916
                }
            };
            let a919 = match v2.clone() {
                Adt2::C0(_, _, _, h917, _, _, _, _, _, _, _) => {
                    h917
                }
            };
            nat_eq(a918, a919)
        } else {
            false
        } {
            let a922 = match v1.clone() {
                Adt2::C0(_, _, _, _, h920, _, _, _, _, _, _) => {
                    h920
                }
            };
            let a923 = match v2.clone() {
                Adt2::C0(_, _, _, _, h921, _, _, _, _, _, _) => {
                    h921
                }
            };
            nat_eq(a922, a923)
        } else {
            false
        }
    } else {
        false
    } {
        let m925 = f71(match v1.clone() {
            Adt2::C0(h924, _, _, _, _, _, _, _, _, _, _) => {
                h924
            }
        });
        match m925 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f9(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h926) => {
                        h926
                    }
                }, f13(v1.clone(), 1u64), f13(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h927, _, _, _, _, _, _, _, _, _, _) => {
                        h927
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
            Adt1::C0(h928, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h929, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        }
    };
    let a930 = 0u64;
    let a931 = v4;
    if if nat_lt(a930, a931) {
        let a932 = v4;
        let a933 = 2147483648u64;
        nat_lt(a932, a933)
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
        let v7: u64 = f78(match v5.clone() {
            Adt2::C0(_, _, h934, _, _, _, _, _, _, _, _) => {
                h934
            }
        }, v4)?;
        let v8: u64 = f78(match v5.clone() {
            Adt2::C0(_, _, _, h935, _, _, _, _, _, _, _) => {
                h935
            }
        }, v4)?;
        let a937 = match v5.clone() {
            Adt2::C0(h936, _, _, _, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a938 = 0u64;
        if if if if if nat_eq(a937, a938) {
            let a940 = match v6.clone() {
                Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
                    h939
                }
            };
            let a941 = 0u64;
            nat_eq(a940, a941)
        } else {
            false
        } {
            let a943 = match v5.clone() {
                Adt2::C0(_, _, _, _, h942, _, _, _, _, _, _) => {
                    h942
                }
            };
            let a944 = 1u64;
            nat_eq(a943, a944)
        } else {
            false
        } {
            let a947 = match v5.clone() {
                Adt2::C0(_, h945, _, _, _, _, _, _, _, _, _) => {
                    h945
                }
            };
            let a948 = match v6.clone() {
                Adt2::C0(_, h946, _, _, _, _, _, _, _, _, _) => {
                    h946
                }
            };
            nat_eq(a947, a948)
        } else {
            false
        } {
            if v0 {
                let a950 = match v5.clone() {
                    Adt2::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
                        h949
                    }
                };
                let a951 = 2147483648u64;
                if if if nat_lt(a950, a951) {
                    let a953 = match v5.clone() {
                        Adt2::C0(_, _, _, h952, _, _, _, _, _, _, _) => {
                            h952
                        }
                    };
                    let a954 = 2147483648u64;
                    nat_lt(a953, a954)
                } else {
                    false
                } {
                    let a957 = {
                        let a955 = v7;
                        let a956 = v8;
                        nat_mul(a955, a956)?
                    };
                    let a958 = 2147483648u64;
                    nat_le(a957, a958)
                } else {
                    false
                } {
                    f79(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a960 = match v1.clone() {
                    Adt1::C0(h959, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h959
                    }
                };
                let a961 = v7;
                if if if if nat_eq(a960, a961) {
                    let a963 = match v1 {
                        Adt1::C0(_, h962, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h962
                        }
                    };
                    let a964 = v8;
                    nat_eq(a963, a964)
                } else {
                    false
                } {
                    f80(v7, v8, v4)?
                } else {
                    false
                } {
                    let a966 = match v6.clone() {
                        Adt2::C0(_, _, h965, _, _, _, _, _, _, _, _) => {
                            h965
                        }
                    };
                    let a967 = v4;
                    if nat_eq(a966, a967) {
                        let a969 = match v6.clone() {
                            Adt2::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                                h968
                            }
                        };
                        let a970 = v4;
                        nat_eq(a969, a970)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a974 = match v6 {
                        Adt2::C0(_, _, _, _, h971, _, _, _, _, _, _) => {
                            h971
                        }
                    };
                    let a975 = {
                        let a972 = v7;
                        let a973 = v8;
                        nat_mul(a972, a973)?
                    };
                    nat_eq(a974, a975)
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
    let a977 = match v2.clone() {
        Adt2::C0(h976, _, _, _, _, _, _, _, _, _, _) => {
            h976
        }
    };
    let a978 = 0u64;
    if if if if if nat_eq(a977, a978) {
        true
    } else {
        let a980 = match v2.clone() {
            Adt2::C0(h979, _, _, _, _, _, _, _, _, _, _) => {
                h979
            }
        };
        let a981 = 1u64;
        nat_eq(a980, a981)
    } {
        true
    } else {
        let a983 = match v2.clone() {
            Adt2::C0(h982, _, _, _, _, _, _, _, _, _, _) => {
                h982
            }
        };
        let a984 = 30u64;
        nat_eq(a983, a984)
    } {
        let a987 = match v2.clone() {
            Adt2::C0(h985, _, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        let a988 = match v3.clone() {
            Adt2::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        nat_eq(a987, a988)
    } else {
        false
    } {
        if if f14(v2.clone(), v3.clone()) {
            let a990 = match v1.clone() {
                Adt1::C0(h989, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h989
                }
            };
            let a991 = 4294967296u64;
            nat_lt(a990, a991)
        } else {
            false
        } {
            let a993 = match v1.clone() {
                Adt1::C0(_, h992, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h992
                }
            };
            let a994 = 4294967296u64;
            nat_lt(a993, a994)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v3.clone(), 1u64)?, {
            let a996 = match v2.clone() {
                Adt2::C0(h995, _, _, _, _, _, _, _, _, _, _) => {
                    h995
                }
            };
            let a997 = 0u64;
            if nat_eq(a996, a997) {
                4u64
            } else {
                2u64
            }
        }, f13(v2.clone(), 1u64), f13(v3, 1u64), Adt4::C13({
            let a998 = v0;
            let a999 = 31u64;
            if nat_eq(a998, a999) {
                0u64
            } else {
                let a1000 = v0;
                let a1001 = 47u64;
                if nat_eq(a1000, a1001) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1002, _, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        }, match v1.clone() {
            Adt1::C0(h1003, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1003
            }
        }, match v1 {
            Adt1::C0(_, h1004, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1004
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f44(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1006 = match v0.clone() {
        Adt2::C0(h1005, _, _, _, _, _, _, _, _, _, _) => {
            h1005
        }
    };
    let a1007 = 0u64;
    if if if nat_eq(a1006, a1007) {
        let a1009 = match v1.clone() {
            Adt2::C0(h1008, _, _, _, _, _, _, _, _, _, _) => {
                h1008
            }
        };
        let a1010 = 0u64;
        nat_eq(a1009, a1010)
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
    let a1012 = match v0.clone() {
        Adt2::C0(h1011, _, _, _, _, _, _, _, _, _, _) => {
            h1011
        }
    };
    let a1013 = 0u64;
    if if nat_eq(a1012, a1013) {
        let a1015 = match v1.clone() {
            Adt2::C0(h1014, _, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1016 = 26u64;
        nat_eq(a1015, a1016)
    } else {
        false
    } {
        let a1018 = match v0.clone() {
            Adt2::C0(_, _, _, h1017, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        let a1019 = 1u64;
        if if if if if if if nat_eq(a1018, a1019) {
            let a1021 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1020, _, _, _, _, _, _) => {
                    h1020
                }
            };
            let a1022 = 1u64;
            nat_eq(a1021, a1022)
        } else {
            false
        } {
            let a1024 = match v0.clone() {
                Adt2::C0(_, h1023, _, _, _, _, _, _, _, _, _) => {
                    h1023
                }
            };
            let a1025 = 2147483647u64;
            nat_le(a1024, a1025)
        } else {
            false
        } {
            let a1028 = match v1.clone() {
                Adt2::C0(_, h1026, _, _, _, _, _, _, _, _, _) => {
                    h1026
                }
            };
            let a1029 = match v0.clone() {
                Adt2::C0(_, _, h1027, _, _, _, _, _, _, _, _) => {
                    h1027
                }
            };
            nat_eq(a1028, a1029)
        } else {
            false
        } {
            let a1031 = match v1.clone() {
                Adt2::C0(_, _, h1030, _, _, _, _, _, _, _, _) => {
                    h1030
                }
            };
            let a1032 = 1u64;
            nat_eq(a1031, a1032)
        } else {
            false
        } {
            let a1034 = match v1.clone() {
                Adt2::C0(_, _, _, h1033, _, _, _, _, _, _, _) => {
                    h1033
                }
            };
            let a1035 = 1u64;
            nat_eq(a1034, a1035)
        } else {
            false
        } {
            let a1037 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1036, _, _, _, _, _, _) => {
                    h1036
                }
            };
            let a1038 = 1u64;
            nat_eq(a1037, a1038)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1041 = match v0.clone() {
                    Adt2::C0(_, _, h1039, _, _, _, _, _, _, _, _) => {
                        h1039
                    }
                };
                let a1042 = f81(match v0.clone() {
                    Adt2::C0(_, h1040, _, _, _, _, _, _, _, _, _) => {
                        h1040
                    }
                })?;
                nat_mul(a1041, a1042)?
            }, 4u64, f13(v0, 1u64), f13(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1043 = v1;
    match m1043 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f82(f13(v0.clone(), 1u64))?;
            let a1045 = match v0.clone() {
                Adt2::C0(h1044, _, _, _, _, _, _, _, _, _, _) => {
                    h1044
                }
            };
            let a1046 = 26u64;
            if if if nat_eq(a1045, a1046) {
                let a1048 = match v3.clone() {
                    Adt2::C0(h1047, _, _, _, _, _, _, _, _, _, _) => {
                        h1047
                    }
                };
                let a1049 = 26u64;
                nat_eq(a1048, a1049)
            } else {
                false
            } {
                let a1051 = match v2.clone() {
                    Adt2::C0(h1050, _, _, _, _, _, _, _, _, _, _) => {
                        h1050
                    }
                };
                let a1052 = 27u64;
                nat_eq(a1051, a1052)
            } else {
                false
            } {
                if if if if if if f14(v0.clone(), v3.clone()) {
                    let a1054 = match v2.clone() {
                        Adt2::C0(_, h1053, _, _, _, _, _, _, _, _, _) => {
                            h1053
                        }
                    };
                    let a1055 = 1u64;
                    nat_eq(a1054, a1055)
                } else {
                    false
                } {
                    let a1057 = match v2.clone() {
                        Adt2::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                            h1056
                        }
                    };
                    let a1058 = 1u64;
                    nat_eq(a1057, a1058)
                } else {
                    false
                } {
                    let a1060 = match v2.clone() {
                        Adt2::C0(_, _, _, h1059, _, _, _, _, _, _, _) => {
                            h1059
                        }
                    };
                    let a1061 = 1u64;
                    nat_eq(a1060, a1061)
                } else {
                    false
                } {
                    let a1063 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1062, _, _, _, _, _, _) => {
                            h1062
                        }
                    };
                    let a1064 = 1u64;
                    nat_eq(a1063, a1064)
                } else {
                    false
                } {
                    let a1065 = v4;
                    let a1066 = 9223372036854775808u64;
                    nat_lt(a1065, a1066)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1069 = 2u64;
                        let a1070 = {
                            let a1067 = v4;
                            let a1068 = 2u64;
                            nat_mul(a1067, a1068)?
                        };
                        nat_add(a1069, a1070)?
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
        Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
            h1071
        }
    };
    let a1072 = v3;
    let a1073 = 0u64;
    if if nat_eq(a1072, a1073) {
        true
    } else {
        let a1074 = v0;
        let a1075 = 13u64;
        if nat_eq(a1074, a1075) {
            let a1076 = v3;
            let a1077 = 1u64;
            if nat_eq(a1076, a1077) {
                true
            } else {
                let a1078 = v3;
                let a1079 = 30u64;
                nat_eq(a1078, a1079)
            }
        } else {
            false
        }
    } {
        let a1081 = v3;
        let a1082 = match v2.clone() {
            Adt2::C0(h1080, _, _, _, _, _, _, _, _, _, _) => {
                h1080
            }
        };
        if if nat_eq(a1081, a1082) {
            let a1083 = v0;
            let a1084 = 13u64;
            if nat_eq(a1083, a1084) {
                let a1086 = match v2.clone() {
                    Adt2::C0(_, h1085, _, _, _, _, _, _, _, _, _) => {
                        h1085
                    }
                };
                let a1087 = 1u64;
                if if if nat_eq(a1086, a1087) {
                    let a1089 = match v2.clone() {
                        Adt2::C0(_, _, h1088, _, _, _, _, _, _, _, _) => {
                            h1088
                        }
                    };
                    let a1090 = 1u64;
                    nat_eq(a1089, a1090)
                } else {
                    false
                } {
                    let a1092 = match v2.clone() {
                        Adt2::C0(_, _, _, h1091, _, _, _, _, _, _, _) => {
                            h1091
                        }
                    };
                    let a1093 = 1u64;
                    nat_eq(a1092, a1093)
                } else {
                    false
                } {
                    let a1095 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1094, _, _, _, _, _, _) => {
                            h1094
                        }
                    };
                    let a1096 = 1u64;
                    nat_eq(a1095, a1096)
                } else {
                    false
                }
            } else {
                let a1098 = match v2.clone() {
                    Adt2::C0(_, h1097, _, _, _, _, _, _, _, _, _) => {
                        h1097
                    }
                };
                let a1099 = 1u64;
                if if if nat_eq(a1098, a1099) {
                    let a1102 = match v1.clone() {
                        Adt2::C0(_, _, h1100, _, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1103 = match v2.clone() {
                        Adt2::C0(_, _, h1101, _, _, _, _, _, _, _, _) => {
                            h1101
                        }
                    };
                    nat_eq(a1102, a1103)
                } else {
                    false
                } {
                    let a1106 = match v1.clone() {
                        Adt2::C0(_, _, _, h1104, _, _, _, _, _, _, _) => {
                            h1104
                        }
                    };
                    let a1107 = match v2.clone() {
                        Adt2::C0(_, _, _, h1105, _, _, _, _, _, _, _) => {
                            h1105
                        }
                    };
                    nat_eq(a1106, a1107)
                } else {
                    false
                } {
                    let a1110 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1108, _, _, _, _, _, _) => {
                            h1108
                        }
                    };
                    let a1111 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1109, _, _, _, _, _, _) => {
                            h1109
                        }
                    };
                    nat_eq(a1110, a1111)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f13(v1, 1u64);
            Ok(Adt6::C4(f83(v4.clone(), v0, v3)?, {
                let a1112 = v3;
                let a1113 = 0u64;
                if nat_eq(a1112, a1113) {
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
    let a1114 = 10u64;
    let a1115 = v0;
    if if nat_le(a1114, a1115) {
        let a1116 = v0;
        let a1117 = 12u64;
        nat_le(a1116, a1117)
    } else {
        false
    } {
        Ok(Some({
            let a1118 = v0;
            let a1119 = 3u64;
            nat_add(a1118, a1119)?
        }))
    } else {
        let a1120 = v0;
        let a1121 = 89u64;
        if if nat_eq(a1120, a1121) {
            let a1122 = v1;
            let a1123 = 7u64;
            nat_eq(a1122, a1123)
        } else {
            false
        } {
            Ok(Some(17u64))
        } else {
            let a1124 = v0;
            let a1125 = 89u64;
            if if nat_eq(a1124, a1125) {
                let a1126 = v1;
                let a1127 = 10u64;
                nat_eq(a1126, a1127)
            } else {
                false
            } {
                Ok(Some(18u64))
            } else {
                let a1128 = v0;
                let a1129 = 89u64;
                if if nat_eq(a1128, a1129) {
                    let a1130 = v1;
                    let a1131 = 13u64;
                    nat_eq(a1130, a1131)
                } else {
                    false
                } {
                    Ok(Some(16u64))
                } else {
                    let a1132 = v0;
                    let a1133 = 8u64;
                    if nat_eq(a1132, a1133) {
                        Ok(Some(5u64))
                    } else {
                        let a1134 = v0;
                        let a1135 = 9u64;
                        if nat_eq(a1134, a1135) {
                            Ok(Some(6u64))
                        } else {
                            let a1136 = v0;
                            let a1137 = 89u64;
                            if nat_eq(a1136, a1137) {
                                let a1138 = v1;
                                let a1139 = 0u64;
                                if nat_eq(a1138, a1139) {
                                    Ok(Some(0u64))
                                } else {
                                    let a1140 = v1;
                                    let a1141 = 1u64;
                                    if nat_eq(a1140, a1141) {
                                        Ok(Some(1u64))
                                    } else {
                                        let a1142 = v1;
                                        let a1143 = 2u64;
                                        if nat_eq(a1142, a1143) {
                                            Ok(Some(2u64))
                                        } else {
                                            let a1144 = v1;
                                            let a1145 = 3u64;
                                            if nat_eq(a1144, a1145) {
                                                Ok(Some(3u64))
                                            } else {
                                                let a1146 = v1;
                                                let a1147 = 6u64;
                                                if nat_eq(a1146, a1147) {
                                                    Ok(Some(4u64))
                                                } else {
                                                    let a1148 = v1;
                                                    let a1149 = 12u64;
                                                    if nat_eq(a1148, a1149) {
                                                        Ok(Some(7u64))
                                                    } else {
                                                        let a1150 = v1;
                                                        let a1151 = 11u64;
                                                        if nat_eq(a1150, a1151) {
                                                            Ok(Some(8u64))
                                                        } else {
                                                            let a1152 = v1;
                                                            let a1153 = 18u64;
                                                            if nat_eq(a1152, a1153) {
                                                                Ok(Some(9u64))
                                                            } else {
                                                                let a1154 = v1;
                                                                let a1155 = 19u64;
                                                                if nat_eq(a1154, a1155) {
                                                                    Ok(Some(10u64))
                                                                } else {
                                                                    let a1156 = v1;
                                                                    let a1157 = 20u64;
                                                                    if nat_eq(a1156, a1157) {
                                                                        Ok(Some(11u64))
                                                                    } else {
                                                                        let a1158 = v1;
                                                                        let a1159 = 21u64;
                                                                        if nat_eq(a1158, a1159) {
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
    let m1160 = f84(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1160 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            let a1161 = v0;
            let a1162 = 2u64;
            if if if if nat_eq(a1161, a1162) {
                true
            } else {
                let a1163 = v0;
                let a1164 = 5u64;
                nat_eq(a1163, a1164)
            } {
                true
            } else {
                let a1165 = v0;
                let a1166 = 6u64;
                nat_eq(a1165, a1166)
            } {
                true
            } else {
                let a1167 = v0;
                let a1168 = 7u64;
                nat_eq(a1167, a1168)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1169 = v3;
                    match m1169 {
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
    let a1171 = match v1.clone() {
        Adt2::C0(h1170, _, _, _, _, _, _, _, _, _, _) => {
            h1170
        }
    };
    let a1172 = 0u64;
    if if if if if nat_eq(a1171, a1172) {
        true
    } else {
        let a1174 = match v1.clone() {
            Adt2::C0(h1173, _, _, _, _, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1175 = 1u64;
        nat_eq(a1174, a1175)
    } {
        true
    } else {
        let a1177 = match v1.clone() {
            Adt2::C0(h1176, _, _, _, _, _, _, _, _, _, _) => {
                h1176
            }
        };
        let a1178 = 30u64;
        nat_eq(a1177, a1178)
    } {
        let a1181 = match v1.clone() {
            Adt2::C0(h1179, _, _, _, _, _, _, _, _, _, _) => {
                h1179
            }
        };
        let a1182 = match v2.clone() {
            Adt2::C0(h1180, _, _, _, _, _, _, _, _, _, _) => {
                h1180
            }
        };
        if nat_eq(a1181, a1182) {
            true
        } else {
            let a1184 = match v2.clone() {
                Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
                    h1183
                }
            };
            let a1185 = 0u64;
            nat_eq(a1184, a1185)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1187 = match v2.clone() {
                Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                    h1186
                }
            };
            let a1188 = 0u64;
            if nat_eq(a1187, a1188) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                h1189
            }
        }, match v2 {
            Adt2::C0(h1190, _, _, _, _, _, _, _, _, _, _) => {
                h1190
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1191 = v1;
    match m1191 {
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
            f85(f86(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f87(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f88(v0.clone(), v16);
            f85(f89(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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
    let m1192 = v3;
    match m1192 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f92(v0, v1, v2.clone())?, f90(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f93(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f94(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f95(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f94(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f96(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f97(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f98(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f90(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f99(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f100(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C15 => {
            f101(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f102(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f103(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f90(f91(v0, match v2.clone() {
                Adt3::C0(h1193, _, _, _, _, _, _, _) => {
                    h1193
                }
            }, match v2.clone() {
                Adt3::C0(_, h1194, _, _, _, _, _, _) => {
                    h1194
                }
            })?, v1)?, f90(v0, v2)?))
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
            f104(v0, v48, v49, v50)
        }
        Adt4::C26(v51, _, _, _) => {
            let a1195 = v51;
            let a1196 = 64u64;
            if if if nat_eq(a1195, a1196) {
                true
            } else {
                let a1197 = v51;
                let a1198 = 65u64;
                nat_eq(a1197, a1198)
            } {
                true
            } else {
                let a1199 = v51;
                let a1200 = 66u64;
                nat_eq(a1199, a1200)
            } {
                Ok(Adt5::C0(Adt6::C5(64u64)))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f81(match v1.clone() {
        Adt3::C0(h1201, _, _, _, _, _, _, _) => {
            h1201
        }
    })?;
    let v5: u64 = {
        let a1202 = v0;
        let a1203 = v4;
        let a1204 = 0u64;
        nat_quot(a1202, a1203, a1204)
    };
    let v6: u64 = {
        let a1205 = v0;
        let a1206 = v4;
        let a1207 = 0u64;
        nat_rem(a1205, a1206, a1207)
    };
    let a1208 = v6;
    let a1209 = 0u64;
    if nat_eq(a1208, a1209) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1210 = v6;
        let a1211 = 1u64;
        if nat_eq(a1210, a1211) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1214 = {
                let a1212 = v6;
                let a1213 = 1u64;
                nat_add(a1212, a1213)?
            };
            let a1215 = v4;
            if nat_eq(a1214, a1215) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f90(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1216 = v6;
                    let a1217 = 2u64;
                    nat_sub(a1216, a1217)
                };
                let v8: u64 = {
                    let a1218 = v7;
                    let a1219 = 3u64;
                    let a1220 = 0u64;
                    nat_quot(a1218, a1219, a1220)
                };
                let v9: u64 = {
                    let a1221 = v7;
                    let a1222 = 3u64;
                    let a1223 = 0u64;
                    nat_rem(a1221, a1222, a1223)
                };
                let v10: u64 = f90({
                    let a1227 = {
                        let a1225 = v5;
                        let a1226 = match v1.clone() {
                            Adt3::C0(h1224, _, _, _, _, _, _, _) => {
                                h1224
                            }
                        };
                        nat_mul(a1225, a1226)?
                    };
                    let a1228 = v8;
                    nat_add(a1227, a1228)?
                }, v1)?;
                let a1229 = v9;
                let a1230 = 0u64;
                if nat_eq(a1229, a1230) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1231 = v9;
                    let a1232 = 1u64;
                    if nat_eq(a1231, a1232) {
                        let a1233 = v3;
                        let a1234 = 2u64;
                        if nat_eq(a1233, a1234) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1235 = v3;
                        let a1236 = 2u64;
                        if if nat_eq(a1235, a1236) {
                            true
                        } else {
                            let a1237 = v3;
                            let a1238 = 3u64;
                            nat_eq(a1237, a1238)
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
        let a1240 = 2u64;
        let a1241 = match v1.clone() {
            Adt3::C0(_, h1239, _, _, _, _, _, _) => {
                h1239
            }
        };
        nat_mul(a1240, a1241)?
    };
    let a1242 = v0;
    let a1243 = v5;
    if nat_lt(a1242, a1243) {
        let a1247 = {
            let a1244 = v0;
            let a1245 = 2u64;
            let a1246 = 0u64;
            nat_rem(a1244, a1245, a1246)
        };
        let a1248 = 0u64;
        if nat_eq(a1247, a1248) {
            Ok(Adt5::C4(19u64, 4u64, f90({
                let a1249 = v0;
                let a1250 = 2u64;
                let a1251 = 0u64;
                nat_quot(a1249, a1250, a1251)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1252 = v4;
            let a1253 = 2147483648u64;
            if if nat_lt(a1252, a1253) {
                let a1255 = v4;
                let a1256 = match v2 {
                    Adt3::C0(_, h1254, _, _, _, _, _, _) => {
                        h1254
                    }
                };
                nat_lt(a1255, a1256)
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
            let a1259 = match v2.clone() {
                Adt3::C0(h1257, _, _, _, _, _, _, _) => {
                    h1257
                }
            };
            let a1260 = match v2.clone() {
                Adt3::C0(_, h1258, _, _, _, _, _, _) => {
                    h1258
                }
            };
            nat_mul(a1259, a1260)?
        };
        let a1263 = {
            let a1261 = v0;
            let a1262 = v5;
            nat_sub(a1261, a1262)
        };
        let a1264 = v6;
        if nat_lt(a1263, a1264) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f90({
                let a1265 = v0;
                let a1266 = v5;
                nat_sub(a1265, a1266)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1269 = {
                    let a1267 = v0;
                    let a1268 = v5;
                    nat_sub(a1267, a1268)
                };
                let a1270 = v6;
                nat_sub(a1269, a1270)
            };
            let v8: u64 = {
                let a1274 = 1u64;
                let a1275 = {
                    let a1272 = 2u64;
                    let a1273 = match v1.clone() {
                        Adt3::C0(h1271, _, _, _, _, _, _, _) => {
                            h1271
                        }
                    };
                    nat_mul(a1272, a1273)?
                };
                nat_add(a1274, a1275)?
            };
            let v9: u64 = {
                let a1276 = v7;
                let a1277 = v8;
                let a1278 = 0u64;
                nat_quot(a1276, a1277, a1278)
            };
            let v10: u64 = {
                let a1279 = v7;
                let a1280 = v8;
                let a1281 = 0u64;
                nat_rem(a1279, a1280, a1281)
            };
            let a1282 = v10;
            let a1283 = 0u64;
            if nat_eq(a1282, a1283) {
                Ok(Adt5::C4(19u64, 4u64, f90(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1287 = {
                    let a1284 = v10;
                    let a1285 = 2u64;
                    let a1286 = 0u64;
                    nat_rem(a1284, a1285, a1286)
                };
                let a1288 = 1u64;
                if nat_eq(a1287, a1288) {
                    Ok(Adt5::C4(6u64, 0u64, f90({
                        let a1297 = {
                            let a1290 = v9;
                            let a1291 = match v1.clone() {
                                Adt3::C0(h1289, _, _, _, _, _, _, _) => {
                                    h1289
                                }
                            };
                            nat_mul(a1290, a1291)?
                        };
                        let a1298 = {
                            let a1294 = {
                                let a1292 = v10;
                                let a1293 = 1u64;
                                nat_sub(a1292, a1293)
                            };
                            let a1295 = 2u64;
                            let a1296 = 0u64;
                            nat_quot(a1294, a1295, a1296)
                        };
                        nat_add(a1297, a1298)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f90({
                        let a1307 = {
                            let a1300 = v4;
                            let a1301 = match v1.clone() {
                                Adt3::C0(h1299, _, _, _, _, _, _, _) => {
                                    h1299
                                }
                            };
                            nat_mul(a1300, a1301)?
                        };
                        let a1308 = {
                            let a1304 = {
                                let a1302 = v10;
                                let a1303 = 1u64;
                                nat_sub(a1302, a1303)
                            };
                            let a1305 = 2u64;
                            let a1306 = 0u64;
                            nat_quot(a1304, a1305, a1306)
                        };
                        nat_add(a1307, a1308)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f90({
                        let a1317 = {
                            let a1310 = v4;
                            let a1311 = match v1 {
                                Adt3::C0(h1309, _, _, _, _, _, _, _) => {
                                    h1309
                                }
                            };
                            nat_mul(a1310, a1311)?
                        };
                        let a1318 = {
                            let a1314 = {
                                let a1312 = v10;
                                let a1313 = 1u64;
                                nat_sub(a1312, a1313)
                            };
                            let a1315 = 2u64;
                            let a1316 = 0u64;
                            nat_quot(a1314, a1315, a1316)
                        };
                        nat_add(a1317, a1318)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1319 = v1;
    let a1320 = f105(v2.clone(), v4.clone())?;
    if nat_lt(a1319, a1320) {
        let a1323 = v1;
        let a1324 = {
            let a1321 = 2u64;
            let a1322 = f82(v4.clone())?;
            nat_mul(a1321, a1322)?
        };
        if nat_lt(a1323, a1324) {
            let a1328 = {
                let a1325 = v1;
                let a1326 = 2u64;
                let a1327 = 0u64;
                nat_rem(a1325, a1326, a1327)
            };
            let a1329 = 0u64;
            if nat_eq(a1328, a1329) {
                f106({
                    let a1330 = v1;
                    let a1331 = 2u64;
                    let a1332 = 0u64;
                    nat_quot(a1330, a1331, a1332)
                }, v4)
            } else {
                if f107(match v2 {
                    Adt3::C0(_, h1333, _, _, _, _, _, _) => {
                        h1333
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1336 = v1;
                let a1337 = {
                    let a1334 = 2u64;
                    let a1335 = f82(v4.clone())?;
                    nat_mul(a1334, a1335)?
                };
                nat_sub(a1336, a1337)
            };
            let v7: u64 = {
                let a1341 = v6;
                let a1342 = {
                    let a1339 = 1u64;
                    let a1340 = match v2.clone() {
                        Adt3::C0(h1338, _, _, _, _, _, _, _) => {
                            h1338
                        }
                    };
                    nat_add(a1339, a1340)?
                };
                let a1343 = 0u64;
                nat_quot(a1341, a1342, a1343)
            };
            let v8: u64 = {
                let a1347 = v6;
                let a1348 = {
                    let a1345 = 1u64;
                    let a1346 = match v2.clone() {
                        Adt3::C0(h1344, _, _, _, _, _, _, _) => {
                            h1344
                        }
                    };
                    nat_add(a1345, a1346)?
                };
                let a1349 = 0u64;
                nat_rem(a1347, a1348, a1349)
            };
            let a1350 = v8;
            let a1351 = 0u64;
            if nat_eq(a1350, a1351) {
                f106(v7, v4)
            } else {
                f108(v0, v7, {
                    let a1352 = v8;
                    let a1353 = 1u64;
                    nat_sub(a1352, a1353)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1356 = match v4.clone() {
            Adt3::C0(h1354, _, _, _, _, _, _, _) => {
                h1354
            }
        };
        let a1357 = match v4.clone() {
            Adt3::C0(_, h1355, _, _, _, _, _, _) => {
                h1355
            }
        };
        nat_mul(a1356, a1357)?
    };
    let v7: u64 = {
        let a1358 = 2u64;
        let a1359 = v6;
        nat_mul(a1358, a1359)?
    };
    let v8: u64 = f109(match v3.clone() {
        Adt3::C0(_, h1360, _, _, _, _, _, _) => {
            h1360
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1361, _, _, _, _, _) => {
            h1361
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1362, _, _, _, _) => {
            h1362
        }
    });
    let a1363 = v0;
    let a1364 = v7;
    if nat_lt(a1363, a1364) {
        let a1368 = {
            let a1365 = v0;
            let a1366 = 2u64;
            let a1367 = 0u64;
            nat_rem(a1365, a1366, a1367)
        };
        let a1369 = 0u64;
        if nat_eq(a1368, a1369) {
            Ok(Adt5::C4(19u64, 5u64, f90({
                let a1370 = v0;
                let a1371 = 2u64;
                let a1372 = 0u64;
                nat_quot(a1370, a1371, a1372)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f110(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1373 = v0;
            let a1374 = v7;
            nat_sub(a1373, a1374)
        };
        let v10: u64 = {
            let a1378 = v9;
            let a1379 = {
                let a1376 = 1u64;
                let a1377 = match v1.clone() {
                    Adt3::C0(h1375, _, _, _, _, _, _, _) => {
                        h1375
                    }
                };
                nat_add(a1376, a1377)?
            };
            let a1380 = 0u64;
            nat_quot(a1378, a1379, a1380)
        };
        let v11: u64 = {
            let a1384 = v9;
            let a1385 = {
                let a1382 = 1u64;
                let a1383 = match v1.clone() {
                    Adt3::C0(h1381, _, _, _, _, _, _, _) => {
                        h1381
                    }
                };
                nat_add(a1382, a1383)?
            };
            let a1386 = 0u64;
            nat_rem(a1384, a1385, a1386)
        };
        let a1387 = v11;
        let a1388 = 0u64;
        if nat_eq(a1387, a1388) {
            Ok(Adt5::C4(19u64, 5u64, f90({
                let a1389 = v10;
                let a1390 = v6;
                let a1391 = 0u64;
                nat_rem(a1389, a1390, a1391)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f110(v5, v8) {
                let v12: u64 = {
                    let a1397 = {
                        let a1393 = v10;
                        let a1394 = match v1.clone() {
                            Adt3::C0(h1392, _, _, _, _, _, _, _) => {
                                h1392
                            }
                        };
                        nat_mul(a1393, a1394)?
                    };
                    let a1398 = {
                        let a1395 = v11;
                        let a1396 = 1u64;
                        nat_sub(a1395, a1396)
                    };
                    nat_add(a1397, a1398)?
                };
                let a1399 = v5;
                let a1400 = 4294967295u64;
                if nat_eq(a1399, a1400) {
                    Ok(Adt5::C1(false, f90(v12, v1)?, f90(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f90(v12, v1)?, 4u64, f90({
                        let a1406 = {
                            let a1402 = v5;
                            let a1403 = match v3.clone() {
                                Adt3::C0(h1401, _, _, _, _, _, _, _) => {
                                    h1401
                                }
                            };
                            nat_mul(a1402, a1403)?
                        };
                        let a1407 = {
                            let a1404 = v11;
                            let a1405 = 1u64;
                            nat_sub(a1404, a1405)
                        };
                        nat_add(a1406, a1407)?
                    }, v3)?, 3u64, f90(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt5> {
    let v8: u64 = {
        let a1410 = v6;
        let a1411 = {
            let a1408 = 3u64;
            let a1409 = v5;
            nat_add(a1408, a1409)?
        };
        nat_sub(a1410, a1411)
    };
    let a1413 = v0;
    let a1414 = f25(v3, match v1.clone() {
        Adt3::C0(h1412, _, _, _, _, _, _, _) => {
            h1412
        }
    }, v5, v8)?;
    if nat_lt(a1413, a1414) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1415, _, _, _, _, _, _, _) => {
                h1415
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1416, _, _, _, _, _, _, _) => {
                h1416
            }
        }, v5)?;
        let a1417 = v0;
        let a1418 = v9;
        if nat_lt(a1417, a1418) {
            f111(v0, v2, f19(v3), {
                let a1419 = 3u64;
                let a1420 = v5;
                nat_add(a1419, a1420)?
            }, v7)
        } else {
            let a1423 = v0;
            let a1424 = {
                let a1421 = v9;
                let a1422 = v10;
                nat_add(a1421, a1422)?
            };
            if nat_lt(a1423, a1424) {
                let v11: u64 = {
                    let a1425 = v0;
                    let a1426 = v9;
                    nat_sub(a1425, a1426)
                };
                let v12: u64 = f112(v3, match v1.clone() {
                    Adt3::C0(h1427, _, _, _, _, _, _, _) => {
                        h1427
                    }
                })?;
                f113({
                    let a1428 = v11;
                    let a1429 = v12;
                    let a1430 = 0u64;
                    nat_rem(a1428, a1429, a1430)
                }, {
                    let a1431 = v11;
                    let a1432 = v12;
                    let a1433 = 0u64;
                    nat_quot(a1431, a1432, a1433)
                }, v1, v3, v4, v7)
            } else {
                f114({
                    let a1436 = v0;
                    let a1437 = {
                        let a1434 = v9;
                        let a1435 = v10;
                        nat_add(a1434, a1435)?
                    };
                    nat_sub(a1436, a1437)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1438 = v0;
    let a1439 = f12(v1)?;
    if nat_lt(a1438, a1439) {
        let a1440 = v0;
        let a1441 = 4u64;
        if nat_lt(a1440, a1441) {
            let a1442 = v0;
            let a1443 = 0u64;
            if nat_eq(a1442, a1443) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1444 = v0;
                let a1445 = 1u64;
                if nat_eq(a1444, a1445) {
                    let a1446 = v5;
                    let a1447 = 0u64;
                    if nat_eq(a1446, a1447) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1448 = v0;
                    let a1449 = 2u64;
                    if nat_eq(a1448, a1449) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1450 = v0;
            let a1451 = 4u64;
            if if nat_eq(a1450, a1451) {
                let a1452 = f115(v5)?;
                let a1453 = v1;
                !nat_eq(a1452, a1453)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1456 = {
                        let a1454 = v0;
                        let a1455 = 4u64;
                        nat_sub(a1454, a1455)
                    };
                    let a1457 = 3u64;
                    let a1458 = 0u64;
                    nat_quot(a1456, a1457, a1458)
                };
                let v7: u64 = {
                    let a1461 = {
                        let a1459 = v0;
                        let a1460 = 4u64;
                        nat_sub(a1459, a1460)
                    };
                    let a1462 = 3u64;
                    let a1463 = 0u64;
                    nat_rem(a1461, a1462, a1463)
                };
                let a1464 = v7;
                let a1465 = 0u64;
                if nat_eq(a1464, a1465) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1466 = v7;
                    let a1467 = 1u64;
                    if nat_eq(a1466, a1467) {
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
        let a1468 = v0;
        let a1469 = 4294967296u64;
        let a1470 = 0u64;
        nat_rem(a1468, a1469, a1470)
    };
    let a1471 = v1;
    let a1472 = 2147483648u64;
    if nat_lt(a1471, a1472) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1475 = 2u64;
    let a1476 = {
        let a1473 = 2u64;
        let a1474 = v0;
        nat_mul(a1473, a1474)?
    };
    nat_add(a1475, a1476)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f116(v0, v3, v6) {
        f116(v1, v4, v6)
    } else {
        false
    } {
        f116(v2, v5, v6)
    } else {
        false
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f117(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f117(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f117(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f117(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f117(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f117(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f63(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1477 = v2;
    let a1478 = 0u64;
    if nat_eq(a1477, a1478) {
        Ok(0u64)
    } else {
        let a1481 = 2u64;
        let a1482 = {
            let a1479 = v0;
            let a1480 = v1;
            nat_mul(a1479, a1480)?
        };
        nat_mul(a1481, a1482)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1483 = v2;
    let a1484 = f112(v0, v1)?;
    nat_mul(a1483, a1484)
}

pub fn f65(v0: u64) -> u64 {
    let a1485 = v0;
    let a1486 = 30u64;
    if nat_eq(a1485, a1486) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1489 = 5u64;
    let a1490 = {
        let a1487 = 4u64;
        let a1488 = v0;
        nat_mul(a1487, a1488)?
    };
    nat_add(a1489, a1490)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f118(match v0.clone() {
        Adt2::C0(h1491, _, _, _, _, _, _, _, _, _, _) => {
            h1491
        }
    }, match v0.clone() {
        Adt2::C0(_, h1492, _, _, _, _, _, _, _, _, _) => {
            h1492
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1493, _, _, _, _, _, _, _, _) => {
            h1493
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1494, _, _, _, _, _, _, _) => {
            h1494
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1495, _, _, _, _, _, _) => {
            h1495
        }
    }, match v1.clone() {
        Adt2::C0(h1496, _, _, _, _, _, _, _, _, _, _) => {
            h1496
        }
    }, match v1.clone() {
        Adt2::C0(_, h1497, _, _, _, _, _, _, _, _, _) => {
            h1497
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1498, _, _, _, _, _, _, _, _) => {
            h1498
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1499, _, _, _, _, _, _, _) => {
            h1499
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1500, _, _, _, _, _, _) => {
            h1500
        }
    }, match v2.clone() {
        Adt2::C0(h1501, _, _, _, _, _, _, _, _, _, _) => {
            h1501
        }
    }, match v2.clone() {
        Adt2::C0(_, h1502, _, _, _, _, _, _, _, _, _) => {
            h1502
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1503, _, _, _, _, _, _, _, _) => {
            h1503
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1504, _, _, _, _, _, _, _) => {
            h1504
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1505, _, _, _, _, _, _) => {
            h1505
        }
    }) {
        let m1507 = f71(match v0.clone() {
            Adt2::C0(h1506, _, _, _, _, _, _, _, _, _, _) => {
                h1506
            }
        });
        match m1507 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1513 = {
                    let a1510 = match v0.clone() {
                        Adt2::C0(_, h1508, _, _, _, _, _, _, _, _, _) => {
                            h1508
                        }
                    };
                    let a1511 = match v3.clone() {
                        Adt11::C0(_, h1509, _) => {
                            h1509
                        }
                    };
                    let a1512 = 0u64;
                    nat_rem(a1510, a1511, a1512)
                };
                let a1514 = 0u64;
                if if nat_eq(a1513, a1514) {
                    if f119(match v0.clone() {
                        Adt2::C0(_, h1515, _, _, _, _, _, _, _, _, _) => {
                            h1515
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1516, _, _, _, _, _, _, _, _, _) => {
                            h1516
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1517, _, _, _, _, _, _, _, _) => {
                            h1517
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1518, _, _, _, _, _, _, _) => {
                            h1518
                        }
                    }) {
                        true
                    } else {
                        let a1520 = 4u64;
                        let a1521 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1519, _, _, _, _, _) => {
                                h1519
                            }
                        };
                        if if nat_le(a1520, a1521) {
                            let a1523 = match v3.clone() {
                                Adt11::C0(_, h1522, _) => {
                                    h1522
                                }
                            };
                            let a1524 = 1u64;
                            if nat_eq(a1523, a1524) {
                                true
                            } else {
                                let a1527 = match v3.clone() {
                                    Adt11::C0(_, _, h1525) => {
                                        h1525
                                    }
                                };
                                let a1528 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1526, _, _, _, _, _) => {
                                        h1526
                                    }
                                };
                                nat_le(a1527, a1528)
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
                        Adt11::C0(_, h1529, _) => {
                            h1529
                        }
                    });
                    Ok(Adt6::C4(if f119(match v0.clone() {
                        Adt2::C0(_, h1530, _, _, _, _, _, _, _, _, _) => {
                            h1530
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1531, _, _, _, _, _, _, _, _, _) => {
                            h1531
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1532, _, _, _, _, _, _, _, _) => {
                            h1532
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1533, _, _, _, _, _, _, _) => {
                            h1533
                        }
                    }) {
                        0u64
                    } else {
                        f105(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1534, _, _, _, _, _, _, _, _, _, _) => {
                            h1534
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

pub fn f68(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if if if if if if if if if if f70(match v0.clone() {
        Adt2::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
            h1535
        }
    }) {
        let a1537 = match v1.clone() {
            Adt2::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
                h1536
            }
        };
        let a1538 = 26u64;
        nat_eq(a1537, a1538)
    } else {
        false
    } {
        let a1540 = match v2.clone() {
            Adt2::C0(h1539, _, _, _, _, _, _, _, _, _, _) => {
                h1539
            }
        };
        let a1541 = 0u64;
        nat_eq(a1540, a1541)
    } else {
        false
    } {
        let a1544 = match v0.clone() {
            Adt2::C0(_, h1542, _, _, _, _, _, _, _, _, _) => {
                h1542
            }
        };
        let a1545 = match v2.clone() {
            Adt2::C0(_, h1543, _, _, _, _, _, _, _, _, _) => {
                h1543
            }
        };
        nat_eq(a1544, a1545)
    } else {
        false
    } {
        let a1548 = match v1.clone() {
            Adt2::C0(_, h1546, _, _, _, _, _, _, _, _, _) => {
                h1546
            }
        };
        let a1549 = match v0.clone() {
            Adt2::C0(_, _, h1547, _, _, _, _, _, _, _, _) => {
                h1547
            }
        };
        nat_le(a1548, a1549)
    } else {
        false
    } {
        let a1551 = match v0.clone() {
            Adt2::C0(_, _, _, h1550, _, _, _, _, _, _, _) => {
                h1550
            }
        };
        let a1552 = 1u64;
        nat_eq(a1551, a1552)
    } else {
        false
    } {
        let a1554 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1553, _, _, _, _, _, _) => {
                h1553
            }
        };
        let a1555 = 1u64;
        nat_eq(a1554, a1555)
    } else {
        false
    } {
        let a1557 = match v2.clone() {
            Adt2::C0(_, _, _, h1556, _, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1558 = 1u64;
        nat_eq(a1557, a1558)
    } else {
        false
    } {
        let a1560 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1559, _, _, _, _, _, _) => {
                h1559
            }
        };
        let a1561 = 1u64;
        nat_eq(a1560, a1561)
    } else {
        false
    } {
        let a1563 = match v1.clone() {
            Adt2::C0(_, _, h1562, _, _, _, _, _, _, _, _) => {
                h1562
            }
        };
        let a1564 = 1u64;
        nat_eq(a1563, a1564)
    } else {
        false
    } {
        let a1566 = match v1.clone() {
            Adt2::C0(_, _, _, h1565, _, _, _, _, _, _, _) => {
                h1565
            }
        };
        let a1567 = 1u64;
        nat_eq(a1566, a1567)
    } else {
        false
    } {
        let a1569 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1568, _, _, _, _, _, _) => {
                h1568
            }
        };
        let a1570 = 1u64;
        nat_eq(a1569, a1570)
    } else {
        false
    } {
        if f23(match v2.clone() {
            Adt2::C0(_, h1571, _, _, _, _, _, _, _, _, _) => {
                h1571
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h1572, _, _, _, _, _, _, _, _) => {
                h1572
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h1573, _, _, _, _, _, _, _) => {
                h1573
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h1574, _, _, _, _, _, _) => {
                h1574
            }
        }) {
            Ok(Adt6::C4(0u64, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a1576 = match v1.clone() {
                Adt2::C0(_, h1575, _, _, _, _, _, _, _, _, _) => {
                    h1575
                }
            };
            let a1577 = 0u64;
            if nat_eq(a1576, a1577) {
                Ok(Adt6::C4(f9(v2.clone(), 1u64)?, 4u64, f13(v0, 1u64), f13(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a1579 = match v0.clone() {
                    Adt2::C0(_, h1578, _, _, _, _, _, _, _, _, _) => {
                        h1578
                    }
                };
                let a1580 = 2147483647u64;
                if if if if if if nat_le(a1579, a1580) {
                    let a1582 = match v1.clone() {
                        Adt2::C0(_, h1581, _, _, _, _, _, _, _, _, _) => {
                            h1581
                        }
                    };
                    let a1583 = 2147483647u64;
                    nat_le(a1582, a1583)
                } else {
                    false
                } {
                    let a1585 = 4u64;
                    let a1586 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h1584, _, _, _, _, _) => {
                            h1584
                        }
                    };
                    nat_le(a1585, a1586)
                } else {
                    false
                } {
                    let a1592 = {
                        let a1589 = match v2.clone() {
                            Adt2::C0(_, h1587, _, _, _, _, _, _, _, _, _) => {
                                h1587
                            }
                        };
                        let a1590 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1588, _, _, _, _, _) => {
                                h1588
                            }
                        };
                        nat_mul(a1589, a1590)?
                    };
                    let a1593 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1591, _, _, _, _) => {
                            h1591
                        }
                    };
                    nat_le(a1592, a1593)
                } else {
                    false
                } {
                    f16(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f16(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1607 = {
                            let a1597 = {
                                let a1595 = 2u64;
                                let a1596 = match v1.clone() {
                                    Adt2::C0(_, h1594, _, _, _, _, _, _, _, _, _) => {
                                        h1594
                                    }
                                };
                                nat_mul(a1595, a1596)?
                            };
                            let a1598 = f9(v2.clone(), 1u64)?;
                            nat_add(a1597, a1598)?
                        };
                        let a1608 = {
                            let a1605 = match v1.clone() {
                                Adt2::C0(_, h1599, _, _, _, _, _, _, _, _, _) => {
                                    h1599
                                }
                            };
                            let a1606 = {
                                let a1603 = 1u64;
                                let a1604 = {
                                    let a1601 = 2u64;
                                    let a1602 = match v0.clone() {
                                        Adt2::C0(_, h1600, _, _, _, _, _, _, _, _, _) => {
                                            h1600
                                        }
                                    };
                                    nat_mul(a1601, a1602)?
                                };
                                nat_add(a1603, a1604)?
                            };
                            nat_mul(a1605, a1606)?
                        };
                        nat_add(a1607, a1608)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h1609, _, _, _, _, _, _, _, _, _) => {
                            h1609
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1610, _, _, _, _, _, _, _, _, _) => {
                            h1610
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h1611, _, _, _, _, _) => {
                            h1611
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1612, _, _, _, _) => {
                            h1612
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h1613, _, _, _) => {
                            h1613
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1614, _, _) => {
                            h1614
                        }
                    }), f13(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h1615, _, _, _, _, _, _, _, _, _, _) => {
                            h1615
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

pub fn f69(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1618 = match v1.clone() {
        Adt2::C0(h1616, _, _, _, _, _, _, _, _, _, _) => {
            h1616
        }
    };
    let a1619 = match v3.clone() {
        Adt2::C0(h1617, _, _, _, _, _, _, _, _, _, _) => {
            h1617
        }
    };
    if if nat_eq(a1618, a1619) {
        let a1620 = v0;
        let a1621 = 33u64;
        if nat_eq(a1620, a1621) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1623 = f71(match v1.clone() {
            Adt2::C0(h1622, _, _, _, _, _, _, _, _, _, _) => {
                h1622
            }
        });
        match m1623 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1624, _) => {
                        h1624
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1629 = {
                    let a1626 = match v1.clone() {
                        Adt2::C0(_, h1625, _, _, _, _, _, _, _, _, _) => {
                            h1625
                        }
                    };
                    let a1627 = v5;
                    let a1628 = 1u64;
                    nat_rem(a1626, a1627, a1628)
                };
                let a1630 = 0u64;
                if if if nat_eq(a1629, a1630) {
                    let a1635 = {
                        let a1632 = match v3.clone() {
                            Adt2::C0(_, h1631, _, _, _, _, _, _, _, _, _) => {
                                h1631
                            }
                        };
                        let a1633 = v5;
                        let a1634 = 1u64;
                        nat_rem(a1632, a1633, a1634)
                    };
                    let a1636 = 0u64;
                    nat_eq(a1635, a1636)
                } else {
                    false
                } {
                    let a1637 = v6;
                    let a1638 = f9(v3.clone(), v5)?;
                    nat_eq(a1637, a1638)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1639) => {
                            h1639
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

pub fn f70(v0: u64) -> bool {
    let a1640 = v0;
    let a1641 = 0u64;
    if if nat_eq(a1640, a1641) {
        true
    } else {
        let a1642 = v0;
        let a1643 = 1u64;
        nat_eq(a1642, a1643)
    } {
        true
    } else {
        let a1644 = v0;
        let a1645 = 30u64;
        nat_eq(a1644, a1645)
    }
}

pub fn f71(v0: u64) -> Adt10 {
    let a1646 = v0;
    let a1647 = 0u64;
    if nat_eq(a1646, a1647) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1648 = v0;
        let a1649 = 1u64;
        if nat_eq(a1648, a1649) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1650 = v0;
            let a1651 = 2u64;
            if nat_eq(a1650, a1651) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1652 = v0;
                let a1653 = 3u64;
                if nat_eq(a1652, a1653) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1654 = v0;
                    let a1655 = 6u64;
                    if nat_eq(a1654, a1655) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1656 = v0;
                        let a1657 = 7u64;
                        if nat_eq(a1656, a1657) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1658 = v0;
                            let a1659 = 8u64;
                            if nat_eq(a1658, a1659) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1660 = v0;
                                let a1661 = 9u64;
                                if nat_eq(a1660, a1661) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1662 = v0;
                                    let a1663 = 10u64;
                                    if nat_eq(a1662, a1663) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1664 = v0;
                                        let a1665 = 11u64;
                                        if nat_eq(a1664, a1665) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1666 = v0;
                                            let a1667 = 12u64;
                                            if nat_eq(a1666, a1667) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1668 = v0;
                                                let a1669 = 13u64;
                                                if nat_eq(a1668, a1669) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1670 = v0;
                                                    let a1671 = 14u64;
                                                    if nat_eq(a1670, a1671) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1672 = v0;
                                                        let a1673 = 15u64;
                                                        if nat_eq(a1672, a1673) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1674 = v0;
                                                            let a1675 = 16u64;
                                                            if nat_eq(a1674, a1675) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1676 = v0;
                                                                let a1677 = 17u64;
                                                                if nat_eq(a1676, a1677) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1678 = v0;
                                                                    let a1679 = 18u64;
                                                                    if nat_eq(a1678, a1679) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1680 = v0;
                                                                        let a1681 = 19u64;
                                                                        if nat_eq(a1680, a1681) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1682 = v0;
                                                                            let a1683 = 20u64;
                                                                            if nat_eq(a1682, a1683) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1684 = v0;
                                                                                let a1685 = 21u64;
                                                                                if nat_eq(a1684, a1685) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1686 = v0;
                                                                                    let a1687 = 22u64;
                                                                                    if nat_eq(a1686, a1687) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1688 = v0;
                                                                                        let a1689 = 23u64;
                                                                                        if nat_eq(a1688, a1689) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1690 = v0;
                                                                                            let a1691 = 24u64;
                                                                                            if nat_eq(a1690, a1691) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1692 = v0;
                                                                                                let a1693 = 25u64;
                                                                                                if nat_eq(a1692, a1693) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1694 = v0;
                                                                                                    let a1695 = 26u64;
                                                                                                    if nat_eq(a1694, a1695) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1696 = v0;
                                                                                                        let a1697 = 27u64;
                                                                                                        if nat_eq(a1696, a1697) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1698 = v0;
                                                                                                            let a1699 = 28u64;
                                                                                                            if nat_eq(a1698, a1699) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1700 = v0;
                                                                                                                let a1701 = 29u64;
                                                                                                                if nat_eq(a1700, a1701) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1702 = v0;
                                                                                                                    let a1703 = 30u64;
                                                                                                                    if nat_eq(a1702, a1703) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1704 = v0;
                                                                                                                        let a1705 = 34u64;
                                                                                                                        if nat_eq(a1704, a1705) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1706 = v0;
                                                                                                                            let a1707 = 35u64;
                                                                                                                            if nat_eq(a1706, a1707) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1708 = v0;
                                                                                                                                let a1709 = 39u64;
                                                                                                                                if nat_eq(a1708, a1709) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1710 = v0;
                                                                                                                                    let a1711 = 40u64;
                                                                                                                                    if nat_eq(a1710, a1711) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1712 = v0;
                                                                                                                                        let a1713 = 41u64;
                                                                                                                                        if nat_eq(a1712, a1713) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1714 = v0;
                                                                                                                                            let a1715 = 42u64;
                                                                                                                                            if nat_eq(a1714, a1715) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1716 = v0;
                                                                                                                                                let a1717 = 43u64;
                                                                                                                                                if nat_eq(a1716, a1717) {
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

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f23(v4, v5, v6, v7) {
        f23(v0, v1, v2, v3)
    } else {
        let a1721 = {
            let a1718 = v0;
            let a1719 = v4;
            let a1720 = 1u64;
            nat_rem(a1718, a1719, a1720)
        };
        let a1722 = 0u64;
        if if if nat_eq(a1721, a1722) {
            let a1726 = {
                let a1723 = v1;
                let a1724 = v5;
                let a1725 = 1u64;
                nat_rem(a1723, a1724, a1725)
            };
            let a1727 = 0u64;
            nat_eq(a1726, a1727)
        } else {
            false
        } {
            let a1731 = {
                let a1728 = v2;
                let a1729 = v6;
                let a1730 = 1u64;
                nat_rem(a1728, a1729, a1730)
            };
            let a1732 = 0u64;
            nat_eq(a1731, a1732)
        } else {
            false
        } {
            let a1736 = {
                let a1733 = v3;
                let a1734 = v7;
                let a1735 = 1u64;
                nat_rem(a1733, a1734, a1735)
            };
            let a1737 = 0u64;
            nat_eq(a1736, a1737)
        } else {
            false
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1738 = v0;
    let a1739 = v1;
    if nat_eq(a1738, a1739) {
        let a1742 = v4;
        let a1743 = {
            let a1740 = v2;
            let a1741 = v3;
            nat_add(a1740, a1741)?
        };
        Ok(nat_eq(a1742, a1743))
    } else {
        let a1744 = v2;
        let a1745 = v3;
        if nat_eq(a1744, a1745) {
            let a1746 = v4;
            let a1747 = v2;
            Ok(nat_eq(a1746, a1747))
        } else {
            Ok(false)
        }
    }
}

pub fn f74(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1750 = {
            let a1748 = v0;
            let a1749 = f120(v1);
            nat_add(a1748, a1749)?
        };
        let a1751 = f120(v2);
        nat_add(a1750, a1751)?
    };
    let v5: u64 = {
        let a1752 = f121(v1);
        let a1753 = f121(v2);
        nat_add(a1752, a1753)?
    };
    let a1754 = v1;
    let a1755 = 4294967295u64;
    if if nat_le(a1754, a1755) {
        let a1756 = v2;
        let a1757 = 4294967295u64;
        nat_le(a1756, a1757)
    } else {
        false
    } {
        let a1758 = v5;
        let a1759 = v4;
        if nat_le(a1758, a1759) {
            let a1762 = v3;
            let a1763 = {
                let a1760 = v4;
                let a1761 = v5;
                nat_sub(a1760, a1761)
            };
            Ok(nat_eq(a1762, a1763))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f75(v0: u64, v1: u64) -> R<bool> {
    let a1764 = v1;
    let a1765 = 4294967295u64;
    if nat_le(a1764, a1765) {
        let a1768 = {
            let a1766 = f120(v1);
            let a1767 = f121(v1);
            nat_add(a1766, a1767)?
        };
        let a1769 = v0;
        Ok(nat_lt(a1768, a1769))
    } else {
        Ok(false)
    }
}

pub fn f76(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f122(v0.clone(), false)?;
    let v6: u64 = f122(v0, true)?;
    let a1770 = v6;
    let a1771 = v1;
    if if nat_le(a1770, a1771) {
        let a1778 = {
            let a1776 = {
                let a1772 = v1;
                let a1773 = v5;
                nat_add(a1772, a1773)?
            };
            let a1777 = {
                let a1774 = v3;
                let a1775 = 0u64;
                if nat_eq(a1774, a1775) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1776, a1777)?
        };
        let a1779 = v2;
        nat_le(a1778, a1779)
    } else {
        false
    } {
        if v4 {
            let a1784 = {
                let a1782 = {
                    let a1780 = v1;
                    let a1781 = v5;
                    nat_add(a1780, a1781)?
                };
                let a1783 = v6;
                nat_sub(a1782, a1783)
            };
            let a1785 = v2;
            Ok(nat_lt(a1784, a1785))
        } else {
            let a1790 = {
                let a1788 = {
                    let a1786 = v1;
                    let a1787 = v5;
                    nat_add(a1786, a1787)?
                };
                let a1789 = v6;
                nat_sub(a1788, a1789)
            };
            let a1791 = v2;
            Ok(nat_le(a1790, a1791))
        }
    } else {
        Ok(false)
    }
}

pub fn f77(v0: u64, v1: u64) -> bool {
    let a1792 = v0;
    let a1793 = 0u64;
    if if nat_eq(a1792, a1793) {
        true
    } else {
        let a1794 = v0;
        let a1795 = 1u64;
        nat_eq(a1794, a1795)
    } {
        let a1796 = v1;
        let a1797 = 4294967296u64;
        nat_lt(a1796, a1797)
    } else {
        false
    }
}

pub fn f78(v0: u64, v1: u64) -> R<u64> {
    let a1802 = {
        let a1800 = v0;
        let a1801 = {
            let a1798 = v1;
            let a1799 = 1u64;
            nat_sub(a1798, a1799)
        };
        nat_add(a1800, a1801)?
    };
    let a1803 = v1;
    let a1804 = 0u64;
    Ok(nat_quot(a1802, a1803, a1804))
}

pub fn f79(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1805, _, _, _, _, _, _, _, _, _) => {
            h1805
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1806, _, _, _, _, _, _, _, _) => {
            h1806
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1807, _, _, _, _, _, _, _) => {
            h1807
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1808, _, _, _, _, _, _) => {
            h1808
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1811 = match v1.clone() {
                Adt2::C0(_, h1809, _, _, _, _, _, _, _, _, _) => {
                    h1809
                }
            };
            let a1812 = match v1.clone() {
                Adt2::C0(_, _, h1810, _, _, _, _, _, _, _, _) => {
                    h1810
                }
            };
            nat_mul(a1811, a1812)?
        };
        let v5: u64 = {
            let a1814 = v4;
            let a1815 = match v1.clone() {
                Adt2::C0(_, _, _, h1813, _, _, _, _, _, _, _) => {
                    h1813
                }
            };
            nat_mul(a1814, a1815)?
        };
        let a1828 = {
            let a1826 = {
                let a1817 = match v0.clone() {
                    Adt2::C0(_, h1816, _, _, _, _, _, _, _, _, _) => {
                        h1816
                    }
                };
                let a1818 = 1u64;
                nat_sub(a1817, a1818)
            };
            let a1827 = {
                let a1824 = f123(match v0.clone() {
                    Adt2::C0(_, _, h1819, _, _, _, _, _, _, _, _) => {
                        h1819
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1820, _, _, _, _, _, _, _, _, _) => {
                        h1820
                    }
                })?;
                let a1825 = f123(match v0 {
                    Adt2::C0(_, _, _, h1821, _, _, _, _, _, _, _) => {
                        h1821
                    }
                }, v2, {
                    let a1822 = v3;
                    let a1823 = v5;
                    nat_mul(a1822, a1823)?
                }, v4)?;
                nat_add(a1824, a1825)?
            };
            nat_add(a1826, a1827)?
        };
        let a1829 = f9(v1, 1u64)?;
        Ok(nat_lt(a1828, a1829))
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1830 = v0;
    let a1831 = 2147483648u64;
    if if if if nat_lt(a1830, a1831) {
        let a1832 = v1;
        let a1833 = 2147483648u64;
        nat_lt(a1832, a1833)
    } else {
        false
    } {
        let a1836 = {
            let a1834 = v0;
            let a1835 = v1;
            nat_mul(a1834, a1835)?
        };
        let a1837 = 2147483648u64;
        nat_lt(a1836, a1837)
    } else {
        false
    } {
        let a1842 = {
            let a1840 = {
                let a1838 = v0;
                let a1839 = 1u64;
                nat_sub(a1838, a1839)
            };
            let a1841 = v2;
            nat_mul(a1840, a1841)?
        };
        let a1843 = 2147483648u64;
        nat_lt(a1842, a1843)
    } else {
        false
    } {
        let a1848 = {
            let a1846 = {
                let a1844 = v1;
                let a1845 = 1u64;
                nat_sub(a1844, a1845)
            };
            let a1847 = v2;
            nat_mul(a1846, a1847)?
        };
        let a1849 = 2147483648u64;
        Ok(nat_lt(a1848, a1849))
    } else {
        Ok(false)
    }
}

pub fn f81(v0: u64) -> R<u64> {
    let a1852 = 3u64;
    let a1853 = {
        let a1850 = v0;
        let a1851 = 3u64;
        nat_mul(a1850, a1851)?
    };
    nat_add(a1852, a1853)
}

pub fn f82(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1854, _, _, _, _, _, _, _) => {
            h1854
        }
    }, match v0.clone() {
        Adt3::C0(_, h1855, _, _, _, _, _, _) => {
            h1855
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1856, _, _, _, _, _) => {
            h1856
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1857, _, _, _, _) => {
            h1857
        }
    }) {
        Ok(0u64)
    } else {
        let a1866 = match v0.clone() {
            Adt3::C0(h1858, _, _, _, _, _, _, _) => {
                h1858
            }
        };
        let a1867 = {
            let a1864 = match v0.clone() {
                Adt3::C0(_, h1859, _, _, _, _, _, _) => {
                    h1859
                }
            };
            let a1865 = {
                let a1862 = match v0.clone() {
                    Adt3::C0(_, _, h1860, _, _, _, _, _) => {
                        h1860
                    }
                };
                let a1863 = match v0 {
                    Adt3::C0(_, _, _, h1861, _, _, _, _) => {
                        h1861
                    }
                };
                nat_mul(a1862, a1863)?
            };
            nat_mul(a1864, a1865)?
        };
        nat_mul(a1866, a1867)
    }
}

pub fn f83(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1868 = v1;
    let a1869 = 13u64;
    if if nat_eq(a1868, a1869) {
        f23(match v0.clone() {
            Adt3::C0(h1870, _, _, _, _, _, _, _) => {
                h1870
            }
        }, match v0.clone() {
            Adt3::C0(_, h1871, _, _, _, _, _, _) => {
                h1871
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1872, _, _, _, _, _) => {
                h1872
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1873, _, _, _, _) => {
                h1873
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1879 = match v0.clone() {
                Adt3::C0(_, h1874, _, _, _, _, _, _) => {
                    h1874
                }
            };
            let a1880 = {
                let a1877 = match v0.clone() {
                    Adt3::C0(_, _, h1875, _, _, _, _, _) => {
                        h1875
                    }
                };
                let a1878 = match v0.clone() {
                    Adt3::C0(_, _, _, h1876, _, _, _, _) => {
                        h1876
                    }
                };
                nat_mul(a1877, a1878)?
            };
            nat_mul(a1879, a1880)?
        };
        let a1881 = v1;
        let a1882 = 13u64;
        if nat_eq(a1881, a1882) {
            let a1891 = 2u64;
            let a1892 = if f124(v0.clone(), v2) {
                let a1884 = v3;
                let a1885 = f125(match v0 {
                    Adt3::C0(h1883, _, _, _, _, _, _, _) => {
                        h1883
                    }
                }, false)?;
                nat_mul(a1884, a1885)?
            } else {
                let a1889 = v3;
                let a1890 = {
                    let a1887 = match v0 {
                        Adt3::C0(h1886, _, _, _, _, _, _, _) => {
                            h1886
                        }
                    };
                    let a1888 = 2u64;
                    nat_mul(a1887, a1888)?
                };
                nat_mul(a1889, a1890)?
            };
            nat_add(a1891, a1892)
        } else {
            let a1896 = v3;
            let a1897 = f125(match v0 {
                Adt3::C0(h1893, _, _, _, _, _, _, _) => {
                    h1893
                }
            }, {
                let a1894 = v1;
                let a1895 = 16u64;
                nat_eq(a1894, a1895)
            })?;
            nat_mul(a1896, a1897)
        }
    }
}

pub fn f84(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1898 = v3;
    match m1898 {
        None => {
            f126(v0, v1, match v2.clone() {
                Adt2::C0(h1899, _, _, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1900, _, _, _, _, _, _, _, _, _, _) => {
                    h1900
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1901, _, _, _, _, _, _, _, _, _) => {
                    h1901
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1902, _, _, _, _, _, _, _, _) => {
                    h1902
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1903, _, _, _, _, _, _, _) => {
                    h1903
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1904, _, _, _, _, _, _) => {
                    h1904
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1905, _, _, _, _, _, _, _, _, _) => {
                    h1905
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1906, _, _, _, _, _, _, _, _) => {
                    h1906
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1907, _, _, _, _, _, _, _) => {
                    h1907
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1908, _, _, _, _, _, _) => {
                    h1908
                }
            })
        }
        Some(v5) => {
            f126(v0, v1, match v2.clone() {
                Adt2::C0(h1909, _, _, _, _, _, _, _, _, _, _) => {
                    h1909
                }
            }, match v5.clone() {
                Adt2::C0(h1910, _, _, _, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v4.clone() {
                Adt2::C0(h1911, _, _, _, _, _, _, _, _, _, _) => {
                    h1911
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1912, _, _, _, _, _, _, _, _, _) => {
                    h1912
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1913, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1914, _, _, _, _, _, _, _) => {
                    h1914
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1915, _, _, _, _, _, _) => {
                    h1915
                }
            }, match v5.clone() {
                Adt2::C0(_, h1916, _, _, _, _, _, _, _, _, _) => {
                    h1916
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1917, _, _, _, _, _, _, _, _) => {
                    h1917
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1918, _, _, _, _, _, _, _) => {
                    h1918
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1919, _, _, _, _, _, _) => {
                    h1919
                }
            }, match v4.clone() {
                Adt2::C0(_, h1920, _, _, _, _, _, _, _, _, _) => {
                    h1920
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1921, _, _, _, _, _, _, _, _) => {
                    h1921
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1922, _, _, _, _, _, _, _) => {
                    h1922
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1923, _, _, _, _, _, _) => {
                    h1923
                }
            })
        }
    }
}

pub fn f85(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1924 = v7;
        let a1925 = 6u64;
        if nat_eq(a1924, a1925) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f127(v3, v4), f127(v5, v6), f128(v7, v8), f129(v1), f129(v9))
}

pub fn f86(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1926 = v0;
    match m1926 {
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
            let a1927 = 13u64;
            let a1928 = v34;
            if nat_le(a1927, a1928) {
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

pub fn f87(v0: Adt4) -> u64 {
    let m1929 = v0;
    match m1929 {
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

pub fn f88(v0: Adt4, v1: u64) -> u64 {
    let m1930 = v0;
    match m1930 {
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
            let a1931 = v1;
            let a1932 = 6u64;
            if nat_eq(a1931, a1932) {
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
            let a1933 = v1;
            let a1934 = 6u64;
            if if if nat_eq(a1933, a1934) {
                true
            } else {
                let a1935 = v1;
                let a1936 = 7u64;
                nat_eq(a1935, a1936)
            } {
                let a1937 = v49;
                let a1938 = 30u64;
                nat_eq(a1937, a1938)
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

pub fn f89(v0: Adt4, v1: u64) -> Adt7 {
    let a1939 = v1;
    let a1940 = 4u64;
    if nat_lt(a1939, a1940) {
        Adt7::C0(v1)
    } else {
        let a1941 = v1;
        let a1942 = 4u64;
        if nat_eq(a1941, a1942) {
            Adt7::C1(0u64)
        } else {
            let a1943 = v1;
            let a1944 = 5u64;
            if nat_eq(a1943, a1944) {
                Adt7::C1(3u64)
            } else {
                let a1945 = v1;
                let a1946 = 6u64;
                if nat_eq(a1945, a1946) {
                    Adt7::C2(f88(v0, v1), 0u64, false)
                } else {
                    let a1947 = v1;
                    let a1948 = 7u64;
                    if nat_eq(a1947, a1948) {
                        Adt7::C2(0u64, f88(v0, v1), false)
                    } else {
                        let a1949 = v1;
                        let a1950 = 16u64;
                        if nat_lt(a1949, a1950) {
                            Adt7::C1({
                                let a1951 = v1;
                                let a1952 = 8u64;
                                nat_sub(a1951, a1952)
                            })
                        } else {
                            let a1953 = v1;
                            let a1954 = 20u64;
                            if nat_lt(a1953, a1954) {
                                Adt7::C5({
                                    let a1955 = v1;
                                    let a1956 = 16u64;
                                    nat_sub(a1955, a1956)
                                })
                            } else {
                                let a1957 = v1;
                                let a1958 = 20u64;
                                if nat_eq(a1957, a1958) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1959 = v1;
                                    let a1960 = 21u64;
                                    if nat_eq(a1959, a1960) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1961 = v1;
                                        let a1962 = 22u64;
                                        if nat_eq(a1961, a1962) {
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

pub fn f90(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1964 = v0;
        let a1965 = match v1.clone() {
            Adt3::C0(h1963, _, _, _, _, _, _, _) => {
                h1963
            }
        };
        let a1966 = 0u64;
        nat_quot(a1964, a1965, a1966)
    };
    let v3: u64 = {
        let a1968 = v2;
        let a1969 = match v1.clone() {
            Adt3::C0(_, h1967, _, _, _, _, _, _) => {
                h1967
            }
        };
        let a1970 = 0u64;
        nat_quot(a1968, a1969, a1970)
    };
    f130({
        let a1972 = v0;
        let a1973 = match v1.clone() {
            Adt3::C0(h1971, _, _, _, _, _, _, _) => {
                h1971
            }
        };
        let a1974 = 0u64;
        nat_rem(a1972, a1973, a1974)
    }, {
        let a1976 = v2;
        let a1977 = match v1.clone() {
            Adt3::C0(_, h1975, _, _, _, _, _, _) => {
                h1975
            }
        };
        let a1978 = 0u64;
        nat_rem(a1976, a1977, a1978)
    }, {
        let a1980 = v3;
        let a1981 = match v1.clone() {
            Adt3::C0(_, _, h1979, _, _, _, _, _) => {
                h1979
            }
        };
        let a1982 = 0u64;
        nat_rem(a1980, a1981, a1982)
    }, {
        let a1984 = v3;
        let a1985 = match v1.clone() {
            Adt3::C0(_, _, h1983, _, _, _, _, _) => {
                h1983
            }
        };
        let a1986 = 0u64;
        nat_quot(a1984, a1985, a1986)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1987, _, _, _) => {
            h1987
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1988, _, _) => {
            h1988
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1989, _) => {
            h1989
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1990) => {
            h1990
        }
    })
}

pub fn f91(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2008 = {
        let a2003 = f131(v2, {
            let a1994 = {
                let a1991 = v0;
                let a1992 = v1;
                let a1993 = 0u64;
                nat_quot(a1991, a1992, a1993)
            };
            let a1995 = v2;
            let a1996 = 0u64;
            nat_rem(a1994, a1995, a1996)
        }, {
            let a2000 = {
                let a1997 = v0;
                let a1998 = v1;
                let a1999 = 0u64;
                nat_quot(a1997, a1998, a1999)
            };
            let a2001 = v2;
            let a2002 = 0u64;
            nat_quot(a2000, a2001, a2002)
        })?;
        let a2004 = v1;
        nat_mul(a2003, a2004)?
    };
    let a2009 = {
        let a2005 = v0;
        let a2006 = v1;
        let a2007 = 0u64;
        nat_rem(a2005, a2006, a2007)
    };
    nat_add(a2008, a2009)
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2011 = v0;
        let a2012 = match v2.clone() {
            Adt3::C0(h2010, _, _, _, _, _, _, _) => {
                h2010
            }
        };
        let a2013 = 0u64;
        nat_quot(a2011, a2012, a2013)
    };
    let v4: u64 = {
        let a2015 = v3;
        let a2016 = match v2.clone() {
            Adt3::C0(_, h2014, _, _, _, _, _, _) => {
                h2014
            }
        };
        let a2017 = 0u64;
        nat_quot(a2015, a2016, a2017)
    };
    f132({
        let a2019 = v0;
        let a2020 = match v2.clone() {
            Adt3::C0(h2018, _, _, _, _, _, _, _) => {
                h2018
            }
        };
        let a2021 = 0u64;
        nat_rem(a2019, a2020, a2021)
    }, {
        let a2023 = v3;
        let a2024 = match v2.clone() {
            Adt3::C0(_, h2022, _, _, _, _, _, _) => {
                h2022
            }
        };
        let a2025 = 0u64;
        nat_rem(a2023, a2024, a2025)
    }, {
        let a2027 = v4;
        let a2028 = match v2.clone() {
            Adt3::C0(_, _, h2026, _, _, _, _, _) => {
                h2026
            }
        };
        let a2029 = 0u64;
        nat_rem(a2027, a2028, a2029)
    }, {
        let a2031 = v4;
        let a2032 = match v2 {
            Adt3::C0(_, _, h2030, _, _, _, _, _) => {
                h2030
            }
        };
        let a2033 = 0u64;
        nat_quot(a2031, a2032, a2033)
    }, match v1.clone() {
        Adt3::C0(h2034, _, _, _, _, _, _, _) => {
            h2034
        }
    }, match v1.clone() {
        Adt3::C0(_, h2035, _, _, _, _, _, _) => {
            h2035
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2036, _, _, _, _, _) => {
            h2036
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2037, _, _, _, _) => {
            h2037
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2038, _, _, _) => {
            h2038
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2039, _, _) => {
            h2039
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2040, _) => {
            h2040
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2041) => {
            h2041
        }
    })
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2043 = v0;
        let a2044 = match v3.clone() {
            Adt3::C0(h2042, _, _, _, _, _, _, _) => {
                h2042
            }
        };
        let a2045 = 0u64;
        nat_quot(a2043, a2044, a2045)
    };
    let v6: u64 = {
        let a2047 = v5;
        let a2048 = match v3.clone() {
            Adt3::C0(_, h2046, _, _, _, _, _, _) => {
                h2046
            }
        };
        let a2049 = 0u64;
        nat_quot(a2047, a2048, a2049)
    };
    let v7: u64 = {
        let a2051 = v0;
        let a2052 = match v3.clone() {
            Adt3::C0(h2050, _, _, _, _, _, _, _) => {
                h2050
            }
        };
        let a2053 = 0u64;
        nat_rem(a2051, a2052, a2053)
    };
    let v8: u64 = {
        let a2055 = v5;
        let a2056 = match v3.clone() {
            Adt3::C0(_, h2054, _, _, _, _, _, _) => {
                h2054
            }
        };
        let a2057 = 0u64;
        nat_rem(a2055, a2056, a2057)
    };
    let v9: u64 = {
        let a2059 = v6;
        let a2060 = match v3.clone() {
            Adt3::C0(_, _, h2058, _, _, _, _, _) => {
                h2058
            }
        };
        let a2061 = 0u64;
        nat_rem(a2059, a2060, a2061)
    };
    let v10: u64 = {
        let a2063 = v6;
        let a2064 = match v3.clone() {
            Adt3::C0(_, _, h2062, _, _, _, _, _) => {
                h2062
            }
        };
        let a2065 = 0u64;
        nat_quot(a2063, a2064, a2065)
    };
    let v11: u64 = {
        let a2066 = v4;
        let a2067 = 0u64;
        if nat_eq(a2066, a2067) {
            match v1.clone() {
                Adt3::C0(h2068, _, _, _, _, _, _, _) => {
                    h2068
                }
            }
        } else {
            let a2069 = v4;
            let a2070 = 1u64;
            if nat_eq(a2069, a2070) {
                match v1.clone() {
                    Adt3::C0(_, h2071, _, _, _, _, _, _) => {
                        h2071
                    }
                }
            } else {
                let a2072 = v4;
                let a2073 = 2u64;
                if nat_eq(a2072, a2073) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2074, _, _, _, _, _) => {
                            h2074
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2075, _, _, _, _) => {
                            h2075
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2082 = v11;
        let a2083 = {
            let a2076 = v4;
            let a2077 = 0u64;
            if nat_eq(a2076, a2077) {
                v7
            } else {
                let a2078 = v4;
                let a2079 = 1u64;
                if nat_eq(a2078, a2079) {
                    v8
                } else {
                    let a2080 = v4;
                    let a2081 = 2u64;
                    if nat_eq(a2080, a2081) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2082, a2083)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f130(if if v12 {
        let a2084 = v4;
        let a2085 = 0u64;
        nat_eq(a2084, a2085)
    } else {
        false
    } {
        let a2086 = v7;
        let a2087 = v11;
        nat_sub(a2086, a2087)
    } else {
        v7
    }, if if v12 {
        let a2088 = v4;
        let a2089 = 1u64;
        nat_eq(a2088, a2089)
    } else {
        false
    } {
        let a2090 = v8;
        let a2091 = v11;
        nat_sub(a2090, a2091)
    } else {
        v8
    }, if if v12 {
        let a2092 = v4;
        let a2093 = 2u64;
        nat_eq(a2092, a2093)
    } else {
        false
    } {
        let a2094 = v9;
        let a2095 = v11;
        nat_sub(a2094, a2095)
    } else {
        v9
    }, if if v12 {
        let a2096 = v4;
        let a2097 = 3u64;
        nat_eq(a2096, a2097)
    } else {
        false
    } {
        let a2098 = v10;
        let a2099 = v11;
        nat_sub(a2098, a2099)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2100, _, _, _) => {
            h2100
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2101, _, _) => {
            h2101
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2102, _) => {
            h2102
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2103) => {
            h2103
        }
    })?, f90(v0, v3)?))
}

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f133(v0, v2.clone(), 0u64);
    let v9: u64 = f133(v0, v2.clone(), 1u64);
    let v10: u64 = f133(v0, v2.clone(), 2u64);
    let v11: u64 = f133(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f134(v1.clone(), f135(v8, v3, match v1.clone() {
            Adt3::C0(h2104, _, _, _, _, _, _, _) => {
                h2104
            }
        })?, f135(v9, v4, match v1.clone() {
            Adt3::C0(_, h2105, _, _, _, _, _, _) => {
                h2105
            }
        })?, f135(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2106, _, _, _, _, _) => {
                h2106
            }
        })?, f135(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2107, _, _, _, _) => {
                h2107
            }
        })?)?, f90(v0, v2)?))
    } else {
        if if if if f136(v8, v3, match v1.clone() {
            Adt3::C0(h2108, _, _, _, _, _, _, _) => {
                h2108
            }
        })? {
            f136(v9, v4, match v1.clone() {
                Adt3::C0(_, h2109, _, _, _, _, _, _) => {
                    h2109
                }
            })?
        } else {
            false
        } {
            f136(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2110, _, _, _, _, _) => {
                    h2110
                }
            })?
        } else {
            false
        } {
            f136(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2111, _, _, _, _) => {
                    h2111
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f134(v1, f137(v8, v3)?, f137(v9, v4)?, f137(v10, v5)?, f137(v11, v6)?)?, f90(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f90(v0, v2)?))
        }
    }
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f133(v0, v2.clone(), 0u64);
    let v5: u64 = f133(v0, v2.clone(), 1u64);
    let v6: u64 = f133(v0, v2.clone(), 2u64);
    let v7: u64 = f133(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f134(v1.clone(), f138(v4, v3, match v1 {
        Adt3::C0(h2112, _, _, _, _, _, _, _) => {
            h2112
        }
    })?, v5, v6, v7)?, f90(v0, v2)?))
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2113 = v0;
    let a2114 = v3;
    if nat_lt(a2113, a2114) {
        Ok(Adt5::C1(false, {
            let a2115 = v0;
            let a2116 = 4u64;
            nat_mul(a2115, a2116)?
        }, {
            let a2117 = v0;
            let a2118 = 4u64;
            nat_mul(a2117, a2118)?
        }))
    } else {
        let v6: u64 = {
            let a2119 = v0;
            let a2120 = v3;
            nat_sub(a2119, a2120)
        };
        let v7: u64 = f90(v6, v1)?;
        let v8: u64 = f139(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f97(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2121 = v0;
    let a2122 = v3;
    if nat_lt(a2121, a2122) {
        Ok(Adt5::C1(true, 0u64, f90(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2123 = v0;
            let a2124 = v3;
            nat_sub(a2123, a2124)
        };
        let v5: u64 = f92(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f90(v4, v1)?, 3u64, v5))
    }
}

pub fn f98(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2126 = v1;
        let a2127 = match v3.clone() {
            Adt3::C0(h2125, _, _, _, _, _, _, _) => {
                h2125
            }
        };
        let a2128 = 0u64;
        nat_rem(a2126, a2127, a2128)
    };
    let v5: u64 = {
        let a2134 = {
            let a2130 = v1;
            let a2131 = match v3.clone() {
                Adt3::C0(h2129, _, _, _, _, _, _, _) => {
                    h2129
                }
            };
            let a2132 = 0u64;
            nat_quot(a2130, a2131, a2132)
        };
        let a2135 = match v3.clone() {
            Adt3::C0(_, h2133, _, _, _, _, _, _) => {
                h2133
            }
        };
        let a2136 = 0u64;
        nat_rem(a2134, a2135, a2136)
    };
    let v6: bool = f140(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f92(v1, v2, v3.clone())?
    } else {
        0u64
    }, f90(v1, v3)?))
}

pub fn f99(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a2137 = v3;
        let a2138 = 8u64;
        nat_add(a2137, a2138)?
    }, 0u64, f90(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f92(v0, v5, v2.clone())?
    }, 3u64, f90(v0, v2)?))
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a2140 = v0;
        let a2141 = match v2.clone() {
            Adt3::C0(h2139, _, _, _, _, _, _, _) => {
                h2139
            }
        };
        let a2142 = 0u64;
        nat_rem(a2140, a2141, a2142)
    };
    let v7: u64 = {
        let a2144 = v0;
        let a2145 = match v2.clone() {
            Adt3::C0(h2143, _, _, _, _, _, _, _) => {
                h2143
            }
        };
        let a2146 = 0u64;
        nat_quot(a2144, a2145, a2146)
    };
    let v8: u64 = {
        let a2148 = v7;
        let a2149 = match v2.clone() {
            Adt3::C0(_, h2147, _, _, _, _, _, _) => {
                h2147
            }
        };
        let a2150 = 0u64;
        nat_rem(a2148, a2149, a2150)
    };
    let v9: u64 = {
        let a2152 = v7;
        let a2153 = match v2.clone() {
            Adt3::C0(_, h2151, _, _, _, _, _, _) => {
                h2151
            }
        };
        let a2154 = 0u64;
        nat_quot(a2152, a2153, a2154)
    };
    let v10: u64 = {
        let a2156 = v9;
        let a2157 = match v2.clone() {
            Adt3::C0(_, _, h2155, _, _, _, _, _) => {
                h2155
            }
        };
        let a2158 = 0u64;
        nat_rem(a2156, a2157, a2158)
    };
    let v11: u64 = {
        let a2160 = v9;
        let a2161 = match v2.clone() {
            Adt3::C0(_, _, h2159, _, _, _, _, _) => {
                h2159
            }
        };
        let a2162 = 0u64;
        nat_quot(a2160, a2161, a2162)
    };
    let v12: u64 = f90(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2171 = {
                let a2166 = {
                    let a2163 = v10;
                    let a2164 = v4;
                    let a2165 = 0u64;
                    nat_quot(a2163, a2164, a2165)
                };
                let a2167 = v5;
                nat_mul(a2166, a2167)?
            };
            let a2172 = {
                let a2168 = v8;
                let a2169 = v4;
                let a2170 = 0u64;
                nat_quot(a2168, a2169, a2170)
            };
            nat_add(a2171, a2172)?
        };
        let v14: u64 = {
            let a2192 = v6;
            let a2193 = {
                let a2190 = match v1.clone() {
                    Adt3::C0(h2173, _, _, _, _, _, _, _) => {
                        h2173
                    }
                };
                let a2191 = {
                    let a2188 = {
                        let a2174 = v8;
                        let a2175 = v4;
                        let a2176 = 0u64;
                        nat_rem(a2174, a2175, a2176)
                    };
                    let a2189 = {
                        let a2186 = match v1.clone() {
                            Adt3::C0(_, h2177, _, _, _, _, _, _) => {
                                h2177
                            }
                        };
                        let a2187 = {
                            let a2184 = {
                                let a2178 = v10;
                                let a2179 = v4;
                                let a2180 = 0u64;
                                nat_rem(a2178, a2179, a2180)
                            };
                            let a2185 = {
                                let a2182 = match v1.clone() {
                                    Adt3::C0(_, _, h2181, _, _, _, _, _) => {
                                        h2181
                                    }
                                };
                                let a2183 = v13;
                                nat_mul(a2182, a2183)?
                            };
                            nat_add(a2184, a2185)?
                        };
                        nat_mul(a2186, a2187)?
                    };
                    nat_add(a2188, a2189)?
                };
                nat_mul(a2190, a2191)?
            };
            nat_add(a2192, a2193)?
        };
        Ok(Adt5::C1(false, f90(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2199 = {
                let a2197 = {
                    let a2194 = v11;
                    let a2195 = v5;
                    let a2196 = 0u64;
                    nat_rem(a2194, a2195, a2196)
                };
                let a2198 = v4;
                nat_mul(a2197, a2198)?
            };
            let a2200 = v8;
            nat_add(a2199, a2200)?
        };
        let v16: u64 = {
            let a2206 = {
                let a2204 = {
                    let a2201 = v11;
                    let a2202 = v5;
                    let a2203 = 0u64;
                    nat_quot(a2201, a2202, a2203)
                };
                let a2205 = v4;
                nat_mul(a2204, a2205)?
            };
            let a2207 = v10;
            nat_add(a2206, a2207)?
        };
        let a2209 = v15;
        let a2210 = match v1.clone() {
            Adt3::C0(_, h2208, _, _, _, _, _, _) => {
                h2208
            }
        };
        if if nat_lt(a2209, a2210) {
            let a2212 = v16;
            let a2213 = match v1.clone() {
                Adt3::C0(_, _, h2211, _, _, _, _, _) => {
                    h2211
                }
            };
            nat_lt(a2212, a2213)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f90({
                let a2222 = v6;
                let a2223 = {
                    let a2220 = match v1.clone() {
                        Adt3::C0(h2214, _, _, _, _, _, _, _) => {
                            h2214
                        }
                    };
                    let a2221 = {
                        let a2218 = v15;
                        let a2219 = {
                            let a2216 = match v1.clone() {
                                Adt3::C0(_, h2215, _, _, _, _, _, _) => {
                                    h2215
                                }
                            };
                            let a2217 = v16;
                            nat_mul(a2216, a2217)?
                        };
                        nat_add(a2218, a2219)?
                    };
                    nat_mul(a2220, a2221)?
                };
                nat_add(a2222, a2223)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f90(v0, v2.clone())?;
    let v4: u64 = f90(v0, v1.clone())?;
    let a2228 = {
        let a2225 = v0;
        let a2226 = match v1 {
            Adt3::C0(h2224, _, _, _, _, _, _, _) => {
                h2224
            }
        };
        let a2227 = 0u64;
        nat_rem(a2225, a2226, a2227)
    };
    let a2229 = 0u64;
    if nat_eq(a2228, a2229) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f90({
            let a2230 = v0;
            let a2231 = 1u64;
            nat_sub(a2230, a2231)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f102(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2232 = v3;
    let a2233 = 13u64;
    if nat_eq(a2232, a2233) {
        f141(v0, v1, v4)
    } else {
        let v5: u64 = f125(match v1.clone() {
            Adt3::C0(h2234, _, _, _, _, _, _, _) => {
                h2234
            }
        }, {
            let a2235 = v3;
            let a2236 = 16u64;
            nat_eq(a2235, a2236)
        })?;
        let v6: u64 = {
            let a2237 = v0;
            let a2238 = v5;
            let a2239 = 0u64;
            nat_quot(a2237, a2238, a2239)
        };
        let v7: u64 = {
            let a2240 = v0;
            let a2241 = v5;
            let a2242 = 0u64;
            nat_rem(a2240, a2241, a2242)
        };
        let v8: u64 = {
            let a2246 = 1u64;
            let a2247 = {
                let a2244 = match v1.clone() {
                    Adt3::C0(h2243, _, _, _, _, _, _, _) => {
                        h2243
                    }
                };
                let a2245 = 2u64;
                nat_mul(a2244, a2245)?
            };
            nat_add(a2246, a2247)?
        };
        let a2248 = v7;
        let a2249 = v8;
        if nat_lt(a2248, a2249) {
            f142(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f90(v6, v2)?;
            let a2250 = v3;
            let a2251 = 14u64;
            if nat_eq(a2250, a2251) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2252 = v7;
                let a2253 = v8;
                if nat_eq(a2252, a2253) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2256 = v7;
                    let a2257 = {
                        let a2254 = v8;
                        let a2255 = 1u64;
                        nat_add(a2254, a2255)?
                    };
                    if nat_eq(a2256, a2257) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2258, _, _, _, _, _, _, _) => {
                                h2258
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

pub fn f103(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2259 = v0;
    let a2260 = 0u64;
    if nat_eq(a2259, a2260) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2267 = {
            let a2261 = v0;
            let a2262 = 1u64;
            nat_add(a2261, a2262)?
        };
        let a2268 = {
            let a2265 = 2u64;
            let a2266 = {
                let a2263 = v3;
                let a2264 = 2u64;
                nat_mul(a2263, a2264)?
            };
            nat_add(a2265, a2266)?
        };
        if nat_eq(a2267, a2268) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2269 = v0;
                let a2270 = 1u64;
                nat_sub(a2269, a2270)
            };
            let v5: u64 = {
                let a2271 = v4;
                let a2272 = 2u64;
                let a2273 = 0u64;
                nat_quot(a2271, a2272, a2273)
            };
            let a2277 = {
                let a2274 = v4;
                let a2275 = 2u64;
                let a2276 = 0u64;
                nat_rem(a2274, a2275, a2276)
            };
            let a2278 = 0u64;
            if nat_eq(a2277, a2278) {
                Ok(Adt5::C4(17u64, 0u64, f90(v5, v1)?, 4u64, f90(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2279 = v0;
    let a2280 = f18(v1, v2, v3)?;
    if nat_lt(a2279, a2280) {
        let a2281 = v0;
        let a2282 = v1;
        if nat_lt(a2281, a2282) {
            Ok(Adt5::C1(false, {
                let a2283 = 4u64;
                let a2284 = v0;
                nat_mul(a2283, a2284)?
            }, {
                let a2285 = 4u64;
                let a2286 = v0;
                nat_mul(a2285, a2286)?
            }))
        } else {
            let v4: u64 = {
                let a2287 = v0;
                let a2288 = v1;
                nat_sub(a2287, a2288)
            };
            let v5: u64 = f143(v4, v2)?;
            let a2289 = v5;
            let a2290 = 2u64;
            if nat_lt(a2289, a2290) {
                Ok(Adt5::C4(6u64, {
                    let a2291 = v5;
                    let a2292 = 0u64;
                    if nat_eq(a2291, a2292) {
                        4u64
                    } else {
                        5u64
                    }
                }, f144(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f145(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f146(v4, v2)? {
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

pub fn f105(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2300 = {
        let a2293 = 2u64;
        let a2294 = f82(v1.clone())?;
        nat_mul(a2293, a2294)?
    };
    let a2301 = {
        let a2298 = f82(v1)?;
        let a2299 = {
            let a2296 = 1u64;
            let a2297 = match v0 {
                Adt3::C0(h2295, _, _, _, _, _, _, _) => {
                    h2295
                }
            };
            nat_add(a2296, a2297)?
        };
        nat_mul(a2298, a2299)?
    };
    nat_add(a2300, a2301)
}

pub fn f106(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f90(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f107(v0: u64, v1: u64) -> bool {
    let a2302 = v1;
    let a2303 = 2147483647u64;
    if nat_le(a2302, a2303) {
        let a2304 = v1;
        let a2305 = v0;
        nat_lt(a2304, a2305)
    } else {
        false
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2306 = f71(v0);
    match m2306 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2308 = v2;
                let a2309 = match v7.clone() {
                    Adt11::C0(_, h2307, _) => {
                        h2307
                    }
                };
                nat_mul(a2308, a2309)?
            };
            let m2322 = f147(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2310, _, _, _, _, _, _, _) => {
                    h2310
                }
            }, match v5.clone() {
                Adt3::C0(_, h2311, _, _, _, _, _, _) => {
                    h2311
                }
            }, match v5 {
                Adt3::C0(_, _, h2312, _, _, _, _, _) => {
                    h2312
                }
            }, {
                let a2315 = match v3.clone() {
                    Adt3::C0(h2313, _, _, _, _, _, _, _) => {
                        h2313
                    }
                };
                let a2316 = match v7.clone() {
                    Adt11::C0(_, h2314, _) => {
                        h2314
                    }
                };
                nat_mul(a2315, a2316)?
            }, match v3.clone() {
                Adt3::C0(_, h2317, _, _, _, _, _, _) => {
                    h2317
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2318, _, _, _) => {
                    h2318
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2319, _, _) => {
                    h2319
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2320, _) => {
                    h2320
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2321) => {
                    h2321
                }
            })?;
            match m2322 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2323 = v0;
                    let a2324 = 0u64;
                    if if nat_eq(a2323, a2324) {
                        true
                    } else {
                        let a2325 = v0;
                        let a2326 = 26u64;
                        nat_eq(a2325, a2326)
                    } {
                        Ok(Adt5::C1(false, v9, f90({
                            let a2330 = {
                                let a2328 = v1;
                                let a2329 = match v4.clone() {
                                    Adt3::C0(h2327, _, _, _, _, _, _, _) => {
                                        h2327
                                    }
                                };
                                nat_mul(a2328, a2329)?
                            };
                            let a2331 = v8;
                            nat_add(a2330, a2331)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2332) => {
                                h2332
                            }
                        }, f90({
                            let a2336 = {
                                let a2334 = v1;
                                let a2335 = match v4.clone() {
                                    Adt3::C0(h2333, _, _, _, _, _, _, _) => {
                                        h2333
                                    }
                                };
                                nat_mul(a2334, a2335)?
                            };
                            let a2337 = v8;
                            nat_add(a2336, a2337)?
                        }, v4.clone())?, {
                            let a2344 = {
                                let a2342 = {
                                    let a2339 = match v7 {
                                        Adt11::C0(_, h2338, _) => {
                                            h2338
                                        }
                                    };
                                    let a2340 = 1u64;
                                    nat_sub(a2339, a2340)
                                };
                                let a2343 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2341, _, _, _) => {
                                        h2341
                                    }
                                };
                                nat_mul(a2342, a2343)?
                            };
                            let a2345 = 4u64;
                            nat_add(a2344, a2345)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2346, _, _, _) => {
                                h2346
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2347 = v1;
    let a2348 = 0u64;
    if if nat_eq(a2347, a2348) {
        true
    } else {
        let a2349 = v2;
        let a2350 = 0u64;
        nat_eq(a2349, a2350)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f110(v0: u64, v1: u64) -> bool {
    let a2351 = v0;
    let a2352 = 4294967295u64;
    if nat_eq(a2351, a2352) {
        true
    } else {
        let a2353 = v0;
        let a2354 = 2147483648u64;
        if nat_lt(a2353, a2354) {
            let a2355 = v0;
            let a2356 = v1;
            nat_lt(a2355, a2356)
        } else {
            false
        }
    }
}

pub fn f111(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2363 = v3;
        let a2364 = {
            let a2360 = f90({
                let a2357 = v0;
                let a2358 = 2u64;
                let a2359 = 0u64;
                nat_quot(a2357, a2358, a2359)
            }, v1)?;
            let a2361 = v2;
            let a2362 = 0u64;
            nat_quot(a2360, a2361, a2362)
        };
        nat_add(a2363, a2364)?
    };
    let a2368 = {
        let a2365 = v0;
        let a2366 = 2u64;
        let a2367 = 0u64;
        nat_rem(a2365, a2366, a2367)
    };
    let a2369 = 0u64;
    if nat_eq(a2368, a2369) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2370 = v4;
        let a2371 = 0u64;
        if nat_eq(a2370, a2371) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f112(v0: u64, v1: u64) -> R<u64> {
    let a2374 = 8u64;
    let a2375 = {
        let a2372 = f148(v0);
        let a2373 = v1;
        nat_mul(a2372, a2373)?
    };
    nat_add(a2374, a2375)
}

pub fn f113(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2376, _, _, _, _, _, _, _) => {
            h2376
        }
    };
    let v7: u64 = f148(v3);
    let v8: u64 = {
        let a2379 = 1u64;
        let a2380 = {
            let a2377 = v6;
            let a2378 = v7;
            nat_mul(a2377, a2378)?
        };
        nat_add(a2379, a2380)?
    };
    let a2381 = v0;
    let a2382 = 0u64;
    if nat_eq(a2381, a2382) {
        Ok(f149(0u64))
    } else {
        let a2383 = v0;
        let a2384 = v8;
        if nat_lt(a2383, a2384) {
            let v9: u64 = {
                let a2385 = v0;
                let a2386 = 1u64;
                nat_sub(a2385, a2386)
            };
            let v10: u64 = {
                let a2387 = v9;
                let a2388 = v7;
                let a2389 = 0u64;
                nat_rem(a2387, a2388, a2389)
            };
            let v11: u64 = f90({
                let a2395 = {
                    let a2390 = v1;
                    let a2391 = v6;
                    nat_mul(a2390, a2391)?
                };
                let a2396 = {
                    let a2392 = v9;
                    let a2393 = v7;
                    let a2394 = 0u64;
                    nat_quot(a2392, a2393, a2394)
                };
                nat_add(a2395, a2396)?
            }, v2)?;
            let a2397 = v3;
            let a2398 = 30u64;
            if nat_eq(a2397, a2398) {
                let a2399 = v10;
                let a2400 = 0u64;
                if nat_eq(a2399, a2400) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2401 = v10;
                    let a2402 = 1u64;
                    if nat_eq(a2401, a2402) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2403 = v10;
                        let a2404 = 2u64;
                        if nat_eq(a2403, a2404) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2405 = v10;
                let a2406 = 0u64;
                if nat_eq(a2405, a2406) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2407 = v10;
                    let a2408 = 1u64;
                    if nat_eq(a2407, a2408) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2409 = v0;
                let a2410 = v8;
                nat_sub(a2409, a2410)
            };
            let a2411 = v12;
            let a2412 = 0u64;
            if nat_eq(a2411, a2412) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2413 = v12;
                let a2414 = 1u64;
                if nat_eq(a2413, a2414) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2415 = v12;
                    let a2416 = 2u64;
                    if nat_eq(a2415, a2416) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2417 = v12;
                        let a2418 = 3u64;
                        if nat_eq(a2417, a2418) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2419 = v12;
                            let a2420 = 4u64;
                            if nat_eq(a2419, a2420) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2421 = v12;
                                let a2422 = 5u64;
                                if nat_eq(a2421, a2422) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2423 = v12;
                                    let a2424 = 6u64;
                                    if nat_eq(a2423, a2424) {
                                        if f150(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2425 = 3u64;
                                                let a2426 = v1;
                                                nat_add(a2425, a2426)?
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

pub fn f114(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a2427 = v0;
        let a2428 = f65(v3);
        let a2429 = 0u64;
        nat_quot(a2427, a2428, a2429)
    };
    let v5: u64 = {
        let a2434 = 3u64;
        let a2435 = {
            let a2431 = v4;
            let a2432 = match v1.clone() {
                Adt3::C0(h2430, _, _, _, _, _, _, _) => {
                    h2430
                }
            };
            let a2433 = 0u64;
            nat_quot(a2431, a2432, a2433)
        };
        nat_add(a2434, a2435)?
    };
    let v6: u64 = f90(v4, v1)?;
    let v7: u64 = f90(v4, v2)?;
    let a2436 = v3;
    let a2437 = 30u64;
    if nat_eq(a2436, a2437) {
        let a2441 = {
            let a2438 = v0;
            let a2439 = 3u64;
            let a2440 = 0u64;
            nat_rem(a2438, a2439, a2440)
        };
        let a2442 = 0u64;
        if nat_eq(a2441, a2442) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2446 = {
                let a2443 = v0;
                let a2444 = 3u64;
                let a2445 = 0u64;
                nat_rem(a2443, a2444, a2445)
            };
            let a2447 = 1u64;
            if nat_eq(a2446, a2447) {
                Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt5::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt5::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f115(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2448 = v0;
        let a2449 = 8388608u64;
        let a2450 = 0u64;
        nat_quot(a2448, a2449, a2450)
    };
    let a2451 = v0;
    let a2452 = 2147483648u64;
    if if if nat_lt(a2451, a2452) {
        let a2453 = 127u64;
        let a2454 = v1;
        nat_le(a2453, a2454)
    } else {
        false
    } {
        let a2455 = v1;
        let a2456 = 190u64;
        nat_lt(a2455, a2456)
    } else {
        false
    } {
        let v2: u64 = {
            let a2460 = 8388608u64;
            let a2461 = {
                let a2457 = v0;
                let a2458 = 8388608u64;
                let a2459 = 0u64;
                nat_rem(a2457, a2458, a2459)
            };
            nat_add(a2460, a2461)?
        };
        let a2462 = v1;
        let a2463 = 150u64;
        if nat_lt(a2462, a2463) {
            let v3: u64 = f151({
                let a2464 = 150u64;
                let a2465 = v1;
                nat_sub(a2464, a2465)
            });
            let a2469 = {
                let a2466 = v2;
                let a2467 = v3;
                let a2468 = 0u64;
                nat_rem(a2466, a2467, a2468)
            };
            let a2470 = 0u64;
            if nat_eq(a2469, a2470) {
                let a2471 = v2;
                let a2472 = v3;
                let a2473 = 0u64;
                Ok(nat_quot(a2471, a2472, a2473))
            } else {
                Ok(0u64)
            }
        } else {
            let a2476 = f151({
                let a2474 = v1;
                let a2475 = 150u64;
                nat_sub(a2474, a2475)
            });
            let a2477 = v2;
            nat_mul(a2476, a2477)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64) -> bool {
    let a2478 = v0;
    let a2479 = 1u64;
    if nat_le(a2478, a2479) {
        true
    } else {
        let a2483 = {
            let a2480 = v1;
            let a2481 = v2;
            let a2482 = 1u64;
            nat_rem(a2480, a2481, a2482)
        };
        let a2484 = 0u64;
        nat_eq(a2483, a2484)
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2485 = v1;
    let a2486 = 1u64;
    if if if nat_le(a2485, a2486) {
        true
    } else {
        let a2487 = v0;
        let a2488 = v2;
        nat_le(a2487, a2488)
    } {
        let a2489 = v3;
        let a2490 = 1u64;
        if nat_le(a2489, a2490) {
            true
        } else {
            let a2497 = {
                let a2495 = v0;
                let a2496 = {
                    let a2493 = {
                        let a2491 = v1;
                        let a2492 = 1u64;
                        nat_sub(a2491, a2492)
                    };
                    let a2494 = v2;
                    nat_mul(a2493, a2494)?
                };
                nat_add(a2495, a2496)?
            };
            let a2498 = v4;
            nat_le(a2497, a2498)
        }
    } else {
        false
    } {
        let a2499 = v5;
        let a2500 = 1u64;
        if nat_le(a2499, a2500) {
            Ok(true)
        } else {
            let a2513 = {
                let a2511 = {
                    let a2505 = v0;
                    let a2506 = {
                        let a2503 = {
                            let a2501 = v1;
                            let a2502 = 1u64;
                            nat_sub(a2501, a2502)
                        };
                        let a2504 = v2;
                        nat_mul(a2503, a2504)?
                    };
                    nat_add(a2505, a2506)?
                };
                let a2512 = {
                    let a2509 = {
                        let a2507 = v3;
                        let a2508 = 1u64;
                        nat_sub(a2507, a2508)
                    };
                    let a2510 = v4;
                    nat_mul(a2509, a2510)?
                };
                nat_add(a2511, a2512)?
            };
            let a2514 = v6;
            Ok(nat_le(a2513, a2514))
        }
    } else {
        Ok(false)
    }
}

pub fn f118(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f152(v0, v1, v6, v7, v8) {
        let a2515 = v5;
        let a2516 = 26u64;
        nat_eq(a2515, a2516)
    } else {
        false
    } {
        let a2517 = v3;
        let a2518 = v7;
        nat_eq(a2517, a2518)
    } else {
        false
    } {
        let a2519 = v4;
        let a2520 = v8;
        nat_eq(a2519, a2520)
    } else {
        false
    } {
        let a2521 = v9;
        let a2522 = 1u64;
        nat_eq(a2521, a2522)
    } else {
        false
    } {
        let a2523 = v10;
        let a2524 = f153(v0);
        nat_eq(a2523, a2524)
    } else {
        false
    } {
        let a2525 = v11;
        let a2526 = v1;
        nat_eq(a2525, a2526)
    } else {
        false
    } {
        let a2527 = v12;
        let a2528 = v6;
        nat_eq(a2527, a2528)
    } else {
        false
    } {
        let a2529 = v13;
        let a2530 = v7;
        nat_eq(a2529, a2530)
    } else {
        false
    } {
        let a2531 = v14;
        let a2532 = v8;
        nat_eq(a2531, a2532)
    } else {
        false
    }
}

pub fn f119(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2533 = v0;
    let a2534 = 0u64;
    if if if nat_eq(a2533, a2534) {
        true
    } else {
        let a2535 = v1;
        let a2536 = 0u64;
        nat_eq(a2535, a2536)
    } {
        true
    } else {
        let a2537 = v2;
        let a2538 = 0u64;
        nat_eq(a2537, a2538)
    } {
        true
    } else {
        let a2539 = v3;
        let a2540 = 0u64;
        nat_eq(a2539, a2540)
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2541 = v0;
    let a2542 = 2147483648u64;
    if nat_lt(a2541, a2542) {
        v0
    } else {
        0u64
    }
}

pub fn f121(v0: u64) -> u64 {
    let a2543 = v0;
    let a2544 = 2147483648u64;
    if nat_lt(a2543, a2544) {
        0u64
    } else {
        let a2545 = 4294967296u64;
        let a2546 = v0;
        nat_sub(a2545, a2546)
    }
}

pub fn f122(v0: Adt3, v1: bool) -> R<u64> {
    f154(v0.clone(), {
        let a2548 = match v0.clone() {
            Adt3::C0(h2547, _, _, _, _, _, _, _) => {
                h2547
            }
        };
        let a2549 = 1u64;
        nat_sub(a2548, a2549)
    }, {
        let a2551 = match v0.clone() {
            Adt3::C0(_, h2550, _, _, _, _, _, _) => {
                h2550
            }
        };
        let a2552 = 1u64;
        nat_sub(a2551, a2552)
    }, {
        let a2554 = match v0.clone() {
            Adt3::C0(_, _, h2553, _, _, _, _, _) => {
                h2553
            }
        };
        let a2555 = 1u64;
        nat_sub(a2554, a2555)
    }, {
        let a2557 = match v0 {
            Adt3::C0(_, _, _, h2556, _, _, _, _) => {
                h2556
            }
        };
        let a2558 = 1u64;
        nat_sub(a2557, a2558)
    }, v1)
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2559 = v0;
    let a2560 = 0u64;
    if nat_eq(a2559, a2560) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2561 = v0;
            let a2562 = 1u64;
            nat_sub(a2561, a2562)
        };
        let v5: u64 = {
            let a2563 = v4;
            let a2564 = v1;
            let a2565 = 0u64;
            nat_quot(a2563, a2564, a2565)
        };
        let v6: u64 = {
            let a2573 = {
                let a2566 = v5;
                let a2567 = v2;
                nat_mul(a2566, a2567)?
            };
            let a2574 = {
                let a2571 = {
                    let a2568 = v4;
                    let a2569 = v1;
                    let a2570 = 0u64;
                    nat_rem(a2568, a2569, a2570)
                };
                let a2572 = v3;
                nat_mul(a2571, a2572)?
            };
            nat_add(a2573, a2574)?
        };
        let a2575 = v5;
        let a2576 = 0u64;
        if nat_eq(a2575, a2576) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2585 = {
                    let a2579 = {
                        let a2577 = v5;
                        let a2578 = 1u64;
                        nat_sub(a2577, a2578)
                    };
                    let a2580 = v2;
                    nat_mul(a2579, a2580)?
                };
                let a2586 = {
                    let a2583 = {
                        let a2581 = v1;
                        let a2582 = 1u64;
                        nat_sub(a2581, a2582)
                    };
                    let a2584 = v3;
                    nat_mul(a2583, a2584)?
                };
                nat_add(a2585, a2586)?
            };
            let a2587 = v6;
            let a2588 = v7;
            if nat_lt(a2587, a2588) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f124(v0: Adt3, v1: u64) -> bool {
    let a2589 = v1;
    let a2590 = 0u64;
    if !nat_eq(a2589, a2590) {
        true
    } else {
        let a2592 = match v0 {
            Adt3::C0(_, _, _, _, h2591, _, _, _) => {
                h2591
            }
        };
        let a2593 = 4u64;
        nat_eq(a2592, a2593)
    }
}

pub fn f125(v0: u64, v1: bool) -> R<u64> {
    let a2596 = {
        let a2594 = v0;
        let a2595 = 2u64;
        nat_mul(a2594, a2595)?
    };
    let a2597 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2596, a2597)
}

pub fn f126(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2598 = f155(v0, v1);
    match m2598 {
        None => {
            if v5 {
                let m2599 = f156(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2599 {
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
                let a2600 = 4u64;
                let a2601 = v28;
                nat_le(a2600, a2601)
            };
            let a2602 = v2;
            let a2603 = 0u64;
            if if if if if nat_eq(a2602, a2603) {
                let a2604 = v4;
                let a2605 = 0u64;
                nat_eq(a2604, a2605)
            } else {
                false
            } {
                let a2606 = v6;
                let a2607 = v14;
                if nat_eq(a2606, a2607) {
                    let a2608 = v7;
                    let a2609 = v15;
                    nat_eq(a2608, a2609)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2610 = v8;
                let a2611 = v16;
                if nat_eq(a2610, a2611) {
                    let a2612 = v9;
                    let a2613 = v17;
                    nat_eq(a2612, a2613)
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
                        let a2614 = v3;
                        let a2615 = 0u64;
                        nat_eq(a2614, a2615)
                    } else {
                        false
                    } {
                        f72(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2620 = {
                        let a2616 = v6;
                        let a2617 = v7;
                        nat_mul(a2616, a2617)?
                    };
                    let a2621 = {
                        let a2618 = v8;
                        let a2619 = v9;
                        nat_mul(a2618, a2619)?
                    };
                    nat_mul(a2620, a2621)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f127(v0: u64, v1: u64) -> Adt8 {
    let a2622 = v0;
    let a2623 = 0u64;
    if nat_eq(a2622, a2623) {
        Adt8::C0(0u64, v1)
    } else {
        let a2624 = v0;
        let a2625 = 1u64;
        if nat_eq(a2624, a2625) {
            Adt8::C1(v1)
        } else {
            let a2626 = v0;
            let a2627 = 2u64;
            if nat_eq(a2626, a2627) {
                Adt8::C2(v1)
            } else {
                let a2628 = v0;
                let a2629 = 3u64;
                if nat_eq(a2628, a2629) {
                    Adt8::C3(v1)
                } else {
                    let a2630 = v0;
                    let a2631 = 13u64;
                    if nat_lt(a2630, a2631) {
                        Adt8::C0({
                            let a2632 = v0;
                            let a2633 = 3u64;
                            nat_sub(a2632, a2633)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f128(v0: u64, v1: u64) -> Adt9 {
    let a2634 = v0;
    let a2635 = 1u64;
    if nat_eq(a2634, a2635) {
        Adt9::C0(v1)
    } else {
        let a2636 = v0;
        let a2637 = 3u64;
        if if nat_eq(a2636, a2637) {
            true
        } else {
            let a2638 = v0;
            let a2639 = 6u64;
            nat_eq(a2638, a2639)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f129(v0: u64) -> u64 {
    let m2640 = f71(v0);
    match m2640 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2642 = match v1.clone() {
                Adt11::C0(_, h2641, _) => {
                    h2641
                }
            };
            let a2643 = 1u64;
            if if nat_eq(a2642, a2643) {
                let a2645 = match v1.clone() {
                    Adt11::C0(_, _, h2644) => {
                        h2644
                    }
                };
                let a2646 = 1u64;
                if if if nat_eq(a2645, a2646) {
                    true
                } else {
                    let a2648 = match v1.clone() {
                        Adt11::C0(_, _, h2647) => {
                            h2647
                        }
                    };
                    let a2649 = 2u64;
                    nat_eq(a2648, a2649)
                } {
                    true
                } else {
                    let a2651 = match v1.clone() {
                        Adt11::C0(_, _, h2650) => {
                            h2650
                        }
                    };
                    let a2652 = 4u64;
                    nat_eq(a2651, a2652)
                } {
                    true
                } else {
                    let a2654 = match v1.clone() {
                        Adt11::C0(_, _, h2653) => {
                            h2653
                        }
                    };
                    let a2655 = 8u64;
                    nat_eq(a2654, a2655)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2656) => {
                        h2656
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2669 = {
        let a2665 = {
            let a2661 = {
                let a2657 = v0;
                let a2658 = v4;
                nat_mul(a2657, a2658)?
            };
            let a2662 = {
                let a2659 = v1;
                let a2660 = v5;
                nat_mul(a2659, a2660)?
            };
            nat_add(a2661, a2662)?
        };
        let a2666 = {
            let a2663 = v2;
            let a2664 = v6;
            nat_mul(a2663, a2664)?
        };
        nat_add(a2665, a2666)?
    };
    let a2670 = {
        let a2667 = v3;
        let a2668 = v7;
        nat_mul(a2667, a2668)?
    };
    nat_add(a2669, a2670)
}

pub fn f131(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2675 = {
        let a2673 = {
            let a2671 = v0;
            let a2672 = v1;
            nat_sub(a2671, a2672)
        };
        let a2674 = 1u64;
        nat_sub(a2673, a2674)
    };
    let a2676 = v2;
    nat_add(a2675, a2676)
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f130({
        let a2677 = v0;
        let a2678 = v4;
        let a2679 = 0u64;
        nat_rem(a2677, a2678, a2679)
    }, {
        let a2680 = v1;
        let a2681 = v5;
        let a2682 = 0u64;
        nat_rem(a2680, a2681, a2682)
    }, {
        let a2683 = v2;
        let a2684 = v6;
        let a2685 = 0u64;
        nat_rem(a2683, a2684, a2685)
    }, {
        let a2686 = v3;
        let a2687 = v7;
        let a2688 = 0u64;
        nat_rem(a2686, a2687, a2688)
    }, v8, v9, v10, v11)
}

pub fn f133(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2690 = v0;
        let a2691 = match v1.clone() {
            Adt3::C0(h2689, _, _, _, _, _, _, _) => {
                h2689
            }
        };
        let a2692 = 0u64;
        nat_quot(a2690, a2691, a2692)
    };
    let v4: u64 = {
        let a2694 = v3;
        let a2695 = match v1.clone() {
            Adt3::C0(_, h2693, _, _, _, _, _, _) => {
                h2693
            }
        };
        let a2696 = 0u64;
        nat_quot(a2694, a2695, a2696)
    };
    let a2697 = v2;
    let a2698 = 0u64;
    if nat_eq(a2697, a2698) {
        let a2700 = v0;
        let a2701 = match v1 {
            Adt3::C0(h2699, _, _, _, _, _, _, _) => {
                h2699
            }
        };
        let a2702 = 0u64;
        nat_rem(a2700, a2701, a2702)
    } else {
        let a2703 = v2;
        let a2704 = 1u64;
        if nat_eq(a2703, a2704) {
            let a2706 = v3;
            let a2707 = match v1 {
                Adt3::C0(_, h2705, _, _, _, _, _, _) => {
                    h2705
                }
            };
            let a2708 = 0u64;
            nat_rem(a2706, a2707, a2708)
        } else {
            let a2709 = v2;
            let a2710 = 2u64;
            if nat_eq(a2709, a2710) {
                let a2712 = v4;
                let a2713 = match v1 {
                    Adt3::C0(_, _, h2711, _, _, _, _, _) => {
                        h2711
                    }
                };
                let a2714 = 0u64;
                nat_rem(a2712, a2713, a2714)
            } else {
                let a2716 = v4;
                let a2717 = match v1 {
                    Adt3::C0(_, _, h2715, _, _, _, _, _) => {
                        h2715
                    }
                };
                let a2718 = 0u64;
                nat_quot(a2716, a2717, a2718)
            }
        }
    }
}

pub fn f134(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f130(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2719, _, _, _) => {
            h2719
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2720, _, _) => {
            h2720
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2721, _) => {
            h2721
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2722) => {
            h2722
        }
    })
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2723 = v1;
    let a2724 = 2147483648u64;
    if nat_lt(a2723, a2724) {
        let a2735 = {
            let a2733 = {
                let a2728 = {
                    let a2725 = v0;
                    let a2726 = v2;
                    let a2727 = 0u64;
                    nat_rem(a2725, a2726, a2727)
                };
                let a2729 = v2;
                nat_add(a2728, a2729)?
            };
            let a2734 = {
                let a2730 = v1;
                let a2731 = v2;
                let a2732 = 0u64;
                nat_rem(a2730, a2731, a2732)
            };
            nat_sub(a2733, a2734)
        };
        let a2736 = v2;
        let a2737 = 0u64;
        Ok(nat_rem(a2735, a2736, a2737))
    } else {
        let a2740 = {
            let a2738 = v0;
            let a2739 = f121(v1);
            nat_add(a2738, a2739)?
        };
        let a2741 = v2;
        let a2742 = 0u64;
        Ok(nat_rem(a2740, a2741, a2742))
    }
}

pub fn f136(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2745 = f120(v1);
    let a2746 = {
        let a2743 = v0;
        let a2744 = f121(v1);
        nat_add(a2743, a2744)?
    };
    if nat_le(a2745, a2746) {
        let a2747 = f137(v0, v1)?;
        let a2748 = v2;
        Ok(nat_lt(a2747, a2748))
    } else {
        Ok(false)
    }
}

pub fn f137(v0: u64, v1: u64) -> R<u64> {
    let a2751 = {
        let a2749 = v0;
        let a2750 = f121(v1);
        nat_add(a2749, a2750)?
    };
    let a2752 = f120(v1);
    Ok(nat_sub(a2751, a2752))
}

pub fn f138(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2753 = v0;
    let a2754 = v1;
    if nat_lt(a2753, a2754) {
        let a2755 = v1;
        let a2756 = v0;
        Ok(nat_sub(a2755, a2756))
    } else {
        let a2759 = v0;
        let a2760 = {
            let a2757 = v1;
            let a2758 = v2;
            nat_add(a2757, a2758)?
        };
        if nat_lt(a2759, a2760) {
            let a2761 = v0;
            let a2762 = v1;
            Ok(nat_sub(a2761, a2762))
        } else {
            let a2769 = {
                let a2765 = 2u64;
                let a2766 = {
                    let a2763 = v2;
                    let a2764 = 1u64;
                    nat_sub(a2763, a2764)
                };
                nat_mul(a2765, a2766)?
            };
            let a2770 = {
                let a2767 = v0;
                let a2768 = v1;
                nat_sub(a2767, a2768)
            };
            Ok(nat_sub(a2769, a2770))
        }
    }
}

pub fn f139(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2773 = {
        let a2771 = v2;
        let a2772 = f154(v1.clone(), f133(v0, v1.clone(), 0u64), f133(v0, v1.clone(), 1u64), f133(v0, v1.clone(), 2u64), f133(v0, v1.clone(), 3u64), false)?;
        nat_add(a2771, a2772)?
    };
    let a2774 = f154(v1.clone(), f133(v0, v1.clone(), 0u64), f133(v0, v1.clone(), 1u64), f133(v0, v1.clone(), 2u64), f133(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2773, a2774))
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> bool {
    let a2775 = v0;
    let a2776 = 0u64;
    if nat_eq(a2775, a2776) {
        let a2777 = v2;
        let a2778 = v1;
        nat_le(a2777, a2778)
    } else {
        let a2779 = v0;
        let a2780 = 1u64;
        if nat_eq(a2779, a2780) {
            let a2781 = v2;
            let a2782 = v1;
            nat_lt(a2781, a2782)
        } else {
            let a2783 = v0;
            let a2784 = 2u64;
            if nat_eq(a2783, a2784) {
                let a2785 = v1;
                let a2786 = v2;
                nat_le(a2785, a2786)
            } else {
                let a2787 = v0;
                let a2788 = 3u64;
                if nat_eq(a2787, a2788) {
                    let a2789 = v1;
                    let a2790 = v2;
                    nat_lt(a2789, a2790)
                } else {
                    let a2791 = v1;
                    let a2792 = v2;
                    nat_eq(a2791, a2792)
                }
            }
        }
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2793 = v0;
    let a2794 = 0u64;
    if nat_eq(a2793, a2794) {
        Ok(f149(0u64))
    } else {
        let a2797 = {
            let a2795 = v0;
            let a2796 = 1u64;
            nat_add(a2795, a2796)?
        };
        let a2798 = f83(v1.clone(), 13u64, v2)?;
        if nat_eq(a2797, a2798) {
            Ok(Adt5::C4({
                let a2799 = v2;
                let a2800 = 0u64;
                if nat_eq(a2799, a2800) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2801 = v0;
                let a2802 = 1u64;
                nat_sub(a2801, a2802)
            };
            if f124(v1.clone(), v2) {
                let v4: u64 = f125(match v1.clone() {
                    Adt3::C0(h2803, _, _, _, _, _, _, _) => {
                        h2803
                    }
                }, false)?;
                let v5: u64 = {
                    let a2804 = v3;
                    let a2805 = v4;
                    let a2806 = 0u64;
                    nat_quot(a2804, a2805, a2806)
                };
                let v6: u64 = {
                    let a2807 = v3;
                    let a2808 = v4;
                    let a2809 = 0u64;
                    nat_rem(a2807, a2808, a2809)
                };
                let a2812 = {
                    let a2810 = v6;
                    let a2811 = 1u64;
                    nat_add(a2810, a2811)?
                };
                let a2813 = v4;
                if nat_eq(a2812, a2813) {
                    Ok(Adt5::C4({
                        let a2814 = v2;
                        let a2815 = 0u64;
                        if nat_eq(a2814, a2815) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f142(v6, v5, v1, v2)
                }
            } else {
                f157({
                    let a2816 = v3;
                    let a2817 = 2u64;
                    let a2818 = 0u64;
                    nat_quot(a2816, a2817, a2818)
                }, {
                    let a2819 = v3;
                    let a2820 = 2u64;
                    let a2821 = 0u64;
                    nat_rem(a2819, a2820, a2821)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f142(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2822 = v0;
    let a2823 = 0u64;
    if nat_eq(a2822, a2823) {
        Ok(f149(1u64))
    } else {
        f157({
            let a2832 = {
                let a2825 = v1;
                let a2826 = match v2.clone() {
                    Adt3::C0(h2824, _, _, _, _, _, _, _) => {
                        h2824
                    }
                };
                nat_mul(a2825, a2826)?
            };
            let a2833 = {
                let a2829 = {
                    let a2827 = v0;
                    let a2828 = 1u64;
                    nat_sub(a2827, a2828)
                };
                let a2830 = 2u64;
                let a2831 = 0u64;
                nat_quot(a2829, a2830, a2831)
            };
            nat_add(a2832, a2833)?
        }, {
            let a2836 = {
                let a2834 = v0;
                let a2835 = 1u64;
                nat_sub(a2834, a2835)
            };
            let a2837 = 2u64;
            let a2838 = 0u64;
            nat_rem(a2836, a2837, a2838)
        }, v2, v3, 1u64)
    }
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2839 = v0;
    let a2840 = f60(v1)?;
    let a2841 = 0u64;
    Ok(nat_rem(a2839, a2840, a2841))
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2842 = 4u64;
    let a2843 = f158(v0, v1)?;
    nat_mul(a2842, a2843)
}

pub fn f145(v0: u64, v1: u64) -> R<u64> {
    let a2844 = 4u64;
    let a2845 = f159(v0, v1)?;
    nat_mul(a2844, a2845)
}

pub fn f146(v0: u64, v1: u64) -> R<bool> {
    let a2851 = {
        let a2848 = {
            let a2846 = f143(v0, v1)?;
            let a2847 = 2u64;
            nat_sub(a2846, a2847)
        };
        let a2849 = 2u64;
        let a2850 = 0u64;
        nat_rem(a2848, a2849, a2850)
    };
    let a2852 = 0u64;
    Ok(nat_eq(a2851, a2852))
}

pub fn f147(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2853 = f71(v0);
    match m2853 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f160(v0) {
                let a2854 = v1;
                let a2855 = v7;
                nat_lt(a2854, a2855)
            } else {
                false
            } {
                f107(v8, v2)
            } else {
                false
            } {
                let a2856 = v3;
                let a2857 = f161(v4, v5, v6)?;
                nat_lt(a2856, a2857)
            } else {
                false
            } {
                let a2862 = {
                    let a2859 = v7;
                    let a2860 = match v13.clone() {
                        Adt11::C0(_, h2858, _) => {
                            h2858
                        }
                    };
                    let a2861 = 0u64;
                    nat_rem(a2859, a2860, a2861)
                };
                let a2863 = 0u64;
                nat_eq(a2862, a2863)
            } else {
                false
            } {
                Ok(Some({
                    let a2892 = {
                        let a2882 = {
                            let a2872 = {
                                let a2868 = {
                                    let a2865 = v1;
                                    let a2866 = match v13 {
                                        Adt11::C0(_, h2864, _) => {
                                            h2864
                                        }
                                    };
                                    let a2867 = 0u64;
                                    nat_quot(a2865, a2866, a2867)
                                };
                                let a2869 = v9;
                                nat_mul(a2868, a2869)?
                            };
                            let a2873 = {
                                let a2870 = v2;
                                let a2871 = v10;
                                nat_mul(a2870, a2871)?
                            };
                            nat_add(a2872, a2873)?
                        };
                        let a2883 = {
                            let a2880 = {
                                let a2877 = {
                                    let a2874 = v3;
                                    let a2875 = v4;
                                    let a2876 = 0u64;
                                    nat_quot(a2874, a2875, a2876)
                                };
                                let a2878 = v5;
                                let a2879 = 0u64;
                                nat_rem(a2877, a2878, a2879)
                            };
                            let a2881 = v11;
                            nat_mul(a2880, a2881)?
                        };
                        nat_add(a2882, a2883)?
                    };
                    let a2893 = {
                        let a2890 = {
                            let a2887 = {
                                let a2884 = v3;
                                let a2885 = v4;
                                let a2886 = 0u64;
                                nat_quot(a2884, a2885, a2886)
                            };
                            let a2888 = v5;
                            let a2889 = 0u64;
                            nat_quot(a2887, a2888, a2889)
                        };
                        let a2891 = v12;
                        nat_mul(a2890, a2891)?
                    };
                    nat_add(a2892, a2893)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f148(v0: u64) -> u64 {
    let a2894 = v0;
    let a2895 = 30u64;
    if nat_eq(a2894, a2895) {
        4u64
    } else {
        3u64
    }
}

pub fn f149(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f150(v0: u64) -> bool {
    let a2896 = 0u64;
    let a2897 = v0;
    if nat_lt(a2896, a2897) {
        let a2898 = v0;
        let a2899 = 2139095040u64;
        nat_le(a2898, a2899)
    } else {
        false
    }
}

pub fn f151(v0: u64) -> u64 {
    let a2900 = v0;
    let a2901 = 40u64;
    if nat_lt(a2900, a2901) {
        let a2902 = v0;
        let a2903 = 20u64;
        if nat_lt(a2902, a2903) {
            let a2904 = v0;
            let a2905 = 10u64;
            if nat_lt(a2904, a2905) {
                let a2906 = v0;
                let a2907 = 5u64;
                if nat_lt(a2906, a2907) {
                    let a2908 = v0;
                    let a2909 = 2u64;
                    if nat_lt(a2908, a2909) {
                        let a2910 = v0;
                        let a2911 = 1u64;
                        if nat_lt(a2910, a2911) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2912 = v0;
                        let a2913 = 3u64;
                        if nat_lt(a2912, a2913) {
                            4u64
                        } else {
                            let a2914 = v0;
                            let a2915 = 4u64;
                            if nat_lt(a2914, a2915) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2916 = v0;
                    let a2917 = 7u64;
                    if nat_lt(a2916, a2917) {
                        let a2918 = v0;
                        let a2919 = 6u64;
                        if nat_lt(a2918, a2919) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2920 = v0;
                        let a2921 = 8u64;
                        if nat_lt(a2920, a2921) {
                            128u64
                        } else {
                            let a2922 = v0;
                            let a2923 = 9u64;
                            if nat_lt(a2922, a2923) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2924 = v0;
                let a2925 = 15u64;
                if nat_lt(a2924, a2925) {
                    let a2926 = v0;
                    let a2927 = 12u64;
                    if nat_lt(a2926, a2927) {
                        let a2928 = v0;
                        let a2929 = 11u64;
                        if nat_lt(a2928, a2929) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2930 = v0;
                        let a2931 = 13u64;
                        if nat_lt(a2930, a2931) {
                            4096u64
                        } else {
                            let a2932 = v0;
                            let a2933 = 14u64;
                            if nat_lt(a2932, a2933) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2934 = v0;
                    let a2935 = 17u64;
                    if nat_lt(a2934, a2935) {
                        let a2936 = v0;
                        let a2937 = 16u64;
                        if nat_lt(a2936, a2937) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2938 = v0;
                        let a2939 = 18u64;
                        if nat_lt(a2938, a2939) {
                            131072u64
                        } else {
                            let a2940 = v0;
                            let a2941 = 19u64;
                            if nat_lt(a2940, a2941) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2942 = v0;
            let a2943 = 30u64;
            if nat_lt(a2942, a2943) {
                let a2944 = v0;
                let a2945 = 25u64;
                if nat_lt(a2944, a2945) {
                    let a2946 = v0;
                    let a2947 = 22u64;
                    if nat_lt(a2946, a2947) {
                        let a2948 = v0;
                        let a2949 = 21u64;
                        if nat_lt(a2948, a2949) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2950 = v0;
                        let a2951 = 23u64;
                        if nat_lt(a2950, a2951) {
                            4194304u64
                        } else {
                            let a2952 = v0;
                            let a2953 = 24u64;
                            if nat_lt(a2952, a2953) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2954 = v0;
                    let a2955 = 27u64;
                    if nat_lt(a2954, a2955) {
                        let a2956 = v0;
                        let a2957 = 26u64;
                        if nat_lt(a2956, a2957) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2958 = v0;
                        let a2959 = 28u64;
                        if nat_lt(a2958, a2959) {
                            134217728u64
                        } else {
                            let a2960 = v0;
                            let a2961 = 29u64;
                            if nat_lt(a2960, a2961) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2962 = v0;
                let a2963 = 35u64;
                if nat_lt(a2962, a2963) {
                    let a2964 = v0;
                    let a2965 = 32u64;
                    if nat_lt(a2964, a2965) {
                        let a2966 = v0;
                        let a2967 = 31u64;
                        if nat_lt(a2966, a2967) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2968 = v0;
                        let a2969 = 33u64;
                        if nat_lt(a2968, a2969) {
                            4294967296u64
                        } else {
                            let a2970 = v0;
                            let a2971 = 34u64;
                            if nat_lt(a2970, a2971) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2972 = v0;
                    let a2973 = 37u64;
                    if nat_lt(a2972, a2973) {
                        let a2974 = v0;
                        let a2975 = 36u64;
                        if nat_lt(a2974, a2975) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2976 = v0;
                        let a2977 = 38u64;
                        if nat_lt(a2976, a2977) {
                            137438953472u64
                        } else {
                            let a2978 = v0;
                            let a2979 = 39u64;
                            if nat_lt(a2978, a2979) {
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

pub fn f152(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f119(v1, v2, v3, v4) {
        f162(v0)
    } else {
        f160(v0)
    }
}

pub fn f153(v0: u64) -> u64 {
    let a2980 = v0;
    let a2981 = 26u64;
    if nat_eq(a2980, a2981) {
        26u64
    } else {
        0u64
    }
}

pub fn f154(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3002 = {
        let a2996 = {
            let a2990 = {
                let a2984 = v1;
                let a2985 = if v5 {
                    f121(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2982, _, _, _) => {
                            h2982
                        }
                    })
                } else {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2983, _, _, _) => {
                            h2983
                        }
                    })
                };
                nat_mul(a2984, a2985)?
            };
            let a2991 = {
                let a2988 = v2;
                let a2989 = if v5 {
                    f121(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2986, _, _) => {
                            h2986
                        }
                    })
                } else {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2987, _, _) => {
                            h2987
                        }
                    })
                };
                nat_mul(a2988, a2989)?
            };
            nat_add(a2990, a2991)?
        };
        let a2997 = {
            let a2994 = v3;
            let a2995 = if v5 {
                f121(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2992, _) => {
                        h2992
                    }
                })
            } else {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2993, _) => {
                        h2993
                    }
                })
            };
            nat_mul(a2994, a2995)?
        };
        nat_add(a2996, a2997)?
    };
    let a3003 = {
        let a3000 = v4;
        let a3001 = if v5 {
            f121(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2998) => {
                    h2998
                }
            })
        } else {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2999) => {
                    h2999
                }
            })
        };
        nat_mul(a3000, a3001)?
    };
    nat_add(a3002, a3003)
}

pub fn f155(v0: u64, v1: u64) -> Option<u64> {
    let a3004 = v0;
    let a3005 = 2u64;
    if nat_eq(a3004, a3005) {
        Some(0u64)
    } else {
        let a3006 = v0;
        let a3007 = 5u64;
        if nat_eq(a3006, a3007) {
            Some(1u64)
        } else {
            let a3008 = v0;
            let a3009 = 6u64;
            if nat_eq(a3008, a3009) {
                Some(2u64)
            } else {
                let a3010 = v0;
                let a3011 = 7u64;
                if nat_eq(a3010, a3011) {
                    Some(3u64)
                } else {
                    let a3012 = v0;
                    let a3013 = 9u64;
                    if nat_eq(a3012, a3013) {
                        Some(4u64)
                    } else {
                        let a3014 = v0;
                        let a3015 = 8u64;
                        if nat_eq(a3014, a3015) {
                            Some(5u64)
                        } else {
                            let a3016 = v0;
                            let a3017 = 89u64;
                            if nat_eq(a3016, a3017) {
                                let a3018 = v1;
                                let a3019 = 2u64;
                                if nat_eq(a3018, a3019) {
                                    Some(6u64)
                                } else {
                                    let a3020 = v1;
                                    let a3021 = 0u64;
                                    if nat_eq(a3020, a3021) {
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

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    if if if f163(v1) {
        f163(v2)
    } else {
        false
    } {
        let a3022 = v1;
        let a3023 = 0u64;
        if if nat_eq(a3022, a3023) {
            true
        } else {
            let a3024 = v2;
            let a3025 = 0u64;
            nat_eq(a3024, a3025)
        } {
            true
        } else {
            let a3026 = v1;
            let a3027 = v2;
            nat_eq(a3026, a3027)
        }
    } else {
        false
    } {
        f164(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f157(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3028 = v1;
    let a3029 = 0u64;
    if nat_eq(a3028, a3029) {
        Ok(Adt5::C4({
            let a3030 = v3;
            let a3031 = 0u64;
            if nat_eq(a3030, a3031) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f90(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3032 = v3;
            let a3033 = 0u64;
            if nat_eq(a3032, a3033) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let a3034 = v0;
    let a3035 = f60(v1)?;
    let a3036 = 0u64;
    Ok(nat_quot(a3034, a3035, a3036))
}

pub fn f159(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f158(v0, v1)?;
    let v3: u64 = {
        let a3039 = {
            let a3037 = f143(v0, v1)?;
            let a3038 = 2u64;
            nat_sub(a3037, a3038)
        };
        let a3040 = 2u64;
        let a3041 = 0u64;
        nat_quot(a3039, a3040, a3041)
    };
    let v4: u64 = {
        let a3042 = v2;
        let a3043 = v1;
        nat_mul(a3042, a3043)?
    };
    if f146(v0, v1)? {
        let a3044 = v4;
        let a3045 = v3;
        nat_add(a3044, a3045)
    } else {
        let a3057 = {
            let a3053 = v4;
            let a3054 = {
                let a3051 = {
                    let a3046 = v1;
                    let a3047 = 1u64;
                    nat_sub(a3046, a3047)
                };
                let a3052 = {
                    let a3048 = v2;
                    let a3049 = v1;
                    let a3050 = 0u64;
                    nat_rem(a3048, a3049, a3050)
                };
                nat_mul(a3051, a3052)?
            };
            nat_sub(a3053, a3054)
        };
        let a3058 = {
            let a3055 = v3;
            let a3056 = v1;
            nat_mul(a3055, a3056)?
        };
        nat_add(a3057, a3058)
    }
}

pub fn f160(v0: u64) -> bool {
    let a3059 = v0;
    let a3060 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3059, a3060) {
        true
    } else {
        let a3061 = v0;
        let a3062 = 1u64;
        nat_eq(a3061, a3062)
    } {
        true
    } else {
        let a3063 = v0;
        let a3064 = 2u64;
        nat_eq(a3063, a3064)
    } {
        true
    } else {
        let a3065 = v0;
        let a3066 = 3u64;
        nat_eq(a3065, a3066)
    } {
        true
    } else {
        let a3067 = v0;
        let a3068 = 6u64;
        nat_eq(a3067, a3068)
    } {
        true
    } else {
        let a3069 = v0;
        let a3070 = 7u64;
        nat_eq(a3069, a3070)
    } {
        true
    } else {
        let a3071 = v0;
        let a3072 = 8u64;
        nat_eq(a3071, a3072)
    } {
        true
    } else {
        let a3073 = v0;
        let a3074 = 10u64;
        nat_eq(a3073, a3074)
    } {
        true
    } else {
        let a3075 = v0;
        let a3076 = 11u64;
        nat_eq(a3075, a3076)
    } {
        true
    } else {
        let a3077 = v0;
        let a3078 = 12u64;
        nat_eq(a3077, a3078)
    } {
        true
    } else {
        let a3079 = v0;
        let a3080 = 13u64;
        nat_eq(a3079, a3080)
    } {
        true
    } else {
        let a3081 = v0;
        let a3082 = 14u64;
        nat_eq(a3081, a3082)
    } {
        true
    } else {
        let a3083 = v0;
        let a3084 = 16u64;
        nat_eq(a3083, a3084)
    } {
        true
    } else {
        let a3085 = v0;
        let a3086 = 17u64;
        nat_eq(a3085, a3086)
    } {
        true
    } else {
        let a3087 = v0;
        let a3088 = 18u64;
        nat_eq(a3087, a3088)
    } {
        true
    } else {
        let a3089 = v0;
        let a3090 = 19u64;
        nat_eq(a3089, a3090)
    } {
        true
    } else {
        let a3091 = v0;
        let a3092 = 20u64;
        nat_eq(a3091, a3092)
    } {
        true
    } else {
        let a3093 = v0;
        let a3094 = 21u64;
        nat_eq(a3093, a3094)
    } {
        true
    } else {
        let a3095 = v0;
        let a3096 = 22u64;
        nat_eq(a3095, a3096)
    } {
        true
    } else {
        let a3097 = v0;
        let a3098 = 23u64;
        nat_eq(a3097, a3098)
    } {
        true
    } else {
        let a3099 = v0;
        let a3100 = 26u64;
        nat_eq(a3099, a3100)
    } {
        true
    } else {
        let a3101 = v0;
        let a3102 = 29u64;
        nat_eq(a3101, a3102)
    } {
        true
    } else {
        let a3103 = v0;
        let a3104 = 30u64;
        nat_eq(a3103, a3104)
    } {
        true
    } else {
        let a3105 = v0;
        let a3106 = 34u64;
        nat_eq(a3105, a3106)
    } {
        true
    } else {
        let a3107 = v0;
        let a3108 = 35u64;
        nat_eq(a3107, a3108)
    } {
        true
    } else {
        let a3109 = v0;
        let a3110 = 39u64;
        nat_eq(a3109, a3110)
    } {
        true
    } else {
        let a3111 = v0;
        let a3112 = 40u64;
        nat_eq(a3111, a3112)
    } {
        true
    } else {
        let a3113 = v0;
        let a3114 = 41u64;
        nat_eq(a3113, a3114)
    } {
        true
    } else {
        let a3115 = v0;
        let a3116 = 42u64;
        nat_eq(a3115, a3116)
    } {
        true
    } else {
        let a3117 = v0;
        let a3118 = 43u64;
        nat_eq(a3117, a3118)
    }
}

pub fn f161(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3119 = v0;
    let a3120 = 0u64;
    if if if nat_eq(a3119, a3120) {
        true
    } else {
        let a3121 = v1;
        let a3122 = 0u64;
        nat_eq(a3121, a3122)
    } {
        true
    } else {
        let a3123 = v2;
        let a3124 = 0u64;
        nat_eq(a3123, a3124)
    } {
        Ok(0u64)
    } else {
        let a3127 = {
            let a3125 = v0;
            let a3126 = v1;
            nat_mul(a3125, a3126)?
        };
        let a3128 = v2;
        nat_mul(a3127, a3128)
    }
}

pub fn f162(v0: u64) -> bool {
    let m3129 = f71(v0);
    match m3129 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}

pub fn f163(v0: u64) -> bool {
    let a3130 = v0;
    let a3131 = 0u64;
    if if nat_eq(a3130, a3131) {
        true
    } else {
        let a3132 = v0;
        let a3133 = 1u64;
        nat_eq(a3132, a3133)
    } {
        true
    } else {
        let a3134 = v0;
        let a3135 = 30u64;
        nat_eq(a3134, a3135)
    }
}

pub fn f164(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a3136 = v0;
    let a3137 = 28u64;
    if if if if nat_eq(a3136, a3137) {
        let a3138 = v1;
        let a3139 = 0u64;
        nat_eq(a3138, a3139)
    } else {
        false
    } {
        let a3140 = v2;
        let a3141 = 0u64;
        nat_eq(a3140, a3141)
    } else {
        false
    } {
        let a3142 = v3;
        let a3143 = 0u64;
        nat_eq(a3142, a3143)
    } else {
        false
    } {
        let a3144 = v4;
        let a3145 = v8;
        if if if if if if if if nat_eq(a3144, a3145) {
            let a3146 = v5;
            let a3147 = v12;
            nat_eq(a3146, a3147)
        } else {
            false
        } {
            let a3148 = v9;
            let a3149 = v13;
            nat_eq(a3148, a3149)
        } else {
            false
        } {
            let a3150 = v10;
            let a3151 = v14;
            nat_eq(a3150, a3151)
        } else {
            false
        } {
            let a3152 = v11;
            let a3153 = v15;
            nat_eq(a3152, a3153)
        } else {
            false
        } {
            let a3154 = 0u64;
            let a3155 = v6;
            nat_lt(a3154, a3155)
        } else {
            false
        } {
            let a3156 = 0u64;
            let a3157 = v7;
            nat_lt(a3156, a3157)
        } else {
            false
        } {
            let a3161 = {
                let a3158 = v10;
                let a3159 = v6;
                let a3160 = 1u64;
                nat_rem(a3158, a3159, a3160)
            };
            let a3162 = 0u64;
            if nat_eq(a3161, a3162) {
                let a3166 = {
                    let a3163 = v11;
                    let a3164 = v7;
                    let a3165 = 1u64;
                    nat_rem(a3163, a3164, a3165)
                };
                let a3167 = 0u64;
                nat_eq(a3166, a3167)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3168 = v10;
                let a3169 = v6;
                let a3170 = 0u64;
                nat_quot(a3168, a3169, a3170)
            }, {
                let a3171 = v11;
                let a3172 = v7;
                let a3173 = 0u64;
                nat_quot(a3171, a3172, a3173)
            }, {
                let a3174 = v5;
                let a3175 = v4;
                nat_mul(a3174, a3175)?
            }, {
                let a3176 = v4;
                let a3177 = v9;
                nat_mul(a3176, a3177)?
            }, {
                let a3178 = v5;
                let a3179 = v9;
                nat_mul(a3178, a3179)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
