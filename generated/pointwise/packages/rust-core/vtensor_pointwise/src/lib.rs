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
                                                                                let m258 = f48(v0, v5);
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

pub fn f48(v0: u64, v1: u64) -> Option<u64> {
    let a1200 = v0;
    let a1201 = 8u64;
    if nat_eq(a1200, a1201) {
        Some(5u64)
    } else {
        let a1202 = v0;
        let a1203 = 9u64;
        if nat_eq(a1202, a1203) {
            Some(6u64)
        } else {
            let a1204 = v0;
            let a1205 = 89u64;
            if nat_eq(a1204, a1205) {
                let a1206 = v1;
                let a1207 = 0u64;
                if nat_eq(a1206, a1207) {
                    Some(0u64)
                } else {
                    let a1208 = v1;
                    let a1209 = 1u64;
                    if nat_eq(a1208, a1209) {
                        Some(1u64)
                    } else {
                        let a1210 = v1;
                        let a1211 = 2u64;
                        if nat_eq(a1210, a1211) {
                            Some(2u64)
                        } else {
                            let a1212 = v1;
                            let a1213 = 3u64;
                            if nat_eq(a1212, a1213) {
                                Some(3u64)
                            } else {
                                let a1214 = v1;
                                let a1215 = 6u64;
                                if nat_eq(a1214, a1215) {
                                    Some(4u64)
                                } else {
                                    let a1216 = v1;
                                    let a1217 = 12u64;
                                    if nat_eq(a1216, a1217) {
                                        Some(7u64)
                                    } else {
                                        let a1218 = v1;
                                        let a1219 = 11u64;
                                        if nat_eq(a1218, a1219) {
                                            Some(8u64)
                                        } else {
                                            let a1220 = v1;
                                            let a1221 = 18u64;
                                            if nat_eq(a1220, a1221) {
                                                Some(9u64)
                                            } else {
                                                let a1222 = v1;
                                                let a1223 = 19u64;
                                                if nat_eq(a1222, a1223) {
                                                    Some(10u64)
                                                } else {
                                                    let a1224 = v1;
                                                    let a1225 = 20u64;
                                                    if nat_eq(a1224, a1225) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1226 = v1;
                                                        let a1227 = 21u64;
                                                        if nat_eq(a1226, a1227) {
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
    let m1228 = f83(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1228 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                let m1229 = v3;
                match m1229 {
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
    let a1231 = match v1.clone() {
        Adt2::C0(h1230, _, _, _, _, _, _, _, _, _, _) => {
            h1230
        }
    };
    let a1232 = 0u64;
    if if if if if nat_eq(a1231, a1232) {
        true
    } else {
        let a1234 = match v1.clone() {
            Adt2::C0(h1233, _, _, _, _, _, _, _, _, _, _) => {
                h1233
            }
        };
        let a1235 = 1u64;
        nat_eq(a1234, a1235)
    } {
        true
    } else {
        let a1237 = match v1.clone() {
            Adt2::C0(h1236, _, _, _, _, _, _, _, _, _, _) => {
                h1236
            }
        };
        let a1238 = 30u64;
        nat_eq(a1237, a1238)
    } {
        let a1241 = match v1.clone() {
            Adt2::C0(h1239, _, _, _, _, _, _, _, _, _, _) => {
                h1239
            }
        };
        let a1242 = match v2.clone() {
            Adt2::C0(h1240, _, _, _, _, _, _, _, _, _, _) => {
                h1240
            }
        };
        if nat_eq(a1241, a1242) {
            true
        } else {
            let a1244 = match v2.clone() {
                Adt2::C0(h1243, _, _, _, _, _, _, _, _, _, _) => {
                    h1243
                }
            };
            let a1245 = 0u64;
            nat_eq(a1244, a1245)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1247 = match v2.clone() {
                Adt2::C0(h1246, _, _, _, _, _, _, _, _, _, _) => {
                    h1246
                }
            };
            let a1248 = 0u64;
            if nat_eq(a1247, a1248) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1249, _, _, _, _, _, _, _, _, _, _) => {
                h1249
            }
        }, match v2 {
            Adt2::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                h1250
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1251 = v1;
    match m1251 {
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
    let m1252 = v3;
    match m1252 {
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
                Adt3::C0(h1253, _, _, _, _, _, _, _) => {
                    h1253
                }
            }, match v2.clone() {
                Adt3::C0(_, h1254, _, _, _, _, _, _) => {
                    h1254
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
            let a1255 = v51;
            let a1256 = 64u64;
            if if if nat_eq(a1255, a1256) {
                true
            } else {
                let a1257 = v51;
                let a1258 = 65u64;
                nat_eq(a1257, a1258)
            } {
                true
            } else {
                let a1259 = v51;
                let a1260 = 66u64;
                nat_eq(a1259, a1260)
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
        Adt3::C0(h1261, _, _, _, _, _, _, _) => {
            h1261
        }
    })?;
    let v5: u64 = {
        let a1262 = v0;
        let a1263 = v4;
        let a1264 = 0u64;
        nat_quot(a1262, a1263, a1264)
    };
    let v6: u64 = {
        let a1265 = v0;
        let a1266 = v4;
        let a1267 = 0u64;
        nat_rem(a1265, a1266, a1267)
    };
    let a1268 = v6;
    let a1269 = 0u64;
    if nat_eq(a1268, a1269) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1270 = v6;
        let a1271 = 1u64;
        if nat_eq(a1270, a1271) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1274 = {
                let a1272 = v6;
                let a1273 = 1u64;
                nat_add(a1272, a1273)?
            };
            let a1275 = v4;
            if nat_eq(a1274, a1275) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1276 = v6;
                    let a1277 = 2u64;
                    nat_sub(a1276, a1277)
                };
                let v8: u64 = {
                    let a1278 = v7;
                    let a1279 = 3u64;
                    let a1280 = 0u64;
                    nat_quot(a1278, a1279, a1280)
                };
                let v9: u64 = {
                    let a1281 = v7;
                    let a1282 = 3u64;
                    let a1283 = 0u64;
                    nat_rem(a1281, a1282, a1283)
                };
                let v10: u64 = f89({
                    let a1287 = {
                        let a1285 = v5;
                        let a1286 = match v1.clone() {
                            Adt3::C0(h1284, _, _, _, _, _, _, _) => {
                                h1284
                            }
                        };
                        nat_mul(a1285, a1286)?
                    };
                    let a1288 = v8;
                    nat_add(a1287, a1288)?
                }, v1)?;
                let a1289 = v9;
                let a1290 = 0u64;
                if nat_eq(a1289, a1290) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1291 = v9;
                    let a1292 = 1u64;
                    if nat_eq(a1291, a1292) {
                        let a1293 = v3;
                        let a1294 = 2u64;
                        if nat_eq(a1293, a1294) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1295 = v3;
                        let a1296 = 2u64;
                        if if nat_eq(a1295, a1296) {
                            true
                        } else {
                            let a1297 = v3;
                            let a1298 = 3u64;
                            nat_eq(a1297, a1298)
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
        let a1300 = 2u64;
        let a1301 = match v1.clone() {
            Adt3::C0(_, h1299, _, _, _, _, _, _) => {
                h1299
            }
        };
        nat_mul(a1300, a1301)?
    };
    let a1302 = v0;
    let a1303 = v5;
    if nat_lt(a1302, a1303) {
        let a1307 = {
            let a1304 = v0;
            let a1305 = 2u64;
            let a1306 = 0u64;
            nat_rem(a1304, a1305, a1306)
        };
        let a1308 = 0u64;
        if nat_eq(a1307, a1308) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1309 = v0;
                let a1310 = 2u64;
                let a1311 = 0u64;
                nat_quot(a1309, a1310, a1311)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1312 = v4;
            let a1313 = 2147483648u64;
            if if nat_lt(a1312, a1313) {
                let a1315 = v4;
                let a1316 = match v2 {
                    Adt3::C0(_, h1314, _, _, _, _, _, _) => {
                        h1314
                    }
                };
                nat_lt(a1315, a1316)
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
            let a1319 = match v2.clone() {
                Adt3::C0(h1317, _, _, _, _, _, _, _) => {
                    h1317
                }
            };
            let a1320 = match v2.clone() {
                Adt3::C0(_, h1318, _, _, _, _, _, _) => {
                    h1318
                }
            };
            nat_mul(a1319, a1320)?
        };
        let a1323 = {
            let a1321 = v0;
            let a1322 = v5;
            nat_sub(a1321, a1322)
        };
        let a1324 = v6;
        if nat_lt(a1323, a1324) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1325 = v0;
                let a1326 = v5;
                nat_sub(a1325, a1326)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1329 = {
                    let a1327 = v0;
                    let a1328 = v5;
                    nat_sub(a1327, a1328)
                };
                let a1330 = v6;
                nat_sub(a1329, a1330)
            };
            let v8: u64 = {
                let a1334 = 1u64;
                let a1335 = {
                    let a1332 = 2u64;
                    let a1333 = match v1.clone() {
                        Adt3::C0(h1331, _, _, _, _, _, _, _) => {
                            h1331
                        }
                    };
                    nat_mul(a1332, a1333)?
                };
                nat_add(a1334, a1335)?
            };
            let v9: u64 = {
                let a1336 = v7;
                let a1337 = v8;
                let a1338 = 0u64;
                nat_quot(a1336, a1337, a1338)
            };
            let v10: u64 = {
                let a1339 = v7;
                let a1340 = v8;
                let a1341 = 0u64;
                nat_rem(a1339, a1340, a1341)
            };
            let a1342 = v10;
            let a1343 = 0u64;
            if nat_eq(a1342, a1343) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1347 = {
                    let a1344 = v10;
                    let a1345 = 2u64;
                    let a1346 = 0u64;
                    nat_rem(a1344, a1345, a1346)
                };
                let a1348 = 1u64;
                if nat_eq(a1347, a1348) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1357 = {
                            let a1350 = v9;
                            let a1351 = match v1.clone() {
                                Adt3::C0(h1349, _, _, _, _, _, _, _) => {
                                    h1349
                                }
                            };
                            nat_mul(a1350, a1351)?
                        };
                        let a1358 = {
                            let a1354 = {
                                let a1352 = v10;
                                let a1353 = 1u64;
                                nat_sub(a1352, a1353)
                            };
                            let a1355 = 2u64;
                            let a1356 = 0u64;
                            nat_quot(a1354, a1355, a1356)
                        };
                        nat_add(a1357, a1358)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1367 = {
                            let a1360 = v4;
                            let a1361 = match v1.clone() {
                                Adt3::C0(h1359, _, _, _, _, _, _, _) => {
                                    h1359
                                }
                            };
                            nat_mul(a1360, a1361)?
                        };
                        let a1368 = {
                            let a1364 = {
                                let a1362 = v10;
                                let a1363 = 1u64;
                                nat_sub(a1362, a1363)
                            };
                            let a1365 = 2u64;
                            let a1366 = 0u64;
                            nat_quot(a1364, a1365, a1366)
                        };
                        nat_add(a1367, a1368)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1377 = {
                            let a1370 = v4;
                            let a1371 = match v1 {
                                Adt3::C0(h1369, _, _, _, _, _, _, _) => {
                                    h1369
                                }
                            };
                            nat_mul(a1370, a1371)?
                        };
                        let a1378 = {
                            let a1374 = {
                                let a1372 = v10;
                                let a1373 = 1u64;
                                nat_sub(a1372, a1373)
                            };
                            let a1375 = 2u64;
                            let a1376 = 0u64;
                            nat_quot(a1374, a1375, a1376)
                        };
                        nat_add(a1377, a1378)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1379 = v1;
    let a1380 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1379, a1380) {
        let a1383 = v1;
        let a1384 = {
            let a1381 = 2u64;
            let a1382 = f81(v4.clone())?;
            nat_mul(a1381, a1382)?
        };
        if nat_lt(a1383, a1384) {
            let a1388 = {
                let a1385 = v1;
                let a1386 = 2u64;
                let a1387 = 0u64;
                nat_rem(a1385, a1386, a1387)
            };
            let a1389 = 0u64;
            if nat_eq(a1388, a1389) {
                f105({
                    let a1390 = v1;
                    let a1391 = 2u64;
                    let a1392 = 0u64;
                    nat_quot(a1390, a1391, a1392)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1393, _, _, _, _, _, _) => {
                        h1393
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1396 = v1;
                let a1397 = {
                    let a1394 = 2u64;
                    let a1395 = f81(v4.clone())?;
                    nat_mul(a1394, a1395)?
                };
                nat_sub(a1396, a1397)
            };
            let v7: u64 = {
                let a1401 = v6;
                let a1402 = {
                    let a1399 = 1u64;
                    let a1400 = match v2.clone() {
                        Adt3::C0(h1398, _, _, _, _, _, _, _) => {
                            h1398
                        }
                    };
                    nat_add(a1399, a1400)?
                };
                let a1403 = 0u64;
                nat_quot(a1401, a1402, a1403)
            };
            let v8: u64 = {
                let a1407 = v6;
                let a1408 = {
                    let a1405 = 1u64;
                    let a1406 = match v2.clone() {
                        Adt3::C0(h1404, _, _, _, _, _, _, _) => {
                            h1404
                        }
                    };
                    nat_add(a1405, a1406)?
                };
                let a1409 = 0u64;
                nat_rem(a1407, a1408, a1409)
            };
            let a1410 = v8;
            let a1411 = 0u64;
            if nat_eq(a1410, a1411) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1412 = v8;
                    let a1413 = 1u64;
                    nat_sub(a1412, a1413)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1416 = match v4.clone() {
            Adt3::C0(h1414, _, _, _, _, _, _, _) => {
                h1414
            }
        };
        let a1417 = match v4.clone() {
            Adt3::C0(_, h1415, _, _, _, _, _, _) => {
                h1415
            }
        };
        nat_mul(a1416, a1417)?
    };
    let v7: u64 = {
        let a1418 = 2u64;
        let a1419 = v6;
        nat_mul(a1418, a1419)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1420, _, _, _, _, _, _) => {
            h1420
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1421, _, _, _, _, _) => {
            h1421
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1422, _, _, _, _) => {
            h1422
        }
    });
    let a1423 = v0;
    let a1424 = v7;
    if nat_lt(a1423, a1424) {
        let a1428 = {
            let a1425 = v0;
            let a1426 = 2u64;
            let a1427 = 0u64;
            nat_rem(a1425, a1426, a1427)
        };
        let a1429 = 0u64;
        if nat_eq(a1428, a1429) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1430 = v0;
                let a1431 = 2u64;
                let a1432 = 0u64;
                nat_quot(a1430, a1431, a1432)
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
            let a1433 = v0;
            let a1434 = v7;
            nat_sub(a1433, a1434)
        };
        let v10: u64 = {
            let a1438 = v9;
            let a1439 = {
                let a1436 = 1u64;
                let a1437 = match v1.clone() {
                    Adt3::C0(h1435, _, _, _, _, _, _, _) => {
                        h1435
                    }
                };
                nat_add(a1436, a1437)?
            };
            let a1440 = 0u64;
            nat_quot(a1438, a1439, a1440)
        };
        let v11: u64 = {
            let a1444 = v9;
            let a1445 = {
                let a1442 = 1u64;
                let a1443 = match v1.clone() {
                    Adt3::C0(h1441, _, _, _, _, _, _, _) => {
                        h1441
                    }
                };
                nat_add(a1442, a1443)?
            };
            let a1446 = 0u64;
            nat_rem(a1444, a1445, a1446)
        };
        let a1447 = v11;
        let a1448 = 0u64;
        if nat_eq(a1447, a1448) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1449 = v10;
                let a1450 = v6;
                let a1451 = 0u64;
                nat_rem(a1449, a1450, a1451)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1457 = {
                        let a1453 = v10;
                        let a1454 = match v1.clone() {
                            Adt3::C0(h1452, _, _, _, _, _, _, _) => {
                                h1452
                            }
                        };
                        nat_mul(a1453, a1454)?
                    };
                    let a1458 = {
                        let a1455 = v11;
                        let a1456 = 1u64;
                        nat_sub(a1455, a1456)
                    };
                    nat_add(a1457, a1458)?
                };
                let a1459 = v5;
                let a1460 = 4294967295u64;
                if nat_eq(a1459, a1460) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1466 = {
                            let a1462 = v5;
                            let a1463 = match v3.clone() {
                                Adt3::C0(h1461, _, _, _, _, _, _, _) => {
                                    h1461
                                }
                            };
                            nat_mul(a1462, a1463)?
                        };
                        let a1467 = {
                            let a1464 = v11;
                            let a1465 = 1u64;
                            nat_sub(a1464, a1465)
                        };
                        nat_add(a1466, a1467)?
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
        let a1470 = v6;
        let a1471 = {
            let a1468 = 3u64;
            let a1469 = v5;
            nat_add(a1468, a1469)?
        };
        nat_sub(a1470, a1471)
    };
    let a1473 = v0;
    let a1474 = f25(v3, match v1.clone() {
        Adt3::C0(h1472, _, _, _, _, _, _, _) => {
            h1472
        }
    }, v5, v8)?;
    if nat_lt(a1473, a1474) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1475, _, _, _, _, _, _, _) => {
                h1475
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1476, _, _, _, _, _, _, _) => {
                h1476
            }
        }, v5)?;
        let a1477 = v0;
        let a1478 = v9;
        if nat_lt(a1477, a1478) {
            f110(v0, v2, f19(v3), {
                let a1479 = 3u64;
                let a1480 = v5;
                nat_add(a1479, a1480)?
            }, v7)
        } else {
            let a1483 = v0;
            let a1484 = {
                let a1481 = v9;
                let a1482 = v10;
                nat_add(a1481, a1482)?
            };
            if nat_lt(a1483, a1484) {
                let v11: u64 = {
                    let a1485 = v0;
                    let a1486 = v9;
                    nat_sub(a1485, a1486)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1487, _, _, _, _, _, _, _) => {
                        h1487
                    }
                })?;
                f112({
                    let a1488 = v11;
                    let a1489 = v12;
                    let a1490 = 0u64;
                    nat_rem(a1488, a1489, a1490)
                }, {
                    let a1491 = v11;
                    let a1492 = v12;
                    let a1493 = 0u64;
                    nat_quot(a1491, a1492, a1493)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1496 = v0;
                    let a1497 = {
                        let a1494 = v9;
                        let a1495 = v10;
                        nat_add(a1494, a1495)?
                    };
                    nat_sub(a1496, a1497)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1498 = v0;
    let a1499 = f12(v1)?;
    if nat_lt(a1498, a1499) {
        let a1500 = v0;
        let a1501 = 4u64;
        if nat_lt(a1500, a1501) {
            let a1502 = v0;
            let a1503 = 0u64;
            if nat_eq(a1502, a1503) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1504 = v0;
                let a1505 = 1u64;
                if nat_eq(a1504, a1505) {
                    let a1506 = v5;
                    let a1507 = 0u64;
                    if nat_eq(a1506, a1507) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1508 = v0;
                    let a1509 = 2u64;
                    if nat_eq(a1508, a1509) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1510 = v0;
            let a1511 = 4u64;
            if if nat_eq(a1510, a1511) {
                let a1512 = f114(v5)?;
                let a1513 = v1;
                !nat_eq(a1512, a1513)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1516 = {
                        let a1514 = v0;
                        let a1515 = 4u64;
                        nat_sub(a1514, a1515)
                    };
                    let a1517 = 3u64;
                    let a1518 = 0u64;
                    nat_quot(a1516, a1517, a1518)
                };
                let v7: u64 = {
                    let a1521 = {
                        let a1519 = v0;
                        let a1520 = 4u64;
                        nat_sub(a1519, a1520)
                    };
                    let a1522 = 3u64;
                    let a1523 = 0u64;
                    nat_rem(a1521, a1522, a1523)
                };
                let a1524 = v7;
                let a1525 = 0u64;
                if nat_eq(a1524, a1525) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1526 = v7;
                    let a1527 = 1u64;
                    if nat_eq(a1526, a1527) {
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
        let a1528 = v0;
        let a1529 = 4294967296u64;
        let a1530 = 0u64;
        nat_rem(a1528, a1529, a1530)
    };
    let a1531 = v1;
    let a1532 = 2147483648u64;
    if nat_lt(a1531, a1532) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1535 = 2u64;
    let a1536 = {
        let a1533 = 2u64;
        let a1534 = v0;
        nat_mul(a1533, a1534)?
    };
    nat_add(a1535, a1536)
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
    let a1537 = v2;
    let a1538 = 0u64;
    if nat_eq(a1537, a1538) {
        Ok(0u64)
    } else {
        let a1541 = 2u64;
        let a1542 = {
            let a1539 = v0;
            let a1540 = v1;
            nat_mul(a1539, a1540)?
        };
        nat_mul(a1541, a1542)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1543 = v2;
    let a1544 = f111(v0, v1)?;
    nat_mul(a1543, a1544)
}

pub fn f65(v0: u64) -> u64 {
    let a1545 = v0;
    let a1546 = 30u64;
    if nat_eq(a1545, a1546) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1549 = 5u64;
    let a1550 = {
        let a1547 = 4u64;
        let a1548 = v0;
        nat_mul(a1547, a1548)?
    };
    nat_add(a1549, a1550)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1551, _, _, _, _, _, _, _, _, _, _) => {
            h1551
        }
    }, match v0.clone() {
        Adt2::C0(_, h1552, _, _, _, _, _, _, _, _, _) => {
            h1552
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1553, _, _, _, _, _, _, _, _) => {
            h1553
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1554, _, _, _, _, _, _, _) => {
            h1554
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1555, _, _, _, _, _, _) => {
            h1555
        }
    }, match v1.clone() {
        Adt2::C0(h1556, _, _, _, _, _, _, _, _, _, _) => {
            h1556
        }
    }, match v1.clone() {
        Adt2::C0(_, h1557, _, _, _, _, _, _, _, _, _) => {
            h1557
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1558, _, _, _, _, _, _, _, _) => {
            h1558
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1559, _, _, _, _, _, _, _) => {
            h1559
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1560, _, _, _, _, _, _) => {
            h1560
        }
    }, match v2.clone() {
        Adt2::C0(h1561, _, _, _, _, _, _, _, _, _, _) => {
            h1561
        }
    }, match v2.clone() {
        Adt2::C0(_, h1562, _, _, _, _, _, _, _, _, _) => {
            h1562
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1563, _, _, _, _, _, _, _, _) => {
            h1563
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1564, _, _, _, _, _, _, _) => {
            h1564
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1565, _, _, _, _, _, _) => {
            h1565
        }
    }) {
        let m1567 = f70(match v0.clone() {
            Adt2::C0(h1566, _, _, _, _, _, _, _, _, _, _) => {
                h1566
            }
        });
        match m1567 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1573 = {
                    let a1570 = match v0.clone() {
                        Adt2::C0(_, h1568, _, _, _, _, _, _, _, _, _) => {
                            h1568
                        }
                    };
                    let a1571 = match v3.clone() {
                        Adt11::C0(_, h1569, _) => {
                            h1569
                        }
                    };
                    let a1572 = 0u64;
                    nat_rem(a1570, a1571, a1572)
                };
                let a1574 = 0u64;
                if if nat_eq(a1573, a1574) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1575, _, _, _, _, _, _, _, _, _) => {
                            h1575
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1576, _, _, _, _, _, _, _, _, _) => {
                            h1576
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1577, _, _, _, _, _, _, _, _) => {
                            h1577
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1578, _, _, _, _, _, _, _) => {
                            h1578
                        }
                    }) {
                        true
                    } else {
                        let a1580 = 4u64;
                        let a1581 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1579, _, _, _, _, _) => {
                                h1579
                            }
                        };
                        if if nat_le(a1580, a1581) {
                            let a1583 = match v3.clone() {
                                Adt11::C0(_, h1582, _) => {
                                    h1582
                                }
                            };
                            let a1584 = 1u64;
                            if nat_eq(a1583, a1584) {
                                true
                            } else {
                                let a1587 = match v3.clone() {
                                    Adt11::C0(_, _, h1585) => {
                                        h1585
                                    }
                                };
                                let a1588 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1586, _, _, _, _, _) => {
                                        h1586
                                    }
                                };
                                nat_le(a1587, a1588)
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
                        Adt11::C0(_, h1589, _) => {
                            h1589
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1590, _, _, _, _, _, _, _, _, _) => {
                            h1590
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1591, _, _, _, _, _, _, _, _, _) => {
                            h1591
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1592, _, _, _, _, _, _, _, _) => {
                            h1592
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1593, _, _, _, _, _, _, _) => {
                            h1593
                        }
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1594, _, _, _, _, _, _, _, _, _, _) => {
                            h1594
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
    let a1597 = match v1.clone() {
        Adt2::C0(h1595, _, _, _, _, _, _, _, _, _, _) => {
            h1595
        }
    };
    let a1598 = match v3.clone() {
        Adt2::C0(h1596, _, _, _, _, _, _, _, _, _, _) => {
            h1596
        }
    };
    if if nat_eq(a1597, a1598) {
        let a1599 = v0;
        let a1600 = 33u64;
        if nat_eq(a1599, a1600) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1602 = f70(match v1.clone() {
            Adt2::C0(h1601, _, _, _, _, _, _, _, _, _, _) => {
                h1601
            }
        });
        match m1602 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1603, _) => {
                        h1603
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1608 = {
                    let a1605 = match v1.clone() {
                        Adt2::C0(_, h1604, _, _, _, _, _, _, _, _, _) => {
                            h1604
                        }
                    };
                    let a1606 = v5;
                    let a1607 = 1u64;
                    nat_rem(a1605, a1606, a1607)
                };
                let a1609 = 0u64;
                if if if nat_eq(a1608, a1609) {
                    let a1614 = {
                        let a1611 = match v3.clone() {
                            Adt2::C0(_, h1610, _, _, _, _, _, _, _, _, _) => {
                                h1610
                            }
                        };
                        let a1612 = v5;
                        let a1613 = 1u64;
                        nat_rem(a1611, a1612, a1613)
                    };
                    let a1615 = 0u64;
                    nat_eq(a1614, a1615)
                } else {
                    false
                } {
                    let a1616 = v6;
                    let a1617 = f9(v3.clone(), v5)?;
                    nat_eq(a1616, a1617)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1618) => {
                            h1618
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
    let a1619 = v0;
    let a1620 = 0u64;
    if if nat_eq(a1619, a1620) {
        true
    } else {
        let a1621 = v0;
        let a1622 = 1u64;
        nat_eq(a1621, a1622)
    } {
        true
    } else {
        let a1623 = v0;
        let a1624 = 30u64;
        nat_eq(a1623, a1624)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1625 = v0;
    let a1626 = 0u64;
    if nat_eq(a1625, a1626) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1627 = v0;
        let a1628 = 1u64;
        if nat_eq(a1627, a1628) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1629 = v0;
            let a1630 = 2u64;
            if nat_eq(a1629, a1630) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1631 = v0;
                let a1632 = 3u64;
                if nat_eq(a1631, a1632) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1633 = v0;
                    let a1634 = 6u64;
                    if nat_eq(a1633, a1634) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1635 = v0;
                        let a1636 = 7u64;
                        if nat_eq(a1635, a1636) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1637 = v0;
                            let a1638 = 8u64;
                            if nat_eq(a1637, a1638) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1639 = v0;
                                let a1640 = 9u64;
                                if nat_eq(a1639, a1640) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1641 = v0;
                                    let a1642 = 10u64;
                                    if nat_eq(a1641, a1642) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1643 = v0;
                                        let a1644 = 11u64;
                                        if nat_eq(a1643, a1644) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1645 = v0;
                                            let a1646 = 12u64;
                                            if nat_eq(a1645, a1646) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1647 = v0;
                                                let a1648 = 13u64;
                                                if nat_eq(a1647, a1648) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1649 = v0;
                                                    let a1650 = 14u64;
                                                    if nat_eq(a1649, a1650) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1651 = v0;
                                                        let a1652 = 15u64;
                                                        if nat_eq(a1651, a1652) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1653 = v0;
                                                            let a1654 = 16u64;
                                                            if nat_eq(a1653, a1654) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1655 = v0;
                                                                let a1656 = 17u64;
                                                                if nat_eq(a1655, a1656) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1657 = v0;
                                                                    let a1658 = 18u64;
                                                                    if nat_eq(a1657, a1658) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1659 = v0;
                                                                        let a1660 = 19u64;
                                                                        if nat_eq(a1659, a1660) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1661 = v0;
                                                                            let a1662 = 20u64;
                                                                            if nat_eq(a1661, a1662) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1663 = v0;
                                                                                let a1664 = 21u64;
                                                                                if nat_eq(a1663, a1664) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1665 = v0;
                                                                                    let a1666 = 22u64;
                                                                                    if nat_eq(a1665, a1666) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1667 = v0;
                                                                                        let a1668 = 23u64;
                                                                                        if nat_eq(a1667, a1668) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1669 = v0;
                                                                                            let a1670 = 24u64;
                                                                                            if nat_eq(a1669, a1670) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1671 = v0;
                                                                                                let a1672 = 25u64;
                                                                                                if nat_eq(a1671, a1672) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1673 = v0;
                                                                                                    let a1674 = 26u64;
                                                                                                    if nat_eq(a1673, a1674) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1675 = v0;
                                                                                                        let a1676 = 27u64;
                                                                                                        if nat_eq(a1675, a1676) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1677 = v0;
                                                                                                            let a1678 = 28u64;
                                                                                                            if nat_eq(a1677, a1678) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1679 = v0;
                                                                                                                let a1680 = 29u64;
                                                                                                                if nat_eq(a1679, a1680) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1681 = v0;
                                                                                                                    let a1682 = 30u64;
                                                                                                                    if nat_eq(a1681, a1682) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1683 = v0;
                                                                                                                        let a1684 = 34u64;
                                                                                                                        if nat_eq(a1683, a1684) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1685 = v0;
                                                                                                                            let a1686 = 35u64;
                                                                                                                            if nat_eq(a1685, a1686) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1687 = v0;
                                                                                                                                let a1688 = 39u64;
                                                                                                                                if nat_eq(a1687, a1688) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1689 = v0;
                                                                                                                                    let a1690 = 40u64;
                                                                                                                                    if nat_eq(a1689, a1690) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1691 = v0;
                                                                                                                                        let a1692 = 41u64;
                                                                                                                                        if nat_eq(a1691, a1692) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1693 = v0;
                                                                                                                                            let a1694 = 42u64;
                                                                                                                                            if nat_eq(a1693, a1694) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1695 = v0;
                                                                                                                                                let a1696 = 43u64;
                                                                                                                                                if nat_eq(a1695, a1696) {
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
        let a1700 = {
            let a1697 = v0;
            let a1698 = v4;
            let a1699 = 1u64;
            nat_rem(a1697, a1698, a1699)
        };
        let a1701 = 0u64;
        if if if nat_eq(a1700, a1701) {
            let a1705 = {
                let a1702 = v1;
                let a1703 = v5;
                let a1704 = 1u64;
                nat_rem(a1702, a1703, a1704)
            };
            let a1706 = 0u64;
            nat_eq(a1705, a1706)
        } else {
            false
        } {
            let a1710 = {
                let a1707 = v2;
                let a1708 = v6;
                let a1709 = 1u64;
                nat_rem(a1707, a1708, a1709)
            };
            let a1711 = 0u64;
            nat_eq(a1710, a1711)
        } else {
            false
        } {
            let a1715 = {
                let a1712 = v3;
                let a1713 = v7;
                let a1714 = 1u64;
                nat_rem(a1712, a1713, a1714)
            };
            let a1716 = 0u64;
            nat_eq(a1715, a1716)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1717 = v0;
    let a1718 = v1;
    if nat_eq(a1717, a1718) {
        let a1721 = v4;
        let a1722 = {
            let a1719 = v2;
            let a1720 = v3;
            nat_add(a1719, a1720)?
        };
        Ok(nat_eq(a1721, a1722))
    } else {
        let a1723 = v2;
        let a1724 = v3;
        if nat_eq(a1723, a1724) {
            let a1725 = v4;
            let a1726 = v2;
            Ok(nat_eq(a1725, a1726))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1729 = {
            let a1727 = v0;
            let a1728 = f119(v1);
            nat_add(a1727, a1728)?
        };
        let a1730 = f119(v2);
        nat_add(a1729, a1730)?
    };
    let v5: u64 = {
        let a1731 = f120(v1);
        let a1732 = f120(v2);
        nat_add(a1731, a1732)?
    };
    let a1733 = v1;
    let a1734 = 4294967295u64;
    if if nat_le(a1733, a1734) {
        let a1735 = v2;
        let a1736 = 4294967295u64;
        nat_le(a1735, a1736)
    } else {
        false
    } {
        let a1737 = v5;
        let a1738 = v4;
        if nat_le(a1737, a1738) {
            let a1741 = v3;
            let a1742 = {
                let a1739 = v4;
                let a1740 = v5;
                nat_sub(a1739, a1740)
            };
            Ok(nat_eq(a1741, a1742))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1743 = v1;
    let a1744 = 4294967295u64;
    if nat_le(a1743, a1744) {
        let a1747 = {
            let a1745 = f119(v1);
            let a1746 = f120(v1);
            nat_add(a1745, a1746)?
        };
        let a1748 = v0;
        Ok(nat_lt(a1747, a1748))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1749 = v6;
    let a1750 = v1;
    if if nat_le(a1749, a1750) {
        let a1757 = {
            let a1755 = {
                let a1751 = v1;
                let a1752 = v5;
                nat_add(a1751, a1752)?
            };
            let a1756 = {
                let a1753 = v3;
                let a1754 = 0u64;
                if nat_eq(a1753, a1754) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1755, a1756)?
        };
        let a1758 = v2;
        nat_le(a1757, a1758)
    } else {
        false
    } {
        if v4 {
            let a1763 = {
                let a1761 = {
                    let a1759 = v1;
                    let a1760 = v5;
                    nat_add(a1759, a1760)?
                };
                let a1762 = v6;
                nat_sub(a1761, a1762)
            };
            let a1764 = v2;
            Ok(nat_lt(a1763, a1764))
        } else {
            let a1769 = {
                let a1767 = {
                    let a1765 = v1;
                    let a1766 = v5;
                    nat_add(a1765, a1766)?
                };
                let a1768 = v6;
                nat_sub(a1767, a1768)
            };
            let a1770 = v2;
            Ok(nat_le(a1769, a1770))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1771 = v0;
    let a1772 = 0u64;
    if if nat_eq(a1771, a1772) {
        true
    } else {
        let a1773 = v0;
        let a1774 = 1u64;
        nat_eq(a1773, a1774)
    } {
        let a1775 = v1;
        let a1776 = 4294967296u64;
        nat_lt(a1775, a1776)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1781 = {
        let a1779 = v0;
        let a1780 = {
            let a1777 = v1;
            let a1778 = 1u64;
            nat_sub(a1777, a1778)
        };
        nat_add(a1779, a1780)?
    };
    let a1782 = v1;
    let a1783 = 0u64;
    Ok(nat_quot(a1781, a1782, a1783))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1784, _, _, _, _, _, _, _, _, _) => {
            h1784
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1785, _, _, _, _, _, _, _, _) => {
            h1785
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1786, _, _, _, _, _, _, _) => {
            h1786
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1787, _, _, _, _, _, _) => {
            h1787
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1790 = match v1.clone() {
                Adt2::C0(_, h1788, _, _, _, _, _, _, _, _, _) => {
                    h1788
                }
            };
            let a1791 = match v1.clone() {
                Adt2::C0(_, _, h1789, _, _, _, _, _, _, _, _) => {
                    h1789
                }
            };
            nat_mul(a1790, a1791)?
        };
        let v5: u64 = {
            let a1793 = v4;
            let a1794 = match v1.clone() {
                Adt2::C0(_, _, _, h1792, _, _, _, _, _, _, _) => {
                    h1792
                }
            };
            nat_mul(a1793, a1794)?
        };
        let a1807 = {
            let a1805 = {
                let a1796 = match v0.clone() {
                    Adt2::C0(_, h1795, _, _, _, _, _, _, _, _, _) => {
                        h1795
                    }
                };
                let a1797 = 1u64;
                nat_sub(a1796, a1797)
            };
            let a1806 = {
                let a1803 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1798, _, _, _, _, _, _, _, _) => {
                        h1798
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1799, _, _, _, _, _, _, _, _, _) => {
                        h1799
                    }
                })?;
                let a1804 = f122(match v0 {
                    Adt2::C0(_, _, _, h1800, _, _, _, _, _, _, _) => {
                        h1800
                    }
                }, v2, {
                    let a1801 = v3;
                    let a1802 = v5;
                    nat_mul(a1801, a1802)?
                }, v4)?;
                nat_add(a1803, a1804)?
            };
            nat_add(a1805, a1806)?
        };
        let a1808 = f9(v1, 1u64)?;
        Ok(nat_lt(a1807, a1808))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1809 = v0;
    let a1810 = 2147483648u64;
    if if if if nat_lt(a1809, a1810) {
        let a1811 = v1;
        let a1812 = 2147483648u64;
        nat_lt(a1811, a1812)
    } else {
        false
    } {
        let a1815 = {
            let a1813 = v0;
            let a1814 = v1;
            nat_mul(a1813, a1814)?
        };
        let a1816 = 2147483648u64;
        nat_lt(a1815, a1816)
    } else {
        false
    } {
        let a1821 = {
            let a1819 = {
                let a1817 = v0;
                let a1818 = 1u64;
                nat_sub(a1817, a1818)
            };
            let a1820 = v2;
            nat_mul(a1819, a1820)?
        };
        let a1822 = 2147483648u64;
        nat_lt(a1821, a1822)
    } else {
        false
    } {
        let a1827 = {
            let a1825 = {
                let a1823 = v1;
                let a1824 = 1u64;
                nat_sub(a1823, a1824)
            };
            let a1826 = v2;
            nat_mul(a1825, a1826)?
        };
        let a1828 = 2147483648u64;
        Ok(nat_lt(a1827, a1828))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1831 = 3u64;
    let a1832 = {
        let a1829 = v0;
        let a1830 = 3u64;
        nat_mul(a1829, a1830)?
    };
    nat_add(a1831, a1832)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1833, _, _, _, _, _, _, _) => {
            h1833
        }
    }, match v0.clone() {
        Adt3::C0(_, h1834, _, _, _, _, _, _) => {
            h1834
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1835, _, _, _, _, _) => {
            h1835
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1836, _, _, _, _) => {
            h1836
        }
    }) {
        Ok(0u64)
    } else {
        let a1845 = match v0.clone() {
            Adt3::C0(h1837, _, _, _, _, _, _, _) => {
                h1837
            }
        };
        let a1846 = {
            let a1843 = match v0.clone() {
                Adt3::C0(_, h1838, _, _, _, _, _, _) => {
                    h1838
                }
            };
            let a1844 = {
                let a1841 = match v0.clone() {
                    Adt3::C0(_, _, h1839, _, _, _, _, _) => {
                        h1839
                    }
                };
                let a1842 = match v0 {
                    Adt3::C0(_, _, _, h1840, _, _, _, _) => {
                        h1840
                    }
                };
                nat_mul(a1841, a1842)?
            };
            nat_mul(a1843, a1844)?
        };
        nat_mul(a1845, a1846)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1847 = v1;
    let a1848 = 13u64;
    if if nat_eq(a1847, a1848) {
        f23(match v0.clone() {
            Adt3::C0(h1849, _, _, _, _, _, _, _) => {
                h1849
            }
        }, match v0.clone() {
            Adt3::C0(_, h1850, _, _, _, _, _, _) => {
                h1850
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1851, _, _, _, _, _) => {
                h1851
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1852, _, _, _, _) => {
                h1852
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1858 = match v0.clone() {
                Adt3::C0(_, h1853, _, _, _, _, _, _) => {
                    h1853
                }
            };
            let a1859 = {
                let a1856 = match v0.clone() {
                    Adt3::C0(_, _, h1854, _, _, _, _, _) => {
                        h1854
                    }
                };
                let a1857 = match v0.clone() {
                    Adt3::C0(_, _, _, h1855, _, _, _, _) => {
                        h1855
                    }
                };
                nat_mul(a1856, a1857)?
            };
            nat_mul(a1858, a1859)?
        };
        let a1860 = v1;
        let a1861 = 13u64;
        if nat_eq(a1860, a1861) {
            let a1870 = 2u64;
            let a1871 = if f123(v0.clone(), v2) {
                let a1863 = v3;
                let a1864 = f124(match v0 {
                    Adt3::C0(h1862, _, _, _, _, _, _, _) => {
                        h1862
                    }
                }, false)?;
                nat_mul(a1863, a1864)?
            } else {
                let a1868 = v3;
                let a1869 = {
                    let a1866 = match v0 {
                        Adt3::C0(h1865, _, _, _, _, _, _, _) => {
                            h1865
                        }
                    };
                    let a1867 = 2u64;
                    nat_mul(a1866, a1867)?
                };
                nat_mul(a1868, a1869)?
            };
            nat_add(a1870, a1871)
        } else {
            let a1875 = v3;
            let a1876 = f124(match v0 {
                Adt3::C0(h1872, _, _, _, _, _, _, _) => {
                    h1872
                }
            }, {
                let a1873 = v1;
                let a1874 = 16u64;
                nat_eq(a1873, a1874)
            })?;
            nat_mul(a1875, a1876)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1877 = v3;
    match m1877 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1878, _, _, _, _, _, _, _, _, _, _) => {
                    h1878
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1879, _, _, _, _, _, _, _, _, _, _) => {
                    h1879
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1880, _, _, _, _, _, _, _, _, _) => {
                    h1880
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1881, _, _, _, _, _, _, _, _) => {
                    h1881
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1882, _, _, _, _, _, _, _) => {
                    h1882
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1883, _, _, _, _, _, _) => {
                    h1883
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1884, _, _, _, _, _, _, _, _, _) => {
                    h1884
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1885, _, _, _, _, _, _, _, _) => {
                    h1885
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1886, _, _, _, _, _, _, _) => {
                    h1886
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1887, _, _, _, _, _, _) => {
                    h1887
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1888, _, _, _, _, _, _, _, _, _, _) => {
                    h1888
                }
            }, match v5.clone() {
                Adt2::C0(h1889, _, _, _, _, _, _, _, _, _, _) => {
                    h1889
                }
            }, match v4.clone() {
                Adt2::C0(h1890, _, _, _, _, _, _, _, _, _, _) => {
                    h1890
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1891, _, _, _, _, _, _, _, _, _) => {
                    h1891
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1892, _, _, _, _, _, _, _, _) => {
                    h1892
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1893, _, _, _, _, _, _, _) => {
                    h1893
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1894, _, _, _, _, _, _) => {
                    h1894
                }
            }, match v5.clone() {
                Adt2::C0(_, h1895, _, _, _, _, _, _, _, _, _) => {
                    h1895
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1896, _, _, _, _, _, _, _, _) => {
                    h1896
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1897, _, _, _, _, _, _, _) => {
                    h1897
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1898, _, _, _, _, _, _) => {
                    h1898
                }
            }, match v4.clone() {
                Adt2::C0(_, h1899, _, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1900, _, _, _, _, _, _, _, _) => {
                    h1900
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1901, _, _, _, _, _, _, _) => {
                    h1901
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1902, _, _, _, _, _, _) => {
                    h1902
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1903 = v7;
        let a1904 = 6u64;
        if nat_eq(a1903, a1904) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1905 = v0;
    match m1905 {
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
    let m1906 = v0;
    match m1906 {
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
    let m1907 = v0;
    match m1907 {
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
            let a1908 = v1;
            let a1909 = 6u64;
            if nat_eq(a1908, a1909) {
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
            let a1910 = v1;
            let a1911 = 6u64;
            if if if nat_eq(a1910, a1911) {
                true
            } else {
                let a1912 = v1;
                let a1913 = 7u64;
                nat_eq(a1912, a1913)
            } {
                let a1914 = v49;
                let a1915 = 30u64;
                nat_eq(a1914, a1915)
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
    let a1916 = v1;
    let a1917 = 4u64;
    if nat_lt(a1916, a1917) {
        Adt7::C0(v1)
    } else {
        let a1918 = v1;
        let a1919 = 4u64;
        if nat_eq(a1918, a1919) {
            Adt7::C1(0u64)
        } else {
            let a1920 = v1;
            let a1921 = 5u64;
            if nat_eq(a1920, a1921) {
                Adt7::C1(3u64)
            } else {
                let a1922 = v1;
                let a1923 = 6u64;
                if nat_eq(a1922, a1923) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1924 = v1;
                    let a1925 = 7u64;
                    if nat_eq(a1924, a1925) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1926 = v1;
                        let a1927 = 16u64;
                        if nat_lt(a1926, a1927) {
                            Adt7::C1({
                                let a1928 = v1;
                                let a1929 = 8u64;
                                nat_sub(a1928, a1929)
                            })
                        } else {
                            let a1930 = v1;
                            let a1931 = 20u64;
                            if nat_lt(a1930, a1931) {
                                Adt7::C5({
                                    let a1932 = v1;
                                    let a1933 = 16u64;
                                    nat_sub(a1932, a1933)
                                })
                            } else {
                                let a1934 = v1;
                                let a1935 = 20u64;
                                if nat_eq(a1934, a1935) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1936 = v1;
                                    let a1937 = 21u64;
                                    if nat_eq(a1936, a1937) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1938 = v1;
                                        let a1939 = 22u64;
                                        if nat_eq(a1938, a1939) {
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
        let a1941 = v0;
        let a1942 = match v1.clone() {
            Adt3::C0(h1940, _, _, _, _, _, _, _) => {
                h1940
            }
        };
        let a1943 = 0u64;
        nat_quot(a1941, a1942, a1943)
    };
    let v3: u64 = {
        let a1945 = v2;
        let a1946 = match v1.clone() {
            Adt3::C0(_, h1944, _, _, _, _, _, _) => {
                h1944
            }
        };
        let a1947 = 0u64;
        nat_quot(a1945, a1946, a1947)
    };
    f129({
        let a1949 = v0;
        let a1950 = match v1.clone() {
            Adt3::C0(h1948, _, _, _, _, _, _, _) => {
                h1948
            }
        };
        let a1951 = 0u64;
        nat_rem(a1949, a1950, a1951)
    }, {
        let a1953 = v2;
        let a1954 = match v1.clone() {
            Adt3::C0(_, h1952, _, _, _, _, _, _) => {
                h1952
            }
        };
        let a1955 = 0u64;
        nat_rem(a1953, a1954, a1955)
    }, {
        let a1957 = v3;
        let a1958 = match v1.clone() {
            Adt3::C0(_, _, h1956, _, _, _, _, _) => {
                h1956
            }
        };
        let a1959 = 0u64;
        nat_rem(a1957, a1958, a1959)
    }, {
        let a1961 = v3;
        let a1962 = match v1.clone() {
            Adt3::C0(_, _, h1960, _, _, _, _, _) => {
                h1960
            }
        };
        let a1963 = 0u64;
        nat_quot(a1961, a1962, a1963)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1964, _, _, _) => {
            h1964
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1965, _, _) => {
            h1965
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1966, _) => {
            h1966
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1967) => {
            h1967
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1985 = {
        let a1980 = f130(v2, {
            let a1971 = {
                let a1968 = v0;
                let a1969 = v1;
                let a1970 = 0u64;
                nat_quot(a1968, a1969, a1970)
            };
            let a1972 = v2;
            let a1973 = 0u64;
            nat_rem(a1971, a1972, a1973)
        }, {
            let a1977 = {
                let a1974 = v0;
                let a1975 = v1;
                let a1976 = 0u64;
                nat_quot(a1974, a1975, a1976)
            };
            let a1978 = v2;
            let a1979 = 0u64;
            nat_quot(a1977, a1978, a1979)
        })?;
        let a1981 = v1;
        nat_mul(a1980, a1981)?
    };
    let a1986 = {
        let a1982 = v0;
        let a1983 = v1;
        let a1984 = 0u64;
        nat_rem(a1982, a1983, a1984)
    };
    nat_add(a1985, a1986)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1988 = v0;
        let a1989 = match v2.clone() {
            Adt3::C0(h1987, _, _, _, _, _, _, _) => {
                h1987
            }
        };
        let a1990 = 0u64;
        nat_quot(a1988, a1989, a1990)
    };
    let v4: u64 = {
        let a1992 = v3;
        let a1993 = match v2.clone() {
            Adt3::C0(_, h1991, _, _, _, _, _, _) => {
                h1991
            }
        };
        let a1994 = 0u64;
        nat_quot(a1992, a1993, a1994)
    };
    f131({
        let a1996 = v0;
        let a1997 = match v2.clone() {
            Adt3::C0(h1995, _, _, _, _, _, _, _) => {
                h1995
            }
        };
        let a1998 = 0u64;
        nat_rem(a1996, a1997, a1998)
    }, {
        let a2000 = v3;
        let a2001 = match v2.clone() {
            Adt3::C0(_, h1999, _, _, _, _, _, _) => {
                h1999
            }
        };
        let a2002 = 0u64;
        nat_rem(a2000, a2001, a2002)
    }, {
        let a2004 = v4;
        let a2005 = match v2.clone() {
            Adt3::C0(_, _, h2003, _, _, _, _, _) => {
                h2003
            }
        };
        let a2006 = 0u64;
        nat_rem(a2004, a2005, a2006)
    }, {
        let a2008 = v4;
        let a2009 = match v2 {
            Adt3::C0(_, _, h2007, _, _, _, _, _) => {
                h2007
            }
        };
        let a2010 = 0u64;
        nat_quot(a2008, a2009, a2010)
    }, match v1.clone() {
        Adt3::C0(h2011, _, _, _, _, _, _, _) => {
            h2011
        }
    }, match v1.clone() {
        Adt3::C0(_, h2012, _, _, _, _, _, _) => {
            h2012
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2013, _, _, _, _, _) => {
            h2013
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2014, _, _, _, _) => {
            h2014
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2015, _, _, _) => {
            h2015
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2016, _, _) => {
            h2016
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2017, _) => {
            h2017
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2018) => {
            h2018
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2020 = v0;
        let a2021 = match v3.clone() {
            Adt3::C0(h2019, _, _, _, _, _, _, _) => {
                h2019
            }
        };
        let a2022 = 0u64;
        nat_quot(a2020, a2021, a2022)
    };
    let v6: u64 = {
        let a2024 = v5;
        let a2025 = match v3.clone() {
            Adt3::C0(_, h2023, _, _, _, _, _, _) => {
                h2023
            }
        };
        let a2026 = 0u64;
        nat_quot(a2024, a2025, a2026)
    };
    let v7: u64 = {
        let a2028 = v0;
        let a2029 = match v3.clone() {
            Adt3::C0(h2027, _, _, _, _, _, _, _) => {
                h2027
            }
        };
        let a2030 = 0u64;
        nat_rem(a2028, a2029, a2030)
    };
    let v8: u64 = {
        let a2032 = v5;
        let a2033 = match v3.clone() {
            Adt3::C0(_, h2031, _, _, _, _, _, _) => {
                h2031
            }
        };
        let a2034 = 0u64;
        nat_rem(a2032, a2033, a2034)
    };
    let v9: u64 = {
        let a2036 = v6;
        let a2037 = match v3.clone() {
            Adt3::C0(_, _, h2035, _, _, _, _, _) => {
                h2035
            }
        };
        let a2038 = 0u64;
        nat_rem(a2036, a2037, a2038)
    };
    let v10: u64 = {
        let a2040 = v6;
        let a2041 = match v3.clone() {
            Adt3::C0(_, _, h2039, _, _, _, _, _) => {
                h2039
            }
        };
        let a2042 = 0u64;
        nat_quot(a2040, a2041, a2042)
    };
    let v11: u64 = {
        let a2043 = v4;
        let a2044 = 0u64;
        if nat_eq(a2043, a2044) {
            match v1.clone() {
                Adt3::C0(h2045, _, _, _, _, _, _, _) => {
                    h2045
                }
            }
        } else {
            let a2046 = v4;
            let a2047 = 1u64;
            if nat_eq(a2046, a2047) {
                match v1.clone() {
                    Adt3::C0(_, h2048, _, _, _, _, _, _) => {
                        h2048
                    }
                }
            } else {
                let a2049 = v4;
                let a2050 = 2u64;
                if nat_eq(a2049, a2050) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2051, _, _, _, _, _) => {
                            h2051
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2052, _, _, _, _) => {
                            h2052
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2059 = v11;
        let a2060 = {
            let a2053 = v4;
            let a2054 = 0u64;
            if nat_eq(a2053, a2054) {
                v7
            } else {
                let a2055 = v4;
                let a2056 = 1u64;
                if nat_eq(a2055, a2056) {
                    v8
                } else {
                    let a2057 = v4;
                    let a2058 = 2u64;
                    if nat_eq(a2057, a2058) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2059, a2060)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2061 = v4;
        let a2062 = 0u64;
        nat_eq(a2061, a2062)
    } else {
        false
    } {
        let a2063 = v7;
        let a2064 = v11;
        nat_sub(a2063, a2064)
    } else {
        v7
    }, if if v12 {
        let a2065 = v4;
        let a2066 = 1u64;
        nat_eq(a2065, a2066)
    } else {
        false
    } {
        let a2067 = v8;
        let a2068 = v11;
        nat_sub(a2067, a2068)
    } else {
        v8
    }, if if v12 {
        let a2069 = v4;
        let a2070 = 2u64;
        nat_eq(a2069, a2070)
    } else {
        false
    } {
        let a2071 = v9;
        let a2072 = v11;
        nat_sub(a2071, a2072)
    } else {
        v9
    }, if if v12 {
        let a2073 = v4;
        let a2074 = 3u64;
        nat_eq(a2073, a2074)
    } else {
        false
    } {
        let a2075 = v10;
        let a2076 = v11;
        nat_sub(a2075, a2076)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2077, _, _, _) => {
            h2077
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2078, _, _) => {
            h2078
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2079, _) => {
            h2079
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2080) => {
            h2080
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
            Adt3::C0(h2081, _, _, _, _, _, _, _) => {
                h2081
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2082, _, _, _, _, _, _) => {
                h2082
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2083, _, _, _, _, _) => {
                h2083
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2084, _, _, _, _) => {
                h2084
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2085, _, _, _, _, _, _, _) => {
                h2085
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2086, _, _, _, _, _, _) => {
                    h2086
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2087, _, _, _, _, _) => {
                    h2087
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2088, _, _, _, _) => {
                    h2088
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
        Adt3::C0(h2089, _, _, _, _, _, _, _) => {
            h2089
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2090 = v0;
    let a2091 = v3;
    if nat_lt(a2090, a2091) {
        Ok(Adt5::C1(false, {
            let a2092 = v0;
            let a2093 = 4u64;
            nat_mul(a2092, a2093)?
        }, {
            let a2094 = v0;
            let a2095 = 4u64;
            nat_mul(a2094, a2095)?
        }))
    } else {
        let v6: u64 = {
            let a2096 = v0;
            let a2097 = v3;
            nat_sub(a2096, a2097)
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
    let a2098 = v0;
    let a2099 = v3;
    if nat_lt(a2098, a2099) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2100 = v0;
            let a2101 = v3;
            nat_sub(a2100, a2101)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2103 = v1;
        let a2104 = match v3.clone() {
            Adt3::C0(h2102, _, _, _, _, _, _, _) => {
                h2102
            }
        };
        let a2105 = 0u64;
        nat_rem(a2103, a2104, a2105)
    };
    let v5: u64 = {
        let a2111 = {
            let a2107 = v1;
            let a2108 = match v3.clone() {
                Adt3::C0(h2106, _, _, _, _, _, _, _) => {
                    h2106
                }
            };
            let a2109 = 0u64;
            nat_quot(a2107, a2108, a2109)
        };
        let a2112 = match v3.clone() {
            Adt3::C0(_, h2110, _, _, _, _, _, _) => {
                h2110
            }
        };
        let a2113 = 0u64;
        nat_rem(a2111, a2112, a2113)
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
        let a2114 = v3;
        let a2115 = 8u64;
        nat_add(a2114, a2115)?
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
        let a2117 = v0;
        let a2118 = match v2.clone() {
            Adt3::C0(h2116, _, _, _, _, _, _, _) => {
                h2116
            }
        };
        let a2119 = 0u64;
        nat_rem(a2117, a2118, a2119)
    };
    let v7: u64 = {
        let a2121 = v0;
        let a2122 = match v2.clone() {
            Adt3::C0(h2120, _, _, _, _, _, _, _) => {
                h2120
            }
        };
        let a2123 = 0u64;
        nat_quot(a2121, a2122, a2123)
    };
    let v8: u64 = {
        let a2125 = v7;
        let a2126 = match v2.clone() {
            Adt3::C0(_, h2124, _, _, _, _, _, _) => {
                h2124
            }
        };
        let a2127 = 0u64;
        nat_rem(a2125, a2126, a2127)
    };
    let v9: u64 = {
        let a2129 = v7;
        let a2130 = match v2.clone() {
            Adt3::C0(_, h2128, _, _, _, _, _, _) => {
                h2128
            }
        };
        let a2131 = 0u64;
        nat_quot(a2129, a2130, a2131)
    };
    let v10: u64 = {
        let a2133 = v9;
        let a2134 = match v2.clone() {
            Adt3::C0(_, _, h2132, _, _, _, _, _) => {
                h2132
            }
        };
        let a2135 = 0u64;
        nat_rem(a2133, a2134, a2135)
    };
    let v11: u64 = {
        let a2137 = v9;
        let a2138 = match v2.clone() {
            Adt3::C0(_, _, h2136, _, _, _, _, _) => {
                h2136
            }
        };
        let a2139 = 0u64;
        nat_quot(a2137, a2138, a2139)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2148 = {
                let a2143 = {
                    let a2140 = v10;
                    let a2141 = v4;
                    let a2142 = 0u64;
                    nat_quot(a2140, a2141, a2142)
                };
                let a2144 = v5;
                nat_mul(a2143, a2144)?
            };
            let a2149 = {
                let a2145 = v8;
                let a2146 = v4;
                let a2147 = 0u64;
                nat_quot(a2145, a2146, a2147)
            };
            nat_add(a2148, a2149)?
        };
        let v14: u64 = {
            let a2169 = v6;
            let a2170 = {
                let a2167 = match v1.clone() {
                    Adt3::C0(h2150, _, _, _, _, _, _, _) => {
                        h2150
                    }
                };
                let a2168 = {
                    let a2165 = {
                        let a2151 = v8;
                        let a2152 = v4;
                        let a2153 = 0u64;
                        nat_rem(a2151, a2152, a2153)
                    };
                    let a2166 = {
                        let a2163 = match v1.clone() {
                            Adt3::C0(_, h2154, _, _, _, _, _, _) => {
                                h2154
                            }
                        };
                        let a2164 = {
                            let a2161 = {
                                let a2155 = v10;
                                let a2156 = v4;
                                let a2157 = 0u64;
                                nat_rem(a2155, a2156, a2157)
                            };
                            let a2162 = {
                                let a2159 = match v1.clone() {
                                    Adt3::C0(_, _, h2158, _, _, _, _, _) => {
                                        h2158
                                    }
                                };
                                let a2160 = v13;
                                nat_mul(a2159, a2160)?
                            };
                            nat_add(a2161, a2162)?
                        };
                        nat_mul(a2163, a2164)?
                    };
                    nat_add(a2165, a2166)?
                };
                nat_mul(a2167, a2168)?
            };
            nat_add(a2169, a2170)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2176 = {
                let a2174 = {
                    let a2171 = v11;
                    let a2172 = v5;
                    let a2173 = 0u64;
                    nat_rem(a2171, a2172, a2173)
                };
                let a2175 = v4;
                nat_mul(a2174, a2175)?
            };
            let a2177 = v8;
            nat_add(a2176, a2177)?
        };
        let v16: u64 = {
            let a2183 = {
                let a2181 = {
                    let a2178 = v11;
                    let a2179 = v5;
                    let a2180 = 0u64;
                    nat_quot(a2178, a2179, a2180)
                };
                let a2182 = v4;
                nat_mul(a2181, a2182)?
            };
            let a2184 = v10;
            nat_add(a2183, a2184)?
        };
        let a2186 = v15;
        let a2187 = match v1.clone() {
            Adt3::C0(_, h2185, _, _, _, _, _, _) => {
                h2185
            }
        };
        if if nat_lt(a2186, a2187) {
            let a2189 = v16;
            let a2190 = match v1.clone() {
                Adt3::C0(_, _, h2188, _, _, _, _, _) => {
                    h2188
                }
            };
            nat_lt(a2189, a2190)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2199 = v6;
                let a2200 = {
                    let a2197 = match v1.clone() {
                        Adt3::C0(h2191, _, _, _, _, _, _, _) => {
                            h2191
                        }
                    };
                    let a2198 = {
                        let a2195 = v15;
                        let a2196 = {
                            let a2193 = match v1.clone() {
                                Adt3::C0(_, h2192, _, _, _, _, _, _) => {
                                    h2192
                                }
                            };
                            let a2194 = v16;
                            nat_mul(a2193, a2194)?
                        };
                        nat_add(a2195, a2196)?
                    };
                    nat_mul(a2197, a2198)?
                };
                nat_add(a2199, a2200)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2205 = {
        let a2202 = v0;
        let a2203 = match v1 {
            Adt3::C0(h2201, _, _, _, _, _, _, _) => {
                h2201
            }
        };
        let a2204 = 0u64;
        nat_rem(a2202, a2203, a2204)
    };
    let a2206 = 0u64;
    if nat_eq(a2205, a2206) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2207 = v0;
            let a2208 = 1u64;
            nat_sub(a2207, a2208)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2209 = v3;
    let a2210 = 13u64;
    if nat_eq(a2209, a2210) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2211, _, _, _, _, _, _, _) => {
                h2211
            }
        }, {
            let a2212 = v3;
            let a2213 = 16u64;
            nat_eq(a2212, a2213)
        })?;
        let v6: u64 = {
            let a2214 = v0;
            let a2215 = v5;
            let a2216 = 0u64;
            nat_quot(a2214, a2215, a2216)
        };
        let v7: u64 = {
            let a2217 = v0;
            let a2218 = v5;
            let a2219 = 0u64;
            nat_rem(a2217, a2218, a2219)
        };
        let v8: u64 = {
            let a2223 = 1u64;
            let a2224 = {
                let a2221 = match v1.clone() {
                    Adt3::C0(h2220, _, _, _, _, _, _, _) => {
                        h2220
                    }
                };
                let a2222 = 2u64;
                nat_mul(a2221, a2222)?
            };
            nat_add(a2223, a2224)?
        };
        let a2225 = v7;
        let a2226 = v8;
        if nat_lt(a2225, a2226) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2227 = v3;
            let a2228 = 14u64;
            if nat_eq(a2227, a2228) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2229 = v7;
                let a2230 = v8;
                if nat_eq(a2229, a2230) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2233 = v7;
                    let a2234 = {
                        let a2231 = v8;
                        let a2232 = 1u64;
                        nat_add(a2231, a2232)?
                    };
                    if nat_eq(a2233, a2234) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2235, _, _, _, _, _, _, _) => {
                                h2235
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
    let a2236 = v0;
    let a2237 = 0u64;
    if nat_eq(a2236, a2237) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2244 = {
            let a2238 = v0;
            let a2239 = 1u64;
            nat_add(a2238, a2239)?
        };
        let a2245 = {
            let a2242 = 2u64;
            let a2243 = {
                let a2240 = v3;
                let a2241 = 2u64;
                nat_mul(a2240, a2241)?
            };
            nat_add(a2242, a2243)?
        };
        if nat_eq(a2244, a2245) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2246 = v0;
                let a2247 = 1u64;
                nat_sub(a2246, a2247)
            };
            let v5: u64 = {
                let a2248 = v4;
                let a2249 = 2u64;
                let a2250 = 0u64;
                nat_quot(a2248, a2249, a2250)
            };
            let a2254 = {
                let a2251 = v4;
                let a2252 = 2u64;
                let a2253 = 0u64;
                nat_rem(a2251, a2252, a2253)
            };
            let a2255 = 0u64;
            if nat_eq(a2254, a2255) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2256 = v0;
    let a2257 = f18(v1, v2, v3)?;
    if nat_lt(a2256, a2257) {
        let a2258 = v0;
        let a2259 = v1;
        if nat_lt(a2258, a2259) {
            Ok(Adt5::C1(false, {
                let a2260 = 4u64;
                let a2261 = v0;
                nat_mul(a2260, a2261)?
            }, {
                let a2262 = 4u64;
                let a2263 = v0;
                nat_mul(a2262, a2263)?
            }))
        } else {
            let v4: u64 = {
                let a2264 = v0;
                let a2265 = v1;
                nat_sub(a2264, a2265)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2266 = v5;
            let a2267 = 2u64;
            if nat_lt(a2266, a2267) {
                Ok(Adt5::C4(6u64, {
                    let a2268 = v5;
                    let a2269 = 0u64;
                    if nat_eq(a2268, a2269) {
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
    let a2277 = {
        let a2270 = 2u64;
        let a2271 = f81(v1.clone())?;
        nat_mul(a2270, a2271)?
    };
    let a2278 = {
        let a2275 = f81(v1)?;
        let a2276 = {
            let a2273 = 1u64;
            let a2274 = match v0 {
                Adt3::C0(h2272, _, _, _, _, _, _, _) => {
                    h2272
                }
            };
            nat_add(a2273, a2274)?
        };
        nat_mul(a2275, a2276)?
    };
    nat_add(a2277, a2278)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2279 = v1;
    let a2280 = 2147483647u64;
    if nat_le(a2279, a2280) {
        let a2281 = v1;
        let a2282 = v0;
        nat_lt(a2281, a2282)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2283 = f70(v0);
    match m2283 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2285 = v2;
                let a2286 = match v7.clone() {
                    Adt11::C0(_, h2284, _) => {
                        h2284
                    }
                };
                nat_mul(a2285, a2286)?
            };
            let m2299 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2287, _, _, _, _, _, _, _) => {
                    h2287
                }
            }, match v5.clone() {
                Adt3::C0(_, h2288, _, _, _, _, _, _) => {
                    h2288
                }
            }, match v5 {
                Adt3::C0(_, _, h2289, _, _, _, _, _) => {
                    h2289
                }
            }, {
                let a2292 = match v3.clone() {
                    Adt3::C0(h2290, _, _, _, _, _, _, _) => {
                        h2290
                    }
                };
                let a2293 = match v7.clone() {
                    Adt11::C0(_, h2291, _) => {
                        h2291
                    }
                };
                nat_mul(a2292, a2293)?
            }, match v3.clone() {
                Adt3::C0(_, h2294, _, _, _, _, _, _) => {
                    h2294
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2295, _, _, _) => {
                    h2295
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2296, _, _) => {
                    h2296
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2297, _) => {
                    h2297
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2298) => {
                    h2298
                }
            })?;
            match m2299 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2300 = v0;
                    let a2301 = 0u64;
                    if if nat_eq(a2300, a2301) {
                        true
                    } else {
                        let a2302 = v0;
                        let a2303 = 26u64;
                        nat_eq(a2302, a2303)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2307 = {
                                let a2305 = v1;
                                let a2306 = match v4.clone() {
                                    Adt3::C0(h2304, _, _, _, _, _, _, _) => {
                                        h2304
                                    }
                                };
                                nat_mul(a2305, a2306)?
                            };
                            let a2308 = v8;
                            nat_add(a2307, a2308)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2309) => {
                                h2309
                            }
                        }, f89({
                            let a2313 = {
                                let a2311 = v1;
                                let a2312 = match v4.clone() {
                                    Adt3::C0(h2310, _, _, _, _, _, _, _) => {
                                        h2310
                                    }
                                };
                                nat_mul(a2311, a2312)?
                            };
                            let a2314 = v8;
                            nat_add(a2313, a2314)?
                        }, v4.clone())?, {
                            let a2321 = {
                                let a2319 = {
                                    let a2316 = match v7 {
                                        Adt11::C0(_, h2315, _) => {
                                            h2315
                                        }
                                    };
                                    let a2317 = 1u64;
                                    nat_sub(a2316, a2317)
                                };
                                let a2320 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2318, _, _, _) => {
                                        h2318
                                    }
                                };
                                nat_mul(a2319, a2320)?
                            };
                            let a2322 = 4u64;
                            nat_add(a2321, a2322)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2323, _, _, _) => {
                                h2323
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2324 = v1;
    let a2325 = 0u64;
    if if nat_eq(a2324, a2325) {
        true
    } else {
        let a2326 = v2;
        let a2327 = 0u64;
        nat_eq(a2326, a2327)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2328 = v0;
    let a2329 = 4294967295u64;
    if nat_eq(a2328, a2329) {
        true
    } else {
        let a2330 = v0;
        let a2331 = 2147483648u64;
        if nat_lt(a2330, a2331) {
            let a2332 = v0;
            let a2333 = v1;
            nat_lt(a2332, a2333)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2340 = v3;
        let a2341 = {
            let a2337 = f89({
                let a2334 = v0;
                let a2335 = 2u64;
                let a2336 = 0u64;
                nat_quot(a2334, a2335, a2336)
            }, v1)?;
            let a2338 = v2;
            let a2339 = 0u64;
            nat_quot(a2337, a2338, a2339)
        };
        nat_add(a2340, a2341)?
    };
    let a2345 = {
        let a2342 = v0;
        let a2343 = 2u64;
        let a2344 = 0u64;
        nat_rem(a2342, a2343, a2344)
    };
    let a2346 = 0u64;
    if nat_eq(a2345, a2346) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2347 = v4;
        let a2348 = 0u64;
        if nat_eq(a2347, a2348) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2351 = 8u64;
    let a2352 = {
        let a2349 = f147(v0);
        let a2350 = v1;
        nat_mul(a2349, a2350)?
    };
    nat_add(a2351, a2352)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2353, _, _, _, _, _, _, _) => {
            h2353
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2356 = 1u64;
        let a2357 = {
            let a2354 = v6;
            let a2355 = v7;
            nat_mul(a2354, a2355)?
        };
        nat_add(a2356, a2357)?
    };
    let a2358 = v0;
    let a2359 = 0u64;
    if nat_eq(a2358, a2359) {
        Ok(f148(0u64))
    } else {
        let a2360 = v0;
        let a2361 = v8;
        if nat_lt(a2360, a2361) {
            let v9: u64 = {
                let a2362 = v0;
                let a2363 = 1u64;
                nat_sub(a2362, a2363)
            };
            let v10: u64 = {
                let a2364 = v9;
                let a2365 = v7;
                let a2366 = 0u64;
                nat_rem(a2364, a2365, a2366)
            };
            let v11: u64 = f89({
                let a2372 = {
                    let a2367 = v1;
                    let a2368 = v6;
                    nat_mul(a2367, a2368)?
                };
                let a2373 = {
                    let a2369 = v9;
                    let a2370 = v7;
                    let a2371 = 0u64;
                    nat_quot(a2369, a2370, a2371)
                };
                nat_add(a2372, a2373)?
            }, v2)?;
            let a2374 = v3;
            let a2375 = 30u64;
            if nat_eq(a2374, a2375) {
                let a2376 = v10;
                let a2377 = 0u64;
                if nat_eq(a2376, a2377) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2378 = v10;
                    let a2379 = 1u64;
                    if nat_eq(a2378, a2379) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2380 = v10;
                        let a2381 = 2u64;
                        if nat_eq(a2380, a2381) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2382 = v10;
                let a2383 = 0u64;
                if nat_eq(a2382, a2383) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2384 = v10;
                    let a2385 = 1u64;
                    if nat_eq(a2384, a2385) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2386 = v0;
                let a2387 = v8;
                nat_sub(a2386, a2387)
            };
            let a2388 = v12;
            let a2389 = 0u64;
            if nat_eq(a2388, a2389) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2390 = v12;
                let a2391 = 1u64;
                if nat_eq(a2390, a2391) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2392 = v12;
                    let a2393 = 2u64;
                    if nat_eq(a2392, a2393) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2394 = v12;
                        let a2395 = 3u64;
                        if nat_eq(a2394, a2395) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2396 = v12;
                            let a2397 = 4u64;
                            if nat_eq(a2396, a2397) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2398 = v12;
                                let a2399 = 5u64;
                                if nat_eq(a2398, a2399) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2400 = v12;
                                    let a2401 = 6u64;
                                    if nat_eq(a2400, a2401) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2402 = 3u64;
                                                let a2403 = v1;
                                                nat_add(a2402, a2403)?
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
        let a2404 = v0;
        let a2405 = f65(v3);
        let a2406 = 0u64;
        nat_quot(a2404, a2405, a2406)
    };
    let v5: u64 = {
        let a2411 = 3u64;
        let a2412 = {
            let a2408 = v4;
            let a2409 = match v1.clone() {
                Adt3::C0(h2407, _, _, _, _, _, _, _) => {
                    h2407
                }
            };
            let a2410 = 0u64;
            nat_quot(a2408, a2409, a2410)
        };
        nat_add(a2411, a2412)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2413 = v3;
    let a2414 = 30u64;
    if nat_eq(a2413, a2414) {
        let a2418 = {
            let a2415 = v0;
            let a2416 = 3u64;
            let a2417 = 0u64;
            nat_rem(a2415, a2416, a2417)
        };
        let a2419 = 0u64;
        if nat_eq(a2418, a2419) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2423 = {
                let a2420 = v0;
                let a2421 = 3u64;
                let a2422 = 0u64;
                nat_rem(a2420, a2421, a2422)
            };
            let a2424 = 1u64;
            if nat_eq(a2423, a2424) {
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
        let a2425 = v0;
        let a2426 = 8388608u64;
        let a2427 = 0u64;
        nat_quot(a2425, a2426, a2427)
    };
    let a2428 = v0;
    let a2429 = 2147483648u64;
    if if if nat_lt(a2428, a2429) {
        let a2430 = 127u64;
        let a2431 = v1;
        nat_le(a2430, a2431)
    } else {
        false
    } {
        let a2432 = v1;
        let a2433 = 190u64;
        nat_lt(a2432, a2433)
    } else {
        false
    } {
        let v2: u64 = {
            let a2437 = 8388608u64;
            let a2438 = {
                let a2434 = v0;
                let a2435 = 8388608u64;
                let a2436 = 0u64;
                nat_rem(a2434, a2435, a2436)
            };
            nat_add(a2437, a2438)?
        };
        let a2439 = v1;
        let a2440 = 150u64;
        if nat_lt(a2439, a2440) {
            let v3: u64 = f150({
                let a2441 = 150u64;
                let a2442 = v1;
                nat_sub(a2441, a2442)
            });
            let a2446 = {
                let a2443 = v2;
                let a2444 = v3;
                let a2445 = 0u64;
                nat_rem(a2443, a2444, a2445)
            };
            let a2447 = 0u64;
            if nat_eq(a2446, a2447) {
                let a2448 = v2;
                let a2449 = v3;
                let a2450 = 0u64;
                Ok(nat_quot(a2448, a2449, a2450))
            } else {
                Ok(0u64)
            }
        } else {
            let a2453 = f150({
                let a2451 = v1;
                let a2452 = 150u64;
                nat_sub(a2451, a2452)
            });
            let a2454 = v2;
            nat_mul(a2453, a2454)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2455 = v0;
    let a2456 = 1u64;
    if nat_le(a2455, a2456) {
        true
    } else {
        let a2460 = {
            let a2457 = v1;
            let a2458 = v2;
            let a2459 = 1u64;
            nat_rem(a2457, a2458, a2459)
        };
        let a2461 = 0u64;
        nat_eq(a2460, a2461)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2462 = v1;
    let a2463 = 1u64;
    if if if nat_le(a2462, a2463) {
        true
    } else {
        let a2464 = v0;
        let a2465 = v2;
        nat_le(a2464, a2465)
    } {
        let a2466 = v3;
        let a2467 = 1u64;
        if nat_le(a2466, a2467) {
            true
        } else {
            let a2474 = {
                let a2472 = v0;
                let a2473 = {
                    let a2470 = {
                        let a2468 = v1;
                        let a2469 = 1u64;
                        nat_sub(a2468, a2469)
                    };
                    let a2471 = v2;
                    nat_mul(a2470, a2471)?
                };
                nat_add(a2472, a2473)?
            };
            let a2475 = v4;
            nat_le(a2474, a2475)
        }
    } else {
        false
    } {
        let a2476 = v5;
        let a2477 = 1u64;
        if nat_le(a2476, a2477) {
            Ok(true)
        } else {
            let a2490 = {
                let a2488 = {
                    let a2482 = v0;
                    let a2483 = {
                        let a2480 = {
                            let a2478 = v1;
                            let a2479 = 1u64;
                            nat_sub(a2478, a2479)
                        };
                        let a2481 = v2;
                        nat_mul(a2480, a2481)?
                    };
                    nat_add(a2482, a2483)?
                };
                let a2489 = {
                    let a2486 = {
                        let a2484 = v3;
                        let a2485 = 1u64;
                        nat_sub(a2484, a2485)
                    };
                    let a2487 = v4;
                    nat_mul(a2486, a2487)?
                };
                nat_add(a2488, a2489)?
            };
            let a2491 = v6;
            Ok(nat_le(a2490, a2491))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2492 = v5;
        let a2493 = 26u64;
        nat_eq(a2492, a2493)
    } else {
        false
    } {
        let a2494 = v3;
        let a2495 = v7;
        nat_eq(a2494, a2495)
    } else {
        false
    } {
        let a2496 = v4;
        let a2497 = v8;
        nat_eq(a2496, a2497)
    } else {
        false
    } {
        let a2498 = v9;
        let a2499 = 1u64;
        nat_eq(a2498, a2499)
    } else {
        false
    } {
        let a2500 = v10;
        let a2501 = f152(v0);
        nat_eq(a2500, a2501)
    } else {
        false
    } {
        let a2502 = v11;
        let a2503 = v1;
        nat_eq(a2502, a2503)
    } else {
        false
    } {
        let a2504 = v12;
        let a2505 = v6;
        nat_eq(a2504, a2505)
    } else {
        false
    } {
        let a2506 = v13;
        let a2507 = v7;
        nat_eq(a2506, a2507)
    } else {
        false
    } {
        let a2508 = v14;
        let a2509 = v8;
        nat_eq(a2508, a2509)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2510 = v0;
    let a2511 = 0u64;
    if if if nat_eq(a2510, a2511) {
        true
    } else {
        let a2512 = v1;
        let a2513 = 0u64;
        nat_eq(a2512, a2513)
    } {
        true
    } else {
        let a2514 = v2;
        let a2515 = 0u64;
        nat_eq(a2514, a2515)
    } {
        true
    } else {
        let a2516 = v3;
        let a2517 = 0u64;
        nat_eq(a2516, a2517)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2518 = v0;
    let a2519 = 2147483648u64;
    if nat_lt(a2518, a2519) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2520 = v0;
    let a2521 = 2147483648u64;
    if nat_lt(a2520, a2521) {
        0u64
    } else {
        let a2522 = 4294967296u64;
        let a2523 = v0;
        nat_sub(a2522, a2523)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2525 = match v0.clone() {
            Adt3::C0(h2524, _, _, _, _, _, _, _) => {
                h2524
            }
        };
        let a2526 = 1u64;
        nat_sub(a2525, a2526)
    }, {
        let a2528 = match v0.clone() {
            Adt3::C0(_, h2527, _, _, _, _, _, _) => {
                h2527
            }
        };
        let a2529 = 1u64;
        nat_sub(a2528, a2529)
    }, {
        let a2531 = match v0.clone() {
            Adt3::C0(_, _, h2530, _, _, _, _, _) => {
                h2530
            }
        };
        let a2532 = 1u64;
        nat_sub(a2531, a2532)
    }, {
        let a2534 = match v0 {
            Adt3::C0(_, _, _, h2533, _, _, _, _) => {
                h2533
            }
        };
        let a2535 = 1u64;
        nat_sub(a2534, a2535)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2536 = v0;
    let a2537 = 0u64;
    if nat_eq(a2536, a2537) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2538 = v0;
            let a2539 = 1u64;
            nat_sub(a2538, a2539)
        };
        let v5: u64 = {
            let a2540 = v4;
            let a2541 = v1;
            let a2542 = 0u64;
            nat_quot(a2540, a2541, a2542)
        };
        let v6: u64 = {
            let a2550 = {
                let a2543 = v5;
                let a2544 = v2;
                nat_mul(a2543, a2544)?
            };
            let a2551 = {
                let a2548 = {
                    let a2545 = v4;
                    let a2546 = v1;
                    let a2547 = 0u64;
                    nat_rem(a2545, a2546, a2547)
                };
                let a2549 = v3;
                nat_mul(a2548, a2549)?
            };
            nat_add(a2550, a2551)?
        };
        let a2552 = v5;
        let a2553 = 0u64;
        if nat_eq(a2552, a2553) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2562 = {
                    let a2556 = {
                        let a2554 = v5;
                        let a2555 = 1u64;
                        nat_sub(a2554, a2555)
                    };
                    let a2557 = v2;
                    nat_mul(a2556, a2557)?
                };
                let a2563 = {
                    let a2560 = {
                        let a2558 = v1;
                        let a2559 = 1u64;
                        nat_sub(a2558, a2559)
                    };
                    let a2561 = v3;
                    nat_mul(a2560, a2561)?
                };
                nat_add(a2562, a2563)?
            };
            let a2564 = v6;
            let a2565 = v7;
            if nat_lt(a2564, a2565) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2566 = v1;
    let a2567 = 0u64;
    if !nat_eq(a2566, a2567) {
        true
    } else {
        let a2569 = match v0 {
            Adt3::C0(_, _, _, _, h2568, _, _, _) => {
                h2568
            }
        };
        let a2570 = 4u64;
        nat_eq(a2569, a2570)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2573 = {
        let a2571 = v0;
        let a2572 = 2u64;
        nat_mul(a2571, a2572)?
    };
    let a2574 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2573, a2574)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2575 = f154(v0, v1);
    match m2575 {
        None => {
            if v5 {
                let m2576 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2576 {
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
                let a2577 = 4u64;
                let a2578 = v28;
                nat_le(a2577, a2578)
            };
            let a2579 = v2;
            let a2580 = 0u64;
            if if if if if nat_eq(a2579, a2580) {
                let a2581 = v4;
                let a2582 = 0u64;
                nat_eq(a2581, a2582)
            } else {
                false
            } {
                let a2583 = v6;
                let a2584 = v14;
                if nat_eq(a2583, a2584) {
                    let a2585 = v7;
                    let a2586 = v15;
                    nat_eq(a2585, a2586)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2587 = v8;
                let a2588 = v16;
                if nat_eq(a2587, a2588) {
                    let a2589 = v9;
                    let a2590 = v17;
                    nat_eq(a2589, a2590)
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
                        let a2591 = v3;
                        let a2592 = 0u64;
                        nat_eq(a2591, a2592)
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
                    let a2597 = {
                        let a2593 = v6;
                        let a2594 = v7;
                        nat_mul(a2593, a2594)?
                    };
                    let a2598 = {
                        let a2595 = v8;
                        let a2596 = v9;
                        nat_mul(a2595, a2596)?
                    };
                    nat_mul(a2597, a2598)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2599 = v0;
    let a2600 = 0u64;
    if nat_eq(a2599, a2600) {
        Adt8::C0(0u64, v1)
    } else {
        let a2601 = v0;
        let a2602 = 1u64;
        if nat_eq(a2601, a2602) {
            Adt8::C1(v1)
        } else {
            let a2603 = v0;
            let a2604 = 2u64;
            if nat_eq(a2603, a2604) {
                Adt8::C2(v1)
            } else {
                let a2605 = v0;
                let a2606 = 3u64;
                if nat_eq(a2605, a2606) {
                    Adt8::C3(v1)
                } else {
                    let a2607 = v0;
                    let a2608 = 13u64;
                    if nat_lt(a2607, a2608) {
                        Adt8::C0({
                            let a2609 = v0;
                            let a2610 = 3u64;
                            nat_sub(a2609, a2610)
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
    let a2611 = v0;
    let a2612 = 1u64;
    if nat_eq(a2611, a2612) {
        Adt9::C0(v1)
    } else {
        let a2613 = v0;
        let a2614 = 3u64;
        if if nat_eq(a2613, a2614) {
            true
        } else {
            let a2615 = v0;
            let a2616 = 6u64;
            nat_eq(a2615, a2616)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2617 = f70(v0);
    match m2617 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2619 = match v1.clone() {
                Adt11::C0(_, h2618, _) => {
                    h2618
                }
            };
            let a2620 = 1u64;
            if if nat_eq(a2619, a2620) {
                let a2622 = match v1.clone() {
                    Adt11::C0(_, _, h2621) => {
                        h2621
                    }
                };
                let a2623 = 1u64;
                if if if nat_eq(a2622, a2623) {
                    true
                } else {
                    let a2625 = match v1.clone() {
                        Adt11::C0(_, _, h2624) => {
                            h2624
                        }
                    };
                    let a2626 = 2u64;
                    nat_eq(a2625, a2626)
                } {
                    true
                } else {
                    let a2628 = match v1.clone() {
                        Adt11::C0(_, _, h2627) => {
                            h2627
                        }
                    };
                    let a2629 = 4u64;
                    nat_eq(a2628, a2629)
                } {
                    true
                } else {
                    let a2631 = match v1.clone() {
                        Adt11::C0(_, _, h2630) => {
                            h2630
                        }
                    };
                    let a2632 = 8u64;
                    nat_eq(a2631, a2632)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2633) => {
                        h2633
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2646 = {
        let a2642 = {
            let a2638 = {
                let a2634 = v0;
                let a2635 = v4;
                nat_mul(a2634, a2635)?
            };
            let a2639 = {
                let a2636 = v1;
                let a2637 = v5;
                nat_mul(a2636, a2637)?
            };
            nat_add(a2638, a2639)?
        };
        let a2643 = {
            let a2640 = v2;
            let a2641 = v6;
            nat_mul(a2640, a2641)?
        };
        nat_add(a2642, a2643)?
    };
    let a2647 = {
        let a2644 = v3;
        let a2645 = v7;
        nat_mul(a2644, a2645)?
    };
    nat_add(a2646, a2647)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2652 = {
        let a2650 = {
            let a2648 = v0;
            let a2649 = v1;
            nat_sub(a2648, a2649)
        };
        let a2651 = 1u64;
        nat_sub(a2650, a2651)
    };
    let a2653 = v2;
    nat_add(a2652, a2653)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2654 = v0;
        let a2655 = v4;
        let a2656 = 0u64;
        nat_rem(a2654, a2655, a2656)
    }, {
        let a2657 = v1;
        let a2658 = v5;
        let a2659 = 0u64;
        nat_rem(a2657, a2658, a2659)
    }, {
        let a2660 = v2;
        let a2661 = v6;
        let a2662 = 0u64;
        nat_rem(a2660, a2661, a2662)
    }, {
        let a2663 = v3;
        let a2664 = v7;
        let a2665 = 0u64;
        nat_rem(a2663, a2664, a2665)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2667 = v0;
        let a2668 = match v1.clone() {
            Adt3::C0(h2666, _, _, _, _, _, _, _) => {
                h2666
            }
        };
        let a2669 = 0u64;
        nat_quot(a2667, a2668, a2669)
    };
    let v4: u64 = {
        let a2671 = v3;
        let a2672 = match v1.clone() {
            Adt3::C0(_, h2670, _, _, _, _, _, _) => {
                h2670
            }
        };
        let a2673 = 0u64;
        nat_quot(a2671, a2672, a2673)
    };
    let a2674 = v2;
    let a2675 = 0u64;
    if nat_eq(a2674, a2675) {
        let a2677 = v0;
        let a2678 = match v1 {
            Adt3::C0(h2676, _, _, _, _, _, _, _) => {
                h2676
            }
        };
        let a2679 = 0u64;
        nat_rem(a2677, a2678, a2679)
    } else {
        let a2680 = v2;
        let a2681 = 1u64;
        if nat_eq(a2680, a2681) {
            let a2683 = v3;
            let a2684 = match v1 {
                Adt3::C0(_, h2682, _, _, _, _, _, _) => {
                    h2682
                }
            };
            let a2685 = 0u64;
            nat_rem(a2683, a2684, a2685)
        } else {
            let a2686 = v2;
            let a2687 = 2u64;
            if nat_eq(a2686, a2687) {
                let a2689 = v4;
                let a2690 = match v1 {
                    Adt3::C0(_, _, h2688, _, _, _, _, _) => {
                        h2688
                    }
                };
                let a2691 = 0u64;
                nat_rem(a2689, a2690, a2691)
            } else {
                let a2693 = v4;
                let a2694 = match v1 {
                    Adt3::C0(_, _, h2692, _, _, _, _, _) => {
                        h2692
                    }
                };
                let a2695 = 0u64;
                nat_quot(a2693, a2694, a2695)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2696, _, _, _) => {
            h2696
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2697, _, _) => {
            h2697
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2698, _) => {
            h2698
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2699) => {
            h2699
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2700 = v1;
    let a2701 = 2147483648u64;
    if nat_lt(a2700, a2701) {
        let a2712 = {
            let a2710 = {
                let a2705 = {
                    let a2702 = v0;
                    let a2703 = v2;
                    let a2704 = 0u64;
                    nat_rem(a2702, a2703, a2704)
                };
                let a2706 = v2;
                nat_add(a2705, a2706)?
            };
            let a2711 = {
                let a2707 = v1;
                let a2708 = v2;
                let a2709 = 0u64;
                nat_rem(a2707, a2708, a2709)
            };
            nat_sub(a2710, a2711)
        };
        let a2713 = v2;
        let a2714 = 0u64;
        Ok(nat_rem(a2712, a2713, a2714))
    } else {
        let a2717 = {
            let a2715 = v0;
            let a2716 = f120(v1);
            nat_add(a2715, a2716)?
        };
        let a2718 = v2;
        let a2719 = 0u64;
        Ok(nat_rem(a2717, a2718, a2719))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2722 = f119(v1);
    let a2723 = {
        let a2720 = v0;
        let a2721 = f120(v1);
        nat_add(a2720, a2721)?
    };
    if nat_le(a2722, a2723) {
        let a2724 = f136(v0, v1)?;
        let a2725 = v2;
        Ok(nat_lt(a2724, a2725))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2728 = {
        let a2726 = v0;
        let a2727 = f120(v1);
        nat_add(a2726, a2727)?
    };
    let a2729 = f119(v1);
    Ok(nat_sub(a2728, a2729))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2730 = v0;
    let a2731 = v1;
    if nat_lt(a2730, a2731) {
        let a2732 = v1;
        let a2733 = v0;
        Ok(nat_sub(a2732, a2733))
    } else {
        let a2736 = v0;
        let a2737 = {
            let a2734 = v1;
            let a2735 = v2;
            nat_add(a2734, a2735)?
        };
        if nat_lt(a2736, a2737) {
            let a2738 = v0;
            let a2739 = v1;
            Ok(nat_sub(a2738, a2739))
        } else {
            let a2746 = {
                let a2742 = 2u64;
                let a2743 = {
                    let a2740 = v2;
                    let a2741 = 1u64;
                    nat_sub(a2740, a2741)
                };
                nat_mul(a2742, a2743)?
            };
            let a2747 = {
                let a2744 = v0;
                let a2745 = v1;
                nat_sub(a2744, a2745)
            };
            Ok(nat_sub(a2746, a2747))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2750 = {
        let a2748 = v2;
        let a2749 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2748, a2749)?
    };
    let a2751 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2750, a2751))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2752 = v0;
    let a2753 = 0u64;
    if nat_eq(a2752, a2753) {
        let a2754 = v2;
        let a2755 = v1;
        nat_le(a2754, a2755)
    } else {
        let a2756 = v0;
        let a2757 = 1u64;
        if nat_eq(a2756, a2757) {
            let a2758 = v2;
            let a2759 = v1;
            nat_lt(a2758, a2759)
        } else {
            let a2760 = v0;
            let a2761 = 2u64;
            if nat_eq(a2760, a2761) {
                let a2762 = v1;
                let a2763 = v2;
                nat_le(a2762, a2763)
            } else {
                let a2764 = v0;
                let a2765 = 3u64;
                if nat_eq(a2764, a2765) {
                    let a2766 = v1;
                    let a2767 = v2;
                    nat_lt(a2766, a2767)
                } else {
                    let a2768 = v1;
                    let a2769 = v2;
                    nat_eq(a2768, a2769)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2770 = v0;
    let a2771 = 0u64;
    if nat_eq(a2770, a2771) {
        Ok(f148(0u64))
    } else {
        let a2774 = {
            let a2772 = v0;
            let a2773 = 1u64;
            nat_add(a2772, a2773)?
        };
        let a2775 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2774, a2775) {
            Ok(Adt5::C4({
                let a2776 = v2;
                let a2777 = 0u64;
                if nat_eq(a2776, a2777) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2778 = v0;
                let a2779 = 1u64;
                nat_sub(a2778, a2779)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2780, _, _, _, _, _, _, _) => {
                        h2780
                    }
                }, false)?;
                let v5: u64 = {
                    let a2781 = v3;
                    let a2782 = v4;
                    let a2783 = 0u64;
                    nat_quot(a2781, a2782, a2783)
                };
                let v6: u64 = {
                    let a2784 = v3;
                    let a2785 = v4;
                    let a2786 = 0u64;
                    nat_rem(a2784, a2785, a2786)
                };
                let a2789 = {
                    let a2787 = v6;
                    let a2788 = 1u64;
                    nat_add(a2787, a2788)?
                };
                let a2790 = v4;
                if nat_eq(a2789, a2790) {
                    Ok(Adt5::C4({
                        let a2791 = v2;
                        let a2792 = 0u64;
                        if nat_eq(a2791, a2792) {
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
                    let a2793 = v3;
                    let a2794 = 2u64;
                    let a2795 = 0u64;
                    nat_quot(a2793, a2794, a2795)
                }, {
                    let a2796 = v3;
                    let a2797 = 2u64;
                    let a2798 = 0u64;
                    nat_rem(a2796, a2797, a2798)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2799 = v0;
    let a2800 = 0u64;
    if nat_eq(a2799, a2800) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2809 = {
                let a2802 = v1;
                let a2803 = match v2.clone() {
                    Adt3::C0(h2801, _, _, _, _, _, _, _) => {
                        h2801
                    }
                };
                nat_mul(a2802, a2803)?
            };
            let a2810 = {
                let a2806 = {
                    let a2804 = v0;
                    let a2805 = 1u64;
                    nat_sub(a2804, a2805)
                };
                let a2807 = 2u64;
                let a2808 = 0u64;
                nat_quot(a2806, a2807, a2808)
            };
            nat_add(a2809, a2810)?
        }, {
            let a2813 = {
                let a2811 = v0;
                let a2812 = 1u64;
                nat_sub(a2811, a2812)
            };
            let a2814 = 2u64;
            let a2815 = 0u64;
            nat_rem(a2813, a2814, a2815)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2816 = v0;
    let a2817 = f60(v1)?;
    let a2818 = 0u64;
    Ok(nat_rem(a2816, a2817, a2818))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2819 = 4u64;
    let a2820 = f157(v0, v1)?;
    nat_mul(a2819, a2820)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2821 = 4u64;
    let a2822 = f158(v0, v1)?;
    nat_mul(a2821, a2822)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2828 = {
        let a2825 = {
            let a2823 = f142(v0, v1)?;
            let a2824 = 2u64;
            nat_sub(a2823, a2824)
        };
        let a2826 = 2u64;
        let a2827 = 0u64;
        nat_rem(a2825, a2826, a2827)
    };
    let a2829 = 0u64;
    Ok(nat_eq(a2828, a2829))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2830 = f70(v0);
    match m2830 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2831 = v1;
                let a2832 = v7;
                nat_lt(a2831, a2832)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2833 = v3;
                let a2834 = f160(v4, v5, v6)?;
                nat_lt(a2833, a2834)
            } else {
                false
            } {
                let a2839 = {
                    let a2836 = v7;
                    let a2837 = match v13.clone() {
                        Adt11::C0(_, h2835, _) => {
                            h2835
                        }
                    };
                    let a2838 = 0u64;
                    nat_rem(a2836, a2837, a2838)
                };
                let a2840 = 0u64;
                nat_eq(a2839, a2840)
            } else {
                false
            } {
                Ok(Some({
                    let a2869 = {
                        let a2859 = {
                            let a2849 = {
                                let a2845 = {
                                    let a2842 = v1;
                                    let a2843 = match v13 {
                                        Adt11::C0(_, h2841, _) => {
                                            h2841
                                        }
                                    };
                                    let a2844 = 0u64;
                                    nat_quot(a2842, a2843, a2844)
                                };
                                let a2846 = v9;
                                nat_mul(a2845, a2846)?
                            };
                            let a2850 = {
                                let a2847 = v2;
                                let a2848 = v10;
                                nat_mul(a2847, a2848)?
                            };
                            nat_add(a2849, a2850)?
                        };
                        let a2860 = {
                            let a2857 = {
                                let a2854 = {
                                    let a2851 = v3;
                                    let a2852 = v4;
                                    let a2853 = 0u64;
                                    nat_quot(a2851, a2852, a2853)
                                };
                                let a2855 = v5;
                                let a2856 = 0u64;
                                nat_rem(a2854, a2855, a2856)
                            };
                            let a2858 = v11;
                            nat_mul(a2857, a2858)?
                        };
                        nat_add(a2859, a2860)?
                    };
                    let a2870 = {
                        let a2867 = {
                            let a2864 = {
                                let a2861 = v3;
                                let a2862 = v4;
                                let a2863 = 0u64;
                                nat_quot(a2861, a2862, a2863)
                            };
                            let a2865 = v5;
                            let a2866 = 0u64;
                            nat_quot(a2864, a2865, a2866)
                        };
                        let a2868 = v12;
                        nat_mul(a2867, a2868)?
                    };
                    nat_add(a2869, a2870)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2871 = v0;
    let a2872 = 30u64;
    if nat_eq(a2871, a2872) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2873 = 0u64;
    let a2874 = v0;
    if nat_lt(a2873, a2874) {
        let a2875 = v0;
        let a2876 = 2139095040u64;
        nat_le(a2875, a2876)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2877 = v0;
    let a2878 = 40u64;
    if nat_lt(a2877, a2878) {
        let a2879 = v0;
        let a2880 = 20u64;
        if nat_lt(a2879, a2880) {
            let a2881 = v0;
            let a2882 = 10u64;
            if nat_lt(a2881, a2882) {
                let a2883 = v0;
                let a2884 = 5u64;
                if nat_lt(a2883, a2884) {
                    let a2885 = v0;
                    let a2886 = 2u64;
                    if nat_lt(a2885, a2886) {
                        let a2887 = v0;
                        let a2888 = 1u64;
                        if nat_lt(a2887, a2888) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2889 = v0;
                        let a2890 = 3u64;
                        if nat_lt(a2889, a2890) {
                            4u64
                        } else {
                            let a2891 = v0;
                            let a2892 = 4u64;
                            if nat_lt(a2891, a2892) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2893 = v0;
                    let a2894 = 7u64;
                    if nat_lt(a2893, a2894) {
                        let a2895 = v0;
                        let a2896 = 6u64;
                        if nat_lt(a2895, a2896) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2897 = v0;
                        let a2898 = 8u64;
                        if nat_lt(a2897, a2898) {
                            128u64
                        } else {
                            let a2899 = v0;
                            let a2900 = 9u64;
                            if nat_lt(a2899, a2900) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2901 = v0;
                let a2902 = 15u64;
                if nat_lt(a2901, a2902) {
                    let a2903 = v0;
                    let a2904 = 12u64;
                    if nat_lt(a2903, a2904) {
                        let a2905 = v0;
                        let a2906 = 11u64;
                        if nat_lt(a2905, a2906) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2907 = v0;
                        let a2908 = 13u64;
                        if nat_lt(a2907, a2908) {
                            4096u64
                        } else {
                            let a2909 = v0;
                            let a2910 = 14u64;
                            if nat_lt(a2909, a2910) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2911 = v0;
                    let a2912 = 17u64;
                    if nat_lt(a2911, a2912) {
                        let a2913 = v0;
                        let a2914 = 16u64;
                        if nat_lt(a2913, a2914) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2915 = v0;
                        let a2916 = 18u64;
                        if nat_lt(a2915, a2916) {
                            131072u64
                        } else {
                            let a2917 = v0;
                            let a2918 = 19u64;
                            if nat_lt(a2917, a2918) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2919 = v0;
            let a2920 = 30u64;
            if nat_lt(a2919, a2920) {
                let a2921 = v0;
                let a2922 = 25u64;
                if nat_lt(a2921, a2922) {
                    let a2923 = v0;
                    let a2924 = 22u64;
                    if nat_lt(a2923, a2924) {
                        let a2925 = v0;
                        let a2926 = 21u64;
                        if nat_lt(a2925, a2926) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2927 = v0;
                        let a2928 = 23u64;
                        if nat_lt(a2927, a2928) {
                            4194304u64
                        } else {
                            let a2929 = v0;
                            let a2930 = 24u64;
                            if nat_lt(a2929, a2930) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2931 = v0;
                    let a2932 = 27u64;
                    if nat_lt(a2931, a2932) {
                        let a2933 = v0;
                        let a2934 = 26u64;
                        if nat_lt(a2933, a2934) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2935 = v0;
                        let a2936 = 28u64;
                        if nat_lt(a2935, a2936) {
                            134217728u64
                        } else {
                            let a2937 = v0;
                            let a2938 = 29u64;
                            if nat_lt(a2937, a2938) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2939 = v0;
                let a2940 = 35u64;
                if nat_lt(a2939, a2940) {
                    let a2941 = v0;
                    let a2942 = 32u64;
                    if nat_lt(a2941, a2942) {
                        let a2943 = v0;
                        let a2944 = 31u64;
                        if nat_lt(a2943, a2944) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2945 = v0;
                        let a2946 = 33u64;
                        if nat_lt(a2945, a2946) {
                            4294967296u64
                        } else {
                            let a2947 = v0;
                            let a2948 = 34u64;
                            if nat_lt(a2947, a2948) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2949 = v0;
                    let a2950 = 37u64;
                    if nat_lt(a2949, a2950) {
                        let a2951 = v0;
                        let a2952 = 36u64;
                        if nat_lt(a2951, a2952) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2953 = v0;
                        let a2954 = 38u64;
                        if nat_lt(a2953, a2954) {
                            137438953472u64
                        } else {
                            let a2955 = v0;
                            let a2956 = 39u64;
                            if nat_lt(a2955, a2956) {
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
    let a2957 = v0;
    let a2958 = 26u64;
    if nat_eq(a2957, a2958) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2979 = {
        let a2973 = {
            let a2967 = {
                let a2961 = v1;
                let a2962 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2959, _, _, _) => {
                            h2959
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2960, _, _, _) => {
                            h2960
                        }
                    })
                };
                nat_mul(a2961, a2962)?
            };
            let a2968 = {
                let a2965 = v2;
                let a2966 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2963, _, _) => {
                            h2963
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2964, _, _) => {
                            h2964
                        }
                    })
                };
                nat_mul(a2965, a2966)?
            };
            nat_add(a2967, a2968)?
        };
        let a2974 = {
            let a2971 = v3;
            let a2972 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2969, _) => {
                        h2969
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2970, _) => {
                        h2970
                    }
                })
            };
            nat_mul(a2971, a2972)?
        };
        nat_add(a2973, a2974)?
    };
    let a2980 = {
        let a2977 = v4;
        let a2978 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2975) => {
                    h2975
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2976) => {
                    h2976
                }
            })
        };
        nat_mul(a2977, a2978)?
    };
    nat_add(a2979, a2980)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a2981 = v0;
    let a2982 = 2u64;
    if nat_eq(a2981, a2982) {
        Some(0u64)
    } else {
        let a2983 = v0;
        let a2984 = 5u64;
        if nat_eq(a2983, a2984) {
            Some(1u64)
        } else {
            let a2985 = v0;
            let a2986 = 6u64;
            if nat_eq(a2985, a2986) {
                Some(2u64)
            } else {
                let a2987 = v0;
                let a2988 = 7u64;
                if nat_eq(a2987, a2988) {
                    Some(3u64)
                } else {
                    let a2989 = v0;
                    let a2990 = 9u64;
                    if nat_eq(a2989, a2990) {
                        Some(4u64)
                    } else {
                        let a2991 = v0;
                        let a2992 = 8u64;
                        if nat_eq(a2991, a2992) {
                            Some(5u64)
                        } else {
                            let a2993 = v0;
                            let a2994 = 89u64;
                            if nat_eq(a2993, a2994) {
                                let a2995 = v1;
                                let a2996 = 2u64;
                                if nat_eq(a2995, a2996) {
                                    Some(6u64)
                                } else {
                                    let a2997 = v1;
                                    let a2998 = 0u64;
                                    if nat_eq(a2997, a2998) {
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
    let a2999 = v0;
    let a3000 = 28u64;
    if if if if nat_eq(a2999, a3000) {
        let a3001 = v1;
        let a3002 = 0u64;
        nat_eq(a3001, a3002)
    } else {
        false
    } {
        let a3003 = v2;
        let a3004 = 0u64;
        nat_eq(a3003, a3004)
    } else {
        false
    } {
        let a3005 = v3;
        let a3006 = 0u64;
        nat_eq(a3005, a3006)
    } else {
        false
    } {
        let a3007 = v4;
        let a3008 = v8;
        if if if if if if if if nat_eq(a3007, a3008) {
            let a3009 = v5;
            let a3010 = v12;
            nat_eq(a3009, a3010)
        } else {
            false
        } {
            let a3011 = v9;
            let a3012 = v13;
            nat_eq(a3011, a3012)
        } else {
            false
        } {
            let a3013 = v10;
            let a3014 = v14;
            nat_eq(a3013, a3014)
        } else {
            false
        } {
            let a3015 = v11;
            let a3016 = v15;
            nat_eq(a3015, a3016)
        } else {
            false
        } {
            let a3017 = 0u64;
            let a3018 = v6;
            nat_lt(a3017, a3018)
        } else {
            false
        } {
            let a3019 = 0u64;
            let a3020 = v7;
            nat_lt(a3019, a3020)
        } else {
            false
        } {
            let a3024 = {
                let a3021 = v10;
                let a3022 = v6;
                let a3023 = 1u64;
                nat_rem(a3021, a3022, a3023)
            };
            let a3025 = 0u64;
            if nat_eq(a3024, a3025) {
                let a3029 = {
                    let a3026 = v11;
                    let a3027 = v7;
                    let a3028 = 1u64;
                    nat_rem(a3026, a3027, a3028)
                };
                let a3030 = 0u64;
                nat_eq(a3029, a3030)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3031 = v10;
                let a3032 = v6;
                let a3033 = 0u64;
                nat_quot(a3031, a3032, a3033)
            }, {
                let a3034 = v11;
                let a3035 = v7;
                let a3036 = 0u64;
                nat_quot(a3034, a3035, a3036)
            }, {
                let a3037 = v5;
                let a3038 = v4;
                nat_mul(a3037, a3038)?
            }, {
                let a3039 = v4;
                let a3040 = v9;
                nat_mul(a3039, a3040)?
            }, {
                let a3041 = v5;
                let a3042 = v9;
                nat_mul(a3041, a3042)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3043 = v1;
    let a3044 = 0u64;
    if nat_eq(a3043, a3044) {
        Ok(Adt5::C4({
            let a3045 = v3;
            let a3046 = 0u64;
            if nat_eq(a3045, a3046) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3047 = v3;
            let a3048 = 0u64;
            if nat_eq(a3047, a3048) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3049 = v0;
    let a3050 = f60(v1)?;
    let a3051 = 0u64;
    Ok(nat_quot(a3049, a3050, a3051))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3054 = {
            let a3052 = f142(v0, v1)?;
            let a3053 = 2u64;
            nat_sub(a3052, a3053)
        };
        let a3055 = 2u64;
        let a3056 = 0u64;
        nat_quot(a3054, a3055, a3056)
    };
    let v4: u64 = {
        let a3057 = v2;
        let a3058 = v1;
        nat_mul(a3057, a3058)?
    };
    if f145(v0, v1)? {
        let a3059 = v4;
        let a3060 = v3;
        nat_add(a3059, a3060)
    } else {
        let a3072 = {
            let a3068 = v4;
            let a3069 = {
                let a3066 = {
                    let a3061 = v1;
                    let a3062 = 1u64;
                    nat_sub(a3061, a3062)
                };
                let a3067 = {
                    let a3063 = v2;
                    let a3064 = v1;
                    let a3065 = 0u64;
                    nat_rem(a3063, a3064, a3065)
                };
                nat_mul(a3066, a3067)?
            };
            nat_sub(a3068, a3069)
        };
        let a3073 = {
            let a3070 = v3;
            let a3071 = v1;
            nat_mul(a3070, a3071)?
        };
        nat_add(a3072, a3073)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3074 = v0;
    let a3075 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3074, a3075) {
        true
    } else {
        let a3076 = v0;
        let a3077 = 1u64;
        nat_eq(a3076, a3077)
    } {
        true
    } else {
        let a3078 = v0;
        let a3079 = 2u64;
        nat_eq(a3078, a3079)
    } {
        true
    } else {
        let a3080 = v0;
        let a3081 = 3u64;
        nat_eq(a3080, a3081)
    } {
        true
    } else {
        let a3082 = v0;
        let a3083 = 6u64;
        nat_eq(a3082, a3083)
    } {
        true
    } else {
        let a3084 = v0;
        let a3085 = 7u64;
        nat_eq(a3084, a3085)
    } {
        true
    } else {
        let a3086 = v0;
        let a3087 = 8u64;
        nat_eq(a3086, a3087)
    } {
        true
    } else {
        let a3088 = v0;
        let a3089 = 10u64;
        nat_eq(a3088, a3089)
    } {
        true
    } else {
        let a3090 = v0;
        let a3091 = 11u64;
        nat_eq(a3090, a3091)
    } {
        true
    } else {
        let a3092 = v0;
        let a3093 = 12u64;
        nat_eq(a3092, a3093)
    } {
        true
    } else {
        let a3094 = v0;
        let a3095 = 13u64;
        nat_eq(a3094, a3095)
    } {
        true
    } else {
        let a3096 = v0;
        let a3097 = 14u64;
        nat_eq(a3096, a3097)
    } {
        true
    } else {
        let a3098 = v0;
        let a3099 = 16u64;
        nat_eq(a3098, a3099)
    } {
        true
    } else {
        let a3100 = v0;
        let a3101 = 17u64;
        nat_eq(a3100, a3101)
    } {
        true
    } else {
        let a3102 = v0;
        let a3103 = 18u64;
        nat_eq(a3102, a3103)
    } {
        true
    } else {
        let a3104 = v0;
        let a3105 = 19u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 20u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 21u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 22u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 23u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 26u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 29u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 30u64;
        nat_eq(a3118, a3119)
    } {
        true
    } else {
        let a3120 = v0;
        let a3121 = 34u64;
        nat_eq(a3120, a3121)
    } {
        true
    } else {
        let a3122 = v0;
        let a3123 = 35u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v0;
        let a3125 = 39u64;
        nat_eq(a3124, a3125)
    } {
        true
    } else {
        let a3126 = v0;
        let a3127 = 40u64;
        nat_eq(a3126, a3127)
    } {
        true
    } else {
        let a3128 = v0;
        let a3129 = 41u64;
        nat_eq(a3128, a3129)
    } {
        true
    } else {
        let a3130 = v0;
        let a3131 = 42u64;
        nat_eq(a3130, a3131)
    } {
        true
    } else {
        let a3132 = v0;
        let a3133 = 43u64;
        nat_eq(a3132, a3133)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3134 = v0;
    let a3135 = 0u64;
    if if if nat_eq(a3134, a3135) {
        true
    } else {
        let a3136 = v1;
        let a3137 = 0u64;
        nat_eq(a3136, a3137)
    } {
        true
    } else {
        let a3138 = v2;
        let a3139 = 0u64;
        nat_eq(a3138, a3139)
    } {
        Ok(0u64)
    } else {
        let a3142 = {
            let a3140 = v0;
            let a3141 = v1;
            nat_mul(a3140, a3141)?
        };
        let a3143 = v2;
        nat_mul(a3142, a3143)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3144 = f70(v0);
    match m3144 {
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
