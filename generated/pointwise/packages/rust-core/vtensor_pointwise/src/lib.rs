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
            let a1229 = v0;
            let a1230 = 2u64;
            if if if if nat_eq(a1229, a1230) {
                true
            } else {
                let a1231 = v0;
                let a1232 = 5u64;
                nat_eq(a1231, a1232)
            } {
                true
            } else {
                let a1233 = v0;
                let a1234 = 6u64;
                nat_eq(a1233, a1234)
            } {
                true
            } else {
                let a1235 = v0;
                let a1236 = 7u64;
                nat_eq(a1235, a1236)
            } {
                Ok(Adt6::C5(67u64))
            } else {
                Ok(Adt6::C1)
            }
        }
        Adt6::C2(v6, v7, _, _, _, _, v12) => {
            if v7 {
                Ok(Adt6::C4(v12, 4u64, f13(v2.clone(), 1u64), f13(v4, 1u64), Adt4::C10(v6, v7, {
                    let m1237 = v3;
                    match m1237 {
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
    let a1239 = match v1.clone() {
        Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
            h1238
        }
    };
    let a1240 = 0u64;
    if if if if if nat_eq(a1239, a1240) {
        true
    } else {
        let a1242 = match v1.clone() {
            Adt2::C0(h1241, _, _, _, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1243 = 1u64;
        nat_eq(a1242, a1243)
    } {
        true
    } else {
        let a1245 = match v1.clone() {
            Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        };
        let a1246 = 30u64;
        nat_eq(a1245, a1246)
    } {
        let a1249 = match v1.clone() {
            Adt2::C0(h1247, _, _, _, _, _, _, _, _, _, _) => {
                h1247
            }
        };
        let a1250 = match v2.clone() {
            Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
                h1248
            }
        };
        if nat_eq(a1249, a1250) {
            true
        } else {
            let a1252 = match v2.clone() {
                Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                    h1251
                }
            };
            let a1253 = 0u64;
            nat_eq(a1252, a1253)
        }
    } else {
        false
    } {
        f14(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f9(v2.clone(), 1u64)?, {
            let a1255 = match v2.clone() {
                Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                    h1254
                }
            };
            let a1256 = 0u64;
            if nat_eq(a1255, a1256) {
                4u64
            } else {
                2u64
            }
        }, f13(v1.clone(), 1u64), f13(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
                h1257
            }
        }, match v2 {
            Adt2::C0(h1258, _, _, _, _, _, _, _, _, _, _) => {
                h1258
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1259 = v1;
    match m1259 {
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
    let m1260 = v3;
    match m1260 {
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
                Adt3::C0(h1261, _, _, _, _, _, _, _) => {
                    h1261
                }
            }, match v2.clone() {
                Adt3::C0(_, h1262, _, _, _, _, _, _) => {
                    h1262
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
            let a1263 = v51;
            let a1264 = 64u64;
            if if if nat_eq(a1263, a1264) {
                true
            } else {
                let a1265 = v51;
                let a1266 = 65u64;
                nat_eq(a1265, a1266)
            } {
                true
            } else {
                let a1267 = v51;
                let a1268 = 66u64;
                nat_eq(a1267, a1268)
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
        Adt3::C0(h1269, _, _, _, _, _, _, _) => {
            h1269
        }
    })?;
    let v5: u64 = {
        let a1270 = v0;
        let a1271 = v4;
        let a1272 = 0u64;
        nat_quot(a1270, a1271, a1272)
    };
    let v6: u64 = {
        let a1273 = v0;
        let a1274 = v4;
        let a1275 = 0u64;
        nat_rem(a1273, a1274, a1275)
    };
    let a1276 = v6;
    let a1277 = 0u64;
    if nat_eq(a1276, a1277) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1278 = v6;
        let a1279 = 1u64;
        if nat_eq(a1278, a1279) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1282 = {
                let a1280 = v6;
                let a1281 = 1u64;
                nat_add(a1280, a1281)?
            };
            let a1283 = v4;
            if nat_eq(a1282, a1283) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f89(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1284 = v6;
                    let a1285 = 2u64;
                    nat_sub(a1284, a1285)
                };
                let v8: u64 = {
                    let a1286 = v7;
                    let a1287 = 3u64;
                    let a1288 = 0u64;
                    nat_quot(a1286, a1287, a1288)
                };
                let v9: u64 = {
                    let a1289 = v7;
                    let a1290 = 3u64;
                    let a1291 = 0u64;
                    nat_rem(a1289, a1290, a1291)
                };
                let v10: u64 = f89({
                    let a1295 = {
                        let a1293 = v5;
                        let a1294 = match v1.clone() {
                            Adt3::C0(h1292, _, _, _, _, _, _, _) => {
                                h1292
                            }
                        };
                        nat_mul(a1293, a1294)?
                    };
                    let a1296 = v8;
                    nat_add(a1295, a1296)?
                }, v1)?;
                let a1297 = v9;
                let a1298 = 0u64;
                if nat_eq(a1297, a1298) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1299 = v9;
                    let a1300 = 1u64;
                    if nat_eq(a1299, a1300) {
                        let a1301 = v3;
                        let a1302 = 2u64;
                        if nat_eq(a1301, a1302) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1303 = v3;
                        let a1304 = 2u64;
                        if if nat_eq(a1303, a1304) {
                            true
                        } else {
                            let a1305 = v3;
                            let a1306 = 3u64;
                            nat_eq(a1305, a1306)
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
        let a1308 = 2u64;
        let a1309 = match v1.clone() {
            Adt3::C0(_, h1307, _, _, _, _, _, _) => {
                h1307
            }
        };
        nat_mul(a1308, a1309)?
    };
    let a1310 = v0;
    let a1311 = v5;
    if nat_lt(a1310, a1311) {
        let a1315 = {
            let a1312 = v0;
            let a1313 = 2u64;
            let a1314 = 0u64;
            nat_rem(a1312, a1313, a1314)
        };
        let a1316 = 0u64;
        if nat_eq(a1315, a1316) {
            Ok(Adt5::C4(19u64, 4u64, f89({
                let a1317 = v0;
                let a1318 = 2u64;
                let a1319 = 0u64;
                nat_quot(a1317, a1318, a1319)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1320 = v4;
            let a1321 = 2147483648u64;
            if if nat_lt(a1320, a1321) {
                let a1323 = v4;
                let a1324 = match v2 {
                    Adt3::C0(_, h1322, _, _, _, _, _, _) => {
                        h1322
                    }
                };
                nat_lt(a1323, a1324)
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
            let a1327 = match v2.clone() {
                Adt3::C0(h1325, _, _, _, _, _, _, _) => {
                    h1325
                }
            };
            let a1328 = match v2.clone() {
                Adt3::C0(_, h1326, _, _, _, _, _, _) => {
                    h1326
                }
            };
            nat_mul(a1327, a1328)?
        };
        let a1331 = {
            let a1329 = v0;
            let a1330 = v5;
            nat_sub(a1329, a1330)
        };
        let a1332 = v6;
        if nat_lt(a1331, a1332) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f89({
                let a1333 = v0;
                let a1334 = v5;
                nat_sub(a1333, a1334)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1337 = {
                    let a1335 = v0;
                    let a1336 = v5;
                    nat_sub(a1335, a1336)
                };
                let a1338 = v6;
                nat_sub(a1337, a1338)
            };
            let v8: u64 = {
                let a1342 = 1u64;
                let a1343 = {
                    let a1340 = 2u64;
                    let a1341 = match v1.clone() {
                        Adt3::C0(h1339, _, _, _, _, _, _, _) => {
                            h1339
                        }
                    };
                    nat_mul(a1340, a1341)?
                };
                nat_add(a1342, a1343)?
            };
            let v9: u64 = {
                let a1344 = v7;
                let a1345 = v8;
                let a1346 = 0u64;
                nat_quot(a1344, a1345, a1346)
            };
            let v10: u64 = {
                let a1347 = v7;
                let a1348 = v8;
                let a1349 = 0u64;
                nat_rem(a1347, a1348, a1349)
            };
            let a1350 = v10;
            let a1351 = 0u64;
            if nat_eq(a1350, a1351) {
                Ok(Adt5::C4(19u64, 4u64, f89(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1355 = {
                    let a1352 = v10;
                    let a1353 = 2u64;
                    let a1354 = 0u64;
                    nat_rem(a1352, a1353, a1354)
                };
                let a1356 = 1u64;
                if nat_eq(a1355, a1356) {
                    Ok(Adt5::C4(6u64, 0u64, f89({
                        let a1365 = {
                            let a1358 = v9;
                            let a1359 = match v1.clone() {
                                Adt3::C0(h1357, _, _, _, _, _, _, _) => {
                                    h1357
                                }
                            };
                            nat_mul(a1358, a1359)?
                        };
                        let a1366 = {
                            let a1362 = {
                                let a1360 = v10;
                                let a1361 = 1u64;
                                nat_sub(a1360, a1361)
                            };
                            let a1363 = 2u64;
                            let a1364 = 0u64;
                            nat_quot(a1362, a1363, a1364)
                        };
                        nat_add(a1365, a1366)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f89({
                        let a1375 = {
                            let a1368 = v4;
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
                    }, v2.clone())?, 1u64, 0u64, 3u64, f89({
                        let a1385 = {
                            let a1378 = v4;
                            let a1379 = match v1 {
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
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a1387 = v1;
    let a1388 = f104(v2.clone(), v4.clone())?;
    if nat_lt(a1387, a1388) {
        let a1391 = v1;
        let a1392 = {
            let a1389 = 2u64;
            let a1390 = f81(v4.clone())?;
            nat_mul(a1389, a1390)?
        };
        if nat_lt(a1391, a1392) {
            let a1396 = {
                let a1393 = v1;
                let a1394 = 2u64;
                let a1395 = 0u64;
                nat_rem(a1393, a1394, a1395)
            };
            let a1397 = 0u64;
            if nat_eq(a1396, a1397) {
                f105({
                    let a1398 = v1;
                    let a1399 = 2u64;
                    let a1400 = 0u64;
                    nat_quot(a1398, a1399, a1400)
                }, v4)
            } else {
                if f106(match v2 {
                    Adt3::C0(_, h1401, _, _, _, _, _, _) => {
                        h1401
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1404 = v1;
                let a1405 = {
                    let a1402 = 2u64;
                    let a1403 = f81(v4.clone())?;
                    nat_mul(a1402, a1403)?
                };
                nat_sub(a1404, a1405)
            };
            let v7: u64 = {
                let a1409 = v6;
                let a1410 = {
                    let a1407 = 1u64;
                    let a1408 = match v2.clone() {
                        Adt3::C0(h1406, _, _, _, _, _, _, _) => {
                            h1406
                        }
                    };
                    nat_add(a1407, a1408)?
                };
                let a1411 = 0u64;
                nat_quot(a1409, a1410, a1411)
            };
            let v8: u64 = {
                let a1415 = v6;
                let a1416 = {
                    let a1413 = 1u64;
                    let a1414 = match v2.clone() {
                        Adt3::C0(h1412, _, _, _, _, _, _, _) => {
                            h1412
                        }
                    };
                    nat_add(a1413, a1414)?
                };
                let a1417 = 0u64;
                nat_rem(a1415, a1416, a1417)
            };
            let a1418 = v8;
            let a1419 = 0u64;
            if nat_eq(a1418, a1419) {
                f105(v7, v4)
            } else {
                f107(v0, v7, {
                    let a1420 = v8;
                    let a1421 = 1u64;
                    nat_sub(a1420, a1421)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1424 = match v4.clone() {
            Adt3::C0(h1422, _, _, _, _, _, _, _) => {
                h1422
            }
        };
        let a1425 = match v4.clone() {
            Adt3::C0(_, h1423, _, _, _, _, _, _) => {
                h1423
            }
        };
        nat_mul(a1424, a1425)?
    };
    let v7: u64 = {
        let a1426 = 2u64;
        let a1427 = v6;
        nat_mul(a1426, a1427)?
    };
    let v8: u64 = f108(match v3.clone() {
        Adt3::C0(_, h1428, _, _, _, _, _, _) => {
            h1428
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1429, _, _, _, _, _) => {
            h1429
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1430, _, _, _, _) => {
            h1430
        }
    });
    let a1431 = v0;
    let a1432 = v7;
    if nat_lt(a1431, a1432) {
        let a1436 = {
            let a1433 = v0;
            let a1434 = 2u64;
            let a1435 = 0u64;
            nat_rem(a1433, a1434, a1435)
        };
        let a1437 = 0u64;
        if nat_eq(a1436, a1437) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1438 = v0;
                let a1439 = 2u64;
                let a1440 = 0u64;
                nat_quot(a1438, a1439, a1440)
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
            let a1441 = v0;
            let a1442 = v7;
            nat_sub(a1441, a1442)
        };
        let v10: u64 = {
            let a1446 = v9;
            let a1447 = {
                let a1444 = 1u64;
                let a1445 = match v1.clone() {
                    Adt3::C0(h1443, _, _, _, _, _, _, _) => {
                        h1443
                    }
                };
                nat_add(a1444, a1445)?
            };
            let a1448 = 0u64;
            nat_quot(a1446, a1447, a1448)
        };
        let v11: u64 = {
            let a1452 = v9;
            let a1453 = {
                let a1450 = 1u64;
                let a1451 = match v1.clone() {
                    Adt3::C0(h1449, _, _, _, _, _, _, _) => {
                        h1449
                    }
                };
                nat_add(a1450, a1451)?
            };
            let a1454 = 0u64;
            nat_rem(a1452, a1453, a1454)
        };
        let a1455 = v11;
        let a1456 = 0u64;
        if nat_eq(a1455, a1456) {
            Ok(Adt5::C4(19u64, 5u64, f89({
                let a1457 = v10;
                let a1458 = v6;
                let a1459 = 0u64;
                nat_rem(a1457, a1458, a1459)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f109(v5, v8) {
                let v12: u64 = {
                    let a1465 = {
                        let a1461 = v10;
                        let a1462 = match v1.clone() {
                            Adt3::C0(h1460, _, _, _, _, _, _, _) => {
                                h1460
                            }
                        };
                        nat_mul(a1461, a1462)?
                    };
                    let a1466 = {
                        let a1463 = v11;
                        let a1464 = 1u64;
                        nat_sub(a1463, a1464)
                    };
                    nat_add(a1465, a1466)?
                };
                let a1467 = v5;
                let a1468 = 4294967295u64;
                if nat_eq(a1467, a1468) {
                    Ok(Adt5::C1(false, f89(v12, v1)?, f89(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f89(v12, v1)?, 4u64, f89({
                        let a1474 = {
                            let a1470 = v5;
                            let a1471 = match v3.clone() {
                                Adt3::C0(h1469, _, _, _, _, _, _, _) => {
                                    h1469
                                }
                            };
                            nat_mul(a1470, a1471)?
                        };
                        let a1475 = {
                            let a1472 = v11;
                            let a1473 = 1u64;
                            nat_sub(a1472, a1473)
                        };
                        nat_add(a1474, a1475)?
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
        let a1478 = v6;
        let a1479 = {
            let a1476 = 3u64;
            let a1477 = v5;
            nat_add(a1476, a1477)?
        };
        nat_sub(a1478, a1479)
    };
    let a1481 = v0;
    let a1482 = f25(v3, match v1.clone() {
        Adt3::C0(h1480, _, _, _, _, _, _, _) => {
            h1480
        }
    }, v5, v8)?;
    if nat_lt(a1481, a1482) {
        let v9: u64 = f63(match v1.clone() {
            Adt3::C0(h1483, _, _, _, _, _, _, _) => {
                h1483
            }
        }, v5, v8)?;
        let v10: u64 = f64(v3, match v1.clone() {
            Adt3::C0(h1484, _, _, _, _, _, _, _) => {
                h1484
            }
        }, v5)?;
        let a1485 = v0;
        let a1486 = v9;
        if nat_lt(a1485, a1486) {
            f110(v0, v2, f19(v3), {
                let a1487 = 3u64;
                let a1488 = v5;
                nat_add(a1487, a1488)?
            }, v7)
        } else {
            let a1491 = v0;
            let a1492 = {
                let a1489 = v9;
                let a1490 = v10;
                nat_add(a1489, a1490)?
            };
            if nat_lt(a1491, a1492) {
                let v11: u64 = {
                    let a1493 = v0;
                    let a1494 = v9;
                    nat_sub(a1493, a1494)
                };
                let v12: u64 = f111(v3, match v1.clone() {
                    Adt3::C0(h1495, _, _, _, _, _, _, _) => {
                        h1495
                    }
                })?;
                f112({
                    let a1496 = v11;
                    let a1497 = v12;
                    let a1498 = 0u64;
                    nat_rem(a1496, a1497, a1498)
                }, {
                    let a1499 = v11;
                    let a1500 = v12;
                    let a1501 = 0u64;
                    nat_quot(a1499, a1500, a1501)
                }, v1, v3, v4, v7)
            } else {
                f113({
                    let a1504 = v0;
                    let a1505 = {
                        let a1502 = v9;
                        let a1503 = v10;
                        nat_add(a1502, a1503)?
                    };
                    nat_sub(a1504, a1505)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a1506 = v0;
    let a1507 = f12(v1)?;
    if nat_lt(a1506, a1507) {
        let a1508 = v0;
        let a1509 = 4u64;
        if nat_lt(a1508, a1509) {
            let a1510 = v0;
            let a1511 = 0u64;
            if nat_eq(a1510, a1511) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1512 = v0;
                let a1513 = 1u64;
                if nat_eq(a1512, a1513) {
                    let a1514 = v5;
                    let a1515 = 0u64;
                    if nat_eq(a1514, a1515) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a1516 = v0;
                    let a1517 = 2u64;
                    if nat_eq(a1516, a1517) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1518 = v0;
            let a1519 = 4u64;
            if if nat_eq(a1518, a1519) {
                let a1520 = f114(v5)?;
                let a1521 = v1;
                !nat_eq(a1520, a1521)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a1524 = {
                        let a1522 = v0;
                        let a1523 = 4u64;
                        nat_sub(a1522, a1523)
                    };
                    let a1525 = 3u64;
                    let a1526 = 0u64;
                    nat_quot(a1524, a1525, a1526)
                };
                let v7: u64 = {
                    let a1529 = {
                        let a1527 = v0;
                        let a1528 = 4u64;
                        nat_sub(a1527, a1528)
                    };
                    let a1530 = 3u64;
                    let a1531 = 0u64;
                    nat_rem(a1529, a1530, a1531)
                };
                let a1532 = v7;
                let a1533 = 0u64;
                if nat_eq(a1532, a1533) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1534 = v7;
                    let a1535 = 1u64;
                    if nat_eq(a1534, a1535) {
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
        let a1536 = v0;
        let a1537 = 4294967296u64;
        let a1538 = 0u64;
        nat_rem(a1536, a1537, a1538)
    };
    let a1539 = v1;
    let a1540 = 2147483648u64;
    if nat_lt(a1539, a1540) {
        v1
    } else {
        0u64
    }
}

pub fn f60(v0: u64) -> R<u64> {
    let a1543 = 2u64;
    let a1544 = {
        let a1541 = 2u64;
        let a1542 = v0;
        nat_mul(a1541, a1542)?
    };
    nat_add(a1543, a1544)
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
    let a1545 = v2;
    let a1546 = 0u64;
    if nat_eq(a1545, a1546) {
        Ok(0u64)
    } else {
        let a1549 = 2u64;
        let a1550 = {
            let a1547 = v0;
            let a1548 = v1;
            nat_mul(a1547, a1548)?
        };
        nat_mul(a1549, a1550)
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1551 = v2;
    let a1552 = f111(v0, v1)?;
    nat_mul(a1551, a1552)
}

pub fn f65(v0: u64) -> u64 {
    let a1553 = v0;
    let a1554 = 30u64;
    if nat_eq(a1553, a1554) {
        3u64
    } else {
        1u64
    }
}

pub fn f66(v0: u64) -> R<u64> {
    let a1557 = 5u64;
    let a1558 = {
        let a1555 = 4u64;
        let a1556 = v0;
        nat_mul(a1555, a1556)?
    };
    nat_add(a1557, a1558)
}

pub fn f67(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f117(match v0.clone() {
        Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
            h1559
        }
    }, match v0.clone() {
        Adt2::C0(_, h1560, _, _, _, _, _, _, _, _, _) => {
            h1560
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1561, _, _, _, _, _, _, _, _) => {
            h1561
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1562, _, _, _, _, _, _, _) => {
            h1562
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1563, _, _, _, _, _, _) => {
            h1563
        }
    }, match v1.clone() {
        Adt2::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
            h1564
        }
    }, match v1.clone() {
        Adt2::C0(_, h1565, _, _, _, _, _, _, _, _, _) => {
            h1565
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1566, _, _, _, _, _, _, _, _) => {
            h1566
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1567, _, _, _, _, _, _, _) => {
            h1567
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1568, _, _, _, _, _, _) => {
            h1568
        }
    }, match v2.clone() {
        Adt2::C0(h1569, _, _, _, _, _, _, _, _, _, _) => {
            h1569
        }
    }, match v2.clone() {
        Adt2::C0(_, h1570, _, _, _, _, _, _, _, _, _) => {
            h1570
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1571, _, _, _, _, _, _, _, _) => {
            h1571
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
            h1572
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1573, _, _, _, _, _, _) => {
            h1573
        }
    }) {
        let m1575 = f70(match v0.clone() {
            Adt2::C0(h1574, _, _, _, _, _, _, _, _, _, _) => {
                h1574
            }
        });
        match m1575 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a1581 = {
                    let a1578 = match v0.clone() {
                        Adt2::C0(_, h1576, _, _, _, _, _, _, _, _, _) => {
                            h1576
                        }
                    };
                    let a1579 = match v3.clone() {
                        Adt11::C0(_, h1577, _) => {
                            h1577
                        }
                    };
                    let a1580 = 0u64;
                    nat_rem(a1578, a1579, a1580)
                };
                let a1582 = 0u64;
                if if nat_eq(a1581, a1582) {
                    if f118(match v0.clone() {
                        Adt2::C0(_, h1583, _, _, _, _, _, _, _, _, _) => {
                            h1583
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1584, _, _, _, _, _, _, _, _, _) => {
                            h1584
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1585, _, _, _, _, _, _, _, _) => {
                            h1585
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1586, _, _, _, _, _, _, _) => {
                            h1586
                        }
                    }) {
                        true
                    } else {
                        let a1588 = 4u64;
                        let a1589 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1587, _, _, _, _, _) => {
                                h1587
                            }
                        };
                        if if nat_le(a1588, a1589) {
                            let a1591 = match v3.clone() {
                                Adt11::C0(_, h1590, _) => {
                                    h1590
                                }
                            };
                            let a1592 = 1u64;
                            if nat_eq(a1591, a1592) {
                                true
                            } else {
                                let a1595 = match v3.clone() {
                                    Adt11::C0(_, _, h1593) => {
                                        h1593
                                    }
                                };
                                let a1596 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1594, _, _, _, _, _) => {
                                        h1594
                                    }
                                };
                                nat_le(a1595, a1596)
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
                        Adt11::C0(_, h1597, _) => {
                            h1597
                        }
                    });
                    Ok(Adt6::C4(if f118(match v0.clone() {
                        Adt2::C0(_, h1598, _, _, _, _, _, _, _, _, _) => {
                            h1598
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1599, _, _, _, _, _, _, _, _, _) => {
                            h1599
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1600, _, _, _, _, _, _, _, _) => {
                            h1600
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1601, _, _, _, _, _, _, _) => {
                            h1601
                        }
                    }) {
                        0u64
                    } else {
                        f104(v4.clone(), f13(v1.clone(), 1u64))?
                    }, 4u64, v4, f13(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
                            h1602
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
    let a1605 = match v1.clone() {
        Adt2::C0(h1603, _, _, _, _, _, _, _, _, _, _) => {
            h1603
        }
    };
    let a1606 = match v3.clone() {
        Adt2::C0(h1604, _, _, _, _, _, _, _, _, _, _) => {
            h1604
        }
    };
    if if nat_eq(a1605, a1606) {
        let a1607 = v0;
        let a1608 = 33u64;
        if nat_eq(a1607, a1608) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1610 = f70(match v1.clone() {
            Adt2::C0(h1609, _, _, _, _, _, _, _, _, _, _) => {
                h1609
            }
        });
        match m1610 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h1611, _) => {
                        h1611
                    }
                };
                let v6: u64 = f9(v1.clone(), v5)?;
                let a1616 = {
                    let a1613 = match v1.clone() {
                        Adt2::C0(_, h1612, _, _, _, _, _, _, _, _, _) => {
                            h1612
                        }
                    };
                    let a1614 = v5;
                    let a1615 = 1u64;
                    nat_rem(a1613, a1614, a1615)
                };
                let a1617 = 0u64;
                if if if nat_eq(a1616, a1617) {
                    let a1622 = {
                        let a1619 = match v3.clone() {
                            Adt2::C0(_, h1618, _, _, _, _, _, _, _, _, _) => {
                                h1618
                            }
                        };
                        let a1620 = v5;
                        let a1621 = 1u64;
                        nat_rem(a1619, a1620, a1621)
                    };
                    let a1623 = 0u64;
                    nat_eq(a1622, a1623)
                } else {
                    false
                } {
                    let a1624 = v6;
                    let a1625 = f9(v3.clone(), v5)?;
                    nat_eq(a1624, a1625)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h1626) => {
                            h1626
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
    let a1627 = v0;
    let a1628 = 0u64;
    if if nat_eq(a1627, a1628) {
        true
    } else {
        let a1629 = v0;
        let a1630 = 1u64;
        nat_eq(a1629, a1630)
    } {
        true
    } else {
        let a1631 = v0;
        let a1632 = 30u64;
        nat_eq(a1631, a1632)
    }
}

pub fn f70(v0: u64) -> Adt10 {
    let a1633 = v0;
    let a1634 = 0u64;
    if nat_eq(a1633, a1634) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a1635 = v0;
        let a1636 = 1u64;
        if nat_eq(a1635, a1636) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a1637 = v0;
            let a1638 = 2u64;
            if nat_eq(a1637, a1638) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a1639 = v0;
                let a1640 = 3u64;
                if nat_eq(a1639, a1640) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a1641 = v0;
                    let a1642 = 6u64;
                    if nat_eq(a1641, a1642) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1643 = v0;
                        let a1644 = 7u64;
                        if nat_eq(a1643, a1644) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1645 = v0;
                            let a1646 = 8u64;
                            if nat_eq(a1645, a1646) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1647 = v0;
                                let a1648 = 9u64;
                                if nat_eq(a1647, a1648) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1649 = v0;
                                    let a1650 = 10u64;
                                    if nat_eq(a1649, a1650) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1651 = v0;
                                        let a1652 = 11u64;
                                        if nat_eq(a1651, a1652) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1653 = v0;
                                            let a1654 = 12u64;
                                            if nat_eq(a1653, a1654) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1655 = v0;
                                                let a1656 = 13u64;
                                                if nat_eq(a1655, a1656) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1657 = v0;
                                                    let a1658 = 14u64;
                                                    if nat_eq(a1657, a1658) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1659 = v0;
                                                        let a1660 = 15u64;
                                                        if nat_eq(a1659, a1660) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1661 = v0;
                                                            let a1662 = 16u64;
                                                            if nat_eq(a1661, a1662) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1663 = v0;
                                                                let a1664 = 17u64;
                                                                if nat_eq(a1663, a1664) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1665 = v0;
                                                                    let a1666 = 18u64;
                                                                    if nat_eq(a1665, a1666) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1667 = v0;
                                                                        let a1668 = 19u64;
                                                                        if nat_eq(a1667, a1668) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1669 = v0;
                                                                            let a1670 = 20u64;
                                                                            if nat_eq(a1669, a1670) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1671 = v0;
                                                                                let a1672 = 21u64;
                                                                                if nat_eq(a1671, a1672) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1673 = v0;
                                                                                    let a1674 = 22u64;
                                                                                    if nat_eq(a1673, a1674) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1675 = v0;
                                                                                        let a1676 = 23u64;
                                                                                        if nat_eq(a1675, a1676) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1677 = v0;
                                                                                            let a1678 = 24u64;
                                                                                            if nat_eq(a1677, a1678) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1679 = v0;
                                                                                                let a1680 = 25u64;
                                                                                                if nat_eq(a1679, a1680) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1681 = v0;
                                                                                                    let a1682 = 26u64;
                                                                                                    if nat_eq(a1681, a1682) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1683 = v0;
                                                                                                        let a1684 = 27u64;
                                                                                                        if nat_eq(a1683, a1684) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1685 = v0;
                                                                                                            let a1686 = 28u64;
                                                                                                            if nat_eq(a1685, a1686) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1687 = v0;
                                                                                                                let a1688 = 29u64;
                                                                                                                if nat_eq(a1687, a1688) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1689 = v0;
                                                                                                                    let a1690 = 30u64;
                                                                                                                    if nat_eq(a1689, a1690) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1691 = v0;
                                                                                                                        let a1692 = 34u64;
                                                                                                                        if nat_eq(a1691, a1692) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1693 = v0;
                                                                                                                            let a1694 = 35u64;
                                                                                                                            if nat_eq(a1693, a1694) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1695 = v0;
                                                                                                                                let a1696 = 39u64;
                                                                                                                                if nat_eq(a1695, a1696) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1697 = v0;
                                                                                                                                    let a1698 = 40u64;
                                                                                                                                    if nat_eq(a1697, a1698) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1699 = v0;
                                                                                                                                        let a1700 = 41u64;
                                                                                                                                        if nat_eq(a1699, a1700) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1701 = v0;
                                                                                                                                            let a1702 = 42u64;
                                                                                                                                            if nat_eq(a1701, a1702) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1703 = v0;
                                                                                                                                                let a1704 = 43u64;
                                                                                                                                                if nat_eq(a1703, a1704) {
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
        let a1708 = {
            let a1705 = v0;
            let a1706 = v4;
            let a1707 = 1u64;
            nat_rem(a1705, a1706, a1707)
        };
        let a1709 = 0u64;
        if if if nat_eq(a1708, a1709) {
            let a1713 = {
                let a1710 = v1;
                let a1711 = v5;
                let a1712 = 1u64;
                nat_rem(a1710, a1711, a1712)
            };
            let a1714 = 0u64;
            nat_eq(a1713, a1714)
        } else {
            false
        } {
            let a1718 = {
                let a1715 = v2;
                let a1716 = v6;
                let a1717 = 1u64;
                nat_rem(a1715, a1716, a1717)
            };
            let a1719 = 0u64;
            nat_eq(a1718, a1719)
        } else {
            false
        } {
            let a1723 = {
                let a1720 = v3;
                let a1721 = v7;
                let a1722 = 1u64;
                nat_rem(a1720, a1721, a1722)
            };
            let a1724 = 0u64;
            nat_eq(a1723, a1724)
        } else {
            false
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1725 = v0;
    let a1726 = v1;
    if nat_eq(a1725, a1726) {
        let a1729 = v4;
        let a1730 = {
            let a1727 = v2;
            let a1728 = v3;
            nat_add(a1727, a1728)?
        };
        Ok(nat_eq(a1729, a1730))
    } else {
        let a1731 = v2;
        let a1732 = v3;
        if nat_eq(a1731, a1732) {
            let a1733 = v4;
            let a1734 = v2;
            Ok(nat_eq(a1733, a1734))
        } else {
            Ok(false)
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1737 = {
            let a1735 = v0;
            let a1736 = f119(v1);
            nat_add(a1735, a1736)?
        };
        let a1738 = f119(v2);
        nat_add(a1737, a1738)?
    };
    let v5: u64 = {
        let a1739 = f120(v1);
        let a1740 = f120(v2);
        nat_add(a1739, a1740)?
    };
    let a1741 = v1;
    let a1742 = 4294967295u64;
    if if nat_le(a1741, a1742) {
        let a1743 = v2;
        let a1744 = 4294967295u64;
        nat_le(a1743, a1744)
    } else {
        false
    } {
        let a1745 = v5;
        let a1746 = v4;
        if nat_le(a1745, a1746) {
            let a1749 = v3;
            let a1750 = {
                let a1747 = v4;
                let a1748 = v5;
                nat_sub(a1747, a1748)
            };
            Ok(nat_eq(a1749, a1750))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f74(v0: u64, v1: u64) -> R<bool> {
    let a1751 = v1;
    let a1752 = 4294967295u64;
    if nat_le(a1751, a1752) {
        let a1755 = {
            let a1753 = f119(v1);
            let a1754 = f120(v1);
            nat_add(a1753, a1754)?
        };
        let a1756 = v0;
        Ok(nat_lt(a1755, a1756))
    } else {
        Ok(false)
    }
}

pub fn f75(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f121(v0.clone(), false)?;
    let v6: u64 = f121(v0, true)?;
    let a1757 = v6;
    let a1758 = v1;
    if if nat_le(a1757, a1758) {
        let a1765 = {
            let a1763 = {
                let a1759 = v1;
                let a1760 = v5;
                nat_add(a1759, a1760)?
            };
            let a1764 = {
                let a1761 = v3;
                let a1762 = 0u64;
                if nat_eq(a1761, a1762) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1763, a1764)?
        };
        let a1766 = v2;
        nat_le(a1765, a1766)
    } else {
        false
    } {
        if v4 {
            let a1771 = {
                let a1769 = {
                    let a1767 = v1;
                    let a1768 = v5;
                    nat_add(a1767, a1768)?
                };
                let a1770 = v6;
                nat_sub(a1769, a1770)
            };
            let a1772 = v2;
            Ok(nat_lt(a1771, a1772))
        } else {
            let a1777 = {
                let a1775 = {
                    let a1773 = v1;
                    let a1774 = v5;
                    nat_add(a1773, a1774)?
                };
                let a1776 = v6;
                nat_sub(a1775, a1776)
            };
            let a1778 = v2;
            Ok(nat_le(a1777, a1778))
        }
    } else {
        Ok(false)
    }
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1779 = v0;
    let a1780 = 0u64;
    if if nat_eq(a1779, a1780) {
        true
    } else {
        let a1781 = v0;
        let a1782 = 1u64;
        nat_eq(a1781, a1782)
    } {
        let a1783 = v1;
        let a1784 = 4294967296u64;
        nat_lt(a1783, a1784)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64) -> R<u64> {
    let a1789 = {
        let a1787 = v0;
        let a1788 = {
            let a1785 = v1;
            let a1786 = 1u64;
            nat_sub(a1785, a1786)
        };
        nat_add(a1787, a1788)?
    };
    let a1790 = v1;
    let a1791 = 0u64;
    Ok(nat_quot(a1789, a1790, a1791))
}

pub fn f78(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1792, _, _, _, _, _, _, _, _, _) => {
            h1792
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1793, _, _, _, _, _, _, _, _) => {
            h1793
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1794, _, _, _, _, _, _, _) => {
            h1794
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1795, _, _, _, _, _, _) => {
            h1795
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1798 = match v1.clone() {
                Adt2::C0(_, h1796, _, _, _, _, _, _, _, _, _) => {
                    h1796
                }
            };
            let a1799 = match v1.clone() {
                Adt2::C0(_, _, h1797, _, _, _, _, _, _, _, _) => {
                    h1797
                }
            };
            nat_mul(a1798, a1799)?
        };
        let v5: u64 = {
            let a1801 = v4;
            let a1802 = match v1.clone() {
                Adt2::C0(_, _, _, h1800, _, _, _, _, _, _, _) => {
                    h1800
                }
            };
            nat_mul(a1801, a1802)?
        };
        let a1815 = {
            let a1813 = {
                let a1804 = match v0.clone() {
                    Adt2::C0(_, h1803, _, _, _, _, _, _, _, _, _) => {
                        h1803
                    }
                };
                let a1805 = 1u64;
                nat_sub(a1804, a1805)
            };
            let a1814 = {
                let a1811 = f122(match v0.clone() {
                    Adt2::C0(_, _, h1806, _, _, _, _, _, _, _, _) => {
                        h1806
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1807, _, _, _, _, _, _, _, _, _) => {
                        h1807
                    }
                })?;
                let a1812 = f122(match v0 {
                    Adt2::C0(_, _, _, h1808, _, _, _, _, _, _, _) => {
                        h1808
                    }
                }, v2, {
                    let a1809 = v3;
                    let a1810 = v5;
                    nat_mul(a1809, a1810)?
                }, v4)?;
                nat_add(a1811, a1812)?
            };
            nat_add(a1813, a1814)?
        };
        let a1816 = f9(v1, 1u64)?;
        Ok(nat_lt(a1815, a1816))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1817 = v0;
    let a1818 = 2147483648u64;
    if if if if nat_lt(a1817, a1818) {
        let a1819 = v1;
        let a1820 = 2147483648u64;
        nat_lt(a1819, a1820)
    } else {
        false
    } {
        let a1823 = {
            let a1821 = v0;
            let a1822 = v1;
            nat_mul(a1821, a1822)?
        };
        let a1824 = 2147483648u64;
        nat_lt(a1823, a1824)
    } else {
        false
    } {
        let a1829 = {
            let a1827 = {
                let a1825 = v0;
                let a1826 = 1u64;
                nat_sub(a1825, a1826)
            };
            let a1828 = v2;
            nat_mul(a1827, a1828)?
        };
        let a1830 = 2147483648u64;
        nat_lt(a1829, a1830)
    } else {
        false
    } {
        let a1835 = {
            let a1833 = {
                let a1831 = v1;
                let a1832 = 1u64;
                nat_sub(a1831, a1832)
            };
            let a1834 = v2;
            nat_mul(a1833, a1834)?
        };
        let a1836 = 2147483648u64;
        Ok(nat_lt(a1835, a1836))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a1839 = 3u64;
    let a1840 = {
        let a1837 = v0;
        let a1838 = 3u64;
        nat_mul(a1837, a1838)?
    };
    nat_add(a1839, a1840)
}

pub fn f81(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
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
    }) {
        Ok(0u64)
    } else {
        let a1853 = match v0.clone() {
            Adt3::C0(h1845, _, _, _, _, _, _, _) => {
                h1845
            }
        };
        let a1854 = {
            let a1851 = match v0.clone() {
                Adt3::C0(_, h1846, _, _, _, _, _, _) => {
                    h1846
                }
            };
            let a1852 = {
                let a1849 = match v0.clone() {
                    Adt3::C0(_, _, h1847, _, _, _, _, _) => {
                        h1847
                    }
                };
                let a1850 = match v0 {
                    Adt3::C0(_, _, _, h1848, _, _, _, _) => {
                        h1848
                    }
                };
                nat_mul(a1849, a1850)?
            };
            nat_mul(a1851, a1852)?
        };
        nat_mul(a1853, a1854)
    }
}

pub fn f82(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1855 = v1;
    let a1856 = 13u64;
    if if nat_eq(a1855, a1856) {
        f23(match v0.clone() {
            Adt3::C0(h1857, _, _, _, _, _, _, _) => {
                h1857
            }
        }, match v0.clone() {
            Adt3::C0(_, h1858, _, _, _, _, _, _) => {
                h1858
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1859, _, _, _, _, _) => {
                h1859
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1860, _, _, _, _) => {
                h1860
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1866 = match v0.clone() {
                Adt3::C0(_, h1861, _, _, _, _, _, _) => {
                    h1861
                }
            };
            let a1867 = {
                let a1864 = match v0.clone() {
                    Adt3::C0(_, _, h1862, _, _, _, _, _) => {
                        h1862
                    }
                };
                let a1865 = match v0.clone() {
                    Adt3::C0(_, _, _, h1863, _, _, _, _) => {
                        h1863
                    }
                };
                nat_mul(a1864, a1865)?
            };
            nat_mul(a1866, a1867)?
        };
        let a1868 = v1;
        let a1869 = 13u64;
        if nat_eq(a1868, a1869) {
            let a1878 = 2u64;
            let a1879 = if f123(v0.clone(), v2) {
                let a1871 = v3;
                let a1872 = f124(match v0 {
                    Adt3::C0(h1870, _, _, _, _, _, _, _) => {
                        h1870
                    }
                }, false)?;
                nat_mul(a1871, a1872)?
            } else {
                let a1876 = v3;
                let a1877 = {
                    let a1874 = match v0 {
                        Adt3::C0(h1873, _, _, _, _, _, _, _) => {
                            h1873
                        }
                    };
                    let a1875 = 2u64;
                    nat_mul(a1874, a1875)?
                };
                nat_mul(a1876, a1877)?
            };
            nat_add(a1878, a1879)
        } else {
            let a1883 = v3;
            let a1884 = f124(match v0 {
                Adt3::C0(h1880, _, _, _, _, _, _, _) => {
                    h1880
                }
            }, {
                let a1881 = v1;
                let a1882 = 16u64;
                nat_eq(a1881, a1882)
            })?;
            nat_mul(a1883, a1884)
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1885 = v3;
    match m1885 {
        None => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1886, _, _, _, _, _, _, _, _, _, _) => {
                    h1886
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1887, _, _, _, _, _, _, _, _, _, _) => {
                    h1887
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1888, _, _, _, _, _, _, _, _, _) => {
                    h1888
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1889, _, _, _, _, _, _, _, _) => {
                    h1889
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1890, _, _, _, _, _, _, _) => {
                    h1890
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1891, _, _, _, _, _, _) => {
                    h1891
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1892, _, _, _, _, _, _, _, _, _) => {
                    h1892
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1893, _, _, _, _, _, _, _, _) => {
                    h1893
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1894, _, _, _, _, _, _, _) => {
                    h1894
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1895, _, _, _, _, _, _) => {
                    h1895
                }
            })
        }
        Some(v5) => {
            f125(v0, v1, match v2.clone() {
                Adt2::C0(h1896, _, _, _, _, _, _, _, _, _, _) => {
                    h1896
                }
            }, match v5.clone() {
                Adt2::C0(h1897, _, _, _, _, _, _, _, _, _, _) => {
                    h1897
                }
            }, match v4.clone() {
                Adt2::C0(h1898, _, _, _, _, _, _, _, _, _, _) => {
                    h1898
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1899, _, _, _, _, _, _, _, _, _) => {
                    h1899
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1900, _, _, _, _, _, _, _, _) => {
                    h1900
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1901, _, _, _, _, _, _, _) => {
                    h1901
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1902, _, _, _, _, _, _) => {
                    h1902
                }
            }, match v5.clone() {
                Adt2::C0(_, h1903, _, _, _, _, _, _, _, _, _) => {
                    h1903
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1904, _, _, _, _, _, _, _, _) => {
                    h1904
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1905, _, _, _, _, _, _, _) => {
                    h1905
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1906, _, _, _, _, _, _) => {
                    h1906
                }
            }, match v4.clone() {
                Adt2::C0(_, h1907, _, _, _, _, _, _, _, _, _) => {
                    h1907
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1908, _, _, _, _, _, _, _, _) => {
                    h1908
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1909, _, _, _, _, _, _, _) => {
                    h1909
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1910, _, _, _, _, _, _) => {
                    h1910
                }
            })
        }
    }
}

pub fn f84(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a1911 = v7;
        let a1912 = 6u64;
        if nat_eq(a1911, a1912) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f126(v3, v4), f126(v5, v6), f127(v7, v8), f128(v1), f128(v9))
}

pub fn f85(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m1913 = v0;
    match m1913 {
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
    let m1914 = v0;
    match m1914 {
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
    let m1915 = v0;
    match m1915 {
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
            let a1916 = v1;
            let a1917 = 6u64;
            if nat_eq(a1916, a1917) {
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
            let a1918 = v1;
            let a1919 = 6u64;
            if if if nat_eq(a1918, a1919) {
                true
            } else {
                let a1920 = v1;
                let a1921 = 7u64;
                nat_eq(a1920, a1921)
            } {
                let a1922 = v49;
                let a1923 = 30u64;
                nat_eq(a1922, a1923)
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
    let a1924 = v1;
    let a1925 = 4u64;
    if nat_lt(a1924, a1925) {
        Adt7::C0(v1)
    } else {
        let a1926 = v1;
        let a1927 = 4u64;
        if nat_eq(a1926, a1927) {
            Adt7::C1(0u64)
        } else {
            let a1928 = v1;
            let a1929 = 5u64;
            if nat_eq(a1928, a1929) {
                Adt7::C1(3u64)
            } else {
                let a1930 = v1;
                let a1931 = 6u64;
                if nat_eq(a1930, a1931) {
                    Adt7::C2(f87(v0, v1), 0u64, false)
                } else {
                    let a1932 = v1;
                    let a1933 = 7u64;
                    if nat_eq(a1932, a1933) {
                        Adt7::C2(0u64, f87(v0, v1), false)
                    } else {
                        let a1934 = v1;
                        let a1935 = 16u64;
                        if nat_lt(a1934, a1935) {
                            Adt7::C1({
                                let a1936 = v1;
                                let a1937 = 8u64;
                                nat_sub(a1936, a1937)
                            })
                        } else {
                            let a1938 = v1;
                            let a1939 = 20u64;
                            if nat_lt(a1938, a1939) {
                                Adt7::C5({
                                    let a1940 = v1;
                                    let a1941 = 16u64;
                                    nat_sub(a1940, a1941)
                                })
                            } else {
                                let a1942 = v1;
                                let a1943 = 20u64;
                                if nat_eq(a1942, a1943) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a1944 = v1;
                                    let a1945 = 21u64;
                                    if nat_eq(a1944, a1945) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a1946 = v1;
                                        let a1947 = 22u64;
                                        if nat_eq(a1946, a1947) {
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
        let a1949 = v0;
        let a1950 = match v1.clone() {
            Adt3::C0(h1948, _, _, _, _, _, _, _) => {
                h1948
            }
        };
        let a1951 = 0u64;
        nat_quot(a1949, a1950, a1951)
    };
    let v3: u64 = {
        let a1953 = v2;
        let a1954 = match v1.clone() {
            Adt3::C0(_, h1952, _, _, _, _, _, _) => {
                h1952
            }
        };
        let a1955 = 0u64;
        nat_quot(a1953, a1954, a1955)
    };
    f129({
        let a1957 = v0;
        let a1958 = match v1.clone() {
            Adt3::C0(h1956, _, _, _, _, _, _, _) => {
                h1956
            }
        };
        let a1959 = 0u64;
        nat_rem(a1957, a1958, a1959)
    }, {
        let a1961 = v2;
        let a1962 = match v1.clone() {
            Adt3::C0(_, h1960, _, _, _, _, _, _) => {
                h1960
            }
        };
        let a1963 = 0u64;
        nat_rem(a1961, a1962, a1963)
    }, {
        let a1965 = v3;
        let a1966 = match v1.clone() {
            Adt3::C0(_, _, h1964, _, _, _, _, _) => {
                h1964
            }
        };
        let a1967 = 0u64;
        nat_rem(a1965, a1966, a1967)
    }, {
        let a1969 = v3;
        let a1970 = match v1.clone() {
            Adt3::C0(_, _, h1968, _, _, _, _, _) => {
                h1968
            }
        };
        let a1971 = 0u64;
        nat_quot(a1969, a1970, a1971)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1972, _, _, _) => {
            h1972
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1973, _, _) => {
            h1973
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1974, _) => {
            h1974
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1975) => {
            h1975
        }
    })
}

pub fn f90(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1993 = {
        let a1988 = f130(v2, {
            let a1979 = {
                let a1976 = v0;
                let a1977 = v1;
                let a1978 = 0u64;
                nat_quot(a1976, a1977, a1978)
            };
            let a1980 = v2;
            let a1981 = 0u64;
            nat_rem(a1979, a1980, a1981)
        }, {
            let a1985 = {
                let a1982 = v0;
                let a1983 = v1;
                let a1984 = 0u64;
                nat_quot(a1982, a1983, a1984)
            };
            let a1986 = v2;
            let a1987 = 0u64;
            nat_quot(a1985, a1986, a1987)
        })?;
        let a1989 = v1;
        nat_mul(a1988, a1989)?
    };
    let a1994 = {
        let a1990 = v0;
        let a1991 = v1;
        let a1992 = 0u64;
        nat_rem(a1990, a1991, a1992)
    };
    nat_add(a1993, a1994)
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1996 = v0;
        let a1997 = match v2.clone() {
            Adt3::C0(h1995, _, _, _, _, _, _, _) => {
                h1995
            }
        };
        let a1998 = 0u64;
        nat_quot(a1996, a1997, a1998)
    };
    let v4: u64 = {
        let a2000 = v3;
        let a2001 = match v2.clone() {
            Adt3::C0(_, h1999, _, _, _, _, _, _) => {
                h1999
            }
        };
        let a2002 = 0u64;
        nat_quot(a2000, a2001, a2002)
    };
    f131({
        let a2004 = v0;
        let a2005 = match v2.clone() {
            Adt3::C0(h2003, _, _, _, _, _, _, _) => {
                h2003
            }
        };
        let a2006 = 0u64;
        nat_rem(a2004, a2005, a2006)
    }, {
        let a2008 = v3;
        let a2009 = match v2.clone() {
            Adt3::C0(_, h2007, _, _, _, _, _, _) => {
                h2007
            }
        };
        let a2010 = 0u64;
        nat_rem(a2008, a2009, a2010)
    }, {
        let a2012 = v4;
        let a2013 = match v2.clone() {
            Adt3::C0(_, _, h2011, _, _, _, _, _) => {
                h2011
            }
        };
        let a2014 = 0u64;
        nat_rem(a2012, a2013, a2014)
    }, {
        let a2016 = v4;
        let a2017 = match v2 {
            Adt3::C0(_, _, h2015, _, _, _, _, _) => {
                h2015
            }
        };
        let a2018 = 0u64;
        nat_quot(a2016, a2017, a2018)
    }, match v1.clone() {
        Adt3::C0(h2019, _, _, _, _, _, _, _) => {
            h2019
        }
    }, match v1.clone() {
        Adt3::C0(_, h2020, _, _, _, _, _, _) => {
            h2020
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2021, _, _, _, _, _) => {
            h2021
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2022, _, _, _, _) => {
            h2022
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2023, _, _, _) => {
            h2023
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2024, _, _) => {
            h2024
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2025, _) => {
            h2025
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2026) => {
            h2026
        }
    })
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2028 = v0;
        let a2029 = match v3.clone() {
            Adt3::C0(h2027, _, _, _, _, _, _, _) => {
                h2027
            }
        };
        let a2030 = 0u64;
        nat_quot(a2028, a2029, a2030)
    };
    let v6: u64 = {
        let a2032 = v5;
        let a2033 = match v3.clone() {
            Adt3::C0(_, h2031, _, _, _, _, _, _) => {
                h2031
            }
        };
        let a2034 = 0u64;
        nat_quot(a2032, a2033, a2034)
    };
    let v7: u64 = {
        let a2036 = v0;
        let a2037 = match v3.clone() {
            Adt3::C0(h2035, _, _, _, _, _, _, _) => {
                h2035
            }
        };
        let a2038 = 0u64;
        nat_rem(a2036, a2037, a2038)
    };
    let v8: u64 = {
        let a2040 = v5;
        let a2041 = match v3.clone() {
            Adt3::C0(_, h2039, _, _, _, _, _, _) => {
                h2039
            }
        };
        let a2042 = 0u64;
        nat_rem(a2040, a2041, a2042)
    };
    let v9: u64 = {
        let a2044 = v6;
        let a2045 = match v3.clone() {
            Adt3::C0(_, _, h2043, _, _, _, _, _) => {
                h2043
            }
        };
        let a2046 = 0u64;
        nat_rem(a2044, a2045, a2046)
    };
    let v10: u64 = {
        let a2048 = v6;
        let a2049 = match v3.clone() {
            Adt3::C0(_, _, h2047, _, _, _, _, _) => {
                h2047
            }
        };
        let a2050 = 0u64;
        nat_quot(a2048, a2049, a2050)
    };
    let v11: u64 = {
        let a2051 = v4;
        let a2052 = 0u64;
        if nat_eq(a2051, a2052) {
            match v1.clone() {
                Adt3::C0(h2053, _, _, _, _, _, _, _) => {
                    h2053
                }
            }
        } else {
            let a2054 = v4;
            let a2055 = 1u64;
            if nat_eq(a2054, a2055) {
                match v1.clone() {
                    Adt3::C0(_, h2056, _, _, _, _, _, _) => {
                        h2056
                    }
                }
            } else {
                let a2057 = v4;
                let a2058 = 2u64;
                if nat_eq(a2057, a2058) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2059, _, _, _, _, _) => {
                            h2059
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2060, _, _, _, _) => {
                            h2060
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2067 = v11;
        let a2068 = {
            let a2061 = v4;
            let a2062 = 0u64;
            if nat_eq(a2061, a2062) {
                v7
            } else {
                let a2063 = v4;
                let a2064 = 1u64;
                if nat_eq(a2063, a2064) {
                    v8
                } else {
                    let a2065 = v4;
                    let a2066 = 2u64;
                    if nat_eq(a2065, a2066) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2067, a2068)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f129(if if v12 {
        let a2069 = v4;
        let a2070 = 0u64;
        nat_eq(a2069, a2070)
    } else {
        false
    } {
        let a2071 = v7;
        let a2072 = v11;
        nat_sub(a2071, a2072)
    } else {
        v7
    }, if if v12 {
        let a2073 = v4;
        let a2074 = 1u64;
        nat_eq(a2073, a2074)
    } else {
        false
    } {
        let a2075 = v8;
        let a2076 = v11;
        nat_sub(a2075, a2076)
    } else {
        v8
    }, if if v12 {
        let a2077 = v4;
        let a2078 = 2u64;
        nat_eq(a2077, a2078)
    } else {
        false
    } {
        let a2079 = v9;
        let a2080 = v11;
        nat_sub(a2079, a2080)
    } else {
        v9
    }, if if v12 {
        let a2081 = v4;
        let a2082 = 3u64;
        nat_eq(a2081, a2082)
    } else {
        false
    } {
        let a2083 = v10;
        let a2084 = v11;
        nat_sub(a2083, a2084)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2085, _, _, _) => {
            h2085
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2086, _, _) => {
            h2086
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2087, _) => {
            h2087
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2088) => {
            h2088
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
            Adt3::C0(h2089, _, _, _, _, _, _, _) => {
                h2089
            }
        })?, f134(v9, v4, match v1.clone() {
            Adt3::C0(_, h2090, _, _, _, _, _, _) => {
                h2090
            }
        })?, f134(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2091, _, _, _, _, _) => {
                h2091
            }
        })?, f134(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2092, _, _, _, _) => {
                h2092
            }
        })?)?, f89(v0, v2)?))
    } else {
        if if if if f135(v8, v3, match v1.clone() {
            Adt3::C0(h2093, _, _, _, _, _, _, _) => {
                h2093
            }
        })? {
            f135(v9, v4, match v1.clone() {
                Adt3::C0(_, h2094, _, _, _, _, _, _) => {
                    h2094
                }
            })?
        } else {
            false
        } {
            f135(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2095, _, _, _, _, _) => {
                    h2095
                }
            })?
        } else {
            false
        } {
            f135(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2096, _, _, _, _) => {
                    h2096
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
        Adt3::C0(h2097, _, _, _, _, _, _, _) => {
            h2097
        }
    })?, v5, v6, v7)?, f89(v0, v2)?))
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a2098 = v0;
    let a2099 = v3;
    if nat_lt(a2098, a2099) {
        Ok(Adt5::C1(false, {
            let a2100 = v0;
            let a2101 = 4u64;
            nat_mul(a2100, a2101)?
        }, {
            let a2102 = v0;
            let a2103 = 4u64;
            nat_mul(a2102, a2103)?
        }))
    } else {
        let v6: u64 = {
            let a2104 = v0;
            let a2105 = v3;
            nat_sub(a2104, a2105)
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
    let a2106 = v0;
    let a2107 = v3;
    if nat_lt(a2106, a2107) {
        Ok(Adt5::C1(true, 0u64, f89(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2108 = v0;
            let a2109 = v3;
            nat_sub(a2108, a2109)
        };
        let v5: u64 = f91(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f89(v4, v1)?, 3u64, v5))
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a2111 = v1;
        let a2112 = match v3.clone() {
            Adt3::C0(h2110, _, _, _, _, _, _, _) => {
                h2110
            }
        };
        let a2113 = 0u64;
        nat_rem(a2111, a2112, a2113)
    };
    let v5: u64 = {
        let a2119 = {
            let a2115 = v1;
            let a2116 = match v3.clone() {
                Adt3::C0(h2114, _, _, _, _, _, _, _) => {
                    h2114
                }
            };
            let a2117 = 0u64;
            nat_quot(a2115, a2116, a2117)
        };
        let a2120 = match v3.clone() {
            Adt3::C0(_, h2118, _, _, _, _, _, _) => {
                h2118
            }
        };
        let a2121 = 0u64;
        nat_rem(a2119, a2120, a2121)
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
        let a2122 = v3;
        let a2123 = 8u64;
        nat_add(a2122, a2123)?
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
        let a2125 = v0;
        let a2126 = match v2.clone() {
            Adt3::C0(h2124, _, _, _, _, _, _, _) => {
                h2124
            }
        };
        let a2127 = 0u64;
        nat_rem(a2125, a2126, a2127)
    };
    let v7: u64 = {
        let a2129 = v0;
        let a2130 = match v2.clone() {
            Adt3::C0(h2128, _, _, _, _, _, _, _) => {
                h2128
            }
        };
        let a2131 = 0u64;
        nat_quot(a2129, a2130, a2131)
    };
    let v8: u64 = {
        let a2133 = v7;
        let a2134 = match v2.clone() {
            Adt3::C0(_, h2132, _, _, _, _, _, _) => {
                h2132
            }
        };
        let a2135 = 0u64;
        nat_rem(a2133, a2134, a2135)
    };
    let v9: u64 = {
        let a2137 = v7;
        let a2138 = match v2.clone() {
            Adt3::C0(_, h2136, _, _, _, _, _, _) => {
                h2136
            }
        };
        let a2139 = 0u64;
        nat_quot(a2137, a2138, a2139)
    };
    let v10: u64 = {
        let a2141 = v9;
        let a2142 = match v2.clone() {
            Adt3::C0(_, _, h2140, _, _, _, _, _) => {
                h2140
            }
        };
        let a2143 = 0u64;
        nat_rem(a2141, a2142, a2143)
    };
    let v11: u64 = {
        let a2145 = v9;
        let a2146 = match v2.clone() {
            Adt3::C0(_, _, h2144, _, _, _, _, _) => {
                h2144
            }
        };
        let a2147 = 0u64;
        nat_quot(a2145, a2146, a2147)
    };
    let v12: u64 = f89(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2156 = {
                let a2151 = {
                    let a2148 = v10;
                    let a2149 = v4;
                    let a2150 = 0u64;
                    nat_quot(a2148, a2149, a2150)
                };
                let a2152 = v5;
                nat_mul(a2151, a2152)?
            };
            let a2157 = {
                let a2153 = v8;
                let a2154 = v4;
                let a2155 = 0u64;
                nat_quot(a2153, a2154, a2155)
            };
            nat_add(a2156, a2157)?
        };
        let v14: u64 = {
            let a2177 = v6;
            let a2178 = {
                let a2175 = match v1.clone() {
                    Adt3::C0(h2158, _, _, _, _, _, _, _) => {
                        h2158
                    }
                };
                let a2176 = {
                    let a2173 = {
                        let a2159 = v8;
                        let a2160 = v4;
                        let a2161 = 0u64;
                        nat_rem(a2159, a2160, a2161)
                    };
                    let a2174 = {
                        let a2171 = match v1.clone() {
                            Adt3::C0(_, h2162, _, _, _, _, _, _) => {
                                h2162
                            }
                        };
                        let a2172 = {
                            let a2169 = {
                                let a2163 = v10;
                                let a2164 = v4;
                                let a2165 = 0u64;
                                nat_rem(a2163, a2164, a2165)
                            };
                            let a2170 = {
                                let a2167 = match v1.clone() {
                                    Adt3::C0(_, _, h2166, _, _, _, _, _) => {
                                        h2166
                                    }
                                };
                                let a2168 = v13;
                                nat_mul(a2167, a2168)?
                            };
                            nat_add(a2169, a2170)?
                        };
                        nat_mul(a2171, a2172)?
                    };
                    nat_add(a2173, a2174)?
                };
                nat_mul(a2175, a2176)?
            };
            nat_add(a2177, a2178)?
        };
        Ok(Adt5::C1(false, f89(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2184 = {
                let a2182 = {
                    let a2179 = v11;
                    let a2180 = v5;
                    let a2181 = 0u64;
                    nat_rem(a2179, a2180, a2181)
                };
                let a2183 = v4;
                nat_mul(a2182, a2183)?
            };
            let a2185 = v8;
            nat_add(a2184, a2185)?
        };
        let v16: u64 = {
            let a2191 = {
                let a2189 = {
                    let a2186 = v11;
                    let a2187 = v5;
                    let a2188 = 0u64;
                    nat_quot(a2186, a2187, a2188)
                };
                let a2190 = v4;
                nat_mul(a2189, a2190)?
            };
            let a2192 = v10;
            nat_add(a2191, a2192)?
        };
        let a2194 = v15;
        let a2195 = match v1.clone() {
            Adt3::C0(_, h2193, _, _, _, _, _, _) => {
                h2193
            }
        };
        if if nat_lt(a2194, a2195) {
            let a2197 = v16;
            let a2198 = match v1.clone() {
                Adt3::C0(_, _, h2196, _, _, _, _, _) => {
                    h2196
                }
            };
            nat_lt(a2197, a2198)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f89({
                let a2207 = v6;
                let a2208 = {
                    let a2205 = match v1.clone() {
                        Adt3::C0(h2199, _, _, _, _, _, _, _) => {
                            h2199
                        }
                    };
                    let a2206 = {
                        let a2203 = v15;
                        let a2204 = {
                            let a2201 = match v1.clone() {
                                Adt3::C0(_, h2200, _, _, _, _, _, _) => {
                                    h2200
                                }
                            };
                            let a2202 = v16;
                            nat_mul(a2201, a2202)?
                        };
                        nat_add(a2203, a2204)?
                    };
                    nat_mul(a2205, a2206)?
                };
                nat_add(a2207, a2208)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f100(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f89(v0, v2.clone())?;
    let v4: u64 = f89(v0, v1.clone())?;
    let a2213 = {
        let a2210 = v0;
        let a2211 = match v1 {
            Adt3::C0(h2209, _, _, _, _, _, _, _) => {
                h2209
            }
        };
        let a2212 = 0u64;
        nat_rem(a2210, a2211, a2212)
    };
    let a2214 = 0u64;
    if nat_eq(a2213, a2214) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f89({
            let a2215 = v0;
            let a2216 = 1u64;
            nat_sub(a2215, a2216)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f101(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2217 = v3;
    let a2218 = 13u64;
    if nat_eq(a2217, a2218) {
        f140(v0, v1, v4)
    } else {
        let v5: u64 = f124(match v1.clone() {
            Adt3::C0(h2219, _, _, _, _, _, _, _) => {
                h2219
            }
        }, {
            let a2220 = v3;
            let a2221 = 16u64;
            nat_eq(a2220, a2221)
        })?;
        let v6: u64 = {
            let a2222 = v0;
            let a2223 = v5;
            let a2224 = 0u64;
            nat_quot(a2222, a2223, a2224)
        };
        let v7: u64 = {
            let a2225 = v0;
            let a2226 = v5;
            let a2227 = 0u64;
            nat_rem(a2225, a2226, a2227)
        };
        let v8: u64 = {
            let a2231 = 1u64;
            let a2232 = {
                let a2229 = match v1.clone() {
                    Adt3::C0(h2228, _, _, _, _, _, _, _) => {
                        h2228
                    }
                };
                let a2230 = 2u64;
                nat_mul(a2229, a2230)?
            };
            nat_add(a2231, a2232)?
        };
        let a2233 = v7;
        let a2234 = v8;
        if nat_lt(a2233, a2234) {
            f141(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f89(v6, v2)?;
            let a2235 = v3;
            let a2236 = 14u64;
            if nat_eq(a2235, a2236) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a2237 = v7;
                let a2238 = v8;
                if nat_eq(a2237, a2238) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2241 = v7;
                    let a2242 = {
                        let a2239 = v8;
                        let a2240 = 1u64;
                        nat_add(a2239, a2240)?
                    };
                    if nat_eq(a2241, a2242) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h2243, _, _, _, _, _, _, _) => {
                                h2243
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
    let a2244 = v0;
    let a2245 = 0u64;
    if nat_eq(a2244, a2245) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2252 = {
            let a2246 = v0;
            let a2247 = 1u64;
            nat_add(a2246, a2247)?
        };
        let a2253 = {
            let a2250 = 2u64;
            let a2251 = {
                let a2248 = v3;
                let a2249 = 2u64;
                nat_mul(a2248, a2249)?
            };
            nat_add(a2250, a2251)?
        };
        if nat_eq(a2252, a2253) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a2254 = v0;
                let a2255 = 1u64;
                nat_sub(a2254, a2255)
            };
            let v5: u64 = {
                let a2256 = v4;
                let a2257 = 2u64;
                let a2258 = 0u64;
                nat_quot(a2256, a2257, a2258)
            };
            let a2262 = {
                let a2259 = v4;
                let a2260 = 2u64;
                let a2261 = 0u64;
                nat_rem(a2259, a2260, a2261)
            };
            let a2263 = 0u64;
            if nat_eq(a2262, a2263) {
                Ok(Adt5::C4(17u64, 0u64, f89(v5, v1)?, 4u64, f89(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a2264 = v0;
    let a2265 = f18(v1, v2, v3)?;
    if nat_lt(a2264, a2265) {
        let a2266 = v0;
        let a2267 = v1;
        if nat_lt(a2266, a2267) {
            Ok(Adt5::C1(false, {
                let a2268 = 4u64;
                let a2269 = v0;
                nat_mul(a2268, a2269)?
            }, {
                let a2270 = 4u64;
                let a2271 = v0;
                nat_mul(a2270, a2271)?
            }))
        } else {
            let v4: u64 = {
                let a2272 = v0;
                let a2273 = v1;
                nat_sub(a2272, a2273)
            };
            let v5: u64 = f142(v4, v2)?;
            let a2274 = v5;
            let a2275 = 2u64;
            if nat_lt(a2274, a2275) {
                Ok(Adt5::C4(6u64, {
                    let a2276 = v5;
                    let a2277 = 0u64;
                    if nat_eq(a2276, a2277) {
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
    let a2285 = {
        let a2278 = 2u64;
        let a2279 = f81(v1.clone())?;
        nat_mul(a2278, a2279)?
    };
    let a2286 = {
        let a2283 = f81(v1)?;
        let a2284 = {
            let a2281 = 1u64;
            let a2282 = match v0 {
                Adt3::C0(h2280, _, _, _, _, _, _, _) => {
                    h2280
                }
            };
            nat_add(a2281, a2282)?
        };
        nat_mul(a2283, a2284)?
    };
    nat_add(a2285, a2286)
}

pub fn f105(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f89(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f106(v0: u64, v1: u64) -> bool {
    let a2287 = v1;
    let a2288 = 2147483647u64;
    if nat_le(a2287, a2288) {
        let a2289 = v1;
        let a2290 = v0;
        nat_lt(a2289, a2290)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m2291 = f70(v0);
    match m2291 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a2293 = v2;
                let a2294 = match v7.clone() {
                    Adt11::C0(_, h2292, _) => {
                        h2292
                    }
                };
                nat_mul(a2293, a2294)?
            };
            let m2307 = f146(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h2295, _, _, _, _, _, _, _) => {
                    h2295
                }
            }, match v5.clone() {
                Adt3::C0(_, h2296, _, _, _, _, _, _) => {
                    h2296
                }
            }, match v5 {
                Adt3::C0(_, _, h2297, _, _, _, _, _) => {
                    h2297
                }
            }, {
                let a2300 = match v3.clone() {
                    Adt3::C0(h2298, _, _, _, _, _, _, _) => {
                        h2298
                    }
                };
                let a2301 = match v7.clone() {
                    Adt11::C0(_, h2299, _) => {
                        h2299
                    }
                };
                nat_mul(a2300, a2301)?
            }, match v3.clone() {
                Adt3::C0(_, h2302, _, _, _, _, _, _) => {
                    h2302
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h2303, _, _, _) => {
                    h2303
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h2304, _, _) => {
                    h2304
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h2305, _) => {
                    h2305
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h2306) => {
                    h2306
                }
            })?;
            match m2307 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a2308 = v0;
                    let a2309 = 0u64;
                    if if nat_eq(a2308, a2309) {
                        true
                    } else {
                        let a2310 = v0;
                        let a2311 = 26u64;
                        nat_eq(a2310, a2311)
                    } {
                        Ok(Adt5::C1(false, v9, f89({
                            let a2315 = {
                                let a2313 = v1;
                                let a2314 = match v4.clone() {
                                    Adt3::C0(h2312, _, _, _, _, _, _, _) => {
                                        h2312
                                    }
                                };
                                nat_mul(a2313, a2314)?
                            };
                            let a2316 = v8;
                            nat_add(a2315, a2316)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h2317) => {
                                h2317
                            }
                        }, f89({
                            let a2321 = {
                                let a2319 = v1;
                                let a2320 = match v4.clone() {
                                    Adt3::C0(h2318, _, _, _, _, _, _, _) => {
                                        h2318
                                    }
                                };
                                nat_mul(a2319, a2320)?
                            };
                            let a2322 = v8;
                            nat_add(a2321, a2322)?
                        }, v4.clone())?, {
                            let a2329 = {
                                let a2327 = {
                                    let a2324 = match v7 {
                                        Adt11::C0(_, h2323, _) => {
                                            h2323
                                        }
                                    };
                                    let a2325 = 1u64;
                                    nat_sub(a2324, a2325)
                                };
                                let a2328 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h2326, _, _, _) => {
                                        h2326
                                    }
                                };
                                nat_mul(a2327, a2328)?
                            };
                            let a2330 = 4u64;
                            nat_add(a2329, a2330)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h2331, _, _, _) => {
                                h2331
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> u64 {
    let a2332 = v1;
    let a2333 = 0u64;
    if if nat_eq(a2332, a2333) {
        true
    } else {
        let a2334 = v2;
        let a2335 = 0u64;
        nat_eq(a2334, a2335)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f109(v0: u64, v1: u64) -> bool {
    let a2336 = v0;
    let a2337 = 4294967295u64;
    if nat_eq(a2336, a2337) {
        true
    } else {
        let a2338 = v0;
        let a2339 = 2147483648u64;
        if nat_lt(a2338, a2339) {
            let a2340 = v0;
            let a2341 = v1;
            nat_lt(a2340, a2341)
        } else {
            false
        }
    }
}

pub fn f110(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2348 = v3;
        let a2349 = {
            let a2345 = f89({
                let a2342 = v0;
                let a2343 = 2u64;
                let a2344 = 0u64;
                nat_quot(a2342, a2343, a2344)
            }, v1)?;
            let a2346 = v2;
            let a2347 = 0u64;
            nat_quot(a2345, a2346, a2347)
        };
        nat_add(a2348, a2349)?
    };
    let a2353 = {
        let a2350 = v0;
        let a2351 = 2u64;
        let a2352 = 0u64;
        nat_rem(a2350, a2351, a2352)
    };
    let a2354 = 0u64;
    if nat_eq(a2353, a2354) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2355 = v4;
        let a2356 = 0u64;
        if nat_eq(a2355, a2356) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a2359 = 8u64;
    let a2360 = {
        let a2357 = f147(v0);
        let a2358 = v1;
        nat_mul(a2357, a2358)?
    };
    nat_add(a2359, a2360)
}

pub fn f112(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2361, _, _, _, _, _, _, _) => {
            h2361
        }
    };
    let v7: u64 = f147(v3);
    let v8: u64 = {
        let a2364 = 1u64;
        let a2365 = {
            let a2362 = v6;
            let a2363 = v7;
            nat_mul(a2362, a2363)?
        };
        nat_add(a2364, a2365)?
    };
    let a2366 = v0;
    let a2367 = 0u64;
    if nat_eq(a2366, a2367) {
        Ok(f148(0u64))
    } else {
        let a2368 = v0;
        let a2369 = v8;
        if nat_lt(a2368, a2369) {
            let v9: u64 = {
                let a2370 = v0;
                let a2371 = 1u64;
                nat_sub(a2370, a2371)
            };
            let v10: u64 = {
                let a2372 = v9;
                let a2373 = v7;
                let a2374 = 0u64;
                nat_rem(a2372, a2373, a2374)
            };
            let v11: u64 = f89({
                let a2380 = {
                    let a2375 = v1;
                    let a2376 = v6;
                    nat_mul(a2375, a2376)?
                };
                let a2381 = {
                    let a2377 = v9;
                    let a2378 = v7;
                    let a2379 = 0u64;
                    nat_quot(a2377, a2378, a2379)
                };
                nat_add(a2380, a2381)?
            }, v2)?;
            let a2382 = v3;
            let a2383 = 30u64;
            if nat_eq(a2382, a2383) {
                let a2384 = v10;
                let a2385 = 0u64;
                if nat_eq(a2384, a2385) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2386 = v10;
                    let a2387 = 1u64;
                    if nat_eq(a2386, a2387) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2388 = v10;
                        let a2389 = 2u64;
                        if nat_eq(a2388, a2389) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2390 = v10;
                let a2391 = 0u64;
                if nat_eq(a2390, a2391) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2392 = v10;
                    let a2393 = 1u64;
                    if nat_eq(a2392, a2393) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2394 = v0;
                let a2395 = v8;
                nat_sub(a2394, a2395)
            };
            let a2396 = v12;
            let a2397 = 0u64;
            if nat_eq(a2396, a2397) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2398 = v12;
                let a2399 = 1u64;
                if nat_eq(a2398, a2399) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2400 = v12;
                    let a2401 = 2u64;
                    if nat_eq(a2400, a2401) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2402 = v12;
                        let a2403 = 3u64;
                        if nat_eq(a2402, a2403) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2404 = v12;
                            let a2405 = 4u64;
                            if nat_eq(a2404, a2405) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2406 = v12;
                                let a2407 = 5u64;
                                if nat_eq(a2406, a2407) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2408 = v12;
                                    let a2409 = 6u64;
                                    if nat_eq(a2408, a2409) {
                                        if f149(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2410 = 3u64;
                                                let a2411 = v1;
                                                nat_add(a2410, a2411)?
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
        let a2412 = v0;
        let a2413 = f65(v3);
        let a2414 = 0u64;
        nat_quot(a2412, a2413, a2414)
    };
    let v5: u64 = {
        let a2419 = 3u64;
        let a2420 = {
            let a2416 = v4;
            let a2417 = match v1.clone() {
                Adt3::C0(h2415, _, _, _, _, _, _, _) => {
                    h2415
                }
            };
            let a2418 = 0u64;
            nat_quot(a2416, a2417, a2418)
        };
        nat_add(a2419, a2420)?
    };
    let v6: u64 = f89(v4, v1)?;
    let v7: u64 = f89(v4, v2)?;
    let a2421 = v3;
    let a2422 = 30u64;
    if nat_eq(a2421, a2422) {
        let a2426 = {
            let a2423 = v0;
            let a2424 = 3u64;
            let a2425 = 0u64;
            nat_rem(a2423, a2424, a2425)
        };
        let a2427 = 0u64;
        if nat_eq(a2426, a2427) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2431 = {
                let a2428 = v0;
                let a2429 = 3u64;
                let a2430 = 0u64;
                nat_rem(a2428, a2429, a2430)
            };
            let a2432 = 1u64;
            if nat_eq(a2431, a2432) {
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
        let a2433 = v0;
        let a2434 = 8388608u64;
        let a2435 = 0u64;
        nat_quot(a2433, a2434, a2435)
    };
    let a2436 = v0;
    let a2437 = 2147483648u64;
    if if if nat_lt(a2436, a2437) {
        let a2438 = 127u64;
        let a2439 = v1;
        nat_le(a2438, a2439)
    } else {
        false
    } {
        let a2440 = v1;
        let a2441 = 190u64;
        nat_lt(a2440, a2441)
    } else {
        false
    } {
        let v2: u64 = {
            let a2445 = 8388608u64;
            let a2446 = {
                let a2442 = v0;
                let a2443 = 8388608u64;
                let a2444 = 0u64;
                nat_rem(a2442, a2443, a2444)
            };
            nat_add(a2445, a2446)?
        };
        let a2447 = v1;
        let a2448 = 150u64;
        if nat_lt(a2447, a2448) {
            let v3: u64 = f150({
                let a2449 = 150u64;
                let a2450 = v1;
                nat_sub(a2449, a2450)
            });
            let a2454 = {
                let a2451 = v2;
                let a2452 = v3;
                let a2453 = 0u64;
                nat_rem(a2451, a2452, a2453)
            };
            let a2455 = 0u64;
            if nat_eq(a2454, a2455) {
                let a2456 = v2;
                let a2457 = v3;
                let a2458 = 0u64;
                Ok(nat_quot(a2456, a2457, a2458))
            } else {
                Ok(0u64)
            }
        } else {
            let a2461 = f150({
                let a2459 = v1;
                let a2460 = 150u64;
                nat_sub(a2459, a2460)
            });
            let a2462 = v2;
            nat_mul(a2461, a2462)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> bool {
    let a2463 = v0;
    let a2464 = 1u64;
    if nat_le(a2463, a2464) {
        true
    } else {
        let a2468 = {
            let a2465 = v1;
            let a2466 = v2;
            let a2467 = 1u64;
            nat_rem(a2465, a2466, a2467)
        };
        let a2469 = 0u64;
        nat_eq(a2468, a2469)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2470 = v1;
    let a2471 = 1u64;
    if if if nat_le(a2470, a2471) {
        true
    } else {
        let a2472 = v0;
        let a2473 = v2;
        nat_le(a2472, a2473)
    } {
        let a2474 = v3;
        let a2475 = 1u64;
        if nat_le(a2474, a2475) {
            true
        } else {
            let a2482 = {
                let a2480 = v0;
                let a2481 = {
                    let a2478 = {
                        let a2476 = v1;
                        let a2477 = 1u64;
                        nat_sub(a2476, a2477)
                    };
                    let a2479 = v2;
                    nat_mul(a2478, a2479)?
                };
                nat_add(a2480, a2481)?
            };
            let a2483 = v4;
            nat_le(a2482, a2483)
        }
    } else {
        false
    } {
        let a2484 = v5;
        let a2485 = 1u64;
        if nat_le(a2484, a2485) {
            Ok(true)
        } else {
            let a2498 = {
                let a2496 = {
                    let a2490 = v0;
                    let a2491 = {
                        let a2488 = {
                            let a2486 = v1;
                            let a2487 = 1u64;
                            nat_sub(a2486, a2487)
                        };
                        let a2489 = v2;
                        nat_mul(a2488, a2489)?
                    };
                    nat_add(a2490, a2491)?
                };
                let a2497 = {
                    let a2494 = {
                        let a2492 = v3;
                        let a2493 = 1u64;
                        nat_sub(a2492, a2493)
                    };
                    let a2495 = v4;
                    nat_mul(a2494, a2495)?
                };
                nat_add(a2496, a2497)?
            };
            let a2499 = v6;
            Ok(nat_le(a2498, a2499))
        }
    } else {
        Ok(false)
    }
}

pub fn f117(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f151(v0, v1, v6, v7, v8) {
        let a2500 = v5;
        let a2501 = 26u64;
        nat_eq(a2500, a2501)
    } else {
        false
    } {
        let a2502 = v3;
        let a2503 = v7;
        nat_eq(a2502, a2503)
    } else {
        false
    } {
        let a2504 = v4;
        let a2505 = v8;
        nat_eq(a2504, a2505)
    } else {
        false
    } {
        let a2506 = v9;
        let a2507 = 1u64;
        nat_eq(a2506, a2507)
    } else {
        false
    } {
        let a2508 = v10;
        let a2509 = f152(v0);
        nat_eq(a2508, a2509)
    } else {
        false
    } {
        let a2510 = v11;
        let a2511 = v1;
        nat_eq(a2510, a2511)
    } else {
        false
    } {
        let a2512 = v12;
        let a2513 = v6;
        nat_eq(a2512, a2513)
    } else {
        false
    } {
        let a2514 = v13;
        let a2515 = v7;
        nat_eq(a2514, a2515)
    } else {
        false
    } {
        let a2516 = v14;
        let a2517 = v8;
        nat_eq(a2516, a2517)
    } else {
        false
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2518 = v0;
    let a2519 = 0u64;
    if if if nat_eq(a2518, a2519) {
        true
    } else {
        let a2520 = v1;
        let a2521 = 0u64;
        nat_eq(a2520, a2521)
    } {
        true
    } else {
        let a2522 = v2;
        let a2523 = 0u64;
        nat_eq(a2522, a2523)
    } {
        true
    } else {
        let a2524 = v3;
        let a2525 = 0u64;
        nat_eq(a2524, a2525)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a2526 = v0;
    let a2527 = 2147483648u64;
    if nat_lt(a2526, a2527) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: u64) -> u64 {
    let a2528 = v0;
    let a2529 = 2147483648u64;
    if nat_lt(a2528, a2529) {
        0u64
    } else {
        let a2530 = 4294967296u64;
        let a2531 = v0;
        nat_sub(a2530, a2531)
    }
}

pub fn f121(v0: Adt3, v1: bool) -> R<u64> {
    f153(v0.clone(), {
        let a2533 = match v0.clone() {
            Adt3::C0(h2532, _, _, _, _, _, _, _) => {
                h2532
            }
        };
        let a2534 = 1u64;
        nat_sub(a2533, a2534)
    }, {
        let a2536 = match v0.clone() {
            Adt3::C0(_, h2535, _, _, _, _, _, _) => {
                h2535
            }
        };
        let a2537 = 1u64;
        nat_sub(a2536, a2537)
    }, {
        let a2539 = match v0.clone() {
            Adt3::C0(_, _, h2538, _, _, _, _, _) => {
                h2538
            }
        };
        let a2540 = 1u64;
        nat_sub(a2539, a2540)
    }, {
        let a2542 = match v0 {
            Adt3::C0(_, _, _, h2541, _, _, _, _) => {
                h2541
            }
        };
        let a2543 = 1u64;
        nat_sub(a2542, a2543)
    }, v1)
}

pub fn f122(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2544 = v0;
    let a2545 = 0u64;
    if nat_eq(a2544, a2545) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2546 = v0;
            let a2547 = 1u64;
            nat_sub(a2546, a2547)
        };
        let v5: u64 = {
            let a2548 = v4;
            let a2549 = v1;
            let a2550 = 0u64;
            nat_quot(a2548, a2549, a2550)
        };
        let v6: u64 = {
            let a2558 = {
                let a2551 = v5;
                let a2552 = v2;
                nat_mul(a2551, a2552)?
            };
            let a2559 = {
                let a2556 = {
                    let a2553 = v4;
                    let a2554 = v1;
                    let a2555 = 0u64;
                    nat_rem(a2553, a2554, a2555)
                };
                let a2557 = v3;
                nat_mul(a2556, a2557)?
            };
            nat_add(a2558, a2559)?
        };
        let a2560 = v5;
        let a2561 = 0u64;
        if nat_eq(a2560, a2561) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2570 = {
                    let a2564 = {
                        let a2562 = v5;
                        let a2563 = 1u64;
                        nat_sub(a2562, a2563)
                    };
                    let a2565 = v2;
                    nat_mul(a2564, a2565)?
                };
                let a2571 = {
                    let a2568 = {
                        let a2566 = v1;
                        let a2567 = 1u64;
                        nat_sub(a2566, a2567)
                    };
                    let a2569 = v3;
                    nat_mul(a2568, a2569)?
                };
                nat_add(a2570, a2571)?
            };
            let a2572 = v6;
            let a2573 = v7;
            if nat_lt(a2572, a2573) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f123(v0: Adt3, v1: u64) -> bool {
    let a2574 = v1;
    let a2575 = 0u64;
    if !nat_eq(a2574, a2575) {
        true
    } else {
        let a2577 = match v0 {
            Adt3::C0(_, _, _, _, h2576, _, _, _) => {
                h2576
            }
        };
        let a2578 = 4u64;
        nat_eq(a2577, a2578)
    }
}

pub fn f124(v0: u64, v1: bool) -> R<u64> {
    let a2581 = {
        let a2579 = v0;
        let a2580 = 2u64;
        nat_mul(a2579, a2580)?
    };
    let a2582 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2581, a2582)
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2583 = f154(v0, v1);
    match m2583 {
        None => {
            if v5 {
                let m2584 = f155(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2584 {
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
                let a2585 = 4u64;
                let a2586 = v28;
                nat_le(a2585, a2586)
            };
            let a2587 = v2;
            let a2588 = 0u64;
            if if if if if nat_eq(a2587, a2588) {
                let a2589 = v4;
                let a2590 = 0u64;
                nat_eq(a2589, a2590)
            } else {
                false
            } {
                let a2591 = v6;
                let a2592 = v14;
                if nat_eq(a2591, a2592) {
                    let a2593 = v7;
                    let a2594 = v15;
                    nat_eq(a2593, a2594)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2595 = v8;
                let a2596 = v16;
                if nat_eq(a2595, a2596) {
                    let a2597 = v9;
                    let a2598 = v17;
                    nat_eq(a2597, a2598)
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
                        let a2599 = v3;
                        let a2600 = 0u64;
                        nat_eq(a2599, a2600)
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
                    let a2605 = {
                        let a2601 = v6;
                        let a2602 = v7;
                        nat_mul(a2601, a2602)?
                    };
                    let a2606 = {
                        let a2603 = v8;
                        let a2604 = v9;
                        nat_mul(a2603, a2604)?
                    };
                    nat_mul(a2605, a2606)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f126(v0: u64, v1: u64) -> Adt8 {
    let a2607 = v0;
    let a2608 = 0u64;
    if nat_eq(a2607, a2608) {
        Adt8::C0(0u64, v1)
    } else {
        let a2609 = v0;
        let a2610 = 1u64;
        if nat_eq(a2609, a2610) {
            Adt8::C1(v1)
        } else {
            let a2611 = v0;
            let a2612 = 2u64;
            if nat_eq(a2611, a2612) {
                Adt8::C2(v1)
            } else {
                let a2613 = v0;
                let a2614 = 3u64;
                if nat_eq(a2613, a2614) {
                    Adt8::C3(v1)
                } else {
                    let a2615 = v0;
                    let a2616 = 13u64;
                    if nat_lt(a2615, a2616) {
                        Adt8::C0({
                            let a2617 = v0;
                            let a2618 = 3u64;
                            nat_sub(a2617, a2618)
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
    let a2619 = v0;
    let a2620 = 1u64;
    if nat_eq(a2619, a2620) {
        Adt9::C0(v1)
    } else {
        let a2621 = v0;
        let a2622 = 3u64;
        if if nat_eq(a2621, a2622) {
            true
        } else {
            let a2623 = v0;
            let a2624 = 6u64;
            nat_eq(a2623, a2624)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f128(v0: u64) -> u64 {
    let m2625 = f70(v0);
    match m2625 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a2627 = match v1.clone() {
                Adt11::C0(_, h2626, _) => {
                    h2626
                }
            };
            let a2628 = 1u64;
            if if nat_eq(a2627, a2628) {
                let a2630 = match v1.clone() {
                    Adt11::C0(_, _, h2629) => {
                        h2629
                    }
                };
                let a2631 = 1u64;
                if if if nat_eq(a2630, a2631) {
                    true
                } else {
                    let a2633 = match v1.clone() {
                        Adt11::C0(_, _, h2632) => {
                            h2632
                        }
                    };
                    let a2634 = 2u64;
                    nat_eq(a2633, a2634)
                } {
                    true
                } else {
                    let a2636 = match v1.clone() {
                        Adt11::C0(_, _, h2635) => {
                            h2635
                        }
                    };
                    let a2637 = 4u64;
                    nat_eq(a2636, a2637)
                } {
                    true
                } else {
                    let a2639 = match v1.clone() {
                        Adt11::C0(_, _, h2638) => {
                            h2638
                        }
                    };
                    let a2640 = 8u64;
                    nat_eq(a2639, a2640)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h2641) => {
                        h2641
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2654 = {
        let a2650 = {
            let a2646 = {
                let a2642 = v0;
                let a2643 = v4;
                nat_mul(a2642, a2643)?
            };
            let a2647 = {
                let a2644 = v1;
                let a2645 = v5;
                nat_mul(a2644, a2645)?
            };
            nat_add(a2646, a2647)?
        };
        let a2651 = {
            let a2648 = v2;
            let a2649 = v6;
            nat_mul(a2648, a2649)?
        };
        nat_add(a2650, a2651)?
    };
    let a2655 = {
        let a2652 = v3;
        let a2653 = v7;
        nat_mul(a2652, a2653)?
    };
    nat_add(a2654, a2655)
}

pub fn f130(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2660 = {
        let a2658 = {
            let a2656 = v0;
            let a2657 = v1;
            nat_sub(a2656, a2657)
        };
        let a2659 = 1u64;
        nat_sub(a2658, a2659)
    };
    let a2661 = v2;
    nat_add(a2660, a2661)
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f129({
        let a2662 = v0;
        let a2663 = v4;
        let a2664 = 0u64;
        nat_rem(a2662, a2663, a2664)
    }, {
        let a2665 = v1;
        let a2666 = v5;
        let a2667 = 0u64;
        nat_rem(a2665, a2666, a2667)
    }, {
        let a2668 = v2;
        let a2669 = v6;
        let a2670 = 0u64;
        nat_rem(a2668, a2669, a2670)
    }, {
        let a2671 = v3;
        let a2672 = v7;
        let a2673 = 0u64;
        nat_rem(a2671, a2672, a2673)
    }, v8, v9, v10, v11)
}

pub fn f132(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2675 = v0;
        let a2676 = match v1.clone() {
            Adt3::C0(h2674, _, _, _, _, _, _, _) => {
                h2674
            }
        };
        let a2677 = 0u64;
        nat_quot(a2675, a2676, a2677)
    };
    let v4: u64 = {
        let a2679 = v3;
        let a2680 = match v1.clone() {
            Adt3::C0(_, h2678, _, _, _, _, _, _) => {
                h2678
            }
        };
        let a2681 = 0u64;
        nat_quot(a2679, a2680, a2681)
    };
    let a2682 = v2;
    let a2683 = 0u64;
    if nat_eq(a2682, a2683) {
        let a2685 = v0;
        let a2686 = match v1 {
            Adt3::C0(h2684, _, _, _, _, _, _, _) => {
                h2684
            }
        };
        let a2687 = 0u64;
        nat_rem(a2685, a2686, a2687)
    } else {
        let a2688 = v2;
        let a2689 = 1u64;
        if nat_eq(a2688, a2689) {
            let a2691 = v3;
            let a2692 = match v1 {
                Adt3::C0(_, h2690, _, _, _, _, _, _) => {
                    h2690
                }
            };
            let a2693 = 0u64;
            nat_rem(a2691, a2692, a2693)
        } else {
            let a2694 = v2;
            let a2695 = 2u64;
            if nat_eq(a2694, a2695) {
                let a2697 = v4;
                let a2698 = match v1 {
                    Adt3::C0(_, _, h2696, _, _, _, _, _) => {
                        h2696
                    }
                };
                let a2699 = 0u64;
                nat_rem(a2697, a2698, a2699)
            } else {
                let a2701 = v4;
                let a2702 = match v1 {
                    Adt3::C0(_, _, h2700, _, _, _, _, _) => {
                        h2700
                    }
                };
                let a2703 = 0u64;
                nat_quot(a2701, a2702, a2703)
            }
        }
    }
}

pub fn f133(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f129(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2704, _, _, _) => {
            h2704
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2705, _, _) => {
            h2705
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2706, _) => {
            h2706
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2707) => {
            h2707
        }
    })
}

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2708 = v1;
    let a2709 = 2147483648u64;
    if nat_lt(a2708, a2709) {
        let a2720 = {
            let a2718 = {
                let a2713 = {
                    let a2710 = v0;
                    let a2711 = v2;
                    let a2712 = 0u64;
                    nat_rem(a2710, a2711, a2712)
                };
                let a2714 = v2;
                nat_add(a2713, a2714)?
            };
            let a2719 = {
                let a2715 = v1;
                let a2716 = v2;
                let a2717 = 0u64;
                nat_rem(a2715, a2716, a2717)
            };
            nat_sub(a2718, a2719)
        };
        let a2721 = v2;
        let a2722 = 0u64;
        Ok(nat_rem(a2720, a2721, a2722))
    } else {
        let a2725 = {
            let a2723 = v0;
            let a2724 = f120(v1);
            nat_add(a2723, a2724)?
        };
        let a2726 = v2;
        let a2727 = 0u64;
        Ok(nat_rem(a2725, a2726, a2727))
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2730 = f119(v1);
    let a2731 = {
        let a2728 = v0;
        let a2729 = f120(v1);
        nat_add(a2728, a2729)?
    };
    if nat_le(a2730, a2731) {
        let a2732 = f136(v0, v1)?;
        let a2733 = v2;
        Ok(nat_lt(a2732, a2733))
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a2736 = {
        let a2734 = v0;
        let a2735 = f120(v1);
        nat_add(a2734, a2735)?
    };
    let a2737 = f119(v1);
    Ok(nat_sub(a2736, a2737))
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2738 = v0;
    let a2739 = v1;
    if nat_lt(a2738, a2739) {
        let a2740 = v1;
        let a2741 = v0;
        Ok(nat_sub(a2740, a2741))
    } else {
        let a2744 = v0;
        let a2745 = {
            let a2742 = v1;
            let a2743 = v2;
            nat_add(a2742, a2743)?
        };
        if nat_lt(a2744, a2745) {
            let a2746 = v0;
            let a2747 = v1;
            Ok(nat_sub(a2746, a2747))
        } else {
            let a2754 = {
                let a2750 = 2u64;
                let a2751 = {
                    let a2748 = v2;
                    let a2749 = 1u64;
                    nat_sub(a2748, a2749)
                };
                nat_mul(a2750, a2751)?
            };
            let a2755 = {
                let a2752 = v0;
                let a2753 = v1;
                nat_sub(a2752, a2753)
            };
            Ok(nat_sub(a2754, a2755))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2758 = {
        let a2756 = v2;
        let a2757 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1.clone(), 3u64), false)?;
        nat_add(a2756, a2757)?
    };
    let a2759 = f153(v1.clone(), f132(v0, v1.clone(), 0u64), f132(v0, v1.clone(), 1u64), f132(v0, v1.clone(), 2u64), f132(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2758, a2759))
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a2760 = v0;
    let a2761 = 0u64;
    if nat_eq(a2760, a2761) {
        let a2762 = v2;
        let a2763 = v1;
        nat_le(a2762, a2763)
    } else {
        let a2764 = v0;
        let a2765 = 1u64;
        if nat_eq(a2764, a2765) {
            let a2766 = v2;
            let a2767 = v1;
            nat_lt(a2766, a2767)
        } else {
            let a2768 = v0;
            let a2769 = 2u64;
            if nat_eq(a2768, a2769) {
                let a2770 = v1;
                let a2771 = v2;
                nat_le(a2770, a2771)
            } else {
                let a2772 = v0;
                let a2773 = 3u64;
                if nat_eq(a2772, a2773) {
                    let a2774 = v1;
                    let a2775 = v2;
                    nat_lt(a2774, a2775)
                } else {
                    let a2776 = v1;
                    let a2777 = v2;
                    nat_eq(a2776, a2777)
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2778 = v0;
    let a2779 = 0u64;
    if nat_eq(a2778, a2779) {
        Ok(f148(0u64))
    } else {
        let a2782 = {
            let a2780 = v0;
            let a2781 = 1u64;
            nat_add(a2780, a2781)?
        };
        let a2783 = f82(v1.clone(), 13u64, v2)?;
        if nat_eq(a2782, a2783) {
            Ok(Adt5::C4({
                let a2784 = v2;
                let a2785 = 0u64;
                if nat_eq(a2784, a2785) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2786 = v0;
                let a2787 = 1u64;
                nat_sub(a2786, a2787)
            };
            if f123(v1.clone(), v2) {
                let v4: u64 = f124(match v1.clone() {
                    Adt3::C0(h2788, _, _, _, _, _, _, _) => {
                        h2788
                    }
                }, false)?;
                let v5: u64 = {
                    let a2789 = v3;
                    let a2790 = v4;
                    let a2791 = 0u64;
                    nat_quot(a2789, a2790, a2791)
                };
                let v6: u64 = {
                    let a2792 = v3;
                    let a2793 = v4;
                    let a2794 = 0u64;
                    nat_rem(a2792, a2793, a2794)
                };
                let a2797 = {
                    let a2795 = v6;
                    let a2796 = 1u64;
                    nat_add(a2795, a2796)?
                };
                let a2798 = v4;
                if nat_eq(a2797, a2798) {
                    Ok(Adt5::C4({
                        let a2799 = v2;
                        let a2800 = 0u64;
                        if nat_eq(a2799, a2800) {
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
                    let a2801 = v3;
                    let a2802 = 2u64;
                    let a2803 = 0u64;
                    nat_quot(a2801, a2802, a2803)
                }, {
                    let a2804 = v3;
                    let a2805 = 2u64;
                    let a2806 = 0u64;
                    nat_rem(a2804, a2805, a2806)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2807 = v0;
    let a2808 = 0u64;
    if nat_eq(a2807, a2808) {
        Ok(f148(1u64))
    } else {
        f156({
            let a2817 = {
                let a2810 = v1;
                let a2811 = match v2.clone() {
                    Adt3::C0(h2809, _, _, _, _, _, _, _) => {
                        h2809
                    }
                };
                nat_mul(a2810, a2811)?
            };
            let a2818 = {
                let a2814 = {
                    let a2812 = v0;
                    let a2813 = 1u64;
                    nat_sub(a2812, a2813)
                };
                let a2815 = 2u64;
                let a2816 = 0u64;
                nat_quot(a2814, a2815, a2816)
            };
            nat_add(a2817, a2818)?
        }, {
            let a2821 = {
                let a2819 = v0;
                let a2820 = 1u64;
                nat_sub(a2819, a2820)
            };
            let a2822 = 2u64;
            let a2823 = 0u64;
            nat_rem(a2821, a2822, a2823)
        }, v2, v3, 1u64)
    }
}

pub fn f142(v0: u64, v1: u64) -> R<u64> {
    let a2824 = v0;
    let a2825 = f60(v1)?;
    let a2826 = 0u64;
    Ok(nat_rem(a2824, a2825, a2826))
}

pub fn f143(v0: u64, v1: u64) -> R<u64> {
    let a2827 = 4u64;
    let a2828 = f157(v0, v1)?;
    nat_mul(a2827, a2828)
}

pub fn f144(v0: u64, v1: u64) -> R<u64> {
    let a2829 = 4u64;
    let a2830 = f158(v0, v1)?;
    nat_mul(a2829, a2830)
}

pub fn f145(v0: u64, v1: u64) -> R<bool> {
    let a2836 = {
        let a2833 = {
            let a2831 = f142(v0, v1)?;
            let a2832 = 2u64;
            nat_sub(a2831, a2832)
        };
        let a2834 = 2u64;
        let a2835 = 0u64;
        nat_rem(a2833, a2834, a2835)
    };
    let a2837 = 0u64;
    Ok(nat_eq(a2836, a2837))
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2838 = f70(v0);
    match m2838 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f159(v0) {
                let a2839 = v1;
                let a2840 = v7;
                nat_lt(a2839, a2840)
            } else {
                false
            } {
                f106(v8, v2)
            } else {
                false
            } {
                let a2841 = v3;
                let a2842 = f160(v4, v5, v6)?;
                nat_lt(a2841, a2842)
            } else {
                false
            } {
                let a2847 = {
                    let a2844 = v7;
                    let a2845 = match v13.clone() {
                        Adt11::C0(_, h2843, _) => {
                            h2843
                        }
                    };
                    let a2846 = 0u64;
                    nat_rem(a2844, a2845, a2846)
                };
                let a2848 = 0u64;
                nat_eq(a2847, a2848)
            } else {
                false
            } {
                Ok(Some({
                    let a2877 = {
                        let a2867 = {
                            let a2857 = {
                                let a2853 = {
                                    let a2850 = v1;
                                    let a2851 = match v13 {
                                        Adt11::C0(_, h2849, _) => {
                                            h2849
                                        }
                                    };
                                    let a2852 = 0u64;
                                    nat_quot(a2850, a2851, a2852)
                                };
                                let a2854 = v9;
                                nat_mul(a2853, a2854)?
                            };
                            let a2858 = {
                                let a2855 = v2;
                                let a2856 = v10;
                                nat_mul(a2855, a2856)?
                            };
                            nat_add(a2857, a2858)?
                        };
                        let a2868 = {
                            let a2865 = {
                                let a2862 = {
                                    let a2859 = v3;
                                    let a2860 = v4;
                                    let a2861 = 0u64;
                                    nat_quot(a2859, a2860, a2861)
                                };
                                let a2863 = v5;
                                let a2864 = 0u64;
                                nat_rem(a2862, a2863, a2864)
                            };
                            let a2866 = v11;
                            nat_mul(a2865, a2866)?
                        };
                        nat_add(a2867, a2868)?
                    };
                    let a2878 = {
                        let a2875 = {
                            let a2872 = {
                                let a2869 = v3;
                                let a2870 = v4;
                                let a2871 = 0u64;
                                nat_quot(a2869, a2870, a2871)
                            };
                            let a2873 = v5;
                            let a2874 = 0u64;
                            nat_quot(a2872, a2873, a2874)
                        };
                        let a2876 = v12;
                        nat_mul(a2875, a2876)?
                    };
                    nat_add(a2877, a2878)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f147(v0: u64) -> u64 {
    let a2879 = v0;
    let a2880 = 30u64;
    if nat_eq(a2879, a2880) {
        4u64
    } else {
        3u64
    }
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64) -> bool {
    let a2881 = 0u64;
    let a2882 = v0;
    if nat_lt(a2881, a2882) {
        let a2883 = v0;
        let a2884 = 2139095040u64;
        nat_le(a2883, a2884)
    } else {
        false
    }
}

pub fn f150(v0: u64) -> u64 {
    let a2885 = v0;
    let a2886 = 40u64;
    if nat_lt(a2885, a2886) {
        let a2887 = v0;
        let a2888 = 20u64;
        if nat_lt(a2887, a2888) {
            let a2889 = v0;
            let a2890 = 10u64;
            if nat_lt(a2889, a2890) {
                let a2891 = v0;
                let a2892 = 5u64;
                if nat_lt(a2891, a2892) {
                    let a2893 = v0;
                    let a2894 = 2u64;
                    if nat_lt(a2893, a2894) {
                        let a2895 = v0;
                        let a2896 = 1u64;
                        if nat_lt(a2895, a2896) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2897 = v0;
                        let a2898 = 3u64;
                        if nat_lt(a2897, a2898) {
                            4u64
                        } else {
                            let a2899 = v0;
                            let a2900 = 4u64;
                            if nat_lt(a2899, a2900) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2901 = v0;
                    let a2902 = 7u64;
                    if nat_lt(a2901, a2902) {
                        let a2903 = v0;
                        let a2904 = 6u64;
                        if nat_lt(a2903, a2904) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2905 = v0;
                        let a2906 = 8u64;
                        if nat_lt(a2905, a2906) {
                            128u64
                        } else {
                            let a2907 = v0;
                            let a2908 = 9u64;
                            if nat_lt(a2907, a2908) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2909 = v0;
                let a2910 = 15u64;
                if nat_lt(a2909, a2910) {
                    let a2911 = v0;
                    let a2912 = 12u64;
                    if nat_lt(a2911, a2912) {
                        let a2913 = v0;
                        let a2914 = 11u64;
                        if nat_lt(a2913, a2914) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2915 = v0;
                        let a2916 = 13u64;
                        if nat_lt(a2915, a2916) {
                            4096u64
                        } else {
                            let a2917 = v0;
                            let a2918 = 14u64;
                            if nat_lt(a2917, a2918) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2919 = v0;
                    let a2920 = 17u64;
                    if nat_lt(a2919, a2920) {
                        let a2921 = v0;
                        let a2922 = 16u64;
                        if nat_lt(a2921, a2922) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2923 = v0;
                        let a2924 = 18u64;
                        if nat_lt(a2923, a2924) {
                            131072u64
                        } else {
                            let a2925 = v0;
                            let a2926 = 19u64;
                            if nat_lt(a2925, a2926) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2927 = v0;
            let a2928 = 30u64;
            if nat_lt(a2927, a2928) {
                let a2929 = v0;
                let a2930 = 25u64;
                if nat_lt(a2929, a2930) {
                    let a2931 = v0;
                    let a2932 = 22u64;
                    if nat_lt(a2931, a2932) {
                        let a2933 = v0;
                        let a2934 = 21u64;
                        if nat_lt(a2933, a2934) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2935 = v0;
                        let a2936 = 23u64;
                        if nat_lt(a2935, a2936) {
                            4194304u64
                        } else {
                            let a2937 = v0;
                            let a2938 = 24u64;
                            if nat_lt(a2937, a2938) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2939 = v0;
                    let a2940 = 27u64;
                    if nat_lt(a2939, a2940) {
                        let a2941 = v0;
                        let a2942 = 26u64;
                        if nat_lt(a2941, a2942) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2943 = v0;
                        let a2944 = 28u64;
                        if nat_lt(a2943, a2944) {
                            134217728u64
                        } else {
                            let a2945 = v0;
                            let a2946 = 29u64;
                            if nat_lt(a2945, a2946) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2947 = v0;
                let a2948 = 35u64;
                if nat_lt(a2947, a2948) {
                    let a2949 = v0;
                    let a2950 = 32u64;
                    if nat_lt(a2949, a2950) {
                        let a2951 = v0;
                        let a2952 = 31u64;
                        if nat_lt(a2951, a2952) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2953 = v0;
                        let a2954 = 33u64;
                        if nat_lt(a2953, a2954) {
                            4294967296u64
                        } else {
                            let a2955 = v0;
                            let a2956 = 34u64;
                            if nat_lt(a2955, a2956) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2957 = v0;
                    let a2958 = 37u64;
                    if nat_lt(a2957, a2958) {
                        let a2959 = v0;
                        let a2960 = 36u64;
                        if nat_lt(a2959, a2960) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2961 = v0;
                        let a2962 = 38u64;
                        if nat_lt(a2961, a2962) {
                            137438953472u64
                        } else {
                            let a2963 = v0;
                            let a2964 = 39u64;
                            if nat_lt(a2963, a2964) {
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
    let a2965 = v0;
    let a2966 = 26u64;
    if nat_eq(a2965, a2966) {
        26u64
    } else {
        0u64
    }
}

pub fn f153(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2987 = {
        let a2981 = {
            let a2975 = {
                let a2969 = v1;
                let a2970 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2967, _, _, _) => {
                            h2967
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2968, _, _, _) => {
                            h2968
                        }
                    })
                };
                nat_mul(a2969, a2970)?
            };
            let a2976 = {
                let a2973 = v2;
                let a2974 = if v5 {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2971, _, _) => {
                            h2971
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2972, _, _) => {
                            h2972
                        }
                    })
                };
                nat_mul(a2973, a2974)?
            };
            nat_add(a2975, a2976)?
        };
        let a2982 = {
            let a2979 = v3;
            let a2980 = if v5 {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2977, _) => {
                        h2977
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2978, _) => {
                        h2978
                    }
                })
            };
            nat_mul(a2979, a2980)?
        };
        nat_add(a2981, a2982)?
    };
    let a2988 = {
        let a2985 = v4;
        let a2986 = if v5 {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2983) => {
                    h2983
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2984) => {
                    h2984
                }
            })
        };
        nat_mul(a2985, a2986)?
    };
    nat_add(a2987, a2988)
}

pub fn f154(v0: u64, v1: u64) -> Option<u64> {
    let a2989 = v0;
    let a2990 = 2u64;
    if nat_eq(a2989, a2990) {
        Some(0u64)
    } else {
        let a2991 = v0;
        let a2992 = 5u64;
        if nat_eq(a2991, a2992) {
            Some(1u64)
        } else {
            let a2993 = v0;
            let a2994 = 6u64;
            if nat_eq(a2993, a2994) {
                Some(2u64)
            } else {
                let a2995 = v0;
                let a2996 = 7u64;
                if nat_eq(a2995, a2996) {
                    Some(3u64)
                } else {
                    let a2997 = v0;
                    let a2998 = 9u64;
                    if nat_eq(a2997, a2998) {
                        Some(4u64)
                    } else {
                        let a2999 = v0;
                        let a3000 = 8u64;
                        if nat_eq(a2999, a3000) {
                            Some(5u64)
                        } else {
                            let a3001 = v0;
                            let a3002 = 89u64;
                            if nat_eq(a3001, a3002) {
                                let a3003 = v1;
                                let a3004 = 2u64;
                                if nat_eq(a3003, a3004) {
                                    Some(6u64)
                                } else {
                                    let a3005 = v1;
                                    let a3006 = 0u64;
                                    if nat_eq(a3005, a3006) {
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
    let a3007 = v0;
    let a3008 = 28u64;
    if if if if nat_eq(a3007, a3008) {
        let a3009 = v1;
        let a3010 = 0u64;
        nat_eq(a3009, a3010)
    } else {
        false
    } {
        let a3011 = v2;
        let a3012 = 0u64;
        nat_eq(a3011, a3012)
    } else {
        false
    } {
        let a3013 = v3;
        let a3014 = 0u64;
        nat_eq(a3013, a3014)
    } else {
        false
    } {
        let a3015 = v4;
        let a3016 = v8;
        if if if if if if if if nat_eq(a3015, a3016) {
            let a3017 = v5;
            let a3018 = v12;
            nat_eq(a3017, a3018)
        } else {
            false
        } {
            let a3019 = v9;
            let a3020 = v13;
            nat_eq(a3019, a3020)
        } else {
            false
        } {
            let a3021 = v10;
            let a3022 = v14;
            nat_eq(a3021, a3022)
        } else {
            false
        } {
            let a3023 = v11;
            let a3024 = v15;
            nat_eq(a3023, a3024)
        } else {
            false
        } {
            let a3025 = 0u64;
            let a3026 = v6;
            nat_lt(a3025, a3026)
        } else {
            false
        } {
            let a3027 = 0u64;
            let a3028 = v7;
            nat_lt(a3027, a3028)
        } else {
            false
        } {
            let a3032 = {
                let a3029 = v10;
                let a3030 = v6;
                let a3031 = 1u64;
                nat_rem(a3029, a3030, a3031)
            };
            let a3033 = 0u64;
            if nat_eq(a3032, a3033) {
                let a3037 = {
                    let a3034 = v11;
                    let a3035 = v7;
                    let a3036 = 1u64;
                    nat_rem(a3034, a3035, a3036)
                };
                let a3038 = 0u64;
                nat_eq(a3037, a3038)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3039 = v10;
                let a3040 = v6;
                let a3041 = 0u64;
                nat_quot(a3039, a3040, a3041)
            }, {
                let a3042 = v11;
                let a3043 = v7;
                let a3044 = 0u64;
                nat_quot(a3042, a3043, a3044)
            }, {
                let a3045 = v5;
                let a3046 = v4;
                nat_mul(a3045, a3046)?
            }, {
                let a3047 = v4;
                let a3048 = v9;
                nat_mul(a3047, a3048)?
            }, {
                let a3049 = v5;
                let a3050 = v9;
                nat_mul(a3049, a3050)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f156(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a3051 = v1;
    let a3052 = 0u64;
    if nat_eq(a3051, a3052) {
        Ok(Adt5::C4({
            let a3053 = v3;
            let a3054 = 0u64;
            if nat_eq(a3053, a3054) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f89(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a3055 = v3;
            let a3056 = 0u64;
            if nat_eq(a3055, a3056) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f157(v0: u64, v1: u64) -> R<u64> {
    let a3057 = v0;
    let a3058 = f60(v1)?;
    let a3059 = 0u64;
    Ok(nat_quot(a3057, a3058, a3059))
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f157(v0, v1)?;
    let v3: u64 = {
        let a3062 = {
            let a3060 = f142(v0, v1)?;
            let a3061 = 2u64;
            nat_sub(a3060, a3061)
        };
        let a3063 = 2u64;
        let a3064 = 0u64;
        nat_quot(a3062, a3063, a3064)
    };
    let v4: u64 = {
        let a3065 = v2;
        let a3066 = v1;
        nat_mul(a3065, a3066)?
    };
    if f145(v0, v1)? {
        let a3067 = v4;
        let a3068 = v3;
        nat_add(a3067, a3068)
    } else {
        let a3080 = {
            let a3076 = v4;
            let a3077 = {
                let a3074 = {
                    let a3069 = v1;
                    let a3070 = 1u64;
                    nat_sub(a3069, a3070)
                };
                let a3075 = {
                    let a3071 = v2;
                    let a3072 = v1;
                    let a3073 = 0u64;
                    nat_rem(a3071, a3072, a3073)
                };
                nat_mul(a3074, a3075)?
            };
            nat_sub(a3076, a3077)
        };
        let a3081 = {
            let a3078 = v3;
            let a3079 = v1;
            nat_mul(a3078, a3079)?
        };
        nat_add(a3080, a3081)
    }
}

pub fn f159(v0: u64) -> bool {
    let a3082 = v0;
    let a3083 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3082, a3083) {
        true
    } else {
        let a3084 = v0;
        let a3085 = 1u64;
        nat_eq(a3084, a3085)
    } {
        true
    } else {
        let a3086 = v0;
        let a3087 = 2u64;
        nat_eq(a3086, a3087)
    } {
        true
    } else {
        let a3088 = v0;
        let a3089 = 3u64;
        nat_eq(a3088, a3089)
    } {
        true
    } else {
        let a3090 = v0;
        let a3091 = 6u64;
        nat_eq(a3090, a3091)
    } {
        true
    } else {
        let a3092 = v0;
        let a3093 = 7u64;
        nat_eq(a3092, a3093)
    } {
        true
    } else {
        let a3094 = v0;
        let a3095 = 8u64;
        nat_eq(a3094, a3095)
    } {
        true
    } else {
        let a3096 = v0;
        let a3097 = 10u64;
        nat_eq(a3096, a3097)
    } {
        true
    } else {
        let a3098 = v0;
        let a3099 = 11u64;
        nat_eq(a3098, a3099)
    } {
        true
    } else {
        let a3100 = v0;
        let a3101 = 12u64;
        nat_eq(a3100, a3101)
    } {
        true
    } else {
        let a3102 = v0;
        let a3103 = 13u64;
        nat_eq(a3102, a3103)
    } {
        true
    } else {
        let a3104 = v0;
        let a3105 = 14u64;
        nat_eq(a3104, a3105)
    } {
        true
    } else {
        let a3106 = v0;
        let a3107 = 16u64;
        nat_eq(a3106, a3107)
    } {
        true
    } else {
        let a3108 = v0;
        let a3109 = 17u64;
        nat_eq(a3108, a3109)
    } {
        true
    } else {
        let a3110 = v0;
        let a3111 = 18u64;
        nat_eq(a3110, a3111)
    } {
        true
    } else {
        let a3112 = v0;
        let a3113 = 19u64;
        nat_eq(a3112, a3113)
    } {
        true
    } else {
        let a3114 = v0;
        let a3115 = 20u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        let a3116 = v0;
        let a3117 = 21u64;
        nat_eq(a3116, a3117)
    } {
        true
    } else {
        let a3118 = v0;
        let a3119 = 22u64;
        nat_eq(a3118, a3119)
    } {
        true
    } else {
        let a3120 = v0;
        let a3121 = 23u64;
        nat_eq(a3120, a3121)
    } {
        true
    } else {
        let a3122 = v0;
        let a3123 = 26u64;
        nat_eq(a3122, a3123)
    } {
        true
    } else {
        let a3124 = v0;
        let a3125 = 29u64;
        nat_eq(a3124, a3125)
    } {
        true
    } else {
        let a3126 = v0;
        let a3127 = 30u64;
        nat_eq(a3126, a3127)
    } {
        true
    } else {
        let a3128 = v0;
        let a3129 = 34u64;
        nat_eq(a3128, a3129)
    } {
        true
    } else {
        let a3130 = v0;
        let a3131 = 35u64;
        nat_eq(a3130, a3131)
    } {
        true
    } else {
        let a3132 = v0;
        let a3133 = 39u64;
        nat_eq(a3132, a3133)
    } {
        true
    } else {
        let a3134 = v0;
        let a3135 = 40u64;
        nat_eq(a3134, a3135)
    } {
        true
    } else {
        let a3136 = v0;
        let a3137 = 41u64;
        nat_eq(a3136, a3137)
    } {
        true
    } else {
        let a3138 = v0;
        let a3139 = 42u64;
        nat_eq(a3138, a3139)
    } {
        true
    } else {
        let a3140 = v0;
        let a3141 = 43u64;
        nat_eq(a3140, a3141)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3142 = v0;
    let a3143 = 0u64;
    if if if nat_eq(a3142, a3143) {
        true
    } else {
        let a3144 = v1;
        let a3145 = 0u64;
        nat_eq(a3144, a3145)
    } {
        true
    } else {
        let a3146 = v2;
        let a3147 = 0u64;
        nat_eq(a3146, a3147)
    } {
        Ok(0u64)
    } else {
        let a3150 = {
            let a3148 = v0;
            let a3149 = v1;
            nat_mul(a3148, a3149)?
        };
        let a3151 = v2;
        nat_mul(a3150, a3151)
    }
}

pub fn f161(v0: u64) -> bool {
    let m3152 = f70(v0);
    match m3152 {
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
