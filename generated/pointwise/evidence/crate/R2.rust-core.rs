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
                    let a6 = 40u64;
                    if nat_eq(a5, a6) {
                        f2(v3, v4, v5)?
                    } else {
                        if f3(v1) {
                            f4(v1, v3, {
                                let m7 = v4;
                                match m7 {
                                    None => {
                                        false
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v5)?
                        } else {
                            let a8 = v1;
                            let a9 = 19u64;
                            if nat_eq(a8, a9) {
                                f5(v3, v5)?
                            } else {
                                let a10 = v1;
                                let a11 = 21u64;
                                if nat_eq(a10, a11) {
                                    f6(v6, v3, v4, v5)?
                                } else {
                                    let a12 = v1;
                                    let a13 = 61u64;
                                    if if if nat_eq(a12, a13) {
                                        true
                                    } else {
                                        let a14 = v1;
                                        let a15 = 62u64;
                                        nat_eq(a14, a15)
                                    } {
                                        true
                                    } else {
                                        let a16 = v1;
                                        let a17 = 63u64;
                                        nat_eq(a16, a17)
                                    } {
                                        f7(v1, v2, v3, v5)?
                                    } else {
                                        let a18 = v1;
                                        let a19 = 4u64;
                                        if if nat_eq(a18, a19) {
                                            true
                                        } else {
                                            let a20 = v1;
                                            let a21 = 32u64;
                                            nat_eq(a20, a21)
                                        } {
                                            f8(v1, v2, v3, v4, v5)?
                                        } else {
                                            let a22 = v1;
                                            let a23 = 20u64;
                                            if nat_eq(a22, a23) {
                                                f9(v3, v5)?
                                            } else {
                                                let a24 = v1;
                                                let a25 = 42u64;
                                                if nat_eq(a24, a25) {
                                                    f10(4u64, v3, v5)?
                                                } else {
                                                    let a26 = v1;
                                                    let a27 = 69u64;
                                                    if nat_eq(a26, a27) {
                                                        let a28 = v6;
                                                        let a29 = 4u64;
                                                        if nat_lt(a28, a29) {
                                                            f10(v6, v3, v5)?
                                                        } else {
                                                            Adt6::C1
                                                        }
                                                    } else {
                                                        let a30 = v1;
                                                        let a31 = 70u64;
                                                        if nat_eq(a30, a31) {
                                                            f11(v6, v3, v5)?
                                                        } else {
                                                            let a32 = v1;
                                                            let a33 = 75u64;
                                                            if if nat_eq(a32, a33) {
                                                                true
                                                            } else {
                                                                let a34 = v1;
                                                                let a35 = 76u64;
                                                                nat_eq(a34, a35)
                                                            } {
                                                                f12({
                                                                    let a36 = v1;
                                                                    let a37 = 76u64;
                                                                    nat_eq(a36, a37)
                                                                }, v2, v3, v5)?
                                                            } else {
                                                                let a38 = v1;
                                                                let a39 = 31u64;
                                                                if if if nat_eq(a38, a39) {
                                                                    true
                                                                } else {
                                                                    let a40 = v1;
                                                                    let a41 = 47u64;
                                                                    nat_eq(a40, a41)
                                                                } {
                                                                    true
                                                                } else {
                                                                    let a42 = v1;
                                                                    let a43 = 68u64;
                                                                    nat_eq(a42, a43)
                                                                } {
                                                                    f13(v1, v2, v3, v5)?
                                                                } else {
                                                                    let a44 = v1;
                                                                    let a45 = 15u64;
                                                                    if nat_eq(a44, a45) {
                                                                        f14(v3, v5)?
                                                                    } else {
                                                                        let a46 = v1;
                                                                        let a47 = 17u64;
                                                                        if nat_eq(a46, a47) {
                                                                            f15(v3, v5)?
                                                                        } else {
                                                                            let a48 = v1;
                                                                            let a49 = 18u64;
                                                                            if nat_eq(a48, a49) {
                                                                                f16(v3, v4, v5)?
                                                                            } else {
                                                                                let a50 = v1;
                                                                                let a51 = 13u64;
                                                                                if if if nat_eq(a50, a51) {
                                                                                    true
                                                                                } else {
                                                                                    let a52 = v1;
                                                                                    let a53 = 14u64;
                                                                                    nat_eq(a52, a53)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a54 = v1;
                                                                                    let a55 = 16u64;
                                                                                    nat_eq(a54, a55)
                                                                                } {
                                                                                    f17(v1, v3, v5)?
                                                                                } else {
                                                                                    let m56 = f18(v1, v6);
                                                                                    match m56 {
                                                                                        None => {
                                                                                            f19(v1, v6, v3, v4, v5)?
                                                                                        }
                                                                                        Some(v8) => {
                                                                                            f20(v8, v3, v5)?
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
            f21(v9, v10, v11, v12)
        }
        Adt0::C2(v13, v14, v15, v16) => {
            f22(v13, v14, v15, v16)
        }
        Adt0::C3(v17, v18, v19, v20, v21) => {
            f23(v17, v18, v19, v20, v21)
        }
        Adt0::C4(v22, v23, v24, v25, v26, v27) => {
            f24(v22, v23, v24, v25, v26, v27)
        }
    }
}

pub fn f1(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m57 = v1;
    match m57 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f25(v0, v3, v2)
        }
    }
}

pub fn f2(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m58 = v1;
    match m58 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a60 = match v0.clone() {
                Adt2::C0(h59, _, _, _, _, _, _, _, _, _, _) => {
                    h59
                }
            };
            let a61 = 0u64;
            if if if if if if if if if if if if if nat_eq(a60, a61) {
                true
            } else {
                let a63 = match v0.clone() {
                    Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                        h62
                    }
                };
                let a64 = 1u64;
                nat_eq(a63, a64)
            } {
                let a66 = match v3.clone() {
                    Adt2::C0(h65, _, _, _, _, _, _, _, _, _, _) => {
                        h65
                    }
                };
                let a67 = 26u64;
                nat_eq(a66, a67)
            } else {
                false
            } {
                let a69 = match v2.clone() {
                    Adt2::C0(h68, _, _, _, _, _, _, _, _, _, _) => {
                        h68
                    }
                };
                let a70 = 0u64;
                nat_eq(a69, a70)
            } else {
                false
            } {
                let a73 = match v0.clone() {
                    Adt2::C0(_, h71, _, _, _, _, _, _, _, _, _) => {
                        h71
                    }
                };
                let a74 = match v2.clone() {
                    Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                        h72
                    }
                };
                nat_eq(a73, a74)
            } else {
                false
            } {
                let a77 = match v3.clone() {
                    Adt2::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                        h75
                    }
                };
                let a78 = match v0.clone() {
                    Adt2::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                        h76
                    }
                };
                nat_le(a77, a78)
            } else {
                false
            } {
                let a80 = match v0.clone() {
                    Adt2::C0(_, _, _, h79, _, _, _, _, _, _, _) => {
                        h79
                    }
                };
                let a81 = 1u64;
                nat_eq(a80, a81)
            } else {
                false
            } {
                let a83 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h82, _, _, _, _, _, _) => {
                        h82
                    }
                };
                let a84 = 1u64;
                nat_eq(a83, a84)
            } else {
                false
            } {
                let a86 = match v2.clone() {
                    Adt2::C0(_, _, _, h85, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                let a87 = 1u64;
                nat_eq(a86, a87)
            } else {
                false
            } {
                let a89 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h88, _, _, _, _, _, _) => {
                        h88
                    }
                };
                let a90 = 1u64;
                nat_eq(a89, a90)
            } else {
                false
            } {
                let a92 = match v3.clone() {
                    Adt2::C0(_, _, h91, _, _, _, _, _, _, _, _) => {
                        h91
                    }
                };
                let a93 = 1u64;
                nat_eq(a92, a93)
            } else {
                false
            } {
                let a95 = match v3.clone() {
                    Adt2::C0(_, _, _, h94, _, _, _, _, _, _, _) => {
                        h94
                    }
                };
                let a96 = 1u64;
                nat_eq(a95, a96)
            } else {
                false
            } {
                let a98 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h97, _, _, _, _, _, _) => {
                        h97
                    }
                };
                let a99 = 1u64;
                nat_eq(a98, a99)
            } else {
                false
            } {
                if f26(match v2.clone() {
                    Adt2::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                        h100
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h102, _, _, _, _, _, _, _) => {
                        h102
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                        h103
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a105 = match v3.clone() {
                        Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                            h104
                        }
                    };
                    let a106 = 0u64;
                    if nat_eq(a105, a106) {
                        Ok(Adt6::C4(f28(v2.clone(), 1u64)?, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a108 = match v0.clone() {
                            Adt2::C0(_, h107, _, _, _, _, _, _, _, _, _) => {
                                h107
                            }
                        };
                        let a109 = 2147483647u64;
                        if if if if if if nat_le(a108, a109) {
                            let a111 = match v3.clone() {
                                Adt2::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                                    h110
                                }
                            };
                            let a112 = 2147483647u64;
                            nat_le(a111, a112)
                        } else {
                            false
                        } {
                            let a114 = 4u64;
                            let a115 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h113, _, _, _, _, _) => {
                                    h113
                                }
                            };
                            nat_le(a114, a115)
                        } else {
                            false
                        } {
                            let a121 = {
                                let a118 = match v2.clone() {
                                    Adt2::C0(_, h116, _, _, _, _, _, _, _, _, _) => {
                                        h116
                                    }
                                };
                                let a119 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h117, _, _, _, _, _) => {
                                        h117
                                    }
                                };
                                nat_mul(a118, a119)?
                            };
                            let a122 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h120, _, _, _, _) => {
                                    h120
                                }
                            };
                            nat_le(a121, a122)
                        } else {
                            false
                        } {
                            f29(v0.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            f29(v3.clone(), v2.clone())?
                        } else {
                            false
                        } {
                            Ok(Adt6::C4({
                                let a136 = {
                                    let a126 = {
                                        let a124 = 2u64;
                                        let a125 = match v3.clone() {
                                            Adt2::C0(_, h123, _, _, _, _, _, _, _, _, _) => {
                                                h123
                                            }
                                        };
                                        nat_mul(a124, a125)?
                                    };
                                    let a127 = f28(v2.clone(), 1u64)?;
                                    nat_add(a126, a127)?
                                };
                                let a137 = {
                                    let a134 = match v3.clone() {
                                        Adt2::C0(_, h128, _, _, _, _, _, _, _, _, _) => {
                                            h128
                                        }
                                    };
                                    let a135 = {
                                        let a132 = 1u64;
                                        let a133 = {
                                            let a130 = 2u64;
                                            let a131 = match v0.clone() {
                                                Adt2::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
                                                    h129
                                                }
                                            };
                                            nat_mul(a130, a131)?
                                        };
                                        nat_add(a132, a133)?
                                    };
                                    nat_mul(a134, a135)?
                                };
                                nat_add(a136, a137)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                                    h138
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h139, _, _, _, _, _, _, _, _, _) => {
                                    h139
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                                    h140
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h141, _, _, _, _) => {
                                    h141
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h142, _, _, _) => {
                                    h142
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h143, _, _) => {
                                    h143
                                }
                            }), f27(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h144, _, _, _, _, _, _, _, _, _, _) => {
                                    h144
                                }
                            }, f27(v3, 1u64))))
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

pub fn f3(v0: u64) -> bool {
    let a145 = v0;
    let a146 = 1u64;
    if if nat_eq(a145, a146) {
        true
    } else {
        let a147 = v0;
        let a148 = 33u64;
        nat_eq(a147, a148)
    } {
        true
    } else {
        let a149 = v0;
        let a150 = 34u64;
        nat_eq(a149, a150)
    }
}

pub fn f4(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a153 = match v1.clone() {
        Adt2::C0(h151, _, _, _, _, _, _, _, _, _, _) => {
            h151
        }
    };
    let a154 = match v3.clone() {
        Adt2::C0(h152, _, _, _, _, _, _, _, _, _, _) => {
            h152
        }
    };
    if nat_eq(a153, a154) {
        f30(v0, v1, v2, v3)
    } else {
        if if if if f31(match v1.clone() {
            Adt2::C0(h155, _, _, _, _, _, _, _, _, _, _) => {
                h155
            }
        }) {
            f31(match v3.clone() {
                Adt2::C0(h156, _, _, _, _, _, _, _, _, _, _) => {
                    h156
                }
            })
        } else {
            false
        } {
            let a157 = v0;
            let a158 = 33u64;
            if nat_eq(a157, a158) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a159 = f28(v1.clone(), 1u64)?;
            let a160 = f28(v3.clone(), 1u64)?;
            nat_eq(a159, a160)
        } else {
            false
        } {
            let m162 = f32(match v3.clone() {
                Adt2::C0(h161, _, _, _, _, _, _, _, _, _, _) => {
                    h161
                }
            });
            match m162 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f28(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h163) => {
                            h163
                        }
                    }, f27(v1.clone(), 1u64), f27(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h164, _, _, _, _, _, _, _, _, _, _) => {
                            h164
                        }
                    }, match v3 {
                        Adt2::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                            h165
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f5(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a167 = match v0.clone() {
        Adt2::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
            h166
        }
    };
    let a168 = 0u64;
    if if if if nat_eq(a167, a168) {
        true
    } else {
        let a170 = match v0.clone() {
            Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a171 = 1u64;
        nat_eq(a170, a171)
    } {
        true
    } else {
        let a173 = match v0.clone() {
            Adt2::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
                h172
            }
        };
        let a174 = 25u64;
        if nat_eq(a173, a174) {
            true
        } else {
            let a176 = match v0.clone() {
                Adt2::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                    h175
                }
            };
            let a177 = 26u64;
            nat_eq(a176, a177)
        }
    } {
        true
    } else {
        let a179 = match v0.clone() {
            Adt2::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
                h178
            }
        };
        let a180 = 30u64;
        nat_eq(a179, a180)
    } {
        let a183 = match v0.clone() {
            Adt2::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a184 = match v1.clone() {
            Adt2::C0(h182, _, _, _, _, _, _, _, _, _, _) => {
                h182
            }
        };
        if if nat_eq(a183, a184) {
            f33(match v1.clone() {
                Adt2::C0(_, h185, _, _, _, _, _, _, _, _, _) => {
                    h185
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                    h186
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h187, _, _, _, _, _, _, _) => {
                    h187
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h188, _, _, _, _, _, _) => {
                    h188
                }
            }, match v0.clone() {
                Adt2::C0(_, h189, _, _, _, _, _, _, _, _, _) => {
                    h189
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h190, _, _, _, _, _, _, _, _) => {
                    h190
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h191, _, _, _, _, _, _, _) => {
                    h191
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h192, _, _, _, _, _, _) => {
                    h192
                }
            })
        } else {
            false
        } {
            let m194 = f32(match v0.clone() {
                Adt2::C0(h193, _, _, _, _, _, _, _, _, _, _) => {
                    h193
                }
            });
            match m194 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f28(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h195) => {
                            h195
                        }
                    }, f27(v0, 1u64), f27(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f6(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m196 = v2;
    match m196 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a197 = v0;
            let a198 = 4u64;
            if if if nat_lt(a197, a198) {
                let a201 = match v1.clone() {
                    Adt2::C0(h199, _, _, _, _, _, _, _, _, _, _) => {
                        h199
                    }
                };
                let a202 = match v4.clone() {
                    Adt2::C0(h200, _, _, _, _, _, _, _, _, _, _) => {
                        h200
                    }
                };
                if nat_eq(a201, a202) {
                    let a205 = match v1.clone() {
                        Adt2::C0(h203, _, _, _, _, _, _, _, _, _, _) => {
                            h203
                        }
                    };
                    let a206 = match v3.clone() {
                        Adt2::C0(h204, _, _, _, _, _, _, _, _, _, _) => {
                            h204
                        }
                    };
                    nat_eq(a205, a206)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f34(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h207, _, _, _, _, _, _, _, _, _) => {
                        h207
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h208, _, _, _, _, _, _, _, _, _) => {
                        h208
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h209, _, _, _, _, _, _, _, _, _) => {
                        h209
                    }
                })? {
                    f34(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h210, _, _, _, _, _, _, _, _) => {
                            h210
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                            h211
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h212, _, _, _, _, _, _, _, _) => {
                            h212
                        }
                    })?
                } else {
                    false
                } {
                    f34(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h213, _, _, _, _, _, _, _) => {
                            h213
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h214, _, _, _, _, _, _, _) => {
                            h214
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h215, _, _, _, _, _, _, _) => {
                            h215
                        }
                    })?
                } else {
                    false
                } {
                    f34(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h216, _, _, _, _, _, _) => {
                            h216
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h217, _, _, _, _, _, _) => {
                            h217
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h218, _, _, _, _, _, _) => {
                            h218
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m220 = f32(match v1.clone() {
                    Adt2::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                        h219
                    }
                });
                match m220 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h221, _) => {
                                h221
                            }
                        };
                        let a226 = {
                            let a223 = match v1.clone() {
                                Adt2::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                                    h222
                                }
                            };
                            let a224 = v6;
                            let a225 = 1u64;
                            nat_rem(a223, a224, a225)
                        };
                        let a227 = 0u64;
                        if if if nat_eq(a226, a227) {
                            let a232 = {
                                let a229 = match v4.clone() {
                                    Adt2::C0(_, h228, _, _, _, _, _, _, _, _, _) => {
                                        h228
                                    }
                                };
                                let a230 = v6;
                                let a231 = 1u64;
                                nat_rem(a229, a230, a231)
                            };
                            let a233 = 0u64;
                            nat_eq(a232, a233)
                        } else {
                            false
                        } {
                            let a238 = {
                                let a235 = match v3.clone() {
                                    Adt2::C0(_, h234, _, _, _, _, _, _, _, _, _) => {
                                        h234
                                    }
                                };
                                let a236 = v6;
                                let a237 = 1u64;
                                nat_rem(a235, a236, a237)
                            };
                            let a239 = 0u64;
                            nat_eq(a238, a239)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f28(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h240) => {
                                    h240
                                }
                            }, f27(v1, v6), f27(v3, v6), Adt4::C2(v0, f27(v4, v6))))
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

pub fn f7(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a242 = match v2.clone() {
        Adt2::C0(h241, _, _, _, _, _, _, _, _, _, _) => {
            h241
        }
    };
    let a243 = 0u64;
    if if nat_eq(a242, a243) {
        let a245 = match v3.clone() {
            Adt2::C0(h244, _, _, _, _, _, _, _, _, _, _) => {
                h244
            }
        };
        let a246 = 0u64;
        nat_eq(a245, a246)
    } else {
        false
    } {
        let a247 = v0;
        let a248 = 61u64;
        if nat_eq(a247, a248) {
            let v4: bool = {
                let a250 = 0u64;
                let a251 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h249, _, _, _, _, _, _, _) => {
                        h249
                    }
                };
                nat_lt(a250, a251)
            };
            if if if if if if f35(match v2.clone() {
                Adt2::C0(_, h252, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            }, match v1.clone() {
                Adt1::C0(h253, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h253
                }
            }, match v1.clone() {
                Adt1::C0(_, h254, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h254
                }
            }, match v3.clone() {
                Adt2::C0(_, h255, _, _, _, _, _, _, _, _, _) => {
                    h255
                }
            })? {
                f35(match v2.clone() {
                    Adt2::C0(_, _, h256, _, _, _, _, _, _, _, _) => {
                        h256
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h257, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h257
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h258, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h258
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h259, _, _, _, _, _, _, _, _) => {
                        h259
                    }
                })?
            } else {
                false
            } {
                f35(match v2.clone() {
                    Adt2::C0(_, _, _, h260, _, _, _, _, _, _, _) => {
                        h260
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h261, _, _, _, _, _, _, _, _, _, _, _) => {
                        h261
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h262, _, _, _, _, _, _, _, _, _, _) => {
                        h262
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h263, _, _, _, _, _, _, _) => {
                        h263
                    }
                })?
            } else {
                false
            } {
                f35(match v2.clone() {
                    Adt2::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                        h264
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h265, _, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h266, _, _, _, _, _, _, _, _) => {
                        h266
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h267, _, _, _, _, _, _) => {
                        h267
                    }
                })?
            } else {
                false
            } {
                let a269 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h268, _, _, _, _, _, _, _) => {
                        h268
                    }
                };
                let a270 = 4294967295u64;
                nat_le(a269, a270)
            } else {
                false
            } {
                if v4 {
                    !f26(match v2.clone() {
                        Adt2::C0(_, h271, _, _, _, _, _, _, _, _, _) => {
                            h271
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h272, _, _, _, _, _, _, _, _) => {
                            h272
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h273, _, _, _, _, _, _, _) => {
                            h273
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h274, _, _, _, _, _, _) => {
                            h274
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h275, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h275
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h276, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h276
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h277, _, _, _, _, _, _, _, _, _, _, _) => {
                        h277
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h278, _, _, _, _, _, _, _, _, _) => {
                        h278
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a279 = v0;
            let a280 = 62u64;
            if nat_eq(a279, a280) {
                let a282 = match v1.clone() {
                    Adt1::C0(h281, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h281
                    }
                };
                let a283 = 2147483648u64;
                if if if if if nat_lt(a282, a283) {
                    let a285 = match v1.clone() {
                        Adt1::C0(_, h284, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h284
                        }
                    };
                    let a286 = 2147483648u64;
                    nat_lt(a285, a286)
                } else {
                    false
                } {
                    let a289 = match v1.clone() {
                        Adt1::C0(h287, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h287
                        }
                    };
                    let a290 = match v2.clone() {
                        Adt2::C0(_, h288, _, _, _, _, _, _, _, _, _) => {
                            h288
                        }
                    };
                    nat_lt(a289, a290)
                } else {
                    false
                } {
                    let a293 = match v1.clone() {
                        Adt1::C0(_, h291, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h291
                        }
                    };
                    let a294 = match v2.clone() {
                        Adt2::C0(_, h292, _, _, _, _, _, _, _, _, _) => {
                            h292
                        }
                    };
                    nat_lt(a293, a294)
                } else {
                    false
                } {
                    let a297 = match v2.clone() {
                        Adt2::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                            h295
                        }
                    };
                    let a298 = match v3.clone() {
                        Adt2::C0(_, _, h296, _, _, _, _, _, _, _, _) => {
                            h296
                        }
                    };
                    if if if nat_eq(a297, a298) {
                        let a301 = match v2.clone() {
                            Adt2::C0(_, _, _, h299, _, _, _, _, _, _, _) => {
                                h299
                            }
                        };
                        let a302 = match v3.clone() {
                            Adt2::C0(_, _, _, h300, _, _, _, _, _, _, _) => {
                                h300
                            }
                        };
                        nat_eq(a301, a302)
                    } else {
                        false
                    } {
                        let a305 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h303, _, _, _, _, _, _) => {
                                h303
                            }
                        };
                        let a306 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h304, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        nat_eq(a305, a306)
                    } else {
                        false
                    } {
                        let a315 = match v3.clone() {
                            Adt2::C0(_, h307, _, _, _, _, _, _, _, _, _) => {
                                h307
                            }
                        };
                        let a316 = {
                            let a313 = {
                                let a310 = match v2.clone() {
                                    Adt2::C0(_, h308, _, _, _, _, _, _, _, _, _) => {
                                        h308
                                    }
                                };
                                let a311 = match v1.clone() {
                                    Adt1::C0(h309, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h309
                                    }
                                };
                                nat_add(a310, a311)?
                            };
                            let a314 = match v1.clone() {
                                Adt1::C0(_, h312, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h312
                                }
                            };
                            nat_add(a313, a314)?
                        };
                        nat_eq(a315, a316)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h317, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h317
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a318 = v0;
                let a319 = 63u64;
                if nat_eq(a318, a319) {
                    if if if if if f36(match v2.clone() {
                        Adt2::C0(_, h320, _, _, _, _, _, _, _, _, _) => {
                            h320
                        }
                    }, match v1.clone() {
                        Adt1::C0(h321, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h321
                        }
                    })? {
                        f36(match v2.clone() {
                            Adt2::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                                h322
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h323
                            }
                        })?
                    } else {
                        false
                    } {
                        f36(match v2.clone() {
                            Adt2::C0(_, _, _, h324, _, _, _, _, _, _, _) => {
                                h324
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h325, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h325
                            }
                        })?
                    } else {
                        false
                    } {
                        f36(match v2.clone() {
                            Adt2::C0(_, _, _, _, h326, _, _, _, _, _, _) => {
                                h326
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h327, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h327
                            }
                        })?
                    } else {
                        false
                    } {
                        let a330 = match v2.clone() {
                            Adt2::C0(_, h328, _, _, _, _, _, _, _, _, _) => {
                                h328
                            }
                        };
                        let a331 = match v3.clone() {
                            Adt2::C0(_, h329, _, _, _, _, _, _, _, _, _) => {
                                h329
                            }
                        };
                        if if if nat_eq(a330, a331) {
                            let a334 = match v2.clone() {
                                Adt2::C0(_, _, h332, _, _, _, _, _, _, _, _) => {
                                    h332
                                }
                            };
                            let a335 = match v3.clone() {
                                Adt2::C0(_, _, h333, _, _, _, _, _, _, _, _) => {
                                    h333
                                }
                            };
                            nat_eq(a334, a335)
                        } else {
                            false
                        } {
                            let a338 = match v2.clone() {
                                Adt2::C0(_, _, _, h336, _, _, _, _, _, _, _) => {
                                    h336
                                }
                            };
                            let a339 = match v3.clone() {
                                Adt2::C0(_, _, _, h337, _, _, _, _, _, _, _) => {
                                    h337
                                }
                            };
                            nat_eq(a338, a339)
                        } else {
                            false
                        } {
                            let a342 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h340, _, _, _, _, _, _) => {
                                    h340
                                }
                            };
                            let a343 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h341, _, _, _, _, _, _) => {
                                    h341
                                }
                            };
                            nat_eq(a342, a343)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h344, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h344
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h345, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h345
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h346, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h346
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h347, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h347
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

pub fn f8(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m348 = v3;
    match m348 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a349 = v0;
                let a350 = 4u64;
                nat_eq(a349, a350)
            };
            let v7: bool = {
                let a352 = 0u64;
                let a353 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h351, _, _, _, _, _, _, _, _, _, _, _) => {
                        h351
                    }
                };
                nat_lt(a352, a353)
            };
            let v8: u64 = f28(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h354, _, _, _, _, _, _, _, _, _) => {
                    h354
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h355, _, _, _, _, _, _, _, _) => {
                    h355
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h356, _, _, _, _, _, _, _) => {
                    h356
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h357, _, _, _, _, _, _) => {
                    h357
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h358, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h358
                }
            }, match v1.clone() {
                Adt1::C0(_, h359, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h359
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h360, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h360
                }
            });
            if if if if if if if if if if if if v6 {
                let a362 = match v2.clone() {
                    Adt2::C0(h361, _, _, _, _, _, _, _, _, _, _) => {
                        h361
                    }
                };
                let a363 = 0u64;
                if nat_eq(a362, a363) {
                    let a365 = match v5.clone() {
                        Adt2::C0(h364, _, _, _, _, _, _, _, _, _, _) => {
                            h364
                        }
                    };
                    let a366 = 0u64;
                    nat_eq(a365, a366)
                } else {
                    false
                }
            } else {
                let a368 = match v2.clone() {
                    Adt2::C0(h367, _, _, _, _, _, _, _, _, _, _) => {
                        h367
                    }
                };
                let a369 = 0u64;
                if if nat_eq(a368, a369) {
                    true
                } else {
                    let a371 = match v2.clone() {
                        Adt2::C0(h370, _, _, _, _, _, _, _, _, _, _) => {
                            h370
                        }
                    };
                    let a372 = 26u64;
                    nat_eq(a371, a372)
                } {
                    let a374 = match v5.clone() {
                        Adt2::C0(h373, _, _, _, _, _, _, _, _, _, _) => {
                            h373
                        }
                    };
                    let a375 = 0u64;
                    if nat_eq(a374, a375) {
                        true
                    } else {
                        let a377 = match v5.clone() {
                            Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                                h376
                            }
                        };
                        let a378 = 26u64;
                        nat_eq(a377, a378)
                    }
                } else {
                    false
                }
            } {
                let a381 = match v2.clone() {
                    Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                        h379
                    }
                };
                let a382 = match v4.clone() {
                    Adt2::C0(h380, _, _, _, _, _, _, _, _, _, _) => {
                        h380
                    }
                };
                nat_eq(a381, a382)
            } else {
                false
            } {
                f37(v2.clone(), v4.clone())
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a384 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h383, _, _, _, _, _) => {
                        h383
                    }
                };
                let a385 = 4u64;
                nat_eq(a384, a385)
            } else {
                false
            } {
                let a386 = v8;
                let a387 = f28(v2.clone(), 1u64)?;
                nat_le(a386, a387)
            } else {
                false
            } {
                let a389 = match v1.clone() {
                    Adt1::C0(_, _, _, h388, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h388
                    }
                };
                let a390 = 1073741824u64;
                nat_lt(a389, a390)
            } else {
                false
            } {
                let a392 = match v1.clone() {
                    Adt1::C0(h391, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h391
                    }
                };
                let a393 = 4294967295u64;
                if if if if nat_le(a392, a393) {
                    let a395 = match v1.clone() {
                        Adt1::C0(_, h394, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h394
                        }
                    };
                    let a396 = 4294967295u64;
                    nat_le(a395, a396)
                } else {
                    false
                } {
                    let a398 = match v1.clone() {
                        Adt1::C0(_, _, h397, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h397
                        }
                    };
                    let a399 = 4294967295u64;
                    nat_le(a398, a399)
                } else {
                    false
                } {
                    let a401 = match v1.clone() {
                        Adt1::C0(_, _, _, h400, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h400
                        }
                    };
                    let a402 = 4294967295u64;
                    nat_le(a401, a402)
                } else {
                    false
                } {
                    let a404 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h403, _, _, _, _, _, _, _, _, _, _, _) => {
                            h403
                        }
                    };
                    let a405 = 4294967295u64;
                    nat_le(a404, a405)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a408 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h406) => {
                            h406
                        }
                    };
                    let a409 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h407) => {
                            h407
                        }
                    };
                    nat_eq(a408, a409)
                } else {
                    true
                }
            } else {
                false
            } {
                f39(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h410, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h410
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h411, _) => {
                        h411
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
                    let a412 = v10;
                    let a413 = v8;
                    nat_add(a412, a413)?
                }, 4u64, f27(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h414, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h414
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f9(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a416 = match v0.clone() {
        Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
            h415
        }
    };
    let a417 = 0u64;
    if if if nat_eq(a416, a417) {
        let a419 = match v1.clone() {
            Adt2::C0(h418, _, _, _, _, _, _, _, _, _, _) => {
                h418
            }
        };
        let a420 = 0u64;
        nat_eq(a419, a420)
    } else {
        false
    } {
        f33(match v0.clone() {
            Adt2::C0(_, h421, _, _, _, _, _, _, _, _, _) => {
                h421
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h422, _, _, _, _, _, _, _, _) => {
                h422
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h423, _, _, _, _, _, _, _) => {
                h423
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h424, _, _, _, _, _, _) => {
                h424
            }
        }, match v1.clone() {
            Adt2::C0(_, h425, _, _, _, _, _, _, _, _, _) => {
                h425
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h426, _, _, _, _, _, _, _, _) => {
                h426
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h427, _, _, _, _, _, _, _) => {
                h427
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h428, _, _, _, _, _, _) => {
                h428
            }
        })
    } else {
        false
    } {
        let v2: u64 = f28(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a429 = v2;
            let a430 = f28(v0.clone(), 1u64)?;
            nat_add(a429, a430)?
        }, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f10(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a432 = match v1.clone() {
        Adt2::C0(h431, _, _, _, _, _, _, _, _, _, _) => {
            h431
        }
    };
    let a433 = 0u64;
    if if if nat_eq(a432, a433) {
        true
    } else {
        let a435 = match v1.clone() {
            Adt2::C0(h434, _, _, _, _, _, _, _, _, _, _) => {
                h434
            }
        };
        let a436 = 1u64;
        nat_eq(a435, a436)
    } {
        true
    } else {
        let a438 = match v1.clone() {
            Adt2::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
                h437
            }
        };
        let a439 = 30u64;
        nat_eq(a438, a439)
    } {
        let a442 = match v1.clone() {
            Adt2::C0(h440, _, _, _, _, _, _, _, _, _, _) => {
                h440
            }
        };
        let a443 = match v2.clone() {
            Adt2::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
                h441
            }
        };
        if if if if if nat_eq(a442, a443) {
            let a446 = match v1.clone() {
                Adt2::C0(_, h444, _, _, _, _, _, _, _, _, _) => {
                    h444
                }
            };
            let a447 = match v2.clone() {
                Adt2::C0(_, h445, _, _, _, _, _, _, _, _, _) => {
                    h445
                }
            };
            nat_eq(a446, a447)
        } else {
            false
        } {
            let a450 = match v1.clone() {
                Adt2::C0(_, _, _, h448, _, _, _, _, _, _, _) => {
                    h448
                }
            };
            let a451 = match v2.clone() {
                Adt2::C0(_, _, _, h449, _, _, _, _, _, _, _) => {
                    h449
                }
            };
            nat_eq(a450, a451)
        } else {
            false
        } {
            let a454 = match v1.clone() {
                Adt2::C0(_, _, _, _, h452, _, _, _, _, _, _) => {
                    h452
                }
            };
            let a455 = match v2.clone() {
                Adt2::C0(_, _, _, _, h453, _, _, _, _, _, _) => {
                    h453
                }
            };
            nat_eq(a454, a455)
        } else {
            false
        } {
            let a456 = v0;
            let a457 = 4u64;
            if nat_eq(a456, a457) {
                let a459 = match v1.clone() {
                    Adt2::C0(_, _, h458, _, _, _, _, _, _, _, _) => {
                        h458
                    }
                };
                let a460 = 1u64;
                if nat_eq(a459, a460) {
                    let a463 = match v1.clone() {
                        Adt2::C0(_, h461, _, _, _, _, _, _, _, _, _) => {
                            h461
                        }
                    };
                    let a464 = match v2.clone() {
                        Adt2::C0(_, _, h462, _, _, _, _, _, _, _, _) => {
                            h462
                        }
                    };
                    nat_eq(a463, a464)
                } else {
                    false
                }
            } else {
                let a465 = v0;
                let a466 = 4u64;
                if if nat_lt(a465, a466) {
                    let a469 = match v1.clone() {
                        Adt2::C0(_, h467, _, _, _, _, _, _, _, _, _) => {
                            h467
                        }
                    };
                    let a470 = match v1.clone() {
                        Adt2::C0(_, _, h468, _, _, _, _, _, _, _, _) => {
                            h468
                        }
                    };
                    nat_eq(a469, a470)
                } else {
                    false
                } {
                    let a473 = match v1.clone() {
                        Adt2::C0(_, _, h471, _, _, _, _, _, _, _, _) => {
                            h471
                        }
                    };
                    let a474 = match v2.clone() {
                        Adt2::C0(_, _, h472, _, _, _, _, _, _, _, _) => {
                            h472
                        }
                    };
                    nat_eq(a473, a474)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m476 = f32(match v1.clone() {
                Adt2::C0(h475, _, _, _, _, _, _, _, _, _, _) => {
                    h475
                }
            });
            match m476 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f28(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h477) => {
                            h477
                        }
                    }, f27(v1, 1u64), f27(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f11(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f40(match v1.clone() {
        Adt2::C0(h478, _, _, _, _, _, _, _, _, _, _) => {
            h478
        }
    }, v0) {
        let a481 = match v1.clone() {
            Adt2::C0(h479, _, _, _, _, _, _, _, _, _, _) => {
                h479
            }
        };
        let a482 = match v2.clone() {
            Adt2::C0(h480, _, _, _, _, _, _, _, _, _, _) => {
                h480
            }
        };
        nat_eq(a481, a482)
    } else {
        false
    } {
        let a485 = match v1.clone() {
            Adt2::C0(_, h483, _, _, _, _, _, _, _, _, _) => {
                h483
            }
        };
        let a486 = match v2.clone() {
            Adt2::C0(_, h484, _, _, _, _, _, _, _, _, _) => {
                h484
            }
        };
        if if if nat_eq(a485, a486) {
            let a489 = match v1.clone() {
                Adt2::C0(_, _, h487, _, _, _, _, _, _, _, _) => {
                    h487
                }
            };
            let a490 = match v2.clone() {
                Adt2::C0(_, _, h488, _, _, _, _, _, _, _, _) => {
                    h488
                }
            };
            nat_eq(a489, a490)
        } else {
            false
        } {
            let a493 = match v1.clone() {
                Adt2::C0(_, _, _, h491, _, _, _, _, _, _, _) => {
                    h491
                }
            };
            let a494 = match v2.clone() {
                Adt2::C0(_, _, _, h492, _, _, _, _, _, _, _) => {
                    h492
                }
            };
            nat_eq(a493, a494)
        } else {
            false
        } {
            let a497 = match v1.clone() {
                Adt2::C0(_, _, _, _, h495, _, _, _, _, _, _) => {
                    h495
                }
            };
            let a498 = match v2.clone() {
                Adt2::C0(_, _, _, _, h496, _, _, _, _, _, _) => {
                    h496
                }
            };
            nat_eq(a497, a498)
        } else {
            false
        }
    } else {
        false
    } {
        let m500 = f32(match v1.clone() {
            Adt2::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                h499
            }
        });
        match m500 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f28(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h501) => {
                        h501
                    }
                }, f27(v1.clone(), 1u64), f27(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h502, _, _, _, _, _, _, _, _, _, _) => {
                        h502
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f12(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h503, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h503
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h504, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h504
            }
        }
    };
    let a505 = 0u64;
    let a506 = v4;
    if if nat_lt(a505, a506) {
        let a507 = v4;
        let a508 = 2147483648u64;
        nat_lt(a507, a508)
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
        let v7: u64 = f41(match v5.clone() {
            Adt2::C0(_, _, h509, _, _, _, _, _, _, _, _) => {
                h509
            }
        }, v4)?;
        let v8: u64 = f41(match v5.clone() {
            Adt2::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                h510
            }
        }, v4)?;
        let a512 = match v5.clone() {
            Adt2::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                h511
            }
        };
        let a513 = 0u64;
        if if if if if nat_eq(a512, a513) {
            let a515 = match v6.clone() {
                Adt2::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
                    h514
                }
            };
            let a516 = 0u64;
            nat_eq(a515, a516)
        } else {
            false
        } {
            let a518 = match v5.clone() {
                Adt2::C0(_, _, _, _, h517, _, _, _, _, _, _) => {
                    h517
                }
            };
            let a519 = 1u64;
            nat_eq(a518, a519)
        } else {
            false
        } {
            let a522 = match v5.clone() {
                Adt2::C0(_, h520, _, _, _, _, _, _, _, _, _) => {
                    h520
                }
            };
            let a523 = match v6.clone() {
                Adt2::C0(_, h521, _, _, _, _, _, _, _, _, _) => {
                    h521
                }
            };
            nat_eq(a522, a523)
        } else {
            false
        } {
            if v0 {
                let a525 = match v5.clone() {
                    Adt2::C0(_, _, h524, _, _, _, _, _, _, _, _) => {
                        h524
                    }
                };
                let a526 = 2147483648u64;
                if if if nat_lt(a525, a526) {
                    let a528 = match v5.clone() {
                        Adt2::C0(_, _, _, h527, _, _, _, _, _, _, _) => {
                            h527
                        }
                    };
                    let a529 = 2147483648u64;
                    nat_lt(a528, a529)
                } else {
                    false
                } {
                    let a532 = {
                        let a530 = v7;
                        let a531 = v8;
                        nat_mul(a530, a531)?
                    };
                    let a533 = 2147483648u64;
                    nat_le(a532, a533)
                } else {
                    false
                } {
                    f42(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a535 = match v1.clone() {
                    Adt1::C0(h534, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h534
                    }
                };
                let a536 = v7;
                if if if if nat_eq(a535, a536) {
                    let a538 = match v1 {
                        Adt1::C0(_, h537, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h537
                        }
                    };
                    let a539 = v8;
                    nat_eq(a538, a539)
                } else {
                    false
                } {
                    f43(v7, v8, v4)?
                } else {
                    false
                } {
                    let a541 = match v6.clone() {
                        Adt2::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                            h540
                        }
                    };
                    let a542 = v4;
                    if nat_eq(a541, a542) {
                        let a544 = match v6.clone() {
                            Adt2::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                                h543
                            }
                        };
                        let a545 = v4;
                        nat_eq(a544, a545)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a549 = match v6 {
                        Adt2::C0(_, _, _, _, h546, _, _, _, _, _, _) => {
                            h546
                        }
                    };
                    let a550 = {
                        let a547 = v7;
                        let a548 = v8;
                        nat_mul(a547, a548)?
                    };
                    nat_eq(a549, a550)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt6::C4(f28(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f13(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a552 = match v2.clone() {
        Adt2::C0(h551, _, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let a553 = 0u64;
    if if if if if nat_eq(a552, a553) {
        true
    } else {
        let a555 = match v2.clone() {
            Adt2::C0(h554, _, _, _, _, _, _, _, _, _, _) => {
                h554
            }
        };
        let a556 = 1u64;
        nat_eq(a555, a556)
    } {
        true
    } else {
        let a558 = match v2.clone() {
            Adt2::C0(h557, _, _, _, _, _, _, _, _, _, _) => {
                h557
            }
        };
        let a559 = 30u64;
        nat_eq(a558, a559)
    } {
        let a562 = match v2.clone() {
            Adt2::C0(h560, _, _, _, _, _, _, _, _, _, _) => {
                h560
            }
        };
        let a563 = match v3.clone() {
            Adt2::C0(h561, _, _, _, _, _, _, _, _, _, _) => {
                h561
            }
        };
        nat_eq(a562, a563)
    } else {
        false
    } {
        if if f37(v2.clone(), v3.clone()) {
            let a565 = match v1.clone() {
                Adt1::C0(h564, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h564
                }
            };
            let a566 = 4294967296u64;
            nat_lt(a565, a566)
        } else {
            false
        } {
            let a568 = match v1.clone() {
                Adt1::C0(_, h567, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h567
                }
            };
            let a569 = 4294967296u64;
            nat_lt(a568, a569)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v3.clone(), 1u64)?, {
            let a571 = match v2.clone() {
                Adt2::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                    h570
                }
            };
            let a572 = 0u64;
            if nat_eq(a571, a572) {
                4u64
            } else {
                2u64
            }
        }, f27(v2.clone(), 1u64), f27(v3, 1u64), Adt4::C13({
            let a573 = v0;
            let a574 = 31u64;
            if nat_eq(a573, a574) {
                0u64
            } else {
                let a575 = v0;
                let a576 = 47u64;
                if nat_eq(a575, a576) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h577, _, _, _, _, _, _, _, _, _, _) => {
                h577
            }
        }, match v1.clone() {
            Adt1::C0(h578, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h578
            }
        }, match v1 {
            Adt1::C0(_, h579, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h579
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f14(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a581 = match v0.clone() {
        Adt2::C0(h580, _, _, _, _, _, _, _, _, _, _) => {
            h580
        }
    };
    let a582 = 0u64;
    if if if nat_eq(a581, a582) {
        let a584 = match v1.clone() {
            Adt2::C0(h583, _, _, _, _, _, _, _, _, _, _) => {
                h583
            }
        };
        let a585 = 0u64;
        nat_eq(a584, a585)
    } else {
        false
    } {
        f37(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v1.clone(), 1u64)?, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f15(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a587 = match v0.clone() {
        Adt2::C0(h586, _, _, _, _, _, _, _, _, _, _) => {
            h586
        }
    };
    let a588 = 0u64;
    if if nat_eq(a587, a588) {
        let a590 = match v1.clone() {
            Adt2::C0(h589, _, _, _, _, _, _, _, _, _, _) => {
                h589
            }
        };
        let a591 = 26u64;
        nat_eq(a590, a591)
    } else {
        false
    } {
        let a593 = match v0.clone() {
            Adt2::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
                h592
            }
        };
        let a594 = 1u64;
        if if if if if if if nat_eq(a593, a594) {
            let a596 = match v0.clone() {
                Adt2::C0(_, _, _, _, h595, _, _, _, _, _, _) => {
                    h595
                }
            };
            let a597 = 1u64;
            nat_eq(a596, a597)
        } else {
            false
        } {
            let a599 = match v0.clone() {
                Adt2::C0(_, h598, _, _, _, _, _, _, _, _, _) => {
                    h598
                }
            };
            let a600 = 2147483647u64;
            nat_le(a599, a600)
        } else {
            false
        } {
            let a603 = match v1.clone() {
                Adt2::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                    h601
                }
            };
            let a604 = match v0.clone() {
                Adt2::C0(_, _, h602, _, _, _, _, _, _, _, _) => {
                    h602
                }
            };
            nat_eq(a603, a604)
        } else {
            false
        } {
            let a606 = match v1.clone() {
                Adt2::C0(_, _, h605, _, _, _, _, _, _, _, _) => {
                    h605
                }
            };
            let a607 = 1u64;
            nat_eq(a606, a607)
        } else {
            false
        } {
            let a609 = match v1.clone() {
                Adt2::C0(_, _, _, h608, _, _, _, _, _, _, _) => {
                    h608
                }
            };
            let a610 = 1u64;
            nat_eq(a609, a610)
        } else {
            false
        } {
            let a612 = match v1.clone() {
                Adt2::C0(_, _, _, _, h611, _, _, _, _, _, _) => {
                    h611
                }
            };
            let a613 = 1u64;
            nat_eq(a612, a613)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a616 = match v0.clone() {
                    Adt2::C0(_, _, h614, _, _, _, _, _, _, _, _) => {
                        h614
                    }
                };
                let a617 = f44(match v0.clone() {
                    Adt2::C0(_, h615, _, _, _, _, _, _, _, _, _) => {
                        h615
                    }
                })?;
                nat_mul(a616, a617)?
            }, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f16(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m618 = v1;
    match m618 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f45(f27(v0.clone(), 1u64))?;
            let a620 = match v0.clone() {
                Adt2::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                    h619
                }
            };
            let a621 = 26u64;
            if if if nat_eq(a620, a621) {
                let a623 = match v3.clone() {
                    Adt2::C0(h622, _, _, _, _, _, _, _, _, _, _) => {
                        h622
                    }
                };
                let a624 = 26u64;
                nat_eq(a623, a624)
            } else {
                false
            } {
                let a626 = match v2.clone() {
                    Adt2::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                        h625
                    }
                };
                let a627 = 27u64;
                nat_eq(a626, a627)
            } else {
                false
            } {
                if if if if if if f37(v0.clone(), v3.clone()) {
                    let a629 = match v2.clone() {
                        Adt2::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                            h628
                        }
                    };
                    let a630 = 1u64;
                    nat_eq(a629, a630)
                } else {
                    false
                } {
                    let a632 = match v2.clone() {
                        Adt2::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                            h631
                        }
                    };
                    let a633 = 1u64;
                    nat_eq(a632, a633)
                } else {
                    false
                } {
                    let a635 = match v2.clone() {
                        Adt2::C0(_, _, _, h634, _, _, _, _, _, _, _) => {
                            h634
                        }
                    };
                    let a636 = 1u64;
                    nat_eq(a635, a636)
                } else {
                    false
                } {
                    let a638 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h637, _, _, _, _, _, _) => {
                            h637
                        }
                    };
                    let a639 = 1u64;
                    nat_eq(a638, a639)
                } else {
                    false
                } {
                    let a640 = v4;
                    let a641 = 9223372036854775808u64;
                    nat_lt(a640, a641)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a644 = 2u64;
                        let a645 = {
                            let a642 = v4;
                            let a643 = 2u64;
                            nat_mul(a642, a643)?
                        };
                        nat_add(a644, a645)?
                    }, 8u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C18(v4, f27(v3, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                Ok(Adt6::C0)
            }
        }
    }
}

pub fn f17(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
            h646
        }
    };
    let a647 = v3;
    let a648 = 0u64;
    if if nat_eq(a647, a648) {
        true
    } else {
        let a649 = v0;
        let a650 = 13u64;
        if nat_eq(a649, a650) {
            let a651 = v3;
            let a652 = 1u64;
            if nat_eq(a651, a652) {
                true
            } else {
                let a653 = v3;
                let a654 = 30u64;
                nat_eq(a653, a654)
            }
        } else {
            false
        }
    } {
        let a656 = v3;
        let a657 = match v2.clone() {
            Adt2::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
                h655
            }
        };
        if if nat_eq(a656, a657) {
            let a658 = v0;
            let a659 = 13u64;
            if nat_eq(a658, a659) {
                let a661 = match v2.clone() {
                    Adt2::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
                        h660
                    }
                };
                let a662 = 1u64;
                if if if nat_eq(a661, a662) {
                    let a664 = match v2.clone() {
                        Adt2::C0(_, _, h663, _, _, _, _, _, _, _, _) => {
                            h663
                        }
                    };
                    let a665 = 1u64;
                    nat_eq(a664, a665)
                } else {
                    false
                } {
                    let a667 = match v2.clone() {
                        Adt2::C0(_, _, _, h666, _, _, _, _, _, _, _) => {
                            h666
                        }
                    };
                    let a668 = 1u64;
                    nat_eq(a667, a668)
                } else {
                    false
                } {
                    let a670 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h669, _, _, _, _, _, _) => {
                            h669
                        }
                    };
                    let a671 = 1u64;
                    nat_eq(a670, a671)
                } else {
                    false
                }
            } else {
                let a673 = match v2.clone() {
                    Adt2::C0(_, h672, _, _, _, _, _, _, _, _, _) => {
                        h672
                    }
                };
                let a674 = 1u64;
                if if if nat_eq(a673, a674) {
                    let a677 = match v1.clone() {
                        Adt2::C0(_, _, h675, _, _, _, _, _, _, _, _) => {
                            h675
                        }
                    };
                    let a678 = match v2.clone() {
                        Adt2::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                            h676
                        }
                    };
                    nat_eq(a677, a678)
                } else {
                    false
                } {
                    let a681 = match v1.clone() {
                        Adt2::C0(_, _, _, h679, _, _, _, _, _, _, _) => {
                            h679
                        }
                    };
                    let a682 = match v2.clone() {
                        Adt2::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                            h680
                        }
                    };
                    nat_eq(a681, a682)
                } else {
                    false
                } {
                    let a685 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                            h683
                        }
                    };
                    let a686 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h684, _, _, _, _, _, _) => {
                            h684
                        }
                    };
                    nat_eq(a685, a686)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f27(v1, 1u64);
            Ok(Adt6::C4(f46(v4.clone(), v0, v3)?, {
                let a687 = v3;
                let a688 = 0u64;
                if nat_eq(a687, a688) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f27(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f18(v0: u64, v1: u64) -> Option<u64> {
    let a689 = v0;
    let a690 = 8u64;
    if nat_eq(a689, a690) {
        Some(5u64)
    } else {
        let a691 = v0;
        let a692 = 9u64;
        if nat_eq(a691, a692) {
            Some(6u64)
        } else {
            let a693 = v0;
            let a694 = 89u64;
            if nat_eq(a693, a694) {
                let a695 = v1;
                let a696 = 0u64;
                if nat_eq(a695, a696) {
                    Some(0u64)
                } else {
                    let a697 = v1;
                    let a698 = 1u64;
                    if nat_eq(a697, a698) {
                        Some(1u64)
                    } else {
                        let a699 = v1;
                        let a700 = 2u64;
                        if nat_eq(a699, a700) {
                            Some(2u64)
                        } else {
                            let a701 = v1;
                            let a702 = 3u64;
                            if nat_eq(a701, a702) {
                                Some(3u64)
                            } else {
                                let a703 = v1;
                                let a704 = 6u64;
                                if nat_eq(a703, a704) {
                                    Some(4u64)
                                } else {
                                    let a705 = v1;
                                    let a706 = 12u64;
                                    if nat_eq(a705, a706) {
                                        Some(7u64)
                                    } else {
                                        let a707 = v1;
                                        let a708 = 11u64;
                                        if nat_eq(a707, a708) {
                                            Some(8u64)
                                        } else {
                                            let a709 = v1;
                                            let a710 = 18u64;
                                            if nat_eq(a709, a710) {
                                                Some(9u64)
                                            } else {
                                                let a711 = v1;
                                                let a712 = 19u64;
                                                if nat_eq(a711, a712) {
                                                    Some(10u64)
                                                } else {
                                                    let a713 = v1;
                                                    let a714 = 20u64;
                                                    if nat_eq(a713, a714) {
                                                        Some(11u64)
                                                    } else {
                                                        let a715 = v1;
                                                        let a716 = 21u64;
                                                        if nat_eq(a715, a716) {
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

pub fn f19(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m717 = f47(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m717 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f27(v2.clone(), 1u64), f27(v4, 1u64), Adt4::C10(v5, v6, {
                let m718 = v3;
                match m718 {
                    None => {
                        f27(v2, 1u64)
                    }
                    Some(v12) => {
                        f27(v12, 1u64)
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

pub fn f20(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a720 = match v1.clone() {
        Adt2::C0(h719, _, _, _, _, _, _, _, _, _, _) => {
            h719
        }
    };
    let a721 = 0u64;
    if if if if if nat_eq(a720, a721) {
        true
    } else {
        let a723 = match v1.clone() {
            Adt2::C0(h722, _, _, _, _, _, _, _, _, _, _) => {
                h722
            }
        };
        let a724 = 1u64;
        nat_eq(a723, a724)
    } {
        true
    } else {
        let a726 = match v1.clone() {
            Adt2::C0(h725, _, _, _, _, _, _, _, _, _, _) => {
                h725
            }
        };
        let a727 = 30u64;
        nat_eq(a726, a727)
    } {
        let a730 = match v1.clone() {
            Adt2::C0(h728, _, _, _, _, _, _, _, _, _, _) => {
                h728
            }
        };
        let a731 = match v2.clone() {
            Adt2::C0(h729, _, _, _, _, _, _, _, _, _, _) => {
                h729
            }
        };
        if nat_eq(a730, a731) {
            true
        } else {
            let a733 = match v2.clone() {
                Adt2::C0(h732, _, _, _, _, _, _, _, _, _, _) => {
                    h732
                }
            };
            let a734 = 0u64;
            nat_eq(a733, a734)
        }
    } else {
        false
    } {
        f37(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f28(v2.clone(), 1u64)?, {
            let a736 = match v2.clone() {
                Adt2::C0(h735, _, _, _, _, _, _, _, _, _, _) => {
                    h735
                }
            };
            let a737 = 0u64;
            if nat_eq(a736, a737) {
                4u64
            } else {
                2u64
            }
        }, f27(v1.clone(), 1u64), f27(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h738, _, _, _, _, _, _, _, _, _, _) => {
                h738
            }
        }, match v2 {
            Adt2::C0(h739, _, _, _, _, _, _, _, _, _, _) => {
                h739
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f21(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m740 = v3;
    match m740 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f48(v0, v1)?, f48(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f49(v0, v1, v2.clone())?, f48(v0, v2)?))
        }
        Adt4::C2(v4, v5) => {
            f50(v0, v1, v5, v2, v4)
        }
        Adt4::C3(v6, v7, v8, v9, v10) => {
            f51(v0, v1, v2, v6, v7, v8, v9, v10)
        }
        Adt4::C4(v11) => {
            f52(v0, v1, v2, v11)
        }
        Adt4::C5(v12, v13, v14, v15) => {
            f51(v0, v1, v2, v12, v13, v14, v15, true)
        }
        Adt4::C6(v16, v17, v18) => {
            f53(v0, v1, v2, v16, v17, v18)
        }
        Adt4::C7(v19) => {
            f54(v0, v1, v2, v19)
        }
        Adt4::C8(v20) => {
            f55(v20, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f48(v0, v2)?))
        }
        Adt4::C10(v23, v24, v25) => {
            f56(v0, v1, v2, v23, v24, v25)
        }
        Adt4::C11(v26, v27) => {
            Ok(Adt5::C3(v26, v27, f48(v0, v1)?, f48(v0, v2)?))
        }
        Adt4::C12(v28, v29, v30) => {
            f57(v0, v1, v2, v28, v29, v30)
        }
        Adt4::C13(_, v32, _, _) => {
            Ok(Adt5::C3(v32, v32, f48(v0, v1)?, f48(v0, v2)?))
        }
        Adt4::C14(_, v36, v37) => {
            Ok(Adt5::C3(v36, v37, f48(v0, v1)?, f48(v0, v2)?))
        }
        Adt4::C15 => {
            f58(v0, v1, v2)
        }
        Adt4::C16(v38, v39) => {
            f59(v0, v1, v2, v38, v39)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v40, v41) => {
            f60(v0, v1, v41, v40)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f22(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f44(match v1.clone() {
        Adt3::C0(h741, _, _, _, _, _, _, _) => {
            h741
        }
    })?;
    let v5: u64 = {
        let a742 = v0;
        let a743 = v4;
        let a744 = 0u64;
        nat_quot(a742, a743, a744)
    };
    let v6: u64 = {
        let a745 = v0;
        let a746 = v4;
        let a747 = 0u64;
        nat_rem(a745, a746, a747)
    };
    let a748 = v6;
    let a749 = 0u64;
    if nat_eq(a748, a749) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a750 = v6;
        let a751 = 1u64;
        if nat_eq(a750, a751) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a754 = {
                let a752 = v6;
                let a753 = 1u64;
                nat_add(a752, a753)?
            };
            let a755 = v4;
            if nat_eq(a754, a755) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f48(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a756 = v6;
                    let a757 = 2u64;
                    nat_sub(a756, a757)
                };
                let v8: u64 = {
                    let a758 = v7;
                    let a759 = 3u64;
                    let a760 = 0u64;
                    nat_quot(a758, a759, a760)
                };
                let v9: u64 = {
                    let a761 = v7;
                    let a762 = 3u64;
                    let a763 = 0u64;
                    nat_rem(a761, a762, a763)
                };
                let v10: u64 = f48({
                    let a767 = {
                        let a765 = v5;
                        let a766 = match v1.clone() {
                            Adt3::C0(h764, _, _, _, _, _, _, _) => {
                                h764
                            }
                        };
                        nat_mul(a765, a766)?
                    };
                    let a768 = v8;
                    nat_add(a767, a768)?
                }, v1)?;
                let a769 = v9;
                let a770 = 0u64;
                if nat_eq(a769, a770) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a771 = v9;
                    let a772 = 1u64;
                    if nat_eq(a771, a772) {
                        let a773 = v3;
                        let a774 = 2u64;
                        if nat_eq(a773, a774) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a775 = v3;
                        let a776 = 2u64;
                        if if nat_eq(a775, a776) {
                            true
                        } else {
                            let a777 = v3;
                            let a778 = 3u64;
                            nat_eq(a777, a778)
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

pub fn f23(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a780 = 2u64;
        let a781 = match v1.clone() {
            Adt3::C0(_, h779, _, _, _, _, _, _) => {
                h779
            }
        };
        nat_mul(a780, a781)?
    };
    let a782 = v0;
    let a783 = v5;
    if nat_lt(a782, a783) {
        let a787 = {
            let a784 = v0;
            let a785 = 2u64;
            let a786 = 0u64;
            nat_rem(a784, a785, a786)
        };
        let a788 = 0u64;
        if nat_eq(a787, a788) {
            Ok(Adt5::C4(19u64, 4u64, f48({
                let a789 = v0;
                let a790 = 2u64;
                let a791 = 0u64;
                nat_quot(a789, a790, a791)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a792 = v4;
            let a793 = 2147483648u64;
            if if nat_lt(a792, a793) {
                let a795 = v4;
                let a796 = match v2 {
                    Adt3::C0(_, h794, _, _, _, _, _, _) => {
                        h794
                    }
                };
                nat_lt(a795, a796)
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
            let a799 = match v2.clone() {
                Adt3::C0(h797, _, _, _, _, _, _, _) => {
                    h797
                }
            };
            let a800 = match v2.clone() {
                Adt3::C0(_, h798, _, _, _, _, _, _) => {
                    h798
                }
            };
            nat_mul(a799, a800)?
        };
        let a803 = {
            let a801 = v0;
            let a802 = v5;
            nat_sub(a801, a802)
        };
        let a804 = v6;
        if nat_lt(a803, a804) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f48({
                let a805 = v0;
                let a806 = v5;
                nat_sub(a805, a806)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a809 = {
                    let a807 = v0;
                    let a808 = v5;
                    nat_sub(a807, a808)
                };
                let a810 = v6;
                nat_sub(a809, a810)
            };
            let v8: u64 = {
                let a814 = 1u64;
                let a815 = {
                    let a812 = 2u64;
                    let a813 = match v1.clone() {
                        Adt3::C0(h811, _, _, _, _, _, _, _) => {
                            h811
                        }
                    };
                    nat_mul(a812, a813)?
                };
                nat_add(a814, a815)?
            };
            let v9: u64 = {
                let a816 = v7;
                let a817 = v8;
                let a818 = 0u64;
                nat_quot(a816, a817, a818)
            };
            let v10: u64 = {
                let a819 = v7;
                let a820 = v8;
                let a821 = 0u64;
                nat_rem(a819, a820, a821)
            };
            let a822 = v10;
            let a823 = 0u64;
            if nat_eq(a822, a823) {
                Ok(Adt5::C4(19u64, 4u64, f48(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a827 = {
                    let a824 = v10;
                    let a825 = 2u64;
                    let a826 = 0u64;
                    nat_rem(a824, a825, a826)
                };
                let a828 = 1u64;
                if nat_eq(a827, a828) {
                    Ok(Adt5::C4(6u64, 0u64, f48({
                        let a837 = {
                            let a830 = v9;
                            let a831 = match v1.clone() {
                                Adt3::C0(h829, _, _, _, _, _, _, _) => {
                                    h829
                                }
                            };
                            nat_mul(a830, a831)?
                        };
                        let a838 = {
                            let a834 = {
                                let a832 = v10;
                                let a833 = 1u64;
                                nat_sub(a832, a833)
                            };
                            let a835 = 2u64;
                            let a836 = 0u64;
                            nat_quot(a834, a835, a836)
                        };
                        nat_add(a837, a838)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f48({
                        let a847 = {
                            let a840 = v4;
                            let a841 = match v1.clone() {
                                Adt3::C0(h839, _, _, _, _, _, _, _) => {
                                    h839
                                }
                            };
                            nat_mul(a840, a841)?
                        };
                        let a848 = {
                            let a844 = {
                                let a842 = v10;
                                let a843 = 1u64;
                                nat_sub(a842, a843)
                            };
                            let a845 = 2u64;
                            let a846 = 0u64;
                            nat_quot(a844, a845, a846)
                        };
                        nat_add(a847, a848)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f48({
                        let a857 = {
                            let a850 = v4;
                            let a851 = match v1 {
                                Adt3::C0(h849, _, _, _, _, _, _, _) => {
                                    h849
                                }
                            };
                            nat_mul(a850, a851)?
                        };
                        let a858 = {
                            let a854 = {
                                let a852 = v10;
                                let a853 = 1u64;
                                nat_sub(a852, a853)
                            };
                            let a855 = 2u64;
                            let a856 = 0u64;
                            nat_quot(a854, a855, a856)
                        };
                        nat_add(a857, a858)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f24(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a859 = v1;
    let a860 = f61(v2.clone(), v4.clone())?;
    if nat_lt(a859, a860) {
        let a863 = v1;
        let a864 = {
            let a861 = 2u64;
            let a862 = f45(v4.clone())?;
            nat_mul(a861, a862)?
        };
        if nat_lt(a863, a864) {
            let a868 = {
                let a865 = v1;
                let a866 = 2u64;
                let a867 = 0u64;
                nat_rem(a865, a866, a867)
            };
            let a869 = 0u64;
            if nat_eq(a868, a869) {
                f62({
                    let a870 = v1;
                    let a871 = 2u64;
                    let a872 = 0u64;
                    nat_quot(a870, a871, a872)
                }, v4)
            } else {
                if f63(match v2 {
                    Adt3::C0(_, h873, _, _, _, _, _, _) => {
                        h873
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a876 = v1;
                let a877 = {
                    let a874 = 2u64;
                    let a875 = f45(v4.clone())?;
                    nat_mul(a874, a875)?
                };
                nat_sub(a876, a877)
            };
            let v7: u64 = {
                let a881 = v6;
                let a882 = {
                    let a879 = 1u64;
                    let a880 = match v2.clone() {
                        Adt3::C0(h878, _, _, _, _, _, _, _) => {
                            h878
                        }
                    };
                    nat_add(a879, a880)?
                };
                let a883 = 0u64;
                nat_quot(a881, a882, a883)
            };
            let v8: u64 = {
                let a887 = v6;
                let a888 = {
                    let a885 = 1u64;
                    let a886 = match v2.clone() {
                        Adt3::C0(h884, _, _, _, _, _, _, _) => {
                            h884
                        }
                    };
                    nat_add(a885, a886)?
                };
                let a889 = 0u64;
                nat_rem(a887, a888, a889)
            };
            let a890 = v8;
            let a891 = 0u64;
            if nat_eq(a890, a891) {
                f62(v7, v4)
            } else {
                f64(v0, v7, {
                    let a892 = v8;
                    let a893 = 1u64;
                    nat_sub(a892, a893)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f25(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f65(match v0.clone() {
        Adt2::C0(h894, _, _, _, _, _, _, _, _, _, _) => {
            h894
        }
    }, match v0.clone() {
        Adt2::C0(_, h895, _, _, _, _, _, _, _, _, _) => {
            h895
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h896, _, _, _, _, _, _, _, _) => {
            h896
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h897, _, _, _, _, _, _, _) => {
            h897
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h898, _, _, _, _, _, _) => {
            h898
        }
    }, match v1.clone() {
        Adt2::C0(h899, _, _, _, _, _, _, _, _, _, _) => {
            h899
        }
    }, match v1.clone() {
        Adt2::C0(_, h900, _, _, _, _, _, _, _, _, _) => {
            h900
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h901, _, _, _, _, _, _, _, _) => {
            h901
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h902, _, _, _, _, _, _, _) => {
            h902
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h903, _, _, _, _, _, _) => {
            h903
        }
    }, match v2.clone() {
        Adt2::C0(h904, _, _, _, _, _, _, _, _, _, _) => {
            h904
        }
    }, match v2.clone() {
        Adt2::C0(_, h905, _, _, _, _, _, _, _, _, _) => {
            h905
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h906, _, _, _, _, _, _, _, _) => {
            h906
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h907, _, _, _, _, _, _, _) => {
            h907
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h908, _, _, _, _, _, _) => {
            h908
        }
    }) {
        let m910 = f32(match v0.clone() {
            Adt2::C0(h909, _, _, _, _, _, _, _, _, _, _) => {
                h909
            }
        });
        match m910 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                let a916 = {
                    let a913 = match v0.clone() {
                        Adt2::C0(_, h911, _, _, _, _, _, _, _, _, _) => {
                            h911
                        }
                    };
                    let a914 = match v3.clone() {
                        Adt8::C0(_, h912, _) => {
                            h912
                        }
                    };
                    let a915 = 0u64;
                    nat_rem(a913, a914, a915)
                };
                let a917 = 0u64;
                if if nat_eq(a916, a917) {
                    if f66(match v0.clone() {
                        Adt2::C0(_, h918, _, _, _, _, _, _, _, _, _) => {
                            h918
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h919, _, _, _, _, _, _, _, _, _) => {
                            h919
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h920, _, _, _, _, _, _, _, _) => {
                            h920
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h921, _, _, _, _, _, _, _) => {
                            h921
                        }
                    }) {
                        true
                    } else {
                        let a923 = 4u64;
                        let a924 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h922, _, _, _, _, _) => {
                                h922
                            }
                        };
                        if if nat_le(a923, a924) {
                            let a926 = match v3.clone() {
                                Adt8::C0(_, h925, _) => {
                                    h925
                                }
                            };
                            let a927 = 1u64;
                            if nat_eq(a926, a927) {
                                true
                            } else {
                                let a930 = match v3.clone() {
                                    Adt8::C0(_, _, h928) => {
                                        h928
                                    }
                                };
                                let a931 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h929, _, _, _, _, _) => {
                                        h929
                                    }
                                };
                                nat_le(a930, a931)
                            }
                        } else {
                            false
                        } {
                            if f29(v0.clone(), v2.clone())? {
                                f29(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f27(v0.clone(), match v3 {
                        Adt8::C0(_, h932, _) => {
                            h932
                        }
                    });
                    Ok(Adt6::C4(if f66(match v0.clone() {
                        Adt2::C0(_, h933, _, _, _, _, _, _, _, _, _) => {
                            h933
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h934, _, _, _, _, _, _, _, _, _) => {
                            h934
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h935, _, _, _, _, _, _, _, _) => {
                            h935
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h936, _, _, _, _, _, _, _) => {
                            h936
                        }
                    }) {
                        0u64
                    } else {
                        f61(v4.clone(), f27(v1.clone(), 1u64))?
                    }, 4u64, v4, f27(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h937, _, _, _, _, _, _, _, _, _, _) => {
                            h937
                        }
                    }, f27(v1, 1u64))))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a938 = v0;
    let a939 = 0u64;
    if if if nat_eq(a938, a939) {
        true
    } else {
        let a940 = v1;
        let a941 = 0u64;
        nat_eq(a940, a941)
    } {
        true
    } else {
        let a942 = v2;
        let a943 = 0u64;
        nat_eq(a942, a943)
    } {
        true
    } else {
        let a944 = v3;
        let a945 = 0u64;
        nat_eq(a944, a945)
    }
}

pub fn f27(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a947 = match v0.clone() {
            Adt2::C0(_, h946, _, _, _, _, _, _, _, _, _) => {
                h946
            }
        };
        let a948 = v1;
        let a949 = 0u64;
        nat_quot(a947, a948, a949)
    }, match v0.clone() {
        Adt2::C0(_, _, h950, _, _, _, _, _, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h951, _, _, _, _, _, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h952, _, _, _, _, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h953, _, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h954, _, _, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h955, _, _, _) => {
            h955
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h956, _, _) => {
            h956
        }
    })
}

pub fn f28(v0: Adt2, v1: u64) -> R<u64> {
    let a968 = {
        let a962 = {
            let a958 = match v0.clone() {
                Adt2::C0(_, h957, _, _, _, _, _, _, _, _, _) => {
                    h957
                }
            };
            let a959 = v1;
            let a960 = 0u64;
            nat_quot(a958, a959, a960)
        };
        let a963 = match v0.clone() {
            Adt2::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                h961
            }
        };
        nat_mul(a962, a963)?
    };
    let a969 = {
        let a966 = match v0.clone() {
            Adt2::C0(_, _, _, h964, _, _, _, _, _, _, _) => {
                h964
            }
        };
        let a967 = match v0 {
            Adt2::C0(_, _, _, _, h965, _, _, _, _, _, _) => {
                h965
            }
        };
        nat_mul(a966, a967)?
    };
    nat_mul(a968, a969)
}

pub fn f29(v0: Adt2, v1: Adt2) -> R<bool> {
    let a971 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h970, _) => {
            h970
        }
    };
    let a972 = 0u64;
    if nat_eq(a971, a972) {
        Ok(true)
    } else {
        let a974 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h973, _) => {
                h973
            }
        };
        let a975 = 0u64;
        if nat_eq(a974, a975) {
            Ok(true)
        } else {
            let a977 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h976) => {
                    h976
                }
            };
            let a978 = 0u64;
            if nat_eq(a977, a978) {
                Ok(true)
            } else {
                let a980 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h979) => {
                        h979
                    }
                };
                let a981 = 0u64;
                if nat_eq(a980, a981) {
                    Ok(true)
                } else {
                    let a987 = {
                        let a984 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h982) => {
                                h982
                            }
                        };
                        let a985 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h983, _) => {
                                h983
                            }
                        };
                        nat_add(a984, a985)?
                    };
                    let a988 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h986) => {
                            h986
                        }
                    };
                    if nat_le(a987, a988) {
                        Ok(true)
                    } else {
                        let a994 = {
                            let a991 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h989) => {
                                    h989
                                }
                            };
                            let a992 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h990, _) => {
                                    h990
                                }
                            };
                            nat_add(a991, a992)?
                        };
                        let a995 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h993) => {
                                h993
                            }
                        };
                        Ok(nat_le(a994, a995))
                    }
                }
            }
        }
    }
}

pub fn f30(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a998 = match v1.clone() {
        Adt2::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
            h996
        }
    };
    let a999 = match v3.clone() {
        Adt2::C0(h997, _, _, _, _, _, _, _, _, _, _) => {
            h997
        }
    };
    if if nat_eq(a998, a999) {
        let a1000 = v0;
        let a1001 = 33u64;
        if nat_eq(a1000, a1001) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1003 = f32(match v1.clone() {
            Adt2::C0(h1002, _, _, _, _, _, _, _, _, _, _) => {
                h1002
            }
        });
        match m1003 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h1004, _) => {
                        h1004
                    }
                };
                let v6: u64 = f28(v1.clone(), v5)?;
                let a1009 = {
                    let a1006 = match v1.clone() {
                        Adt2::C0(_, h1005, _, _, _, _, _, _, _, _, _) => {
                            h1005
                        }
                    };
                    let a1007 = v5;
                    let a1008 = 1u64;
                    nat_rem(a1006, a1007, a1008)
                };
                let a1010 = 0u64;
                if if if nat_eq(a1009, a1010) {
                    let a1015 = {
                        let a1012 = match v3.clone() {
                            Adt2::C0(_, h1011, _, _, _, _, _, _, _, _, _) => {
                                h1011
                            }
                        };
                        let a1013 = v5;
                        let a1014 = 1u64;
                        nat_rem(a1012, a1013, a1014)
                    };
                    let a1016 = 0u64;
                    nat_eq(a1015, a1016)
                } else {
                    false
                } {
                    let a1017 = v6;
                    let a1018 = f28(v3.clone(), v5)?;
                    nat_eq(a1017, a1018)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h1019) => {
                            h1019
                        }
                    }, f27(v1, v5), f27(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt6::C1)
                }
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f31(v0: u64) -> bool {
    let a1020 = v0;
    let a1021 = 0u64;
    if if nat_eq(a1020, a1021) {
        true
    } else {
        let a1022 = v0;
        let a1023 = 1u64;
        nat_eq(a1022, a1023)
    } {
        true
    } else {
        let a1024 = v0;
        let a1025 = 30u64;
        nat_eq(a1024, a1025)
    }
}

pub fn f32(v0: u64) -> Adt7 {
    let a1026 = v0;
    let a1027 = 0u64;
    if nat_eq(a1026, a1027) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a1028 = v0;
        let a1029 = 1u64;
        if nat_eq(a1028, a1029) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a1030 = v0;
            let a1031 = 2u64;
            if nat_eq(a1030, a1031) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a1032 = v0;
                let a1033 = 3u64;
                if nat_eq(a1032, a1033) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a1034 = v0;
                    let a1035 = 6u64;
                    if nat_eq(a1034, a1035) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1036 = v0;
                        let a1037 = 7u64;
                        if nat_eq(a1036, a1037) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1038 = v0;
                            let a1039 = 8u64;
                            if nat_eq(a1038, a1039) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1040 = v0;
                                let a1041 = 9u64;
                                if nat_eq(a1040, a1041) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1042 = v0;
                                    let a1043 = 10u64;
                                    if nat_eq(a1042, a1043) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1044 = v0;
                                        let a1045 = 11u64;
                                        if nat_eq(a1044, a1045) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1046 = v0;
                                            let a1047 = 12u64;
                                            if nat_eq(a1046, a1047) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1048 = v0;
                                                let a1049 = 13u64;
                                                if nat_eq(a1048, a1049) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1050 = v0;
                                                    let a1051 = 14u64;
                                                    if nat_eq(a1050, a1051) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1052 = v0;
                                                        let a1053 = 15u64;
                                                        if nat_eq(a1052, a1053) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1054 = v0;
                                                            let a1055 = 16u64;
                                                            if nat_eq(a1054, a1055) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1056 = v0;
                                                                let a1057 = 17u64;
                                                                if nat_eq(a1056, a1057) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1058 = v0;
                                                                    let a1059 = 18u64;
                                                                    if nat_eq(a1058, a1059) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1060 = v0;
                                                                        let a1061 = 19u64;
                                                                        if nat_eq(a1060, a1061) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1062 = v0;
                                                                            let a1063 = 20u64;
                                                                            if nat_eq(a1062, a1063) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1064 = v0;
                                                                                let a1065 = 21u64;
                                                                                if nat_eq(a1064, a1065) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1066 = v0;
                                                                                    let a1067 = 22u64;
                                                                                    if nat_eq(a1066, a1067) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1068 = v0;
                                                                                        let a1069 = 23u64;
                                                                                        if nat_eq(a1068, a1069) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1070 = v0;
                                                                                            let a1071 = 24u64;
                                                                                            if nat_eq(a1070, a1071) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1072 = v0;
                                                                                                let a1073 = 25u64;
                                                                                                if nat_eq(a1072, a1073) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1074 = v0;
                                                                                                    let a1075 = 26u64;
                                                                                                    if nat_eq(a1074, a1075) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1076 = v0;
                                                                                                        let a1077 = 27u64;
                                                                                                        if nat_eq(a1076, a1077) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1078 = v0;
                                                                                                            let a1079 = 28u64;
                                                                                                            if nat_eq(a1078, a1079) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1080 = v0;
                                                                                                                let a1081 = 29u64;
                                                                                                                if nat_eq(a1080, a1081) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1082 = v0;
                                                                                                                    let a1083 = 30u64;
                                                                                                                    if nat_eq(a1082, a1083) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1084 = v0;
                                                                                                                        let a1085 = 34u64;
                                                                                                                        if nat_eq(a1084, a1085) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1086 = v0;
                                                                                                                            let a1087 = 35u64;
                                                                                                                            if nat_eq(a1086, a1087) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1088 = v0;
                                                                                                                                let a1089 = 39u64;
                                                                                                                                if nat_eq(a1088, a1089) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1090 = v0;
                                                                                                                                    let a1091 = 40u64;
                                                                                                                                    if nat_eq(a1090, a1091) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1092 = v0;
                                                                                                                                        let a1093 = 41u64;
                                                                                                                                        if nat_eq(a1092, a1093) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1094 = v0;
                                                                                                                                            let a1095 = 42u64;
                                                                                                                                            if nat_eq(a1094, a1095) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1096 = v0;
                                                                                                                                                let a1097 = 43u64;
                                                                                                                                                if nat_eq(a1096, a1097) {
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

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f26(v4, v5, v6, v7) {
        f26(v0, v1, v2, v3)
    } else {
        let a1101 = {
            let a1098 = v0;
            let a1099 = v4;
            let a1100 = 1u64;
            nat_rem(a1098, a1099, a1100)
        };
        let a1102 = 0u64;
        if if if nat_eq(a1101, a1102) {
            let a1106 = {
                let a1103 = v1;
                let a1104 = v5;
                let a1105 = 1u64;
                nat_rem(a1103, a1104, a1105)
            };
            let a1107 = 0u64;
            nat_eq(a1106, a1107)
        } else {
            false
        } {
            let a1111 = {
                let a1108 = v2;
                let a1109 = v6;
                let a1110 = 1u64;
                nat_rem(a1108, a1109, a1110)
            };
            let a1112 = 0u64;
            nat_eq(a1111, a1112)
        } else {
            false
        } {
            let a1116 = {
                let a1113 = v3;
                let a1114 = v7;
                let a1115 = 1u64;
                nat_rem(a1113, a1114, a1115)
            };
            let a1117 = 0u64;
            nat_eq(a1116, a1117)
        } else {
            false
        }
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1118 = v0;
    let a1119 = v1;
    if nat_eq(a1118, a1119) {
        let a1122 = v4;
        let a1123 = {
            let a1120 = v2;
            let a1121 = v3;
            nat_add(a1120, a1121)?
        };
        Ok(nat_eq(a1122, a1123))
    } else {
        let a1124 = v2;
        let a1125 = v3;
        if nat_eq(a1124, a1125) {
            let a1126 = v4;
            let a1127 = v2;
            Ok(nat_eq(a1126, a1127))
        } else {
            Ok(false)
        }
    }
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1130 = {
            let a1128 = v0;
            let a1129 = f67(v1);
            nat_add(a1128, a1129)?
        };
        let a1131 = f67(v2);
        nat_add(a1130, a1131)?
    };
    let v5: u64 = {
        let a1132 = f68(v1);
        let a1133 = f68(v2);
        nat_add(a1132, a1133)?
    };
    let a1134 = v1;
    let a1135 = 4294967295u64;
    if if nat_le(a1134, a1135) {
        let a1136 = v2;
        let a1137 = 4294967295u64;
        nat_le(a1136, a1137)
    } else {
        false
    } {
        let a1138 = v5;
        let a1139 = v4;
        if nat_le(a1138, a1139) {
            let a1142 = v3;
            let a1143 = {
                let a1140 = v4;
                let a1141 = v5;
                nat_sub(a1140, a1141)
            };
            Ok(nat_eq(a1142, a1143))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f36(v0: u64, v1: u64) -> R<bool> {
    let a1144 = v1;
    let a1145 = 4294967295u64;
    if nat_le(a1144, a1145) {
        let a1148 = {
            let a1146 = f67(v1);
            let a1147 = f68(v1);
            nat_add(a1146, a1147)?
        };
        let a1149 = v0;
        Ok(nat_lt(a1148, a1149))
    } else {
        Ok(false)
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> bool {
    let a1152 = match v0.clone() {
        Adt2::C0(_, h1150, _, _, _, _, _, _, _, _, _) => {
            h1150
        }
    };
    let a1153 = match v1.clone() {
        Adt2::C0(_, h1151, _, _, _, _, _, _, _, _, _) => {
            h1151
        }
    };
    if if if nat_eq(a1152, a1153) {
        let a1156 = match v0.clone() {
            Adt2::C0(_, _, h1154, _, _, _, _, _, _, _, _) => {
                h1154
            }
        };
        let a1157 = match v1.clone() {
            Adt2::C0(_, _, h1155, _, _, _, _, _, _, _, _) => {
                h1155
            }
        };
        nat_eq(a1156, a1157)
    } else {
        false
    } {
        let a1160 = match v0.clone() {
            Adt2::C0(_, _, _, h1158, _, _, _, _, _, _, _) => {
                h1158
            }
        };
        let a1161 = match v1.clone() {
            Adt2::C0(_, _, _, h1159, _, _, _, _, _, _, _) => {
                h1159
            }
        };
        nat_eq(a1160, a1161)
    } else {
        false
    } {
        let a1164 = match v0 {
            Adt2::C0(_, _, _, _, h1162, _, _, _, _, _, _) => {
                h1162
            }
        };
        let a1165 = match v1 {
            Adt2::C0(_, _, _, _, h1163, _, _, _, _, _, _) => {
                h1163
            }
        };
        nat_eq(a1164, a1165)
    } else {
        false
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let a1167 = match v0.clone() {
        Adt2::C0(_, h1166, _, _, _, _, _, _, _, _, _) => {
            h1166
        }
    };
    let a1168 = 1u64;
    if if if if nat_eq(a1167, a1168) {
        true
    } else {
        let a1170 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1169, _, _, _, _, _) => {
                h1169
            }
        };
        let a1171 = 4u64;
        nat_eq(a1170, a1171)
    } {
        let a1173 = match v0.clone() {
            Adt2::C0(_, _, h1172, _, _, _, _, _, _, _, _) => {
                h1172
            }
        };
        let a1174 = 1u64;
        if nat_eq(a1173, a1174) {
            true
        } else {
            let a1179 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1175, _, _, _, _) => {
                    h1175
                }
            };
            let a1180 = {
                let a1177 = match v0.clone() {
                    Adt2::C0(_, h1176, _, _, _, _, _, _, _, _, _) => {
                        h1176
                    }
                };
                let a1178 = 4u64;
                nat_mul(a1177, a1178)?
            };
            nat_eq(a1179, a1180)
        }
    } else {
        false
    } {
        let a1182 = match v0.clone() {
            Adt2::C0(_, _, _, h1181, _, _, _, _, _, _, _) => {
                h1181
            }
        };
        let a1183 = 1u64;
        if nat_eq(a1182, a1183) {
            true
        } else {
            let a1191 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1184, _, _, _) => {
                    h1184
                }
            };
            let a1192 = {
                let a1189 = {
                    let a1187 = match v0.clone() {
                        Adt2::C0(_, h1185, _, _, _, _, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    let a1188 = match v0.clone() {
                        Adt2::C0(_, _, h1186, _, _, _, _, _, _, _, _) => {
                            h1186
                        }
                    };
                    nat_mul(a1187, a1188)?
                };
                let a1190 = 4u64;
                nat_mul(a1189, a1190)?
            };
            nat_eq(a1191, a1192)
        }
    } else {
        false
    } {
        let a1194 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1193, _, _, _, _, _, _) => {
                h1193
            }
        };
        let a1195 = 1u64;
        if nat_eq(a1194, a1195) {
            Ok(true)
        } else {
            let a1206 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1196, _, _) => {
                    h1196
                }
            };
            let a1207 = {
                let a1204 = {
                    let a1202 = {
                        let a1199 = match v0.clone() {
                            Adt2::C0(_, h1197, _, _, _, _, _, _, _, _, _) => {
                                h1197
                            }
                        };
                        let a1200 = match v0.clone() {
                            Adt2::C0(_, _, h1198, _, _, _, _, _, _, _, _) => {
                                h1198
                            }
                        };
                        nat_mul(a1199, a1200)?
                    };
                    let a1203 = match v0 {
                        Adt2::C0(_, _, _, h1201, _, _, _, _, _, _, _) => {
                            h1201
                        }
                    };
                    nat_mul(a1202, a1203)?
                };
                let a1205 = 4u64;
                nat_mul(a1204, a1205)?
            };
            Ok(nat_eq(a1206, a1207))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f69(v0.clone(), false)?;
    let v6: u64 = f69(v0, true)?;
    let a1208 = v6;
    let a1209 = v1;
    if if nat_le(a1208, a1209) {
        let a1216 = {
            let a1214 = {
                let a1210 = v1;
                let a1211 = v5;
                nat_add(a1210, a1211)?
            };
            let a1215 = {
                let a1212 = v3;
                let a1213 = 0u64;
                if nat_eq(a1212, a1213) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1214, a1215)?
        };
        let a1217 = v2;
        nat_le(a1216, a1217)
    } else {
        false
    } {
        if v4 {
            let a1222 = {
                let a1220 = {
                    let a1218 = v1;
                    let a1219 = v5;
                    nat_add(a1218, a1219)?
                };
                let a1221 = v6;
                nat_sub(a1220, a1221)
            };
            let a1223 = v2;
            Ok(nat_lt(a1222, a1223))
        } else {
            let a1228 = {
                let a1226 = {
                    let a1224 = v1;
                    let a1225 = v5;
                    nat_add(a1224, a1225)?
                };
                let a1227 = v6;
                nat_sub(a1226, a1227)
            };
            let a1229 = v2;
            Ok(nat_le(a1228, a1229))
        }
    } else {
        Ok(false)
    }
}

pub fn f40(v0: u64, v1: u64) -> bool {
    let a1230 = v0;
    let a1231 = 0u64;
    if if nat_eq(a1230, a1231) {
        true
    } else {
        let a1232 = v0;
        let a1233 = 1u64;
        nat_eq(a1232, a1233)
    } {
        let a1234 = v1;
        let a1235 = 4294967296u64;
        nat_lt(a1234, a1235)
    } else {
        false
    }
}

pub fn f41(v0: u64, v1: u64) -> R<u64> {
    let a1240 = {
        let a1238 = v0;
        let a1239 = {
            let a1236 = v1;
            let a1237 = 1u64;
            nat_sub(a1236, a1237)
        };
        nat_add(a1238, a1239)?
    };
    let a1241 = v1;
    let a1242 = 0u64;
    Ok(nat_quot(a1240, a1241, a1242))
}

pub fn f42(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f26(match v0.clone() {
        Adt2::C0(_, h1243, _, _, _, _, _, _, _, _, _) => {
            h1243
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1244, _, _, _, _, _, _, _, _) => {
            h1244
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1245, _, _, _, _, _, _, _) => {
            h1245
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1246, _, _, _, _, _, _) => {
            h1246
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1249 = match v1.clone() {
                Adt2::C0(_, h1247, _, _, _, _, _, _, _, _, _) => {
                    h1247
                }
            };
            let a1250 = match v1.clone() {
                Adt2::C0(_, _, h1248, _, _, _, _, _, _, _, _) => {
                    h1248
                }
            };
            nat_mul(a1249, a1250)?
        };
        let v5: u64 = {
            let a1252 = v4;
            let a1253 = match v1.clone() {
                Adt2::C0(_, _, _, h1251, _, _, _, _, _, _, _) => {
                    h1251
                }
            };
            nat_mul(a1252, a1253)?
        };
        let a1266 = {
            let a1264 = {
                let a1255 = match v0.clone() {
                    Adt2::C0(_, h1254, _, _, _, _, _, _, _, _, _) => {
                        h1254
                    }
                };
                let a1256 = 1u64;
                nat_sub(a1255, a1256)
            };
            let a1265 = {
                let a1262 = f70(match v0.clone() {
                    Adt2::C0(_, _, h1257, _, _, _, _, _, _, _, _) => {
                        h1257
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1258, _, _, _, _, _, _, _, _, _) => {
                        h1258
                    }
                })?;
                let a1263 = f70(match v0 {
                    Adt2::C0(_, _, _, h1259, _, _, _, _, _, _, _) => {
                        h1259
                    }
                }, v2, {
                    let a1260 = v3;
                    let a1261 = v5;
                    nat_mul(a1260, a1261)?
                }, v4)?;
                nat_add(a1262, a1263)?
            };
            nat_add(a1264, a1265)?
        };
        let a1267 = f28(v1, 1u64)?;
        Ok(nat_lt(a1266, a1267))
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1268 = v0;
    let a1269 = 2147483648u64;
    if if if if nat_lt(a1268, a1269) {
        let a1270 = v1;
        let a1271 = 2147483648u64;
        nat_lt(a1270, a1271)
    } else {
        false
    } {
        let a1274 = {
            let a1272 = v0;
            let a1273 = v1;
            nat_mul(a1272, a1273)?
        };
        let a1275 = 2147483648u64;
        nat_lt(a1274, a1275)
    } else {
        false
    } {
        let a1280 = {
            let a1278 = {
                let a1276 = v0;
                let a1277 = 1u64;
                nat_sub(a1276, a1277)
            };
            let a1279 = v2;
            nat_mul(a1278, a1279)?
        };
        let a1281 = 2147483648u64;
        nat_lt(a1280, a1281)
    } else {
        false
    } {
        let a1286 = {
            let a1284 = {
                let a1282 = v1;
                let a1283 = 1u64;
                nat_sub(a1282, a1283)
            };
            let a1285 = v2;
            nat_mul(a1284, a1285)?
        };
        let a1287 = 2147483648u64;
        Ok(nat_lt(a1286, a1287))
    } else {
        Ok(false)
    }
}

pub fn f44(v0: u64) -> R<u64> {
    let a1290 = 3u64;
    let a1291 = {
        let a1288 = v0;
        let a1289 = 3u64;
        nat_mul(a1288, a1289)?
    };
    nat_add(a1290, a1291)
}

pub fn f45(v0: Adt3) -> R<u64> {
    if f26(match v0.clone() {
        Adt3::C0(h1292, _, _, _, _, _, _, _) => {
            h1292
        }
    }, match v0.clone() {
        Adt3::C0(_, h1293, _, _, _, _, _, _) => {
            h1293
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1294, _, _, _, _, _) => {
            h1294
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1295, _, _, _, _) => {
            h1295
        }
    }) {
        Ok(0u64)
    } else {
        let a1304 = match v0.clone() {
            Adt3::C0(h1296, _, _, _, _, _, _, _) => {
                h1296
            }
        };
        let a1305 = {
            let a1302 = match v0.clone() {
                Adt3::C0(_, h1297, _, _, _, _, _, _) => {
                    h1297
                }
            };
            let a1303 = {
                let a1300 = match v0.clone() {
                    Adt3::C0(_, _, h1298, _, _, _, _, _) => {
                        h1298
                    }
                };
                let a1301 = match v0 {
                    Adt3::C0(_, _, _, h1299, _, _, _, _) => {
                        h1299
                    }
                };
                nat_mul(a1300, a1301)?
            };
            nat_mul(a1302, a1303)?
        };
        nat_mul(a1304, a1305)
    }
}

pub fn f46(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1306 = v1;
    let a1307 = 13u64;
    if if nat_eq(a1306, a1307) {
        f26(match v0.clone() {
            Adt3::C0(h1308, _, _, _, _, _, _, _) => {
                h1308
            }
        }, match v0.clone() {
            Adt3::C0(_, h1309, _, _, _, _, _, _) => {
                h1309
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1310, _, _, _, _, _) => {
                h1310
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1311, _, _, _, _) => {
                h1311
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1317 = match v0.clone() {
                Adt3::C0(_, h1312, _, _, _, _, _, _) => {
                    h1312
                }
            };
            let a1318 = {
                let a1315 = match v0.clone() {
                    Adt3::C0(_, _, h1313, _, _, _, _, _) => {
                        h1313
                    }
                };
                let a1316 = match v0.clone() {
                    Adt3::C0(_, _, _, h1314, _, _, _, _) => {
                        h1314
                    }
                };
                nat_mul(a1315, a1316)?
            };
            nat_mul(a1317, a1318)?
        };
        let a1319 = v1;
        let a1320 = 13u64;
        if nat_eq(a1319, a1320) {
            let a1329 = 2u64;
            let a1330 = if f71(v0.clone(), v2) {
                let a1322 = v3;
                let a1323 = f72(match v0 {
                    Adt3::C0(h1321, _, _, _, _, _, _, _) => {
                        h1321
                    }
                }, false)?;
                nat_mul(a1322, a1323)?
            } else {
                let a1327 = v3;
                let a1328 = {
                    let a1325 = match v0 {
                        Adt3::C0(h1324, _, _, _, _, _, _, _) => {
                            h1324
                        }
                    };
                    let a1326 = 2u64;
                    nat_mul(a1325, a1326)?
                };
                nat_mul(a1327, a1328)?
            };
            nat_add(a1329, a1330)
        } else {
            let a1334 = v3;
            let a1335 = f72(match v0 {
                Adt3::C0(h1331, _, _, _, _, _, _, _) => {
                    h1331
                }
            }, {
                let a1332 = v1;
                let a1333 = 16u64;
                nat_eq(a1332, a1333)
            })?;
            nat_mul(a1334, a1335)
        }
    }
}

pub fn f47(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1336 = v3;
    match m1336 {
        None => {
            f73(v0, v1, match v2.clone() {
                Adt2::C0(h1337, _, _, _, _, _, _, _, _, _, _) => {
                    h1337
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1338, _, _, _, _, _, _, _, _, _, _) => {
                    h1338
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1339, _, _, _, _, _, _, _, _, _) => {
                    h1339
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1340, _, _, _, _, _, _, _, _) => {
                    h1340
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1341, _, _, _, _, _, _, _) => {
                    h1341
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1342, _, _, _, _, _, _) => {
                    h1342
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1343, _, _, _, _, _, _, _, _, _) => {
                    h1343
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1344, _, _, _, _, _, _, _, _) => {
                    h1344
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1345, _, _, _, _, _, _, _) => {
                    h1345
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1346, _, _, _, _, _, _) => {
                    h1346
                }
            })
        }
        Some(v5) => {
            f73(v0, v1, match v2.clone() {
                Adt2::C0(h1347, _, _, _, _, _, _, _, _, _, _) => {
                    h1347
                }
            }, match v5.clone() {
                Adt2::C0(h1348, _, _, _, _, _, _, _, _, _, _) => {
                    h1348
                }
            }, match v4.clone() {
                Adt2::C0(h1349, _, _, _, _, _, _, _, _, _, _) => {
                    h1349
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h1350, _, _, _, _, _, _, _, _, _) => {
                    h1350
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1351, _, _, _, _, _, _, _, _) => {
                    h1351
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1352, _, _, _, _, _, _, _) => {
                    h1352
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1353, _, _, _, _, _, _) => {
                    h1353
                }
            }, match v5.clone() {
                Adt2::C0(_, h1354, _, _, _, _, _, _, _, _, _) => {
                    h1354
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1355, _, _, _, _, _, _, _, _) => {
                    h1355
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1356, _, _, _, _, _, _, _) => {
                    h1356
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h1357, _, _, _, _, _, _) => {
                    h1357
                }
            }, match v4.clone() {
                Adt2::C0(_, h1358, _, _, _, _, _, _, _, _, _) => {
                    h1358
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1359, _, _, _, _, _, _, _, _) => {
                    h1359
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1360, _, _, _, _, _, _, _) => {
                    h1360
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h1361, _, _, _, _, _, _) => {
                    h1361
                }
            })
        }
    }
}

pub fn f48(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1363 = v0;
        let a1364 = match v1.clone() {
            Adt3::C0(h1362, _, _, _, _, _, _, _) => {
                h1362
            }
        };
        let a1365 = 0u64;
        nat_quot(a1363, a1364, a1365)
    };
    let v3: u64 = {
        let a1367 = v2;
        let a1368 = match v1.clone() {
            Adt3::C0(_, h1366, _, _, _, _, _, _) => {
                h1366
            }
        };
        let a1369 = 0u64;
        nat_quot(a1367, a1368, a1369)
    };
    f74({
        let a1371 = v0;
        let a1372 = match v1.clone() {
            Adt3::C0(h1370, _, _, _, _, _, _, _) => {
                h1370
            }
        };
        let a1373 = 0u64;
        nat_rem(a1371, a1372, a1373)
    }, {
        let a1375 = v2;
        let a1376 = match v1.clone() {
            Adt3::C0(_, h1374, _, _, _, _, _, _) => {
                h1374
            }
        };
        let a1377 = 0u64;
        nat_rem(a1375, a1376, a1377)
    }, {
        let a1379 = v3;
        let a1380 = match v1.clone() {
            Adt3::C0(_, _, h1378, _, _, _, _, _) => {
                h1378
            }
        };
        let a1381 = 0u64;
        nat_rem(a1379, a1380, a1381)
    }, {
        let a1383 = v3;
        let a1384 = match v1.clone() {
            Adt3::C0(_, _, h1382, _, _, _, _, _) => {
                h1382
            }
        };
        let a1385 = 0u64;
        nat_quot(a1383, a1384, a1385)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1386, _, _, _) => {
            h1386
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1387, _, _) => {
            h1387
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1388, _) => {
            h1388
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1389) => {
            h1389
        }
    })
}

pub fn f49(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1391 = v0;
        let a1392 = match v2.clone() {
            Adt3::C0(h1390, _, _, _, _, _, _, _) => {
                h1390
            }
        };
        let a1393 = 0u64;
        nat_quot(a1391, a1392, a1393)
    };
    let v4: u64 = {
        let a1395 = v3;
        let a1396 = match v2.clone() {
            Adt3::C0(_, h1394, _, _, _, _, _, _) => {
                h1394
            }
        };
        let a1397 = 0u64;
        nat_quot(a1395, a1396, a1397)
    };
    f75({
        let a1399 = v0;
        let a1400 = match v2.clone() {
            Adt3::C0(h1398, _, _, _, _, _, _, _) => {
                h1398
            }
        };
        let a1401 = 0u64;
        nat_rem(a1399, a1400, a1401)
    }, {
        let a1403 = v3;
        let a1404 = match v2.clone() {
            Adt3::C0(_, h1402, _, _, _, _, _, _) => {
                h1402
            }
        };
        let a1405 = 0u64;
        nat_rem(a1403, a1404, a1405)
    }, {
        let a1407 = v4;
        let a1408 = match v2.clone() {
            Adt3::C0(_, _, h1406, _, _, _, _, _) => {
                h1406
            }
        };
        let a1409 = 0u64;
        nat_rem(a1407, a1408, a1409)
    }, {
        let a1411 = v4;
        let a1412 = match v2 {
            Adt3::C0(_, _, h1410, _, _, _, _, _) => {
                h1410
            }
        };
        let a1413 = 0u64;
        nat_quot(a1411, a1412, a1413)
    }, match v1.clone() {
        Adt3::C0(h1414, _, _, _, _, _, _, _) => {
            h1414
        }
    }, match v1.clone() {
        Adt3::C0(_, h1415, _, _, _, _, _, _) => {
            h1415
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1416, _, _, _, _, _) => {
            h1416
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1417, _, _, _, _) => {
            h1417
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1418, _, _, _) => {
            h1418
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1419, _, _) => {
            h1419
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1420, _) => {
            h1420
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1421) => {
            h1421
        }
    })
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1423 = v0;
        let a1424 = match v3.clone() {
            Adt3::C0(h1422, _, _, _, _, _, _, _) => {
                h1422
            }
        };
        let a1425 = 0u64;
        nat_quot(a1423, a1424, a1425)
    };
    let v6: u64 = {
        let a1427 = v5;
        let a1428 = match v3.clone() {
            Adt3::C0(_, h1426, _, _, _, _, _, _) => {
                h1426
            }
        };
        let a1429 = 0u64;
        nat_quot(a1427, a1428, a1429)
    };
    let v7: u64 = {
        let a1431 = v0;
        let a1432 = match v3.clone() {
            Adt3::C0(h1430, _, _, _, _, _, _, _) => {
                h1430
            }
        };
        let a1433 = 0u64;
        nat_rem(a1431, a1432, a1433)
    };
    let v8: u64 = {
        let a1435 = v5;
        let a1436 = match v3.clone() {
            Adt3::C0(_, h1434, _, _, _, _, _, _) => {
                h1434
            }
        };
        let a1437 = 0u64;
        nat_rem(a1435, a1436, a1437)
    };
    let v9: u64 = {
        let a1439 = v6;
        let a1440 = match v3.clone() {
            Adt3::C0(_, _, h1438, _, _, _, _, _) => {
                h1438
            }
        };
        let a1441 = 0u64;
        nat_rem(a1439, a1440, a1441)
    };
    let v10: u64 = {
        let a1443 = v6;
        let a1444 = match v3.clone() {
            Adt3::C0(_, _, h1442, _, _, _, _, _) => {
                h1442
            }
        };
        let a1445 = 0u64;
        nat_quot(a1443, a1444, a1445)
    };
    let v11: u64 = {
        let a1446 = v4;
        let a1447 = 0u64;
        if nat_eq(a1446, a1447) {
            match v1.clone() {
                Adt3::C0(h1448, _, _, _, _, _, _, _) => {
                    h1448
                }
            }
        } else {
            let a1449 = v4;
            let a1450 = 1u64;
            if nat_eq(a1449, a1450) {
                match v1.clone() {
                    Adt3::C0(_, h1451, _, _, _, _, _, _) => {
                        h1451
                    }
                }
            } else {
                let a1452 = v4;
                let a1453 = 2u64;
                if nat_eq(a1452, a1453) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1454, _, _, _, _, _) => {
                            h1454
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1455, _, _, _, _) => {
                            h1455
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1462 = v11;
        let a1463 = {
            let a1456 = v4;
            let a1457 = 0u64;
            if nat_eq(a1456, a1457) {
                v7
            } else {
                let a1458 = v4;
                let a1459 = 1u64;
                if nat_eq(a1458, a1459) {
                    v8
                } else {
                    let a1460 = v4;
                    let a1461 = 2u64;
                    if nat_eq(a1460, a1461) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1462, a1463)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f74(if if v12 {
        let a1464 = v4;
        let a1465 = 0u64;
        nat_eq(a1464, a1465)
    } else {
        false
    } {
        let a1466 = v7;
        let a1467 = v11;
        nat_sub(a1466, a1467)
    } else {
        v7
    }, if if v12 {
        let a1468 = v4;
        let a1469 = 1u64;
        nat_eq(a1468, a1469)
    } else {
        false
    } {
        let a1470 = v8;
        let a1471 = v11;
        nat_sub(a1470, a1471)
    } else {
        v8
    }, if if v12 {
        let a1472 = v4;
        let a1473 = 2u64;
        nat_eq(a1472, a1473)
    } else {
        false
    } {
        let a1474 = v9;
        let a1475 = v11;
        nat_sub(a1474, a1475)
    } else {
        v9
    }, if if v12 {
        let a1476 = v4;
        let a1477 = 3u64;
        nat_eq(a1476, a1477)
    } else {
        false
    } {
        let a1478 = v10;
        let a1479 = v11;
        nat_sub(a1478, a1479)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1480, _, _, _) => {
            h1480
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1481, _, _) => {
            h1481
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1482, _) => {
            h1482
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1483) => {
            h1483
        }
    })?, f48(v0, v3)?))
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f76(v0, v2.clone(), 0u64);
    let v9: u64 = f76(v0, v2.clone(), 1u64);
    let v10: u64 = f76(v0, v2.clone(), 2u64);
    let v11: u64 = f76(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f77(v1.clone(), f78(v8, v3, match v1.clone() {
            Adt3::C0(h1484, _, _, _, _, _, _, _) => {
                h1484
            }
        })?, f78(v9, v4, match v1.clone() {
            Adt3::C0(_, h1485, _, _, _, _, _, _) => {
                h1485
            }
        })?, f78(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1486, _, _, _, _, _) => {
                h1486
            }
        })?, f78(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1487, _, _, _, _) => {
                h1487
            }
        })?)?, f48(v0, v2)?))
    } else {
        if if if if f79(v8, v3, match v1.clone() {
            Adt3::C0(h1488, _, _, _, _, _, _, _) => {
                h1488
            }
        })? {
            f79(v9, v4, match v1.clone() {
                Adt3::C0(_, h1489, _, _, _, _, _, _) => {
                    h1489
                }
            })?
        } else {
            false
        } {
            f79(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1490, _, _, _, _, _) => {
                    h1490
                }
            })?
        } else {
            false
        } {
            f79(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1491, _, _, _, _) => {
                    h1491
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f77(v1, f80(v8, v3)?, f80(v9, v4)?, f80(v10, v5)?, f80(v11, v6)?)?, f48(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f48(v0, v2)?))
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f76(v0, v2.clone(), 0u64);
    let v5: u64 = f76(v0, v2.clone(), 1u64);
    let v6: u64 = f76(v0, v2.clone(), 2u64);
    let v7: u64 = f76(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f77(v1.clone(), f81(v4, v3, match v1 {
        Adt3::C0(h1492, _, _, _, _, _, _, _) => {
            h1492
        }
    })?, v5, v6, v7)?, f48(v0, v2)?))
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1493 = v0;
    let a1494 = v3;
    if nat_lt(a1493, a1494) {
        Ok(Adt5::C1(false, {
            let a1495 = v0;
            let a1496 = 4u64;
            nat_mul(a1495, a1496)?
        }, {
            let a1497 = v0;
            let a1498 = 4u64;
            nat_mul(a1497, a1498)?
        }))
    } else {
        let v6: u64 = {
            let a1499 = v0;
            let a1500 = v3;
            nat_sub(a1499, a1500)
        };
        let v7: u64 = f48(v6, v1)?;
        let v8: u64 = f82(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1501 = v0;
    let a1502 = v3;
    if nat_lt(a1501, a1502) {
        Ok(Adt5::C1(true, 0u64, f48(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1503 = v0;
            let a1504 = v3;
            nat_sub(a1503, a1504)
        };
        let v5: u64 = f49(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f48(v4, v1)?, 3u64, v5))
    }
}

pub fn f55(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1506 = v1;
        let a1507 = match v3.clone() {
            Adt3::C0(h1505, _, _, _, _, _, _, _) => {
                h1505
            }
        };
        let a1508 = 0u64;
        nat_rem(a1506, a1507, a1508)
    };
    let v5: u64 = {
        let a1514 = {
            let a1510 = v1;
            let a1511 = match v3.clone() {
                Adt3::C0(h1509, _, _, _, _, _, _, _) => {
                    h1509
                }
            };
            let a1512 = 0u64;
            nat_quot(a1510, a1511, a1512)
        };
        let a1515 = match v3.clone() {
            Adt3::C0(_, h1513, _, _, _, _, _, _) => {
                h1513
            }
        };
        let a1516 = 0u64;
        nat_rem(a1514, a1515, a1516)
    };
    let v6: bool = f83(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f49(v1, v2, v3.clone())?
    } else {
        0u64
    }, f48(v1, v3)?))
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1517 = v3;
        let a1518 = 8u64;
        nat_add(a1517, a1518)?
    }, 0u64, f48(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f49(v0, v5, v2.clone())?
    }, 3u64, f48(v0, v2)?))
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1520 = v0;
        let a1521 = match v2.clone() {
            Adt3::C0(h1519, _, _, _, _, _, _, _) => {
                h1519
            }
        };
        let a1522 = 0u64;
        nat_rem(a1520, a1521, a1522)
    };
    let v7: u64 = {
        let a1524 = v0;
        let a1525 = match v2.clone() {
            Adt3::C0(h1523, _, _, _, _, _, _, _) => {
                h1523
            }
        };
        let a1526 = 0u64;
        nat_quot(a1524, a1525, a1526)
    };
    let v8: u64 = {
        let a1528 = v7;
        let a1529 = match v2.clone() {
            Adt3::C0(_, h1527, _, _, _, _, _, _) => {
                h1527
            }
        };
        let a1530 = 0u64;
        nat_rem(a1528, a1529, a1530)
    };
    let v9: u64 = {
        let a1532 = v7;
        let a1533 = match v2.clone() {
            Adt3::C0(_, h1531, _, _, _, _, _, _) => {
                h1531
            }
        };
        let a1534 = 0u64;
        nat_quot(a1532, a1533, a1534)
    };
    let v10: u64 = {
        let a1536 = v9;
        let a1537 = match v2.clone() {
            Adt3::C0(_, _, h1535, _, _, _, _, _) => {
                h1535
            }
        };
        let a1538 = 0u64;
        nat_rem(a1536, a1537, a1538)
    };
    let v11: u64 = {
        let a1540 = v9;
        let a1541 = match v2.clone() {
            Adt3::C0(_, _, h1539, _, _, _, _, _) => {
                h1539
            }
        };
        let a1542 = 0u64;
        nat_quot(a1540, a1541, a1542)
    };
    let v12: u64 = f48(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1551 = {
                let a1546 = {
                    let a1543 = v10;
                    let a1544 = v4;
                    let a1545 = 0u64;
                    nat_quot(a1543, a1544, a1545)
                };
                let a1547 = v5;
                nat_mul(a1546, a1547)?
            };
            let a1552 = {
                let a1548 = v8;
                let a1549 = v4;
                let a1550 = 0u64;
                nat_quot(a1548, a1549, a1550)
            };
            nat_add(a1551, a1552)?
        };
        let v14: u64 = {
            let a1572 = v6;
            let a1573 = {
                let a1570 = match v1.clone() {
                    Adt3::C0(h1553, _, _, _, _, _, _, _) => {
                        h1553
                    }
                };
                let a1571 = {
                    let a1568 = {
                        let a1554 = v8;
                        let a1555 = v4;
                        let a1556 = 0u64;
                        nat_rem(a1554, a1555, a1556)
                    };
                    let a1569 = {
                        let a1566 = match v1.clone() {
                            Adt3::C0(_, h1557, _, _, _, _, _, _) => {
                                h1557
                            }
                        };
                        let a1567 = {
                            let a1564 = {
                                let a1558 = v10;
                                let a1559 = v4;
                                let a1560 = 0u64;
                                nat_rem(a1558, a1559, a1560)
                            };
                            let a1565 = {
                                let a1562 = match v1.clone() {
                                    Adt3::C0(_, _, h1561, _, _, _, _, _) => {
                                        h1561
                                    }
                                };
                                let a1563 = v13;
                                nat_mul(a1562, a1563)?
                            };
                            nat_add(a1564, a1565)?
                        };
                        nat_mul(a1566, a1567)?
                    };
                    nat_add(a1568, a1569)?
                };
                nat_mul(a1570, a1571)?
            };
            nat_add(a1572, a1573)?
        };
        Ok(Adt5::C1(false, f48(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1579 = {
                let a1577 = {
                    let a1574 = v11;
                    let a1575 = v5;
                    let a1576 = 0u64;
                    nat_rem(a1574, a1575, a1576)
                };
                let a1578 = v4;
                nat_mul(a1577, a1578)?
            };
            let a1580 = v8;
            nat_add(a1579, a1580)?
        };
        let v16: u64 = {
            let a1586 = {
                let a1584 = {
                    let a1581 = v11;
                    let a1582 = v5;
                    let a1583 = 0u64;
                    nat_quot(a1581, a1582, a1583)
                };
                let a1585 = v4;
                nat_mul(a1584, a1585)?
            };
            let a1587 = v10;
            nat_add(a1586, a1587)?
        };
        let a1589 = v15;
        let a1590 = match v1.clone() {
            Adt3::C0(_, h1588, _, _, _, _, _, _) => {
                h1588
            }
        };
        if if nat_lt(a1589, a1590) {
            let a1592 = v16;
            let a1593 = match v1.clone() {
                Adt3::C0(_, _, h1591, _, _, _, _, _) => {
                    h1591
                }
            };
            nat_lt(a1592, a1593)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f48({
                let a1602 = v6;
                let a1603 = {
                    let a1600 = match v1.clone() {
                        Adt3::C0(h1594, _, _, _, _, _, _, _) => {
                            h1594
                        }
                    };
                    let a1601 = {
                        let a1598 = v15;
                        let a1599 = {
                            let a1596 = match v1.clone() {
                                Adt3::C0(_, h1595, _, _, _, _, _, _) => {
                                    h1595
                                }
                            };
                            let a1597 = v16;
                            nat_mul(a1596, a1597)?
                        };
                        nat_add(a1598, a1599)?
                    };
                    nat_mul(a1600, a1601)?
                };
                nat_add(a1602, a1603)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f48(v0, v2.clone())?;
    let v4: u64 = f48(v0, v1.clone())?;
    let a1608 = {
        let a1605 = v0;
        let a1606 = match v1 {
            Adt3::C0(h1604, _, _, _, _, _, _, _) => {
                h1604
            }
        };
        let a1607 = 0u64;
        nat_rem(a1605, a1606, a1607)
    };
    let a1609 = 0u64;
    if nat_eq(a1608, a1609) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f48({
            let a1610 = v0;
            let a1611 = 1u64;
            nat_sub(a1610, a1611)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1612 = v3;
    let a1613 = 13u64;
    if nat_eq(a1612, a1613) {
        f84(v0, v1, v4)
    } else {
        let v5: u64 = f72(match v1.clone() {
            Adt3::C0(h1614, _, _, _, _, _, _, _) => {
                h1614
            }
        }, {
            let a1615 = v3;
            let a1616 = 16u64;
            nat_eq(a1615, a1616)
        })?;
        let v6: u64 = {
            let a1617 = v0;
            let a1618 = v5;
            let a1619 = 0u64;
            nat_quot(a1617, a1618, a1619)
        };
        let v7: u64 = {
            let a1620 = v0;
            let a1621 = v5;
            let a1622 = 0u64;
            nat_rem(a1620, a1621, a1622)
        };
        let v8: u64 = {
            let a1626 = 1u64;
            let a1627 = {
                let a1624 = match v1.clone() {
                    Adt3::C0(h1623, _, _, _, _, _, _, _) => {
                        h1623
                    }
                };
                let a1625 = 2u64;
                nat_mul(a1624, a1625)?
            };
            nat_add(a1626, a1627)?
        };
        let a1628 = v7;
        let a1629 = v8;
        if nat_lt(a1628, a1629) {
            f85(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f48(v6, v2)?;
            let a1630 = v3;
            let a1631 = 14u64;
            if nat_eq(a1630, a1631) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1632 = v7;
                let a1633 = v8;
                if nat_eq(a1632, a1633) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1636 = v7;
                    let a1637 = {
                        let a1634 = v8;
                        let a1635 = 1u64;
                        nat_add(a1634, a1635)?
                    };
                    if nat_eq(a1636, a1637) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1638, _, _, _, _, _, _, _) => {
                                h1638
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

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1639 = v0;
    let a1640 = 0u64;
    if nat_eq(a1639, a1640) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1647 = {
            let a1641 = v0;
            let a1642 = 1u64;
            nat_add(a1641, a1642)?
        };
        let a1648 = {
            let a1645 = 2u64;
            let a1646 = {
                let a1643 = v3;
                let a1644 = 2u64;
                nat_mul(a1643, a1644)?
            };
            nat_add(a1645, a1646)?
        };
        if nat_eq(a1647, a1648) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1649 = v0;
                let a1650 = 1u64;
                nat_sub(a1649, a1650)
            };
            let v5: u64 = {
                let a1651 = v4;
                let a1652 = 2u64;
                let a1653 = 0u64;
                nat_quot(a1651, a1652, a1653)
            };
            let a1657 = {
                let a1654 = v4;
                let a1655 = 2u64;
                let a1656 = 0u64;
                nat_rem(a1654, a1655, a1656)
            };
            let a1658 = 0u64;
            if nat_eq(a1657, a1658) {
                Ok(Adt5::C4(17u64, 0u64, f48(v5, v1)?, 4u64, f48(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f61(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1666 = {
        let a1659 = 2u64;
        let a1660 = f45(v1.clone())?;
        nat_mul(a1659, a1660)?
    };
    let a1667 = {
        let a1664 = f45(v1)?;
        let a1665 = {
            let a1662 = 1u64;
            let a1663 = match v0 {
                Adt3::C0(h1661, _, _, _, _, _, _, _) => {
                    h1661
                }
            };
            nat_add(a1662, a1663)?
        };
        nat_mul(a1664, a1665)?
    };
    nat_add(a1666, a1667)
}

pub fn f62(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f48(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f63(v0: u64, v1: u64) -> bool {
    let a1668 = v1;
    let a1669 = 2147483647u64;
    if nat_le(a1668, a1669) {
        let a1670 = v1;
        let a1671 = v0;
        nat_lt(a1670, a1671)
    } else {
        false
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1672 = f32(v0);
    match m1672 {
        Adt7::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt7::C1(v7) => {
            let v8: u64 = {
                let a1674 = v2;
                let a1675 = match v7.clone() {
                    Adt8::C0(_, h1673, _) => {
                        h1673
                    }
                };
                nat_mul(a1674, a1675)?
            };
            let m1688 = f86(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1676, _, _, _, _, _, _, _) => {
                    h1676
                }
            }, match v5.clone() {
                Adt3::C0(_, h1677, _, _, _, _, _, _) => {
                    h1677
                }
            }, match v5 {
                Adt3::C0(_, _, h1678, _, _, _, _, _) => {
                    h1678
                }
            }, {
                let a1681 = match v3.clone() {
                    Adt3::C0(h1679, _, _, _, _, _, _, _) => {
                        h1679
                    }
                };
                let a1682 = match v7.clone() {
                    Adt8::C0(_, h1680, _) => {
                        h1680
                    }
                };
                nat_mul(a1681, a1682)?
            }, match v3.clone() {
                Adt3::C0(_, h1683, _, _, _, _, _, _) => {
                    h1683
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1684, _, _, _) => {
                    h1684
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1685, _, _) => {
                    h1685
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1686, _) => {
                    h1686
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1687) => {
                    h1687
                }
            })?;
            match m1688 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1689 = v0;
                    let a1690 = 0u64;
                    if if nat_eq(a1689, a1690) {
                        true
                    } else {
                        let a1691 = v0;
                        let a1692 = 26u64;
                        nat_eq(a1691, a1692)
                    } {
                        Ok(Adt5::C1(false, v9, f48({
                            let a1696 = {
                                let a1694 = v1;
                                let a1695 = match v4.clone() {
                                    Adt3::C0(h1693, _, _, _, _, _, _, _) => {
                                        h1693
                                    }
                                };
                                nat_mul(a1694, a1695)?
                            };
                            let a1697 = v8;
                            nat_add(a1696, a1697)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt8::C0(_, _, h1698) => {
                                h1698
                            }
                        }, f48({
                            let a1702 = {
                                let a1700 = v1;
                                let a1701 = match v4.clone() {
                                    Adt3::C0(h1699, _, _, _, _, _, _, _) => {
                                        h1699
                                    }
                                };
                                nat_mul(a1700, a1701)?
                            };
                            let a1703 = v8;
                            nat_add(a1702, a1703)?
                        }, v4.clone())?, {
                            let a1710 = {
                                let a1708 = {
                                    let a1705 = match v7 {
                                        Adt8::C0(_, h1704, _) => {
                                            h1704
                                        }
                                    };
                                    let a1706 = 1u64;
                                    nat_sub(a1705, a1706)
                                };
                                let a1709 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1707, _, _, _) => {
                                        h1707
                                    }
                                };
                                nat_mul(a1708, a1709)?
                            };
                            let a1711 = 4u64;
                            nat_add(a1710, a1711)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1712, _, _, _) => {
                                h1712
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f87(v0, v1, v6, v7, v8) {
        let a1713 = v5;
        let a1714 = 26u64;
        nat_eq(a1713, a1714)
    } else {
        false
    } {
        let a1715 = v3;
        let a1716 = v7;
        nat_eq(a1715, a1716)
    } else {
        false
    } {
        let a1717 = v4;
        let a1718 = v8;
        nat_eq(a1717, a1718)
    } else {
        false
    } {
        let a1719 = v9;
        let a1720 = 1u64;
        nat_eq(a1719, a1720)
    } else {
        false
    } {
        let a1721 = v10;
        let a1722 = f88(v0);
        nat_eq(a1721, a1722)
    } else {
        false
    } {
        let a1723 = v11;
        let a1724 = v1;
        nat_eq(a1723, a1724)
    } else {
        false
    } {
        let a1725 = v12;
        let a1726 = v6;
        nat_eq(a1725, a1726)
    } else {
        false
    } {
        let a1727 = v13;
        let a1728 = v7;
        nat_eq(a1727, a1728)
    } else {
        false
    } {
        let a1729 = v14;
        let a1730 = v8;
        nat_eq(a1729, a1730)
    } else {
        false
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1731 = v0;
    let a1732 = 0u64;
    if if if nat_eq(a1731, a1732) {
        true
    } else {
        let a1733 = v1;
        let a1734 = 0u64;
        nat_eq(a1733, a1734)
    } {
        true
    } else {
        let a1735 = v2;
        let a1736 = 0u64;
        nat_eq(a1735, a1736)
    } {
        true
    } else {
        let a1737 = v3;
        let a1738 = 0u64;
        nat_eq(a1737, a1738)
    }
}

pub fn f67(v0: u64) -> u64 {
    let a1739 = v0;
    let a1740 = 2147483648u64;
    if nat_lt(a1739, a1740) {
        v0
    } else {
        0u64
    }
}

pub fn f68(v0: u64) -> u64 {
    let a1741 = v0;
    let a1742 = 2147483648u64;
    if nat_lt(a1741, a1742) {
        0u64
    } else {
        let a1743 = 4294967296u64;
        let a1744 = v0;
        nat_sub(a1743, a1744)
    }
}

pub fn f69(v0: Adt3, v1: bool) -> R<u64> {
    f89(v0.clone(), {
        let a1746 = match v0.clone() {
            Adt3::C0(h1745, _, _, _, _, _, _, _) => {
                h1745
            }
        };
        let a1747 = 1u64;
        nat_sub(a1746, a1747)
    }, {
        let a1749 = match v0.clone() {
            Adt3::C0(_, h1748, _, _, _, _, _, _) => {
                h1748
            }
        };
        let a1750 = 1u64;
        nat_sub(a1749, a1750)
    }, {
        let a1752 = match v0.clone() {
            Adt3::C0(_, _, h1751, _, _, _, _, _) => {
                h1751
            }
        };
        let a1753 = 1u64;
        nat_sub(a1752, a1753)
    }, {
        let a1755 = match v0 {
            Adt3::C0(_, _, _, h1754, _, _, _, _) => {
                h1754
            }
        };
        let a1756 = 1u64;
        nat_sub(a1755, a1756)
    }, v1)
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1757 = v0;
    let a1758 = 0u64;
    if nat_eq(a1757, a1758) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a1759 = v0;
            let a1760 = 1u64;
            nat_sub(a1759, a1760)
        };
        let v5: u64 = {
            let a1761 = v4;
            let a1762 = v1;
            let a1763 = 0u64;
            nat_quot(a1761, a1762, a1763)
        };
        let v6: u64 = {
            let a1771 = {
                let a1764 = v5;
                let a1765 = v2;
                nat_mul(a1764, a1765)?
            };
            let a1772 = {
                let a1769 = {
                    let a1766 = v4;
                    let a1767 = v1;
                    let a1768 = 0u64;
                    nat_rem(a1766, a1767, a1768)
                };
                let a1770 = v3;
                nat_mul(a1769, a1770)?
            };
            nat_add(a1771, a1772)?
        };
        let a1773 = v5;
        let a1774 = 0u64;
        if nat_eq(a1773, a1774) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a1783 = {
                    let a1777 = {
                        let a1775 = v5;
                        let a1776 = 1u64;
                        nat_sub(a1775, a1776)
                    };
                    let a1778 = v2;
                    nat_mul(a1777, a1778)?
                };
                let a1784 = {
                    let a1781 = {
                        let a1779 = v1;
                        let a1780 = 1u64;
                        nat_sub(a1779, a1780)
                    };
                    let a1782 = v3;
                    nat_mul(a1781, a1782)?
                };
                nat_add(a1783, a1784)?
            };
            let a1785 = v6;
            let a1786 = v7;
            if nat_lt(a1785, a1786) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f71(v0: Adt3, v1: u64) -> bool {
    let a1787 = v1;
    let a1788 = 0u64;
    if !nat_eq(a1787, a1788) {
        true
    } else {
        let a1790 = match v0 {
            Adt3::C0(_, _, _, _, h1789, _, _, _) => {
                h1789
            }
        };
        let a1791 = 4u64;
        nat_eq(a1790, a1791)
    }
}

pub fn f72(v0: u64, v1: bool) -> R<u64> {
    let a1794 = {
        let a1792 = v0;
        let a1793 = 2u64;
        nat_mul(a1792, a1793)?
    };
    let a1795 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a1794, a1795)
}

pub fn f73(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m1796 = f90(v0, v1);
    match m1796 {
        None => {
            if v5 {
                let m1797 = f91(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m1797 {
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
                let a1798 = 4u64;
                let a1799 = v28;
                nat_le(a1798, a1799)
            };
            let a1800 = v2;
            let a1801 = 0u64;
            if if if if if nat_eq(a1800, a1801) {
                let a1802 = v4;
                let a1803 = 0u64;
                nat_eq(a1802, a1803)
            } else {
                false
            } {
                let a1804 = v6;
                let a1805 = v14;
                if nat_eq(a1804, a1805) {
                    let a1806 = v7;
                    let a1807 = v15;
                    nat_eq(a1806, a1807)
                } else {
                    false
                }
            } else {
                false
            } {
                let a1808 = v8;
                let a1809 = v16;
                if nat_eq(a1808, a1809) {
                    let a1810 = v9;
                    let a1811 = v17;
                    nat_eq(a1810, a1811)
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
                        let a1812 = v3;
                        let a1813 = 0u64;
                        nat_eq(a1812, a1813)
                    } else {
                        false
                    } {
                        f33(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a1818 = {
                        let a1814 = v6;
                        let a1815 = v7;
                        nat_mul(a1814, a1815)?
                    };
                    let a1819 = {
                        let a1816 = v8;
                        let a1817 = v9;
                        nat_mul(a1816, a1817)?
                    };
                    nat_mul(a1818, a1819)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f74(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a1832 = {
        let a1828 = {
            let a1824 = {
                let a1820 = v0;
                let a1821 = v4;
                nat_mul(a1820, a1821)?
            };
            let a1825 = {
                let a1822 = v1;
                let a1823 = v5;
                nat_mul(a1822, a1823)?
            };
            nat_add(a1824, a1825)?
        };
        let a1829 = {
            let a1826 = v2;
            let a1827 = v6;
            nat_mul(a1826, a1827)?
        };
        nat_add(a1828, a1829)?
    };
    let a1833 = {
        let a1830 = v3;
        let a1831 = v7;
        nat_mul(a1830, a1831)?
    };
    nat_add(a1832, a1833)
}

pub fn f75(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f74({
        let a1834 = v0;
        let a1835 = v4;
        let a1836 = 0u64;
        nat_rem(a1834, a1835, a1836)
    }, {
        let a1837 = v1;
        let a1838 = v5;
        let a1839 = 0u64;
        nat_rem(a1837, a1838, a1839)
    }, {
        let a1840 = v2;
        let a1841 = v6;
        let a1842 = 0u64;
        nat_rem(a1840, a1841, a1842)
    }, {
        let a1843 = v3;
        let a1844 = v7;
        let a1845 = 0u64;
        nat_rem(a1843, a1844, a1845)
    }, v8, v9, v10, v11)
}

pub fn f76(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1847 = v0;
        let a1848 = match v1.clone() {
            Adt3::C0(h1846, _, _, _, _, _, _, _) => {
                h1846
            }
        };
        let a1849 = 0u64;
        nat_quot(a1847, a1848, a1849)
    };
    let v4: u64 = {
        let a1851 = v3;
        let a1852 = match v1.clone() {
            Adt3::C0(_, h1850, _, _, _, _, _, _) => {
                h1850
            }
        };
        let a1853 = 0u64;
        nat_quot(a1851, a1852, a1853)
    };
    let a1854 = v2;
    let a1855 = 0u64;
    if nat_eq(a1854, a1855) {
        let a1857 = v0;
        let a1858 = match v1 {
            Adt3::C0(h1856, _, _, _, _, _, _, _) => {
                h1856
            }
        };
        let a1859 = 0u64;
        nat_rem(a1857, a1858, a1859)
    } else {
        let a1860 = v2;
        let a1861 = 1u64;
        if nat_eq(a1860, a1861) {
            let a1863 = v3;
            let a1864 = match v1 {
                Adt3::C0(_, h1862, _, _, _, _, _, _) => {
                    h1862
                }
            };
            let a1865 = 0u64;
            nat_rem(a1863, a1864, a1865)
        } else {
            let a1866 = v2;
            let a1867 = 2u64;
            if nat_eq(a1866, a1867) {
                let a1869 = v4;
                let a1870 = match v1 {
                    Adt3::C0(_, _, h1868, _, _, _, _, _) => {
                        h1868
                    }
                };
                let a1871 = 0u64;
                nat_rem(a1869, a1870, a1871)
            } else {
                let a1873 = v4;
                let a1874 = match v1 {
                    Adt3::C0(_, _, h1872, _, _, _, _, _) => {
                        h1872
                    }
                };
                let a1875 = 0u64;
                nat_quot(a1873, a1874, a1875)
            }
        }
    }
}

pub fn f77(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f74(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1876, _, _, _) => {
            h1876
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1877, _, _) => {
            h1877
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1878, _) => {
            h1878
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1879) => {
            h1879
        }
    })
}

pub fn f78(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1880 = v1;
    let a1881 = 2147483648u64;
    if nat_lt(a1880, a1881) {
        let a1892 = {
            let a1890 = {
                let a1885 = {
                    let a1882 = v0;
                    let a1883 = v2;
                    let a1884 = 0u64;
                    nat_rem(a1882, a1883, a1884)
                };
                let a1886 = v2;
                nat_add(a1885, a1886)?
            };
            let a1891 = {
                let a1887 = v1;
                let a1888 = v2;
                let a1889 = 0u64;
                nat_rem(a1887, a1888, a1889)
            };
            nat_sub(a1890, a1891)
        };
        let a1893 = v2;
        let a1894 = 0u64;
        Ok(nat_rem(a1892, a1893, a1894))
    } else {
        let a1897 = {
            let a1895 = v0;
            let a1896 = f68(v1);
            nat_add(a1895, a1896)?
        };
        let a1898 = v2;
        let a1899 = 0u64;
        Ok(nat_rem(a1897, a1898, a1899))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1902 = f67(v1);
    let a1903 = {
        let a1900 = v0;
        let a1901 = f68(v1);
        nat_add(a1900, a1901)?
    };
    if nat_le(a1902, a1903) {
        let a1904 = f80(v0, v1)?;
        let a1905 = v2;
        Ok(nat_lt(a1904, a1905))
    } else {
        Ok(false)
    }
}

pub fn f80(v0: u64, v1: u64) -> R<u64> {
    let a1908 = {
        let a1906 = v0;
        let a1907 = f68(v1);
        nat_add(a1906, a1907)?
    };
    let a1909 = f67(v1);
    Ok(nat_sub(a1908, a1909))
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1910 = v0;
    let a1911 = v1;
    if nat_lt(a1910, a1911) {
        let a1912 = v1;
        let a1913 = v0;
        Ok(nat_sub(a1912, a1913))
    } else {
        let a1916 = v0;
        let a1917 = {
            let a1914 = v1;
            let a1915 = v2;
            nat_add(a1914, a1915)?
        };
        if nat_lt(a1916, a1917) {
            let a1918 = v0;
            let a1919 = v1;
            Ok(nat_sub(a1918, a1919))
        } else {
            let a1926 = {
                let a1922 = 2u64;
                let a1923 = {
                    let a1920 = v2;
                    let a1921 = 1u64;
                    nat_sub(a1920, a1921)
                };
                nat_mul(a1922, a1923)?
            };
            let a1927 = {
                let a1924 = v0;
                let a1925 = v1;
                nat_sub(a1924, a1925)
            };
            Ok(nat_sub(a1926, a1927))
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a1930 = {
        let a1928 = v2;
        let a1929 = f89(v1.clone(), f76(v0, v1.clone(), 0u64), f76(v0, v1.clone(), 1u64), f76(v0, v1.clone(), 2u64), f76(v0, v1.clone(), 3u64), false)?;
        nat_add(a1928, a1929)?
    };
    let a1931 = f89(v1.clone(), f76(v0, v1.clone(), 0u64), f76(v0, v1.clone(), 1u64), f76(v0, v1.clone(), 2u64), f76(v0, v1, 3u64), true)?;
    Ok(nat_sub(a1930, a1931))
}

pub fn f83(v0: u64, v1: u64, v2: u64) -> bool {
    let a1932 = v0;
    let a1933 = 0u64;
    if nat_eq(a1932, a1933) {
        let a1934 = v2;
        let a1935 = v1;
        nat_le(a1934, a1935)
    } else {
        let a1936 = v0;
        let a1937 = 1u64;
        if nat_eq(a1936, a1937) {
            let a1938 = v2;
            let a1939 = v1;
            nat_lt(a1938, a1939)
        } else {
            let a1940 = v0;
            let a1941 = 2u64;
            if nat_eq(a1940, a1941) {
                let a1942 = v1;
                let a1943 = v2;
                nat_le(a1942, a1943)
            } else {
                let a1944 = v0;
                let a1945 = 3u64;
                if nat_eq(a1944, a1945) {
                    let a1946 = v1;
                    let a1947 = v2;
                    nat_lt(a1946, a1947)
                } else {
                    let a1948 = v1;
                    let a1949 = v2;
                    nat_eq(a1948, a1949)
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a1950 = v0;
    let a1951 = 0u64;
    if nat_eq(a1950, a1951) {
        Ok(f92(0u64))
    } else {
        let a1954 = {
            let a1952 = v0;
            let a1953 = 1u64;
            nat_add(a1952, a1953)?
        };
        let a1955 = f46(v1.clone(), 13u64, v2)?;
        if nat_eq(a1954, a1955) {
            Ok(Adt5::C4({
                let a1956 = v2;
                let a1957 = 0u64;
                if nat_eq(a1956, a1957) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a1958 = v0;
                let a1959 = 1u64;
                nat_sub(a1958, a1959)
            };
            if f71(v1.clone(), v2) {
                let v4: u64 = f72(match v1.clone() {
                    Adt3::C0(h1960, _, _, _, _, _, _, _) => {
                        h1960
                    }
                }, false)?;
                let v5: u64 = {
                    let a1961 = v3;
                    let a1962 = v4;
                    let a1963 = 0u64;
                    nat_quot(a1961, a1962, a1963)
                };
                let v6: u64 = {
                    let a1964 = v3;
                    let a1965 = v4;
                    let a1966 = 0u64;
                    nat_rem(a1964, a1965, a1966)
                };
                let a1969 = {
                    let a1967 = v6;
                    let a1968 = 1u64;
                    nat_add(a1967, a1968)?
                };
                let a1970 = v4;
                if nat_eq(a1969, a1970) {
                    Ok(Adt5::C4({
                        let a1971 = v2;
                        let a1972 = 0u64;
                        if nat_eq(a1971, a1972) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f85(v6, v5, v1, v2)
                }
            } else {
                f93({
                    let a1973 = v3;
                    let a1974 = 2u64;
                    let a1975 = 0u64;
                    nat_quot(a1973, a1974, a1975)
                }, {
                    let a1976 = v3;
                    let a1977 = 2u64;
                    let a1978 = 0u64;
                    nat_rem(a1976, a1977, a1978)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1979 = v0;
    let a1980 = 0u64;
    if nat_eq(a1979, a1980) {
        Ok(f92(1u64))
    } else {
        f93({
            let a1989 = {
                let a1982 = v1;
                let a1983 = match v2.clone() {
                    Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                        h1981
                    }
                };
                nat_mul(a1982, a1983)?
            };
            let a1990 = {
                let a1986 = {
                    let a1984 = v0;
                    let a1985 = 1u64;
                    nat_sub(a1984, a1985)
                };
                let a1987 = 2u64;
                let a1988 = 0u64;
                nat_quot(a1986, a1987, a1988)
            };
            nat_add(a1989, a1990)?
        }, {
            let a1993 = {
                let a1991 = v0;
                let a1992 = 1u64;
                nat_sub(a1991, a1992)
            };
            let a1994 = 2u64;
            let a1995 = 0u64;
            nat_rem(a1993, a1994, a1995)
        }, v2, v3, 1u64)
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m1996 = f32(v0);
    match m1996 {
        Adt7::C0 => {
            Ok(None::<u64>)
        }
        Adt7::C1(v13) => {
            if if if if if f94(v0) {
                let a1997 = v1;
                let a1998 = v7;
                nat_lt(a1997, a1998)
            } else {
                false
            } {
                f63(v8, v2)
            } else {
                false
            } {
                let a1999 = v3;
                let a2000 = f95(v4, v5, v6)?;
                nat_lt(a1999, a2000)
            } else {
                false
            } {
                let a2005 = {
                    let a2002 = v7;
                    let a2003 = match v13.clone() {
                        Adt8::C0(_, h2001, _) => {
                            h2001
                        }
                    };
                    let a2004 = 0u64;
                    nat_rem(a2002, a2003, a2004)
                };
                let a2006 = 0u64;
                nat_eq(a2005, a2006)
            } else {
                false
            } {
                Ok(Some({
                    let a2035 = {
                        let a2025 = {
                            let a2015 = {
                                let a2011 = {
                                    let a2008 = v1;
                                    let a2009 = match v13 {
                                        Adt8::C0(_, h2007, _) => {
                                            h2007
                                        }
                                    };
                                    let a2010 = 0u64;
                                    nat_quot(a2008, a2009, a2010)
                                };
                                let a2012 = v9;
                                nat_mul(a2011, a2012)?
                            };
                            let a2016 = {
                                let a2013 = v2;
                                let a2014 = v10;
                                nat_mul(a2013, a2014)?
                            };
                            nat_add(a2015, a2016)?
                        };
                        let a2026 = {
                            let a2023 = {
                                let a2020 = {
                                    let a2017 = v3;
                                    let a2018 = v4;
                                    let a2019 = 0u64;
                                    nat_quot(a2017, a2018, a2019)
                                };
                                let a2021 = v5;
                                let a2022 = 0u64;
                                nat_rem(a2020, a2021, a2022)
                            };
                            let a2024 = v11;
                            nat_mul(a2023, a2024)?
                        };
                        nat_add(a2025, a2026)?
                    };
                    let a2036 = {
                        let a2033 = {
                            let a2030 = {
                                let a2027 = v3;
                                let a2028 = v4;
                                let a2029 = 0u64;
                                nat_quot(a2027, a2028, a2029)
                            };
                            let a2031 = v5;
                            let a2032 = 0u64;
                            nat_quot(a2030, a2031, a2032)
                        };
                        let a2034 = v12;
                        nat_mul(a2033, a2034)?
                    };
                    nat_add(a2035, a2036)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f66(v1, v2, v3, v4) {
        f96(v0)
    } else {
        f94(v0)
    }
}

pub fn f88(v0: u64) -> u64 {
    let a2037 = v0;
    let a2038 = 26u64;
    if nat_eq(a2037, a2038) {
        26u64
    } else {
        0u64
    }
}

pub fn f89(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2059 = {
        let a2053 = {
            let a2047 = {
                let a2041 = v1;
                let a2042 = if v5 {
                    f68(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2039, _, _, _) => {
                            h2039
                        }
                    })
                } else {
                    f67(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2040, _, _, _) => {
                            h2040
                        }
                    })
                };
                nat_mul(a2041, a2042)?
            };
            let a2048 = {
                let a2045 = v2;
                let a2046 = if v5 {
                    f68(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2043, _, _) => {
                            h2043
                        }
                    })
                } else {
                    f67(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2044, _, _) => {
                            h2044
                        }
                    })
                };
                nat_mul(a2045, a2046)?
            };
            nat_add(a2047, a2048)?
        };
        let a2054 = {
            let a2051 = v3;
            let a2052 = if v5 {
                f68(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2049, _) => {
                        h2049
                    }
                })
            } else {
                f67(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2050, _) => {
                        h2050
                    }
                })
            };
            nat_mul(a2051, a2052)?
        };
        nat_add(a2053, a2054)?
    };
    let a2060 = {
        let a2057 = v4;
        let a2058 = if v5 {
            f68(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2055) => {
                    h2055
                }
            })
        } else {
            f67(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2056) => {
                    h2056
                }
            })
        };
        nat_mul(a2057, a2058)?
    };
    nat_add(a2059, a2060)
}

pub fn f90(v0: u64, v1: u64) -> Option<u64> {
    let a2061 = v0;
    let a2062 = 2u64;
    if nat_eq(a2061, a2062) {
        Some(0u64)
    } else {
        let a2063 = v0;
        let a2064 = 5u64;
        if nat_eq(a2063, a2064) {
            Some(1u64)
        } else {
            let a2065 = v0;
            let a2066 = 6u64;
            if nat_eq(a2065, a2066) {
                Some(2u64)
            } else {
                let a2067 = v0;
                let a2068 = 7u64;
                if nat_eq(a2067, a2068) {
                    Some(3u64)
                } else {
                    let a2069 = v0;
                    let a2070 = 9u64;
                    if nat_eq(a2069, a2070) {
                        Some(4u64)
                    } else {
                        let a2071 = v0;
                        let a2072 = 8u64;
                        if nat_eq(a2071, a2072) {
                            Some(5u64)
                        } else {
                            let a2073 = v0;
                            let a2074 = 89u64;
                            if nat_eq(a2073, a2074) {
                                let a2075 = v1;
                                let a2076 = 2u64;
                                if nat_eq(a2075, a2076) {
                                    Some(6u64)
                                } else {
                                    let a2077 = v1;
                                    let a2078 = 0u64;
                                    if nat_eq(a2077, a2078) {
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

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a2079 = v0;
    let a2080 = 28u64;
    if if if if nat_eq(a2079, a2080) {
        let a2081 = v1;
        let a2082 = 0u64;
        nat_eq(a2081, a2082)
    } else {
        false
    } {
        let a2083 = v2;
        let a2084 = 0u64;
        nat_eq(a2083, a2084)
    } else {
        false
    } {
        let a2085 = v3;
        let a2086 = 0u64;
        nat_eq(a2085, a2086)
    } else {
        false
    } {
        let a2087 = v4;
        let a2088 = v8;
        if if if if if if if if nat_eq(a2087, a2088) {
            let a2089 = v5;
            let a2090 = v12;
            nat_eq(a2089, a2090)
        } else {
            false
        } {
            let a2091 = v9;
            let a2092 = v13;
            nat_eq(a2091, a2092)
        } else {
            false
        } {
            let a2093 = v10;
            let a2094 = v14;
            nat_eq(a2093, a2094)
        } else {
            false
        } {
            let a2095 = v11;
            let a2096 = v15;
            nat_eq(a2095, a2096)
        } else {
            false
        } {
            let a2097 = 0u64;
            let a2098 = v6;
            nat_lt(a2097, a2098)
        } else {
            false
        } {
            let a2099 = 0u64;
            let a2100 = v7;
            nat_lt(a2099, a2100)
        } else {
            false
        } {
            let a2104 = {
                let a2101 = v10;
                let a2102 = v6;
                let a2103 = 1u64;
                nat_rem(a2101, a2102, a2103)
            };
            let a2105 = 0u64;
            if nat_eq(a2104, a2105) {
                let a2109 = {
                    let a2106 = v11;
                    let a2107 = v7;
                    let a2108 = 1u64;
                    nat_rem(a2106, a2107, a2108)
                };
                let a2110 = 0u64;
                nat_eq(a2109, a2110)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a2111 = v10;
                let a2112 = v6;
                let a2113 = 0u64;
                nat_quot(a2111, a2112, a2113)
            }, {
                let a2114 = v11;
                let a2115 = v7;
                let a2116 = 0u64;
                nat_quot(a2114, a2115, a2116)
            }, {
                let a2117 = v5;
                let a2118 = v4;
                nat_mul(a2117, a2118)?
            }, {
                let a2119 = v4;
                let a2120 = v9;
                nat_mul(a2119, a2120)?
            }, {
                let a2121 = v5;
                let a2122 = v9;
                nat_mul(a2121, a2122)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f92(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f93(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2123 = v1;
    let a2124 = 0u64;
    if nat_eq(a2123, a2124) {
        Ok(Adt5::C4({
            let a2125 = v3;
            let a2126 = 0u64;
            if nat_eq(a2125, a2126) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f48(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2127 = v3;
            let a2128 = 0u64;
            if nat_eq(a2127, a2128) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f94(v0: u64) -> bool {
    let a2129 = v0;
    let a2130 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2129, a2130) {
        true
    } else {
        let a2131 = v0;
        let a2132 = 1u64;
        nat_eq(a2131, a2132)
    } {
        true
    } else {
        let a2133 = v0;
        let a2134 = 2u64;
        nat_eq(a2133, a2134)
    } {
        true
    } else {
        let a2135 = v0;
        let a2136 = 3u64;
        nat_eq(a2135, a2136)
    } {
        true
    } else {
        let a2137 = v0;
        let a2138 = 6u64;
        nat_eq(a2137, a2138)
    } {
        true
    } else {
        let a2139 = v0;
        let a2140 = 7u64;
        nat_eq(a2139, a2140)
    } {
        true
    } else {
        let a2141 = v0;
        let a2142 = 8u64;
        nat_eq(a2141, a2142)
    } {
        true
    } else {
        let a2143 = v0;
        let a2144 = 10u64;
        nat_eq(a2143, a2144)
    } {
        true
    } else {
        let a2145 = v0;
        let a2146 = 11u64;
        nat_eq(a2145, a2146)
    } {
        true
    } else {
        let a2147 = v0;
        let a2148 = 12u64;
        nat_eq(a2147, a2148)
    } {
        true
    } else {
        let a2149 = v0;
        let a2150 = 13u64;
        nat_eq(a2149, a2150)
    } {
        true
    } else {
        let a2151 = v0;
        let a2152 = 14u64;
        nat_eq(a2151, a2152)
    } {
        true
    } else {
        let a2153 = v0;
        let a2154 = 16u64;
        nat_eq(a2153, a2154)
    } {
        true
    } else {
        let a2155 = v0;
        let a2156 = 17u64;
        nat_eq(a2155, a2156)
    } {
        true
    } else {
        let a2157 = v0;
        let a2158 = 18u64;
        nat_eq(a2157, a2158)
    } {
        true
    } else {
        let a2159 = v0;
        let a2160 = 19u64;
        nat_eq(a2159, a2160)
    } {
        true
    } else {
        let a2161 = v0;
        let a2162 = 20u64;
        nat_eq(a2161, a2162)
    } {
        true
    } else {
        let a2163 = v0;
        let a2164 = 21u64;
        nat_eq(a2163, a2164)
    } {
        true
    } else {
        let a2165 = v0;
        let a2166 = 22u64;
        nat_eq(a2165, a2166)
    } {
        true
    } else {
        let a2167 = v0;
        let a2168 = 23u64;
        nat_eq(a2167, a2168)
    } {
        true
    } else {
        let a2169 = v0;
        let a2170 = 26u64;
        nat_eq(a2169, a2170)
    } {
        true
    } else {
        let a2171 = v0;
        let a2172 = 29u64;
        nat_eq(a2171, a2172)
    } {
        true
    } else {
        let a2173 = v0;
        let a2174 = 30u64;
        nat_eq(a2173, a2174)
    } {
        true
    } else {
        let a2175 = v0;
        let a2176 = 34u64;
        nat_eq(a2175, a2176)
    } {
        true
    } else {
        let a2177 = v0;
        let a2178 = 35u64;
        nat_eq(a2177, a2178)
    } {
        true
    } else {
        let a2179 = v0;
        let a2180 = 39u64;
        nat_eq(a2179, a2180)
    } {
        true
    } else {
        let a2181 = v0;
        let a2182 = 40u64;
        nat_eq(a2181, a2182)
    } {
        true
    } else {
        let a2183 = v0;
        let a2184 = 41u64;
        nat_eq(a2183, a2184)
    } {
        true
    } else {
        let a2185 = v0;
        let a2186 = 42u64;
        nat_eq(a2185, a2186)
    } {
        true
    } else {
        let a2187 = v0;
        let a2188 = 43u64;
        nat_eq(a2187, a2188)
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2189 = v0;
    let a2190 = 0u64;
    if if if nat_eq(a2189, a2190) {
        true
    } else {
        let a2191 = v1;
        let a2192 = 0u64;
        nat_eq(a2191, a2192)
    } {
        true
    } else {
        let a2193 = v2;
        let a2194 = 0u64;
        nat_eq(a2193, a2194)
    } {
        Ok(0u64)
    } else {
        let a2197 = {
            let a2195 = v0;
            let a2196 = v1;
            nat_mul(a2195, a2196)?
        };
        let a2198 = v2;
        nat_mul(a2197, a2198)
    }
}

pub fn f96(v0: u64) -> bool {
    let m2199 = f32(v0);
    match m2199 {
        Adt7::C0 => {
            false
        }
        Adt7::C1(_) => {
            true
        }
    }
}
