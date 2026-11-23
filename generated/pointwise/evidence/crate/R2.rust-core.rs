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
    let a1126 = v0;
    let a1127 = 89u64;
    if if nat_eq(a1126, a1127) {
        let a1128 = v1;
        let a1129 = 4u64;
        nat_eq(a1128, a1129)
    } else {
        false
    } {
        Ok(Some(19u64))
    } else {
        let a1130 = 10u64;
        let a1131 = v0;
        if if nat_le(a1130, a1131) {
            let a1132 = v0;
            let a1133 = 12u64;
            nat_le(a1132, a1133)
        } else {
            false
        } {
            Ok(Some({
                let a1134 = v0;
                let a1135 = 3u64;
                nat_add(a1134, a1135)?
            }))
        } else {
            let a1136 = v0;
            let a1137 = 89u64;
            if if nat_eq(a1136, a1137) {
                let a1138 = v1;
                let a1139 = 7u64;
                nat_eq(a1138, a1139)
            } else {
                false
            } {
                Ok(Some(17u64))
            } else {
                let a1140 = v0;
                let a1141 = 89u64;
                if if nat_eq(a1140, a1141) {
                    let a1142 = v1;
                    let a1143 = 10u64;
                    nat_eq(a1142, a1143)
                } else {
                    false
                } {
                    Ok(Some(18u64))
                } else {
                    let a1144 = v0;
                    let a1145 = 89u64;
                    if if nat_eq(a1144, a1145) {
                        let a1146 = v1;
                        let a1147 = 13u64;
                        nat_eq(a1146, a1147)
                    } else {
                        false
                    } {
                        Ok(Some(16u64))
                    } else {
                        let a1148 = v0;
                        let a1149 = 8u64;
                        if nat_eq(a1148, a1149) {
                            Ok(Some(5u64))
                        } else {
                            let a1150 = v0;
                            let a1151 = 9u64;
                            if nat_eq(a1150, a1151) {
                                Ok(Some(6u64))
                            } else {
                                let a1152 = v0;
                                let a1153 = 89u64;
                                if nat_eq(a1152, a1153) {
                                    let a1154 = v1;
                                    let a1155 = 0u64;
                                    if nat_eq(a1154, a1155) {
                                        Ok(Some(0u64))
                                    } else {
                                        let a1156 = v1;
                                        let a1157 = 1u64;
                                        if nat_eq(a1156, a1157) {
                                            Ok(Some(1u64))
                                        } else {
                                            let a1158 = v1;
                                            let a1159 = 2u64;
                                            if nat_eq(a1158, a1159) {
                                                Ok(Some(2u64))
                                            } else {
                                                let a1160 = v1;
                                                let a1161 = 3u64;
                                                if nat_eq(a1160, a1161) {
                                                    Ok(Some(3u64))
                                                } else {
                                                    let a1162 = v1;
                                                    let a1163 = 6u64;
                                                    if nat_eq(a1162, a1163) {
                                                        Ok(Some(4u64))
                                                    } else {
                                                        let a1164 = v1;
                                                        let a1165 = 12u64;
                                                        if nat_eq(a1164, a1165) {
                                                            Ok(Some(7u64))
                                                        } else {
                                                            let a1166 = v1;
                                                            let a1167 = 11u64;
                                                            if nat_eq(a1166, a1167) {
                                                                Ok(Some(8u64))
                                                            } else {
                                                                let a1168 = v1;
                                                                let a1169 = 18u64;
                                                                if nat_eq(a1168, a1169) {
                                                                    Ok(Some(9u64))
                                                                } else {
                                                                    let a1170 = v1;
                                                                    let a1171 = 19u64;
                                                                    if nat_eq(a1170, a1171) {
                                                                        Ok(Some(10u64))
                                                                    } else {
                                                                        let a1172 = v1;
                                                                        let a1173 = 20u64;
                                                                        if nat_eq(a1172, a1173) {
                                                                            Ok(Some(11u64))
                                                                        } else {
                                                                            let a1174 = v1;
                                                                            let a1175 = 21u64;
                                                                            if nat_eq(a1174, a1175) {
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

pub fn f50(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1176 = f88(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1176 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            let a1177 = v0;
            let a1178 = 2u64;
            if if if if nat_eq(a1177, a1178) {
                true
            } else {
                let a1179 = v0;
                let a1180 = 5u64;
                nat_eq(a1179, a1180)
            } {
                true
            } else {
                let a1181 = v0;
                let a1182 = 6u64;
                nat_eq(a1181, a1182)
            } {
                true
            } else {
                let a1183 = v0;
                let a1184 = 7u64;
                nat_eq(a1183, a1184)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1185 = v3;
                    match m1185 {
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
    let a1187 = match v1.clone() {
        Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
            h1186
        }
    };
    let a1188 = 0u64;
    if if if if if nat_eq(a1187, a1188) {
        true
    } else {
        let a1190 = match v1.clone() {
            Adt2::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                h1189
            }
        };
        let a1191 = 1u64;
        nat_eq(a1190, a1191)
    } {
        true
    } else {
        let a1193 = match v1.clone() {
            Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        let a1194 = 30u64;
        nat_eq(a1193, a1194)
    } {
        let a1197 = match v1.clone() {
            Adt2::C0(h1195, _, _, _, _, _, _, _, _, _, _) => {
                h1195
            }
        };
        let a1198 = match v2.clone() {
            Adt2::C0(h1196, _, _, _, _, _, _, _, _, _, _) => {
                h1196
            }
        };
        if nat_eq(a1197, a1198) {
            true
        } else {
            let a1200 = match v2.clone() {
                Adt2::C0(h1199, _, _, _, _, _, _, _, _, _, _) => {
                    h1199
                }
            };
            let a1201 = 0u64;
            nat_eq(a1200, a1201)
        }
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v2.clone(), 1u64)?, {
            let a1203 = match v2.clone() {
                Adt2::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                    h1202
                }
            };
            let a1204 = 0u64;
            if nat_eq(a1203, a1204) {
                4u64
            } else {
                2u64
            }
        }, f14(v1.clone(), 1u64), f14(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1205, _, _, _, _, _, _, _, _, _, _) => {
                h1205
            }
        }, match v2 {
            Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                h1206
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f52(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1207 = v1;
    match m1207 {
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
    let m1208 = v3;
    match m1208 {
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
                Adt3::C0(h1209, _, _, _, _, _, _, _) => {
                    h1209
                }
            }, match v2.clone() {
                Adt3::C0(_, h1210, _, _, _, _, _, _) => {
                    h1210
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
            let a1211 = v51;
            let a1212 = 64u64;
            if if if nat_eq(a1211, a1212) {
                true
            } else {
                let a1213 = v51;
                let a1214 = 65u64;
                nat_eq(a1213, a1214)
            } {
                true
            } else {
                let a1215 = v51;
                let a1216 = 66u64;
                nat_eq(a1215, a1216)
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
        Adt3::C0(h1217, _, _, _, _, _, _, _) => {
            h1217
        }
    })?;
    let v5: u64 = {
        let a1218 = v0;
        let a1219 = v4;
        let a1220 = 0u64;
        nat_quot(a1218, a1219, a1220)
    };
    let v6: u64 = {
        let a1221 = v0;
        let a1222 = v4;
        let a1223 = 0u64;
        nat_rem(a1221, a1222, a1223)
    };
    let a1224 = v6;
    let a1225 = 0u64;
    if nat_eq(a1224, a1225) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1226 = v6;
        let a1227 = 1u64;
        if nat_eq(a1226, a1227) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1230 = {
                let a1228 = v6;
                let a1229 = 1u64;
                nat_add(a1228, a1229)?
            };
            let a1231 = v4;
            if nat_eq(a1230, a1231) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f94(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1232 = v6;
                    let a1233 = 2u64;
                    nat_sub(a1232, a1233)
                };
                let v8: u64 = {
                    let a1234 = v7;
                    let a1235 = 3u64;
                    let a1236 = 0u64;
                    nat_quot(a1234, a1235, a1236)
                };
                let v9: u64 = {
                    let a1237 = v7;
                    let a1238 = 3u64;
                    let a1239 = 0u64;
                    nat_rem(a1237, a1238, a1239)
                };
                let v10: u64 = f94({
                    let a1243 = {
                        let a1241 = v5;
                        let a1242 = match v1.clone() {
                            Adt3::C0(h1240, _, _, _, _, _, _, _) => {
                                h1240
                            }
                        };
                        nat_mul(a1241, a1242)?
                    };
                    let a1244 = v8;
                    nat_add(a1243, a1244)?
                }, v1)?;
                let a1245 = v9;
                let a1246 = 0u64;
                if nat_eq(a1245, a1246) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1247 = v9;
                    let a1248 = 1u64;
                    if nat_eq(a1247, a1248) {
                        let a1249 = v3;
                        let a1250 = 2u64;
                        if nat_eq(a1249, a1250) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1251 = v3;
                        let a1252 = 2u64;
                        if if nat_eq(a1251, a1252) {
                            true
                        } else {
                            let a1253 = v3;
                            let a1254 = 3u64;
                            nat_eq(a1253, a1254)
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
        let a1256 = 2u64;
        let a1257 = match v1.clone() {
            Adt3::C0(_, h1255, _, _, _, _, _, _) => {
                h1255
            }
        };
        nat_mul(a1256, a1257)?
    };
    let a1258 = v0;
    let a1259 = v5;
    if nat_lt(a1258, a1259) {
        let a1263 = {
            let a1260 = v0;
            let a1261 = 2u64;
            let a1262 = 0u64;
            nat_rem(a1260, a1261, a1262)
        };
        let a1264 = 0u64;
        if nat_eq(a1263, a1264) {
            Ok(Adt5::C4(19u64, 4u64, f94({
                let a1265 = v0;
                let a1266 = 2u64;
                let a1267 = 0u64;
                nat_quot(a1265, a1266, a1267)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1268 = v4;
            let a1269 = 2147483648u64;
            if if nat_lt(a1268, a1269) {
                let a1271 = v4;
                let a1272 = match v2 {
                    Adt3::C0(_, h1270, _, _, _, _, _, _) => {
                        h1270
                    }
                };
                nat_lt(a1271, a1272)
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
            let a1275 = match v2.clone() {
                Adt3::C0(h1273, _, _, _, _, _, _, _) => {
                    h1273
                }
            };
            let a1276 = match v2.clone() {
                Adt3::C0(_, h1274, _, _, _, _, _, _) => {
                    h1274
                }
            };
            nat_mul(a1275, a1276)?
        };
        let a1279 = {
            let a1277 = v0;
            let a1278 = v5;
            nat_sub(a1277, a1278)
        };
        let a1280 = v6;
        if nat_lt(a1279, a1280) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f94({
                let a1281 = v0;
                let a1282 = v5;
                nat_sub(a1281, a1282)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1285 = {
                    let a1283 = v0;
                    let a1284 = v5;
                    nat_sub(a1283, a1284)
                };
                let a1286 = v6;
                nat_sub(a1285, a1286)
            };
            let v8: u64 = {
                let a1290 = 1u64;
                let a1291 = {
                    let a1288 = 2u64;
                    let a1289 = match v1.clone() {
                        Adt3::C0(h1287, _, _, _, _, _, _, _) => {
                            h1287
                        }
                    };
                    nat_mul(a1288, a1289)?
                };
                nat_add(a1290, a1291)?
            };
            let v9: u64 = {
                let a1292 = v7;
                let a1293 = v8;
                let a1294 = 0u64;
                nat_quot(a1292, a1293, a1294)
            };
            let v10: u64 = {
                let a1295 = v7;
                let a1296 = v8;
                let a1297 = 0u64;
                nat_rem(a1295, a1296, a1297)
            };
            let a1298 = v10;
            let a1299 = 0u64;
            if nat_eq(a1298, a1299) {
                Ok(Adt5::C4(19u64, 4u64, f94(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1303 = {
                    let a1300 = v10;
                    let a1301 = 2u64;
                    let a1302 = 0u64;
                    nat_rem(a1300, a1301, a1302)
                };
                let a1304 = 1u64;
                if nat_eq(a1303, a1304) {
                    Ok(Adt5::C4(6u64, 0u64, f94({
                        let a1313 = {
                            let a1306 = v9;
                            let a1307 = match v1.clone() {
                                Adt3::C0(h1305, _, _, _, _, _, _, _) => {
                                    h1305
                                }
                            };
                            nat_mul(a1306, a1307)?
                        };
                        let a1314 = {
                            let a1310 = {
                                let a1308 = v10;
                                let a1309 = 1u64;
                                nat_sub(a1308, a1309)
                            };
                            let a1311 = 2u64;
                            let a1312 = 0u64;
                            nat_quot(a1310, a1311, a1312)
                        };
                        nat_add(a1313, a1314)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f94({
                        let a1323 = {
                            let a1316 = v4;
                            let a1317 = match v1.clone() {
                                Adt3::C0(h1315, _, _, _, _, _, _, _) => {
                                    h1315
                                }
                            };
                            nat_mul(a1316, a1317)?
                        };
                        let a1324 = {
                            let a1320 = {
                                let a1318 = v10;
                                let a1319 = 1u64;
                                nat_sub(a1318, a1319)
                            };
                            let a1321 = 2u64;
                            let a1322 = 0u64;
                            nat_quot(a1320, a1321, a1322)
                        };
                        nat_add(a1323, a1324)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f94({
                        let a1333 = {
                            let a1326 = v4;
                            let a1327 = match v1 {
                                Adt3::C0(h1325, _, _, _, _, _, _, _) => {
                                    h1325
                                }
                            };
                            nat_mul(a1326, a1327)?
                        };
                        let a1334 = {
                            let a1330 = {
                                let a1328 = v10;
                                let a1329 = 1u64;
                                nat_sub(a1328, a1329)
                            };
                            let a1331 = 2u64;
                            let a1332 = 0u64;
                            nat_quot(a1330, a1331, a1332)
                        };
                        nat_add(a1333, a1334)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f56(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1335 = v1;
    let a1336 = f109(v2.clone(), v4.clone())?;
    if nat_lt(a1335, a1336) {
        let a1339 = v1;
        let a1340 = {
            let a1337 = 2u64;
            let a1338 = f86(v4.clone())?;
            nat_mul(a1337, a1338)?
        };
        if nat_lt(a1339, a1340) {
            let a1344 = {
                let a1341 = v1;
                let a1342 = 2u64;
                let a1343 = 0u64;
                nat_rem(a1341, a1342, a1343)
            };
            let a1345 = 0u64;
            if nat_eq(a1344, a1345) {
                f110({
                    let a1346 = v1;
                    let a1347 = 2u64;
                    let a1348 = 0u64;
                    nat_quot(a1346, a1347, a1348)
                }, v4)
            } else {
                if f111(match v2 {
                    Adt3::C0(_, h1349, _, _, _, _, _, _) => {
                        h1349
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1352 = v1;
                let a1353 = {
                    let a1350 = 2u64;
                    let a1351 = f86(v4.clone())?;
                    nat_mul(a1350, a1351)?
                };
                nat_sub(a1352, a1353)
            };
            let v7: u64 = {
                let a1357 = v6;
                let a1358 = {
                    let a1355 = 1u64;
                    let a1356 = match v2.clone() {
                        Adt3::C0(h1354, _, _, _, _, _, _, _) => {
                            h1354
                        }
                    };
                    nat_add(a1355, a1356)?
                };
                let a1359 = 0u64;
                nat_quot(a1357, a1358, a1359)
            };
            let v8: u64 = {
                let a1363 = v6;
                let a1364 = {
                    let a1361 = 1u64;
                    let a1362 = match v2.clone() {
                        Adt3::C0(h1360, _, _, _, _, _, _, _) => {
                            h1360
                        }
                    };
                    nat_add(a1361, a1362)?
                };
                let a1365 = 0u64;
                nat_rem(a1363, a1364, a1365)
            };
            let a1366 = v8;
            let a1367 = 0u64;
            if nat_eq(a1366, a1367) {
                f110(v7, v4)
            } else {
                f112(v0, v7, {
                    let a1368 = v8;
                    let a1369 = 1u64;
                    nat_sub(a1368, a1369)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1372 = match v4.clone() {
            Adt3::C0(h1370, _, _, _, _, _, _, _) => {
                h1370
            }
        };
        let a1373 = match v4.clone() {
            Adt3::C0(_, h1371, _, _, _, _, _, _) => {
                h1371
            }
        };
        nat_mul(a1372, a1373)?
    };
    let v7: u64 = {
        let a1374 = 2u64;
        let a1375 = v6;
        nat_mul(a1374, a1375)?
    };
    let v8: u64 = f113(match v3.clone() {
        Adt3::C0(_, h1376, _, _, _, _, _, _) => {
            h1376
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1377, _, _, _, _, _) => {
            h1377
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1378, _, _, _, _) => {
            h1378
        }
    });
    let a1379 = v0;
    let a1380 = v7;
    if nat_lt(a1379, a1380) {
        let a1384 = {
            let a1381 = v0;
            let a1382 = 2u64;
            let a1383 = 0u64;
            nat_rem(a1381, a1382, a1383)
        };
        let a1385 = 0u64;
        if nat_eq(a1384, a1385) {
            Ok(Adt5::C4(19u64, 5u64, f94({
                let a1386 = v0;
                let a1387 = 2u64;
                let a1388 = 0u64;
                nat_quot(a1386, a1387, a1388)
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
            let a1389 = v0;
            let a1390 = v7;
            nat_sub(a1389, a1390)
        };
        let v10: u64 = {
            let a1394 = v9;
            let a1395 = {
                let a1392 = 1u64;
                let a1393 = match v1.clone() {
                    Adt3::C0(h1391, _, _, _, _, _, _, _) => {
                        h1391
                    }
                };
                nat_add(a1392, a1393)?
            };
            let a1396 = 0u64;
            nat_quot(a1394, a1395, a1396)
        };
        let v11: u64 = {
            let a1400 = v9;
            let a1401 = {
                let a1398 = 1u64;
                let a1399 = match v1.clone() {
                    Adt3::C0(h1397, _, _, _, _, _, _, _) => {
                        h1397
                    }
                };
                nat_add(a1398, a1399)?
            };
            let a1402 = 0u64;
            nat_rem(a1400, a1401, a1402)
        };
        let a1403 = v11;
        let a1404 = 0u64;
        if nat_eq(a1403, a1404) {
            Ok(Adt5::C4(19u64, 5u64, f94({
                let a1405 = v10;
                let a1406 = v6;
                let a1407 = 0u64;
                nat_rem(a1405, a1406, a1407)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f114(v5, v8) {
                let v12: u64 = {
                    let a1413 = {
                        let a1409 = v10;
                        let a1410 = match v1.clone() {
                            Adt3::C0(h1408, _, _, _, _, _, _, _) => {
                                h1408
                            }
                        };
                        nat_mul(a1409, a1410)?
                    };
                    let a1414 = {
                        let a1411 = v11;
                        let a1412 = 1u64;
                        nat_sub(a1411, a1412)
                    };
                    nat_add(a1413, a1414)?
                };
                let a1415 = v5;
                let a1416 = 4294967295u64;
                if nat_eq(a1415, a1416) {
                    Ok(Adt5::C1(false, f94(v12, v1)?, f94(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f94(v12, v1)?, 4u64, f94({
                        let a1422 = {
                            let a1418 = v5;
                            let a1419 = match v3.clone() {
                                Adt3::C0(h1417, _, _, _, _, _, _, _) => {
                                    h1417
                                }
                            };
                            nat_mul(a1418, a1419)?
                        };
                        let a1423 = {
                            let a1420 = v11;
                            let a1421 = 1u64;
                            nat_sub(a1420, a1421)
                        };
                        nat_add(a1422, a1423)?
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
        let a1426 = v6;
        let a1427 = {
            let a1424 = 3u64;
            let a1425 = v5;
            nat_add(a1424, a1425)?
        };
        nat_sub(a1426, a1427)
    };
    let a1429 = v0;
    let a1430 = f26(v3, match v1.clone() {
        Adt3::C0(h1428, _, _, _, _, _, _, _) => {
            h1428
        }
    }, v5, v8)?;
    if nat_lt(a1429, a1430) {
        let v9: u64 = f67(match v1.clone() {
            Adt3::C0(h1431, _, _, _, _, _, _, _) => {
                h1431
            }
        }, v5, v8)?;
        let v10: u64 = f68(v3, match v1.clone() {
            Adt3::C0(h1432, _, _, _, _, _, _, _) => {
                h1432
            }
        }, v5)?;
        let a1433 = v0;
        let a1434 = v9;
        if nat_lt(a1433, a1434) {
            f115(v0, v2, f20(v3), {
                let a1435 = 3u64;
                let a1436 = v5;
                nat_add(a1435, a1436)?
            }, v7)
        } else {
            let a1439 = v0;
            let a1440 = {
                let a1437 = v9;
                let a1438 = v10;
                nat_add(a1437, a1438)?
            };
            if nat_lt(a1439, a1440) {
                let v11: u64 = {
                    let a1441 = v0;
                    let a1442 = v9;
                    nat_sub(a1441, a1442)
                };
                let v12: u64 = f116(v3, match v1.clone() {
                    Adt3::C0(h1443, _, _, _, _, _, _, _) => {
                        h1443
                    }
                })?;
                f117({
                    let a1444 = v11;
                    let a1445 = v12;
                    let a1446 = 0u64;
                    nat_rem(a1444, a1445, a1446)
                }, {
                    let a1447 = v11;
                    let a1448 = v12;
                    let a1449 = 0u64;
                    nat_quot(a1447, a1448, a1449)
                }, v1, v3, v4, v7)
            } else {
                f118({
                    let a1452 = v0;
                    let a1453 = {
                        let a1450 = v9;
                        let a1451 = v10;
                        nat_add(a1450, a1451)?
                    };
                    nat_sub(a1452, a1453)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1454 = v0;
    let a1455 = f13(v1)?;
    if nat_lt(a1454, a1455) {
        let a1456 = v0;
        let a1457 = 4u64;
        if nat_lt(a1456, a1457) {
            let a1458 = v0;
            let a1459 = 0u64;
            if nat_eq(a1458, a1459) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1460 = v0;
                let a1461 = 1u64;
                if nat_eq(a1460, a1461) {
                    let a1462 = v5;
                    let a1463 = 0u64;
                    if nat_eq(a1462, a1463) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1464 = v0;
                    let a1465 = 2u64;
                    if nat_eq(a1464, a1465) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1466 = v0;
            let a1467 = 4u64;
            if if nat_eq(a1466, a1467) {
                let a1468 = f119(v5)?;
                let a1469 = v1;
                !nat_eq(a1468, a1469)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1472 = {
                        let a1470 = v0;
                        let a1471 = 4u64;
                        nat_sub(a1470, a1471)
                    };
                    let a1473 = 3u64;
                    let a1474 = 0u64;
                    nat_quot(a1472, a1473, a1474)
                };
                let v7: u64 = {
                    let a1477 = {
                        let a1475 = v0;
                        let a1476 = 4u64;
                        nat_sub(a1475, a1476)
                    };
                    let a1478 = 3u64;
                    let a1479 = 0u64;
                    nat_rem(a1477, a1478, a1479)
                };
                let a1480 = v7;
                let a1481 = 0u64;
                if nat_eq(a1480, a1481) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1482 = v7;
                    let a1483 = 1u64;
                    if nat_eq(a1482, a1483) {
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
    let a1484 = v1;
    let a1485 = 0u64;
    if nat_eq(a1484, a1485) {
        if v0 {
            Adt7::C2(v3, v4, false)
        } else {
            Adt7::C0(v2)
        }
    } else {
        let a1486 = v1;
        let a1487 = 1u64;
        if nat_eq(a1486, a1487) {
            Adt7::C1(v2)
        } else {
            let a1488 = v1;
            let a1489 = 2u64;
            if nat_eq(a1488, a1489) {
                Adt7::C5(v2)
            } else {
                let a1490 = v1;
                let a1491 = 3u64;
                if nat_eq(a1490, a1491) {
                    Adt7::C3(v2, v2, false)
                } else {
                    let a1492 = v1;
                    let a1493 = 4u64;
                    if nat_eq(a1492, a1493) {
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
    let a1494 = v0;
    let a1495 = 0u64;
    if nat_eq(a1494, a1495) {
        Adt8::C0(0u64, v1)
    } else {
        let a1496 = v0;
        let a1497 = 1u64;
        if nat_eq(a1496, a1497) {
            Adt8::C1(v1)
        } else {
            let a1498 = v0;
            let a1499 = 2u64;
            if nat_eq(a1498, a1499) {
                Adt8::C2(v1)
            } else {
                let a1500 = v0;
                let a1501 = 3u64;
                if nat_eq(a1500, a1501) {
                    Adt8::C3(v1)
                } else {
                    let a1502 = v0;
                    let a1503 = 13u64;
                    if nat_lt(a1502, a1503) {
                        Adt8::C0({
                            let a1504 = v0;
                            let a1505 = 3u64;
                            nat_sub(a1504, a1505)
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
    let a1506 = v0;
    let a1507 = 1u64;
    if nat_eq(a1506, a1507) {
        Adt9::C0(v1)
    } else {
        let a1508 = v0;
        let a1509 = 3u64;
        if if nat_eq(a1508, a1509) {
            true
        } else {
            let a1510 = v0;
            let a1511 = 6u64;
            nat_eq(a1510, a1511)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f63(v0: u64) -> u64 {
    let v1: u64 = {
        let a1512 = v0;
        let a1513 = 4294967296u64;
        let a1514 = 0u64;
        nat_rem(a1512, a1513, a1514)
    };
    let a1515 = v1;
    let a1516 = 2147483648u64;
    if nat_lt(a1515, a1516) {
        v1
    } else {
        0u64
    }
}

pub fn f64(v0: u64) -> R<u64> {
    let a1519 = 2u64;
    let a1520 = {
        let a1517 = 2u64;
        let a1518 = v0;
        nat_mul(a1517, a1518)?
    };
    nat_add(a1519, a1520)
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
    let a1521 = v2;
    let a1522 = 0u64;
    if nat_eq(a1521, a1522) {
        Ok(0u64)
    } else {
        let a1525 = 2u64;
        let a1526 = {
            let a1523 = v0;
            let a1524 = v1;
            nat_mul(a1523, a1524)?
        };
        nat_mul(a1525, a1526)
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1527 = v2;
    let a1528 = f116(v0, v1)?;
    nat_mul(a1527, a1528)
}

pub fn f69(v0: u64) -> u64 {
    let a1529 = v0;
    let a1530 = 30u64;
    if nat_eq(a1529, a1530) {
        3u64
    } else {
        1u64
    }
}

pub fn f70(v0: u64) -> R<u64> {
    let a1533 = 5u64;
    let a1534 = {
        let a1531 = 4u64;
        let a1532 = v0;
        nat_mul(a1531, a1532)?
    };
    nat_add(a1533, a1534)
}

pub fn f71(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f122(match v0.clone() {
        Adt2::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
            h1535
        }
    }, match v0.clone() {
        Adt2::C0(_, h1536, _, _, _, _, _, _, _, _, _) => {
            h1536
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1537, _, _, _, _, _, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1538, _, _, _, _, _, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1539, _, _, _, _, _, _) => {
            h1539
        }
    }, match v1.clone() {
        Adt2::C0(h1540, _, _, _, _, _, _, _, _, _, _) => {
            h1540
        }
    }, match v1.clone() {
        Adt2::C0(_, h1541, _, _, _, _, _, _, _, _, _) => {
            h1541
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1542, _, _, _, _, _, _, _, _) => {
            h1542
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1543, _, _, _, _, _, _, _) => {
            h1543
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1544, _, _, _, _, _, _) => {
            h1544
        }
    }, match v2.clone() {
        Adt2::C0(h1545, _, _, _, _, _, _, _, _, _, _) => {
            h1545
        }
    }, match v2.clone() {
        Adt2::C0(_, h1546, _, _, _, _, _, _, _, _, _) => {
            h1546
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1547, _, _, _, _, _, _, _, _) => {
            h1547
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1548, _, _, _, _, _, _, _) => {
            h1548
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1549, _, _, _, _, _, _) => {
            h1549
        }
    }) {
        let m1551 = f75(match v0.clone() {
            Adt2::C0(h1550, _, _, _, _, _, _, _, _, _, _) => {
                h1550
            }
        });
        match m1551 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1557 = {
                    let a1554 = match v0.clone() {
                        Adt2::C0(_, h1552, _, _, _, _, _, _, _, _, _) => {
                            h1552
                        }
                    };
                    let a1555 = match v3.clone() {
                        Adt11::C0(_, h1553, _) => {
                            h1553
                        }
                    };
                    let a1556 = 0u64;
                    nat_rem(a1554, a1555, a1556)
                };
                let a1558 = 0u64;
                if if nat_eq(a1557, a1558) {
                    if f123(match v0.clone() {
                        Adt2::C0(_, h1559, _, _, _, _, _, _, _, _, _) => {
                            h1559
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1560, _, _, _, _, _, _, _, _, _) => {
                            h1560
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1561, _, _, _, _, _, _, _, _) => {
                            h1561
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1562, _, _, _, _, _, _, _) => {
                            h1562
                        }
                    }) {
                        true
                    } else {
                        let a1564 = 4u64;
                        let a1565 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1563, _, _, _, _, _) => {
                                h1563
                            }
                        };
                        if if nat_le(a1564, a1565) {
                            let a1567 = match v3.clone() {
                                Adt11::C0(_, h1566, _) => {
                                    h1566
                                }
                            };
                            let a1568 = 1u64;
                            if nat_eq(a1567, a1568) {
                                true
                            } else {
                                let a1571 = match v3.clone() {
                                    Adt11::C0(_, _, h1569) => {
                                        h1569
                                    }
                                };
                                let a1572 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1570, _, _, _, _, _) => {
                                        h1570
                                    }
                                };
                                nat_le(a1571, a1572)
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
                        Adt11::C0(_, h1573, _) => {
                            h1573
                        }
                    });
                    Ok(Adt6::C4(if f123(match v0.clone() {
                        Adt2::C0(_, h1574, _, _, _, _, _, _, _, _, _) => {
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
                    }) {
                        0u64
                    } else {
                        f109(v4.clone(), f14(v1.clone(), 1u64))?
                    }, 4u64, v4, f14(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1578, _, _, _, _, _, _, _, _, _, _) => {
                            h1578
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
        Adt2::C0(h1579, _, _, _, _, _, _, _, _, _, _) => {
            h1579
        }
    }) {
        let a1581 = match v1.clone() {
            Adt2::C0(h1580, _, _, _, _, _, _, _, _, _, _) => {
                h1580
            }
        };
        let a1582 = 26u64;
        nat_eq(a1581, a1582)
    } else {
        false
    } {
        let a1584 = match v2.clone() {
            Adt2::C0(h1583, _, _, _, _, _, _, _, _, _, _) => {
                h1583
            }
        };
        let a1585 = 0u64;
        nat_eq(a1584, a1585)
    } else {
        false
    } {
        let a1588 = match v0.clone() {
            Adt2::C0(_, h1586, _, _, _, _, _, _, _, _, _) => {
                h1586
            }
        };
        let a1589 = match v2.clone() {
            Adt2::C0(_, h1587, _, _, _, _, _, _, _, _, _) => {
                h1587
            }
        };
        nat_eq(a1588, a1589)
    } else {
        false
    } {
        let a1592 = match v1.clone() {
            Adt2::C0(_, h1590, _, _, _, _, _, _, _, _, _) => {
                h1590
            }
        };
        let a1593 = match v0.clone() {
            Adt2::C0(_, _, h1591, _, _, _, _, _, _, _, _) => {
                h1591
            }
        };
        nat_le(a1592, a1593)
    } else {
        false
    } {
        let a1595 = match v0.clone() {
            Adt2::C0(_, _, _, h1594, _, _, _, _, _, _, _) => {
                h1594
            }
        };
        let a1596 = 1u64;
        nat_eq(a1595, a1596)
    } else {
        false
    } {
        let a1598 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1597, _, _, _, _, _, _) => {
                h1597
            }
        };
        let a1599 = 1u64;
        nat_eq(a1598, a1599)
    } else {
        false
    } {
        let a1601 = match v2.clone() {
            Adt2::C0(_, _, _, h1600, _, _, _, _, _, _, _) => {
                h1600
            }
        };
        let a1602 = 1u64;
        nat_eq(a1601, a1602)
    } else {
        false
    } {
        let a1604 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1603, _, _, _, _, _, _) => {
                h1603
            }
        };
        let a1605 = 1u64;
        nat_eq(a1604, a1605)
    } else {
        false
    } {
        let a1607 = match v1.clone() {
            Adt2::C0(_, _, h1606, _, _, _, _, _, _, _, _) => {
                h1606
            }
        };
        let a1608 = 1u64;
        nat_eq(a1607, a1608)
    } else {
        false
    } {
        let a1610 = match v1.clone() {
            Adt2::C0(_, _, _, h1609, _, _, _, _, _, _, _) => {
                h1609
            }
        };
        let a1611 = 1u64;
        nat_eq(a1610, a1611)
    } else {
        false
    } {
        let a1613 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1612, _, _, _, _, _, _) => {
                h1612
            }
        };
        let a1614 = 1u64;
        nat_eq(a1613, a1614)
    } else {
        false
    } {
        if f24(match v2.clone() {
            Adt2::C0(_, h1615, _, _, _, _, _, _, _, _, _) => {
                h1615
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h1616, _, _, _, _, _, _, _, _) => {
                h1616
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h1617, _, _, _, _, _, _, _) => {
                h1617
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h1618, _, _, _, _, _, _) => {
                h1618
            }
        }) {
            Ok(Adt6::C4(0u64, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a1620 = match v1.clone() {
                Adt2::C0(_, h1619, _, _, _, _, _, _, _, _, _) => {
                    h1619
                }
            };
            let a1621 = 0u64;
            if nat_eq(a1620, a1621) {
                Ok(Adt6::C4(f10(v2.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a1623 = match v0.clone() {
                    Adt2::C0(_, h1622, _, _, _, _, _, _, _, _, _) => {
                        h1622
                    }
                };
                let a1624 = 2147483647u64;
                if if if if if if nat_le(a1623, a1624) {
                    let a1626 = match v1.clone() {
                        Adt2::C0(_, h1625, _, _, _, _, _, _, _, _, _) => {
                            h1625
                        }
                    };
                    let a1627 = 2147483647u64;
                    nat_le(a1626, a1627)
                } else {
                    false
                } {
                    let a1629 = 4u64;
                    let a1630 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h1628, _, _, _, _, _) => {
                            h1628
                        }
                    };
                    nat_le(a1629, a1630)
                } else {
                    false
                } {
                    let a1636 = {
                        let a1633 = match v2.clone() {
                            Adt2::C0(_, h1631, _, _, _, _, _, _, _, _, _) => {
                                h1631
                            }
                        };
                        let a1634 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1632, _, _, _, _, _) => {
                                h1632
                            }
                        };
                        nat_mul(a1633, a1634)?
                    };
                    let a1637 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1635, _, _, _, _) => {
                            h1635
                        }
                    };
                    nat_le(a1636, a1637)
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
                        let a1651 = {
                            let a1641 = {
                                let a1639 = 2u64;
                                let a1640 = match v1.clone() {
                                    Adt2::C0(_, h1638, _, _, _, _, _, _, _, _, _) => {
                                        h1638
                                    }
                                };
                                nat_mul(a1639, a1640)?
                            };
                            let a1642 = f10(v2.clone(), 1u64)?;
                            nat_add(a1641, a1642)?
                        };
                        let a1652 = {
                            let a1649 = match v1.clone() {
                                Adt2::C0(_, h1643, _, _, _, _, _, _, _, _, _) => {
                                    h1643
                                }
                            };
                            let a1650 = {
                                let a1647 = 1u64;
                                let a1648 = {
                                    let a1645 = 2u64;
                                    let a1646 = match v0.clone() {
                                        Adt2::C0(_, h1644, _, _, _, _, _, _, _, _, _) => {
                                            h1644
                                        }
                                    };
                                    nat_mul(a1645, a1646)?
                                };
                                nat_add(a1647, a1648)?
                            };
                            nat_mul(a1649, a1650)?
                        };
                        nat_add(a1651, a1652)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h1653, _, _, _, _, _, _, _, _, _) => {
                            h1653
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1654, _, _, _, _, _, _, _, _, _) => {
                            h1654
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h1655, _, _, _, _, _) => {
                            h1655
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1656, _, _, _, _) => {
                            h1656
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h1657, _, _, _) => {
                            h1657
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1658, _, _) => {
                            h1658
                        }
                    }), f14(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h1659, _, _, _, _, _, _, _, _, _, _) => {
                            h1659
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
    let a1662 = match v1.clone() {
        Adt2::C0(h1660, _, _, _, _, _, _, _, _, _, _) => {
            h1660
        }
    };
    let a1663 = match v3.clone() {
        Adt2::C0(h1661, _, _, _, _, _, _, _, _, _, _) => {
            h1661
        }
    };
    if if nat_eq(a1662, a1663) {
        let a1664 = v0;
        let a1665 = 33u64;
        if nat_eq(a1664, a1665) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1667 = f75(match v1.clone() {
            Adt2::C0(h1666, _, _, _, _, _, _, _, _, _, _) => {
                h1666
            }
        });
        match m1667 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1668, _) => {
                        h1668
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a1673 = {
                    let a1670 = match v1.clone() {
                        Adt2::C0(_, h1669, _, _, _, _, _, _, _, _, _) => {
                            h1669
                        }
                    };
                    let a1671 = v5;
                    let a1672 = 1u64;
                    nat_rem(a1670, a1671, a1672)
                };
                let a1674 = 0u64;
                if if if nat_eq(a1673, a1674) {
                    let a1679 = {
                        let a1676 = match v3.clone() {
                            Adt2::C0(_, h1675, _, _, _, _, _, _, _, _, _) => {
                                h1675
                            }
                        };
                        let a1677 = v5;
                        let a1678 = 1u64;
                        nat_rem(a1676, a1677, a1678)
                    };
                    let a1680 = 0u64;
                    nat_eq(a1679, a1680)
                } else {
                    false
                } {
                    let a1681 = v6;
                    let a1682 = f10(v3.clone(), v5)?;
                    nat_eq(a1681, a1682)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1683) => {
                            h1683
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
    let a1684 = v0;
    let a1685 = 0u64;
    if if nat_eq(a1684, a1685) {
        true
    } else {
        let a1686 = v0;
        let a1687 = 1u64;
        nat_eq(a1686, a1687)
    } {
        true
    } else {
        let a1688 = v0;
        let a1689 = 30u64;
        nat_eq(a1688, a1689)
    }
}

pub fn f75(v0: u64) -> Adt10 {
    let a1690 = v0;
    let a1691 = 0u64;
    if nat_eq(a1690, a1691) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1692 = v0;
        let a1693 = 1u64;
        if nat_eq(a1692, a1693) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1694 = v0;
            let a1695 = 2u64;
            if nat_eq(a1694, a1695) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1696 = v0;
                let a1697 = 3u64;
                if nat_eq(a1696, a1697) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1698 = v0;
                    let a1699 = 6u64;
                    if nat_eq(a1698, a1699) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1700 = v0;
                        let a1701 = 7u64;
                        if nat_eq(a1700, a1701) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1702 = v0;
                            let a1703 = 8u64;
                            if nat_eq(a1702, a1703) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1704 = v0;
                                let a1705 = 9u64;
                                if nat_eq(a1704, a1705) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1706 = v0;
                                    let a1707 = 10u64;
                                    if nat_eq(a1706, a1707) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1708 = v0;
                                        let a1709 = 11u64;
                                        if nat_eq(a1708, a1709) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1710 = v0;
                                            let a1711 = 12u64;
                                            if nat_eq(a1710, a1711) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1712 = v0;
                                                let a1713 = 13u64;
                                                if nat_eq(a1712, a1713) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1714 = v0;
                                                    let a1715 = 14u64;
                                                    if nat_eq(a1714, a1715) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1716 = v0;
                                                        let a1717 = 15u64;
                                                        if nat_eq(a1716, a1717) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1718 = v0;
                                                            let a1719 = 16u64;
                                                            if nat_eq(a1718, a1719) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1720 = v0;
                                                                let a1721 = 17u64;
                                                                if nat_eq(a1720, a1721) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1722 = v0;
                                                                    let a1723 = 18u64;
                                                                    if nat_eq(a1722, a1723) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1724 = v0;
                                                                        let a1725 = 19u64;
                                                                        if nat_eq(a1724, a1725) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1726 = v0;
                                                                            let a1727 = 20u64;
                                                                            if nat_eq(a1726, a1727) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1728 = v0;
                                                                                let a1729 = 21u64;
                                                                                if nat_eq(a1728, a1729) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1730 = v0;
                                                                                    let a1731 = 22u64;
                                                                                    if nat_eq(a1730, a1731) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1732 = v0;
                                                                                        let a1733 = 23u64;
                                                                                        if nat_eq(a1732, a1733) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1734 = v0;
                                                                                            let a1735 = 24u64;
                                                                                            if nat_eq(a1734, a1735) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1736 = v0;
                                                                                                let a1737 = 25u64;
                                                                                                if nat_eq(a1736, a1737) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1738 = v0;
                                                                                                    let a1739 = 26u64;
                                                                                                    if nat_eq(a1738, a1739) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1740 = v0;
                                                                                                        let a1741 = 27u64;
                                                                                                        if nat_eq(a1740, a1741) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1742 = v0;
                                                                                                            let a1743 = 28u64;
                                                                                                            if nat_eq(a1742, a1743) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1744 = v0;
                                                                                                                let a1745 = 29u64;
                                                                                                                if nat_eq(a1744, a1745) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1746 = v0;
                                                                                                                    let a1747 = 30u64;
                                                                                                                    if nat_eq(a1746, a1747) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1748 = v0;
                                                                                                                        let a1749 = 34u64;
                                                                                                                        if nat_eq(a1748, a1749) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1750 = v0;
                                                                                                                            let a1751 = 35u64;
                                                                                                                            if nat_eq(a1750, a1751) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1752 = v0;
                                                                                                                                let a1753 = 39u64;
                                                                                                                                if nat_eq(a1752, a1753) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1754 = v0;
                                                                                                                                    let a1755 = 40u64;
                                                                                                                                    if nat_eq(a1754, a1755) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1756 = v0;
                                                                                                                                        let a1757 = 41u64;
                                                                                                                                        if nat_eq(a1756, a1757) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1758 = v0;
                                                                                                                                            let a1759 = 42u64;
                                                                                                                                            if nat_eq(a1758, a1759) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1760 = v0;
                                                                                                                                                let a1761 = 43u64;
                                                                                                                                                if nat_eq(a1760, a1761) {
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
        let a1765 = {
            let a1762 = v0;
            let a1763 = v4;
            let a1764 = 1u64;
            nat_rem(a1762, a1763, a1764)
        };
        let a1766 = 0u64;
        if if if nat_eq(a1765, a1766) {
            let a1770 = {
                let a1767 = v1;
                let a1768 = v5;
                let a1769 = 1u64;
                nat_rem(a1767, a1768, a1769)
            };
            let a1771 = 0u64;
            nat_eq(a1770, a1771)
        } else {
            false
        } {
            let a1775 = {
                let a1772 = v2;
                let a1773 = v6;
                let a1774 = 1u64;
                nat_rem(a1772, a1773, a1774)
            };
            let a1776 = 0u64;
            nat_eq(a1775, a1776)
        } else {
            false
        } {
            let a1780 = {
                let a1777 = v3;
                let a1778 = v7;
                let a1779 = 1u64;
                nat_rem(a1777, a1778, a1779)
            };
            let a1781 = 0u64;
            nat_eq(a1780, a1781)
        } else {
            false
        }
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1782 = v0;
    let a1783 = v1;
    if nat_eq(a1782, a1783) {
        let a1786 = v4;
        let a1787 = {
            let a1784 = v2;
            let a1785 = v3;
            nat_add(a1784, a1785)?
        };
        Ok(nat_eq(a1786, a1787))
    } else {
        let a1788 = v2;
        let a1789 = v3;
        if nat_eq(a1788, a1789) {
            let a1790 = v4;
            let a1791 = v2;
            Ok(nat_eq(a1790, a1791))
        } else {
            Ok(false)
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1794 = {
            let a1792 = v0;
            let a1793 = f124(v1);
            nat_add(a1792, a1793)?
        };
        let a1795 = f124(v2);
        nat_add(a1794, a1795)?
    };
    let v5: u64 = {
        let a1796 = f125(v1);
        let a1797 = f125(v2);
        nat_add(a1796, a1797)?
    };
    let a1798 = v1;
    let a1799 = 4294967295u64;
    if if nat_le(a1798, a1799) {
        let a1800 = v2;
        let a1801 = 4294967295u64;
        nat_le(a1800, a1801)
    } else {
        false
    } {
        let a1802 = v5;
        let a1803 = v4;
        if nat_le(a1802, a1803) {
            let a1806 = v3;
            let a1807 = {
                let a1804 = v4;
                let a1805 = v5;
                nat_sub(a1804, a1805)
            };
            Ok(nat_eq(a1806, a1807))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f79(v0: u64, v1: u64) -> R<bool> {
    let a1808 = v1;
    let a1809 = 4294967295u64;
    if nat_le(a1808, a1809) {
        let a1812 = {
            let a1810 = f124(v1);
            let a1811 = f125(v1);
            nat_add(a1810, a1811)?
        };
        let a1813 = v0;
        Ok(nat_lt(a1812, a1813))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f126(v0.clone(), false)?;
    let v6: u64 = f126(v0, true)?;
    let a1814 = v6;
    let a1815 = v1;
    if if nat_le(a1814, a1815) {
        let a1822 = {
            let a1820 = {
                let a1816 = v1;
                let a1817 = v5;
                nat_add(a1816, a1817)?
            };
            let a1821 = {
                let a1818 = v3;
                let a1819 = 0u64;
                if nat_eq(a1818, a1819) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1820, a1821)?
        };
        let a1823 = v2;
        nat_le(a1822, a1823)
    } else {
        false
    } {
        if v4 {
            let a1828 = {
                let a1826 = {
                    let a1824 = v1;
                    let a1825 = v5;
                    nat_add(a1824, a1825)?
                };
                let a1827 = v6;
                nat_sub(a1826, a1827)
            };
            let a1829 = v2;
            Ok(nat_lt(a1828, a1829))
        } else {
            let a1834 = {
                let a1832 = {
                    let a1830 = v1;
                    let a1831 = v5;
                    nat_add(a1830, a1831)?
                };
                let a1833 = v6;
                nat_sub(a1832, a1833)
            };
            let a1835 = v2;
            Ok(nat_le(a1834, a1835))
        }
    } else {
        Ok(false)
    }
}

pub fn f81(v0: u64, v1: u64) -> bool {
    let a1836 = v0;
    let a1837 = 0u64;
    if if nat_eq(a1836, a1837) {
        true
    } else {
        let a1838 = v0;
        let a1839 = 1u64;
        nat_eq(a1838, a1839)
    } {
        let a1840 = v1;
        let a1841 = 4294967296u64;
        nat_lt(a1840, a1841)
    } else {
        false
    }
}

pub fn f82(v0: u64, v1: u64) -> R<u64> {
    let a1846 = {
        let a1844 = v0;
        let a1845 = {
            let a1842 = v1;
            let a1843 = 1u64;
            nat_sub(a1842, a1843)
        };
        nat_add(a1844, a1845)?
    };
    let a1847 = v1;
    let a1848 = 0u64;
    Ok(nat_quot(a1846, a1847, a1848))
}

pub fn f83(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f24(match v0.clone() {
        Adt2::C0(_, h1849, _, _, _, _, _, _, _, _, _) => {
            h1849
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1850, _, _, _, _, _, _, _, _) => {
            h1850
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1851, _, _, _, _, _, _, _) => {
            h1851
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1852, _, _, _, _, _, _) => {
            h1852
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1855 = match v1.clone() {
                Adt2::C0(_, h1853, _, _, _, _, _, _, _, _, _) => {
                    h1853
                }
            };
            let a1856 = match v1.clone() {
                Adt2::C0(_, _, h1854, _, _, _, _, _, _, _, _) => {
                    h1854
                }
            };
            nat_mul(a1855, a1856)?
        };
        let v5: u64 = {
            let a1858 = v4;
            let a1859 = match v1.clone() {
                Adt2::C0(_, _, _, h1857, _, _, _, _, _, _, _) => {
                    h1857
                }
            };
            nat_mul(a1858, a1859)?
        };
        let a1872 = {
            let a1870 = {
                let a1861 = match v0.clone() {
                    Adt2::C0(_, h1860, _, _, _, _, _, _, _, _, _) => {
                        h1860
                    }
                };
                let a1862 = 1u64;
                nat_sub(a1861, a1862)
            };
            let a1871 = {
                let a1868 = f127(match v0.clone() {
                    Adt2::C0(_, _, h1863, _, _, _, _, _, _, _, _) => {
                        h1863
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1864, _, _, _, _, _, _, _, _, _) => {
                        h1864
                    }
                })?;
                let a1869 = f127(match v0 {
                    Adt2::C0(_, _, _, h1865, _, _, _, _, _, _, _) => {
                        h1865
                    }
                }, v2, {
                    let a1866 = v3;
                    let a1867 = v5;
                    nat_mul(a1866, a1867)?
                }, v4)?;
                nat_add(a1868, a1869)?
            };
            nat_add(a1870, a1871)?
        };
        let a1873 = f10(v1, 1u64)?;
        Ok(nat_lt(a1872, a1873))
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1874 = v0;
    let a1875 = 2147483648u64;
    if if if if nat_lt(a1874, a1875) {
        let a1876 = v1;
        let a1877 = 2147483648u64;
        nat_lt(a1876, a1877)
    } else {
        false
    } {
        let a1880 = {
            let a1878 = v0;
            let a1879 = v1;
            nat_mul(a1878, a1879)?
        };
        let a1881 = 2147483648u64;
        nat_lt(a1880, a1881)
    } else {
        false
    } {
        let a1886 = {
            let a1884 = {
                let a1882 = v0;
                let a1883 = 1u64;
                nat_sub(a1882, a1883)
            };
            let a1885 = v2;
            nat_mul(a1884, a1885)?
        };
        let a1887 = 2147483648u64;
        nat_lt(a1886, a1887)
    } else {
        false
    } {
        let a1892 = {
            let a1890 = {
                let a1888 = v1;
                let a1889 = 1u64;
                nat_sub(a1888, a1889)
            };
            let a1891 = v2;
            nat_mul(a1890, a1891)?
        };
        let a1893 = 2147483648u64;
        Ok(nat_lt(a1892, a1893))
    } else {
        Ok(false)
    }
}

pub fn f85(v0: u64) -> R<u64> {
    let a1896 = 3u64;
    let a1897 = {
        let a1894 = v0;
        let a1895 = 3u64;
        nat_mul(a1894, a1895)?
    };
    nat_add(a1896, a1897)
}

pub fn f86(v0: Adt3) -> R<u64> {
    if f24(match v0.clone() {
        Adt3::C0(h1898, _, _, _, _, _, _, _) => {
            h1898
        }
    }, match v0.clone() {
        Adt3::C0(_, h1899, _, _, _, _, _, _) => {
            h1899
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1900, _, _, _, _, _) => {
            h1900
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1901, _, _, _, _) => {
            h1901
        }
    }) {
        Ok(0u64)
    } else {
        let a1910 = match v0.clone() {
            Adt3::C0(h1902, _, _, _, _, _, _, _) => {
                h1902
            }
        };
        let a1911 = {
            let a1908 = match v0.clone() {
                Adt3::C0(_, h1903, _, _, _, _, _, _) => {
                    h1903
                }
            };
            let a1909 = {
                let a1906 = match v0.clone() {
                    Adt3::C0(_, _, h1904, _, _, _, _, _) => {
                        h1904
                    }
                };
                let a1907 = match v0 {
                    Adt3::C0(_, _, _, h1905, _, _, _, _) => {
                        h1905
                    }
                };
                nat_mul(a1906, a1907)?
            };
            nat_mul(a1908, a1909)?
        };
        nat_mul(a1910, a1911)
    }
}

pub fn f87(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1912 = v1;
    let a1913 = 13u64;
    if if nat_eq(a1912, a1913) {
        f24(match v0.clone() {
            Adt3::C0(h1914, _, _, _, _, _, _, _) => {
                h1914
            }
        }, match v0.clone() {
            Adt3::C0(_, h1915, _, _, _, _, _, _) => {
                h1915
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1916, _, _, _, _, _) => {
                h1916
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1917, _, _, _, _) => {
                h1917
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1923 = match v0.clone() {
                Adt3::C0(_, h1918, _, _, _, _, _, _) => {
                    h1918
                }
            };
            let a1924 = {
                let a1921 = match v0.clone() {
                    Adt3::C0(_, _, h1919, _, _, _, _, _) => {
                        h1919
                    }
                };
                let a1922 = match v0.clone() {
                    Adt3::C0(_, _, _, h1920, _, _, _, _) => {
                        h1920
                    }
                };
                nat_mul(a1921, a1922)?
            };
            nat_mul(a1923, a1924)?
        };
        let a1925 = v1;
        let a1926 = 13u64;
        if nat_eq(a1925, a1926) {
            let a1935 = 2u64;
            let a1936 = if f128(v0.clone(), v2) {
                let a1928 = v3;
                let a1929 = f129(match v0 {
                    Adt3::C0(h1927, _, _, _, _, _, _, _) => {
                        h1927
                    }
                }, false)?;
                nat_mul(a1928, a1929)?
            } else {
                let a1933 = v3;
                let a1934 = {
                    let a1931 = match v0 {
                        Adt3::C0(h1930, _, _, _, _, _, _, _) => {
                            h1930
                        }
                    };
                    let a1932 = 2u64;
                    nat_mul(a1931, a1932)?
                };
                nat_mul(a1933, a1934)?
            };
            nat_add(a1935, a1936)
        } else {
            let a1940 = v3;
            let a1941 = f129(match v0 {
                Adt3::C0(h1937, _, _, _, _, _, _, _) => {
                    h1937
                }
            }, {
                let a1938 = v1;
                let a1939 = 16u64;
                nat_eq(a1938, a1939)
            })?;
            nat_mul(a1940, a1941)
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let a1942 = v0;
    let a1943 = 30u64;
    if if nat_eq(a1942, a1943) {
        let a1946 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h1944, _, _, _, _) => {
                h1944
            }
        };
        let a1947 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1945, _, _, _, _, _) => {
                h1945
            }
        };
        nat_lt(a1946, a1947)
    } else {
        false
    } {
        Ok(Adt6::C1)
    } else {
        let m1948 = v3;
        match m1948 {
            None => {
                f130(v0, v1, match v2.clone() {
                    Adt2::C0(h1949, _, _, _, _, _, _, _, _, _, _) => {
                        h1949
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h1950, _, _, _, _, _, _, _, _, _, _) => {
                        h1950
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h1951, _, _, _, _, _, _, _, _, _) => {
                        h1951
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h1952, _, _, _, _, _, _, _, _) => {
                        h1952
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h1953, _, _, _, _, _, _, _) => {
                        h1953
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h1954, _, _, _, _, _, _) => {
                        h1954
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h1955, _, _, _, _, _, _, _, _, _) => {
                        h1955
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h1956, _, _, _, _, _, _, _, _) => {
                        h1956
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h1957, _, _, _, _, _, _, _) => {
                        h1957
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h1958, _, _, _, _, _, _) => {
                        h1958
                    }
                })
            }
            Some(v5) => {
                f130(v0, v1, match v2.clone() {
                    Adt2::C0(h1959, _, _, _, _, _, _, _, _, _, _) => {
                        h1959
                    }
                }, match v5.clone() {
                    Adt2::C0(h1960, _, _, _, _, _, _, _, _, _, _) => {
                        h1960
                    }
                }, match v4.clone() {
                    Adt2::C0(h1961, _, _, _, _, _, _, _, _, _, _) => {
                        h1961
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h1962, _, _, _, _, _, _, _, _, _) => {
                        h1962
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h1963, _, _, _, _, _, _, _, _) => {
                        h1963
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h1964, _, _, _, _, _, _, _) => {
                        h1964
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h1965, _, _, _, _, _, _) => {
                        h1965
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h1966, _, _, _, _, _, _, _, _, _) => {
                        h1966
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h1967, _, _, _, _, _, _, _, _) => {
                        h1967
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h1968, _, _, _, _, _, _, _) => {
                        h1968
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h1969, _, _, _, _, _, _) => {
                        h1969
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1970, _, _, _, _, _, _, _, _, _) => {
                        h1970
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h1971, _, _, _, _, _, _, _, _) => {
                        h1971
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h1972, _, _, _, _, _, _, _) => {
                        h1972
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h1973, _, _, _, _, _, _) => {
                        h1973
                    }
                })
            }
        }
    }
}

pub fn f89(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1974 = v7;
        let a1975 = 6u64;
        if nat_eq(a1974, a1975) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f61(v3, v4), f61(v5, v6), f62(v7, v8), f131(v1), f131(v9))
}

pub fn f90(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1976 = v0;
    match m1976 {
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
            let a1977 = 13u64;
            let a1978 = v34;
            if nat_le(a1977, a1978) {
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
    let m1979 = v0;
    match m1979 {
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
    let m1980 = v0;
    match m1980 {
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
            let a1981 = v1;
            let a1982 = 6u64;
            if nat_eq(a1981, a1982) {
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
            let a1983 = v1;
            let a1984 = 6u64;
            if if if nat_eq(a1983, a1984) {
                true
            } else {
                let a1985 = v1;
                let a1986 = 7u64;
                nat_eq(a1985, a1986)
            } {
                let a1987 = v49;
                let a1988 = 30u64;
                nat_eq(a1987, a1988)
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
    let a1989 = v1;
    let a1990 = 4u64;
    if nat_lt(a1989, a1990) {
        Adt7::C0(v1)
    } else {
        let a1991 = v1;
        let a1992 = 4u64;
        if nat_eq(a1991, a1992) {
            Adt7::C1(0u64)
        } else {
            let a1993 = v1;
            let a1994 = 5u64;
            if nat_eq(a1993, a1994) {
                Adt7::C1(3u64)
            } else {
                let a1995 = v1;
                let a1996 = 6u64;
                if nat_eq(a1995, a1996) {
                    Adt7::C2(f92(v0, v1), 0u64, false)
                } else {
                    let a1997 = v1;
                    let a1998 = 7u64;
                    if nat_eq(a1997, a1998) {
                        Adt7::C2(0u64, f92(v0, v1), false)
                    } else {
                        let a1999 = v1;
                        let a2000 = 16u64;
                        if nat_lt(a1999, a2000) {
                            Adt7::C1({
                                let a2001 = v1;
                                let a2002 = 8u64;
                                nat_sub(a2001, a2002)
                            })
                        } else {
                            let a2003 = v1;
                            let a2004 = 20u64;
                            if nat_lt(a2003, a2004) {
                                Adt7::C5({
                                    let a2005 = v1;
                                    let a2006 = 16u64;
                                    nat_sub(a2005, a2006)
                                })
                            } else {
                                let a2007 = v1;
                                let a2008 = 20u64;
                                if nat_eq(a2007, a2008) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a2009 = v1;
                                    let a2010 = 21u64;
                                    if nat_eq(a2009, a2010) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a2011 = v1;
                                        let a2012 = 22u64;
                                        if nat_eq(a2011, a2012) {
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
        let a2014 = v0;
        let a2015 = match v1.clone() {
            Adt3::C0(h2013, _, _, _, _, _, _, _) => {
                h2013
            }
        };
        let a2016 = 0u64;
        nat_quot(a2014, a2015, a2016)
    };
    let v3: u64 = {
        let a2018 = v2;
        let a2019 = match v1.clone() {
            Adt3::C0(_, h2017, _, _, _, _, _, _) => {
                h2017
            }
        };
        let a2020 = 0u64;
        nat_quot(a2018, a2019, a2020)
    };
    f132({
        let a2022 = v0;
        let a2023 = match v1.clone() {
            Adt3::C0(h2021, _, _, _, _, _, _, _) => {
                h2021
            }
        };
        let a2024 = 0u64;
        nat_rem(a2022, a2023, a2024)
    }, {
        let a2026 = v2;
        let a2027 = match v1.clone() {
            Adt3::C0(_, h2025, _, _, _, _, _, _) => {
                h2025
            }
        };
        let a2028 = 0u64;
        nat_rem(a2026, a2027, a2028)
    }, {
        let a2030 = v3;
        let a2031 = match v1.clone() {
            Adt3::C0(_, _, h2029, _, _, _, _, _) => {
                h2029
            }
        };
        let a2032 = 0u64;
        nat_rem(a2030, a2031, a2032)
    }, {
        let a2034 = v3;
        let a2035 = match v1.clone() {
            Adt3::C0(_, _, h2033, _, _, _, _, _) => {
                h2033
            }
        };
        let a2036 = 0u64;
        nat_quot(a2034, a2035, a2036)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2037, _, _, _) => {
            h2037
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2038, _, _) => {
            h2038
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2039, _) => {
            h2039
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2040) => {
            h2040
        }
    })
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2058 = {
        let a2053 = f133(v2, {
            let a2044 = {
                let a2041 = v0;
                let a2042 = v1;
                let a2043 = 0u64;
                nat_quot(a2041, a2042, a2043)
            };
            let a2045 = v2;
            let a2046 = 0u64;
            nat_rem(a2044, a2045, a2046)
        }, {
            let a2050 = {
                let a2047 = v0;
                let a2048 = v1;
                let a2049 = 0u64;
                nat_quot(a2047, a2048, a2049)
            };
            let a2051 = v2;
            let a2052 = 0u64;
            nat_quot(a2050, a2051, a2052)
        })?;
        let a2054 = v1;
        nat_mul(a2053, a2054)?
    };
    let a2059 = {
        let a2055 = v0;
        let a2056 = v1;
        let a2057 = 0u64;
        nat_rem(a2055, a2056, a2057)
    };
    nat_add(a2058, a2059)
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2061 = v0;
        let a2062 = match v2.clone() {
            Adt3::C0(h2060, _, _, _, _, _, _, _) => {
                h2060
            }
        };
        let a2063 = 0u64;
        nat_quot(a2061, a2062, a2063)
    };
    let v4: u64 = {
        let a2065 = v3;
        let a2066 = match v2.clone() {
            Adt3::C0(_, h2064, _, _, _, _, _, _) => {
                h2064
            }
        };
        let a2067 = 0u64;
        nat_quot(a2065, a2066, a2067)
    };
    f134({
        let a2069 = v0;
        let a2070 = match v2.clone() {
            Adt3::C0(h2068, _, _, _, _, _, _, _) => {
                h2068
            }
        };
        let a2071 = 0u64;
        nat_rem(a2069, a2070, a2071)
    }, {
        let a2073 = v3;
        let a2074 = match v2.clone() {
            Adt3::C0(_, h2072, _, _, _, _, _, _) => {
                h2072
            }
        };
        let a2075 = 0u64;
        nat_rem(a2073, a2074, a2075)
    }, {
        let a2077 = v4;
        let a2078 = match v2.clone() {
            Adt3::C0(_, _, h2076, _, _, _, _, _) => {
                h2076
            }
        };
        let a2079 = 0u64;
        nat_rem(a2077, a2078, a2079)
    }, {
        let a2081 = v4;
        let a2082 = match v2 {
            Adt3::C0(_, _, h2080, _, _, _, _, _) => {
                h2080
            }
        };
        let a2083 = 0u64;
        nat_quot(a2081, a2082, a2083)
    }, match v1.clone() {
        Adt3::C0(h2084, _, _, _, _, _, _, _) => {
            h2084
        }
    }, match v1.clone() {
        Adt3::C0(_, h2085, _, _, _, _, _, _) => {
            h2085
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2086, _, _, _, _, _) => {
            h2086
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2087, _, _, _, _) => {
            h2087
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2088, _, _, _) => {
            h2088
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2089, _, _) => {
            h2089
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2090, _) => {
            h2090
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2091) => {
            h2091
        }
    })
}

pub fn f97(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2093 = v0;
        let a2094 = match v3.clone() {
            Adt3::C0(h2092, _, _, _, _, _, _, _) => {
                h2092
            }
        };
        let a2095 = 0u64;
        nat_quot(a2093, a2094, a2095)
    };
    let v6: u64 = {
        let a2097 = v5;
        let a2098 = match v3.clone() {
            Adt3::C0(_, h2096, _, _, _, _, _, _) => {
                h2096
            }
        };
        let a2099 = 0u64;
        nat_quot(a2097, a2098, a2099)
    };
    let v7: u64 = {
        let a2101 = v0;
        let a2102 = match v3.clone() {
            Adt3::C0(h2100, _, _, _, _, _, _, _) => {
                h2100
            }
        };
        let a2103 = 0u64;
        nat_rem(a2101, a2102, a2103)
    };
    let v8: u64 = {
        let a2105 = v5;
        let a2106 = match v3.clone() {
            Adt3::C0(_, h2104, _, _, _, _, _, _) => {
                h2104
            }
        };
        let a2107 = 0u64;
        nat_rem(a2105, a2106, a2107)
    };
    let v9: u64 = {
        let a2109 = v6;
        let a2110 = match v3.clone() {
            Adt3::C0(_, _, h2108, _, _, _, _, _) => {
                h2108
            }
        };
        let a2111 = 0u64;
        nat_rem(a2109, a2110, a2111)
    };
    let v10: u64 = {
        let a2113 = v6;
        let a2114 = match v3.clone() {
            Adt3::C0(_, _, h2112, _, _, _, _, _) => {
                h2112
            }
        };
        let a2115 = 0u64;
        nat_quot(a2113, a2114, a2115)
    };
    let v11: u64 = {
        let a2116 = v4;
        let a2117 = 0u64;
        if nat_eq(a2116, a2117) {
            match v1.clone() {
                Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                    h2118
                }
            }
        } else {
            let a2119 = v4;
            let a2120 = 1u64;
            if nat_eq(a2119, a2120) {
                match v1.clone() {
                    Adt3::C0(_, h2121, _, _, _, _, _, _) => {
                        h2121
                    }
                }
            } else {
                let a2122 = v4;
                let a2123 = 2u64;
                if nat_eq(a2122, a2123) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2124, _, _, _, _, _) => {
                            h2124
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2125, _, _, _, _) => {
                            h2125
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2132 = v11;
        let a2133 = {
            let a2126 = v4;
            let a2127 = 0u64;
            if nat_eq(a2126, a2127) {
                v7
            } else {
                let a2128 = v4;
                let a2129 = 1u64;
                if nat_eq(a2128, a2129) {
                    v8
                } else {
                    let a2130 = v4;
                    let a2131 = 2u64;
                    if nat_eq(a2130, a2131) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2132, a2133)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f132(if if v12 {
        let a2134 = v4;
        let a2135 = 0u64;
        nat_eq(a2134, a2135)
    } else {
        false
    } {
        let a2136 = v7;
        let a2137 = v11;
        nat_sub(a2136, a2137)
    } else {
        v7
    }, if if v12 {
        let a2138 = v4;
        let a2139 = 1u64;
        nat_eq(a2138, a2139)
    } else {
        false
    } {
        let a2140 = v8;
        let a2141 = v11;
        nat_sub(a2140, a2141)
    } else {
        v8
    }, if if v12 {
        let a2142 = v4;
        let a2143 = 2u64;
        nat_eq(a2142, a2143)
    } else {
        false
    } {
        let a2144 = v9;
        let a2145 = v11;
        nat_sub(a2144, a2145)
    } else {
        v9
    }, if if v12 {
        let a2146 = v4;
        let a2147 = 3u64;
        nat_eq(a2146, a2147)
    } else {
        false
    } {
        let a2148 = v10;
        let a2149 = v11;
        nat_sub(a2148, a2149)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2150, _, _, _) => {
            h2150
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2151, _, _) => {
            h2151
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2152, _) => {
            h2152
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2153) => {
            h2153
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
            Adt3::C0(h2154, _, _, _, _, _, _, _) => {
                h2154
            }
        })?, f137(v9, v4, match v1.clone() {
            Adt3::C0(_, h2155, _, _, _, _, _, _) => {
                h2155
            }
        })?, f137(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2156, _, _, _, _, _) => {
                h2156
            }
        })?, f137(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2157, _, _, _, _) => {
                h2157
            }
        })?)?, f94(v0, v2)?))
    } else {
        if if if if f138(v8, v3, match v1.clone() {
            Adt3::C0(h2158, _, _, _, _, _, _, _) => {
                h2158
            }
        })? {
            f138(v9, v4, match v1.clone() {
                Adt3::C0(_, h2159, _, _, _, _, _, _) => {
                    h2159
                }
            })?
        } else {
            false
        } {
            f138(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2160, _, _, _, _, _) => {
                    h2160
                }
            })?
        } else {
            false
        } {
            f138(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2161, _, _, _, _) => {
                    h2161
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
        Adt3::C0(h2162, _, _, _, _, _, _, _) => {
            h2162
        }
    })?, v5, v6, v7)?, f94(v0, v2)?))
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2163 = v0;
    let a2164 = v3;
    if nat_lt(a2163, a2164) {
        Ok(Adt5::C1(false, {
            let a2165 = v0;
            let a2166 = 4u64;
            nat_mul(a2165, a2166)?
        }, {
            let a2167 = v0;
            let a2168 = 4u64;
            nat_mul(a2167, a2168)?
        }))
    } else {
        let v6: u64 = {
            let a2169 = v0;
            let a2170 = v3;
            nat_sub(a2169, a2170)
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
    let a2171 = v0;
    let a2172 = v3;
    if nat_lt(a2171, a2172) {
        Ok(Adt5::C1(true, 0u64, f94(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2173 = v0;
            let a2174 = v3;
            nat_sub(a2173, a2174)
        };
        let v5: u64 = f96(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f94(v4, v1)?, 3u64, v5))
    }
}

pub fn f102(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2176 = v1;
        let a2177 = match v3.clone() {
            Adt3::C0(h2175, _, _, _, _, _, _, _) => {
                h2175
            }
        };
        let a2178 = 0u64;
        nat_rem(a2176, a2177, a2178)
    };
    let v5: u64 = {
        let a2184 = {
            let a2180 = v1;
            let a2181 = match v3.clone() {
                Adt3::C0(h2179, _, _, _, _, _, _, _) => {
                    h2179
                }
            };
            let a2182 = 0u64;
            nat_quot(a2180, a2181, a2182)
        };
        let a2185 = match v3.clone() {
            Adt3::C0(_, h2183, _, _, _, _, _, _) => {
                h2183
            }
        };
        let a2186 = 0u64;
        nat_rem(a2184, a2185, a2186)
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
        let a2187 = v3;
        let a2188 = 8u64;
        nat_add(a2187, a2188)?
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
        let a2190 = v0;
        let a2191 = match v2.clone() {
            Adt3::C0(h2189, _, _, _, _, _, _, _) => {
                h2189
            }
        };
        let a2192 = 0u64;
        nat_rem(a2190, a2191, a2192)
    };
    let v7: u64 = {
        let a2194 = v0;
        let a2195 = match v2.clone() {
            Adt3::C0(h2193, _, _, _, _, _, _, _) => {
                h2193
            }
        };
        let a2196 = 0u64;
        nat_quot(a2194, a2195, a2196)
    };
    let v8: u64 = {
        let a2198 = v7;
        let a2199 = match v2.clone() {
            Adt3::C0(_, h2197, _, _, _, _, _, _) => {
                h2197
            }
        };
        let a2200 = 0u64;
        nat_rem(a2198, a2199, a2200)
    };
    let v9: u64 = {
        let a2202 = v7;
        let a2203 = match v2.clone() {
            Adt3::C0(_, h2201, _, _, _, _, _, _) => {
                h2201
            }
        };
        let a2204 = 0u64;
        nat_quot(a2202, a2203, a2204)
    };
    let v10: u64 = {
        let a2206 = v9;
        let a2207 = match v2.clone() {
            Adt3::C0(_, _, h2205, _, _, _, _, _) => {
                h2205
            }
        };
        let a2208 = 0u64;
        nat_rem(a2206, a2207, a2208)
    };
    let v11: u64 = {
        let a2210 = v9;
        let a2211 = match v2.clone() {
            Adt3::C0(_, _, h2209, _, _, _, _, _) => {
                h2209
            }
        };
        let a2212 = 0u64;
        nat_quot(a2210, a2211, a2212)
    };
    let v12: u64 = f94(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2221 = {
                let a2216 = {
                    let a2213 = v10;
                    let a2214 = v4;
                    let a2215 = 0u64;
                    nat_quot(a2213, a2214, a2215)
                };
                let a2217 = v5;
                nat_mul(a2216, a2217)?
            };
            let a2222 = {
                let a2218 = v8;
                let a2219 = v4;
                let a2220 = 0u64;
                nat_quot(a2218, a2219, a2220)
            };
            nat_add(a2221, a2222)?
        };
        let v14: u64 = {
            let a2242 = v6;
            let a2243 = {
                let a2240 = match v1.clone() {
                    Adt3::C0(h2223, _, _, _, _, _, _, _) => {
                        h2223
                    }
                };
                let a2241 = {
                    let a2238 = {
                        let a2224 = v8;
                        let a2225 = v4;
                        let a2226 = 0u64;
                        nat_rem(a2224, a2225, a2226)
                    };
                    let a2239 = {
                        let a2236 = match v1.clone() {
                            Adt3::C0(_, h2227, _, _, _, _, _, _) => {
                                h2227
                            }
                        };
                        let a2237 = {
                            let a2234 = {
                                let a2228 = v10;
                                let a2229 = v4;
                                let a2230 = 0u64;
                                nat_rem(a2228, a2229, a2230)
                            };
                            let a2235 = {
                                let a2232 = match v1.clone() {
                                    Adt3::C0(_, _, h2231, _, _, _, _, _) => {
                                        h2231
                                    }
                                };
                                let a2233 = v13;
                                nat_mul(a2232, a2233)?
                            };
                            nat_add(a2234, a2235)?
                        };
                        nat_mul(a2236, a2237)?
                    };
                    nat_add(a2238, a2239)?
                };
                nat_mul(a2240, a2241)?
            };
            nat_add(a2242, a2243)?
        };
        Ok(Adt5::C1(false, f94(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2249 = {
                let a2247 = {
                    let a2244 = v11;
                    let a2245 = v5;
                    let a2246 = 0u64;
                    nat_rem(a2244, a2245, a2246)
                };
                let a2248 = v4;
                nat_mul(a2247, a2248)?
            };
            let a2250 = v8;
            nat_add(a2249, a2250)?
        };
        let v16: u64 = {
            let a2256 = {
                let a2254 = {
                    let a2251 = v11;
                    let a2252 = v5;
                    let a2253 = 0u64;
                    nat_quot(a2251, a2252, a2253)
                };
                let a2255 = v4;
                nat_mul(a2254, a2255)?
            };
            let a2257 = v10;
            nat_add(a2256, a2257)?
        };
        let a2259 = v15;
        let a2260 = match v1.clone() {
            Adt3::C0(_, h2258, _, _, _, _, _, _) => {
                h2258
            }
        };
        if if nat_lt(a2259, a2260) {
            let a2262 = v16;
            let a2263 = match v1.clone() {
                Adt3::C0(_, _, h2261, _, _, _, _, _) => {
                    h2261
                }
            };
            nat_lt(a2262, a2263)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f94({
                let a2272 = v6;
                let a2273 = {
                    let a2270 = match v1.clone() {
                        Adt3::C0(h2264, _, _, _, _, _, _, _) => {
                            h2264
                        }
                    };
                    let a2271 = {
                        let a2268 = v15;
                        let a2269 = {
                            let a2266 = match v1.clone() {
                                Adt3::C0(_, h2265, _, _, _, _, _, _) => {
                                    h2265
                                }
                            };
                            let a2267 = v16;
                            nat_mul(a2266, a2267)?
                        };
                        nat_add(a2268, a2269)?
                    };
                    nat_mul(a2270, a2271)?
                };
                nat_add(a2272, a2273)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f105(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f94(v0, v2.clone())?;
    let v4: u64 = f94(v0, v1.clone())?;
    let a2278 = {
        let a2275 = v0;
        let a2276 = match v1 {
            Adt3::C0(h2274, _, _, _, _, _, _, _) => {
                h2274
            }
        };
        let a2277 = 0u64;
        nat_rem(a2275, a2276, a2277)
    };
    let a2279 = 0u64;
    if nat_eq(a2278, a2279) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f94({
            let a2280 = v0;
            let a2281 = 1u64;
            nat_sub(a2280, a2281)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f106(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2282 = v3;
    let a2283 = 13u64;
    if nat_eq(a2282, a2283) {
        f143(v0, v1, v4)
    } else {
        let v5: u64 = f129(match v1.clone() {
            Adt3::C0(h2284, _, _, _, _, _, _, _) => {
                h2284
            }
        }, {
            let a2285 = v3;
            let a2286 = 16u64;
            nat_eq(a2285, a2286)
        })?;
        let v6: u64 = {
            let a2287 = v0;
            let a2288 = v5;
            let a2289 = 0u64;
            nat_quot(a2287, a2288, a2289)
        };
        let v7: u64 = {
            let a2290 = v0;
            let a2291 = v5;
            let a2292 = 0u64;
            nat_rem(a2290, a2291, a2292)
        };
        let v8: u64 = {
            let a2296 = 1u64;
            let a2297 = {
                let a2294 = match v1.clone() {
                    Adt3::C0(h2293, _, _, _, _, _, _, _) => {
                        h2293
                    }
                };
                let a2295 = 2u64;
                nat_mul(a2294, a2295)?
            };
            nat_add(a2296, a2297)?
        };
        let a2298 = v7;
        let a2299 = v8;
        if nat_lt(a2298, a2299) {
            f144(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f94(v6, v2)?;
            let a2300 = v3;
            let a2301 = 14u64;
            if nat_eq(a2300, a2301) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2302 = v7;
                let a2303 = v8;
                if nat_eq(a2302, a2303) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2306 = v7;
                    let a2307 = {
                        let a2304 = v8;
                        let a2305 = 1u64;
                        nat_add(a2304, a2305)?
                    };
                    if nat_eq(a2306, a2307) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2308, _, _, _, _, _, _, _) => {
                                h2308
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
    let a2309 = v0;
    let a2310 = 0u64;
    if nat_eq(a2309, a2310) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2317 = {
            let a2311 = v0;
            let a2312 = 1u64;
            nat_add(a2311, a2312)?
        };
        let a2318 = {
            let a2315 = 2u64;
            let a2316 = {
                let a2313 = v3;
                let a2314 = 2u64;
                nat_mul(a2313, a2314)?
            };
            nat_add(a2315, a2316)?
        };
        if nat_eq(a2317, a2318) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2319 = v0;
                let a2320 = 1u64;
                nat_sub(a2319, a2320)
            };
            let v5: u64 = {
                let a2321 = v4;
                let a2322 = 2u64;
                let a2323 = 0u64;
                nat_quot(a2321, a2322, a2323)
            };
            let a2327 = {
                let a2324 = v4;
                let a2325 = 2u64;
                let a2326 = 0u64;
                nat_rem(a2324, a2325, a2326)
            };
            let a2328 = 0u64;
            if nat_eq(a2327, a2328) {
                Ok(Adt5::C4(17u64, 0u64, f94(v5, v1)?, 4u64, f94(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2329 = v0;
    let a2330 = f19(v1, v2, v3)?;
    if nat_lt(a2329, a2330) {
        let a2331 = v0;
        let a2332 = v1;
        if nat_lt(a2331, a2332) {
            Ok(Adt5::C1(false, {
                let a2333 = 4u64;
                let a2334 = v0;
                nat_mul(a2333, a2334)?
            }, {
                let a2335 = 4u64;
                let a2336 = v0;
                nat_mul(a2335, a2336)?
            }))
        } else {
            let v4: u64 = {
                let a2337 = v0;
                let a2338 = v1;
                nat_sub(a2337, a2338)
            };
            let v5: u64 = f145(v4, v2)?;
            let a2339 = v5;
            let a2340 = 2u64;
            if nat_lt(a2339, a2340) {
                Ok(Adt5::C4(6u64, {
                    let a2341 = v5;
                    let a2342 = 0u64;
                    if nat_eq(a2341, a2342) {
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
    let a2350 = {
        let a2343 = 2u64;
        let a2344 = f86(v1.clone())?;
        nat_mul(a2343, a2344)?
    };
    let a2351 = {
        let a2348 = f86(v1)?;
        let a2349 = {
            let a2346 = 1u64;
            let a2347 = match v0 {
                Adt3::C0(h2345, _, _, _, _, _, _, _) => {
                    h2345
                }
            };
            nat_add(a2346, a2347)?
        };
        nat_mul(a2348, a2349)?
    };
    nat_add(a2350, a2351)
}

pub fn f110(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f94(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f111(v0: u64, v1: u64) -> bool {
    let a2352 = v1;
    let a2353 = 2147483647u64;
    if nat_le(a2352, a2353) {
        let a2354 = v1;
        let a2355 = v0;
        nat_lt(a2354, a2355)
    } else {
        false
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2356 = f75(v0);
    match m2356 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2358 = v2;
                let a2359 = match v7.clone() {
                    Adt11::C0(_, h2357, _) => {
                        h2357
                    }
                };
                nat_mul(a2358, a2359)?
            };
            let m2372 = f149(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2360, _, _, _, _, _, _, _) => {
                    h2360
                }
            }, match v5.clone() {
                Adt3::C0(_, h2361, _, _, _, _, _, _) => {
                    h2361
                }
            }, match v5 {
                Adt3::C0(_, _, h2362, _, _, _, _, _) => {
                    h2362
                }
            }, {
                let a2365 = match v3.clone() {
                    Adt3::C0(h2363, _, _, _, _, _, _, _) => {
                        h2363
                    }
                };
                let a2366 = match v7.clone() {
                    Adt11::C0(_, h2364, _) => {
                        h2364
                    }
                };
                nat_mul(a2365, a2366)?
            }, match v3.clone() {
                Adt3::C0(_, h2367, _, _, _, _, _, _) => {
                    h2367
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2368, _, _, _) => {
                    h2368
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2369, _, _) => {
                    h2369
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2370, _) => {
                    h2370
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2371) => {
                    h2371
                }
            })?;
            match m2372 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2373 = v0;
                    let a2374 = 0u64;
                    if if nat_eq(a2373, a2374) {
                        true
                    } else {
                        let a2375 = v0;
                        let a2376 = 26u64;
                        nat_eq(a2375, a2376)
                    } {
                        Ok(Adt5::C1(false, v9, f94({
                            let a2380 = {
                                let a2378 = v1;
                                let a2379 = match v4.clone() {
                                    Adt3::C0(h2377, _, _, _, _, _, _, _) => {
                                        h2377
                                    }
                                };
                                nat_mul(a2378, a2379)?
                            };
                            let a2381 = v8;
                            nat_add(a2380, a2381)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2382) => {
                                h2382
                            }
                        }, f94({
                            let a2386 = {
                                let a2384 = v1;
                                let a2385 = match v4.clone() {
                                    Adt3::C0(h2383, _, _, _, _, _, _, _) => {
                                        h2383
                                    }
                                };
                                nat_mul(a2384, a2385)?
                            };
                            let a2387 = v8;
                            nat_add(a2386, a2387)?
                        }, v4.clone())?, {
                            let a2394 = {
                                let a2392 = {
                                    let a2389 = match v7 {
                                        Adt11::C0(_, h2388, _) => {
                                            h2388
                                        }
                                    };
                                    let a2390 = 1u64;
                                    nat_sub(a2389, a2390)
                                };
                                let a2393 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2391, _, _, _) => {
                                        h2391
                                    }
                                };
                                nat_mul(a2392, a2393)?
                            };
                            let a2395 = 4u64;
                            nat_add(a2394, a2395)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2396, _, _, _) => {
                                h2396
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2397 = v1;
    let a2398 = 0u64;
    if if nat_eq(a2397, a2398) {
        true
    } else {
        let a2399 = v2;
        let a2400 = 0u64;
        nat_eq(a2399, a2400)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f114(v0: u64, v1: u64) -> bool {
    let a2401 = v0;
    let a2402 = 4294967295u64;
    if nat_eq(a2401, a2402) {
        true
    } else {
        let a2403 = v0;
        let a2404 = 2147483648u64;
        if nat_lt(a2403, a2404) {
            let a2405 = v0;
            let a2406 = v1;
            nat_lt(a2405, a2406)
        } else {
            false
        }
    }
}

pub fn f115(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2413 = v3;
        let a2414 = {
            let a2410 = f94({
                let a2407 = v0;
                let a2408 = 2u64;
                let a2409 = 0u64;
                nat_quot(a2407, a2408, a2409)
            }, v1)?;
            let a2411 = v2;
            let a2412 = 0u64;
            nat_quot(a2410, a2411, a2412)
        };
        nat_add(a2413, a2414)?
    };
    let a2418 = {
        let a2415 = v0;
        let a2416 = 2u64;
        let a2417 = 0u64;
        nat_rem(a2415, a2416, a2417)
    };
    let a2419 = 0u64;
    if nat_eq(a2418, a2419) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2420 = v4;
        let a2421 = 0u64;
        if nat_eq(a2420, a2421) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f116(v0: u64, v1: u64) -> R<u64> {
    let a2424 = 8u64;
    let a2425 = {
        let a2422 = f150(v0);
        let a2423 = v1;
        nat_mul(a2422, a2423)?
    };
    nat_add(a2424, a2425)
}

pub fn f117(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2426, _, _, _, _, _, _, _) => {
            h2426
        }
    };
    let v7: u64 = f150(v3);
    let v8: u64 = {
        let a2429 = 1u64;
        let a2430 = {
            let a2427 = v6;
            let a2428 = v7;
            nat_mul(a2427, a2428)?
        };
        nat_add(a2429, a2430)?
    };
    let a2431 = v0;
    let a2432 = 0u64;
    if nat_eq(a2431, a2432) {
        Ok(f151(0u64))
    } else {
        let a2433 = v0;
        let a2434 = v8;
        if nat_lt(a2433, a2434) {
            let v9: u64 = {
                let a2435 = v0;
                let a2436 = 1u64;
                nat_sub(a2435, a2436)
            };
            let v10: u64 = {
                let a2437 = v9;
                let a2438 = v7;
                let a2439 = 0u64;
                nat_rem(a2437, a2438, a2439)
            };
            let v11: u64 = f94({
                let a2445 = {
                    let a2440 = v1;
                    let a2441 = v6;
                    nat_mul(a2440, a2441)?
                };
                let a2446 = {
                    let a2442 = v9;
                    let a2443 = v7;
                    let a2444 = 0u64;
                    nat_quot(a2442, a2443, a2444)
                };
                nat_add(a2445, a2446)?
            }, v2)?;
            let a2447 = v3;
            let a2448 = 30u64;
            if nat_eq(a2447, a2448) {
                let a2449 = v10;
                let a2450 = 0u64;
                if nat_eq(a2449, a2450) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2451 = v10;
                    let a2452 = 1u64;
                    if nat_eq(a2451, a2452) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2453 = v10;
                        let a2454 = 2u64;
                        if nat_eq(a2453, a2454) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2455 = v10;
                let a2456 = 0u64;
                if nat_eq(a2455, a2456) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2457 = v10;
                    let a2458 = 1u64;
                    if nat_eq(a2457, a2458) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2459 = v0;
                let a2460 = v8;
                nat_sub(a2459, a2460)
            };
            let a2461 = v12;
            let a2462 = 0u64;
            if nat_eq(a2461, a2462) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2463 = v12;
                let a2464 = 1u64;
                if nat_eq(a2463, a2464) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2465 = v12;
                    let a2466 = 2u64;
                    if nat_eq(a2465, a2466) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2467 = v12;
                        let a2468 = 3u64;
                        if nat_eq(a2467, a2468) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2469 = v12;
                            let a2470 = 4u64;
                            if nat_eq(a2469, a2470) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2471 = v12;
                                let a2472 = 5u64;
                                if nat_eq(a2471, a2472) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2473 = v12;
                                    let a2474 = 6u64;
                                    if nat_eq(a2473, a2474) {
                                        if f152(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2475 = 3u64;
                                                let a2476 = v1;
                                                nat_add(a2475, a2476)?
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
        let a2477 = v0;
        let a2478 = f69(v3);
        let a2479 = 0u64;
        nat_quot(a2477, a2478, a2479)
    };
    let v5: u64 = {
        let a2484 = 3u64;
        let a2485 = {
            let a2481 = v4;
            let a2482 = match v1.clone() {
                Adt3::C0(h2480, _, _, _, _, _, _, _) => {
                    h2480
                }
            };
            let a2483 = 0u64;
            nat_quot(a2481, a2482, a2483)
        };
        nat_add(a2484, a2485)?
    };
    let v6: u64 = f94(v4, v1)?;
    let v7: u64 = f94(v4, v2)?;
    let a2486 = v3;
    let a2487 = 30u64;
    if nat_eq(a2486, a2487) {
        let a2491 = {
            let a2488 = v0;
            let a2489 = 3u64;
            let a2490 = 0u64;
            nat_rem(a2488, a2489, a2490)
        };
        let a2492 = 0u64;
        if nat_eq(a2491, a2492) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2496 = {
                let a2493 = v0;
                let a2494 = 3u64;
                let a2495 = 0u64;
                nat_rem(a2493, a2494, a2495)
            };
            let a2497 = 1u64;
            if nat_eq(a2496, a2497) {
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
        let a2498 = v0;
        let a2499 = 8388608u64;
        let a2500 = 0u64;
        nat_quot(a2498, a2499, a2500)
    };
    let a2501 = v0;
    let a2502 = 2147483648u64;
    if if if nat_lt(a2501, a2502) {
        let a2503 = 127u64;
        let a2504 = v1;
        nat_le(a2503, a2504)
    } else {
        false
    } {
        let a2505 = v1;
        let a2506 = 190u64;
        nat_lt(a2505, a2506)
    } else {
        false
    } {
        let v2: u64 = {
            let a2510 = 8388608u64;
            let a2511 = {
                let a2507 = v0;
                let a2508 = 8388608u64;
                let a2509 = 0u64;
                nat_rem(a2507, a2508, a2509)
            };
            nat_add(a2510, a2511)?
        };
        let a2512 = v1;
        let a2513 = 150u64;
        if nat_lt(a2512, a2513) {
            let v3: u64 = f153({
                let a2514 = 150u64;
                let a2515 = v1;
                nat_sub(a2514, a2515)
            });
            let a2519 = {
                let a2516 = v2;
                let a2517 = v3;
                let a2518 = 0u64;
                nat_rem(a2516, a2517, a2518)
            };
            let a2520 = 0u64;
            if nat_eq(a2519, a2520) {
                let a2521 = v2;
                let a2522 = v3;
                let a2523 = 0u64;
                Ok(nat_quot(a2521, a2522, a2523))
            } else {
                Ok(0u64)
            }
        } else {
            let a2526 = f153({
                let a2524 = v1;
                let a2525 = 150u64;
                nat_sub(a2524, a2525)
            });
            let a2527 = v2;
            nat_mul(a2526, a2527)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f120(v0: u64, v1: u64, v2: u64) -> bool {
    let a2528 = v0;
    let a2529 = 1u64;
    if nat_le(a2528, a2529) {
        true
    } else {
        let a2533 = {
            let a2530 = v1;
            let a2531 = v2;
            let a2532 = 1u64;
            nat_rem(a2530, a2531, a2532)
        };
        let a2534 = 0u64;
        nat_eq(a2533, a2534)
    }
}

pub fn f121(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2535 = v1;
    let a2536 = 1u64;
    if if if nat_le(a2535, a2536) {
        true
    } else {
        let a2537 = v0;
        let a2538 = v2;
        nat_le(a2537, a2538)
    } {
        let a2539 = v3;
        let a2540 = 1u64;
        if nat_le(a2539, a2540) {
            true
        } else {
            let a2547 = {
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
            let a2548 = v4;
            nat_le(a2547, a2548)
        }
    } else {
        false
    } {
        let a2549 = v5;
        let a2550 = 1u64;
        if nat_le(a2549, a2550) {
            Ok(true)
        } else {
            let a2563 = {
                let a2561 = {
                    let a2555 = v0;
                    let a2556 = {
                        let a2553 = {
                            let a2551 = v1;
                            let a2552 = 1u64;
                            nat_sub(a2551, a2552)
                        };
                        let a2554 = v2;
                        nat_mul(a2553, a2554)?
                    };
                    nat_add(a2555, a2556)?
                };
                let a2562 = {
                    let a2559 = {
                        let a2557 = v3;
                        let a2558 = 1u64;
                        nat_sub(a2557, a2558)
                    };
                    let a2560 = v4;
                    nat_mul(a2559, a2560)?
                };
                nat_add(a2561, a2562)?
            };
            let a2564 = v6;
            Ok(nat_le(a2563, a2564))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f154(v0, v1, v6, v7, v8) {
        let a2565 = v5;
        let a2566 = 26u64;
        nat_eq(a2565, a2566)
    } else {
        false
    } {
        let a2567 = v3;
        let a2568 = v7;
        nat_eq(a2567, a2568)
    } else {
        false
    } {
        let a2569 = v4;
        let a2570 = v8;
        nat_eq(a2569, a2570)
    } else {
        false
    } {
        let a2571 = v9;
        let a2572 = 1u64;
        nat_eq(a2571, a2572)
    } else {
        false
    } {
        let a2573 = v10;
        let a2574 = f155(v0);
        nat_eq(a2573, a2574)
    } else {
        false
    } {
        let a2575 = v11;
        let a2576 = v1;
        nat_eq(a2575, a2576)
    } else {
        false
    } {
        let a2577 = v12;
        let a2578 = v6;
        nat_eq(a2577, a2578)
    } else {
        false
    } {
        let a2579 = v13;
        let a2580 = v7;
        nat_eq(a2579, a2580)
    } else {
        false
    } {
        let a2581 = v14;
        let a2582 = v8;
        nat_eq(a2581, a2582)
    } else {
        false
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2583 = v0;
    let a2584 = 0u64;
    if if if nat_eq(a2583, a2584) {
        true
    } else {
        let a2585 = v1;
        let a2586 = 0u64;
        nat_eq(a2585, a2586)
    } {
        true
    } else {
        let a2587 = v2;
        let a2588 = 0u64;
        nat_eq(a2587, a2588)
    } {
        true
    } else {
        let a2589 = v3;
        let a2590 = 0u64;
        nat_eq(a2589, a2590)
    }
}

pub fn f124(v0: u64) -> u64 {
    let a2591 = v0;
    let a2592 = 2147483648u64;
    if nat_lt(a2591, a2592) {
        v0
    } else {
        0u64
    }
}

pub fn f125(v0: u64) -> u64 {
    let a2593 = v0;
    let a2594 = 2147483648u64;
    if nat_lt(a2593, a2594) {
        0u64
    } else {
        let a2595 = 4294967296u64;
        let a2596 = v0;
        nat_sub(a2595, a2596)
    }
}

pub fn f126(v0: Adt3, v1: bool) -> R<u64> {
    f156(v0.clone(), {
        let a2598 = match v0.clone() {
            Adt3::C0(h2597, _, _, _, _, _, _, _) => {
                h2597
            }
        };
        let a2599 = 1u64;
        nat_sub(a2598, a2599)
    }, {
        let a2601 = match v0.clone() {
            Adt3::C0(_, h2600, _, _, _, _, _, _) => {
                h2600
            }
        };
        let a2602 = 1u64;
        nat_sub(a2601, a2602)
    }, {
        let a2604 = match v0.clone() {
            Adt3::C0(_, _, h2603, _, _, _, _, _) => {
                h2603
            }
        };
        let a2605 = 1u64;
        nat_sub(a2604, a2605)
    }, {
        let a2607 = match v0 {
            Adt3::C0(_, _, _, h2606, _, _, _, _) => {
                h2606
            }
        };
        let a2608 = 1u64;
        nat_sub(a2607, a2608)
    }, v1)
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2609 = v0;
    let a2610 = 0u64;
    if nat_eq(a2609, a2610) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2611 = v0;
            let a2612 = 1u64;
            nat_sub(a2611, a2612)
        };
        let v5: u64 = {
            let a2613 = v4;
            let a2614 = v1;
            let a2615 = 0u64;
            nat_quot(a2613, a2614, a2615)
        };
        let v6: u64 = {
            let a2623 = {
                let a2616 = v5;
                let a2617 = v2;
                nat_mul(a2616, a2617)?
            };
            let a2624 = {
                let a2621 = {
                    let a2618 = v4;
                    let a2619 = v1;
                    let a2620 = 0u64;
                    nat_rem(a2618, a2619, a2620)
                };
                let a2622 = v3;
                nat_mul(a2621, a2622)?
            };
            nat_add(a2623, a2624)?
        };
        let a2625 = v5;
        let a2626 = 0u64;
        if nat_eq(a2625, a2626) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2635 = {
                    let a2629 = {
                        let a2627 = v5;
                        let a2628 = 1u64;
                        nat_sub(a2627, a2628)
                    };
                    let a2630 = v2;
                    nat_mul(a2629, a2630)?
                };
                let a2636 = {
                    let a2633 = {
                        let a2631 = v1;
                        let a2632 = 1u64;
                        nat_sub(a2631, a2632)
                    };
                    let a2634 = v3;
                    nat_mul(a2633, a2634)?
                };
                nat_add(a2635, a2636)?
            };
            let a2637 = v6;
            let a2638 = v7;
            if nat_lt(a2637, a2638) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f128(v0: Adt3, v1: u64) -> bool {
    let a2639 = v1;
    let a2640 = 0u64;
    if !nat_eq(a2639, a2640) {
        true
    } else {
        let a2642 = match v0 {
            Adt3::C0(_, _, _, _, h2641, _, _, _) => {
                h2641
            }
        };
        let a2643 = 4u64;
        nat_eq(a2642, a2643)
    }
}

pub fn f129(v0: u64, v1: bool) -> R<u64> {
    let a2646 = {
        let a2644 = v0;
        let a2645 = 2u64;
        nat_mul(a2644, a2645)?
    };
    let a2647 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2646, a2647)
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2648 = f157(v0, v1);
    match m2648 {
        None => {
            if v5 {
                let m2649 = f158(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2649 {
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
                let a2650 = 4u64;
                let a2651 = v28;
                nat_le(a2650, a2651)
            };
            let a2652 = v2;
            let a2653 = 0u64;
            if if if if if nat_eq(a2652, a2653) {
                let a2654 = v4;
                let a2655 = 0u64;
                nat_eq(a2654, a2655)
            } else {
                false
            } {
                let a2656 = v6;
                let a2657 = v14;
                if nat_eq(a2656, a2657) {
                    let a2658 = v7;
                    let a2659 = v15;
                    nat_eq(a2658, a2659)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2660 = v8;
                let a2661 = v16;
                if nat_eq(a2660, a2661) {
                    let a2662 = v9;
                    let a2663 = v17;
                    nat_eq(a2662, a2663)
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
                        let a2664 = v3;
                        let a2665 = 0u64;
                        nat_eq(a2664, a2665)
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
                    let a2670 = {
                        let a2666 = v6;
                        let a2667 = v7;
                        nat_mul(a2666, a2667)?
                    };
                    let a2671 = {
                        let a2668 = v8;
                        let a2669 = v9;
                        nat_mul(a2668, a2669)?
                    };
                    nat_mul(a2670, a2671)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f131(v0: u64) -> u64 {
    let m2672 = f75(v0);
    match m2672 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2674 = match v1.clone() {
                Adt11::C0(_, h2673, _) => {
                    h2673
                }
            };
            let a2675 = 1u64;
            if if nat_eq(a2674, a2675) {
                let a2677 = match v1.clone() {
                    Adt11::C0(_, _, h2676) => {
                        h2676
                    }
                };
                let a2678 = 1u64;
                if if if nat_eq(a2677, a2678) {
                    true
                } else {
                    let a2680 = match v1.clone() {
                        Adt11::C0(_, _, h2679) => {
                            h2679
                        }
                    };
                    let a2681 = 2u64;
                    nat_eq(a2680, a2681)
                } {
                    true
                } else {
                    let a2683 = match v1.clone() {
                        Adt11::C0(_, _, h2682) => {
                            h2682
                        }
                    };
                    let a2684 = 4u64;
                    nat_eq(a2683, a2684)
                } {
                    true
                } else {
                    let a2686 = match v1.clone() {
                        Adt11::C0(_, _, h2685) => {
                            h2685
                        }
                    };
                    let a2687 = 8u64;
                    nat_eq(a2686, a2687)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2688) => {
                        h2688
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2701 = {
        let a2697 = {
            let a2693 = {
                let a2689 = v0;
                let a2690 = v4;
                nat_mul(a2689, a2690)?
            };
            let a2694 = {
                let a2691 = v1;
                let a2692 = v5;
                nat_mul(a2691, a2692)?
            };
            nat_add(a2693, a2694)?
        };
        let a2698 = {
            let a2695 = v2;
            let a2696 = v6;
            nat_mul(a2695, a2696)?
        };
        nat_add(a2697, a2698)?
    };
    let a2702 = {
        let a2699 = v3;
        let a2700 = v7;
        nat_mul(a2699, a2700)?
    };
    nat_add(a2701, a2702)
}

pub fn f133(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2707 = {
        let a2705 = {
            let a2703 = v0;
            let a2704 = v1;
            nat_sub(a2703, a2704)
        };
        let a2706 = 1u64;
        nat_sub(a2705, a2706)
    };
    let a2708 = v2;
    nat_add(a2707, a2708)
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f132({
        let a2709 = v0;
        let a2710 = v4;
        let a2711 = 0u64;
        nat_rem(a2709, a2710, a2711)
    }, {
        let a2712 = v1;
        let a2713 = v5;
        let a2714 = 0u64;
        nat_rem(a2712, a2713, a2714)
    }, {
        let a2715 = v2;
        let a2716 = v6;
        let a2717 = 0u64;
        nat_rem(a2715, a2716, a2717)
    }, {
        let a2718 = v3;
        let a2719 = v7;
        let a2720 = 0u64;
        nat_rem(a2718, a2719, a2720)
    }, v8, v9, v10, v11)
}

pub fn f135(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2722 = v0;
        let a2723 = match v1.clone() {
            Adt3::C0(h2721, _, _, _, _, _, _, _) => {
                h2721
            }
        };
        let a2724 = 0u64;
        nat_quot(a2722, a2723, a2724)
    };
    let v4: u64 = {
        let a2726 = v3;
        let a2727 = match v1.clone() {
            Adt3::C0(_, h2725, _, _, _, _, _, _) => {
                h2725
            }
        };
        let a2728 = 0u64;
        nat_quot(a2726, a2727, a2728)
    };
    let a2729 = v2;
    let a2730 = 0u64;
    if nat_eq(a2729, a2730) {
        let a2732 = v0;
        let a2733 = match v1 {
            Adt3::C0(h2731, _, _, _, _, _, _, _) => {
                h2731
            }
        };
        let a2734 = 0u64;
        nat_rem(a2732, a2733, a2734)
    } else {
        let a2735 = v2;
        let a2736 = 1u64;
        if nat_eq(a2735, a2736) {
            let a2738 = v3;
            let a2739 = match v1 {
                Adt3::C0(_, h2737, _, _, _, _, _, _) => {
                    h2737
                }
            };
            let a2740 = 0u64;
            nat_rem(a2738, a2739, a2740)
        } else {
            let a2741 = v2;
            let a2742 = 2u64;
            if nat_eq(a2741, a2742) {
                let a2744 = v4;
                let a2745 = match v1 {
                    Adt3::C0(_, _, h2743, _, _, _, _, _) => {
                        h2743
                    }
                };
                let a2746 = 0u64;
                nat_rem(a2744, a2745, a2746)
            } else {
                let a2748 = v4;
                let a2749 = match v1 {
                    Adt3::C0(_, _, h2747, _, _, _, _, _) => {
                        h2747
                    }
                };
                let a2750 = 0u64;
                nat_quot(a2748, a2749, a2750)
            }
        }
    }
}

pub fn f136(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f132(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2751, _, _, _) => {
            h2751
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2752, _, _) => {
            h2752
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2753, _) => {
            h2753
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2754) => {
            h2754
        }
    })
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2755 = v1;
    let a2756 = 2147483648u64;
    if nat_lt(a2755, a2756) {
        let a2767 = {
            let a2765 = {
                let a2760 = {
                    let a2757 = v0;
                    let a2758 = v2;
                    let a2759 = 0u64;
                    nat_rem(a2757, a2758, a2759)
                };
                let a2761 = v2;
                nat_add(a2760, a2761)?
            };
            let a2766 = {
                let a2762 = v1;
                let a2763 = v2;
                let a2764 = 0u64;
                nat_rem(a2762, a2763, a2764)
            };
            nat_sub(a2765, a2766)
        };
        let a2768 = v2;
        let a2769 = 0u64;
        Ok(nat_rem(a2767, a2768, a2769))
    } else {
        let a2772 = {
            let a2770 = v0;
            let a2771 = f125(v1);
            nat_add(a2770, a2771)?
        };
        let a2773 = v2;
        let a2774 = 0u64;
        Ok(nat_rem(a2772, a2773, a2774))
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2777 = f124(v1);
    let a2778 = {
        let a2775 = v0;
        let a2776 = f125(v1);
        nat_add(a2775, a2776)?
    };
    if nat_le(a2777, a2778) {
        let a2779 = f139(v0, v1)?;
        let a2780 = v2;
        Ok(nat_lt(a2779, a2780))
    } else {
        Ok(false)
    }
}

pub fn f139(v0: u64, v1: u64) -> R<u64> {
    let a2783 = {
        let a2781 = v0;
        let a2782 = f125(v1);
        nat_add(a2781, a2782)?
    };
    let a2784 = f124(v1);
    Ok(nat_sub(a2783, a2784))
}

pub fn f140(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2785 = v0;
    let a2786 = v1;
    if nat_lt(a2785, a2786) {
        let a2787 = v1;
        let a2788 = v0;
        Ok(nat_sub(a2787, a2788))
    } else {
        let a2791 = v0;
        let a2792 = {
            let a2789 = v1;
            let a2790 = v2;
            nat_add(a2789, a2790)?
        };
        if nat_lt(a2791, a2792) {
            let a2793 = v0;
            let a2794 = v1;
            Ok(nat_sub(a2793, a2794))
        } else {
            let a2801 = {
                let a2797 = 2u64;
                let a2798 = {
                    let a2795 = v2;
                    let a2796 = 1u64;
                    nat_sub(a2795, a2796)
                };
                nat_mul(a2797, a2798)?
            };
            let a2802 = {
                let a2799 = v0;
                let a2800 = v1;
                nat_sub(a2799, a2800)
            };
            Ok(nat_sub(a2801, a2802))
        }
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2805 = {
        let a2803 = v2;
        let a2804 = f156(v1.clone(), f135(v0, v1.clone(), 0u64), f135(v0, v1.clone(), 1u64), f135(v0, v1.clone(), 2u64), f135(v0, v1.clone(), 3u64), false)?;
        nat_add(a2803, a2804)?
    };
    let a2806 = f156(v1.clone(), f135(v0, v1.clone(), 0u64), f135(v0, v1.clone(), 1u64), f135(v0, v1.clone(), 2u64), f135(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2805, a2806))
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> bool {
    let a2807 = v0;
    let a2808 = 0u64;
    if nat_eq(a2807, a2808) {
        let a2809 = v2;
        let a2810 = v1;
        nat_le(a2809, a2810)
    } else {
        let a2811 = v0;
        let a2812 = 1u64;
        if nat_eq(a2811, a2812) {
            let a2813 = v2;
            let a2814 = v1;
            nat_lt(a2813, a2814)
        } else {
            let a2815 = v0;
            let a2816 = 2u64;
            if nat_eq(a2815, a2816) {
                let a2817 = v1;
                let a2818 = v2;
                nat_le(a2817, a2818)
            } else {
                let a2819 = v0;
                let a2820 = 3u64;
                if nat_eq(a2819, a2820) {
                    let a2821 = v1;
                    let a2822 = v2;
                    nat_lt(a2821, a2822)
                } else {
                    let a2823 = v1;
                    let a2824 = v2;
                    nat_eq(a2823, a2824)
                }
            }
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2825 = v0;
    let a2826 = 0u64;
    if nat_eq(a2825, a2826) {
        Ok(f151(0u64))
    } else {
        let a2829 = {
            let a2827 = v0;
            let a2828 = 1u64;
            nat_add(a2827, a2828)?
        };
        let a2830 = f87(v1.clone(), 13u64, v2)?;
        if nat_eq(a2829, a2830) {
            Ok(Adt5::C4({
                let a2831 = v2;
                let a2832 = 0u64;
                if nat_eq(a2831, a2832) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2833 = v0;
                let a2834 = 1u64;
                nat_sub(a2833, a2834)
            };
            if f128(v1.clone(), v2) {
                let v4: u64 = f129(match v1.clone() {
                    Adt3::C0(h2835, _, _, _, _, _, _, _) => {
                        h2835
                    }
                }, false)?;
                let v5: u64 = {
                    let a2836 = v3;
                    let a2837 = v4;
                    let a2838 = 0u64;
                    nat_quot(a2836, a2837, a2838)
                };
                let v6: u64 = {
                    let a2839 = v3;
                    let a2840 = v4;
                    let a2841 = 0u64;
                    nat_rem(a2839, a2840, a2841)
                };
                let a2844 = {
                    let a2842 = v6;
                    let a2843 = 1u64;
                    nat_add(a2842, a2843)?
                };
                let a2845 = v4;
                if nat_eq(a2844, a2845) {
                    Ok(Adt5::C4({
                        let a2846 = v2;
                        let a2847 = 0u64;
                        if nat_eq(a2846, a2847) {
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
                    let a2848 = v3;
                    let a2849 = 2u64;
                    let a2850 = 0u64;
                    nat_quot(a2848, a2849, a2850)
                }, {
                    let a2851 = v3;
                    let a2852 = 2u64;
                    let a2853 = 0u64;
                    nat_rem(a2851, a2852, a2853)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f144(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2854 = v0;
    let a2855 = 0u64;
    if nat_eq(a2854, a2855) {
        Ok(f151(1u64))
    } else {
        f159({
            let a2864 = {
                let a2857 = v1;
                let a2858 = match v2.clone() {
                    Adt3::C0(h2856, _, _, _, _, _, _, _) => {
                        h2856
                    }
                };
                nat_mul(a2857, a2858)?
            };
            let a2865 = {
                let a2861 = {
                    let a2859 = v0;
                    let a2860 = 1u64;
                    nat_sub(a2859, a2860)
                };
                let a2862 = 2u64;
                let a2863 = 0u64;
                nat_quot(a2861, a2862, a2863)
            };
            nat_add(a2864, a2865)?
        }, {
            let a2868 = {
                let a2866 = v0;
                let a2867 = 1u64;
                nat_sub(a2866, a2867)
            };
            let a2869 = 2u64;
            let a2870 = 0u64;
            nat_rem(a2868, a2869, a2870)
        }, v2, v3, 1u64)
    }
}

pub fn f145(v0: u64, v1: u64) -> R<u64> {
    let a2871 = v0;
    let a2872 = f64(v1)?;
    let a2873 = 0u64;
    Ok(nat_rem(a2871, a2872, a2873))
}

pub fn f146(v0: u64, v1: u64) -> R<u64> {
    let a2874 = 4u64;
    let a2875 = f160(v0, v1)?;
    nat_mul(a2874, a2875)
}

pub fn f147(v0: u64, v1: u64) -> R<u64> {
    let a2876 = 4u64;
    let a2877 = f161(v0, v1)?;
    nat_mul(a2876, a2877)
}

pub fn f148(v0: u64, v1: u64) -> R<bool> {
    let a2883 = {
        let a2880 = {
            let a2878 = f145(v0, v1)?;
            let a2879 = 2u64;
            nat_sub(a2878, a2879)
        };
        let a2881 = 2u64;
        let a2882 = 0u64;
        nat_rem(a2880, a2881, a2882)
    };
    let a2884 = 0u64;
    Ok(nat_eq(a2883, a2884))
}

pub fn f149(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2885 = f75(v0);
    match m2885 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f162(v0) {
                let a2886 = v1;
                let a2887 = v7;
                nat_lt(a2886, a2887)
            } else {
                false
            } {
                f111(v8, v2)
            } else {
                false
            } {
                let a2888 = v3;
                let a2889 = f163(v4, v5, v6)?;
                nat_lt(a2888, a2889)
            } else {
                false
            } {
                let a2894 = {
                    let a2891 = v7;
                    let a2892 = match v13.clone() {
                        Adt11::C0(_, h2890, _) => {
                            h2890
                        }
                    };
                    let a2893 = 0u64;
                    nat_rem(a2891, a2892, a2893)
                };
                let a2895 = 0u64;
                nat_eq(a2894, a2895)
            } else {
                false
            } {
                Ok(Some({
                    let a2924 = {
                        let a2914 = {
                            let a2904 = {
                                let a2900 = {
                                    let a2897 = v1;
                                    let a2898 = match v13 {
                                        Adt11::C0(_, h2896, _) => {
                                            h2896
                                        }
                                    };
                                    let a2899 = 0u64;
                                    nat_quot(a2897, a2898, a2899)
                                };
                                let a2901 = v9;
                                nat_mul(a2900, a2901)?
                            };
                            let a2905 = {
                                let a2902 = v2;
                                let a2903 = v10;
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
                                nat_rem(a2909, a2910, a2911)
                            };
                            let a2913 = v11;
                            nat_mul(a2912, a2913)?
                        };
                        nat_add(a2914, a2915)?
                    };
                    let a2925 = {
                        let a2922 = {
                            let a2919 = {
                                let a2916 = v3;
                                let a2917 = v4;
                                let a2918 = 0u64;
                                nat_quot(a2916, a2917, a2918)
                            };
                            let a2920 = v5;
                            let a2921 = 0u64;
                            nat_quot(a2919, a2920, a2921)
                        };
                        let a2923 = v12;
                        nat_mul(a2922, a2923)?
                    };
                    nat_add(a2924, a2925)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2926 = v0;
    let a2927 = 30u64;
    if nat_eq(a2926, a2927) {
        4u64
    } else {
        3u64
    }
}

pub fn f151(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f152(v0: u64) -> bool {
    let a2928 = 0u64;
    let a2929 = v0;
    if nat_lt(a2928, a2929) {
        let a2930 = v0;
        let a2931 = 2139095040u64;
        nat_le(a2930, a2931)
    } else {
        false
    }
}

pub fn f153(v0: u64) -> u64 {
    let a2932 = v0;
    let a2933 = 40u64;
    if nat_lt(a2932, a2933) {
        let a2934 = v0;
        let a2935 = 20u64;
        if nat_lt(a2934, a2935) {
            let a2936 = v0;
            let a2937 = 10u64;
            if nat_lt(a2936, a2937) {
                let a2938 = v0;
                let a2939 = 5u64;
                if nat_lt(a2938, a2939) {
                    let a2940 = v0;
                    let a2941 = 2u64;
                    if nat_lt(a2940, a2941) {
                        let a2942 = v0;
                        let a2943 = 1u64;
                        if nat_lt(a2942, a2943) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2944 = v0;
                        let a2945 = 3u64;
                        if nat_lt(a2944, a2945) {
                            4u64
                        } else {
                            let a2946 = v0;
                            let a2947 = 4u64;
                            if nat_lt(a2946, a2947) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2948 = v0;
                    let a2949 = 7u64;
                    if nat_lt(a2948, a2949) {
                        let a2950 = v0;
                        let a2951 = 6u64;
                        if nat_lt(a2950, a2951) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2952 = v0;
                        let a2953 = 8u64;
                        if nat_lt(a2952, a2953) {
                            128u64
                        } else {
                            let a2954 = v0;
                            let a2955 = 9u64;
                            if nat_lt(a2954, a2955) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2956 = v0;
                let a2957 = 15u64;
                if nat_lt(a2956, a2957) {
                    let a2958 = v0;
                    let a2959 = 12u64;
                    if nat_lt(a2958, a2959) {
                        let a2960 = v0;
                        let a2961 = 11u64;
                        if nat_lt(a2960, a2961) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2962 = v0;
                        let a2963 = 13u64;
                        if nat_lt(a2962, a2963) {
                            4096u64
                        } else {
                            let a2964 = v0;
                            let a2965 = 14u64;
                            if nat_lt(a2964, a2965) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2966 = v0;
                    let a2967 = 17u64;
                    if nat_lt(a2966, a2967) {
                        let a2968 = v0;
                        let a2969 = 16u64;
                        if nat_lt(a2968, a2969) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2970 = v0;
                        let a2971 = 18u64;
                        if nat_lt(a2970, a2971) {
                            131072u64
                        } else {
                            let a2972 = v0;
                            let a2973 = 19u64;
                            if nat_lt(a2972, a2973) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2974 = v0;
            let a2975 = 30u64;
            if nat_lt(a2974, a2975) {
                let a2976 = v0;
                let a2977 = 25u64;
                if nat_lt(a2976, a2977) {
                    let a2978 = v0;
                    let a2979 = 22u64;
                    if nat_lt(a2978, a2979) {
                        let a2980 = v0;
                        let a2981 = 21u64;
                        if nat_lt(a2980, a2981) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2982 = v0;
                        let a2983 = 23u64;
                        if nat_lt(a2982, a2983) {
                            4194304u64
                        } else {
                            let a2984 = v0;
                            let a2985 = 24u64;
                            if nat_lt(a2984, a2985) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2986 = v0;
                    let a2987 = 27u64;
                    if nat_lt(a2986, a2987) {
                        let a2988 = v0;
                        let a2989 = 26u64;
                        if nat_lt(a2988, a2989) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2990 = v0;
                        let a2991 = 28u64;
                        if nat_lt(a2990, a2991) {
                            134217728u64
                        } else {
                            let a2992 = v0;
                            let a2993 = 29u64;
                            if nat_lt(a2992, a2993) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2994 = v0;
                let a2995 = 35u64;
                if nat_lt(a2994, a2995) {
                    let a2996 = v0;
                    let a2997 = 32u64;
                    if nat_lt(a2996, a2997) {
                        let a2998 = v0;
                        let a2999 = 31u64;
                        if nat_lt(a2998, a2999) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3000 = v0;
                        let a3001 = 33u64;
                        if nat_lt(a3000, a3001) {
                            4294967296u64
                        } else {
                            let a3002 = v0;
                            let a3003 = 34u64;
                            if nat_lt(a3002, a3003) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3004 = v0;
                    let a3005 = 37u64;
                    if nat_lt(a3004, a3005) {
                        let a3006 = v0;
                        let a3007 = 36u64;
                        if nat_lt(a3006, a3007) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3008 = v0;
                        let a3009 = 38u64;
                        if nat_lt(a3008, a3009) {
                            137438953472u64
                        } else {
                            let a3010 = v0;
                            let a3011 = 39u64;
                            if nat_lt(a3010, a3011) {
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
    let a3012 = v0;
    let a3013 = 26u64;
    if nat_eq(a3012, a3013) {
        26u64
    } else {
        0u64
    }
}

pub fn f156(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3034 = {
        let a3028 = {
            let a3022 = {
                let a3016 = v1;
                let a3017 = if v5 {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3014, _, _, _) => {
                            h3014
                        }
                    })
                } else {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3015, _, _, _) => {
                            h3015
                        }
                    })
                };
                nat_mul(a3016, a3017)?
            };
            let a3023 = {
                let a3020 = v2;
                let a3021 = if v5 {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3018, _, _) => {
                            h3018
                        }
                    })
                } else {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3019, _, _) => {
                            h3019
                        }
                    })
                };
                nat_mul(a3020, a3021)?
            };
            nat_add(a3022, a3023)?
        };
        let a3029 = {
            let a3026 = v3;
            let a3027 = if v5 {
                f125(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3024, _) => {
                        h3024
                    }
                })
            } else {
                f124(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3025, _) => {
                        h3025
                    }
                })
            };
            nat_mul(a3026, a3027)?
        };
        nat_add(a3028, a3029)?
    };
    let a3035 = {
        let a3032 = v4;
        let a3033 = if v5 {
            f125(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3030) => {
                    h3030
                }
            })
        } else {
            f124(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3031) => {
                    h3031
                }
            })
        };
        nat_mul(a3032, a3033)?
    };
    nat_add(a3034, a3035)
}

pub fn f157(v0: u64, v1: u64) -> Option<u64> {
    let a3036 = v0;
    let a3037 = 2u64;
    if nat_eq(a3036, a3037) {
        Some(0u64)
    } else {
        let a3038 = v0;
        let a3039 = 5u64;
        if nat_eq(a3038, a3039) {
            Some(1u64)
        } else {
            let a3040 = v0;
            let a3041 = 6u64;
            if nat_eq(a3040, a3041) {
                Some(2u64)
            } else {
                let a3042 = v0;
                let a3043 = 7u64;
                if nat_eq(a3042, a3043) {
                    Some(3u64)
                } else {
                    let a3044 = v0;
                    let a3045 = 9u64;
                    if nat_eq(a3044, a3045) {
                        Some(4u64)
                    } else {
                        let a3046 = v0;
                        let a3047 = 8u64;
                        if nat_eq(a3046, a3047) {
                            Some(5u64)
                        } else {
                            let a3048 = v0;
                            let a3049 = 89u64;
                            if nat_eq(a3048, a3049) {
                                let a3050 = v1;
                                let a3051 = 2u64;
                                if nat_eq(a3050, a3051) {
                                    Some(6u64)
                                } else {
                                    let a3052 = v1;
                                    let a3053 = 0u64;
                                    if nat_eq(a3052, a3053) {
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
    let a3054 = v0;
    let a3055 = 30u64;
    if nat_eq(a3054, a3055) {
        let a3056 = v2;
        let a3057 = 0u64;
        if if nat_eq(a3056, a3057) {
            let a3058 = v1;
            let a3059 = 0u64;
            if nat_eq(a3058, a3059) {
                true
            } else {
                let a3060 = v1;
                let a3061 = 1u64;
                if if nat_eq(a3060, a3061) {
                    let a3062 = v6;
                    let a3063 = v10;
                    nat_eq(a3062, a3063)
                } else {
                    false
                } {
                    let a3064 = v7;
                    let a3065 = v11;
                    nat_eq(a3064, a3065)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            f165(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt12::C0)
        }
    } else {
        if if if f166(v1) {
            f166(v2)
        } else {
            false
        } {
            let a3066 = v1;
            let a3067 = 0u64;
            if if nat_eq(a3066, a3067) {
                true
            } else {
                let a3068 = v2;
                let a3069 = 0u64;
                nat_eq(a3068, a3069)
            } {
                true
            } else {
                let a3070 = v1;
                let a3071 = v2;
                nat_eq(a3070, a3071)
            }
        } else {
            false
        } {
            f165(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt12::C0)
        }
    }
}

pub fn f159(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3072 = v1;
    let a3073 = 0u64;
    if nat_eq(a3072, a3073) {
        Ok(Adt5::C4({
            let a3074 = v3;
            let a3075 = 0u64;
            if nat_eq(a3074, a3075) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f94(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3076 = v3;
            let a3077 = 0u64;
            if nat_eq(a3076, a3077) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f160(v0: u64, v1: u64) -> R<u64> {
    let a3078 = v0;
    let a3079 = f64(v1)?;
    let a3080 = 0u64;
    Ok(nat_quot(a3078, a3079, a3080))
}

pub fn f161(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f160(v0, v1)?;
    let v3: u64 = {
        let a3083 = {
            let a3081 = f145(v0, v1)?;
            let a3082 = 2u64;
            nat_sub(a3081, a3082)
        };
        let a3084 = 2u64;
        let a3085 = 0u64;
        nat_quot(a3083, a3084, a3085)
    };
    let v4: u64 = {
        let a3086 = v2;
        let a3087 = v1;
        nat_mul(a3086, a3087)?
    };
    if f148(v0, v1)? {
        let a3088 = v4;
        let a3089 = v3;
        nat_add(a3088, a3089)
    } else {
        let a3101 = {
            let a3097 = v4;
            let a3098 = {
                let a3095 = {
                    let a3090 = v1;
                    let a3091 = 1u64;
                    nat_sub(a3090, a3091)
                };
                let a3096 = {
                    let a3092 = v2;
                    let a3093 = v1;
                    let a3094 = 0u64;
                    nat_rem(a3092, a3093, a3094)
                };
                nat_mul(a3095, a3096)?
            };
            nat_sub(a3097, a3098)
        };
        let a3102 = {
            let a3099 = v3;
            let a3100 = v1;
            nat_mul(a3099, a3100)?
        };
        nat_add(a3101, a3102)
    }
}

pub fn f162(v0: u64) -> bool {
    let a3103 = v0;
    let a3104 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3103, a3104) {
        true
    } else {
        let a3105 = v0;
        let a3106 = 1u64;
        nat_eq(a3105, a3106)
    } {
        true
    } else {
        let a3107 = v0;
        let a3108 = 2u64;
        nat_eq(a3107, a3108)
    } {
        true
    } else {
        let a3109 = v0;
        let a3110 = 3u64;
        nat_eq(a3109, a3110)
    } {
        true
    } else {
        let a3111 = v0;
        let a3112 = 6u64;
        nat_eq(a3111, a3112)
    } {
        true
    } else {
        let a3113 = v0;
        let a3114 = 7u64;
        nat_eq(a3113, a3114)
    } {
        true
    } else {
        let a3115 = v0;
        let a3116 = 8u64;
        nat_eq(a3115, a3116)
    } {
        true
    } else {
        let a3117 = v0;
        let a3118 = 10u64;
        nat_eq(a3117, a3118)
    } {
        true
    } else {
        let a3119 = v0;
        let a3120 = 11u64;
        nat_eq(a3119, a3120)
    } {
        true
    } else {
        let a3121 = v0;
        let a3122 = 12u64;
        nat_eq(a3121, a3122)
    } {
        true
    } else {
        let a3123 = v0;
        let a3124 = 13u64;
        nat_eq(a3123, a3124)
    } {
        true
    } else {
        let a3125 = v0;
        let a3126 = 14u64;
        nat_eq(a3125, a3126)
    } {
        true
    } else {
        let a3127 = v0;
        let a3128 = 16u64;
        nat_eq(a3127, a3128)
    } {
        true
    } else {
        let a3129 = v0;
        let a3130 = 17u64;
        nat_eq(a3129, a3130)
    } {
        true
    } else {
        let a3131 = v0;
        let a3132 = 18u64;
        nat_eq(a3131, a3132)
    } {
        true
    } else {
        let a3133 = v0;
        let a3134 = 19u64;
        nat_eq(a3133, a3134)
    } {
        true
    } else {
        let a3135 = v0;
        let a3136 = 20u64;
        nat_eq(a3135, a3136)
    } {
        true
    } else {
        let a3137 = v0;
        let a3138 = 21u64;
        nat_eq(a3137, a3138)
    } {
        true
    } else {
        let a3139 = v0;
        let a3140 = 22u64;
        nat_eq(a3139, a3140)
    } {
        true
    } else {
        let a3141 = v0;
        let a3142 = 23u64;
        nat_eq(a3141, a3142)
    } {
        true
    } else {
        let a3143 = v0;
        let a3144 = 26u64;
        nat_eq(a3143, a3144)
    } {
        true
    } else {
        let a3145 = v0;
        let a3146 = 29u64;
        nat_eq(a3145, a3146)
    } {
        true
    } else {
        let a3147 = v0;
        let a3148 = 30u64;
        nat_eq(a3147, a3148)
    } {
        true
    } else {
        let a3149 = v0;
        let a3150 = 34u64;
        nat_eq(a3149, a3150)
    } {
        true
    } else {
        let a3151 = v0;
        let a3152 = 35u64;
        nat_eq(a3151, a3152)
    } {
        true
    } else {
        let a3153 = v0;
        let a3154 = 39u64;
        nat_eq(a3153, a3154)
    } {
        true
    } else {
        let a3155 = v0;
        let a3156 = 40u64;
        nat_eq(a3155, a3156)
    } {
        true
    } else {
        let a3157 = v0;
        let a3158 = 41u64;
        nat_eq(a3157, a3158)
    } {
        true
    } else {
        let a3159 = v0;
        let a3160 = 42u64;
        nat_eq(a3159, a3160)
    } {
        true
    } else {
        let a3161 = v0;
        let a3162 = 43u64;
        nat_eq(a3161, a3162)
    }
}

pub fn f163(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3163 = v0;
    let a3164 = 0u64;
    if if if nat_eq(a3163, a3164) {
        true
    } else {
        let a3165 = v1;
        let a3166 = 0u64;
        nat_eq(a3165, a3166)
    } {
        true
    } else {
        let a3167 = v2;
        let a3168 = 0u64;
        nat_eq(a3167, a3168)
    } {
        Ok(0u64)
    } else {
        let a3171 = {
            let a3169 = v0;
            let a3170 = v1;
            nat_mul(a3169, a3170)?
        };
        let a3172 = v2;
        nat_mul(a3171, a3172)
    }
}

pub fn f164(v0: u64) -> bool {
    let m3173 = f75(v0);
    match m3173 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a3174 = v0;
    let a3175 = 28u64;
    if if if if nat_eq(a3174, a3175) {
        let a3176 = v1;
        let a3177 = 0u64;
        nat_eq(a3176, a3177)
    } else {
        false
    } {
        let a3178 = v2;
        let a3179 = 0u64;
        nat_eq(a3178, a3179)
    } else {
        false
    } {
        let a3180 = v3;
        let a3181 = 0u64;
        nat_eq(a3180, a3181)
    } else {
        false
    } {
        let a3182 = v4;
        let a3183 = v8;
        if if if if if if if if nat_eq(a3182, a3183) {
            let a3184 = v5;
            let a3185 = v12;
            nat_eq(a3184, a3185)
        } else {
            false
        } {
            let a3186 = v9;
            let a3187 = v13;
            nat_eq(a3186, a3187)
        } else {
            false
        } {
            let a3188 = v10;
            let a3189 = v14;
            nat_eq(a3188, a3189)
        } else {
            false
        } {
            let a3190 = v11;
            let a3191 = v15;
            nat_eq(a3190, a3191)
        } else {
            false
        } {
            let a3192 = 0u64;
            let a3193 = v6;
            nat_lt(a3192, a3193)
        } else {
            false
        } {
            let a3194 = 0u64;
            let a3195 = v7;
            nat_lt(a3194, a3195)
        } else {
            false
        } {
            let a3199 = {
                let a3196 = v10;
                let a3197 = v6;
                let a3198 = 1u64;
                nat_rem(a3196, a3197, a3198)
            };
            let a3200 = 0u64;
            if nat_eq(a3199, a3200) {
                let a3204 = {
                    let a3201 = v11;
                    let a3202 = v7;
                    let a3203 = 1u64;
                    nat_rem(a3201, a3202, a3203)
                };
                let a3205 = 0u64;
                nat_eq(a3204, a3205)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3206 = v10;
                let a3207 = v6;
                let a3208 = 0u64;
                nat_quot(a3206, a3207, a3208)
            }, {
                let a3209 = v11;
                let a3210 = v7;
                let a3211 = 0u64;
                nat_quot(a3209, a3210, a3211)
            }, {
                let a3212 = v5;
                let a3213 = v4;
                nat_mul(a3212, a3213)?
            }, {
                let a3214 = v4;
                let a3215 = v9;
                nat_mul(a3214, a3215)?
            }, {
                let a3216 = v5;
                let a3217 = v9;
                nat_mul(a3216, a3217)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f166(v0: u64) -> bool {
    let a3218 = v0;
    let a3219 = 0u64;
    if if nat_eq(a3218, a3219) {
        true
    } else {
        let a3220 = v0;
        let a3221 = 1u64;
        nat_eq(a3220, a3221)
    } {
        true
    } else {
        let a3222 = v0;
        let a3223 = 30u64;
        nat_eq(a3222, a3223)
    }
}
