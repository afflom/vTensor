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
                        let a6 = 3u64;
                        if nat_eq(a5, a6) {
                            Ok(Adt5::C0(f2(v14, v4, v5, v13)?))
                        } else {
                            f3(v1, v2, v14, v4, v13)
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18) => {
            f4(v15, v16, v17, v18)
        }
        Adt0::C2(v19, v20, v21, v22) => {
            f5(v19, v20, v21, v22)
        }
        Adt0::C3(v23, v24, v25, v26, v27) => {
            f6(v23, v24, v25, v26, v27)
        }
        Adt0::C4(v28, v29, v30, v31, v32, v33) => {
            f7(v28, v29, v30, v31, v32, v33)
        }
        Adt0::C5(v34, v35, v36, v37, v38, v39) => {
            f8(v34, v35, v36, v37, v38, v39)
        }
        Adt0::C6(v40, v41, v42, v43, v44, v45) => {
            f9(v40, v41, v42, v43, v44, v45)
        }
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt6> {
    let v2: u64 = f10(v1.clone(), 1u64)?;
    let a8 = match v1.clone() {
        Adt2::C0(h7, _, _, _, _, _, _, _, _, _, _) => {
            h7
        }
    };
    let a9 = 0u64;
    if if if if if if if if nat_eq(a8, a9) {
        let a11 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h10, _, _, _, _, _) => {
                h10
            }
        };
        let a12 = 4u64;
        nat_eq(a11, a12)
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(h13, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h13
            }
        })
    } else {
        false
    } {
        f11(match v0.clone() {
            Adt1::C0(_, h14, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h14
            }
        })
    } else {
        false
    } {
        let a16 = 0u64;
        let a17 = match v0.clone() {
            Adt1::C0(_, _, h15, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h15
            }
        };
        nat_lt(a16, a17)
    } else {
        false
    } {
        let a19 = match v0.clone() {
            Adt1::C0(_, _, h18, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h18
            }
        };
        let a20 = 2139095040u64;
        nat_lt(a19, a20)
    } else {
        false
    } {
        let a21 = v2;
        let a22 = 9223372036854775808u64;
        nat_lt(a21, a22)
    } else {
        false
    } {
        let a24 = f12(v2)?;
        let a25 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h23, _) => {
                h23
            }
        };
        nat_le(a24, a25)
    } else {
        false
    } {
        Ok(Adt6::C4(f13(v2)?, 4u64, f14(v1.clone(), 1u64), f14(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h26, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h26
            }
        }, match v0.clone() {
            Adt1::C0(_, h27, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h27
            }
        }, match v0 {
            Adt1::C0(_, _, h28, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h28
            }
        }, v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f2(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m29 = v1;
    match m29 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let m30 = v2;
            match m30 {
                None => {
                    Ok(Adt6::C1)
                }
                Some(v5) => {
                    let a32 = match v0.clone() {
                        Adt2::C0(h31, _, _, _, _, _, _, _, _, _, _) => {
                            h31
                        }
                    };
                    let a33 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a32, a33) {
                        let a35 = match v4.clone() {
                            Adt2::C0(h34, _, _, _, _, _, _, _, _, _, _) => {
                                h34
                            }
                        };
                        let a36 = 0u64;
                        nat_eq(a35, a36)
                    } else {
                        false
                    } {
                        let a38 = match v5.clone() {
                            Adt2::C0(h37, _, _, _, _, _, _, _, _, _, _) => {
                                h37
                            }
                        };
                        let a39 = 26u64;
                        nat_eq(a38, a39)
                    } else {
                        false
                    } {
                        let a41 = match v3.clone() {
                            Adt2::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                                h40
                            }
                        };
                        let a42 = 0u64;
                        nat_eq(a41, a42)
                    } else {
                        false
                    } {
                        f15(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a45 = match v0.clone() {
                            Adt2::C0(_, h43, _, _, _, _, _, _, _, _, _) => {
                                h43
                            }
                        };
                        let a46 = match v4.clone() {
                            Adt2::C0(_, h44, _, _, _, _, _, _, _, _, _) => {
                                h44
                            }
                        };
                        nat_eq(a45, a46)
                    } else {
                        false
                    } {
                        let a49 = match v0.clone() {
                            Adt2::C0(_, _, h47, _, _, _, _, _, _, _, _) => {
                                h47
                            }
                        };
                        let a50 = match v5.clone() {
                            Adt2::C0(_, h48, _, _, _, _, _, _, _, _, _) => {
                                h48
                            }
                        };
                        nat_eq(a49, a50)
                    } else {
                        false
                    } {
                        let a53 = match v0.clone() {
                            Adt2::C0(_, _, _, h51, _, _, _, _, _, _, _) => {
                                h51
                            }
                        };
                        let a54 = match v5.clone() {
                            Adt2::C0(_, _, h52, _, _, _, _, _, _, _, _) => {
                                h52
                            }
                        };
                        nat_eq(a53, a54)
                    } else {
                        false
                    } {
                        let a56 = 0u64;
                        let a57 = match v5.clone() {
                            Adt2::C0(_, _, _, h55, _, _, _, _, _, _, _) => {
                                h55
                            }
                        };
                        nat_lt(a56, a57)
                    } else {
                        false
                    } {
                        let a59 = 0u64;
                        let a60 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h58, _, _, _, _, _, _) => {
                                h58
                            }
                        };
                        nat_lt(a59, a60)
                    } else {
                        false
                    } {
                        let a62 = 4u64;
                        let a63 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h61, _, _, _, _, _) => {
                                h61
                            }
                        };
                        nat_le(a62, a63)
                    } else {
                        false
                    } {
                        let a65 = 4u64;
                        let a66 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h64, _, _, _, _, _) => {
                                h64
                            }
                        };
                        nat_le(a65, a66)
                    } else {
                        false
                    } {
                        let a68 = 4u64;
                        let a69 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h67, _, _, _, _, _) => {
                                h67
                            }
                        };
                        nat_le(a68, a69)
                    } else {
                        false
                    } {
                        let a71 = 4u64;
                        let a72 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h70, _, _, _, _, _) => {
                                h70
                            }
                        };
                        nat_le(a71, a72)
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
                        Ok(Adt6::C4(f17(match v0.clone() {
                            Adt2::C0(_, h73, _, _, _, _, _, _, _, _, _) => {
                                h73
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h74, _, _, _, _, _, _, _, _) => {
                                h74
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                                h75
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h76, _, _, _, _, _, _) => {
                                h76
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

pub fn f3(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt5> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h77, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h77
        }
    };
    Ok(Adt5::C0({
        let a78 = v0;
        let a79 = 39u64;
        if nat_eq(a78, a79) {
            f18(v2, v3, v4)?
        } else {
            let a80 = v0;
            let a81 = 77u64;
            if nat_eq(a80, a81) {
                f19(v2, v4)?
            } else {
                let a82 = v0;
                let a83 = 40u64;
                if nat_eq(a82, a83) {
                    f20(v2, v3, v4)?
                } else {
                    if f21(v0) {
                        f22(v0, v2, {
                            let m84 = v3;
                            match m84 {
                                None => {
                                    false
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v4)?
                    } else {
                        let a85 = v0;
                        let a86 = 19u64;
                        if nat_eq(a85, a86) {
                            f23(v2, v4)?
                        } else {
                            let a87 = v0;
                            let a88 = 21u64;
                            if nat_eq(a87, a88) {
                                f24(v5, v2, v3, v4)?
                            } else {
                                let a89 = v0;
                                let a90 = 61u64;
                                if if if nat_eq(a89, a90) {
                                    true
                                } else {
                                    let a91 = v0;
                                    let a92 = 62u64;
                                    nat_eq(a91, a92)
                                } {
                                    true
                                } else {
                                    let a93 = v0;
                                    let a94 = 63u64;
                                    nat_eq(a93, a94)
                                } {
                                    f25(v0, v1, v2, v4)?
                                } else {
                                    let a95 = v0;
                                    let a96 = 4u64;
                                    if if nat_eq(a95, a96) {
                                        true
                                    } else {
                                        let a97 = v0;
                                        let a98 = 32u64;
                                        nat_eq(a97, a98)
                                    } {
                                        f26(v0, v1, v2, v3, v4)?
                                    } else {
                                        let a99 = v0;
                                        let a100 = 20u64;
                                        if nat_eq(a99, a100) {
                                            f27(v2, v4)?
                                        } else {
                                            let a101 = v0;
                                            let a102 = 42u64;
                                            if nat_eq(a101, a102) {
                                                f28(4u64, v2, v4)?
                                            } else {
                                                let a103 = v0;
                                                let a104 = 69u64;
                                                if nat_eq(a103, a104) {
                                                    let a105 = v5;
                                                    let a106 = 4u64;
                                                    if nat_lt(a105, a106) {
                                                        f28(v5, v2, v4)?
                                                    } else {
                                                        Adt6::C1
                                                    }
                                                } else {
                                                    let a107 = v0;
                                                    let a108 = 70u64;
                                                    if nat_eq(a107, a108) {
                                                        f29(v5, v2, v4)?
                                                    } else {
                                                        let a109 = v0;
                                                        let a110 = 75u64;
                                                        if if nat_eq(a109, a110) {
                                                            true
                                                        } else {
                                                            let a111 = v0;
                                                            let a112 = 76u64;
                                                            nat_eq(a111, a112)
                                                        } {
                                                            f30({
                                                                let a113 = v0;
                                                                let a114 = 76u64;
                                                                nat_eq(a113, a114)
                                                            }, v1, v2, v4)?
                                                        } else {
                                                            let a115 = v0;
                                                            let a116 = 31u64;
                                                            if if if nat_eq(a115, a116) {
                                                                true
                                                            } else {
                                                                let a117 = v0;
                                                                let a118 = 47u64;
                                                                nat_eq(a117, a118)
                                                            } {
                                                                true
                                                            } else {
                                                                let a119 = v0;
                                                                let a120 = 68u64;
                                                                nat_eq(a119, a120)
                                                            } {
                                                                f31(v0, v1, v2, v4)?
                                                            } else {
                                                                let a121 = v0;
                                                                let a122 = 15u64;
                                                                if nat_eq(a121, a122) {
                                                                    f32(v2, v4)?
                                                                } else {
                                                                    let a123 = v0;
                                                                    let a124 = 17u64;
                                                                    if nat_eq(a123, a124) {
                                                                        f33(v2, v4)?
                                                                    } else {
                                                                        let a125 = v0;
                                                                        let a126 = 18u64;
                                                                        if nat_eq(a125, a126) {
                                                                            f34(v2, v3, v4)?
                                                                        } else {
                                                                            let a127 = v0;
                                                                            let a128 = 13u64;
                                                                            if if if nat_eq(a127, a128) {
                                                                                true
                                                                            } else {
                                                                                let a129 = v0;
                                                                                let a130 = 14u64;
                                                                                nat_eq(a129, a130)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a131 = v0;
                                                                                let a132 = 16u64;
                                                                                nat_eq(a131, a132)
                                                                            } {
                                                                                f35(v0, v2, v4)?
                                                                            } else {
                                                                                let m133 = f36(v0, v5);
                                                                                match m133 {
                                                                                    None => {
                                                                                        f37(v0, v5, v2, v3, v4)?
                                                                                    }
                                                                                    Some(v7) => {
                                                                                        f38(v7, v2, v4)?
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f4(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt5> {
    let m134 = v3;
    match m134 {
        Adt4::C0 => {
            Ok(Adt5::C1(false, f39(v0, v1)?, f39(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt5::C1(false, f41(v0, v1, v2.clone())?, f39(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f42(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f43(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f44(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f43(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f45(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f46(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f47(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt5::C1(true, 0u64, f39(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f48(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt5::C3(v28, v29, f39(v0, v1)?, f39(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f49(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt5::C3(v34, v34, f39(v0, v1)?, f39(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt5::C3(v38, v39, f39(v0, v1)?, f39(v0, v2)?))
        }
        Adt4::C15 => {
            f50(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f51(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C18(v42, v43) => {
            f52(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C21 => {
            Ok(Adt5::C1(false, f39(f40(v0, match v2.clone() {
                Adt3::C0(h135, _, _, _, _, _, _, _) => {
                    h135
                }
            }, match v2.clone() {
                Adt3::C0(_, h136, _, _, _, _, _, _) => {
                    h136
                }
            })?, v1)?, f39(v0, v2)?))
        }
        Adt4::C22(_, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt4::C23(_, _, _, _) => {
            Ok(Adt5::C0(Adt6::C1))
        }
    }
}

pub fn f5(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f53(match v1.clone() {
        Adt3::C0(h137, _, _, _, _, _, _, _) => {
            h137
        }
    })?;
    let v5: u64 = {
        let a138 = v0;
        let a139 = v4;
        let a140 = 0u64;
        nat_quot(a138, a139, a140)
    };
    let v6: u64 = {
        let a141 = v0;
        let a142 = v4;
        let a143 = 0u64;
        nat_rem(a141, a142, a143)
    };
    let a144 = v6;
    let a145 = 0u64;
    if nat_eq(a144, a145) {
        Ok(Adt5::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a146 = v6;
        let a147 = 1u64;
        if nat_eq(a146, a147) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a150 = {
                let a148 = v6;
                let a149 = 1u64;
                nat_add(a148, a149)?
            };
            let a151 = v4;
            if nat_eq(a150, a151) {
                Ok(Adt5::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f39(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a152 = v6;
                    let a153 = 2u64;
                    nat_sub(a152, a153)
                };
                let v8: u64 = {
                    let a154 = v7;
                    let a155 = 3u64;
                    let a156 = 0u64;
                    nat_quot(a154, a155, a156)
                };
                let v9: u64 = {
                    let a157 = v7;
                    let a158 = 3u64;
                    let a159 = 0u64;
                    nat_rem(a157, a158, a159)
                };
                let v10: u64 = f39({
                    let a163 = {
                        let a161 = v5;
                        let a162 = match v1.clone() {
                            Adt3::C0(h160, _, _, _, _, _, _, _) => {
                                h160
                            }
                        };
                        nat_mul(a161, a162)?
                    };
                    let a164 = v8;
                    nat_add(a163, a164)?
                }, v1)?;
                let a165 = v9;
                let a166 = 0u64;
                if nat_eq(a165, a166) {
                    Ok(Adt5::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a167 = v9;
                    let a168 = 1u64;
                    if nat_eq(a167, a168) {
                        let a169 = v3;
                        let a170 = 2u64;
                        if nat_eq(a169, a170) {
                            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt5::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a171 = v3;
                        let a172 = 2u64;
                        if if nat_eq(a171, a172) {
                            true
                        } else {
                            let a173 = v3;
                            let a174 = 3u64;
                            nat_eq(a173, a174)
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

pub fn f6(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a176 = 2u64;
        let a177 = match v1.clone() {
            Adt3::C0(_, h175, _, _, _, _, _, _) => {
                h175
            }
        };
        nat_mul(a176, a177)?
    };
    let a178 = v0;
    let a179 = v5;
    if nat_lt(a178, a179) {
        let a183 = {
            let a180 = v0;
            let a181 = 2u64;
            let a182 = 0u64;
            nat_rem(a180, a181, a182)
        };
        let a184 = 0u64;
        if nat_eq(a183, a184) {
            Ok(Adt5::C4(19u64, 4u64, f39({
                let a185 = v0;
                let a186 = 2u64;
                let a187 = 0u64;
                nat_quot(a185, a186, a187)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a188 = v4;
            let a189 = 2147483648u64;
            if if nat_lt(a188, a189) {
                let a191 = v4;
                let a192 = match v2 {
                    Adt3::C0(_, h190, _, _, _, _, _, _) => {
                        h190
                    }
                };
                nat_lt(a191, a192)
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
            let a195 = match v2.clone() {
                Adt3::C0(h193, _, _, _, _, _, _, _) => {
                    h193
                }
            };
            let a196 = match v2.clone() {
                Adt3::C0(_, h194, _, _, _, _, _, _) => {
                    h194
                }
            };
            nat_mul(a195, a196)?
        };
        let a199 = {
            let a197 = v0;
            let a198 = v5;
            nat_sub(a197, a198)
        };
        let a200 = v6;
        if nat_lt(a199, a200) {
            Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f39({
                let a201 = v0;
                let a202 = v5;
                nat_sub(a201, a202)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a205 = {
                    let a203 = v0;
                    let a204 = v5;
                    nat_sub(a203, a204)
                };
                let a206 = v6;
                nat_sub(a205, a206)
            };
            let v8: u64 = {
                let a210 = 1u64;
                let a211 = {
                    let a208 = 2u64;
                    let a209 = match v1.clone() {
                        Adt3::C0(h207, _, _, _, _, _, _, _) => {
                            h207
                        }
                    };
                    nat_mul(a208, a209)?
                };
                nat_add(a210, a211)?
            };
            let v9: u64 = {
                let a212 = v7;
                let a213 = v8;
                let a214 = 0u64;
                nat_quot(a212, a213, a214)
            };
            let v10: u64 = {
                let a215 = v7;
                let a216 = v8;
                let a217 = 0u64;
                nat_rem(a215, a216, a217)
            };
            let a218 = v10;
            let a219 = 0u64;
            if nat_eq(a218, a219) {
                Ok(Adt5::C4(19u64, 4u64, f39(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a223 = {
                    let a220 = v10;
                    let a221 = 2u64;
                    let a222 = 0u64;
                    nat_rem(a220, a221, a222)
                };
                let a224 = 1u64;
                if nat_eq(a223, a224) {
                    Ok(Adt5::C4(6u64, 0u64, f39({
                        let a233 = {
                            let a226 = v9;
                            let a227 = match v1.clone() {
                                Adt3::C0(h225, _, _, _, _, _, _, _) => {
                                    h225
                                }
                            };
                            nat_mul(a226, a227)?
                        };
                        let a234 = {
                            let a230 = {
                                let a228 = v10;
                                let a229 = 1u64;
                                nat_sub(a228, a229)
                            };
                            let a231 = 2u64;
                            let a232 = 0u64;
                            nat_quot(a230, a231, a232)
                        };
                        nat_add(a233, a234)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt5::C4(4u64, 3u64, f39({
                        let a243 = {
                            let a236 = v4;
                            let a237 = match v1.clone() {
                                Adt3::C0(h235, _, _, _, _, _, _, _) => {
                                    h235
                                }
                            };
                            nat_mul(a236, a237)?
                        };
                        let a244 = {
                            let a240 = {
                                let a238 = v10;
                                let a239 = 1u64;
                                nat_sub(a238, a239)
                            };
                            let a241 = 2u64;
                            let a242 = 0u64;
                            nat_quot(a240, a241, a242)
                        };
                        nat_add(a243, a244)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f39({
                        let a253 = {
                            let a246 = v4;
                            let a247 = match v1 {
                                Adt3::C0(h245, _, _, _, _, _, _, _) => {
                                    h245
                                }
                            };
                            nat_mul(a246, a247)?
                        };
                        let a254 = {
                            let a250 = {
                                let a248 = v10;
                                let a249 = 1u64;
                                nat_sub(a248, a249)
                            };
                            let a251 = 2u64;
                            let a252 = 0u64;
                            nat_quot(a250, a251, a252)
                        };
                        nat_add(a253, a254)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let a255 = v1;
    let a256 = f54(v2.clone(), v4.clone())?;
    if nat_lt(a255, a256) {
        let a259 = v1;
        let a260 = {
            let a257 = 2u64;
            let a258 = f55(v4.clone())?;
            nat_mul(a257, a258)?
        };
        if nat_lt(a259, a260) {
            let a264 = {
                let a261 = v1;
                let a262 = 2u64;
                let a263 = 0u64;
                nat_rem(a261, a262, a263)
            };
            let a265 = 0u64;
            if nat_eq(a264, a265) {
                f56({
                    let a266 = v1;
                    let a267 = 2u64;
                    let a268 = 0u64;
                    nat_quot(a266, a267, a268)
                }, v4)
            } else {
                if f57(match v2 {
                    Adt3::C0(_, h269, _, _, _, _, _, _) => {
                        h269
                    }
                }, v5) {
                    Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt5::C0(Adt6::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a272 = v1;
                let a273 = {
                    let a270 = 2u64;
                    let a271 = f55(v4.clone())?;
                    nat_mul(a270, a271)?
                };
                nat_sub(a272, a273)
            };
            let v7: u64 = {
                let a277 = v6;
                let a278 = {
                    let a275 = 1u64;
                    let a276 = match v2.clone() {
                        Adt3::C0(h274, _, _, _, _, _, _, _) => {
                            h274
                        }
                    };
                    nat_add(a275, a276)?
                };
                let a279 = 0u64;
                nat_quot(a277, a278, a279)
            };
            let v8: u64 = {
                let a283 = v6;
                let a284 = {
                    let a281 = 1u64;
                    let a282 = match v2.clone() {
                        Adt3::C0(h280, _, _, _, _, _, _, _) => {
                            h280
                        }
                    };
                    nat_add(a281, a282)?
                };
                let a285 = 0u64;
                nat_rem(a283, a284, a285)
            };
            let a286 = v8;
            let a287 = 0u64;
            if nat_eq(a286, a287) {
                f56(v7, v4)
            } else {
                f58(v0, v7, {
                    let a288 = v8;
                    let a289 = 1u64;
                    nat_sub(a288, a289)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt5::C0(Adt6::C1))
    }
}

pub fn f8(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a292 = match v4.clone() {
            Adt3::C0(h290, _, _, _, _, _, _, _) => {
                h290
            }
        };
        let a293 = match v4.clone() {
            Adt3::C0(_, h291, _, _, _, _, _, _) => {
                h291
            }
        };
        nat_mul(a292, a293)?
    };
    let v7: u64 = {
        let a294 = 2u64;
        let a295 = v6;
        nat_mul(a294, a295)?
    };
    let v8: u64 = f59(match v3.clone() {
        Adt3::C0(_, h296, _, _, _, _, _, _) => {
            h296
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h297, _, _, _, _, _) => {
            h297
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h298, _, _, _, _) => {
            h298
        }
    });
    let a299 = v0;
    let a300 = v7;
    if nat_lt(a299, a300) {
        let a304 = {
            let a301 = v0;
            let a302 = 2u64;
            let a303 = 0u64;
            nat_rem(a301, a302, a303)
        };
        let a305 = 0u64;
        if nat_eq(a304, a305) {
            Ok(Adt5::C4(19u64, 5u64, f39({
                let a306 = v0;
                let a307 = 2u64;
                let a308 = 0u64;
                nat_quot(a306, a307, a308)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f60(v5, v8) {
                Ok(Adt5::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a309 = v0;
            let a310 = v7;
            nat_sub(a309, a310)
        };
        let v10: u64 = {
            let a314 = v9;
            let a315 = {
                let a312 = 1u64;
                let a313 = match v1.clone() {
                    Adt3::C0(h311, _, _, _, _, _, _, _) => {
                        h311
                    }
                };
                nat_add(a312, a313)?
            };
            let a316 = 0u64;
            nat_quot(a314, a315, a316)
        };
        let v11: u64 = {
            let a320 = v9;
            let a321 = {
                let a318 = 1u64;
                let a319 = match v1.clone() {
                    Adt3::C0(h317, _, _, _, _, _, _, _) => {
                        h317
                    }
                };
                nat_add(a318, a319)?
            };
            let a322 = 0u64;
            nat_rem(a320, a321, a322)
        };
        let a323 = v11;
        let a324 = 0u64;
        if nat_eq(a323, a324) {
            Ok(Adt5::C4(19u64, 5u64, f39({
                let a325 = v10;
                let a326 = v6;
                let a327 = 0u64;
                nat_rem(a325, a326, a327)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f60(v5, v8) {
                let v12: u64 = {
                    let a333 = {
                        let a329 = v10;
                        let a330 = match v1.clone() {
                            Adt3::C0(h328, _, _, _, _, _, _, _) => {
                                h328
                            }
                        };
                        nat_mul(a329, a330)?
                    };
                    let a334 = {
                        let a331 = v11;
                        let a332 = 1u64;
                        nat_sub(a331, a332)
                    };
                    nat_add(a333, a334)?
                };
                let a335 = v5;
                let a336 = 4294967295u64;
                if nat_eq(a335, a336) {
                    Ok(Adt5::C1(false, f39(v12, v1)?, f39(v12, v2)?))
                } else {
                    Ok(Adt5::C4(8u64, 0u64, f39(v12, v1)?, 4u64, f39({
                        let a342 = {
                            let a338 = v5;
                            let a339 = match v3.clone() {
                                Adt3::C0(h337, _, _, _, _, _, _, _) => {
                                    h337
                                }
                            };
                            nat_mul(a338, a339)?
                        };
                        let a343 = {
                            let a340 = v11;
                            let a341 = 1u64;
                            nat_sub(a340, a341)
                        };
                        nat_add(a342, a343)?
                    }, v3)?, 3u64, f39(v12, v2)?))
                }
            } else {
                Ok(Adt5::C0(Adt6::C1))
            }
        }
    }
}

pub fn f9(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt5> {
    let a344 = v0;
    let a345 = f13(v1)?;
    if nat_lt(a344, a345) {
        let a346 = v0;
        let a347 = 4u64;
        if nat_lt(a346, a347) {
            let a348 = v0;
            let a349 = 0u64;
            if nat_eq(a348, a349) {
                Ok(Adt5::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a350 = v0;
                let a351 = 1u64;
                if nat_eq(a350, a351) {
                    let a352 = v5;
                    let a353 = 0u64;
                    if nat_eq(a352, a353) {
                        Ok(Adt5::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C0(Adt6::C1))
                    }
                } else {
                    let a354 = v0;
                    let a355 = 2u64;
                    if nat_eq(a354, a355) {
                        Ok(Adt5::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt5::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a356 = v0;
            let a357 = 4u64;
            if if nat_eq(a356, a357) {
                let a358 = f61(v5)?;
                let a359 = v1;
                !nat_eq(a358, a359)
            } else {
                false
            } {
                Ok(Adt5::C0(Adt6::C1))
            } else {
                let v6: u64 = {
                    let a362 = {
                        let a360 = v0;
                        let a361 = 4u64;
                        nat_sub(a360, a361)
                    };
                    let a363 = 3u64;
                    let a364 = 0u64;
                    nat_quot(a362, a363, a364)
                };
                let v7: u64 = {
                    let a367 = {
                        let a365 = v0;
                        let a366 = 4u64;
                        nat_sub(a365, a366)
                    };
                    let a368 = 3u64;
                    let a369 = 0u64;
                    nat_rem(a367, a368, a369)
                };
                let a370 = v7;
                let a371 = 0u64;
                if nat_eq(a370, a371) {
                    Ok(Adt5::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a372 = v7;
                    let a373 = 1u64;
                    if nat_eq(a372, a373) {
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

pub fn f10(v0: Adt2, v1: u64) -> R<u64> {
    let a385 = {
        let a379 = {
            let a375 = match v0.clone() {
                Adt2::C0(_, h374, _, _, _, _, _, _, _, _, _) => {
                    h374
                }
            };
            let a376 = v1;
            let a377 = 0u64;
            nat_quot(a375, a376, a377)
        };
        let a380 = match v0.clone() {
            Adt2::C0(_, _, h378, _, _, _, _, _, _, _, _) => {
                h378
            }
        };
        nat_mul(a379, a380)?
    };
    let a386 = {
        let a383 = match v0.clone() {
            Adt2::C0(_, _, _, h381, _, _, _, _, _, _, _) => {
                h381
            }
        };
        let a384 = match v0 {
            Adt2::C0(_, _, _, _, h382, _, _, _, _, _, _) => {
                h382
            }
        };
        nat_mul(a383, a384)?
    };
    nat_mul(a385, a386)
}

pub fn f11(v0: u64) -> bool {
    let a387 = v0;
    let a388 = 4294967296u64;
    if nat_lt(a387, a388) {
        let a392 = {
            let a389 = v0;
            let a390 = 2147483648u64;
            let a391 = 0u64;
            nat_rem(a389, a390, a391)
        };
        let a393 = 2139095040u64;
        nat_lt(a392, a393)
    } else {
        false
    }
}

pub fn f12(v0: u64) -> R<u64> {
    let a394 = 4u64;
    let a395 = v0;
    nat_mul(a394, a395)
}

pub fn f13(v0: u64) -> R<u64> {
    let a396 = v0;
    let a397 = 0u64;
    if nat_eq(a396, a397) {
        Ok(0u64)
    } else {
        let a400 = 4u64;
        let a401 = {
            let a398 = 3u64;
            let a399 = v0;
            nat_mul(a398, a399)?
        };
        nat_add(a400, a401)
    }
}

pub fn f14(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a403 = match v0.clone() {
            Adt2::C0(_, h402, _, _, _, _, _, _, _, _, _) => {
                h402
            }
        };
        let a404 = v1;
        let a405 = 0u64;
        nat_quot(a403, a404, a405)
    }, match v0.clone() {
        Adt2::C0(_, _, h406, _, _, _, _, _, _, _, _) => {
            h406
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h407, _, _, _, _, _, _, _) => {
            h407
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h408, _, _, _, _, _, _) => {
            h408
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h409, _, _, _, _, _) => {
            h409
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h410, _, _, _, _) => {
            h410
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h411, _, _, _) => {
            h411
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h412, _, _) => {
            h412
        }
    })
}

pub fn f15(v0: Adt2, v1: Adt2) -> bool {
    let a415 = match v0.clone() {
        Adt2::C0(_, h413, _, _, _, _, _, _, _, _, _) => {
            h413
        }
    };
    let a416 = match v1.clone() {
        Adt2::C0(_, h414, _, _, _, _, _, _, _, _, _) => {
            h414
        }
    };
    if if if nat_eq(a415, a416) {
        let a419 = match v0.clone() {
            Adt2::C0(_, _, h417, _, _, _, _, _, _, _, _) => {
                h417
            }
        };
        let a420 = match v1.clone() {
            Adt2::C0(_, _, h418, _, _, _, _, _, _, _, _) => {
                h418
            }
        };
        nat_eq(a419, a420)
    } else {
        false
    } {
        let a423 = match v0.clone() {
            Adt2::C0(_, _, _, h421, _, _, _, _, _, _, _) => {
                h421
            }
        };
        let a424 = match v1.clone() {
            Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                h422
            }
        };
        nat_eq(a423, a424)
    } else {
        false
    } {
        let a427 = match v0 {
            Adt2::C0(_, _, _, _, h425, _, _, _, _, _, _) => {
                h425
            }
        };
        let a428 = match v1 {
            Adt2::C0(_, _, _, _, h426, _, _, _, _, _, _) => {
                h426
            }
        };
        nat_eq(a427, a428)
    } else {
        false
    }
}

pub fn f16(v0: Adt2, v1: Adt2) -> R<bool> {
    let a430 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h429, _) => {
            h429
        }
    };
    let a431 = 0u64;
    if nat_eq(a430, a431) {
        Ok(true)
    } else {
        let a433 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h432, _) => {
                h432
            }
        };
        let a434 = 0u64;
        if nat_eq(a433, a434) {
            Ok(true)
        } else {
            let a436 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h435) => {
                    h435
                }
            };
            let a437 = 0u64;
            if nat_eq(a436, a437) {
                Ok(true)
            } else {
                let a439 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h438) => {
                        h438
                    }
                };
                let a440 = 0u64;
                if nat_eq(a439, a440) {
                    Ok(true)
                } else {
                    let a446 = {
                        let a443 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h441) => {
                                h441
                            }
                        };
                        let a444 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h442, _) => {
                                h442
                            }
                        };
                        nat_add(a443, a444)?
                    };
                    let a447 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h445) => {
                            h445
                        }
                    };
                    if nat_le(a446, a447) {
                        Ok(true)
                    } else {
                        let a453 = {
                            let a450 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h448) => {
                                    h448
                                }
                            };
                            let a451 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h449, _) => {
                                    h449
                                }
                            };
                            nat_add(a450, a451)?
                        };
                        let a454 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h452) => {
                                h452
                            }
                        };
                        Ok(nat_le(a453, a454))
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a455 = v0;
    let a456 = 0u64;
    if if if if nat_eq(a455, a456) {
        true
    } else {
        let a457 = v1;
        let a458 = 0u64;
        nat_eq(a457, a458)
    } {
        true
    } else {
        let a459 = v2;
        let a460 = 0u64;
        nat_eq(a459, a460)
    } {
        true
    } else {
        let a461 = v3;
        let a462 = 0u64;
        nat_eq(a461, a462)
    } {
        Ok(0u64)
    } else {
        let a475 = {
            let a465 = 2u64;
            let a466 = {
                let a463 = v1;
                let a464 = v2;
                nat_mul(a463, a464)?
            };
            nat_mul(a465, a466)?
        };
        let a476 = {
            let a473 = {
                let a469 = {
                    let a467 = v1;
                    let a468 = v2;
                    nat_mul(a467, a468)?
                };
                let a470 = v3;
                nat_mul(a469, a470)?
            };
            let a474 = {
                let a471 = 1u64;
                let a472 = v0;
                nat_add(a471, a472)?
            };
            nat_mul(a473, a474)?
        };
        nat_add(a475, a476)
    }
}

pub fn f18(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m477 = v1;
    match m477 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            f62(v0, v3, v2)
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a479 = match v0.clone() {
        Adt2::C0(h478, _, _, _, _, _, _, _, _, _, _) => {
            h478
        }
    };
    let a480 = 1u64;
    if if nat_eq(a479, a480) {
        true
    } else {
        let a482 = match v0.clone() {
            Adt2::C0(h481, _, _, _, _, _, _, _, _, _, _) => {
                h481
            }
        };
        let a483 = 30u64;
        nat_eq(a482, a483)
    } {
        let a485 = match v1.clone() {
            Adt2::C0(h484, _, _, _, _, _, _, _, _, _, _) => {
                h484
            }
        };
        let a486 = 1u64;
        if if if if if if if if if if if nat_eq(a485, a486) {
            let a489 = match v0.clone() {
                Adt2::C0(_, h487, _, _, _, _, _, _, _, _, _) => {
                    h487
                }
            };
            let a490 = match v1.clone() {
                Adt2::C0(_, h488, _, _, _, _, _, _, _, _, _) => {
                    h488
                }
            };
            nat_eq(a489, a490)
        } else {
            false
        } {
            let a492 = 0u64;
            let a493 = match v1.clone() {
                Adt2::C0(_, _, h491, _, _, _, _, _, _, _, _) => {
                    h491
                }
            };
            nat_lt(a492, a493)
        } else {
            false
        } {
            let a496 = match v1.clone() {
                Adt2::C0(_, _, h494, _, _, _, _, _, _, _, _) => {
                    h494
                }
            };
            let a497 = match v1.clone() {
                Adt2::C0(_, _, _, h495, _, _, _, _, _, _, _) => {
                    h495
                }
            };
            nat_eq(a496, a497)
        } else {
            false
        } {
            let a499 = match v1.clone() {
                Adt2::C0(_, _, _, _, h498, _, _, _, _, _, _) => {
                    h498
                }
            };
            let a500 = 1u64;
            nat_eq(a499, a500)
        } else {
            false
        } {
            let a508 = match v0.clone() {
                Adt2::C0(_, _, h501, _, _, _, _, _, _, _, _) => {
                    h501
                }
            };
            let a509 = {
                let a506 = {
                    let a504 = match v1.clone() {
                        Adt2::C0(_, _, h502, _, _, _, _, _, _, _, _) => {
                            h502
                        }
                    };
                    let a505 = match v1.clone() {
                        Adt2::C0(_, _, h503, _, _, _, _, _, _, _, _) => {
                            h503
                        }
                    };
                    nat_add(a504, a505)?
                };
                let a507 = 1u64;
                nat_sub(a506, a507)
            };
            nat_eq(a508, a509)
        } else {
            false
        } {
            let a511 = 0u64;
            let a512 = match v0.clone() {
                Adt2::C0(_, _, _, h510, _, _, _, _, _, _, _) => {
                    h510
                }
            };
            nat_lt(a511, a512)
        } else {
            false
        } {
            let a514 = 0u64;
            let a515 = match v0.clone() {
                Adt2::C0(_, _, _, _, h513, _, _, _, _, _, _) => {
                    h513
                }
            };
            nat_lt(a514, a515)
        } else {
            false
        } {
            let a517 = 2u64;
            let a518 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h516, _, _, _, _, _) => {
                    h516
                }
            };
            nat_le(a517, a518)
        } else {
            false
        } {
            let a520 = 2u64;
            let a521 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h519, _, _, _, _, _) => {
                    h519
                }
            };
            nat_le(a520, a521)
        } else {
            false
        } {
            f16(v0.clone(), v1.clone())?
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

pub fn f20(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m522 = v1;
    match m522 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let a524 = match v0.clone() {
                Adt2::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                    h523
                }
            };
            let a525 = 0u64;
            if if if if if if if if if if if if if nat_eq(a524, a525) {
                true
            } else {
                let a527 = match v0.clone() {
                    Adt2::C0(h526, _, _, _, _, _, _, _, _, _, _) => {
                        h526
                    }
                };
                let a528 = 1u64;
                nat_eq(a527, a528)
            } {
                let a530 = match v3.clone() {
                    Adt2::C0(h529, _, _, _, _, _, _, _, _, _, _) => {
                        h529
                    }
                };
                let a531 = 26u64;
                nat_eq(a530, a531)
            } else {
                false
            } {
                let a533 = match v2.clone() {
                    Adt2::C0(h532, _, _, _, _, _, _, _, _, _, _) => {
                        h532
                    }
                };
                let a534 = 0u64;
                nat_eq(a533, a534)
            } else {
                false
            } {
                let a537 = match v0.clone() {
                    Adt2::C0(_, h535, _, _, _, _, _, _, _, _, _) => {
                        h535
                    }
                };
                let a538 = match v2.clone() {
                    Adt2::C0(_, h536, _, _, _, _, _, _, _, _, _) => {
                        h536
                    }
                };
                nat_eq(a537, a538)
            } else {
                false
            } {
                let a541 = match v3.clone() {
                    Adt2::C0(_, h539, _, _, _, _, _, _, _, _, _) => {
                        h539
                    }
                };
                let a542 = match v0.clone() {
                    Adt2::C0(_, _, h540, _, _, _, _, _, _, _, _) => {
                        h540
                    }
                };
                nat_le(a541, a542)
            } else {
                false
            } {
                let a544 = match v0.clone() {
                    Adt2::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                        h543
                    }
                };
                let a545 = 1u64;
                nat_eq(a544, a545)
            } else {
                false
            } {
                let a547 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h546, _, _, _, _, _, _) => {
                        h546
                    }
                };
                let a548 = 1u64;
                nat_eq(a547, a548)
            } else {
                false
            } {
                let a550 = match v2.clone() {
                    Adt2::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                        h549
                    }
                };
                let a551 = 1u64;
                nat_eq(a550, a551)
            } else {
                false
            } {
                let a553 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h552, _, _, _, _, _, _) => {
                        h552
                    }
                };
                let a554 = 1u64;
                nat_eq(a553, a554)
            } else {
                false
            } {
                let a556 = match v3.clone() {
                    Adt2::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                        h555
                    }
                };
                let a557 = 1u64;
                nat_eq(a556, a557)
            } else {
                false
            } {
                let a559 = match v3.clone() {
                    Adt2::C0(_, _, _, h558, _, _, _, _, _, _, _) => {
                        h558
                    }
                };
                let a560 = 1u64;
                nat_eq(a559, a560)
            } else {
                false
            } {
                let a562 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h561, _, _, _, _, _, _) => {
                        h561
                    }
                };
                let a563 = 1u64;
                nat_eq(a562, a563)
            } else {
                false
            } {
                if f63(match v2.clone() {
                    Adt2::C0(_, h564, _, _, _, _, _, _, _, _, _) => {
                        h564
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h565, _, _, _, _, _, _, _, _) => {
                        h565
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h566, _, _, _, _, _, _, _) => {
                        h566
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, _, h567, _, _, _, _, _, _) => {
                        h567
                    }
                }) {
                    Ok(Adt6::C4(0u64, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
                } else {
                    let a569 = match v3.clone() {
                        Adt2::C0(_, h568, _, _, _, _, _, _, _, _, _) => {
                            h568
                        }
                    };
                    let a570 = 0u64;
                    if nat_eq(a569, a570) {
                        Ok(Adt6::C4(f10(v2.clone(), 1u64)?, 4u64, f14(v0, 1u64), f14(v2, 1u64), Adt4::C9(0u64, 0u64)))
                    } else {
                        let a572 = match v0.clone() {
                            Adt2::C0(_, h571, _, _, _, _, _, _, _, _, _) => {
                                h571
                            }
                        };
                        let a573 = 2147483647u64;
                        if if if if if if nat_le(a572, a573) {
                            let a575 = match v3.clone() {
                                Adt2::C0(_, h574, _, _, _, _, _, _, _, _, _) => {
                                    h574
                                }
                            };
                            let a576 = 2147483647u64;
                            nat_le(a575, a576)
                        } else {
                            false
                        } {
                            let a578 = 4u64;
                            let a579 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h577, _, _, _, _, _) => {
                                    h577
                                }
                            };
                            nat_le(a578, a579)
                        } else {
                            false
                        } {
                            let a585 = {
                                let a582 = match v2.clone() {
                                    Adt2::C0(_, h580, _, _, _, _, _, _, _, _, _) => {
                                        h580
                                    }
                                };
                                let a583 = match v2.clone() {
                                    Adt2::C0(_, _, _, _, _, h581, _, _, _, _, _) => {
                                        h581
                                    }
                                };
                                nat_mul(a582, a583)?
                            };
                            let a586 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, _, h584, _, _, _, _) => {
                                    h584
                                }
                            };
                            nat_le(a585, a586)
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
                                let a600 = {
                                    let a590 = {
                                        let a588 = 2u64;
                                        let a589 = match v3.clone() {
                                            Adt2::C0(_, h587, _, _, _, _, _, _, _, _, _) => {
                                                h587
                                            }
                                        };
                                        nat_mul(a588, a589)?
                                    };
                                    let a591 = f10(v2.clone(), 1u64)?;
                                    nat_add(a590, a591)?
                                };
                                let a601 = {
                                    let a598 = match v3.clone() {
                                        Adt2::C0(_, h592, _, _, _, _, _, _, _, _, _) => {
                                            h592
                                        }
                                    };
                                    let a599 = {
                                        let a596 = 1u64;
                                        let a597 = {
                                            let a594 = 2u64;
                                            let a595 = match v0.clone() {
                                                Adt2::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
                                                    h593
                                                }
                                            };
                                            nat_mul(a594, a595)?
                                        };
                                        nat_add(a596, a597)?
                                    };
                                    nat_mul(a598, a599)?
                                };
                                nat_add(a600, a601)?
                            }, 4u64, Adt3::C0(match v0.clone() {
                                Adt2::C0(_, h602, _, _, _, _, _, _, _, _, _) => {
                                    h602
                                }
                            }, match v3.clone() {
                                Adt2::C0(_, h603, _, _, _, _, _, _, _, _, _) => {
                                    h603
                                }
                            }, 1u64, 1u64, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h604, _, _, _, _, _) => {
                                    h604
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, h605, _, _, _, _) => {
                                    h605
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, h606, _, _, _) => {
                                    h606
                                }
                            }, match v0.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, h607, _, _) => {
                                    h607
                                }
                            }), f14(v2, 1u64), Adt4::C19(match v0 {
                                Adt2::C0(h608, _, _, _, _, _, _, _, _, _, _) => {
                                    h608
                                }
                            }, f14(v3, 1u64))))
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

pub fn f21(v0: u64) -> bool {
    let a609 = v0;
    let a610 = 1u64;
    if if nat_eq(a609, a610) {
        true
    } else {
        let a611 = v0;
        let a612 = 33u64;
        nat_eq(a611, a612)
    } {
        true
    } else {
        let a613 = v0;
        let a614 = 34u64;
        nat_eq(a613, a614)
    }
}

pub fn f22(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a617 = match v1.clone() {
        Adt2::C0(h615, _, _, _, _, _, _, _, _, _, _) => {
            h615
        }
    };
    let a618 = match v3.clone() {
        Adt2::C0(h616, _, _, _, _, _, _, _, _, _, _) => {
            h616
        }
    };
    if nat_eq(a617, a618) {
        f64(v0, v1, v2, v3)
    } else {
        if if if if f65(match v1.clone() {
            Adt2::C0(h619, _, _, _, _, _, _, _, _, _, _) => {
                h619
            }
        }) {
            f65(match v3.clone() {
                Adt2::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                    h620
                }
            })
        } else {
            false
        } {
            let a621 = v0;
            let a622 = 33u64;
            if nat_eq(a621, a622) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a623 = f10(v1.clone(), 1u64)?;
            let a624 = f10(v3.clone(), 1u64)?;
            nat_eq(a623, a624)
        } else {
            false
        } {
            let m626 = f66(match v3.clone() {
                Adt2::C0(h625, _, _, _, _, _, _, _, _, _, _) => {
                    h625
                }
            });
            match m626 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v4) => {
                    Ok(Adt6::C4(f10(v3.clone(), 1u64)?, match v4 {
                        Adt8::C0(_, _, h627) => {
                            h627
                        }
                    }, f14(v1.clone(), 1u64), f14(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h628, _, _, _, _, _, _, _, _, _, _) => {
                            h628
                        }
                    }, match v3 {
                        Adt2::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                            h629
                        }
                    })))
                }
            }
        } else {
            Ok(Adt6::C1)
        }
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a631 = match v0.clone() {
        Adt2::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
            h630
        }
    };
    let a632 = 0u64;
    if if if if nat_eq(a631, a632) {
        true
    } else {
        let a634 = match v0.clone() {
            Adt2::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                h633
            }
        };
        let a635 = 1u64;
        nat_eq(a634, a635)
    } {
        true
    } else {
        let a637 = match v0.clone() {
            Adt2::C0(h636, _, _, _, _, _, _, _, _, _, _) => {
                h636
            }
        };
        let a638 = 25u64;
        if nat_eq(a637, a638) {
            true
        } else {
            let a640 = match v0.clone() {
                Adt2::C0(h639, _, _, _, _, _, _, _, _, _, _) => {
                    h639
                }
            };
            let a641 = 26u64;
            nat_eq(a640, a641)
        }
    } {
        true
    } else {
        let a643 = match v0.clone() {
            Adt2::C0(h642, _, _, _, _, _, _, _, _, _, _) => {
                h642
            }
        };
        let a644 = 30u64;
        nat_eq(a643, a644)
    } {
        let a647 = match v0.clone() {
            Adt2::C0(h645, _, _, _, _, _, _, _, _, _, _) => {
                h645
            }
        };
        let a648 = match v1.clone() {
            Adt2::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
                h646
            }
        };
        if if nat_eq(a647, a648) {
            f67(match v1.clone() {
                Adt2::C0(_, h649, _, _, _, _, _, _, _, _, _) => {
                    h649
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h650, _, _, _, _, _, _, _, _) => {
                    h650
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                    h651
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h652, _, _, _, _, _, _) => {
                    h652
                }
            }, match v0.clone() {
                Adt2::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
                    h653
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h654, _, _, _, _, _, _, _, _) => {
                    h654
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h655, _, _, _, _, _, _, _) => {
                    h655
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h656, _, _, _, _, _, _) => {
                    h656
                }
            })
        } else {
            false
        } {
            let m658 = f66(match v0.clone() {
                Adt2::C0(h657, _, _, _, _, _, _, _, _, _, _) => {
                    h657
                }
            });
            match m658 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v2) => {
                    Ok(Adt6::C4(f10(v1.clone(), 1u64)?, match v2 {
                        Adt8::C0(_, _, h659) => {
                            h659
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

pub fn f24(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt6> {
    let m660 = v2;
    match m660 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v4) => {
            let a661 = v0;
            let a662 = 4u64;
            if if if nat_lt(a661, a662) {
                let a665 = match v1.clone() {
                    Adt2::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
                        h663
                    }
                };
                let a666 = match v4.clone() {
                    Adt2::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
                        h664
                    }
                };
                if nat_eq(a665, a666) {
                    let a669 = match v1.clone() {
                        Adt2::C0(h667, _, _, _, _, _, _, _, _, _, _) => {
                            h667
                        }
                    };
                    let a670 = match v3.clone() {
                        Adt2::C0(h668, _, _, _, _, _, _, _, _, _, _) => {
                            h668
                        }
                    };
                    nat_eq(a669, a670)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f68(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h671, _, _, _, _, _, _, _, _, _) => {
                        h671
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h672, _, _, _, _, _, _, _, _, _) => {
                        h672
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h673, _, _, _, _, _, _, _, _, _) => {
                        h673
                    }
                })? {
                    f68(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h674, _, _, _, _, _, _, _, _) => {
                            h674
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h675, _, _, _, _, _, _, _, _) => {
                            h675
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h676, _, _, _, _, _, _, _, _) => {
                            h676
                        }
                    })?
                } else {
                    false
                } {
                    f68(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h677, _, _, _, _, _, _, _) => {
                            h677
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h678, _, _, _, _, _, _, _) => {
                            h678
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h679, _, _, _, _, _, _, _) => {
                            h679
                        }
                    })?
                } else {
                    false
                } {
                    f68(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h680, _, _, _, _, _, _) => {
                            h680
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h681, _, _, _, _, _, _) => {
                            h681
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h682, _, _, _, _, _, _) => {
                            h682
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m684 = f66(match v1.clone() {
                    Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                        h683
                    }
                });
                match m684 {
                    Adt7::C0 => {
                        Ok(Adt6::C0)
                    }
                    Adt7::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt8::C0(_, h685, _) => {
                                h685
                            }
                        };
                        let a690 = {
                            let a687 = match v1.clone() {
                                Adt2::C0(_, h686, _, _, _, _, _, _, _, _, _) => {
                                    h686
                                }
                            };
                            let a688 = v6;
                            let a689 = 1u64;
                            nat_rem(a687, a688, a689)
                        };
                        let a691 = 0u64;
                        if if if nat_eq(a690, a691) {
                            let a696 = {
                                let a693 = match v4.clone() {
                                    Adt2::C0(_, h692, _, _, _, _, _, _, _, _, _) => {
                                        h692
                                    }
                                };
                                let a694 = v6;
                                let a695 = 1u64;
                                nat_rem(a693, a694, a695)
                            };
                            let a697 = 0u64;
                            nat_eq(a696, a697)
                        } else {
                            false
                        } {
                            let a702 = {
                                let a699 = match v3.clone() {
                                    Adt2::C0(_, h698, _, _, _, _, _, _, _, _, _) => {
                                        h698
                                    }
                                };
                                let a700 = v6;
                                let a701 = 1u64;
                                nat_rem(a699, a700, a701)
                            };
                            let a703 = 0u64;
                            nat_eq(a702, a703)
                        } else {
                            false
                        } {
                            Ok(Adt6::C4(f10(v3.clone(), v6)?, match v5 {
                                Adt8::C0(_, _, h704) => {
                                    h704
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

pub fn f25(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a706 = match v2.clone() {
        Adt2::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
            h705
        }
    };
    let a707 = 0u64;
    if if nat_eq(a706, a707) {
        let a709 = match v3.clone() {
            Adt2::C0(h708, _, _, _, _, _, _, _, _, _, _) => {
                h708
            }
        };
        let a710 = 0u64;
        nat_eq(a709, a710)
    } else {
        false
    } {
        let a711 = v0;
        let a712 = 61u64;
        if nat_eq(a711, a712) {
            let v4: bool = {
                let a714 = 0u64;
                let a715 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h713, _, _, _, _, _, _, _) => {
                        h713
                    }
                };
                nat_lt(a714, a715)
            };
            if if if if if if f69(match v2.clone() {
                Adt2::C0(_, h716, _, _, _, _, _, _, _, _, _) => {
                    h716
                }
            }, match v1.clone() {
                Adt1::C0(h717, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h717
                }
            }, match v1.clone() {
                Adt1::C0(_, h718, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h718
                }
            }, match v3.clone() {
                Adt2::C0(_, h719, _, _, _, _, _, _, _, _, _) => {
                    h719
                }
            })? {
                f69(match v2.clone() {
                    Adt2::C0(_, _, h720, _, _, _, _, _, _, _, _) => {
                        h720
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h721, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h721
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h722, _, _, _, _, _, _, _, _, _, _, _, _) => {
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
                f69(match v2.clone() {
                    Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
                        h724
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h725, _, _, _, _, _, _, _, _, _, _, _) => {
                        h725
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h726, _, _, _, _, _, _, _, _, _, _) => {
                        h726
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h727, _, _, _, _, _, _, _) => {
                        h727
                    }
                })?
            } else {
                false
            } {
                f69(match v2.clone() {
                    Adt2::C0(_, _, _, _, h728, _, _, _, _, _, _) => {
                        h728
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h729, _, _, _, _, _, _, _, _, _) => {
                        h729
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h730, _, _, _, _, _, _, _, _) => {
                        h730
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h731, _, _, _, _, _, _) => {
                        h731
                    }
                })?
            } else {
                false
            } {
                let a733 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h732, _, _, _, _, _, _, _) => {
                        h732
                    }
                };
                let a734 = 4294967295u64;
                nat_le(a733, a734)
            } else {
                false
            } {
                if v4 {
                    !f63(match v2.clone() {
                        Adt2::C0(_, h735, _, _, _, _, _, _, _, _, _) => {
                            h735
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h736, _, _, _, _, _, _, _, _) => {
                            h736
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h737, _, _, _, _, _, _, _) => {
                            h737
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h738, _, _, _, _, _, _) => {
                            h738
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h739
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h740, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h740
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h741, _, _, _, _, _, _, _, _, _, _, _) => {
                        h741
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h742, _, _, _, _, _, _, _, _, _) => {
                        h742
                    }
                }, v4)))
            } else {
                Ok(Adt6::C1)
            }
        } else {
            let a743 = v0;
            let a744 = 62u64;
            if nat_eq(a743, a744) {
                let a746 = match v1.clone() {
                    Adt1::C0(h745, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h745
                    }
                };
                let a747 = 2147483648u64;
                if if if if if nat_lt(a746, a747) {
                    let a749 = match v1.clone() {
                        Adt1::C0(_, h748, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h748
                        }
                    };
                    let a750 = 2147483648u64;
                    nat_lt(a749, a750)
                } else {
                    false
                } {
                    let a753 = match v1.clone() {
                        Adt1::C0(h751, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h751
                        }
                    };
                    let a754 = match v2.clone() {
                        Adt2::C0(_, h752, _, _, _, _, _, _, _, _, _) => {
                            h752
                        }
                    };
                    nat_lt(a753, a754)
                } else {
                    false
                } {
                    let a757 = match v1.clone() {
                        Adt1::C0(_, h755, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h755
                        }
                    };
                    let a758 = match v2.clone() {
                        Adt2::C0(_, h756, _, _, _, _, _, _, _, _, _) => {
                            h756
                        }
                    };
                    nat_lt(a757, a758)
                } else {
                    false
                } {
                    let a761 = match v2.clone() {
                        Adt2::C0(_, _, h759, _, _, _, _, _, _, _, _) => {
                            h759
                        }
                    };
                    let a762 = match v3.clone() {
                        Adt2::C0(_, _, h760, _, _, _, _, _, _, _, _) => {
                            h760
                        }
                    };
                    if if if nat_eq(a761, a762) {
                        let a765 = match v2.clone() {
                            Adt2::C0(_, _, _, h763, _, _, _, _, _, _, _) => {
                                h763
                            }
                        };
                        let a766 = match v3.clone() {
                            Adt2::C0(_, _, _, h764, _, _, _, _, _, _, _) => {
                                h764
                            }
                        };
                        nat_eq(a765, a766)
                    } else {
                        false
                    } {
                        let a769 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h767, _, _, _, _, _, _) => {
                                h767
                            }
                        };
                        let a770 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h768, _, _, _, _, _, _) => {
                                h768
                            }
                        };
                        nat_eq(a769, a770)
                    } else {
                        false
                    } {
                        let a779 = match v3.clone() {
                            Adt2::C0(_, h771, _, _, _, _, _, _, _, _, _) => {
                                h771
                            }
                        };
                        let a780 = {
                            let a777 = {
                                let a774 = match v2.clone() {
                                    Adt2::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                                        h772
                                    }
                                };
                                let a775 = match v1.clone() {
                                    Adt1::C0(h773, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h773
                                    }
                                };
                                nat_add(a774, a775)?
                            };
                            let a778 = match v1.clone() {
                                Adt1::C0(_, h776, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h776
                                }
                            };
                            nat_add(a777, a778)?
                        };
                        nat_eq(a779, a780)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h781, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h781
                        }
                    })))
                } else {
                    Ok(Adt6::C1)
                }
            } else {
                let a782 = v0;
                let a783 = 63u64;
                if nat_eq(a782, a783) {
                    if if if if if f70(match v2.clone() {
                        Adt2::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                            h784
                        }
                    }, match v1.clone() {
                        Adt1::C0(h785, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h785
                        }
                    })? {
                        f70(match v2.clone() {
                            Adt2::C0(_, _, h786, _, _, _, _, _, _, _, _) => {
                                h786
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h787, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h787
                            }
                        })?
                    } else {
                        false
                    } {
                        f70(match v2.clone() {
                            Adt2::C0(_, _, _, h788, _, _, _, _, _, _, _) => {
                                h788
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h789, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h789
                            }
                        })?
                    } else {
                        false
                    } {
                        f70(match v2.clone() {
                            Adt2::C0(_, _, _, _, h790, _, _, _, _, _, _) => {
                                h790
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h791, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h791
                            }
                        })?
                    } else {
                        false
                    } {
                        let a794 = match v2.clone() {
                            Adt2::C0(_, h792, _, _, _, _, _, _, _, _, _) => {
                                h792
                            }
                        };
                        let a795 = match v3.clone() {
                            Adt2::C0(_, h793, _, _, _, _, _, _, _, _, _) => {
                                h793
                            }
                        };
                        if if if nat_eq(a794, a795) {
                            let a798 = match v2.clone() {
                                Adt2::C0(_, _, h796, _, _, _, _, _, _, _, _) => {
                                    h796
                                }
                            };
                            let a799 = match v3.clone() {
                                Adt2::C0(_, _, h797, _, _, _, _, _, _, _, _) => {
                                    h797
                                }
                            };
                            nat_eq(a798, a799)
                        } else {
                            false
                        } {
                            let a802 = match v2.clone() {
                                Adt2::C0(_, _, _, h800, _, _, _, _, _, _, _) => {
                                    h800
                                }
                            };
                            let a803 = match v3.clone() {
                                Adt2::C0(_, _, _, h801, _, _, _, _, _, _, _) => {
                                    h801
                                }
                            };
                            nat_eq(a802, a803)
                        } else {
                            false
                        } {
                            let a806 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h804, _, _, _, _, _, _) => {
                                    h804
                                }
                            };
                            let a807 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h805, _, _, _, _, _, _) => {
                                    h805
                                }
                            };
                            nat_eq(a806, a807)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt6::C4(f10(v3.clone(), 1u64)?, 4u64, f14(v2, 1u64), f14(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h808, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h808
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h809, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h809
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h810, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h810
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h811, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                h811
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

pub fn f26(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m812 = v3;
    match m812 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a813 = v0;
                let a814 = 4u64;
                nat_eq(a813, a814)
            };
            let v7: bool = {
                let a816 = 0u64;
                let a817 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h815, _, _, _, _, _, _, _, _, _, _, _) => {
                        h815
                    }
                };
                nat_lt(a816, a817)
            };
            let v8: u64 = f10(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                    h818
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h819, _, _, _, _, _, _, _, _) => {
                    h819
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h820, _, _, _, _, _, _, _) => {
                    h820
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h821, _, _, _, _, _, _) => {
                    h821
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h822, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h822
                }
            }, match v1.clone() {
                Adt1::C0(_, h823, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h823
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h824, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h824
                }
            });
            if if if if if if if if if if if if v6 {
                let a826 = match v2.clone() {
                    Adt2::C0(h825, _, _, _, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = 0u64;
                if nat_eq(a826, a827) {
                    let a829 = match v5.clone() {
                        Adt2::C0(h828, _, _, _, _, _, _, _, _, _, _) => {
                            h828
                        }
                    };
                    let a830 = 0u64;
                    nat_eq(a829, a830)
                } else {
                    false
                }
            } else {
                let a832 = match v2.clone() {
                    Adt2::C0(h831, _, _, _, _, _, _, _, _, _, _) => {
                        h831
                    }
                };
                let a833 = 0u64;
                if if nat_eq(a832, a833) {
                    true
                } else {
                    let a835 = match v2.clone() {
                        Adt2::C0(h834, _, _, _, _, _, _, _, _, _, _) => {
                            h834
                        }
                    };
                    let a836 = 26u64;
                    nat_eq(a835, a836)
                } {
                    let a838 = match v5.clone() {
                        Adt2::C0(h837, _, _, _, _, _, _, _, _, _, _) => {
                            h837
                        }
                    };
                    let a839 = 0u64;
                    if nat_eq(a838, a839) {
                        true
                    } else {
                        let a841 = match v5.clone() {
                            Adt2::C0(h840, _, _, _, _, _, _, _, _, _, _) => {
                                h840
                            }
                        };
                        let a842 = 26u64;
                        nat_eq(a841, a842)
                    }
                } else {
                    false
                }
            } {
                let a845 = match v2.clone() {
                    Adt2::C0(h843, _, _, _, _, _, _, _, _, _, _) => {
                        h843
                    }
                };
                let a846 = match v4.clone() {
                    Adt2::C0(h844, _, _, _, _, _, _, _, _, _, _) => {
                        h844
                    }
                };
                nat_eq(a845, a846)
            } else {
                false
            } {
                f15(v2.clone(), v4.clone())
            } else {
                false
            } {
                f71(v2.clone())?
            } else {
                false
            } {
                f71(v4.clone())?
            } else {
                false
            } {
                let a848 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h847, _, _, _, _, _) => {
                        h847
                    }
                };
                let a849 = 4u64;
                nat_eq(a848, a849)
            } else {
                false
            } {
                let a850 = v8;
                let a851 = f10(v2.clone(), 1u64)?;
                nat_le(a850, a851)
            } else {
                false
            } {
                let a853 = match v1.clone() {
                    Adt1::C0(_, _, _, h852, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h852
                    }
                };
                let a854 = 1073741824u64;
                nat_lt(a853, a854)
            } else {
                false
            } {
                let a856 = match v1.clone() {
                    Adt1::C0(h855, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h855
                    }
                };
                let a857 = 4294967295u64;
                if if if if nat_le(a856, a857) {
                    let a859 = match v1.clone() {
                        Adt1::C0(_, h858, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h858
                        }
                    };
                    let a860 = 4294967295u64;
                    nat_le(a859, a860)
                } else {
                    false
                } {
                    let a862 = match v1.clone() {
                        Adt1::C0(_, _, h861, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h861
                        }
                    };
                    let a863 = 4294967295u64;
                    nat_le(a862, a863)
                } else {
                    false
                } {
                    let a865 = match v1.clone() {
                        Adt1::C0(_, _, _, h864, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h864
                        }
                    };
                    let a866 = 4294967295u64;
                    nat_le(a865, a866)
                } else {
                    false
                } {
                    let a868 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h867, _, _, _, _, _, _, _, _, _, _, _) => {
                            h867
                        }
                    };
                    let a869 = 4294967295u64;
                    nat_le(a868, a869)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a872 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h870) => {
                            h870
                        }
                    };
                    let a873 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h871) => {
                            h871
                        }
                    };
                    nat_eq(a872, a873)
                } else {
                    true
                }
            } else {
                false
            } {
                f72(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h874, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h874
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h875, _) => {
                        h875
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
                    let a876 = v10;
                    let a877 = v8;
                    nat_add(a876, a877)?
                }, 4u64, f14(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h878, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h878
                    }
                }, v6)))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a880 = match v0.clone() {
        Adt2::C0(h879, _, _, _, _, _, _, _, _, _, _) => {
            h879
        }
    };
    let a881 = 0u64;
    if if if nat_eq(a880, a881) {
        let a883 = match v1.clone() {
            Adt2::C0(h882, _, _, _, _, _, _, _, _, _, _) => {
                h882
            }
        };
        let a884 = 0u64;
        nat_eq(a883, a884)
    } else {
        false
    } {
        f67(match v0.clone() {
            Adt2::C0(_, h885, _, _, _, _, _, _, _, _, _) => {
                h885
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h886, _, _, _, _, _, _, _, _) => {
                h886
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h887, _, _, _, _, _, _, _) => {
                h887
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h888, _, _, _, _, _, _) => {
                h888
            }
        }, match v1.clone() {
            Adt2::C0(_, h889, _, _, _, _, _, _, _, _, _) => {
                h889
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h890, _, _, _, _, _, _, _, _) => {
                h890
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h891, _, _, _, _, _, _, _) => {
                h891
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h892, _, _, _, _, _, _) => {
                h892
            }
        })
    } else {
        false
    } {
        let v2: u64 = f10(v1.clone(), 1u64)?;
        Ok(Adt6::C4({
            let a893 = v2;
            let a894 = f10(v0.clone(), 1u64)?;
            nat_add(a893, a894)?
        }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f28(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a896 = match v1.clone() {
        Adt2::C0(h895, _, _, _, _, _, _, _, _, _, _) => {
            h895
        }
    };
    let a897 = 0u64;
    if if if nat_eq(a896, a897) {
        true
    } else {
        let a899 = match v1.clone() {
            Adt2::C0(h898, _, _, _, _, _, _, _, _, _, _) => {
                h898
            }
        };
        let a900 = 1u64;
        nat_eq(a899, a900)
    } {
        true
    } else {
        let a902 = match v1.clone() {
            Adt2::C0(h901, _, _, _, _, _, _, _, _, _, _) => {
                h901
            }
        };
        let a903 = 30u64;
        nat_eq(a902, a903)
    } {
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
        if if if if if nat_eq(a906, a907) {
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
            nat_eq(a910, a911)
        } else {
            false
        } {
            let a914 = match v1.clone() {
                Adt2::C0(_, _, _, h912, _, _, _, _, _, _, _) => {
                    h912
                }
            };
            let a915 = match v2.clone() {
                Adt2::C0(_, _, _, h913, _, _, _, _, _, _, _) => {
                    h913
                }
            };
            nat_eq(a914, a915)
        } else {
            false
        } {
            let a918 = match v1.clone() {
                Adt2::C0(_, _, _, _, h916, _, _, _, _, _, _) => {
                    h916
                }
            };
            let a919 = match v2.clone() {
                Adt2::C0(_, _, _, _, h917, _, _, _, _, _, _) => {
                    h917
                }
            };
            nat_eq(a918, a919)
        } else {
            false
        } {
            let a920 = v0;
            let a921 = 4u64;
            if nat_eq(a920, a921) {
                let a923 = match v1.clone() {
                    Adt2::C0(_, _, h922, _, _, _, _, _, _, _, _) => {
                        h922
                    }
                };
                let a924 = 1u64;
                if nat_eq(a923, a924) {
                    let a927 = match v1.clone() {
                        Adt2::C0(_, h925, _, _, _, _, _, _, _, _, _) => {
                            h925
                        }
                    };
                    let a928 = match v2.clone() {
                        Adt2::C0(_, _, h926, _, _, _, _, _, _, _, _) => {
                            h926
                        }
                    };
                    nat_eq(a927, a928)
                } else {
                    false
                }
            } else {
                let a929 = v0;
                let a930 = 4u64;
                if if nat_lt(a929, a930) {
                    let a933 = match v1.clone() {
                        Adt2::C0(_, h931, _, _, _, _, _, _, _, _, _) => {
                            h931
                        }
                    };
                    let a934 = match v1.clone() {
                        Adt2::C0(_, _, h932, _, _, _, _, _, _, _, _) => {
                            h932
                        }
                    };
                    nat_eq(a933, a934)
                } else {
                    false
                } {
                    let a937 = match v1.clone() {
                        Adt2::C0(_, _, h935, _, _, _, _, _, _, _, _) => {
                            h935
                        }
                    };
                    let a938 = match v2.clone() {
                        Adt2::C0(_, _, h936, _, _, _, _, _, _, _, _) => {
                            h936
                        }
                    };
                    nat_eq(a937, a938)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m940 = f66(match v1.clone() {
                Adt2::C0(h939, _, _, _, _, _, _, _, _, _, _) => {
                    h939
                }
            });
            match m940 {
                Adt7::C0 => {
                    Ok(Adt6::C0)
                }
                Adt7::C1(v3) => {
                    Ok(Adt6::C4(f10(v2.clone(), 1u64)?, match v3 {
                        Adt8::C0(_, _, h941) => {
                            h941
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

pub fn f29(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if if if f73(match v1.clone() {
        Adt2::C0(h942, _, _, _, _, _, _, _, _, _, _) => {
            h942
        }
    }, v0) {
        let a945 = match v1.clone() {
            Adt2::C0(h943, _, _, _, _, _, _, _, _, _, _) => {
                h943
            }
        };
        let a946 = match v2.clone() {
            Adt2::C0(h944, _, _, _, _, _, _, _, _, _, _) => {
                h944
            }
        };
        nat_eq(a945, a946)
    } else {
        false
    } {
        let a949 = match v1.clone() {
            Adt2::C0(_, h947, _, _, _, _, _, _, _, _, _) => {
                h947
            }
        };
        let a950 = match v2.clone() {
            Adt2::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
                h948
            }
        };
        if if if nat_eq(a949, a950) {
            let a953 = match v1.clone() {
                Adt2::C0(_, _, h951, _, _, _, _, _, _, _, _) => {
                    h951
                }
            };
            let a954 = match v2.clone() {
                Adt2::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                    h952
                }
            };
            nat_eq(a953, a954)
        } else {
            false
        } {
            let a957 = match v1.clone() {
                Adt2::C0(_, _, _, h955, _, _, _, _, _, _, _) => {
                    h955
                }
            };
            let a958 = match v2.clone() {
                Adt2::C0(_, _, _, h956, _, _, _, _, _, _, _) => {
                    h956
                }
            };
            nat_eq(a957, a958)
        } else {
            false
        } {
            let a961 = match v1.clone() {
                Adt2::C0(_, _, _, _, h959, _, _, _, _, _, _) => {
                    h959
                }
            };
            let a962 = match v2.clone() {
                Adt2::C0(_, _, _, _, h960, _, _, _, _, _, _) => {
                    h960
                }
            };
            nat_eq(a961, a962)
        } else {
            false
        }
    } else {
        false
    } {
        let m964 = f66(match v1.clone() {
            Adt2::C0(h963, _, _, _, _, _, _, _, _, _, _) => {
                h963
            }
        });
        match m964 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                Ok(Adt6::C4(f10(v2.clone(), 1u64)?, match v3 {
                    Adt8::C0(_, _, h965) => {
                        h965
                    }
                }, f14(v1.clone(), 1u64), f14(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h966, _, _, _, _, _, _, _, _, _, _) => {
                        h966
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f30(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h967, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h967
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h968, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h968
            }
        }
    };
    let a969 = 0u64;
    let a970 = v4;
    if if nat_lt(a969, a970) {
        let a971 = v4;
        let a972 = 2147483648u64;
        nat_lt(a971, a972)
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
        let v7: u64 = f74(match v5.clone() {
            Adt2::C0(_, _, h973, _, _, _, _, _, _, _, _) => {
                h973
            }
        }, v4)?;
        let v8: u64 = f74(match v5.clone() {
            Adt2::C0(_, _, _, h974, _, _, _, _, _, _, _) => {
                h974
            }
        }, v4)?;
        let a976 = match v5.clone() {
            Adt2::C0(h975, _, _, _, _, _, _, _, _, _, _) => {
                h975
            }
        };
        let a977 = 0u64;
        if if if if if nat_eq(a976, a977) {
            let a979 = match v6.clone() {
                Adt2::C0(h978, _, _, _, _, _, _, _, _, _, _) => {
                    h978
                }
            };
            let a980 = 0u64;
            nat_eq(a979, a980)
        } else {
            false
        } {
            let a982 = match v5.clone() {
                Adt2::C0(_, _, _, _, h981, _, _, _, _, _, _) => {
                    h981
                }
            };
            let a983 = 1u64;
            nat_eq(a982, a983)
        } else {
            false
        } {
            let a986 = match v5.clone() {
                Adt2::C0(_, h984, _, _, _, _, _, _, _, _, _) => {
                    h984
                }
            };
            let a987 = match v6.clone() {
                Adt2::C0(_, h985, _, _, _, _, _, _, _, _, _) => {
                    h985
                }
            };
            nat_eq(a986, a987)
        } else {
            false
        } {
            if v0 {
                let a989 = match v5.clone() {
                    Adt2::C0(_, _, h988, _, _, _, _, _, _, _, _) => {
                        h988
                    }
                };
                let a990 = 2147483648u64;
                if if if nat_lt(a989, a990) {
                    let a992 = match v5.clone() {
                        Adt2::C0(_, _, _, h991, _, _, _, _, _, _, _) => {
                            h991
                        }
                    };
                    let a993 = 2147483648u64;
                    nat_lt(a992, a993)
                } else {
                    false
                } {
                    let a996 = {
                        let a994 = v7;
                        let a995 = v8;
                        nat_mul(a994, a995)?
                    };
                    let a997 = 2147483648u64;
                    nat_le(a996, a997)
                } else {
                    false
                } {
                    f75(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a999 = match v1.clone() {
                    Adt1::C0(h998, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h998
                    }
                };
                let a1000 = v7;
                if if if if nat_eq(a999, a1000) {
                    let a1002 = match v1 {
                        Adt1::C0(_, h1001, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1001
                        }
                    };
                    let a1003 = v8;
                    nat_eq(a1002, a1003)
                } else {
                    false
                } {
                    f76(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1005 = match v6.clone() {
                        Adt2::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                            h1004
                        }
                    };
                    let a1006 = v4;
                    if nat_eq(a1005, a1006) {
                        let a1008 = match v6.clone() {
                            Adt2::C0(_, _, _, h1007, _, _, _, _, _, _, _) => {
                                h1007
                            }
                        };
                        let a1009 = v4;
                        nat_eq(a1008, a1009)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1013 = match v6 {
                        Adt2::C0(_, _, _, _, h1010, _, _, _, _, _, _) => {
                            h1010
                        }
                    };
                    let a1014 = {
                        let a1011 = v7;
                        let a1012 = v8;
                        nat_mul(a1011, a1012)?
                    };
                    nat_eq(a1013, a1014)
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

pub fn f31(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt6> {
    let a1016 = match v2.clone() {
        Adt2::C0(h1015, _, _, _, _, _, _, _, _, _, _) => {
            h1015
        }
    };
    let a1017 = 0u64;
    if if if if if nat_eq(a1016, a1017) {
        true
    } else {
        let a1019 = match v2.clone() {
            Adt2::C0(h1018, _, _, _, _, _, _, _, _, _, _) => {
                h1018
            }
        };
        let a1020 = 1u64;
        nat_eq(a1019, a1020)
    } {
        true
    } else {
        let a1022 = match v2.clone() {
            Adt2::C0(h1021, _, _, _, _, _, _, _, _, _, _) => {
                h1021
            }
        };
        let a1023 = 30u64;
        nat_eq(a1022, a1023)
    } {
        let a1026 = match v2.clone() {
            Adt2::C0(h1024, _, _, _, _, _, _, _, _, _, _) => {
                h1024
            }
        };
        let a1027 = match v3.clone() {
            Adt2::C0(h1025, _, _, _, _, _, _, _, _, _, _) => {
                h1025
            }
        };
        nat_eq(a1026, a1027)
    } else {
        false
    } {
        if if f15(v2.clone(), v3.clone()) {
            let a1029 = match v1.clone() {
                Adt1::C0(h1028, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1028
                }
            };
            let a1030 = 4294967296u64;
            nat_lt(a1029, a1030)
        } else {
            false
        } {
            let a1032 = match v1.clone() {
                Adt1::C0(_, h1031, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1031
                }
            };
            let a1033 = 4294967296u64;
            nat_lt(a1032, a1033)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v3.clone(), 1u64)?, {
            let a1035 = match v2.clone() {
                Adt2::C0(h1034, _, _, _, _, _, _, _, _, _, _) => {
                    h1034
                }
            };
            let a1036 = 0u64;
            if nat_eq(a1035, a1036) {
                4u64
            } else {
                2u64
            }
        }, f14(v2.clone(), 1u64), f14(v3, 1u64), Adt4::C13({
            let a1037 = v0;
            let a1038 = 31u64;
            if nat_eq(a1037, a1038) {
                0u64
            } else {
                let a1039 = v0;
                let a1040 = 47u64;
                if nat_eq(a1039, a1040) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1041, _, _, _, _, _, _, _, _, _, _) => {
                h1041
            }
        }, match v1.clone() {
            Adt1::C0(h1042, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1042
            }
        }, match v1 {
            Adt1::C0(_, h1043, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1043
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1045 = match v0.clone() {
        Adt2::C0(h1044, _, _, _, _, _, _, _, _, _, _) => {
            h1044
        }
    };
    let a1046 = 0u64;
    if if if nat_eq(a1045, a1046) {
        let a1048 = match v1.clone() {
            Adt2::C0(h1047, _, _, _, _, _, _, _, _, _, _) => {
                h1047
            }
        };
        let a1049 = 0u64;
        nat_eq(a1048, a1049)
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

pub fn f33(v0: Adt2, v1: Adt2) -> R<Adt6> {
    let a1051 = match v0.clone() {
        Adt2::C0(h1050, _, _, _, _, _, _, _, _, _, _) => {
            h1050
        }
    };
    let a1052 = 0u64;
    if if nat_eq(a1051, a1052) {
        let a1054 = match v1.clone() {
            Adt2::C0(h1053, _, _, _, _, _, _, _, _, _, _) => {
                h1053
            }
        };
        let a1055 = 26u64;
        nat_eq(a1054, a1055)
    } else {
        false
    } {
        let a1057 = match v0.clone() {
            Adt2::C0(_, _, _, h1056, _, _, _, _, _, _, _) => {
                h1056
            }
        };
        let a1058 = 1u64;
        if if if if if if if nat_eq(a1057, a1058) {
            let a1060 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1059, _, _, _, _, _, _) => {
                    h1059
                }
            };
            let a1061 = 1u64;
            nat_eq(a1060, a1061)
        } else {
            false
        } {
            let a1063 = match v0.clone() {
                Adt2::C0(_, h1062, _, _, _, _, _, _, _, _, _) => {
                    h1062
                }
            };
            let a1064 = 2147483647u64;
            nat_le(a1063, a1064)
        } else {
            false
        } {
            let a1067 = match v1.clone() {
                Adt2::C0(_, h1065, _, _, _, _, _, _, _, _, _) => {
                    h1065
                }
            };
            let a1068 = match v0.clone() {
                Adt2::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
                    h1066
                }
            };
            nat_eq(a1067, a1068)
        } else {
            false
        } {
            let a1070 = match v1.clone() {
                Adt2::C0(_, _, h1069, _, _, _, _, _, _, _, _) => {
                    h1069
                }
            };
            let a1071 = 1u64;
            nat_eq(a1070, a1071)
        } else {
            false
        } {
            let a1073 = match v1.clone() {
                Adt2::C0(_, _, _, h1072, _, _, _, _, _, _, _) => {
                    h1072
                }
            };
            let a1074 = 1u64;
            nat_eq(a1073, a1074)
        } else {
            false
        } {
            let a1076 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1075, _, _, _, _, _, _) => {
                    h1075
                }
            };
            let a1077 = 1u64;
            nat_eq(a1076, a1077)
        } else {
            false
        } {
            Ok(Adt6::C4({
                let a1080 = match v0.clone() {
                    Adt2::C0(_, _, h1078, _, _, _, _, _, _, _, _) => {
                        h1078
                    }
                };
                let a1081 = f53(match v0.clone() {
                    Adt2::C0(_, h1079, _, _, _, _, _, _, _, _, _) => {
                        h1079
                    }
                })?;
                nat_mul(a1080, a1081)?
            }, 4u64, f14(v0, 1u64), f14(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt6::C1)
        }
    } else {
        Ok(Adt6::C0)
    }
}

pub fn f34(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt6> {
    let m1082 = v1;
    match m1082 {
        None => {
            Ok(Adt6::C1)
        }
        Some(v3) => {
            let v4: u64 = f55(f14(v0.clone(), 1u64))?;
            let a1084 = match v0.clone() {
                Adt2::C0(h1083, _, _, _, _, _, _, _, _, _, _) => {
                    h1083
                }
            };
            let a1085 = 26u64;
            if if if nat_eq(a1084, a1085) {
                let a1087 = match v3.clone() {
                    Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                        h1086
                    }
                };
                let a1088 = 26u64;
                nat_eq(a1087, a1088)
            } else {
                false
            } {
                let a1090 = match v2.clone() {
                    Adt2::C0(h1089, _, _, _, _, _, _, _, _, _, _) => {
                        h1089
                    }
                };
                let a1091 = 27u64;
                nat_eq(a1090, a1091)
            } else {
                false
            } {
                if if if if if if f15(v0.clone(), v3.clone()) {
                    let a1093 = match v2.clone() {
                        Adt2::C0(_, h1092, _, _, _, _, _, _, _, _, _) => {
                            h1092
                        }
                    };
                    let a1094 = 1u64;
                    nat_eq(a1093, a1094)
                } else {
                    false
                } {
                    let a1096 = match v2.clone() {
                        Adt2::C0(_, _, h1095, _, _, _, _, _, _, _, _) => {
                            h1095
                        }
                    };
                    let a1097 = 1u64;
                    nat_eq(a1096, a1097)
                } else {
                    false
                } {
                    let a1099 = match v2.clone() {
                        Adt2::C0(_, _, _, h1098, _, _, _, _, _, _, _) => {
                            h1098
                        }
                    };
                    let a1100 = 1u64;
                    nat_eq(a1099, a1100)
                } else {
                    false
                } {
                    let a1102 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1101, _, _, _, _, _, _) => {
                            h1101
                        }
                    };
                    let a1103 = 1u64;
                    nat_eq(a1102, a1103)
                } else {
                    false
                } {
                    let a1104 = v4;
                    let a1105 = 9223372036854775808u64;
                    nat_lt(a1104, a1105)
                } else {
                    false
                } {
                    Ok(Adt6::C4({
                        let a1108 = 2u64;
                        let a1109 = {
                            let a1106 = v4;
                            let a1107 = 2u64;
                            nat_mul(a1106, a1107)?
                        };
                        nat_add(a1108, a1109)?
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

pub fn f35(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1110, _, _, _, _, _, _, _, _, _, _) => {
            h1110
        }
    };
    let a1111 = v3;
    let a1112 = 0u64;
    if if nat_eq(a1111, a1112) {
        true
    } else {
        let a1113 = v0;
        let a1114 = 13u64;
        if nat_eq(a1113, a1114) {
            let a1115 = v3;
            let a1116 = 1u64;
            if nat_eq(a1115, a1116) {
                true
            } else {
                let a1117 = v3;
                let a1118 = 30u64;
                nat_eq(a1117, a1118)
            }
        } else {
            false
        }
    } {
        let a1120 = v3;
        let a1121 = match v2.clone() {
            Adt2::C0(h1119, _, _, _, _, _, _, _, _, _, _) => {
                h1119
            }
        };
        if if nat_eq(a1120, a1121) {
            let a1122 = v0;
            let a1123 = 13u64;
            if nat_eq(a1122, a1123) {
                let a1125 = match v2.clone() {
                    Adt2::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
                        h1124
                    }
                };
                let a1126 = 1u64;
                if if if nat_eq(a1125, a1126) {
                    let a1128 = match v2.clone() {
                        Adt2::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                            h1127
                        }
                    };
                    let a1129 = 1u64;
                    nat_eq(a1128, a1129)
                } else {
                    false
                } {
                    let a1131 = match v2.clone() {
                        Adt2::C0(_, _, _, h1130, _, _, _, _, _, _, _) => {
                            h1130
                        }
                    };
                    let a1132 = 1u64;
                    nat_eq(a1131, a1132)
                } else {
                    false
                } {
                    let a1134 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1133, _, _, _, _, _, _) => {
                            h1133
                        }
                    };
                    let a1135 = 1u64;
                    nat_eq(a1134, a1135)
                } else {
                    false
                }
            } else {
                let a1137 = match v2.clone() {
                    Adt2::C0(_, h1136, _, _, _, _, _, _, _, _, _) => {
                        h1136
                    }
                };
                let a1138 = 1u64;
                if if if nat_eq(a1137, a1138) {
                    let a1141 = match v1.clone() {
                        Adt2::C0(_, _, h1139, _, _, _, _, _, _, _, _) => {
                            h1139
                        }
                    };
                    let a1142 = match v2.clone() {
                        Adt2::C0(_, _, h1140, _, _, _, _, _, _, _, _) => {
                            h1140
                        }
                    };
                    nat_eq(a1141, a1142)
                } else {
                    false
                } {
                    let a1145 = match v1.clone() {
                        Adt2::C0(_, _, _, h1143, _, _, _, _, _, _, _) => {
                            h1143
                        }
                    };
                    let a1146 = match v2.clone() {
                        Adt2::C0(_, _, _, h1144, _, _, _, _, _, _, _) => {
                            h1144
                        }
                    };
                    nat_eq(a1145, a1146)
                } else {
                    false
                } {
                    let a1149 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1147, _, _, _, _, _, _) => {
                            h1147
                        }
                    };
                    let a1150 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1148, _, _, _, _, _, _) => {
                            h1148
                        }
                    };
                    nat_eq(a1149, a1150)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f14(v1, 1u64);
            Ok(Adt6::C4(f77(v4.clone(), v0, v3)?, {
                let a1151 = v3;
                let a1152 = 0u64;
                if nat_eq(a1151, a1152) {
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

pub fn f36(v0: u64, v1: u64) -> Option<u64> {
    let a1153 = v0;
    let a1154 = 8u64;
    if nat_eq(a1153, a1154) {
        Some(5u64)
    } else {
        let a1155 = v0;
        let a1156 = 9u64;
        if nat_eq(a1155, a1156) {
            Some(6u64)
        } else {
            let a1157 = v0;
            let a1158 = 89u64;
            if nat_eq(a1157, a1158) {
                let a1159 = v1;
                let a1160 = 0u64;
                if nat_eq(a1159, a1160) {
                    Some(0u64)
                } else {
                    let a1161 = v1;
                    let a1162 = 1u64;
                    if nat_eq(a1161, a1162) {
                        Some(1u64)
                    } else {
                        let a1163 = v1;
                        let a1164 = 2u64;
                        if nat_eq(a1163, a1164) {
                            Some(2u64)
                        } else {
                            let a1165 = v1;
                            let a1166 = 3u64;
                            if nat_eq(a1165, a1166) {
                                Some(3u64)
                            } else {
                                let a1167 = v1;
                                let a1168 = 6u64;
                                if nat_eq(a1167, a1168) {
                                    Some(4u64)
                                } else {
                                    let a1169 = v1;
                                    let a1170 = 12u64;
                                    if nat_eq(a1169, a1170) {
                                        Some(7u64)
                                    } else {
                                        let a1171 = v1;
                                        let a1172 = 11u64;
                                        if nat_eq(a1171, a1172) {
                                            Some(8u64)
                                        } else {
                                            let a1173 = v1;
                                            let a1174 = 18u64;
                                            if nat_eq(a1173, a1174) {
                                                Some(9u64)
                                            } else {
                                                let a1175 = v1;
                                                let a1176 = 19u64;
                                                if nat_eq(a1175, a1176) {
                                                    Some(10u64)
                                                } else {
                                                    let a1177 = v1;
                                                    let a1178 = 20u64;
                                                    if nat_eq(a1177, a1178) {
                                                        Some(11u64)
                                                    } else {
                                                        let a1179 = v1;
                                                        let a1180 = 21u64;
                                                        if nat_eq(a1179, a1180) {
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

pub fn f37(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1181 = f78(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1181 {
        Adt6::C0 => {
            Ok(Adt6::C0)
        }
        Adt6::C1 => {
            Ok(Adt6::C1)
        }
        Adt6::C2(v5, v6, _, _, _, _, v11) => {
            Ok(Adt6::C4(v11, 4u64, f14(v2.clone(), 1u64), f14(v4, 1u64), Adt4::C10(v5, v6, {
                let m1182 = v3;
                match m1182 {
                    None => {
                        f14(v2, 1u64)
                    }
                    Some(v12) => {
                        f14(v12, 1u64)
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

pub fn f38(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt6> {
    let a1184 = match v1.clone() {
        Adt2::C0(h1183, _, _, _, _, _, _, _, _, _, _) => {
            h1183
        }
    };
    let a1185 = 0u64;
    if if if if if nat_eq(a1184, a1185) {
        true
    } else {
        let a1187 = match v1.clone() {
            Adt2::C0(h1186, _, _, _, _, _, _, _, _, _, _) => {
                h1186
            }
        };
        let a1188 = 1u64;
        nat_eq(a1187, a1188)
    } {
        true
    } else {
        let a1190 = match v1.clone() {
            Adt2::C0(h1189, _, _, _, _, _, _, _, _, _, _) => {
                h1189
            }
        };
        let a1191 = 30u64;
        nat_eq(a1190, a1191)
    } {
        let a1194 = match v1.clone() {
            Adt2::C0(h1192, _, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        let a1195 = match v2.clone() {
            Adt2::C0(h1193, _, _, _, _, _, _, _, _, _, _) => {
                h1193
            }
        };
        if nat_eq(a1194, a1195) {
            true
        } else {
            let a1197 = match v2.clone() {
                Adt2::C0(h1196, _, _, _, _, _, _, _, _, _, _) => {
                    h1196
                }
            };
            let a1198 = 0u64;
            nat_eq(a1197, a1198)
        }
    } else {
        false
    } {
        f15(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt6::C4(f10(v2.clone(), 1u64)?, {
            let a1200 = match v2.clone() {
                Adt2::C0(h1199, _, _, _, _, _, _, _, _, _, _) => {
                    h1199
                }
            };
            let a1201 = 0u64;
            if nat_eq(a1200, a1201) {
                4u64
            } else {
                2u64
            }
        }, f14(v1.clone(), 1u64), f14(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        }, match v2 {
            Adt2::C0(h1203, _, _, _, _, _, _, _, _, _, _) => {
                h1203
            }
        })))
    } else {
        Ok(Adt6::C1)
    }
}

pub fn f39(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a1205 = v0;
        let a1206 = match v1.clone() {
            Adt3::C0(h1204, _, _, _, _, _, _, _) => {
                h1204
            }
        };
        let a1207 = 0u64;
        nat_quot(a1205, a1206, a1207)
    };
    let v3: u64 = {
        let a1209 = v2;
        let a1210 = match v1.clone() {
            Adt3::C0(_, h1208, _, _, _, _, _, _) => {
                h1208
            }
        };
        let a1211 = 0u64;
        nat_quot(a1209, a1210, a1211)
    };
    f79({
        let a1213 = v0;
        let a1214 = match v1.clone() {
            Adt3::C0(h1212, _, _, _, _, _, _, _) => {
                h1212
            }
        };
        let a1215 = 0u64;
        nat_rem(a1213, a1214, a1215)
    }, {
        let a1217 = v2;
        let a1218 = match v1.clone() {
            Adt3::C0(_, h1216, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1219 = 0u64;
        nat_rem(a1217, a1218, a1219)
    }, {
        let a1221 = v3;
        let a1222 = match v1.clone() {
            Adt3::C0(_, _, h1220, _, _, _, _, _) => {
                h1220
            }
        };
        let a1223 = 0u64;
        nat_rem(a1221, a1222, a1223)
    }, {
        let a1225 = v3;
        let a1226 = match v1.clone() {
            Adt3::C0(_, _, h1224, _, _, _, _, _) => {
                h1224
            }
        };
        let a1227 = 0u64;
        nat_quot(a1225, a1226, a1227)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1228, _, _, _) => {
            h1228
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1229, _, _) => {
            h1229
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1230, _) => {
            h1230
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1231) => {
            h1231
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1249 = {
        let a1244 = f80(v2, {
            let a1235 = {
                let a1232 = v0;
                let a1233 = v1;
                let a1234 = 0u64;
                nat_quot(a1232, a1233, a1234)
            };
            let a1236 = v2;
            let a1237 = 0u64;
            nat_rem(a1235, a1236, a1237)
        }, {
            let a1241 = {
                let a1238 = v0;
                let a1239 = v1;
                let a1240 = 0u64;
                nat_quot(a1238, a1239, a1240)
            };
            let a1242 = v2;
            let a1243 = 0u64;
            nat_quot(a1241, a1242, a1243)
        })?;
        let a1245 = v1;
        nat_mul(a1244, a1245)?
    };
    let a1250 = {
        let a1246 = v0;
        let a1247 = v1;
        let a1248 = 0u64;
        nat_rem(a1246, a1247, a1248)
    };
    nat_add(a1249, a1250)
}

pub fn f41(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a1252 = v0;
        let a1253 = match v2.clone() {
            Adt3::C0(h1251, _, _, _, _, _, _, _) => {
                h1251
            }
        };
        let a1254 = 0u64;
        nat_quot(a1252, a1253, a1254)
    };
    let v4: u64 = {
        let a1256 = v3;
        let a1257 = match v2.clone() {
            Adt3::C0(_, h1255, _, _, _, _, _, _) => {
                h1255
            }
        };
        let a1258 = 0u64;
        nat_quot(a1256, a1257, a1258)
    };
    f81({
        let a1260 = v0;
        let a1261 = match v2.clone() {
            Adt3::C0(h1259, _, _, _, _, _, _, _) => {
                h1259
            }
        };
        let a1262 = 0u64;
        nat_rem(a1260, a1261, a1262)
    }, {
        let a1264 = v3;
        let a1265 = match v2.clone() {
            Adt3::C0(_, h1263, _, _, _, _, _, _) => {
                h1263
            }
        };
        let a1266 = 0u64;
        nat_rem(a1264, a1265, a1266)
    }, {
        let a1268 = v4;
        let a1269 = match v2.clone() {
            Adt3::C0(_, _, h1267, _, _, _, _, _) => {
                h1267
            }
        };
        let a1270 = 0u64;
        nat_rem(a1268, a1269, a1270)
    }, {
        let a1272 = v4;
        let a1273 = match v2 {
            Adt3::C0(_, _, h1271, _, _, _, _, _) => {
                h1271
            }
        };
        let a1274 = 0u64;
        nat_quot(a1272, a1273, a1274)
    }, match v1.clone() {
        Adt3::C0(h1275, _, _, _, _, _, _, _) => {
            h1275
        }
    }, match v1.clone() {
        Adt3::C0(_, h1276, _, _, _, _, _, _) => {
            h1276
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1277, _, _, _, _, _) => {
            h1277
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h1278, _, _, _, _) => {
            h1278
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h1279, _, _, _) => {
            h1279
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h1280, _, _) => {
            h1280
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h1281, _) => {
            h1281
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h1282) => {
            h1282
        }
    })
}

pub fn f42(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt5> {
    let v5: u64 = {
        let a1284 = v0;
        let a1285 = match v3.clone() {
            Adt3::C0(h1283, _, _, _, _, _, _, _) => {
                h1283
            }
        };
        let a1286 = 0u64;
        nat_quot(a1284, a1285, a1286)
    };
    let v6: u64 = {
        let a1288 = v5;
        let a1289 = match v3.clone() {
            Adt3::C0(_, h1287, _, _, _, _, _, _) => {
                h1287
            }
        };
        let a1290 = 0u64;
        nat_quot(a1288, a1289, a1290)
    };
    let v7: u64 = {
        let a1292 = v0;
        let a1293 = match v3.clone() {
            Adt3::C0(h1291, _, _, _, _, _, _, _) => {
                h1291
            }
        };
        let a1294 = 0u64;
        nat_rem(a1292, a1293, a1294)
    };
    let v8: u64 = {
        let a1296 = v5;
        let a1297 = match v3.clone() {
            Adt3::C0(_, h1295, _, _, _, _, _, _) => {
                h1295
            }
        };
        let a1298 = 0u64;
        nat_rem(a1296, a1297, a1298)
    };
    let v9: u64 = {
        let a1300 = v6;
        let a1301 = match v3.clone() {
            Adt3::C0(_, _, h1299, _, _, _, _, _) => {
                h1299
            }
        };
        let a1302 = 0u64;
        nat_rem(a1300, a1301, a1302)
    };
    let v10: u64 = {
        let a1304 = v6;
        let a1305 = match v3.clone() {
            Adt3::C0(_, _, h1303, _, _, _, _, _) => {
                h1303
            }
        };
        let a1306 = 0u64;
        nat_quot(a1304, a1305, a1306)
    };
    let v11: u64 = {
        let a1307 = v4;
        let a1308 = 0u64;
        if nat_eq(a1307, a1308) {
            match v1.clone() {
                Adt3::C0(h1309, _, _, _, _, _, _, _) => {
                    h1309
                }
            }
        } else {
            let a1310 = v4;
            let a1311 = 1u64;
            if nat_eq(a1310, a1311) {
                match v1.clone() {
                    Adt3::C0(_, h1312, _, _, _, _, _, _) => {
                        h1312
                    }
                }
            } else {
                let a1313 = v4;
                let a1314 = 2u64;
                if nat_eq(a1313, a1314) {
                    match v1.clone() {
                        Adt3::C0(_, _, h1315, _, _, _, _, _) => {
                            h1315
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h1316, _, _, _, _) => {
                            h1316
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a1323 = v11;
        let a1324 = {
            let a1317 = v4;
            let a1318 = 0u64;
            if nat_eq(a1317, a1318) {
                v7
            } else {
                let a1319 = v4;
                let a1320 = 1u64;
                if nat_eq(a1319, a1320) {
                    v8
                } else {
                    let a1321 = v4;
                    let a1322 = 2u64;
                    if nat_eq(a1321, a1322) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a1323, a1324)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt5::C1(v12, f79(if if v12 {
        let a1325 = v4;
        let a1326 = 0u64;
        nat_eq(a1325, a1326)
    } else {
        false
    } {
        let a1327 = v7;
        let a1328 = v11;
        nat_sub(a1327, a1328)
    } else {
        v7
    }, if if v12 {
        let a1329 = v4;
        let a1330 = 1u64;
        nat_eq(a1329, a1330)
    } else {
        false
    } {
        let a1331 = v8;
        let a1332 = v11;
        nat_sub(a1331, a1332)
    } else {
        v8
    }, if if v12 {
        let a1333 = v4;
        let a1334 = 2u64;
        nat_eq(a1333, a1334)
    } else {
        false
    } {
        let a1335 = v9;
        let a1336 = v11;
        nat_sub(a1335, a1336)
    } else {
        v9
    }, if if v12 {
        let a1337 = v4;
        let a1338 = 3u64;
        nat_eq(a1337, a1338)
    } else {
        false
    } {
        let a1339 = v10;
        let a1340 = v11;
        nat_sub(a1339, a1340)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h1341, _, _, _) => {
            h1341
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h1342, _, _) => {
            h1342
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h1343, _) => {
            h1343
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h1344) => {
            h1344
        }
    })?, f39(v0, v3)?))
}

pub fn f43(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt5> {
    let v8: u64 = f82(v0, v2.clone(), 0u64);
    let v9: u64 = f82(v0, v2.clone(), 1u64);
    let v10: u64 = f82(v0, v2.clone(), 2u64);
    let v11: u64 = f82(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt5::C1(false, f83(v1.clone(), f84(v8, v3, match v1.clone() {
            Adt3::C0(h1345, _, _, _, _, _, _, _) => {
                h1345
            }
        })?, f84(v9, v4, match v1.clone() {
            Adt3::C0(_, h1346, _, _, _, _, _, _) => {
                h1346
            }
        })?, f84(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h1347, _, _, _, _, _) => {
                h1347
            }
        })?, f84(v11, v6, match v1 {
            Adt3::C0(_, _, _, h1348, _, _, _, _) => {
                h1348
            }
        })?)?, f39(v0, v2)?))
    } else {
        if if if if f85(v8, v3, match v1.clone() {
            Adt3::C0(h1349, _, _, _, _, _, _, _) => {
                h1349
            }
        })? {
            f85(v9, v4, match v1.clone() {
                Adt3::C0(_, h1350, _, _, _, _, _, _) => {
                    h1350
                }
            })?
        } else {
            false
        } {
            f85(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h1351, _, _, _, _, _) => {
                    h1351
                }
            })?
        } else {
            false
        } {
            f85(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h1352, _, _, _, _) => {
                    h1352
                }
            })?
        } else {
            false
        } {
            Ok(Adt5::C1(false, f83(v1, f86(v8, v3)?, f86(v9, v4)?, f86(v10, v5)?, f86(v11, v6)?)?, f39(v0, v2)?))
        } else {
            Ok(Adt5::C1(true, 0u64, f39(v0, v2)?))
        }
    }
}

pub fn f44(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let v4: u64 = f82(v0, v2.clone(), 0u64);
    let v5: u64 = f82(v0, v2.clone(), 1u64);
    let v6: u64 = f82(v0, v2.clone(), 2u64);
    let v7: u64 = f82(v0, v2.clone(), 3u64);
    Ok(Adt5::C1(false, f83(v1.clone(), f87(v4, v3, match v1 {
        Adt3::C0(h1353, _, _, _, _, _, _, _) => {
            h1353
        }
    })?, v5, v6, v7)?, f39(v0, v2)?))
}

pub fn f45(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt5> {
    let a1354 = v0;
    let a1355 = v3;
    if nat_lt(a1354, a1355) {
        Ok(Adt5::C1(false, {
            let a1356 = v0;
            let a1357 = 4u64;
            nat_mul(a1356, a1357)?
        }, {
            let a1358 = v0;
            let a1359 = 4u64;
            nat_mul(a1358, a1359)?
        }))
    } else {
        let v6: u64 = {
            let a1360 = v0;
            let a1361 = v3;
            nat_sub(a1360, a1361)
        };
        let v7: u64 = f39(v6, v1)?;
        let v8: u64 = f88(v6, v2, v4)?;
        if v5 {
            Ok(Adt5::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt5::C1(true, v7, v8))
        }
    }
}

pub fn f46(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1362 = v0;
    let a1363 = v3;
    if nat_lt(a1362, a1363) {
        Ok(Adt5::C1(true, 0u64, f39(v0, v2)?))
    } else {
        let v4: u64 = {
            let a1364 = v0;
            let a1365 = v3;
            nat_sub(a1364, a1365)
        };
        let v5: u64 = f41(v4, v2, v1.clone())?;
        Ok(Adt5::C4(8u64, 3u64, v5, 0u64, f39(v4, v1)?, 3u64, v5))
    }
}

pub fn f47(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt5> {
    let v4: u64 = {
        let a1367 = v1;
        let a1368 = match v3.clone() {
            Adt3::C0(h1366, _, _, _, _, _, _, _) => {
                h1366
            }
        };
        let a1369 = 0u64;
        nat_rem(a1367, a1368, a1369)
    };
    let v5: u64 = {
        let a1375 = {
            let a1371 = v1;
            let a1372 = match v3.clone() {
                Adt3::C0(h1370, _, _, _, _, _, _, _) => {
                    h1370
                }
            };
            let a1373 = 0u64;
            nat_quot(a1371, a1372, a1373)
        };
        let a1376 = match v3.clone() {
            Adt3::C0(_, h1374, _, _, _, _, _, _) => {
                h1374
            }
        };
        let a1377 = 0u64;
        nat_rem(a1375, a1376, a1377)
    };
    let v6: bool = f89(v0, v4, v5);
    Ok(Adt5::C1(!v6, if v6 {
        f41(v1, v2, v3.clone())?
    } else {
        0u64
    }, f39(v1, v3)?))
}

pub fn f48(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt5> {
    Ok(Adt5::C4({
        let a1378 = v3;
        let a1379 = 8u64;
        nat_add(a1378, a1379)?
    }, 0u64, f39(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f41(v0, v5, v2.clone())?
    }, 3u64, f39(v0, v2)?))
}

pub fn f49(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt5> {
    let v6: u64 = {
        let a1381 = v0;
        let a1382 = match v2.clone() {
            Adt3::C0(h1380, _, _, _, _, _, _, _) => {
                h1380
            }
        };
        let a1383 = 0u64;
        nat_rem(a1381, a1382, a1383)
    };
    let v7: u64 = {
        let a1385 = v0;
        let a1386 = match v2.clone() {
            Adt3::C0(h1384, _, _, _, _, _, _, _) => {
                h1384
            }
        };
        let a1387 = 0u64;
        nat_quot(a1385, a1386, a1387)
    };
    let v8: u64 = {
        let a1389 = v7;
        let a1390 = match v2.clone() {
            Adt3::C0(_, h1388, _, _, _, _, _, _) => {
                h1388
            }
        };
        let a1391 = 0u64;
        nat_rem(a1389, a1390, a1391)
    };
    let v9: u64 = {
        let a1393 = v7;
        let a1394 = match v2.clone() {
            Adt3::C0(_, h1392, _, _, _, _, _, _) => {
                h1392
            }
        };
        let a1395 = 0u64;
        nat_quot(a1393, a1394, a1395)
    };
    let v10: u64 = {
        let a1397 = v9;
        let a1398 = match v2.clone() {
            Adt3::C0(_, _, h1396, _, _, _, _, _) => {
                h1396
            }
        };
        let a1399 = 0u64;
        nat_rem(a1397, a1398, a1399)
    };
    let v11: u64 = {
        let a1401 = v9;
        let a1402 = match v2.clone() {
            Adt3::C0(_, _, h1400, _, _, _, _, _) => {
                h1400
            }
        };
        let a1403 = 0u64;
        nat_quot(a1401, a1402, a1403)
    };
    let v12: u64 = f39(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a1412 = {
                let a1407 = {
                    let a1404 = v10;
                    let a1405 = v4;
                    let a1406 = 0u64;
                    nat_quot(a1404, a1405, a1406)
                };
                let a1408 = v5;
                nat_mul(a1407, a1408)?
            };
            let a1413 = {
                let a1409 = v8;
                let a1410 = v4;
                let a1411 = 0u64;
                nat_quot(a1409, a1410, a1411)
            };
            nat_add(a1412, a1413)?
        };
        let v14: u64 = {
            let a1433 = v6;
            let a1434 = {
                let a1431 = match v1.clone() {
                    Adt3::C0(h1414, _, _, _, _, _, _, _) => {
                        h1414
                    }
                };
                let a1432 = {
                    let a1429 = {
                        let a1415 = v8;
                        let a1416 = v4;
                        let a1417 = 0u64;
                        nat_rem(a1415, a1416, a1417)
                    };
                    let a1430 = {
                        let a1427 = match v1.clone() {
                            Adt3::C0(_, h1418, _, _, _, _, _, _) => {
                                h1418
                            }
                        };
                        let a1428 = {
                            let a1425 = {
                                let a1419 = v10;
                                let a1420 = v4;
                                let a1421 = 0u64;
                                nat_rem(a1419, a1420, a1421)
                            };
                            let a1426 = {
                                let a1423 = match v1.clone() {
                                    Adt3::C0(_, _, h1422, _, _, _, _, _) => {
                                        h1422
                                    }
                                };
                                let a1424 = v13;
                                nat_mul(a1423, a1424)?
                            };
                            nat_add(a1425, a1426)?
                        };
                        nat_mul(a1427, a1428)?
                    };
                    nat_add(a1429, a1430)?
                };
                nat_mul(a1431, a1432)?
            };
            nat_add(a1433, a1434)?
        };
        Ok(Adt5::C1(false, f39(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a1440 = {
                let a1438 = {
                    let a1435 = v11;
                    let a1436 = v5;
                    let a1437 = 0u64;
                    nat_rem(a1435, a1436, a1437)
                };
                let a1439 = v4;
                nat_mul(a1438, a1439)?
            };
            let a1441 = v8;
            nat_add(a1440, a1441)?
        };
        let v16: u64 = {
            let a1447 = {
                let a1445 = {
                    let a1442 = v11;
                    let a1443 = v5;
                    let a1444 = 0u64;
                    nat_quot(a1442, a1443, a1444)
                };
                let a1446 = v4;
                nat_mul(a1445, a1446)?
            };
            let a1448 = v10;
            nat_add(a1447, a1448)?
        };
        let a1450 = v15;
        let a1451 = match v1.clone() {
            Adt3::C0(_, h1449, _, _, _, _, _, _) => {
                h1449
            }
        };
        if if nat_lt(a1450, a1451) {
            let a1453 = v16;
            let a1454 = match v1.clone() {
                Adt3::C0(_, _, h1452, _, _, _, _, _) => {
                    h1452
                }
            };
            nat_lt(a1453, a1454)
        } else {
            false
        } {
            Ok(Adt5::C1(false, f39({
                let a1463 = v6;
                let a1464 = {
                    let a1461 = match v1.clone() {
                        Adt3::C0(h1455, _, _, _, _, _, _, _) => {
                            h1455
                        }
                    };
                    let a1462 = {
                        let a1459 = v15;
                        let a1460 = {
                            let a1457 = match v1.clone() {
                                Adt3::C0(_, h1456, _, _, _, _, _, _) => {
                                    h1456
                                }
                            };
                            let a1458 = v16;
                            nat_mul(a1457, a1458)?
                        };
                        nat_add(a1459, a1460)?
                    };
                    nat_mul(a1461, a1462)?
                };
                nat_add(a1463, a1464)?
            }, v1)?, v12))
        } else {
            Ok(Adt5::C1(true, 0u64, v12))
        }
    }
}

pub fn f50(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt5> {
    let v3: u64 = f39(v0, v2.clone())?;
    let v4: u64 = f39(v0, v1.clone())?;
    let a1469 = {
        let a1466 = v0;
        let a1467 = match v1 {
            Adt3::C0(h1465, _, _, _, _, _, _, _) => {
                h1465
            }
        };
        let a1468 = 0u64;
        nat_rem(a1466, a1467, a1468)
    };
    let a1470 = 0u64;
    if nat_eq(a1469, a1470) {
        Ok(Adt5::C1(false, v4, v3))
    } else {
        Ok(Adt5::C4(8u64, 3u64, f39({
            let a1471 = v0;
            let a1472 = 1u64;
            nat_sub(a1471, a1472)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f51(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a1473 = v3;
    let a1474 = 13u64;
    if nat_eq(a1473, a1474) {
        f90(v0, v1, v4)
    } else {
        let v5: u64 = f91(match v1.clone() {
            Adt3::C0(h1475, _, _, _, _, _, _, _) => {
                h1475
            }
        }, {
            let a1476 = v3;
            let a1477 = 16u64;
            nat_eq(a1476, a1477)
        })?;
        let v6: u64 = {
            let a1478 = v0;
            let a1479 = v5;
            let a1480 = 0u64;
            nat_quot(a1478, a1479, a1480)
        };
        let v7: u64 = {
            let a1481 = v0;
            let a1482 = v5;
            let a1483 = 0u64;
            nat_rem(a1481, a1482, a1483)
        };
        let v8: u64 = {
            let a1487 = 1u64;
            let a1488 = {
                let a1485 = match v1.clone() {
                    Adt3::C0(h1484, _, _, _, _, _, _, _) => {
                        h1484
                    }
                };
                let a1486 = 2u64;
                nat_mul(a1485, a1486)?
            };
            nat_add(a1487, a1488)?
        };
        let a1489 = v7;
        let a1490 = v8;
        if nat_lt(a1489, a1490) {
            f92(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f39(v6, v2)?;
            let a1491 = v3;
            let a1492 = 14u64;
            if nat_eq(a1491, a1492) {
                Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a1493 = v7;
                let a1494 = v8;
                if nat_eq(a1493, a1494) {
                    Ok(Adt5::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a1497 = v7;
                    let a1498 = {
                        let a1495 = v8;
                        let a1496 = 1u64;
                        nat_add(a1495, a1496)?
                    };
                    if nat_eq(a1497, a1498) {
                        Ok(Adt5::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h1499, _, _, _, _, _, _, _) => {
                                h1499
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

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt5> {
    let a1500 = v0;
    let a1501 = 0u64;
    if nat_eq(a1500, a1501) {
        Ok(Adt5::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1508 = {
            let a1502 = v0;
            let a1503 = 1u64;
            nat_add(a1502, a1503)?
        };
        let a1509 = {
            let a1506 = 2u64;
            let a1507 = {
                let a1504 = v3;
                let a1505 = 2u64;
                nat_mul(a1504, a1505)?
            };
            nat_add(a1506, a1507)?
        };
        if nat_eq(a1508, a1509) {
            Ok(Adt5::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a1510 = v0;
                let a1511 = 1u64;
                nat_sub(a1510, a1511)
            };
            let v5: u64 = {
                let a1512 = v4;
                let a1513 = 2u64;
                let a1514 = 0u64;
                nat_quot(a1512, a1513, a1514)
            };
            let a1518 = {
                let a1515 = v4;
                let a1516 = 2u64;
                let a1517 = 0u64;
                nat_rem(a1515, a1516, a1517)
            };
            let a1519 = 0u64;
            if nat_eq(a1518, a1519) {
                Ok(Adt5::C4(17u64, 0u64, f39(v5, v1)?, 4u64, f39(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt5::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f53(v0: u64) -> R<u64> {
    let a1522 = 3u64;
    let a1523 = {
        let a1520 = v0;
        let a1521 = 3u64;
        nat_mul(a1520, a1521)?
    };
    nat_add(a1522, a1523)
}

pub fn f54(v0: Adt3, v1: Adt3) -> R<u64> {
    let a1531 = {
        let a1524 = 2u64;
        let a1525 = f55(v1.clone())?;
        nat_mul(a1524, a1525)?
    };
    let a1532 = {
        let a1529 = f55(v1)?;
        let a1530 = {
            let a1527 = 1u64;
            let a1528 = match v0 {
                Adt3::C0(h1526, _, _, _, _, _, _, _) => {
                    h1526
                }
            };
            nat_add(a1527, a1528)?
        };
        nat_mul(a1529, a1530)?
    };
    nat_add(a1531, a1532)
}

pub fn f55(v0: Adt3) -> R<u64> {
    if f63(match v0.clone() {
        Adt3::C0(h1533, _, _, _, _, _, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt3::C0(_, h1534, _, _, _, _, _, _) => {
            h1534
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h1535, _, _, _, _, _) => {
            h1535
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h1536, _, _, _, _) => {
            h1536
        }
    }) {
        Ok(0u64)
    } else {
        let a1545 = match v0.clone() {
            Adt3::C0(h1537, _, _, _, _, _, _, _) => {
                h1537
            }
        };
        let a1546 = {
            let a1543 = match v0.clone() {
                Adt3::C0(_, h1538, _, _, _, _, _, _) => {
                    h1538
                }
            };
            let a1544 = {
                let a1541 = match v0.clone() {
                    Adt3::C0(_, _, h1539, _, _, _, _, _) => {
                        h1539
                    }
                };
                let a1542 = match v0 {
                    Adt3::C0(_, _, _, h1540, _, _, _, _) => {
                        h1540
                    }
                };
                nat_mul(a1541, a1542)?
            };
            nat_mul(a1543, a1544)?
        };
        nat_mul(a1545, a1546)
    }
}

pub fn f56(v0: u64, v1: Adt3) -> R<Adt5> {
    Ok(Adt5::C4(19u64, 4u64, f39(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f57(v0: u64, v1: u64) -> bool {
    let a1547 = v1;
    let a1548 = 2147483647u64;
    if nat_le(a1547, a1548) {
        let a1549 = v1;
        let a1550 = v0;
        nat_lt(a1549, a1550)
    } else {
        false
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt5> {
    let m1551 = f66(v0);
    match m1551 {
        Adt7::C0 => {
            Ok(Adt5::C0(Adt6::C1))
        }
        Adt7::C1(v7) => {
            let v8: u64 = {
                let a1553 = v2;
                let a1554 = match v7.clone() {
                    Adt8::C0(_, h1552, _) => {
                        h1552
                    }
                };
                nat_mul(a1553, a1554)?
            };
            let m1567 = f93(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h1555, _, _, _, _, _, _, _) => {
                    h1555
                }
            }, match v5.clone() {
                Adt3::C0(_, h1556, _, _, _, _, _, _) => {
                    h1556
                }
            }, match v5 {
                Adt3::C0(_, _, h1557, _, _, _, _, _) => {
                    h1557
                }
            }, {
                let a1560 = match v3.clone() {
                    Adt3::C0(h1558, _, _, _, _, _, _, _) => {
                        h1558
                    }
                };
                let a1561 = match v7.clone() {
                    Adt8::C0(_, h1559, _) => {
                        h1559
                    }
                };
                nat_mul(a1560, a1561)?
            }, match v3.clone() {
                Adt3::C0(_, h1562, _, _, _, _, _, _) => {
                    h1562
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h1563, _, _, _) => {
                    h1563
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h1564, _, _) => {
                    h1564
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h1565, _) => {
                    h1565
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h1566) => {
                    h1566
                }
            })?;
            match m1567 {
                None => {
                    Ok(Adt5::C0(Adt6::C1))
                }
                Some(v9) => {
                    let a1568 = v0;
                    let a1569 = 0u64;
                    if if nat_eq(a1568, a1569) {
                        true
                    } else {
                        let a1570 = v0;
                        let a1571 = 26u64;
                        nat_eq(a1570, a1571)
                    } {
                        Ok(Adt5::C1(false, v9, f39({
                            let a1575 = {
                                let a1573 = v1;
                                let a1574 = match v4.clone() {
                                    Adt3::C0(h1572, _, _, _, _, _, _, _) => {
                                        h1572
                                    }
                                };
                                nat_mul(a1573, a1574)?
                            };
                            let a1576 = v8;
                            nat_add(a1575, a1576)?
                        }, v4)?))
                    } else {
                        Ok(Adt5::C5(v0, v9, match v7.clone() {
                            Adt8::C0(_, _, h1577) => {
                                h1577
                            }
                        }, f39({
                            let a1581 = {
                                let a1579 = v1;
                                let a1580 = match v4.clone() {
                                    Adt3::C0(h1578, _, _, _, _, _, _, _) => {
                                        h1578
                                    }
                                };
                                nat_mul(a1579, a1580)?
                            };
                            let a1582 = v8;
                            nat_add(a1581, a1582)?
                        }, v4.clone())?, {
                            let a1589 = {
                                let a1587 = {
                                    let a1584 = match v7 {
                                        Adt8::C0(_, h1583, _) => {
                                            h1583
                                        }
                                    };
                                    let a1585 = 1u64;
                                    nat_sub(a1584, a1585)
                                };
                                let a1588 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h1586, _, _, _) => {
                                        h1586
                                    }
                                };
                                nat_mul(a1587, a1588)?
                            };
                            let a1590 = 4u64;
                            nat_add(a1589, a1590)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h1591, _, _, _) => {
                                h1591
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64) -> u64 {
    let a1592 = v1;
    let a1593 = 0u64;
    if if nat_eq(a1592, a1593) {
        true
    } else {
        let a1594 = v2;
        let a1595 = 0u64;
        nat_eq(a1594, a1595)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f60(v0: u64, v1: u64) -> bool {
    let a1596 = v0;
    let a1597 = 4294967295u64;
    if nat_eq(a1596, a1597) {
        true
    } else {
        let a1598 = v0;
        let a1599 = 2147483648u64;
        if nat_lt(a1598, a1599) {
            let a1600 = v0;
            let a1601 = v1;
            nat_lt(a1600, a1601)
        } else {
            false
        }
    }
}

pub fn f61(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a1602 = v0;
        let a1603 = 8388608u64;
        let a1604 = 0u64;
        nat_quot(a1602, a1603, a1604)
    };
    let a1605 = v0;
    let a1606 = 2147483648u64;
    if if if nat_lt(a1605, a1606) {
        let a1607 = 127u64;
        let a1608 = v1;
        nat_le(a1607, a1608)
    } else {
        false
    } {
        let a1609 = v1;
        let a1610 = 190u64;
        nat_lt(a1609, a1610)
    } else {
        false
    } {
        let v2: u64 = {
            let a1614 = 8388608u64;
            let a1615 = {
                let a1611 = v0;
                let a1612 = 8388608u64;
                let a1613 = 0u64;
                nat_rem(a1611, a1612, a1613)
            };
            nat_add(a1614, a1615)?
        };
        let a1616 = v1;
        let a1617 = 150u64;
        if nat_lt(a1616, a1617) {
            let v3: u64 = f94({
                let a1618 = 150u64;
                let a1619 = v1;
                nat_sub(a1618, a1619)
            });
            let a1623 = {
                let a1620 = v2;
                let a1621 = v3;
                let a1622 = 0u64;
                nat_rem(a1620, a1621, a1622)
            };
            let a1624 = 0u64;
            if nat_eq(a1623, a1624) {
                let a1625 = v2;
                let a1626 = v3;
                let a1627 = 0u64;
                Ok(nat_quot(a1625, a1626, a1627))
            } else {
                Ok(0u64)
            }
        } else {
            let a1630 = f94({
                let a1628 = v1;
                let a1629 = 150u64;
                nat_sub(a1628, a1629)
            });
            let a1631 = v2;
            nat_mul(a1630, a1631)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f62(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt6> {
    if f95(match v0.clone() {
        Adt2::C0(h1632, _, _, _, _, _, _, _, _, _, _) => {
            h1632
        }
    }, match v0.clone() {
        Adt2::C0(_, h1633, _, _, _, _, _, _, _, _, _) => {
            h1633
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1634, _, _, _, _, _, _, _, _) => {
            h1634
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1635, _, _, _, _, _, _, _) => {
            h1635
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1636, _, _, _, _, _, _) => {
            h1636
        }
    }, match v1.clone() {
        Adt2::C0(h1637, _, _, _, _, _, _, _, _, _, _) => {
            h1637
        }
    }, match v1.clone() {
        Adt2::C0(_, h1638, _, _, _, _, _, _, _, _, _) => {
            h1638
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h1639, _, _, _, _, _, _, _, _) => {
            h1639
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h1640, _, _, _, _, _, _, _) => {
            h1640
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h1641, _, _, _, _, _, _) => {
            h1641
        }
    }, match v2.clone() {
        Adt2::C0(h1642, _, _, _, _, _, _, _, _, _, _) => {
            h1642
        }
    }, match v2.clone() {
        Adt2::C0(_, h1643, _, _, _, _, _, _, _, _, _) => {
            h1643
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h1644, _, _, _, _, _, _, _, _) => {
            h1644
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h1645, _, _, _, _, _, _, _) => {
            h1645
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h1646, _, _, _, _, _, _) => {
            h1646
        }
    }) {
        let m1648 = f66(match v0.clone() {
            Adt2::C0(h1647, _, _, _, _, _, _, _, _, _, _) => {
                h1647
            }
        });
        match m1648 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v3) => {
                let a1654 = {
                    let a1651 = match v0.clone() {
                        Adt2::C0(_, h1649, _, _, _, _, _, _, _, _, _) => {
                            h1649
                        }
                    };
                    let a1652 = match v3.clone() {
                        Adt8::C0(_, h1650, _) => {
                            h1650
                        }
                    };
                    let a1653 = 0u64;
                    nat_rem(a1651, a1652, a1653)
                };
                let a1655 = 0u64;
                if if nat_eq(a1654, a1655) {
                    if f96(match v0.clone() {
                        Adt2::C0(_, h1656, _, _, _, _, _, _, _, _, _) => {
                            h1656
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1657, _, _, _, _, _, _, _, _, _) => {
                            h1657
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1658, _, _, _, _, _, _, _, _) => {
                            h1658
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1659, _, _, _, _, _, _, _) => {
                            h1659
                        }
                    }) {
                        true
                    } else {
                        let a1661 = 4u64;
                        let a1662 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h1660, _, _, _, _, _) => {
                                h1660
                            }
                        };
                        if if nat_le(a1661, a1662) {
                            let a1664 = match v3.clone() {
                                Adt8::C0(_, h1663, _) => {
                                    h1663
                                }
                            };
                            let a1665 = 1u64;
                            if nat_eq(a1664, a1665) {
                                true
                            } else {
                                let a1668 = match v3.clone() {
                                    Adt8::C0(_, _, h1666) => {
                                        h1666
                                    }
                                };
                                let a1669 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h1667, _, _, _, _, _) => {
                                        h1667
                                    }
                                };
                                nat_le(a1668, a1669)
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
                    let v4: Adt3 = f14(v0.clone(), match v3 {
                        Adt8::C0(_, h1670, _) => {
                            h1670
                        }
                    });
                    Ok(Adt6::C4(if f96(match v0.clone() {
                        Adt2::C0(_, h1671, _, _, _, _, _, _, _, _, _) => {
                            h1671
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h1672, _, _, _, _, _, _, _, _, _) => {
                            h1672
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h1673, _, _, _, _, _, _, _, _) => {
                            h1673
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h1674, _, _, _, _, _, _, _) => {
                            h1674
                        }
                    }) {
                        0u64
                    } else {
                        f54(v4.clone(), f14(v1.clone(), 1u64))?
                    }, 4u64, v4, f14(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h1675, _, _, _, _, _, _, _, _, _, _) => {
                            h1675
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

pub fn f63(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1676 = v0;
    let a1677 = 0u64;
    if if if nat_eq(a1676, a1677) {
        true
    } else {
        let a1678 = v1;
        let a1679 = 0u64;
        nat_eq(a1678, a1679)
    } {
        true
    } else {
        let a1680 = v2;
        let a1681 = 0u64;
        nat_eq(a1680, a1681)
    } {
        true
    } else {
        let a1682 = v3;
        let a1683 = 0u64;
        nat_eq(a1682, a1683)
    }
}

pub fn f64(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt6> {
    let a1686 = match v1.clone() {
        Adt2::C0(h1684, _, _, _, _, _, _, _, _, _, _) => {
            h1684
        }
    };
    let a1687 = match v3.clone() {
        Adt2::C0(h1685, _, _, _, _, _, _, _, _, _, _) => {
            h1685
        }
    };
    if if nat_eq(a1686, a1687) {
        let a1688 = v0;
        let a1689 = 33u64;
        if nat_eq(a1688, a1689) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m1691 = f66(match v1.clone() {
            Adt2::C0(h1690, _, _, _, _, _, _, _, _, _, _) => {
                h1690
            }
        });
        match m1691 {
            Adt7::C0 => {
                Ok(Adt6::C0)
            }
            Adt7::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt8::C0(_, h1692, _) => {
                        h1692
                    }
                };
                let v6: u64 = f10(v1.clone(), v5)?;
                let a1697 = {
                    let a1694 = match v1.clone() {
                        Adt2::C0(_, h1693, _, _, _, _, _, _, _, _, _) => {
                            h1693
                        }
                    };
                    let a1695 = v5;
                    let a1696 = 1u64;
                    nat_rem(a1694, a1695, a1696)
                };
                let a1698 = 0u64;
                if if if nat_eq(a1697, a1698) {
                    let a1703 = {
                        let a1700 = match v3.clone() {
                            Adt2::C0(_, h1699, _, _, _, _, _, _, _, _, _) => {
                                h1699
                            }
                        };
                        let a1701 = v5;
                        let a1702 = 1u64;
                        nat_rem(a1700, a1701, a1702)
                    };
                    let a1704 = 0u64;
                    nat_eq(a1703, a1704)
                } else {
                    false
                } {
                    let a1705 = v6;
                    let a1706 = f10(v3.clone(), v5)?;
                    nat_eq(a1705, a1706)
                } else {
                    false
                } {
                    Ok(Adt6::C4(v6, match v4 {
                        Adt8::C0(_, _, h1707) => {
                            h1707
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

pub fn f65(v0: u64) -> bool {
    let a1708 = v0;
    let a1709 = 0u64;
    if if nat_eq(a1708, a1709) {
        true
    } else {
        let a1710 = v0;
        let a1711 = 1u64;
        nat_eq(a1710, a1711)
    } {
        true
    } else {
        let a1712 = v0;
        let a1713 = 30u64;
        nat_eq(a1712, a1713)
    }
}

pub fn f66(v0: u64) -> Adt7 {
    let a1714 = v0;
    let a1715 = 0u64;
    if nat_eq(a1714, a1715) {
        Adt7::C1(Adt8::C0(0u64, 1u64, 4u64))
    } else {
        let a1716 = v0;
        let a1717 = 1u64;
        if nat_eq(a1716, a1717) {
            Adt7::C1(Adt8::C0(1u64, 1u64, 2u64))
        } else {
            let a1718 = v0;
            let a1719 = 2u64;
            if nat_eq(a1718, a1719) {
                Adt7::C1(Adt8::C0(2u64, 32u64, 18u64))
            } else {
                let a1720 = v0;
                let a1721 = 3u64;
                if nat_eq(a1720, a1721) {
                    Adt7::C1(Adt8::C0(3u64, 32u64, 20u64))
                } else {
                    let a1722 = v0;
                    let a1723 = 6u64;
                    if nat_eq(a1722, a1723) {
                        Adt7::C1(Adt8::C0(6u64, 32u64, 22u64))
                    } else {
                        let a1724 = v0;
                        let a1725 = 7u64;
                        if nat_eq(a1724, a1725) {
                            Adt7::C1(Adt8::C0(7u64, 32u64, 24u64))
                        } else {
                            let a1726 = v0;
                            let a1727 = 8u64;
                            if nat_eq(a1726, a1727) {
                                Adt7::C1(Adt8::C0(8u64, 32u64, 34u64))
                            } else {
                                let a1728 = v0;
                                let a1729 = 9u64;
                                if nat_eq(a1728, a1729) {
                                    Adt7::C1(Adt8::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a1730 = v0;
                                    let a1731 = 10u64;
                                    if nat_eq(a1730, a1731) {
                                        Adt7::C1(Adt8::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a1732 = v0;
                                        let a1733 = 11u64;
                                        if nat_eq(a1732, a1733) {
                                            Adt7::C1(Adt8::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a1734 = v0;
                                            let a1735 = 12u64;
                                            if nat_eq(a1734, a1735) {
                                                Adt7::C1(Adt8::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a1736 = v0;
                                                let a1737 = 13u64;
                                                if nat_eq(a1736, a1737) {
                                                    Adt7::C1(Adt8::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a1738 = v0;
                                                    let a1739 = 14u64;
                                                    if nat_eq(a1738, a1739) {
                                                        Adt7::C1(Adt8::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a1740 = v0;
                                                        let a1741 = 15u64;
                                                        if nat_eq(a1740, a1741) {
                                                            Adt7::C1(Adt8::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a1742 = v0;
                                                            let a1743 = 16u64;
                                                            if nat_eq(a1742, a1743) {
                                                                Adt7::C1(Adt8::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a1744 = v0;
                                                                let a1745 = 17u64;
                                                                if nat_eq(a1744, a1745) {
                                                                    Adt7::C1(Adt8::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a1746 = v0;
                                                                    let a1747 = 18u64;
                                                                    if nat_eq(a1746, a1747) {
                                                                        Adt7::C1(Adt8::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a1748 = v0;
                                                                        let a1749 = 19u64;
                                                                        if nat_eq(a1748, a1749) {
                                                                            Adt7::C1(Adt8::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a1750 = v0;
                                                                            let a1751 = 20u64;
                                                                            if nat_eq(a1750, a1751) {
                                                                                Adt7::C1(Adt8::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a1752 = v0;
                                                                                let a1753 = 21u64;
                                                                                if nat_eq(a1752, a1753) {
                                                                                    Adt7::C1(Adt8::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a1754 = v0;
                                                                                    let a1755 = 22u64;
                                                                                    if nat_eq(a1754, a1755) {
                                                                                        Adt7::C1(Adt8::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a1756 = v0;
                                                                                        let a1757 = 23u64;
                                                                                        if nat_eq(a1756, a1757) {
                                                                                            Adt7::C1(Adt8::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a1758 = v0;
                                                                                            let a1759 = 24u64;
                                                                                            if nat_eq(a1758, a1759) {
                                                                                                Adt7::C1(Adt8::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a1760 = v0;
                                                                                                let a1761 = 25u64;
                                                                                                if nat_eq(a1760, a1761) {
                                                                                                    Adt7::C1(Adt8::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a1762 = v0;
                                                                                                    let a1763 = 26u64;
                                                                                                    if nat_eq(a1762, a1763) {
                                                                                                        Adt7::C1(Adt8::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a1764 = v0;
                                                                                                        let a1765 = 27u64;
                                                                                                        if nat_eq(a1764, a1765) {
                                                                                                            Adt7::C1(Adt8::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a1766 = v0;
                                                                                                            let a1767 = 28u64;
                                                                                                            if nat_eq(a1766, a1767) {
                                                                                                                Adt7::C1(Adt8::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a1768 = v0;
                                                                                                                let a1769 = 29u64;
                                                                                                                if nat_eq(a1768, a1769) {
                                                                                                                    Adt7::C1(Adt8::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a1770 = v0;
                                                                                                                    let a1771 = 30u64;
                                                                                                                    if nat_eq(a1770, a1771) {
                                                                                                                        Adt7::C1(Adt8::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a1772 = v0;
                                                                                                                        let a1773 = 34u64;
                                                                                                                        if nat_eq(a1772, a1773) {
                                                                                                                            Adt7::C1(Adt8::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a1774 = v0;
                                                                                                                            let a1775 = 35u64;
                                                                                                                            if nat_eq(a1774, a1775) {
                                                                                                                                Adt7::C1(Adt8::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a1776 = v0;
                                                                                                                                let a1777 = 39u64;
                                                                                                                                if nat_eq(a1776, a1777) {
                                                                                                                                    Adt7::C1(Adt8::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a1778 = v0;
                                                                                                                                    let a1779 = 40u64;
                                                                                                                                    if nat_eq(a1778, a1779) {
                                                                                                                                        Adt7::C1(Adt8::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a1780 = v0;
                                                                                                                                        let a1781 = 41u64;
                                                                                                                                        if nat_eq(a1780, a1781) {
                                                                                                                                            Adt7::C1(Adt8::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a1782 = v0;
                                                                                                                                            let a1783 = 42u64;
                                                                                                                                            if nat_eq(a1782, a1783) {
                                                                                                                                                Adt7::C1(Adt8::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a1784 = v0;
                                                                                                                                                let a1785 = 43u64;
                                                                                                                                                if nat_eq(a1784, a1785) {
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

pub fn f67(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f63(v4, v5, v6, v7) {
        f63(v0, v1, v2, v3)
    } else {
        let a1789 = {
            let a1786 = v0;
            let a1787 = v4;
            let a1788 = 1u64;
            nat_rem(a1786, a1787, a1788)
        };
        let a1790 = 0u64;
        if if if nat_eq(a1789, a1790) {
            let a1794 = {
                let a1791 = v1;
                let a1792 = v5;
                let a1793 = 1u64;
                nat_rem(a1791, a1792, a1793)
            };
            let a1795 = 0u64;
            nat_eq(a1794, a1795)
        } else {
            false
        } {
            let a1799 = {
                let a1796 = v2;
                let a1797 = v6;
                let a1798 = 1u64;
                nat_rem(a1796, a1797, a1798)
            };
            let a1800 = 0u64;
            nat_eq(a1799, a1800)
        } else {
            false
        } {
            let a1804 = {
                let a1801 = v3;
                let a1802 = v7;
                let a1803 = 1u64;
                nat_rem(a1801, a1802, a1803)
            };
            let a1805 = 0u64;
            nat_eq(a1804, a1805)
        } else {
            false
        }
    }
}

pub fn f68(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a1806 = v0;
    let a1807 = v1;
    if nat_eq(a1806, a1807) {
        let a1810 = v4;
        let a1811 = {
            let a1808 = v2;
            let a1809 = v3;
            nat_add(a1808, a1809)?
        };
        Ok(nat_eq(a1810, a1811))
    } else {
        let a1812 = v2;
        let a1813 = v3;
        if nat_eq(a1812, a1813) {
            let a1814 = v4;
            let a1815 = v2;
            Ok(nat_eq(a1814, a1815))
        } else {
            Ok(false)
        }
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a1818 = {
            let a1816 = v0;
            let a1817 = f97(v1);
            nat_add(a1816, a1817)?
        };
        let a1819 = f97(v2);
        nat_add(a1818, a1819)?
    };
    let v5: u64 = {
        let a1820 = f98(v1);
        let a1821 = f98(v2);
        nat_add(a1820, a1821)?
    };
    let a1822 = v1;
    let a1823 = 4294967295u64;
    if if nat_le(a1822, a1823) {
        let a1824 = v2;
        let a1825 = 4294967295u64;
        nat_le(a1824, a1825)
    } else {
        false
    } {
        let a1826 = v5;
        let a1827 = v4;
        if nat_le(a1826, a1827) {
            let a1830 = v3;
            let a1831 = {
                let a1828 = v4;
                let a1829 = v5;
                nat_sub(a1828, a1829)
            };
            Ok(nat_eq(a1830, a1831))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f70(v0: u64, v1: u64) -> R<bool> {
    let a1832 = v1;
    let a1833 = 4294967295u64;
    if nat_le(a1832, a1833) {
        let a1836 = {
            let a1834 = f97(v1);
            let a1835 = f98(v1);
            nat_add(a1834, a1835)?
        };
        let a1837 = v0;
        Ok(nat_lt(a1836, a1837))
    } else {
        Ok(false)
    }
}

pub fn f71(v0: Adt2) -> R<bool> {
    let a1839 = match v0.clone() {
        Adt2::C0(_, h1838, _, _, _, _, _, _, _, _, _) => {
            h1838
        }
    };
    let a1840 = 1u64;
    if if if if nat_eq(a1839, a1840) {
        true
    } else {
        let a1842 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1841, _, _, _, _, _) => {
                h1841
            }
        };
        let a1843 = 4u64;
        nat_eq(a1842, a1843)
    } {
        let a1845 = match v0.clone() {
            Adt2::C0(_, _, h1844, _, _, _, _, _, _, _, _) => {
                h1844
            }
        };
        let a1846 = 1u64;
        if nat_eq(a1845, a1846) {
            true
        } else {
            let a1851 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1847, _, _, _, _) => {
                    h1847
                }
            };
            let a1852 = {
                let a1849 = match v0.clone() {
                    Adt2::C0(_, h1848, _, _, _, _, _, _, _, _, _) => {
                        h1848
                    }
                };
                let a1850 = 4u64;
                nat_mul(a1849, a1850)?
            };
            nat_eq(a1851, a1852)
        }
    } else {
        false
    } {
        let a1854 = match v0.clone() {
            Adt2::C0(_, _, _, h1853, _, _, _, _, _, _, _) => {
                h1853
            }
        };
        let a1855 = 1u64;
        if nat_eq(a1854, a1855) {
            true
        } else {
            let a1863 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1856, _, _, _) => {
                    h1856
                }
            };
            let a1864 = {
                let a1861 = {
                    let a1859 = match v0.clone() {
                        Adt2::C0(_, h1857, _, _, _, _, _, _, _, _, _) => {
                            h1857
                        }
                    };
                    let a1860 = match v0.clone() {
                        Adt2::C0(_, _, h1858, _, _, _, _, _, _, _, _) => {
                            h1858
                        }
                    };
                    nat_mul(a1859, a1860)?
                };
                let a1862 = 4u64;
                nat_mul(a1861, a1862)?
            };
            nat_eq(a1863, a1864)
        }
    } else {
        false
    } {
        let a1866 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1865, _, _, _, _, _, _) => {
                h1865
            }
        };
        let a1867 = 1u64;
        if nat_eq(a1866, a1867) {
            Ok(true)
        } else {
            let a1878 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1868, _, _) => {
                    h1868
                }
            };
            let a1879 = {
                let a1876 = {
                    let a1874 = {
                        let a1871 = match v0.clone() {
                            Adt2::C0(_, h1869, _, _, _, _, _, _, _, _, _) => {
                                h1869
                            }
                        };
                        let a1872 = match v0.clone() {
                            Adt2::C0(_, _, h1870, _, _, _, _, _, _, _, _) => {
                                h1870
                            }
                        };
                        nat_mul(a1871, a1872)?
                    };
                    let a1875 = match v0 {
                        Adt2::C0(_, _, _, h1873, _, _, _, _, _, _, _) => {
                            h1873
                        }
                    };
                    nat_mul(a1874, a1875)?
                };
                let a1877 = 4u64;
                nat_mul(a1876, a1877)?
            };
            Ok(nat_eq(a1878, a1879))
        }
    } else {
        Ok(false)
    }
}

pub fn f72(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f99(v0.clone(), false)?;
    let v6: u64 = f99(v0, true)?;
    let a1880 = v6;
    let a1881 = v1;
    if if nat_le(a1880, a1881) {
        let a1888 = {
            let a1886 = {
                let a1882 = v1;
                let a1883 = v5;
                nat_add(a1882, a1883)?
            };
            let a1887 = {
                let a1884 = v3;
                let a1885 = 0u64;
                if nat_eq(a1884, a1885) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a1886, a1887)?
        };
        let a1889 = v2;
        nat_le(a1888, a1889)
    } else {
        false
    } {
        if v4 {
            let a1894 = {
                let a1892 = {
                    let a1890 = v1;
                    let a1891 = v5;
                    nat_add(a1890, a1891)?
                };
                let a1893 = v6;
                nat_sub(a1892, a1893)
            };
            let a1895 = v2;
            Ok(nat_lt(a1894, a1895))
        } else {
            let a1900 = {
                let a1898 = {
                    let a1896 = v1;
                    let a1897 = v5;
                    nat_add(a1896, a1897)?
                };
                let a1899 = v6;
                nat_sub(a1898, a1899)
            };
            let a1901 = v2;
            Ok(nat_le(a1900, a1901))
        }
    } else {
        Ok(false)
    }
}

pub fn f73(v0: u64, v1: u64) -> bool {
    let a1902 = v0;
    let a1903 = 0u64;
    if if nat_eq(a1902, a1903) {
        true
    } else {
        let a1904 = v0;
        let a1905 = 1u64;
        nat_eq(a1904, a1905)
    } {
        let a1906 = v1;
        let a1907 = 4294967296u64;
        nat_lt(a1906, a1907)
    } else {
        false
    }
}

pub fn f74(v0: u64, v1: u64) -> R<u64> {
    let a1912 = {
        let a1910 = v0;
        let a1911 = {
            let a1908 = v1;
            let a1909 = 1u64;
            nat_sub(a1908, a1909)
        };
        nat_add(a1910, a1911)?
    };
    let a1913 = v1;
    let a1914 = 0u64;
    Ok(nat_quot(a1912, a1913, a1914))
}

pub fn f75(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f63(match v0.clone() {
        Adt2::C0(_, h1915, _, _, _, _, _, _, _, _, _) => {
            h1915
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1916, _, _, _, _, _, _, _, _) => {
            h1916
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1917, _, _, _, _, _, _, _) => {
            h1917
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1918, _, _, _, _, _, _) => {
            h1918
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a1921 = match v1.clone() {
                Adt2::C0(_, h1919, _, _, _, _, _, _, _, _, _) => {
                    h1919
                }
            };
            let a1922 = match v1.clone() {
                Adt2::C0(_, _, h1920, _, _, _, _, _, _, _, _) => {
                    h1920
                }
            };
            nat_mul(a1921, a1922)?
        };
        let v5: u64 = {
            let a1924 = v4;
            let a1925 = match v1.clone() {
                Adt2::C0(_, _, _, h1923, _, _, _, _, _, _, _) => {
                    h1923
                }
            };
            nat_mul(a1924, a1925)?
        };
        let a1938 = {
            let a1936 = {
                let a1927 = match v0.clone() {
                    Adt2::C0(_, h1926, _, _, _, _, _, _, _, _, _) => {
                        h1926
                    }
                };
                let a1928 = 1u64;
                nat_sub(a1927, a1928)
            };
            let a1937 = {
                let a1934 = f100(match v0.clone() {
                    Adt2::C0(_, _, h1929, _, _, _, _, _, _, _, _) => {
                        h1929
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h1930, _, _, _, _, _, _, _, _, _) => {
                        h1930
                    }
                })?;
                let a1935 = f100(match v0 {
                    Adt2::C0(_, _, _, h1931, _, _, _, _, _, _, _) => {
                        h1931
                    }
                }, v2, {
                    let a1932 = v3;
                    let a1933 = v5;
                    nat_mul(a1932, a1933)?
                }, v4)?;
                nat_add(a1934, a1935)?
            };
            nat_add(a1936, a1937)?
        };
        let a1939 = f10(v1, 1u64)?;
        Ok(nat_lt(a1938, a1939))
    }
}

pub fn f76(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a1940 = v0;
    let a1941 = 2147483648u64;
    if if if if nat_lt(a1940, a1941) {
        let a1942 = v1;
        let a1943 = 2147483648u64;
        nat_lt(a1942, a1943)
    } else {
        false
    } {
        let a1946 = {
            let a1944 = v0;
            let a1945 = v1;
            nat_mul(a1944, a1945)?
        };
        let a1947 = 2147483648u64;
        nat_lt(a1946, a1947)
    } else {
        false
    } {
        let a1952 = {
            let a1950 = {
                let a1948 = v0;
                let a1949 = 1u64;
                nat_sub(a1948, a1949)
            };
            let a1951 = v2;
            nat_mul(a1950, a1951)?
        };
        let a1953 = 2147483648u64;
        nat_lt(a1952, a1953)
    } else {
        false
    } {
        let a1958 = {
            let a1956 = {
                let a1954 = v1;
                let a1955 = 1u64;
                nat_sub(a1954, a1955)
            };
            let a1957 = v2;
            nat_mul(a1956, a1957)?
        };
        let a1959 = 2147483648u64;
        Ok(nat_lt(a1958, a1959))
    } else {
        Ok(false)
    }
}

pub fn f77(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a1960 = v1;
    let a1961 = 13u64;
    if if nat_eq(a1960, a1961) {
        f63(match v0.clone() {
            Adt3::C0(h1962, _, _, _, _, _, _, _) => {
                h1962
            }
        }, match v0.clone() {
            Adt3::C0(_, h1963, _, _, _, _, _, _) => {
                h1963
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h1964, _, _, _, _, _) => {
                h1964
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h1965, _, _, _, _) => {
                h1965
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a1971 = match v0.clone() {
                Adt3::C0(_, h1966, _, _, _, _, _, _) => {
                    h1966
                }
            };
            let a1972 = {
                let a1969 = match v0.clone() {
                    Adt3::C0(_, _, h1967, _, _, _, _, _) => {
                        h1967
                    }
                };
                let a1970 = match v0.clone() {
                    Adt3::C0(_, _, _, h1968, _, _, _, _) => {
                        h1968
                    }
                };
                nat_mul(a1969, a1970)?
            };
            nat_mul(a1971, a1972)?
        };
        let a1973 = v1;
        let a1974 = 13u64;
        if nat_eq(a1973, a1974) {
            let a1983 = 2u64;
            let a1984 = if f101(v0.clone(), v2) {
                let a1976 = v3;
                let a1977 = f91(match v0 {
                    Adt3::C0(h1975, _, _, _, _, _, _, _) => {
                        h1975
                    }
                }, false)?;
                nat_mul(a1976, a1977)?
            } else {
                let a1981 = v3;
                let a1982 = {
                    let a1979 = match v0 {
                        Adt3::C0(h1978, _, _, _, _, _, _, _) => {
                            h1978
                        }
                    };
                    let a1980 = 2u64;
                    nat_mul(a1979, a1980)?
                };
                nat_mul(a1981, a1982)?
            };
            nat_add(a1983, a1984)
        } else {
            let a1988 = v3;
            let a1989 = f91(match v0 {
                Adt3::C0(h1985, _, _, _, _, _, _, _) => {
                    h1985
                }
            }, {
                let a1986 = v1;
                let a1987 = 16u64;
                nat_eq(a1986, a1987)
            })?;
            nat_mul(a1988, a1989)
        }
    }
}

pub fn f78(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt6> {
    let m1990 = v3;
    match m1990 {
        None => {
            f102(v0, v1, match v2.clone() {
                Adt2::C0(h1991, _, _, _, _, _, _, _, _, _, _) => {
                    h1991
                }
            }, 0u64, match v4.clone() {
                Adt2::C0(h1992, _, _, _, _, _, _, _, _, _, _) => {
                    h1992
                }
            }, false, match v2.clone() {
                Adt2::C0(_, h1993, _, _, _, _, _, _, _, _, _) => {
                    h1993
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h1994, _, _, _, _, _, _, _, _) => {
                    h1994
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h1995, _, _, _, _, _, _, _) => {
                    h1995
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h1996, _, _, _, _, _, _) => {
                    h1996
                }
            }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                Adt2::C0(_, h1997, _, _, _, _, _, _, _, _, _) => {
                    h1997
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h1998, _, _, _, _, _, _, _, _) => {
                    h1998
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h1999, _, _, _, _, _, _, _) => {
                    h1999
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2000, _, _, _, _, _, _) => {
                    h2000
                }
            })
        }
        Some(v5) => {
            f102(v0, v1, match v2.clone() {
                Adt2::C0(h2001, _, _, _, _, _, _, _, _, _, _) => {
                    h2001
                }
            }, match v5.clone() {
                Adt2::C0(h2002, _, _, _, _, _, _, _, _, _, _) => {
                    h2002
                }
            }, match v4.clone() {
                Adt2::C0(h2003, _, _, _, _, _, _, _, _, _, _) => {
                    h2003
                }
            }, true, match v2.clone() {
                Adt2::C0(_, h2004, _, _, _, _, _, _, _, _, _) => {
                    h2004
                }
            }, match v2.clone() {
                Adt2::C0(_, _, h2005, _, _, _, _, _, _, _, _) => {
                    h2005
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h2006, _, _, _, _, _, _, _) => {
                    h2006
                }
            }, match v2 {
                Adt2::C0(_, _, _, _, h2007, _, _, _, _, _, _) => {
                    h2007
                }
            }, match v5.clone() {
                Adt2::C0(_, h2008, _, _, _, _, _, _, _, _, _) => {
                    h2008
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2009, _, _, _, _, _, _, _, _) => {
                    h2009
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2010, _, _, _, _, _, _, _) => {
                    h2010
                }
            }, match v5 {
                Adt2::C0(_, _, _, _, h2011, _, _, _, _, _, _) => {
                    h2011
                }
            }, match v4.clone() {
                Adt2::C0(_, h2012, _, _, _, _, _, _, _, _, _) => {
                    h2012
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h2013, _, _, _, _, _, _, _, _) => {
                    h2013
                }
            }, match v4.clone() {
                Adt2::C0(_, _, _, h2014, _, _, _, _, _, _, _) => {
                    h2014
                }
            }, match v4 {
                Adt2::C0(_, _, _, _, h2015, _, _, _, _, _, _) => {
                    h2015
                }
            })
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2028 = {
        let a2024 = {
            let a2020 = {
                let a2016 = v0;
                let a2017 = v4;
                nat_mul(a2016, a2017)?
            };
            let a2021 = {
                let a2018 = v1;
                let a2019 = v5;
                nat_mul(a2018, a2019)?
            };
            nat_add(a2020, a2021)?
        };
        let a2025 = {
            let a2022 = v2;
            let a2023 = v6;
            nat_mul(a2022, a2023)?
        };
        nat_add(a2024, a2025)?
    };
    let a2029 = {
        let a2026 = v3;
        let a2027 = v7;
        nat_mul(a2026, a2027)?
    };
    nat_add(a2028, a2029)
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2034 = {
        let a2032 = {
            let a2030 = v0;
            let a2031 = v1;
            nat_sub(a2030, a2031)
        };
        let a2033 = 1u64;
        nat_sub(a2032, a2033)
    };
    let a2035 = v2;
    nat_add(a2034, a2035)
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f79({
        let a2036 = v0;
        let a2037 = v4;
        let a2038 = 0u64;
        nat_rem(a2036, a2037, a2038)
    }, {
        let a2039 = v1;
        let a2040 = v5;
        let a2041 = 0u64;
        nat_rem(a2039, a2040, a2041)
    }, {
        let a2042 = v2;
        let a2043 = v6;
        let a2044 = 0u64;
        nat_rem(a2042, a2043, a2044)
    }, {
        let a2045 = v3;
        let a2046 = v7;
        let a2047 = 0u64;
        nat_rem(a2045, a2046, a2047)
    }, v8, v9, v10, v11)
}

pub fn f82(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2049 = v0;
        let a2050 = match v1.clone() {
            Adt3::C0(h2048, _, _, _, _, _, _, _) => {
                h2048
            }
        };
        let a2051 = 0u64;
        nat_quot(a2049, a2050, a2051)
    };
    let v4: u64 = {
        let a2053 = v3;
        let a2054 = match v1.clone() {
            Adt3::C0(_, h2052, _, _, _, _, _, _) => {
                h2052
            }
        };
        let a2055 = 0u64;
        nat_quot(a2053, a2054, a2055)
    };
    let a2056 = v2;
    let a2057 = 0u64;
    if nat_eq(a2056, a2057) {
        let a2059 = v0;
        let a2060 = match v1 {
            Adt3::C0(h2058, _, _, _, _, _, _, _) => {
                h2058
            }
        };
        let a2061 = 0u64;
        nat_rem(a2059, a2060, a2061)
    } else {
        let a2062 = v2;
        let a2063 = 1u64;
        if nat_eq(a2062, a2063) {
            let a2065 = v3;
            let a2066 = match v1 {
                Adt3::C0(_, h2064, _, _, _, _, _, _) => {
                    h2064
                }
            };
            let a2067 = 0u64;
            nat_rem(a2065, a2066, a2067)
        } else {
            let a2068 = v2;
            let a2069 = 2u64;
            if nat_eq(a2068, a2069) {
                let a2071 = v4;
                let a2072 = match v1 {
                    Adt3::C0(_, _, h2070, _, _, _, _, _) => {
                        h2070
                    }
                };
                let a2073 = 0u64;
                nat_rem(a2071, a2072, a2073)
            } else {
                let a2075 = v4;
                let a2076 = match v1 {
                    Adt3::C0(_, _, h2074, _, _, _, _, _) => {
                        h2074
                    }
                };
                let a2077 = 0u64;
                nat_quot(a2075, a2076, a2077)
            }
        }
    }
}

pub fn f83(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f79(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2078, _, _, _) => {
            h2078
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2079, _, _) => {
            h2079
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2080, _) => {
            h2080
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2081) => {
            h2081
        }
    })
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2082 = v1;
    let a2083 = 2147483648u64;
    if nat_lt(a2082, a2083) {
        let a2094 = {
            let a2092 = {
                let a2087 = {
                    let a2084 = v0;
                    let a2085 = v2;
                    let a2086 = 0u64;
                    nat_rem(a2084, a2085, a2086)
                };
                let a2088 = v2;
                nat_add(a2087, a2088)?
            };
            let a2093 = {
                let a2089 = v1;
                let a2090 = v2;
                let a2091 = 0u64;
                nat_rem(a2089, a2090, a2091)
            };
            nat_sub(a2092, a2093)
        };
        let a2095 = v2;
        let a2096 = 0u64;
        Ok(nat_rem(a2094, a2095, a2096))
    } else {
        let a2099 = {
            let a2097 = v0;
            let a2098 = f98(v1);
            nat_add(a2097, a2098)?
        };
        let a2100 = v2;
        let a2101 = 0u64;
        Ok(nat_rem(a2099, a2100, a2101))
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2104 = f97(v1);
    let a2105 = {
        let a2102 = v0;
        let a2103 = f98(v1);
        nat_add(a2102, a2103)?
    };
    if nat_le(a2104, a2105) {
        let a2106 = f86(v0, v1)?;
        let a2107 = v2;
        Ok(nat_lt(a2106, a2107))
    } else {
        Ok(false)
    }
}

pub fn f86(v0: u64, v1: u64) -> R<u64> {
    let a2110 = {
        let a2108 = v0;
        let a2109 = f98(v1);
        nat_add(a2108, a2109)?
    };
    let a2111 = f97(v1);
    Ok(nat_sub(a2110, a2111))
}

pub fn f87(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2112 = v0;
    let a2113 = v1;
    if nat_lt(a2112, a2113) {
        let a2114 = v1;
        let a2115 = v0;
        Ok(nat_sub(a2114, a2115))
    } else {
        let a2118 = v0;
        let a2119 = {
            let a2116 = v1;
            let a2117 = v2;
            nat_add(a2116, a2117)?
        };
        if nat_lt(a2118, a2119) {
            let a2120 = v0;
            let a2121 = v1;
            Ok(nat_sub(a2120, a2121))
        } else {
            let a2128 = {
                let a2124 = 2u64;
                let a2125 = {
                    let a2122 = v2;
                    let a2123 = 1u64;
                    nat_sub(a2122, a2123)
                };
                nat_mul(a2124, a2125)?
            };
            let a2129 = {
                let a2126 = v0;
                let a2127 = v1;
                nat_sub(a2126, a2127)
            };
            Ok(nat_sub(a2128, a2129))
        }
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a2132 = {
        let a2130 = v2;
        let a2131 = f103(v1.clone(), f82(v0, v1.clone(), 0u64), f82(v0, v1.clone(), 1u64), f82(v0, v1.clone(), 2u64), f82(v0, v1.clone(), 3u64), false)?;
        nat_add(a2130, a2131)?
    };
    let a2133 = f103(v1.clone(), f82(v0, v1.clone(), 0u64), f82(v0, v1.clone(), 1u64), f82(v0, v1.clone(), 2u64), f82(v0, v1, 3u64), true)?;
    Ok(nat_sub(a2132, a2133))
}

pub fn f89(v0: u64, v1: u64, v2: u64) -> bool {
    let a2134 = v0;
    let a2135 = 0u64;
    if nat_eq(a2134, a2135) {
        let a2136 = v2;
        let a2137 = v1;
        nat_le(a2136, a2137)
    } else {
        let a2138 = v0;
        let a2139 = 1u64;
        if nat_eq(a2138, a2139) {
            let a2140 = v2;
            let a2141 = v1;
            nat_lt(a2140, a2141)
        } else {
            let a2142 = v0;
            let a2143 = 2u64;
            if nat_eq(a2142, a2143) {
                let a2144 = v1;
                let a2145 = v2;
                nat_le(a2144, a2145)
            } else {
                let a2146 = v0;
                let a2147 = 3u64;
                if nat_eq(a2146, a2147) {
                    let a2148 = v1;
                    let a2149 = v2;
                    nat_lt(a2148, a2149)
                } else {
                    let a2150 = v1;
                    let a2151 = v2;
                    nat_eq(a2150, a2151)
                }
            }
        }
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: u64) -> R<Adt5> {
    let a2152 = v0;
    let a2153 = 0u64;
    if nat_eq(a2152, a2153) {
        Ok(f104(0u64))
    } else {
        let a2156 = {
            let a2154 = v0;
            let a2155 = 1u64;
            nat_add(a2154, a2155)?
        };
        let a2157 = f77(v1.clone(), 13u64, v2)?;
        if nat_eq(a2156, a2157) {
            Ok(Adt5::C4({
                let a2158 = v2;
                let a2159 = 0u64;
                if nat_eq(a2158, a2159) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a2160 = v0;
                let a2161 = 1u64;
                nat_sub(a2160, a2161)
            };
            if f101(v1.clone(), v2) {
                let v4: u64 = f91(match v1.clone() {
                    Adt3::C0(h2162, _, _, _, _, _, _, _) => {
                        h2162
                    }
                }, false)?;
                let v5: u64 = {
                    let a2163 = v3;
                    let a2164 = v4;
                    let a2165 = 0u64;
                    nat_quot(a2163, a2164, a2165)
                };
                let v6: u64 = {
                    let a2166 = v3;
                    let a2167 = v4;
                    let a2168 = 0u64;
                    nat_rem(a2166, a2167, a2168)
                };
                let a2171 = {
                    let a2169 = v6;
                    let a2170 = 1u64;
                    nat_add(a2169, a2170)?
                };
                let a2172 = v4;
                if nat_eq(a2171, a2172) {
                    Ok(Adt5::C4({
                        let a2173 = v2;
                        let a2174 = 0u64;
                        if nat_eq(a2173, a2174) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f92(v6, v5, v1, v2)
                }
            } else {
                f105({
                    let a2175 = v3;
                    let a2176 = 2u64;
                    let a2177 = 0u64;
                    nat_quot(a2175, a2176, a2177)
                }, {
                    let a2178 = v3;
                    let a2179 = 2u64;
                    let a2180 = 0u64;
                    nat_rem(a2178, a2179, a2180)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f91(v0: u64, v1: bool) -> R<u64> {
    let a2183 = {
        let a2181 = v0;
        let a2182 = 2u64;
        nat_mul(a2181, a2182)?
    };
    let a2184 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a2183, a2184)
}

pub fn f92(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt5> {
    let a2185 = v0;
    let a2186 = 0u64;
    if nat_eq(a2185, a2186) {
        Ok(f104(1u64))
    } else {
        f105({
            let a2195 = {
                let a2188 = v1;
                let a2189 = match v2.clone() {
                    Adt3::C0(h2187, _, _, _, _, _, _, _) => {
                        h2187
                    }
                };
                nat_mul(a2188, a2189)?
            };
            let a2196 = {
                let a2192 = {
                    let a2190 = v0;
                    let a2191 = 1u64;
                    nat_sub(a2190, a2191)
                };
                let a2193 = 2u64;
                let a2194 = 0u64;
                nat_quot(a2192, a2193, a2194)
            };
            nat_add(a2195, a2196)?
        }, {
            let a2199 = {
                let a2197 = v0;
                let a2198 = 1u64;
                nat_sub(a2197, a2198)
            };
            let a2200 = 2u64;
            let a2201 = 0u64;
            nat_rem(a2199, a2200, a2201)
        }, v2, v3, 1u64)
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m2202 = f66(v0);
    match m2202 {
        Adt7::C0 => {
            Ok(None::<u64>)
        }
        Adt7::C1(v13) => {
            if if if if if f106(v0) {
                let a2203 = v1;
                let a2204 = v7;
                nat_lt(a2203, a2204)
            } else {
                false
            } {
                f57(v8, v2)
            } else {
                false
            } {
                let a2205 = v3;
                let a2206 = f107(v4, v5, v6)?;
                nat_lt(a2205, a2206)
            } else {
                false
            } {
                let a2211 = {
                    let a2208 = v7;
                    let a2209 = match v13.clone() {
                        Adt8::C0(_, h2207, _) => {
                            h2207
                        }
                    };
                    let a2210 = 0u64;
                    nat_rem(a2208, a2209, a2210)
                };
                let a2212 = 0u64;
                nat_eq(a2211, a2212)
            } else {
                false
            } {
                Ok(Some({
                    let a2241 = {
                        let a2231 = {
                            let a2221 = {
                                let a2217 = {
                                    let a2214 = v1;
                                    let a2215 = match v13 {
                                        Adt8::C0(_, h2213, _) => {
                                            h2213
                                        }
                                    };
                                    let a2216 = 0u64;
                                    nat_quot(a2214, a2215, a2216)
                                };
                                let a2218 = v9;
                                nat_mul(a2217, a2218)?
                            };
                            let a2222 = {
                                let a2219 = v2;
                                let a2220 = v10;
                                nat_mul(a2219, a2220)?
                            };
                            nat_add(a2221, a2222)?
                        };
                        let a2232 = {
                            let a2229 = {
                                let a2226 = {
                                    let a2223 = v3;
                                    let a2224 = v4;
                                    let a2225 = 0u64;
                                    nat_quot(a2223, a2224, a2225)
                                };
                                let a2227 = v5;
                                let a2228 = 0u64;
                                nat_rem(a2226, a2227, a2228)
                            };
                            let a2230 = v11;
                            nat_mul(a2229, a2230)?
                        };
                        nat_add(a2231, a2232)?
                    };
                    let a2242 = {
                        let a2239 = {
                            let a2236 = {
                                let a2233 = v3;
                                let a2234 = v4;
                                let a2235 = 0u64;
                                nat_quot(a2233, a2234, a2235)
                            };
                            let a2237 = v5;
                            let a2238 = 0u64;
                            nat_quot(a2236, a2237, a2238)
                        };
                        let a2240 = v12;
                        nat_mul(a2239, a2240)?
                    };
                    nat_add(a2241, a2242)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f94(v0: u64) -> u64 {
    let a2243 = v0;
    let a2244 = 40u64;
    if nat_lt(a2243, a2244) {
        let a2245 = v0;
        let a2246 = 20u64;
        if nat_lt(a2245, a2246) {
            let a2247 = v0;
            let a2248 = 10u64;
            if nat_lt(a2247, a2248) {
                let a2249 = v0;
                let a2250 = 5u64;
                if nat_lt(a2249, a2250) {
                    let a2251 = v0;
                    let a2252 = 2u64;
                    if nat_lt(a2251, a2252) {
                        let a2253 = v0;
                        let a2254 = 1u64;
                        if nat_lt(a2253, a2254) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a2255 = v0;
                        let a2256 = 3u64;
                        if nat_lt(a2255, a2256) {
                            4u64
                        } else {
                            let a2257 = v0;
                            let a2258 = 4u64;
                            if nat_lt(a2257, a2258) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a2259 = v0;
                    let a2260 = 7u64;
                    if nat_lt(a2259, a2260) {
                        let a2261 = v0;
                        let a2262 = 6u64;
                        if nat_lt(a2261, a2262) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a2263 = v0;
                        let a2264 = 8u64;
                        if nat_lt(a2263, a2264) {
                            128u64
                        } else {
                            let a2265 = v0;
                            let a2266 = 9u64;
                            if nat_lt(a2265, a2266) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a2267 = v0;
                let a2268 = 15u64;
                if nat_lt(a2267, a2268) {
                    let a2269 = v0;
                    let a2270 = 12u64;
                    if nat_lt(a2269, a2270) {
                        let a2271 = v0;
                        let a2272 = 11u64;
                        if nat_lt(a2271, a2272) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a2273 = v0;
                        let a2274 = 13u64;
                        if nat_lt(a2273, a2274) {
                            4096u64
                        } else {
                            let a2275 = v0;
                            let a2276 = 14u64;
                            if nat_lt(a2275, a2276) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a2277 = v0;
                    let a2278 = 17u64;
                    if nat_lt(a2277, a2278) {
                        let a2279 = v0;
                        let a2280 = 16u64;
                        if nat_lt(a2279, a2280) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a2281 = v0;
                        let a2282 = 18u64;
                        if nat_lt(a2281, a2282) {
                            131072u64
                        } else {
                            let a2283 = v0;
                            let a2284 = 19u64;
                            if nat_lt(a2283, a2284) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a2285 = v0;
            let a2286 = 30u64;
            if nat_lt(a2285, a2286) {
                let a2287 = v0;
                let a2288 = 25u64;
                if nat_lt(a2287, a2288) {
                    let a2289 = v0;
                    let a2290 = 22u64;
                    if nat_lt(a2289, a2290) {
                        let a2291 = v0;
                        let a2292 = 21u64;
                        if nat_lt(a2291, a2292) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a2293 = v0;
                        let a2294 = 23u64;
                        if nat_lt(a2293, a2294) {
                            4194304u64
                        } else {
                            let a2295 = v0;
                            let a2296 = 24u64;
                            if nat_lt(a2295, a2296) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a2297 = v0;
                    let a2298 = 27u64;
                    if nat_lt(a2297, a2298) {
                        let a2299 = v0;
                        let a2300 = 26u64;
                        if nat_lt(a2299, a2300) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a2301 = v0;
                        let a2302 = 28u64;
                        if nat_lt(a2301, a2302) {
                            134217728u64
                        } else {
                            let a2303 = v0;
                            let a2304 = 29u64;
                            if nat_lt(a2303, a2304) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a2305 = v0;
                let a2306 = 35u64;
                if nat_lt(a2305, a2306) {
                    let a2307 = v0;
                    let a2308 = 32u64;
                    if nat_lt(a2307, a2308) {
                        let a2309 = v0;
                        let a2310 = 31u64;
                        if nat_lt(a2309, a2310) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a2311 = v0;
                        let a2312 = 33u64;
                        if nat_lt(a2311, a2312) {
                            4294967296u64
                        } else {
                            let a2313 = v0;
                            let a2314 = 34u64;
                            if nat_lt(a2313, a2314) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a2315 = v0;
                    let a2316 = 37u64;
                    if nat_lt(a2315, a2316) {
                        let a2317 = v0;
                        let a2318 = 36u64;
                        if nat_lt(a2317, a2318) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a2319 = v0;
                        let a2320 = 38u64;
                        if nat_lt(a2319, a2320) {
                            137438953472u64
                        } else {
                            let a2321 = v0;
                            let a2322 = 39u64;
                            if nat_lt(a2321, a2322) {
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

pub fn f95(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f108(v0, v1, v6, v7, v8) {
        let a2323 = v5;
        let a2324 = 26u64;
        nat_eq(a2323, a2324)
    } else {
        false
    } {
        let a2325 = v3;
        let a2326 = v7;
        nat_eq(a2325, a2326)
    } else {
        false
    } {
        let a2327 = v4;
        let a2328 = v8;
        nat_eq(a2327, a2328)
    } else {
        false
    } {
        let a2329 = v9;
        let a2330 = 1u64;
        nat_eq(a2329, a2330)
    } else {
        false
    } {
        let a2331 = v10;
        let a2332 = f109(v0);
        nat_eq(a2331, a2332)
    } else {
        false
    } {
        let a2333 = v11;
        let a2334 = v1;
        nat_eq(a2333, a2334)
    } else {
        false
    } {
        let a2335 = v12;
        let a2336 = v6;
        nat_eq(a2335, a2336)
    } else {
        false
    } {
        let a2337 = v13;
        let a2338 = v7;
        nat_eq(a2337, a2338)
    } else {
        false
    } {
        let a2339 = v14;
        let a2340 = v8;
        nat_eq(a2339, a2340)
    } else {
        false
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a2341 = v0;
    let a2342 = 0u64;
    if if if nat_eq(a2341, a2342) {
        true
    } else {
        let a2343 = v1;
        let a2344 = 0u64;
        nat_eq(a2343, a2344)
    } {
        true
    } else {
        let a2345 = v2;
        let a2346 = 0u64;
        nat_eq(a2345, a2346)
    } {
        true
    } else {
        let a2347 = v3;
        let a2348 = 0u64;
        nat_eq(a2347, a2348)
    }
}

pub fn f97(v0: u64) -> u64 {
    let a2349 = v0;
    let a2350 = 2147483648u64;
    if nat_lt(a2349, a2350) {
        v0
    } else {
        0u64
    }
}

pub fn f98(v0: u64) -> u64 {
    let a2351 = v0;
    let a2352 = 2147483648u64;
    if nat_lt(a2351, a2352) {
        0u64
    } else {
        let a2353 = 4294967296u64;
        let a2354 = v0;
        nat_sub(a2353, a2354)
    }
}

pub fn f99(v0: Adt3, v1: bool) -> R<u64> {
    f103(v0.clone(), {
        let a2356 = match v0.clone() {
            Adt3::C0(h2355, _, _, _, _, _, _, _) => {
                h2355
            }
        };
        let a2357 = 1u64;
        nat_sub(a2356, a2357)
    }, {
        let a2359 = match v0.clone() {
            Adt3::C0(_, h2358, _, _, _, _, _, _) => {
                h2358
            }
        };
        let a2360 = 1u64;
        nat_sub(a2359, a2360)
    }, {
        let a2362 = match v0.clone() {
            Adt3::C0(_, _, h2361, _, _, _, _, _) => {
                h2361
            }
        };
        let a2363 = 1u64;
        nat_sub(a2362, a2363)
    }, {
        let a2365 = match v0 {
            Adt3::C0(_, _, _, h2364, _, _, _, _) => {
                h2364
            }
        };
        let a2366 = 1u64;
        nat_sub(a2365, a2366)
    }, v1)
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a2367 = v0;
    let a2368 = 0u64;
    if nat_eq(a2367, a2368) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a2369 = v0;
            let a2370 = 1u64;
            nat_sub(a2369, a2370)
        };
        let v5: u64 = {
            let a2371 = v4;
            let a2372 = v1;
            let a2373 = 0u64;
            nat_quot(a2371, a2372, a2373)
        };
        let v6: u64 = {
            let a2381 = {
                let a2374 = v5;
                let a2375 = v2;
                nat_mul(a2374, a2375)?
            };
            let a2382 = {
                let a2379 = {
                    let a2376 = v4;
                    let a2377 = v1;
                    let a2378 = 0u64;
                    nat_rem(a2376, a2377, a2378)
                };
                let a2380 = v3;
                nat_mul(a2379, a2380)?
            };
            nat_add(a2381, a2382)?
        };
        let a2383 = v5;
        let a2384 = 0u64;
        if nat_eq(a2383, a2384) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a2393 = {
                    let a2387 = {
                        let a2385 = v5;
                        let a2386 = 1u64;
                        nat_sub(a2385, a2386)
                    };
                    let a2388 = v2;
                    nat_mul(a2387, a2388)?
                };
                let a2394 = {
                    let a2391 = {
                        let a2389 = v1;
                        let a2390 = 1u64;
                        nat_sub(a2389, a2390)
                    };
                    let a2392 = v3;
                    nat_mul(a2391, a2392)?
                };
                nat_add(a2393, a2394)?
            };
            let a2395 = v6;
            let a2396 = v7;
            if nat_lt(a2395, a2396) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f101(v0: Adt3, v1: u64) -> bool {
    let a2397 = v1;
    let a2398 = 0u64;
    if !nat_eq(a2397, a2398) {
        true
    } else {
        let a2400 = match v0 {
            Adt3::C0(_, _, _, _, h2399, _, _, _) => {
                h2399
            }
        };
        let a2401 = 4u64;
        nat_eq(a2400, a2401)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt6> {
    let m2402 = f110(v0, v1);
    match m2402 {
        None => {
            if v5 {
                let m2403 = f111(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m2403 {
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
                let a2404 = 4u64;
                let a2405 = v28;
                nat_le(a2404, a2405)
            };
            let a2406 = v2;
            let a2407 = 0u64;
            if if if if if nat_eq(a2406, a2407) {
                let a2408 = v4;
                let a2409 = 0u64;
                nat_eq(a2408, a2409)
            } else {
                false
            } {
                let a2410 = v6;
                let a2411 = v14;
                if nat_eq(a2410, a2411) {
                    let a2412 = v7;
                    let a2413 = v15;
                    nat_eq(a2412, a2413)
                } else {
                    false
                }
            } else {
                false
            } {
                let a2414 = v8;
                let a2415 = v16;
                if nat_eq(a2414, a2415) {
                    let a2416 = v9;
                    let a2417 = v17;
                    nat_eq(a2416, a2417)
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
                        let a2418 = v3;
                        let a2419 = 0u64;
                        nat_eq(a2418, a2419)
                    } else {
                        false
                    } {
                        f67(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt6::C2(v28, v29, v6, v7, v8, v9, {
                    let a2424 = {
                        let a2420 = v6;
                        let a2421 = v7;
                        nat_mul(a2420, a2421)?
                    };
                    let a2425 = {
                        let a2422 = v8;
                        let a2423 = v9;
                        nat_mul(a2422, a2423)?
                    };
                    nat_mul(a2424, a2425)?
                }))
            } else {
                Ok(Adt6::C1)
            }
        }
    }
}

pub fn f103(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a2446 = {
        let a2440 = {
            let a2434 = {
                let a2428 = v1;
                let a2429 = if v5 {
                    f98(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2426, _, _, _) => {
                            h2426
                        }
                    })
                } else {
                    f97(match v0.clone() {
                        Adt3::C0(_, _, _, _, h2427, _, _, _) => {
                            h2427
                        }
                    })
                };
                nat_mul(a2428, a2429)?
            };
            let a2435 = {
                let a2432 = v2;
                let a2433 = if v5 {
                    f98(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2430, _, _) => {
                            h2430
                        }
                    })
                } else {
                    f97(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h2431, _, _) => {
                            h2431
                        }
                    })
                };
                nat_mul(a2432, a2433)?
            };
            nat_add(a2434, a2435)?
        };
        let a2441 = {
            let a2438 = v3;
            let a2439 = if v5 {
                f98(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2436, _) => {
                        h2436
                    }
                })
            } else {
                f97(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h2437, _) => {
                        h2437
                    }
                })
            };
            nat_mul(a2438, a2439)?
        };
        nat_add(a2440, a2441)?
    };
    let a2447 = {
        let a2444 = v4;
        let a2445 = if v5 {
            f98(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2442) => {
                    h2442
                }
            })
        } else {
            f97(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h2443) => {
                    h2443
                }
            })
        };
        nat_mul(a2444, a2445)?
    };
    nat_add(a2446, a2447)
}

pub fn f104(v0: u64) -> Adt5 {
    Adt5::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f105(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt5> {
    let a2448 = v1;
    let a2449 = 0u64;
    if nat_eq(a2448, a2449) {
        Ok(Adt5::C4({
            let a2450 = v3;
            let a2451 = 0u64;
            if nat_eq(a2450, a2451) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f39(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt5::C4({
            let a2452 = v3;
            let a2453 = 0u64;
            if nat_eq(a2452, a2453) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f106(v0: u64) -> bool {
    let a2454 = v0;
    let a2455 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a2454, a2455) {
        true
    } else {
        let a2456 = v0;
        let a2457 = 1u64;
        nat_eq(a2456, a2457)
    } {
        true
    } else {
        let a2458 = v0;
        let a2459 = 2u64;
        nat_eq(a2458, a2459)
    } {
        true
    } else {
        let a2460 = v0;
        let a2461 = 3u64;
        nat_eq(a2460, a2461)
    } {
        true
    } else {
        let a2462 = v0;
        let a2463 = 6u64;
        nat_eq(a2462, a2463)
    } {
        true
    } else {
        let a2464 = v0;
        let a2465 = 7u64;
        nat_eq(a2464, a2465)
    } {
        true
    } else {
        let a2466 = v0;
        let a2467 = 8u64;
        nat_eq(a2466, a2467)
    } {
        true
    } else {
        let a2468 = v0;
        let a2469 = 10u64;
        nat_eq(a2468, a2469)
    } {
        true
    } else {
        let a2470 = v0;
        let a2471 = 11u64;
        nat_eq(a2470, a2471)
    } {
        true
    } else {
        let a2472 = v0;
        let a2473 = 12u64;
        nat_eq(a2472, a2473)
    } {
        true
    } else {
        let a2474 = v0;
        let a2475 = 13u64;
        nat_eq(a2474, a2475)
    } {
        true
    } else {
        let a2476 = v0;
        let a2477 = 14u64;
        nat_eq(a2476, a2477)
    } {
        true
    } else {
        let a2478 = v0;
        let a2479 = 16u64;
        nat_eq(a2478, a2479)
    } {
        true
    } else {
        let a2480 = v0;
        let a2481 = 17u64;
        nat_eq(a2480, a2481)
    } {
        true
    } else {
        let a2482 = v0;
        let a2483 = 18u64;
        nat_eq(a2482, a2483)
    } {
        true
    } else {
        let a2484 = v0;
        let a2485 = 19u64;
        nat_eq(a2484, a2485)
    } {
        true
    } else {
        let a2486 = v0;
        let a2487 = 20u64;
        nat_eq(a2486, a2487)
    } {
        true
    } else {
        let a2488 = v0;
        let a2489 = 21u64;
        nat_eq(a2488, a2489)
    } {
        true
    } else {
        let a2490 = v0;
        let a2491 = 22u64;
        nat_eq(a2490, a2491)
    } {
        true
    } else {
        let a2492 = v0;
        let a2493 = 23u64;
        nat_eq(a2492, a2493)
    } {
        true
    } else {
        let a2494 = v0;
        let a2495 = 26u64;
        nat_eq(a2494, a2495)
    } {
        true
    } else {
        let a2496 = v0;
        let a2497 = 29u64;
        nat_eq(a2496, a2497)
    } {
        true
    } else {
        let a2498 = v0;
        let a2499 = 30u64;
        nat_eq(a2498, a2499)
    } {
        true
    } else {
        let a2500 = v0;
        let a2501 = 34u64;
        nat_eq(a2500, a2501)
    } {
        true
    } else {
        let a2502 = v0;
        let a2503 = 35u64;
        nat_eq(a2502, a2503)
    } {
        true
    } else {
        let a2504 = v0;
        let a2505 = 39u64;
        nat_eq(a2504, a2505)
    } {
        true
    } else {
        let a2506 = v0;
        let a2507 = 40u64;
        nat_eq(a2506, a2507)
    } {
        true
    } else {
        let a2508 = v0;
        let a2509 = 41u64;
        nat_eq(a2508, a2509)
    } {
        true
    } else {
        let a2510 = v0;
        let a2511 = 42u64;
        nat_eq(a2510, a2511)
    } {
        true
    } else {
        let a2512 = v0;
        let a2513 = 43u64;
        nat_eq(a2512, a2513)
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2514 = v0;
    let a2515 = 0u64;
    if if if nat_eq(a2514, a2515) {
        true
    } else {
        let a2516 = v1;
        let a2517 = 0u64;
        nat_eq(a2516, a2517)
    } {
        true
    } else {
        let a2518 = v2;
        let a2519 = 0u64;
        nat_eq(a2518, a2519)
    } {
        Ok(0u64)
    } else {
        let a2522 = {
            let a2520 = v0;
            let a2521 = v1;
            nat_mul(a2520, a2521)?
        };
        let a2523 = v2;
        nat_mul(a2522, a2523)
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f96(v1, v2, v3, v4) {
        f112(v0)
    } else {
        f106(v0)
    }
}

pub fn f109(v0: u64) -> u64 {
    let a2524 = v0;
    let a2525 = 26u64;
    if nat_eq(a2524, a2525) {
        26u64
    } else {
        0u64
    }
}

pub fn f110(v0: u64, v1: u64) -> Option<u64> {
    let a2526 = v0;
    let a2527 = 2u64;
    if nat_eq(a2526, a2527) {
        Some(0u64)
    } else {
        let a2528 = v0;
        let a2529 = 5u64;
        if nat_eq(a2528, a2529) {
            Some(1u64)
        } else {
            let a2530 = v0;
            let a2531 = 6u64;
            if nat_eq(a2530, a2531) {
                Some(2u64)
            } else {
                let a2532 = v0;
                let a2533 = 7u64;
                if nat_eq(a2532, a2533) {
                    Some(3u64)
                } else {
                    let a2534 = v0;
                    let a2535 = 9u64;
                    if nat_eq(a2534, a2535) {
                        Some(4u64)
                    } else {
                        let a2536 = v0;
                        let a2537 = 8u64;
                        if nat_eq(a2536, a2537) {
                            Some(5u64)
                        } else {
                            let a2538 = v0;
                            let a2539 = 89u64;
                            if nat_eq(a2538, a2539) {
                                let a2540 = v1;
                                let a2541 = 2u64;
                                if nat_eq(a2540, a2541) {
                                    Some(6u64)
                                } else {
                                    let a2542 = v1;
                                    let a2543 = 0u64;
                                    if nat_eq(a2542, a2543) {
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

pub fn f111(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt9> {
    let a2544 = v0;
    let a2545 = 28u64;
    if if if if nat_eq(a2544, a2545) {
        let a2546 = v1;
        let a2547 = 0u64;
        nat_eq(a2546, a2547)
    } else {
        false
    } {
        let a2548 = v2;
        let a2549 = 0u64;
        nat_eq(a2548, a2549)
    } else {
        false
    } {
        let a2550 = v3;
        let a2551 = 0u64;
        nat_eq(a2550, a2551)
    } else {
        false
    } {
        let a2552 = v4;
        let a2553 = v8;
        if if if if if if if if nat_eq(a2552, a2553) {
            let a2554 = v5;
            let a2555 = v12;
            nat_eq(a2554, a2555)
        } else {
            false
        } {
            let a2556 = v9;
            let a2557 = v13;
            nat_eq(a2556, a2557)
        } else {
            false
        } {
            let a2558 = v10;
            let a2559 = v14;
            nat_eq(a2558, a2559)
        } else {
            false
        } {
            let a2560 = v11;
            let a2561 = v15;
            nat_eq(a2560, a2561)
        } else {
            false
        } {
            let a2562 = 0u64;
            let a2563 = v6;
            nat_lt(a2562, a2563)
        } else {
            false
        } {
            let a2564 = 0u64;
            let a2565 = v7;
            nat_lt(a2564, a2565)
        } else {
            false
        } {
            let a2569 = {
                let a2566 = v10;
                let a2567 = v6;
                let a2568 = 1u64;
                nat_rem(a2566, a2567, a2568)
            };
            let a2570 = 0u64;
            if nat_eq(a2569, a2570) {
                let a2574 = {
                    let a2571 = v11;
                    let a2572 = v7;
                    let a2573 = 1u64;
                    nat_rem(a2571, a2572, a2573)
                };
                let a2575 = 0u64;
                nat_eq(a2574, a2575)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt9::C2(v5, v4, v9, v10, v11, {
                let a2576 = v10;
                let a2577 = v6;
                let a2578 = 0u64;
                nat_quot(a2576, a2577, a2578)
            }, {
                let a2579 = v11;
                let a2580 = v7;
                let a2581 = 0u64;
                nat_quot(a2579, a2580, a2581)
            }, {
                let a2582 = v5;
                let a2583 = v4;
                nat_mul(a2582, a2583)?
            }, {
                let a2584 = v4;
                let a2585 = v9;
                nat_mul(a2584, a2585)?
            }, {
                let a2586 = v5;
                let a2587 = v9;
                nat_mul(a2586, a2587)?
            }))
        } else {
            Ok(Adt9::C1)
        }
    } else {
        Ok(Adt9::C0)
    }
}

pub fn f112(v0: u64) -> bool {
    let m2588 = f66(v0);
    match m2588 {
        Adt7::C0 => {
            false
        }
        Adt7::C1(_) => {
            true
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt5> {
    f0(p0)
}
