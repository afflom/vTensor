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
    C3(bool, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
        Adt0::C3(v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33) => {
            Ok(f9(v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, v32, v33))
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
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
            Adt1::C0(h25, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h25
            }
        })
    } else {
        false
    } {
        f11(match v0.clone() {
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
        Ok(Adt6::C4(f13(v2)?, 4u64, f14(v1.clone(), 1u64), f14(v1, 1u64), Adt4::C23(match v0.clone() {
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
                                Adt1::C0(h100, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                        Ok(Adt6::C4(f19(v9, v8, v10)?, 4u64, f14(v1, 1u64), f14(v4, 1u64), Adt4::C25(v9, v8, v10)))
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
            Adt1::C0(h117, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
        Ok(Adt6::C4(f26(v3, match v1.clone() {
            Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, v5, v6)?, v4, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h136, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, v5, f27(match v1 {
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
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        f28(match v0.clone() {
            Adt1::C0(h144, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
        Ok(Adt6::C4(f29(match v3.clone() {
            Adt3::C0(h152, _, _, _, _, _, _, _) => {
                h152
            }
        }, v4)?, 4u64, v3, f14(v2, 1u64), Adt4::C24(match v0 {
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
                        Ok(Adt6::C4(f30(match v0.clone() {
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
            f31(v2, v3, v4)?
        } else {
            let a205 = v0;
            let a206 = 77u64;
            if nat_eq(a205, a206) {
                f32(v2, v4)?
            } else {
                let a207 = v0;
                let a208 = 40u64;
                if nat_eq(a207, a208) {
                    f33(v2, v3, v4)?
                } else {
                    if f34(v0) {
                        f35(v0, v2, {
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
                            f36(v2, v4)?
                        } else {
                            let a212 = v0;
                            let a213 = 21u64;
                            if nat_eq(a212, a213) {
                                f37(v5, v2, v3, v4)?
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
                                    f38(v0, v1, v2, v4)?
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
                                        f39(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a224 = v0;
                                        let a225 = 20u64;
                                        if nat_eq(a224, a225) {
                                            f40(v2, v4)?
                                        } else {
                                            let a226 = v0;
                                            let a227 = 42u64;
                                            if nat_eq(a226, a227) {
                                                f41(4u64, v2, v4)?
                                            } else {
                                                let a228 = v0;
                                                let a229 = 69u64;
                                                if nat_eq(a228, a229) {
                                                    let a230 = v5;
                                                    let a231 = 4u64;
                                                    if nat_lt(a230, a231) {
                                                        f41(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a232 = v0;
                                                    let a233 = 70u64;
                                                    if nat_eq(a232, a233) {
                                                        f42(v5, v2, v4)?
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
                                                            f43({
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
                                                                f44(v0, v1, v2, v4)?
                                                            } else {
                                                                let a246 = v0;
                                                                let a247 = 15u64;
                                                                if nat_eq(a246, a247) {
                                                                    f45(v2, v4)?
                                                                } else {
                                                                    let a248 = v0;
                                                                    let a249 = 17u64;
                                                                    if nat_eq(a248, a249) {
                                                                        f46(v2, v4)?
                                                                    } else {
                                                                        let a250 = v0;
                                                                        let a251 = 18u64;
                                                                        if nat_eq(a250, a251) {
                                                                            f47(v2, v3, v4)?
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
                                                                                f48(v0, v2, v4)?
                                                                            } else {
                                                                                let m258 = f49(v0, v5)?;
                                                                                match m258 {
                                                                                    None => {
                                                                                        f50(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f51(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    Ok(f52(v3.clone(), {
        let m259 = v3.clone();
        match m259 {
            Adt4::C0 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f54(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f55(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f56(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f57(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v57, v58, v59, v60) => {
                f59(v0, v60, v57, v58, v59, v4)?
            }
            Adt4::C24(_) => {
                Adt5::C0(Adt6::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f53(v0, v1, v2, v3)?
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
                        f58(v0, v1, v2, v52, v53, v54, v55, v4)?
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

pub fn f9(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt5 {
    let a279 = v1;
    let a280 = 5u64;
    if if if if nat_lt(a279, a280) {
        let a281 = v5;
        let a282 = 5u64;
        nat_lt(a281, a282)
    } else {
        false
    } {
        let a283 = v7;
        let a284 = 5u64;
        nat_lt(a283, a284)
    } else {
        false
    } {
        let a285 = v9;
        let a286 = 2u64;
        if nat_lt(a285, a286) {
            true
        } else {
            let a287 = v9;
            let a288 = 3u64;
            nat_eq(a287, a288)
        }
    } else {
        false
    } {
        Adt5::C7(f60(v0, v1, v2, v3, v4), f61(v5, v6), f61(v7, v8), f62({
            let a289 = v9;
            let a290 = 0u64;
            if nat_eq(a289, a290) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt5::C0(Adt6::C1)
    }
}

pub fn f10(v0: Adt2, v1: u64) -> R<u64> {
    let a302 = {
        let a296 = {
            let a292 = match v0.clone() {
                Adt2::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a293 = v1;
            let a294 = 0u64;
            nat_quot(a292, a293, a294)
        };
        let a297 = match v0.clone() {
            Adt2::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                h295
            }
        };
        nat_mul(a296, a297)?
    };
    let a303 = {
        let a300 = match v0.clone() {
            Adt2::C0(_, _, _, h298, _, _, _, _, _, _, _) => {
                h298
            }
        };
        let a301 = match v0 {
            Adt2::C0(_, _, _, _, h299, _, _, _, _, _, _) => {
                h299
            }
        };
        nat_mul(a300, a301)?
    };
    nat_mul(a302, a303)
}

pub fn f11(v0: u64) -> bool {
    let a304 = v0;
    let a305 = 4294967296u64;
    if nat_lt(a304, a305) {
        let a309 = {
            let a306 = v0;
            let a307 = 2147483648u64;
            let a308 = 0u64;
            nat_rem(a306, a307, a308)
        };
        let a310 = 2139095040u64;
        nat_lt(a309, a310)
    } else {
        false
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a311 = 4u64;
    let a312 = v0;
    nat_mul(a311, a312)
}

pub fn f13(v0: u64) -> R<u64> {
    let a313 = v0;
    let a314 = 0u64;
    if nat_eq(a313, a314) {
        Ok(0u64)
    } else {
        let a317 = 4u64;
        let a318 = {
            let a315 = 3u64;
            let a316 = v0;
            nat_mul(a315, a316)?
        };
        nat_add(a317, a318)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a320 = match v0.clone() {
            Adt2::C0(_, h319, _, _, _, _, _, _, _, _, _) => {
                h319
            }
        };
        let a321 = v1;
        let a322 = 0u64;
        nat_quot(a320, a321, a322)
    }, match v0.clone() {
        Adt2::C0(_, _, h323, _, _, _, _, _, _, _, _) => {
            h323
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
            h324
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h325, _, _, _, _, _, _) => {
            h325
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h326, _, _, _, _, _) => {
            h326
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h327, _, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h328, _, _, _) => {
            h328
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h329, _, _) => {
            h329
        }
    })
}

pub fn f15(v0: Adt2, v1: Adt2) -> bool {
    let a332 = match v0.clone() {
        Adt2::C0(_, h330, _, _, _, _, _, _, _, _, _) => {
            h330
        }
    };
    let a333 = match v1.clone() {
        Adt2::C0(_, h331, _, _, _, _, _, _, _, _, _) => {
            h331
        }
    };
    if if if nat_eq(a332, a333) {
        let a336 = match v0.clone() {
            Adt2::C0(_, _, h334, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a337 = match v1.clone() {
            Adt2::C0(_, _, h335, _, _, _, _, _, _, _, _) => {
                h335
            }
        };
        nat_eq(a336, a337)
    } else {
        false
    } {
        let a340 = match v0.clone() {
            Adt2::C0(_, _, _, h338, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a341 = match v1.clone() {
            Adt2::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                h339
            }
        };
        nat_eq(a340, a341)
    } else {
        false
    } {
        let a344 = match v0 {
            Adt2::C0(_, _, _, _, h342, _, _, _, _, _, _) => {
                h342
            }
        };
        let a345 = match v1 {
            Adt2::C0(_, _, _, _, h343, _, _, _, _, _, _) => {
                h343
            }
        };
        nat_eq(a344, a345)
    } else {
        false
    }
}

pub fn f16(v0: Adt2) -> R<bool> {
    let a347 = match v0.clone() {
        Adt2::C0(_, h346, _, _, _, _, _, _, _, _, _) => {
            h346
        }
    };
    let a348 = 1u64;
    if if if if nat_eq(a347, a348) {
        true
    } else {
        let a350 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h349, _, _, _, _, _) => {
                h349
            }
        };
        let a351 = 4u64;
        nat_eq(a350, a351)
    } {
        let a353 = match v0.clone() {
            Adt2::C0(_, _, h352, _, _, _, _, _, _, _, _) => {
                h352
            }
        };
        let a354 = 1u64;
        if nat_eq(a353, a354) {
            true
        } else {
            let a359 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h355, _, _, _, _) => {
                    h355
                }
            };
            let a360 = {
                let a357 = match v0.clone() {
                    Adt2::C0(_, h356, _, _, _, _, _, _, _, _, _) => {
                        h356
                    }
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
            Adt2::C0(_, _, _, h361, _, _, _, _, _, _, _) => {
                h361
            }
        };
        let a363 = 1u64;
        if nat_eq(a362, a363) {
            true
        } else {
            let a371 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h364, _, _, _) => {
                    h364
                }
            };
            let a372 = {
                let a369 = {
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
                let a370 = 4u64;
                nat_mul(a369, a370)?
            };
            nat_eq(a371, a372)
        }
    } else {
        false
    } {
        let a374 = match v0.clone() {
            Adt2::C0(_, _, _, _, h373, _, _, _, _, _, _) => {
                h373
            }
        };
        let a375 = 1u64;
        if nat_eq(a374, a375) {
            Ok(true)
        } else {
            let a386 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h376, _, _) => {
                    h376
                }
            };
            let a387 = {
                let a384 = {
                    let a382 = {
                        let a379 = match v0.clone() {
                            Adt2::C0(_, h377, _, _, _, _, _, _, _, _, _) => {
                                h377
                            }
                        };
                        let a380 = match v0.clone() {
                            Adt2::C0(_, _, h378, _, _, _, _, _, _, _, _) => {
                                h378
                            }
                        };
                        nat_mul(a379, a380)?
                    };
                    let a383 = match v0 {
                        Adt2::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
                            h381
                        }
                    };
                    nat_mul(a382, a383)?
                };
                let a385 = 4u64;
                nat_mul(a384, a385)?
            };
            Ok(nat_eq(a386, a387))
        }
    } else {
        Ok(false)
    }
}

pub fn f17(v0: Adt2, v1: Adt2) -> R<bool> {
    let a389 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h388, _) => {
            h388
        }
    };
    let a390 = 0u64;
    if nat_eq(a389, a390) {
        Ok(true)
    } else {
        let a392 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h391, _) => {
                h391
            }
        };
        let a393 = 0u64;
        if nat_eq(a392, a393) {
            Ok(true)
        } else {
            let a395 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h394) => {
                    h394
                }
            };
            let a396 = 0u64;
            if nat_eq(a395, a396) {
                Ok(true)
            } else {
                let a398 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h397) => {
                        h397
                    }
                };
                let a399 = 0u64;
                if nat_eq(a398, a399) {
                    Ok(true)
                } else {
                    let a405 = {
                        let a402 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h400) => {
                                h400
                            }
                        };
                        let a403 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h401, _) => {
                                h401
                            }
                        };
                        nat_add(a402, a403)?
                    };
                    let a406 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h404) => {
                            h404
                        }
                    };
                    if nat_le(a405, a406) {
                        Ok(true)
                    } else {
                        let a412 = {
                            let a409 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h407) => {
                                    h407
                                }
                            };
                            let a410 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h408, _) => {
                                    h408
                                }
                            };
                            nat_add(a409, a410)?
                        };
                        let a413 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h411) => {
                                h411
                            }
                        };
                        Ok(nat_le(a412, a413))
                    }
                }
            }
        }
    }
}

pub fn f18(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a418 = {
        let a414 = v0;
        let a415 = v1;
        nat_mul(a414, a415)?
    };
    let a419 = {
        let a416 = v2;
        let a417 = f63(v3);
        nat_mul(a416, a417)?
    };
    nat_mul(a418, a419)
}

pub fn f19(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a422 = v0;
    let a423 = {
        let a420 = f64(v1)?;
        let a421 = v2;
        nat_mul(a420, a421)?
    };
    nat_add(a422, a423)
}

pub fn f20(v0: u64) -> u64 {
    let a424 = v0;
    let a425 = 30u64;
    if nat_eq(a424, a425) {
        2u64
    } else {
        4u64
    }
}

pub fn f21(v0: u64) -> bool {
    let a426 = v0;
    let a427 = 2139095040u64;
    if nat_le(a426, a427) {
        true
    } else {
        let a428 = v0;
        let a429 = 2147483648u64;
        nat_eq(a428, a429)
    }
}

pub fn f22(v0: Adt2, v1: u64) -> bool {
    let a431 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h430, _, _, _, _, _) => {
            h430
        }
    };
    let a432 = v1;
    if if nat_eq(a431, a432) {
        let a437 = {
            let a434 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h433) => {
                    h433
                }
            };
            let a435 = v1;
            let a436 = 0u64;
            nat_rem(a434, a435, a436)
        };
        let a438 = 0u64;
        nat_eq(a437, a438)
    } else {
        false
    } {
        f65(match v0.clone() {
            Adt2::C0(_, _, h439, _, _, _, _, _, _, _, _) => {
                h439
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h440, _, _, _, _, _, _, _) => {
                h440
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h441, _, _, _, _, _, _) => {
                h441
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h442, _, _, _, _) => {
                h442
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h443, _, _, _) => {
                h443
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h444, _, _) => {
                h444
            }
        }, v1)
    } else {
        false
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f15(v0.clone(), v1.clone()) {
        let a447 = match v0.clone() {
            Adt2::C0(h445, _, _, _, _, _, _, _, _, _, _) => {
                h445
            }
        };
        let a448 = match v1.clone() {
            Adt2::C0(h446, _, _, _, _, _, _, _, _, _, _) => {
                h446
            }
        };
        nat_eq(a447, a448)
    } else {
        false
    } {
        let a451 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h449) => {
                h449
            }
        };
        let a452 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h450) => {
                h450
            }
        };
        nat_eq(a451, a452)
    } else {
        false
    } {
        let a455 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h453, _, _, _, _, _) => {
                h453
            }
        };
        let a456 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h454, _, _, _, _, _) => {
                h454
            }
        };
        nat_eq(a455, a456)
    } else {
        false
    } {
        let a459 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h457, _, _, _, _) => {
                h457
            }
        };
        let a460 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h458, _, _, _, _) => {
                h458
            }
        };
        nat_eq(a459, a460)
    } else {
        false
    } {
        let a463 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h461, _, _, _) => {
                h461
            }
        };
        let a464 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h462, _, _, _) => {
                h462
            }
        };
        nat_eq(a463, a464)
    } else {
        false
    } {
        let a467 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h465, _, _) => {
                h465
            }
        };
        let a468 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h466, _, _) => {
                h466
            }
        };
        nat_eq(a467, a468)
    } else {
        false
    }
}

pub fn f24(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a469 = v0;
    let a470 = 0u64;
    if if if nat_eq(a469, a470) {
        true
    } else {
        let a471 = v1;
        let a472 = 0u64;
        nat_eq(a471, a472)
    } {
        true
    } else {
        let a473 = v2;
        let a474 = 0u64;
        nat_eq(a473, a474)
    } {
        true
    } else {
        let a475 = v3;
        let a476 = 0u64;
        nat_eq(a475, a476)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> R<bool> {
    f66({
        let a478 = match v0.clone() {
            Adt2::C0(_, h477, _, _, _, _, _, _, _, _, _) => {
                h477
            }
        };
        let a479 = v1;
        nat_mul(a478, a479)?
    }, match v0.clone() {
        Adt2::C0(_, _, h480, _, _, _, _, _, _, _, _) => {
            h480
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h481, _, _, _, _, _, _, _) => {
            h481
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h482, _, _, _, _, _, _) => {
            h482
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h483, _, _, _, _) => {
            h483
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h484, _, _, _) => {
            h484
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h485, _, _) => {
            h485
        }
    })
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a486 = v1;
    let a487 = 0u64;
    if if nat_eq(a486, a487) {
        true
    } else {
        let a488 = v2;
        let a489 = 0u64;
        nat_eq(a488, a489)
    } {
        Ok(0u64)
    } else {
        let a496 = {
            let a490 = f67(v1, v2, v3)?;
            let a491 = f68(v0, v1, v2)?;
            nat_add(a490, a491)?
        };
        let a497 = {
            let a494 = f69(v0);
            let a495 = {
                let a492 = v1;
                let a493 = v2;
                nat_mul(a492, a493)?
            };
            nat_mul(a494, a495)?
        };
        nat_add(a496, a497)
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a498 = v0;
    let a499 = 0u64;
    if if nat_eq(a498, a499) {
        true
    } else {
        let a500 = v1;
        let a501 = 0u64;
        nat_eq(a500, a501)
    } {
        Ok(3u64)
    } else {
        let a504 = {
            let a502 = 3u64;
            let a503 = v1;
            nat_add(a502, a503)?
        };
        let a505 = v2;
        nat_add(a504, a505)
    }
}

pub fn f28(v0: u64) -> bool {
    let a506 = v0;
    let a507 = 2139095040u64;
    if nat_le(a506, a507) {
        true
    } else {
        let a508 = v0;
        let a509 = 2147483648u64;
        nat_eq(a508, a509)
    }
}

pub fn f29(v0: u64, v1: u64) -> R<u64> {
    let a510 = v0;
    let a511 = 0u64;
    if nat_eq(a510, a511) {
        Ok(0u64)
    } else {
        let a512 = v1;
        let a513 = f70(v0)?;
        nat_mul(a512, a513)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a514 = v0;
    let a515 = 0u64;
    if if if if nat_eq(a514, a515) {
        true
    } else {
        let a516 = v1;
        let a517 = 0u64;
        nat_eq(a516, a517)
    } {
        true
    } else {
        let a518 = v2;
        let a519 = 0u64;
        nat_eq(a518, a519)
    } {
        true
    } else {
        let a520 = v3;
        let a521 = 0u64;
        nat_eq(a520, a521)
    } {
        Ok(0u64)
    } else {
        let a534 = {
            let a524 = 2u64;
            let a525 = {
                let a522 = v1;
                let a523 = v2;
                nat_mul(a522, a523)?
            };
            nat_mul(a524, a525)?
        };
        let a535 = {
            let a532 = {
                let a528 = {
                    let a526 = v1;
                    let a527 = v2;
                    nat_mul(a526, a527)?
                };
                let a529 = v3;
                nat_mul(a528, a529)?
            };
            let a533 = {
                let a530 = 1u64;
                let a531 = v0;
                nat_add(a530, a531)?
            };
            nat_mul(a532, a533)?
        };
        nat_add(a534, a535)
    }
}

pub fn f31(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m536 = v1;
    match m536 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f71(v0, v3, v2)
        }
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a538 = match v0.clone() {
        Adt2::C0(h537, _, _, _, _, _, _, _, _, _, _) => {
            h537
        }
    };
    let a539 = 1u64;
    if if nat_eq(a538, a539) {
        true
    } else {
        let a541 = match v0.clone() {
            Adt2::C0(h540, _, _, _, _, _, _, _, _, _, _) => {
                h540
            }
        };
        let a542 = 30u64;
        nat_eq(a541, a542)
    } {
        let a544 = match v1.clone() {
            Adt2::C0(h543, _, _, _, _, _, _, _, _, _, _) => {
                h543
            }
        };
        let a545 = 1u64;
        if if if if if if if if if if if nat_eq(a544, a545) {
            let a548 = match v0.clone() {
                Adt2::C0(_, h546, _, _, _, _, _, _, _, _, _) => {
                    h546
                }
            };
            let a549 = match v1.clone() {
                Adt2::C0(_, h547, _, _, _, _, _, _, _, _, _) => {
                    h547
                }
            };
            nat_eq(a548, a549)
        } else {
            false
        } {
            let a551 = 0u64;
            let a552 = match v1.clone() {
                Adt2::C0(_, _, h550, _, _, _, _, _, _, _, _) => {
                    h550
                }
            };
            nat_lt(a551, a552)
        } else {
            false
        } {
            let a555 = match v1.clone() {
                Adt2::C0(_, _, h553, _, _, _, _, _, _, _, _) => {
                    h553
                }
            };
            let a556 = match v1.clone() {
                Adt2::C0(_, _, _, h554, _, _, _, _, _, _, _) => {
                    h554
                }
            };
            nat_eq(a555, a556)
        } else {
            false
        } {
            let a558 = match v1.clone() {
                Adt2::C0(_, _, _, _, h557, _, _, _, _, _, _) => {
                    h557
                }
            };
            let a559 = 1u64;
            nat_eq(a558, a559)
        } else {
            false
        } {
            let a567 = match v0.clone() {
                Adt2::C0(_, _, h560, _, _, _, _, _, _, _, _) => {
                    h560
                }
            };
            let a568 = {
                let a565 = {
                    let a563 = match v1.clone() {
                        Adt2::C0(_, _, h561, _, _, _, _, _, _, _, _) => {
                            h561
                        }
                    };
                    let a564 = match v1.clone() {
                        Adt2::C0(_, _, h562, _, _, _, _, _, _, _, _) => {
                            h562
                        }
                    };
                    nat_add(a563, a564)?
                };
                let a566 = 1u64;
                nat_sub(a565, a566)
            };
            nat_eq(a567, a568)
        } else {
            false
        } {
            let a570 = 0u64;
            let a571 = match v0.clone() {
                Adt2::C0(_, _, _, h569, _, _, _, _, _, _, _) => {
                    h569
                }
            };
            nat_lt(a570, a571)
        } else {
            false
        } {
            let a573 = 0u64;
            let a574 = match v0.clone() {
                Adt2::C0(_, _, _, _, h572, _, _, _, _, _, _) => {
                    h572
                }
            };
            nat_lt(a573, a574)
        } else {
            false
        } {
            let a576 = 2u64;
            let a577 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h575, _, _, _, _, _) => {
                    h575
                }
            };
            nat_le(a576, a577)
        } else {
            false
        } {
            let a579 = 2u64;
            let a580 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h578, _, _, _, _, _) => {
                    h578
                }
            };
            nat_le(a579, a580)
        } else {
            false
        } {
            f17(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f10(v1.clone(), 1u64)?, 2u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f33(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m581 = v1;
    match m581 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f72(v0, v3, v2)
        }
    }
}

pub fn f34(v0: u64) -> bool {
    let a582 = v0;
    let a583 = 1u64;
    if if nat_eq(a582, a583) {
        true
    } else {
        let a584 = v0;
        let a585 = 33u64;
        nat_eq(a584, a585)
    } {
        true
    } else {
        let a586 = v0;
        let a587 = 34u64;
        nat_eq(a586, a587)
    }
}

pub fn f35(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a590 = match v1.clone() {
        Adt2::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
            h588
        }
    };
    let a591 = match v3.clone() {
        Adt2::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    };
    if nat_eq(a590, a591) {
        f73(v0, v1, v2, v3)
    } else {
        if if if if f74(match v1.clone() {
            Adt2::C0(h592, _, _, _, _, _, _, _, _, _, _) => {
                h592
            }
        }) {
            f74(match v3.clone() {
                Adt2::C0(h593, _, _, _, _, _, _, _, _, _, _) => {
                    h593
                }
            })
        } else {
            false
        } {
            let a594 = v0;
            let a595 = 33u64;
            if nat_eq(a594, a595) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a596 = f10(v1.clone(), 1u64)?;
            let a597 = f10(v3.clone(), 1u64)?;
            nat_eq(a596, a597)
        } else {
            false
        } {
            let m599 = f75(match v3.clone() {
                Adt2::C0(h598, _, _, _, _, _, _, _, _, _, _) => {
                    h598
                }
            });
            match m599 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f10(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h600) => {
                            h600
                        }
                    }, f14(v1.clone(), 1u64), f14(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h601, _, _, _, _, _, _, _, _, _, _) => {
                            h601
                        }
                    }, match v3 {
                        Adt2::C0(h602, _, _, _, _, _, _, _, _, _, _) => {
                            h602
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f36(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a604 = match v0.clone() {
        Adt2::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
            h603
        }
    };
    let a605 = 0u64;
    if if if if nat_eq(a604, a605) {
        true
    } else {
        let a607 = match v0.clone() {
            Adt2::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a608 = 1u64;
        nat_eq(a607, a608)
    } {
        true
    } else {
        let a610 = match v0.clone() {
            Adt2::C0(h609, _, _, _, _, _, _, _, _, _, _) => {
                h609
            }
        };
        let a611 = 25u64;
        if nat_eq(a610, a611) {
            true
        } else {
            let a613 = match v0.clone() {
                Adt2::C0(h612, _, _, _, _, _, _, _, _, _, _) => {
                    h612
                }
            };
            let a614 = 26u64;
            nat_eq(a613, a614)
        }
    } {
        true
    } else {
        let a616 = match v0.clone() {
            Adt2::C0(h615, _, _, _, _, _, _, _, _, _, _) => {
                h615
            }
        };
        let a617 = 30u64;
        nat_eq(a616, a617)
    } {
        let a620 = match v0.clone() {
            Adt2::C0(h618, _, _, _, _, _, _, _, _, _, _) => {
                h618
            }
        };
        let a621 = match v1.clone() {
            Adt2::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                h619
            }
        };
        if if nat_eq(a620, a621) {
            f76(match v1.clone() {
                Adt2::C0(_, h622, _, _, _, _, _, _, _, _, _) => {
                    h622
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h623, _, _, _, _, _, _, _, _) => {
                    h623
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h624, _, _, _, _, _, _, _) => {
                    h624
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h625, _, _, _, _, _, _) => {
                    h625
                }
            }, match v0.clone() {
                Adt2::C0(_, h626, _, _, _, _, _, _, _, _, _) => {
                    h626
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h627, _, _, _, _, _, _, _, _) => {
                    h627
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h628, _, _, _, _, _, _, _) => {
                    h628
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h629, _, _, _, _, _, _) => {
                    h629
                }
            })
        } else {
            false
        } {
            let m631 = f75(match v0.clone() {
                Adt2::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                    h630
                }
            });
            match m631 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f10(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h632) => {
                            h632
                        }
                    }, f14(v0, 1u64), f14(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f37(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m633 = v2;
    match m633 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a634 = v0;
            let a635 = 4u64;
            if if if nat_lt(a634, a635) {
                let a638 = match v1.clone() {
                    Adt2::C0(h636, _, _, _, _, _, _, _, _, _, _) => {
                        h636
                    }
                };
                let a639 = match v4.clone() {
                    Adt2::C0(h637, _, _, _, _, _, _, _, _, _, _) => {
                        h637
                    }
                };
                if nat_eq(a638, a639) {
                    let a642 = match v1.clone() {
                        Adt2::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                            h640
                        }
                    };
                    let a643 = match v3.clone() {
                        Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                            h641
                        }
                    };
                    nat_eq(a642, a643)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f77(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h645, _, _, _, _, _, _, _, _, _) => {
                        h645
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h646, _, _, _, _, _, _, _, _, _) => {
                        h646
                    }
                })? {
                    f77(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h647, _, _, _, _, _, _, _, _) => {
                            h647
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h648, _, _, _, _, _, _, _, _) => {
                            h648
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h649, _, _, _, _, _, _, _, _) => {
                            h649
                        }
                    })?
                } else {
                    false
                } {
                    f77(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h650, _, _, _, _, _, _, _) => {
                            h650
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                            h651
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h652, _, _, _, _, _, _, _) => {
                            h652
                        }
                    })?
                } else {
                    false
                } {
                    f77(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h653, _, _, _, _, _, _) => {
                            h653
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h654, _, _, _, _, _, _) => {
                            h654
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h655, _, _, _, _, _, _) => {
                            h655
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m657 = f75(match v1.clone() {
                    Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _) => {
                        h656
                    }
                });
                match m657 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h658, _) => {
                                h658
                            }
                        };
                        let a663 = {
                            let a660 = match v1.clone() {
                                Adt2::C0(_, h659, _, _, _, _, _, _, _, _, _) => {
                                    h659
                                }
                            };
                            let a661 = v6;
                            let a662 = 1u64;
                            nat_rem(a660, a661, a662)
                        };
                        let a664 = 0u64;
                        if if if nat_eq(a663, a664) {
                            let a669 = {
                                let a666 = match v4.clone() {
                                    Adt2::C0(_, h665, _, _, _, _, _, _, _, _, _) => {
                                        h665
                                    }
                                };
                                let a667 = v6;
                                let a668 = 1u64;
                                nat_rem(a666, a667, a668)
                            };
                            let a670 = 0u64;
                            nat_eq(a669, a670)
                        } else {
                            false
                        } {
                            let a675 = {
                                let a672 = match v3.clone() {
                                    Adt2::C0(_, h671, _, _, _, _, _, _, _, _, _) => {
                                        h671
                                    }
                                };
                                let a673 = v6;
                                let a674 = 1u64;
                                nat_rem(a672, a673, a674)
                            };
                            let a676 = 0u64;
                            nat_eq(a675, a676)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f10(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h677) => {
                                    h677
                                }
                            }, f14(v1, v6), f14(v3, v6), Adt4::C2(v0, f14(v4, v6))))
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

pub fn f38(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a679 = match v2.clone() {
        Adt2::C0(h678, _, _, _, _, _, _, _, _, _, _) => {
            h678
        }
    };
    let a680 = 0u64;
    if if nat_eq(a679, a680) {
        let a682 = match v3.clone() {
            Adt2::C0(h681, _, _, _, _, _, _, _, _, _, _) => {
                h681
            }
        };
        let a683 = 0u64;
        nat_eq(a682, a683)
    } else {
        false
    } {
        let a684 = v0;
        let a685 = 61u64;
        if nat_eq(a684, a685) {
            let v4: bool = {
                let a687 = 0u64;
                let a688 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h686, _, _, _, _, _, _, _) => {
                        h686
                    }
                };
                nat_lt(a687, a688)
            };
            if if if if if if f78(match v2.clone() {
                Adt2::C0(_, h689, _, _, _, _, _, _, _, _, _) => {
                    h689
                }
            }, match v1.clone() {
                Adt1::C0(h690, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h690
                }
            }, match v1.clone() {
                Adt1::C0(_, h691, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h691
                }
            }, match v3.clone() {
                Adt2::C0(_, h692, _, _, _, _, _, _, _, _, _) => {
                    h692
                }
            })? {
                f78(match v2.clone() {
                    Adt2::C0(_, _, h693, _, _, _, _, _, _, _, _) => {
                        h693
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h694, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h694
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h695, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h695
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h696, _, _, _, _, _, _, _, _) => {
                        h696
                    }
                })?
            } else {
                false
            } {
                f78(match v2.clone() {
                    Adt2::C0(_, _, _, h697, _, _, _, _, _, _, _) => {
                        h697
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h698, _, _, _, _, _, _, _, _, _, _, _) => {
                        h698
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h699, _, _, _, _, _, _, _, _, _, _) => {
                        h699
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h700, _, _, _, _, _, _, _) => {
                        h700
                    }
                })?
            } else {
                false
            } {
                f78(match v2.clone() {
                    Adt2::C0(_, _, _, _, h701, _, _, _, _, _, _) => {
                        h701
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h702, _, _, _, _, _, _, _, _, _) => {
                        h702
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h703, _, _, _, _, _, _, _, _) => {
                        h703
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h704, _, _, _, _, _, _) => {
                        h704
                    }
                })?
            } else {
                false
            } {
                let a706 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h705, _, _, _, _, _, _, _) => {
                        h705
                    }
                };
                let a707 = 4294967295u64;
                nat_le(a706, a707)
            } else {
                false
            } {
                if v4 {
                    !f24(match v2.clone() {
                        Adt2::C0(_, h708, _, _, _, _, _, _, _, _, _) => {
                            h708
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h709, _, _, _, _, _, _, _, _) => {
                            h709
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h710, _, _, _, _, _, _, _) => {
                            h710
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h711, _, _, _, _, _, _) => {
                            h711
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h712, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h712
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h713, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h713
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h714, _, _, _, _, _, _, _, _, _, _, _) => {
                        h714
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h715, _, _, _, _, _, _, _, _, _) => {
                        h715
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a716 = v0;
            let a717 = 62u64;
            if nat_eq(a716, a717) {
                let a719 = match v1.clone() {
                    Adt1::C0(h718, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h718
                    }
                };
                let a720 = 2147483648u64;
                if if if if if nat_lt(a719, a720) {
                    let a722 = match v1.clone() {
                        Adt1::C0(_, h721, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h721
                        }
                    };
                    let a723 = 2147483648u64;
                    nat_lt(a722, a723)
                } else {
                    false
                } {
                    let a726 = match v1.clone() {
                        Adt1::C0(h724, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h724
                        }
                    };
                    let a727 = match v2.clone() {
                        Adt2::C0(_, h725, _, _, _, _, _, _, _, _, _) => {
                            h725
                        }
                    };
                    nat_lt(a726, a727)
                } else {
                    false
                } {
                    let a730 = match v1.clone() {
                        Adt1::C0(_, h728, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h728
                        }
                    };
                    let a731 = match v2.clone() {
                        Adt2::C0(_, h729, _, _, _, _, _, _, _, _, _) => {
                            h729
                        }
                    };
                    nat_lt(a730, a731)
                } else {
                    false
                } {
                    let a734 = match v2.clone() {
                        Adt2::C0(_, _, h732, _, _, _, _, _, _, _, _) => {
                            h732
                        }
                    };
                    let a735 = match v3.clone() {
                        Adt2::C0(_, _, h733, _, _, _, _, _, _, _, _) => {
                            h733
                        }
                    };
                    if if if nat_eq(a734, a735) {
                        let a738 = match v2.clone() {
                            Adt2::C0(_, _, _, h736, _, _, _, _, _, _, _) => {
                                h736
                            }
                        };
                        let a739 = match v3.clone() {
                            Adt2::C0(_, _, _, h737, _, _, _, _, _, _, _) => {
                                h737
                            }
                        };
                        nat_eq(a738, a739)
                    } else {
                        false
                    } {
                        let a742 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h740, _, _, _, _, _, _) => {
                                h740
                            }
                        };
                        let a743 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h741, _, _, _, _, _, _) => {
                                h741
                            }
                        };
                        nat_eq(a742, a743)
                    } else {
                        false
                    } {
                        let a752 = match v3.clone() {
                            Adt2::C0(_, h744, _, _, _, _, _, _, _, _, _) => {
                                h744
                            }
                        };
                        let a753 = {
                            let a750 = {
                                let a747 = match v2.clone() {
                                    Adt2::C0(_, h745, _, _, _, _, _, _, _, _, _) => {
                                        h745
                                    }
                                };
                                let a748 = match v1.clone() {
                                    Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h746
                                    }
                                };
                                nat_add(a747, a748)?
                            };
                            let a751 = match v1.clone() {
                                Adt1::C0(_, h749, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h749
                                }
                            };
                            nat_add(a750, a751)?
                        };
                        nat_eq(a752, a753)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h754, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h754
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a755 = v0;
                let a756 = 63u64;
                if nat_eq(a755, a756) {
                    if if if if if f79(match v2.clone() {
                        Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                            h757
                        }
                    }, match v1.clone() {
                        Adt1::C0(h758, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h758
                        }
                    })? {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, h759, _, _, _, _, _, _, _, _) => {
                                h759
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h760, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h760
                            }
                        })?
                    } else {
                        false
                    } {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, _, h761, _, _, _, _, _, _, _) => {
                                h761
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h762, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h762
                            }
                        })?
                    } else {
                        false
                    } {
                        f79(match v2.clone() {
                            Adt2::C0(_, _, _, _, h763, _, _, _, _, _, _) => {
                                h763
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h764, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h764
                            }
                        })?
                    } else {
                        false
                    } {
                        let a767 = match v2.clone() {
                            Adt2::C0(_, h765, _, _, _, _, _, _, _, _, _) => {
                                h765
                            }
                        };
                        let a768 = match v3.clone() {
                            Adt2::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                                h766
                            }
                        };
                        if if if nat_eq(a767, a768) {
                            let a771 = match v2.clone() {
                                Adt2::C0(_, _, h769, _, _, _, _, _, _, _, _) => {
                                    h769
                                }
                            };
                            let a772 = match v3.clone() {
                                Adt2::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                                    h770
                                }
                            };
                            nat_eq(a771, a772)
                        } else {
                            false
                        } {
                            let a775 = match v2.clone() {
                                Adt2::C0(_, _, _, h773, _, _, _, _, _, _, _) => {
                                    h773
                                }
                            };
                            let a776 = match v3.clone() {
                                Adt2::C0(_, _, _, h774, _, _, _, _, _, _, _) => {
                                    h774
                                }
                            };
                            nat_eq(a775, a776)
                        } else {
                            false
                        } {
                            let a779 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h777, _, _, _, _, _, _) => {
                                    h777
                                }
                            };
                            let a780 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h778, _, _, _, _, _, _) => {
                                    h778
                                }
                            };
                            nat_eq(a779, a780)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h781, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h781
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h782, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h782
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h783, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h783
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h784, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h784
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

pub fn f39(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m785 = v3;
    match m785 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a786 = v0;
                let a787 = 4u64;
                nat_eq(a786, a787)
            };
            let v7: bool = {
                let a789 = 0u64;
                let a790 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h788, _, _, _, _, _, _, _, _, _, _, _) => {
                        h788
                    }
                };
                nat_lt(a789, a790)
            };
            let v8: u64 = f10(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h791, _, _, _, _, _, _, _, _, _) => {
                    h791
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                    h792
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h793, _, _, _, _, _, _, _) => {
                    h793
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h794, _, _, _, _, _, _) => {
                    h794
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h795, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h795
                }
            }, match v1.clone() {
                Adt1::C0(_, h796, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h796
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h797, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h797
                }
            });
            if if if if if if if if if if if if v6 {
                let a799 = match v2.clone() {
                    Adt2::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                        h798
                    }
                };
                let a800 = 0u64;
                if nat_eq(a799, a800) {
                    let a802 = match v5.clone() {
                        Adt2::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                            h801
                        }
                    };
                    let a803 = 0u64;
                    nat_eq(a802, a803)
                } else {
                    false
                }
            } else {
                let a805 = match v2.clone() {
                    Adt2::C0(h804, _, _, _, _, _, _, _, _, _, _) => {
                        h804
                    }
                };
                let a806 = 0u64;
                if if nat_eq(a805, a806) {
                    true
                } else {
                    let a808 = match v2.clone() {
                        Adt2::C0(h807, _, _, _, _, _, _, _, _, _, _) => {
                            h807
                        }
                    };
                    let a809 = 26u64;
                    nat_eq(a808, a809)
                } {
                    let a811 = match v5.clone() {
                        Adt2::C0(h810, _, _, _, _, _, _, _, _, _, _) => {
                            h810
                        }
                    };
                    let a812 = 0u64;
                    if nat_eq(a811, a812) {
                        true
                    } else {
                        let a814 = match v5.clone() {
                            Adt2::C0(h813, _, _, _, _, _, _, _, _, _, _) => {
                                h813
                            }
                        };
                        let a815 = 26u64;
                        nat_eq(a814, a815)
                    }
                } else {
                    false
                }
            } {
                let a818 = match v2.clone() {
                    Adt2::C0(h816, _, _, _, _, _, _, _, _, _, _) => {
                        h816
                    }
                };
                let a819 = match v4.clone() {
                    Adt2::C0(h817, _, _, _, _, _, _, _, _, _, _) => {
                        h817
                    }
                };
                nat_eq(a818, a819)
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
                let a821 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h820, _, _, _, _, _) => {
                        h820
                    }
                };
                let a822 = 4u64;
                nat_eq(a821, a822)
            } else {
                false
            } {
                let a823 = v8;
                let a824 = f10(v2.clone(), 1u64)?;
                nat_le(a823, a824)
            } else {
                false
            } {
                let a826 = match v1.clone() {
                    Adt1::C0(_, _, _, h825, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = 1073741824u64;
                nat_lt(a826, a827)
            } else {
                false
            } {
                let a829 = match v1.clone() {
                    Adt1::C0(h828, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h828
                    }
                };
                let a830 = 4294967295u64;
                if if if if nat_le(a829, a830) {
                    let a832 = match v1.clone() {
                        Adt1::C0(_, h831, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h831
                        }
                    };
                    let a833 = 4294967295u64;
                    nat_le(a832, a833)
                } else {
                    false
                } {
                    let a835 = match v1.clone() {
                        Adt1::C0(_, _, h834, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h834
                        }
                    };
                    let a836 = 4294967295u64;
                    nat_le(a835, a836)
                } else {
                    false
                } {
                    let a838 = match v1.clone() {
                        Adt1::C0(_, _, _, h837, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h837
                        }
                    };
                    let a839 = 4294967295u64;
                    nat_le(a838, a839)
                } else {
                    false
                } {
                    let a841 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h840, _, _, _, _, _, _, _, _, _, _, _) => {
                            h840
                        }
                    };
                    let a842 = 4294967295u64;
                    nat_le(a841, a842)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a845 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h843) => {
                            h843
                        }
                    };
                    let a846 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h844) => {
                            h844
                        }
                    };
                    nat_eq(a845, a846)
                } else {
                    true
                }
            } else {
                false
            } {
                f80(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h847, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h847
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h848, _) => {
                        h848
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
                Ok(Adt6::C4({
                    let a849 = v10;
                    let a850 = v8;
                    nat_add(a849, a850)?
                }, 4u64, f14(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h851, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h851
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f40(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a853 = match v0.clone() {
        Adt2::C0(h852, _, _, _, _, _, _, _, _, _, _) => {
            h852
        }
    };
    let a854 = 0u64;
    if if if nat_eq(a853, a854) {
        let a856 = match v1.clone() {
            Adt2::C0(h855, _, _, _, _, _, _, _, _, _, _) => {
                h855
            }
        };
        let a857 = 0u64;
        nat_eq(a856, a857)
    } else {
        false
    } {
        f76(match v0.clone() {
            Adt2::C0(_, h858, _, _, _, _, _, _, _, _, _) => {
                h858
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h859, _, _, _, _, _, _, _, _) => {
                h859
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h860, _, _, _, _, _, _, _) => {
                h860
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h861, _, _, _, _, _, _) => {
                h861
            }
        }, match v1.clone() {
            Adt2::C0(_, h862, _, _, _, _, _, _, _, _, _) => {
                h862
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h863, _, _, _, _, _, _, _, _) => {
                h863
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h864, _, _, _, _, _, _, _) => {
                h864
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h865, _, _, _, _, _, _) => {
                h865
            }
        })
    } else {
        false
    } {
        let v2: u64 = f10(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a866 = v2;
            let a867 = f10(v0.clone(), 1u64)?;
            nat_add(a866, a867)?
        }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a869 = match v1.clone() {
        Adt2::C0(h868, _, _, _, _, _, _, _, _, _, _) => {
            h868
        }
    };
    let a870 = 0u64;
    if if if nat_eq(a869, a870) {
        true
    } else {
        let a872 = match v1.clone() {
            Adt2::C0(h871, _, _, _, _, _, _, _, _, _, _) => {
                h871
            }
        };
        let a873 = 1u64;
        nat_eq(a872, a873)
    } {
        true
    } else {
        let a875 = match v1.clone() {
            Adt2::C0(h874, _, _, _, _, _, _, _, _, _, _) => {
                h874
            }
        };
        let a876 = 30u64;
        nat_eq(a875, a876)
    } {
        let a879 = match v1.clone() {
            Adt2::C0(h877, _, _, _, _, _, _, _, _, _, _) => {
                h877
            }
        };
        let a880 = match v2.clone() {
            Adt2::C0(h878, _, _, _, _, _, _, _, _, _, _) => {
                h878
            }
        };
        if if if if if nat_eq(a879, a880) {
            let a883 = match v1.clone() {
                Adt2::C0(_, h881, _, _, _, _, _, _, _, _, _) => {
                    h881
                }
            };
            let a884 = match v2.clone() {
                Adt2::C0(_, h882, _, _, _, _, _, _, _, _, _) => {
                    h882
                }
            };
            nat_eq(a883, a884)
        } else {
            false
        } {
            let a887 = match v1.clone() {
                Adt2::C0(_, _, _, h885, _, _, _, _, _, _, _) => {
                    h885
                }
            };
            let a888 = match v2.clone() {
                Adt2::C0(_, _, _, h886, _, _, _, _, _, _, _) => {
                    h886
                }
            };
            nat_eq(a887, a888)
        } else {
            false
        } {
            let a891 = match v1.clone() {
                Adt2::C0(_, _, _, _, h889, _, _, _, _, _, _) => {
                    h889
                }
            };
            let a892 = match v2.clone() {
                Adt2::C0(_, _, _, _, h890, _, _, _, _, _, _) => {
                    h890
                }
            };
            nat_eq(a891, a892)
        } else {
            false
        } {
            let a893 = v0;
            let a894 = 4u64;
            if nat_eq(a893, a894) {
                let a896 = match v1.clone() {
                    Adt2::C0(_, _, h895, _, _, _, _, _, _, _, _) => {
                        h895
                    }
                };
                let a897 = 1u64;
                if nat_eq(a896, a897) {
                    let a900 = match v1.clone() {
                        Adt2::C0(_, h898, _, _, _, _, _, _, _, _, _) => {
                            h898
                        }
                    };
                    let a901 = match v2.clone() {
                        Adt2::C0(_, _, h899, _, _, _, _, _, _, _, _) => {
                            h899
                        }
                    };
                    nat_eq(a900, a901)
                } else {
                    false
                }
            } else {
                let a902 = v0;
                let a903 = 4u64;
                if if nat_lt(a902, a903) {
                    let a906 = match v1.clone() {
                        Adt2::C0(_, h904, _, _, _, _, _, _, _, _, _) => {
                            h904
                        }
                    };
                    let a907 = match v1.clone() {
                        Adt2::C0(_, _, h905, _, _, _, _, _, _, _, _) => {
                            h905
                        }
                    };
                    nat_eq(a906, a907)
                } else {
                    false
                } {
                    let a910 = match v1.clone() {
                        Adt2::C0(_, _, h908, _, _, _, _, _, _, _, _) => {
                            h908
                        }
                    };
                    let a911 = match v2.clone() {
                        Adt2::C0(_, _, h909, _, _, _, _, _, _, _, _) => {
                            h909
                        }
                    };
                    nat_eq(a910, a911)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m913 = f75(match v1.clone() {
                Adt2::C0(h912, _, _, _, _, _, _, _, _, _, _) => {
                    h912
                }
            });
            match m913 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f10(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h914) => {
                            h914
                        }
                    }, f14(v1, 1u64), f14(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f42(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f81(match v1.clone() {
        Adt2::C0(h915, _, _, _, _, _, _, _, _, _, _) => {
            h915
        }
    }, v0) {
        let a918 = match v1.clone() {
            Adt2::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a919 = match v2.clone() {
            Adt2::C0(h917, _, _, _, _, _, _, _, _, _, _) => {
                h917
            }
        };
        nat_eq(a918, a919)
    } else {
        false
    } {
        let a922 = match v1.clone() {
            Adt2::C0(_, h920, _, _, _, _, _, _, _, _, _) => {
                h920
            }
        };
        let a923 = match v2.clone() {
            Adt2::C0(_, h921, _, _, _, _, _, _, _, _, _) => {
                h921
            }
        };
        if if if nat_eq(a922, a923) {
            let a926 = match v1.clone() {
                Adt2::C0(_, _, h924, _, _, _, _, _, _, _, _) => {
                    h924
                }
            };
            let a927 = match v2.clone() {
                Adt2::C0(_, _, h925, _, _, _, _, _, _, _, _) => {
                    h925
                }
            };
            nat_eq(a926, a927)
        } else {
            false
        } {
            let a930 = match v1.clone() {
                Adt2::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                    h928
                }
            };
            let a931 = match v2.clone() {
                Adt2::C0(_, _, _, h929, _, _, _, _, _, _, _) => {
                    h929
                }
            };
            nat_eq(a930, a931)
        } else {
            false
        } {
            let a934 = match v1.clone() {
                Adt2::C0(_, _, _, _, h932, _, _, _, _, _, _) => {
                    h932
                }
            };
            let a935 = match v2.clone() {
                Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
                    h933
                }
            };
            nat_eq(a934, a935)
        } else {
            false
        }
    } else {
        false
    } {
        let m937 = f75(match v1.clone() {
            Adt2::C0(h936, _, _, _, _, _, _, _, _, _, _) => {
                h936
            }
        });
        match m937 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f10(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h938) => {
                        h938
                    }
                }, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
                        h939
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f43(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h940, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h940
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h941, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h941
            }
        }
    };
    let a942 = 0u64;
    let a943 = v4;
    if if nat_lt(a942, a943) {
        let a944 = v4;
        let a945 = 2147483648u64;
        nat_lt(a944, a945)
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
        let v7: u64 = f82(match v5.clone() {
            Adt2::C0(_, _, h946, _, _, _, _, _, _, _, _) => {
                h946
            }
        }, v4)?;
        let v8: u64 = f82(match v5.clone() {
            Adt2::C0(_, _, _, h947, _, _, _, _, _, _, _) => {
                h947
            }
        }, v4)?;
        let a949 = match v5.clone() {
            Adt2::C0(h948, _, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        let a950 = 0u64;
        if if if if if nat_eq(a949, a950) {
            let a952 = match v6.clone() {
                Adt2::C0(h951, _, _, _, _, _, _, _, _, _, _) => {
                    h951
                }
            };
            let a953 = 0u64;
            nat_eq(a952, a953)
        } else {
            false
        } {
            let a955 = match v5.clone() {
                Adt2::C0(_, _, _, _, h954, _, _, _, _, _, _) => {
                    h954
                }
            };
            let a956 = 1u64;
            nat_eq(a955, a956)
        } else {
            false
        } {
            let a959 = match v5.clone() {
                Adt2::C0(_, h957, _, _, _, _, _, _, _, _, _) => {
                    h957
                }
            };
            let a960 = match v6.clone() {
                Adt2::C0(_, h958, _, _, _, _, _, _, _, _, _) => {
                    h958
                }
            };
            nat_eq(a959, a960)
        } else {
            false
        } {
            if v0 {
                let a962 = match v5.clone() {
                    Adt2::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                        h961
                    }
                };
                let a963 = 2147483648u64;
                if if if nat_lt(a962, a963) {
                    let a965 = match v5.clone() {
                        Adt2::C0(_, _, _, h964, _, _, _, _, _, _, _) => {
                            h964
                        }
                    };
                    let a966 = 2147483648u64;
                    nat_lt(a965, a966)
                } else {
                    false
                } {
                    let a969 = {
                        let a967 = v7;
                        let a968 = v8;
                        nat_mul(a967, a968)?
                    };
                    let a970 = 2147483648u64;
                    nat_le(a969, a970)
                } else {
                    false
                } {
                    f83(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a972 = match v1.clone() {
                    Adt1::C0(h971, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h971
                    }
                };
                let a973 = v7;
                if if if if nat_eq(a972, a973) {
                    let a975 = match v1 {
                        Adt1::C0(_, h974, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h974
                        }
                    };
                    let a976 = v8;
                    nat_eq(a975, a976)
                } else {
                    false
                } {
                    f84(v7, v8, v4)?
                } else {
                    false
                } {
                    let a978 = match v6.clone() {
                        Adt2::C0(_, _, h977, _, _, _, _, _, _, _, _) => {
                            h977
                        }
                    };
                    let a979 = v4;
                    if nat_eq(a978, a979) {
                        let a981 = match v6.clone() {
                            Adt2::C0(_, _, _, h980, _, _, _, _, _, _, _) => {
                                h980
                            }
                        };
                        let a982 = v4;
                        nat_eq(a981, a982)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a986 = match v6 {
                        Adt2::C0(_, _, _, _, h983, _, _, _, _, _, _) => {
                            h983
                        }
                    };
                    let a987 = {
                        let a984 = v7;
                        let a985 = v8;
                        nat_mul(a984, a985)?
                    };
                    nat_eq(a986, a987)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f44(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a989 = match v2.clone() {
        Adt2::C0(h988, _, _, _, _, _, _, _, _, _, _) => {
            h988
        }
    };
    let a990 = 0u64;
    if if if if if nat_eq(a989, a990) {
        true
    } else {
        let a992 = match v2.clone() {
            Adt2::C0(h991, _, _, _, _, _, _, _, _, _, _) => {
                h991
            }
        };
        let a993 = 1u64;
        nat_eq(a992, a993)
    } {
        true
    } else {
        let a995 = match v2.clone() {
            Adt2::C0(h994, _, _, _, _, _, _, _, _, _, _) => {
                h994
            }
        };
        let a996 = 30u64;
        nat_eq(a995, a996)
    } {
        let a999 = match v2.clone() {
            Adt2::C0(h997, _, _, _, _, _, _, _, _, _, _) => {
                h997
            }
        };
        let a1000 = match v3.clone() {
            Adt2::C0(h998, _, _, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        nat_eq(a999, a1000)
    } else {
        false
    } {
        if if f15(v2.clone(), v3.clone()) {
            let a1002 = match v1.clone() {
                Adt1::C0(h1001, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1001
                }
            };
            let a1003 = 4294967296u64;
            nat_lt(a1002, a1003)
        } else {
            false
        } {
            let a1005 = match v1.clone() {
                Adt1::C0(_, h1004, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1004
                }
            };
            let a1006 = 4294967296u64;
            nat_lt(a1005, a1006)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v3.clone(), 1u64)?, {
            let a1008 = match v2.clone() {
                Adt2::C0(h1007, _, _, _, _, _, _, _, _, _, _) => {
                    h1007
                }
            };
            let a1009 = 0u64;
            if nat_eq(a1008, a1009) {
                4u64
            } else {
                2u64
            }
        }, f14(v2.clone(), 1u64), f14(v3, 1u64), Adt4::C13({
            let a1010 = v0;
            let a1011 = 31u64;
            if nat_eq(a1010, a1011) {
                0u64
            } else {
                let a1012 = v0;
                let a1013 = 47u64;
                if nat_eq(a1012, a1013) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1014, _, _, _, _, _, _, _, _, _, _) => {
                h1014
            }
        }, match v1.clone() {
            Adt1::C0(h1015, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1015
            }
        }, match v1 {
            Adt1::C0(_, h1016, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1016
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1018 = match v0.clone() {
        Adt2::C0(h1017, _, _, _, _, _, _, _, _, _, _) => {
            h1017
        }
    };
    let a1019 = 0u64;
    if if if nat_eq(a1018, a1019) {
        let a1021 = match v1.clone() {
            Adt2::C0(h1020, _, _, _, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1022 = 0u64;
        nat_eq(a1021, a1022)
    } else {
        false
    } {
        f15(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v1.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1024 = match v0.clone() {
        Adt2::C0(h1023, _, _, _, _, _, _, _, _, _, _) => {
            h1023
        }
    };
    let a1025 = 0u64;
    if if nat_eq(a1024, a1025) {
        let a1027 = match v1.clone() {
            Adt2::C0(h1026, _, _, _, _, _, _, _, _, _, _) => {
                h1026
            }
        };
        let a1028 = 26u64;
        nat_eq(a1027, a1028)
    } else {
        false
    } {
        let a1030 = match v0.clone() {
            Adt2::C0(_, _, _, h1029, _, _, _, _, _, _, _) => {
                h1029
            }
        };
        let a1031 = 1u64;
        if if if if if if if nat_eq(a1030, a1031) {
            let a1033 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1032, _, _, _, _, _, _) => {
                    h1032
                }
            };
            let a1034 = 1u64;
            nat_eq(a1033, a1034)
        } else {
            false
        } {
            let a1036 = match v0.clone() {
                Adt2::C0(_, h1035, _, _, _, _, _, _, _, _, _) => {
                    h1035
                }
            };
            let a1037 = 2147483647u64;
            nat_le(a1036, a1037)
        } else {
            false
        } {
            let a1040 = match v1.clone() {
                Adt2::C0(_, h1038, _, _, _, _, _, _, _, _, _) => {
                    h1038
                }
            };
            let a1041 = match v0.clone() {
                Adt2::C0(_, _, h1039, _, _, _, _, _, _, _, _) => {
                    h1039
                }
            };
            nat_eq(a1040, a1041)
        } else {
            false
        } {
            let a1043 = match v1.clone() {
                Adt2::C0(_, _, h1042, _, _, _, _, _, _, _, _) => {
                    h1042
                }
            };
            let a1044 = 1u64;
            nat_eq(a1043, a1044)
        } else {
            false
        } {
            let a1046 = match v1.clone() {
                Adt2::C0(_, _, _, h1045, _, _, _, _, _, _, _) => {
                    h1045
                }
            };
            let a1047 = 1u64;
            nat_eq(a1046, a1047)
        } else {
            false
        } {
            let a1049 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1048, _, _, _, _, _, _) => {
                    h1048
                }
            };
            let a1050 = 1u64;
            nat_eq(a1049, a1050)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1053 = match v0.clone() {
                    Adt2::C0(_, _, h1051, _, _, _, _, _, _, _, _) => {
                        h1051
                    }
                };
                let a1054 = f85(match v0.clone() {
                    Adt2::C0(_, h1052, _, _, _, _, _, _, _, _, _) => {
                        h1052
                    }
                })?;
                nat_mul(a1053, a1054)?
            }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f47(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1055 = v1;
    match m1055 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f86(f14(v0.clone(), 1u64))?;
            let a1057 = match v0.clone() {
                Adt2::C0(h1056, _, _, _, _, _, _, _, _, _, _) => {
                    h1056
                }
            };
            let a1058 = 26u64;
            if if if nat_eq(a1057, a1058) {
                let a1060 = match v3.clone() {
                    Adt2::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
                        h1059
                    }
                };
                let a1061 = 26u64;
                nat_eq(a1060, a1061)
            } else {
                false
            } {
                let a1063 = match v2.clone() {
                    Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
                        h1062
                    }
                };
                let a1064 = 27u64;
                nat_eq(a1063, a1064)
            } else {
                false
            } {
                if if if if if if f15(v0.clone(), v3.clone()) {
                    let a1066 = match v2.clone() {
                        Adt2::C0(_, h1065, _, _, _, _, _, _, _, _, _) => {
                            h1065
                        }
                    };
                    let a1067 = 1u64;
                    nat_eq(a1066, a1067)
                } else {
                    false
                } {
                    let a1069 = match v2.clone() {
                        Adt2::C0(_, _, h1068, _, _, _, _, _, _, _, _) => {
                            h1068
                        }
                    };
                    let a1070 = 1u64;
                    nat_eq(a1069, a1070)
                } else {
                    false
                } {
                    let a1072 = match v2.clone() {
                        Adt2::C0(_, _, _, h1071, _, _, _, _, _, _, _) => {
                            h1071
                        }
                    };
                    let a1073 = 1u64;
                    nat_eq(a1072, a1073)
                } else {
                    false
                } {
                    let a1075 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1074, _, _, _, _, _, _) => {
                            h1074
                        }
                    };
                    let a1076 = 1u64;
                    nat_eq(a1075, a1076)
                } else {
                    false
                } {
                    let a1077 = v4;
                    let a1078 = 9223372036854775808u64;
                    nat_lt(a1077, a1078)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1081 = 2u64;
                        let a1082 = {
                            let a1079 = v4;
                            let a1080 = 2u64;
                            nat_mul(a1079, a1080)?
                        };
                        nat_add(a1081, a1082)?
                    }, 8u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C18(v4, f14(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f48(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1083, _, _, _, _, _, _, _, _, _, _) => {
            h1083
        }
    };
    let a1084 = v3;
    let a1085 = 0u64;
    if if nat_eq(a1084, a1085) {
        true
    } else {
        let a1086 = v0;
        let a1087 = 13u64;
        if nat_eq(a1086, a1087) {
            let a1088 = v3;
            let a1089 = 1u64;
            if nat_eq(a1088, a1089) {
                true
            } else {
                let a1090 = v3;
                let a1091 = 30u64;
                nat_eq(a1090, a1091)
            }
        } else {
            false
        }
    } {
        let a1093 = v3;
        let a1094 = match v2.clone() {
            Adt2::C0(h1092, _, _, _, _, _, _, _, _, _, _) => {
                h1092
            }
        };
        if if nat_eq(a1093, a1094) {
            let a1095 = v0;
            let a1096 = 13u64;
            if nat_eq(a1095, a1096) {
                let a1098 = match v2.clone() {
                    Adt2::C0(_, h1097, _, _, _, _, _, _, _, _, _) => {
                        h1097
                    }
                };
                let a1099 = 1u64;
                if if if nat_eq(a1098, a1099) {
                    let a1101 = match v2.clone() {
                        Adt2::C0(_, _, h1100, _, _, _, _, _, _, _, _) => {
                            h1100
                        }
                    };
                    let a1102 = 1u64;
                    nat_eq(a1101, a1102)
                } else {
                    false
                } {
                    let a1104 = match v2.clone() {
                        Adt2::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                            h1103
                        }
                    };
                    let a1105 = 1u64;
                    nat_eq(a1104, a1105)
                } else {
                    false
                } {
                    let a1107 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1106, _, _, _, _, _, _) => {
                            h1106
                        }
                    };
                    let a1108 = 1u64;
                    nat_eq(a1107, a1108)
                } else {
                    false
                }
            } else {
                let a1110 = match v2.clone() {
                    Adt2::C0(_, h1109, _, _, _, _, _, _, _, _, _) => {
                        h1109
                    }
                };
                let a1111 = 1u64;
                if if if nat_eq(a1110, a1111) {
                    let a1114 = match v1.clone() {
                        Adt2::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
                            h1112
                        }
                    };
                    let a1115 = match v2.clone() {
                        Adt2::C0(_, _, h1113, _, _, _, _, _, _, _, _) => {
                            h1113
                        }
                    };
                    nat_eq(a1114, a1115)
                } else {
                    false
                } {
                    let a1118 = match v1.clone() {
                        Adt2::C0(_, _, _, h1116, _, _, _, _, _, _, _) => {
                            h1116
                        }
                    };
                    let a1119 = match v2.clone() {
                        Adt2::C0(_, _, _, h1117, _, _, _, _, _, _, _) => {
                            h1117
                        }
                    };
                    nat_eq(a1118, a1119)
                } else {
                    false
                } {
                    let a1122 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1120, _, _, _, _, _, _) => {
                            h1120
                        }
                    };
                    let a1123 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1121, _, _, _, _, _, _) => {
                            h1121
                        }
                    };
                    nat_eq(a1122, a1123)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f14(v1, 1u64);
            Ok(Adt6::C4(f87(v4.clone(), v0, v3)?, {
                let a1124 = v3;
                let a1125 = 0u64;
                if nat_eq(a1124, a1125) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f14(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f49(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1126 = 10u64;
    let a1127 = v0;
    if if nat_le(a1126, a1127) {
        let a1128 = v0;
        let a1129 = 12u64;
        nat_le(a1128, a1129)
    } else {
        false
    } {
        Ok(Some({
            let a1130 = v0;
            let a1131 = 3u64;
            nat_add(a1130, a1131)?
        }))
    } else {
        let a1132 = v0;
        let a1133 = 89u64;
        if if nat_eq(a1132, a1133) {
            let a1134 = v1;
            let a1135 = 7u64;
            nat_eq(a1134, a1135)
        } else {
            false
        } {
            Ok(Some(17u64))
        } else {
            let a1136 = v0;
            let a1137 = 89u64;
            if if nat_eq(a1136, a1137) {
                let a1138 = v1;
                let a1139 = 10u64;
                nat_eq(a1138, a1139)
            } else {
                false
            } {
                Ok(Some(18u64))
            } else {
                let a1140 = v0;
                let a1141 = 89u64;
                if if nat_eq(a1140, a1141) {
                    let a1142 = v1;
                    let a1143 = 13u64;
                    nat_eq(a1142, a1143)
                } else {
                    false
                } {
                    Ok(Some(16u64))
                } else {
                    let a1144 = v0;
                    let a1145 = 8u64;
                    if nat_eq(a1144, a1145) {
                        Ok(Some(5u64))
                    } else {
                        let a1146 = v0;
                        let a1147 = 9u64;
                        if nat_eq(a1146, a1147) {
                            Ok(Some(6u64))
                        } else {
                            let a1148 = v0;
                            let a1149 = 89u64;
                            if nat_eq(a1148, a1149) {
                                let a1150 = v1;
                                let a1151 = 0u64;
                                if nat_eq(a1150, a1151) {
                                    Ok(Some(0u64))
                                } else {
                                    let a1152 = v1;
                                    let a1153 = 1u64;
                                    if nat_eq(a1152, a1153) {
                                        Ok(Some(1u64))
                                    } else {
                                        let a1154 = v1;
                                        let a1155 = 2u64;
                                        if nat_eq(a1154, a1155) {
                                            Ok(Some(2u64))
                                        } else {
                                            let a1156 = v1;
                                            let a1157 = 3u64;
                                            if nat_eq(a1156, a1157) {
                                                Ok(Some(3u64))
                                            } else {
                                                let a1158 = v1;
                                                let a1159 = 6u64;
                                                if nat_eq(a1158, a1159) {
                                                    Ok(Some(4u64))
                                                } else {
                                                    let a1160 = v1;
                                                    let a1161 = 12u64;
                                                    if nat_eq(a1160, a1161) {
                                                        Ok(Some(7u64))
                                                    } else {
                                                        let a1162 = v1;
                                                        let a1163 = 11u64;
                                                        if nat_eq(a1162, a1163) {
                                                            Ok(Some(8u64))
                                                        } else {
                                                            let a1164 = v1;
                                                            let a1165 = 18u64;
                                                            if nat_eq(a1164, a1165) {
                                                                Ok(Some(9u64))
                                                            } else {
                                                                let a1166 = v1;
                                                                let a1167 = 19u64;
                                                                if nat_eq(a1166, a1167) {
                                                                    Ok(Some(10u64))
                                                                } else {
                                                                    let a1168 = v1;
                                                                    let a1169 = 20u64;
                                                                    if nat_eq(a1168, a1169) {
                                                                        Ok(Some(11u64))
                                                                    } else {
                                                                        let a1170 = v1;
                                                                        let a1171 = 21u64;
                                                                        if nat_eq(a1170, a1171) {
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

pub fn f50(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1172 = f88(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1172 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            let a1173 = v0;
            let a1174 = 2u64;
            if if if if nat_eq(a1173, a1174) {
                true
            } else {
                let a1175 = v0;
                let a1176 = 5u64;
                nat_eq(a1175, a1176)
            } {
                true
            } else {
                let a1177 = v0;
                let a1178 = 6u64;
                nat_eq(a1177, a1178)
            } {
                true
            } else {
                let a1179 = v0;
                let a1180 = 7u64;
                nat_eq(a1179, a1180)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1181 = v3;
                    match m1181 {
                        None => {
                            f14(v2, 1u64)
                        }
                        Some(v13) => {
                            f14(v13, 1u64)
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

pub fn f51(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1183 = match v1.clone() {
        Adt2::C0(h1182, _, _, _, _, _, _, _, _, _, _) => {
            h1182
        }
    };
    let a1184 = 0u64;
    if if if if if nat_eq(a1183, a1184) {
        true
    } else {
        let a1186 = match v1.clone() {
            Adt2::C0(h1185, _, _, _, _, _, _, _, _, _, _) => {
                h1185
            }
        };
        let a1187 = 1u64;
        nat_eq(a1186, a1187)
    } {
        true
    } else {
        let a1189 = match v1.clone() {
            Adt2::C0(h1188, _, _, _, _, _, _, _, _, _, _) => {
                h1188
            }
        };
        let a1190 = 30u64;
        nat_eq(a1189, a1190)
    } {
        let a1193 = match v1.clone() {
            Adt2::C0(h1191, _, _, _, _, _, _, _, _, _, _) => {
                h1191
            }
        };
        let a1194 = match v2.clone() {
            Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        if nat_eq(a1193, a1194) {
            true
        } else {
            let a1196 = match v2.clone() {
                Adt2::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                    h1195
                }
            };
            let a1197 = 0u64;
            nat_eq(a1196, a1197)
        }
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v2.clone(), 1u64)?, {
            let a1199 = match v2.clone() {
                Adt2::C0(h1198, _, _, _, _, _, _, _, _, _, _) => {
                    h1198
                }
            };
            let a1200 = 0u64;
            if nat_eq(a1199, a1200) {
                4u64
            } else {
                2u64
            }
        }, f14(v1.clone(), 1u64), f14(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        }, match v2 {
            Adt2::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f52(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1203 = v1;
    match m1203 {
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
            f89(f90(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f91(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f92(v0.clone(), v16);
            f89(f93(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m1204 = v3;
    match m1204 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f96(v0, v1, v2.clone())?, f94(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f97(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f98(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f99(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f98(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f100(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f101(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f102(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f94(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f103(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f104(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C15 => {
            f105(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f106(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f107(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f94(f95(v0, match v2.clone() {
                Adt3::C0(h1205, _, _, _, _, _, _, _) => {
                    h1205
                }
            }, match v2.clone() {
                Adt3::C0(_, h1206, _, _, _, _, _, _) => {
                    h1206
                }
            })?, v1)?, f94(v0, v2)?))
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
            f108(v0, v48, v49, v50)
        }
        Adt4::C26(v51, _, _, _) => {
            let a1207 = v51;
            let a1208 = 64u64;
            if if if nat_eq(a1207, a1208) {
                true
            } else {
                let a1209 = v51;
                let a1210 = 65u64;
                nat_eq(a1209, a1210)
            } {
                true
            } else {
                let a1211 = v51;
                let a1212 = 66u64;
                nat_eq(a1211, a1212)
            } {
                Ok(Adt5::C0(Adt6::C5(64u64)))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f85(match v1.clone() {
        Adt3::C0(h1213, _, _, _, _, _, _, _) => {
            h1213
        }
    })?;
    let v5: u64 = {
        let a1214 = v0;
        let a1215 = v4;
        let a1216 = 0u64;
        nat_quot(a1214, a1215, a1216)
    };
    let v6: u64 = {
        let a1217 = v0;
        let a1218 = v4;
        let a1219 = 0u64;
        nat_rem(a1217, a1218, a1219)
    };
    let a1220 = v6;
    let a1221 = 0u64;
    if nat_eq(a1220, a1221) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1222 = v6;
        let a1223 = 1u64;
        if nat_eq(a1222, a1223) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1226 = {
                let a1224 = v6;
                let a1225 = 1u64;
                nat_add(a1224, a1225)?
            };
            let a1227 = v4;
            if nat_eq(a1226, a1227) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f94(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1228 = v6;
                    let a1229 = 2u64;
                    nat_sub(a1228, a1229)
                };
                let v8: u64 = {
                    let a1230 = v7;
                    let a1231 = 3u64;
                    let a1232 = 0u64;
                    nat_quot(a1230, a1231, a1232)
                };
                let v9: u64 = {
                    let a1233 = v7;
                    let a1234 = 3u64;
                    let a1235 = 0u64;
                    nat_rem(a1233, a1234, a1235)
                };
                let v10: u64 = f94({
                    let a1239 = {
                        let a1237 = v5;
                        let a1238 = match v1.clone() {
                            Adt3::C0(h1236, _, _, _, _, _, _, _) => {
                                h1236
                            }
                        };
                        nat_mul(a1237, a1238)?
                    };
                    let a1240 = v8;
                    nat_add(a1239, a1240)?
                }, v1)?;
                let a1241 = v9;
                let a1242 = 0u64;
                if nat_eq(a1241, a1242) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1243 = v9;
                    let a1244 = 1u64;
                    if nat_eq(a1243, a1244) {
                        let a1245 = v3;
                        let a1246 = 2u64;
                        if nat_eq(a1245, a1246) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1247 = v3;
                        let a1248 = 2u64;
                        if if nat_eq(a1247, a1248) {
                            true
                        } else {
                            let a1249 = v3;
                            let a1250 = 3u64;
                            nat_eq(a1249, a1250)
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

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1252 = 2u64;
        let a1253 = match v1.clone() {
            Adt3::C0(_, h1251, _, _, _, _, _, _) => {
                h1251
            }
        };
        nat_mul(a1252, a1253)?
    };
    let a1254 = v0;
    let a1255 = v5;
    if nat_lt(a1254, a1255) {
        let a1259 = {
            let a1256 = v0;
            let a1257 = 2u64;
            let a1258 = 0u64;
            nat_rem(a1256, a1257, a1258)
        };
        let a1260 = 0u64;
        if nat_eq(a1259, a1260) {
            Ok(Adt5::C4(19u64, 4u64, f94({
                let a1261 = v0;
                let a1262 = 2u64;
                let a1263 = 0u64;
                nat_quot(a1261, a1262, a1263)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1264 = v4;
            let a1265 = 2147483648u64;
            if if nat_lt(a1264, a1265) {
                let a1267 = v4;
                let a1268 = match v2 {
                    Adt3::C0(_, h1266, _, _, _, _, _, _) => {
                        h1266
                    }
                };
                nat_lt(a1267, a1268)
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
            let a1271 = match v2.clone() {
                Adt3::C0(h1269, _, _, _, _, _, _, _) => {
                    h1269
                }
            };
            let a1272 = match v2.clone() {
                Adt3::C0(_, h1270, _, _, _, _, _, _) => {
                    h1270
                }
            };
            nat_mul(a1271, a1272)?
        };
        let a1275 = {
            let a1273 = v0;
            let a1274 = v5;
            nat_sub(a1273, a1274)
        };
        let a1276 = v6;
        if nat_lt(a1275, a1276) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f94({
                let a1277 = v0;
                let a1278 = v5;
                nat_sub(a1277, a1278)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1281 = {
                    let a1279 = v0;
                    let a1280 = v5;
                    nat_sub(a1279, a1280)
                };
                let a1282 = v6;
                nat_sub(a1281, a1282)
            };
            let v8: u64 = {
                let a1286 = 1u64;
                let a1287 = {
                    let a1284 = 2u64;
                    let a1285 = match v1.clone() {
                        Adt3::C0(h1283, _, _, _, _, _, _, _) => {
                            h1283
                        }
                    };
                    nat_mul(a1284, a1285)?
                };
                nat_add(a1286, a1287)?
            };
            let v9: u64 = {
                let a1288 = v7;
                let a1289 = v8;
                let a1290 = 0u64;
                nat_quot(a1288, a1289, a1290)
            };
            let v10: u64 = {
                let a1291 = v7;
                let a1292 = v8;
                let a1293 = 0u64;
                nat_rem(a1291, a1292, a1293)
            };
            let a1294 = v10;
            let a1295 = 0u64;
            if nat_eq(a1294, a1295) {
                Ok(Adt5::C4(19u64, 4u64, f94(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1299 = {
                    let a1296 = v10;
                    let a1297 = 2u64;
                    let a1298 = 0u64;
                    nat_rem(a1296, a1297, a1298)
                };
                let a1300 = 1u64;
                if nat_eq(a1299, a1300) {
                    Ok(Adt5::C4(6u64, 0u64, f94({
                        let a1309 = {
                            let a1302 = v9;
                            let a1303 = match v1.clone() {
                                Adt3::C0(h1301, _, _, _, _, _, _, _) => {
                                    h1301
                                }
                            };
                            nat_mul(a1302, a1303)?
                        };
                        let a1310 = {
                            let a1306 = {
                                let a1304 = v10;
                                let a1305 = 1u64;
                                nat_sub(a1304, a1305)
                            };
                            let a1307 = 2u64;
                            let a1308 = 0u64;
                            nat_quot(a1306, a1307, a1308)
                        };
                        nat_add(a1309, a1310)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f94({
                        let a1319 = {
                            let a1312 = v4;
                            let a1313 = match v1.clone() {
                                Adt3::C0(h1311, _, _, _, _, _, _, _) => {
                                    h1311
                                }
                            };
                            nat_mul(a1312, a1313)?
                        };
                        let a1320 = {
                            let a1316 = {
                                let a1314 = v10;
                                let a1315 = 1u64;
                                nat_sub(a1314, a1315)
                            };
                            let a1317 = 2u64;
                            let a1318 = 0u64;
                            nat_quot(a1316, a1317, a1318)
                        };
                        nat_add(a1319, a1320)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f94({
                        let a1329 = {
                            let a1322 = v4;
                            let a1323 = match v1 {
                                Adt3::C0(h1321, _, _, _, _, _, _, _) => {
                                    h1321
                                }
                            };
                            nat_mul(a1322, a1323)?
                        };
                        let a1330 = {
                            let a1326 = {
                                let a1324 = v10;
                                let a1325 = 1u64;
                                nat_sub(a1324, a1325)
                            };
                            let a1327 = 2u64;
                            let a1328 = 0u64;
                            nat_quot(a1326, a1327, a1328)
                        };
                        nat_add(a1329, a1330)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f56(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1331 = v1;
    let a1332 = f109(v2.clone(), v4.clone())?;
    if nat_lt(a1331, a1332) {
        let a1335 = v1;
        let a1336 = {
            let a1333 = 2u64;
            let a1334 = f86(v4.clone())?;
            nat_mul(a1333, a1334)?
        };
        if nat_lt(a1335, a1336) {
            let a1340 = {
                let a1337 = v1;
                let a1338 = 2u64;
                let a1339 = 0u64;
                nat_rem(a1337, a1338, a1339)
            };
            let a1341 = 0u64;
            if nat_eq(a1340, a1341) {
                f110({
                    let a1342 = v1;
                    let a1343 = 2u64;
                    let a1344 = 0u64;
                    nat_quot(a1342, a1343, a1344)
                }, v4)
            } else {
                if f111(match v2 {
                    Adt3::C0(_, h1345, _, _, _, _, _, _) => {
                        h1345
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1348 = v1;
                let a1349 = {
                    let a1346 = 2u64;
                    let a1347 = f86(v4.clone())?;
                    nat_mul(a1346, a1347)?
                };
                nat_sub(a1348, a1349)
            };
            let v7: u64 = {
                let a1353 = v6;
                let a1354 = {
                    let a1351 = 1u64;
                    let a1352 = match v2.clone() {
                        Adt3::C0(h1350, _, _, _, _, _, _, _) => {
                            h1350
                        }
                    };
                    nat_add(a1351, a1352)?
                };
                let a1355 = 0u64;
                nat_quot(a1353, a1354, a1355)
            };
            let v8: u64 = {
                let a1359 = v6;
                let a1360 = {
                    let a1357 = 1u64;
                    let a1358 = match v2.clone() {
                        Adt3::C0(h1356, _, _, _, _, _, _, _) => {
                            h1356
                        }
                    };
                    nat_add(a1357, a1358)?
                };
                let a1361 = 0u64;
                nat_rem(a1359, a1360, a1361)
            };
            let a1362 = v8;
            let a1363 = 0u64;
            if nat_eq(a1362, a1363) {
                f110(v7, v4)
            } else {
                f112(v0, v7, {
                    let a1364 = v8;
                    let a1365 = 1u64;
                    nat_sub(a1364, a1365)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1368 = match v4.clone() {
            Adt3::C0(h1366, _, _, _, _, _, _, _) => {
                h1366
            }
        };
        let a1369 = match v4.clone() {
            Adt3::C0(_, h1367, _, _, _, _, _, _) => {
                h1367
            }
        };
        nat_mul(a1368, a1369)?
    };
    let v7: u64 = {
        let a1370 = 2u64;
        let a1371 = v6;
        nat_mul(a1370, a1371)?
    };
    let v8: u64 = f113(match v3.clone() {
        Adt3::C0(_, h1372, _, _, _, _, _, _) => {
            h1372
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1373, _, _, _, _, _) => {
            h1373
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1374, _, _, _, _) => {
            h1374
        }
    });
    let a1375 = v0;
    let a1376 = v7;
    if nat_lt(a1375, a1376) {
        let a1380 = {
            let a1377 = v0;
            let a1378 = 2u64;
            let a1379 = 0u64;
            nat_rem(a1377, a1378, a1379)
        };
        let a1381 = 0u64;
        if nat_eq(a1380, a1381) {
            Ok(Adt5::C4(19u64, 5u64, f94({
                let a1382 = v0;
                let a1383 = 2u64;
                let a1384 = 0u64;
                nat_quot(a1382, a1383, a1384)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f114(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1385 = v0;
            let a1386 = v7;
            nat_sub(a1385, a1386)
        };
        let v10: u64 = {
            let a1390 = v9;
            let a1391 = {
                let a1388 = 1u64;
                let a1389 = match v1.clone() {
                    Adt3::C0(h1387, _, _, _, _, _, _, _) => {
                        h1387
                    }
                };
                nat_add(a1388, a1389)?
            };
            let a1392 = 0u64;
            nat_quot(a1390, a1391, a1392)
        };
        let v11: u64 = {
            let a1396 = v9;
            let a1397 = {
                let a1394 = 1u64;
                let a1395 = match v1.clone() {
                    Adt3::C0(h1393, _, _, _, _, _, _, _) => {
                        h1393
                    }
                };
                nat_add(a1394, a1395)?
            };
            let a1398 = 0u64;
            nat_rem(a1396, a1397, a1398)
        };
        let a1399 = v11;
        let a1400 = 0u64;
        if nat_eq(a1399, a1400) {
            Ok(Adt5::C4(19u64, 5u64, f94({
                let a1401 = v10;
                let a1402 = v6;
                let a1403 = 0u64;
                nat_rem(a1401, a1402, a1403)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f114(v5, v8) {
                let v12: u64 = {
                    let a1409 = {
                        let a1405 = v10;
                        let a1406 = match v1.clone() {
                            Adt3::C0(h1404, _, _, _, _, _, _, _) => {
                                h1404
                            }
                        };
                        nat_mul(a1405, a1406)?
                    };
                    let a1410 = {
                        let a1407 = v11;
                        let a1408 = 1u64;
                        nat_sub(a1407, a1408)
                    };
                    nat_add(a1409, a1410)?
                };
                let a1411 = v5;
                let a1412 = 4294967295u64;
                if nat_eq(a1411, a1412) {
                    Ok(Adt5::C1(false, f94(v12, v1)?, f94(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f94(v12, v1)?, 4u64, f94({
                        let a1418 = {
                            let a1414 = v5;
                            let a1415 = match v3.clone() {
                                Adt3::C0(h1413, _, _, _, _, _, _, _) => {
                                    h1413
                                }
                            };
                            nat_mul(a1414, a1415)?
                        };
                        let a1419 = {
                            let a1416 = v11;
                            let a1417 = 1u64;
                            nat_sub(a1416, a1417)
                        };
                        nat_add(a1418, a1419)?
                    }, v3)?, 3u64, f94(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt5> {
    let v8: u64 = {
        let a1422 = v6;
        let a1423 = {
            let a1420 = 3u64;
            let a1421 = v5;
            nat_add(a1420, a1421)?
        };
        nat_sub(a1422, a1423)
    };
    let a1425 = v0;
    let a1426 = f26(v3, match v1.clone() {
        Adt3::C0(h1424, _, _, _, _, _, _, _) => {
            h1424
        }
    }, v5, v8)?;
    if nat_lt(a1425, a1426) {
        let v9: u64 = f67(match v1.clone() {
            Adt3::C0(h1427, _, _, _, _, _, _, _) => {
                h1427
            }
        }, v5, v8)?;
        let v10: u64 = f68(v3, match v1.clone() {
            Adt3::C0(h1428, _, _, _, _, _, _, _) => {
                h1428
            }
        }, v5)?;
        let a1429 = v0;
        let a1430 = v9;
        if nat_lt(a1429, a1430) {
            f115(v0, v2, f20(v3), {
                let a1431 = 3u64;
                let a1432 = v5;
                nat_add(a1431, a1432)?
            }, v7)
        } else {
            let a1435 = v0;
            let a1436 = {
                let a1433 = v9;
                let a1434 = v10;
                nat_add(a1433, a1434)?
            };
            if nat_lt(a1435, a1436) {
                let v11: u64 = {
                    let a1437 = v0;
                    let a1438 = v9;
                    nat_sub(a1437, a1438)
                };
                let v12: u64 = f116(v3, match v1.clone() {
                    Adt3::C0(h1439, _, _, _, _, _, _, _) => {
                        h1439
                    }
                })?;
                f117({
                    let a1440 = v11;
                    let a1441 = v12;
                    let a1442 = 0u64;
                    nat_rem(a1440, a1441, a1442)
                }, {
                    let a1443 = v11;
                    let a1444 = v12;
                    let a1445 = 0u64;
                    nat_quot(a1443, a1444, a1445)
                }, v1, v3, v4, v7)
            } else {
                f118({
                    let a1448 = v0;
                    let a1449 = {
                        let a1446 = v9;
                        let a1447 = v10;
                        nat_add(a1446, a1447)?
                    };
                    nat_sub(a1448, a1449)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1450 = v0;
    let a1451 = f13(v1)?;
    if nat_lt(a1450, a1451) {
        let a1452 = v0;
        let a1453 = 4u64;
        if nat_lt(a1452, a1453) {
            let a1454 = v0;
            let a1455 = 0u64;
            if nat_eq(a1454, a1455) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1456 = v0;
                let a1457 = 1u64;
                if nat_eq(a1456, a1457) {
                    let a1458 = v5;
                    let a1459 = 0u64;
                    if nat_eq(a1458, a1459) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1460 = v0;
                    let a1461 = 2u64;
                    if nat_eq(a1460, a1461) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1462 = v0;
            let a1463 = 4u64;
            if if nat_eq(a1462, a1463) {
                let a1464 = f119(v5)?;
                let a1465 = v1;
                !nat_eq(a1464, a1465)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1468 = {
                        let a1466 = v0;
                        let a1467 = 4u64;
                        nat_sub(a1466, a1467)
                    };
                    let a1469 = 3u64;
                    let a1470 = 0u64;
                    nat_quot(a1468, a1469, a1470)
                };
                let v7: u64 = {
                    let a1473 = {
                        let a1471 = v0;
                        let a1472 = 4u64;
                        nat_sub(a1471, a1472)
                    };
                    let a1474 = 3u64;
                    let a1475 = 0u64;
                    nat_rem(a1473, a1474, a1475)
                };
                let a1476 = v7;
                let a1477 = 0u64;
                if nat_eq(a1476, a1477) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1478 = v7;
                    let a1479 = 1u64;
                    if nat_eq(a1478, a1479) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f12(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f60(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt7 {
    let a1480 = v1;
    let a1481 = 0u64;
    if nat_eq(a1480, a1481) {
        if v0 {
            Adt7::C2(v3, v4, false)
        } else {
            Adt7::C0(v2)
        }
    } else {
        let a1482 = v1;
        let a1483 = 1u64;
        if nat_eq(a1482, a1483) {
            Adt7::C1(v2)
        } else {
            let a1484 = v1;
            let a1485 = 2u64;
            if nat_eq(a1484, a1485) {
                Adt7::C5(v2)
            } else {
                let a1486 = v1;
                let a1487 = 3u64;
                if nat_eq(a1486, a1487) {
                    Adt7::C3(v2, v2, false)
                } else {
                    let a1488 = v1;
                    let a1489 = 4u64;
                    if nat_eq(a1488, a1489) {
                        Adt7::C2(v3, v4, false)
                    } else {
                        Adt7::C6
                    }
                }
            }
        }
    }
}

pub fn f61(v0: u64, v1: u64) -> Adt8 {
    let a1490 = v0;
    let a1491 = 0u64;
    if nat_eq(a1490, a1491) {
        Adt8::C0(0u64, v1)
    } else {
        let a1492 = v0;
        let a1493 = 1u64;
        if nat_eq(a1492, a1493) {
            Adt8::C1(v1)
        } else {
            let a1494 = v0;
            let a1495 = 2u64;
            if nat_eq(a1494, a1495) {
                Adt8::C2(v1)
            } else {
                let a1496 = v0;
                let a1497 = 3u64;
                if nat_eq(a1496, a1497) {
                    Adt8::C3(v1)
                } else {
                    let a1498 = v0;
                    let a1499 = 13u64;
                    if nat_lt(a1498, a1499) {
                        Adt8::C0({
                            let a1500 = v0;
                            let a1501 = 3u64;
                            nat_sub(a1500, a1501)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f62(v0: u64, v1: u64) -> Adt9 {
    let a1502 = v0;
    let a1503 = 1u64;
    if nat_eq(a1502, a1503) {
        Adt9::C0(v1)
    } else {
        let a1504 = v0;
        let a1505 = 3u64;
        if if nat_eq(a1504, a1505) {
            true
        } else {
            let a1506 = v0;
            let a1507 = 6u64;
            nat_eq(a1506, a1507)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f63(v0: u64) -> u64 {
    let v1: u64 = {
        let a1508 = v0;
        let a1509 = 4294967296u64;
        let a1510 = 0u64;
        nat_rem(a1508, a1509, a1510)
    };
    let a1511 = v1;
    let a1512 = 2147483648u64;
    if nat_lt(a1511, a1512) {
        v1
    } else {
        0u64
    }
}

pub fn f64(v0: u64) -> R<u64> {
    let a1515 = 2u64;
    let a1516 = {
        let a1513 = 2u64;
        let a1514 = v0;
        nat_mul(a1513, a1514)?
    };
    nat_add(a1515, a1516)
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f120(v0, v3, v6) {
        f120(v1, v4, v6)
    } else {
        false
    } {
        f120(v2, v5, v6)
    } else {
        false
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f121(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f121(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f121(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f121(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f121(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f121(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f67(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1517 = v2;
    let a1518 = 0u64;
    if nat_eq(a1517, a1518) {
        Ok(0u64)
    } else {
        let a1521 = 2u64;
        let a1522 = {
            let a1519 = v0;
            let a1520 = v1;
            nat_mul(a1519, a1520)?
        };
        nat_mul(a1521, a1522)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1523 = v2;
    let a1524 = f116(v0, v1)?;
    nat_mul(a1523, a1524)
}

pub fn f69(v0: u64) -> u64 {
    let a1525 = v0;
    let a1526 = 30u64;
    if nat_eq(a1525, a1526) {
        3u64
    } else {
        1u64
    }
}

pub fn f70(v0: u64) -> R<u64> {
    let a1529 = 5u64;
    let a1530 = {
        let a1527 = 4u64;
        let a1528 = v0;
        nat_mul(a1527, a1528)?
    };
    nat_add(a1529, a1530)
}

pub fn f71(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f122(match v0.clone() {
        Adt2::C0(h1531, _, _, _, _, _, _, _, _, _, _) => {
            h1531
        }
    }, match v0.clone() {
        Adt2::C0(_, h1532, _, _, _, _, _, _, _, _, _) => {
            h1532
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1533, _, _, _, _, _, _, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1534, _, _, _, _, _, _, _) => {
            h1534
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1535, _, _, _, _, _, _) => {
            h1535
        }
    }, match v1.clone() {
        Adt2::C0(h1536, _, _, _, _, _, _, _, _, _, _) => {
            h1536
        }
    }, match v1.clone() {
        Adt2::C0(_, h1537, _, _, _, _, _, _, _, _, _) => {
            h1537
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1538, _, _, _, _, _, _, _, _) => {
            h1538
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1539, _, _, _, _, _, _, _) => {
            h1539
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1540, _, _, _, _, _, _) => {
            h1540
        }
    }, match v2.clone() {
        Adt2::C0(h1541, _, _, _, _, _, _, _, _, _, _) => {
            h1541
        }
    }, match v2.clone() {
        Adt2::C0(_, h1542, _, _, _, _, _, _, _, _, _) => {
            h1542
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1543, _, _, _, _, _, _, _, _) => {
            h1543
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1544, _, _, _, _, _, _, _) => {
            h1544
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1545, _, _, _, _, _, _) => {
            h1545
        }
    }) {
        let m1547 = f75(match v0.clone() {
            Adt2::C0(h1546, _, _, _, _, _, _, _, _, _, _) => {
                h1546
            }
        });
        match m1547 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1553 = {
                    let a1550 = match v0.clone() {
                        Adt2::C0(_, h1548, _, _, _, _, _, _, _, _, _) => {
                            h1548
                        }
                    };
                    let a1551 = match v3.clone() {
                        Adt11::C0(_, h1549, _) => {
                            h1549
                        }
                    };
                    let a1552 = 0u64;
                    nat_rem(a1550, a1551, a1552)
                };
                let a1554 = 0u64;
                if if nat_eq(a1553, a1554) {
                    if f123(match v0.clone() {
                        Adt2::C0(_, h1555, _, _, _, _, _, _, _, _, _) => {
                            h1555
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1556, _, _, _, _, _, _, _, _, _) => {
                            h1556
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1557, _, _, _, _, _, _, _, _) => {
                            h1557
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1558, _, _, _, _, _, _, _) => {
                            h1558
                        }
                    }) {
                        true
                    } else {
                        let a1560 = 4u64;
                        let a1561 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1559, _, _, _, _, _) => {
                                h1559
                            }
                        };
                        if if nat_le(a1560, a1561) {
                            let a1563 = match v3.clone() {
                                Adt11::C0(_, h1562, _) => {
                                    h1562
                                }
                            };
                            let a1564 = 1u64;
                            if nat_eq(a1563, a1564) {
                                true
                            } else {
                                let a1567 = match v3.clone() {
                                    Adt11::C0(_, _, h1565) => {
                                        h1565
                                    }
                                };
                                let a1568 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1566, _, _, _, _, _) => {
                                        h1566
                                    }
                                };
                                nat_le(a1567, a1568)
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
                        Adt11::C0(_, h1569, _) => {
                            h1569
                        }
                    });
                    Ok(Adt6::C4(if f123(match v0.clone() {
                        Adt2::C0(_, h1570, _, _, _, _, _, _, _, _, _) => {
                            h1570
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1571, _, _, _, _, _, _, _, _, _) => {
                            h1571
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1572, _, _, _, _, _, _, _, _) => {
                            h1572
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1573, _, _, _, _, _, _, _) => {
                            h1573
                        }
                    }) {
                        0u64
                    } else {
                        f109(v4.clone(), f14(v1.clone(), 1u64))?
                    }, 4u64, v4, f14(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
                            h1574
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f72(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if if if if if if if if if if f74(match v0.clone() {
        Adt2::C0(h1575, _, _, _, _, _, _, _, _, _, _) => {
            h1575
        }
    }) {
        let a1577 = match v1.clone() {
            Adt2::C0(h1576, _, _, _, _, _, _, _, _, _, _) => {
                h1576
            }
        };
        let a1578 = 26u64;
        nat_eq(a1577, a1578)
    } else {
        false
    } {
        let a1580 = match v2.clone() {
            Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
                h1579
            }
        };
        let a1581 = 0u64;
        nat_eq(a1580, a1581)
    } else {
        false
    } {
        let a1584 = match v0.clone() {
            Adt2::C0(_, h1582, _, _, _, _, _, _, _, _, _) => {
                h1582
            }
        };
        let a1585 = match v2.clone() {
            Adt2::C0(_, h1583, _, _, _, _, _, _, _, _, _) => {
                h1583
            }
        };
        nat_eq(a1584, a1585)
    } else {
        false
    } {
        let a1588 = match v1.clone() {
            Adt2::C0(_, h1586, _, _, _, _, _, _, _, _, _) => {
                h1586
            }
        };
        let a1589 = match v0.clone() {
            Adt2::C0(_, _, h1587, _, _, _, _, _, _, _, _) => {
                h1587
            }
        };
        nat_le(a1588, a1589)
    } else {
        false
    } {
        let a1591 = match v0.clone() {
            Adt2::C0(_, _, _, h1590, _, _, _, _, _, _, _) => {
                h1590
            }
        };
        let a1592 = 1u64;
        nat_eq(a1591, a1592)
    } else {
        false
    } {
        let a1594 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1593, _, _, _, _, _, _) => {
                h1593
            }
        };
        let a1595 = 1u64;
        nat_eq(a1594, a1595)
    } else {
        false
    } {
        let a1597 = match v2.clone() {
            Adt2::C0(_, _, _, h1596, _, _, _, _, _, _, _) => {
                h1596
            }
        };
        let a1598 = 1u64;
        nat_eq(a1597, a1598)
    } else {
        false
    } {
        let a1600 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1599, _, _, _, _, _, _) => {
                h1599
            }
        };
        let a1601 = 1u64;
        nat_eq(a1600, a1601)
    } else {
        false
    } {
        let a1603 = match v1.clone() {
            Adt2::C0(_, _, h1602, _, _, _, _, _, _, _, _) => {
                h1602
            }
        };
        let a1604 = 1u64;
        nat_eq(a1603, a1604)
    } else {
        false
    } {
        let a1606 = match v1.clone() {
            Adt2::C0(_, _, _, h1605, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1607 = 1u64;
        nat_eq(a1606, a1607)
    } else {
        false
    } {
        let a1609 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1608, _, _, _, _, _, _) => {
                h1608
            }
        };
        let a1610 = 1u64;
        nat_eq(a1609, a1610)
    } else {
        false
    } {
        if f24(match v2.clone() {
            Adt2::C0(_, h1611, _, _, _, _, _, _, _, _, _) => {
                h1611
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h1612, _, _, _, _, _, _, _, _) => {
                h1612
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h1613, _, _, _, _, _, _, _) => {
                h1613
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h1614, _, _, _, _, _, _) => {
                h1614
            }
        }) {
            Ok(Adt6::C4(0u64, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a1616 = match v1.clone() {
                Adt2::C0(_, h1615, _, _, _, _, _, _, _, _, _) => {
                    h1615
                }
            };
            let a1617 = 0u64;
            if nat_eq(a1616, a1617) {
                Ok(Adt6::C4(f10(v2.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a1619 = match v0.clone() {
                    Adt2::C0(_, h1618, _, _, _, _, _, _, _, _, _) => {
                        h1618
                    }
                };
                let a1620 = 2147483647u64;
                if if if if if if nat_le(a1619, a1620) {
                    let a1622 = match v1.clone() {
                        Adt2::C0(_, h1621, _, _, _, _, _, _, _, _, _) => {
                            h1621
                        }
                    };
                    let a1623 = 2147483647u64;
                    nat_le(a1622, a1623)
                } else {
                    false
                } {
                    let a1625 = 4u64;
                    let a1626 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h1624, _, _, _, _, _) => {
                            h1624
                        }
                    };
                    nat_le(a1625, a1626)
                } else {
                    false
                } {
                    let a1632 = {
                        let a1629 = match v2.clone() {
                            Adt2::C0(_, h1627, _, _, _, _, _, _, _, _, _) => {
                                h1627
                            }
                        };
                        let a1630 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1628, _, _, _, _, _) => {
                                h1628
                            }
                        };
                        nat_mul(a1629, a1630)?
                    };
                    let a1633 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1631, _, _, _, _) => {
                            h1631
                        }
                    };
                    nat_le(a1632, a1633)
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
                    Ok(Adt6::C4({
                        let a1647 = {
                            let a1637 = {
                                let a1635 = 2u64;
                                let a1636 = match v1.clone() {
                                    Adt2::C0(_, h1634, _, _, _, _, _, _, _, _, _) => {
                                        h1634
                                    }
                                };
                                nat_mul(a1635, a1636)?
                            };
                            let a1638 = f10(v2.clone(), 1u64)?;
                            nat_add(a1637, a1638)?
                        };
                        let a1648 = {
                            let a1645 = match v1.clone() {
                                Adt2::C0(_, h1639, _, _, _, _, _, _, _, _, _) => {
                                    h1639
                                }
                            };
                            let a1646 = {
                                let a1643 = 1u64;
                                let a1644 = {
                                    let a1641 = 2u64;
                                    let a1642 = match v0.clone() {
                                        Adt2::C0(_, h1640, _, _, _, _, _, _, _, _, _) => {
                                            h1640
                                        }
                                    };
                                    nat_mul(a1641, a1642)?
                                };
                                nat_add(a1643, a1644)?
                            };
                            nat_mul(a1645, a1646)?
                        };
                        nat_add(a1647, a1648)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h1649, _, _, _, _, _, _, _, _, _) => {
                            h1649
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1650, _, _, _, _, _, _, _, _, _) => {
                            h1650
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h1651, _, _, _, _, _) => {
                            h1651
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1652, _, _, _, _) => {
                            h1652
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h1653, _, _, _) => {
                            h1653
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1654, _, _) => {
                            h1654
                        }
                    }), f14(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h1655, _, _, _, _, _, _, _, _, _, _) => {
                            h1655
                        }
                    }, f14(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f73(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1658 = match v1.clone() {
        Adt2::C0(h1656, _, _, _, _, _, _, _, _, _, _) => {
            h1656
        }
    };
    let a1659 = match v3.clone() {
        Adt2::C0(h1657, _, _, _, _, _, _, _, _, _, _) => {
            h1657
        }
    };
    if if nat_eq(a1658, a1659) {
        let a1660 = v0;
        let a1661 = 33u64;
        if nat_eq(a1660, a1661) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1663 = f75(match v1.clone() {
            Adt2::C0(h1662, _, _, _, _, _, _, _, _, _, _) => {
                h1662
            }
        });
        match m1663 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1664, _) => {
                        h1664
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a1669 = {
                    let a1666 = match v1.clone() {
                        Adt2::C0(_, h1665, _, _, _, _, _, _, _, _, _) => {
                            h1665
                        }
                    };
                    let a1667 = v5;
                    let a1668 = 1u64;
                    nat_rem(a1666, a1667, a1668)
                };
                let a1670 = 0u64;
                if if if nat_eq(a1669, a1670) {
                    let a1675 = {
                        let a1672 = match v3.clone() {
                            Adt2::C0(_, h1671, _, _, _, _, _, _, _, _, _) => {
                                h1671
                            }
                        };
                        let a1673 = v5;
                        let a1674 = 1u64;
                        nat_rem(a1672, a1673, a1674)
                    };
                    let a1676 = 0u64;
                    nat_eq(a1675, a1676)
                } else {
                    false
                } {
                    let a1677 = v6;
                    let a1678 = f10(v3.clone(), v5)?;
                    nat_eq(a1677, a1678)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1679) => {
                            h1679
                        }
                    }, f14(v1, v5), f14(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f74(v0: u64) -> bool {
    let a1680 = v0;
    let a1681 = 0u64;
    if if nat_eq(a1680, a1681) {
        true
    } else {
        let a1682 = v0;
        let a1683 = 1u64;
        nat_eq(a1682, a1683)
    } {
        true
    } else {
        let a1684 = v0;
        let a1685 = 30u64;
        nat_eq(a1684, a1685)
    }
}

pub fn f75(v0: u64) -> Adt10 {
    let a1686 = v0;
    let a1687 = 0u64;
    if nat_eq(a1686, a1687) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1688 = v0;
        let a1689 = 1u64;
        if nat_eq(a1688, a1689) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1690 = v0;
            let a1691 = 2u64;
            if nat_eq(a1690, a1691) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1692 = v0;
                let a1693 = 3u64;
                if nat_eq(a1692, a1693) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1694 = v0;
                    let a1695 = 6u64;
                    if nat_eq(a1694, a1695) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1696 = v0;
                        let a1697 = 7u64;
                        if nat_eq(a1696, a1697) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1698 = v0;
                            let a1699 = 8u64;
                            if nat_eq(a1698, a1699) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1700 = v0;
                                let a1701 = 9u64;
                                if nat_eq(a1700, a1701) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1702 = v0;
                                    let a1703 = 10u64;
                                    if nat_eq(a1702, a1703) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1704 = v0;
                                        let a1705 = 11u64;
                                        if nat_eq(a1704, a1705) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1706 = v0;
                                            let a1707 = 12u64;
                                            if nat_eq(a1706, a1707) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1708 = v0;
                                                let a1709 = 13u64;
                                                if nat_eq(a1708, a1709) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1710 = v0;
                                                    let a1711 = 14u64;
                                                    if nat_eq(a1710, a1711) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1712 = v0;
                                                        let a1713 = 15u64;
                                                        if nat_eq(a1712, a1713) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1714 = v0;
                                                            let a1715 = 16u64;
                                                            if nat_eq(a1714, a1715) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1716 = v0;
                                                                let a1717 = 17u64;
                                                                if nat_eq(a1716, a1717) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1718 = v0;
                                                                    let a1719 = 18u64;
                                                                    if nat_eq(a1718, a1719) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1720 = v0;
                                                                        let a1721 = 19u64;
                                                                        if nat_eq(a1720, a1721) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1722 = v0;
                                                                            let a1723 = 20u64;
                                                                            if nat_eq(a1722, a1723) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1724 = v0;
                                                                                let a1725 = 21u64;
                                                                                if nat_eq(a1724, a1725) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1726 = v0;
                                                                                    let a1727 = 22u64;
                                                                                    if nat_eq(a1726, a1727) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1728 = v0;
                                                                                        let a1729 = 23u64;
                                                                                        if nat_eq(a1728, a1729) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1730 = v0;
                                                                                            let a1731 = 24u64;
                                                                                            if nat_eq(a1730, a1731) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1732 = v0;
                                                                                                let a1733 = 25u64;
                                                                                                if nat_eq(a1732, a1733) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1734 = v0;
                                                                                                    let a1735 = 26u64;
                                                                                                    if nat_eq(a1734, a1735) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1736 = v0;
                                                                                                        let a1737 = 27u64;
                                                                                                        if nat_eq(a1736, a1737) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1738 = v0;
                                                                                                            let a1739 = 28u64;
                                                                                                            if nat_eq(a1738, a1739) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1740 = v0;
                                                                                                                let a1741 = 29u64;
                                                                                                                if nat_eq(a1740, a1741) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1742 = v0;
                                                                                                                    let a1743 = 30u64;
                                                                                                                    if nat_eq(a1742, a1743) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1744 = v0;
                                                                                                                        let a1745 = 34u64;
                                                                                                                        if nat_eq(a1744, a1745) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1746 = v0;
                                                                                                                            let a1747 = 35u64;
                                                                                                                            if nat_eq(a1746, a1747) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1748 = v0;
                                                                                                                                let a1749 = 39u64;
                                                                                                                                if nat_eq(a1748, a1749) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1750 = v0;
                                                                                                                                    let a1751 = 40u64;
                                                                                                                                    if nat_eq(a1750, a1751) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1752 = v0;
                                                                                                                                        let a1753 = 41u64;
                                                                                                                                        if nat_eq(a1752, a1753) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1754 = v0;
                                                                                                                                            let a1755 = 42u64;
                                                                                                                                            if nat_eq(a1754, a1755) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1756 = v0;
                                                                                                                                                let a1757 = 43u64;
                                                                                                                                                if nat_eq(a1756, a1757) {
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

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f24(v4, v5, v6, v7) {
        f24(v0, v1, v2, v3)
    } else {
        let a1761 = {
            let a1758 = v0;
            let a1759 = v4;
            let a1760 = 1u64;
            nat_rem(a1758, a1759, a1760)
        };
        let a1762 = 0u64;
        if if if nat_eq(a1761, a1762) {
            let a1766 = {
                let a1763 = v1;
                let a1764 = v5;
                let a1765 = 1u64;
                nat_rem(a1763, a1764, a1765)
            };
            let a1767 = 0u64;
            nat_eq(a1766, a1767)
        } else {
            false
        } {
            let a1771 = {
                let a1768 = v2;
                let a1769 = v6;
                let a1770 = 1u64;
                nat_rem(a1768, a1769, a1770)
            };
            let a1772 = 0u64;
            nat_eq(a1771, a1772)
        } else {
            false
        } {
            let a1776 = {
                let a1773 = v3;
                let a1774 = v7;
                let a1775 = 1u64;
                nat_rem(a1773, a1774, a1775)
            };
            let a1777 = 0u64;
            nat_eq(a1776, a1777)
        } else {
            false
        }
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1778 = v0;
    let a1779 = v1;
    if nat_eq(a1778, a1779) {
        let a1782 = v4;
        let a1783 = {
            let a1780 = v2;
            let a1781 = v3;
            nat_add(a1780, a1781)?
        };
        Ok(nat_eq(a1782, a1783))
    } else {
        let a1784 = v2;
        let a1785 = v3;
        if nat_eq(a1784, a1785) {
            let a1786 = v4;
            let a1787 = v2;
            Ok(nat_eq(a1786, a1787))
        } else {
            Ok(false)
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1790 = {
            let a1788 = v0;
            let a1789 = f124(v1);
            nat_add(a1788, a1789)?
        };
        let a1791 = f124(v2);
        nat_add(a1790, a1791)?
    };
    let v5: u64 = {
        let a1792 = f125(v1);
        let a1793 = f125(v2);
        nat_add(a1792, a1793)?
    };
    let a1794 = v1;
    let a1795 = 4294967295u64;
    if if nat_le(a1794, a1795) {
        let a1796 = v2;
        let a1797 = 4294967295u64;
        nat_le(a1796, a1797)
    } else {
        false
    } {
        let a1798 = v5;
        let a1799 = v4;
        if nat_le(a1798, a1799) {
            let a1802 = v3;
            let a1803 = {
                let a1800 = v4;
                let a1801 = v5;
                nat_sub(a1800, a1801)
            };
            Ok(nat_eq(a1802, a1803))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u64, v1: u64) -> R<bool> {
    let a1804 = v1;
    let a1805 = 4294967295u64;
    if nat_le(a1804, a1805) {
        let a1808 = {
            let a1806 = f124(v1);
            let a1807 = f125(v1);
            nat_add(a1806, a1807)?
        };
        let a1809 = v0;
        Ok(nat_lt(a1808, a1809))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f126(v0.clone(), false)?;
    let v6: u64 = f126(v0, true)?;
    let a1810 = v6;
    let a1811 = v1;
    if if nat_le(a1810, a1811) {
        let a1818 = {
            let a1816 = {
                let a1812 = v1;
                let a1813 = v5;
                nat_add(a1812, a1813)?
            };
            let a1817 = {
                let a1814 = v3;
                let a1815 = 0u64;
                if nat_eq(a1814, a1815) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1816, a1817)?
        };
        let a1819 = v2;
        nat_le(a1818, a1819)
    } else {
        false
    } {
        if v4 {
            let a1824 = {
                let a1822 = {
                    let a1820 = v1;
                    let a1821 = v5;
                    nat_add(a1820, a1821)?
                };
                let a1823 = v6;
                nat_sub(a1822, a1823)
            };
            let a1825 = v2;
            Ok(nat_lt(a1824, a1825))
        } else {
            let a1830 = {
                let a1828 = {
                    let a1826 = v1;
                    let a1827 = v5;
                    nat_add(a1826, a1827)?
                };
                let a1829 = v6;
                nat_sub(a1828, a1829)
            };
            let a1831 = v2;
            Ok(nat_le(a1830, a1831))
        }
    } else {
        Ok(false)
    }
}

pub fn f81(v0: u64, v1: u64) -> bool {
    let a1832 = v0;
    let a1833 = 0u64;
    if if nat_eq(a1832, a1833) {
        true
    } else {
        let a1834 = v0;
        let a1835 = 1u64;
        nat_eq(a1834, a1835)
    } {
        let a1836 = v1;
        let a1837 = 4294967296u64;
        nat_lt(a1836, a1837)
    } else {
        false
    }
}

pub fn f82(v0: u64, v1: u64) -> R<u64> {
    let a1842 = {
        let a1840 = v0;
        let a1841 = {
            let a1838 = v1;
            let a1839 = 1u64;
            nat_sub(a1838, a1839)
        };
        nat_add(a1840, a1841)?
    };
    let a1843 = v1;
    let a1844 = 0u64;
    Ok(nat_quot(a1842, a1843, a1844))
}

pub fn f83(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f24(match v0.clone() {
        Adt2::C0(_, h1845, _, _, _, _, _, _, _, _, _) => {
            h1845
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1846, _, _, _, _, _, _, _, _) => {
            h1846
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1847, _, _, _, _, _, _, _) => {
            h1847
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1848, _, _, _, _, _, _) => {
            h1848
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1851 = match v1.clone() {
                Adt2::C0(_, h1849, _, _, _, _, _, _, _, _, _) => {
                    h1849
                }
            };
            let a1852 = match v1.clone() {
                Adt2::C0(_, _, h1850, _, _, _, _, _, _, _, _) => {
                    h1850
                }
            };
            nat_mul(a1851, a1852)?
        };
        let v5: u64 = {
            let a1854 = v4;
            let a1855 = match v1.clone() {
                Adt2::C0(_, _, _, h1853, _, _, _, _, _, _, _) => {
                    h1853
                }
            };
            nat_mul(a1854, a1855)?
        };
        let a1868 = {
            let a1866 = {
                let a1857 = match v0.clone() {
                    Adt2::C0(_, h1856, _, _, _, _, _, _, _, _, _) => {
                        h1856
                    }
                };
                let a1858 = 1u64;
                nat_sub(a1857, a1858)
            };
            let a1867 = {
                let a1864 = f127(match v0.clone() {
                    Adt2::C0(_, _, h1859, _, _, _, _, _, _, _, _) => {
                        h1859
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1860, _, _, _, _, _, _, _, _, _) => {
                        h1860
                    }
                })?;
                let a1865 = f127(match v0 {
                    Adt2::C0(_, _, _, h1861, _, _, _, _, _, _, _) => {
                        h1861
                    }
                }, v2, {
                    let a1862 = v3;
                    let a1863 = v5;
                    nat_mul(a1862, a1863)?
                }, v4)?;
                nat_add(a1864, a1865)?
            };
            nat_add(a1866, a1867)?
        };
        let a1869 = f10(v1, 1u64)?;
        Ok(nat_lt(a1868, a1869))
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1870 = v0;
    let a1871 = 2147483648u64;
    if if if if nat_lt(a1870, a1871) {
        let a1872 = v1;
        let a1873 = 2147483648u64;
        nat_lt(a1872, a1873)
    } else {
        false
    } {
        let a1876 = {
            let a1874 = v0;
            let a1875 = v1;
            nat_mul(a1874, a1875)?
        };
        let a1877 = 2147483648u64;
        nat_lt(a1876, a1877)
    } else {
        false
    } {
        let a1882 = {
            let a1880 = {
                let a1878 = v0;
                let a1879 = 1u64;
                nat_sub(a1878, a1879)
            };
            let a1881 = v2;
            nat_mul(a1880, a1881)?
        };
        let a1883 = 2147483648u64;
        nat_lt(a1882, a1883)
    } else {
        false
    } {
        let a1888 = {
            let a1886 = {
                let a1884 = v1;
                let a1885 = 1u64;
                nat_sub(a1884, a1885)
            };
            let a1887 = v2;
            nat_mul(a1886, a1887)?
        };
        let a1889 = 2147483648u64;
        Ok(nat_lt(a1888, a1889))
    } else {
        Ok(false)
    }
}

pub fn f85(v0: u64) -> R<u64> {
    let a1892 = 3u64;
    let a1893 = {
        let a1890 = v0;
        let a1891 = 3u64;
        nat_mul(a1890, a1891)?
    };
    nat_add(a1892, a1893)
}

pub fn f86(v0: Adt3) -> R<u64> {
    if f24(match v0.clone() {
        Adt3::C0(h1894, _, _, _, _, _, _, _) => {
            h1894
        }
    }, match v0.clone() {
        Adt3::C0(_, h1895, _, _, _, _, _, _) => {
            h1895
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1896, _, _, _, _, _) => {
            h1896
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1897, _, _, _, _) => {
            h1897
        }
    }) {
        Ok(0u64)
    } else {
        let a1906 = match v0.clone() {
            Adt3::C0(h1898, _, _, _, _, _, _, _) => {
                h1898
            }
        };
        let a1907 = {
            let a1904 = match v0.clone() {
                Adt3::C0(_, h1899, _, _, _, _, _, _) => {
                    h1899
                }
            };
            let a1905 = {
                let a1902 = match v0.clone() {
                    Adt3::C0(_, _, h1900, _, _, _, _, _) => {
                        h1900
                    }
                };
                let a1903 = match v0 {
                    Adt3::C0(_, _, _, h1901, _, _, _, _) => {
                        h1901
                    }
                };
                nat_mul(a1902, a1903)?
            };
            nat_mul(a1904, a1905)?
        };
        nat_mul(a1906, a1907)
    }
}

pub fn f87(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1908 = v1;
    let a1909 = 13u64;
    if if nat_eq(a1908, a1909) {
        f24(match v0.clone() {
            Adt3::C0(h1910, _, _, _, _, _, _, _) => {
                h1910
            }
        }, match v0.clone() {
            Adt3::C0(_, h1911, _, _, _, _, _, _) => {
                h1911
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1912, _, _, _, _, _) => {
                h1912
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1913, _, _, _, _) => {
                h1913
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1919 = match v0.clone() {
                Adt3::C0(_, h1914, _, _, _, _, _, _) => {
                    h1914
                }
            };
            let a1920 = {
                let a1917 = match v0.clone() {
                    Adt3::C0(_, _, h1915, _, _, _, _, _) => {
                        h1915
                    }
                };
                let a1918 = match v0.clone() {
                    Adt3::C0(_, _, _, h1916, _, _, _, _) => {
                        h1916
                    }
                };
                nat_mul(a1917, a1918)?
            };
            nat_mul(a1919, a1920)?
        };
        let a1921 = v1;
        let a1922 = 13u64;
        if nat_eq(a1921, a1922) {
            let a1931 = 2u64;
            let a1932 = if f128(v0.clone(), v2) {
                let a1924 = v3;
                let a1925 = f129(match v0 {
                    Adt3::C0(h1923, _, _, _, _, _, _, _) => {
                        h1923
                    }
                }, false)?;
                nat_mul(a1924, a1925)?
            } else {
                let a1929 = v3;
                let a1930 = {
                    let a1927 = match v0 {
                        Adt3::C0(h1926, _, _, _, _, _, _, _) => {
                            h1926
                        }
                    };
                    let a1928 = 2u64;
                    nat_mul(a1927, a1928)?
                };
                nat_mul(a1929, a1930)?
            };
            nat_add(a1931, a1932)
        } else {
            let a1936 = v3;
            let a1937 = f129(match v0 {
                Adt3::C0(h1933, _, _, _, _, _, _, _) => {
                    h1933
                }
            }, {
                let a1934 = v1;
                let a1935 = 16u64;
                nat_eq(a1934, a1935)
            })?;
            nat_mul(a1936, a1937)
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1938 = v3;
    match m1938 {
        None => {
            f130(v0, v1, match v2.clone() {
                Adt2::C0(h1939, _, _, _, _, _, _, _, _, _, _) => {
                    h1939
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1940, _, _, _, _, _, _, _, _, _, _) => {
                    h1940
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1941, _, _, _, _, _, _, _, _, _) => {
                    h1941
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1942, _, _, _, _, _, _, _, _) => {
                    h1942
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1943, _, _, _, _, _, _, _) => {
                    h1943
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1944, _, _, _, _, _, _) => {
                    h1944
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1945, _, _, _, _, _, _, _, _, _) => {
                    h1945
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1946, _, _, _, _, _, _, _, _) => {
                    h1946
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1947, _, _, _, _, _, _, _) => {
                    h1947
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1948, _, _, _, _, _, _) => {
                    h1948
                }
            })
        }
        Some(v5) => {
            f130(v0, v1, match v2.clone() {
                Adt2::C0(h1949, _, _, _, _, _, _, _, _, _, _) => {
                    h1949
                }
            }, match v5.clone() {
                Adt2::C0(h1950, _, _, _, _, _, _, _, _, _, _) => {
                    h1950
                }
            }, match v4.clone() {
                Adt2::C0(h1951, _, _, _, _, _, _, _, _, _, _) => {
                    h1951
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1952, _, _, _, _, _, _, _, _, _) => {
                    h1952
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1953, _, _, _, _, _, _, _, _) => {
                    h1953
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1954, _, _, _, _, _, _, _) => {
                    h1954
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1955, _, _, _, _, _, _) => {
                    h1955
                }
            }, match v5.clone() {
                Adt2::C0(_, h1956, _, _, _, _, _, _, _, _, _) => {
                    h1956
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1957, _, _, _, _, _, _, _, _) => {
                    h1957
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1958, _, _, _, _, _, _, _) => {
                    h1958
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1959, _, _, _, _, _, _) => {
                    h1959
                }
            }, match v4.clone() {
                Adt2::C0(_, h1960, _, _, _, _, _, _, _, _, _) => {
                    h1960
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1961, _, _, _, _, _, _, _, _) => {
                    h1961
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1962, _, _, _, _, _, _, _) => {
                    h1962
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1963, _, _, _, _, _, _) => {
                    h1963
                }
            })
        }
    }
}

pub fn f89(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1964 = v7;
        let a1965 = 6u64;
        if nat_eq(a1964, a1965) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f61(v3, v4), f61(v5, v6), f62(v7, v8), f131(v1), f131(v9))
}

pub fn f90(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1966 = v0;
    match m1966 {
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
            let a1967 = 13u64;
            let a1968 = v34;
            if nat_le(a1967, a1968) {
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

pub fn f91(v0: Adt4) -> u64 {
    let m1969 = v0;
    match m1969 {
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

pub fn f92(v0: Adt4, v1: u64) -> u64 {
    let m1970 = v0;
    match m1970 {
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
            let a1971 = v1;
            let a1972 = 6u64;
            if nat_eq(a1971, a1972) {
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
            let a1973 = v1;
            let a1974 = 6u64;
            if if if nat_eq(a1973, a1974) {
                true
            } else {
                let a1975 = v1;
                let a1976 = 7u64;
                nat_eq(a1975, a1976)
            } {
                let a1977 = v49;
                let a1978 = 30u64;
                nat_eq(a1977, a1978)
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

pub fn f93(v0: Adt4, v1: u64) -> Adt7 {
    let a1979 = v1;
    let a1980 = 4u64;
    if nat_lt(a1979, a1980) {
        Adt7::C0(v1)
    } else {
        let a1981 = v1;
        let a1982 = 4u64;
        if nat_eq(a1981, a1982) {
            Adt7::C1(0u64)
        } else {
            let a1983 = v1;
            let a1984 = 5u64;
            if nat_eq(a1983, a1984) {
                Adt7::C1(3u64)
            } else {
                let a1985 = v1;
                let a1986 = 6u64;
                if nat_eq(a1985, a1986) {
                    Adt7::C2(f92(v0, v1), 0u64, false)
                } else {
                    let a1987 = v1;
                    let a1988 = 7u64;
                    if nat_eq(a1987, a1988) {
                        Adt7::C2(0u64, f92(v0, v1), false)
                    } else {
                        let a1989 = v1;
                        let a1990 = 16u64;
                        if nat_lt(a1989, a1990) {
                            Adt7::C1({
                                let a1991 = v1;
                                let a1992 = 8u64;
                                nat_sub(a1991, a1992)
                            })
                        } else {
                            let a1993 = v1;
                            let a1994 = 20u64;
                            if nat_lt(a1993, a1994) {
                                Adt7::C5({
                                    let a1995 = v1;
                                    let a1996 = 16u64;
                                    nat_sub(a1995, a1996)
                                })
                            } else {
                                let a1997 = v1;
                                let a1998 = 20u64;
                                if nat_eq(a1997, a1998) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1999 = v1;
                                    let a2000 = 21u64;
                                    if nat_eq(a1999, a2000) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a2001 = v1;
                                        let a2002 = 22u64;
                                        if nat_eq(a2001, a2002) {
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

pub fn f94(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2004 = v0;
        let a2005 = match v1.clone() {
            Adt3::C0(h2003, _, _, _, _, _, _, _) => {
                h2003
            }
        };
        let a2006 = 0u64;
        nat_quot(a2004, a2005, a2006)
    };
    let v3: u64 = {
        let a2008 = v2;
        let a2009 = match v1.clone() {
            Adt3::C0(_, h2007, _, _, _, _, _, _) => {
                h2007
            }
        };
        let a2010 = 0u64;
        nat_quot(a2008, a2009, a2010)
    };
    f132({
        let a2012 = v0;
        let a2013 = match v1.clone() {
            Adt3::C0(h2011, _, _, _, _, _, _, _) => {
                h2011
            }
        };
        let a2014 = 0u64;
        nat_rem(a2012, a2013, a2014)
    }, {
        let a2016 = v2;
        let a2017 = match v1.clone() {
            Adt3::C0(_, h2015, _, _, _, _, _, _) => {
                h2015
            }
        };
        let a2018 = 0u64;
        nat_rem(a2016, a2017, a2018)
    }, {
        let a2020 = v3;
        let a2021 = match v1.clone() {
            Adt3::C0(_, _, h2019, _, _, _, _, _) => {
                h2019
            }
        };
        let a2022 = 0u64;
        nat_rem(a2020, a2021, a2022)
    }, {
        let a2024 = v3;
        let a2025 = match v1.clone() {
            Adt3::C0(_, _, h2023, _, _, _, _, _) => {
                h2023
            }
        };
        let a2026 = 0u64;
        nat_quot(a2024, a2025, a2026)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2027, _, _, _) => {
            h2027
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2028, _, _) => {
            h2028
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2029, _) => {
            h2029
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2030) => {
            h2030
        }
    })
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2048 = {
        let a2043 = f133(v2, {
            let a2034 = {
                let a2031 = v0;
                let a2032 = v1;
                let a2033 = 0u64;
                nat_quot(a2031, a2032, a2033)
            };
            let a2035 = v2;
            let a2036 = 0u64;
            nat_rem(a2034, a2035, a2036)
        }, {
            let a2040 = {
                let a2037 = v0;
                let a2038 = v1;
                let a2039 = 0u64;
                nat_quot(a2037, a2038, a2039)
            };
            let a2041 = v2;
            let a2042 = 0u64;
            nat_quot(a2040, a2041, a2042)
        })?;
        let a2044 = v1;
        nat_mul(a2043, a2044)?
    };
    let a2049 = {
        let a2045 = v0;
        let a2046 = v1;
        let a2047 = 0u64;
        nat_rem(a2045, a2046, a2047)
    };
    nat_add(a2048, a2049)
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2051 = v0;
        let a2052 = match v2.clone() {
            Adt3::C0(h2050, _, _, _, _, _, _, _) => {
                h2050
            }
        };
        let a2053 = 0u64;
        nat_quot(a2051, a2052, a2053)
    };
    let v4: u64 = {
        let a2055 = v3;
        let a2056 = match v2.clone() {
            Adt3::C0(_, h2054, _, _, _, _, _, _) => {
                h2054
            }
        };
        let a2057 = 0u64;
        nat_quot(a2055, a2056, a2057)
    };
    f134({
        let a2059 = v0;
        let a2060 = match v2.clone() {
            Adt3::C0(h2058, _, _, _, _, _, _, _) => {
                h2058
            }
        };
        let a2061 = 0u64;
        nat_rem(a2059, a2060, a2061)
    }, {
        let a2063 = v3;
        let a2064 = match v2.clone() {
            Adt3::C0(_, h2062, _, _, _, _, _, _) => {
                h2062
            }
        };
        let a2065 = 0u64;
        nat_rem(a2063, a2064, a2065)
    }, {
        let a2067 = v4;
        let a2068 = match v2.clone() {
            Adt3::C0(_, _, h2066, _, _, _, _, _) => {
                h2066
            }
        };
        let a2069 = 0u64;
        nat_rem(a2067, a2068, a2069)
    }, {
        let a2071 = v4;
        let a2072 = match v2 {
            Adt3::C0(_, _, h2070, _, _, _, _, _) => {
                h2070
            }
        };
        let a2073 = 0u64;
        nat_quot(a2071, a2072, a2073)
    }, match v1.clone() {
        Adt3::C0(h2074, _, _, _, _, _, _, _) => {
            h2074
        }
    }, match v1.clone() {
        Adt3::C0(_, h2075, _, _, _, _, _, _) => {
            h2075
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2076, _, _, _, _, _) => {
            h2076
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2077, _, _, _, _) => {
            h2077
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2078, _, _, _) => {
            h2078
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2079, _, _) => {
            h2079
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2080, _) => {
            h2080
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2081) => {
            h2081
        }
    })
}

pub fn f97(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2083 = v0;
        let a2084 = match v3.clone() {
            Adt3::C0(h2082, _, _, _, _, _, _, _) => {
                h2082
            }
        };
        let a2085 = 0u64;
        nat_quot(a2083, a2084, a2085)
    };
    let v6: u64 = {
        let a2087 = v5;
        let a2088 = match v3.clone() {
            Adt3::C0(_, h2086, _, _, _, _, _, _) => {
                h2086
            }
        };
        let a2089 = 0u64;
        nat_quot(a2087, a2088, a2089)
    };
    let v7: u64 = {
        let a2091 = v0;
        let a2092 = match v3.clone() {
            Adt3::C0(h2090, _, _, _, _, _, _, _) => {
                h2090
            }
        };
        let a2093 = 0u64;
        nat_rem(a2091, a2092, a2093)
    };
    let v8: u64 = {
        let a2095 = v5;
        let a2096 = match v3.clone() {
            Adt3::C0(_, h2094, _, _, _, _, _, _) => {
                h2094
            }
        };
        let a2097 = 0u64;
        nat_rem(a2095, a2096, a2097)
    };
    let v9: u64 = {
        let a2099 = v6;
        let a2100 = match v3.clone() {
            Adt3::C0(_, _, h2098, _, _, _, _, _) => {
                h2098
            }
        };
        let a2101 = 0u64;
        nat_rem(a2099, a2100, a2101)
    };
    let v10: u64 = {
        let a2103 = v6;
        let a2104 = match v3.clone() {
            Adt3::C0(_, _, h2102, _, _, _, _, _) => {
                h2102
            }
        };
        let a2105 = 0u64;
        nat_quot(a2103, a2104, a2105)
    };
    let v11: u64 = {
        let a2106 = v4;
        let a2107 = 0u64;
        if nat_eq(a2106, a2107) {
            match v1.clone() {
                Adt3::C0(h2108, _, _, _, _, _, _, _) => {
                    h2108
                }
            }
        } else {
            let a2109 = v4;
            let a2110 = 1u64;
            if nat_eq(a2109, a2110) {
                match v1.clone() {
                    Adt3::C0(_, h2111, _, _, _, _, _, _) => {
                        h2111
                    }
                }
            } else {
                let a2112 = v4;
                let a2113 = 2u64;
                if nat_eq(a2112, a2113) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2114, _, _, _, _, _) => {
                            h2114
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2115, _, _, _, _) => {
                            h2115
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2122 = v11;
        let a2123 = {
            let a2116 = v4;
            let a2117 = 0u64;
            if nat_eq(a2116, a2117) {
                v7
            } else {
                let a2118 = v4;
                let a2119 = 1u64;
                if nat_eq(a2118, a2119) {
                    v8
                } else {
                    let a2120 = v4;
                    let a2121 = 2u64;
                    if nat_eq(a2120, a2121) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2122, a2123)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f132(if if v12 {
        let a2124 = v4;
        let a2125 = 0u64;
        nat_eq(a2124, a2125)
    } else {
        false
    } {
        let a2126 = v7;
        let a2127 = v11;
        nat_sub(a2126, a2127)
    } else {
        v7
    }, if if v12 {
        let a2128 = v4;
        let a2129 = 1u64;
        nat_eq(a2128, a2129)
    } else {
        false
    } {
        let a2130 = v8;
        let a2131 = v11;
        nat_sub(a2130, a2131)
    } else {
        v8
    }, if if v12 {
        let a2132 = v4;
        let a2133 = 2u64;
        nat_eq(a2132, a2133)
    } else {
        false
    } {
        let a2134 = v9;
        let a2135 = v11;
        nat_sub(a2134, a2135)
    } else {
        v9
    }, if if v12 {
        let a2136 = v4;
        let a2137 = 3u64;
        nat_eq(a2136, a2137)
    } else {
        false
    } {
        let a2138 = v10;
        let a2139 = v11;
        nat_sub(a2138, a2139)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2140, _, _, _) => {
            h2140
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2141, _, _) => {
            h2141
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2142, _) => {
            h2142
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2143) => {
            h2143
        }
    })?, f94(v0, v3)?))
}

pub fn f98(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f135(v0, v2.clone(), 0u64);
    let v9: u64 = f135(v0, v2.clone(), 1u64);
    let v10: u64 = f135(v0, v2.clone(), 2u64);
    let v11: u64 = f135(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f136(v1.clone(), f137(v8, v3, match v1.clone() {
            Adt3::C0(h2144, _, _, _, _, _, _, _) => {
                h2144
            }
        })?, f137(v9, v4, match v1.clone() {
            Adt3::C0(_, h2145, _, _, _, _, _, _) => {
                h2145
            }
        })?, f137(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2146, _, _, _, _, _) => {
                h2146
            }
        })?, f137(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2147, _, _, _, _) => {
                h2147
            }
        })?)?, f94(v0, v2)?))
    } else {
        if if if if f138(v8, v3, match v1.clone() {
            Adt3::C0(h2148, _, _, _, _, _, _, _) => {
                h2148
            }
        })? {
            f138(v9, v4, match v1.clone() {
                Adt3::C0(_, h2149, _, _, _, _, _, _) => {
                    h2149
                }
            })?
        } else {
            false
        } {
            f138(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2150, _, _, _, _, _) => {
                    h2150
                }
            })?
        } else {
            false
        } {
            f138(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2151, _, _, _, _) => {
                    h2151
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f136(v1, f139(v8, v3)?, f139(v9, v4)?, f139(v10, v5)?, f139(v11, v6)?)?, f94(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f94(v0, v2)?))
        }
    }
}

pub fn f99(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f135(v0, v2.clone(), 0u64);
    let v5: u64 = f135(v0, v2.clone(), 1u64);
    let v6: u64 = f135(v0, v2.clone(), 2u64);
    let v7: u64 = f135(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f136(v1.clone(), f140(v4, v3, match v1 {
        Adt3::C0(h2152, _, _, _, _, _, _, _) => {
            h2152
        }
    })?, v5, v6, v7)?, f94(v0, v2)?))
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2153 = v0;
    let a2154 = v3;
    if nat_lt(a2153, a2154) {
        Ok(Adt5::C1(false, {
            let a2155 = v0;
            let a2156 = 4u64;
            nat_mul(a2155, a2156)?
        }, {
            let a2157 = v0;
            let a2158 = 4u64;
            nat_mul(a2157, a2158)?
        }))
    } else {
        let v6: u64 = {
            let a2159 = v0;
            let a2160 = v3;
            nat_sub(a2159, a2160)
        };
        let v7: u64 = f94(v6, v1)?;
        let v8: u64 = f141(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2161 = v0;
    let a2162 = v3;
    if nat_lt(a2161, a2162) {
        Ok(Adt5::C1(true, 0u64, f94(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2163 = v0;
            let a2164 = v3;
            nat_sub(a2163, a2164)
        };
        let v5: u64 = f96(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f94(v4, v1)?, 3u64, v5))
    }
}

pub fn f102(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2166 = v1;
        let a2167 = match v3.clone() {
            Adt3::C0(h2165, _, _, _, _, _, _, _) => {
                h2165
            }
        };
        let a2168 = 0u64;
        nat_rem(a2166, a2167, a2168)
    };
    let v5: u64 = {
        let a2174 = {
            let a2170 = v1;
            let a2171 = match v3.clone() {
                Adt3::C0(h2169, _, _, _, _, _, _, _) => {
                    h2169
                }
            };
            let a2172 = 0u64;
            nat_quot(a2170, a2171, a2172)
        };
        let a2175 = match v3.clone() {
            Adt3::C0(_, h2173, _, _, _, _, _, _) => {
                h2173
            }
        };
        let a2176 = 0u64;
        nat_rem(a2174, a2175, a2176)
    };
    let v6: bool = f142(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f96(v1, v2, v3.clone())?
    } else {
        0u64
    }, f94(v1, v3)?))
}

pub fn f103(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a2177 = v3;
        let a2178 = 8u64;
        nat_add(a2177, a2178)?
    }, 0u64, f94(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f96(v0, v5, v2.clone())?
    }, 3u64, f94(v0, v2)?))
}

pub fn f104(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a2180 = v0;
        let a2181 = match v2.clone() {
            Adt3::C0(h2179, _, _, _, _, _, _, _) => {
                h2179
            }
        };
        let a2182 = 0u64;
        nat_rem(a2180, a2181, a2182)
    };
    let v7: u64 = {
        let a2184 = v0;
        let a2185 = match v2.clone() {
            Adt3::C0(h2183, _, _, _, _, _, _, _) => {
                h2183
            }
        };
        let a2186 = 0u64;
        nat_quot(a2184, a2185, a2186)
    };
    let v8: u64 = {
        let a2188 = v7;
        let a2189 = match v2.clone() {
            Adt3::C0(_, h2187, _, _, _, _, _, _) => {
                h2187
            }
        };
        let a2190 = 0u64;
        nat_rem(a2188, a2189, a2190)
    };
    let v9: u64 = {
        let a2192 = v7;
        let a2193 = match v2.clone() {
            Adt3::C0(_, h2191, _, _, _, _, _, _) => {
                h2191
            }
        };
        let a2194 = 0u64;
        nat_quot(a2192, a2193, a2194)
    };
    let v10: u64 = {
        let a2196 = v9;
        let a2197 = match v2.clone() {
            Adt3::C0(_, _, h2195, _, _, _, _, _) => {
                h2195
            }
        };
        let a2198 = 0u64;
        nat_rem(a2196, a2197, a2198)
    };
    let v11: u64 = {
        let a2200 = v9;
        let a2201 = match v2.clone() {
            Adt3::C0(_, _, h2199, _, _, _, _, _) => {
                h2199
            }
        };
        let a2202 = 0u64;
        nat_quot(a2200, a2201, a2202)
    };
    let v12: u64 = f94(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2211 = {
                let a2206 = {
                    let a2203 = v10;
                    let a2204 = v4;
                    let a2205 = 0u64;
                    nat_quot(a2203, a2204, a2205)
                };
                let a2207 = v5;
                nat_mul(a2206, a2207)?
            };
            let a2212 = {
                let a2208 = v8;
                let a2209 = v4;
                let a2210 = 0u64;
                nat_quot(a2208, a2209, a2210)
            };
            nat_add(a2211, a2212)?
        };
        let v14: u64 = {
            let a2232 = v6;
            let a2233 = {
                let a2230 = match v1.clone() {
                    Adt3::C0(h2213, _, _, _, _, _, _, _) => {
                        h2213
                    }
                };
                let a2231 = {
                    let a2228 = {
                        let a2214 = v8;
                        let a2215 = v4;
                        let a2216 = 0u64;
                        nat_rem(a2214, a2215, a2216)
                    };
                    let a2229 = {
                        let a2226 = match v1.clone() {
                            Adt3::C0(_, h2217, _, _, _, _, _, _) => {
                                h2217
                            }
                        };
                        let a2227 = {
                            let a2224 = {
                                let a2218 = v10;
                                let a2219 = v4;
                                let a2220 = 0u64;
                                nat_rem(a2218, a2219, a2220)
                            };
                            let a2225 = {
                                let a2222 = match v1.clone() {
                                    Adt3::C0(_, _, h2221, _, _, _, _, _) => {
                                        h2221
                                    }
                                };
                                let a2223 = v13;
                                nat_mul(a2222, a2223)?
                            };
                            nat_add(a2224, a2225)?
                        };
                        nat_mul(a2226, a2227)?
                    };
                    nat_add(a2228, a2229)?
                };
                nat_mul(a2230, a2231)?
            };
            nat_add(a2232, a2233)?
        };
        Ok(Adt5::C1(false, f94(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2239 = {
                let a2237 = {
                    let a2234 = v11;
                    let a2235 = v5;
                    let a2236 = 0u64;
                    nat_rem(a2234, a2235, a2236)
                };
                let a2238 = v4;
                nat_mul(a2237, a2238)?
            };
            let a2240 = v8;
            nat_add(a2239, a2240)?
        };
        let v16: u64 = {
            let a2246 = {
                let a2244 = {
                    let a2241 = v11;
                    let a2242 = v5;
                    let a2243 = 0u64;
                    nat_quot(a2241, a2242, a2243)
                };
                let a2245 = v4;
                nat_mul(a2244, a2245)?
            };
            let a2247 = v10;
            nat_add(a2246, a2247)?
        };
        let a2249 = v15;
        let a2250 = match v1.clone() {
            Adt3::C0(_, h2248, _, _, _, _, _, _) => {
                h2248
            }
        };
        if if nat_lt(a2249, a2250) {
            let a2252 = v16;
            let a2253 = match v1.clone() {
                Adt3::C0(_, _, h2251, _, _, _, _, _) => {
                    h2251
                }
            };
            nat_lt(a2252, a2253)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f94({
                let a2262 = v6;
                let a2263 = {
                    let a2260 = match v1.clone() {
                        Adt3::C0(h2254, _, _, _, _, _, _, _) => {
                            h2254
                        }
                    };
                    let a2261 = {
                        let a2258 = v15;
                        let a2259 = {
                            let a2256 = match v1.clone() {
                                Adt3::C0(_, h2255, _, _, _, _, _, _) => {
                                    h2255
                                }
                            };
                            let a2257 = v16;
                            nat_mul(a2256, a2257)?
                        };
                        nat_add(a2258, a2259)?
                    };
                    nat_mul(a2260, a2261)?
                };
                nat_add(a2262, a2263)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f105(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f94(v0, v2.clone())?;
    let v4: u64 = f94(v0, v1.clone())?;
    let a2268 = {
        let a2265 = v0;
        let a2266 = match v1 {
            Adt3::C0(h2264, _, _, _, _, _, _, _) => {
                h2264
            }
        };
        let a2267 = 0u64;
        nat_rem(a2265, a2266, a2267)
    };
    let a2269 = 0u64;
    if nat_eq(a2268, a2269) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f94({
            let a2270 = v0;
            let a2271 = 1u64;
            nat_sub(a2270, a2271)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f106(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2272 = v3;
    let a2273 = 13u64;
    if nat_eq(a2272, a2273) {
        f143(v0, v1, v4)
    } else {
        let v5: u64 = f129(match v1.clone() {
            Adt3::C0(h2274, _, _, _, _, _, _, _) => {
                h2274
            }
        }, {
            let a2275 = v3;
            let a2276 = 16u64;
            nat_eq(a2275, a2276)
        })?;
        let v6: u64 = {
            let a2277 = v0;
            let a2278 = v5;
            let a2279 = 0u64;
            nat_quot(a2277, a2278, a2279)
        };
        let v7: u64 = {
            let a2280 = v0;
            let a2281 = v5;
            let a2282 = 0u64;
            nat_rem(a2280, a2281, a2282)
        };
        let v8: u64 = {
            let a2286 = 1u64;
            let a2287 = {
                let a2284 = match v1.clone() {
                    Adt3::C0(h2283, _, _, _, _, _, _, _) => {
                        h2283
                    }
                };
                let a2285 = 2u64;
                nat_mul(a2284, a2285)?
            };
            nat_add(a2286, a2287)?
        };
        let a2288 = v7;
        let a2289 = v8;
        if nat_lt(a2288, a2289) {
            f144(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f94(v6, v2)?;
            let a2290 = v3;
            let a2291 = 14u64;
            if nat_eq(a2290, a2291) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2292 = v7;
                let a2293 = v8;
                if nat_eq(a2292, a2293) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2296 = v7;
                    let a2297 = {
                        let a2294 = v8;
                        let a2295 = 1u64;
                        nat_add(a2294, a2295)?
                    };
                    if nat_eq(a2296, a2297) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2298, _, _, _, _, _, _, _) => {
                                h2298
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

pub fn f107(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2299 = v0;
    let a2300 = 0u64;
    if nat_eq(a2299, a2300) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2307 = {
            let a2301 = v0;
            let a2302 = 1u64;
            nat_add(a2301, a2302)?
        };
        let a2308 = {
            let a2305 = 2u64;
            let a2306 = {
                let a2303 = v3;
                let a2304 = 2u64;
                nat_mul(a2303, a2304)?
            };
            nat_add(a2305, a2306)?
        };
        if nat_eq(a2307, a2308) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2309 = v0;
                let a2310 = 1u64;
                nat_sub(a2309, a2310)
            };
            let v5: u64 = {
                let a2311 = v4;
                let a2312 = 2u64;
                let a2313 = 0u64;
                nat_quot(a2311, a2312, a2313)
            };
            let a2317 = {
                let a2314 = v4;
                let a2315 = 2u64;
                let a2316 = 0u64;
                nat_rem(a2314, a2315, a2316)
            };
            let a2318 = 0u64;
            if nat_eq(a2317, a2318) {
                Ok(Adt5::C4(17u64, 0u64, f94(v5, v1)?, 4u64, f94(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2319 = v0;
    let a2320 = f19(v1, v2, v3)?;
    if nat_lt(a2319, a2320) {
        let a2321 = v0;
        let a2322 = v1;
        if nat_lt(a2321, a2322) {
            Ok(Adt5::C1(false, {
                let a2323 = 4u64;
                let a2324 = v0;
                nat_mul(a2323, a2324)?
            }, {
                let a2325 = 4u64;
                let a2326 = v0;
                nat_mul(a2325, a2326)?
            }))
        } else {
            let v4: u64 = {
                let a2327 = v0;
                let a2328 = v1;
                nat_sub(a2327, a2328)
            };
            let v5: u64 = f145(v4, v2)?;
            let a2329 = v5;
            let a2330 = 2u64;
            if nat_lt(a2329, a2330) {
                Ok(Adt5::C4(6u64, {
                    let a2331 = v5;
                    let a2332 = 0u64;
                    if nat_eq(a2331, a2332) {
                        4u64
                    } else {
                        5u64
                    }
                }, f146(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f147(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f148(v4, v2)? {
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

pub fn f109(v0: Adt3, v1: Adt3) -> R<u64> {
    let a2340 = {
        let a2333 = 2u64;
        let a2334 = f86(v1.clone())?;
        nat_mul(a2333, a2334)?
    };
    let a2341 = {
        let a2338 = f86(v1)?;
        let a2339 = {
            let a2336 = 1u64;
            let a2337 = match v0 {
                Adt3::C0(h2335, _, _, _, _, _, _, _) => {
                    h2335
                }
            };
            nat_add(a2336, a2337)?
        };
        nat_mul(a2338, a2339)?
    };
    nat_add(a2340, a2341)
}

pub fn f110(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f94(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f111(v0: u64, v1: u64) -> bool {
    let a2342 = v1;
    let a2343 = 2147483647u64;
    if nat_le(a2342, a2343) {
        let a2344 = v1;
        let a2345 = v0;
        nat_lt(a2344, a2345)
    } else {
        false
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2346 = f75(v0);
    match m2346 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2348 = v2;
                let a2349 = match v7.clone() {
                    Adt11::C0(_, h2347, _) => {
                        h2347
                    }
                };
                nat_mul(a2348, a2349)?
            };
            let m2362 = f149(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2350, _, _, _, _, _, _, _) => {
                    h2350
                }
            }, match v5.clone() {
                Adt3::C0(_, h2351, _, _, _, _, _, _) => {
                    h2351
                }
            }, match v5 {
                Adt3::C0(_, _, h2352, _, _, _, _, _) => {
                    h2352
                }
            }, {
                let a2355 = match v3.clone() {
                    Adt3::C0(h2353, _, _, _, _, _, _, _) => {
                        h2353
                    }
                };
                let a2356 = match v7.clone() {
                    Adt11::C0(_, h2354, _) => {
                        h2354
                    }
                };
                nat_mul(a2355, a2356)?
            }, match v3.clone() {
                Adt3::C0(_, h2357, _, _, _, _, _, _) => {
                    h2357
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2358, _, _, _) => {
                    h2358
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2359, _, _) => {
                    h2359
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2360, _) => {
                    h2360
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2361) => {
                    h2361
                }
            })?;
            match m2362 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2363 = v0;
                    let a2364 = 0u64;
                    if if nat_eq(a2363, a2364) {
                        true
                    } else {
                        let a2365 = v0;
                        let a2366 = 26u64;
                        nat_eq(a2365, a2366)
                    } {
                        Ok(Adt5::C1(false, v9, f94({
                            let a2370 = {
                                let a2368 = v1;
                                let a2369 = match v4.clone() {
                                    Adt3::C0(h2367, _, _, _, _, _, _, _) => {
                                        h2367
                                    }
                                };
                                nat_mul(a2368, a2369)?
                            };
                            let a2371 = v8;
                            nat_add(a2370, a2371)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2372) => {
                                h2372
                            }
                        }, f94({
                            let a2376 = {
                                let a2374 = v1;
                                let a2375 = match v4.clone() {
                                    Adt3::C0(h2373, _, _, _, _, _, _, _) => {
                                        h2373
                                    }
                                };
                                nat_mul(a2374, a2375)?
                            };
                            let a2377 = v8;
                            nat_add(a2376, a2377)?
                        }, v4.clone())?, {
                            let a2384 = {
                                let a2382 = {
                                    let a2379 = match v7 {
                                        Adt11::C0(_, h2378, _) => {
                                            h2378
                                        }
                                    };
                                    let a2380 = 1u64;
                                    nat_sub(a2379, a2380)
                                };
                                let a2383 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2381, _, _, _) => {
                                        h2381
                                    }
                                };
                                nat_mul(a2382, a2383)?
                            };
                            let a2385 = 4u64;
                            nat_add(a2384, a2385)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2386, _, _, _) => {
                                h2386
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2387 = v1;
    let a2388 = 0u64;
    if if nat_eq(a2387, a2388) {
        true
    } else {
        let a2389 = v2;
        let a2390 = 0u64;
        nat_eq(a2389, a2390)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f114(v0: u64, v1: u64) -> bool {
    let a2391 = v0;
    let a2392 = 4294967295u64;
    if nat_eq(a2391, a2392) {
        true
    } else {
        let a2393 = v0;
        let a2394 = 2147483648u64;
        if nat_lt(a2393, a2394) {
            let a2395 = v0;
            let a2396 = v1;
            nat_lt(a2395, a2396)
        } else {
            false
        }
    }
}

pub fn f115(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2403 = v3;
        let a2404 = {
            let a2400 = f94({
                let a2397 = v0;
                let a2398 = 2u64;
                let a2399 = 0u64;
                nat_quot(a2397, a2398, a2399)
            }, v1)?;
            let a2401 = v2;
            let a2402 = 0u64;
            nat_quot(a2400, a2401, a2402)
        };
        nat_add(a2403, a2404)?
    };
    let a2408 = {
        let a2405 = v0;
        let a2406 = 2u64;
        let a2407 = 0u64;
        nat_rem(a2405, a2406, a2407)
    };
    let a2409 = 0u64;
    if nat_eq(a2408, a2409) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2410 = v4;
        let a2411 = 0u64;
        if nat_eq(a2410, a2411) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f116(v0: u64, v1: u64) -> R<u64> {
    let a2414 = 8u64;
    let a2415 = {
        let a2412 = f150(v0);
        let a2413 = v1;
        nat_mul(a2412, a2413)?
    };
    nat_add(a2414, a2415)
}

pub fn f117(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2416, _, _, _, _, _, _, _) => {
            h2416
        }
    };
    let v7: u64 = f150(v3);
    let v8: u64 = {
        let a2419 = 1u64;
        let a2420 = {
            let a2417 = v6;
            let a2418 = v7;
            nat_mul(a2417, a2418)?
        };
        nat_add(a2419, a2420)?
    };
    let a2421 = v0;
    let a2422 = 0u64;
    if nat_eq(a2421, a2422) {
        Ok(f151(0u64))
    } else {
        let a2423 = v0;
        let a2424 = v8;
        if nat_lt(a2423, a2424) {
            let v9: u64 = {
                let a2425 = v0;
                let a2426 = 1u64;
                nat_sub(a2425, a2426)
            };
            let v10: u64 = {
                let a2427 = v9;
                let a2428 = v7;
                let a2429 = 0u64;
                nat_rem(a2427, a2428, a2429)
            };
            let v11: u64 = f94({
                let a2435 = {
                    let a2430 = v1;
                    let a2431 = v6;
                    nat_mul(a2430, a2431)?
                };
                let a2436 = {
                    let a2432 = v9;
                    let a2433 = v7;
                    let a2434 = 0u64;
                    nat_quot(a2432, a2433, a2434)
                };
                nat_add(a2435, a2436)?
            }, v2)?;
            let a2437 = v3;
            let a2438 = 30u64;
            if nat_eq(a2437, a2438) {
                let a2439 = v10;
                let a2440 = 0u64;
                if nat_eq(a2439, a2440) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2441 = v10;
                    let a2442 = 1u64;
                    if nat_eq(a2441, a2442) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2443 = v10;
                        let a2444 = 2u64;
                        if nat_eq(a2443, a2444) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2445 = v10;
                let a2446 = 0u64;
                if nat_eq(a2445, a2446) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2447 = v10;
                    let a2448 = 1u64;
                    if nat_eq(a2447, a2448) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2449 = v0;
                let a2450 = v8;
                nat_sub(a2449, a2450)
            };
            let a2451 = v12;
            let a2452 = 0u64;
            if nat_eq(a2451, a2452) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2453 = v12;
                let a2454 = 1u64;
                if nat_eq(a2453, a2454) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2455 = v12;
                    let a2456 = 2u64;
                    if nat_eq(a2455, a2456) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2457 = v12;
                        let a2458 = 3u64;
                        if nat_eq(a2457, a2458) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2459 = v12;
                            let a2460 = 4u64;
                            if nat_eq(a2459, a2460) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2461 = v12;
                                let a2462 = 5u64;
                                if nat_eq(a2461, a2462) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2463 = v12;
                                    let a2464 = 6u64;
                                    if nat_eq(a2463, a2464) {
                                        if f152(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2465 = 3u64;
                                                let a2466 = v1;
                                                nat_add(a2465, a2466)?
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

pub fn f118(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a2467 = v0;
        let a2468 = f69(v3);
        let a2469 = 0u64;
        nat_quot(a2467, a2468, a2469)
    };
    let v5: u64 = {
        let a2474 = 3u64;
        let a2475 = {
            let a2471 = v4;
            let a2472 = match v1.clone() {
                Adt3::C0(h2470, _, _, _, _, _, _, _) => {
                    h2470
                }
            };
            let a2473 = 0u64;
            nat_quot(a2471, a2472, a2473)
        };
        nat_add(a2474, a2475)?
    };
    let v6: u64 = f94(v4, v1)?;
    let v7: u64 = f94(v4, v2)?;
    let a2476 = v3;
    let a2477 = 30u64;
    if nat_eq(a2476, a2477) {
        let a2481 = {
            let a2478 = v0;
            let a2479 = 3u64;
            let a2480 = 0u64;
            nat_rem(a2478, a2479, a2480)
        };
        let a2482 = 0u64;
        if nat_eq(a2481, a2482) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2486 = {
                let a2483 = v0;
                let a2484 = 3u64;
                let a2485 = 0u64;
                nat_rem(a2483, a2484, a2485)
            };
            let a2487 = 1u64;
            if nat_eq(a2486, a2487) {
                Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt5::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt5::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f119(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a2488 = v0;
        let a2489 = 8388608u64;
        let a2490 = 0u64;
        nat_quot(a2488, a2489, a2490)
    };
    let a2491 = v0;
    let a2492 = 2147483648u64;
    if if if nat_lt(a2491, a2492) {
        let a2493 = 127u64;
        let a2494 = v1;
        nat_le(a2493, a2494)
    } else {
        false
    } {
        let a2495 = v1;
        let a2496 = 190u64;
        nat_lt(a2495, a2496)
    } else {
        false
    } {
        let v2: u64 = {
            let a2500 = 8388608u64;
            let a2501 = {
                let a2497 = v0;
                let a2498 = 8388608u64;
                let a2499 = 0u64;
                nat_rem(a2497, a2498, a2499)
            };
            nat_add(a2500, a2501)?
        };
        let a2502 = v1;
        let a2503 = 150u64;
        if nat_lt(a2502, a2503) {
            let v3: u64 = f153({
                let a2504 = 150u64;
                let a2505 = v1;
                nat_sub(a2504, a2505)
            });
            let a2509 = {
                let a2506 = v2;
                let a2507 = v3;
                let a2508 = 0u64;
                nat_rem(a2506, a2507, a2508)
            };
            let a2510 = 0u64;
            if nat_eq(a2509, a2510) {
                let a2511 = v2;
                let a2512 = v3;
                let a2513 = 0u64;
                Ok(nat_quot(a2511, a2512, a2513))
            } else {
                Ok(0u64)
            }
        } else {
            let a2516 = f153({
                let a2514 = v1;
                let a2515 = 150u64;
                nat_sub(a2514, a2515)
            });
            let a2517 = v2;
            nat_mul(a2516, a2517)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f120(v0: u64, v1: u64, v2: u64) -> bool {
    let a2518 = v0;
    let a2519 = 1u64;
    if nat_le(a2518, a2519) {
        true
    } else {
        let a2523 = {
            let a2520 = v1;
            let a2521 = v2;
            let a2522 = 1u64;
            nat_rem(a2520, a2521, a2522)
        };
        let a2524 = 0u64;
        nat_eq(a2523, a2524)
    }
}

pub fn f121(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2525 = v1;
    let a2526 = 1u64;
    if if if nat_le(a2525, a2526) {
        true
    } else {
        let a2527 = v0;
        let a2528 = v2;
        nat_le(a2527, a2528)
    } {
        let a2529 = v3;
        let a2530 = 1u64;
        if nat_le(a2529, a2530) {
            true
        } else {
            let a2537 = {
                let a2535 = v0;
                let a2536 = {
                    let a2533 = {
                        let a2531 = v1;
                        let a2532 = 1u64;
                        nat_sub(a2531, a2532)
                    };
                    let a2534 = v2;
                    nat_mul(a2533, a2534)?
                };
                nat_add(a2535, a2536)?
            };
            let a2538 = v4;
            nat_le(a2537, a2538)
        }
    } else {
        false
    } {
        let a2539 = v5;
        let a2540 = 1u64;
        if nat_le(a2539, a2540) {
            Ok(true)
        } else {
            let a2553 = {
                let a2551 = {
                    let a2545 = v0;
                    let a2546 = {
                        let a2543 = {
                            let a2541 = v1;
                            let a2542 = 1u64;
                            nat_sub(a2541, a2542)
                        };
                        let a2544 = v2;
                        nat_mul(a2543, a2544)?
                    };
                    nat_add(a2545, a2546)?
                };
                let a2552 = {
                    let a2549 = {
                        let a2547 = v3;
                        let a2548 = 1u64;
                        nat_sub(a2547, a2548)
                    };
                    let a2550 = v4;
                    nat_mul(a2549, a2550)?
                };
                nat_add(a2551, a2552)?
            };
            let a2554 = v6;
            Ok(nat_le(a2553, a2554))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f154(v0, v1, v6, v7, v8) {
        let a2555 = v5;
        let a2556 = 26u64;
        nat_eq(a2555, a2556)
    } else {
        false
    } {
        let a2557 = v3;
        let a2558 = v7;
        nat_eq(a2557, a2558)
    } else {
        false
    } {
        let a2559 = v4;
        let a2560 = v8;
        nat_eq(a2559, a2560)
    } else {
        false
    } {
        let a2561 = v9;
        let a2562 = 1u64;
        nat_eq(a2561, a2562)
    } else {
        false
    } {
        let a2563 = v10;
        let a2564 = f155(v0);
        nat_eq(a2563, a2564)
    } else {
        false
    } {
        let a2565 = v11;
        let a2566 = v1;
        nat_eq(a2565, a2566)
    } else {
        false
    } {
        let a2567 = v12;
        let a2568 = v6;
        nat_eq(a2567, a2568)
    } else {
        false
    } {
        let a2569 = v13;
        let a2570 = v7;
        nat_eq(a2569, a2570)
    } else {
        false
    } {
        let a2571 = v14;
        let a2572 = v8;
        nat_eq(a2571, a2572)
    } else {
        false
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2573 = v0;
    let a2574 = 0u64;
    if if if nat_eq(a2573, a2574) {
        true
    } else {
        let a2575 = v1;
        let a2576 = 0u64;
        nat_eq(a2575, a2576)
    } {
        true
    } else {
        let a2577 = v2;
        let a2578 = 0u64;
        nat_eq(a2577, a2578)
    } {
        true
    } else {
        let a2579 = v3;
        let a2580 = 0u64;
        nat_eq(a2579, a2580)
    }
}

pub fn f124(v0: u64) -> u64 {
    let a2581 = v0;
    let a2582 = 2147483648u64;
    if nat_lt(a2581, a2582) {
        v0
    } else {
        0u64
    }
}

pub fn f125(v0: u64) -> u64 {
    let a2583 = v0;
    let a2584 = 2147483648u64;
    if nat_lt(a2583, a2584) {
        0u64
    } else {
        let a2585 = 4294967296u64;
        let a2586 = v0;
        nat_sub(a2585, a2586)
    }
}

pub fn f126(v0: Adt3, v1: bool) -> R<u64> {
    f156(v0.clone(), {
        let a2588 = match v0.clone() {
            Adt3::C0(h2587, _, _, _, _, _, _, _) => {
                h2587
            }
        };
        let a2589 = 1u64;
        nat_sub(a2588, a2589)
    }, {
        let a2591 = match v0.clone() {
            Adt3::C0(_, h2590, _, _, _, _, _, _) => {
                h2590
            }
        };
        let a2592 = 1u64;
        nat_sub(a2591, a2592)
    }, {
        let a2594 = match v0.clone() {
            Adt3::C0(_, _, h2593, _, _, _, _, _) => {
                h2593
            }
        };
        let a2595 = 1u64;
        nat_sub(a2594, a2595)
    }, {
        let a2597 = match v0 {
            Adt3::C0(_, _, _, h2596, _, _, _, _) => {
                h2596
            }
        };
        let a2598 = 1u64;
        nat_sub(a2597, a2598)
    }, v1)
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2599 = v0;
    let a2600 = 0u64;
    if nat_eq(a2599, a2600) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2601 = v0;
            let a2602 = 1u64;
            nat_sub(a2601, a2602)
        };
        let v5: u64 = {
            let a2603 = v4;
            let a2604 = v1;
            let a2605 = 0u64;
            nat_quot(a2603, a2604, a2605)
        };
        let v6: u64 = {
            let a2613 = {
                let a2606 = v5;
                let a2607 = v2;
                nat_mul(a2606, a2607)?
            };
            let a2614 = {
                let a2611 = {
                    let a2608 = v4;
                    let a2609 = v1;
                    let a2610 = 0u64;
                    nat_rem(a2608, a2609, a2610)
                };
                let a2612 = v3;
                nat_mul(a2611, a2612)?
            };
            nat_add(a2613, a2614)?
        };
        let a2615 = v5;
        let a2616 = 0u64;
        if nat_eq(a2615, a2616) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2625 = {
                    let a2619 = {
                        let a2617 = v5;
                        let a2618 = 1u64;
                        nat_sub(a2617, a2618)
                    };
                    let a2620 = v2;
                    nat_mul(a2619, a2620)?
                };
                let a2626 = {
                    let a2623 = {
                        let a2621 = v1;
                        let a2622 = 1u64;
                        nat_sub(a2621, a2622)
                    };
                    let a2624 = v3;
                    nat_mul(a2623, a2624)?
                };
                nat_add(a2625, a2626)?
            };
            let a2627 = v6;
            let a2628 = v7;
            if nat_lt(a2627, a2628) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f128(v0: Adt3, v1: u64) -> bool {
    let a2629 = v1;
    let a2630 = 0u64;
    if !nat_eq(a2629, a2630) {
        true
    } else {
        let a2632 = match v0 {
            Adt3::C0(_, _, _, _, h2631, _, _, _) => {
                h2631
            }
        };
        let a2633 = 4u64;
        nat_eq(a2632, a2633)
    }
}

pub fn f129(v0: u64, v1: bool) -> R<u64> {
    let a2636 = {
        let a2634 = v0;
        let a2635 = 2u64;
        nat_mul(a2634, a2635)?
    };
    let a2637 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2636, a2637)
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2638 = f157(v0, v1);
    match m2638 {
        None => {
            if v5 {
                let m2639 = f158(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2639 {
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
                let a2640 = 4u64;
                let a2641 = v28;
                nat_le(a2640, a2641)
            };
            let a2642 = v2;
            let a2643 = 0u64;
            if if if if if nat_eq(a2642, a2643) {
                let a2644 = v4;
                let a2645 = 0u64;
                nat_eq(a2644, a2645)
            } else {
                false
            } {
                let a2646 = v6;
                let a2647 = v14;
                if nat_eq(a2646, a2647) {
                    let a2648 = v7;
                    let a2649 = v15;
                    nat_eq(a2648, a2649)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2650 = v8;
                let a2651 = v16;
                if nat_eq(a2650, a2651) {
                    let a2652 = v9;
                    let a2653 = v17;
                    nat_eq(a2652, a2653)
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
                        let a2654 = v3;
                        let a2655 = 0u64;
                        nat_eq(a2654, a2655)
                    } else {
                        false
                    } {
                        f76(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2660 = {
                        let a2656 = v6;
                        let a2657 = v7;
                        nat_mul(a2656, a2657)?
                    };
                    let a2661 = {
                        let a2658 = v8;
                        let a2659 = v9;
                        nat_mul(a2658, a2659)?
                    };
                    nat_mul(a2660, a2661)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f131(v0: u64) -> u64 {
    let m2662 = f75(v0);
    match m2662 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2664 = match v1.clone() {
                Adt11::C0(_, h2663, _) => {
                    h2663
                }
            };
            let a2665 = 1u64;
            if if nat_eq(a2664, a2665) {
                let a2667 = match v1.clone() {
                    Adt11::C0(_, _, h2666) => {
                        h2666
                    }
                };
                let a2668 = 1u64;
                if if if nat_eq(a2667, a2668) {
                    true
                } else {
                    let a2670 = match v1.clone() {
                        Adt11::C0(_, _, h2669) => {
                            h2669
                        }
                    };
                    let a2671 = 2u64;
                    nat_eq(a2670, a2671)
                } {
                    true
                } else {
                    let a2673 = match v1.clone() {
                        Adt11::C0(_, _, h2672) => {
                            h2672
                        }
                    };
                    let a2674 = 4u64;
                    nat_eq(a2673, a2674)
                } {
                    true
                } else {
                    let a2676 = match v1.clone() {
                        Adt11::C0(_, _, h2675) => {
                            h2675
                        }
                    };
                    let a2677 = 8u64;
                    nat_eq(a2676, a2677)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2678) => {
                        h2678
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2691 = {
        let a2687 = {
            let a2683 = {
                let a2679 = v0;
                let a2680 = v4;
                nat_mul(a2679, a2680)?
            };
            let a2684 = {
                let a2681 = v1;
                let a2682 = v5;
                nat_mul(a2681, a2682)?
            };
            nat_add(a2683, a2684)?
        };
        let a2688 = {
            let a2685 = v2;
            let a2686 = v6;
            nat_mul(a2685, a2686)?
        };
        nat_add(a2687, a2688)?
    };
    let a2692 = {
        let a2689 = v3;
        let a2690 = v7;
        nat_mul(a2689, a2690)?
    };
    nat_add(a2691, a2692)
}

pub fn f133(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2697 = {
        let a2695 = {
            let a2693 = v0;
            let a2694 = v1;
            nat_sub(a2693, a2694)
        };
        let a2696 = 1u64;
        nat_sub(a2695, a2696)
    };
    let a2698 = v2;
    nat_add(a2697, a2698)
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f132({
        let a2699 = v0;
        let a2700 = v4;
        let a2701 = 0u64;
        nat_rem(a2699, a2700, a2701)
    }, {
        let a2702 = v1;
        let a2703 = v5;
        let a2704 = 0u64;
        nat_rem(a2702, a2703, a2704)
    }, {
        let a2705 = v2;
        let a2706 = v6;
        let a2707 = 0u64;
        nat_rem(a2705, a2706, a2707)
    }, {
        let a2708 = v3;
        let a2709 = v7;
        let a2710 = 0u64;
        nat_rem(a2708, a2709, a2710)
    }, v8, v9, v10, v11)
}

pub fn f135(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2712 = v0;
        let a2713 = match v1.clone() {
            Adt3::C0(h2711, _, _, _, _, _, _, _) => {
                h2711
            }
        };
        let a2714 = 0u64;
        nat_quot(a2712, a2713, a2714)
    };
    let v4: u64 = {
        let a2716 = v3;
        let a2717 = match v1.clone() {
            Adt3::C0(_, h2715, _, _, _, _, _, _) => {
                h2715
            }
        };
        let a2718 = 0u64;
        nat_quot(a2716, a2717, a2718)
    };
    let a2719 = v2;
    let a2720 = 0u64;
    if nat_eq(a2719, a2720) {
        let a2722 = v0;
        let a2723 = match v1 {
            Adt3::C0(h2721, _, _, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = 0u64;
        nat_rem(a2722, a2723, a2724)
    } else {
        let a2725 = v2;
        let a2726 = 1u64;
        if nat_eq(a2725, a2726) {
            let a2728 = v3;
            let a2729 = match v1 {
                Adt3::C0(_, h2727, _, _, _, _, _, _) => {
                    h2727
                }
            };
            let a2730 = 0u64;
            nat_rem(a2728, a2729, a2730)
        } else {
            let a2731 = v2;
            let a2732 = 2u64;
            if nat_eq(a2731, a2732) {
                let a2734 = v4;
                let a2735 = match v1 {
                    Adt3::C0(_, _, h2733, _, _, _, _, _) => {
                        h2733
                    }
                };
                let a2736 = 0u64;
                nat_rem(a2734, a2735, a2736)
            } else {
                let a2738 = v4;
                let a2739 = match v1 {
                    Adt3::C0(_, _, h2737, _, _, _, _, _) => {
                        h2737
                    }
                };
                let a2740 = 0u64;
                nat_quot(a2738, a2739, a2740)
            }
        }
    }
}

pub fn f136(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f132(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2741, _, _, _) => {
            h2741
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2742, _, _) => {
            h2742
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2743, _) => {
            h2743
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2744) => {
            h2744
        }
    })
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2745 = v1;
    let a2746 = 2147483648u64;
    if nat_lt(a2745, a2746) {
        let a2757 = {
            let a2755 = {
                let a2750 = {
                    let a2747 = v0;
                    let a2748 = v2;
                    let a2749 = 0u64;
                    nat_rem(a2747, a2748, a2749)
                };
                let a2751 = v2;
                nat_add(a2750, a2751)?
            };
            let a2756 = {
                let a2752 = v1;
                let a2753 = v2;
                let a2754 = 0u64;
                nat_rem(a2752, a2753, a2754)
            };
            nat_sub(a2755, a2756)
        };
        let a2758 = v2;
        let a2759 = 0u64;
        Ok(nat_rem(a2757, a2758, a2759))
    } else {
        let a2762 = {
            let a2760 = v0;
            let a2761 = f125(v1);
            nat_add(a2760, a2761)?
        };
        let a2763 = v2;
        let a2764 = 0u64;
        Ok(nat_rem(a2762, a2763, a2764))
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2767 = f124(v1);
    let a2768 = {
        let a2765 = v0;
        let a2766 = f125(v1);
        nat_add(a2765, a2766)?
    };
    if nat_le(a2767, a2768) {
        let a2769 = f139(v0, v1)?;
        let a2770 = v2;
        Ok(nat_lt(a2769, a2770))
    } else {
        Ok(false)
    }
}

pub fn f139(v0: u64, v1: u64) -> R<u64> {
    let a2773 = {
        let a2771 = v0;
        let a2772 = f125(v1);
        nat_add(a2771, a2772)?
    };
    let a2774 = f124(v1);
    Ok(nat_sub(a2773, a2774))
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2775 = v0;
    let a2776 = v1;
    if nat_lt(a2775, a2776) {
        let a2777 = v1;
        let a2778 = v0;
        Ok(nat_sub(a2777, a2778))
    } else {
        let a2781 = v0;
        let a2782 = {
            let a2779 = v1;
            let a2780 = v2;
            nat_add(a2779, a2780)?
        };
        if nat_lt(a2781, a2782) {
            let a2783 = v0;
            let a2784 = v1;
            Ok(nat_sub(a2783, a2784))
        } else {
            let a2791 = {
                let a2787 = 2u64;
                let a2788 = {
                    let a2785 = v2;
                    let a2786 = 1u64;
                    nat_sub(a2785, a2786)
                };
                nat_mul(a2787, a2788)?
            };
            let a2792 = {
                let a2789 = v0;
                let a2790 = v1;
                nat_sub(a2789, a2790)
            };
            Ok(nat_sub(a2791, a2792))
        }
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2795 = {
        let a2793 = v2;
        let a2794 = f156(v1.clone(), f135(v0, v1.clone(), 0u64), f135(v0, v1.clone(), 1u64), f135(v0, v1.clone(), 2u64), f135(v0, v1.clone(), 3u64), false)?;
        nat_add(a2793, a2794)?
    };
    let a2796 = f156(v1.clone(), f135(v0, v1.clone(), 0u64), f135(v0, v1.clone(), 1u64), f135(v0, v1.clone(), 2u64), f135(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2795, a2796))
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> bool {
    let a2797 = v0;
    let a2798 = 0u64;
    if nat_eq(a2797, a2798) {
        let a2799 = v2;
        let a2800 = v1;
        nat_le(a2799, a2800)
    } else {
        let a2801 = v0;
        let a2802 = 1u64;
        if nat_eq(a2801, a2802) {
            let a2803 = v2;
            let a2804 = v1;
            nat_lt(a2803, a2804)
        } else {
            let a2805 = v0;
            let a2806 = 2u64;
            if nat_eq(a2805, a2806) {
                let a2807 = v1;
                let a2808 = v2;
                nat_le(a2807, a2808)
            } else {
                let a2809 = v0;
                let a2810 = 3u64;
                if nat_eq(a2809, a2810) {
                    let a2811 = v1;
                    let a2812 = v2;
                    nat_lt(a2811, a2812)
                } else {
                    let a2813 = v1;
                    let a2814 = v2;
                    nat_eq(a2813, a2814)
                }
            }
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2815 = v0;
    let a2816 = 0u64;
    if nat_eq(a2815, a2816) {
        Ok(f151(0u64))
    } else {
        let a2819 = {
            let a2817 = v0;
            let a2818 = 1u64;
            nat_add(a2817, a2818)?
        };
        let a2820 = f87(v1.clone(), 13u64, v2)?;
        if nat_eq(a2819, a2820) {
            Ok(Adt5::C4({
                let a2821 = v2;
                let a2822 = 0u64;
                if nat_eq(a2821, a2822) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2823 = v0;
                let a2824 = 1u64;
                nat_sub(a2823, a2824)
            };
            if f128(v1.clone(), v2) {
                let v4: u64 = f129(match v1.clone() {
                    Adt3::C0(h2825, _, _, _, _, _, _, _) => {
                        h2825
                    }
                }, false)?;
                let v5: u64 = {
                    let a2826 = v3;
                    let a2827 = v4;
                    let a2828 = 0u64;
                    nat_quot(a2826, a2827, a2828)
                };
                let v6: u64 = {
                    let a2829 = v3;
                    let a2830 = v4;
                    let a2831 = 0u64;
                    nat_rem(a2829, a2830, a2831)
                };
                let a2834 = {
                    let a2832 = v6;
                    let a2833 = 1u64;
                    nat_add(a2832, a2833)?
                };
                let a2835 = v4;
                if nat_eq(a2834, a2835) {
                    Ok(Adt5::C4({
                        let a2836 = v2;
                        let a2837 = 0u64;
                        if nat_eq(a2836, a2837) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f144(v6, v5, v1, v2)
                }
            } else {
                f159({
                    let a2838 = v3;
                    let a2839 = 2u64;
                    let a2840 = 0u64;
                    nat_quot(a2838, a2839, a2840)
                }, {
                    let a2841 = v3;
                    let a2842 = 2u64;
                    let a2843 = 0u64;
                    nat_rem(a2841, a2842, a2843)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f144(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2844 = v0;
    let a2845 = 0u64;
    if nat_eq(a2844, a2845) {
        Ok(f151(1u64))
    } else {
        f159({
            let a2854 = {
                let a2847 = v1;
                let a2848 = match v2.clone() {
                    Adt3::C0(h2846, _, _, _, _, _, _, _) => {
                        h2846
                    }
                };
                nat_mul(a2847, a2848)?
            };
            let a2855 = {
                let a2851 = {
                    let a2849 = v0;
                    let a2850 = 1u64;
                    nat_sub(a2849, a2850)
                };
                let a2852 = 2u64;
                let a2853 = 0u64;
                nat_quot(a2851, a2852, a2853)
            };
            nat_add(a2854, a2855)?
        }, {
            let a2858 = {
                let a2856 = v0;
                let a2857 = 1u64;
                nat_sub(a2856, a2857)
            };
            let a2859 = 2u64;
            let a2860 = 0u64;
            nat_rem(a2858, a2859, a2860)
        }, v2, v3, 1u64)
    }
}

pub fn f145(v0: u64, v1: u64) -> R<u64> {
    let a2861 = v0;
    let a2862 = f64(v1)?;
    let a2863 = 0u64;
    Ok(nat_rem(a2861, a2862, a2863))
}

pub fn f146(v0: u64, v1: u64) -> R<u64> {
    let a2864 = 4u64;
    let a2865 = f160(v0, v1)?;
    nat_mul(a2864, a2865)
}

pub fn f147(v0: u64, v1: u64) -> R<u64> {
    let a2866 = 4u64;
    let a2867 = f161(v0, v1)?;
    nat_mul(a2866, a2867)
}

pub fn f148(v0: u64, v1: u64) -> R<bool> {
    let a2873 = {
        let a2870 = {
            let a2868 = f145(v0, v1)?;
            let a2869 = 2u64;
            nat_sub(a2868, a2869)
        };
        let a2871 = 2u64;
        let a2872 = 0u64;
        nat_rem(a2870, a2871, a2872)
    };
    let a2874 = 0u64;
    Ok(nat_eq(a2873, a2874))
}

pub fn f149(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2875 = f75(v0);
    match m2875 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f162(v0) {
                let a2876 = v1;
                let a2877 = v7;
                nat_lt(a2876, a2877)
            } else {
                false
            } {
                f111(v8, v2)
            } else {
                false
            } {
                let a2878 = v3;
                let a2879 = f163(v4, v5, v6)?;
                nat_lt(a2878, a2879)
            } else {
                false
            } {
                let a2884 = {
                    let a2881 = v7;
                    let a2882 = match v13.clone() {
                        Adt11::C0(_, h2880, _) => {
                            h2880
                        }
                    };
                    let a2883 = 0u64;
                    nat_rem(a2881, a2882, a2883)
                };
                let a2885 = 0u64;
                nat_eq(a2884, a2885)
            } else {
                false
            } {
                Ok(Some({
                    let a2914 = {
                        let a2904 = {
                            let a2894 = {
                                let a2890 = {
                                    let a2887 = v1;
                                    let a2888 = match v13 {
                                        Adt11::C0(_, h2886, _) => {
                                            h2886
                                        }
                                    };
                                    let a2889 = 0u64;
                                    nat_quot(a2887, a2888, a2889)
                                };
                                let a2891 = v9;
                                nat_mul(a2890, a2891)?
                            };
                            let a2895 = {
                                let a2892 = v2;
                                let a2893 = v10;
                                nat_mul(a2892, a2893)?
                            };
                            nat_add(a2894, a2895)?
                        };
                        let a2905 = {
                            let a2902 = {
                                let a2899 = {
                                    let a2896 = v3;
                                    let a2897 = v4;
                                    let a2898 = 0u64;
                                    nat_quot(a2896, a2897, a2898)
                                };
                                let a2900 = v5;
                                let a2901 = 0u64;
                                nat_rem(a2899, a2900, a2901)
                            };
                            let a2903 = v11;
                            nat_mul(a2902, a2903)?
                        };
                        nat_add(a2904, a2905)?
                    };
                    let a2915 = {
                        let a2912 = {
                            let a2909 = {
                                let a2906 = v3;
                                let a2907 = v4;
                                let a2908 = 0u64;
                                nat_quot(a2906, a2907, a2908)
                            };
                            let a2910 = v5;
                            let a2911 = 0u64;
                            nat_quot(a2909, a2910, a2911)
                        };
                        let a2913 = v12;
                        nat_mul(a2912, a2913)?
                    };
                    nat_add(a2914, a2915)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2916 = v0;
    let a2917 = 30u64;
    if nat_eq(a2916, a2917) {
        4u64
    } else {
        3u64
    }
}

pub fn f151(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f152(v0: u64) -> bool {
    let a2918 = 0u64;
    let a2919 = v0;
    if nat_lt(a2918, a2919) {
        let a2920 = v0;
        let a2921 = 2139095040u64;
        nat_le(a2920, a2921)
    } else {
        false
    }
}

pub fn f153(v0: u64) -> u64 {
    let a2922 = v0;
    let a2923 = 40u64;
    if nat_lt(a2922, a2923) {
        let a2924 = v0;
        let a2925 = 20u64;
        if nat_lt(a2924, a2925) {
            let a2926 = v0;
            let a2927 = 10u64;
            if nat_lt(a2926, a2927) {
                let a2928 = v0;
                let a2929 = 5u64;
                if nat_lt(a2928, a2929) {
                    let a2930 = v0;
                    let a2931 = 2u64;
                    if nat_lt(a2930, a2931) {
                        let a2932 = v0;
                        let a2933 = 1u64;
                        if nat_lt(a2932, a2933) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2934 = v0;
                        let a2935 = 3u64;
                        if nat_lt(a2934, a2935) {
                            4u64
                        } else {
                            let a2936 = v0;
                            let a2937 = 4u64;
                            if nat_lt(a2936, a2937) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2938 = v0;
                    let a2939 = 7u64;
                    if nat_lt(a2938, a2939) {
                        let a2940 = v0;
                        let a2941 = 6u64;
                        if nat_lt(a2940, a2941) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2942 = v0;
                        let a2943 = 8u64;
                        if nat_lt(a2942, a2943) {
                            128u64
                        } else {
                            let a2944 = v0;
                            let a2945 = 9u64;
                            if nat_lt(a2944, a2945) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2946 = v0;
                let a2947 = 15u64;
                if nat_lt(a2946, a2947) {
                    let a2948 = v0;
                    let a2949 = 12u64;
                    if nat_lt(a2948, a2949) {
                        let a2950 = v0;
                        let a2951 = 11u64;
                        if nat_lt(a2950, a2951) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2952 = v0;
                        let a2953 = 13u64;
                        if nat_lt(a2952, a2953) {
                            4096u64
                        } else {
                            let a2954 = v0;
                            let a2955 = 14u64;
                            if nat_lt(a2954, a2955) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2956 = v0;
                    let a2957 = 17u64;
                    if nat_lt(a2956, a2957) {
                        let a2958 = v0;
                        let a2959 = 16u64;
                        if nat_lt(a2958, a2959) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2960 = v0;
                        let a2961 = 18u64;
                        if nat_lt(a2960, a2961) {
                            131072u64
                        } else {
                            let a2962 = v0;
                            let a2963 = 19u64;
                            if nat_lt(a2962, a2963) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2964 = v0;
            let a2965 = 30u64;
            if nat_lt(a2964, a2965) {
                let a2966 = v0;
                let a2967 = 25u64;
                if nat_lt(a2966, a2967) {
                    let a2968 = v0;
                    let a2969 = 22u64;
                    if nat_lt(a2968, a2969) {
                        let a2970 = v0;
                        let a2971 = 21u64;
                        if nat_lt(a2970, a2971) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2972 = v0;
                        let a2973 = 23u64;
                        if nat_lt(a2972, a2973) {
                            4194304u64
                        } else {
                            let a2974 = v0;
                            let a2975 = 24u64;
                            if nat_lt(a2974, a2975) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2976 = v0;
                    let a2977 = 27u64;
                    if nat_lt(a2976, a2977) {
                        let a2978 = v0;
                        let a2979 = 26u64;
                        if nat_lt(a2978, a2979) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2980 = v0;
                        let a2981 = 28u64;
                        if nat_lt(a2980, a2981) {
                            134217728u64
                        } else {
                            let a2982 = v0;
                            let a2983 = 29u64;
                            if nat_lt(a2982, a2983) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2984 = v0;
                let a2985 = 35u64;
                if nat_lt(a2984, a2985) {
                    let a2986 = v0;
                    let a2987 = 32u64;
                    if nat_lt(a2986, a2987) {
                        let a2988 = v0;
                        let a2989 = 31u64;
                        if nat_lt(a2988, a2989) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2990 = v0;
                        let a2991 = 33u64;
                        if nat_lt(a2990, a2991) {
                            4294967296u64
                        } else {
                            let a2992 = v0;
                            let a2993 = 34u64;
                            if nat_lt(a2992, a2993) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2994 = v0;
                    let a2995 = 37u64;
                    if nat_lt(a2994, a2995) {
                        let a2996 = v0;
                        let a2997 = 36u64;
                        if nat_lt(a2996, a2997) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2998 = v0;
                        let a2999 = 38u64;
                        if nat_lt(a2998, a2999) {
                            137438953472u64
                        } else {
                            let a3000 = v0;
                            let a3001 = 39u64;
                            if nat_lt(a3000, a3001) {
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

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f123(v1, v2, v3, v4) {
        f164(v0)
    } else {
        f162(v0)
    }
}

pub fn f155(v0: u64) -> u64 {
    let a3002 = v0;
    let a3003 = 26u64;
    if nat_eq(a3002, a3003) {
        26u64
    } else {
        0u64
    }
}

pub fn f156(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3024 = {
        let a3018 = {
            let a3012 = {
                let a3006 = v1;
                let a3007 = if v5 {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3004, _, _, _) => {
                            h3004
                        }
                    })
                } else {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3005, _, _, _) => {
                            h3005
                        }
                    })
                };
                nat_mul(a3006, a3007)?
            };
            let a3013 = {
                let a3010 = v2;
                let a3011 = if v5 {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3008, _, _) => {
                            h3008
                        }
                    })
                } else {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3009, _, _) => {
                            h3009
                        }
                    })
                };
                nat_mul(a3010, a3011)?
            };
            nat_add(a3012, a3013)?
        };
        let a3019 = {
            let a3016 = v3;
            let a3017 = if v5 {
                f125(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3014, _) => {
                        h3014
                    }
                })
            } else {
                f124(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3015, _) => {
                        h3015
                    }
                })
            };
            nat_mul(a3016, a3017)?
        };
        nat_add(a3018, a3019)?
    };
    let a3025 = {
        let a3022 = v4;
        let a3023 = if v5 {
            f125(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3020) => {
                    h3020
                }
            })
        } else {
            f124(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3021) => {
                    h3021
                }
            })
        };
        nat_mul(a3022, a3023)?
    };
    nat_add(a3024, a3025)
}

pub fn f157(v0: u64, v1: u64) -> Option<u64> {
    let a3026 = v0;
    let a3027 = 2u64;
    if nat_eq(a3026, a3027) {
        Some(0u64)
    } else {
        let a3028 = v0;
        let a3029 = 5u64;
        if nat_eq(a3028, a3029) {
            Some(1u64)
        } else {
            let a3030 = v0;
            let a3031 = 6u64;
            if nat_eq(a3030, a3031) {
                Some(2u64)
            } else {
                let a3032 = v0;
                let a3033 = 7u64;
                if nat_eq(a3032, a3033) {
                    Some(3u64)
                } else {
                    let a3034 = v0;
                    let a3035 = 9u64;
                    if nat_eq(a3034, a3035) {
                        Some(4u64)
                    } else {
                        let a3036 = v0;
                        let a3037 = 8u64;
                        if nat_eq(a3036, a3037) {
                            Some(5u64)
                        } else {
                            let a3038 = v0;
                            let a3039 = 89u64;
                            if nat_eq(a3038, a3039) {
                                let a3040 = v1;
                                let a3041 = 2u64;
                                if nat_eq(a3040, a3041) {
                                    Some(6u64)
                                } else {
                                    let a3042 = v1;
                                    let a3043 = 0u64;
                                    if nat_eq(a3042, a3043) {
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

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    if if if f165(v1) {
        f165(v2)
    } else {
        false
    } {
        let a3044 = v1;
        let a3045 = 0u64;
        if if nat_eq(a3044, a3045) {
            true
        } else {
            let a3046 = v2;
            let a3047 = 0u64;
            nat_eq(a3046, a3047)
        } {
            true
        } else {
            let a3048 = v1;
            let a3049 = v2;
            nat_eq(a3048, a3049)
        }
    } else {
        false
    } {
        f166(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f159(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3050 = v1;
    let a3051 = 0u64;
    if nat_eq(a3050, a3051) {
        Ok(Adt5::C4({
            let a3052 = v3;
            let a3053 = 0u64;
            if nat_eq(a3052, a3053) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f94(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3054 = v3;
            let a3055 = 0u64;
            if nat_eq(a3054, a3055) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f160(v0: u64, v1: u64) -> R<u64> {
    let a3056 = v0;
    let a3057 = f64(v1)?;
    let a3058 = 0u64;
    Ok(nat_quot(a3056, a3057, a3058))
}

pub fn f161(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f160(v0, v1)?;
    let v3: u64 = {
        let a3061 = {
            let a3059 = f145(v0, v1)?;
            let a3060 = 2u64;
            nat_sub(a3059, a3060)
        };
        let a3062 = 2u64;
        let a3063 = 0u64;
        nat_quot(a3061, a3062, a3063)
    };
    let v4: u64 = {
        let a3064 = v2;
        let a3065 = v1;
        nat_mul(a3064, a3065)?
    };
    if f148(v0, v1)? {
        let a3066 = v4;
        let a3067 = v3;
        nat_add(a3066, a3067)
    } else {
        let a3079 = {
            let a3075 = v4;
            let a3076 = {
                let a3073 = {
                    let a3068 = v1;
                    let a3069 = 1u64;
                    nat_sub(a3068, a3069)
                };
                let a3074 = {
                    let a3070 = v2;
                    let a3071 = v1;
                    let a3072 = 0u64;
                    nat_rem(a3070, a3071, a3072)
                };
                nat_mul(a3073, a3074)?
            };
            nat_sub(a3075, a3076)
        };
        let a3080 = {
            let a3077 = v3;
            let a3078 = v1;
            nat_mul(a3077, a3078)?
        };
        nat_add(a3079, a3080)
    }
}

pub fn f162(v0: u64) -> bool {
    let a3081 = v0;
    let a3082 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3081, a3082) {
        true
    } else {
        let a3083 = v0;
        let a3084 = 1u64;
        nat_eq(a3083, a3084)
    } {
        true
    } else {
        let a3085 = v0;
        let a3086 = 2u64;
        nat_eq(a3085, a3086)
    } {
        true
    } else {
        let a3087 = v0;
        let a3088 = 3u64;
        nat_eq(a3087, a3088)
    } {
        true
    } else {
        let a3089 = v0;
        let a3090 = 6u64;
        nat_eq(a3089, a3090)
    } {
        true
    } else {
        let a3091 = v0;
        let a3092 = 7u64;
        nat_eq(a3091, a3092)
    } {
        true
    } else {
        let a3093 = v0;
        let a3094 = 8u64;
        nat_eq(a3093, a3094)
    } {
        true
    } else {
        let a3095 = v0;
        let a3096 = 10u64;
        nat_eq(a3095, a3096)
    } {
        true
    } else {
        let a3097 = v0;
        let a3098 = 11u64;
        nat_eq(a3097, a3098)
    } {
        true
    } else {
        let a3099 = v0;
        let a3100 = 12u64;
        nat_eq(a3099, a3100)
    } {
        true
    } else {
        let a3101 = v0;
        let a3102 = 13u64;
        nat_eq(a3101, a3102)
    } {
        true
    } else {
        let a3103 = v0;
        let a3104 = 14u64;
        nat_eq(a3103, a3104)
    } {
        true
    } else {
        let a3105 = v0;
        let a3106 = 16u64;
        nat_eq(a3105, a3106)
    } {
        true
    } else {
        let a3107 = v0;
        let a3108 = 17u64;
        nat_eq(a3107, a3108)
    } {
        true
    } else {
        let a3109 = v0;
        let a3110 = 18u64;
        nat_eq(a3109, a3110)
    } {
        true
    } else {
        let a3111 = v0;
        let a3112 = 19u64;
        nat_eq(a3111, a3112)
    } {
        true
    } else {
        let a3113 = v0;
        let a3114 = 20u64;
        nat_eq(a3113, a3114)
    } {
        true
    } else {
        let a3115 = v0;
        let a3116 = 21u64;
        nat_eq(a3115, a3116)
    } {
        true
    } else {
        let a3117 = v0;
        let a3118 = 22u64;
        nat_eq(a3117, a3118)
    } {
        true
    } else {
        let a3119 = v0;
        let a3120 = 23u64;
        nat_eq(a3119, a3120)
    } {
        true
    } else {
        let a3121 = v0;
        let a3122 = 26u64;
        nat_eq(a3121, a3122)
    } {
        true
    } else {
        let a3123 = v0;
        let a3124 = 29u64;
        nat_eq(a3123, a3124)
    } {
        true
    } else {
        let a3125 = v0;
        let a3126 = 30u64;
        nat_eq(a3125, a3126)
    } {
        true
    } else {
        let a3127 = v0;
        let a3128 = 34u64;
        nat_eq(a3127, a3128)
    } {
        true
    } else {
        let a3129 = v0;
        let a3130 = 35u64;
        nat_eq(a3129, a3130)
    } {
        true
    } else {
        let a3131 = v0;
        let a3132 = 39u64;
        nat_eq(a3131, a3132)
    } {
        true
    } else {
        let a3133 = v0;
        let a3134 = 40u64;
        nat_eq(a3133, a3134)
    } {
        true
    } else {
        let a3135 = v0;
        let a3136 = 41u64;
        nat_eq(a3135, a3136)
    } {
        true
    } else {
        let a3137 = v0;
        let a3138 = 42u64;
        nat_eq(a3137, a3138)
    } {
        true
    } else {
        let a3139 = v0;
        let a3140 = 43u64;
        nat_eq(a3139, a3140)
    }
}

pub fn f163(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3141 = v0;
    let a3142 = 0u64;
    if if if nat_eq(a3141, a3142) {
        true
    } else {
        let a3143 = v1;
        let a3144 = 0u64;
        nat_eq(a3143, a3144)
    } {
        true
    } else {
        let a3145 = v2;
        let a3146 = 0u64;
        nat_eq(a3145, a3146)
    } {
        Ok(0u64)
    } else {
        let a3149 = {
            let a3147 = v0;
            let a3148 = v1;
            nat_mul(a3147, a3148)?
        };
        let a3150 = v2;
        nat_mul(a3149, a3150)
    }
}

pub fn f164(v0: u64) -> bool {
    let m3151 = f75(v0);
    match m3151 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}

pub fn f165(v0: u64) -> bool {
    let a3152 = v0;
    let a3153 = 0u64;
    if if nat_eq(a3152, a3153) {
        true
    } else {
        let a3154 = v0;
        let a3155 = 1u64;
        nat_eq(a3154, a3155)
    } {
        true
    } else {
        let a3156 = v0;
        let a3157 = 30u64;
        nat_eq(a3156, a3157)
    }
}

pub fn f166(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a3158 = v0;
    let a3159 = 28u64;
    if if if if nat_eq(a3158, a3159) {
        let a3160 = v1;
        let a3161 = 0u64;
        nat_eq(a3160, a3161)
    } else {
        false
    } {
        let a3162 = v2;
        let a3163 = 0u64;
        nat_eq(a3162, a3163)
    } else {
        false
    } {
        let a3164 = v3;
        let a3165 = 0u64;
        nat_eq(a3164, a3165)
    } else {
        false
    } {
        let a3166 = v4;
        let a3167 = v8;
        if if if if if if if if nat_eq(a3166, a3167) {
            let a3168 = v5;
            let a3169 = v12;
            nat_eq(a3168, a3169)
        } else {
            false
        } {
            let a3170 = v9;
            let a3171 = v13;
            nat_eq(a3170, a3171)
        } else {
            false
        } {
            let a3172 = v10;
            let a3173 = v14;
            nat_eq(a3172, a3173)
        } else {
            false
        } {
            let a3174 = v11;
            let a3175 = v15;
            nat_eq(a3174, a3175)
        } else {
            false
        } {
            let a3176 = 0u64;
            let a3177 = v6;
            nat_lt(a3176, a3177)
        } else {
            false
        } {
            let a3178 = 0u64;
            let a3179 = v7;
            nat_lt(a3178, a3179)
        } else {
            false
        } {
            let a3183 = {
                let a3180 = v10;
                let a3181 = v6;
                let a3182 = 1u64;
                nat_rem(a3180, a3181, a3182)
            };
            let a3184 = 0u64;
            if nat_eq(a3183, a3184) {
                let a3188 = {
                    let a3185 = v11;
                    let a3186 = v7;
                    let a3187 = 1u64;
                    nat_rem(a3185, a3186, a3187)
                };
                let a3189 = 0u64;
                nat_eq(a3188, a3189)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3190 = v10;
                let a3191 = v6;
                let a3192 = 0u64;
                nat_quot(a3190, a3191, a3192)
            }, {
                let a3193 = v11;
                let a3194 = v7;
                let a3195 = 0u64;
                nat_quot(a3193, a3194, a3195)
            }, {
                let a3196 = v5;
                let a3197 = v4;
                nat_mul(a3196, a3197)?
            }, {
                let a3198 = v4;
                let a3199 = v9;
                nat_mul(a3198, a3199)?
            }, {
                let a3200 = v5;
                let a3201 = v9;
                nat_mul(a3200, a3201)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}
