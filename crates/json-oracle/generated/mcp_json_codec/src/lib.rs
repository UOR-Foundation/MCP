#![forbid(unsafe_code)]
use core::cmp::Ordering;
use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;
pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);
pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }
pub fn work() -> u64 { WORK.load(Memory::Relaxed) }


pub trait Same { fn same(&self, other: &Self) -> bool; }
pub trait Key { fn key(&self, other: &Self) -> Ordering; }
macro_rules! scalar {
    ($($t:ty),*) => { $(
        impl Same for $t { fn same(&self, other: &Self) -> bool { self == other } }
        impl Key for $t { fn key(&self, other: &Self) -> Ordering { self.cmp(other) } }
    )* };
}
scalar!(bool, u8, u16, u32, u64, i8, i16, i32, i64);
impl Same for Ordering { fn same(&self, other: &Self) -> bool { self == other } }
impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering {
        match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided }
    }
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
    C0,
    C1(bool),
    C2(Adt1),
    C3(Str),
    C4(List<Adt0>),
    C5(List<(Str, Adt0)>),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(i64, i64),
}

#[derive(Clone)]
pub enum Adt2 {
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
pub enum Adt3 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt4 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt5 {
    C0(List<u64>),
    C1(u64, Adt0),
    C2(u64, List<Adt0>),
    C3(u64, List<Adt0>),
    C4(u64, List<(Str, Adt0)>),
    C5(u64, List<(Str, Adt0)>),
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt7, List<Adt11>, u64),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt8, List<u64>, bool),
    C7(u64, u64),
    C8(Adt9),
    C9(Adt0),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt9 {
    C0(bool, Adt10, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt10 {
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
pub enum Adt11 {
    C0(List<Adt0>),
    C1(List<(Str, Adt0)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(Adt2),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Adt2),
    C1(List<Adt5>, List<u64>),
}

#[derive(Clone)]
pub enum Adt14 {
    C0,
    C1,
    C2(Adt0),
}

pub fn f0(v0: u64, v1: Bytes) -> R<List<u64>> {
    let a1 = v0;
    let a2 = 0u64;
    if nat_eq(a1, a2) {
        f1(f2(v1)?)
    } else {
        let a3 = v0;
        let a4 = 1u64;
        if nat_eq(a3, a4) {
            Ok(f3(f4(f5(v1)?)?))
        } else {
            let a5 = v0;
            let a6 = 2u64;
            if nat_eq(a5, a6) {
                f6(v1)
            } else {
                let a7 = v0;
                let a8 = 3u64;
                if nat_eq(a7, a8) {
                    f7(f2(v1)?)
                } else {
                    f1(f8(f5(v1)?)?)
                }
            }
        }
    }
}

pub fn f1(v0: Result<Adt0, Adt2>) -> R<List<u64>> {
    let m9 = v0;
    match m9 {
        Ok(v2) => {
            f10(v2)
        }
        Err(v1) => {
            Ok(f9(1u64, v1))
        }
    }
}

pub fn f2(v0: Bytes) -> R<Result<Adt0, Adt2>> {
    f11(f5(v0)?)
}

pub fn f3(v0: Adt3) -> List<u64> {
    let m10 = v0;
    match m10 {
        Adt3::C0 => {
            List::cons(0u64, List::<u64>::nil())
        }
        Adt3::C1(v1) => {
            List::cons(1u64, List::cons(0u64, List::cons(v1, List::<u64>::nil())))
        }
        Adt3::C2(v2) => {
            List::cons(1u64, List::cons(1u64, List::cons(v2, List::<u64>::nil())))
        }
    }
}

pub fn f4(v0: List<u64>) -> R<Adt3> {
    Ok(f12({
        let a11 = v0.clone();
        length_list(a11)
    }, f13(Adt4::C0, 0u64, 0u64, v0)?))
}

pub fn f5(v0: Bytes) -> R<List<u64>> {
    f14(v0.clone(), {
        let a12 = v0;
        length_bytes(a12)
    }, List::<u64>::nil())
}

pub fn f6(v0: Bytes) -> R<List<u64>> {
    let v1: u64 = f15(v0.clone(), 0u64)?;
    let v2: u64 = {
        let a15 = {
            let a13 = f15(v0.clone(), 1u64)?;
            let a14 = 256u64;
            nat_mul(a13, a14)?
        };
        let a16 = f15(v0, 2u64)?;
        nat_add(a15, a16)?
    };
    let m19 = f16({
        let a17 = v1;
        let a18 = 5u64;
        if nat_eq(a17, a18) {
            v2
        } else {
            f17()
        }
    }, f18(v1, v2)?)?;
    match m19 {
        Ok(v4) => {
            Ok(List::cons(0u64, v4))
        }
        Err(v3) => {
            Ok(f9(2u64, v3))
        }
    }
}

pub fn f7(v0: Result<Adt0, Adt2>) -> R<List<u64>> {
    let m20 = v0;
    match m20 {
        Ok(v2) => {
            let m21 = v2;
            match m21 {
                Adt0::C0 => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C1(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C2(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C3(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C4(_) => {
                    Ok(List::cons(4u64, List::<u64>::nil()))
                }
                Adt0::C5(v7) => {
                    let m22 = f19(Str::lit("a"), v7);
                    match m22 {
                        None => {
                            Ok(List::cons(3u64, List::<u64>::nil()))
                        }
                        Some(v8) => {
                            f10(v8)
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(f9(1u64, v1))
        }
    }
}

pub fn f8(v0: List<u64>) -> R<Result<Adt0, Adt2>> {
    f20(f21(), 0u64, v0)
}

pub fn f9(v0: u64, v1: Adt2) -> List<u64> {
    List::cons(v0, List::cons(f22(v1.clone()), List::cons(f23(v1), List::<u64>::nil())))
}

pub fn f10(v0: Adt0) -> R<List<u64>> {
    let m23 = f24(v0)?;
    match m23 {
        Ok(v2) => {
            Ok(List::cons(0u64, v2))
        }
        Err(v1) => {
            Ok(f9(2u64, v1))
        }
    }
}

pub fn f11(v0: List<u64>) -> R<Result<Adt0, Adt2>> {
    let m24 = f4(v0.clone())?;
    match m24 {
        Adt3::C0 => {
            f20(f21(), 0u64, v0)
        }
        Adt3::C1(v1) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C0(v1)))
        }
        Adt3::C2(v2) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C1(v2)))
        }
    }
}

pub fn f12(v0: u64, v1: u64) -> Adt3 {
    let a25 = v1;
    let a26 = 0u64;
    if nat_eq(a25, a26) {
        Adt3::C0
    } else {
        let a27 = v1;
        let a28 = 2u64;
        if nat_eq(a27, a28) {
            Adt3::C2(v0)
        } else {
            Adt3::C1({
                let a32 = v0;
                let a33 = {
                    let a29 = v1;
                    let a30 = 3u64;
                    let a31 = 0u64;
                    nat_quot(a29, a30, a31)
                };
                nat_sub(a32, a33)
            })
        }
    }
}

pub fn f13(p61: Adt4, p62: u64, p63: u64, p64: List<u64>) -> R<u64> {
    let m60 = (p61, (p62, (p63, p64)));
    {
        let mut m60 = m60;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m60;
                let m34 = v3;
                match m34.uncons() {
                    None => {
                        let m35 = v0;
                        match m35 {
                            Adt4::C0 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt4::C1 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt4::C2 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt4::C3 => {
                                Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m36 = v0;
                        match m36 {
                            Adt4::C0 => {
                                let a37 = v4;
                                let a38 = 128u64;
                                if nat_lt(a37, a38) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a39 = v4;
                                    let a40 = 194u64;
                                    if nat_lt(a39, a40) {
                                        let v6: u64 = f25({
                                            let a41 = List::cons(v4, v5);
                                            length_list(a41)
                                        })?;
                                        Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a42 = v4;
                                        let a43 = 224u64;
                                        if nat_lt(a42, a43) {
                                            Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a44 = v4;
                                            let a45 = 224u64;
                                            if nat_eq(a44, a45) {
                                                Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a46 = v4;
                                                let a47 = 237u64;
                                                if nat_eq(a46, a47) {
                                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a48 = v4;
                                                    let a49 = 240u64;
                                                    if nat_lt(a48, a49) {
                                                        Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a50 = v4;
                                                        let a51 = 240u64;
                                                        if nat_eq(a50, a51) {
                                                            Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a52 = v4;
                                                            let a53 = 244u64;
                                                            if nat_eq(a52, a53) {
                                                                Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a54 = v4;
                                                                let a55 = 244u64;
                                                                if nat_lt(a54, a55) {
                                                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f25({
                                                                        let a56 = List::cons(v4, v5);
                                                                        length_list(a56)
                                                                    })?;
                                                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt4::C1 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f25({
                                        let a57 = List::cons(v4, v5);
                                        length_list(a57)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt4::C2 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f25({
                                        let a58 = List::cons(v4, v5);
                                        length_list(a58)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt4::C3 => {
                                if f26(v4, v1, v2) {
                                    Ok::<(Adt4, (u64, (u64, List<u64>))), R<u64>>((Adt4::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f25({
                                        let a59 = List::cons(v4, v5);
                                        length_list(a59)
                                    })?;
                                    Err::<(Adt4, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m60 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f14(v0: Bytes, p67: u64, p68: List<u64>) -> R<List<u64>> {
    let m66 = (p67, p68);
    {
        let mut m66 = m66;
        loop {
            let step = {
                let (v1, v2) = m66;
                let m65 = v1;
                if m65 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m65 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f15(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m66 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: u64) -> R<u64> {
    let m71 = {
        let a69 = v0;
        let a70 = v1;
        index_bytes(a69, a70)
    };
    match m71 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a116 = {
                let a92 = {
                    let a80 = {
                        let a74 = {
                            let a72 = v3;
                            let a73 = 128u8;
                            fixed_u8::bit_and(a72, a73)
                        };
                        let a75 = 128u8;
                        if equal(a74, a75) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a81 = {
                        let a78 = {
                            let a76 = v3;
                            let a77 = 64u8;
                            fixed_u8::bit_and(a76, a77)
                        };
                        let a79 = 64u8;
                        if equal(a78, a79) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a80, a81)?
                };
                let a93 = {
                    let a90 = {
                        let a84 = {
                            let a82 = v3;
                            let a83 = 32u8;
                            fixed_u8::bit_and(a82, a83)
                        };
                        let a85 = 32u8;
                        if equal(a84, a85) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a91 = {
                        let a88 = {
                            let a86 = v3;
                            let a87 = 16u8;
                            fixed_u8::bit_and(a86, a87)
                        };
                        let a89 = 16u8;
                        if equal(a88, a89) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a90, a91)?
                };
                nat_add(a92, a93)?
            };
            let a117 = {
                let a114 = {
                    let a102 = {
                        let a96 = {
                            let a94 = v3;
                            let a95 = 8u8;
                            fixed_u8::bit_and(a94, a95)
                        };
                        let a97 = 8u8;
                        if equal(a96, a97) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a103 = {
                        let a100 = {
                            let a98 = v3;
                            let a99 = 4u8;
                            fixed_u8::bit_and(a98, a99)
                        };
                        let a101 = 4u8;
                        if equal(a100, a101) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a102, a103)?
                };
                let a115 = {
                    let a112 = {
                        let a106 = {
                            let a104 = v3;
                            let a105 = 2u8;
                            fixed_u8::bit_and(a104, a105)
                        };
                        let a107 = 2u8;
                        if equal(a106, a107) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a113 = {
                        let a110 = {
                            let a108 = v3;
                            let a109 = 1u8;
                            fixed_u8::bit_and(a108, a109)
                        };
                        let a111 = 1u8;
                        if equal(a110, a111) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a112, a113)?
                };
                nat_add(a114, a115)?
            };
            nat_add(a116, a117)
        }
    }
}

pub fn f16(v0: u64, v1: Adt0) -> R<Result<List<u64>, Adt2>> {
    let m118 = f27(v0, List::cons(Adt5::C1(0u64, v1), List::<Adt5>::nil()), List::<u64>::nil())?;
    match m118 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt2>(f28(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt2>(v3))
        }
    }
}

pub fn f17() -> u64 {
    67108864u64
}

pub fn f18(v0: u64, v1: u64) -> R<Adt0> {
    let a119 = v0;
    let a120 = 0u64;
    if nat_eq(a119, a120) {
        Ok(f29(v1, Adt0::C0))
    } else {
        let a121 = v0;
        let a122 = 1u64;
        if nat_eq(a121, a122) {
            Ok(f30(f31({
                let a123 = v1;
                let a124 = 1u64;
                nat_sub(a123, a124)
            }, 1i64)?, 0i64))
        } else {
            let a125 = v0;
            let a126 = 2u64;
            if nat_eq(a125, a126) {
                Ok(f30(1i64, f32(v1, 0i64)?))
            } else {
                let a127 = v0;
                let a128 = 3u64;
                if nat_eq(a127, a128) {
                    Ok(f30(1i64, {
                        let a129 = f32(v1, 0i64)?;
                        int_neg(a129)?
                    }))
                } else {
                    let a130 = v0;
                    let a131 = 4u64;
                    if nat_eq(a130, a131) {
                        Ok(Adt0::C3(f33()))
                    } else {
                        Ok(f29(5u64, Adt0::C0))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Str, p138: List<(Str, Adt0)>) -> Option<Adt0> {
    let m137 = p138;
    {
        let mut m137 = m137;
        loop {
            let step = {
                let v1 = m137;
                let m132 = v1;
                match m132.uncons() {
                    None => {
                        Err::<List<(Str, Adt0)>, Option<Adt0>>(None::<Adt0>)
                    }
                    Some((v2, v3)) => {
                        let a134 = {
                            let (h133, _) = v2;
                            h133
                        };
                        let a135 = v0.clone();
                        if equal(a134, a135) {
                            Err::<List<(Str, Adt0)>, Option<Adt0>>(Some({
                                let (_, h136) = v2;
                                h136
                            }))
                        } else {
                            Ok::<List<(Str, Adt0)>, Option<Adt0>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f20(p145: Adt6, p146: u64, p147: List<u64>) -> R<Result<Adt0, Adt2>> {
    let m144 = (p145, (p146, p147));
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v0, (v1, v2)) = m144;
                let m139 = v2;
                match m139.uncons() {
                    None => {
                        let m140 = f34(v0, v1)?;
                        match m140 {
                            Ok(v3) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Ok::<Adt0, Adt2>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Err::<Adt0, Adt2>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m141 = f35(v0, v1, v5)?;
                        match m141 {
                            Adt12::C0(v7) => {
                                Err::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>(Ok(Err::<Adt0, Adt2>(v7)))
                            }
                            Adt12::C1(v8) => {
                                Ok::<(Adt6, (u64, List<u64>)), R<Result<Adt0, Adt2>>>((v8, ({
                                    let a142 = v1;
                                    let a143 = 1u64;
                                    nat_add(a142, a143)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f21() -> Adt6 {
    Adt6::C0(Adt7::C0, List::<Adt11>::nil(), 0u64)
}

pub fn f22(v0: Adt2) -> u64 {
    let m148 = v0;
    match m148 {
        Adt2::C0(_) => {
            0u64
        }
        Adt2::C1(_) => {
            1u64
        }
        Adt2::C2(_) => {
            2u64
        }
        Adt2::C3 => {
            3u64
        }
        Adt2::C4(_) => {
            4u64
        }
        Adt2::C5(_) => {
            5u64
        }
        Adt2::C6(_) => {
            6u64
        }
        Adt2::C7(_) => {
            7u64
        }
        Adt2::C8(_) => {
            8u64
        }
        Adt2::C9(_) => {
            9u64
        }
        Adt2::C10(_) => {
            10u64
        }
        Adt2::C11 => {
            11u64
        }
        Adt2::C12 => {
            12u64
        }
        Adt2::C13 => {
            13u64
        }
        Adt2::C14 => {
            14u64
        }
    }
}

pub fn f23(v0: Adt2) -> u64 {
    let m149 = v0;
    match m149 {
        Adt2::C0(v1) => {
            v1
        }
        Adt2::C1(v2) => {
            v2
        }
        Adt2::C2(v3) => {
            v3
        }
        Adt2::C3 => {
            0u64
        }
        Adt2::C4(v4) => {
            v4
        }
        Adt2::C5(v5) => {
            v5
        }
        Adt2::C6(v6) => {
            v6
        }
        Adt2::C7(v7) => {
            v7
        }
        Adt2::C8(v8) => {
            v8
        }
        Adt2::C9(v9) => {
            v9
        }
        Adt2::C10(v10) => {
            v10
        }
        Adt2::C11 => {
            0u64
        }
        Adt2::C12 => {
            0u64
        }
        Adt2::C13 => {
            0u64
        }
        Adt2::C14 => {
            0u64
        }
    }
}

pub fn f24(v0: Adt0) -> R<Result<List<u64>, Adt2>> {
    f16(f17(), v0)
}

pub fn f25(v0: u64) -> R<u64> {
    let a152 = {
        let a150 = v0;
        let a151 = 3u64;
        nat_mul(a150, a151)?
    };
    let a153 = 1u64;
    nat_add(a152, a153)
}

pub fn f26(v0: u64, v1: u64, v2: u64) -> bool {
    let a154 = v1;
    let a155 = v0;
    if nat_le(a154, a155) {
        let a156 = v0;
        let a157 = v2;
        nat_le(a156, a157)
    } else {
        false
    }
}

pub fn f27(p162: u64, p163: List<Adt5>, p164: List<u64>) -> R<Result<List<u64>, Adt2>> {
    let m161 = (p162, (p163, p164));
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v0, (v1, v2)) = m161;
                let m158 = v0;
                if m158 == 0 {
                    Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Err::<List<u64>, Adt2>(Adt2::C13)))
                } else {
                    let v3 = m158 - 1;
                    let m159 = v1;
                    match m159.uncons() {
                        None => {
                            Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Ok::<List<u64>, Adt2>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m160 = f36(v4, v5, v2)?;
                            match m160 {
                                Adt13::C0(v6) => {
                                    Err::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>(Ok(Err::<List<u64>, Adt2>(v6)))
                                }
                                Adt13::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt5>, List<u64>)), R<Result<List<u64>, Adt2>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28(p167: List<u64>, p168: List<u64>) -> List<u64> {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v0, v1) = m166;
                let m165 = v0;
                match m165.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29(p171: u64, p172: Adt0) -> Adt0 {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v0, v1) = m170;
                let m169 = v0;
                if m169 == 0 {
                    Err::<(u64, Adt0), Adt0>(v1)
                } else {
                    let v2 = m169 - 1;
                    Ok::<(u64, Adt0), Adt0>((v2, Adt0::C4(List::cons(v1, List::<Adt0>::nil()))))
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30(v0: i64, v1: i64) -> Adt0 {
    Adt0::C2(Adt1::C0(v0, v1))
}

pub fn f31(p177: u64, p178: i64) -> R<i64> {
    let m176 = (p177, p178);
    {
        let mut m176 = m176;
        loop {
            let step = {
                let (v0, v1) = m176;
                let m173 = v0;
                if m173 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m173 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a174 = v1;
                        let a175 = 10i64;
                        int_mul(a174, a175)?
                    }))
                }
            };
            match step {
                Ok(next) => { m176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(p181: u64, p182: i64) -> R<i64> {
    let m180 = (p181, p182);
    {
        let mut m180 = m180;
        loop {
            let step = {
                let (v0, v1) = m180;
                let m179 = v0;
                if m179 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m179 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f37(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f33() -> Str {
    let m184 = {
        let a183 = f38(f39(), f40(128u64, List::<u64>::nil()));
        utf8_decode(a183)
    };
    match m184 {
        None => {
            Str::lit("")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f34(v0: Adt6, v1: u64) -> R<Result<Adt0, Adt2>> {
    let m186 = match v0.clone() {
        Adt6::C0(h185, _, _) => {
            h185
        }
    };
    match m186 {
        Adt7::C0 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C1 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C2 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C3 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C4 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C5 => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C6(_, _, _) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C7(_, _) => {
            Ok(Err::<Adt0, Adt2>(Adt2::C3))
        }
        Adt7::C8(v3) => {
            let m187 = f41(v3)?;
            match m187 {
                Adt14::C0 => {
                    Ok(Err::<Adt0, Adt2>(Adt2::C7(v1)))
                }
                Adt14::C1 => {
                    Ok(Err::<Adt0, Adt2>(Adt2::C8(v1)))
                }
                Adt14::C2(v4) => {
                    let m192 = {
                        let m191 = f42(v4, match v0.clone() {
                            Adt6::C0(_, h189, _) => {
                                h189
                            }
                        }, match v0 {
                            Adt6::C0(_, _, h190) => {
                                h190
                            }
                        });
                        match m191 {
                            Adt6::C0(h188, _, _) => {
                                h188
                            }
                        }
                    };
                    match m192 {
                        Adt7::C0 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C1 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C2 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C3 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C4 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C5 => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C6(_, _, _) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C7(_, _) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C8(_) => {
                            Ok(Err::<Adt0, Adt2>(Adt2::C3))
                        }
                        Adt7::C9(v5) => {
                            Ok(Ok::<Adt0, Adt2>(v5))
                        }
                    }
                }
            }
        }
        Adt7::C9(v2) => {
            Ok(Ok::<Adt0, Adt2>(v2))
        }
    }
}

pub fn f35(v0: Adt6, v1: u64, v2: u64) -> R<Adt12> {
    let m194 = match v0.clone() {
        Adt6::C0(h193, _, _) => {
            h193
        }
    };
    match m194 {
        Adt7::C0 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                f44(v0, v1, v2)
            }
        }
        Adt7::C1 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a195 = v2;
                let a196 = 93u64;
                if nat_eq(a195, a196) {
                    Ok(f45(v0, v1))
                } else {
                    f44(v0, v1, v2)
                }
            }
        }
        Adt7::C2 => {
            Ok(f46(v0, v1, v2))
        }
        Adt7::C3 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a197 = v2;
                let a198 = 125u64;
                if nat_eq(a197, a198) {
                    Ok(f47(v0, v1))
                } else {
                    let a199 = v2;
                    let a200 = 34u64;
                    if nat_eq(a199, a200) {
                        Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f49(Adt2::C2(v1)))
                    }
                }
            }
        }
        Adt7::C4 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a201 = v2;
                let a202 = 34u64;
                if nat_eq(a201, a202) {
                    Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f49(Adt2::C2(v1)))
                }
            }
        }
        Adt7::C5 => {
            if f43(v2) {
                Ok(Adt12::C1(v0))
            } else {
                let a203 = v2;
                let a204 = 58u64;
                if nat_eq(a203, a204) {
                    Ok(f48(v0, Adt7::C0))
                } else {
                    Ok(f49(Adt2::C2(v1)))
                }
            }
        }
        Adt7::C6(v3, v4, v5) => {
            f50(v0, v1, v2, v3, v4, v5)
        }
        Adt7::C7(v6, v7) => {
            f51(v0, v1, v2, v6, v7)
        }
        Adt7::C8(v8) => {
            f52(v0, v1, v2, v8)
        }
        Adt7::C9(_) => {
            Ok(f46(v0, v1, v2))
        }
    }
}

pub fn f36(v0: Adt5, v1: List<Adt5>, v2: List<u64>) -> R<Adt13> {
    let m205 = v0;
    match m205 {
        Adt5::C0(v3) => {
            Ok(Adt13::C1(v1, f28(v3, v2)))
        }
        Adt5::C1(v4, v5) => {
            f53(v4, v5, v1, v2)
        }
        Adt5::C2(v6, v7) => {
            let m206 = v7;
            match m206.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C1(v6, v8), List::cons(Adt5::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt5::C3(v10, v11) => {
            let m207 = v11.clone();
            match m207.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt5::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt5::C4(v14, v15) => {
            let m208 = v15;
            match m208.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0({
                        let a210 = f54({
                            let (h209, _) = v16;
                            h209
                        })?;
                        let a211 = List::cons(58u64, List::<u64>::nil());
                        append_list(a210, a211)
                    }), List::cons(Adt5::C1(v14, {
                        let (_, h212) = v16;
                        h212
                    }), List::cons(Adt5::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt5::C5(v18, v19) => {
            let m213 = v19.clone();
            match m213.uncons() {
                None => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt13::C1(List::cons(Adt5::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt5::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f37(v0: i64, v1: i64) -> R<i64> {
    let a215 = v0;
    let a216 = {
        let a214 = v1;
        int_neg(a214)?
    };
    int_sub(a215, a216)
}

pub fn f38(p221: Bytes, p222: List<u64>) -> Bytes {
    let m220 = (p221, p222);
    {
        let mut m220 = m220;
        loop {
            let step = {
                let (v0, v1) = m220;
                let m217 = v1;
                match m217.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let a218 = v0;
                            let a219 = f55(v2);
                            append_bytes(a218, a219)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39() -> Bytes {
    Bytes::lit(&[])
}

pub fn f40(p225: u64, p226: List<u64>) -> List<u64> {
    let m224 = (p225, p226);
    {
        let mut m224 = m224;
        loop {
            let step = {
                let (v0, v1) = m224;
                let m223 = v0;
                if m223 == 0 {
                    Err::<(u64, List<u64>), List<u64>>(v1)
                } else {
                    let v2 = m223 - 1;
                    Ok::<(u64, List<u64>), List<u64>>((v2, List::cons(v2, v1)))
                }
            };
            match step {
                Ok(next) => { m224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41(v0: Adt9) -> R<Adt14> {
    if f56(match v0.clone() {
        Adt9::C0(_, h227, _, _, _, _, _) => {
            h227
        }
    }) {
        let v1: i64 = {
            let a233 = if match v0.clone() {
                Adt9::C0(_, _, _, _, _, h228, _) => {
                    h228
                }
            } {
                let a230 = match v0.clone() {
                    Adt9::C0(_, _, _, _, _, _, h229) => {
                        h229
                    }
                };
                int_neg(a230)?
            } else {
                match v0.clone() {
                    Adt9::C0(_, _, _, _, _, _, h231) => {
                        h231
                    }
                }
            };
            let a234 = match v0.clone() {
                Adt9::C0(_, _, _, _, h232, _, _) => {
                    h232
                }
            };
            int_sub(a233, a234)?
        };
        let m237 = {
            let a235 = v1;
            let a236 = -9999i64;
            compare(a235, a236)
        };
        if if match m237 {
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
            let m240 = {
                let a238 = 9999i64;
                let a239 = v1;
                compare(a238, a239)
            };
            match m240 {
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
            Ok(Adt14::C1)
        } else {
            Ok(Adt14::C2(Adt0::C2(Adt1::C0(if match v0.clone() {
                Adt9::C0(h241, _, _, _, _, _, _) => {
                    h241
                }
            } {
                let a243 = match v0 {
                    Adt9::C0(_, _, h242, _, _, _, _) => {
                        h242
                    }
                };
                int_neg(a243)?
            } else {
                match v0 {
                    Adt9::C0(_, _, h244, _, _, _, _) => {
                        h244
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt14::C0)
    }
}

pub fn f42(v0: Adt0, v1: List<Adt11>, v2: u64) -> Adt6 {
    let m245 = v1.clone();
    match m245.uncons() {
        None => {
            Adt6::C0(Adt7::C9(v0), List::<Adt11>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m246 = v3;
            match m246 {
                Adt11::C0(v5) => {
                    Adt6::C0(Adt7::C2, List::cons(Adt11::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt11::C1(v6, v7) => {
                    let m247 = v7;
                    match m247 {
                        None => {
                            Adt6::C0(Adt7::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt6::C0(Adt7::C2, List::cons(Adt11::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f43(v0: u64) -> bool {
    let a248 = v0;
    let a249 = 32u64;
    if nat_eq(a248, a249) {
        true
    } else {
        let a250 = v0;
        let a251 = 10u64;
        if nat_eq(a250, a251) {
            true
        } else {
            let a252 = v0;
            let a253 = 13u64;
            if nat_eq(a252, a253) {
                true
            } else {
                let a254 = v0;
                let a255 = 9u64;
                nat_eq(a254, a255)
            }
        }
    }
}

pub fn f44(v0: Adt6, v1: u64, v2: u64) -> R<Adt12> {
    let a256 = v2;
    let a257 = 34u64;
    if nat_eq(a256, a257) {
        Ok(f48(v0, Adt7::C6(Adt8::C0, List::<u64>::nil(), false)))
    } else {
        let a258 = v2;
        let a259 = 123u64;
        if nat_eq(a258, a259) {
            f57(v0, Adt11::C1(List::<(Str, Adt0)>::nil(), None::<Str>), Adt7::C3, v1)
        } else {
            let a260 = v2;
            let a261 = 91u64;
            if nat_eq(a260, a261) {
                f57(v0, Adt11::C0(List::<Adt0>::nil()), Adt7::C1, v1)
            } else {
                let a262 = v2;
                let a263 = 116u64;
                if nat_eq(a262, a263) {
                    Ok(f48(v0, Adt7::C7(0u64, 1u64)))
                } else {
                    let a264 = v2;
                    let a265 = 102u64;
                    if nat_eq(a264, a265) {
                        Ok(f48(v0, Adt7::C7(1u64, 1u64)))
                    } else {
                        let a266 = v2;
                        let a267 = 110u64;
                        if nat_eq(a266, a267) {
                            Ok(f48(v0, Adt7::C7(2u64, 1u64)))
                        } else {
                            let a268 = v2;
                            let a269 = 45u64;
                            if nat_eq(a268, a269) {
                                Ok(f48(v0, f58(true, Adt10::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a270 = v2;
                                let a271 = 48u64;
                                if nat_eq(a270, a271) {
                                    Ok(f48(v0, f58(false, Adt10::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f26(v2, 49u64, 57u64) {
                                        Ok(f48(v0, f58(false, Adt10::C2, f59({
                                            let a272 = v2;
                                            let a273 = 48u64;
                                            nat_sub(a272, a273)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f49(Adt2::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f45(v0: Adt6, v1: u64) -> Adt12 {
    let m275 = match v0.clone() {
        Adt6::C0(_, h274, _) => {
            h274
        }
    };
    match m275.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, v3)) => {
            let m276 = v2;
            match m276 {
                Adt11::C0(v4) => {
                    Adt12::C1(f42(Adt0::C4(f60(v4, List::<Adt0>::nil())), v3, {
                        let a278 = match v0 {
                            Adt6::C0(_, _, h277) => {
                                h277
                            }
                        };
                        let a279 = 1u64;
                        nat_sub(a278, a279)
                    }))
                }
                Adt11::C1(_, _) => {
                    f49(Adt2::C2(v1))
                }
            }
        }
    }
}

pub fn f46(v0: Adt6, v1: u64, v2: u64) -> Adt12 {
    let m281 = match v0.clone() {
        Adt6::C0(h280, _, _) => {
            h280
        }
    };
    match m281 {
        Adt7::C0 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C1 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C2 => {
            if f43(v2) {
                Adt12::C1(v0)
            } else {
                let a282 = v2;
                let a283 = 44u64;
                if nat_eq(a282, a283) {
                    f61(v0, v1)
                } else {
                    let a284 = v2;
                    let a285 = 93u64;
                    if nat_eq(a284, a285) {
                        f45(v0, v1)
                    } else {
                        let a286 = v2;
                        let a287 = 125u64;
                        if nat_eq(a286, a287) {
                            f47(v0, v1)
                        } else {
                            f49(Adt2::C2(v1))
                        }
                    }
                }
            }
        }
        Adt7::C3 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C4 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C5 => {
            f49(Adt2::C2(v1))
        }
        Adt7::C6(_, _, _) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C7(_, _) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C8(_) => {
            f49(Adt2::C2(v1))
        }
        Adt7::C9(_) => {
            if f43(v2) {
                Adt12::C1(v0)
            } else {
                f49(Adt2::C10(v1))
            }
        }
    }
}

pub fn f47(v0: Adt6, v1: u64) -> Adt12 {
    let m289 = match v0.clone() {
        Adt6::C0(_, h288, _) => {
            h288
        }
    };
    match m289.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, v3)) => {
            let m290 = v2;
            match m290 {
                Adt11::C0(_) => {
                    f49(Adt2::C2(v1))
                }
                Adt11::C1(v5, _) => {
                    Adt12::C1(f42(Adt0::C5(f62(v5, List::<(Str, Adt0)>::nil())), v3, {
                        let a292 = match v0 {
                            Adt6::C0(_, _, h291) => {
                                h291
                            }
                        };
                        let a293 = 1u64;
                        nat_sub(a292, a293)
                    }))
                }
            }
        }
    }
}

pub fn f48(v0: Adt6, v1: Adt7) -> Adt12 {
    Adt12::C1(f63(v0, v1))
}

pub fn f49(v0: Adt2) -> Adt12 {
    Adt12::C0(v0)
}

pub fn f50(v0: Adt6, v1: u64, v2: u64, v3: Adt8, v4: List<u64>, v5: bool) -> R<Adt12> {
    let m294 = v3;
    match m294 {
        Adt8::C0 => {
            let a295 = v2;
            let a296 = 34u64;
            if nat_eq(a295, a296) {
                Ok(f64(v0, v1, v4, v5))
            } else {
                let a297 = v2;
                let a298 = 92u64;
                if nat_eq(a297, a298) {
                    Ok(f48(v0, Adt7::C6(Adt8::C1, v4, v5)))
                } else {
                    let a299 = v2;
                    let a300 = 32u64;
                    if nat_lt(a299, a300) {
                        Ok(f49(Adt2::C4(v1)))
                    } else {
                        Ok(f48(v0, Adt7::C6(Adt8::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt8::C1 => {
            let a301 = v2;
            let a302 = 117u64;
            if nat_eq(a301, a302) {
                Ok(f48(v0, Adt7::C6(Adt8::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a303 = f65(v2);
                let a304 = 0u64;
                if nat_eq(a303, a304) {
                    Ok(f49(Adt2::C5(v1)))
                } else {
                    Ok(f48(v0, Adt7::C6(Adt8::C0, List::cons(f65(v2), v4), v5)))
                }
            }
        }
        Adt8::C2(v6, v7, v8) => {
            let a305 = 15u64;
            let a306 = f66(v2);
            if nat_lt(a305, a306) {
                Ok(f49(Adt2::C5(v1)))
            } else {
                let a309 = {
                    let a307 = v6;
                    let a308 = 1u64;
                    nat_add(a307, a308)?
                };
                let a310 = 4u64;
                if nat_eq(a309, a310) {
                    f67(v0, v1, {
                        let a313 = {
                            let a311 = v7;
                            let a312 = 16u64;
                            nat_mul(a311, a312)?
                        };
                        let a314 = f66(v2);
                        nat_add(a313, a314)?
                    }, v8, v4, v5)
                } else {
                    Ok(f48(v0, Adt7::C6(Adt8::C2({
                        let a315 = v6;
                        let a316 = 1u64;
                        nat_add(a315, a316)?
                    }, {
                        let a319 = {
                            let a317 = v7;
                            let a318 = 16u64;
                            nat_mul(a317, a318)?
                        };
                        let a320 = f66(v2);
                        nat_add(a319, a320)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt8::C3(v9) => {
            let a321 = v2;
            let a322 = 92u64;
            if nat_eq(a321, a322) {
                Ok(f48(v0, Adt7::C6(Adt8::C4(v9), v4, v5)))
            } else {
                Ok(f49(Adt2::C6(v1)))
            }
        }
        Adt8::C4(v10) => {
            let a323 = v2;
            let a324 = 117u64;
            if nat_eq(a323, a324) {
                Ok(f48(v0, Adt7::C6(Adt8::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f49(Adt2::C6(v1)))
            }
        }
    }
}

pub fn f51(v0: Adt6, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt12> {
    let a325 = v2;
    let a326 = f68(v3, v4);
    if nat_eq(a325, a326) {
        let a330 = {
            let a327 = v4;
            let a328 = 1u64;
            nat_add(a327, a328)?
        };
        let a331 = {
            let a329 = f69(v3);
            length_list(a329)
        };
        if nat_eq(a330, a331) {
            Ok(f70(v0, f71(v3)))
        } else {
            Ok(f48(v0, Adt7::C7(v3, {
                let a332 = v4;
                let a333 = 1u64;
                nat_add(a332, a333)?
            })))
        }
    } else {
        Ok(f49(Adt2::C2(v1)))
    }
}

pub fn f52(v0: Adt6, v1: u64, v2: u64, v3: Adt9) -> R<Adt12> {
    let m335 = match v3.clone() {
        Adt9::C0(_, h334, _, _, _, _, _) => {
            h334
        }
    };
    match m335 {
        Adt10::C0 => {
            let a336 = v2;
            let a337 = 48u64;
            if nat_eq(a336, a337) {
                Ok(f72(v0, v3, Adt10::C1))
            } else {
                if f26(v2, 49u64, 57u64) {
                    f73(v0, v1, v2, v3, false)
                } else {
                    Ok(f49(Adt2::C7(v1)))
                }
            }
        }
        Adt10::C1 => {
            let a338 = v2;
            let a339 = 46u64;
            if nat_eq(a338, a339) {
                Ok(f72(v0, v3, Adt10::C3))
            } else {
                if f74(v2) {
                    Ok(f72(v0, v3, Adt10::C5))
                } else {
                    if f75(v2) {
                        Ok(f49(Adt2::C7(v1)))
                    } else {
                        f76(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt10::C2 => {
            if f75(v2) {
                f73(v0, v1, v2, v3, false)
            } else {
                let a340 = v2;
                let a341 = 46u64;
                if nat_eq(a340, a341) {
                    Ok(f72(v0, v3, Adt10::C3))
                } else {
                    if f74(v2) {
                        Ok(f72(v0, v3, Adt10::C5))
                    } else {
                        f76(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt10::C3 => {
            if f75(v2) {
                f73(v0, v1, v2, v3, true)
            } else {
                Ok(f49(Adt2::C7(v1)))
            }
        }
        Adt10::C4 => {
            if f75(v2) {
                f73(v0, v1, v2, v3, true)
            } else {
                if f74(v2) {
                    Ok(f72(v0, v3, Adt10::C5))
                } else {
                    f76(v0, v1, v2, v3)
                }
            }
        }
        Adt10::C5 => {
            let a342 = v2;
            let a343 = 43u64;
            if nat_eq(a342, a343) {
                Ok(f48(v0, f58(match v3.clone() {
                    Adt9::C0(h344, _, _, _, _, _, _) => {
                        h344
                    }
                }, Adt10::C6, match v3.clone() {
                    Adt9::C0(_, _, h345, _, _, _, _) => {
                        h345
                    }
                }, match v3.clone() {
                    Adt9::C0(_, _, _, h346, _, _, _) => {
                        h346
                    }
                }, match v3 {
                    Adt9::C0(_, _, _, _, h347, _, _) => {
                        h347
                    }
                }, false, 0i64)))
            } else {
                let a348 = v2;
                let a349 = 45u64;
                if nat_eq(a348, a349) {
                    Ok(f48(v0, f58(match v3.clone() {
                        Adt9::C0(h350, _, _, _, _, _, _) => {
                            h350
                        }
                    }, Adt10::C6, match v3.clone() {
                        Adt9::C0(_, _, h351, _, _, _, _) => {
                            h351
                        }
                    }, match v3.clone() {
                        Adt9::C0(_, _, _, h352, _, _, _) => {
                            h352
                        }
                    }, match v3 {
                        Adt9::C0(_, _, _, _, h353, _, _) => {
                            h353
                        }
                    }, true, 0i64)))
                } else {
                    if f75(v2) {
                        f77(v0, v1, v2, Adt9::C0(match v3.clone() {
                            Adt9::C0(h354, _, _, _, _, _, _) => {
                                h354
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, h355, _, _, _, _, _) => {
                                h355
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, _, h356, _, _, _, _) => {
                                h356
                            }
                        }, match v3.clone() {
                            Adt9::C0(_, _, _, h357, _, _, _) => {
                                h357
                            }
                        }, match v3 {
                            Adt9::C0(_, _, _, _, h358, _, _) => {
                                h358
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f49(Adt2::C7(v1)))
                    }
                }
            }
        }
        Adt10::C6 => {
            if f75(v2) {
                f77(v0, v1, v2, v3.clone(), match v3 {
                    Adt9::C0(_, _, _, _, _, h359, _) => {
                        h359
                    }
                })
            } else {
                Ok(f49(Adt2::C7(v1)))
            }
        }
        Adt10::C7 => {
            if f75(v2) {
                f77(v0, v1, v2, v3.clone(), match v3 {
                    Adt9::C0(_, _, _, _, _, h360, _) => {
                        h360
                    }
                })
            } else {
                f76(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt0, v2: List<Adt5>, v3: List<u64>) -> R<Adt13> {
    let m361 = v1;
    match m361 {
        Adt0::C0 => {
            Ok(Adt13::C1(v2, f28(f78(2u64), v3)))
        }
        Adt0::C1(v4) => {
            Ok(Adt13::C1(v2, f28(if v4 {
                f78(0u64)
            } else {
                f78(1u64)
            }, v3)))
        }
        Adt0::C2(v5) => {
            if f79(v5.clone())? {
                Ok(Adt13::C1(v2, f28(f80(v5)?, v3)))
            } else {
                Ok(Adt13::C0(Adt2::C11))
            }
        }
        Adt0::C3(v6) => {
            Ok(Adt13::C1(v2, f28(f54(v6)?, v3)))
        }
        Adt0::C4(v7) => {
            let a362 = v0;
            let a363 = f81();
            if nat_lt(a362, a363) {
                Ok(Adt13::C1(f82(91u64, Adt5::C2({
                    let a364 = v0;
                    let a365 = 1u64;
                    nat_add(a364, a365)?
                }, v7), v2), v3))
            } else {
                Ok(Adt13::C0(Adt2::C12))
            }
        }
        Adt0::C5(v8) => {
            let a366 = v0;
            let a367 = f81();
            if nat_lt(a366, a367) {
                Ok(Adt13::C1(f82(123u64, Adt5::C4({
                    let a368 = v0;
                    let a369 = 1u64;
                    nat_add(a368, a369)?
                }, v8), v2), v3))
            } else {
                Ok(Adt13::C0(Adt2::C12))
            }
        }
    }
}

pub fn f54(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a371 = f28(f83(f5({
            let a370 = v0;
            utf8_encode(a370)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a372 = List::cons(34u64, List::<u64>::nil());
        append_list(a371, a372)
    }))
}

pub fn f55(v0: u64) -> Bytes {
    let m376 = {
        let a373 = f84();
        let a374 = v0;
        let a375 = 1u64;
        slice_bytes(a373, a374, a375)
    };
    match m376 {
        None => {
            f39()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f56(v0: Adt10) -> bool {
    let m377 = v0;
    match m377 {
        Adt10::C0 => {
            false
        }
        Adt10::C1 => {
            true
        }
        Adt10::C2 => {
            true
        }
        Adt10::C3 => {
            false
        }
        Adt10::C4 => {
            true
        }
        Adt10::C5 => {
            false
        }
        Adt10::C6 => {
            false
        }
        Adt10::C7 => {
            true
        }
    }
}

pub fn f57(v0: Adt6, v1: Adt11, v2: Adt7, v3: u64) -> R<Adt12> {
    let a379 = match v0.clone() {
        Adt6::C0(_, _, h378) => {
            h378
        }
    };
    let a380 = f81();
    if nat_lt(a379, a380) {
        Ok(Adt12::C1(Adt6::C0(v2, List::cons(v1, match v0.clone() {
            Adt6::C0(_, h381, _) => {
                h381
            }
        }), {
            let a383 = match v0 {
                Adt6::C0(_, _, h382) => {
                    h382
                }
            };
            let a384 = 1u64;
            nat_add(a383, a384)?
        })))
    } else {
        Ok(f49(Adt2::C9(v3)))
    }
}

pub fn f58(v0: bool, v1: Adt10, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt7 {
    Adt7::C8(Adt9::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f59(v0: u64) -> i64 {
    let a385 = v0;
    let a386 = 0u64;
    if nat_eq(a385, a386) {
        0i64
    } else {
        let a387 = v0;
        let a388 = 1u64;
        if nat_eq(a387, a388) {
            1i64
        } else {
            let a389 = v0;
            let a390 = 2u64;
            if nat_eq(a389, a390) {
                2i64
            } else {
                let a391 = v0;
                let a392 = 3u64;
                if nat_eq(a391, a392) {
                    3i64
                } else {
                    let a393 = v0;
                    let a394 = 4u64;
                    if nat_eq(a393, a394) {
                        4i64
                    } else {
                        let a395 = v0;
                        let a396 = 5u64;
                        if nat_eq(a395, a396) {
                            5i64
                        } else {
                            let a397 = v0;
                            let a398 = 6u64;
                            if nat_eq(a397, a398) {
                                6i64
                            } else {
                                let a399 = v0;
                                let a400 = 7u64;
                                if nat_eq(a399, a400) {
                                    7i64
                                } else {
                                    let a401 = v0;
                                    let a402 = 8u64;
                                    if nat_eq(a401, a402) {
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

pub fn f60(p405: List<Adt0>, p406: List<Adt0>) -> List<Adt0> {
    let m404 = (p405, p406);
    {
        let mut m404 = m404;
        loop {
            let step = {
                let (v0, v1) = m404;
                let m403 = v0;
                match m403.uncons() {
                    None => {
                        Err::<(List<Adt0>, List<Adt0>), List<Adt0>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt0>, List<Adt0>), List<Adt0>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Adt6, v1: u64) -> Adt12 {
    let m408 = match v0.clone() {
        Adt6::C0(_, h407, _) => {
            h407
        }
    };
    match m408.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v2, _)) => {
            let m409 = v2;
            match m409 {
                Adt11::C0(_) => {
                    f48(v0, Adt7::C0)
                }
                Adt11::C1(_, _) => {
                    f48(v0, Adt7::C4)
                }
            }
        }
    }
}

pub fn f62(p412: List<(Str, Adt0)>, p413: List<(Str, Adt0)>) -> List<(Str, Adt0)> {
    let m411 = (p412, p413);
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v0, v1) = m411;
                let m410 = v0;
                match m410.uncons() {
                    None => {
                        Err::<(List<(Str, Adt0)>, List<(Str, Adt0)>), List<(Str, Adt0)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt0)>, List<(Str, Adt0)>), List<(Str, Adt0)>>((v3, List::cons(v2, v1)))
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

pub fn f63(v0: Adt6, v1: Adt7) -> Adt6 {
    Adt6::C0(v1, match v0.clone() {
        Adt6::C0(_, h414, _) => {
            h414
        }
    }, match v0 {
        Adt6::C0(_, _, h415) => {
            h415
        }
    })
}

pub fn f64(v0: Adt6, v1: u64, v2: List<u64>, v3: bool) -> Adt12 {
    let m417 = {
        let a416 = f38(f39(), f28(v2, List::<u64>::nil()));
        utf8_decode(a416)
    };
    match m417 {
        None => {
            f49(Adt2::C14)
        }
        Some(v4) => {
            if v3 {
                f85(v0, v1, v4)
            } else {
                f70(v0, Adt0::C3(v4))
            }
        }
    }
}

pub fn f65(v0: u64) -> u64 {
    let a418 = v0;
    let a419 = 34u64;
    if nat_eq(a418, a419) {
        34u64
    } else {
        let a420 = v0;
        let a421 = 92u64;
        if nat_eq(a420, a421) {
            92u64
        } else {
            let a422 = v0;
            let a423 = 47u64;
            if nat_eq(a422, a423) {
                47u64
            } else {
                let a424 = v0;
                let a425 = 98u64;
                if nat_eq(a424, a425) {
                    8u64
                } else {
                    let a426 = v0;
                    let a427 = 102u64;
                    if nat_eq(a426, a427) {
                        12u64
                    } else {
                        let a428 = v0;
                        let a429 = 110u64;
                        if nat_eq(a428, a429) {
                            10u64
                        } else {
                            let a430 = v0;
                            let a431 = 114u64;
                            if nat_eq(a430, a431) {
                                13u64
                            } else {
                                let a432 = v0;
                                let a433 = 116u64;
                                if nat_eq(a432, a433) {
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

pub fn f66(v0: u64) -> u64 {
    let a434 = v0;
    let a435 = 48u64;
    if nat_lt(a434, a435) {
        16u64
    } else {
        let a436 = v0;
        let a437 = 58u64;
        if nat_lt(a436, a437) {
            let a438 = v0;
            let a439 = 48u64;
            nat_sub(a438, a439)
        } else {
            let a440 = v0;
            let a441 = 65u64;
            if nat_lt(a440, a441) {
                16u64
            } else {
                let a442 = v0;
                let a443 = 71u64;
                if nat_lt(a442, a443) {
                    let a444 = v0;
                    let a445 = 55u64;
                    nat_sub(a444, a445)
                } else {
                    let a446 = v0;
                    let a447 = 97u64;
                    if nat_lt(a446, a447) {
                        16u64
                    } else {
                        let a448 = v0;
                        let a449 = 103u64;
                        if nat_lt(a448, a449) {
                            let a450 = v0;
                            let a451 = 87u64;
                            nat_sub(a450, a451)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f67(v0: Adt6, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt12> {
    let a452 = v3;
    let a453 = 0u64;
    if nat_eq(a452, a453) {
        if f26(v2, 55296u64, 56319u64) {
            Ok(f48(v0, Adt7::C6(Adt8::C3(v2), v4, v5)))
        } else {
            if f26(v2, 56320u64, 57343u64) {
                Ok(f49(Adt2::C6(v1)))
            } else {
                Ok(f48(v0, Adt7::C6(Adt8::C0, f86(v2, v4)?, v5)))
            }
        }
    } else {
        if f26(v2, 56320u64, 57343u64) {
            Ok(f48(v0, Adt7::C6(Adt8::C0, f86({
                let a462 = 65536u64;
                let a463 = {
                    let a460 = {
                        let a456 = {
                            let a454 = v3;
                            let a455 = 55296u64;
                            nat_sub(a454, a455)
                        };
                        let a457 = 1024u64;
                        nat_mul(a456, a457)?
                    };
                    let a461 = {
                        let a458 = v2;
                        let a459 = 56320u64;
                        nat_sub(a458, a459)
                    };
                    nat_add(a460, a461)?
                };
                nat_add(a462, a463)?
            }, v4)?, v5)))
        } else {
            Ok(f49(Adt2::C6(v1)))
        }
    }
}

pub fn f68(v0: u64, v1: u64) -> u64 {
    let m466 = {
        let a464 = f69(v0);
        let a465 = v1;
        index_list(a464, a465)
    };
    match m466 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f69(v0: u64) -> List<u64> {
    let a467 = v0;
    let a468 = 0u64;
    if nat_eq(a467, a468) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a469 = v0;
        let a470 = 1u64;
        if nat_eq(a469, a470) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f70(v0: Adt6, v1: Adt0) -> Adt12 {
    Adt12::C1(f42(v1, match v0.clone() {
        Adt6::C0(_, h471, _) => {
            h471
        }
    }, match v0 {
        Adt6::C0(_, _, h472) => {
            h472
        }
    }))
}

pub fn f71(v0: u64) -> Adt0 {
    let a473 = v0;
    let a474 = 0u64;
    if nat_eq(a473, a474) {
        Adt0::C1(true)
    } else {
        let a475 = v0;
        let a476 = 1u64;
        if nat_eq(a475, a476) {
            Adt0::C1(false)
        } else {
            Adt0::C0
        }
    }
}

pub fn f72(v0: Adt6, v1: Adt9, v2: Adt10) -> Adt12 {
    f48(v0, Adt7::C8(Adt9::C0(match v1.clone() {
        Adt9::C0(h477, _, _, _, _, _, _) => {
            h477
        }
    }, v2, match v1.clone() {
        Adt9::C0(_, _, h478, _, _, _, _) => {
            h478
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, h479, _, _, _) => {
            h479
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, _, h480, _, _) => {
            h480
        }
    }, match v1.clone() {
        Adt9::C0(_, _, _, _, _, h481, _) => {
            h481
        }
    }, match v1 {
        Adt9::C0(_, _, _, _, _, _, h482) => {
            h482
        }
    })))
}

pub fn f73(v0: Adt6, v1: u64, v2: u64, v3: Adt9, v4: bool) -> R<Adt12> {
    let v5: u64 = {
        let a484 = match v3.clone() {
            Adt9::C0(_, _, _, h483, _, _, _) => {
                h483
            }
        };
        let a485 = 0u64;
        if if nat_eq(a484, a485) {
            let a486 = v2;
            let a487 = 48u64;
            nat_eq(a486, a487)
        } else {
            false
        } {
            0u64
        } else {
            let a489 = match v3.clone() {
                Adt9::C0(_, _, _, h488, _, _, _) => {
                    h488
                }
            };
            let a490 = 1u64;
            nat_add(a489, a490)?
        }
    };
    let v6: i64 = if v4 {
        f37(match v3.clone() {
            Adt9::C0(_, _, _, _, h491, _, _) => {
                h491
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt9::C0(_, _, _, _, h492, _, _) => {
                h492
            }
        }
    };
    let a493 = f87();
    let a494 = v5;
    if nat_lt(a493, a494) {
        Ok(f49(Adt2::C8(v1)))
    } else {
        let m497 = {
            let a495 = 99999i64;
            let a496 = v6;
            compare(a495, a496)
        };
        if match m497 {
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
            Ok(f49(Adt2::C8(v1)))
        } else {
            Ok(f48(v0, f58(match v3.clone() {
                Adt9::C0(h498, _, _, _, _, _, _) => {
                    h498
                }
            }, if v4 {
                Adt10::C4
            } else {
                Adt10::C2
            }, f37({
                let a500 = match v3.clone() {
                    Adt9::C0(_, _, h499, _, _, _, _) => {
                        h499
                    }
                };
                let a501 = 10i64;
                int_mul(a500, a501)?
            }, f59({
                let a502 = v2;
                let a503 = 48u64;
                nat_sub(a502, a503)
            }))?, v5, v6, match v3.clone() {
                Adt9::C0(_, _, _, _, _, h504, _) => {
                    h504
                }
            }, match v3 {
                Adt9::C0(_, _, _, _, _, _, h505) => {
                    h505
                }
            })))
        }
    }
}

pub fn f74(v0: u64) -> bool {
    let a506 = v0;
    let a507 = 101u64;
    if nat_eq(a506, a507) {
        true
    } else {
        let a508 = v0;
        let a509 = 69u64;
        nat_eq(a508, a509)
    }
}

pub fn f75(v0: u64) -> bool {
    f26(v0, 48u64, 57u64)
}

pub fn f76(v0: Adt6, v1: u64, v2: u64, v3: Adt9) -> R<Adt12> {
    let m510 = f41(v3)?;
    match m510 {
        Adt14::C0 => {
            Ok(f49(Adt2::C7(v1)))
        }
        Adt14::C1 => {
            Ok(f49(Adt2::C8(v1)))
        }
        Adt14::C2(v4) => {
            Ok(f46(f42(v4, match v0.clone() {
                Adt6::C0(_, h511, _) => {
                    h511
                }
            }, match v0 {
                Adt6::C0(_, _, h512) => {
                    h512
                }
            }), v1, v2))
        }
    }
}

pub fn f77(v0: Adt6, v1: u64, v2: u64, v3: Adt9, v4: bool) -> R<Adt12> {
    let v5: i64 = f37({
        let a514 = match v3.clone() {
            Adt9::C0(_, _, _, _, _, _, h513) => {
                h513
            }
        };
        let a515 = 10i64;
        int_mul(a514, a515)?
    }, f59({
        let a516 = v2;
        let a517 = 48u64;
        nat_sub(a516, a517)
    }))?;
    let m520 = {
        let a518 = 99999i64;
        let a519 = v5;
        compare(a518, a519)
    };
    if match m520 {
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
        Ok(f49(Adt2::C8(v1)))
    } else {
        Ok(f48(v0, f58(match v3.clone() {
            Adt9::C0(h521, _, _, _, _, _, _) => {
                h521
            }
        }, Adt10::C7, match v3.clone() {
            Adt9::C0(_, _, h522, _, _, _, _) => {
                h522
            }
        }, match v3.clone() {
            Adt9::C0(_, _, _, h523, _, _, _) => {
                h523
            }
        }, match v3 {
            Adt9::C0(_, _, _, _, h524, _, _) => {
                h524
            }
        }, v4, v5)))
    }
}

pub fn f78(v0: u64) -> List<u64> {
    f69(v0)
}

pub fn f79(v0: Adt1) -> R<bool> {
    let a542 = {
        let m528 = {
            let a526 = f88();
            let a527 = match v0.clone() {
                Adt1::C0(h525, _) => {
                    h525
                }
            };
            compare(a526, a527)
        };
        if match m528 {
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
            let m533 = {
                let a531 = match v0.clone() {
                    Adt1::C0(h529, _) => {
                        h529
                    }
                };
                let a532 = {
                    let a530 = f88();
                    int_neg(a530)?
                };
                compare(a531, a532)
            };
            if match m533 {
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
                let m537 = {
                    let a535 = 9999i64;
                    let a536 = match v0.clone() {
                        Adt1::C0(_, h534) => {
                            h534
                        }
                    };
                    compare(a535, a536)
                };
                if match m537 {
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
                    let m541 = {
                        let a539 = match v0 {
                            Adt1::C0(_, h538) => {
                                h538
                            }
                        };
                        let a540 = -9999i64;
                        compare(a539, a540)
                    };
                    match m541 {
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
    Ok(bool_not(a542))
}

pub fn f80(v0: Adt1) -> R<List<u64>> {
    let m546 = {
        let a544 = match v0.clone() {
            Adt1::C0(_, h543) => {
                h543
            }
        };
        let a545 = 0i64;
        compare(a544, a545)
    };
    if if match m546 {
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
        let m550 = {
            let a548 = 0i64;
            let a549 = match v0.clone() {
                Adt1::C0(_, h547) => {
                    h547
                }
            };
            compare(a548, a549)
        };
        match m550 {
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
        let a553 = f89(match v0.clone() {
            Adt1::C0(h551, _) => {
                h551
            }
        })?;
        let a554 = List::cons(101u64, f89(match v0 {
            Adt1::C0(_, h552) => {
                h552
            }
        })?);
        Ok(append_list(a553, a554))
    } else {
        f89(match v0 {
            Adt1::C0(h555, _) => {
                h555
            }
        })
    }
}

pub fn f81() -> u64 {
    512u64
}

pub fn f82(v0: u64, v1: Adt5, v2: List<Adt5>) -> List<Adt5> {
    List::cons(Adt5::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f83(p558: List<u64>, p559: List<u64>) -> R<List<u64>> {
    let m557 = (p558, p559);
    {
        let mut m557 = m557;
        loop {
            let step = {
                let (v0, v1) = m557;
                let m556 = v0;
                match m556.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f28(f90(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m557 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f85(v0: Adt6, v1: u64, v2: Str) -> Adt12 {
    let m561 = match v0.clone() {
        Adt6::C0(_, h560, _) => {
            h560
        }
    };
    match m561.uncons() {
        None => {
            f49(Adt2::C2(v1))
        }
        Some((v3, v4)) => {
            let m562 = v3;
            match m562 {
                Adt11::C0(_) => {
                    f49(Adt2::C2(v1))
                }
                Adt11::C1(v6, _) => {
                    Adt12::C1(Adt6::C0(Adt7::C5, List::cons(Adt11::C1(v6, Some(v2)), v4), match v0 {
                        Adt6::C0(_, _, h563) => {
                            h563
                        }
                    }))
                }
            }
        }
    }
}

pub fn f86(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a564 = v0;
    let a565 = 128u64;
    if nat_lt(a564, a565) {
        Ok(List::cons(v0, v1))
    } else {
        let a566 = v0;
        let a567 = 2048u64;
        if nat_lt(a566, a567) {
            Ok(List::cons({
                let a568 = 128u64;
                let a569 = f91(v0, 64u64);
                nat_add(a568, a569)?
            }, List::cons({
                let a570 = 192u64;
                let a571 = f92(v0, 64u64);
                nat_add(a570, a571)?
            }, v1)))
        } else {
            let a572 = v0;
            let a573 = 65536u64;
            if nat_lt(a572, a573) {
                Ok(List::cons({
                    let a574 = 128u64;
                    let a575 = f91(v0, 64u64);
                    nat_add(a574, a575)?
                }, List::cons({
                    let a576 = 128u64;
                    let a577 = f91(f92(v0, 64u64), 64u64);
                    nat_add(a576, a577)?
                }, List::cons({
                    let a578 = 224u64;
                    let a579 = f92(v0, 4096u64);
                    nat_add(a578, a579)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a580 = 128u64;
                    let a581 = f91(v0, 64u64);
                    nat_add(a580, a581)?
                }, List::cons({
                    let a582 = 128u64;
                    let a583 = f91(f92(v0, 64u64), 64u64);
                    nat_add(a582, a583)?
                }, List::cons({
                    let a584 = 128u64;
                    let a585 = f91(f92(v0, 4096u64), 64u64);
                    nat_add(a584, a585)?
                }, List::cons({
                    let a586 = 240u64;
                    let a587 = f92(v0, 262144u64);
                    nat_add(a586, a587)?
                }, v1)))))
            }
        }
    }
}

pub fn f87() -> u64 {
    18u64
}

pub fn f88() -> i64 {
    999999999999999999i64
}

pub fn f89(v0: i64) -> R<List<u64>> {
    let m590 = {
        let a588 = v0;
        let a589 = 0i64;
        compare(a588, a589)
    };
    if match m590 {
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
        Ok(List::cons(45u64, f93(20u64, {
            let a591 = v0;
            int_neg(a591)?
        }, List::<u64>::nil())?))
    } else {
        f93(20u64, v0, List::<u64>::nil())
    }
}

pub fn f90(v0: u64) -> R<List<u64>> {
    let a592 = v0;
    let a593 = 34u64;
    if nat_eq(a592, a593) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a594 = v0;
        let a595 = 92u64;
        if nat_eq(a594, a595) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a596 = v0;
            let a597 = 8u64;
            if nat_eq(a596, a597) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a598 = v0;
                let a599 = 9u64;
                if nat_eq(a598, a599) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a600 = v0;
                    let a601 = 10u64;
                    if nat_eq(a600, a601) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a602 = v0;
                        let a603 = 12u64;
                        if nat_eq(a602, a603) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a604 = v0;
                            let a605 = 13u64;
                            if nat_eq(a604, a605) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a606 = v0;
                                let a607 = 32u64;
                                if nat_lt(a606, a607) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f94(f92(v0, 16u64))?, List::cons(f94(f91(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f91(v0: u64, v1: u64) -> u64 {
    let a608 = v0;
    let a609 = v1;
    let a610 = 0u64;
    nat_rem(a608, a609, a610)
}

pub fn f92(v0: u64, v1: u64) -> u64 {
    let a611 = v0;
    let a612 = v1;
    let a613 = 0u64;
    nat_quot(a611, a612, a613)
}

pub fn f93(p625: u64, p626: i64, p627: List<u64>) -> R<List<u64>> {
    let m624 = (p625, (p626, p627));
    {
        let mut m624 = m624;
        loop {
            let step = {
                let (v0, (v1, v2)) = m624;
                let m614 = v0;
                if m614 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m614 - 1;
                    let v4: i64 = {
                        let a615 = v1;
                        let a616 = 10i64;
                        let a617 = 0i64;
                        int_quot(a615, a616, a617)?
                    };
                    let v5: i64 = {
                        let a618 = v1;
                        let a619 = 10i64;
                        let a620 = 0i64;
                        int_rem(a618, a619, a620)
                    };
                    let m623 = {
                        let a621 = v4;
                        let a622 = 1i64;
                        compare(a621, a622)
                    };
                    if match m623 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f95(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f95(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a628 = v0;
    let a629 = 10u64;
    if nat_lt(a628, a629) {
        let a630 = 48u64;
        let a631 = v0;
        nat_add(a630, a631)
    } else {
        let a632 = 87u64;
        let a633 = v0;
        nat_add(a632, a633)
    }
}

pub fn f95(v0: i64) -> u64 {
    let m636 = {
        let a634 = v0;
        let a635 = 1i64;
        compare(a634, a635)
    };
    if match m636 {
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
        let m639 = {
            let a637 = v0;
            let a638 = 2i64;
            compare(a637, a638)
        };
        if match m639 {
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
            let m642 = {
                let a640 = v0;
                let a641 = 3i64;
                compare(a640, a641)
            };
            if match m642 {
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
                let m645 = {
                    let a643 = v0;
                    let a644 = 4i64;
                    compare(a643, a644)
                };
                if match m645 {
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
                    let m648 = {
                        let a646 = v0;
                        let a647 = 5i64;
                        compare(a646, a647)
                    };
                    if match m648 {
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
                        let m651 = {
                            let a649 = v0;
                            let a650 = 6i64;
                            compare(a649, a650)
                        };
                        if match m651 {
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
                            let m654 = {
                                let a652 = v0;
                                let a653 = 7i64;
                                compare(a652, a653)
                            };
                            if match m654 {
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
                                let m657 = {
                                    let a655 = v0;
                                    let a656 = 8i64;
                                    compare(a655, a656)
                                };
                                if match m657 {
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
                                    let m660 = {
                                        let a658 = v0;
                                        let a659 = 9i64;
                                        compare(a658, a659)
                                    };
                                    if match m660 {
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

pub fn run(p0: u64, p1: Bytes) -> R<List<u64>> {
    f0(p0, p1)
}
