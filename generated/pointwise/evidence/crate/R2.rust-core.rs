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
    C0(u64, Adt1, Adt2, Option<Adt2>, Adt2),
    C1(u64, Adt3, Adt3, Adt4),
    C2(u64, Adt3, Adt3, u64),
    C3(u64, Adt3, Adt3, Adt3, u64),
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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
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
    C0,
    C1(Adt8),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0,
    C1,
    C2(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
}

pub fn f0(v0: Adt0) -> R<Adt5> {
    let m1 = v0;
    match m1 {
        Adt0::C0(v1, v2, v3, v4, v5) => {
            let v6: u64 = match v2.clone() {
                Adt1::C0(h2, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2
                }
            };
            Ok(Adt5::C0({
                let a3 = v1;
                let a4 = 40u64;
                if nat_eq(a3, a4) {
                    f1(v3, v4, v5)?
                } else {
                    if f2(v1) {
                        f3(v1, v3, {
                            let m5 = v4;
                            match m5 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v5)?
                    } else {
                        let a6 = v1;
                        let a7 = 19u64;
                        if nat_eq(a6, a7) {
                            f4(v3, v5)?
                        } else {
                            let a8 = v1;
                            let a9 = 21u64;
                            if nat_eq(a8, a9) {
                                f5(v6, v3, v4, v5)?
                            } else {
                                let a10 = v1;
                                let a11 = 61u64;
                                if if if nat_eq(a10, a11) {
                                    true
                                } else {
                                    let a12 = v1;
                                    let a13 = 62u64;
                                    nat_eq(a12, a13)
                                } {
                                    true
                                } else {
                                    let a14 = v1;
                                    let a15 = 63u64;
                                    nat_eq(a14, a15)
                                } {
                                    f6(v1, v2, v3, v5)?
                                } else {
                                    let a16 = v1;
                                    let a17 = 4u64;
                                    if if nat_eq(a16, a17) {
                                        true
                                    } else {
                                        let a18 = v1;
                                        let a19 = 32u64;
                                        nat_eq(a18, a19)
                                    } {
                                        f7(v1, v2, v3, v4, v5)?
                                    } else {
                                        let a20 = v1;
                                        let a21 = 20u64;
                                        if nat_eq(a20, a21) {
                                            f8(v3, v5)?
                                        } else {
                                            let a22 = v1;
                                            let a23 = 42u64;
                                            if nat_eq(a22, a23) {
                                                f9(4u64, v3, v5)?
                                            } else {
                                                let a24 = v1;
                                                let a25 = 69u64;
                                                if nat_eq(a24, a25) {
                                                    let a26 = v6;
                                                    let a27 = 4u64;
                                                    if nat_lt(a26, a27) {
                                                        f9(v6, v3, v5)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a28 = v1;
                                                    let a29 = 70u64;
                                                    if nat_eq(a28, a29) {
                                                        f10(v6, v3, v5)?
                                                    } else {
                                                        let a30 = v1;
                                                        let a31 = 75u64;
                                                        if if nat_eq(a30, a31) {
                                                            true
                                                        } else {
                                                            let a32 = v1;
                                                            let a33 = 76u64;
                                                            nat_eq(a32, a33)
                                                        } {
                                                            f11({
                                                                let a34 = v1;
                                                                let a35 = 76u64;
                                                                nat_eq(a34, a35)
                                                            }, v2, v3, v5)?
                                                        } else {
                                                            let a36 = v1;
                                                            let a37 = 31u64;
                                                            if if if nat_eq(a36, a37) {
                                                                true
                                                            } else {
                                                                let a38 = v1;
                                                                let a39 = 47u64;
                                                                nat_eq(a38, a39)
                                                            } {
                                                                true
                                                            } else {
                                                                let a40 = v1;
                                                                let a41 = 68u64;
                                                                nat_eq(a40, a41)
                                                            } {
                                                                f12(v1, v2, v3, v5)?
                                                            } else {
                                                                let a42 = v1;
                                                                let a43 = 15u64;
                                                                if nat_eq(a42, a43) {
                                                                    f13(v3, v5)?
                                                                } else {
                                                                    let a44 = v1;
                                                                    let a45 = 17u64;
                                                                    if nat_eq(a44, a45) {
                                                                        f14(v3, v5)?
                                                                    } else {
                                                                        let a46 = v1;
                                                                        let a47 = 18u64;
                                                                        if nat_eq(a46, a47) {
                                                                            f15(v3, v4, v5)?
                                                                        } else {
                                                                            let a48 = v1;
                                                                            let a49 = 13u64;
                                                                            if if if nat_eq(a48, a49) {
                                                                                true
                                                                            } else {
                                                                                let a50 = v1;
                                                                                let a51 = 14u64;
                                                                                nat_eq(a50, a51)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a52 = v1;
                                                                                let a53 = 16u64;
                                                                                nat_eq(a52, a53)
                                                                            } {
                                                                                f16(v1, v3, v5)?
                                                                            } else {
                                                                                let m54 = f17(v1, v6);
                                                                                match m54 {
                                                                                    None => {
                                                                                        f18(v1, v6, v3, v4, v5)?
                                                                                    }
                                                                                    Some(v8) => {
                                                                                        f19(v8, v3, v5)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
        Adt0::C1(v9, v10, v11, v12) => {
            f20(v9, v10, v11, v12)
        }
        Adt0::C2(v13, v14, v15, v16) => {
            f21(v13, v14, v15, v16)
        }
        Adt0::C3(v17, v18, v19, v20, v21) => {
            f22(v17, v18, v19, v20, v21)
        }
    }
}

pub fn f1(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m55 = v1;
    match m55 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a57 = match v0.clone() {
                Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            };
            let a58 = 0u64;
            if if if if if if if if if if if if if nat_eq(a57, a58) {
                true
            } else {
                let a60 = match v0.clone() {
                    Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                        h59
                    }
                };
                let a61 = 1u64;
                nat_eq(a60, a61)
            } {
                let a63 = match v3.clone() {
                    Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                        h62
                    }
                };
                let a64 = 26u64;
                nat_eq(a63, a64)
            } else {
                false
            } {
                let a66 = match v2.clone() {
                    Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                        h65
                    }
                };
                let a67 = 0u64;
                nat_eq(a66, a67)
            } else {
                false
            } {
                let a70 = match v0.clone() {
                    Adt2::C0(_, h68, _, _, _, _, _, _, _, _, _) => {
                        h68
                    }
                };
                let a71 = match v2.clone() {
                    Adt2::C0(_, h69, _, _, _, _, _, _, _, _, _) => {
                        h69
                    }
                };
                nat_eq(a70, a71)
            } else {
                false
            } {
                let a74 = match v3.clone() {
                    Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                        h72
                    }
                };
                let a75 = match v0.clone() {
                    Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                        h73
                    }
                };
                nat_le(a74, a75)
            } else {
                false
            } {
                let a77 = match v0.clone() {
                    Adt2::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                        h76
                    }
                };
                let a78 = 1u64;
                nat_eq(a77, a78)
            } else {
                false
            } {
                let a80 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h79, _, _, _, _, _, _) => {
                        h79
                    }
                };
                let a81 = 1u64;
                nat_eq(a80, a81)
            } else {
                false
            } {
                let a83 = match v2.clone() {
                    Adt2::C0(_, _, _, h82, _, _, _, _, _, _, _) => {
                        h82
                    }
                };
                let a84 = 1u64;
                nat_eq(a83, a84)
            } else {
                false
            } {
                let a86 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h85, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a87 = 1u64;
                nat_eq(a86, a87)
            } else {
                false
            } {
                let a89 = match v3.clone() {
                    Adt2::C0(_, _, h88, _, _, _, _, _, _, _, _) => {
                        h88
                    }
                };
                let a90 = 1u64;
                nat_eq(a89, a90)
            } else {
                false
            } {
                let a92 = match v3.clone() {
                    Adt2::C0(_, _, _, h91, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a93 = 1u64;
                nat_eq(a92, a93)
            } else {
                false
            } {
                let a95 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h94, _, _, _, _, _, _) => {
                        h94
                    }
                };
                let a96 = 1u64;
                nat_eq(a95, a96)
            } else {
                false
            } {
                if f23(match v2.clone() {
                    Adt2::C0(_, h97, _, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h98, _, _, _, _, _, _, _, _) => {
                        h98
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                        h99
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h100, _, _, _, _, _, _) => {
                        h100
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a102 = match v3.clone() {
                        Adt2::C0(_, h101, _, _, _, _, _, _, _, _, _) => {
                            h101
                        }
                    };
                    let a103 = 0u64;
                    if nat_eq(a102, a103) {
                        Ok(Adt6::C4(f25(v2.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a105 = match v0.clone() {
                            Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                                h104
                            }
                        };
                        let a106 = 2147483647u64;
                        if if if if if if nat_le(a105, a106) {
                            let a108 = match v3.clone() {
                                Adt2::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                                    h107
                                }
                            };
                            let a109 = 2147483647u64;
                            nat_le(a108, a109)
                        } else {
                            false
                        } {
                            let a111 = 4u64;
                            let a112 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h110, _, _, _, _, _) => {
                                    h110
                                }
                            };
                            nat_le(a111, a112)
                        } else {
                            false
                        } {
                            let a118 = {
                                let a115 = match v2.clone() {
                                    Adt2::C0(_, h113, _, _, _, _, _, _, _, _, _) => {
                                        h113
                                    }
                                };
                                let a116 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h114, _, _, _, _, _) => {
                                        h114
                                    }
                                };
                                nat_mul(a115, a116)?
                            };
                            let a119 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h117, _, _, _, _) => {
                                    h117
                                }
                            };
                            nat_le(a118, a119)
                        } else {
                            false
                        } {
                            f26(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f26(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a133 = {
                                    let a123 = {
                                        let a121 = 2u64;
                                        let a122 = match v3.clone() {
                                            Adt2::C0(_, h120, _, _, _, _, _, _, _, _, _) => {
                                                h120
                                            }
                                        };
                                        nat_mul(a121, a122)?
                                    };
                                    let a124 = f25(v2.clone(), 1u64)?;
                                    nat_add(a123, a124)?
                                };
                                let a134 = {
                                    let a131 = match v3.clone() {
                                        Adt2::C0(_, h125, _, _, _, _, _, _, _, _, _) => {
                                            h125
                                        }
                                    };
                                    let a132 = {
                                        let a129 = 1u64;
                                        let a130 = {
                                            let a127 = 2u64;
                                            let a128 = match v0.clone() {
                                                Adt2::C0(_, h126, _, _, _, _, _, _, _, _, _) => {
                                                    h126
                                                }
                                            };
                                            nat_mul(a127, a128)?
                                        };
                                        nat_add(a129, a130)?
                                    };
                                    nat_mul(a131, a132)?
                                };
                                nat_add(a133, a134)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                                    h135
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h136, _, _, _, _, _, _, _, _, _) => {
                                    h136
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h137, _, _, _, _, _) => {
                                    h137
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h138, _, _, _, _) => {
                                    h138
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h139, _, _, _) => {
                                    h139
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h140, _, _) => {
                                    h140
                                }
                            }), f24(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                                    h141
                                }
                            }, f24(v3, 1u64))))
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

pub fn f2(v0: u64) -> bool {
    let a142 = v0;
    let a143 = 1u64;
    if if nat_eq(a142, a143) {
        true
    } else {
        let a144 = v0;
        let a145 = 33u64;
        nat_eq(a144, a145)
    } {
        true
    } else {
        let a146 = v0;
        let a147 = 34u64;
        nat_eq(a146, a147)
    }
}

pub fn f3(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a150 = match v1.clone() {
        Adt2::C0(h148, _, _, _, _, _, _, _, _, _, _) => {
            h148
        }
    };
    let a151 = match v3.clone() {
        Adt2::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
            h149
        }
    };
    if nat_eq(a150, a151) {
        f27(v0, v1, v2, v3)
    } else {
        if if if if f28(match v1.clone() {
            Adt2::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
                h152
            }
        }) {
            f28(match v3.clone() {
                Adt2::C0(h153, _, _, _, _, _, _, _, _, _, _) => {
                    h153
                }
            })
        } else {
            false
        } {
            let a154 = v0;
            let a155 = 33u64;
            if nat_eq(a154, a155) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a156 = f25(v1.clone(), 1u64)?;
            let a157 = f25(v3.clone(), 1u64)?;
            nat_eq(a156, a157)
        } else {
            false
        } {
            let m159 = f29(match v3.clone() {
                Adt2::C0(h158, _, _, _, _, _, _, _, _, _, _) => {
                    h158
                }
            });
            match m159 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f25(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h160) => {
                            h160
                        }
                    }, f24(v1.clone(), 1u64), f24(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h161, _, _, _, _, _, _, _, _, _, _) => {
                            h161
                        }
                    }, match v3 {
                        Adt2::C0(h162, _, _, _, _, _, _, _, _, _, _) => {
                            h162
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f4(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a164 = match v0.clone() {
        Adt2::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
            h163
        }
    };
    let a165 = 0u64;
    if if if if nat_eq(a164, a165) {
        true
    } else {
        let a167 = match v0.clone() {
            Adt2::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
                h166
            }
        };
        let a168 = 1u64;
        nat_eq(a167, a168)
    } {
        true
    } else {
        let a170 = match v0.clone() {
            Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a171 = 25u64;
        if nat_eq(a170, a171) {
            true
        } else {
            let a173 = match v0.clone() {
                Adt2::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
                    h172
                }
            };
            let a174 = 26u64;
            nat_eq(a173, a174)
        }
    } {
        true
    } else {
        let a176 = match v0.clone() {
            Adt2::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                h175
            }
        };
        let a177 = 30u64;
        nat_eq(a176, a177)
    } {
        let a180 = match v0.clone() {
            Adt2::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
                h178
            }
        };
        let a181 = match v1.clone() {
            Adt2::C0(h179, _, _, _, _, _, _, _, _, _, _) => {
                h179
            }
        };
        if if nat_eq(a180, a181) {
            f30(match v1.clone() {
                Adt2::C0(_, h182, _, _, _, _, _, _, _, _, _) => {
                    h182
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h183, _, _, _, _, _, _, _, _) => {
                    h183
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h184, _, _, _, _, _, _, _) => {
                    h184
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h185, _, _, _, _, _, _) => {
                    h185
                }
            }, match v0.clone() {
                Adt2::C0(_, h186, _, _, _, _, _, _, _, _, _) => {
                    h186
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h187, _, _, _, _, _, _, _, _) => {
                    h187
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h188, _, _, _, _, _, _, _) => {
                    h188
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h189, _, _, _, _, _, _) => {
                    h189
                }
            })
        } else {
            false
        } {
            let m191 = f29(match v0.clone() {
                Adt2::C0(h190, _, _, _, _, _, _, _, _, _, _) => {
                    h190
                }
            });
            match m191 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f25(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h192) => {
                            h192
                        }
                    }, f24(v0, 1u64), f24(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m193 = v2;
    match m193 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a194 = v0;
            let a195 = 4u64;
            if if if nat_lt(a194, a195) {
                let a198 = match v1.clone() {
                    Adt2::C0(h196, _, _, _, _, _, _, _, _, _, _) => {
                        h196
                    }
                };
                let a199 = match v4.clone() {
                    Adt2::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
                        h197
                    }
                };
                if nat_eq(a198, a199) {
                    let a202 = match v1.clone() {
                        Adt2::C0(h200, _, _, _, _, _, _, _, _, _, _) => {
                            h200
                        }
                    };
                    let a203 = match v3.clone() {
                        Adt2::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                            h201
                        }
                    };
                    nat_eq(a202, a203)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f31(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                        h204
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h205, _, _, _, _, _, _, _, _, _) => {
                        h205
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h206, _, _, _, _, _, _, _, _, _) => {
                        h206
                    }
                })? {
                    f31(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h207, _, _, _, _, _, _, _, _) => {
                            h207
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h208, _, _, _, _, _, _, _, _) => {
                            h208
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                            h209
                        }
                    })?
                } else {
                    false
                } {
                    f31(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h210, _, _, _, _, _, _, _) => {
                            h210
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h211, _, _, _, _, _, _, _) => {
                            h211
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                            h212
                        }
                    })?
                } else {
                    false
                } {
                    f31(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                            h213
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h214, _, _, _, _, _, _) => {
                            h214
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h215, _, _, _, _, _, _) => {
                            h215
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m217 = f29(match v1.clone() {
                    Adt2::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
                        h216
                    }
                });
                match m217 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h218, _) => {
                                h218
                            }
                        };
                        let a223 = {
                            let a220 = match v1.clone() {
                                Adt2::C0(_, h219, _, _, _, _, _, _, _, _, _) => {
                                    h219
                                }
                            };
                            let a221 = v6;
                            let a222 = 1u64;
                            nat_rem(a220, a221, a222)
                        };
                        let a224 = 0u64;
                        if if if nat_eq(a223, a224) {
                            let a229 = {
                                let a226 = match v4.clone() {
                                    Adt2::C0(_, h225, _, _, _, _, _, _, _, _, _) => {
                                        h225
                                    }
                                };
                                let a227 = v6;
                                let a228 = 1u64;
                                nat_rem(a226, a227, a228)
                            };
                            let a230 = 0u64;
                            nat_eq(a229, a230)
                        } else {
                            false
                        } {
                            let a235 = {
                                let a232 = match v3.clone() {
                                    Adt2::C0(_, h231, _, _, _, _, _, _, _, _, _) => {
                                        h231
                                    }
                                };
                                let a233 = v6;
                                let a234 = 1u64;
                                nat_rem(a232, a233, a234)
                            };
                            let a236 = 0u64;
                            nat_eq(a235, a236)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f25(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h237) => {
                                    h237
                                }
                            }, f24(v1, v6), f24(v3, v6), Adt4::C2(v0, f24(v4, v6))))
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

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a239 = match v2.clone() {
        Adt2::C0(h238, _, _, _, _, _, _, _, _, _, _) => {
            h238
        }
    };
    let a240 = 0u64;
    if if nat_eq(a239, a240) {
        let a242 = match v3.clone() {
            Adt2::C0(h241, _, _, _, _, _, _, _, _, _, _) => {
                h241
            }
        };
        let a243 = 0u64;
        nat_eq(a242, a243)
    } else {
        false
    } {
        let a244 = v0;
        let a245 = 61u64;
        if nat_eq(a244, a245) {
            let v4: bool = {
                let a247 = 0u64;
                let a248 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h246, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                nat_lt(a247, a248)
            };
            if if if if if if f32(match v2.clone() {
                Adt2::C0(_, h249, _, _, _, _, _, _, _, _, _) => {
                    h249
                }
            }, match v1.clone() {
                Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h250
                }
            }, match v1.clone() {
                Adt1::C0(_, h251, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h251
                }
            }, match v3.clone() {
                Adt2::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            })? {
                f32(match v2.clone() {
                    Adt2::C0(_, _, h253, _, _, _, _, _, _, _, _) => {
                        h253
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h254, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h254
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h255, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h255
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h256, _, _, _, _, _, _, _, _) => {
                        h256
                    }
                })?
            } else {
                false
            } {
                f32(match v2.clone() {
                    Adt2::C0(_, _, _, h257, _, _, _, _, _, _, _) => {
                        h257
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h258, _, _, _, _, _, _, _, _, _, _, _) => {
                        h258
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h259, _, _, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h260, _, _, _, _, _, _, _) => {
                        h260
                    }
                })?
            } else {
                false
            } {
                f32(match v2.clone() {
                    Adt2::C0(_, _, _, _, h261, _, _, _, _, _, _) => {
                        h261
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h262, _, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h263, _, _, _, _, _, _, _, _) => {
                        h263
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                        h264
                    }
                })?
            } else {
                false
            } {
                let a266 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h265, _, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 4294967295u64;
                nat_le(a266, a267)
            } else {
                false
            } {
                if v4 {
                    !f23(match v2.clone() {
                        Adt2::C0(_, h268, _, _, _, _, _, _, _, _, _) => {
                            h268
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
                            h269
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h270, _, _, _, _, _, _, _) => {
                            h270
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h271, _, _, _, _, _, _) => {
                            h271
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f25(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h272, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h272
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h273, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h273
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h274, _, _, _, _, _, _, _, _, _, _, _) => {
                        h274
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h275, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a276 = v0;
            let a277 = 62u64;
            if nat_eq(a276, a277) {
                let a279 = match v1.clone() {
                    Adt1::C0(h278, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h278
                    }
                };
                let a280 = 2147483648u64;
                if if if if if nat_lt(a279, a280) {
                    let a282 = match v1.clone() {
                        Adt1::C0(_, h281, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h281
                        }
                    };
                    let a283 = 2147483648u64;
                    nat_lt(a282, a283)
                } else {
                    false
                } {
                    let a286 = match v1.clone() {
                        Adt1::C0(h284, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h284
                        }
                    };
                    let a287 = match v2.clone() {
                        Adt2::C0(_, h285, _, _, _, _, _, _, _, _, _) => {
                            h285
                        }
                    };
                    nat_lt(a286, a287)
                } else {
                    false
                } {
                    let a290 = match v1.clone() {
                        Adt1::C0(_, h288, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h288
                        }
                    };
                    let a291 = match v2.clone() {
                        Adt2::C0(_, h289, _, _, _, _, _, _, _, _, _) => {
                            h289
                        }
                    };
                    nat_lt(a290, a291)
                } else {
                    false
                } {
                    let a294 = match v2.clone() {
                        Adt2::C0(_, _, h292, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    let a295 = match v3.clone() {
                        Adt2::C0(_, _, h293, _, _, _, _, _, _, _, _) => {
                            h293
                        }
                    };
                    if if if nat_eq(a294, a295) {
                        let a298 = match v2.clone() {
                            Adt2::C0(_, _, _, h296, _, _, _, _, _, _, _) => {
                                h296
                            }
                        };
                        let a299 = match v3.clone() {
                            Adt2::C0(_, _, _, h297, _, _, _, _, _, _, _) => {
                                h297
                            }
                        };
                        nat_eq(a298, a299)
                    } else {
                        false
                    } {
                        let a302 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h300, _, _, _, _, _, _) => {
                                h300
                            }
                        };
                        let a303 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        nat_eq(a302, a303)
                    } else {
                        false
                    } {
                        let a312 = match v3.clone() {
                            Adt2::C0(_, h304, _, _, _, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        let a313 = {
                            let a310 = {
                                let a307 = match v2.clone() {
                                    Adt2::C0(_, h305, _, _, _, _, _, _, _, _, _) => {
                                        h305
                                    }
                                };
                                let a308 = match v1.clone() {
                                    Adt1::C0(h306, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h306
                                    }
                                };
                                nat_add(a307, a308)?
                            };
                            let a311 = match v1.clone() {
                                Adt1::C0(_, h309, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h309
                                }
                            };
                            nat_add(a310, a311)?
                        };
                        nat_eq(a312, a313)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f25(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h314, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h314
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a315 = v0;
                let a316 = 63u64;
                if nat_eq(a315, a316) {
                    if if if if if f33(match v2.clone() {
                        Adt2::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                            h317
                        }
                    }, match v1.clone() {
                        Adt1::C0(h318, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h318
                        }
                    })? {
                        f33(match v2.clone() {
                            Adt2::C0(_, _, h319, _, _, _, _, _, _, _, _) => {
                                h319
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h320, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h320
                            }
                        })?
                    } else {
                        false
                    } {
                        f33(match v2.clone() {
                            Adt2::C0(_, _, _, h321, _, _, _, _, _, _, _) => {
                                h321
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h322, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h322
                            }
                        })?
                    } else {
                        false
                    } {
                        f33(match v2.clone() {
                            Adt2::C0(_, _, _, _, h323, _, _, _, _, _, _) => {
                                h323
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h324, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h324
                            }
                        })?
                    } else {
                        false
                    } {
                        let a327 = match v2.clone() {
                            Adt2::C0(_, h325, _, _, _, _, _, _, _, _, _) => {
                                h325
                            }
                        };
                        let a328 = match v3.clone() {
                            Adt2::C0(_, h326, _, _, _, _, _, _, _, _, _) => {
                                h326
                            }
                        };
                        if if if nat_eq(a327, a328) {
                            let a331 = match v2.clone() {
                                Adt2::C0(_, _, h329, _, _, _, _, _, _, _, _) => {
                                    h329
                                }
                            };
                            let a332 = match v3.clone() {
                                Adt2::C0(_, _, h330, _, _, _, _, _, _, _, _) => {
                                    h330
                                }
                            };
                            nat_eq(a331, a332)
                        } else {
                            false
                        } {
                            let a335 = match v2.clone() {
                                Adt2::C0(_, _, _, h333, _, _, _, _, _, _, _) => {
                                    h333
                                }
                            };
                            let a336 = match v3.clone() {
                                Adt2::C0(_, _, _, h334, _, _, _, _, _, _, _) => {
                                    h334
                                }
                            };
                            nat_eq(a335, a336)
                        } else {
                            false
                        } {
                            let a339 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h337, _, _, _, _, _, _) => {
                                    h337
                                }
                            };
                            let a340 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h338, _, _, _, _, _, _) => {
                                    h338
                                }
                            };
                            nat_eq(a339, a340)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f25(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h341, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h341
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h342, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h342
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h343, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h343
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h344, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h344
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

pub fn f7(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m345 = v3;
    match m345 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a346 = v0;
                let a347 = 4u64;
                nat_eq(a346, a347)
            };
            let v7: bool = {
                let a349 = 0u64;
                let a350 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h348, _, _, _, _, _, _, _, _, _, _, _) => {
                        h348
                    }
                };
                nat_lt(a349, a350)
            };
            let v8: u64 = f25(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h351, _, _, _, _, _, _, _, _, _) => {
                    h351
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h352, _, _, _, _, _, _, _, _) => {
                    h352
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h353, _, _, _, _, _, _, _) => {
                    h353
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h354, _, _, _, _, _, _) => {
                    h354
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h355
                }
            }, match v1.clone() {
                Adt1::C0(_, h356, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h356
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h357, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h357
                }
            });
            if if if if if if if if if if if if v6 {
                let a359 = match v2.clone() {
                    Adt2::C0(h358, _, _, _, _, _, _, _, _, _, _) => {
                        h358
                    }
                };
                let a360 = 0u64;
                if nat_eq(a359, a360) {
                    let a362 = match v5.clone() {
                        Adt2::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                            h361
                        }
                    };
                    let a363 = 0u64;
                    nat_eq(a362, a363)
                } else {
                    false
                }
            } else {
                let a365 = match v2.clone() {
                    Adt2::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
                        h364
                    }
                };
                let a366 = 0u64;
                if if nat_eq(a365, a366) {
                    true
                } else {
                    let a368 = match v2.clone() {
                        Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
                            h367
                        }
                    };
                    let a369 = 26u64;
                    nat_eq(a368, a369)
                } {
                    let a371 = match v5.clone() {
                        Adt2::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
                            h370
                        }
                    };
                    let a372 = 0u64;
                    if nat_eq(a371, a372) {
                        true
                    } else {
                        let a374 = match v5.clone() {
                            Adt2::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
                                h373
                            }
                        };
                        let a375 = 26u64;
                        nat_eq(a374, a375)
                    }
                } else {
                    false
                }
            } {
                let a378 = match v2.clone() {
                    Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                        h376
                    }
                };
                let a379 = match v4.clone() {
                    Adt2::C0(h377, _, _, _, _, _, _, _, _, _, _) => {
                        h377
                    }
                };
                nat_eq(a378, a379)
            } else {
                false
            } {
                f34(v2.clone(), v4.clone())
            } else {
                false
            } {
                f35(v2.clone())?
            } else {
                false
            } {
                f35(v4.clone())?
            } else {
                false
            } {
                let a381 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h380, _, _, _, _, _) => {
                        h380
                    }
                };
                let a382 = 4u64;
                nat_eq(a381, a382)
            } else {
                false
            } {
                let a383 = v8;
                let a384 = f25(v2.clone(), 1u64)?;
                nat_le(a383, a384)
            } else {
                false
            } {
                let a386 = match v1.clone() {
                    Adt1::C0(_, _, _, h385, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h385
                    }
                };
                let a387 = 1073741824u64;
                nat_lt(a386, a387)
            } else {
                false
            } {
                let a389 = match v1.clone() {
                    Adt1::C0(h388, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h388
                    }
                };
                let a390 = 4294967295u64;
                if if if if nat_le(a389, a390) {
                    let a392 = match v1.clone() {
                        Adt1::C0(_, h391, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h391
                        }
                    };
                    let a393 = 4294967295u64;
                    nat_le(a392, a393)
                } else {
                    false
                } {
                    let a395 = match v1.clone() {
                        Adt1::C0(_, _, h394, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h394
                        }
                    };
                    let a396 = 4294967295u64;
                    nat_le(a395, a396)
                } else {
                    false
                } {
                    let a398 = match v1.clone() {
                        Adt1::C0(_, _, _, h397, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h397
                        }
                    };
                    let a399 = 4294967295u64;
                    nat_le(a398, a399)
                } else {
                    false
                } {
                    let a401 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h400, _, _, _, _, _, _, _, _, _, _, _) => {
                            h400
                        }
                    };
                    let a402 = 4294967295u64;
                    nat_le(a401, a402)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a405 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h403) => {
                            h403
                        }
                    };
                    let a406 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h404) => {
                            h404
                        }
                    };
                    nat_eq(a405, a406)
                } else {
                    true
                }
            } else {
                false
            } {
                f36(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h407, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h407
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h408, _) => {
                        h408
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f25(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a409 = v10;
                    let a410 = v8;
                    nat_add(a409, a410)?
                }, 4u64, f24(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h411, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h411
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f8(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a413 = match v0.clone() {
        Adt2::C0(h412, _, _, _, _, _, _, _, _, _, _) => {
            h412
        }
    };
    let a414 = 0u64;
    if if if nat_eq(a413, a414) {
        let a416 = match v1.clone() {
            Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
                h415
            }
        };
        let a417 = 0u64;
        nat_eq(a416, a417)
    } else {
        false
    } {
        f30(match v0.clone() {
            Adt2::C0(_, h418, _, _, _, _, _, _, _, _, _) => {
                h418
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                h419
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h420, _, _, _, _, _, _, _) => {
                h420
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                h421
            }
        }, match v1.clone() {
            Adt2::C0(_, h422, _, _, _, _, _, _, _, _, _) => {
                h422
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h423, _, _, _, _, _, _, _, _) => {
                h423
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h424, _, _, _, _, _, _, _) => {
                h424
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h425, _, _, _, _, _, _) => {
                h425
            }
        })
    } else {
        false
    } {
        let v2: u64 = f25(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a426 = v2;
            let a427 = f25(v0.clone(), 1u64)?;
            nat_add(a426, a427)?
        }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f9(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a429 = match v1.clone() {
        Adt2::C0(h428, _, _, _, _, _, _, _, _, _, _) => {
            h428
        }
    };
    let a430 = 0u64;
    if if if nat_eq(a429, a430) {
        true
    } else {
        let a432 = match v1.clone() {
            Adt2::C0(h431, _, _, _, _, _, _, _, _, _, _) => {
                h431
            }
        };
        let a433 = 1u64;
        nat_eq(a432, a433)
    } {
        true
    } else {
        let a435 = match v1.clone() {
            Adt2::C0(h434, _, _, _, _, _, _, _, _, _, _) => {
                h434
            }
        };
        let a436 = 30u64;
        nat_eq(a435, a436)
    } {
        let a439 = match v1.clone() {
            Adt2::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a440 = match v2.clone() {
            Adt2::C0(h438, _, _, _, _, _, _, _, _, _, _) => {
                h438
            }
        };
        if if if if if nat_eq(a439, a440) {
            let a443 = match v1.clone() {
                Adt2::C0(_, h441, _, _, _, _, _, _, _, _, _) => {
                    h441
                }
            };
            let a444 = match v2.clone() {
                Adt2::C0(_, h442, _, _, _, _, _, _, _, _, _) => {
                    h442
                }
            };
            nat_eq(a443, a444)
        } else {
            false
        } {
            let a447 = match v1.clone() {
                Adt2::C0(_, _, _, h445, _, _, _, _, _, _, _) => {
                    h445
                }
            };
            let a448 = match v2.clone() {
                Adt2::C0(_, _, _, h446, _, _, _, _, _, _, _) => {
                    h446
                }
            };
            nat_eq(a447, a448)
        } else {
            false
        } {
            let a451 = match v1.clone() {
                Adt2::C0(_, _, _, _, h449, _, _, _, _, _, _) => {
                    h449
                }
            };
            let a452 = match v2.clone() {
                Adt2::C0(_, _, _, _, h450, _, _, _, _, _, _) => {
                    h450
                }
            };
            nat_eq(a451, a452)
        } else {
            false
        } {
            let a453 = v0;
            let a454 = 4u64;
            if nat_eq(a453, a454) {
                let a456 = match v1.clone() {
                    Adt2::C0(_, _, h455, _, _, _, _, _, _, _, _) => {
                        h455
                    }
                };
                let a457 = 1u64;
                if nat_eq(a456, a457) {
                    let a460 = match v1.clone() {
                        Adt2::C0(_, h458, _, _, _, _, _, _, _, _, _) => {
                            h458
                        }
                    };
                    let a461 = match v2.clone() {
                        Adt2::C0(_, _, h459, _, _, _, _, _, _, _, _) => {
                            h459
                        }
                    };
                    nat_eq(a460, a461)
                } else {
                    false
                }
            } else {
                let a462 = v0;
                let a463 = 4u64;
                if if nat_lt(a462, a463) {
                    let a466 = match v1.clone() {
                        Adt2::C0(_, h464, _, _, _, _, _, _, _, _, _) => {
                            h464
                        }
                    };
                    let a467 = match v1.clone() {
                        Adt2::C0(_, _, h465, _, _, _, _, _, _, _, _) => {
                            h465
                        }
                    };
                    nat_eq(a466, a467)
                } else {
                    false
                } {
                    let a470 = match v1.clone() {
                        Adt2::C0(_, _, h468, _, _, _, _, _, _, _, _) => {
                            h468
                        }
                    };
                    let a471 = match v2.clone() {
                        Adt2::C0(_, _, h469, _, _, _, _, _, _, _, _) => {
                            h469
                        }
                    };
                    nat_eq(a470, a471)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m473 = f29(match v1.clone() {
                Adt2::C0(h472, _, _, _, _, _, _, _, _, _, _) => {
                    h472
                }
            });
            match m473 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f25(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h474) => {
                            h474
                        }
                    }, f24(v1, 1u64), f24(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f37(match v1.clone() {
        Adt2::C0(h475, _, _, _, _, _, _, _, _, _, _) => {
            h475
        }
    }, v0) {
        let a478 = match v1.clone() {
            Adt2::C0(h476, _, _, _, _, _, _, _, _, _, _) => {
                h476
            }
        };
        let a479 = match v2.clone() {
            Adt2::C0(h477, _, _, _, _, _, _, _, _, _, _) => {
                h477
            }
        };
        nat_eq(a478, a479)
    } else {
        false
    } {
        let a482 = match v1.clone() {
            Adt2::C0(_, h480, _, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a483 = match v2.clone() {
            Adt2::C0(_, h481, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        };
        if if if nat_eq(a482, a483) {
            let a486 = match v1.clone() {
                Adt2::C0(_, _, h484, _, _, _, _, _, _, _, _) => {
                    h484
                }
            };
            let a487 = match v2.clone() {
                Adt2::C0(_, _, h485, _, _, _, _, _, _, _, _) => {
                    h485
                }
            };
            nat_eq(a486, a487)
        } else {
            false
        } {
            let a490 = match v1.clone() {
                Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                    h488
                }
            };
            let a491 = match v2.clone() {
                Adt2::C0(_, _, _, h489, _, _, _, _, _, _, _) => {
                    h489
                }
            };
            nat_eq(a490, a491)
        } else {
            false
        } {
            let a494 = match v1.clone() {
                Adt2::C0(_, _, _, _, h492, _, _, _, _, _, _) => {
                    h492
                }
            };
            let a495 = match v2.clone() {
                Adt2::C0(_, _, _, _, h493, _, _, _, _, _, _) => {
                    h493
                }
            };
            nat_eq(a494, a495)
        } else {
            false
        }
    } else {
        false
    } {
        let m497 = f29(match v1.clone() {
            Adt2::C0(h496, _, _, _, _, _, _, _, _, _, _) => {
                h496
            }
        });
        match m497 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f25(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h498) => {
                        h498
                    }
                }, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                        h499
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h500, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h500
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h501, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h501
            }
        }
    };
    let a502 = 0u64;
    let a503 = v4;
    if if nat_lt(a502, a503) {
        let a504 = v4;
        let a505 = 2147483648u64;
        nat_lt(a504, a505)
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
        let v7: u64 = f38(match v5.clone() {
            Adt2::C0(_, _, h506, _, _, _, _, _, _, _, _) => {
                h506
            }
        }, v4)?;
        let v8: u64 = f38(match v5.clone() {
            Adt2::C0(_, _, _, h507, _, _, _, _, _, _, _) => {
                h507
            }
        }, v4)?;
        let a509 = match v5.clone() {
            Adt2::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                h508
            }
        };
        let a510 = 0u64;
        if if if if if nat_eq(a509, a510) {
            let a512 = match v6.clone() {
                Adt2::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                    h511
                }
            };
            let a513 = 0u64;
            nat_eq(a512, a513)
        } else {
            false
        } {
            let a515 = match v5.clone() {
                Adt2::C0(_, _, _, _, h514, _, _, _, _, _, _) => {
                    h514
                }
            };
            let a516 = 1u64;
            nat_eq(a515, a516)
        } else {
            false
        } {
            let a519 = match v5.clone() {
                Adt2::C0(_, h517, _, _, _, _, _, _, _, _, _) => {
                    h517
                }
            };
            let a520 = match v6.clone() {
                Adt2::C0(_, h518, _, _, _, _, _, _, _, _, _) => {
                    h518
                }
            };
            nat_eq(a519, a520)
        } else {
            false
        } {
            if v0 {
                let a522 = match v5.clone() {
                    Adt2::C0(_, _, h521, _, _, _, _, _, _, _, _) => {
                        h521
                    }
                };
                let a523 = 2147483648u64;
                if if if nat_lt(a522, a523) {
                    let a525 = match v5.clone() {
                        Adt2::C0(_, _, _, h524, _, _, _, _, _, _, _) => {
                            h524
                        }
                    };
                    let a526 = 2147483648u64;
                    nat_lt(a525, a526)
                } else {
                    false
                } {
                    let a529 = {
                        let a527 = v7;
                        let a528 = v8;
                        nat_mul(a527, a528)?
                    };
                    let a530 = 2147483648u64;
                    nat_le(a529, a530)
                } else {
                    false
                } {
                    f39(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a532 = match v1.clone() {
                    Adt1::C0(h531, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h531
                    }
                };
                let a533 = v7;
                if if if if nat_eq(a532, a533) {
                    let a535 = match v1 {
                        Adt1::C0(_, h534, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h534
                        }
                    };
                    let a536 = v8;
                    nat_eq(a535, a536)
                } else {
                    false
                } {
                    f40(v7, v8, v4)?
                } else {
                    false
                } {
                    let a538 = match v6.clone() {
                        Adt2::C0(_, _, h537, _, _, _, _, _, _, _, _) => {
                            h537
                        }
                    };
                    let a539 = v4;
                    if nat_eq(a538, a539) {
                        let a541 = match v6.clone() {
                            Adt2::C0(_, _, _, h540, _, _, _, _, _, _, _) => {
                                h540
                            }
                        };
                        let a542 = v4;
                        nat_eq(a541, a542)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a546 = match v6 {
                        Adt2::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                            h543
                        }
                    };
                    let a547 = {
                        let a544 = v7;
                        let a545 = v8;
                        nat_mul(a544, a545)?
                    };
                    nat_eq(a546, a547)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f25(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a549 = match v2.clone() {
        Adt2::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
            h548
        }
    };
    let a550 = 0u64;
    if if if if if nat_eq(a549, a550) {
        true
    } else {
        let a552 = match v2.clone() {
            Adt2::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
                h551
            }
        };
        let a553 = 1u64;
        nat_eq(a552, a553)
    } {
        true
    } else {
        let a555 = match v2.clone() {
            Adt2::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
                h554
            }
        };
        let a556 = 30u64;
        nat_eq(a555, a556)
    } {
        let a559 = match v2.clone() {
            Adt2::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a560 = match v3.clone() {
            Adt2::C0(h558, _, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        nat_eq(a559, a560)
    } else {
        false
    } {
        if if f34(v2.clone(), v3.clone()) {
            let a562 = match v1.clone() {
                Adt1::C0(h561, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h561
                }
            };
            let a563 = 4294967296u64;
            nat_lt(a562, a563)
        } else {
            false
        } {
            let a565 = match v1.clone() {
                Adt1::C0(_, h564, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h564
                }
            };
            let a566 = 4294967296u64;
            nat_lt(a565, a566)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f25(v3.clone(), 1u64)?, {
            let a568 = match v2.clone() {
                Adt2::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                    h567
                }
            };
            let a569 = 0u64;
            if nat_eq(a568, a569) {
                4u64
            } else {
                2u64
            }
        }, f24(v2.clone(), 1u64), f24(v3, 1u64), Adt4::C13({
            let a570 = v0;
            let a571 = 31u64;
            if nat_eq(a570, a571) {
                0u64
            } else {
                let a572 = v0;
                let a573 = 47u64;
                if nat_eq(a572, a573) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h574, _, _, _, _, _, _, _, _, _, _) => {
                h574
            }
        }, match v1.clone() {
            Adt1::C0(h575, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h575
            }
        }, match v1 {
            Adt1::C0(_, h576, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h576
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a578 = match v0.clone() {
        Adt2::C0(h577, _, _, _, _, _, _, _, _, _, _) => {
            h577
        }
    };
    let a579 = 0u64;
    if if if nat_eq(a578, a579) {
        let a581 = match v1.clone() {
            Adt2::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
                h580
            }
        };
        let a582 = 0u64;
        nat_eq(a581, a582)
    } else {
        false
    } {
        f34(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f25(v1.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f14(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a584 = match v0.clone() {
        Adt2::C0(h583, _, _, _, _, _, _, _, _, _, _) => {
            h583
        }
    };
    let a585 = 0u64;
    if if nat_eq(a584, a585) {
        let a587 = match v1.clone() {
            Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
                h586
            }
        };
        let a588 = 26u64;
        nat_eq(a587, a588)
    } else {
        false
    } {
        let a590 = match v0.clone() {
            Adt2::C0(_, _, _, h589, _, _, _, _, _, _, _) => {
                h589
            }
        };
        let a591 = 1u64;
        if if if if if if if nat_eq(a590, a591) {
            let a593 = match v0.clone() {
                Adt2::C0(_, _, _, _, h592, _, _, _, _, _, _) => {
                    h592
                }
            };
            let a594 = 1u64;
            nat_eq(a593, a594)
        } else {
            false
        } {
            let a596 = match v0.clone() {
                Adt2::C0(_, h595, _, _, _, _, _, _, _, _, _) => {
                    h595
                }
            };
            let a597 = 2147483647u64;
            nat_le(a596, a597)
        } else {
            false
        } {
            let a600 = match v1.clone() {
                Adt2::C0(_, h598, _, _, _, _, _, _, _, _, _) => {
                    h598
                }
            };
            let a601 = match v0.clone() {
                Adt2::C0(_, _, h599, _, _, _, _, _, _, _, _) => {
                    h599
                }
            };
            nat_eq(a600, a601)
        } else {
            false
        } {
            let a603 = match v1.clone() {
                Adt2::C0(_, _, h602, _, _, _, _, _, _, _, _) => {
                    h602
                }
            };
            let a604 = 1u64;
            nat_eq(a603, a604)
        } else {
            false
        } {
            let a606 = match v1.clone() {
                Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                    h605
                }
            };
            let a607 = 1u64;
            nat_eq(a606, a607)
        } else {
            false
        } {
            let a609 = match v1.clone() {
                Adt2::C0(_, _, _, _, h608, _, _, _, _, _, _) => {
                    h608
                }
            };
            let a610 = 1u64;
            nat_eq(a609, a610)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a613 = match v0.clone() {
                    Adt2::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                        h611
                    }
                };
                let a614 = f41(match v0.clone() {
                    Adt2::C0(_, h612, _, _, _, _, _, _, _, _, _) => {
                        h612
                    }
                })?;
                nat_mul(a613, a614)?
            }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f15(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m615 = v1;
    match m615 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f42(f24(v0.clone(), 1u64))?;
            let a617 = match v0.clone() {
                Adt2::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
                    h616
                }
            };
            let a618 = 26u64;
            if if if nat_eq(a617, a618) {
                let a620 = match v3.clone() {
                    Adt2::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                        h619
                    }
                };
                let a621 = 26u64;
                nat_eq(a620, a621)
            } else {
                false
            } {
                let a623 = match v2.clone() {
                    Adt2::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                        h622
                    }
                };
                let a624 = 27u64;
                nat_eq(a623, a624)
            } else {
                false
            } {
                if if if if if if f34(v0.clone(), v3.clone()) {
                    let a626 = match v2.clone() {
                        Adt2::C0(_, h625, _, _, _, _, _, _, _, _, _) => {
                            h625
                        }
                    };
                    let a627 = 1u64;
                    nat_eq(a626, a627)
                } else {
                    false
                } {
                    let a629 = match v2.clone() {
                        Adt2::C0(_, _, h628, _, _, _, _, _, _, _, _) => {
                            h628
                        }
                    };
                    let a630 = 1u64;
                    nat_eq(a629, a630)
                } else {
                    false
                } {
                    let a632 = match v2.clone() {
                        Adt2::C0(_, _, _, h631, _, _, _, _, _, _, _) => {
                            h631
                        }
                    };
                    let a633 = 1u64;
                    nat_eq(a632, a633)
                } else {
                    false
                } {
                    let a635 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h634, _, _, _, _, _, _) => {
                            h634
                        }
                    };
                    let a636 = 1u64;
                    nat_eq(a635, a636)
                } else {
                    false
                } {
                    let a637 = v4;
                    let a638 = 9223372036854775808u64;
                    nat_lt(a637, a638)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a641 = 2u64;
                        let a642 = {
                            let a639 = v4;
                            let a640 = 2u64;
                            nat_mul(a639, a640)?
                        };
                        nat_add(a641, a642)?
                    }, 8u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C18(v4, f24(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f16(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h643, _, _, _, _, _, _, _, _, _, _) => {
            h643
        }
    };
    let a644 = v3;
    let a645 = 0u64;
    if if nat_eq(a644, a645) {
        true
    } else {
        let a646 = v0;
        let a647 = 13u64;
        if nat_eq(a646, a647) {
            let a648 = v3;
            let a649 = 1u64;
            if nat_eq(a648, a649) {
                true
            } else {
                let a650 = v3;
                let a651 = 30u64;
                nat_eq(a650, a651)
            }
        } else {
            false
        }
    } {
        let a653 = v3;
        let a654 = match v2.clone() {
            Adt2::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        };
        if if nat_eq(a653, a654) {
            let a655 = v0;
            let a656 = 13u64;
            if nat_eq(a655, a656) {
                let a658 = match v2.clone() {
                    Adt2::C0(_, h657, _, _, _, _, _, _, _, _, _) => {
                        h657
                    }
                };
                let a659 = 1u64;
                if if if nat_eq(a658, a659) {
                    let a661 = match v2.clone() {
                        Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                            h660
                        }
                    };
                    let a662 = 1u64;
                    nat_eq(a661, a662)
                } else {
                    false
                } {
                    let a664 = match v2.clone() {
                        Adt2::C0(_, _, _, h663, _, _, _, _, _, _, _) => {
                            h663
                        }
                    };
                    let a665 = 1u64;
                    nat_eq(a664, a665)
                } else {
                    false
                } {
                    let a667 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h666, _, _, _, _, _, _) => {
                            h666
                        }
                    };
                    let a668 = 1u64;
                    nat_eq(a667, a668)
                } else {
                    false
                }
            } else {
                let a670 = match v2.clone() {
                    Adt2::C0(_, h669, _, _, _, _, _, _, _, _, _) => {
                        h669
                    }
                };
                let a671 = 1u64;
                if if if nat_eq(a670, a671) {
                    let a674 = match v1.clone() {
                        Adt2::C0(_, _, h672, _, _, _, _, _, _, _, _) => {
                            h672
                        }
                    };
                    let a675 = match v2.clone() {
                        Adt2::C0(_, _, h673, _, _, _, _, _, _, _, _) => {
                            h673
                        }
                    };
                    nat_eq(a674, a675)
                } else {
                    false
                } {
                    let a678 = match v1.clone() {
                        Adt2::C0(_, _, _, h676, _, _, _, _, _, _, _) => {
                            h676
                        }
                    };
                    let a679 = match v2.clone() {
                        Adt2::C0(_, _, _, h677, _, _, _, _, _, _, _) => {
                            h677
                        }
                    };
                    nat_eq(a678, a679)
                } else {
                    false
                } {
                    let a682 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h680, _, _, _, _, _, _) => {
                            h680
                        }
                    };
                    let a683 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h681, _, _, _, _, _, _) => {
                            h681
                        }
                    };
                    nat_eq(a682, a683)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f24(v1, 1u64);
            Ok(Adt6::C4(f43(v4.clone(), v0, v3)?, {
                let a684 = v3;
                let a685 = 0u64;
                if nat_eq(a684, a685) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f24(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f17(v0: u64, v1: u64) -> Option<u64> {
    let a686 = v0;
    let a687 = 8u64;
    if nat_eq(a686, a687) {
        Some(5u64)
    } else {
        let a688 = v0;
        let a689 = 9u64;
        if nat_eq(a688, a689) {
            Some(6u64)
        } else {
            let a690 = v0;
            let a691 = 89u64;
            if nat_eq(a690, a691) {
                let a692 = v1;
                let a693 = 0u64;
                if nat_eq(a692, a693) {
                    Some(0u64)
                } else {
                    let a694 = v1;
                    let a695 = 1u64;
                    if nat_eq(a694, a695) {
                        Some(1u64)
                    } else {
                        let a696 = v1;
                        let a697 = 2u64;
                        if nat_eq(a696, a697) {
                            Some(2u64)
                        } else {
                            let a698 = v1;
                            let a699 = 3u64;
                            if nat_eq(a698, a699) {
                                Some(3u64)
                            } else {
                                let a700 = v1;
                                let a701 = 6u64;
                                if nat_eq(a700, a701) {
                                    Some(4u64)
                                } else {
                                    let a702 = v1;
                                    let a703 = 12u64;
                                    if nat_eq(a702, a703) {
                                        Some(7u64)
                                    } else {
                                        let a704 = v1;
                                        let a705 = 11u64;
                                        if nat_eq(a704, a705) {
                                            Some(8u64)
                                        } else {
                                            let a706 = v1;
                                            let a707 = 18u64;
                                            if nat_eq(a706, a707) {
                                                Some(9u64)
                                            } else {
                                                let a708 = v1;
                                                let a709 = 19u64;
                                                if nat_eq(a708, a709) {
                                                    Some(10u64)
                                                } else {
                                                    let a710 = v1;
                                                    let a711 = 20u64;
                                                    if nat_eq(a710, a711) {
                                                        Some(11u64)
                                                    } else {
                                                        let a712 = v1;
                                                        let a713 = 21u64;
                                                        if nat_eq(a712, a713) {
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

pub fn f18(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m714 = f44(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m714 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f24(v2.clone(), 1u64), f24(v4, 1u64), Adt4::C10(v5, v6, {
                let m715 = v3;
                match m715 {
                    None => {
                        f24(v2, 1u64)
                    }
                    Some(v12) => {
                        f24(v12, 1u64)
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

pub fn f19(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a717 = match v1.clone() {
        Adt2::C0(h716, _, _, _, _, _, _, _, _, _, _) => {
            h716
        }
    };
    let a718 = 0u64;
    if if if if if nat_eq(a717, a718) {
        true
    } else {
        let a720 = match v1.clone() {
            Adt2::C0(h719, _, _, _, _, _, _, _, _, _, _) => {
                h719
            }
        };
        let a721 = 1u64;
        nat_eq(a720, a721)
    } {
        true
    } else {
        let a723 = match v1.clone() {
            Adt2::C0(h722, _, _, _, _, _, _, _, _, _, _) => {
                h722
            }
        };
        let a724 = 30u64;
        nat_eq(a723, a724)
    } {
        let a727 = match v1.clone() {
            Adt2::C0(h725, _, _, _, _, _, _, _, _, _, _) => {
                h725
            }
        };
        let a728 = match v2.clone() {
            Adt2::C0(h726, _, _, _, _, _, _, _, _, _, _) => {
                h726
            }
        };
        if nat_eq(a727, a728) {
            true
        } else {
            let a730 = match v2.clone() {
                Adt2::C0(h729, _, _, _, _, _, _, _, _, _, _) => {
                    h729
                }
            };
            let a731 = 0u64;
            nat_eq(a730, a731)
        }
    } else {
        false
    } {
        f34(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f25(v2.clone(), 1u64)?, {
            let a733 = match v2.clone() {
                Adt2::C0(h732, _, _, _, _, _, _, _, _, _, _) => {
                    h732
                }
            };
            let a734 = 0u64;
            if nat_eq(a733, a734) {
                4u64
            } else {
                2u64
            }
        }, f24(v1.clone(), 1u64), f24(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h735, _, _, _, _, _, _, _, _, _, _) => {
                h735
            }
        }, match v2 {
            Adt2::C0(h736, _, _, _, _, _, _, _, _, _, _) => {
                h736
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f20(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m737 = v3;
    match m737 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f46(v0, v1, v2.clone())?, f45(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f47(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f48(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f49(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f48(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f50(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f51(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f52(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f53(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f54(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f45(v0, v1)?, f45(v0, v2)?))
        }
        Adt4::C15 => {
            f55(v0, v1, v2)
        }
        Adt4::C16(v38, v39) => {
            f56(v0, v1, v2, v38, v39)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v40, v41) => {
            f57(v0, v1, v41, v40)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f21(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f41(match v1.clone() {
        Adt3::C0(h738, _, _, _, _, _, _, _) => {
            h738
        }
    })?;
    let v5: u64 = {
        let a739 = v0;
        let a740 = v4;
        let a741 = 0u64;
        nat_quot(a739, a740, a741)
    };
    let v6: u64 = {
        let a742 = v0;
        let a743 = v4;
        let a744 = 0u64;
        nat_rem(a742, a743, a744)
    };
    let a745 = v6;
    let a746 = 0u64;
    if nat_eq(a745, a746) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a747 = v6;
        let a748 = 1u64;
        if nat_eq(a747, a748) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a751 = {
                let a749 = v6;
                let a750 = 1u64;
                nat_add(a749, a750)?
            };
            let a752 = v4;
            if nat_eq(a751, a752) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f45(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a753 = v6;
                    let a754 = 2u64;
                    nat_sub(a753, a754)
                };
                let v8: u64 = {
                    let a755 = v7;
                    let a756 = 3u64;
                    let a757 = 0u64;
                    nat_quot(a755, a756, a757)
                };
                let v9: u64 = {
                    let a758 = v7;
                    let a759 = 3u64;
                    let a760 = 0u64;
                    nat_rem(a758, a759, a760)
                };
                let v10: u64 = f45({
                    let a764 = {
                        let a762 = v5;
                        let a763 = match v1.clone() {
                            Adt3::C0(h761, _, _, _, _, _, _, _) => {
                                h761
                            }
                        };
                        nat_mul(a762, a763)?
                    };
                    let a765 = v8;
                    nat_add(a764, a765)?
                }, v1)?;
                let a766 = v9;
                let a767 = 0u64;
                if nat_eq(a766, a767) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a768 = v9;
                    let a769 = 1u64;
                    if nat_eq(a768, a769) {
                        let a770 = v3;
                        let a771 = 2u64;
                        if nat_eq(a770, a771) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a772 = v3;
                        let a773 = 2u64;
                        if if nat_eq(a772, a773) {
                            true
                        } else {
                            let a774 = v3;
                            let a775 = 3u64;
                            nat_eq(a774, a775)
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

pub fn f22(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a777 = 2u64;
        let a778 = match v1.clone() {
            Adt3::C0(_, h776, _, _, _, _, _, _) => {
                h776
            }
        };
        nat_mul(a777, a778)?
    };
    let a779 = v0;
    let a780 = v5;
    if nat_lt(a779, a780) {
        let a784 = {
            let a781 = v0;
            let a782 = 2u64;
            let a783 = 0u64;
            nat_rem(a781, a782, a783)
        };
        let a785 = 0u64;
        if nat_eq(a784, a785) {
            Ok(Adt5::C4(19u64, 4u64, f45({
                let a786 = v0;
                let a787 = 2u64;
                let a788 = 0u64;
                nat_quot(a786, a787, a788)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a789 = v4;
            let a790 = 2147483648u64;
            if if nat_lt(a789, a790) {
                let a792 = v4;
                let a793 = match v2 {
                    Adt3::C0(_, h791, _, _, _, _, _, _) => {
                        h791
                    }
                };
                nat_lt(a792, a793)
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
            let a796 = match v2.clone() {
                Adt3::C0(h794, _, _, _, _, _, _, _) => {
                    h794
                }
            };
            let a797 = match v2.clone() {
                Adt3::C0(_, h795, _, _, _, _, _, _) => {
                    h795
                }
            };
            nat_mul(a796, a797)?
        };
        let a800 = {
            let a798 = v0;
            let a799 = v5;
            nat_sub(a798, a799)
        };
        let a801 = v6;
        if nat_lt(a800, a801) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f45({
                let a802 = v0;
                let a803 = v5;
                nat_sub(a802, a803)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a806 = {
                    let a804 = v0;
                    let a805 = v5;
                    nat_sub(a804, a805)
                };
                let a807 = v6;
                nat_sub(a806, a807)
            };
            let v8: u64 = {
                let a811 = 1u64;
                let a812 = {
                    let a809 = 2u64;
                    let a810 = match v1.clone() {
                        Adt3::C0(h808, _, _, _, _, _, _, _) => {
                            h808
                        }
                    };
                    nat_mul(a809, a810)?
                };
                nat_add(a811, a812)?
            };
            let v9: u64 = {
                let a813 = v7;
                let a814 = v8;
                let a815 = 0u64;
                nat_quot(a813, a814, a815)
            };
            let v10: u64 = {
                let a816 = v7;
                let a817 = v8;
                let a818 = 0u64;
                nat_rem(a816, a817, a818)
            };
            let a819 = v10;
            let a820 = 0u64;
            if nat_eq(a819, a820) {
                Ok(Adt5::C4(19u64, 4u64, f45(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a824 = {
                    let a821 = v10;
                    let a822 = 2u64;
                    let a823 = 0u64;
                    nat_rem(a821, a822, a823)
                };
                let a825 = 1u64;
                if nat_eq(a824, a825) {
                    Ok(Adt5::C4(6u64, 0u64, f45({
                        let a834 = {
                            let a827 = v9;
                            let a828 = match v1.clone() {
                                Adt3::C0(h826, _, _, _, _, _, _, _) => {
                                    h826
                                }
                            };
                            nat_mul(a827, a828)?
                        };
                        let a835 = {
                            let a831 = {
                                let a829 = v10;
                                let a830 = 1u64;
                                nat_sub(a829, a830)
                            };
                            let a832 = 2u64;
                            let a833 = 0u64;
                            nat_quot(a831, a832, a833)
                        };
                        nat_add(a834, a835)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f45({
                        let a844 = {
                            let a837 = v4;
                            let a838 = match v1.clone() {
                                Adt3::C0(h836, _, _, _, _, _, _, _) => {
                                    h836
                                }
                            };
                            nat_mul(a837, a838)?
                        };
                        let a845 = {
                            let a841 = {
                                let a839 = v10;
                                let a840 = 1u64;
                                nat_sub(a839, a840)
                            };
                            let a842 = 2u64;
                            let a843 = 0u64;
                            nat_quot(a841, a842, a843)
                        };
                        nat_add(a844, a845)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f45({
                        let a854 = {
                            let a847 = v4;
                            let a848 = match v1 {
                                Adt3::C0(h846, _, _, _, _, _, _, _) => {
                                    h846
                                }
                            };
                            nat_mul(a847, a848)?
                        };
                        let a855 = {
                            let a851 = {
                                let a849 = v10;
                                let a850 = 1u64;
                                nat_sub(a849, a850)
                            };
                            let a852 = 2u64;
                            let a853 = 0u64;
                            nat_quot(a851, a852, a853)
                        };
                        nat_add(a854, a855)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f23(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a856 = v0;
    let a857 = 0u64;
    if if if nat_eq(a856, a857) {
        true
    } else {
        let a858 = v1;
        let a859 = 0u64;
        nat_eq(a858, a859)
    } {
        true
    } else {
        let a860 = v2;
        let a861 = 0u64;
        nat_eq(a860, a861)
    } {
        true
    } else {
        let a862 = v3;
        let a863 = 0u64;
        nat_eq(a862, a863)
    }
}

pub fn f24(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a865 = match v0.clone() {
            Adt2::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
                h864
            }
        };
        let a866 = v1;
        let a867 = 0u64;
        nat_quot(a865, a866, a867)
    }, match v0.clone() {
        Adt2::C0(_, _, h868, _, _, _, _, _, _, _, _) => {
            h868
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h869, _, _, _, _, _, _, _) => {
            h869
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h870, _, _, _, _, _, _) => {
            h870
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h871, _, _, _, _, _) => {
            h871
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h872, _, _, _, _) => {
            h872
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h873, _, _, _) => {
            h873
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h874, _, _) => {
            h874
        }
    })
}

pub fn f25(v0: Adt2, v1: u64) -> R<u64> {
    let a886 = {
        let a880 = {
            let a876 = match v0.clone() {
                Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                    h875
                }
            };
            let a877 = v1;
            let a878 = 0u64;
            nat_quot(a876, a877, a878)
        };
        let a881 = match v0.clone() {
            Adt2::C0(_, _, h879, _, _, _, _, _, _, _, _) => {
                h879
            }
        };
        nat_mul(a880, a881)?
    };
    let a887 = {
        let a884 = match v0.clone() {
            Adt2::C0(_, _, _, h882, _, _, _, _, _, _, _) => {
                h882
            }
        };
        let a885 = match v0 {
            Adt2::C0(_, _, _, _, h883, _, _, _, _, _, _) => {
                h883
            }
        };
        nat_mul(a884, a885)?
    };
    nat_mul(a886, a887)
}

pub fn f26(v0: Adt2, v1: Adt2) -> R<bool> {
    let a889 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h888, _) => {
            h888
        }
    };
    let a890 = 0u64;
    if nat_eq(a889, a890) {
        Ok(true)
    } else {
        let a892 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h891, _) => {
                h891
            }
        };
        let a893 = 0u64;
        if nat_eq(a892, a893) {
            Ok(true)
        } else {
            let a895 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h894) => {
                    h894
                }
            };
            let a896 = 0u64;
            if nat_eq(a895, a896) {
                Ok(true)
            } else {
                let a898 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h897) => {
                        h897
                    }
                };
                let a899 = 0u64;
                if nat_eq(a898, a899) {
                    Ok(true)
                } else {
                    let a905 = {
                        let a902 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h900) => {
                                h900
                            }
                        };
                        let a903 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h901, _) => {
                                h901
                            }
                        };
                        nat_add(a902, a903)?
                    };
                    let a906 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h904) => {
                            h904
                        }
                    };
                    if nat_le(a905, a906) {
                        Ok(true)
                    } else {
                        let a912 = {
                            let a909 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h907) => {
                                    h907
                                }
                            };
                            let a910 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h908, _) => {
                                    h908
                                }
                            };
                            nat_add(a909, a910)?
                        };
                        let a913 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h911) => {
                                h911
                            }
                        };
                        Ok(nat_le(a912, a913))
                    }
                }
            }
        }
    }
}

pub fn f27(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a916 = match v1.clone() {
        Adt2::C0(h914, _, _, _, _, _, _, _, _, _, _) => {
            h914
        }
    };
    let a917 = match v3.clone() {
        Adt2::C0(h915, _, _, _, _, _, _, _, _, _, _) => {
            h915
        }
    };
    if if nat_eq(a916, a917) {
        let a918 = v0;
        let a919 = 33u64;
        if nat_eq(a918, a919) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m921 = f29(match v1.clone() {
            Adt2::C0(h920, _, _, _, _, _, _, _, _, _, _) => {
                h920
            }
        });
        match m921 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h922, _) => {
                        h922
                    }
                };
                let v6: u64 = f25(v1.clone(), v5)?;
                let a927 = {
                    let a924 = match v1.clone() {
                        Adt2::C0(_, h923, _, _, _, _, _, _, _, _, _) => {
                            h923
                        }
                    };
                    let a925 = v5;
                    let a926 = 1u64;
                    nat_rem(a924, a925, a926)
                };
                let a928 = 0u64;
                if if if nat_eq(a927, a928) {
                    let a933 = {
                        let a930 = match v3.clone() {
                            Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                                h929
                            }
                        };
                        let a931 = v5;
                        let a932 = 1u64;
                        nat_rem(a930, a931, a932)
                    };
                    let a934 = 0u64;
                    nat_eq(a933, a934)
                } else {
                    false
                } {
                    let a935 = v6;
                    let a936 = f25(v3.clone(), v5)?;
                    nat_eq(a935, a936)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h937) => {
                            h937
                        }
                    }, f24(v1, v5), f24(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f28(v0: u64) -> bool {
    let a938 = v0;
    let a939 = 0u64;
    if if nat_eq(a938, a939) {
        true
    } else {
        let a940 = v0;
        let a941 = 1u64;
        nat_eq(a940, a941)
    } {
        true
    } else {
        let a942 = v0;
        let a943 = 30u64;
        nat_eq(a942, a943)
    }
}

pub fn f29(v0: u64) -> Adt7 {
    let a944 = v0;
    let a945 = 0u64;
    if nat_eq(a944, a945) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a946 = v0;
        let a947 = 1u64;
        if nat_eq(a946, a947) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a948 = v0;
            let a949 = 2u64;
            if nat_eq(a948, a949) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a950 = v0;
                let a951 = 3u64;
                if nat_eq(a950, a951) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a952 = v0;
                    let a953 = 6u64;
                    if nat_eq(a952, a953) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a954 = v0;
                        let a955 = 7u64;
                        if nat_eq(a954, a955) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a956 = v0;
                            let a957 = 8u64;
                            if nat_eq(a956, a957) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a958 = v0;
                                let a959 = 9u64;
                                if nat_eq(a958, a959) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a960 = v0;
                                    let a961 = 10u64;
                                    if nat_eq(a960, a961) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a962 = v0;
                                        let a963 = 11u64;
                                        if nat_eq(a962, a963) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a964 = v0;
                                            let a965 = 12u64;
                                            if nat_eq(a964, a965) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a966 = v0;
                                                let a967 = 13u64;
                                                if nat_eq(a966, a967) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a968 = v0;
                                                    let a969 = 14u64;
                                                    if nat_eq(a968, a969) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a970 = v0;
                                                        let a971 = 15u64;
                                                        if nat_eq(a970, a971) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a972 = v0;
                                                            let a973 = 16u64;
                                                            if nat_eq(a972, a973) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a974 = v0;
                                                                let a975 = 17u64;
                                                                if nat_eq(a974, a975) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a976 = v0;
                                                                    let a977 = 18u64;
                                                                    if nat_eq(a976, a977) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a978 = v0;
                                                                        let a979 = 19u64;
                                                                        if nat_eq(a978, a979) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a980 = v0;
                                                                            let a981 = 20u64;
                                                                            if nat_eq(a980, a981) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a982 = v0;
                                                                                let a983 = 21u64;
                                                                                if nat_eq(a982, a983) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a984 = v0;
                                                                                    let a985 = 22u64;
                                                                                    if nat_eq(a984, a985) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a986 = v0;
                                                                                        let a987 = 23u64;
                                                                                        if nat_eq(a986, a987) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a988 = v0;
                                                                                            let a989 = 24u64;
                                                                                            if nat_eq(a988, a989) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a990 = v0;
                                                                                                let a991 = 25u64;
                                                                                                if nat_eq(a990, a991) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a992 = v0;
                                                                                                    let a993 = 26u64;
                                                                                                    if nat_eq(a992, a993) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a994 = v0;
                                                                                                        let a995 = 27u64;
                                                                                                        if nat_eq(a994, a995) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a996 = v0;
                                                                                                            let a997 = 28u64;
                                                                                                            if nat_eq(a996, a997) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a998 = v0;
                                                                                                                let a999 = 29u64;
                                                                                                                if nat_eq(a998, a999) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1000 = v0;
                                                                                                                    let a1001 = 30u64;
                                                                                                                    if nat_eq(a1000, a1001) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1002 = v0;
                                                                                                                        let a1003 = 34u64;
                                                                                                                        if nat_eq(a1002, a1003) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1004 = v0;
                                                                                                                            let a1005 = 35u64;
                                                                                                                            if nat_eq(a1004, a1005) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1006 = v0;
                                                                                                                                let a1007 = 39u64;
                                                                                                                                if nat_eq(a1006, a1007) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1008 = v0;
                                                                                                                                    let a1009 = 40u64;
                                                                                                                                    if nat_eq(a1008, a1009) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1010 = v0;
                                                                                                                                        let a1011 = 41u64;
                                                                                                                                        if nat_eq(a1010, a1011) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1012 = v0;
                                                                                                                                            let a1013 = 42u64;
                                                                                                                                            if nat_eq(a1012, a1013) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1014 = v0;
                                                                                                                                                let a1015 = 43u64;
                                                                                                                                                if nat_eq(a1014, a1015) {
                                                                                                                                                    Adt7::C1(Adt8::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt7::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f23(v4, v5, v6, v7) {
        f23(v0, v1, v2, v3)
    } else {
        let a1019 = {
            let a1016 = v0;
            let a1017 = v4;
            let a1018 = 1u64;
            nat_rem(a1016, a1017, a1018)
        };
        let a1020 = 0u64;
        if if if nat_eq(a1019, a1020) {
            let a1024 = {
                let a1021 = v1;
                let a1022 = v5;
                let a1023 = 1u64;
                nat_rem(a1021, a1022, a1023)
            };
            let a1025 = 0u64;
            nat_eq(a1024, a1025)
        } else {
            false
        } {
            let a1029 = {
                let a1026 = v2;
                let a1027 = v6;
                let a1028 = 1u64;
                nat_rem(a1026, a1027, a1028)
            };
            let a1030 = 0u64;
            nat_eq(a1029, a1030)
        } else {
            false
        } {
            let a1034 = {
                let a1031 = v3;
                let a1032 = v7;
                let a1033 = 1u64;
                nat_rem(a1031, a1032, a1033)
            };
            let a1035 = 0u64;
            nat_eq(a1034, a1035)
        } else {
            false
        }
    }
}

pub fn f31(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1036 = v0;
    let a1037 = v1;
    if nat_eq(a1036, a1037) {
        let a1040 = v4;
        let a1041 = {
            let a1038 = v2;
            let a1039 = v3;
            nat_add(a1038, a1039)?
        };
        Ok(nat_eq(a1040, a1041))
    } else {
        let a1042 = v2;
        let a1043 = v3;
        if nat_eq(a1042, a1043) {
            let a1044 = v4;
            let a1045 = v2;
            Ok(nat_eq(a1044, a1045))
        } else {
            Ok(false)
        }
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1048 = {
            let a1046 = v0;
            let a1047 = f58(v1);
            nat_add(a1046, a1047)?
        };
        let a1049 = f58(v2);
        nat_add(a1048, a1049)?
    };
    let v5: u64 = {
        let a1050 = f59(v1);
        let a1051 = f59(v2);
        nat_add(a1050, a1051)?
    };
    let a1052 = v1;
    let a1053 = 4294967295u64;
    if if nat_le(a1052, a1053) {
        let a1054 = v2;
        let a1055 = 4294967295u64;
        nat_le(a1054, a1055)
    } else {
        false
    } {
        let a1056 = v5;
        let a1057 = v4;
        if nat_le(a1056, a1057) {
            let a1060 = v3;
            let a1061 = {
                let a1058 = v4;
                let a1059 = v5;
                nat_sub(a1058, a1059)
            };
            Ok(nat_eq(a1060, a1061))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f33(v0: u64, v1: u64) -> R<bool> {
    let a1062 = v1;
    let a1063 = 4294967295u64;
    if nat_le(a1062, a1063) {
        let a1066 = {
            let a1064 = f58(v1);
            let a1065 = f59(v1);
            nat_add(a1064, a1065)?
        };
        let a1067 = v0;
        Ok(nat_lt(a1066, a1067))
    } else {
        Ok(false)
    }
}

pub fn f34(v0: Adt2, v1: Adt2) -> bool {
    let a1070 = match v0.clone() {
        Adt2::C0(_, h1068, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    };
    let a1071 = match v1.clone() {
        Adt2::C0(_, h1069, _, _, _, _, _, _, _, _, _) => {
            h1069
        }
    };
    if if if nat_eq(a1070, a1071) {
        let a1074 = match v0.clone() {
            Adt2::C0(_, _, h1072, _, _, _, _, _, _, _, _) => {
                h1072
            }
        };
        let a1075 = match v1.clone() {
            Adt2::C0(_, _, h1073, _, _, _, _, _, _, _, _) => {
                h1073
            }
        };
        nat_eq(a1074, a1075)
    } else {
        false
    } {
        let a1078 = match v0.clone() {
            Adt2::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                h1076
            }
        };
        let a1079 = match v1.clone() {
            Adt2::C0(_, _, _, h1077, _, _, _, _, _, _, _) => {
                h1077
            }
        };
        nat_eq(a1078, a1079)
    } else {
        false
    } {
        let a1082 = match v0 {
            Adt2::C0(_, _, _, _, h1080, _, _, _, _, _, _) => {
                h1080
            }
        };
        let a1083 = match v1 {
            Adt2::C0(_, _, _, _, h1081, _, _, _, _, _, _) => {
                h1081
            }
        };
        nat_eq(a1082, a1083)
    } else {
        false
    }
}

pub fn f35(v0: Adt2) -> R<bool> {
    let a1085 = match v0.clone() {
        Adt2::C0(_, h1084, _, _, _, _, _, _, _, _, _) => {
            h1084
        }
    };
    let a1086 = 1u64;
    if if if if nat_eq(a1085, a1086) {
        true
    } else {
        let a1088 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1087, _, _, _, _, _) => {
                h1087
            }
        };
        let a1089 = 4u64;
        nat_eq(a1088, a1089)
    } {
        let a1091 = match v0.clone() {
            Adt2::C0(_, _, h1090, _, _, _, _, _, _, _, _) => {
                h1090
            }
        };
        let a1092 = 1u64;
        if nat_eq(a1091, a1092) {
            true
        } else {
            let a1097 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1093, _, _, _, _) => {
                    h1093
                }
            };
            let a1098 = {
                let a1095 = match v0.clone() {
                    Adt2::C0(_, h1094, _, _, _, _, _, _, _, _, _) => {
                        h1094
                    }
                };
                let a1096 = 4u64;
                nat_mul(a1095, a1096)?
            };
            nat_eq(a1097, a1098)
        }
    } else {
        false
    } {
        let a1100 = match v0.clone() {
            Adt2::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1101 = 1u64;
        if nat_eq(a1100, a1101) {
            true
        } else {
            let a1109 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1102, _, _, _) => {
                    h1102
                }
            };
            let a1110 = {
                let a1107 = {
                    let a1105 = match v0.clone() {
                        Adt2::C0(_, h1103, _, _, _, _, _, _, _, _, _) => {
                            h1103
                        }
                    };
                    let a1106 = match v0.clone() {
                        Adt2::C0(_, _, h1104, _, _, _, _, _, _, _, _) => {
                            h1104
                        }
                    };
                    nat_mul(a1105, a1106)?
                };
                let a1108 = 4u64;
                nat_mul(a1107, a1108)?
            };
            nat_eq(a1109, a1110)
        }
    } else {
        false
    } {
        let a1112 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1111, _, _, _, _, _, _) => {
                h1111
            }
        };
        let a1113 = 1u64;
        if nat_eq(a1112, a1113) {
            Ok(true)
        } else {
            let a1124 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1114, _, _) => {
                    h1114
                }
            };
            let a1125 = {
                let a1122 = {
                    let a1120 = {
                        let a1117 = match v0.clone() {
                            Adt2::C0(_, h1115, _, _, _, _, _, _, _, _, _) => {
                                h1115
                            }
                        };
                        let a1118 = match v0.clone() {
                            Adt2::C0(_, _, h1116, _, _, _, _, _, _, _, _) => {
                                h1116
                            }
                        };
                        nat_mul(a1117, a1118)?
                    };
                    let a1121 = match v0 {
                        Adt2::C0(_, _, _, h1119, _, _, _, _, _, _, _) => {
                            h1119
                        }
                    };
                    nat_mul(a1120, a1121)?
                };
                let a1123 = 4u64;
                nat_mul(a1122, a1123)?
            };
            Ok(nat_eq(a1124, a1125))
        }
    } else {
        Ok(false)
    }
}

pub fn f36(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f60(v0.clone(), false)?;
    let v6: u64 = f60(v0, true)?;
    let a1126 = v6;
    let a1127 = v1;
    if if nat_le(a1126, a1127) {
        let a1134 = {
            let a1132 = {
                let a1128 = v1;
                let a1129 = v5;
                nat_add(a1128, a1129)?
            };
            let a1133 = {
                let a1130 = v3;
                let a1131 = 0u64;
                if nat_eq(a1130, a1131) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1132, a1133)?
        };
        let a1135 = v2;
        nat_le(a1134, a1135)
    } else {
        false
    } {
        if v4 {
            let a1140 = {
                let a1138 = {
                    let a1136 = v1;
                    let a1137 = v5;
                    nat_add(a1136, a1137)?
                };
                let a1139 = v6;
                nat_sub(a1138, a1139)
            };
            let a1141 = v2;
            Ok(nat_lt(a1140, a1141))
        } else {
            let a1146 = {
                let a1144 = {
                    let a1142 = v1;
                    let a1143 = v5;
                    nat_add(a1142, a1143)?
                };
                let a1145 = v6;
                nat_sub(a1144, a1145)
            };
            let a1147 = v2;
            Ok(nat_le(a1146, a1147))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64, v1: u64) -> bool {
    let a1148 = v0;
    let a1149 = 0u64;
    if if nat_eq(a1148, a1149) {
        true
    } else {
        let a1150 = v0;
        let a1151 = 1u64;
        nat_eq(a1150, a1151)
    } {
        let a1152 = v1;
        let a1153 = 4294967296u64;
        nat_lt(a1152, a1153)
    } else {
        false
    }
}

pub fn f38(v0: u64, v1: u64) -> R<u64> {
    let a1158 = {
        let a1156 = v0;
        let a1157 = {
            let a1154 = v1;
            let a1155 = 1u64;
            nat_sub(a1154, a1155)
        };
        nat_add(a1156, a1157)?
    };
    let a1159 = v1;
    let a1160 = 0u64;
    Ok(nat_quot(a1158, a1159, a1160))
}

pub fn f39(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f23(match v0.clone() {
        Adt2::C0(_, h1161, _, _, _, _, _, _, _, _, _) => {
            h1161
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1162, _, _, _, _, _, _, _, _) => {
            h1162
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1163, _, _, _, _, _, _, _) => {
            h1163
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1164, _, _, _, _, _, _) => {
            h1164
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1167 = match v1.clone() {
                Adt2::C0(_, h1165, _, _, _, _, _, _, _, _, _) => {
                    h1165
                }
            };
            let a1168 = match v1.clone() {
                Adt2::C0(_, _, h1166, _, _, _, _, _, _, _, _) => {
                    h1166
                }
            };
            nat_mul(a1167, a1168)?
        };
        let v5: u64 = {
            let a1170 = v4;
            let a1171 = match v1.clone() {
                Adt2::C0(_, _, _, h1169, _, _, _, _, _, _, _) => {
                    h1169
                }
            };
            nat_mul(a1170, a1171)?
        };
        let a1184 = {
            let a1182 = {
                let a1173 = match v0.clone() {
                    Adt2::C0(_, h1172, _, _, _, _, _, _, _, _, _) => {
                        h1172
                    }
                };
                let a1174 = 1u64;
                nat_sub(a1173, a1174)
            };
            let a1183 = {
                let a1180 = f61(match v0.clone() {
                    Adt2::C0(_, _, h1175, _, _, _, _, _, _, _, _) => {
                        h1175
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1176, _, _, _, _, _, _, _, _, _) => {
                        h1176
                    }
                })?;
                let a1181 = f61(match v0 {
                    Adt2::C0(_, _, _, h1177, _, _, _, _, _, _, _) => {
                        h1177
                    }
                }, v2, {
                    let a1178 = v3;
                    let a1179 = v5;
                    nat_mul(a1178, a1179)?
                }, v4)?;
                nat_add(a1180, a1181)?
            };
            nat_add(a1182, a1183)?
        };
        let a1185 = f25(v1, 1u64)?;
        Ok(nat_lt(a1184, a1185))
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1186 = v0;
    let a1187 = 2147483648u64;
    if if if if nat_lt(a1186, a1187) {
        let a1188 = v1;
        let a1189 = 2147483648u64;
        nat_lt(a1188, a1189)
    } else {
        false
    } {
        let a1192 = {
            let a1190 = v0;
            let a1191 = v1;
            nat_mul(a1190, a1191)?
        };
        let a1193 = 2147483648u64;
        nat_lt(a1192, a1193)
    } else {
        false
    } {
        let a1198 = {
            let a1196 = {
                let a1194 = v0;
                let a1195 = 1u64;
                nat_sub(a1194, a1195)
            };
            let a1197 = v2;
            nat_mul(a1196, a1197)?
        };
        let a1199 = 2147483648u64;
        nat_lt(a1198, a1199)
    } else {
        false
    } {
        let a1204 = {
            let a1202 = {
                let a1200 = v1;
                let a1201 = 1u64;
                nat_sub(a1200, a1201)
            };
            let a1203 = v2;
            nat_mul(a1202, a1203)?
        };
        let a1205 = 2147483648u64;
        Ok(nat_lt(a1204, a1205))
    } else {
        Ok(false)
    }
}

pub fn f41(v0: u64) -> R<u64> {
    let a1208 = 3u64;
    let a1209 = {
        let a1206 = v0;
        let a1207 = 3u64;
        nat_mul(a1206, a1207)?
    };
    nat_add(a1208, a1209)
}

pub fn f42(v0: Adt3) -> R<u64> {
    if f23(match v0.clone() {
        Adt3::C0(h1210, _, _, _, _, _, _, _) => {
            h1210
        }
    }, match v0.clone() {
        Adt3::C0(_, h1211, _, _, _, _, _, _) => {
            h1211
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1212, _, _, _, _, _) => {
            h1212
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1213, _, _, _, _) => {
            h1213
        }
    }) {
        Ok(0u64)
    } else {
        let a1222 = match v0.clone() {
            Adt3::C0(h1214, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        let a1223 = {
            let a1220 = match v0.clone() {
                Adt3::C0(_, h1215, _, _, _, _, _, _) => {
                    h1215
                }
            };
            let a1221 = {
                let a1218 = match v0.clone() {
                    Adt3::C0(_, _, h1216, _, _, _, _, _) => {
                        h1216
                    }
                };
                let a1219 = match v0 {
                    Adt3::C0(_, _, _, h1217, _, _, _, _) => {
                        h1217
                    }
                };
                nat_mul(a1218, a1219)?
            };
            nat_mul(a1220, a1221)?
        };
        nat_mul(a1222, a1223)
    }
}

pub fn f43(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1224 = v1;
    let a1225 = 13u64;
    if if nat_eq(a1224, a1225) {
        f23(match v0.clone() {
            Adt3::C0(h1226, _, _, _, _, _, _, _) => {
                h1226
            }
        }, match v0.clone() {
            Adt3::C0(_, h1227, _, _, _, _, _, _) => {
                h1227
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1228, _, _, _, _, _) => {
                h1228
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1229, _, _, _, _) => {
                h1229
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1235 = match v0.clone() {
                Adt3::C0(_, h1230, _, _, _, _, _, _) => {
                    h1230
                }
            };
            let a1236 = {
                let a1233 = match v0.clone() {
                    Adt3::C0(_, _, h1231, _, _, _, _, _) => {
                        h1231
                    }
                };
                let a1234 = match v0.clone() {
                    Adt3::C0(_, _, _, h1232, _, _, _, _) => {
                        h1232
                    }
                };
                nat_mul(a1233, a1234)?
            };
            nat_mul(a1235, a1236)?
        };
        let a1237 = v1;
        let a1238 = 13u64;
        if nat_eq(a1237, a1238) {
            let a1247 = 2u64;
            let a1248 = if f62(v0.clone(), v2) {
                let a1240 = v3;
                let a1241 = f63(match v0 {
                    Adt3::C0(h1239, _, _, _, _, _, _, _) => {
                        h1239
                    }
                }, false)?;
                nat_mul(a1240, a1241)?
            } else {
                let a1245 = v3;
                let a1246 = {
                    let a1243 = match v0 {
                        Adt3::C0(h1242, _, _, _, _, _, _, _) => {
                            h1242
                        }
                    };
                    let a1244 = 2u64;
                    nat_mul(a1243, a1244)?
                };
                nat_mul(a1245, a1246)?
            };
            nat_add(a1247, a1248)
        } else {
            let a1252 = v3;
            let a1253 = f63(match v0 {
                Adt3::C0(h1249, _, _, _, _, _, _, _) => {
                    h1249
                }
            }, {
                let a1250 = v1;
                let a1251 = 16u64;
                nat_eq(a1250, a1251)
            })?;
            nat_mul(a1252, a1253)
        }
    }
}

pub fn f44(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1254 = v3;
    match m1254 {
        None => {
            f64(v0, v1, match v2.clone() {
                Adt2::C0(h1255, _, _, _, _, _, _, _, _, _, _) => {
                    h1255
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1256, _, _, _, _, _, _, _, _, _, _) => {
                    h1256
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1257, _, _, _, _, _, _, _, _, _) => {
                    h1257
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1258, _, _, _, _, _, _, _, _) => {
                    h1258
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1259, _, _, _, _, _, _, _) => {
                    h1259
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1260, _, _, _, _, _, _) => {
                    h1260
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1261, _, _, _, _, _, _, _, _, _) => {
                    h1261
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1262, _, _, _, _, _, _, _, _) => {
                    h1262
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1263, _, _, _, _, _, _, _) => {
                    h1263
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1264, _, _, _, _, _, _) => {
                    h1264
                }
            })
        }
        Some(v5) => {
            f64(v0, v1, match v2.clone() {
                Adt2::C0(h1265, _, _, _, _, _, _, _, _, _, _) => {
                    h1265
                }
            }, match v5.clone() {
                Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                    h1266
                }
            }, match v4.clone() {
                Adt2::C0(h1267, _, _, _, _, _, _, _, _, _, _) => {
                    h1267
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1268, _, _, _, _, _, _, _, _, _) => {
                    h1268
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1269, _, _, _, _, _, _, _, _) => {
                    h1269
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1270, _, _, _, _, _, _, _) => {
                    h1270
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1271, _, _, _, _, _, _) => {
                    h1271
                }
            }, match v5.clone() {
                Adt2::C0(_, h1272, _, _, _, _, _, _, _, _, _) => {
                    h1272
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1273, _, _, _, _, _, _, _, _) => {
                    h1273
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1274, _, _, _, _, _, _, _) => {
                    h1274
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1275, _, _, _, _, _, _) => {
                    h1275
                }
            }, match v4.clone() {
                Adt2::C0(_, h1276, _, _, _, _, _, _, _, _, _) => {
                    h1276
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1277, _, _, _, _, _, _, _, _) => {
                    h1277
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1278, _, _, _, _, _, _, _) => {
                    h1278
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1279, _, _, _, _, _, _) => {
                    h1279
                }
            })
        }
    }
}

pub fn f45(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1281 = v0;
        let a1282 = match v1.clone() {
            Adt3::C0(h1280, _, _, _, _, _, _, _) => {
                h1280
            }
        };
        let a1283 = 0u64;
        nat_quot(a1281, a1282, a1283)
    };
    let v3: u64 = {
        let a1285 = v2;
        let a1286 = match v1.clone() {
            Adt3::C0(_, h1284, _, _, _, _, _, _) => {
                h1284
            }
        };
        let a1287 = 0u64;
        nat_quot(a1285, a1286, a1287)
    };
    f65({
        let a1289 = v0;
        let a1290 = match v1.clone() {
            Adt3::C0(h1288, _, _, _, _, _, _, _) => {
                h1288
            }
        };
        let a1291 = 0u64;
        nat_rem(a1289, a1290, a1291)
    }, {
        let a1293 = v2;
        let a1294 = match v1.clone() {
            Adt3::C0(_, h1292, _, _, _, _, _, _) => {
                h1292
            }
        };
        let a1295 = 0u64;
        nat_rem(a1293, a1294, a1295)
    }, {
        let a1297 = v3;
        let a1298 = match v1.clone() {
            Adt3::C0(_, _, h1296, _, _, _, _, _) => {
                h1296
            }
        };
        let a1299 = 0u64;
        nat_rem(a1297, a1298, a1299)
    }, {
        let a1301 = v3;
        let a1302 = match v1.clone() {
            Adt3::C0(_, _, h1300, _, _, _, _, _) => {
                h1300
            }
        };
        let a1303 = 0u64;
        nat_quot(a1301, a1302, a1303)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1304, _, _, _) => {
            h1304
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1305, _, _) => {
            h1305
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1306, _) => {
            h1306
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1307) => {
            h1307
        }
    })
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1309 = v0;
        let a1310 = match v2.clone() {
            Adt3::C0(h1308, _, _, _, _, _, _, _) => {
                h1308
            }
        };
        let a1311 = 0u64;
        nat_quot(a1309, a1310, a1311)
    };
    let v4: u64 = {
        let a1313 = v3;
        let a1314 = match v2.clone() {
            Adt3::C0(_, h1312, _, _, _, _, _, _) => {
                h1312
            }
        };
        let a1315 = 0u64;
        nat_quot(a1313, a1314, a1315)
    };
    f66({
        let a1317 = v0;
        let a1318 = match v2.clone() {
            Adt3::C0(h1316, _, _, _, _, _, _, _) => {
                h1316
            }
        };
        let a1319 = 0u64;
        nat_rem(a1317, a1318, a1319)
    }, {
        let a1321 = v3;
        let a1322 = match v2.clone() {
            Adt3::C0(_, h1320, _, _, _, _, _, _) => {
                h1320
            }
        };
        let a1323 = 0u64;
        nat_rem(a1321, a1322, a1323)
    }, {
        let a1325 = v4;
        let a1326 = match v2.clone() {
            Adt3::C0(_, _, h1324, _, _, _, _, _) => {
                h1324
            }
        };
        let a1327 = 0u64;
        nat_rem(a1325, a1326, a1327)
    }, {
        let a1329 = v4;
        let a1330 = match v2 {
            Adt3::C0(_, _, h1328, _, _, _, _, _) => {
                h1328
            }
        };
        let a1331 = 0u64;
        nat_quot(a1329, a1330, a1331)
    }, match v1.clone() {
        Adt3::C0(h1332, _, _, _, _, _, _, _) => {
            h1332
        }
    }, match v1.clone() {
        Adt3::C0(_, h1333, _, _, _, _, _, _) => {
            h1333
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1334, _, _, _, _, _) => {
            h1334
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1335, _, _, _, _) => {
            h1335
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1336, _, _, _) => {
            h1336
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1337, _, _) => {
            h1337
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1338, _) => {
            h1338
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1339) => {
            h1339
        }
    })
}

pub fn f47(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1341 = v0;
        let a1342 = match v3.clone() {
            Adt3::C0(h1340, _, _, _, _, _, _, _) => {
                h1340
            }
        };
        let a1343 = 0u64;
        nat_quot(a1341, a1342, a1343)
    };
    let v6: u64 = {
        let a1345 = v5;
        let a1346 = match v3.clone() {
            Adt3::C0(_, h1344, _, _, _, _, _, _) => {
                h1344
            }
        };
        let a1347 = 0u64;
        nat_quot(a1345, a1346, a1347)
    };
    let v7: u64 = {
        let a1349 = v0;
        let a1350 = match v3.clone() {
            Adt3::C0(h1348, _, _, _, _, _, _, _) => {
                h1348
            }
        };
        let a1351 = 0u64;
        nat_rem(a1349, a1350, a1351)
    };
    let v8: u64 = {
        let a1353 = v5;
        let a1354 = match v3.clone() {
            Adt3::C0(_, h1352, _, _, _, _, _, _) => {
                h1352
            }
        };
        let a1355 = 0u64;
        nat_rem(a1353, a1354, a1355)
    };
    let v9: u64 = {
        let a1357 = v6;
        let a1358 = match v3.clone() {
            Adt3::C0(_, _, h1356, _, _, _, _, _) => {
                h1356
            }
        };
        let a1359 = 0u64;
        nat_rem(a1357, a1358, a1359)
    };
    let v10: u64 = {
        let a1361 = v6;
        let a1362 = match v3.clone() {
            Adt3::C0(_, _, h1360, _, _, _, _, _) => {
                h1360
            }
        };
        let a1363 = 0u64;
        nat_quot(a1361, a1362, a1363)
    };
    let v11: u64 = {
        let a1364 = v4;
        let a1365 = 0u64;
        if nat_eq(a1364, a1365) {
            match v1.clone() {
                Adt3::C0(h1366, _, _, _, _, _, _, _) => {
                    h1366
                }
            }
        } else {
            let a1367 = v4;
            let a1368 = 1u64;
            if nat_eq(a1367, a1368) {
                match v1.clone() {
                    Adt3::C0(_, h1369, _, _, _, _, _, _) => {
                        h1369
                    }
                }
            } else {
                let a1370 = v4;
                let a1371 = 2u64;
                if nat_eq(a1370, a1371) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1372, _, _, _, _, _) => {
                            h1372
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1373, _, _, _, _) => {
                            h1373
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1380 = v11;
        let a1381 = {
            let a1374 = v4;
            let a1375 = 0u64;
            if nat_eq(a1374, a1375) {
                v7
            } else {
                let a1376 = v4;
                let a1377 = 1u64;
                if nat_eq(a1376, a1377) {
                    v8
                } else {
                    let a1378 = v4;
                    let a1379 = 2u64;
                    if nat_eq(a1378, a1379) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1380, a1381)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f65(if if v12 {
        let a1382 = v4;
        let a1383 = 0u64;
        nat_eq(a1382, a1383)
    } else {
        false
    } {
        let a1384 = v7;
        let a1385 = v11;
        nat_sub(a1384, a1385)
    } else {
        v7
    }, if if v12 {
        let a1386 = v4;
        let a1387 = 1u64;
        nat_eq(a1386, a1387)
    } else {
        false
    } {
        let a1388 = v8;
        let a1389 = v11;
        nat_sub(a1388, a1389)
    } else {
        v8
    }, if if v12 {
        let a1390 = v4;
        let a1391 = 2u64;
        nat_eq(a1390, a1391)
    } else {
        false
    } {
        let a1392 = v9;
        let a1393 = v11;
        nat_sub(a1392, a1393)
    } else {
        v9
    }, if if v12 {
        let a1394 = v4;
        let a1395 = 3u64;
        nat_eq(a1394, a1395)
    } else {
        false
    } {
        let a1396 = v10;
        let a1397 = v11;
        nat_sub(a1396, a1397)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1398, _, _, _) => {
            h1398
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1399, _, _) => {
            h1399
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1400, _) => {
            h1400
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1401) => {
            h1401
        }
    })?, f45(v0, v3)?))
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f67(v0, v2.clone(), 0u64);
    let v9: u64 = f67(v0, v2.clone(), 1u64);
    let v10: u64 = f67(v0, v2.clone(), 2u64);
    let v11: u64 = f67(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f68(v1.clone(), f69(v8, v3, match v1.clone() {
            Adt3::C0(h1402, _, _, _, _, _, _, _) => {
                h1402
            }
        })?, f69(v9, v4, match v1.clone() {
            Adt3::C0(_, h1403, _, _, _, _, _, _) => {
                h1403
            }
        })?, f69(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1404, _, _, _, _, _) => {
                h1404
            }
        })?, f69(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1405, _, _, _, _) => {
                h1405
            }
        })?)?, f45(v0, v2)?))
    } else {
        if if if if f70(v8, v3, match v1.clone() {
            Adt3::C0(h1406, _, _, _, _, _, _, _) => {
                h1406
            }
        })? {
            f70(v9, v4, match v1.clone() {
                Adt3::C0(_, h1407, _, _, _, _, _, _) => {
                    h1407
                }
            })?
        } else {
            false
        } {
            f70(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1408, _, _, _, _, _) => {
                    h1408
                }
            })?
        } else {
            false
        } {
            f70(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1409, _, _, _, _) => {
                    h1409
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f68(v1, f71(v8, v3)?, f71(v9, v4)?, f71(v10, v5)?, f71(v11, v6)?)?, f45(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
        }
    }
}

pub fn f49(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f67(v0, v2.clone(), 0u64);
    let v5: u64 = f67(v0, v2.clone(), 1u64);
    let v6: u64 = f67(v0, v2.clone(), 2u64);
    let v7: u64 = f67(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f68(v1.clone(), f72(v4, v3, match v1 {
        Adt3::C0(h1410, _, _, _, _, _, _, _) => {
            h1410
        }
    })?, v5, v6, v7)?, f45(v0, v2)?))
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1411 = v0;
    let a1412 = v3;
    if nat_lt(a1411, a1412) {
        Ok(Adt5::C1(false, {
            let a1413 = v0;
            let a1414 = 4u64;
            nat_mul(a1413, a1414)?
        }, {
            let a1415 = v0;
            let a1416 = 4u64;
            nat_mul(a1415, a1416)?
        }))
    } else {
        let v6: u64 = {
            let a1417 = v0;
            let a1418 = v3;
            nat_sub(a1417, a1418)
        };
        let v7: u64 = f45(v6, v1)?;
        let v8: u64 = f73(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1419 = v0;
    let a1420 = v3;
    if nat_lt(a1419, a1420) {
        Ok(Adt5::C1(true, 0u64, f45(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1421 = v0;
            let a1422 = v3;
            nat_sub(a1421, a1422)
        };
        let v5: u64 = f46(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f45(v4, v1)?, 3u64, v5))
    }
}

pub fn f52(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1424 = v1;
        let a1425 = match v3.clone() {
            Adt3::C0(h1423, _, _, _, _, _, _, _) => {
                h1423
            }
        };
        let a1426 = 0u64;
        nat_rem(a1424, a1425, a1426)
    };
    let v5: u64 = {
        let a1432 = {
            let a1428 = v1;
            let a1429 = match v3.clone() {
                Adt3::C0(h1427, _, _, _, _, _, _, _) => {
                    h1427
                }
            };
            let a1430 = 0u64;
            nat_quot(a1428, a1429, a1430)
        };
        let a1433 = match v3.clone() {
            Adt3::C0(_, h1431, _, _, _, _, _, _) => {
                h1431
            }
        };
        let a1434 = 0u64;
        nat_rem(a1432, a1433, a1434)
    };
    let v6: bool = f74(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f46(v1, v2, v3.clone())?
    } else {
        0u64
    }, f45(v1, v3)?))
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1435 = v3;
        let a1436 = 8u64;
        nat_add(a1435, a1436)?
    }, 0u64, f45(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f46(v0, v5, v2.clone())?
    }, 3u64, f45(v0, v2)?))
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1438 = v0;
        let a1439 = match v2.clone() {
            Adt3::C0(h1437, _, _, _, _, _, _, _) => {
                h1437
            }
        };
        let a1440 = 0u64;
        nat_rem(a1438, a1439, a1440)
    };
    let v7: u64 = {
        let a1442 = v0;
        let a1443 = match v2.clone() {
            Adt3::C0(h1441, _, _, _, _, _, _, _) => {
                h1441
            }
        };
        let a1444 = 0u64;
        nat_quot(a1442, a1443, a1444)
    };
    let v8: u64 = {
        let a1446 = v7;
        let a1447 = match v2.clone() {
            Adt3::C0(_, h1445, _, _, _, _, _, _) => {
                h1445
            }
        };
        let a1448 = 0u64;
        nat_rem(a1446, a1447, a1448)
    };
    let v9: u64 = {
        let a1450 = v7;
        let a1451 = match v2.clone() {
            Adt3::C0(_, h1449, _, _, _, _, _, _) => {
                h1449
            }
        };
        let a1452 = 0u64;
        nat_quot(a1450, a1451, a1452)
    };
    let v10: u64 = {
        let a1454 = v9;
        let a1455 = match v2.clone() {
            Adt3::C0(_, _, h1453, _, _, _, _, _) => {
                h1453
            }
        };
        let a1456 = 0u64;
        nat_rem(a1454, a1455, a1456)
    };
    let v11: u64 = {
        let a1458 = v9;
        let a1459 = match v2.clone() {
            Adt3::C0(_, _, h1457, _, _, _, _, _) => {
                h1457
            }
        };
        let a1460 = 0u64;
        nat_quot(a1458, a1459, a1460)
    };
    let v12: u64 = f45(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1469 = {
                let a1464 = {
                    let a1461 = v10;
                    let a1462 = v4;
                    let a1463 = 0u64;
                    nat_quot(a1461, a1462, a1463)
                };
                let a1465 = v5;
                nat_mul(a1464, a1465)?
            };
            let a1470 = {
                let a1466 = v8;
                let a1467 = v4;
                let a1468 = 0u64;
                nat_quot(a1466, a1467, a1468)
            };
            nat_add(a1469, a1470)?
        };
        let v14: u64 = {
            let a1490 = v6;
            let a1491 = {
                let a1488 = match v1.clone() {
                    Adt3::C0(h1471, _, _, _, _, _, _, _) => {
                        h1471
                    }
                };
                let a1489 = {
                    let a1486 = {
                        let a1472 = v8;
                        let a1473 = v4;
                        let a1474 = 0u64;
                        nat_rem(a1472, a1473, a1474)
                    };
                    let a1487 = {
                        let a1484 = match v1.clone() {
                            Adt3::C0(_, h1475, _, _, _, _, _, _) => {
                                h1475
                            }
                        };
                        let a1485 = {
                            let a1482 = {
                                let a1476 = v10;
                                let a1477 = v4;
                                let a1478 = 0u64;
                                nat_rem(a1476, a1477, a1478)
                            };
                            let a1483 = {
                                let a1480 = match v1.clone() {
                                    Adt3::C0(_, _, h1479, _, _, _, _, _) => {
                                        h1479
                                    }
                                };
                                let a1481 = v13;
                                nat_mul(a1480, a1481)?
                            };
                            nat_add(a1482, a1483)?
                        };
                        nat_mul(a1484, a1485)?
                    };
                    nat_add(a1486, a1487)?
                };
                nat_mul(a1488, a1489)?
            };
            nat_add(a1490, a1491)?
        };
        Ok(Adt5::C1(false, f45(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1497 = {
                let a1495 = {
                    let a1492 = v11;
                    let a1493 = v5;
                    let a1494 = 0u64;
                    nat_rem(a1492, a1493, a1494)
                };
                let a1496 = v4;
                nat_mul(a1495, a1496)?
            };
            let a1498 = v8;
            nat_add(a1497, a1498)?
        };
        let v16: u64 = {
            let a1504 = {
                let a1502 = {
                    let a1499 = v11;
                    let a1500 = v5;
                    let a1501 = 0u64;
                    nat_quot(a1499, a1500, a1501)
                };
                let a1503 = v4;
                nat_mul(a1502, a1503)?
            };
            let a1505 = v10;
            nat_add(a1504, a1505)?
        };
        let a1507 = v15;
        let a1508 = match v1.clone() {
            Adt3::C0(_, h1506, _, _, _, _, _, _) => {
                h1506
            }
        };
        if if nat_lt(a1507, a1508) {
            let a1510 = v16;
            let a1511 = match v1.clone() {
                Adt3::C0(_, _, h1509, _, _, _, _, _) => {
                    h1509
                }
            };
            nat_lt(a1510, a1511)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f45({
                let a1520 = v6;
                let a1521 = {
                    let a1518 = match v1.clone() {
                        Adt3::C0(h1512, _, _, _, _, _, _, _) => {
                            h1512
                        }
                    };
                    let a1519 = {
                        let a1516 = v15;
                        let a1517 = {
                            let a1514 = match v1.clone() {
                                Adt3::C0(_, h1513, _, _, _, _, _, _) => {
                                    h1513
                                }
                            };
                            let a1515 = v16;
                            nat_mul(a1514, a1515)?
                        };
                        nat_add(a1516, a1517)?
                    };
                    nat_mul(a1518, a1519)?
                };
                nat_add(a1520, a1521)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f45(v0, v2.clone())?;
    let v4: u64 = f45(v0, v1.clone())?;
    let a1526 = {
        let a1523 = v0;
        let a1524 = match v1 {
            Adt3::C0(h1522, _, _, _, _, _, _, _) => {
                h1522
            }
        };
        let a1525 = 0u64;
        nat_rem(a1523, a1524, a1525)
    };
    let a1527 = 0u64;
    if nat_eq(a1526, a1527) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f45({
            let a1528 = v0;
            let a1529 = 1u64;
            nat_sub(a1528, a1529)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1530 = v3;
    let a1531 = 13u64;
    if nat_eq(a1530, a1531) {
        f75(v0, v1, v4)
    } else {
        let v5: u64 = f63(match v1.clone() {
            Adt3::C0(h1532, _, _, _, _, _, _, _) => {
                h1532
            }
        }, {
            let a1533 = v3;
            let a1534 = 16u64;
            nat_eq(a1533, a1534)
        })?;
        let v6: u64 = {
            let a1535 = v0;
            let a1536 = v5;
            let a1537 = 0u64;
            nat_quot(a1535, a1536, a1537)
        };
        let v7: u64 = {
            let a1538 = v0;
            let a1539 = v5;
            let a1540 = 0u64;
            nat_rem(a1538, a1539, a1540)
        };
        let v8: u64 = {
            let a1544 = 1u64;
            let a1545 = {
                let a1542 = match v1.clone() {
                    Adt3::C0(h1541, _, _, _, _, _, _, _) => {
                        h1541
                    }
                };
                let a1543 = 2u64;
                nat_mul(a1542, a1543)?
            };
            nat_add(a1544, a1545)?
        };
        let a1546 = v7;
        let a1547 = v8;
        if nat_lt(a1546, a1547) {
            f76(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f45(v6, v2)?;
            let a1548 = v3;
            let a1549 = 14u64;
            if nat_eq(a1548, a1549) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1550 = v7;
                let a1551 = v8;
                if nat_eq(a1550, a1551) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1554 = v7;
                    let a1555 = {
                        let a1552 = v8;
                        let a1553 = 1u64;
                        nat_add(a1552, a1553)?
                    };
                    if nat_eq(a1554, a1555) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1556, _, _, _, _, _, _, _) => {
                                h1556
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

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1557 = v0;
    let a1558 = 0u64;
    if nat_eq(a1557, a1558) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1565 = {
            let a1559 = v0;
            let a1560 = 1u64;
            nat_add(a1559, a1560)?
        };
        let a1566 = {
            let a1563 = 2u64;
            let a1564 = {
                let a1561 = v3;
                let a1562 = 2u64;
                nat_mul(a1561, a1562)?
            };
            nat_add(a1563, a1564)?
        };
        if nat_eq(a1565, a1566) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1567 = v0;
                let a1568 = 1u64;
                nat_sub(a1567, a1568)
            };
            let v5: u64 = {
                let a1569 = v4;
                let a1570 = 2u64;
                let a1571 = 0u64;
                nat_quot(a1569, a1570, a1571)
            };
            let a1575 = {
                let a1572 = v4;
                let a1573 = 2u64;
                let a1574 = 0u64;
                nat_rem(a1572, a1573, a1574)
            };
            let a1576 = 0u64;
            if nat_eq(a1575, a1576) {
                Ok(Adt5::C4(17u64, 0u64, f45(v5, v1)?, 4u64, f45(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f58(v0: u64) -> u64 {
    let a1577 = v0;
    let a1578 = 2147483648u64;
    if nat_lt(a1577, a1578) {
        v0
    } else {
        0u64
    }
}

pub fn f59(v0: u64) -> u64 {
    let a1579 = v0;
    let a1580 = 2147483648u64;
    if nat_lt(a1579, a1580) {
        0u64
    } else {
        let a1581 = 4294967296u64;
        let a1582 = v0;
        nat_sub(a1581, a1582)
    }
}

pub fn f60(v0: Adt3, v1: bool) -> R<u64> {
    f77(v0.clone(), {
        let a1584 = match v0.clone() {
            Adt3::C0(h1583, _, _, _, _, _, _, _) => {
                h1583
            }
        };
        let a1585 = 1u64;
        nat_sub(a1584, a1585)
    }, {
        let a1587 = match v0.clone() {
            Adt3::C0(_, h1586, _, _, _, _, _, _) => {
                h1586
            }
        };
        let a1588 = 1u64;
        nat_sub(a1587, a1588)
    }, {
        let a1590 = match v0.clone() {
            Adt3::C0(_, _, h1589, _, _, _, _, _) => {
                h1589
            }
        };
        let a1591 = 1u64;
        nat_sub(a1590, a1591)
    }, {
        let a1593 = match v0 {
            Adt3::C0(_, _, _, h1592, _, _, _, _) => {
                h1592
            }
        };
        let a1594 = 1u64;
        nat_sub(a1593, a1594)
    }, v1)
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1595 = v0;
    let a1596 = 0u64;
    if nat_eq(a1595, a1596) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1597 = v0;
            let a1598 = 1u64;
            nat_sub(a1597, a1598)
        };
        let v5: u64 = {
            let a1599 = v4;
            let a1600 = v1;
            let a1601 = 0u64;
            nat_quot(a1599, a1600, a1601)
        };
        let v6: u64 = {
            let a1609 = {
                let a1602 = v5;
                let a1603 = v2;
                nat_mul(a1602, a1603)?
            };
            let a1610 = {
                let a1607 = {
                    let a1604 = v4;
                    let a1605 = v1;
                    let a1606 = 0u64;
                    nat_rem(a1604, a1605, a1606)
                };
                let a1608 = v3;
                nat_mul(a1607, a1608)?
            };
            nat_add(a1609, a1610)?
        };
        let a1611 = v5;
        let a1612 = 0u64;
        if nat_eq(a1611, a1612) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1621 = {
                    let a1615 = {
                        let a1613 = v5;
                        let a1614 = 1u64;
                        nat_sub(a1613, a1614)
                    };
                    let a1616 = v2;
                    nat_mul(a1615, a1616)?
                };
                let a1622 = {
                    let a1619 = {
                        let a1617 = v1;
                        let a1618 = 1u64;
                        nat_sub(a1617, a1618)
                    };
                    let a1620 = v3;
                    nat_mul(a1619, a1620)?
                };
                nat_add(a1621, a1622)?
            };
            let a1623 = v6;
            let a1624 = v7;
            if nat_lt(a1623, a1624) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f62(v0: Adt3, v1: u64) -> bool {
    let a1625 = v1;
    let a1626 = 0u64;
    if !nat_eq(a1625, a1626) {
        true
    } else {
        let a1628 = match v0 {
            Adt3::C0(_, _, _, _, h1627, _, _, _) => {
                h1627
            }
        };
        let a1629 = 4u64;
        nat_eq(a1628, a1629)
    }
}

pub fn f63(v0: u64, v1: bool) -> R<u64> {
    let a1632 = {
        let a1630 = v0;
        let a1631 = 2u64;
        nat_mul(a1630, a1631)?
    };
    let a1633 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a1632, a1633)
}

pub fn f64(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1634 = f78(v0, v1);
    match m1634 {
        None => {
            if v5 {
                let m1635 = f79(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1635 {
                    Adt9::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt9::C1 => {
                        Ok(Adt6::C1)
                    }
                    Adt9::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt6::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt6::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a1636 = 4u64;
                let a1637 = v28;
                nat_le(a1636, a1637)
            };
            let a1638 = v2;
            let a1639 = 0u64;
            if if if if if nat_eq(a1638, a1639) {
                let a1640 = v4;
                let a1641 = 0u64;
                nat_eq(a1640, a1641)
            } else {
                false
            } {
                let a1642 = v6;
                let a1643 = v14;
                if nat_eq(a1642, a1643) {
                    let a1644 = v7;
                    let a1645 = v15;
                    nat_eq(a1644, a1645)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1646 = v8;
                let a1647 = v16;
                if nat_eq(a1646, a1647) {
                    let a1648 = v9;
                    let a1649 = v17;
                    nat_eq(a1648, a1649)
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
                        let a1650 = v3;
                        let a1651 = 0u64;
                        nat_eq(a1650, a1651)
                    } else {
                        false
                    } {
                        f30(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1656 = {
                        let a1652 = v6;
                        let a1653 = v7;
                        nat_mul(a1652, a1653)?
                    };
                    let a1657 = {
                        let a1654 = v8;
                        let a1655 = v9;
                        nat_mul(a1654, a1655)?
                    };
                    nat_mul(a1656, a1657)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1670 = {
        let a1666 = {
            let a1662 = {
                let a1658 = v0;
                let a1659 = v4;
                nat_mul(a1658, a1659)?
            };
            let a1663 = {
                let a1660 = v1;
                let a1661 = v5;
                nat_mul(a1660, a1661)?
            };
            nat_add(a1662, a1663)?
        };
        let a1667 = {
            let a1664 = v2;
            let a1665 = v6;
            nat_mul(a1664, a1665)?
        };
        nat_add(a1666, a1667)?
    };
    let a1671 = {
        let a1668 = v3;
        let a1669 = v7;
        nat_mul(a1668, a1669)?
    };
    nat_add(a1670, a1671)
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f65({
        let a1672 = v0;
        let a1673 = v4;
        let a1674 = 0u64;
        nat_rem(a1672, a1673, a1674)
    }, {
        let a1675 = v1;
        let a1676 = v5;
        let a1677 = 0u64;
        nat_rem(a1675, a1676, a1677)
    }, {
        let a1678 = v2;
        let a1679 = v6;
        let a1680 = 0u64;
        nat_rem(a1678, a1679, a1680)
    }, {
        let a1681 = v3;
        let a1682 = v7;
        let a1683 = 0u64;
        nat_rem(a1681, a1682, a1683)
    }, v8, v9, v10, v11)
}

pub fn f67(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1685 = v0;
        let a1686 = match v1.clone() {
            Adt3::C0(h1684, _, _, _, _, _, _, _) => {
                h1684
            }
        };
        let a1687 = 0u64;
        nat_quot(a1685, a1686, a1687)
    };
    let v4: u64 = {
        let a1689 = v3;
        let a1690 = match v1.clone() {
            Adt3::C0(_, h1688, _, _, _, _, _, _) => {
                h1688
            }
        };
        let a1691 = 0u64;
        nat_quot(a1689, a1690, a1691)
    };
    let a1692 = v2;
    let a1693 = 0u64;
    if nat_eq(a1692, a1693) {
        let a1695 = v0;
        let a1696 = match v1 {
            Adt3::C0(h1694, _, _, _, _, _, _, _) => {
                h1694
            }
        };
        let a1697 = 0u64;
        nat_rem(a1695, a1696, a1697)
    } else {
        let a1698 = v2;
        let a1699 = 1u64;
        if nat_eq(a1698, a1699) {
            let a1701 = v3;
            let a1702 = match v1 {
                Adt3::C0(_, h1700, _, _, _, _, _, _) => {
                    h1700
                }
            };
            let a1703 = 0u64;
            nat_rem(a1701, a1702, a1703)
        } else {
            let a1704 = v2;
            let a1705 = 2u64;
            if nat_eq(a1704, a1705) {
                let a1707 = v4;
                let a1708 = match v1 {
                    Adt3::C0(_, _, h1706, _, _, _, _, _) => {
                        h1706
                    }
                };
                let a1709 = 0u64;
                nat_rem(a1707, a1708, a1709)
            } else {
                let a1711 = v4;
                let a1712 = match v1 {
                    Adt3::C0(_, _, h1710, _, _, _, _, _) => {
                        h1710
                    }
                };
                let a1713 = 0u64;
                nat_quot(a1711, a1712, a1713)
            }
        }
    }
}

pub fn f68(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f65(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1714, _, _, _) => {
            h1714
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1715, _, _) => {
            h1715
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1716, _) => {
            h1716
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1717) => {
            h1717
        }
    })
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1718 = v1;
    let a1719 = 2147483648u64;
    if nat_lt(a1718, a1719) {
        let a1730 = {
            let a1728 = {
                let a1723 = {
                    let a1720 = v0;
                    let a1721 = v2;
                    let a1722 = 0u64;
                    nat_rem(a1720, a1721, a1722)
                };
                let a1724 = v2;
                nat_add(a1723, a1724)?
            };
            let a1729 = {
                let a1725 = v1;
                let a1726 = v2;
                let a1727 = 0u64;
                nat_rem(a1725, a1726, a1727)
            };
            nat_sub(a1728, a1729)
        };
        let a1731 = v2;
        let a1732 = 0u64;
        Ok(nat_rem(a1730, a1731, a1732))
    } else {
        let a1735 = {
            let a1733 = v0;
            let a1734 = f59(v1);
            nat_add(a1733, a1734)?
        };
        let a1736 = v2;
        let a1737 = 0u64;
        Ok(nat_rem(a1735, a1736, a1737))
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1740 = f58(v1);
    let a1741 = {
        let a1738 = v0;
        let a1739 = f59(v1);
        nat_add(a1738, a1739)?
    };
    if nat_le(a1740, a1741) {
        let a1742 = f71(v0, v1)?;
        let a1743 = v2;
        Ok(nat_lt(a1742, a1743))
    } else {
        Ok(false)
    }
}

pub fn f71(v0: u64, v1: u64) -> R<u64> {
    let a1746 = {
        let a1744 = v0;
        let a1745 = f59(v1);
        nat_add(a1744, a1745)?
    };
    let a1747 = f58(v1);
    Ok(nat_sub(a1746, a1747))
}

pub fn f72(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1748 = v0;
    let a1749 = v1;
    if nat_lt(a1748, a1749) {
        let a1750 = v1;
        let a1751 = v0;
        Ok(nat_sub(a1750, a1751))
    } else {
        let a1754 = v0;
        let a1755 = {
            let a1752 = v1;
            let a1753 = v2;
            nat_add(a1752, a1753)?
        };
        if nat_lt(a1754, a1755) {
            let a1756 = v0;
            let a1757 = v1;
            Ok(nat_sub(a1756, a1757))
        } else {
            let a1764 = {
                let a1760 = 2u64;
                let a1761 = {
                    let a1758 = v2;
                    let a1759 = 1u64;
                    nat_sub(a1758, a1759)
                };
                nat_mul(a1760, a1761)?
            };
            let a1765 = {
                let a1762 = v0;
                let a1763 = v1;
                nat_sub(a1762, a1763)
            };
            Ok(nat_sub(a1764, a1765))
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1768 = {
        let a1766 = v2;
        let a1767 = f77(v1.clone(), f67(v0, v1.clone(), 0u64), f67(v0, v1.clone(), 1u64), f67(v0, v1.clone(), 2u64), f67(v0, v1.clone(), 3u64), false)?;
        nat_add(a1766, a1767)?
    };
    let a1769 = f77(v1.clone(), f67(v0, v1.clone(), 0u64), f67(v0, v1.clone(), 1u64), f67(v0, v1.clone(), 2u64), f67(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1768, a1769))
}

pub fn f74(v0: u64, v1: u64, v2: u64) -> bool {
    let a1770 = v0;
    let a1771 = 0u64;
    if nat_eq(a1770, a1771) {
        let a1772 = v2;
        let a1773 = v1;
        nat_le(a1772, a1773)
    } else {
        let a1774 = v0;
        let a1775 = 1u64;
        if nat_eq(a1774, a1775) {
            let a1776 = v2;
            let a1777 = v1;
            nat_lt(a1776, a1777)
        } else {
            let a1778 = v0;
            let a1779 = 2u64;
            if nat_eq(a1778, a1779) {
                let a1780 = v1;
                let a1781 = v2;
                nat_le(a1780, a1781)
            } else {
                let a1782 = v0;
                let a1783 = 3u64;
                if nat_eq(a1782, a1783) {
                    let a1784 = v1;
                    let a1785 = v2;
                    nat_lt(a1784, a1785)
                } else {
                    let a1786 = v1;
                    let a1787 = v2;
                    nat_eq(a1786, a1787)
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a1788 = v0;
    let a1789 = 0u64;
    if nat_eq(a1788, a1789) {
        Ok(f80(0u64))
    } else {
        let a1792 = {
            let a1790 = v0;
            let a1791 = 1u64;
            nat_add(a1790, a1791)?
        };
        let a1793 = f43(v1.clone(), 13u64, v2)?;
        if nat_eq(a1792, a1793) {
            Ok(Adt5::C4({
                let a1794 = v2;
                let a1795 = 0u64;
                if nat_eq(a1794, a1795) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a1796 = v0;
                let a1797 = 1u64;
                nat_sub(a1796, a1797)
            };
            if f62(v1.clone(), v2) {
                let v4: u64 = f63(match v1.clone() {
                    Adt3::C0(h1798, _, _, _, _, _, _, _) => {
                        h1798
                    }
                }, false)?;
                let v5: u64 = {
                    let a1799 = v3;
                    let a1800 = v4;
                    let a1801 = 0u64;
                    nat_quot(a1799, a1800, a1801)
                };
                let v6: u64 = {
                    let a1802 = v3;
                    let a1803 = v4;
                    let a1804 = 0u64;
                    nat_rem(a1802, a1803, a1804)
                };
                let a1807 = {
                    let a1805 = v6;
                    let a1806 = 1u64;
                    nat_add(a1805, a1806)?
                };
                let a1808 = v4;
                if nat_eq(a1807, a1808) {
                    Ok(Adt5::C4({
                        let a1809 = v2;
                        let a1810 = 0u64;
                        if nat_eq(a1809, a1810) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f76(v6, v5, v1, v2)
                }
            } else {
                f81({
                    let a1811 = v3;
                    let a1812 = 2u64;
                    let a1813 = 0u64;
                    nat_quot(a1811, a1812, a1813)
                }, {
                    let a1814 = v3;
                    let a1815 = 2u64;
                    let a1816 = 0u64;
                    nat_rem(a1814, a1815, a1816)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f76(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1817 = v0;
    let a1818 = 0u64;
    if nat_eq(a1817, a1818) {
        Ok(f80(1u64))
    } else {
        f81({
            let a1827 = {
                let a1820 = v1;
                let a1821 = match v2.clone() {
                    Adt3::C0(h1819, _, _, _, _, _, _, _) => {
                        h1819
                    }
                };
                nat_mul(a1820, a1821)?
            };
            let a1828 = {
                let a1824 = {
                    let a1822 = v0;
                    let a1823 = 1u64;
                    nat_sub(a1822, a1823)
                };
                let a1825 = 2u64;
                let a1826 = 0u64;
                nat_quot(a1824, a1825, a1826)
            };
            nat_add(a1827, a1828)?
        }, {
            let a1831 = {
                let a1829 = v0;
                let a1830 = 1u64;
                nat_sub(a1829, a1830)
            };
            let a1832 = 2u64;
            let a1833 = 0u64;
            nat_rem(a1831, a1832, a1833)
        }, v2, v3, 1u64)
    }
}

pub fn f77(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a1854 = {
        let a1848 = {
            let a1842 = {
                let a1836 = v1;
                let a1837 = if v5 {
                    f59(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1834, _, _, _) => {
                            h1834
                        }
                    })
                } else {
                    f58(match v0.clone() {
                        Adt3::C0(_, _, _, _, h1835, _, _, _) => {
                            h1835
                        }
                    })
                };
                nat_mul(a1836, a1837)?
            };
            let a1843 = {
                let a1840 = v2;
                let a1841 = if v5 {
                    f59(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1838, _, _) => {
                            h1838
                        }
                    })
                } else {
                    f58(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h1839, _, _) => {
                            h1839
                        }
                    })
                };
                nat_mul(a1840, a1841)?
            };
            nat_add(a1842, a1843)?
        };
        let a1849 = {
            let a1846 = v3;
            let a1847 = if v5 {
                f59(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1844, _) => {
                        h1844
                    }
                })
            } else {
                f58(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h1845, _) => {
                        h1845
                    }
                })
            };
            nat_mul(a1846, a1847)?
        };
        nat_add(a1848, a1849)?
    };
    let a1855 = {
        let a1852 = v4;
        let a1853 = if v5 {
            f59(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1850) => {
                    h1850
                }
            })
        } else {
            f58(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h1851) => {
                    h1851
                }
            })
        };
        nat_mul(a1852, a1853)?
    };
    nat_add(a1854, a1855)
}

pub fn f78(v0: u64, v1: u64) -> Option<u64> {
    let a1856 = v0;
    let a1857 = 2u64;
    if nat_eq(a1856, a1857) {
        Some(0u64)
    } else {
        let a1858 = v0;
        let a1859 = 5u64;
        if nat_eq(a1858, a1859) {
            Some(1u64)
        } else {
            let a1860 = v0;
            let a1861 = 6u64;
            if nat_eq(a1860, a1861) {
                Some(2u64)
            } else {
                let a1862 = v0;
                let a1863 = 7u64;
                if nat_eq(a1862, a1863) {
                    Some(3u64)
                } else {
                    let a1864 = v0;
                    let a1865 = 9u64;
                    if nat_eq(a1864, a1865) {
                        Some(4u64)
                    } else {
                        let a1866 = v0;
                        let a1867 = 8u64;
                        if nat_eq(a1866, a1867) {
                            Some(5u64)
                        } else {
                            let a1868 = v0;
                            let a1869 = 89u64;
                            if nat_eq(a1868, a1869) {
                                let a1870 = v1;
                                let a1871 = 2u64;
                                if nat_eq(a1870, a1871) {
                                    Some(6u64)
                                } else {
                                    let a1872 = v1;
                                    let a1873 = 0u64;
                                    if nat_eq(a1872, a1873) {
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

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a1874 = v0;
    let a1875 = 28u64;
    if if if if nat_eq(a1874, a1875) {
        let a1876 = v1;
        let a1877 = 0u64;
        nat_eq(a1876, a1877)
    } else {
        false
    } {
        let a1878 = v2;
        let a1879 = 0u64;
        nat_eq(a1878, a1879)
    } else {
        false
    } {
        let a1880 = v3;
        let a1881 = 0u64;
        nat_eq(a1880, a1881)
    } else {
        false
    } {
        let a1882 = v4;
        let a1883 = v8;
        if if if if if if if if nat_eq(a1882, a1883) {
            let a1884 = v5;
            let a1885 = v12;
            nat_eq(a1884, a1885)
        } else {
            false
        } {
            let a1886 = v9;
            let a1887 = v13;
            nat_eq(a1886, a1887)
        } else {
            false
        } {
            let a1888 = v10;
            let a1889 = v14;
            nat_eq(a1888, a1889)
        } else {
            false
        } {
            let a1890 = v11;
            let a1891 = v15;
            nat_eq(a1890, a1891)
        } else {
            false
        } {
            let a1892 = 0u64;
            let a1893 = v6;
            nat_lt(a1892, a1893)
        } else {
            false
        } {
            let a1894 = 0u64;
            let a1895 = v7;
            nat_lt(a1894, a1895)
        } else {
            false
        } {
            let a1899 = {
                let a1896 = v10;
                let a1897 = v6;
                let a1898 = 1u64;
                nat_rem(a1896, a1897, a1898)
            };
            let a1900 = 0u64;
            if nat_eq(a1899, a1900) {
                let a1904 = {
                    let a1901 = v11;
                    let a1902 = v7;
                    let a1903 = 1u64;
                    nat_rem(a1901, a1902, a1903)
                };
                let a1905 = 0u64;
                nat_eq(a1904, a1905)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a1906 = v10;
                let a1907 = v6;
                let a1908 = 0u64;
                nat_quot(a1906, a1907, a1908)
            }, {
                let a1909 = v11;
                let a1910 = v7;
                let a1911 = 0u64;
                nat_quot(a1909, a1910, a1911)
            }, {
                let a1912 = v5;
                let a1913 = v4;
                nat_mul(a1912, a1913)?
            }, {
                let a1914 = v4;
                let a1915 = v9;
                nat_mul(a1914, a1915)?
            }, {
                let a1916 = v5;
                let a1917 = v9;
                nat_mul(a1916, a1917)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f80(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f81(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1918 = v1;
    let a1919 = 0u64;
    if nat_eq(a1918, a1919) {
        Ok(Adt5::C4({
            let a1920 = v3;
            let a1921 = 0u64;
            if nat_eq(a1920, a1921) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f45(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a1922 = v3;
            let a1923 = 0u64;
            if nat_eq(a1922, a1923) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}
