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
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>, u64),
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
    C0(i64, Str, Str, Bytes),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt16),
    C1(Adt21),
    C2(Adt22),
    C3(Adt21),
    C4(Adt23),
    C5(Adt21),
    C6(Adt21),
    C7(Adt24),
    C8(Adt25),
    C9(Adt30),
    C10(Adt31),
    C11(Adt32),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Adt17),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Str, Adt18, Option<Adt5>, Option<Adt19>, Option<Adt20>, Adt5),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt19 {
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
pub enum Adt20 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Adt17, Option<Str>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt17, Str, Option<Adt5>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt17, Str, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt17, Str, Option<List<(Str, Str)>>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt17, Adt26, Adt28, Option<Adt29>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Adt27),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt17, Str),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt17, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt17, Str),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt34),
    C1(Adt35),
    C2(Adt38),
    C3(Adt50),
    C4(Adt52),
    C5(Adt53),
    C6(Adt54),
    C7(Adt56),
    C8(Adt59),
    C9(Adt61),
    C10(Adt63),
    C11,
    C12(Adt82),
    C13(Adt83),
    C14(Adt88),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt10, List<Str>, Adt4, Option<Str>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt10, Option<Str>, List<Adt36>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Adt5, Option<Adt5>, Option<Adt37>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(List<Adt39>, Option<Adt6>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Adt40),
    C1(Adt43),
    C2(Adt44),
    C3(Adt45),
    C4(Adt46),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Str, Option<Adt41>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0(Option<List<Adt42>>, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt42 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt43 {
    C0(Str, Str, Option<Adt41>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Str, Str, Option<Adt41>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt41>, Option<u64>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(Adt47, Option<Adt41>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Adt48),
    C1(Adt49),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt38, List<Adt51>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Adt7, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Adt10, Option<Str>, List<Adt45>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt10, List<Adt47>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Adt10, Option<Str>, List<Adt55>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt41>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Adt10, Option<Str>, List<Adt57>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Option<List<Adt58>>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Option<Str>, List<Adt60>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Adt42, Adt39),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Adt62),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Adt64),
    C1(Adt64, List<(Str, Adt65)>),
    C2(Adt64, Adt5),
    C3(Adt64, Adt81),
    C4(Adt64),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Str, Option<Str>, Str, Str, Option<i64>, Option<i64>),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(Adt66),
    C1(Adt70),
    C2(Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Adt67),
    C1(Adt69),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(Option<Adt68>, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt68 {
    C0,
}

#[derive(Clone)]
pub enum Adt69 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt70 {
    C0(List<Adt71>, Option<Adt76>, Option<Str>, Option<Adt78>, Option<Adt7>, u64, Option<List<Str>>, Option<Adt5>, Option<List<Adt36>>, Option<Adt79>),
}

#[derive(Clone)]
pub enum Adt71 {
    C0(Adt42, Adt72, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt72 {
    C0(Adt73),
    C1(List<Adt73>),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Adt40),
    C1(Adt43),
    C2(Adt44),
    C3(Adt74),
    C4(Adt75),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Str, Str, Adt5, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Str, List<Adt39>, Option<Adt6>, Option<bool>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Option<List<Adt77>>, Option<Adt7>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt78 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt79 {
    C0(Option<Adt80>),
}

#[derive(Clone)]
pub enum Adt80 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt81 {
    C0(i64, Str, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt82 {
    C0(List<(Str, Adt65)>),
    C1(Str),
    C2(List<(Str, Adt65)>, Str),
}

#[derive(Clone)]
pub enum Adt83 {
    C0(Adt84, Adt64, Adt85),
}

#[derive(Clone)]
pub enum Adt84 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt85 {
    C0(Adt63, i64, Adt86),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(i64, List<(Str, Adt65)>, i64, Adt87),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(Adt5),
    C1(Adt81),
}

#[derive(Clone)]
pub enum Adt88 {
    C0(Adt85, Adt89, bool),
}

#[derive(Clone)]
pub enum Adt89 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt90 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt18),
    C9(Str),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(List<u64>, List<u64>, List<Adt92>, Bytes, u64, List<List<u64>>, Adt14),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(u64),
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(u64, List<Adt92>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt96 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt97 {
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
pub enum Adt98 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt99 {
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
pub enum Adt100 {
    C0,
    C1(Option<Adt98>, Adt101),
}

#[derive(Clone)]
pub enum Adt101 {
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
pub enum Adt102 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt103 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt98, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0,
    C1(List<Adt107>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt107 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1(List<Adt107>, Str),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Adt6, Adt6, List<Adt107>),
    C1(List<Adt6>, Adt6, List<Adt107>),
    C2(List<Adt6>, Adt6, List<Adt107>, u64),
    C3(List<Adt107>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt107>),
    C5(Adt110, Adt6, Adt6, List<Adt107>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt111 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt113>, List<Adt115>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt114),
    C7(Adt114),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Str, List<Str>, Adt13, Adt116),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt118 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Adt120, List<Adt124>, u64),
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt121, List<u64>, bool),
    C7(u64, u64),
    C8(Adt122),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt121 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(bool, Adt123, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt123 {
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
pub enum Adt124 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Adt108, List<Adt109>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt98, Result<Adt128, Adt131>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0(Adt33, Adt129, List<Adt130>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Adt20, Adt51),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Adt132),
    C1(Str, List<Str>),
    C2(Adt113),
    C3(Str),
    C4(Adt90),
}

#[derive(Clone)]
pub enum Adt132 {
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
pub enum Adt133 {
    C0(Option<Str>, Option<Adt113>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1(Adt98),
    C2,
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt99),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt109>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt99),
    C1(List<Adt111>, List<u64>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt98, Adt140, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt140 {
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
pub enum Adt141 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1(Adt140),
}

#[derive(Clone)]
pub enum Adt143 {
    C0(Adt144),
    C1(Adt131),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt98, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt147 {
    C0(Adt97),
    C1(Adt125),
}

#[derive(Clone)]
pub enum Adt148 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt150 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt151 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt152 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt153 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt154 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt155 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt156 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt157 {
    C0,
    C1(Adt19),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14, p1: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn0::F3 => f3(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F55,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt22) -> Result<Adt50, Adt90> {
        match self {
            Fn1::F55 => f55(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F67(Fn0, Adt91),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn2::F67(k0, k1) => f67(k0.clone(), k1.clone(), p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    f1(f2()?, Fn0::F3, v0)
}

pub fn f1(v0: Adt0, v1: Fn0, v2: Bytes) -> R<List<u64>> {
    let m1 = f4(v2)?;
    match m1 {
        Ok(v4) => {
            Ok(f5(f7(v0, v1, v4)?))
        }
        Err(_) => {
            Ok(f5(f6()))
        }
    }
}

pub fn f2() -> R<Adt0> {
    Ok(Adt0::C0(f8(), None::<Str>, f9(), f10(), f11(f12()?)?, f13()))
}

pub fn f3(_: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    f14(v1)
}

pub fn f4(v0: Bytes) -> R<Result<Adt91, Adt93>> {
    let m2 = f15(v0.clone(), 0u64)?;
    match m2 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(0u64)))
        }
        Some(v1) => {
            f16(v0.clone(), v1.clone(), f15(v0, match v1 {
                Adt95::C0(_, h3) => {
                    h3
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt94) -> List<u64> {
    let a27 = f17(match v0.clone() {
        Adt94::C0(h4, _, _, _, _) => {
            h4
        }
    });
    let a28 = {
        let a25 = f17({
            let a6 = match v0.clone() {
                Adt94::C0(_, h5, _, _, _) => {
                    h5
                }
            };
            length_list(a6)
        });
        let a26 = {
            let a23 = f18(match v0.clone() {
                Adt94::C0(_, h7, _, _, _) => {
                    h7
                }
            });
            let a24 = {
                let a21 = f17({
                    let a9 = match v0.clone() {
                        Adt94::C0(_, _, h8, _, _) => {
                            h8
                        }
                    };
                    length_list(a9)
                });
                let a22 = {
                    let a19 = f19(match v0.clone() {
                        Adt94::C0(_, _, h10, _, _) => {
                            h10
                        }
                    });
                    let a20 = {
                        let a17 = f17({
                            let a12 = match v0.clone() {
                                Adt94::C0(_, _, _, h11, _) => {
                                    h11
                                }
                            };
                            length_list(a12)
                        });
                        let a18 = {
                            let a15 = match v0.clone() {
                                Adt94::C0(_, _, _, h13, _) => {
                                    h13
                                }
                            };
                            let a16 = f20(match v0 {
                                Adt94::C0(_, _, _, _, h14) => {
                                    h14
                                }
                            });
                            append_list(a15, a16)
                        };
                        append_list(a17, a18)
                    };
                    append_list(a19, a20)
                };
                append_list(a21, a22)
            };
            append_list(a23, a24)
        };
        append_list(a25, a26)
    };
    append_list(a27, a28)
}

pub fn f6() -> Adt94 {
    Adt94::C0(f21(), f22(), List::<List<u64>>::nil(), f23(), None::<List<u64>>)
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt91::C0(_, _, h29, _, _, _, _) => {
            h29
        }
    }, match v2 {
        Adt91::C0(_, _, _, _, _, h30, _) => {
            h30
        }
    })?)
}

pub fn f8() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f9() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt8>, None::<Adt9>, Some(Adt8::C0(None::<bool>)), Some(Adt5::C0(List::cons((f26(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f10() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f11(v0: List<Adt36>) -> R<List<Adt12>> {
    let m31 = f27(v0)?;
    match m31 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f12() -> R<List<Adt36>> {
    f28(f29(), List::<Adt36>::nil())
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m32 = f30(f12()?)?;
    match m32 {
        None => {
            f31(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt33, Adt90>(v1))
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<Option<Adt95>> {
    let m33 = f32(v0.clone(), v1)?;
    match m33 {
        None => {
            Ok(None::<Adt95>)
        }
        Some(v2) => {
            let m39 = {
                let a36 = v0;
                let a37 = {
                    let a34 = v1;
                    let a35 = 4u64;
                    nat_add(a34, a35)?
                };
                let a38 = v2;
                slice_bytes(a36, a37, a38)
            };
            match m39 {
                None => {
                    Ok(None::<Adt95>)
                }
                Some(v3) => {
                    Ok(Some(Adt95::C0(v3, {
                        let a42 = {
                            let a40 = v1;
                            let a41 = 4u64;
                            nat_add(a40, a41)?
                        };
                        let a43 = v2;
                        nat_add(a42, a43)?
                    })))
                }
            }
        }
    }
}

pub fn f16(v0: Bytes, v1: Adt95, v2: Option<Adt95>) -> R<Result<Adt91, Adt93>> {
    let m44 = v2;
    match m44 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v1 {
                Adt95::C0(_, h45) => {
                    h45
                }
            })))
        }
        Some(v3) => {
            f33(v0.clone(), v1, v3.clone(), f32(v0, match v3 {
                Adt95::C0(_, h46) => {
                    h46
                }
            })?)
        }
    }
}

pub fn f17(v0: u64) -> List<u64> {
    List::cons({
        let a50 = {
            let a47 = v0;
            let a48 = 16777216u64;
            let a49 = 0u64;
            nat_quot(a47, a48, a49)
        };
        let a51 = 256u64;
        let a52 = 0u64;
        nat_rem(a50, a51, a52)
    }, List::cons({
        let a56 = {
            let a53 = v0;
            let a54 = 65536u64;
            let a55 = 0u64;
            nat_quot(a53, a54, a55)
        };
        let a57 = 256u64;
        let a58 = 0u64;
        nat_rem(a56, a57, a58)
    }, List::cons({
        let a62 = {
            let a59 = v0;
            let a60 = 256u64;
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        let a63 = 256u64;
        let a64 = 0u64;
        nat_rem(a62, a63, a64)
    }, List::cons({
        let a65 = v0;
        let a66 = 256u64;
        let a67 = 0u64;
        nat_rem(a65, a66, a67)
    }, List::<u64>::nil()))))
}

pub fn f18(v0: List<Adt92>) -> List<u64> {
    f34(f35(v0, List::<Adt92>::nil()), List::<u64>::nil())
}

pub fn f19(v0: List<List<u64>>) -> List<u64> {
    f36(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f20(v0: Option<List<u64>>) -> List<u64> {
    let m68 = v0;
    match m68 {
        None => {
            let a69 = f17(0u64);
            let a70 = f17(0u64);
            append_list(a69, a70)
        }
        Some(v1) => {
            let a74 = f17(1u64);
            let a75 = {
                let a72 = f17({
                    let a71 = v1.clone();
                    length_list(a71)
                });
                let a73 = v1;
                append_list(a72, a73)
            };
            append_list(a74, a75)
        }
    }
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt92> {
    List::cons(Adt92::C0(f38(), f39()), List::<Adt92>::nil())
}

pub fn f23() -> List<u64> {
    let a78 = f40();
    let a79 = {
        let a76 = f41();
        let a77 = f42();
        append_list(a76, a77)
    };
    append_list(a78, a79)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt91, v3: Option<Adt96>) -> R<Adt94> {
    let m80 = v3;
    match m80 {
        None => {
            f44(v0, v1, v2)
        }
        Some(v4) => {
            f43(v4)
        }
    }
}

pub fn f25(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m81 = f45(v0.clone(), v1.clone())?;
    match m81 {
        None => {
            f46(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f26() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f27(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt97>> {
    let m82 = f47(v0.clone());
    match m82 {
        None => {
            f48(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f28(p85: u64, p86: List<Adt36>) -> R<List<Adt36>> {
    let m84 = (p85, p86);
    {
        let mut m84 = m84;
        loop {
            let step = {
                let (v0, v1) = m84;
                let m83 = v0;
                if m83 == 0 {
                    Err::<(u64, List<Adt36>), R<List<Adt36>>>(Ok(v1))
                } else {
                    let v2 = m83 - 1;
                    let v3: Adt36 = f49(f50(v2)?, Str::lit("Answers with its own name"), f51());
                    Ok::<(u64, List<Adt36>), R<List<Adt36>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m84 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29() -> u64 {
    60u64
}

pub fn f30(v0: List<Adt36>) -> R<Option<Adt90>> {
    let m87 = f27(v0)?;
    match m87 {
        Ok(_) => {
            Ok(None::<Adt90>)
        }
        Err(v1) => {
            Ok(Some(Adt90::C9(f52(v1))))
        }
    }
}

pub fn f31(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m88 = v0;
    match m88 {
        Adt15::C0(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C1(v1) => {
            Ok(f53(f10(), f12()?, v1))
        }
        Adt15::C2(v2) => {
            f54(f12()?, Fn1::F55, v2)
        }
        Adt15::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C4(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C5(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C6(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C7(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C8(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C9(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C10(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt15::C11(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the crowd offers tools only"))))
        }
    }
}

pub fn f32(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a92 = {
        let a89 = v1;
        let a90 = 4u64;
        nat_add(a89, a90)?
    };
    let a93 = {
        let a91 = v0.clone();
        length_bytes(a91)
    };
    if nat_le(a92, a93) {
        Ok(Some({
            let a110 = {
                let a94 = f56(v0.clone(), v1)?;
                let a95 = 16777216u64;
                nat_mul(a94, a95)?
            };
            let a111 = {
                let a108 = {
                    let a98 = f56(v0.clone(), {
                        let a96 = v1;
                        let a97 = 1u64;
                        nat_add(a96, a97)?
                    })?;
                    let a99 = 65536u64;
                    nat_mul(a98, a99)?
                };
                let a109 = {
                    let a106 = {
                        let a102 = f56(v0.clone(), {
                            let a100 = v1;
                            let a101 = 2u64;
                            nat_add(a100, a101)?
                        })?;
                        let a103 = 256u64;
                        nat_mul(a102, a103)?
                    };
                    let a107 = f56(v0, {
                        let a104 = v1;
                        let a105 = 3u64;
                        nat_add(a104, a105)?
                    })?;
                    nat_add(a106, a107)?
                };
                nat_add(a108, a109)?
            };
            nat_add(a110, a111)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f33(v0: Bytes, v1: Adt95, v2: Adt95, v3: Option<u64>) -> R<Result<Adt91, Adt93>> {
    let m112 = v3;
    match m112 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0(match v2 {
                Adt95::C0(_, h113) => {
                    h113
                }
            })))
        }
        Some(v4) => {
            let m117 = f57(v0.clone(), {
                let a115 = match v2.clone() {
                    Adt95::C0(_, h114) => {
                        h114
                    }
                };
                let a116 = 4u64;
                nat_add(a115, a116)?
            }, v4, List::<Adt92>::nil())?;
            match m117 {
                Ok(v6) => {
                    f58(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt91, Adt93>(v5))
                }
            }
        }
    }
}

pub fn f34(p122: List<Adt92>, p123: List<u64>) -> List<u64> {
    let m121 = (p122, p123);
    {
        let mut m121 = m121;
        loop {
            let step = {
                let (v0, v1) = m121;
                let m118 = v0;
                match m118.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<u64>), List<u64>>((v3, {
                            let a119 = f59(v2);
                            let a120 = v1;
                            append_list(a119, a120)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m121 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(p126: List<Adt92>, p127: List<Adt92>) -> List<Adt92> {
    let m125 = (p126, p127);
    {
        let mut m125 = m125;
        loop {
            let step = {
                let (v0, v1) = m125;
                let m124 = v0;
                match m124.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m125 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f36(p132: List<List<u64>>, p133: List<u64>) -> List<u64> {
    let m131 = (p132, p133);
    {
        let mut m131 = m131;
        loop {
            let step = {
                let (v0, v1) = m131;
                let m128 = v0;
                match m128.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a129 = f60(v2);
                            let a130 = v1;
                            append_list(a129, a130)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m131 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(p136: List<List<u64>>, p137: List<List<u64>>) -> List<List<u64>> {
    let m135 = (p136, p137);
    {
        let mut m135 = m135;
        loop {
            let step = {
                let (v0, v1) = m135;
                let m134 = v0;
                match m134.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f38() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f39() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f40() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f41() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f42() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f43(v0: Adt96) -> R<Adt94> {
    f61(f62(v0.clone()), f63(None::<Adt98>, f64(v0)))
}

pub fn f44(v0: Adt0, v1: Fn0, v2: Adt91) -> R<Adt94> {
    if f65(match v2.clone() {
        Adt91::C0(h138, _, _, _, _, _, _) => {
            h138
        }
    }) {
        f66(match v2.clone() {
            Adt91::C0(_, _, _, _, _, _, h139) => {
                h139
            }
        }, v0, Fn2::F67(v1, v2.clone()), match v2.clone() {
            Adt91::C0(_, _, _, _, h140, _, _) => {
                h140
            }
        }, match v2.clone() {
            Adt91::C0(_, _, h141, _, _, _, _) => {
                h141
            }
        }, f68(match v2 {
            Adt91::C0(_, _, _, h142, _, _, _) => {
                h142
            }
        })?)
    } else {
        Ok(f69())
    }
}

pub fn f45(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m143 = f70(v0, f71())?;
    match m143 {
        Adt102::C0 => {
            Ok(None::<Adt96>)
        }
        Adt102::C1 => {
            Ok(Some(Adt96::C0))
        }
        Adt102::C2 => {
            Ok(Some(Adt96::C0))
        }
        Adt102::C3(v2) => {
            f72(f73(v2), v1, Adt96::C0)
        }
    }
}

pub fn f46(v0: List<Adt92>, v1: List<List<u64>>) -> R<Option<Adt96>> {
    let m144 = f70(v0, f74())?;
    match m144 {
        Adt102::C0 => {
            Ok(None::<Adt96>)
        }
        Adt102::C1 => {
            Ok(Some(Adt96::C1))
        }
        Adt102::C2 => {
            Ok(Some(Adt96::C1))
        }
        Adt102::C3(v2) => {
            f72(f75(v2), v1, Adt96::C1)
        }
    }
}

pub fn f47(v0: List<Adt36>) -> Option<Adt97> {
    let m145 = f76(v0, List::<Str>::nil());
    match m145 {
        None => {
            None::<Adt97>
        }
        Some(v1) => {
            Some(Adt97::C1(v1))
        }
    }
}

pub fn f48(p149: List<Adt36>, p150: List<Adt12>) -> R<Result<List<Adt12>, Adt97>> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v0, v1) = m148;
                let m146 = v0;
                match m146.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Ok::<List<Adt12>, Adt97>(f77(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m147 = f78(v2)?;
                        match m147 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>((v3, f77(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt97>>>(Ok(Err::<List<Adt12>, Adt97>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f50(v0: u64) -> R<Str> {
    let a152 = List::cons(Str::lit("crowd_"), List::cons({
        let a151 = f79(v0, 0i64)?;
        format_i64(a151)
    }, List::<Str>::nil()));
    let a153 = Str::lit("");
    Ok(join(a152, a153))
}

pub fn f51() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f52(v0: Adt97) -> Str {
    let m154 = v0;
    match m154 {
        Adt97::C0(v1) => {
            let a155 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a156 = Str::lit(" ");
            join(a155, a156)
        }
        Adt97::C1(v2) => {
            let a157 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a158 = Str::lit(" ");
            join(a157, a158)
        }
        Adt97::C2(v3) => {
            let a159 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a160 = Str::lit(" ");
            join(a159, a160)
        }
        Adt97::C3(v4) => {
            let a161 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a162 = Str::lit(" ");
            join(a161, a162)
        }
        Adt97::C4(v5, v6) => {
            let a163 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a164 = Str::lit(" ");
            join(a163, a164)
        }
        Adt97::C5(v7, v8) => {
            let a165 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a166 = Str::lit(" ");
            join(a165, a166)
        }
        Adt97::C6(v9, v10) => {
            let a167 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a168 = Str::lit(" ");
            join(a167, a168)
        }
        Adt97::C7(v11) => {
            let a169 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a170 = Str::lit(" ");
            join(a169, a170)
        }
        Adt97::C8(v12) => {
            let a171 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a172 = Str::lit(" ");
            join(a171, a172)
        }
        Adt97::C9(v13) => {
            let a173 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a174 = Str::lit(" ");
            join(a173, a174)
        }
        Adt97::C10(v14) => {
            let a175 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a176 = Str::lit(" ");
            join(a175, a176)
        }
        Adt97::C11(v15) => {
            let a177 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a178 = Str::lit(" ");
            join(a177, a178)
        }
        Adt97::C12(v16, v17) => {
            let a179 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a180 = Str::lit(" ");
            join(a179, a180)
        }
    }
}

pub fn f53(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m182 = match v2 {
        Adt21::C0(_, h181) => {
            h181
        }
    };
    match m182 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f54(v0: List<Adt36>, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m184 = f80(match v2.clone() {
        Adt22::C0(_, h183, _, _, _) => {
            h183
        }
    }, v0);
    match m184 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h185, _, _, _) => {
                    h185
                }
            })))
        }
        Some(v3) => {
            f81(v3, v1, v2)
        }
    }
}

pub fn f55(v0: Adt22) -> Result<Adt50, Adt90> {
    Ok::<Adt50, Adt90>(f82(List::cons(f83(match v0 {
        Adt22::C0(_, h186, _, _, _) => {
            h186
        }
    }), List::<Adt39>::nil())))
}

pub fn f56(v0: Bytes, v1: u64) -> R<u64> {
    let m189 = {
        let a187 = v0;
        let a188 = v1;
        index_bytes(a187, a188)
    };
    match m189 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a234 = {
                let a210 = {
                    let a198 = {
                        let a192 = {
                            let a190 = v3;
                            let a191 = 128u8;
                            fixed_u8::bit_and(a190, a191)
                        };
                        let a193 = 128u8;
                        if equal(a192, a193) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a199 = {
                        let a196 = {
                            let a194 = v3;
                            let a195 = 64u8;
                            fixed_u8::bit_and(a194, a195)
                        };
                        let a197 = 64u8;
                        if equal(a196, a197) {
                            64u64
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
                            let a201 = 32u8;
                            fixed_u8::bit_and(a200, a201)
                        };
                        let a203 = 32u8;
                        if equal(a202, a203) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a209 = {
                        let a206 = {
                            let a204 = v3;
                            let a205 = 16u8;
                            fixed_u8::bit_and(a204, a205)
                        };
                        let a207 = 16u8;
                        if equal(a206, a207) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a208, a209)?
                };
                nat_add(a210, a211)?
            };
            let a235 = {
                let a232 = {
                    let a220 = {
                        let a214 = {
                            let a212 = v3;
                            let a213 = 8u8;
                            fixed_u8::bit_and(a212, a213)
                        };
                        let a215 = 8u8;
                        if equal(a214, a215) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a221 = {
                        let a218 = {
                            let a216 = v3;
                            let a217 = 4u8;
                            fixed_u8::bit_and(a216, a217)
                        };
                        let a219 = 4u8;
                        if equal(a218, a219) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a220, a221)?
                };
                let a233 = {
                    let a230 = {
                        let a224 = {
                            let a222 = v3;
                            let a223 = 2u8;
                            fixed_u8::bit_and(a222, a223)
                        };
                        let a225 = 2u8;
                        if equal(a224, a225) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a231 = {
                        let a228 = {
                            let a226 = v3;
                            let a227 = 1u8;
                            fixed_u8::bit_and(a226, a227)
                        };
                        let a229 = 1u8;
                        if equal(a228, a229) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a230, a231)?
                };
                nat_add(a232, a233)?
            };
            nat_add(a234, a235)
        }
    }
}

pub fn f57(v0: Bytes, p245: u64, p246: u64, p247: List<Adt92>) -> R<Result<(List<Adt92>, u64), Adt93>> {
    let m244 = (p245, (p246, p247));
    {
        let mut m244 = m244;
        loop {
            let step = {
                let (v1, (v2, v3)) = m244;
                let m236 = v2;
                if m236 == 0 {
                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Ok::<(List<Adt92>, u64), Adt93>((f84(v3, List::<Adt92>::nil()), v1))))
                } else {
                    let v4 = m236 - 1;
                    let m237 = f15(v0.clone(), v1)?;
                    match m237 {
                        None => {
                            Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            let m239 = f15(v0.clone(), match v5.clone() {
                                Adt95::C0(_, h238) => {
                                    h238
                                }
                            })?;
                            match m239 {
                                None => {
                                    Err::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>(Ok(Err::<(List<Adt92>, u64), Adt93>(Adt93::C0(match v5 {
                                        Adt95::C0(_, h240) => {
                                            h240
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt92>)), R<Result<(List<Adt92>, u64), Adt93>>>((match v6.clone() {
                                        Adt95::C0(_, h241) => {
                                            h241
                                        }
                                    }, (v4, List::cons(Adt92::C0(f85(match v5 {
                                        Adt95::C0(h242, _) => {
                                            h242
                                        }
                                    })?, f85(match v6 {
                                        Adt95::C0(h243, _) => {
                                            h243
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: Adt95, v2: Adt95, v3: (List<Adt92>, u64)) -> R<Result<Adt91, Adt93>> {
    let m249 = f15(v0.clone(), {
        let (_, h248) = v3.clone();
        h248
    })?;
    match m249 {
        None => {
            Ok(Err::<Adt91, Adt93>(Adt93::C0({
                let (_, h250) = v3;
                h250
            })))
        }
        Some(v4) => {
            let m252 = f32(v0.clone(), match v4.clone() {
                Adt95::C0(_, h251) => {
                    h251
                }
            })?;
            match m252 {
                None => {
                    Ok(Err::<Adt91, Adt93>(Adt93::C0(match v4 {
                        Adt95::C0(_, h253) => {
                            h253
                        }
                    })))
                }
                Some(v5) => {
                    f86(v0.clone(), Adt91::C0(f85(match v1 {
                        Adt95::C0(h254, _) => {
                            h254
                        }
                    })?, f85(match v2 {
                        Adt95::C0(h255, _) => {
                            h255
                        }
                    })?, {
                        let (h256, _) = v3;
                        h256
                    }, match v4.clone() {
                        Adt95::C0(h257, _) => {
                            h257
                        }
                    }, v5, List::<List<u64>>::nil(), Adt14::C0(0i64, Str::lit(""), Str::lit(""), f87())), f88(v0, {
                        let a259 = match v4 {
                            Adt95::C0(_, h258) => {
                                h258
                            }
                        };
                        let a260 = 4u64;
                        nat_add(a259, a260)?
                    })?)
                }
            }
        }
    }
}

pub fn f59(v0: Adt92) -> List<u64> {
    let a271 = f17({
        let a262 = match v0.clone() {
            Adt92::C0(h261, _) => {
                h261
            }
        };
        length_list(a262)
    });
    let a272 = {
        let a269 = match v0.clone() {
            Adt92::C0(h263, _) => {
                h263
            }
        };
        let a270 = {
            let a267 = f17({
                let a265 = match v0.clone() {
                    Adt92::C0(_, h264) => {
                        h264
                    }
                };
                length_list(a265)
            });
            let a268 = match v0 {
                Adt92::C0(_, h266) => {
                    h266
                }
            };
            append_list(a267, a268)
        };
        append_list(a269, a270)
    };
    append_list(a271, a272)
}

pub fn f60(v0: List<u64>) -> List<u64> {
    let a274 = f17({
        let a273 = v0.clone();
        length_list(a273)
    });
    let a275 = v0;
    append_list(a274, a275)
}

pub fn f61(v0: u64, v1: Adt6) -> R<Adt94> {
    let m276 = f89(v1)?;
    match m276 {
        Ok(v3) => {
            Ok(Adt94::C0(v0, f22(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f62(v0: Adt96) -> u64 {
    let m277 = v0;
    match m277 {
        Adt96::C0 => {
            f90()
        }
        Adt96::C1 => {
            f90()
        }
    }
}

pub fn f63(v0: Option<Adt98>, v1: Adt81) -> Adt6 {
    Adt6::C5(f91(v0, v1))
}

pub fn f64(v0: Adt96) -> Adt81 {
    Adt81::C0(f92(), f93(v0), None::<Adt6>)
}

pub fn f65(v0: List<u64>) -> bool {
    f94(v0, f95())
}

pub fn f66(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Result<Adt6, Adt99>) -> R<Adt94> {
    let m278 = v5;
    match m278 {
        Ok(v7) => {
            f98(v0, v1, v2, v3, v4, f99(v7)?)
        }
        Err(_) => {
            f61(f96(), f63(None::<Adt98>, f97(Adt100::C0)))
        }
    }
}

pub fn f67(v0: Fn0, v1: Adt91, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c279: Fn0 = v0;
    c279.apply(match v1 {
        Adt91::C0(_, _, _, _, _, _, h280) => {
            h280
        }
    }, v2)
}

pub fn f68(v0: Bytes) -> R<Result<Adt6, Adt99>> {
    f100(f85(v0)?)
}

pub fn f69() -> Adt94 {
    Adt94::C0(f101(), List::cons(Adt92::C0(f102(), f103()), List::<Adt92>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f70(v0: List<Adt92>, v1: List<u64>) -> R<Adt102> {
    let m281 = f104(v0, v1, Adt103::C0)?;
    match m281 {
        Adt103::C0 => {
            Ok(Adt102::C0)
        }
        Adt103::C1(v2) => {
            let m282 = f105(v2);
            match m282 {
                None => {
                    Ok(Adt102::C1)
                }
                Some(v3) => {
                    Ok(Adt102::C3(v3))
                }
            }
        }
        Adt103::C2 => {
            Ok(Adt102::C2)
        }
    }
}

pub fn f71() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f72(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt96) -> R<Option<Adt96>> {
    let m283 = v0;
    match m283 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f106(v3, v1)? {
                Ok(None::<Adt96>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f73(v0: List<u64>) -> Option<List<u64>> {
    let m284 = f107(v0);
    match m284 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f75(v1)
        }
    }
}

pub fn f74() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f75(v0: List<u64>) -> Option<List<u64>> {
    let m285 = v0.clone();
    match m285.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a286 = v1;
            let a287 = 91u64;
            if nat_eq(a286, a287) {
                f108(f109(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f108(f110(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f76(p293: List<Adt36>, p294: List<Str>) -> Option<Str> {
    let m292 = (p293, p294);
    {
        let mut m292 = m292;
        loop {
            let step = {
                let (v0, v1) = m292;
                let m288 = v0;
                match m288.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f111(match v2.clone() {
                            Adt36::C0(h289, _, _, _, _, _, _, _) => {
                                h289
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h290, _, _, _, _, _, _, _) => {
                                    h290
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h291, _, _, _, _, _, _, _) => {
                                    h291
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(p297: List<Adt12>, p298: List<Adt12>) -> List<Adt12> {
    let m296 = (p297, p298);
    {
        let mut m296 = m296;
        loop {
            let step = {
                let (v0, v1) = m296;
                let m295 = v0;
                match m295.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m296 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m301 = f112(match v0.clone() {
        Adt36::C0(h299, _, _, _, _, _, _, _) => {
            h299
        }
    }, f113(match v0.clone() {
        Adt36::C0(h300, _, _, _, _, _, _, _) => {
            h300
        }
    })?);
    match m301 {
        None => {
            f114(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f79(p304: u64, p305: i64) -> R<i64> {
    let m303 = (p304, p305);
    {
        let mut m303 = m303;
        loop {
            let step = {
                let (v0, v1) = m303;
                let m302 = v0;
                if m302 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m302 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f115(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Str, p311: List<Adt36>) -> Option<Adt36> {
    let m310 = p311;
    {
        let mut m310 = m310;
        loop {
            let step = {
                let v1 = m310;
                let m306 = v1;
                match m306.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a308 = match v2.clone() {
                            Adt36::C0(h307, _, _, _, _, _, _, _) => {
                                h307
                            }
                        };
                        let a309 = v0.clone();
                        if equal(a308, a309) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m318 = f116(Adt6::C5({
        let m314 = match v0.clone() {
            Adt36::C0(_, _, _, _, h313, _, _, _) => {
                h313
            }
        };
        match m314 {
            Adt5::C0(h312) => {
                h312
            }
        }
    }), Adt6::C5({
        let m317 = match v0.clone() {
            Adt36::C0(_, _, _, _, h316, _, _, _) => {
                h316
            }
        };
        match m317 {
            Adt5::C0(h315) => {
                h315
            }
        }
    }), f117(v2.clone()))?;
    match m318 {
        Adt106::C0 => {
            f118(v0, v1, v2)
        }
        Adt106::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f119(f120(v3, v4)?, true))))
        }
        Adt106::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt106::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt106::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f82(v0: List<Adt39>) -> Adt50 {
    f121(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f83(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f84(p321: List<Adt92>, p322: List<Adt92>) -> List<Adt92> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v0, v1) = m320;
                let m319 = v0;
                match m319.uncons() {
                    None => {
                        Err::<(List<Adt92>, List<Adt92>), List<Adt92>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt92>, List<Adt92>), List<Adt92>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m320 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Bytes) -> R<List<u64>> {
    f122(v0.clone(), {
        let a323 = v0;
        length_bytes(a323)
    }, List::<u64>::nil())
}

pub fn f86(v0: Bytes, v1: Adt91, v2: Result<(Adt14, u64), Adt93>) -> R<Result<Adt91, Adt93>> {
    let m324 = v2;
    match m324 {
        Ok(v4) => {
            f123(v0, Adt91::C0(match v1.clone() {
                Adt91::C0(h325, _, _, _, _, _, _) => {
                    h325
                }
            }, match v1.clone() {
                Adt91::C0(_, h326, _, _, _, _, _) => {
                    h326
                }
            }, match v1.clone() {
                Adt91::C0(_, _, h327, _, _, _, _) => {
                    h327
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, h328, _, _, _) => {
                    h328
                }
            }, match v1.clone() {
                Adt91::C0(_, _, _, _, h329, _, _) => {
                    h329
                }
            }, match v1 {
                Adt91::C0(_, _, _, _, _, h330, _) => {
                    h330
                }
            }, {
                let (h331, _) = v4;
                h331
            }), {
                let (_, h332) = v4;
                h332
            })
        }
        Err(v3) => {
            Ok(Err::<Adt91, Adt93>(v3))
        }
    }
}

pub fn f87() -> Bytes {
    Bytes::lit(&[])
}

pub fn f88(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt93>> {
    let m333 = f124(v0.clone(), v1)?;
    match m333 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m336 = f125(v0.clone(), {
                let a334 = v1;
                let a335 = 8u64;
                nat_add(a334, a335)?
            })?;
            match m336 {
                Ok(v4) => {
                    f126(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt93>(v3))
                }
            }
        }
    }
}

pub fn f89(v0: Adt6) -> R<Result<List<u64>, Adt99>> {
    f127(f128(), v0)
}

pub fn f90() -> u64 {
    403u64
}

pub fn f91(v0: Option<Adt98>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f129())), {
        let a337 = f130(v0);
        let a338 = List::cons((Str::lit("error"), f131(v1)), List::<(Str, Adt6)>::nil());
        append_list(a337, a338)
    })
}

pub fn f92() -> i64 {
    -32000i64
}

pub fn f93(v0: Adt96) -> Str {
    let m339 = v0;
    match m339 {
        Adt96::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt96::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f94(p346: List<u64>, p347: List<u64>) -> bool {
    let m345 = (p346, p347);
    {
        let mut m345 = m345;
        loop {
            let step = {
                let (v0, v1) = m345;
                let m340 = v0;
                match m340.uncons() {
                    None => {
                        let m341 = v1;
                        match m341.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m342 = v1;
                        match m342.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a343 = v4;
                                let a344 = v6;
                                if nat_eq(a343, a344) {
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
                Ok(next) => { m345 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f96() -> u64 {
    400u64
}

pub fn f97(v0: Adt100) -> Adt81 {
    Adt81::C0(f132(v0.clone()), f133(v0), None::<Adt6>)
}

pub fn f98(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Result<Adt104, Adt100>) -> R<Adt94> {
    let m348 = v5;
    match m348 {
        Ok(v7) => {
            f136(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f61(f134(v6.clone()), f63(f135(v6.clone()), f97(v6)))
        }
    }
}

pub fn f99(v0: Adt6) -> R<Result<Adt104, Adt100>> {
    let m349 = v0;
    match m349 {
        Adt6::C0 => {
            Ok(f138(None::<Adt98>, Adt101::C0))
        }
        Adt6::C1(_) => {
            Ok(f138(None::<Adt98>, Adt101::C0))
        }
        Adt6::C2(_) => {
            Ok(f138(None::<Adt98>, Adt101::C0))
        }
        Adt6::C3(_) => {
            Ok(f138(None::<Adt98>, Adt101::C0))
        }
        Adt6::C4(_) => {
            Ok(f138(None::<Adt98>, Adt101::C1))
        }
        Adt6::C5(v1) => {
            f137(Adt5::C0(v1))
        }
    }
}

pub fn f100(v0: List<u64>) -> R<Result<Adt6, Adt99>> {
    let m350 = f139(v0.clone())?;
    match m350 {
        Adt117::C0 => {
            f140(f141(), 0u64, v0)
        }
        Adt117::C1(v1) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C0(v1)))
        }
        Adt117::C2(v2) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C1(v2)))
        }
    }
}

pub fn f101() -> u64 {
    405u64
}

pub fn f102() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f103() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f104(p355: List<Adt92>, v1: List<u64>, p356: Adt103) -> R<Adt103> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v2) = m354;
                let m351 = v0;
                match m351.uncons() {
                    None => {
                        Err::<(List<Adt92>, Adt103), R<Adt103>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f142(match v3.clone() {
                            Adt92::C0(h352, _) => {
                                h352
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt92>, Adt103), R<Adt103>>((v4, f143(v2, match v3 {
                                Adt92::C0(_, h353) => {
                                    h353
                                }
                            })))
                        } else {
                            Ok::<(List<Adt92>, Adt103), R<Adt103>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f144(v0);
    if f145(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f106(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f142(v0.clone(), f146())? {
        Ok(true)
    } else {
        if f142(v0.clone(), f147())? {
            Ok(true)
        } else {
            if f142(v0.clone(), f148())? {
                Ok(true)
            } else {
                f149(v0, v1)
            }
        }
    }
}

pub fn f107(p361: List<u64>) -> Option<List<u64>> {
    let m360 = p361;
    {
        let mut m360 = m360;
        loop {
            let step = {
                let v0 = m360;
                let m357 = v0;
                match m357.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a358 = v1;
                        let a359 = 58u64;
                        if nat_eq(a358, a359) {
                            Err::<List<u64>, Option<List<u64>>>(f150(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m360 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m362 = v0;
    match m362 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f151({
                let (_, h363) = v1;
                h363
            }) {
                Some({
                    let (h364, _) = v1;
                    h364
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f109(p369: List<u64>, p370: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m368 = (p369, p370);
    {
        let mut m368 = m368;
        loop {
            let step = {
                let (v0, v1) = m368;
                let m365 = v0;
                match m365.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a366 = v2;
                        let a367 = 93u64;
                        if nat_eq(a366, a367) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f152(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(p375: List<u64>, p376: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m374 = (p375, p376);
    {
        let mut m374 = m374;
        loop {
            let step = {
                let (v0, v1) = m374;
                let m371 = v0;
                match m371.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f152(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a372 = v2;
                        let a373 = 58u64;
                        if nat_eq(a372, a373) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f152(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m374 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Str, p381: List<Str>) -> bool {
    let m380 = p381;
    {
        let mut m380 = m380;
        loop {
            let step = {
                let v1 = m380;
                let m377 = v1;
                match m377.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a378 = v0.clone();
                        let a379 = v2;
                        if equal(a378, a379) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Str, v1: List<u64>) -> Option<Adt97> {
    if f153(v1) {
        Some(Adt97::C0(v0))
    } else {
        None::<Adt97>
    }
}

pub fn f113(v0: Str) -> R<List<u64>> {
    f85({
        let a382 = v0;
        utf8_encode(a382)
    })
}

pub fn f114(v0: Adt36) -> R<Result<List<Adt12>, Adt97>> {
    let m383 = f154(v0.clone());
    match m383 {
        None => {
            f155(v0.clone(), f156(f157(), match v0.clone() {
                Adt36::C0(h384, _, _, _, _, _, _, _) => {
                    h384
                }
            }, Adt6::C5({
                let m387 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h386, _, _, _) => {
                        h386
                    }
                };
                match m387 {
                    Adt5::C0(h385) => {
                        h385
                    }
                }
            }), List::cons(Adt105::C0(Adt6::C5({
                let m390 = match v0 {
                    Adt36::C0(_, _, _, _, h389, _, _, _) => {
                        h389
                    }
                };
                match m390 {
                    Adt5::C0(h388) => {
                        h388
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt105>::nil()), f158())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt97>(v1))
        }
    }
}

pub fn f115(v0: i64, v1: i64) -> R<i64> {
    let a392 = v0;
    let a393 = {
        let a391 = v1;
        int_neg(a391)?
    };
    int_sub(a392, a393)
}

pub fn f116(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt106> {
    f159(v0, f160(), Adt108::C0, List::cons(Adt109::C0(v1, v2, List::<Adt107>::nil()), List::<Adt109>::nil()))
}

pub fn f117(v0: Adt22) -> Adt6 {
    let m395 = match v0 {
        Adt22::C0(_, _, h394, _, _) => {
            h394
        }
    };
    match m395 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h396) => {
                    h396
                }
            })
        }
    }
}

pub fn f118(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m398 = {
        let c397: Fn1 = v1;
        c397.apply(v2)
    };
    match m398 {
        Ok(v4) => {
            f161(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f119(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f120(v0: List<Adt107>, v1: Str) -> R<Str> {
    let v2: Str = f162(v0)?;
    let a401 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a399 = v2.clone();
        let a400 = Str::lit("");
        if equal(a399, a400) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a402 = Str::lit(" ");
    Ok(join(a401, a402))
}

pub fn f121(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f122(v0: Bytes, p405: u64, p406: List<u64>) -> R<List<u64>> {
    let m404 = (p405, p406);
    {
        let mut m404 = m404;
        loop {
            let step = {
                let (v1, v2) = m404;
                let m403 = v1;
                if m403 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m403 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f56(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f123(v0: Bytes, v1: Adt91, v2: u64) -> R<Result<Adt91, Adt93>> {
    let a408 = v2;
    let a409 = {
        let a407 = v0.clone();
        length_bytes(a407)
    };
    if nat_eq(a408, a409) {
        Ok(Ok::<Adt91, Adt93>(v1))
    } else {
        let m410 = f32(v0.clone(), v2)?;
        match m410 {
            None => {
                Ok(Err::<Adt91, Adt93>(Adt93::C0(v2)))
            }
            Some(v3) => {
                let m413 = f163(v0.clone(), {
                    let a411 = v2;
                    let a412 = 4u64;
                    nat_add(a411, a412)?
                }, v3, List::<List<u64>>::nil())?;
                match m413 {
                    Ok(v5) => {
                        let a416 = {
                            let (_, h414) = v5.clone();
                            h414
                        };
                        let a417 = {
                            let a415 = v0;
                            length_bytes(a415)
                        };
                        if nat_eq(a416, a417) {
                            Ok(Ok::<Adt91, Adt93>(f164(v1, {
                                let (h418, _) = v5;
                                h418
                            })))
                        } else {
                            Ok(Err::<Adt91, Adt93>(Adt93::C1({
                                let (_, h419) = v5;
                                h419
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt91, Adt93>(v4))
                    }
                }
            }
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a423 = {
        let a420 = v1;
        let a421 = 8u64;
        nat_add(a420, a421)?
    };
    let a424 = {
        let a422 = v0.clone();
        length_bytes(a422)
    };
    if nat_le(a423, a424) {
        Ok(Some(f165(f165(f165(f165(f165(f165(f165(f165(0i64, f56(v0.clone(), v1)?)?, f56(v0.clone(), {
            let a425 = v1;
            let a426 = 1u64;
            nat_add(a425, a426)?
        })?)?, f56(v0.clone(), {
            let a427 = v1;
            let a428 = 2u64;
            nat_add(a427, a428)?
        })?)?, f56(v0.clone(), {
            let a429 = v1;
            let a430 = 3u64;
            nat_add(a429, a430)?
        })?)?, f56(v0.clone(), {
            let a431 = v1;
            let a432 = 4u64;
            nat_add(a431, a432)?
        })?)?, f56(v0.clone(), {
            let a433 = v1;
            let a434 = 5u64;
            nat_add(a433, a434)?
        })?)?, f56(v0.clone(), {
            let a435 = v1;
            let a436 = 6u64;
            nat_add(a435, a436)?
        })?)?, f56(v0, {
            let a437 = v1;
            let a438 = 7u64;
            nat_add(a437, a438)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt93>> {
    let m439 = f15(v0, v1)?;
    match m439 {
        None => {
            Ok(Err::<(Str, u64), Adt93>(Adt93::C0(v1)))
        }
        Some(v2) => {
            let m441 = f166(f85(match v2.clone() {
                Adt95::C0(h440, _) => {
                    h440
                }
            })?);
            match m441 {
                None => {
                    Ok(Err::<(Str, u64), Adt93>(Adt93::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt93>((v3, match v2 {
                        Adt95::C0(_, h442) => {
                            h442
                        }
                    })))
                }
            }
        }
    }
}

pub fn f126(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m444 = f125(v0.clone(), {
        let (_, h443) = v2;
        h443
    })?;
    match m444 {
        Ok(v4) => {
            f167(v0, v1, {
                let (h445, _) = v2;
                h445
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt93>(v3))
        }
    }
}

pub fn f127(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt99>> {
    let m446 = f168(v0, List::cons(Adt111::C1(0u64, v1), List::<Adt111>::nil()), List::<u64>::nil())?;
    match m446 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt99>(f169(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt99>(v3))
        }
    }
}

pub fn f128() -> u64 {
    67108864u64
}

pub fn f129() -> Str {
    Str::lit("2.0")
}

pub fn f130(v0: Option<Adt98>) -> List<(Str, Adt6)> {
    let m447 = v0;
    match m447 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f170(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f131(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h448, _, _) => {
            h448
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h449, _) => {
            h449
        }
    })), f171(match v0 {
        Adt81::C0(_, _, h450) => {
            h450
        }
    }))))
}

pub fn f132(v0: Adt100) -> i64 {
    let m451 = v0;
    match m451 {
        Adt100::C0 => {
            f172()
        }
        Adt100::C1(_, _) => {
            f173()
        }
    }
}

pub fn f133(v0: Adt100) -> Str {
    let m452 = v0;
    match m452 {
        Adt100::C0 => {
            Str::lit("Parse error")
        }
        Adt100::C1(_, v2) => {
            f174(v2)
        }
    }
}

pub fn f134(v0: Adt100) -> u64 {
    let m453 = v0;
    match m453 {
        Adt100::C0 => {
            f96()
        }
        Adt100::C1(_, _) => {
            f96()
        }
    }
}

pub fn f135(v0: Adt100) -> Option<Adt98> {
    let m454 = v0;
    match m454 {
        Adt100::C0 => {
            None::<Adt98>
        }
        Adt100::C1(v1, _) => {
            v1
        }
    }
}

pub fn f136(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt92>, v5: Adt104) -> R<Adt94> {
    f175(v0, f176(v4.clone())?, f177(v1.clone(), v2, v3, Some(f178(v4, match v1 {
        Adt0::C0(_, _, _, _, h455, _) => {
            h455
        }
    }, f179(v5.clone()), match v5.clone() {
        Adt104::C0(_, h456, _) => {
            h456
        }
    })?), f180(), v5)?)
}

pub fn f137(v0: Adt5) -> R<Result<Adt104, Adt100>> {
    let m457 = f181(v0.clone())?;
    match m457 {
        Adt134::C0 => {
            let m458 = f182(Str::lit("method"), v0.clone());
            match m458 {
                Adt135::C0 => {
                    Ok(f138(None::<Adt98>, f183(v0)))
                }
                Adt135::C1(_) => {
                    Ok(f138(None::<Adt98>, if f184(v0) {
                        Adt101::C8
                    } else {
                        Adt101::C7
                    }))
                }
            }
        }
        Adt134::C1(v2) => {
            Ok(f185(v2, v0))
        }
        Adt134::C2 => {
            Ok(f138(None::<Adt98>, Adt101::C5))
        }
    }
}

pub fn f138(v0: Option<Adt98>, v1: Adt101) -> Result<Adt104, Adt100> {
    Err::<Adt104, Adt100>(Adt100::C1(v0, v1))
}

pub fn f139(v0: List<u64>) -> R<Adt117> {
    Ok(f186({
        let a459 = v0.clone();
        length_list(a459)
    }, f187(Adt118::C0, 0u64, 0u64, v0)?))
}

pub fn f140(p466: Adt119, p467: u64, p468: List<u64>) -> R<Result<Adt6, Adt99>> {
    let m465 = (p466, (p467, p468));
    {
        let mut m465 = m465;
        loop {
            let step = {
                let (v0, (v1, v2)) = m465;
                let m460 = v2;
                match m460.uncons() {
                    None => {
                        let m461 = f188(v0, v1)?;
                        match m461 {
                            Ok(v3) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Ok::<Adt6, Adt99>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Err::<Adt6, Adt99>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m462 = f189(v0, v1, v5)?;
                        match m462 {
                            Adt136::C0(v7) => {
                                Err::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>(Ok(Err::<Adt6, Adt99>(v7)))
                            }
                            Adt136::C1(v8) => {
                                Ok::<(Adt119, (u64, List<u64>)), R<Result<Adt6, Adt99>>>((v8, ({
                                    let a463 = v1;
                                    let a464 = 1u64;
                                    nat_add(a463, a464)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m465 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141() -> Adt119 {
    Adt119::C0(Adt120::C0, List::<Adt124>::nil(), 0u64)
}

pub fn f142(p475: List<u64>, p476: List<u64>) -> R<bool> {
    let m474 = (p475, p476);
    {
        let mut m474 = m474;
        loop {
            let step = {
                let (v0, v1) = m474;
                let m469 = v0;
                match m469.uncons() {
                    None => {
                        let m470 = v1;
                        match m470.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m471 = v1;
                        match m471.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a472 = f190(v4)?;
                                let a473 = f190(v6)?;
                                if nat_eq(a472, a473) {
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
                Ok(next) => { m474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Adt103, v1: List<u64>) -> Adt103 {
    let m477 = v0;
    match m477 {
        Adt103::C0 => {
            Adt103::C1(v1)
        }
        Adt103::C1(_) => {
            Adt103::C2
        }
        Adt103::C2 => {
            Adt103::C2
        }
    }
}

pub fn f144(v0: List<u64>) -> List<u64> {
    f152(f191(f152(f191(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f145(p480: List<u64>) -> bool {
    let m479 = p480;
    {
        let mut m479 = m479;
        loop {
            let step = {
                let v0 = m479;
                let m478 = v0;
                match m478.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f192(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m479 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f147() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f148() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f149(v0: List<u64>, p483: List<List<u64>>) -> R<bool> {
    let m482 = p483;
    {
        let mut m482 = m482;
        loop {
            let step = {
                let v1 = m482;
                let m481 = v1;
                match m481.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f142(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: List<u64>) -> Option<List<u64>> {
    let m484 = v0;
    match m484.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m485 = v2;
            match m485.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a486 = v1;
                    let a487 = 47u64;
                    if if nat_eq(a486, a487) {
                        let a488 = v3;
                        let a489 = 47u64;
                        nat_eq(a488, a489)
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

pub fn f151(v0: List<u64>) -> bool {
    let m490 = v0;
    match m490.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a491 = v1;
            let a492 = 58u64;
            if nat_eq(a491, a492) {
                let m493 = v2.clone();
                match m493.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f193(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f152(p496: List<u64>, p497: List<u64>) -> List<u64> {
    let m495 = (p496, p497);
    {
        let mut m495 = m495;
        loop {
            let step = {
                let (v0, v1) = m495;
                let m494 = v0;
                match m494.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: List<u64>) -> bool {
    let m498 = v0.clone();
    match m498.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a502 = if f194(v0.clone(), false) {
                let a500 = {
                    let a499 = v0;
                    length_list(a499)
                };
                let a501 = f195();
                nat_le(a500, a501)
            } else {
                false
            };
            bool_not(a502)
        }
    }
}

pub fn f154(v0: Adt36) -> Option<Adt97> {
    let m507 = f196(match v0.clone() {
        Adt36::C0(h503, _, _, _, _, _, _, _) => {
            h503
        }
    }, {
        let m506 = match v0.clone() {
            Adt36::C0(_, _, _, _, h505, _, _, _) => {
                h505
            }
        };
        match m506 {
            Adt5::C0(h504) => {
                h504
            }
        }
    });
    match m507 {
        None => {
            f197(match v0.clone() {
                Adt36::C0(h508, _, _, _, _, _, _, _) => {
                    h508
                }
            }, {
                let m511 = match v0 {
                    Adt36::C0(_, _, _, _, h510, _, _, _) => {
                        h510
                    }
                };
                match m511 {
                    Adt5::C0(h509) => {
                        h509
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f155(v0: Adt36, v1: Result<Adt125, Adt97>) -> R<Result<List<Adt12>, Adt97>> {
    let m512 = v1;
    match m512 {
        Ok(v3) => {
            let m513 = f198(v0)?;
            match m513 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt97>(f77(match v3 {
                        Adt125::C0(h514, _) => {
                            h514
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt97>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt97>(v2))
        }
    }
}

pub fn f156(p521: u64, v1: Str, v2: Adt6, p522: List<Adt105>, p523: Adt125) -> R<Result<Adt125, Adt97>> {
    let m520 = (p521, (p522, p523));
    {
        let mut m520 = m520;
        loop {
            let step = {
                let (v0, (v3, v4)) = m520;
                let m515 = v0;
                if m515 == 0 {
                    Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Err::<Adt125, Adt97>(Adt97::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m515 - 1;
                    let m516 = v3;
                    match m516.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Ok::<Adt125, Adt97>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m517 = f199(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m517 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>((v5, ({
                                        let (h518, _) = v9;
                                        h518
                                    }, {
                                        let (_, h519) = v9;
                                        h519
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt105>, Adt125)), R<Result<Adt125, Adt97>>>(Ok(Err::<Adt125, Adt97>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157() -> u64 {
    67108864u64
}

pub fn f158() -> Adt125 {
    Adt125::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f159(v0: Adt6, p542: u64, p543: Adt108, p544: List<Adt109>) -> R<Adt106> {
    let m541 = (p542, (p543, p544));
    {
        let mut m541 = m541;
        loop {
            let step = {
                let (v1, (v2, v3)) = m541;
                let m524 = v1;
                if m524 == 0 {
                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C4))
                } else {
                    let v4 = m524 - 1;
                    let m525 = v3;
                    match m525.uncons() {
                        None => {
                            let m526 = v2;
                            match m526 {
                                Adt108::C0 => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C0))
                                }
                                Adt108::C1(v5, v6) => {
                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m527 = v2.clone();
                            match m527 {
                                Adt108::C0 => {
                                    let m528 = v7.clone();
                                    match m528 {
                                        Adt109::C0(v9, v10, v11) => {
                                            let m529 = f200(v0.clone(), v9, v10, v11.clone())?;
                                            match m529 {
                                                Adt137::C0(v12) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v11, v12), v8)))
                                                }
                                                Adt137::C1(v13) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v13)))
                                                }
                                                Adt137::C2(v14) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v14)))
                                                }
                                                Adt137::C3(v15) => {
                                                    let v16: List<Adt109> = {
                                                        let a530 = v15;
                                                        let a531 = v8;
                                                        append_list(a530, a531)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt109::C1(_, _, _) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v8)))
                                        }
                                        Adt109::C2(_, _, _, _) => {
                                            let v24: Adt126 = f201(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v24.clone() {
                                                Adt126::C0(h532, _) => {
                                                    h532
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h533) => {
                                                    h533
                                                }
                                            })))
                                        }
                                        Adt109::C3(v25) => {
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt109::C4(_, _, _, _) => {
                                            let v39: Adt126 = f201(v7, v8)?;
                                            Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v39.clone() {
                                                Adt126::C0(h537, _) => {
                                                    h537
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h538) => {
                                                    h538
                                                }
                                            })))
                                        }
                                        Adt109::C5(v26, v27, v28, v29) => {
                                            let m534 = f202(v26, v27, v28, v29.clone());
                                            match m534 {
                                                Adt137::C0(v30) => {
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (Adt108::C1(v29, v30), v8)))
                                                }
                                                Adt137::C1(v31) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C2(v31)))
                                                }
                                                Adt137::C2(v32) => {
                                                    Err::<(u64, (Adt108, List<Adt109>)), R<Adt106>>(Ok(Adt106::C3(v32)))
                                                }
                                                Adt137::C3(v33) => {
                                                    let v34: List<Adt109> = {
                                                        let a535 = v33;
                                                        let a536 = v8;
                                                        append_list(a535, a536)
                                                    };
                                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt108::C1(v40, v41) => {
                                    let v42: Adt126 = f203(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt108, List<Adt109>)), R<Adt106>>((v4, (match v42.clone() {
                                        Adt126::C0(h539, _) => {
                                            h539
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h540) => {
                                            h540
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160() -> u64 {
    67108864u64
}

pub fn f161(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f204({
        let m547 = match v1.clone() {
            Adt50::C0(h546, _) => {
                h546
            }
        };
        match m547 {
            Adt38::C0(h545, _, _) => {
                h545
            }
        }
    })? {
        f205(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f162(v0: List<Adt107>) -> R<Str> {
    let a548 = List::cons(Str::lit(""), f206(v0, List::<Str>::nil())?);
    let a549 = Str::lit("/");
    Ok(join(a548, a549))
}

pub fn f163(v0: Bytes, p555: u64, p556: u64, p557: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt93>> {
    let m554 = (p555, (p556, p557));
    {
        let mut m554 = m554;
        loop {
            let step = {
                let (v1, (v2, v3)) = m554;
                let m550 = v2;
                if m550 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Ok::<(List<List<u64>>, u64), Adt93>((f37(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m550 - 1;
                    let m551 = f15(v0.clone(), v1)?;
                    match m551 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>(Ok(Err::<(List<List<u64>>, u64), Adt93>(Adt93::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt93>>>((match v5.clone() {
                                Adt95::C0(_, h552) => {
                                    h552
                                }
                            }, (v4, List::cons(f85(match v5 {
                                Adt95::C0(h553, _) => {
                                    h553
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt91, v1: List<List<u64>>) -> Adt91 {
    Adt91::C0(match v0.clone() {
        Adt91::C0(h558, _, _, _, _, _, _) => {
            h558
        }
    }, match v0.clone() {
        Adt91::C0(_, h559, _, _, _, _, _) => {
            h559
        }
    }, match v0.clone() {
        Adt91::C0(_, _, h560, _, _, _, _) => {
            h560
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, h561, _, _, _) => {
            h561
        }
    }, match v0.clone() {
        Adt91::C0(_, _, _, _, h562, _, _) => {
            h562
        }
    }, v1, match v0 {
        Adt91::C0(_, _, _, _, _, _, h563) => {
            h563
        }
    })
}

pub fn f165(v0: i64, v1: u64) -> R<i64> {
    f115({
        let a564 = v0;
        let a565 = 256i64;
        int_mul(a564, a565)?
    }, f79(v1, 0i64)?)
}

pub fn f166(v0: List<u64>) -> Option<Str> {
    let a566 = f207(f87(), v0);
    utf8_decode(a566)
}

pub fn f167(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt93>> {
    let m568 = f15(v0, {
        let (_, h567) = v3.clone();
        h567
    })?;
    match m568 {
        None => {
            Ok(Err::<(Adt14, u64), Adt93>(Adt93::C0({
                let (_, h569) = v3;
                h569
            })))
        }
        Some(v4) => {
            Ok(f208(v1, v2, {
                let (h570, _) = v3;
                h570
            }, v4))
        }
    }
}

pub fn f168(p575: u64, p576: List<Adt111>, p577: List<u64>) -> R<Result<List<u64>, Adt99>> {
    let m574 = (p575, (p576, p577));
    {
        let mut m574 = m574;
        loop {
            let step = {
                let (v0, (v1, v2)) = m574;
                let m571 = v0;
                if m571 == 0 {
                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Err::<List<u64>, Adt99>(Adt99::C13)))
                } else {
                    let v3 = m571 - 1;
                    let m572 = v1;
                    match m572.uncons() {
                        None => {
                            Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Ok::<List<u64>, Adt99>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m573 = f209(v4, v5, v2)?;
                            match m573 {
                                Adt138::C0(v6) => {
                                    Err::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>(Ok(Err::<List<u64>, Adt99>(v6)))
                                }
                                Adt138::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt111>, List<u64>)), R<Result<List<u64>, Adt99>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(p580: List<u64>, p581: List<u64>) -> List<u64> {
    let m579 = (p580, p581);
    {
        let mut m579 = m579;
        loop {
            let step = {
                let (v0, v1) = m579;
                let m578 = v0;
                match m578.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f170(v0: Adt98) -> Adt6 {
    let m582 = v0;
    match m582 {
        Adt98::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt98::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f171(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m583 = v0;
    match m583 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f172() -> i64 {
    -32700i64
}

pub fn f173() -> i64 {
    -32600i64
}

pub fn f174(v0: Adt101) -> Str {
    let m584 = v0;
    match m584 {
        Adt101::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt101::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt101::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt101::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt101::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt101::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt101::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt101::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt101::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f175(v0: Adt14, v1: bool, v2: Adt127) -> R<Adt94> {
    let m586 = match v2.clone() {
        Adt127::C0(_, h585) => {
            h585
        }
    };
    match m586 {
        Ok(v3) => {
            let m587 = f210(v3.clone())?;
            match m587 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f211(v0, f212(match v3.clone() {
                        Adt128::C0(h588, _, _) => {
                            h588
                        }
                    }), f213(v1, f61(f214(), f215(match v2 {
                        Adt127::C0(h589, _) => {
                            h589
                        }
                    }, v4))?, f216(f217(match v3.clone() {
                        Adt128::C0(_, _, h590) => {
                            h590
                        }
                    }, List::<List<u64>>::nil())?, f218(f212(match v3 {
                        Adt128::C0(h591, _, _) => {
                            h591
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f61(f219(v5.clone()), f63(Some(match v2 {
                Adt127::C0(h592, _) => {
                    h592
                }
            }), f220(v5)?))
        }
    }
}

pub fn f176(p597: List<Adt92>) -> R<bool> {
    let m596 = p597;
    {
        let mut m596 = m596;
        loop {
            let step = {
                let v0 = m596;
                let m593 = v0;
                match m593.uncons() {
                    None => {
                        Err::<List<Adt92>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f142(match v1.clone() {
                            Adt92::C0(h594, _) => {
                                h594
                            }
                        }, f221())? {
                            if f222(f223(44u64, match v1 {
                                Adt92::C0(_, h595) => {
                                    h595
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt92>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt92>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt92>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt112>, v4: List<Str>, v5: Adt104) -> R<Adt127> {
    Ok(Adt127::C0(match v5.clone() {
        Adt104::C0(h598, _, _) => {
            h598
        }
    }, f224(v0, v1, v2, f225(v3, v4, v5)?)?))
}

pub fn f178(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt112> {
    let v4: Adt133 = f226(Adt114::C0, f70(v0.clone(), f227())?);
    let v5: Adt133 = f226(Adt114::C1, f70(v0.clone(), f228())?);
    let v6: Adt133 = if f229(v3) {
        f226(Adt114::C2, f230(f70(v0.clone(), f231())?)?)
    } else {
        f232()
    };
    Ok(Adt112::C0(match v4.clone() {
        Adt133::C0(h599, _) => {
            h599
        }
    }, match v5.clone() {
        Adt133::C0(h600, _) => {
            h600
        }
    }, match v6.clone() {
        Adt133::C0(h601, _) => {
            h601
        }
    }, f233(match v4 {
        Adt133::C0(_, h602) => {
            h602
        }
    }, f233(match v5 {
        Adt133::C0(_, h603) => {
            h603
        }
    }, match v6 {
        Adt133::C0(_, h604) => {
            h604
        }
    })), f234(v0, v1, v2)?))
}

pub fn f179(v0: Adt104) -> Option<Str> {
    let a606 = match v0.clone() {
        Adt104::C0(_, h605, _) => {
            h605
        }
    };
    let a607 = Str::lit("tools/call");
    if equal(a606, a607) {
        let m609 = match v0 {
            Adt104::C0(_, _, h608) => {
                h608
            }
        };
        match m609 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m610 = f182(Str::lit("name"), v1);
                match m610 {
                    Adt135::C0 => {
                        None::<Str>
                    }
                    Adt135::C1(v2) => {
                        f235(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f180() -> List<Str> {
    List::cons(f236(), List::<Str>::nil())
}

pub fn f181(v0: Adt5) -> R<Adt134> {
    let m611 = f182(Str::lit("id"), v0);
    match m611 {
        Adt135::C0 => {
            Ok(Adt134::C0)
        }
        Adt135::C1(v1) => {
            let m612 = v1;
            match m612 {
                Adt6::C0 => {
                    Ok(Adt134::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C2(v3) => {
                    let m613 = f237(v3)?;
                    match m613 {
                        None => {
                            Ok(Adt134::C2)
                        }
                        Some(v4) => {
                            Ok(Adt134::C1(Adt98::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt134::C1(Adt98::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt134::C2)
                }
            }
        }
    }
}

pub fn f182(v0: Str, v1: Adt5) -> Adt135 {
    f238(v0, match v1 {
        Adt5::C0(h614) => {
            h614
        }
    })
}

pub fn f183(v0: Adt5) -> Adt101 {
    if f184(v0) {
        Adt101::C8
    } else {
        Adt101::C3
    }
}

pub fn f184(v0: Adt5) -> bool {
    if f239(Str::lit("result"), v0.clone()) {
        true
    } else {
        f239(Str::lit("error"), v0)
    }
}

pub fn f185(v0: Adt98, v1: Adt5) -> Result<Adt104, Adt100> {
    let m615 = f182(Str::lit("jsonrpc"), v1.clone());
    match m615 {
        Adt135::C0 => {
            f138(Some(v0), Adt101::C2)
        }
        Adt135::C1(v2) => {
            let m616 = f235(v2);
            match m616 {
                None => {
                    f138(Some(v0), Adt101::C2)
                }
                Some(v3) => {
                    let a617 = v3;
                    let a618 = f129();
                    if equal(a617, a618) {
                        f240(v0, v1)
                    } else {
                        f138(Some(v0), Adt101::C2)
                    }
                }
            }
        }
    }
}

pub fn f186(v0: u64, v1: u64) -> Adt117 {
    let a619 = v1;
    let a620 = 0u64;
    if nat_eq(a619, a620) {
        Adt117::C0
    } else {
        let a621 = v1;
        let a622 = 2u64;
        if nat_eq(a621, a622) {
            Adt117::C2(v0)
        } else {
            Adt117::C1({
                let a626 = v0;
                let a627 = {
                    let a623 = v1;
                    let a624 = 3u64;
                    let a625 = 0u64;
                    nat_quot(a623, a624, a625)
                };
                nat_sub(a626, a627)
            })
        }
    }
}

pub fn f187(p655: Adt118, p656: u64, p657: u64, p658: List<u64>) -> R<u64> {
    let m654 = (p655, (p656, (p657, p658)));
    {
        let mut m654 = m654;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m654;
                let m628 = v3;
                match m628.uncons() {
                    None => {
                        let m629 = v0;
                        match m629 {
                            Adt118::C0 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt118::C1 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C2 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt118::C3 => {
                                Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m630 = v0;
                        match m630 {
                            Adt118::C0 => {
                                let a631 = v4;
                                let a632 = 128u64;
                                if nat_lt(a631, a632) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a633 = v4;
                                    let a634 = 194u64;
                                    if nat_lt(a633, a634) {
                                        let v6: u64 = f241({
                                            let a635 = List::cons(v4, v5);
                                            length_list(a635)
                                        })?;
                                        Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a636 = v4;
                                        let a637 = 224u64;
                                        if nat_lt(a636, a637) {
                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a638 = v4;
                                            let a639 = 224u64;
                                            if nat_eq(a638, a639) {
                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a640 = v4;
                                                let a641 = 237u64;
                                                if nat_eq(a640, a641) {
                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a642 = v4;
                                                    let a643 = 240u64;
                                                    if nat_lt(a642, a643) {
                                                        Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a644 = v4;
                                                        let a645 = 240u64;
                                                        if nat_eq(a644, a645) {
                                                            Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a646 = v4;
                                                            let a647 = 244u64;
                                                            if nat_eq(a646, a647) {
                                                                Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a648 = v4;
                                                                let a649 = 244u64;
                                                                if nat_lt(a648, a649) {
                                                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f241({
                                                                        let a650 = List::cons(v4, v5);
                                                                        length_list(a650)
                                                                    })?;
                                                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt118::C1 => {
                                if f242(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f241({
                                        let a651 = List::cons(v4, v5);
                                        length_list(a651)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt118::C2 => {
                                if f242(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f241({
                                        let a652 = List::cons(v4, v5);
                                        length_list(a652)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt118::C3 => {
                                if f242(v4, v1, v2) {
                                    Ok::<(Adt118, (u64, (u64, List<u64>))), R<u64>>((Adt118::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f241({
                                        let a653 = List::cons(v4, v5);
                                        length_list(a653)
                                    })?;
                                    Err::<(Adt118, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m654 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Adt119, v1: u64) -> R<Result<Adt6, Adt99>> {
    let m660 = match v0.clone() {
        Adt119::C0(h659, _, _) => {
            h659
        }
    };
    match m660 {
        Adt120::C0 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C1 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C2 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C3 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C4 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C5 => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C7(_, _) => {
            Ok(Err::<Adt6, Adt99>(Adt99::C3))
        }
        Adt120::C8(v3) => {
            let m661 = f243(v3)?;
            match m661 {
                Adt141::C0 => {
                    Ok(Err::<Adt6, Adt99>(Adt99::C7(v1)))
                }
                Adt141::C1 => {
                    Ok(Err::<Adt6, Adt99>(Adt99::C8(v1)))
                }
                Adt141::C2(v4) => {
                    let m666 = {
                        let m665 = f244(v4, match v0.clone() {
                            Adt119::C0(_, h663, _) => {
                                h663
                            }
                        }, match v0 {
                            Adt119::C0(_, _, h664) => {
                                h664
                            }
                        });
                        match m665 {
                            Adt119::C0(h662, _, _) => {
                                h662
                            }
                        }
                    };
                    match m666 {
                        Adt120::C0 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C1 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C2 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C3 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C4 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C5 => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C7(_, _) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C8(_) => {
                            Ok(Err::<Adt6, Adt99>(Adt99::C3))
                        }
                        Adt120::C9(v5) => {
                            Ok(Ok::<Adt6, Adt99>(v5))
                        }
                    }
                }
            }
        }
        Adt120::C9(v2) => {
            Ok(Ok::<Adt6, Adt99>(v2))
        }
    }
}

pub fn f189(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let m668 = match v0.clone() {
        Adt119::C0(h667, _, _) => {
            h667
        }
    };
    match m668 {
        Adt120::C0 => {
            if f245(v2) {
                Ok(Adt136::C1(v0))
            } else {
                f246(v0, v1, v2)
            }
        }
        Adt120::C1 => {
            if f245(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a669 = v2;
                let a670 = 93u64;
                if nat_eq(a669, a670) {
                    Ok(f247(v0, v1))
                } else {
                    f246(v0, v1, v2)
                }
            }
        }
        Adt120::C2 => {
            Ok(f248(v0, v1, v2))
        }
        Adt120::C3 => {
            if f245(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a671 = v2;
                let a672 = 125u64;
                if nat_eq(a671, a672) {
                    Ok(f249(v0, v1))
                } else {
                    let a673 = v2;
                    let a674 = 34u64;
                    if nat_eq(a673, a674) {
                        Ok(f250(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f251(Adt99::C2(v1)))
                    }
                }
            }
        }
        Adt120::C4 => {
            if f245(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a675 = v2;
                let a676 = 34u64;
                if nat_eq(a675, a676) {
                    Ok(f250(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f251(Adt99::C2(v1)))
                }
            }
        }
        Adt120::C5 => {
            if f245(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a677 = v2;
                let a678 = 58u64;
                if nat_eq(a677, a678) {
                    Ok(f250(v0, Adt120::C0))
                } else {
                    Ok(f251(Adt99::C2(v1)))
                }
            }
        }
        Adt120::C6(v3, v4, v5) => {
            f252(v0, v1, v2, v3, v4, v5)
        }
        Adt120::C7(v6, v7) => {
            f253(v0, v1, v2, v6, v7)
        }
        Adt120::C8(v8) => {
            f254(v0, v1, v2, v8)
        }
        Adt120::C9(_) => {
            Ok(f248(v0, v1, v2))
        }
    }
}

pub fn f190(v0: u64) -> R<u64> {
    let a679 = 64u64;
    let a680 = v0;
    if if nat_lt(a679, a680) {
        let a681 = v0;
        let a682 = 91u64;
        nat_lt(a681, a682)
    } else {
        false
    } {
        let a683 = v0;
        let a684 = 32u64;
        nat_add(a683, a684)
    } else {
        Ok(v0)
    }
}

pub fn f191(p687: List<u64>) -> List<u64> {
    let m686 = p687;
    {
        let mut m686 = m686;
        loop {
            let step = {
                let v0 = m686;
                let m685 = v0;
                match m685.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f255(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m686 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: u64) -> bool {
    let a688 = v0;
    let a689 = 9u64;
    if nat_eq(a688, a689) {
        true
    } else {
        let a690 = 31u64;
        let a691 = v0;
        if nat_lt(a690, a691) {
            let a692 = v0;
            let a693 = 127u64;
            nat_lt(a692, a693)
        } else {
            false
        }
    }
}

pub fn f193(p696: List<u64>) -> bool {
    let m695 = p696;
    {
        let mut m695 = m695;
        loop {
            let step = {
                let v0 = m695;
                let m694 = v0;
                match m694.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f256(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m695 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(p699: List<u64>, v1: bool) -> bool {
    let m698 = p699;
    {
        let mut m698 = m698;
        loop {
            let step = {
                let v0 = m698;
                let m697 = v0;
                match m697.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f257(v2)
                        } else {
                            f258(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195() -> u64 {
    64u64
}

pub fn f196(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m700 = f238(Str::lit("type"), v1);
    match m700 {
        Adt135::C0 => {
            Some(Adt97::C3(v0))
        }
        Adt135::C1(v2) => {
            let m701 = f235(v2);
            match m701 {
                None => {
                    Some(Adt97::C3(v0))
                }
                Some(v3) => {
                    let a702 = v3;
                    let a703 = Str::lit("object");
                    if equal(a702, a703) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f197(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt97> {
    let m704 = f238(Str::lit("$schema"), v1);
    match m704 {
        Adt135::C0 => {
            None::<Adt97>
        }
        Adt135::C1(v2) => {
            let m705 = f235(v2);
            match m705 {
                None => {
                    Some(Adt97::C2(v0))
                }
                Some(v3) => {
                    let a706 = v3;
                    let a707 = f259();
                    if equal(a706, a707) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f198(v0: Adt36) -> R<Option<Adt97>> {
    let m709 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h708, _, _) => {
            h708
        }
    };
    match m709 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v1) => {
            let m712 = f197(match v0.clone() {
                Adt36::C0(h710, _, _, _, _, _, _, _) => {
                    h710
                }
            }, match v1.clone() {
                Adt5::C0(h711) => {
                    h711
                }
            });
            match m712 {
                None => {
                    let m716 = f156(f157(), match v0 {
                        Adt36::C0(h713, _, _, _, _, _, _, _) => {
                            h713
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h714) => {
                            h714
                        }
                    }), List::cons(Adt105::C0(Adt6::C5(match v1 {
                        Adt5::C0(h715) => {
                            h715
                        }
                    }), false, List::<Str>::nil()), List::<Adt105>::nil()), f158())?;
                    match m716 {
                        Ok(_) => {
                            Ok(None::<Adt97>)
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

pub fn f199(v0: Str, v1: Adt6, v2: Adt105, v3: Adt125, v4: List<Adt105>) -> R<Result<(List<Adt105>, Adt125), Adt97>> {
    let m717 = v2;
    match m717 {
        Adt105::C0(v5, v6, v7) => {
            let m718 = f260(v5.clone());
            match m718 {
                None => {
                    let m719 = f261(v5);
                    match m719 {
                        None => {
                            Ok(Err::<(List<Adt105>, Adt125), Adt97>(Adt97::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f262(v0, v1, match v9 {
                                Adt5::C0(h720) => {
                                    h720
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt105>, Adt125), Adt97>((v4, v3)))
                }
            }
        }
    }
}

pub fn f200(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    let m721 = f260(v1.clone());
    match m721 {
        None => {
            let m722 = f261(v1);
            match m722 {
                None => {
                    Ok(Adt137::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f263(v0, match v5 {
                        Adt5::C0(h723) => {
                            h723
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt137::C3(List::<Adt109>::nil()))
            } else {
                Ok(Adt137::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f201(v0: Adt109, v1: List<Adt109>) -> R<Adt126> {
    let m724 = v0;
    match m724 {
        Adt109::C0(_, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
        Adt109::C1(_, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
        Adt109::C2(v8, v9, v10, v11) => {
            Ok(f264(v8, v9, v10, {
                let a725 = v11;
                let a726 = 1u64;
                nat_add(a725, a726)?
            }, v1))
        }
        Adt109::C3(v12) => {
            Ok(Adt126::C0(Adt108::C1(v12, Str::lit("not")), v1))
        }
        Adt109::C4(v13, _, v15, v16) => {
            Ok(f265(v13, v15, v16, v1))
        }
        Adt109::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt108::C0, v1))
        }
    }
}

pub fn f202(v0: Adt110, v1: Adt6, v2: Adt6, v3: List<Adt107>) -> Adt137 {
    let m727 = f261(v1);
    match m727 {
        None => {
            Adt137::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m728 = v0;
            match m728 {
                Adt110::C0 => {
                    f266(match v4 {
                        Adt5::C0(h729) => {
                            h729
                        }
                    }, v2, v3)
                }
                Adt110::C1 => {
                    f267(match v4 {
                        Adt5::C0(h730) => {
                            h730
                        }
                    }, v2, v3)
                }
                Adt110::C2 => {
                    f268(match v4 {
                        Adt5::C0(h731) => {
                            h731
                        }
                    }, v2, v3)
                }
                Adt110::C3 => {
                    f269(match v4 {
                        Adt5::C0(h732) => {
                            h732
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f203(v0: Adt109, v1: List<Adt109>, v2: List<Adt107>, v3: Str) -> Adt126 {
    let m733 = v0;
    match m733 {
        Adt109::C0(_, _, _) => {
            Adt126::C0(Adt108::C1(v2, v3), v1)
        }
        Adt109::C1(v7, v8, v9) => {
            f270(v7, v8, v9, v1)
        }
        Adt109::C2(v10, v11, v12, v13) => {
            f264(v10, v11, v12, v13, v1)
        }
        Adt109::C3(_) => {
            Adt126::C0(Adt108::C0, v1)
        }
        Adt109::C4(_, v16, v17, v18) => {
            f265(v16, v17, v18, v1)
        }
        Adt109::C5(_, _, _, _) => {
            Adt126::C0(Adt108::C1(v2, v3), v1)
        }
    }
}

pub fn f204(p742: List<Adt39>) -> R<bool> {
    let m741 = p742;
    {
        let mut m741 = m741;
        loop {
            let step = {
                let v0 = m741;
                let m734 = v0;
                match m734.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m735 = v1;
                        match m735 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f271(match v4 {
                                    Adt43::C0(h736, _, _, _) => {
                                        h736
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f271(match v5 {
                                    Adt44::C0(h737, _, _, _) => {
                                        h737
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C3(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C4(v7) => {
                                let m739 = match v7 {
                                    Adt46::C0(h738, _, _) => {
                                        h738
                                    }
                                };
                                match m739 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f271(match v9 {
                                            Adt49::C0(_, _, h740, _) => {
                                                h740
                                            }
                                        })? {
                                            Ok::<List<Adt39>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt39>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m741 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f272(match v1.clone() {
        Adt50::C0(h743, _) => {
            h743
        }
    }) {
        f273(v1)
    } else {
        let m745 = match v0 {
            Adt36::C0(_, _, _, _, _, h744, _, _) => {
                h744
            }
        };
        match m745 {
            None => {
                f273(v1)
            }
            Some(v2) => {
                f274(v2, v1)
            }
        }
    }
}

pub fn f206(p748: List<Adt107>, p749: List<Str>) -> R<List<Str>> {
    let m747 = (p748, p749);
    {
        let mut m747 = m747;
        loop {
            let step = {
                let (v0, v1) = m747;
                let m746 = v0;
                match m746.uncons() {
                    None => {
                        Err::<(List<Adt107>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt107>, List<Str>), R<List<Str>>>((v3, List::cons(f275(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Bytes, v1: List<u64>) -> Bytes {
    let a750 = v0;
    let a751 = f276(f87(), f277(List::<Bytes>::nil(), v1));
    append_bytes(a750, a751)
}

pub fn f208(v0: i64, v1: Str, v2: Str, v3: Adt95) -> Result<(Adt14, u64), Adt93> {
    Ok::<(Adt14, u64), Adt93>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt95::C0(h752, _) => {
            h752
        }
    }), match v3 {
        Adt95::C0(_, h753) => {
            h753
        }
    }))
}

pub fn f209(v0: Adt111, v1: List<Adt111>, v2: List<u64>) -> R<Adt138> {
    let m754 = v0;
    match m754 {
        Adt111::C0(v3) => {
            Ok(Adt138::C1(v1, f169(v3, v2)))
        }
        Adt111::C1(v4, v5) => {
            f278(v4, v5, v1, v2)
        }
        Adt111::C2(v6, v7) => {
            let m755 = v7;
            match m755.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C1(v6, v8), List::cons(Adt111::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt111::C3(v10, v11) => {
            let m756 = v11.clone();
            match m756.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt111::C4(v14, v15) => {
            let m757 = v15;
            match m757.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0({
                        let a759 = f279({
                            let (h758, _) = v16;
                            h758
                        })?;
                        let a760 = List::cons(58u64, List::<u64>::nil());
                        append_list(a759, a760)
                    }), List::cons(Adt111::C1(v14, {
                        let (_, h761) = v16;
                        h761
                    }), List::cons(Adt111::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt111::C5(v18, v19) => {
            let m762 = v19.clone();
            match m762.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt111::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt111::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f210(v0: Adt128) -> R<Option<Adt5>> {
    let m764 = f280(match v0.clone() {
        Adt128::C0(h763, _, _) => {
            h763
        }
    })?;
    match m764 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a767 = v1;
                let a768 = {
                    let m766 = f281(v0);
                    match m766 {
                        Adt5::C0(h765) => {
                            h765
                        }
                    }
                };
                append_list(a767, a768)
            })))
        }
    }
}

pub fn f211(v0: Adt14, v1: Option<Adt88>, v2: Adt94) -> R<Adt94> {
    let m769 = v1;
    match m769 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a771 = match v2.clone() {
                Adt94::C0(h770, _, _, _, _) => {
                    h770
                }
            };
            let a772 = f214();
            if nat_eq(a771, a772) {
                Ok(f282(f283(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f212(v0: Adt33) -> Option<Adt88> {
    let m773 = v0;
    match m773 {
        Adt33::C0(_) => {
            None::<Adt88>
        }
        Adt33::C1(_) => {
            None::<Adt88>
        }
        Adt33::C2(_) => {
            None::<Adt88>
        }
        Adt33::C3(_) => {
            None::<Adt88>
        }
        Adt33::C4(_) => {
            None::<Adt88>
        }
        Adt33::C5(_) => {
            None::<Adt88>
        }
        Adt33::C6(_) => {
            None::<Adt88>
        }
        Adt33::C7(_) => {
            None::<Adt88>
        }
        Adt33::C8(_) => {
            None::<Adt88>
        }
        Adt33::C9(_) => {
            None::<Adt88>
        }
        Adt33::C10(_) => {
            None::<Adt88>
        }
        Adt33::C11 => {
            None::<Adt88>
        }
        Adt33::C12(_) => {
            None::<Adt88>
        }
        Adt33::C13(v13) => {
            Some(Adt88::C0(match v13 {
                Adt83::C0(_, _, h774) => {
                    h774
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f213(v0: bool, v1: Adt94, v2: Option<List<List<u64>>>) -> Adt94 {
    let m775 = v2;
    match m775 {
        None => {
            f6()
        }
        Some(v3) => {
            f284(v0, v1, f37(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f214() -> u64 {
    200u64
}

pub fn f215(v0: Adt98, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f129())), List::cons((Str::lit("id"), f170(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h776) => {
            h776
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f216(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m777 = v0;
    match m777 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m778 = v1;
            match m778 {
                None => {
                    None::<List<List<u64>>>
                }
                Some(v3) => {
                    Some(f37(v3, v2))
                }
            }
        }
    }
}

pub fn f217(p782: List<Adt130>, p783: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m781 = (p782, p783);
    {
        let mut m781 = m781;
        loop {
            let step = {
                let (v0, v1) = m781;
                let m779 = v0;
                match m779.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m780 = f285(v2)?;
                        match m780 {
                            None => {
                                Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f218(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m784 = v0;
    match m784 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f286(v1)
        }
    }
}

pub fn f219(v0: Adt131) -> u64 {
    let m785 = v0;
    match m785 {
        Adt131::C0(_) => {
            f96()
        }
        Adt131::C1(_, _) => {
            f96()
        }
        Adt131::C2(_) => {
            f96()
        }
        Adt131::C3(_) => {
            f287()
        }
        Adt131::C4(v6) => {
            f288(v6)
        }
    }
}

pub fn f220(v0: Adt131) -> R<Adt81> {
    Ok(Adt81::C0(f289(v0.clone()), f290(v0.clone()), f291(v0)?))
}

pub fn f221() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f222(p788: List<List<u64>>) -> R<bool> {
    let m787 = p788;
    {
        let mut m787 = m787;
        loop {
            let step = {
                let v0 = m787;
                let m786 = v0;
                match m786.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f292(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: u64, p793: List<u64>, p794: List<u64>, p795: List<List<u64>>) -> List<List<u64>> {
    let m792 = (p793, (p794, p795));
    {
        let mut m792 = m792;
        loop {
            let step = {
                let (v1, (v2, v3)) = m792;
                let m789 = v1;
                match m789.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f37(List::cons(f152(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a790 = v4;
                        let a791 = v0;
                        if nat_eq(a790, a791) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f152(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt139, Adt131>) -> R<Result<Adt128, Adt131>> {
    let m796 = v3;
    match m796 {
        Ok(v5) => {
            f293(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(v4))
        }
    }
}

pub fn f225(v0: Option<Adt112>, v1: List<Str>, v2: Adt104) -> R<Result<Adt139, Adt131>> {
    Ok(f294(f295(match v2.clone() {
        Adt104::C0(_, h797, _) => {
            h797
        }
    }), f296(v0, v1, v2)?))
}

pub fn f226(v0: Adt114, v1: Adt102) -> Adt133 {
    let m798 = v1;
    match m798 {
        Adt102::C0 => {
            f232()
        }
        Adt102::C1 => {
            Adt133::C0(None::<Str>, Some(Adt113::C6(v0)))
        }
        Adt102::C2 => {
            Adt133::C0(None::<Str>, Some(Adt113::C7(v0)))
        }
        Adt102::C3(v2) => {
            let m799 = f166(v2);
            match m799 {
                None => {
                    Adt133::C0(None::<Str>, Some(Adt113::C6(v0)))
                }
                Some(v3) => {
                    Adt133::C0(Some(v3), None::<Adt113>)
                }
            }
        }
    }
}

pub fn f227() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f228() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f229(v0: Str) -> bool {
    let m800 = f297(v0);
    match m800 {
        Adt145::C0 => {
            false
        }
        Adt145::C1 => {
            true
        }
        Adt145::C2 => {
            true
        }
        Adt145::C3 => {
            true
        }
    }
}

pub fn f230(v0: Adt102) -> R<Adt102> {
    let m801 = v0.clone();
    match m801 {
        Adt102::C0 => {
            Ok(v0)
        }
        Adt102::C1 => {
            Ok(v0)
        }
        Adt102::C2 => {
            Ok(v0)
        }
        Adt102::C3(v1) => {
            let m802 = f298(v1);
            match m802 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m803 = f299(v2)?;
                    match m803 {
                        None => {
                            Ok(Adt102::C1)
                        }
                        Some(v3) => {
                            Ok(Adt102::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f231() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f232() -> Adt133 {
    Adt133::C0(None::<Str>, None::<Adt113>)
}

pub fn f233(v0: Option<Adt113>, v1: Option<Adt113>) -> Option<Adt113> {
    let m804 = v0.clone();
    match m804 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f234(v0: List<Adt92>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt115>> {
    Ok(f300(f301(v0, v1, v2, List::<Adt115>::nil())?, List::<Adt115>::nil()))
}

pub fn f235(v0: Adt6) -> Option<Str> {
    let m805 = v0;
    match m805 {
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

pub fn f236() -> Str {
    Str::lit("2026-07-28")
}

pub fn f237(v0: Adt7) -> R<Option<i64>> {
    let m808 = f302(40u64, match v0.clone() {
        Adt7::C0(h806, _) => {
            h806
        }
    }, match v0 {
        Adt7::C0(_, h807) => {
            h807
        }
    })?;
    match m808 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f303(v1))
        }
    }
}

pub fn f238(v0: Str, p814: List<(Str, Adt6)>) -> Adt135 {
    let m813 = p814;
    {
        let mut m813 = m813;
        loop {
            let step = {
                let v1 = m813;
                let m809 = v1;
                match m809.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m811 = f304({
                            let (h810, _) = v2;
                            h810
                        }, v0.clone());
                        match m811 {
                            Adt146::C0 => {
                                Err::<List<(Str, Adt6)>, Adt135>(Adt135::C1({
                                    let (_, h812) = v2;
                                    h812
                                }))
                            }
                            Adt146::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt135>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: Str, v1: Adt5) -> bool {
    f305(f182(v0, v1))
}

pub fn f240(v0: Adt98, v1: Adt5) -> Result<Adt104, Adt100> {
    let m815 = f182(Str::lit("method"), v1.clone());
    match m815 {
        Adt135::C0 => {
            f138(Some(v0), f183(v1))
        }
        Adt135::C1(v2) => {
            let m816 = f235(v2);
            match m816 {
                None => {
                    f138(Some(v0), Adt101::C4)
                }
                Some(v3) => {
                    f306(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f241(v0: u64) -> R<u64> {
    let a819 = {
        let a817 = v0;
        let a818 = 3u64;
        nat_mul(a817, a818)?
    };
    let a820 = 1u64;
    nat_add(a819, a820)
}

pub fn f242(v0: u64, v1: u64, v2: u64) -> bool {
    let a821 = v1;
    let a822 = v0;
    if nat_le(a821, a822) {
        let a823 = v0;
        let a824 = v2;
        nat_le(a823, a824)
    } else {
        false
    }
}

pub fn f243(v0: Adt122) -> R<Adt141> {
    if f307(match v0.clone() {
        Adt122::C0(_, h825, _, _, _, _, _) => {
            h825
        }
    }) {
        let v1: i64 = {
            let a831 = if match v0.clone() {
                Adt122::C0(_, _, _, _, _, h826, _) => {
                    h826
                }
            } {
                let a828 = match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h827) => {
                        h827
                    }
                };
                int_neg(a828)?
            } else {
                match v0.clone() {
                    Adt122::C0(_, _, _, _, _, _, h829) => {
                        h829
                    }
                }
            };
            let a832 = match v0.clone() {
                Adt122::C0(_, _, _, _, h830, _, _) => {
                    h830
                }
            };
            int_sub(a831, a832)?
        };
        let m835 = {
            let a833 = v1;
            let a834 = -9999i64;
            compare(a833, a834)
        };
        if if match m835 {
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
            let m838 = {
                let a836 = 9999i64;
                let a837 = v1;
                compare(a836, a837)
            };
            match m838 {
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
            Ok(Adt141::C1)
        } else {
            Ok(Adt141::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt122::C0(h839, _, _, _, _, _, _) => {
                    h839
                }
            } {
                let a841 = match v0 {
                    Adt122::C0(_, _, h840, _, _, _, _) => {
                        h840
                    }
                };
                int_neg(a841)?
            } else {
                match v0 {
                    Adt122::C0(_, _, h842, _, _, _, _) => {
                        h842
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt141::C0)
    }
}

pub fn f244(v0: Adt6, v1: List<Adt124>, v2: u64) -> Adt119 {
    let m843 = v1.clone();
    match m843.uncons() {
        None => {
            Adt119::C0(Adt120::C9(v0), List::<Adt124>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m844 = v3;
            match m844 {
                Adt124::C0(v5) => {
                    Adt119::C0(Adt120::C2, List::cons(Adt124::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt124::C1(v6, v7) => {
                    let m845 = v7;
                    match m845 {
                        None => {
                            Adt119::C0(Adt120::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt119::C0(Adt120::C2, List::cons(Adt124::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f245(v0: u64) -> bool {
    let a846 = v0;
    let a847 = 32u64;
    if nat_eq(a846, a847) {
        true
    } else {
        let a848 = v0;
        let a849 = 10u64;
        if nat_eq(a848, a849) {
            true
        } else {
            let a850 = v0;
            let a851 = 13u64;
            if nat_eq(a850, a851) {
                true
            } else {
                let a852 = v0;
                let a853 = 9u64;
                nat_eq(a852, a853)
            }
        }
    }
}

pub fn f246(v0: Adt119, v1: u64, v2: u64) -> R<Adt136> {
    let a854 = v2;
    let a855 = 34u64;
    if nat_eq(a854, a855) {
        Ok(f250(v0, Adt120::C6(Adt121::C0, List::<u64>::nil(), false)))
    } else {
        let a856 = v2;
        let a857 = 123u64;
        if nat_eq(a856, a857) {
            f308(v0, Adt124::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt120::C3, v1)
        } else {
            let a858 = v2;
            let a859 = 91u64;
            if nat_eq(a858, a859) {
                f308(v0, Adt124::C0(List::<Adt6>::nil()), Adt120::C1, v1)
            } else {
                let a860 = v2;
                let a861 = 116u64;
                if nat_eq(a860, a861) {
                    Ok(f250(v0, Adt120::C7(0u64, 1u64)))
                } else {
                    let a862 = v2;
                    let a863 = 102u64;
                    if nat_eq(a862, a863) {
                        Ok(f250(v0, Adt120::C7(1u64, 1u64)))
                    } else {
                        let a864 = v2;
                        let a865 = 110u64;
                        if nat_eq(a864, a865) {
                            Ok(f250(v0, Adt120::C7(2u64, 1u64)))
                        } else {
                            let a866 = v2;
                            let a867 = 45u64;
                            if nat_eq(a866, a867) {
                                Ok(f250(v0, f309(true, Adt123::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a868 = v2;
                                let a869 = 48u64;
                                if nat_eq(a868, a869) {
                                    Ok(f250(v0, f309(false, Adt123::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f242(v2, 49u64, 57u64) {
                                        Ok(f250(v0, f309(false, Adt123::C2, f310({
                                            let a870 = v2;
                                            let a871 = 48u64;
                                            nat_sub(a870, a871)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f251(Adt99::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f247(v0: Adt119, v1: u64) -> Adt136 {
    let m873 = match v0.clone() {
        Adt119::C0(_, h872, _) => {
            h872
        }
    };
    match m873.uncons() {
        None => {
            f251(Adt99::C2(v1))
        }
        Some((v2, v3)) => {
            let m874 = v2;
            match m874 {
                Adt124::C0(v4) => {
                    Adt136::C1(f244(Adt6::C4(f311(v4, List::<Adt6>::nil())), v3, {
                        let a876 = match v0 {
                            Adt119::C0(_, _, h875) => {
                                h875
                            }
                        };
                        let a877 = 1u64;
                        nat_sub(a876, a877)
                    }))
                }
                Adt124::C1(_, _) => {
                    f251(Adt99::C2(v1))
                }
            }
        }
    }
}

pub fn f248(v0: Adt119, v1: u64, v2: u64) -> Adt136 {
    let m879 = match v0.clone() {
        Adt119::C0(h878, _, _) => {
            h878
        }
    };
    match m879 {
        Adt120::C0 => {
            f251(Adt99::C2(v1))
        }
        Adt120::C1 => {
            f251(Adt99::C2(v1))
        }
        Adt120::C2 => {
            if f245(v2) {
                Adt136::C1(v0)
            } else {
                let a880 = v2;
                let a881 = 44u64;
                if nat_eq(a880, a881) {
                    f312(v0, v1)
                } else {
                    let a882 = v2;
                    let a883 = 93u64;
                    if nat_eq(a882, a883) {
                        f247(v0, v1)
                    } else {
                        let a884 = v2;
                        let a885 = 125u64;
                        if nat_eq(a884, a885) {
                            f249(v0, v1)
                        } else {
                            f251(Adt99::C2(v1))
                        }
                    }
                }
            }
        }
        Adt120::C3 => {
            f251(Adt99::C2(v1))
        }
        Adt120::C4 => {
            f251(Adt99::C2(v1))
        }
        Adt120::C5 => {
            f251(Adt99::C2(v1))
        }
        Adt120::C6(_, _, _) => {
            f251(Adt99::C2(v1))
        }
        Adt120::C7(_, _) => {
            f251(Adt99::C2(v1))
        }
        Adt120::C8(_) => {
            f251(Adt99::C2(v1))
        }
        Adt120::C9(_) => {
            if f245(v2) {
                Adt136::C1(v0)
            } else {
                f251(Adt99::C10(v1))
            }
        }
    }
}

pub fn f249(v0: Adt119, v1: u64) -> Adt136 {
    let m887 = match v0.clone() {
        Adt119::C0(_, h886, _) => {
            h886
        }
    };
    match m887.uncons() {
        None => {
            f251(Adt99::C2(v1))
        }
        Some((v2, v3)) => {
            let m888 = v2;
            match m888 {
                Adt124::C0(_) => {
                    f251(Adt99::C2(v1))
                }
                Adt124::C1(v5, _) => {
                    Adt136::C1(f244(Adt6::C5(f313(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a890 = match v0 {
                            Adt119::C0(_, _, h889) => {
                                h889
                            }
                        };
                        let a891 = 1u64;
                        nat_sub(a890, a891)
                    }))
                }
            }
        }
    }
}

pub fn f250(v0: Adt119, v1: Adt120) -> Adt136 {
    Adt136::C1(f314(v0, v1))
}

pub fn f251(v0: Adt99) -> Adt136 {
    Adt136::C0(v0)
}

pub fn f252(v0: Adt119, v1: u64, v2: u64, v3: Adt121, v4: List<u64>, v5: bool) -> R<Adt136> {
    let m892 = v3;
    match m892 {
        Adt121::C0 => {
            let a893 = v2;
            let a894 = 34u64;
            if nat_eq(a893, a894) {
                Ok(f315(v0, v1, v4, v5))
            } else {
                let a895 = v2;
                let a896 = 92u64;
                if nat_eq(a895, a896) {
                    Ok(f250(v0, Adt120::C6(Adt121::C1, v4, v5)))
                } else {
                    let a897 = v2;
                    let a898 = 32u64;
                    if nat_lt(a897, a898) {
                        Ok(f251(Adt99::C4(v1)))
                    } else {
                        Ok(f250(v0, Adt120::C6(Adt121::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt121::C1 => {
            let a899 = v2;
            let a900 = 117u64;
            if nat_eq(a899, a900) {
                Ok(f250(v0, Adt120::C6(Adt121::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a901 = f316(v2);
                let a902 = 0u64;
                if nat_eq(a901, a902) {
                    Ok(f251(Adt99::C5(v1)))
                } else {
                    Ok(f250(v0, Adt120::C6(Adt121::C0, List::cons(f316(v2), v4), v5)))
                }
            }
        }
        Adt121::C2(v6, v7, v8) => {
            let a903 = 15u64;
            let a904 = f317(v2);
            if nat_lt(a903, a904) {
                Ok(f251(Adt99::C5(v1)))
            } else {
                let a907 = {
                    let a905 = v6;
                    let a906 = 1u64;
                    nat_add(a905, a906)?
                };
                let a908 = 4u64;
                if nat_eq(a907, a908) {
                    f318(v0, v1, {
                        let a911 = {
                            let a909 = v7;
                            let a910 = 16u64;
                            nat_mul(a909, a910)?
                        };
                        let a912 = f317(v2);
                        nat_add(a911, a912)?
                    }, v8, v4, v5)
                } else {
                    Ok(f250(v0, Adt120::C6(Adt121::C2({
                        let a913 = v6;
                        let a914 = 1u64;
                        nat_add(a913, a914)?
                    }, {
                        let a917 = {
                            let a915 = v7;
                            let a916 = 16u64;
                            nat_mul(a915, a916)?
                        };
                        let a918 = f317(v2);
                        nat_add(a917, a918)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt121::C3(v9) => {
            let a919 = v2;
            let a920 = 92u64;
            if nat_eq(a919, a920) {
                Ok(f250(v0, Adt120::C6(Adt121::C4(v9), v4, v5)))
            } else {
                Ok(f251(Adt99::C6(v1)))
            }
        }
        Adt121::C4(v10) => {
            let a921 = v2;
            let a922 = 117u64;
            if nat_eq(a921, a922) {
                Ok(f250(v0, Adt120::C6(Adt121::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f251(Adt99::C6(v1)))
            }
        }
    }
}

pub fn f253(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt136> {
    let a923 = v2;
    let a924 = f319(v3, v4);
    if nat_eq(a923, a924) {
        let a928 = {
            let a925 = v4;
            let a926 = 1u64;
            nat_add(a925, a926)?
        };
        let a929 = {
            let a927 = f320(v3);
            length_list(a927)
        };
        if nat_eq(a928, a929) {
            Ok(f321(v0, f322(v3)))
        } else {
            Ok(f250(v0, Adt120::C7(v3, {
                let a930 = v4;
                let a931 = 1u64;
                nat_add(a930, a931)?
            })))
        }
    } else {
        Ok(f251(Adt99::C2(v1)))
    }
}

pub fn f254(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m933 = match v3.clone() {
        Adt122::C0(_, h932, _, _, _, _, _) => {
            h932
        }
    };
    match m933 {
        Adt123::C0 => {
            let a934 = v2;
            let a935 = 48u64;
            if nat_eq(a934, a935) {
                Ok(f323(v0, v3, Adt123::C1))
            } else {
                if f242(v2, 49u64, 57u64) {
                    f324(v0, v1, v2, v3, false)
                } else {
                    Ok(f251(Adt99::C7(v1)))
                }
            }
        }
        Adt123::C1 => {
            let a936 = v2;
            let a937 = 46u64;
            if nat_eq(a936, a937) {
                Ok(f323(v0, v3, Adt123::C3))
            } else {
                if f325(v2) {
                    Ok(f323(v0, v3, Adt123::C5))
                } else {
                    if f326(v2) {
                        Ok(f251(Adt99::C7(v1)))
                    } else {
                        f327(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C2 => {
            if f326(v2) {
                f324(v0, v1, v2, v3, false)
            } else {
                let a938 = v2;
                let a939 = 46u64;
                if nat_eq(a938, a939) {
                    Ok(f323(v0, v3, Adt123::C3))
                } else {
                    if f325(v2) {
                        Ok(f323(v0, v3, Adt123::C5))
                    } else {
                        f327(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt123::C3 => {
            if f326(v2) {
                f324(v0, v1, v2, v3, true)
            } else {
                Ok(f251(Adt99::C7(v1)))
            }
        }
        Adt123::C4 => {
            if f326(v2) {
                f324(v0, v1, v2, v3, true)
            } else {
                if f325(v2) {
                    Ok(f323(v0, v3, Adt123::C5))
                } else {
                    f327(v0, v1, v2, v3)
                }
            }
        }
        Adt123::C5 => {
            let a940 = v2;
            let a941 = 43u64;
            if nat_eq(a940, a941) {
                Ok(f250(v0, f309(match v3.clone() {
                    Adt122::C0(h942, _, _, _, _, _, _) => {
                        h942
                    }
                }, Adt123::C6, match v3.clone() {
                    Adt122::C0(_, _, h943, _, _, _, _) => {
                        h943
                    }
                }, match v3.clone() {
                    Adt122::C0(_, _, _, h944, _, _, _) => {
                        h944
                    }
                }, match v3 {
                    Adt122::C0(_, _, _, _, h945, _, _) => {
                        h945
                    }
                }, false, 0i64)))
            } else {
                let a946 = v2;
                let a947 = 45u64;
                if nat_eq(a946, a947) {
                    Ok(f250(v0, f309(match v3.clone() {
                        Adt122::C0(h948, _, _, _, _, _, _) => {
                            h948
                        }
                    }, Adt123::C6, match v3.clone() {
                        Adt122::C0(_, _, h949, _, _, _, _) => {
                            h949
                        }
                    }, match v3.clone() {
                        Adt122::C0(_, _, _, h950, _, _, _) => {
                            h950
                        }
                    }, match v3 {
                        Adt122::C0(_, _, _, _, h951, _, _) => {
                            h951
                        }
                    }, true, 0i64)))
                } else {
                    if f326(v2) {
                        f328(v0, v1, v2, Adt122::C0(match v3.clone() {
                            Adt122::C0(h952, _, _, _, _, _, _) => {
                                h952
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, h953, _, _, _, _, _) => {
                                h953
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, h954, _, _, _, _) => {
                                h954
                            }
                        }, match v3.clone() {
                            Adt122::C0(_, _, _, h955, _, _, _) => {
                                h955
                            }
                        }, match v3 {
                            Adt122::C0(_, _, _, _, h956, _, _) => {
                                h956
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f251(Adt99::C7(v1)))
                    }
                }
            }
        }
        Adt123::C6 => {
            if f326(v2) {
                f328(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h957, _) => {
                        h957
                    }
                })
            } else {
                Ok(f251(Adt99::C7(v1)))
            }
        }
        Adt123::C7 => {
            if f326(v2) {
                f328(v0, v1, v2, v3.clone(), match v3 {
                    Adt122::C0(_, _, _, _, _, h958, _) => {
                        h958
                    }
                })
            } else {
                f327(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f255(v0: u64) -> bool {
    let a959 = v0;
    let a960 = 9u64;
    if nat_eq(a959, a960) {
        true
    } else {
        let a961 = v0;
        let a962 = 32u64;
        nat_eq(a961, a962)
    }
}

pub fn f256(v0: u64) -> bool {
    let a963 = 47u64;
    let a964 = v0;
    if nat_lt(a963, a964) {
        let a965 = v0;
        let a966 = 58u64;
        nat_lt(a965, a966)
    } else {
        false
    }
}

pub fn f257(v0: u64) -> bool {
    let a967 = 48u64;
    let a968 = v0;
    if if nat_le(a967, a968) {
        let a969 = v0;
        let a970 = 57u64;
        nat_le(a969, a970)
    } else {
        false
    } {
        true
    } else {
        let a971 = 65u64;
        let a972 = v0;
        if if nat_le(a971, a972) {
            let a973 = v0;
            let a974 = 90u64;
            nat_le(a973, a974)
        } else {
            false
        } {
            true
        } else {
            let a975 = 97u64;
            let a976 = v0;
            if if nat_le(a975, a976) {
                let a977 = v0;
                let a978 = 122u64;
                nat_le(a977, a978)
            } else {
                false
            } {
                true
            } else {
                let a979 = v0;
                let a980 = 33u64;
                if nat_eq(a979, a980) {
                    true
                } else {
                    let a981 = v0;
                    let a982 = 35u64;
                    if nat_eq(a981, a982) {
                        true
                    } else {
                        let a983 = v0;
                        let a984 = 36u64;
                        if nat_eq(a983, a984) {
                            true
                        } else {
                            let a985 = v0;
                            let a986 = 37u64;
                            if nat_eq(a985, a986) {
                                true
                            } else {
                                let a987 = v0;
                                let a988 = 38u64;
                                if nat_eq(a987, a988) {
                                    true
                                } else {
                                    let a989 = v0;
                                    let a990 = 39u64;
                                    if nat_eq(a989, a990) {
                                        true
                                    } else {
                                        let a991 = v0;
                                        let a992 = 42u64;
                                        if nat_eq(a991, a992) {
                                            true
                                        } else {
                                            let a993 = v0;
                                            let a994 = 43u64;
                                            if nat_eq(a993, a994) {
                                                true
                                            } else {
                                                let a995 = v0;
                                                let a996 = 45u64;
                                                if nat_eq(a995, a996) {
                                                    true
                                                } else {
                                                    let a997 = v0;
                                                    let a998 = 46u64;
                                                    if nat_eq(a997, a998) {
                                                        true
                                                    } else {
                                                        let a999 = v0;
                                                        let a1000 = 94u64;
                                                        if nat_eq(a999, a1000) {
                                                            true
                                                        } else {
                                                            let a1001 = v0;
                                                            let a1002 = 95u64;
                                                            if nat_eq(a1001, a1002) {
                                                                true
                                                            } else {
                                                                let a1003 = v0;
                                                                let a1004 = 96u64;
                                                                if nat_eq(a1003, a1004) {
                                                                    true
                                                                } else {
                                                                    let a1005 = v0;
                                                                    let a1006 = 124u64;
                                                                    if nat_eq(a1005, a1006) {
                                                                        true
                                                                    } else {
                                                                        let a1007 = v0;
                                                                        let a1008 = 126u64;
                                                                        nat_eq(a1007, a1008)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: u64) -> bool {
    let a1009 = 48u64;
    let a1010 = v0;
    if if nat_le(a1009, a1010) {
        let a1011 = v0;
        let a1012 = 57u64;
        nat_le(a1011, a1012)
    } else {
        false
    } {
        true
    } else {
        let a1013 = 65u64;
        let a1014 = v0;
        if if nat_le(a1013, a1014) {
            let a1015 = v0;
            let a1016 = 90u64;
            nat_le(a1015, a1016)
        } else {
            false
        } {
            true
        } else {
            let a1017 = 97u64;
            let a1018 = v0;
            if if nat_le(a1017, a1018) {
                let a1019 = v0;
                let a1020 = 122u64;
                nat_le(a1019, a1020)
            } else {
                false
            } {
                true
            } else {
                let a1021 = v0;
                let a1022 = 95u64;
                if nat_eq(a1021, a1022) {
                    true
                } else {
                    let a1023 = v0;
                    let a1024 = 45u64;
                    if nat_eq(a1023, a1024) {
                        true
                    } else {
                        let a1025 = v0;
                        let a1026 = 46u64;
                        nat_eq(a1025, a1026)
                    }
                }
            }
        }
    }
}

pub fn f259() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f260(v0: Adt6) -> Option<bool> {
    let m1027 = v0;
    match m1027 {
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

pub fn f261(v0: Adt6) -> Option<Adt5> {
    let m1028 = v0;
    match m1028 {
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

pub fn f262(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt125, v6: List<Adt105>) -> R<Result<(List<Adt105>, Adt125), Adt97>> {
    let m1029 = f329(v0.clone(), v1, v2.clone());
    match m1029 {
        None => {
            let m1030 = f330(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1030 {
                Adt147::C0(v8) => {
                    Ok(Err::<(List<Adt105>, Adt125), Adt97>(v8))
                }
                Adt147::C1(v9) => {
                    Ok(Ok::<(List<Adt105>, Adt125), Adt97>((f331(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt105>, Adt125), Adt97>(v7))
        }
    }
}

pub fn f263(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    let m1031 = f332(v1.clone(), v2.clone())?;
    match m1031 {
        Adt137::C0(v4) => {
            Ok(Adt137::C0(v4))
        }
        Adt137::C1(v5) => {
            Ok(Adt137::C1(v5))
        }
        Adt137::C2(v6) => {
            Ok(Adt137::C2(v6))
        }
        Adt137::C3(_) => {
            f333(v0, v1, v2, v3)
        }
    }
}

pub fn f264(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: u64, v4: List<Adt109>) -> Adt126 {
    let m1032 = v0;
    match m1032.uncons() {
        None => {
            let a1033 = v3;
            let a1034 = 1u64;
            if nat_eq(a1033, a1034) {
                Adt126::C0(Adt108::C0, v4)
            } else {
                Adt126::C0(Adt108::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f265(v0: Option<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt126 {
    let m1035 = v0;
    match m1035 {
        None => {
            Adt126::C0(Adt108::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f266(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1036 = f238(Str::lit("anyOf"), v0);
    match m1036 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1037 = f334(v3);
            match m1037 {
                None => {
                    Adt137::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1038 = v4;
                    match m1038.uncons() {
                        None => {
                            Adt137::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt137::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C1(v6, v1, v2), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f267(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1039 = f238(Str::lit("oneOf"), v0);
    match m1039 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1040 = f334(v3);
            match m1040 {
                None => {
                    Adt137::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1041 = v4;
                    match m1041.uncons() {
                        None => {
                            Adt137::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt137::C3(List::cons(Adt109::C0(v5, v1.clone(), v2.clone()), List::cons(Adt109::C2(v6, v1, v2, 0u64), List::<Adt109>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f268(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1042 = f238(Str::lit("not"), v0);
    match m1042 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            Adt137::C3(List::cons(Adt109::C0(v3, v1, v2.clone()), List::cons(Adt109::C3(v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f269(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1043 = f238(Str::lit("if"), v0.clone());
    match m1043 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            Adt137::C3(List::cons(Adt109::C0(v3, v1.clone(), v2.clone()), List::cons(Adt109::C4(f335(Str::lit("then"), v0.clone()), f335(Str::lit("else"), v0), v1, v2), List::<Adt109>::nil())))
        }
    }
}

pub fn f270(v0: List<Adt6>, v1: Adt6, v2: List<Adt107>, v3: List<Adt109>) -> Adt126 {
    let m1044 = v0;
    match m1044.uncons() {
        None => {
            Adt126::C0(Adt108::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt108::C0, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), List::cons(Adt109::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f271(v0: Str) -> R<bool> {
    let m1046 = f299(f85({
        let a1045 = v0;
        utf8_encode(a1045)
    })?)?;
    match m1046 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f272(v0: Adt38) -> bool {
    let m1048 = match v0 {
        Adt38::C0(_, _, h1047) => {
            h1047
        }
    };
    match m1048 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f273(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1050 = match v0.clone() {
        Adt50::C0(_, h1049) => {
            h1049
        }
    };
    match m1050.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1051, _) => {
                    h1051
                }
            })))
        }
        Some((_, _)) => {
            if f336(match v0.clone() {
                Adt50::C0(_, h1052) => {
                    h1052
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f274(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1056 = {
        let m1055 = match v1.clone() {
            Adt50::C0(h1054, _) => {
                h1054
            }
        };
        match m1055 {
            Adt38::C0(_, h1053, _) => {
                h1053
            }
        }
    };
    match m1056 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1059 = f116(Adt6::C5(match v0.clone() {
                Adt5::C0(h1057) => {
                    h1057
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1058) => {
                    h1058
                }
            }), v2)?;
            match m1059 {
                Adt106::C0 => {
                    f273(v1)
                }
                Adt106::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt106::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt106::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt106::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f275(v0: Adt107) -> R<Str> {
    let m1060 = v0;
    match m1060 {
        Adt107::C0(v1) => {
            f337(v1)
        }
        Adt107::C1(v2) => {
            let a1061 = f79(v2, 0i64)?;
            Ok(format_i64(a1061))
        }
    }
}

pub fn f276(p1066: Bytes, p1067: List<Bytes>) -> Bytes {
    let m1065 = (p1066, p1067);
    {
        let mut m1065 = m1065;
        loop {
            let step = {
                let (v0, v1) = m1065;
                let m1062 = v1;
                match m1062.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1063 = v2;
                            let a1064 = v0;
                            append_bytes(a1063, a1064)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f277(p1070: List<Bytes>, p1071: List<u64>) -> List<Bytes> {
    let m1069 = (p1070, p1071);
    {
        let mut m1069 = m1069;
        loop {
            let step = {
                let (v0, v1) = m1069;
                let m1068 = v1;
                match m1068.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f338(f339(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: u64, v1: Adt6, v2: List<Adt111>, v3: List<u64>) -> R<Adt138> {
    let m1072 = v1;
    match m1072 {
        Adt6::C0 => {
            Ok(Adt138::C1(v2, f169(f340(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt138::C1(v2, f169(if v4 {
                f340(0u64)
            } else {
                f340(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f341(v5.clone())? {
                Ok(Adt138::C1(v2, f169(f342(v5)?, v3)))
            } else {
                Ok(Adt138::C0(Adt99::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt138::C1(v2, f169(f279(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1073 = v0;
            let a1074 = f343();
            if nat_lt(a1073, a1074) {
                Ok(Adt138::C1(f344(91u64, Adt111::C2({
                    let a1075 = v0;
                    let a1076 = 1u64;
                    nat_add(a1075, a1076)?
                }, v7), v2), v3))
            } else {
                Ok(Adt138::C0(Adt99::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1077 = v0;
            let a1078 = f343();
            if nat_lt(a1077, a1078) {
                Ok(Adt138::C1(f344(123u64, Adt111::C4({
                    let a1079 = v0;
                    let a1080 = 1u64;
                    nat_add(a1079, a1080)?
                }, v8), v2), v3))
            } else {
                Ok(Adt138::C0(Adt99::C12))
            }
        }
    }
}

pub fn f279(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1082 = f169(f345(f85({
            let a1081 = v0;
            utf8_encode(a1081)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1083 = List::cons(34u64, List::<u64>::nil());
        append_list(a1082, a1083)
    }))
}

pub fn f280(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1084 = v0;
    match m1084 {
        Adt33::C0(v1) => {
            Ok(Some(f346(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f347(v2)?))
        }
        Adt33::C2(v3) => {
            f348(f349(v3)?)
        }
        Adt33::C3(v4) => {
            f348(f349(match v4 {
                Adt50::C0(h1085, _) => {
                    h1085
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f350(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f351(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f352(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f353(v8)?))
        }
        Adt33::C8(v9) => {
            f348(f354(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f355(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f348(f356(v12))
        }
        Adt33::C13(v13) => {
            f348(f357({
                let m1088 = match v13 {
                    Adt83::C0(_, _, h1087) => {
                        h1087
                    }
                };
                match m1088 {
                    Adt85::C0(h1086, _, _) => {
                        h1086
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f358(v14))
        }
    }
}

pub fn f281(v0: Adt128) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f359(f360(match v0.clone() {
        Adt128::C0(h1089, _, _) => {
            h1089
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f361({
        let m1092 = match v0 {
            Adt128::C0(_, h1091, _) => {
                h1091
            }
        };
        match m1092 {
            Adt129::C0(h1090, _) => {
                h1090
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f282(v0: Option<Option<List<u64>>>, v1: Adt94) -> Adt94 {
    let m1093 = v0;
    match m1093 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt94::C0(match v1.clone() {
                Adt94::C0(h1094, _, _, _, _) => {
                    h1094
                }
            }, match v1.clone() {
                Adt94::C0(_, h1095, _, _, _) => {
                    h1095
                }
            }, match v1.clone() {
                Adt94::C0(_, _, h1096, _, _) => {
                    h1096
                }
            }, match v1 {
                Adt94::C0(_, _, _, h1097, _) => {
                    h1097
                }
            }, v2)
        }
    }
}

pub fn f283(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1099 = f362(match v0.clone() {
        Adt14::C0(_, _, _, h1098) => {
            h1098
        }
    })?;
    match m1099 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1102 = f363(f364(match v1 {
                Adt88::C0(h1100, _, _) => {
                    h1100
                }
            }, v2, match v0 {
                Adt14::C0(h1101, _, _, _) => {
                    h1101
                }
            })?)?;
            match m1102 {
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

pub fn f284(v0: bool, v1: Adt94, v2: List<List<u64>>) -> Adt94 {
    if f365(v0, match v1.clone() {
        Adt94::C0(h1103, _, _, _, _) => {
            h1103
        }
    }, v2.clone()) {
        f366(match v1.clone() {
            Adt94::C0(h1104, _, _, _, _) => {
                h1104
            }
        }, {
            let a1106 = v2;
            let a1107 = List::cons(match v1 {
                Adt94::C0(_, _, _, h1105, _) => {
                    h1105
                }
            }, List::<List<u64>>::nil());
            append_list(a1106, a1107)
        })
    } else {
        v1
    }
}

pub fn f285(v0: Adt130) -> R<Option<List<u64>>> {
    let m1108 = f367(f368(v0))?;
    match m1108 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1109 = f89(v1)?;
            match m1109 {
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

pub fn f286(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f369(v0.clone()) {
        let m1113 = f370({
            let m1112 = match v0 {
                Adt88::C0(h1111, _, _) => {
                    h1111
                }
            };
            match m1112 {
                Adt85::C0(h1110, _, _) => {
                    h1110
                }
            }
        });
        match m1113 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1114 = f367(v1)?;
                match m1114 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1115 = f89(v2)?;
                        match m1115 {
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

pub fn f287() -> u64 {
    404u64
}

pub fn f288(v0: Adt90) -> u64 {
    let m1116 = v0;
    match m1116 {
        Adt90::C0(_) => {
            f96()
        }
        Adt90::C1(_) => {
            f96()
        }
        Adt90::C2(_) => {
            f96()
        }
        Adt90::C3(_) => {
            f96()
        }
        Adt90::C4(_) => {
            f96()
        }
        Adt90::C5(_) => {
            f96()
        }
        Adt90::C6(_) => {
            f21()
        }
        Adt90::C7(_) => {
            f371()
        }
        Adt90::C8(_) => {
            f96()
        }
        Adt90::C9(_) => {
            f21()
        }
    }
}

pub fn f289(v0: Adt131) -> i64 {
    let m1117 = v0;
    match m1117 {
        Adt131::C0(_) => {
            f372()
        }
        Adt131::C1(_, _) => {
            f373()
        }
        Adt131::C2(_) => {
            f374()
        }
        Adt131::C3(_) => {
            f375()
        }
        Adt131::C4(v6) => {
            f376(v6)
        }
    }
}

pub fn f290(v0: Adt131) -> Str {
    let m1118 = v0;
    match m1118 {
        Adt131::C0(v1) => {
            f377(v1)
        }
        Adt131::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt131::C2(v4) => {
            f378(v4)
        }
        Adt131::C3(_) => {
            Str::lit("Method not found")
        }
        Adt131::C4(v6) => {
            let m1119 = v6;
            match m1119 {
                Adt90::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt90::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt90::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt90::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt90::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt90::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt90::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt90::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt90::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt90::C9(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f291(v0: Adt131) -> R<Option<Adt6>> {
    let m1120 = v0;
    match m1120 {
        Adt131::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f379(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt131::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C4(v6) => {
            let m1121 = v6;
            match m1121 {
                Adt90::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C2(v9) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(v9)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C4(v11) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("reason"), Adt6::C3(v11)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f79(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f380(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f292(v0: List<u64>) -> R<bool> {
    let m1122 = f223(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1122.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f381(v2)? {
                Ok(false)
            } else {
                f142(f144(v1), f382())
            }
        }
    }
}

pub fn f293(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139) -> R<Result<Adt128, Adt131>> {
    f383(v0.clone(), v1, v2, v3.clone(), f384(match v0 {
        Adt0::C0(_, _, h1123, _, _, _) => {
            h1123
        }
    }, match v3 {
        Adt139::C0(_, h1124, _, _) => {
            h1124
        }
    }))
}

pub fn f294(v0: Adt142, v1: Adt143) -> Result<Adt139, Adt131> {
    let m1125 = v1;
    match m1125 {
        Adt143::C0(v3) => {
            f385(v3, v0)
        }
        Adt143::C1(v2) => {
            Err::<Adt139, Adt131>(v2)
        }
    }
}

pub fn f295(v0: Str) -> Adt142 {
    let a1126 = v0.clone();
    let a1127 = Str::lit("server/discover");
    if equal(a1126, a1127) {
        Adt142::C1(Adt140::C0)
    } else {
        let a1128 = v0.clone();
        let a1129 = Str::lit("tools/list");
        if equal(a1128, a1129) {
            Adt142::C1(Adt140::C1)
        } else {
            let a1130 = v0.clone();
            let a1131 = Str::lit("tools/call");
            if equal(a1130, a1131) {
                Adt142::C1(Adt140::C2)
            } else {
                let a1132 = v0.clone();
                let a1133 = Str::lit("resources/list");
                if equal(a1132, a1133) {
                    Adt142::C1(Adt140::C3)
                } else {
                    let a1134 = v0.clone();
                    let a1135 = Str::lit("resources/read");
                    if equal(a1134, a1135) {
                        Adt142::C1(Adt140::C4)
                    } else {
                        let a1136 = v0.clone();
                        let a1137 = Str::lit("resources/templates/list");
                        if equal(a1136, a1137) {
                            Adt142::C1(Adt140::C5)
                        } else {
                            let a1138 = v0.clone();
                            let a1139 = Str::lit("prompts/list");
                            if equal(a1138, a1139) {
                                Adt142::C1(Adt140::C6)
                            } else {
                                let a1140 = v0.clone();
                                let a1141 = Str::lit("prompts/get");
                                if equal(a1140, a1141) {
                                    Adt142::C1(Adt140::C7)
                                } else {
                                    let a1142 = v0.clone();
                                    let a1143 = Str::lit("completion/complete");
                                    if equal(a1142, a1143) {
                                        Adt142::C1(Adt140::C8)
                                    } else {
                                        let a1144 = v0.clone();
                                        let a1145 = Str::lit("tasks/get");
                                        if equal(a1144, a1145) {
                                            Adt142::C1(Adt140::C9)
                                        } else {
                                            let a1146 = v0.clone();
                                            let a1147 = Str::lit("tasks/update");
                                            if equal(a1146, a1147) {
                                                Adt142::C1(Adt140::C10)
                                            } else {
                                                let a1148 = v0;
                                                let a1149 = Str::lit("tasks/cancel");
                                                if equal(a1148, a1149) {
                                                    Adt142::C1(Adt140::C11)
                                                } else {
                                                    Adt142::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f296(v0: Option<Adt112>, v1: List<Str>, v2: Adt104) -> R<Adt143> {
    f386(v0, v1, v2.clone(), f387(match v2 {
        Adt104::C0(_, _, h1150) => {
            h1150
        }
    }))
}

pub fn f297(v0: Str) -> Adt145 {
    let m1151 = f295(v0);
    match m1151 {
        Adt142::C0 => {
            Adt145::C0
        }
        Adt142::C1(v1) => {
            f388(v1)
        }
    }
}

pub fn f298(v0: List<u64>) -> Option<List<u64>> {
    let m1152 = f389(f390(), v0);
    match m1152 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1153 = f389(f391(), f152(v1, List::<u64>::nil()));
            match m1153 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f152(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f299(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1155 = f392({
        let a1154 = v0.clone();
        length_list(a1154)
    }, v0, List::<u64>::nil())?;
    match m1155 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f169(v1, List::<u64>::nil())))
        }
    }
}

pub fn f300(p1158: List<Adt115>, p1159: List<Adt115>) -> List<Adt115> {
    let m1157 = (p1158, p1159);
    {
        let mut m1157 = m1157;
        loop {
            let step = {
                let (v0, v1) = m1157;
                let m1156 = v0;
                match m1156.uncons() {
                    None => {
                        Err::<(List<Adt115>, List<Adt115>), List<Adt115>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt115>, List<Adt115>), List<Adt115>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: List<Adt92>, p1166: List<Adt12>, v2: Option<Str>, p1167: List<Adt115>) -> R<List<Adt115>> {
    let m1165 = (p1166, p1167);
    {
        let mut m1165 = m1165;
        loop {
            let step = {
                let (v1, v3) = m1165;
                let m1160 = v1;
                match m1160.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f393(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, List::cons(Adt115::C0(match v4.clone() {
                                Adt12::C0(_, h1161, _, _) => {
                                    h1161
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1162, _) => {
                                    h1162
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1163) => {
                                    h1163
                                }
                            }, f394(v0.clone(), match v4 {
                                Adt12::C0(_, h1164, _, _) => {
                                    h1164
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt115>), R<List<Adt115>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1165 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(p1183: u64, p1184: i64, p1185: i64) -> R<Option<i64>> {
    let m1182 = (p1183, (p1184, p1185));
    {
        let mut m1182 = m1182;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1182;
                let m1168 = v0;
                if m1168 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1168 - 1;
                    if f395(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1171 = {
                            let a1169 = v2;
                            let a1170 = 0i64;
                            compare(a1169, a1170)
                        };
                        if match m1171 {
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
                            if f395({
                                let a1172 = v1;
                                let a1173 = 10i64;
                                let a1174 = 0i64;
                                int_rem(a1172, a1173, a1174)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1175 = v1;
                                    let a1176 = 10i64;
                                    let a1177 = 0i64;
                                    int_quot(a1175, a1176, a1177)?
                                }, f115(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1178 = v1;
                                let a1179 = 10i64;
                                int_mul(a1178, a1179)?
                            }, {
                                let a1180 = v2;
                                let a1181 = 1i64;
                                int_sub(a1180, a1181)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1182 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(v0: i64) -> Option<i64> {
    let m1188 = {
        let a1186 = 9007199254740991i64;
        let a1187 = v0;
        compare(a1186, a1187)
    };
    if if match m1188 {
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
        let m1191 = {
            let a1189 = v0;
            let a1190 = -9007199254740991i64;
            compare(a1189, a1190)
        };
        match m1191 {
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

pub fn f304(v0: Str, v1: Str) -> Adt146 {
    let a1192 = v0;
    let a1193 = v1;
    if equal(a1192, a1193) {
        Adt146::C0
    } else {
        Adt146::C1
    }
}

pub fn f305(v0: Adt135) -> bool {
    let m1194 = v0;
    match m1194 {
        Adt135::C0 => {
            false
        }
        Adt135::C1(_) => {
            true
        }
    }
}

pub fn f306(v0: Adt98, v1: Str, v2: Adt5) -> Result<Adt104, Adt100> {
    let m1195 = f182(Str::lit("params"), v2);
    match m1195 {
        Adt135::C0 => {
            Ok::<Adt104, Adt100>(Adt104::C0(v0, v1, None::<Adt5>))
        }
        Adt135::C1(v3) => {
            let m1196 = f261(v3);
            match m1196 {
                None => {
                    f138(Some(v0), Adt101::C6)
                }
                Some(v4) => {
                    Ok::<Adt104, Adt100>(Adt104::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f307(v0: Adt123) -> bool {
    let m1197 = v0;
    match m1197 {
        Adt123::C0 => {
            false
        }
        Adt123::C1 => {
            true
        }
        Adt123::C2 => {
            true
        }
        Adt123::C3 => {
            false
        }
        Adt123::C4 => {
            true
        }
        Adt123::C5 => {
            false
        }
        Adt123::C6 => {
            false
        }
        Adt123::C7 => {
            true
        }
    }
}

pub fn f308(v0: Adt119, v1: Adt124, v2: Adt120, v3: u64) -> R<Adt136> {
    let a1199 = match v0.clone() {
        Adt119::C0(_, _, h1198) => {
            h1198
        }
    };
    let a1200 = f343();
    if nat_lt(a1199, a1200) {
        Ok(Adt136::C1(Adt119::C0(v2, List::cons(v1, match v0.clone() {
            Adt119::C0(_, h1201, _) => {
                h1201
            }
        }), {
            let a1203 = match v0 {
                Adt119::C0(_, _, h1202) => {
                    h1202
                }
            };
            let a1204 = 1u64;
            nat_add(a1203, a1204)?
        })))
    } else {
        Ok(f251(Adt99::C9(v3)))
    }
}

pub fn f309(v0: bool, v1: Adt123, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt120 {
    Adt120::C8(Adt122::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f310(v0: u64) -> i64 {
    let a1205 = v0;
    let a1206 = 0u64;
    if nat_eq(a1205, a1206) {
        0i64
    } else {
        let a1207 = v0;
        let a1208 = 1u64;
        if nat_eq(a1207, a1208) {
            1i64
        } else {
            let a1209 = v0;
            let a1210 = 2u64;
            if nat_eq(a1209, a1210) {
                2i64
            } else {
                let a1211 = v0;
                let a1212 = 3u64;
                if nat_eq(a1211, a1212) {
                    3i64
                } else {
                    let a1213 = v0;
                    let a1214 = 4u64;
                    if nat_eq(a1213, a1214) {
                        4i64
                    } else {
                        let a1215 = v0;
                        let a1216 = 5u64;
                        if nat_eq(a1215, a1216) {
                            5i64
                        } else {
                            let a1217 = v0;
                            let a1218 = 6u64;
                            if nat_eq(a1217, a1218) {
                                6i64
                            } else {
                                let a1219 = v0;
                                let a1220 = 7u64;
                                if nat_eq(a1219, a1220) {
                                    7i64
                                } else {
                                    let a1221 = v0;
                                    let a1222 = 8u64;
                                    if nat_eq(a1221, a1222) {
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

pub fn f311(p1225: List<Adt6>, p1226: List<Adt6>) -> List<Adt6> {
    let m1224 = (p1225, p1226);
    {
        let mut m1224 = m1224;
        loop {
            let step = {
                let (v0, v1) = m1224;
                let m1223 = v0;
                match m1223.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Adt119, v1: u64) -> Adt136 {
    let m1228 = match v0.clone() {
        Adt119::C0(_, h1227, _) => {
            h1227
        }
    };
    match m1228.uncons() {
        None => {
            f251(Adt99::C2(v1))
        }
        Some((v2, _)) => {
            let m1229 = v2;
            match m1229 {
                Adt124::C0(_) => {
                    f250(v0, Adt120::C0)
                }
                Adt124::C1(_, _) => {
                    f250(v0, Adt120::C4)
                }
            }
        }
    }
}

pub fn f313(p1232: List<(Str, Adt6)>, p1233: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1231 = (p1232, p1233);
    {
        let mut m1231 = m1231;
        loop {
            let step = {
                let (v0, v1) = m1231;
                let m1230 = v0;
                match m1230.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
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

pub fn f314(v0: Adt119, v1: Adt120) -> Adt119 {
    Adt119::C0(v1, match v0.clone() {
        Adt119::C0(_, h1234, _) => {
            h1234
        }
    }, match v0 {
        Adt119::C0(_, _, h1235) => {
            h1235
        }
    })
}

pub fn f315(v0: Adt119, v1: u64, v2: List<u64>, v3: bool) -> Adt136 {
    let m1237 = {
        let a1236 = f207(f87(), f169(v2, List::<u64>::nil()));
        utf8_decode(a1236)
    };
    match m1237 {
        None => {
            f251(Adt99::C14)
        }
        Some(v4) => {
            if v3 {
                f396(v0, v1, v4)
            } else {
                f321(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f316(v0: u64) -> u64 {
    let a1238 = v0;
    let a1239 = 34u64;
    if nat_eq(a1238, a1239) {
        34u64
    } else {
        let a1240 = v0;
        let a1241 = 92u64;
        if nat_eq(a1240, a1241) {
            92u64
        } else {
            let a1242 = v0;
            let a1243 = 47u64;
            if nat_eq(a1242, a1243) {
                47u64
            } else {
                let a1244 = v0;
                let a1245 = 98u64;
                if nat_eq(a1244, a1245) {
                    8u64
                } else {
                    let a1246 = v0;
                    let a1247 = 102u64;
                    if nat_eq(a1246, a1247) {
                        12u64
                    } else {
                        let a1248 = v0;
                        let a1249 = 110u64;
                        if nat_eq(a1248, a1249) {
                            10u64
                        } else {
                            let a1250 = v0;
                            let a1251 = 114u64;
                            if nat_eq(a1250, a1251) {
                                13u64
                            } else {
                                let a1252 = v0;
                                let a1253 = 116u64;
                                if nat_eq(a1252, a1253) {
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

pub fn f317(v0: u64) -> u64 {
    let a1254 = v0;
    let a1255 = 48u64;
    if nat_lt(a1254, a1255) {
        16u64
    } else {
        let a1256 = v0;
        let a1257 = 58u64;
        if nat_lt(a1256, a1257) {
            let a1258 = v0;
            let a1259 = 48u64;
            nat_sub(a1258, a1259)
        } else {
            let a1260 = v0;
            let a1261 = 65u64;
            if nat_lt(a1260, a1261) {
                16u64
            } else {
                let a1262 = v0;
                let a1263 = 71u64;
                if nat_lt(a1262, a1263) {
                    let a1264 = v0;
                    let a1265 = 55u64;
                    nat_sub(a1264, a1265)
                } else {
                    let a1266 = v0;
                    let a1267 = 97u64;
                    if nat_lt(a1266, a1267) {
                        16u64
                    } else {
                        let a1268 = v0;
                        let a1269 = 103u64;
                        if nat_lt(a1268, a1269) {
                            let a1270 = v0;
                            let a1271 = 87u64;
                            nat_sub(a1270, a1271)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f318(v0: Adt119, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt136> {
    let a1272 = v3;
    let a1273 = 0u64;
    if nat_eq(a1272, a1273) {
        if f242(v2, 55296u64, 56319u64) {
            Ok(f250(v0, Adt120::C6(Adt121::C3(v2), v4, v5)))
        } else {
            if f242(v2, 56320u64, 57343u64) {
                Ok(f251(Adt99::C6(v1)))
            } else {
                Ok(f250(v0, Adt120::C6(Adt121::C0, f397(v2, v4)?, v5)))
            }
        }
    } else {
        if f242(v2, 56320u64, 57343u64) {
            Ok(f250(v0, Adt120::C6(Adt121::C0, f397({
                let a1282 = 65536u64;
                let a1283 = {
                    let a1280 = {
                        let a1276 = {
                            let a1274 = v3;
                            let a1275 = 55296u64;
                            nat_sub(a1274, a1275)
                        };
                        let a1277 = 1024u64;
                        nat_mul(a1276, a1277)?
                    };
                    let a1281 = {
                        let a1278 = v2;
                        let a1279 = 56320u64;
                        nat_sub(a1278, a1279)
                    };
                    nat_add(a1280, a1281)?
                };
                nat_add(a1282, a1283)?
            }, v4)?, v5)))
        } else {
            Ok(f251(Adt99::C6(v1)))
        }
    }
}

pub fn f319(v0: u64, v1: u64) -> u64 {
    let m1286 = {
        let a1284 = f320(v0);
        let a1285 = v1;
        index_list(a1284, a1285)
    };
    match m1286 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f320(v0: u64) -> List<u64> {
    let a1287 = v0;
    let a1288 = 0u64;
    if nat_eq(a1287, a1288) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1289 = v0;
        let a1290 = 1u64;
        if nat_eq(a1289, a1290) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f321(v0: Adt119, v1: Adt6) -> Adt136 {
    Adt136::C1(f244(v1, match v0.clone() {
        Adt119::C0(_, h1291, _) => {
            h1291
        }
    }, match v0 {
        Adt119::C0(_, _, h1292) => {
            h1292
        }
    }))
}

pub fn f322(v0: u64) -> Adt6 {
    let a1293 = v0;
    let a1294 = 0u64;
    if nat_eq(a1293, a1294) {
        Adt6::C1(true)
    } else {
        let a1295 = v0;
        let a1296 = 1u64;
        if nat_eq(a1295, a1296) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f323(v0: Adt119, v1: Adt122, v2: Adt123) -> Adt136 {
    f250(v0, Adt120::C8(Adt122::C0(match v1.clone() {
        Adt122::C0(h1297, _, _, _, _, _, _) => {
            h1297
        }
    }, v2, match v1.clone() {
        Adt122::C0(_, _, h1298, _, _, _, _) => {
            h1298
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, h1299, _, _, _) => {
            h1299
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, h1300, _, _) => {
            h1300
        }
    }, match v1.clone() {
        Adt122::C0(_, _, _, _, _, h1301, _) => {
            h1301
        }
    }, match v1 {
        Adt122::C0(_, _, _, _, _, _, h1302) => {
            h1302
        }
    })))
}

pub fn f324(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: u64 = {
        let a1304 = match v3.clone() {
            Adt122::C0(_, _, _, h1303, _, _, _) => {
                h1303
            }
        };
        let a1305 = 0u64;
        if if nat_eq(a1304, a1305) {
            let a1306 = v2;
            let a1307 = 48u64;
            nat_eq(a1306, a1307)
        } else {
            false
        } {
            0u64
        } else {
            let a1309 = match v3.clone() {
                Adt122::C0(_, _, _, h1308, _, _, _) => {
                    h1308
                }
            };
            let a1310 = 1u64;
            nat_add(a1309, a1310)?
        }
    };
    let v6: i64 = if v4 {
        f115(match v3.clone() {
            Adt122::C0(_, _, _, _, h1311, _, _) => {
                h1311
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt122::C0(_, _, _, _, h1312, _, _) => {
                h1312
            }
        }
    };
    let a1313 = f398();
    let a1314 = v5;
    if nat_lt(a1313, a1314) {
        Ok(f251(Adt99::C8(v1)))
    } else {
        let m1317 = {
            let a1315 = 99999i64;
            let a1316 = v6;
            compare(a1315, a1316)
        };
        if match m1317 {
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
            Ok(f251(Adt99::C8(v1)))
        } else {
            Ok(f250(v0, f309(match v3.clone() {
                Adt122::C0(h1318, _, _, _, _, _, _) => {
                    h1318
                }
            }, if v4 {
                Adt123::C4
            } else {
                Adt123::C2
            }, f115({
                let a1320 = match v3.clone() {
                    Adt122::C0(_, _, h1319, _, _, _, _) => {
                        h1319
                    }
                };
                let a1321 = 10i64;
                int_mul(a1320, a1321)?
            }, f310({
                let a1322 = v2;
                let a1323 = 48u64;
                nat_sub(a1322, a1323)
            }))?, v5, v6, match v3.clone() {
                Adt122::C0(_, _, _, _, _, h1324, _) => {
                    h1324
                }
            }, match v3 {
                Adt122::C0(_, _, _, _, _, _, h1325) => {
                    h1325
                }
            })))
        }
    }
}

pub fn f325(v0: u64) -> bool {
    let a1326 = v0;
    let a1327 = 101u64;
    if nat_eq(a1326, a1327) {
        true
    } else {
        let a1328 = v0;
        let a1329 = 69u64;
        nat_eq(a1328, a1329)
    }
}

pub fn f326(v0: u64) -> bool {
    f242(v0, 48u64, 57u64)
}

pub fn f327(v0: Adt119, v1: u64, v2: u64, v3: Adt122) -> R<Adt136> {
    let m1330 = f243(v3)?;
    match m1330 {
        Adt141::C0 => {
            Ok(f251(Adt99::C7(v1)))
        }
        Adt141::C1 => {
            Ok(f251(Adt99::C8(v1)))
        }
        Adt141::C2(v4) => {
            Ok(f248(f244(v4, match v0.clone() {
                Adt119::C0(_, h1331, _) => {
                    h1331
                }
            }, match v0 {
                Adt119::C0(_, _, h1332) => {
                    h1332
                }
            }), v1, v2))
        }
    }
}

pub fn f328(v0: Adt119, v1: u64, v2: u64, v3: Adt122, v4: bool) -> R<Adt136> {
    let v5: i64 = f115({
        let a1334 = match v3.clone() {
            Adt122::C0(_, _, _, _, _, _, h1333) => {
                h1333
            }
        };
        let a1335 = 10i64;
        int_mul(a1334, a1335)?
    }, f310({
        let a1336 = v2;
        let a1337 = 48u64;
        nat_sub(a1336, a1337)
    }))?;
    let m1340 = {
        let a1338 = 99999i64;
        let a1339 = v5;
        compare(a1338, a1339)
    };
    if match m1340 {
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
        Ok(f251(Adt99::C8(v1)))
    } else {
        Ok(f250(v0, f309(match v3.clone() {
            Adt122::C0(h1341, _, _, _, _, _, _) => {
                h1341
            }
        }, Adt123::C7, match v3.clone() {
            Adt122::C0(_, _, h1342, _, _, _, _) => {
                h1342
            }
        }, match v3.clone() {
            Adt122::C0(_, _, _, h1343, _, _, _) => {
                h1343
            }
        }, match v3 {
            Adt122::C0(_, _, _, _, h1344, _, _) => {
                h1344
            }
        }, v4, v5)))
    }
}

pub fn f329(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1345 = f399(f400(), v2.clone());
    match m1345 {
        None => {
            let m1346 = f401(f402(), v2.clone());
            match m1346 {
                None => {
                    f403(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt97::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt97::C4(v0, v3))
        }
    }
}

pub fn f330(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt125) -> R<Adt147> {
    let m1347 = f238(Str::lit("x-mcp-header"), v1.clone());
    match m1347 {
        Adt135::C0 => {
            Ok(Adt147::C1(v4))
        }
        Adt135::C1(v5) => {
            f404(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f331(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt105>) -> List<Adt105> {
    f405(Str::lit("properties"), v0.clone(), v1, v2, f405(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f406(Str::lit("items"), v0.clone(), f406(Str::lit("additionalProperties"), v0.clone(), f407(Str::lit("allOf"), v0.clone(), f407(Str::lit("anyOf"), v0.clone(), f407(Str::lit("oneOf"), v0.clone(), f406(Str::lit("not"), v0.clone(), f406(Str::lit("if"), v0.clone(), f406(Str::lit("then"), v0.clone(), f406(Str::lit("else"), v0, v3)))))))))))
}

pub fn f332(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt137> {
    Ok(f408(List::cons((Str::lit("type"), f409(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f410(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f411(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f412(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f412(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f412(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f412(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1348 = f413(v0.clone(), v1.clone())?;
        let a1349 = List::cons((Str::lit("required"), f414(v0, v1)), List::<(Str, Adt148)>::nil());
        append_list(a1348, a1349)
    })))))))))
}

pub fn f333(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> R<Adt137> {
    Ok(f415(f416(v0, v1.clone(), v2.clone(), v3.clone()), f415(f417(v1.clone(), v2.clone(), v3.clone()), f415(f418(v1.clone(), v2.clone(), v3.clone()), f415(f419(v1.clone(), v2.clone(), v3.clone())?, f415(f420(v1.clone(), v2.clone(), v3.clone()), f415(f421(Adt110::C0, f266(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f415(f421(Adt110::C1, f267(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f415(f421(Adt110::C2, f268(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f421(Adt110::C3, f269(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f334(v0: Adt6) -> Option<List<Adt6>> {
    let m1350 = v0;
    match m1350 {
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

pub fn f335(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1351 = f238(v0, v1);
    match m1351 {
        Adt135::C0 => {
            None::<Adt6>
        }
        Adt135::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f336(v0: List<Adt51>) -> R<bool> {
    let m1352 = v0;
    match m1352.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f422(v2, match v1 {
                Adt51::C0(h1353, _, _) => {
                    h1353
                }
            })
        }
    }
}

pub fn f337(v0: Str) -> R<Str> {
    let m1356 = {
        let a1355 = f207(f87(), f169(f423(f85({
            let a1354 = v0.clone();
            utf8_encode(a1354)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1355)
    };
    match m1356 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f338(p1365: Bytes, p1366: List<Bytes>) -> List<Bytes> {
    let m1364 = (p1365, p1366);
    {
        let mut m1364 = m1364;
        loop {
            let step = {
                let (v0, v1) = m1364;
                let m1357 = v1;
                match m1357.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1360 = {
                            let a1358 = v2.clone();
                            length_bytes(a1358)
                        };
                        let a1361 = {
                            let a1359 = v0.clone();
                            length_bytes(a1359)
                        };
                        if nat_eq(a1360, a1361) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1362 = v2;
                                let a1363 = v0;
                                append_bytes(a1362, a1363)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1364 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: u64) -> Bytes {
    let m1370 = {
        let a1367 = f424();
        let a1368 = v0;
        let a1369 = 1u64;
        slice_bytes(a1367, a1368, a1369)
    };
    match m1370 {
        None => {
            f87()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f340(v0: u64) -> List<u64> {
    f320(v0)
}

pub fn f341(v0: Adt7) -> R<bool> {
    let a1388 = {
        let m1374 = {
            let a1372 = f425();
            let a1373 = match v0.clone() {
                Adt7::C0(h1371, _) => {
                    h1371
                }
            };
            compare(a1372, a1373)
        };
        if match m1374 {
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
            let m1379 = {
                let a1377 = match v0.clone() {
                    Adt7::C0(h1375, _) => {
                        h1375
                    }
                };
                let a1378 = {
                    let a1376 = f425();
                    int_neg(a1376)?
                };
                compare(a1377, a1378)
            };
            if match m1379 {
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
                let m1383 = {
                    let a1381 = 9999i64;
                    let a1382 = match v0.clone() {
                        Adt7::C0(_, h1380) => {
                            h1380
                        }
                    };
                    compare(a1381, a1382)
                };
                if match m1383 {
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
                    let m1387 = {
                        let a1385 = match v0 {
                            Adt7::C0(_, h1384) => {
                                h1384
                            }
                        };
                        let a1386 = -9999i64;
                        compare(a1385, a1386)
                    };
                    match m1387 {
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
    Ok(bool_not(a1388))
}

pub fn f342(v0: Adt7) -> R<List<u64>> {
    let m1392 = {
        let a1390 = match v0.clone() {
            Adt7::C0(_, h1389) => {
                h1389
            }
        };
        let a1391 = 0i64;
        compare(a1390, a1391)
    };
    if if match m1392 {
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
        let m1396 = {
            let a1394 = 0i64;
            let a1395 = match v0.clone() {
                Adt7::C0(_, h1393) => {
                    h1393
                }
            };
            compare(a1394, a1395)
        };
        match m1396 {
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
        let a1399 = f426(match v0.clone() {
            Adt7::C0(h1397, _) => {
                h1397
            }
        })?;
        let a1400 = List::cons(101u64, f426(match v0 {
            Adt7::C0(_, h1398) => {
                h1398
            }
        })?);
        Ok(append_list(a1399, a1400))
    } else {
        f426(match v0 {
            Adt7::C0(h1401, _) => {
                h1401
            }
        })
    }
}

pub fn f343() -> u64 {
    512u64
}

pub fn f344(v0: u64, v1: Adt111, v2: List<Adt111>) -> List<Adt111> {
    List::cons(Adt111::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f345(p1404: List<u64>, p1405: List<u64>) -> R<List<u64>> {
    let m1403 = (p1404, p1405);
    {
        let mut m1403 = m1403;
        loop {
            let step = {
                let (v0, v1) = m1403;
                let m1402 = v0;
                match m1402.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f169(f427(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1403 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f428(match v0.clone() {
        Adt34::C0(_, h1406, _, _) => {
            h1406
        }
    })), List::cons((Str::lit("capabilities"), f429(match v0.clone() {
        Adt34::C0(_, _, h1407, _) => {
            h1407
        }
    })), {
        let a1410 = f430(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1408) => {
                h1408
            }
        });
        let a1411 = f431(match v0 {
            Adt34::C0(h1409, _, _, _) => {
                h1409
            }
        })?;
        append_list(a1410, a1411)
    })))
}

pub fn f347(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f432(match v0.clone() {
        Adt35::C0(_, _, h1412) => {
            h1412
        }
    }))), {
        let a1415 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1413, _) => {
                h1413
            }
        });
        let a1416 = f431(match v0 {
            Adt35::C0(h1414, _, _) => {
                h1414
            }
        })?;
        append_list(a1415, a1416)
    }))
}

pub fn f348(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1417 = v0;
    match m1417 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1418 = f367(Adt6::C5(v1))?;
            match m1418 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1419 = f261(v2);
                    match m1419 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1420) => {
                                    h1420
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f349(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m1422 = f433(match v0.clone() {
        Adt38::C0(h1421, _, _) => {
            h1421
        }
    })?;
    match m1422 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1425 = f434(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h1423, _) => {
                        h1423
                    }
                });
                let a1426 = f435(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h1424) => {
                        h1424
                    }
                });
                append_list(a1425, a1426)
            })))
        }
    }
}

pub fn f350(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f436(match v0.clone() {
        Adt52::C0(_, _, h1427) => {
            h1427
        }
    })?)), {
        let a1430 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1428, _) => {
                h1428
            }
        });
        let a1431 = f431(match v0 {
            Adt52::C0(h1429, _, _) => {
                h1429
            }
        })?;
        append_list(a1430, a1431)
    }))
}

pub fn f351(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f437(match v0.clone() {
        Adt53::C0(_, h1432) => {
            h1432
        }
    }))), f431(match v0 {
        Adt53::C0(h1433, _) => {
            h1433
        }
    })?))
}

pub fn f352(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f438(match v0.clone() {
        Adt54::C0(_, _, h1434) => {
            h1434
        }
    }))), {
        let a1437 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1435, _) => {
                h1435
            }
        });
        let a1438 = f431(match v0 {
            Adt54::C0(h1436, _, _) => {
                h1436
            }
        })?;
        append_list(a1437, a1438)
    }))
}

pub fn f353(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f439(match v0.clone() {
        Adt56::C0(_, _, h1439) => {
            h1439
        }
    }))), {
        let a1442 = f430(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1440, _) => {
                h1440
            }
        });
        let a1443 = f431(match v0 {
            Adt56::C0(h1441, _, _) => {
                h1441
            }
        })?;
        append_list(a1442, a1443)
    }))
}

pub fn f354(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1445 = f440(match v0.clone() {
        Adt59::C0(_, h1444) => {
            h1444
        }
    })?;
    match m1445 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1447 = f430(Str::lit("description"), match v0 {
                    Adt59::C0(h1446, _) => {
                        h1446
                    }
                });
                let a1448 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1447, a1448)
            }))
        }
    }
}

pub fn f355(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f379({
        let m1451 = match v0.clone() {
            Adt61::C0(h1450) => {
                h1450
            }
        };
        match m1451 {
            Adt62::C0(h1449, _, _) => {
                h1449
            }
        }
    }))), {
        let a1458 = f441(Str::lit("total"), {
            let m1454 = match v0.clone() {
                Adt61::C0(h1453) => {
                    h1453
                }
            };
            match m1454 {
                Adt62::C0(_, h1452, _) => {
                    h1452
                }
            }
        })?;
        let a1459 = f435(Str::lit("hasMore"), {
            let m1457 = match v0 {
                Adt61::C0(h1456) => {
                    h1456
                }
            };
            match m1457 {
                Adt62::C0(_, _, h1455) => {
                    h1455
                }
            }
        });
        append_list(a1458, a1459)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f356(v0: Adt82) -> Option<List<(Str, Adt6)>> {
    let m1460 = f442(f443(v0.clone()));
    match m1460 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1462 = {
                    let m1461 = f443(v0.clone());
                    match m1461.uncons() {
                        None => {
                            List::<(Str, Adt6)>::nil()
                        }
                        Some((_, _)) => {
                            List::cons((Str::lit("inputRequests"), v1), List::<(Str, Adt6)>::nil())
                        }
                    }
                };
                let a1463 = f444(Str::lit("requestState"), f445(v0));
                append_list(a1462, a1463)
            })
        }
    }
}

pub fn f357(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1464 = f446(v0.clone());
    match m1464 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1465 = f447(f448(v0.clone()), f449(f450(v0)));
                let a1466 = v1;
                append_list(a1465, a1466)
            })
        }
    }
}

pub fn f358(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1468 = match v0.clone() {
        Adt88::C0(_, h1467, _) => {
            h1467
        }
    };
    match m1468 {
        Adt89::C0 => {
            f357({
                let m1471 = match v0 {
                    Adt88::C0(h1470, _, _) => {
                        h1470
                    }
                };
                match m1471 {
                    Adt85::C0(h1469, _, _) => {
                        h1469
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f359(v0: Adt149) -> Str {
    let m1472 = v0;
    match m1472 {
        Adt149::C0 => {
            Str::lit("complete")
        }
        Adt149::C1 => {
            Str::lit("input_required")
        }
        Adt149::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f360(v0: Adt33) -> Adt149 {
    let m1473 = v0;
    match m1473 {
        Adt33::C0(_) => {
            Adt149::C0
        }
        Adt33::C1(_) => {
            Adt149::C0
        }
        Adt33::C2(_) => {
            Adt149::C0
        }
        Adt33::C3(_) => {
            Adt149::C0
        }
        Adt33::C4(_) => {
            Adt149::C0
        }
        Adt33::C5(_) => {
            Adt149::C0
        }
        Adt33::C6(_) => {
            Adt149::C0
        }
        Adt33::C7(_) => {
            Adt149::C0
        }
        Adt33::C8(_) => {
            Adt149::C0
        }
        Adt33::C9(_) => {
            Adt149::C0
        }
        Adt33::C10(_) => {
            Adt149::C0
        }
        Adt33::C11 => {
            Adt149::C0
        }
        Adt33::C12(_) => {
            Adt149::C1
        }
        Adt33::C13(_) => {
            Adt149::C2
        }
        Adt33::C14(_) => {
            Adt149::C0
        }
    }
}

pub fn f361(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1474 = v0;
    match m1474 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f451(), f452(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f362(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a1476 = {
        let a1475 = v0.clone();
        length_bytes(a1475)
    };
    let a1477 = 0u64;
    if nat_eq(a1476, a1477) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m1478 = f68(v0)?;
        match m1478 {
            Ok(v2) => {
                let m1479 = f334(v2);
                match m1479 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f453(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f363(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m1480 = f454(v0);
    match m1480 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1481 = f89(v1)?;
            match m1481 {
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

pub fn f364(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f455(f456(f457(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f365(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f458(v2) {
            let a1482 = v1;
            let a1483 = 200u64;
            nat_eq(a1482, a1483)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f366(v0: u64, v1: List<List<u64>>) -> Adt94 {
    Adt94::C0(v0, f459(), List::<List<u64>>::nil(), f460(v1), None::<List<u64>>)
}

pub fn f367(v0: Adt6) -> R<Option<Adt6>> {
    f461(f462(), Adt150::C0(v0), List::<Adt151>::nil())
}

pub fn f368(v0: Adt130) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f129())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f463(match v0.clone() {
        Adt130::C0(h1484, _) => {
            h1484
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1487 = match v0.clone() {
            Adt130::C0(_, h1486) => {
                h1486
            }
        };
        match m1487 {
            Adt51::C0(h1485, _, _) => {
                h1485
            }
        }
    })), {
        let a1494 = f464({
            let m1490 = match v0.clone() {
                Adt130::C0(_, h1489) => {
                    h1489
                }
            };
            match m1490 {
                Adt51::C0(_, h1488, _) => {
                    h1488
                }
            }
        });
        let a1495 = f465({
            let m1493 = match v0 {
                Adt130::C0(_, h1492) => {
                    h1492
                }
            };
            match m1493 {
                Adt51::C0(_, _, h1491) => {
                    h1491
                }
            }
        });
        append_list(a1494, a1495)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f369(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h1496) => {
            h1496
        }
    } {
        let m1498 = match v0 {
            Adt88::C0(_, h1497, _) => {
                h1497
            }
        };
        match m1498 {
            Adt89::C0 => {
                true
            }
            Adt89::C1 => {
                true
            }
        }
    } else {
        false
    }
}

pub fn f370(v0: Adt63) -> Option<Adt6> {
    let m1499 = f357(v0);
    match m1499 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f371() -> u64 {
    429u64
}

pub fn f372() -> i64 {
    -32602i64
}

pub fn f373() -> i64 {
    -32022i64
}

pub fn f374() -> i64 {
    -32020i64
}

pub fn f375() -> i64 {
    -32601i64
}

pub fn f376(v0: Adt90) -> i64 {
    let m1500 = v0;
    match m1500 {
        Adt90::C0(_) => {
            f372()
        }
        Adt90::C1(_) => {
            f372()
        }
        Adt90::C2(_) => {
            f372()
        }
        Adt90::C3(_) => {
            f372()
        }
        Adt90::C4(_) => {
            f372()
        }
        Adt90::C5(_) => {
            f372()
        }
        Adt90::C6(_) => {
            f466()
        }
        Adt90::C7(_) => {
            f467()
        }
        Adt90::C8(_) => {
            f468()
        }
        Adt90::C9(_) => {
            f466()
        }
    }
}

pub fn f377(v0: Adt132) -> Str {
    let m1501 = v0;
    match m1501 {
        Adt132::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt132::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt132::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt132::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt132::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt132::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt132::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt132::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt132::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f378(v0: Adt113) -> Str {
    let m1502 = v0;
    match m1502 {
        Adt113::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt113::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt113::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt113::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt113::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt113::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt113::C6(v1) => {
            let m1503 = v1;
            match m1503 {
                Adt114::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt114::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt114::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt113::C7(v2) => {
            let m1504 = v2;
            match m1504 {
                Adt114::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt114::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt114::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt113::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt113::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt113::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f379(v0: List<Str>) -> List<Adt6> {
    f311(f469(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f380(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a1516 = f470(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h1505, _, _, _, _) => {
                h1505
            }
        });
        let a1517 = {
            let a1514 = f470(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h1506, _, _, _) => {
                    h1506
                }
            });
            let a1515 = {
                let a1512 = f470(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h1507, _, _) => {
                        h1507
                    }
                });
                let a1513 = {
                    let a1510 = f470(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h1508, _) => {
                            h1508
                        }
                    });
                    let a1511 = f470(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h1509) => {
                            h1509
                        }
                    });
                    append_list(a1510, a1511)
                };
                append_list(a1512, a1513)
            };
            append_list(a1514, a1515)
        };
        append_list(a1516, a1517)
    })
}

pub fn f381(p1520: List<List<u64>>) -> R<bool> {
    let m1519 = p1520;
    {
        let mut m1519 = m1519;
        loop {
            let step = {
                let v0 = m1519;
                let m1518 = v0;
                match m1518.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f471(f144(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f383(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139, v4: bool) -> R<Result<Adt128, Adt131>> {
    if v4 {
        f472(v0, v1, v2, f473(match v3.clone() {
            Adt139::C0(_, h1521, _, _) => {
                h1521
            }
        }, match v3.clone() {
            Adt139::C0(_, _, h1522, _) => {
                h1522
            }
        }, match v3 {
            Adt139::C0(_, _, _, h1523) => {
                h1523
            }
        }))
    } else {
        Ok(Err::<Adt128, Adt131>(Adt131::C3(f474(match v3 {
            Adt139::C0(_, h1524, _, _) => {
                h1524
            }
        }))))
    }
}

pub fn f384(v0: Adt4, v1: Adt140) -> bool {
    let m1525 = v1;
    match m1525 {
        Adt140::C0 => {
            true
        }
        Adt140::C1 => {
            f475(match v0 {
                Adt4::C0(_, _, _, _, _, h1526, _) => {
                    h1526
                }
            })
        }
        Adt140::C2 => {
            f475(match v0 {
                Adt4::C0(_, _, _, _, _, h1527, _) => {
                    h1527
                }
            })
        }
        Adt140::C3 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1528, _, _) => {
                    h1528
                }
            })
        }
        Adt140::C4 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1529, _, _) => {
                    h1529
                }
            })
        }
        Adt140::C5 => {
            f476(match v0 {
                Adt4::C0(_, _, _, _, h1530, _, _) => {
                    h1530
                }
            })
        }
        Adt140::C6 => {
            f475(match v0 {
                Adt4::C0(_, _, _, h1531, _, _, _) => {
                    h1531
                }
            })
        }
        Adt140::C7 => {
            f475(match v0 {
                Adt4::C0(_, _, _, h1532, _, _, _) => {
                    h1532
                }
            })
        }
        Adt140::C8 => {
            f477(match v0 {
                Adt4::C0(_, _, h1533, _, _, _, _) => {
                    h1533
                }
            })
        }
        Adt140::C9 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1534) => {
                    h1534
                }
            }, f26())
        }
        Adt140::C10 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1535) => {
                    h1535
                }
            }, f26())
        }
        Adt140::C11 => {
            f478(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1536) => {
                    h1536
                }
            }, f26())
        }
    }
}

pub fn f385(v0: Adt144, v1: Adt142) -> Result<Adt139, Adt131> {
    let m1537 = v1;
    match m1537 {
        Adt142::C0 => {
            Err::<Adt139, Adt131>(Adt131::C3(match v0 {
                Adt144::C0(_, h1538, _, _) => {
                    h1538
                }
            }))
        }
        Adt142::C1(v2) => {
            Ok::<Adt139, Adt131>(Adt139::C0(match v0.clone() {
                Adt144::C0(h1539, _, _, _) => {
                    h1539
                }
            }, v2, match v0.clone() {
                Adt144::C0(_, _, h1540, _) => {
                    h1540
                }
            }, match v0 {
                Adt144::C0(_, _, _, h1541) => {
                    h1541
                }
            }))
        }
    }
}

pub fn f386(v0: Option<Adt112>, v1: List<Str>, v2: Adt104, v3: Result<Adt17, Adt132>) -> R<Adt143> {
    let m1542 = v3;
    match m1542 {
        Ok(v5) => {
            Ok(f479(v1, v2.clone(), v5.clone(), f480(v0, f297(match v2.clone() {
                Adt104::C0(_, h1543, _) => {
                    h1543
                }
            }), match v2.clone() {
                Adt104::C0(_, h1544, _) => {
                    h1544
                }
            }, match v5 {
                Adt17::C0(h1545, _, _, _, _, _) => {
                    h1545
                }
            }, f481(match v2 {
                Adt104::C0(_, _, h1546) => {
                    h1546
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt143::C1(Adt131::C0(v4)))
        }
    }
}

pub fn f387(v0: Option<Adt5>) -> Result<Adt17, Adt132> {
    let m1547 = v0;
    match m1547 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C0)
        }
        Some(v1) => {
            let m1548 = f182(Str::lit("_meta"), v1);
            match m1548 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C1)
                }
                Adt135::C1(v2) => {
                    let m1549 = f261(v2);
                    match m1549 {
                        None => {
                            Err::<Adt17, Adt132>(Adt132::C4)
                        }
                        Some(v3) => {
                            f482(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f388(v0: Adt140) -> Adt145 {
    let m1550 = v0;
    match m1550 {
        Adt140::C0 => {
            Adt145::C0
        }
        Adt140::C1 => {
            Adt145::C0
        }
        Adt140::C2 => {
            Adt145::C1
        }
        Adt140::C3 => {
            Adt145::C0
        }
        Adt140::C4 => {
            Adt145::C2
        }
        Adt140::C5 => {
            Adt145::C0
        }
        Adt140::C6 => {
            Adt145::C0
        }
        Adt140::C7 => {
            Adt145::C1
        }
        Adt140::C8 => {
            Adt145::C0
        }
        Adt140::C9 => {
            Adt145::C3
        }
        Adt140::C10 => {
            Adt145::C3
        }
        Adt140::C11 => {
            Adt145::C3
        }
    }
}

pub fn f389(p1556: List<u64>, p1557: List<u64>) -> Option<List<u64>> {
    let m1555 = (p1556, p1557);
    {
        let mut m1555 = m1555;
        loop {
            let step = {
                let (v0, v1) = m1555;
                let m1551 = v0;
                match m1551.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1552 = v1;
                        match m1552.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1553 = v4;
                                let a1554 = v2;
                                if nat_eq(a1553, a1554) {
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
                Ok(next) => { m1555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f391() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f392(p1566: u64, p1567: List<u64>, p1568: List<u64>) -> R<Option<List<u64>>> {
    let m1565 = (p1566, (p1567, p1568));
    {
        let mut m1565 = m1565;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1565;
                let m1558 = v0;
                if m1558 == 0 {
                    let m1559 = v1;
                    match m1559.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1558 - 1;
                    let m1560 = v1;
                    match m1560.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1561 = v7;
                            match m1561.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1562 = v9;
                                    match m1562.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1563 = v11;
                                            match m1563.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1564 = f483(v6, v8, v10, v12, f484(v13.clone()))?;
                                                    match m1564 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f169(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: Adt12, v1: Option<Str>) -> bool {
    let m1569 = v1;
    match m1569 {
        None => {
            false
        }
        Some(v2) => {
            let a1571 = match v0 {
                Adt12::C0(h1570, _, _, _) => {
                    h1570
                }
            };
            let a1572 = v2;
            equal(a1571, a1572)
        }
    }
}

pub fn f394(v0: List<Adt92>, v1: Str) -> R<Adt116> {
    let m1575 = f230(f70(v0, {
        let a1573 = f485();
        let a1574 = f486(v1)?;
        append_list(a1573, a1574)
    })?)?;
    match m1575 {
        Adt102::C0 => {
            Ok(Adt116::C0)
        }
        Adt102::C1 => {
            Ok(Adt116::C1)
        }
        Adt102::C2 => {
            Ok(Adt116::C1)
        }
        Adt102::C3(v2) => {
            let m1576 = f166(v2);
            match m1576 {
                None => {
                    Ok(Adt116::C1)
                }
                Some(v3) => {
                    Ok(Adt116::C2(v3))
                }
            }
        }
    }
}

pub fn f395(v0: i64) -> bool {
    let a1583 = {
        let m1579 = {
            let a1577 = v0;
            let a1578 = 0i64;
            compare(a1577, a1578)
        };
        if match m1579 {
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
            let m1582 = {
                let a1580 = 0i64;
                let a1581 = v0;
                compare(a1580, a1581)
            };
            match m1582 {
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
    bool_not(a1583)
}

pub fn f396(v0: Adt119, v1: u64, v2: Str) -> Adt136 {
    let m1585 = match v0.clone() {
        Adt119::C0(_, h1584, _) => {
            h1584
        }
    };
    match m1585.uncons() {
        None => {
            f251(Adt99::C2(v1))
        }
        Some((v3, v4)) => {
            let m1586 = v3;
            match m1586 {
                Adt124::C0(_) => {
                    f251(Adt99::C2(v1))
                }
                Adt124::C1(v6, _) => {
                    Adt136::C1(Adt119::C0(Adt120::C5, List::cons(Adt124::C1(v6, Some(v2)), v4), match v0 {
                        Adt119::C0(_, _, h1587) => {
                            h1587
                        }
                    }))
                }
            }
        }
    }
}

pub fn f397(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1588 = v0;
    let a1589 = 128u64;
    if nat_lt(a1588, a1589) {
        Ok(List::cons(v0, v1))
    } else {
        let a1590 = v0;
        let a1591 = 2048u64;
        if nat_lt(a1590, a1591) {
            Ok(List::cons({
                let a1592 = 128u64;
                let a1593 = f487(v0, 64u64);
                nat_add(a1592, a1593)?
            }, List::cons({
                let a1594 = 192u64;
                let a1595 = f488(v0, 64u64);
                nat_add(a1594, a1595)?
            }, v1)))
        } else {
            let a1596 = v0;
            let a1597 = 65536u64;
            if nat_lt(a1596, a1597) {
                Ok(List::cons({
                    let a1598 = 128u64;
                    let a1599 = f487(v0, 64u64);
                    nat_add(a1598, a1599)?
                }, List::cons({
                    let a1600 = 128u64;
                    let a1601 = f487(f488(v0, 64u64), 64u64);
                    nat_add(a1600, a1601)?
                }, List::cons({
                    let a1602 = 224u64;
                    let a1603 = f488(v0, 4096u64);
                    nat_add(a1602, a1603)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1604 = 128u64;
                    let a1605 = f487(v0, 64u64);
                    nat_add(a1604, a1605)?
                }, List::cons({
                    let a1606 = 128u64;
                    let a1607 = f487(f488(v0, 64u64), 64u64);
                    nat_add(a1606, a1607)?
                }, List::cons({
                    let a1608 = 128u64;
                    let a1609 = f487(f488(v0, 4096u64), 64u64);
                    nat_add(a1608, a1609)?
                }, List::cons({
                    let a1610 = 240u64;
                    let a1611 = f488(v0, 262144u64);
                    nat_add(a1610, a1611)?
                }, v1)))))
            }
        }
    }
}

pub fn f398() -> u64 {
    18u64
}

pub fn f399(p1615: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1614 = p1615;
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let v0 = m1614;
                let m1612 = v0;
                match m1612.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1613 = f238(v2.clone(), v1.clone());
                        match m1613 {
                            Adt135::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt135::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
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

pub fn f400() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f401(p1622: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1621 = p1622;
    {
        let mut m1621 = m1621;
        loop {
            let step = {
                let v0 = m1621;
                let m1616 = v0;
                match m1616.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1618 = f238({
                            let (h1617, _) = v2.clone();
                            h1617
                        }, v1.clone());
                        match m1618 {
                            Adt135::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt135::C1(v4) => {
                                if f489({
                                    let (_, h1619) = v2;
                                    h1619
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1620, _) = v2;
                                        h1620
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1621 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f403(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt97> {
    let m1623 = f238(Str::lit("$ref"), v2);
    match m1623 {
        Adt135::C0 => {
            None::<Adt97>
        }
        Adt135::C1(v3) => {
            let m1624 = f235(v3);
            match m1624 {
                None => {
                    None::<Adt97>
                }
                Some(v4) => {
                    let m1625 = f490(v1, v4.clone());
                    match m1625 {
                        None => {
                            Some(Adt97::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt97>
                        }
                    }
                }
            }
        }
    }
}

pub fn f404(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt125) -> R<Adt147> {
    if v3 {
        let m1626 = f235(v2);
        match m1626 {
            None => {
                Ok(Adt147::C0(Adt97::C7(v0)))
            }
            Some(v6) => {
                f491(v0, v1, v6.clone(), f113(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt147::C0(Adt97::C10(v0)))
    }
}

pub fn f405(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt105>) -> List<Adt105> {
    let m1627 = f238(v0.clone(), v1);
    match m1627 {
        Adt135::C0 => {
            v4
        }
        Adt135::C1(v5) => {
            let m1628 = f261(v5);
            match m1628 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1629 = v0;
                    let a1630 = Str::lit("properties");
                    if equal(a1629, a1630) {
                        f492(match v6 {
                            Adt5::C0(h1631) => {
                                h1631
                            }
                        }, v2, v3, v4)
                    } else {
                        f493(match v6 {
                            Adt5::C0(h1632) => {
                                h1632
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f406(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1633 = f238(v0, v1);
    match m1633 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            List::cons(Adt105::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f407(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt105>) -> List<Adt105> {
    let m1634 = f238(v0, v1);
    match m1634 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            let m1635 = f334(v3);
            match m1635 {
                None => {
                    v2
                }
                Some(v4) => {
                    f494(v4, v2)
                }
            }
        }
    }
}

pub fn f408(p1642: List<(Str, Adt148)>) -> Adt137 {
    let m1641 = p1642;
    {
        let mut m1641 = m1641;
        loop {
            let step = {
                let v0 = m1641;
                let m1636 = v0;
                match m1636.uncons() {
                    None => {
                        Err::<List<(Str, Adt148)>, Adt137>(Adt137::C3(List::<Adt109>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1638 = {
                            let (_, h1637) = v1;
                            h1637
                        };
                        match m1638 {
                            Adt148::C0 => {
                                Ok::<List<(Str, Adt148)>, Adt137>(v2)
                            }
                            Adt148::C1 => {
                                Err::<List<(Str, Adt148)>, Adt137>(Adt137::C0({
                                    let (h1639, _) = v1;
                                    h1639
                                }))
                            }
                            Adt148::C2 => {
                                Err::<List<(Str, Adt148)>, Adt137>(Adt137::C1({
                                    let (h1640, _) = v1;
                                    h1640
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1643 = f238(Str::lit("type"), v0);
    match m1643 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1644 = f235(v2.clone());
            match m1644 {
                None => {
                    let m1645 = f334(v2);
                    match m1645 {
                        None => {
                            Ok(Adt148::C2)
                        }
                        Some(v4) => {
                            f497(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f495(v3.clone()) {
                        if f496(v3, v1)? {
                            Ok(Adt148::C0)
                        } else {
                            Ok(Adt148::C1)
                        }
                    } else {
                        Ok(Adt148::C2)
                    }
                }
            }
        }
    }
}

pub fn f410(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1646 = f238(Str::lit("enum"), v0);
    match m1646 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1647 = f334(v2);
            match m1647 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v3) => {
                    f498(v3, v1)
                }
            }
        }
    }
}

pub fn f411(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt148> {
    let m1648 = f238(Str::lit("const"), v0);
    match m1648 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v2) => {
            let m1649 = f499(v2, v1)?;
            match m1649 {
                Adt152::C0 => {
                    Ok(Adt148::C0)
                }
                Adt152::C1 => {
                    Ok(Adt148::C1)
                }
                Adt152::C2 => {
                    Ok(Adt148::C2)
                }
            }
        }
    }
}

pub fn f412(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt148> {
    let m1650 = f238(v0, v2);
    match m1650 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v4) => {
            let m1651 = f500(v4);
            match m1651 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v5) => {
                    let m1652 = f500(v3);
                    match m1652 {
                        None => {
                            Ok(Adt148::C0)
                        }
                        Some(v6) => {
                            if f501(v1, f502(v6, v5)?) {
                                Ok(Adt148::C0)
                            } else {
                                Ok(Adt148::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f413(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt148)>> {
    let m1653 = v1;
    match m1653 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt148)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f503(Str::lit("minLength"), true, v0.clone(), f504(v4.clone())?)?), List::cons((Str::lit("maxLength"), f503(Str::lit("maxLength"), false, v0, f504(v4)?)?), List::<(Str, Adt148)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f503(Str::lit("minItems"), true, v0.clone(), f505(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f503(Str::lit("maxItems"), false, v0, f505(v5, 0u64)?)?), List::<(Str, Adt148)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f503(Str::lit("minProperties"), true, v0.clone(), f506(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f503(Str::lit("maxProperties"), false, v0, f506(v6, 0u64)?)?), List::<(Str, Adt148)>::nil())))
        }
    }
}

pub fn f414(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt148 {
    let m1654 = f238(Str::lit("required"), v0);
    match m1654 {
        Adt135::C0 => {
            Adt148::C0
        }
        Adt135::C1(v2) => {
            let m1655 = f334(v2);
            match m1655 {
                None => {
                    Adt148::C2
                }
                Some(v3) => {
                    let m1656 = f261(v1);
                    match m1656 {
                        None => {
                            Adt148::C0
                        }
                        Some(v4) => {
                            f507(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f415(v0: Adt137, v1: Adt137) -> Adt137 {
    let m1657 = v0.clone();
    match m1657 {
        Adt137::C0(_) => {
            v0
        }
        Adt137::C1(_) => {
            v0
        }
        Adt137::C2(_) => {
            v0
        }
        Adt137::C3(v5) => {
            let m1658 = v1.clone();
            match m1658 {
                Adt137::C0(_) => {
                    v1
                }
                Adt137::C1(_) => {
                    v1
                }
                Adt137::C2(_) => {
                    v1
                }
                Adt137::C3(v9) => {
                    Adt137::C3({
                        let a1659 = v5;
                        let a1660 = v9;
                        append_list(a1659, a1660)
                    })
                }
            }
        }
    }
}

pub fn f416(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>) -> Adt137 {
    let m1661 = f238(Str::lit("$ref"), v1);
    match m1661 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v4) => {
            let m1662 = f235(v4);
            match m1662 {
                None => {
                    Adt137::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1663 = f490(v0, v5.clone());
                    match m1663 {
                        None => {
                            Adt137::C2(v5)
                        }
                        Some(v6) => {
                            Adt137::C3(List::cons(Adt109::C0(v6, v2, v3), List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f417(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1664 = f238(Str::lit("properties"), v0);
    match m1664 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1665 = f261(v3);
            match m1665 {
                None => {
                    Adt137::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1666 = f261(v1);
                    match m1666 {
                        None => {
                            Adt137::C3(List::<Adt109>::nil())
                        }
                        Some(v5) => {
                            Adt137::C3(f508(match v4 {
                                Adt5::C0(h1667) => {
                                    h1667
                                }
                            }, match v5 {
                                Adt5::C0(h1668) => {
                                    h1668
                                }
                            }, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f418(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1669 = f238(Str::lit("additionalProperties"), v0.clone());
    match m1669 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1670 = f261(v1);
            match m1670 {
                None => {
                    Adt137::C3(List::<Adt109>::nil())
                }
                Some(v4) => {
                    let m1671 = f260(v3.clone());
                    match m1671 {
                        None => {
                            Adt137::C3(f511(match v4 {
                                Adt5::C0(h1673) => {
                                    h1673
                                }
                            }, f510(v0), v3, v2, List::<Adt109>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt137::C3(List::<Adt109>::nil())
                            } else {
                                if f509(match v4 {
                                    Adt5::C0(h1672) => {
                                        h1672
                                    }
                                }, f510(v0)) {
                                    Adt137::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt137::C3(List::<Adt109>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f419(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> R<Adt137> {
    let m1674 = f238(Str::lit("items"), v0);
    match m1674 {
        Adt135::C0 => {
            Ok(Adt137::C3(List::<Adt109>::nil()))
        }
        Adt135::C1(v3) => {
            let m1675 = f334(v1);
            match m1675 {
                None => {
                    Ok(Adt137::C3(List::<Adt109>::nil()))
                }
                Some(v4) => {
                    Ok(Adt137::C3(f512(v4, 0u64, v3, v2, List::<Adt109>::nil())?))
                }
            }
        }
    }
}

pub fn f420(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt107>) -> Adt137 {
    let m1676 = f238(Str::lit("allOf"), v0);
    match m1676 {
        Adt135::C0 => {
            Adt137::C3(List::<Adt109>::nil())
        }
        Adt135::C1(v3) => {
            let m1677 = f334(v3);
            match m1677 {
                None => {
                    Adt137::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1678 = v4.clone();
                    match m1678.uncons() {
                        None => {
                            Adt137::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt137::C3(f513(v4, v1, v2, List::<Adt109>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f421(v0: Adt110, v1: Adt137, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt107>) -> Adt137 {
    let m1679 = v1.clone();
    match m1679 {
        Adt137::C0(_) => {
            v1
        }
        Adt137::C1(_) => {
            v1
        }
        Adt137::C2(_) => {
            v1
        }
        Adt137::C3(v8) => {
            let m1680 = v8;
            match m1680.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt137::C3(List::cons(Adt109::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt109>::nil()))
                }
            }
        }
    }
}

pub fn f422(p1685: List<Adt51>, p1686: Adt7) -> R<bool> {
    let m1684 = (p1685, p1686);
    {
        let mut m1684 = m1684;
        loop {
            let step = {
                let (v0, v1) = m1684;
                let m1681 = v0;
                match m1681.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f514(f502(match v2.clone() {
                            Adt51::C0(h1682, _, _) => {
                                h1682
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h1683, _, _) => {
                                    h1683
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1684 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f423(p1693: List<u64>, p1694: List<u64>) -> List<u64> {
    let m1692 = (p1693, p1694);
    {
        let mut m1692 = m1692;
        loop {
            let step = {
                let (v0, v1) = m1692;
                let m1687 = v0;
                match m1687.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1688 = v2;
                        let a1689 = 126u64;
                        if nat_eq(a1688, a1689) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1690 = v2;
                            let a1691 = 47u64;
                            if nat_eq(a1690, a1691) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f424() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f425() -> i64 {
    999999999999999999i64
}

pub fn f426(v0: i64) -> R<List<u64>> {
    let m1697 = {
        let a1695 = v0;
        let a1696 = 0i64;
        compare(a1695, a1696)
    };
    if match m1697 {
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
        Ok(List::cons(45u64, f515(20u64, {
            let a1698 = v0;
            int_neg(a1698)?
        }, List::<u64>::nil())?))
    } else {
        f515(20u64, v0, List::<u64>::nil())
    }
}

pub fn f427(v0: u64) -> R<List<u64>> {
    let a1699 = v0;
    let a1700 = 34u64;
    if nat_eq(a1699, a1700) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1701 = v0;
        let a1702 = 92u64;
        if nat_eq(a1701, a1702) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1703 = v0;
            let a1704 = 8u64;
            if nat_eq(a1703, a1704) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1705 = v0;
                let a1706 = 9u64;
                if nat_eq(a1705, a1706) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1707 = v0;
                    let a1708 = 10u64;
                    if nat_eq(a1707, a1708) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1709 = v0;
                        let a1710 = 12u64;
                        if nat_eq(a1709, a1710) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1711 = v0;
                            let a1712 = 13u64;
                            if nat_eq(a1711, a1712) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1713 = v0;
                                let a1714 = 32u64;
                                if nat_lt(a1713, a1714) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f516(f488(v0, 16u64))?, List::cons(f516(f487(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f428(v0: List<Str>) -> Adt6 {
    Adt6::C4(f379(v0))
}

pub fn f429(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1732 = f470(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1715, _, _, _, _, _, _) => {
                h1715
            }
        });
        let a1733 = {
            let a1730 = f470(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1716, _, _, _, _, _) => {
                    h1716
                }
            });
            let a1731 = {
                let a1728 = f470(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1717, _, _, _, _) => {
                        h1717
                    }
                });
                let a1729 = {
                    let a1726 = f517(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1718, _, _, _) => {
                            h1718
                        }
                    });
                    let a1727 = {
                        let a1724 = f518(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1719, _, _) => {
                                h1719
                            }
                        });
                        let a1725 = {
                            let a1722 = f517(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1720, _) => {
                                    h1720
                                }
                            });
                            let a1723 = f470(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1721) => {
                                    h1721
                                }
                            });
                            append_list(a1722, a1723)
                        };
                        append_list(a1724, a1725)
                    };
                    append_list(a1726, a1727)
                };
                append_list(a1728, a1729)
            };
            append_list(a1730, a1731)
        };
        append_list(a1732, a1733)
    })
}

pub fn f430(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1734 = v1;
    match m1734 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f431(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f519(match v0.clone() {
        Adt10::C0(h1735, _) => {
            h1735
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f520(match v0 {
        Adt10::C0(_, h1736) => {
            h1736
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f432(v0: List<Adt36>) -> List<Adt6> {
    f311(f521(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f433(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m1737 = f522(v0, List::<Adt6>::nil())?;
    match m1737 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f311(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f434(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1738 = v1;
    match m1738 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f435(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1739 = v1;
    match m1739 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f436(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f311(f523(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f437(v0: List<Adt47>) -> List<Adt6> {
    f311(f524(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f438(v0: List<Adt55>) -> List<Adt6> {
    f311(f525(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f439(v0: List<Adt57>) -> List<Adt6> {
    f311(f526(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f440(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m1740 = f527(v0, List::<Adt6>::nil())?;
    match m1740 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f311(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f441(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1741 = v1;
    match m1741 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f519(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f442(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m1742 = f528(v0, List::<(Str, Adt6)>::nil());
    match m1742 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f313(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f443(v0: Adt82) -> List<(Str, Adt65)> {
    let m1743 = v0;
    match m1743 {
        Adt82::C0(v1) => {
            v1
        }
        Adt82::C1(_) => {
            List::<(Str, Adt65)>::nil()
        }
        Adt82::C2(v3, _) => {
            v3
        }
    }
}

pub fn f444(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1744 = v1;
    match m1744 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f445(v0: Adt82) -> Option<Str> {
    let m1745 = v0;
    match m1745 {
        Adt82::C0(_) => {
            None::<Str>
        }
        Adt82::C1(v2) => {
            Some(v2)
        }
        Adt82::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f446(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1746 = v0;
    match m1746 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m1747 = f442(v3);
            match m1747 {
                None => {
                    None::<List<(Str, Adt6)>>
                }
                Some(v4) => {
                    Some(List::cons((Str::lit("inputRequests"), v4), List::<(Str, Adt6)>::nil()))
                }
            }
        }
        Adt63::C2(_, v6) => {
            Some(List::cons((Str::lit("result"), Adt6::C5(match v6 {
                Adt5::C0(h1748) => {
                    h1748
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f131(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f447(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h1749, _, _, _, _, _) => {
            h1749
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a1755 = f444(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h1750, _, _, _, _) => {
                h1750
            }
        });
        let a1756 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h1751, _, _, _) => {
                h1751
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h1752, _, _) => {
                h1752
            }
        })), List::cons((Str::lit("ttlMs"), f529(match v0.clone() {
            Adt64::C0(_, _, _, _, h1753, _) => {
                h1753
            }
        })), f530(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h1754) => {
                h1754
            }
        }))));
        append_list(a1755, a1756)
    }))
}

pub fn f448(v0: Adt63) -> Adt64 {
    let m1757 = v0;
    match m1757 {
        Adt63::C0(v1) => {
            v1
        }
        Adt63::C1(v2, _) => {
            v2
        }
        Adt63::C2(v4, _) => {
            v4
        }
        Adt63::C3(v6, _) => {
            v6
        }
        Adt63::C4(v8) => {
            v8
        }
    }
}

pub fn f449(v0: Adt84) -> Str {
    let m1758 = v0;
    match m1758 {
        Adt84::C0 => {
            Str::lit("working")
        }
        Adt84::C1 => {
            Str::lit("input_required")
        }
        Adt84::C2 => {
            Str::lit("completed")
        }
        Adt84::C3 => {
            Str::lit("failed")
        }
        Adt84::C4 => {
            Str::lit("cancelled")
        }
    }
}

pub fn f450(v0: Adt63) -> Adt84 {
    let m1759 = v0;
    match m1759 {
        Adt63::C0(_) => {
            Adt84::C0
        }
        Adt63::C1(_, _) => {
            Adt84::C1
        }
        Adt63::C2(_, _) => {
            Adt84::C2
        }
        Adt63::C3(_, _) => {
            Adt84::C3
        }
        Adt63::C4(_) => {
            Adt84::C4
        }
    }
}

pub fn f451() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f452(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1760, _, _, _, _, _) => {
            h1760
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1761, _, _, _) => {
            h1761
        }
    })), {
        let a1770 = f430(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1762, _, _, _, _) => {
                h1762
            }
        });
        let a1771 = {
            let a1768 = f430(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1763, _, _) => {
                    h1763
                }
            });
            let a1769 = {
                let a1766 = f430(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1764, _) => {
                        h1764
                    }
                });
                let a1767 = f531(match v0 {
                    Adt1::C0(_, _, _, _, _, h1765) => {
                        h1765
                    }
                });
                append_list(a1766, a1767)
            };
            append_list(a1768, a1769)
        };
        append_list(a1770, a1771)
    })))
}

pub fn f453(p1775: List<Adt6>, p1776: List<Adt85>) -> Option<List<Adt85>> {
    let m1774 = (p1775, p1776);
    {
        let mut m1774 = m1774;
        loop {
            let step = {
                let (v0, v1) = m1774;
                let m1772 = v0;
                match m1772.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f455(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1773 = f532(v2);
                        match m1773 {
                            None => {
                                Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(None::<List<Adt85>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f454(v0: List<Adt85>) -> Option<Adt6> {
    let m1777 = f533(v0, List::<Adt6>::nil());
    match m1777 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f311(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f455(p1780: List<Adt85>, p1781: List<Adt85>) -> List<Adt85> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v0, v1) = m1779;
                let m1778 = v0;
                match m1778.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1779 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: Str, p1786: List<Adt85>, v2: i64, p1787: List<Adt85>) -> R<List<Adt85>> {
    let m1785 = (p1786, p1787);
    {
        let mut m1785 = m1785;
        loop {
            let step = {
                let (v1, v3) = m1785;
                let m1782 = v1;
                match m1782.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a1783 = f457(v4.clone());
                        let a1784 = v0.clone();
                        if if equal(a1783, a1784) {
                            true
                        } else {
                            f534(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1785 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Adt85) -> Str {
    let m1790 = f448(match v0 {
        Adt85::C0(h1789, _, _) => {
            h1789
        }
    });
    match m1790 {
        Adt64::C0(h1788, _, _, _, _, _) => {
            h1788
        }
    }
}

pub fn f458(v0: List<List<u64>>) -> bool {
    let m1791 = v0;
    match m1791.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f459() -> List<Adt92> {
    List::cons(Adt92::C0(f535(), f382()), List::cons(Adt92::C0(f536(), f537()), List::cons(Adt92::C0(f538(), f539()), List::<Adt92>::nil())))
}

pub fn f460(v0: List<List<u64>>) -> List<u64> {
    f540(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f461(p1806: u64, p1807: Adt150, p1808: List<Adt151>) -> R<Option<Adt6>> {
    let m1805 = (p1806, (p1807, p1808));
    {
        let mut m1805 = m1805;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1805;
                let m1792 = v0;
                if m1792 == 0 {
                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1792 - 1;
                    let m1793 = v1;
                    match m1793 {
                        Adt150::C0(v4) => {
                            let m1794 = v4.clone();
                            match m1794 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f541(v7)?;
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1795 = v9;
                                    match m1795.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v10), List::cons(Adt151::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1796 = v12;
                                    match m1796.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f541({
                                                let (h1797, _) = v13;
                                                h1797
                                            })?;
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                let (_, h1798) = v13;
                                                h1798
                                            }), List::cons(Adt151::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt150::C1(v16) => {
                            let m1799 = v2;
                            match m1799.uncons() {
                                None => {
                                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1800 = v17;
                                    match m1800 {
                                        Adt151::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1801 = v20;
                                            match m1801.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C4(f311(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v22), List::cons(Adt151::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt151::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1802 = v26;
                                            match m1802.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C5(f313(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f541({
                                                        let (h1803, _) = v28;
                                                        h1803
                                                    })?;
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                        let (_, h1804) = v28;
                                                        h1804
                                                    }), List::cons(Adt151::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1805 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462() -> u64 {
    67108864u64
}

pub fn f463(v0: Adt20) -> Adt6 {
    let m1809 = v0;
    match m1809 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f464(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1810 = v0;
    match m1810 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f465(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1811 = v0;
    match m1811 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f466() -> i64 {
    -32603i64
}

pub fn f467() -> i64 {
    429i64
}

pub fn f468() -> i64 {
    -32021i64
}

pub fn f469(p1814: List<Str>, p1815: List<Adt6>) -> List<Adt6> {
    let m1813 = (p1814, p1815);
    {
        let mut m1813 = m1813;
        loop {
            let step = {
                let (v0, v1) = m1813;
                let m1812 = v0;
                match m1812.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1813 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f470(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1816 = v1;
    match m1816 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1817) => {
                    h1817
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f471(v0: List<u64>) -> R<bool> {
    let m1818 = v0;
    match m1818.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1819 = v2;
            match m1819.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1820 = f190(v1)?;
                    let a1821 = 113u64;
                    if if nat_eq(a1820, a1821) {
                        let a1822 = v3;
                        let a1823 = 61u64;
                        nat_eq(a1822, a1823)
                    } else {
                        false
                    } {
                        Ok(f542(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f472(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt15, Adt132>) -> R<Result<Adt128, Adt131>> {
    let m1824 = v3;
    match m1824 {
        Ok(v5) => {
            f543(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(Adt131::C0(v4)))
        }
    }
}

pub fn f473(v0: Adt140, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt132> {
    let m1825 = v0;
    match m1825 {
        Adt140::C0 => {
            f544(v1, v2)
        }
        Adt140::C1 => {
            f545(v1, v2)
        }
        Adt140::C2 => {
            f546(v1, v2)
        }
        Adt140::C3 => {
            f547(v1, v2)
        }
        Adt140::C4 => {
            f548(v1, v2)
        }
        Adt140::C5 => {
            f549(v1, v2)
        }
        Adt140::C6 => {
            f550(v1, v2)
        }
        Adt140::C7 => {
            f551(v1, v2)
        }
        Adt140::C8 => {
            f552(v1, v2)
        }
        Adt140::C9 => {
            f553(v1, v2)
        }
        Adt140::C10 => {
            f554(v1, v2)
        }
        Adt140::C11 => {
            f555(v1, v2)
        }
    }
}

pub fn f474(v0: Adt140) -> Str {
    let m1826 = v0;
    match m1826 {
        Adt140::C0 => {
            Str::lit("server/discover")
        }
        Adt140::C1 => {
            Str::lit("tools/list")
        }
        Adt140::C2 => {
            Str::lit("tools/call")
        }
        Adt140::C3 => {
            Str::lit("resources/list")
        }
        Adt140::C4 => {
            Str::lit("resources/read")
        }
        Adt140::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt140::C6 => {
            Str::lit("prompts/list")
        }
        Adt140::C7 => {
            Str::lit("prompts/get")
        }
        Adt140::C8 => {
            Str::lit("completion/complete")
        }
        Adt140::C9 => {
            Str::lit("tasks/get")
        }
        Adt140::C10 => {
            Str::lit("tasks/update")
        }
        Adt140::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f475(v0: Option<Adt8>) -> bool {
    let m1827 = v0;
    match m1827 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f476(v0: Option<Adt9>) -> bool {
    let m1828 = v0;
    match m1828 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f477(v0: Option<Adt5>) -> bool {
    let m1829 = v0;
    match m1829 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f478(v0: Option<Adt5>, v1: Str) -> bool {
    let m1830 = v0;
    match m1830 {
        None => {
            false
        }
        Some(v2) => {
            f239(v1, v2)
        }
    }
}

pub fn f479(v0: List<Str>, v1: Adt104, v2: Adt17, v3: Option<Adt113>) -> Adt143 {
    let m1831 = v3;
    match m1831 {
        None => {
            f556(v1, v2.clone(), f557(match v2 {
                Adt17::C0(h1832, _, _, _, _, _) => {
                    h1832
                }
            }, v0))
        }
        Some(v4) => {
            Adt143::C1(Adt131::C2(v4))
        }
    }
}

pub fn f480(v0: Option<Adt112>, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m1833 = v0;
    match m1833 {
        None => {
            Ok(None::<Adt113>)
        }
        Some(v5) => {
            let m1835 = match v5.clone() {
                Adt112::C0(_, _, _, h1834, _) => {
                    h1834
                }
            };
            match m1835 {
                None => {
                    f558(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f481(v0: Option<Adt5>) -> Adt5 {
    let m1836 = v0;
    match m1836 {
        None => {
            f559()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f482(v0: Adt5) -> Result<Adt17, Adt132> {
    let m1837 = f560(v0.clone());
    match m1837 {
        Adt154::C0 => {
            f561(v0.clone(), f182(f562(), v0.clone()), f182(f563(), v0))
        }
        Adt154::C1 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt154::C2 => {
            Err::<Adt17, Adt132>(Adt132::C3)
        }
    }
}

pub fn f483(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1838 = f564(v0)?;
    match m1838 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1839 = f564(v1)?;
            match m1839 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1840 = v2;
                    let a1841 = 61u64;
                    if nat_eq(a1840, a1841) {
                        f565(v5, v6, v4, v3)
                    } else {
                        let m1842 = f564(v2)?;
                        match m1842 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1843 = v3;
                                let a1844 = 61u64;
                                if nat_eq(a1843, a1844) {
                                    f566(v5, v6, v7, v4)
                                } else {
                                    let m1845 = f564(v3)?;
                                    match m1845 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f567(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f484(v0: List<u64>) -> bool {
    let m1846 = v0;
    match m1846.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f485() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f486(v0: Str) -> R<List<u64>> {
    f85({
        let a1847 = v0;
        utf8_encode(a1847)
    })
}

pub fn f487(v0: u64, v1: u64) -> u64 {
    let a1848 = v0;
    let a1849 = v1;
    let a1850 = 0u64;
    nat_rem(a1848, a1849, a1850)
}

pub fn f488(v0: u64, v1: u64) -> u64 {
    let a1851 = v0;
    let a1852 = v1;
    let a1853 = 0u64;
    nat_quot(a1851, a1852, a1853)
}

pub fn f489(v0: u64, v1: Adt6) -> bool {
    let a1854 = v0;
    let a1855 = 0u64;
    if nat_eq(a1854, a1855) {
        f568(v1)
    } else {
        let a1856 = v0;
        let a1857 = 1u64;
        if nat_eq(a1856, a1857) {
            f569(v1)
        } else {
            let a1858 = v0;
            let a1859 = 2u64;
            if nat_eq(a1858, a1859) {
                f570(v1)
            } else {
                let a1860 = v0;
                let a1861 = 3u64;
                if nat_eq(a1860, a1861) {
                    f571(v1)
                } else {
                    let a1862 = v0;
                    let a1863 = 4u64;
                    if nat_eq(a1862, a1863) {
                        f572(v1)
                    } else {
                        let a1864 = v0;
                        let a1865 = 5u64;
                        if nat_eq(a1864, a1865) {
                            f573(v1)
                        } else {
                            let a1866 = v0;
                            let a1867 = 6u64;
                            if nat_eq(a1866, a1867) {
                                f574(v1)
                            } else {
                                let a1868 = v0;
                                let a1869 = 7u64;
                                if nat_eq(a1868, a1869) {
                                    f575(v1)
                                } else {
                                    f576(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f490(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1870 = v1.clone();
    let a1871 = Str::lit("#");
    if equal(a1870, a1871) {
        Some(v0)
    } else {
        let m1872 = f261(v0);
        match m1872 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1873 = f182(Str::lit("$defs"), v2);
                match m1873 {
                    Adt135::C0 => {
                        None::<Adt6>
                    }
                    Adt135::C1(v3) => {
                        let m1874 = f261(v3);
                        match m1874 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f577(v1, match v4 {
                                    Adt5::C0(h1875) => {
                                        h1875
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

pub fn f491(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt125) -> R<Adt147> {
    let m1876 = v3.clone();
    match m1876.uncons() {
        None => {
            Ok(Adt147::C0(Adt97::C8(v0)))
        }
        Some((_, _)) => {
            if f194(v3.clone(), true) {
                let m1877 = f578(v1);
                match m1877 {
                    None => {
                        Ok(Adt147::C0(Adt97::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f579(v3)?;
                        if f580(v9.clone(), match v5.clone() {
                            Adt125::C0(_, h1878) => {
                                h1878
                            }
                        }) {
                            Ok(Adt147::C0(Adt97::C12(v0, v2)))
                        } else {
                            Ok(Adt147::C1(Adt125::C0(List::cons(Adt12::C0(v0, v2, f581(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt125::C0(h1879, _) => {
                                    h1879
                                }
                            }), List::cons(v9, match v5 {
                                Adt125::C0(_, h1880) => {
                                    h1880
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt147::C0(Adt97::C9(v0)))
            }
        }
    }
}

pub fn f492(p1885: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1886: List<Adt105>) -> List<Adt105> {
    let m1884 = (p1885, p1886);
    {
        let mut m1884 = m1884;
        loop {
            let step = {
                let (v0, v3) = m1884;
                let m1881 = v0;
                match m1881.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v5, List::cons(Adt105::C0({
                            let (_, h1882) = v4;
                            h1882
                        }, v1, List::cons({
                            let (h1883, _) = v4;
                            h1883
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1884 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(p1890: List<(Str, Adt6)>, p1891: List<Adt105>) -> List<Adt105> {
    let m1889 = (p1890, p1891);
    {
        let mut m1889 = m1889;
        loop {
            let step = {
                let (v0, v1) = m1889;
                let m1887 = v0;
                match m1887.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0({
                            let (_, h1888) = v2;
                            h1888
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(p1894: List<Adt6>, p1895: List<Adt105>) -> List<Adt105> {
    let m1893 = (p1894, p1895);
    {
        let mut m1893 = m1893;
        loop {
            let step = {
                let (v0, v1) = m1893;
                let m1892 = v0;
                match m1892.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt105>), List<Adt105>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt105>), List<Adt105>>((v3, List::cons(Adt105::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Str) -> bool {
    let a1896 = v0.clone();
    let a1897 = Str::lit("null");
    if equal(a1896, a1897) {
        true
    } else {
        let a1898 = v0.clone();
        let a1899 = Str::lit("boolean");
        if equal(a1898, a1899) {
            true
        } else {
            let a1900 = v0.clone();
            let a1901 = Str::lit("object");
            if equal(a1900, a1901) {
                true
            } else {
                let a1902 = v0.clone();
                let a1903 = Str::lit("array");
                if equal(a1902, a1903) {
                    true
                } else {
                    let a1904 = v0.clone();
                    let a1905 = Str::lit("number");
                    if equal(a1904, a1905) {
                        true
                    } else {
                        let a1906 = v0.clone();
                        let a1907 = Str::lit("string");
                        if equal(a1906, a1907) {
                            true
                        } else {
                            let a1908 = v0;
                            let a1909 = Str::lit("integer");
                            equal(a1908, a1909)
                        }
                    }
                }
            }
        }
    }
}

pub fn f496(v0: Str, v1: Adt6) -> R<bool> {
    let a1910 = v0.clone();
    let a1911 = f582(v1.clone());
    if equal(a1910, a1911) {
        Ok(true)
    } else {
        let a1912 = v0;
        let a1913 = Str::lit("integer");
        if equal(a1912, a1913) {
            let m1914 = f500(v1);
            match m1914 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f583(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f497(p1917: List<Adt6>, v1: Adt6, p1918: bool) -> R<Adt148> {
    let m1916 = (p1917, p1918);
    {
        let mut m1916 = m1916;
        loop {
            let step = {
                let (v0, v2) = m1916;
                let m1915 = v0;
                match m1915.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f495(f584(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt148>>((v4, if v2 {
                                true
                            } else {
                                f496(f584(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt148>>(Ok(Adt148::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(p1922: List<Adt6>, v1: Adt6) -> R<Adt148> {
    let m1921 = p1922;
    {
        let mut m1921 = m1921;
        loop {
            let step = {
                let v0 = m1921;
                let m1919 = v0;
                match m1919.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C1))
                    }
                    Some((v2, v3)) => {
                        let m1920 = f499(v2, v1.clone())?;
                        match m1920 {
                            Adt152::C0 => {
                                Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C0))
                            }
                            Adt152::C1 => {
                                Ok::<List<Adt6>, R<Adt148>>(v3)
                            }
                            Adt152::C2 => {
                                Err::<List<Adt6>, R<Adt148>>(Ok(Adt148::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: Adt6, v1: Adt6) -> R<Adt152> {
    f585(f586(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f500(v0: Adt6) -> Option<Adt7> {
    let m1923 = v0;
    match m1923 {
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

pub fn f501(v0: u64, v1: Adt153) -> bool {
    let a1924 = v0;
    let a1925 = 0u64;
    if nat_eq(a1924, a1925) {
        let a1926 = f587(v1);
        bool_not(a1926)
    } else {
        let a1927 = v0;
        let a1928 = 1u64;
        if nat_eq(a1927, a1928) {
            let a1929 = f514(v1);
            bool_not(a1929)
        } else {
            let a1930 = v0;
            let a1931 = 2u64;
            if nat_eq(a1930, a1931) {
                f514(v1)
            } else {
                f587(v1)
            }
        }
    }
}

pub fn f502(v0: Adt7, v1: Adt7) -> R<Adt153> {
    f588(v0.clone(), v1.clone(), f589(match v0 {
        Adt7::C0(h1932, _) => {
            h1932
        }
    }), f589(match v1 {
        Adt7::C0(h1933, _) => {
            h1933
        }
    }))
}

pub fn f503(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt148> {
    let m1934 = f238(v0, v2);
    match m1934 {
        Adt135::C0 => {
            Ok(Adt148::C0)
        }
        Adt135::C1(v4) => {
            let m1935 = f590(v4);
            match m1935 {
                None => {
                    Ok(Adt148::C2)
                }
                Some(v5) => {
                    let m1938 = {
                        let a1936 = v5;
                        let a1937 = 0i64;
                        compare(a1936, a1937)
                    };
                    if match m1938 {
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
                        Ok(Adt148::C2)
                    } else {
                        if v1 {
                            if f587(f591(f79(v3, 0i64)?, v5)) {
                                Ok(Adt148::C1)
                            } else {
                                Ok(Adt148::C0)
                            }
                        } else {
                            if f514(f591(f79(v3, 0i64)?, v5)) {
                                Ok(Adt148::C1)
                            } else {
                                Ok(Adt148::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f504(v0: Str) -> R<u64> {
    f592(f85({
        let a1939 = v0;
        utf8_encode(a1939)
    })?, 0u64)
}

pub fn f505(p1944: List<Adt6>, p1945: u64) -> R<u64> {
    let m1943 = (p1944, p1945);
    {
        let mut m1943 = m1943;
        loop {
            let step = {
                let (v0, v1) = m1943;
                let m1940 = v0;
                match m1940.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1941 = v1;
                            let a1942 = 1u64;
                            nat_add(a1941, a1942)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1943 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(p1950: List<(Str, Adt6)>, p1951: u64) -> R<u64> {
    let m1949 = (p1950, p1951);
    {
        let mut m1949 = m1949;
        loop {
            let step = {
                let (v0, v1) = m1949;
                let m1946 = v0;
                match m1946.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1947 = v1;
                            let a1948 = 1u64;
                            nat_add(a1947, a1948)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1949 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(p1955: List<Adt6>, v1: Adt5) -> Adt148 {
    let m1954 = p1955;
    {
        let mut m1954 = m1954;
        loop {
            let step = {
                let v0 = m1954;
                let m1952 = v0;
                match m1952.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt148>(Adt148::C0)
                    }
                    Some((v2, v3)) => {
                        let m1953 = f235(v2);
                        match m1953 {
                            None => {
                                Err::<List<Adt6>, Adt148>(Adt148::C2)
                            }
                            Some(v4) => {
                                if f239(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt148>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt148>(Adt148::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1954 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(p1962: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt107>, p1963: List<Adt109>) -> List<Adt109> {
    let m1961 = (p1962, p1963);
    {
        let mut m1961 = m1961;
        loop {
            let step = {
                let (v0, v3) = m1961;
                let m1956 = v0;
                match m1956.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1958 = f238({
                            let (h1957, _) = v4.clone();
                            h1957
                        }, v1.clone());
                        match m1958 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, v3))
                            }
                            Adt135::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0({
                                    let (_, h1959) = v4;
                                    h1959
                                }, v6, List::cons(Adt107::C0({
                                    let (h1960, _) = v4;
                                    h1960
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(p1968: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1967 = p1968;
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let v0 = m1967;
                let m1964 = v0;
                match m1964.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1966 = f238({
                            let (h1965, _) = v2;
                            h1965
                        }, v1.clone());
                        match m1966 {
                            Adt135::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt135::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1969 = f238(Str::lit("properties"), v0);
    match m1969 {
        Adt135::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt135::C1(v1) => {
            let m1970 = f261(v1);
            match m1970 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1971) => {
                            h1971
                        }
                    }
                }
            }
        }
    }
}

pub fn f511(p1978: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt107>, p1979: List<Adt109>) -> List<Adt109> {
    let m1977 = (p1978, p1979);
    {
        let mut m1977 = m1977;
        loop {
            let step = {
                let (v0, v4) = m1977;
                let m1972 = v0;
                match m1972.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1974 = f238({
                            let (h1973, _) = v5.clone();
                            h1973
                        }, v1.clone());
                        match m1974 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, List::cons(Adt109::C0(v2.clone(), {
                                    let (_, h1975) = v5;
                                    h1975
                                }, List::cons(Adt107::C0({
                                    let (h1976, _) = v5;
                                    h1976
                                }), v3.clone())), v4)))
                            }
                            Adt135::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt109>), List<Adt109>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(p1984: List<Adt6>, p1985: u64, v2: Adt6, v3: List<Adt107>, p1986: List<Adt109>) -> R<List<Adt109>> {
    let m1983 = (p1984, (p1985, p1986));
    {
        let mut m1983 = m1983;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1983;
                let m1980 = v0;
                match m1980.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt109>)), R<List<Adt109>>>((v6, ({
                            let a1981 = v1;
                            let a1982 = 1u64;
                            nat_add(a1981, a1982)?
                        }, List::cons(Adt109::C0(v2.clone(), v5, List::cons(Adt107::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(p1989: List<Adt6>, v1: Adt6, v2: List<Adt107>, p1990: List<Adt109>) -> List<Adt109> {
    let m1988 = (p1989, p1990);
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let (v0, v3) = m1988;
                let m1987 = v0;
                match m1987.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt109>), List<Adt109>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt109>), List<Adt109>>((v5, List::cons(Adt109::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f514(v0: Adt153) -> bool {
    let m1991 = v0;
    match m1991 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            true
        }
    }
}

pub fn f515(p2003: u64, p2004: i64, p2005: List<u64>) -> R<List<u64>> {
    let m2002 = (p2003, (p2004, p2005));
    {
        let mut m2002 = m2002;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2002;
                let m1992 = v0;
                if m1992 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1992 - 1;
                    let v4: i64 = {
                        let a1993 = v1;
                        let a1994 = 10i64;
                        let a1995 = 0i64;
                        int_quot(a1993, a1994, a1995)?
                    };
                    let v5: i64 = {
                        let a1996 = v1;
                        let a1997 = 10i64;
                        let a1998 = 0i64;
                        int_rem(a1996, a1997, a1998)
                    };
                    let m2001 = {
                        let a1999 = v4;
                        let a2000 = 1i64;
                        compare(a1999, a2000)
                    };
                    if match m2001 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f593(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f593(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m2002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: u64) -> R<u64> {
    let a2006 = v0;
    let a2007 = 10u64;
    if nat_lt(a2006, a2007) {
        let a2008 = 48u64;
        let a2009 = v0;
        nat_add(a2008, a2009)
    } else {
        let a2010 = 87u64;
        let a2011 = v0;
        nat_add(a2010, a2011)
    }
}

pub fn f517(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m2012 = v1;
    match m2012 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f594(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f518(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m2013 = v0;
    match m2013 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a2016 = f435(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h2014, _) => {
                        h2014
                    }
                });
                let a2017 = f435(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h2015) => {
                        h2015
                    }
                });
                append_list(a2016, a2017)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f519(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f595(v0, 0i64)?, 0i64)))
}

pub fn f520(v0: Adt11) -> Str {
    let m2018 = v0;
    match m2018 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f521(p2021: List<Adt36>, p2022: List<Adt6>) -> List<Adt6> {
    let m2020 = (p2021, p2022);
    {
        let mut m2020 = m2020;
        loop {
            let step = {
                let (v0, v1) = m2020;
                let m2019 = v0;
                match m2019.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f596(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f522(p2026: List<Adt39>, p2027: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2025 = (p2026, p2027);
    {
        let mut m2025 = m2025;
        loop {
            let step = {
                let (v0, v1) = m2025;
                let m2023 = v0;
                match m2023.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2024 = f597(v2)?;
                        match m2024 {
                            None => {
                                Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(p2030: List<Adt45>, p2031: List<Adt6>) -> R<List<Adt6>> {
    let m2029 = (p2030, p2031);
    {
        let mut m2029 = m2029;
        loop {
            let step = {
                let (v0, v1) = m2029;
                let m2028 = v0;
                match m2028.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f598(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(p2034: List<Adt47>, p2035: List<Adt6>) -> List<Adt6> {
    let m2033 = (p2034, p2035);
    {
        let mut m2033 = m2033;
        loop {
            let step = {
                let (v0, v1) = m2033;
                let m2032 = v0;
                match m2032.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f599(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2033 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(p2038: List<Adt55>, p2039: List<Adt6>) -> List<Adt6> {
    let m2037 = (p2038, p2039);
    {
        let mut m2037 = m2037;
        loop {
            let step = {
                let (v0, v1) = m2037;
                let m2036 = v0;
                match m2036.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f600(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(p2042: List<Adt57>, p2043: List<Adt6>) -> List<Adt6> {
    let m2041 = (p2042, p2043);
    {
        let mut m2041 = m2041;
        loop {
            let step = {
                let (v0, v1) = m2041;
                let m2040 = v0;
                match m2040.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f601(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(p2047: List<Adt60>, p2048: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2046 = (p2047, p2048);
    {
        let mut m2046 = m2046;
        loop {
            let step = {
                let (v0, v1) = m2046;
                let m2044 = v0;
                match m2044.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2045 = f602(v2)?;
                        match m2045 {
                            None => {
                                Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(p2054: List<(Str, Adt65)>, p2055: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2053 = (p2054, p2055);
    {
        let mut m2053 = m2053;
        loop {
            let step = {
                let (v0, v1) = m2053;
                let m2049 = v0;
                match m2049.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2051 = f603({
                            let (_, h2050) = v2;
                            h2050
                        });
                        match m2051 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2052, _) = v2;
                                    h2052
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2053 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(v0: Option<i64>) -> Adt6 {
    let m2056 = v0;
    match m2056 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f604(v1)
        }
    }
}

pub fn f530(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m2057 = v1;
    match m2057 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f604(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f531(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m2058 = v0;
    match m2058 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f605(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f532(v0: Adt6) -> Option<Adt85> {
    let m2059 = f261(v0);
    match m2059 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m2060 = f606(Str::lit("task"), v1.clone());
            match m2060 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m2061 = f607(v2);
                    match m2061 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m2062 = f608(Str::lit("createdMs"), v1.clone());
                            match m2062 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m2063 = f606(Str::lit("plan"), v1);
                                    match m2063 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m2064 = f609(v5);
                                            match m2064 {
                                                None => {
                                                    None::<Adt85>
                                                }
                                                Some(v6) => {
                                                    Some(Adt85::C0(v3, v4, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f533(p2068: List<Adt85>, p2069: List<Adt6>) -> Option<List<Adt6>> {
    let m2067 = (p2068, p2069);
    {
        let mut m2067 = m2067;
        loop {
            let step = {
                let (v0, v1) = m2067;
                let m2065 = v0;
                match m2065.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2066 = f610(v2);
                        match m2066 {
                            None => {
                                Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(None::<List<Adt6>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2067 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f534(v0: Adt85, v1: i64) -> R<bool> {
    let m2073 = {
        let m2072 = f448(match v0.clone() {
            Adt85::C0(h2071, _, _) => {
                h2071
            }
        });
        match m2072 {
            Adt64::C0(_, _, _, _, h2070, _) => {
                h2070
            }
        }
    };
    match m2073 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2080 = {
                let m2079 = {
                    let a2077 = {
                        let a2075 = v1;
                        let a2076 = match v0 {
                            Adt85::C0(_, h2074, _) => {
                                h2074
                            }
                        };
                        int_sub(a2075, a2076)?
                    };
                    let a2078 = v2;
                    compare(a2077, a2078)
                };
                match m2079 {
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
            Ok(bool_not(a2080))
        }
    }
}

pub fn f535() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f536() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f537() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f538() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f539() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f540(p2085: List<List<u64>>, p2086: List<u64>) -> List<u64> {
    let m2084 = (p2085, p2086);
    {
        let mut m2084 = m2084;
        loop {
            let step = {
                let (v0, v1) = m2084;
                let m2081 = v0;
                match m2081.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2082 = f611(v2);
                            let a2083 = v1;
                            append_list(a2082, a2083)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2084 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: Str) -> R<Str> {
    let m2089 = {
        let a2088 = f207(f87(), f169(f612(f85({
            let a2087 = v0;
            utf8_encode(a2087)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2088)
    };
    match m2089 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f542(v0: List<u64>) -> bool {
    let m2090 = v0;
    match m2090.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2091 = v1;
            let a2092 = 48u64;
            if nat_eq(a2091, a2092) {
                f613(v2)
            } else {
                false
            }
        }
    }
}

pub fn f543(v0: Adt0, v1: Fn2, v2: u64, v3: Adt15) -> R<Result<Adt128, Adt131>> {
    if f614(f615(v3.clone())) {
        Ok(Ok::<Adt128, Adt131>(f616(v0.clone(), Adt33::C0(f617(v0)))))
    } else {
        Ok(f618(v0.clone(), v3.clone(), f619(match v0 {
            Adt0::C0(_, _, _, _, _, h2093) => {
                h2093
            }
        }, v2, v1, v3)?))
    }
}

pub fn f544(v0: Adt17, _: Adt5) -> Result<Adt15, Adt132> {
    Ok::<Adt15, Adt132>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f545(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2094 = f620(Str::lit("cursor"), v1);
    match m2094 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f546(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2095 = f621(Str::lit("name"), v1.clone());
    match m2095 {
        Ok(v2) => {
            let m2096 = f622(Str::lit("arguments"), v1.clone());
            match m2096 {
                Ok(v3) => {
                    let m2097 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2097 {
                        Ok(v4) => {
                            let m2098 = f620(Str::lit("requestState"), v1);
                            match m2098 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt132>(Adt15::C2(Adt22::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt132>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt132>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt132>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt132>(v9)
        }
    }
}

pub fn f547(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2099 = f620(Str::lit("cursor"), v1);
    match m2099 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f548(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2100 = f621(Str::lit("uri"), v1.clone());
    match m2100 {
        Ok(v2) => {
            let m2101 = f622(Str::lit("inputResponses"), v1.clone());
            match m2101 {
                Ok(v3) => {
                    let m2102 = f620(Str::lit("requestState"), v1);
                    match m2102 {
                        Ok(v4) => {
                            Ok::<Adt15, Adt132>(Adt15::C4(Adt23::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt15, Adt132>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt15, Adt132>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt15, Adt132>(v7)
        }
    }
}

pub fn f549(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2103 = f620(Str::lit("cursor"), v1);
    match m2103 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f550(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2104 = f620(Str::lit("cursor"), v1);
    match m2104 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f551(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2105 = f621(Str::lit("name"), v1.clone());
    match m2105 {
        Ok(v2) => {
            let m2106 = f623(Str::lit("arguments"), v1.clone());
            match m2106 {
                Ok(v3) => {
                    let m2107 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2107 {
                        Ok(v4) => {
                            let m2108 = f620(Str::lit("requestState"), v1);
                            match m2108 {
                                Ok(v5) => {
                                    Ok::<Adt15, Adt132>(Adt15::C7(Adt24::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt15, Adt132>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt15, Adt132>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt15, Adt132>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt15, Adt132>(v9)
        }
    }
}

pub fn f552(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2109 = f624(Str::lit("ref"), v1.clone());
    match m2109 {
        Ok(v2) => {
            let m2110 = f625(v2);
            match m2110 {
                Ok(v3) => {
                    let m2111 = f624(Str::lit("argument"), v1.clone());
                    match m2111 {
                        Ok(v4) => {
                            let m2112 = f626(v4);
                            match m2112 {
                                Ok(v5) => {
                                    let m2113 = f627(v1);
                                    match m2113 {
                                        Ok(v6) => {
                                            Ok::<Adt15, Adt132>(Adt15::C8(Adt25::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt15, Adt132>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt15, Adt132>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt15, Adt132>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt15, Adt132>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt15, Adt132>(v11)
        }
    }
}

pub fn f553(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2114 = f621(Str::lit("taskId"), v1);
    match m2114 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f554(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2115 = f621(Str::lit("taskId"), v1.clone());
    match m2115 {
        Ok(v2) => {
            let m2116 = f624(Str::lit("inputResponses"), v1);
            match m2116 {
                Ok(v3) => {
                    Ok::<Adt15, Adt132>(Adt15::C10(Adt31::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt15, Adt132>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt15, Adt132>(v5)
        }
    }
}

pub fn f555(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2117 = f621(Str::lit("taskId"), v1);
    match m2117 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f556(v0: Adt104, v1: Adt17, v2: Option<Adt131>) -> Adt143 {
    let m2118 = v2;
    match m2118 {
        None => {
            Adt143::C0(Adt144::C0(match v0.clone() {
                Adt104::C0(h2119, _, _) => {
                    h2119
                }
            }, match v0.clone() {
                Adt104::C0(_, h2120, _) => {
                    h2120
                }
            }, v1, f481(match v0 {
                Adt104::C0(_, _, h2121) => {
                    h2121
                }
            })))
        }
        Some(v3) => {
            Adt143::C1(v3)
        }
    }
}

pub fn f557(v0: Str, v1: List<Str>) -> Option<Adt131> {
    f628(f629(v0.clone(), v1.clone()), v0, v1)
}

pub fn f558(v0: Adt112, v1: Adt145, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt113>> {
    let m2123 = match v0.clone() {
        Adt112::C0(h2122, _, _, _, _) => {
            h2122
        }
    };
    match m2123 {
        None => {
            Ok(Some(Adt113::C0))
        }
        Some(v5) => {
            let a2124 = v5;
            let a2125 = v3;
            if equal(a2124, a2125) {
                let m2127 = match v0.clone() {
                    Adt112::C0(_, h2126, _, _, _) => {
                        h2126
                    }
                };
                match m2127 {
                    None => {
                        Ok(Some(Adt113::C1))
                    }
                    Some(v6) => {
                        let a2128 = v6;
                        let a2129 = v2;
                        if equal(a2128, a2129) {
                            f630(f631(match v0.clone() {
                                Adt112::C0(_, _, h2130, _, _) => {
                                    h2130
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt112::C0(_, _, _, _, h2131) => {
                                    h2131
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt113::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt113::C3))
            }
        }
    }
}

pub fn f559() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f560(v0: Adt5) -> Adt154 {
    f632(f182(f562(), v0.clone()), f182(f563(), v0))
}

pub fn f561(v0: Adt5, v1: Adt135, v2: Adt135) -> Result<Adt17, Adt132> {
    let m2132 = v1;
    match m2132 {
        Adt135::C0 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt135::C1(v3) => {
            let m2133 = v2;
            match m2133 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C3)
                }
                Adt135::C1(v4) => {
                    f633(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f562() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f563() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f564(v0: u64) -> R<Option<u64>> {
    let a2134 = 64u64;
    let a2135 = v0;
    if if nat_lt(a2134, a2135) {
        let a2136 = v0;
        let a2137 = 91u64;
        nat_lt(a2136, a2137)
    } else {
        false
    } {
        Ok(Some({
            let a2138 = v0;
            let a2139 = 65u64;
            nat_sub(a2138, a2139)
        }))
    } else {
        let a2140 = 96u64;
        let a2141 = v0;
        if if nat_lt(a2140, a2141) {
            let a2142 = v0;
            let a2143 = 123u64;
            nat_lt(a2142, a2143)
        } else {
            false
        } {
            Ok(Some({
                let a2144 = v0;
                let a2145 = 71u64;
                nat_sub(a2144, a2145)
            }))
        } else {
            let a2146 = 47u64;
            let a2147 = v0;
            if if nat_lt(a2146, a2147) {
                let a2148 = v0;
                let a2149 = 58u64;
                nat_lt(a2148, a2149)
            } else {
                false
            } {
                Ok(Some({
                    let a2150 = v0;
                    let a2151 = 4u64;
                    nat_add(a2150, a2151)?
                }))
            } else {
                let a2152 = v0;
                let a2153 = 43u64;
                if nat_eq(a2152, a2153) {
                    Ok(Some(62u64))
                } else {
                    let a2154 = v0;
                    let a2155 = 47u64;
                    if nat_eq(a2154, a2155) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f565(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2156 = v3;
        let a2157 = 61u64;
        if nat_eq(a2156, a2157) {
            let a2158 = f634(v1, 16u64);
            let a2159 = 0u64;
            nat_eq(a2158, a2159)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f635(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f566(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2160 = f634(v2, 4u64);
        let a2161 = 0u64;
        nat_eq(a2160, a2161)
    } else {
        false
    } {
        Ok(Some(f636(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f567(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2164 = {
            let a2162 = v0;
            let a2163 = 4u64;
            nat_mul(a2162, a2163)?
        };
        let a2165 = f637(v1, 16u64);
        nat_add(a2164, a2165)?
    }, List::cons({
        let a2168 = {
            let a2166 = f634(v1, 16u64);
            let a2167 = 16u64;
            nat_mul(a2166, a2167)?
        };
        let a2169 = f637(v2, 4u64);
        nat_add(a2168, a2169)?
    }, List::cons({
        let a2172 = {
            let a2170 = f634(v2, 4u64);
            let a2171 = 64u64;
            nat_mul(a2170, a2171)?
        };
        let a2173 = v3;
        nat_add(a2172, a2173)?
    }, List::<u64>::nil()))))
}

pub fn f568(v0: Adt6) -> bool {
    let m2174 = f235(v0);
    match m2174 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f569(v0: Adt6) -> bool {
    let m2175 = f500(v0);
    match m2175 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f570(v0: Adt6) -> bool {
    let m2176 = f590(v0);
    match m2176 {
        None => {
            false
        }
        Some(v1) => {
            let a2180 = {
                let m2179 = {
                    let a2177 = v1;
                    let a2178 = 0i64;
                    compare(a2177, a2178)
                };
                match m2179 {
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
            bool_not(a2180)
        }
    }
}

pub fn f571(v0: Adt6) -> bool {
    let m2181 = f334(v0);
    match m2181 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f572(v0: Adt6) -> bool {
    let m2182 = f261(v0);
    match m2182 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f573(v0: Adt6) -> bool {
    let m2183 = f334(v0);
    match m2183 {
        None => {
            false
        }
        Some(v1) => {
            f638(v1)
        }
    }
}

pub fn f574(v0: Adt6) -> bool {
    let m2184 = f260(v0);
    match m2184 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f575(v0: Adt6) -> bool {
    let m2185 = f235(v0.clone());
    match m2185 {
        None => {
            let m2186 = f334(v0);
            match m2186 {
                None => {
                    false
                }
                Some(v2) => {
                    f639(v2)
                }
            }
        }
        Some(v1) => {
            f495(v1)
        }
    }
}

pub fn f576(v0: Adt6) -> bool {
    let m2187 = f334(v0);
    match m2187 {
        None => {
            false
        }
        Some(v1) => {
            let m2188 = v1;
            match m2188.uncons() {
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

pub fn f577(v0: Str, p2206: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2205 = p2206;
    {
        let mut m2205 = m2205;
        loop {
            let step = {
                let v1 = m2205;
                let m2189 = v1;
                match m2189.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2193 = v0.clone();
                        let a2194 = {
                            let a2191 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2190, _) = v2.clone();
                                h2190
                            }, List::<Str>::nil()));
                            let a2192 = Str::lit("");
                            join(a2191, a2192)
                        };
                        if equal(a2193, a2194) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2195) = v2;
                                h2195
                            }))
                        } else {
                            let m2197 = f261({
                                let (_, h2196) = v2.clone();
                                h2196
                            });
                            match m2197 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2198 = f182(Str::lit("$anchor"), v4);
                                    match m2198 {
                                        Adt135::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt135::C1(v5) => {
                                            let m2199 = f235(v5);
                                            match m2199 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2202 = v0.clone();
                                                    let a2203 = {
                                                        let a2200 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2201 = Str::lit("");
                                                        join(a2200, a2201)
                                                    };
                                                    if equal(a2202, a2203) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2204) = v2;
                                                            h2204
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
                Ok(next) => { m2205 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2207 = f238(Str::lit("type"), v0);
    match m2207 {
        Adt135::C0 => {
            None::<Adt13>
        }
        Adt135::C1(v1) => {
            let m2208 = f235(v1);
            match m2208 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f640(v2)
                }
            }
        }
    }
}

pub fn f579(v0: List<u64>) -> R<List<u64>> {
    Ok(f169(f641(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f580(v0: List<u64>, p2211: List<List<u64>>) -> bool {
    let m2210 = p2211;
    {
        let mut m2210 = m2210;
        loop {
            let step = {
                let v1 = m2210;
                let m2209 = v1;
                match m2209.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f642(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2210 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p2214: List<Str>, p2215: List<Str>) -> List<Str> {
    let m2213 = (p2214, p2215);
    {
        let mut m2213 = m2213;
        loop {
            let step = {
                let (v0, v1) = m2213;
                let m2212 = v0;
                match m2212.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: Adt6) -> Str {
    let m2216 = v0;
    match m2216 {
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

pub fn f583(v0: Adt7) -> R<bool> {
    if f395(match v0.clone() {
        Adt7::C0(h2217, _) => {
            h2217
        }
    }) {
        Ok(true)
    } else {
        f643(40u64, match v0.clone() {
            Adt7::C0(h2218, _) => {
                h2218
            }
        }, match v0 {
            Adt7::C0(_, h2219) => {
                h2219
            }
        })
    }
}

pub fn f584(v0: Adt6) -> Str {
    let m2220 = f235(v0);
    match m2220 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f585(p2227: u64, p2228: List<(Adt6, Adt6)>) -> R<Adt152> {
    let m2226 = (p2227, p2228);
    {
        let mut m2226 = m2226;
        loop {
            let step = {
                let (v0, v1) = m2226;
                let m2221 = v0;
                if m2221 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C2))
                } else {
                    let v2 = m2221 - 1;
                    let m2222 = v1;
                    match m2222.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C0))
                        }
                        Some((v3, v4)) => {
                            let m2225 = f644({
                                let (h2223, _) = v3;
                                h2223
                            }, {
                                let (_, h2224) = v3;
                                h2224
                            }, v4)?;
                            match m2225 {
                                Adt155::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt152>>(Ok(Adt152::C1))
                                }
                                Adt155::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt152>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2226 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586() -> u64 {
    67108864u64
}

pub fn f587(v0: Adt153) -> bool {
    let m2229 = v0;
    match m2229 {
        Adt153::C0 => {
            true
        }
        Adt153::C1 => {
            false
        }
        Adt153::C2 => {
            false
        }
    }
}

pub fn f588(v0: Adt7, v1: Adt7, v2: Adt153, v3: Adt153) -> R<Adt153> {
    let m2230 = v2;
    match m2230 {
        Adt153::C0 => {
            let m2232 = v3;
            match m2232 {
                Adt153::C0 => {
                    Ok(f645(Adt153::C0, f646(f647(match v0.clone() {
                        Adt7::C0(h2233, _) => {
                            h2233
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2234) => {
                            h2234
                        }
                    }, f647(match v1.clone() {
                        Adt7::C0(h2235, _) => {
                            h2235
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2236) => {
                            h2236
                        }
                    })?))
                }
                Adt153::C1 => {
                    Ok(Adt153::C0)
                }
                Adt153::C2 => {
                    Ok(Adt153::C0)
                }
            }
        }
        Adt153::C1 => {
            let m2231 = v3;
            match m2231 {
                Adt153::C0 => {
                    Ok(Adt153::C2)
                }
                Adt153::C1 => {
                    Ok(Adt153::C1)
                }
                Adt153::C2 => {
                    Ok(Adt153::C0)
                }
            }
        }
        Adt153::C2 => {
            let m2237 = v3;
            match m2237 {
                Adt153::C0 => {
                    Ok(Adt153::C2)
                }
                Adt153::C1 => {
                    Ok(Adt153::C2)
                }
                Adt153::C2 => {
                    f646(match v0.clone() {
                        Adt7::C0(h2238, _) => {
                            h2238
                        }
                    }, match v0 {
                        Adt7::C0(_, h2239) => {
                            h2239
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2240, _) => {
                            h2240
                        }
                    }, match v1 {
                        Adt7::C0(_, h2241) => {
                            h2241
                        }
                    })
                }
            }
        }
    }
}

pub fn f589(v0: i64) -> Adt153 {
    f591(v0, 0i64)
}

pub fn f590(v0: Adt6) -> Option<i64> {
    let m2242 = v0;
    match m2242 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f395(match v2.clone() {
                Adt7::C0(_, h2243) => {
                    h2243
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2244, _) => {
                        h2244
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

pub fn f591(v0: i64, v1: i64) -> Adt153 {
    let m2247 = {
        let a2245 = v0;
        let a2246 = v1;
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
        Adt153::C0
    } else {
        let m2250 = {
            let a2248 = v1;
            let a2249 = v0;
            compare(a2248, a2249)
        };
        if match m2250 {
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
            Adt153::C2
        } else {
            Adt153::C1
        }
    }
}

pub fn f592(p2259: List<u64>, p2260: u64) -> R<u64> {
    let m2258 = (p2259, p2260);
    {
        let mut m2258 = m2258;
        loop {
            let step = {
                let (v0, v1) = m2258;
                let m2251 = v0;
                match m2251.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2252 = v2;
                        let a2253 = 128u64;
                        if if nat_lt(a2252, a2253) {
                            true
                        } else {
                            let a2254 = 191u64;
                            let a2255 = v2;
                            nat_lt(a2254, a2255)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2256 = v1;
                                let a2257 = 1u64;
                                nat_add(a2256, a2257)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: i64) -> u64 {
    let m2263 = {
        let a2261 = v0;
        let a2262 = 1i64;
        compare(a2261, a2262)
    };
    if match m2263 {
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
        let m2266 = {
            let a2264 = v0;
            let a2265 = 2i64;
            compare(a2264, a2265)
        };
        if match m2266 {
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
            let m2269 = {
                let a2267 = v0;
                let a2268 = 3i64;
                compare(a2267, a2268)
            };
            if match m2269 {
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
                let m2272 = {
                    let a2270 = v0;
                    let a2271 = 4i64;
                    compare(a2270, a2271)
                };
                if match m2272 {
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
                    let m2275 = {
                        let a2273 = v0;
                        let a2274 = 5i64;
                        compare(a2273, a2274)
                    };
                    if match m2275 {
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
                        let m2278 = {
                            let a2276 = v0;
                            let a2277 = 6i64;
                            compare(a2276, a2277)
                        };
                        if match m2278 {
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
                            let m2281 = {
                                let a2279 = v0;
                                let a2280 = 7i64;
                                compare(a2279, a2280)
                            };
                            if match m2281 {
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
                                let m2284 = {
                                    let a2282 = v0;
                                    let a2283 = 8i64;
                                    compare(a2282, a2283)
                                };
                                if match m2284 {
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
                                    let m2287 = {
                                        let a2285 = v0;
                                        let a2286 = 9i64;
                                        compare(a2285, a2286)
                                    };
                                    if match m2287 {
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

pub fn f594(v0: Adt8) -> Adt6 {
    Adt6::C5(f435(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2288) => {
            h2288
        }
    }))
}

pub fn f595(p2293: u64, p2294: i64) -> R<i64> {
    let m2292 = (p2293, p2294);
    {
        let mut m2292 = m2292;
        loop {
            let step = {
                let (v0, v1) = m2292;
                let m2289 = v0;
                if m2289 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2289 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2290 = v1;
                        let a2291 = -1i64;
                        int_sub(a2290, a2291)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2292 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2295, _, _, _, _, _, _, _) => {
            h2295
        }
    })), {
        let a2313 = f430(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2296, _, _, _, _, _, _) => {
                h2296
            }
        });
        let a2314 = {
            let a2311 = f531(match v0.clone() {
                Adt36::C0(_, _, h2297, _, _, _, _, _) => {
                    h2297
                }
            });
            let a2312 = {
                let a2309 = f430(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2298, _, _, _, _) => {
                        h2298
                    }
                });
                let a2310 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2301 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2300, _, _, _) => {
                            h2300
                        }
                    };
                    match m2301 {
                        Adt5::C0(h2299) => {
                            h2299
                        }
                    }
                })), {
                    let a2307 = f470(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2302, _, _) => {
                            h2302
                        }
                    });
                    let a2308 = {
                        let a2305 = f648(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2303, _) => {
                                h2303
                            }
                        });
                        let a2306 = f649(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2304) => {
                                h2304
                            }
                        });
                        append_list(a2305, a2306)
                    };
                    append_list(a2307, a2308)
                });
                append_list(a2309, a2310)
            };
            append_list(a2311, a2312)
        };
        append_list(a2313, a2314)
    }))
}

pub fn f597(v0: Adt39) -> R<Option<Adt6>> {
    let m2315 = v0;
    match m2315 {
        Adt39::C0(v1) => {
            Ok(Some(f650(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f651(Str::lit("image"), match v2.clone() {
                Adt43::C0(h2316, _, _, _) => {
                    h2316
                }
            }, match v2.clone() {
                Adt43::C0(_, h2317, _, _) => {
                    h2317
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h2318, _) => {
                    h2318
                }
            }, match v2 {
                Adt43::C0(_, _, _, h2319) => {
                    h2319
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f651(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h2320, _, _, _) => {
                    h2320
                }
            }, match v3.clone() {
                Adt44::C0(_, h2321, _, _) => {
                    h2321
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h2322, _) => {
                    h2322
                }
            }, match v3 {
                Adt44::C0(_, _, _, h2323) => {
                    h2323
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f652(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f653(v5)))
        }
    }
}

pub fn f598(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2347 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2324, _, _, _, _, _, _, _, _) => {
                h2324
            }
        })), List::<(Str, Adt6)>::nil());
        let a2348 = {
            let a2345 = f430(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2325, _, _, _, _, _, _, _) => {
                    h2325
                }
            });
            let a2346 = {
                let a2343 = f531(match v0.clone() {
                    Adt45::C0(_, _, h2326, _, _, _, _, _, _) => {
                        h2326
                    }
                });
                let a2344 = {
                    let a2341 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2327, _, _, _, _, _) => {
                            h2327
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2342 = {
                        let a2339 = f430(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2328, _, _, _, _) => {
                                h2328
                            }
                        });
                        let a2340 = {
                            let a2337 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2329, _, _, _) => {
                                    h2329
                                }
                            });
                            let a2338 = {
                                let a2335 = f654(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2330, _, _) => {
                                        h2330
                                    }
                                });
                                let a2336 = {
                                    let a2333 = f655(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2331, _) => {
                                            h2331
                                        }
                                    })?;
                                    let a2334 = f649(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2332) => {
                                            h2332
                                        }
                                    });
                                    append_list(a2333, a2334)
                                };
                                append_list(a2335, a2336)
                            };
                            append_list(a2337, a2338)
                        };
                        append_list(a2339, a2340)
                    };
                    append_list(a2341, a2342)
                };
                append_list(a2343, a2344)
            };
            append_list(a2345, a2346)
        };
        append_list(a2347, a2348)
    }))
}

pub fn f599(v0: Adt47) -> Adt6 {
    let m2349 = v0;
    match m2349 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2350, _, _, _) => {
                    h2350
                }
            })), {
                let a2354 = f430(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2351, _, _) => {
                        h2351
                    }
                });
                let a2355 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2352, _) => {
                        h2352
                    }
                })), f649(match v1 {
                    Adt48::C0(_, _, _, h2353) => {
                        h2353
                    }
                }));
                append_list(a2354, a2355)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2356, _, _, _) => {
                    h2356
                }
            })), {
                let a2360 = f430(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2357, _, _) => {
                        h2357
                    }
                });
                let a2361 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2358, _) => {
                        h2358
                    }
                })), f649(match v2 {
                    Adt49::C0(_, _, _, h2359) => {
                        h2359
                    }
                }));
                append_list(a2360, a2361)
            }))
        }
    }
}

pub fn f600(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2382 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2362, _, _, _, _, _, _, _) => {
                h2362
            }
        })), List::<(Str, Adt6)>::nil());
        let a2383 = {
            let a2380 = f430(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2363, _, _, _, _, _, _) => {
                    h2363
                }
            });
            let a2381 = {
                let a2378 = f531(match v0.clone() {
                    Adt55::C0(_, _, h2364, _, _, _, _, _) => {
                        h2364
                    }
                });
                let a2379 = {
                    let a2376 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2365, _, _, _, _) => {
                            h2365
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2377 = {
                        let a2374 = f430(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2366, _, _, _) => {
                                h2366
                            }
                        });
                        let a2375 = {
                            let a2372 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2367, _, _) => {
                                    h2367
                                }
                            });
                            let a2373 = {
                                let a2370 = f654(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2368, _) => {
                                        h2368
                                    }
                                });
                                let a2371 = f649(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2369) => {
                                        h2369
                                    }
                                });
                                append_list(a2370, a2371)
                            };
                            append_list(a2372, a2373)
                        };
                        append_list(a2374, a2375)
                    };
                    append_list(a2376, a2377)
                };
                append_list(a2378, a2379)
            };
            append_list(a2380, a2381)
        };
        append_list(a2382, a2383)
    })
}

pub fn f601(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2399 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2384, _, _, _, _, _) => {
                h2384
            }
        })), List::<(Str, Adt6)>::nil());
        let a2400 = {
            let a2397 = f430(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2385, _, _, _, _) => {
                    h2385
                }
            });
            let a2398 = {
                let a2395 = f531(match v0.clone() {
                    Adt57::C0(_, _, h2386, _, _, _) => {
                        h2386
                    }
                });
                let a2396 = {
                    let a2393 = f430(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2387, _, _) => {
                            h2387
                        }
                    });
                    let a2394 = {
                        let a2391 = {
                            let m2389 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2388, _) => {
                                    h2388
                                }
                            };
                            match m2389 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f656(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2392 = f649(match v0 {
                            Adt57::C0(_, _, _, _, _, h2390) => {
                                h2390
                            }
                        });
                        append_list(a2391, a2392)
                    };
                    append_list(a2393, a2394)
                };
                append_list(a2395, a2396)
            };
            append_list(a2397, a2398)
        };
        append_list(a2399, a2400)
    })
}

pub fn f602(v0: Adt60) -> R<Option<Adt6>> {
    let m2402 = f597(match v0.clone() {
        Adt60::C0(_, h2401) => {
            h2401
        }
    })?;
    match m2402 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f657(match v0 {
                Adt60::C0(h2403, _) => {
                    h2403
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f603(v0: Adt65) -> Option<Adt6> {
    let m2404 = v0;
    match m2404 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f658(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2405 = v3;
                match m2405 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2406) => {
                                h2406
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f604(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f605(v0: List<Adt2>) -> List<Adt6> {
    f311(f659(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m2407 = f182(v0, v1);
    match m2407 {
        Adt135::C0 => {
            None::<Adt5>
        }
        Adt135::C1(v2) => {
            f261(v2)
        }
    }
}

pub fn f607(v0: Adt5) -> Option<Adt63> {
    let m2408 = f660(v0.clone());
    match m2408 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m2409 = f661(Str::lit("status"), v0.clone());
            match m2409 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a2410 = v2.clone();
                    let a2411 = Str::lit("working");
                    if equal(a2410, a2411) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a2412 = v2.clone();
                        let a2413 = Str::lit("cancelled");
                        if equal(a2412, a2413) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a2414 = v2.clone();
                            let a2415 = Str::lit("input_required");
                            if equal(a2414, a2415) {
                                let m2416 = f662(Str::lit("inputRequests"), v0);
                                match m2416 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a2417 = v2.clone();
                                let a2418 = Str::lit("completed");
                                if equal(a2417, a2418) {
                                    let m2419 = f606(Str::lit("result"), v0);
                                    match m2419 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a2420 = v2;
                                    let a2421 = Str::lit("failed");
                                    if equal(a2420, a2421) {
                                        let m2422 = f606(Str::lit("error"), v0);
                                        match m2422 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m2423 = f663(v5);
                                                match m2423 {
                                                    None => {
                                                        None::<Adt63>
                                                    }
                                                    Some(v6) => {
                                                        Some(Adt63::C3(v1, v6))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        None::<Adt63>
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f608(v0: Str, v1: Adt5) -> Option<i64> {
    let m2424 = f182(v0, v1);
    match m2424 {
        Adt135::C0 => {
            None::<i64>
        }
        Adt135::C1(v2) => {
            f590(v2)
        }
    }
}

pub fn f609(v0: Adt5) -> Option<Adt86> {
    let m2425 = f608(Str::lit("parkAfterMs"), v0.clone());
    match m2425 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m2426 = f662(Str::lit("parked"), v0.clone());
            match m2426 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m2427 = f608(Str::lit("finishAfterMs"), v0.clone());
                    match m2427 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m2428 = f606(Str::lit("outcome"), v0);
                            match m2428 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m2429 = f664(v4);
                                    match m2429 {
                                        None => {
                                            None::<Adt86>
                                        }
                                        Some(v5) => {
                                            Some(Adt86::C0(v1, v2, v3, v5))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f610(v0: Adt85) -> Option<Adt6> {
    let m2431 = f357(match v0.clone() {
        Adt85::C0(h2430, _, _) => {
            h2430
        }
    });
    match m2431 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2433 = f665(match v0.clone() {
                Adt85::C0(_, _, h2432) => {
                    h2432
                }
            });
            match m2433 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f604(match v0 {
                        Adt85::C0(_, h2434, _) => {
                            h2434
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f611(v0: List<u64>) -> List<u64> {
    let a2437 = f666();
    let a2438 = {
        let a2435 = v0;
        let a2436 = f667();
        append_list(a2435, a2436)
    };
    append_list(a2437, a2438)
}

pub fn f612(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f668(v0, 0u64, v1)
}

pub fn f613(p2445: List<u64>) -> bool {
    let m2444 = p2445;
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let v0 = m2444;
                let m2439 = v0;
                match m2439.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2440 = v1;
                        let a2441 = 48u64;
                        if if nat_eq(a2440, a2441) {
                            true
                        } else {
                            let a2442 = v1;
                            let a2443 = 46u64;
                            nat_eq(a2442, a2443)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Adt140) -> bool {
    let m2446 = v0;
    match m2446 {
        Adt140::C0 => {
            true
        }
        Adt140::C1 => {
            false
        }
        Adt140::C2 => {
            false
        }
        Adt140::C3 => {
            false
        }
        Adt140::C4 => {
            false
        }
        Adt140::C5 => {
            false
        }
        Adt140::C6 => {
            false
        }
        Adt140::C7 => {
            false
        }
        Adt140::C8 => {
            false
        }
        Adt140::C9 => {
            false
        }
        Adt140::C10 => {
            false
        }
        Adt140::C11 => {
            false
        }
    }
}

pub fn f615(v0: Adt15) -> Adt140 {
    let m2447 = v0;
    match m2447 {
        Adt15::C0(_) => {
            Adt140::C0
        }
        Adt15::C1(_) => {
            Adt140::C1
        }
        Adt15::C2(_) => {
            Adt140::C2
        }
        Adt15::C3(_) => {
            Adt140::C3
        }
        Adt15::C4(_) => {
            Adt140::C4
        }
        Adt15::C5(_) => {
            Adt140::C5
        }
        Adt15::C6(_) => {
            Adt140::C6
        }
        Adt15::C7(_) => {
            Adt140::C7
        }
        Adt15::C8(_) => {
            Adt140::C8
        }
        Adt15::C9(_) => {
            Adt140::C9
        }
        Adt15::C10(_) => {
            Adt140::C10
        }
        Adt15::C11(_) => {
            Adt140::C11
        }
    }
}

pub fn f616(v0: Adt0, v1: Adt33) -> Adt128 {
    Adt128::C0(v1, Adt129::C0(Some(match v0 {
        Adt0::C0(h2448, _, _, _, _, _) => {
            h2448
        }
    }), None::<Adt5>), List::<Adt130>::nil())
}

pub fn f617(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2449, _, _) => {
            h2449
        }
    }, f180(), match v0.clone() {
        Adt0::C0(_, _, h2450, _, _, _) => {
            h2450
        }
    }, match v0 {
        Adt0::C0(_, h2451, _, _, _, _) => {
            h2451
        }
    })
}

pub fn f618(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt128, Adt131> {
    let m2452 = v2;
    match m2452 {
        Ok(v4) => {
            if f669(f615(v1.clone()), v4.clone()) {
                f670(v0, v1, v4)
            } else {
                Err::<Adt128, Adt131>(Adt131::C4(f671()))
            }
        }
        Err(v3) => {
            Err::<Adt128, Adt131>(Adt131::C4(v3))
        }
    }
}

pub fn f619(v0: u64, v1: u64, v2: Fn2, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2453 = v3.clone();
    match m2453 {
        Adt15::C0(_) => {
            let c2455: Fn2 = v2;
            c2455.apply(v3)
        }
        Adt15::C1(_) => {
            let c2456: Fn2 = v2;
            c2456.apply(v3)
        }
        Adt15::C2(_) => {
            if f672(v0, v1) {
                let c2454: Fn2 = v2;
                c2454.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2457: Fn2 = v2;
            c2457.apply(v3)
        }
        Adt15::C4(_) => {
            let c2458: Fn2 = v2;
            c2458.apply(v3)
        }
        Adt15::C5(_) => {
            let c2459: Fn2 = v2;
            c2459.apply(v3)
        }
        Adt15::C6(_) => {
            let c2460: Fn2 = v2;
            c2460.apply(v3)
        }
        Adt15::C7(_) => {
            let c2461: Fn2 = v2;
            c2461.apply(v3)
        }
        Adt15::C8(_) => {
            let c2462: Fn2 = v2;
            c2462.apply(v3)
        }
        Adt15::C9(_) => {
            let c2463: Fn2 = v2;
            c2463.apply(v3)
        }
        Adt15::C10(_) => {
            let c2464: Fn2 = v2;
            c2464.apply(v3)
        }
        Adt15::C11(_) => {
            let c2465: Fn2 = v2;
            c2465.apply(v3)
        }
    }
}

pub fn f620(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt132> {
    let m2466 = f182(v0.clone(), v1);
    match m2466 {
        Adt135::C0 => {
            Ok::<Option<Str>, Adt132>(None::<Str>)
        }
        Adt135::C1(v2) => {
            let m2467 = f235(v2);
            match m2467 {
                None => {
                    Err::<Option<Str>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt132>(Some(v3))
                }
            }
        }
    }
}

pub fn f621(v0: Str, v1: Adt5) -> Result<Str, Adt132> {
    let m2468 = f182(v0.clone(), v1);
    match m2468 {
        Adt135::C0 => {
            Err::<Str, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2469 = f235(v2);
            match m2469 {
                None => {
                    Err::<Str, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt132>(v3)
                }
            }
        }
    }
}

pub fn f622(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt132> {
    let m2470 = f182(v0.clone(), v1);
    match m2470 {
        Adt135::C0 => {
            Ok::<Option<Adt5>, Adt132>(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2471 = f261(v2);
            match m2471 {
                None => {
                    Err::<Option<Adt5>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt132>(Some(v3))
                }
            }
        }
    }
}

pub fn f623(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt132> {
    let m2472 = f182(v0.clone(), v1);
    match m2472 {
        Adt135::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt132>(None::<List<(Str, Str)>>)
        }
        Adt135::C1(v2) => {
            let m2473 = f261(v2);
            match m2473 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    let m2475 = f673(v0, match v3 {
                        Adt5::C0(h2474) => {
                            h2474
                        }
                    });
                    match m2475 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt132>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt132>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f624(v0: Str, v1: Adt5) -> Result<Adt5, Adt132> {
    let m2476 = f182(v0.clone(), v1);
    match m2476 {
        Adt135::C0 => {
            Err::<Adt5, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2477 = f261(v2);
            match m2477 {
                None => {
                    Err::<Adt5, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt132>(v3)
                }
            }
        }
    }
}

pub fn f625(v0: Adt5) -> Result<Adt26, Adt132> {
    let m2478 = f621(Str::lit("type"), v0.clone());
    match m2478 {
        Ok(v1) => {
            let a2479 = v1.clone();
            let a2480 = Str::lit("ref/prompt");
            if equal(a2479, a2480) {
                let m2481 = f621(Str::lit("name"), v0.clone());
                match m2481 {
                    Ok(v2) => {
                        let m2482 = f620(Str::lit("title"), v0);
                        match m2482 {
                            Ok(v3) => {
                                Ok::<Adt26, Adt132>(Adt26::C0(Adt27::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt26, Adt132>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt26, Adt132>(v5)
                    }
                }
            } else {
                let a2483 = v1;
                let a2484 = Str::lit("ref/resource");
                if equal(a2483, a2484) {
                    let m2485 = f621(Str::lit("uri"), v0);
                    match m2485 {
                        Ok(v6) => {
                            Ok::<Adt26, Adt132>(Adt26::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt26, Adt132>(v7)
                        }
                    }
                } else {
                    Err::<Adt26, Adt132>(Adt132::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt26, Adt132>(v8)
        }
    }
}

pub fn f626(v0: Adt5) -> Result<Adt28, Adt132> {
    let m2486 = f621(Str::lit("name"), v0.clone());
    match m2486 {
        Ok(v1) => {
            let m2487 = f621(Str::lit("value"), v0);
            match m2487 {
                Ok(v2) => {
                    Ok::<Adt28, Adt132>(Adt28::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt28, Adt132>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt28, Adt132>(v4)
        }
    }
}

pub fn f627(v0: Adt5) -> Result<Option<Adt29>, Adt132> {
    let m2488 = f182(Str::lit("context"), v0);
    match m2488 {
        Adt135::C0 => {
            Ok::<Option<Adt29>, Adt132>(None::<Adt29>)
        }
        Adt135::C1(v1) => {
            let m2489 = f261(v1);
            match m2489 {
                None => {
                    Err::<Option<Adt29>, Adt132>(Adt132::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2490 = f623(Str::lit("arguments"), v2);
                    match m2490 {
                        Ok(v3) => {
                            Ok::<Option<Adt29>, Adt132>(Some(Adt29::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt29>, Adt132>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f628(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt131> {
    if v0 {
        None::<Adt131>
    } else {
        Some(Adt131::C1(v1, v2))
    }
}

pub fn f629(v0: Str, p2495: List<Str>) -> bool {
    let m2494 = p2495;
    {
        let mut m2494 = m2494;
        loop {
            let step = {
                let v1 = m2494;
                let m2491 = v1;
                match m2491.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2492 = v2;
                        let a2493 = v0.clone();
                        if equal(a2492, a2493) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: Option<Adt113>, v1: List<Adt115>, v2: Adt5) -> R<Option<Adt113>> {
    let m2496 = v0;
    match m2496 {
        None => {
            f674(v1, f675(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f631(v0: Option<Str>, v1: Adt145, v2: Adt5) -> Option<Adt113> {
    let m2497 = v1.clone();
    match m2497 {
        Adt145::C0 => {
            None::<Adt113>
        }
        Adt145::C1 => {
            f676(v0, f677(v1, v2))
        }
        Adt145::C2 => {
            f676(v0, f677(v1, v2))
        }
        Adt145::C3 => {
            f676(v0, f677(v1, v2))
        }
    }
}

pub fn f632(v0: Adt135, v1: Adt135) -> Adt154 {
    let m2498 = v0;
    match m2498 {
        Adt135::C0 => {
            Adt154::C1
        }
        Adt135::C1(_) => {
            let m2499 = v1;
            match m2499 {
                Adt135::C0 => {
                    Adt154::C2
                }
                Adt135::C1(_) => {
                    Adt154::C0
                }
            }
        }
    }
}

pub fn f633(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt132> {
    let m2500 = f235(v1);
    match m2500 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C5)
        }
        Some(v3) => {
            let m2501 = f678(v2);
            match m2501 {
                None => {
                    Err::<Adt17, Adt132>(Adt132::C6)
                }
                Some(v4) => {
                    let m2502 = f622(f679(), v0.clone());
                    match m2502 {
                        Ok(v6) => {
                            let m2503 = f680(v0.clone());
                            match m2503 {
                                Ok(v8) => {
                                    let m2504 = f681(v0.clone());
                                    match m2504 {
                                        Ok(v10) => {
                                            Ok::<Adt17, Adt132>(Adt17::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt17, Adt132>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt17, Adt132>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt17, Adt132>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f634(v0: u64, v1: u64) -> u64 {
    let a2505 = v0;
    let a2506 = v1;
    let a2507 = 0u64;
    nat_rem(a2505, a2506, a2507)
}

pub fn f635(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2510 = {
            let a2508 = v0;
            let a2509 = 4u64;
            nat_mul(a2508, a2509)?
        };
        let a2511 = f637(v1, 16u64);
        nat_add(a2510, a2511)?
    }, List::<u64>::nil()))
}

pub fn f636(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2514 = {
            let a2512 = v0;
            let a2513 = 4u64;
            nat_mul(a2512, a2513)?
        };
        let a2515 = f637(v1, 16u64);
        nat_add(a2514, a2515)?
    }, List::cons({
        let a2518 = {
            let a2516 = f634(v1, 16u64);
            let a2517 = 16u64;
            nat_mul(a2516, a2517)?
        };
        let a2519 = f637(v2, 4u64);
        nat_add(a2518, a2519)?
    }, List::<u64>::nil())))
}

pub fn f637(v0: u64, v1: u64) -> u64 {
    let a2520 = v0;
    let a2521 = v1;
    let a2522 = 0u64;
    nat_quot(a2520, a2521, a2522)
}

pub fn f638(p2526: List<Adt6>) -> bool {
    let m2525 = p2526;
    {
        let mut m2525 = m2525;
        loop {
            let step = {
                let v0 = m2525;
                let m2523 = v0;
                match m2523.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2524 = f235(v1);
                        match m2524 {
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
                Ok(next) => { m2525 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639(p2530: List<Adt6>) -> bool {
    let m2529 = p2530;
    {
        let mut m2529 = m2529;
        loop {
            let step = {
                let v0 = m2529;
                let m2527 = v0;
                match m2527.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2528 = f235(v1);
                        match m2528 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f495(v3) {
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
                Ok(next) => { m2529 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: Str) -> Option<Adt13> {
    let a2531 = v0.clone();
    let a2532 = Str::lit("string");
    if equal(a2531, a2532) {
        Some(Adt13::C0)
    } else {
        let a2533 = v0.clone();
        let a2534 = Str::lit("integer");
        if equal(a2533, a2534) {
            Some(Adt13::C1)
        } else {
            let a2535 = v0;
            let a2536 = Str::lit("boolean");
            if equal(a2535, a2536) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f641(p2545: List<u64>, p2546: List<u64>) -> R<List<u64>> {
    let m2544 = (p2545, p2546);
    {
        let mut m2544 = m2544;
        loop {
            let step = {
                let (v0, v1) = m2544;
                let m2537 = v0;
                match m2537.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2538 = 65u64;
                        let a2539 = v2;
                        if if nat_le(a2538, a2539) {
                            let a2540 = v2;
                            let a2541 = 90u64;
                            nat_le(a2540, a2541)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2542 = v2;
                                let a2543 = 32u64;
                                nat_add(a2542, a2543)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(p2553: List<u64>, p2554: List<u64>) -> bool {
    let m2552 = (p2553, p2554);
    {
        let mut m2552 = m2552;
        loop {
            let step = {
                let (v0, v1) = m2552;
                let m2547 = v0;
                match m2547.uncons() {
                    None => {
                        let m2548 = v1;
                        match m2548.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2549 = v1;
                        match m2549.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2550 = v4;
                                let a2551 = v6;
                                if nat_eq(a2550, a2551) {
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
                Ok(next) => { m2552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2566: u64, p2567: i64, p2568: i64) -> R<bool> {
    let m2565 = (p2566, (p2567, p2568));
    {
        let mut m2565 = m2565;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2565;
                let m2555 = v0;
                if m2555 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2555 - 1;
                    let m2558 = {
                        let a2556 = v2;
                        let a2557 = 0i64;
                        compare(a2556, a2557)
                    };
                    if match m2558 {
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
                        if f395({
                            let a2559 = v1;
                            let a2560 = 10i64;
                            let a2561 = 0i64;
                            int_rem(a2559, a2560, a2561)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2562 = v1;
                                let a2563 = 10i64;
                                let a2564 = 0i64;
                                int_quot(a2562, a2563, a2564)?
                            }, f115(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2565 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt155> {
    let a2569 = f682(v0.clone());
    let a2570 = f682(v1.clone());
    if nat_eq(a2569, a2570) {
        let m2571 = v0;
        match m2571 {
            Adt6::C0 => {
                Ok(Adt155::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2572 = f260(v1);
                match m2572 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v4) => {
                        let a2573 = v3;
                        let a2574 = v4;
                        if equal(a2573, a2574) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2575 = f500(v1);
                match m2575 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v6) => {
                        if f683(f502(v5, v6)?) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2576 = f235(v1);
                match m2576 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v8) => {
                        let a2577 = v7;
                        let a2578 = v8;
                        if equal(a2577, a2578) {
                            Ok(Adt155::C1(v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2579 = f334(v1);
                match m2579 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v10) => {
                        Ok(f684(f685(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2580 = f261(v1);
                match m2580 {
                    None => {
                        Ok(Adt155::C0)
                    }
                    Some(v12) => {
                        let a2582 = f506(v11.clone(), 0u64)?;
                        let a2583 = f506(match v12.clone() {
                            Adt5::C0(h2581) => {
                                h2581
                            }
                        }, 0u64)?;
                        if nat_eq(a2582, a2583) {
                            Ok(f684(f686(v11, match v12 {
                                Adt5::C0(h2584) => {
                                    h2584
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt155::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt155::C0)
    }
}

pub fn f645(v0: Adt153, v1: Adt153) -> Adt153 {
    let m2585 = v0;
    match m2585 {
        Adt153::C0 => {
            f687(v1)
        }
        Adt153::C1 => {
            Adt153::C1
        }
        Adt153::C2 => {
            v1
        }
    }
}

pub fn f646(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt153> {
    let m2586 = f591(f688(v0, v1)?, f688(v2, v3)?);
    match m2586 {
        Adt153::C0 => {
            Ok(Adt153::C0)
        }
        Adt153::C1 => {
            f689(v0, f690(v0)?, v2, f690(v2)?)
        }
        Adt153::C2 => {
            Ok(Adt153::C2)
        }
    }
}

pub fn f647(v0: i64) -> R<i64> {
    let m2589 = {
        let a2587 = v0;
        let a2588 = 0i64;
        compare(a2587, a2588)
    };
    if match m2589 {
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
        let a2590 = v0;
        int_neg(a2590)
    } else {
        Ok(v0)
    }
}

pub fn f648(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m2591 = v0;
    match m2591 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2603 = f430(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h2592, _, _, _, _) => {
                        h2592
                    }
                });
                let a2604 = {
                    let a2601 = f435(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h2593, _, _, _) => {
                            h2593
                        }
                    });
                    let a2602 = {
                        let a2599 = f435(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h2594, _, _) => {
                                h2594
                            }
                        });
                        let a2600 = {
                            let a2597 = f435(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h2595, _) => {
                                    h2595
                                }
                            });
                            let a2598 = f435(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h2596) => {
                                    h2596
                                }
                            });
                            append_list(a2597, a2598)
                        };
                        append_list(a2599, a2600)
                    };
                    append_list(a2601, a2602)
                };
                append_list(a2603, a2604)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f649(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f470(Str::lit("_meta"), v0)
}

pub fn f650(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2605, _, _) => {
            h2605
        }
    })), {
        let a2608 = f654(match v0.clone() {
            Adt40::C0(_, h2606, _) => {
                h2606
            }
        });
        let a2609 = f649(match v0 {
            Adt40::C0(_, _, h2607) => {
                h2607
            }
        });
        append_list(a2608, a2609)
    })))
}

pub fn f651(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2610 = f654(v3);
        let a2611 = f649(v4);
        append_list(a2610, a2611)
    }))))
}

pub fn f652(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2635 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2612, _, _, _, _, _, _, _, _) => {
                h2612
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2636 = {
            let a2633 = f430(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2613, _, _, _, _, _, _, _) => {
                    h2613
                }
            });
            let a2634 = {
                let a2631 = f531(match v0.clone() {
                    Adt45::C0(_, _, h2614, _, _, _, _, _, _) => {
                        h2614
                    }
                });
                let a2632 = {
                    let a2629 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2615, _, _, _, _, _) => {
                            h2615
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2630 = {
                        let a2627 = f430(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2616, _, _, _, _) => {
                                h2616
                            }
                        });
                        let a2628 = {
                            let a2625 = f430(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2617, _, _, _) => {
                                    h2617
                                }
                            });
                            let a2626 = {
                                let a2623 = f654(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2618, _, _) => {
                                        h2618
                                    }
                                });
                                let a2624 = {
                                    let a2621 = f655(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2619, _) => {
                                            h2619
                                        }
                                    })?;
                                    let a2622 = f649(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2620) => {
                                            h2620
                                        }
                                    });
                                    append_list(a2621, a2622)
                                };
                                append_list(a2623, a2624)
                            };
                            append_list(a2625, a2626)
                        };
                        append_list(a2627, a2628)
                    };
                    append_list(a2629, a2630)
                };
                append_list(a2631, a2632)
            };
            append_list(a2633, a2634)
        };
        append_list(a2635, a2636)
    }))
}

pub fn f653(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f599(match v0.clone() {
        Adt46::C0(h2637, _, _) => {
            h2637
        }
    })), {
        let a2640 = f654(match v0.clone() {
            Adt46::C0(_, h2638, _) => {
                h2638
            }
        });
        let a2641 = f649(match v0 {
            Adt46::C0(_, _, h2639) => {
                h2639
            }
        });
        append_list(a2640, a2641)
    })))
}

pub fn f654(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2642 = v0;
    match m2642 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2648 = f691(match v1.clone() {
                    Adt41::C0(h2643, _, _) => {
                        h2643
                    }
                });
                let a2649 = {
                    let a2646 = f692(match v1.clone() {
                        Adt41::C0(_, h2644, _) => {
                            h2644
                        }
                    });
                    let a2647 = f430(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2645) => {
                            h2645
                        }
                    });
                    append_list(a2646, a2647)
                };
                append_list(a2648, a2649)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f655(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2650 = v0;
    match m2650 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f519(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f656(v0: List<Adt58>) -> List<Adt6> {
    f311(f693(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f657(v0: Adt42) -> Str {
    let m2651 = v0;
    match m2651 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f658(v0: Adt66) -> List<(Str, Adt6)> {
    let m2652 = v0;
    match m2652 {
        Adt66::C0(v1) => {
            let a2659 = {
                let m2654 = match v1.clone() {
                    Adt67::C0(h2653, _, _) => {
                        h2653
                    }
                };
                match m2654 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2660 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h2655, _) => {
                    h2655
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m2658 = match v1 {
                    Adt67::C0(_, _, h2657) => {
                        h2657
                    }
                };
                match m2658 {
                    Adt5::C0(h2656) => {
                        h2656
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a2659, a2660)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h2661, _) => {
                    h2661
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h2662) => {
                    h2662
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f659(p2665: List<Adt2>, p2666: List<Adt6>) -> List<Adt6> {
    let m2664 = (p2665, p2666);
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let (v0, v1) = m2664;
                let m2663 = v0;
                match m2663.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f694(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f660(v0: Adt5) -> Option<Adt64> {
    let m2667 = f661(Str::lit("taskId"), v0.clone());
    match m2667 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m2668 = f661(Str::lit("createdAt"), v0.clone());
            match m2668 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m2669 = f661(Str::lit("lastUpdatedAt"), v0.clone());
                    match m2669 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f661(Str::lit("statusMessage"), v0.clone()), v2, v3, f608(Str::lit("ttlMs"), v0.clone()), f608(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f661(v0: Str, v1: Adt5) -> Option<Str> {
    let m2670 = f182(v0, v1);
    match m2670 {
        Adt135::C0 => {
            None::<Str>
        }
        Adt135::C1(v2) => {
            f235(v2)
        }
    }
}

pub fn f662(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m2671 = f606(v0, v1);
    match m2671 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f695(match v2 {
                Adt5::C0(h2672) => {
                    h2672
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f663(v0: Adt5) -> Option<Adt81> {
    let m2673 = f608(Str::lit("code"), v0.clone());
    match m2673 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m2674 = f661(Str::lit("message"), v0.clone());
            match m2674 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m2675 = f182(Str::lit("data"), v0);
                        match m2675 {
                            Adt135::C0 => {
                                None::<Adt6>
                            }
                            Adt135::C1(v3) => {
                                Some(v3)
                            }
                        }
                    }))
                }
            }
        }
    }
}

pub fn f664(v0: Adt5) -> Option<Adt87> {
    let m2676 = f606(Str::lit("completes"), v0.clone());
    match m2676 {
        None => {
            let m2677 = f606(Str::lit("fails"), v0);
            match m2677 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m2678 = f663(v2);
                    match m2678 {
                        None => {
                            None::<Adt87>
                        }
                        Some(v3) => {
                            Some(Adt87::C1(v3))
                        }
                    }
                }
            }
        }
        Some(v1) => {
            Some(Adt87::C0(v1))
        }
    }
}

pub fn f665(v0: Adt86) -> Option<Adt6> {
    let m2680 = f442(match v0.clone() {
        Adt86::C0(_, h2679, _, _) => {
            h2679
        }
    });
    match m2680 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f604(match v0.clone() {
                Adt86::C0(h2681, _, _, _) => {
                    h2681
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f604(match v0.clone() {
                Adt86::C0(_, _, h2682, _) => {
                    h2682
                }
            })), List::cons((Str::lit("outcome"), f696(match v0 {
                Adt86::C0(_, _, _, h2683) => {
                    h2683
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f666() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f667() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f668(p2688: List<u64>, p2689: u64, p2690: List<u64>) -> List<u64> {
    let m2687 = (p2688, (p2689, p2690));
    {
        let mut m2687 = m2687;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2687;
                let m2684 = v0;
                match m2684.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f697(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt156 = f698(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt156::C0(h2685, _) => {
                                h2685
                            }
                        }, match v5 {
                            Adt156::C0(_, h2686) => {
                                h2686
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f669(v0: Adt140, v1: Adt33) -> bool {
    let m2691 = v1;
    match m2691 {
        Adt33::C0(_) => {
            f699(v0, Adt140::C0)
        }
        Adt33::C1(_) => {
            f699(v0, Adt140::C1)
        }
        Adt33::C2(_) => {
            f699(v0, Adt140::C2)
        }
        Adt33::C3(_) => {
            f699(v0, Adt140::C2)
        }
        Adt33::C4(_) => {
            f699(v0, Adt140::C3)
        }
        Adt33::C5(_) => {
            f699(v0, Adt140::C4)
        }
        Adt33::C6(_) => {
            f699(v0, Adt140::C5)
        }
        Adt33::C7(_) => {
            f699(v0, Adt140::C6)
        }
        Adt33::C8(_) => {
            f699(v0, Adt140::C7)
        }
        Adt33::C9(_) => {
            f699(v0, Adt140::C8)
        }
        Adt33::C10(_) => {
            f699(v0, Adt140::C9)
        }
        Adt33::C11 => {
            if f699(v0.clone(), Adt140::C10) {
                true
            } else {
                f699(v0, Adt140::C11)
            }
        }
        Adt33::C12(_) => {
            f700(v0)
        }
        Adt33::C13(_) => {
            f701(v0)
        }
        Adt33::C14(v15) => {
            let m2693 = match v15 {
                Adt88::C0(_, h2692, _) => {
                    h2692
                }
            };
            match m2693 {
                Adt89::C0 => {
                    f699(v0, Adt140::C9)
                }
                Adt89::C1 => {
                    if f699(v0.clone(), Adt140::C10) {
                        true
                    } else {
                        f699(v0, Adt140::C11)
                    }
                }
            }
        }
    }
}

pub fn f670(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt128, Adt131> {
    let m2694 = v2.clone();
    match m2694 {
        Adt33::C0(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C1(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C2(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C3(v9) => {
            Ok::<Adt128, Adt131>(f706(v0, f703(v1), v9))
        }
        Adt33::C4(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C5(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C6(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C7(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C8(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C9(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C10(_) => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C11 => {
            Ok::<Adt128, Adt131>(f616(v0, v2))
        }
        Adt33::C12(v3) => {
            f702(v0, f703(v1), v3)
        }
        Adt33::C13(v4) => {
            f704(v0, f703(v1), v4)
        }
        Adt33::C14(v5) => {
            f705(v0, f703(v1), v5)
        }
    }
}

pub fn f671() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f672(v0: u64, v1: u64) -> bool {
    let a2695 = v1;
    let a2696 = v0;
    nat_lt(a2695, a2696)
}

pub fn f673(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2697 = f707(v0, v1, List::<(Str, Str)>::nil());
    match m2697 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt132>(f708(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt132>(v3)
        }
    }
}

pub fn f674(p2701: List<Adt115>, v1: Adt5) -> R<Option<Adt113>> {
    let m2700 = p2701;
    {
        let mut m2700 = m2700;
        loop {
            let step = {
                let v0 = m2700;
                let m2698 = v0;
                match m2698.uncons() {
                    None => {
                        Err::<List<Adt115>, R<Option<Adt113>>>(Ok(None::<Adt113>))
                    }
                    Some((v2, v3)) => {
                        let m2699 = f709(v2, v1.clone())?;
                        match m2699 {
                            None => {
                                Ok::<List<Adt115>, R<Option<Adt113>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt115>, R<Option<Adt113>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(v0: Adt5) -> Adt5 {
    let m2702 = f182(Str::lit("arguments"), v0);
    match m2702 {
        Adt135::C0 => {
            f559()
        }
        Adt135::C1(v1) => {
            let m2703 = f261(v1);
            match m2703 {
                None => {
                    f559()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f676(v0: Option<Str>, v1: Option<Str>) -> Option<Adt113> {
    let m2704 = v0;
    match m2704 {
        None => {
            Some(Adt113::C2)
        }
        Some(v2) => {
            let m2705 = v1;
            match m2705 {
                None => {
                    None::<Adt113>
                }
                Some(v3) => {
                    let a2706 = v2;
                    let a2707 = v3;
                    if equal(a2706, a2707) {
                        None::<Adt113>
                    } else {
                        Some(Adt113::C5)
                    }
                }
            }
        }
    }
}

pub fn f677(v0: Adt145, v1: Adt5) -> Option<Str> {
    let m2708 = v0;
    match m2708 {
        Adt145::C0 => {
            None::<Str>
        }
        Adt145::C1 => {
            let m2709 = f182(Str::lit("name"), v1);
            match m2709 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v2) => {
                    f235(v2)
                }
            }
        }
        Adt145::C2 => {
            let m2710 = f182(Str::lit("uri"), v1);
            match m2710 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v3) => {
                    f235(v3)
                }
            }
        }
        Adt145::C3 => {
            let m2711 = f182(Str::lit("taskId"), v1);
            match m2711 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v4) => {
                    f235(v4)
                }
            }
        }
    }
}

pub fn f678(v0: Adt6) -> Option<Adt18> {
    let m2712 = f261(v0);
    match m2712 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m2713 = f710(Str::lit("experimental"), v1.clone());
            match m2713 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m2714 = f710(Str::lit("roots"), v1.clone());
                    match m2714 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m2715 = f710(Str::lit("sampling"), v1.clone());
                            match m2715 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m2716 = f710(Str::lit("elicitation"), v1.clone());
                                    match m2716 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m2717 = f710(Str::lit("extensions"), v1);
                                            match m2717 {
                                                None => {
                                                    None::<Adt18>
                                                }
                                                Some(v6) => {
                                                    Some(Adt18::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f679() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f680(v0: Adt5) -> Result<Option<Adt19>, Adt132> {
    let m2718 = f620(f711(), v0);
    match m2718 {
        Ok(v2) => {
            let m2719 = v2;
            match m2719 {
                None => {
                    Ok::<Option<Adt19>, Adt132>(None::<Adt19>)
                }
                Some(v3) => {
                    let m2720 = f712(v3);
                    match m2720 {
                        Adt157::C0 => {
                            Err::<Option<Adt19>, Adt132>(Adt132::C8(f711()))
                        }
                        Adt157::C1(v4) => {
                            Ok::<Option<Adt19>, Adt132>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt19>, Adt132>(v1)
        }
    }
}

pub fn f681(v0: Adt5) -> Result<Option<Adt20>, Adt132> {
    let m2721 = f182(f713(), v0);
    match m2721 {
        Adt135::C0 => {
            Ok::<Option<Adt20>, Adt132>(None::<Adt20>)
        }
        Adt135::C1(v1) => {
            let m2722 = f235(v1.clone());
            match m2722 {
                None => {
                    let m2723 = f590(v1);
                    match m2723 {
                        None => {
                            Err::<Option<Adt20>, Adt132>(Adt132::C8(f713()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt20>, Adt132>(Some(Adt20::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt20>, Adt132>(Some(Adt20::C0(v2)))
                }
            }
        }
    }
}

pub fn f682(v0: Adt6) -> u64 {
    let m2724 = v0;
    match m2724 {
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

pub fn f683(v0: Adt153) -> bool {
    let m2725 = v0;
    match m2725 {
        Adt153::C0 => {
            false
        }
        Adt153::C1 => {
            true
        }
        Adt153::C2 => {
            false
        }
    }
}

pub fn f684(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt155 {
    let m2726 = v0;
    match m2726 {
        None => {
            Adt155::C0
        }
        Some(v2) => {
            Adt155::C1({
                let a2727 = v2;
                let a2728 = v1;
                append_list(a2727, a2728)
            })
        }
    }
}

pub fn f685(p2733: List<Adt6>, p2734: List<Adt6>, p2735: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2732 = (p2733, (p2734, p2735));
    {
        let mut m2732 = m2732;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2732;
                let m2729 = v0;
                match m2729.uncons() {
                    None => {
                        let m2730 = v1;
                        match m2730.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2731 = v1;
                        match m2731.uncons() {
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
                Ok(next) => { m2732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(p2741: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2742: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2740 = (p2741, p2742);
    {
        let mut m2740 = m2740;
        loop {
            let step = {
                let (v0, v2) = m2740;
                let m2736 = v0;
                match m2736.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2738 = f238({
                            let (h2737, _) = v3;
                            h2737
                        }, v1.clone());
                        match m2738 {
                            Adt135::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt135::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2739) = v3;
                                    h2739
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2740 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f687(v0: Adt153) -> Adt153 {
    let m2743 = v0;
    match m2743 {
        Adt153::C0 => {
            Adt153::C2
        }
        Adt153::C1 => {
            Adt153::C1
        }
        Adt153::C2 => {
            Adt153::C0
        }
    }
}

pub fn f688(v0: i64, v1: i64) -> R<i64> {
    f115(f79(f690(v0)?, 0i64)?, v1)
}

pub fn f689(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt153> {
    let a2744 = v1;
    let a2745 = v3;
    if nat_lt(a2744, a2745) {
        Ok(f591(f714({
            let a2746 = v3;
            let a2747 = v1;
            nat_sub(a2746, a2747)
        }, v0)?, v2))
    } else {
        let a2748 = v3;
        let a2749 = v1;
        if nat_lt(a2748, a2749) {
            Ok(f591(v0, f714({
                let a2750 = v1;
                let a2751 = v3;
                nat_sub(a2750, a2751)
            }, v2)?))
        } else {
            Ok(f591(v0, v2))
        }
    }
}

pub fn f690(v0: i64) -> R<u64> {
    f715(20u64, v0, 0u64)
}

pub fn f691(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2752 = v0;
    match m2752 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f716(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f692(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2753 = v0;
    match m2753 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f693(p2756: List<Adt58>, p2757: List<Adt6>) -> List<Adt6> {
    let m2755 = (p2756, p2757);
    {
        let mut m2755 = m2755;
        loop {
            let step = {
                let (v0, v1) = m2755;
                let m2754 = v0;
                match m2754.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f717(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2758, _, _, _) => {
            h2758
        }
    })), {
        let a2764 = f430(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2759, _, _) => {
                h2759
            }
        });
        let a2765 = {
            let a2762 = f718(match v0.clone() {
                Adt2::C0(_, _, h2760, _) => {
                    h2760
                }
            });
            let a2763 = f719(match v0 {
                Adt2::C0(_, _, _, h2761) => {
                    h2761
                }
            });
            append_list(a2762, a2763)
        };
        append_list(a2764, a2765)
    }))
}

pub fn f695(p2771: List<(Str, Adt6)>, p2772: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2770 = (p2771, p2772);
    {
        let mut m2770 = m2770;
        loop {
            let step = {
                let (v0, v1) = m2770;
                let m2766 = v0;
                match m2766.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f720(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2768 = f721({
                            let (_, h2767) = v2;
                            h2767
                        });
                        match m2768 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2769, _) = v2;
                                    h2769
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f696(v0: Adt87) -> Adt6 {
    let m2773 = v0;
    match m2773 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h2774) => {
                    h2774
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f131(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f697(v0: u64, v1: List<u64>) -> List<u64> {
    let a2775 = v0;
    let a2776 = 0u64;
    if nat_eq(a2775, a2776) {
        v1
    } else {
        let a2777 = v0;
        let a2778 = 1u64;
        if nat_eq(a2777, a2778) {
            List::cons(194u64, v1)
        } else {
            let a2779 = v0;
            let a2780 = 2u64;
            if nat_eq(a2779, a2780) {
                List::cons(226u64, v1)
            } else {
                let a2781 = v0;
                let a2782 = 3u64;
                if nat_eq(a2781, a2782) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f698(v0: u64, v1: u64, v2: List<u64>) -> Adt156 {
    let a2783 = v0;
    let a2784 = 0u64;
    if nat_eq(a2783, a2784) {
        f722(v1, v2)
    } else {
        let a2785 = v0;
        let a2786 = 1u64;
        if nat_eq(a2785, a2786) {
            let a2787 = 128u64;
            let a2788 = v1;
            if if nat_le(a2787, a2788) {
                let a2789 = v1;
                let a2790 = 159u64;
                nat_le(a2789, a2790)
            } else {
                false
            } {
                Adt156::C0(0u64, f723(v2))
            } else {
                f722(v1, List::cons(194u64, v2))
            }
        } else {
            let a2791 = v0;
            let a2792 = 2u64;
            if nat_eq(a2791, a2792) {
                let a2793 = v1;
                let a2794 = 128u64;
                if nat_eq(a2793, a2794) {
                    Adt156::C0(3u64, v2)
                } else {
                    let a2795 = v1;
                    let a2796 = 129u64;
                    if nat_eq(a2795, a2796) {
                        Adt156::C0(4u64, v2)
                    } else {
                        f722(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2797 = v0;
                let a2798 = 3u64;
                if nat_eq(a2797, a2798) {
                    let a2799 = 170u64;
                    let a2800 = v1;
                    if if nat_le(a2799, a2800) {
                        let a2801 = v1;
                        let a2802 = 174u64;
                        nat_le(a2801, a2802)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f723(v2))
                    } else {
                        f722(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2803 = 166u64;
                    let a2804 = v1;
                    if if nat_le(a2803, a2804) {
                        let a2805 = v1;
                        let a2806 = 169u64;
                        nat_le(a2805, a2806)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f723(v2))
                    } else {
                        f722(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f699(v0: Adt140, v1: Adt140) -> bool {
    let a2807 = f474(v0);
    let a2808 = f474(v1);
    equal(a2807, a2808)
}

pub fn f700(v0: Adt140) -> bool {
    let m2809 = v0;
    match m2809 {
        Adt140::C0 => {
            false
        }
        Adt140::C1 => {
            false
        }
        Adt140::C2 => {
            true
        }
        Adt140::C3 => {
            false
        }
        Adt140::C4 => {
            true
        }
        Adt140::C5 => {
            false
        }
        Adt140::C6 => {
            false
        }
        Adt140::C7 => {
            true
        }
        Adt140::C8 => {
            false
        }
        Adt140::C9 => {
            false
        }
        Adt140::C10 => {
            false
        }
        Adt140::C11 => {
            false
        }
    }
}

pub fn f701(v0: Adt140) -> bool {
    let m2810 = v0;
    match m2810 {
        Adt140::C0 => {
            false
        }
        Adt140::C1 => {
            false
        }
        Adt140::C2 => {
            true
        }
        Adt140::C3 => {
            false
        }
        Adt140::C4 => {
            false
        }
        Adt140::C5 => {
            false
        }
        Adt140::C6 => {
            false
        }
        Adt140::C7 => {
            false
        }
        Adt140::C8 => {
            false
        }
        Adt140::C9 => {
            false
        }
        Adt140::C10 => {
            false
        }
        Adt140::C11 => {
            false
        }
    }
}

pub fn f702(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt128, Adt131> {
    f724(v0, v2.clone(), f725(match v1 {
        Adt17::C0(_, h2811, _, _, _, _) => {
            h2811
        }
    }, f443(v2)))
}

pub fn f703(v0: Adt15) -> Adt17 {
    let m2812 = v0;
    match m2812 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h2813) => {
                    h2813
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h2814, _) => {
                    h2814
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h2815, _, _, _, _) => {
                    h2815
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h2816, _) => {
                    h2816
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h2817, _, _, _) => {
                    h2817
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h2818, _) => {
                    h2818
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h2819, _) => {
                    h2819
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h2820, _, _, _, _) => {
                    h2820
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h2821, _, _, _) => {
                    h2821
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h2822, _) => {
                    h2822
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h2823, _, _) => {
                    h2823
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h2824, _) => {
                    h2824
                }
            }
        }
    }
}

pub fn f704(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt128, Adt131> {
    f726(v0, v2, f727(match v1 {
        Adt17::C0(_, h2825, _, _, _, _) => {
            h2825
        }
    }))
}

pub fn f705(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt128, Adt131> {
    f728(v0, v2, f727(match v1 {
        Adt17::C0(_, h2826, _, _, _, _) => {
            h2826
        }
    }))
}

pub fn f706(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt128 {
    Adt128::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h2827, _) => {
            h2827
        }
    }), {
        let m2829 = f616(v0, Adt33::C11);
        match m2829 {
            Adt128::C0(_, h2828, _) => {
                h2828
            }
        }
    }, f729(v1, match v2 {
        Adt50::C0(_, h2830) => {
            h2830
        }
    }))
}

pub fn f707(v0: Str, p2836: List<(Str, Adt6)>, p2837: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2835 = (p2836, p2837);
    {
        let mut m2835 = m2835;
        loop {
            let step = {
                let (v1, v2) = m2835;
                let m2831 = v1;
                match m2831.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Ok::<List<(Str, Str)>, Adt132>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2833 = f235({
                            let (_, h2832) = v3;
                            h2832
                        });
                        match m2833 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Err::<List<(Str, Str)>, Adt132>(Adt132::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>((v4, List::cons(({
                                    let (h2834, _) = v3;
                                    h2834
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2835 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(p2840: List<(Str, Str)>, p2841: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2839 = (p2840, p2841);
    {
        let mut m2839 = m2839;
        loop {
            let step = {
                let (v0, v1) = m2839;
                let m2838 = v0;
                match m2838.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(v0: Adt115, v1: Adt5) -> R<Option<Adt113>> {
    let m2843 = f730(match v0.clone() {
        Adt115::C0(_, h2842, _, _) => {
            h2842
        }
    }, v1);
    match m2843 {
        Adt135::C0 => {
            Ok(f731(v0))
        }
        Adt135::C1(v2) => {
            if f732(v2.clone()) {
                Ok(f731(v0))
            } else {
                f733(v0, v2)
            }
        }
    }
}

pub fn f710(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2844 = f182(v0, v1);
    match m2844 {
        Adt135::C0 => {
            Some(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2845 = f261(v2);
            match m2845 {
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

pub fn f711() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f712(v0: Str) -> Adt157 {
    let a2846 = v0.clone();
    let a2847 = Str::lit("debug");
    if equal(a2846, a2847) {
        Adt157::C1(Adt19::C0)
    } else {
        let a2848 = v0.clone();
        let a2849 = Str::lit("info");
        if equal(a2848, a2849) {
            Adt157::C1(Adt19::C1)
        } else {
            let a2850 = v0.clone();
            let a2851 = Str::lit("notice");
            if equal(a2850, a2851) {
                Adt157::C1(Adt19::C2)
            } else {
                let a2852 = v0.clone();
                let a2853 = Str::lit("warning");
                if equal(a2852, a2853) {
                    Adt157::C1(Adt19::C3)
                } else {
                    let a2854 = v0.clone();
                    let a2855 = Str::lit("error");
                    if equal(a2854, a2855) {
                        Adt157::C1(Adt19::C4)
                    } else {
                        let a2856 = v0.clone();
                        let a2857 = Str::lit("critical");
                        if equal(a2856, a2857) {
                            Adt157::C1(Adt19::C5)
                        } else {
                            let a2858 = v0.clone();
                            let a2859 = Str::lit("alert");
                            if equal(a2858, a2859) {
                                Adt157::C1(Adt19::C6)
                            } else {
                                let a2860 = v0;
                                let a2861 = Str::lit("emergency");
                                if equal(a2860, a2861) {
                                    Adt157::C1(Adt19::C7)
                                } else {
                                    Adt157::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f713() -> Str {
    Str::lit("progressToken")
}

pub fn f714(p2866: u64, p2867: i64) -> R<i64> {
    let m2865 = (p2866, p2867);
    {
        let mut m2865 = m2865;
        loop {
            let step = {
                let (v0, v1) = m2865;
                let m2862 = v0;
                if m2862 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2862 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2863 = v1;
                        let a2864 = 10i64;
                        int_mul(a2863, a2864)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f715(p2878: u64, p2879: i64, p2880: u64) -> R<u64> {
    let m2877 = (p2878, (p2879, p2880));
    {
        let mut m2877 = m2877;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2877;
                let m2868 = v0;
                if m2868 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2868 - 1;
                    let v4: u64 = {
                        let a2869 = v2;
                        let a2870 = 1u64;
                        nat_add(a2869, a2870)?
                    };
                    let m2873 = {
                        let a2871 = v1;
                        let a2872 = 10i64;
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
                        Err::<(u64, (i64, u64)), R<u64>>(Ok(v4))
                    } else {
                        Ok::<(u64, (i64, u64)), R<u64>>((v3, ({
                            let a2874 = v1;
                            let a2875 = 10i64;
                            let a2876 = 0i64;
                            int_quot(a2874, a2875, a2876)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f716(v0: List<Adt42>) -> List<Adt6> {
    f311(f734(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f717(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2889 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2881, _, _, _) => {
                h2881
            }
        })), List::<(Str, Adt6)>::nil());
        let a2890 = {
            let a2887 = f430(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2882, _, _) => {
                    h2882
                }
            });
            let a2888 = {
                let a2885 = f430(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h2883, _) => {
                        h2883
                    }
                });
                let a2886 = f435(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h2884) => {
                        h2884
                    }
                });
                append_list(a2885, a2886)
            };
            append_list(a2887, a2888)
        };
        append_list(a2889, a2890)
    })
}

pub fn f718(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2891 = v0;
    match m2891 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f379(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f719(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2892 = v0;
    match m2892 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f735(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f720(p2895: List<(Str, Adt65)>, p2896: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m2894 = (p2895, p2896);
    {
        let mut m2894 = m2894;
        loop {
            let step = {
                let (v0, v1) = m2894;
                let m2893 = v0;
                match m2893.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f721(v0: Adt6) -> Option<Adt65> {
    let m2897 = f261(v0);
    match m2897 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2898 = f661(Str::lit("method"), v1.clone());
            match m2898 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a2899 = v2.clone();
                    let a2900 = Str::lit("elicitation/create");
                    if equal(a2899, a2900) {
                        let m2901 = f606(Str::lit("params"), v1);
                        match m2901 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f736(v3)
                            }
                        }
                    } else {
                        let a2902 = v2;
                        let a2903 = Str::lit("roots/list");
                        if equal(a2902, a2903) {
                            f737(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f722(v0: u64, v1: List<u64>) -> Adt156 {
    let a2904 = v0;
    let a2905 = 32u64;
    if nat_lt(a2904, a2905) {
        if f738(v0) {
            Adt156::C0(0u64, List::cons(v0, v1))
        } else {
            Adt156::C0(0u64, f723(v1))
        }
    } else {
        let a2906 = v0;
        let a2907 = 127u64;
        if nat_eq(a2906, a2907) {
            Adt156::C0(0u64, f723(v1))
        } else {
            let a2908 = v0;
            let a2909 = 194u64;
            if nat_eq(a2908, a2909) {
                Adt156::C0(1u64, v1)
            } else {
                let a2910 = v0;
                let a2911 = 226u64;
                if nat_eq(a2910, a2911) {
                    Adt156::C0(2u64, v1)
                } else {
                    Adt156::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f723(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f724(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f739(f443(v1)))))
    }
}

pub fn f725(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a2912 = f740(v1.clone());
    if if if bool_not(a2912) {
        true
    } else {
        f477(match v0.clone() {
            Adt18::C0(_, _, _, h2913, _) => {
                h2913
            }
        })
    } {
        let a2914 = f741(v1.clone());
        if bool_not(a2914) {
            true
        } else {
            f477(match v0.clone() {
                Adt18::C0(_, _, h2915, _, _) => {
                    h2915
                }
            })
        }
    } else {
        false
    } {
        let a2916 = f742(v1);
        if bool_not(a2916) {
            true
        } else {
            f477(match v0 {
                Adt18::C0(_, h2917, _, _, _) => {
                    h2917
                }
            })
        }
    } else {
        false
    }
}

pub fn f726(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f743())))
    }
}

pub fn f727(v0: Adt18) -> bool {
    f478(match v0 {
        Adt18::C0(_, _, _, _, h2918) => {
            h2918
        }
    }, f26())
}

pub fn f728(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f743())))
    }
}

pub fn f729(v0: Adt17, v1: List<Adt51>) -> List<Adt130> {
    let m2920 = match v0 {
        Adt17::C0(_, _, _, _, h2919, _) => {
            h2919
        }
    };
    match m2920 {
        None => {
            List::<Adt130>::nil()
        }
        Some(v2) => {
            f744(f745(v2, v1, List::<Adt130>::nil()), List::<Adt130>::nil())
        }
    }
}

pub fn f730(p2926: List<Str>, p2927: Adt5) -> Adt135 {
    let m2925 = (p2926, p2927);
    {
        let mut m2925 = m2925;
        loop {
            let step = {
                let (v0, v1) = m2925;
                let m2921 = v0;
                match m2921.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m2922 = f182(v2, v1);
                        match m2922 {
                            Adt135::C0 => {
                                Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                            }
                            Adt135::C1(v4) => {
                                let m2923 = v3.clone();
                                match m2923.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2924 = f261(v4);
                                        match m2924 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt135>((v3, v7))
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
                Ok(next) => { m2925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f731(v0: Adt115) -> Option<Adt113> {
    let m2929 = match v0.clone() {
        Adt115::C0(_, _, _, h2928) => {
            h2928
        }
    };
    match m2929 {
        Adt116::C0 => {
            None::<Adt113>
        }
        Adt116::C1 => {
            Some(Adt113::C10(match v0 {
                Adt115::C0(h2930, _, _, _) => {
                    h2930
                }
            }))
        }
        Adt116::C2(_) => {
            None::<Adt113>
        }
    }
}

pub fn f732(v0: Adt6) -> bool {
    let m2931 = v0;
    match m2931 {
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

pub fn f733(v0: Adt115, v1: Adt6) -> R<Option<Adt113>> {
    let m2933 = match v0.clone() {
        Adt115::C0(_, _, _, h2932) => {
            h2932
        }
    };
    match m2933 {
        Adt116::C0 => {
            Ok(Some(Adt113::C8(match v0 {
                Adt115::C0(h2934, _, _, _) => {
                    h2934
                }
            })))
        }
        Adt116::C1 => {
            Ok(Some(Adt113::C10(match v0 {
                Adt115::C0(h2935, _, _, _) => {
                    h2935
                }
            })))
        }
        Adt116::C2(v2) => {
            if f746(match v0.clone() {
                Adt115::C0(_, _, h2936, _) => {
                    h2936
                }
            }, v2, v1)? {
                Ok(None::<Adt113>)
            } else {
                Ok(Some(Adt113::C9(match v0 {
                    Adt115::C0(h2937, _, _, _) => {
                        h2937
                    }
                })))
            }
        }
    }
}

pub fn f734(p2940: List<Adt42>, p2941: List<Adt6>) -> List<Adt6> {
    let m2939 = (p2940, p2941);
    {
        let mut m2939 = m2939;
        loop {
            let step = {
                let (v0, v1) = m2939;
                let m2938 = v0;
                match m2938.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f657(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2939 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f735(v0: Adt3) -> Str {
    let m2942 = v0;
    match m2942 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f736(v0: Adt5) -> Option<Adt65> {
    let m2943 = f661(Str::lit("message"), v0.clone());
    match m2943 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m2944 = f182(Str::lit("mode"), v0.clone());
            match m2944 {
                Adt135::C0 => {
                    let m2945 = f606(Str::lit("requestedSchema"), v0);
                    match m2945 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt135::C1(v3) => {
                    let m2946 = f235(v3);
                    match m2946 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a2947 = v4.clone();
                            let a2948 = Str::lit("url");
                            if equal(a2947, a2948) {
                                let m2949 = f661(Str::lit("url"), v0);
                                match m2949 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a2950 = v4;
                                let a2951 = Str::lit("form");
                                if equal(a2950, a2951) {
                                    let m2952 = f606(Str::lit("requestedSchema"), v0);
                                    match m2952 {
                                        None => {
                                            None::<Adt65>
                                        }
                                        Some(v6) => {
                                            Some(Adt65::C0(Adt66::C0(Adt67::C0(Some(Adt68::C0), v1, v6))))
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

pub fn f737(v0: Adt5) -> Option<Adt65> {
    let m2953 = f182(Str::lit("params"), v0);
    match m2953 {
        Adt135::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt135::C1(v1) => {
            let m2954 = f261(v1);
            match m2954 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    Some(Adt65::C2(Some(v2)))
                }
            }
        }
    }
}

pub fn f738(v0: u64) -> bool {
    let a2955 = v0;
    let a2956 = 9u64;
    if nat_eq(a2955, a2956) {
        true
    } else {
        let a2957 = v0;
        let a2958 = 10u64;
        if nat_eq(a2957, a2958) {
            true
        } else {
            let a2959 = v0;
            let a2960 = 13u64;
            nat_eq(a2959, a2960)
        }
    }
}

pub fn f739(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f742(v0.clone()) {
        Some(f559())
    } else {
        None::<Adt5>
    }, if f741(v0.clone()) {
        Some(f559())
    } else {
        None::<Adt5>
    }, if f740(v0) {
        Some(f559())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f740(p2965: List<(Str, Adt65)>) -> bool {
    let m2964 = p2965;
    {
        let mut m2964 = m2964;
        loop {
            let step = {
                let v0 = m2964;
                let m2961 = v0;
                match m2961.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2963 = {
                            let (_, h2962) = v1;
                            h2962
                        };
                        match m2963 {
                            Adt65::C0(_) => {
                                Err::<List<(Str, Adt65)>, bool>(true)
                            }
                            Adt65::C1(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                            Adt65::C2(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2964 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f741(p2970: List<(Str, Adt65)>) -> bool {
    let m2969 = p2970;
    {
        let mut m2969 = m2969;
        loop {
            let step = {
                let v0 = m2969;
                let m2966 = v0;
                match m2966.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2968 = {
                            let (_, h2967) = v1;
                            h2967
                        };
                        match m2968 {
                            Adt65::C0(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                            Adt65::C1(_) => {
                                Err::<List<(Str, Adt65)>, bool>(true)
                            }
                            Adt65::C2(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2969 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f742(p2975: List<(Str, Adt65)>) -> bool {
    let m2974 = p2975;
    {
        let mut m2974 = m2974;
        loop {
            let step = {
                let v0 = m2974;
                let m2971 = v0;
                match m2971.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2973 = {
                            let (_, h2972) = v1;
                            h2972
                        };
                        match m2973 {
                            Adt65::C0(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                            Adt65::C1(_) => {
                                Ok::<List<(Str, Adt65)>, bool>(v2)
                            }
                            Adt65::C2(_) => {
                                Err::<List<(Str, Adt65)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f743() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f26(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f744(p2978: List<Adt130>, p2979: List<Adt130>) -> List<Adt130> {
    let m2977 = (p2978, p2979);
    {
        let mut m2977 = m2977;
        loop {
            let step = {
                let (v0, v1) = m2977;
                let m2976 = v0;
                match m2976.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2977 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Adt20, p2982: List<Adt51>, p2983: List<Adt130>) -> List<Adt130> {
    let m2981 = (p2982, p2983);
    {
        let mut m2981 = m2981;
        loop {
            let step = {
                let (v1, v2) = m2981;
                let m2980 = v1;
                match m2980.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt130>), List<Adt130>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt130>), List<Adt130>>((v4, List::cons(Adt130::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2984 = v0;
    match m2984 {
        Adt13::C0 => {
            let m2985 = f235(v2);
            match m2985 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2986 = v1;
                    let a2987 = v3;
                    Ok(equal(a2986, a2987))
                }
            }
        }
        Adt13::C1 => {
            f747(v1, v2)
        }
        Adt13::C2 => {
            let m2988 = f260(v2);
            match m2988 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2989 = v1;
                        let a2990 = Str::lit("true");
                        Ok(equal(a2989, a2990))
                    } else {
                        let a2991 = v1;
                        let a2992 = Str::lit("false");
                        Ok(equal(a2991, a2992))
                    }
                }
            }
        }
    }
}

pub fn f747(v0: Str, v1: Adt6) -> R<bool> {
    let m2993 = f500(v1);
    match m2993 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2994 = f237(v2)?;
            match m2994 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2996 = f748(f85({
                        let a2995 = v0;
                        utf8_encode(a2995)
                    })?)?;
                    match m2996 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f395({
                                let a2997 = v3;
                                let a2998 = v4;
                                int_sub(a2997, a2998)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f748(v0: List<u64>) -> R<Option<i64>> {
    let m2999 = v0.clone();
    match m2999.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3000 = v1;
            let a3001 = 45u64;
            if nat_eq(a3000, a3001) {
                let m3002 = v2.clone();
                match m3002.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3003 = f749(v2, 0i64)?;
                        match m3003 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3004 = v5;
                                    int_neg(a3004)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f749(v0, 0i64)
            }
        }
    }
}

pub fn f749(p3010: List<u64>, p3011: i64) -> R<Option<i64>> {
    let m3009 = (p3010, p3011);
    {
        let mut m3009 = m3009;
        loop {
            let step = {
                let (v0, v1) = m3009;
                let m3005 = v0;
                match m3005.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3006 = f750(v2);
                        match m3006 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f115({
                                    let a3007 = v1;
                                    let a3008 = 10i64;
                                    int_mul(a3007, a3008)?
                                }, f310(v4))?))
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

pub fn f750(v0: u64) -> Option<u64> {
    let a3012 = v0;
    let a3013 = 48u64;
    if if nat_lt(a3012, a3013) {
        true
    } else {
        let a3014 = 57u64;
        let a3015 = v0;
        nat_lt(a3014, a3015)
    } {
        None::<u64>
    } else {
        Some({
            let a3016 = v0;
            let a3017 = 48u64;
            nat_sub(a3016, a3017)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
