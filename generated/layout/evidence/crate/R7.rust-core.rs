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
    C0,
    C1(Adt1),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(u64, u64, u64),
}

pub fn f0(v0: u64) -> Adt0 {
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        Adt0::C1(Adt1::C0(0u64, 1u64, 4u64))
    } else {
        let a3 = v0;
        let a4 = 1u64;
        if nat_eq(a3, a4) {
            Adt0::C1(Adt1::C0(1u64, 1u64, 2u64))
        } else {
            let a5 = v0;
            let a6 = 2u64;
            if nat_eq(a5, a6) {
                Adt0::C1(Adt1::C0(2u64, 32u64, 18u64))
            } else {
                let a7 = v0;
                let a8 = 3u64;
                if nat_eq(a7, a8) {
                    Adt0::C1(Adt1::C0(3u64, 32u64, 20u64))
                } else {
                    let a9 = v0;
                    let a10 = 6u64;
                    if nat_eq(a9, a10) {
                        Adt0::C1(Adt1::C0(6u64, 32u64, 22u64))
                    } else {
                        let a11 = v0;
                        let a12 = 7u64;
                        if nat_eq(a11, a12) {
                            Adt0::C1(Adt1::C0(7u64, 32u64, 24u64))
                        } else {
                            let a13 = v0;
                            let a14 = 8u64;
                            if nat_eq(a13, a14) {
                                Adt0::C1(Adt1::C0(8u64, 32u64, 34u64))
                            } else {
                                let a15 = v0;
                                let a16 = 9u64;
                                if nat_eq(a15, a16) {
                                    Adt0::C1(Adt1::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a17 = v0;
                                    let a18 = 10u64;
                                    if nat_eq(a17, a18) {
                                        Adt0::C1(Adt1::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a19 = v0;
                                        let a20 = 11u64;
                                        if nat_eq(a19, a20) {
                                            Adt0::C1(Adt1::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a21 = v0;
                                            let a22 = 12u64;
                                            if nat_eq(a21, a22) {
                                                Adt0::C1(Adt1::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a23 = v0;
                                                let a24 = 13u64;
                                                if nat_eq(a23, a24) {
                                                    Adt0::C1(Adt1::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a25 = v0;
                                                    let a26 = 14u64;
                                                    if nat_eq(a25, a26) {
                                                        Adt0::C1(Adt1::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a27 = v0;
                                                        let a28 = 15u64;
                                                        if nat_eq(a27, a28) {
                                                            Adt0::C1(Adt1::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a29 = v0;
                                                            let a30 = 16u64;
                                                            if nat_eq(a29, a30) {
                                                                Adt0::C1(Adt1::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a31 = v0;
                                                                let a32 = 17u64;
                                                                if nat_eq(a31, a32) {
                                                                    Adt0::C1(Adt1::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a33 = v0;
                                                                    let a34 = 18u64;
                                                                    if nat_eq(a33, a34) {
                                                                        Adt0::C1(Adt1::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a35 = v0;
                                                                        let a36 = 19u64;
                                                                        if nat_eq(a35, a36) {
                                                                            Adt0::C1(Adt1::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a37 = v0;
                                                                            let a38 = 20u64;
                                                                            if nat_eq(a37, a38) {
                                                                                Adt0::C1(Adt1::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a39 = v0;
                                                                                let a40 = 21u64;
                                                                                if nat_eq(a39, a40) {
                                                                                    Adt0::C1(Adt1::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a41 = v0;
                                                                                    let a42 = 22u64;
                                                                                    if nat_eq(a41, a42) {
                                                                                        Adt0::C1(Adt1::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a43 = v0;
                                                                                        let a44 = 23u64;
                                                                                        if nat_eq(a43, a44) {
                                                                                            Adt0::C1(Adt1::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a45 = v0;
                                                                                            let a46 = 24u64;
                                                                                            if nat_eq(a45, a46) {
                                                                                                Adt0::C1(Adt1::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a47 = v0;
                                                                                                let a48 = 25u64;
                                                                                                if nat_eq(a47, a48) {
                                                                                                    Adt0::C1(Adt1::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a49 = v0;
                                                                                                    let a50 = 26u64;
                                                                                                    if nat_eq(a49, a50) {
                                                                                                        Adt0::C1(Adt1::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a51 = v0;
                                                                                                        let a52 = 27u64;
                                                                                                        if nat_eq(a51, a52) {
                                                                                                            Adt0::C1(Adt1::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a53 = v0;
                                                                                                            let a54 = 28u64;
                                                                                                            if nat_eq(a53, a54) {
                                                                                                                Adt0::C1(Adt1::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a55 = v0;
                                                                                                                let a56 = 29u64;
                                                                                                                if nat_eq(a55, a56) {
                                                                                                                    Adt0::C1(Adt1::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a57 = v0;
                                                                                                                    let a58 = 30u64;
                                                                                                                    if nat_eq(a57, a58) {
                                                                                                                        Adt0::C1(Adt1::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a59 = v0;
                                                                                                                        let a60 = 34u64;
                                                                                                                        if nat_eq(a59, a60) {
                                                                                                                            Adt0::C1(Adt1::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a61 = v0;
                                                                                                                            let a62 = 35u64;
                                                                                                                            if nat_eq(a61, a62) {
                                                                                                                                Adt0::C1(Adt1::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a63 = v0;
                                                                                                                                let a64 = 39u64;
                                                                                                                                if nat_eq(a63, a64) {
                                                                                                                                    Adt0::C1(Adt1::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a65 = v0;
                                                                                                                                    let a66 = 40u64;
                                                                                                                                    if nat_eq(a65, a66) {
                                                                                                                                        Adt0::C1(Adt1::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a67 = v0;
                                                                                                                                        let a68 = 41u64;
                                                                                                                                        if nat_eq(a67, a68) {
                                                                                                                                            Adt0::C1(Adt1::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a69 = v0;
                                                                                                                                            let a70 = 42u64;
                                                                                                                                            if nat_eq(a69, a70) {
                                                                                                                                                Adt0::C1(Adt1::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a71 = v0;
                                                                                                                                                let a72 = 43u64;
                                                                                                                                                if nat_eq(a71, a72) {
                                                                                                                                                    Adt0::C1(Adt1::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt0::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
