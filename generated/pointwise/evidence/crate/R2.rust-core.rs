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
    C1(u64, Adt3, Adt3, Adt4),
    C2(u64, Adt3, Adt3, u64),
    C3(u64, Adt3, Adt3, Adt3, u64),
    C4(u64, u64, Adt3, Adt3, Adt3, u64),
    C5(u64, Adt3, Adt3, Adt3, Adt3, u64),
    C6(u64, u64, u64, u64, u64, u64),
    C7(u64, Adt3, Adt3, Adt4, u64),
    C8(Adt4),
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
                            let a8 = 24u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt5::C0(f3(v2, v14, v13)?))
                            } else {
                                let a9 = v1;
                                let a10 = 27u64;
                                if nat_eq(a9, a10) {
                                    Ok(Adt5::C0(f4(v2, v14, v13)?))
                                } else {
                                    let a11 = v1;
                                    let a12 = 3u64;
                                    if nat_eq(a11, a12) {
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
        Adt0::C1(v15, v16, v17, v18) => {
            f7(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f8(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f9(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f10(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f11(v34, v35, v36, v37, v38, v39)
        }
        Adt0::C6(v40, v41, v42, v43, v44, v45) => {
            f12(v40, v41, v42, v43, v44, v45)
        }
        Adt0::C7(v46, v47, v48, v49, v50) => {
            f13(v46, v47, v48, v49, v50)
        }
        Adt0::C8(v51) => {
            f14(v51)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f15(v1.clone(), 1u64)?;
    let a14 = match v1.clone() {
        Adt2::C0(h13, _, _, _, _, _, _, _, _, _, _) => {
            h13
        }
    };
    let a15 = 0u64;
    if if if if if if if if nat_eq(a14, a15) {
        let a17 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h16, _, _, _, _, _) => {
                h16
            }
        };
        let a18 = 4u64;
        nat_eq(a17, a18)
    } else {
        false
    } {
        f16(match v0.clone() {
            Adt1::C0(h19, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h19
            }
        })
    } else {
        false
    } {
        f16(match v0.clone() {
            Adt1::C0(_, h20, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h20
            }
        })
    } else {
        false
    } {
        let a22 = 0u64;
        let a23 = match v0.clone() {
            Adt1::C0(_, _, h21, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h21
            }
        };
        nat_lt(a22, a23)
    } else {
        false
    } {
        let a25 = match v0.clone() {
            Adt1::C0(_, _, h24, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h24
            }
        };
        let a26 = 2139095040u64;
        nat_lt(a25, a26)
    } else {
        false
    } {
        let a27 = v2;
        let a28 = 9223372036854775808u64;
        nat_lt(a27, a28)
    } else {
        false
    } {
        let a30 = f17(v2)?;
        let a31 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h29, _) => {
                h29
            }
        };
        nat_le(a30, a31)
    } else {
        false
    } {
        Ok(Adt6::C4(f18(v2)?, 4u64, f19(v1.clone(), 1u64), f19(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h32, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h32
            }
        }, match v0.clone() {
            Adt1::C0(_, h33, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h33
            }
        }, match v0 {
            Adt1::C0(_, _, h34, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h34
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m35 = v2;
    match m35 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let m36 = v3;
            match m36 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v6) => {
                    let a38 = match v1.clone() {
                        Adt2::C0(h37, _, _, _, _, _, _, _, _, _, _) => {
                            h37
                        }
                    };
                    let a39 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a38, a39) {
                        let a41 = match v5.clone() {
                            Adt2::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                                h40
                            }
                        };
                        let a42 = 0u64;
                        nat_eq(a41, a42)
                    } else {
                        false
                    } {
                        let a44 = match v6.clone() {
                            Adt2::C0(h43, _, _, _, _, _, _, _, _, _, _) => {
                                h43
                            }
                        };
                        let a45 = 0u64;
                        nat_eq(a44, a45)
                    } else {
                        false
                    } {
                        let a47 = match v4.clone() {
                            Adt2::C0(h46, _, _, _, _, _, _, _, _, _, _) => {
                                h46
                            }
                        };
                        let a48 = 0u64;
                        nat_eq(a47, a48)
                    } else {
                        false
                    } {
                        f20(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f20(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f21(v1.clone())?
                    } else {
                        false
                    } {
                        f21(v5.clone())?
                    } else {
                        false
                    } {
                        f21(v6)?
                    } else {
                        false
                    } {
                        f21(v4.clone())?
                    } else {
                        false
                    } {
                        let a54 = match v1.clone() {
                            Adt2::C0(_, h49, _, _, _, _, _, _, _, _, _) => {
                                h49
                            }
                        };
                        let a55 = {
                            let a52 = match v5.clone() {
                                Adt2::C0(_, h50, _, _, _, _, _, _, _, _, _) => {
                                    h50
                                }
                            };
                            let a53 = match v5.clone() {
                                Adt2::C0(_, h51, _, _, _, _, _, _, _, _, _) => {
                                    h51
                                }
                            };
                            nat_mul(a52, a53)?
                        };
                        nat_eq(a54, a55)
                    } else {
                        false
                    } {
                        let a61 = match v1.clone() {
                            Adt2::C0(_, _, h56, _, _, _, _, _, _, _, _) => {
                                h56
                            }
                        };
                        let a62 = {
                            let a59 = match v5.clone() {
                                Adt2::C0(_, _, h57, _, _, _, _, _, _, _, _) => {
                                    h57
                                }
                            };
                            let a60 = match v5.clone() {
                                Adt2::C0(_, _, _, h58, _, _, _, _, _, _, _) => {
                                    h58
                                }
                            };
                            nat_mul(a59, a60)?
                        };
                        nat_eq(a61, a62)
                    } else {
                        false
                    } {
                        let a65 = match v1.clone() {
                            Adt2::C0(_, _, _, h63, _, _, _, _, _, _, _) => {
                                h63
                            }
                        };
                        let a66 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h64, _, _, _, _, _, _) => {
                                h64
                            }
                        };
                        nat_eq(a65, a66)
                    } else {
                        false
                    } {
                        let a68 = match v0.clone() {
                            Adt1::C0(h67, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h67
                            }
                        };
                        let a69 = 4294967296u64;
                        nat_lt(a68, a69)
                    } else {
                        false
                    } {
                        let a71 = match v0.clone() {
                            Adt1::C0(h70, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h70
                            }
                        };
                        let a72 = 0u64;
                        if nat_eq(a71, a72) {
                            if f22(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a75 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h73) => {
                                        h73
                                    }
                                };
                                let a76 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h74) => {
                                        h74
                                    }
                                };
                                nat_eq(a75, a76)
                            }
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
                        false
                    } {
                        let v7: bool = {
                            let a82 = match v4.clone() {
                                Adt2::C0(_, h81, _, _, _, _, _, _, _, _, _) => {
                                    h81
                                }
                            };
                            let a83 = 0u64;
                            if if if nat_eq(a82, a83) {
                                true
                            } else {
                                let a85 = match v4.clone() {
                                    Adt2::C0(_, _, h84, _, _, _, _, _, _, _, _) => {
                                        h84
                                    }
                                };
                                let a86 = 0u64;
                                nat_eq(a85, a86)
                            } {
                                true
                            } else {
                                let a88 = match v4.clone() {
                                    Adt2::C0(_, _, _, h87, _, _, _, _, _, _, _) => {
                                        h87
                                    }
                                };
                                let a89 = 0u64;
                                nat_eq(a88, a89)
                            } {
                                true
                            } else {
                                let a91 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h90, _, _, _, _, _, _) => {
                                        h90
                                    }
                                };
                                let a92 = 0u64;
                                nat_eq(a91, a92)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                                h93
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a95 = match v0 {
                                Adt1::C0(h94, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h94
                                }
                            };
                            let a96 = 0u64;
                            if nat_eq(a95, a96) {
                                f15(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f23(v8, match v5.clone() {
                                Adt2::C0(_, _, h97, _, _, _, _, _, _, _, _) => {
                                    h97
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h98, _, _, _, _, _, _, _) => {
                                    h98
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h99, _, _, _, _, _, _) => {
                                    h99
                                }
                            })?
                        };
                        Ok(Adt6::C4(f24(v9, v8, v10)?, 4u64, f19(v1, 1u64), f19(v4, 1u64), Adt4::C25(v9, v8, v10)))
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
        Adt2::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
            h100
        }
    };
    let v4: u64 = f25(v3);
    let a102 = match v1.clone() {
        Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
            h101
        }
    };
    let a103 = 0u64;
    if if if if if if if if nat_eq(a102, a103) {
        true
    } else {
        let a105 = match v1.clone() {
            Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                h104
            }
        };
        let a106 = 30u64;
        nat_eq(a105, a106)
    } {
        let a109 = match v1.clone() {
            Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                h107
            }
        };
        let a110 = match v2.clone() {
            Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                h108
            }
        };
        nat_eq(a109, a110)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f26(match v0.clone() {
            Adt1::C0(h111, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h111
            }
        })
    } else {
        false
    } {
        f27(v1.clone(), v4)
    } else {
        false
    } {
        f27(v2.clone(), v4)
    } else {
        false
    } {
        if f22(v1.clone(), v2.clone())? {
            true
        } else {
            f28(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f29(match v1.clone() {
            Adt2::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
                h112
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                h113
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h114, _, _, _, _, _, _, _) => {
                h114
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h115, _, _, _, _, _, _) => {
                h115
            }
        }) {
            0u64
        } else {
            let a121 = match v1.clone() {
                Adt2::C0(_, _, h116, _, _, _, _, _, _, _, _) => {
                    h116
                }
            };
            let a122 = {
                let a119 = match v1.clone() {
                    Adt2::C0(_, _, _, h117, _, _, _, _, _, _, _) => {
                        h117
                    }
                };
                let a120 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h118, _, _, _, _, _, _) => {
                        h118
                    }
                };
                nat_mul(a119, a120)?
            };
            nat_mul(a121, a122)?
        };
        let v6: u64 = {
            let a123 = v5;
            let a124 = 0u64;
            if if nat_eq(a123, a124) {
                true
            } else {
                f30(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a126 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h125, _) => {
                        h125
                    }
                };
                let a127 = v4;
                let a128 = 0u64;
                nat_quot(a126, a127, a128)
            }
        };
        Ok(Adt6::C4(f31(v3, match v1.clone() {
            Adt2::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
                h129
            }
        }, v5, v6)?, v4, f19(v1.clone(), 1u64), f19(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h130, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h130
            }
        }, v5, f32(match v1 {
            Adt2::C0(_, h131, _, _, _, _, _, _, _, _, _) => {
                h131
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a133 = match v1.clone() {
        Adt2::C0(h132, _, _, _, _, _, _, _, _, _, _) => {
            h132
        }
    };
    let a134 = 0u64;
    if if if if if nat_eq(a133, a134) {
        let a136 = match v2.clone() {
            Adt2::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        };
        let a137 = 0u64;
        nat_eq(a136, a137)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f33(match v0.clone() {
            Adt1::C0(h138, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        })
    } else {
        false
    } {
        if f22(v1.clone(), v2.clone())? {
            true
        } else {
            f28(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f19(v1, 1u64);
        let v4: u64 = {
            let a144 = match v3.clone() {
                Adt3::C0(_, h139, _, _, _, _, _, _) => {
                    h139
                }
            };
            let a145 = {
                let a142 = match v3.clone() {
                    Adt3::C0(_, _, h140, _, _, _, _, _) => {
                        h140
                    }
                };
                let a143 = match v3.clone() {
                    Adt3::C0(_, _, _, h141, _, _, _, _) => {
                        h141
                    }
                };
                nat_mul(a142, a143)?
            };
            nat_mul(a144, a145)?
        };
        Ok(Adt6::C4(f34(match v3.clone() {
            Adt3::C0(h146, _, _, _, _, _, _, _) => {
                h146
            }
        }, v4)?, 4u64, v3, f19(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h147, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h147
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f5(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m148 = v1;
    match m148 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m149 = v2;
            match m149 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a151 = match v0.clone() {
                        Adt2::C0(h150, _, _, _, _, _, _, _, _, _, _) => {
                            h150
                        }
                    };
                    let a152 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a151, a152) {
                        let a154 = match v4.clone() {
                            Adt2::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
                                h153
                            }
                        };
                        let a155 = 0u64;
                        nat_eq(a154, a155)
                    } else {
                        false
                    } {
                        let a157 = match v5.clone() {
                            Adt2::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                                h156
                            }
                        };
                        let a158 = 26u64;
                        nat_eq(a157, a158)
                    } else {
                        false
                    } {
                        let a160 = match v3.clone() {
                            Adt2::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                                h159
                            }
                        };
                        let a161 = 0u64;
                        nat_eq(a160, a161)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a164 = match v0.clone() {
                            Adt2::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
                                h162
                            }
                        };
                        let a165 = match v4.clone() {
                            Adt2::C0(_, h163, _, _, _, _, _, _, _, _, _) => {
                                h163
                            }
                        };
                        nat_eq(a164, a165)
                    } else {
                        false
                    } {
                        let a168 = match v0.clone() {
                            Adt2::C0(_, _, h166, _, _, _, _, _, _, _, _) => {
                                h166
                            }
                        };
                        let a169 = match v5.clone() {
                            Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                                h167
                            }
                        };
                        nat_eq(a168, a169)
                    } else {
                        false
                    } {
                        let a172 = match v0.clone() {
                            Adt2::C0(_, _, _, h170, _, _, _, _, _, _, _) => {
                                h170
                            }
                        };
                        let a173 = match v5.clone() {
                            Adt2::C0(_, _, h171, _, _, _, _, _, _, _, _) => {
                                h171
                            }
                        };
                        nat_eq(a172, a173)
                    } else {
                        false
                    } {
                        let a175 = 0u64;
                        let a176 = match v5.clone() {
                            Adt2::C0(_, _, _, h174, _, _, _, _, _, _, _) => {
                                h174
                            }
                        };
                        nat_lt(a175, a176)
                    } else {
                        false
                    } {
                        let a178 = 0u64;
                        let a179 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h177, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        nat_lt(a178, a179)
                    } else {
                        false
                    } {
                        let a181 = 4u64;
                        let a182 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h180, _, _, _, _, _) => {
                                h180
                            }
                        };
                        nat_le(a181, a182)
                    } else {
                        false
                    } {
                        let a184 = 4u64;
                        let a185 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h183, _, _, _, _, _) => {
                                h183
                            }
                        };
                        nat_le(a184, a185)
                    } else {
                        false
                    } {
                        let a187 = 4u64;
                        let a188 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h186, _, _, _, _, _) => {
                                h186
                            }
                        };
                        nat_le(a187, a188)
                    } else {
                        false
                    } {
                        let a190 = 4u64;
                        let a191 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                                h189
                            }
                        };
                        nat_le(a190, a191)
                    } else {
                        false
                    } {
                        f22(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f22(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f22(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f35(match v0.clone() {
                            Adt2::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                                h192
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h193, _, _, _, _, _, _, _, _) => {
                                h193
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h194, _, _, _, _, _, _, _) => {
                                h194
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h195, _, _, _, _, _, _) => {
                                h195
                            }
                        })?, 4u64, f19(v0, 1u64), f19(v3, 1u64), Adt4::C22(f19(v4, 1u64), f19(v5, 1u64))))
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
        Adt1::C0(h196, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h196
        }
    };
    Ok(Adt5::C0({
        let a197 = v0;
        let a198 = 39u64;
        if nat_eq(a197, a198) {
            f36(v2, v3, v4)?
        } else {
            let a199 = v0;
            let a200 = 77u64;
            if nat_eq(a199, a200) {
                f37(v2, v4)?
            } else {
                let a201 = v0;
                let a202 = 40u64;
                if nat_eq(a201, a202) {
                    f38(v2, v3, v4)?
                } else {
                    if f39(v0) {
                        f40(v0, v2, {
                            let m203 = v3;
                            match m203 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a204 = v0;
                        let a205 = 19u64;
                        if nat_eq(a204, a205) {
                            f41(v2, v4)?
                        } else {
                            let a206 = v0;
                            let a207 = 21u64;
                            if nat_eq(a206, a207) {
                                f42(v5, v2, v3, v4)?
                            } else {
                                let a208 = v0;
                                let a209 = 61u64;
                                if if if nat_eq(a208, a209) {
                                    true
                                } else {
                                    let a210 = v0;
                                    let a211 = 62u64;
                                    nat_eq(a210, a211)
                                } {
                                    true
                                } else {
                                    let a212 = v0;
                                    let a213 = 63u64;
                                    nat_eq(a212, a213)
                                } {
                                    f43(v0, v1, v2, v4)?
                                } else {
                                    let a214 = v0;
                                    let a215 = 4u64;
                                    if if nat_eq(a214, a215) {
                                        true
                                    } else {
                                        let a216 = v0;
                                        let a217 = 32u64;
                                        nat_eq(a216, a217)
                                    } {
                                        f44(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a218 = v0;
                                        let a219 = 20u64;
                                        if nat_eq(a218, a219) {
                                            f45(v2, v4)?
                                        } else {
                                            let a220 = v0;
                                            let a221 = 42u64;
                                            if nat_eq(a220, a221) {
                                                f46(4u64, v2, v4)?
                                            } else {
                                                let a222 = v0;
                                                let a223 = 69u64;
                                                if nat_eq(a222, a223) {
                                                    let a224 = v5;
                                                    let a225 = 4u64;
                                                    if nat_lt(a224, a225) {
                                                        f46(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a226 = v0;
                                                    let a227 = 70u64;
                                                    if nat_eq(a226, a227) {
                                                        f47(v5, v2, v4)?
                                                    } else {
                                                        let a228 = v0;
                                                        let a229 = 75u64;
                                                        if if nat_eq(a228, a229) {
                                                            true
                                                        } else {
                                                            let a230 = v0;
                                                            let a231 = 76u64;
                                                            nat_eq(a230, a231)
                                                        } {
                                                            f48({
                                                                let a232 = v0;
                                                                let a233 = 76u64;
                                                                nat_eq(a232, a233)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a234 = v0;
                                                            let a235 = 31u64;
                                                            if if if nat_eq(a234, a235) {
                                                                true
                                                            } else {
                                                                let a236 = v0;
                                                                let a237 = 47u64;
                                                                nat_eq(a236, a237)
                                                            } {
                                                                true
                                                            } else {
                                                                let a238 = v0;
                                                                let a239 = 68u64;
                                                                nat_eq(a238, a239)
                                                            } {
                                                                f49(v0, v1, v2, v4)?
                                                            } else {
                                                                let a240 = v0;
                                                                let a241 = 15u64;
                                                                if nat_eq(a240, a241) {
                                                                    f50(v2, v4)?
                                                                } else {
                                                                    let a242 = v0;
                                                                    let a243 = 17u64;
                                                                    if nat_eq(a242, a243) {
                                                                        f51(v2, v4)?
                                                                    } else {
                                                                        let a244 = v0;
                                                                        let a245 = 18u64;
                                                                        if nat_eq(a244, a245) {
                                                                            f52(v2, v3, v4)?
                                                                        } else {
                                                                            let a246 = v0;
                                                                            let a247 = 13u64;
                                                                            if if if nat_eq(a246, a247) {
                                                                                true
                                                                            } else {
                                                                                let a248 = v0;
                                                                                let a249 = 14u64;
                                                                                nat_eq(a248, a249)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a250 = v0;
                                                                                let a251 = 16u64;
                                                                                nat_eq(a250, a251)
                                                                            } {
                                                                                f53(v0, v2, v4)?
                                                                            } else {
                                                                                let m252 = f54(v0, v5);
                                                                                match m252 {
                                                                                    None => {
                                                                                        f55(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f56(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f7(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m253 = v3;
    match m253 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f57(v0, v1)?, f57(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f59(v0, v1, v2.clone())?, f57(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f60(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f61(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f62(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f61(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f63(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f64(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f65(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f57(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f66(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f57(v0, v1)?, f57(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f67(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f57(v0, v1)?, f57(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f57(v0, v1)?, f57(v0, v2)?))
        }
        Adt4::C15 => {
            f68(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f69(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f70(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f57(f58(v0, match v2.clone() {
                Adt3::C0(h254, _, _, _, _, _, _, _) => {
                    h254
                }
            }, match v2.clone() {
                Adt3::C0(_, h255, _, _, _, _, _, _) => {
                    h255
                }
            })?, v1)?, f57(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C24(_) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C25(v48, v49, v50) => {
            f71(v0, v48, v49, v50)
        }
        Adt4::C26(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f8(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f72(match v1.clone() {
        Adt3::C0(h256, _, _, _, _, _, _, _) => {
            h256
        }
    })?;
    let v5: u64 = {
        let a257 = v0;
        let a258 = v4;
        let a259 = 0u64;
        nat_quot(a257, a258, a259)
    };
    let v6: u64 = {
        let a260 = v0;
        let a261 = v4;
        let a262 = 0u64;
        nat_rem(a260, a261, a262)
    };
    let a263 = v6;
    let a264 = 0u64;
    if nat_eq(a263, a264) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a265 = v6;
        let a266 = 1u64;
        if nat_eq(a265, a266) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a269 = {
                let a267 = v6;
                let a268 = 1u64;
                nat_add(a267, a268)?
            };
            let a270 = v4;
            if nat_eq(a269, a270) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f57(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a271 = v6;
                    let a272 = 2u64;
                    nat_sub(a271, a272)
                };
                let v8: u64 = {
                    let a273 = v7;
                    let a274 = 3u64;
                    let a275 = 0u64;
                    nat_quot(a273, a274, a275)
                };
                let v9: u64 = {
                    let a276 = v7;
                    let a277 = 3u64;
                    let a278 = 0u64;
                    nat_rem(a276, a277, a278)
                };
                let v10: u64 = f57({
                    let a282 = {
                        let a280 = v5;
                        let a281 = match v1.clone() {
                            Adt3::C0(h279, _, _, _, _, _, _, _) => {
                                h279
                            }
                        };
                        nat_mul(a280, a281)?
                    };
                    let a283 = v8;
                    nat_add(a282, a283)?
                }, v1)?;
                let a284 = v9;
                let a285 = 0u64;
                if nat_eq(a284, a285) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a286 = v9;
                    let a287 = 1u64;
                    if nat_eq(a286, a287) {
                        let a288 = v3;
                        let a289 = 2u64;
                        if nat_eq(a288, a289) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a290 = v3;
                        let a291 = 2u64;
                        if if nat_eq(a290, a291) {
                            true
                        } else {
                            let a292 = v3;
                            let a293 = 3u64;
                            nat_eq(a292, a293)
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

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a295 = 2u64;
        let a296 = match v1.clone() {
            Adt3::C0(_, h294, _, _, _, _, _, _) => {
                h294
            }
        };
        nat_mul(a295, a296)?
    };
    let a297 = v0;
    let a298 = v5;
    if nat_lt(a297, a298) {
        let a302 = {
            let a299 = v0;
            let a300 = 2u64;
            let a301 = 0u64;
            nat_rem(a299, a300, a301)
        };
        let a303 = 0u64;
        if nat_eq(a302, a303) {
            Ok(Adt5::C4(19u64, 4u64, f57({
                let a304 = v0;
                let a305 = 2u64;
                let a306 = 0u64;
                nat_quot(a304, a305, a306)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a307 = v4;
            let a308 = 2147483648u64;
            if if nat_lt(a307, a308) {
                let a310 = v4;
                let a311 = match v2 {
                    Adt3::C0(_, h309, _, _, _, _, _, _) => {
                        h309
                    }
                };
                nat_lt(a310, a311)
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
            let a314 = match v2.clone() {
                Adt3::C0(h312, _, _, _, _, _, _, _) => {
                    h312
                }
            };
            let a315 = match v2.clone() {
                Adt3::C0(_, h313, _, _, _, _, _, _) => {
                    h313
                }
            };
            nat_mul(a314, a315)?
        };
        let a318 = {
            let a316 = v0;
            let a317 = v5;
            nat_sub(a316, a317)
        };
        let a319 = v6;
        if nat_lt(a318, a319) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f57({
                let a320 = v0;
                let a321 = v5;
                nat_sub(a320, a321)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a324 = {
                    let a322 = v0;
                    let a323 = v5;
                    nat_sub(a322, a323)
                };
                let a325 = v6;
                nat_sub(a324, a325)
            };
            let v8: u64 = {
                let a329 = 1u64;
                let a330 = {
                    let a327 = 2u64;
                    let a328 = match v1.clone() {
                        Adt3::C0(h326, _, _, _, _, _, _, _) => {
                            h326
                        }
                    };
                    nat_mul(a327, a328)?
                };
                nat_add(a329, a330)?
            };
            let v9: u64 = {
                let a331 = v7;
                let a332 = v8;
                let a333 = 0u64;
                nat_quot(a331, a332, a333)
            };
            let v10: u64 = {
                let a334 = v7;
                let a335 = v8;
                let a336 = 0u64;
                nat_rem(a334, a335, a336)
            };
            let a337 = v10;
            let a338 = 0u64;
            if nat_eq(a337, a338) {
                Ok(Adt5::C4(19u64, 4u64, f57(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a342 = {
                    let a339 = v10;
                    let a340 = 2u64;
                    let a341 = 0u64;
                    nat_rem(a339, a340, a341)
                };
                let a343 = 1u64;
                if nat_eq(a342, a343) {
                    Ok(Adt5::C4(6u64, 0u64, f57({
                        let a352 = {
                            let a345 = v9;
                            let a346 = match v1.clone() {
                                Adt3::C0(h344, _, _, _, _, _, _, _) => {
                                    h344
                                }
                            };
                            nat_mul(a345, a346)?
                        };
                        let a353 = {
                            let a349 = {
                                let a347 = v10;
                                let a348 = 1u64;
                                nat_sub(a347, a348)
                            };
                            let a350 = 2u64;
                            let a351 = 0u64;
                            nat_quot(a349, a350, a351)
                        };
                        nat_add(a352, a353)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f57({
                        let a362 = {
                            let a355 = v4;
                            let a356 = match v1.clone() {
                                Adt3::C0(h354, _, _, _, _, _, _, _) => {
                                    h354
                                }
                            };
                            nat_mul(a355, a356)?
                        };
                        let a363 = {
                            let a359 = {
                                let a357 = v10;
                                let a358 = 1u64;
                                nat_sub(a357, a358)
                            };
                            let a360 = 2u64;
                            let a361 = 0u64;
                            nat_quot(a359, a360, a361)
                        };
                        nat_add(a362, a363)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f57({
                        let a372 = {
                            let a365 = v4;
                            let a366 = match v1 {
                                Adt3::C0(h364, _, _, _, _, _, _, _) => {
                                    h364
                                }
                            };
                            nat_mul(a365, a366)?
                        };
                        let a373 = {
                            let a369 = {
                                let a367 = v10;
                                let a368 = 1u64;
                                nat_sub(a367, a368)
                            };
                            let a370 = 2u64;
                            let a371 = 0u64;
                            nat_quot(a369, a370, a371)
                        };
                        nat_add(a372, a373)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a374 = v1;
    let a375 = f73(v2.clone(), v4.clone())?;
    if nat_lt(a374, a375) {
        let a378 = v1;
        let a379 = {
            let a376 = 2u64;
            let a377 = f74(v4.clone())?;
            nat_mul(a376, a377)?
        };
        if nat_lt(a378, a379) {
            let a383 = {
                let a380 = v1;
                let a381 = 2u64;
                let a382 = 0u64;
                nat_rem(a380, a381, a382)
            };
            let a384 = 0u64;
            if nat_eq(a383, a384) {
                f75({
                    let a385 = v1;
                    let a386 = 2u64;
                    let a387 = 0u64;
                    nat_quot(a385, a386, a387)
                }, v4)
            } else {
                if f76(match v2 {
                    Adt3::C0(_, h388, _, _, _, _, _, _) => {
                        h388
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a391 = v1;
                let a392 = {
                    let a389 = 2u64;
                    let a390 = f74(v4.clone())?;
                    nat_mul(a389, a390)?
                };
                nat_sub(a391, a392)
            };
            let v7: u64 = {
                let a396 = v6;
                let a397 = {
                    let a394 = 1u64;
                    let a395 = match v2.clone() {
                        Adt3::C0(h393, _, _, _, _, _, _, _) => {
                            h393
                        }
                    };
                    nat_add(a394, a395)?
                };
                let a398 = 0u64;
                nat_quot(a396, a397, a398)
            };
            let v8: u64 = {
                let a402 = v6;
                let a403 = {
                    let a400 = 1u64;
                    let a401 = match v2.clone() {
                        Adt3::C0(h399, _, _, _, _, _, _, _) => {
                            h399
                        }
                    };
                    nat_add(a400, a401)?
                };
                let a404 = 0u64;
                nat_rem(a402, a403, a404)
            };
            let a405 = v8;
            let a406 = 0u64;
            if nat_eq(a405, a406) {
                f75(v7, v4)
            } else {
                f77(v0, v7, {
                    let a407 = v8;
                    let a408 = 1u64;
                    nat_sub(a407, a408)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f11(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a411 = match v4.clone() {
            Adt3::C0(h409, _, _, _, _, _, _, _) => {
                h409
            }
        };
        let a412 = match v4.clone() {
            Adt3::C0(_, h410, _, _, _, _, _, _) => {
                h410
            }
        };
        nat_mul(a411, a412)?
    };
    let v7: u64 = {
        let a413 = 2u64;
        let a414 = v6;
        nat_mul(a413, a414)?
    };
    let v8: u64 = f78(match v3.clone() {
        Adt3::C0(_, h415, _, _, _, _, _, _) => {
            h415
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h416, _, _, _, _, _) => {
            h416
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h417, _, _, _, _) => {
            h417
        }
    });
    let a418 = v0;
    let a419 = v7;
    if nat_lt(a418, a419) {
        let a423 = {
            let a420 = v0;
            let a421 = 2u64;
            let a422 = 0u64;
            nat_rem(a420, a421, a422)
        };
        let a424 = 0u64;
        if nat_eq(a423, a424) {
            Ok(Adt5::C4(19u64, 5u64, f57({
                let a425 = v0;
                let a426 = 2u64;
                let a427 = 0u64;
                nat_quot(a425, a426, a427)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f79(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a428 = v0;
            let a429 = v7;
            nat_sub(a428, a429)
        };
        let v10: u64 = {
            let a433 = v9;
            let a434 = {
                let a431 = 1u64;
                let a432 = match v1.clone() {
                    Adt3::C0(h430, _, _, _, _, _, _, _) => {
                        h430
                    }
                };
                nat_add(a431, a432)?
            };
            let a435 = 0u64;
            nat_quot(a433, a434, a435)
        };
        let v11: u64 = {
            let a439 = v9;
            let a440 = {
                let a437 = 1u64;
                let a438 = match v1.clone() {
                    Adt3::C0(h436, _, _, _, _, _, _, _) => {
                        h436
                    }
                };
                nat_add(a437, a438)?
            };
            let a441 = 0u64;
            nat_rem(a439, a440, a441)
        };
        let a442 = v11;
        let a443 = 0u64;
        if nat_eq(a442, a443) {
            Ok(Adt5::C4(19u64, 5u64, f57({
                let a444 = v10;
                let a445 = v6;
                let a446 = 0u64;
                nat_rem(a444, a445, a446)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f79(v5, v8) {
                let v12: u64 = {
                    let a452 = {
                        let a448 = v10;
                        let a449 = match v1.clone() {
                            Adt3::C0(h447, _, _, _, _, _, _, _) => {
                                h447
                            }
                        };
                        nat_mul(a448, a449)?
                    };
                    let a453 = {
                        let a450 = v11;
                        let a451 = 1u64;
                        nat_sub(a450, a451)
                    };
                    nat_add(a452, a453)?
                };
                let a454 = v5;
                let a455 = 4294967295u64;
                if nat_eq(a454, a455) {
                    Ok(Adt5::C1(false, f57(v12, v1)?, f57(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f57(v12, v1)?, 4u64, f57({
                        let a461 = {
                            let a457 = v5;
                            let a458 = match v3.clone() {
                                Adt3::C0(h456, _, _, _, _, _, _, _) => {
                                    h456
                                }
                            };
                            nat_mul(a457, a458)?
                        };
                        let a462 = {
                            let a459 = v11;
                            let a460 = 1u64;
                            nat_sub(a459, a460)
                        };
                        nat_add(a461, a462)?
                    }, v3)?, 3u64, f57(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f12(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a463 = v0;
    let a464 = f18(v1)?;
    if nat_lt(a463, a464) {
        let a465 = v0;
        let a466 = 4u64;
        if nat_lt(a465, a466) {
            let a467 = v0;
            let a468 = 0u64;
            if nat_eq(a467, a468) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a469 = v0;
                let a470 = 1u64;
                if nat_eq(a469, a470) {
                    let a471 = v5;
                    let a472 = 0u64;
                    if nat_eq(a471, a472) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a473 = v0;
                    let a474 = 2u64;
                    if nat_eq(a473, a474) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a475 = v0;
            let a476 = 4u64;
            if if nat_eq(a475, a476) {
                let a477 = f80(v5)?;
                let a478 = v1;
                !nat_eq(a477, a478)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a481 = {
                        let a479 = v0;
                        let a480 = 4u64;
                        nat_sub(a479, a480)
                    };
                    let a482 = 3u64;
                    let a483 = 0u64;
                    nat_quot(a481, a482, a483)
                };
                let v7: u64 = {
                    let a486 = {
                        let a484 = v0;
                        let a485 = 4u64;
                        nat_sub(a484, a485)
                    };
                    let a487 = 3u64;
                    let a488 = 0u64;
                    nat_rem(a486, a487, a488)
                };
                let a489 = v7;
                let a490 = 0u64;
                if nat_eq(a489, a490) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a491 = v7;
                    let a492 = 1u64;
                    if nat_eq(a491, a492) {
                        Ok(Adt5::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt5::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f17(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f13(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt5> {
    Ok(f81(v3.clone(), {
        let m493 = v3.clone();
        match m493 {
            Adt4::C0 => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f8(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f9(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f10(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f11(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v57, v58, v59, v60) => {
                f12(v0, v60, v57, v58, v59, v4)?
            }
            Adt4::C24(v56) => {
                f83(v0, v1, v2, v56, v4)?
            }
            Adt4::C25(_, _, _) => {
                f7(v0, v1, v2, v3)?
            }
            Adt4::C26(v52, v53, v54, v55) => {
                f82(v0, v1, v2, v52, v53, v54, v55, v4)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt5> {
    let m494 = v0;
    match m494 {
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
                let a499 = {
                    let a495 = 0u64;
                    let a496 = v45;
                    if nat_lt(a495, a496) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a500 = {
                    let a497 = 0u64;
                    let a498 = v47;
                    if nat_lt(a497, a498) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a499, a500)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(_, _, _, _) => {
            Ok(Adt5::C6(1u64, 0u64, 0u64, false))
        }
    }
}

pub fn f15(v0: Adt2, v1: u64) -> R<u64> {
    let a512 = {
        let a506 = {
            let a502 = match v0.clone() {
                Adt2::C0(_, h501, _, _, _, _, _, _, _, _, _) => {
                    h501
                }
            };
            let a503 = v1;
            let a504 = 0u64;
            nat_quot(a502, a503, a504)
        };
        let a507 = match v0.clone() {
            Adt2::C0(_, _, h505, _, _, _, _, _, _, _, _) => {
                h505
            }
        };
        nat_mul(a506, a507)?
    };
    let a513 = {
        let a510 = match v0.clone() {
            Adt2::C0(_, _, _, h508, _, _, _, _, _, _, _) => {
                h508
            }
        };
        let a511 = match v0 {
            Adt2::C0(_, _, _, _, h509, _, _, _, _, _, _) => {
                h509
            }
        };
        nat_mul(a510, a511)?
    };
    nat_mul(a512, a513)
}

pub fn f16(v0: u64) -> bool {
    let a514 = v0;
    let a515 = 4294967296u64;
    if nat_lt(a514, a515) {
        let a519 = {
            let a516 = v0;
            let a517 = 2147483648u64;
            let a518 = 0u64;
            nat_rem(a516, a517, a518)
        };
        let a520 = 2139095040u64;
        nat_lt(a519, a520)
    } else {
        false
    }
}

pub fn f17(v0: u64) -> R<u64> {
    let a521 = 4u64;
    let a522 = v0;
    nat_mul(a521, a522)
}

pub fn f18(v0: u64) -> R<u64> {
    let a523 = v0;
    let a524 = 0u64;
    if nat_eq(a523, a524) {
        Ok(0u64)
    } else {
        let a527 = 4u64;
        let a528 = {
            let a525 = 3u64;
            let a526 = v0;
            nat_mul(a525, a526)?
        };
        nat_add(a527, a528)
    }
}

pub fn f19(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a530 = match v0.clone() {
            Adt2::C0(_, h529, _, _, _, _, _, _, _, _, _) => {
                h529
            }
        };
        let a531 = v1;
        let a532 = 0u64;
        nat_quot(a530, a531, a532)
    }, match v0.clone() {
        Adt2::C0(_, _, h533, _, _, _, _, _, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h534, _, _, _, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h535, _, _, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h536, _, _, _, _, _) => {
            h536
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h537, _, _, _, _) => {
            h537
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h538, _, _, _) => {
            h538
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h539, _, _) => {
            h539
        }
    })
}

pub fn f20(v0: Adt2, v1: Adt2) -> bool {
    let a542 = match v0.clone() {
        Adt2::C0(_, h540, _, _, _, _, _, _, _, _, _) => {
            h540
        }
    };
    let a543 = match v1.clone() {
        Adt2::C0(_, h541, _, _, _, _, _, _, _, _, _) => {
            h541
        }
    };
    if if if nat_eq(a542, a543) {
        let a546 = match v0.clone() {
            Adt2::C0(_, _, h544, _, _, _, _, _, _, _, _) => {
                h544
            }
        };
        let a547 = match v1.clone() {
            Adt2::C0(_, _, h545, _, _, _, _, _, _, _, _) => {
                h545
            }
        };
        nat_eq(a546, a547)
    } else {
        false
    } {
        let a550 = match v0.clone() {
            Adt2::C0(_, _, _, h548, _, _, _, _, _, _, _) => {
                h548
            }
        };
        let a551 = match v1.clone() {
            Adt2::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                h549
            }
        };
        nat_eq(a550, a551)
    } else {
        false
    } {
        let a554 = match v0 {
            Adt2::C0(_, _, _, _, h552, _, _, _, _, _, _) => {
                h552
            }
        };
        let a555 = match v1 {
            Adt2::C0(_, _, _, _, h553, _, _, _, _, _, _) => {
                h553
            }
        };
        nat_eq(a554, a555)
    } else {
        false
    }
}

pub fn f21(v0: Adt2) -> R<bool> {
    let a557 = match v0.clone() {
        Adt2::C0(_, h556, _, _, _, _, _, _, _, _, _) => {
            h556
        }
    };
    let a558 = 1u64;
    if if if if nat_eq(a557, a558) {
        true
    } else {
        let a560 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h559, _, _, _, _, _) => {
                h559
            }
        };
        let a561 = 4u64;
        nat_eq(a560, a561)
    } {
        let a563 = match v0.clone() {
            Adt2::C0(_, _, h562, _, _, _, _, _, _, _, _) => {
                h562
            }
        };
        let a564 = 1u64;
        if nat_eq(a563, a564) {
            true
        } else {
            let a569 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h565, _, _, _, _) => {
                    h565
                }
            };
            let a570 = {
                let a567 = match v0.clone() {
                    Adt2::C0(_, h566, _, _, _, _, _, _, _, _, _) => {
                        h566
                    }
                };
                let a568 = 4u64;
                nat_mul(a567, a568)?
            };
            nat_eq(a569, a570)
        }
    } else {
        false
    } {
        let a572 = match v0.clone() {
            Adt2::C0(_, _, _, h571, _, _, _, _, _, _, _) => {
                h571
            }
        };
        let a573 = 1u64;
        if nat_eq(a572, a573) {
            true
        } else {
            let a581 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h574, _, _, _) => {
                    h574
                }
            };
            let a582 = {
                let a579 = {
                    let a577 = match v0.clone() {
                        Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                            h575
                        }
                    };
                    let a578 = match v0.clone() {
                        Adt2::C0(_, _, h576, _, _, _, _, _, _, _, _) => {
                            h576
                        }
                    };
                    nat_mul(a577, a578)?
                };
                let a580 = 4u64;
                nat_mul(a579, a580)?
            };
            nat_eq(a581, a582)
        }
    } else {
        false
    } {
        let a584 = match v0.clone() {
            Adt2::C0(_, _, _, _, h583, _, _, _, _, _, _) => {
                h583
            }
        };
        let a585 = 1u64;
        if nat_eq(a584, a585) {
            Ok(true)
        } else {
            let a596 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h586, _, _) => {
                    h586
                }
            };
            let a597 = {
                let a594 = {
                    let a592 = {
                        let a589 = match v0.clone() {
                            Adt2::C0(_, h587, _, _, _, _, _, _, _, _, _) => {
                                h587
                            }
                        };
                        let a590 = match v0.clone() {
                            Adt2::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                                h588
                            }
                        };
                        nat_mul(a589, a590)?
                    };
                    let a593 = match v0 {
                        Adt2::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
                            h591
                        }
                    };
                    nat_mul(a592, a593)?
                };
                let a595 = 4u64;
                nat_mul(a594, a595)?
            };
            Ok(nat_eq(a596, a597))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2, v1: Adt2) -> R<bool> {
    let a599 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h598, _) => {
            h598
        }
    };
    let a600 = 0u64;
    if nat_eq(a599, a600) {
        Ok(true)
    } else {
        let a602 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h601, _) => {
                h601
            }
        };
        let a603 = 0u64;
        if nat_eq(a602, a603) {
            Ok(true)
        } else {
            let a605 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h604) => {
                    h604
                }
            };
            let a606 = 0u64;
            if nat_eq(a605, a606) {
                Ok(true)
            } else {
                let a608 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h607) => {
                        h607
                    }
                };
                let a609 = 0u64;
                if nat_eq(a608, a609) {
                    Ok(true)
                } else {
                    let a615 = {
                        let a612 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h610) => {
                                h610
                            }
                        };
                        let a613 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h611, _) => {
                                h611
                            }
                        };
                        nat_add(a612, a613)?
                    };
                    let a616 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h614) => {
                            h614
                        }
                    };
                    if nat_le(a615, a616) {
                        Ok(true)
                    } else {
                        let a622 = {
                            let a619 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h617) => {
                                    h617
                                }
                            };
                            let a620 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h618, _) => {
                                    h618
                                }
                            };
                            nat_add(a619, a620)?
                        };
                        let a623 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h621) => {
                                h621
                            }
                        };
                        Ok(nat_le(a622, a623))
                    }
                }
            }
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a628 = {
        let a624 = v0;
        let a625 = v1;
        nat_mul(a624, a625)?
    };
    let a629 = {
        let a626 = v2;
        let a627 = f84(v3);
        nat_mul(a626, a627)?
    };
    nat_mul(a628, a629)
}

pub fn f24(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a632 = v0;
    let a633 = {
        let a630 = f85(v1)?;
        let a631 = v2;
        nat_mul(a630, a631)?
    };
    nat_add(a632, a633)
}

pub fn f25(v0: u64) -> u64 {
    let a634 = v0;
    let a635 = 30u64;
    if nat_eq(a634, a635) {
        2u64
    } else {
        4u64
    }
}

pub fn f26(v0: u64) -> bool {
    let a636 = v0;
    let a637 = 2139095040u64;
    if nat_le(a636, a637) {
        true
    } else {
        let a638 = v0;
        let a639 = 2147483648u64;
        nat_eq(a638, a639)
    }
}

pub fn f27(v0: Adt2, v1: u64) -> bool {
    let a641 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h640, _, _, _, _, _) => {
            h640
        }
    };
    let a642 = v1;
    if if nat_eq(a641, a642) {
        let a647 = {
            let a644 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h643) => {
                    h643
                }
            };
            let a645 = v1;
            let a646 = 0u64;
            nat_rem(a644, a645, a646)
        };
        let a648 = 0u64;
        nat_eq(a647, a648)
    } else {
        false
    } {
        f86(match v0.clone() {
            Adt2::C0(_, _, h649, _, _, _, _, _, _, _, _) => {
                h649
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h650, _, _, _, _, _, _, _) => {
                h650
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h651, _, _, _, _, _, _) => {
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
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h654, _, _) => {
                h654
            }
        }, v1)
    } else {
        false
    }
}

pub fn f28(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f20(v0.clone(), v1.clone()) {
        let a657 = match v0.clone() {
            Adt2::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
                h655
            }
        };
        let a658 = match v1.clone() {
            Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        nat_eq(a657, a658)
    } else {
        false
    } {
        let a661 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h659) => {
                h659
            }
        };
        let a662 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h660) => {
                h660
            }
        };
        nat_eq(a661, a662)
    } else {
        false
    } {
        let a665 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h663, _, _, _, _, _) => {
                h663
            }
        };
        let a666 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h664, _, _, _, _, _) => {
                h664
            }
        };
        nat_eq(a665, a666)
    } else {
        false
    } {
        let a669 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h667, _, _, _, _) => {
                h667
            }
        };
        let a670 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h668, _, _, _, _) => {
                h668
            }
        };
        nat_eq(a669, a670)
    } else {
        false
    } {
        let a673 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h671, _, _, _) => {
                h671
            }
        };
        let a674 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h672, _, _, _) => {
                h672
            }
        };
        nat_eq(a673, a674)
    } else {
        false
    } {
        let a677 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h675, _, _) => {
                h675
            }
        };
        let a678 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h676, _, _) => {
                h676
            }
        };
        nat_eq(a677, a678)
    } else {
        false
    }
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a679 = v0;
    let a680 = 0u64;
    if if if nat_eq(a679, a680) {
        true
    } else {
        let a681 = v1;
        let a682 = 0u64;
        nat_eq(a681, a682)
    } {
        true
    } else {
        let a683 = v2;
        let a684 = 0u64;
        nat_eq(a683, a684)
    } {
        true
    } else {
        let a685 = v3;
        let a686 = 0u64;
        nat_eq(a685, a686)
    }
}

pub fn f30(v0: Adt2, v1: u64) -> R<bool> {
    f87({
        let a688 = match v0.clone() {
            Adt2::C0(_, h687, _, _, _, _, _, _, _, _, _) => {
                h687
            }
        };
        let a689 = v1;
        nat_mul(a688, a689)?
    }, match v0.clone() {
        Adt2::C0(_, _, h690, _, _, _, _, _, _, _, _) => {
            h690
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h691, _, _, _, _, _, _, _) => {
            h691
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h692, _, _, _, _, _, _) => {
            h692
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h693, _, _, _, _) => {
            h693
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h694, _, _, _) => {
            h694
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h695, _, _) => {
            h695
        }
    })
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a696 = v1;
    let a697 = 0u64;
    if if nat_eq(a696, a697) {
        true
    } else {
        let a698 = v2;
        let a699 = 0u64;
        nat_eq(a698, a699)
    } {
        Ok(0u64)
    } else {
        let a706 = {
            let a700 = f88(v1, v2, v3)?;
            let a701 = f89(v0, v1, v2)?;
            nat_add(a700, a701)?
        };
        let a707 = {
            let a704 = f90(v0);
            let a705 = {
                let a702 = v1;
                let a703 = v2;
                nat_mul(a702, a703)?
            };
            nat_mul(a704, a705)?
        };
        nat_add(a706, a707)
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a708 = v0;
    let a709 = 0u64;
    if if nat_eq(a708, a709) {
        true
    } else {
        let a710 = v1;
        let a711 = 0u64;
        nat_eq(a710, a711)
    } {
        Ok(3u64)
    } else {
        let a714 = {
            let a712 = 3u64;
            let a713 = v1;
            nat_add(a712, a713)?
        };
        let a715 = v2;
        nat_add(a714, a715)
    }
}

pub fn f33(v0: u64) -> bool {
    let a716 = v0;
    let a717 = 2139095040u64;
    if nat_le(a716, a717) {
        true
    } else {
        let a718 = v0;
        let a719 = 2147483648u64;
        nat_eq(a718, a719)
    }
}

pub fn f34(v0: u64, v1: u64) -> R<u64> {
    let a720 = v0;
    let a721 = 0u64;
    if nat_eq(a720, a721) {
        Ok(0u64)
    } else {
        let a722 = v1;
        let a723 = f91(v0)?;
        nat_mul(a722, a723)
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a724 = v0;
    let a725 = 0u64;
    if if if if nat_eq(a724, a725) {
        true
    } else {
        let a726 = v1;
        let a727 = 0u64;
        nat_eq(a726, a727)
    } {
        true
    } else {
        let a728 = v2;
        let a729 = 0u64;
        nat_eq(a728, a729)
    } {
        true
    } else {
        let a730 = v3;
        let a731 = 0u64;
        nat_eq(a730, a731)
    } {
        Ok(0u64)
    } else {
        let a744 = {
            let a734 = 2u64;
            let a735 = {
                let a732 = v1;
                let a733 = v2;
                nat_mul(a732, a733)?
            };
            nat_mul(a734, a735)?
        };
        let a745 = {
            let a742 = {
                let a738 = {
                    let a736 = v1;
                    let a737 = v2;
                    nat_mul(a736, a737)?
                };
                let a739 = v3;
                nat_mul(a738, a739)?
            };
            let a743 = {
                let a740 = 1u64;
                let a741 = v0;
                nat_add(a740, a741)?
            };
            nat_mul(a742, a743)?
        };
        nat_add(a744, a745)
    }
}

pub fn f36(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m746 = v1;
    match m746 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f92(v0, v3, v2)
        }
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a748 = match v0.clone() {
        Adt2::C0(h747, _, _, _, _, _, _, _, _, _, _) => {
            h747
        }
    };
    let a749 = 1u64;
    if if nat_eq(a748, a749) {
        true
    } else {
        let a751 = match v0.clone() {
            Adt2::C0(h750, _, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        };
        let a752 = 30u64;
        nat_eq(a751, a752)
    } {
        let a754 = match v1.clone() {
            Adt2::C0(h753, _, _, _, _, _, _, _, _, _, _) => {
                h753
            }
        };
        let a755 = 1u64;
        if if if if if if if if if if if nat_eq(a754, a755) {
            let a758 = match v0.clone() {
                Adt2::C0(_, h756, _, _, _, _, _, _, _, _, _) => {
                    h756
                }
            };
            let a759 = match v1.clone() {
                Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                    h757
                }
            };
            nat_eq(a758, a759)
        } else {
            false
        } {
            let a761 = 0u64;
            let a762 = match v1.clone() {
                Adt2::C0(_, _, h760, _, _, _, _, _, _, _, _) => {
                    h760
                }
            };
            nat_lt(a761, a762)
        } else {
            false
        } {
            let a765 = match v1.clone() {
                Adt2::C0(_, _, h763, _, _, _, _, _, _, _, _) => {
                    h763
                }
            };
            let a766 = match v1.clone() {
                Adt2::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                    h764
                }
            };
            nat_eq(a765, a766)
        } else {
            false
        } {
            let a768 = match v1.clone() {
                Adt2::C0(_, _, _, _, h767, _, _, _, _, _, _) => {
                    h767
                }
            };
            let a769 = 1u64;
            nat_eq(a768, a769)
        } else {
            false
        } {
            let a777 = match v0.clone() {
                Adt2::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                    h770
                }
            };
            let a778 = {
                let a775 = {
                    let a773 = match v1.clone() {
                        Adt2::C0(_, _, h771, _, _, _, _, _, _, _, _) => {
                            h771
                        }
                    };
                    let a774 = match v1.clone() {
                        Adt2::C0(_, _, h772, _, _, _, _, _, _, _, _) => {
                            h772
                        }
                    };
                    nat_add(a773, a774)?
                };
                let a776 = 1u64;
                nat_sub(a775, a776)
            };
            nat_eq(a777, a778)
        } else {
            false
        } {
            let a780 = 0u64;
            let a781 = match v0.clone() {
                Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                    h779
                }
            };
            nat_lt(a780, a781)
        } else {
            false
        } {
            let a783 = 0u64;
            let a784 = match v0.clone() {
                Adt2::C0(_, _, _, _, h782, _, _, _, _, _, _) => {
                    h782
                }
            };
            nat_lt(a783, a784)
        } else {
            false
        } {
            let a786 = 2u64;
            let a787 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h785, _, _, _, _, _) => {
                    h785
                }
            };
            nat_le(a786, a787)
        } else {
            false
        } {
            let a789 = 2u64;
            let a790 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h788, _, _, _, _, _) => {
                    h788
                }
            };
            nat_le(a789, a790)
        } else {
            false
        } {
            f22(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f15(v1.clone(), 1u64)?, 2u64, f19(v0, 1u64), f19(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f38(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m791 = v1;
    match m791 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a793 = match v0.clone() {
                Adt2::C0(h792, _, _, _, _, _, _, _, _, _, _) => {
                    h792
                }
            };
            let a794 = 0u64;
            if if if if if if if if if if if if if nat_eq(a793, a794) {
                true
            } else {
                let a796 = match v0.clone() {
                    Adt2::C0(h795, _, _, _, _, _, _, _, _, _, _) => {
                        h795
                    }
                };
                let a797 = 1u64;
                nat_eq(a796, a797)
            } {
                let a799 = match v3.clone() {
                    Adt2::C0(h798, _, _, _, _, _, _, _, _, _, _) => {
                        h798
                    }
                };
                let a800 = 26u64;
                nat_eq(a799, a800)
            } else {
                false
            } {
                let a802 = match v2.clone() {
                    Adt2::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                        h801
                    }
                };
                let a803 = 0u64;
                nat_eq(a802, a803)
            } else {
                false
            } {
                let a806 = match v0.clone() {
                    Adt2::C0(_, h804, _, _, _, _, _, _, _, _, _) => {
                        h804
                    }
                };
                let a807 = match v2.clone() {
                    Adt2::C0(_, h805, _, _, _, _, _, _, _, _, _) => {
                        h805
                    }
                };
                nat_eq(a806, a807)
            } else {
                false
            } {
                let a810 = match v3.clone() {
                    Adt2::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                        h808
                    }
                };
                let a811 = match v0.clone() {
                    Adt2::C0(_, _, h809, _, _, _, _, _, _, _, _) => {
                        h809
                    }
                };
                nat_le(a810, a811)
            } else {
                false
            } {
                let a813 = match v0.clone() {
                    Adt2::C0(_, _, _, h812, _, _, _, _, _, _, _) => {
                        h812
                    }
                };
                let a814 = 1u64;
                nat_eq(a813, a814)
            } else {
                false
            } {
                let a816 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h815, _, _, _, _, _, _) => {
                        h815
                    }
                };
                let a817 = 1u64;
                nat_eq(a816, a817)
            } else {
                false
            } {
                let a819 = match v2.clone() {
                    Adt2::C0(_, _, _, h818, _, _, _, _, _, _, _) => {
                        h818
                    }
                };
                let a820 = 1u64;
                nat_eq(a819, a820)
            } else {
                false
            } {
                let a822 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h821, _, _, _, _, _, _) => {
                        h821
                    }
                };
                let a823 = 1u64;
                nat_eq(a822, a823)
            } else {
                false
            } {
                let a825 = match v3.clone() {
                    Adt2::C0(_, _, h824, _, _, _, _, _, _, _, _) => {
                        h824
                    }
                };
                let a826 = 1u64;
                nat_eq(a825, a826)
            } else {
                false
            } {
                let a828 = match v3.clone() {
                    Adt2::C0(_, _, _, h827, _, _, _, _, _, _, _) => {
                        h827
                    }
                };
                let a829 = 1u64;
                nat_eq(a828, a829)
            } else {
                false
            } {
                let a831 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h830, _, _, _, _, _, _) => {
                        h830
                    }
                };
                let a832 = 1u64;
                nat_eq(a831, a832)
            } else {
                false
            } {
                if f29(match v2.clone() {
                    Adt2::C0(_, h833, _, _, _, _, _, _, _, _, _) => {
                        h833
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h834, _, _, _, _, _, _, _, _) => {
                        h834
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
                        h835
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h836, _, _, _, _, _, _) => {
                        h836
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f19(v0, 1u64), f19(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a838 = match v3.clone() {
                        Adt2::C0(_, h837, _, _, _, _, _, _, _, _, _) => {
                            h837
                        }
                    };
                    let a839 = 0u64;
                    if nat_eq(a838, a839) {
                        Ok(Adt6::C4(f15(v2.clone(), 1u64)?, 4u64, f19(v0, 1u64), f19(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a841 = match v0.clone() {
                            Adt2::C0(_, h840, _, _, _, _, _, _, _, _, _) => {
                                h840
                            }
                        };
                        let a842 = 2147483647u64;
                        if if if if if if nat_le(a841, a842) {
                            let a844 = match v3.clone() {
                                Adt2::C0(_, h843, _, _, _, _, _, _, _, _, _) => {
                                    h843
                                }
                            };
                            let a845 = 2147483647u64;
                            nat_le(a844, a845)
                        } else {
                            false
                        } {
                            let a847 = 4u64;
                            let a848 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h846, _, _, _, _, _) => {
                                    h846
                                }
                            };
                            nat_le(a847, a848)
                        } else {
                            false
                        } {
                            let a854 = {
                                let a851 = match v2.clone() {
                                    Adt2::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                                        h849
                                    }
                                };
                                let a852 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h850, _, _, _, _, _) => {
                                        h850
                                    }
                                };
                                nat_mul(a851, a852)?
                            };
                            let a855 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h853, _, _, _, _) => {
                                    h853
                                }
                            };
                            nat_le(a854, a855)
                        } else {
                            false
                        } {
                            f22(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f22(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a869 = {
                                    let a859 = {
                                        let a857 = 2u64;
                                        let a858 = match v3.clone() {
                                            Adt2::C0(_, h856, _, _, _, _, _, _, _, _, _) => {
                                                h856
                                            }
                                        };
                                        nat_mul(a857, a858)?
                                    };
                                    let a860 = f15(v2.clone(), 1u64)?;
                                    nat_add(a859, a860)?
                                };
                                let a870 = {
                                    let a867 = match v3.clone() {
                                        Adt2::C0(_, h861, _, _, _, _, _, _, _, _, _) => {
                                            h861
                                        }
                                    };
                                    let a868 = {
                                        let a865 = 1u64;
                                        let a866 = {
                                            let a863 = 2u64;
                                            let a864 = match v0.clone() {
                                                Adt2::C0(_, h862, _, _, _, _, _, _, _, _, _) => {
                                                    h862
                                                }
                                            };
                                            nat_mul(a863, a864)?
                                        };
                                        nat_add(a865, a866)?
                                    };
                                    nat_mul(a867, a868)?
                                };
                                nat_add(a869, a870)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h871, _, _, _, _, _, _, _, _, _) => {
                                    h871
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h872, _, _, _, _, _, _, _, _, _) => {
                                    h872
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h873, _, _, _, _, _) => {
                                    h873
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h874, _, _, _, _) => {
                                    h874
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h875, _, _, _) => {
                                    h875
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h876, _, _) => {
                                    h876
                                }
                            }), f19(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h877, _, _, _, _, _, _, _, _, _, _) => {
                                    h877
                                }
                            }, f19(v3, 1u64))))
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

pub fn f39(v0: u64) -> bool {
    let a878 = v0;
    let a879 = 1u64;
    if if nat_eq(a878, a879) {
        true
    } else {
        let a880 = v0;
        let a881 = 33u64;
        nat_eq(a880, a881)
    } {
        true
    } else {
        let a882 = v0;
        let a883 = 34u64;
        nat_eq(a882, a883)
    }
}

pub fn f40(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a886 = match v1.clone() {
        Adt2::C0(h884, _, _, _, _, _, _, _, _, _, _) => {
            h884
        }
    };
    let a887 = match v3.clone() {
        Adt2::C0(h885, _, _, _, _, _, _, _, _, _, _) => {
            h885
        }
    };
    if nat_eq(a886, a887) {
        f93(v0, v1, v2, v3)
    } else {
        if if if if f94(match v1.clone() {
            Adt2::C0(h888, _, _, _, _, _, _, _, _, _, _) => {
                h888
            }
        }) {
            f94(match v3.clone() {
                Adt2::C0(h889, _, _, _, _, _, _, _, _, _, _) => {
                    h889
                }
            })
        } else {
            false
        } {
            let a890 = v0;
            let a891 = 33u64;
            if nat_eq(a890, a891) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a892 = f15(v1.clone(), 1u64)?;
            let a893 = f15(v3.clone(), 1u64)?;
            nat_eq(a892, a893)
        } else {
            false
        } {
            let m895 = f95(match v3.clone() {
                Adt2::C0(h894, _, _, _, _, _, _, _, _, _, _) => {
                    h894
                }
            });
            match m895 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v4) => {
                    Ok(Adt6::C4(f15(v3.clone(), 1u64)?, match v4 {
                        Adt11::C0(_, _, h896) => {
                            h896
                        }
                    }, f19(v1.clone(), 1u64), f19(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h897, _, _, _, _, _, _, _, _, _, _) => {
                            h897
                        }
                    }, match v3 {
                        Adt2::C0(h898, _, _, _, _, _, _, _, _, _, _) => {
                            h898
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f41(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a900 = match v0.clone() {
        Adt2::C0(h899, _, _, _, _, _, _, _, _, _, _) => {
            h899
        }
    };
    let a901 = 0u64;
    if if if if nat_eq(a900, a901) {
        true
    } else {
        let a903 = match v0.clone() {
            Adt2::C0(h902, _, _, _, _, _, _, _, _, _, _) => {
                h902
            }
        };
        let a904 = 1u64;
        nat_eq(a903, a904)
    } {
        true
    } else {
        let a906 = match v0.clone() {
            Adt2::C0(h905, _, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        };
        let a907 = 25u64;
        if nat_eq(a906, a907) {
            true
        } else {
            let a909 = match v0.clone() {
                Adt2::C0(h908, _, _, _, _, _, _, _, _, _, _) => {
                    h908
                }
            };
            let a910 = 26u64;
            nat_eq(a909, a910)
        }
    } {
        true
    } else {
        let a912 = match v0.clone() {
            Adt2::C0(h911, _, _, _, _, _, _, _, _, _, _) => {
                h911
            }
        };
        let a913 = 30u64;
        nat_eq(a912, a913)
    } {
        let a916 = match v0.clone() {
            Adt2::C0(h914, _, _, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a917 = match v1.clone() {
            Adt2::C0(h915, _, _, _, _, _, _, _, _, _, _) => {
                h915
            }
        };
        if if nat_eq(a916, a917) {
            f96(match v1.clone() {
                Adt2::C0(_, h918, _, _, _, _, _, _, _, _, _) => {
                    h918
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h919, _, _, _, _, _, _, _, _) => {
                    h919
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h920, _, _, _, _, _, _, _) => {
                    h920
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h921, _, _, _, _, _, _) => {
                    h921
                }
            }, match v0.clone() {
                Adt2::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
                    h922
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h923, _, _, _, _, _, _, _, _) => {
                    h923
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h924, _, _, _, _, _, _, _) => {
                    h924
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h925, _, _, _, _, _, _) => {
                    h925
                }
            })
        } else {
            false
        } {
            let m927 = f95(match v0.clone() {
                Adt2::C0(h926, _, _, _, _, _, _, _, _, _, _) => {
                    h926
                }
            });
            match m927 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v2) => {
                    Ok(Adt6::C4(f15(v1.clone(), 1u64)?, match v2 {
                        Adt11::C0(_, _, h928) => {
                            h928
                        }
                    }, f19(v0, 1u64), f19(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f42(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m929 = v2;
    match m929 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a930 = v0;
            let a931 = 4u64;
            if if if nat_lt(a930, a931) {
                let a934 = match v1.clone() {
                    Adt2::C0(h932, _, _, _, _, _, _, _, _, _, _) => {
                        h932
                    }
                };
                let a935 = match v4.clone() {
                    Adt2::C0(h933, _, _, _, _, _, _, _, _, _, _) => {
                        h933
                    }
                };
                if nat_eq(a934, a935) {
                    let a938 = match v1.clone() {
                        Adt2::C0(h936, _, _, _, _, _, _, _, _, _, _) => {
                            h936
                        }
                    };
                    let a939 = match v3.clone() {
                        Adt2::C0(h937, _, _, _, _, _, _, _, _, _, _) => {
                            h937
                        }
                    };
                    nat_eq(a938, a939)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f97(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h940, _, _, _, _, _, _, _, _, _) => {
                        h940
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h941, _, _, _, _, _, _, _, _, _) => {
                        h941
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h942, _, _, _, _, _, _, _, _, _) => {
                        h942
                    }
                })? {
                    f97(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h943, _, _, _, _, _, _, _, _) => {
                            h943
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h944, _, _, _, _, _, _, _, _) => {
                            h944
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h945, _, _, _, _, _, _, _, _) => {
                            h945
                        }
                    })?
                } else {
                    false
                } {
                    f97(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h946, _, _, _, _, _, _, _) => {
                            h946
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h947, _, _, _, _, _, _, _) => {
                            h947
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h948, _, _, _, _, _, _, _) => {
                            h948
                        }
                    })?
                } else {
                    false
                } {
                    f97(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h949, _, _, _, _, _, _) => {
                            h949
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h950, _, _, _, _, _, _) => {
                            h950
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h951, _, _, _, _, _, _) => {
                            h951
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m953 = f95(match v1.clone() {
                    Adt2::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
                        h952
                    }
                });
                match m953 {
                    Adt10::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt10::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt11::C0(_, h954, _) => {
                                h954
                            }
                        };
                        let a959 = {
                            let a956 = match v1.clone() {
                                Adt2::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
                                    h955
                                }
                            };
                            let a957 = v6;
                            let a958 = 1u64;
                            nat_rem(a956, a957, a958)
                        };
                        let a960 = 0u64;
                        if if if nat_eq(a959, a960) {
                            let a965 = {
                                let a962 = match v4.clone() {
                                    Adt2::C0(_, h961, _, _, _, _, _, _, _, _, _) => {
                                        h961
                                    }
                                };
                                let a963 = v6;
                                let a964 = 1u64;
                                nat_rem(a962, a963, a964)
                            };
                            let a966 = 0u64;
                            nat_eq(a965, a966)
                        } else {
                            false
                        } {
                            let a971 = {
                                let a968 = match v3.clone() {
                                    Adt2::C0(_, h967, _, _, _, _, _, _, _, _, _) => {
                                        h967
                                    }
                                };
                                let a969 = v6;
                                let a970 = 1u64;
                                nat_rem(a968, a969, a970)
                            };
                            let a972 = 0u64;
                            nat_eq(a971, a972)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f15(v3.clone(), v6)?, match v5 {
                                Adt11::C0(_, _, h973) => {
                                    h973
                                }
                            }, f19(v1, v6), f19(v3, v6), Adt4::C2(v0, f19(v4, v6))))
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

pub fn f43(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a975 = match v2.clone() {
        Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
            h974
        }
    };
    let a976 = 0u64;
    if if nat_eq(a975, a976) {
        let a978 = match v3.clone() {
            Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a979 = 0u64;
        nat_eq(a978, a979)
    } else {
        false
    } {
        let a980 = v0;
        let a981 = 61u64;
        if nat_eq(a980, a981) {
            let v4: bool = {
                let a983 = 0u64;
                let a984 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h982, _, _, _, _, _, _, _) => {
                        h982
                    }
                };
                nat_lt(a983, a984)
            };
            if if if if if if f98(match v2.clone() {
                Adt2::C0(_, h985, _, _, _, _, _, _, _, _, _) => {
                    h985
                }
            }, match v1.clone() {
                Adt1::C0(h986, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h986
                }
            }, match v1.clone() {
                Adt1::C0(_, h987, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h987
                }
            }, match v3.clone() {
                Adt2::C0(_, h988, _, _, _, _, _, _, _, _, _) => {
                    h988
                }
            })? {
                f98(match v2.clone() {
                    Adt2::C0(_, _, h989, _, _, _, _, _, _, _, _) => {
                        h989
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h990, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h990
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h991, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h991
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h992, _, _, _, _, _, _, _, _) => {
                        h992
                    }
                })?
            } else {
                false
            } {
                f98(match v2.clone() {
                    Adt2::C0(_, _, _, h993, _, _, _, _, _, _, _) => {
                        h993
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h994, _, _, _, _, _, _, _, _, _, _, _) => {
                        h994
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h995, _, _, _, _, _, _, _, _, _, _) => {
                        h995
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h996, _, _, _, _, _, _, _) => {
                        h996
                    }
                })?
            } else {
                false
            } {
                f98(match v2.clone() {
                    Adt2::C0(_, _, _, _, h997, _, _, _, _, _, _) => {
                        h997
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h998, _, _, _, _, _, _, _, _, _) => {
                        h998
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h999, _, _, _, _, _, _, _, _) => {
                        h999
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1000, _, _, _, _, _, _) => {
                        h1000
                    }
                })?
            } else {
                false
            } {
                let a1002 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1001, _, _, _, _, _, _, _) => {
                        h1001
                    }
                };
                let a1003 = 4294967295u64;
                nat_le(a1002, a1003)
            } else {
                false
            } {
                if v4 {
                    !f29(match v2.clone() {
                        Adt2::C0(_, h1004, _, _, _, _, _, _, _, _, _) => {
                            h1004
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
                            h1005
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1006, _, _, _, _, _, _, _) => {
                            h1006
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1007, _, _, _, _, _, _) => {
                            h1007
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f19(v2, 1u64), f19(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1008, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1008
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1009, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1009
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1010, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1010
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1011, _, _, _, _, _, _, _, _, _) => {
                        h1011
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a1012 = v0;
            let a1013 = 62u64;
            if nat_eq(a1012, a1013) {
                let a1015 = match v1.clone() {
                    Adt1::C0(h1014, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1014
                    }
                };
                let a1016 = 2147483648u64;
                if if if if if nat_lt(a1015, a1016) {
                    let a1018 = match v1.clone() {
                        Adt1::C0(_, h1017, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1017
                        }
                    };
                    let a1019 = 2147483648u64;
                    nat_lt(a1018, a1019)
                } else {
                    false
                } {
                    let a1022 = match v1.clone() {
                        Adt1::C0(h1020, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1020
                        }
                    };
                    let a1023 = match v2.clone() {
                        Adt2::C0(_, h1021, _, _, _, _, _, _, _, _, _) => {
                            h1021
                        }
                    };
                    nat_lt(a1022, a1023)
                } else {
                    false
                } {
                    let a1026 = match v1.clone() {
                        Adt1::C0(_, h1024, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1024
                        }
                    };
                    let a1027 = match v2.clone() {
                        Adt2::C0(_, h1025, _, _, _, _, _, _, _, _, _) => {
                            h1025
                        }
                    };
                    nat_lt(a1026, a1027)
                } else {
                    false
                } {
                    let a1030 = match v2.clone() {
                        Adt2::C0(_, _, h1028, _, _, _, _, _, _, _, _) => {
                            h1028
                        }
                    };
                    let a1031 = match v3.clone() {
                        Adt2::C0(_, _, h1029, _, _, _, _, _, _, _, _) => {
                            h1029
                        }
                    };
                    if if if nat_eq(a1030, a1031) {
                        let a1034 = match v2.clone() {
                            Adt2::C0(_, _, _, h1032, _, _, _, _, _, _, _) => {
                                h1032
                            }
                        };
                        let a1035 = match v3.clone() {
                            Adt2::C0(_, _, _, h1033, _, _, _, _, _, _, _) => {
                                h1033
                            }
                        };
                        nat_eq(a1034, a1035)
                    } else {
                        false
                    } {
                        let a1038 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1036, _, _, _, _, _, _) => {
                                h1036
                            }
                        };
                        let a1039 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1037, _, _, _, _, _, _) => {
                                h1037
                            }
                        };
                        nat_eq(a1038, a1039)
                    } else {
                        false
                    } {
                        let a1048 = match v3.clone() {
                            Adt2::C0(_, h1040, _, _, _, _, _, _, _, _, _) => {
                                h1040
                            }
                        };
                        let a1049 = {
                            let a1046 = {
                                let a1043 = match v2.clone() {
                                    Adt2::C0(_, h1041, _, _, _, _, _, _, _, _, _) => {
                                        h1041
                                    }
                                };
                                let a1044 = match v1.clone() {
                                    Adt1::C0(h1042, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1042
                                    }
                                };
                                nat_add(a1043, a1044)?
                            };
                            let a1047 = match v1.clone() {
                                Adt1::C0(_, h1045, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h1045
                                }
                            };
                            nat_add(a1046, a1047)?
                        };
                        nat_eq(a1048, a1049)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f19(v2, 1u64), f19(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1050, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1050
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a1051 = v0;
                let a1052 = 63u64;
                if nat_eq(a1051, a1052) {
                    if if if if if f99(match v2.clone() {
                        Adt2::C0(_, h1053, _, _, _, _, _, _, _, _, _) => {
                            h1053
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1054, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1054
                        }
                    })? {
                        f99(match v2.clone() {
                            Adt2::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                                h1055
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1056, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1056
                            }
                        })?
                    } else {
                        false
                    } {
                        f99(match v2.clone() {
                            Adt2::C0(_, _, _, h1057, _, _, _, _, _, _, _) => {
                                h1057
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1058, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1058
                            }
                        })?
                    } else {
                        false
                    } {
                        f99(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1059, _, _, _, _, _, _) => {
                                h1059
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1060, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1060
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1063 = match v2.clone() {
                            Adt2::C0(_, h1061, _, _, _, _, _, _, _, _, _) => {
                                h1061
                            }
                        };
                        let a1064 = match v3.clone() {
                            Adt2::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                                h1062
                            }
                        };
                        if if if nat_eq(a1063, a1064) {
                            let a1067 = match v2.clone() {
                                Adt2::C0(_, _, h1065, _, _, _, _, _, _, _, _) => {
                                    h1065
                                }
                            };
                            let a1068 = match v3.clone() {
                                Adt2::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
                                    h1066
                                }
                            };
                            nat_eq(a1067, a1068)
                        } else {
                            false
                        } {
                            let a1071 = match v2.clone() {
                                Adt2::C0(_, _, _, h1069, _, _, _, _, _, _, _) => {
                                    h1069
                                }
                            };
                            let a1072 = match v3.clone() {
                                Adt2::C0(_, _, _, h1070, _, _, _, _, _, _, _) => {
                                    h1070
                                }
                            };
                            nat_eq(a1071, a1072)
                        } else {
                            false
                        } {
                            let a1075 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1073, _, _, _, _, _, _) => {
                                    h1073
                                }
                            };
                            let a1076 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1074, _, _, _, _, _, _) => {
                                    h1074
                                }
                            };
                            nat_eq(a1075, a1076)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f19(v2, 1u64), f19(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1077, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1077
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1078, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1078
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1079, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1079
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1080, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1080
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

pub fn f44(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1081 = v3;
    match m1081 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1082 = v0;
                let a1083 = 4u64;
                nat_eq(a1082, a1083)
            };
            let v7: bool = {
                let a1085 = 0u64;
                let a1086 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1084, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1084
                    }
                };
                nat_lt(a1085, a1086)
            };
            let v8: u64 = f15(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1087, _, _, _, _, _, _, _, _, _) => {
                    h1087
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1088, _, _, _, _, _, _, _, _) => {
                    h1088
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1089, _, _, _, _, _, _, _) => {
                    h1089
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1090, _, _, _, _, _, _) => {
                    h1090
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1091, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1091
                }
            }, match v1.clone() {
                Adt1::C0(_, h1092, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1092
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1093, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1093
                }
            });
            if if if if if if if if if if if if v6 {
                let a1095 = match v2.clone() {
                    Adt2::C0(h1094, _, _, _, _, _, _, _, _, _, _) => {
                        h1094
                    }
                };
                let a1096 = 0u64;
                if nat_eq(a1095, a1096) {
                    let a1098 = match v5.clone() {
                        Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    let a1099 = 0u64;
                    nat_eq(a1098, a1099)
                } else {
                    false
                }
            } else {
                let a1101 = match v2.clone() {
                    Adt2::C0(h1100, _, _, _, _, _, _, _, _, _, _) => {
                        h1100
                    }
                };
                let a1102 = 0u64;
                if if nat_eq(a1101, a1102) {
                    true
                } else {
                    let a1104 = match v2.clone() {
                        Adt2::C0(h1103, _, _, _, _, _, _, _, _, _, _) => {
                            h1103
                        }
                    };
                    let a1105 = 26u64;
                    nat_eq(a1104, a1105)
                } {
                    let a1107 = match v5.clone() {
                        Adt2::C0(h1106, _, _, _, _, _, _, _, _, _, _) => {
                            h1106
                        }
                    };
                    let a1108 = 0u64;
                    if nat_eq(a1107, a1108) {
                        true
                    } else {
                        let a1110 = match v5.clone() {
                            Adt2::C0(h1109, _, _, _, _, _, _, _, _, _, _) => {
                                h1109
                            }
                        };
                        let a1111 = 26u64;
                        nat_eq(a1110, a1111)
                    }
                } else {
                    false
                }
            } {
                let a1114 = match v2.clone() {
                    Adt2::C0(h1112, _, _, _, _, _, _, _, _, _, _) => {
                        h1112
                    }
                };
                let a1115 = match v4.clone() {
                    Adt2::C0(h1113, _, _, _, _, _, _, _, _, _, _) => {
                        h1113
                    }
                };
                nat_eq(a1114, a1115)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                f21(v4.clone())?
            } else {
                false
            } {
                let a1117 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1116, _, _, _, _, _) => {
                        h1116
                    }
                };
                let a1118 = 4u64;
                nat_eq(a1117, a1118)
            } else {
                false
            } {
                let a1119 = v8;
                let a1120 = f15(v2.clone(), 1u64)?;
                nat_le(a1119, a1120)
            } else {
                false
            } {
                let a1122 = match v1.clone() {
                    Adt1::C0(_, _, _, h1121, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1121
                    }
                };
                let a1123 = 1073741824u64;
                nat_lt(a1122, a1123)
            } else {
                false
            } {
                let a1125 = match v1.clone() {
                    Adt1::C0(h1124, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                };
                let a1126 = 4294967295u64;
                if if if if nat_le(a1125, a1126) {
                    let a1128 = match v1.clone() {
                        Adt1::C0(_, h1127, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1127
                        }
                    };
                    let a1129 = 4294967295u64;
                    nat_le(a1128, a1129)
                } else {
                    false
                } {
                    let a1131 = match v1.clone() {
                        Adt1::C0(_, _, h1130, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1130
                        }
                    };
                    let a1132 = 4294967295u64;
                    nat_le(a1131, a1132)
                } else {
                    false
                } {
                    let a1134 = match v1.clone() {
                        Adt1::C0(_, _, _, h1133, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1133
                        }
                    };
                    let a1135 = 4294967295u64;
                    nat_le(a1134, a1135)
                } else {
                    false
                } {
                    let a1137 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1136, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1136
                        }
                    };
                    let a1138 = 4294967295u64;
                    nat_le(a1137, a1138)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1141 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1139) => {
                            h1139
                        }
                    };
                    let a1142 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1140) => {
                            h1140
                        }
                    };
                    nat_eq(a1141, a1142)
                } else {
                    true
                }
            } else {
                false
            } {
                f100(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1143, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1143
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1144, _) => {
                        h1144
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f15(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a1145 = v10;
                    let a1146 = v8;
                    nat_add(a1145, a1146)?
                }, 4u64, f19(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1147, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1147
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f45(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1149 = match v0.clone() {
        Adt2::C0(h1148, _, _, _, _, _, _, _, _, _, _) => {
            h1148
        }
    };
    let a1150 = 0u64;
    if if if nat_eq(a1149, a1150) {
        let a1152 = match v1.clone() {
            Adt2::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
                h1151
            }
        };
        let a1153 = 0u64;
        nat_eq(a1152, a1153)
    } else {
        false
    } {
        f96(match v0.clone() {
            Adt2::C0(_, h1154, _, _, _, _, _, _, _, _, _) => {
                h1154
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1155, _, _, _, _, _, _, _, _) => {
                h1155
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                h1156
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1157, _, _, _, _, _, _) => {
                h1157
            }
        }, match v1.clone() {
            Adt2::C0(_, h1158, _, _, _, _, _, _, _, _, _) => {
                h1158
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                h1159
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1160, _, _, _, _, _, _, _) => {
                h1160
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1161, _, _, _, _, _, _) => {
                h1161
            }
        })
    } else {
        false
    } {
        let v2: u64 = f15(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a1162 = v2;
            let a1163 = f15(v0.clone(), 1u64)?;
            nat_add(a1162, a1163)?
        }, 4u64, f19(v0, 1u64), f19(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f46(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1165 = match v1.clone() {
        Adt2::C0(h1164, _, _, _, _, _, _, _, _, _, _) => {
            h1164
        }
    };
    let a1166 = 0u64;
    if if if nat_eq(a1165, a1166) {
        true
    } else {
        let a1168 = match v1.clone() {
            Adt2::C0(h1167, _, _, _, _, _, _, _, _, _, _) => {
                h1167
            }
        };
        let a1169 = 1u64;
        nat_eq(a1168, a1169)
    } {
        true
    } else {
        let a1171 = match v1.clone() {
            Adt2::C0(h1170, _, _, _, _, _, _, _, _, _, _) => {
                h1170
            }
        };
        let a1172 = 30u64;
        nat_eq(a1171, a1172)
    } {
        let a1175 = match v1.clone() {
            Adt2::C0(h1173, _, _, _, _, _, _, _, _, _, _) => {
                h1173
            }
        };
        let a1176 = match v2.clone() {
            Adt2::C0(h1174, _, _, _, _, _, _, _, _, _, _) => {
                h1174
            }
        };
        if if if if if nat_eq(a1175, a1176) {
            let a1179 = match v1.clone() {
                Adt2::C0(_, h1177, _, _, _, _, _, _, _, _, _) => {
                    h1177
                }
            };
            let a1180 = match v2.clone() {
                Adt2::C0(_, h1178, _, _, _, _, _, _, _, _, _) => {
                    h1178
                }
            };
            nat_eq(a1179, a1180)
        } else {
            false
        } {
            let a1183 = match v1.clone() {
                Adt2::C0(_, _, _, h1181, _, _, _, _, _, _, _) => {
                    h1181
                }
            };
            let a1184 = match v2.clone() {
                Adt2::C0(_, _, _, h1182, _, _, _, _, _, _, _) => {
                    h1182
                }
            };
            nat_eq(a1183, a1184)
        } else {
            false
        } {
            let a1187 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1185, _, _, _, _, _, _) => {
                    h1185
                }
            };
            let a1188 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1186, _, _, _, _, _, _) => {
                    h1186
                }
            };
            nat_eq(a1187, a1188)
        } else {
            false
        } {
            let a1189 = v0;
            let a1190 = 4u64;
            if nat_eq(a1189, a1190) {
                let a1192 = match v1.clone() {
                    Adt2::C0(_, _, h1191, _, _, _, _, _, _, _, _) => {
                        h1191
                    }
                };
                let a1193 = 1u64;
                if nat_eq(a1192, a1193) {
                    let a1196 = match v1.clone() {
                        Adt2::C0(_, h1194, _, _, _, _, _, _, _, _, _) => {
                            h1194
                        }
                    };
                    let a1197 = match v2.clone() {
                        Adt2::C0(_, _, h1195, _, _, _, _, _, _, _, _) => {
                            h1195
                        }
                    };
                    nat_eq(a1196, a1197)
                } else {
                    false
                }
            } else {
                let a1198 = v0;
                let a1199 = 4u64;
                if if nat_lt(a1198, a1199) {
                    let a1202 = match v1.clone() {
                        Adt2::C0(_, h1200, _, _, _, _, _, _, _, _, _) => {
                            h1200
                        }
                    };
                    let a1203 = match v1.clone() {
                        Adt2::C0(_, _, h1201, _, _, _, _, _, _, _, _) => {
                            h1201
                        }
                    };
                    nat_eq(a1202, a1203)
                } else {
                    false
                } {
                    let a1206 = match v1.clone() {
                        Adt2::C0(_, _, h1204, _, _, _, _, _, _, _, _) => {
                            h1204
                        }
                    };
                    let a1207 = match v2.clone() {
                        Adt2::C0(_, _, h1205, _, _, _, _, _, _, _, _) => {
                            h1205
                        }
                    };
                    nat_eq(a1206, a1207)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1209 = f95(match v1.clone() {
                Adt2::C0(h1208, _, _, _, _, _, _, _, _, _, _) => {
                    h1208
                }
            });
            match m1209 {
                Adt10::C0 => {
                    Ok(Adt6::C0)
                }
                Adt10::C1(v3) => {
                    Ok(Adt6::C4(f15(v2.clone(), 1u64)?, match v3 {
                        Adt11::C0(_, _, h1210) => {
                            h1210
                        }
                    }, f19(v1, 1u64), f19(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f101(match v1.clone() {
        Adt2::C0(h1211, _, _, _, _, _, _, _, _, _, _) => {
            h1211
        }
    }, v0) {
        let a1214 = match v1.clone() {
            Adt2::C0(h1212, _, _, _, _, _, _, _, _, _, _) => {
                h1212
            }
        };
        let a1215 = match v2.clone() {
            Adt2::C0(h1213, _, _, _, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        nat_eq(a1214, a1215)
    } else {
        false
    } {
        let a1218 = match v1.clone() {
            Adt2::C0(_, h1216, _, _, _, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1219 = match v2.clone() {
            Adt2::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                h1217
            }
        };
        if if if nat_eq(a1218, a1219) {
            let a1222 = match v1.clone() {
                Adt2::C0(_, _, h1220, _, _, _, _, _, _, _, _) => {
                    h1220
                }
            };
            let a1223 = match v2.clone() {
                Adt2::C0(_, _, h1221, _, _, _, _, _, _, _, _) => {
                    h1221
                }
            };
            nat_eq(a1222, a1223)
        } else {
            false
        } {
            let a1226 = match v1.clone() {
                Adt2::C0(_, _, _, h1224, _, _, _, _, _, _, _) => {
                    h1224
                }
            };
            let a1227 = match v2.clone() {
                Adt2::C0(_, _, _, h1225, _, _, _, _, _, _, _) => {
                    h1225
                }
            };
            nat_eq(a1226, a1227)
        } else {
            false
        } {
            let a1230 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1228, _, _, _, _, _, _) => {
                    h1228
                }
            };
            let a1231 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1229, _, _, _, _, _, _) => {
                    h1229
                }
            };
            nat_eq(a1230, a1231)
        } else {
            false
        }
    } else {
        false
    } {
        let m1233 = f95(match v1.clone() {
            Adt2::C0(h1232, _, _, _, _, _, _, _, _, _, _) => {
                h1232
            }
        });
        match m1233 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                Ok(Adt6::C4(f15(v2.clone(), 1u64)?, match v3 {
                    Adt11::C0(_, _, h1234) => {
                        h1234
                    }
                }, f19(v1.clone(), 1u64), f19(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1235, _, _, _, _, _, _, _, _, _, _) => {
                        h1235
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f48(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1236, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1236
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1237, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1237
            }
        }
    };
    let a1238 = 0u64;
    let a1239 = v4;
    if if nat_lt(a1238, a1239) {
        let a1240 = v4;
        let a1241 = 2147483648u64;
        nat_lt(a1240, a1241)
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
        let v7: u64 = f102(match v5.clone() {
            Adt2::C0(_, _, h1242, _, _, _, _, _, _, _, _) => {
                h1242
            }
        }, v4)?;
        let v8: u64 = f102(match v5.clone() {
            Adt2::C0(_, _, _, h1243, _, _, _, _, _, _, _) => {
                h1243
            }
        }, v4)?;
        let a1245 = match v5.clone() {
            Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        };
        let a1246 = 0u64;
        if if if if if nat_eq(a1245, a1246) {
            let a1248 = match v6.clone() {
                Adt2::C0(h1247, _, _, _, _, _, _, _, _, _, _) => {
                    h1247
                }
            };
            let a1249 = 0u64;
            nat_eq(a1248, a1249)
        } else {
            false
        } {
            let a1251 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1250, _, _, _, _, _, _) => {
                    h1250
                }
            };
            let a1252 = 1u64;
            nat_eq(a1251, a1252)
        } else {
            false
        } {
            let a1255 = match v5.clone() {
                Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                    h1253
                }
            };
            let a1256 = match v6.clone() {
                Adt2::C0(_, h1254, _, _, _, _, _, _, _, _, _) => {
                    h1254
                }
            };
            nat_eq(a1255, a1256)
        } else {
            false
        } {
            if v0 {
                let a1258 = match v5.clone() {
                    Adt2::C0(_, _, h1257, _, _, _, _, _, _, _, _) => {
                        h1257
                    }
                };
                let a1259 = 2147483648u64;
                if if if nat_lt(a1258, a1259) {
                    let a1261 = match v5.clone() {
                        Adt2::C0(_, _, _, h1260, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    };
                    let a1262 = 2147483648u64;
                    nat_lt(a1261, a1262)
                } else {
                    false
                } {
                    let a1265 = {
                        let a1263 = v7;
                        let a1264 = v8;
                        nat_mul(a1263, a1264)?
                    };
                    let a1266 = 2147483648u64;
                    nat_le(a1265, a1266)
                } else {
                    false
                } {
                    f103(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1268 = match v1.clone() {
                    Adt1::C0(h1267, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1267
                    }
                };
                let a1269 = v7;
                if if if if nat_eq(a1268, a1269) {
                    let a1271 = match v1 {
                        Adt1::C0(_, h1270, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1270
                        }
                    };
                    let a1272 = v8;
                    nat_eq(a1271, a1272)
                } else {
                    false
                } {
                    f104(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1274 = match v6.clone() {
                        Adt2::C0(_, _, h1273, _, _, _, _, _, _, _, _) => {
                            h1273
                        }
                    };
                    let a1275 = v4;
                    if nat_eq(a1274, a1275) {
                        let a1277 = match v6.clone() {
                            Adt2::C0(_, _, _, h1276, _, _, _, _, _, _, _) => {
                                h1276
                            }
                        };
                        let a1278 = v4;
                        nat_eq(a1277, a1278)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1282 = match v6 {
                        Adt2::C0(_, _, _, _, h1279, _, _, _, _, _, _) => {
                            h1279
                        }
                    };
                    let a1283 = {
                        let a1280 = v7;
                        let a1281 = v8;
                        nat_mul(a1280, a1281)?
                    };
                    nat_eq(a1282, a1283)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f15(v3.clone(), 1u64)?, 4u64, f19(v2, 1u64), f19(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f49(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1285 = match v2.clone() {
        Adt2::C0(h1284, _, _, _, _, _, _, _, _, _, _) => {
            h1284
        }
    };
    let a1286 = 0u64;
    if if if if if nat_eq(a1285, a1286) {
        true
    } else {
        let a1288 = match v2.clone() {
            Adt2::C0(h1287, _, _, _, _, _, _, _, _, _, _) => {
                h1287
            }
        };
        let a1289 = 1u64;
        nat_eq(a1288, a1289)
    } {
        true
    } else {
        let a1291 = match v2.clone() {
            Adt2::C0(h1290, _, _, _, _, _, _, _, _, _, _) => {
                h1290
            }
        };
        let a1292 = 30u64;
        nat_eq(a1291, a1292)
    } {
        let a1295 = match v2.clone() {
            Adt2::C0(h1293, _, _, _, _, _, _, _, _, _, _) => {
                h1293
            }
        };
        let a1296 = match v3.clone() {
            Adt2::C0(h1294, _, _, _, _, _, _, _, _, _, _) => {
                h1294
            }
        };
        nat_eq(a1295, a1296)
    } else {
        false
    } {
        if if f20(v2.clone(), v3.clone()) {
            let a1298 = match v1.clone() {
                Adt1::C0(h1297, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1297
                }
            };
            let a1299 = 4294967296u64;
            nat_lt(a1298, a1299)
        } else {
            false
        } {
            let a1301 = match v1.clone() {
                Adt1::C0(_, h1300, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1300
                }
            };
            let a1302 = 4294967296u64;
            nat_lt(a1301, a1302)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f15(v3.clone(), 1u64)?, {
            let a1304 = match v2.clone() {
                Adt2::C0(h1303, _, _, _, _, _, _, _, _, _, _) => {
                    h1303
                }
            };
            let a1305 = 0u64;
            if nat_eq(a1304, a1305) {
                4u64
            } else {
                2u64
            }
        }, f19(v2.clone(), 1u64), f19(v3, 1u64), Adt4::C13({
            let a1306 = v0;
            let a1307 = 31u64;
            if nat_eq(a1306, a1307) {
                0u64
            } else {
                let a1308 = v0;
                let a1309 = 47u64;
                if nat_eq(a1308, a1309) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1310, _, _, _, _, _, _, _, _, _, _) => {
                h1310
            }
        }, match v1.clone() {
            Adt1::C0(h1311, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1311
            }
        }, match v1 {
            Adt1::C0(_, h1312, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1312
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1314 = match v0.clone() {
        Adt2::C0(h1313, _, _, _, _, _, _, _, _, _, _) => {
            h1313
        }
    };
    let a1315 = 0u64;
    if if if nat_eq(a1314, a1315) {
        let a1317 = match v1.clone() {
            Adt2::C0(h1316, _, _, _, _, _, _, _, _, _, _) => {
                h1316
            }
        };
        let a1318 = 0u64;
        nat_eq(a1317, a1318)
    } else {
        false
    } {
        f20(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f15(v1.clone(), 1u64)?, 4u64, f19(v0, 1u64), f19(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1320 = match v0.clone() {
        Adt2::C0(h1319, _, _, _, _, _, _, _, _, _, _) => {
            h1319
        }
    };
    let a1321 = 0u64;
    if if nat_eq(a1320, a1321) {
        let a1323 = match v1.clone() {
            Adt2::C0(h1322, _, _, _, _, _, _, _, _, _, _) => {
                h1322
            }
        };
        let a1324 = 26u64;
        nat_eq(a1323, a1324)
    } else {
        false
    } {
        let a1326 = match v0.clone() {
            Adt2::C0(_, _, _, h1325, _, _, _, _, _, _, _) => {
                h1325
            }
        };
        let a1327 = 1u64;
        if if if if if if if nat_eq(a1326, a1327) {
            let a1329 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1328, _, _, _, _, _, _) => {
                    h1328
                }
            };
            let a1330 = 1u64;
            nat_eq(a1329, a1330)
        } else {
            false
        } {
            let a1332 = match v0.clone() {
                Adt2::C0(_, h1331, _, _, _, _, _, _, _, _, _) => {
                    h1331
                }
            };
            let a1333 = 2147483647u64;
            nat_le(a1332, a1333)
        } else {
            false
        } {
            let a1336 = match v1.clone() {
                Adt2::C0(_, h1334, _, _, _, _, _, _, _, _, _) => {
                    h1334
                }
            };
            let a1337 = match v0.clone() {
                Adt2::C0(_, _, h1335, _, _, _, _, _, _, _, _) => {
                    h1335
                }
            };
            nat_eq(a1336, a1337)
        } else {
            false
        } {
            let a1339 = match v1.clone() {
                Adt2::C0(_, _, h1338, _, _, _, _, _, _, _, _) => {
                    h1338
                }
            };
            let a1340 = 1u64;
            nat_eq(a1339, a1340)
        } else {
            false
        } {
            let a1342 = match v1.clone() {
                Adt2::C0(_, _, _, h1341, _, _, _, _, _, _, _) => {
                    h1341
                }
            };
            let a1343 = 1u64;
            nat_eq(a1342, a1343)
        } else {
            false
        } {
            let a1345 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1344, _, _, _, _, _, _) => {
                    h1344
                }
            };
            let a1346 = 1u64;
            nat_eq(a1345, a1346)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1349 = match v0.clone() {
                    Adt2::C0(_, _, h1347, _, _, _, _, _, _, _, _) => {
                        h1347
                    }
                };
                let a1350 = f72(match v0.clone() {
                    Adt2::C0(_, h1348, _, _, _, _, _, _, _, _, _) => {
                        h1348
                    }
                })?;
                nat_mul(a1349, a1350)?
            }, 4u64, f19(v0, 1u64), f19(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f52(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1351 = v1;
    match m1351 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f74(f19(v0.clone(), 1u64))?;
            let a1353 = match v0.clone() {
                Adt2::C0(h1352, _, _, _, _, _, _, _, _, _, _) => {
                    h1352
                }
            };
            let a1354 = 26u64;
            if if if nat_eq(a1353, a1354) {
                let a1356 = match v3.clone() {
                    Adt2::C0(h1355, _, _, _, _, _, _, _, _, _, _) => {
                        h1355
                    }
                };
                let a1357 = 26u64;
                nat_eq(a1356, a1357)
            } else {
                false
            } {
                let a1359 = match v2.clone() {
                    Adt2::C0(h1358, _, _, _, _, _, _, _, _, _, _) => {
                        h1358
                    }
                };
                let a1360 = 27u64;
                nat_eq(a1359, a1360)
            } else {
                false
            } {
                if if if if if if f20(v0.clone(), v3.clone()) {
                    let a1362 = match v2.clone() {
                        Adt2::C0(_, h1361, _, _, _, _, _, _, _, _, _) => {
                            h1361
                        }
                    };
                    let a1363 = 1u64;
                    nat_eq(a1362, a1363)
                } else {
                    false
                } {
                    let a1365 = match v2.clone() {
                        Adt2::C0(_, _, h1364, _, _, _, _, _, _, _, _) => {
                            h1364
                        }
                    };
                    let a1366 = 1u64;
                    nat_eq(a1365, a1366)
                } else {
                    false
                } {
                    let a1368 = match v2.clone() {
                        Adt2::C0(_, _, _, h1367, _, _, _, _, _, _, _) => {
                            h1367
                        }
                    };
                    let a1369 = 1u64;
                    nat_eq(a1368, a1369)
                } else {
                    false
                } {
                    let a1371 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1370, _, _, _, _, _, _) => {
                            h1370
                        }
                    };
                    let a1372 = 1u64;
                    nat_eq(a1371, a1372)
                } else {
                    false
                } {
                    let a1373 = v4;
                    let a1374 = 9223372036854775808u64;
                    nat_lt(a1373, a1374)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1377 = 2u64;
                        let a1378 = {
                            let a1375 = v4;
                            let a1376 = 2u64;
                            nat_mul(a1375, a1376)?
                        };
                        nat_add(a1377, a1378)?
                    }, 8u64, f19(v0, 1u64), f19(v2, 1u64), Adt4::C18(v4, f19(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1379, _, _, _, _, _, _, _, _, _, _) => {
            h1379
        }
    };
    let a1380 = v3;
    let a1381 = 0u64;
    if if nat_eq(a1380, a1381) {
        true
    } else {
        let a1382 = v0;
        let a1383 = 13u64;
        if nat_eq(a1382, a1383) {
            let a1384 = v3;
            let a1385 = 1u64;
            if nat_eq(a1384, a1385) {
                true
            } else {
                let a1386 = v3;
                let a1387 = 30u64;
                nat_eq(a1386, a1387)
            }
        } else {
            false
        }
    } {
        let a1389 = v3;
        let a1390 = match v2.clone() {
            Adt2::C0(h1388, _, _, _, _, _, _, _, _, _, _) => {
                h1388
            }
        };
        if if nat_eq(a1389, a1390) {
            let a1391 = v0;
            let a1392 = 13u64;
            if nat_eq(a1391, a1392) {
                let a1394 = match v2.clone() {
                    Adt2::C0(_, h1393, _, _, _, _, _, _, _, _, _) => {
                        h1393
                    }
                };
                let a1395 = 1u64;
                if if if nat_eq(a1394, a1395) {
                    let a1397 = match v2.clone() {
                        Adt2::C0(_, _, h1396, _, _, _, _, _, _, _, _) => {
                            h1396
                        }
                    };
                    let a1398 = 1u64;
                    nat_eq(a1397, a1398)
                } else {
                    false
                } {
                    let a1400 = match v2.clone() {
                        Adt2::C0(_, _, _, h1399, _, _, _, _, _, _, _) => {
                            h1399
                        }
                    };
                    let a1401 = 1u64;
                    nat_eq(a1400, a1401)
                } else {
                    false
                } {
                    let a1403 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1402, _, _, _, _, _, _) => {
                            h1402
                        }
                    };
                    let a1404 = 1u64;
                    nat_eq(a1403, a1404)
                } else {
                    false
                }
            } else {
                let a1406 = match v2.clone() {
                    Adt2::C0(_, h1405, _, _, _, _, _, _, _, _, _) => {
                        h1405
                    }
                };
                let a1407 = 1u64;
                if if if nat_eq(a1406, a1407) {
                    let a1410 = match v1.clone() {
                        Adt2::C0(_, _, h1408, _, _, _, _, _, _, _, _) => {
                            h1408
                        }
                    };
                    let a1411 = match v2.clone() {
                        Adt2::C0(_, _, h1409, _, _, _, _, _, _, _, _) => {
                            h1409
                        }
                    };
                    nat_eq(a1410, a1411)
                } else {
                    false
                } {
                    let a1414 = match v1.clone() {
                        Adt2::C0(_, _, _, h1412, _, _, _, _, _, _, _) => {
                            h1412
                        }
                    };
                    let a1415 = match v2.clone() {
                        Adt2::C0(_, _, _, h1413, _, _, _, _, _, _, _) => {
                            h1413
                        }
                    };
                    nat_eq(a1414, a1415)
                } else {
                    false
                } {
                    let a1418 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1416, _, _, _, _, _, _) => {
                            h1416
                        }
                    };
                    let a1419 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1417, _, _, _, _, _, _) => {
                            h1417
                        }
                    };
                    nat_eq(a1418, a1419)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f19(v1, 1u64);
            Ok(Adt6::C4(f105(v4.clone(), v0, v3)?, {
                let a1420 = v3;
                let a1421 = 0u64;
                if nat_eq(a1420, a1421) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f19(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f54(v0: u64, v1: u64) -> Option<u64> {
    let a1422 = v0;
    let a1423 = 8u64;
    if nat_eq(a1422, a1423) {
        Some(5u64)
    } else {
        let a1424 = v0;
        let a1425 = 9u64;
        if nat_eq(a1424, a1425) {
            Some(6u64)
        } else {
            let a1426 = v0;
            let a1427 = 89u64;
            if nat_eq(a1426, a1427) {
                let a1428 = v1;
                let a1429 = 0u64;
                if nat_eq(a1428, a1429) {
                    Some(0u64)
                } else {
                    let a1430 = v1;
                    let a1431 = 1u64;
                    if nat_eq(a1430, a1431) {
                        Some(1u64)
                    } else {
                        let a1432 = v1;
                        let a1433 = 2u64;
                        if nat_eq(a1432, a1433) {
                            Some(2u64)
                        } else {
                            let a1434 = v1;
                            let a1435 = 3u64;
                            if nat_eq(a1434, a1435) {
                                Some(3u64)
                            } else {
                                let a1436 = v1;
                                let a1437 = 6u64;
                                if nat_eq(a1436, a1437) {
                                    Some(4u64)
                                } else {
                                    let a1438 = v1;
                                    let a1439 = 12u64;
                                    if nat_eq(a1438, a1439) {
                                        Some(7u64)
                                    } else {
                                        let a1440 = v1;
                                        let a1441 = 11u64;
                                        if nat_eq(a1440, a1441) {
                                            Some(8u64)
                                        } else {
                                            let a1442 = v1;
                                            let a1443 = 18u64;
                                            if nat_eq(a1442, a1443) {
                                                Some(9u64)
                                            } else {
                                                let a1444 = v1;
                                                let a1445 = 19u64;
                                                if nat_eq(a1444, a1445) {
                                                    Some(10u64)
                                                } else {
                                                    let a1446 = v1;
                                                    let a1447 = 20u64;
                                                    if nat_eq(a1446, a1447) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1448 = v1;
                                                        let a1449 = 21u64;
                                                        if nat_eq(a1448, a1449) {
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

pub fn f55(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1450 = f106(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1450 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f19(v2.clone(), 1u64), f19(v4, 1u64), Adt4::C10(v5, v6, {
                let m1451 = v3;
                match m1451 {
                    None => {
                        f19(v2, 1u64)
                    }
                    Some(v12) => {
                        f19(v12, 1u64)
                    }
                }
            })))
        }
        Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) => {
            Ok(Adt6::C3(v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
        }
        Adt6::C4(v23, v24, v25, v26, v27) => {
            Ok(Adt6::C4(v23, v24, v25, v26, v27))
        }
    }
}

pub fn f56(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1453 = match v1.clone() {
        Adt2::C0(h1452, _, _, _, _, _, _, _, _, _, _) => {
            h1452
        }
    };
    let a1454 = 0u64;
    if if if if if nat_eq(a1453, a1454) {
        true
    } else {
        let a1456 = match v1.clone() {
            Adt2::C0(h1455, _, _, _, _, _, _, _, _, _, _) => {
                h1455
            }
        };
        let a1457 = 1u64;
        nat_eq(a1456, a1457)
    } {
        true
    } else {
        let a1459 = match v1.clone() {
            Adt2::C0(h1458, _, _, _, _, _, _, _, _, _, _) => {
                h1458
            }
        };
        let a1460 = 30u64;
        nat_eq(a1459, a1460)
    } {
        let a1463 = match v1.clone() {
            Adt2::C0(h1461, _, _, _, _, _, _, _, _, _, _) => {
                h1461
            }
        };
        let a1464 = match v2.clone() {
            Adt2::C0(h1462, _, _, _, _, _, _, _, _, _, _) => {
                h1462
            }
        };
        if nat_eq(a1463, a1464) {
            true
        } else {
            let a1466 = match v2.clone() {
                Adt2::C0(h1465, _, _, _, _, _, _, _, _, _, _) => {
                    h1465
                }
            };
            let a1467 = 0u64;
            nat_eq(a1466, a1467)
        }
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f15(v2.clone(), 1u64)?, {
            let a1469 = match v2.clone() {
                Adt2::C0(h1468, _, _, _, _, _, _, _, _, _, _) => {
                    h1468
                }
            };
            let a1470 = 0u64;
            if nat_eq(a1469, a1470) {
                4u64
            } else {
                2u64
            }
        }, f19(v1.clone(), 1u64), f19(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1471, _, _, _, _, _, _, _, _, _, _) => {
                h1471
            }
        }, match v2 {
            Adt2::C0(h1472, _, _, _, _, _, _, _, _, _, _) => {
                h1472
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f57(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1474 = v0;
        let a1475 = match v1.clone() {
            Adt3::C0(h1473, _, _, _, _, _, _, _) => {
                h1473
            }
        };
        let a1476 = 0u64;
        nat_quot(a1474, a1475, a1476)
    };
    let v3: u64 = {
        let a1478 = v2;
        let a1479 = match v1.clone() {
            Adt3::C0(_, h1477, _, _, _, _, _, _) => {
                h1477
            }
        };
        let a1480 = 0u64;
        nat_quot(a1478, a1479, a1480)
    };
    f107({
        let a1482 = v0;
        let a1483 = match v1.clone() {
            Adt3::C0(h1481, _, _, _, _, _, _, _) => {
                h1481
            }
        };
        let a1484 = 0u64;
        nat_rem(a1482, a1483, a1484)
    }, {
        let a1486 = v2;
        let a1487 = match v1.clone() {
            Adt3::C0(_, h1485, _, _, _, _, _, _) => {
                h1485
            }
        };
        let a1488 = 0u64;
        nat_rem(a1486, a1487, a1488)
    }, {
        let a1490 = v3;
        let a1491 = match v1.clone() {
            Adt3::C0(_, _, h1489, _, _, _, _, _) => {
                h1489
            }
        };
        let a1492 = 0u64;
        nat_rem(a1490, a1491, a1492)
    }, {
        let a1494 = v3;
        let a1495 = match v1.clone() {
            Adt3::C0(_, _, h1493, _, _, _, _, _) => {
                h1493
            }
        };
        let a1496 = 0u64;
        nat_quot(a1494, a1495, a1496)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1497, _, _, _) => {
            h1497
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1498, _, _) => {
            h1498
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1499, _) => {
            h1499
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1500) => {
            h1500
        }
    })
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1518 = {
        let a1513 = f108(v2, {
            let a1504 = {
                let a1501 = v0;
                let a1502 = v1;
                let a1503 = 0u64;
                nat_quot(a1501, a1502, a1503)
            };
            let a1505 = v2;
            let a1506 = 0u64;
            nat_rem(a1504, a1505, a1506)
        }, {
            let a1510 = {
                let a1507 = v0;
                let a1508 = v1;
                let a1509 = 0u64;
                nat_quot(a1507, a1508, a1509)
            };
            let a1511 = v2;
            let a1512 = 0u64;
            nat_quot(a1510, a1511, a1512)
        })?;
        let a1514 = v1;
        nat_mul(a1513, a1514)?
    };
    let a1519 = {
        let a1515 = v0;
        let a1516 = v1;
        let a1517 = 0u64;
        nat_rem(a1515, a1516, a1517)
    };
    nat_add(a1518, a1519)
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1521 = v0;
        let a1522 = match v2.clone() {
            Adt3::C0(h1520, _, _, _, _, _, _, _) => {
                h1520
            }
        };
        let a1523 = 0u64;
        nat_quot(a1521, a1522, a1523)
    };
    let v4: u64 = {
        let a1525 = v3;
        let a1526 = match v2.clone() {
            Adt3::C0(_, h1524, _, _, _, _, _, _) => {
                h1524
            }
        };
        let a1527 = 0u64;
        nat_quot(a1525, a1526, a1527)
    };
    f109({
        let a1529 = v0;
        let a1530 = match v2.clone() {
            Adt3::C0(h1528, _, _, _, _, _, _, _) => {
                h1528
            }
        };
        let a1531 = 0u64;
        nat_rem(a1529, a1530, a1531)
    }, {
        let a1533 = v3;
        let a1534 = match v2.clone() {
            Adt3::C0(_, h1532, _, _, _, _, _, _) => {
                h1532
            }
        };
        let a1535 = 0u64;
        nat_rem(a1533, a1534, a1535)
    }, {
        let a1537 = v4;
        let a1538 = match v2.clone() {
            Adt3::C0(_, _, h1536, _, _, _, _, _) => {
                h1536
            }
        };
        let a1539 = 0u64;
        nat_rem(a1537, a1538, a1539)
    }, {
        let a1541 = v4;
        let a1542 = match v2 {
            Adt3::C0(_, _, h1540, _, _, _, _, _) => {
                h1540
            }
        };
        let a1543 = 0u64;
        nat_quot(a1541, a1542, a1543)
    }, match v1.clone() {
        Adt3::C0(h1544, _, _, _, _, _, _, _) => {
            h1544
        }
    }, match v1.clone() {
        Adt3::C0(_, h1545, _, _, _, _, _, _) => {
            h1545
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1546, _, _, _, _, _) => {
            h1546
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1547, _, _, _, _) => {
            h1547
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1548, _, _, _) => {
            h1548
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1549, _, _) => {
            h1549
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1550, _) => {
            h1550
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1551) => {
            h1551
        }
    })
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1553 = v0;
        let a1554 = match v3.clone() {
            Adt3::C0(h1552, _, _, _, _, _, _, _) => {
                h1552
            }
        };
        let a1555 = 0u64;
        nat_quot(a1553, a1554, a1555)
    };
    let v6: u64 = {
        let a1557 = v5;
        let a1558 = match v3.clone() {
            Adt3::C0(_, h1556, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1559 = 0u64;
        nat_quot(a1557, a1558, a1559)
    };
    let v7: u64 = {
        let a1561 = v0;
        let a1562 = match v3.clone() {
            Adt3::C0(h1560, _, _, _, _, _, _, _) => {
                h1560
            }
        };
        let a1563 = 0u64;
        nat_rem(a1561, a1562, a1563)
    };
    let v8: u64 = {
        let a1565 = v5;
        let a1566 = match v3.clone() {
            Adt3::C0(_, h1564, _, _, _, _, _, _) => {
                h1564
            }
        };
        let a1567 = 0u64;
        nat_rem(a1565, a1566, a1567)
    };
    let v9: u64 = {
        let a1569 = v6;
        let a1570 = match v3.clone() {
            Adt3::C0(_, _, h1568, _, _, _, _, _) => {
                h1568
            }
        };
        let a1571 = 0u64;
        nat_rem(a1569, a1570, a1571)
    };
    let v10: u64 = {
        let a1573 = v6;
        let a1574 = match v3.clone() {
            Adt3::C0(_, _, h1572, _, _, _, _, _) => {
                h1572
            }
        };
        let a1575 = 0u64;
        nat_quot(a1573, a1574, a1575)
    };
    let v11: u64 = {
        let a1576 = v4;
        let a1577 = 0u64;
        if nat_eq(a1576, a1577) {
            match v1.clone() {
                Adt3::C0(h1578, _, _, _, _, _, _, _) => {
                    h1578
                }
            }
        } else {
            let a1579 = v4;
            let a1580 = 1u64;
            if nat_eq(a1579, a1580) {
                match v1.clone() {
                    Adt3::C0(_, h1581, _, _, _, _, _, _) => {
                        h1581
                    }
                }
            } else {
                let a1582 = v4;
                let a1583 = 2u64;
                if nat_eq(a1582, a1583) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1584, _, _, _, _, _) => {
                            h1584
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1585, _, _, _, _) => {
                            h1585
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1592 = v11;
        let a1593 = {
            let a1586 = v4;
            let a1587 = 0u64;
            if nat_eq(a1586, a1587) {
                v7
            } else {
                let a1588 = v4;
                let a1589 = 1u64;
                if nat_eq(a1588, a1589) {
                    v8
                } else {
                    let a1590 = v4;
                    let a1591 = 2u64;
                    if nat_eq(a1590, a1591) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1592, a1593)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f107(if if v12 {
        let a1594 = v4;
        let a1595 = 0u64;
        nat_eq(a1594, a1595)
    } else {
        false
    } {
        let a1596 = v7;
        let a1597 = v11;
        nat_sub(a1596, a1597)
    } else {
        v7
    }, if if v12 {
        let a1598 = v4;
        let a1599 = 1u64;
        nat_eq(a1598, a1599)
    } else {
        false
    } {
        let a1600 = v8;
        let a1601 = v11;
        nat_sub(a1600, a1601)
    } else {
        v8
    }, if if v12 {
        let a1602 = v4;
        let a1603 = 2u64;
        nat_eq(a1602, a1603)
    } else {
        false
    } {
        let a1604 = v9;
        let a1605 = v11;
        nat_sub(a1604, a1605)
    } else {
        v9
    }, if if v12 {
        let a1606 = v4;
        let a1607 = 3u64;
        nat_eq(a1606, a1607)
    } else {
        false
    } {
        let a1608 = v10;
        let a1609 = v11;
        nat_sub(a1608, a1609)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1610, _, _, _) => {
            h1610
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1611, _, _) => {
            h1611
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1612, _) => {
            h1612
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1613) => {
            h1613
        }
    })?, f57(v0, v3)?))
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f110(v0, v2.clone(), 0u64);
    let v9: u64 = f110(v0, v2.clone(), 1u64);
    let v10: u64 = f110(v0, v2.clone(), 2u64);
    let v11: u64 = f110(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f111(v1.clone(), f112(v8, v3, match v1.clone() {
            Adt3::C0(h1614, _, _, _, _, _, _, _) => {
                h1614
            }
        })?, f112(v9, v4, match v1.clone() {
            Adt3::C0(_, h1615, _, _, _, _, _, _) => {
                h1615
            }
        })?, f112(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1616, _, _, _, _, _) => {
                h1616
            }
        })?, f112(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1617, _, _, _, _) => {
                h1617
            }
        })?)?, f57(v0, v2)?))
    } else {
        if if if if f113(v8, v3, match v1.clone() {
            Adt3::C0(h1618, _, _, _, _, _, _, _) => {
                h1618
            }
        })? {
            f113(v9, v4, match v1.clone() {
                Adt3::C0(_, h1619, _, _, _, _, _, _) => {
                    h1619
                }
            })?
        } else {
            false
        } {
            f113(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1620, _, _, _, _, _) => {
                    h1620
                }
            })?
        } else {
            false
        } {
            f113(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1621, _, _, _, _) => {
                    h1621
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f111(v1, f114(v8, v3)?, f114(v9, v4)?, f114(v10, v5)?, f114(v11, v6)?)?, f57(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f57(v0, v2)?))
        }
    }
}

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f110(v0, v2.clone(), 0u64);
    let v5: u64 = f110(v0, v2.clone(), 1u64);
    let v6: u64 = f110(v0, v2.clone(), 2u64);
    let v7: u64 = f110(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f111(v1.clone(), f115(v4, v3, match v1 {
        Adt3::C0(h1622, _, _, _, _, _, _, _) => {
            h1622
        }
    })?, v5, v6, v7)?, f57(v0, v2)?))
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1623 = v0;
    let a1624 = v3;
    if nat_lt(a1623, a1624) {
        Ok(Adt5::C1(false, {
            let a1625 = v0;
            let a1626 = 4u64;
            nat_mul(a1625, a1626)?
        }, {
            let a1627 = v0;
            let a1628 = 4u64;
            nat_mul(a1627, a1628)?
        }))
    } else {
        let v6: u64 = {
            let a1629 = v0;
            let a1630 = v3;
            nat_sub(a1629, a1630)
        };
        let v7: u64 = f57(v6, v1)?;
        let v8: u64 = f116(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1631 = v0;
    let a1632 = v3;
    if nat_lt(a1631, a1632) {
        Ok(Adt5::C1(true, 0u64, f57(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1633 = v0;
            let a1634 = v3;
            nat_sub(a1633, a1634)
        };
        let v5: u64 = f59(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f57(v4, v1)?, 3u64, v5))
    }
}

pub fn f65(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1636 = v1;
        let a1637 = match v3.clone() {
            Adt3::C0(h1635, _, _, _, _, _, _, _) => {
                h1635
            }
        };
        let a1638 = 0u64;
        nat_rem(a1636, a1637, a1638)
    };
    let v5: u64 = {
        let a1644 = {
            let a1640 = v1;
            let a1641 = match v3.clone() {
                Adt3::C0(h1639, _, _, _, _, _, _, _) => {
                    h1639
                }
            };
            let a1642 = 0u64;
            nat_quot(a1640, a1641, a1642)
        };
        let a1645 = match v3.clone() {
            Adt3::C0(_, h1643, _, _, _, _, _, _) => {
                h1643
            }
        };
        let a1646 = 0u64;
        nat_rem(a1644, a1645, a1646)
    };
    let v6: bool = f117(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f59(v1, v2, v3.clone())?
    } else {
        0u64
    }, f57(v1, v3)?))
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1647 = v3;
        let a1648 = 8u64;
        nat_add(a1647, a1648)?
    }, 0u64, f57(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f59(v0, v5, v2.clone())?
    }, 3u64, f57(v0, v2)?))
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1650 = v0;
        let a1651 = match v2.clone() {
            Adt3::C0(h1649, _, _, _, _, _, _, _) => {
                h1649
            }
        };
        let a1652 = 0u64;
        nat_rem(a1650, a1651, a1652)
    };
    let v7: u64 = {
        let a1654 = v0;
        let a1655 = match v2.clone() {
            Adt3::C0(h1653, _, _, _, _, _, _, _) => {
                h1653
            }
        };
        let a1656 = 0u64;
        nat_quot(a1654, a1655, a1656)
    };
    let v8: u64 = {
        let a1658 = v7;
        let a1659 = match v2.clone() {
            Adt3::C0(_, h1657, _, _, _, _, _, _) => {
                h1657
            }
        };
        let a1660 = 0u64;
        nat_rem(a1658, a1659, a1660)
    };
    let v9: u64 = {
        let a1662 = v7;
        let a1663 = match v2.clone() {
            Adt3::C0(_, h1661, _, _, _, _, _, _) => {
                h1661
            }
        };
        let a1664 = 0u64;
        nat_quot(a1662, a1663, a1664)
    };
    let v10: u64 = {
        let a1666 = v9;
        let a1667 = match v2.clone() {
            Adt3::C0(_, _, h1665, _, _, _, _, _) => {
                h1665
            }
        };
        let a1668 = 0u64;
        nat_rem(a1666, a1667, a1668)
    };
    let v11: u64 = {
        let a1670 = v9;
        let a1671 = match v2.clone() {
            Adt3::C0(_, _, h1669, _, _, _, _, _) => {
                h1669
            }
        };
        let a1672 = 0u64;
        nat_quot(a1670, a1671, a1672)
    };
    let v12: u64 = f57(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1681 = {
                let a1676 = {
                    let a1673 = v10;
                    let a1674 = v4;
                    let a1675 = 0u64;
                    nat_quot(a1673, a1674, a1675)
                };
                let a1677 = v5;
                nat_mul(a1676, a1677)?
            };
            let a1682 = {
                let a1678 = v8;
                let a1679 = v4;
                let a1680 = 0u64;
                nat_quot(a1678, a1679, a1680)
            };
            nat_add(a1681, a1682)?
        };
        let v14: u64 = {
            let a1702 = v6;
            let a1703 = {
                let a1700 = match v1.clone() {
                    Adt3::C0(h1683, _, _, _, _, _, _, _) => {
                        h1683
                    }
                };
                let a1701 = {
                    let a1698 = {
                        let a1684 = v8;
                        let a1685 = v4;
                        let a1686 = 0u64;
                        nat_rem(a1684, a1685, a1686)
                    };
                    let a1699 = {
                        let a1696 = match v1.clone() {
                            Adt3::C0(_, h1687, _, _, _, _, _, _) => {
                                h1687
                            }
                        };
                        let a1697 = {
                            let a1694 = {
                                let a1688 = v10;
                                let a1689 = v4;
                                let a1690 = 0u64;
                                nat_rem(a1688, a1689, a1690)
                            };
                            let a1695 = {
                                let a1692 = match v1.clone() {
                                    Adt3::C0(_, _, h1691, _, _, _, _, _) => {
                                        h1691
                                    }
                                };
                                let a1693 = v13;
                                nat_mul(a1692, a1693)?
                            };
                            nat_add(a1694, a1695)?
                        };
                        nat_mul(a1696, a1697)?
                    };
                    nat_add(a1698, a1699)?
                };
                nat_mul(a1700, a1701)?
            };
            nat_add(a1702, a1703)?
        };
        Ok(Adt5::C1(false, f57(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1709 = {
                let a1707 = {
                    let a1704 = v11;
                    let a1705 = v5;
                    let a1706 = 0u64;
                    nat_rem(a1704, a1705, a1706)
                };
                let a1708 = v4;
                nat_mul(a1707, a1708)?
            };
            let a1710 = v8;
            nat_add(a1709, a1710)?
        };
        let v16: u64 = {
            let a1716 = {
                let a1714 = {
                    let a1711 = v11;
                    let a1712 = v5;
                    let a1713 = 0u64;
                    nat_quot(a1711, a1712, a1713)
                };
                let a1715 = v4;
                nat_mul(a1714, a1715)?
            };
            let a1717 = v10;
            nat_add(a1716, a1717)?
        };
        let a1719 = v15;
        let a1720 = match v1.clone() {
            Adt3::C0(_, h1718, _, _, _, _, _, _) => {
                h1718
            }
        };
        if if nat_lt(a1719, a1720) {
            let a1722 = v16;
            let a1723 = match v1.clone() {
                Adt3::C0(_, _, h1721, _, _, _, _, _) => {
                    h1721
                }
            };
            nat_lt(a1722, a1723)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f57({
                let a1732 = v6;
                let a1733 = {
                    let a1730 = match v1.clone() {
                        Adt3::C0(h1724, _, _, _, _, _, _, _) => {
                            h1724
                        }
                    };
                    let a1731 = {
                        let a1728 = v15;
                        let a1729 = {
                            let a1726 = match v1.clone() {
                                Adt3::C0(_, h1725, _, _, _, _, _, _) => {
                                    h1725
                                }
                            };
                            let a1727 = v16;
                            nat_mul(a1726, a1727)?
                        };
                        nat_add(a1728, a1729)?
                    };
                    nat_mul(a1730, a1731)?
                };
                nat_add(a1732, a1733)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f57(v0, v2.clone())?;
    let v4: u64 = f57(v0, v1.clone())?;
    let a1738 = {
        let a1735 = v0;
        let a1736 = match v1 {
            Adt3::C0(h1734, _, _, _, _, _, _, _) => {
                h1734
            }
        };
        let a1737 = 0u64;
        nat_rem(a1735, a1736, a1737)
    };
    let a1739 = 0u64;
    if nat_eq(a1738, a1739) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f57({
            let a1740 = v0;
            let a1741 = 1u64;
            nat_sub(a1740, a1741)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1742 = v3;
    let a1743 = 13u64;
    if nat_eq(a1742, a1743) {
        f118(v0, v1, v4)
    } else {
        let v5: u64 = f119(match v1.clone() {
            Adt3::C0(h1744, _, _, _, _, _, _, _) => {
                h1744
            }
        }, {
            let a1745 = v3;
            let a1746 = 16u64;
            nat_eq(a1745, a1746)
        })?;
        let v6: u64 = {
            let a1747 = v0;
            let a1748 = v5;
            let a1749 = 0u64;
            nat_quot(a1747, a1748, a1749)
        };
        let v7: u64 = {
            let a1750 = v0;
            let a1751 = v5;
            let a1752 = 0u64;
            nat_rem(a1750, a1751, a1752)
        };
        let v8: u64 = {
            let a1756 = 1u64;
            let a1757 = {
                let a1754 = match v1.clone() {
                    Adt3::C0(h1753, _, _, _, _, _, _, _) => {
                        h1753
                    }
                };
                let a1755 = 2u64;
                nat_mul(a1754, a1755)?
            };
            nat_add(a1756, a1757)?
        };
        let a1758 = v7;
        let a1759 = v8;
        if nat_lt(a1758, a1759) {
            f120(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f57(v6, v2)?;
            let a1760 = v3;
            let a1761 = 14u64;
            if nat_eq(a1760, a1761) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1762 = v7;
                let a1763 = v8;
                if nat_eq(a1762, a1763) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1766 = v7;
                    let a1767 = {
                        let a1764 = v8;
                        let a1765 = 1u64;
                        nat_add(a1764, a1765)?
                    };
                    if nat_eq(a1766, a1767) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1768, _, _, _, _, _, _, _) => {
                                h1768
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

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1769 = v0;
    let a1770 = 0u64;
    if nat_eq(a1769, a1770) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1777 = {
            let a1771 = v0;
            let a1772 = 1u64;
            nat_add(a1771, a1772)?
        };
        let a1778 = {
            let a1775 = 2u64;
            let a1776 = {
                let a1773 = v3;
                let a1774 = 2u64;
                nat_mul(a1773, a1774)?
            };
            nat_add(a1775, a1776)?
        };
        if nat_eq(a1777, a1778) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1779 = v0;
                let a1780 = 1u64;
                nat_sub(a1779, a1780)
            };
            let v5: u64 = {
                let a1781 = v4;
                let a1782 = 2u64;
                let a1783 = 0u64;
                nat_quot(a1781, a1782, a1783)
            };
            let a1787 = {
                let a1784 = v4;
                let a1785 = 2u64;
                let a1786 = 0u64;
                nat_rem(a1784, a1785, a1786)
            };
            let a1788 = 0u64;
            if nat_eq(a1787, a1788) {
                Ok(Adt5::C4(17u64, 0u64, f57(v5, v1)?, 4u64, f57(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt5> {
    let a1789 = v0;
    let a1790 = f24(v1, v2, v3)?;
    if nat_lt(a1789, a1790) {
        let a1791 = v0;
        let a1792 = v1;
        if nat_lt(a1791, a1792) {
            Ok(Adt5::C1(false, {
                let a1793 = 4u64;
                let a1794 = v0;
                nat_mul(a1793, a1794)?
            }, {
                let a1795 = 4u64;
                let a1796 = v0;
                nat_mul(a1795, a1796)?
            }))
        } else {
            let v4: u64 = {
                let a1797 = v0;
                let a1798 = v1;
                nat_sub(a1797, a1798)
            };
            let v5: u64 = f121(v4, v2)?;
            let a1799 = v5;
            let a1800 = 2u64;
            if nat_lt(a1799, a1800) {
                Ok(Adt5::C4(6u64, {
                    let a1801 = v5;
                    let a1802 = 0u64;
                    if nat_eq(a1801, a1802) {
                        4u64
                    } else {
                        5u64
                    }
                }, f122(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f123(v4, v2)?;
                Ok(Adt5::C4(8u64, 3u64, v6, 1u64, if f124(v4, v2)? {
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

pub fn f72(v0: u64) -> R<u64> {
    let a1805 = 3u64;
    let a1806 = {
        let a1803 = v0;
        let a1804 = 3u64;
        nat_mul(a1803, a1804)?
    };
    nat_add(a1805, a1806)
}

pub fn f73(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1814 = {
        let a1807 = 2u64;
        let a1808 = f74(v1.clone())?;
        nat_mul(a1807, a1808)?
    };
    let a1815 = {
        let a1812 = f74(v1)?;
        let a1813 = {
            let a1810 = 1u64;
            let a1811 = match v0 {
                Adt3::C0(h1809, _, _, _, _, _, _, _) => {
                    h1809
                }
            };
            nat_add(a1810, a1811)?
        };
        nat_mul(a1812, a1813)?
    };
    nat_add(a1814, a1815)
}

pub fn f74(v0: Adt3) -> R<u64> {
    if f29(match v0.clone() {
        Adt3::C0(h1816, _, _, _, _, _, _, _) => {
            h1816
        }
    }, match v0.clone() {
        Adt3::C0(_, h1817, _, _, _, _, _, _) => {
            h1817
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1818, _, _, _, _, _) => {
            h1818
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1819, _, _, _, _) => {
            h1819
        }
    }) {
        Ok(0u64)
    } else {
        let a1828 = match v0.clone() {
            Adt3::C0(h1820, _, _, _, _, _, _, _) => {
                h1820
            }
        };
        let a1829 = {
            let a1826 = match v0.clone() {
                Adt3::C0(_, h1821, _, _, _, _, _, _) => {
                    h1821
                }
            };
            let a1827 = {
                let a1824 = match v0.clone() {
                    Adt3::C0(_, _, h1822, _, _, _, _, _) => {
                        h1822
                    }
                };
                let a1825 = match v0 {
                    Adt3::C0(_, _, _, h1823, _, _, _, _) => {
                        h1823
                    }
                };
                nat_mul(a1824, a1825)?
            };
            nat_mul(a1826, a1827)?
        };
        nat_mul(a1828, a1829)
    }
}

pub fn f75(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f57(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f76(v0: u64, v1: u64) -> bool {
    let a1830 = v1;
    let a1831 = 2147483647u64;
    if nat_le(a1830, a1831) {
        let a1832 = v1;
        let a1833 = v0;
        nat_lt(a1832, a1833)
    } else {
        false
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1834 = f95(v0);
    match m1834 {
        Adt10::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt10::C1(v7) => {
            let v8: u64 = {
                let a1836 = v2;
                let a1837 = match v7.clone() {
                    Adt11::C0(_, h1835, _) => {
                        h1835
                    }
                };
                nat_mul(a1836, a1837)?
            };
            let m1850 = f125(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1838, _, _, _, _, _, _, _) => {
                    h1838
                }
            }, match v5.clone() {
                Adt3::C0(_, h1839, _, _, _, _, _, _) => {
                    h1839
                }
            }, match v5 {
                Adt3::C0(_, _, h1840, _, _, _, _, _) => {
                    h1840
                }
            }, {
                let a1843 = match v3.clone() {
                    Adt3::C0(h1841, _, _, _, _, _, _, _) => {
                        h1841
                    }
                };
                let a1844 = match v7.clone() {
                    Adt11::C0(_, h1842, _) => {
                        h1842
                    }
                };
                nat_mul(a1843, a1844)?
            }, match v3.clone() {
                Adt3::C0(_, h1845, _, _, _, _, _, _) => {
                    h1845
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1846, _, _, _) => {
                    h1846
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1847, _, _) => {
                    h1847
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1848, _) => {
                    h1848
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1849) => {
                    h1849
                }
            })?;
            match m1850 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1851 = v0;
                    let a1852 = 0u64;
                    if if nat_eq(a1851, a1852) {
                        true
                    } else {
                        let a1853 = v0;
                        let a1854 = 26u64;
                        nat_eq(a1853, a1854)
                    } {
                        Ok(Adt5::C1(false, v9, f57({
                            let a1858 = {
                                let a1856 = v1;
                                let a1857 = match v4.clone() {
                                    Adt3::C0(h1855, _, _, _, _, _, _, _) => {
                                        h1855
                                    }
                                };
                                nat_mul(a1856, a1857)?
                            };
                            let a1859 = v8;
                            nat_add(a1858, a1859)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt11::C0(_, _, h1860) => {
                                h1860
                            }
                        }, f57({
                            let a1864 = {
                                let a1862 = v1;
                                let a1863 = match v4.clone() {
                                    Adt3::C0(h1861, _, _, _, _, _, _, _) => {
                                        h1861
                                    }
                                };
                                nat_mul(a1862, a1863)?
                            };
                            let a1865 = v8;
                            nat_add(a1864, a1865)?
                        }, v4.clone())?, {
                            let a1872 = {
                                let a1870 = {
                                    let a1867 = match v7 {
                                        Adt11::C0(_, h1866, _) => {
                                            h1866
                                        }
                                    };
                                    let a1868 = 1u64;
                                    nat_sub(a1867, a1868)
                                };
                                let a1871 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1869, _, _, _) => {
                                        h1869
                                    }
                                };
                                nat_mul(a1870, a1871)?
                            };
                            let a1873 = 4u64;
                            nat_add(a1872, a1873)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1874, _, _, _) => {
                                h1874
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1875 = v1;
    let a1876 = 0u64;
    if if nat_eq(a1875, a1876) {
        true
    } else {
        let a1877 = v2;
        let a1878 = 0u64;
        nat_eq(a1877, a1878)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f79(v0: u64, v1: u64) -> bool {
    let a1879 = v0;
    let a1880 = 4294967295u64;
    if nat_eq(a1879, a1880) {
        true
    } else {
        let a1881 = v0;
        let a1882 = 2147483648u64;
        if nat_lt(a1881, a1882) {
            let a1883 = v0;
            let a1884 = v1;
            nat_lt(a1883, a1884)
        } else {
            false
        }
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1885 = v0;
        let a1886 = 8388608u64;
        let a1887 = 0u64;
        nat_quot(a1885, a1886, a1887)
    };
    let a1888 = v0;
    let a1889 = 2147483648u64;
    if if if nat_lt(a1888, a1889) {
        let a1890 = 127u64;
        let a1891 = v1;
        nat_le(a1890, a1891)
    } else {
        false
    } {
        let a1892 = v1;
        let a1893 = 190u64;
        nat_lt(a1892, a1893)
    } else {
        false
    } {
        let v2: u64 = {
            let a1897 = 8388608u64;
            let a1898 = {
                let a1894 = v0;
                let a1895 = 8388608u64;
                let a1896 = 0u64;
                nat_rem(a1894, a1895, a1896)
            };
            nat_add(a1897, a1898)?
        };
        let a1899 = v1;
        let a1900 = 150u64;
        if nat_lt(a1899, a1900) {
            let v3: u64 = f126({
                let a1901 = 150u64;
                let a1902 = v1;
                nat_sub(a1901, a1902)
            });
            let a1906 = {
                let a1903 = v2;
                let a1904 = v3;
                let a1905 = 0u64;
                nat_rem(a1903, a1904, a1905)
            };
            let a1907 = 0u64;
            if nat_eq(a1906, a1907) {
                let a1908 = v2;
                let a1909 = v3;
                let a1910 = 0u64;
                Ok(nat_quot(a1908, a1909, a1910))
            } else {
                Ok(0u64)
            }
        } else {
            let a1913 = f126({
                let a1911 = v1;
                let a1912 = 150u64;
                nat_sub(a1911, a1912)
            });
            let a1914 = v2;
            nat_mul(a1913, a1914)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f81(v0: Adt4, v1: Adt5) -> Adt5 {
    let m1915 = v1;
    match m1915 {
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
            f127(f128(v0.clone(), v12, v13), v12, v13, 0u64, v14, 2u64, f129(v0), 3u64, v15)
        }
        Adt5::C4(v16, v17, v18, v19, v20, v21, v22) => {
            let v23: u64 = f130(v0.clone(), v16);
            f127(f131(v0, v16), v23, v23, v17, v18, v19, v20, v21, v22)
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

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt5> {
    let v8: u64 = {
        let a1918 = v6;
        let a1919 = {
            let a1916 = 3u64;
            let a1917 = v5;
            nat_add(a1916, a1917)?
        };
        nat_sub(a1918, a1919)
    };
    let a1921 = v0;
    let a1922 = f31(v3, match v1.clone() {
        Adt3::C0(h1920, _, _, _, _, _, _, _) => {
            h1920
        }
    }, v5, v8)?;
    if nat_lt(a1921, a1922) {
        let v9: u64 = f88(match v1.clone() {
            Adt3::C0(h1923, _, _, _, _, _, _, _) => {
                h1923
            }
        }, v5, v8)?;
        let v10: u64 = f89(v3, match v1.clone() {
            Adt3::C0(h1924, _, _, _, _, _, _, _) => {
                h1924
            }
        }, v5)?;
        let a1925 = v0;
        let a1926 = v9;
        if nat_lt(a1925, a1926) {
            f132(v0, v2, v3, v5, v7)
        } else {
            let a1929 = v0;
            let a1930 = {
                let a1927 = v9;
                let a1928 = v10;
                nat_add(a1927, a1928)?
            };
            if nat_lt(a1929, a1930) {
                let v11: u64 = {
                    let a1931 = v0;
                    let a1932 = v9;
                    nat_sub(a1931, a1932)
                };
                let v12: u64 = f133(v3, match v1.clone() {
                    Adt3::C0(h1933, _, _, _, _, _, _, _) => {
                        h1933
                    }
                })?;
                f134({
                    let a1934 = v11;
                    let a1935 = v12;
                    let a1936 = 0u64;
                    nat_rem(a1934, a1935, a1936)
                }, {
                    let a1937 = v11;
                    let a1938 = v12;
                    let a1939 = 0u64;
                    nat_quot(a1937, a1938, a1939)
                }, v1, v3, v4, v7)
            } else {
                f135({
                    let a1942 = v0;
                    let a1943 = {
                        let a1940 = v9;
                        let a1941 = v10;
                        nat_add(a1940, a1941)?
                    };
                    nat_sub(a1942, a1943)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = f91(match v1.clone() {
        Adt3::C0(h1944, _, _, _, _, _, _, _) => {
            h1944
        }
    })?;
    let v6: u64 = {
        let a1950 = match v1.clone() {
            Adt3::C0(_, h1945, _, _, _, _, _, _) => {
                h1945
            }
        };
        let a1951 = {
            let a1948 = match v1.clone() {
                Adt3::C0(_, _, h1946, _, _, _, _, _) => {
                    h1946
                }
            };
            let a1949 = match v1.clone() {
                Adt3::C0(_, _, _, h1947, _, _, _, _) => {
                    h1947
                }
            };
            nat_mul(a1948, a1949)?
        };
        nat_mul(a1950, a1951)?
    };
    let a1953 = v0;
    let a1954 = f34(match v1.clone() {
        Adt3::C0(h1952, _, _, _, _, _, _, _) => {
            h1952
        }
    }, v6)?;
    if nat_lt(a1953, a1954) {
        f136({
            let a1955 = v0;
            let a1956 = v5;
            let a1957 = 0u64;
            nat_rem(a1955, a1956, a1957)
        }, {
            let a1958 = v0;
            let a1959 = v5;
            let a1960 = 0u64;
            nat_quot(a1958, a1959, a1960)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f84(v0: u64) -> u64 {
    let v1: u64 = {
        let a1961 = v0;
        let a1962 = 4294967296u64;
        let a1963 = 0u64;
        nat_rem(a1961, a1962, a1963)
    };
    let a1964 = v1;
    let a1965 = 2147483648u64;
    if nat_lt(a1964, a1965) {
        v1
    } else {
        0u64
    }
}

pub fn f85(v0: u64) -> R<u64> {
    let a1968 = 2u64;
    let a1969 = {
        let a1966 = 2u64;
        let a1967 = v0;
        nat_mul(a1966, a1967)?
    };
    nat_add(a1968, a1969)
}

pub fn f86(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f137(v0, v3, v6) {
        f137(v1, v4, v6)
    } else {
        false
    } {
        f137(v2, v5, v6)
    } else {
        false
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f138(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f138(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f138(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f138(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f138(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f138(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1970 = v2;
    let a1971 = 0u64;
    if nat_eq(a1970, a1971) {
        Ok(0u64)
    } else {
        let a1974 = 2u64;
        let a1975 = {
            let a1972 = v0;
            let a1973 = v1;
            nat_mul(a1972, a1973)?
        };
        nat_mul(a1974, a1975)
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1976 = v2;
    let a1977 = f133(v0, v1)?;
    nat_mul(a1976, a1977)
}

pub fn f90(v0: u64) -> u64 {
    let a1978 = v0;
    let a1979 = 30u64;
    if nat_eq(a1978, a1979) {
        3u64
    } else {
        1u64
    }
}

pub fn f91(v0: u64) -> R<u64> {
    let a1982 = 5u64;
    let a1983 = {
        let a1980 = 4u64;
        let a1981 = v0;
        nat_mul(a1980, a1981)?
    };
    nat_add(a1982, a1983)
}

pub fn f92(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f139(match v0.clone() {
        Adt2::C0(h1984, _, _, _, _, _, _, _, _, _, _) => {
            h1984
        }
    }, match v0.clone() {
        Adt2::C0(_, h1985, _, _, _, _, _, _, _, _, _) => {
            h1985
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1986, _, _, _, _, _, _, _, _) => {
            h1986
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1987, _, _, _, _, _, _, _) => {
            h1987
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1988, _, _, _, _, _, _) => {
            h1988
        }
    }, match v1.clone() {
        Adt2::C0(h1989, _, _, _, _, _, _, _, _, _, _) => {
            h1989
        }
    }, match v1.clone() {
        Adt2::C0(_, h1990, _, _, _, _, _, _, _, _, _) => {
            h1990
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1991, _, _, _, _, _, _, _, _) => {
            h1991
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1992, _, _, _, _, _, _, _) => {
            h1992
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1993, _, _, _, _, _, _) => {
            h1993
        }
    }, match v2.clone() {
        Adt2::C0(h1994, _, _, _, _, _, _, _, _, _, _) => {
            h1994
        }
    }, match v2.clone() {
        Adt2::C0(_, h1995, _, _, _, _, _, _, _, _, _) => {
            h1995
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1996, _, _, _, _, _, _, _, _) => {
            h1996
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1997, _, _, _, _, _, _, _) => {
            h1997
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1998, _, _, _, _, _, _) => {
            h1998
        }
    }) {
        let m2000 = f95(match v0.clone() {
            Adt2::C0(h1999, _, _, _, _, _, _, _, _, _, _) => {
                h1999
            }
        });
        match m2000 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v3) => {
                let a2006 = {
                    let a2003 = match v0.clone() {
                        Adt2::C0(_, h2001, _, _, _, _, _, _, _, _, _) => {
                            h2001
                        }
                    };
                    let a2004 = match v3.clone() {
                        Adt11::C0(_, h2002, _) => {
                            h2002
                        }
                    };
                    let a2005 = 0u64;
                    nat_rem(a2003, a2004, a2005)
                };
                let a2007 = 0u64;
                if if nat_eq(a2006, a2007) {
                    if f140(match v0.clone() {
                        Adt2::C0(_, h2008, _, _, _, _, _, _, _, _, _) => {
                            h2008
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2009, _, _, _, _, _, _, _, _, _) => {
                            h2009
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2010, _, _, _, _, _, _, _, _) => {
                            h2010
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2011, _, _, _, _, _, _, _) => {
                            h2011
                        }
                    }) {
                        true
                    } else {
                        let a2013 = 4u64;
                        let a2014 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2012, _, _, _, _, _) => {
                                h2012
                            }
                        };
                        if if nat_le(a2013, a2014) {
                            let a2016 = match v3.clone() {
                                Adt11::C0(_, h2015, _) => {
                                    h2015
                                }
                            };
                            let a2017 = 1u64;
                            if nat_eq(a2016, a2017) {
                                true
                            } else {
                                let a2020 = match v3.clone() {
                                    Adt11::C0(_, _, h2018) => {
                                        h2018
                                    }
                                };
                                let a2021 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2019, _, _, _, _, _) => {
                                        h2019
                                    }
                                };
                                nat_le(a2020, a2021)
                            }
                        } else {
                            false
                        } {
                            if f22(v0.clone(), v2.clone())? {
                                f22(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f19(v0.clone(), match v3 {
                        Adt11::C0(_, h2022, _) => {
                            h2022
                        }
                    });
                    Ok(Adt6::C4(if f140(match v0.clone() {
                        Adt2::C0(_, h2023, _, _, _, _, _, _, _, _, _) => {
                            h2023
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2024, _, _, _, _, _, _, _, _, _) => {
                            h2024
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2025, _, _, _, _, _, _, _, _) => {
                            h2025
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2026, _, _, _, _, _, _, _) => {
                            h2026
                        }
                    }) {
                        0u64
                    } else {
                        f73(v4.clone(), f19(v1.clone(), 1u64))?
                    }, 4u64, v4, f19(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2027, _, _, _, _, _, _, _, _, _, _) => {
                            h2027
                        }
                    }, f19(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f93(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a2030 = match v1.clone() {
        Adt2::C0(h2028, _, _, _, _, _, _, _, _, _, _) => {
            h2028
        }
    };
    let a2031 = match v3.clone() {
        Adt2::C0(h2029, _, _, _, _, _, _, _, _, _, _) => {
            h2029
        }
    };
    if if nat_eq(a2030, a2031) {
        let a2032 = v0;
        let a2033 = 33u64;
        if nat_eq(a2032, a2033) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2035 = f95(match v1.clone() {
            Adt2::C0(h2034, _, _, _, _, _, _, _, _, _, _) => {
                h2034
            }
        });
        match m2035 {
            Adt10::C0 => {
                Ok(Adt6::C0)
            }
            Adt10::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt11::C0(_, h2036, _) => {
                        h2036
                    }
                };
                let v6: u64 = f15(v1.clone(), v5)?;
                let a2041 = {
                    let a2038 = match v1.clone() {
                        Adt2::C0(_, h2037, _, _, _, _, _, _, _, _, _) => {
                            h2037
                        }
                    };
                    let a2039 = v5;
                    let a2040 = 1u64;
                    nat_rem(a2038, a2039, a2040)
                };
                let a2042 = 0u64;
                if if if nat_eq(a2041, a2042) {
                    let a2047 = {
                        let a2044 = match v3.clone() {
                            Adt2::C0(_, h2043, _, _, _, _, _, _, _, _, _) => {
                                h2043
                            }
                        };
                        let a2045 = v5;
                        let a2046 = 1u64;
                        nat_rem(a2044, a2045, a2046)
                    };
                    let a2048 = 0u64;
                    nat_eq(a2047, a2048)
                } else {
                    false
                } {
                    let a2049 = v6;
                    let a2050 = f15(v3.clone(), v5)?;
                    nat_eq(a2049, a2050)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt11::C0(_, _, h2051) => {
                            h2051
                        }
                    }, f19(v1, v5), f19(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f94(v0: u64) -> bool {
    let a2052 = v0;
    let a2053 = 0u64;
    if if nat_eq(a2052, a2053) {
        true
    } else {
        let a2054 = v0;
        let a2055 = 1u64;
        nat_eq(a2054, a2055)
    } {
        true
    } else {
        let a2056 = v0;
        let a2057 = 30u64;
        nat_eq(a2056, a2057)
    }
}

pub fn f95(v0: u64) -> Adt10 {
    let a2058 = v0;
    let a2059 = 0u64;
    if nat_eq(a2058, a2059) {
        Adt10::C1(Adt11::C0(0u64, 1u64, 4u64))
    } else {
        let a2060 = v0;
        let a2061 = 1u64;
        if nat_eq(a2060, a2061) {
            Adt10::C1(Adt11::C0(1u64, 1u64, 2u64))
        } else {
            let a2062 = v0;
            let a2063 = 2u64;
            if nat_eq(a2062, a2063) {
                Adt10::C1(Adt11::C0(2u64, 32u64, 18u64))
            } else {
                let a2064 = v0;
                let a2065 = 3u64;
                if nat_eq(a2064, a2065) {
                    Adt10::C1(Adt11::C0(3u64, 32u64, 20u64))
                } else {
                    let a2066 = v0;
                    let a2067 = 6u64;
                    if nat_eq(a2066, a2067) {
                        Adt10::C1(Adt11::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2068 = v0;
                        let a2069 = 7u64;
                        if nat_eq(a2068, a2069) {
                            Adt10::C1(Adt11::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2070 = v0;
                            let a2071 = 8u64;
                            if nat_eq(a2070, a2071) {
                                Adt10::C1(Adt11::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2072 = v0;
                                let a2073 = 9u64;
                                if nat_eq(a2072, a2073) {
                                    Adt10::C1(Adt11::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2074 = v0;
                                    let a2075 = 10u64;
                                    if nat_eq(a2074, a2075) {
                                        Adt10::C1(Adt11::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2076 = v0;
                                        let a2077 = 11u64;
                                        if nat_eq(a2076, a2077) {
                                            Adt10::C1(Adt11::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2078 = v0;
                                            let a2079 = 12u64;
                                            if nat_eq(a2078, a2079) {
                                                Adt10::C1(Adt11::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2080 = v0;
                                                let a2081 = 13u64;
                                                if nat_eq(a2080, a2081) {
                                                    Adt10::C1(Adt11::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2082 = v0;
                                                    let a2083 = 14u64;
                                                    if nat_eq(a2082, a2083) {
                                                        Adt10::C1(Adt11::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2084 = v0;
                                                        let a2085 = 15u64;
                                                        if nat_eq(a2084, a2085) {
                                                            Adt10::C1(Adt11::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2086 = v0;
                                                            let a2087 = 16u64;
                                                            if nat_eq(a2086, a2087) {
                                                                Adt10::C1(Adt11::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2088 = v0;
                                                                let a2089 = 17u64;
                                                                if nat_eq(a2088, a2089) {
                                                                    Adt10::C1(Adt11::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2090 = v0;
                                                                    let a2091 = 18u64;
                                                                    if nat_eq(a2090, a2091) {
                                                                        Adt10::C1(Adt11::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2092 = v0;
                                                                        let a2093 = 19u64;
                                                                        if nat_eq(a2092, a2093) {
                                                                            Adt10::C1(Adt11::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2094 = v0;
                                                                            let a2095 = 20u64;
                                                                            if nat_eq(a2094, a2095) {
                                                                                Adt10::C1(Adt11::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2096 = v0;
                                                                                let a2097 = 21u64;
                                                                                if nat_eq(a2096, a2097) {
                                                                                    Adt10::C1(Adt11::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2098 = v0;
                                                                                    let a2099 = 22u64;
                                                                                    if nat_eq(a2098, a2099) {
                                                                                        Adt10::C1(Adt11::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2100 = v0;
                                                                                        let a2101 = 23u64;
                                                                                        if nat_eq(a2100, a2101) {
                                                                                            Adt10::C1(Adt11::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2102 = v0;
                                                                                            let a2103 = 24u64;
                                                                                            if nat_eq(a2102, a2103) {
                                                                                                Adt10::C1(Adt11::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2104 = v0;
                                                                                                let a2105 = 25u64;
                                                                                                if nat_eq(a2104, a2105) {
                                                                                                    Adt10::C1(Adt11::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2106 = v0;
                                                                                                    let a2107 = 26u64;
                                                                                                    if nat_eq(a2106, a2107) {
                                                                                                        Adt10::C1(Adt11::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2108 = v0;
                                                                                                        let a2109 = 27u64;
                                                                                                        if nat_eq(a2108, a2109) {
                                                                                                            Adt10::C1(Adt11::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2110 = v0;
                                                                                                            let a2111 = 28u64;
                                                                                                            if nat_eq(a2110, a2111) {
                                                                                                                Adt10::C1(Adt11::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2112 = v0;
                                                                                                                let a2113 = 29u64;
                                                                                                                if nat_eq(a2112, a2113) {
                                                                                                                    Adt10::C1(Adt11::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2114 = v0;
                                                                                                                    let a2115 = 30u64;
                                                                                                                    if nat_eq(a2114, a2115) {
                                                                                                                        Adt10::C1(Adt11::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2116 = v0;
                                                                                                                        let a2117 = 34u64;
                                                                                                                        if nat_eq(a2116, a2117) {
                                                                                                                            Adt10::C1(Adt11::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2118 = v0;
                                                                                                                            let a2119 = 35u64;
                                                                                                                            if nat_eq(a2118, a2119) {
                                                                                                                                Adt10::C1(Adt11::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2120 = v0;
                                                                                                                                let a2121 = 39u64;
                                                                                                                                if nat_eq(a2120, a2121) {
                                                                                                                                    Adt10::C1(Adt11::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2122 = v0;
                                                                                                                                    let a2123 = 40u64;
                                                                                                                                    if nat_eq(a2122, a2123) {
                                                                                                                                        Adt10::C1(Adt11::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2124 = v0;
                                                                                                                                        let a2125 = 41u64;
                                                                                                                                        if nat_eq(a2124, a2125) {
                                                                                                                                            Adt10::C1(Adt11::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2126 = v0;
                                                                                                                                            let a2127 = 42u64;
                                                                                                                                            if nat_eq(a2126, a2127) {
                                                                                                                                                Adt10::C1(Adt11::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2128 = v0;
                                                                                                                                                let a2129 = 43u64;
                                                                                                                                                if nat_eq(a2128, a2129) {
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

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f29(v4, v5, v6, v7) {
        f29(v0, v1, v2, v3)
    } else {
        let a2133 = {
            let a2130 = v0;
            let a2131 = v4;
            let a2132 = 1u64;
            nat_rem(a2130, a2131, a2132)
        };
        let a2134 = 0u64;
        if if if nat_eq(a2133, a2134) {
            let a2138 = {
                let a2135 = v1;
                let a2136 = v5;
                let a2137 = 1u64;
                nat_rem(a2135, a2136, a2137)
            };
            let a2139 = 0u64;
            nat_eq(a2138, a2139)
        } else {
            false
        } {
            let a2143 = {
                let a2140 = v2;
                let a2141 = v6;
                let a2142 = 1u64;
                nat_rem(a2140, a2141, a2142)
            };
            let a2144 = 0u64;
            nat_eq(a2143, a2144)
        } else {
            false
        } {
            let a2148 = {
                let a2145 = v3;
                let a2146 = v7;
                let a2147 = 1u64;
                nat_rem(a2145, a2146, a2147)
            };
            let a2149 = 0u64;
            nat_eq(a2148, a2149)
        } else {
            false
        }
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2150 = v0;
    let a2151 = v1;
    if nat_eq(a2150, a2151) {
        let a2154 = v4;
        let a2155 = {
            let a2152 = v2;
            let a2153 = v3;
            nat_add(a2152, a2153)?
        };
        Ok(nat_eq(a2154, a2155))
    } else {
        let a2156 = v2;
        let a2157 = v3;
        if nat_eq(a2156, a2157) {
            let a2158 = v4;
            let a2159 = v2;
            Ok(nat_eq(a2158, a2159))
        } else {
            Ok(false)
        }
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2162 = {
            let a2160 = v0;
            let a2161 = f141(v1);
            nat_add(a2160, a2161)?
        };
        let a2163 = f141(v2);
        nat_add(a2162, a2163)?
    };
    let v5: u64 = {
        let a2164 = f142(v1);
        let a2165 = f142(v2);
        nat_add(a2164, a2165)?
    };
    let a2166 = v1;
    let a2167 = 4294967295u64;
    if if nat_le(a2166, a2167) {
        let a2168 = v2;
        let a2169 = 4294967295u64;
        nat_le(a2168, a2169)
    } else {
        false
    } {
        let a2170 = v5;
        let a2171 = v4;
        if nat_le(a2170, a2171) {
            let a2174 = v3;
            let a2175 = {
                let a2172 = v4;
                let a2173 = v5;
                nat_sub(a2172, a2173)
            };
            Ok(nat_eq(a2174, a2175))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f99(v0: u64, v1: u64) -> R<bool> {
    let a2176 = v1;
    let a2177 = 4294967295u64;
    if nat_le(a2176, a2177) {
        let a2180 = {
            let a2178 = f141(v1);
            let a2179 = f142(v1);
            nat_add(a2178, a2179)?
        };
        let a2181 = v0;
        Ok(nat_lt(a2180, a2181))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f143(v0.clone(), false)?;
    let v6: u64 = f143(v0, true)?;
    let a2182 = v6;
    let a2183 = v1;
    if if nat_le(a2182, a2183) {
        let a2190 = {
            let a2188 = {
                let a2184 = v1;
                let a2185 = v5;
                nat_add(a2184, a2185)?
            };
            let a2189 = {
                let a2186 = v3;
                let a2187 = 0u64;
                if nat_eq(a2186, a2187) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2188, a2189)?
        };
        let a2191 = v2;
        nat_le(a2190, a2191)
    } else {
        false
    } {
        if v4 {
            let a2196 = {
                let a2194 = {
                    let a2192 = v1;
                    let a2193 = v5;
                    nat_add(a2192, a2193)?
                };
                let a2195 = v6;
                nat_sub(a2194, a2195)
            };
            let a2197 = v2;
            Ok(nat_lt(a2196, a2197))
        } else {
            let a2202 = {
                let a2200 = {
                    let a2198 = v1;
                    let a2199 = v5;
                    nat_add(a2198, a2199)?
                };
                let a2201 = v6;
                nat_sub(a2200, a2201)
            };
            let a2203 = v2;
            Ok(nat_le(a2202, a2203))
        }
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64, v1: u64) -> bool {
    let a2204 = v0;
    let a2205 = 0u64;
    if if nat_eq(a2204, a2205) {
        true
    } else {
        let a2206 = v0;
        let a2207 = 1u64;
        nat_eq(a2206, a2207)
    } {
        let a2208 = v1;
        let a2209 = 4294967296u64;
        nat_lt(a2208, a2209)
    } else {
        false
    }
}

pub fn f102(v0: u64, v1: u64) -> R<u64> {
    let a2214 = {
        let a2212 = v0;
        let a2213 = {
            let a2210 = v1;
            let a2211 = 1u64;
            nat_sub(a2210, a2211)
        };
        nat_add(a2212, a2213)?
    };
    let a2215 = v1;
    let a2216 = 0u64;
    Ok(nat_quot(a2214, a2215, a2216))
}

pub fn f103(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f29(match v0.clone() {
        Adt2::C0(_, h2217, _, _, _, _, _, _, _, _, _) => {
            h2217
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2218, _, _, _, _, _, _, _, _) => {
            h2218
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2219, _, _, _, _, _, _, _) => {
            h2219
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2220, _, _, _, _, _, _) => {
            h2220
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2223 = match v1.clone() {
                Adt2::C0(_, h2221, _, _, _, _, _, _, _, _, _) => {
                    h2221
                }
            };
            let a2224 = match v1.clone() {
                Adt2::C0(_, _, h2222, _, _, _, _, _, _, _, _) => {
                    h2222
                }
            };
            nat_mul(a2223, a2224)?
        };
        let v5: u64 = {
            let a2226 = v4;
            let a2227 = match v1.clone() {
                Adt2::C0(_, _, _, h2225, _, _, _, _, _, _, _) => {
                    h2225
                }
            };
            nat_mul(a2226, a2227)?
        };
        let a2240 = {
            let a2238 = {
                let a2229 = match v0.clone() {
                    Adt2::C0(_, h2228, _, _, _, _, _, _, _, _, _) => {
                        h2228
                    }
                };
                let a2230 = 1u64;
                nat_sub(a2229, a2230)
            };
            let a2239 = {
                let a2236 = f144(match v0.clone() {
                    Adt2::C0(_, _, h2231, _, _, _, _, _, _, _, _) => {
                        h2231
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2232, _, _, _, _, _, _, _, _, _) => {
                        h2232
                    }
                })?;
                let a2237 = f144(match v0 {
                    Adt2::C0(_, _, _, h2233, _, _, _, _, _, _, _) => {
                        h2233
                    }
                }, v2, {
                    let a2234 = v3;
                    let a2235 = v5;
                    nat_mul(a2234, a2235)?
                }, v4)?;
                nat_add(a2236, a2237)?
            };
            nat_add(a2238, a2239)?
        };
        let a2241 = f15(v1, 1u64)?;
        Ok(nat_lt(a2240, a2241))
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2242 = v0;
    let a2243 = 2147483648u64;
    if if if if nat_lt(a2242, a2243) {
        let a2244 = v1;
        let a2245 = 2147483648u64;
        nat_lt(a2244, a2245)
    } else {
        false
    } {
        let a2248 = {
            let a2246 = v0;
            let a2247 = v1;
            nat_mul(a2246, a2247)?
        };
        let a2249 = 2147483648u64;
        nat_lt(a2248, a2249)
    } else {
        false
    } {
        let a2254 = {
            let a2252 = {
                let a2250 = v0;
                let a2251 = 1u64;
                nat_sub(a2250, a2251)
            };
            let a2253 = v2;
            nat_mul(a2252, a2253)?
        };
        let a2255 = 2147483648u64;
        nat_lt(a2254, a2255)
    } else {
        false
    } {
        let a2260 = {
            let a2258 = {
                let a2256 = v1;
                let a2257 = 1u64;
                nat_sub(a2256, a2257)
            };
            let a2259 = v2;
            nat_mul(a2258, a2259)?
        };
        let a2261 = 2147483648u64;
        Ok(nat_lt(a2260, a2261))
    } else {
        Ok(false)
    }
}

pub fn f105(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2262 = v1;
    let a2263 = 13u64;
    if if nat_eq(a2262, a2263) {
        f29(match v0.clone() {
            Adt3::C0(h2264, _, _, _, _, _, _, _) => {
                h2264
            }
        }, match v0.clone() {
            Adt3::C0(_, h2265, _, _, _, _, _, _) => {
                h2265
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2266, _, _, _, _, _) => {
                h2266
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2267, _, _, _, _) => {
                h2267
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2273 = match v0.clone() {
                Adt3::C0(_, h2268, _, _, _, _, _, _) => {
                    h2268
                }
            };
            let a2274 = {
                let a2271 = match v0.clone() {
                    Adt3::C0(_, _, h2269, _, _, _, _, _) => {
                        h2269
                    }
                };
                let a2272 = match v0.clone() {
                    Adt3::C0(_, _, _, h2270, _, _, _, _) => {
                        h2270
                    }
                };
                nat_mul(a2271, a2272)?
            };
            nat_mul(a2273, a2274)?
        };
        let a2275 = v1;
        let a2276 = 13u64;
        if nat_eq(a2275, a2276) {
            let a2285 = 2u64;
            let a2286 = if f145(v0.clone(), v2) {
                let a2278 = v3;
                let a2279 = f119(match v0 {
                    Adt3::C0(h2277, _, _, _, _, _, _, _) => {
                        h2277
                    }
                }, false)?;
                nat_mul(a2278, a2279)?
            } else {
                let a2283 = v3;
                let a2284 = {
                    let a2281 = match v0 {
                        Adt3::C0(h2280, _, _, _, _, _, _, _) => {
                            h2280
                        }
                    };
                    let a2282 = 2u64;
                    nat_mul(a2281, a2282)?
                };
                nat_mul(a2283, a2284)?
            };
            nat_add(a2285, a2286)
        } else {
            let a2290 = v3;
            let a2291 = f119(match v0 {
                Adt3::C0(h2287, _, _, _, _, _, _, _) => {
                    h2287
                }
            }, {
                let a2288 = v1;
                let a2289 = 16u64;
                nat_eq(a2288, a2289)
            })?;
            nat_mul(a2290, a2291)
        }
    }
}

pub fn f106(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m2292 = v3;
    match m2292 {
        None => {
            f146(v0, v1, match v2.clone() {
                Adt2::C0(h2293, _, _, _, _, _, _, _, _, _, _) => {
                    h2293
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h2294, _, _, _, _, _, _, _, _, _, _) => {
                    h2294
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h2295, _, _, _, _, _, _, _, _, _) => {
                    h2295
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2296, _, _, _, _, _, _, _, _) => {
                    h2296
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2297, _, _, _, _, _, _, _) => {
                    h2297
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2298, _, _, _, _, _, _) => {
                    h2298
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h2299, _, _, _, _, _, _, _, _, _) => {
                    h2299
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2300, _, _, _, _, _, _, _, _) => {
                    h2300
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2301, _, _, _, _, _, _, _) => {
                    h2301
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2302, _, _, _, _, _, _) => {
                    h2302
                }
            })
        }
        Some(v5) => {
            f146(v0, v1, match v2.clone() {
                Adt2::C0(h2303, _, _, _, _, _, _, _, _, _, _) => {
                    h2303
                }
            }, match v5.clone() {
                Adt2::C0(h2304, _, _, _, _, _, _, _, _, _, _) => {
                    h2304
                }
            }, match v4.clone() {
                Adt2::C0(h2305, _, _, _, _, _, _, _, _, _, _) => {
                    h2305
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h2306, _, _, _, _, _, _, _, _, _) => {
                    h2306
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2307, _, _, _, _, _, _, _, _) => {
                    h2307
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2308, _, _, _, _, _, _, _) => {
                    h2308
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2309, _, _, _, _, _, _) => {
                    h2309
                }
            }, match v5.clone() {
                Adt2::C0(_, h2310, _, _, _, _, _, _, _, _, _) => {
                    h2310
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2311, _, _, _, _, _, _, _, _) => {
                    h2311
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2312, _, _, _, _, _, _, _) => {
                    h2312
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h2313, _, _, _, _, _, _) => {
                    h2313
                }
            }, match v4.clone() {
                Adt2::C0(_, h2314, _, _, _, _, _, _, _, _, _) => {
                    h2314
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2315, _, _, _, _, _, _, _, _) => {
                    h2315
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2316, _, _, _, _, _, _, _) => {
                    h2316
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2317, _, _, _, _, _, _) => {
                    h2317
                }
            })
        }
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2330 = {
        let a2326 = {
            let a2322 = {
                let a2318 = v0;
                let a2319 = v4;
                nat_mul(a2318, a2319)?
            };
            let a2323 = {
                let a2320 = v1;
                let a2321 = v5;
                nat_mul(a2320, a2321)?
            };
            nat_add(a2322, a2323)?
        };
        let a2327 = {
            let a2324 = v2;
            let a2325 = v6;
            nat_mul(a2324, a2325)?
        };
        nat_add(a2326, a2327)?
    };
    let a2331 = {
        let a2328 = v3;
        let a2329 = v7;
        nat_mul(a2328, a2329)?
    };
    nat_add(a2330, a2331)
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2336 = {
        let a2334 = {
            let a2332 = v0;
            let a2333 = v1;
            nat_sub(a2332, a2333)
        };
        let a2335 = 1u64;
        nat_sub(a2334, a2335)
    };
    let a2337 = v2;
    nat_add(a2336, a2337)
}

pub fn f109(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f107({
        let a2338 = v0;
        let a2339 = v4;
        let a2340 = 0u64;
        nat_rem(a2338, a2339, a2340)
    }, {
        let a2341 = v1;
        let a2342 = v5;
        let a2343 = 0u64;
        nat_rem(a2341, a2342, a2343)
    }, {
        let a2344 = v2;
        let a2345 = v6;
        let a2346 = 0u64;
        nat_rem(a2344, a2345, a2346)
    }, {
        let a2347 = v3;
        let a2348 = v7;
        let a2349 = 0u64;
        nat_rem(a2347, a2348, a2349)
    }, v8, v9, v10, v11)
}

pub fn f110(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2351 = v0;
        let a2352 = match v1.clone() {
            Adt3::C0(h2350, _, _, _, _, _, _, _) => {
                h2350
            }
        };
        let a2353 = 0u64;
        nat_quot(a2351, a2352, a2353)
    };
    let v4: u64 = {
        let a2355 = v3;
        let a2356 = match v1.clone() {
            Adt3::C0(_, h2354, _, _, _, _, _, _) => {
                h2354
            }
        };
        let a2357 = 0u64;
        nat_quot(a2355, a2356, a2357)
    };
    let a2358 = v2;
    let a2359 = 0u64;
    if nat_eq(a2358, a2359) {
        let a2361 = v0;
        let a2362 = match v1 {
            Adt3::C0(h2360, _, _, _, _, _, _, _) => {
                h2360
            }
        };
        let a2363 = 0u64;
        nat_rem(a2361, a2362, a2363)
    } else {
        let a2364 = v2;
        let a2365 = 1u64;
        if nat_eq(a2364, a2365) {
            let a2367 = v3;
            let a2368 = match v1 {
                Adt3::C0(_, h2366, _, _, _, _, _, _) => {
                    h2366
                }
            };
            let a2369 = 0u64;
            nat_rem(a2367, a2368, a2369)
        } else {
            let a2370 = v2;
            let a2371 = 2u64;
            if nat_eq(a2370, a2371) {
                let a2373 = v4;
                let a2374 = match v1 {
                    Adt3::C0(_, _, h2372, _, _, _, _, _) => {
                        h2372
                    }
                };
                let a2375 = 0u64;
                nat_rem(a2373, a2374, a2375)
            } else {
                let a2377 = v4;
                let a2378 = match v1 {
                    Adt3::C0(_, _, h2376, _, _, _, _, _) => {
                        h2376
                    }
                };
                let a2379 = 0u64;
                nat_quot(a2377, a2378, a2379)
            }
        }
    }
}

pub fn f111(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f107(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2380, _, _, _) => {
            h2380
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2381, _, _) => {
            h2381
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2382, _) => {
            h2382
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2383) => {
            h2383
        }
    })
}

pub fn f112(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2384 = v1;
    let a2385 = 2147483648u64;
    if nat_lt(a2384, a2385) {
        let a2396 = {
            let a2394 = {
                let a2389 = {
                    let a2386 = v0;
                    let a2387 = v2;
                    let a2388 = 0u64;
                    nat_rem(a2386, a2387, a2388)
                };
                let a2390 = v2;
                nat_add(a2389, a2390)?
            };
            let a2395 = {
                let a2391 = v1;
                let a2392 = v2;
                let a2393 = 0u64;
                nat_rem(a2391, a2392, a2393)
            };
            nat_sub(a2394, a2395)
        };
        let a2397 = v2;
        let a2398 = 0u64;
        Ok(nat_rem(a2396, a2397, a2398))
    } else {
        let a2401 = {
            let a2399 = v0;
            let a2400 = f142(v1);
            nat_add(a2399, a2400)?
        };
        let a2402 = v2;
        let a2403 = 0u64;
        Ok(nat_rem(a2401, a2402, a2403))
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2406 = f141(v1);
    let a2407 = {
        let a2404 = v0;
        let a2405 = f142(v1);
        nat_add(a2404, a2405)?
    };
    if nat_le(a2406, a2407) {
        let a2408 = f114(v0, v1)?;
        let a2409 = v2;
        Ok(nat_lt(a2408, a2409))
    } else {
        Ok(false)
    }
}

pub fn f114(v0: u64, v1: u64) -> R<u64> {
    let a2412 = {
        let a2410 = v0;
        let a2411 = f142(v1);
        nat_add(a2410, a2411)?
    };
    let a2413 = f141(v1);
    Ok(nat_sub(a2412, a2413))
}

pub fn f115(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2414 = v0;
    let a2415 = v1;
    if nat_lt(a2414, a2415) {
        let a2416 = v1;
        let a2417 = v0;
        Ok(nat_sub(a2416, a2417))
    } else {
        let a2420 = v0;
        let a2421 = {
            let a2418 = v1;
            let a2419 = v2;
            nat_add(a2418, a2419)?
        };
        if nat_lt(a2420, a2421) {
            let a2422 = v0;
            let a2423 = v1;
            Ok(nat_sub(a2422, a2423))
        } else {
            let a2430 = {
                let a2426 = 2u64;
                let a2427 = {
                    let a2424 = v2;
                    let a2425 = 1u64;
                    nat_sub(a2424, a2425)
                };
                nat_mul(a2426, a2427)?
            };
            let a2431 = {
                let a2428 = v0;
                let a2429 = v1;
                nat_sub(a2428, a2429)
            };
            Ok(nat_sub(a2430, a2431))
        }
    }
}

pub fn f116(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2434 = {
        let a2432 = v2;
        let a2433 = f147(v1.clone(), f110(v0, v1.clone(), 0u64), f110(v0, v1.clone(), 1u64), f110(v0, v1.clone(), 2u64), f110(v0, v1.clone(), 3u64), false)?;
        nat_add(a2432, a2433)?
    };
    let a2435 = f147(v1.clone(), f110(v0, v1.clone(), 0u64), f110(v0, v1.clone(), 1u64), f110(v0, v1.clone(), 2u64), f110(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2434, a2435))
}

pub fn f117(v0: u64, v1: u64, v2: u64) -> bool {
    let a2436 = v0;
    let a2437 = 0u64;
    if nat_eq(a2436, a2437) {
        let a2438 = v2;
        let a2439 = v1;
        nat_le(a2438, a2439)
    } else {
        let a2440 = v0;
        let a2441 = 1u64;
        if nat_eq(a2440, a2441) {
            let a2442 = v2;
            let a2443 = v1;
            nat_lt(a2442, a2443)
        } else {
            let a2444 = v0;
            let a2445 = 2u64;
            if nat_eq(a2444, a2445) {
                let a2446 = v1;
                let a2447 = v2;
                nat_le(a2446, a2447)
            } else {
                let a2448 = v0;
                let a2449 = 3u64;
                if nat_eq(a2448, a2449) {
                    let a2450 = v1;
                    let a2451 = v2;
                    nat_lt(a2450, a2451)
                } else {
                    let a2452 = v1;
                    let a2453 = v2;
                    nat_eq(a2452, a2453)
                }
            }
        }
    }
}

pub fn f118(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2454 = v0;
    let a2455 = 0u64;
    if nat_eq(a2454, a2455) {
        Ok(f148(0u64))
    } else {
        let a2458 = {
            let a2456 = v0;
            let a2457 = 1u64;
            nat_add(a2456, a2457)?
        };
        let a2459 = f105(v1.clone(), 13u64, v2)?;
        if nat_eq(a2458, a2459) {
            Ok(Adt5::C4({
                let a2460 = v2;
                let a2461 = 0u64;
                if nat_eq(a2460, a2461) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2462 = v0;
                let a2463 = 1u64;
                nat_sub(a2462, a2463)
            };
            if f145(v1.clone(), v2) {
                let v4: u64 = f119(match v1.clone() {
                    Adt3::C0(h2464, _, _, _, _, _, _, _) => {
                        h2464
                    }
                }, false)?;
                let v5: u64 = {
                    let a2465 = v3;
                    let a2466 = v4;
                    let a2467 = 0u64;
                    nat_quot(a2465, a2466, a2467)
                };
                let v6: u64 = {
                    let a2468 = v3;
                    let a2469 = v4;
                    let a2470 = 0u64;
                    nat_rem(a2468, a2469, a2470)
                };
                let a2473 = {
                    let a2471 = v6;
                    let a2472 = 1u64;
                    nat_add(a2471, a2472)?
                };
                let a2474 = v4;
                if nat_eq(a2473, a2474) {
                    Ok(Adt5::C4({
                        let a2475 = v2;
                        let a2476 = 0u64;
                        if nat_eq(a2475, a2476) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f120(v6, v5, v1, v2)
                }
            } else {
                f149({
                    let a2477 = v3;
                    let a2478 = 2u64;
                    let a2479 = 0u64;
                    nat_quot(a2477, a2478, a2479)
                }, {
                    let a2480 = v3;
                    let a2481 = 2u64;
                    let a2482 = 0u64;
                    nat_rem(a2480, a2481, a2482)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f119(v0: u64, v1: bool) -> R<u64> {
    let a2485 = {
        let a2483 = v0;
        let a2484 = 2u64;
        nat_mul(a2483, a2484)?
    };
    let a2486 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2485, a2486)
}

pub fn f120(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2487 = v0;
    let a2488 = 0u64;
    if nat_eq(a2487, a2488) {
        Ok(f148(1u64))
    } else {
        f149({
            let a2497 = {
                let a2490 = v1;
                let a2491 = match v2.clone() {
                    Adt3::C0(h2489, _, _, _, _, _, _, _) => {
                        h2489
                    }
                };
                nat_mul(a2490, a2491)?
            };
            let a2498 = {
                let a2494 = {
                    let a2492 = v0;
                    let a2493 = 1u64;
                    nat_sub(a2492, a2493)
                };
                let a2495 = 2u64;
                let a2496 = 0u64;
                nat_quot(a2494, a2495, a2496)
            };
            nat_add(a2497, a2498)?
        }, {
            let a2501 = {
                let a2499 = v0;
                let a2500 = 1u64;
                nat_sub(a2499, a2500)
            };
            let a2502 = 2u64;
            let a2503 = 0u64;
            nat_rem(a2501, a2502, a2503)
        }, v2, v3, 1u64)
    }
}

pub fn f121(v0: u64, v1: u64) -> R<u64> {
    let a2504 = v0;
    let a2505 = f85(v1)?;
    let a2506 = 0u64;
    Ok(nat_rem(a2504, a2505, a2506))
}

pub fn f122(v0: u64, v1: u64) -> R<u64> {
    let a2507 = 4u64;
    let a2508 = f150(v0, v1)?;
    nat_mul(a2507, a2508)
}

pub fn f123(v0: u64, v1: u64) -> R<u64> {
    let a2509 = 4u64;
    let a2510 = f151(v0, v1)?;
    nat_mul(a2509, a2510)
}

pub fn f124(v0: u64, v1: u64) -> R<bool> {
    let a2516 = {
        let a2513 = {
            let a2511 = f121(v0, v1)?;
            let a2512 = 2u64;
            nat_sub(a2511, a2512)
        };
        let a2514 = 2u64;
        let a2515 = 0u64;
        nat_rem(a2513, a2514, a2515)
    };
    let a2517 = 0u64;
    Ok(nat_eq(a2516, a2517))
}

pub fn f125(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2518 = f95(v0);
    match m2518 {
        Adt10::C0 => {
            Ok(None::<u64>)
        }
        Adt10::C1(v13) => {
            if if if if if f152(v0) {
                let a2519 = v1;
                let a2520 = v7;
                nat_lt(a2519, a2520)
            } else {
                false
            } {
                f76(v8, v2)
            } else {
                false
            } {
                let a2521 = v3;
                let a2522 = f153(v4, v5, v6)?;
                nat_lt(a2521, a2522)
            } else {
                false
            } {
                let a2527 = {
                    let a2524 = v7;
                    let a2525 = match v13.clone() {
                        Adt11::C0(_, h2523, _) => {
                            h2523
                        }
                    };
                    let a2526 = 0u64;
                    nat_rem(a2524, a2525, a2526)
                };
                let a2528 = 0u64;
                nat_eq(a2527, a2528)
            } else {
                false
            } {
                Ok(Some({
                    let a2557 = {
                        let a2547 = {
                            let a2537 = {
                                let a2533 = {
                                    let a2530 = v1;
                                    let a2531 = match v13 {
                                        Adt11::C0(_, h2529, _) => {
                                            h2529
                                        }
                                    };
                                    let a2532 = 0u64;
                                    nat_quot(a2530, a2531, a2532)
                                };
                                let a2534 = v9;
                                nat_mul(a2533, a2534)?
                            };
                            let a2538 = {
                                let a2535 = v2;
                                let a2536 = v10;
                                nat_mul(a2535, a2536)?
                            };
                            nat_add(a2537, a2538)?
                        };
                        let a2548 = {
                            let a2545 = {
                                let a2542 = {
                                    let a2539 = v3;
                                    let a2540 = v4;
                                    let a2541 = 0u64;
                                    nat_quot(a2539, a2540, a2541)
                                };
                                let a2543 = v5;
                                let a2544 = 0u64;
                                nat_rem(a2542, a2543, a2544)
                            };
                            let a2546 = v11;
                            nat_mul(a2545, a2546)?
                        };
                        nat_add(a2547, a2548)?
                    };
                    let a2558 = {
                        let a2555 = {
                            let a2552 = {
                                let a2549 = v3;
                                let a2550 = v4;
                                let a2551 = 0u64;
                                nat_quot(a2549, a2550, a2551)
                            };
                            let a2553 = v5;
                            let a2554 = 0u64;
                            nat_quot(a2552, a2553, a2554)
                        };
                        let a2556 = v12;
                        nat_mul(a2555, a2556)?
                    };
                    nat_add(a2557, a2558)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f126(v0: u64) -> u64 {
    let a2559 = v0;
    let a2560 = 40u64;
    if nat_lt(a2559, a2560) {
        let a2561 = v0;
        let a2562 = 20u64;
        if nat_lt(a2561, a2562) {
            let a2563 = v0;
            let a2564 = 10u64;
            if nat_lt(a2563, a2564) {
                let a2565 = v0;
                let a2566 = 5u64;
                if nat_lt(a2565, a2566) {
                    let a2567 = v0;
                    let a2568 = 2u64;
                    if nat_lt(a2567, a2568) {
                        let a2569 = v0;
                        let a2570 = 1u64;
                        if nat_lt(a2569, a2570) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2571 = v0;
                        let a2572 = 3u64;
                        if nat_lt(a2571, a2572) {
                            4u64
                        } else {
                            let a2573 = v0;
                            let a2574 = 4u64;
                            if nat_lt(a2573, a2574) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2575 = v0;
                    let a2576 = 7u64;
                    if nat_lt(a2575, a2576) {
                        let a2577 = v0;
                        let a2578 = 6u64;
                        if nat_lt(a2577, a2578) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2579 = v0;
                        let a2580 = 8u64;
                        if nat_lt(a2579, a2580) {
                            128u64
                        } else {
                            let a2581 = v0;
                            let a2582 = 9u64;
                            if nat_lt(a2581, a2582) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2583 = v0;
                let a2584 = 15u64;
                if nat_lt(a2583, a2584) {
                    let a2585 = v0;
                    let a2586 = 12u64;
                    if nat_lt(a2585, a2586) {
                        let a2587 = v0;
                        let a2588 = 11u64;
                        if nat_lt(a2587, a2588) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2589 = v0;
                        let a2590 = 13u64;
                        if nat_lt(a2589, a2590) {
                            4096u64
                        } else {
                            let a2591 = v0;
                            let a2592 = 14u64;
                            if nat_lt(a2591, a2592) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2593 = v0;
                    let a2594 = 17u64;
                    if nat_lt(a2593, a2594) {
                        let a2595 = v0;
                        let a2596 = 16u64;
                        if nat_lt(a2595, a2596) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2597 = v0;
                        let a2598 = 18u64;
                        if nat_lt(a2597, a2598) {
                            131072u64
                        } else {
                            let a2599 = v0;
                            let a2600 = 19u64;
                            if nat_lt(a2599, a2600) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2601 = v0;
            let a2602 = 30u64;
            if nat_lt(a2601, a2602) {
                let a2603 = v0;
                let a2604 = 25u64;
                if nat_lt(a2603, a2604) {
                    let a2605 = v0;
                    let a2606 = 22u64;
                    if nat_lt(a2605, a2606) {
                        let a2607 = v0;
                        let a2608 = 21u64;
                        if nat_lt(a2607, a2608) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2609 = v0;
                        let a2610 = 23u64;
                        if nat_lt(a2609, a2610) {
                            4194304u64
                        } else {
                            let a2611 = v0;
                            let a2612 = 24u64;
                            if nat_lt(a2611, a2612) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2613 = v0;
                    let a2614 = 27u64;
                    if nat_lt(a2613, a2614) {
                        let a2615 = v0;
                        let a2616 = 26u64;
                        if nat_lt(a2615, a2616) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2617 = v0;
                        let a2618 = 28u64;
                        if nat_lt(a2617, a2618) {
                            134217728u64
                        } else {
                            let a2619 = v0;
                            let a2620 = 29u64;
                            if nat_lt(a2619, a2620) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2621 = v0;
                let a2622 = 35u64;
                if nat_lt(a2621, a2622) {
                    let a2623 = v0;
                    let a2624 = 32u64;
                    if nat_lt(a2623, a2624) {
                        let a2625 = v0;
                        let a2626 = 31u64;
                        if nat_lt(a2625, a2626) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2627 = v0;
                        let a2628 = 33u64;
                        if nat_lt(a2627, a2628) {
                            4294967296u64
                        } else {
                            let a2629 = v0;
                            let a2630 = 34u64;
                            if nat_lt(a2629, a2630) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2631 = v0;
                    let a2632 = 37u64;
                    if nat_lt(a2631, a2632) {
                        let a2633 = v0;
                        let a2634 = 36u64;
                        if nat_lt(a2633, a2634) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2635 = v0;
                        let a2636 = 38u64;
                        if nat_lt(a2635, a2636) {
                            137438953472u64
                        } else {
                            let a2637 = v0;
                            let a2638 = 39u64;
                            if nat_lt(a2637, a2638) {
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

pub fn f127(v0: Adt7, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt5 {
    let v9: u64 = {
        let a2639 = v7;
        let a2640 = 6u64;
        if nat_eq(a2639, a2640) {
            27u64
        } else {
            v2
        }
    };
    Adt5::C7(v0, f154(v3, v4), f154(v5, v6), f155(v7, v8), f156(v1), f156(v9))
}

pub fn f128(v0: Adt4, v1: u64, v2: u64) -> Adt7 {
    let m2641 = v0;
    match m2641 {
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

pub fn f129(v0: Adt4) -> u64 {
    let m2642 = v0;
    match m2642 {
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

pub fn f130(v0: Adt4, v1: u64) -> u64 {
    let m2643 = v0;
    match m2643 {
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
            let a2644 = v1;
            let a2645 = 6u64;
            if nat_eq(a2644, a2645) {
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
            let a2646 = v1;
            let a2647 = 6u64;
            if if nat_eq(a2646, a2647) {
                true
            } else {
                let a2648 = v1;
                let a2649 = 7u64;
                nat_eq(a2648, a2649)
            } {
                v49
            } else {
                0u64
            }
        }
    }
}

pub fn f131(v0: Adt4, v1: u64) -> Adt7 {
    let a2650 = v1;
    let a2651 = 4u64;
    if nat_lt(a2650, a2651) {
        Adt7::C0(v1)
    } else {
        let a2652 = v1;
        let a2653 = 4u64;
        if nat_eq(a2652, a2653) {
            Adt7::C1(0u64)
        } else {
            let a2654 = v1;
            let a2655 = 5u64;
            if nat_eq(a2654, a2655) {
                Adt7::C1(3u64)
            } else {
                let a2656 = v1;
                let a2657 = 6u64;
                if nat_eq(a2656, a2657) {
                    Adt7::C2(f130(v0, v1), 0u64, false)
                } else {
                    let a2658 = v1;
                    let a2659 = 7u64;
                    if nat_eq(a2658, a2659) {
                        Adt7::C2(0u64, f130(v0, v1), false)
                    } else {
                        let a2660 = v1;
                        let a2661 = 16u64;
                        if nat_lt(a2660, a2661) {
                            Adt7::C1({
                                let a2662 = v1;
                                let a2663 = 8u64;
                                nat_sub(a2662, a2663)
                            })
                        } else {
                            let a2664 = v1;
                            let a2665 = 20u64;
                            if nat_lt(a2664, a2665) {
                                Adt7::C5({
                                    let a2666 = v1;
                                    let a2667 = 16u64;
                                    nat_sub(a2666, a2667)
                                })
                            } else {
                                let a2668 = v1;
                                let a2669 = 20u64;
                                if nat_eq(a2668, a2669) {
                                    Adt7::C3(0u64, 0u64, false)
                                } else {
                                    let a2670 = v1;
                                    let a2671 = 21u64;
                                    if nat_eq(a2670, a2671) {
                                        Adt7::C0(4u64)
                                    } else {
                                        let a2672 = v1;
                                        let a2673 = 22u64;
                                        if nat_eq(a2672, a2673) {
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

pub fn f132(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a2682 = {
            let a2674 = 3u64;
            let a2675 = v3;
            nat_add(a2674, a2675)?
        };
        let a2683 = {
            let a2679 = f57({
                let a2676 = v0;
                let a2677 = 2u64;
                let a2678 = 0u64;
                nat_quot(a2676, a2677, a2678)
            }, v1)?;
            let a2680 = f25(v2);
            let a2681 = 0u64;
            nat_quot(a2679, a2680, a2681)
        };
        nat_add(a2682, a2683)?
    };
    let a2687 = {
        let a2684 = v0;
        let a2685 = 2u64;
        let a2686 = 0u64;
        nat_rem(a2684, a2685, a2686)
    };
    let a2688 = 0u64;
    if nat_eq(a2687, a2688) {
        Ok(Adt5::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a2689 = v4;
        let a2690 = 0u64;
        if nat_eq(a2689, a2690) {
            Ok(Adt5::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f133(v0: u64, v1: u64) -> R<u64> {
    let a2693 = 8u64;
    let a2694 = {
        let a2691 = f157(v0);
        let a2692 = v1;
        nat_mul(a2691, a2692)?
    };
    nat_add(a2693, a2694)
}

pub fn f134(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2695, _, _, _, _, _, _, _) => {
            h2695
        }
    };
    let v7: u64 = f157(v3);
    let v8: u64 = {
        let a2698 = 1u64;
        let a2699 = {
            let a2696 = v6;
            let a2697 = v7;
            nat_mul(a2696, a2697)?
        };
        nat_add(a2698, a2699)?
    };
    let a2700 = v0;
    let a2701 = 0u64;
    if nat_eq(a2700, a2701) {
        Ok(f148(0u64))
    } else {
        let a2702 = v0;
        let a2703 = v8;
        if nat_lt(a2702, a2703) {
            let v9: u64 = {
                let a2704 = v0;
                let a2705 = 1u64;
                nat_sub(a2704, a2705)
            };
            let v10: u64 = {
                let a2706 = v9;
                let a2707 = v7;
                let a2708 = 0u64;
                nat_rem(a2706, a2707, a2708)
            };
            let v11: u64 = f57({
                let a2714 = {
                    let a2709 = v1;
                    let a2710 = v6;
                    nat_mul(a2709, a2710)?
                };
                let a2715 = {
                    let a2711 = v9;
                    let a2712 = v7;
                    let a2713 = 0u64;
                    nat_quot(a2711, a2712, a2713)
                };
                nat_add(a2714, a2715)?
            }, v2)?;
            let a2716 = v3;
            let a2717 = 30u64;
            if nat_eq(a2716, a2717) {
                let a2718 = v10;
                let a2719 = 0u64;
                if nat_eq(a2718, a2719) {
                    Ok(Adt5::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2720 = v10;
                    let a2721 = 1u64;
                    if nat_eq(a2720, a2721) {
                        Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a2722 = v10;
                        let a2723 = 2u64;
                        if nat_eq(a2722, a2723) {
                            Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a2724 = v10;
                let a2725 = 0u64;
                if nat_eq(a2724, a2725) {
                    Ok(Adt5::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a2726 = v10;
                    let a2727 = 1u64;
                    if nat_eq(a2726, a2727) {
                        Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a2728 = v0;
                let a2729 = v8;
                nat_sub(a2728, a2729)
            };
            let a2730 = v12;
            let a2731 = 0u64;
            if nat_eq(a2730, a2731) {
                Ok(Adt5::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2732 = v12;
                let a2733 = 1u64;
                if nat_eq(a2732, a2733) {
                    Ok(Adt5::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a2734 = v12;
                    let a2735 = 2u64;
                    if nat_eq(a2734, a2735) {
                        Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a2736 = v12;
                        let a2737 = 3u64;
                        if nat_eq(a2736, a2737) {
                            Ok(Adt5::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a2738 = v12;
                            let a2739 = 4u64;
                            if nat_eq(a2738, a2739) {
                                Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a2740 = v12;
                                let a2741 = 5u64;
                                if nat_eq(a2740, a2741) {
                                    Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a2742 = v12;
                                    let a2743 = 6u64;
                                    if nat_eq(a2742, a2743) {
                                        if f158(v5) {
                                            Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a2744 = 3u64;
                                                let a2745 = v1;
                                                nat_add(a2744, a2745)?
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

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = {
        let a2746 = v0;
        let a2747 = f90(v3);
        let a2748 = 0u64;
        nat_quot(a2746, a2747, a2748)
    };
    let v5: u64 = {
        let a2753 = 3u64;
        let a2754 = {
            let a2750 = v4;
            let a2751 = match v1.clone() {
                Adt3::C0(h2749, _, _, _, _, _, _, _) => {
                    h2749
                }
            };
            let a2752 = 0u64;
            nat_quot(a2750, a2751, a2752)
        };
        nat_add(a2753, a2754)?
    };
    let v6: u64 = f57(v4, v1)?;
    let v7: u64 = f57(v4, v2)?;
    let a2755 = v3;
    let a2756 = 30u64;
    if nat_eq(a2755, a2756) {
        let a2760 = {
            let a2757 = v0;
            let a2758 = 3u64;
            let a2759 = 0u64;
            nat_rem(a2757, a2758, a2759)
        };
        let a2761 = 0u64;
        if nat_eq(a2760, a2761) {
            Ok(Adt5::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2765 = {
                let a2762 = v0;
                let a2763 = 3u64;
                let a2764 = 0u64;
                nat_rem(a2762, a2763, a2764)
            };
            let a2766 = 1u64;
            if nat_eq(a2765, a2766) {
                Ok(Adt5::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt5::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt5::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f136(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h2767, _, _, _, _, _, _, _) => {
            h2767
        }
    };
    let v7: u64 = {
        let a2770 = 1u64;
        let a2771 = {
            let a2768 = 3u64;
            let a2769 = v6;
            nat_mul(a2768, a2769)?
        };
        nat_add(a2770, a2771)?
    };
    let a2772 = v0;
    let a2773 = 0u64;
    if nat_eq(a2772, a2773) {
        Ok(f148(0u64))
    } else {
        let a2774 = v0;
        let a2775 = v7;
        if nat_lt(a2774, a2775) {
            let v8: u64 = {
                let a2776 = v0;
                let a2777 = 1u64;
                nat_sub(a2776, a2777)
            };
            let v9: u64 = {
                let a2783 = {
                    let a2778 = v1;
                    let a2779 = v6;
                    nat_mul(a2778, a2779)?
                };
                let a2784 = {
                    let a2780 = v8;
                    let a2781 = 3u64;
                    let a2782 = 0u64;
                    nat_quot(a2780, a2781, a2782)
                };
                nat_add(a2783, a2784)?
            };
            let v10: u64 = f57(v9, v2)?;
            let v11: u64 = {
                let a2785 = v8;
                let a2786 = 3u64;
                let a2787 = 0u64;
                nat_rem(a2785, a2786, a2787)
            };
            let a2788 = v11;
            let a2789 = 0u64;
            if nat_eq(a2788, a2789) {
                Ok(Adt5::C4(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a2790 = v11;
                let a2791 = 1u64;
                if nat_eq(a2790, a2791) {
                    Ok(Adt5::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt5::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a2792 = v0;
            let a2793 = v7;
            if nat_eq(a2792, a2793) {
                Ok(Adt5::C4(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a2796 = v0;
                let a2797 = {
                    let a2794 = v7;
                    let a2795 = 1u64;
                    nat_add(a2794, a2795)?
                };
                if nat_eq(a2796, a2797) {
                    Ok(Adt5::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a2800 = v0;
                    let a2801 = {
                        let a2798 = v7;
                        let a2799 = 2u64;
                        nat_add(a2798, a2799)?
                    };
                    if nat_eq(a2800, a2801) {
                        Ok(Adt5::C4(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a2804 = v0;
                        let a2805 = {
                            let a2802 = v7;
                            let a2803 = 3u64;
                            nat_add(a2802, a2803)?
                        };
                        if nat_eq(a2804, a2805) {
                            let a2806 = v5;
                            let a2807 = 1u64;
                            if if nat_eq(a2806, a2807) {
                                true
                            } else {
                                let a2808 = v5;
                                let a2809 = 2u64;
                                nat_eq(a2808, a2809)
                            } {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(Adt5::C4(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a2816 = {
                                    let a2810 = v1;
                                    let a2811 = v6;
                                    nat_mul(a2810, a2811)?
                                };
                                let a2817 = {
                                    let a2814 = v0;
                                    let a2815 = {
                                        let a2812 = v7;
                                        let a2813 = 4u64;
                                        nat_add(a2812, a2813)?
                                    };
                                    nat_sub(a2814, a2815)
                                };
                                nat_add(a2816, a2817)?
                            };
                            Ok(Adt5::C4(10u64, 0u64, f57(v12, v2)?, 1u64, 2u64, 3u64, f57(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: u64, v1: u64, v2: u64) -> bool {
    let a2818 = v0;
    let a2819 = 1u64;
    if nat_le(a2818, a2819) {
        true
    } else {
        let a2823 = {
            let a2820 = v1;
            let a2821 = v2;
            let a2822 = 1u64;
            nat_rem(a2820, a2821, a2822)
        };
        let a2824 = 0u64;
        nat_eq(a2823, a2824)
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a2825 = v1;
    let a2826 = 1u64;
    if if if nat_le(a2825, a2826) {
        true
    } else {
        let a2827 = v0;
        let a2828 = v2;
        nat_le(a2827, a2828)
    } {
        let a2829 = v3;
        let a2830 = 1u64;
        if nat_le(a2829, a2830) {
            true
        } else {
            let a2837 = {
                let a2835 = v0;
                let a2836 = {
                    let a2833 = {
                        let a2831 = v1;
                        let a2832 = 1u64;
                        nat_sub(a2831, a2832)
                    };
                    let a2834 = v2;
                    nat_mul(a2833, a2834)?
                };
                nat_add(a2835, a2836)?
            };
            let a2838 = v4;
            nat_le(a2837, a2838)
        }
    } else {
        false
    } {
        let a2839 = v5;
        let a2840 = 1u64;
        if nat_le(a2839, a2840) {
            Ok(true)
        } else {
            let a2853 = {
                let a2851 = {
                    let a2845 = v0;
                    let a2846 = {
                        let a2843 = {
                            let a2841 = v1;
                            let a2842 = 1u64;
                            nat_sub(a2841, a2842)
                        };
                        let a2844 = v2;
                        nat_mul(a2843, a2844)?
                    };
                    nat_add(a2845, a2846)?
                };
                let a2852 = {
                    let a2849 = {
                        let a2847 = v3;
                        let a2848 = 1u64;
                        nat_sub(a2847, a2848)
                    };
                    let a2850 = v4;
                    nat_mul(a2849, a2850)?
                };
                nat_add(a2851, a2852)?
            };
            let a2854 = v6;
            Ok(nat_le(a2853, a2854))
        }
    } else {
        Ok(false)
    }
}

pub fn f139(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f159(v0, v1, v6, v7, v8) {
        let a2855 = v5;
        let a2856 = 26u64;
        nat_eq(a2855, a2856)
    } else {
        false
    } {
        let a2857 = v3;
        let a2858 = v7;
        nat_eq(a2857, a2858)
    } else {
        false
    } {
        let a2859 = v4;
        let a2860 = v8;
        nat_eq(a2859, a2860)
    } else {
        false
    } {
        let a2861 = v9;
        let a2862 = 1u64;
        nat_eq(a2861, a2862)
    } else {
        false
    } {
        let a2863 = v10;
        let a2864 = f160(v0);
        nat_eq(a2863, a2864)
    } else {
        false
    } {
        let a2865 = v11;
        let a2866 = v1;
        nat_eq(a2865, a2866)
    } else {
        false
    } {
        let a2867 = v12;
        let a2868 = v6;
        nat_eq(a2867, a2868)
    } else {
        false
    } {
        let a2869 = v13;
        let a2870 = v7;
        nat_eq(a2869, a2870)
    } else {
        false
    } {
        let a2871 = v14;
        let a2872 = v8;
        nat_eq(a2871, a2872)
    } else {
        false
    }
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2873 = v0;
    let a2874 = 0u64;
    if if if nat_eq(a2873, a2874) {
        true
    } else {
        let a2875 = v1;
        let a2876 = 0u64;
        nat_eq(a2875, a2876)
    } {
        true
    } else {
        let a2877 = v2;
        let a2878 = 0u64;
        nat_eq(a2877, a2878)
    } {
        true
    } else {
        let a2879 = v3;
        let a2880 = 0u64;
        nat_eq(a2879, a2880)
    }
}

pub fn f141(v0: u64) -> u64 {
    let a2881 = v0;
    let a2882 = 2147483648u64;
    if nat_lt(a2881, a2882) {
        v0
    } else {
        0u64
    }
}

pub fn f142(v0: u64) -> u64 {
    let a2883 = v0;
    let a2884 = 2147483648u64;
    if nat_lt(a2883, a2884) {
        0u64
    } else {
        let a2885 = 4294967296u64;
        let a2886 = v0;
        nat_sub(a2885, a2886)
    }
}

pub fn f143(v0: Adt3, v1: bool) -> R<u64> {
    f147(v0.clone(), {
        let a2888 = match v0.clone() {
            Adt3::C0(h2887, _, _, _, _, _, _, _) => {
                h2887
            }
        };
        let a2889 = 1u64;
        nat_sub(a2888, a2889)
    }, {
        let a2891 = match v0.clone() {
            Adt3::C0(_, h2890, _, _, _, _, _, _) => {
                h2890
            }
        };
        let a2892 = 1u64;
        nat_sub(a2891, a2892)
    }, {
        let a2894 = match v0.clone() {
            Adt3::C0(_, _, h2893, _, _, _, _, _) => {
                h2893
            }
        };
        let a2895 = 1u64;
        nat_sub(a2894, a2895)
    }, {
        let a2897 = match v0 {
            Adt3::C0(_, _, _, h2896, _, _, _, _) => {
                h2896
            }
        };
        let a2898 = 1u64;
        nat_sub(a2897, a2898)
    }, v1)
}

pub fn f144(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2899 = v0;
    let a2900 = 0u64;
    if nat_eq(a2899, a2900) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2901 = v0;
            let a2902 = 1u64;
            nat_sub(a2901, a2902)
        };
        let v5: u64 = {
            let a2903 = v4;
            let a2904 = v1;
            let a2905 = 0u64;
            nat_quot(a2903, a2904, a2905)
        };
        let v6: u64 = {
            let a2913 = {
                let a2906 = v5;
                let a2907 = v2;
                nat_mul(a2906, a2907)?
            };
            let a2914 = {
                let a2911 = {
                    let a2908 = v4;
                    let a2909 = v1;
                    let a2910 = 0u64;
                    nat_rem(a2908, a2909, a2910)
                };
                let a2912 = v3;
                nat_mul(a2911, a2912)?
            };
            nat_add(a2913, a2914)?
        };
        let a2915 = v5;
        let a2916 = 0u64;
        if nat_eq(a2915, a2916) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2925 = {
                    let a2919 = {
                        let a2917 = v5;
                        let a2918 = 1u64;
                        nat_sub(a2917, a2918)
                    };
                    let a2920 = v2;
                    nat_mul(a2919, a2920)?
                };
                let a2926 = {
                    let a2923 = {
                        let a2921 = v1;
                        let a2922 = 1u64;
                        nat_sub(a2921, a2922)
                    };
                    let a2924 = v3;
                    nat_mul(a2923, a2924)?
                };
                nat_add(a2925, a2926)?
            };
            let a2927 = v6;
            let a2928 = v7;
            if nat_lt(a2927, a2928) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f145(v0: Adt3, v1: u64) -> bool {
    let a2929 = v1;
    let a2930 = 0u64;
    if !nat_eq(a2929, a2930) {
        true
    } else {
        let a2932 = match v0 {
            Adt3::C0(_, _, _, _, h2931, _, _, _) => {
                h2931
            }
        };
        let a2933 = 4u64;
        nat_eq(a2932, a2933)
    }
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2934 = f161(v0, v1);
    match m2934 {
        None => {
            if v5 {
                let m2935 = f162(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2935 {
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
                let a2936 = 4u64;
                let a2937 = v28;
                nat_le(a2936, a2937)
            };
            let a2938 = v2;
            let a2939 = 0u64;
            if if if if if nat_eq(a2938, a2939) {
                let a2940 = v4;
                let a2941 = 0u64;
                nat_eq(a2940, a2941)
            } else {
                false
            } {
                let a2942 = v6;
                let a2943 = v14;
                if nat_eq(a2942, a2943) {
                    let a2944 = v7;
                    let a2945 = v15;
                    nat_eq(a2944, a2945)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2946 = v8;
                let a2947 = v16;
                if nat_eq(a2946, a2947) {
                    let a2948 = v9;
                    let a2949 = v17;
                    nat_eq(a2948, a2949)
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
                        let a2950 = v3;
                        let a2951 = 0u64;
                        nat_eq(a2950, a2951)
                    } else {
                        false
                    } {
                        f96(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2956 = {
                        let a2952 = v6;
                        let a2953 = v7;
                        nat_mul(a2952, a2953)?
                    };
                    let a2957 = {
                        let a2954 = v8;
                        let a2955 = v9;
                        nat_mul(a2954, a2955)?
                    };
                    nat_mul(a2956, a2957)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f147(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2978 = {
        let a2972 = {
            let a2966 = {
                let a2960 = v1;
                let a2961 = if v5 {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2958, _, _, _) => {
                            h2958
                        }
                    })
                } else {
                    f141(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2959, _, _, _) => {
                            h2959
                        }
                    })
                };
                nat_mul(a2960, a2961)?
            };
            let a2967 = {
                let a2964 = v2;
                let a2965 = if v5 {
                    f142(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2962, _, _) => {
                            h2962
                        }
                    })
                } else {
                    f141(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2963, _, _) => {
                            h2963
                        }
                    })
                };
                nat_mul(a2964, a2965)?
            };
            nat_add(a2966, a2967)?
        };
        let a2973 = {
            let a2970 = v3;
            let a2971 = if v5 {
                f142(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2968, _) => {
                        h2968
                    }
                })
            } else {
                f141(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2969, _) => {
                        h2969
                    }
                })
            };
            nat_mul(a2970, a2971)?
        };
        nat_add(a2972, a2973)?
    };
    let a2979 = {
        let a2976 = v4;
        let a2977 = if v5 {
            f142(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2974) => {
                    h2974
                }
            })
        } else {
            f141(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2975) => {
                    h2975
                }
            })
        };
        nat_mul(a2976, a2977)?
    };
    nat_add(a2978, a2979)
}

pub fn f148(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f149(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2980 = v1;
    let a2981 = 0u64;
    if nat_eq(a2980, a2981) {
        Ok(Adt5::C4({
            let a2982 = v3;
            let a2983 = 0u64;
            if nat_eq(a2982, a2983) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f57(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2984 = v3;
            let a2985 = 0u64;
            if nat_eq(a2984, a2985) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f150(v0: u64, v1: u64) -> R<u64> {
    let a2986 = v0;
    let a2987 = f85(v1)?;
    let a2988 = 0u64;
    Ok(nat_quot(a2986, a2987, a2988))
}

pub fn f151(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f150(v0, v1)?;
    let v3: u64 = {
        let a2991 = {
            let a2989 = f121(v0, v1)?;
            let a2990 = 2u64;
            nat_sub(a2989, a2990)
        };
        let a2992 = 2u64;
        let a2993 = 0u64;
        nat_quot(a2991, a2992, a2993)
    };
    let v4: u64 = {
        let a2994 = v2;
        let a2995 = v1;
        nat_mul(a2994, a2995)?
    };
    if f124(v0, v1)? {
        let a2996 = v4;
        let a2997 = v3;
        nat_add(a2996, a2997)
    } else {
        let a3009 = {
            let a3005 = v4;
            let a3006 = {
                let a3003 = {
                    let a2998 = v1;
                    let a2999 = 1u64;
                    nat_sub(a2998, a2999)
                };
                let a3004 = {
                    let a3000 = v2;
                    let a3001 = v1;
                    let a3002 = 0u64;
                    nat_rem(a3000, a3001, a3002)
                };
                nat_mul(a3003, a3004)?
            };
            nat_sub(a3005, a3006)
        };
        let a3010 = {
            let a3007 = v3;
            let a3008 = v1;
            nat_mul(a3007, a3008)?
        };
        nat_add(a3009, a3010)
    }
}

pub fn f152(v0: u64) -> bool {
    let a3011 = v0;
    let a3012 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3011, a3012) {
        true
    } else {
        let a3013 = v0;
        let a3014 = 1u64;
        nat_eq(a3013, a3014)
    } {
        true
    } else {
        let a3015 = v0;
        let a3016 = 2u64;
        nat_eq(a3015, a3016)
    } {
        true
    } else {
        let a3017 = v0;
        let a3018 = 3u64;
        nat_eq(a3017, a3018)
    } {
        true
    } else {
        let a3019 = v0;
        let a3020 = 6u64;
        nat_eq(a3019, a3020)
    } {
        true
    } else {
        let a3021 = v0;
        let a3022 = 7u64;
        nat_eq(a3021, a3022)
    } {
        true
    } else {
        let a3023 = v0;
        let a3024 = 8u64;
        nat_eq(a3023, a3024)
    } {
        true
    } else {
        let a3025 = v0;
        let a3026 = 10u64;
        nat_eq(a3025, a3026)
    } {
        true
    } else {
        let a3027 = v0;
        let a3028 = 11u64;
        nat_eq(a3027, a3028)
    } {
        true
    } else {
        let a3029 = v0;
        let a3030 = 12u64;
        nat_eq(a3029, a3030)
    } {
        true
    } else {
        let a3031 = v0;
        let a3032 = 13u64;
        nat_eq(a3031, a3032)
    } {
        true
    } else {
        let a3033 = v0;
        let a3034 = 14u64;
        nat_eq(a3033, a3034)
    } {
        true
    } else {
        let a3035 = v0;
        let a3036 = 16u64;
        nat_eq(a3035, a3036)
    } {
        true
    } else {
        let a3037 = v0;
        let a3038 = 17u64;
        nat_eq(a3037, a3038)
    } {
        true
    } else {
        let a3039 = v0;
        let a3040 = 18u64;
        nat_eq(a3039, a3040)
    } {
        true
    } else {
        let a3041 = v0;
        let a3042 = 19u64;
        nat_eq(a3041, a3042)
    } {
        true
    } else {
        let a3043 = v0;
        let a3044 = 20u64;
        nat_eq(a3043, a3044)
    } {
        true
    } else {
        let a3045 = v0;
        let a3046 = 21u64;
        nat_eq(a3045, a3046)
    } {
        true
    } else {
        let a3047 = v0;
        let a3048 = 22u64;
        nat_eq(a3047, a3048)
    } {
        true
    } else {
        let a3049 = v0;
        let a3050 = 23u64;
        nat_eq(a3049, a3050)
    } {
        true
    } else {
        let a3051 = v0;
        let a3052 = 26u64;
        nat_eq(a3051, a3052)
    } {
        true
    } else {
        let a3053 = v0;
        let a3054 = 29u64;
        nat_eq(a3053, a3054)
    } {
        true
    } else {
        let a3055 = v0;
        let a3056 = 30u64;
        nat_eq(a3055, a3056)
    } {
        true
    } else {
        let a3057 = v0;
        let a3058 = 34u64;
        nat_eq(a3057, a3058)
    } {
        true
    } else {
        let a3059 = v0;
        let a3060 = 35u64;
        nat_eq(a3059, a3060)
    } {
        true
    } else {
        let a3061 = v0;
        let a3062 = 39u64;
        nat_eq(a3061, a3062)
    } {
        true
    } else {
        let a3063 = v0;
        let a3064 = 40u64;
        nat_eq(a3063, a3064)
    } {
        true
    } else {
        let a3065 = v0;
        let a3066 = 41u64;
        nat_eq(a3065, a3066)
    } {
        true
    } else {
        let a3067 = v0;
        let a3068 = 42u64;
        nat_eq(a3067, a3068)
    } {
        true
    } else {
        let a3069 = v0;
        let a3070 = 43u64;
        nat_eq(a3069, a3070)
    }
}

pub fn f153(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3071 = v0;
    let a3072 = 0u64;
    if if if nat_eq(a3071, a3072) {
        true
    } else {
        let a3073 = v1;
        let a3074 = 0u64;
        nat_eq(a3073, a3074)
    } {
        true
    } else {
        let a3075 = v2;
        let a3076 = 0u64;
        nat_eq(a3075, a3076)
    } {
        Ok(0u64)
    } else {
        let a3079 = {
            let a3077 = v0;
            let a3078 = v1;
            nat_mul(a3077, a3078)?
        };
        let a3080 = v2;
        nat_mul(a3079, a3080)
    }
}

pub fn f154(v0: u64, v1: u64) -> Adt8 {
    let a3081 = v0;
    let a3082 = 0u64;
    if nat_eq(a3081, a3082) {
        Adt8::C0(0u64, v1)
    } else {
        let a3083 = v0;
        let a3084 = 1u64;
        if nat_eq(a3083, a3084) {
            Adt8::C1(v1)
        } else {
            let a3085 = v0;
            let a3086 = 2u64;
            if nat_eq(a3085, a3086) {
                Adt8::C2(v1)
            } else {
                let a3087 = v0;
                let a3088 = 3u64;
                if nat_eq(a3087, a3088) {
                    Adt8::C3(v1)
                } else {
                    let a3089 = v0;
                    let a3090 = 13u64;
                    if nat_lt(a3089, a3090) {
                        Adt8::C0({
                            let a3091 = v0;
                            let a3092 = 3u64;
                            nat_sub(a3091, a3092)
                        }, v1)
                    } else {
                        Adt8::C4
                    }
                }
            }
        }
    }
}

pub fn f155(v0: u64, v1: u64) -> Adt9 {
    let a3093 = v0;
    let a3094 = 1u64;
    if nat_eq(a3093, a3094) {
        Adt9::C0(v1)
    } else {
        let a3095 = v0;
        let a3096 = 3u64;
        if if nat_eq(a3095, a3096) {
            true
        } else {
            let a3097 = v0;
            let a3098 = 6u64;
            nat_eq(a3097, a3098)
        } {
            Adt9::C1(v1)
        } else {
            Adt9::C2
        }
    }
}

pub fn f156(v0: u64) -> u64 {
    let m3099 = f95(v0);
    match m3099 {
        Adt10::C0 => {
            0u64
        }
        Adt10::C1(v1) => {
            let a3101 = match v1.clone() {
                Adt11::C0(_, h3100, _) => {
                    h3100
                }
            };
            let a3102 = 1u64;
            if if nat_eq(a3101, a3102) {
                let a3104 = match v1.clone() {
                    Adt11::C0(_, _, h3103) => {
                        h3103
                    }
                };
                let a3105 = 1u64;
                if if if nat_eq(a3104, a3105) {
                    true
                } else {
                    let a3107 = match v1.clone() {
                        Adt11::C0(_, _, h3106) => {
                            h3106
                        }
                    };
                    let a3108 = 2u64;
                    nat_eq(a3107, a3108)
                } {
                    true
                } else {
                    let a3110 = match v1.clone() {
                        Adt11::C0(_, _, h3109) => {
                            h3109
                        }
                    };
                    let a3111 = 4u64;
                    nat_eq(a3110, a3111)
                } {
                    true
                } else {
                    let a3113 = match v1.clone() {
                        Adt11::C0(_, _, h3112) => {
                            h3112
                        }
                    };
                    let a3114 = 8u64;
                    nat_eq(a3113, a3114)
                }
            } else {
                false
            } {
                match v1 {
                    Adt11::C0(_, _, h3115) => {
                        h3115
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f157(v0: u64) -> u64 {
    let a3116 = v0;
    let a3117 = 30u64;
    if nat_eq(a3116, a3117) {
        4u64
    } else {
        3u64
    }
}

pub fn f158(v0: u64) -> bool {
    let a3118 = 0u64;
    let a3119 = v0;
    if nat_lt(a3118, a3119) {
        let a3120 = v0;
        let a3121 = 2139095040u64;
        nat_le(a3120, a3121)
    } else {
        false
    }
}

pub fn f159(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f140(v1, v2, v3, v4) {
        f163(v0)
    } else {
        f152(v0)
    }
}

pub fn f160(v0: u64) -> u64 {
    let a3122 = v0;
    let a3123 = 26u64;
    if nat_eq(a3122, a3123) {
        26u64
    } else {
        0u64
    }
}

pub fn f161(v0: u64, v1: u64) -> Option<u64> {
    let a3124 = v0;
    let a3125 = 2u64;
    if nat_eq(a3124, a3125) {
        Some(0u64)
    } else {
        let a3126 = v0;
        let a3127 = 5u64;
        if nat_eq(a3126, a3127) {
            Some(1u64)
        } else {
            let a3128 = v0;
            let a3129 = 6u64;
            if nat_eq(a3128, a3129) {
                Some(2u64)
            } else {
                let a3130 = v0;
                let a3131 = 7u64;
                if nat_eq(a3130, a3131) {
                    Some(3u64)
                } else {
                    let a3132 = v0;
                    let a3133 = 9u64;
                    if nat_eq(a3132, a3133) {
                        Some(4u64)
                    } else {
                        let a3134 = v0;
                        let a3135 = 8u64;
                        if nat_eq(a3134, a3135) {
                            Some(5u64)
                        } else {
                            let a3136 = v0;
                            let a3137 = 89u64;
                            if nat_eq(a3136, a3137) {
                                let a3138 = v1;
                                let a3139 = 2u64;
                                if nat_eq(a3138, a3139) {
                                    Some(6u64)
                                } else {
                                    let a3140 = v1;
                                    let a3141 = 0u64;
                                    if nat_eq(a3140, a3141) {
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

pub fn f162(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt12> {
    let a3142 = v0;
    let a3143 = 28u64;
    if if if if nat_eq(a3142, a3143) {
        let a3144 = v1;
        let a3145 = 0u64;
        nat_eq(a3144, a3145)
    } else {
        false
    } {
        let a3146 = v2;
        let a3147 = 0u64;
        nat_eq(a3146, a3147)
    } else {
        false
    } {
        let a3148 = v3;
        let a3149 = 0u64;
        nat_eq(a3148, a3149)
    } else {
        false
    } {
        let a3150 = v4;
        let a3151 = v8;
        if if if if if if if if nat_eq(a3150, a3151) {
            let a3152 = v5;
            let a3153 = v12;
            nat_eq(a3152, a3153)
        } else {
            false
        } {
            let a3154 = v9;
            let a3155 = v13;
            nat_eq(a3154, a3155)
        } else {
            false
        } {
            let a3156 = v10;
            let a3157 = v14;
            nat_eq(a3156, a3157)
        } else {
            false
        } {
            let a3158 = v11;
            let a3159 = v15;
            nat_eq(a3158, a3159)
        } else {
            false
        } {
            let a3160 = 0u64;
            let a3161 = v6;
            nat_lt(a3160, a3161)
        } else {
            false
        } {
            let a3162 = 0u64;
            let a3163 = v7;
            nat_lt(a3162, a3163)
        } else {
            false
        } {
            let a3167 = {
                let a3164 = v10;
                let a3165 = v6;
                let a3166 = 1u64;
                nat_rem(a3164, a3165, a3166)
            };
            let a3168 = 0u64;
            if nat_eq(a3167, a3168) {
                let a3172 = {
                    let a3169 = v11;
                    let a3170 = v7;
                    let a3171 = 1u64;
                    nat_rem(a3169, a3170, a3171)
                };
                let a3173 = 0u64;
                nat_eq(a3172, a3173)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt12::C2(v5, v4, v9, v10, v11, {
                let a3174 = v10;
                let a3175 = v6;
                let a3176 = 0u64;
                nat_quot(a3174, a3175, a3176)
            }, {
                let a3177 = v11;
                let a3178 = v7;
                let a3179 = 0u64;
                nat_quot(a3177, a3178, a3179)
            }, {
                let a3180 = v5;
                let a3181 = v4;
                nat_mul(a3180, a3181)?
            }, {
                let a3182 = v4;
                let a3183 = v9;
                nat_mul(a3182, a3183)?
            }, {
                let a3184 = v5;
                let a3185 = v9;
                nat_mul(a3184, a3185)?
            }))
        } else {
            Ok(Adt12::C1)
        }
    } else {
        Ok(Adt12::C0)
    }
}

pub fn f163(v0: u64) -> bool {
    let m3186 = f95(v0);
    match m3186 {
        Adt10::C0 => {
            false
        }
        Adt10::C1(_) => {
            true
        }
    }
}
