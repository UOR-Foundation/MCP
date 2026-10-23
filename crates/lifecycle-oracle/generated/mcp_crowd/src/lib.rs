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
    C0(Adt15),
    C1(Adt20),
    C2(Adt21),
    C3(Adt20),
    C4(Adt22),
    C5(Adt20),
    C6(Adt20),
    C7(Adt23),
    C8(Adt24),
    C9(Adt29),
    C10(Adt30),
    C11(Adt31),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt16),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Str, Adt17, Option<Adt5>, Option<Adt18>, Option<Adt19>, Adt5),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt18 {
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
pub enum Adt19 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Adt16, Option<Str>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Adt16, Str, Option<Adt5>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt16, Str, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt16, Str, Option<List<(Str, Str)>>, Option<Adt5>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Adt16, Adt25, Adt27, Option<Adt28>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Adt26),
    C1(Str),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Str, Option<Str>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Option<List<(Str, Str)>>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt16, Str),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt16, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt16, Str),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt33),
    C1(Adt34),
    C2(Adt37),
    C3(Adt49),
    C4(Adt51),
    C5(Adt52),
    C6(Adt53),
    C7(Adt55),
    C8(Adt58),
    C9(Adt60),
    C10(Adt62),
    C11,
    C12(Adt81),
    C13(Adt82),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt10, List<Str>, Adt4, Option<Str>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Adt10, Option<Str>, List<Adt35>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Adt5, Option<Adt5>, Option<Adt36>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Option<Str>, Option<bool>, Option<bool>, Option<bool>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(List<Adt38>, Option<Adt6>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(Adt39),
    C1(Adt42),
    C2(Adt43),
    C3(Adt44),
    C4(Adt45),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Str, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Option<List<Adt41>>, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt41 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt42 {
    C0(Str, Str, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt43 {
    C0(Str, Str, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt44 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt40>, Option<u64>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt45 {
    C0(Adt46, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt46 {
    C0(Adt47),
    C1(Adt48),
}

#[derive(Clone)]
pub enum Adt47 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt48 {
    C0(Str, Option<Str>, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt49 {
    C0(Adt37, List<Adt50>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt7, Option<Adt7>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Adt10, Option<Str>, List<Adt44>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Adt10, List<Adt46>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt10, Option<Str>, List<Adt54>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Adt10, Option<Str>, List<Adt56>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Option<List<Adt57>>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Option<Str>, List<Adt59>),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(Adt41, Adt38),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Adt61),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Adt63),
    C1(Adt63, List<(Str, Adt64)>),
    C2(Adt63, Adt5),
    C3(Adt63, Adt80),
    C4(Adt63),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Str, Option<Str>, Str, Str, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Adt65),
    C1(Adt69),
    C2(Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt65 {
    C0(Adt66),
    C1(Adt68),
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Option<Adt67>, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt67 {
    C0,
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt69 {
    C0(List<Adt70>, Option<Adt75>, Option<Str>, Option<Adt77>, Option<Adt7>, u64, Option<List<Str>>, Option<Adt5>, Option<List<Adt35>>, Option<Adt78>),
}

#[derive(Clone)]
pub enum Adt70 {
    C0(Adt41, Adt71, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt71 {
    C0(Adt72),
    C1(List<Adt72>),
}

#[derive(Clone)]
pub enum Adt72 {
    C0(Adt39),
    C1(Adt42),
    C2(Adt43),
    C3(Adt73),
    C4(Adt74),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Str, Str, Adt5, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Str, List<Adt38>, Option<Adt6>, Option<bool>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0(Option<List<Adt76>>, Option<Adt7>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt78 {
    C0(Option<Adt79>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt80 {
    C0(i64, Str, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt81 {
    C0(List<(Str, Adt64)>),
    C1(Str),
    C2(List<(Str, Adt64)>, Str),
}

#[derive(Clone)]
pub enum Adt82 {
    C0(Adt83, Adt63),
}

#[derive(Clone)]
pub enum Adt83 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt84 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(u64),
    C8(Adt17),
    C9(Str),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(List<u64>, List<u64>, List<Adt86>, Bytes, u64),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt88 {
    C0(u64, List<Adt86>, List<List<u64>>, List<u64>),
}

#[derive(Clone)]
pub enum Adt89 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt90 {
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
pub enum Adt91 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt92 {
    C0,
    C1(Option<Adt91>, Adt93),
}

#[derive(Clone)]
pub enum Adt93 {
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
pub enum Adt94 {
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
pub enum Adt95 {
    C0(Adt91, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt97>, List<Adt99>),
}

#[derive(Clone)]
pub enum Adt97 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt98),
    C7(Adt98),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt98 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Str, List<Str>, Adt13, Adt100),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt102 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt103 {
    C0(Adt104, List<Adt108>, u64),
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt105, List<u64>, bool),
    C7(u64, u64),
    C8(Adt106),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt105 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(bool, Adt107, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt107 {
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
pub enum Adt108 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1(List<Adt110>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1(List<Adt110>, Str),
}

#[derive(Clone)]
pub enum Adt112 {
    C0(Adt6, Adt6, List<Adt110>),
    C1(List<Adt6>, Adt6, List<Adt110>),
    C2(List<Adt6>, Adt6, List<Adt110>, u64),
    C3(List<Adt110>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt110>),
    C5(Adt113, Adt6, Adt6, List<Adt110>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt114 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Adt91, Result<Adt116, Adt119>),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Adt32, Adt117, List<Adt118>),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt118 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt119 {
    C0(Adt120),
    C1(Str, List<Str>),
    C2(Adt97),
    C3(Str),
    C4(Adt84),
}

#[derive(Clone)]
pub enum Adt120 {
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
pub enum Adt121 {
    C0(Option<Str>, Option<Adt97>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1(Adt91),
    C2,
}

#[derive(Clone)]
pub enum Adt123 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt90),
    C1(Adt103),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Adt111, List<Adt112>),
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt91, Adt128, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt128 {
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
pub enum Adt129 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt131 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt112>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0(Adt90),
    C1(List<Adt114>, List<u64>),
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(Adt128),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt137),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt91, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt138 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt139 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt142 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt94),
    C1(Adt132),
}

#[derive(Clone)]
pub enum Adt145 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt148 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt150 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> R<Result<Adt32, Adt84>> {
        match self {
            Fn0::F3 => f3(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F34,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> Result<Adt49, Adt84> {
        match self {
            Fn1::F34 => f34(p0),
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

pub fn f3(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2 = f14(f12()?)?;
    match m2 {
        None => {
            f15(v0)
        }
        Some(v1) => {
            Ok(Err::<Adt32, Adt84>(v1))
        }
    }
}

pub fn f4(v0: Bytes) -> R<Result<Adt85, Adt87>> {
    let m3 = f16(v0.clone(), 0u64)?;
    match m3 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(0u64)))
        }
        Some(v1) => {
            f17(v0.clone(), v1.clone(), f16(v0, match v1 {
                Adt89::C0(_, h4) => {
                    h4
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt88) -> List<u64> {
    let a25 = f18(match v0.clone() {
        Adt88::C0(h5, _, _, _) => {
            h5
        }
    });
    let a26 = {
        let a23 = f18({
            let a7 = match v0.clone() {
                Adt88::C0(_, h6, _, _) => {
                    h6
                }
            };
            length_list(a7)
        });
        let a24 = {
            let a21 = f19(match v0.clone() {
                Adt88::C0(_, h8, _, _) => {
                    h8
                }
            });
            let a22 = {
                let a19 = f18({
                    let a10 = match v0.clone() {
                        Adt88::C0(_, _, h9, _) => {
                            h9
                        }
                    };
                    length_list(a10)
                });
                let a20 = {
                    let a17 = f20(match v0.clone() {
                        Adt88::C0(_, _, h11, _) => {
                            h11
                        }
                    });
                    let a18 = {
                        let a15 = f18({
                            let a13 = match v0.clone() {
                                Adt88::C0(_, _, _, h12) => {
                                    h12
                                }
                            };
                            length_list(a13)
                        });
                        let a16 = match v0 {
                            Adt88::C0(_, _, _, h14) => {
                                h14
                            }
                        };
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
}

pub fn f6() -> Adt88 {
    Adt88::C0(f21(), f22(), List::<List<u64>>::nil(), f23())
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt85) -> R<Adt88> {
    if f24(match v2.clone() {
        Adt85::C0(h27, _, _, _, _) => {
            h27
        }
    }) {
        f25(v0, v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h28) => {
                h28
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h29, _, _) => {
                h29
            }
        }, f26(match v2 {
            Adt85::C0(_, _, _, h30, _) => {
                h30
            }
        })?)
    } else {
        Ok(f27())
    }
}

pub fn f8() -> Adt1 {
    Adt1::C0(Str::lit("mcp-core-fixture"), None::<Str>, Str::lit("0.1.0"), None::<Str>, None::<Str>, None::<List<Adt2>>)
}

pub fn f9() -> Adt4 {
    Adt4::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt8>, None::<Adt9>, Some(Adt8::C0(None::<bool>)), None::<Adt5>)
}

pub fn f10() -> Adt10 {
    Adt10::C0(0u64, Adt11::C1)
}

pub fn f11(v0: List<Adt35>) -> R<List<Adt12>> {
    let m31 = f28(v0)?;
    match m31 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f12() -> R<List<Adt35>> {
    f29(f30(), List::<Adt35>::nil())
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt35>) -> R<Option<Adt84>> {
    let m32 = f28(v0)?;
    match m32 {
        Ok(_) => {
            Ok(None::<Adt84>)
        }
        Err(v1) => {
            Ok(Some(Adt84::C9(f31(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m33 = v0;
    match m33 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f32(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f33(f12()?, Fn1::F34, v2)
        }
        Adt14::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C4(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C5(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C6(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C7(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C8(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C9(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C10(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C11(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt89>> {
    let m34 = f35(v0.clone(), v1)?;
    match m34 {
        None => {
            Ok(None::<Adt89>)
        }
        Some(v2) => {
            let m40 = {
                let a37 = v0;
                let a38 = {
                    let a35 = v1;
                    let a36 = 4u64;
                    nat_add(a35, a36)?
                };
                let a39 = v2;
                slice_bytes(a37, a38, a39)
            };
            match m40 {
                None => {
                    Ok(None::<Adt89>)
                }
                Some(v3) => {
                    Ok(Some(Adt89::C0(v3, {
                        let a43 = {
                            let a41 = v1;
                            let a42 = 4u64;
                            nat_add(a41, a42)?
                        };
                        let a44 = v2;
                        nat_add(a43, a44)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt89, v2: Option<Adt89>) -> R<Result<Adt85, Adt87>> {
    let m45 = v2;
    match m45 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v1 {
                Adt89::C0(_, h46) => {
                    h46
                }
            })))
        }
        Some(v3) => {
            f36(v0.clone(), v1, v3.clone(), f35(v0, match v3 {
                Adt89::C0(_, h47) => {
                    h47
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a51 = {
            let a48 = v0;
            let a49 = 16777216u64;
            let a50 = 0u64;
            nat_quot(a48, a49, a50)
        };
        let a52 = 256u64;
        let a53 = 0u64;
        nat_rem(a51, a52, a53)
    }, List::cons({
        let a57 = {
            let a54 = v0;
            let a55 = 65536u64;
            let a56 = 0u64;
            nat_quot(a54, a55, a56)
        };
        let a58 = 256u64;
        let a59 = 0u64;
        nat_rem(a57, a58, a59)
    }, List::cons({
        let a63 = {
            let a60 = v0;
            let a61 = 256u64;
            let a62 = 0u64;
            nat_quot(a60, a61, a62)
        };
        let a64 = 256u64;
        let a65 = 0u64;
        nat_rem(a63, a64, a65)
    }, List::cons({
        let a66 = v0;
        let a67 = 256u64;
        let a68 = 0u64;
        nat_rem(a66, a67, a68)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt86>) -> List<u64> {
    f37(f38(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f39(f40(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f41(), f42()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a71 = f43();
    let a72 = {
        let a69 = f44();
        let a70 = f45();
        append_list(a69, a70)
    };
    append_list(a71, a72)
}

pub fn f24(v0: List<u64>) -> bool {
    f46(v0, f47())
}

pub fn f25(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt90>) -> R<Adt88> {
    let m73 = v4;
    match m73 {
        Ok(v6) => {
            f52(v0, v1, v2, v3, f53(v6)?)
        }
        Err(_) => {
            f48(f49(), f50(None::<Adt91>, f51(Adt92::C0)))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Result<Adt6, Adt90>> {
    f54(f55(v0)?)
}

pub fn f27() -> Adt88 {
    Adt88::C0(f56(), List::cons(Adt86::C0(f57(), f58()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f28(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt94>> {
    let m74 = f59(v0.clone());
    match m74 {
        None => {
            f60(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f29(p77: u64, p78: List<Adt35>) -> R<List<Adt35>> {
    let m76 = (p77, p78);
    {
        let mut m76 = m76;
        loop {
            let step = {
                let (v0, v1) = m76;
                let m75 = v0;
                if m75 == 0 {
                    Err::<(u64, List<Adt35>), R<List<Adt35>>>(Ok(v1))
                } else {
                    let v2 = m75 - 1;
                    let v3: Adt35 = f61(f62(v2)?, Str::lit("Answers with its own name"), f63());
                    Ok::<(u64, List<Adt35>), R<List<Adt35>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m76 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f30() -> u64 {
    60u64
}

pub fn f31(v0: Adt94) -> Str {
    let m79 = v0;
    match m79 {
        Adt94::C0(v1) => {
            let a80 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a81 = Str::lit(" ");
            join(a80, a81)
        }
        Adt94::C1(v2) => {
            let a82 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a83 = Str::lit(" ");
            join(a82, a83)
        }
        Adt94::C2(v3) => {
            let a84 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a85 = Str::lit(" ");
            join(a84, a85)
        }
        Adt94::C3(v4) => {
            let a86 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a87 = Str::lit(" ");
            join(a86, a87)
        }
        Adt94::C4(v5, v6) => {
            let a88 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt94::C5(v7, v8) => {
            let a90 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt94::C6(v9, v10) => {
            let a92 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt94::C7(v11) => {
            let a94 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt94::C8(v12) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt94::C9(v13) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt94::C10(v14) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
        Adt94::C11(v15) => {
            let a102 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a103 = Str::lit(" ");
            join(a102, a103)
        }
        Adt94::C12(v16, v17) => {
            let a104 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a105 = Str::lit(" ");
            join(a104, a105)
        }
    }
}

pub fn f32(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m107 = match v2 {
        Adt20::C0(_, h106) => {
            h106
        }
    };
    match m107 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f33(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m109 = f64(match v2.clone() {
        Adt21::C0(_, h108, _, _, _) => {
            h108
        }
    }, v0);
    match m109 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h110, _, _, _) => {
                    h110
                }
            })))
        }
        Some(v3) => {
            f65(v3, v1, v2)
        }
    }
}

pub fn f34(v0: Adt21) -> Result<Adt49, Adt84> {
    Ok::<Adt49, Adt84>(f66(List::cons(f67(match v0 {
        Adt21::C0(_, h111, _, _, _) => {
            h111
        }
    }), List::<Adt38>::nil())))
}

pub fn f35(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a115 = {
        let a112 = v1;
        let a113 = 4u64;
        nat_add(a112, a113)?
    };
    let a116 = {
        let a114 = v0.clone();
        length_bytes(a114)
    };
    if nat_le(a115, a116) {
        Ok(Some({
            let a133 = {
                let a117 = f68(v0.clone(), v1)?;
                let a118 = 16777216u64;
                nat_mul(a117, a118)?
            };
            let a134 = {
                let a131 = {
                    let a121 = f68(v0.clone(), {
                        let a119 = v1;
                        let a120 = 1u64;
                        nat_add(a119, a120)?
                    })?;
                    let a122 = 65536u64;
                    nat_mul(a121, a122)?
                };
                let a132 = {
                    let a129 = {
                        let a125 = f68(v0.clone(), {
                            let a123 = v1;
                            let a124 = 2u64;
                            nat_add(a123, a124)?
                        })?;
                        let a126 = 256u64;
                        nat_mul(a125, a126)?
                    };
                    let a130 = f68(v0, {
                        let a127 = v1;
                        let a128 = 3u64;
                        nat_add(a127, a128)?
                    })?;
                    nat_add(a129, a130)?
                };
                nat_add(a131, a132)?
            };
            nat_add(a133, a134)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f36(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m135 = v3;
    match m135 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h136) => {
                    h136
                }
            })))
        }
        Some(v4) => {
            let m140 = f69(v0.clone(), {
                let a138 = match v2.clone() {
                    Adt89::C0(_, h137) => {
                        h137
                    }
                };
                let a139 = 4u64;
                nat_add(a138, a139)?
            }, v4, List::<Adt86>::nil())?;
            match m140 {
                Ok(v6) => {
                    f70(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f37(p145: List<Adt86>, p146: List<u64>) -> List<u64> {
    let m144 = (p145, p146);
    {
        let mut m144 = m144;
        loop {
            let step = {
                let (v0, v1) = m144;
                let m141 = v0;
                match m141.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a142 = f71(v2);
                            let a143 = v1;
                            append_list(a142, a143)
                        }))
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

pub fn f38(p149: List<Adt86>, p150: List<Adt86>) -> List<Adt86> {
    let m148 = (p149, p150);
    {
        let mut m148 = m148;
        loop {
            let step = {
                let (v0, v1) = m148;
                let m147 = v0;
                match m147.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f39(p155: List<List<u64>>, p156: List<u64>) -> List<u64> {
    let m154 = (p155, p156);
    {
        let mut m154 = m154;
        loop {
            let step = {
                let (v0, v1) = m154;
                let m151 = v0;
                match m151.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a152 = f72(v2);
                            let a153 = v1;
                            append_list(a152, a153)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m154 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40(p159: List<List<u64>>, p160: List<List<u64>>) -> List<List<u64>> {
    let m158 = (p159, p160);
    {
        let mut m158 = m158;
        loop {
            let step = {
                let (v0, v1) = m158;
                let m157 = v0;
                match m157.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f42() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f43() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f44() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f45() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f46(p167: List<u64>, p168: List<u64>) -> bool {
    let m166 = (p167, p168);
    {
        let mut m166 = m166;
        loop {
            let step = {
                let (v0, v1) = m166;
                let m161 = v0;
                match m161.uncons() {
                    None => {
                        let m162 = v1;
                        match m162.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m163 = v1;
                        match m163.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a164 = v4;
                                let a165 = v6;
                                if nat_eq(a164, a165) {
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
                Ok(next) => { m166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f48(v0: u64, v1: Adt6) -> R<Adt88> {
    let m169 = f73(v1)?;
    match m169 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f49() -> u64 {
    400u64
}

pub fn f50(v0: Option<Adt91>, v1: Adt80) -> Adt6 {
    Adt6::C5(f74(v0, v1))
}

pub fn f51(v0: Adt92) -> Adt80 {
    Adt80::C0(f75(v0.clone()), f76(v0), None::<Adt6>)
}

pub fn f52(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt95, Adt92>) -> R<Adt88> {
    let m170 = v4;
    match m170 {
        Ok(v6) => {
            f79(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f48(f77(v5.clone()), f50(f78(v5.clone()), f51(v5)))
        }
    }
}

pub fn f53(v0: Adt6) -> R<Result<Adt95, Adt92>> {
    let m171 = v0;
    match m171 {
        Adt6::C0 => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C1(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C2(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C3(_) => {
            Ok(f81(None::<Adt91>, Adt93::C0))
        }
        Adt6::C4(_) => {
            Ok(f81(None::<Adt91>, Adt93::C1))
        }
        Adt6::C5(v1) => {
            f80(Adt5::C0(v1))
        }
    }
}

pub fn f54(v0: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m172 = f82(v0.clone())?;
    match m172 {
        Adt101::C0 => {
            f83(f84(), 0u64, v0)
        }
        Adt101::C1(v1) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C0(v1)))
        }
        Adt101::C2(v2) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C1(v2)))
        }
    }
}

pub fn f55(v0: Bytes) -> R<List<u64>> {
    f85(v0.clone(), {
        let a173 = v0;
        length_bytes(a173)
    }, List::<u64>::nil())
}

pub fn f56() -> u64 {
    405u64
}

pub fn f57() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f58() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f59(v0: List<Adt35>) -> Option<Adt94> {
    let m174 = f86(v0, List::<Str>::nil());
    match m174 {
        None => {
            None::<Adt94>
        }
        Some(v1) => {
            Some(Adt94::C1(v1))
        }
    }
}

pub fn f60(p178: List<Adt35>, p179: List<Adt12>) -> R<Result<List<Adt12>, Adt94>> {
    let m177 = (p178, p179);
    {
        let mut m177 = m177;
        loop {
            let step = {
                let (v0, v1) = m177;
                let m175 = v0;
                match m175.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Ok::<List<Adt12>, Adt94>(f87(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m176 = f88(v2)?;
                        match m176 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>((v3, f87(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Err::<List<Adt12>, Adt94>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m177 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f62(v0: u64) -> R<Str> {
    let a181 = List::cons(Str::lit("crowd_"), List::cons({
        let a180 = f89(v0, 0i64)?;
        format_i64(a180)
    }, List::<Str>::nil()));
    let a182 = Str::lit("");
    Ok(join(a181, a182))
}

pub fn f63() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f64(v0: Str, p188: List<Adt35>) -> Option<Adt35> {
    let m187 = p188;
    {
        let mut m187 = m187;
        loop {
            let step = {
                let v1 = m187;
                let m183 = v1;
                match m183.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a185 = match v2.clone() {
                            Adt35::C0(h184, _, _, _, _, _, _, _) => {
                                h184
                            }
                        };
                        let a186 = v0.clone();
                        if equal(a185, a186) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m187 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f65(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m195 = f90(Adt6::C5({
        let m191 = match v0.clone() {
            Adt35::C0(_, _, _, _, h190, _, _, _) => {
                h190
            }
        };
        match m191 {
            Adt5::C0(h189) => {
                h189
            }
        }
    }), Adt6::C5({
        let m194 = match v0.clone() {
            Adt35::C0(_, _, _, _, h193, _, _, _) => {
                h193
            }
        };
        match m194 {
            Adt5::C0(h192) => {
                h192
            }
        }
    }), f91(v2.clone()))?;
    match m195 {
        Adt109::C0 => {
            f92(v0, v1, v2)
        }
        Adt109::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f93(f94(v3, v4)?, true))))
        }
        Adt109::C2(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt109::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt109::C4 => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f66(v0: List<Adt38>) -> Adt49 {
    f95(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f67(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f68(v0: Bytes, v1: u64) -> R<u64> {
    let m198 = {
        let a196 = v0;
        let a197 = v1;
        index_bytes(a196, a197)
    };
    match m198 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a243 = {
                let a219 = {
                    let a207 = {
                        let a201 = {
                            let a199 = v3;
                            let a200 = 128u8;
                            fixed_u8::bit_and(a199, a200)
                        };
                        let a202 = 128u8;
                        if equal(a201, a202) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a208 = {
                        let a205 = {
                            let a203 = v3;
                            let a204 = 64u8;
                            fixed_u8::bit_and(a203, a204)
                        };
                        let a206 = 64u8;
                        if equal(a205, a206) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a207, a208)?
                };
                let a220 = {
                    let a217 = {
                        let a211 = {
                            let a209 = v3;
                            let a210 = 32u8;
                            fixed_u8::bit_and(a209, a210)
                        };
                        let a212 = 32u8;
                        if equal(a211, a212) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a218 = {
                        let a215 = {
                            let a213 = v3;
                            let a214 = 16u8;
                            fixed_u8::bit_and(a213, a214)
                        };
                        let a216 = 16u8;
                        if equal(a215, a216) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a217, a218)?
                };
                nat_add(a219, a220)?
            };
            let a244 = {
                let a241 = {
                    let a229 = {
                        let a223 = {
                            let a221 = v3;
                            let a222 = 8u8;
                            fixed_u8::bit_and(a221, a222)
                        };
                        let a224 = 8u8;
                        if equal(a223, a224) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a230 = {
                        let a227 = {
                            let a225 = v3;
                            let a226 = 4u8;
                            fixed_u8::bit_and(a225, a226)
                        };
                        let a228 = 4u8;
                        if equal(a227, a228) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a229, a230)?
                };
                let a242 = {
                    let a239 = {
                        let a233 = {
                            let a231 = v3;
                            let a232 = 2u8;
                            fixed_u8::bit_and(a231, a232)
                        };
                        let a234 = 2u8;
                        if equal(a233, a234) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a240 = {
                        let a237 = {
                            let a235 = v3;
                            let a236 = 1u8;
                            fixed_u8::bit_and(a235, a236)
                        };
                        let a238 = 1u8;
                        if equal(a237, a238) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a239, a240)?
                };
                nat_add(a241, a242)?
            };
            nat_add(a243, a244)
        }
    }
}

pub fn f69(v0: Bytes, p254: u64, p255: u64, p256: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m253 = (p254, (p255, p256));
    {
        let mut m253 = m253;
        loop {
            let step = {
                let (v1, (v2, v3)) = m253;
                let m245 = v2;
                if m245 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f96(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m245 - 1;
                    let m246 = f16(v0.clone(), v1)?;
                    match m246 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m248 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h247) => {
                                    h247
                                }
                            })?;
                            match m248 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h249) => {
                                            h249
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h250) => {
                                            h250
                                        }
                                    }, (v4, List::cons(Adt86::C0(f55(match v5 {
                                        Adt89::C0(h251, _) => {
                                            h251
                                        }
                                    })?, f55(match v6 {
                                        Adt89::C0(h252, _) => {
                                            h252
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m253 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m258 = f16(v0.clone(), {
        let (_, h257) = v3.clone();
        h257
    })?;
    match m258 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h259) = v3;
                h259
            })))
        }
        Some(v4) => {
            let m261 = f35(v0.clone(), match v4.clone() {
                Adt89::C0(_, h260) => {
                    h260
                }
            })?;
            match m261 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h262) => {
                            h262
                        }
                    })))
                }
                Some(v5) => {
                    let a267 = {
                        let a264 = match v4.clone() {
                            Adt89::C0(_, h263) => {
                                h263
                            }
                        };
                        let a265 = 4u64;
                        nat_add(a264, a265)?
                    };
                    let a268 = {
                        let a266 = v0;
                        length_bytes(a266)
                    };
                    if nat_eq(a267, a268) {
                        Ok(Ok::<Adt85, Adt87>(Adt85::C0(f55(match v1 {
                            Adt89::C0(h269, _) => {
                                h269
                            }
                        })?, f55(match v2 {
                            Adt89::C0(h270, _) => {
                                h270
                            }
                        })?, {
                            let (h271, _) = v3;
                            h271
                        }, match v4 {
                            Adt89::C0(h272, _) => {
                                h272
                            }
                        }, v5)))
                    } else {
                        Ok(Err::<Adt85, Adt87>(Adt87::C1({
                            let a274 = match v4 {
                                Adt89::C0(_, h273) => {
                                    h273
                                }
                            };
                            let a275 = 4u64;
                            nat_add(a274, a275)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f71(v0: Adt86) -> List<u64> {
    let a286 = f18({
        let a277 = match v0.clone() {
            Adt86::C0(h276, _) => {
                h276
            }
        };
        length_list(a277)
    });
    let a287 = {
        let a284 = match v0.clone() {
            Adt86::C0(h278, _) => {
                h278
            }
        };
        let a285 = {
            let a282 = f18({
                let a280 = match v0.clone() {
                    Adt86::C0(_, h279) => {
                        h279
                    }
                };
                length_list(a280)
            });
            let a283 = match v0 {
                Adt86::C0(_, h281) => {
                    h281
                }
            };
            append_list(a282, a283)
        };
        append_list(a284, a285)
    };
    append_list(a286, a287)
}

pub fn f72(v0: List<u64>) -> List<u64> {
    let a289 = f18({
        let a288 = v0.clone();
        length_list(a288)
    });
    let a290 = v0;
    append_list(a289, a290)
}

pub fn f73(v0: Adt6) -> R<Result<List<u64>, Adt90>> {
    f97(f98(), v0)
}

pub fn f74(v0: Option<Adt91>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), {
        let a291 = f100(v0);
        let a292 = List::cons((Str::lit("error"), f101(v1)), List::<(Str, Adt6)>::nil());
        append_list(a291, a292)
    })
}

pub fn f75(v0: Adt92) -> i64 {
    let m293 = v0;
    match m293 {
        Adt92::C0 => {
            f102()
        }
        Adt92::C1(_, _) => {
            f103()
        }
    }
}

pub fn f76(v0: Adt92) -> Str {
    let m294 = v0;
    match m294 {
        Adt92::C0 => {
            Str::lit("Parse error")
        }
        Adt92::C1(_, v2) => {
            f104(v2)
        }
    }
}

pub fn f77(v0: Adt92) -> u64 {
    let m295 = v0;
    match m295 {
        Adt92::C0 => {
            f49()
        }
        Adt92::C1(_, _) => {
            f49()
        }
    }
}

pub fn f78(v0: Adt92) -> Option<Adt91> {
    let m296 = v0;
    match m296 {
        Adt92::C0 => {
            None::<Adt91>
        }
        Adt92::C1(v1, _) => {
            v1
        }
    }
}

pub fn f79(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt95) -> R<Adt88> {
    f105(f106(v0.clone(), v1, v2, Some(f107(v3, match v0 {
        Adt0::C0(_, _, _, _, h297, _) => {
            h297
        }
    }, f108(v4.clone()), match v4.clone() {
        Adt95::C0(_, h298, _) => {
            h298
        }
    })?), f109(), v4)?)
}

pub fn f80(v0: Adt5) -> R<Result<Adt95, Adt92>> {
    let m299 = f110(v0.clone())?;
    match m299 {
        Adt122::C0 => {
            let m300 = f111(Str::lit("method"), v0.clone());
            match m300 {
                Adt123::C0 => {
                    Ok(f81(None::<Adt91>, f112(v0)))
                }
                Adt123::C1(_) => {
                    Ok(f81(None::<Adt91>, if f113(v0) {
                        Adt93::C8
                    } else {
                        Adt93::C7
                    }))
                }
            }
        }
        Adt122::C1(v2) => {
            Ok(f114(v2, v0))
        }
        Adt122::C2 => {
            Ok(f81(None::<Adt91>, Adt93::C5))
        }
    }
}

pub fn f81(v0: Option<Adt91>, v1: Adt93) -> Result<Adt95, Adt92> {
    Err::<Adt95, Adt92>(Adt92::C1(v0, v1))
}

pub fn f82(v0: List<u64>) -> R<Adt101> {
    Ok(f115({
        let a301 = v0.clone();
        length_list(a301)
    }, f116(Adt102::C0, 0u64, 0u64, v0)?))
}

pub fn f83(p308: Adt103, p309: u64, p310: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m307 = (p308, (p309, p310));
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v0, (v1, v2)) = m307;
                let m302 = v2;
                match m302.uncons() {
                    None => {
                        let m303 = f117(v0, v1)?;
                        match m303 {
                            Ok(v3) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Ok::<Adt6, Adt90>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m304 = f118(v0, v1, v5)?;
                        match m304 {
                            Adt124::C0(v7) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v7)))
                            }
                            Adt124::C1(v8) => {
                                Ok::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>((v8, ({
                                    let a305 = v1;
                                    let a306 = 1u64;
                                    nat_add(a305, a306)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m307 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84() -> Adt103 {
    Adt103::C0(Adt104::C0, List::<Adt108>::nil(), 0u64)
}

pub fn f85(v0: Bytes, p313: u64, p314: List<u64>) -> R<List<u64>> {
    let m312 = (p313, p314);
    {
        let mut m312 = m312;
        loop {
            let step = {
                let (v1, v2) = m312;
                let m311 = v1;
                if m311 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m311 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f68(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(p320: List<Adt35>, p321: List<Str>) -> Option<Str> {
    let m319 = (p320, p321);
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v0, v1) = m319;
                let m315 = v0;
                match m315.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f119(match v2.clone() {
                            Adt35::C0(h316, _, _, _, _, _, _, _) => {
                                h316
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h317, _, _, _, _, _, _, _) => {
                                    h317
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h318, _, _, _, _, _, _, _) => {
                                    h318
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(p324: List<Adt12>, p325: List<Adt12>) -> List<Adt12> {
    let m323 = (p324, p325);
    {
        let mut m323 = m323;
        loop {
            let step = {
                let (v0, v1) = m323;
                let m322 = v0;
                match m322.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f88(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m328 = f120(match v0.clone() {
        Adt35::C0(h326, _, _, _, _, _, _, _) => {
            h326
        }
    }, f121(match v0.clone() {
        Adt35::C0(h327, _, _, _, _, _, _, _) => {
            h327
        }
    })?);
    match m328 {
        None => {
            f122(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f89(p331: u64, p332: i64) -> R<i64> {
    let m330 = (p331, p332);
    {
        let mut m330 = m330;
        loop {
            let step = {
                let (v0, v1) = m330;
                let m329 = v0;
                if m329 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m329 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f123(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt109> {
    f124(v0, f125(), Adt111::C0, List::cons(Adt112::C0(v1, v2, List::<Adt110>::nil()), List::<Adt112>::nil()))
}

pub fn f91(v0: Adt21) -> Adt6 {
    let m334 = match v0 {
        Adt21::C0(_, _, h333, _, _) => {
            h333
        }
    };
    match m334 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h335) => {
                    h335
                }
            })
        }
    }
}

pub fn f92(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m337 = {
        let c336: Fn1 = v1;
        c336.apply(v2)
    };
    match m337 {
        Ok(v4) => {
            f126(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f93(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f94(v0: List<Adt110>, v1: Str) -> R<Str> {
    let v2: Str = f127(v0)?;
    let a340 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a338 = v2.clone();
        let a339 = Str::lit("");
        if equal(a338, a339) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a341 = Str::lit(" ");
    Ok(join(a340, a341))
}

pub fn f95(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f96(p344: List<Adt86>, p345: List<Adt86>) -> List<Adt86> {
    let m343 = (p344, p345);
    {
        let mut m343 = m343;
        loop {
            let step = {
                let (v0, v1) = m343;
                let m342 = v0;
                match m342.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt90>> {
    let m346 = f128(v0, List::cons(Adt114::C1(0u64, v1), List::<Adt114>::nil()), List::<u64>::nil())?;
    match m346 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt90>(f129(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt90>(v3))
        }
    }
}

pub fn f98() -> u64 {
    67108864u64
}

pub fn f99() -> Str {
    Str::lit("2.0")
}

pub fn f100(v0: Option<Adt91>) -> List<(Str, Adt6)> {
    let m347 = v0;
    match m347 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f130(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f101(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h348, _, _) => {
            h348
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h349, _) => {
            h349
        }
    })), f131(match v0 {
        Adt80::C0(_, _, h350) => {
            h350
        }
    }))))
}

pub fn f102() -> i64 {
    -32700i64
}

pub fn f103() -> i64 {
    -32600i64
}

pub fn f104(v0: Adt93) -> Str {
    let m351 = v0;
    match m351 {
        Adt93::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt93::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt93::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt93::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt93::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt93::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt93::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt93::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt93::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f105(v0: Adt115) -> R<Adt88> {
    let m353 = match v0.clone() {
        Adt115::C0(_, h352) => {
            h352
        }
    };
    match m353 {
        Ok(v1) => {
            let m354 = f132(v1.clone())?;
            match m354 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    Ok(f133(f48(f134(), f135(match v0 {
                        Adt115::C0(h355, _) => {
                            h355
                        }
                    }, v2))?, f136(match v1 {
                        Adt116::C0(_, _, h356) => {
                            h356
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v3) => {
            f48(f137(v3.clone()), f50(Some(match v0 {
                Adt115::C0(h357, _) => {
                    h357
                }
            }), f138(v3)?))
        }
    }
}

pub fn f106(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt96>, v4: List<Str>, v5: Adt95) -> R<Adt115> {
    Ok(Adt115::C0(match v5.clone() {
        Adt95::C0(h358, _, _) => {
            h358
        }
    }, f139(v0, v1, v2, f140(v3, v4, v5)?)?))
}

pub fn f107(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt96> {
    let v4: Adt121 = f141(Adt98::C0, f142(v0.clone(), f143())?);
    let v5: Adt121 = f141(Adt98::C1, f142(v0.clone(), f144())?);
    let v6: Adt121 = if f145(v3) {
        f141(Adt98::C2, f146(f142(v0.clone(), f147())?)?)
    } else {
        f148()
    };
    Ok(Adt96::C0(match v4.clone() {
        Adt121::C0(h359, _) => {
            h359
        }
    }, match v5.clone() {
        Adt121::C0(h360, _) => {
            h360
        }
    }, match v6.clone() {
        Adt121::C0(h361, _) => {
            h361
        }
    }, f149(match v4 {
        Adt121::C0(_, h362) => {
            h362
        }
    }, f149(match v5 {
        Adt121::C0(_, h363) => {
            h363
        }
    }, match v6 {
        Adt121::C0(_, h364) => {
            h364
        }
    })), f150(v0, v1, v2)?))
}

pub fn f108(v0: Adt95) -> Option<Str> {
    let a366 = match v0.clone() {
        Adt95::C0(_, h365, _) => {
            h365
        }
    };
    let a367 = Str::lit("tools/call");
    if equal(a366, a367) {
        let m369 = match v0 {
            Adt95::C0(_, _, h368) => {
                h368
            }
        };
        match m369 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m370 = f111(Str::lit("name"), v1);
                match m370 {
                    Adt123::C0 => {
                        None::<Str>
                    }
                    Adt123::C1(v2) => {
                        f151(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f109() -> List<Str> {
    List::cons(f152(), List::<Str>::nil())
}

pub fn f110(v0: Adt5) -> R<Adt122> {
    let m371 = f111(Str::lit("id"), v0);
    match m371 {
        Adt123::C0 => {
            Ok(Adt122::C0)
        }
        Adt123::C1(v1) => {
            let m372 = v1;
            match m372 {
                Adt6::C0 => {
                    Ok(Adt122::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt122::C2)
                }
                Adt6::C2(v3) => {
                    let m373 = f153(v3)?;
                    match m373 {
                        None => {
                            Ok(Adt122::C2)
                        }
                        Some(v4) => {
                            Ok(Adt122::C1(Adt91::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt122::C1(Adt91::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt122::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt122::C2)
                }
            }
        }
    }
}

pub fn f111(v0: Str, v1: Adt5) -> Adt123 {
    f154(v0, match v1 {
        Adt5::C0(h374) => {
            h374
        }
    })
}

pub fn f112(v0: Adt5) -> Adt93 {
    if f113(v0) {
        Adt93::C8
    } else {
        Adt93::C3
    }
}

pub fn f113(v0: Adt5) -> bool {
    if f155(Str::lit("result"), v0.clone()) {
        true
    } else {
        f155(Str::lit("error"), v0)
    }
}

pub fn f114(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m375 = f111(Str::lit("jsonrpc"), v1.clone());
    match m375 {
        Adt123::C0 => {
            f81(Some(v0), Adt93::C2)
        }
        Adt123::C1(v2) => {
            let m376 = f151(v2);
            match m376 {
                None => {
                    f81(Some(v0), Adt93::C2)
                }
                Some(v3) => {
                    let a377 = v3;
                    let a378 = f99();
                    if equal(a377, a378) {
                        f156(v0, v1)
                    } else {
                        f81(Some(v0), Adt93::C2)
                    }
                }
            }
        }
    }
}

pub fn f115(v0: u64, v1: u64) -> Adt101 {
    let a379 = v1;
    let a380 = 0u64;
    if nat_eq(a379, a380) {
        Adt101::C0
    } else {
        let a381 = v1;
        let a382 = 2u64;
        if nat_eq(a381, a382) {
            Adt101::C2(v0)
        } else {
            Adt101::C1({
                let a386 = v0;
                let a387 = {
                    let a383 = v1;
                    let a384 = 3u64;
                    let a385 = 0u64;
                    nat_quot(a383, a384, a385)
                };
                nat_sub(a386, a387)
            })
        }
    }
}

pub fn f116(p415: Adt102, p416: u64, p417: u64, p418: List<u64>) -> R<u64> {
    let m414 = (p415, (p416, (p417, p418)));
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m414;
                let m388 = v3;
                match m388.uncons() {
                    None => {
                        let m389 = v0;
                        match m389 {
                            Adt102::C0 => {
                                Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt102::C1 => {
                                Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt102::C2 => {
                                Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt102::C3 => {
                                Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m390 = v0;
                        match m390 {
                            Adt102::C0 => {
                                let a391 = v4;
                                let a392 = 128u64;
                                if nat_lt(a391, a392) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a393 = v4;
                                    let a394 = 194u64;
                                    if nat_lt(a393, a394) {
                                        let v6: u64 = f157({
                                            let a395 = List::cons(v4, v5);
                                            length_list(a395)
                                        })?;
                                        Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a396 = v4;
                                        let a397 = 224u64;
                                        if nat_lt(a396, a397) {
                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a398 = v4;
                                            let a399 = 224u64;
                                            if nat_eq(a398, a399) {
                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a400 = v4;
                                                let a401 = 237u64;
                                                if nat_eq(a400, a401) {
                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a402 = v4;
                                                    let a403 = 240u64;
                                                    if nat_lt(a402, a403) {
                                                        Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a404 = v4;
                                                        let a405 = 240u64;
                                                        if nat_eq(a404, a405) {
                                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a406 = v4;
                                                            let a407 = 244u64;
                                                            if nat_eq(a406, a407) {
                                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a408 = v4;
                                                                let a409 = 244u64;
                                                                if nat_lt(a408, a409) {
                                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f157({
                                                                        let a410 = List::cons(v4, v5);
                                                                        length_list(a410)
                                                                    })?;
                                                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt102::C1 => {
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f157({
                                        let a411 = List::cons(v4, v5);
                                        length_list(a411)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt102::C2 => {
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f157({
                                        let a412 = List::cons(v4, v5);
                                        length_list(a412)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt102::C3 => {
                                if f158(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f157({
                                        let a413 = List::cons(v4, v5);
                                        length_list(a413)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt103, v1: u64) -> R<Result<Adt6, Adt90>> {
    let m420 = match v0.clone() {
        Adt103::C0(h419, _, _) => {
            h419
        }
    };
    match m420 {
        Adt104::C0 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C1 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C2 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C3 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C4 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C5 => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C7(_, _) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C3))
        }
        Adt104::C8(v3) => {
            let m421 = f159(v3)?;
            match m421 {
                Adt131::C0 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C7(v1)))
                }
                Adt131::C1 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C8(v1)))
                }
                Adt131::C2(v4) => {
                    let m426 = {
                        let m425 = f160(v4, match v0.clone() {
                            Adt103::C0(_, h423, _) => {
                                h423
                            }
                        }, match v0 {
                            Adt103::C0(_, _, h424) => {
                                h424
                            }
                        });
                        match m425 {
                            Adt103::C0(h422, _, _) => {
                                h422
                            }
                        }
                    };
                    match m426 {
                        Adt104::C0 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C1 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C2 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C3 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C4 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C5 => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C7(_, _) => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C8(_) => {
                            Ok(Err::<Adt6, Adt90>(Adt90::C3))
                        }
                        Adt104::C9(v5) => {
                            Ok(Ok::<Adt6, Adt90>(v5))
                        }
                    }
                }
            }
        }
        Adt104::C9(v2) => {
            Ok(Ok::<Adt6, Adt90>(v2))
        }
    }
}

pub fn f118(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let m428 = match v0.clone() {
        Adt103::C0(h427, _, _) => {
            h427
        }
    };
    match m428 {
        Adt104::C0 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                f162(v0, v1, v2)
            }
        }
        Adt104::C1 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a429 = v2;
                let a430 = 93u64;
                if nat_eq(a429, a430) {
                    Ok(f163(v0, v1))
                } else {
                    f162(v0, v1, v2)
                }
            }
        }
        Adt104::C2 => {
            Ok(f164(v0, v1, v2))
        }
        Adt104::C3 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a431 = v2;
                let a432 = 125u64;
                if nat_eq(a431, a432) {
                    Ok(f165(v0, v1))
                } else {
                    let a433 = v2;
                    let a434 = 34u64;
                    if nat_eq(a433, a434) {
                        Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f167(Adt90::C2(v1)))
                    }
                }
            }
        }
        Adt104::C4 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a435 = v2;
                let a436 = 34u64;
                if nat_eq(a435, a436) {
                    Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f167(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C5 => {
            if f161(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a437 = v2;
                let a438 = 58u64;
                if nat_eq(a437, a438) {
                    Ok(f166(v0, Adt104::C0))
                } else {
                    Ok(f167(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C6(v3, v4, v5) => {
            f168(v0, v1, v2, v3, v4, v5)
        }
        Adt104::C7(v6, v7) => {
            f169(v0, v1, v2, v6, v7)
        }
        Adt104::C8(v8) => {
            f170(v0, v1, v2, v8)
        }
        Adt104::C9(_) => {
            Ok(f164(v0, v1, v2))
        }
    }
}

pub fn f119(v0: Str, p443: List<Str>) -> bool {
    let m442 = p443;
    {
        let mut m442 = m442;
        loop {
            let step = {
                let v1 = m442;
                let m439 = v1;
                match m439.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a440 = v0.clone();
                        let a441 = v2;
                        if equal(a440, a441) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m442 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Str, v1: List<u64>) -> Option<Adt94> {
    if f171(v1) {
        Some(Adt94::C0(v0))
    } else {
        None::<Adt94>
    }
}

pub fn f121(v0: Str) -> R<List<u64>> {
    f55({
        let a444 = v0;
        utf8_encode(a444)
    })
}

pub fn f122(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m445 = f172(v0.clone());
    match m445 {
        None => {
            f173(v0.clone(), f174(f175(), match v0.clone() {
                Adt35::C0(h446, _, _, _, _, _, _, _) => {
                    h446
                }
            }, Adt6::C5({
                let m449 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h448, _, _, _) => {
                        h448
                    }
                };
                match m449 {
                    Adt5::C0(h447) => {
                        h447
                    }
                }
            }), List::cons(Adt125::C0(Adt6::C5({
                let m452 = match v0 {
                    Adt35::C0(_, _, _, _, h451, _, _, _) => {
                        h451
                    }
                };
                match m452 {
                    Adt5::C0(h450) => {
                        h450
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt125>::nil()), f176())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f123(v0: i64, v1: i64) -> R<i64> {
    let a454 = v0;
    let a455 = {
        let a453 = v1;
        int_neg(a453)?
    };
    int_sub(a454, a455)
}

pub fn f124(v0: Adt6, p474: u64, p475: Adt111, p476: List<Adt112>) -> R<Adt109> {
    let m473 = (p474, (p475, p476));
    {
        let mut m473 = m473;
        loop {
            let step = {
                let (v1, (v2, v3)) = m473;
                let m456 = v1;
                if m456 == 0 {
                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C4))
                } else {
                    let v4 = m456 - 1;
                    let m457 = v3;
                    match m457.uncons() {
                        None => {
                            let m458 = v2;
                            match m458 {
                                Adt111::C0 => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C0))
                                }
                                Adt111::C1(v5, v6) => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m459 = v2.clone();
                            match m459 {
                                Adt111::C0 => {
                                    let m460 = v7.clone();
                                    match m460 {
                                        Adt112::C0(v9, v10, v11) => {
                                            let m461 = f177(v0.clone(), v9, v10, v11.clone())?;
                                            match m461 {
                                                Adt133::C0(v12) => {
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v11, v12), v8)))
                                                }
                                                Adt133::C1(v13) => {
                                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C2(v13)))
                                                }
                                                Adt133::C2(v14) => {
                                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C3(v14)))
                                                }
                                                Adt133::C3(v15) => {
                                                    let v16: List<Adt112> = {
                                                        let a462 = v15;
                                                        let a463 = v8;
                                                        append_list(a462, a463)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt112::C1(_, _, _) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v8)))
                                        }
                                        Adt112::C2(_, _, _, _) => {
                                            let v24: Adt126 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v24.clone() {
                                                Adt126::C0(h464, _) => {
                                                    h464
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h465) => {
                                                    h465
                                                }
                                            })))
                                        }
                                        Adt112::C3(v25) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt112::C4(_, _, _, _) => {
                                            let v39: Adt126 = f178(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v39.clone() {
                                                Adt126::C0(h469, _) => {
                                                    h469
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h470) => {
                                                    h470
                                                }
                                            })))
                                        }
                                        Adt112::C5(v26, v27, v28, v29) => {
                                            let m466 = f179(v26, v27, v28, v29.clone());
                                            match m466 {
                                                Adt133::C0(v30) => {
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v29, v30), v8)))
                                                }
                                                Adt133::C1(v31) => {
                                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C2(v31)))
                                                }
                                                Adt133::C2(v32) => {
                                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C3(v32)))
                                                }
                                                Adt133::C3(v33) => {
                                                    let v34: List<Adt112> = {
                                                        let a467 = v33;
                                                        let a468 = v8;
                                                        append_list(a467, a468)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt111::C1(v40, v41) => {
                                    let v42: Adt126 = f180(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v42.clone() {
                                        Adt126::C0(h471, _) => {
                                            h471
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h472) => {
                                            h472
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125() -> u64 {
    67108864u64
}

pub fn f126(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f181({
        let m479 = match v1.clone() {
            Adt49::C0(h478, _) => {
                h478
            }
        };
        match m479 {
            Adt37::C0(h477, _, _) => {
                h477
            }
        }
    })? {
        f182(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f127(v0: List<Adt110>) -> R<Str> {
    let a480 = List::cons(Str::lit(""), f183(v0, List::<Str>::nil())?);
    let a481 = Str::lit("/");
    Ok(join(a480, a481))
}

pub fn f128(p486: u64, p487: List<Adt114>, p488: List<u64>) -> R<Result<List<u64>, Adt90>> {
    let m485 = (p486, (p487, p488));
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v0, (v1, v2)) = m485;
                let m482 = v0;
                if m482 == 0 {
                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Err::<List<u64>, Adt90>(Adt90::C13)))
                } else {
                    let v3 = m482 - 1;
                    let m483 = v1;
                    match m483.uncons() {
                        None => {
                            Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Ok::<List<u64>, Adt90>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m484 = f184(v4, v5, v2)?;
                            match m484 {
                                Adt134::C0(v6) => {
                                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Err::<List<u64>, Adt90>(v6)))
                                }
                                Adt134::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(p491: List<u64>, p492: List<u64>) -> List<u64> {
    let m490 = (p491, p492);
    {
        let mut m490 = m490;
        loop {
            let step = {
                let (v0, v1) = m490;
                let m489 = v0;
                match m489.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: Adt91) -> Adt6 {
    let m493 = v0;
    match m493 {
        Adt91::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt91::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f131(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m494 = v0;
    match m494 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f132(v0: Adt116) -> R<Option<Adt5>> {
    let m496 = f185(match v0.clone() {
        Adt116::C0(h495, _, _) => {
            h495
        }
    })?;
    match m496 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a499 = v1;
                let a500 = {
                    let m498 = f186(v0);
                    match m498 {
                        Adt5::C0(h497) => {
                            h497
                        }
                    }
                };
                append_list(a499, a500)
            })))
        }
    }
}

pub fn f133(v0: Adt88, v1: Option<List<List<u64>>>) -> Adt88 {
    let m501 = v1;
    match m501 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt88::C0(match v0.clone() {
                Adt88::C0(h502, _, _, _) => {
                    h502
                }
            }, match v0.clone() {
                Adt88::C0(_, h503, _, _) => {
                    h503
                }
            }, f40(v2, List::<List<u64>>::nil()), match v0 {
                Adt88::C0(_, _, _, h504) => {
                    h504
                }
            })
        }
    }
}

pub fn f134() -> u64 {
    200u64
}

pub fn f135(v0: Adt91, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), List::cons((Str::lit("id"), f130(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h505) => {
            h505
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f136(p509: List<Adt118>, p510: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m508 = (p509, p510);
    {
        let mut m508 = m508;
        loop {
            let step = {
                let (v0, v1) = m508;
                let m506 = v0;
                match m506.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m507 = f187(v2)?;
                        match m507 {
                            None => {
                                Err::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Adt119) -> u64 {
    let m511 = v0;
    match m511 {
        Adt119::C0(_) => {
            f49()
        }
        Adt119::C1(_, _) => {
            f49()
        }
        Adt119::C2(_) => {
            f49()
        }
        Adt119::C3(_) => {
            f188()
        }
        Adt119::C4(v6) => {
            f189(v6)
        }
    }
}

pub fn f138(v0: Adt119) -> R<Adt80> {
    Ok(Adt80::C0(f190(v0.clone()), f191(v0.clone()), f192(v0)?))
}

pub fn f139(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt127, Adt119>) -> R<Result<Adt116, Adt119>> {
    let m512 = v3;
    match m512 {
        Ok(v5) => {
            f193(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(v4))
        }
    }
}

pub fn f140(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Result<Adt127, Adt119>> {
    Ok(f194(f195(match v2.clone() {
        Adt95::C0(_, h513, _) => {
            h513
        }
    }), f196(v0, v1, v2)?))
}

pub fn f141(v0: Adt98, v1: Adt129) -> Adt121 {
    let m514 = v1;
    match m514 {
        Adt129::C0 => {
            f148()
        }
        Adt129::C1 => {
            Adt121::C0(None::<Str>, Some(Adt97::C6(v0)))
        }
        Adt129::C2 => {
            Adt121::C0(None::<Str>, Some(Adt97::C7(v0)))
        }
        Adt129::C3(v2) => {
            let m515 = f197(v2);
            match m515 {
                None => {
                    Adt121::C0(None::<Str>, Some(Adt97::C6(v0)))
                }
                Some(v3) => {
                    Adt121::C0(Some(v3), None::<Adt97>)
                }
            }
        }
    }
}

pub fn f142(v0: List<Adt86>, v1: List<u64>) -> R<Adt129> {
    let m516 = f198(v0, v1, Adt130::C0)?;
    match m516 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v2) => {
            let m517 = f199(v2);
            match m517 {
                None => {
                    Ok(Adt129::C1)
                }
                Some(v3) => {
                    Ok(Adt129::C3(v3))
                }
            }
        }
        Adt130::C2 => {
            Ok(Adt129::C2)
        }
    }
}

pub fn f143() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f144() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f145(v0: Str) -> bool {
    let m518 = f200(v0);
    match m518 {
        Adt138::C0 => {
            false
        }
        Adt138::C1 => {
            true
        }
        Adt138::C2 => {
            true
        }
        Adt138::C3 => {
            true
        }
    }
}

pub fn f146(v0: Adt129) -> R<Adt129> {
    let m519 = v0.clone();
    match m519 {
        Adt129::C0 => {
            Ok(v0)
        }
        Adt129::C1 => {
            Ok(v0)
        }
        Adt129::C2 => {
            Ok(v0)
        }
        Adt129::C3(v1) => {
            let m520 = f201(v1);
            match m520 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m521 = f202(v2)?;
                    match m521 {
                        None => {
                            Ok(Adt129::C1)
                        }
                        Some(v3) => {
                            Ok(Adt129::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f147() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f148() -> Adt121 {
    Adt121::C0(None::<Str>, None::<Adt97>)
}

pub fn f149(v0: Option<Adt97>, v1: Option<Adt97>) -> Option<Adt97> {
    let m522 = v0.clone();
    match m522 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f150(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt99>> {
    Ok(f203(f204(v0, v1, v2, List::<Adt99>::nil())?, List::<Adt99>::nil()))
}

pub fn f151(v0: Adt6) -> Option<Str> {
    let m523 = v0;
    match m523 {
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

pub fn f152() -> Str {
    Str::lit("2026-07-28")
}

pub fn f153(v0: Adt7) -> R<Option<i64>> {
    let m526 = f205(40u64, match v0.clone() {
        Adt7::C0(h524, _) => {
            h524
        }
    }, match v0 {
        Adt7::C0(_, h525) => {
            h525
        }
    })?;
    match m526 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f206(v1))
        }
    }
}

pub fn f154(v0: Str, p532: List<(Str, Adt6)>) -> Adt123 {
    let m531 = p532;
    {
        let mut m531 = m531;
        loop {
            let step = {
                let v1 = m531;
                let m527 = v1;
                match m527.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m529 = f207({
                            let (h528, _) = v2;
                            h528
                        }, v0.clone());
                        match m529 {
                            Adt139::C0 => {
                                Err::<List<(Str, Adt6)>, Adt123>(Adt123::C1({
                                    let (_, h530) = v2;
                                    h530
                                }))
                            }
                            Adt139::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt123>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Str, v1: Adt5) -> bool {
    f208(f111(v0, v1))
}

pub fn f156(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m533 = f111(Str::lit("method"), v1.clone());
    match m533 {
        Adt123::C0 => {
            f81(Some(v0), f112(v1))
        }
        Adt123::C1(v2) => {
            let m534 = f151(v2);
            match m534 {
                None => {
                    f81(Some(v0), Adt93::C4)
                }
                Some(v3) => {
                    f209(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f157(v0: u64) -> R<u64> {
    let a537 = {
        let a535 = v0;
        let a536 = 3u64;
        nat_mul(a535, a536)?
    };
    let a538 = 1u64;
    nat_add(a537, a538)
}

pub fn f158(v0: u64, v1: u64, v2: u64) -> bool {
    let a539 = v1;
    let a540 = v0;
    if nat_le(a539, a540) {
        let a541 = v0;
        let a542 = v2;
        nat_le(a541, a542)
    } else {
        false
    }
}

pub fn f159(v0: Adt106) -> R<Adt131> {
    if f210(match v0.clone() {
        Adt106::C0(_, h543, _, _, _, _, _) => {
            h543
        }
    }) {
        let v1: i64 = {
            let a549 = if match v0.clone() {
                Adt106::C0(_, _, _, _, _, h544, _) => {
                    h544
                }
            } {
                let a546 = match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h545) => {
                        h545
                    }
                };
                int_neg(a546)?
            } else {
                match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h547) => {
                        h547
                    }
                }
            };
            let a550 = match v0.clone() {
                Adt106::C0(_, _, _, _, h548, _, _) => {
                    h548
                }
            };
            int_sub(a549, a550)?
        };
        let m553 = {
            let a551 = v1;
            let a552 = -9999i64;
            compare(a551, a552)
        };
        if if match m553 {
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
            let m556 = {
                let a554 = 9999i64;
                let a555 = v1;
                compare(a554, a555)
            };
            match m556 {
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
            Ok(Adt131::C1)
        } else {
            Ok(Adt131::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt106::C0(h557, _, _, _, _, _, _) => {
                    h557
                }
            } {
                let a559 = match v0 {
                    Adt106::C0(_, _, h558, _, _, _, _) => {
                        h558
                    }
                };
                int_neg(a559)?
            } else {
                match v0 {
                    Adt106::C0(_, _, h560, _, _, _, _) => {
                        h560
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt131::C0)
    }
}

pub fn f160(v0: Adt6, v1: List<Adt108>, v2: u64) -> Adt103 {
    let m561 = v1.clone();
    match m561.uncons() {
        None => {
            Adt103::C0(Adt104::C9(v0), List::<Adt108>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m562 = v3;
            match m562 {
                Adt108::C0(v5) => {
                    Adt103::C0(Adt104::C2, List::cons(Adt108::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt108::C1(v6, v7) => {
                    let m563 = v7;
                    match m563 {
                        None => {
                            Adt103::C0(Adt104::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt103::C0(Adt104::C2, List::cons(Adt108::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(v0: u64) -> bool {
    let a564 = v0;
    let a565 = 32u64;
    if nat_eq(a564, a565) {
        true
    } else {
        let a566 = v0;
        let a567 = 10u64;
        if nat_eq(a566, a567) {
            true
        } else {
            let a568 = v0;
            let a569 = 13u64;
            if nat_eq(a568, a569) {
                true
            } else {
                let a570 = v0;
                let a571 = 9u64;
                nat_eq(a570, a571)
            }
        }
    }
}

pub fn f162(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let a572 = v2;
    let a573 = 34u64;
    if nat_eq(a572, a573) {
        Ok(f166(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), false)))
    } else {
        let a574 = v2;
        let a575 = 123u64;
        if nat_eq(a574, a575) {
            f211(v0, Adt108::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt104::C3, v1)
        } else {
            let a576 = v2;
            let a577 = 91u64;
            if nat_eq(a576, a577) {
                f211(v0, Adt108::C0(List::<Adt6>::nil()), Adt104::C1, v1)
            } else {
                let a578 = v2;
                let a579 = 116u64;
                if nat_eq(a578, a579) {
                    Ok(f166(v0, Adt104::C7(0u64, 1u64)))
                } else {
                    let a580 = v2;
                    let a581 = 102u64;
                    if nat_eq(a580, a581) {
                        Ok(f166(v0, Adt104::C7(1u64, 1u64)))
                    } else {
                        let a582 = v2;
                        let a583 = 110u64;
                        if nat_eq(a582, a583) {
                            Ok(f166(v0, Adt104::C7(2u64, 1u64)))
                        } else {
                            let a584 = v2;
                            let a585 = 45u64;
                            if nat_eq(a584, a585) {
                                Ok(f166(v0, f212(true, Adt107::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a586 = v2;
                                let a587 = 48u64;
                                if nat_eq(a586, a587) {
                                    Ok(f166(v0, f212(false, Adt107::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f158(v2, 49u64, 57u64) {
                                        Ok(f166(v0, f212(false, Adt107::C2, f213({
                                            let a588 = v2;
                                            let a589 = 48u64;
                                            nat_sub(a588, a589)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f167(Adt90::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f163(v0: Adt103, v1: u64) -> Adt124 {
    let m591 = match v0.clone() {
        Adt103::C0(_, h590, _) => {
            h590
        }
    };
    match m591.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m592 = v2;
            match m592 {
                Adt108::C0(v4) => {
                    Adt124::C1(f160(Adt6::C4(f214(v4, List::<Adt6>::nil())), v3, {
                        let a594 = match v0 {
                            Adt103::C0(_, _, h593) => {
                                h593
                            }
                        };
                        let a595 = 1u64;
                        nat_sub(a594, a595)
                    }))
                }
                Adt108::C1(_, _) => {
                    f167(Adt90::C2(v1))
                }
            }
        }
    }
}

pub fn f164(v0: Adt103, v1: u64, v2: u64) -> Adt124 {
    let m597 = match v0.clone() {
        Adt103::C0(h596, _, _) => {
            h596
        }
    };
    match m597 {
        Adt104::C0 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C1 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C2 => {
            if f161(v2) {
                Adt124::C1(v0)
            } else {
                let a598 = v2;
                let a599 = 44u64;
                if nat_eq(a598, a599) {
                    f215(v0, v1)
                } else {
                    let a600 = v2;
                    let a601 = 93u64;
                    if nat_eq(a600, a601) {
                        f163(v0, v1)
                    } else {
                        let a602 = v2;
                        let a603 = 125u64;
                        if nat_eq(a602, a603) {
                            f165(v0, v1)
                        } else {
                            f167(Adt90::C2(v1))
                        }
                    }
                }
            }
        }
        Adt104::C3 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C4 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C5 => {
            f167(Adt90::C2(v1))
        }
        Adt104::C6(_, _, _) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C7(_, _) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C8(_) => {
            f167(Adt90::C2(v1))
        }
        Adt104::C9(_) => {
            if f161(v2) {
                Adt124::C1(v0)
            } else {
                f167(Adt90::C10(v1))
            }
        }
    }
}

pub fn f165(v0: Adt103, v1: u64) -> Adt124 {
    let m605 = match v0.clone() {
        Adt103::C0(_, h604, _) => {
            h604
        }
    };
    match m605.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m606 = v2;
            match m606 {
                Adt108::C0(_) => {
                    f167(Adt90::C2(v1))
                }
                Adt108::C1(v5, _) => {
                    Adt124::C1(f160(Adt6::C5(f216(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a608 = match v0 {
                            Adt103::C0(_, _, h607) => {
                                h607
                            }
                        };
                        let a609 = 1u64;
                        nat_sub(a608, a609)
                    }))
                }
            }
        }
    }
}

pub fn f166(v0: Adt103, v1: Adt104) -> Adt124 {
    Adt124::C1(f217(v0, v1))
}

pub fn f167(v0: Adt90) -> Adt124 {
    Adt124::C0(v0)
}

pub fn f168(v0: Adt103, v1: u64, v2: u64, v3: Adt105, v4: List<u64>, v5: bool) -> R<Adt124> {
    let m610 = v3;
    match m610 {
        Adt105::C0 => {
            let a611 = v2;
            let a612 = 34u64;
            if nat_eq(a611, a612) {
                Ok(f218(v0, v1, v4, v5))
            } else {
                let a613 = v2;
                let a614 = 92u64;
                if nat_eq(a613, a614) {
                    Ok(f166(v0, Adt104::C6(Adt105::C1, v4, v5)))
                } else {
                    let a615 = v2;
                    let a616 = 32u64;
                    if nat_lt(a615, a616) {
                        Ok(f167(Adt90::C4(v1)))
                    } else {
                        Ok(f166(v0, Adt104::C6(Adt105::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt105::C1 => {
            let a617 = v2;
            let a618 = 117u64;
            if nat_eq(a617, a618) {
                Ok(f166(v0, Adt104::C6(Adt105::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a619 = f219(v2);
                let a620 = 0u64;
                if nat_eq(a619, a620) {
                    Ok(f167(Adt90::C5(v1)))
                } else {
                    Ok(f166(v0, Adt104::C6(Adt105::C0, List::cons(f219(v2), v4), v5)))
                }
            }
        }
        Adt105::C2(v6, v7, v8) => {
            let a621 = 15u64;
            let a622 = f220(v2);
            if nat_lt(a621, a622) {
                Ok(f167(Adt90::C5(v1)))
            } else {
                let a625 = {
                    let a623 = v6;
                    let a624 = 1u64;
                    nat_add(a623, a624)?
                };
                let a626 = 4u64;
                if nat_eq(a625, a626) {
                    f221(v0, v1, {
                        let a629 = {
                            let a627 = v7;
                            let a628 = 16u64;
                            nat_mul(a627, a628)?
                        };
                        let a630 = f220(v2);
                        nat_add(a629, a630)?
                    }, v8, v4, v5)
                } else {
                    Ok(f166(v0, Adt104::C6(Adt105::C2({
                        let a631 = v6;
                        let a632 = 1u64;
                        nat_add(a631, a632)?
                    }, {
                        let a635 = {
                            let a633 = v7;
                            let a634 = 16u64;
                            nat_mul(a633, a634)?
                        };
                        let a636 = f220(v2);
                        nat_add(a635, a636)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt105::C3(v9) => {
            let a637 = v2;
            let a638 = 92u64;
            if nat_eq(a637, a638) {
                Ok(f166(v0, Adt104::C6(Adt105::C4(v9), v4, v5)))
            } else {
                Ok(f167(Adt90::C6(v1)))
            }
        }
        Adt105::C4(v10) => {
            let a639 = v2;
            let a640 = 117u64;
            if nat_eq(a639, a640) {
                Ok(f166(v0, Adt104::C6(Adt105::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f167(Adt90::C6(v1)))
            }
        }
    }
}

pub fn f169(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt124> {
    let a641 = v2;
    let a642 = f222(v3, v4);
    if nat_eq(a641, a642) {
        let a646 = {
            let a643 = v4;
            let a644 = 1u64;
            nat_add(a643, a644)?
        };
        let a647 = {
            let a645 = f223(v3);
            length_list(a645)
        };
        if nat_eq(a646, a647) {
            Ok(f224(v0, f225(v3)))
        } else {
            Ok(f166(v0, Adt104::C7(v3, {
                let a648 = v4;
                let a649 = 1u64;
                nat_add(a648, a649)?
            })))
        }
    } else {
        Ok(f167(Adt90::C2(v1)))
    }
}

pub fn f170(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m651 = match v3.clone() {
        Adt106::C0(_, h650, _, _, _, _, _) => {
            h650
        }
    };
    match m651 {
        Adt107::C0 => {
            let a652 = v2;
            let a653 = 48u64;
            if nat_eq(a652, a653) {
                Ok(f226(v0, v3, Adt107::C1))
            } else {
                if f158(v2, 49u64, 57u64) {
                    f227(v0, v1, v2, v3, false)
                } else {
                    Ok(f167(Adt90::C7(v1)))
                }
            }
        }
        Adt107::C1 => {
            let a654 = v2;
            let a655 = 46u64;
            if nat_eq(a654, a655) {
                Ok(f226(v0, v3, Adt107::C3))
            } else {
                if f228(v2) {
                    Ok(f226(v0, v3, Adt107::C5))
                } else {
                    if f229(v2) {
                        Ok(f167(Adt90::C7(v1)))
                    } else {
                        f230(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C2 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, false)
            } else {
                let a656 = v2;
                let a657 = 46u64;
                if nat_eq(a656, a657) {
                    Ok(f226(v0, v3, Adt107::C3))
                } else {
                    if f228(v2) {
                        Ok(f226(v0, v3, Adt107::C5))
                    } else {
                        f230(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C3 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, true)
            } else {
                Ok(f167(Adt90::C7(v1)))
            }
        }
        Adt107::C4 => {
            if f229(v2) {
                f227(v0, v1, v2, v3, true)
            } else {
                if f228(v2) {
                    Ok(f226(v0, v3, Adt107::C5))
                } else {
                    f230(v0, v1, v2, v3)
                }
            }
        }
        Adt107::C5 => {
            let a658 = v2;
            let a659 = 43u64;
            if nat_eq(a658, a659) {
                Ok(f166(v0, f212(match v3.clone() {
                    Adt106::C0(h660, _, _, _, _, _, _) => {
                        h660
                    }
                }, Adt107::C6, match v3.clone() {
                    Adt106::C0(_, _, h661, _, _, _, _) => {
                        h661
                    }
                }, match v3.clone() {
                    Adt106::C0(_, _, _, h662, _, _, _) => {
                        h662
                    }
                }, match v3 {
                    Adt106::C0(_, _, _, _, h663, _, _) => {
                        h663
                    }
                }, false, 0i64)))
            } else {
                let a664 = v2;
                let a665 = 45u64;
                if nat_eq(a664, a665) {
                    Ok(f166(v0, f212(match v3.clone() {
                        Adt106::C0(h666, _, _, _, _, _, _) => {
                            h666
                        }
                    }, Adt107::C6, match v3.clone() {
                        Adt106::C0(_, _, h667, _, _, _, _) => {
                            h667
                        }
                    }, match v3.clone() {
                        Adt106::C0(_, _, _, h668, _, _, _) => {
                            h668
                        }
                    }, match v3 {
                        Adt106::C0(_, _, _, _, h669, _, _) => {
                            h669
                        }
                    }, true, 0i64)))
                } else {
                    if f229(v2) {
                        f231(v0, v1, v2, Adt106::C0(match v3.clone() {
                            Adt106::C0(h670, _, _, _, _, _, _) => {
                                h670
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, h671, _, _, _, _, _) => {
                                h671
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, h672, _, _, _, _) => {
                                h672
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, _, h673, _, _, _) => {
                                h673
                            }
                        }, match v3 {
                            Adt106::C0(_, _, _, _, h674, _, _) => {
                                h674
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f167(Adt90::C7(v1)))
                    }
                }
            }
        }
        Adt107::C6 => {
            if f229(v2) {
                f231(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h675, _) => {
                        h675
                    }
                })
            } else {
                Ok(f167(Adt90::C7(v1)))
            }
        }
        Adt107::C7 => {
            if f229(v2) {
                f231(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h676, _) => {
                        h676
                    }
                })
            } else {
                f230(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f171(v0: List<u64>) -> bool {
    let m677 = v0.clone();
    match m677.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a681 = if f232(v0.clone(), false) {
                let a679 = {
                    let a678 = v0;
                    length_list(a678)
                };
                let a680 = f233();
                nat_le(a679, a680)
            } else {
                false
            };
            bool_not(a681)
        }
    }
}

pub fn f172(v0: Adt35) -> Option<Adt94> {
    let m686 = f234(match v0.clone() {
        Adt35::C0(h682, _, _, _, _, _, _, _) => {
            h682
        }
    }, {
        let m685 = match v0.clone() {
            Adt35::C0(_, _, _, _, h684, _, _, _) => {
                h684
            }
        };
        match m685 {
            Adt5::C0(h683) => {
                h683
            }
        }
    });
    match m686 {
        None => {
            f235(match v0.clone() {
                Adt35::C0(h687, _, _, _, _, _, _, _) => {
                    h687
                }
            }, {
                let m690 = match v0 {
                    Adt35::C0(_, _, _, _, h689, _, _, _) => {
                        h689
                    }
                };
                match m690 {
                    Adt5::C0(h688) => {
                        h688
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f173(v0: Adt35, v1: Result<Adt132, Adt94>) -> R<Result<List<Adt12>, Adt94>> {
    let m691 = v1;
    match m691 {
        Ok(v3) => {
            let m692 = f236(v0)?;
            match m692 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt94>(f87(match v3 {
                        Adt132::C0(h693, _) => {
                            h693
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt94>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt94>(v2))
        }
    }
}

pub fn f174(p700: u64, v1: Str, v2: Adt6, p701: List<Adt125>, p702: Adt132) -> R<Result<Adt132, Adt94>> {
    let m699 = (p700, (p701, p702));
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v0, (v3, v4)) = m699;
                let m694 = v0;
                if m694 == 0 {
                    Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Err::<Adt132, Adt94>(Adt94::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m694 - 1;
                    let m695 = v3;
                    match m695.uncons() {
                        None => {
                            Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Ok::<Adt132, Adt94>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m696 = f237(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m696 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>((v5, ({
                                        let (h697, _) = v9;
                                        h697
                                    }, {
                                        let (_, h698) = v9;
                                        h698
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Err::<Adt132, Adt94>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m699 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f175() -> u64 {
    67108864u64
}

pub fn f176() -> Adt132 {
    Adt132::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f177(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m703 = f238(v1.clone());
    match m703 {
        None => {
            let m704 = f239(v1);
            match m704 {
                None => {
                    Ok(Adt133::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f240(v0, match v5 {
                        Adt5::C0(h705) => {
                            h705
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt133::C3(List::<Adt112>::nil()))
            } else {
                Ok(Adt133::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f178(v0: Adt112, v1: List<Adt112>) -> R<Adt126> {
    let m706 = v0;
    match m706 {
        Adt112::C0(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C1(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C2(v8, v9, v10, v11) => {
            Ok(f241(v8, v9, v10, {
                let a707 = v11;
                let a708 = 1u64;
                nat_add(a707, a708)?
            }, v1))
        }
        Adt112::C3(v12) => {
            Ok(Adt126::C0(Adt111::C1(v12, Str::lit("not")), v1))
        }
        Adt112::C4(v13, _, v15, v16) => {
            Ok(f242(v13, v15, v16, v1))
        }
        Adt112::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
    }
}

pub fn f179(v0: Adt113, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m709 = f239(v1);
    match m709 {
        None => {
            Adt133::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m710 = v0;
            match m710 {
                Adt113::C0 => {
                    f243(match v4 {
                        Adt5::C0(h711) => {
                            h711
                        }
                    }, v2, v3)
                }
                Adt113::C1 => {
                    f244(match v4 {
                        Adt5::C0(h712) => {
                            h712
                        }
                    }, v2, v3)
                }
                Adt113::C2 => {
                    f245(match v4 {
                        Adt5::C0(h713) => {
                            h713
                        }
                    }, v2, v3)
                }
                Adt113::C3 => {
                    f246(match v4 {
                        Adt5::C0(h714) => {
                            h714
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f180(v0: Adt112, v1: List<Adt112>, v2: List<Adt110>, v3: Str) -> Adt126 {
    let m715 = v0;
    match m715 {
        Adt112::C0(_, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
        Adt112::C1(v7, v8, v9) => {
            f247(v7, v8, v9, v1)
        }
        Adt112::C2(v10, v11, v12, v13) => {
            f241(v10, v11, v12, v13, v1)
        }
        Adt112::C3(_) => {
            Adt126::C0(Adt111::C0, v1)
        }
        Adt112::C4(_, v16, v17, v18) => {
            f242(v16, v17, v18, v1)
        }
        Adt112::C5(_, _, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
    }
}

pub fn f181(p724: List<Adt38>) -> R<bool> {
    let m723 = p724;
    {
        let mut m723 = m723;
        loop {
            let step = {
                let v0 = m723;
                let m716 = v0;
                match m716.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m717 = v1;
                        match m717 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f248(match v4 {
                                    Adt42::C0(h718, _, _, _) => {
                                        h718
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f248(match v5 {
                                    Adt43::C0(h719, _, _, _) => {
                                        h719
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C3(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C4(v7) => {
                                let m721 = match v7 {
                                    Adt45::C0(h720, _, _) => {
                                        h720
                                    }
                                };
                                match m721 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f248(match v9 {
                                            Adt48::C0(_, _, h722, _) => {
                                                h722
                                            }
                                        })? {
                                            Ok::<List<Adt38>, R<bool>>(v2)
                                        } else {
                                            Err::<List<Adt38>, R<bool>>(Ok(false))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f249(match v1.clone() {
        Adt49::C0(h725, _) => {
            h725
        }
    }) {
        f250(v1)
    } else {
        let m727 = match v0 {
            Adt35::C0(_, _, _, _, _, h726, _, _) => {
                h726
            }
        };
        match m727 {
            None => {
                f250(v1)
            }
            Some(v2) => {
                f251(v2, v1)
            }
        }
    }
}

pub fn f183(p730: List<Adt110>, p731: List<Str>) -> R<List<Str>> {
    let m729 = (p730, p731);
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v0, v1) = m729;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Str>), R<List<Str>>>((v3, List::cons(f252(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f184(v0: Adt114, v1: List<Adt114>, v2: List<u64>) -> R<Adt134> {
    let m732 = v0;
    match m732 {
        Adt114::C0(v3) => {
            Ok(Adt134::C1(v1, f129(v3, v2)))
        }
        Adt114::C1(v4, v5) => {
            f253(v4, v5, v1, v2)
        }
        Adt114::C2(v6, v7) => {
            let m733 = v7;
            match m733.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C1(v6, v8), List::cons(Adt114::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt114::C3(v10, v11) => {
            let m734 = v11.clone();
            match m734.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt114::C4(v14, v15) => {
            let m735 = v15;
            match m735.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0({
                        let a737 = f254({
                            let (h736, _) = v16;
                            h736
                        })?;
                        let a738 = List::cons(58u64, List::<u64>::nil());
                        append_list(a737, a738)
                    }), List::cons(Adt114::C1(v14, {
                        let (_, h739) = v16;
                        h739
                    }), List::cons(Adt114::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt114::C5(v18, v19) => {
            let m740 = v19.clone();
            match m740.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f185(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m741 = v0;
    match m741 {
        Adt32::C0(v1) => {
            Ok(Some(f255(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f256(v2)?))
        }
        Adt32::C2(v3) => {
            f257(f258(v3)?)
        }
        Adt32::C3(v4) => {
            f257(f258(match v4 {
                Adt49::C0(h742, _) => {
                    h742
                }
            })?)
        }
        Adt32::C4(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C5(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C6(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C7(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C8(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C9(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f186(v0: Adt116) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f259(f260(match v0.clone() {
        Adt116::C0(h743, _, _) => {
            h743
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f261({
        let m746 = match v0 {
            Adt116::C0(_, h745, _) => {
                h745
            }
        };
        match m746 {
            Adt117::C0(h744, _) => {
                h744
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f187(v0: Adt118) -> R<Option<List<u64>>> {
    let m747 = f262(f263(v0))?;
    match m747 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m748 = f73(v1)?;
            match m748 {
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

pub fn f188() -> u64 {
    404u64
}

pub fn f189(v0: Adt84) -> u64 {
    let m749 = v0;
    match m749 {
        Adt84::C0(_) => {
            f49()
        }
        Adt84::C1(_) => {
            f49()
        }
        Adt84::C2(_) => {
            f49()
        }
        Adt84::C3(_) => {
            f49()
        }
        Adt84::C4(_) => {
            f49()
        }
        Adt84::C5(_) => {
            f49()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f264()
        }
        Adt84::C8(_) => {
            f49()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f190(v0: Adt119) -> i64 {
    let m750 = v0;
    match m750 {
        Adt119::C0(_) => {
            f265()
        }
        Adt119::C1(_, _) => {
            f266()
        }
        Adt119::C2(_) => {
            f267()
        }
        Adt119::C3(_) => {
            f268()
        }
        Adt119::C4(v6) => {
            f269(v6)
        }
    }
}

pub fn f191(v0: Adt119) -> Str {
    let m751 = v0;
    match m751 {
        Adt119::C0(v1) => {
            f270(v1)
        }
        Adt119::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt119::C2(v4) => {
            f271(v4)
        }
        Adt119::C3(_) => {
            Str::lit("Method not found")
        }
        Adt119::C4(v6) => {
            let m752 = v6;
            match m752 {
                Adt84::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt84::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt84::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt84::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt84::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt84::C5(_) => {
                    Str::lit("Invalid params: invalid cursor")
                }
                Adt84::C6(_) => {
                    Str::lit("Internal error")
                }
                Adt84::C7(_) => {
                    Str::lit("Rate limit exceeded: too many tool calls")
                }
                Adt84::C8(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt84::C9(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f192(v0: Adt119) -> R<Option<Adt6>> {
    let m753 = v0;
    match m753 {
        Adt119::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f272(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt119::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C4(v6) => {
            let m754 = v6;
            match m754 {
                Adt84::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C2(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C4(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f89(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f273(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f193(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127) -> R<Result<Adt116, Adt119>> {
    f274(v0.clone(), v1, v2, v3.clone(), f275(match v0 {
        Adt0::C0(_, _, h755, _, _, _) => {
            h755
        }
    }, match v3 {
        Adt127::C0(_, h756, _, _) => {
            h756
        }
    }))
}

pub fn f194(v0: Adt135, v1: Adt136) -> Result<Adt127, Adt119> {
    let m757 = v1;
    match m757 {
        Adt136::C0(v3) => {
            f276(v3, v0)
        }
        Adt136::C1(v2) => {
            Err::<Adt127, Adt119>(v2)
        }
    }
}

pub fn f195(v0: Str) -> Adt135 {
    let a758 = v0.clone();
    let a759 = Str::lit("server/discover");
    if equal(a758, a759) {
        Adt135::C1(Adt128::C0)
    } else {
        let a760 = v0.clone();
        let a761 = Str::lit("tools/list");
        if equal(a760, a761) {
            Adt135::C1(Adt128::C1)
        } else {
            let a762 = v0.clone();
            let a763 = Str::lit("tools/call");
            if equal(a762, a763) {
                Adt135::C1(Adt128::C2)
            } else {
                let a764 = v0.clone();
                let a765 = Str::lit("resources/list");
                if equal(a764, a765) {
                    Adt135::C1(Adt128::C3)
                } else {
                    let a766 = v0.clone();
                    let a767 = Str::lit("resources/read");
                    if equal(a766, a767) {
                        Adt135::C1(Adt128::C4)
                    } else {
                        let a768 = v0.clone();
                        let a769 = Str::lit("resources/templates/list");
                        if equal(a768, a769) {
                            Adt135::C1(Adt128::C5)
                        } else {
                            let a770 = v0.clone();
                            let a771 = Str::lit("prompts/list");
                            if equal(a770, a771) {
                                Adt135::C1(Adt128::C6)
                            } else {
                                let a772 = v0.clone();
                                let a773 = Str::lit("prompts/get");
                                if equal(a772, a773) {
                                    Adt135::C1(Adt128::C7)
                                } else {
                                    let a774 = v0.clone();
                                    let a775 = Str::lit("completion/complete");
                                    if equal(a774, a775) {
                                        Adt135::C1(Adt128::C8)
                                    } else {
                                        let a776 = v0.clone();
                                        let a777 = Str::lit("tasks/get");
                                        if equal(a776, a777) {
                                            Adt135::C1(Adt128::C9)
                                        } else {
                                            let a778 = v0.clone();
                                            let a779 = Str::lit("tasks/update");
                                            if equal(a778, a779) {
                                                Adt135::C1(Adt128::C10)
                                            } else {
                                                let a780 = v0;
                                                let a781 = Str::lit("tasks/cancel");
                                                if equal(a780, a781) {
                                                    Adt135::C1(Adt128::C11)
                                                } else {
                                                    Adt135::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Adt136> {
    f277(v0, v1, v2.clone(), f278(match v2 {
        Adt95::C0(_, _, h782) => {
            h782
        }
    }))
}

pub fn f197(v0: List<u64>) -> Option<Str> {
    let a783 = f279(f280(), v0);
    utf8_decode(a783)
}

pub fn f198(p788: List<Adt86>, v1: List<u64>, p789: Adt130) -> R<Adt130> {
    let m787 = (p788, p789);
    {
        let mut m787 = m787;
        loop {
            let step = {
                let (v0, v2) = m787;
                let m784 = v0;
                match m784.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt130), R<Adt130>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f281(match v3.clone() {
                            Adt86::C0(h785, _) => {
                                h785
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, f282(v2, match v3 {
                                Adt86::C0(_, h786) => {
                                    h786
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, v2))
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

pub fn f199(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f283(v0);
    if f284(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f200(v0: Str) -> Adt138 {
    let m790 = f195(v0);
    match m790 {
        Adt135::C0 => {
            Adt138::C0
        }
        Adt135::C1(v1) => {
            f285(v1)
        }
    }
}

pub fn f201(v0: List<u64>) -> Option<List<u64>> {
    let m791 = f286(f287(), v0);
    match m791 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m792 = f286(f288(), f289(v1, List::<u64>::nil()));
            match m792 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f289(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f202(v0: List<u64>) -> R<Option<List<u64>>> {
    let m794 = f290({
        let a793 = v0.clone();
        length_list(a793)
    }, v0, List::<u64>::nil())?;
    match m794 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f129(v1, List::<u64>::nil())))
        }
    }
}

pub fn f203(p797: List<Adt99>, p798: List<Adt99>) -> List<Adt99> {
    let m796 = (p797, p798);
    {
        let mut m796 = m796;
        loop {
            let step = {
                let (v0, v1) = m796;
                let m795 = v0;
                match m795.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Adt99>), List<Adt99>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: List<Adt86>, p805: List<Adt12>, v2: Option<Str>, p806: List<Adt99>) -> R<List<Adt99>> {
    let m804 = (p805, p806);
    {
        let mut m804 = m804;
        loop {
            let step = {
                let (v1, v3) = m804;
                let m799 = v1;
                match m799.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f291(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, List::cons(Adt99::C0(match v4.clone() {
                                Adt12::C0(_, h800, _, _) => {
                                    h800
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h801, _) => {
                                    h801
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h802) => {
                                    h802
                                }
                            }, f292(v0.clone(), match v4 {
                                Adt12::C0(_, h803, _, _) => {
                                    h803
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m804 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(p822: u64, p823: i64, p824: i64) -> R<Option<i64>> {
    let m821 = (p822, (p823, p824));
    {
        let mut m821 = m821;
        loop {
            let step = {
                let (v0, (v1, v2)) = m821;
                let m807 = v0;
                if m807 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m807 - 1;
                    if f293(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m810 = {
                            let a808 = v2;
                            let a809 = 0i64;
                            compare(a808, a809)
                        };
                        if match m810 {
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
                            if f293({
                                let a811 = v1;
                                let a812 = 10i64;
                                let a813 = 0i64;
                                int_rem(a811, a812, a813)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a814 = v1;
                                    let a815 = 10i64;
                                    let a816 = 0i64;
                                    int_quot(a814, a815, a816)?
                                }, f123(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a817 = v1;
                                let a818 = 10i64;
                                int_mul(a817, a818)?
                            }, {
                                let a819 = v2;
                                let a820 = 1i64;
                                int_sub(a819, a820)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: i64) -> Option<i64> {
    let m827 = {
        let a825 = 9007199254740991i64;
        let a826 = v0;
        compare(a825, a826)
    };
    if if match m827 {
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
        let m830 = {
            let a828 = v0;
            let a829 = -9007199254740991i64;
            compare(a828, a829)
        };
        match m830 {
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

pub fn f207(v0: Str, v1: Str) -> Adt139 {
    let a831 = v0;
    let a832 = v1;
    if equal(a831, a832) {
        Adt139::C0
    } else {
        Adt139::C1
    }
}

pub fn f208(v0: Adt123) -> bool {
    let m833 = v0;
    match m833 {
        Adt123::C0 => {
            false
        }
        Adt123::C1(_) => {
            true
        }
    }
}

pub fn f209(v0: Adt91, v1: Str, v2: Adt5) -> Result<Adt95, Adt92> {
    let m834 = f111(Str::lit("params"), v2);
    match m834 {
        Adt123::C0 => {
            Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, None::<Adt5>))
        }
        Adt123::C1(v3) => {
            let m835 = f239(v3);
            match m835 {
                None => {
                    f81(Some(v0), Adt93::C6)
                }
                Some(v4) => {
                    Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f210(v0: Adt107) -> bool {
    let m836 = v0;
    match m836 {
        Adt107::C0 => {
            false
        }
        Adt107::C1 => {
            true
        }
        Adt107::C2 => {
            true
        }
        Adt107::C3 => {
            false
        }
        Adt107::C4 => {
            true
        }
        Adt107::C5 => {
            false
        }
        Adt107::C6 => {
            false
        }
        Adt107::C7 => {
            true
        }
    }
}

pub fn f211(v0: Adt103, v1: Adt108, v2: Adt104, v3: u64) -> R<Adt124> {
    let a838 = match v0.clone() {
        Adt103::C0(_, _, h837) => {
            h837
        }
    };
    let a839 = f294();
    if nat_lt(a838, a839) {
        Ok(Adt124::C1(Adt103::C0(v2, List::cons(v1, match v0.clone() {
            Adt103::C0(_, h840, _) => {
                h840
            }
        }), {
            let a842 = match v0 {
                Adt103::C0(_, _, h841) => {
                    h841
                }
            };
            let a843 = 1u64;
            nat_add(a842, a843)?
        })))
    } else {
        Ok(f167(Adt90::C9(v3)))
    }
}

pub fn f212(v0: bool, v1: Adt107, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt104 {
    Adt104::C8(Adt106::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f213(v0: u64) -> i64 {
    let a844 = v0;
    let a845 = 0u64;
    if nat_eq(a844, a845) {
        0i64
    } else {
        let a846 = v0;
        let a847 = 1u64;
        if nat_eq(a846, a847) {
            1i64
        } else {
            let a848 = v0;
            let a849 = 2u64;
            if nat_eq(a848, a849) {
                2i64
            } else {
                let a850 = v0;
                let a851 = 3u64;
                if nat_eq(a850, a851) {
                    3i64
                } else {
                    let a852 = v0;
                    let a853 = 4u64;
                    if nat_eq(a852, a853) {
                        4i64
                    } else {
                        let a854 = v0;
                        let a855 = 5u64;
                        if nat_eq(a854, a855) {
                            5i64
                        } else {
                            let a856 = v0;
                            let a857 = 6u64;
                            if nat_eq(a856, a857) {
                                6i64
                            } else {
                                let a858 = v0;
                                let a859 = 7u64;
                                if nat_eq(a858, a859) {
                                    7i64
                                } else {
                                    let a860 = v0;
                                    let a861 = 8u64;
                                    if nat_eq(a860, a861) {
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

pub fn f214(p864: List<Adt6>, p865: List<Adt6>) -> List<Adt6> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v0, v1) = m863;
                let m862 = v0;
                match m862.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f215(v0: Adt103, v1: u64) -> Adt124 {
    let m867 = match v0.clone() {
        Adt103::C0(_, h866, _) => {
            h866
        }
    };
    match m867.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v2, _)) => {
            let m868 = v2;
            match m868 {
                Adt108::C0(_) => {
                    f166(v0, Adt104::C0)
                }
                Adt108::C1(_, _) => {
                    f166(v0, Adt104::C4)
                }
            }
        }
    }
}

pub fn f216(p871: List<(Str, Adt6)>, p872: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m870 = (p871, p872);
    {
        let mut m870 = m870;
        loop {
            let step = {
                let (v0, v1) = m870;
                let m869 = v0;
                match m869.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m870 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Adt103, v1: Adt104) -> Adt103 {
    Adt103::C0(v1, match v0.clone() {
        Adt103::C0(_, h873, _) => {
            h873
        }
    }, match v0 {
        Adt103::C0(_, _, h874) => {
            h874
        }
    })
}

pub fn f218(v0: Adt103, v1: u64, v2: List<u64>, v3: bool) -> Adt124 {
    let m876 = {
        let a875 = f279(f280(), f129(v2, List::<u64>::nil()));
        utf8_decode(a875)
    };
    match m876 {
        None => {
            f167(Adt90::C14)
        }
        Some(v4) => {
            if v3 {
                f295(v0, v1, v4)
            } else {
                f224(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f219(v0: u64) -> u64 {
    let a877 = v0;
    let a878 = 34u64;
    if nat_eq(a877, a878) {
        34u64
    } else {
        let a879 = v0;
        let a880 = 92u64;
        if nat_eq(a879, a880) {
            92u64
        } else {
            let a881 = v0;
            let a882 = 47u64;
            if nat_eq(a881, a882) {
                47u64
            } else {
                let a883 = v0;
                let a884 = 98u64;
                if nat_eq(a883, a884) {
                    8u64
                } else {
                    let a885 = v0;
                    let a886 = 102u64;
                    if nat_eq(a885, a886) {
                        12u64
                    } else {
                        let a887 = v0;
                        let a888 = 110u64;
                        if nat_eq(a887, a888) {
                            10u64
                        } else {
                            let a889 = v0;
                            let a890 = 114u64;
                            if nat_eq(a889, a890) {
                                13u64
                            } else {
                                let a891 = v0;
                                let a892 = 116u64;
                                if nat_eq(a891, a892) {
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

pub fn f220(v0: u64) -> u64 {
    let a893 = v0;
    let a894 = 48u64;
    if nat_lt(a893, a894) {
        16u64
    } else {
        let a895 = v0;
        let a896 = 58u64;
        if nat_lt(a895, a896) {
            let a897 = v0;
            let a898 = 48u64;
            nat_sub(a897, a898)
        } else {
            let a899 = v0;
            let a900 = 65u64;
            if nat_lt(a899, a900) {
                16u64
            } else {
                let a901 = v0;
                let a902 = 71u64;
                if nat_lt(a901, a902) {
                    let a903 = v0;
                    let a904 = 55u64;
                    nat_sub(a903, a904)
                } else {
                    let a905 = v0;
                    let a906 = 97u64;
                    if nat_lt(a905, a906) {
                        16u64
                    } else {
                        let a907 = v0;
                        let a908 = 103u64;
                        if nat_lt(a907, a908) {
                            let a909 = v0;
                            let a910 = 87u64;
                            nat_sub(a909, a910)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f221(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt124> {
    let a911 = v3;
    let a912 = 0u64;
    if nat_eq(a911, a912) {
        if f158(v2, 55296u64, 56319u64) {
            Ok(f166(v0, Adt104::C6(Adt105::C3(v2), v4, v5)))
        } else {
            if f158(v2, 56320u64, 57343u64) {
                Ok(f167(Adt90::C6(v1)))
            } else {
                Ok(f166(v0, Adt104::C6(Adt105::C0, f296(v2, v4)?, v5)))
            }
        }
    } else {
        if f158(v2, 56320u64, 57343u64) {
            Ok(f166(v0, Adt104::C6(Adt105::C0, f296({
                let a921 = 65536u64;
                let a922 = {
                    let a919 = {
                        let a915 = {
                            let a913 = v3;
                            let a914 = 55296u64;
                            nat_sub(a913, a914)
                        };
                        let a916 = 1024u64;
                        nat_mul(a915, a916)?
                    };
                    let a920 = {
                        let a917 = v2;
                        let a918 = 56320u64;
                        nat_sub(a917, a918)
                    };
                    nat_add(a919, a920)?
                };
                nat_add(a921, a922)?
            }, v4)?, v5)))
        } else {
            Ok(f167(Adt90::C6(v1)))
        }
    }
}

pub fn f222(v0: u64, v1: u64) -> u64 {
    let m925 = {
        let a923 = f223(v0);
        let a924 = v1;
        index_list(a923, a924)
    };
    match m925 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f223(v0: u64) -> List<u64> {
    let a926 = v0;
    let a927 = 0u64;
    if nat_eq(a926, a927) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a928 = v0;
        let a929 = 1u64;
        if nat_eq(a928, a929) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f224(v0: Adt103, v1: Adt6) -> Adt124 {
    Adt124::C1(f160(v1, match v0.clone() {
        Adt103::C0(_, h930, _) => {
            h930
        }
    }, match v0 {
        Adt103::C0(_, _, h931) => {
            h931
        }
    }))
}

pub fn f225(v0: u64) -> Adt6 {
    let a932 = v0;
    let a933 = 0u64;
    if nat_eq(a932, a933) {
        Adt6::C1(true)
    } else {
        let a934 = v0;
        let a935 = 1u64;
        if nat_eq(a934, a935) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f226(v0: Adt103, v1: Adt106, v2: Adt107) -> Adt124 {
    f166(v0, Adt104::C8(Adt106::C0(match v1.clone() {
        Adt106::C0(h936, _, _, _, _, _, _) => {
            h936
        }
    }, v2, match v1.clone() {
        Adt106::C0(_, _, h937, _, _, _, _) => {
            h937
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, h938, _, _, _) => {
            h938
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, h939, _, _) => {
            h939
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, _, h940, _) => {
            h940
        }
    }, match v1 {
        Adt106::C0(_, _, _, _, _, _, h941) => {
            h941
        }
    })))
}

pub fn f227(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: u64 = {
        let a943 = match v3.clone() {
            Adt106::C0(_, _, _, h942, _, _, _) => {
                h942
            }
        };
        let a944 = 0u64;
        if if nat_eq(a943, a944) {
            let a945 = v2;
            let a946 = 48u64;
            nat_eq(a945, a946)
        } else {
            false
        } {
            0u64
        } else {
            let a948 = match v3.clone() {
                Adt106::C0(_, _, _, h947, _, _, _) => {
                    h947
                }
            };
            let a949 = 1u64;
            nat_add(a948, a949)?
        }
    };
    let v6: i64 = if v4 {
        f123(match v3.clone() {
            Adt106::C0(_, _, _, _, h950, _, _) => {
                h950
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt106::C0(_, _, _, _, h951, _, _) => {
                h951
            }
        }
    };
    let a952 = f297();
    let a953 = v5;
    if nat_lt(a952, a953) {
        Ok(f167(Adt90::C8(v1)))
    } else {
        let m956 = {
            let a954 = 99999i64;
            let a955 = v6;
            compare(a954, a955)
        };
        if match m956 {
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
            Ok(f167(Adt90::C8(v1)))
        } else {
            Ok(f166(v0, f212(match v3.clone() {
                Adt106::C0(h957, _, _, _, _, _, _) => {
                    h957
                }
            }, if v4 {
                Adt107::C4
            } else {
                Adt107::C2
            }, f123({
                let a959 = match v3.clone() {
                    Adt106::C0(_, _, h958, _, _, _, _) => {
                        h958
                    }
                };
                let a960 = 10i64;
                int_mul(a959, a960)?
            }, f213({
                let a961 = v2;
                let a962 = 48u64;
                nat_sub(a961, a962)
            }))?, v5, v6, match v3.clone() {
                Adt106::C0(_, _, _, _, _, h963, _) => {
                    h963
                }
            }, match v3 {
                Adt106::C0(_, _, _, _, _, _, h964) => {
                    h964
                }
            })))
        }
    }
}

pub fn f228(v0: u64) -> bool {
    let a965 = v0;
    let a966 = 101u64;
    if nat_eq(a965, a966) {
        true
    } else {
        let a967 = v0;
        let a968 = 69u64;
        nat_eq(a967, a968)
    }
}

pub fn f229(v0: u64) -> bool {
    f158(v0, 48u64, 57u64)
}

pub fn f230(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m969 = f159(v3)?;
    match m969 {
        Adt131::C0 => {
            Ok(f167(Adt90::C7(v1)))
        }
        Adt131::C1 => {
            Ok(f167(Adt90::C8(v1)))
        }
        Adt131::C2(v4) => {
            Ok(f164(f160(v4, match v0.clone() {
                Adt103::C0(_, h970, _) => {
                    h970
                }
            }, match v0 {
                Adt103::C0(_, _, h971) => {
                    h971
                }
            }), v1, v2))
        }
    }
}

pub fn f231(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: i64 = f123({
        let a973 = match v3.clone() {
            Adt106::C0(_, _, _, _, _, _, h972) => {
                h972
            }
        };
        let a974 = 10i64;
        int_mul(a973, a974)?
    }, f213({
        let a975 = v2;
        let a976 = 48u64;
        nat_sub(a975, a976)
    }))?;
    let m979 = {
        let a977 = 99999i64;
        let a978 = v5;
        compare(a977, a978)
    };
    if match m979 {
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
        Ok(f167(Adt90::C8(v1)))
    } else {
        Ok(f166(v0, f212(match v3.clone() {
            Adt106::C0(h980, _, _, _, _, _, _) => {
                h980
            }
        }, Adt107::C7, match v3.clone() {
            Adt106::C0(_, _, h981, _, _, _, _) => {
                h981
            }
        }, match v3.clone() {
            Adt106::C0(_, _, _, h982, _, _, _) => {
                h982
            }
        }, match v3 {
            Adt106::C0(_, _, _, _, h983, _, _) => {
                h983
            }
        }, v4, v5)))
    }
}

pub fn f232(p986: List<u64>, v1: bool) -> bool {
    let m985 = p986;
    {
        let mut m985 = m985;
        loop {
            let step = {
                let v0 = m985;
                let m984 = v0;
                match m984.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f298(v2)
                        } else {
                            f299(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233() -> u64 {
    64u64
}

pub fn f234(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m987 = f154(Str::lit("type"), v1);
    match m987 {
        Adt123::C0 => {
            Some(Adt94::C3(v0))
        }
        Adt123::C1(v2) => {
            let m988 = f151(v2);
            match m988 {
                None => {
                    Some(Adt94::C3(v0))
                }
                Some(v3) => {
                    let a989 = v3;
                    let a990 = Str::lit("object");
                    if equal(a989, a990) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f235(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m991 = f154(Str::lit("$schema"), v1);
    match m991 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v2) => {
            let m992 = f151(v2);
            match m992 {
                None => {
                    Some(Adt94::C2(v0))
                }
                Some(v3) => {
                    let a993 = v3;
                    let a994 = f300();
                    if equal(a993, a994) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f236(v0: Adt35) -> R<Option<Adt94>> {
    let m996 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h995, _, _) => {
            h995
        }
    };
    match m996 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v1) => {
            let m999 = f235(match v0.clone() {
                Adt35::C0(h997, _, _, _, _, _, _, _) => {
                    h997
                }
            }, match v1.clone() {
                Adt5::C0(h998) => {
                    h998
                }
            });
            match m999 {
                None => {
                    let m1003 = f174(f175(), match v0 {
                        Adt35::C0(h1000, _, _, _, _, _, _, _) => {
                            h1000
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1001) => {
                            h1001
                        }
                    }), List::cons(Adt125::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1002) => {
                            h1002
                        }
                    }), false, List::<Str>::nil()), List::<Adt125>::nil()), f176())?;
                    match m1003 {
                        Ok(_) => {
                            Ok(None::<Adt94>)
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

pub fn f237(v0: Str, v1: Adt6, v2: Adt125, v3: Adt132, v4: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1004 = v2;
    match m1004 {
        Adt125::C0(v5, v6, v7) => {
            let m1005 = f238(v5.clone());
            match m1005 {
                None => {
                    let m1006 = f239(v5);
                    match m1006 {
                        None => {
                            Ok(Err::<(List<Adt125>, Adt132), Adt94>(Adt94::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f301(v0, v1, match v9 {
                                Adt5::C0(h1007) => {
                                    h1007
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt125>, Adt132), Adt94>((v4, v3)))
                }
            }
        }
    }
}

pub fn f238(v0: Adt6) -> Option<bool> {
    let m1008 = v0;
    match m1008 {
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

pub fn f239(v0: Adt6) -> Option<Adt5> {
    let m1009 = v0;
    match m1009 {
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

pub fn f240(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m1010 = f302(v1.clone(), v2.clone())?;
    match m1010 {
        Adt133::C0(v4) => {
            Ok(Adt133::C0(v4))
        }
        Adt133::C1(v5) => {
            Ok(Adt133::C1(v5))
        }
        Adt133::C2(v6) => {
            Ok(Adt133::C2(v6))
        }
        Adt133::C3(_) => {
            f303(v0, v1, v2, v3)
        }
    }
}

pub fn f241(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: u64, v4: List<Adt112>) -> Adt126 {
    let m1011 = v0;
    match m1011.uncons() {
        None => {
            let a1012 = v3;
            let a1013 = 1u64;
            if nat_eq(a1012, a1013) {
                Adt126::C0(Adt111::C0, v4)
            } else {
                Adt126::C0(Adt111::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v5, v1.clone(), v2.clone()), List::cons(Adt112::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f242(v0: Option<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1014 = v0;
    match m1014 {
        None => {
            Adt126::C0(Adt111::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f243(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1015 = f154(Str::lit("anyOf"), v0);
    match m1015 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1016 = f304(v3);
            match m1016 {
                None => {
                    Adt133::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1017 = v4;
                    match m1017.uncons() {
                        None => {
                            Adt133::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt133::C3(List::cons(Adt112::C0(v5, v1.clone(), v2.clone()), List::cons(Adt112::C1(v6, v1, v2), List::<Adt112>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f244(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1018 = f154(Str::lit("oneOf"), v0);
    match m1018 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1019 = f304(v3);
            match m1019 {
                None => {
                    Adt133::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1020 = v4;
                    match m1020.uncons() {
                        None => {
                            Adt133::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt133::C3(List::cons(Adt112::C0(v5, v1.clone(), v2.clone()), List::cons(Adt112::C2(v6, v1, v2, 0u64), List::<Adt112>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f245(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1021 = f154(Str::lit("not"), v0);
    match m1021 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1, v2.clone()), List::cons(Adt112::C3(v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f246(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1022 = f154(Str::lit("if"), v0.clone());
    match m1022 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1.clone(), v2.clone()), List::cons(Adt112::C4(f305(Str::lit("then"), v0.clone()), f305(Str::lit("else"), v0), v1, v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f247(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1023 = v0;
    match m1023.uncons() {
        None => {
            Adt126::C0(Adt111::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), List::cons(Adt112::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f248(v0: Str) -> R<bool> {
    let m1025 = f202(f55({
        let a1024 = v0;
        utf8_encode(a1024)
    })?)?;
    match m1025 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f249(v0: Adt37) -> bool {
    let m1027 = match v0 {
        Adt37::C0(_, _, h1026) => {
            h1026
        }
    };
    match m1027 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f250(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1029 = match v0.clone() {
        Adt49::C0(_, h1028) => {
            h1028
        }
    };
    match m1029.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1030, _) => {
                    h1030
                }
            })))
        }
        Some((_, _)) => {
            if f306(match v0.clone() {
                Adt49::C0(_, h1031) => {
                    h1031
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f251(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1035 = {
        let m1034 = match v1.clone() {
            Adt49::C0(h1033, _) => {
                h1033
            }
        };
        match m1034 {
            Adt37::C0(_, h1032, _) => {
                h1032
            }
        }
    };
    match m1035 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1038 = f90(Adt6::C5(match v0.clone() {
                Adt5::C0(h1036) => {
                    h1036
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1037) => {
                    h1037
                }
            }), v2)?;
            match m1038 {
                Adt109::C0 => {
                    f250(v1)
                }
                Adt109::C1(_, _) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt109::C2(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt109::C3(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt109::C4 => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f252(v0: Adt110) -> R<Str> {
    let m1039 = v0;
    match m1039 {
        Adt110::C0(v1) => {
            f307(v1)
        }
        Adt110::C1(v2) => {
            let a1040 = f89(v2, 0i64)?;
            Ok(format_i64(a1040))
        }
    }
}

pub fn f253(v0: u64, v1: Adt6, v2: List<Adt114>, v3: List<u64>) -> R<Adt134> {
    let m1041 = v1;
    match m1041 {
        Adt6::C0 => {
            Ok(Adt134::C1(v2, f129(f308(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt134::C1(v2, f129(if v4 {
                f308(0u64)
            } else {
                f308(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f309(v5.clone())? {
                Ok(Adt134::C1(v2, f129(f310(v5)?, v3)))
            } else {
                Ok(Adt134::C0(Adt90::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt134::C1(v2, f129(f254(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1042 = v0;
            let a1043 = f294();
            if nat_lt(a1042, a1043) {
                Ok(Adt134::C1(f311(91u64, Adt114::C2({
                    let a1044 = v0;
                    let a1045 = 1u64;
                    nat_add(a1044, a1045)?
                }, v7), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1046 = v0;
            let a1047 = f294();
            if nat_lt(a1046, a1047) {
                Ok(Adt134::C1(f311(123u64, Adt114::C4({
                    let a1048 = v0;
                    let a1049 = 1u64;
                    nat_add(a1048, a1049)?
                }, v8), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
    }
}

pub fn f254(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1051 = f129(f312(f55({
            let a1050 = v0;
            utf8_encode(a1050)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1052 = List::cons(34u64, List::<u64>::nil());
        append_list(a1051, a1052)
    }))
}

pub fn f255(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f313(match v0.clone() {
        Adt33::C0(_, h1053, _, _) => {
            h1053
        }
    })), List::cons((Str::lit("capabilities"), f314(match v0.clone() {
        Adt33::C0(_, _, h1054, _) => {
            h1054
        }
    })), {
        let a1057 = f315(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1055) => {
                h1055
            }
        });
        let a1058 = f316(match v0 {
            Adt33::C0(h1056, _, _, _) => {
                h1056
            }
        })?;
        append_list(a1057, a1058)
    })))
}

pub fn f256(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f317(match v0.clone() {
        Adt34::C0(_, _, h1059) => {
            h1059
        }
    }))), {
        let a1062 = f315(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1060, _) => {
                h1060
            }
        });
        let a1063 = f316(match v0 {
            Adt34::C0(h1061, _, _) => {
                h1061
            }
        })?;
        append_list(a1062, a1063)
    }))
}

pub fn f257(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1064 = v0;
    match m1064 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1065 = f262(Adt6::C5(v1))?;
            match m1065 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1066 = f239(v2);
                    match m1066 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1067) => {
                                    h1067
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f258(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1069 = f318(match v0.clone() {
        Adt37::C0(h1068, _, _) => {
            h1068
        }
    })?;
    match m1069 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1072 = f319(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1070, _) => {
                        h1070
                    }
                });
                let a1073 = f320(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1071) => {
                        h1071
                    }
                });
                append_list(a1072, a1073)
            })))
        }
    }
}

pub fn f259(v0: Adt140) -> Str {
    let m1074 = v0;
    match m1074 {
        Adt140::C0 => {
            Str::lit("complete")
        }
        Adt140::C1 => {
            Str::lit("input_required")
        }
        Adt140::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f260(v0: Adt32) -> Adt140 {
    let m1075 = v0;
    match m1075 {
        Adt32::C0(_) => {
            Adt140::C0
        }
        Adt32::C1(_) => {
            Adt140::C0
        }
        Adt32::C2(_) => {
            Adt140::C0
        }
        Adt32::C3(_) => {
            Adt140::C0
        }
        Adt32::C4(_) => {
            Adt140::C0
        }
        Adt32::C5(_) => {
            Adt140::C0
        }
        Adt32::C6(_) => {
            Adt140::C0
        }
        Adt32::C7(_) => {
            Adt140::C0
        }
        Adt32::C8(_) => {
            Adt140::C0
        }
        Adt32::C9(_) => {
            Adt140::C0
        }
        Adt32::C10(_) => {
            Adt140::C0
        }
        Adt32::C11 => {
            Adt140::C0
        }
        Adt32::C12(_) => {
            Adt140::C1
        }
        Adt32::C13(_) => {
            Adt140::C2
        }
    }
}

pub fn f261(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1076 = v0;
    match m1076 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f321(), f322(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f262(v0: Adt6) -> R<Option<Adt6>> {
    f323(f324(), Adt141::C0(v0), List::<Adt142>::nil())
}

pub fn f263(v0: Adt118) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f99())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f325(match v0.clone() {
        Adt118::C0(h1077, _) => {
            h1077
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1080 = match v0.clone() {
            Adt118::C0(_, h1079) => {
                h1079
            }
        };
        match m1080 {
            Adt50::C0(h1078, _, _) => {
                h1078
            }
        }
    })), {
        let a1087 = f326({
            let m1083 = match v0.clone() {
                Adt118::C0(_, h1082) => {
                    h1082
                }
            };
            match m1083 {
                Adt50::C0(_, h1081, _) => {
                    h1081
                }
            }
        });
        let a1088 = f327({
            let m1086 = match v0 {
                Adt118::C0(_, h1085) => {
                    h1085
                }
            };
            match m1086 {
                Adt50::C0(_, _, h1084) => {
                    h1084
                }
            }
        });
        append_list(a1087, a1088)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f264() -> u64 {
    429u64
}

pub fn f265() -> i64 {
    -32602i64
}

pub fn f266() -> i64 {
    -32022i64
}

pub fn f267() -> i64 {
    -32020i64
}

pub fn f268() -> i64 {
    -32601i64
}

pub fn f269(v0: Adt84) -> i64 {
    let m1089 = v0;
    match m1089 {
        Adt84::C0(_) => {
            f265()
        }
        Adt84::C1(_) => {
            f265()
        }
        Adt84::C2(_) => {
            f265()
        }
        Adt84::C3(_) => {
            f265()
        }
        Adt84::C4(_) => {
            f265()
        }
        Adt84::C5(_) => {
            f265()
        }
        Adt84::C6(_) => {
            f328()
        }
        Adt84::C7(_) => {
            f329()
        }
        Adt84::C8(_) => {
            f330()
        }
        Adt84::C9(_) => {
            f328()
        }
    }
}

pub fn f270(v0: Adt120) -> Str {
    let m1090 = v0;
    match m1090 {
        Adt120::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt120::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt120::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt120::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt120::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt120::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt120::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt120::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt120::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f271(v0: Adt97) -> Str {
    let m1091 = v0;
    match m1091 {
        Adt97::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt97::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt97::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt97::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt97::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt97::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt97::C6(v1) => {
            let m1092 = v1;
            match m1092 {
                Adt98::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt98::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt98::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt97::C7(v2) => {
            let m1093 = v2;
            match m1093 {
                Adt98::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt98::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt98::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt97::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt97::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt97::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f272(v0: List<Str>) -> List<Adt6> {
    f214(f331(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f273(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1105 = f332(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1094, _, _, _, _) => {
                h1094
            }
        });
        let a1106 = {
            let a1103 = f332(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1095, _, _, _) => {
                    h1095
                }
            });
            let a1104 = {
                let a1101 = f332(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1096, _, _) => {
                        h1096
                    }
                });
                let a1102 = {
                    let a1099 = f332(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1097, _) => {
                            h1097
                        }
                    });
                    let a1100 = f332(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1098) => {
                            h1098
                        }
                    });
                    append_list(a1099, a1100)
                };
                append_list(a1101, a1102)
            };
            append_list(a1103, a1104)
        };
        append_list(a1105, a1106)
    })
}

pub fn f274(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127, v4: bool) -> R<Result<Adt116, Adt119>> {
    if v4 {
        f333(v0, v1, v2, f334(match v3.clone() {
            Adt127::C0(_, h1107, _, _) => {
                h1107
            }
        }, match v3.clone() {
            Adt127::C0(_, _, h1108, _) => {
                h1108
            }
        }, match v3 {
            Adt127::C0(_, _, _, h1109) => {
                h1109
            }
        }))
    } else {
        Ok(Err::<Adt116, Adt119>(Adt119::C3(f335(match v3 {
            Adt127::C0(_, h1110, _, _) => {
                h1110
            }
        }))))
    }
}

pub fn f275(v0: Adt4, v1: Adt128) -> bool {
    let m1111 = v1;
    match m1111 {
        Adt128::C0 => {
            true
        }
        Adt128::C1 => {
            f336(match v0 {
                Adt4::C0(_, _, _, _, _, h1112, _) => {
                    h1112
                }
            })
        }
        Adt128::C2 => {
            f336(match v0 {
                Adt4::C0(_, _, _, _, _, h1113, _) => {
                    h1113
                }
            })
        }
        Adt128::C3 => {
            f337(match v0 {
                Adt4::C0(_, _, _, _, h1114, _, _) => {
                    h1114
                }
            })
        }
        Adt128::C4 => {
            f337(match v0 {
                Adt4::C0(_, _, _, _, h1115, _, _) => {
                    h1115
                }
            })
        }
        Adt128::C5 => {
            f337(match v0 {
                Adt4::C0(_, _, _, _, h1116, _, _) => {
                    h1116
                }
            })
        }
        Adt128::C6 => {
            f336(match v0 {
                Adt4::C0(_, _, _, h1117, _, _, _) => {
                    h1117
                }
            })
        }
        Adt128::C7 => {
            f336(match v0 {
                Adt4::C0(_, _, _, h1118, _, _, _) => {
                    h1118
                }
            })
        }
        Adt128::C8 => {
            f338(match v0 {
                Adt4::C0(_, _, h1119, _, _, _, _) => {
                    h1119
                }
            })
        }
        Adt128::C9 => {
            f339(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1120) => {
                    h1120
                }
            }, f340())
        }
        Adt128::C10 => {
            f339(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1121) => {
                    h1121
                }
            }, f340())
        }
        Adt128::C11 => {
            f339(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1122) => {
                    h1122
                }
            }, f340())
        }
    }
}

pub fn f276(v0: Adt137, v1: Adt135) -> Result<Adt127, Adt119> {
    let m1123 = v1;
    match m1123 {
        Adt135::C0 => {
            Err::<Adt127, Adt119>(Adt119::C3(match v0 {
                Adt137::C0(_, h1124, _, _) => {
                    h1124
                }
            }))
        }
        Adt135::C1(v2) => {
            Ok::<Adt127, Adt119>(Adt127::C0(match v0.clone() {
                Adt137::C0(h1125, _, _, _) => {
                    h1125
                }
            }, v2, match v0.clone() {
                Adt137::C0(_, _, h1126, _) => {
                    h1126
                }
            }, match v0 {
                Adt137::C0(_, _, _, h1127) => {
                    h1127
                }
            }))
        }
    }
}

pub fn f277(v0: Option<Adt96>, v1: List<Str>, v2: Adt95, v3: Result<Adt16, Adt120>) -> R<Adt136> {
    let m1128 = v3;
    match m1128 {
        Ok(v5) => {
            Ok(f341(v1, v2.clone(), v5.clone(), f342(v0, f200(match v2.clone() {
                Adt95::C0(_, h1129, _) => {
                    h1129
                }
            }), match v2.clone() {
                Adt95::C0(_, h1130, _) => {
                    h1130
                }
            }, match v5 {
                Adt16::C0(h1131, _, _, _, _, _) => {
                    h1131
                }
            }, f343(match v2 {
                Adt95::C0(_, _, h1132) => {
                    h1132
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt136::C1(Adt119::C0(v4)))
        }
    }
}

pub fn f278(v0: Option<Adt5>) -> Result<Adt16, Adt120> {
    let m1133 = v0;
    match m1133 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C0)
        }
        Some(v1) => {
            let m1134 = f111(Str::lit("_meta"), v1);
            match m1134 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C1)
                }
                Adt123::C1(v2) => {
                    let m1135 = f239(v2);
                    match m1135 {
                        None => {
                            Err::<Adt16, Adt120>(Adt120::C4)
                        }
                        Some(v3) => {
                            f344(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1136 = v0;
    let a1137 = f345(f280(), f346(List::<Bytes>::nil(), v1));
    append_bytes(a1136, a1137)
}

pub fn f280() -> Bytes {
    Bytes::lit(&[])
}

pub fn f281(p1144: List<u64>, p1145: List<u64>) -> R<bool> {
    let m1143 = (p1144, p1145);
    {
        let mut m1143 = m1143;
        loop {
            let step = {
                let (v0, v1) = m1143;
                let m1138 = v0;
                match m1138.uncons() {
                    None => {
                        let m1139 = v1;
                        match m1139.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1140 = v1;
                        match m1140.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1141 = f347(v4)?;
                                let a1142 = f347(v6)?;
                                if nat_eq(a1141, a1142) {
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
                Ok(next) => { m1143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: Adt130, v1: List<u64>) -> Adt130 {
    let m1146 = v0;
    match m1146 {
        Adt130::C0 => {
            Adt130::C1(v1)
        }
        Adt130::C1(_) => {
            Adt130::C2
        }
        Adt130::C2 => {
            Adt130::C2
        }
    }
}

pub fn f283(v0: List<u64>) -> List<u64> {
    f289(f348(f289(f348(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f284(p1149: List<u64>) -> bool {
    let m1148 = p1149;
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let v0 = m1148;
                let m1147 = v0;
                match m1147.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f349(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(v0: Adt128) -> Adt138 {
    let m1150 = v0;
    match m1150 {
        Adt128::C0 => {
            Adt138::C0
        }
        Adt128::C1 => {
            Adt138::C0
        }
        Adt128::C2 => {
            Adt138::C1
        }
        Adt128::C3 => {
            Adt138::C0
        }
        Adt128::C4 => {
            Adt138::C2
        }
        Adt128::C5 => {
            Adt138::C0
        }
        Adt128::C6 => {
            Adt138::C0
        }
        Adt128::C7 => {
            Adt138::C1
        }
        Adt128::C8 => {
            Adt138::C0
        }
        Adt128::C9 => {
            Adt138::C3
        }
        Adt128::C10 => {
            Adt138::C3
        }
        Adt128::C11 => {
            Adt138::C3
        }
    }
}

pub fn f286(p1156: List<u64>, p1157: List<u64>) -> Option<List<u64>> {
    let m1155 = (p1156, p1157);
    {
        let mut m1155 = m1155;
        loop {
            let step = {
                let (v0, v1) = m1155;
                let m1151 = v0;
                match m1151.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1152 = v1;
                        match m1152.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1153 = v4;
                                let a1154 = v2;
                                if nat_eq(a1153, a1154) {
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
                Ok(next) => { m1155 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f288() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f289(p1160: List<u64>, p1161: List<u64>) -> List<u64> {
    let m1159 = (p1160, p1161);
    {
        let mut m1159 = m1159;
        loop {
            let step = {
                let (v0, v1) = m1159;
                let m1158 = v0;
                match m1158.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(p1170: u64, p1171: List<u64>, p1172: List<u64>) -> R<Option<List<u64>>> {
    let m1169 = (p1170, (p1171, p1172));
    {
        let mut m1169 = m1169;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1169;
                let m1162 = v0;
                if m1162 == 0 {
                    let m1163 = v1;
                    match m1163.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1162 - 1;
                    let m1164 = v1;
                    match m1164.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1165 = v7;
                            match m1165.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1166 = v9;
                                    match m1166.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1167 = v11;
                                            match m1167.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1168 = f350(v6, v8, v10, v12, f351(v13.clone()))?;
                                                    match m1168 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f129(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f291(v0: Adt12, v1: Option<Str>) -> bool {
    let m1173 = v1;
    match m1173 {
        None => {
            false
        }
        Some(v2) => {
            let a1175 = match v0 {
                Adt12::C0(h1174, _, _, _) => {
                    h1174
                }
            };
            let a1176 = v2;
            equal(a1175, a1176)
        }
    }
}

pub fn f292(v0: List<Adt86>, v1: Str) -> R<Adt100> {
    let m1179 = f146(f142(v0, {
        let a1177 = f352();
        let a1178 = f353(v1)?;
        append_list(a1177, a1178)
    })?)?;
    match m1179 {
        Adt129::C0 => {
            Ok(Adt100::C0)
        }
        Adt129::C1 => {
            Ok(Adt100::C1)
        }
        Adt129::C2 => {
            Ok(Adt100::C1)
        }
        Adt129::C3(v2) => {
            let m1180 = f197(v2);
            match m1180 {
                None => {
                    Ok(Adt100::C1)
                }
                Some(v3) => {
                    Ok(Adt100::C2(v3))
                }
            }
        }
    }
}

pub fn f293(v0: i64) -> bool {
    let a1187 = {
        let m1183 = {
            let a1181 = v0;
            let a1182 = 0i64;
            compare(a1181, a1182)
        };
        if match m1183 {
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
            let m1186 = {
                let a1184 = 0i64;
                let a1185 = v0;
                compare(a1184, a1185)
            };
            match m1186 {
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
    bool_not(a1187)
}

pub fn f294() -> u64 {
    512u64
}

pub fn f295(v0: Adt103, v1: u64, v2: Str) -> Adt124 {
    let m1189 = match v0.clone() {
        Adt103::C0(_, h1188, _) => {
            h1188
        }
    };
    match m1189.uncons() {
        None => {
            f167(Adt90::C2(v1))
        }
        Some((v3, v4)) => {
            let m1190 = v3;
            match m1190 {
                Adt108::C0(_) => {
                    f167(Adt90::C2(v1))
                }
                Adt108::C1(v6, _) => {
                    Adt124::C1(Adt103::C0(Adt104::C5, List::cons(Adt108::C1(v6, Some(v2)), v4), match v0 {
                        Adt103::C0(_, _, h1191) => {
                            h1191
                        }
                    }))
                }
            }
        }
    }
}

pub fn f296(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1192 = v0;
    let a1193 = 128u64;
    if nat_lt(a1192, a1193) {
        Ok(List::cons(v0, v1))
    } else {
        let a1194 = v0;
        let a1195 = 2048u64;
        if nat_lt(a1194, a1195) {
            Ok(List::cons({
                let a1196 = 128u64;
                let a1197 = f354(v0, 64u64);
                nat_add(a1196, a1197)?
            }, List::cons({
                let a1198 = 192u64;
                let a1199 = f355(v0, 64u64);
                nat_add(a1198, a1199)?
            }, v1)))
        } else {
            let a1200 = v0;
            let a1201 = 65536u64;
            if nat_lt(a1200, a1201) {
                Ok(List::cons({
                    let a1202 = 128u64;
                    let a1203 = f354(v0, 64u64);
                    nat_add(a1202, a1203)?
                }, List::cons({
                    let a1204 = 128u64;
                    let a1205 = f354(f355(v0, 64u64), 64u64);
                    nat_add(a1204, a1205)?
                }, List::cons({
                    let a1206 = 224u64;
                    let a1207 = f355(v0, 4096u64);
                    nat_add(a1206, a1207)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1208 = 128u64;
                    let a1209 = f354(v0, 64u64);
                    nat_add(a1208, a1209)?
                }, List::cons({
                    let a1210 = 128u64;
                    let a1211 = f354(f355(v0, 64u64), 64u64);
                    nat_add(a1210, a1211)?
                }, List::cons({
                    let a1212 = 128u64;
                    let a1213 = f354(f355(v0, 4096u64), 64u64);
                    nat_add(a1212, a1213)?
                }, List::cons({
                    let a1214 = 240u64;
                    let a1215 = f355(v0, 262144u64);
                    nat_add(a1214, a1215)?
                }, v1)))))
            }
        }
    }
}

pub fn f297() -> u64 {
    18u64
}

pub fn f298(v0: u64) -> bool {
    let a1216 = 48u64;
    let a1217 = v0;
    if if nat_le(a1216, a1217) {
        let a1218 = v0;
        let a1219 = 57u64;
        nat_le(a1218, a1219)
    } else {
        false
    } {
        true
    } else {
        let a1220 = 65u64;
        let a1221 = v0;
        if if nat_le(a1220, a1221) {
            let a1222 = v0;
            let a1223 = 90u64;
            nat_le(a1222, a1223)
        } else {
            false
        } {
            true
        } else {
            let a1224 = 97u64;
            let a1225 = v0;
            if if nat_le(a1224, a1225) {
                let a1226 = v0;
                let a1227 = 122u64;
                nat_le(a1226, a1227)
            } else {
                false
            } {
                true
            } else {
                let a1228 = v0;
                let a1229 = 33u64;
                if nat_eq(a1228, a1229) {
                    true
                } else {
                    let a1230 = v0;
                    let a1231 = 35u64;
                    if nat_eq(a1230, a1231) {
                        true
                    } else {
                        let a1232 = v0;
                        let a1233 = 36u64;
                        if nat_eq(a1232, a1233) {
                            true
                        } else {
                            let a1234 = v0;
                            let a1235 = 37u64;
                            if nat_eq(a1234, a1235) {
                                true
                            } else {
                                let a1236 = v0;
                                let a1237 = 38u64;
                                if nat_eq(a1236, a1237) {
                                    true
                                } else {
                                    let a1238 = v0;
                                    let a1239 = 39u64;
                                    if nat_eq(a1238, a1239) {
                                        true
                                    } else {
                                        let a1240 = v0;
                                        let a1241 = 42u64;
                                        if nat_eq(a1240, a1241) {
                                            true
                                        } else {
                                            let a1242 = v0;
                                            let a1243 = 43u64;
                                            if nat_eq(a1242, a1243) {
                                                true
                                            } else {
                                                let a1244 = v0;
                                                let a1245 = 45u64;
                                                if nat_eq(a1244, a1245) {
                                                    true
                                                } else {
                                                    let a1246 = v0;
                                                    let a1247 = 46u64;
                                                    if nat_eq(a1246, a1247) {
                                                        true
                                                    } else {
                                                        let a1248 = v0;
                                                        let a1249 = 94u64;
                                                        if nat_eq(a1248, a1249) {
                                                            true
                                                        } else {
                                                            let a1250 = v0;
                                                            let a1251 = 95u64;
                                                            if nat_eq(a1250, a1251) {
                                                                true
                                                            } else {
                                                                let a1252 = v0;
                                                                let a1253 = 96u64;
                                                                if nat_eq(a1252, a1253) {
                                                                    true
                                                                } else {
                                                                    let a1254 = v0;
                                                                    let a1255 = 124u64;
                                                                    if nat_eq(a1254, a1255) {
                                                                        true
                                                                    } else {
                                                                        let a1256 = v0;
                                                                        let a1257 = 126u64;
                                                                        nat_eq(a1256, a1257)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f299(v0: u64) -> bool {
    let a1258 = 48u64;
    let a1259 = v0;
    if if nat_le(a1258, a1259) {
        let a1260 = v0;
        let a1261 = 57u64;
        nat_le(a1260, a1261)
    } else {
        false
    } {
        true
    } else {
        let a1262 = 65u64;
        let a1263 = v0;
        if if nat_le(a1262, a1263) {
            let a1264 = v0;
            let a1265 = 90u64;
            nat_le(a1264, a1265)
        } else {
            false
        } {
            true
        } else {
            let a1266 = 97u64;
            let a1267 = v0;
            if if nat_le(a1266, a1267) {
                let a1268 = v0;
                let a1269 = 122u64;
                nat_le(a1268, a1269)
            } else {
                false
            } {
                true
            } else {
                let a1270 = v0;
                let a1271 = 95u64;
                if nat_eq(a1270, a1271) {
                    true
                } else {
                    let a1272 = v0;
                    let a1273 = 45u64;
                    if nat_eq(a1272, a1273) {
                        true
                    } else {
                        let a1274 = v0;
                        let a1275 = 46u64;
                        nat_eq(a1274, a1275)
                    }
                }
            }
        }
    }
}

pub fn f300() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f301(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt132, v6: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1276 = f356(v0.clone(), v1, v2.clone());
    match m1276 {
        None => {
            let m1277 = f357(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1277 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt125>, Adt132), Adt94>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt125>, Adt132), Adt94>((f358(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt125>, Adt132), Adt94>(v7))
        }
    }
}

pub fn f302(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt133> {
    Ok(f359(List::cons((Str::lit("type"), f360(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f361(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f362(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f363(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f363(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f363(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f363(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1278 = f364(v0.clone(), v1.clone())?;
        let a1279 = List::cons((Str::lit("required"), f365(v0, v1)), List::<(Str, Adt143)>::nil());
        append_list(a1278, a1279)
    })))))))))
}

pub fn f303(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    Ok(f366(f367(v0, v1.clone(), v2.clone(), v3.clone()), f366(f368(v1.clone(), v2.clone(), v3.clone()), f366(f369(v1.clone(), v2.clone(), v3.clone()), f366(f370(v1.clone(), v2.clone(), v3.clone())?, f366(f371(v1.clone(), v2.clone(), v3.clone()), f366(f372(Adt113::C0, f243(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f366(f372(Adt113::C1, f244(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f366(f372(Adt113::C2, f245(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f372(Adt113::C3, f246(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f304(v0: Adt6) -> Option<List<Adt6>> {
    let m1280 = v0;
    match m1280 {
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

pub fn f305(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1281 = f154(v0, v1);
    match m1281 {
        Adt123::C0 => {
            None::<Adt6>
        }
        Adt123::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f306(v0: List<Adt50>) -> R<bool> {
    let m1282 = v0;
    match m1282.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f373(v2, match v1 {
                Adt50::C0(h1283, _, _) => {
                    h1283
                }
            })
        }
    }
}

pub fn f307(v0: Str) -> R<Str> {
    let m1286 = {
        let a1285 = f279(f280(), f129(f374(f55({
            let a1284 = v0.clone();
            utf8_encode(a1284)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1285)
    };
    match m1286 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f308(v0: u64) -> List<u64> {
    f223(v0)
}

pub fn f309(v0: Adt7) -> R<bool> {
    let a1304 = {
        let m1290 = {
            let a1288 = f375();
            let a1289 = match v0.clone() {
                Adt7::C0(h1287, _) => {
                    h1287
                }
            };
            compare(a1288, a1289)
        };
        if match m1290 {
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
            let m1295 = {
                let a1293 = match v0.clone() {
                    Adt7::C0(h1291, _) => {
                        h1291
                    }
                };
                let a1294 = {
                    let a1292 = f375();
                    int_neg(a1292)?
                };
                compare(a1293, a1294)
            };
            if match m1295 {
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
                let m1299 = {
                    let a1297 = 9999i64;
                    let a1298 = match v0.clone() {
                        Adt7::C0(_, h1296) => {
                            h1296
                        }
                    };
                    compare(a1297, a1298)
                };
                if match m1299 {
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
                    let m1303 = {
                        let a1301 = match v0 {
                            Adt7::C0(_, h1300) => {
                                h1300
                            }
                        };
                        let a1302 = -9999i64;
                        compare(a1301, a1302)
                    };
                    match m1303 {
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
    Ok(bool_not(a1304))
}

pub fn f310(v0: Adt7) -> R<List<u64>> {
    let m1308 = {
        let a1306 = match v0.clone() {
            Adt7::C0(_, h1305) => {
                h1305
            }
        };
        let a1307 = 0i64;
        compare(a1306, a1307)
    };
    if if match m1308 {
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
        let m1312 = {
            let a1310 = 0i64;
            let a1311 = match v0.clone() {
                Adt7::C0(_, h1309) => {
                    h1309
                }
            };
            compare(a1310, a1311)
        };
        match m1312 {
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
        let a1315 = f376(match v0.clone() {
            Adt7::C0(h1313, _) => {
                h1313
            }
        })?;
        let a1316 = List::cons(101u64, f376(match v0 {
            Adt7::C0(_, h1314) => {
                h1314
            }
        })?);
        Ok(append_list(a1315, a1316))
    } else {
        f376(match v0 {
            Adt7::C0(h1317, _) => {
                h1317
            }
        })
    }
}

pub fn f311(v0: u64, v1: Adt114, v2: List<Adt114>) -> List<Adt114> {
    List::cons(Adt114::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f312(p1320: List<u64>, p1321: List<u64>) -> R<List<u64>> {
    let m1319 = (p1320, p1321);
    {
        let mut m1319 = m1319;
        loop {
            let step = {
                let (v0, v1) = m1319;
                let m1318 = v0;
                match m1318.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f129(f377(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1319 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313(v0: List<Str>) -> Adt6 {
    Adt6::C4(f272(v0))
}

pub fn f314(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1339 = f332(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1322, _, _, _, _, _, _) => {
                h1322
            }
        });
        let a1340 = {
            let a1337 = f332(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1323, _, _, _, _, _) => {
                    h1323
                }
            });
            let a1338 = {
                let a1335 = f332(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1324, _, _, _, _) => {
                        h1324
                    }
                });
                let a1336 = {
                    let a1333 = f378(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1325, _, _, _) => {
                            h1325
                        }
                    });
                    let a1334 = {
                        let a1331 = f379(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1326, _, _) => {
                                h1326
                            }
                        });
                        let a1332 = {
                            let a1329 = f378(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1327, _) => {
                                    h1327
                                }
                            });
                            let a1330 = f332(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1328) => {
                                    h1328
                                }
                            });
                            append_list(a1329, a1330)
                        };
                        append_list(a1331, a1332)
                    };
                    append_list(a1333, a1334)
                };
                append_list(a1335, a1336)
            };
            append_list(a1337, a1338)
        };
        append_list(a1339, a1340)
    })
}

pub fn f315(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1341 = v1;
    match m1341 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f316(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f380(match v0.clone() {
        Adt10::C0(h1342, _) => {
            h1342
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f381(match v0 {
        Adt10::C0(_, h1343) => {
            h1343
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f317(v0: List<Adt35>) -> List<Adt6> {
    f214(f382(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f318(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1344 = f383(v0, List::<Adt6>::nil())?;
    match m1344 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f214(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f319(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1345 = v1;
    match m1345 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f320(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1346 = v1;
    match m1346 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f321() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f322(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1347, _, _, _, _, _) => {
            h1347
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1348, _, _, _) => {
            h1348
        }
    })), {
        let a1357 = f315(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1349, _, _, _, _) => {
                h1349
            }
        });
        let a1358 = {
            let a1355 = f315(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1350, _, _) => {
                    h1350
                }
            });
            let a1356 = {
                let a1353 = f315(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1351, _) => {
                        h1351
                    }
                });
                let a1354 = f384(match v0 {
                    Adt1::C0(_, _, _, _, _, h1352) => {
                        h1352
                    }
                });
                append_list(a1353, a1354)
            };
            append_list(a1355, a1356)
        };
        append_list(a1357, a1358)
    })))
}

pub fn f323(p1373: u64, p1374: Adt141, p1375: List<Adt142>) -> R<Option<Adt6>> {
    let m1372 = (p1373, (p1374, p1375));
    {
        let mut m1372 = m1372;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1372;
                let m1359 = v0;
                if m1359 == 0 {
                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1359 - 1;
                    let m1360 = v1;
                    match m1360 {
                        Adt141::C0(v4) => {
                            let m1361 = v4.clone();
                            match m1361 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f385(v7)?;
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1362 = v9;
                                    match m1362.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v10), List::cons(Adt142::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1363 = v12;
                                    match m1363.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f385({
                                                let (h1364, _) = v13;
                                                h1364
                                            })?;
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                let (_, h1365) = v13;
                                                h1365
                                            }), List::cons(Adt142::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt141::C1(v16) => {
                            let m1366 = v2;
                            match m1366.uncons() {
                                None => {
                                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1367 = v17;
                                    match m1367 {
                                        Adt142::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1368 = v20;
                                            match m1368.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C4(f214(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v22), List::cons(Adt142::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt142::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1369 = v26;
                                            match m1369.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C5(f216(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f385({
                                                        let (h1370, _) = v28;
                                                        h1370
                                                    })?;
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                        let (_, h1371) = v28;
                                                        h1371
                                                    }), List::cons(Adt142::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324() -> u64 {
    67108864u64
}

pub fn f325(v0: Adt19) -> Adt6 {
    let m1376 = v0;
    match m1376 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f326(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1377 = v0;
    match m1377 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f327(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1378 = v0;
    match m1378 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f328() -> i64 {
    -32603i64
}

pub fn f329() -> i64 {
    429i64
}

pub fn f330() -> i64 {
    -32021i64
}

pub fn f331(p1381: List<Str>, p1382: List<Adt6>) -> List<Adt6> {
    let m1380 = (p1381, p1382);
    {
        let mut m1380 = m1380;
        loop {
            let step = {
                let (v0, v1) = m1380;
                let m1379 = v0;
                match m1379.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1383 = v1;
    match m1383 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1384) => {
                    h1384
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f333(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt120>) -> R<Result<Adt116, Adt119>> {
    let m1385 = v3;
    match m1385 {
        Ok(v5) => {
            f386(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(Adt119::C0(v4)))
        }
    }
}

pub fn f334(v0: Adt128, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt120> {
    let m1386 = v0;
    match m1386 {
        Adt128::C0 => {
            f387(v1, v2)
        }
        Adt128::C1 => {
            f388(v1, v2)
        }
        Adt128::C2 => {
            f389(v1, v2)
        }
        Adt128::C3 => {
            f390(v1, v2)
        }
        Adt128::C4 => {
            f391(v1, v2)
        }
        Adt128::C5 => {
            f392(v1, v2)
        }
        Adt128::C6 => {
            f393(v1, v2)
        }
        Adt128::C7 => {
            f394(v1, v2)
        }
        Adt128::C8 => {
            f395(v1, v2)
        }
        Adt128::C9 => {
            f396(v1, v2)
        }
        Adt128::C10 => {
            f397(v1, v2)
        }
        Adt128::C11 => {
            f398(v1, v2)
        }
    }
}

pub fn f335(v0: Adt128) -> Str {
    let m1387 = v0;
    match m1387 {
        Adt128::C0 => {
            Str::lit("server/discover")
        }
        Adt128::C1 => {
            Str::lit("tools/list")
        }
        Adt128::C2 => {
            Str::lit("tools/call")
        }
        Adt128::C3 => {
            Str::lit("resources/list")
        }
        Adt128::C4 => {
            Str::lit("resources/read")
        }
        Adt128::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt128::C6 => {
            Str::lit("prompts/list")
        }
        Adt128::C7 => {
            Str::lit("prompts/get")
        }
        Adt128::C8 => {
            Str::lit("completion/complete")
        }
        Adt128::C9 => {
            Str::lit("tasks/get")
        }
        Adt128::C10 => {
            Str::lit("tasks/update")
        }
        Adt128::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f336(v0: Option<Adt8>) -> bool {
    let m1388 = v0;
    match m1388 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f337(v0: Option<Adt9>) -> bool {
    let m1389 = v0;
    match m1389 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f338(v0: Option<Adt5>) -> bool {
    let m1390 = v0;
    match m1390 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f339(v0: Option<Adt5>, v1: Str) -> bool {
    let m1391 = v0;
    match m1391 {
        None => {
            false
        }
        Some(v2) => {
            f155(v1, v2)
        }
    }
}

pub fn f340() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f341(v0: List<Str>, v1: Adt95, v2: Adt16, v3: Option<Adt97>) -> Adt136 {
    let m1392 = v3;
    match m1392 {
        None => {
            f399(v1, v2.clone(), f400(match v2 {
                Adt16::C0(h1393, _, _, _, _, _) => {
                    h1393
                }
            }, v0))
        }
        Some(v4) => {
            Adt136::C1(Adt119::C2(v4))
        }
    }
}

pub fn f342(v0: Option<Adt96>, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1394 = v0;
    match m1394 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v5) => {
            let m1396 = match v5.clone() {
                Adt96::C0(_, _, _, h1395, _) => {
                    h1395
                }
            };
            match m1396 {
                None => {
                    f401(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f343(v0: Option<Adt5>) -> Adt5 {
    let m1397 = v0;
    match m1397 {
        None => {
            f402()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f344(v0: Adt5) -> Result<Adt16, Adt120> {
    let m1398 = f403(v0.clone());
    match m1398 {
        Adt145::C0 => {
            f404(v0.clone(), f111(f405(), v0.clone()), f111(f406(), v0))
        }
        Adt145::C1 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt145::C2 => {
            Err::<Adt16, Adt120>(Adt120::C3)
        }
    }
}

pub fn f345(p1403: Bytes, p1404: List<Bytes>) -> Bytes {
    let m1402 = (p1403, p1404);
    {
        let mut m1402 = m1402;
        loop {
            let step = {
                let (v0, v1) = m1402;
                let m1399 = v1;
                match m1399.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1400 = v2;
                            let a1401 = v0;
                            append_bytes(a1400, a1401)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1402 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f346(p1407: List<Bytes>, p1408: List<u64>) -> List<Bytes> {
    let m1406 = (p1407, p1408);
    {
        let mut m1406 = m1406;
        loop {
            let step = {
                let (v0, v1) = m1406;
                let m1405 = v1;
                match m1405.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f407(f408(v2), v0), v3))
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

pub fn f347(v0: u64) -> R<u64> {
    let a1409 = 64u64;
    let a1410 = v0;
    if if nat_lt(a1409, a1410) {
        let a1411 = v0;
        let a1412 = 91u64;
        nat_lt(a1411, a1412)
    } else {
        false
    } {
        let a1413 = v0;
        let a1414 = 32u64;
        nat_add(a1413, a1414)
    } else {
        Ok(v0)
    }
}

pub fn f348(p1417: List<u64>) -> List<u64> {
    let m1416 = p1417;
    {
        let mut m1416 = m1416;
        loop {
            let step = {
                let v0 = m1416;
                let m1415 = v0;
                match m1415.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f409(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f349(v0: u64) -> bool {
    let a1418 = v0;
    let a1419 = 9u64;
    if nat_eq(a1418, a1419) {
        true
    } else {
        let a1420 = 31u64;
        let a1421 = v0;
        if nat_lt(a1420, a1421) {
            let a1422 = v0;
            let a1423 = 127u64;
            nat_lt(a1422, a1423)
        } else {
            false
        }
    }
}

pub fn f350(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1424 = f410(v0)?;
    match m1424 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1425 = f410(v1)?;
            match m1425 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1426 = v2;
                    let a1427 = 61u64;
                    if nat_eq(a1426, a1427) {
                        f411(v5, v6, v4, v3)
                    } else {
                        let m1428 = f410(v2)?;
                        match m1428 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1429 = v3;
                                let a1430 = 61u64;
                                if nat_eq(a1429, a1430) {
                                    f412(v5, v6, v7, v4)
                                } else {
                                    let m1431 = f410(v3)?;
                                    match m1431 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f413(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f351(v0: List<u64>) -> bool {
    let m1432 = v0;
    match m1432.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f352() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f353(v0: Str) -> R<List<u64>> {
    f55({
        let a1433 = v0;
        utf8_encode(a1433)
    })
}

pub fn f354(v0: u64, v1: u64) -> u64 {
    let a1434 = v0;
    let a1435 = v1;
    let a1436 = 0u64;
    nat_rem(a1434, a1435, a1436)
}

pub fn f355(v0: u64, v1: u64) -> u64 {
    let a1437 = v0;
    let a1438 = v1;
    let a1439 = 0u64;
    nat_quot(a1437, a1438, a1439)
}

pub fn f356(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1440 = f414(f415(), v2.clone());
    match m1440 {
        None => {
            let m1441 = f416(f417(), v2.clone());
            match m1441 {
                None => {
                    f418(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt94::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt94::C4(v0, v3))
        }
    }
}

pub fn f357(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt132) -> R<Adt144> {
    let m1442 = f154(Str::lit("x-mcp-header"), v1.clone());
    match m1442 {
        Adt123::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt123::C1(v5) => {
            f419(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f358(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt125>) -> List<Adt125> {
    f420(Str::lit("properties"), v0.clone(), v1, v2, f420(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f421(Str::lit("items"), v0.clone(), f421(Str::lit("additionalProperties"), v0.clone(), f422(Str::lit("allOf"), v0.clone(), f422(Str::lit("anyOf"), v0.clone(), f422(Str::lit("oneOf"), v0.clone(), f421(Str::lit("not"), v0.clone(), f421(Str::lit("if"), v0.clone(), f421(Str::lit("then"), v0.clone(), f421(Str::lit("else"), v0, v3)))))))))))
}

pub fn f359(p1449: List<(Str, Adt143)>) -> Adt133 {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1443 = v0;
                match m1443.uncons() {
                    None => {
                        Err::<List<(Str, Adt143)>, Adt133>(Adt133::C3(List::<Adt112>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1445 = {
                            let (_, h1444) = v1;
                            h1444
                        };
                        match m1445 {
                            Adt143::C0 => {
                                Ok::<List<(Str, Adt143)>, Adt133>(v2)
                            }
                            Adt143::C1 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C0({
                                    let (h1446, _) = v1;
                                    h1446
                                }))
                            }
                            Adt143::C2 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C1({
                                    let (h1447, _) = v1;
                                    h1447
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f360(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1450 = f154(Str::lit("type"), v0);
    match m1450 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1451 = f151(v2.clone());
            match m1451 {
                None => {
                    let m1452 = f304(v2);
                    match m1452 {
                        None => {
                            Ok(Adt143::C2)
                        }
                        Some(v4) => {
                            f425(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f423(v3.clone()) {
                        if f424(v3, v1)? {
                            Ok(Adt143::C0)
                        } else {
                            Ok(Adt143::C1)
                        }
                    } else {
                        Ok(Adt143::C2)
                    }
                }
            }
        }
    }
}

pub fn f361(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1453 = f154(Str::lit("enum"), v0);
    match m1453 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1454 = f304(v2);
            match m1454 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v3) => {
                    f426(v3, v1)
                }
            }
        }
    }
}

pub fn f362(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1455 = f154(Str::lit("const"), v0);
    match m1455 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1456 = f427(v2, v1)?;
            match m1456 {
                Adt146::C0 => {
                    Ok(Adt143::C0)
                }
                Adt146::C1 => {
                    Ok(Adt143::C1)
                }
                Adt146::C2 => {
                    Ok(Adt143::C2)
                }
            }
        }
    }
}

pub fn f363(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt143> {
    let m1457 = f154(v0, v2);
    match m1457 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1458 = f428(v4);
            match m1458 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1459 = f428(v3);
                    match m1459 {
                        None => {
                            Ok(Adt143::C0)
                        }
                        Some(v6) => {
                            if f429(v1, f430(v6, v5)?) {
                                Ok(Adt143::C0)
                            } else {
                                Ok(Adt143::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f364(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt143)>> {
    let m1460 = v1;
    match m1460 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt143)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt143)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt143)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f431(Str::lit("minLength"), true, v0.clone(), f432(v4.clone())?)?), List::cons((Str::lit("maxLength"), f431(Str::lit("maxLength"), false, v0, f432(v4)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f431(Str::lit("minItems"), true, v0.clone(), f433(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f431(Str::lit("maxItems"), false, v0, f433(v5, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f431(Str::lit("minProperties"), true, v0.clone(), f434(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f431(Str::lit("maxProperties"), false, v0, f434(v6, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
    }
}

pub fn f365(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt143 {
    let m1461 = f154(Str::lit("required"), v0);
    match m1461 {
        Adt123::C0 => {
            Adt143::C0
        }
        Adt123::C1(v2) => {
            let m1462 = f304(v2);
            match m1462 {
                None => {
                    Adt143::C2
                }
                Some(v3) => {
                    let m1463 = f239(v1);
                    match m1463 {
                        None => {
                            Adt143::C0
                        }
                        Some(v4) => {
                            f435(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f366(v0: Adt133, v1: Adt133) -> Adt133 {
    let m1464 = v0.clone();
    match m1464 {
        Adt133::C0(_) => {
            v0
        }
        Adt133::C1(_) => {
            v0
        }
        Adt133::C2(_) => {
            v0
        }
        Adt133::C3(v5) => {
            let m1465 = v1.clone();
            match m1465 {
                Adt133::C0(_) => {
                    v1
                }
                Adt133::C1(_) => {
                    v1
                }
                Adt133::C2(_) => {
                    v1
                }
                Adt133::C3(v9) => {
                    Adt133::C3({
                        let a1466 = v5;
                        let a1467 = v9;
                        append_list(a1466, a1467)
                    })
                }
            }
        }
    }
}

pub fn f367(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m1468 = f154(Str::lit("$ref"), v1);
    match m1468 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v4) => {
            let m1469 = f151(v4);
            match m1469 {
                None => {
                    Adt133::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1470 = f436(v0, v5.clone());
                    match m1470 {
                        None => {
                            Adt133::C2(v5)
                        }
                        Some(v6) => {
                            Adt133::C3(List::cons(Adt112::C0(v6, v2, v3), List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f368(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1471 = f154(Str::lit("properties"), v0);
    match m1471 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1472 = f239(v3);
            match m1472 {
                None => {
                    Adt133::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1473 = f239(v1);
                    match m1473 {
                        None => {
                            Adt133::C3(List::<Adt112>::nil())
                        }
                        Some(v5) => {
                            Adt133::C3(f437(match v4 {
                                Adt5::C0(h1474) => {
                                    h1474
                                }
                            }, match v5 {
                                Adt5::C0(h1475) => {
                                    h1475
                                }
                            }, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f369(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1476 = f154(Str::lit("additionalProperties"), v0.clone());
    match m1476 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1477 = f239(v1);
            match m1477 {
                None => {
                    Adt133::C3(List::<Adt112>::nil())
                }
                Some(v4) => {
                    let m1478 = f238(v3.clone());
                    match m1478 {
                        None => {
                            Adt133::C3(f440(match v4 {
                                Adt5::C0(h1480) => {
                                    h1480
                                }
                            }, f439(v0), v3, v2, List::<Adt112>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt133::C3(List::<Adt112>::nil())
                            } else {
                                if f438(match v4 {
                                    Adt5::C0(h1479) => {
                                        h1479
                                    }
                                }, f439(v0)) {
                                    Adt133::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt133::C3(List::<Adt112>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f370(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> R<Adt133> {
    let m1481 = f154(Str::lit("items"), v0);
    match m1481 {
        Adt123::C0 => {
            Ok(Adt133::C3(List::<Adt112>::nil()))
        }
        Adt123::C1(v3) => {
            let m1482 = f304(v1);
            match m1482 {
                None => {
                    Ok(Adt133::C3(List::<Adt112>::nil()))
                }
                Some(v4) => {
                    Ok(Adt133::C3(f441(v4, 0u64, v3, v2, List::<Adt112>::nil())?))
                }
            }
        }
    }
}

pub fn f371(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1483 = f154(Str::lit("allOf"), v0);
    match m1483 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1484 = f304(v3);
            match m1484 {
                None => {
                    Adt133::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1485 = v4.clone();
                    match m1485.uncons() {
                        None => {
                            Adt133::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt133::C3(f442(v4, v1, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f372(v0: Adt113, v1: Adt133, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt110>) -> Adt133 {
    let m1486 = v1.clone();
    match m1486 {
        Adt133::C0(_) => {
            v1
        }
        Adt133::C1(_) => {
            v1
        }
        Adt133::C2(_) => {
            v1
        }
        Adt133::C3(v8) => {
            let m1487 = v8;
            match m1487.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt133::C3(List::cons(Adt112::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt112>::nil()))
                }
            }
        }
    }
}

pub fn f373(p1492: List<Adt50>, p1493: Adt7) -> R<bool> {
    let m1491 = (p1492, p1493);
    {
        let mut m1491 = m1491;
        loop {
            let step = {
                let (v0, v1) = m1491;
                let m1488 = v0;
                match m1488.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f443(f430(match v2.clone() {
                            Adt50::C0(h1489, _, _) => {
                                h1489
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1490, _, _) => {
                                    h1490
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(p1500: List<u64>, p1501: List<u64>) -> List<u64> {
    let m1499 = (p1500, p1501);
    {
        let mut m1499 = m1499;
        loop {
            let step = {
                let (v0, v1) = m1499;
                let m1494 = v0;
                match m1494.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1495 = v2;
                        let a1496 = 126u64;
                        if nat_eq(a1495, a1496) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1497 = v2;
                            let a1498 = 47u64;
                            if nat_eq(a1497, a1498) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1499 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375() -> i64 {
    999999999999999999i64
}

pub fn f376(v0: i64) -> R<List<u64>> {
    let m1504 = {
        let a1502 = v0;
        let a1503 = 0i64;
        compare(a1502, a1503)
    };
    if match m1504 {
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
        Ok(List::cons(45u64, f444(20u64, {
            let a1505 = v0;
            int_neg(a1505)?
        }, List::<u64>::nil())?))
    } else {
        f444(20u64, v0, List::<u64>::nil())
    }
}

pub fn f377(v0: u64) -> R<List<u64>> {
    let a1506 = v0;
    let a1507 = 34u64;
    if nat_eq(a1506, a1507) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1508 = v0;
        let a1509 = 92u64;
        if nat_eq(a1508, a1509) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1510 = v0;
            let a1511 = 8u64;
            if nat_eq(a1510, a1511) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1512 = v0;
                let a1513 = 9u64;
                if nat_eq(a1512, a1513) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1514 = v0;
                    let a1515 = 10u64;
                    if nat_eq(a1514, a1515) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1516 = v0;
                        let a1517 = 12u64;
                        if nat_eq(a1516, a1517) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1518 = v0;
                            let a1519 = 13u64;
                            if nat_eq(a1518, a1519) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1520 = v0;
                                let a1521 = 32u64;
                                if nat_lt(a1520, a1521) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f445(f355(v0, 16u64))?, List::cons(f445(f354(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f378(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1522 = v1;
    match m1522 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f446(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f379(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1523 = v0;
    match m1523 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1526 = f320(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1524, _) => {
                        h1524
                    }
                });
                let a1527 = f320(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1525) => {
                        h1525
                    }
                });
                append_list(a1526, a1527)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f380(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f447(v0, 0i64)?, 0i64)))
}

pub fn f381(v0: Adt11) -> Str {
    let m1528 = v0;
    match m1528 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f382(p1531: List<Adt35>, p1532: List<Adt6>) -> List<Adt6> {
    let m1530 = (p1531, p1532);
    {
        let mut m1530 = m1530;
        loop {
            let step = {
                let (v0, v1) = m1530;
                let m1529 = v0;
                match m1529.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f448(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(p1536: List<Adt38>, p1537: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1535 = (p1536, p1537);
    {
        let mut m1535 = m1535;
        loop {
            let step = {
                let (v0, v1) = m1535;
                let m1533 = v0;
                match m1533.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1534 = f449(v2)?;
                        match m1534 {
                            None => {
                                Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1538 = v0;
    match m1538 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f450(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f385(v0: Str) -> R<Str> {
    let m1541 = {
        let a1540 = f279(f280(), f129(f451(f55({
            let a1539 = v0;
            utf8_encode(a1539)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1540)
    };
    match m1541 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f386(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt116, Adt119>> {
    if f452(f453(v3.clone())) {
        Ok(Ok::<Adt116, Adt119>(f454(v0.clone(), Adt32::C0(f455(v0)))))
    } else {
        Ok(f456(v0.clone(), v3.clone(), f457(match v0 {
            Adt0::C0(_, _, _, _, _, h1542) => {
                h1542
            }
        }, v2, v1, v3)?))
    }
}

pub fn f387(v0: Adt16, _: Adt5) -> Result<Adt14, Adt120> {
    Ok::<Adt14, Adt120>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f388(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1543 = f458(Str::lit("cursor"), v1);
    match m1543 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f389(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1544 = f459(Str::lit("name"), v1.clone());
    match m1544 {
        Ok(v2) => {
            let m1545 = f460(Str::lit("arguments"), v1.clone());
            match m1545 {
                Ok(v3) => {
                    let m1546 = f460(Str::lit("inputResponses"), v1.clone());
                    match m1546 {
                        Ok(v4) => {
                            let m1547 = f458(Str::lit("requestState"), v1);
                            match m1547 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt120>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt120>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt120>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt120>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt120>(v9)
        }
    }
}

pub fn f390(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1548 = f458(Str::lit("cursor"), v1);
    match m1548 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f391(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1549 = f459(Str::lit("uri"), v1.clone());
    match m1549 {
        Ok(v2) => {
            let m1550 = f460(Str::lit("inputResponses"), v1.clone());
            match m1550 {
                Ok(v3) => {
                    let m1551 = f458(Str::lit("requestState"), v1);
                    match m1551 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt120>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt120>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt120>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt120>(v7)
        }
    }
}

pub fn f392(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1552 = f458(Str::lit("cursor"), v1);
    match m1552 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f393(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1553 = f458(Str::lit("cursor"), v1);
    match m1553 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f394(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1554 = f459(Str::lit("name"), v1.clone());
    match m1554 {
        Ok(v2) => {
            let m1555 = f461(Str::lit("arguments"), v1.clone());
            match m1555 {
                Ok(v3) => {
                    let m1556 = f460(Str::lit("inputResponses"), v1.clone());
                    match m1556 {
                        Ok(v4) => {
                            let m1557 = f458(Str::lit("requestState"), v1);
                            match m1557 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt120>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt120>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt120>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt120>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt120>(v9)
        }
    }
}

pub fn f395(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1558 = f462(Str::lit("ref"), v1.clone());
    match m1558 {
        Ok(v2) => {
            let m1559 = f463(v2);
            match m1559 {
                Ok(v3) => {
                    let m1560 = f462(Str::lit("argument"), v1.clone());
                    match m1560 {
                        Ok(v4) => {
                            let m1561 = f464(v4);
                            match m1561 {
                                Ok(v5) => {
                                    let m1562 = f465(v1);
                                    match m1562 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt120>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt120>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt120>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt120>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt120>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt120>(v11)
        }
    }
}

pub fn f396(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1563 = f459(Str::lit("taskId"), v1);
    match m1563 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f397(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1564 = f459(Str::lit("taskId"), v1.clone());
    match m1564 {
        Ok(v2) => {
            let m1565 = f462(Str::lit("inputResponses"), v1);
            match m1565 {
                Ok(v3) => {
                    Ok::<Adt14, Adt120>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt120>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt120>(v5)
        }
    }
}

pub fn f398(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1566 = f459(Str::lit("taskId"), v1);
    match m1566 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f399(v0: Adt95, v1: Adt16, v2: Option<Adt119>) -> Adt136 {
    let m1567 = v2;
    match m1567 {
        None => {
            Adt136::C0(Adt137::C0(match v0.clone() {
                Adt95::C0(h1568, _, _) => {
                    h1568
                }
            }, match v0.clone() {
                Adt95::C0(_, h1569, _) => {
                    h1569
                }
            }, v1, f343(match v0 {
                Adt95::C0(_, _, h1570) => {
                    h1570
                }
            })))
        }
        Some(v3) => {
            Adt136::C1(v3)
        }
    }
}

pub fn f400(v0: Str, v1: List<Str>) -> Option<Adt119> {
    f466(f467(v0.clone(), v1.clone()), v0, v1)
}

pub fn f401(v0: Adt96, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1572 = match v0.clone() {
        Adt96::C0(h1571, _, _, _, _) => {
            h1571
        }
    };
    match m1572 {
        None => {
            Ok(Some(Adt97::C0))
        }
        Some(v5) => {
            let a1573 = v5;
            let a1574 = v3;
            if equal(a1573, a1574) {
                let m1576 = match v0.clone() {
                    Adt96::C0(_, h1575, _, _, _) => {
                        h1575
                    }
                };
                match m1576 {
                    None => {
                        Ok(Some(Adt97::C1))
                    }
                    Some(v6) => {
                        let a1577 = v6;
                        let a1578 = v2;
                        if equal(a1577, a1578) {
                            f468(f469(match v0.clone() {
                                Adt96::C0(_, _, h1579, _, _) => {
                                    h1579
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt96::C0(_, _, _, _, h1580) => {
                                    h1580
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt97::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt97::C3))
            }
        }
    }
}

pub fn f402() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f403(v0: Adt5) -> Adt145 {
    f470(f111(f405(), v0.clone()), f111(f406(), v0))
}

pub fn f404(v0: Adt5, v1: Adt123, v2: Adt123) -> Result<Adt16, Adt120> {
    let m1581 = v1;
    match m1581 {
        Adt123::C0 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt123::C1(v3) => {
            let m1582 = v2;
            match m1582 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C3)
                }
                Adt123::C1(v4) => {
                    f471(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f405() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f406() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f407(p1591: Bytes, p1592: List<Bytes>) -> List<Bytes> {
    let m1590 = (p1591, p1592);
    {
        let mut m1590 = m1590;
        loop {
            let step = {
                let (v0, v1) = m1590;
                let m1583 = v1;
                match m1583.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1586 = {
                            let a1584 = v2.clone();
                            length_bytes(a1584)
                        };
                        let a1587 = {
                            let a1585 = v0.clone();
                            length_bytes(a1585)
                        };
                        if nat_eq(a1586, a1587) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1588 = v2;
                                let a1589 = v0;
                                append_bytes(a1588, a1589)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1590 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(v0: u64) -> Bytes {
    let m1596 = {
        let a1593 = f472();
        let a1594 = v0;
        let a1595 = 1u64;
        slice_bytes(a1593, a1594, a1595)
    };
    match m1596 {
        None => {
            f280()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f409(v0: u64) -> bool {
    let a1597 = v0;
    let a1598 = 9u64;
    if nat_eq(a1597, a1598) {
        true
    } else {
        let a1599 = v0;
        let a1600 = 32u64;
        nat_eq(a1599, a1600)
    }
}

pub fn f410(v0: u64) -> R<Option<u64>> {
    let a1601 = 64u64;
    let a1602 = v0;
    if if nat_lt(a1601, a1602) {
        let a1603 = v0;
        let a1604 = 91u64;
        nat_lt(a1603, a1604)
    } else {
        false
    } {
        Ok(Some({
            let a1605 = v0;
            let a1606 = 65u64;
            nat_sub(a1605, a1606)
        }))
    } else {
        let a1607 = 96u64;
        let a1608 = v0;
        if if nat_lt(a1607, a1608) {
            let a1609 = v0;
            let a1610 = 123u64;
            nat_lt(a1609, a1610)
        } else {
            false
        } {
            Ok(Some({
                let a1611 = v0;
                let a1612 = 71u64;
                nat_sub(a1611, a1612)
            }))
        } else {
            let a1613 = 47u64;
            let a1614 = v0;
            if if nat_lt(a1613, a1614) {
                let a1615 = v0;
                let a1616 = 58u64;
                nat_lt(a1615, a1616)
            } else {
                false
            } {
                Ok(Some({
                    let a1617 = v0;
                    let a1618 = 4u64;
                    nat_add(a1617, a1618)?
                }))
            } else {
                let a1619 = v0;
                let a1620 = 43u64;
                if nat_eq(a1619, a1620) {
                    Ok(Some(62u64))
                } else {
                    let a1621 = v0;
                    let a1622 = 47u64;
                    if nat_eq(a1621, a1622) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f411(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1623 = v3;
        let a1624 = 61u64;
        if nat_eq(a1623, a1624) {
            let a1625 = f473(v1, 16u64);
            let a1626 = 0u64;
            nat_eq(a1625, a1626)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f474(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f412(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1627 = f473(v2, 4u64);
        let a1628 = 0u64;
        nat_eq(a1627, a1628)
    } else {
        false
    } {
        Ok(Some(f475(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f413(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1631 = {
            let a1629 = v0;
            let a1630 = 4u64;
            nat_mul(a1629, a1630)?
        };
        let a1632 = f476(v1, 16u64);
        nat_add(a1631, a1632)?
    }, List::cons({
        let a1635 = {
            let a1633 = f473(v1, 16u64);
            let a1634 = 16u64;
            nat_mul(a1633, a1634)?
        };
        let a1636 = f476(v2, 4u64);
        nat_add(a1635, a1636)?
    }, List::cons({
        let a1639 = {
            let a1637 = f473(v2, 4u64);
            let a1638 = 64u64;
            nat_mul(a1637, a1638)?
        };
        let a1640 = v3;
        nat_add(a1639, a1640)?
    }, List::<u64>::nil()))))
}

pub fn f414(p1644: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1643 = p1644;
    {
        let mut m1643 = m1643;
        loop {
            let step = {
                let v0 = m1643;
                let m1641 = v0;
                match m1641.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1642 = f154(v2.clone(), v1.clone());
                        match m1642 {
                            Adt123::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt123::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1643 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f416(p1651: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1650 = p1651;
    {
        let mut m1650 = m1650;
        loop {
            let step = {
                let v0 = m1650;
                let m1645 = v0;
                match m1645.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1647 = f154({
                            let (h1646, _) = v2.clone();
                            h1646
                        }, v1.clone());
                        match m1647 {
                            Adt123::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt123::C1(v4) => {
                                if f477({
                                    let (_, h1648) = v2;
                                    h1648
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1649, _) = v2;
                                        h1649
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f418(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1652 = f154(Str::lit("$ref"), v2);
    match m1652 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v3) => {
            let m1653 = f151(v3);
            match m1653 {
                None => {
                    None::<Adt94>
                }
                Some(v4) => {
                    let m1654 = f436(v1, v4.clone());
                    match m1654 {
                        None => {
                            Some(Adt94::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt94>
                        }
                    }
                }
            }
        }
    }
}

pub fn f419(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    if v3 {
        let m1655 = f151(v2);
        match m1655 {
            None => {
                Ok(Adt144::C0(Adt94::C7(v0)))
            }
            Some(v6) => {
                f478(v0, v1, v6.clone(), f121(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt94::C10(v0)))
    }
}

pub fn f420(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt125>) -> List<Adt125> {
    let m1656 = f154(v0.clone(), v1);
    match m1656 {
        Adt123::C0 => {
            v4
        }
        Adt123::C1(v5) => {
            let m1657 = f239(v5);
            match m1657 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1658 = v0;
                    let a1659 = Str::lit("properties");
                    if equal(a1658, a1659) {
                        f479(match v6 {
                            Adt5::C0(h1660) => {
                                h1660
                            }
                        }, v2, v3, v4)
                    } else {
                        f480(match v6 {
                            Adt5::C0(h1661) => {
                                h1661
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f421(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1662 = f154(v0, v1);
    match m1662 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            List::cons(Adt125::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f422(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1663 = f154(v0, v1);
    match m1663 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            let m1664 = f304(v3);
            match m1664 {
                None => {
                    v2
                }
                Some(v4) => {
                    f481(v4, v2)
                }
            }
        }
    }
}

pub fn f423(v0: Str) -> bool {
    let a1665 = v0.clone();
    let a1666 = Str::lit("null");
    if equal(a1665, a1666) {
        true
    } else {
        let a1667 = v0.clone();
        let a1668 = Str::lit("boolean");
        if equal(a1667, a1668) {
            true
        } else {
            let a1669 = v0.clone();
            let a1670 = Str::lit("object");
            if equal(a1669, a1670) {
                true
            } else {
                let a1671 = v0.clone();
                let a1672 = Str::lit("array");
                if equal(a1671, a1672) {
                    true
                } else {
                    let a1673 = v0.clone();
                    let a1674 = Str::lit("number");
                    if equal(a1673, a1674) {
                        true
                    } else {
                        let a1675 = v0.clone();
                        let a1676 = Str::lit("string");
                        if equal(a1675, a1676) {
                            true
                        } else {
                            let a1677 = v0;
                            let a1678 = Str::lit("integer");
                            equal(a1677, a1678)
                        }
                    }
                }
            }
        }
    }
}

pub fn f424(v0: Str, v1: Adt6) -> R<bool> {
    let a1679 = v0.clone();
    let a1680 = f482(v1.clone());
    if equal(a1679, a1680) {
        Ok(true)
    } else {
        let a1681 = v0;
        let a1682 = Str::lit("integer");
        if equal(a1681, a1682) {
            let m1683 = f428(v1);
            match m1683 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f483(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f425(p1686: List<Adt6>, v1: Adt6, p1687: bool) -> R<Adt143> {
    let m1685 = (p1686, p1687);
    {
        let mut m1685 = m1685;
        loop {
            let step = {
                let (v0, v2) = m1685;
                let m1684 = v0;
                match m1684.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f423(f484(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt143>>((v4, if v2 {
                                true
                            } else {
                                f424(f484(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(p1691: List<Adt6>, v1: Adt6) -> R<Adt143> {
    let m1690 = p1691;
    {
        let mut m1690 = m1690;
        loop {
            let step = {
                let v0 = m1690;
                let m1688 = v0;
                match m1688.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C1))
                    }
                    Some((v2, v3)) => {
                        let m1689 = f427(v2, v1.clone())?;
                        match m1689 {
                            Adt146::C0 => {
                                Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C0))
                            }
                            Adt146::C1 => {
                                Ok::<List<Adt6>, R<Adt143>>(v3)
                            }
                            Adt146::C2 => {
                                Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f485(f486(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f428(v0: Adt6) -> Option<Adt7> {
    let m1692 = v0;
    match m1692 {
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

pub fn f429(v0: u64, v1: Adt147) -> bool {
    let a1693 = v0;
    let a1694 = 0u64;
    if nat_eq(a1693, a1694) {
        let a1695 = f487(v1);
        bool_not(a1695)
    } else {
        let a1696 = v0;
        let a1697 = 1u64;
        if nat_eq(a1696, a1697) {
            let a1698 = f443(v1);
            bool_not(a1698)
        } else {
            let a1699 = v0;
            let a1700 = 2u64;
            if nat_eq(a1699, a1700) {
                f443(v1)
            } else {
                f487(v1)
            }
        }
    }
}

pub fn f430(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f488(v0.clone(), v1.clone(), f489(match v0 {
        Adt7::C0(h1701, _) => {
            h1701
        }
    }), f489(match v1 {
        Adt7::C0(h1702, _) => {
            h1702
        }
    }))
}

pub fn f431(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt143> {
    let m1703 = f154(v0, v2);
    match m1703 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1704 = f490(v4);
            match m1704 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1707 = {
                        let a1705 = v5;
                        let a1706 = 0i64;
                        compare(a1705, a1706)
                    };
                    if match m1707 {
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
                        Ok(Adt143::C2)
                    } else {
                        if v1 {
                            if f487(f491(f89(v3, 0i64)?, v5)) {
                                Ok(Adt143::C1)
                            } else {
                                Ok(Adt143::C0)
                            }
                        } else {
                            if f443(f491(f89(v3, 0i64)?, v5)) {
                                Ok(Adt143::C1)
                            } else {
                                Ok(Adt143::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f432(v0: Str) -> R<u64> {
    f492(f55({
        let a1708 = v0;
        utf8_encode(a1708)
    })?, 0u64)
}

pub fn f433(p1713: List<Adt6>, p1714: u64) -> R<u64> {
    let m1712 = (p1713, p1714);
    {
        let mut m1712 = m1712;
        loop {
            let step = {
                let (v0, v1) = m1712;
                let m1709 = v0;
                match m1709.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1710 = v1;
                            let a1711 = 1u64;
                            nat_add(a1710, a1711)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(p1719: List<(Str, Adt6)>, p1720: u64) -> R<u64> {
    let m1718 = (p1719, p1720);
    {
        let mut m1718 = m1718;
        loop {
            let step = {
                let (v0, v1) = m1718;
                let m1715 = v0;
                match m1715.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1716 = v1;
                            let a1717 = 1u64;
                            nat_add(a1716, a1717)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1718 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(p1724: List<Adt6>, v1: Adt5) -> Adt143 {
    let m1723 = p1724;
    {
        let mut m1723 = m1723;
        loop {
            let step = {
                let v0 = m1723;
                let m1721 = v0;
                match m1721.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt143>(Adt143::C0)
                    }
                    Some((v2, v3)) => {
                        let m1722 = f151(v2);
                        match m1722 {
                            None => {
                                Err::<List<Adt6>, Adt143>(Adt143::C2)
                            }
                            Some(v4) => {
                                if f155(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt143>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt143>(Adt143::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f436(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1725 = v1.clone();
    let a1726 = Str::lit("#");
    if equal(a1725, a1726) {
        Some(v0)
    } else {
        let m1727 = f239(v0);
        match m1727 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1728 = f111(Str::lit("$defs"), v2);
                match m1728 {
                    Adt123::C0 => {
                        None::<Adt6>
                    }
                    Adt123::C1(v3) => {
                        let m1729 = f239(v3);
                        match m1729 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f493(v1, match v4 {
                                    Adt5::C0(h1730) => {
                                        h1730
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

pub fn f437(p1737: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt110>, p1738: List<Adt112>) -> List<Adt112> {
    let m1736 = (p1737, p1738);
    {
        let mut m1736 = m1736;
        loop {
            let step = {
                let (v0, v3) = m1736;
                let m1731 = v0;
                match m1731.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1733 = f154({
                            let (h1732, _) = v4.clone();
                            h1732
                        }, v1.clone());
                        match m1733 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, v3))
                            }
                            Adt123::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0({
                                    let (_, h1734) = v4;
                                    h1734
                                }, v6, List::cons(Adt110::C0({
                                    let (h1735, _) = v4;
                                    h1735
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1736 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(p1743: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1742 = p1743;
    {
        let mut m1742 = m1742;
        loop {
            let step = {
                let v0 = m1742;
                let m1739 = v0;
                match m1739.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1741 = f154({
                            let (h1740, _) = v2;
                            h1740
                        }, v1.clone());
                        match m1741 {
                            Adt123::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt123::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1744 = f154(Str::lit("properties"), v0);
    match m1744 {
        Adt123::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt123::C1(v1) => {
            let m1745 = f239(v1);
            match m1745 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1746) => {
                            h1746
                        }
                    }
                }
            }
        }
    }
}

pub fn f440(p1753: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>, p1754: List<Adt112>) -> List<Adt112> {
    let m1752 = (p1753, p1754);
    {
        let mut m1752 = m1752;
        loop {
            let step = {
                let (v0, v4) = m1752;
                let m1747 = v0;
                match m1747.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1749 = f154({
                            let (h1748, _) = v5.clone();
                            h1748
                        }, v1.clone());
                        match m1749 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v6, List::cons(Adt112::C0(v2.clone(), {
                                    let (_, h1750) = v5;
                                    h1750
                                }, List::cons(Adt110::C0({
                                    let (h1751, _) = v5;
                                    h1751
                                }), v3.clone())), v4)))
                            }
                            Adt123::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(p1759: List<Adt6>, p1760: u64, v2: Adt6, v3: List<Adt110>, p1761: List<Adt112>) -> R<List<Adt112>> {
    let m1758 = (p1759, (p1760, p1761));
    {
        let mut m1758 = m1758;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1758;
                let m1755 = v0;
                match m1755.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>((v6, ({
                            let a1756 = v1;
                            let a1757 = 1u64;
                            nat_add(a1756, a1757)?
                        }, List::cons(Adt112::C0(v2.clone(), v5, List::cons(Adt110::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(p1764: List<Adt6>, v1: Adt6, v2: List<Adt110>, p1765: List<Adt112>) -> List<Adt112> {
    let m1763 = (p1764, p1765);
    {
        let mut m1763 = m1763;
        loop {
            let step = {
                let (v0, v3) = m1763;
                let m1762 = v0;
                match m1762.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1763 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: Adt147) -> bool {
    let m1766 = v0;
    match m1766 {
        Adt147::C0 => {
            false
        }
        Adt147::C1 => {
            false
        }
        Adt147::C2 => {
            true
        }
    }
}

pub fn f444(p1778: u64, p1779: i64, p1780: List<u64>) -> R<List<u64>> {
    let m1777 = (p1778, (p1779, p1780));
    {
        let mut m1777 = m1777;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1777;
                let m1767 = v0;
                if m1767 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1767 - 1;
                    let v4: i64 = {
                        let a1768 = v1;
                        let a1769 = 10i64;
                        let a1770 = 0i64;
                        int_quot(a1768, a1769, a1770)?
                    };
                    let v5: i64 = {
                        let a1771 = v1;
                        let a1772 = 10i64;
                        let a1773 = 0i64;
                        int_rem(a1771, a1772, a1773)
                    };
                    let m1776 = {
                        let a1774 = v4;
                        let a1775 = 1i64;
                        compare(a1774, a1775)
                    };
                    if match m1776 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f494(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f494(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1777 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: u64) -> R<u64> {
    let a1781 = v0;
    let a1782 = 10u64;
    if nat_lt(a1781, a1782) {
        let a1783 = 48u64;
        let a1784 = v0;
        nat_add(a1783, a1784)
    } else {
        let a1785 = 87u64;
        let a1786 = v0;
        nat_add(a1785, a1786)
    }
}

pub fn f446(v0: Adt8) -> Adt6 {
    Adt6::C5(f320(Str::lit("listChanged"), match v0 {
        Adt8::C0(h1787) => {
            h1787
        }
    }))
}

pub fn f447(p1792: u64, p1793: i64) -> R<i64> {
    let m1791 = (p1792, p1793);
    {
        let mut m1791 = m1791;
        loop {
            let step = {
                let (v0, v1) = m1791;
                let m1788 = v0;
                if m1788 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m1788 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a1789 = v1;
                        let a1790 = -1i64;
                        int_sub(a1789, a1790)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h1794, _, _, _, _, _, _, _) => {
            h1794
        }
    })), {
        let a1812 = f315(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h1795, _, _, _, _, _, _) => {
                h1795
            }
        });
        let a1813 = {
            let a1810 = f384(match v0.clone() {
                Adt35::C0(_, _, h1796, _, _, _, _, _) => {
                    h1796
                }
            });
            let a1811 = {
                let a1808 = f315(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h1797, _, _, _, _) => {
                        h1797
                    }
                });
                let a1809 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m1800 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h1799, _, _, _) => {
                            h1799
                        }
                    };
                    match m1800 {
                        Adt5::C0(h1798) => {
                            h1798
                        }
                    }
                })), {
                    let a1806 = f332(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h1801, _, _) => {
                            h1801
                        }
                    });
                    let a1807 = {
                        let a1804 = f495(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h1802, _) => {
                                h1802
                            }
                        });
                        let a1805 = f496(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h1803) => {
                                h1803
                            }
                        });
                        append_list(a1804, a1805)
                    };
                    append_list(a1806, a1807)
                });
                append_list(a1808, a1809)
            };
            append_list(a1810, a1811)
        };
        append_list(a1812, a1813)
    }))
}

pub fn f449(v0: Adt38) -> R<Option<Adt6>> {
    let m1814 = v0;
    match m1814 {
        Adt38::C0(v1) => {
            Ok(Some(f497(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f498(Str::lit("image"), match v2.clone() {
                Adt42::C0(h1815, _, _, _) => {
                    h1815
                }
            }, match v2.clone() {
                Adt42::C0(_, h1816, _, _) => {
                    h1816
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h1817, _) => {
                    h1817
                }
            }, match v2 {
                Adt42::C0(_, _, _, h1818) => {
                    h1818
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f498(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h1819, _, _, _) => {
                    h1819
                }
            }, match v3.clone() {
                Adt43::C0(_, h1820, _, _) => {
                    h1820
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h1821, _) => {
                    h1821
                }
            }, match v3 {
                Adt43::C0(_, _, _, h1822) => {
                    h1822
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f499(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f500(v5)))
        }
    }
}

pub fn f450(v0: List<Adt2>) -> List<Adt6> {
    f214(f501(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f451(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f502(v0, 0u64, v1)
}

pub fn f452(v0: Adt128) -> bool {
    let m1823 = v0;
    match m1823 {
        Adt128::C0 => {
            true
        }
        Adt128::C1 => {
            false
        }
        Adt128::C2 => {
            false
        }
        Adt128::C3 => {
            false
        }
        Adt128::C4 => {
            false
        }
        Adt128::C5 => {
            false
        }
        Adt128::C6 => {
            false
        }
        Adt128::C7 => {
            false
        }
        Adt128::C8 => {
            false
        }
        Adt128::C9 => {
            false
        }
        Adt128::C10 => {
            false
        }
        Adt128::C11 => {
            false
        }
    }
}

pub fn f453(v0: Adt14) -> Adt128 {
    let m1824 = v0;
    match m1824 {
        Adt14::C0(_) => {
            Adt128::C0
        }
        Adt14::C1(_) => {
            Adt128::C1
        }
        Adt14::C2(_) => {
            Adt128::C2
        }
        Adt14::C3(_) => {
            Adt128::C3
        }
        Adt14::C4(_) => {
            Adt128::C4
        }
        Adt14::C5(_) => {
            Adt128::C5
        }
        Adt14::C6(_) => {
            Adt128::C6
        }
        Adt14::C7(_) => {
            Adt128::C7
        }
        Adt14::C8(_) => {
            Adt128::C8
        }
        Adt14::C9(_) => {
            Adt128::C9
        }
        Adt14::C10(_) => {
            Adt128::C10
        }
        Adt14::C11(_) => {
            Adt128::C11
        }
    }
}

pub fn f454(v0: Adt0, v1: Adt32) -> Adt116 {
    Adt116::C0(v1, Adt117::C0(Some(match v0 {
        Adt0::C0(h1825, _, _, _, _, _) => {
            h1825
        }
    }), None::<Adt5>), List::<Adt118>::nil())
}

pub fn f455(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h1826, _, _) => {
            h1826
        }
    }, f109(), match v0.clone() {
        Adt0::C0(_, _, h1827, _, _, _) => {
            h1827
        }
    }, match v0 {
        Adt0::C0(_, h1828, _, _, _, _) => {
            h1828
        }
    })
}

pub fn f456(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt116, Adt119> {
    let m1829 = v2;
    match m1829 {
        Ok(v4) => {
            if f503(f453(v1.clone()), v4.clone()) {
                f504(v0, v1, v4)
            } else {
                Err::<Adt116, Adt119>(Adt119::C4(f505()))
            }
        }
        Err(v3) => {
            Err::<Adt116, Adt119>(Adt119::C4(v3))
        }
    }
}

pub fn f457(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m1830 = v3.clone();
    match m1830 {
        Adt14::C0(_) => {
            let c1832: Fn0 = v2;
            c1832.apply(v3)
        }
        Adt14::C1(_) => {
            let c1833: Fn0 = v2;
            c1833.apply(v3)
        }
        Adt14::C2(_) => {
            if f506(v0, v1) {
                let c1831: Fn0 = v2;
                c1831.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c1834: Fn0 = v2;
            c1834.apply(v3)
        }
        Adt14::C4(_) => {
            let c1835: Fn0 = v2;
            c1835.apply(v3)
        }
        Adt14::C5(_) => {
            let c1836: Fn0 = v2;
            c1836.apply(v3)
        }
        Adt14::C6(_) => {
            let c1837: Fn0 = v2;
            c1837.apply(v3)
        }
        Adt14::C7(_) => {
            let c1838: Fn0 = v2;
            c1838.apply(v3)
        }
        Adt14::C8(_) => {
            let c1839: Fn0 = v2;
            c1839.apply(v3)
        }
        Adt14::C9(_) => {
            let c1840: Fn0 = v2;
            c1840.apply(v3)
        }
        Adt14::C10(_) => {
            let c1841: Fn0 = v2;
            c1841.apply(v3)
        }
        Adt14::C11(_) => {
            let c1842: Fn0 = v2;
            c1842.apply(v3)
        }
    }
}

pub fn f458(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt120> {
    let m1843 = f111(v0.clone(), v1);
    match m1843 {
        Adt123::C0 => {
            Ok::<Option<Str>, Adt120>(None::<Str>)
        }
        Adt123::C1(v2) => {
            let m1844 = f151(v2);
            match m1844 {
                None => {
                    Err::<Option<Str>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt120>(Some(v3))
                }
            }
        }
    }
}

pub fn f459(v0: Str, v1: Adt5) -> Result<Str, Adt120> {
    let m1845 = f111(v0.clone(), v1);
    match m1845 {
        Adt123::C0 => {
            Err::<Str, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1846 = f151(v2);
            match m1846 {
                None => {
                    Err::<Str, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt120>(v3)
                }
            }
        }
    }
}

pub fn f460(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt120> {
    let m1847 = f111(v0.clone(), v1);
    match m1847 {
        Adt123::C0 => {
            Ok::<Option<Adt5>, Adt120>(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m1848 = f239(v2);
            match m1848 {
                None => {
                    Err::<Option<Adt5>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt120>(Some(v3))
                }
            }
        }
    }
}

pub fn f461(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt120> {
    let m1849 = f111(v0.clone(), v1);
    match m1849 {
        Adt123::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt120>(None::<List<(Str, Str)>>)
        }
        Adt123::C1(v2) => {
            let m1850 = f239(v2);
            match m1850 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    let m1852 = f507(v0, match v3 {
                        Adt5::C0(h1851) => {
                            h1851
                        }
                    });
                    match m1852 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt120>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt120>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f462(v0: Str, v1: Adt5) -> Result<Adt5, Adt120> {
    let m1853 = f111(v0.clone(), v1);
    match m1853 {
        Adt123::C0 => {
            Err::<Adt5, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1854 = f239(v2);
            match m1854 {
                None => {
                    Err::<Adt5, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt120>(v3)
                }
            }
        }
    }
}

pub fn f463(v0: Adt5) -> Result<Adt25, Adt120> {
    let m1855 = f459(Str::lit("type"), v0.clone());
    match m1855 {
        Ok(v1) => {
            let a1856 = v1.clone();
            let a1857 = Str::lit("ref/prompt");
            if equal(a1856, a1857) {
                let m1858 = f459(Str::lit("name"), v0.clone());
                match m1858 {
                    Ok(v2) => {
                        let m1859 = f458(Str::lit("title"), v0);
                        match m1859 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt120>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt120>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt120>(v5)
                    }
                }
            } else {
                let a1860 = v1;
                let a1861 = Str::lit("ref/resource");
                if equal(a1860, a1861) {
                    let m1862 = f459(Str::lit("uri"), v0);
                    match m1862 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt120>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt120>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt120>(Adt120::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt120>(v8)
        }
    }
}

pub fn f464(v0: Adt5) -> Result<Adt27, Adt120> {
    let m1863 = f459(Str::lit("name"), v0.clone());
    match m1863 {
        Ok(v1) => {
            let m1864 = f459(Str::lit("value"), v0);
            match m1864 {
                Ok(v2) => {
                    Ok::<Adt27, Adt120>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt120>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt120>(v4)
        }
    }
}

pub fn f465(v0: Adt5) -> Result<Option<Adt28>, Adt120> {
    let m1865 = f111(Str::lit("context"), v0);
    match m1865 {
        Adt123::C0 => {
            Ok::<Option<Adt28>, Adt120>(None::<Adt28>)
        }
        Adt123::C1(v1) => {
            let m1866 = f239(v1);
            match m1866 {
                None => {
                    Err::<Option<Adt28>, Adt120>(Adt120::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m1867 = f461(Str::lit("arguments"), v2);
                    match m1867 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt120>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt120>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f466(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt119> {
    if v0 {
        None::<Adt119>
    } else {
        Some(Adt119::C1(v1, v2))
    }
}

pub fn f467(v0: Str, p1872: List<Str>) -> bool {
    let m1871 = p1872;
    {
        let mut m1871 = m1871;
        loop {
            let step = {
                let v1 = m1871;
                let m1868 = v1;
                match m1868.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1869 = v2;
                        let a1870 = v0.clone();
                        if equal(a1869, a1870) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f468(v0: Option<Adt97>, v1: List<Adt99>, v2: Adt5) -> R<Option<Adt97>> {
    let m1873 = v0;
    match m1873 {
        None => {
            f508(v1, f509(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f469(v0: Option<Str>, v1: Adt138, v2: Adt5) -> Option<Adt97> {
    let m1874 = v1.clone();
    match m1874 {
        Adt138::C0 => {
            None::<Adt97>
        }
        Adt138::C1 => {
            f510(v0, f511(v1, v2))
        }
        Adt138::C2 => {
            f510(v0, f511(v1, v2))
        }
        Adt138::C3 => {
            f510(v0, f511(v1, v2))
        }
    }
}

pub fn f470(v0: Adt123, v1: Adt123) -> Adt145 {
    let m1875 = v0;
    match m1875 {
        Adt123::C0 => {
            Adt145::C1
        }
        Adt123::C1(_) => {
            let m1876 = v1;
            match m1876 {
                Adt123::C0 => {
                    Adt145::C2
                }
                Adt123::C1(_) => {
                    Adt145::C0
                }
            }
        }
    }
}

pub fn f471(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt120> {
    let m1877 = f151(v1);
    match m1877 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C5)
        }
        Some(v3) => {
            let m1878 = f512(v2);
            match m1878 {
                None => {
                    Err::<Adt16, Adt120>(Adt120::C6)
                }
                Some(v4) => {
                    let m1879 = f460(f513(), v0.clone());
                    match m1879 {
                        Ok(v6) => {
                            let m1880 = f514(v0.clone());
                            match m1880 {
                                Ok(v8) => {
                                    let m1881 = f515(v0.clone());
                                    match m1881 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt120>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt120>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt120>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt120>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f472() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f473(v0: u64, v1: u64) -> u64 {
    let a1882 = v0;
    let a1883 = v1;
    let a1884 = 0u64;
    nat_rem(a1882, a1883, a1884)
}

pub fn f474(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1887 = {
            let a1885 = v0;
            let a1886 = 4u64;
            nat_mul(a1885, a1886)?
        };
        let a1888 = f476(v1, 16u64);
        nat_add(a1887, a1888)?
    }, List::<u64>::nil()))
}

pub fn f475(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1891 = {
            let a1889 = v0;
            let a1890 = 4u64;
            nat_mul(a1889, a1890)?
        };
        let a1892 = f476(v1, 16u64);
        nat_add(a1891, a1892)?
    }, List::cons({
        let a1895 = {
            let a1893 = f473(v1, 16u64);
            let a1894 = 16u64;
            nat_mul(a1893, a1894)?
        };
        let a1896 = f476(v2, 4u64);
        nat_add(a1895, a1896)?
    }, List::<u64>::nil())))
}

pub fn f476(v0: u64, v1: u64) -> u64 {
    let a1897 = v0;
    let a1898 = v1;
    let a1899 = 0u64;
    nat_quot(a1897, a1898, a1899)
}

pub fn f477(v0: u64, v1: Adt6) -> bool {
    let a1900 = v0;
    let a1901 = 0u64;
    if nat_eq(a1900, a1901) {
        f516(v1)
    } else {
        let a1902 = v0;
        let a1903 = 1u64;
        if nat_eq(a1902, a1903) {
            f517(v1)
        } else {
            let a1904 = v0;
            let a1905 = 2u64;
            if nat_eq(a1904, a1905) {
                f518(v1)
            } else {
                let a1906 = v0;
                let a1907 = 3u64;
                if nat_eq(a1906, a1907) {
                    f519(v1)
                } else {
                    let a1908 = v0;
                    let a1909 = 4u64;
                    if nat_eq(a1908, a1909) {
                        f520(v1)
                    } else {
                        let a1910 = v0;
                        let a1911 = 5u64;
                        if nat_eq(a1910, a1911) {
                            f521(v1)
                        } else {
                            let a1912 = v0;
                            let a1913 = 6u64;
                            if nat_eq(a1912, a1913) {
                                f522(v1)
                            } else {
                                let a1914 = v0;
                                let a1915 = 7u64;
                                if nat_eq(a1914, a1915) {
                                    f523(v1)
                                } else {
                                    f524(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f478(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    let m1916 = v3.clone();
    match m1916.uncons() {
        None => {
            Ok(Adt144::C0(Adt94::C8(v0)))
        }
        Some((_, _)) => {
            if f232(v3.clone(), true) {
                let m1917 = f525(v1);
                match m1917 {
                    None => {
                        Ok(Adt144::C0(Adt94::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f526(v3)?;
                        if f527(v9.clone(), match v5.clone() {
                            Adt132::C0(_, h1918) => {
                                h1918
                            }
                        }) {
                            Ok(Adt144::C0(Adt94::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt132::C0(List::cons(Adt12::C0(v0, v2, f528(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt132::C0(h1919, _) => {
                                    h1919
                                }
                            }), List::cons(v9, match v5 {
                                Adt132::C0(_, h1920) => {
                                    h1920
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt144::C0(Adt94::C9(v0)))
            }
        }
    }
}

pub fn f479(p1925: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1926: List<Adt125>) -> List<Adt125> {
    let m1924 = (p1925, p1926);
    {
        let mut m1924 = m1924;
        loop {
            let step = {
                let (v0, v3) = m1924;
                let m1921 = v0;
                match m1921.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                            let (_, h1922) = v4;
                            h1922
                        }, v1, List::cons({
                            let (h1923, _) = v4;
                            h1923
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1924 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(p1930: List<(Str, Adt6)>, p1931: List<Adt125>) -> List<Adt125> {
    let m1929 = (p1930, p1931);
    {
        let mut m1929 = m1929;
        loop {
            let step = {
                let (v0, v1) = m1929;
                let m1927 = v0;
                match m1927.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0({
                            let (_, h1928) = v2;
                            h1928
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1929 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(p1934: List<Adt6>, p1935: List<Adt125>) -> List<Adt125> {
    let m1933 = (p1934, p1935);
    {
        let mut m1933 = m1933;
        loop {
            let step = {
                let (v0, v1) = m1933;
                let m1932 = v0;
                match m1932.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1933 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: Adt6) -> Str {
    let m1936 = v0;
    match m1936 {
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

pub fn f483(v0: Adt7) -> R<bool> {
    if f293(match v0.clone() {
        Adt7::C0(h1937, _) => {
            h1937
        }
    }) {
        Ok(true)
    } else {
        f529(40u64, match v0.clone() {
            Adt7::C0(h1938, _) => {
                h1938
            }
        }, match v0 {
            Adt7::C0(_, h1939) => {
                h1939
            }
        })
    }
}

pub fn f484(v0: Adt6) -> Str {
    let m1940 = f151(v0);
    match m1940 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f485(p1947: u64, p1948: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1946 = (p1947, p1948);
    {
        let mut m1946 = m1946;
        loop {
            let step = {
                let (v0, v1) = m1946;
                let m1941 = v0;
                if m1941 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1941 - 1;
                    let m1942 = v1;
                    match m1942.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1945 = f530({
                                let (h1943, _) = v3;
                                h1943
                            }, {
                                let (_, h1944) = v3;
                                h1944
                            }, v4)?;
                            match m1945 {
                                Adt149::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C1))
                                }
                                Adt149::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt146>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486() -> u64 {
    67108864u64
}

pub fn f487(v0: Adt147) -> bool {
    let m1949 = v0;
    match m1949 {
        Adt147::C0 => {
            true
        }
        Adt147::C1 => {
            false
        }
        Adt147::C2 => {
            false
        }
    }
}

pub fn f488(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1950 = v2;
    match m1950 {
        Adt147::C0 => {
            let m1952 = v3;
            match m1952 {
                Adt147::C0 => {
                    Ok(f531(Adt147::C0, f532(f533(match v0.clone() {
                        Adt7::C0(h1953, _) => {
                            h1953
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1954) => {
                            h1954
                        }
                    }, f533(match v1.clone() {
                        Adt7::C0(h1955, _) => {
                            h1955
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1956) => {
                            h1956
                        }
                    })?))
                }
                Adt147::C1 => {
                    Ok(Adt147::C0)
                }
                Adt147::C2 => {
                    Ok(Adt147::C0)
                }
            }
        }
        Adt147::C1 => {
            let m1951 = v3;
            match m1951 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C1)
                }
                Adt147::C2 => {
                    Ok(Adt147::C0)
                }
            }
        }
        Adt147::C2 => {
            let m1957 = v3;
            match m1957 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f532(match v0.clone() {
                        Adt7::C0(h1958, _) => {
                            h1958
                        }
                    }, match v0 {
                        Adt7::C0(_, h1959) => {
                            h1959
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1960, _) => {
                            h1960
                        }
                    }, match v1 {
                        Adt7::C0(_, h1961) => {
                            h1961
                        }
                    })
                }
            }
        }
    }
}

pub fn f489(v0: i64) -> Adt147 {
    f491(v0, 0i64)
}

pub fn f490(v0: Adt6) -> Option<i64> {
    let m1962 = v0;
    match m1962 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f293(match v2.clone() {
                Adt7::C0(_, h1963) => {
                    h1963
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1964, _) => {
                        h1964
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

pub fn f491(v0: i64, v1: i64) -> Adt147 {
    let m1967 = {
        let a1965 = v0;
        let a1966 = v1;
        compare(a1965, a1966)
    };
    if match m1967 {
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
        Adt147::C0
    } else {
        let m1970 = {
            let a1968 = v1;
            let a1969 = v0;
            compare(a1968, a1969)
        };
        if match m1970 {
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
            Adt147::C2
        } else {
            Adt147::C1
        }
    }
}

pub fn f492(p1979: List<u64>, p1980: u64) -> R<u64> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v1) = m1978;
                let m1971 = v0;
                match m1971.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1972 = v2;
                        let a1973 = 128u64;
                        if if nat_lt(a1972, a1973) {
                            true
                        } else {
                            let a1974 = 191u64;
                            let a1975 = v2;
                            nat_lt(a1974, a1975)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1976 = v1;
                                let a1977 = 1u64;
                                nat_add(a1976, a1977)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1978 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: Str, p1998: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1997 = p1998;
    {
        let mut m1997 = m1997;
        loop {
            let step = {
                let v1 = m1997;
                let m1981 = v1;
                match m1981.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1985 = v0.clone();
                        let a1986 = {
                            let a1983 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1982, _) = v2.clone();
                                h1982
                            }, List::<Str>::nil()));
                            let a1984 = Str::lit("");
                            join(a1983, a1984)
                        };
                        if equal(a1985, a1986) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1987) = v2;
                                h1987
                            }))
                        } else {
                            let m1989 = f239({
                                let (_, h1988) = v2.clone();
                                h1988
                            });
                            match m1989 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1990 = f111(Str::lit("$anchor"), v4);
                                    match m1990 {
                                        Adt123::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt123::C1(v5) => {
                                            let m1991 = f151(v5);
                                            match m1991 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1994 = v0.clone();
                                                    let a1995 = {
                                                        let a1992 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1993 = Str::lit("");
                                                        join(a1992, a1993)
                                                    };
                                                    if equal(a1994, a1995) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1996) = v2;
                                                            h1996
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
                Ok(next) => { m1997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: i64) -> u64 {
    let m2001 = {
        let a1999 = v0;
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
        48u64
    } else {
        let m2004 = {
            let a2002 = v0;
            let a2003 = 2i64;
            compare(a2002, a2003)
        };
        if match m2004 {
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
            let m2007 = {
                let a2005 = v0;
                let a2006 = 3i64;
                compare(a2005, a2006)
            };
            if match m2007 {
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
                let m2010 = {
                    let a2008 = v0;
                    let a2009 = 4i64;
                    compare(a2008, a2009)
                };
                if match m2010 {
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
                    let m2013 = {
                        let a2011 = v0;
                        let a2012 = 5i64;
                        compare(a2011, a2012)
                    };
                    if match m2013 {
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
                        let m2016 = {
                            let a2014 = v0;
                            let a2015 = 6i64;
                            compare(a2014, a2015)
                        };
                        if match m2016 {
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
                            let m2019 = {
                                let a2017 = v0;
                                let a2018 = 7i64;
                                compare(a2017, a2018)
                            };
                            if match m2019 {
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
                                let m2022 = {
                                    let a2020 = v0;
                                    let a2021 = 8i64;
                                    compare(a2020, a2021)
                                };
                                if match m2022 {
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
                                    let m2025 = {
                                        let a2023 = v0;
                                        let a2024 = 9i64;
                                        compare(a2023, a2024)
                                    };
                                    if match m2025 {
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

pub fn f495(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2026 = v0;
    match m2026 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2038 = f315(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2027, _, _, _, _) => {
                        h2027
                    }
                });
                let a2039 = {
                    let a2036 = f320(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2028, _, _, _) => {
                            h2028
                        }
                    });
                    let a2037 = {
                        let a2034 = f320(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2029, _, _) => {
                                h2029
                            }
                        });
                        let a2035 = {
                            let a2032 = f320(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2030, _) => {
                                    h2030
                                }
                            });
                            let a2033 = f320(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2031) => {
                                    h2031
                                }
                            });
                            append_list(a2032, a2033)
                        };
                        append_list(a2034, a2035)
                    };
                    append_list(a2036, a2037)
                };
                append_list(a2038, a2039)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f496(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f332(Str::lit("_meta"), v0)
}

pub fn f497(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2040, _, _) => {
            h2040
        }
    })), {
        let a2043 = f534(match v0.clone() {
            Adt39::C0(_, h2041, _) => {
                h2041
            }
        });
        let a2044 = f496(match v0 {
            Adt39::C0(_, _, h2042) => {
                h2042
            }
        });
        append_list(a2043, a2044)
    })))
}

pub fn f498(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2045 = f534(v3);
        let a2046 = f496(v4);
        append_list(a2045, a2046)
    }))))
}

pub fn f499(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2070 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2047, _, _, _, _, _, _, _, _) => {
                h2047
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2071 = {
            let a2068 = f315(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2048, _, _, _, _, _, _, _) => {
                    h2048
                }
            });
            let a2069 = {
                let a2066 = f384(match v0.clone() {
                    Adt44::C0(_, _, h2049, _, _, _, _, _, _) => {
                        h2049
                    }
                });
                let a2067 = {
                    let a2064 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2050, _, _, _, _, _) => {
                            h2050
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2065 = {
                        let a2062 = f315(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2051, _, _, _, _) => {
                                h2051
                            }
                        });
                        let a2063 = {
                            let a2060 = f315(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2052, _, _, _) => {
                                    h2052
                                }
                            });
                            let a2061 = {
                                let a2058 = f534(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2053, _, _) => {
                                        h2053
                                    }
                                });
                                let a2059 = {
                                    let a2056 = f535(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2054, _) => {
                                            h2054
                                        }
                                    })?;
                                    let a2057 = f496(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2055) => {
                                            h2055
                                        }
                                    });
                                    append_list(a2056, a2057)
                                };
                                append_list(a2058, a2059)
                            };
                            append_list(a2060, a2061)
                        };
                        append_list(a2062, a2063)
                    };
                    append_list(a2064, a2065)
                };
                append_list(a2066, a2067)
            };
            append_list(a2068, a2069)
        };
        append_list(a2070, a2071)
    }))
}

pub fn f500(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f536(match v0.clone() {
        Adt45::C0(h2072, _, _) => {
            h2072
        }
    })), {
        let a2075 = f534(match v0.clone() {
            Adt45::C0(_, h2073, _) => {
                h2073
            }
        });
        let a2076 = f496(match v0 {
            Adt45::C0(_, _, h2074) => {
                h2074
            }
        });
        append_list(a2075, a2076)
    })))
}

pub fn f501(p2079: List<Adt2>, p2080: List<Adt6>) -> List<Adt6> {
    let m2078 = (p2079, p2080);
    {
        let mut m2078 = m2078;
        loop {
            let step = {
                let (v0, v1) = m2078;
                let m2077 = v0;
                match m2077.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f537(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2078 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(p2085: List<u64>, p2086: u64, p2087: List<u64>) -> List<u64> {
    let m2084 = (p2085, (p2086, p2087));
    {
        let mut m2084 = m2084;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2084;
                let m2081 = v0;
                match m2081.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f538(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt148 = f539(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt148::C0(h2082, _) => {
                                h2082
                            }
                        }, match v5 {
                            Adt148::C0(_, h2083) => {
                                h2083
                            }
                        })))
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

pub fn f503(v0: Adt128, v1: Adt32) -> bool {
    let m2088 = v1;
    match m2088 {
        Adt32::C0(_) => {
            f540(v0, Adt128::C0)
        }
        Adt32::C1(_) => {
            f540(v0, Adt128::C1)
        }
        Adt32::C2(_) => {
            f540(v0, Adt128::C2)
        }
        Adt32::C3(_) => {
            f540(v0, Adt128::C2)
        }
        Adt32::C4(_) => {
            f540(v0, Adt128::C3)
        }
        Adt32::C5(_) => {
            f540(v0, Adt128::C4)
        }
        Adt32::C6(_) => {
            f540(v0, Adt128::C5)
        }
        Adt32::C7(_) => {
            f540(v0, Adt128::C6)
        }
        Adt32::C8(_) => {
            f540(v0, Adt128::C7)
        }
        Adt32::C9(_) => {
            f540(v0, Adt128::C8)
        }
        Adt32::C10(_) => {
            f540(v0, Adt128::C9)
        }
        Adt32::C11 => {
            if f540(v0.clone(), Adt128::C10) {
                true
            } else {
                f540(v0, Adt128::C11)
            }
        }
        Adt32::C12(_) => {
            f541(v0)
        }
        Adt32::C13(_) => {
            f542(v0)
        }
    }
}

pub fn f504(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt116, Adt119> {
    let m2089 = v2.clone();
    match m2089 {
        Adt32::C0(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt116, Adt119>(f546(v0, f544(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt116, Adt119>(f454(v0, v2))
        }
        Adt32::C12(v3) => {
            f543(v0, f544(v1), v3)
        }
        Adt32::C13(v4) => {
            f545(v0, f544(v1), v4)
        }
    }
}

pub fn f505() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f506(v0: u64, v1: u64) -> bool {
    let a2090 = v1;
    let a2091 = v0;
    nat_lt(a2090, a2091)
}

pub fn f507(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2092 = f547(v0, v1, List::<(Str, Str)>::nil());
    match m2092 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt120>(f548(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt120>(v3)
        }
    }
}

pub fn f508(p2096: List<Adt99>, v1: Adt5) -> R<Option<Adt97>> {
    let m2095 = p2096;
    {
        let mut m2095 = m2095;
        loop {
            let step = {
                let v0 = m2095;
                let m2093 = v0;
                match m2093.uncons() {
                    None => {
                        Err::<List<Adt99>, R<Option<Adt97>>>(Ok(None::<Adt97>))
                    }
                    Some((v2, v3)) => {
                        let m2094 = f549(v2, v1.clone())?;
                        match m2094 {
                            None => {
                                Ok::<List<Adt99>, R<Option<Adt97>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt99>, R<Option<Adt97>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2095 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: Adt5) -> Adt5 {
    let m2097 = f111(Str::lit("arguments"), v0);
    match m2097 {
        Adt123::C0 => {
            f402()
        }
        Adt123::C1(v1) => {
            let m2098 = f239(v1);
            match m2098 {
                None => {
                    f402()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f510(v0: Option<Str>, v1: Option<Str>) -> Option<Adt97> {
    let m2099 = v0;
    match m2099 {
        None => {
            Some(Adt97::C2)
        }
        Some(v2) => {
            let m2100 = v1;
            match m2100 {
                None => {
                    None::<Adt97>
                }
                Some(v3) => {
                    let a2101 = v2;
                    let a2102 = v3;
                    if equal(a2101, a2102) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C5)
                    }
                }
            }
        }
    }
}

pub fn f511(v0: Adt138, v1: Adt5) -> Option<Str> {
    let m2103 = v0;
    match m2103 {
        Adt138::C0 => {
            None::<Str>
        }
        Adt138::C1 => {
            let m2104 = f111(Str::lit("name"), v1);
            match m2104 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v2) => {
                    f151(v2)
                }
            }
        }
        Adt138::C2 => {
            let m2105 = f111(Str::lit("uri"), v1);
            match m2105 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v3) => {
                    f151(v3)
                }
            }
        }
        Adt138::C3 => {
            let m2106 = f111(Str::lit("taskId"), v1);
            match m2106 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v4) => {
                    f151(v4)
                }
            }
        }
    }
}

pub fn f512(v0: Adt6) -> Option<Adt17> {
    let m2107 = f239(v0);
    match m2107 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2108 = f550(Str::lit("experimental"), v1.clone());
            match m2108 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2109 = f550(Str::lit("roots"), v1.clone());
                    match m2109 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2110 = f550(Str::lit("sampling"), v1.clone());
                            match m2110 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2111 = f550(Str::lit("elicitation"), v1.clone());
                                    match m2111 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2112 = f550(Str::lit("extensions"), v1);
                                            match m2112 {
                                                None => {
                                                    None::<Adt17>
                                                }
                                                Some(v6) => {
                                                    Some(Adt17::C0(v2, v3, v4, v5, v6))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f513() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f514(v0: Adt5) -> Result<Option<Adt18>, Adt120> {
    let m2113 = f458(f551(), v0);
    match m2113 {
        Ok(v2) => {
            let m2114 = v2;
            match m2114 {
                None => {
                    Ok::<Option<Adt18>, Adt120>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2115 = f552(v3);
                    match m2115 {
                        Adt150::C0 => {
                            Err::<Option<Adt18>, Adt120>(Adt120::C8(f551()))
                        }
                        Adt150::C1(v4) => {
                            Ok::<Option<Adt18>, Adt120>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt120>(v1)
        }
    }
}

pub fn f515(v0: Adt5) -> Result<Option<Adt19>, Adt120> {
    let m2116 = f111(f553(), v0);
    match m2116 {
        Adt123::C0 => {
            Ok::<Option<Adt19>, Adt120>(None::<Adt19>)
        }
        Adt123::C1(v1) => {
            let m2117 = f151(v1.clone());
            match m2117 {
                None => {
                    let m2118 = f490(v1);
                    match m2118 {
                        None => {
                            Err::<Option<Adt19>, Adt120>(Adt120::C8(f553()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt120>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt120>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f516(v0: Adt6) -> bool {
    let m2119 = f151(v0);
    match m2119 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f517(v0: Adt6) -> bool {
    let m2120 = f428(v0);
    match m2120 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f518(v0: Adt6) -> bool {
    let m2121 = f490(v0);
    match m2121 {
        None => {
            false
        }
        Some(v1) => {
            let a2125 = {
                let m2124 = {
                    let a2122 = v1;
                    let a2123 = 0i64;
                    compare(a2122, a2123)
                };
                match m2124 {
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
            bool_not(a2125)
        }
    }
}

pub fn f519(v0: Adt6) -> bool {
    let m2126 = f304(v0);
    match m2126 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f520(v0: Adt6) -> bool {
    let m2127 = f239(v0);
    match m2127 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f521(v0: Adt6) -> bool {
    let m2128 = f304(v0);
    match m2128 {
        None => {
            false
        }
        Some(v1) => {
            f554(v1)
        }
    }
}

pub fn f522(v0: Adt6) -> bool {
    let m2129 = f238(v0);
    match m2129 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f523(v0: Adt6) -> bool {
    let m2130 = f151(v0.clone());
    match m2130 {
        None => {
            let m2131 = f304(v0);
            match m2131 {
                None => {
                    false
                }
                Some(v2) => {
                    f555(v2)
                }
            }
        }
        Some(v1) => {
            f423(v1)
        }
    }
}

pub fn f524(v0: Adt6) -> bool {
    let m2132 = f304(v0);
    match m2132 {
        None => {
            false
        }
        Some(v1) => {
            let m2133 = v1;
            match m2133.uncons() {
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

pub fn f525(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2134 = f154(Str::lit("type"), v0);
    match m2134 {
        Adt123::C0 => {
            None::<Adt13>
        }
        Adt123::C1(v1) => {
            let m2135 = f151(v1);
            match m2135 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f556(v2)
                }
            }
        }
    }
}

pub fn f526(v0: List<u64>) -> R<List<u64>> {
    Ok(f129(f557(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f527(v0: List<u64>, p2138: List<List<u64>>) -> bool {
    let m2137 = p2138;
    {
        let mut m2137 = m2137;
        loop {
            let step = {
                let v1 = m2137;
                let m2136 = v1;
                match m2136.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f558(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(p2141: List<Str>, p2142: List<Str>) -> List<Str> {
    let m2140 = (p2141, p2142);
    {
        let mut m2140 = m2140;
        loop {
            let step = {
                let (v0, v1) = m2140;
                let m2139 = v0;
                match m2139.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(p2154: u64, p2155: i64, p2156: i64) -> R<bool> {
    let m2153 = (p2154, (p2155, p2156));
    {
        let mut m2153 = m2153;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2153;
                let m2143 = v0;
                if m2143 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2143 - 1;
                    let m2146 = {
                        let a2144 = v2;
                        let a2145 = 0i64;
                        compare(a2144, a2145)
                    };
                    if match m2146 {
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
                        if f293({
                            let a2147 = v1;
                            let a2148 = 10i64;
                            let a2149 = 0i64;
                            int_rem(a2147, a2148, a2149)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2150 = v1;
                                let a2151 = 10i64;
                                let a2152 = 0i64;
                                int_quot(a2150, a2151, a2152)?
                            }, f123(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
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

pub fn f530(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2157 = f559(v0.clone());
    let a2158 = f559(v1.clone());
    if nat_eq(a2157, a2158) {
        let m2159 = v0;
        match m2159 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2160 = f238(v1);
                match m2160 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2161 = v3;
                        let a2162 = v4;
                        if equal(a2161, a2162) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2163 = f428(v1);
                match m2163 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f560(f430(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2164 = f151(v1);
                match m2164 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2165 = v7;
                        let a2166 = v8;
                        if equal(a2165, a2166) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2167 = f304(v1);
                match m2167 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f561(f562(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2168 = f239(v1);
                match m2168 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2170 = f434(v11.clone(), 0u64)?;
                        let a2171 = f434(match v12.clone() {
                            Adt5::C0(h2169) => {
                                h2169
                            }
                        }, 0u64)?;
                        if nat_eq(a2170, a2171) {
                            Ok(f561(f563(v11, match v12 {
                                Adt5::C0(h2172) => {
                                    h2172
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt149::C0)
    }
}

pub fn f531(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2173 = v0;
    match m2173 {
        Adt147::C0 => {
            f564(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f532(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2174 = f491(f565(v0, v1)?, f565(v2, v3)?);
    match m2174 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f566(v0, f567(v0)?, v2, f567(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f533(v0: i64) -> R<i64> {
    let m2177 = {
        let a2175 = v0;
        let a2176 = 0i64;
        compare(a2175, a2176)
    };
    if match m2177 {
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
        let a2178 = v0;
        int_neg(a2178)
    } else {
        Ok(v0)
    }
}

pub fn f534(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2179 = v0;
    match m2179 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2185 = f568(match v1.clone() {
                    Adt40::C0(h2180, _, _) => {
                        h2180
                    }
                });
                let a2186 = {
                    let a2183 = f569(match v1.clone() {
                        Adt40::C0(_, h2181, _) => {
                            h2181
                        }
                    });
                    let a2184 = f315(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2182) => {
                            h2182
                        }
                    });
                    append_list(a2183, a2184)
                };
                append_list(a2185, a2186)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f535(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2187 = v0;
    match m2187 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f380(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f536(v0: Adt46) -> Adt6 {
    let m2188 = v0;
    match m2188 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2189, _, _, _) => {
                    h2189
                }
            })), {
                let a2193 = f315(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2190, _, _) => {
                        h2190
                    }
                });
                let a2194 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2191, _) => {
                        h2191
                    }
                })), f496(match v1 {
                    Adt47::C0(_, _, _, h2192) => {
                        h2192
                    }
                }));
                append_list(a2193, a2194)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2195, _, _, _) => {
                    h2195
                }
            })), {
                let a2199 = f315(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2196, _, _) => {
                        h2196
                    }
                });
                let a2200 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2197, _) => {
                        h2197
                    }
                })), f496(match v2 {
                    Adt48::C0(_, _, _, h2198) => {
                        h2198
                    }
                }));
                append_list(a2199, a2200)
            }))
        }
    }
}

pub fn f537(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2201, _, _, _) => {
            h2201
        }
    })), {
        let a2207 = f315(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2202, _, _) => {
                h2202
            }
        });
        let a2208 = {
            let a2205 = f570(match v0.clone() {
                Adt2::C0(_, _, h2203, _) => {
                    h2203
                }
            });
            let a2206 = f571(match v0 {
                Adt2::C0(_, _, _, h2204) => {
                    h2204
                }
            });
            append_list(a2205, a2206)
        };
        append_list(a2207, a2208)
    }))
}

pub fn f538(v0: u64, v1: List<u64>) -> List<u64> {
    let a2209 = v0;
    let a2210 = 0u64;
    if nat_eq(a2209, a2210) {
        v1
    } else {
        let a2211 = v0;
        let a2212 = 1u64;
        if nat_eq(a2211, a2212) {
            List::cons(194u64, v1)
        } else {
            let a2213 = v0;
            let a2214 = 2u64;
            if nat_eq(a2213, a2214) {
                List::cons(226u64, v1)
            } else {
                let a2215 = v0;
                let a2216 = 3u64;
                if nat_eq(a2215, a2216) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f539(v0: u64, v1: u64, v2: List<u64>) -> Adt148 {
    let a2217 = v0;
    let a2218 = 0u64;
    if nat_eq(a2217, a2218) {
        f572(v1, v2)
    } else {
        let a2219 = v0;
        let a2220 = 1u64;
        if nat_eq(a2219, a2220) {
            let a2221 = 128u64;
            let a2222 = v1;
            if if nat_le(a2221, a2222) {
                let a2223 = v1;
                let a2224 = 159u64;
                nat_le(a2223, a2224)
            } else {
                false
            } {
                Adt148::C0(0u64, f573(v2))
            } else {
                f572(v1, List::cons(194u64, v2))
            }
        } else {
            let a2225 = v0;
            let a2226 = 2u64;
            if nat_eq(a2225, a2226) {
                let a2227 = v1;
                let a2228 = 128u64;
                if nat_eq(a2227, a2228) {
                    Adt148::C0(3u64, v2)
                } else {
                    let a2229 = v1;
                    let a2230 = 129u64;
                    if nat_eq(a2229, a2230) {
                        Adt148::C0(4u64, v2)
                    } else {
                        f572(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2231 = v0;
                let a2232 = 3u64;
                if nat_eq(a2231, a2232) {
                    let a2233 = 170u64;
                    let a2234 = v1;
                    if if nat_le(a2233, a2234) {
                        let a2235 = v1;
                        let a2236 = 174u64;
                        nat_le(a2235, a2236)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f573(v2))
                    } else {
                        f572(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2237 = 166u64;
                    let a2238 = v1;
                    if if nat_le(a2237, a2238) {
                        let a2239 = v1;
                        let a2240 = 169u64;
                        nat_le(a2239, a2240)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f573(v2))
                    } else {
                        f572(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f540(v0: Adt128, v1: Adt128) -> bool {
    let a2241 = f335(v0);
    let a2242 = f335(v1);
    equal(a2241, a2242)
}

pub fn f541(v0: Adt128) -> bool {
    let m2243 = v0;
    match m2243 {
        Adt128::C0 => {
            false
        }
        Adt128::C1 => {
            false
        }
        Adt128::C2 => {
            true
        }
        Adt128::C3 => {
            false
        }
        Adt128::C4 => {
            true
        }
        Adt128::C5 => {
            false
        }
        Adt128::C6 => {
            false
        }
        Adt128::C7 => {
            true
        }
        Adt128::C8 => {
            false
        }
        Adt128::C9 => {
            false
        }
        Adt128::C10 => {
            false
        }
        Adt128::C11 => {
            false
        }
    }
}

pub fn f542(v0: Adt128) -> bool {
    let m2244 = v0;
    match m2244 {
        Adt128::C0 => {
            false
        }
        Adt128::C1 => {
            false
        }
        Adt128::C2 => {
            true
        }
        Adt128::C3 => {
            false
        }
        Adt128::C4 => {
            false
        }
        Adt128::C5 => {
            false
        }
        Adt128::C6 => {
            false
        }
        Adt128::C7 => {
            false
        }
        Adt128::C8 => {
            false
        }
        Adt128::C9 => {
            false
        }
        Adt128::C10 => {
            false
        }
        Adt128::C11 => {
            false
        }
    }
}

pub fn f543(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt116, Adt119> {
    f574(v0, v2.clone(), f575(match v1 {
        Adt16::C0(_, h2245, _, _, _, _) => {
            h2245
        }
    }, f576(v2)))
}

pub fn f544(v0: Adt14) -> Adt16 {
    let m2246 = v0;
    match m2246 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2247) => {
                    h2247
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2248, _) => {
                    h2248
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2249, _, _, _, _) => {
                    h2249
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2250, _) => {
                    h2250
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2251, _, _, _) => {
                    h2251
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2252, _) => {
                    h2252
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2253, _) => {
                    h2253
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2254, _, _, _, _) => {
                    h2254
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2255, _, _, _) => {
                    h2255
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2256, _) => {
                    h2256
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2257, _, _) => {
                    h2257
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2258, _) => {
                    h2258
                }
            }
        }
    }
}

pub fn f545(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt116, Adt119> {
    f577(v0, v2, f578(match v1 {
        Adt16::C0(_, h2259, _, _, _, _) => {
            h2259
        }
    }))
}

pub fn f546(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt116 {
    Adt116::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2260, _) => {
            h2260
        }
    }), {
        let m2262 = f454(v0, Adt32::C11);
        match m2262 {
            Adt116::C0(_, h2261, _) => {
                h2261
            }
        }
    }, f579(v1, match v2 {
        Adt49::C0(_, h2263) => {
            h2263
        }
    }))
}

pub fn f547(v0: Str, p2269: List<(Str, Adt6)>, p2270: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2268 = (p2269, p2270);
    {
        let mut m2268 = m2268;
        loop {
            let step = {
                let (v1, v2) = m2268;
                let m2264 = v1;
                match m2264.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Ok::<List<(Str, Str)>, Adt120>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2266 = f151({
                            let (_, h2265) = v3;
                            h2265
                        });
                        match m2266 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Err::<List<(Str, Str)>, Adt120>(Adt120::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>((v4, List::cons(({
                                    let (h2267, _) = v3;
                                    h2267
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(p2273: List<(Str, Str)>, p2274: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2272 = (p2273, p2274);
    {
        let mut m2272 = m2272;
        loop {
            let step = {
                let (v0, v1) = m2272;
                let m2271 = v0;
                match m2271.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549(v0: Adt99, v1: Adt5) -> R<Option<Adt97>> {
    let m2276 = f580(match v0.clone() {
        Adt99::C0(_, h2275, _, _) => {
            h2275
        }
    }, v1);
    match m2276 {
        Adt123::C0 => {
            Ok(f581(v0))
        }
        Adt123::C1(v2) => {
            if f582(v2.clone()) {
                Ok(f581(v0))
            } else {
                f583(v0, v2)
            }
        }
    }
}

pub fn f550(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2277 = f111(v0, v1);
    match m2277 {
        Adt123::C0 => {
            Some(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m2278 = f239(v2);
            match m2278 {
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

pub fn f551() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f552(v0: Str) -> Adt150 {
    let a2279 = v0.clone();
    let a2280 = Str::lit("debug");
    if equal(a2279, a2280) {
        Adt150::C1(Adt18::C0)
    } else {
        let a2281 = v0.clone();
        let a2282 = Str::lit("info");
        if equal(a2281, a2282) {
            Adt150::C1(Adt18::C1)
        } else {
            let a2283 = v0.clone();
            let a2284 = Str::lit("notice");
            if equal(a2283, a2284) {
                Adt150::C1(Adt18::C2)
            } else {
                let a2285 = v0.clone();
                let a2286 = Str::lit("warning");
                if equal(a2285, a2286) {
                    Adt150::C1(Adt18::C3)
                } else {
                    let a2287 = v0.clone();
                    let a2288 = Str::lit("error");
                    if equal(a2287, a2288) {
                        Adt150::C1(Adt18::C4)
                    } else {
                        let a2289 = v0.clone();
                        let a2290 = Str::lit("critical");
                        if equal(a2289, a2290) {
                            Adt150::C1(Adt18::C5)
                        } else {
                            let a2291 = v0.clone();
                            let a2292 = Str::lit("alert");
                            if equal(a2291, a2292) {
                                Adt150::C1(Adt18::C6)
                            } else {
                                let a2293 = v0;
                                let a2294 = Str::lit("emergency");
                                if equal(a2293, a2294) {
                                    Adt150::C1(Adt18::C7)
                                } else {
                                    Adt150::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f553() -> Str {
    Str::lit("progressToken")
}

pub fn f554(p2298: List<Adt6>) -> bool {
    let m2297 = p2298;
    {
        let mut m2297 = m2297;
        loop {
            let step = {
                let v0 = m2297;
                let m2295 = v0;
                match m2295.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2296 = f151(v1);
                        match m2296 {
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
                Ok(next) => { m2297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f555(p2302: List<Adt6>) -> bool {
    let m2301 = p2302;
    {
        let mut m2301 = m2301;
        loop {
            let step = {
                let v0 = m2301;
                let m2299 = v0;
                match m2299.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2300 = f151(v1);
                        match m2300 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f423(v3) {
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
                Ok(next) => { m2301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: Str) -> Option<Adt13> {
    let a2303 = v0.clone();
    let a2304 = Str::lit("string");
    if equal(a2303, a2304) {
        Some(Adt13::C0)
    } else {
        let a2305 = v0.clone();
        let a2306 = Str::lit("integer");
        if equal(a2305, a2306) {
            Some(Adt13::C1)
        } else {
            let a2307 = v0;
            let a2308 = Str::lit("boolean");
            if equal(a2307, a2308) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f557(p2317: List<u64>, p2318: List<u64>) -> R<List<u64>> {
    let m2316 = (p2317, p2318);
    {
        let mut m2316 = m2316;
        loop {
            let step = {
                let (v0, v1) = m2316;
                let m2309 = v0;
                match m2309.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2310 = 65u64;
                        let a2311 = v2;
                        if if nat_le(a2310, a2311) {
                            let a2312 = v2;
                            let a2313 = 90u64;
                            nat_le(a2312, a2313)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2314 = v2;
                                let a2315 = 32u64;
                                nat_add(a2314, a2315)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f558(p2325: List<u64>, p2326: List<u64>) -> bool {
    let m2324 = (p2325, p2326);
    {
        let mut m2324 = m2324;
        loop {
            let step = {
                let (v0, v1) = m2324;
                let m2319 = v0;
                match m2319.uncons() {
                    None => {
                        let m2320 = v1;
                        match m2320.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2321 = v1;
                        match m2321.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2322 = v4;
                                let a2323 = v6;
                                if nat_eq(a2322, a2323) {
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
                Ok(next) => { m2324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: Adt6) -> u64 {
    let m2327 = v0;
    match m2327 {
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

pub fn f560(v0: Adt147) -> bool {
    let m2328 = v0;
    match m2328 {
        Adt147::C0 => {
            false
        }
        Adt147::C1 => {
            true
        }
        Adt147::C2 => {
            false
        }
    }
}

pub fn f561(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2329 = v0;
    match m2329 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2330 = v2;
                let a2331 = v1;
                append_list(a2330, a2331)
            })
        }
    }
}

pub fn f562(p2336: List<Adt6>, p2337: List<Adt6>, p2338: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2335 = (p2336, (p2337, p2338));
    {
        let mut m2335 = m2335;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2335;
                let m2332 = v0;
                match m2332.uncons() {
                    None => {
                        let m2333 = v1;
                        match m2333.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2334 = v1;
                        match m2334.uncons() {
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
                Ok(next) => { m2335 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f563(p2344: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2345: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2343 = (p2344, p2345);
    {
        let mut m2343 = m2343;
        loop {
            let step = {
                let (v0, v2) = m2343;
                let m2339 = v0;
                match m2339.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2341 = f154({
                            let (h2340, _) = v3;
                            h2340
                        }, v1.clone());
                        match m2341 {
                            Adt123::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt123::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2342) = v3;
                                    h2342
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f564(v0: Adt147) -> Adt147 {
    let m2346 = v0;
    match m2346 {
        Adt147::C0 => {
            Adt147::C2
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            Adt147::C0
        }
    }
}

pub fn f565(v0: i64, v1: i64) -> R<i64> {
    f123(f89(f567(v0)?, 0i64)?, v1)
}

pub fn f566(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2347 = v1;
    let a2348 = v3;
    if nat_lt(a2347, a2348) {
        Ok(f491(f584({
            let a2349 = v3;
            let a2350 = v1;
            nat_sub(a2349, a2350)
        }, v0)?, v2))
    } else {
        let a2351 = v3;
        let a2352 = v1;
        if nat_lt(a2351, a2352) {
            Ok(f491(v0, f584({
                let a2353 = v1;
                let a2354 = v3;
                nat_sub(a2353, a2354)
            }, v2)?))
        } else {
            Ok(f491(v0, v2))
        }
    }
}

pub fn f567(v0: i64) -> R<u64> {
    f585(20u64, v0, 0u64)
}

pub fn f568(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2355 = v0;
    match m2355 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f586(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f569(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2356 = v0;
    match m2356 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f570(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2357 = v0;
    match m2357 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f272(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f571(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2358 = v0;
    match m2358 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f587(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f572(v0: u64, v1: List<u64>) -> Adt148 {
    let a2359 = v0;
    let a2360 = 32u64;
    if nat_lt(a2359, a2360) {
        if f588(v0) {
            Adt148::C0(0u64, List::cons(v0, v1))
        } else {
            Adt148::C0(0u64, f573(v1))
        }
    } else {
        let a2361 = v0;
        let a2362 = 127u64;
        if nat_eq(a2361, a2362) {
            Adt148::C0(0u64, f573(v1))
        } else {
            let a2363 = v0;
            let a2364 = 194u64;
            if nat_eq(a2363, a2364) {
                Adt148::C0(1u64, v1)
            } else {
                let a2365 = v0;
                let a2366 = 226u64;
                if nat_eq(a2365, a2366) {
                    Adt148::C0(2u64, v1)
                } else {
                    Adt148::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f573(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f574(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f454(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f589(f576(v1)))))
    }
}

pub fn f575(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2367 = f590(v1.clone());
    if if if bool_not(a2367) {
        true
    } else {
        f338(match v0.clone() {
            Adt17::C0(_, _, _, h2368, _) => {
                h2368
            }
        })
    } {
        let a2369 = f591(v1.clone());
        if bool_not(a2369) {
            true
        } else {
            f338(match v0.clone() {
                Adt17::C0(_, _, h2370, _, _) => {
                    h2370
                }
            })
        }
    } else {
        false
    } {
        let a2371 = f592(v1);
        if bool_not(a2371) {
            true
        } else {
            f338(match v0 {
                Adt17::C0(_, h2372, _, _, _) => {
                    h2372
                }
            })
        }
    } else {
        false
    }
}

pub fn f576(v0: Adt81) -> List<(Str, Adt64)> {
    let m2373 = v0;
    match m2373 {
        Adt81::C0(v1) => {
            v1
        }
        Adt81::C1(_) => {
            List::<(Str, Adt64)>::nil()
        }
        Adt81::C2(v3, _) => {
            v3
        }
    }
}

pub fn f577(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f454(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f593())))
    }
}

pub fn f578(v0: Adt17) -> bool {
    f339(match v0 {
        Adt17::C0(_, _, _, _, h2374) => {
            h2374
        }
    }, f340())
}

pub fn f579(v0: Adt16, v1: List<Adt50>) -> List<Adt118> {
    let m2376 = match v0 {
        Adt16::C0(_, _, _, _, h2375, _) => {
            h2375
        }
    };
    match m2376 {
        None => {
            List::<Adt118>::nil()
        }
        Some(v2) => {
            f594(f595(v2, v1, List::<Adt118>::nil()), List::<Adt118>::nil())
        }
    }
}

pub fn f580(p2382: List<Str>, p2383: Adt5) -> Adt123 {
    let m2381 = (p2382, p2383);
    {
        let mut m2381 = m2381;
        loop {
            let step = {
                let (v0, v1) = m2381;
                let m2377 = v0;
                match m2377.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m2378 = f111(v2, v1);
                        match m2378 {
                            Adt123::C0 => {
                                Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                            }
                            Adt123::C1(v4) => {
                                let m2379 = v3.clone();
                                match m2379.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2380 = f239(v4);
                                        match m2380 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt123>((v3, v7))
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
                Ok(next) => { m2381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(v0: Adt99) -> Option<Adt97> {
    let m2385 = match v0.clone() {
        Adt99::C0(_, _, _, h2384) => {
            h2384
        }
    };
    match m2385 {
        Adt100::C0 => {
            None::<Adt97>
        }
        Adt100::C1 => {
            Some(Adt97::C10(match v0 {
                Adt99::C0(h2386, _, _, _) => {
                    h2386
                }
            }))
        }
        Adt100::C2(_) => {
            None::<Adt97>
        }
    }
}

pub fn f582(v0: Adt6) -> bool {
    let m2387 = v0;
    match m2387 {
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

pub fn f583(v0: Adt99, v1: Adt6) -> R<Option<Adt97>> {
    let m2389 = match v0.clone() {
        Adt99::C0(_, _, _, h2388) => {
            h2388
        }
    };
    match m2389 {
        Adt100::C0 => {
            Ok(Some(Adt97::C8(match v0 {
                Adt99::C0(h2390, _, _, _) => {
                    h2390
                }
            })))
        }
        Adt100::C1 => {
            Ok(Some(Adt97::C10(match v0 {
                Adt99::C0(h2391, _, _, _) => {
                    h2391
                }
            })))
        }
        Adt100::C2(v2) => {
            if f596(match v0.clone() {
                Adt99::C0(_, _, h2392, _) => {
                    h2392
                }
            }, v2, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(Adt97::C9(match v0 {
                    Adt99::C0(h2393, _, _, _) => {
                        h2393
                    }
                })))
            }
        }
    }
}

pub fn f584(p2398: u64, p2399: i64) -> R<i64> {
    let m2397 = (p2398, p2399);
    {
        let mut m2397 = m2397;
        loop {
            let step = {
                let (v0, v1) = m2397;
                let m2394 = v0;
                if m2394 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2394 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2395 = v1;
                        let a2396 = 10i64;
                        int_mul(a2395, a2396)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f585(p2410: u64, p2411: i64, p2412: u64) -> R<u64> {
    let m2409 = (p2410, (p2411, p2412));
    {
        let mut m2409 = m2409;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2409;
                let m2400 = v0;
                if m2400 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2400 - 1;
                    let v4: u64 = {
                        let a2401 = v2;
                        let a2402 = 1u64;
                        nat_add(a2401, a2402)?
                    };
                    let m2405 = {
                        let a2403 = v1;
                        let a2404 = 10i64;
                        compare(a2403, a2404)
                    };
                    if match m2405 {
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
                            let a2406 = v1;
                            let a2407 = 10i64;
                            let a2408 = 0i64;
                            int_quot(a2406, a2407, a2408)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: List<Adt41>) -> List<Adt6> {
    f214(f597(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f587(v0: Adt3) -> Str {
    let m2413 = v0;
    match m2413 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f588(v0: u64) -> bool {
    let a2414 = v0;
    let a2415 = 9u64;
    if nat_eq(a2414, a2415) {
        true
    } else {
        let a2416 = v0;
        let a2417 = 10u64;
        if nat_eq(a2416, a2417) {
            true
        } else {
            let a2418 = v0;
            let a2419 = 13u64;
            nat_eq(a2418, a2419)
        }
    }
}

pub fn f589(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f592(v0.clone()) {
        Some(f402())
    } else {
        None::<Adt5>
    }, if f591(v0.clone()) {
        Some(f402())
    } else {
        None::<Adt5>
    }, if f590(v0) {
        Some(f402())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f590(p2424: List<(Str, Adt64)>) -> bool {
    let m2423 = p2424;
    {
        let mut m2423 = m2423;
        loop {
            let step = {
                let v0 = m2423;
                let m2420 = v0;
                match m2420.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2422 = {
                            let (_, h2421) = v1;
                            h2421
                        };
                        match m2422 {
                            Adt64::C0(_) => {
                                Err::<List<(Str, Adt64)>, bool>(true)
                            }
                            Adt64::C1(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                            Adt64::C2(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(p2429: List<(Str, Adt64)>) -> bool {
    let m2428 = p2429;
    {
        let mut m2428 = m2428;
        loop {
            let step = {
                let v0 = m2428;
                let m2425 = v0;
                match m2425.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2427 = {
                            let (_, h2426) = v1;
                            h2426
                        };
                        match m2427 {
                            Adt64::C0(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                            Adt64::C1(_) => {
                                Err::<List<(Str, Adt64)>, bool>(true)
                            }
                            Adt64::C2(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2428 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(p2434: List<(Str, Adt64)>) -> bool {
    let m2433 = p2434;
    {
        let mut m2433 = m2433;
        loop {
            let step = {
                let v0 = m2433;
                let m2430 = v0;
                match m2430.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2432 = {
                            let (_, h2431) = v1;
                            h2431
                        };
                        match m2432 {
                            Adt64::C0(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                            Adt64::C1(_) => {
                                Ok::<List<(Str, Adt64)>, bool>(v2)
                            }
                            Adt64::C2(_) => {
                                Err::<List<(Str, Adt64)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2433 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f340(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f594(p2437: List<Adt118>, p2438: List<Adt118>) -> List<Adt118> {
    let m2436 = (p2437, p2438);
    {
        let mut m2436 = m2436;
        loop {
            let step = {
                let (v0, v1) = m2436;
                let m2435 = v0;
                match m2435.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Adt118>), List<Adt118>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: Adt19, p2441: List<Adt50>, p2442: List<Adt118>) -> List<Adt118> {
    let m2440 = (p2441, p2442);
    {
        let mut m2440 = m2440;
        loop {
            let step = {
                let (v1, v2) = m2440;
                let m2439 = v1;
                match m2439.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt118>), List<Adt118>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt118>), List<Adt118>>((v4, List::cons(Adt118::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f596(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2443 = v0;
    match m2443 {
        Adt13::C0 => {
            let m2444 = f151(v2);
            match m2444 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2445 = v1;
                    let a2446 = v3;
                    Ok(equal(a2445, a2446))
                }
            }
        }
        Adt13::C1 => {
            f598(v1, v2)
        }
        Adt13::C2 => {
            let m2447 = f238(v2);
            match m2447 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2448 = v1;
                        let a2449 = Str::lit("true");
                        Ok(equal(a2448, a2449))
                    } else {
                        let a2450 = v1;
                        let a2451 = Str::lit("false");
                        Ok(equal(a2450, a2451))
                    }
                }
            }
        }
    }
}

pub fn f597(p2454: List<Adt41>, p2455: List<Adt6>) -> List<Adt6> {
    let m2453 = (p2454, p2455);
    {
        let mut m2453 = m2453;
        loop {
            let step = {
                let (v0, v1) = m2453;
                let m2452 = v0;
                match m2452.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f599(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: Str, v1: Adt6) -> R<bool> {
    let m2456 = f428(v1);
    match m2456 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2457 = f153(v2)?;
            match m2457 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2459 = f600(f55({
                        let a2458 = v0;
                        utf8_encode(a2458)
                    })?)?;
                    match m2459 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f293({
                                let a2460 = v3;
                                let a2461 = v4;
                                int_sub(a2460, a2461)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f599(v0: Adt41) -> Str {
    let m2462 = v0;
    match m2462 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f600(v0: List<u64>) -> R<Option<i64>> {
    let m2463 = v0.clone();
    match m2463.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2464 = v1;
            let a2465 = 45u64;
            if nat_eq(a2464, a2465) {
                let m2466 = v2.clone();
                match m2466.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2467 = f601(v2, 0i64)?;
                        match m2467 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2468 = v5;
                                    int_neg(a2468)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f601(v0, 0i64)
            }
        }
    }
}

pub fn f601(p2474: List<u64>, p2475: i64) -> R<Option<i64>> {
    let m2473 = (p2474, p2475);
    {
        let mut m2473 = m2473;
        loop {
            let step = {
                let (v0, v1) = m2473;
                let m2469 = v0;
                match m2469.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2470 = f602(v2);
                        match m2470 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f123({
                                    let a2471 = v1;
                                    let a2472 = 10i64;
                                    int_mul(a2471, a2472)?
                                }, f213(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2473 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f602(v0: u64) -> Option<u64> {
    let a2476 = v0;
    let a2477 = 48u64;
    if if nat_lt(a2476, a2477) {
        true
    } else {
        let a2478 = 57u64;
        let a2479 = v0;
        nat_lt(a2478, a2479)
    } {
        None::<u64>
    } else {
        Some({
            let a2480 = v0;
            let a2481 = 48u64;
            nat_sub(a2480, a2481)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
