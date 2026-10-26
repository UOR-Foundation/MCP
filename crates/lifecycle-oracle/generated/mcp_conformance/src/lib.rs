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
pub struct Str(std::rc::Rc<str>);
impl Str {
    pub fn lit(text: &str) -> Str { Str(std::rc::Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
}
#[derive(Clone)]
pub struct Bytes(std::rc::Rc<[u8]>);
impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(std::rc::Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
}

pub struct Node<T> { head: T, tail: List<T> }
/// A list is a run of cells laid out contiguously, from the newest at the end of a buffer
/// to the oldest at its start, or a cell in front of another list. A `cons` onto a run that
/// owns its buffer alone pushes in place; a `cons` onto any other list is one cell, so
/// building and taking apart a cell is constant work whatever the sharing, and no buffer is
/// written while another list can see it.
pub enum Repr<T> { Nil, Run { buffer: std::rc::Rc<Vec<T>>, len: usize }, Cell(std::rc::Rc<Node<T>>) }
pub struct List<T>(Repr<T>);
impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(match &self.0 {
            Repr::Nil => Repr::Nil,
            Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len },
            Repr::Cell(cell) => Repr::Cell(cell.clone()),
        })
    }
}
impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { std::mem::replace(&mut self.0, Repr::Nil) }
}
impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None };
        while let Some(cell) = next { // release
            match std::rc::Rc::try_unwrap(cell) {
                Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None },
                Err(_) => break,
            }
        }
    }
}
impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self {
        List(match tail.take() {
            Repr::Nil => {
                let mut buffer = Vec::with_capacity(16);
                buffer.push(head);
                Repr::Run { buffer: std::rc::Rc::new(buffer), len: 1 }
            }
            Repr::Run { mut buffer, len } => {
                if let Some(cells) = std::rc::Rc::get_mut(&mut buffer) {
                    cells.truncate(len);
                    cells.push(head);
                    Repr::Run { buffer, len: len + 1 }
                } else {
                    Repr::Cell(std::rc::Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) }))
                }
            }
            rest => Repr::Cell(std::rc::Rc::new(Node { head, tail: List(rest) })),
        })
    }
    pub fn uncons(&self) -> Option<(T, List<T>)> {
        match &self.0 {
            Repr::Nil => None,
            Repr::Run { buffer, len } => {
                let head = buffer[len - 1].clone();
                Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } })))
            }
            Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())),
        }
    }
    /// Walk the items in order, ticking one per item, until `each` says to stop: the cells
    /// in front, then the run they end in.
    fn walk(&self, each: &mut dyn FnMut(&T) -> bool) {
        let mut cursor = self;
        while let Repr::Cell(cell) = &cursor.0 { tick(1); if !each(&cell.head) { return; } cursor = &cell.tail; }
        if let Repr::Run { buffer, len } = &cursor.0 {
            for item in buffer[..*len].iter().rev() { tick(1); if !each(item) { return; } }
        }
    }
    fn items(&self) -> Vec<T> {
        let mut out = Vec::new();
        self.walk(&mut |item| { out.push(item.clone()); true });
        out
    }
    fn onto(items: Vec<T>, tail: List<T>) -> List<T> {
        let mut out = tail;
        for item in items.into_iter().rev() { tick(1); out = List::cons(item, out); }
        out
    }
}

fn chars(text: &str) -> u64 {
    let mut count = 0;
    for _ in text.chars() { tick(1); count += 1; }
    count
}

impl Same for Str {
    fn same(&self, other: &Self) -> bool {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => {} }
        }
    }
}
impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}
impl Key for Str {
    fn key(&self, other: &Self) -> Ordering {
        let mut left = self.0.chars();
        let mut right = other.0.chars();
        loop {
            tick(1);
            match (left.next(), right.next()) {
                (None, None) => return Ordering::Equal,
                (None, Some(_)) => return Ordering::Less,
                (Some(_), None) => return Ordering::Greater,
                (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => {} decided => return decided },
            }
        }
    }
}

pub fn append_list<T: Clone>(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
pub fn append_bytes(a: Bytes, b: Bytes) -> Bytes {
    tick((a.0.len() + b.0.len()) as u64);
    let mut out = Vec::with_capacity(a.0.len() + b.0.len());
    out.extend_from_slice(&a.0);
    out.extend_from_slice(&b.0);
    Bytes(std::rc::Rc::from(out))
}
pub fn length_list<T: Clone>(a: List<T>) -> u64 {
    let mut count = 0;
    a.walk(&mut |_| { count += 1; true });
    count
}
pub fn length_bytes(a: Bytes) -> u64 { a.0.len() as u64 }
pub fn length_string(a: Str) -> u64 { chars(&a.0) }
pub fn index_list<T: Clone>(a: List<T>, i: u64) -> Option<T> {
    let mut position = 0;
    let mut found = None;
    a.walk(&mut |item| { if position == i { found = Some(item.clone()); return false; } position += 1; true });
    found
}
pub fn index_bytes(a: Bytes, i: u64) -> Option<u8> { usize::try_from(i).ok().and_then(|i| a.0.get(i)).copied() }
pub fn slice_list<T: Clone>(a: List<T>, start: u64, count: u64) -> Option<List<T>> {
    let end = u128::from(start) + u128::from(count);
    let mut part = Vec::new();
    let mut position: u128 = 0;
    a.walk(&mut |item| {
        if position >= end { return false; }
        if position >= u128::from(start) { part.push(item.clone()); }
        position += 1;
        true
    });
    if position < end { None } else { Some(List::onto(part, List::nil())) }
}
pub fn slice_bytes(a: Bytes, start: u64, count: u64) -> Option<Bytes> {
    let end = u128::from(start) + u128::from(count);
    if end > a.0.len() as u128 { return None; }
    tick(count);
    Some(Bytes::lit(&a.0[start as usize..end as usize]))
}
pub fn utf8_encode(a: Str) -> Bytes { tick(a.0.len() as u64); Bytes::lit(a.0.as_bytes()) }
pub fn utf8_decode(a: Bytes) -> Option<Str> { tick(a.0.len() as u64); core::str::from_utf8(&a.0).ok().map(Str::lit) }
pub fn compare_bytes(a: Bytes, b: Bytes) -> Ordering { tick(1 + a.0.len().min(b.0.len()) as u64); a.0.cmp(&b.0) }
pub fn split_exact(text: Str, delimiter: Str, maximum: u32) -> Option<List<Str>> {
    if delimiter.0.is_empty() { return None; }
    chars(&text.0);
    let fields: Vec<Str> = text.0.split(&*delimiter.0).map(Str::lit).collect();
    if fields.len() as u128 > u128::from(maximum) { return None; }
    Some(List::onto(fields, List::nil()))
}
pub fn join(texts: List<Str>, delimiter: Str) -> Str {
    let mut out = String::new();
    let mut first = true;
    texts.walk(&mut |text| {
        if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }
        first = false;
        chars(&text.0);
        out.push_str(&text.0);
        true
    });
    Str::lit(&out)
}
fn canonical_decimal(text: &str) -> Option<Option<i128>> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0";
    for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }
    if !canonical { return None; }
    Some(text.parse::<i128>().ok())
}
pub fn parse_int(text: Str) -> R<Option<i64>> {
    match canonical_decimal(&text.0) {
        None => Ok(None),
        Some(Some(n)) => i64::try_from(n).map(Some).map_err(|_| Overflow),
        Some(None) => Err(Overflow),
    }
}
macro_rules! decimal {
    ($t:ident, $format:ident, $parse:ident) => {
        pub fn $format(a: $t) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
        pub fn $parse(text: Str) -> Option<$t> {
            match canonical_decimal(&text.0) { Some(Some(n)) => $t::try_from(n).ok(), _ => None }
        }
    };
}
decimal!(u8, format_u8, parse_u8); decimal!(u16, format_u16, parse_u16);
decimal!(u32, format_u32, parse_u32); decimal!(u64, format_u64, parse_u64);
decimal!(i8, format_i8, parse_i8); decimal!(i16, format_i16, parse_i16);
decimal!(i32, format_i32, parse_i32); decimal!(i64, format_i64, parse_i64);

#[derive(Clone)]
pub enum Adt0 {
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>, u64, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(Str, Option<Str>, Str, Option<Str>, Option<Str>, Option<List<Adt2>>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Str, Option<Str>, Option<List<Str>>, Option<Adt3>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt8>, Option<Adt9>, Option<Adt8>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1(bool),
    C2(Adt7),
    C3(Str),
    C4(List<Adt6>),
    C5(List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0(i64, i64),
}

#[derive(Clone)]
pub enum Adt8 {
    C0(Option<bool>),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(u64, Adt11),
}

#[derive(Clone)]
pub enum Adt11 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Str, Str, List<Str>, Adt13),
}

#[derive(Clone)]
pub enum Adt13 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Adt15>, Adt16, Adt17),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(i64, Str, Str, Bytes),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Adt18),
    C1(Adt23),
    C2(Adt24),
    C3(Adt23),
    C4(Adt25),
    C5(Adt23),
    C6(Adt23),
    C7(Adt26),
    C8(Adt27),
    C9(Adt32),
    C10(Adt33),
    C11(Adt34),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Adt19),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Str, Adt20, Option<Adt5>, Option<Adt21>, Option<Adt22>, Adt5),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt19, Option<Str>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt19, Str, Option<Adt5>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt19, Str, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt19, Str, Option<List<(Str, Str)>>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Adt19, Adt28, Adt30, Option<Adt31>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Adt29),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt19, Str),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt19, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt19, Str),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt36),
    C1(Adt37),
    C2(Adt40),
    C3(Adt52),
    C4(Adt54),
    C5(Adt55),
    C6(Adt56),
    C7(Adt58),
    C8(Adt61),
    C9(Adt63),
    C10(Adt65),
    C11,
    C12(Adt84),
    C13(Adt85),
    C14(Adt90),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Adt10, List<Str>, Adt4, Option<Str>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Adt10, Option<Str>, List<Adt38>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Adt5, Option<Adt5>, Option<Adt39>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(List<Adt41>, Option<Adt6>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(Adt42),
    C1(Adt45),
    C2(Adt46),
    C3(Adt47),
    C4(Adt48),
}

#[derive(Clone)]
pub enum Adt42 {
    C0(Str, Option<Adt43>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(Option<List<Adt44>>, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt45 {
    C0(Str, Str, Option<Adt43>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(Str, Str, Option<Adt43>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt43>, Option<u64>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Adt49, Option<Adt43>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Adt50),
    C1(Adt51),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Adt40, List<Adt53>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt7, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Adt10, Option<Str>, List<Adt47>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Adt10, List<Adt49>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Adt10, Option<Str>, List<Adt57>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt43>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Adt10, Option<Str>, List<Adt59>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Option<List<Adt60>>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Option<Str>, List<Adt62>),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Adt44, Adt41),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Adt64),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(Adt66),
    C1(Adt66, List<(Str, Adt67)>),
    C2(Adt66, Adt5),
    C3(Adt66, Adt83),
    C4(Adt66),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Str, Option<Str>, Str, Str, Option<i64>, Option<i64>),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(Adt68),
    C1(Adt72),
    C2(Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Adt69),
    C1(Adt71),
}

#[derive(Clone)]
pub enum Adt69 {
    C0(Option<Adt70>, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt70 {
    C0,
}

#[derive(Clone)]
pub enum Adt71 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt72 {
    C0(List<Adt73>, Option<Adt78>, Option<Str>, Option<Adt80>, Option<Adt7>, u64, Option<List<Str>>, Option<Adt5>, Option<List<Adt38>>, Option<Adt81>),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Adt44, Adt74, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Adt75),
    C1(List<Adt75>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Adt42),
    C1(Adt45),
    C2(Adt46),
    C3(Adt76),
    C4(Adt77),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Str, Str, Adt5, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0(Str, List<Adt41>, Option<Adt6>, Option<bool>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt78 {
    C0(Option<List<Adt79>>, Option<Adt7>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt80 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt81 {
    C0(Option<Adt82>),
}

#[derive(Clone)]
pub enum Adt82 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt83 {
    C0(i64, Str, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt84 {
    C0(List<(Str, Adt67)>),
    C1(Str),
    C2(List<(Str, Adt67)>, Str),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(Adt86, Adt66, Adt87),
}

#[derive(Clone)]
pub enum Adt86 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt87 {
    C0(Adt65, i64, Adt88),
}

#[derive(Clone)]
pub enum Adt88 {
    C0(i64, List<(Str, Adt67)>, i64, Adt89),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(Adt5),
    C1(Adt83),
}

#[derive(Clone)]
pub enum Adt90 {
    C0(Adt87, Adt91, bool),
}

#[derive(Clone)]
pub enum Adt91 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt92 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt20),
    C9(Str),
    C10,
    C11,
    C12(Str),
    C13,
    C14,
    C15(Adt93),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(List<u64>),
    C1(Adt93),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(Bytes, List<Adt15>),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(u64),
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt97 {
    C0(List<u64>, List<u64>, List<Adt98>, Bytes, u64, List<List<u64>>, Bytes, Adt16),
}

#[derive(Clone)]
pub enum Adt98 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt100 {
    C0(u64, List<Adt98>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt101 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str, Str),
    C5(Str, Str),
    C6(Str, Str),
    C7(Str),
    C8(Str),
    C9(Str),
    C10(Str),
    C11(Str),
    C12(Str, Str),
}

#[derive(Clone)]
pub enum Adt102 {
    C0(Adt52),
    C1(Adt103, Adt52, Adt88),
    C2(Adt84),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt52),
    C1(Adt84),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt107 {
    C0(Adt87, bool),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1,
    C2(Adt5),
}

#[derive(Clone)]
pub enum Adt111 {
    C0(u64),
    C1(u64),
    C2(u64),
    C3,
    C4(u64),
    C5(u64),
    C6(u64),
    C7(u64),
    C8(u64),
    C9(u64),
    C10(u64),
    C11,
    C12,
    C13,
    C14,
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1(Option<Adt108>, Adt113),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1(List<Adt118>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1(List<Adt118>, Str),
}

#[derive(Clone)]
pub enum Adt120 {
    C0(Adt6, Adt6, List<Adt118>),
    C1(List<Adt6>, Adt6, List<Adt118>),
    C2(List<Adt6>, Adt6, List<Adt118>, u64),
    C3(List<Adt118>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt118>),
    C5(Adt121, Adt6, Adt6, List<Adt118>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Adt108, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt119, List<Adt120>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt129>, List<Adt131>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt130),
    C7(Adt130),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Str, List<Str>, Adt13, Adt132),
}

#[derive(Clone)]
pub enum Adt132 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt133 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt135 {
    C0(Adt136, List<Adt140>, u64),
}

#[derive(Clone)]
pub enum Adt136 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt137, List<u64>, bool),
    C7(u64, u64),
    C8(Adt138),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt137 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(bool, Adt139, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt139 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

#[derive(Clone)]
pub enum Adt140 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt120>),
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt108, Result<Adt144, Adt147>),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt35, Adt145, List<Adt146>),
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt146 {
    C0(Adt22, Adt53),
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt148),
    C1(Str, List<Str>),
    C2(Adt129),
    C3(Str),
    C4(Adt92),
}

#[derive(Clone)]
pub enum Adt148 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7(Str),
    C8(Str),
}

#[derive(Clone)]
pub enum Adt149 {
    C0(Option<Str>, Option<Adt129>),
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1(Adt108),
    C2,
}

#[derive(Clone)]
pub enum Adt151 {
    C0(Adt111),
    C1(Adt135),
}

#[derive(Clone)]
pub enum Adt152 {
    C0(Adt111),
    C1(List<Adt127>, List<u64>),
}

#[derive(Clone)]
pub enum Adt153 {
    C0(Adt108, Adt154, Adt19, Adt5),
}

#[derive(Clone)]
pub enum Adt154 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
}

#[derive(Clone)]
pub enum Adt155 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt156 {
    C0(Adt101),
    C1(Adt123),
}

#[derive(Clone)]
pub enum Adt157 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt158 {
    C0,
    C1(Adt154),
}

#[derive(Clone)]
pub enum Adt159 {
    C0(Adt160),
    C1(Adt147),
}

#[derive(Clone)]
pub enum Adt160 {
    C0(Adt108, Str, Adt19, Adt5),
}

#[derive(Clone)]
pub enum Adt161 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt162 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt163 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt164 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt165 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt166 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt167 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt168 {
    C0,
    C1(Adt21),
}

#[derive(Clone)]
pub enum Adt169 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt170 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt171 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt170, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> R<Result<Adt35, Adt92>> {
        match self {
            Fn0::F3 => f3(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F29(Fn0, Adt95),
}

impl Fn1 {
    pub fn apply(&self, p0: Adt16, p1: Adt17) -> R<Result<Adt35, Adt92>> {
        match self {
            Fn1::F29(k0, k1) => f29(k0.clone(), k1.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F40,
}

impl Fn2 {
    pub fn apply(&self, p0: Adt24) -> R<Option<Adt102>> {
        match self {
            Fn2::F40 => f40(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F87,
}

impl Fn3 {
    pub fn apply(&self, p0: Adt24) -> R<Result<Adt105, Adt92>> {
        match self {
            Fn3::F87 => f87(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F89,
}

impl Fn4 {
    pub fn apply(&self, p0: Adt24) -> R<Result<Adt52, Adt92>> {
        match self {
            Fn4::F89 => f89(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F140(Fn3, Adt38),
    F258(Fn4, Adt38),
}

impl Fn5 {
    pub fn apply(&self, p0: Adt24) -> R<Result<Adt35, Adt92>> {
        match self {
            Fn5::F140(k0, k1) => f140(k0.clone(), k1.clone(), p0),
            Fn5::F258(k0, k1) => f258(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F171,
}

impl Fn6 {
    pub fn apply(&self, p0: Str) -> R<Option<List<Adt49>>> {
        match self {
            Fn6::F171 => f171(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F183,
}

impl Fn7 {
    pub fn apply(&self, p0: Str, p1: List<(Str, Str)>) -> Adt61 {
        match self {
            Fn7::F183 => f183(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F185,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt27) -> R<Result<List<Str>, Adt92>> {
        match self {
            Fn8::F185 => f185(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F206(Fn1, Adt97),
}

impl Fn9 {
    pub fn apply(&self, p0: Adt17) -> R<Result<Adt35, Adt92>> {
        match self {
            Fn9::F206(k0, k1) => f206(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F1063(Bytes),
    F1088(Bytes, Bytes),
}

impl Fn10 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn10::F1063(k0) => f1063(k0.clone(), p0),
            Fn10::F1088(k0, k1) => f1088(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F1068(Bytes),
}

impl Fn11 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn11::F1068(k0) => f1068(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F1073(Bytes),
}

impl Fn12 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn12::F1073(k0) => f1073(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F1096(Bytes, u64, u64, u64),
    F1103(Bytes, u64, u64),
}

impl Fn13 {
    pub fn apply(&self, p0: (u64, Adt169)) -> R<(u64, Adt169)> {
        match self {
            Fn13::F1096(k0, k1, k2, k3) => f1096(k0.clone(), *k1, *k2, *k3, p0),
            Fn13::F1103(k0, k1, k2) => f1103(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F1098,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt171) -> R<Adt171> {
        match self {
            Fn14::F1098 => f1098(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F1128,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F1128 => f1128(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    f1(f2()?, Fn0::F3, v0)
}

pub fn f1(v0: Adt0, v1: Fn0, v2: Bytes) -> R<List<u64>> {
    f4(f5(v0, v1, v2)?)
}

pub fn f2() -> R<Adt0> {
    Ok(Adt0::C0({
        let m2 = f6()?;
        match m2 {
            Adt0::C0(h1, _, _, _, _, _, _, _) => {
                h1
            }
        }
    }, {
        let m4 = f6()?;
        match m4 {
            Adt0::C0(_, h3, _, _, _, _, _, _) => {
                h3
            }
        }
    }, {
        let m6 = f6()?;
        match m6 {
            Adt0::C0(_, _, h5, _, _, _, _, _) => {
                h5
            }
        }
    }, {
        let m8 = f6()?;
        match m8 {
            Adt0::C0(_, _, _, h7, _, _, _, _) => {
                h7
            }
        }
    }, {
        let m10 = f6()?;
        match m10 {
            Adt0::C0(_, _, _, _, h9, _, _, _) => {
                h9
            }
        }
    }, {
        let m12 = f6()?;
        match m12 {
            Adt0::C0(_, _, _, _, _, h11, _, _) => {
                h11
            }
        }
    }, {
        let m14 = f6()?;
        match m14 {
            Adt0::C0(_, _, _, _, _, _, h13, _) => {
                h13
            }
        }
    }, f7()))
}

pub fn f3(v0: Adt14) -> R<Result<Adt35, Adt92>> {
    f8(match v0.clone() {
        Adt14::C0(_, h15, _) => {
            h15
        }
    }, match v0 {
        Adt14::C0(_, _, h16) => {
            h16
        }
    })
}

pub fn f4(v0: Adt94) -> R<List<u64>> {
    let m17 = v0;
    match m17 {
        Adt94::C0(v1) => {
            let a18 = f9(0u64);
            let a19 = v1;
            Ok(append_list(a18, a19))
        }
        Adt94::C1(v2) => {
            let a24 = f9(1u64);
            let a25 = {
                let a22 = f9(match v2.clone() {
                    Adt93::C0(h20, _) => {
                        h20
                    }
                });
                let a23 = f10(match v2 {
                    Adt93::C0(_, h21) => {
                        h21
                    }
                })?;
                append_list(a22, a23)
            };
            Ok(append_list(a24, a25))
        }
    }
}

pub fn f5(v0: Adt0, v1: Fn0, v2: Bytes) -> R<Adt94> {
    let m26 = f11(v2)?;
    match m26 {
        Ok(v4) => {
            f13(v0, v1, v4.clone(), f14(match v4 {
                Adt95::C0(h27, _) => {
                    h27
                }
            })?)
        }
        Err(_) => {
            Ok(f12())
        }
    }
}

pub fn f6() -> R<Adt0> {
    Ok(Adt0::C0(f15(), None::<Str>, f16(), f17(), f18(f19()?)?, f20(), Bytes::lit(&[]), 0u64))
}

pub fn f7() -> u64 {
    4u64
}

pub fn f8(v0: Adt16, v1: Adt17) -> R<Result<Adt35, Adt92>> {
    let m28 = f21(f19()?)?;
    match m28 {
        None => {
            f22(v0, v1)
        }
        Some(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f9(v0: u64) -> List<u64> {
    List::cons({
        let a32 = {
            let a29 = v0;
            let a30 = 16777216u64;
            let a31 = 0u64;
            nat_quot(a29, a30, a31)
        };
        let a33 = 256u64;
        let a34 = 0u64;
        nat_rem(a32, a33, a34)
    }, List::cons({
        let a38 = {
            let a35 = v0;
            let a36 = 65536u64;
            let a37 = 0u64;
            nat_quot(a35, a36, a37)
        };
        let a39 = 256u64;
        let a40 = 0u64;
        nat_rem(a38, a39, a40)
    }, List::cons({
        let a44 = {
            let a41 = v0;
            let a42 = 256u64;
            let a43 = 0u64;
            nat_quot(a41, a42, a43)
        };
        let a45 = 256u64;
        let a46 = 0u64;
        nat_rem(a44, a45, a46)
    }, List::cons({
        let a47 = v0;
        let a48 = 256u64;
        let a49 = 0u64;
        nat_rem(a47, a48, a49)
    }, List::<u64>::nil()))))
}

pub fn f10(v0: Bytes) -> R<List<u64>> {
    f23(v0.clone(), {
        let a50 = v0;
        length_bytes(a50)
    }, List::<u64>::nil())
}

pub fn f11(v0: Bytes) -> R<Result<Adt95, Adt96>> {
    f24(v0.clone(), f25(v0, 0u64)?)
}

pub fn f12() -> Adt94 {
    Adt94::C0(f26(f27()))
}

pub fn f13(v0: Adt0, v1: Fn0, v2: Adt95, v3: Result<Adt97, Adt96>) -> R<Adt94> {
    let m51 = v3;
    match m51 {
        Ok(v5) => {
            f28(v0.clone(), Fn1::F29(v1, v2.clone()), f30(match v2 {
                Adt95::C0(_, h52) => {
                    h52
                }
            }, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, h53) => {
                    h53
                }
            }), v5)
        }
        Err(_) => {
            Ok(f12())
        }
    }
}

pub fn f14(v0: Bytes) -> R<Result<Adt97, Adt96>> {
    let m54 = f25(v0.clone(), 0u64)?;
    match m54 {
        None => {
            Ok(Err::<Adt97, Adt96>(Adt96::C0(0u64)))
        }
        Some(v1) => {
            f31(v0.clone(), v1.clone(), f25(v0, match v1 {
                Adt99::C0(_, h55) => {
                    h55
                }
            })?)
        }
    }
}

pub fn f15() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f16() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, Some(f32()), Some(Adt8::C0(None::<bool>)), Some(Adt9::C0(None::<bool>, None::<bool>)), Some(Adt8::C0(None::<bool>)), Some(Adt5::C0(List::cons((f33(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f17() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f18(v0: List<Adt38>) -> R<List<Adt12>> {
    let m56 = f34(v0)?;
    match m56 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f19() -> R<List<Adt38>> {
    let a59 = {
        let a57 = f35()?;
        let a58 = f36();
        append_list(a57, a58)
    };
    let a60 = f37();
    Ok(append_list(a59, a60))
}

pub fn f20() -> u64 {
    100u64
}

pub fn f21(v0: List<Adt38>) -> R<Option<Adt92>> {
    let m61 = f34(v0)?;
    match m61 {
        Ok(_) => {
            Ok(None::<Adt92>)
        }
        Err(v1) => {
            Ok(Some(Adt92::C9(f38(v1))))
        }
    }
}

pub fn f22(v0: Adt16, v1: Adt17) -> R<Result<Adt35, Adt92>> {
    let m62 = v1.clone();
    match m62 {
        Adt17::C0(_) => {
            f41(v1)
        }
        Adt17::C1(_) => {
            f41(v1)
        }
        Adt17::C2(v2) => {
            f39(v0, f19()?, Fn2::F40, f41(v1)?, v2)
        }
        Adt17::C3(_) => {
            f41(v1)
        }
        Adt17::C4(_) => {
            f41(v1)
        }
        Adt17::C5(_) => {
            f41(v1)
        }
        Adt17::C6(_) => {
            f41(v1)
        }
        Adt17::C7(_) => {
            f41(v1)
        }
        Adt17::C8(_) => {
            f41(v1)
        }
        Adt17::C9(v3) => {
            f42(v0, v3)
        }
        Adt17::C10(v4) => {
            f43(v0, v4)
        }
        Adt17::C11(v5) => {
            f44(v0, v5)
        }
    }
}

pub fn f23(v0: Bytes, p65: u64, p66: List<u64>) -> R<List<u64>> {
    let m64 = (p65, p66);
    {
        let mut m64 = m64;
        loop {
            let step = {
                let (v1, v2) = m64;
                let m63 = v1;
                if m63 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m63 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f45(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m64 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f24(v0: Bytes, v1: Option<Adt99>) -> R<Result<Adt95, Adt96>> {
    let m67 = v1;
    match m67 {
        None => {
            Ok(Err::<Adt95, Adt96>(Adt96::C0(0u64)))
        }
        Some(v2) => {
            f46(v0.clone(), v2.clone(), f47(v0, match v2 {
                Adt99::C0(_, h68) => {
                    h68
                }
            })?)
        }
    }
}

pub fn f25(v0: Bytes, v1: u64) -> R<Option<Adt99>> {
    let m69 = f47(v0.clone(), v1)?;
    match m69 {
        None => {
            Ok(None::<Adt99>)
        }
        Some(v2) => {
            let m75 = {
                let a72 = v0;
                let a73 = {
                    let a70 = v1;
                    let a71 = 4u64;
                    nat_add(a70, a71)?
                };
                let a74 = v2;
                slice_bytes(a72, a73, a74)
            };
            match m75 {
                None => {
                    Ok(None::<Adt99>)
                }
                Some(v3) => {
                    Ok(Some(Adt99::C0(v3, {
                        let a78 = {
                            let a76 = v1;
                            let a77 = 4u64;
                            nat_add(a76, a77)?
                        };
                        let a79 = v2;
                        nat_add(a78, a79)?
                    })))
                }
            }
        }
    }
}

pub fn f26(v0: Adt100) -> List<u64> {
    let a103 = f9(match v0.clone() {
        Adt100::C0(h80, _, _, _, _) => {
            h80
        }
    });
    let a104 = {
        let a101 = f9({
            let a82 = match v0.clone() {
                Adt100::C0(_, h81, _, _, _) => {
                    h81
                }
            };
            length_list(a82)
        });
        let a102 = {
            let a99 = f48(match v0.clone() {
                Adt100::C0(_, h83, _, _, _) => {
                    h83
                }
            });
            let a100 = {
                let a97 = f9({
                    let a85 = match v0.clone() {
                        Adt100::C0(_, _, h84, _, _) => {
                            h84
                        }
                    };
                    length_list(a85)
                });
                let a98 = {
                    let a95 = f49(match v0.clone() {
                        Adt100::C0(_, _, h86, _, _) => {
                            h86
                        }
                    });
                    let a96 = {
                        let a93 = f9({
                            let a88 = match v0.clone() {
                                Adt100::C0(_, _, _, h87, _) => {
                                    h87
                                }
                            };
                            length_list(a88)
                        });
                        let a94 = {
                            let a91 = match v0.clone() {
                                Adt100::C0(_, _, _, h89, _) => {
                                    h89
                                }
                            };
                            let a92 = f50(match v0 {
                                Adt100::C0(_, _, _, _, h90) => {
                                    h90
                                }
                            });
                            append_list(a91, a92)
                        };
                        append_list(a93, a94)
                    };
                    append_list(a95, a96)
                };
                append_list(a97, a98)
            };
            append_list(a99, a100)
        };
        append_list(a101, a102)
    };
    append_list(a103, a104)
}

pub fn f27() -> Adt100 {
    Adt100::C0(f51(), f52(), List::<List<u64>>::nil(), f53(), None::<List<u64>>)
}

pub fn f28(v0: Adt0, v1: Fn1, v2: bool, v3: Adt97) -> R<Adt94> {
    f54(v0, v1, v2, v3.clone(), f55(match v3.clone() {
        Adt97::C0(_, _, h105, _, _, _, _, _) => {
            h105
        }
    }, match v3 {
        Adt97::C0(_, _, _, _, _, h106, _, _) => {
            h106
        }
    })?)
}

pub fn f29(v0: Fn0, v1: Adt95, v2: Adt16, v3: Adt17) -> R<Result<Adt35, Adt92>> {
    let c107: Fn0 = v0;
    c107.apply(Adt14::C0(match v1 {
        Adt95::C0(_, h108) => {
            h108
        }
    }, v2, v3))
}

pub fn f30(p114: List<Adt15>, v1: u64) -> bool {
    let m113 = p114;
    {
        let mut m113 = m113;
        loop {
            let step = {
                let v0 = m113;
                let m109 = v0;
                match m109.uncons() {
                    None => {
                        Err::<List<Adt15>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a111 = match v2 {
                            Adt15::C0(h110, _) => {
                                h110
                            }
                        };
                        let a112 = v1;
                        if nat_eq(a111, a112) {
                            Err::<List<Adt15>, bool>(true)
                        } else {
                            Ok::<List<Adt15>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m113 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f31(v0: Bytes, v1: Adt99, v2: Option<Adt99>) -> R<Result<Adt97, Adt96>> {
    let m115 = v2;
    match m115 {
        None => {
            Ok(Err::<Adt97, Adt96>(Adt96::C0(match v1 {
                Adt99::C0(_, h116) => {
                    h116
                }
            })))
        }
        Some(v3) => {
            f56(v0.clone(), v1, v3.clone(), f47(v0, match v3 {
                Adt99::C0(_, h117) => {
                    h117
                }
            })?)
        }
    }
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f33() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f34(v0: List<Adt38>) -> R<Result<List<Adt12>, Adt101>> {
    let m118 = f57(v0.clone());
    match m118 {
        None => {
            f58(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt101>(v1))
        }
    }
}

pub fn f35() -> R<List<Adt38>> {
    Ok(List::cons(f59(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f60()), List::cons(f59(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f61()), List::cons(f59(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f60()), List::cons(f59(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f62()), List::cons(f59(Str::lit("test_image_content"), Str::lit("Tests image content response"), f62()), List::cons(f59(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f62()), List::cons(f59(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f62()), List::cons(f59(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f62()), List::cons(f59(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f62()), List::cons(f59(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f62()), List::cons(f59(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f63()), List::cons(f59(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f64()?), List::cons(f59(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f65()), List::cons(f66(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f62(), f67()), List::cons(f66(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f62(), f67()), List::cons(f59(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f62()), List::cons(f59(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f62()), List::cons(f59(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f62()), List::cons(f59(Str::lit("test_streaming_elicitation"), Str::lit("Diagnostic tool validating response progress streams"), f62()), List::cons(f59(Str::lit("test_logging_tool"), Str::lit("Diagnostic logging validator tool"), f62()), List::<Adt38>::nil())))))))))))))))))))))
}

pub fn f36() -> List<Adt38> {
    List::cons(f68(Str::lit("test_input_required_result_elicitation"), Str::lit("MRTR: returns InputRequiredResult with elicitation request")), List::cons(f68(Str::lit("test_input_required_result_sampling"), Str::lit("MRTR: returns InputRequiredResult with sampling request")), List::cons(f68(Str::lit("test_input_required_result_list_roots"), Str::lit("MRTR: returns InputRequiredResult with roots/list request")), List::cons(f68(Str::lit("test_input_required_result_request_state"), Str::lit("MRTR: returns InputRequiredResult with requestState")), List::cons(f68(Str::lit("test_input_required_result_multiple_inputs"), Str::lit("MRTR: returns InputRequiredResult with multiple input requests")), List::cons(f68(Str::lit("test_input_required_result_multi_round"), Str::lit("MRTR: multi-round InputRequiredResult workflow")), List::cons(f68(Str::lit("test_input_required_result_tampered_state"), Str::lit("MRTR: HMAC-signed requestState integrity test")), List::cons(f68(Str::lit("test_input_required_result_capabilities"), Str::lit("MRTR: respects client capabilities in inputRequests")), List::<Adt38>::nil()))))))))
}

pub fn f37() -> List<Adt38> {
    List::cons(f69(Str::lit("greet"), Str::lit("Answers a greeting at once; never a task"), f70(Str::lit("name"))), List::cons(f69(Str::lit("slow_compute"), Str::lit("Computes for a number of seconds; a task for a client that declared the tasks extension"), f71()), List::cons(f69(Str::lit("failing_job"), Str::lit("Always fails; always a task, so the client must declare the tasks extension"), f72()), List::cons(f69(Str::lit("protocol_error_job"), Str::lit("Ends in a protocol error; a task for a client that declared the tasks extension"), f72()), List::cons(f69(Str::lit("confirm_delete"), Str::lit("Asks the client to confirm a deletion; a task that waits for input"), f70(Str::lit("filename"))), List::cons(f69(Str::lit("multi_input"), Str::lit("Asks the client two questions at once; a task that waits for both answers"), f72()), List::cons(f69(Str::lit("test_tool_with_task"), Str::lit("Asks for a name, then becomes a task; the client must declare the tasks extension"), f72()), List::<Adt38>::nil())))))))
}

pub fn f38(v0: Adt101) -> Str {
    let m119 = v0;
    match m119 {
        Adt101::C0(v1) => {
            let a120 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a121 = Str::lit(" ");
            join(a120, a121)
        }
        Adt101::C1(v2) => {
            let a122 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a123 = Str::lit(" ");
            join(a122, a123)
        }
        Adt101::C2(v3) => {
            let a124 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a125 = Str::lit(" ");
            join(a124, a125)
        }
        Adt101::C3(v4) => {
            let a126 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a127 = Str::lit(" ");
            join(a126, a127)
        }
        Adt101::C4(v5, v6) => {
            let a128 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a129 = Str::lit(" ");
            join(a128, a129)
        }
        Adt101::C5(v7, v8) => {
            let a130 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a131 = Str::lit(" ");
            join(a130, a131)
        }
        Adt101::C6(v9, v10) => {
            let a132 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a133 = Str::lit(" ");
            join(a132, a133)
        }
        Adt101::C7(v11) => {
            let a134 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a135 = Str::lit(" ");
            join(a134, a135)
        }
        Adt101::C8(v12) => {
            let a136 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a137 = Str::lit(" ");
            join(a136, a137)
        }
        Adt101::C9(v13) => {
            let a138 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a139 = Str::lit(" ");
            join(a138, a139)
        }
        Adt101::C10(v14) => {
            let a140 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a141 = Str::lit(" ");
            join(a140, a141)
        }
        Adt101::C11(v15) => {
            let a142 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a143 = Str::lit(" ");
            join(a142, a143)
        }
        Adt101::C12(v16, v17) => {
            let a144 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a145 = Str::lit(" ");
            join(a144, a145)
        }
    }
}

pub fn f39(v0: Adt16, v1: List<Adt38>, v2: Fn2, v3: Result<Adt35, Adt92>, v4: Adt24) -> R<Result<Adt35, Adt92>> {
    let m147 = f73(match v4.clone() {
        Adt24::C0(_, h146, _, _, _) => {
            h146
        }
    }, v1);
    match m147 {
        None => {
            Ok(Err::<Adt35, Adt92>(Adt92::C0(match v4 {
                Adt24::C0(_, h148, _, _, _) => {
                    h148
                }
            })))
        }
        Some(v5) => {
            f74(v0, v5, v4.clone(), {
                let c149: Fn2 = v2;
                c149.apply(v4)?
            }, v3)
        }
    }
}

pub fn f40(v0: Adt24) -> R<Option<Adt102>> {
    if f75(v0.clone(), Str::lit("greet")) {
        Ok(Some(Adt102::C0(f76({
            let a150 = List::cons(Str::lit("Hello, "), List::cons(f77(v0, Str::lit("name"), Str::lit("world")), List::cons(Str::lit("!"), List::<Str>::nil())));
            let a151 = Str::lit("");
            join(a150, a151)
        }, false))))
    } else {
        if f75(v0.clone(), Str::lit("slow_compute")) {
            Ok(Some(f78(v0)?))
        } else {
            if f75(v0.clone(), Str::lit("confirm_delete")) {
                Ok(Some(f79(v0)?))
            } else {
                if f75(v0.clone(), Str::lit("multi_input")) {
                    Ok(Some(f80()?))
                } else {
                    if f75(v0.clone(), Str::lit("failing_job")) {
                        Ok(Some(f81()?))
                    } else {
                        if f75(v0.clone(), Str::lit("protocol_error_job")) {
                            Ok(Some(f82()))
                        } else {
                            if f75(v0.clone(), Str::lit("test_tool_with_task")) {
                                Ok(Some(f83(v0)?))
                            } else {
                                Ok(None::<Adt102>)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f41(v0: Adt17) -> R<Result<Adt35, Adt92>> {
    let m152 = v0;
    match m152 {
        Adt17::C0(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt17::C1(v1) => {
            Ok(f84(f17(), f19()?, v1))
        }
        Adt17::C2(v2) => {
            if f85(match v2.clone() {
                Adt24::C0(_, h153, _, _, _) => {
                    h153
                }
            }) {
                f86(f19()?, Fn3::F87, v2)
            } else {
                f88(f19()?, Fn4::F89, v2)
            }
        }
        Adt17::C3(v4) => {
            Ok(f90(v4))
        }
        Adt17::C4(v5) => {
            f91(v5)
        }
        Adt17::C5(v6) => {
            Ok(f92(v6))
        }
        Adt17::C6(v7) => {
            Ok(f93(v7))
        }
        Adt17::C7(v8) => {
            if f94(match v8.clone() {
                Adt26::C0(_, h154, _, _, _) => {
                    h154
                }
            }) {
                Ok(f95(v8))
            } else {
                Ok(f96(v8))
            }
        }
        Adt17::C8(v9) => {
            f97(v9)
        }
        Adt17::C9(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt17::C10(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt17::C11(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f42(v0: Adt16, v1: Adt32) -> R<Result<Adt35, Adt92>> {
    let m157 = f98(v0.clone(), match v1.clone() {
        Adt32::C0(h155, _) => {
            h155
        }
    }, match v1 {
        Adt32::C0(_, h156) => {
            h156
        }
    })?;
    match m157 {
        Ok(v3) => {
            f99(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f43(v0: Adt16, v1: Adt33) -> R<Result<Adt35, Adt92>> {
    let m160 = f98(v0.clone(), match v1.clone() {
        Adt33::C0(h158, _, _) => {
            h158
        }
    }, match v1.clone() {
        Adt33::C0(_, h159, _) => {
            h159
        }
    })?;
    match m160 {
        Ok(v3) => {
            f100(v0, v3, match v1 {
                Adt33::C0(_, _, h161) => {
                    h161
                }
            })
        }
        Err(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f44(v0: Adt16, v1: Adt34) -> R<Result<Adt35, Adt92>> {
    let m164 = f98(v0.clone(), match v1.clone() {
        Adt34::C0(h162, _) => {
            h162
        }
    }, match v1 {
        Adt34::C0(_, h163) => {
            h163
        }
    })?;
    match m164 {
        Ok(v3) => {
            f101(v0, v3)
        }
        Err(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f45(v0: Bytes, v1: u64) -> R<u64> {
    let m167 = {
        let a165 = v0;
        let a166 = v1;
        index_bytes(a165, a166)
    };
    match m167 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a212 = {
                let a188 = {
                    let a176 = {
                        let a170 = {
                            let a168 = v3;
                            let a169 = 128u8;
                            fixed_u8::bit_and(a168, a169)
                        };
                        let a171 = 128u8;
                        if equal(a170, a171) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a177 = {
                        let a174 = {
                            let a172 = v3;
                            let a173 = 64u8;
                            fixed_u8::bit_and(a172, a173)
                        };
                        let a175 = 64u8;
                        if equal(a174, a175) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a176, a177)?
                };
                let a189 = {
                    let a186 = {
                        let a180 = {
                            let a178 = v3;
                            let a179 = 32u8;
                            fixed_u8::bit_and(a178, a179)
                        };
                        let a181 = 32u8;
                        if equal(a180, a181) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a187 = {
                        let a184 = {
                            let a182 = v3;
                            let a183 = 16u8;
                            fixed_u8::bit_and(a182, a183)
                        };
                        let a185 = 16u8;
                        if equal(a184, a185) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a186, a187)?
                };
                nat_add(a188, a189)?
            };
            let a213 = {
                let a210 = {
                    let a198 = {
                        let a192 = {
                            let a190 = v3;
                            let a191 = 8u8;
                            fixed_u8::bit_and(a190, a191)
                        };
                        let a193 = 8u8;
                        if equal(a192, a193) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a199 = {
                        let a196 = {
                            let a194 = v3;
                            let a195 = 4u8;
                            fixed_u8::bit_and(a194, a195)
                        };
                        let a197 = 4u8;
                        if equal(a196, a197) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a198, a199)?
                };
                let a211 = {
                    let a208 = {
                        let a202 = {
                            let a200 = v3;
                            let a201 = 2u8;
                            fixed_u8::bit_and(a200, a201)
                        };
                        let a203 = 2u8;
                        if equal(a202, a203) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a209 = {
                        let a206 = {
                            let a204 = v3;
                            let a205 = 1u8;
                            fixed_u8::bit_and(a204, a205)
                        };
                        let a207 = 1u8;
                        if equal(a206, a207) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a208, a209)?
                };
                nat_add(a210, a211)?
            };
            nat_add(a212, a213)
        }
    }
}

pub fn f46(v0: Bytes, v1: Adt99, v2: Option<u64>) -> R<Result<Adt95, Adt96>> {
    let m214 = v2;
    match m214 {
        None => {
            Ok(Err::<Adt95, Adt96>(Adt96::C0(match v1 {
                Adt99::C0(_, h215) => {
                    h215
                }
            })))
        }
        Some(v3) => {
            let m219 = f102(v0.clone(), {
                let a217 = match v1.clone() {
                    Adt99::C0(_, h216) => {
                        h216
                    }
                };
                let a218 = 4u64;
                nat_add(a217, a218)?
            }, v3, List::<Adt15>::nil())?;
            match m219 {
                Ok(v5) => {
                    Ok(f103(v0, match v1 {
                        Adt99::C0(h220, _) => {
                            h220
                        }
                    }, v5))
                }
                Err(v4) => {
                    Ok(Err::<Adt95, Adt96>(v4))
                }
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a224 = {
        let a221 = v1;
        let a222 = 4u64;
        nat_add(a221, a222)?
    };
    let a225 = {
        let a223 = v0.clone();
        length_bytes(a223)
    };
    if nat_le(a224, a225) {
        Ok(Some({
            let a242 = {
                let a226 = f45(v0.clone(), v1)?;
                let a227 = 16777216u64;
                nat_mul(a226, a227)?
            };
            let a243 = {
                let a240 = {
                    let a230 = f45(v0.clone(), {
                        let a228 = v1;
                        let a229 = 1u64;
                        nat_add(a228, a229)?
                    })?;
                    let a231 = 65536u64;
                    nat_mul(a230, a231)?
                };
                let a241 = {
                    let a238 = {
                        let a234 = f45(v0.clone(), {
                            let a232 = v1;
                            let a233 = 2u64;
                            nat_add(a232, a233)?
                        })?;
                        let a235 = 256u64;
                        nat_mul(a234, a235)?
                    };
                    let a239 = f45(v0, {
                        let a236 = v1;
                        let a237 = 3u64;
                        nat_add(a236, a237)?
                    })?;
                    nat_add(a238, a239)?
                };
                nat_add(a240, a241)?
            };
            nat_add(a242, a243)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f48(v0: List<Adt98>) -> List<u64> {
    f104(f105(v0, List::<Adt98>::nil()), List::<u64>::nil())
}

pub fn f49(v0: List<List<u64>>) -> List<u64> {
    f106(f107(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f50(v0: Option<List<u64>>) -> List<u64> {
    let m244 = v0;
    match m244 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a248 = f9(1u64);
            let a249 = {
                let a246 = f9({
                    let a245 = v1.clone();
                    length_list(a245)
                });
                let a247 = v1;
                append_list(a246, a247)
            };
            append_list(a248, a249)
        }
    }
}

pub fn f51() -> u64 {
    500u64
}

pub fn f52() -> List<Adt98> {
    List::cons(Adt98::C0(f108(), f109()), List::<Adt98>::nil())
}

pub fn f53() -> List<u64> {
    let a252 = f110();
    let a253 = {
        let a250 = f111();
        let a251 = f112();
        append_list(a250, a251)
    };
    append_list(a252, a253)
}

pub fn f54(v0: Adt0, v1: Fn1, v2: bool, v3: Adt97, v4: Option<Adt104>) -> R<Adt94> {
    let m254 = v4;
    match m254 {
        None => {
            f114(v0, v1, v2, v3)
        }
        Some(v5) => {
            Ok(Adt94::C0(f26(f113(v5)?)))
        }
    }
}

pub fn f55(v0: List<Adt98>, v1: List<List<u64>>) -> R<Option<Adt104>> {
    let m255 = f115(v0.clone(), v1.clone())?;
    match m255 {
        None => {
            f116(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt99, v2: Adt99, v3: Option<u64>) -> R<Result<Adt97, Adt96>> {
    let m256 = v3;
    match m256 {
        None => {
            Ok(Err::<Adt97, Adt96>(Adt96::C0(match v2 {
                Adt99::C0(_, h257) => {
                    h257
                }
            })))
        }
        Some(v4) => {
            let m261 = f117(v0.clone(), {
                let a259 = match v2.clone() {
                    Adt99::C0(_, h258) => {
                        h258
                    }
                };
                let a260 = 4u64;
                nat_add(a259, a260)?
            }, v4, List::<Adt98>::nil())?;
            match m261 {
                Ok(v6) => {
                    f118(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt97, Adt96>(v5))
                }
            }
        }
    }
}

pub fn f57(v0: List<Adt38>) -> Option<Adt101> {
    let m262 = f119(v0, List::<Str>::nil());
    match m262 {
        None => {
            None::<Adt101>
        }
        Some(v1) => {
            Some(Adt101::C1(v1))
        }
    }
}

pub fn f58(p266: List<Adt38>, p267: List<Adt12>) -> R<Result<List<Adt12>, Adt101>> {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v0, v1) = m265;
                let m263 = v0;
                match m263.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt12>), R<Result<List<Adt12>, Adt101>>>(Ok(Ok::<List<Adt12>, Adt101>(f120(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m264 = f121(v2)?;
                        match m264 {
                            Ok(v5) => {
                                Ok::<(List<Adt38>, List<Adt12>), R<Result<List<Adt12>, Adt101>>>((v3, f120(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt38>, List<Adt12>), R<Result<List<Adt12>, Adt101>>>(Ok(Err::<List<Adt12>, Adt101>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Str, v1: Str, v2: Adt5) -> Adt38 {
    Adt38::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt39>, None::<Adt5>)
}

pub fn f60() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f61() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f122(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f123()), List::cons((Str::lit("limit"), f122(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f122(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f62() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f64() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f123()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f124(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f124(0u64)?), List::cons((Str::lit("maximum"), f124(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f124(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f65() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f66(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt38 {
    Adt38::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt39>, None::<Adt5>)
}

pub fn f67() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f68(v0: Str, v1: Str) -> Adt38 {
    Adt38::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))), None::<Adt5>, None::<Adt39>, None::<Adt5>)
}

pub fn f69(v0: Str, v1: Str, v2: Adt5) -> Adt38 {
    Adt38::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt39>, None::<Adt5>)
}

pub fn f70(v0: Str) -> Adt5 {
    f125(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), f126(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f71() -> Adt5 {
    f125(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("seconds"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), Adt6::C2(Adt7::C0(0i64, 0i64))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("label"), f126(Str::lit("string"))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("seconds")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f72() -> Adt5 {
    f125(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f73(v0: Str, p273: List<Adt38>) -> Option<Adt38> {
    let m272 = p273;
    {
        let mut m272 = m272;
        loop {
            let step = {
                let v1 = m272;
                let m268 = v1;
                match m268.uncons() {
                    None => {
                        Err::<List<Adt38>, Option<Adt38>>(None::<Adt38>)
                    }
                    Some((v2, v3)) => {
                        let a270 = match v2.clone() {
                            Adt38::C0(h269, _, _, _, _, _, _, _) => {
                                h269
                            }
                        };
                        let a271 = v0.clone();
                        if equal(a270, a271) {
                            Err::<List<Adt38>, Option<Adt38>>(Some(v2))
                        } else {
                            Ok::<List<Adt38>, Option<Adt38>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(v0: Adt16, v1: Adt38, v2: Adt24, v3: Option<Adt102>, v4: Result<Adt35, Adt92>) -> R<Result<Adt35, Adt92>> {
    let m274 = v3;
    match m274 {
        None => {
            Ok(v4)
        }
        Some(v5) => {
            f127(v0, v1.clone(), v2.clone(), v5, f128(v1, v2)?)
        }
    }
}

pub fn f75(v0: Adt24, v1: Str) -> bool {
    let a276 = match v0 {
        Adt24::C0(_, h275, _, _, _) => {
            h275
        }
    };
    let a277 = v1;
    equal(a276, a277)
}

pub fn f76(v0: Str, v1: bool) -> Adt52 {
    Adt52::C0(Adt40::C0(List::cons(f129(v0), List::<Adt41>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    }), List::<Adt53>::nil())
}

pub fn f77(v0: Adt24, v1: Str, v2: Str) -> Str {
    let m278 = f130(v0, v1);
    match m278 {
        Adt109::C0 => {
            v2
        }
        Adt109::C1(v3) => {
            let m279 = f131(v3);
            match m279 {
                None => {
                    v2
                }
                Some(v4) => {
                    v4
                }
            }
        }
    }
}

pub fn f78(v0: Adt24) -> R<Adt102> {
    let v1: i64 = f132(v0.clone());
    let v2: Str = {
        let a281 = List::cons(Str::lit("slow_compute"), List::cons(f77(v0, Str::lit("label"), Str::lit("unnamed")), List::cons(Str::lit("finished after"), List::cons({
            let a280 = v1;
            format_i64(a280)
        }, List::cons(Str::lit("seconds"), List::<Str>::nil())))));
        let a282 = Str::lit(" ");
        join(a281, a282)
    };
    Ok(Adt102::C1(Adt103::C0, f76(v2.clone(), false), f133(0i64, List::<(Str, Adt67)>::nil(), {
        let a283 = v1;
        let a284 = 1000i64;
        int_mul(a283, a284)?
    }, f134(v2, false)?)))
}

pub fn f79(v0: Adt24) -> R<Adt102> {
    let v1: Str = {
        let a285 = List::cons(Str::lit("Deleted"), List::cons(f77(v0.clone(), Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a286 = Str::lit(" ");
        join(a285, a286)
    };
    Ok(Adt102::C1(Adt103::C0, f76(v1.clone(), false), f133(0i64, List::cons((Str::lit("confirm"), f135({
        let a287 = List::cons(Str::lit("Delete"), List::cons(f77(v0, Str::lit("filename"), Str::lit("nothing")), List::<Str>::nil()));
        let a288 = Str::lit(" ");
        join(a287, a288)
    }, Str::lit("confirm"), Str::lit("boolean"))), List::<(Str, Adt67)>::nil()), 0i64, f134(v1, false)?)))
}

pub fn f80() -> R<Adt102> {
    Ok(Adt102::C1(Adt103::C0, f76(Str::lit("Both inputs received"), false), f133(0i64, List::cons((Str::lit("first"), f135(Str::lit("First name"), Str::lit("name"), Str::lit("string"))), List::cons((Str::lit("second"), f135(Str::lit("Second name"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt67)>::nil())), 0i64, f134(Str::lit("Both inputs received"), false)?)))
}

pub fn f81() -> R<Adt102> {
    Ok(Adt102::C1(Adt103::C1, f76(Str::lit("failing_job always fails"), true), f133(0i64, List::<(Str, Adt67)>::nil(), 1000i64, f134(Str::lit("failing_job always fails"), true)?)))
}

pub fn f82() -> Adt102 {
    Adt102::C1(Adt103::C0, f76(Str::lit("protocol_error_job has no result"), true), f133(0i64, List::<(Str, Adt67)>::nil(), 500i64, Adt89::C1(Adt83::C0(-32603i64, Str::lit("protocol_error_job failed"), None::<Adt6>))))
}

pub fn f83(v0: Adt24) -> R<Adt102> {
    let m290 = match v0 {
        Adt24::C0(_, _, _, h289, _) => {
            h289
        }
    };
    match m290 {
        None => {
            Ok(f136())
        }
        Some(v1) => {
            let m291 = f137(v1);
            match m291 {
                None => {
                    Ok(f136())
                }
                Some(v2) => {
                    f138(v2)
                }
            }
        }
    }
}

pub fn f84(v0: Adt10, v1: List<Adt38>, v2: Adt23) -> Result<Adt35, Adt92> {
    let m293 = match v2 {
        Adt23::C0(_, h292) => {
            h292
        }
    };
    match m293 {
        None => {
            Ok::<Adt35, Adt92>(Adt35::C1(Adt37::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt35, Adt92>(Adt92::C5(v3))
        }
    }
}

pub fn f85(v0: Str) -> bool {
    let a294 = v0.clone();
    let a295 = Str::lit("test_input_required_result_elicitation");
    if equal(a294, a295) {
        true
    } else {
        let a296 = v0.clone();
        let a297 = Str::lit("test_input_required_result_sampling");
        if equal(a296, a297) {
            true
        } else {
            let a298 = v0.clone();
            let a299 = Str::lit("test_input_required_result_list_roots");
            if equal(a298, a299) {
                true
            } else {
                let a300 = v0.clone();
                let a301 = Str::lit("test_input_required_result_request_state");
                if equal(a300, a301) {
                    true
                } else {
                    let a302 = v0.clone();
                    let a303 = Str::lit("test_input_required_result_multiple_inputs");
                    if equal(a302, a303) {
                        true
                    } else {
                        let a304 = v0.clone();
                        let a305 = Str::lit("test_input_required_result_multi_round");
                        if equal(a304, a305) {
                            true
                        } else {
                            let a306 = v0.clone();
                            let a307 = Str::lit("test_input_required_result_tampered_state");
                            if equal(a306, a307) {
                                true
                            } else {
                                let a308 = v0;
                                let a309 = Str::lit("test_input_required_result_capabilities");
                                equal(a308, a309)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f86(v0: List<Adt38>, v1: Fn3, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    let m311 = f73(match v2.clone() {
        Adt24::C0(_, h310, _, _, _) => {
            h310
        }
    }, v0);
    match m311 {
        None => {
            Ok(Err::<Adt35, Adt92>(Adt92::C0(match v2 {
                Adt24::C0(_, h312, _, _, _) => {
                    h312
                }
            })))
        }
        Some(v3) => {
            f139(v3.clone(), v2, Fn5::F140(v1, v3))
        }
    }
}

pub fn f87(v0: Adt24) -> R<Result<Adt105, Adt92>> {
    if f141(v0.clone(), Str::lit("test_input_required_result_elicitation")) {
        Ok(f142(v0))
    } else {
        if f141(v0.clone(), Str::lit("test_input_required_result_sampling")) {
            Ok(f143(v0))
        } else {
            if f141(v0.clone(), Str::lit("test_input_required_result_list_roots")) {
                f144(v0)
            } else {
                if f141(v0.clone(), Str::lit("test_input_required_result_request_state")) {
                    Ok(f145(v0))
                } else {
                    if f141(v0.clone(), Str::lit("test_input_required_result_multiple_inputs")) {
                        f146(v0)
                    } else {
                        if f141(v0.clone(), Str::lit("test_input_required_result_multi_round")) {
                            f147(v0)
                        } else {
                            if f141(v0.clone(), Str::lit("test_input_required_result_tampered_state")) {
                                Ok(f148(v0))
                            } else {
                                if f141(v0.clone(), Str::lit("test_input_required_result_capabilities")) {
                                    Ok(f149(v0))
                                } else {
                                    Ok(Err::<Adt105, Adt92>(Adt92::C9(Str::lit("not an input-required tool"))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f88(v0: List<Adt38>, v1: Fn4, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    let m314 = f73(match v2.clone() {
        Adt24::C0(_, h313, _, _, _) => {
            h313
        }
    }, v0);
    match m314 {
        None => {
            Ok(Err::<Adt35, Adt92>(Adt92::C0(match v2 {
                Adt24::C0(_, h315, _, _, _) => {
                    h315
                }
            })))
        }
        Some(v3) => {
            f150(v3, v1, v2)
        }
    }
}

pub fn f89(v0: Adt24) -> R<Result<Adt52, Adt92>> {
    if f151(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("This is a simple text response for testing.")), List::<Adt41>::nil()))))
    } else {
        if f151(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt52, Adt92>(f152(List::cons(f154(), List::<Adt41>::nil()))))
        } else {
            if f151(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt52, Adt92>(f152(List::cons(f155(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt41>::nil()))))
            } else {
                if f151(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt52, Adt92>(f152(List::cons(f156(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt41>::nil()))))
                } else {
                    if f151(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("Multiple content types test:")), List::cons(f154(), List::cons(f156(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt41>::nil()))))))
                    } else {
                        if f151(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt52, Adt92>(f157(List::cons(f153(Str::lit("This tool intentionally returns an error for testing")), List::<Adt41>::nil()), None::<Adt6>, true, List::<Adt53>::nil())))
                        } else {
                            if f151(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt52, Adt92>(f158(v0, f159()?)))
                            } else {
                                if f151(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt52, Adt92>(f158(v0, f160()?)))
                                } else {
                                    if f151(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt52, Adt92>(f157(List::cons(f153(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt41>::nil()), Some(f161()?), false, List::<Adt53>::nil())))
                                    } else {
                                        if f151(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt52, Adt92>(f157(List::cons(f153(Str::lit("{\"sum\":\"three\"}")), List::<Adt41>::nil()), Some(f162()), false, List::<Adt53>::nil())))
                                        } else {
                                            if f151(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt52, Adt92>(f152(List::cons(f163(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt41>::nil()))))
                                            } else {
                                                if f151(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt52, Adt92>(f152(List::cons(f153(f164()), List::<Adt41>::nil()))))
                                                } else {
                                                    if f151(v0.clone(), Str::lit("test_streaming_elicitation")) {
                                                        Ok(Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("Streaming complete")), List::<Adt41>::nil()))))
                                                    } else {
                                                        if f151(v0.clone(), Str::lit("test_logging_tool")) {
                                                            Ok(Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("Logging evaluated")), List::<Adt41>::nil()))))
                                                        } else {
                                                            Ok(f165(v0))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f90(v0: Adt23) -> Result<Adt35, Adt92> {
    f166(f167(), f168(), v0)
}

pub fn f91(v0: Adt25) -> R<Result<Adt35, Adt92>> {
    f169(f170(), Fn6::F171, v0)
}

pub fn f92(v0: Adt23) -> Result<Adt35, Adt92> {
    f172(f167(), f173(), v0)
}

pub fn f93(v0: Adt23) -> Result<Adt35, Adt92> {
    f174(f167(), f175(), v0)
}

pub fn f94(v0: Str) -> bool {
    let a316 = v0;
    let a317 = f176();
    equal(a316, a317)
}

pub fn f95(v0: Adt26) -> Result<Adt35, Adt92> {
    let m319 = f177(Str::lit("user_context"), Adt106::C0, match v0 {
        Adt26::C0(_, _, _, h318, _) => {
            h318
        }
    });
    match m319 {
        Adt110::C0 => {
            Ok::<Adt35, Adt92>(Adt35::C12(Adt84::C0(f180(Str::lit("user_context"), f181(Str::lit("What context should the prompt use?"), Str::lit("context"), Str::lit("string"))))))
        }
        Adt110::C1 => {
            Err::<Adt35, Adt92>(Adt92::C11)
        }
        Adt110::C2(v1) => {
            Ok::<Adt35, Adt92>(Adt35::C8(f178(f179(v1, Str::lit("context"), Str::lit("unknown")))))
        }
    }
}

pub fn f96(v0: Adt26) -> Result<Adt35, Adt92> {
    f182(f175(), Fn7::F183, v0)
}

pub fn f97(v0: Adt27) -> R<Result<Adt35, Adt92>> {
    f184(Fn8::F185, v0)
}

pub fn f98(v0: Adt16, v1: Adt19, v2: Str) -> R<Result<Adt87, Adt92>> {
    if f186(match v1 {
        Adt19::C0(_, h320, _, _, _, _) => {
            h320
        }
    }) {
        let m322 = f187(match v0.clone() {
            Adt16::C0(_, _, _, h321) => {
                h321
            }
        })?;
        match m322 {
            None => {
                Ok(Err::<Adt87, Adt92>(f188()))
            }
            Some(v3) => {
                let m324 = f189(v2.clone(), v3, match v0 {
                    Adt16::C0(h323, _, _, _) => {
                        h323
                    }
                })?;
                match m324 {
                    None => {
                        Ok(Err::<Adt87, Adt92>(Adt92::C3(v2)))
                    }
                    Some(v4) => {
                        Ok(Ok::<Adt87, Adt92>(v4))
                    }
                }
            }
        }
    } else {
        Ok(Err::<Adt87, Adt92>(f190()))
    }
}

pub fn f99(v0: Adt16, v1: Adt87) -> R<Result<Adt35, Adt92>> {
    Ok(Ok::<Adt35, Adt92>(f191(f192(v1, match v0.clone() {
        Adt16::C0(h325, _, _, _) => {
            h325
        }
    }, match v0 {
        Adt16::C0(_, h326, _, _) => {
            h326
        }
    })?, Adt91::C0, false)))
}

pub fn f100(v0: Adt16, v1: Adt87, v2: Adt5) -> R<Result<Adt35, Adt92>> {
    let v3: Adt107 = f192(v1, match v0.clone() {
        Adt16::C0(h327, _, _, _) => {
            h327
        }
    }, match v0.clone() {
        Adt16::C0(_, h328, _, _) => {
            h328
        }
    })?;
    Ok(Ok::<Adt35, Adt92>(f191(f193(match v3.clone() {
        Adt107::C0(h329, _) => {
            h329
        }
    }, v2, match v0.clone() {
        Adt16::C0(h330, _, _, _) => {
            h330
        }
    }, match v0 {
        Adt16::C0(_, h331, _, _) => {
            h331
        }
    })?, Adt91::C1, match v3 {
        Adt107::C0(_, h332) => {
            h332
        }
    })))
}

pub fn f101(v0: Adt16, v1: Adt87) -> R<Result<Adt35, Adt92>> {
    Ok(Ok::<Adt35, Adt92>(f191(f194(v1, match v0.clone() {
        Adt16::C0(h333, _, _, _) => {
            h333
        }
    }, match v0 {
        Adt16::C0(_, h334, _, _) => {
            h334
        }
    })?, Adt91::C1, false)))
}

pub fn f102(v0: Bytes, p345: u64, p346: u64, p347: List<Adt15>) -> R<Result<(List<Adt15>, u64), Adt96>> {
    let m344 = (p345, (p346, p347));
    {
        let mut m344 = m344;
        loop {
            let step = {
                let (v1, (v2, v3)) = m344;
                let m335 = v2;
                if m335 == 0 {
                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt96>>>(Ok(Ok::<(List<Adt15>, u64), Adt96>((f195(v3, List::<Adt15>::nil()), v1))))
                } else {
                    let v4 = m335 - 1;
                    let m336 = f47(v0.clone(), v1)?;
                    match m336 {
                        None => {
                            Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt96>>>(Ok(Err::<(List<Adt15>, u64), Adt96>(Adt96::C0(v1))))
                        }
                        Some(v5) => {
                            let m339 = f25(v0.clone(), {
                                let a337 = v1;
                                let a338 = 4u64;
                                nat_add(a337, a338)?
                            })?;
                            match m339 {
                                None => {
                                    Err::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt96>>>(Ok(Err::<(List<Adt15>, u64), Adt96>(Adt96::C0({
                                        let a340 = v1;
                                        let a341 = 4u64;
                                        nat_add(a340, a341)?
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt15>)), R<Result<(List<Adt15>, u64), Adt96>>>((match v6.clone() {
                                        Adt99::C0(_, h342) => {
                                            h342
                                        }
                                    }, (v4, List::cons(Adt15::C0(v5, match v6 {
                                        Adt99::C0(h343, _) => {
                                            h343
                                        }
                                    }), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f103(v0: Bytes, v1: Bytes, v2: (List<Adt15>, u64)) -> Result<Adt95, Adt96> {
    f196({
        let a350 = {
            let (_, h348) = v2.clone();
            h348
        };
        let a351 = {
            let a349 = v0;
            length_bytes(a349)
        };
        nat_eq(a350, a351)
    }, v1, {
        let (h352, _) = v2;
        h352
    }, {
        let (_, h353) = v2;
        h353
    })
}

pub fn f104(p358: List<Adt98>, p359: List<u64>) -> List<u64> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v0, v1) = m357;
                let m354 = v0;
                match m354.uncons() {
                    None => {
                        Err::<(List<Adt98>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt98>, List<u64>), List<u64>>((v3, {
                            let a355 = f197(v2);
                            let a356 = v1;
                            append_list(a355, a356)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(p362: List<Adt98>, p363: List<Adt98>) -> List<Adt98> {
    let m361 = (p362, p363);
    {
        let mut m361 = m361;
        loop {
            let step = {
                let (v0, v1) = m361;
                let m360 = v0;
                match m360.uncons() {
                    None => {
                        Err::<(List<Adt98>, List<Adt98>), List<Adt98>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt98>, List<Adt98>), List<Adt98>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m361 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(p368: List<List<u64>>, p369: List<u64>) -> List<u64> {
    let m367 = (p368, p369);
    {
        let mut m367 = m367;
        loop {
            let step = {
                let (v0, v1) = m367;
                let m364 = v0;
                match m364.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a365 = f198(v2);
                            let a366 = v1;
                            append_list(a365, a366)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(p372: List<List<u64>>, p373: List<List<u64>>) -> List<List<u64>> {
    let m371 = (p372, p373);
    {
        let mut m371 = m371;
        loop {
            let step = {
                let (v0, v1) = m371;
                let m370 = v0;
                match m370.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m371 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f109() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f110() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f111() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f112() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f113(v0: Adt104) -> R<Adt100> {
    f199(f200(v0.clone()), f201(None::<Adt108>, f202(v0)))
}

pub fn f114(v0: Adt0, v1: Fn1, v2: bool, v3: Adt97) -> R<Adt94> {
    if f203(match v3.clone() {
        Adt97::C0(h374, _, _, _, _, _, _, _) => {
            h374
        }
    }) {
        f204(match v3.clone() {
            Adt97::C0(_, _, _, _, _, _, _, h375) => {
                h375
            }
        }, f205(v0, match v3.clone() {
            Adt97::C0(_, _, _, _, _, _, h376, _) => {
                h376
            }
        }), Fn9::F206(v1, v3.clone()), v2, match v3.clone() {
            Adt97::C0(_, _, _, _, h377, _, _, _) => {
                h377
            }
        }, match v3.clone() {
            Adt97::C0(_, _, h378, _, _, _, _, _) => {
                h378
            }
        }, f207(match v3 {
            Adt97::C0(_, _, _, h379, _, _, _, _) => {
                h379
            }
        })?)
    } else {
        Ok(Adt94::C0(f26(f208())))
    }
}

pub fn f115(v0: List<Adt98>, v1: List<List<u64>>) -> R<Option<Adt104>> {
    let m380 = f209(v0, f210())?;
    match m380 {
        Adt114::C0 => {
            Ok(None::<Adt104>)
        }
        Adt114::C1 => {
            Ok(Some(Adt104::C0))
        }
        Adt114::C2 => {
            Ok(Some(Adt104::C0))
        }
        Adt114::C3(v2) => {
            f211(f212(v2), v1, Adt104::C0)
        }
    }
}

pub fn f116(v0: List<Adt98>, v1: List<List<u64>>) -> R<Option<Adt104>> {
    let m381 = f209(v0, f213())?;
    match m381 {
        Adt114::C0 => {
            Ok(None::<Adt104>)
        }
        Adt114::C1 => {
            Ok(Some(Adt104::C1))
        }
        Adt114::C2 => {
            Ok(Some(Adt104::C1))
        }
        Adt114::C3(v2) => {
            f211(f214(v2), v1, Adt104::C1)
        }
    }
}

pub fn f117(v0: Bytes, p391: u64, p392: u64, p393: List<Adt98>) -> R<Result<(List<Adt98>, u64), Adt96>> {
    let m390 = (p391, (p392, p393));
    {
        let mut m390 = m390;
        loop {
            let step = {
                let (v1, (v2, v3)) = m390;
                let m382 = v2;
                if m382 == 0 {
                    Err::<(u64, (u64, List<Adt98>)), R<Result<(List<Adt98>, u64), Adt96>>>(Ok(Ok::<(List<Adt98>, u64), Adt96>((f215(v3, List::<Adt98>::nil()), v1))))
                } else {
                    let v4 = m382 - 1;
                    let m383 = f25(v0.clone(), v1)?;
                    match m383 {
                        None => {
                            Err::<(u64, (u64, List<Adt98>)), R<Result<(List<Adt98>, u64), Adt96>>>(Ok(Err::<(List<Adt98>, u64), Adt96>(Adt96::C0(v1))))
                        }
                        Some(v5) => {
                            let m385 = f25(v0.clone(), match v5.clone() {
                                Adt99::C0(_, h384) => {
                                    h384
                                }
                            })?;
                            match m385 {
                                None => {
                                    Err::<(u64, (u64, List<Adt98>)), R<Result<(List<Adt98>, u64), Adt96>>>(Ok(Err::<(List<Adt98>, u64), Adt96>(Adt96::C0(match v5 {
                                        Adt99::C0(_, h386) => {
                                            h386
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt98>)), R<Result<(List<Adt98>, u64), Adt96>>>((match v6.clone() {
                                        Adt99::C0(_, h387) => {
                                            h387
                                        }
                                    }, (v4, List::cons(Adt98::C0(f10(match v5 {
                                        Adt99::C0(h388, _) => {
                                            h388
                                        }
                                    })?, f10(match v6 {
                                        Adt99::C0(h389, _) => {
                                            h389
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Bytes, v1: Adt99, v2: Adt99, v3: (List<Adt98>, u64)) -> R<Result<Adt97, Adt96>> {
    let m395 = f25(v0.clone(), {
        let (_, h394) = v3.clone();
        h394
    })?;
    match m395 {
        None => {
            Ok(Err::<Adt97, Adt96>(Adt96::C0({
                let (_, h396) = v3;
                h396
            })))
        }
        Some(v4) => {
            let m398 = f47(v0.clone(), match v4.clone() {
                Adt99::C0(_, h397) => {
                    h397
                }
            })?;
            match m398 {
                None => {
                    Ok(Err::<Adt97, Adt96>(Adt96::C0(match v4 {
                        Adt99::C0(_, h399) => {
                            h399
                        }
                    })))
                }
                Some(v5) => {
                    f216(v0.clone(), Adt97::C0(f10(match v1 {
                        Adt99::C0(h400, _) => {
                            h400
                        }
                    })?, f10(match v2 {
                        Adt99::C0(h401, _) => {
                            h401
                        }
                    })?, {
                        let (h402, _) = v3;
                        h402
                    }, match v4.clone() {
                        Adt99::C0(h403, _) => {
                            h403
                        }
                    }, v5, List::<List<u64>>::nil(), Bytes::lit(&[]), Adt16::C0(0i64, Str::lit(""), Str::lit(""), f217())), f218(v0, {
                        let a405 = match v4 {
                            Adt99::C0(_, h404) => {
                                h404
                            }
                        };
                        let a406 = 4u64;
                        nat_add(a405, a406)?
                    })?)
                }
            }
        }
    }
}

pub fn f119(p412: List<Adt38>, p413: List<Str>) -> Option<Str> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v0, v1) = m411;
                let m407 = v0;
                match m407.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f219(match v2.clone() {
                            Adt38::C0(h408, _, _, _, _, _, _, _) => {
                                h408
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt38>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt38::C0(h409, _, _, _, _, _, _, _) => {
                                    h409
                                }
                            }))
                        } else {
                            Ok::<(List<Adt38>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt38::C0(h410, _, _, _, _, _, _, _) => {
                                    h410
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m411 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(p416: List<Adt12>, p417: List<Adt12>) -> List<Adt12> {
    let m415 = (p416, p417);
    {
        let mut m415 = m415;
        loop {
            let step = {
                let (v0, v1) = m415;
                let m414 = v0;
                match m414.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt38) -> R<Result<List<Adt12>, Adt101>> {
    let m420 = f220(match v0.clone() {
        Adt38::C0(h418, _, _, _, _, _, _, _) => {
            h418
        }
    }, f221(match v0.clone() {
        Adt38::C0(h419, _, _, _, _, _, _, _) => {
            h419
        }
    })?);
    match m420 {
        None => {
            f222(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt101>(v1))
        }
    }
}

pub fn f122(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f123() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f124(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f223(v0, 0i64)?, 0i64)))
}

pub fn f125(v0: List<(Str, Adt6)>) -> Adt5 {
    Adt5::C0(v0)
}

pub fn f126(v0: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::<(Str, Adt6)>::nil()))
}

pub fn f127(v0: Adt16, v1: Adt38, v2: Adt24, v3: Adt102, v4: Result<Option<Adt35>, Adt92>) -> R<Result<Adt35, Adt92>> {
    let m421 = v4;
    match m421 {
        Ok(v6) => {
            let m422 = v6;
            match m422 {
                None => {
                    f224(v0, v1, v2, v3)
                }
                Some(v7) => {
                    Ok(Ok::<Adt35, Adt92>(v7))
                }
            }
        }
        Err(v5) => {
            Ok(Err::<Adt35, Adt92>(v5))
        }
    }
}

pub fn f128(v0: Adt38, v1: Adt24) -> R<Result<Option<Adt35>, Adt92>> {
    let m429 = f225(Adt6::C5({
        let m425 = match v0.clone() {
            Adt38::C0(_, _, _, _, h424, _, _, _) => {
                h424
            }
        };
        match m425 {
            Adt5::C0(h423) => {
                h423
            }
        }
    }), Adt6::C5({
        let m428 = match v0 {
            Adt38::C0(_, _, _, _, h427, _, _, _) => {
                h427
            }
        };
        match m428 {
            Adt5::C0(h426) => {
                h426
            }
        }
    }), f226(v1))?;
    match m429 {
        Adt117::C0 => {
            Ok(Ok::<Option<Adt35>, Adt92>(None::<Adt35>))
        }
        Adt117::C1(v2, v3) => {
            Ok(Ok::<Option<Adt35>, Adt92>(Some(Adt35::C2(f227(f228(v2, v3)?, true)))))
        }
        Adt117::C2(_) => {
            Ok(Err::<Option<Adt35>, Adt92>(Adt92::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt117::C3(_) => {
            Ok(Err::<Option<Adt35>, Adt92>(Adt92::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt117::C4 => {
            Ok(Err::<Option<Adt35>, Adt92>(Adt92::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f129(v0: Str) -> Adt41 {
    Adt41::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>))
}

pub fn f130(v0: Adt24, v1: Str) -> Adt109 {
    let m431 = match v0 {
        Adt24::C0(_, _, h430, _, _) => {
            h430
        }
    };
    match m431 {
        None => {
            Adt109::C0
        }
        Some(v2) => {
            f229(v1, v2)
        }
    }
}

pub fn f131(v0: Adt6) -> Option<Str> {
    let m432 = v0;
    match m432 {
        Adt6::C0 => {
            None::<Str>
        }
        Adt6::C1(_) => {
            None::<Str>
        }
        Adt6::C2(_) => {
            None::<Str>
        }
        Adt6::C3(v3) => {
            Some(v3)
        }
        Adt6::C4(_) => {
            None::<Str>
        }
        Adt6::C5(_) => {
            None::<Str>
        }
    }
}

pub fn f132(v0: Adt24) -> i64 {
    let m433 = f130(v0, Str::lit("seconds"));
    match m433 {
        Adt109::C0 => {
            0i64
        }
        Adt109::C1(v1) => {
            let m434 = f230(v1);
            match m434 {
                None => {
                    0i64
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f133(v0: i64, v1: List<(Str, Adt67)>, v2: i64, v3: Adt89) -> Adt88 {
    Adt88::C0(v0, v1, v2, v3)
}

pub fn f134(v0: Str, v1: bool) -> R<Adt89> {
    let m437 = f231(f232({
        let m436 = f76(v0, v1);
        match m436 {
            Adt52::C0(h435, _) => {
                h435
            }
        }
    })?)?;
    match m437 {
        None => {
            Ok(f233())
        }
        Some(v2) => {
            Ok(Adt89::C0(Adt5::C0(v2)))
        }
    }
}

pub fn f135(v0: Str, v1: Str, v2: Str) -> Adt67 {
    Adt67::C0(Adt68::C0(Adt69::C0(Some(Adt70::C0), v0, f125(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v1.clone(), f126(v2)), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v1), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))))))
}

pub fn f136() -> Adt102 {
    Adt102::C2(Adt84::C0(List::cons((Str::lit("user_name"), f135(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))), List::<(Str, Adt67)>::nil())))
}

pub fn f137(v0: Adt5) -> Option<Str> {
    let m438 = f229(Str::lit("user_name"), v0);
    match m438 {
        Adt109::C0 => {
            None::<Str>
        }
        Adt109::C1(v1) => {
            let m439 = f234(v1);
            match m439 {
                None => {
                    None::<Str>
                }
                Some(v2) => {
                    let m440 = f229(Str::lit("content"), v2);
                    match m440 {
                        Adt109::C0 => {
                            None::<Str>
                        }
                        Adt109::C1(v3) => {
                            let m441 = f234(v3);
                            match m441 {
                                None => {
                                    None::<Str>
                                }
                                Some(v4) => {
                                    let m442 = f229(Str::lit("name"), v4);
                                    match m442 {
                                        Adt109::C0 => {
                                            None::<Str>
                                        }
                                        Adt109::C1(v5) => {
                                            f131(v5)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f138(v0: Str) -> R<Adt102> {
    let v1: Str = {
        let a443 = List::cons(Str::lit("Hello, "), List::cons(v0, List::cons(Str::lit("! (from a task)"), List::<Str>::nil())));
        let a444 = Str::lit("");
        join(a443, a444)
    };
    Ok(Adt102::C1(Adt103::C1, f76(v1.clone(), false), f133(0i64, List::<(Str, Adt67)>::nil(), 100i64, f134(v1, false)?)))
}

pub fn f139(v0: Adt38, v1: Adt24, v2: Fn5) -> R<Result<Adt35, Adt92>> {
    let m451 = f225(Adt6::C5({
        let m447 = match v0.clone() {
            Adt38::C0(_, _, _, _, h446, _, _, _) => {
                h446
            }
        };
        match m447 {
            Adt5::C0(h445) => {
                h445
            }
        }
    }), Adt6::C5({
        let m450 = match v0 {
            Adt38::C0(_, _, _, _, h449, _, _, _) => {
                h449
            }
        };
        match m450 {
            Adt5::C0(h448) => {
                h448
            }
        }
    }), f226(v1.clone()))?;
    match m451 {
        Adt117::C0 => {
            let c452: Fn5 = v2;
            c452.apply(v1)
        }
        Adt117::C1(v3, v4) => {
            Ok(Ok::<Adt35, Adt92>(Adt35::C2(f227(f228(v3, v4)?, true))))
        }
        Adt117::C2(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt117::C3(_) => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt117::C4 => {
            Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f140(v0: Fn3, v1: Adt38, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    f235(v1, {
        let c453: Fn3 = v0;
        c453.apply(v2)?
    })
}

pub fn f141(v0: Adt24, v1: Str) -> bool {
    let a455 = match v0 {
        Adt24::C0(_, h454, _, _, _) => {
            h454
        }
    };
    let a456 = v1;
    equal(a455, a456)
}

pub fn f142(v0: Adt24) -> Result<Adt105, Adt92> {
    let m458 = f177(Str::lit("user_name"), Adt106::C0, match v0 {
        Adt24::C0(_, _, _, h457, _) => {
            h457
        }
    });
    match m458 {
        Adt110::C0 => {
            f238(f180(Str::lit("user_name"), f239()))
        }
        Adt110::C1 => {
            f240()
        }
        Adt110::C2(v1) => {
            f236(f237(List::cons(Str::lit("Hello, "), List::cons(f179(v1, Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("!"), List::<Str>::nil())))))
        }
    }
}

pub fn f143(v0: Adt24) -> Result<Adt105, Adt92> {
    let m460 = f177(Str::lit("capital_question"), Adt106::C1, match v0 {
        Adt24::C0(_, _, _, h459, _) => {
            h459
        }
    });
    match m460 {
        Adt110::C0 => {
            f238(f180(Str::lit("capital_question"), f241(Str::lit("What is the capital of France?"), 100u64)))
        }
        Adt110::C1 => {
            f240()
        }
        Adt110::C2(v1) => {
            f236(f237(List::cons(Str::lit("Sampling result: "), List::cons(f179(v1, Str::lit("text"), Str::lit("no response")), List::<Str>::nil()))))
        }
    }
}

pub fn f144(v0: Adt24) -> R<Result<Adt105, Adt92>> {
    let m462 = f177(Str::lit("client_roots"), Adt106::C2, match v0 {
        Adt24::C0(_, _, _, h461, _) => {
            h461
        }
    });
    match m462 {
        Adt110::C0 => {
            Ok(f238(f180(Str::lit("client_roots"), f244())))
        }
        Adt110::C1 => {
            Ok(f240())
        }
        Adt110::C2(v1) => {
            Ok(f236(f237(List::cons(Str::lit("Found "), List::cons(f242(f243(v1))?, List::cons(Str::lit(" root(s)"), List::<Str>::nil()))))))
        }
    }
}

pub fn f145(v0: Adt24) -> Result<Adt105, Adt92> {
    f245(v0, Str::lit("request-state"), Str::lit("state-ok: requestState validated"))
}

pub fn f146(v0: Adt24) -> R<Result<Adt105, Adt92>> {
    if f246(v0.clone(), Str::lit("multiple-inputs")) {
        f247(f177(Str::lit("user_name"), Adt106::C0, match v0.clone() {
            Adt24::C0(_, _, _, h463, _) => {
                h463
            }
        }), f177(Str::lit("greeting"), Adt106::C1, match v0.clone() {
            Adt24::C0(_, _, _, h464, _) => {
                h464
            }
        }), f177(Str::lit("client_roots"), Adt106::C2, match v0 {
            Adt24::C0(_, _, _, h465, _) => {
                h465
            }
        }))
    } else {
        Ok(f248(f249(Adt110::C0, Adt110::C0, Adt110::C0), Str::lit("multiple-inputs")))
    }
}

pub fn f147(v0: Adt24) -> R<Result<Adt105, Adt92>> {
    let v1: u64 = f250(f251(v0.clone()))?;
    let a466 = v1;
    let a467 = 49u64;
    if nat_eq(a466, a467) {
        Ok(f252(v0))
    } else {
        let a468 = v1;
        let a469 = 50u64;
        if nat_eq(a468, a469) {
            Ok(f253(v0))
        } else {
            Ok(f248(f180(Str::lit("step1"), f254()), Str::lit("1")))
        }
    }
}

pub fn f148(v0: Adt24) -> Result<Adt105, Adt92> {
    f245(v0, Str::lit("tamper-test"), Str::lit("integrity-ok: state verified"))
}

pub fn f149(v0: Adt24) -> Result<Adt105, Adt92> {
    let m471 = match v0.clone() {
        Adt24::C0(_, _, _, h470, _) => {
            h470
        }
    };
    match m471 {
        None => {
            f255(f256({
                let m474 = match v0 {
                    Adt24::C0(h473, _, _, _, _) => {
                        h473
                    }
                };
                match m474 {
                    Adt19::C0(_, h472, _, _, _, _) => {
                        h472
                    }
                }
            }))
        }
        Some(v1) => {
            let m476 = match v1.clone() {
                Adt5::C0(h475) => {
                    h475
                }
            };
            match m476.uncons() {
                None => {
                    f255(f256({
                        let m479 = match v0 {
                            Adt24::C0(h478, _, _, _, _) => {
                                h478
                            }
                        };
                        match m479 {
                            Adt19::C0(_, h477, _, _, _, _) => {
                                h477
                            }
                        }
                    }))
                }
                Some((_, _)) => {
                    f236(f237(List::cons(Str::lit("capabilities-ok: received "), List::cons({
                        let a481 = f257(match v1 {
                            Adt5::C0(h480) => {
                                h480
                            }
                        });
                        let a482 = Str::lit(",");
                        join(a481, a482)
                    }, List::<Str>::nil()))))
                }
            }
        }
    }
}

pub fn f150(v0: Adt38, v1: Fn4, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    f139(v0.clone(), v2, Fn5::F258(v1, v0))
}

pub fn f151(v0: Adt24, v1: Str) -> bool {
    let a484 = match v0 {
        Adt24::C0(_, h483, _, _, _) => {
            h483
        }
    };
    let a485 = v1;
    equal(a484, a485)
}

pub fn f152(v0: List<Adt41>) -> Adt52 {
    f157(v0, None::<Adt6>, false, List::<Adt53>::nil())
}

pub fn f153(v0: Str) -> Adt41 {
    Adt41::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>))
}

pub fn f154() -> Adt41 {
    f163(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f155(v0: Str, v1: Str) -> Adt41 {
    Adt41::C2(Adt46::C0(v0, v1, None::<Adt43>, None::<Adt5>))
}

pub fn f156(v0: Str, v1: Str, v2: Str) -> Adt41 {
    Adt41::C4(Adt48::C0(Adt49::C0(Adt50::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt43>, None::<Adt5>))
}

pub fn f157(v0: List<Adt41>, v1: Option<Adt6>, v2: bool, v3: List<Adt53>) -> Adt52 {
    Adt52::C0(Adt40::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f158(v0: Adt24, v1: List<Adt53>) -> Adt52 {
    f157(List::cons(f153(f259({
        let m488 = match v0 {
            Adt24::C0(h487, _, _, _, _) => {
                h487
            }
        };
        match m488 {
            Adt19::C0(_, _, _, _, h486, _) => {
                h486
            }
        }
    })), List::<Adt41>::nil()), None::<Adt6>, false, v1)
}

pub fn f159() -> R<List<Adt53>> {
    Ok(List::cons(f260(0u64, 100u64)?, List::cons(f260(50u64, 100u64)?, List::cons(f260(100u64, 100u64)?, List::<Adt53>::nil()))))
}

pub fn f160() -> R<List<Adt53>> {
    Ok(List::cons(f260(50u64, 100u64)?, List::cons(f260(10u64, 100u64)?, List::<Adt53>::nil())))
}

pub fn f161() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f124(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f162() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f163(v0: Str, v1: Str) -> Adt41 {
    Adt41::C1(Adt45::C0(v0, v1, None::<Adt43>, None::<Adt5>))
}

pub fn f164() -> Str {
    let m490 = {
        let a489 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a489)
    };
    match m490 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f165(v0: Adt24) -> Result<Adt52, Adt92> {
    if f151(v0.clone(), Str::lit("test_missing_capability")) {
        let m496 = {
            let m495 = {
                let m494 = match v0 {
                    Adt24::C0(h493, _, _, _, _) => {
                        h493
                    }
                };
                match m494 {
                    Adt19::C0(_, h492, _, _, _, _) => {
                        h492
                    }
                }
            };
            match m495 {
                Adt20::C0(_, _, h491, _, _) => {
                    h491
                }
            }
        };
        match m496 {
            None => {
                Err::<Adt52, Adt92>(Adt92::C8(f261()))
            }
            Some(_) => {
                Ok::<Adt52, Adt92>(f262())
            }
        }
    } else {
        if f151(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt41>::nil())))
        } else {
            if f151(v0, Str::lit("test_arguments")) {
                Ok::<Adt52, Adt92>(f152(List::cons(f153(Str::lit("arguments accepted")), List::<Adt41>::nil())))
            } else {
                Ok::<Adt52, Adt92>(f262())
            }
        }
    }
}

pub fn f166(v0: Adt10, v1: List<Adt47>, v2: Adt23) -> Result<Adt35, Adt92> {
    let m498 = match v2 {
        Adt23::C0(_, h497) => {
            h497
        }
    };
    match m498 {
        None => {
            Ok::<Adt35, Adt92>(Adt35::C4(Adt54::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt35, Adt92>(Adt92::C5(v3))
        }
    }
}

pub fn f167() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C0)
}

pub fn f168() -> List<Adt47> {
    f263(f264())
}

pub fn f169(v0: Adt10, v1: Fn6, v2: Adt25) -> R<Result<Adt35, Adt92>> {
    Ok(f265(v0, match v2.clone() {
        Adt25::C0(_, h499, _, _) => {
            h499
        }
    }, {
        let c500: Fn6 = v1;
        c500.apply(match v2 {
            Adt25::C0(_, h501, _, _) => {
                h501
            }
        })?
    }))
}

pub fn f170() -> Adt10 {
    Adt10::C0(300000u64, Adt11::C1)
}

pub fn f171(v0: Str) -> R<Option<List<Adt49>>> {
    let m502 = f266(v0.clone(), f264());
    match m502 {
        None => {
            f267(v0)
        }
        Some(v1) => {
            Ok(Some(v1))
        }
    }
}

pub fn f172(v0: Adt10, v1: List<Adt57>, v2: Adt23) -> Result<Adt35, Adt92> {
    let m504 = match v2 {
        Adt23::C0(_, h503) => {
            h503
        }
    };
    match m504 {
        None => {
            Ok::<Adt35, Adt92>(Adt35::C6(Adt56::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt35, Adt92>(Adt92::C5(v3))
        }
    }
}

pub fn f173() -> List<Adt57> {
    List::cons(Adt57::C0(Str::lit("template"), Some(Str::lit("Resource Template")), None::<List<Adt2>>, f268(), Some(Str::lit("A resource template with parameter substitution")), Some(Str::lit("application/json")), None::<Adt43>, None::<Adt5>), List::<Adt57>::nil())
}

pub fn f174(v0: Adt10, v1: List<Adt59>, v2: Adt23) -> Result<Adt35, Adt92> {
    let m506 = match v2 {
        Adt23::C0(_, h505) => {
            h505
        }
    };
    match m506 {
        None => {
            Ok::<Adt35, Adt92>(Adt35::C7(Adt58::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt35, Adt92>(Adt92::C5(v3))
        }
    }
}

pub fn f175() -> List<Adt59> {
    List::cons(f269(Str::lit("test_simple_prompt"), Str::lit("Simple Test Prompt"), Str::lit("A simple prompt without arguments"), None::<List<Adt60>>), List::cons(f269(Str::lit("test_prompt_with_arguments"), Str::lit("Prompt With Arguments"), Str::lit("A prompt with required arguments"), Some(List::cons(f270(Str::lit("arg1"), Str::lit("First test argument")), List::cons(f270(Str::lit("arg2"), Str::lit("Second test argument")), List::<Adt60>::nil())))), List::cons(f269(Str::lit("test_prompt_with_embedded_resource"), Str::lit("Prompt With Embedded Resource"), Str::lit("A prompt that includes an embedded resource"), Some(List::cons(f270(Str::lit("resourceUri"), Str::lit("URI of the resource to embed")), List::<Adt60>::nil()))), List::cons(f269(Str::lit("test_prompt_with_image"), Str::lit("Prompt With Image"), Str::lit("A prompt that includes image content"), None::<List<Adt60>>), List::<Adt59>::nil()))))
}

pub fn f176() -> Str {
    Str::lit("test_input_required_result_prompt")
}

pub fn f177(v0: Str, v1: Adt106, v2: Option<Adt5>) -> Adt110 {
    let m507 = v2;
    match m507 {
        None => {
            Adt110::C0
        }
        Some(v3) => {
            let m508 = f229(v0, v3);
            match m508 {
                Adt109::C0 => {
                    Adt110::C0
                }
                Adt109::C1(v4) => {
                    let m509 = f234(v4);
                    match m509 {
                        None => {
                            Adt110::C1
                        }
                        Some(v5) => {
                            if f271(v1, v5.clone()) {
                                Adt110::C2(v5)
                            } else {
                                Adt110::C1
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f178(v0: Str) -> Adt61 {
    Adt61::C0(None::<Str>, List::cons(Adt62::C0(Adt44::C0, Adt41::C0(Adt42::C0(f237(List::cons(Str::lit("Prompt with context: "), List::cons(v0, List::<Str>::nil()))), None::<Adt43>, None::<Adt5>))), List::<Adt62>::nil()))
}

pub fn f179(v0: Adt5, v1: Str, v2: Str) -> Str {
    let m510 = f229(Str::lit("content"), v0);
    match m510 {
        Adt109::C0 => {
            v2
        }
        Adt109::C1(v3) => {
            let m511 = f234(v3);
            match m511 {
                None => {
                    v2
                }
                Some(v4) => {
                    f272(v1, v4, v2)
                }
            }
        }
    }
}

pub fn f180(v0: Str, v1: Adt67) -> List<(Str, Adt67)> {
    List::cons((v0, v1), List::<(Str, Adt67)>::nil())
}

pub fn f181(v0: Str, v1: Str, v2: Str) -> Adt67 {
    Adt67::C0(Adt68::C0(Adt69::C0(None::<Adt70>, v0, f273(v1, v2))))
}

pub fn f182(v0: List<Adt59>, v1: Fn7, v2: Adt26) -> Result<Adt35, Adt92> {
    let m513 = f274(match v2.clone() {
        Adt26::C0(_, h512, _, _, _) => {
            h512
        }
    }, v0);
    match m513 {
        None => {
            Err::<Adt35, Adt92>(Adt92::C1(match v2 {
                Adt26::C0(_, h514, _, _, _) => {
                    h514
                }
            }))
        }
        Some(v3) => {
            let m516 = f275(f276(v3), f277(match v2.clone() {
                Adt26::C0(_, _, h515, _, _) => {
                    h515
                }
            }));
            match m516 {
                None => {
                    Ok::<Adt35, Adt92>(Adt35::C8({
                        let c519: Fn7 = v1;
                        c519.apply(match v2.clone() {
                            Adt26::C0(_, h520, _, _, _) => {
                                h520
                            }
                        }, f277(match v2 {
                            Adt26::C0(_, _, h521, _, _) => {
                                h521
                            }
                        }))
                    }))
                }
                Some(v4) => {
                    Err::<Adt35, Adt92>(Adt92::C4({
                        let a517 = List::cons(Str::lit("missing required argument:"), List::cons(v4, List::<Str>::nil()));
                        let a518 = Str::lit(" ");
                        join(a517, a518)
                    }))
                }
            }
        }
    }
}

pub fn f183(v0: Str, v1: List<(Str, Str)>) -> Adt61 {
    let a522 = v0.clone();
    let a523 = Str::lit("test_simple_prompt");
    if equal(a522, a523) {
        f278(List::cons(f279(Str::lit("This is a simple prompt for testing.")), List::<Adt62>::nil()))
    } else {
        let a524 = v0.clone();
        let a525 = Str::lit("test_prompt_with_arguments");
        if equal(a524, a525) {
            f278(List::cons(f279({
                let a526 = List::cons(Str::lit("Prompt with arguments: arg1='"), List::cons(f280(Str::lit("arg1"), v1.clone()), List::cons(Str::lit("', arg2='"), List::cons(f280(Str::lit("arg2"), v1), List::cons(Str::lit("'"), List::<Str>::nil())))));
                let a527 = Str::lit("");
                join(a526, a527)
            }), List::<Adt62>::nil()))
        } else {
            let a528 = v0.clone();
            let a529 = Str::lit("test_prompt_with_embedded_resource");
            if equal(a528, a529) {
                f278(List::cons(Adt62::C0(Adt44::C0, Adt41::C4(Adt48::C0(f281(f280(Str::lit("resourceUri"), v1), Str::lit("text/plain"), Str::lit("Embedded resource content for testing.")), None::<Adt43>, None::<Adt5>))), List::cons(f279(Str::lit("Please process the embedded resource above.")), List::<Adt62>::nil())))
            } else {
                let a530 = v0;
                let a531 = Str::lit("test_prompt_with_image");
                if equal(a530, a531) {
                    f278(List::cons(Adt62::C0(Adt44::C0, Adt41::C1(Adt45::C0(f282(), Str::lit("image/png"), None::<Adt43>, None::<Adt5>))), List::cons(f279(Str::lit("Please analyze the image above.")), List::<Adt62>::nil())))
                } else {
                    f278(List::<Adt62>::nil())
                }
            }
        }
    }
}

pub fn f184(v0: Fn8, v1: Adt27) -> R<Result<Adt35, Adt92>> {
    let m533 = {
        let c532: Fn8 = v0;
        c532.apply(v1)?
    };
    match m533 {
        Ok(v3) => {
            Ok(Ok::<Adt35, Adt92>(Adt35::C9(f283(v3)?)))
        }
        Err(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f185(v0: Adt27) -> R<Result<List<Str>, Adt92>> {
    let m535 = match v0.clone() {
        Adt27::C0(_, h534, _, _) => {
            h534
        }
    };
    match m535 {
        Adt28::C0(v1) => {
            let m537 = f274(match v1.clone() {
                Adt29::C0(h536, _) => {
                    h536
                }
            }, f175());
            match m537 {
                None => {
                    Ok(Err::<List<Str>, Adt92>(Adt92::C1(match v1 {
                        Adt29::C0(h538, _) => {
                            h538
                        }
                    })))
                }
                Some(v2) => {
                    if f284(f276(v2), {
                        let m541 = match v0.clone() {
                            Adt27::C0(_, _, h540, _) => {
                                h540
                            }
                        };
                        match m541 {
                            Adt30::C0(h539, _) => {
                                h539
                            }
                        }
                    }) {
                        Ok(Ok::<List<Str>, Adt92>(f285({
                            let m544 = match v0.clone() {
                                Adt27::C0(_, _, h543, _) => {
                                    h543
                                }
                            };
                            match m544 {
                                Adt30::C0(_, h542) => {
                                    h542
                                }
                            }
                        }, f286(match v1 {
                            Adt29::C0(h545, _) => {
                                h545
                            }
                        }, {
                            let m548 = match v0 {
                                Adt27::C0(_, _, h547, _) => {
                                    h547
                                }
                            };
                            match m548 {
                                Adt30::C0(h546, _) => {
                                    h546
                                }
                            }
                        }))?))
                    } else {
                        Ok(Err::<List<Str>, Adt92>(Adt92::C4(Str::lit("the prompt declares no such argument"))))
                    }
                }
            }
        }
        Adt28::C1(v3) => {
            let a549 = v3.clone();
            let a550 = f268();
            if equal(a549, a550) {
                let a554 = {
                    let m553 = match v0.clone() {
                        Adt27::C0(_, _, h552, _) => {
                            h552
                        }
                    };
                    match m553 {
                        Adt30::C0(h551, _) => {
                            h551
                        }
                    }
                };
                let a555 = Str::lit("id");
                if equal(a554, a555) {
                    Ok(Ok::<List<Str>, Adt92>(f285({
                        let m558 = match v0 {
                            Adt27::C0(_, _, h557, _) => {
                                h557
                            }
                        };
                        match m558 {
                            Adt30::C0(_, h556) => {
                                h556
                            }
                        }
                    }, f287())?))
                } else {
                    Ok(Err::<List<Str>, Adt92>(Adt92::C4(Str::lit("the template declares no such argument"))))
                }
            } else {
                Ok(Err::<List<Str>, Adt92>(Adt92::C2(v3)))
            }
        }
    }
}

pub fn f186(v0: Adt20) -> bool {
    f288(match v0 {
        Adt20::C0(_, _, _, _, h559) => {
            h559
        }
    }, f33())
}

pub fn f187(v0: Bytes) -> R<Option<List<Adt87>>> {
    let a561 = {
        let a560 = v0.clone();
        length_bytes(a560)
    };
    let a562 = 0u64;
    if nat_eq(a561, a562) {
        Ok(Some(List::<Adt87>::nil()))
    } else {
        let m563 = f207(v0)?;
        match m563 {
            Ok(v2) => {
                let m564 = f289(v2);
                match m564 {
                    None => {
                        Ok(None::<List<Adt87>>)
                    }
                    Some(v3) => {
                        Ok(f290(v3, List::<Adt87>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt87>>)
            }
        }
    }
}

pub fn f188() -> Adt92 {
    Adt92::C9(Str::lit("the task table is not readable"))
}

pub fn f189(v0: Str, p569: List<Adt87>, v2: i64) -> R<Option<Adt87>> {
    let m568 = p569;
    {
        let mut m568 = m568;
        loop {
            let step = {
                let v1 = m568;
                let m565 = v1;
                match m565.uncons() {
                    None => {
                        Err::<List<Adt87>, R<Option<Adt87>>>(Ok(None::<Adt87>))
                    }
                    Some((v3, v4)) => {
                        let a566 = f291(v3.clone());
                        let a567 = v0.clone();
                        if equal(a566, a567) {
                            if f292(v3.clone(), v2)? {
                                Err::<List<Adt87>, R<Option<Adt87>>>(Ok(None::<Adt87>))
                            } else {
                                Err::<List<Adt87>, R<Option<Adt87>>>(Ok(Some(v3)))
                            }
                        } else {
                            Ok::<List<Adt87>, R<Option<Adt87>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190() -> Adt92 {
    Adt92::C8(f293())
}

pub fn f191(v0: Adt107, v1: Adt91, v2: bool) -> Adt35 {
    Adt35::C14(Adt90::C0(match v0.clone() {
        Adt107::C0(h570, _) => {
            h570
        }
    }, v1, if match v0 {
        Adt107::C0(_, h571) => {
            h571
        }
    } {
        true
    } else {
        v2
    }))
}

pub fn f192(v0: Adt87, v1: i64, v2: Str) -> R<Adt107> {
    let m573 = match v0.clone() {
        Adt87::C0(h572, _, _) => {
            h572
        }
    };
    match m573 {
        Adt65::C0(v3) => {
            f294(v0, v3, v1, v2)
        }
        Adt65::C1(_, _) => {
            Ok(f295(v0))
        }
        Adt65::C2(_, _) => {
            Ok(f295(v0))
        }
        Adt65::C3(_, _) => {
            Ok(f295(v0))
        }
        Adt65::C4(_) => {
            Ok(f295(v0))
        }
    }
}

pub fn f193(v0: Adt87, v1: Adt5, v2: i64, v3: Str) -> R<Adt107> {
    let m575 = match v0.clone() {
        Adt87::C0(h574, _, _) => {
            h574
        }
    };
    match m575 {
        Adt65::C0(_) => {
            Ok(f295(v0))
        }
        Adt65::C1(v5, v6) => {
            f296(v0, v5, v6, v1, v2, v3)
        }
        Adt65::C2(_, _) => {
            Ok(f295(v0))
        }
        Adt65::C3(_, _) => {
            Ok(f295(v0))
        }
        Adt65::C4(_) => {
            Ok(f295(v0))
        }
    }
}

pub fn f194(v0: Adt87, v1: i64, v2: Str) -> R<Adt107> {
    Ok(f297(f192(v0, v1, v2.clone())?, v2))
}

pub fn f195(p578: List<Adt15>, p579: List<Adt15>) -> List<Adt15> {
    let m577 = (p578, p579);
    {
        let mut m577 = m577;
        loop {
            let step = {
                let (v0, v1) = m577;
                let m576 = v0;
                match m576.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: bool, v1: Bytes, v2: List<Adt15>, v3: u64) -> Result<Adt95, Adt96> {
    if v0 {
        Ok::<Adt95, Adt96>(Adt95::C0(v1, v2))
    } else {
        Err::<Adt95, Adt96>(Adt96::C1(v3))
    }
}

pub fn f197(v0: Adt98) -> List<u64> {
    let a590 = f9({
        let a581 = match v0.clone() {
            Adt98::C0(h580, _) => {
                h580
            }
        };
        length_list(a581)
    });
    let a591 = {
        let a588 = match v0.clone() {
            Adt98::C0(h582, _) => {
                h582
            }
        };
        let a589 = {
            let a586 = f9({
                let a584 = match v0.clone() {
                    Adt98::C0(_, h583) => {
                        h583
                    }
                };
                length_list(a584)
            });
            let a587 = match v0 {
                Adt98::C0(_, h585) => {
                    h585
                }
            };
            append_list(a586, a587)
        };
        append_list(a588, a589)
    };
    append_list(a590, a591)
}

pub fn f198(v0: List<u64>) -> List<u64> {
    let a593 = f9({
        let a592 = v0.clone();
        length_list(a592)
    });
    let a594 = v0;
    append_list(a593, a594)
}

pub fn f199(v0: u64, v1: Adt6) -> R<Adt100> {
    let m595 = f298(v1)?;
    match m595 {
        Ok(v3) => {
            Ok(Adt100::C0(v0, f52(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f27())
        }
    }
}

pub fn f200(v0: Adt104) -> u64 {
    let m596 = v0;
    match m596 {
        Adt104::C0 => {
            f299()
        }
        Adt104::C1 => {
            f299()
        }
    }
}

pub fn f201(v0: Option<Adt108>, v1: Adt83) -> Adt6 {
    Adt6::C5(f300(v0, v1))
}

pub fn f202(v0: Adt104) -> Adt83 {
    Adt83::C0(f301(), f302(v0), None::<Adt6>)
}

pub fn f203(v0: List<u64>) -> bool {
    f303(v0, f304())
}

pub fn f204(v0: Adt16, v1: Adt0, v2: Fn9, v3: bool, v4: u64, v5: List<Adt98>, v6: Result<Adt6, Adt111>) -> R<Adt94> {
    let m597 = v6;
    match m597 {
        Ok(v8) => {
            f307(v0, v1, v2, v3, v4, v5, f308(v8)?)
        }
        Err(_) => {
            Ok(Adt94::C0(f26(f199(f305(), f201(None::<Adt108>, f306(Adt112::C0)))?)))
        }
    }
}

pub fn f205(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h598, _, _, _, _, _, _, _) => {
            h598
        }
    }, match v0.clone() {
        Adt0::C0(_, h599, _, _, _, _, _, _) => {
            h599
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h600, _, _, _, _, _) => {
            h600
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h601, _, _, _, _) => {
            h601
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h602, _, _, _) => {
            h602
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h603, _, _) => {
            h603
        }
    }, v1, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h604) => {
            h604
        }
    })
}

pub fn f206(v0: Fn1, v1: Adt97, v2: Adt17) -> R<Result<Adt35, Adt92>> {
    let c605: Fn1 = v0;
    c605.apply(match v1 {
        Adt97::C0(_, _, _, _, _, _, _, h606) => {
            h606
        }
    }, v2)
}

pub fn f207(v0: Bytes) -> R<Result<Adt6, Adt111>> {
    f309(f10(v0)?)
}

pub fn f208() -> Adt100 {
    Adt100::C0(f310(), List::cons(Adt98::C0(f311(), f312()), List::<Adt98>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f209(v0: List<Adt98>, v1: List<u64>) -> R<Adt114> {
    let m607 = f313(v0, v1, Adt115::C0)?;
    match m607 {
        Adt115::C0 => {
            Ok(Adt114::C0)
        }
        Adt115::C1(v2) => {
            let m608 = f314(v2);
            match m608 {
                None => {
                    Ok(Adt114::C1)
                }
                Some(v3) => {
                    Ok(Adt114::C3(v3))
                }
            }
        }
        Adt115::C2 => {
            Ok(Adt114::C2)
        }
    }
}

pub fn f210() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f211(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt104) -> R<Option<Adt104>> {
    let m609 = v0;
    match m609 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f315(v3, v1)? {
                Ok(None::<Adt104>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f212(v0: List<u64>) -> Option<List<u64>> {
    let m610 = f316(v0);
    match m610 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f214(v1)
        }
    }
}

pub fn f213() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f214(v0: List<u64>) -> Option<List<u64>> {
    let m611 = v0.clone();
    match m611.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a612 = v1;
            let a613 = 91u64;
            if nat_eq(a612, a613) {
                f317(f318(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f317(f319(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f215(p616: List<Adt98>, p617: List<Adt98>) -> List<Adt98> {
    let m615 = (p616, p617);
    {
        let mut m615 = m615;
        loop {
            let step = {
                let (v0, v1) = m615;
                let m614 = v0;
                match m614.uncons() {
                    None => {
                        Err::<(List<Adt98>, List<Adt98>), List<Adt98>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt98>, List<Adt98>), List<Adt98>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: Bytes, v1: Adt97, v2: Result<(Adt16, u64), Adt96>) -> R<Result<Adt97, Adt96>> {
    let m618 = v2;
    match m618 {
        Ok(v4) => {
            f320(v0, Adt97::C0(match v1.clone() {
                Adt97::C0(h619, _, _, _, _, _, _, _) => {
                    h619
                }
            }, match v1.clone() {
                Adt97::C0(_, h620, _, _, _, _, _, _) => {
                    h620
                }
            }, match v1.clone() {
                Adt97::C0(_, _, h621, _, _, _, _, _) => {
                    h621
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, h622, _, _, _, _) => {
                    h622
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, _, h623, _, _, _) => {
                    h623
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, _, _, h624, _, _) => {
                    h624
                }
            }, match v1 {
                Adt97::C0(_, _, _, _, _, _, h625, _) => {
                    h625
                }
            }, {
                let (h626, _) = v4;
                h626
            }), {
                let (_, h627) = v4;
                h627
            })
        }
        Err(v3) => {
            Ok(Err::<Adt97, Adt96>(v3))
        }
    }
}

pub fn f217() -> Bytes {
    Bytes::lit(&[])
}

pub fn f218(v0: Bytes, v1: u64) -> R<Result<(Adt16, u64), Adt96>> {
    let m628 = f321(v0.clone(), v1)?;
    match m628 {
        None => {
            Ok(Err::<(Adt16, u64), Adt96>(Adt96::C0(v1)))
        }
        Some(v2) => {
            let m631 = f322(v0.clone(), {
                let a629 = v1;
                let a630 = 8u64;
                nat_add(a629, a630)?
            })?;
            match m631 {
                Ok(v4) => {
                    f323(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt16, u64), Adt96>(v3))
                }
            }
        }
    }
}

pub fn f219(v0: Str, p636: List<Str>) -> bool {
    let m635 = p636;
    {
        let mut m635 = m635;
        loop {
            let step = {
                let v1 = m635;
                let m632 = v1;
                match m632.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a633 = v0.clone();
                        let a634 = v2;
                        if equal(a633, a634) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Str, v1: List<u64>) -> Option<Adt101> {
    if f324(v1) {
        Some(Adt101::C0(v0))
    } else {
        None::<Adt101>
    }
}

pub fn f221(v0: Str) -> R<List<u64>> {
    f10({
        let a637 = v0;
        utf8_encode(a637)
    })
}

pub fn f222(v0: Adt38) -> R<Result<List<Adt12>, Adt101>> {
    let m638 = f325(v0.clone());
    match m638 {
        None => {
            f326(v0.clone(), f327(f328(), match v0.clone() {
                Adt38::C0(h639, _, _, _, _, _, _, _) => {
                    h639
                }
            }, Adt6::C5({
                let m642 = match v0.clone() {
                    Adt38::C0(_, _, _, _, h641, _, _, _) => {
                        h641
                    }
                };
                match m642 {
                    Adt5::C0(h640) => {
                        h640
                    }
                }
            }), List::cons(Adt116::C0(Adt6::C5({
                let m645 = match v0 {
                    Adt38::C0(_, _, _, _, h644, _, _, _) => {
                        h644
                    }
                };
                match m645 {
                    Adt5::C0(h643) => {
                        h643
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt116>::nil()), f329())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt101>(v1))
        }
    }
}

pub fn f223(p650: u64, p651: i64) -> R<i64> {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v0, v1) = m649;
                let m646 = v0;
                if m646 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m646 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a647 = v1;
                        let a648 = -1i64;
                        int_sub(a647, a648)?
                    }))
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Adt16, v1: Adt38, v2: Adt24, v3: Adt102) -> R<Result<Adt35, Adt92>> {
    let m652 = v3;
    match m652 {
        Adt102::C0(v4) => {
            f330(v1, v4)
        }
        Adt102::C1(v6, v7, v8) => {
            if f331(v2) {
                f332(v0, v8)
            } else {
                f333(v6, v1, v7)
            }
        }
        Adt102::C2(v5) => {
            Ok(Ok::<Adt35, Adt92>(Adt35::C12(v5)))
        }
    }
}

pub fn f225(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt117> {
    f334(v0, f335(), Adt119::C0, List::cons(Adt120::C0(v1, v2, List::<Adt118>::nil()), List::<Adt120>::nil()))
}

pub fn f226(v0: Adt24) -> Adt6 {
    let m654 = match v0 {
        Adt24::C0(_, _, h653, _, _) => {
            h653
        }
    };
    match m654 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h655) => {
                    h655
                }
            })
        }
    }
}

pub fn f227(v0: Str, v1: bool) -> Adt40 {
    Adt40::C0(List::cons(Adt41::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>)), List::<Adt41>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f228(v0: List<Adt118>, v1: Str) -> R<Str> {
    let v2: Str = f336(v0)?;
    let a658 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a656 = v2.clone();
        let a657 = Str::lit("");
        if equal(a656, a657) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a659 = Str::lit(" ");
    Ok(join(a658, a659))
}

pub fn f229(v0: Str, v1: Adt5) -> Adt109 {
    f337(v0, match v1 {
        Adt5::C0(h660) => {
            h660
        }
    })
}

pub fn f230(v0: Adt6) -> Option<i64> {
    let m661 = v0;
    match m661 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f338(match v2.clone() {
                Adt7::C0(_, h662) => {
                    h662
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h663, _) => {
                        h663
                    }
                })
            } else {
                None::<i64>
            }
        }
        Adt6::C3(_) => {
            None::<i64>
        }
        Adt6::C4(_) => {
            None::<i64>
        }
        Adt6::C5(_) => {
            None::<i64>
        }
    }
}

pub fn f231(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m664 = v0;
    match m664 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m665 = f339(Adt6::C5(v1))?;
            match m665 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m666 = f234(v2);
                    match m666 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h667) => {
                                    h667
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f232(v0: Adt40) -> R<Option<List<(Str, Adt6)>>> {
    let m669 = f340(match v0.clone() {
        Adt40::C0(h668, _, _) => {
            h668
        }
    })?;
    match m669 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a672 = f341(Str::lit("structuredContent"), match v0.clone() {
                    Adt40::C0(_, h670, _) => {
                        h670
                    }
                });
                let a673 = f342(Str::lit("isError"), match v0 {
                    Adt40::C0(_, _, h671) => {
                        h671
                    }
                });
                append_list(a672, a673)
            })))
        }
    }
}

pub fn f233() -> Adt89 {
    Adt89::C1(Adt83::C0(-32603i64, Str::lit("Internal error"), None::<Adt6>))
}

pub fn f234(v0: Adt6) -> Option<Adt5> {
    let m674 = v0;
    match m674 {
        Adt6::C0 => {
            None::<Adt5>
        }
        Adt6::C1(_) => {
            None::<Adt5>
        }
        Adt6::C2(_) => {
            None::<Adt5>
        }
        Adt6::C3(_) => {
            None::<Adt5>
        }
        Adt6::C4(_) => {
            None::<Adt5>
        }
        Adt6::C5(v5) => {
            Some(Adt5::C0(v5))
        }
    }
}

pub fn f235(v0: Adt38, v1: Result<Adt105, Adt92>) -> R<Result<Adt35, Adt92>> {
    let m675 = v1;
    match m675 {
        Ok(v3) => {
            let m676 = v3;
            match m676 {
                Adt105::C0(v4) => {
                    f330(v0, v4)
                }
                Adt105::C1(v5) => {
                    Ok(Ok::<Adt35, Adt92>(Adt35::C12(v5)))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<Adt35, Adt92>(v2))
        }
    }
}

pub fn f236(v0: Str) -> Result<Adt105, Adt92> {
    Ok::<Adt105, Adt92>(Adt105::C0(Adt52::C0(Adt40::C0(List::cons(Adt41::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>)), List::<Adt41>::nil()), None::<Adt6>, None::<bool>), List::<Adt53>::nil())))
}

pub fn f237(v0: List<Str>) -> Str {
    let a677 = v0;
    let a678 = Str::lit("");
    join(a677, a678)
}

pub fn f238(v0: List<(Str, Adt67)>) -> Result<Adt105, Adt92> {
    Ok::<Adt105, Adt92>(Adt105::C1(Adt84::C0(v0)))
}

pub fn f239() -> Adt67 {
    f181(Str::lit("What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f240() -> Result<Adt105, Adt92> {
    Err::<Adt105, Adt92>(Adt92::C11)
}

pub fn f241(v0: Str, v1: u64) -> Adt67 {
    Adt67::C1(Adt72::C0(List::cons(f343(v0), List::<Adt73>::nil()), None::<Adt78>, None::<Str>, None::<Adt80>, None::<Adt7>, v1, None::<List<Str>>, None::<Adt5>, None::<List<Adt38>>, None::<Adt81>))
}

pub fn f242(v0: u64) -> R<Str> {
    let a679 = f344(v0, 0i64)?;
    Ok(format_i64(a679))
}

pub fn f243(v0: Adt5) -> u64 {
    let m680 = f229(Str::lit("roots"), v0);
    match m680 {
        Adt109::C0 => {
            0u64
        }
        Adt109::C1(v1) => {
            let m681 = f289(v1);
            match m681 {
                None => {
                    0u64
                }
                Some(v2) => {
                    let a682 = v2;
                    length_list(a682)
                }
            }
        }
    }
}

pub fn f244() -> Adt67 {
    Adt67::C2(Some(f32()))
}

pub fn f245(v0: Adt24, v1: Str, v2: Str) -> Result<Adt105, Adt92> {
    if f246(v0.clone(), v1.clone()) {
        let m684 = f177(Str::lit("confirm"), Adt106::C0, match v0 {
            Adt24::C0(_, _, _, h683, _) => {
                h683
            }
        });
        match m684 {
            Adt110::C0 => {
                f248(f180(Str::lit("confirm"), f346()), v1)
            }
            Adt110::C1 => {
                f240()
            }
            Adt110::C2(v3) => {
                if f345(v3) {
                    f236(v2)
                } else {
                    f248(f180(Str::lit("confirm"), f346()), v1)
                }
            }
        }
    } else {
        f248(f180(Str::lit("confirm"), f346()), v1)
    }
}

pub fn f246(v0: Adt24, v1: Str) -> bool {
    let m686 = match v0 {
        Adt24::C0(_, _, _, _, h685) => {
            h685
        }
    };
    match m686 {
        None => {
            false
        }
        Some(v2) => {
            let a687 = v2;
            let a688 = v1;
            equal(a687, a688)
        }
    }
}

pub fn f247(v0: Adt110, v1: Adt110, v2: Adt110) -> R<Result<Adt105, Adt92>> {
    if if f347(v0.clone()) {
        true
    } else {
        if f347(v1.clone()) {
            true
        } else {
            f347(v2.clone())
        }
    } {
        Ok(f240())
    } else {
        let m689 = f249(v0.clone(), v1.clone(), v2.clone());
        match m689.uncons() {
            None => {
                Ok(f236(f237(List::cons(Str::lit("Name: "), List::cons(f179(f348(v0), Str::lit("name"), Str::lit("unknown")), List::cons(Str::lit("; Greeting: "), List::cons(f179(f348(v1), Str::lit("text"), Str::lit("Hello there!")), List::cons(Str::lit("; Roots: "), List::cons(f242(f243(f348(v2)))?, List::<Str>::nil())))))))))
            }
            Some((_, _)) => {
                Ok(f248(f249(v0, v1, v2), Str::lit("multiple-inputs")))
            }
        }
    }
}

pub fn f248(v0: List<(Str, Adt67)>, v1: Str) -> Result<Adt105, Adt92> {
    Ok::<Adt105, Adt92>(Adt105::C1(Adt84::C2(v0, v1)))
}

pub fn f249(v0: Adt110, v1: Adt110, v2: Adt110) -> List<(Str, Adt67)> {
    let a692 = if f349(v0) {
        f180(Str::lit("user_name"), f239())
    } else {
        List::<(Str, Adt67)>::nil()
    };
    let a693 = {
        let a690 = if f349(v1) {
            f180(Str::lit("greeting"), f241(Str::lit("Generate a greeting"), 50u64))
        } else {
            List::<(Str, Adt67)>::nil()
        };
        let a691 = if f349(v2) {
            f180(Str::lit("client_roots"), f244())
        } else {
            List::<(Str, Adt67)>::nil()
        };
        append_list(a690, a691)
    };
    append_list(a692, a693)
}

pub fn f250(v0: Str) -> R<u64> {
    f45({
        let a694 = v0;
        utf8_encode(a694)
    }, 0u64)
}

pub fn f251(v0: Adt24) -> Str {
    let m696 = match v0 {
        Adt24::C0(_, _, _, _, h695) => {
            h695
        }
    };
    match m696 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f252(v0: Adt24) -> Result<Adt105, Adt92> {
    let m698 = f177(Str::lit("step1"), Adt106::C0, match v0 {
        Adt24::C0(_, _, _, h697, _) => {
            h697
        }
    });
    match m698 {
        Adt110::C0 => {
            f248(f180(Str::lit("step1"), f254()), Str::lit("1"))
        }
        Adt110::C1 => {
            f240()
        }
        Adt110::C2(v1) => {
            f248(f180(Str::lit("step2"), f350()), f237(List::cons(Str::lit("2"), List::cons(f179(v1, Str::lit("name"), Str::lit("unknown")), List::<Str>::nil()))))
        }
    }
}

pub fn f253(v0: Adt24) -> Result<Adt105, Adt92> {
    let m700 = f177(Str::lit("step2"), Adt106::C0, match v0.clone() {
        Adt24::C0(_, _, _, h699, _) => {
            h699
        }
    });
    match m700 {
        Adt110::C0 => {
            f248(f180(Str::lit("step2"), f350()), f251(v0))
        }
        Adt110::C1 => {
            f240()
        }
        Adt110::C2(v1) => {
            f236(f237(List::cons(Str::lit("Multi-round complete for "), List::cons(f351(f251(v0)), List::cons(Str::lit(" who likes "), List::cons(f179(v1, Str::lit("color"), Str::lit("unknown")), List::<Str>::nil()))))))
        }
    }
}

pub fn f254() -> Adt67 {
    f181(Str::lit("Step 1: What is your name?"), Str::lit("name"), Str::lit("string"))
}

pub fn f255(v0: List<(Str, Adt67)>) -> Result<Adt105, Adt92> {
    let m701 = v0.clone();
    match m701.uncons() {
        None => {
            f236(Str::lit("No supported capabilities declared"))
        }
        Some((_, _)) => {
            f248(v0, Str::lit("capabilities-test"))
        }
    }
}

pub fn f256(v0: Adt20) -> List<(Str, Adt67)> {
    let a704 = if f352(match v0.clone() {
        Adt20::C0(_, _, _, h702, _) => {
            h702
        }
    }) {
        f180(Str::lit("elicit_input"), f181(Str::lit("Elicitation input"), Str::lit("value"), Str::lit("string")))
    } else {
        List::<(Str, Adt67)>::nil()
    };
    let a705 = if f352(match v0 {
        Adt20::C0(_, _, h703, _, _) => {
            h703
        }
    }) {
        f180(Str::lit("sample_input"), f241(Str::lit("Sample request"), 50u64))
    } else {
        List::<(Str, Adt67)>::nil()
    };
    append_list(a704, a705)
}

pub fn f257(v0: List<(Str, Adt6)>) -> List<Str> {
    f353(f354(v0, List::<Str>::nil()), List::<Str>::nil())
}

pub fn f258(v0: Fn4, v1: Adt38, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    f355(v1, v0, v2)
}

pub fn f259(v0: Option<Adt22>) -> Str {
    let m706 = v0;
    match m706 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m707 = v1;
            match m707 {
                Adt22::C0(v2) => {
                    v2
                }
                Adt22::C1(v3) => {
                    let a708 = v3;
                    format_i64(a708)
                }
            }
        }
    }
}

pub fn f260(v0: u64, v1: u64) -> R<Adt53> {
    Ok(Adt53::C0(f356(v0)?, Some(f356(v1)?), Some({
        let a711 = List::cons(Str::lit("Completed step"), List::cons({
            let a709 = f344(v0, 0i64)?;
            format_i64(a709)
        }, List::cons(Str::lit("of"), List::cons({
            let a710 = f344(v1, 0i64)?;
            format_i64(a710)
        }, List::<Str>::nil()))));
        let a712 = Str::lit(" ");
        join(a711, a712)
    })))
}

pub fn f261() -> Adt20 {
    Adt20::C0(None::<Adt5>, None::<Adt5>, Some(f32()), None::<Adt5>, None::<Adt5>)
}

pub fn f262() -> Adt52 {
    f152(List::cons(f153(Str::lit("ok")), List::<Adt41>::nil()))
}

pub fn f263(v0: List<(Adt47, List<Adt49>)>) -> List<Adt47> {
    f357(f358(v0, List::<Adt47>::nil()), List::<Adt47>::nil())
}

pub fn f264() -> List<(Adt47, List<Adt49>)> {
    List::cons(f359(f360(Str::lit("static-text"), Some(Str::lit("Static Text Resource")), Str::lit("test://static-text"), Str::lit("A static text resource for testing"), Str::lit("text/plain")), f281(Str::lit("test://static-text"), Str::lit("text/plain"), Str::lit("This is the content of the static text resource."))), List::cons(f359(f360(Str::lit("static-binary"), Some(Str::lit("Static Binary Resource")), Str::lit("test://static-binary"), Str::lit("A static binary resource (image) for testing"), Str::lit("image/png")), f361(Str::lit("test://static-binary"), Str::lit("image/png"), f282())), List::cons(f359(f360(Str::lit("watched-resource"), Some(Str::lit("Watched Resource")), Str::lit("test://watched-resource"), Str::lit("A resource that auto-updates every 3 seconds"), Str::lit("text/plain")), f281(Str::lit("test://watched-resource"), Str::lit("text/plain"), Str::lit("Watched resource content"))), List::cons(f359(f360(Str::lit("Stateless Static Text"), None::<Str>, Str::lit("test://stateless-static-text"), Str::lit("A static text resource served on the draft path"), Str::lit("text/plain")), f281(Str::lit("test://stateless-static-text"), Str::lit("text/plain"), Str::lit("Static text content from the stateless draft path."))), List::<(Adt47, List<Adt49>)>::nil()))))
}

pub fn f265(v0: Adt10, v1: Str, v2: Option<List<Adt49>>) -> Result<Adt35, Adt92> {
    let m713 = v2;
    match m713 {
        None => {
            Err::<Adt35, Adt92>(Adt92::C2(v1))
        }
        Some(v3) => {
            let m714 = v3.clone();
            match m714.uncons() {
                None => {
                    Err::<Adt35, Adt92>(Adt92::C9(Str::lit("a listed resource has no contents")))
                }
                Some((_, _)) => {
                    Ok::<Adt35, Adt92>(Adt35::C5(Adt55::C0(v0, v3)))
                }
            }
        }
    }
}

pub fn f266(v0: Str, p723: List<(Adt47, List<Adt49>)>) -> Option<List<Adt49>> {
    let m722 = p723;
    {
        let mut m722 = m722;
        loop {
            let step = {
                let v1 = m722;
                let m715 = v1;
                match m715.uncons() {
                    None => {
                        Err::<List<(Adt47, List<Adt49>)>, Option<List<Adt49>>>(None::<List<Adt49>>)
                    }
                    Some((v2, v3)) => {
                        let a719 = {
                            let m718 = {
                                let (h717, _) = v2;
                                h717
                            };
                            match m718 {
                                Adt47::C0(_, _, _, h716, _, _, _, _, _) => {
                                    h716
                                }
                            }
                        };
                        let a720 = v0.clone();
                        if equal(a719, a720) {
                            Err::<List<(Adt47, List<Adt49>)>, Option<List<Adt49>>>(Some({
                                let (_, h721) = v2;
                                h721
                            }))
                        } else {
                            Ok::<List<(Adt47, List<Adt49>)>, Option<List<Adt49>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: Str) -> R<Option<List<Adt49>>> {
    let m724 = f362(v0.clone())?;
    match m724 {
        None => {
            Ok(None::<List<Adt49>>)
        }
        Some(v1) => {
            let m725 = f363(v1)?;
            match m725 {
                None => {
                    Ok(None::<List<Adt49>>)
                }
                Some(v2) => {
                    Ok(Some(List::cons(f281(v0, Str::lit("application/json"), v2), List::<Adt49>::nil())))
                }
            }
        }
    }
}

pub fn f268() -> Str {
    Str::lit("test://template/{id}/data")
}

pub fn f269(v0: Str, v1: Str, v2: Str, v3: Option<List<Adt60>>) -> Adt59 {
    Adt59::C0(v0, Some(v1), None::<List<Adt2>>, Some(v2), v3, None::<Adt5>)
}

pub fn f270(v0: Str, v1: Str) -> Adt60 {
    Adt60::C0(v0, None::<Str>, Some(v1), Some(true))
}

pub fn f271(v0: Adt106, v1: Adt5) -> bool {
    let m726 = v0;
    match m726 {
        Adt106::C0 => {
            f364(v1)
        }
        Adt106::C1 => {
            f365(v1)
        }
        Adt106::C2 => {
            f366(v1)
        }
    }
}

pub fn f272(v0: Str, v1: Adt5, v2: Str) -> Str {
    let m727 = f229(v0, v1);
    match m727 {
        Adt109::C0 => {
            v2
        }
        Adt109::C1(v3) => {
            let m728 = f131(v3);
            match m728 {
                None => {
                    v2
                }
                Some(v4) => {
                    v4
                }
            }
        }
    }
}

pub fn f273(v0: Str, v1: Str) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((v0.clone(), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(v0), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))))
}

pub fn f274(v0: Str, p734: List<Adt59>) -> Option<Adt59> {
    let m733 = p734;
    {
        let mut m733 = m733;
        loop {
            let step = {
                let v1 = m733;
                let m729 = v1;
                match m729.uncons() {
                    None => {
                        Err::<List<Adt59>, Option<Adt59>>(None::<Adt59>)
                    }
                    Some((v2, v3)) => {
                        let a731 = match v2.clone() {
                            Adt59::C0(h730, _, _, _, _, _) => {
                                h730
                            }
                        };
                        let a732 = v0.clone();
                        if equal(a731, a732) {
                            Err::<List<Adt59>, Option<Adt59>>(Some(v2))
                        } else {
                            Ok::<List<Adt59>, Option<Adt59>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m733 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(p740: List<Adt60>, v1: List<(Str, Str)>) -> Option<Str> {
    let m739 = p740;
    {
        let mut m739 = m739;
        loop {
            let step = {
                let v0 = m739;
                let m735 = v0;
                match m735.uncons() {
                    None => {
                        Err::<List<Adt60>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f367(v2.clone()) {
                            let m737 = f368(match v2.clone() {
                                Adt60::C0(h736, _, _, _) => {
                                    h736
                                }
                            }, v1.clone());
                            match m737 {
                                None => {
                                    Err::<List<Adt60>, Option<Str>>(Some(match v2 {
                                        Adt60::C0(h738, _, _, _) => {
                                            h738
                                        }
                                    }))
                                }
                                Some(_) => {
                                    Ok::<List<Adt60>, Option<Str>>(v3)
                                }
                            }
                        } else {
                            Ok::<List<Adt60>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Adt59) -> List<Adt60> {
    let m742 = match v0 {
        Adt59::C0(_, _, _, _, h741, _) => {
            h741
        }
    };
    match m742 {
        None => {
            List::<Adt60>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f277(v0: Option<List<(Str, Str)>>) -> List<(Str, Str)> {
    let m743 = v0;
    match m743 {
        None => {
            List::<(Str, Str)>::nil()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f278(v0: List<Adt62>) -> Adt61 {
    Adt61::C0(None::<Str>, v0)
}

pub fn f279(v0: Str) -> Adt62 {
    Adt62::C0(Adt44::C0, Adt41::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>)))
}

pub fn f280(v0: Str, v1: List<(Str, Str)>) -> Str {
    let m744 = f368(v0, v1);
    match m744 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f281(v0: Str, v1: Str, v2: Str) -> Adt49 {
    Adt49::C0(Adt50::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f282() -> Str {
    Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg==")
}

pub fn f283(v0: List<Str>) -> R<Adt63> {
    let v1: u64 = f369(v0.clone(), 0u64)?;
    Ok(Adt63::C0(Adt64::C0(f370(v0), Some(v1), Some({
        let a745 = f371();
        let a746 = v1;
        nat_lt(a745, a746)
    }))))
}

pub fn f284(p752: List<Adt60>, v1: Str) -> bool {
    let m751 = p752;
    {
        let mut m751 = m751;
        loop {
            let step = {
                let v0 = m751;
                let m747 = v0;
                match m747.uncons() {
                    None => {
                        Err::<List<Adt60>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a749 = match v2 {
                            Adt60::C0(h748, _, _, _) => {
                                h748
                            }
                        };
                        let a750 = v1.clone();
                        if equal(a749, a750) {
                            Err::<List<Adt60>, bool>(true)
                        } else {
                            Ok::<List<Adt60>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Str, v1: List<Str>) -> R<List<Str>> {
    Ok(f353(f372(v0, v1, List::<Str>::nil())?, List::<Str>::nil()))
}

pub fn f286(v0: Str, v1: Str) -> List<Str> {
    let a753 = v0;
    let a754 = Str::lit("test_prompt_with_embedded_resource");
    if if equal(a753, a754) {
        let a755 = v1;
        let a756 = Str::lit("resourceUri");
        equal(a755, a756)
    } else {
        false
    } {
        f373()
    } else {
        List::<Str>::nil()
    }
}

pub fn f287() -> List<Str> {
    List::cons(Str::lit("123"), List::cons(Str::lit("124"), List::cons(Str::lit("456"), List::<Str>::nil())))
}

pub fn f288(v0: Option<Adt5>, v1: Str) -> bool {
    let m757 = v0;
    match m757 {
        None => {
            false
        }
        Some(v2) => {
            f374(v1, v2)
        }
    }
}

pub fn f289(v0: Adt6) -> Option<List<Adt6>> {
    let m758 = v0;
    match m758 {
        Adt6::C0 => {
            None::<List<Adt6>>
        }
        Adt6::C1(_) => {
            None::<List<Adt6>>
        }
        Adt6::C2(_) => {
            None::<List<Adt6>>
        }
        Adt6::C3(_) => {
            None::<List<Adt6>>
        }
        Adt6::C4(v4) => {
            Some(v4)
        }
        Adt6::C5(_) => {
            None::<List<Adt6>>
        }
    }
}

pub fn f290(p762: List<Adt6>, p763: List<Adt87>) -> Option<List<Adt87>> {
    let m761 = (p762, p763);
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v0, v1) = m761;
                let m759 = v0;
                match m759.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt87>), Option<List<Adt87>>>(Some(f375(v1, List::<Adt87>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m760 = f376(v2);
                        match m760 {
                            None => {
                                Err::<(List<Adt6>, List<Adt87>), Option<List<Adt87>>>(None::<List<Adt87>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt6>, List<Adt87>), Option<List<Adt87>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Adt87) -> Str {
    let m766 = f377(match v0 {
        Adt87::C0(h765, _, _) => {
            h765
        }
    });
    match m766 {
        Adt66::C0(h764, _, _, _, _, _) => {
            h764
        }
    }
}

pub fn f292(v0: Adt87, v1: i64) -> R<bool> {
    let m770 = {
        let m769 = f377(match v0.clone() {
            Adt87::C0(h768, _, _) => {
                h768
            }
        });
        match m769 {
            Adt66::C0(_, _, _, _, h767, _) => {
                h767
            }
        }
    };
    match m770 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a777 = {
                let m776 = {
                    let a774 = {
                        let a772 = v1;
                        let a773 = match v0 {
                            Adt87::C0(_, h771, _) => {
                                h771
                            }
                        };
                        int_sub(a772, a773)?
                    };
                    let a775 = v2;
                    compare(a774, a775)
                };
                match m776 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                }
            };
            Ok(bool_not(a777))
        }
    }
}

pub fn f293() -> Adt20 {
    Adt20::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f33(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f294(v0: Adt87, v1: Adt66, v2: i64, v3: Str) -> R<Adt107> {
    let m781 = {
        let m780 = match v0.clone() {
            Adt87::C0(_, _, h779) => {
                h779
            }
        };
        match m780 {
            Adt88::C0(_, h778, _, _) => {
                h778
            }
        }
    };
    match m781.uncons() {
        None => {
            if f378({
                let m784 = match v0.clone() {
                    Adt87::C0(_, _, h783) => {
                        h783
                    }
                };
                match m784 {
                    Adt88::C0(_, _, h782, _) => {
                        h782
                    }
                }
            }, match v0.clone() {
                Adt87::C0(_, h785, _) => {
                    h785
                }
            }, v2)? {
                Ok(f379(v0.clone(), f380({
                    let m788 = match v0.clone() {
                        Adt87::C0(_, _, h787) => {
                            h787
                        }
                    };
                    match m788 {
                        Adt88::C0(_, _, _, h786) => {
                            h786
                        }
                    }
                }, f381(v1, v3)), match v0 {
                    Adt87::C0(_, _, h789) => {
                        h789
                    }
                }))
            } else {
                Ok(f295(v0))
            }
        }
        Some((_, _)) => {
            if f378({
                let m792 = match v0.clone() {
                    Adt87::C0(_, _, h791) => {
                        h791
                    }
                };
                match m792 {
                    Adt88::C0(h790, _, _, _) => {
                        h790
                    }
                }
            }, match v0.clone() {
                Adt87::C0(_, h793, _) => {
                    h793
                }
            }, v2)? {
                Ok(f379(v0.clone(), Adt65::C1(f381(v1, v3), {
                    let m796 = match v0.clone() {
                        Adt87::C0(_, _, h795) => {
                            h795
                        }
                    };
                    match m796 {
                        Adt88::C0(_, h794, _, _) => {
                            h794
                        }
                    }
                }), f382(match v0 {
                    Adt87::C0(_, _, h797) => {
                        h797
                    }
                })))
            } else {
                Ok(f295(v0))
            }
        }
    }
}

pub fn f295(v0: Adt87) -> Adt107 {
    Adt107::C0(v0, false)
}

pub fn f296(v0: Adt87, v1: Adt66, v2: List<(Str, Adt67)>, v3: Adt5, v4: i64, v5: Str) -> R<Adt107> {
    let m798 = f383(v2.clone(), v3);
    match m798.uncons() {
        None => {
            f384(v0, v1, v4, v5)
        }
        Some((v6, v7)) => {
            let a801 = {
                let a799 = List::cons(v6.clone(), v7.clone());
                length_list(a799)
            };
            let a802 = {
                let a800 = v2;
                length_list(a800)
            };
            if nat_eq(a801, a802) {
                Ok(f295(v0))
            } else {
                Ok(f379(v0.clone(), Adt65::C1(f381(v1, v5), List::cons(v6, v7)), match v0 {
                    Adt87::C0(_, _, h803) => {
                        h803
                    }
                }))
            }
        }
    }
}

pub fn f297(v0: Adt107, v1: Str) -> Adt107 {
    let m807 = {
        let m806 = match v0.clone() {
            Adt107::C0(h805, _) => {
                h805
            }
        };
        match m806 {
            Adt87::C0(h804, _, _) => {
                h804
            }
        }
    };
    match m807 {
        Adt65::C0(v2) => {
            f379(match v0.clone() {
                Adt107::C0(h808, _) => {
                    h808
                }
            }, Adt65::C4(f381(v2, v1)), {
                let m811 = match v0 {
                    Adt107::C0(h810, _) => {
                        h810
                    }
                };
                match m811 {
                    Adt87::C0(_, _, h809) => {
                        h809
                    }
                }
            })
        }
        Adt65::C1(v3, _) => {
            f379(match v0.clone() {
                Adt107::C0(h812, _) => {
                    h812
                }
            }, Adt65::C4(f381(v3, v1)), {
                let m815 = match v0 {
                    Adt107::C0(h814, _) => {
                        h814
                    }
                };
                match m815 {
                    Adt87::C0(_, _, h813) => {
                        h813
                    }
                }
            })
        }
        Adt65::C2(_, _) => {
            v0
        }
        Adt65::C3(_, _) => {
            v0
        }
        Adt65::C4(_) => {
            v0
        }
    }
}

pub fn f298(v0: Adt6) -> R<Result<List<u64>, Adt111>> {
    f385(f386(), v0)
}

pub fn f299() -> u64 {
    403u64
}

pub fn f300(v0: Option<Adt108>, v1: Adt83) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f387())), {
        let a816 = f388(v0);
        let a817 = List::cons((Str::lit("error"), f389(v1)), List::<(Str, Adt6)>::nil());
        append_list(a816, a817)
    })
}

pub fn f301() -> i64 {
    -32000i64
}

pub fn f302(v0: Adt104) -> Str {
    let m818 = v0;
    match m818 {
        Adt104::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt104::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f303(p825: List<u64>, p826: List<u64>) -> bool {
    let m824 = (p825, p826);
    {
        let mut m824 = m824;
        loop {
            let step = {
                let (v0, v1) = m824;
                let m819 = v0;
                match m819.uncons() {
                    None => {
                        let m820 = v1;
                        match m820.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m821 = v1;
                        match m821.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a822 = v4;
                                let a823 = v6;
                                if nat_eq(a822, a823) {
                                    Ok::<(List<u64>, List<u64>), bool>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f305() -> u64 {
    400u64
}

pub fn f306(v0: Adt112) -> Adt83 {
    Adt83::C0(f390(v0.clone()), f391(v0), None::<Adt6>)
}

pub fn f307(v0: Adt16, v1: Adt0, v2: Fn9, v3: bool, v4: u64, v5: List<Adt98>, v6: Result<Adt122, Adt112>) -> R<Adt94> {
    let m827 = v6;
    match m827 {
        Ok(v8) => {
            f394(v0, v1, v2, v3, v4, v5, v8)
        }
        Err(v7) => {
            Ok(Adt94::C0(f26(f199(f392(v7.clone()), f201(f393(v7.clone()), f306(v7)))?)))
        }
    }
}

pub fn f308(v0: Adt6) -> R<Result<Adt122, Adt112>> {
    let m828 = v0;
    match m828 {
        Adt6::C0 => {
            Ok(f396(None::<Adt108>, Adt113::C0))
        }
        Adt6::C1(_) => {
            Ok(f396(None::<Adt108>, Adt113::C0))
        }
        Adt6::C2(_) => {
            Ok(f396(None::<Adt108>, Adt113::C0))
        }
        Adt6::C3(_) => {
            Ok(f396(None::<Adt108>, Adt113::C0))
        }
        Adt6::C4(_) => {
            Ok(f396(None::<Adt108>, Adt113::C1))
        }
        Adt6::C5(v1) => {
            f395(Adt5::C0(v1))
        }
    }
}

pub fn f309(v0: List<u64>) -> R<Result<Adt6, Adt111>> {
    let m829 = f397(v0.clone())?;
    match m829 {
        Adt133::C0 => {
            f398(f399(), 0u64, v0)
        }
        Adt133::C1(v1) => {
            Ok(Err::<Adt6, Adt111>(Adt111::C0(v1)))
        }
        Adt133::C2(v2) => {
            Ok(Err::<Adt6, Adt111>(Adt111::C1(v2)))
        }
    }
}

pub fn f310() -> u64 {
    405u64
}

pub fn f311() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f312() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f313(p834: List<Adt98>, v1: List<u64>, p835: Adt115) -> R<Adt115> {
    let m833 = (p834, p835);
    {
        let mut m833 = m833;
        loop {
            let step = {
                let (v0, v2) = m833;
                let m830 = v0;
                match m830.uncons() {
                    None => {
                        Err::<(List<Adt98>, Adt115), R<Adt115>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f400(match v3.clone() {
                            Adt98::C0(h831, _) => {
                                h831
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt98>, Adt115), R<Adt115>>((v4, f401(v2, match v3 {
                                Adt98::C0(_, h832) => {
                                    h832
                                }
                            })))
                        } else {
                            Ok::<(List<Adt98>, Adt115), R<Adt115>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f314(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f402(v0);
    if f403(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f315(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f400(v0.clone(), f404())? {
        Ok(true)
    } else {
        if f400(v0.clone(), f405())? {
            Ok(true)
        } else {
            if f400(v0.clone(), f406())? {
                Ok(true)
            } else {
                f407(v0, v1)
            }
        }
    }
}

pub fn f316(p840: List<u64>) -> Option<List<u64>> {
    let m839 = p840;
    {
        let mut m839 = m839;
        loop {
            let step = {
                let v0 = m839;
                let m836 = v0;
                match m836.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a837 = v1;
                        let a838 = 58u64;
                        if nat_eq(a837, a838) {
                            Err::<List<u64>, Option<List<u64>>>(f408(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m841 = v0;
    match m841 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f409({
                let (_, h842) = v1;
                h842
            }) {
                Some({
                    let (h843, _) = v1;
                    h843
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f318(p848: List<u64>, p849: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m847 = (p848, p849);
    {
        let mut m847 = m847;
        loop {
            let step = {
                let (v0, v1) = m847;
                let m844 = v0;
                match m844.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a845 = v2;
                        let a846 = 93u64;
                        if nat_eq(a845, a846) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f410(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(p854: List<u64>, p855: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, v1) = m853;
                let m850 = v0;
                match m850.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f410(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a851 = v2;
                        let a852 = 58u64;
                        if nat_eq(a851, a852) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f410(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f320(v0: Bytes, v1: Adt97, v2: u64) -> R<Result<Adt97, Adt96>> {
    let m856 = f411(v0.clone(), v2)?;
    match m856 {
        None => {
            Ok(Err::<Adt97, Adt96>(Adt96::C0(v2)))
        }
        Some(v3) => {
            f412(v0, Adt97::C0(match v1.clone() {
                Adt97::C0(h857, _, _, _, _, _, _, _) => {
                    h857
                }
            }, match v1.clone() {
                Adt97::C0(_, h858, _, _, _, _, _, _) => {
                    h858
                }
            }, match v1.clone() {
                Adt97::C0(_, _, h859, _, _, _, _, _) => {
                    h859
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, h860, _, _, _, _) => {
                    h860
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, _, h861, _, _, _) => {
                    h861
                }
            }, match v1.clone() {
                Adt97::C0(_, _, _, _, _, h862, _, _) => {
                    h862
                }
            }, match v3.clone() {
                Adt99::C0(h863, _) => {
                    h863
                }
            }, match v1 {
                Adt97::C0(_, _, _, _, _, _, _, h864) => {
                    h864
                }
            }), match v3 {
                Adt99::C0(_, h865) => {
                    h865
                }
            })
        }
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a869 = {
        let a866 = v1;
        let a867 = 8u64;
        nat_add(a866, a867)?
    };
    let a870 = {
        let a868 = v0.clone();
        length_bytes(a868)
    };
    if nat_le(a869, a870) {
        Ok(Some(f413(f413(f413(f413(f413(f413(f413(f413(0i64, f45(v0.clone(), v1)?)?, f45(v0.clone(), {
            let a871 = v1;
            let a872 = 1u64;
            nat_add(a871, a872)?
        })?)?, f45(v0.clone(), {
            let a873 = v1;
            let a874 = 2u64;
            nat_add(a873, a874)?
        })?)?, f45(v0.clone(), {
            let a875 = v1;
            let a876 = 3u64;
            nat_add(a875, a876)?
        })?)?, f45(v0.clone(), {
            let a877 = v1;
            let a878 = 4u64;
            nat_add(a877, a878)?
        })?)?, f45(v0.clone(), {
            let a879 = v1;
            let a880 = 5u64;
            nat_add(a879, a880)?
        })?)?, f45(v0.clone(), {
            let a881 = v1;
            let a882 = 6u64;
            nat_add(a881, a882)?
        })?)?, f45(v0, {
            let a883 = v1;
            let a884 = 7u64;
            nat_add(a883, a884)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt96>> {
    let m885 = f25(v0, v1)?;
    match m885 {
        None => {
            Ok(Err::<(Str, u64), Adt96>(Adt96::C0(v1)))
        }
        Some(v2) => {
            let m887 = f414(f10(match v2.clone() {
                Adt99::C0(h886, _) => {
                    h886
                }
            })?);
            match m887 {
                None => {
                    Ok(Err::<(Str, u64), Adt96>(Adt96::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt96>((v3, match v2 {
                        Adt99::C0(_, h888) => {
                            h888
                        }
                    })))
                }
            }
        }
    }
}

pub fn f323(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt16, u64), Adt96>> {
    let m890 = f322(v0.clone(), {
        let (_, h889) = v2;
        h889
    })?;
    match m890 {
        Ok(v4) => {
            f415(v0, v1, {
                let (h891, _) = v2;
                h891
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt16, u64), Adt96>(v3))
        }
    }
}

pub fn f324(v0: List<u64>) -> bool {
    let m892 = v0.clone();
    match m892.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a896 = if f416(v0.clone(), false) {
                let a894 = {
                    let a893 = v0;
                    length_list(a893)
                };
                let a895 = f417();
                nat_le(a894, a895)
            } else {
                false
            };
            bool_not(a896)
        }
    }
}

pub fn f325(v0: Adt38) -> Option<Adt101> {
    let m901 = f418(match v0.clone() {
        Adt38::C0(h897, _, _, _, _, _, _, _) => {
            h897
        }
    }, {
        let m900 = match v0.clone() {
            Adt38::C0(_, _, _, _, h899, _, _, _) => {
                h899
            }
        };
        match m900 {
            Adt5::C0(h898) => {
                h898
            }
        }
    });
    match m901 {
        None => {
            f419(match v0.clone() {
                Adt38::C0(h902, _, _, _, _, _, _, _) => {
                    h902
                }
            }, {
                let m905 = match v0 {
                    Adt38::C0(_, _, _, _, h904, _, _, _) => {
                        h904
                    }
                };
                match m905 {
                    Adt5::C0(h903) => {
                        h903
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f326(v0: Adt38, v1: Result<Adt123, Adt101>) -> R<Result<List<Adt12>, Adt101>> {
    let m906 = v1;
    match m906 {
        Ok(v3) => {
            let m907 = f420(v0)?;
            match m907 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt101>(f120(match v3 {
                        Adt123::C0(h908, _) => {
                            h908
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt101>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt101>(v2))
        }
    }
}

pub fn f327(p915: u64, v1: Str, v2: Adt6, p916: List<Adt116>, p917: Adt123) -> R<Result<Adt123, Adt101>> {
    let m914 = (p915, (p916, p917));
    {
        let mut m914 = m914;
        loop {
            let step = {
                let (v0, (v3, v4)) = m914;
                let m909 = v0;
                if m909 == 0 {
                    Err::<(u64, (List<Adt116>, Adt123)), R<Result<Adt123, Adt101>>>(Ok(Err::<Adt123, Adt101>(Adt101::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m909 - 1;
                    let m910 = v3;
                    match m910.uncons() {
                        None => {
                            Err::<(u64, (List<Adt116>, Adt123)), R<Result<Adt123, Adt101>>>(Ok(Ok::<Adt123, Adt101>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m911 = f421(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m911 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt116>, Adt123)), R<Result<Adt123, Adt101>>>((v5, ({
                                        let (h912, _) = v9;
                                        h912
                                    }, {
                                        let (_, h913) = v9;
                                        h913
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt116>, Adt123)), R<Result<Adt123, Adt101>>>(Ok(Err::<Adt123, Adt101>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328() -> u64 {
    67108864u64
}

pub fn f329() -> Adt123 {
    Adt123::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f330(v0: Adt38, v1: Adt52) -> R<Result<Adt35, Adt92>> {
    if f422({
        let m920 = match v1.clone() {
            Adt52::C0(h919, _) => {
                h919
            }
        };
        match m920 {
            Adt40::C0(h918, _, _) => {
                h918
            }
        }
    })? {
        f423(v0, v1)
    } else {
        Ok(Err::<Adt35, Adt92>(Adt92::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f331(v0: Adt24) -> bool {
    f186({
        let m923 = match v0 {
            Adt24::C0(h922, _, _, _, _) => {
                h922
            }
        };
        match m923 {
            Adt19::C0(_, h921, _, _, _, _) => {
                h921
            }
        }
    })
}

pub fn f332(v0: Adt16, v1: Adt88) -> R<Result<Adt35, Adt92>> {
    let a925 = match v0.clone() {
        Adt16::C0(_, _, h924, _) => {
            h924
        }
    };
    let a926 = Str::lit("");
    if equal(a925, a926) {
        Ok(Err::<Adt35, Adt92>(f424()))
    } else {
        let m928 = f187(match v0.clone() {
            Adt16::C0(_, _, _, h927) => {
                h927
            }
        })?;
        match m928 {
            None => {
                Ok(Err::<Adt35, Adt92>(f188()))
            }
            Some(v2) => {
                let m931 = f189(match v0.clone() {
                    Adt16::C0(_, _, h929, _) => {
                        h929
                    }
                }, v2, match v0.clone() {
                    Adt16::C0(h930, _, _, _) => {
                        h930
                    }
                })?;
                match m931 {
                    None => {
                        Ok(Ok::<Adt35, Adt92>(Adt35::C13(Adt85::C0(Adt86::C0, f377({
                            let m933 = f426(v0.clone(), v1.clone());
                            match m933 {
                                Adt87::C0(h932, _, _) => {
                                    h932
                                }
                            }
                        }), f426(v0, v1)))))
                    }
                    Some(_) => {
                        Ok(Err::<Adt35, Adt92>(f425()))
                    }
                }
            }
        }
    }
}

pub fn f333(v0: Adt103, v1: Adt38, v2: Adt52) -> R<Result<Adt35, Adt92>> {
    let m934 = v0;
    match m934 {
        Adt103::C0 => {
            f330(v1, v2)
        }
        Adt103::C1 => {
            Ok(Err::<Adt35, Adt92>(f190()))
        }
    }
}

pub fn f334(v0: Adt6, p953: u64, p954: Adt119, p955: List<Adt120>) -> R<Adt117> {
    let m952 = (p953, (p954, p955));
    {
        let mut m952 = m952;
        loop {
            let step = {
                let (v1, (v2, v3)) = m952;
                let m935 = v1;
                if m935 == 0 {
                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C4))
                } else {
                    let v4 = m935 - 1;
                    let m936 = v3;
                    match m936.uncons() {
                        None => {
                            let m937 = v2;
                            match m937 {
                                Adt119::C0 => {
                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C0))
                                }
                                Adt119::C1(v5, v6) => {
                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m938 = v2.clone();
                            match m938 {
                                Adt119::C0 => {
                                    let m939 = v7.clone();
                                    match m939 {
                                        Adt120::C0(v9, v10, v11) => {
                                            let m940 = f427(v0.clone(), v9, v10, v11.clone())?;
                                            match m940 {
                                                Adt141::C0(v12) => {
                                                    Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (Adt119::C1(v11, v12), v8)))
                                                }
                                                Adt141::C1(v13) => {
                                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C2(v13)))
                                                }
                                                Adt141::C2(v14) => {
                                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C3(v14)))
                                                }
                                                Adt141::C3(v15) => {
                                                    let v16: List<Adt120> = {
                                                        let a941 = v15;
                                                        let a942 = v8;
                                                        append_list(a941, a942)
                                                    };
                                                    Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt120::C1(_, _, _) => {
                                            Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (v2, v8)))
                                        }
                                        Adt120::C2(_, _, _, _) => {
                                            let v24: Adt124 = f428(v7, v8)?;
                                            Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (match v24.clone() {
                                                Adt124::C0(h943, _) => {
                                                    h943
                                                }
                                            }, match v24 {
                                                Adt124::C0(_, h944) => {
                                                    h944
                                                }
                                            })))
                                        }
                                        Adt120::C3(v25) => {
                                            Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (Adt119::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt120::C4(_, _, _, _) => {
                                            let v39: Adt124 = f428(v7, v8)?;
                                            Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (match v39.clone() {
                                                Adt124::C0(h948, _) => {
                                                    h948
                                                }
                                            }, match v39 {
                                                Adt124::C0(_, h949) => {
                                                    h949
                                                }
                                            })))
                                        }
                                        Adt120::C5(v26, v27, v28, v29) => {
                                            let m945 = f429(v26, v27, v28, v29.clone());
                                            match m945 {
                                                Adt141::C0(v30) => {
                                                    Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (Adt119::C1(v29, v30), v8)))
                                                }
                                                Adt141::C1(v31) => {
                                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C2(v31)))
                                                }
                                                Adt141::C2(v32) => {
                                                    Err::<(u64, (Adt119, List<Adt120>)), R<Adt117>>(Ok(Adt117::C3(v32)))
                                                }
                                                Adt141::C3(v33) => {
                                                    let v34: List<Adt120> = {
                                                        let a946 = v33;
                                                        let a947 = v8;
                                                        append_list(a946, a947)
                                                    };
                                                    Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt119::C1(v40, v41) => {
                                    let v42: Adt124 = f430(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt119, List<Adt120>)), R<Adt117>>((v4, (match v42.clone() {
                                        Adt124::C0(h950, _) => {
                                            h950
                                        }
                                    }, match v42 {
                                        Adt124::C0(_, h951) => {
                                            h951
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m952 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335() -> u64 {
    67108864u64
}

pub fn f336(v0: List<Adt118>) -> R<Str> {
    let a956 = List::cons(Str::lit(""), f431(v0, List::<Str>::nil())?);
    let a957 = Str::lit("/");
    Ok(join(a956, a957))
}

pub fn f337(v0: Str, p963: List<(Str, Adt6)>) -> Adt109 {
    let m962 = p963;
    {
        let mut m962 = m962;
        loop {
            let step = {
                let v1 = m962;
                let m958 = v1;
                match m958.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt109>(Adt109::C0)
                    }
                    Some((v2, v3)) => {
                        let m960 = f432({
                            let (h959, _) = v2;
                            h959
                        }, v0.clone());
                        match m960 {
                            Adt142::C0 => {
                                Err::<List<(Str, Adt6)>, Adt109>(Adt109::C1({
                                    let (_, h961) = v2;
                                    h961
                                }))
                            }
                            Adt142::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt109>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m962 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: i64) -> bool {
    let a970 = {
        let m966 = {
            let a964 = v0;
            let a965 = 0i64;
            compare(a964, a965)
        };
        if match m966 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m969 = {
                let a967 = 0i64;
                let a968 = v0;
                compare(a967, a968)
            };
            match m969 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            }
        }
    };
    bool_not(a970)
}

pub fn f339(v0: Adt6) -> R<Option<Adt6>> {
    f433(f434(), Adt125::C0(v0), List::<Adt126>::nil())
}

pub fn f340(v0: List<Adt41>) -> R<Option<List<Adt6>>> {
    let m971 = f435(v0, List::<Adt6>::nil())?;
    match m971 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f436(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f341(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m972 = v1;
    match m972 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f342(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m973 = v1;
    match m973 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f343(v0: Str) -> Adt73 {
    Adt73::C0(Adt44::C0, Adt74::C0(Adt75::C0(Adt42::C0(v0, None::<Adt43>, None::<Adt5>))), None::<Adt5>)
}

pub fn f344(p976: u64, p977: i64) -> R<i64> {
    let m975 = (p976, p977);
    {
        let mut m975 = m975;
        loop {
            let step = {
                let (v0, v1) = m975;
                let m974 = v0;
                if m974 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m974 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f437(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m975 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: Adt5) -> bool {
    let m978 = f229(Str::lit("content"), v0);
    match m978 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(v1) => {
            let m979 = f234(v1);
            match m979 {
                None => {
                    false
                }
                Some(v2) => {
                    let m980 = f229(Str::lit("ok"), v2);
                    match m980 {
                        Adt109::C0 => {
                            false
                        }
                        Adt109::C1(v3) => {
                            let m981 = f438(v3);
                            match m981 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    v4
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f346() -> Adt67 {
    f181(Str::lit("Please confirm"), Str::lit("ok"), Str::lit("boolean"))
}

pub fn f347(v0: Adt110) -> bool {
    let m982 = v0;
    match m982 {
        Adt110::C0 => {
            false
        }
        Adt110::C1 => {
            true
        }
        Adt110::C2(_) => {
            false
        }
    }
}

pub fn f348(v0: Adt110) -> Adt5 {
    let m983 = v0;
    match m983 {
        Adt110::C0 => {
            f32()
        }
        Adt110::C1 => {
            f32()
        }
        Adt110::C2(v1) => {
            v1
        }
    }
}

pub fn f349(v0: Adt110) -> bool {
    let m984 = v0;
    match m984 {
        Adt110::C0 => {
            true
        }
        Adt110::C1 => {
            false
        }
        Adt110::C2(_) => {
            false
        }
    }
}

pub fn f350() -> Adt67 {
    f181(Str::lit("Step 2: What is your favorite color?"), Str::lit("color"), Str::lit("string"))
}

pub fn f351(v0: Str) -> Str {
    let v1: Bytes = {
        let a985 = v0;
        utf8_encode(a985)
    };
    let m992 = {
        let a989 = v1.clone();
        let a990 = 1u64;
        let a991 = {
            let a987 = {
                let a986 = v1;
                length_bytes(a986)
            };
            let a988 = 1u64;
            nat_sub(a987, a988)
        };
        slice_bytes(a989, a990, a991)
    };
    match m992 {
        None => {
            Str::lit("")
        }
        Some(v2) => {
            let m994 = {
                let a993 = v2;
                utf8_decode(a993)
            };
            match m994 {
                None => {
                    Str::lit("")
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f352(v0: Option<Adt5>) -> bool {
    let m995 = v0;
    match m995 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f353(p998: List<Str>, p999: List<Str>) -> List<Str> {
    let m997 = (p998, p999);
    {
        let mut m997 = m997;
        loop {
            let step = {
                let (v0, v1) = m997;
                let m996 = v0;
                match m996.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(p1003: List<(Str, Adt6)>, p1004: List<Str>) -> List<Str> {
    let m1002 = (p1003, p1004);
    {
        let mut m1002 = m1002;
        loop {
            let step = {
                let (v0, v1) = m1002;
                let m1000 = v0;
                match m1000.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Str>), List<Str>>((v3, List::cons({
                            let (h1001, _) = v2;
                            h1001
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Adt38, v1: Fn4, v2: Adt24) -> R<Result<Adt35, Adt92>> {
    let m1006 = {
        let c1005: Fn4 = v1;
        c1005.apply(v2)?
    };
    match m1006 {
        Ok(v4) => {
            f330(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt35, Adt92>(v3))
        }
    }
}

pub fn f356(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f344(v0, 0i64)?, 0i64))
}

pub fn f357(p1009: List<Adt47>, p1010: List<Adt47>) -> List<Adt47> {
    let m1008 = (p1009, p1010);
    {
        let mut m1008 = m1008;
        loop {
            let step = {
                let (v0, v1) = m1008;
                let m1007 = v0;
                match m1007.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt47>), List<Adt47>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt47>), List<Adt47>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1008 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(p1014: List<(Adt47, List<Adt49>)>, p1015: List<Adt47>) -> List<Adt47> {
    let m1013 = (p1014, p1015);
    {
        let mut m1013 = m1013;
        loop {
            let step = {
                let (v0, v1) = m1013;
                let m1011 = v0;
                match m1011.uncons() {
                    None => {
                        Err::<(List<(Adt47, List<Adt49>)>, List<Adt47>), List<Adt47>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Adt47, List<Adt49>)>, List<Adt47>), List<Adt47>>((v3, List::cons({
                            let (h1012, _) = v2;
                            h1012
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Adt47, v1: Adt49) -> (Adt47, List<Adt49>) {
    (v0, List::cons(v1, List::<Adt49>::nil()))
}

pub fn f360(v0: Str, v1: Option<Str>, v2: Str, v3: Str, v4: Str) -> Adt47 {
    Adt47::C0(v0, v1, None::<List<Adt2>>, v2, Some(v3), Some(v4), None::<Adt43>, None::<u64>, None::<Adt5>)
}

pub fn f361(v0: Str, v1: Str, v2: Str) -> Adt49 {
    Adt49::C1(Adt51::C0(v0, Some(v1), v2, None::<Adt5>))
}

pub fn f362(v0: Str) -> R<Option<Str>> {
    let m1016 = f439(v0)?;
    match m1016 {
        None => {
            Ok(None::<Str>)
        }
        Some(v1) => {
            let a1017 = f440(f217(), v1);
            Ok(utf8_decode(a1017))
        }
    }
}

pub fn f363(v0: Str) -> R<Option<Str>> {
    let m1020 = f441(Adt6::C5(List::cons((Str::lit("id"), Adt6::C3(v0.clone())), List::cons((Str::lit("templateTest"), Adt6::C1(true)), List::cons((Str::lit("data"), Adt6::C3({
        let a1018 = List::cons(Str::lit("Data for ID:"), List::cons(v0, List::<Str>::nil()));
        let a1019 = Str::lit(" ");
        join(a1018, a1019)
    })), List::<(Str, Adt6)>::nil())))))?;
    match m1020 {
        Ok(v1) => {
            let a1021 = v1;
            Ok(utf8_decode(a1021))
        }
        Err(_) => {
            Ok(None::<Str>)
        }
    }
}

pub fn f364(v0: Adt5) -> bool {
    if f442(v0.clone()) {
        f443(v0)
    } else {
        false
    }
}

pub fn f365(v0: Adt5) -> bool {
    if f444(v0.clone()) {
        if f445(f229(Str::lit("model"), v0.clone())) {
            f446(v0)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f366(v0: Adt5) -> bool {
    let m1022 = f229(Str::lit("roots"), v0);
    match m1022 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(v1) => {
            let m1023 = f289(v1);
            match m1023 {
                None => {
                    false
                }
                Some(v2) => {
                    f447(v2)
                }
            }
        }
    }
}

pub fn f367(v0: Adt60) -> bool {
    let m1025 = match v0 {
        Adt60::C0(_, _, _, h1024) => {
            h1024
        }
    };
    match m1025 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f368(v0: Str, p1032: List<(Str, Str)>) -> Option<Str> {
    let m1031 = p1032;
    {
        let mut m1031 = m1031;
        loop {
            let step = {
                let v1 = m1031;
                let m1026 = v1;
                match m1026.uncons() {
                    None => {
                        Err::<List<(Str, Str)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let a1028 = {
                            let (h1027, _) = v2;
                            h1027
                        };
                        let a1029 = v0.clone();
                        if equal(a1028, a1029) {
                            Err::<List<(Str, Str)>, Option<Str>>(Some({
                                let (_, h1030) = v2;
                                h1030
                            }))
                        } else {
                            Ok::<List<(Str, Str)>, Option<Str>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1031 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f369(p1037: List<Str>, p1038: u64) -> R<u64> {
    let m1036 = (p1037, p1038);
    {
        let mut m1036 = m1036;
        loop {
            let step = {
                let (v0, v1) = m1036;
                let m1033 = v0;
                match m1033.uncons() {
                    None => {
                        Err::<(List<Str>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Str>, u64), R<u64>>((v3, {
                            let a1034 = v1;
                            let a1035 = 1u64;
                            nat_add(a1034, a1035)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1036 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370(v0: List<Str>) -> List<Str> {
    f353(f448(v0, f371(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f371() -> u64 {
    100u64
}

pub fn f372(v0: Str, p1041: List<Str>, p1042: List<Str>) -> R<List<Str>> {
    let m1040 = (p1041, p1042);
    {
        let mut m1040 = m1040;
        loop {
            let step = {
                let (v1, v2) = m1040;
                let m1039 = v1;
                match m1039.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), R<List<Str>>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f449(v0.clone(), v3.clone())? {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, List::cons(v3, v2)))
                        } else {
                            Ok::<(List<Str>, List<Str>), R<List<Str>>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373() -> List<Str> {
    f353(f450(f168(), List::<Str>::nil()), List::<Str>::nil())
}

pub fn f374(v0: Str, v1: Adt5) -> bool {
    f451(f229(v0, v1))
}

pub fn f375(p1045: List<Adt87>, p1046: List<Adt87>) -> List<Adt87> {
    let m1044 = (p1045, p1046);
    {
        let mut m1044 = m1044;
        loop {
            let step = {
                let (v0, v1) = m1044;
                let m1043 = v0;
                match m1043.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<Adt87>), List<Adt87>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt87>, List<Adt87>), List<Adt87>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1044 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376(v0: Adt6) -> Option<Adt87> {
    let m1047 = f234(v0);
    match m1047 {
        None => {
            None::<Adt87>
        }
        Some(v1) => {
            let m1048 = f452(Str::lit("task"), v1.clone());
            match m1048 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m1049 = f453(v2);
                    match m1049 {
                        None => {
                            None::<Adt87>
                        }
                        Some(v3) => {
                            let m1050 = f454(Str::lit("createdMs"), v1.clone());
                            match m1050 {
                                None => {
                                    None::<Adt87>
                                }
                                Some(v4) => {
                                    let m1051 = f452(Str::lit("plan"), v1);
                                    match m1051 {
                                        None => {
                                            None::<Adt87>
                                        }
                                        Some(v5) => {
                                            let m1052 = f455(v5);
                                            match m1052 {
                                                None => {
                                                    None::<Adt87>
                                                }
                                                Some(v6) => {
                                                    Some(Adt87::C0(v3, v4, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: Adt65) -> Adt66 {
    let m1053 = v0;
    match m1053 {
        Adt65::C0(v1) => {
            v1
        }
        Adt65::C1(v2, _) => {
            v2
        }
        Adt65::C2(v4, _) => {
            v4
        }
        Adt65::C3(v6, _) => {
            v6
        }
        Adt65::C4(v8) => {
            v8
        }
    }
}

pub fn f378(v0: i64, v1: i64, v2: i64) -> R<bool> {
    let a1059 = {
        let m1058 = {
            let a1056 = {
                let a1054 = v2;
                let a1055 = v1;
                int_sub(a1054, a1055)?
            };
            let a1057 = v0;
            compare(a1056, a1057)
        };
        match m1058 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        }
    };
    Ok(bool_not(a1059))
}

pub fn f379(v0: Adt87, v1: Adt65, v2: Adt88) -> Adt107 {
    Adt107::C0(Adt87::C0(v1, match v0 {
        Adt87::C0(_, h1060, _) => {
            h1060
        }
    }, v2), true)
}

pub fn f380(v0: Adt89, v1: Adt66) -> Adt65 {
    let m1061 = v0;
    match m1061 {
        Adt89::C0(v2) => {
            Adt65::C2(v1, v2)
        }
        Adt89::C1(v3) => {
            Adt65::C3(v1, v3)
        }
    }
}

pub fn f381(v0: Adt66, v1: Str) -> Adt66 {
    Adt66::C0(match v0.clone() {
        Adt66::C0(h1062, _, _, _, _, _) => {
            h1062
        }
    }, match v0.clone() {
        Adt66::C0(_, h1063, _, _, _, _) => {
            h1063
        }
    }, match v0.clone() {
        Adt66::C0(_, _, h1064, _, _, _) => {
            h1064
        }
    }, v1, match v0.clone() {
        Adt66::C0(_, _, _, _, h1065, _) => {
            h1065
        }
    }, match v0 {
        Adt66::C0(_, _, _, _, _, h1066) => {
            h1066
        }
    })
}

pub fn f382(v0: Adt88) -> Adt88 {
    Adt88::C0(match v0.clone() {
        Adt88::C0(h1067, _, _, _) => {
            h1067
        }
    }, List::<(Str, Adt67)>::nil(), match v0.clone() {
        Adt88::C0(_, _, h1068, _) => {
            h1068
        }
    }, match v0 {
        Adt88::C0(_, _, _, h1069) => {
            h1069
        }
    })
}

pub fn f383(v0: List<(Str, Adt67)>, v1: Adt5) -> List<(Str, Adt67)> {
    f456(f457(v0, v1, List::<(Str, Adt67)>::nil()), List::<(Str, Adt67)>::nil())
}

pub fn f384(v0: Adt87, v1: Adt66, v2: i64, v3: Str) -> R<Adt107> {
    let v4: Adt87 = Adt87::C0(Adt65::C0(f381(v1, v3.clone())), match v0.clone() {
        Adt87::C0(_, h1070, _) => {
            h1070
        }
    }, match v0 {
        Adt87::C0(_, _, h1071) => {
            h1071
        }
    });
    Ok(Adt107::C0({
        let m1073 = f192(v4, v2, v3)?;
        match m1073 {
            Adt107::C0(h1072, _) => {
                h1072
            }
        }
    }, true))
}

pub fn f385(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt111>> {
    let m1074 = f458(v0, List::cons(Adt127::C1(0u64, v1), List::<Adt127>::nil()), List::<u64>::nil())?;
    match m1074 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt111>(f459(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt111>(v3))
        }
    }
}

pub fn f386() -> u64 {
    67108864u64
}

pub fn f387() -> Str {
    Str::lit("2.0")
}

pub fn f388(v0: Option<Adt108>) -> List<(Str, Adt6)> {
    let m1075 = v0;
    match m1075 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f460(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f389(v0: Adt83) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt83::C0(h1076, _, _) => {
            h1076
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt83::C0(_, h1077, _) => {
            h1077
        }
    })), f461(match v0 {
        Adt83::C0(_, _, h1078) => {
            h1078
        }
    }))))
}

pub fn f390(v0: Adt112) -> i64 {
    let m1079 = v0;
    match m1079 {
        Adt112::C0 => {
            f462()
        }
        Adt112::C1(_, _) => {
            f463()
        }
    }
}

pub fn f391(v0: Adt112) -> Str {
    let m1080 = v0;
    match m1080 {
        Adt112::C0 => {
            Str::lit("Parse error")
        }
        Adt112::C1(_, v2) => {
            f464(v2)
        }
    }
}

pub fn f392(v0: Adt112) -> u64 {
    let m1081 = v0;
    match m1081 {
        Adt112::C0 => {
            f305()
        }
        Adt112::C1(_, _) => {
            f305()
        }
    }
}

pub fn f393(v0: Adt112) -> Option<Adt108> {
    let m1082 = v0;
    match m1082 {
        Adt112::C0 => {
            None::<Adt108>
        }
        Adt112::C1(v1, _) => {
            v1
        }
    }
}

pub fn f394(v0: Adt16, v1: Adt0, v2: Fn9, v3: bool, v4: u64, v5: List<Adt98>, v6: Adt122) -> R<Adt94> {
    f465(v1.clone(), v3, v0, f466(v5.clone())?, f467(v1.clone(), v2, v4, Some(f468(v5, match v1 {
        Adt0::C0(_, _, _, _, h1083, _, _, _) => {
            h1083
        }
    }, f469(v6.clone()), match v6.clone() {
        Adt122::C0(_, h1084, _) => {
            h1084
        }
    })?), f470(), v6)?)
}

pub fn f395(v0: Adt5) -> R<Result<Adt122, Adt112>> {
    let m1085 = f471(v0.clone())?;
    match m1085 {
        Adt150::C0 => {
            let m1086 = f229(Str::lit("method"), v0.clone());
            match m1086 {
                Adt109::C0 => {
                    Ok(f396(None::<Adt108>, f472(v0)))
                }
                Adt109::C1(_) => {
                    Ok(f396(None::<Adt108>, if f473(v0) {
                        Adt113::C8
                    } else {
                        Adt113::C7
                    }))
                }
            }
        }
        Adt150::C1(v2) => {
            Ok(f474(v2, v0))
        }
        Adt150::C2 => {
            Ok(f396(None::<Adt108>, Adt113::C5))
        }
    }
}

pub fn f396(v0: Option<Adt108>, v1: Adt113) -> Result<Adt122, Adt112> {
    Err::<Adt122, Adt112>(Adt112::C1(v0, v1))
}

pub fn f397(v0: List<u64>) -> R<Adt133> {
    Ok(f475({
        let a1087 = v0.clone();
        length_list(a1087)
    }, f476(Adt134::C0, 0u64, 0u64, v0)?))
}

pub fn f398(p1094: Adt135, p1095: u64, p1096: List<u64>) -> R<Result<Adt6, Adt111>> {
    let m1093 = (p1094, (p1095, p1096));
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1093;
                let m1088 = v2;
                match m1088.uncons() {
                    None => {
                        let m1089 = f477(v0, v1)?;
                        match m1089 {
                            Ok(v3) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt6, Adt111>>>(Ok(Ok::<Adt6, Adt111>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt6, Adt111>>>(Ok(Err::<Adt6, Adt111>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m1090 = f478(v0, v1, v5)?;
                        match m1090 {
                            Adt151::C0(v7) => {
                                Err::<(Adt135, (u64, List<u64>)), R<Result<Adt6, Adt111>>>(Ok(Err::<Adt6, Adt111>(v7)))
                            }
                            Adt151::C1(v8) => {
                                Ok::<(Adt135, (u64, List<u64>)), R<Result<Adt6, Adt111>>>((v8, ({
                                    let a1091 = v1;
                                    let a1092 = 1u64;
                                    nat_add(a1091, a1092)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1093 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399() -> Adt135 {
    Adt135::C0(Adt136::C0, List::<Adt140>::nil(), 0u64)
}

pub fn f400(p1103: List<u64>, p1104: List<u64>) -> R<bool> {
    let m1102 = (p1103, p1104);
    {
        let mut m1102 = m1102;
        loop {
            let step = {
                let (v0, v1) = m1102;
                let m1097 = v0;
                match m1097.uncons() {
                    None => {
                        let m1098 = v1;
                        match m1098.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1099 = v1;
                        match m1099.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1100 = f479(v4)?;
                                let a1101 = f479(v6)?;
                                if nat_eq(a1100, a1101) {
                                    Ok::<(List<u64>, List<u64>), R<bool>>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: Adt115, v1: List<u64>) -> Adt115 {
    let m1105 = v0;
    match m1105 {
        Adt115::C0 => {
            Adt115::C1(v1)
        }
        Adt115::C1(_) => {
            Adt115::C2
        }
        Adt115::C2 => {
            Adt115::C2
        }
    }
}

pub fn f402(v0: List<u64>) -> List<u64> {
    f410(f480(f410(f480(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f403(p1108: List<u64>) -> bool {
    let m1107 = p1108;
    {
        let mut m1107 = m1107;
        loop {
            let step = {
                let v0 = m1107;
                let m1106 = v0;
                match m1106.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f481(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f405() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f406() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f407(v0: List<u64>, p1111: List<List<u64>>) -> R<bool> {
    let m1110 = p1111;
    {
        let mut m1110 = m1110;
        loop {
            let step = {
                let v1 = m1110;
                let m1109 = v1;
                match m1109.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f400(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1110 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: List<u64>) -> Option<List<u64>> {
    let m1112 = v0;
    match m1112.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m1113 = v2;
            match m1113.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a1114 = v1;
                    let a1115 = 47u64;
                    if if nat_eq(a1114, a1115) {
                        let a1116 = v3;
                        let a1117 = 47u64;
                        nat_eq(a1116, a1117)
                    } else {
                        false
                    } {
                        Some(v4)
                    } else {
                        None::<List<u64>>
                    }
                }
            }
        }
    }
}

pub fn f409(v0: List<u64>) -> bool {
    let m1118 = v0;
    match m1118.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a1119 = v1;
            let a1120 = 58u64;
            if nat_eq(a1119, a1120) {
                let m1121 = v2.clone();
                match m1121.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f482(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f410(p1124: List<u64>, p1125: List<u64>) -> List<u64> {
    let m1123 = (p1124, p1125);
    {
        let mut m1123 = m1123;
        loop {
            let step = {
                let (v0, v1) = m1123;
                let m1122 = v0;
                match m1122.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: Bytes, v1: u64) -> R<Option<Adt99>> {
    let a1127 = v1;
    let a1128 = {
        let a1126 = v0.clone();
        length_bytes(a1126)
    };
    if nat_eq(a1127, a1128) {
        Ok(Some(Adt99::C0(Bytes::lit(&[]), v1)))
    } else {
        f25(v0, v1)
    }
}

pub fn f412(v0: Bytes, v1: Adt97, v2: u64) -> R<Result<Adt97, Adt96>> {
    let a1130 = v2;
    let a1131 = {
        let a1129 = v0.clone();
        length_bytes(a1129)
    };
    if nat_eq(a1130, a1131) {
        Ok(Ok::<Adt97, Adt96>(v1))
    } else {
        let m1132 = f47(v0.clone(), v2)?;
        match m1132 {
            None => {
                Ok(Err::<Adt97, Adt96>(Adt96::C0(v2)))
            }
            Some(v3) => {
                let m1135 = f483(v0.clone(), {
                    let a1133 = v2;
                    let a1134 = 4u64;
                    nat_add(a1133, a1134)?
                }, v3, List::<List<u64>>::nil())?;
                match m1135 {
                    Ok(v5) => {
                        let a1138 = {
                            let (_, h1136) = v5.clone();
                            h1136
                        };
                        let a1139 = {
                            let a1137 = v0;
                            length_bytes(a1137)
                        };
                        if nat_eq(a1138, a1139) {
                            Ok(Ok::<Adt97, Adt96>(f484(v1, {
                                let (h1140, _) = v5;
                                h1140
                            })))
                        } else {
                            Ok(Err::<Adt97, Adt96>(Adt96::C1({
                                let (_, h1141) = v5;
                                h1141
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt97, Adt96>(v4))
                    }
                }
            }
        }
    }
}

pub fn f413(v0: i64, v1: u64) -> R<i64> {
    f437({
        let a1142 = v0;
        let a1143 = 256i64;
        int_mul(a1142, a1143)?
    }, f344(v1, 0i64)?)
}

pub fn f414(v0: List<u64>) -> Option<Str> {
    let a1144 = f440(f217(), v0);
    utf8_decode(a1144)
}

pub fn f415(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt16, u64), Adt96>> {
    let m1146 = f25(v0, {
        let (_, h1145) = v3.clone();
        h1145
    })?;
    match m1146 {
        None => {
            Ok(Err::<(Adt16, u64), Adt96>(Adt96::C0({
                let (_, h1147) = v3;
                h1147
            })))
        }
        Some(v4) => {
            Ok(f485(v1, v2, {
                let (h1148, _) = v3;
                h1148
            }, v4))
        }
    }
}

pub fn f416(p1151: List<u64>, v1: bool) -> bool {
    let m1150 = p1151;
    {
        let mut m1150 = m1150;
        loop {
            let step = {
                let v0 = m1150;
                let m1149 = v0;
                match m1149.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f486(v2)
                        } else {
                            f487(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417() -> u64 {
    64u64
}

pub fn f418(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt101> {
    let m1152 = f337(Str::lit("type"), v1);
    match m1152 {
        Adt109::C0 => {
            Some(Adt101::C3(v0))
        }
        Adt109::C1(v2) => {
            let m1153 = f131(v2);
            match m1153 {
                None => {
                    Some(Adt101::C3(v0))
                }
                Some(v3) => {
                    let a1154 = v3;
                    let a1155 = Str::lit("object");
                    if equal(a1154, a1155) {
                        None::<Adt101>
                    } else {
                        Some(Adt101::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f419(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt101> {
    let m1156 = f337(Str::lit("$schema"), v1);
    match m1156 {
        Adt109::C0 => {
            None::<Adt101>
        }
        Adt109::C1(v2) => {
            let m1157 = f131(v2);
            match m1157 {
                None => {
                    Some(Adt101::C2(v0))
                }
                Some(v3) => {
                    let a1158 = v3;
                    let a1159 = f488();
                    if equal(a1158, a1159) {
                        None::<Adt101>
                    } else {
                        Some(Adt101::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f420(v0: Adt38) -> R<Option<Adt101>> {
    let m1161 = match v0.clone() {
        Adt38::C0(_, _, _, _, _, h1160, _, _) => {
            h1160
        }
    };
    match m1161 {
        None => {
            Ok(None::<Adt101>)
        }
        Some(v1) => {
            let m1164 = f419(match v0.clone() {
                Adt38::C0(h1162, _, _, _, _, _, _, _) => {
                    h1162
                }
            }, match v1.clone() {
                Adt5::C0(h1163) => {
                    h1163
                }
            });
            match m1164 {
                None => {
                    let m1168 = f327(f328(), match v0 {
                        Adt38::C0(h1165, _, _, _, _, _, _, _) => {
                            h1165
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1166) => {
                            h1166
                        }
                    }), List::cons(Adt116::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1167) => {
                            h1167
                        }
                    }), false, List::<Str>::nil()), List::<Adt116>::nil()), f329())?;
                    match m1168 {
                        Ok(_) => {
                            Ok(None::<Adt101>)
                        }
                        Err(v3) => {
                            Ok(Some(v3))
                        }
                    }
                }
                Some(v2) => {
                    Ok(Some(v2))
                }
            }
        }
    }
}

pub fn f421(v0: Str, v1: Adt6, v2: Adt116, v3: Adt123, v4: List<Adt116>) -> R<Result<(List<Adt116>, Adt123), Adt101>> {
    let m1169 = v2;
    match m1169 {
        Adt116::C0(v5, v6, v7) => {
            let m1170 = f438(v5.clone());
            match m1170 {
                None => {
                    let m1171 = f234(v5);
                    match m1171 {
                        None => {
                            Ok(Err::<(List<Adt116>, Adt123), Adt101>(Adt101::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f489(v0, v1, match v9 {
                                Adt5::C0(h1172) => {
                                    h1172
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt116>, Adt123), Adt101>((v4, v3)))
                }
            }
        }
    }
}

pub fn f422(p1181: List<Adt41>) -> R<bool> {
    let m1180 = p1181;
    {
        let mut m1180 = m1180;
        loop {
            let step = {
                let v0 = m1180;
                let m1173 = v0;
                match m1173.uncons() {
                    None => {
                        Err::<List<Adt41>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1174 = v1;
                        match m1174 {
                            Adt41::C0(_) => {
                                Ok::<List<Adt41>, R<bool>>(v2)
                            }
                            Adt41::C1(v4) => {
                                if f490(match v4 {
                                    Adt45::C0(h1175, _, _, _) => {
                                        h1175
                                    }
                                })? {
                                    Ok::<List<Adt41>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt41>, R<bool>>(Ok(false))
                                }
                            }
                            Adt41::C2(v5) => {
                                if f490(match v5 {
                                    Adt46::C0(h1176, _, _, _) => {
                                        h1176
                                    }
                                })? {
                                    Ok::<List<Adt41>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt41>, R<bool>>(Ok(false))
                                }
                            }
                            Adt41::C3(_) => {
                                Ok::<List<Adt41>, R<bool>>(v2)
                            }
                            Adt41::C4(v7) => {
                                let m1178 = match v7 {
                                    Adt48::C0(h1177, _, _) => {
                                        h1177
                                    }
                                };
                                match m1178 {
                                    Adt49::C0(_) => {
                                        Ok::<List<Adt41>, R<bool>>(v2)
                                    }
                                    Adt49::C1(v9) => {
                                        if f490(match v9 {
                                            Adt51::C0(_, _, h1179, _) => {
                                                h1179
                                            }
                                        })? {
                                            Ok::<List<Adt41>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt41>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(v0: Adt38, v1: Adt52) -> R<Result<Adt35, Adt92>> {
    if f491(match v1.clone() {
        Adt52::C0(h1182, _) => {
            h1182
        }
    }) {
        f492(v1)
    } else {
        let m1184 = match v0 {
            Adt38::C0(_, _, _, _, _, h1183, _, _) => {
                h1183
            }
        };
        match m1184 {
            None => {
                f492(v1)
            }
            Some(v2) => {
                f493(v2, v1)
            }
        }
    }
}

pub fn f424() -> Adt92 {
    Adt92::C9(Str::lit("the host supplied no task id"))
}

pub fn f425() -> Adt92 {
    Adt92::C9(Str::lit("the host supplied a task id that a task already has"))
}

pub fn f426(v0: Adt16, v1: Adt88) -> Adt87 {
    Adt87::C0(Adt65::C0(Adt66::C0(match v0.clone() {
        Adt16::C0(_, _, h1185, _) => {
            h1185
        }
    }, None::<Str>, match v0.clone() {
        Adt16::C0(_, h1186, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt16::C0(_, h1187, _, _) => {
            h1187
        }
    }, Some(f494()), Some(f495()))), match v0 {
        Adt16::C0(h1188, _, _, _) => {
            h1188
        }
    }, v1)
}

pub fn f427(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt118>) -> R<Adt141> {
    let m1189 = f438(v1.clone());
    match m1189 {
        None => {
            let m1190 = f234(v1);
            match m1190 {
                None => {
                    Ok(Adt141::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f496(v0, match v5 {
                        Adt5::C0(h1191) => {
                            h1191
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt141::C3(List::<Adt120>::nil()))
            } else {
                Ok(Adt141::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f428(v0: Adt120, v1: List<Adt120>) -> R<Adt124> {
    let m1192 = v0;
    match m1192 {
        Adt120::C0(_, _, _) => {
            Ok(Adt124::C0(Adt119::C0, v1))
        }
        Adt120::C1(_, _, _) => {
            Ok(Adt124::C0(Adt119::C0, v1))
        }
        Adt120::C2(v8, v9, v10, v11) => {
            Ok(f497(v8, v9, v10, {
                let a1193 = v11;
                let a1194 = 1u64;
                nat_add(a1193, a1194)?
            }, v1))
        }
        Adt120::C3(v12) => {
            Ok(Adt124::C0(Adt119::C1(v12, Str::lit("not")), v1))
        }
        Adt120::C4(v13, _, v15, v16) => {
            Ok(f498(v13, v15, v16, v1))
        }
        Adt120::C5(_, _, _, _) => {
            Ok(Adt124::C0(Adt119::C0, v1))
        }
    }
}

pub fn f429(v0: Adt121, v1: Adt6, v2: Adt6, v3: List<Adt118>) -> Adt141 {
    let m1195 = f234(v1);
    match m1195 {
        None => {
            Adt141::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1196 = v0;
            match m1196 {
                Adt121::C0 => {
                    f499(match v4 {
                        Adt5::C0(h1197) => {
                            h1197
                        }
                    }, v2, v3)
                }
                Adt121::C1 => {
                    f500(match v4 {
                        Adt5::C0(h1198) => {
                            h1198
                        }
                    }, v2, v3)
                }
                Adt121::C2 => {
                    f501(match v4 {
                        Adt5::C0(h1199) => {
                            h1199
                        }
                    }, v2, v3)
                }
                Adt121::C3 => {
                    f502(match v4 {
                        Adt5::C0(h1200) => {
                            h1200
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f430(v0: Adt120, v1: List<Adt120>, v2: List<Adt118>, v3: Str) -> Adt124 {
    let m1201 = v0;
    match m1201 {
        Adt120::C0(_, _, _) => {
            Adt124::C0(Adt119::C1(v2, v3), v1)
        }
        Adt120::C1(v7, v8, v9) => {
            f503(v7, v8, v9, v1)
        }
        Adt120::C2(v10, v11, v12, v13) => {
            f497(v10, v11, v12, v13, v1)
        }
        Adt120::C3(_) => {
            Adt124::C0(Adt119::C0, v1)
        }
        Adt120::C4(_, v16, v17, v18) => {
            f498(v16, v17, v18, v1)
        }
        Adt120::C5(_, _, _, _) => {
            Adt124::C0(Adt119::C1(v2, v3), v1)
        }
    }
}

pub fn f431(p1204: List<Adt118>, p1205: List<Str>) -> R<List<Str>> {
    let m1203 = (p1204, p1205);
    {
        let mut m1203 = m1203;
        loop {
            let step = {
                let (v0, v1) = m1203;
                let m1202 = v0;
                match m1202.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Str>), R<List<Str>>>((v3, List::cons(f504(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1203 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: Str, v1: Str) -> Adt142 {
    let a1206 = v0;
    let a1207 = v1;
    if equal(a1206, a1207) {
        Adt142::C0
    } else {
        Adt142::C1
    }
}

pub fn f433(p1222: u64, p1223: Adt125, p1224: List<Adt126>) -> R<Option<Adt6>> {
    let m1221 = (p1222, (p1223, p1224));
    {
        let mut m1221 = m1221;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1221;
                let m1208 = v0;
                if m1208 == 0 {
                    Err::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1208 - 1;
                    let m1209 = v1;
                    match m1209 {
                        Adt125::C0(v4) => {
                            let m1210 = v4.clone();
                            match m1210 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f505(v7)?;
                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1211 = v9;
                                    match m1211.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C0(v10), List::cons(Adt126::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1212 = v12;
                                    match m1212.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f505({
                                                let (h1213, _) = v13;
                                                h1213
                                            })?;
                                            Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C0({
                                                let (_, h1214) = v13;
                                                h1214
                                            }), List::cons(Adt126::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt125::C1(v16) => {
                            let m1215 = v2;
                            match m1215.uncons() {
                                None => {
                                    Err::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1216 = v17;
                                    match m1216 {
                                        Adt126::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1217 = v20;
                                            match m1217.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(Adt6::C4(f436(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C0(v22), List::cons(Adt126::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt126::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1218 = v26;
                                            match m1218.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C1(Adt6::C5(f506(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f505({
                                                        let (h1219, _) = v28;
                                                        h1219
                                                    })?;
                                                    Ok::<(u64, (Adt125, List<Adt126>)), R<Option<Adt6>>>((v3, (Adt125::C0({
                                                        let (_, h1220) = v28;
                                                        h1220
                                                    }), List::cons(Adt126::C1(v30, v27, v29), v18))))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1221 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434() -> u64 {
    67108864u64
}

pub fn f435(p1228: List<Adt41>, p1229: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1227 = (p1228, p1229);
    {
        let mut m1227 = m1227;
        loop {
            let step = {
                let (v0, v1) = m1227;
                let m1225 = v0;
                match m1225.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1226 = f507(v2)?;
                        match m1226 {
                            None => {
                                Err::<(List<Adt41>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt41>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(p1232: List<Adt6>, p1233: List<Adt6>) -> List<Adt6> {
    let m1231 = (p1232, p1233);
    {
        let mut m1231 = m1231;
        loop {
            let step = {
                let (v0, v1) = m1231;
                let m1230 = v0;
                match m1230.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1231 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: i64, v1: i64) -> R<i64> {
    let a1235 = v0;
    let a1236 = {
        let a1234 = v1;
        int_neg(a1234)?
    };
    int_sub(a1235, a1236)
}

pub fn f438(v0: Adt6) -> Option<bool> {
    let m1237 = v0;
    match m1237 {
        Adt6::C0 => {
            None::<bool>
        }
        Adt6::C1(v1) => {
            Some(v1)
        }
        Adt6::C2(_) => {
            None::<bool>
        }
        Adt6::C3(_) => {
            None::<bool>
        }
        Adt6::C4(_) => {
            None::<bool>
        }
        Adt6::C5(_) => {
            None::<bool>
        }
    }
}

pub fn f439(v0: Str) -> R<Option<List<u64>>> {
    let m1238 = f508(f509(f510())?, f509(v0)?);
    match m1238 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1239 = f508(f511(List::cons(47u64, f509(f512())?)), f511(v1));
            match m1239 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v2) => {
                    let v3: List<u64> = f511(v2);
                    if if f513(v3.clone()) {
                        f514(v3.clone())
                    } else {
                        false
                    } {
                        Ok(Some(v3))
                    } else {
                        Ok(None::<List<u64>>)
                    }
                }
            }
        }
    }
}

pub fn f440(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1240 = v0;
    let a1241 = f515(f217(), f516(List::<Bytes>::nil(), v1));
    append_bytes(a1240, a1241)
}

pub fn f441(v0: Adt6) -> R<Result<Bytes, Adt111>> {
    let m1242 = f298(v0)?;
    match m1242 {
        Ok(v1) => {
            Ok(Ok::<Bytes, Adt111>(f440(f217(), v1)))
        }
        Err(v2) => {
            Ok(Err::<Bytes, Adt111>(v2))
        }
    }
}

pub fn f442(v0: Adt5) -> bool {
    if f517(f229(Str::lit("action"), v0.clone()), Str::lit("accept")) {
        true
    } else {
        if f517(f229(Str::lit("action"), v0.clone()), Str::lit("decline")) {
            true
        } else {
            f517(f229(Str::lit("action"), v0), Str::lit("cancel"))
        }
    }
}

pub fn f443(v0: Adt5) -> bool {
    let m1243 = f229(Str::lit("content"), v0);
    match m1243 {
        Adt109::C0 => {
            true
        }
        Adt109::C1(v1) => {
            let m1244 = f234(v1);
            match m1244 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f444(v0: Adt5) -> bool {
    if f517(f229(Str::lit("role"), v0.clone()), Str::lit("user")) {
        true
    } else {
        f517(f229(Str::lit("role"), v0), Str::lit("assistant"))
    }
}

pub fn f445(v0: Adt109) -> bool {
    let m1245 = v0;
    match m1245 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(v1) => {
            let m1246 = f131(v1);
            match m1246 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f446(v0: Adt5) -> bool {
    let m1247 = f229(Str::lit("content"), v0);
    match m1247 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(v1) => {
            let m1248 = f234(v1.clone());
            match m1248 {
                None => {
                    let m1249 = f289(v1);
                    match m1249 {
                        None => {
                            false
                        }
                        Some(_) => {
                            true
                        }
                    }
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f447(p1253: List<Adt6>) -> bool {
    let m1252 = p1253;
    {
        let mut m1252 = m1252;
        loop {
            let step = {
                let v0 = m1252;
                let m1250 = v0;
                match m1250.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m1251 = f234(v1);
                        match m1251 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f445(f229(Str::lit("uri"), v3)) {
                                    Ok::<List<Adt6>, bool>(v2)
                                } else {
                                    Err::<List<Adt6>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1252 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(p1257: List<Str>, p1258: u64, p1259: List<Str>) -> List<Str> {
    let m1256 = (p1257, (p1258, p1259));
    {
        let mut m1256 = m1256;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1256;
                let m1254 = v0;
                match m1254.uncons() {
                    None => {
                        Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                    }
                    Some((v3, v4)) => {
                        let m1255 = v1;
                        if m1255 == 0 {
                            Err::<(List<Str>, (u64, List<Str>)), List<Str>>(v2)
                        } else {
                            let v5 = m1255 - 1;
                            Ok::<(List<Str>, (u64, List<Str>)), List<Str>>((v4, (v5, List::cons(v3, v2))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1256 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: Str, v1: Str) -> R<bool> {
    Ok(f518(f509(v0)?, f509(v1)?))
}

pub fn f450(p1263: List<Adt47>, p1264: List<Str>) -> List<Str> {
    let m1262 = (p1263, p1264);
    {
        let mut m1262 = m1262;
        loop {
            let step = {
                let (v0, v1) = m1262;
                let m1260 = v0;
                match m1260.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Str>), List<Str>>((v3, List::cons(match v2 {
                            Adt47::C0(_, _, _, h1261, _, _, _, _, _) => {
                                h1261
                            }
                        }, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f451(v0: Adt109) -> bool {
    let m1265 = v0;
    match m1265 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(_) => {
            true
        }
    }
}

pub fn f452(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m1266 = f229(v0, v1);
    match m1266 {
        Adt109::C0 => {
            None::<Adt5>
        }
        Adt109::C1(v2) => {
            f234(v2)
        }
    }
}

pub fn f453(v0: Adt5) -> Option<Adt65> {
    let m1267 = f519(v0.clone());
    match m1267 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m1268 = f520(Str::lit("status"), v0.clone());
            match m1268 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a1269 = v2.clone();
                    let a1270 = Str::lit("working");
                    if equal(a1269, a1270) {
                        Some(Adt65::C0(v1))
                    } else {
                        let a1271 = v2.clone();
                        let a1272 = Str::lit("cancelled");
                        if equal(a1271, a1272) {
                            Some(Adt65::C4(v1))
                        } else {
                            let a1273 = v2.clone();
                            let a1274 = Str::lit("input_required");
                            if equal(a1273, a1274) {
                                let m1275 = f521(Str::lit("inputRequests"), v0);
                                match m1275 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v3) => {
                                        Some(Adt65::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a1276 = v2.clone();
                                let a1277 = Str::lit("completed");
                                if equal(a1276, a1277) {
                                    let m1278 = f452(Str::lit("result"), v0);
                                    match m1278 {
                                        None => {
                                            None::<Adt65>
                                        }
                                        Some(v4) => {
                                            Some(Adt65::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a1279 = v2;
                                    let a1280 = Str::lit("failed");
                                    if equal(a1279, a1280) {
                                        let m1281 = f452(Str::lit("error"), v0);
                                        match m1281 {
                                            None => {
                                                None::<Adt65>
                                            }
                                            Some(v5) => {
                                                let m1282 = f522(v5);
                                                match m1282 {
                                                    None => {
                                                        None::<Adt65>
                                                    }
                                                    Some(v6) => {
                                                        Some(Adt65::C3(v1, v6))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        None::<Adt65>
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f454(v0: Str, v1: Adt5) -> Option<i64> {
    let m1283 = f229(v0, v1);
    match m1283 {
        Adt109::C0 => {
            None::<i64>
        }
        Adt109::C1(v2) => {
            f230(v2)
        }
    }
}

pub fn f455(v0: Adt5) -> Option<Adt88> {
    let m1284 = f454(Str::lit("parkAfterMs"), v0.clone());
    match m1284 {
        None => {
            None::<Adt88>
        }
        Some(v1) => {
            let m1285 = f521(Str::lit("parked"), v0.clone());
            match m1285 {
                None => {
                    None::<Adt88>
                }
                Some(v2) => {
                    let m1286 = f454(Str::lit("finishAfterMs"), v0.clone());
                    match m1286 {
                        None => {
                            None::<Adt88>
                        }
                        Some(v3) => {
                            let m1287 = f452(Str::lit("outcome"), v0);
                            match m1287 {
                                None => {
                                    None::<Adt88>
                                }
                                Some(v4) => {
                                    let m1288 = f523(v4);
                                    match m1288 {
                                        None => {
                                            None::<Adt88>
                                        }
                                        Some(v5) => {
                                            Some(Adt88::C0(v1, v2, v3, v5))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f456(p1291: List<(Str, Adt67)>, p1292: List<(Str, Adt67)>) -> List<(Str, Adt67)> {
    let m1290 = (p1291, p1292);
    {
        let mut m1290 = m1290;
        loop {
            let step = {
                let (v0, v1) = m1290;
                let m1289 = v0;
                match m1289.uncons() {
                    None => {
                        Err::<(List<(Str, Adt67)>, List<(Str, Adt67)>), List<(Str, Adt67)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt67)>, List<(Str, Adt67)>), List<(Str, Adt67)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1290 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(p1296: List<(Str, Adt67)>, v1: Adt5, p1297: List<(Str, Adt67)>) -> List<(Str, Adt67)> {
    let m1295 = (p1296, p1297);
    {
        let mut m1295 = m1295;
        loop {
            let step = {
                let (v0, v2) = m1295;
                let m1293 = v0;
                match m1293.uncons() {
                    None => {
                        Err::<(List<(Str, Adt67)>, List<(Str, Adt67)>), List<(Str, Adt67)>>(v2)
                    }
                    Some((v3, v4)) => {
                        if f374({
                            let (h1294, _) = v3.clone();
                            h1294
                        }, v1.clone()) {
                            Ok::<(List<(Str, Adt67)>, List<(Str, Adt67)>), List<(Str, Adt67)>>((v4, v2))
                        } else {
                            Ok::<(List<(Str, Adt67)>, List<(Str, Adt67)>), List<(Str, Adt67)>>((v4, List::cons(v3, v2)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p1302: u64, p1303: List<Adt127>, p1304: List<u64>) -> R<Result<List<u64>, Adt111>> {
    let m1301 = (p1302, (p1303, p1304));
    {
        let mut m1301 = m1301;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1301;
                let m1298 = v0;
                if m1298 == 0 {
                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Err::<List<u64>, Adt111>(Adt111::C13)))
                } else {
                    let v3 = m1298 - 1;
                    let m1299 = v1;
                    match m1299.uncons() {
                        None => {
                            Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Ok::<List<u64>, Adt111>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m1300 = f524(v4, v5, v2)?;
                            match m1300 {
                                Adt152::C0(v6) => {
                                    Err::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt111>>>(Ok(Err::<List<u64>, Adt111>(v6)))
                                }
                                Adt152::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt127>, List<u64>)), R<Result<List<u64>, Adt111>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(p1307: List<u64>, p1308: List<u64>) -> List<u64> {
    let m1306 = (p1307, p1308);
    {
        let mut m1306 = m1306;
        loop {
            let step = {
                let (v0, v1) = m1306;
                let m1305 = v0;
                match m1305.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Adt108) -> Adt6 {
    let m1309 = v0;
    match m1309 {
        Adt108::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt108::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f461(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1310 = v0;
    match m1310 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f462() -> i64 {
    -32700i64
}

pub fn f463() -> i64 {
    -32600i64
}

pub fn f464(v0: Adt113) -> Str {
    let m1311 = v0;
    match m1311 {
        Adt113::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt113::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt113::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt113::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt113::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt113::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt113::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt113::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt113::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f465(v0: Adt0, v1: bool, v2: Adt16, v3: bool, v4: Adt143) -> R<Adt94> {
    let m1313 = f525(match v4.clone() {
        Adt143::C0(_, h1312) => {
            h1312
        }
    });
    match m1313 {
        None => {
            Ok(f527(v0, v1, f528(v2, v3, v4)?))
        }
        Some(v5) => {
            f526(v2, v3, v4, v5)
        }
    }
}

pub fn f466(p1318: List<Adt98>) -> R<bool> {
    let m1317 = p1318;
    {
        let mut m1317 = m1317;
        loop {
            let step = {
                let v0 = m1317;
                let m1314 = v0;
                match m1314.uncons() {
                    None => {
                        Err::<List<Adt98>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f400(match v1.clone() {
                            Adt98::C0(h1315, _) => {
                                h1315
                            }
                        }, f529())? {
                            if f530(f531(44u64, match v1 {
                                Adt98::C0(_, h1316) => {
                                    h1316
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt98>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt98>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt98>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1317 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt0, v1: Fn9, v2: u64, v3: Option<Adt128>, v4: List<Str>, v5: Adt122) -> R<Adt143> {
    Ok(Adt143::C0(match v5.clone() {
        Adt122::C0(h1319, _, _) => {
            h1319
        }
    }, f532(v0, v1, v2, f533(v3, v4, v5)?)?))
}

pub fn f468(v0: List<Adt98>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt128> {
    let v4: Adt149 = f534(Adt130::C0, f209(v0.clone(), f535())?);
    let v5: Adt149 = f534(Adt130::C1, f209(v0.clone(), f536())?);
    let v6: Adt149 = if f537(v3) {
        f534(Adt130::C2, f538(f209(v0.clone(), f539())?)?)
    } else {
        f540()
    };
    Ok(Adt128::C0(match v4.clone() {
        Adt149::C0(h1320, _) => {
            h1320
        }
    }, match v5.clone() {
        Adt149::C0(h1321, _) => {
            h1321
        }
    }, match v6.clone() {
        Adt149::C0(h1322, _) => {
            h1322
        }
    }, f541(match v4 {
        Adt149::C0(_, h1323) => {
            h1323
        }
    }, f541(match v5 {
        Adt149::C0(_, h1324) => {
            h1324
        }
    }, match v6 {
        Adt149::C0(_, h1325) => {
            h1325
        }
    })), f542(v0, v1, v2)?))
}

pub fn f469(v0: Adt122) -> Option<Str> {
    let a1327 = match v0.clone() {
        Adt122::C0(_, h1326, _) => {
            h1326
        }
    };
    let a1328 = Str::lit("tools/call");
    if equal(a1327, a1328) {
        let m1330 = match v0 {
            Adt122::C0(_, _, h1329) => {
                h1329
            }
        };
        match m1330 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m1331 = f229(Str::lit("name"), v1);
                match m1331 {
                    Adt109::C0 => {
                        None::<Str>
                    }
                    Adt109::C1(v2) => {
                        f131(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f470() -> List<Str> {
    List::cons(f543(), List::<Str>::nil())
}

pub fn f471(v0: Adt5) -> R<Adt150> {
    let m1332 = f229(Str::lit("id"), v0);
    match m1332 {
        Adt109::C0 => {
            Ok(Adt150::C0)
        }
        Adt109::C1(v1) => {
            let m1333 = v1;
            match m1333 {
                Adt6::C0 => {
                    Ok(Adt150::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt150::C2)
                }
                Adt6::C2(v3) => {
                    let m1334 = f544(v3)?;
                    match m1334 {
                        None => {
                            Ok(Adt150::C2)
                        }
                        Some(v4) => {
                            Ok(Adt150::C1(Adt108::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt150::C1(Adt108::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt150::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt150::C2)
                }
            }
        }
    }
}

pub fn f472(v0: Adt5) -> Adt113 {
    if f473(v0) {
        Adt113::C8
    } else {
        Adt113::C3
    }
}

pub fn f473(v0: Adt5) -> bool {
    if f374(Str::lit("result"), v0.clone()) {
        true
    } else {
        f374(Str::lit("error"), v0)
    }
}

pub fn f474(v0: Adt108, v1: Adt5) -> Result<Adt122, Adt112> {
    let m1335 = f229(Str::lit("jsonrpc"), v1.clone());
    match m1335 {
        Adt109::C0 => {
            f396(Some(v0), Adt113::C2)
        }
        Adt109::C1(v2) => {
            let m1336 = f131(v2);
            match m1336 {
                None => {
                    f396(Some(v0), Adt113::C2)
                }
                Some(v3) => {
                    let a1337 = v3;
                    let a1338 = f387();
                    if equal(a1337, a1338) {
                        f545(v0, v1)
                    } else {
                        f396(Some(v0), Adt113::C2)
                    }
                }
            }
        }
    }
}

pub fn f475(v0: u64, v1: u64) -> Adt133 {
    let a1339 = v1;
    let a1340 = 0u64;
    if nat_eq(a1339, a1340) {
        Adt133::C0
    } else {
        let a1341 = v1;
        let a1342 = 2u64;
        if nat_eq(a1341, a1342) {
            Adt133::C2(v0)
        } else {
            Adt133::C1({
                let a1346 = v0;
                let a1347 = {
                    let a1343 = v1;
                    let a1344 = 3u64;
                    let a1345 = 0u64;
                    nat_quot(a1343, a1344, a1345)
                };
                nat_sub(a1346, a1347)
            })
        }
    }
}

pub fn f476(p1375: Adt134, p1376: u64, p1377: u64, p1378: List<u64>) -> R<u64> {
    let m1374 = (p1375, (p1376, (p1377, p1378)));
    {
        let mut m1374 = m1374;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1374;
                let m1348 = v3;
                match m1348.uncons() {
                    None => {
                        let m1349 = v0;
                        match m1349 {
                            Adt134::C0 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt134::C1 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt134::C2 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt134::C3 => {
                                Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1350 = v0;
                        match m1350 {
                            Adt134::C0 => {
                                let a1351 = v4;
                                let a1352 = 128u64;
                                if nat_lt(a1351, a1352) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a1353 = v4;
                                    let a1354 = 194u64;
                                    if nat_lt(a1353, a1354) {
                                        let v6: u64 = f546({
                                            let a1355 = List::cons(v4, v5);
                                            length_list(a1355)
                                        })?;
                                        Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a1356 = v4;
                                        let a1357 = 224u64;
                                        if nat_lt(a1356, a1357) {
                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a1358 = v4;
                                            let a1359 = 224u64;
                                            if nat_eq(a1358, a1359) {
                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a1360 = v4;
                                                let a1361 = 237u64;
                                                if nat_eq(a1360, a1361) {
                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a1362 = v4;
                                                    let a1363 = 240u64;
                                                    if nat_lt(a1362, a1363) {
                                                        Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a1364 = v4;
                                                        let a1365 = 240u64;
                                                        if nat_eq(a1364, a1365) {
                                                            Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a1366 = v4;
                                                            let a1367 = 244u64;
                                                            if nat_eq(a1366, a1367) {
                                                                Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a1368 = v4;
                                                                let a1369 = 244u64;
                                                                if nat_lt(a1368, a1369) {
                                                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f546({
                                                                        let a1370 = List::cons(v4, v5);
                                                                        length_list(a1370)
                                                                    })?;
                                                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt134::C1 => {
                                if f547(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f546({
                                        let a1371 = List::cons(v4, v5);
                                        length_list(a1371)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt134::C2 => {
                                if f547(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f546({
                                        let a1372 = List::cons(v4, v5);
                                        length_list(a1372)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt134::C3 => {
                                if f547(v4, v1, v2) {
                                    Ok::<(Adt134, (u64, (u64, List<u64>))), R<u64>>((Adt134::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f546({
                                        let a1373 = List::cons(v4, v5);
                                        length_list(a1373)
                                    })?;
                                    Err::<(Adt134, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: Adt135, v1: u64) -> R<Result<Adt6, Adt111>> {
    let m1380 = match v0.clone() {
        Adt135::C0(h1379, _, _) => {
            h1379
        }
    };
    match m1380 {
        Adt136::C0 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C1 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C2 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C3 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C4 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C5 => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C7(_, _) => {
            Ok(Err::<Adt6, Adt111>(Adt111::C3))
        }
        Adt136::C8(v3) => {
            let m1381 = f548(v3)?;
            match m1381 {
                Adt155::C0 => {
                    Ok(Err::<Adt6, Adt111>(Adt111::C7(v1)))
                }
                Adt155::C1 => {
                    Ok(Err::<Adt6, Adt111>(Adt111::C8(v1)))
                }
                Adt155::C2(v4) => {
                    let m1386 = {
                        let m1385 = f549(v4, match v0.clone() {
                            Adt135::C0(_, h1383, _) => {
                                h1383
                            }
                        }, match v0 {
                            Adt135::C0(_, _, h1384) => {
                                h1384
                            }
                        });
                        match m1385 {
                            Adt135::C0(h1382, _, _) => {
                                h1382
                            }
                        }
                    };
                    match m1386 {
                        Adt136::C0 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C1 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C2 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C3 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C4 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C5 => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C7(_, _) => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C8(_) => {
                            Ok(Err::<Adt6, Adt111>(Adt111::C3))
                        }
                        Adt136::C9(v5) => {
                            Ok(Ok::<Adt6, Adt111>(v5))
                        }
                    }
                }
            }
        }
        Adt136::C9(v2) => {
            Ok(Ok::<Adt6, Adt111>(v2))
        }
    }
}

pub fn f478(v0: Adt135, v1: u64, v2: u64) -> R<Adt151> {
    let m1388 = match v0.clone() {
        Adt135::C0(h1387, _, _) => {
            h1387
        }
    };
    match m1388 {
        Adt136::C0 => {
            if f550(v2) {
                Ok(Adt151::C1(v0))
            } else {
                f551(v0, v1, v2)
            }
        }
        Adt136::C1 => {
            if f550(v2) {
                Ok(Adt151::C1(v0))
            } else {
                let a1389 = v2;
                let a1390 = 93u64;
                if nat_eq(a1389, a1390) {
                    Ok(f552(v0, v1))
                } else {
                    f551(v0, v1, v2)
                }
            }
        }
        Adt136::C2 => {
            Ok(f553(v0, v1, v2))
        }
        Adt136::C3 => {
            if f550(v2) {
                Ok(Adt151::C1(v0))
            } else {
                let a1391 = v2;
                let a1392 = 125u64;
                if nat_eq(a1391, a1392) {
                    Ok(f554(v0, v1))
                } else {
                    let a1393 = v2;
                    let a1394 = 34u64;
                    if nat_eq(a1393, a1394) {
                        Ok(f555(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f556(Adt111::C2(v1)))
                    }
                }
            }
        }
        Adt136::C4 => {
            if f550(v2) {
                Ok(Adt151::C1(v0))
            } else {
                let a1395 = v2;
                let a1396 = 34u64;
                if nat_eq(a1395, a1396) {
                    Ok(f555(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f556(Adt111::C2(v1)))
                }
            }
        }
        Adt136::C5 => {
            if f550(v2) {
                Ok(Adt151::C1(v0))
            } else {
                let a1397 = v2;
                let a1398 = 58u64;
                if nat_eq(a1397, a1398) {
                    Ok(f555(v0, Adt136::C0))
                } else {
                    Ok(f556(Adt111::C2(v1)))
                }
            }
        }
        Adt136::C6(v3, v4, v5) => {
            f557(v0, v1, v2, v3, v4, v5)
        }
        Adt136::C7(v6, v7) => {
            f558(v0, v1, v2, v6, v7)
        }
        Adt136::C8(v8) => {
            f559(v0, v1, v2, v8)
        }
        Adt136::C9(_) => {
            Ok(f553(v0, v1, v2))
        }
    }
}

pub fn f479(v0: u64) -> R<u64> {
    let a1399 = 64u64;
    let a1400 = v0;
    if if nat_lt(a1399, a1400) {
        let a1401 = v0;
        let a1402 = 91u64;
        nat_lt(a1401, a1402)
    } else {
        false
    } {
        let a1403 = v0;
        let a1404 = 32u64;
        nat_add(a1403, a1404)
    } else {
        Ok(v0)
    }
}

pub fn f480(p1407: List<u64>) -> List<u64> {
    let m1406 = p1407;
    {
        let mut m1406 = m1406;
        loop {
            let step = {
                let v0 = m1406;
                let m1405 = v0;
                match m1405.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f560(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: u64) -> bool {
    let a1408 = v0;
    let a1409 = 9u64;
    if nat_eq(a1408, a1409) {
        true
    } else {
        let a1410 = 31u64;
        let a1411 = v0;
        if nat_lt(a1410, a1411) {
            let a1412 = v0;
            let a1413 = 127u64;
            nat_lt(a1412, a1413)
        } else {
            false
        }
    }
}

pub fn f482(p1416: List<u64>) -> bool {
    let m1415 = p1416;
    {
        let mut m1415 = m1415;
        loop {
            let step = {
                let v0 = m1415;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f561(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f483(v0: Bytes, p1422: u64, p1423: u64, p1424: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt96>> {
    let m1421 = (p1422, (p1423, p1424));
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1421;
                let m1417 = v2;
                if m1417 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt96>>>(Ok(Ok::<(List<List<u64>>, u64), Adt96>((f107(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m1417 - 1;
                    let m1418 = f25(v0.clone(), v1)?;
                    match m1418 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt96>>>(Ok(Err::<(List<List<u64>>, u64), Adt96>(Adt96::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt96>>>((match v5.clone() {
                                Adt99::C0(_, h1419) => {
                                    h1419
                                }
                            }, (v4, List::cons(f10(match v5 {
                                Adt99::C0(h1420, _) => {
                                    h1420
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f484(v0: Adt97, v1: List<List<u64>>) -> Adt97 {
    Adt97::C0(match v0.clone() {
        Adt97::C0(h1425, _, _, _, _, _, _, _) => {
            h1425
        }
    }, match v0.clone() {
        Adt97::C0(_, h1426, _, _, _, _, _, _) => {
            h1426
        }
    }, match v0.clone() {
        Adt97::C0(_, _, h1427, _, _, _, _, _) => {
            h1427
        }
    }, match v0.clone() {
        Adt97::C0(_, _, _, h1428, _, _, _, _) => {
            h1428
        }
    }, match v0.clone() {
        Adt97::C0(_, _, _, _, h1429, _, _, _) => {
            h1429
        }
    }, v1, match v0.clone() {
        Adt97::C0(_, _, _, _, _, _, h1430, _) => {
            h1430
        }
    }, match v0 {
        Adt97::C0(_, _, _, _, _, _, _, h1431) => {
            h1431
        }
    })
}

pub fn f485(v0: i64, v1: Str, v2: Str, v3: Adt99) -> Result<(Adt16, u64), Adt96> {
    Ok::<(Adt16, u64), Adt96>((Adt16::C0(v0, v1, v2, match v3.clone() {
        Adt99::C0(h1432, _) => {
            h1432
        }
    }), match v3 {
        Adt99::C0(_, h1433) => {
            h1433
        }
    }))
}

pub fn f486(v0: u64) -> bool {
    let a1434 = 48u64;
    let a1435 = v0;
    if if nat_le(a1434, a1435) {
        let a1436 = v0;
        let a1437 = 57u64;
        nat_le(a1436, a1437)
    } else {
        false
    } {
        true
    } else {
        let a1438 = 65u64;
        let a1439 = v0;
        if if nat_le(a1438, a1439) {
            let a1440 = v0;
            let a1441 = 90u64;
            nat_le(a1440, a1441)
        } else {
            false
        } {
            true
        } else {
            let a1442 = 97u64;
            let a1443 = v0;
            if if nat_le(a1442, a1443) {
                let a1444 = v0;
                let a1445 = 122u64;
                nat_le(a1444, a1445)
            } else {
                false
            } {
                true
            } else {
                let a1446 = v0;
                let a1447 = 33u64;
                if nat_eq(a1446, a1447) {
                    true
                } else {
                    let a1448 = v0;
                    let a1449 = 35u64;
                    if nat_eq(a1448, a1449) {
                        true
                    } else {
                        let a1450 = v0;
                        let a1451 = 36u64;
                        if nat_eq(a1450, a1451) {
                            true
                        } else {
                            let a1452 = v0;
                            let a1453 = 37u64;
                            if nat_eq(a1452, a1453) {
                                true
                            } else {
                                let a1454 = v0;
                                let a1455 = 38u64;
                                if nat_eq(a1454, a1455) {
                                    true
                                } else {
                                    let a1456 = v0;
                                    let a1457 = 39u64;
                                    if nat_eq(a1456, a1457) {
                                        true
                                    } else {
                                        let a1458 = v0;
                                        let a1459 = 42u64;
                                        if nat_eq(a1458, a1459) {
                                            true
                                        } else {
                                            let a1460 = v0;
                                            let a1461 = 43u64;
                                            if nat_eq(a1460, a1461) {
                                                true
                                            } else {
                                                let a1462 = v0;
                                                let a1463 = 45u64;
                                                if nat_eq(a1462, a1463) {
                                                    true
                                                } else {
                                                    let a1464 = v0;
                                                    let a1465 = 46u64;
                                                    if nat_eq(a1464, a1465) {
                                                        true
                                                    } else {
                                                        let a1466 = v0;
                                                        let a1467 = 94u64;
                                                        if nat_eq(a1466, a1467) {
                                                            true
                                                        } else {
                                                            let a1468 = v0;
                                                            let a1469 = 95u64;
                                                            if nat_eq(a1468, a1469) {
                                                                true
                                                            } else {
                                                                let a1470 = v0;
                                                                let a1471 = 96u64;
                                                                if nat_eq(a1470, a1471) {
                                                                    true
                                                                } else {
                                                                    let a1472 = v0;
                                                                    let a1473 = 124u64;
                                                                    if nat_eq(a1472, a1473) {
                                                                        true
                                                                    } else {
                                                                        let a1474 = v0;
                                                                        let a1475 = 126u64;
                                                                        nat_eq(a1474, a1475)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f487(v0: u64) -> bool {
    let a1476 = 48u64;
    let a1477 = v0;
    if if nat_le(a1476, a1477) {
        let a1478 = v0;
        let a1479 = 57u64;
        nat_le(a1478, a1479)
    } else {
        false
    } {
        true
    } else {
        let a1480 = 65u64;
        let a1481 = v0;
        if if nat_le(a1480, a1481) {
            let a1482 = v0;
            let a1483 = 90u64;
            nat_le(a1482, a1483)
        } else {
            false
        } {
            true
        } else {
            let a1484 = 97u64;
            let a1485 = v0;
            if if nat_le(a1484, a1485) {
                let a1486 = v0;
                let a1487 = 122u64;
                nat_le(a1486, a1487)
            } else {
                false
            } {
                true
            } else {
                let a1488 = v0;
                let a1489 = 95u64;
                if nat_eq(a1488, a1489) {
                    true
                } else {
                    let a1490 = v0;
                    let a1491 = 45u64;
                    if nat_eq(a1490, a1491) {
                        true
                    } else {
                        let a1492 = v0;
                        let a1493 = 46u64;
                        nat_eq(a1492, a1493)
                    }
                }
            }
        }
    }
}

pub fn f488() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f489(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt123, v6: List<Adt116>) -> R<Result<(List<Adt116>, Adt123), Adt101>> {
    let m1494 = f562(v0.clone(), v1, v2.clone());
    match m1494 {
        None => {
            let m1495 = f563(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1495 {
                Adt156::C0(v8) => {
                    Ok(Err::<(List<Adt116>, Adt123), Adt101>(v8))
                }
                Adt156::C1(v9) => {
                    Ok(Ok::<(List<Adt116>, Adt123), Adt101>((f564(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt116>, Adt123), Adt101>(v7))
        }
    }
}

pub fn f490(v0: Str) -> R<bool> {
    let m1497 = f565(f10({
        let a1496 = v0;
        utf8_encode(a1496)
    })?)?;
    match m1497 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f491(v0: Adt40) -> bool {
    let m1499 = match v0 {
        Adt40::C0(_, _, h1498) => {
            h1498
        }
    };
    match m1499 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f492(v0: Adt52) -> R<Result<Adt35, Adt92>> {
    let m1501 = match v0.clone() {
        Adt52::C0(_, h1500) => {
            h1500
        }
    };
    match m1501.uncons() {
        None => {
            Ok(Ok::<Adt35, Adt92>(Adt35::C2(match v0 {
                Adt52::C0(h1502, _) => {
                    h1502
                }
            })))
        }
        Some((_, _)) => {
            if f566(match v0.clone() {
                Adt52::C0(_, h1503) => {
                    h1503
                }
            })? {
                Ok(Ok::<Adt35, Adt92>(Adt35::C3(v0)))
            } else {
                Ok(Err::<Adt35, Adt92>(Adt92::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f493(v0: Adt5, v1: Adt52) -> R<Result<Adt35, Adt92>> {
    let m1507 = {
        let m1506 = match v1.clone() {
            Adt52::C0(h1505, _) => {
                h1505
            }
        };
        match m1506 {
            Adt40::C0(_, h1504, _) => {
                h1504
            }
        }
    };
    match m1507 {
        None => {
            Ok(Err::<Adt35, Adt92>(Adt92::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1510 = f225(Adt6::C5(match v0.clone() {
                Adt5::C0(h1508) => {
                    h1508
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1509) => {
                    h1509
                }
            }), v2)?;
            match m1510 {
                Adt117::C0 => {
                    f492(v1)
                }
                Adt117::C1(_, _) => {
                    Ok(Err::<Adt35, Adt92>(Adt92::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt117::C2(_) => {
                    Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt117::C3(_) => {
                    Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt117::C4 => {
                    Ok(Err::<Adt35, Adt92>(Adt92::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f494() -> i64 {
    60000i64
}

pub fn f495() -> i64 {
    1000i64
}

pub fn f496(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt118>) -> R<Adt141> {
    let m1511 = f567(v1.clone(), v2.clone())?;
    match m1511 {
        Adt141::C0(v4) => {
            Ok(Adt141::C0(v4))
        }
        Adt141::C1(v5) => {
            Ok(Adt141::C1(v5))
        }
        Adt141::C2(v6) => {
            Ok(Adt141::C2(v6))
        }
        Adt141::C3(_) => {
            f568(v0, v1, v2, v3)
        }
    }
}

pub fn f497(v0: List<Adt6>, v1: Adt6, v2: List<Adt118>, v3: u64, v4: List<Adt120>) -> Adt124 {
    let m1512 = v0;
    match m1512.uncons() {
        None => {
            let a1513 = v3;
            let a1514 = 1u64;
            if nat_eq(a1513, a1514) {
                Adt124::C0(Adt119::C0, v4)
            } else {
                Adt124::C0(Adt119::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt124::C0(Adt119::C0, List::cons(Adt120::C0(v5, v1.clone(), v2.clone()), List::cons(Adt120::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f498(v0: Option<Adt6>, v1: Adt6, v2: List<Adt118>, v3: List<Adt120>) -> Adt124 {
    let m1515 = v0;
    match m1515 {
        None => {
            Adt124::C0(Adt119::C0, v3)
        }
        Some(v4) => {
            Adt124::C0(Adt119::C0, List::cons(Adt120::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f499(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m1516 = f337(Str::lit("anyOf"), v0);
    match m1516 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            let m1517 = f289(v3);
            match m1517 {
                None => {
                    Adt141::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1518 = v4;
                    match m1518.uncons() {
                        None => {
                            Adt141::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt141::C3(List::cons(Adt120::C0(v5, v1.clone(), v2.clone()), List::cons(Adt120::C1(v6, v1, v2), List::<Adt120>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f500(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m1519 = f337(Str::lit("oneOf"), v0);
    match m1519 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            let m1520 = f289(v3);
            match m1520 {
                None => {
                    Adt141::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1521 = v4;
                    match m1521.uncons() {
                        None => {
                            Adt141::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt141::C3(List::cons(Adt120::C0(v5, v1.clone(), v2.clone()), List::cons(Adt120::C2(v6, v1, v2, 0u64), List::<Adt120>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f501(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m1522 = f337(Str::lit("not"), v0);
    match m1522 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            Adt141::C3(List::cons(Adt120::C0(v3, v1, v2.clone()), List::cons(Adt120::C3(v2), List::<Adt120>::nil())))
        }
    }
}

pub fn f502(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m1523 = f337(Str::lit("if"), v0.clone());
    match m1523 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            Adt141::C3(List::cons(Adt120::C0(v3, v1.clone(), v2.clone()), List::cons(Adt120::C4(f569(Str::lit("then"), v0.clone()), f569(Str::lit("else"), v0), v1, v2), List::<Adt120>::nil())))
        }
    }
}

pub fn f503(v0: List<Adt6>, v1: Adt6, v2: List<Adt118>, v3: List<Adt120>) -> Adt124 {
    let m1524 = v0;
    match m1524.uncons() {
        None => {
            Adt124::C0(Adt119::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt124::C0(Adt119::C0, List::cons(Adt120::C0(v4, v1.clone(), v2.clone()), List::cons(Adt120::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f504(v0: Adt118) -> R<Str> {
    let m1525 = v0;
    match m1525 {
        Adt118::C0(v1) => {
            f570(v1)
        }
        Adt118::C1(v2) => {
            let a1526 = f344(v2, 0i64)?;
            Ok(format_i64(a1526))
        }
    }
}

pub fn f505(v0: Str) -> R<Str> {
    let m1529 = {
        let a1528 = f440(f217(), f459(f571(f10({
            let a1527 = v0;
            utf8_encode(a1527)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1528)
    };
    match m1529 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f506(p1532: List<(Str, Adt6)>, p1533: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1531 = (p1532, p1533);
    {
        let mut m1531 = m1531;
        loop {
            let step = {
                let (v0, v1) = m1531;
                let m1530 = v0;
                match m1530.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: Adt41) -> R<Option<Adt6>> {
    let m1534 = v0;
    match m1534 {
        Adt41::C0(v1) => {
            Ok(Some(f572(v1)))
        }
        Adt41::C1(v2) => {
            Ok(Some(f573(Str::lit("image"), match v2.clone() {
                Adt45::C0(h1535, _, _, _) => {
                    h1535
                }
            }, match v2.clone() {
                Adt45::C0(_, h1536, _, _) => {
                    h1536
                }
            }, match v2.clone() {
                Adt45::C0(_, _, h1537, _) => {
                    h1537
                }
            }, match v2 {
                Adt45::C0(_, _, _, h1538) => {
                    h1538
                }
            })))
        }
        Adt41::C2(v3) => {
            Ok(Some(f573(Str::lit("audio"), match v3.clone() {
                Adt46::C0(h1539, _, _, _) => {
                    h1539
                }
            }, match v3.clone() {
                Adt46::C0(_, h1540, _, _) => {
                    h1540
                }
            }, match v3.clone() {
                Adt46::C0(_, _, h1541, _) => {
                    h1541
                }
            }, match v3 {
                Adt46::C0(_, _, _, h1542) => {
                    h1542
                }
            })))
        }
        Adt41::C3(v4) => {
            Ok(Some(f574(v4)?))
        }
        Adt41::C4(v5) => {
            Ok(Some(f575(v5)))
        }
    }
}

pub fn f508(p1548: List<u64>, p1549: List<u64>) -> Option<List<u64>> {
    let m1547 = (p1548, p1549);
    {
        let mut m1547 = m1547;
        loop {
            let step = {
                let (v0, v1) = m1547;
                let m1543 = v0;
                match m1543.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1544 = v1;
                        match m1544.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1545 = v2;
                                let a1546 = v4;
                                if nat_eq(a1545, a1546) {
                                    Ok::<(List<u64>, List<u64>), Option<List<u64>>>((v3, v5))
                                } else {
                                    Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: Str) -> R<List<u64>> {
    f10({
        let a1550 = v0;
        utf8_encode(a1550)
    })
}

pub fn f510() -> Str {
    Str::lit("test://template/")
}

pub fn f511(v0: List<u64>) -> List<u64> {
    f459(v0, List::<u64>::nil())
}

pub fn f512() -> Str {
    Str::lit("data")
}

pub fn f513(v0: List<u64>) -> bool {
    let m1551 = v0;
    match m1551.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f514(p1556: List<u64>) -> bool {
    let m1555 = p1556;
    {
        let mut m1555 = m1555;
        loop {
            let step = {
                let v0 = m1555;
                let m1552 = v0;
                match m1552.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a1553 = v1;
                        let a1554 = 47u64;
                        if nat_eq(a1553, a1554) {
                            Err::<List<u64>, bool>(false)
                        } else {
                            Ok::<List<u64>, bool>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(p1561: Bytes, p1562: List<Bytes>) -> Bytes {
    let m1560 = (p1561, p1562);
    {
        let mut m1560 = m1560;
        loop {
            let step = {
                let (v0, v1) = m1560;
                let m1557 = v1;
                match m1557.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1558 = v2;
                            let a1559 = v0;
                            append_bytes(a1558, a1559)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(p1565: List<Bytes>, p1566: List<u64>) -> List<Bytes> {
    let m1564 = (p1565, p1566);
    {
        let mut m1564 = m1564;
        loop {
            let step = {
                let (v0, v1) = m1564;
                let m1563 = v1;
                match m1563.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f576(f577(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: Adt109, v1: Str) -> bool {
    let m1567 = v0;
    match m1567 {
        Adt109::C0 => {
            false
        }
        Adt109::C1(v2) => {
            let m1568 = f131(v2);
            match m1568 {
                None => {
                    false
                }
                Some(v3) => {
                    let a1569 = v3;
                    let a1570 = v1;
                    equal(a1569, a1570)
                }
            }
        }
    }
}

pub fn f518(p1576: List<u64>, p1577: List<u64>) -> bool {
    let m1575 = (p1576, p1577);
    {
        let mut m1575 = m1575;
        loop {
            let step = {
                let (v0, v1) = m1575;
                let m1571 = v0;
                match m1571.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m1572 = v1;
                        match m1572.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v4, v5)) => {
                                let a1573 = v2;
                                let a1574 = v4;
                                if nat_eq(a1573, a1574) {
                                    Ok::<(List<u64>, List<u64>), bool>((v3, v5))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1575 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f519(v0: Adt5) -> Option<Adt66> {
    let m1578 = f520(Str::lit("taskId"), v0.clone());
    match m1578 {
        None => {
            None::<Adt66>
        }
        Some(v1) => {
            let m1579 = f520(Str::lit("createdAt"), v0.clone());
            match m1579 {
                None => {
                    None::<Adt66>
                }
                Some(v2) => {
                    let m1580 = f520(Str::lit("lastUpdatedAt"), v0.clone());
                    match m1580 {
                        None => {
                            None::<Adt66>
                        }
                        Some(v3) => {
                            Some(Adt66::C0(v1, f520(Str::lit("statusMessage"), v0.clone()), v2, v3, f454(Str::lit("ttlMs"), v0.clone()), f454(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f520(v0: Str, v1: Adt5) -> Option<Str> {
    let m1581 = f229(v0, v1);
    match m1581 {
        Adt109::C0 => {
            None::<Str>
        }
        Adt109::C1(v2) => {
            f131(v2)
        }
    }
}

pub fn f521(v0: Str, v1: Adt5) -> Option<List<(Str, Adt67)>> {
    let m1582 = f452(v0, v1);
    match m1582 {
        None => {
            None::<List<(Str, Adt67)>>
        }
        Some(v2) => {
            f578(match v2 {
                Adt5::C0(h1583) => {
                    h1583
                }
            }, List::<(Str, Adt67)>::nil())
        }
    }
}

pub fn f522(v0: Adt5) -> Option<Adt83> {
    let m1584 = f454(Str::lit("code"), v0.clone());
    match m1584 {
        None => {
            None::<Adt83>
        }
        Some(v1) => {
            let m1585 = f520(Str::lit("message"), v0.clone());
            match m1585 {
                None => {
                    None::<Adt83>
                }
                Some(v2) => {
                    Some(Adt83::C0(v1, v2, {
                        let m1586 = f229(Str::lit("data"), v0);
                        match m1586 {
                            Adt109::C0 => {
                                None::<Adt6>
                            }
                            Adt109::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f523(v0: Adt5) -> Option<Adt89> {
    let m1587 = f452(Str::lit("completes"), v0.clone());
    match m1587 {
        None => {
            let m1588 = f452(Str::lit("fails"), v0);
            match m1588 {
                None => {
                    None::<Adt89>
                }
                Some(v2) => {
                    let m1589 = f522(v2);
                    match m1589 {
                        None => {
                            None::<Adt89>
                        }
                        Some(v3) => {
                            Some(Adt89::C1(v3))
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(Adt89::C0(v1))
        }
    }
}

pub fn f524(v0: Adt127, v1: List<Adt127>, v2: List<u64>) -> R<Adt152> {
    let m1590 = v0;
    match m1590 {
        Adt127::C0(v3) => {
            Ok(Adt152::C1(v1, f459(v3, v2)))
        }
        Adt127::C1(v4, v5) => {
            f579(v4, v5, v1, v2)
        }
        Adt127::C2(v6, v7) => {
            let m1591 = v7;
            match m1591.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C1(v6, v8), List::cons(Adt127::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt127::C3(v10, v11) => {
            let m1592 = v11.clone();
            match m1592.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt127::C4(v14, v15) => {
            let m1593 = v15;
            match m1593.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0({
                        let a1595 = f580({
                            let (h1594, _) = v16;
                            h1594
                        })?;
                        let a1596 = List::cons(58u64, List::<u64>::nil());
                        append_list(a1595, a1596)
                    }), List::cons(Adt127::C1(v14, {
                        let (_, h1597) = v16;
                        h1597
                    }), List::cons(Adt127::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt127::C5(v18, v19) => {
            let m1598 = v19.clone();
            match m1598.uncons() {
                None => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt152::C1(List::cons(Adt127::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt127::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f525(v0: Result<Adt144, Adt147>) -> Option<Adt93> {
    let m1599 = v0;
    match m1599 {
        Ok(_) => {
            None::<Adt93>
        }
        Err(v2) => {
            let m1600 = v2;
            match m1600 {
                Adt147::C0(_) => {
                    None::<Adt93>
                }
                Adt147::C1(_, _) => {
                    None::<Adt93>
                }
                Adt147::C2(_) => {
                    None::<Adt93>
                }
                Adt147::C3(_) => {
                    None::<Adt93>
                }
                Adt147::C4(v8) => {
                    f581(v8)
                }
            }
        }
    }
}

pub fn f526(v0: Adt16, v1: bool, v2: Adt143, v3: Adt93) -> R<Adt94> {
    if f582(match v3.clone() {
        Adt93::C0(h1601, _) => {
            h1601
        }
    }) {
        Ok(Adt94::C1(v3))
    } else {
        Ok(Adt94::C0(f26(f528(v0, v1, Adt143::C0(match v2 {
            Adt143::C0(h1602, _) => {
                h1602
            }
        }, Err::<Adt144, Adt147>(f583())))?)))
    }
}

pub fn f527(v0: Adt0, v1: bool, v2: Adt100) -> Adt94 {
    let m1604 = match v2.clone() {
        Adt100::C0(_, _, _, _, h1603) => {
            h1603
        }
    };
    match m1604 {
        None => {
            Adt94::C0(f26(v2))
        }
        Some(v3) => {
            f584(v0, v1, v3, v2)
        }
    }
}

pub fn f528(v0: Adt16, v1: bool, v2: Adt143) -> R<Adt100> {
    let m1606 = match v2.clone() {
        Adt143::C0(_, h1605) => {
            h1605
        }
    };
    match m1606 {
        Ok(v3) => {
            let m1607 = f585(v3.clone())?;
            match m1607 {
                None => {
                    Ok(f27())
                }
                Some(v4) => {
                    f586(v0, f587(match v3.clone() {
                        Adt144::C0(h1608, _, _) => {
                            h1608
                        }
                    }), f588(v1, f199(f589(), f590(match v2 {
                        Adt143::C0(h1609, _) => {
                            h1609
                        }
                    }, v4))?, f591(f592(match v3.clone() {
                        Adt144::C0(_, _, h1610) => {
                            h1610
                        }
                    }, List::<List<u64>>::nil())?, f593(f587(match v3 {
                        Adt144::C0(h1611, _, _) => {
                            h1611
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f199(f594(v5.clone()), f201(Some(match v2 {
                Adt143::C0(h1612, _) => {
                    h1612
                }
            }), f595(v5)?))
        }
    }
}

pub fn f529() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f530(p1615: List<List<u64>>) -> R<bool> {
    let m1614 = p1615;
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let v0 = m1614;
                let m1613 = v0;
                match m1613.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f596(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(v0: u64, p1620: List<u64>, p1621: List<u64>, p1622: List<List<u64>>) -> List<List<u64>> {
    let m1619 = (p1620, (p1621, p1622));
    {
        let mut m1619 = m1619;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1619;
                let m1616 = v1;
                match m1616.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f107(List::cons(f410(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a1617 = v4;
                        let a1618 = v0;
                        if nat_eq(a1617, a1618) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f410(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1619 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Adt0, v1: Fn9, v2: u64, v3: Result<Adt153, Adt147>) -> R<Result<Adt144, Adt147>> {
    let m1623 = v3;
    match m1623 {
        Ok(v5) => {
            f597(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt144, Adt147>(v4))
        }
    }
}

pub fn f533(v0: Option<Adt128>, v1: List<Str>, v2: Adt122) -> R<Result<Adt153, Adt147>> {
    Ok(f598(f599(match v2.clone() {
        Adt122::C0(_, h1624, _) => {
            h1624
        }
    }), f600(v0, v1, v2)?))
}

pub fn f534(v0: Adt130, v1: Adt114) -> Adt149 {
    let m1625 = v1;
    match m1625 {
        Adt114::C0 => {
            f540()
        }
        Adt114::C1 => {
            Adt149::C0(None::<Str>, Some(Adt129::C6(v0)))
        }
        Adt114::C2 => {
            Adt149::C0(None::<Str>, Some(Adt129::C7(v0)))
        }
        Adt114::C3(v2) => {
            let m1626 = f414(v2);
            match m1626 {
                None => {
                    Adt149::C0(None::<Str>, Some(Adt129::C6(v0)))
                }
                Some(v3) => {
                    Adt149::C0(Some(v3), None::<Adt129>)
                }
            }
        }
    }
}

pub fn f535() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f536() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f537(v0: Str) -> bool {
    let m1627 = f601(v0);
    match m1627 {
        Adt161::C0 => {
            false
        }
        Adt161::C1 => {
            true
        }
        Adt161::C2 => {
            true
        }
        Adt161::C3 => {
            true
        }
    }
}

pub fn f538(v0: Adt114) -> R<Adt114> {
    let m1628 = v0.clone();
    match m1628 {
        Adt114::C0 => {
            Ok(v0)
        }
        Adt114::C1 => {
            Ok(v0)
        }
        Adt114::C2 => {
            Ok(v0)
        }
        Adt114::C3(v1) => {
            let m1629 = f602(v1);
            match m1629 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m1630 = f565(v2)?;
                    match m1630 {
                        None => {
                            Ok(Adt114::C1)
                        }
                        Some(v3) => {
                            Ok(Adt114::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f539() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f540() -> Adt149 {
    Adt149::C0(None::<Str>, None::<Adt129>)
}

pub fn f541(v0: Option<Adt129>, v1: Option<Adt129>) -> Option<Adt129> {
    let m1631 = v0.clone();
    match m1631 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f542(v0: List<Adt98>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt131>> {
    Ok(f603(f604(v0, v1, v2, List::<Adt131>::nil())?, List::<Adt131>::nil()))
}

pub fn f543() -> Str {
    Str::lit("2026-07-28")
}

pub fn f544(v0: Adt7) -> R<Option<i64>> {
    let m1634 = f605(40u64, match v0.clone() {
        Adt7::C0(h1632, _) => {
            h1632
        }
    }, match v0 {
        Adt7::C0(_, h1633) => {
            h1633
        }
    })?;
    match m1634 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f606(v1))
        }
    }
}

pub fn f545(v0: Adt108, v1: Adt5) -> Result<Adt122, Adt112> {
    let m1635 = f229(Str::lit("method"), v1.clone());
    match m1635 {
        Adt109::C0 => {
            f396(Some(v0), f472(v1))
        }
        Adt109::C1(v2) => {
            let m1636 = f131(v2);
            match m1636 {
                None => {
                    f396(Some(v0), Adt113::C4)
                }
                Some(v3) => {
                    f607(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f546(v0: u64) -> R<u64> {
    let a1639 = {
        let a1637 = v0;
        let a1638 = 3u64;
        nat_mul(a1637, a1638)?
    };
    let a1640 = 1u64;
    nat_add(a1639, a1640)
}

pub fn f547(v0: u64, v1: u64, v2: u64) -> bool {
    let a1641 = v1;
    let a1642 = v0;
    if nat_le(a1641, a1642) {
        let a1643 = v0;
        let a1644 = v2;
        nat_le(a1643, a1644)
    } else {
        false
    }
}

pub fn f548(v0: Adt138) -> R<Adt155> {
    if f608(match v0.clone() {
        Adt138::C0(_, h1645, _, _, _, _, _) => {
            h1645
        }
    }) {
        let v1: i64 = {
            let a1651 = if match v0.clone() {
                Adt138::C0(_, _, _, _, _, h1646, _) => {
                    h1646
                }
            } {
                let a1648 = match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1647) => {
                        h1647
                    }
                };
                int_neg(a1648)?
            } else {
                match v0.clone() {
                    Adt138::C0(_, _, _, _, _, _, h1649) => {
                        h1649
                    }
                }
            };
            let a1652 = match v0.clone() {
                Adt138::C0(_, _, _, _, h1650, _, _) => {
                    h1650
                }
            };
            int_sub(a1651, a1652)?
        };
        let m1655 = {
            let a1653 = v1;
            let a1654 = -9999i64;
            compare(a1653, a1654)
        };
        if if match m1655 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m1658 = {
                let a1656 = 9999i64;
                let a1657 = v1;
                compare(a1656, a1657)
            };
            match m1658 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            }
        } {
            Ok(Adt155::C1)
        } else {
            Ok(Adt155::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt138::C0(h1659, _, _, _, _, _, _) => {
                    h1659
                }
            } {
                let a1661 = match v0 {
                    Adt138::C0(_, _, h1660, _, _, _, _) => {
                        h1660
                    }
                };
                int_neg(a1661)?
            } else {
                match v0 {
                    Adt138::C0(_, _, h1662, _, _, _, _) => {
                        h1662
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt155::C0)
    }
}

pub fn f549(v0: Adt6, v1: List<Adt140>, v2: u64) -> Adt135 {
    let m1663 = v1.clone();
    match m1663.uncons() {
        None => {
            Adt135::C0(Adt136::C9(v0), List::<Adt140>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m1664 = v3;
            match m1664 {
                Adt140::C0(v5) => {
                    Adt135::C0(Adt136::C2, List::cons(Adt140::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt140::C1(v6, v7) => {
                    let m1665 = v7;
                    match m1665 {
                        None => {
                            Adt135::C0(Adt136::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt135::C0(Adt136::C2, List::cons(Adt140::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f550(v0: u64) -> bool {
    let a1666 = v0;
    let a1667 = 32u64;
    if nat_eq(a1666, a1667) {
        true
    } else {
        let a1668 = v0;
        let a1669 = 10u64;
        if nat_eq(a1668, a1669) {
            true
        } else {
            let a1670 = v0;
            let a1671 = 13u64;
            if nat_eq(a1670, a1671) {
                true
            } else {
                let a1672 = v0;
                let a1673 = 9u64;
                nat_eq(a1672, a1673)
            }
        }
    }
}

pub fn f551(v0: Adt135, v1: u64, v2: u64) -> R<Adt151> {
    let a1674 = v2;
    let a1675 = 34u64;
    if nat_eq(a1674, a1675) {
        Ok(f555(v0, Adt136::C6(Adt137::C0, List::<u64>::nil(), false)))
    } else {
        let a1676 = v2;
        let a1677 = 123u64;
        if nat_eq(a1676, a1677) {
            f609(v0, Adt140::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt136::C3, v1)
        } else {
            let a1678 = v2;
            let a1679 = 91u64;
            if nat_eq(a1678, a1679) {
                f609(v0, Adt140::C0(List::<Adt6>::nil()), Adt136::C1, v1)
            } else {
                let a1680 = v2;
                let a1681 = 116u64;
                if nat_eq(a1680, a1681) {
                    Ok(f555(v0, Adt136::C7(0u64, 1u64)))
                } else {
                    let a1682 = v2;
                    let a1683 = 102u64;
                    if nat_eq(a1682, a1683) {
                        Ok(f555(v0, Adt136::C7(1u64, 1u64)))
                    } else {
                        let a1684 = v2;
                        let a1685 = 110u64;
                        if nat_eq(a1684, a1685) {
                            Ok(f555(v0, Adt136::C7(2u64, 1u64)))
                        } else {
                            let a1686 = v2;
                            let a1687 = 45u64;
                            if nat_eq(a1686, a1687) {
                                Ok(f555(v0, f610(true, Adt139::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a1688 = v2;
                                let a1689 = 48u64;
                                if nat_eq(a1688, a1689) {
                                    Ok(f555(v0, f610(false, Adt139::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f547(v2, 49u64, 57u64) {
                                        Ok(f555(v0, f610(false, Adt139::C2, f611({
                                            let a1690 = v2;
                                            let a1691 = 48u64;
                                            nat_sub(a1690, a1691)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f556(Adt111::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f552(v0: Adt135, v1: u64) -> Adt151 {
    let m1693 = match v0.clone() {
        Adt135::C0(_, h1692, _) => {
            h1692
        }
    };
    match m1693.uncons() {
        None => {
            f556(Adt111::C2(v1))
        }
        Some((v2, v3)) => {
            let m1694 = v2;
            match m1694 {
                Adt140::C0(v4) => {
                    Adt151::C1(f549(Adt6::C4(f436(v4, List::<Adt6>::nil())), v3, {
                        let a1696 = match v0 {
                            Adt135::C0(_, _, h1695) => {
                                h1695
                            }
                        };
                        let a1697 = 1u64;
                        nat_sub(a1696, a1697)
                    }))
                }
                Adt140::C1(_, _) => {
                    f556(Adt111::C2(v1))
                }
            }
        }
    }
}

pub fn f553(v0: Adt135, v1: u64, v2: u64) -> Adt151 {
    let m1699 = match v0.clone() {
        Adt135::C0(h1698, _, _) => {
            h1698
        }
    };
    match m1699 {
        Adt136::C0 => {
            f556(Adt111::C2(v1))
        }
        Adt136::C1 => {
            f556(Adt111::C2(v1))
        }
        Adt136::C2 => {
            if f550(v2) {
                Adt151::C1(v0)
            } else {
                let a1700 = v2;
                let a1701 = 44u64;
                if nat_eq(a1700, a1701) {
                    f612(v0, v1)
                } else {
                    let a1702 = v2;
                    let a1703 = 93u64;
                    if nat_eq(a1702, a1703) {
                        f552(v0, v1)
                    } else {
                        let a1704 = v2;
                        let a1705 = 125u64;
                        if nat_eq(a1704, a1705) {
                            f554(v0, v1)
                        } else {
                            f556(Adt111::C2(v1))
                        }
                    }
                }
            }
        }
        Adt136::C3 => {
            f556(Adt111::C2(v1))
        }
        Adt136::C4 => {
            f556(Adt111::C2(v1))
        }
        Adt136::C5 => {
            f556(Adt111::C2(v1))
        }
        Adt136::C6(_, _, _) => {
            f556(Adt111::C2(v1))
        }
        Adt136::C7(_, _) => {
            f556(Adt111::C2(v1))
        }
        Adt136::C8(_) => {
            f556(Adt111::C2(v1))
        }
        Adt136::C9(_) => {
            if f550(v2) {
                Adt151::C1(v0)
            } else {
                f556(Adt111::C10(v1))
            }
        }
    }
}

pub fn f554(v0: Adt135, v1: u64) -> Adt151 {
    let m1707 = match v0.clone() {
        Adt135::C0(_, h1706, _) => {
            h1706
        }
    };
    match m1707.uncons() {
        None => {
            f556(Adt111::C2(v1))
        }
        Some((v2, v3)) => {
            let m1708 = v2;
            match m1708 {
                Adt140::C0(_) => {
                    f556(Adt111::C2(v1))
                }
                Adt140::C1(v5, _) => {
                    Adt151::C1(f549(Adt6::C5(f506(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a1710 = match v0 {
                            Adt135::C0(_, _, h1709) => {
                                h1709
                            }
                        };
                        let a1711 = 1u64;
                        nat_sub(a1710, a1711)
                    }))
                }
            }
        }
    }
}

pub fn f555(v0: Adt135, v1: Adt136) -> Adt151 {
    Adt151::C1(f613(v0, v1))
}

pub fn f556(v0: Adt111) -> Adt151 {
    Adt151::C0(v0)
}

pub fn f557(v0: Adt135, v1: u64, v2: u64, v3: Adt137, v4: List<u64>, v5: bool) -> R<Adt151> {
    let m1712 = v3;
    match m1712 {
        Adt137::C0 => {
            let a1713 = v2;
            let a1714 = 34u64;
            if nat_eq(a1713, a1714) {
                Ok(f614(v0, v1, v4, v5))
            } else {
                let a1715 = v2;
                let a1716 = 92u64;
                if nat_eq(a1715, a1716) {
                    Ok(f555(v0, Adt136::C6(Adt137::C1, v4, v5)))
                } else {
                    let a1717 = v2;
                    let a1718 = 32u64;
                    if nat_lt(a1717, a1718) {
                        Ok(f556(Adt111::C4(v1)))
                    } else {
                        Ok(f555(v0, Adt136::C6(Adt137::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt137::C1 => {
            let a1719 = v2;
            let a1720 = 117u64;
            if nat_eq(a1719, a1720) {
                Ok(f555(v0, Adt136::C6(Adt137::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a1721 = f615(v2);
                let a1722 = 0u64;
                if nat_eq(a1721, a1722) {
                    Ok(f556(Adt111::C5(v1)))
                } else {
                    Ok(f555(v0, Adt136::C6(Adt137::C0, List::cons(f615(v2), v4), v5)))
                }
            }
        }
        Adt137::C2(v6, v7, v8) => {
            let a1723 = 15u64;
            let a1724 = f616(v2);
            if nat_lt(a1723, a1724) {
                Ok(f556(Adt111::C5(v1)))
            } else {
                let a1727 = {
                    let a1725 = v6;
                    let a1726 = 1u64;
                    nat_add(a1725, a1726)?
                };
                let a1728 = 4u64;
                if nat_eq(a1727, a1728) {
                    f617(v0, v1, {
                        let a1731 = {
                            let a1729 = v7;
                            let a1730 = 16u64;
                            nat_mul(a1729, a1730)?
                        };
                        let a1732 = f616(v2);
                        nat_add(a1731, a1732)?
                    }, v8, v4, v5)
                } else {
                    Ok(f555(v0, Adt136::C6(Adt137::C2({
                        let a1733 = v6;
                        let a1734 = 1u64;
                        nat_add(a1733, a1734)?
                    }, {
                        let a1737 = {
                            let a1735 = v7;
                            let a1736 = 16u64;
                            nat_mul(a1735, a1736)?
                        };
                        let a1738 = f616(v2);
                        nat_add(a1737, a1738)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt137::C3(v9) => {
            let a1739 = v2;
            let a1740 = 92u64;
            if nat_eq(a1739, a1740) {
                Ok(f555(v0, Adt136::C6(Adt137::C4(v9), v4, v5)))
            } else {
                Ok(f556(Adt111::C6(v1)))
            }
        }
        Adt137::C4(v10) => {
            let a1741 = v2;
            let a1742 = 117u64;
            if nat_eq(a1741, a1742) {
                Ok(f555(v0, Adt136::C6(Adt137::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f556(Adt111::C6(v1)))
            }
        }
    }
}

pub fn f558(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt151> {
    let a1743 = v2;
    let a1744 = f618(v3, v4);
    if nat_eq(a1743, a1744) {
        let a1748 = {
            let a1745 = v4;
            let a1746 = 1u64;
            nat_add(a1745, a1746)?
        };
        let a1749 = {
            let a1747 = f619(v3);
            length_list(a1747)
        };
        if nat_eq(a1748, a1749) {
            Ok(f620(v0, f621(v3)))
        } else {
            Ok(f555(v0, Adt136::C7(v3, {
                let a1750 = v4;
                let a1751 = 1u64;
                nat_add(a1750, a1751)?
            })))
        }
    } else {
        Ok(f556(Adt111::C2(v1)))
    }
}

pub fn f559(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt151> {
    let m1753 = match v3.clone() {
        Adt138::C0(_, h1752, _, _, _, _, _) => {
            h1752
        }
    };
    match m1753 {
        Adt139::C0 => {
            let a1754 = v2;
            let a1755 = 48u64;
            if nat_eq(a1754, a1755) {
                Ok(f622(v0, v3, Adt139::C1))
            } else {
                if f547(v2, 49u64, 57u64) {
                    f623(v0, v1, v2, v3, false)
                } else {
                    Ok(f556(Adt111::C7(v1)))
                }
            }
        }
        Adt139::C1 => {
            let a1756 = v2;
            let a1757 = 46u64;
            if nat_eq(a1756, a1757) {
                Ok(f622(v0, v3, Adt139::C3))
            } else {
                if f624(v2) {
                    Ok(f622(v0, v3, Adt139::C5))
                } else {
                    if f625(v2) {
                        Ok(f556(Adt111::C7(v1)))
                    } else {
                        f626(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C2 => {
            if f625(v2) {
                f623(v0, v1, v2, v3, false)
            } else {
                let a1758 = v2;
                let a1759 = 46u64;
                if nat_eq(a1758, a1759) {
                    Ok(f622(v0, v3, Adt139::C3))
                } else {
                    if f624(v2) {
                        Ok(f622(v0, v3, Adt139::C5))
                    } else {
                        f626(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt139::C3 => {
            if f625(v2) {
                f623(v0, v1, v2, v3, true)
            } else {
                Ok(f556(Adt111::C7(v1)))
            }
        }
        Adt139::C4 => {
            if f625(v2) {
                f623(v0, v1, v2, v3, true)
            } else {
                if f624(v2) {
                    Ok(f622(v0, v3, Adt139::C5))
                } else {
                    f626(v0, v1, v2, v3)
                }
            }
        }
        Adt139::C5 => {
            let a1760 = v2;
            let a1761 = 43u64;
            if nat_eq(a1760, a1761) {
                Ok(f555(v0, f610(match v3.clone() {
                    Adt138::C0(h1762, _, _, _, _, _, _) => {
                        h1762
                    }
                }, Adt139::C6, match v3.clone() {
                    Adt138::C0(_, _, h1763, _, _, _, _) => {
                        h1763
                    }
                }, match v3.clone() {
                    Adt138::C0(_, _, _, h1764, _, _, _) => {
                        h1764
                    }
                }, match v3 {
                    Adt138::C0(_, _, _, _, h1765, _, _) => {
                        h1765
                    }
                }, false, 0i64)))
            } else {
                let a1766 = v2;
                let a1767 = 45u64;
                if nat_eq(a1766, a1767) {
                    Ok(f555(v0, f610(match v3.clone() {
                        Adt138::C0(h1768, _, _, _, _, _, _) => {
                            h1768
                        }
                    }, Adt139::C6, match v3.clone() {
                        Adt138::C0(_, _, h1769, _, _, _, _) => {
                            h1769
                        }
                    }, match v3.clone() {
                        Adt138::C0(_, _, _, h1770, _, _, _) => {
                            h1770
                        }
                    }, match v3 {
                        Adt138::C0(_, _, _, _, h1771, _, _) => {
                            h1771
                        }
                    }, true, 0i64)))
                } else {
                    if f625(v2) {
                        f627(v0, v1, v2, Adt138::C0(match v3.clone() {
                            Adt138::C0(h1772, _, _, _, _, _, _) => {
                                h1772
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, h1773, _, _, _, _, _) => {
                                h1773
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, h1774, _, _, _, _) => {
                                h1774
                            }
                        }, match v3.clone() {
                            Adt138::C0(_, _, _, h1775, _, _, _) => {
                                h1775
                            }
                        }, match v3 {
                            Adt138::C0(_, _, _, _, h1776, _, _) => {
                                h1776
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f556(Adt111::C7(v1)))
                    }
                }
            }
        }
        Adt139::C6 => {
            if f625(v2) {
                f627(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1777, _) => {
                        h1777
                    }
                })
            } else {
                Ok(f556(Adt111::C7(v1)))
            }
        }
        Adt139::C7 => {
            if f625(v2) {
                f627(v0, v1, v2, v3.clone(), match v3 {
                    Adt138::C0(_, _, _, _, _, h1778, _) => {
                        h1778
                    }
                })
            } else {
                f626(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f560(v0: u64) -> bool {
    let a1779 = v0;
    let a1780 = 9u64;
    if nat_eq(a1779, a1780) {
        true
    } else {
        let a1781 = v0;
        let a1782 = 32u64;
        nat_eq(a1781, a1782)
    }
}

pub fn f561(v0: u64) -> bool {
    let a1783 = 47u64;
    let a1784 = v0;
    if nat_lt(a1783, a1784) {
        let a1785 = v0;
        let a1786 = 58u64;
        nat_lt(a1785, a1786)
    } else {
        false
    }
}

pub fn f562(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt101> {
    let m1787 = f628(f629(), v2.clone());
    match m1787 {
        None => {
            let m1788 = f630(f631(), v2.clone());
            match m1788 {
                None => {
                    f632(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt101::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt101::C4(v0, v3))
        }
    }
}

pub fn f563(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt123) -> R<Adt156> {
    let m1789 = f337(Str::lit("x-mcp-header"), v1.clone());
    match m1789 {
        Adt109::C0 => {
            Ok(Adt156::C1(v4))
        }
        Adt109::C1(v5) => {
            f633(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f564(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt116>) -> List<Adt116> {
    f634(Str::lit("properties"), v0.clone(), v1, v2, f634(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f635(Str::lit("items"), v0.clone(), f635(Str::lit("additionalProperties"), v0.clone(), f636(Str::lit("allOf"), v0.clone(), f636(Str::lit("anyOf"), v0.clone(), f636(Str::lit("oneOf"), v0.clone(), f635(Str::lit("not"), v0.clone(), f635(Str::lit("if"), v0.clone(), f635(Str::lit("then"), v0.clone(), f635(Str::lit("else"), v0, v3)))))))))))
}

pub fn f565(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1791 = f637({
        let a1790 = v0.clone();
        length_list(a1790)
    }, v0, List::<u64>::nil())?;
    match m1791 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f459(v1, List::<u64>::nil())))
        }
    }
}

pub fn f566(v0: List<Adt53>) -> R<bool> {
    let m1792 = v0;
    match m1792.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f638(v2, match v1 {
                Adt53::C0(h1793, _, _) => {
                    h1793
                }
            })
        }
    }
}

pub fn f567(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt141> {
    Ok(f639(List::cons((Str::lit("type"), f640(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f641(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f642(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f643(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f643(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f643(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f643(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1794 = f644(v0.clone(), v1.clone())?;
        let a1795 = List::cons((Str::lit("required"), f645(v0, v1)), List::<(Str, Adt157)>::nil());
        append_list(a1794, a1795)
    })))))))))
}

pub fn f568(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt118>) -> R<Adt141> {
    Ok(f646(f647(v0, v1.clone(), v2.clone(), v3.clone()), f646(f648(v1.clone(), v2.clone(), v3.clone()), f646(f649(v1.clone(), v2.clone(), v3.clone()), f646(f650(v1.clone(), v2.clone(), v3.clone())?, f646(f651(v1.clone(), v2.clone(), v3.clone()), f646(f652(Adt121::C0, f499(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f646(f652(Adt121::C1, f500(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f646(f652(Adt121::C2, f501(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f652(Adt121::C3, f502(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f569(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1796 = f337(v0, v1);
    match m1796 {
        Adt109::C0 => {
            None::<Adt6>
        }
        Adt109::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f570(v0: Str) -> R<Str> {
    let m1799 = {
        let a1798 = f440(f217(), f459(f653(f10({
            let a1797 = v0.clone();
            utf8_encode(a1797)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1798)
    };
    match m1799 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f571(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f654(v0, 0u64, v1)
}

pub fn f572(v0: Adt42) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt42::C0(h1800, _, _) => {
            h1800
        }
    })), {
        let a1803 = f655(match v0.clone() {
            Adt42::C0(_, h1801, _) => {
                h1801
            }
        });
        let a1804 = f656(match v0 {
            Adt42::C0(_, _, h1802) => {
                h1802
            }
        });
        append_list(a1803, a1804)
    })))
}

pub fn f573(v0: Str, v1: Str, v2: Str, v3: Option<Adt43>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a1805 = f655(v3);
        let a1806 = f656(v4);
        append_list(a1805, a1806)
    }))))
}

pub fn f574(v0: Adt47) -> R<Adt6> {
    Ok(Adt6::C5({
        let a1830 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt47::C0(h1807, _, _, _, _, _, _, _, _) => {
                h1807
            }
        })), List::<(Str, Adt6)>::nil()));
        let a1831 = {
            let a1828 = f657(Str::lit("title"), match v0.clone() {
                Adt47::C0(_, h1808, _, _, _, _, _, _, _) => {
                    h1808
                }
            });
            let a1829 = {
                let a1826 = f658(match v0.clone() {
                    Adt47::C0(_, _, h1809, _, _, _, _, _, _) => {
                        h1809
                    }
                });
                let a1827 = {
                    let a1824 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt47::C0(_, _, _, h1810, _, _, _, _, _) => {
                            h1810
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a1825 = {
                        let a1822 = f657(Str::lit("description"), match v0.clone() {
                            Adt47::C0(_, _, _, _, h1811, _, _, _, _) => {
                                h1811
                            }
                        });
                        let a1823 = {
                            let a1820 = f657(Str::lit("mimeType"), match v0.clone() {
                                Adt47::C0(_, _, _, _, _, h1812, _, _, _) => {
                                    h1812
                                }
                            });
                            let a1821 = {
                                let a1818 = f655(match v0.clone() {
                                    Adt47::C0(_, _, _, _, _, _, h1813, _, _) => {
                                        h1813
                                    }
                                });
                                let a1819 = {
                                    let a1816 = f659(match v0.clone() {
                                        Adt47::C0(_, _, _, _, _, _, _, h1814, _) => {
                                            h1814
                                        }
                                    })?;
                                    let a1817 = f656(match v0 {
                                        Adt47::C0(_, _, _, _, _, _, _, _, h1815) => {
                                            h1815
                                        }
                                    });
                                    append_list(a1816, a1817)
                                };
                                append_list(a1818, a1819)
                            };
                            append_list(a1820, a1821)
                        };
                        append_list(a1822, a1823)
                    };
                    append_list(a1824, a1825)
                };
                append_list(a1826, a1827)
            };
            append_list(a1828, a1829)
        };
        append_list(a1830, a1831)
    }))
}

pub fn f575(v0: Adt48) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f660(match v0.clone() {
        Adt48::C0(h1832, _, _) => {
            h1832
        }
    })), {
        let a1835 = f655(match v0.clone() {
            Adt48::C0(_, h1833, _) => {
                h1833
            }
        });
        let a1836 = f656(match v0 {
            Adt48::C0(_, _, h1834) => {
                h1834
            }
        });
        append_list(a1835, a1836)
    })))
}

pub fn f576(p1845: Bytes, p1846: List<Bytes>) -> List<Bytes> {
    let m1844 = (p1845, p1846);
    {
        let mut m1844 = m1844;
        loop {
            let step = {
                let (v0, v1) = m1844;
                let m1837 = v1;
                match m1837.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1840 = {
                            let a1838 = v2.clone();
                            length_bytes(a1838)
                        };
                        let a1841 = {
                            let a1839 = v0.clone();
                            length_bytes(a1839)
                        };
                        if nat_eq(a1840, a1841) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1842 = v2;
                                let a1843 = v0;
                                append_bytes(a1842, a1843)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1844 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(v0: u64) -> Bytes {
    let m1850 = {
        let a1847 = f661();
        let a1848 = v0;
        let a1849 = 1u64;
        slice_bytes(a1847, a1848, a1849)
    };
    match m1850 {
        None => {
            f217()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f578(p1856: List<(Str, Adt6)>, p1857: List<(Str, Adt67)>) -> Option<List<(Str, Adt67)>> {
    let m1855 = (p1856, p1857);
    {
        let mut m1855 = m1855;
        loop {
            let step = {
                let (v0, v1) = m1855;
                let m1851 = v0;
                match m1851.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt67)>), Option<List<(Str, Adt67)>>>(Some(f456(v1, List::<(Str, Adt67)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1853 = f662({
                            let (_, h1852) = v2;
                            h1852
                        });
                        match m1853 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt67)>), Option<List<(Str, Adt67)>>>(None::<List<(Str, Adt67)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt67)>), Option<List<(Str, Adt67)>>>((v3, List::cons(({
                                    let (h1854, _) = v2;
                                    h1854
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1855 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: u64, v1: Adt6, v2: List<Adt127>, v3: List<u64>) -> R<Adt152> {
    let m1858 = v1;
    match m1858 {
        Adt6::C0 => {
            Ok(Adt152::C1(v2, f459(f663(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt152::C1(v2, f459(if v4 {
                f663(0u64)
            } else {
                f663(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f664(v5.clone())? {
                Ok(Adt152::C1(v2, f459(f665(v5)?, v3)))
            } else {
                Ok(Adt152::C0(Adt111::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt152::C1(v2, f459(f580(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1859 = v0;
            let a1860 = f666();
            if nat_lt(a1859, a1860) {
                Ok(Adt152::C1(f667(91u64, Adt127::C2({
                    let a1861 = v0;
                    let a1862 = 1u64;
                    nat_add(a1861, a1862)?
                }, v7), v2), v3))
            } else {
                Ok(Adt152::C0(Adt111::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1863 = v0;
            let a1864 = f666();
            if nat_lt(a1863, a1864) {
                Ok(Adt152::C1(f667(123u64, Adt127::C4({
                    let a1865 = v0;
                    let a1866 = 1u64;
                    nat_add(a1865, a1866)?
                }, v8), v2), v3))
            } else {
                Ok(Adt152::C0(Adt111::C12))
            }
        }
    }
}

pub fn f580(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1868 = f459(f668(f10({
            let a1867 = v0;
            utf8_encode(a1867)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1869 = List::cons(34u64, List::<u64>::nil());
        append_list(a1868, a1869)
    }))
}

pub fn f581(v0: Adt92) -> Option<Adt93> {
    let m1870 = v0;
    match m1870 {
        Adt92::C0(_) => {
            None::<Adt93>
        }
        Adt92::C1(_) => {
            None::<Adt93>
        }
        Adt92::C2(_) => {
            None::<Adt93>
        }
        Adt92::C3(_) => {
            None::<Adt93>
        }
        Adt92::C4(_) => {
            None::<Adt93>
        }
        Adt92::C5(_) => {
            None::<Adt93>
        }
        Adt92::C6(_) => {
            None::<Adt93>
        }
        Adt92::C7(_) => {
            None::<Adt93>
        }
        Adt92::C8(_) => {
            None::<Adt93>
        }
        Adt92::C9(_) => {
            None::<Adt93>
        }
        Adt92::C10 => {
            None::<Adt93>
        }
        Adt92::C11 => {
            None::<Adt93>
        }
        Adt92::C12(_) => {
            None::<Adt93>
        }
        Adt92::C13 => {
            None::<Adt93>
        }
        Adt92::C14 => {
            None::<Adt93>
        }
        Adt92::C15(v12) => {
            Some(v12)
        }
    }
}

pub fn f582(v0: u64) -> bool {
    let a1871 = v0;
    let a1872 = f669();
    nat_lt(a1871, a1872)
}

pub fn f583() -> Adt147 {
    Adt147::C4(Adt92::C9(Str::lit("the effect tag does not fit thirty-two bits")))
}

pub fn f584(v0: Adt0, v1: bool, v2: List<u64>, v3: Adt100) -> Adt94 {
    if v1 {
        Adt94::C0(f26(f670(v3)))
    } else {
        if f582(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, _, h1873) => {
                h1873
            }
        }) {
            Adt94::C1(f671(v0, v2))
        } else {
            f672()
        }
    }
}

pub fn f585(v0: Adt144) -> R<Option<Adt5>> {
    let m1875 = f673(match v0.clone() {
        Adt144::C0(h1874, _, _) => {
            h1874
        }
    })?;
    match m1875 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a1878 = v1;
                let a1879 = {
                    let m1877 = f674(v0);
                    match m1877 {
                        Adt5::C0(h1876) => {
                            h1876
                        }
                    }
                };
                append_list(a1878, a1879)
            })))
        }
    }
}

pub fn f586(v0: Adt16, v1: Option<Adt90>, v2: Adt100) -> R<Adt100> {
    let m1880 = v1;
    match m1880 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a1882 = match v2.clone() {
                Adt100::C0(h1881, _, _, _, _) => {
                    h1881
                }
            };
            let a1883 = f589();
            if nat_eq(a1882, a1883) {
                Ok(f675(f676(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f587(v0: Adt35) -> Option<Adt90> {
    let m1884 = v0;
    match m1884 {
        Adt35::C0(_) => {
            None::<Adt90>
        }
        Adt35::C1(_) => {
            None::<Adt90>
        }
        Adt35::C2(_) => {
            None::<Adt90>
        }
        Adt35::C3(_) => {
            None::<Adt90>
        }
        Adt35::C4(_) => {
            None::<Adt90>
        }
        Adt35::C5(_) => {
            None::<Adt90>
        }
        Adt35::C6(_) => {
            None::<Adt90>
        }
        Adt35::C7(_) => {
            None::<Adt90>
        }
        Adt35::C8(_) => {
            None::<Adt90>
        }
        Adt35::C9(_) => {
            None::<Adt90>
        }
        Adt35::C10(_) => {
            None::<Adt90>
        }
        Adt35::C11 => {
            None::<Adt90>
        }
        Adt35::C12(_) => {
            None::<Adt90>
        }
        Adt35::C13(v13) => {
            Some(Adt90::C0(match v13 {
                Adt85::C0(_, _, h1885) => {
                    h1885
                }
            }, Adt91::C0, false))
        }
        Adt35::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f588(v0: bool, v1: Adt100, v2: Option<List<List<u64>>>) -> Adt100 {
    let m1886 = v2;
    match m1886 {
        None => {
            f27()
        }
        Some(v3) => {
            f677(v0, v1, f107(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f589() -> u64 {
    200u64
}

pub fn f590(v0: Adt108, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f387())), List::cons((Str::lit("id"), f460(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h1887) => {
            h1887
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f591(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m1888 = v0;
    match m1888 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m1889 = v1;
            match m1889 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f107(v3, v2))
                }
            }
        }
    }
}

pub fn f592(p1893: List<Adt146>, p1894: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m1892 = (p1893, p1894);
    {
        let mut m1892 = m1892;
        loop {
            let step = {
                let (v0, v1) = m1892;
                let m1890 = v0;
                match m1890.uncons() {
                    None => {
                        Err::<(List<Adt146>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1891 = f678(v2)?;
                        match m1891 {
                            None => {
                                Err::<(List<Adt146>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt146>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Option<Adt90>) -> R<Option<List<List<u64>>>> {
    let m1895 = v0;
    match m1895 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f679(v1)
        }
    }
}

pub fn f594(v0: Adt147) -> u64 {
    let m1896 = v0;
    match m1896 {
        Adt147::C0(_) => {
            f305()
        }
        Adt147::C1(_, _) => {
            f305()
        }
        Adt147::C2(_) => {
            f305()
        }
        Adt147::C3(_) => {
            f680()
        }
        Adt147::C4(v6) => {
            f681(v6)
        }
    }
}

pub fn f595(v0: Adt147) -> R<Adt83> {
    Ok(Adt83::C0(f682(v0.clone()), f683(v0.clone()), f684(v0)?))
}

pub fn f596(v0: List<u64>) -> R<bool> {
    let m1897 = f531(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1897.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f685(v2)? {
                Ok(false)
            } else {
                f400(f402(v1), f686())
            }
        }
    }
}

pub fn f597(v0: Adt0, v1: Fn9, v2: u64, v3: Adt153) -> R<Result<Adt144, Adt147>> {
    f687(v0.clone(), v1, v2, v3.clone(), f688(match v0 {
        Adt0::C0(_, _, h1898, _, _, _, _, _) => {
            h1898
        }
    }, match v3 {
        Adt153::C0(_, h1899, _, _) => {
            h1899
        }
    }))
}

pub fn f598(v0: Adt158, v1: Adt159) -> Result<Adt153, Adt147> {
    let m1900 = v1;
    match m1900 {
        Adt159::C0(v3) => {
            f689(v3, v0)
        }
        Adt159::C1(v2) => {
            Err::<Adt153, Adt147>(v2)
        }
    }
}

pub fn f599(v0: Str) -> Adt158 {
    let a1901 = v0.clone();
    let a1902 = Str::lit("server/discover");
    if equal(a1901, a1902) {
        Adt158::C1(Adt154::C0)
    } else {
        let a1903 = v0.clone();
        let a1904 = Str::lit("tools/list");
        if equal(a1903, a1904) {
            Adt158::C1(Adt154::C1)
        } else {
            let a1905 = v0.clone();
            let a1906 = Str::lit("tools/call");
            if equal(a1905, a1906) {
                Adt158::C1(Adt154::C2)
            } else {
                let a1907 = v0.clone();
                let a1908 = Str::lit("resources/list");
                if equal(a1907, a1908) {
                    Adt158::C1(Adt154::C3)
                } else {
                    let a1909 = v0.clone();
                    let a1910 = Str::lit("resources/read");
                    if equal(a1909, a1910) {
                        Adt158::C1(Adt154::C4)
                    } else {
                        let a1911 = v0.clone();
                        let a1912 = Str::lit("resources/templates/list");
                        if equal(a1911, a1912) {
                            Adt158::C1(Adt154::C5)
                        } else {
                            let a1913 = v0.clone();
                            let a1914 = Str::lit("prompts/list");
                            if equal(a1913, a1914) {
                                Adt158::C1(Adt154::C6)
                            } else {
                                let a1915 = v0.clone();
                                let a1916 = Str::lit("prompts/get");
                                if equal(a1915, a1916) {
                                    Adt158::C1(Adt154::C7)
                                } else {
                                    let a1917 = v0.clone();
                                    let a1918 = Str::lit("completion/complete");
                                    if equal(a1917, a1918) {
                                        Adt158::C1(Adt154::C8)
                                    } else {
                                        let a1919 = v0.clone();
                                        let a1920 = Str::lit("tasks/get");
                                        if equal(a1919, a1920) {
                                            Adt158::C1(Adt154::C9)
                                        } else {
                                            let a1921 = v0.clone();
                                            let a1922 = Str::lit("tasks/update");
                                            if equal(a1921, a1922) {
                                                Adt158::C1(Adt154::C10)
                                            } else {
                                                let a1923 = v0;
                                                let a1924 = Str::lit("tasks/cancel");
                                                if equal(a1923, a1924) {
                                                    Adt158::C1(Adt154::C11)
                                                } else {
                                                    Adt158::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f600(v0: Option<Adt128>, v1: List<Str>, v2: Adt122) -> R<Adt159> {
    f690(v0, v1, v2.clone(), f691(match v2 {
        Adt122::C0(_, _, h1925) => {
            h1925
        }
    }))
}

pub fn f601(v0: Str) -> Adt161 {
    let m1926 = f599(v0);
    match m1926 {
        Adt158::C0 => {
            Adt161::C0
        }
        Adt158::C1(v1) => {
            f692(v1)
        }
    }
}

pub fn f602(v0: List<u64>) -> Option<List<u64>> {
    let m1927 = f693(f694(), v0);
    match m1927 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1928 = f693(f695(), f410(v1, List::<u64>::nil()));
            match m1928 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f410(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f603(p1931: List<Adt131>, p1932: List<Adt131>) -> List<Adt131> {
    let m1930 = (p1931, p1932);
    {
        let mut m1930 = m1930;
        loop {
            let step = {
                let (v0, v1) = m1930;
                let m1929 = v0;
                match m1929.uncons() {
                    None => {
                        Err::<(List<Adt131>, List<Adt131>), List<Adt131>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt131>, List<Adt131>), List<Adt131>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: List<Adt98>, p1939: List<Adt12>, v2: Option<Str>, p1940: List<Adt131>) -> R<List<Adt131>> {
    let m1938 = (p1939, p1940);
    {
        let mut m1938 = m1938;
        loop {
            let step = {
                let (v1, v3) = m1938;
                let m1933 = v1;
                match m1933.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt131>), R<List<Adt131>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f696(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt131>), R<List<Adt131>>>((v5, List::cons(Adt131::C0(match v4.clone() {
                                Adt12::C0(_, h1934, _, _) => {
                                    h1934
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1935, _) => {
                                    h1935
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1936) => {
                                    h1936
                                }
                            }, f697(v0.clone(), match v4 {
                                Adt12::C0(_, h1937, _, _) => {
                                    h1937
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt131>), R<List<Adt131>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1938 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(p1956: u64, p1957: i64, p1958: i64) -> R<Option<i64>> {
    let m1955 = (p1956, (p1957, p1958));
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1955;
                let m1941 = v0;
                if m1941 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1941 - 1;
                    if f338(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1944 = {
                            let a1942 = v2;
                            let a1943 = 0i64;
                            compare(a1942, a1943)
                        };
                        if match m1944 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            if f338({
                                let a1945 = v1;
                                let a1946 = 10i64;
                                let a1947 = 0i64;
                                int_rem(a1945, a1946, a1947)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1948 = v1;
                                    let a1949 = 10i64;
                                    let a1950 = 0i64;
                                    int_quot(a1948, a1949, a1950)?
                                }, f437(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1951 = v1;
                                let a1952 = 10i64;
                                int_mul(a1951, a1952)?
                            }, {
                                let a1953 = v2;
                                let a1954 = 1i64;
                                int_sub(a1953, a1954)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(v0: i64) -> Option<i64> {
    let m1961 = {
        let a1959 = 9007199254740991i64;
        let a1960 = v0;
        compare(a1959, a1960)
    };
    if if match m1961 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        true
    } else {
        let m1964 = {
            let a1962 = v0;
            let a1963 = -9007199254740991i64;
            compare(a1962, a1963)
        };
        match m1964 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        }
    } {
        None::<i64>
    } else {
        Some(v0)
    }
}

pub fn f607(v0: Adt108, v1: Str, v2: Adt5) -> Result<Adt122, Adt112> {
    let m1965 = f229(Str::lit("params"), v2);
    match m1965 {
        Adt109::C0 => {
            Ok::<Adt122, Adt112>(Adt122::C0(v0, v1, None::<Adt5>))
        }
        Adt109::C1(v3) => {
            let m1966 = f234(v3);
            match m1966 {
                None => {
                    f396(Some(v0), Adt113::C6)
                }
                Some(v4) => {
                    Ok::<Adt122, Adt112>(Adt122::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f608(v0: Adt139) -> bool {
    let m1967 = v0;
    match m1967 {
        Adt139::C0 => {
            false
        }
        Adt139::C1 => {
            true
        }
        Adt139::C2 => {
            true
        }
        Adt139::C3 => {
            false
        }
        Adt139::C4 => {
            true
        }
        Adt139::C5 => {
            false
        }
        Adt139::C6 => {
            false
        }
        Adt139::C7 => {
            true
        }
    }
}

pub fn f609(v0: Adt135, v1: Adt140, v2: Adt136, v3: u64) -> R<Adt151> {
    let a1969 = match v0.clone() {
        Adt135::C0(_, _, h1968) => {
            h1968
        }
    };
    let a1970 = f666();
    if nat_lt(a1969, a1970) {
        Ok(Adt151::C1(Adt135::C0(v2, List::cons(v1, match v0.clone() {
            Adt135::C0(_, h1971, _) => {
                h1971
            }
        }), {
            let a1973 = match v0 {
                Adt135::C0(_, _, h1972) => {
                    h1972
                }
            };
            let a1974 = 1u64;
            nat_add(a1973, a1974)?
        })))
    } else {
        Ok(f556(Adt111::C9(v3)))
    }
}

pub fn f610(v0: bool, v1: Adt139, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt136 {
    Adt136::C8(Adt138::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f611(v0: u64) -> i64 {
    let a1975 = v0;
    let a1976 = 0u64;
    if nat_eq(a1975, a1976) {
        0i64
    } else {
        let a1977 = v0;
        let a1978 = 1u64;
        if nat_eq(a1977, a1978) {
            1i64
        } else {
            let a1979 = v0;
            let a1980 = 2u64;
            if nat_eq(a1979, a1980) {
                2i64
            } else {
                let a1981 = v0;
                let a1982 = 3u64;
                if nat_eq(a1981, a1982) {
                    3i64
                } else {
                    let a1983 = v0;
                    let a1984 = 4u64;
                    if nat_eq(a1983, a1984) {
                        4i64
                    } else {
                        let a1985 = v0;
                        let a1986 = 5u64;
                        if nat_eq(a1985, a1986) {
                            5i64
                        } else {
                            let a1987 = v0;
                            let a1988 = 6u64;
                            if nat_eq(a1987, a1988) {
                                6i64
                            } else {
                                let a1989 = v0;
                                let a1990 = 7u64;
                                if nat_eq(a1989, a1990) {
                                    7i64
                                } else {
                                    let a1991 = v0;
                                    let a1992 = 8u64;
                                    if nat_eq(a1991, a1992) {
                                        8i64
                                    } else {
                                        9i64
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f612(v0: Adt135, v1: u64) -> Adt151 {
    let m1994 = match v0.clone() {
        Adt135::C0(_, h1993, _) => {
            h1993
        }
    };
    match m1994.uncons() {
        None => {
            f556(Adt111::C2(v1))
        }
        Some((v2, _)) => {
            let m1995 = v2;
            match m1995 {
                Adt140::C0(_) => {
                    f555(v0, Adt136::C0)
                }
                Adt140::C1(_, _) => {
                    f555(v0, Adt136::C4)
                }
            }
        }
    }
}

pub fn f613(v0: Adt135, v1: Adt136) -> Adt135 {
    Adt135::C0(v1, match v0.clone() {
        Adt135::C0(_, h1996, _) => {
            h1996
        }
    }, match v0 {
        Adt135::C0(_, _, h1997) => {
            h1997
        }
    })
}

pub fn f614(v0: Adt135, v1: u64, v2: List<u64>, v3: bool) -> Adt151 {
    let m1999 = {
        let a1998 = f440(f217(), f459(v2, List::<u64>::nil()));
        utf8_decode(a1998)
    };
    match m1999 {
        None => {
            f556(Adt111::C14)
        }
        Some(v4) => {
            if v3 {
                f698(v0, v1, v4)
            } else {
                f620(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f615(v0: u64) -> u64 {
    let a2000 = v0;
    let a2001 = 34u64;
    if nat_eq(a2000, a2001) {
        34u64
    } else {
        let a2002 = v0;
        let a2003 = 92u64;
        if nat_eq(a2002, a2003) {
            92u64
        } else {
            let a2004 = v0;
            let a2005 = 47u64;
            if nat_eq(a2004, a2005) {
                47u64
            } else {
                let a2006 = v0;
                let a2007 = 98u64;
                if nat_eq(a2006, a2007) {
                    8u64
                } else {
                    let a2008 = v0;
                    let a2009 = 102u64;
                    if nat_eq(a2008, a2009) {
                        12u64
                    } else {
                        let a2010 = v0;
                        let a2011 = 110u64;
                        if nat_eq(a2010, a2011) {
                            10u64
                        } else {
                            let a2012 = v0;
                            let a2013 = 114u64;
                            if nat_eq(a2012, a2013) {
                                13u64
                            } else {
                                let a2014 = v0;
                                let a2015 = 116u64;
                                if nat_eq(a2014, a2015) {
                                    9u64
                                } else {
                                    0u64
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f616(v0: u64) -> u64 {
    let a2016 = v0;
    let a2017 = 48u64;
    if nat_lt(a2016, a2017) {
        16u64
    } else {
        let a2018 = v0;
        let a2019 = 58u64;
        if nat_lt(a2018, a2019) {
            let a2020 = v0;
            let a2021 = 48u64;
            nat_sub(a2020, a2021)
        } else {
            let a2022 = v0;
            let a2023 = 65u64;
            if nat_lt(a2022, a2023) {
                16u64
            } else {
                let a2024 = v0;
                let a2025 = 71u64;
                if nat_lt(a2024, a2025) {
                    let a2026 = v0;
                    let a2027 = 55u64;
                    nat_sub(a2026, a2027)
                } else {
                    let a2028 = v0;
                    let a2029 = 97u64;
                    if nat_lt(a2028, a2029) {
                        16u64
                    } else {
                        let a2030 = v0;
                        let a2031 = 103u64;
                        if nat_lt(a2030, a2031) {
                            let a2032 = v0;
                            let a2033 = 87u64;
                            nat_sub(a2032, a2033)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f617(v0: Adt135, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt151> {
    let a2034 = v3;
    let a2035 = 0u64;
    if nat_eq(a2034, a2035) {
        if f547(v2, 55296u64, 56319u64) {
            Ok(f555(v0, Adt136::C6(Adt137::C3(v2), v4, v5)))
        } else {
            if f547(v2, 56320u64, 57343u64) {
                Ok(f556(Adt111::C6(v1)))
            } else {
                Ok(f555(v0, Adt136::C6(Adt137::C0, f699(v2, v4)?, v5)))
            }
        }
    } else {
        if f547(v2, 56320u64, 57343u64) {
            Ok(f555(v0, Adt136::C6(Adt137::C0, f699({
                let a2044 = 65536u64;
                let a2045 = {
                    let a2042 = {
                        let a2038 = {
                            let a2036 = v3;
                            let a2037 = 55296u64;
                            nat_sub(a2036, a2037)
                        };
                        let a2039 = 1024u64;
                        nat_mul(a2038, a2039)?
                    };
                    let a2043 = {
                        let a2040 = v2;
                        let a2041 = 56320u64;
                        nat_sub(a2040, a2041)
                    };
                    nat_add(a2042, a2043)?
                };
                nat_add(a2044, a2045)?
            }, v4)?, v5)))
        } else {
            Ok(f556(Adt111::C6(v1)))
        }
    }
}

pub fn f618(v0: u64, v1: u64) -> u64 {
    let m2048 = {
        let a2046 = f619(v0);
        let a2047 = v1;
        index_list(a2046, a2047)
    };
    match m2048 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f619(v0: u64) -> List<u64> {
    let a2049 = v0;
    let a2050 = 0u64;
    if nat_eq(a2049, a2050) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a2051 = v0;
        let a2052 = 1u64;
        if nat_eq(a2051, a2052) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f620(v0: Adt135, v1: Adt6) -> Adt151 {
    Adt151::C1(f549(v1, match v0.clone() {
        Adt135::C0(_, h2053, _) => {
            h2053
        }
    }, match v0 {
        Adt135::C0(_, _, h2054) => {
            h2054
        }
    }))
}

pub fn f621(v0: u64) -> Adt6 {
    let a2055 = v0;
    let a2056 = 0u64;
    if nat_eq(a2055, a2056) {
        Adt6::C1(true)
    } else {
        let a2057 = v0;
        let a2058 = 1u64;
        if nat_eq(a2057, a2058) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f622(v0: Adt135, v1: Adt138, v2: Adt139) -> Adt151 {
    f555(v0, Adt136::C8(Adt138::C0(match v1.clone() {
        Adt138::C0(h2059, _, _, _, _, _, _) => {
            h2059
        }
    }, v2, match v1.clone() {
        Adt138::C0(_, _, h2060, _, _, _, _) => {
            h2060
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, h2061, _, _, _) => {
            h2061
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, _, h2062, _, _) => {
            h2062
        }
    }, match v1.clone() {
        Adt138::C0(_, _, _, _, _, h2063, _) => {
            h2063
        }
    }, match v1 {
        Adt138::C0(_, _, _, _, _, _, h2064) => {
            h2064
        }
    })))
}

pub fn f623(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt151> {
    let v5: u64 = {
        let a2066 = match v3.clone() {
            Adt138::C0(_, _, _, h2065, _, _, _) => {
                h2065
            }
        };
        let a2067 = 0u64;
        if if nat_eq(a2066, a2067) {
            let a2068 = v2;
            let a2069 = 48u64;
            nat_eq(a2068, a2069)
        } else {
            false
        } {
            0u64
        } else {
            let a2071 = match v3.clone() {
                Adt138::C0(_, _, _, h2070, _, _, _) => {
                    h2070
                }
            };
            let a2072 = 1u64;
            nat_add(a2071, a2072)?
        }
    };
    let v6: i64 = if v4 {
        f437(match v3.clone() {
            Adt138::C0(_, _, _, _, h2073, _, _) => {
                h2073
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt138::C0(_, _, _, _, h2074, _, _) => {
                h2074
            }
        }
    };
    let a2075 = f700();
    let a2076 = v5;
    if nat_lt(a2075, a2076) {
        Ok(f556(Adt111::C8(v1)))
    } else {
        let m2079 = {
            let a2077 = 99999i64;
            let a2078 = v6;
            compare(a2077, a2078)
        };
        if match m2079 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Ok(f556(Adt111::C8(v1)))
        } else {
            Ok(f555(v0, f610(match v3.clone() {
                Adt138::C0(h2080, _, _, _, _, _, _) => {
                    h2080
                }
            }, if v4 {
                Adt139::C4
            } else {
                Adt139::C2
            }, f437({
                let a2082 = match v3.clone() {
                    Adt138::C0(_, _, h2081, _, _, _, _) => {
                        h2081
                    }
                };
                let a2083 = 10i64;
                int_mul(a2082, a2083)?
            }, f611({
                let a2084 = v2;
                let a2085 = 48u64;
                nat_sub(a2084, a2085)
            }))?, v5, v6, match v3.clone() {
                Adt138::C0(_, _, _, _, _, h2086, _) => {
                    h2086
                }
            }, match v3 {
                Adt138::C0(_, _, _, _, _, _, h2087) => {
                    h2087
                }
            })))
        }
    }
}

pub fn f624(v0: u64) -> bool {
    let a2088 = v0;
    let a2089 = 101u64;
    if nat_eq(a2088, a2089) {
        true
    } else {
        let a2090 = v0;
        let a2091 = 69u64;
        nat_eq(a2090, a2091)
    }
}

pub fn f625(v0: u64) -> bool {
    f547(v0, 48u64, 57u64)
}

pub fn f626(v0: Adt135, v1: u64, v2: u64, v3: Adt138) -> R<Adt151> {
    let m2092 = f548(v3)?;
    match m2092 {
        Adt155::C0 => {
            Ok(f556(Adt111::C7(v1)))
        }
        Adt155::C1 => {
            Ok(f556(Adt111::C8(v1)))
        }
        Adt155::C2(v4) => {
            Ok(f553(f549(v4, match v0.clone() {
                Adt135::C0(_, h2093, _) => {
                    h2093
                }
            }, match v0 {
                Adt135::C0(_, _, h2094) => {
                    h2094
                }
            }), v1, v2))
        }
    }
}

pub fn f627(v0: Adt135, v1: u64, v2: u64, v3: Adt138, v4: bool) -> R<Adt151> {
    let v5: i64 = f437({
        let a2096 = match v3.clone() {
            Adt138::C0(_, _, _, _, _, _, h2095) => {
                h2095
            }
        };
        let a2097 = 10i64;
        int_mul(a2096, a2097)?
    }, f611({
        let a2098 = v2;
        let a2099 = 48u64;
        nat_sub(a2098, a2099)
    }))?;
    let m2102 = {
        let a2100 = 99999i64;
        let a2101 = v5;
        compare(a2100, a2101)
    };
    if match m2102 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(f556(Adt111::C8(v1)))
    } else {
        Ok(f555(v0, f610(match v3.clone() {
            Adt138::C0(h2103, _, _, _, _, _, _) => {
                h2103
            }
        }, Adt139::C7, match v3.clone() {
            Adt138::C0(_, _, h2104, _, _, _, _) => {
                h2104
            }
        }, match v3.clone() {
            Adt138::C0(_, _, _, h2105, _, _, _) => {
                h2105
            }
        }, match v3 {
            Adt138::C0(_, _, _, _, h2106, _, _) => {
                h2106
            }
        }, v4, v5)))
    }
}

pub fn f628(p2110: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2109 = p2110;
    {
        let mut m2109 = m2109;
        loop {
            let step = {
                let v0 = m2109;
                let m2107 = v0;
                match m2107.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2108 = f337(v2.clone(), v1.clone());
                        match m2108 {
                            Adt109::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt109::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f629() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f630(p2117: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m2116 = p2117;
    {
        let mut m2116 = m2116;
        loop {
            let step = {
                let v0 = m2116;
                let m2111 = v0;
                match m2111.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m2113 = f337({
                            let (h2112, _) = v2.clone();
                            h2112
                        }, v1.clone());
                        match m2113 {
                            Adt109::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt109::C1(v4) => {
                                if f701({
                                    let (_, h2114) = v2;
                                    h2114
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h2115, _) = v2;
                                        h2115
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f632(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt101> {
    let m2118 = f337(Str::lit("$ref"), v2);
    match m2118 {
        Adt109::C0 => {
            None::<Adt101>
        }
        Adt109::C1(v3) => {
            let m2119 = f131(v3);
            match m2119 {
                None => {
                    None::<Adt101>
                }
                Some(v4) => {
                    let m2120 = f702(v1, v4.clone());
                    match m2120 {
                        None => {
                            Some(Adt101::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt101>
                        }
                    }
                }
            }
        }
    }
}

pub fn f633(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt123) -> R<Adt156> {
    if v3 {
        let m2121 = f131(v2);
        match m2121 {
            None => {
                Ok(Adt156::C0(Adt101::C7(v0)))
            }
            Some(v6) => {
                f703(v0, v1, v6.clone(), f221(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt156::C0(Adt101::C10(v0)))
    }
}

pub fn f634(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt116>) -> List<Adt116> {
    let m2122 = f337(v0.clone(), v1);
    match m2122 {
        Adt109::C0 => {
            v4
        }
        Adt109::C1(v5) => {
            let m2123 = f234(v5);
            match m2123 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a2124 = v0;
                    let a2125 = Str::lit("properties");
                    if equal(a2124, a2125) {
                        f704(match v6 {
                            Adt5::C0(h2126) => {
                                h2126
                            }
                        }, v2, v3, v4)
                    } else {
                        f705(match v6 {
                            Adt5::C0(h2127) => {
                                h2127
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f635(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt116>) -> List<Adt116> {
    let m2128 = f337(v0, v1);
    match m2128 {
        Adt109::C0 => {
            v2
        }
        Adt109::C1(v3) => {
            List::cons(Adt116::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f636(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt116>) -> List<Adt116> {
    let m2129 = f337(v0, v1);
    match m2129 {
        Adt109::C0 => {
            v2
        }
        Adt109::C1(v3) => {
            let m2130 = f289(v3);
            match m2130 {
                None => {
                    v2
                }
                Some(v4) => {
                    f706(v4, v2)
                }
            }
        }
    }
}

pub fn f637(p2139: u64, p2140: List<u64>, p2141: List<u64>) -> R<Option<List<u64>>> {
    let m2138 = (p2139, (p2140, p2141));
    {
        let mut m2138 = m2138;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2138;
                let m2131 = v0;
                if m2131 == 0 {
                    let m2132 = v1;
                    match m2132.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m2131 - 1;
                    let m2133 = v1;
                    match m2133.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m2134 = v7;
                            match m2134.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m2135 = v9;
                                    match m2135.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m2136 = v11;
                                            match m2136.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m2137 = f707(v6, v8, v10, v12, f708(v13.clone()))?;
                                                    match m2137 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f459(v14, v2))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f638(p2146: List<Adt53>, p2147: Adt7) -> R<bool> {
    let m2145 = (p2146, p2147);
    {
        let mut m2145 = m2145;
        loop {
            let step = {
                let (v0, v1) = m2145;
                let m2142 = v0;
                match m2142.uncons() {
                    None => {
                        Err::<(List<Adt53>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f709(f710(match v2.clone() {
                            Adt53::C0(h2143, _, _) => {
                                h2143
                            }
                        }, v1)?) {
                            Ok::<(List<Adt53>, Adt7), R<bool>>((v3, match v2 {
                                Adt53::C0(h2144, _, _) => {
                                    h2144
                                }
                            }))
                        } else {
                            Err::<(List<Adt53>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639(p2154: List<(Str, Adt157)>) -> Adt141 {
    let m2153 = p2154;
    {
        let mut m2153 = m2153;
        loop {
            let step = {
                let v0 = m2153;
                let m2148 = v0;
                match m2148.uncons() {
                    None => {
                        Err::<List<(Str, Adt157)>, Adt141>(Adt141::C3(List::<Adt120>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m2150 = {
                            let (_, h2149) = v1;
                            h2149
                        };
                        match m2150 {
                            Adt157::C0 => {
                                Ok::<List<(Str, Adt157)>, Adt141>(v2)
                            }
                            Adt157::C1 => {
                                Err::<List<(Str, Adt157)>, Adt141>(Adt141::C0({
                                    let (h2151, _) = v1;
                                    h2151
                                }))
                            }
                            Adt157::C2 => {
                                Err::<List<(Str, Adt157)>, Adt141>(Adt141::C1({
                                    let (h2152, _) = v1;
                                    h2152
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2155 = f337(Str::lit("type"), v0);
    match m2155 {
        Adt109::C0 => {
            Ok(Adt157::C0)
        }
        Adt109::C1(v2) => {
            let m2156 = f131(v2.clone());
            match m2156 {
                None => {
                    let m2157 = f289(v2);
                    match m2157 {
                        None => {
                            Ok(Adt157::C2)
                        }
                        Some(v4) => {
                            f713(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f711(v3.clone()) {
                        if f712(v3, v1)? {
                            Ok(Adt157::C0)
                        } else {
                            Ok(Adt157::C1)
                        }
                    } else {
                        Ok(Adt157::C2)
                    }
                }
            }
        }
    }
}

pub fn f641(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2158 = f337(Str::lit("enum"), v0);
    match m2158 {
        Adt109::C0 => {
            Ok(Adt157::C0)
        }
        Adt109::C1(v2) => {
            let m2159 = f289(v2);
            match m2159 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v3) => {
                    f714(v3, v1)
                }
            }
        }
    }
}

pub fn f642(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt157> {
    let m2160 = f337(Str::lit("const"), v0);
    match m2160 {
        Adt109::C0 => {
            Ok(Adt157::C0)
        }
        Adt109::C1(v2) => {
            let m2161 = f715(v2, v1)?;
            match m2161 {
                Adt164::C0 => {
                    Ok(Adt157::C0)
                }
                Adt164::C1 => {
                    Ok(Adt157::C1)
                }
                Adt164::C2 => {
                    Ok(Adt157::C2)
                }
            }
        }
    }
}

pub fn f643(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt157> {
    let m2162 = f337(v0, v2);
    match m2162 {
        Adt109::C0 => {
            Ok(Adt157::C0)
        }
        Adt109::C1(v4) => {
            let m2163 = f716(v4);
            match m2163 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v5) => {
                    let m2164 = f716(v3);
                    match m2164 {
                        None => {
                            Ok(Adt157::C0)
                        }
                        Some(v6) => {
                            if f717(v1, f710(v6, v5)?) {
                                Ok(Adt157::C0)
                            } else {
                                Ok(Adt157::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f644(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt157)>> {
    let m2165 = v1;
    match m2165 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt157)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f718(Str::lit("minLength"), true, v0.clone(), f719(v4.clone())?)?), List::cons((Str::lit("maxLength"), f718(Str::lit("maxLength"), false, v0, f719(v4)?)?), List::<(Str, Adt157)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f718(Str::lit("minItems"), true, v0.clone(), f720(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f718(Str::lit("maxItems"), false, v0, f720(v5, 0u64)?)?), List::<(Str, Adt157)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f718(Str::lit("minProperties"), true, v0.clone(), f721(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f718(Str::lit("maxProperties"), false, v0, f721(v6, 0u64)?)?), List::<(Str, Adt157)>::nil())))
        }
    }
}

pub fn f645(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt157 {
    let m2166 = f337(Str::lit("required"), v0);
    match m2166 {
        Adt109::C0 => {
            Adt157::C0
        }
        Adt109::C1(v2) => {
            let m2167 = f289(v2);
            match m2167 {
                None => {
                    Adt157::C2
                }
                Some(v3) => {
                    let m2168 = f234(v1);
                    match m2168 {
                        None => {
                            Adt157::C0
                        }
                        Some(v4) => {
                            f722(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f646(v0: Adt141, v1: Adt141) -> Adt141 {
    let m2169 = v0.clone();
    match m2169 {
        Adt141::C0(_) => {
            v0
        }
        Adt141::C1(_) => {
            v0
        }
        Adt141::C2(_) => {
            v0
        }
        Adt141::C3(v5) => {
            let m2170 = v1.clone();
            match m2170 {
                Adt141::C0(_) => {
                    v1
                }
                Adt141::C1(_) => {
                    v1
                }
                Adt141::C2(_) => {
                    v1
                }
                Adt141::C3(v9) => {
                    Adt141::C3({
                        let a2171 = v5;
                        let a2172 = v9;
                        append_list(a2171, a2172)
                    })
                }
            }
        }
    }
}

pub fn f647(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt118>) -> Adt141 {
    let m2173 = f337(Str::lit("$ref"), v1);
    match m2173 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v4) => {
            let m2174 = f131(v4);
            match m2174 {
                None => {
                    Adt141::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m2175 = f702(v0, v5.clone());
                    match m2175 {
                        None => {
                            Adt141::C2(v5)
                        }
                        Some(v6) => {
                            Adt141::C3(List::cons(Adt120::C0(v6, v2, v3), List::<Adt120>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f648(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m2176 = f337(Str::lit("properties"), v0);
    match m2176 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            let m2177 = f234(v3);
            match m2177 {
                None => {
                    Adt141::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m2178 = f234(v1);
                    match m2178 {
                        None => {
                            Adt141::C3(List::<Adt120>::nil())
                        }
                        Some(v5) => {
                            Adt141::C3(f723(match v4 {
                                Adt5::C0(h2179) => {
                                    h2179
                                }
                            }, match v5 {
                                Adt5::C0(h2180) => {
                                    h2180
                                }
                            }, v2, List::<Adt120>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f649(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m2181 = f337(Str::lit("additionalProperties"), v0.clone());
    match m2181 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            let m2182 = f234(v1);
            match m2182 {
                None => {
                    Adt141::C3(List::<Adt120>::nil())
                }
                Some(v4) => {
                    let m2183 = f438(v3.clone());
                    match m2183 {
                        None => {
                            Adt141::C3(f726(match v4 {
                                Adt5::C0(h2185) => {
                                    h2185
                                }
                            }, f725(v0), v3, v2, List::<Adt120>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt141::C3(List::<Adt120>::nil())
                            } else {
                                if f724(match v4 {
                                    Adt5::C0(h2184) => {
                                        h2184
                                    }
                                }, f725(v0)) {
                                    Adt141::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt141::C3(List::<Adt120>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f650(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> R<Adt141> {
    let m2186 = f337(Str::lit("items"), v0);
    match m2186 {
        Adt109::C0 => {
            Ok(Adt141::C3(List::<Adt120>::nil()))
        }
        Adt109::C1(v3) => {
            let m2187 = f289(v1);
            match m2187 {
                None => {
                    Ok(Adt141::C3(List::<Adt120>::nil()))
                }
                Some(v4) => {
                    Ok(Adt141::C3(f727(v4, 0u64, v3, v2, List::<Adt120>::nil())?))
                }
            }
        }
    }
}

pub fn f651(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt118>) -> Adt141 {
    let m2188 = f337(Str::lit("allOf"), v0);
    match m2188 {
        Adt109::C0 => {
            Adt141::C3(List::<Adt120>::nil())
        }
        Adt109::C1(v3) => {
            let m2189 = f289(v3);
            match m2189 {
                None => {
                    Adt141::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m2190 = v4.clone();
                    match m2190.uncons() {
                        None => {
                            Adt141::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt141::C3(f728(v4, v1, v2, List::<Adt120>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f652(v0: Adt121, v1: Adt141, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt118>) -> Adt141 {
    let m2191 = v1.clone();
    match m2191 {
        Adt141::C0(_) => {
            v1
        }
        Adt141::C1(_) => {
            v1
        }
        Adt141::C2(_) => {
            v1
        }
        Adt141::C3(v8) => {
            let m2192 = v8;
            match m2192.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt141::C3(List::cons(Adt120::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt120>::nil()))
                }
            }
        }
    }
}

pub fn f653(p2199: List<u64>, p2200: List<u64>) -> List<u64> {
    let m2198 = (p2199, p2200);
    {
        let mut m2198 = m2198;
        loop {
            let step = {
                let (v0, v1) = m2198;
                let m2193 = v0;
                match m2193.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a2194 = v2;
                        let a2195 = 126u64;
                        if nat_eq(a2194, a2195) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a2196 = v2;
                            let a2197 = 47u64;
                            if nat_eq(a2196, a2197) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2198 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654(p2205: List<u64>, p2206: u64, p2207: List<u64>) -> List<u64> {
    let m2204 = (p2205, (p2206, p2207));
    {
        let mut m2204 = m2204;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2204;
                let m2201 = v0;
                match m2201.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f729(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt162 = f730(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt162::C0(h2202, _) => {
                                h2202
                            }
                        }, match v5 {
                            Adt162::C0(_, h2203) => {
                                h2203
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2204 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: Option<Adt43>) -> List<(Str, Adt6)> {
    let m2208 = v0;
    match m2208 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2214 = f731(match v1.clone() {
                    Adt43::C0(h2209, _, _) => {
                        h2209
                    }
                });
                let a2215 = {
                    let a2212 = f732(match v1.clone() {
                        Adt43::C0(_, h2210, _) => {
                            h2210
                        }
                    });
                    let a2213 = f657(Str::lit("lastModified"), match v1 {
                        Adt43::C0(_, _, h2211) => {
                            h2211
                        }
                    });
                    append_list(a2212, a2213)
                };
                append_list(a2214, a2215)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f656(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f733(Str::lit("_meta"), v0)
}

pub fn f657(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m2216 = v1;
    match m2216 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f658(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2217 = v0;
    match m2217 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f734(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f659(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2218 = v0;
    match m2218 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f124(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f660(v0: Adt49) -> Adt6 {
    let m2219 = v0;
    match m2219 {
        Adt49::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt50::C0(h2220, _, _, _) => {
                    h2220
                }
            })), {
                let a2224 = f657(Str::lit("mimeType"), match v1.clone() {
                    Adt50::C0(_, h2221, _, _) => {
                        h2221
                    }
                });
                let a2225 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt50::C0(_, _, h2222, _) => {
                        h2222
                    }
                })), f656(match v1 {
                    Adt50::C0(_, _, _, h2223) => {
                        h2223
                    }
                }));
                append_list(a2224, a2225)
            }))
        }
        Adt49::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt51::C0(h2226, _, _, _) => {
                    h2226
                }
            })), {
                let a2230 = f657(Str::lit("mimeType"), match v2.clone() {
                    Adt51::C0(_, h2227, _, _) => {
                        h2227
                    }
                });
                let a2231 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt51::C0(_, _, h2228, _) => {
                        h2228
                    }
                })), f656(match v2 {
                    Adt51::C0(_, _, _, h2229) => {
                        h2229
                    }
                }));
                append_list(a2230, a2231)
            }))
        }
    }
}

pub fn f661() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f662(v0: Adt6) -> Option<Adt67> {
    let m2232 = f234(v0);
    match m2232 {
        None => {
            None::<Adt67>
        }
        Some(v1) => {
            let m2233 = f520(Str::lit("method"), v1.clone());
            match m2233 {
                None => {
                    None::<Adt67>
                }
                Some(v2) => {
                    let a2234 = v2.clone();
                    let a2235 = Str::lit("elicitation/create");
                    if equal(a2234, a2235) {
                        let m2236 = f452(Str::lit("params"), v1);
                        match m2236 {
                            None => {
                                None::<Adt67>
                            }
                            Some(v3) => {
                                f735(v3)
                            }
                        }
                    } else {
                        let a2237 = v2;
                        let a2238 = Str::lit("roots/list");
                        if equal(a2237, a2238) {
                            f736(v1)
                        } else {
                            None::<Adt67>
                        }
                    }
                }
            }
        }
    }
}

pub fn f663(v0: u64) -> List<u64> {
    f619(v0)
}

pub fn f664(v0: Adt7) -> R<bool> {
    let a2256 = {
        let m2242 = {
            let a2240 = f737();
            let a2241 = match v0.clone() {
                Adt7::C0(h2239, _) => {
                    h2239
                }
            };
            compare(a2240, a2241)
        };
        if match m2242 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            true
        } else {
            let m2247 = {
                let a2245 = match v0.clone() {
                    Adt7::C0(h2243, _) => {
                        h2243
                    }
                };
                let a2246 = {
                    let a2244 = f737();
                    int_neg(a2244)?
                };
                compare(a2245, a2246)
            };
            if match m2247 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                true
            } else {
                let m2251 = {
                    let a2249 = 9999i64;
                    let a2250 = match v0.clone() {
                        Adt7::C0(_, h2248) => {
                            h2248
                        }
                    };
                    compare(a2249, a2250)
                };
                if match m2251 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                } {
                    true
                } else {
                    let m2255 = {
                        let a2253 = match v0 {
                            Adt7::C0(_, h2252) => {
                                h2252
                            }
                        };
                        let a2254 = -9999i64;
                        compare(a2253, a2254)
                    };
                    match m2255 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    }
                }
            }
        }
    };
    Ok(bool_not(a2256))
}

pub fn f665(v0: Adt7) -> R<List<u64>> {
    let m2260 = {
        let a2258 = match v0.clone() {
            Adt7::C0(_, h2257) => {
                h2257
            }
        };
        let a2259 = 0i64;
        compare(a2258, a2259)
    };
    if if match m2260 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        true
    } else {
        let m2264 = {
            let a2262 = 0i64;
            let a2263 = match v0.clone() {
                Adt7::C0(_, h2261) => {
                    h2261
                }
            };
            compare(a2262, a2263)
        };
        match m2264 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        }
    } {
        let a2267 = f738(match v0.clone() {
            Adt7::C0(h2265, _) => {
                h2265
            }
        })?;
        let a2268 = List::cons(101u64, f738(match v0 {
            Adt7::C0(_, h2266) => {
                h2266
            }
        })?);
        Ok(append_list(a2267, a2268))
    } else {
        f738(match v0 {
            Adt7::C0(h2269, _) => {
                h2269
            }
        })
    }
}

pub fn f666() -> u64 {
    512u64
}

pub fn f667(v0: u64, v1: Adt127, v2: List<Adt127>) -> List<Adt127> {
    List::cons(Adt127::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f668(p2272: List<u64>, p2273: List<u64>) -> R<List<u64>> {
    let m2271 = (p2272, p2273);
    {
        let mut m2271 = m2271;
        loop {
            let step = {
                let (v0, v1) = m2271;
                let m2270 = v0;
                match m2270.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f459(f739(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2271 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f669() -> u64 {
    4294967296u64
}

pub fn f670(v0: Adt100) -> Adt100 {
    Adt100::C0(match v0.clone() {
        Adt100::C0(h2274, _, _, _, _) => {
            h2274
        }
    }, match v0.clone() {
        Adt100::C0(_, h2275, _, _, _) => {
            h2275
        }
    }, match v0.clone() {
        Adt100::C0(_, _, h2276, _, _) => {
            h2276
        }
    }, match v0 {
        Adt100::C0(_, _, _, h2277, _) => {
            h2277
        }
    }, None::<List<u64>>)
}

pub fn f671(v0: Adt0, v1: List<u64>) -> Adt93 {
    Adt93::C0(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, h2278) => {
            h2278
        }
    }, f440(f217(), {
        let a2280 = f9({
            let a2279 = v1.clone();
            length_list(a2279)
        });
        let a2281 = v1;
        append_list(a2280, a2281)
    }))
}

pub fn f672() -> Adt94 {
    Adt94::C0(f26(f27()))
}

pub fn f673(v0: Adt35) -> R<Option<List<(Str, Adt6)>>> {
    let m2282 = v0;
    match m2282 {
        Adt35::C0(v1) => {
            Ok(Some(f740(v1)?))
        }
        Adt35::C1(v2) => {
            Ok(Some(f741(v2)?))
        }
        Adt35::C2(v3) => {
            f231(f232(v3)?)
        }
        Adt35::C3(v4) => {
            f231(f232(match v4 {
                Adt52::C0(h2283, _) => {
                    h2283
                }
            })?)
        }
        Adt35::C4(v5) => {
            Ok(Some(f742(v5)?))
        }
        Adt35::C5(v6) => {
            Ok(Some(f743(v6)?))
        }
        Adt35::C6(v7) => {
            Ok(Some(f744(v7)?))
        }
        Adt35::C7(v8) => {
            Ok(Some(f745(v8)?))
        }
        Adt35::C8(v9) => {
            f231(f746(v9)?)
        }
        Adt35::C9(v10) => {
            Ok(Some(f747(v10)?))
        }
        Adt35::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt35::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt35::C12(v12) => {
            f231(f748(v12)?)
        }
        Adt35::C13(v13) => {
            f231(f749({
                let m2286 = match v13 {
                    Adt85::C0(_, _, h2285) => {
                        h2285
                    }
                };
                match m2286 {
                    Adt87::C0(h2284, _, _) => {
                        h2284
                    }
                }
            }))
        }
        Adt35::C14(v14) => {
            Ok(f750(v14))
        }
    }
}

pub fn f674(v0: Adt144) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f751(f752(match v0.clone() {
        Adt144::C0(h2287, _, _) => {
            h2287
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f753({
        let m2290 = match v0 {
            Adt144::C0(_, h2289, _) => {
                h2289
            }
        };
        match m2290 {
            Adt145::C0(h2288, _) => {
                h2288
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f675(v0: Option<Option<List<u64>>>, v1: Adt100) -> Adt100 {
    let m2291 = v0;
    match m2291 {
        None => {
            f27()
        }
        Some(v2) => {
            Adt100::C0(match v1.clone() {
                Adt100::C0(h2292, _, _, _, _) => {
                    h2292
                }
            }, match v1.clone() {
                Adt100::C0(_, h2293, _, _, _) => {
                    h2293
                }
            }, match v1.clone() {
                Adt100::C0(_, _, h2294, _, _) => {
                    h2294
                }
            }, match v1 {
                Adt100::C0(_, _, _, h2295, _) => {
                    h2295
                }
            }, v2)
        }
    }
}

pub fn f676(v0: Adt16, v1: Adt90) -> R<Option<Option<List<u64>>>> {
    let m2297 = f187(match v0.clone() {
        Adt16::C0(_, _, _, h2296) => {
            h2296
        }
    })?;
    match m2297 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m2300 = f754(f755(match v1 {
                Adt90::C0(h2298, _, _) => {
                    h2298
                }
            }, v2, match v0 {
                Adt16::C0(h2299, _, _, _) => {
                    h2299
                }
            })?)?;
            match m2300 {
                None => {
                    Ok(None::<Option<List<u64>>>)
                }
                Some(v3) => {
                    Ok(Some(Some(v3)))
                }
            }
        }
    }
}

pub fn f677(v0: bool, v1: Adt100, v2: List<List<u64>>) -> Adt100 {
    if f756(v0, match v1.clone() {
        Adt100::C0(h2301, _, _, _, _) => {
            h2301
        }
    }, v2.clone()) {
        f757(match v1.clone() {
            Adt100::C0(h2302, _, _, _, _) => {
                h2302
            }
        }, {
            let a2304 = v2;
            let a2305 = List::cons(match v1 {
                Adt100::C0(_, _, _, h2303, _) => {
                    h2303
                }
            }, List::<List<u64>>::nil());
            append_list(a2304, a2305)
        })
    } else {
        v1
    }
}

pub fn f678(v0: Adt146) -> R<Option<List<u64>>> {
    let m2306 = f339(f758(v0))?;
    match m2306 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2307 = f298(v1)?;
            match m2307 {
                Ok(v3) => {
                    Ok(Some(v3))
                }
                Err(_) => {
                    Ok(None::<List<u64>>)
                }
            }
        }
    }
}

pub fn f679(v0: Adt90) -> R<Option<List<List<u64>>>> {
    if f759(v0.clone()) {
        let m2311 = f760({
            let m2310 = match v0 {
                Adt90::C0(h2309, _, _) => {
                    h2309
                }
            };
            match m2310 {
                Adt87::C0(h2308, _, _) => {
                    h2308
                }
            }
        });
        match m2311 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m2312 = f339(v1)?;
                match m2312 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m2313 = f298(v2)?;
                        match m2313 {
                            Ok(v4) => {
                                Ok(Some(List::cons(v4, List::<List<u64>>::nil())))
                            }
                            Err(_) => {
                                Ok(None::<List<List<u64>>>)
                            }
                        }
                    }
                }
            }
        }
    } else {
        Ok(Some(List::<List<u64>>::nil()))
    }
}

pub fn f680() -> u64 {
    404u64
}

pub fn f681(v0: Adt92) -> u64 {
    let m2314 = v0;
    match m2314 {
        Adt92::C0(_) => {
            f305()
        }
        Adt92::C1(_) => {
            f305()
        }
        Adt92::C2(_) => {
            f305()
        }
        Adt92::C3(_) => {
            f305()
        }
        Adt92::C4(_) => {
            f305()
        }
        Adt92::C5(_) => {
            f305()
        }
        Adt92::C6(_) => {
            f51()
        }
        Adt92::C7(_) => {
            f761()
        }
        Adt92::C8(_) => {
            f305()
        }
        Adt92::C9(_) => {
            f51()
        }
        Adt92::C10 => {
            f305()
        }
        Adt92::C11 => {
            f305()
        }
        Adt92::C12(_) => {
            f51()
        }
        Adt92::C13 => {
            f51()
        }
        Adt92::C14 => {
            f51()
        }
        Adt92::C15(_) => {
            f51()
        }
    }
}

pub fn f682(v0: Adt147) -> i64 {
    let m2315 = v0;
    match m2315 {
        Adt147::C0(_) => {
            f762()
        }
        Adt147::C1(_, _) => {
            f763()
        }
        Adt147::C2(_) => {
            f764()
        }
        Adt147::C3(_) => {
            f765()
        }
        Adt147::C4(v6) => {
            f766(v6)
        }
    }
}

pub fn f683(v0: Adt147) -> Str {
    let m2316 = v0;
    match m2316 {
        Adt147::C0(v1) => {
            f767(v1)
        }
        Adt147::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt147::C2(v4) => {
            f768(v4)
        }
        Adt147::C3(_) => {
            Str::lit("Method not found")
        }
        Adt147::C4(v6) => {
            let m2317 = v6;
            match m2317 {
                Adt92::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt92::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt92::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt92::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt92::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt92::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt92::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt92::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt92::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt92::C9(_) => {
                    Str::lit("Internal error")
                }
                Adt92::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt92::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt92::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt92::C13 => {
                    Str::lit("Internal error")
                }
                Adt92::C14 => {
                    Str::lit("Internal error")
                }
                Adt92::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f684(v0: Adt147) -> R<Option<Adt6>> {
    let m2318 = v0;
    match m2318 {
        Adt147::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt147::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f769(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt147::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt147::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt147::C4(v6) => {
            let m2319 = v6;
            match m2319 {
                Adt92::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C2(v9) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(v9)), List::<(Str, Adt6)>::nil()))))
                }
                Adt92::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C4(v11) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("reason"), Adt6::C3(v11)), List::<(Str, Adt6)>::nil()))))
                }
                Adt92::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f344(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt92::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f770(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt92::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt92::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt92::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt92::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt92::C14 => {
                    Ok(None::<Adt6>)
                }
                Adt92::C15(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f685(p2322: List<List<u64>>) -> R<bool> {
    let m2321 = p2322;
    {
        let mut m2321 = m2321;
        loop {
            let step = {
                let v0 = m2321;
                let m2320 = v0;
                match m2320.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f771(f402(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f687(v0: Adt0, v1: Fn9, v2: u64, v3: Adt153, v4: bool) -> R<Result<Adt144, Adt147>> {
    if v4 {
        f772(v0, v1, v2, f773(match v3.clone() {
            Adt153::C0(_, h2323, _, _) => {
                h2323
            }
        }, match v3.clone() {
            Adt153::C0(_, _, h2324, _) => {
                h2324
            }
        }, match v3 {
            Adt153::C0(_, _, _, h2325) => {
                h2325
            }
        }))
    } else {
        Ok(Err::<Adt144, Adt147>(Adt147::C3(f774(match v3 {
            Adt153::C0(_, h2326, _, _) => {
                h2326
            }
        }))))
    }
}

pub fn f688(v0: Adt4, v1: Adt154) -> bool {
    let m2327 = v1;
    match m2327 {
        Adt154::C0 => {
            true
        }
        Adt154::C1 => {
            f775(match v0 {
                Adt4::C0(_, _, _, _, _, h2328, _) => {
                    h2328
                }
            })
        }
        Adt154::C2 => {
            f775(match v0 {
                Adt4::C0(_, _, _, _, _, h2329, _) => {
                    h2329
                }
            })
        }
        Adt154::C3 => {
            f776(match v0 {
                Adt4::C0(_, _, _, _, h2330, _, _) => {
                    h2330
                }
            })
        }
        Adt154::C4 => {
            f776(match v0 {
                Adt4::C0(_, _, _, _, h2331, _, _) => {
                    h2331
                }
            })
        }
        Adt154::C5 => {
            f776(match v0 {
                Adt4::C0(_, _, _, _, h2332, _, _) => {
                    h2332
                }
            })
        }
        Adt154::C6 => {
            f775(match v0 {
                Adt4::C0(_, _, _, h2333, _, _, _) => {
                    h2333
                }
            })
        }
        Adt154::C7 => {
            f775(match v0 {
                Adt4::C0(_, _, _, h2334, _, _, _) => {
                    h2334
                }
            })
        }
        Adt154::C8 => {
            f352(match v0 {
                Adt4::C0(_, _, h2335, _, _, _, _) => {
                    h2335
                }
            })
        }
        Adt154::C9 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2336) => {
                    h2336
                }
            }, f33())
        }
        Adt154::C10 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2337) => {
                    h2337
                }
            }, f33())
        }
        Adt154::C11 => {
            f288(match v0 {
                Adt4::C0(_, _, _, _, _, _, h2338) => {
                    h2338
                }
            }, f33())
        }
    }
}

pub fn f689(v0: Adt160, v1: Adt158) -> Result<Adt153, Adt147> {
    let m2339 = v1;
    match m2339 {
        Adt158::C0 => {
            Err::<Adt153, Adt147>(Adt147::C3(match v0 {
                Adt160::C0(_, h2340, _, _) => {
                    h2340
                }
            }))
        }
        Adt158::C1(v2) => {
            Ok::<Adt153, Adt147>(Adt153::C0(match v0.clone() {
                Adt160::C0(h2341, _, _, _) => {
                    h2341
                }
            }, v2, match v0.clone() {
                Adt160::C0(_, _, h2342, _) => {
                    h2342
                }
            }, match v0 {
                Adt160::C0(_, _, _, h2343) => {
                    h2343
                }
            }))
        }
    }
}

pub fn f690(v0: Option<Adt128>, v1: List<Str>, v2: Adt122, v3: Result<Adt19, Adt148>) -> R<Adt159> {
    let m2344 = v3;
    match m2344 {
        Ok(v5) => {
            Ok(f777(v1, v2.clone(), v5.clone(), f778(v0, f601(match v2.clone() {
                Adt122::C0(_, h2345, _) => {
                    h2345
                }
            }), match v2.clone() {
                Adt122::C0(_, h2346, _) => {
                    h2346
                }
            }, match v5 {
                Adt19::C0(h2347, _, _, _, _, _) => {
                    h2347
                }
            }, f779(match v2 {
                Adt122::C0(_, _, h2348) => {
                    h2348
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt159::C1(Adt147::C0(v4)))
        }
    }
}

pub fn f691(v0: Option<Adt5>) -> Result<Adt19, Adt148> {
    let m2349 = v0;
    match m2349 {
        None => {
            Err::<Adt19, Adt148>(Adt148::C0)
        }
        Some(v1) => {
            let m2350 = f229(Str::lit("_meta"), v1);
            match m2350 {
                Adt109::C0 => {
                    Err::<Adt19, Adt148>(Adt148::C1)
                }
                Adt109::C1(v2) => {
                    let m2351 = f234(v2);
                    match m2351 {
                        None => {
                            Err::<Adt19, Adt148>(Adt148::C4)
                        }
                        Some(v3) => {
                            f780(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f692(v0: Adt154) -> Adt161 {
    let m2352 = v0;
    match m2352 {
        Adt154::C0 => {
            Adt161::C0
        }
        Adt154::C1 => {
            Adt161::C0
        }
        Adt154::C2 => {
            Adt161::C1
        }
        Adt154::C3 => {
            Adt161::C0
        }
        Adt154::C4 => {
            Adt161::C2
        }
        Adt154::C5 => {
            Adt161::C0
        }
        Adt154::C6 => {
            Adt161::C0
        }
        Adt154::C7 => {
            Adt161::C1
        }
        Adt154::C8 => {
            Adt161::C0
        }
        Adt154::C9 => {
            Adt161::C3
        }
        Adt154::C10 => {
            Adt161::C3
        }
        Adt154::C11 => {
            Adt161::C3
        }
    }
}

pub fn f693(p2358: List<u64>, p2359: List<u64>) -> Option<List<u64>> {
    let m2357 = (p2358, p2359);
    {
        let mut m2357 = m2357;
        loop {
            let step = {
                let (v0, v1) = m2357;
                let m2353 = v0;
                match m2353.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2354 = v1;
                        match m2354.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a2355 = v4;
                                let a2356 = v2;
                                if nat_eq(a2355, a2356) {
                                    Ok::<(List<u64>, List<u64>), Option<List<u64>>>((v3, v5))
                                } else {
                                    Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f695() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f696(v0: Adt12, v1: Option<Str>) -> bool {
    let m2360 = v1;
    match m2360 {
        None => {
            false
        }
        Some(v2) => {
            let a2362 = match v0 {
                Adt12::C0(h2361, _, _, _) => {
                    h2361
                }
            };
            let a2363 = v2;
            equal(a2362, a2363)
        }
    }
}

pub fn f697(v0: List<Adt98>, v1: Str) -> R<Adt132> {
    let m2366 = f538(f209(v0, {
        let a2364 = f781();
        let a2365 = f782(v1)?;
        append_list(a2364, a2365)
    })?)?;
    match m2366 {
        Adt114::C0 => {
            Ok(Adt132::C0)
        }
        Adt114::C1 => {
            Ok(Adt132::C1)
        }
        Adt114::C2 => {
            Ok(Adt132::C1)
        }
        Adt114::C3(v2) => {
            let m2367 = f414(v2);
            match m2367 {
                None => {
                    Ok(Adt132::C1)
                }
                Some(v3) => {
                    Ok(Adt132::C2(v3))
                }
            }
        }
    }
}

pub fn f698(v0: Adt135, v1: u64, v2: Str) -> Adt151 {
    let m2369 = match v0.clone() {
        Adt135::C0(_, h2368, _) => {
            h2368
        }
    };
    match m2369.uncons() {
        None => {
            f556(Adt111::C2(v1))
        }
        Some((v3, v4)) => {
            let m2370 = v3;
            match m2370 {
                Adt140::C0(_) => {
                    f556(Adt111::C2(v1))
                }
                Adt140::C1(v6, _) => {
                    Adt151::C1(Adt135::C0(Adt136::C5, List::cons(Adt140::C1(v6, Some(v2)), v4), match v0 {
                        Adt135::C0(_, _, h2371) => {
                            h2371
                        }
                    }))
                }
            }
        }
    }
}

pub fn f699(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a2372 = v0;
    let a2373 = 128u64;
    if nat_lt(a2372, a2373) {
        Ok(List::cons(v0, v1))
    } else {
        let a2374 = v0;
        let a2375 = 2048u64;
        if nat_lt(a2374, a2375) {
            Ok(List::cons({
                let a2376 = 128u64;
                let a2377 = f783(v0, 64u64);
                nat_add(a2376, a2377)?
            }, List::cons({
                let a2378 = 192u64;
                let a2379 = f784(v0, 64u64);
                nat_add(a2378, a2379)?
            }, v1)))
        } else {
            let a2380 = v0;
            let a2381 = 65536u64;
            if nat_lt(a2380, a2381) {
                Ok(List::cons({
                    let a2382 = 128u64;
                    let a2383 = f783(v0, 64u64);
                    nat_add(a2382, a2383)?
                }, List::cons({
                    let a2384 = 128u64;
                    let a2385 = f783(f784(v0, 64u64), 64u64);
                    nat_add(a2384, a2385)?
                }, List::cons({
                    let a2386 = 224u64;
                    let a2387 = f784(v0, 4096u64);
                    nat_add(a2386, a2387)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a2388 = 128u64;
                    let a2389 = f783(v0, 64u64);
                    nat_add(a2388, a2389)?
                }, List::cons({
                    let a2390 = 128u64;
                    let a2391 = f783(f784(v0, 64u64), 64u64);
                    nat_add(a2390, a2391)?
                }, List::cons({
                    let a2392 = 128u64;
                    let a2393 = f783(f784(v0, 4096u64), 64u64);
                    nat_add(a2392, a2393)?
                }, List::cons({
                    let a2394 = 240u64;
                    let a2395 = f784(v0, 262144u64);
                    nat_add(a2394, a2395)?
                }, v1)))))
            }
        }
    }
}

pub fn f700() -> u64 {
    18u64
}

pub fn f701(v0: u64, v1: Adt6) -> bool {
    let a2396 = v0;
    let a2397 = 0u64;
    if nat_eq(a2396, a2397) {
        f785(v1)
    } else {
        let a2398 = v0;
        let a2399 = 1u64;
        if nat_eq(a2398, a2399) {
            f786(v1)
        } else {
            let a2400 = v0;
            let a2401 = 2u64;
            if nat_eq(a2400, a2401) {
                f787(v1)
            } else {
                let a2402 = v0;
                let a2403 = 3u64;
                if nat_eq(a2402, a2403) {
                    f788(v1)
                } else {
                    let a2404 = v0;
                    let a2405 = 4u64;
                    if nat_eq(a2404, a2405) {
                        f789(v1)
                    } else {
                        let a2406 = v0;
                        let a2407 = 5u64;
                        if nat_eq(a2406, a2407) {
                            f790(v1)
                        } else {
                            let a2408 = v0;
                            let a2409 = 6u64;
                            if nat_eq(a2408, a2409) {
                                f791(v1)
                            } else {
                                let a2410 = v0;
                                let a2411 = 7u64;
                                if nat_eq(a2410, a2411) {
                                    f792(v1)
                                } else {
                                    f793(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f702(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a2412 = v1.clone();
    let a2413 = Str::lit("#");
    if equal(a2412, a2413) {
        Some(v0)
    } else {
        let m2414 = f234(v0);
        match m2414 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m2415 = f229(Str::lit("$defs"), v2);
                match m2415 {
                    Adt109::C0 => {
                        None::<Adt6>
                    }
                    Adt109::C1(v3) => {
                        let m2416 = f234(v3);
                        match m2416 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f794(v1, match v4 {
                                    Adt5::C0(h2417) => {
                                        h2417
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f703(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt123) -> R<Adt156> {
    let m2418 = v3.clone();
    match m2418.uncons() {
        None => {
            Ok(Adt156::C0(Adt101::C8(v0)))
        }
        Some((_, _)) => {
            if f416(v3.clone(), true) {
                let m2419 = f795(v1);
                match m2419 {
                    None => {
                        Ok(Adt156::C0(Adt101::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f796(v3)?;
                        if f797(v9.clone(), match v5.clone() {
                            Adt123::C0(_, h2420) => {
                                h2420
                            }
                        }) {
                            Ok(Adt156::C0(Adt101::C12(v0, v2)))
                        } else {
                            Ok(Adt156::C1(Adt123::C0(List::cons(Adt12::C0(v0, v2, f353(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt123::C0(h2421, _) => {
                                    h2421
                                }
                            }), List::cons(v9, match v5 {
                                Adt123::C0(_, h2422) => {
                                    h2422
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt156::C0(Adt101::C9(v0)))
            }
        }
    }
}

pub fn f704(p2427: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p2428: List<Adt116>) -> List<Adt116> {
    let m2426 = (p2427, p2428);
    {
        let mut m2426 = m2426;
        loop {
            let step = {
                let (v0, v3) = m2426;
                let m2423 = v0;
                match m2423.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt116>), List<Adt116>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt116>), List<Adt116>>((v5, List::cons(Adt116::C0({
                            let (_, h2424) = v4;
                            h2424
                        }, v1, List::cons({
                            let (h2425, _) = v4;
                            h2425
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(p2432: List<(Str, Adt6)>, p2433: List<Adt116>) -> List<Adt116> {
    let m2431 = (p2432, p2433);
    {
        let mut m2431 = m2431;
        loop {
            let step = {
                let (v0, v1) = m2431;
                let m2429 = v0;
                match m2429.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt116>), List<Adt116>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt116>), List<Adt116>>((v3, List::cons(Adt116::C0({
                            let (_, h2430) = v2;
                            h2430
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f706(p2436: List<Adt6>, p2437: List<Adt116>) -> List<Adt116> {
    let m2435 = (p2436, p2437);
    {
        let mut m2435 = m2435;
        loop {
            let step = {
                let (v0, v1) = m2435;
                let m2434 = v0;
                match m2434.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt116>), List<Adt116>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt116>), List<Adt116>>((v3, List::cons(Adt116::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f707(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m2438 = f798(v0)?;
    match m2438 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m2439 = f798(v1)?;
            match m2439 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a2440 = v2;
                    let a2441 = 61u64;
                    if nat_eq(a2440, a2441) {
                        f799(v5, v6, v4, v3)
                    } else {
                        let m2442 = f798(v2)?;
                        match m2442 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a2443 = v3;
                                let a2444 = 61u64;
                                if nat_eq(a2443, a2444) {
                                    f800(v5, v6, v7, v4)
                                } else {
                                    let m2445 = f798(v3)?;
                                    match m2445 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f801(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f708(v0: List<u64>) -> bool {
    let m2446 = v0;
    match m2446.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f709(v0: Adt163) -> bool {
    let m2447 = v0;
    match m2447 {
        Adt163::C0 => {
            false
        }
        Adt163::C1 => {
            false
        }
        Adt163::C2 => {
            true
        }
    }
}

pub fn f710(v0: Adt7, v1: Adt7) -> R<Adt163> {
    f802(v0.clone(), v1.clone(), f803(match v0 {
        Adt7::C0(h2448, _) => {
            h2448
        }
    }), f803(match v1 {
        Adt7::C0(h2449, _) => {
            h2449
        }
    }))
}

pub fn f711(v0: Str) -> bool {
    let a2450 = v0.clone();
    let a2451 = Str::lit("null");
    if equal(a2450, a2451) {
        true
    } else {
        let a2452 = v0.clone();
        let a2453 = Str::lit("boolean");
        if equal(a2452, a2453) {
            true
        } else {
            let a2454 = v0.clone();
            let a2455 = Str::lit("object");
            if equal(a2454, a2455) {
                true
            } else {
                let a2456 = v0.clone();
                let a2457 = Str::lit("array");
                if equal(a2456, a2457) {
                    true
                } else {
                    let a2458 = v0.clone();
                    let a2459 = Str::lit("number");
                    if equal(a2458, a2459) {
                        true
                    } else {
                        let a2460 = v0.clone();
                        let a2461 = Str::lit("string");
                        if equal(a2460, a2461) {
                            true
                        } else {
                            let a2462 = v0;
                            let a2463 = Str::lit("integer");
                            equal(a2462, a2463)
                        }
                    }
                }
            }
        }
    }
}

pub fn f712(v0: Str, v1: Adt6) -> R<bool> {
    let a2464 = v0.clone();
    let a2465 = f804(v1.clone());
    if equal(a2464, a2465) {
        Ok(true)
    } else {
        let a2466 = v0;
        let a2467 = Str::lit("integer");
        if equal(a2466, a2467) {
            let m2468 = f716(v1);
            match m2468 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f805(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f713(p2471: List<Adt6>, v1: Adt6, p2472: bool) -> R<Adt157> {
    let m2470 = (p2471, p2472);
    {
        let mut m2470 = m2470;
        loop {
            let step = {
                let (v0, v2) = m2470;
                let m2469 = v0;
                match m2469.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f711(f806(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt157>>((v4, if v2 {
                                true
                            } else {
                                f712(f806(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt157>>(Ok(Adt157::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(p2476: List<Adt6>, v1: Adt6) -> R<Adt157> {
    let m2475 = p2476;
    {
        let mut m2475 = m2475;
        loop {
            let step = {
                let v0 = m2475;
                let m2473 = v0;
                match m2473.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C1))
                    }
                    Some((v2, v3)) => {
                        let m2474 = f715(v2, v1.clone())?;
                        match m2474 {
                            Adt164::C0 => {
                                Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C0))
                            }
                            Adt164::C1 => {
                                Ok::<List<Adt6>, R<Adt157>>(v3)
                            }
                            Adt164::C2 => {
                                Err::<List<Adt6>, R<Adt157>>(Ok(Adt157::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f715(v0: Adt6, v1: Adt6) -> R<Adt164> {
    f807(f808(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f716(v0: Adt6) -> Option<Adt7> {
    let m2477 = v0;
    match m2477 {
        Adt6::C0 => {
            None::<Adt7>
        }
        Adt6::C1(_) => {
            None::<Adt7>
        }
        Adt6::C2(v2) => {
            Some(v2)
        }
        Adt6::C3(_) => {
            None::<Adt7>
        }
        Adt6::C4(_) => {
            None::<Adt7>
        }
        Adt6::C5(_) => {
            None::<Adt7>
        }
    }
}

pub fn f717(v0: u64, v1: Adt163) -> bool {
    let a2478 = v0;
    let a2479 = 0u64;
    if nat_eq(a2478, a2479) {
        let a2480 = f809(v1);
        bool_not(a2480)
    } else {
        let a2481 = v0;
        let a2482 = 1u64;
        if nat_eq(a2481, a2482) {
            let a2483 = f709(v1);
            bool_not(a2483)
        } else {
            let a2484 = v0;
            let a2485 = 2u64;
            if nat_eq(a2484, a2485) {
                f709(v1)
            } else {
                f809(v1)
            }
        }
    }
}

pub fn f718(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt157> {
    let m2486 = f337(v0, v2);
    match m2486 {
        Adt109::C0 => {
            Ok(Adt157::C0)
        }
        Adt109::C1(v4) => {
            let m2487 = f230(v4);
            match m2487 {
                None => {
                    Ok(Adt157::C2)
                }
                Some(v5) => {
                    let m2490 = {
                        let a2488 = v5;
                        let a2489 = 0i64;
                        compare(a2488, a2489)
                    };
                    if match m2490 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Ok(Adt157::C2)
                    } else {
                        if v1 {
                            if f809(f810(f344(v3, 0i64)?, v5)) {
                                Ok(Adt157::C1)
                            } else {
                                Ok(Adt157::C0)
                            }
                        } else {
                            if f709(f810(f344(v3, 0i64)?, v5)) {
                                Ok(Adt157::C1)
                            } else {
                                Ok(Adt157::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f719(v0: Str) -> R<u64> {
    f811(f10({
        let a2491 = v0;
        utf8_encode(a2491)
    })?, 0u64)
}

pub fn f720(p2496: List<Adt6>, p2497: u64) -> R<u64> {
    let m2495 = (p2496, p2497);
    {
        let mut m2495 = m2495;
        loop {
            let step = {
                let (v0, v1) = m2495;
                let m2492 = v0;
                match m2492.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2493 = v1;
                            let a2494 = 1u64;
                            nat_add(a2493, a2494)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f721(p2502: List<(Str, Adt6)>, p2503: u64) -> R<u64> {
    let m2501 = (p2502, p2503);
    {
        let mut m2501 = m2501;
        loop {
            let step = {
                let (v0, v1) = m2501;
                let m2498 = v0;
                match m2498.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2499 = v1;
                            let a2500 = 1u64;
                            nat_add(a2499, a2500)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(p2507: List<Adt6>, v1: Adt5) -> Adt157 {
    let m2506 = p2507;
    {
        let mut m2506 = m2506;
        loop {
            let step = {
                let v0 = m2506;
                let m2504 = v0;
                match m2504.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt157>(Adt157::C0)
                    }
                    Some((v2, v3)) => {
                        let m2505 = f131(v2);
                        match m2505 {
                            None => {
                                Err::<List<Adt6>, Adt157>(Adt157::C2)
                            }
                            Some(v4) => {
                                if f374(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt157>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt157>(Adt157::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f723(p2514: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt118>, p2515: List<Adt120>) -> List<Adt120> {
    let m2513 = (p2514, p2515);
    {
        let mut m2513 = m2513;
        loop {
            let step = {
                let (v0, v3) = m2513;
                let m2508 = v0;
                match m2508.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2510 = f337({
                            let (h2509, _) = v4.clone();
                            h2509
                        }, v1.clone());
                        match m2510 {
                            Adt109::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>((v5, v3))
                            }
                            Adt109::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>((v5, List::cons(Adt120::C0({
                                    let (_, h2511) = v4;
                                    h2511
                                }, v6, List::cons(Adt118::C0({
                                    let (h2512, _) = v4;
                                    h2512
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2513 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(p2520: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2519 = p2520;
    {
        let mut m2519 = m2519;
        loop {
            let step = {
                let v0 = m2519;
                let m2516 = v0;
                match m2516.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2518 = f337({
                            let (h2517, _) = v2;
                            h2517
                        }, v1.clone());
                        match m2518 {
                            Adt109::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt109::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f725(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2521 = f337(Str::lit("properties"), v0);
    match m2521 {
        Adt109::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt109::C1(v1) => {
            let m2522 = f234(v1);
            match m2522 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2523) => {
                            h2523
                        }
                    }
                }
            }
        }
    }
}

pub fn f726(p2530: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt118>, p2531: List<Adt120>) -> List<Adt120> {
    let m2529 = (p2530, p2531);
    {
        let mut m2529 = m2529;
        loop {
            let step = {
                let (v0, v4) = m2529;
                let m2524 = v0;
                match m2524.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2526 = f337({
                            let (h2525, _) = v5.clone();
                            h2525
                        }, v1.clone());
                        match m2526 {
                            Adt109::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>((v6, List::cons(Adt120::C0(v2.clone(), {
                                    let (_, h2527) = v5;
                                    h2527
                                }, List::cons(Adt118::C0({
                                    let (h2528, _) = v5;
                                    h2528
                                }), v3.clone())), v4)))
                            }
                            Adt109::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt120>), List<Adt120>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(p2536: List<Adt6>, p2537: u64, v2: Adt6, v3: List<Adt118>, p2538: List<Adt120>) -> R<List<Adt120>> {
    let m2535 = (p2536, (p2537, p2538));
    {
        let mut m2535 = m2535;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2535;
                let m2532 = v0;
                match m2532.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt120>)), R<List<Adt120>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt120>)), R<List<Adt120>>>((v6, ({
                            let a2533 = v1;
                            let a2534 = 1u64;
                            nat_add(a2533, a2534)?
                        }, List::cons(Adt120::C0(v2.clone(), v5, List::cons(Adt118::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f728(p2541: List<Adt6>, v1: Adt6, v2: List<Adt118>, p2542: List<Adt120>) -> List<Adt120> {
    let m2540 = (p2541, p2542);
    {
        let mut m2540 = m2540;
        loop {
            let step = {
                let (v0, v3) = m2540;
                let m2539 = v0;
                match m2539.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt120>), List<Adt120>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt120>), List<Adt120>>((v5, List::cons(Adt120::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f729(v0: u64, v1: List<u64>) -> List<u64> {
    let a2543 = v0;
    let a2544 = 0u64;
    if nat_eq(a2543, a2544) {
        v1
    } else {
        let a2545 = v0;
        let a2546 = 1u64;
        if nat_eq(a2545, a2546) {
            List::cons(194u64, v1)
        } else {
            let a2547 = v0;
            let a2548 = 2u64;
            if nat_eq(a2547, a2548) {
                List::cons(226u64, v1)
            } else {
                let a2549 = v0;
                let a2550 = 3u64;
                if nat_eq(a2549, a2550) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f730(v0: u64, v1: u64, v2: List<u64>) -> Adt162 {
    let a2551 = v0;
    let a2552 = 0u64;
    if nat_eq(a2551, a2552) {
        f812(v1, v2)
    } else {
        let a2553 = v0;
        let a2554 = 1u64;
        if nat_eq(a2553, a2554) {
            let a2555 = 128u64;
            let a2556 = v1;
            if if nat_le(a2555, a2556) {
                let a2557 = v1;
                let a2558 = 159u64;
                nat_le(a2557, a2558)
            } else {
                false
            } {
                Adt162::C0(0u64, f813(v2))
            } else {
                f812(v1, List::cons(194u64, v2))
            }
        } else {
            let a2559 = v0;
            let a2560 = 2u64;
            if nat_eq(a2559, a2560) {
                let a2561 = v1;
                let a2562 = 128u64;
                if nat_eq(a2561, a2562) {
                    Adt162::C0(3u64, v2)
                } else {
                    let a2563 = v1;
                    let a2564 = 129u64;
                    if nat_eq(a2563, a2564) {
                        Adt162::C0(4u64, v2)
                    } else {
                        f812(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2565 = v0;
                let a2566 = 3u64;
                if nat_eq(a2565, a2566) {
                    let a2567 = 170u64;
                    let a2568 = v1;
                    if if nat_le(a2567, a2568) {
                        let a2569 = v1;
                        let a2570 = 174u64;
                        nat_le(a2569, a2570)
                    } else {
                        false
                    } {
                        Adt162::C0(0u64, f813(v2))
                    } else {
                        f812(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2571 = 166u64;
                    let a2572 = v1;
                    if if nat_le(a2571, a2572) {
                        let a2573 = v1;
                        let a2574 = 169u64;
                        nat_le(a2573, a2574)
                    } else {
                        false
                    } {
                        Adt162::C0(0u64, f813(v2))
                    } else {
                        f812(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f731(v0: Option<List<Adt44>>) -> List<(Str, Adt6)> {
    let m2575 = v0;
    match m2575 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f814(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f732(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2576 = v0;
    match m2576 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f733(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m2577 = v1;
    match m2577 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h2578) => {
                    h2578
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f734(v0: List<Adt2>) -> List<Adt6> {
    f436(f815(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f735(v0: Adt5) -> Option<Adt67> {
    let m2579 = f520(Str::lit("message"), v0.clone());
    match m2579 {
        None => {
            None::<Adt67>
        }
        Some(v1) => {
            let m2580 = f229(Str::lit("mode"), v0.clone());
            match m2580 {
                Adt109::C0 => {
                    let m2581 = f452(Str::lit("requestedSchema"), v0);
                    match m2581 {
                        None => {
                            None::<Adt67>
                        }
                        Some(v2) => {
                            Some(Adt67::C0(Adt68::C0(Adt69::C0(None::<Adt70>, v1, v2))))
                        }
                    }
                }
                Adt109::C1(v3) => {
                    let m2582 = f131(v3);
                    match m2582 {
                        None => {
                            None::<Adt67>
                        }
                        Some(v4) => {
                            let a2583 = v4.clone();
                            let a2584 = Str::lit("url");
                            if equal(a2583, a2584) {
                                let m2585 = f520(Str::lit("url"), v0);
                                match m2585 {
                                    None => {
                                        None::<Adt67>
                                    }
                                    Some(v5) => {
                                        Some(Adt67::C0(Adt68::C1(Adt71::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2586 = v4;
                                let a2587 = Str::lit("form");
                                if equal(a2586, a2587) {
                                    let m2588 = f452(Str::lit("requestedSchema"), v0);
                                    match m2588 {
                                        None => {
                                            None::<Adt67>
                                        }
                                        Some(v6) => {
                                            Some(Adt67::C0(Adt68::C0(Adt69::C0(Some(Adt70::C0), v1, v6))))
                                        }
                                    }
                                } else {
                                    None::<Adt67>
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f736(v0: Adt5) -> Option<Adt67> {
    let m2589 = f229(Str::lit("params"), v0);
    match m2589 {
        Adt109::C0 => {
            Some(Adt67::C2(None::<Adt5>))
        }
        Adt109::C1(v1) => {
            let m2590 = f234(v1);
            match m2590 {
                None => {
                    None::<Adt67>
                }
                Some(v2) => {
                    Some(Adt67::C2(Some(v2)))
                }
            }
        }
    }
}

pub fn f737() -> i64 {
    999999999999999999i64
}

pub fn f738(v0: i64) -> R<List<u64>> {
    let m2593 = {
        let a2591 = v0;
        let a2592 = 0i64;
        compare(a2591, a2592)
    };
    if match m2593 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Ok(List::cons(45u64, f816(20u64, {
            let a2594 = v0;
            int_neg(a2594)?
        }, List::<u64>::nil())?))
    } else {
        f816(20u64, v0, List::<u64>::nil())
    }
}

pub fn f739(v0: u64) -> R<List<u64>> {
    let a2595 = v0;
    let a2596 = 34u64;
    if nat_eq(a2595, a2596) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a2597 = v0;
        let a2598 = 92u64;
        if nat_eq(a2597, a2598) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a2599 = v0;
            let a2600 = 8u64;
            if nat_eq(a2599, a2600) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a2601 = v0;
                let a2602 = 9u64;
                if nat_eq(a2601, a2602) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a2603 = v0;
                    let a2604 = 10u64;
                    if nat_eq(a2603, a2604) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a2605 = v0;
                        let a2606 = 12u64;
                        if nat_eq(a2605, a2606) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a2607 = v0;
                            let a2608 = 13u64;
                            if nat_eq(a2607, a2608) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a2609 = v0;
                                let a2610 = 32u64;
                                if nat_lt(a2609, a2610) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f817(f784(v0, 16u64))?, List::cons(f817(f783(v0, 16u64))?, List::<u64>::nil())))))))
                                } else {
                                    Ok(List::cons(v0, List::<u64>::nil()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f740(v0: Adt36) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f818(match v0.clone() {
        Adt36::C0(_, h2611, _, _) => {
            h2611
        }
    })), List::cons((Str::lit("capabilities"), f819(match v0.clone() {
        Adt36::C0(_, _, h2612, _) => {
            h2612
        }
    })), {
        let a2615 = f657(Str::lit("instructions"), match v0.clone() {
            Adt36::C0(_, _, _, h2613) => {
                h2613
            }
        });
        let a2616 = f820(match v0 {
            Adt36::C0(h2614, _, _, _) => {
                h2614
            }
        })?;
        append_list(a2615, a2616)
    })))
}

pub fn f741(v0: Adt37) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f821(match v0.clone() {
        Adt37::C0(_, _, h2617) => {
            h2617
        }
    }))), {
        let a2620 = f657(Str::lit("nextCursor"), match v0.clone() {
            Adt37::C0(_, h2618, _) => {
                h2618
            }
        });
        let a2621 = f820(match v0 {
            Adt37::C0(h2619, _, _) => {
                h2619
            }
        })?;
        append_list(a2620, a2621)
    }))
}

pub fn f742(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f822(match v0.clone() {
        Adt54::C0(_, _, h2622) => {
            h2622
        }
    })?)), {
        let a2625 = f657(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h2623, _) => {
                h2623
            }
        });
        let a2626 = f820(match v0 {
            Adt54::C0(h2624, _, _) => {
                h2624
            }
        })?;
        append_list(a2625, a2626)
    }))
}

pub fn f743(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f823(match v0.clone() {
        Adt55::C0(_, h2627) => {
            h2627
        }
    }))), f820(match v0 {
        Adt55::C0(h2628, _) => {
            h2628
        }
    })?))
}

pub fn f744(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f824(match v0.clone() {
        Adt56::C0(_, _, h2629) => {
            h2629
        }
    }))), {
        let a2632 = f657(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h2630, _) => {
                h2630
            }
        });
        let a2633 = f820(match v0 {
            Adt56::C0(h2631, _, _) => {
                h2631
            }
        })?;
        append_list(a2632, a2633)
    }))
}

pub fn f745(v0: Adt58) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f825(match v0.clone() {
        Adt58::C0(_, _, h2634) => {
            h2634
        }
    }))), {
        let a2637 = f657(Str::lit("nextCursor"), match v0.clone() {
            Adt58::C0(_, h2635, _) => {
                h2635
            }
        });
        let a2638 = f820(match v0 {
            Adt58::C0(h2636, _, _) => {
                h2636
            }
        })?;
        append_list(a2637, a2638)
    }))
}

pub fn f746(v0: Adt61) -> R<Option<List<(Str, Adt6)>>> {
    let m2640 = f826(match v0.clone() {
        Adt61::C0(_, h2639) => {
            h2639
        }
    })?;
    match m2640 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2642 = f657(Str::lit("description"), match v0 {
                    Adt61::C0(h2641, _) => {
                        h2641
                    }
                });
                let a2643 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a2642, a2643)
            }))
        }
    }
}

pub fn f747(v0: Adt63) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f769({
        let m2646 = match v0.clone() {
            Adt63::C0(h2645) => {
                h2645
            }
        };
        match m2646 {
            Adt64::C0(h2644, _, _) => {
                h2644
            }
        }
    }))), {
        let a2653 = f827(Str::lit("total"), {
            let m2649 = match v0.clone() {
                Adt63::C0(h2648) => {
                    h2648
                }
            };
            match m2649 {
                Adt64::C0(_, h2647, _) => {
                    h2647
                }
            }
        })?;
        let a2654 = f342(Str::lit("hasMore"), {
            let m2652 = match v0 {
                Adt63::C0(h2651) => {
                    h2651
                }
            };
            match m2652 {
                Adt64::C0(_, _, h2650) => {
                    h2650
                }
            }
        });
        append_list(a2653, a2654)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f748(v0: Adt84) -> R<Option<List<(Str, Adt6)>>> {
    let m2655 = f828(f829(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m2655 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a2656 = f830(v1);
                let a2657 = f831(f832(v0));
                append_list(a2656, a2657)
            }))
        }
    }
}

pub fn f749(v0: Adt65) -> Option<List<(Str, Adt6)>> {
    let m2658 = f833(v0.clone());
    match m2658 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a2659 = f834(f377(v0.clone()), f835(f836(v0)));
                let a2660 = v1;
                append_list(a2659, a2660)
            })
        }
    }
}

pub fn f750(v0: Adt90) -> Option<List<(Str, Adt6)>> {
    let m2662 = match v0.clone() {
        Adt90::C0(_, h2661, _) => {
            h2661
        }
    };
    match m2662 {
        Adt91::C0 => {
            f749({
                let m2665 = match v0 {
                    Adt90::C0(h2664, _, _) => {
                        h2664
                    }
                };
                match m2665 {
                    Adt87::C0(h2663, _, _) => {
                        h2663
                    }
                }
            })
        }
        Adt91::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f751(v0: Adt165) -> Str {
    let m2666 = v0;
    match m2666 {
        Adt165::C0 => {
            Str::lit("complete")
        }
        Adt165::C1 => {
            Str::lit("input_required")
        }
        Adt165::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f752(v0: Adt35) -> Adt165 {
    let m2667 = v0;
    match m2667 {
        Adt35::C0(_) => {
            Adt165::C0
        }
        Adt35::C1(_) => {
            Adt165::C0
        }
        Adt35::C2(_) => {
            Adt165::C0
        }
        Adt35::C3(_) => {
            Adt165::C0
        }
        Adt35::C4(_) => {
            Adt165::C0
        }
        Adt35::C5(_) => {
            Adt165::C0
        }
        Adt35::C6(_) => {
            Adt165::C0
        }
        Adt35::C7(_) => {
            Adt165::C0
        }
        Adt35::C8(_) => {
            Adt165::C0
        }
        Adt35::C9(_) => {
            Adt165::C0
        }
        Adt35::C10(_) => {
            Adt165::C0
        }
        Adt35::C11 => {
            Adt165::C0
        }
        Adt35::C12(_) => {
            Adt165::C1
        }
        Adt35::C13(_) => {
            Adt165::C2
        }
        Adt35::C14(_) => {
            Adt165::C0
        }
    }
}

pub fn f753(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m2668 = v0;
    match m2668 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f837(), f838(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f754(v0: List<Adt87>) -> R<Option<List<u64>>> {
    let m2669 = f839(v0);
    match m2669 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m2670 = f298(v1)?;
            match m2670 {
                Ok(v3) => {
                    Ok(Some(v3))
                }
                Err(_) => {
                    Ok(None::<List<u64>>)
                }
            }
        }
    }
}

pub fn f755(v0: Adt87, v1: List<Adt87>, v2: i64) -> R<List<Adt87>> {
    Ok(List::cons(v0.clone(), f375(f840(f291(v0), v1, v2, List::<Adt87>::nil())?, List::<Adt87>::nil())))
}

pub fn f756(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f841(v2) {
            let a2671 = v1;
            let a2672 = 200u64;
            nat_eq(a2671, a2672)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f757(v0: u64, v1: List<List<u64>>) -> Adt100 {
    Adt100::C0(v0, f842(), List::<List<u64>>::nil(), f843(v1), None::<List<u64>>)
}

pub fn f758(v0: Adt146) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f387())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f844(match v0.clone() {
        Adt146::C0(h2673, _) => {
            h2673
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m2676 = match v0.clone() {
            Adt146::C0(_, h2675) => {
                h2675
            }
        };
        match m2676 {
            Adt53::C0(h2674, _, _) => {
                h2674
            }
        }
    })), {
        let a2683 = f845({
            let m2679 = match v0.clone() {
                Adt146::C0(_, h2678) => {
                    h2678
                }
            };
            match m2679 {
                Adt53::C0(_, h2677, _) => {
                    h2677
                }
            }
        });
        let a2684 = f846({
            let m2682 = match v0 {
                Adt146::C0(_, h2681) => {
                    h2681
                }
            };
            match m2682 {
                Adt53::C0(_, _, h2680) => {
                    h2680
                }
            }
        });
        append_list(a2683, a2684)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f759(v0: Adt90) -> bool {
    if match v0.clone() {
        Adt90::C0(_, _, h2685) => {
            h2685
        }
    } {
        let m2687 = match v0 {
            Adt90::C0(_, h2686, _) => {
                h2686
            }
        };
        match m2687 {
            Adt91::C0 => {
                true
            }
            Adt91::C1 => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f760(v0: Adt65) -> Option<Adt6> {
    let m2688 = f749(v0);
    match m2688 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f761() -> u64 {
    429u64
}

pub fn f762() -> i64 {
    -32602i64
}

pub fn f763() -> i64 {
    -32022i64
}

pub fn f764() -> i64 {
    -32020i64
}

pub fn f765() -> i64 {
    -32601i64
}

pub fn f766(v0: Adt92) -> i64 {
    let m2689 = v0;
    match m2689 {
        Adt92::C0(_) => {
            f762()
        }
        Adt92::C1(_) => {
            f762()
        }
        Adt92::C2(_) => {
            f762()
        }
        Adt92::C3(_) => {
            f762()
        }
        Adt92::C4(_) => {
            f762()
        }
        Adt92::C5(_) => {
            f762()
        }
        Adt92::C6(_) => {
            f847()
        }
        Adt92::C7(_) => {
            f848()
        }
        Adt92::C8(_) => {
            f849()
        }
        Adt92::C9(_) => {
            f847()
        }
        Adt92::C10 => {
            f762()
        }
        Adt92::C11 => {
            f762()
        }
        Adt92::C12(_) => {
            f847()
        }
        Adt92::C13 => {
            f847()
        }
        Adt92::C14 => {
            f847()
        }
        Adt92::C15(_) => {
            f847()
        }
    }
}

pub fn f767(v0: Adt148) -> Str {
    let m2690 = v0;
    match m2690 {
        Adt148::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt148::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt148::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt148::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt148::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt148::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt148::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt148::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt148::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f768(v0: Adt129) -> Str {
    let m2691 = v0;
    match m2691 {
        Adt129::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt129::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt129::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt129::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt129::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt129::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt129::C6(v1) => {
            let m2692 = v1;
            match m2692 {
                Adt130::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt130::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt130::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt129::C7(v2) => {
            let m2693 = v2;
            match m2693 {
                Adt130::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt130::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt130::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt129::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt129::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt129::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f769(v0: List<Str>) -> List<Adt6> {
    f436(f850(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f770(v0: Adt20) -> Adt6 {
    Adt6::C5({
        let a2705 = f733(Str::lit("experimental"), match v0.clone() {
            Adt20::C0(h2694, _, _, _, _) => {
                h2694
            }
        });
        let a2706 = {
            let a2703 = f733(Str::lit("roots"), match v0.clone() {
                Adt20::C0(_, h2695, _, _, _) => {
                    h2695
                }
            });
            let a2704 = {
                let a2701 = f733(Str::lit("sampling"), match v0.clone() {
                    Adt20::C0(_, _, h2696, _, _) => {
                        h2696
                    }
                });
                let a2702 = {
                    let a2699 = f733(Str::lit("elicitation"), match v0.clone() {
                        Adt20::C0(_, _, _, h2697, _) => {
                            h2697
                        }
                    });
                    let a2700 = f733(Str::lit("extensions"), match v0 {
                        Adt20::C0(_, _, _, _, h2698) => {
                            h2698
                        }
                    });
                    append_list(a2699, a2700)
                };
                append_list(a2701, a2702)
            };
            append_list(a2703, a2704)
        };
        append_list(a2705, a2706)
    })
}

pub fn f771(v0: List<u64>) -> R<bool> {
    let m2707 = v0;
    match m2707.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m2708 = v2;
            match m2708.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a2709 = f479(v1)?;
                    let a2710 = 113u64;
                    if if nat_eq(a2709, a2710) {
                        let a2711 = v3;
                        let a2712 = 61u64;
                        nat_eq(a2711, a2712)
                    } else {
                        false
                    } {
                        Ok(f851(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f772(v0: Adt0, v1: Fn9, v2: u64, v3: Result<Adt17, Adt148>) -> R<Result<Adt144, Adt147>> {
    let m2713 = v3;
    match m2713 {
        Ok(v5) => {
            f852(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt144, Adt147>(Adt147::C0(v4)))
        }
    }
}

pub fn f773(v0: Adt154, v1: Adt19, v2: Adt5) -> Result<Adt17, Adt148> {
    let m2714 = v0;
    match m2714 {
        Adt154::C0 => {
            f853(v1, v2)
        }
        Adt154::C1 => {
            f854(v1, v2)
        }
        Adt154::C2 => {
            f855(v1, v2)
        }
        Adt154::C3 => {
            f856(v1, v2)
        }
        Adt154::C4 => {
            f857(v1, v2)
        }
        Adt154::C5 => {
            f858(v1, v2)
        }
        Adt154::C6 => {
            f859(v1, v2)
        }
        Adt154::C7 => {
            f860(v1, v2)
        }
        Adt154::C8 => {
            f861(v1, v2)
        }
        Adt154::C9 => {
            f862(v1, v2)
        }
        Adt154::C10 => {
            f863(v1, v2)
        }
        Adt154::C11 => {
            f864(v1, v2)
        }
    }
}

pub fn f774(v0: Adt154) -> Str {
    let m2715 = v0;
    match m2715 {
        Adt154::C0 => {
            Str::lit("server/discover")
        }
        Adt154::C1 => {
            Str::lit("tools/list")
        }
        Adt154::C2 => {
            Str::lit("tools/call")
        }
        Adt154::C3 => {
            Str::lit("resources/list")
        }
        Adt154::C4 => {
            Str::lit("resources/read")
        }
        Adt154::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt154::C6 => {
            Str::lit("prompts/list")
        }
        Adt154::C7 => {
            Str::lit("prompts/get")
        }
        Adt154::C8 => {
            Str::lit("completion/complete")
        }
        Adt154::C9 => {
            Str::lit("tasks/get")
        }
        Adt154::C10 => {
            Str::lit("tasks/update")
        }
        Adt154::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f775(v0: Option<Adt8>) -> bool {
    let m2716 = v0;
    match m2716 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f776(v0: Option<Adt9>) -> bool {
    let m2717 = v0;
    match m2717 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f777(v0: List<Str>, v1: Adt122, v2: Adt19, v3: Option<Adt129>) -> Adt159 {
    let m2718 = v3;
    match m2718 {
        None => {
            f865(v1, v2.clone(), f866(match v2 {
                Adt19::C0(h2719, _, _, _, _, _) => {
                    h2719
                }
            }, v0))
        }
        Some(v4) => {
            Adt159::C1(Adt147::C2(v4))
        }
    }
}

pub fn f778(v0: Option<Adt128>, v1: Adt161, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt129>> {
    let m2720 = v0;
    match m2720 {
        None => {
            Ok(None::<Adt129>)
        }
        Some(v5) => {
            let m2722 = match v5.clone() {
                Adt128::C0(_, _, _, h2721, _) => {
                    h2721
                }
            };
            match m2722 {
                None => {
                    f867(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f779(v0: Option<Adt5>) -> Adt5 {
    let m2723 = v0;
    match m2723 {
        None => {
            f32()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f780(v0: Adt5) -> Result<Adt19, Adt148> {
    let m2724 = f868(v0.clone());
    match m2724 {
        Adt166::C0 => {
            f869(v0.clone(), f229(f870(), v0.clone()), f229(f871(), v0))
        }
        Adt166::C1 => {
            Err::<Adt19, Adt148>(Adt148::C2)
        }
        Adt166::C2 => {
            Err::<Adt19, Adt148>(Adt148::C3)
        }
    }
}

pub fn f781() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f782(v0: Str) -> R<List<u64>> {
    f10({
        let a2725 = v0;
        utf8_encode(a2725)
    })
}

pub fn f783(v0: u64, v1: u64) -> u64 {
    let a2726 = v0;
    let a2727 = v1;
    let a2728 = 0u64;
    nat_rem(a2726, a2727, a2728)
}

pub fn f784(v0: u64, v1: u64) -> u64 {
    let a2729 = v0;
    let a2730 = v1;
    let a2731 = 0u64;
    nat_quot(a2729, a2730, a2731)
}

pub fn f785(v0: Adt6) -> bool {
    let m2732 = f131(v0);
    match m2732 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f786(v0: Adt6) -> bool {
    let m2733 = f716(v0);
    match m2733 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f787(v0: Adt6) -> bool {
    let m2734 = f230(v0);
    match m2734 {
        None => {
            false
        }
        Some(v1) => {
            let a2738 = {
                let m2737 = {
                    let a2735 = v1;
                    let a2736 = 0i64;
                    compare(a2735, a2736)
                };
                match m2737 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                }
            };
            bool_not(a2738)
        }
    }
}

pub fn f788(v0: Adt6) -> bool {
    let m2739 = f289(v0);
    match m2739 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f789(v0: Adt6) -> bool {
    let m2740 = f234(v0);
    match m2740 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f790(v0: Adt6) -> bool {
    let m2741 = f289(v0);
    match m2741 {
        None => {
            false
        }
        Some(v1) => {
            f872(v1)
        }
    }
}

pub fn f791(v0: Adt6) -> bool {
    let m2742 = f438(v0);
    match m2742 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f792(v0: Adt6) -> bool {
    let m2743 = f131(v0.clone());
    match m2743 {
        None => {
            let m2744 = f289(v0);
            match m2744 {
                None => {
                    false
                }
                Some(v2) => {
                    f873(v2)
                }
            }
        }
        Some(v1) => {
            f711(v1)
        }
    }
}

pub fn f793(v0: Adt6) -> bool {
    let m2745 = f289(v0);
    match m2745 {
        None => {
            false
        }
        Some(v1) => {
            let m2746 = v1;
            match m2746.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
    }
}

pub fn f794(v0: Str, p2764: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2763 = p2764;
    {
        let mut m2763 = m2763;
        loop {
            let step = {
                let v1 = m2763;
                let m2747 = v1;
                match m2747.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2751 = v0.clone();
                        let a2752 = {
                            let a2749 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2748, _) = v2.clone();
                                h2748
                            }, List::<Str>::nil()));
                            let a2750 = Str::lit("");
                            join(a2749, a2750)
                        };
                        if equal(a2751, a2752) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2753) = v2;
                                h2753
                            }))
                        } else {
                            let m2755 = f234({
                                let (_, h2754) = v2.clone();
                                h2754
                            });
                            match m2755 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2756 = f229(Str::lit("$anchor"), v4);
                                    match m2756 {
                                        Adt109::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt109::C1(v5) => {
                                            let m2757 = f131(v5);
                                            match m2757 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2760 = v0.clone();
                                                    let a2761 = {
                                                        let a2758 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2759 = Str::lit("");
                                                        join(a2758, a2759)
                                                    };
                                                    if equal(a2760, a2761) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2762) = v2;
                                                            h2762
                                                        }))
                                                    } else {
                                                        Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f795(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2765 = f337(Str::lit("type"), v0);
    match m2765 {
        Adt109::C0 => {
            None::<Adt13>
        }
        Adt109::C1(v1) => {
            let m2766 = f131(v1);
            match m2766 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f874(v2)
                }
            }
        }
    }
}

pub fn f796(v0: List<u64>) -> R<List<u64>> {
    Ok(f459(f875(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f797(v0: List<u64>, p2769: List<List<u64>>) -> bool {
    let m2768 = p2769;
    {
        let mut m2768 = m2768;
        loop {
            let step = {
                let v1 = m2768;
                let m2767 = v1;
                match m2767.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f876(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f798(v0: u64) -> R<Option<u64>> {
    let a2770 = 64u64;
    let a2771 = v0;
    if if nat_lt(a2770, a2771) {
        let a2772 = v0;
        let a2773 = 91u64;
        nat_lt(a2772, a2773)
    } else {
        false
    } {
        Ok(Some({
            let a2774 = v0;
            let a2775 = 65u64;
            nat_sub(a2774, a2775)
        }))
    } else {
        let a2776 = 96u64;
        let a2777 = v0;
        if if nat_lt(a2776, a2777) {
            let a2778 = v0;
            let a2779 = 123u64;
            nat_lt(a2778, a2779)
        } else {
            false
        } {
            Ok(Some({
                let a2780 = v0;
                let a2781 = 71u64;
                nat_sub(a2780, a2781)
            }))
        } else {
            let a2782 = 47u64;
            let a2783 = v0;
            if if nat_lt(a2782, a2783) {
                let a2784 = v0;
                let a2785 = 58u64;
                nat_lt(a2784, a2785)
            } else {
                false
            } {
                Ok(Some({
                    let a2786 = v0;
                    let a2787 = 4u64;
                    nat_add(a2786, a2787)?
                }))
            } else {
                let a2788 = v0;
                let a2789 = 43u64;
                if nat_eq(a2788, a2789) {
                    Ok(Some(62u64))
                } else {
                    let a2790 = v0;
                    let a2791 = 47u64;
                    if nat_eq(a2790, a2791) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f799(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2792 = v3;
        let a2793 = 61u64;
        if nat_eq(a2792, a2793) {
            let a2794 = f877(v1, 16u64);
            let a2795 = 0u64;
            nat_eq(a2794, a2795)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f878(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f800(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2796 = f877(v2, 4u64);
        let a2797 = 0u64;
        nat_eq(a2796, a2797)
    } else {
        false
    } {
        Ok(Some(f879(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f801(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2800 = {
            let a2798 = v0;
            let a2799 = 4u64;
            nat_mul(a2798, a2799)?
        };
        let a2801 = f880(v1, 16u64);
        nat_add(a2800, a2801)?
    }, List::cons({
        let a2804 = {
            let a2802 = f877(v1, 16u64);
            let a2803 = 16u64;
            nat_mul(a2802, a2803)?
        };
        let a2805 = f880(v2, 4u64);
        nat_add(a2804, a2805)?
    }, List::cons({
        let a2808 = {
            let a2806 = f877(v2, 4u64);
            let a2807 = 64u64;
            nat_mul(a2806, a2807)?
        };
        let a2809 = v3;
        nat_add(a2808, a2809)?
    }, List::<u64>::nil()))))
}

pub fn f802(v0: Adt7, v1: Adt7, v2: Adt163, v3: Adt163) -> R<Adt163> {
    let m2810 = v2;
    match m2810 {
        Adt163::C0 => {
            let m2812 = v3;
            match m2812 {
                Adt163::C0 => {
                    Ok(f881(Adt163::C0, f882(f883(match v0.clone() {
                        Adt7::C0(h2813, _) => {
                            h2813
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2814) => {
                            h2814
                        }
                    }, f883(match v1.clone() {
                        Adt7::C0(h2815, _) => {
                            h2815
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2816) => {
                            h2816
                        }
                    })?))
                }
                Adt163::C1 => {
                    Ok(Adt163::C0)
                }
                Adt163::C2 => {
                    Ok(Adt163::C0)
                }
            }
        }
        Adt163::C1 => {
            let m2811 = v3;
            match m2811 {
                Adt163::C0 => {
                    Ok(Adt163::C2)
                }
                Adt163::C1 => {
                    Ok(Adt163::C1)
                }
                Adt163::C2 => {
                    Ok(Adt163::C0)
                }
            }
        }
        Adt163::C2 => {
            let m2817 = v3;
            match m2817 {
                Adt163::C0 => {
                    Ok(Adt163::C2)
                }
                Adt163::C1 => {
                    Ok(Adt163::C2)
                }
                Adt163::C2 => {
                    f882(match v0.clone() {
                        Adt7::C0(h2818, _) => {
                            h2818
                        }
                    }, match v0 {
                        Adt7::C0(_, h2819) => {
                            h2819
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2820, _) => {
                            h2820
                        }
                    }, match v1 {
                        Adt7::C0(_, h2821) => {
                            h2821
                        }
                    })
                }
            }
        }
    }
}

pub fn f803(v0: i64) -> Adt163 {
    f810(v0, 0i64)
}

pub fn f804(v0: Adt6) -> Str {
    let m2822 = v0;
    match m2822 {
        Adt6::C0 => {
            Str::lit("null")
        }
        Adt6::C1(_) => {
            Str::lit("boolean")
        }
        Adt6::C2(_) => {
            Str::lit("number")
        }
        Adt6::C3(_) => {
            Str::lit("string")
        }
        Adt6::C4(_) => {
            Str::lit("array")
        }
        Adt6::C5(_) => {
            Str::lit("object")
        }
    }
}

pub fn f805(v0: Adt7) -> R<bool> {
    if f338(match v0.clone() {
        Adt7::C0(h2823, _) => {
            h2823
        }
    }) {
        Ok(true)
    } else {
        f884(40u64, match v0.clone() {
            Adt7::C0(h2824, _) => {
                h2824
            }
        }, match v0 {
            Adt7::C0(_, h2825) => {
                h2825
            }
        })
    }
}

pub fn f806(v0: Adt6) -> Str {
    let m2826 = f131(v0);
    match m2826 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f807(p2833: u64, p2834: List<(Adt6, Adt6)>) -> R<Adt164> {
    let m2832 = (p2833, p2834);
    {
        let mut m2832 = m2832;
        loop {
            let step = {
                let (v0, v1) = m2832;
                let m2827 = v0;
                if m2827 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt164>>(Ok(Adt164::C2))
                } else {
                    let v2 = m2827 - 1;
                    let m2828 = v1;
                    match m2828.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt164>>(Ok(Adt164::C0))
                        }
                        Some((v3, v4)) => {
                            let m2831 = f885({
                                let (h2829, _) = v3;
                                h2829
                            }, {
                                let (_, h2830) = v3;
                                h2830
                            }, v4)?;
                            match m2831 {
                                Adt167::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt164>>(Ok(Adt164::C1))
                                }
                                Adt167::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt164>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2832 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f808() -> u64 {
    67108864u64
}

pub fn f809(v0: Adt163) -> bool {
    let m2835 = v0;
    match m2835 {
        Adt163::C0 => {
            true
        }
        Adt163::C1 => {
            false
        }
        Adt163::C2 => {
            false
        }
    }
}

pub fn f810(v0: i64, v1: i64) -> Adt163 {
    let m2838 = {
        let a2836 = v0;
        let a2837 = v1;
        compare(a2836, a2837)
    };
    if match m2838 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        Adt163::C0
    } else {
        let m2841 = {
            let a2839 = v1;
            let a2840 = v0;
            compare(a2839, a2840)
        };
        if match m2841 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            Adt163::C2
        } else {
            Adt163::C1
        }
    }
}

pub fn f811(p2850: List<u64>, p2851: u64) -> R<u64> {
    let m2849 = (p2850, p2851);
    {
        let mut m2849 = m2849;
        loop {
            let step = {
                let (v0, v1) = m2849;
                let m2842 = v0;
                match m2842.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2843 = v2;
                        let a2844 = 128u64;
                        if if nat_lt(a2843, a2844) {
                            true
                        } else {
                            let a2845 = 191u64;
                            let a2846 = v2;
                            nat_lt(a2845, a2846)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2847 = v1;
                                let a2848 = 1u64;
                                nat_add(a2847, a2848)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f812(v0: u64, v1: List<u64>) -> Adt162 {
    let a2852 = v0;
    let a2853 = 32u64;
    if nat_lt(a2852, a2853) {
        if f886(v0) {
            Adt162::C0(0u64, List::cons(v0, v1))
        } else {
            Adt162::C0(0u64, f813(v1))
        }
    } else {
        let a2854 = v0;
        let a2855 = 127u64;
        if nat_eq(a2854, a2855) {
            Adt162::C0(0u64, f813(v1))
        } else {
            let a2856 = v0;
            let a2857 = 194u64;
            if nat_eq(a2856, a2857) {
                Adt162::C0(1u64, v1)
            } else {
                let a2858 = v0;
                let a2859 = 226u64;
                if nat_eq(a2858, a2859) {
                    Adt162::C0(2u64, v1)
                } else {
                    Adt162::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f813(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f814(v0: List<Adt44>) -> List<Adt6> {
    f436(f887(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f815(p2862: List<Adt2>, p2863: List<Adt6>) -> List<Adt6> {
    let m2861 = (p2862, p2863);
    {
        let mut m2861 = m2861;
        loop {
            let step = {
                let (v0, v1) = m2861;
                let m2860 = v0;
                match m2860.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f888(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f816(p2875: u64, p2876: i64, p2877: List<u64>) -> R<List<u64>> {
    let m2874 = (p2875, (p2876, p2877));
    {
        let mut m2874 = m2874;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2874;
                let m2864 = v0;
                if m2864 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m2864 - 1;
                    let v4: i64 = {
                        let a2865 = v1;
                        let a2866 = 10i64;
                        let a2867 = 0i64;
                        int_quot(a2865, a2866, a2867)?
                    };
                    let v5: i64 = {
                        let a2868 = v1;
                        let a2869 = 10i64;
                        let a2870 = 0i64;
                        int_rem(a2868, a2869, a2870)
                    };
                    let m2873 = {
                        let a2871 = v4;
                        let a2872 = 1i64;
                        compare(a2871, a2872)
                    };
                    if match m2873 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f889(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f889(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f817(v0: u64) -> R<u64> {
    let a2878 = v0;
    let a2879 = 10u64;
    if nat_lt(a2878, a2879) {
        let a2880 = 48u64;
        let a2881 = v0;
        nat_add(a2880, a2881)
    } else {
        let a2882 = 87u64;
        let a2883 = v0;
        nat_add(a2882, a2883)
    }
}

pub fn f818(v0: List<Str>) -> Adt6 {
    Adt6::C4(f769(v0))
}

pub fn f819(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a2901 = f733(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h2884, _, _, _, _, _, _) => {
                h2884
            }
        });
        let a2902 = {
            let a2899 = f733(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h2885, _, _, _, _, _) => {
                    h2885
                }
            });
            let a2900 = {
                let a2897 = f733(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h2886, _, _, _, _) => {
                        h2886
                    }
                });
                let a2898 = {
                    let a2895 = f890(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h2887, _, _, _) => {
                            h2887
                        }
                    });
                    let a2896 = {
                        let a2893 = f891(match v0.clone() {
                            Adt4::C0(_, _, _, _, h2888, _, _) => {
                                h2888
                            }
                        });
                        let a2894 = {
                            let a2891 = f890(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h2889, _) => {
                                    h2889
                                }
                            });
                            let a2892 = f733(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h2890) => {
                                    h2890
                                }
                            });
                            append_list(a2891, a2892)
                        };
                        append_list(a2893, a2894)
                    };
                    append_list(a2895, a2896)
                };
                append_list(a2897, a2898)
            };
            append_list(a2899, a2900)
        };
        append_list(a2901, a2902)
    })
}

pub fn f820(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f124(match v0.clone() {
        Adt10::C0(h2903, _) => {
            h2903
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f892(match v0 {
        Adt10::C0(_, h2904) => {
            h2904
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f821(v0: List<Adt38>) -> List<Adt6> {
    f436(f893(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f822(v0: List<Adt47>) -> R<List<Adt6>> {
    Ok(f436(f894(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f823(v0: List<Adt49>) -> List<Adt6> {
    f436(f895(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f824(v0: List<Adt57>) -> List<Adt6> {
    f436(f896(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f825(v0: List<Adt59>) -> List<Adt6> {
    f436(f897(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f826(v0: List<Adt62>) -> R<Option<List<Adt6>>> {
    let m2905 = f898(v0, List::<Adt6>::nil())?;
    match m2905 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f436(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f827(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2906 = v1;
    match m2906 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f124(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f828(p2912: List<(Str, Adt67)>, p2913: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m2911 = (p2912, p2913);
    {
        let mut m2911 = m2911;
        loop {
            let step = {
                let (v0, v1) = m2911;
                let m2907 = v0;
                match m2907.uncons() {
                    None => {
                        Err::<(List<(Str, Adt67)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2909 = f899({
                            let (_, h2908) = v2;
                            h2908
                        })?;
                        match m2909 {
                            None => {
                                Err::<(List<(Str, Adt67)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt67)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h2910, _) = v2;
                                    h2910
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f829(v0: Adt84) -> List<(Str, Adt67)> {
    let m2914 = v0;
    match m2914 {
        Adt84::C0(v1) => {
            v1
        }
        Adt84::C1(_) => {
            List::<(Str, Adt67)>::nil()
        }
        Adt84::C2(v3, _) => {
            v3
        }
    }
}

pub fn f830(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2915 = v0.clone();
    match m2915.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f506(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f831(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2916 = v0;
    match m2916 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f832(v0: Adt84) -> Option<Str> {
    let m2917 = v0;
    match m2917 {
        Adt84::C0(_) => {
            None::<Str>
        }
        Adt84::C1(v2) => {
            Some(v2)
        }
        Adt84::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f833(v0: Adt65) -> Option<List<(Str, Adt6)>> {
    let m2918 = v0;
    match m2918 {
        Adt65::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt65::C1(_, v3) => {
            let m2919 = f900(v3);
            match m2919 {
                None => {
                    None::<List<(Str, Adt6)>>
                }
                Some(v4) => {
                    Some(List::cons((Str::lit("inputRequests"), v4), List::<(Str, Adt6)>::nil()))
                }
            }
        }
        Adt65::C2(_, v6) => {
            Some(List::cons((Str::lit("result"), Adt6::C5(match v6 {
                Adt5::C0(h2920) => {
                    h2920
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt65::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f389(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt65::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f834(v0: Adt66, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt66::C0(h2921, _, _, _, _, _) => {
            h2921
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a2927 = f901(Str::lit("statusMessage"), match v0.clone() {
            Adt66::C0(_, h2922, _, _, _, _) => {
                h2922
            }
        });
        let a2928 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, _, h2923, _, _, _) => {
                h2923
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, _, _, h2924, _, _) => {
                h2924
            }
        })), List::cons((Str::lit("ttlMs"), f902(match v0.clone() {
            Adt66::C0(_, _, _, _, h2925, _) => {
                h2925
            }
        })), f903(Str::lit("pollIntervalMs"), match v0 {
            Adt66::C0(_, _, _, _, _, h2926) => {
                h2926
            }
        }))));
        append_list(a2927, a2928)
    }))
}

pub fn f835(v0: Adt86) -> Str {
    let m2929 = v0;
    match m2929 {
        Adt86::C0 => {
            Str::lit("working")
        }
        Adt86::C1 => {
            Str::lit("input_required")
        }
        Adt86::C2 => {
            Str::lit("completed")
        }
        Adt86::C3 => {
            Str::lit("failed")
        }
        Adt86::C4 => {
            Str::lit("cancelled")
        }
    }
}

pub fn f836(v0: Adt65) -> Adt86 {
    let m2930 = v0;
    match m2930 {
        Adt65::C0(_) => {
            Adt86::C0
        }
        Adt65::C1(_, _) => {
            Adt86::C1
        }
        Adt65::C2(_, _) => {
            Adt86::C2
        }
        Adt65::C3(_, _) => {
            Adt86::C3
        }
        Adt65::C4(_) => {
            Adt86::C4
        }
    }
}

pub fn f837() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f838(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h2931, _, _, _, _, _) => {
            h2931
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h2932, _, _, _) => {
            h2932
        }
    })), {
        let a2941 = f657(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h2933, _, _, _, _) => {
                h2933
            }
        });
        let a2942 = {
            let a2939 = f657(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h2934, _, _) => {
                    h2934
                }
            });
            let a2940 = {
                let a2937 = f657(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h2935, _) => {
                        h2935
                    }
                });
                let a2938 = f658(match v0 {
                    Adt1::C0(_, _, _, _, _, h2936) => {
                        h2936
                    }
                });
                append_list(a2937, a2938)
            };
            append_list(a2939, a2940)
        };
        append_list(a2941, a2942)
    })))
}

pub fn f839(v0: List<Adt87>) -> Option<Adt6> {
    let m2943 = f904(v0, List::<Adt6>::nil());
    match m2943 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f436(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f840(v0: Str, p2948: List<Adt87>, v2: i64, p2949: List<Adt87>) -> R<List<Adt87>> {
    let m2947 = (p2948, p2949);
    {
        let mut m2947 = m2947;
        loop {
            let step = {
                let (v1, v3) = m2947;
                let m2944 = v1;
                match m2944.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<Adt87>), R<List<Adt87>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a2945 = f291(v4.clone());
                        let a2946 = v0.clone();
                        if if equal(a2945, a2946) {
                            true
                        } else {
                            f292(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt87>, List<Adt87>), R<List<Adt87>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt87>, List<Adt87>), R<List<Adt87>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2947 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f841(v0: List<List<u64>>) -> bool {
    let m2950 = v0;
    match m2950.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f842() -> List<Adt98> {
    List::cons(Adt98::C0(f905(), f686()), List::cons(Adt98::C0(f906(), f907()), List::cons(Adt98::C0(f908(), f909()), List::<Adt98>::nil())))
}

pub fn f843(v0: List<List<u64>>) -> List<u64> {
    f910(f107(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f844(v0: Adt22) -> Adt6 {
    let m2951 = v0;
    match m2951 {
        Adt22::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt22::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f845(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2952 = v0;
    match m2952 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f846(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m2953 = v0;
    match m2953 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f847() -> i64 {
    -32603i64
}

pub fn f848() -> i64 {
    429i64
}

pub fn f849() -> i64 {
    -32021i64
}

pub fn f850(p2956: List<Str>, p2957: List<Adt6>) -> List<Adt6> {
    let m2955 = (p2956, p2957);
    {
        let mut m2955 = m2955;
        loop {
            let step = {
                let (v0, v1) = m2955;
                let m2954 = v0;
                match m2954.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f851(v0: List<u64>) -> bool {
    let m2958 = v0;
    match m2958.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2959 = v1;
            let a2960 = 48u64;
            if nat_eq(a2959, a2960) {
                f911(v2)
            } else {
                false
            }
        }
    }
}

pub fn f852(v0: Adt0, v1: Fn9, v2: u64, v3: Adt17) -> R<Result<Adt144, Adt147>> {
    if f912(f913(v3.clone())) {
        Ok(Ok::<Adt144, Adt147>(f914(v0.clone(), Adt35::C0(f915(v0)))))
    } else {
        Ok(f916(v0.clone(), v3.clone(), f917(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2961, _) => {
                h2961
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2962, _, _) => {
                h2962
            }
        }, v2, v1, v3)?))
    }
}

pub fn f853(v0: Adt19, _: Adt5) -> Result<Adt17, Adt148> {
    Ok::<Adt17, Adt148>(Adt17::C0(Adt18::C0(v0)))
}

pub fn f854(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2963 = f918(Str::lit("cursor"), v1);
    match m2963 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C1(Adt23::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f855(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2964 = f919(Str::lit("name"), v1.clone());
    match m2964 {
        Ok(v2) => {
            let m2965 = f920(Str::lit("arguments"), v1.clone());
            match m2965 {
                Ok(v3) => {
                    let m2966 = f920(Str::lit("inputResponses"), v1.clone());
                    match m2966 {
                        Ok(v4) => {
                            let m2967 = f918(Str::lit("requestState"), v1);
                            match m2967 {
                                Ok(v5) => {
                                    Ok::<Adt17, Adt148>(Adt17::C2(Adt24::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt17, Adt148>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt17, Adt148>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt17, Adt148>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt17, Adt148>(v9)
        }
    }
}

pub fn f856(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2968 = f918(Str::lit("cursor"), v1);
    match m2968 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C3(Adt23::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f857(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2969 = f919(Str::lit("uri"), v1.clone());
    match m2969 {
        Ok(v2) => {
            let m2970 = f920(Str::lit("inputResponses"), v1.clone());
            match m2970 {
                Ok(v3) => {
                    let m2971 = f918(Str::lit("requestState"), v1);
                    match m2971 {
                        Ok(v4) => {
                            Ok::<Adt17, Adt148>(Adt17::C4(Adt25::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt17, Adt148>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt17, Adt148>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt17, Adt148>(v7)
        }
    }
}

pub fn f858(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2972 = f918(Str::lit("cursor"), v1);
    match m2972 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C5(Adt23::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f859(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2973 = f918(Str::lit("cursor"), v1);
    match m2973 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C6(Adt23::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f860(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2974 = f919(Str::lit("name"), v1.clone());
    match m2974 {
        Ok(v2) => {
            let m2975 = f921(Str::lit("arguments"), v1.clone());
            match m2975 {
                Ok(v3) => {
                    let m2976 = f920(Str::lit("inputResponses"), v1.clone());
                    match m2976 {
                        Ok(v4) => {
                            let m2977 = f918(Str::lit("requestState"), v1);
                            match m2977 {
                                Ok(v5) => {
                                    Ok::<Adt17, Adt148>(Adt17::C7(Adt26::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt17, Adt148>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt17, Adt148>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt17, Adt148>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt17, Adt148>(v9)
        }
    }
}

pub fn f861(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2978 = f922(Str::lit("ref"), v1.clone());
    match m2978 {
        Ok(v2) => {
            let m2979 = f923(v2);
            match m2979 {
                Ok(v3) => {
                    let m2980 = f922(Str::lit("argument"), v1.clone());
                    match m2980 {
                        Ok(v4) => {
                            let m2981 = f924(v4);
                            match m2981 {
                                Ok(v5) => {
                                    let m2982 = f925(v1);
                                    match m2982 {
                                        Ok(v6) => {
                                            Ok::<Adt17, Adt148>(Adt17::C8(Adt27::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt17, Adt148>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt17, Adt148>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt17, Adt148>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt17, Adt148>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt17, Adt148>(v11)
        }
    }
}

pub fn f862(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2983 = f919(Str::lit("taskId"), v1);
    match m2983 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C9(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f863(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2984 = f919(Str::lit("taskId"), v1.clone());
    match m2984 {
        Ok(v2) => {
            let m2985 = f922(Str::lit("inputResponses"), v1);
            match m2985 {
                Ok(v3) => {
                    Ok::<Adt17, Adt148>(Adt17::C10(Adt33::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt17, Adt148>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt17, Adt148>(v5)
        }
    }
}

pub fn f864(v0: Adt19, v1: Adt5) -> Result<Adt17, Adt148> {
    let m2986 = f919(Str::lit("taskId"), v1);
    match m2986 {
        Ok(v2) => {
            Ok::<Adt17, Adt148>(Adt17::C11(Adt34::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt17, Adt148>(v3)
        }
    }
}

pub fn f865(v0: Adt122, v1: Adt19, v2: Option<Adt147>) -> Adt159 {
    let m2987 = v2;
    match m2987 {
        None => {
            Adt159::C0(Adt160::C0(match v0.clone() {
                Adt122::C0(h2988, _, _) => {
                    h2988
                }
            }, match v0.clone() {
                Adt122::C0(_, h2989, _) => {
                    h2989
                }
            }, v1, f779(match v0 {
                Adt122::C0(_, _, h2990) => {
                    h2990
                }
            })))
        }
        Some(v3) => {
            Adt159::C1(v3)
        }
    }
}

pub fn f866(v0: Str, v1: List<Str>) -> Option<Adt147> {
    f926(f927(v0.clone(), v1.clone()), v0, v1)
}

pub fn f867(v0: Adt128, v1: Adt161, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt129>> {
    let m2992 = match v0.clone() {
        Adt128::C0(h2991, _, _, _, _) => {
            h2991
        }
    };
    match m2992 {
        None => {
            Ok(Some(Adt129::C0))
        }
        Some(v5) => {
            let a2993 = v5;
            let a2994 = v3;
            if equal(a2993, a2994) {
                let m2996 = match v0.clone() {
                    Adt128::C0(_, h2995, _, _, _) => {
                        h2995
                    }
                };
                match m2996 {
                    None => {
                        Ok(Some(Adt129::C1))
                    }
                    Some(v6) => {
                        let a2997 = v6;
                        let a2998 = v2;
                        if equal(a2997, a2998) {
                            f928(f929(match v0.clone() {
                                Adt128::C0(_, _, h2999, _, _) => {
                                    h2999
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt128::C0(_, _, _, _, h3000) => {
                                    h3000
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt129::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt129::C3))
            }
        }
    }
}

pub fn f868(v0: Adt5) -> Adt166 {
    f930(f229(f870(), v0.clone()), f229(f871(), v0))
}

pub fn f869(v0: Adt5, v1: Adt109, v2: Adt109) -> Result<Adt19, Adt148> {
    let m3001 = v1;
    match m3001 {
        Adt109::C0 => {
            Err::<Adt19, Adt148>(Adt148::C2)
        }
        Adt109::C1(v3) => {
            let m3002 = v2;
            match m3002 {
                Adt109::C0 => {
                    Err::<Adt19, Adt148>(Adt148::C3)
                }
                Adt109::C1(v4) => {
                    f931(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f870() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f871() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f872(p3006: List<Adt6>) -> bool {
    let m3005 = p3006;
    {
        let mut m3005 = m3005;
        loop {
            let step = {
                let v0 = m3005;
                let m3003 = v0;
                match m3003.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3004 = f131(v1);
                        match m3004 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<Adt6>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3005 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873(p3010: List<Adt6>) -> bool {
    let m3009 = p3010;
    {
        let mut m3009 = m3009;
        loop {
            let step = {
                let v0 = m3009;
                let m3007 = v0;
                match m3007.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3008 = f131(v1);
                        match m3008 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f711(v3) {
                                    Ok::<List<Adt6>, bool>(v2)
                                } else {
                                    Err::<List<Adt6>, bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f874(v0: Str) -> Option<Adt13> {
    let a3011 = v0.clone();
    let a3012 = Str::lit("string");
    if equal(a3011, a3012) {
        Some(Adt13::C0)
    } else {
        let a3013 = v0.clone();
        let a3014 = Str::lit("integer");
        if equal(a3013, a3014) {
            Some(Adt13::C1)
        } else {
            let a3015 = v0;
            let a3016 = Str::lit("boolean");
            if equal(a3015, a3016) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f875(p3025: List<u64>, p3026: List<u64>) -> R<List<u64>> {
    let m3024 = (p3025, p3026);
    {
        let mut m3024 = m3024;
        loop {
            let step = {
                let (v0, v1) = m3024;
                let m3017 = v0;
                match m3017.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a3018 = 65u64;
                        let a3019 = v2;
                        if if nat_le(a3018, a3019) {
                            let a3020 = v2;
                            let a3021 = 90u64;
                            nat_le(a3020, a3021)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a3022 = v2;
                                let a3023 = 32u64;
                                nat_add(a3022, a3023)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f876(p3033: List<u64>, p3034: List<u64>) -> bool {
    let m3032 = (p3033, p3034);
    {
        let mut m3032 = m3032;
        loop {
            let step = {
                let (v0, v1) = m3032;
                let m3027 = v0;
                match m3027.uncons() {
                    None => {
                        let m3028 = v1;
                        match m3028.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m3029 = v1;
                        match m3029.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a3030 = v4;
                                let a3031 = v6;
                                if nat_eq(a3030, a3031) {
                                    Ok::<(List<u64>, List<u64>), bool>((v5, v7))
                                } else {
                                    Err::<(List<u64>, List<u64>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f877(v0: u64, v1: u64) -> u64 {
    let a3035 = v0;
    let a3036 = v1;
    let a3037 = 0u64;
    nat_rem(a3035, a3036, a3037)
}

pub fn f878(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3040 = {
            let a3038 = v0;
            let a3039 = 4u64;
            nat_mul(a3038, a3039)?
        };
        let a3041 = f880(v1, 16u64);
        nat_add(a3040, a3041)?
    }, List::<u64>::nil()))
}

pub fn f879(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a3044 = {
            let a3042 = v0;
            let a3043 = 4u64;
            nat_mul(a3042, a3043)?
        };
        let a3045 = f880(v1, 16u64);
        nat_add(a3044, a3045)?
    }, List::cons({
        let a3048 = {
            let a3046 = f877(v1, 16u64);
            let a3047 = 16u64;
            nat_mul(a3046, a3047)?
        };
        let a3049 = f880(v2, 4u64);
        nat_add(a3048, a3049)?
    }, List::<u64>::nil())))
}

pub fn f880(v0: u64, v1: u64) -> u64 {
    let a3050 = v0;
    let a3051 = v1;
    let a3052 = 0u64;
    nat_quot(a3050, a3051, a3052)
}

pub fn f881(v0: Adt163, v1: Adt163) -> Adt163 {
    let m3053 = v0;
    match m3053 {
        Adt163::C0 => {
            f932(v1)
        }
        Adt163::C1 => {
            Adt163::C1
        }
        Adt163::C2 => {
            v1
        }
    }
}

pub fn f882(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt163> {
    let m3054 = f810(f933(v0, v1)?, f933(v2, v3)?);
    match m3054 {
        Adt163::C0 => {
            Ok(Adt163::C0)
        }
        Adt163::C1 => {
            f934(v0, f935(v0)?, v2, f935(v2)?)
        }
        Adt163::C2 => {
            Ok(Adt163::C2)
        }
    }
}

pub fn f883(v0: i64) -> R<i64> {
    let m3057 = {
        let a3055 = v0;
        let a3056 = 0i64;
        compare(a3055, a3056)
    };
    if match m3057 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        let a3058 = v0;
        int_neg(a3058)
    } else {
        Ok(v0)
    }
}

pub fn f884(p3070: u64, p3071: i64, p3072: i64) -> R<bool> {
    let m3069 = (p3070, (p3071, p3072));
    {
        let mut m3069 = m3069;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3069;
                let m3059 = v0;
                if m3059 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m3059 - 1;
                    let m3062 = {
                        let a3060 = v2;
                        let a3061 = 0i64;
                        compare(a3060, a3061)
                    };
                    if match m3062 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        if f338({
                            let a3063 = v1;
                            let a3064 = 10i64;
                            let a3065 = 0i64;
                            int_rem(a3063, a3064, a3065)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a3066 = v1;
                                let a3067 = 10i64;
                                let a3068 = 0i64;
                                int_quot(a3066, a3067, a3068)?
                            }, f437(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m3069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f885(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt167> {
    let a3073 = f936(v0.clone());
    let a3074 = f936(v1.clone());
    if nat_eq(a3073, a3074) {
        let m3075 = v0;
        match m3075 {
            Adt6::C0 => {
                Ok(Adt167::C1(v2))
            }
            Adt6::C1(v3) => {
                let m3076 = f438(v1);
                match m3076 {
                    None => {
                        Ok(Adt167::C0)
                    }
                    Some(v4) => {
                        let a3077 = v3;
                        let a3078 = v4;
                        if equal(a3077, a3078) {
                            Ok(Adt167::C1(v2))
                        } else {
                            Ok(Adt167::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m3079 = f716(v1);
                match m3079 {
                    None => {
                        Ok(Adt167::C0)
                    }
                    Some(v6) => {
                        if f937(f710(v5, v6)?) {
                            Ok(Adt167::C1(v2))
                        } else {
                            Ok(Adt167::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m3080 = f131(v1);
                match m3080 {
                    None => {
                        Ok(Adt167::C0)
                    }
                    Some(v8) => {
                        let a3081 = v7;
                        let a3082 = v8;
                        if equal(a3081, a3082) {
                            Ok(Adt167::C1(v2))
                        } else {
                            Ok(Adt167::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m3083 = f289(v1);
                match m3083 {
                    None => {
                        Ok(Adt167::C0)
                    }
                    Some(v10) => {
                        Ok(f938(f939(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m3084 = f234(v1);
                match m3084 {
                    None => {
                        Ok(Adt167::C0)
                    }
                    Some(v12) => {
                        let a3086 = f721(v11.clone(), 0u64)?;
                        let a3087 = f721(match v12.clone() {
                            Adt5::C0(h3085) => {
                                h3085
                            }
                        }, 0u64)?;
                        if nat_eq(a3086, a3087) {
                            Ok(f938(f940(v11, match v12 {
                                Adt5::C0(h3088) => {
                                    h3088
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt167::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt167::C0)
    }
}

pub fn f886(v0: u64) -> bool {
    let a3089 = v0;
    let a3090 = 9u64;
    if nat_eq(a3089, a3090) {
        true
    } else {
        let a3091 = v0;
        let a3092 = 10u64;
        if nat_eq(a3091, a3092) {
            true
        } else {
            let a3093 = v0;
            let a3094 = 13u64;
            nat_eq(a3093, a3094)
        }
    }
}

pub fn f887(p3097: List<Adt44>, p3098: List<Adt6>) -> List<Adt6> {
    let m3096 = (p3097, p3098);
    {
        let mut m3096 = m3096;
        loop {
            let step = {
                let (v0, v1) = m3096;
                let m3095 = v0;
                match m3095.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f941(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3096 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f888(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h3099, _, _, _) => {
            h3099
        }
    })), {
        let a3105 = f657(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h3100, _, _) => {
                h3100
            }
        });
        let a3106 = {
            let a3103 = f942(match v0.clone() {
                Adt2::C0(_, _, h3101, _) => {
                    h3101
                }
            });
            let a3104 = f943(match v0 {
                Adt2::C0(_, _, _, h3102) => {
                    h3102
                }
            });
            append_list(a3103, a3104)
        };
        append_list(a3105, a3106)
    }))
}

pub fn f889(v0: i64) -> u64 {
    let m3109 = {
        let a3107 = v0;
        let a3108 = 1i64;
        compare(a3107, a3108)
    };
    if match m3109 {
        Ordering::Less => {
            true
        }
        Ordering::Equal => {
            false
        }
        Ordering::Greater => {
            false
        }
    } {
        48u64
    } else {
        let m3112 = {
            let a3110 = v0;
            let a3111 = 2i64;
            compare(a3110, a3111)
        };
        if match m3112 {
            Ordering::Less => {
                true
            }
            Ordering::Equal => {
                false
            }
            Ordering::Greater => {
                false
            }
        } {
            49u64
        } else {
            let m3115 = {
                let a3113 = v0;
                let a3114 = 3i64;
                compare(a3113, a3114)
            };
            if match m3115 {
                Ordering::Less => {
                    true
                }
                Ordering::Equal => {
                    false
                }
                Ordering::Greater => {
                    false
                }
            } {
                50u64
            } else {
                let m3118 = {
                    let a3116 = v0;
                    let a3117 = 4i64;
                    compare(a3116, a3117)
                };
                if match m3118 {
                    Ordering::Less => {
                        true
                    }
                    Ordering::Equal => {
                        false
                    }
                    Ordering::Greater => {
                        false
                    }
                } {
                    51u64
                } else {
                    let m3121 = {
                        let a3119 = v0;
                        let a3120 = 5i64;
                        compare(a3119, a3120)
                    };
                    if match m3121 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        52u64
                    } else {
                        let m3124 = {
                            let a3122 = v0;
                            let a3123 = 6i64;
                            compare(a3122, a3123)
                        };
                        if match m3124 {
                            Ordering::Less => {
                                true
                            }
                            Ordering::Equal => {
                                false
                            }
                            Ordering::Greater => {
                                false
                            }
                        } {
                            53u64
                        } else {
                            let m3127 = {
                                let a3125 = v0;
                                let a3126 = 7i64;
                                compare(a3125, a3126)
                            };
                            if match m3127 {
                                Ordering::Less => {
                                    true
                                }
                                Ordering::Equal => {
                                    false
                                }
                                Ordering::Greater => {
                                    false
                                }
                            } {
                                54u64
                            } else {
                                let m3130 = {
                                    let a3128 = v0;
                                    let a3129 = 8i64;
                                    compare(a3128, a3129)
                                };
                                if match m3130 {
                                    Ordering::Less => {
                                        true
                                    }
                                    Ordering::Equal => {
                                        false
                                    }
                                    Ordering::Greater => {
                                        false
                                    }
                                } {
                                    55u64
                                } else {
                                    let m3133 = {
                                        let a3131 = v0;
                                        let a3132 = 9i64;
                                        compare(a3131, a3132)
                                    };
                                    if match m3133 {
                                        Ordering::Less => {
                                            true
                                        }
                                        Ordering::Equal => {
                                            false
                                        }
                                        Ordering::Greater => {
                                            false
                                        }
                                    } {
                                        56u64
                                    } else {
                                        57u64
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f890(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m3134 = v1;
    match m3134 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f944(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f891(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m3135 = v0;
    match m3135 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a3138 = f342(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h3136, _) => {
                        h3136
                    }
                });
                let a3139 = f342(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h3137) => {
                        h3137
                    }
                });
                append_list(a3138, a3139)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f892(v0: Adt11) -> Str {
    let m3140 = v0;
    match m3140 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f893(p3143: List<Adt38>, p3144: List<Adt6>) -> List<Adt6> {
    let m3142 = (p3143, p3144);
    {
        let mut m3142 = m3142;
        loop {
            let step = {
                let (v0, v1) = m3142;
                let m3141 = v0;
                match m3141.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt38>, List<Adt6>), List<Adt6>>((v3, List::cons(f945(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f894(p3147: List<Adt47>, p3148: List<Adt6>) -> R<List<Adt6>> {
    let m3146 = (p3147, p3148);
    {
        let mut m3146 = m3146;
        loop {
            let step = {
                let (v0, v1) = m3146;
                let m3145 = v0;
                match m3145.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f946(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f895(p3151: List<Adt49>, p3152: List<Adt6>) -> List<Adt6> {
    let m3150 = (p3151, p3152);
    {
        let mut m3150 = m3150;
        loop {
            let step = {
                let (v0, v1) = m3150;
                let m3149 = v0;
                match m3149.uncons() {
                    None => {
                        Err::<(List<Adt49>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt49>, List<Adt6>), List<Adt6>>((v3, List::cons(f660(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f896(p3155: List<Adt57>, p3156: List<Adt6>) -> List<Adt6> {
    let m3154 = (p3155, p3156);
    {
        let mut m3154 = m3154;
        loop {
            let step = {
                let (v0, v1) = m3154;
                let m3153 = v0;
                match m3153.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f947(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f897(p3159: List<Adt59>, p3160: List<Adt6>) -> List<Adt6> {
    let m3158 = (p3159, p3160);
    {
        let mut m3158 = m3158;
        loop {
            let step = {
                let (v0, v1) = m3158;
                let m3157 = v0;
                match m3157.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt59>, List<Adt6>), List<Adt6>>((v3, List::cons(f948(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f898(p3164: List<Adt62>, p3165: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3163 = (p3164, p3165);
    {
        let mut m3163 = m3163;
        loop {
            let step = {
                let (v0, v1) = m3163;
                let m3161 = v0;
                match m3161.uncons() {
                    None => {
                        Err::<(List<Adt62>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3162 = f949(v2)?;
                        match m3162 {
                            None => {
                                Err::<(List<Adt62>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt62>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f899(v0: Adt67) -> R<Option<Adt6>> {
    let m3166 = v0;
    match m3166 {
        Adt67::C0(v1) => {
            Ok(Some(f950(Str::lit("elicitation/create"), Some(f951(v1)))))
        }
        Adt67::C1(v2) => {
            let m3167 = f952(v2)?;
            match m3167 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f950(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt67::C2(v4) => {
            Ok(Some(f950(Str::lit("roots/list"), {
                let m3168 = v4;
                match m3168 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h3169) => {
                                h3169
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f900(v0: List<(Str, Adt67)>) -> Option<Adt6> {
    let m3170 = f953(v0, List::<(Str, Adt6)>::nil());
    match m3170 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f506(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f901(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m3171 = v1;
    match m3171 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f902(v0: Option<i64>) -> Adt6 {
    let m3172 = v0;
    match m3172 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f954(v1)
        }
    }
}

pub fn f903(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m3173 = v1;
    match m3173 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f954(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f904(p3177: List<Adt87>, p3178: List<Adt6>) -> Option<List<Adt6>> {
    let m3176 = (p3177, p3178);
    {
        let mut m3176 = m3176;
        loop {
            let step = {
                let (v0, v1) = m3176;
                let m3174 = v0;
                match m3174.uncons() {
                    None => {
                        Err::<(List<Adt87>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3175 = f955(v2);
                        match m3175 {
                            None => {
                                Err::<(List<Adt87>, List<Adt6>), Option<List<Adt6>>>(None::<List<Adt6>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt87>, List<Adt6>), Option<List<Adt6>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f905() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f906() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f907() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f908() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f909() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f910(p3183: List<List<u64>>, p3184: List<u64>) -> List<u64> {
    let m3182 = (p3183, p3184);
    {
        let mut m3182 = m3182;
        loop {
            let step = {
                let (v0, v1) = m3182;
                let m3179 = v0;
                match m3179.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a3180 = f956(v2);
                            let a3181 = v1;
                            append_list(a3180, a3181)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m3182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f911(p3191: List<u64>) -> bool {
    let m3190 = p3191;
    {
        let mut m3190 = m3190;
        loop {
            let step = {
                let v0 = m3190;
                let m3185 = v0;
                match m3185.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a3186 = v1;
                        let a3187 = 48u64;
                        if if nat_eq(a3186, a3187) {
                            true
                        } else {
                            let a3188 = v1;
                            let a3189 = 46u64;
                            nat_eq(a3188, a3189)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f912(v0: Adt154) -> bool {
    let m3192 = v0;
    match m3192 {
        Adt154::C0 => {
            true
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            false
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            false
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            false
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f913(v0: Adt17) -> Adt154 {
    let m3193 = v0;
    match m3193 {
        Adt17::C0(_) => {
            Adt154::C0
        }
        Adt17::C1(_) => {
            Adt154::C1
        }
        Adt17::C2(_) => {
            Adt154::C2
        }
        Adt17::C3(_) => {
            Adt154::C3
        }
        Adt17::C4(_) => {
            Adt154::C4
        }
        Adt17::C5(_) => {
            Adt154::C5
        }
        Adt17::C6(_) => {
            Adt154::C6
        }
        Adt17::C7(_) => {
            Adt154::C7
        }
        Adt17::C8(_) => {
            Adt154::C8
        }
        Adt17::C9(_) => {
            Adt154::C9
        }
        Adt17::C10(_) => {
            Adt154::C10
        }
        Adt17::C11(_) => {
            Adt154::C11
        }
    }
}

pub fn f914(v0: Adt0, v1: Adt35) -> Adt144 {
    Adt144::C0(v1, Adt145::C0(Some(match v0 {
        Adt0::C0(h3194, _, _, _, _, _, _, _) => {
            h3194
        }
    }), None::<Adt5>), List::<Adt146>::nil())
}

pub fn f915(v0: Adt0) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt0::C0(_, _, _, h3195, _, _, _, _) => {
            h3195
        }
    }, f470(), match v0.clone() {
        Adt0::C0(_, _, h3196, _, _, _, _, _) => {
            h3196
        }
    }, match v0 {
        Adt0::C0(_, h3197, _, _, _, _, _, _) => {
            h3197
        }
    })
}

pub fn f916(v0: Adt0, v1: Adt17, v2: Result<Adt35, Adt92>) -> Result<Adt144, Adt147> {
    let m3198 = v2;
    match m3198 {
        Ok(v4) => {
            if f957(f913(v1.clone()), v4.clone()) {
                f958(v0, v1, v4)
            } else {
                Err::<Adt144, Adt147>(Adt147::C4(f959()))
            }
        }
        Err(v3) => {
            Err::<Adt144, Adt147>(Adt147::C4(v3))
        }
    }
}

pub fn f917(v0: Bytes, v1: u64, v2: u64, v3: Fn9, v4: Adt17) -> R<Result<Adt35, Adt92>> {
    f960(v0.clone(), v1, v2, v3, v4.clone(), f961(v0, v4)?)
}

pub fn f918(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt148> {
    let m3199 = f229(v0.clone(), v1);
    match m3199 {
        Adt109::C0 => {
            Ok::<Option<Str>, Adt148>(None::<Str>)
        }
        Adt109::C1(v2) => {
            let m3200 = f131(v2);
            match m3200 {
                None => {
                    Err::<Option<Str>, Adt148>(Adt148::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt148>(Some(v3))
                }
            }
        }
    }
}

pub fn f919(v0: Str, v1: Adt5) -> Result<Str, Adt148> {
    let m3201 = f229(v0.clone(), v1);
    match m3201 {
        Adt109::C0 => {
            Err::<Str, Adt148>(Adt148::C7(v0))
        }
        Adt109::C1(v2) => {
            let m3202 = f131(v2);
            match m3202 {
                None => {
                    Err::<Str, Adt148>(Adt148::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt148>(v3)
                }
            }
        }
    }
}

pub fn f920(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt148> {
    let m3203 = f229(v0.clone(), v1);
    match m3203 {
        Adt109::C0 => {
            Ok::<Option<Adt5>, Adt148>(None::<Adt5>)
        }
        Adt109::C1(v2) => {
            let m3204 = f234(v2);
            match m3204 {
                None => {
                    Err::<Option<Adt5>, Adt148>(Adt148::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt148>(Some(v3))
                }
            }
        }
    }
}

pub fn f921(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt148> {
    let m3205 = f229(v0.clone(), v1);
    match m3205 {
        Adt109::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt148>(None::<List<(Str, Str)>>)
        }
        Adt109::C1(v2) => {
            let m3206 = f234(v2);
            match m3206 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt148>(Adt148::C8(v0))
                }
                Some(v3) => {
                    let m3208 = f962(v0, match v3 {
                        Adt5::C0(h3207) => {
                            h3207
                        }
                    });
                    match m3208 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt148>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt148>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f922(v0: Str, v1: Adt5) -> Result<Adt5, Adt148> {
    let m3209 = f229(v0.clone(), v1);
    match m3209 {
        Adt109::C0 => {
            Err::<Adt5, Adt148>(Adt148::C7(v0))
        }
        Adt109::C1(v2) => {
            let m3210 = f234(v2);
            match m3210 {
                None => {
                    Err::<Adt5, Adt148>(Adt148::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt148>(v3)
                }
            }
        }
    }
}

pub fn f923(v0: Adt5) -> Result<Adt28, Adt148> {
    let m3211 = f919(Str::lit("type"), v0.clone());
    match m3211 {
        Ok(v1) => {
            let a3212 = v1.clone();
            let a3213 = Str::lit("ref/prompt");
            if equal(a3212, a3213) {
                let m3214 = f919(Str::lit("name"), v0.clone());
                match m3214 {
                    Ok(v2) => {
                        let m3215 = f918(Str::lit("title"), v0);
                        match m3215 {
                            Ok(v3) => {
                                Ok::<Adt28, Adt148>(Adt28::C0(Adt29::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt28, Adt148>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt28, Adt148>(v5)
                    }
                }
            } else {
                let a3216 = v1;
                let a3217 = Str::lit("ref/resource");
                if equal(a3216, a3217) {
                    let m3218 = f919(Str::lit("uri"), v0);
                    match m3218 {
                        Ok(v6) => {
                            Ok::<Adt28, Adt148>(Adt28::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt28, Adt148>(v7)
                        }
                    }
                } else {
                    Err::<Adt28, Adt148>(Adt148::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt28, Adt148>(v8)
        }
    }
}

pub fn f924(v0: Adt5) -> Result<Adt30, Adt148> {
    let m3219 = f919(Str::lit("name"), v0.clone());
    match m3219 {
        Ok(v1) => {
            let m3220 = f919(Str::lit("value"), v0);
            match m3220 {
                Ok(v2) => {
                    Ok::<Adt30, Adt148>(Adt30::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt30, Adt148>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt30, Adt148>(v4)
        }
    }
}

pub fn f925(v0: Adt5) -> Result<Option<Adt31>, Adt148> {
    let m3221 = f229(Str::lit("context"), v0);
    match m3221 {
        Adt109::C0 => {
            Ok::<Option<Adt31>, Adt148>(None::<Adt31>)
        }
        Adt109::C1(v1) => {
            let m3222 = f234(v1);
            match m3222 {
                None => {
                    Err::<Option<Adt31>, Adt148>(Adt148::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m3223 = f921(Str::lit("arguments"), v2);
                    match m3223 {
                        Ok(v3) => {
                            Ok::<Option<Adt31>, Adt148>(Some(Adt31::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt31>, Adt148>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f926(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt147> {
    if v0 {
        None::<Adt147>
    } else {
        Some(Adt147::C1(v1, v2))
    }
}

pub fn f927(v0: Str, p3228: List<Str>) -> bool {
    let m3227 = p3228;
    {
        let mut m3227 = m3227;
        loop {
            let step = {
                let v1 = m3227;
                let m3224 = v1;
                match m3224.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3225 = v2;
                        let a3226 = v0.clone();
                        if equal(a3225, a3226) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f928(v0: Option<Adt129>, v1: List<Adt131>, v2: Adt5) -> R<Option<Adt129>> {
    let m3229 = v0;
    match m3229 {
        None => {
            f963(v1, f964(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f929(v0: Option<Str>, v1: Adt161, v2: Adt5) -> Option<Adt129> {
    let m3230 = v1.clone();
    match m3230 {
        Adt161::C0 => {
            None::<Adt129>
        }
        Adt161::C1 => {
            f965(v0, f966(v1, v2))
        }
        Adt161::C2 => {
            f965(v0, f966(v1, v2))
        }
        Adt161::C3 => {
            f965(v0, f966(v1, v2))
        }
    }
}

pub fn f930(v0: Adt109, v1: Adt109) -> Adt166 {
    let m3231 = v0;
    match m3231 {
        Adt109::C0 => {
            Adt166::C1
        }
        Adt109::C1(_) => {
            let m3232 = v1;
            match m3232 {
                Adt109::C0 => {
                    Adt166::C2
                }
                Adt109::C1(_) => {
                    Adt166::C0
                }
            }
        }
    }
}

pub fn f931(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt19, Adt148> {
    let m3233 = f131(v1);
    match m3233 {
        None => {
            Err::<Adt19, Adt148>(Adt148::C5)
        }
        Some(v3) => {
            let m3234 = f967(v2);
            match m3234 {
                None => {
                    Err::<Adt19, Adt148>(Adt148::C6)
                }
                Some(v4) => {
                    let m3235 = f920(f968(), v0.clone());
                    match m3235 {
                        Ok(v6) => {
                            let m3236 = f969(v0.clone());
                            match m3236 {
                                Ok(v8) => {
                                    let m3237 = f970(v0.clone());
                                    match m3237 {
                                        Ok(v10) => {
                                            Ok::<Adt19, Adt148>(Adt19::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt19, Adt148>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt19, Adt148>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt19, Adt148>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f932(v0: Adt163) -> Adt163 {
    let m3238 = v0;
    match m3238 {
        Adt163::C0 => {
            Adt163::C2
        }
        Adt163::C1 => {
            Adt163::C1
        }
        Adt163::C2 => {
            Adt163::C0
        }
    }
}

pub fn f933(v0: i64, v1: i64) -> R<i64> {
    f437(f344(f935(v0)?, 0i64)?, v1)
}

pub fn f934(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt163> {
    let a3239 = v1;
    let a3240 = v3;
    if nat_lt(a3239, a3240) {
        Ok(f810(f971({
            let a3241 = v3;
            let a3242 = v1;
            nat_sub(a3241, a3242)
        }, v0)?, v2))
    } else {
        let a3243 = v3;
        let a3244 = v1;
        if nat_lt(a3243, a3244) {
            Ok(f810(v0, f971({
                let a3245 = v1;
                let a3246 = v3;
                nat_sub(a3245, a3246)
            }, v2)?))
        } else {
            Ok(f810(v0, v2))
        }
    }
}

pub fn f935(v0: i64) -> R<u64> {
    f972(20u64, v0, 0u64)
}

pub fn f936(v0: Adt6) -> u64 {
    let m3247 = v0;
    match m3247 {
        Adt6::C0 => {
            0u64
        }
        Adt6::C1(_) => {
            1u64
        }
        Adt6::C2(_) => {
            2u64
        }
        Adt6::C3(_) => {
            3u64
        }
        Adt6::C4(_) => {
            4u64
        }
        Adt6::C5(_) => {
            5u64
        }
    }
}

pub fn f937(v0: Adt163) -> bool {
    let m3248 = v0;
    match m3248 {
        Adt163::C0 => {
            false
        }
        Adt163::C1 => {
            true
        }
        Adt163::C2 => {
            false
        }
    }
}

pub fn f938(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt167 {
    let m3249 = v0;
    match m3249 {
        None => {
            Adt167::C0
        }
        Some(v2) => {
            Adt167::C1({
                let a3250 = v2;
                let a3251 = v1;
                append_list(a3250, a3251)
            })
        }
    }
}

pub fn f939(p3256: List<Adt6>, p3257: List<Adt6>, p3258: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3255 = (p3256, (p3257, p3258));
    {
        let mut m3255 = m3255;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3255;
                let m3252 = v0;
                match m3252.uncons() {
                    None => {
                        let m3253 = v1;
                        match m3253.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m3254 = v1;
                        match m3254.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Some((v7, v8)) => {
                                Ok::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>((v6, (v8, List::cons((v5, v7), v2))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3255 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f940(p3264: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p3265: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m3263 = (p3264, p3265);
    {
        let mut m3263 = m3263;
        loop {
            let step = {
                let (v0, v2) = m3263;
                let m3259 = v0;
                match m3259.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m3261 = f337({
                            let (h3260, _) = v3;
                            h3260
                        }, v1.clone());
                        match m3261 {
                            Adt109::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt109::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h3262) = v3;
                                    h3262
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f941(v0: Adt44) -> Str {
    let m3266 = v0;
    match m3266 {
        Adt44::C0 => {
            Str::lit("user")
        }
        Adt44::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f942(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3267 = v0;
    match m3267 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f769(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f943(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3268 = v0;
    match m3268 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f973(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f944(v0: Adt8) -> Adt6 {
    Adt6::C5(f342(Str::lit("listChanged"), match v0 {
        Adt8::C0(h3269) => {
            h3269
        }
    }))
}

pub fn f945(v0: Adt38) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt38::C0(h3270, _, _, _, _, _, _, _) => {
            h3270
        }
    })), {
        let a3288 = f657(Str::lit("title"), match v0.clone() {
            Adt38::C0(_, h3271, _, _, _, _, _, _) => {
                h3271
            }
        });
        let a3289 = {
            let a3286 = f658(match v0.clone() {
                Adt38::C0(_, _, h3272, _, _, _, _, _) => {
                    h3272
                }
            });
            let a3287 = {
                let a3284 = f657(Str::lit("description"), match v0.clone() {
                    Adt38::C0(_, _, _, h3273, _, _, _, _) => {
                        h3273
                    }
                });
                let a3285 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m3276 = match v0.clone() {
                        Adt38::C0(_, _, _, _, h3275, _, _, _) => {
                            h3275
                        }
                    };
                    match m3276 {
                        Adt5::C0(h3274) => {
                            h3274
                        }
                    }
                })), {
                    let a3282 = f733(Str::lit("outputSchema"), match v0.clone() {
                        Adt38::C0(_, _, _, _, _, h3277, _, _) => {
                            h3277
                        }
                    });
                    let a3283 = {
                        let a3280 = f974(match v0.clone() {
                            Adt38::C0(_, _, _, _, _, _, h3278, _) => {
                                h3278
                            }
                        });
                        let a3281 = f656(match v0 {
                            Adt38::C0(_, _, _, _, _, _, _, h3279) => {
                                h3279
                            }
                        });
                        append_list(a3280, a3281)
                    };
                    append_list(a3282, a3283)
                });
                append_list(a3284, a3285)
            };
            append_list(a3286, a3287)
        };
        append_list(a3288, a3289)
    }))
}

pub fn f946(v0: Adt47) -> R<Adt6> {
    Ok(Adt6::C5({
        let a3313 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt47::C0(h3290, _, _, _, _, _, _, _, _) => {
                h3290
            }
        })), List::<(Str, Adt6)>::nil());
        let a3314 = {
            let a3311 = f657(Str::lit("title"), match v0.clone() {
                Adt47::C0(_, h3291, _, _, _, _, _, _, _) => {
                    h3291
                }
            });
            let a3312 = {
                let a3309 = f658(match v0.clone() {
                    Adt47::C0(_, _, h3292, _, _, _, _, _, _) => {
                        h3292
                    }
                });
                let a3310 = {
                    let a3307 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt47::C0(_, _, _, h3293, _, _, _, _, _) => {
                            h3293
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3308 = {
                        let a3305 = f657(Str::lit("description"), match v0.clone() {
                            Adt47::C0(_, _, _, _, h3294, _, _, _, _) => {
                                h3294
                            }
                        });
                        let a3306 = {
                            let a3303 = f657(Str::lit("mimeType"), match v0.clone() {
                                Adt47::C0(_, _, _, _, _, h3295, _, _, _) => {
                                    h3295
                                }
                            });
                            let a3304 = {
                                let a3301 = f655(match v0.clone() {
                                    Adt47::C0(_, _, _, _, _, _, h3296, _, _) => {
                                        h3296
                                    }
                                });
                                let a3302 = {
                                    let a3299 = f659(match v0.clone() {
                                        Adt47::C0(_, _, _, _, _, _, _, h3297, _) => {
                                            h3297
                                        }
                                    })?;
                                    let a3300 = f656(match v0 {
                                        Adt47::C0(_, _, _, _, _, _, _, _, h3298) => {
                                            h3298
                                        }
                                    });
                                    append_list(a3299, a3300)
                                };
                                append_list(a3301, a3302)
                            };
                            append_list(a3303, a3304)
                        };
                        append_list(a3305, a3306)
                    };
                    append_list(a3307, a3308)
                };
                append_list(a3309, a3310)
            };
            append_list(a3311, a3312)
        };
        append_list(a3313, a3314)
    }))
}

pub fn f947(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a3335 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h3315, _, _, _, _, _, _, _) => {
                h3315
            }
        })), List::<(Str, Adt6)>::nil());
        let a3336 = {
            let a3333 = f657(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h3316, _, _, _, _, _, _) => {
                    h3316
                }
            });
            let a3334 = {
                let a3331 = f658(match v0.clone() {
                    Adt57::C0(_, _, h3317, _, _, _, _, _) => {
                        h3317
                    }
                });
                let a3332 = {
                    let a3329 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt57::C0(_, _, _, h3318, _, _, _, _) => {
                            h3318
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a3330 = {
                        let a3327 = f657(Str::lit("description"), match v0.clone() {
                            Adt57::C0(_, _, _, _, h3319, _, _, _) => {
                                h3319
                            }
                        });
                        let a3328 = {
                            let a3325 = f657(Str::lit("mimeType"), match v0.clone() {
                                Adt57::C0(_, _, _, _, _, h3320, _, _) => {
                                    h3320
                                }
                            });
                            let a3326 = {
                                let a3323 = f655(match v0.clone() {
                                    Adt57::C0(_, _, _, _, _, _, h3321, _) => {
                                        h3321
                                    }
                                });
                                let a3324 = f656(match v0 {
                                    Adt57::C0(_, _, _, _, _, _, _, h3322) => {
                                        h3322
                                    }
                                });
                                append_list(a3323, a3324)
                            };
                            append_list(a3325, a3326)
                        };
                        append_list(a3327, a3328)
                    };
                    append_list(a3329, a3330)
                };
                append_list(a3331, a3332)
            };
            append_list(a3333, a3334)
        };
        append_list(a3335, a3336)
    })
}

pub fn f948(v0: Adt59) -> Adt6 {
    Adt6::C5({
        let a3352 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt59::C0(h3337, _, _, _, _, _) => {
                h3337
            }
        })), List::<(Str, Adt6)>::nil());
        let a3353 = {
            let a3350 = f657(Str::lit("title"), match v0.clone() {
                Adt59::C0(_, h3338, _, _, _, _) => {
                    h3338
                }
            });
            let a3351 = {
                let a3348 = f658(match v0.clone() {
                    Adt59::C0(_, _, h3339, _, _, _) => {
                        h3339
                    }
                });
                let a3349 = {
                    let a3346 = f657(Str::lit("description"), match v0.clone() {
                        Adt59::C0(_, _, _, h3340, _, _) => {
                            h3340
                        }
                    });
                    let a3347 = {
                        let a3344 = {
                            let m3342 = match v0.clone() {
                                Adt59::C0(_, _, _, _, h3341, _) => {
                                    h3341
                                }
                            };
                            match m3342 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f975(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a3345 = f656(match v0 {
                            Adt59::C0(_, _, _, _, _, h3343) => {
                                h3343
                            }
                        });
                        append_list(a3344, a3345)
                    };
                    append_list(a3346, a3347)
                };
                append_list(a3348, a3349)
            };
            append_list(a3350, a3351)
        };
        append_list(a3352, a3353)
    })
}

pub fn f949(v0: Adt62) -> R<Option<Adt6>> {
    let m3355 = f507(match v0.clone() {
        Adt62::C0(_, h3354) => {
            h3354
        }
    })?;
    match m3355 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f941(match v0 {
                Adt62::C0(h3356, _) => {
                    h3356
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f950(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f341(Str::lit("params"), v1)))
}

pub fn f951(v0: Adt68) -> Adt6 {
    let m3357 = v0;
    match m3357 {
        Adt68::C0(v1) => {
            f976(v1)
        }
        Adt68::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt71::C0(h3358, _) => {
                    h3358
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt71::C0(_, h3359) => {
                    h3359
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f952(v0: Adt72) -> R<Option<Adt6>> {
    let m3361 = f977(match v0.clone() {
        Adt72::C0(h3360, _, _, _, _, _, _, _, _, _) => {
            h3360
        }
    })?;
    match m3361 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a3363 = f978(v0.clone());
                let a3364 = List::cons((Str::lit("maxTokens"), f124(match v0.clone() {
                    Adt72::C0(_, _, _, _, _, h3362, _, _, _, _) => {
                        h3362
                    }
                })?), f979(v0));
                append_list(a3363, a3364)
            }))))
        }
    }
}

pub fn f953(p3370: List<(Str, Adt67)>, p3371: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m3369 = (p3370, p3371);
    {
        let mut m3369 = m3369;
        loop {
            let step = {
                let (v0, v1) = m3369;
                let m3365 = v0;
                match m3365.uncons() {
                    None => {
                        Err::<(List<(Str, Adt67)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m3367 = f980({
                            let (_, h3366) = v2;
                            h3366
                        });
                        match m3367 {
                            None => {
                                Err::<(List<(Str, Adt67)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt67)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h3368, _) = v2;
                                    h3368
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f954(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f955(v0: Adt87) -> Option<Adt6> {
    let m3373 = f749(match v0.clone() {
        Adt87::C0(h3372, _, _) => {
            h3372
        }
    });
    match m3373 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m3375 = f981(match v0.clone() {
                Adt87::C0(_, _, h3374) => {
                    h3374
                }
            });
            match m3375 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f954(match v0 {
                        Adt87::C0(_, h3376, _) => {
                            h3376
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f956(v0: List<u64>) -> List<u64> {
    let a3379 = f982();
    let a3380 = {
        let a3377 = v0;
        let a3378 = f983();
        append_list(a3377, a3378)
    };
    append_list(a3379, a3380)
}

pub fn f957(v0: Adt154, v1: Adt35) -> bool {
    let m3381 = v1;
    match m3381 {
        Adt35::C0(_) => {
            f984(v0, Adt154::C0)
        }
        Adt35::C1(_) => {
            f984(v0, Adt154::C1)
        }
        Adt35::C2(_) => {
            f984(v0, Adt154::C2)
        }
        Adt35::C3(_) => {
            f984(v0, Adt154::C2)
        }
        Adt35::C4(_) => {
            f984(v0, Adt154::C3)
        }
        Adt35::C5(_) => {
            f984(v0, Adt154::C4)
        }
        Adt35::C6(_) => {
            f984(v0, Adt154::C5)
        }
        Adt35::C7(_) => {
            f984(v0, Adt154::C6)
        }
        Adt35::C8(_) => {
            f984(v0, Adt154::C7)
        }
        Adt35::C9(_) => {
            f984(v0, Adt154::C8)
        }
        Adt35::C10(_) => {
            f984(v0, Adt154::C9)
        }
        Adt35::C11 => {
            if f984(v0.clone(), Adt154::C10) {
                true
            } else {
                f984(v0, Adt154::C11)
            }
        }
        Adt35::C12(_) => {
            f985(v0)
        }
        Adt35::C13(_) => {
            f986(v0)
        }
        Adt35::C14(v15) => {
            let m3383 = match v15 {
                Adt90::C0(_, h3382, _) => {
                    h3382
                }
            };
            match m3383 {
                Adt91::C0 => {
                    f984(v0, Adt154::C9)
                }
                Adt91::C1 => {
                    if f984(v0.clone(), Adt154::C10) {
                        true
                    } else {
                        f984(v0, Adt154::C11)
                    }
                }
            }
        }
    }
}

pub fn f958(v0: Adt0, v1: Adt17, v2: Adt35) -> Result<Adt144, Adt147> {
    let m3384 = v2.clone();
    match m3384 {
        Adt35::C0(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C1(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C2(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C3(v9) => {
            Ok::<Adt144, Adt147>(f991(v0, f988(v1), v9))
        }
        Adt35::C4(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C5(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C6(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C7(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C8(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C9(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C10(_) => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C11 => {
            Ok::<Adt144, Adt147>(f914(v0, v2))
        }
        Adt35::C12(v3) => {
            f987(v0, f988(v1), v3)
        }
        Adt35::C13(v4) => {
            f989(v0, f988(v1), v4)
        }
        Adt35::C14(v5) => {
            f990(v0, f988(v1), v5)
        }
    }
}

pub fn f959() -> Adt92 {
    Adt92::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f960(v0: Bytes, v1: u64, v2: u64, v3: Fn9, v4: Adt17, v5: Result<Adt17, Adt92>) -> R<Result<Adt35, Adt92>> {
    let m3385 = v5;
    match m3385 {
        Ok(v7) => {
            f992(v0, v4, f993(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt35, Adt92>(v6))
        }
    }
}

pub fn f961(v0: Bytes, v1: Adt17) -> R<Result<Adt17, Adt92>> {
    let m3386 = v1.clone();
    match m3386 {
        Adt17::C0(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C1(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C2(v2) => {
            f994(v0, v2)
        }
        Adt17::C3(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C4(v3) => {
            f995(v0, v3)
        }
        Adt17::C5(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C6(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C7(v4) => {
            f996(v0, v4)
        }
        Adt17::C8(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C9(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C10(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
        Adt17::C11(_) => {
            Ok(Ok::<Adt17, Adt92>(v1))
        }
    }
}

pub fn f962(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt148> {
    let m3387 = f997(v0, v1, List::<(Str, Str)>::nil());
    match m3387 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt148>(f998(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt148>(v3)
        }
    }
}

pub fn f963(p3391: List<Adt131>, v1: Adt5) -> R<Option<Adt129>> {
    let m3390 = p3391;
    {
        let mut m3390 = m3390;
        loop {
            let step = {
                let v0 = m3390;
                let m3388 = v0;
                match m3388.uncons() {
                    None => {
                        Err::<List<Adt131>, R<Option<Adt129>>>(Ok(None::<Adt129>))
                    }
                    Some((v2, v3)) => {
                        let m3389 = f999(v2, v1.clone())?;
                        match m3389 {
                            None => {
                                Ok::<List<Adt131>, R<Option<Adt129>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt131>, R<Option<Adt129>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f964(v0: Adt5) -> Adt5 {
    let m3392 = f229(Str::lit("arguments"), v0);
    match m3392 {
        Adt109::C0 => {
            f32()
        }
        Adt109::C1(v1) => {
            let m3393 = f234(v1);
            match m3393 {
                None => {
                    f32()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f965(v0: Option<Str>, v1: Option<Str>) -> Option<Adt129> {
    let m3394 = v0;
    match m3394 {
        None => {
            Some(Adt129::C2)
        }
        Some(v2) => {
            let m3395 = v1;
            match m3395 {
                None => {
                    None::<Adt129>
                }
                Some(v3) => {
                    let a3396 = v2;
                    let a3397 = v3;
                    if equal(a3396, a3397) {
                        None::<Adt129>
                    } else {
                        Some(Adt129::C5)
                    }
                }
            }
        }
    }
}

pub fn f966(v0: Adt161, v1: Adt5) -> Option<Str> {
    let m3398 = v0;
    match m3398 {
        Adt161::C0 => {
            None::<Str>
        }
        Adt161::C1 => {
            let m3399 = f229(Str::lit("name"), v1);
            match m3399 {
                Adt109::C0 => {
                    None::<Str>
                }
                Adt109::C1(v2) => {
                    f131(v2)
                }
            }
        }
        Adt161::C2 => {
            let m3400 = f229(Str::lit("uri"), v1);
            match m3400 {
                Adt109::C0 => {
                    None::<Str>
                }
                Adt109::C1(v3) => {
                    f131(v3)
                }
            }
        }
        Adt161::C3 => {
            let m3401 = f229(Str::lit("taskId"), v1);
            match m3401 {
                Adt109::C0 => {
                    None::<Str>
                }
                Adt109::C1(v4) => {
                    f131(v4)
                }
            }
        }
    }
}

pub fn f967(v0: Adt6) -> Option<Adt20> {
    let m3402 = f234(v0);
    match m3402 {
        None => {
            None::<Adt20>
        }
        Some(v1) => {
            let m3403 = f1000(Str::lit("experimental"), v1.clone());
            match m3403 {
                None => {
                    None::<Adt20>
                }
                Some(v2) => {
                    let m3404 = f1000(Str::lit("roots"), v1.clone());
                    match m3404 {
                        None => {
                            None::<Adt20>
                        }
                        Some(v3) => {
                            let m3405 = f1000(Str::lit("sampling"), v1.clone());
                            match m3405 {
                                None => {
                                    None::<Adt20>
                                }
                                Some(v4) => {
                                    let m3406 = f1000(Str::lit("elicitation"), v1.clone());
                                    match m3406 {
                                        None => {
                                            None::<Adt20>
                                        }
                                        Some(v5) => {
                                            let m3407 = f1000(Str::lit("extensions"), v1);
                                            match m3407 {
                                                None => {
                                                    None::<Adt20>
                                                }
                                                Some(v6) => {
                                                    Some(Adt20::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f968() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f969(v0: Adt5) -> Result<Option<Adt21>, Adt148> {
    let m3408 = f918(f1001(), v0);
    match m3408 {
        Ok(v2) => {
            let m3409 = v2;
            match m3409 {
                None => {
                    Ok::<Option<Adt21>, Adt148>(None::<Adt21>)
                }
                Some(v3) => {
                    let m3410 = f1002(v3);
                    match m3410 {
                        Adt168::C0 => {
                            Err::<Option<Adt21>, Adt148>(Adt148::C8(f1001()))
                        }
                        Adt168::C1(v4) => {
                            Ok::<Option<Adt21>, Adt148>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt21>, Adt148>(v1)
        }
    }
}

pub fn f970(v0: Adt5) -> Result<Option<Adt22>, Adt148> {
    let m3411 = f229(f1003(), v0);
    match m3411 {
        Adt109::C0 => {
            Ok::<Option<Adt22>, Adt148>(None::<Adt22>)
        }
        Adt109::C1(v1) => {
            let m3412 = f131(v1.clone());
            match m3412 {
                None => {
                    let m3413 = f230(v1);
                    match m3413 {
                        None => {
                            Err::<Option<Adt22>, Adt148>(Adt148::C8(f1003()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt22>, Adt148>(Some(Adt22::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt22>, Adt148>(Some(Adt22::C0(v2)))
                }
            }
        }
    }
}

pub fn f971(p3418: u64, p3419: i64) -> R<i64> {
    let m3417 = (p3418, p3419);
    {
        let mut m3417 = m3417;
        loop {
            let step = {
                let (v0, v1) = m3417;
                let m3414 = v0;
                if m3414 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3414 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3415 = v1;
                        let a3416 = 10i64;
                        int_mul(a3415, a3416)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f972(p3430: u64, p3431: i64, p3432: u64) -> R<u64> {
    let m3429 = (p3430, (p3431, p3432));
    {
        let mut m3429 = m3429;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3429;
                let m3420 = v0;
                if m3420 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3420 - 1;
                    let v4: u64 = {
                        let a3421 = v2;
                        let a3422 = 1u64;
                        nat_add(a3421, a3422)?
                    };
                    let m3425 = {
                        let a3423 = v1;
                        let a3424 = 10i64;
                        compare(a3423, a3424)
                    };
                    if match m3425 {
                        Ordering::Less => {
                            true
                        }
                        Ordering::Equal => {
                            false
                        }
                        Ordering::Greater => {
                            false
                        }
                    } {
                        Err::<(u64, (i64, u64)), R<u64>>(Ok(v4))
                    } else {
                        Ok::<(u64, (i64, u64)), R<u64>>((v3, ({
                            let a3426 = v1;
                            let a3427 = 10i64;
                            let a3428 = 0i64;
                            int_quot(a3426, a3427, a3428)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f973(v0: Adt3) -> Str {
    let m3433 = v0;
    match m3433 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f974(v0: Option<Adt39>) -> List<(Str, Adt6)> {
    let m3434 = v0;
    match m3434 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a3446 = f657(Str::lit("title"), match v1.clone() {
                    Adt39::C0(h3435, _, _, _, _) => {
                        h3435
                    }
                });
                let a3447 = {
                    let a3444 = f342(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt39::C0(_, h3436, _, _, _) => {
                            h3436
                        }
                    });
                    let a3445 = {
                        let a3442 = f342(Str::lit("destructiveHint"), match v1.clone() {
                            Adt39::C0(_, _, h3437, _, _) => {
                                h3437
                            }
                        });
                        let a3443 = {
                            let a3440 = f342(Str::lit("idempotentHint"), match v1.clone() {
                                Adt39::C0(_, _, _, h3438, _) => {
                                    h3438
                                }
                            });
                            let a3441 = f342(Str::lit("openWorldHint"), match v1 {
                                Adt39::C0(_, _, _, _, h3439) => {
                                    h3439
                                }
                            });
                            append_list(a3440, a3441)
                        };
                        append_list(a3442, a3443)
                    };
                    append_list(a3444, a3445)
                };
                append_list(a3446, a3447)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f975(v0: List<Adt60>) -> List<Adt6> {
    f436(f1004(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f976(v0: Adt69) -> Adt6 {
    Adt6::C5({
        let a3454 = {
            let m3449 = match v0.clone() {
                Adt69::C0(h3448, _, _) => {
                    h3448
                }
            };
            match m3449 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3455 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt69::C0(_, h3450, _) => {
                h3450
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m3453 = match v0 {
                Adt69::C0(_, _, h3452) => {
                    h3452
                }
            };
            match m3453 {
                Adt5::C0(h3451) => {
                    h3451
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a3454, a3455)
    })
}

pub fn f977(v0: List<Adt73>) -> R<Option<List<Adt6>>> {
    let m3456 = f1005(v0, List::<Adt6>::nil())?;
    match m3456 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f436(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f978(v0: Adt72) -> List<(Str, Adt6)> {
    let a3467 = {
        let m3458 = match v0.clone() {
            Adt72::C0(_, h3457, _, _, _, _, _, _, _, _) => {
                h3457
            }
        };
        match m3458 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f1006(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3468 = {
        let a3465 = f657(Str::lit("systemPrompt"), match v0.clone() {
            Adt72::C0(_, _, h3459, _, _, _, _, _, _, _) => {
                h3459
            }
        });
        let a3466 = {
            let a3463 = {
                let m3461 = match v0.clone() {
                    Adt72::C0(_, _, _, h3460, _, _, _, _, _, _) => {
                        h3460
                    }
                };
                match m3461 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f1007(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3464 = f1008(Str::lit("temperature"), match v0 {
                Adt72::C0(_, _, _, _, h3462, _, _, _, _, _) => {
                    h3462
                }
            });
            append_list(a3463, a3464)
        };
        append_list(a3465, a3466)
    };
    append_list(a3467, a3468)
}

pub fn f979(v0: Adt72) -> List<(Str, Adt6)> {
    let a3480 = {
        let m3470 = match v0.clone() {
            Adt72::C0(_, _, _, _, _, _, h3469, _, _, _) => {
                h3469
            }
        };
        match m3470 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f818(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a3481 = {
        let a3478 = f733(Str::lit("metadata"), match v0.clone() {
            Adt72::C0(_, _, _, _, _, _, _, h3471, _, _) => {
                h3471
            }
        });
        let a3479 = {
            let a3476 = {
                let m3473 = match v0.clone() {
                    Adt72::C0(_, _, _, _, _, _, _, _, h3472, _) => {
                        h3472
                    }
                };
                match m3473 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f821(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3477 = {
                let m3475 = match v0 {
                    Adt72::C0(_, _, _, _, _, _, _, _, _, h3474) => {
                        h3474
                    }
                };
                match m3475 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f1009(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a3476, a3477)
        };
        append_list(a3478, a3479)
    };
    append_list(a3480, a3481)
}

pub fn f980(v0: Adt67) -> Option<Adt6> {
    let m3482 = v0;
    match m3482 {
        Adt67::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f1010(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt67::C1(_) => {
            None::<Adt6>
        }
        Adt67::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m3483 = v3;
                match m3483 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h3484) => {
                                h3484
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f981(v0: Adt88) -> Option<Adt6> {
    let m3486 = f900(match v0.clone() {
        Adt88::C0(_, h3485, _, _) => {
            h3485
        }
    });
    match m3486 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f954(match v0.clone() {
                Adt88::C0(h3487, _, _, _) => {
                    h3487
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f954(match v0.clone() {
                Adt88::C0(_, _, h3488, _) => {
                    h3488
                }
            })), List::cons((Str::lit("outcome"), f1011(match v0 {
                Adt88::C0(_, _, _, h3489) => {
                    h3489
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f982() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f983() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f984(v0: Adt154, v1: Adt154) -> bool {
    let a3490 = f774(v0);
    let a3491 = f774(v1);
    equal(a3490, a3491)
}

pub fn f985(v0: Adt154) -> bool {
    let m3492 = v0;
    match m3492 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            true
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            true
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            true
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f986(v0: Adt154) -> bool {
    let m3493 = v0;
    match m3493 {
        Adt154::C0 => {
            false
        }
        Adt154::C1 => {
            false
        }
        Adt154::C2 => {
            true
        }
        Adt154::C3 => {
            false
        }
        Adt154::C4 => {
            false
        }
        Adt154::C5 => {
            false
        }
        Adt154::C6 => {
            false
        }
        Adt154::C7 => {
            false
        }
        Adt154::C8 => {
            false
        }
        Adt154::C9 => {
            false
        }
        Adt154::C10 => {
            false
        }
        Adt154::C11 => {
            false
        }
    }
}

pub fn f987(v0: Adt0, v1: Adt19, v2: Adt84) -> Result<Adt144, Adt147> {
    f1012(v0, v2.clone(), f1013(match v1 {
        Adt19::C0(_, h3494, _, _, _, _) => {
            h3494
        }
    }, f829(v2)))
}

pub fn f988(v0: Adt17) -> Adt19 {
    let m3495 = v0;
    match m3495 {
        Adt17::C0(v1) => {
            match v1 {
                Adt18::C0(h3496) => {
                    h3496
                }
            }
        }
        Adt17::C1(v2) => {
            match v2 {
                Adt23::C0(h3497, _) => {
                    h3497
                }
            }
        }
        Adt17::C2(v3) => {
            match v3 {
                Adt24::C0(h3498, _, _, _, _) => {
                    h3498
                }
            }
        }
        Adt17::C3(v4) => {
            match v4 {
                Adt23::C0(h3499, _) => {
                    h3499
                }
            }
        }
        Adt17::C4(v5) => {
            match v5 {
                Adt25::C0(h3500, _, _, _) => {
                    h3500
                }
            }
        }
        Adt17::C5(v6) => {
            match v6 {
                Adt23::C0(h3501, _) => {
                    h3501
                }
            }
        }
        Adt17::C6(v7) => {
            match v7 {
                Adt23::C0(h3502, _) => {
                    h3502
                }
            }
        }
        Adt17::C7(v8) => {
            match v8 {
                Adt26::C0(h3503, _, _, _, _) => {
                    h3503
                }
            }
        }
        Adt17::C8(v9) => {
            match v9 {
                Adt27::C0(h3504, _, _, _) => {
                    h3504
                }
            }
        }
        Adt17::C9(v10) => {
            match v10 {
                Adt32::C0(h3505, _) => {
                    h3505
                }
            }
        }
        Adt17::C10(v11) => {
            match v11 {
                Adt33::C0(h3506, _, _) => {
                    h3506
                }
            }
        }
        Adt17::C11(v12) => {
            match v12 {
                Adt34::C0(h3507, _) => {
                    h3507
                }
            }
        }
    }
}

pub fn f989(v0: Adt0, v1: Adt19, v2: Adt85) -> Result<Adt144, Adt147> {
    f1014(v0, v2, f186(match v1 {
        Adt19::C0(_, h3508, _, _, _, _) => {
            h3508
        }
    }))
}

pub fn f990(v0: Adt0, v1: Adt19, v2: Adt90) -> Result<Adt144, Adt147> {
    f1015(v0, v2, f186(match v1 {
        Adt19::C0(_, h3509, _, _, _, _) => {
            h3509
        }
    }))
}

pub fn f991(v0: Adt0, v1: Adt19, v2: Adt52) -> Adt144 {
    Adt144::C0(Adt35::C2(match v2.clone() {
        Adt52::C0(h3510, _) => {
            h3510
        }
    }), {
        let m3512 = f914(v0, Adt35::C11);
        match m3512 {
            Adt144::C0(_, h3511, _) => {
                h3511
            }
        }
    }, f1016(v1, match v2 {
        Adt52::C0(_, h3513) => {
            h3513
        }
    }))
}

pub fn f992(v0: Bytes, v1: Adt17, v2: Result<Adt35, Adt92>) -> R<Result<Adt35, Adt92>> {
    let m3514 = v2;
    match m3514 {
        Ok(v4) => {
            f1017(v0, f913(v1.clone()), f1018(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt35, Adt92>(v3))
        }
    }
}

pub fn f993(v0: u64, v1: u64, v2: Fn9, v3: Adt17) -> R<Result<Adt35, Adt92>> {
    let m3515 = v3.clone();
    match m3515 {
        Adt17::C0(_) => {
            let c3517: Fn9 = v2;
            c3517.apply(v3)
        }
        Adt17::C1(_) => {
            let c3518: Fn9 = v2;
            c3518.apply(v3)
        }
        Adt17::C2(_) => {
            if f1019(v0, v1) {
                let c3516: Fn9 = v2;
                c3516.apply(v3)
            } else {
                Ok(Err::<Adt35, Adt92>(Adt92::C7(v0)))
            }
        }
        Adt17::C3(_) => {
            let c3519: Fn9 = v2;
            c3519.apply(v3)
        }
        Adt17::C4(_) => {
            let c3520: Fn9 = v2;
            c3520.apply(v3)
        }
        Adt17::C5(_) => {
            let c3521: Fn9 = v2;
            c3521.apply(v3)
        }
        Adt17::C6(_) => {
            let c3522: Fn9 = v2;
            c3522.apply(v3)
        }
        Adt17::C7(_) => {
            let c3523: Fn9 = v2;
            c3523.apply(v3)
        }
        Adt17::C8(_) => {
            let c3524: Fn9 = v2;
            c3524.apply(v3)
        }
        Adt17::C9(_) => {
            let c3525: Fn9 = v2;
            c3525.apply(v3)
        }
        Adt17::C10(_) => {
            let c3526: Fn9 = v2;
            c3526.apply(v3)
        }
        Adt17::C11(_) => {
            let c3527: Fn9 = v2;
            c3527.apply(v3)
        }
    }
}

pub fn f994(v0: Bytes, v1: Adt24) -> R<Result<Adt17, Adt92>> {
    let m3531 = f1020(v0, Str::lit("tools/call"), match v1.clone() {
        Adt24::C0(_, h3528, _, _, _) => {
            h3528
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, _, h3529) => {
            h3529
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h3530, _) => {
            h3530
        }
    })?;
    match m3531 {
        Ok(v3) => {
            Ok(Ok::<Adt17, Adt92>(Adt17::C2(Adt24::C0(match v1.clone() {
                Adt24::C0(h3532, _, _, _, _) => {
                    h3532
                }
            }, match v1.clone() {
                Adt24::C0(_, h3533, _, _, _) => {
                    h3533
                }
            }, match v1.clone() {
                Adt24::C0(_, _, h3534, _, _) => {
                    h3534
                }
            }, match v1 {
                Adt24::C0(_, _, _, h3535, _) => {
                    h3535
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt17, Adt92>(v2))
        }
    }
}

pub fn f995(v0: Bytes, v1: Adt25) -> R<Result<Adt17, Adt92>> {
    let m3539 = f1020(v0, Str::lit("resources/read"), match v1.clone() {
        Adt25::C0(_, h3536, _, _) => {
            h3536
        }
    }, match v1.clone() {
        Adt25::C0(_, _, _, h3537) => {
            h3537
        }
    }, match v1.clone() {
        Adt25::C0(_, _, h3538, _) => {
            h3538
        }
    })?;
    match m3539 {
        Ok(v3) => {
            Ok(Ok::<Adt17, Adt92>(Adt17::C4(Adt25::C0(match v1.clone() {
                Adt25::C0(h3540, _, _, _) => {
                    h3540
                }
            }, match v1.clone() {
                Adt25::C0(_, h3541, _, _) => {
                    h3541
                }
            }, match v1 {
                Adt25::C0(_, _, h3542, _) => {
                    h3542
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt17, Adt92>(v2))
        }
    }
}

pub fn f996(v0: Bytes, v1: Adt26) -> R<Result<Adt17, Adt92>> {
    let m3546 = f1020(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt26::C0(_, h3543, _, _, _) => {
            h3543
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, _, h3544) => {
            h3544
        }
    }, match v1.clone() {
        Adt26::C0(_, _, _, h3545, _) => {
            h3545
        }
    })?;
    match m3546 {
        Ok(v3) => {
            Ok(Ok::<Adt17, Adt92>(Adt17::C7(Adt26::C0(match v1.clone() {
                Adt26::C0(h3547, _, _, _, _) => {
                    h3547
                }
            }, match v1.clone() {
                Adt26::C0(_, h3548, _, _, _) => {
                    h3548
                }
            }, match v1.clone() {
                Adt26::C0(_, _, h3549, _, _) => {
                    h3549
                }
            }, match v1 {
                Adt26::C0(_, _, _, h3550, _) => {
                    h3550
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt17, Adt92>(v2))
        }
    }
}

pub fn f997(v0: Str, p3556: List<(Str, Adt6)>, p3557: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt148> {
    let m3555 = (p3556, p3557);
    {
        let mut m3555 = m3555;
        loop {
            let step = {
                let (v1, v2) = m3555;
                let m3551 = v1;
                match m3551.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt148>>(Ok::<List<(Str, Str)>, Adt148>(v2))
                    }
                    Some((v3, v4)) => {
                        let m3553 = f131({
                            let (_, h3552) = v3;
                            h3552
                        });
                        match m3553 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt148>>(Err::<List<(Str, Str)>, Adt148>(Adt148::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt148>>((v4, List::cons(({
                                    let (h3554, _) = v3;
                                    h3554
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f998(p3560: List<(Str, Str)>, p3561: List<(Str, Str)>) -> List<(Str, Str)> {
    let m3559 = (p3560, p3561);
    {
        let mut m3559 = m3559;
        loop {
            let step = {
                let (v0, v1) = m3559;
                let m3558 = v0;
                match m3558.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f999(v0: Adt131, v1: Adt5) -> R<Option<Adt129>> {
    let m3563 = f1021(match v0.clone() {
        Adt131::C0(_, h3562, _, _) => {
            h3562
        }
    }, v1);
    match m3563 {
        Adt109::C0 => {
            Ok(f1022(v0))
        }
        Adt109::C1(v2) => {
            if f1023(v2.clone()) {
                Ok(f1022(v0))
            } else {
                f1024(v0, v2)
            }
        }
    }
}

pub fn f1000(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m3564 = f229(v0, v1);
    match m3564 {
        Adt109::C0 => {
            Some(None::<Adt5>)
        }
        Adt109::C1(v2) => {
            let m3565 = f234(v2);
            match m3565 {
                None => {
                    None::<Option<Adt5>>
                }
                Some(v3) => {
                    Some(Some(v3))
                }
            }
        }
    }
}

pub fn f1001() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f1002(v0: Str) -> Adt168 {
    let a3566 = v0.clone();
    let a3567 = Str::lit("debug");
    if equal(a3566, a3567) {
        Adt168::C1(Adt21::C0)
    } else {
        let a3568 = v0.clone();
        let a3569 = Str::lit("info");
        if equal(a3568, a3569) {
            Adt168::C1(Adt21::C1)
        } else {
            let a3570 = v0.clone();
            let a3571 = Str::lit("notice");
            if equal(a3570, a3571) {
                Adt168::C1(Adt21::C2)
            } else {
                let a3572 = v0.clone();
                let a3573 = Str::lit("warning");
                if equal(a3572, a3573) {
                    Adt168::C1(Adt21::C3)
                } else {
                    let a3574 = v0.clone();
                    let a3575 = Str::lit("error");
                    if equal(a3574, a3575) {
                        Adt168::C1(Adt21::C4)
                    } else {
                        let a3576 = v0.clone();
                        let a3577 = Str::lit("critical");
                        if equal(a3576, a3577) {
                            Adt168::C1(Adt21::C5)
                        } else {
                            let a3578 = v0.clone();
                            let a3579 = Str::lit("alert");
                            if equal(a3578, a3579) {
                                Adt168::C1(Adt21::C6)
                            } else {
                                let a3580 = v0;
                                let a3581 = Str::lit("emergency");
                                if equal(a3580, a3581) {
                                    Adt168::C1(Adt21::C7)
                                } else {
                                    Adt168::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f1003() -> Str {
    Str::lit("progressToken")
}

pub fn f1004(p3584: List<Adt60>, p3585: List<Adt6>) -> List<Adt6> {
    let m3583 = (p3584, p3585);
    {
        let mut m3583 = m3583;
        loop {
            let step = {
                let (v0, v1) = m3583;
                let m3582 = v0;
                match m3582.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt60>, List<Adt6>), List<Adt6>>((v3, List::cons(f1025(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1005(p3589: List<Adt73>, p3590: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3588 = (p3589, p3590);
    {
        let mut m3588 = m3588;
        loop {
            let step = {
                let (v0, v1) = m3588;
                let m3586 = v0;
                match m3586.uncons() {
                    None => {
                        Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3587 = f1026(v2)?;
                        match m3587 {
                            None => {
                                Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3588 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1006(v0: Adt78) -> Adt6 {
    Adt6::C5({
        let a3600 = {
            let m3592 = match v0.clone() {
                Adt78::C0(h3591, _, _, _) => {
                    h3591
                }
            };
            match m3592 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f436(f1027(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a3601 = {
            let a3598 = f1008(Str::lit("costPriority"), match v0.clone() {
                Adt78::C0(_, h3593, _, _) => {
                    h3593
                }
            });
            let a3599 = {
                let a3596 = f1008(Str::lit("speedPriority"), match v0.clone() {
                    Adt78::C0(_, _, h3594, _) => {
                        h3594
                    }
                });
                let a3597 = f1008(Str::lit("intelligencePriority"), match v0 {
                    Adt78::C0(_, _, _, h3595) => {
                        h3595
                    }
                });
                append_list(a3596, a3597)
            };
            append_list(a3598, a3599)
        };
        append_list(a3600, a3601)
    })
}

pub fn f1007(v0: Adt80) -> Str {
    let m3602 = v0;
    match m3602 {
        Adt80::C0 => {
            Str::lit("none")
        }
        Adt80::C1 => {
            Str::lit("thisServer")
        }
        Adt80::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f1008(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m3603 = v1;
    match m3603 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f1009(v0: Adt81) -> List<(Str, Adt6)> {
    let m3605 = match v0 {
        Adt81::C0(h3604) => {
            h3604
        }
    };
    match m3605 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f1028(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f1010(v0: Adt68) -> List<(Str, Adt6)> {
    let m3606 = v0;
    match m3606 {
        Adt68::C0(v1) => {
            let a3613 = {
                let m3608 = match v1.clone() {
                    Adt69::C0(h3607, _, _) => {
                        h3607
                    }
                };
                match m3608 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a3614 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt69::C0(_, h3609, _) => {
                    h3609
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m3612 = match v1 {
                    Adt69::C0(_, _, h3611) => {
                        h3611
                    }
                };
                match m3612 {
                    Adt5::C0(h3610) => {
                        h3610
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a3613, a3614)
        }
        Adt68::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt71::C0(h3615, _) => {
                    h3615
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt71::C0(_, h3616) => {
                    h3616
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f1011(v0: Adt89) -> Adt6 {
    let m3617 = v0;
    match m3617 {
        Adt89::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h3618) => {
                    h3618
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt89::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f389(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f1012(v0: Adt0, v1: Adt84, v2: bool) -> Result<Adt144, Adt147> {
    if v2 {
        Ok::<Adt144, Adt147>(f914(v0, Adt35::C12(v1)))
    } else {
        Err::<Adt144, Adt147>(Adt147::C4(Adt92::C8(f1029(f829(v1)))))
    }
}

pub fn f1013(v0: Adt20, v1: List<(Str, Adt67)>) -> bool {
    let a3619 = f1030(v1.clone());
    if if if bool_not(a3619) {
        true
    } else {
        f352(match v0.clone() {
            Adt20::C0(_, _, _, h3620, _) => {
                h3620
            }
        })
    } {
        let a3621 = f1031(v1.clone());
        if bool_not(a3621) {
            true
        } else {
            f352(match v0.clone() {
                Adt20::C0(_, _, h3622, _, _) => {
                    h3622
                }
            })
        }
    } else {
        false
    } {
        let a3623 = f1032(v1);
        if bool_not(a3623) {
            true
        } else {
            f352(match v0 {
                Adt20::C0(_, h3624, _, _, _) => {
                    h3624
                }
            })
        }
    } else {
        false
    }
}

pub fn f1014(v0: Adt0, v1: Adt85, v2: bool) -> Result<Adt144, Adt147> {
    if v2 {
        Ok::<Adt144, Adt147>(f914(v0, Adt35::C13(v1)))
    } else {
        Err::<Adt144, Adt147>(Adt147::C4(Adt92::C8(f293())))
    }
}

pub fn f1015(v0: Adt0, v1: Adt90, v2: bool) -> Result<Adt144, Adt147> {
    if v2 {
        Ok::<Adt144, Adt147>(f914(v0, Adt35::C14(v1)))
    } else {
        Err::<Adt144, Adt147>(Adt147::C4(Adt92::C8(f293())))
    }
}

pub fn f1016(v0: Adt19, v1: List<Adt53>) -> List<Adt146> {
    let m3626 = match v0 {
        Adt19::C0(_, _, _, _, h3625, _) => {
            h3625
        }
    };
    match m3626 {
        None => {
            List::<Adt146>::nil()
        }
        Some(v2) => {
            f1033(f1034(v2, v1, List::<Adt146>::nil()), List::<Adt146>::nil())
        }
    }
}

pub fn f1017(v0: Bytes, v1: Adt154, v2: Str, v3: Adt35) -> R<Result<Adt35, Adt92>> {
    let m3627 = v3.clone();
    match m3627 {
        Adt35::C0(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C1(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C2(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C3(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C4(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C5(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C6(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C7(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C8(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C9(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C10(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C11 => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C12(v4) => {
            f1035(v0, v1, v2, v4)
        }
        Adt35::C13(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
        Adt35::C14(_) => {
            Ok(Ok::<Adt35, Adt92>(v3))
        }
    }
}

pub fn f1018(v0: Adt17) -> Str {
    let m3628 = v0;
    match m3628 {
        Adt17::C0(_) => {
            Str::lit("")
        }
        Adt17::C1(_) => {
            Str::lit("")
        }
        Adt17::C2(v1) => {
            match v1 {
                Adt24::C0(_, h3629, _, _, _) => {
                    h3629
                }
            }
        }
        Adt17::C3(_) => {
            Str::lit("")
        }
        Adt17::C4(v2) => {
            match v2 {
                Adt25::C0(_, h3630, _, _) => {
                    h3630
                }
            }
        }
        Adt17::C5(_) => {
            Str::lit("")
        }
        Adt17::C6(_) => {
            Str::lit("")
        }
        Adt17::C7(v3) => {
            match v3 {
                Adt26::C0(_, h3631, _, _, _) => {
                    h3631
                }
            }
        }
        Adt17::C8(_) => {
            Str::lit("")
        }
        Adt17::C9(_) => {
            Str::lit("")
        }
        Adt17::C10(_) => {
            Str::lit("")
        }
        Adt17::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f1019(v0: u64, v1: u64) -> bool {
    let a3632 = v1;
    let a3633 = v0;
    nat_lt(a3632, a3633)
}

pub fn f1020(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt92>> {
    Ok(f1036(f1037(v0, v1, v2, v3)?, f1038(v4)))
}

pub fn f1021(p3639: List<Str>, p3640: Adt5) -> Adt109 {
    let m3638 = (p3639, p3640);
    {
        let mut m3638 = m3638;
        loop {
            let step = {
                let (v0, v1) = m3638;
                let m3634 = v0;
                match m3634.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt109>(Adt109::C0)
                    }
                    Some((v2, v3)) => {
                        let m3635 = f229(v2, v1);
                        match m3635 {
                            Adt109::C0 => {
                                Err::<(List<Str>, Adt5), Adt109>(Adt109::C0)
                            }
                            Adt109::C1(v4) => {
                                let m3636 = v3.clone();
                                match m3636.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt109>(Adt109::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3637 = f234(v4);
                                        match m3637 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt109>(Adt109::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt109>((v3, v7))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1022(v0: Adt131) -> Option<Adt129> {
    let m3642 = match v0.clone() {
        Adt131::C0(_, _, _, h3641) => {
            h3641
        }
    };
    match m3642 {
        Adt132::C0 => {
            None::<Adt129>
        }
        Adt132::C1 => {
            Some(Adt129::C10(match v0 {
                Adt131::C0(h3643, _, _, _) => {
                    h3643
                }
            }))
        }
        Adt132::C2(_) => {
            None::<Adt129>
        }
    }
}

pub fn f1023(v0: Adt6) -> bool {
    let m3644 = v0;
    match m3644 {
        Adt6::C0 => {
            true
        }
        Adt6::C1(_) => {
            false
        }
        Adt6::C2(_) => {
            false
        }
        Adt6::C3(_) => {
            false
        }
        Adt6::C4(_) => {
            false
        }
        Adt6::C5(_) => {
            false
        }
    }
}

pub fn f1024(v0: Adt131, v1: Adt6) -> R<Option<Adt129>> {
    let m3646 = match v0.clone() {
        Adt131::C0(_, _, _, h3645) => {
            h3645
        }
    };
    match m3646 {
        Adt132::C0 => {
            Ok(Some(Adt129::C8(match v0 {
                Adt131::C0(h3647, _, _, _) => {
                    h3647
                }
            })))
        }
        Adt132::C1 => {
            Ok(Some(Adt129::C10(match v0 {
                Adt131::C0(h3648, _, _, _) => {
                    h3648
                }
            })))
        }
        Adt132::C2(v2) => {
            if f1039(match v0.clone() {
                Adt131::C0(_, _, h3649, _) => {
                    h3649
                }
            }, v2, v1)? {
                Ok(None::<Adt129>)
            } else {
                Ok(Some(Adt129::C9(match v0 {
                    Adt131::C0(h3650, _, _, _) => {
                        h3650
                    }
                })))
            }
        }
    }
}

pub fn f1025(v0: Adt60) -> Adt6 {
    Adt6::C5({
        let a3659 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt60::C0(h3651, _, _, _) => {
                h3651
            }
        })), List::<(Str, Adt6)>::nil());
        let a3660 = {
            let a3657 = f657(Str::lit("title"), match v0.clone() {
                Adt60::C0(_, h3652, _, _) => {
                    h3652
                }
            });
            let a3658 = {
                let a3655 = f657(Str::lit("description"), match v0.clone() {
                    Adt60::C0(_, _, h3653, _) => {
                        h3653
                    }
                });
                let a3656 = f342(Str::lit("required"), match v0 {
                    Adt60::C0(_, _, _, h3654) => {
                        h3654
                    }
                });
                append_list(a3655, a3656)
            };
            append_list(a3657, a3658)
        };
        append_list(a3659, a3660)
    })
}

pub fn f1026(v0: Adt73) -> R<Option<Adt6>> {
    let m3662 = f1040(match v0.clone() {
        Adt73::C0(_, h3661, _) => {
            h3661
        }
    })?;
    match m3662 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f941(match v0.clone() {
                Adt73::C0(h3663, _, _) => {
                    h3663
                }
            }))), List::cons((Str::lit("content"), v1), f656(match v0 {
                Adt73::C0(_, _, h3664) => {
                    h3664
                }
            }))))))
        }
    }
}

pub fn f1027(p3668: List<Adt79>, p3669: List<Adt6>) -> List<Adt6> {
    let m3667 = (p3668, p3669);
    {
        let mut m3667 = m3667;
        loop {
            let step = {
                let (v0, v1) = m3667;
                let m3665 = v0;
                match m3665.uncons() {
                    None => {
                        Err::<(List<Adt79>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt79>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f657(Str::lit("name"), match v2 {
                            Adt79::C0(h3666) => {
                                h3666
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3667 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1028(v0: Adt82) -> Str {
    let m3670 = v0;
    match m3670 {
        Adt82::C0 => {
            Str::lit("auto")
        }
        Adt82::C1 => {
            Str::lit("required")
        }
        Adt82::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f1029(v0: List<(Str, Adt67)>) -> Adt20 {
    Adt20::C0(None::<Adt5>, if f1032(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt5>
    }, if f1031(v0.clone()) {
        Some(f32())
    } else {
        None::<Adt5>
    }, if f1030(v0) {
        Some(f32())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f1030(p3675: List<(Str, Adt67)>) -> bool {
    let m3674 = p3675;
    {
        let mut m3674 = m3674;
        loop {
            let step = {
                let v0 = m3674;
                let m3671 = v0;
                match m3671.uncons() {
                    None => {
                        Err::<List<(Str, Adt67)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3673 = {
                            let (_, h3672) = v1;
                            h3672
                        };
                        match m3673 {
                            Adt67::C0(_) => {
                                Err::<List<(Str, Adt67)>, bool>(true)
                            }
                            Adt67::C1(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                            Adt67::C2(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3674 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1031(p3680: List<(Str, Adt67)>) -> bool {
    let m3679 = p3680;
    {
        let mut m3679 = m3679;
        loop {
            let step = {
                let v0 = m3679;
                let m3676 = v0;
                match m3676.uncons() {
                    None => {
                        Err::<List<(Str, Adt67)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3678 = {
                            let (_, h3677) = v1;
                            h3677
                        };
                        match m3678 {
                            Adt67::C0(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                            Adt67::C1(_) => {
                                Err::<List<(Str, Adt67)>, bool>(true)
                            }
                            Adt67::C2(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1032(p3685: List<(Str, Adt67)>) -> bool {
    let m3684 = p3685;
    {
        let mut m3684 = m3684;
        loop {
            let step = {
                let v0 = m3684;
                let m3681 = v0;
                match m3681.uncons() {
                    None => {
                        Err::<List<(Str, Adt67)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3683 = {
                            let (_, h3682) = v1;
                            h3682
                        };
                        match m3683 {
                            Adt67::C0(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                            Adt67::C1(_) => {
                                Ok::<List<(Str, Adt67)>, bool>(v2)
                            }
                            Adt67::C2(_) => {
                                Err::<List<(Str, Adt67)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1033(p3688: List<Adt146>, p3689: List<Adt146>) -> List<Adt146> {
    let m3687 = (p3688, p3689);
    {
        let mut m3687 = m3687;
        loop {
            let step = {
                let (v0, v1) = m3687;
                let m3686 = v0;
                match m3686.uncons() {
                    None => {
                        Err::<(List<Adt146>, List<Adt146>), List<Adt146>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt146>, List<Adt146>), List<Adt146>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1034(v0: Adt22, p3692: List<Adt53>, p3693: List<Adt146>) -> List<Adt146> {
    let m3691 = (p3692, p3693);
    {
        let mut m3691 = m3691;
        loop {
            let step = {
                let (v1, v2) = m3691;
                let m3690 = v1;
                match m3690.uncons() {
                    None => {
                        Err::<(List<Adt53>, List<Adt146>), List<Adt146>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt53>, List<Adt146>), List<Adt146>>((v4, List::cons(Adt146::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1035(v0: Bytes, v1: Adt154, v2: Str, v3: Adt84) -> R<Result<Adt35, Adt92>> {
    if f985(v1.clone()) {
        if f1041(v3.clone()) {
            let m3694 = f1042(v0, f774(v1), v2, v3)?;
            match m3694 {
                Ok(v5) => {
                    Ok(Ok::<Adt35, Adt92>(Adt35::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt35, Adt92>(v4))
                }
            }
        } else {
            Ok(Err::<Adt35, Adt92>(Adt92::C13))
        }
    } else {
        Ok(Err::<Adt35, Adt92>(Adt92::C12(f774(v1))))
    }
}

pub fn f1036(v0: Result<Option<Str>, Adt92>, v1: bool) -> Result<Option<Str>, Adt92> {
    let m3695 = v0;
    match m3695 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt92>(v3)
            } else {
                Err::<Option<Str>, Adt92>(Adt92::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt92>(v2)
        }
    }
}

pub fn f1037(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt92>> {
    let m3696 = v3;
    match m3696 {
        None => {
            Ok(Ok::<Option<Str>, Adt92>(None::<Str>))
        }
        Some(v4) => {
            Ok(f1043(f1044(v0, v1, v2, v4)?))
        }
    }
}

pub fn f1038(v0: Option<Adt5>) -> bool {
    let m3697 = v0;
    match m3697 {
        None => {
            true
        }
        Some(v1) => {
            f1045(match v1 {
                Adt5::C0(h3698) => {
                    h3698
                }
            })
        }
    }
}

pub fn f1039(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3699 = v0;
    match m3699 {
        Adt13::C0 => {
            let m3700 = f131(v2);
            match m3700 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3701 = v1;
                    let a3702 = v3;
                    Ok(equal(a3701, a3702))
                }
            }
        }
        Adt13::C1 => {
            f1046(v1, v2)
        }
        Adt13::C2 => {
            let m3703 = f438(v2);
            match m3703 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3704 = v1;
                        let a3705 = Str::lit("true");
                        Ok(equal(a3704, a3705))
                    } else {
                        let a3706 = v1;
                        let a3707 = Str::lit("false");
                        Ok(equal(a3706, a3707))
                    }
                }
            }
        }
    }
}

pub fn f1040(v0: Adt74) -> R<Option<Adt6>> {
    let m3708 = v0;
    match m3708 {
        Adt74::C0(v1) => {
            f1047(v1)
        }
        Adt74::C1(v2) => {
            let m3709 = f1048(v2, List::<Adt6>::nil())?;
            match m3709 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f436(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f1041(v0: Adt84) -> bool {
    let m3710 = v0;
    match m3710 {
        Adt84::C0(v1) => {
            let m3711 = v1;
            match m3711.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt84::C1(_) => {
            true
        }
        Adt84::C2(_, _) => {
            true
        }
    }
}

pub fn f1042(v0: Bytes, v1: Str, v2: Str, v3: Adt84) -> R<Result<Adt84, Adt92>> {
    let m3712 = v3.clone();
    match m3712 {
        Adt84::C0(_) => {
            Ok(Ok::<Adt84, Adt92>(v3))
        }
        Adt84::C1(v5) => {
            Ok(f1049(f1050(v0, v1, v2, v5)?, List::<(Str, Adt67)>::nil()))
        }
        Adt84::C2(v6, v7) => {
            Ok(f1049(f1050(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f1043(v0: Option<Str>) -> Result<Option<Str>, Adt92> {
    let m3713 = v0;
    match m3713 {
        None => {
            Err::<Option<Str>, Adt92>(Adt92::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt92>(Some(v1))
        }
    }
}

pub fn f1044(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3717 = f1051(v0, {
        let a3714 = v1;
        utf8_encode(a3714)
    }, {
        let a3715 = v2;
        utf8_encode(a3715)
    }, {
        let a3716 = v3;
        utf8_encode(a3716)
    })?;
    match m3717 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3718 = v4;
            Ok(utf8_decode(a3718))
        }
    }
}

pub fn f1045(p3723: List<(Str, Adt6)>) -> bool {
    let m3722 = p3723;
    {
        let mut m3722 = m3722;
        loop {
            let step = {
                let v0 = m3722;
                let m3719 = v0;
                match m3719.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3721 = f234({
                            let (_, h3720) = v1;
                            h3720
                        });
                        match m3721 {
                            None => {
                                Err::<List<(Str, Adt6)>, bool>(false)
                            }
                            Some(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3722 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1046(v0: Str, v1: Adt6) -> R<bool> {
    let m3724 = f716(v1);
    match m3724 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3725 = f544(v2)?;
            match m3725 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3727 = f1052(f10({
                        let a3726 = v0;
                        utf8_encode(a3726)
                    })?)?;
                    match m3727 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f338({
                                let a3728 = v3;
                                let a3729 = v4;
                                int_sub(a3728, a3729)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f1047(v0: Adt75) -> R<Option<Adt6>> {
    let m3730 = v0;
    match m3730 {
        Adt75::C0(v1) => {
            Ok(Some(f572(v1)))
        }
        Adt75::C1(v2) => {
            Ok(Some(f573(Str::lit("image"), match v2.clone() {
                Adt45::C0(h3731, _, _, _) => {
                    h3731
                }
            }, match v2.clone() {
                Adt45::C0(_, h3732, _, _) => {
                    h3732
                }
            }, match v2.clone() {
                Adt45::C0(_, _, h3733, _) => {
                    h3733
                }
            }, match v2 {
                Adt45::C0(_, _, _, h3734) => {
                    h3734
                }
            })))
        }
        Adt75::C2(v3) => {
            Ok(Some(f573(Str::lit("audio"), match v3.clone() {
                Adt46::C0(h3735, _, _, _) => {
                    h3735
                }
            }, match v3.clone() {
                Adt46::C0(_, h3736, _, _) => {
                    h3736
                }
            }, match v3.clone() {
                Adt46::C0(_, _, h3737, _) => {
                    h3737
                }
            }, match v3 {
                Adt46::C0(_, _, _, h3738) => {
                    h3738
                }
            })))
        }
        Adt75::C3(v4) => {
            Ok(Some(f1053(v4)))
        }
        Adt75::C4(v5) => {
            f1054(v5)
        }
    }
}

pub fn f1048(p3742: List<Adt75>, p3743: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3741 = (p3742, p3743);
    {
        let mut m3741 = m3741;
        loop {
            let step = {
                let (v0, v1) = m3741;
                let m3739 = v0;
                match m3739.uncons() {
                    None => {
                        Err::<(List<Adt75>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3740 = f1047(v2)?;
                        match m3740 {
                            None => {
                                Err::<(List<Adt75>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt75>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1049(v0: Option<Str>, v1: List<(Str, Adt67)>) -> Result<Adt84, Adt92> {
    let m3744 = v0;
    match m3744 {
        None => {
            Err::<Adt84, Adt92>(Adt92::C14)
        }
        Some(v2) => {
            let m3745 = v1.clone();
            match m3745.uncons() {
                None => {
                    Ok::<Adt84, Adt92>(Adt84::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt84, Adt92>(Adt84::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f1050(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3749 = f1055(v0, {
        let a3746 = v1;
        utf8_encode(a3746)
    }, {
        let a3747 = v2;
        utf8_encode(a3747)
    }, {
        let a3748 = v3;
        utf8_encode(a3748)
    })?;
    match m3749 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3750 = v4;
            Ok(utf8_decode(a3750))
        }
    }
}

pub fn f1051(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3752 = f1056();
    let a3753 = {
        let a3751 = v0.clone();
        length_bytes(a3751)
    };
    if nat_le(a3752, a3753) {
        let a3755 = f1057(v3.clone())?;
        let a3756 = {
            let a3754 = v3.clone();
            length_bytes(a3754)
        };
        if nat_eq(a3755, a3756) {
            Ok(None::<Bytes>)
        } else {
            f1058(v0, v1, v2, v3.clone(), f1057(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1052(v0: List<u64>) -> R<Option<i64>> {
    let m3757 = v0.clone();
    match m3757.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3758 = v1;
            let a3759 = 45u64;
            if nat_eq(a3758, a3759) {
                let m3760 = v2.clone();
                match m3760.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3761 = f1059(v2, 0i64)?;
                        match m3761 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3762 = v5;
                                    int_neg(a3762)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f1059(v0, 0i64)
            }
        }
    }
}

pub fn f1053(v0: Adt76) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt76::C0(h3763, _, _, _) => {
            h3763
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt76::C0(_, h3764, _, _) => {
            h3764
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3767 = match v0.clone() {
            Adt76::C0(_, _, h3766, _) => {
                h3766
            }
        };
        match m3767 {
            Adt5::C0(h3765) => {
                h3765
            }
        }
    })), f656(match v0 {
        Adt76::C0(_, _, _, h3768) => {
            h3768
        }
    }))))))
}

pub fn f1054(v0: Adt77) -> R<Option<Adt6>> {
    let m3770 = f340(match v0.clone() {
        Adt77::C0(_, h3769, _, _, _) => {
            h3769
        }
    })?;
    match m3770 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt77::C0(h3771, _, _, _, _) => {
                    h3771
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3777 = f341(Str::lit("structuredContent"), match v0.clone() {
                    Adt77::C0(_, _, h3772, _, _) => {
                        h3772
                    }
                });
                let a3778 = {
                    let a3775 = f342(Str::lit("isError"), match v0.clone() {
                        Adt77::C0(_, _, _, h3773, _) => {
                            h3773
                        }
                    });
                    let a3776 = f656(match v0 {
                        Adt77::C0(_, _, _, _, h3774) => {
                            h3774
                        }
                    });
                    append_list(a3775, a3776)
                };
                append_list(a3777, a3778)
            }))))))
        }
    }
}

pub fn f1055(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3780 = f1056();
    let a3781 = {
        let a3779 = v0.clone();
        length_bytes(a3779)
    };
    if nat_le(a3780, a3781) {
        Ok(Some({
            let a3784 = f1060(v3.clone())?;
            let a3785 = {
                let a3782 = Bytes::lit(&[0x2e]);
                let a3783 = f1060(f1061(v0, v1, v2, v3)?)?;
                append_bytes(a3782, a3783)
            };
            append_bytes(a3784, a3785)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1056() -> u64 {
    32u64
}

pub fn f1057(v0: Bytes) -> R<u64> {
    let (_, h3786) = f1062(Fn10::F1063(v0.clone()), {
        let a3787 = v0.clone();
        length_bytes(a3787)
    }, (0u64, {
        let a3788 = v0;
        length_bytes(a3788)
    }))?;
    Ok(h3786)
}

pub fn f1058(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3792 = {
        let a3789 = v3.clone();
        let a3790 = 0u64;
        let a3791 = v4;
        slice_bytes(a3789, a3790, a3791)
    };
    match m3792 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3803 = {
                let a3800 = v3.clone();
                let a3801 = {
                    let a3793 = v4;
                    let a3794 = 1u64;
                    nat_add(a3793, a3794)?
                };
                let a3802 = {
                    let a3798 = {
                        let a3795 = v3;
                        length_bytes(a3795)
                    };
                    let a3799 = {
                        let a3796 = v4;
                        let a3797 = 1u64;
                        nat_add(a3796, a3797)?
                    };
                    nat_sub(a3798, a3799)
                };
                slice_bytes(a3800, a3801, a3802)
            };
            match m3803 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f1064(v0, v1, v2, f1065(v5)?, f1065(v6)?)
                }
            }
        }
    }
}

pub fn f1059(p3809: List<u64>, p3810: i64) -> R<Option<i64>> {
    let m3808 = (p3809, p3810);
    {
        let mut m3808 = m3808;
        loop {
            let step = {
                let (v0, v1) = m3808;
                let m3804 = v0;
                match m3804.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3805 = f1066(v2);
                        match m3805 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f437({
                                    let a3806 = v1;
                                    let a3807 = 10i64;
                                    int_mul(a3806, a3807)?
                                }, f611(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3808 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1060(v0: Bytes) -> R<Bytes> {
    let (_, h3811) = f1067(Fn11::F1068(v0.clone()), {
        let a3812 = v0;
        length_bytes(a3812)
    }, (0u64, f217()))?;
    Ok(h3811)
}

pub fn f1061(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f1069(v0, {
        let a3813 = f1070(v1, v2);
        let a3814 = v3;
        append_bytes(a3813, a3814)
    })
}

pub fn f1062(v0: Fn10, p3818: u64, p3819: (u64, u64)) -> R<(u64, u64)> {
    let m3817 = (p3818, p3819);
    {
        let mut m3817 = m3817;
        loop {
            let step = {
                let (v1, v2) = m3817;
                let m3815 = v1;
                if m3815 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3815 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3816: Fn10 = v0.clone();
                        c3816.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1063(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3821 = {
            let (h3820, _) = v1;
            h3820
        };
        let a3822 = 1u64;
        nat_add(a3821, a3822)?
    }, {
        let a3825 = {
            let (_, h3823) = v1;
            h3823
        };
        let a3826 = {
            let a3824 = v0.clone();
            length_bytes(a3824)
        };
        if nat_eq(a3825, a3826) {
            let a3828 = f45(v0, {
                let (h3827, _) = v1;
                h3827
            })?;
            let a3829 = 46u64;
            if nat_eq(a3828, a3829) {
                let (h3830, _) = v1;
                h3830
            } else {
                let (_, h3831) = v1;
                h3831
            }
        } else {
            let (_, h3832) = v1;
            h3832
        }
    }))
}

pub fn f1064(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3833 = v3;
    match m3833 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3834 = v4;
            match m3834 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f1071(f1061(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f1065(v0: Bytes) -> R<Option<Bytes>> {
    let a3839 = {
        let a3836 = {
            let a3835 = v0.clone();
            length_bytes(a3835)
        };
        let a3837 = 2u64;
        let a3838 = 0u64;
        nat_rem(a3836, a3837, a3838)
    };
    let a3840 = 0u64;
    if nat_eq(a3839, a3840) {
        let v1: (u64, (Bytes, u64)) = f1072(Fn12::F1073(v0.clone()), {
            let a3842 = {
                let a3841 = v0;
                length_bytes(a3841)
            };
            let a3843 = 2u64;
            let a3844 = 0u64;
            nat_quot(a3842, a3843, a3844)
        }, (0u64, (f217(), 0u64)))?;
        let a3847 = {
            let (_, h3845) = {
                let (_, h3846) = v1.clone();
                h3846
            };
            h3845
        };
        let a3848 = 0u64;
        if nat_eq(a3847, a3848) {
            Ok(Some({
                let (h3849, _) = {
                    let (_, h3850) = v1;
                    h3850
                };
                h3849
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f1066(v0: u64) -> Option<u64> {
    let a3851 = v0;
    let a3852 = 48u64;
    if if nat_lt(a3851, a3852) {
        true
    } else {
        let a3853 = 57u64;
        let a3854 = v0;
        nat_lt(a3853, a3854)
    } {
        None::<u64>
    } else {
        Some({
            let a3855 = v0;
            let a3856 = 48u64;
            nat_sub(a3855, a3856)
        })
    }
}

pub fn f1067(v0: Fn11, p3860: u64, p3861: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3859 = (p3860, p3861);
    {
        let mut m3859 = m3859;
        loop {
            let step = {
                let (v1, v2) = m3859;
                let m3857 = v1;
                if m3857 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3857 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3858: Fn11 = v0.clone();
                        c3858.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1068(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3863 = {
            let (h3862, _) = v1.clone();
            h3862
        };
        let a3864 = 1u64;
        nat_add(a3863, a3864)?
    }, {
        let a3867 = {
            let (_, h3865) = v1;
            h3865
        };
        let a3868 = f1074(f45(v0, {
            let (h3866, _) = v1;
            h3866
        })?)?;
        append_bytes(a3867, a3868)
    }))
}

pub fn f1069(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f1075(f1076(v0, v1)?)
}

pub fn f1070(v0: Bytes, v1: Bytes) -> Bytes {
    let a3875 = f1077({
        let a3869 = v0.clone();
        length_bytes(a3869)
    });
    let a3876 = {
        let a3873 = v0;
        let a3874 = {
            let a3871 = f1077({
                let a3870 = v1.clone();
                length_bytes(a3870)
            });
            let a3872 = v1;
            append_bytes(a3871, a3872)
        };
        append_bytes(a3873, a3874)
    };
    append_bytes(a3875, a3876)
}

pub fn f1071(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3879 = {
        let a3877 = v0.clone();
        length_bytes(a3877)
    };
    let a3880 = {
        let a3878 = v1.clone();
        length_bytes(a3878)
    };
    if nat_eq(a3879, a3880) {
        let a3881 = f1078(v0, v1)?;
        let a3882 = 0u64;
        Ok(nat_eq(a3881, a3882))
    } else {
        Ok(false)
    }
}

pub fn f1072(v0: Fn12, p3886: u64, p3887: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3885 = (p3886, p3887);
    {
        let mut m3885 = m3885;
        loop {
            let step = {
                let (v1, v2) = m3885;
                let m3883 = v1;
                if m3883 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3883 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3884: Fn12 = v0.clone();
                        c3884.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1073(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f1079(v0, v1)
}

pub fn f1074(v0: u64) -> R<Bytes> {
    let a3894 = f1080({
        let a3888 = v0;
        let a3889 = 16u64;
        let a3890 = 0u64;
        nat_quot(a3888, a3889, a3890)
    })?;
    let a3895 = f1080({
        let a3891 = v0;
        let a3892 = 16u64;
        let a3893 = 0u64;
        nat_rem(a3891, a3892, a3893)
    })?;
    Ok(append_bytes(a3894, a3895))
}

pub fn f1075(v0: Adt169) -> R<Bytes> {
    let a3916 = f1081(match v0.clone() {
        Adt169::C0(h3896, _, _, _, _, _, _, _) => {
            h3896
        }
    })?;
    let a3917 = {
        let a3914 = f1081(match v0.clone() {
            Adt169::C0(_, h3897, _, _, _, _, _, _) => {
                h3897
            }
        })?;
        let a3915 = {
            let a3912 = f1081(match v0.clone() {
                Adt169::C0(_, _, h3898, _, _, _, _, _) => {
                    h3898
                }
            })?;
            let a3913 = {
                let a3910 = f1081(match v0.clone() {
                    Adt169::C0(_, _, _, h3899, _, _, _, _) => {
                        h3899
                    }
                })?;
                let a3911 = {
                    let a3908 = f1081(match v0.clone() {
                        Adt169::C0(_, _, _, _, h3900, _, _, _) => {
                            h3900
                        }
                    })?;
                    let a3909 = {
                        let a3906 = f1081(match v0.clone() {
                            Adt169::C0(_, _, _, _, _, h3901, _, _) => {
                                h3901
                            }
                        })?;
                        let a3907 = {
                            let a3904 = f1081(match v0.clone() {
                                Adt169::C0(_, _, _, _, _, _, h3902, _) => {
                                    h3902
                                }
                            })?;
                            let a3905 = f1081(match v0 {
                                Adt169::C0(_, _, _, _, _, _, _, h3903) => {
                                    h3903
                                }
                            })?;
                            append_bytes(a3904, a3905)
                        };
                        append_bytes(a3906, a3907)
                    };
                    append_bytes(a3908, a3909)
                };
                append_bytes(a3910, a3911)
            };
            append_bytes(a3912, a3913)
        };
        append_bytes(a3914, a3915)
    };
    Ok(append_bytes(a3916, a3917))
}

pub fn f1076(v0: Bytes, v1: Bytes) -> R<Adt169> {
    let v2: Adt170 = f1082(v0)?;
    let v3: Adt169 = f1083(f1084(f1085(), f1086(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f1084(f1084(f1085(), f1086(v2, 1549556828u32))?, f1087(v3))
}

pub fn f1077(v0: u64) -> Bytes {
    let a3943 = f577({
        let a3921 = {
            let a3918 = v0;
            let a3919 = 16777216u64;
            let a3920 = 0u64;
            nat_quot(a3918, a3919, a3920)
        };
        let a3922 = 256u64;
        let a3923 = 0u64;
        nat_rem(a3921, a3922, a3923)
    });
    let a3944 = {
        let a3941 = f577({
            let a3927 = {
                let a3924 = v0;
                let a3925 = 65536u64;
                let a3926 = 0u64;
                nat_quot(a3924, a3925, a3926)
            };
            let a3928 = 256u64;
            let a3929 = 0u64;
            nat_rem(a3927, a3928, a3929)
        });
        let a3942 = {
            let a3939 = f577({
                let a3933 = {
                    let a3930 = v0;
                    let a3931 = 256u64;
                    let a3932 = 0u64;
                    nat_quot(a3930, a3931, a3932)
                };
                let a3934 = 256u64;
                let a3935 = 0u64;
                nat_rem(a3933, a3934, a3935)
            });
            let a3940 = f577({
                let a3936 = v0;
                let a3937 = 256u64;
                let a3938 = 0u64;
                nat_rem(a3936, a3937, a3938)
            });
            append_bytes(a3939, a3940)
        };
        append_bytes(a3941, a3942)
    };
    append_bytes(a3943, a3944)
}

pub fn f1078(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3945) = f1062(Fn10::F1088(v0.clone(), v1), {
        let a3946 = v0;
        length_bytes(a3946)
    }, (0u64, 0u64))?;
    Ok(h3945)
}

pub fn f1079(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f1089(f45(v0.clone(), {
        let (h3947, _) = v1.clone();
        h3947
    })?);
    let v3: u64 = f1089(f45(v0, {
        let a3949 = {
            let (h3948, _) = v1.clone();
            h3948
        };
        let a3950 = 1u64;
        nat_add(a3949, a3950)?
    })?);
    let a3951 = v2;
    let a3952 = 16u64;
    if if nat_lt(a3951, a3952) {
        let a3953 = v3;
        let a3954 = 16u64;
        nat_lt(a3953, a3954)
    } else {
        false
    } {
        Ok(({
            let a3956 = {
                let (h3955, _) = v1.clone();
                h3955
            };
            let a3957 = 2u64;
            nat_add(a3956, a3957)?
        }, ({
            let a3964 = {
                let (h3958, _) = {
                    let (_, h3959) = v1.clone();
                    h3959
                };
                h3958
            };
            let a3965 = f577({
                let a3962 = {
                    let a3960 = v2;
                    let a3961 = 16u64;
                    nat_mul(a3960, a3961)?
                };
                let a3963 = v3;
                nat_add(a3962, a3963)?
            });
            append_bytes(a3964, a3965)
        }, {
            let (_, h3966) = {
                let (_, h3967) = v1;
                h3967
            };
            h3966
        })))
    } else {
        Ok(({
            let a3969 = {
                let (h3968, _) = v1.clone();
                h3968
            };
            let a3970 = 2u64;
            nat_add(a3969, a3970)?
        }, ({
            let (h3971, _) = {
                let (_, h3972) = v1.clone();
                h3972
            };
            h3971
        }, {
            let a3975 = {
                let (_, h3973) = {
                    let (_, h3974) = v1;
                    h3974
                };
                h3973
            };
            let a3976 = 1u64;
            nat_add(a3975, a3976)?
        })))
    }
}

pub fn f1080(v0: u64) -> R<Bytes> {
    Ok(f577({
        let a3977 = v0;
        let a3978 = 10u64;
        if nat_lt(a3977, a3978) {
            let a3979 = 48u64;
            let a3980 = v0;
            nat_add(a3979, a3980)?
        } else {
            let a3981 = 87u64;
            let a3982 = v0;
            nat_add(a3981, a3982)?
        }
    }))
}

pub fn f1081(v0: u32) -> R<Bytes> {
    let a3987 = f1090(v0, 24u32)?;
    let a3988 = {
        let a3985 = f1090(v0, 16u32)?;
        let a3986 = {
            let a3983 = f1090(v0, 8u32)?;
            let a3984 = f1090(v0, 0u32)?;
            append_bytes(a3983, a3984)
        };
        append_bytes(a3985, a3986)
    };
    Ok(append_bytes(a3987, a3988))
}

pub fn f1082(v0: Bytes) -> R<Adt170> {
    let a3990 = f1091();
    let a3991 = {
        let a3989 = v0.clone();
        length_bytes(a3989)
    };
    if nat_lt(a3990, a3991) {
        Ok(f1092(f1093(v0)?))
    } else {
        f1094(v0)
    }
}

pub fn f1083(v0: Adt169, v1: u64, v2: Bytes) -> R<Adt169> {
    let v3: u64 = {
        let a3992 = v2.clone();
        length_bytes(a3992)
    };
    let v4: u64 = {
        let a3998 = {
            let a3995 = {
                let a3993 = v3;
                let a3994 = 72u64;
                nat_add(a3993, a3994)?
            };
            let a3996 = 64u64;
            let a3997 = 0u64;
            nat_quot(a3995, a3996, a3997)
        };
        let a3999 = 64u64;
        nat_mul(a3998, a3999)?
    };
    let (_, h4000) = f1095(Fn13::F1096(v2, v1, v3, v4), {
        let a4001 = v4;
        let a4002 = 64u64;
        let a4003 = 0u64;
        nat_quot(a4001, a4002, a4003)
    }, (0u64, v0))?;
    Ok(h4000)
}

pub fn f1084(v0: Adt169, v1: Adt170) -> R<Adt169> {
    let v2: Adt171 = f1097(Fn14::F1098, 64u64, Adt171::C0(match v0.clone() {
        Adt169::C0(h4004, _, _, _, _, _, _, _) => {
            h4004
        }
    }, match v0.clone() {
        Adt169::C0(_, h4005, _, _, _, _, _, _) => {
            h4005
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h4006, _, _, _, _, _) => {
            h4006
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h4007, _, _, _, _) => {
            h4007
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4008, _, _, _) => {
            h4008
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4009, _, _) => {
            h4009
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4010, _) => {
            h4010
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, _, h4011) => {
            h4011
        }
    }, v1, 0u64))?;
    Ok(Adt169::C0(f1099(match v0.clone() {
        Adt169::C0(h4012, _, _, _, _, _, _, _) => {
            h4012
        }
    }, match v2.clone() {
        Adt171::C0(h4013, _, _, _, _, _, _, _, _, _) => {
            h4013
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, h4014, _, _, _, _, _, _) => {
            h4014
        }
    }, match v2.clone() {
        Adt171::C0(_, h4015, _, _, _, _, _, _, _, _) => {
            h4015
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, _, h4016, _, _, _, _, _) => {
            h4016
        }
    }, match v2.clone() {
        Adt171::C0(_, _, h4017, _, _, _, _, _, _, _) => {
            h4017
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, _, _, h4018, _, _, _, _) => {
            h4018
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, h4019, _, _, _, _, _, _) => {
            h4019
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, _, _, _, h4020, _, _, _) => {
            h4020
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, h4021, _, _, _, _, _) => {
            h4021
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4022, _, _) => {
            h4022
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, h4023, _, _, _, _) => {
            h4023
        }
    }), f1099(match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4024, _) => {
            h4024
        }
    }, match v2.clone() {
        Adt171::C0(_, _, _, _, _, _, h4025, _, _, _) => {
            h4025
        }
    }), f1099(match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4026) => {
            h4026
        }
    }, match v2 {
        Adt171::C0(_, _, _, _, _, _, _, h4027, _, _) => {
            h4027
        }
    })))
}

pub fn f1085() -> Adt169 {
    Adt169::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f1086(v0: Adt170, v1: u32) -> Adt170 {
    Adt170::C0({
        let a4029 = match v0.clone() {
            Adt170::C0(h4028, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4028
            }
        };
        let a4030 = v1;
        fixed_u32::bit_xor(a4029, a4030)
    }, {
        let a4032 = match v0.clone() {
            Adt170::C0(_, h4031, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4031
            }
        };
        let a4033 = v1;
        fixed_u32::bit_xor(a4032, a4033)
    }, {
        let a4035 = match v0.clone() {
            Adt170::C0(_, _, h4034, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4034
            }
        };
        let a4036 = v1;
        fixed_u32::bit_xor(a4035, a4036)
    }, {
        let a4038 = match v0.clone() {
            Adt170::C0(_, _, _, h4037, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4037
            }
        };
        let a4039 = v1;
        fixed_u32::bit_xor(a4038, a4039)
    }, {
        let a4041 = match v0.clone() {
            Adt170::C0(_, _, _, _, h4040, _, _, _, _, _, _, _, _, _, _, _) => {
                h4040
            }
        };
        let a4042 = v1;
        fixed_u32::bit_xor(a4041, a4042)
    }, {
        let a4044 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, h4043, _, _, _, _, _, _, _, _, _, _) => {
                h4043
            }
        };
        let a4045 = v1;
        fixed_u32::bit_xor(a4044, a4045)
    }, {
        let a4047 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, h4046, _, _, _, _, _, _, _, _, _) => {
                h4046
            }
        };
        let a4048 = v1;
        fixed_u32::bit_xor(a4047, a4048)
    }, {
        let a4050 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, h4049, _, _, _, _, _, _, _, _) => {
                h4049
            }
        };
        let a4051 = v1;
        fixed_u32::bit_xor(a4050, a4051)
    }, {
        let a4053 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, h4052, _, _, _, _, _, _, _) => {
                h4052
            }
        };
        let a4054 = v1;
        fixed_u32::bit_xor(a4053, a4054)
    }, {
        let a4056 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, h4055, _, _, _, _, _, _) => {
                h4055
            }
        };
        let a4057 = v1;
        fixed_u32::bit_xor(a4056, a4057)
    }, {
        let a4059 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, h4058, _, _, _, _, _) => {
                h4058
            }
        };
        let a4060 = v1;
        fixed_u32::bit_xor(a4059, a4060)
    }, {
        let a4062 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h4061, _, _, _, _) => {
                h4061
            }
        };
        let a4063 = v1;
        fixed_u32::bit_xor(a4062, a4063)
    }, {
        let a4065 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4064, _, _, _) => {
                h4064
            }
        };
        let a4066 = v1;
        fixed_u32::bit_xor(a4065, a4066)
    }, {
        let a4068 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4067, _, _) => {
                h4067
            }
        };
        let a4069 = v1;
        fixed_u32::bit_xor(a4068, a4069)
    }, {
        let a4071 = match v0.clone() {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4070, _) => {
                h4070
            }
        };
        let a4072 = v1;
        fixed_u32::bit_xor(a4071, a4072)
    }, {
        let a4074 = match v0 {
            Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4073) => {
                h4073
            }
        };
        let a4075 = v1;
        fixed_u32::bit_xor(a4074, a4075)
    })
}

pub fn f1087(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h4076, _, _, _, _, _, _, _) => {
            h4076
        }
    }, match v0.clone() {
        Adt169::C0(_, h4077, _, _, _, _, _, _) => {
            h4077
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h4078, _, _, _, _, _) => {
            h4078
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h4079, _, _, _, _) => {
            h4079
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4080, _, _, _) => {
            h4080
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4081, _, _) => {
            h4081
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4082, _) => {
            h4082
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4083) => {
            h4083
        }
    }, 2147483648u32, f1100(), f1100(), f1100(), f1100(), f1100(), f1100(), 768u32)
}

pub fn f1088(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a4085 = {
            let (h4084, _) = v2;
            h4084
        };
        let a4086 = 1u64;
        nat_add(a4085, a4086)?
    }, {
        let a4138 = {
            let (_, h4087) = v2;
            h4087
        };
        let a4139 = {
            let v3: u8 = {
                let a4090 = f1101(v0, {
                    let (h4088, _) = v2;
                    h4088
                });
                let a4091 = f1101(v1, {
                    let (h4089, _) = v2;
                    h4089
                });
                fixed_u8::bit_xor(a4090, a4091)
            };
            let a4136 = {
                let a4112 = {
                    let a4100 = {
                        let a4094 = {
                            let a4092 = v3;
                            let a4093 = 128u8;
                            fixed_u8::bit_and(a4092, a4093)
                        };
                        let a4095 = 128u8;
                        if equal(a4094, a4095) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a4101 = {
                        let a4098 = {
                            let a4096 = v3;
                            let a4097 = 64u8;
                            fixed_u8::bit_and(a4096, a4097)
                        };
                        let a4099 = 64u8;
                        if equal(a4098, a4099) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4100, a4101)?
                };
                let a4113 = {
                    let a4110 = {
                        let a4104 = {
                            let a4102 = v3;
                            let a4103 = 32u8;
                            fixed_u8::bit_and(a4102, a4103)
                        };
                        let a4105 = 32u8;
                        if equal(a4104, a4105) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a4111 = {
                        let a4108 = {
                            let a4106 = v3;
                            let a4107 = 16u8;
                            fixed_u8::bit_and(a4106, a4107)
                        };
                        let a4109 = 16u8;
                        if equal(a4108, a4109) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4110, a4111)?
                };
                nat_add(a4112, a4113)?
            };
            let a4137 = {
                let a4134 = {
                    let a4122 = {
                        let a4116 = {
                            let a4114 = v3;
                            let a4115 = 8u8;
                            fixed_u8::bit_and(a4114, a4115)
                        };
                        let a4117 = 8u8;
                        if equal(a4116, a4117) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a4123 = {
                        let a4120 = {
                            let a4118 = v3;
                            let a4119 = 4u8;
                            fixed_u8::bit_and(a4118, a4119)
                        };
                        let a4121 = 4u8;
                        if equal(a4120, a4121) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4122, a4123)?
                };
                let a4135 = {
                    let a4132 = {
                        let a4126 = {
                            let a4124 = v3;
                            let a4125 = 2u8;
                            fixed_u8::bit_and(a4124, a4125)
                        };
                        let a4127 = 2u8;
                        if equal(a4126, a4127) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a4133 = {
                        let a4130 = {
                            let a4128 = v3;
                            let a4129 = 1u8;
                            fixed_u8::bit_and(a4128, a4129)
                        };
                        let a4131 = 1u8;
                        if equal(a4130, a4131) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a4132, a4133)?
                };
                nat_add(a4134, a4135)?
            };
            nat_add(a4136, a4137)?
        };
        nat_add(a4138, a4139)?
    }))
}

pub fn f1089(v0: u64) -> u64 {
    let a4140 = v0;
    let a4141 = 48u64;
    if nat_lt(a4140, a4141) {
        16u64
    } else {
        let a4142 = v0;
        let a4143 = 58u64;
        if nat_lt(a4142, a4143) {
            let a4144 = v0;
            let a4145 = 48u64;
            nat_sub(a4144, a4145)
        } else {
            let a4146 = v0;
            let a4147 = 97u64;
            if nat_lt(a4146, a4147) {
                16u64
            } else {
                let a4148 = v0;
                let a4149 = 103u64;
                if nat_lt(a4148, a4149) {
                    let a4150 = v0;
                    let a4151 = 87u64;
                    nat_sub(a4150, a4151)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f1090(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f577({
        let v2: u32 = {
            let a4152 = f1102(v0, v1);
            let a4153 = 255u32;
            fixed_u32::bit_and(a4152, a4153)
        };
        let a4342 = {
            let a4246 = {
                let a4198 = {
                    let a4174 = {
                        let a4162 = {
                            let a4156 = {
                                let a4154 = v2;
                                let a4155 = 2147483648u32;
                                fixed_u32::bit_and(a4154, a4155)
                            };
                            let a4157 = 2147483648u32;
                            if equal(a4156, a4157) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a4163 = {
                            let a4160 = {
                                let a4158 = v2;
                                let a4159 = 1073741824u32;
                                fixed_u32::bit_and(a4158, a4159)
                            };
                            let a4161 = 1073741824u32;
                            if equal(a4160, a4161) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4162, a4163)?
                    };
                    let a4175 = {
                        let a4172 = {
                            let a4166 = {
                                let a4164 = v2;
                                let a4165 = 536870912u32;
                                fixed_u32::bit_and(a4164, a4165)
                            };
                            let a4167 = 536870912u32;
                            if equal(a4166, a4167) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a4173 = {
                            let a4170 = {
                                let a4168 = v2;
                                let a4169 = 268435456u32;
                                fixed_u32::bit_and(a4168, a4169)
                            };
                            let a4171 = 268435456u32;
                            if equal(a4170, a4171) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4172, a4173)?
                    };
                    nat_add(a4174, a4175)?
                };
                let a4199 = {
                    let a4196 = {
                        let a4184 = {
                            let a4178 = {
                                let a4176 = v2;
                                let a4177 = 134217728u32;
                                fixed_u32::bit_and(a4176, a4177)
                            };
                            let a4179 = 134217728u32;
                            if equal(a4178, a4179) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a4185 = {
                            let a4182 = {
                                let a4180 = v2;
                                let a4181 = 67108864u32;
                                fixed_u32::bit_and(a4180, a4181)
                            };
                            let a4183 = 67108864u32;
                            if equal(a4182, a4183) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4184, a4185)?
                    };
                    let a4197 = {
                        let a4194 = {
                            let a4188 = {
                                let a4186 = v2;
                                let a4187 = 33554432u32;
                                fixed_u32::bit_and(a4186, a4187)
                            };
                            let a4189 = 33554432u32;
                            if equal(a4188, a4189) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a4195 = {
                            let a4192 = {
                                let a4190 = v2;
                                let a4191 = 16777216u32;
                                fixed_u32::bit_and(a4190, a4191)
                            };
                            let a4193 = 16777216u32;
                            if equal(a4192, a4193) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4194, a4195)?
                    };
                    nat_add(a4196, a4197)?
                };
                nat_add(a4198, a4199)?
            };
            let a4247 = {
                let a4244 = {
                    let a4220 = {
                        let a4208 = {
                            let a4202 = {
                                let a4200 = v2;
                                let a4201 = 8388608u32;
                                fixed_u32::bit_and(a4200, a4201)
                            };
                            let a4203 = 8388608u32;
                            if equal(a4202, a4203) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a4209 = {
                            let a4206 = {
                                let a4204 = v2;
                                let a4205 = 4194304u32;
                                fixed_u32::bit_and(a4204, a4205)
                            };
                            let a4207 = 4194304u32;
                            if equal(a4206, a4207) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4208, a4209)?
                    };
                    let a4221 = {
                        let a4218 = {
                            let a4212 = {
                                let a4210 = v2;
                                let a4211 = 2097152u32;
                                fixed_u32::bit_and(a4210, a4211)
                            };
                            let a4213 = 2097152u32;
                            if equal(a4212, a4213) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a4219 = {
                            let a4216 = {
                                let a4214 = v2;
                                let a4215 = 1048576u32;
                                fixed_u32::bit_and(a4214, a4215)
                            };
                            let a4217 = 1048576u32;
                            if equal(a4216, a4217) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4218, a4219)?
                    };
                    nat_add(a4220, a4221)?
                };
                let a4245 = {
                    let a4242 = {
                        let a4230 = {
                            let a4224 = {
                                let a4222 = v2;
                                let a4223 = 524288u32;
                                fixed_u32::bit_and(a4222, a4223)
                            };
                            let a4225 = 524288u32;
                            if equal(a4224, a4225) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a4231 = {
                            let a4228 = {
                                let a4226 = v2;
                                let a4227 = 262144u32;
                                fixed_u32::bit_and(a4226, a4227)
                            };
                            let a4229 = 262144u32;
                            if equal(a4228, a4229) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4230, a4231)?
                    };
                    let a4243 = {
                        let a4240 = {
                            let a4234 = {
                                let a4232 = v2;
                                let a4233 = 131072u32;
                                fixed_u32::bit_and(a4232, a4233)
                            };
                            let a4235 = 131072u32;
                            if equal(a4234, a4235) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a4241 = {
                            let a4238 = {
                                let a4236 = v2;
                                let a4237 = 65536u32;
                                fixed_u32::bit_and(a4236, a4237)
                            };
                            let a4239 = 65536u32;
                            if equal(a4238, a4239) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4240, a4241)?
                    };
                    nat_add(a4242, a4243)?
                };
                nat_add(a4244, a4245)?
            };
            nat_add(a4246, a4247)?
        };
        let a4343 = {
            let a4340 = {
                let a4292 = {
                    let a4268 = {
                        let a4256 = {
                            let a4250 = {
                                let a4248 = v2;
                                let a4249 = 32768u32;
                                fixed_u32::bit_and(a4248, a4249)
                            };
                            let a4251 = 32768u32;
                            if equal(a4250, a4251) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a4257 = {
                            let a4254 = {
                                let a4252 = v2;
                                let a4253 = 16384u32;
                                fixed_u32::bit_and(a4252, a4253)
                            };
                            let a4255 = 16384u32;
                            if equal(a4254, a4255) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4256, a4257)?
                    };
                    let a4269 = {
                        let a4266 = {
                            let a4260 = {
                                let a4258 = v2;
                                let a4259 = 8192u32;
                                fixed_u32::bit_and(a4258, a4259)
                            };
                            let a4261 = 8192u32;
                            if equal(a4260, a4261) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a4267 = {
                            let a4264 = {
                                let a4262 = v2;
                                let a4263 = 4096u32;
                                fixed_u32::bit_and(a4262, a4263)
                            };
                            let a4265 = 4096u32;
                            if equal(a4264, a4265) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4266, a4267)?
                    };
                    nat_add(a4268, a4269)?
                };
                let a4293 = {
                    let a4290 = {
                        let a4278 = {
                            let a4272 = {
                                let a4270 = v2;
                                let a4271 = 2048u32;
                                fixed_u32::bit_and(a4270, a4271)
                            };
                            let a4273 = 2048u32;
                            if equal(a4272, a4273) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a4279 = {
                            let a4276 = {
                                let a4274 = v2;
                                let a4275 = 1024u32;
                                fixed_u32::bit_and(a4274, a4275)
                            };
                            let a4277 = 1024u32;
                            if equal(a4276, a4277) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4278, a4279)?
                    };
                    let a4291 = {
                        let a4288 = {
                            let a4282 = {
                                let a4280 = v2;
                                let a4281 = 512u32;
                                fixed_u32::bit_and(a4280, a4281)
                            };
                            let a4283 = 512u32;
                            if equal(a4282, a4283) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a4289 = {
                            let a4286 = {
                                let a4284 = v2;
                                let a4285 = 256u32;
                                fixed_u32::bit_and(a4284, a4285)
                            };
                            let a4287 = 256u32;
                            if equal(a4286, a4287) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4288, a4289)?
                    };
                    nat_add(a4290, a4291)?
                };
                nat_add(a4292, a4293)?
            };
            let a4341 = {
                let a4338 = {
                    let a4314 = {
                        let a4302 = {
                            let a4296 = {
                                let a4294 = v2;
                                let a4295 = 128u32;
                                fixed_u32::bit_and(a4294, a4295)
                            };
                            let a4297 = 128u32;
                            if equal(a4296, a4297) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a4303 = {
                            let a4300 = {
                                let a4298 = v2;
                                let a4299 = 64u32;
                                fixed_u32::bit_and(a4298, a4299)
                            };
                            let a4301 = 64u32;
                            if equal(a4300, a4301) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4302, a4303)?
                    };
                    let a4315 = {
                        let a4312 = {
                            let a4306 = {
                                let a4304 = v2;
                                let a4305 = 32u32;
                                fixed_u32::bit_and(a4304, a4305)
                            };
                            let a4307 = 32u32;
                            if equal(a4306, a4307) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a4313 = {
                            let a4310 = {
                                let a4308 = v2;
                                let a4309 = 16u32;
                                fixed_u32::bit_and(a4308, a4309)
                            };
                            let a4311 = 16u32;
                            if equal(a4310, a4311) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4312, a4313)?
                    };
                    nat_add(a4314, a4315)?
                };
                let a4339 = {
                    let a4336 = {
                        let a4324 = {
                            let a4318 = {
                                let a4316 = v2;
                                let a4317 = 8u32;
                                fixed_u32::bit_and(a4316, a4317)
                            };
                            let a4319 = 8u32;
                            if equal(a4318, a4319) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a4325 = {
                            let a4322 = {
                                let a4320 = v2;
                                let a4321 = 4u32;
                                fixed_u32::bit_and(a4320, a4321)
                            };
                            let a4323 = 4u32;
                            if equal(a4322, a4323) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4324, a4325)?
                    };
                    let a4337 = {
                        let a4334 = {
                            let a4328 = {
                                let a4326 = v2;
                                let a4327 = 2u32;
                                fixed_u32::bit_and(a4326, a4327)
                            };
                            let a4329 = 2u32;
                            if equal(a4328, a4329) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a4335 = {
                            let a4332 = {
                                let a4330 = v2;
                                let a4331 = 1u32;
                                fixed_u32::bit_and(a4330, a4331)
                            };
                            let a4333 = 1u32;
                            if equal(a4332, a4333) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a4334, a4335)?
                    };
                    nat_add(a4336, a4337)?
                };
                nat_add(a4338, a4339)?
            };
            nat_add(a4340, a4341)?
        };
        nat_add(a4342, a4343)?
    }))
}

pub fn f1091() -> u64 {
    64u64
}

pub fn f1092(v0: Adt169) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt169::C0(h4344, _, _, _, _, _, _, _) => {
            h4344
        }
    }, match v0.clone() {
        Adt169::C0(_, h4345, _, _, _, _, _, _) => {
            h4345
        }
    }, match v0.clone() {
        Adt169::C0(_, _, h4346, _, _, _, _, _) => {
            h4346
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, h4347, _, _, _, _) => {
            h4347
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, h4348, _, _, _) => {
            h4348
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, h4349, _, _) => {
            h4349
        }
    }, match v0.clone() {
        Adt169::C0(_, _, _, _, _, _, h4350, _) => {
            h4350
        }
    }, match v0 {
        Adt169::C0(_, _, _, _, _, _, _, h4351) => {
            h4351
        }
    }, f1100(), f1100(), f1100(), f1100(), f1100(), f1100(), f1100(), f1100())
}

pub fn f1093(v0: Bytes) -> R<Adt169> {
    let v1: u64 = {
        let a4352 = v0.clone();
        length_bytes(a4352)
    };
    let v2: u64 = {
        let a4358 = {
            let a4355 = {
                let a4353 = v1;
                let a4354 = 72u64;
                nat_add(a4353, a4354)?
            };
            let a4356 = 64u64;
            let a4357 = 0u64;
            nat_quot(a4355, a4356, a4357)
        };
        let a4359 = 64u64;
        nat_mul(a4358, a4359)?
    };
    let (_, h4360) = f1095(Fn13::F1103(v0, v1, v2), {
        let a4361 = v2;
        let a4362 = 64u64;
        let a4363 = 0u64;
        nat_quot(a4361, a4362, a4363)
    }, (0u64, f1085()))?;
    Ok(h4360)
}

pub fn f1094(v0: Bytes) -> R<Adt170> {
    Ok(Adt170::C0(f1104(v0.clone(), 0u64)?, f1104(v0.clone(), 4u64)?, f1104(v0.clone(), 8u64)?, f1104(v0.clone(), 12u64)?, f1104(v0.clone(), 16u64)?, f1104(v0.clone(), 20u64)?, f1104(v0.clone(), 24u64)?, f1104(v0.clone(), 28u64)?, f1104(v0.clone(), 32u64)?, f1104(v0.clone(), 36u64)?, f1104(v0.clone(), 40u64)?, f1104(v0.clone(), 44u64)?, f1104(v0.clone(), 48u64)?, f1104(v0.clone(), 52u64)?, f1104(v0.clone(), 56u64)?, f1104(v0, 60u64)?))
}

pub fn f1095(v0: Fn13, p4367: u64, p4368: (u64, Adt169)) -> R<(u64, Adt169)> {
    let m4366 = (p4367, p4368);
    {
        let mut m4366 = m4366;
        loop {
            let step = {
                let (v1, v2) = m4366;
                let m4364 = v1;
                if m4364 == 0 {
                    Err::<(u64, (u64, Adt169)), R<(u64, Adt169)>>(Ok(v2))
                } else {
                    let v3 = m4364 - 1;
                    Ok::<(u64, (u64, Adt169)), R<(u64, Adt169)>>((v3, {
                        let c4365: Fn13 = v0.clone();
                        c4365.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1096(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4370 = {
            let (h4369, _) = v4.clone();
            h4369
        };
        let a4371 = 64u64;
        nat_add(a4370, a4371)?
    }, f1084({
        let (_, h4372) = v4;
        h4372
    }, f1105(v0, v2, v3, {
        let a4375 = {
            let a4373 = v1;
            let a4374 = v2;
            nat_add(a4373, a4374)?
        };
        let a4376 = 8u64;
        nat_mul(a4375, a4376)?
    }, {
        let (h4377, _) = v4;
        h4377
    })?)?))
}

pub fn f1097(v0: Fn14, p4381: u64, p4382: Adt171) -> R<Adt171> {
    let m4380 = (p4381, p4382);
    {
        let mut m4380 = m4380;
        loop {
            let step = {
                let (v1, v2) = m4380;
                let m4378 = v1;
                if m4378 == 0 {
                    Err::<(u64, Adt171), R<Adt171>>(Ok(v2))
                } else {
                    let v3 = m4378 - 1;
                    Ok::<(u64, Adt171), R<Adt171>>((v3, {
                        let c4379: Fn14 = v0.clone();
                        c4379.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m4380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1098(v0: Adt171) -> R<Adt171> {
    f1106(v0)
}

pub fn f1099(v0: u32, v1: u32) -> u32 {
    let m4385 = {
        let a4383 = f1107(v0);
        let a4384 = f1107(v1);
        fixed_u64::checked_add(a4383, a4384)
    };
    match m4385 {
        None => {
            0u32
        }
        Some(v2) => {
            f1108(v2)
        }
    }
}

pub fn f1100() -> u32 {
    0u32
}

pub fn f1101(v0: Bytes, v1: u64) -> u8 {
    let m4388 = {
        let a4386 = v0;
        let a4387 = v1;
        index_bytes(a4386, a4387)
    };
    match m4388 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1102(v0: u32, v1: u32) -> u32 {
    let m4391 = {
        let a4389 = v0;
        let a4390 = v1;
        fixed_u32::shift_right(a4389, a4390)
    };
    match m4391 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1103(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt169)) -> R<(u64, Adt169)> {
    Ok(({
        let a4393 = {
            let (h4392, _) = v3.clone();
            h4392
        };
        let a4394 = 64u64;
        nat_add(a4393, a4394)?
    }, f1084({
        let (_, h4395) = v3;
        h4395
    }, f1109(v0, v1, v2, {
        let (h4396, _) = v3;
        h4396
    })?)?))
}

pub fn f1104(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f1110(f1111(v0.clone(), v1), f1111(v0.clone(), {
        let a4397 = v1;
        let a4398 = 1u64;
        nat_add(a4397, a4398)?
    }), f1111(v0.clone(), {
        let a4399 = v1;
        let a4400 = 2u64;
        nat_add(a4399, a4400)?
    }), f1111(v0, {
        let a4401 = v1;
        let a4402 = 3u64;
        nat_add(a4401, a4402)?
    })))
}

pub fn f1105(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1112(v0.clone(), v1, v2, v3, v4)?, f1112(v0.clone(), v1, v2, v3, {
        let a4403 = v4;
        let a4404 = 4u64;
        nat_add(a4403, a4404)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4405 = v4;
        let a4406 = 8u64;
        nat_add(a4405, a4406)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4407 = v4;
        let a4408 = 12u64;
        nat_add(a4407, a4408)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4409 = v4;
        let a4410 = 16u64;
        nat_add(a4409, a4410)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4411 = v4;
        let a4412 = 20u64;
        nat_add(a4411, a4412)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4413 = v4;
        let a4414 = 24u64;
        nat_add(a4413, a4414)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4415 = v4;
        let a4416 = 28u64;
        nat_add(a4415, a4416)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4417 = v4;
        let a4418 = 32u64;
        nat_add(a4417, a4418)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4419 = v4;
        let a4420 = 36u64;
        nat_add(a4419, a4420)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4421 = v4;
        let a4422 = 40u64;
        nat_add(a4421, a4422)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4423 = v4;
        let a4424 = 44u64;
        nat_add(a4423, a4424)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4425 = v4;
        let a4426 = 48u64;
        nat_add(a4425, a4426)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4427 = v4;
        let a4428 = 52u64;
        nat_add(a4427, a4428)?
    })?, f1112(v0.clone(), v1, v2, v3, {
        let a4429 = v4;
        let a4430 = 56u64;
        nat_add(a4429, a4430)?
    })?, f1112(v0, v1, v2, v3, {
        let a4431 = v4;
        let a4432 = 60u64;
        nat_add(a4431, a4432)?
    })?))
}

pub fn f1106(v0: Adt171) -> R<Adt171> {
    let v1: u32 = f1113(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, _, h4433) => {
            h4433
        }
    });
    let v2: u32 = f1099(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, h4434, _, _) => {
            h4434
        }
    }, f1099(f1114(match v0.clone() {
        Adt171::C0(_, _, _, _, h4435, _, _, _, _, _) => {
            h4435
        }
    }), f1099(f1115(match v0.clone() {
        Adt171::C0(_, _, _, _, h4436, _, _, _, _, _) => {
            h4436
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4437, _, _, _, _) => {
            h4437
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4438, _, _, _) => {
            h4438
        }
    }), f1099(v1, {
        let m4441 = match v0.clone() {
            Adt171::C0(_, _, _, _, _, _, _, _, h4440, _) => {
                h4440
            }
        };
        match m4441 {
            Adt170::C0(h4439, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h4439
            }
        }
    }))));
    let v3: u32 = f1099(f1116(match v0.clone() {
        Adt171::C0(h4442, _, _, _, _, _, _, _, _, _) => {
            h4442
        }
    }), f1117(match v0.clone() {
        Adt171::C0(h4443, _, _, _, _, _, _, _, _, _) => {
            h4443
        }
    }, match v0.clone() {
        Adt171::C0(_, h4444, _, _, _, _, _, _, _, _) => {
            h4444
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4445, _, _, _, _, _, _, _) => {
            h4445
        }
    }));
    Ok(Adt171::C0(f1099(v2, v3), match v0.clone() {
        Adt171::C0(h4446, _, _, _, _, _, _, _, _, _) => {
            h4446
        }
    }, match v0.clone() {
        Adt171::C0(_, h4447, _, _, _, _, _, _, _, _) => {
            h4447
        }
    }, match v0.clone() {
        Adt171::C0(_, _, h4448, _, _, _, _, _, _, _) => {
            h4448
        }
    }, f1099(match v0.clone() {
        Adt171::C0(_, _, _, h4449, _, _, _, _, _, _) => {
            h4449
        }
    }, v2), match v0.clone() {
        Adt171::C0(_, _, _, _, h4450, _, _, _, _, _) => {
            h4450
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, h4451, _, _, _, _) => {
            h4451
        }
    }, match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, h4452, _, _, _) => {
            h4452
        }
    }, f1118(match v0.clone() {
        Adt171::C0(_, _, _, _, _, _, _, _, h4453, _) => {
            h4453
        }
    }), {
        let a4455 = match v0 {
            Adt171::C0(_, _, _, _, _, _, _, _, _, h4454) => {
                h4454
            }
        };
        let a4456 = 1u64;
        nat_add(a4455, a4456)?
    }))
}

pub fn f1107(v0: u32) -> u64 {
    let m4458 = {
        let a4457 = v0;
        fixed_u64::convert(i128::from(a4457))
    };
    match m4458 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1108(v0: u64) -> u32 {
    let m4462 = {
        let a4461 = {
            let a4459 = v0;
            let a4460 = 4294967295u64;
            fixed_u64::bit_and(a4459, a4460)
        };
        fixed_u32::convert(i128::from(a4461))
    };
    match m4462 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f1109(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt170> {
    Ok(Adt170::C0(f1119(v0.clone(), v1, v2, v3)?, f1119(v0.clone(), v1, v2, {
        let a4463 = v3;
        let a4464 = 4u64;
        nat_add(a4463, a4464)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4465 = v3;
        let a4466 = 8u64;
        nat_add(a4465, a4466)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4467 = v3;
        let a4468 = 12u64;
        nat_add(a4467, a4468)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4469 = v3;
        let a4470 = 16u64;
        nat_add(a4469, a4470)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4471 = v3;
        let a4472 = 20u64;
        nat_add(a4471, a4472)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4473 = v3;
        let a4474 = 24u64;
        nat_add(a4473, a4474)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4475 = v3;
        let a4476 = 28u64;
        nat_add(a4475, a4476)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4477 = v3;
        let a4478 = 32u64;
        nat_add(a4477, a4478)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4479 = v3;
        let a4480 = 36u64;
        nat_add(a4479, a4480)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4481 = v3;
        let a4482 = 40u64;
        nat_add(a4481, a4482)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4483 = v3;
        let a4484 = 44u64;
        nat_add(a4483, a4484)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4485 = v3;
        let a4486 = 48u64;
        nat_add(a4485, a4486)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4487 = v3;
        let a4488 = 52u64;
        nat_add(a4487, a4488)?
    })?, f1119(v0.clone(), v1, v2, {
        let a4489 = v3;
        let a4490 = 56u64;
        nat_add(a4489, a4490)?
    })?, f1119(v0, v1, v2, {
        let a4491 = v3;
        let a4492 = 60u64;
        nat_add(a4491, a4492)?
    })?))
}

pub fn f1110(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a4497 = f1120(v0, 24u32);
    let a4498 = {
        let a4495 = f1120(v1, 16u32);
        let a4496 = {
            let a4493 = f1120(v2, 8u32);
            let a4494 = v3;
            fixed_u32::bit_or(a4493, a4494)
        };
        fixed_u32::bit_or(a4495, a4496)
    };
    fixed_u32::bit_or(a4497, a4498)
}

pub fn f1111(v0: Bytes, v1: u64) -> u32 {
    let m4501 = {
        let a4499 = v0;
        let a4500 = v1;
        index_bytes(a4499, a4500)
    };
    match m4501 {
        None => {
            0u32
        }
        Some(v2) => {
            let m4503 = {
                let a4502 = v2;
                fixed_u32::convert(i128::from(a4502))
            };
            match m4503 {
                None => {
                    0u32
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f1112(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a4506 = {
        let a4504 = v4;
        let a4505 = 8u64;
        nat_add(a4504, a4505)?
    };
    let a4507 = v2;
    if nat_eq(a4506, a4507) {
        Ok(f1121(v3, true))
    } else {
        let a4510 = {
            let a4508 = v4;
            let a4509 = 4u64;
            nat_add(a4508, a4509)?
        };
        let a4511 = v2;
        if nat_eq(a4510, a4511) {
            Ok(f1121(v3, false))
        } else {
            Ok(f1110(f1122(v0.clone(), v1, v4), f1122(v0.clone(), v1, {
                let a4512 = v4;
                let a4513 = 1u64;
                nat_add(a4512, a4513)?
            }), f1122(v0.clone(), v1, {
                let a4514 = v4;
                let a4515 = 2u64;
                nat_add(a4514, a4515)?
            }), f1122(v0, v1, {
                let a4516 = v4;
                let a4517 = 3u64;
                nat_add(a4516, a4517)?
            })))
        }
    }
}

pub fn f1113(v0: u64) -> u32 {
    let a4518 = v0;
    let a4519 = 32u64;
    if nat_lt(a4518, a4519) {
        let a4520 = v0;
        let a4521 = 16u64;
        if nat_lt(a4520, a4521) {
            let a4522 = v0;
            let a4523 = 8u64;
            if nat_lt(a4522, a4523) {
                let a4524 = v0;
                let a4525 = 4u64;
                if nat_lt(a4524, a4525) {
                    let a4526 = v0;
                    let a4527 = 2u64;
                    if nat_lt(a4526, a4527) {
                        let a4528 = v0;
                        let a4529 = 1u64;
                        if nat_lt(a4528, a4529) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4530 = v0;
                        let a4531 = 3u64;
                        if nat_lt(a4530, a4531) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4532 = v0;
                    let a4533 = 6u64;
                    if nat_lt(a4532, a4533) {
                        let a4534 = v0;
                        let a4535 = 5u64;
                        if nat_lt(a4534, a4535) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4536 = v0;
                        let a4537 = 7u64;
                        if nat_lt(a4536, a4537) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4538 = v0;
                let a4539 = 12u64;
                if nat_lt(a4538, a4539) {
                    let a4540 = v0;
                    let a4541 = 10u64;
                    if nat_lt(a4540, a4541) {
                        let a4542 = v0;
                        let a4543 = 9u64;
                        if nat_lt(a4542, a4543) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4544 = v0;
                        let a4545 = 11u64;
                        if nat_lt(a4544, a4545) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4546 = v0;
                    let a4547 = 14u64;
                    if nat_lt(a4546, a4547) {
                        let a4548 = v0;
                        let a4549 = 13u64;
                        if nat_lt(a4548, a4549) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4550 = v0;
                        let a4551 = 15u64;
                        if nat_lt(a4550, a4551) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4552 = v0;
            let a4553 = 24u64;
            if nat_lt(a4552, a4553) {
                let a4554 = v0;
                let a4555 = 20u64;
                if nat_lt(a4554, a4555) {
                    let a4556 = v0;
                    let a4557 = 18u64;
                    if nat_lt(a4556, a4557) {
                        let a4558 = v0;
                        let a4559 = 17u64;
                        if nat_lt(a4558, a4559) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4560 = v0;
                        let a4561 = 19u64;
                        if nat_lt(a4560, a4561) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4562 = v0;
                    let a4563 = 22u64;
                    if nat_lt(a4562, a4563) {
                        let a4564 = v0;
                        let a4565 = 21u64;
                        if nat_lt(a4564, a4565) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4566 = v0;
                        let a4567 = 23u64;
                        if nat_lt(a4566, a4567) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4568 = v0;
                let a4569 = 28u64;
                if nat_lt(a4568, a4569) {
                    let a4570 = v0;
                    let a4571 = 26u64;
                    if nat_lt(a4570, a4571) {
                        let a4572 = v0;
                        let a4573 = 25u64;
                        if nat_lt(a4572, a4573) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4574 = v0;
                        let a4575 = 27u64;
                        if nat_lt(a4574, a4575) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4576 = v0;
                    let a4577 = 30u64;
                    if nat_lt(a4576, a4577) {
                        let a4578 = v0;
                        let a4579 = 29u64;
                        if nat_lt(a4578, a4579) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4580 = v0;
                        let a4581 = 31u64;
                        if nat_lt(a4580, a4581) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4582 = v0;
        let a4583 = 48u64;
        if nat_lt(a4582, a4583) {
            let a4584 = v0;
            let a4585 = 40u64;
            if nat_lt(a4584, a4585) {
                let a4586 = v0;
                let a4587 = 36u64;
                if nat_lt(a4586, a4587) {
                    let a4588 = v0;
                    let a4589 = 34u64;
                    if nat_lt(a4588, a4589) {
                        let a4590 = v0;
                        let a4591 = 33u64;
                        if nat_lt(a4590, a4591) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4592 = v0;
                        let a4593 = 35u64;
                        if nat_lt(a4592, a4593) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4594 = v0;
                    let a4595 = 38u64;
                    if nat_lt(a4594, a4595) {
                        let a4596 = v0;
                        let a4597 = 37u64;
                        if nat_lt(a4596, a4597) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4598 = v0;
                        let a4599 = 39u64;
                        if nat_lt(a4598, a4599) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4600 = v0;
                let a4601 = 44u64;
                if nat_lt(a4600, a4601) {
                    let a4602 = v0;
                    let a4603 = 42u64;
                    if nat_lt(a4602, a4603) {
                        let a4604 = v0;
                        let a4605 = 41u64;
                        if nat_lt(a4604, a4605) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4606 = v0;
                        let a4607 = 43u64;
                        if nat_lt(a4606, a4607) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4608 = v0;
                    let a4609 = 46u64;
                    if nat_lt(a4608, a4609) {
                        let a4610 = v0;
                        let a4611 = 45u64;
                        if nat_lt(a4610, a4611) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4612 = v0;
                        let a4613 = 47u64;
                        if nat_lt(a4612, a4613) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4614 = v0;
            let a4615 = 56u64;
            if nat_lt(a4614, a4615) {
                let a4616 = v0;
                let a4617 = 52u64;
                if nat_lt(a4616, a4617) {
                    let a4618 = v0;
                    let a4619 = 50u64;
                    if nat_lt(a4618, a4619) {
                        let a4620 = v0;
                        let a4621 = 49u64;
                        if nat_lt(a4620, a4621) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4622 = v0;
                        let a4623 = 51u64;
                        if nat_lt(a4622, a4623) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4624 = v0;
                    let a4625 = 54u64;
                    if nat_lt(a4624, a4625) {
                        let a4626 = v0;
                        let a4627 = 53u64;
                        if nat_lt(a4626, a4627) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4628 = v0;
                        let a4629 = 55u64;
                        if nat_lt(a4628, a4629) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4630 = v0;
                let a4631 = 60u64;
                if nat_lt(a4630, a4631) {
                    let a4632 = v0;
                    let a4633 = 58u64;
                    if nat_lt(a4632, a4633) {
                        let a4634 = v0;
                        let a4635 = 57u64;
                        if nat_lt(a4634, a4635) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4636 = v0;
                        let a4637 = 59u64;
                        if nat_lt(a4636, a4637) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4638 = v0;
                    let a4639 = 62u64;
                    if nat_lt(a4638, a4639) {
                        let a4640 = v0;
                        let a4641 = 61u64;
                        if nat_lt(a4640, a4641) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4642 = v0;
                        let a4643 = 63u64;
                        if nat_lt(a4642, a4643) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f1114(v0: u32) -> u32 {
    let a4646 = {
        let a4644 = f1123(v0, 6u32, 26u32);
        let a4645 = f1123(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4644, a4645)
    };
    let a4647 = f1123(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4646, a4647)
}

pub fn f1115(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4653 = {
        let a4648 = v0;
        let a4649 = v1;
        fixed_u32::bit_and(a4648, a4649)
    };
    let a4654 = {
        let a4651 = {
            let a4650 = v0;
            fixed_u32::bit_not(a4650)
        };
        let a4652 = v2;
        fixed_u32::bit_and(a4651, a4652)
    };
    fixed_u32::bit_xor(a4653, a4654)
}

pub fn f1116(v0: u32) -> u32 {
    let a4657 = {
        let a4655 = f1123(v0, 2u32, 30u32);
        let a4656 = f1123(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4655, a4656)
    };
    let a4658 = f1123(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4657, a4658)
}

pub fn f1117(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4667 = {
        let a4663 = {
            let a4659 = v0;
            let a4660 = v1;
            fixed_u32::bit_and(a4659, a4660)
        };
        let a4664 = {
            let a4661 = v0;
            let a4662 = v2;
            fixed_u32::bit_and(a4661, a4662)
        };
        fixed_u32::bit_xor(a4663, a4664)
    };
    let a4668 = {
        let a4665 = v1;
        let a4666 = v2;
        fixed_u32::bit_and(a4665, a4666)
    };
    fixed_u32::bit_xor(a4667, a4668)
}

pub fn f1118(v0: Adt170) -> Adt170 {
    Adt170::C0(match v0.clone() {
        Adt170::C0(_, h4669, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4669
        }
    }, match v0.clone() {
        Adt170::C0(_, _, h4670, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4670
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, h4671, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4671
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, h4672, _, _, _, _, _, _, _, _, _, _, _) => {
            h4672
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, h4673, _, _, _, _, _, _, _, _, _, _) => {
            h4673
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, h4674, _, _, _, _, _, _, _, _, _) => {
            h4674
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, h4675, _, _, _, _, _, _, _, _) => {
            h4675
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, h4676, _, _, _, _, _, _, _) => {
            h4676
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4677, _, _, _, _, _, _) => {
            h4677
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, h4678, _, _, _, _, _) => {
            h4678
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, h4679, _, _, _, _) => {
            h4679
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4680, _, _, _) => {
            h4680
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4681, _, _) => {
            h4681
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4682, _) => {
            h4682
        }
    }, match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4683) => {
            h4683
        }
    }, f1099(f1124(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4684, _) => {
            h4684
        }
    }), f1099(match v0.clone() {
        Adt170::C0(_, _, _, _, _, _, _, _, _, h4685, _, _, _, _, _, _) => {
            h4685
        }
    }, f1099(f1125(match v0.clone() {
        Adt170::C0(_, h4686, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4686
        }
    }), match v0 {
        Adt170::C0(h4687, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4687
        }
    }))))
}

pub fn f1119(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4690 = {
        let a4688 = v3;
        let a4689 = 8u64;
        nat_add(a4688, a4689)?
    };
    let a4691 = v2;
    if nat_eq(a4690, a4691) {
        Ok(f1126({
            let a4694 = {
                let a4692 = v1;
                let a4693 = 8u64;
                nat_mul(a4692, a4693)?
            };
            let a4695 = 4294967296u64;
            let a4696 = 0u64;
            nat_quot(a4694, a4695, a4696)
        }))
    } else {
        let a4699 = {
            let a4697 = v3;
            let a4698 = 4u64;
            nat_add(a4697, a4698)?
        };
        let a4700 = v2;
        if nat_eq(a4699, a4700) {
            Ok(f1126({
                let a4703 = {
                    let a4701 = v1;
                    let a4702 = 8u64;
                    nat_mul(a4701, a4702)?
                };
                let a4704 = 4294967296u64;
                let a4705 = 0u64;
                nat_rem(a4703, a4704, a4705)
            }))
        } else {
            Ok(f1110(f1122(v0.clone(), v1, v3), f1122(v0.clone(), v1, {
                let a4706 = v3;
                let a4707 = 1u64;
                nat_add(a4706, a4707)?
            }), f1122(v0.clone(), v1, {
                let a4708 = v3;
                let a4709 = 2u64;
                nat_add(a4708, a4709)?
            }), f1122(v0, v1, {
                let a4710 = v3;
                let a4711 = 3u64;
                nat_add(a4710, a4711)?
            })))
        }
    }
}

pub fn f1120(v0: u32, v1: u32) -> u32 {
    let m4714 = {
        let a4712 = v0;
        let a4713 = v1;
        fixed_u32::shift_left(a4712, a4713)
    };
    match m4714 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f1121(v0: u64, v1: bool) -> u32 {
    if v1 {
        f1126({
            let a4715 = v0;
            let a4716 = 4294967296u64;
            let a4717 = 0u64;
            nat_quot(a4715, a4716, a4717)
        })
    } else {
        f1126({
            let a4718 = v0;
            let a4719 = 4294967296u64;
            let a4720 = 0u64;
            nat_rem(a4718, a4719, a4720)
        })
    }
}

pub fn f1122(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4721 = v2;
    let a4722 = v1;
    if nat_lt(a4721, a4722) {
        let m4725 = {
            let a4723 = v0;
            let a4724 = v2;
            index_bytes(a4723, a4724)
        };
        match m4725 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4727 = {
                    let a4726 = v3;
                    fixed_u32::convert(i128::from(a4726))
                };
                match m4727 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a4728 = v2;
        let a4729 = v1;
        if nat_eq(a4728, a4729) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f1123(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4730 = f1102(v0, v1);
    let a4731 = f1120(v0, v2);
    fixed_u32::bit_or(a4730, a4731)
}

pub fn f1124(v0: u32) -> u32 {
    let a4734 = {
        let a4732 = f1123(v0, 17u32, 15u32);
        let a4733 = f1123(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4732, a4733)
    };
    let a4735 = f1102(v0, 10u32);
    fixed_u32::bit_xor(a4734, a4735)
}

pub fn f1125(v0: u32) -> u32 {
    let a4738 = {
        let a4736 = f1123(v0, 7u32, 25u32);
        let a4737 = f1123(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4736, a4737)
    };
    let a4739 = f1102(v0, 3u32);
    fixed_u32::bit_xor(a4738, a4739)
}

pub fn f1126(v0: u64) -> u32 {
    let (_, h4740) = {
        let (_, h4741) = f1127(Fn15::F1128, 32u64, (v0, (1u32, 0u32)));
        h4741
    };
    h4740
}

pub fn f1127(v0: Fn15, p4745: u64, p4746: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4744 = (p4745, p4746);
    {
        let mut m4744 = m4744;
        loop {
            let step = {
                let (v1, v2) = m4744;
                let m4742 = v1;
                if m4742 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4742 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4743: Fn15 = v0.clone();
                        c4743.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f1128(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4747, _) = v0;
        h4747
    };
    let v2: u32 = {
        let (h4748, _) = {
            let (_, h4749) = v0;
            h4749
        };
        h4748
    };
    let v3: u32 = {
        let (_, h4750) = {
            let (_, h4751) = v0;
            h4751
        };
        h4750
    };
    ({
        let a4752 = v1;
        let a4753 = 2u64;
        let a4754 = 0u64;
        nat_quot(a4752, a4753, a4754)
    }, (f1120(v2, 1u32), {
        let a4758 = {
            let a4755 = v1;
            let a4756 = 2u64;
            let a4757 = 0u64;
            nat_rem(a4755, a4756, a4757)
        };
        let a4759 = 1u64;
        if nat_eq(a4758, a4759) {
            let a4760 = v3;
            let a4761 = v2;
            fixed_u32::bit_or(a4760, a4761)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
