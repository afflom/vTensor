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
    C4(u64, u64, Adt3, Adt3, Adt3, u64),
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
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6),
    C1(bool, u64, u64),
    C2(u64, u64, u64, u64, u64, u64),
    C3(u64, u64, u64, u64),
    C4(u64, u64, u64, u64, u64, u64, u64),
    C5(u64, u64, u64, u64, u64, u64),
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
                let a4 = 39u64;
                if nat_eq(a3, a4) {
                    f1(v3, v4, v5)?
                } else {
                    let a5 = v1;
                    let a6 = 77u64;
                    if nat_eq(a5, a6) {
                        f2(v3, v5)?
                    } else {
                        let a7 = v1;
                        let a8 = 40u64;
                        if nat_eq(a7, a8) {
                            f3(v3, v4, v5)?
                        } else {
                            if f4(v1) {
                                f5(v1, v3, {
                                    let m9 = v4;
                                    match m9 {
                                        None => {
                                            false
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }, v5)?
                            } else {
                                let a10 = v1;
                                let a11 = 19u64;
                                if nat_eq(a10, a11) {
                                    f6(v3, v5)?
                                } else {
                                    let a12 = v1;
                                    let a13 = 21u64;
                                    if nat_eq(a12, a13) {
                                        f7(v6, v3, v4, v5)?
                                    } else {
                                        let a14 = v1;
                                        let a15 = 61u64;
                                        if if if nat_eq(a14, a15) {
                                            true
                                        } else {
                                            let a16 = v1;
                                            let a17 = 62u64;
                                            nat_eq(a16, a17)
                                        } {
                                            true
                                        } else {
                                            let a18 = v1;
                                            let a19 = 63u64;
                                            nat_eq(a18, a19)
                                        } {
                                            f8(v1, v2, v3, v5)?
                                        } else {
                                            let a20 = v1;
                                            let a21 = 4u64;
                                            if if nat_eq(a20, a21) {
                                                true
                                            } else {
                                                let a22 = v1;
                                                let a23 = 32u64;
                                                nat_eq(a22, a23)
                                            } {
                                                f9(v1, v2, v3, v4, v5)?
                                            } else {
                                                let a24 = v1;
                                                let a25 = 20u64;
                                                if nat_eq(a24, a25) {
                                                    f10(v3, v5)?
                                                } else {
                                                    let a26 = v1;
                                                    let a27 = 42u64;
                                                    if nat_eq(a26, a27) {
                                                        f11(4u64, v3, v5)?
                                                    } else {
                                                        let a28 = v1;
                                                        let a29 = 69u64;
                                                        if nat_eq(a28, a29) {
                                                            let a30 = v6;
                                                            let a31 = 4u64;
                                                            if nat_lt(a30, a31) {
                                                                f11(v6, v3, v5)?
                                                            } else {
                                                                Adt6::C1
                                                            }
                                                        } else {
                                                            let a32 = v1;
                                                            let a33 = 70u64;
                                                            if nat_eq(a32, a33) {
                                                                f12(v6, v3, v5)?
                                                            } else {
                                                                let a34 = v1;
                                                                let a35 = 75u64;
                                                                if if nat_eq(a34, a35) {
                                                                    true
                                                                } else {
                                                                    let a36 = v1;
                                                                    let a37 = 76u64;
                                                                    nat_eq(a36, a37)
                                                                } {
                                                                    f13({
                                                                        let a38 = v1;
                                                                        let a39 = 76u64;
                                                                        nat_eq(a38, a39)
                                                                    }, v2, v3, v5)?
                                                                } else {
                                                                    let a40 = v1;
                                                                    let a41 = 31u64;
                                                                    if if if nat_eq(a40, a41) {
                                                                        true
                                                                    } else {
                                                                        let a42 = v1;
                                                                        let a43 = 47u64;
                                                                        nat_eq(a42, a43)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a44 = v1;
                                                                        let a45 = 68u64;
                                                                        nat_eq(a44, a45)
                                                                    } {
                                                                        f14(v1, v2, v3, v5)?
                                                                    } else {
                                                                        let a46 = v1;
                                                                        let a47 = 15u64;
                                                                        if nat_eq(a46, a47) {
                                                                            f15(v3, v5)?
                                                                        } else {
                                                                            let a48 = v1;
                                                                            let a49 = 17u64;
                                                                            if nat_eq(a48, a49) {
                                                                                f16(v3, v5)?
                                                                            } else {
                                                                                let a50 = v1;
                                                                                let a51 = 18u64;
                                                                                if nat_eq(a50, a51) {
                                                                                    f17(v3, v4, v5)?
                                                                                } else {
                                                                                    let a52 = v1;
                                                                                    let a53 = 13u64;
                                                                                    if if if nat_eq(a52, a53) {
                                                                                        true
                                                                                    } else {
                                                                                        let a54 = v1;
                                                                                        let a55 = 14u64;
                                                                                        nat_eq(a54, a55)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a56 = v1;
                                                                                        let a57 = 16u64;
                                                                                        nat_eq(a56, a57)
                                                                                    } {
                                                                                        f18(v1, v3, v5)?
                                                                                    } else {
                                                                                        let m58 = f19(v1, v6);
                                                                                        match m58 {
                                                                                            None => {
                                                                                                f20(v1, v6, v3, v4, v5)?
                                                                                            }
                                                                                            Some(v8) => {
                                                                                                f21(v8, v3, v5)?
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
            f22(v9, v10, v11, v12)
        }
        Adt0::C2(v13, v14, v15, v16) => {
            f23(v13, v14, v15, v16)
        }
        Adt0::C3(v17, v18, v19, v20, v21) => {
            f24(v17, v18, v19, v20, v21)
        }
        Adt0::C4(v22, v23, v24, v25, v26, v27) => {
            f25(v22, v23, v24, v25, v26, v27)
        }
    }
}

pub fn f1(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m59 = v1;
    match m59 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f26(v0, v3, v2)
        }
    }
}

pub fn f2(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a61 = match v0.clone() {
        Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
            h60
        }
    };
    let a62 = 1u64;
    if if nat_eq(a61, a62) {
        true
    } else {
        let a64 = match v0.clone() {
            Adt2::C0(h63, _, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a65 = 30u64;
        nat_eq(a64, a65)
    } {
        let a67 = match v1.clone() {
            Adt2::C0(h66, _, _, _, _, _, _, _, _, _, _) => {
                h66
            }
        };
        let a68 = 1u64;
        if if if if if if if if if if if nat_eq(a67, a68) {
            let a71 = match v0.clone() {
                Adt2::C0(_, h69, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            };
            let a72 = match v1.clone() {
                Adt2::C0(_, h70, _, _, _, _, _, _, _, _, _) => {
                    h70
                }
            };
            nat_eq(a71, a72)
        } else {
            false
        } {
            let a74 = 0u64;
            let a75 = match v1.clone() {
                Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                    h73
                }
            };
            nat_lt(a74, a75)
        } else {
            false
        } {
            let a78 = match v1.clone() {
                Adt2::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a79 = match v1.clone() {
                Adt2::C0(_, _, _, h77, _, _, _, _, _, _, _) => {
                    h77
                }
            };
            nat_eq(a78, a79)
        } else {
            false
        } {
            let a81 = match v1.clone() {
                Adt2::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                    h80
                }
            };
            let a82 = 1u64;
            nat_eq(a81, a82)
        } else {
            false
        } {
            let a90 = match v0.clone() {
                Adt2::C0(_, _, h83, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a91 = {
                let a88 = {
                    let a86 = match v1.clone() {
                        Adt2::C0(_, _, h84, _, _, _, _, _, _, _, _) => {
                            h84
                        }
                    };
                    let a87 = match v1.clone() {
                        Adt2::C0(_, _, h85, _, _, _, _, _, _, _, _) => {
                            h85
                        }
                    };
                    nat_add(a86, a87)?
                };
                let a89 = 1u64;
                nat_sub(a88, a89)
            };
            nat_eq(a90, a91)
        } else {
            false
        } {
            let a93 = 0u64;
            let a94 = match v0.clone() {
                Adt2::C0(_, _, _, h92, _, _, _, _, _, _, _) => {
                    h92
                }
            };
            nat_lt(a93, a94)
        } else {
            false
        } {
            let a96 = 0u64;
            let a97 = match v0.clone() {
                Adt2::C0(_, _, _, _, h95, _, _, _, _, _, _) => {
                    h95
                }
            };
            nat_lt(a96, a97)
        } else {
            false
        } {
            let a99 = 2u64;
            let a100 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h98, _, _, _, _, _) => {
                    h98
                }
            };
            nat_le(a99, a100)
        } else {
            false
        } {
            let a102 = 2u64;
            let a103 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h101, _, _, _, _, _) => {
                    h101
                }
            };
            nat_le(a102, a103)
        } else {
            false
        } {
            f27(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt6::C4(f28(v1.clone(), 1u64)?, 2u64, f29(v0, 1u64), f29(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f3(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m104 = v1;
    match m104 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a106 = match v0.clone() {
                Adt2::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
                    h105
                }
            };
            let a107 = 0u64;
            if if if if if if if if if if if if if nat_eq(a106, a107) {
                true
            } else {
                let a109 = match v0.clone() {
                    Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
                        h108
                    }
                };
                let a110 = 1u64;
                nat_eq(a109, a110)
            } {
                let a112 = match v3.clone() {
                    Adt2::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                        h111
                    }
                };
                let a113 = 26u64;
                nat_eq(a112, a113)
            } else {
                false
            } {
                let a115 = match v2.clone() {
                    Adt2::C0(h114, _, _, _, _, _, _, _, _, _, _) => {
                        h114
                    }
                };
                let a116 = 0u64;
                nat_eq(a115, a116)
            } else {
                false
            } {
                let a119 = match v0.clone() {
                    Adt2::C0(_, h117, _, _, _, _, _, _, _, _, _) => {
                        h117
                    }
                };
                let a120 = match v2.clone() {
                    Adt2::C0(_, h118, _, _, _, _, _, _, _, _, _) => {
                        h118
                    }
                };
                nat_eq(a119, a120)
            } else {
                false
            } {
                let a123 = match v3.clone() {
                    Adt2::C0(_, h121, _, _, _, _, _, _, _, _, _) => {
                        h121
                    }
                };
                let a124 = match v0.clone() {
                    Adt2::C0(_, _, h122, _, _, _, _, _, _, _, _) => {
                        h122
                    }
                };
                nat_le(a123, a124)
            } else {
                false
            } {
                let a126 = match v0.clone() {
                    Adt2::C0(_, _, _, h125, _, _, _, _, _, _, _) => {
                        h125
                    }
                };
                let a127 = 1u64;
                nat_eq(a126, a127)
            } else {
                false
            } {
                let a129 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h128, _, _, _, _, _, _) => {
                        h128
                    }
                };
                let a130 = 1u64;
                nat_eq(a129, a130)
            } else {
                false
            } {
                let a132 = match v2.clone() {
                    Adt2::C0(_, _, _, h131, _, _, _, _, _, _, _) => {
                        h131
                    }
                };
                let a133 = 1u64;
                nat_eq(a132, a133)
            } else {
                false
            } {
                let a135 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h134, _, _, _, _, _, _) => {
                        h134
                    }
                };
                let a136 = 1u64;
                nat_eq(a135, a136)
            } else {
                false
            } {
                let a138 = match v3.clone() {
                    Adt2::C0(_, _, h137, _, _, _, _, _, _, _, _) => {
                        h137
                    }
                };
                let a139 = 1u64;
                nat_eq(a138, a139)
            } else {
                false
            } {
                let a141 = match v3.clone() {
                    Adt2::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
                        h140
                    }
                };
                let a142 = 1u64;
                nat_eq(a141, a142)
            } else {
                false
            } {
                let a144 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h143, _, _, _, _, _, _) => {
                        h143
                    }
                };
                let a145 = 1u64;
                nat_eq(a144, a145)
            } else {
                false
            } {
                if f30(match v2.clone() {
                    Adt2::C0(_, h146, _, _, _, _, _, _, _, _, _) => {
                        h146
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h147, _, _, _, _, _, _, _, _) => {
                        h147
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h148, _, _, _, _, _, _, _) => {
                        h148
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h149, _, _, _, _, _, _) => {
                        h149
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f29(v0, 1u64), f29(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a151 = match v3.clone() {
                        Adt2::C0(_, h150, _, _, _, _, _, _, _, _, _) => {
                            h150
                        }
                    };
                    let a152 = 0u64;
                    if nat_eq(a151, a152) {
                        Ok(Adt6::C4(f28(v2.clone(), 1u64)?, 4u64, f29(v0, 1u64), f29(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a154 = match v0.clone() {
                            Adt2::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                                h153
                            }
                        };
                        let a155 = 2147483647u64;
                        if if if if if if nat_le(a154, a155) {
                            let a157 = match v3.clone() {
                                Adt2::C0(_, h156, _, _, _, _, _, _, _, _, _) => {
                                    h156
                                }
                            };
                            let a158 = 2147483647u64;
                            nat_le(a157, a158)
                        } else {
                            false
                        } {
                            let a160 = 4u64;
                            let a161 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h159, _, _, _, _, _) => {
                                    h159
                                }
                            };
                            nat_le(a160, a161)
                        } else {
                            false
                        } {
                            let a167 = {
                                let a164 = match v2.clone() {
                                    Adt2::C0(_, h162, _, _, _, _, _, _, _, _, _) => {
                                        h162
                                    }
                                };
                                let a165 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h163, _, _, _, _, _) => {
                                        h163
                                    }
                                };
                                nat_mul(a164, a165)?
                            };
                            let a168 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h166, _, _, _, _) => {
                                    h166
                                }
                            };
                            nat_le(a167, a168)
                        } else {
                            false
                        } {
                            f27(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f27(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a182 = {
                                    let a172 = {
                                        let a170 = 2u64;
                                        let a171 = match v3.clone() {
                                            Adt2::C0(_, h169, _, _, _, _, _, _, _, _, _) => {
                                                h169
                                            }
                                        };
                                        nat_mul(a170, a171)?
                                    };
                                    let a173 = f28(v2.clone(), 1u64)?;
                                    nat_add(a172, a173)?
                                };
                                let a183 = {
                                    let a180 = match v3.clone() {
                                        Adt2::C0(_, h174, _, _, _, _, _, _, _, _, _) => {
                                            h174
                                        }
                                    };
                                    let a181 = {
                                        let a178 = 1u64;
                                        let a179 = {
                                            let a176 = 2u64;
                                            let a177 = match v0.clone() {
                                                Adt2::C0(_, h175, _, _, _, _, _, _, _, _, _) => {
                                                    h175
                                                }
                                            };
                                            nat_mul(a176, a177)?
                                        };
                                        nat_add(a178, a179)?
                                    };
                                    nat_mul(a180, a181)?
                                };
                                nat_add(a182, a183)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h184, _, _, _, _, _, _, _, _, _) => {
                                    h184
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                                    h185
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h186, _, _, _, _, _) => {
                                    h186
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h187, _, _, _, _) => {
                                    h187
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h188, _, _, _) => {
                                    h188
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h189, _, _) => {
                                    h189
                                }
                            }), f29(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h190, _, _, _, _, _, _, _, _, _, _) => {
                                    h190
                                }
                            }, f29(v3, 1u64))))
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

pub fn f4(v0: u64) -> bool {
    let a191 = v0;
    let a192 = 1u64;
    if if nat_eq(a191, a192) {
        true
    } else {
        let a193 = v0;
        let a194 = 33u64;
        nat_eq(a193, a194)
    } {
        true
    } else {
        let a195 = v0;
        let a196 = 34u64;
        nat_eq(a195, a196)
    }
}

pub fn f5(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a199 = match v1.clone() {
        Adt2::C0(h197, _, _, _, _, _, _, _, _, _, _) => {
            h197
        }
    };
    let a200 = match v3.clone() {
        Adt2::C0(h198, _, _, _, _, _, _, _, _, _, _) => {
            h198
        }
    };
    if nat_eq(a199, a200) {
        f31(v0, v1, v2, v3)
    } else {
        if if if if f32(match v1.clone() {
            Adt2::C0(h201, _, _, _, _, _, _, _, _, _, _) => {
                h201
            }
        }) {
            f32(match v3.clone() {
                Adt2::C0(h202, _, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            })
        } else {
            false
        } {
            let a203 = v0;
            let a204 = 33u64;
            if nat_eq(a203, a204) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a205 = f28(v1.clone(), 1u64)?;
            let a206 = f28(v3.clone(), 1u64)?;
            nat_eq(a205, a206)
        } else {
            false
        } {
            let m208 = f33(match v3.clone() {
                Adt2::C0(h207, _, _, _, _, _, _, _, _, _, _) => {
                    h207
                }
            });
            match m208 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f28(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h209) => {
                            h209
                        }
                    }, f29(v1.clone(), 1u64), f29(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h210, _, _, _, _, _, _, _, _, _, _) => {
                            h210
                        }
                    }, match v3 {
                        Adt2::C0(h211, _, _, _, _, _, _, _, _, _, _) => {
                            h211
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f6(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a213 = match v0.clone() {
        Adt2::C0(h212, _, _, _, _, _, _, _, _, _, _) => {
            h212
        }
    };
    let a214 = 0u64;
    if if if if nat_eq(a213, a214) {
        true
    } else {
        let a216 = match v0.clone() {
            Adt2::C0(h215, _, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        };
        let a217 = 1u64;
        nat_eq(a216, a217)
    } {
        true
    } else {
        let a219 = match v0.clone() {
            Adt2::C0(h218, _, _, _, _, _, _, _, _, _, _) => {
                h218
            }
        };
        let a220 = 25u64;
        if nat_eq(a219, a220) {
            true
        } else {
            let a222 = match v0.clone() {
                Adt2::C0(h221, _, _, _, _, _, _, _, _, _, _) => {
                    h221
                }
            };
            let a223 = 26u64;
            nat_eq(a222, a223)
        }
    } {
        true
    } else {
        let a225 = match v0.clone() {
            Adt2::C0(h224, _, _, _, _, _, _, _, _, _, _) => {
                h224
            }
        };
        let a226 = 30u64;
        nat_eq(a225, a226)
    } {
        let a229 = match v0.clone() {
            Adt2::C0(h227, _, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        };
        let a230 = match v1.clone() {
            Adt2::C0(h228, _, _, _, _, _, _, _, _, _, _) => {
                h228
            }
        };
        if if nat_eq(a229, a230) {
            f34(match v1.clone() {
                Adt2::C0(_, h231, _, _, _, _, _, _, _, _, _) => {
                    h231
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h232, _, _, _, _, _, _, _, _) => {
                    h232
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h233, _, _, _, _, _, _, _) => {
                    h233
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h234, _, _, _, _, _, _) => {
                    h234
                }
            }, match v0.clone() {
                Adt2::C0(_, h235, _, _, _, _, _, _, _, _, _) => {
                    h235
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h236, _, _, _, _, _, _, _, _) => {
                    h236
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h237, _, _, _, _, _, _, _) => {
                    h237
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h238, _, _, _, _, _, _) => {
                    h238
                }
            })
        } else {
            false
        } {
            let m240 = f33(match v0.clone() {
                Adt2::C0(h239, _, _, _, _, _, _, _, _, _, _) => {
                    h239
                }
            });
            match m240 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f28(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h241) => {
                            h241
                        }
                    }, f29(v0, 1u64), f29(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f7(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m242 = v2;
    match m242 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a243 = v0;
            let a244 = 4u64;
            if if if nat_lt(a243, a244) {
                let a247 = match v1.clone() {
                    Adt2::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                        h245
                    }
                };
                let a248 = match v4.clone() {
                    Adt2::C0(h246, _, _, _, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                if nat_eq(a247, a248) {
                    let a251 = match v1.clone() {
                        Adt2::C0(h249, _, _, _, _, _, _, _, _, _, _) => {
                            h249
                        }
                    };
                    let a252 = match v3.clone() {
                        Adt2::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                            h250
                        }
                    };
                    nat_eq(a251, a252)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f35(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h253, _, _, _, _, _, _, _, _, _) => {
                        h253
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h254, _, _, _, _, _, _, _, _, _) => {
                        h254
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h255, _, _, _, _, _, _, _, _, _) => {
                        h255
                    }
                })? {
                    f35(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h256, _, _, _, _, _, _, _, _) => {
                            h256
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h257, _, _, _, _, _, _, _, _) => {
                            h257
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h258, _, _, _, _, _, _, _, _) => {
                            h258
                        }
                    })?
                } else {
                    false
                } {
                    f35(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h259, _, _, _, _, _, _, _) => {
                            h259
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h260, _, _, _, _, _, _, _) => {
                            h260
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h261, _, _, _, _, _, _, _) => {
                            h261
                        }
                    })?
                } else {
                    false
                } {
                    f35(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h262, _, _, _, _, _, _) => {
                            h262
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h263, _, _, _, _, _, _) => {
                            h263
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                            h264
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m266 = f33(match v1.clone() {
                    Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                });
                match m266 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h267, _) => {
                                h267
                            }
                        };
                        let a272 = {
                            let a269 = match v1.clone() {
                                Adt2::C0(_, h268, _, _, _, _, _, _, _, _, _) => {
                                    h268
                                }
                            };
                            let a270 = v6;
                            let a271 = 1u64;
                            nat_rem(a269, a270, a271)
                        };
                        let a273 = 0u64;
                        if if if nat_eq(a272, a273) {
                            let a278 = {
                                let a275 = match v4.clone() {
                                    Adt2::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                                        h274
                                    }
                                };
                                let a276 = v6;
                                let a277 = 1u64;
                                nat_rem(a275, a276, a277)
                            };
                            let a279 = 0u64;
                            nat_eq(a278, a279)
                        } else {
                            false
                        } {
                            let a284 = {
                                let a281 = match v3.clone() {
                                    Adt2::C0(_, h280, _, _, _, _, _, _, _, _, _) => {
                                        h280
                                    }
                                };
                                let a282 = v6;
                                let a283 = 1u64;
                                nat_rem(a281, a282, a283)
                            };
                            let a285 = 0u64;
                            nat_eq(a284, a285)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f28(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h286) => {
                                    h286
                                }
                            }, f29(v1, v6), f29(v3, v6), Adt4::C2(v0, f29(v4, v6))))
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

pub fn f8(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a288 = match v2.clone() {
        Adt2::C0(h287, _, _, _, _, _, _, _, _, _, _) => {
            h287
        }
    };
    let a289 = 0u64;
    if if nat_eq(a288, a289) {
        let a291 = match v3.clone() {
            Adt2::C0(h290, _, _, _, _, _, _, _, _, _, _) => {
                h290
            }
        };
        let a292 = 0u64;
        nat_eq(a291, a292)
    } else {
        false
    } {
        let a293 = v0;
        let a294 = 61u64;
        if nat_eq(a293, a294) {
            let v4: bool = {
                let a296 = 0u64;
                let a297 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h295, _, _, _, _, _, _, _) => {
                        h295
                    }
                };
                nat_lt(a296, a297)
            };
            if if if if if if f36(match v2.clone() {
                Adt2::C0(_, h298, _, _, _, _, _, _, _, _, _) => {
                    h298
                }
            }, match v1.clone() {
                Adt1::C0(h299, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h299
                }
            }, match v1.clone() {
                Adt1::C0(_, h300, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h300
                }
            }, match v3.clone() {
                Adt2::C0(_, h301, _, _, _, _, _, _, _, _, _) => {
                    h301
                }
            })? {
                f36(match v2.clone() {
                    Adt2::C0(_, _, h302, _, _, _, _, _, _, _, _) => {
                        h302
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h303, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h303
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h304, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h304
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h305, _, _, _, _, _, _, _, _) => {
                        h305
                    }
                })?
            } else {
                false
            } {
                f36(match v2.clone() {
                    Adt2::C0(_, _, _, h306, _, _, _, _, _, _, _) => {
                        h306
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h307, _, _, _, _, _, _, _, _, _, _, _) => {
                        h307
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h308, _, _, _, _, _, _, _, _, _, _) => {
                        h308
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h309, _, _, _, _, _, _, _) => {
                        h309
                    }
                })?
            } else {
                false
            } {
                f36(match v2.clone() {
                    Adt2::C0(_, _, _, _, h310, _, _, _, _, _, _) => {
                        h310
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h311, _, _, _, _, _, _, _, _, _) => {
                        h311
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h312, _, _, _, _, _, _, _, _) => {
                        h312
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h313, _, _, _, _, _, _) => {
                        h313
                    }
                })?
            } else {
                false
            } {
                let a315 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h314, _, _, _, _, _, _, _) => {
                        h314
                    }
                };
                let a316 = 4294967295u64;
                nat_le(a315, a316)
            } else {
                false
            } {
                if v4 {
                    !f30(match v2.clone() {
                        Adt2::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                            h317
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                            h318
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h319, _, _, _, _, _, _, _) => {
                            h319
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                            h320
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f29(v2, 1u64), f29(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h321, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h321
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h322, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h322
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h323, _, _, _, _, _, _, _, _, _, _, _) => {
                        h323
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h324, _, _, _, _, _, _, _, _, _) => {
                        h324
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a325 = v0;
            let a326 = 62u64;
            if nat_eq(a325, a326) {
                let a328 = match v1.clone() {
                    Adt1::C0(h327, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h327
                    }
                };
                let a329 = 2147483648u64;
                if if if if if nat_lt(a328, a329) {
                    let a331 = match v1.clone() {
                        Adt1::C0(_, h330, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h330
                        }
                    };
                    let a332 = 2147483648u64;
                    nat_lt(a331, a332)
                } else {
                    false
                } {
                    let a335 = match v1.clone() {
                        Adt1::C0(h333, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h333
                        }
                    };
                    let a336 = match v2.clone() {
                        Adt2::C0(_, h334, _, _, _, _, _, _, _, _, _) => {
                            h334
                        }
                    };
                    nat_lt(a335, a336)
                } else {
                    false
                } {
                    let a339 = match v1.clone() {
                        Adt1::C0(_, h337, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h337
                        }
                    };
                    let a340 = match v2.clone() {
                        Adt2::C0(_, h338, _, _, _, _, _, _, _, _, _) => {
                            h338
                        }
                    };
                    nat_lt(a339, a340)
                } else {
                    false
                } {
                    let a343 = match v2.clone() {
                        Adt2::C0(_, _, h341, _, _, _, _, _, _, _, _) => {
                            h341
                        }
                    };
                    let a344 = match v3.clone() {
                        Adt2::C0(_, _, h342, _, _, _, _, _, _, _, _) => {
                            h342
                        }
                    };
                    if if if nat_eq(a343, a344) {
                        let a347 = match v2.clone() {
                            Adt2::C0(_, _, _, h345, _, _, _, _, _, _, _) => {
                                h345
                            }
                        };
                        let a348 = match v3.clone() {
                            Adt2::C0(_, _, _, h346, _, _, _, _, _, _, _) => {
                                h346
                            }
                        };
                        nat_eq(a347, a348)
                    } else {
                        false
                    } {
                        let a351 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h349, _, _, _, _, _, _) => {
                                h349
                            }
                        };
                        let a352 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h350, _, _, _, _, _, _) => {
                                h350
                            }
                        };
                        nat_eq(a351, a352)
                    } else {
                        false
                    } {
                        let a361 = match v3.clone() {
                            Adt2::C0(_, h353, _, _, _, _, _, _, _, _, _) => {
                                h353
                            }
                        };
                        let a362 = {
                            let a359 = {
                                let a356 = match v2.clone() {
                                    Adt2::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                                        h354
                                    }
                                };
                                let a357 = match v1.clone() {
                                    Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h355
                                    }
                                };
                                nat_add(a356, a357)?
                            };
                            let a360 = match v1.clone() {
                                Adt1::C0(_, h358, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h358
                                }
                            };
                            nat_add(a359, a360)?
                        };
                        nat_eq(a361, a362)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f29(v2, 1u64), f29(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h363, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h363
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a364 = v0;
                let a365 = 63u64;
                if nat_eq(a364, a365) {
                    if if if if if f37(match v2.clone() {
                        Adt2::C0(_, h366, _, _, _, _, _, _, _, _, _) => {
                            h366
                        }
                    }, match v1.clone() {
                        Adt1::C0(h367, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h367
                        }
                    })? {
                        f37(match v2.clone() {
                            Adt2::C0(_, _, h368, _, _, _, _, _, _, _, _) => {
                                h368
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h369, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h369
                            }
                        })?
                    } else {
                        false
                    } {
                        f37(match v2.clone() {
                            Adt2::C0(_, _, _, h370, _, _, _, _, _, _, _) => {
                                h370
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h371, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h371
                            }
                        })?
                    } else {
                        false
                    } {
                        f37(match v2.clone() {
                            Adt2::C0(_, _, _, _, h372, _, _, _, _, _, _) => {
                                h372
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h373, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h373
                            }
                        })?
                    } else {
                        false
                    } {
                        let a376 = match v2.clone() {
                            Adt2::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                                h374
                            }
                        };
                        let a377 = match v3.clone() {
                            Adt2::C0(_, h375, _, _, _, _, _, _, _, _, _) => {
                                h375
                            }
                        };
                        if if if nat_eq(a376, a377) {
                            let a380 = match v2.clone() {
                                Adt2::C0(_, _, h378, _, _, _, _, _, _, _, _) => {
                                    h378
                                }
                            };
                            let a381 = match v3.clone() {
                                Adt2::C0(_, _, h379, _, _, _, _, _, _, _, _) => {
                                    h379
                                }
                            };
                            nat_eq(a380, a381)
                        } else {
                            false
                        } {
                            let a384 = match v2.clone() {
                                Adt2::C0(_, _, _, h382, _, _, _, _, _, _, _) => {
                                    h382
                                }
                            };
                            let a385 = match v3.clone() {
                                Adt2::C0(_, _, _, h383, _, _, _, _, _, _, _) => {
                                    h383
                                }
                            };
                            nat_eq(a384, a385)
                        } else {
                            false
                        } {
                            let a388 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h386, _, _, _, _, _, _) => {
                                    h386
                                }
                            };
                            let a389 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h387, _, _, _, _, _, _) => {
                                    h387
                                }
                            };
                            nat_eq(a388, a389)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f29(v2, 1u64), f29(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h390, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h390
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h391, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h391
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h392, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h392
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h393, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h393
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

pub fn f9(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m394 = v3;
    match m394 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a395 = v0;
                let a396 = 4u64;
                nat_eq(a395, a396)
            };
            let v7: bool = {
                let a398 = 0u64;
                let a399 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h397, _, _, _, _, _, _, _, _, _, _, _) => {
                        h397
                    }
                };
                nat_lt(a398, a399)
            };
            let v8: u64 = f28(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h400, _, _, _, _, _, _, _, _, _) => {
                    h400
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h401, _, _, _, _, _, _, _, _) => {
                    h401
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h402, _, _, _, _, _, _, _) => {
                    h402
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h403, _, _, _, _, _, _) => {
                    h403
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h404, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h404
                }
            }, match v1.clone() {
                Adt1::C0(_, h405, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h405
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h406, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            });
            if if if if if if if if if if if if v6 {
                let a408 = match v2.clone() {
                    Adt2::C0(h407, _, _, _, _, _, _, _, _, _, _) => {
                        h407
                    }
                };
                let a409 = 0u64;
                if nat_eq(a408, a409) {
                    let a411 = match v5.clone() {
                        Adt2::C0(h410, _, _, _, _, _, _, _, _, _, _) => {
                            h410
                        }
                    };
                    let a412 = 0u64;
                    nat_eq(a411, a412)
                } else {
                    false
                }
            } else {
                let a414 = match v2.clone() {
                    Adt2::C0(h413, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a415 = 0u64;
                if if nat_eq(a414, a415) {
                    true
                } else {
                    let a417 = match v2.clone() {
                        Adt2::C0(h416, _, _, _, _, _, _, _, _, _, _) => {
                            h416
                        }
                    };
                    let a418 = 26u64;
                    nat_eq(a417, a418)
                } {
                    let a420 = match v5.clone() {
                        Adt2::C0(h419, _, _, _, _, _, _, _, _, _, _) => {
                            h419
                        }
                    };
                    let a421 = 0u64;
                    if nat_eq(a420, a421) {
                        true
                    } else {
                        let a423 = match v5.clone() {
                            Adt2::C0(h422, _, _, _, _, _, _, _, _, _, _) => {
                                h422
                            }
                        };
                        let a424 = 26u64;
                        nat_eq(a423, a424)
                    }
                } else {
                    false
                }
            } {
                let a427 = match v2.clone() {
                    Adt2::C0(h425, _, _, _, _, _, _, _, _, _, _) => {
                        h425
                    }
                };
                let a428 = match v4.clone() {
                    Adt2::C0(h426, _, _, _, _, _, _, _, _, _, _) => {
                        h426
                    }
                };
                nat_eq(a427, a428)
            } else {
                false
            } {
                f38(v2.clone(), v4.clone())
            } else {
                false
            } {
                f39(v2.clone())?
            } else {
                false
            } {
                f39(v4.clone())?
            } else {
                false
            } {
                let a430 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h429, _, _, _, _, _) => {
                        h429
                    }
                };
                let a431 = 4u64;
                nat_eq(a430, a431)
            } else {
                false
            } {
                let a432 = v8;
                let a433 = f28(v2.clone(), 1u64)?;
                nat_le(a432, a433)
            } else {
                false
            } {
                let a435 = match v1.clone() {
                    Adt1::C0(_, _, _, h434, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h434
                    }
                };
                let a436 = 1073741824u64;
                nat_lt(a435, a436)
            } else {
                false
            } {
                let a438 = match v1.clone() {
                    Adt1::C0(h437, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h437
                    }
                };
                let a439 = 4294967295u64;
                if if if if nat_le(a438, a439) {
                    let a441 = match v1.clone() {
                        Adt1::C0(_, h440, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h440
                        }
                    };
                    let a442 = 4294967295u64;
                    nat_le(a441, a442)
                } else {
                    false
                } {
                    let a444 = match v1.clone() {
                        Adt1::C0(_, _, h443, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h443
                        }
                    };
                    let a445 = 4294967295u64;
                    nat_le(a444, a445)
                } else {
                    false
                } {
                    let a447 = match v1.clone() {
                        Adt1::C0(_, _, _, h446, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h446
                        }
                    };
                    let a448 = 4294967295u64;
                    nat_le(a447, a448)
                } else {
                    false
                } {
                    let a450 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h449, _, _, _, _, _, _, _, _, _, _, _) => {
                            h449
                        }
                    };
                    let a451 = 4294967295u64;
                    nat_le(a450, a451)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a454 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h452) => {
                            h452
                        }
                    };
                    let a455 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h453) => {
                            h453
                        }
                    };
                    nat_eq(a454, a455)
                } else {
                    true
                }
            } else {
                false
            } {
                f40(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h456, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h456
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h457, _) => {
                        h457
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f28(v2, 1u64)?
                };
                Ok(Adt6::C4({
                    let a458 = v10;
                    let a459 = v8;
                    nat_add(a458, a459)?
                }, 4u64, f29(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h460, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h460
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f10(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a462 = match v0.clone() {
        Adt2::C0(h461, _, _, _, _, _, _, _, _, _, _) => {
            h461
        }
    };
    let a463 = 0u64;
    if if if nat_eq(a462, a463) {
        let a465 = match v1.clone() {
            Adt2::C0(h464, _, _, _, _, _, _, _, _, _, _) => {
                h464
            }
        };
        let a466 = 0u64;
        nat_eq(a465, a466)
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt2::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                h467
            }
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
        }, match v1.clone() {
            Adt2::C0(_, h471, _, _, _, _, _, _, _, _, _) => {
                h471
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h472, _, _, _, _, _, _, _, _) => {
                h472
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h473, _, _, _, _, _, _, _) => {
                h473
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h474, _, _, _, _, _, _) => {
                h474
            }
        })
    } else {
        false
    } {
        let v2: u64 = f28(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a475 = v2;
            let a476 = f28(v0.clone(), 1u64)?;
            nat_add(a475, a476)?
        }, 4u64, f29(v0, 1u64), f29(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f11(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a478 = match v1.clone() {
        Adt2::C0(h477, _, _, _, _, _, _, _, _, _, _) => {
            h477
        }
    };
    let a479 = 0u64;
    if if if nat_eq(a478, a479) {
        true
    } else {
        let a481 = match v1.clone() {
            Adt2::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        let a482 = 1u64;
        nat_eq(a481, a482)
    } {
        true
    } else {
        let a484 = match v1.clone() {
            Adt2::C0(h483, _, _, _, _, _, _, _, _, _, _) => {
                h483
            }
        };
        let a485 = 30u64;
        nat_eq(a484, a485)
    } {
        let a488 = match v1.clone() {
            Adt2::C0(h486, _, _, _, _, _, _, _, _, _, _) => {
                h486
            }
        };
        let a489 = match v2.clone() {
            Adt2::C0(h487, _, _, _, _, _, _, _, _, _, _) => {
                h487
            }
        };
        if if if if if nat_eq(a488, a489) {
            let a492 = match v1.clone() {
                Adt2::C0(_, h490, _, _, _, _, _, _, _, _, _) => {
                    h490
                }
            };
            let a493 = match v2.clone() {
                Adt2::C0(_, h491, _, _, _, _, _, _, _, _, _) => {
                    h491
                }
            };
            nat_eq(a492, a493)
        } else {
            false
        } {
            let a496 = match v1.clone() {
                Adt2::C0(_, _, _, h494, _, _, _, _, _, _, _) => {
                    h494
                }
            };
            let a497 = match v2.clone() {
                Adt2::C0(_, _, _, h495, _, _, _, _, _, _, _) => {
                    h495
                }
            };
            nat_eq(a496, a497)
        } else {
            false
        } {
            let a500 = match v1.clone() {
                Adt2::C0(_, _, _, _, h498, _, _, _, _, _, _) => {
                    h498
                }
            };
            let a501 = match v2.clone() {
                Adt2::C0(_, _, _, _, h499, _, _, _, _, _, _) => {
                    h499
                }
            };
            nat_eq(a500, a501)
        } else {
            false
        } {
            let a502 = v0;
            let a503 = 4u64;
            if nat_eq(a502, a503) {
                let a505 = match v1.clone() {
                    Adt2::C0(_, _, h504, _, _, _, _, _, _, _, _) => {
                        h504
                    }
                };
                let a506 = 1u64;
                if nat_eq(a505, a506) {
                    let a509 = match v1.clone() {
                        Adt2::C0(_, h507, _, _, _, _, _, _, _, _, _) => {
                            h507
                        }
                    };
                    let a510 = match v2.clone() {
                        Adt2::C0(_, _, h508, _, _, _, _, _, _, _, _) => {
                            h508
                        }
                    };
                    nat_eq(a509, a510)
                } else {
                    false
                }
            } else {
                let a511 = v0;
                let a512 = 4u64;
                if if nat_lt(a511, a512) {
                    let a515 = match v1.clone() {
                        Adt2::C0(_, h513, _, _, _, _, _, _, _, _, _) => {
                            h513
                        }
                    };
                    let a516 = match v1.clone() {
                        Adt2::C0(_, _, h514, _, _, _, _, _, _, _, _) => {
                            h514
                        }
                    };
                    nat_eq(a515, a516)
                } else {
                    false
                } {
                    let a519 = match v1.clone() {
                        Adt2::C0(_, _, h517, _, _, _, _, _, _, _, _) => {
                            h517
                        }
                    };
                    let a520 = match v2.clone() {
                        Adt2::C0(_, _, h518, _, _, _, _, _, _, _, _) => {
                            h518
                        }
                    };
                    nat_eq(a519, a520)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m522 = f33(match v1.clone() {
                Adt2::C0(h521, _, _, _, _, _, _, _, _, _, _) => {
                    h521
                }
            });
            match m522 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f28(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h523) => {
                            h523
                        }
                    }, f29(v1, 1u64), f29(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f12(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f41(match v1.clone() {
        Adt2::C0(h524, _, _, _, _, _, _, _, _, _, _) => {
            h524
        }
    }, v0) {
        let a527 = match v1.clone() {
            Adt2::C0(h525, _, _, _, _, _, _, _, _, _, _) => {
                h525
            }
        };
        let a528 = match v2.clone() {
            Adt2::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                h526
            }
        };
        nat_eq(a527, a528)
    } else {
        false
    } {
        let a531 = match v1.clone() {
            Adt2::C0(_, h529, _, _, _, _, _, _, _, _, _) => {
                h529
            }
        };
        let a532 = match v2.clone() {
            Adt2::C0(_, h530, _, _, _, _, _, _, _, _, _) => {
                h530
            }
        };
        if if if nat_eq(a531, a532) {
            let a535 = match v1.clone() {
                Adt2::C0(_, _, h533, _, _, _, _, _, _, _, _) => {
                    h533
                }
            };
            let a536 = match v2.clone() {
                Adt2::C0(_, _, h534, _, _, _, _, _, _, _, _) => {
                    h534
                }
            };
            nat_eq(a535, a536)
        } else {
            false
        } {
            let a539 = match v1.clone() {
                Adt2::C0(_, _, _, h537, _, _, _, _, _, _, _) => {
                    h537
                }
            };
            let a540 = match v2.clone() {
                Adt2::C0(_, _, _, h538, _, _, _, _, _, _, _) => {
                    h538
                }
            };
            nat_eq(a539, a540)
        } else {
            false
        } {
            let a543 = match v1.clone() {
                Adt2::C0(_, _, _, _, h541, _, _, _, _, _, _) => {
                    h541
                }
            };
            let a544 = match v2.clone() {
                Adt2::C0(_, _, _, _, h542, _, _, _, _, _, _) => {
                    h542
                }
            };
            nat_eq(a543, a544)
        } else {
            false
        }
    } else {
        false
    } {
        let m546 = f33(match v1.clone() {
            Adt2::C0(h545, _, _, _, _, _, _, _, _, _, _) => {
                h545
            }
        });
        match m546 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f28(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h547) => {
                        h547
                    }
                }, f29(v1.clone(), 1u64), f29(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h548, _, _, _, _, _, _, _, _, _, _) => {
                        h548
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h549, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h549
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h550, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h550
            }
        }
    };
    let a551 = 0u64;
    let a552 = v4;
    if if nat_lt(a551, a552) {
        let a553 = v4;
        let a554 = 2147483648u64;
        nat_lt(a553, a554)
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
        let v7: u64 = f42(match v5.clone() {
            Adt2::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                h555
            }
        }, v4)?;
        let v8: u64 = f42(match v5.clone() {
            Adt2::C0(_, _, _, h556, _, _, _, _, _, _, _) => {
                h556
            }
        }, v4)?;
        let a558 = match v5.clone() {
            Adt2::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a559 = 0u64;
        if if if if if nat_eq(a558, a559) {
            let a561 = match v6.clone() {
                Adt2::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                    h560
                }
            };
            let a562 = 0u64;
            nat_eq(a561, a562)
        } else {
            false
        } {
            let a564 = match v5.clone() {
                Adt2::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
                    h563
                }
            };
            let a565 = 1u64;
            nat_eq(a564, a565)
        } else {
            false
        } {
            let a568 = match v5.clone() {
                Adt2::C0(_, h566, _, _, _, _, _, _, _, _, _) => {
                    h566
                }
            };
            let a569 = match v6.clone() {
                Adt2::C0(_, h567, _, _, _, _, _, _, _, _, _) => {
                    h567
                }
            };
            nat_eq(a568, a569)
        } else {
            false
        } {
            if v0 {
                let a571 = match v5.clone() {
                    Adt2::C0(_, _, h570, _, _, _, _, _, _, _, _) => {
                        h570
                    }
                };
                let a572 = 2147483648u64;
                if if if nat_lt(a571, a572) {
                    let a574 = match v5.clone() {
                        Adt2::C0(_, _, _, h573, _, _, _, _, _, _, _) => {
                            h573
                        }
                    };
                    let a575 = 2147483648u64;
                    nat_lt(a574, a575)
                } else {
                    false
                } {
                    let a578 = {
                        let a576 = v7;
                        let a577 = v8;
                        nat_mul(a576, a577)?
                    };
                    let a579 = 2147483648u64;
                    nat_le(a578, a579)
                } else {
                    false
                } {
                    f43(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a581 = match v1.clone() {
                    Adt1::C0(h580, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h580
                    }
                };
                let a582 = v7;
                if if if if nat_eq(a581, a582) {
                    let a584 = match v1 {
                        Adt1::C0(_, h583, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h583
                        }
                    };
                    let a585 = v8;
                    nat_eq(a584, a585)
                } else {
                    false
                } {
                    f44(v7, v8, v4)?
                } else {
                    false
                } {
                    let a587 = match v6.clone() {
                        Adt2::C0(_, _, h586, _, _, _, _, _, _, _, _) => {
                            h586
                        }
                    };
                    let a588 = v4;
                    if nat_eq(a587, a588) {
                        let a590 = match v6.clone() {
                            Adt2::C0(_, _, _, h589, _, _, _, _, _, _, _) => {
                                h589
                            }
                        };
                        let a591 = v4;
                        nat_eq(a590, a591)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a595 = match v6 {
                        Adt2::C0(_, _, _, _, h592, _, _, _, _, _, _) => {
                            h592
                        }
                    };
                    let a596 = {
                        let a593 = v7;
                        let a594 = v8;
                        nat_mul(a593, a594)?
                    };
                    nat_eq(a595, a596)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f29(v2, 1u64), f29(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f14(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a598 = match v2.clone() {
        Adt2::C0(h597, _, _, _, _, _, _, _, _, _, _) => {
            h597
        }
    };
    let a599 = 0u64;
    if if if if if nat_eq(a598, a599) {
        true
    } else {
        let a601 = match v2.clone() {
            Adt2::C0(h600, _, _, _, _, _, _, _, _, _, _) => {
                h600
            }
        };
        let a602 = 1u64;
        nat_eq(a601, a602)
    } {
        true
    } else {
        let a604 = match v2.clone() {
            Adt2::C0(h603, _, _, _, _, _, _, _, _, _, _) => {
                h603
            }
        };
        let a605 = 30u64;
        nat_eq(a604, a605)
    } {
        let a608 = match v2.clone() {
            Adt2::C0(h606, _, _, _, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a609 = match v3.clone() {
            Adt2::C0(h607, _, _, _, _, _, _, _, _, _, _) => {
                h607
            }
        };
        nat_eq(a608, a609)
    } else {
        false
    } {
        if if f38(v2.clone(), v3.clone()) {
            let a611 = match v1.clone() {
                Adt1::C0(h610, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h610
                }
            };
            let a612 = 4294967296u64;
            nat_lt(a611, a612)
        } else {
            false
        } {
            let a614 = match v1.clone() {
                Adt1::C0(_, h613, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h613
                }
            };
            let a615 = 4294967296u64;
            nat_lt(a614, a615)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v3.clone(), 1u64)?, {
            let a617 = match v2.clone() {
                Adt2::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
                    h616
                }
            };
            let a618 = 0u64;
            if nat_eq(a617, a618) {
                4u64
            } else {
                2u64
            }
        }, f29(v2.clone(), 1u64), f29(v3, 1u64), Adt4::C13({
            let a619 = v0;
            let a620 = 31u64;
            if nat_eq(a619, a620) {
                0u64
            } else {
                let a621 = v0;
                let a622 = 47u64;
                if nat_eq(a621, a622) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h623, _, _, _, _, _, _, _, _, _, _) => {
                h623
            }
        }, match v1.clone() {
            Adt1::C0(h624, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h624
            }
        }, match v1 {
            Adt1::C0(_, h625, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h625
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f15(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a627 = match v0.clone() {
        Adt2::C0(h626, _, _, _, _, _, _, _, _, _, _) => {
            h626
        }
    };
    let a628 = 0u64;
    if if if nat_eq(a627, a628) {
        let a630 = match v1.clone() {
            Adt2::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                h629
            }
        };
        let a631 = 0u64;
        nat_eq(a630, a631)
    } else {
        false
    } {
        f38(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v1.clone(), 1u64)?, 4u64, f29(v0, 1u64), f29(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f16(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a633 = match v0.clone() {
        Adt2::C0(h632, _, _, _, _, _, _, _, _, _, _) => {
            h632
        }
    };
    let a634 = 0u64;
    if if nat_eq(a633, a634) {
        let a636 = match v1.clone() {
            Adt2::C0(h635, _, _, _, _, _, _, _, _, _, _) => {
                h635
            }
        };
        let a637 = 26u64;
        nat_eq(a636, a637)
    } else {
        false
    } {
        let a639 = match v0.clone() {
            Adt2::C0(_, _, _, h638, _, _, _, _, _, _, _) => {
                h638
            }
        };
        let a640 = 1u64;
        if if if if if if if nat_eq(a639, a640) {
            let a642 = match v0.clone() {
                Adt2::C0(_, _, _, _, h641, _, _, _, _, _, _) => {
                    h641
                }
            };
            let a643 = 1u64;
            nat_eq(a642, a643)
        } else {
            false
        } {
            let a645 = match v0.clone() {
                Adt2::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                    h644
                }
            };
            let a646 = 2147483647u64;
            nat_le(a645, a646)
        } else {
            false
        } {
            let a649 = match v1.clone() {
                Adt2::C0(_, h647, _, _, _, _, _, _, _, _, _) => {
                    h647
                }
            };
            let a650 = match v0.clone() {
                Adt2::C0(_, _, h648, _, _, _, _, _, _, _, _) => {
                    h648
                }
            };
            nat_eq(a649, a650)
        } else {
            false
        } {
            let a652 = match v1.clone() {
                Adt2::C0(_, _, h651, _, _, _, _, _, _, _, _) => {
                    h651
                }
            };
            let a653 = 1u64;
            nat_eq(a652, a653)
        } else {
            false
        } {
            let a655 = match v1.clone() {
                Adt2::C0(_, _, _, h654, _, _, _, _, _, _, _) => {
                    h654
                }
            };
            let a656 = 1u64;
            nat_eq(a655, a656)
        } else {
            false
        } {
            let a658 = match v1.clone() {
                Adt2::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
                    h657
                }
            };
            let a659 = 1u64;
            nat_eq(a658, a659)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a662 = match v0.clone() {
                    Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                        h660
                    }
                };
                let a663 = f45(match v0.clone() {
                    Adt2::C0(_, h661, _, _, _, _, _, _, _, _, _) => {
                        h661
                    }
                })?;
                nat_mul(a662, a663)?
            }, 4u64, f29(v0, 1u64), f29(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f17(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m664 = v1;
    match m664 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f46(f29(v0.clone(), 1u64))?;
            let a666 = match v0.clone() {
                Adt2::C0(h665, _, _, _, _, _, _, _, _, _, _) => {
                    h665
                }
            };
            let a667 = 26u64;
            if if if nat_eq(a666, a667) {
                let a669 = match v3.clone() {
                    Adt2::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
                        h668
                    }
                };
                let a670 = 26u64;
                nat_eq(a669, a670)
            } else {
                false
            } {
                let a672 = match v2.clone() {
                    Adt2::C0(h671, _, _, _, _, _, _, _, _, _, _) => {
                        h671
                    }
                };
                let a673 = 27u64;
                nat_eq(a672, a673)
            } else {
                false
            } {
                if if if if if if f38(v0.clone(), v3.clone()) {
                    let a675 = match v2.clone() {
                        Adt2::C0(_, h674, _, _, _, _, _, _, _, _, _) => {
                            h674
                        }
                    };
                    let a676 = 1u64;
                    nat_eq(a675, a676)
                } else {
                    false
                } {
                    let a678 = match v2.clone() {
                        Adt2::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                            h677
                        }
                    };
                    let a679 = 1u64;
                    nat_eq(a678, a679)
                } else {
                    false
                } {
                    let a681 = match v2.clone() {
                        Adt2::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                            h680
                        }
                    };
                    let a682 = 1u64;
                    nat_eq(a681, a682)
                } else {
                    false
                } {
                    let a684 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                            h683
                        }
                    };
                    let a685 = 1u64;
                    nat_eq(a684, a685)
                } else {
                    false
                } {
                    let a686 = v4;
                    let a687 = 9223372036854775808u64;
                    nat_lt(a686, a687)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a690 = 2u64;
                        let a691 = {
                            let a688 = v4;
                            let a689 = 2u64;
                            nat_mul(a688, a689)?
                        };
                        nat_add(a690, a691)?
                    }, 8u64, f29(v0, 1u64), f29(v2, 1u64), Adt4::C18(v4, f29(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f18(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h692, _, _, _, _, _, _, _, _, _, _) => {
            h692
        }
    };
    let a693 = v3;
    let a694 = 0u64;
    if if nat_eq(a693, a694) {
        true
    } else {
        let a695 = v0;
        let a696 = 13u64;
        if nat_eq(a695, a696) {
            let a697 = v3;
            let a698 = 1u64;
            if nat_eq(a697, a698) {
                true
            } else {
                let a699 = v3;
                let a700 = 30u64;
                nat_eq(a699, a700)
            }
        } else {
            false
        }
    } {
        let a702 = v3;
        let a703 = match v2.clone() {
            Adt2::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                h701
            }
        };
        if if nat_eq(a702, a703) {
            let a704 = v0;
            let a705 = 13u64;
            if nat_eq(a704, a705) {
                let a707 = match v2.clone() {
                    Adt2::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                };
                let a708 = 1u64;
                if if if nat_eq(a707, a708) {
                    let a710 = match v2.clone() {
                        Adt2::C0(_, _, h709, _, _, _, _, _, _, _, _) => {
                            h709
                        }
                    };
                    let a711 = 1u64;
                    nat_eq(a710, a711)
                } else {
                    false
                } {
                    let a713 = match v2.clone() {
                        Adt2::C0(_, _, _, h712, _, _, _, _, _, _, _) => {
                            h712
                        }
                    };
                    let a714 = 1u64;
                    nat_eq(a713, a714)
                } else {
                    false
                } {
                    let a716 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h715, _, _, _, _, _, _) => {
                            h715
                        }
                    };
                    let a717 = 1u64;
                    nat_eq(a716, a717)
                } else {
                    false
                }
            } else {
                let a719 = match v2.clone() {
                    Adt2::C0(_, h718, _, _, _, _, _, _, _, _, _) => {
                        h718
                    }
                };
                let a720 = 1u64;
                if if if nat_eq(a719, a720) {
                    let a723 = match v1.clone() {
                        Adt2::C0(_, _, h721, _, _, _, _, _, _, _, _) => {
                            h721
                        }
                    };
                    let a724 = match v2.clone() {
                        Adt2::C0(_, _, h722, _, _, _, _, _, _, _, _) => {
                            h722
                        }
                    };
                    nat_eq(a723, a724)
                } else {
                    false
                } {
                    let a727 = match v1.clone() {
                        Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
                            h725
                        }
                    };
                    let a728 = match v2.clone() {
                        Adt2::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                            h726
                        }
                    };
                    nat_eq(a727, a728)
                } else {
                    false
                } {
                    let a731 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h729, _, _, _, _, _, _) => {
                            h729
                        }
                    };
                    let a732 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h730, _, _, _, _, _, _) => {
                            h730
                        }
                    };
                    nat_eq(a731, a732)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f29(v1, 1u64);
            Ok(Adt6::C4(f47(v4.clone(), v0, v3)?, {
                let a733 = v3;
                let a734 = 0u64;
                if nat_eq(a733, a734) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f29(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f19(v0: u64, v1: u64) -> Option<u64> {
    let a735 = v0;
    let a736 = 8u64;
    if nat_eq(a735, a736) {
        Some(5u64)
    } else {
        let a737 = v0;
        let a738 = 9u64;
        if nat_eq(a737, a738) {
            Some(6u64)
        } else {
            let a739 = v0;
            let a740 = 89u64;
            if nat_eq(a739, a740) {
                let a741 = v1;
                let a742 = 0u64;
                if nat_eq(a741, a742) {
                    Some(0u64)
                } else {
                    let a743 = v1;
                    let a744 = 1u64;
                    if nat_eq(a743, a744) {
                        Some(1u64)
                    } else {
                        let a745 = v1;
                        let a746 = 2u64;
                        if nat_eq(a745, a746) {
                            Some(2u64)
                        } else {
                            let a747 = v1;
                            let a748 = 3u64;
                            if nat_eq(a747, a748) {
                                Some(3u64)
                            } else {
                                let a749 = v1;
                                let a750 = 6u64;
                                if nat_eq(a749, a750) {
                                    Some(4u64)
                                } else {
                                    let a751 = v1;
                                    let a752 = 12u64;
                                    if nat_eq(a751, a752) {
                                        Some(7u64)
                                    } else {
                                        let a753 = v1;
                                        let a754 = 11u64;
                                        if nat_eq(a753, a754) {
                                            Some(8u64)
                                        } else {
                                            let a755 = v1;
                                            let a756 = 18u64;
                                            if nat_eq(a755, a756) {
                                                Some(9u64)
                                            } else {
                                                let a757 = v1;
                                                let a758 = 19u64;
                                                if nat_eq(a757, a758) {
                                                    Some(10u64)
                                                } else {
                                                    let a759 = v1;
                                                    let a760 = 20u64;
                                                    if nat_eq(a759, a760) {
                                                        Some(11u64)
                                                    } else {
                                                        let a761 = v1;
                                                        let a762 = 21u64;
                                                        if nat_eq(a761, a762) {
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

pub fn f20(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m763 = f48(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m763 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f29(v2.clone(), 1u64), f29(v4, 1u64), Adt4::C10(v5, v6, {
                let m764 = v3;
                match m764 {
                    None => {
                        f29(v2, 1u64)
                    }
                    Some(v12) => {
                        f29(v12, 1u64)
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

pub fn f21(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a766 = match v1.clone() {
        Adt2::C0(h765, _, _, _, _, _, _, _, _, _, _) => {
            h765
        }
    };
    let a767 = 0u64;
    if if if if if nat_eq(a766, a767) {
        true
    } else {
        let a769 = match v1.clone() {
            Adt2::C0(h768, _, _, _, _, _, _, _, _, _, _) => {
                h768
            }
        };
        let a770 = 1u64;
        nat_eq(a769, a770)
    } {
        true
    } else {
        let a772 = match v1.clone() {
            Adt2::C0(h771, _, _, _, _, _, _, _, _, _, _) => {
                h771
            }
        };
        let a773 = 30u64;
        nat_eq(a772, a773)
    } {
        let a776 = match v1.clone() {
            Adt2::C0(h774, _, _, _, _, _, _, _, _, _, _) => {
                h774
            }
        };
        let a777 = match v2.clone() {
            Adt2::C0(h775, _, _, _, _, _, _, _, _, _, _) => {
                h775
            }
        };
        if nat_eq(a776, a777) {
            true
        } else {
            let a779 = match v2.clone() {
                Adt2::C0(h778, _, _, _, _, _, _, _, _, _, _) => {
                    h778
                }
            };
            let a780 = 0u64;
            nat_eq(a779, a780)
        }
    } else {
        false
    } {
        f38(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v2.clone(), 1u64)?, {
            let a782 = match v2.clone() {
                Adt2::C0(h781, _, _, _, _, _, _, _, _, _, _) => {
                    h781
                }
            };
            let a783 = 0u64;
            if nat_eq(a782, a783) {
                4u64
            } else {
                2u64
            }
        }, f29(v1.clone(), 1u64), f29(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h784, _, _, _, _, _, _, _, _, _, _) => {
                h784
            }
        }, match v2 {
            Adt2::C0(h785, _, _, _, _, _, _, _, _, _, _) => {
                h785
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f22(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m786 = v3;
    match m786 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f51(v0, v1, v2.clone())?, f49(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f52(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f53(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f54(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f53(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f55(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f56(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f57(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f58(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f59(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f49(v0, v1)?, f49(v0, v2)?))
        }
        Adt4::C15 => {
            f60(v0, v1, v2)
        }
        Adt4::C16(v38, v39) => {
            f61(v0, v1, v2, v38, v39)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v40, v41) => {
            f62(v0, v1, v41, v40)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f49(f50(v0, match v2.clone() {
                Adt3::C0(h787, _, _, _, _, _, _, _) => {
                    h787
                }
            }, match v2.clone() {
                Adt3::C0(_, h788, _, _, _, _, _, _) => {
                    h788
                }
            })?, v1)?, f49(v0, v2)?))
        }
    }
}

pub fn f23(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f45(match v1.clone() {
        Adt3::C0(h789, _, _, _, _, _, _, _) => {
            h789
        }
    })?;
    let v5: u64 = {
        let a790 = v0;
        let a791 = v4;
        let a792 = 0u64;
        nat_quot(a790, a791, a792)
    };
    let v6: u64 = {
        let a793 = v0;
        let a794 = v4;
        let a795 = 0u64;
        nat_rem(a793, a794, a795)
    };
    let a796 = v6;
    let a797 = 0u64;
    if nat_eq(a796, a797) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a798 = v6;
        let a799 = 1u64;
        if nat_eq(a798, a799) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a802 = {
                let a800 = v6;
                let a801 = 1u64;
                nat_add(a800, a801)?
            };
            let a803 = v4;
            if nat_eq(a802, a803) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f49(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a804 = v6;
                    let a805 = 2u64;
                    nat_sub(a804, a805)
                };
                let v8: u64 = {
                    let a806 = v7;
                    let a807 = 3u64;
                    let a808 = 0u64;
                    nat_quot(a806, a807, a808)
                };
                let v9: u64 = {
                    let a809 = v7;
                    let a810 = 3u64;
                    let a811 = 0u64;
                    nat_rem(a809, a810, a811)
                };
                let v10: u64 = f49({
                    let a815 = {
                        let a813 = v5;
                        let a814 = match v1.clone() {
                            Adt3::C0(h812, _, _, _, _, _, _, _) => {
                                h812
                            }
                        };
                        nat_mul(a813, a814)?
                    };
                    let a816 = v8;
                    nat_add(a815, a816)?
                }, v1)?;
                let a817 = v9;
                let a818 = 0u64;
                if nat_eq(a817, a818) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a819 = v9;
                    let a820 = 1u64;
                    if nat_eq(a819, a820) {
                        let a821 = v3;
                        let a822 = 2u64;
                        if nat_eq(a821, a822) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a823 = v3;
                        let a824 = 2u64;
                        if if nat_eq(a823, a824) {
                            true
                        } else {
                            let a825 = v3;
                            let a826 = 3u64;
                            nat_eq(a825, a826)
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

pub fn f24(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a828 = 2u64;
        let a829 = match v1.clone() {
            Adt3::C0(_, h827, _, _, _, _, _, _) => {
                h827
            }
        };
        nat_mul(a828, a829)?
    };
    let a830 = v0;
    let a831 = v5;
    if nat_lt(a830, a831) {
        let a835 = {
            let a832 = v0;
            let a833 = 2u64;
            let a834 = 0u64;
            nat_rem(a832, a833, a834)
        };
        let a836 = 0u64;
        if nat_eq(a835, a836) {
            Ok(Adt5::C4(19u64, 4u64, f49({
                let a837 = v0;
                let a838 = 2u64;
                let a839 = 0u64;
                nat_quot(a837, a838, a839)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a840 = v4;
            let a841 = 2147483648u64;
            if if nat_lt(a840, a841) {
                let a843 = v4;
                let a844 = match v2 {
                    Adt3::C0(_, h842, _, _, _, _, _, _) => {
                        h842
                    }
                };
                nat_lt(a843, a844)
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
            let a847 = match v2.clone() {
                Adt3::C0(h845, _, _, _, _, _, _, _) => {
                    h845
                }
            };
            let a848 = match v2.clone() {
                Adt3::C0(_, h846, _, _, _, _, _, _) => {
                    h846
                }
            };
            nat_mul(a847, a848)?
        };
        let a851 = {
            let a849 = v0;
            let a850 = v5;
            nat_sub(a849, a850)
        };
        let a852 = v6;
        if nat_lt(a851, a852) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f49({
                let a853 = v0;
                let a854 = v5;
                nat_sub(a853, a854)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a857 = {
                    let a855 = v0;
                    let a856 = v5;
                    nat_sub(a855, a856)
                };
                let a858 = v6;
                nat_sub(a857, a858)
            };
            let v8: u64 = {
                let a862 = 1u64;
                let a863 = {
                    let a860 = 2u64;
                    let a861 = match v1.clone() {
                        Adt3::C0(h859, _, _, _, _, _, _, _) => {
                            h859
                        }
                    };
                    nat_mul(a860, a861)?
                };
                nat_add(a862, a863)?
            };
            let v9: u64 = {
                let a864 = v7;
                let a865 = v8;
                let a866 = 0u64;
                nat_quot(a864, a865, a866)
            };
            let v10: u64 = {
                let a867 = v7;
                let a868 = v8;
                let a869 = 0u64;
                nat_rem(a867, a868, a869)
            };
            let a870 = v10;
            let a871 = 0u64;
            if nat_eq(a870, a871) {
                Ok(Adt5::C4(19u64, 4u64, f49(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a875 = {
                    let a872 = v10;
                    let a873 = 2u64;
                    let a874 = 0u64;
                    nat_rem(a872, a873, a874)
                };
                let a876 = 1u64;
                if nat_eq(a875, a876) {
                    Ok(Adt5::C4(6u64, 0u64, f49({
                        let a885 = {
                            let a878 = v9;
                            let a879 = match v1.clone() {
                                Adt3::C0(h877, _, _, _, _, _, _, _) => {
                                    h877
                                }
                            };
                            nat_mul(a878, a879)?
                        };
                        let a886 = {
                            let a882 = {
                                let a880 = v10;
                                let a881 = 1u64;
                                nat_sub(a880, a881)
                            };
                            let a883 = 2u64;
                            let a884 = 0u64;
                            nat_quot(a882, a883, a884)
                        };
                        nat_add(a885, a886)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f49({
                        let a895 = {
                            let a888 = v4;
                            let a889 = match v1.clone() {
                                Adt3::C0(h887, _, _, _, _, _, _, _) => {
                                    h887
                                }
                            };
                            nat_mul(a888, a889)?
                        };
                        let a896 = {
                            let a892 = {
                                let a890 = v10;
                                let a891 = 1u64;
                                nat_sub(a890, a891)
                            };
                            let a893 = 2u64;
                            let a894 = 0u64;
                            nat_quot(a892, a893, a894)
                        };
                        nat_add(a895, a896)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f49({
                        let a905 = {
                            let a898 = v4;
                            let a899 = match v1 {
                                Adt3::C0(h897, _, _, _, _, _, _, _) => {
                                    h897
                                }
                            };
                            nat_mul(a898, a899)?
                        };
                        let a906 = {
                            let a902 = {
                                let a900 = v10;
                                let a901 = 1u64;
                                nat_sub(a900, a901)
                            };
                            let a903 = 2u64;
                            let a904 = 0u64;
                            nat_quot(a902, a903, a904)
                        };
                        nat_add(a905, a906)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f25(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a907 = v1;
    let a908 = f63(v2.clone(), v4.clone())?;
    if nat_lt(a907, a908) {
        let a911 = v1;
        let a912 = {
            let a909 = 2u64;
            let a910 = f46(v4.clone())?;
            nat_mul(a909, a910)?
        };
        if nat_lt(a911, a912) {
            let a916 = {
                let a913 = v1;
                let a914 = 2u64;
                let a915 = 0u64;
                nat_rem(a913, a914, a915)
            };
            let a917 = 0u64;
            if nat_eq(a916, a917) {
                f64({
                    let a918 = v1;
                    let a919 = 2u64;
                    let a920 = 0u64;
                    nat_quot(a918, a919, a920)
                }, v4)
            } else {
                if f65(match v2 {
                    Adt3::C0(_, h921, _, _, _, _, _, _) => {
                        h921
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a924 = v1;
                let a925 = {
                    let a922 = 2u64;
                    let a923 = f46(v4.clone())?;
                    nat_mul(a922, a923)?
                };
                nat_sub(a924, a925)
            };
            let v7: u64 = {
                let a929 = v6;
                let a930 = {
                    let a927 = 1u64;
                    let a928 = match v2.clone() {
                        Adt3::C0(h926, _, _, _, _, _, _, _) => {
                            h926
                        }
                    };
                    nat_add(a927, a928)?
                };
                let a931 = 0u64;
                nat_quot(a929, a930, a931)
            };
            let v8: u64 = {
                let a935 = v6;
                let a936 = {
                    let a933 = 1u64;
                    let a934 = match v2.clone() {
                        Adt3::C0(h932, _, _, _, _, _, _, _) => {
                            h932
                        }
                    };
                    nat_add(a933, a934)?
                };
                let a937 = 0u64;
                nat_rem(a935, a936, a937)
            };
            let a938 = v8;
            let a939 = 0u64;
            if nat_eq(a938, a939) {
                f64(v7, v4)
            } else {
                f66(v0, v7, {
                    let a940 = v8;
                    let a941 = 1u64;
                    nat_sub(a940, a941)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f26(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f67(match v0.clone() {
        Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt2::C0(_, h943, _, _, _, _, _, _, _, _, _) => {
            h943
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h944, _, _, _, _, _, _, _, _) => {
            h944
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
            h945
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h946, _, _, _, _, _, _) => {
            h946
        }
    }, match v1.clone() {
        Adt2::C0(h947, _, _, _, _, _, _, _, _, _, _) => {
            h947
        }
    }, match v1.clone() {
        Adt2::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
            h948
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
            h949
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h950, _, _, _, _, _, _, _) => {
            h950
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h951, _, _, _, _, _, _) => {
            h951
        }
    }, match v2.clone() {
        Adt2::C0(h952, _, _, _, _, _, _, _, _, _, _) => {
            h952
        }
    }, match v2.clone() {
        Adt2::C0(_, h953, _, _, _, _, _, _, _, _, _) => {
            h953
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h954, _, _, _, _, _, _, _, _) => {
            h954
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h955, _, _, _, _, _, _, _) => {
            h955
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h956, _, _, _, _, _, _) => {
            h956
        }
    }) {
        let m958 = f33(match v0.clone() {
            Adt2::C0(h957, _, _, _, _, _, _, _, _, _, _) => {
                h957
            }
        });
        match m958 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                let a964 = {
                    let a961 = match v0.clone() {
                        Adt2::C0(_, h959, _, _, _, _, _, _, _, _, _) => {
                            h959
                        }
                    };
                    let a962 = match v3.clone() {
                        Adt8::C0(_, h960, _) => {
                            h960
                        }
                    };
                    let a963 = 0u64;
                    nat_rem(a961, a962, a963)
                };
                let a965 = 0u64;
                if if nat_eq(a964, a965) {
                    if f68(match v0.clone() {
                        Adt2::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
                            h966
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h967, _, _, _, _, _, _, _, _, _) => {
                            h967
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h968, _, _, _, _, _, _, _, _) => {
                            h968
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h969, _, _, _, _, _, _, _) => {
                            h969
                        }
                    }) {
                        true
                    } else {
                        let a971 = 4u64;
                        let a972 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h970, _, _, _, _, _) => {
                                h970
                            }
                        };
                        if if nat_le(a971, a972) {
                            let a974 = match v3.clone() {
                                Adt8::C0(_, h973, _) => {
                                    h973
                                }
                            };
                            let a975 = 1u64;
                            if nat_eq(a974, a975) {
                                true
                            } else {
                                let a978 = match v3.clone() {
                                    Adt8::C0(_, _, h976) => {
                                        h976
                                    }
                                };
                                let a979 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h977, _, _, _, _, _) => {
                                        h977
                                    }
                                };
                                nat_le(a978, a979)
                            }
                        } else {
                            false
                        } {
                            if f27(v0.clone(), v2.clone())? {
                                f27(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f29(v0.clone(), match v3 {
                        Adt8::C0(_, h980, _) => {
                            h980
                        }
                    });
                    Ok(Adt6::C4(if f68(match v0.clone() {
                        Adt2::C0(_, h981, _, _, _, _, _, _, _, _, _) => {
                            h981
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h982, _, _, _, _, _, _, _, _, _) => {
                            h982
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h983, _, _, _, _, _, _, _, _) => {
                            h983
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h984, _, _, _, _, _, _, _) => {
                            h984
                        }
                    }) {
                        0u64
                    } else {
                        f63(v4.clone(), f29(v1.clone(), 1u64))?
                    }, 4u64, v4, f29(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h985, _, _, _, _, _, _, _, _, _, _) => {
                            h985
                        }
                    }, f29(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a987 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h986, _) => {
            h986
        }
    };
    let a988 = 0u64;
    if nat_eq(a987, a988) {
        Ok(true)
    } else {
        let a990 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h989, _) => {
                h989
            }
        };
        let a991 = 0u64;
        if nat_eq(a990, a991) {
            Ok(true)
        } else {
            let a993 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h992) => {
                    h992
                }
            };
            let a994 = 0u64;
            if nat_eq(a993, a994) {
                Ok(true)
            } else {
                let a996 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h995) => {
                        h995
                    }
                };
                let a997 = 0u64;
                if nat_eq(a996, a997) {
                    Ok(true)
                } else {
                    let a1003 = {
                        let a1000 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h998) => {
                                h998
                            }
                        };
                        let a1001 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h999, _) => {
                                h999
                            }
                        };
                        nat_add(a1000, a1001)?
                    };
                    let a1004 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1002) => {
                            h1002
                        }
                    };
                    if nat_le(a1003, a1004) {
                        Ok(true)
                    } else {
                        let a1010 = {
                            let a1007 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1005) => {
                                    h1005
                                }
                            };
                            let a1008 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1006, _) => {
                                    h1006
                                }
                            };
                            nat_add(a1007, a1008)?
                        };
                        let a1011 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1009) => {
                                h1009
                            }
                        };
                        Ok(nat_le(a1010, a1011))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<u64> {
    let a1023 = {
        let a1017 = {
            let a1013 = match v0.clone() {
                Adt2::C0(_, h1012, _, _, _, _, _, _, _, _, _) => {
                    h1012
                }
            };
            let a1014 = v1;
            let a1015 = 0u64;
            nat_quot(a1013, a1014, a1015)
        };
        let a1018 = match v0.clone() {
            Adt2::C0(_, _, h1016, _, _, _, _, _, _, _, _) => {
                h1016
            }
        };
        nat_mul(a1017, a1018)?
    };
    let a1024 = {
        let a1021 = match v0.clone() {
            Adt2::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                h1019
            }
        };
        let a1022 = match v0 {
            Adt2::C0(_, _, _, _, h1020, _, _, _, _, _, _) => {
                h1020
            }
        };
        nat_mul(a1021, a1022)?
    };
    nat_mul(a1023, a1024)
}

pub fn f29(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1026 = match v0.clone() {
            Adt2::C0(_, h1025, _, _, _, _, _, _, _, _, _) => {
                h1025
            }
        };
        let a1027 = v1;
        let a1028 = 0u64;
        nat_quot(a1026, a1027, a1028)
    }, match v0.clone() {
        Adt2::C0(_, _, h1029, _, _, _, _, _, _, _, _) => {
            h1029
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1030, _, _, _, _, _, _, _) => {
            h1030
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1031, _, _, _, _, _, _) => {
            h1031
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1032, _, _, _, _, _) => {
            h1032
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1033, _, _, _, _) => {
            h1033
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1034, _, _, _) => {
            h1034
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1035, _, _) => {
            h1035
        }
    })
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1036 = v0;
    let a1037 = 0u64;
    if if if nat_eq(a1036, a1037) {
        true
    } else {
        let a1038 = v1;
        let a1039 = 0u64;
        nat_eq(a1038, a1039)
    } {
        true
    } else {
        let a1040 = v2;
        let a1041 = 0u64;
        nat_eq(a1040, a1041)
    } {
        true
    } else {
        let a1042 = v3;
        let a1043 = 0u64;
        nat_eq(a1042, a1043)
    }
}

pub fn f31(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1046 = match v1.clone() {
        Adt2::C0(h1044, _, _, _, _, _, _, _, _, _, _) => {
            h1044
        }
    };
    let a1047 = match v3.clone() {
        Adt2::C0(h1045, _, _, _, _, _, _, _, _, _, _) => {
            h1045
        }
    };
    if if nat_eq(a1046, a1047) {
        let a1048 = v0;
        let a1049 = 33u64;
        if nat_eq(a1048, a1049) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1051 = f33(match v1.clone() {
            Adt2::C0(h1050, _, _, _, _, _, _, _, _, _, _) => {
                h1050
            }
        });
        match m1051 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h1052, _) => {
                        h1052
                    }
                };
                let v6: u64 = f28(v1.clone(), v5)?;
                let a1057 = {
                    let a1054 = match v1.clone() {
                        Adt2::C0(_, h1053, _, _, _, _, _, _, _, _, _) => {
                            h1053
                        }
                    };
                    let a1055 = v5;
                    let a1056 = 1u64;
                    nat_rem(a1054, a1055, a1056)
                };
                let a1058 = 0u64;
                if if if nat_eq(a1057, a1058) {
                    let a1063 = {
                        let a1060 = match v3.clone() {
                            Adt2::C0(_, h1059, _, _, _, _, _, _, _, _, _) => {
                                h1059
                            }
                        };
                        let a1061 = v5;
                        let a1062 = 1u64;
                        nat_rem(a1060, a1061, a1062)
                    };
                    let a1064 = 0u64;
                    nat_eq(a1063, a1064)
                } else {
                    false
                } {
                    let a1065 = v6;
                    let a1066 = f28(v3.clone(), v5)?;
                    nat_eq(a1065, a1066)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h1067) => {
                            h1067
                        }
                    }, f29(v1, v5), f29(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f32(v0: u64) -> bool {
    let a1068 = v0;
    let a1069 = 0u64;
    if if nat_eq(a1068, a1069) {
        true
    } else {
        let a1070 = v0;
        let a1071 = 1u64;
        nat_eq(a1070, a1071)
    } {
        true
    } else {
        let a1072 = v0;
        let a1073 = 30u64;
        nat_eq(a1072, a1073)
    }
}

pub fn f33(v0: u64) -> Adt7 {
    let a1074 = v0;
    let a1075 = 0u64;
    if nat_eq(a1074, a1075) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a1076 = v0;
        let a1077 = 1u64;
        if nat_eq(a1076, a1077) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a1078 = v0;
            let a1079 = 2u64;
            if nat_eq(a1078, a1079) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a1080 = v0;
                let a1081 = 3u64;
                if nat_eq(a1080, a1081) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a1082 = v0;
                    let a1083 = 6u64;
                    if nat_eq(a1082, a1083) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1084 = v0;
                        let a1085 = 7u64;
                        if nat_eq(a1084, a1085) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1086 = v0;
                            let a1087 = 8u64;
                            if nat_eq(a1086, a1087) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1088 = v0;
                                let a1089 = 9u64;
                                if nat_eq(a1088, a1089) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1090 = v0;
                                    let a1091 = 10u64;
                                    if nat_eq(a1090, a1091) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1092 = v0;
                                        let a1093 = 11u64;
                                        if nat_eq(a1092, a1093) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1094 = v0;
                                            let a1095 = 12u64;
                                            if nat_eq(a1094, a1095) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1096 = v0;
                                                let a1097 = 13u64;
                                                if nat_eq(a1096, a1097) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1098 = v0;
                                                    let a1099 = 14u64;
                                                    if nat_eq(a1098, a1099) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1100 = v0;
                                                        let a1101 = 15u64;
                                                        if nat_eq(a1100, a1101) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1102 = v0;
                                                            let a1103 = 16u64;
                                                            if nat_eq(a1102, a1103) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1104 = v0;
                                                                let a1105 = 17u64;
                                                                if nat_eq(a1104, a1105) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1106 = v0;
                                                                    let a1107 = 18u64;
                                                                    if nat_eq(a1106, a1107) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1108 = v0;
                                                                        let a1109 = 19u64;
                                                                        if nat_eq(a1108, a1109) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1110 = v0;
                                                                            let a1111 = 20u64;
                                                                            if nat_eq(a1110, a1111) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1112 = v0;
                                                                                let a1113 = 21u64;
                                                                                if nat_eq(a1112, a1113) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1114 = v0;
                                                                                    let a1115 = 22u64;
                                                                                    if nat_eq(a1114, a1115) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1116 = v0;
                                                                                        let a1117 = 23u64;
                                                                                        if nat_eq(a1116, a1117) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1118 = v0;
                                                                                            let a1119 = 24u64;
                                                                                            if nat_eq(a1118, a1119) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1120 = v0;
                                                                                                let a1121 = 25u64;
                                                                                                if nat_eq(a1120, a1121) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1122 = v0;
                                                                                                    let a1123 = 26u64;
                                                                                                    if nat_eq(a1122, a1123) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1124 = v0;
                                                                                                        let a1125 = 27u64;
                                                                                                        if nat_eq(a1124, a1125) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1126 = v0;
                                                                                                            let a1127 = 28u64;
                                                                                                            if nat_eq(a1126, a1127) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1128 = v0;
                                                                                                                let a1129 = 29u64;
                                                                                                                if nat_eq(a1128, a1129) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1130 = v0;
                                                                                                                    let a1131 = 30u64;
                                                                                                                    if nat_eq(a1130, a1131) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1132 = v0;
                                                                                                                        let a1133 = 34u64;
                                                                                                                        if nat_eq(a1132, a1133) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1134 = v0;
                                                                                                                            let a1135 = 35u64;
                                                                                                                            if nat_eq(a1134, a1135) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1136 = v0;
                                                                                                                                let a1137 = 39u64;
                                                                                                                                if nat_eq(a1136, a1137) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1138 = v0;
                                                                                                                                    let a1139 = 40u64;
                                                                                                                                    if nat_eq(a1138, a1139) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1140 = v0;
                                                                                                                                        let a1141 = 41u64;
                                                                                                                                        if nat_eq(a1140, a1141) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1142 = v0;
                                                                                                                                            let a1143 = 42u64;
                                                                                                                                            if nat_eq(a1142, a1143) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1144 = v0;
                                                                                                                                                let a1145 = 43u64;
                                                                                                                                                if nat_eq(a1144, a1145) {
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

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f30(v4, v5, v6, v7) {
        f30(v0, v1, v2, v3)
    } else {
        let a1149 = {
            let a1146 = v0;
            let a1147 = v4;
            let a1148 = 1u64;
            nat_rem(a1146, a1147, a1148)
        };
        let a1150 = 0u64;
        if if if nat_eq(a1149, a1150) {
            let a1154 = {
                let a1151 = v1;
                let a1152 = v5;
                let a1153 = 1u64;
                nat_rem(a1151, a1152, a1153)
            };
            let a1155 = 0u64;
            nat_eq(a1154, a1155)
        } else {
            false
        } {
            let a1159 = {
                let a1156 = v2;
                let a1157 = v6;
                let a1158 = 1u64;
                nat_rem(a1156, a1157, a1158)
            };
            let a1160 = 0u64;
            nat_eq(a1159, a1160)
        } else {
            false
        } {
            let a1164 = {
                let a1161 = v3;
                let a1162 = v7;
                let a1163 = 1u64;
                nat_rem(a1161, a1162, a1163)
            };
            let a1165 = 0u64;
            nat_eq(a1164, a1165)
        } else {
            false
        }
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1166 = v0;
    let a1167 = v1;
    if nat_eq(a1166, a1167) {
        let a1170 = v4;
        let a1171 = {
            let a1168 = v2;
            let a1169 = v3;
            nat_add(a1168, a1169)?
        };
        Ok(nat_eq(a1170, a1171))
    } else {
        let a1172 = v2;
        let a1173 = v3;
        if nat_eq(a1172, a1173) {
            let a1174 = v4;
            let a1175 = v2;
            Ok(nat_eq(a1174, a1175))
        } else {
            Ok(false)
        }
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1178 = {
            let a1176 = v0;
            let a1177 = f69(v1);
            nat_add(a1176, a1177)?
        };
        let a1179 = f69(v2);
        nat_add(a1178, a1179)?
    };
    let v5: u64 = {
        let a1180 = f70(v1);
        let a1181 = f70(v2);
        nat_add(a1180, a1181)?
    };
    let a1182 = v1;
    let a1183 = 4294967295u64;
    if if nat_le(a1182, a1183) {
        let a1184 = v2;
        let a1185 = 4294967295u64;
        nat_le(a1184, a1185)
    } else {
        false
    } {
        let a1186 = v5;
        let a1187 = v4;
        if nat_le(a1186, a1187) {
            let a1190 = v3;
            let a1191 = {
                let a1188 = v4;
                let a1189 = v5;
                nat_sub(a1188, a1189)
            };
            Ok(nat_eq(a1190, a1191))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64, v1: u64) -> R<bool> {
    let a1192 = v1;
    let a1193 = 4294967295u64;
    if nat_le(a1192, a1193) {
        let a1196 = {
            let a1194 = f69(v1);
            let a1195 = f70(v1);
            nat_add(a1194, a1195)?
        };
        let a1197 = v0;
        Ok(nat_lt(a1196, a1197))
    } else {
        Ok(false)
    }
}

pub fn f38(v0: Adt2, v1: Adt2) -> bool {
    let a1200 = match v0.clone() {
        Adt2::C0(_, h1198, _, _, _, _, _, _, _, _, _) => {
            h1198
        }
    };
    let a1201 = match v1.clone() {
        Adt2::C0(_, h1199, _, _, _, _, _, _, _, _, _) => {
            h1199
        }
    };
    if if if nat_eq(a1200, a1201) {
        let a1204 = match v0.clone() {
            Adt2::C0(_, _, h1202, _, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        let a1205 = match v1.clone() {
            Adt2::C0(_, _, h1203, _, _, _, _, _, _, _, _) => {
                h1203
            }
        };
        nat_eq(a1204, a1205)
    } else {
        false
    } {
        let a1208 = match v0.clone() {
            Adt2::C0(_, _, _, h1206, _, _, _, _, _, _, _) => {
                h1206
            }
        };
        let a1209 = match v1.clone() {
            Adt2::C0(_, _, _, h1207, _, _, _, _, _, _, _) => {
                h1207
            }
        };
        nat_eq(a1208, a1209)
    } else {
        false
    } {
        let a1212 = match v0 {
            Adt2::C0(_, _, _, _, h1210, _, _, _, _, _, _) => {
                h1210
            }
        };
        let a1213 = match v1 {
            Adt2::C0(_, _, _, _, h1211, _, _, _, _, _, _) => {
                h1211
            }
        };
        nat_eq(a1212, a1213)
    } else {
        false
    }
}

pub fn f39(v0: Adt2) -> R<bool> {
    let a1215 = match v0.clone() {
        Adt2::C0(_, h1214, _, _, _, _, _, _, _, _, _) => {
            h1214
        }
    };
    let a1216 = 1u64;
    if if if if nat_eq(a1215, a1216) {
        true
    } else {
        let a1218 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1217, _, _, _, _, _) => {
                h1217
            }
        };
        let a1219 = 4u64;
        nat_eq(a1218, a1219)
    } {
        let a1221 = match v0.clone() {
            Adt2::C0(_, _, h1220, _, _, _, _, _, _, _, _) => {
                h1220
            }
        };
        let a1222 = 1u64;
        if nat_eq(a1221, a1222) {
            true
        } else {
            let a1227 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1223, _, _, _, _) => {
                    h1223
                }
            };
            let a1228 = {
                let a1225 = match v0.clone() {
                    Adt2::C0(_, h1224, _, _, _, _, _, _, _, _, _) => {
                        h1224
                    }
                };
                let a1226 = 4u64;
                nat_mul(a1225, a1226)?
            };
            nat_eq(a1227, a1228)
        }
    } else {
        false
    } {
        let a1230 = match v0.clone() {
            Adt2::C0(_, _, _, h1229, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1231 = 1u64;
        if nat_eq(a1230, a1231) {
            true
        } else {
            let a1239 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1232, _, _, _) => {
                    h1232
                }
            };
            let a1240 = {
                let a1237 = {
                    let a1235 = match v0.clone() {
                        Adt2::C0(_, h1233, _, _, _, _, _, _, _, _, _) => {
                            h1233
                        }
                    };
                    let a1236 = match v0.clone() {
                        Adt2::C0(_, _, h1234, _, _, _, _, _, _, _, _) => {
                            h1234
                        }
                    };
                    nat_mul(a1235, a1236)?
                };
                let a1238 = 4u64;
                nat_mul(a1237, a1238)?
            };
            nat_eq(a1239, a1240)
        }
    } else {
        false
    } {
        let a1242 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1241, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1243 = 1u64;
        if nat_eq(a1242, a1243) {
            Ok(true)
        } else {
            let a1254 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1244, _, _) => {
                    h1244
                }
            };
            let a1255 = {
                let a1252 = {
                    let a1250 = {
                        let a1247 = match v0.clone() {
                            Adt2::C0(_, h1245, _, _, _, _, _, _, _, _, _) => {
                                h1245
                            }
                        };
                        let a1248 = match v0.clone() {
                            Adt2::C0(_, _, h1246, _, _, _, _, _, _, _, _) => {
                                h1246
                            }
                        };
                        nat_mul(a1247, a1248)?
                    };
                    let a1251 = match v0 {
                        Adt2::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                            h1249
                        }
                    };
                    nat_mul(a1250, a1251)?
                };
                let a1253 = 4u64;
                nat_mul(a1252, a1253)?
            };
            Ok(nat_eq(a1254, a1255))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f71(v0.clone(), false)?;
    let v6: u64 = f71(v0, true)?;
    let a1256 = v6;
    let a1257 = v1;
    if if nat_le(a1256, a1257) {
        let a1264 = {
            let a1262 = {
                let a1258 = v1;
                let a1259 = v5;
                nat_add(a1258, a1259)?
            };
            let a1263 = {
                let a1260 = v3;
                let a1261 = 0u64;
                if nat_eq(a1260, a1261) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1262, a1263)?
        };
        let a1265 = v2;
        nat_le(a1264, a1265)
    } else {
        false
    } {
        if v4 {
            let a1270 = {
                let a1268 = {
                    let a1266 = v1;
                    let a1267 = v5;
                    nat_add(a1266, a1267)?
                };
                let a1269 = v6;
                nat_sub(a1268, a1269)
            };
            let a1271 = v2;
            Ok(nat_lt(a1270, a1271))
        } else {
            let a1276 = {
                let a1274 = {
                    let a1272 = v1;
                    let a1273 = v5;
                    nat_add(a1272, a1273)?
                };
                let a1275 = v6;
                nat_sub(a1274, a1275)
            };
            let a1277 = v2;
            Ok(nat_le(a1276, a1277))
        }
    } else {
        Ok(false)
    }
}

pub fn f41(v0: u64, v1: u64) -> bool {
    let a1278 = v0;
    let a1279 = 0u64;
    if if nat_eq(a1278, a1279) {
        true
    } else {
        let a1280 = v0;
        let a1281 = 1u64;
        nat_eq(a1280, a1281)
    } {
        let a1282 = v1;
        let a1283 = 4294967296u64;
        nat_lt(a1282, a1283)
    } else {
        false
    }
}

pub fn f42(v0: u64, v1: u64) -> R<u64> {
    let a1288 = {
        let a1286 = v0;
        let a1287 = {
            let a1284 = v1;
            let a1285 = 1u64;
            nat_sub(a1284, a1285)
        };
        nat_add(a1286, a1287)?
    };
    let a1289 = v1;
    let a1290 = 0u64;
    Ok(nat_quot(a1288, a1289, a1290))
}

pub fn f43(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f30(match v0.clone() {
        Adt2::C0(_, h1291, _, _, _, _, _, _, _, _, _) => {
            h1291
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1292, _, _, _, _, _, _, _, _) => {
            h1292
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1293, _, _, _, _, _, _, _) => {
            h1293
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1294, _, _, _, _, _, _) => {
            h1294
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1297 = match v1.clone() {
                Adt2::C0(_, h1295, _, _, _, _, _, _, _, _, _) => {
                    h1295
                }
            };
            let a1298 = match v1.clone() {
                Adt2::C0(_, _, h1296, _, _, _, _, _, _, _, _) => {
                    h1296
                }
            };
            nat_mul(a1297, a1298)?
        };
        let v5: u64 = {
            let a1300 = v4;
            let a1301 = match v1.clone() {
                Adt2::C0(_, _, _, h1299, _, _, _, _, _, _, _) => {
                    h1299
                }
            };
            nat_mul(a1300, a1301)?
        };
        let a1314 = {
            let a1312 = {
                let a1303 = match v0.clone() {
                    Adt2::C0(_, h1302, _, _, _, _, _, _, _, _, _) => {
                        h1302
                    }
                };
                let a1304 = 1u64;
                nat_sub(a1303, a1304)
            };
            let a1313 = {
                let a1310 = f72(match v0.clone() {
                    Adt2::C0(_, _, h1305, _, _, _, _, _, _, _, _) => {
                        h1305
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1306, _, _, _, _, _, _, _, _, _) => {
                        h1306
                    }
                })?;
                let a1311 = f72(match v0 {
                    Adt2::C0(_, _, _, h1307, _, _, _, _, _, _, _) => {
                        h1307
                    }
                }, v2, {
                    let a1308 = v3;
                    let a1309 = v5;
                    nat_mul(a1308, a1309)?
                }, v4)?;
                nat_add(a1310, a1311)?
            };
            nat_add(a1312, a1313)?
        };
        let a1315 = f28(v1, 1u64)?;
        Ok(nat_lt(a1314, a1315))
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1316 = v0;
    let a1317 = 2147483648u64;
    if if if if nat_lt(a1316, a1317) {
        let a1318 = v1;
        let a1319 = 2147483648u64;
        nat_lt(a1318, a1319)
    } else {
        false
    } {
        let a1322 = {
            let a1320 = v0;
            let a1321 = v1;
            nat_mul(a1320, a1321)?
        };
        let a1323 = 2147483648u64;
        nat_lt(a1322, a1323)
    } else {
        false
    } {
        let a1328 = {
            let a1326 = {
                let a1324 = v0;
                let a1325 = 1u64;
                nat_sub(a1324, a1325)
            };
            let a1327 = v2;
            nat_mul(a1326, a1327)?
        };
        let a1329 = 2147483648u64;
        nat_lt(a1328, a1329)
    } else {
        false
    } {
        let a1334 = {
            let a1332 = {
                let a1330 = v1;
                let a1331 = 1u64;
                nat_sub(a1330, a1331)
            };
            let a1333 = v2;
            nat_mul(a1332, a1333)?
        };
        let a1335 = 2147483648u64;
        Ok(nat_lt(a1334, a1335))
    } else {
        Ok(false)
    }
}

pub fn f45(v0: u64) -> R<u64> {
    let a1338 = 3u64;
    let a1339 = {
        let a1336 = v0;
        let a1337 = 3u64;
        nat_mul(a1336, a1337)?
    };
    nat_add(a1338, a1339)
}

pub fn f46(v0: Adt3) -> R<u64> {
    if f30(match v0.clone() {
        Adt3::C0(h1340, _, _, _, _, _, _, _) => {
            h1340
        }
    }, match v0.clone() {
        Adt3::C0(_, h1341, _, _, _, _, _, _) => {
            h1341
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1342, _, _, _, _, _) => {
            h1342
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1343, _, _, _, _) => {
            h1343
        }
    }) {
        Ok(0u64)
    } else {
        let a1352 = match v0.clone() {
            Adt3::C0(h1344, _, _, _, _, _, _, _) => {
                h1344
            }
        };
        let a1353 = {
            let a1350 = match v0.clone() {
                Adt3::C0(_, h1345, _, _, _, _, _, _) => {
                    h1345
                }
            };
            let a1351 = {
                let a1348 = match v0.clone() {
                    Adt3::C0(_, _, h1346, _, _, _, _, _) => {
                        h1346
                    }
                };
                let a1349 = match v0 {
                    Adt3::C0(_, _, _, h1347, _, _, _, _) => {
                        h1347
                    }
                };
                nat_mul(a1348, a1349)?
            };
            nat_mul(a1350, a1351)?
        };
        nat_mul(a1352, a1353)
    }
}

pub fn f47(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1354 = v1;
    let a1355 = 13u64;
    if if nat_eq(a1354, a1355) {
        f30(match v0.clone() {
            Adt3::C0(h1356, _, _, _, _, _, _, _) => {
                h1356
            }
        }, match v0.clone() {
            Adt3::C0(_, h1357, _, _, _, _, _, _) => {
                h1357
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1358, _, _, _, _, _) => {
                h1358
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1359, _, _, _, _) => {
                h1359
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1365 = match v0.clone() {
                Adt3::C0(_, h1360, _, _, _, _, _, _) => {
                    h1360
                }
            };
            let a1366 = {
                let a1363 = match v0.clone() {
                    Adt3::C0(_, _, h1361, _, _, _, _, _) => {
                        h1361
                    }
                };
                let a1364 = match v0.clone() {
                    Adt3::C0(_, _, _, h1362, _, _, _, _) => {
                        h1362
                    }
                };
                nat_mul(a1363, a1364)?
            };
            nat_mul(a1365, a1366)?
        };
        let a1367 = v1;
        let a1368 = 13u64;
        if nat_eq(a1367, a1368) {
            let a1377 = 2u64;
            let a1378 = if f73(v0.clone(), v2) {
                let a1370 = v3;
                let a1371 = f74(match v0 {
                    Adt3::C0(h1369, _, _, _, _, _, _, _) => {
                        h1369
                    }
                }, false)?;
                nat_mul(a1370, a1371)?
            } else {
                let a1375 = v3;
                let a1376 = {
                    let a1373 = match v0 {
                        Adt3::C0(h1372, _, _, _, _, _, _, _) => {
                            h1372
                        }
                    };
                    let a1374 = 2u64;
                    nat_mul(a1373, a1374)?
                };
                nat_mul(a1375, a1376)?
            };
            nat_add(a1377, a1378)
        } else {
            let a1382 = v3;
            let a1383 = f74(match v0 {
                Adt3::C0(h1379, _, _, _, _, _, _, _) => {
                    h1379
                }
            }, {
                let a1380 = v1;
                let a1381 = 16u64;
                nat_eq(a1380, a1381)
            })?;
            nat_mul(a1382, a1383)
        }
    }
}

pub fn f48(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1384 = v3;
    match m1384 {
        None => {
            f75(v0, v1, match v2.clone() {
                Adt2::C0(h1385, _, _, _, _, _, _, _, _, _, _) => {
                    h1385
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1386, _, _, _, _, _, _, _, _, _, _) => {
                    h1386
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1387, _, _, _, _, _, _, _, _, _) => {
                    h1387
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1388, _, _, _, _, _, _, _, _) => {
                    h1388
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1389, _, _, _, _, _, _, _) => {
                    h1389
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1390, _, _, _, _, _, _) => {
                    h1390
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1391, _, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1392, _, _, _, _, _, _, _, _) => {
                    h1392
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1393, _, _, _, _, _, _, _) => {
                    h1393
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1394, _, _, _, _, _, _) => {
                    h1394
                }
            })
        }
        Some(v5) => {
            f75(v0, v1, match v2.clone() {
                Adt2::C0(h1395, _, _, _, _, _, _, _, _, _, _) => {
                    h1395
                }
            }, match v5.clone() {
                Adt2::C0(h1396, _, _, _, _, _, _, _, _, _, _) => {
                    h1396
                }
            }, match v4.clone() {
                Adt2::C0(h1397, _, _, _, _, _, _, _, _, _, _) => {
                    h1397
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1398, _, _, _, _, _, _, _, _, _) => {
                    h1398
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1399, _, _, _, _, _, _, _, _) => {
                    h1399
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1400, _, _, _, _, _, _, _) => {
                    h1400
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1401, _, _, _, _, _, _) => {
                    h1401
                }
            }, match v5.clone() {
                Adt2::C0(_, h1402, _, _, _, _, _, _, _, _, _) => {
                    h1402
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1403, _, _, _, _, _, _, _, _) => {
                    h1403
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1404, _, _, _, _, _, _, _) => {
                    h1404
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1405, _, _, _, _, _, _) => {
                    h1405
                }
            }, match v4.clone() {
                Adt2::C0(_, h1406, _, _, _, _, _, _, _, _, _) => {
                    h1406
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1407, _, _, _, _, _, _, _, _) => {
                    h1407
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1408, _, _, _, _, _, _, _) => {
                    h1408
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1409, _, _, _, _, _, _) => {
                    h1409
                }
            })
        }
    }
}

pub fn f49(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1411 = v0;
        let a1412 = match v1.clone() {
            Adt3::C0(h1410, _, _, _, _, _, _, _) => {
                h1410
            }
        };
        let a1413 = 0u64;
        nat_quot(a1411, a1412, a1413)
    };
    let v3: u64 = {
        let a1415 = v2;
        let a1416 = match v1.clone() {
            Adt3::C0(_, h1414, _, _, _, _, _, _) => {
                h1414
            }
        };
        let a1417 = 0u64;
        nat_quot(a1415, a1416, a1417)
    };
    f76({
        let a1419 = v0;
        let a1420 = match v1.clone() {
            Adt3::C0(h1418, _, _, _, _, _, _, _) => {
                h1418
            }
        };
        let a1421 = 0u64;
        nat_rem(a1419, a1420, a1421)
    }, {
        let a1423 = v2;
        let a1424 = match v1.clone() {
            Adt3::C0(_, h1422, _, _, _, _, _, _) => {
                h1422
            }
        };
        let a1425 = 0u64;
        nat_rem(a1423, a1424, a1425)
    }, {
        let a1427 = v3;
        let a1428 = match v1.clone() {
            Adt3::C0(_, _, h1426, _, _, _, _, _) => {
                h1426
            }
        };
        let a1429 = 0u64;
        nat_rem(a1427, a1428, a1429)
    }, {
        let a1431 = v3;
        let a1432 = match v1.clone() {
            Adt3::C0(_, _, h1430, _, _, _, _, _) => {
                h1430
            }
        };
        let a1433 = 0u64;
        nat_quot(a1431, a1432, a1433)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1434, _, _, _) => {
            h1434
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1435, _, _) => {
            h1435
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1436, _) => {
            h1436
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1437) => {
            h1437
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1455 = {
        let a1450 = f77(v2, {
            let a1441 = {
                let a1438 = v0;
                let a1439 = v1;
                let a1440 = 0u64;
                nat_quot(a1438, a1439, a1440)
            };
            let a1442 = v2;
            let a1443 = 0u64;
            nat_rem(a1441, a1442, a1443)
        }, {
            let a1447 = {
                let a1444 = v0;
                let a1445 = v1;
                let a1446 = 0u64;
                nat_quot(a1444, a1445, a1446)
            };
            let a1448 = v2;
            let a1449 = 0u64;
            nat_quot(a1447, a1448, a1449)
        })?;
        let a1451 = v1;
        nat_mul(a1450, a1451)?
    };
    let a1456 = {
        let a1452 = v0;
        let a1453 = v1;
        let a1454 = 0u64;
        nat_rem(a1452, a1453, a1454)
    };
    nat_add(a1455, a1456)
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1458 = v0;
        let a1459 = match v2.clone() {
            Adt3::C0(h1457, _, _, _, _, _, _, _) => {
                h1457
            }
        };
        let a1460 = 0u64;
        nat_quot(a1458, a1459, a1460)
    };
    let v4: u64 = {
        let a1462 = v3;
        let a1463 = match v2.clone() {
            Adt3::C0(_, h1461, _, _, _, _, _, _) => {
                h1461
            }
        };
        let a1464 = 0u64;
        nat_quot(a1462, a1463, a1464)
    };
    f78({
        let a1466 = v0;
        let a1467 = match v2.clone() {
            Adt3::C0(h1465, _, _, _, _, _, _, _) => {
                h1465
            }
        };
        let a1468 = 0u64;
        nat_rem(a1466, a1467, a1468)
    }, {
        let a1470 = v3;
        let a1471 = match v2.clone() {
            Adt3::C0(_, h1469, _, _, _, _, _, _) => {
                h1469
            }
        };
        let a1472 = 0u64;
        nat_rem(a1470, a1471, a1472)
    }, {
        let a1474 = v4;
        let a1475 = match v2.clone() {
            Adt3::C0(_, _, h1473, _, _, _, _, _) => {
                h1473
            }
        };
        let a1476 = 0u64;
        nat_rem(a1474, a1475, a1476)
    }, {
        let a1478 = v4;
        let a1479 = match v2 {
            Adt3::C0(_, _, h1477, _, _, _, _, _) => {
                h1477
            }
        };
        let a1480 = 0u64;
        nat_quot(a1478, a1479, a1480)
    }, match v1.clone() {
        Adt3::C0(h1481, _, _, _, _, _, _, _) => {
            h1481
        }
    }, match v1.clone() {
        Adt3::C0(_, h1482, _, _, _, _, _, _) => {
            h1482
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1483, _, _, _, _, _) => {
            h1483
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1484, _, _, _, _) => {
            h1484
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1485, _, _, _) => {
            h1485
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1486, _, _) => {
            h1486
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1487, _) => {
            h1487
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1488) => {
            h1488
        }
    })
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1490 = v0;
        let a1491 = match v3.clone() {
            Adt3::C0(h1489, _, _, _, _, _, _, _) => {
                h1489
            }
        };
        let a1492 = 0u64;
        nat_quot(a1490, a1491, a1492)
    };
    let v6: u64 = {
        let a1494 = v5;
        let a1495 = match v3.clone() {
            Adt3::C0(_, h1493, _, _, _, _, _, _) => {
                h1493
            }
        };
        let a1496 = 0u64;
        nat_quot(a1494, a1495, a1496)
    };
    let v7: u64 = {
        let a1498 = v0;
        let a1499 = match v3.clone() {
            Adt3::C0(h1497, _, _, _, _, _, _, _) => {
                h1497
            }
        };
        let a1500 = 0u64;
        nat_rem(a1498, a1499, a1500)
    };
    let v8: u64 = {
        let a1502 = v5;
        let a1503 = match v3.clone() {
            Adt3::C0(_, h1501, _, _, _, _, _, _) => {
                h1501
            }
        };
        let a1504 = 0u64;
        nat_rem(a1502, a1503, a1504)
    };
    let v9: u64 = {
        let a1506 = v6;
        let a1507 = match v3.clone() {
            Adt3::C0(_, _, h1505, _, _, _, _, _) => {
                h1505
            }
        };
        let a1508 = 0u64;
        nat_rem(a1506, a1507, a1508)
    };
    let v10: u64 = {
        let a1510 = v6;
        let a1511 = match v3.clone() {
            Adt3::C0(_, _, h1509, _, _, _, _, _) => {
                h1509
            }
        };
        let a1512 = 0u64;
        nat_quot(a1510, a1511, a1512)
    };
    let v11: u64 = {
        let a1513 = v4;
        let a1514 = 0u64;
        if nat_eq(a1513, a1514) {
            match v1.clone() {
                Adt3::C0(h1515, _, _, _, _, _, _, _) => {
                    h1515
                }
            }
        } else {
            let a1516 = v4;
            let a1517 = 1u64;
            if nat_eq(a1516, a1517) {
                match v1.clone() {
                    Adt3::C0(_, h1518, _, _, _, _, _, _) => {
                        h1518
                    }
                }
            } else {
                let a1519 = v4;
                let a1520 = 2u64;
                if nat_eq(a1519, a1520) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1521, _, _, _, _, _) => {
                            h1521
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1522, _, _, _, _) => {
                            h1522
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1529 = v11;
        let a1530 = {
            let a1523 = v4;
            let a1524 = 0u64;
            if nat_eq(a1523, a1524) {
                v7
            } else {
                let a1525 = v4;
                let a1526 = 1u64;
                if nat_eq(a1525, a1526) {
                    v8
                } else {
                    let a1527 = v4;
                    let a1528 = 2u64;
                    if nat_eq(a1527, a1528) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1529, a1530)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f76(if if v12 {
        let a1531 = v4;
        let a1532 = 0u64;
        nat_eq(a1531, a1532)
    } else {
        false
    } {
        let a1533 = v7;
        let a1534 = v11;
        nat_sub(a1533, a1534)
    } else {
        v7
    }, if if v12 {
        let a1535 = v4;
        let a1536 = 1u64;
        nat_eq(a1535, a1536)
    } else {
        false
    } {
        let a1537 = v8;
        let a1538 = v11;
        nat_sub(a1537, a1538)
    } else {
        v8
    }, if if v12 {
        let a1539 = v4;
        let a1540 = 2u64;
        nat_eq(a1539, a1540)
    } else {
        false
    } {
        let a1541 = v9;
        let a1542 = v11;
        nat_sub(a1541, a1542)
    } else {
        v9
    }, if if v12 {
        let a1543 = v4;
        let a1544 = 3u64;
        nat_eq(a1543, a1544)
    } else {
        false
    } {
        let a1545 = v10;
        let a1546 = v11;
        nat_sub(a1545, a1546)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1547, _, _, _) => {
            h1547
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1548, _, _) => {
            h1548
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1549, _) => {
            h1549
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1550) => {
            h1550
        }
    })?, f49(v0, v3)?))
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f79(v0, v2.clone(), 0u64);
    let v9: u64 = f79(v0, v2.clone(), 1u64);
    let v10: u64 = f79(v0, v2.clone(), 2u64);
    let v11: u64 = f79(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f80(v1.clone(), f81(v8, v3, match v1.clone() {
            Adt3::C0(h1551, _, _, _, _, _, _, _) => {
                h1551
            }
        })?, f81(v9, v4, match v1.clone() {
            Adt3::C0(_, h1552, _, _, _, _, _, _) => {
                h1552
            }
        })?, f81(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1553, _, _, _, _, _) => {
                h1553
            }
        })?, f81(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1554, _, _, _, _) => {
                h1554
            }
        })?)?, f49(v0, v2)?))
    } else {
        if if if if f82(v8, v3, match v1.clone() {
            Adt3::C0(h1555, _, _, _, _, _, _, _) => {
                h1555
            }
        })? {
            f82(v9, v4, match v1.clone() {
                Adt3::C0(_, h1556, _, _, _, _, _, _) => {
                    h1556
                }
            })?
        } else {
            false
        } {
            f82(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1557, _, _, _, _, _) => {
                    h1557
                }
            })?
        } else {
            false
        } {
            f82(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1558, _, _, _, _) => {
                    h1558
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f80(v1, f83(v8, v3)?, f83(v9, v4)?, f83(v10, v5)?, f83(v11, v6)?)?, f49(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f79(v0, v2.clone(), 0u64);
    let v5: u64 = f79(v0, v2.clone(), 1u64);
    let v6: u64 = f79(v0, v2.clone(), 2u64);
    let v7: u64 = f79(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f80(v1.clone(), f84(v4, v3, match v1 {
        Adt3::C0(h1559, _, _, _, _, _, _, _) => {
            h1559
        }
    })?, v5, v6, v7)?, f49(v0, v2)?))
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1560 = v0;
    let a1561 = v3;
    if nat_lt(a1560, a1561) {
        Ok(Adt5::C1(false, {
            let a1562 = v0;
            let a1563 = 4u64;
            nat_mul(a1562, a1563)?
        }, {
            let a1564 = v0;
            let a1565 = 4u64;
            nat_mul(a1564, a1565)?
        }))
    } else {
        let v6: u64 = {
            let a1566 = v0;
            let a1567 = v3;
            nat_sub(a1566, a1567)
        };
        let v7: u64 = f49(v6, v1)?;
        let v8: u64 = f85(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1568 = v0;
    let a1569 = v3;
    if nat_lt(a1568, a1569) {
        Ok(Adt5::C1(true, 0u64, f49(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1570 = v0;
            let a1571 = v3;
            nat_sub(a1570, a1571)
        };
        let v5: u64 = f51(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f49(v4, v1)?, 3u64, v5))
    }
}

pub fn f57(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1573 = v1;
        let a1574 = match v3.clone() {
            Adt3::C0(h1572, _, _, _, _, _, _, _) => {
                h1572
            }
        };
        let a1575 = 0u64;
        nat_rem(a1573, a1574, a1575)
    };
    let v5: u64 = {
        let a1581 = {
            let a1577 = v1;
            let a1578 = match v3.clone() {
                Adt3::C0(h1576, _, _, _, _, _, _, _) => {
                    h1576
                }
            };
            let a1579 = 0u64;
            nat_quot(a1577, a1578, a1579)
        };
        let a1582 = match v3.clone() {
            Adt3::C0(_, h1580, _, _, _, _, _, _) => {
                h1580
            }
        };
        let a1583 = 0u64;
        nat_rem(a1581, a1582, a1583)
    };
    let v6: bool = f86(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f51(v1, v2, v3.clone())?
    } else {
        0u64
    }, f49(v1, v3)?))
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1584 = v3;
        let a1585 = 8u64;
        nat_add(a1584, a1585)?
    }, 0u64, f49(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f51(v0, v5, v2.clone())?
    }, 3u64, f49(v0, v2)?))
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1587 = v0;
        let a1588 = match v2.clone() {
            Adt3::C0(h1586, _, _, _, _, _, _, _) => {
                h1586
            }
        };
        let a1589 = 0u64;
        nat_rem(a1587, a1588, a1589)
    };
    let v7: u64 = {
        let a1591 = v0;
        let a1592 = match v2.clone() {
            Adt3::C0(h1590, _, _, _, _, _, _, _) => {
                h1590
            }
        };
        let a1593 = 0u64;
        nat_quot(a1591, a1592, a1593)
    };
    let v8: u64 = {
        let a1595 = v7;
        let a1596 = match v2.clone() {
            Adt3::C0(_, h1594, _, _, _, _, _, _) => {
                h1594
            }
        };
        let a1597 = 0u64;
        nat_rem(a1595, a1596, a1597)
    };
    let v9: u64 = {
        let a1599 = v7;
        let a1600 = match v2.clone() {
            Adt3::C0(_, h1598, _, _, _, _, _, _) => {
                h1598
            }
        };
        let a1601 = 0u64;
        nat_quot(a1599, a1600, a1601)
    };
    let v10: u64 = {
        let a1603 = v9;
        let a1604 = match v2.clone() {
            Adt3::C0(_, _, h1602, _, _, _, _, _) => {
                h1602
            }
        };
        let a1605 = 0u64;
        nat_rem(a1603, a1604, a1605)
    };
    let v11: u64 = {
        let a1607 = v9;
        let a1608 = match v2.clone() {
            Adt3::C0(_, _, h1606, _, _, _, _, _) => {
                h1606
            }
        };
        let a1609 = 0u64;
        nat_quot(a1607, a1608, a1609)
    };
    let v12: u64 = f49(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1618 = {
                let a1613 = {
                    let a1610 = v10;
                    let a1611 = v4;
                    let a1612 = 0u64;
                    nat_quot(a1610, a1611, a1612)
                };
                let a1614 = v5;
                nat_mul(a1613, a1614)?
            };
            let a1619 = {
                let a1615 = v8;
                let a1616 = v4;
                let a1617 = 0u64;
                nat_quot(a1615, a1616, a1617)
            };
            nat_add(a1618, a1619)?
        };
        let v14: u64 = {
            let a1639 = v6;
            let a1640 = {
                let a1637 = match v1.clone() {
                    Adt3::C0(h1620, _, _, _, _, _, _, _) => {
                        h1620
                    }
                };
                let a1638 = {
                    let a1635 = {
                        let a1621 = v8;
                        let a1622 = v4;
                        let a1623 = 0u64;
                        nat_rem(a1621, a1622, a1623)
                    };
                    let a1636 = {
                        let a1633 = match v1.clone() {
                            Adt3::C0(_, h1624, _, _, _, _, _, _) => {
                                h1624
                            }
                        };
                        let a1634 = {
                            let a1631 = {
                                let a1625 = v10;
                                let a1626 = v4;
                                let a1627 = 0u64;
                                nat_rem(a1625, a1626, a1627)
                            };
                            let a1632 = {
                                let a1629 = match v1.clone() {
                                    Adt3::C0(_, _, h1628, _, _, _, _, _) => {
                                        h1628
                                    }
                                };
                                let a1630 = v13;
                                nat_mul(a1629, a1630)?
                            };
                            nat_add(a1631, a1632)?
                        };
                        nat_mul(a1633, a1634)?
                    };
                    nat_add(a1635, a1636)?
                };
                nat_mul(a1637, a1638)?
            };
            nat_add(a1639, a1640)?
        };
        Ok(Adt5::C1(false, f49(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1646 = {
                let a1644 = {
                    let a1641 = v11;
                    let a1642 = v5;
                    let a1643 = 0u64;
                    nat_rem(a1641, a1642, a1643)
                };
                let a1645 = v4;
                nat_mul(a1644, a1645)?
            };
            let a1647 = v8;
            nat_add(a1646, a1647)?
        };
        let v16: u64 = {
            let a1653 = {
                let a1651 = {
                    let a1648 = v11;
                    let a1649 = v5;
                    let a1650 = 0u64;
                    nat_quot(a1648, a1649, a1650)
                };
                let a1652 = v4;
                nat_mul(a1651, a1652)?
            };
            let a1654 = v10;
            nat_add(a1653, a1654)?
        };
        let a1656 = v15;
        let a1657 = match v1.clone() {
            Adt3::C0(_, h1655, _, _, _, _, _, _) => {
                h1655
            }
        };
        if if nat_lt(a1656, a1657) {
            let a1659 = v16;
            let a1660 = match v1.clone() {
                Adt3::C0(_, _, h1658, _, _, _, _, _) => {
                    h1658
                }
            };
            nat_lt(a1659, a1660)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f49({
                let a1669 = v6;
                let a1670 = {
                    let a1667 = match v1.clone() {
                        Adt3::C0(h1661, _, _, _, _, _, _, _) => {
                            h1661
                        }
                    };
                    let a1668 = {
                        let a1665 = v15;
                        let a1666 = {
                            let a1663 = match v1.clone() {
                                Adt3::C0(_, h1662, _, _, _, _, _, _) => {
                                    h1662
                                }
                            };
                            let a1664 = v16;
                            nat_mul(a1663, a1664)?
                        };
                        nat_add(a1665, a1666)?
                    };
                    nat_mul(a1667, a1668)?
                };
                nat_add(a1669, a1670)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f49(v0, v2.clone())?;
    let v4: u64 = f49(v0, v1.clone())?;
    let a1675 = {
        let a1672 = v0;
        let a1673 = match v1 {
            Adt3::C0(h1671, _, _, _, _, _, _, _) => {
                h1671
            }
        };
        let a1674 = 0u64;
        nat_rem(a1672, a1673, a1674)
    };
    let a1676 = 0u64;
    if nat_eq(a1675, a1676) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f49({
            let a1677 = v0;
            let a1678 = 1u64;
            nat_sub(a1677, a1678)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1679 = v3;
    let a1680 = 13u64;
    if nat_eq(a1679, a1680) {
        f87(v0, v1, v4)
    } else {
        let v5: u64 = f74(match v1.clone() {
            Adt3::C0(h1681, _, _, _, _, _, _, _) => {
                h1681
            }
        }, {
            let a1682 = v3;
            let a1683 = 16u64;
            nat_eq(a1682, a1683)
        })?;
        let v6: u64 = {
            let a1684 = v0;
            let a1685 = v5;
            let a1686 = 0u64;
            nat_quot(a1684, a1685, a1686)
        };
        let v7: u64 = {
            let a1687 = v0;
            let a1688 = v5;
            let a1689 = 0u64;
            nat_rem(a1687, a1688, a1689)
        };
        let v8: u64 = {
            let a1693 = 1u64;
            let a1694 = {
                let a1691 = match v1.clone() {
                    Adt3::C0(h1690, _, _, _, _, _, _, _) => {
                        h1690
                    }
                };
                let a1692 = 2u64;
                nat_mul(a1691, a1692)?
            };
            nat_add(a1693, a1694)?
        };
        let a1695 = v7;
        let a1696 = v8;
        if nat_lt(a1695, a1696) {
            f88(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f49(v6, v2)?;
            let a1697 = v3;
            let a1698 = 14u64;
            if nat_eq(a1697, a1698) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1699 = v7;
                let a1700 = v8;
                if nat_eq(a1699, a1700) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1703 = v7;
                    let a1704 = {
                        let a1701 = v8;
                        let a1702 = 1u64;
                        nat_add(a1701, a1702)?
                    };
                    if nat_eq(a1703, a1704) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1705, _, _, _, _, _, _, _) => {
                                h1705
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

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1706 = v0;
    let a1707 = 0u64;
    if nat_eq(a1706, a1707) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1714 = {
            let a1708 = v0;
            let a1709 = 1u64;
            nat_add(a1708, a1709)?
        };
        let a1715 = {
            let a1712 = 2u64;
            let a1713 = {
                let a1710 = v3;
                let a1711 = 2u64;
                nat_mul(a1710, a1711)?
            };
            nat_add(a1712, a1713)?
        };
        if nat_eq(a1714, a1715) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1716 = v0;
                let a1717 = 1u64;
                nat_sub(a1716, a1717)
            };
            let v5: u64 = {
                let a1718 = v4;
                let a1719 = 2u64;
                let a1720 = 0u64;
                nat_quot(a1718, a1719, a1720)
            };
            let a1724 = {
                let a1721 = v4;
                let a1722 = 2u64;
                let a1723 = 0u64;
                nat_rem(a1721, a1722, a1723)
            };
            let a1725 = 0u64;
            if nat_eq(a1724, a1725) {
                Ok(Adt5::C4(17u64, 0u64, f49(v5, v1)?, 4u64, f49(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f63(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1733 = {
        let a1726 = 2u64;
        let a1727 = f46(v1.clone())?;
        nat_mul(a1726, a1727)?
    };
    let a1734 = {
        let a1731 = f46(v1)?;
        let a1732 = {
            let a1729 = 1u64;
            let a1730 = match v0 {
                Adt3::C0(h1728, _, _, _, _, _, _, _) => {
                    h1728
                }
            };
            nat_add(a1729, a1730)?
        };
        nat_mul(a1731, a1732)?
    };
    nat_add(a1733, a1734)
}

pub fn f64(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f49(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f65(v0: u64, v1: u64) -> bool {
    let a1735 = v1;
    let a1736 = 2147483647u64;
    if nat_le(a1735, a1736) {
        let a1737 = v1;
        let a1738 = v0;
        nat_lt(a1737, a1738)
    } else {
        false
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1739 = f33(v0);
    match m1739 {
        Adt7::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt7::C1(v7) => {
            let v8: u64 = {
                let a1741 = v2;
                let a1742 = match v7.clone() {
                    Adt8::C0(_, h1740, _) => {
                        h1740
                    }
                };
                nat_mul(a1741, a1742)?
            };
            let m1755 = f89(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1743, _, _, _, _, _, _, _) => {
                    h1743
                }
            }, match v5.clone() {
                Adt3::C0(_, h1744, _, _, _, _, _, _) => {
                    h1744
                }
            }, match v5 {
                Adt3::C0(_, _, h1745, _, _, _, _, _) => {
                    h1745
                }
            }, {
                let a1748 = match v3.clone() {
                    Adt3::C0(h1746, _, _, _, _, _, _, _) => {
                        h1746
                    }
                };
                let a1749 = match v7.clone() {
                    Adt8::C0(_, h1747, _) => {
                        h1747
                    }
                };
                nat_mul(a1748, a1749)?
            }, match v3.clone() {
                Adt3::C0(_, h1750, _, _, _, _, _, _) => {
                    h1750
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1751, _, _, _) => {
                    h1751
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1752, _, _) => {
                    h1752
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1753, _) => {
                    h1753
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1754) => {
                    h1754
                }
            })?;
            match m1755 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1756 = v0;
                    let a1757 = 0u64;
                    if if nat_eq(a1756, a1757) {
                        true
                    } else {
                        let a1758 = v0;
                        let a1759 = 26u64;
                        nat_eq(a1758, a1759)
                    } {
                        Ok(Adt5::C1(false, v9, f49({
                            let a1763 = {
                                let a1761 = v1;
                                let a1762 = match v4.clone() {
                                    Adt3::C0(h1760, _, _, _, _, _, _, _) => {
                                        h1760
                                    }
                                };
                                nat_mul(a1761, a1762)?
                            };
                            let a1764 = v8;
                            nat_add(a1763, a1764)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt8::C0(_, _, h1765) => {
                                h1765
                            }
                        }, f49({
                            let a1769 = {
                                let a1767 = v1;
                                let a1768 = match v4.clone() {
                                    Adt3::C0(h1766, _, _, _, _, _, _, _) => {
                                        h1766
                                    }
                                };
                                nat_mul(a1767, a1768)?
                            };
                            let a1770 = v8;
                            nat_add(a1769, a1770)?
                        }, v4.clone())?, {
                            let a1777 = {
                                let a1775 = {
                                    let a1772 = match v7 {
                                        Adt8::C0(_, h1771, _) => {
                                            h1771
                                        }
                                    };
                                    let a1773 = 1u64;
                                    nat_sub(a1772, a1773)
                                };
                                let a1776 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1774, _, _, _) => {
                                        h1774
                                    }
                                };
                                nat_mul(a1775, a1776)?
                            };
                            let a1778 = 4u64;
                            nat_add(a1777, a1778)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1779, _, _, _) => {
                                h1779
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f67(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f90(v0, v1, v6, v7, v8) {
        let a1780 = v5;
        let a1781 = 26u64;
        nat_eq(a1780, a1781)
    } else {
        false
    } {
        let a1782 = v3;
        let a1783 = v7;
        nat_eq(a1782, a1783)
    } else {
        false
    } {
        let a1784 = v4;
        let a1785 = v8;
        nat_eq(a1784, a1785)
    } else {
        false
    } {
        let a1786 = v9;
        let a1787 = 1u64;
        nat_eq(a1786, a1787)
    } else {
        false
    } {
        let a1788 = v10;
        let a1789 = f91(v0);
        nat_eq(a1788, a1789)
    } else {
        false
    } {
        let a1790 = v11;
        let a1791 = v1;
        nat_eq(a1790, a1791)
    } else {
        false
    } {
        let a1792 = v12;
        let a1793 = v6;
        nat_eq(a1792, a1793)
    } else {
        false
    } {
        let a1794 = v13;
        let a1795 = v7;
        nat_eq(a1794, a1795)
    } else {
        false
    } {
        let a1796 = v14;
        let a1797 = v8;
        nat_eq(a1796, a1797)
    } else {
        false
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1798 = v0;
    let a1799 = 0u64;
    if if if nat_eq(a1798, a1799) {
        true
    } else {
        let a1800 = v1;
        let a1801 = 0u64;
        nat_eq(a1800, a1801)
    } {
        true
    } else {
        let a1802 = v2;
        let a1803 = 0u64;
        nat_eq(a1802, a1803)
    } {
        true
    } else {
        let a1804 = v3;
        let a1805 = 0u64;
        nat_eq(a1804, a1805)
    }
}

pub fn f69(v0: u64) -> u64 {
    let a1806 = v0;
    let a1807 = 2147483648u64;
    if nat_lt(a1806, a1807) {
        v0
    } else {
        0u64
    }
}

pub fn f70(v0: u64) -> u64 {
    let a1808 = v0;
    let a1809 = 2147483648u64;
    if nat_lt(a1808, a1809) {
        0u64
    } else {
        let a1810 = 4294967296u64;
        let a1811 = v0;
        nat_sub(a1810, a1811)
    }
}

pub fn f71(v0: Adt3, v1: bool) -> R<u64> {
    f92(v0.clone(), {
        let a1813 = match v0.clone() {
            Adt3::C0(h1812, _, _, _, _, _, _, _) => {
                h1812
            }
        };
        let a1814 = 1u64;
        nat_sub(a1813, a1814)
    }, {
        let a1816 = match v0.clone() {
            Adt3::C0(_, h1815, _, _, _, _, _, _) => {
                h1815
            }
        };
        let a1817 = 1u64;
        nat_sub(a1816, a1817)
    }, {
        let a1819 = match v0.clone() {
            Adt3::C0(_, _, h1818, _, _, _, _, _) => {
                h1818
            }
        };
        let a1820 = 1u64;
        nat_sub(a1819, a1820)
    }, {
        let a1822 = match v0 {
            Adt3::C0(_, _, _, h1821, _, _, _, _) => {
                h1821
            }
        };
        let a1823 = 1u64;
        nat_sub(a1822, a1823)
    }, v1)
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1824 = v0;
    let a1825 = 0u64;
    if nat_eq(a1824, a1825) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1826 = v0;
            let a1827 = 1u64;
            nat_sub(a1826, a1827)
        };
        let v5: u64 = {
            let a1828 = v4;
            let a1829 = v1;
            let a1830 = 0u64;
            nat_quot(a1828, a1829, a1830)
        };
        let v6: u64 = {
            let a1838 = {
                let a1831 = v5;
                let a1832 = v2;
                nat_mul(a1831, a1832)?
            };
            let a1839 = {
                let a1836 = {
                    let a1833 = v4;
                    let a1834 = v1;
                    let a1835 = 0u64;
                    nat_rem(a1833, a1834, a1835)
                };
                let a1837 = v3;
                nat_mul(a1836, a1837)?
            };
            nat_add(a1838, a1839)?
        };
        let a1840 = v5;
        let a1841 = 0u64;
        if nat_eq(a1840, a1841) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1850 = {
                    let a1844 = {
                        let a1842 = v5;
                        let a1843 = 1u64;
                        nat_sub(a1842, a1843)
                    };
                    let a1845 = v2;
                    nat_mul(a1844, a1845)?
                };
                let a1851 = {
                    let a1848 = {
                        let a1846 = v1;
                        let a1847 = 1u64;
                        nat_sub(a1846, a1847)
                    };
                    let a1849 = v3;
                    nat_mul(a1848, a1849)?
                };
                nat_add(a1850, a1851)?
            };
            let a1852 = v6;
            let a1853 = v7;
            if nat_lt(a1852, a1853) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f73(v0: Adt3, v1: u64) -> bool {
    let a1854 = v1;
    let a1855 = 0u64;
    if !nat_eq(a1854, a1855) {
        true
    } else {
        let a1857 = match v0 {
            Adt3::C0(_, _, _, _, h1856, _, _, _) => {
                h1856
            }
        };
        let a1858 = 4u64;
        nat_eq(a1857, a1858)
    }
}

pub fn f74(v0: u64, v1: bool) -> R<u64> {
    let a1861 = {
        let a1859 = v0;
        let a1860 = 2u64;
        nat_mul(a1859, a1860)?
    };
    let a1862 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a1861, a1862)
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1863 = f93(v0, v1);
    match m1863 {
        None => {
            if v5 {
                let m1864 = f94(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1864 {
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
                let a1865 = 4u64;
                let a1866 = v28;
                nat_le(a1865, a1866)
            };
            let a1867 = v2;
            let a1868 = 0u64;
            if if if if if nat_eq(a1867, a1868) {
                let a1869 = v4;
                let a1870 = 0u64;
                nat_eq(a1869, a1870)
            } else {
                false
            } {
                let a1871 = v6;
                let a1872 = v14;
                if nat_eq(a1871, a1872) {
                    let a1873 = v7;
                    let a1874 = v15;
                    nat_eq(a1873, a1874)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1875 = v8;
                let a1876 = v16;
                if nat_eq(a1875, a1876) {
                    let a1877 = v9;
                    let a1878 = v17;
                    nat_eq(a1877, a1878)
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
                        let a1879 = v3;
                        let a1880 = 0u64;
                        nat_eq(a1879, a1880)
                    } else {
                        false
                    } {
                        f34(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1885 = {
                        let a1881 = v6;
                        let a1882 = v7;
                        nat_mul(a1881, a1882)?
                    };
                    let a1886 = {
                        let a1883 = v8;
                        let a1884 = v9;
                        nat_mul(a1883, a1884)?
                    };
                    nat_mul(a1885, a1886)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f76(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1899 = {
        let a1895 = {
            let a1891 = {
                let a1887 = v0;
                let a1888 = v4;
                nat_mul(a1887, a1888)?
            };
            let a1892 = {
                let a1889 = v1;
                let a1890 = v5;
                nat_mul(a1889, a1890)?
            };
            nat_add(a1891, a1892)?
        };
        let a1896 = {
            let a1893 = v2;
            let a1894 = v6;
            nat_mul(a1893, a1894)?
        };
        nat_add(a1895, a1896)?
    };
    let a1900 = {
        let a1897 = v3;
        let a1898 = v7;
        nat_mul(a1897, a1898)?
    };
    nat_add(a1899, a1900)
}

pub fn f77(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1905 = {
        let a1903 = {
            let a1901 = v0;
            let a1902 = v1;
            nat_sub(a1901, a1902)
        };
        let a1904 = 1u64;
        nat_sub(a1903, a1904)
    };
    let a1906 = v2;
    nat_add(a1905, a1906)
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f76({
        let a1907 = v0;
        let a1908 = v4;
        let a1909 = 0u64;
        nat_rem(a1907, a1908, a1909)
    }, {
        let a1910 = v1;
        let a1911 = v5;
        let a1912 = 0u64;
        nat_rem(a1910, a1911, a1912)
    }, {
        let a1913 = v2;
        let a1914 = v6;
        let a1915 = 0u64;
        nat_rem(a1913, a1914, a1915)
    }, {
        let a1916 = v3;
        let a1917 = v7;
        let a1918 = 0u64;
        nat_rem(a1916, a1917, a1918)
    }, v8, v9, v10, v11)
}

pub fn f79(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1920 = v0;
        let a1921 = match v1.clone() {
            Adt3::C0(h1919, _, _, _, _, _, _, _) => {
                h1919
            }
        };
        let a1922 = 0u64;
        nat_quot(a1920, a1921, a1922)
    };
    let v4: u64 = {
        let a1924 = v3;
        let a1925 = match v1.clone() {
            Adt3::C0(_, h1923, _, _, _, _, _, _) => {
                h1923
            }
        };
        let a1926 = 0u64;
        nat_quot(a1924, a1925, a1926)
    };
    let a1927 = v2;
    let a1928 = 0u64;
    if nat_eq(a1927, a1928) {
        let a1930 = v0;
        let a1931 = match v1 {
            Adt3::C0(h1929, _, _, _, _, _, _, _) => {
                h1929
            }
        };
        let a1932 = 0u64;
        nat_rem(a1930, a1931, a1932)
    } else {
        let a1933 = v2;
        let a1934 = 1u64;
        if nat_eq(a1933, a1934) {
            let a1936 = v3;
            let a1937 = match v1 {
                Adt3::C0(_, h1935, _, _, _, _, _, _) => {
                    h1935
                }
            };
            let a1938 = 0u64;
            nat_rem(a1936, a1937, a1938)
        } else {
            let a1939 = v2;
            let a1940 = 2u64;
            if nat_eq(a1939, a1940) {
                let a1942 = v4;
                let a1943 = match v1 {
                    Adt3::C0(_, _, h1941, _, _, _, _, _) => {
                        h1941
                    }
                };
                let a1944 = 0u64;
                nat_rem(a1942, a1943, a1944)
            } else {
                let a1946 = v4;
                let a1947 = match v1 {
                    Adt3::C0(_, _, h1945, _, _, _, _, _) => {
                        h1945
                    }
                };
                let a1948 = 0u64;
                nat_quot(a1946, a1947, a1948)
            }
        }
    }
}

pub fn f80(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f76(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1949, _, _, _) => {
            h1949
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1950, _, _) => {
            h1950
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1951, _) => {
            h1951
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1952) => {
            h1952
        }
    })
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1953 = v1;
    let a1954 = 2147483648u64;
    if nat_lt(a1953, a1954) {
        let a1965 = {
            let a1963 = {
                let a1958 = {
                    let a1955 = v0;
                    let a1956 = v2;
                    let a1957 = 0u64;
                    nat_rem(a1955, a1956, a1957)
                };
                let a1959 = v2;
                nat_add(a1958, a1959)?
            };
            let a1964 = {
                let a1960 = v1;
                let a1961 = v2;
                let a1962 = 0u64;
                nat_rem(a1960, a1961, a1962)
            };
            nat_sub(a1963, a1964)
        };
        let a1966 = v2;
        let a1967 = 0u64;
        Ok(nat_rem(a1965, a1966, a1967))
    } else {
        let a1970 = {
            let a1968 = v0;
            let a1969 = f70(v1);
            nat_add(a1968, a1969)?
        };
        let a1971 = v2;
        let a1972 = 0u64;
        Ok(nat_rem(a1970, a1971, a1972))
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1975 = f69(v1);
    let a1976 = {
        let a1973 = v0;
        let a1974 = f70(v1);
        nat_add(a1973, a1974)?
    };
    if nat_le(a1975, a1976) {
        let a1977 = f83(v0, v1)?;
        let a1978 = v2;
        Ok(nat_lt(a1977, a1978))
    } else {
        Ok(false)
    }
}

pub fn f83(v0: u64, v1: u64) -> R<u64> {
    let a1981 = {
        let a1979 = v0;
        let a1980 = f70(v1);
        nat_add(a1979, a1980)?
    };
    let a1982 = f69(v1);
    Ok(nat_sub(a1981, a1982))
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1983 = v0;
    let a1984 = v1;
    if nat_lt(a1983, a1984) {
        let a1985 = v1;
        let a1986 = v0;
        Ok(nat_sub(a1985, a1986))
    } else {
        let a1989 = v0;
        let a1990 = {
            let a1987 = v1;
            let a1988 = v2;
            nat_add(a1987, a1988)?
        };
        if nat_lt(a1989, a1990) {
            let a1991 = v0;
            let a1992 = v1;
            Ok(nat_sub(a1991, a1992))
        } else {
            let a1999 = {
                let a1995 = 2u64;
                let a1996 = {
                    let a1993 = v2;
                    let a1994 = 1u64;
                    nat_sub(a1993, a1994)
                };
                nat_mul(a1995, a1996)?
            };
            let a2000 = {
                let a1997 = v0;
                let a1998 = v1;
                nat_sub(a1997, a1998)
            };
            Ok(nat_sub(a1999, a2000))
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2003 = {
        let a2001 = v2;
        let a2002 = f92(v1.clone(), f79(v0, v1.clone(), 0u64), f79(v0, v1.clone(), 1u64), f79(v0, v1.clone(), 2u64), f79(v0, v1.clone(), 3u64), false)?;
        nat_add(a2001, a2002)?
    };
    let a2004 = f92(v1.clone(), f79(v0, v1.clone(), 0u64), f79(v0, v1.clone(), 1u64), f79(v0, v1.clone(), 2u64), f79(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2003, a2004))
}

pub fn f86(v0: u64, v1: u64, v2: u64) -> bool {
    let a2005 = v0;
    let a2006 = 0u64;
    if nat_eq(a2005, a2006) {
        let a2007 = v2;
        let a2008 = v1;
        nat_le(a2007, a2008)
    } else {
        let a2009 = v0;
        let a2010 = 1u64;
        if nat_eq(a2009, a2010) {
            let a2011 = v2;
            let a2012 = v1;
            nat_lt(a2011, a2012)
        } else {
            let a2013 = v0;
            let a2014 = 2u64;
            if nat_eq(a2013, a2014) {
                let a2015 = v1;
                let a2016 = v2;
                nat_le(a2015, a2016)
            } else {
                let a2017 = v0;
                let a2018 = 3u64;
                if nat_eq(a2017, a2018) {
                    let a2019 = v1;
                    let a2020 = v2;
                    nat_lt(a2019, a2020)
                } else {
                    let a2021 = v1;
                    let a2022 = v2;
                    nat_eq(a2021, a2022)
                }
            }
        }
    }
}

pub fn f87(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2023 = v0;
    let a2024 = 0u64;
    if nat_eq(a2023, a2024) {
        Ok(f95(0u64))
    } else {
        let a2027 = {
            let a2025 = v0;
            let a2026 = 1u64;
            nat_add(a2025, a2026)?
        };
        let a2028 = f47(v1.clone(), 13u64, v2)?;
        if nat_eq(a2027, a2028) {
            Ok(Adt5::C4({
                let a2029 = v2;
                let a2030 = 0u64;
                if nat_eq(a2029, a2030) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2031 = v0;
                let a2032 = 1u64;
                nat_sub(a2031, a2032)
            };
            if f73(v1.clone(), v2) {
                let v4: u64 = f74(match v1.clone() {
                    Adt3::C0(h2033, _, _, _, _, _, _, _) => {
                        h2033
                    }
                }, false)?;
                let v5: u64 = {
                    let a2034 = v3;
                    let a2035 = v4;
                    let a2036 = 0u64;
                    nat_quot(a2034, a2035, a2036)
                };
                let v6: u64 = {
                    let a2037 = v3;
                    let a2038 = v4;
                    let a2039 = 0u64;
                    nat_rem(a2037, a2038, a2039)
                };
                let a2042 = {
                    let a2040 = v6;
                    let a2041 = 1u64;
                    nat_add(a2040, a2041)?
                };
                let a2043 = v4;
                if nat_eq(a2042, a2043) {
                    Ok(Adt5::C4({
                        let a2044 = v2;
                        let a2045 = 0u64;
                        if nat_eq(a2044, a2045) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f88(v6, v5, v1, v2)
                }
            } else {
                f96({
                    let a2046 = v3;
                    let a2047 = 2u64;
                    let a2048 = 0u64;
                    nat_quot(a2046, a2047, a2048)
                }, {
                    let a2049 = v3;
                    let a2050 = 2u64;
                    let a2051 = 0u64;
                    nat_rem(a2049, a2050, a2051)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2052 = v0;
    let a2053 = 0u64;
    if nat_eq(a2052, a2053) {
        Ok(f95(1u64))
    } else {
        f96({
            let a2062 = {
                let a2055 = v1;
                let a2056 = match v2.clone() {
                    Adt3::C0(h2054, _, _, _, _, _, _, _) => {
                        h2054
                    }
                };
                nat_mul(a2055, a2056)?
            };
            let a2063 = {
                let a2059 = {
                    let a2057 = v0;
                    let a2058 = 1u64;
                    nat_sub(a2057, a2058)
                };
                let a2060 = 2u64;
                let a2061 = 0u64;
                nat_quot(a2059, a2060, a2061)
            };
            nat_add(a2062, a2063)?
        }, {
            let a2066 = {
                let a2064 = v0;
                let a2065 = 1u64;
                nat_sub(a2064, a2065)
            };
            let a2067 = 2u64;
            let a2068 = 0u64;
            nat_rem(a2066, a2067, a2068)
        }, v2, v3, 1u64)
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2069 = f33(v0);
    match m2069 {
        Adt7::C0 => {
            Ok(None::<u64>)
        }
        Adt7::C1(v13) => {
            if if if if if f97(v0) {
                let a2070 = v1;
                let a2071 = v7;
                nat_lt(a2070, a2071)
            } else {
                false
            } {
                f65(v8, v2)
            } else {
                false
            } {
                let a2072 = v3;
                let a2073 = f98(v4, v5, v6)?;
                nat_lt(a2072, a2073)
            } else {
                false
            } {
                let a2078 = {
                    let a2075 = v7;
                    let a2076 = match v13.clone() {
                        Adt8::C0(_, h2074, _) => {
                            h2074
                        }
                    };
                    let a2077 = 0u64;
                    nat_rem(a2075, a2076, a2077)
                };
                let a2079 = 0u64;
                nat_eq(a2078, a2079)
            } else {
                false
            } {
                Ok(Some({
                    let a2108 = {
                        let a2098 = {
                            let a2088 = {
                                let a2084 = {
                                    let a2081 = v1;
                                    let a2082 = match v13 {
                                        Adt8::C0(_, h2080, _) => {
                                            h2080
                                        }
                                    };
                                    let a2083 = 0u64;
                                    nat_quot(a2081, a2082, a2083)
                                };
                                let a2085 = v9;
                                nat_mul(a2084, a2085)?
                            };
                            let a2089 = {
                                let a2086 = v2;
                                let a2087 = v10;
                                nat_mul(a2086, a2087)?
                            };
                            nat_add(a2088, a2089)?
                        };
                        let a2099 = {
                            let a2096 = {
                                let a2093 = {
                                    let a2090 = v3;
                                    let a2091 = v4;
                                    let a2092 = 0u64;
                                    nat_quot(a2090, a2091, a2092)
                                };
                                let a2094 = v5;
                                let a2095 = 0u64;
                                nat_rem(a2093, a2094, a2095)
                            };
                            let a2097 = v11;
                            nat_mul(a2096, a2097)?
                        };
                        nat_add(a2098, a2099)?
                    };
                    let a2109 = {
                        let a2106 = {
                            let a2103 = {
                                let a2100 = v3;
                                let a2101 = v4;
                                let a2102 = 0u64;
                                nat_quot(a2100, a2101, a2102)
                            };
                            let a2104 = v5;
                            let a2105 = 0u64;
                            nat_quot(a2103, a2104, a2105)
                        };
                        let a2107 = v12;
                        nat_mul(a2106, a2107)?
                    };
                    nat_add(a2108, a2109)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f90(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f68(v1, v2, v3, v4) {
        f99(v0)
    } else {
        f97(v0)
    }
}

pub fn f91(v0: u64) -> u64 {
    let a2110 = v0;
    let a2111 = 26u64;
    if nat_eq(a2110, a2111) {
        26u64
    } else {
        0u64
    }
}

pub fn f92(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2132 = {
        let a2126 = {
            let a2120 = {
                let a2114 = v1;
                let a2115 = if v5 {
                    f70(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2112, _, _, _) => {
                            h2112
                        }
                    })
                } else {
                    f69(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2113, _, _, _) => {
                            h2113
                        }
                    })
                };
                nat_mul(a2114, a2115)?
            };
            let a2121 = {
                let a2118 = v2;
                let a2119 = if v5 {
                    f70(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2116, _, _) => {
                            h2116
                        }
                    })
                } else {
                    f69(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2117, _, _) => {
                            h2117
                        }
                    })
                };
                nat_mul(a2118, a2119)?
            };
            nat_add(a2120, a2121)?
        };
        let a2127 = {
            let a2124 = v3;
            let a2125 = if v5 {
                f70(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2122, _) => {
                        h2122
                    }
                })
            } else {
                f69(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2123, _) => {
                        h2123
                    }
                })
            };
            nat_mul(a2124, a2125)?
        };
        nat_add(a2126, a2127)?
    };
    let a2133 = {
        let a2130 = v4;
        let a2131 = if v5 {
            f70(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2128) => {
                    h2128
                }
            })
        } else {
            f69(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2129) => {
                    h2129
                }
            })
        };
        nat_mul(a2130, a2131)?
    };
    nat_add(a2132, a2133)
}

pub fn f93(v0: u64, v1: u64) -> Option<u64> {
    let a2134 = v0;
    let a2135 = 2u64;
    if nat_eq(a2134, a2135) {
        Some(0u64)
    } else {
        let a2136 = v0;
        let a2137 = 5u64;
        if nat_eq(a2136, a2137) {
            Some(1u64)
        } else {
            let a2138 = v0;
            let a2139 = 6u64;
            if nat_eq(a2138, a2139) {
                Some(2u64)
            } else {
                let a2140 = v0;
                let a2141 = 7u64;
                if nat_eq(a2140, a2141) {
                    Some(3u64)
                } else {
                    let a2142 = v0;
                    let a2143 = 9u64;
                    if nat_eq(a2142, a2143) {
                        Some(4u64)
                    } else {
                        let a2144 = v0;
                        let a2145 = 8u64;
                        if nat_eq(a2144, a2145) {
                            Some(5u64)
                        } else {
                            let a2146 = v0;
                            let a2147 = 89u64;
                            if nat_eq(a2146, a2147) {
                                let a2148 = v1;
                                let a2149 = 2u64;
                                if nat_eq(a2148, a2149) {
                                    Some(6u64)
                                } else {
                                    let a2150 = v1;
                                    let a2151 = 0u64;
                                    if nat_eq(a2150, a2151) {
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

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a2152 = v0;
    let a2153 = 28u64;
    if if if if nat_eq(a2152, a2153) {
        let a2154 = v1;
        let a2155 = 0u64;
        nat_eq(a2154, a2155)
    } else {
        false
    } {
        let a2156 = v2;
        let a2157 = 0u64;
        nat_eq(a2156, a2157)
    } else {
        false
    } {
        let a2158 = v3;
        let a2159 = 0u64;
        nat_eq(a2158, a2159)
    } else {
        false
    } {
        let a2160 = v4;
        let a2161 = v8;
        if if if if if if if if nat_eq(a2160, a2161) {
            let a2162 = v5;
            let a2163 = v12;
            nat_eq(a2162, a2163)
        } else {
            false
        } {
            let a2164 = v9;
            let a2165 = v13;
            nat_eq(a2164, a2165)
        } else {
            false
        } {
            let a2166 = v10;
            let a2167 = v14;
            nat_eq(a2166, a2167)
        } else {
            false
        } {
            let a2168 = v11;
            let a2169 = v15;
            nat_eq(a2168, a2169)
        } else {
            false
        } {
            let a2170 = 0u64;
            let a2171 = v6;
            nat_lt(a2170, a2171)
        } else {
            false
        } {
            let a2172 = 0u64;
            let a2173 = v7;
            nat_lt(a2172, a2173)
        } else {
            false
        } {
            let a2177 = {
                let a2174 = v10;
                let a2175 = v6;
                let a2176 = 1u64;
                nat_rem(a2174, a2175, a2176)
            };
            let a2178 = 0u64;
            if nat_eq(a2177, a2178) {
                let a2182 = {
                    let a2179 = v11;
                    let a2180 = v7;
                    let a2181 = 1u64;
                    nat_rem(a2179, a2180, a2181)
                };
                let a2183 = 0u64;
                nat_eq(a2182, a2183)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a2184 = v10;
                let a2185 = v6;
                let a2186 = 0u64;
                nat_quot(a2184, a2185, a2186)
            }, {
                let a2187 = v11;
                let a2188 = v7;
                let a2189 = 0u64;
                nat_quot(a2187, a2188, a2189)
            }, {
                let a2190 = v5;
                let a2191 = v4;
                nat_mul(a2190, a2191)?
            }, {
                let a2192 = v4;
                let a2193 = v9;
                nat_mul(a2192, a2193)?
            }, {
                let a2194 = v5;
                let a2195 = v9;
                nat_mul(a2194, a2195)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f95(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f96(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2196 = v1;
    let a2197 = 0u64;
    if nat_eq(a2196, a2197) {
        Ok(Adt5::C4({
            let a2198 = v3;
            let a2199 = 0u64;
            if nat_eq(a2198, a2199) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f49(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2200 = v3;
            let a2201 = 0u64;
            if nat_eq(a2200, a2201) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f97(v0: u64) -> bool {
    let a2202 = v0;
    let a2203 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2202, a2203) {
        true
    } else {
        let a2204 = v0;
        let a2205 = 1u64;
        nat_eq(a2204, a2205)
    } {
        true
    } else {
        let a2206 = v0;
        let a2207 = 2u64;
        nat_eq(a2206, a2207)
    } {
        true
    } else {
        let a2208 = v0;
        let a2209 = 3u64;
        nat_eq(a2208, a2209)
    } {
        true
    } else {
        let a2210 = v0;
        let a2211 = 6u64;
        nat_eq(a2210, a2211)
    } {
        true
    } else {
        let a2212 = v0;
        let a2213 = 7u64;
        nat_eq(a2212, a2213)
    } {
        true
    } else {
        let a2214 = v0;
        let a2215 = 8u64;
        nat_eq(a2214, a2215)
    } {
        true
    } else {
        let a2216 = v0;
        let a2217 = 10u64;
        nat_eq(a2216, a2217)
    } {
        true
    } else {
        let a2218 = v0;
        let a2219 = 11u64;
        nat_eq(a2218, a2219)
    } {
        true
    } else {
        let a2220 = v0;
        let a2221 = 12u64;
        nat_eq(a2220, a2221)
    } {
        true
    } else {
        let a2222 = v0;
        let a2223 = 13u64;
        nat_eq(a2222, a2223)
    } {
        true
    } else {
        let a2224 = v0;
        let a2225 = 14u64;
        nat_eq(a2224, a2225)
    } {
        true
    } else {
        let a2226 = v0;
        let a2227 = 16u64;
        nat_eq(a2226, a2227)
    } {
        true
    } else {
        let a2228 = v0;
        let a2229 = 17u64;
        nat_eq(a2228, a2229)
    } {
        true
    } else {
        let a2230 = v0;
        let a2231 = 18u64;
        nat_eq(a2230, a2231)
    } {
        true
    } else {
        let a2232 = v0;
        let a2233 = 19u64;
        nat_eq(a2232, a2233)
    } {
        true
    } else {
        let a2234 = v0;
        let a2235 = 20u64;
        nat_eq(a2234, a2235)
    } {
        true
    } else {
        let a2236 = v0;
        let a2237 = 21u64;
        nat_eq(a2236, a2237)
    } {
        true
    } else {
        let a2238 = v0;
        let a2239 = 22u64;
        nat_eq(a2238, a2239)
    } {
        true
    } else {
        let a2240 = v0;
        let a2241 = 23u64;
        nat_eq(a2240, a2241)
    } {
        true
    } else {
        let a2242 = v0;
        let a2243 = 26u64;
        nat_eq(a2242, a2243)
    } {
        true
    } else {
        let a2244 = v0;
        let a2245 = 29u64;
        nat_eq(a2244, a2245)
    } {
        true
    } else {
        let a2246 = v0;
        let a2247 = 30u64;
        nat_eq(a2246, a2247)
    } {
        true
    } else {
        let a2248 = v0;
        let a2249 = 34u64;
        nat_eq(a2248, a2249)
    } {
        true
    } else {
        let a2250 = v0;
        let a2251 = 35u64;
        nat_eq(a2250, a2251)
    } {
        true
    } else {
        let a2252 = v0;
        let a2253 = 39u64;
        nat_eq(a2252, a2253)
    } {
        true
    } else {
        let a2254 = v0;
        let a2255 = 40u64;
        nat_eq(a2254, a2255)
    } {
        true
    } else {
        let a2256 = v0;
        let a2257 = 41u64;
        nat_eq(a2256, a2257)
    } {
        true
    } else {
        let a2258 = v0;
        let a2259 = 42u64;
        nat_eq(a2258, a2259)
    } {
        true
    } else {
        let a2260 = v0;
        let a2261 = 43u64;
        nat_eq(a2260, a2261)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2262 = v0;
    let a2263 = 0u64;
    if if if nat_eq(a2262, a2263) {
        true
    } else {
        let a2264 = v1;
        let a2265 = 0u64;
        nat_eq(a2264, a2265)
    } {
        true
    } else {
        let a2266 = v2;
        let a2267 = 0u64;
        nat_eq(a2266, a2267)
    } {
        Ok(0u64)
    } else {
        let a2270 = {
            let a2268 = v0;
            let a2269 = v1;
            nat_mul(a2268, a2269)?
        };
        let a2271 = v2;
        nat_mul(a2270, a2271)
    }
}

pub fn f99(v0: u64) -> bool {
    let m2272 = f33(v0);
    match m2272 {
        Adt7::C0 => {
            false
        }
        Adt7::C1(_) => {
            true
        }
    }
}
