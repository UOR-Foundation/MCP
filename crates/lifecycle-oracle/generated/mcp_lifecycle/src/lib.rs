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
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>),
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
    C4(Adt50),
    C5(Adt51),
    C6(Adt53),
    C7(Adt56),
    C8(Adt58),
    C9(Adt60),
    C10,
    C11(Adt79),
    C12(Adt80),
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
    C0(Adt10, Option<Str>, List<Adt44>),
}

#[derive(Clone)]
pub enum Adt50 {
    C0(Adt10, List<Adt46>),
}

#[derive(Clone)]
pub enum Adt51 {
    C0(Adt10, Option<Str>, List<Adt52>),
}

#[derive(Clone)]
pub enum Adt52 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Str, Option<Str>, Option<Str>, Option<Adt40>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt53 {
    C0(Adt10, Option<Str>, List<Adt54>),
}

#[derive(Clone)]
pub enum Adt54 {
    C0(Str, Option<Str>, Option<List<Adt2>>, Option<Str>, Option<List<Adt55>>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt55 {
    C0(Str, Option<Str>, Option<Str>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt56 {
    C0(Option<Str>, List<Adt57>),
}

#[derive(Clone)]
pub enum Adt57 {
    C0(Adt41, Adt38),
}

#[derive(Clone)]
pub enum Adt58 {
    C0(Adt59),
}

#[derive(Clone)]
pub enum Adt59 {
    C0(List<Str>, Option<u64>, Option<bool>),
}

#[derive(Clone)]
pub enum Adt60 {
    C0(Adt61),
    C1(Adt61, List<(Str, Adt62)>),
    C2(Adt61, Adt5),
    C3(Adt61, Adt78),
    C4(Adt61),
}

#[derive(Clone)]
pub enum Adt61 {
    C0(Str, Option<Str>, Str, Str, Option<u64>, Option<u64>),
}

#[derive(Clone)]
pub enum Adt62 {
    C0(Adt63),
    C1(Adt67),
    C2(Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt63 {
    C0(Adt64),
    C1(Adt66),
}

#[derive(Clone)]
pub enum Adt64 {
    C0(Option<Adt65>, Str, Adt5),
}

#[derive(Clone)]
pub enum Adt65 {
    C0,
}

#[derive(Clone)]
pub enum Adt66 {
    C0(Str, Str),
}

#[derive(Clone)]
pub enum Adt67 {
    C0(List<Adt68>, Option<Adt73>, Option<Str>, Option<Adt75>, Option<Adt7>, u64, Option<List<Str>>, Option<Adt5>, Option<List<Adt35>>, Option<Adt76>),
}

#[derive(Clone)]
pub enum Adt68 {
    C0(Adt41, Adt69, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt69 {
    C0(Adt70),
    C1(List<Adt70>),
}

#[derive(Clone)]
pub enum Adt70 {
    C0(Adt39),
    C1(Adt42),
    C2(Adt43),
    C3(Adt71),
    C4(Adt72),
}

#[derive(Clone)]
pub enum Adt71 {
    C0(Str, Str, Adt5, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt72 {
    C0(Str, List<Adt38>, Option<Adt6>, Option<bool>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt73 {
    C0(Option<List<Adt74>>, Option<Adt7>, Option<Adt7>, Option<Adt7>),
}

#[derive(Clone)]
pub enum Adt74 {
    C0(Option<Str>),
}

#[derive(Clone)]
pub enum Adt75 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt76 {
    C0(Option<Adt77>),
}

#[derive(Clone)]
pub enum Adt77 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt78 {
    C0(i64, Str, Option<Adt6>),
}

#[derive(Clone)]
pub enum Adt79 {
    C0(List<(Str, Adt62)>),
    C1(Str),
    C2(List<(Str, Adt62)>, Str),
}

#[derive(Clone)]
pub enum Adt80 {
    C0(Adt81, Adt61),
}

#[derive(Clone)]
pub enum Adt81 {
    C0,
    C1,
    C2,
    C3,
    C4,
}

#[derive(Clone)]
pub enum Adt82 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Adt17),
    C6(Str),
}

#[derive(Clone)]
pub enum Adt83 {
    C0(List<u64>, List<u64>, List<Adt84>, Bytes),
}

#[derive(Clone)]
pub enum Adt84 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt85 {
    C0(u64),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt86 {
    C0(u64, List<Adt84>, List<u64>),
}

#[derive(Clone)]
pub enum Adt87 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt88 {
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
pub enum Adt89 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt90 {
    C0,
    C1(Option<Adt89>, Adt91),
}

#[derive(Clone)]
pub enum Adt91 {
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
pub enum Adt92 {
    C0(Adt89, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt94>, List<Adt96>),
}

#[derive(Clone)]
pub enum Adt94 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt95),
    C7(Adt95),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt95 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt96 {
    C0(Str, List<Str>, Adt13, Adt97),
}

#[derive(Clone)]
pub enum Adt97 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt98 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt99 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt100 {
    C0(Adt101, List<Adt105>, u64),
}

#[derive(Clone)]
pub enum Adt101 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt102, List<u64>, bool),
    C7(u64, u64),
    C8(Adt103),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt102 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt103 {
    C0(bool, Adt104, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt104 {
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
pub enum Adt105 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt107 {
    C0(Adt89, Result<Adt108, Adt110>),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Adt32, Adt109),
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Adt111),
    C1(Str, List<Str>),
    C2(Adt94),
    C3(Str),
    C4(Adt82),
}

#[derive(Clone)]
pub enum Adt111 {
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
pub enum Adt112 {
    C0(Option<Str>, Option<Adt94>),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1(Adt89),
    C2,
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Adt88),
    C1(Adt100),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(Adt89, Adt117, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt117 {
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
pub enum Adt118 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt119 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt120 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Adt88),
    C1(List<Adt106>, List<u64>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1(Adt117),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Adt124),
    C1(Adt110),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt89, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt125 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt127 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Fn0 {
    F3,
}

impl Fn0 {
    pub fn apply(&self, p0: Adt14) -> Result<Adt32, Adt82> {
        match self {
            Fn0::F3 => f3(p0),
        }
    }
}

pub fn f0(v0: Bytes) -> R<List<u64>> {
    f1(f2(), Fn0::F3, v0)
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

pub fn f2() -> Adt0 {
    Adt0::C0(f8(), None::<Str>, f9(), f10(), f11())
}

pub fn f3(v0: Adt14) -> Result<Adt32, Adt82> {
    let m2 = v0;
    match m2 {
        Adt14::C0(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C1(_) => {
            Ok::<Adt32, Adt82>(Adt32::C1(Adt34::C0(f10(), None::<Str>, f12())))
        }
        Adt14::C2(v2) => {
            f13(v2)
        }
        Adt14::C3(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C4(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C5(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C6(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C7(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C8(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C9(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C10(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
        Adt14::C11(_) => {
            Err::<Adt32, Adt82>(Adt82::C6(Str::lit("the fixture offers tools only")))
        }
    }
}

pub fn f4(v0: Bytes) -> R<Result<Adt83, Adt85>> {
    let m3 = f14(v0.clone(), 0u64)?;
    match m3 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0(0u64)))
        }
        Some(v1) => {
            f15(v0.clone(), v1.clone(), f14(v0, match v1 {
                Adt87::C0(_, h4) => {
                    h4
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt86) -> List<u64> {
    let a18 = f16(match v0.clone() {
        Adt86::C0(h5, _, _) => {
            h5
        }
    });
    let a19 = {
        let a16 = f16({
            let a7 = match v0.clone() {
                Adt86::C0(_, h6, _) => {
                    h6
                }
            };
            length_list(a7)
        });
        let a17 = {
            let a14 = f17(match v0.clone() {
                Adt86::C0(_, h8, _) => {
                    h8
                }
            });
            let a15 = {
                let a12 = f16({
                    let a10 = match v0.clone() {
                        Adt86::C0(_, _, h9) => {
                            h9
                        }
                    };
                    length_list(a10)
                });
                let a13 = match v0 {
                    Adt86::C0(_, _, h11) => {
                        h11
                    }
                };
                append_list(a12, a13)
            };
            append_list(a14, a15)
        };
        append_list(a16, a17)
    };
    append_list(a18, a19)
}

pub fn f6() -> Adt86 {
    Adt86::C0(f18(), f19(), f20())
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt83) -> R<Adt86> {
    if f21(match v2.clone() {
        Adt83::C0(h20, _, _, _) => {
            h20
        }
    }) {
        f22(v0, v1, match v2.clone() {
            Adt83::C0(_, _, h21, _) => {
                h21
            }
        }, f23(match v2 {
            Adt83::C0(_, _, _, h22) => {
                h22
            }
        })?)
    } else {
        Ok(f24())
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

pub fn f11() -> List<Adt12> {
    List::cons(Adt12::C0(Str::lit("execute_sql"), Str::lit("Region"), List::cons(Str::lit("region"), List::<Str>::nil()), Adt13::C0), List::cons(Adt12::C0(Str::lit("execute_sql"), Str::lit("Limit"), List::cons(Str::lit("limit"), List::<Str>::nil()), Adt13::C1), List::cons(Adt12::C0(Str::lit("execute_sql"), Str::lit("Dry-Run"), List::cons(Str::lit("dry_run"), List::<Str>::nil()), Adt13::C2), List::<Adt12>::nil())))
}

pub fn f12() -> List<Adt35> {
    List::cons(f25(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f26(List::<(Str, Adt6)>::nil(), List::<Str>::nil())), List::cons(f25(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f26(List::cons((Str::lit("region"), f27(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f28()), List::cons((Str::lit("limit"), f27(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f27(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))), List::cons(Str::lit("region"), List::cons(Str::lit("query"), List::<Str>::nil())))), List::cons(f25(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f26(List::<(Str, Adt6)>::nil(), List::<Str>::nil())), List::<Adt35>::nil())))
}

pub fn f13(v0: Adt21) -> Result<Adt32, Adt82> {
    let a24 = match v0.clone() {
        Adt21::C0(_, h23, _, _, _) => {
            h23
        }
    };
    let a25 = Str::lit("echo");
    if equal(a24, a25) {
        Ok::<Adt32, Adt82>(Adt32::C2(f29()))
    } else {
        let a27 = match v0.clone() {
            Adt21::C0(_, h26, _, _, _) => {
                h26
            }
        };
        let a28 = Str::lit("execute_sql");
        if equal(a27, a28) {
            Ok::<Adt32, Adt82>(Adt32::C2(f29()))
        } else {
            let a30 = match v0.clone() {
                Adt21::C0(_, h29, _, _, _) => {
                    h29
                }
            };
            let a31 = Str::lit("test_missing_capability");
            if equal(a30, a31) {
                let m37 = {
                    let m36 = {
                        let m35 = match v0 {
                            Adt21::C0(h34, _, _, _, _) => {
                                h34
                            }
                        };
                        match m35 {
                            Adt16::C0(_, h33, _, _, _, _) => {
                                h33
                            }
                        }
                    };
                    match m36 {
                        Adt17::C0(_, _, h32, _, _) => {
                            h32
                        }
                    }
                };
                match m37 {
                    None => {
                        Err::<Adt32, Adt82>(Adt82::C5(f30()))
                    }
                    Some(_) => {
                        Ok::<Adt32, Adt82>(Adt32::C2(f29()))
                    }
                }
            } else {
                Err::<Adt32, Adt82>(Adt82::C0(match v0 {
                    Adt21::C0(_, h38, _, _, _) => {
                        h38
                    }
                }))
            }
        }
    }
}

pub fn f14(v0: Bytes, v1: u64) -> R<Option<Adt87>> {
    let m39 = f31(v0.clone(), v1)?;
    match m39 {
        None => {
            Ok(None::<Adt87>)
        }
        Some(v2) => {
            let m45 = {
                let a42 = v0;
                let a43 = {
                    let a40 = v1;
                    let a41 = 4u64;
                    nat_add(a40, a41)?
                };
                let a44 = v2;
                slice_bytes(a42, a43, a44)
            };
            match m45 {
                None => {
                    Ok(None::<Adt87>)
                }
                Some(v3) => {
                    Ok(Some(Adt87::C0(v3, {
                        let a48 = {
                            let a46 = v1;
                            let a47 = 4u64;
                            nat_add(a46, a47)?
                        };
                        let a49 = v2;
                        nat_add(a48, a49)?
                    })))
                }
            }
        }
    }
}

pub fn f15(v0: Bytes, v1: Adt87, v2: Option<Adt87>) -> R<Result<Adt83, Adt85>> {
    let m50 = v2;
    match m50 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0(match v1 {
                Adt87::C0(_, h51) => {
                    h51
                }
            })))
        }
        Some(v3) => {
            f32(v0.clone(), v1, v3.clone(), f31(v0, match v3 {
                Adt87::C0(_, h52) => {
                    h52
                }
            })?)
        }
    }
}

pub fn f16(v0: u64) -> List<u64> {
    List::cons({
        let a56 = {
            let a53 = v0;
            let a54 = 16777216u64;
            let a55 = 0u64;
            nat_quot(a53, a54, a55)
        };
        let a57 = 256u64;
        let a58 = 0u64;
        nat_rem(a56, a57, a58)
    }, List::cons({
        let a62 = {
            let a59 = v0;
            let a60 = 65536u64;
            let a61 = 0u64;
            nat_quot(a59, a60, a61)
        };
        let a63 = 256u64;
        let a64 = 0u64;
        nat_rem(a62, a63, a64)
    }, List::cons({
        let a68 = {
            let a65 = v0;
            let a66 = 256u64;
            let a67 = 0u64;
            nat_quot(a65, a66, a67)
        };
        let a69 = 256u64;
        let a70 = 0u64;
        nat_rem(a68, a69, a70)
    }, List::cons({
        let a71 = v0;
        let a72 = 256u64;
        let a73 = 0u64;
        nat_rem(a71, a72, a73)
    }, List::<u64>::nil()))))
}

pub fn f17(v0: List<Adt84>) -> List<u64> {
    let m74 = v0;
    match m74.uncons() {
        None => {
            List::<u64>::nil()
        }
        Some((v1, v2)) => {
            let a75 = f33(v1);
            let a76 = f17(v2);
            append_list(a75, a76)
        }
    }
}

pub fn f18() -> u64 {
    500u64
}

pub fn f19() -> List<Adt84> {
    List::cons(Adt84::C0(f34(), f35()), List::<Adt84>::nil())
}

pub fn f20() -> List<u64> {
    let a79 = f36();
    let a80 = {
        let a77 = f37();
        let a78 = f38();
        append_list(a77, a78)
    };
    append_list(a79, a80)
}

pub fn f21(v0: List<u64>) -> bool {
    f39(v0, f40())
}

pub fn f22(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Result<Adt6, Adt88>) -> R<Adt86> {
    let m81 = v3;
    match m81 {
        Ok(v5) => {
            f45(v0, v1, v2, f46(v5)?)
        }
        Err(_) => {
            f41(f42(), f43(None::<Adt89>, f44(Adt90::C0)))
        }
    }
}

pub fn f23(v0: Bytes) -> R<Result<Adt6, Adt88>> {
    f47(f48(v0)?)
}

pub fn f24() -> Adt86 {
    Adt86::C0(f49(), List::cons(Adt84::C0(f50(), f51()), List::<Adt84>::nil()), List::<u64>::nil())
}

pub fn f25(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f26(v0: List<(Str, Adt6)>, v1: List<Str>) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(v0)), List::cons((Str::lit("required"), Adt6::C4(f52(v1))), List::<(Str, Adt6)>::nil()))))
}

pub fn f27(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f28() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f29() -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(Str::lit("ok"), None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, None::<bool>)
}

pub fn f30() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f53()), None::<Adt5>, None::<Adt5>)
}

pub fn f31(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a85 = {
        let a82 = v1;
        let a83 = 4u64;
        nat_add(a82, a83)?
    };
    let a86 = {
        let a84 = v0.clone();
        length_bytes(a84)
    };
    if nat_le(a85, a86) {
        Ok(Some({
            let a103 = {
                let a87 = f54(v0.clone(), v1)?;
                let a88 = 16777216u64;
                nat_mul(a87, a88)?
            };
            let a104 = {
                let a101 = {
                    let a91 = f54(v0.clone(), {
                        let a89 = v1;
                        let a90 = 1u64;
                        nat_add(a89, a90)?
                    })?;
                    let a92 = 65536u64;
                    nat_mul(a91, a92)?
                };
                let a102 = {
                    let a99 = {
                        let a95 = f54(v0.clone(), {
                            let a93 = v1;
                            let a94 = 2u64;
                            nat_add(a93, a94)?
                        })?;
                        let a96 = 256u64;
                        nat_mul(a95, a96)?
                    };
                    let a100 = f54(v0, {
                        let a97 = v1;
                        let a98 = 3u64;
                        nat_add(a97, a98)?
                    })?;
                    nat_add(a99, a100)?
                };
                nat_add(a101, a102)?
            };
            nat_add(a103, a104)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f32(v0: Bytes, v1: Adt87, v2: Adt87, v3: Option<u64>) -> R<Result<Adt83, Adt85>> {
    let m105 = v3;
    match m105 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0(match v2 {
                Adt87::C0(_, h106) => {
                    h106
                }
            })))
        }
        Some(v4) => {
            let m110 = f55(v0.clone(), {
                let a108 = match v2.clone() {
                    Adt87::C0(_, h107) => {
                        h107
                    }
                };
                let a109 = 4u64;
                nat_add(a108, a109)?
            }, v4, List::<Adt84>::nil())?;
            match m110 {
                Ok(v6) => {
                    f56(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt83, Adt85>(v5))
                }
            }
        }
    }
}

pub fn f33(v0: Adt84) -> List<u64> {
    let a121 = f16({
        let a112 = match v0.clone() {
            Adt84::C0(h111, _) => {
                h111
            }
        };
        length_list(a112)
    });
    let a122 = {
        let a119 = match v0.clone() {
            Adt84::C0(h113, _) => {
                h113
            }
        };
        let a120 = {
            let a117 = f16({
                let a115 = match v0.clone() {
                    Adt84::C0(_, h114) => {
                        h114
                    }
                };
                length_list(a115)
            });
            let a118 = match v0 {
                Adt84::C0(_, h116) => {
                    h116
                }
            };
            append_list(a117, a118)
        };
        append_list(a119, a120)
    };
    append_list(a121, a122)
}

pub fn f34() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f35() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f36() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::cons(44u64, List::cons(34u64, List::cons(105u64, List::cons(100u64, List::cons(34u64, List::cons(58u64, List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f37() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f38() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f39(p129: List<u64>, p130: List<u64>) -> bool {
    let m128 = (p129, p130);
    {
        let mut m128 = m128;
        loop {
            let step = {
                let (v0, v1) = m128;
                let m123 = v0;
                match m123.uncons() {
                    None => {
                        let m124 = v1;
                        match m124.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m125 = v1;
                        match m125.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a126 = v4;
                                let a127 = v6;
                                if nat_eq(a126, a127) {
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
                Ok(next) => { m128 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f40() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f41(v0: u64, v1: Adt6) -> R<Adt86> {
    let m131 = f57(v1)?;
    match m131 {
        Ok(v3) => {
            Ok(Adt86::C0(v0, f19(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f42() -> u64 {
    400u64
}

pub fn f43(v0: Option<Adt89>, v1: Adt78) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f58())), List::cons((Str::lit("id"), f59(v0)), List::cons((Str::lit("error"), f60(v1)), List::<(Str, Adt6)>::nil()))))
}

pub fn f44(v0: Adt90) -> Adt78 {
    Adt78::C0(f61(v0.clone()), f62(v0), None::<Adt6>)
}

pub fn f45(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Result<Adt92, Adt90>) -> R<Adt86> {
    let m132 = v3;
    match m132 {
        Ok(v5) => {
            f65(v0, v1, v2, v5)
        }
        Err(v4) => {
            f41(f63(v4.clone()), f43(f64(v4.clone()), f44(v4)))
        }
    }
}

pub fn f46(v0: Adt6) -> R<Result<Adt92, Adt90>> {
    let m133 = v0;
    match m133 {
        Adt6::C0 => {
            Ok(f67(None::<Adt89>, Adt91::C0))
        }
        Adt6::C1(_) => {
            Ok(f67(None::<Adt89>, Adt91::C0))
        }
        Adt6::C2(_) => {
            Ok(f67(None::<Adt89>, Adt91::C0))
        }
        Adt6::C3(_) => {
            Ok(f67(None::<Adt89>, Adt91::C0))
        }
        Adt6::C4(_) => {
            Ok(f67(None::<Adt89>, Adt91::C1))
        }
        Adt6::C5(v1) => {
            f66(Adt5::C0(v1))
        }
    }
}

pub fn f47(v0: List<u64>) -> R<Result<Adt6, Adt88>> {
    let m134 = f68(v0.clone())?;
    match m134 {
        Adt98::C0 => {
            f69(f70(), 0u64, v0)
        }
        Adt98::C1(v1) => {
            Ok(Err::<Adt6, Adt88>(Adt88::C0(v1)))
        }
        Adt98::C2(v2) => {
            Ok(Err::<Adt6, Adt88>(Adt88::C1(v2)))
        }
    }
}

pub fn f48(v0: Bytes) -> R<List<u64>> {
    f71(v0.clone(), {
        let a135 = v0;
        length_bytes(a135)
    }, List::<u64>::nil())
}

pub fn f49() -> u64 {
    405u64
}

pub fn f50() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f51() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f52(v0: List<Str>) -> List<Adt6> {
    let m136 = v0;
    match m136.uncons() {
        None => {
            List::<Adt6>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt6::C3(v1), f52(v2))
        }
    }
}

pub fn f53() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f54(v0: Bytes, v1: u64) -> R<u64> {
    let m139 = {
        let a137 = v0;
        let a138 = v1;
        index_bytes(a137, a138)
    };
    match m139 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a184 = {
                let a160 = {
                    let a148 = {
                        let a142 = {
                            let a140 = v3;
                            let a141 = 128u8;
                            fixed_u8::bit_and(a140, a141)
                        };
                        let a143 = 128u8;
                        if equal(a142, a143) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a149 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 64u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 64u8;
                        if equal(a146, a147) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a148, a149)?
                };
                let a161 = {
                    let a158 = {
                        let a152 = {
                            let a150 = v3;
                            let a151 = 32u8;
                            fixed_u8::bit_and(a150, a151)
                        };
                        let a153 = 32u8;
                        if equal(a152, a153) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a159 = {
                        let a156 = {
                            let a154 = v3;
                            let a155 = 16u8;
                            fixed_u8::bit_and(a154, a155)
                        };
                        let a157 = 16u8;
                        if equal(a156, a157) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a158, a159)?
                };
                nat_add(a160, a161)?
            };
            let a185 = {
                let a182 = {
                    let a170 = {
                        let a164 = {
                            let a162 = v3;
                            let a163 = 8u8;
                            fixed_u8::bit_and(a162, a163)
                        };
                        let a165 = 8u8;
                        if equal(a164, a165) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a171 = {
                        let a168 = {
                            let a166 = v3;
                            let a167 = 4u8;
                            fixed_u8::bit_and(a166, a167)
                        };
                        let a169 = 4u8;
                        if equal(a168, a169) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a170, a171)?
                };
                let a183 = {
                    let a180 = {
                        let a174 = {
                            let a172 = v3;
                            let a173 = 2u8;
                            fixed_u8::bit_and(a172, a173)
                        };
                        let a175 = 2u8;
                        if equal(a174, a175) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a181 = {
                        let a178 = {
                            let a176 = v3;
                            let a177 = 1u8;
                            fixed_u8::bit_and(a176, a177)
                        };
                        let a179 = 1u8;
                        if equal(a178, a179) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a180, a181)?
                };
                nat_add(a182, a183)?
            };
            nat_add(a184, a185)
        }
    }
}

pub fn f55(v0: Bytes, p195: u64, p196: u64, p197: List<Adt84>) -> R<Result<(List<Adt84>, u64), Adt85>> {
    let m194 = (p195, (p196, p197));
    {
        let mut m194 = m194;
        loop {
            let step = {
                let (v1, (v2, v3)) = m194;
                let m186 = v2;
                if m186 == 0 {
                    Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Ok::<(List<Adt84>, u64), Adt85>((f72(v3, List::<Adt84>::nil()), v1))))
                } else {
                    let v4 = m186 - 1;
                    let m187 = f14(v0.clone(), v1)?;
                    match m187 {
                        None => {
                            Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Err::<(List<Adt84>, u64), Adt85>(Adt85::C0(v1))))
                        }
                        Some(v5) => {
                            let m189 = f14(v0.clone(), match v5.clone() {
                                Adt87::C0(_, h188) => {
                                    h188
                                }
                            })?;
                            match m189 {
                                None => {
                                    Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Err::<(List<Adt84>, u64), Adt85>(Adt85::C0(match v5 {
                                        Adt87::C0(_, h190) => {
                                            h190
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>((match v6.clone() {
                                        Adt87::C0(_, h191) => {
                                            h191
                                        }
                                    }, (v4, List::cons(Adt84::C0(f48(match v5 {
                                        Adt87::C0(h192, _) => {
                                            h192
                                        }
                                    })?, f48(match v6 {
                                        Adt87::C0(h193, _) => {
                                            h193
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Bytes, v1: Adt87, v2: Adt87, v3: (List<Adt84>, u64)) -> R<Result<Adt83, Adt85>> {
    let m199 = f14(v0.clone(), {
        let (_, h198) = v3.clone();
        h198
    })?;
    match m199 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0({
                let (_, h200) = v3;
                h200
            })))
        }
        Some(v4) => {
            let a203 = match v4.clone() {
                Adt87::C0(_, h201) => {
                    h201
                }
            };
            let a204 = {
                let a202 = v0;
                length_bytes(a202)
            };
            if nat_eq(a203, a204) {
                Ok(Ok::<Adt83, Adt85>(Adt83::C0(f48(match v1 {
                    Adt87::C0(h205, _) => {
                        h205
                    }
                })?, f48(match v2 {
                    Adt87::C0(h206, _) => {
                        h206
                    }
                })?, {
                    let (h207, _) = v3;
                    h207
                }, match v4 {
                    Adt87::C0(h208, _) => {
                        h208
                    }
                })))
            } else {
                Ok(Err::<Adt83, Adt85>(Adt85::C1(match v4 {
                    Adt87::C0(_, h209) => {
                        h209
                    }
                })))
            }
        }
    }
}

pub fn f57(v0: Adt6) -> R<Result<List<u64>, Adt88>> {
    f73(f74(), v0)
}

pub fn f58() -> Str {
    Str::lit("2.0")
}

pub fn f59(v0: Option<Adt89>) -> Adt6 {
    let m210 = v0;
    match m210 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f75(v1)
        }
    }
}

pub fn f60(v0: Adt78) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt78::C0(h211, _, _) => {
            h211
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt78::C0(_, h212, _) => {
            h212
        }
    })), f76(match v0 {
        Adt78::C0(_, _, h213) => {
            h213
        }
    }))))
}

pub fn f61(v0: Adt90) -> i64 {
    let m214 = v0;
    match m214 {
        Adt90::C0 => {
            f77()
        }
        Adt90::C1(_, _) => {
            f78()
        }
    }
}

pub fn f62(v0: Adt90) -> Str {
    let m215 = v0;
    match m215 {
        Adt90::C0 => {
            Str::lit("Parse error")
        }
        Adt90::C1(_, v2) => {
            f79(v2)
        }
    }
}

pub fn f63(v0: Adt90) -> u64 {
    let m216 = v0;
    match m216 {
        Adt90::C0 => {
            f42()
        }
        Adt90::C1(_, _) => {
            f42()
        }
    }
}

pub fn f64(v0: Adt90) -> Option<Adt89> {
    let m217 = v0;
    match m217 {
        Adt90::C0 => {
            None::<Adt89>
        }
        Adt90::C1(v1, _) => {
            v1
        }
    }
}

pub fn f65(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Adt92) -> R<Adt86> {
    f80(f81(v0.clone(), v1, Some(f82(v2, match v0 {
        Adt0::C0(_, _, _, _, h218) => {
            h218
        }
    }, f83(v3.clone()), match v3.clone() {
        Adt92::C0(_, h219, _) => {
            h219
        }
    })?), f84(), v3)?)
}

pub fn f66(v0: Adt5) -> R<Result<Adt92, Adt90>> {
    let m220 = f85(v0.clone())?;
    match m220 {
        Adt113::C0 => {
            let m221 = f86(Str::lit("method"), v0.clone());
            match m221 {
                Adt114::C0 => {
                    Ok(f67(None::<Adt89>, f87(v0)))
                }
                Adt114::C1(_) => {
                    Ok(f67(None::<Adt89>, if f88(v0) {
                        Adt91::C8
                    } else {
                        Adt91::C7
                    }))
                }
            }
        }
        Adt113::C1(v2) => {
            Ok(f89(v2, v0))
        }
        Adt113::C2 => {
            Ok(f67(None::<Adt89>, Adt91::C5))
        }
    }
}

pub fn f67(v0: Option<Adt89>, v1: Adt91) -> Result<Adt92, Adt90> {
    Err::<Adt92, Adt90>(Adt90::C1(v0, v1))
}

pub fn f68(v0: List<u64>) -> R<Adt98> {
    Ok(f90({
        let a222 = v0.clone();
        length_list(a222)
    }, f91(Adt99::C0, 0u64, 0u64, v0)?))
}

pub fn f69(p229: Adt100, p230: u64, p231: List<u64>) -> R<Result<Adt6, Adt88>> {
    let m228 = (p229, (p230, p231));
    {
        let mut m228 = m228;
        loop {
            let step = {
                let (v0, (v1, v2)) = m228;
                let m223 = v2;
                match m223.uncons() {
                    None => {
                        let m224 = f92(v0, v1)?;
                        match m224 {
                            Ok(v3) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Ok::<Adt6, Adt88>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Err::<Adt6, Adt88>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m225 = f93(v0, v1, v5)?;
                        match m225 {
                            Adt115::C0(v7) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Err::<Adt6, Adt88>(v7)))
                            }
                            Adt115::C1(v8) => {
                                Ok::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>((v8, ({
                                    let a226 = v1;
                                    let a227 = 1u64;
                                    nat_add(a226, a227)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70() -> Adt100 {
    Adt100::C0(Adt101::C0, List::<Adt105>::nil(), 0u64)
}

pub fn f71(v0: Bytes, p234: u64, p235: List<u64>) -> R<List<u64>> {
    let m233 = (p234, p235);
    {
        let mut m233 = m233;
        loop {
            let step = {
                let (v1, v2) = m233;
                let m232 = v1;
                if m232 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m232 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f54(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(p238: List<Adt84>, p239: List<Adt84>) -> List<Adt84> {
    let m237 = (p238, p239);
    {
        let mut m237 = m237;
        loop {
            let step = {
                let (v0, v1) = m237;
                let m236 = v0;
                match m236.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt84>), List<Adt84>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt84>, List<Adt84>), List<Adt84>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt88>> {
    let m240 = f94(v0, List::cons(Adt106::C1(0u64, v1), List::<Adt106>::nil()), List::<u64>::nil())?;
    match m240 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt88>(f95(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt88>(v3))
        }
    }
}

pub fn f74() -> u64 {
    67108864u64
}

pub fn f75(v0: Adt89) -> Adt6 {
    let m241 = v0;
    match m241 {
        Adt89::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt89::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f76(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m242 = v0;
    match m242 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f77() -> i64 {
    -32700i64
}

pub fn f78() -> i64 {
    -32600i64
}

pub fn f79(v0: Adt91) -> Str {
    let m243 = v0;
    match m243 {
        Adt91::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt91::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt91::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt91::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt91::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt91::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt91::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt91::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt91::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f80(v0: Adt107) -> R<Adt86> {
    let m245 = match v0.clone() {
        Adt107::C0(_, h244) => {
            h244
        }
    };
    match m245 {
        Ok(v1) => {
            let m246 = f96(v1)?;
            match m246 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    f41(f97(), f98(match v0 {
                        Adt107::C0(h247, _) => {
                            h247
                        }
                    }, v2))
                }
            }
        }
        Err(v3) => {
            f41(f99(v3.clone()), f43(Some(match v0 {
                Adt107::C0(h248, _) => {
                    h248
                }
            }), f100(v3)))
        }
    }
}

pub fn f81(v0: Adt0, v1: Fn0, v2: Option<Adt93>, v3: List<Str>, v4: Adt92) -> R<Adt107> {
    Ok(Adt107::C0(match v4.clone() {
        Adt92::C0(h249, _, _) => {
            h249
        }
    }, f101(v0, v1, f102(v2, v3, v4)?)))
}

pub fn f82(v0: List<Adt84>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt93> {
    let v4: Adt112 = f103(Adt95::C0, f104(v0.clone(), f105())?);
    let v5: Adt112 = f103(Adt95::C1, f104(v0.clone(), f106())?);
    let v6: Adt112 = if f107(v3) {
        f103(Adt95::C2, f108(f104(v0.clone(), f109())?)?)
    } else {
        f110()
    };
    Ok(Adt93::C0(match v4.clone() {
        Adt112::C0(h250, _) => {
            h250
        }
    }, match v5.clone() {
        Adt112::C0(h251, _) => {
            h251
        }
    }, match v6.clone() {
        Adt112::C0(h252, _) => {
            h252
        }
    }, f111(match v4 {
        Adt112::C0(_, h253) => {
            h253
        }
    }, f111(match v5 {
        Adt112::C0(_, h254) => {
            h254
        }
    }, match v6 {
        Adt112::C0(_, h255) => {
            h255
        }
    })), f112(v0, v1, v2)?))
}

pub fn f83(v0: Adt92) -> Option<Str> {
    let a257 = match v0.clone() {
        Adt92::C0(_, h256, _) => {
            h256
        }
    };
    let a258 = Str::lit("tools/call");
    if equal(a257, a258) {
        let m260 = match v0 {
            Adt92::C0(_, _, h259) => {
                h259
            }
        };
        match m260 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m261 = f86(Str::lit("name"), v1);
                match m261 {
                    Adt114::C0 => {
                        None::<Str>
                    }
                    Adt114::C1(v2) => {
                        f113(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f84() -> List<Str> {
    List::cons(f114(), List::<Str>::nil())
}

pub fn f85(v0: Adt5) -> R<Adt113> {
    let m262 = f86(Str::lit("id"), v0);
    match m262 {
        Adt114::C0 => {
            Ok(Adt113::C0)
        }
        Adt114::C1(v1) => {
            let m263 = v1;
            match m263 {
                Adt6::C0 => {
                    Ok(Adt113::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt113::C2)
                }
                Adt6::C2(v3) => {
                    let m264 = f115(v3)?;
                    match m264 {
                        None => {
                            Ok(Adt113::C2)
                        }
                        Some(v4) => {
                            Ok(Adt113::C1(Adt89::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt113::C1(Adt89::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt113::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt113::C2)
                }
            }
        }
    }
}

pub fn f86(v0: Str, v1: Adt5) -> Adt114 {
    f116(v0, match v1 {
        Adt5::C0(h265) => {
            h265
        }
    })
}

pub fn f87(v0: Adt5) -> Adt91 {
    if f88(v0) {
        Adt91::C8
    } else {
        Adt91::C3
    }
}

pub fn f88(v0: Adt5) -> bool {
    if f117(Str::lit("result"), v0.clone()) {
        true
    } else {
        f117(Str::lit("error"), v0)
    }
}

pub fn f89(v0: Adt89, v1: Adt5) -> Result<Adt92, Adt90> {
    let m266 = f86(Str::lit("jsonrpc"), v1.clone());
    match m266 {
        Adt114::C0 => {
            f67(Some(v0), Adt91::C2)
        }
        Adt114::C1(v2) => {
            let m267 = f113(v2);
            match m267 {
                None => {
                    f67(Some(v0), Adt91::C2)
                }
                Some(v3) => {
                    let a268 = v3;
                    let a269 = f58();
                    if equal(a268, a269) {
                        f118(v0, v1)
                    } else {
                        f67(Some(v0), Adt91::C2)
                    }
                }
            }
        }
    }
}

pub fn f90(v0: u64, v1: u64) -> Adt98 {
    let a270 = v1;
    let a271 = 0u64;
    if nat_eq(a270, a271) {
        Adt98::C0
    } else {
        let a272 = v1;
        let a273 = 2u64;
        if nat_eq(a272, a273) {
            Adt98::C2(v0)
        } else {
            Adt98::C1({
                let a277 = v0;
                let a278 = {
                    let a274 = v1;
                    let a275 = 3u64;
                    let a276 = 0u64;
                    nat_quot(a274, a275, a276)
                };
                nat_sub(a277, a278)
            })
        }
    }
}

pub fn f91(p306: Adt99, p307: u64, p308: u64, p309: List<u64>) -> R<u64> {
    let m305 = (p306, (p307, (p308, p309)));
    {
        let mut m305 = m305;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m305;
                let m279 = v3;
                match m279.uncons() {
                    None => {
                        let m280 = v0;
                        match m280 {
                            Adt99::C0 => {
                                Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt99::C1 => {
                                Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt99::C2 => {
                                Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt99::C3 => {
                                Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m281 = v0;
                        match m281 {
                            Adt99::C0 => {
                                let a282 = v4;
                                let a283 = 128u64;
                                if nat_lt(a282, a283) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a284 = v4;
                                    let a285 = 194u64;
                                    if nat_lt(a284, a285) {
                                        let v6: u64 = f119({
                                            let a286 = List::cons(v4, v5);
                                            length_list(a286)
                                        })?;
                                        Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a287 = v4;
                                        let a288 = 224u64;
                                        if nat_lt(a287, a288) {
                                            Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a289 = v4;
                                            let a290 = 224u64;
                                            if nat_eq(a289, a290) {
                                                Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a291 = v4;
                                                let a292 = 237u64;
                                                if nat_eq(a291, a292) {
                                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a293 = v4;
                                                    let a294 = 240u64;
                                                    if nat_lt(a293, a294) {
                                                        Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a295 = v4;
                                                        let a296 = 240u64;
                                                        if nat_eq(a295, a296) {
                                                            Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a297 = v4;
                                                            let a298 = 244u64;
                                                            if nat_eq(a297, a298) {
                                                                Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a299 = v4;
                                                                let a300 = 244u64;
                                                                if nat_lt(a299, a300) {
                                                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f119({
                                                                        let a301 = List::cons(v4, v5);
                                                                        length_list(a301)
                                                                    })?;
                                                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt99::C1 => {
                                if f120(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f119({
                                        let a302 = List::cons(v4, v5);
                                        length_list(a302)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt99::C2 => {
                                if f120(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f119({
                                        let a303 = List::cons(v4, v5);
                                        length_list(a303)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt99::C3 => {
                                if f120(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f119({
                                        let a304 = List::cons(v4, v5);
                                        length_list(a304)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt100, v1: u64) -> R<Result<Adt6, Adt88>> {
    let m311 = match v0.clone() {
        Adt100::C0(h310, _, _) => {
            h310
        }
    };
    match m311 {
        Adt101::C0 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C1 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C2 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C3 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C4 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C5 => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C7(_, _) => {
            Ok(Err::<Adt6, Adt88>(Adt88::C3))
        }
        Adt101::C8(v3) => {
            let m312 = f121(v3)?;
            match m312 {
                Adt120::C0 => {
                    Ok(Err::<Adt6, Adt88>(Adt88::C7(v1)))
                }
                Adt120::C1 => {
                    Ok(Err::<Adt6, Adt88>(Adt88::C8(v1)))
                }
                Adt120::C2(v4) => {
                    let m317 = {
                        let m316 = f122(v4, match v0.clone() {
                            Adt100::C0(_, h314, _) => {
                                h314
                            }
                        }, match v0 {
                            Adt100::C0(_, _, h315) => {
                                h315
                            }
                        });
                        match m316 {
                            Adt100::C0(h313, _, _) => {
                                h313
                            }
                        }
                    };
                    match m317 {
                        Adt101::C0 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C1 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C2 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C3 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C4 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C5 => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C7(_, _) => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C8(_) => {
                            Ok(Err::<Adt6, Adt88>(Adt88::C3))
                        }
                        Adt101::C9(v5) => {
                            Ok(Ok::<Adt6, Adt88>(v5))
                        }
                    }
                }
            }
        }
        Adt101::C9(v2) => {
            Ok(Ok::<Adt6, Adt88>(v2))
        }
    }
}

pub fn f93(v0: Adt100, v1: u64, v2: u64) -> R<Adt115> {
    let m319 = match v0.clone() {
        Adt100::C0(h318, _, _) => {
            h318
        }
    };
    match m319 {
        Adt101::C0 => {
            if f123(v2) {
                Ok(Adt115::C1(v0))
            } else {
                f124(v0, v1, v2)
            }
        }
        Adt101::C1 => {
            if f123(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a320 = v2;
                let a321 = 93u64;
                if nat_eq(a320, a321) {
                    Ok(f125(v0, v1))
                } else {
                    f124(v0, v1, v2)
                }
            }
        }
        Adt101::C2 => {
            Ok(f126(v0, v1, v2))
        }
        Adt101::C3 => {
            if f123(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a322 = v2;
                let a323 = 125u64;
                if nat_eq(a322, a323) {
                    Ok(f127(v0, v1))
                } else {
                    let a324 = v2;
                    let a325 = 34u64;
                    if nat_eq(a324, a325) {
                        Ok(f128(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f129(Adt88::C2(v1)))
                    }
                }
            }
        }
        Adt101::C4 => {
            if f123(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a326 = v2;
                let a327 = 34u64;
                if nat_eq(a326, a327) {
                    Ok(f128(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f129(Adt88::C2(v1)))
                }
            }
        }
        Adt101::C5 => {
            if f123(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a328 = v2;
                let a329 = 58u64;
                if nat_eq(a328, a329) {
                    Ok(f128(v0, Adt101::C0))
                } else {
                    Ok(f129(Adt88::C2(v1)))
                }
            }
        }
        Adt101::C6(v3, v4, v5) => {
            f130(v0, v1, v2, v3, v4, v5)
        }
        Adt101::C7(v6, v7) => {
            f131(v0, v1, v2, v6, v7)
        }
        Adt101::C8(v8) => {
            f132(v0, v1, v2, v8)
        }
        Adt101::C9(_) => {
            Ok(f126(v0, v1, v2))
        }
    }
}

pub fn f94(p334: u64, p335: List<Adt106>, p336: List<u64>) -> R<Result<List<u64>, Adt88>> {
    let m333 = (p334, (p335, p336));
    {
        let mut m333 = m333;
        loop {
            let step = {
                let (v0, (v1, v2)) = m333;
                let m330 = v0;
                if m330 == 0 {
                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>(Ok(Err::<List<u64>, Adt88>(Adt88::C13)))
                } else {
                    let v3 = m330 - 1;
                    let m331 = v1;
                    match m331.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>(Ok(Ok::<List<u64>, Adt88>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m332 = f133(v4, v5, v2)?;
                            match m332 {
                                Adt121::C0(v6) => {
                                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>(Ok(Err::<List<u64>, Adt88>(v6)))
                                }
                                Adt121::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p339: List<u64>, p340: List<u64>) -> List<u64> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v0, v1) = m338;
                let m337 = v0;
                match m337.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: Adt108) -> R<Option<Adt5>> {
    let m342 = f134(match v0.clone() {
        Adt108::C0(h341, _) => {
            h341
        }
    })?;
    match m342 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a345 = v1;
                let a346 = {
                    let m344 = f135(v0);
                    match m344 {
                        Adt5::C0(h343) => {
                            h343
                        }
                    }
                };
                append_list(a345, a346)
            })))
        }
    }
}

pub fn f97() -> u64 {
    200u64
}

pub fn f98(v0: Adt89, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f58())), List::cons((Str::lit("id"), f75(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h347) => {
            h347
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f99(v0: Adt110) -> u64 {
    let m348 = v0;
    match m348 {
        Adt110::C0(_) => {
            f42()
        }
        Adt110::C1(_, _) => {
            f42()
        }
        Adt110::C2(_) => {
            f42()
        }
        Adt110::C3(_) => {
            f136()
        }
        Adt110::C4(v6) => {
            f137(v6)
        }
    }
}

pub fn f100(v0: Adt110) -> Adt78 {
    Adt78::C0(f138(v0.clone()), f139(v0.clone()), f140(v0))
}

pub fn f101(v0: Adt0, v1: Fn0, v2: Result<Adt116, Adt110>) -> Result<Adt108, Adt110> {
    let m349 = v2;
    match m349 {
        Ok(v4) => {
            f141(v0, v1, v4)
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(v3)
        }
    }
}

pub fn f102(v0: Option<Adt93>, v1: List<Str>, v2: Adt92) -> R<Result<Adt116, Adt110>> {
    Ok(f142(f143(match v2.clone() {
        Adt92::C0(_, h350, _) => {
            h350
        }
    }), f144(v0, v1, v2)?))
}

pub fn f103(v0: Adt95, v1: Adt118) -> Adt112 {
    let m351 = v1;
    match m351 {
        Adt118::C0 => {
            f110()
        }
        Adt118::C1 => {
            Adt112::C0(None::<Str>, Some(Adt94::C6(v0)))
        }
        Adt118::C2 => {
            Adt112::C0(None::<Str>, Some(Adt94::C7(v0)))
        }
        Adt118::C3(v2) => {
            let m352 = f145(v2);
            match m352 {
                None => {
                    Adt112::C0(None::<Str>, Some(Adt94::C6(v0)))
                }
                Some(v3) => {
                    Adt112::C0(Some(v3), None::<Adt94>)
                }
            }
        }
    }
}

pub fn f104(v0: List<Adt84>, v1: List<u64>) -> R<Adt118> {
    let m353 = f146(v0, v1, Adt119::C0)?;
    match m353 {
        Adt119::C0 => {
            Ok(Adt118::C0)
        }
        Adt119::C1(v2) => {
            let m354 = f147(v2);
            match m354 {
                None => {
                    Ok(Adt118::C1)
                }
                Some(v3) => {
                    Ok(Adt118::C3(v3))
                }
            }
        }
        Adt119::C2 => {
            Ok(Adt118::C2)
        }
    }
}

pub fn f105() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f106() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f107(v0: Str) -> bool {
    let m355 = f148(v0);
    match m355 {
        Adt125::C0 => {
            false
        }
        Adt125::C1 => {
            true
        }
        Adt125::C2 => {
            true
        }
        Adt125::C3 => {
            true
        }
    }
}

pub fn f108(v0: Adt118) -> R<Adt118> {
    let m356 = v0.clone();
    match m356 {
        Adt118::C0 => {
            Ok(v0)
        }
        Adt118::C1 => {
            Ok(v0)
        }
        Adt118::C2 => {
            Ok(v0)
        }
        Adt118::C3(v1) => {
            let m357 = f149(v1);
            match m357 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m358 = f150(v2)?;
                    match m358 {
                        None => {
                            Ok(Adt118::C1)
                        }
                        Some(v3) => {
                            Ok(Adt118::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f109() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f110() -> Adt112 {
    Adt112::C0(None::<Str>, None::<Adt94>)
}

pub fn f111(v0: Option<Adt94>, v1: Option<Adt94>) -> Option<Adt94> {
    let m359 = v0.clone();
    match m359 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f112(v0: List<Adt84>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt96>> {
    let m360 = v1;
    match m360.uncons() {
        None => {
            Ok(List::<Adt96>::nil())
        }
        Some((v3, v4)) => {
            if f151(v3.clone(), v2.clone()) {
                Ok(List::cons(Adt96::C0(match v3.clone() {
                    Adt12::C0(_, h361, _, _) => {
                        h361
                    }
                }, match v3.clone() {
                    Adt12::C0(_, _, h362, _) => {
                        h362
                    }
                }, match v3.clone() {
                    Adt12::C0(_, _, _, h363) => {
                        h363
                    }
                }, f152(v0.clone(), match v3 {
                    Adt12::C0(_, h364, _, _) => {
                        h364
                    }
                })?), f112(v0, v4, v2)?))
            } else {
                f112(v0, v4, v2)
            }
        }
    }
}

pub fn f113(v0: Adt6) -> Option<Str> {
    let m365 = v0;
    match m365 {
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

pub fn f114() -> Str {
    Str::lit("2026-07-28")
}

pub fn f115(v0: Adt7) -> R<Option<i64>> {
    let m368 = f153(40u64, match v0.clone() {
        Adt7::C0(h366, _) => {
            h366
        }
    }, match v0 {
        Adt7::C0(_, h367) => {
            h367
        }
    })?;
    match m368 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f154(v1))
        }
    }
}

pub fn f116(v0: Str, p374: List<(Str, Adt6)>) -> Adt114 {
    let m373 = p374;
    {
        let mut m373 = m373;
        loop {
            let step = {
                let v1 = m373;
                let m369 = v1;
                match m369.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m371 = f155({
                            let (h370, _) = v2;
                            h370
                        }, v0.clone());
                        match m371 {
                            Adt126::C0 => {
                                Err::<List<(Str, Adt6)>, Adt114>(Adt114::C1({
                                    let (_, h372) = v2;
                                    h372
                                }))
                            }
                            Adt126::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt114>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Str, v1: Adt5) -> bool {
    f156(f86(v0, v1))
}

pub fn f118(v0: Adt89, v1: Adt5) -> Result<Adt92, Adt90> {
    let m375 = f86(Str::lit("method"), v1.clone());
    match m375 {
        Adt114::C0 => {
            f67(Some(v0), f87(v1))
        }
        Adt114::C1(v2) => {
            let m376 = f113(v2);
            match m376 {
                None => {
                    f67(Some(v0), Adt91::C4)
                }
                Some(v3) => {
                    f157(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f119(v0: u64) -> R<u64> {
    let a379 = {
        let a377 = v0;
        let a378 = 3u64;
        nat_mul(a377, a378)?
    };
    let a380 = 1u64;
    nat_add(a379, a380)
}

pub fn f120(v0: u64, v1: u64, v2: u64) -> bool {
    let a381 = v1;
    let a382 = v0;
    if nat_le(a381, a382) {
        let a383 = v0;
        let a384 = v2;
        nat_le(a383, a384)
    } else {
        false
    }
}

pub fn f121(v0: Adt103) -> R<Adt120> {
    if f158(match v0.clone() {
        Adt103::C0(_, h385, _, _, _, _, _) => {
            h385
        }
    }) {
        let v1: i64 = {
            let a391 = if match v0.clone() {
                Adt103::C0(_, _, _, _, _, h386, _) => {
                    h386
                }
            } {
                let a388 = match v0.clone() {
                    Adt103::C0(_, _, _, _, _, _, h387) => {
                        h387
                    }
                };
                int_neg(a388)?
            } else {
                match v0.clone() {
                    Adt103::C0(_, _, _, _, _, _, h389) => {
                        h389
                    }
                }
            };
            let a392 = match v0.clone() {
                Adt103::C0(_, _, _, _, h390, _, _) => {
                    h390
                }
            };
            int_sub(a391, a392)?
        };
        let m395 = {
            let a393 = v1;
            let a394 = -9999i64;
            compare(a393, a394)
        };
        if if match m395 {
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
            let m398 = {
                let a396 = 9999i64;
                let a397 = v1;
                compare(a396, a397)
            };
            match m398 {
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
            Ok(Adt120::C1)
        } else {
            Ok(Adt120::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt103::C0(h399, _, _, _, _, _, _) => {
                    h399
                }
            } {
                let a401 = match v0 {
                    Adt103::C0(_, _, h400, _, _, _, _) => {
                        h400
                    }
                };
                int_neg(a401)?
            } else {
                match v0 {
                    Adt103::C0(_, _, h402, _, _, _, _) => {
                        h402
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt120::C0)
    }
}

pub fn f122(v0: Adt6, v1: List<Adt105>, v2: u64) -> Adt100 {
    let m403 = v1.clone();
    match m403.uncons() {
        None => {
            Adt100::C0(Adt101::C9(v0), List::<Adt105>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m404 = v3;
            match m404 {
                Adt105::C0(v5) => {
                    Adt100::C0(Adt101::C2, List::cons(Adt105::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt105::C1(v6, v7) => {
                    let m405 = v7;
                    match m405 {
                        None => {
                            Adt100::C0(Adt101::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt100::C0(Adt101::C2, List::cons(Adt105::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(v0: u64) -> bool {
    let a406 = v0;
    let a407 = 32u64;
    if nat_eq(a406, a407) {
        true
    } else {
        let a408 = v0;
        let a409 = 10u64;
        if nat_eq(a408, a409) {
            true
        } else {
            let a410 = v0;
            let a411 = 13u64;
            if nat_eq(a410, a411) {
                true
            } else {
                let a412 = v0;
                let a413 = 9u64;
                nat_eq(a412, a413)
            }
        }
    }
}

pub fn f124(v0: Adt100, v1: u64, v2: u64) -> R<Adt115> {
    let a414 = v2;
    let a415 = 34u64;
    if nat_eq(a414, a415) {
        Ok(f128(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), false)))
    } else {
        let a416 = v2;
        let a417 = 123u64;
        if nat_eq(a416, a417) {
            f159(v0, Adt105::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt101::C3, v1)
        } else {
            let a418 = v2;
            let a419 = 91u64;
            if nat_eq(a418, a419) {
                f159(v0, Adt105::C0(List::<Adt6>::nil()), Adt101::C1, v1)
            } else {
                let a420 = v2;
                let a421 = 116u64;
                if nat_eq(a420, a421) {
                    Ok(f128(v0, Adt101::C7(0u64, 1u64)))
                } else {
                    let a422 = v2;
                    let a423 = 102u64;
                    if nat_eq(a422, a423) {
                        Ok(f128(v0, Adt101::C7(1u64, 1u64)))
                    } else {
                        let a424 = v2;
                        let a425 = 110u64;
                        if nat_eq(a424, a425) {
                            Ok(f128(v0, Adt101::C7(2u64, 1u64)))
                        } else {
                            let a426 = v2;
                            let a427 = 45u64;
                            if nat_eq(a426, a427) {
                                Ok(f128(v0, f160(true, Adt104::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a428 = v2;
                                let a429 = 48u64;
                                if nat_eq(a428, a429) {
                                    Ok(f128(v0, f160(false, Adt104::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f120(v2, 49u64, 57u64) {
                                        Ok(f128(v0, f160(false, Adt104::C2, f161({
                                            let a430 = v2;
                                            let a431 = 48u64;
                                            nat_sub(a430, a431)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f129(Adt88::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f125(v0: Adt100, v1: u64) -> Adt115 {
    let m433 = match v0.clone() {
        Adt100::C0(_, h432, _) => {
            h432
        }
    };
    match m433.uncons() {
        None => {
            f129(Adt88::C2(v1))
        }
        Some((v2, v3)) => {
            let m434 = v2;
            match m434 {
                Adt105::C0(v4) => {
                    Adt115::C1(f122(Adt6::C4(f162(v4, List::<Adt6>::nil())), v3, {
                        let a436 = match v0 {
                            Adt100::C0(_, _, h435) => {
                                h435
                            }
                        };
                        let a437 = 1u64;
                        nat_sub(a436, a437)
                    }))
                }
                Adt105::C1(_, _) => {
                    f129(Adt88::C2(v1))
                }
            }
        }
    }
}

pub fn f126(v0: Adt100, v1: u64, v2: u64) -> Adt115 {
    let m439 = match v0.clone() {
        Adt100::C0(h438, _, _) => {
            h438
        }
    };
    match m439 {
        Adt101::C0 => {
            f129(Adt88::C2(v1))
        }
        Adt101::C1 => {
            f129(Adt88::C2(v1))
        }
        Adt101::C2 => {
            if f123(v2) {
                Adt115::C1(v0)
            } else {
                let a440 = v2;
                let a441 = 44u64;
                if nat_eq(a440, a441) {
                    f163(v0, v1)
                } else {
                    let a442 = v2;
                    let a443 = 93u64;
                    if nat_eq(a442, a443) {
                        f125(v0, v1)
                    } else {
                        let a444 = v2;
                        let a445 = 125u64;
                        if nat_eq(a444, a445) {
                            f127(v0, v1)
                        } else {
                            f129(Adt88::C2(v1))
                        }
                    }
                }
            }
        }
        Adt101::C3 => {
            f129(Adt88::C2(v1))
        }
        Adt101::C4 => {
            f129(Adt88::C2(v1))
        }
        Adt101::C5 => {
            f129(Adt88::C2(v1))
        }
        Adt101::C6(_, _, _) => {
            f129(Adt88::C2(v1))
        }
        Adt101::C7(_, _) => {
            f129(Adt88::C2(v1))
        }
        Adt101::C8(_) => {
            f129(Adt88::C2(v1))
        }
        Adt101::C9(_) => {
            if f123(v2) {
                Adt115::C1(v0)
            } else {
                f129(Adt88::C10(v1))
            }
        }
    }
}

pub fn f127(v0: Adt100, v1: u64) -> Adt115 {
    let m447 = match v0.clone() {
        Adt100::C0(_, h446, _) => {
            h446
        }
    };
    match m447.uncons() {
        None => {
            f129(Adt88::C2(v1))
        }
        Some((v2, v3)) => {
            let m448 = v2;
            match m448 {
                Adt105::C0(_) => {
                    f129(Adt88::C2(v1))
                }
                Adt105::C1(v5, _) => {
                    Adt115::C1(f122(Adt6::C5(f164(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a450 = match v0 {
                            Adt100::C0(_, _, h449) => {
                                h449
                            }
                        };
                        let a451 = 1u64;
                        nat_sub(a450, a451)
                    }))
                }
            }
        }
    }
}

pub fn f128(v0: Adt100, v1: Adt101) -> Adt115 {
    Adt115::C1(f165(v0, v1))
}

pub fn f129(v0: Adt88) -> Adt115 {
    Adt115::C0(v0)
}

pub fn f130(v0: Adt100, v1: u64, v2: u64, v3: Adt102, v4: List<u64>, v5: bool) -> R<Adt115> {
    let m452 = v3;
    match m452 {
        Adt102::C0 => {
            let a453 = v2;
            let a454 = 34u64;
            if nat_eq(a453, a454) {
                Ok(f166(v0, v1, v4, v5))
            } else {
                let a455 = v2;
                let a456 = 92u64;
                if nat_eq(a455, a456) {
                    Ok(f128(v0, Adt101::C6(Adt102::C1, v4, v5)))
                } else {
                    let a457 = v2;
                    let a458 = 32u64;
                    if nat_lt(a457, a458) {
                        Ok(f129(Adt88::C4(v1)))
                    } else {
                        Ok(f128(v0, Adt101::C6(Adt102::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt102::C1 => {
            let a459 = v2;
            let a460 = 117u64;
            if nat_eq(a459, a460) {
                Ok(f128(v0, Adt101::C6(Adt102::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a461 = f167(v2);
                let a462 = 0u64;
                if nat_eq(a461, a462) {
                    Ok(f129(Adt88::C5(v1)))
                } else {
                    Ok(f128(v0, Adt101::C6(Adt102::C0, List::cons(f167(v2), v4), v5)))
                }
            }
        }
        Adt102::C2(v6, v7, v8) => {
            let a463 = 15u64;
            let a464 = f168(v2);
            if nat_lt(a463, a464) {
                Ok(f129(Adt88::C5(v1)))
            } else {
                let a467 = {
                    let a465 = v6;
                    let a466 = 1u64;
                    nat_add(a465, a466)?
                };
                let a468 = 4u64;
                if nat_eq(a467, a468) {
                    f169(v0, v1, {
                        let a471 = {
                            let a469 = v7;
                            let a470 = 16u64;
                            nat_mul(a469, a470)?
                        };
                        let a472 = f168(v2);
                        nat_add(a471, a472)?
                    }, v8, v4, v5)
                } else {
                    Ok(f128(v0, Adt101::C6(Adt102::C2({
                        let a473 = v6;
                        let a474 = 1u64;
                        nat_add(a473, a474)?
                    }, {
                        let a477 = {
                            let a475 = v7;
                            let a476 = 16u64;
                            nat_mul(a475, a476)?
                        };
                        let a478 = f168(v2);
                        nat_add(a477, a478)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt102::C3(v9) => {
            let a479 = v2;
            let a480 = 92u64;
            if nat_eq(a479, a480) {
                Ok(f128(v0, Adt101::C6(Adt102::C4(v9), v4, v5)))
            } else {
                Ok(f129(Adt88::C6(v1)))
            }
        }
        Adt102::C4(v10) => {
            let a481 = v2;
            let a482 = 117u64;
            if nat_eq(a481, a482) {
                Ok(f128(v0, Adt101::C6(Adt102::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f129(Adt88::C6(v1)))
            }
        }
    }
}

pub fn f131(v0: Adt100, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt115> {
    let a483 = v2;
    let a484 = f170(v3, v4);
    if nat_eq(a483, a484) {
        let a488 = {
            let a485 = v4;
            let a486 = 1u64;
            nat_add(a485, a486)?
        };
        let a489 = {
            let a487 = f171(v3);
            length_list(a487)
        };
        if nat_eq(a488, a489) {
            Ok(f172(v0, f173(v3)))
        } else {
            Ok(f128(v0, Adt101::C7(v3, {
                let a490 = v4;
                let a491 = 1u64;
                nat_add(a490, a491)?
            })))
        }
    } else {
        Ok(f129(Adt88::C2(v1)))
    }
}

pub fn f132(v0: Adt100, v1: u64, v2: u64, v3: Adt103) -> R<Adt115> {
    let m493 = match v3.clone() {
        Adt103::C0(_, h492, _, _, _, _, _) => {
            h492
        }
    };
    match m493 {
        Adt104::C0 => {
            let a494 = v2;
            let a495 = 48u64;
            if nat_eq(a494, a495) {
                Ok(f174(v0, v3, Adt104::C1))
            } else {
                if f120(v2, 49u64, 57u64) {
                    f175(v0, v1, v2, v3, false)
                } else {
                    Ok(f129(Adt88::C7(v1)))
                }
            }
        }
        Adt104::C1 => {
            let a496 = v2;
            let a497 = 46u64;
            if nat_eq(a496, a497) {
                Ok(f174(v0, v3, Adt104::C3))
            } else {
                if f176(v2) {
                    Ok(f174(v0, v3, Adt104::C5))
                } else {
                    if f177(v2) {
                        Ok(f129(Adt88::C7(v1)))
                    } else {
                        f178(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt104::C2 => {
            if f177(v2) {
                f175(v0, v1, v2, v3, false)
            } else {
                let a498 = v2;
                let a499 = 46u64;
                if nat_eq(a498, a499) {
                    Ok(f174(v0, v3, Adt104::C3))
                } else {
                    if f176(v2) {
                        Ok(f174(v0, v3, Adt104::C5))
                    } else {
                        f178(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt104::C3 => {
            if f177(v2) {
                f175(v0, v1, v2, v3, true)
            } else {
                Ok(f129(Adt88::C7(v1)))
            }
        }
        Adt104::C4 => {
            if f177(v2) {
                f175(v0, v1, v2, v3, true)
            } else {
                if f176(v2) {
                    Ok(f174(v0, v3, Adt104::C5))
                } else {
                    f178(v0, v1, v2, v3)
                }
            }
        }
        Adt104::C5 => {
            let a500 = v2;
            let a501 = 43u64;
            if nat_eq(a500, a501) {
                Ok(f128(v0, f160(match v3.clone() {
                    Adt103::C0(h502, _, _, _, _, _, _) => {
                        h502
                    }
                }, Adt104::C6, match v3.clone() {
                    Adt103::C0(_, _, h503, _, _, _, _) => {
                        h503
                    }
                }, match v3.clone() {
                    Adt103::C0(_, _, _, h504, _, _, _) => {
                        h504
                    }
                }, match v3 {
                    Adt103::C0(_, _, _, _, h505, _, _) => {
                        h505
                    }
                }, false, 0i64)))
            } else {
                let a506 = v2;
                let a507 = 45u64;
                if nat_eq(a506, a507) {
                    Ok(f128(v0, f160(match v3.clone() {
                        Adt103::C0(h508, _, _, _, _, _, _) => {
                            h508
                        }
                    }, Adt104::C6, match v3.clone() {
                        Adt103::C0(_, _, h509, _, _, _, _) => {
                            h509
                        }
                    }, match v3.clone() {
                        Adt103::C0(_, _, _, h510, _, _, _) => {
                            h510
                        }
                    }, match v3 {
                        Adt103::C0(_, _, _, _, h511, _, _) => {
                            h511
                        }
                    }, true, 0i64)))
                } else {
                    if f177(v2) {
                        f179(v0, v1, v2, Adt103::C0(match v3.clone() {
                            Adt103::C0(h512, _, _, _, _, _, _) => {
                                h512
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, h513, _, _, _, _, _) => {
                                h513
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, _, h514, _, _, _, _) => {
                                h514
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, _, _, h515, _, _, _) => {
                                h515
                            }
                        }, match v3 {
                            Adt103::C0(_, _, _, _, h516, _, _) => {
                                h516
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f129(Adt88::C7(v1)))
                    }
                }
            }
        }
        Adt104::C6 => {
            if f177(v2) {
                f179(v0, v1, v2, v3.clone(), match v3 {
                    Adt103::C0(_, _, _, _, _, h517, _) => {
                        h517
                    }
                })
            } else {
                Ok(f129(Adt88::C7(v1)))
            }
        }
        Adt104::C7 => {
            if f177(v2) {
                f179(v0, v1, v2, v3.clone(), match v3 {
                    Adt103::C0(_, _, _, _, _, h518, _) => {
                        h518
                    }
                })
            } else {
                f178(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f133(v0: Adt106, v1: List<Adt106>, v2: List<u64>) -> R<Adt121> {
    let m519 = v0;
    match m519 {
        Adt106::C0(v3) => {
            Ok(Adt121::C1(v1, f95(v3, v2)))
        }
        Adt106::C1(v4, v5) => {
            f180(v4, v5, v1, v2)
        }
        Adt106::C2(v6, v7) => {
            let m520 = v7;
            match m520.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C1(v6, v8), List::cons(Adt106::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt106::C3(v10, v11) => {
            let m521 = v11.clone();
            match m521.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt106::C4(v14, v15) => {
            let m522 = v15;
            match m522.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C0({
                        let a524 = f181({
                            let (h523, _) = v16;
                            h523
                        })?;
                        let a525 = List::cons(58u64, List::<u64>::nil());
                        append_list(a524, a525)
                    }), List::cons(Adt106::C1(v14, {
                        let (_, h526) = v16;
                        h526
                    }), List::cons(Adt106::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt106::C5(v18, v19) => {
            let m527 = v19.clone();
            match m527.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f134(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m528 = v0;
    match m528 {
        Adt32::C0(v1) => {
            Ok(Some(f182(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f183(v2)?))
        }
        Adt32::C2(v3) => {
            Ok(f184(v3))
        }
        Adt32::C3(_) => {
            Ok(None::<List<(Str, Adt6)>>)
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
        Adt32::C10 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C11(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C12(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f135(v0: Adt108) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f185(f186(match v0.clone() {
        Adt108::C0(h529, _) => {
            h529
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f187({
        let m532 = match v0 {
            Adt108::C0(_, h531) => {
                h531
            }
        };
        match m532 {
            Adt109::C0(h530, _) => {
                h530
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f136() -> u64 {
    404u64
}

pub fn f137(v0: Adt82) -> u64 {
    let m533 = v0;
    match m533 {
        Adt82::C0(_) => {
            f42()
        }
        Adt82::C1(_) => {
            f42()
        }
        Adt82::C2(_) => {
            f42()
        }
        Adt82::C3(_) => {
            f42()
        }
        Adt82::C4(_) => {
            f42()
        }
        Adt82::C5(_) => {
            f42()
        }
        Adt82::C6(_) => {
            f18()
        }
    }
}

pub fn f138(v0: Adt110) -> i64 {
    let m534 = v0;
    match m534 {
        Adt110::C0(_) => {
            f188()
        }
        Adt110::C1(_, _) => {
            f189()
        }
        Adt110::C2(_) => {
            f190()
        }
        Adt110::C3(_) => {
            f191()
        }
        Adt110::C4(v6) => {
            f192(v6)
        }
    }
}

pub fn f139(v0: Adt110) -> Str {
    let m535 = v0;
    match m535 {
        Adt110::C0(v1) => {
            f193(v1)
        }
        Adt110::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt110::C2(v4) => {
            f194(v4)
        }
        Adt110::C3(_) => {
            Str::lit("Method not found")
        }
        Adt110::C4(v6) => {
            let m536 = v6;
            match m536 {
                Adt82::C0(_) => {
                    Str::lit("Invalid params: unknown tool")
                }
                Adt82::C1(_) => {
                    Str::lit("Invalid params: unknown prompt")
                }
                Adt82::C2(_) => {
                    Str::lit("Invalid params: resource not found")
                }
                Adt82::C3(_) => {
                    Str::lit("Invalid params: unknown task")
                }
                Adt82::C4(_) => {
                    Str::lit("Invalid params: invalid arguments")
                }
                Adt82::C5(_) => {
                    Str::lit("Missing required client capability")
                }
                Adt82::C6(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f140(v0: Adt110) -> Option<Adt6> {
    let m537 = v0;
    match m537 {
        Adt110::C0(_) => {
            None::<Adt6>
        }
        Adt110::C1(v2, v3) => {
            Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f52(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil()))))
        }
        Adt110::C2(_) => {
            None::<Adt6>
        }
        Adt110::C3(_) => {
            None::<Adt6>
        }
        Adt110::C4(v6) => {
            let m538 = v6;
            match m538 {
                Adt82::C0(_) => {
                    None::<Adt6>
                }
                Adt82::C1(_) => {
                    None::<Adt6>
                }
                Adt82::C2(_) => {
                    None::<Adt6>
                }
                Adt82::C3(_) => {
                    None::<Adt6>
                }
                Adt82::C4(_) => {
                    None::<Adt6>
                }
                Adt82::C5(v12) => {
                    Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f195(v12)), List::<(Str, Adt6)>::nil())))
                }
                Adt82::C6(_) => {
                    None::<Adt6>
                }
            }
        }
    }
}

pub fn f141(v0: Adt0, v1: Fn0, v2: Adt116) -> Result<Adt108, Adt110> {
    f196(v0.clone(), v1, v2.clone(), f197(match v0 {
        Adt0::C0(_, _, h539, _, _) => {
            h539
        }
    }, match v2 {
        Adt116::C0(_, h540, _, _) => {
            h540
        }
    }))
}

pub fn f142(v0: Adt122, v1: Adt123) -> Result<Adt116, Adt110> {
    let m541 = v1;
    match m541 {
        Adt123::C0(v3) => {
            f198(v3, v0)
        }
        Adt123::C1(v2) => {
            Err::<Adt116, Adt110>(v2)
        }
    }
}

pub fn f143(v0: Str) -> Adt122 {
    let a542 = v0.clone();
    let a543 = Str::lit("server/discover");
    if equal(a542, a543) {
        Adt122::C1(Adt117::C0)
    } else {
        let a544 = v0.clone();
        let a545 = Str::lit("tools/list");
        if equal(a544, a545) {
            Adt122::C1(Adt117::C1)
        } else {
            let a546 = v0.clone();
            let a547 = Str::lit("tools/call");
            if equal(a546, a547) {
                Adt122::C1(Adt117::C2)
            } else {
                let a548 = v0.clone();
                let a549 = Str::lit("resources/list");
                if equal(a548, a549) {
                    Adt122::C1(Adt117::C3)
                } else {
                    let a550 = v0.clone();
                    let a551 = Str::lit("resources/read");
                    if equal(a550, a551) {
                        Adt122::C1(Adt117::C4)
                    } else {
                        let a552 = v0.clone();
                        let a553 = Str::lit("resources/templates/list");
                        if equal(a552, a553) {
                            Adt122::C1(Adt117::C5)
                        } else {
                            let a554 = v0.clone();
                            let a555 = Str::lit("prompts/list");
                            if equal(a554, a555) {
                                Adt122::C1(Adt117::C6)
                            } else {
                                let a556 = v0.clone();
                                let a557 = Str::lit("prompts/get");
                                if equal(a556, a557) {
                                    Adt122::C1(Adt117::C7)
                                } else {
                                    let a558 = v0.clone();
                                    let a559 = Str::lit("completion/complete");
                                    if equal(a558, a559) {
                                        Adt122::C1(Adt117::C8)
                                    } else {
                                        let a560 = v0.clone();
                                        let a561 = Str::lit("tasks/get");
                                        if equal(a560, a561) {
                                            Adt122::C1(Adt117::C9)
                                        } else {
                                            let a562 = v0.clone();
                                            let a563 = Str::lit("tasks/update");
                                            if equal(a562, a563) {
                                                Adt122::C1(Adt117::C10)
                                            } else {
                                                let a564 = v0;
                                                let a565 = Str::lit("tasks/cancel");
                                                if equal(a564, a565) {
                                                    Adt122::C1(Adt117::C11)
                                                } else {
                                                    Adt122::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f144(v0: Option<Adt93>, v1: List<Str>, v2: Adt92) -> R<Adt123> {
    f199(v0, v1, v2.clone(), f200(match v2 {
        Adt92::C0(_, _, h566) => {
            h566
        }
    }))
}

pub fn f145(v0: List<u64>) -> Option<Str> {
    let a567 = f201(f202(), v0);
    utf8_decode(a567)
}

pub fn f146(p572: List<Adt84>, v1: List<u64>, p573: Adt119) -> R<Adt119> {
    let m571 = (p572, p573);
    {
        let mut m571 = m571;
        loop {
            let step = {
                let (v0, v2) = m571;
                let m568 = v0;
                match m568.uncons() {
                    None => {
                        Err::<(List<Adt84>, Adt119), R<Adt119>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f203(match v3.clone() {
                            Adt84::C0(h569, _) => {
                                h569
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt84>, Adt119), R<Adt119>>((v4, f204(v2, match v3 {
                                Adt84::C0(_, h570) => {
                                    h570
                                }
                            })))
                        } else {
                            Ok::<(List<Adt84>, Adt119), R<Adt119>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f205(v0);
    if f206(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f148(v0: Str) -> Adt125 {
    let m574 = f143(v0);
    match m574 {
        Adt122::C0 => {
            Adt125::C0
        }
        Adt122::C1(v1) => {
            f207(v1)
        }
    }
}

pub fn f149(v0: List<u64>) -> Option<List<u64>> {
    let m575 = f208(f209(), v0);
    match m575 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m576 = f208(f210(), f211(v1, List::<u64>::nil()));
            match m576 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f211(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f150(v0: List<u64>) -> R<Option<List<u64>>> {
    f212({
        let a577 = v0.clone();
        length_list(a577)
    }, v0, List::<u64>::nil())
}

pub fn f151(v0: Adt12, v1: Option<Str>) -> bool {
    let m578 = v1;
    match m578 {
        None => {
            false
        }
        Some(v2) => {
            let a580 = match v0 {
                Adt12::C0(h579, _, _, _) => {
                    h579
                }
            };
            let a581 = v2;
            equal(a580, a581)
        }
    }
}

pub fn f152(v0: List<Adt84>, v1: Str) -> R<Adt97> {
    let m584 = f108(f104(v0, {
        let a582 = f213();
        let a583 = f214(v1)?;
        append_list(a582, a583)
    })?)?;
    match m584 {
        Adt118::C0 => {
            Ok(Adt97::C0)
        }
        Adt118::C1 => {
            Ok(Adt97::C1)
        }
        Adt118::C2 => {
            Ok(Adt97::C1)
        }
        Adt118::C3(v2) => {
            let m585 = f145(v2);
            match m585 {
                None => {
                    Ok(Adt97::C1)
                }
                Some(v3) => {
                    Ok(Adt97::C2(v3))
                }
            }
        }
    }
}

pub fn f153(p601: u64, p602: i64, p603: i64) -> R<Option<i64>> {
    let m600 = (p601, (p602, p603));
    {
        let mut m600 = m600;
        loop {
            let step = {
                let (v0, (v1, v2)) = m600;
                let m586 = v0;
                if m586 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m586 - 1;
                    if f215(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m589 = {
                            let a587 = v2;
                            let a588 = 0i64;
                            compare(a587, a588)
                        };
                        if match m589 {
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
                            if f215({
                                let a590 = v1;
                                let a591 = 10i64;
                                let a592 = 0i64;
                                int_rem(a590, a591, a592)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a593 = v1;
                                    let a594 = 10i64;
                                    let a595 = 0i64;
                                    int_quot(a593, a594, a595)?
                                }, f216(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a596 = v1;
                                let a597 = 10i64;
                                int_mul(a596, a597)?
                            }, {
                                let a598 = v2;
                                let a599 = 1i64;
                                int_sub(a598, a599)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: i64) -> Option<i64> {
    let m606 = {
        let a604 = 9007199254740991i64;
        let a605 = v0;
        compare(a604, a605)
    };
    if if match m606 {
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
        let m609 = {
            let a607 = v0;
            let a608 = -9007199254740991i64;
            compare(a607, a608)
        };
        match m609 {
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

pub fn f155(v0: Str, v1: Str) -> Adt126 {
    let a610 = v0;
    let a611 = v1;
    if equal(a610, a611) {
        Adt126::C0
    } else {
        Adt126::C1
    }
}

pub fn f156(v0: Adt114) -> bool {
    let m612 = v0;
    match m612 {
        Adt114::C0 => {
            false
        }
        Adt114::C1(_) => {
            true
        }
    }
}

pub fn f157(v0: Adt89, v1: Str, v2: Adt5) -> Result<Adt92, Adt90> {
    let m613 = f86(Str::lit("params"), v2);
    match m613 {
        Adt114::C0 => {
            Ok::<Adt92, Adt90>(Adt92::C0(v0, v1, None::<Adt5>))
        }
        Adt114::C1(v3) => {
            let m614 = f217(v3);
            match m614 {
                None => {
                    f67(Some(v0), Adt91::C6)
                }
                Some(v4) => {
                    Ok::<Adt92, Adt90>(Adt92::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f158(v0: Adt104) -> bool {
    let m615 = v0;
    match m615 {
        Adt104::C0 => {
            false
        }
        Adt104::C1 => {
            true
        }
        Adt104::C2 => {
            true
        }
        Adt104::C3 => {
            false
        }
        Adt104::C4 => {
            true
        }
        Adt104::C5 => {
            false
        }
        Adt104::C6 => {
            false
        }
        Adt104::C7 => {
            true
        }
    }
}

pub fn f159(v0: Adt100, v1: Adt105, v2: Adt101, v3: u64) -> R<Adt115> {
    let a617 = match v0.clone() {
        Adt100::C0(_, _, h616) => {
            h616
        }
    };
    let a618 = f218();
    if nat_lt(a617, a618) {
        Ok(Adt115::C1(Adt100::C0(v2, List::cons(v1, match v0.clone() {
            Adt100::C0(_, h619, _) => {
                h619
            }
        }), {
            let a621 = match v0 {
                Adt100::C0(_, _, h620) => {
                    h620
                }
            };
            let a622 = 1u64;
            nat_add(a621, a622)?
        })))
    } else {
        Ok(f129(Adt88::C9(v3)))
    }
}

pub fn f160(v0: bool, v1: Adt104, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt101 {
    Adt101::C8(Adt103::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f161(v0: u64) -> i64 {
    let a623 = v0;
    let a624 = 0u64;
    if nat_eq(a623, a624) {
        0i64
    } else {
        let a625 = v0;
        let a626 = 1u64;
        if nat_eq(a625, a626) {
            1i64
        } else {
            let a627 = v0;
            let a628 = 2u64;
            if nat_eq(a627, a628) {
                2i64
            } else {
                let a629 = v0;
                let a630 = 3u64;
                if nat_eq(a629, a630) {
                    3i64
                } else {
                    let a631 = v0;
                    let a632 = 4u64;
                    if nat_eq(a631, a632) {
                        4i64
                    } else {
                        let a633 = v0;
                        let a634 = 5u64;
                        if nat_eq(a633, a634) {
                            5i64
                        } else {
                            let a635 = v0;
                            let a636 = 6u64;
                            if nat_eq(a635, a636) {
                                6i64
                            } else {
                                let a637 = v0;
                                let a638 = 7u64;
                                if nat_eq(a637, a638) {
                                    7i64
                                } else {
                                    let a639 = v0;
                                    let a640 = 8u64;
                                    if nat_eq(a639, a640) {
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

pub fn f162(p643: List<Adt6>, p644: List<Adt6>) -> List<Adt6> {
    let m642 = (p643, p644);
    {
        let mut m642 = m642;
        loop {
            let step = {
                let (v0, v1) = m642;
                let m641 = v0;
                match m641.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Adt100, v1: u64) -> Adt115 {
    let m646 = match v0.clone() {
        Adt100::C0(_, h645, _) => {
            h645
        }
    };
    match m646.uncons() {
        None => {
            f129(Adt88::C2(v1))
        }
        Some((v2, _)) => {
            let m647 = v2;
            match m647 {
                Adt105::C0(_) => {
                    f128(v0, Adt101::C0)
                }
                Adt105::C1(_, _) => {
                    f128(v0, Adt101::C4)
                }
            }
        }
    }
}

pub fn f164(p650: List<(Str, Adt6)>, p651: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m649 = (p650, p651);
    {
        let mut m649 = m649;
        loop {
            let step = {
                let (v0, v1) = m649;
                let m648 = v0;
                match m648.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m649 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Adt100, v1: Adt101) -> Adt100 {
    Adt100::C0(v1, match v0.clone() {
        Adt100::C0(_, h652, _) => {
            h652
        }
    }, match v0 {
        Adt100::C0(_, _, h653) => {
            h653
        }
    })
}

pub fn f166(v0: Adt100, v1: u64, v2: List<u64>, v3: bool) -> Adt115 {
    let m655 = {
        let a654 = f201(f202(), f95(v2, List::<u64>::nil()));
        utf8_decode(a654)
    };
    match m655 {
        None => {
            f129(Adt88::C14)
        }
        Some(v4) => {
            if v3 {
                f219(v0, v1, v4)
            } else {
                f172(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f167(v0: u64) -> u64 {
    let a656 = v0;
    let a657 = 34u64;
    if nat_eq(a656, a657) {
        34u64
    } else {
        let a658 = v0;
        let a659 = 92u64;
        if nat_eq(a658, a659) {
            92u64
        } else {
            let a660 = v0;
            let a661 = 47u64;
            if nat_eq(a660, a661) {
                47u64
            } else {
                let a662 = v0;
                let a663 = 98u64;
                if nat_eq(a662, a663) {
                    8u64
                } else {
                    let a664 = v0;
                    let a665 = 102u64;
                    if nat_eq(a664, a665) {
                        12u64
                    } else {
                        let a666 = v0;
                        let a667 = 110u64;
                        if nat_eq(a666, a667) {
                            10u64
                        } else {
                            let a668 = v0;
                            let a669 = 114u64;
                            if nat_eq(a668, a669) {
                                13u64
                            } else {
                                let a670 = v0;
                                let a671 = 116u64;
                                if nat_eq(a670, a671) {
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

pub fn f168(v0: u64) -> u64 {
    let a672 = v0;
    let a673 = 48u64;
    if nat_lt(a672, a673) {
        16u64
    } else {
        let a674 = v0;
        let a675 = 58u64;
        if nat_lt(a674, a675) {
            let a676 = v0;
            let a677 = 48u64;
            nat_sub(a676, a677)
        } else {
            let a678 = v0;
            let a679 = 65u64;
            if nat_lt(a678, a679) {
                16u64
            } else {
                let a680 = v0;
                let a681 = 71u64;
                if nat_lt(a680, a681) {
                    let a682 = v0;
                    let a683 = 55u64;
                    nat_sub(a682, a683)
                } else {
                    let a684 = v0;
                    let a685 = 97u64;
                    if nat_lt(a684, a685) {
                        16u64
                    } else {
                        let a686 = v0;
                        let a687 = 103u64;
                        if nat_lt(a686, a687) {
                            let a688 = v0;
                            let a689 = 87u64;
                            nat_sub(a688, a689)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f169(v0: Adt100, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt115> {
    let a690 = v3;
    let a691 = 0u64;
    if nat_eq(a690, a691) {
        if f120(v2, 55296u64, 56319u64) {
            Ok(f128(v0, Adt101::C6(Adt102::C3(v2), v4, v5)))
        } else {
            if f120(v2, 56320u64, 57343u64) {
                Ok(f129(Adt88::C6(v1)))
            } else {
                Ok(f128(v0, Adt101::C6(Adt102::C0, f220(v2, v4)?, v5)))
            }
        }
    } else {
        if f120(v2, 56320u64, 57343u64) {
            Ok(f128(v0, Adt101::C6(Adt102::C0, f220({
                let a700 = 65536u64;
                let a701 = {
                    let a698 = {
                        let a694 = {
                            let a692 = v3;
                            let a693 = 55296u64;
                            nat_sub(a692, a693)
                        };
                        let a695 = 1024u64;
                        nat_mul(a694, a695)?
                    };
                    let a699 = {
                        let a696 = v2;
                        let a697 = 56320u64;
                        nat_sub(a696, a697)
                    };
                    nat_add(a698, a699)?
                };
                nat_add(a700, a701)?
            }, v4)?, v5)))
        } else {
            Ok(f129(Adt88::C6(v1)))
        }
    }
}

pub fn f170(v0: u64, v1: u64) -> u64 {
    let m704 = {
        let a702 = f171(v0);
        let a703 = v1;
        index_list(a702, a703)
    };
    match m704 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f171(v0: u64) -> List<u64> {
    let a705 = v0;
    let a706 = 0u64;
    if nat_eq(a705, a706) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a707 = v0;
        let a708 = 1u64;
        if nat_eq(a707, a708) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f172(v0: Adt100, v1: Adt6) -> Adt115 {
    Adt115::C1(f122(v1, match v0.clone() {
        Adt100::C0(_, h709, _) => {
            h709
        }
    }, match v0 {
        Adt100::C0(_, _, h710) => {
            h710
        }
    }))
}

pub fn f173(v0: u64) -> Adt6 {
    let a711 = v0;
    let a712 = 0u64;
    if nat_eq(a711, a712) {
        Adt6::C1(true)
    } else {
        let a713 = v0;
        let a714 = 1u64;
        if nat_eq(a713, a714) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f174(v0: Adt100, v1: Adt103, v2: Adt104) -> Adt115 {
    f128(v0, Adt101::C8(Adt103::C0(match v1.clone() {
        Adt103::C0(h715, _, _, _, _, _, _) => {
            h715
        }
    }, v2, match v1.clone() {
        Adt103::C0(_, _, h716, _, _, _, _) => {
            h716
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, h717, _, _, _) => {
            h717
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, _, h718, _, _) => {
            h718
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, _, _, h719, _) => {
            h719
        }
    }, match v1 {
        Adt103::C0(_, _, _, _, _, _, h720) => {
            h720
        }
    })))
}

pub fn f175(v0: Adt100, v1: u64, v2: u64, v3: Adt103, v4: bool) -> R<Adt115> {
    let v5: u64 = {
        let a722 = match v3.clone() {
            Adt103::C0(_, _, _, h721, _, _, _) => {
                h721
            }
        };
        let a723 = 0u64;
        if if nat_eq(a722, a723) {
            let a724 = v2;
            let a725 = 48u64;
            nat_eq(a724, a725)
        } else {
            false
        } {
            0u64
        } else {
            let a727 = match v3.clone() {
                Adt103::C0(_, _, _, h726, _, _, _) => {
                    h726
                }
            };
            let a728 = 1u64;
            nat_add(a727, a728)?
        }
    };
    let v6: i64 = if v4 {
        f216(match v3.clone() {
            Adt103::C0(_, _, _, _, h729, _, _) => {
                h729
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt103::C0(_, _, _, _, h730, _, _) => {
                h730
            }
        }
    };
    let a731 = f221();
    let a732 = v5;
    if nat_lt(a731, a732) {
        Ok(f129(Adt88::C8(v1)))
    } else {
        let m735 = {
            let a733 = 99999i64;
            let a734 = v6;
            compare(a733, a734)
        };
        if match m735 {
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
            Ok(f129(Adt88::C8(v1)))
        } else {
            Ok(f128(v0, f160(match v3.clone() {
                Adt103::C0(h736, _, _, _, _, _, _) => {
                    h736
                }
            }, if v4 {
                Adt104::C4
            } else {
                Adt104::C2
            }, f216({
                let a738 = match v3.clone() {
                    Adt103::C0(_, _, h737, _, _, _, _) => {
                        h737
                    }
                };
                let a739 = 10i64;
                int_mul(a738, a739)?
            }, f161({
                let a740 = v2;
                let a741 = 48u64;
                nat_sub(a740, a741)
            }))?, v5, v6, match v3.clone() {
                Adt103::C0(_, _, _, _, _, h742, _) => {
                    h742
                }
            }, match v3 {
                Adt103::C0(_, _, _, _, _, _, h743) => {
                    h743
                }
            })))
        }
    }
}

pub fn f176(v0: u64) -> bool {
    let a744 = v0;
    let a745 = 101u64;
    if nat_eq(a744, a745) {
        true
    } else {
        let a746 = v0;
        let a747 = 69u64;
        nat_eq(a746, a747)
    }
}

pub fn f177(v0: u64) -> bool {
    f120(v0, 48u64, 57u64)
}

pub fn f178(v0: Adt100, v1: u64, v2: u64, v3: Adt103) -> R<Adt115> {
    let m748 = f121(v3)?;
    match m748 {
        Adt120::C0 => {
            Ok(f129(Adt88::C7(v1)))
        }
        Adt120::C1 => {
            Ok(f129(Adt88::C8(v1)))
        }
        Adt120::C2(v4) => {
            Ok(f126(f122(v4, match v0.clone() {
                Adt100::C0(_, h749, _) => {
                    h749
                }
            }, match v0 {
                Adt100::C0(_, _, h750) => {
                    h750
                }
            }), v1, v2))
        }
    }
}

pub fn f179(v0: Adt100, v1: u64, v2: u64, v3: Adt103, v4: bool) -> R<Adt115> {
    let v5: i64 = f216({
        let a752 = match v3.clone() {
            Adt103::C0(_, _, _, _, _, _, h751) => {
                h751
            }
        };
        let a753 = 10i64;
        int_mul(a752, a753)?
    }, f161({
        let a754 = v2;
        let a755 = 48u64;
        nat_sub(a754, a755)
    }))?;
    let m758 = {
        let a756 = 99999i64;
        let a757 = v5;
        compare(a756, a757)
    };
    if match m758 {
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
        Ok(f129(Adt88::C8(v1)))
    } else {
        Ok(f128(v0, f160(match v3.clone() {
            Adt103::C0(h759, _, _, _, _, _, _) => {
                h759
            }
        }, Adt104::C7, match v3.clone() {
            Adt103::C0(_, _, h760, _, _, _, _) => {
                h760
            }
        }, match v3.clone() {
            Adt103::C0(_, _, _, h761, _, _, _) => {
                h761
            }
        }, match v3 {
            Adt103::C0(_, _, _, _, h762, _, _) => {
                h762
            }
        }, v4, v5)))
    }
}

pub fn f180(v0: u64, v1: Adt6, v2: List<Adt106>, v3: List<u64>) -> R<Adt121> {
    let m763 = v1;
    match m763 {
        Adt6::C0 => {
            Ok(Adt121::C1(v2, f95(f222(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt121::C1(v2, f95(if v4 {
                f222(0u64)
            } else {
                f222(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f223(v5.clone())? {
                Ok(Adt121::C1(v2, f95(f224(v5)?, v3)))
            } else {
                Ok(Adt121::C0(Adt88::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt121::C1(v2, f95(f181(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a764 = v0;
            let a765 = f218();
            if nat_lt(a764, a765) {
                Ok(Adt121::C1(f225(91u64, Adt106::C2({
                    let a766 = v0;
                    let a767 = 1u64;
                    nat_add(a766, a767)?
                }, v7), v2), v3))
            } else {
                Ok(Adt121::C0(Adt88::C12))
            }
        }
        Adt6::C5(v8) => {
            let a768 = v0;
            let a769 = f218();
            if nat_lt(a768, a769) {
                Ok(Adt121::C1(f225(123u64, Adt106::C4({
                    let a770 = v0;
                    let a771 = 1u64;
                    nat_add(a770, a771)?
                }, v8), v2), v3))
            } else {
                Ok(Adt121::C0(Adt88::C12))
            }
        }
    }
}

pub fn f181(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a773 = f95(f226(f48({
            let a772 = v0;
            utf8_encode(a772)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a774 = List::cons(34u64, List::<u64>::nil());
        append_list(a773, a774)
    }))
}

pub fn f182(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f227(match v0.clone() {
        Adt33::C0(_, h775, _, _) => {
            h775
        }
    })), List::cons((Str::lit("capabilities"), f228(match v0.clone() {
        Adt33::C0(_, _, h776, _) => {
            h776
        }
    })), {
        let a779 = f229(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h777) => {
                h777
            }
        });
        let a780 = f230(match v0 {
            Adt33::C0(h778, _, _, _) => {
                h778
            }
        })?;
        append_list(a779, a780)
    })))
}

pub fn f183(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f231(match v0.clone() {
        Adt34::C0(_, _, h781) => {
            h781
        }
    }))), {
        let a784 = f229(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h782, _) => {
                h782
            }
        });
        let a785 = f230(match v0 {
            Adt34::C0(h783, _, _) => {
                h783
            }
        })?;
        append_list(a784, a785)
    }))
}

pub fn f184(v0: Adt37) -> Option<List<(Str, Adt6)>> {
    let m787 = f232(match v0.clone() {
        Adt37::C0(h786, _, _) => {
            h786
        }
    });
    match m787 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a790 = f233(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h788, _) => {
                        h788
                    }
                });
                let a791 = f234(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h789) => {
                        h789
                    }
                });
                append_list(a790, a791)
            }))
        }
    }
}

pub fn f185(v0: Adt127) -> Str {
    let m792 = v0;
    match m792 {
        Adt127::C0 => {
            Str::lit("complete")
        }
        Adt127::C1 => {
            Str::lit("input_required")
        }
        Adt127::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f186(v0: Adt32) -> Adt127 {
    let m793 = v0;
    match m793 {
        Adt32::C0(_) => {
            Adt127::C0
        }
        Adt32::C1(_) => {
            Adt127::C0
        }
        Adt32::C2(_) => {
            Adt127::C0
        }
        Adt32::C3(_) => {
            Adt127::C0
        }
        Adt32::C4(_) => {
            Adt127::C0
        }
        Adt32::C5(_) => {
            Adt127::C0
        }
        Adt32::C6(_) => {
            Adt127::C0
        }
        Adt32::C7(_) => {
            Adt127::C0
        }
        Adt32::C8(_) => {
            Adt127::C0
        }
        Adt32::C9(_) => {
            Adt127::C0
        }
        Adt32::C10 => {
            Adt127::C0
        }
        Adt32::C11(_) => {
            Adt127::C1
        }
        Adt32::C12(_) => {
            Adt127::C2
        }
    }
}

pub fn f187(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m794 = v0;
    match m794 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f235(), f236(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f188() -> i64 {
    -32602i64
}

pub fn f189() -> i64 {
    -32022i64
}

pub fn f190() -> i64 {
    -32020i64
}

pub fn f191() -> i64 {
    -32601i64
}

pub fn f192(v0: Adt82) -> i64 {
    let m795 = v0;
    match m795 {
        Adt82::C0(_) => {
            f188()
        }
        Adt82::C1(_) => {
            f188()
        }
        Adt82::C2(_) => {
            f188()
        }
        Adt82::C3(_) => {
            f188()
        }
        Adt82::C4(_) => {
            f188()
        }
        Adt82::C5(_) => {
            f237()
        }
        Adt82::C6(_) => {
            f238()
        }
    }
}

pub fn f193(v0: Adt111) -> Str {
    let m796 = v0;
    match m796 {
        Adt111::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt111::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt111::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt111::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt111::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt111::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt111::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt111::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt111::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f194(v0: Adt94) -> Str {
    let m797 = v0;
    match m797 {
        Adt94::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt94::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt94::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt94::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt94::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt94::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt94::C6(v1) => {
            let m798 = v1;
            match m798 {
                Adt95::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt95::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt95::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt94::C7(v2) => {
            let m799 = v2;
            match m799 {
                Adt95::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt95::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt95::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt94::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt94::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt94::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f195(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a811 = f239(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h800, _, _, _, _) => {
                h800
            }
        });
        let a812 = {
            let a809 = f239(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h801, _, _, _) => {
                    h801
                }
            });
            let a810 = {
                let a807 = f239(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h802, _, _) => {
                        h802
                    }
                });
                let a808 = {
                    let a805 = f239(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h803, _) => {
                            h803
                        }
                    });
                    let a806 = f239(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h804) => {
                            h804
                        }
                    });
                    append_list(a805, a806)
                };
                append_list(a807, a808)
            };
            append_list(a809, a810)
        };
        append_list(a811, a812)
    })
}

pub fn f196(v0: Adt0, v1: Fn0, v2: Adt116, v3: bool) -> Result<Adt108, Adt110> {
    if v3 {
        f240(v0, v1, f241(match v2.clone() {
            Adt116::C0(_, h813, _, _) => {
                h813
            }
        }, match v2.clone() {
            Adt116::C0(_, _, h814, _) => {
                h814
            }
        }, match v2 {
            Adt116::C0(_, _, _, h815) => {
                h815
            }
        }))
    } else {
        Err::<Adt108, Adt110>(Adt110::C3(f242(match v2 {
            Adt116::C0(_, h816, _, _) => {
                h816
            }
        })))
    }
}

pub fn f197(v0: Adt4, v1: Adt117) -> bool {
    let m817 = v1;
    match m817 {
        Adt117::C0 => {
            true
        }
        Adt117::C1 => {
            f243(match v0 {
                Adt4::C0(_, _, _, _, _, h818, _) => {
                    h818
                }
            })
        }
        Adt117::C2 => {
            f243(match v0 {
                Adt4::C0(_, _, _, _, _, h819, _) => {
                    h819
                }
            })
        }
        Adt117::C3 => {
            f244(match v0 {
                Adt4::C0(_, _, _, _, h820, _, _) => {
                    h820
                }
            })
        }
        Adt117::C4 => {
            f244(match v0 {
                Adt4::C0(_, _, _, _, h821, _, _) => {
                    h821
                }
            })
        }
        Adt117::C5 => {
            f244(match v0 {
                Adt4::C0(_, _, _, _, h822, _, _) => {
                    h822
                }
            })
        }
        Adt117::C6 => {
            f243(match v0 {
                Adt4::C0(_, _, _, h823, _, _, _) => {
                    h823
                }
            })
        }
        Adt117::C7 => {
            f243(match v0 {
                Adt4::C0(_, _, _, h824, _, _, _) => {
                    h824
                }
            })
        }
        Adt117::C8 => {
            f245(match v0 {
                Adt4::C0(_, _, h825, _, _, _, _) => {
                    h825
                }
            })
        }
        Adt117::C9 => {
            f246(match v0 {
                Adt4::C0(_, _, _, _, _, _, h826) => {
                    h826
                }
            }, f247())
        }
        Adt117::C10 => {
            f246(match v0 {
                Adt4::C0(_, _, _, _, _, _, h827) => {
                    h827
                }
            }, f247())
        }
        Adt117::C11 => {
            f246(match v0 {
                Adt4::C0(_, _, _, _, _, _, h828) => {
                    h828
                }
            }, f247())
        }
    }
}

pub fn f198(v0: Adt124, v1: Adt122) -> Result<Adt116, Adt110> {
    let m829 = v1;
    match m829 {
        Adt122::C0 => {
            Err::<Adt116, Adt110>(Adt110::C3(match v0 {
                Adt124::C0(_, h830, _, _) => {
                    h830
                }
            }))
        }
        Adt122::C1(v2) => {
            Ok::<Adt116, Adt110>(Adt116::C0(match v0.clone() {
                Adt124::C0(h831, _, _, _) => {
                    h831
                }
            }, v2, match v0.clone() {
                Adt124::C0(_, _, h832, _) => {
                    h832
                }
            }, match v0 {
                Adt124::C0(_, _, _, h833) => {
                    h833
                }
            }))
        }
    }
}

pub fn f199(v0: Option<Adt93>, v1: List<Str>, v2: Adt92, v3: Result<Adt16, Adt111>) -> R<Adt123> {
    let m834 = v3;
    match m834 {
        Ok(v5) => {
            Ok(f248(v1, v2.clone(), v5.clone(), f249(v0, f148(match v2.clone() {
                Adt92::C0(_, h835, _) => {
                    h835
                }
            }), match v2.clone() {
                Adt92::C0(_, h836, _) => {
                    h836
                }
            }, match v5 {
                Adt16::C0(h837, _, _, _, _, _) => {
                    h837
                }
            }, f250(match v2 {
                Adt92::C0(_, _, h838) => {
                    h838
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt123::C1(Adt110::C0(v4)))
        }
    }
}

pub fn f200(v0: Option<Adt5>) -> Result<Adt16, Adt111> {
    let m839 = v0;
    match m839 {
        None => {
            Err::<Adt16, Adt111>(Adt111::C0)
        }
        Some(v1) => {
            let m840 = f86(Str::lit("_meta"), v1);
            match m840 {
                Adt114::C0 => {
                    Err::<Adt16, Adt111>(Adt111::C1)
                }
                Adt114::C1(v2) => {
                    let m841 = f217(v2);
                    match m841 {
                        None => {
                            Err::<Adt16, Adt111>(Adt111::C4)
                        }
                        Some(v3) => {
                            f251(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f201(p846: Bytes, p847: List<u64>) -> Bytes {
    let m845 = (p846, p847);
    {
        let mut m845 = m845;
        loop {
            let step = {
                let (v0, v1) = m845;
                let m842 = v1;
                match m842.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let a843 = v0;
                            let a844 = f252(v2);
                            append_bytes(a843, a844)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202() -> Bytes {
    Bytes::lit(&[])
}

pub fn f203(p854: List<u64>, p855: List<u64>) -> R<bool> {
    let m853 = (p854, p855);
    {
        let mut m853 = m853;
        loop {
            let step = {
                let (v0, v1) = m853;
                let m848 = v0;
                match m848.uncons() {
                    None => {
                        let m849 = v1;
                        match m849.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m850 = v1;
                        match m850.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a851 = f253(v4)?;
                                let a852 = f253(v6)?;
                                if nat_eq(a851, a852) {
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
                Ok(next) => { m853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Adt119, v1: List<u64>) -> Adt119 {
    let m856 = v0;
    match m856 {
        Adt119::C0 => {
            Adt119::C1(v1)
        }
        Adt119::C1(_) => {
            Adt119::C2
        }
        Adt119::C2 => {
            Adt119::C2
        }
    }
}

pub fn f205(v0: List<u64>) -> List<u64> {
    f211(f254(f211(f254(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f206(p859: List<u64>) -> bool {
    let m858 = p859;
    {
        let mut m858 = m858;
        loop {
            let step = {
                let v0 = m858;
                let m857 = v0;
                match m857.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f255(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m858 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Adt117) -> Adt125 {
    let m860 = v0;
    match m860 {
        Adt117::C0 => {
            Adt125::C0
        }
        Adt117::C1 => {
            Adt125::C0
        }
        Adt117::C2 => {
            Adt125::C1
        }
        Adt117::C3 => {
            Adt125::C0
        }
        Adt117::C4 => {
            Adt125::C2
        }
        Adt117::C5 => {
            Adt125::C0
        }
        Adt117::C6 => {
            Adt125::C0
        }
        Adt117::C7 => {
            Adt125::C1
        }
        Adt117::C8 => {
            Adt125::C0
        }
        Adt117::C9 => {
            Adt125::C3
        }
        Adt117::C10 => {
            Adt125::C3
        }
        Adt117::C11 => {
            Adt125::C3
        }
    }
}

pub fn f208(p866: List<u64>, p867: List<u64>) -> Option<List<u64>> {
    let m865 = (p866, p867);
    {
        let mut m865 = m865;
        loop {
            let step = {
                let (v0, v1) = m865;
                let m861 = v0;
                match m861.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m862 = v1;
                        match m862.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a863 = v4;
                                let a864 = v2;
                                if nat_eq(a863, a864) {
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
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f210() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f211(p870: List<u64>, p871: List<u64>) -> List<u64> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v0, v1) = m869;
                let m868 = v0;
                match m868.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f212(p882: u64, p883: List<u64>, p884: List<u64>) -> R<Option<List<u64>>> {
    let m881 = (p882, (p883, p884));
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v0, (v1, v2)) = m881;
                let m872 = v0;
                if m872 == 0 {
                    let m873 = v1;
                    match m873.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m872 - 1;
                    let m874 = v1;
                    match m874.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m875 = v7;
                            match m875.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m876 = v9;
                                    match m876.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m877 = v11;
                                            match m877.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m878 = f256(v6, v8, v10, v12, f257(v13.clone()))?;
                                                    match m878 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, {
                                                                let a879 = v2;
                                                                let a880 = v14;
                                                                append_list(a879, a880)
                                                            })))
                                                        }
                                                    }
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
                Ok(next) => { m881 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f214(v0: Str) -> R<List<u64>> {
    f48({
        let a885 = v0;
        utf8_encode(a885)
    })
}

pub fn f215(v0: i64) -> bool {
    let a892 = {
        let m888 = {
            let a886 = v0;
            let a887 = 0i64;
            compare(a886, a887)
        };
        if match m888 {
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
            let m891 = {
                let a889 = 0i64;
                let a890 = v0;
                compare(a889, a890)
            };
            match m891 {
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
    bool_not(a892)
}

pub fn f216(v0: i64, v1: i64) -> R<i64> {
    let a894 = v0;
    let a895 = {
        let a893 = v1;
        int_neg(a893)?
    };
    int_sub(a894, a895)
}

pub fn f217(v0: Adt6) -> Option<Adt5> {
    let m896 = v0;
    match m896 {
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

pub fn f218() -> u64 {
    512u64
}

pub fn f219(v0: Adt100, v1: u64, v2: Str) -> Adt115 {
    let m898 = match v0.clone() {
        Adt100::C0(_, h897, _) => {
            h897
        }
    };
    match m898.uncons() {
        None => {
            f129(Adt88::C2(v1))
        }
        Some((v3, v4)) => {
            let m899 = v3;
            match m899 {
                Adt105::C0(_) => {
                    f129(Adt88::C2(v1))
                }
                Adt105::C1(v6, _) => {
                    Adt115::C1(Adt100::C0(Adt101::C5, List::cons(Adt105::C1(v6, Some(v2)), v4), match v0 {
                        Adt100::C0(_, _, h900) => {
                            h900
                        }
                    }))
                }
            }
        }
    }
}

pub fn f220(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a901 = v0;
    let a902 = 128u64;
    if nat_lt(a901, a902) {
        Ok(List::cons(v0, v1))
    } else {
        let a903 = v0;
        let a904 = 2048u64;
        if nat_lt(a903, a904) {
            Ok(List::cons({
                let a905 = 128u64;
                let a906 = f258(v0, 64u64);
                nat_add(a905, a906)?
            }, List::cons({
                let a907 = 192u64;
                let a908 = f259(v0, 64u64);
                nat_add(a907, a908)?
            }, v1)))
        } else {
            let a909 = v0;
            let a910 = 65536u64;
            if nat_lt(a909, a910) {
                Ok(List::cons({
                    let a911 = 128u64;
                    let a912 = f258(v0, 64u64);
                    nat_add(a911, a912)?
                }, List::cons({
                    let a913 = 128u64;
                    let a914 = f258(f259(v0, 64u64), 64u64);
                    nat_add(a913, a914)?
                }, List::cons({
                    let a915 = 224u64;
                    let a916 = f259(v0, 4096u64);
                    nat_add(a915, a916)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a917 = 128u64;
                    let a918 = f258(v0, 64u64);
                    nat_add(a917, a918)?
                }, List::cons({
                    let a919 = 128u64;
                    let a920 = f258(f259(v0, 64u64), 64u64);
                    nat_add(a919, a920)?
                }, List::cons({
                    let a921 = 128u64;
                    let a922 = f258(f259(v0, 4096u64), 64u64);
                    nat_add(a921, a922)?
                }, List::cons({
                    let a923 = 240u64;
                    let a924 = f259(v0, 262144u64);
                    nat_add(a923, a924)?
                }, v1)))))
            }
        }
    }
}

pub fn f221() -> u64 {
    18u64
}

pub fn f222(v0: u64) -> List<u64> {
    f171(v0)
}

pub fn f223(v0: Adt7) -> R<bool> {
    let a942 = {
        let m928 = {
            let a926 = f260();
            let a927 = match v0.clone() {
                Adt7::C0(h925, _) => {
                    h925
                }
            };
            compare(a926, a927)
        };
        if match m928 {
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
            let m933 = {
                let a931 = match v0.clone() {
                    Adt7::C0(h929, _) => {
                        h929
                    }
                };
                let a932 = {
                    let a930 = f260();
                    int_neg(a930)?
                };
                compare(a931, a932)
            };
            if match m933 {
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
                let m937 = {
                    let a935 = 9999i64;
                    let a936 = match v0.clone() {
                        Adt7::C0(_, h934) => {
                            h934
                        }
                    };
                    compare(a935, a936)
                };
                if match m937 {
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
                    let m941 = {
                        let a939 = match v0 {
                            Adt7::C0(_, h938) => {
                                h938
                            }
                        };
                        let a940 = -9999i64;
                        compare(a939, a940)
                    };
                    match m941 {
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
    Ok(bool_not(a942))
}

pub fn f224(v0: Adt7) -> R<List<u64>> {
    let m946 = {
        let a944 = match v0.clone() {
            Adt7::C0(_, h943) => {
                h943
            }
        };
        let a945 = 0i64;
        compare(a944, a945)
    };
    if if match m946 {
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
        let m950 = {
            let a948 = 0i64;
            let a949 = match v0.clone() {
                Adt7::C0(_, h947) => {
                    h947
                }
            };
            compare(a948, a949)
        };
        match m950 {
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
        let a953 = f261(match v0.clone() {
            Adt7::C0(h951, _) => {
                h951
            }
        })?;
        let a954 = List::cons(101u64, f261(match v0 {
            Adt7::C0(_, h952) => {
                h952
            }
        })?);
        Ok(append_list(a953, a954))
    } else {
        f261(match v0 {
            Adt7::C0(h955, _) => {
                h955
            }
        })
    }
}

pub fn f225(v0: u64, v1: Adt106, v2: List<Adt106>) -> List<Adt106> {
    List::cons(Adt106::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f226(p958: List<u64>, p959: List<u64>) -> R<List<u64>> {
    let m957 = (p958, p959);
    {
        let mut m957 = m957;
        loop {
            let step = {
                let (v0, v1) = m957;
                let m956 = v0;
                match m956.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f95(f262(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m957 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: List<Str>) -> Adt6 {
    Adt6::C4(f52(v0))
}

pub fn f228(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a977 = f239(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h960, _, _, _, _, _, _) => {
                h960
            }
        });
        let a978 = {
            let a975 = f239(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h961, _, _, _, _, _) => {
                    h961
                }
            });
            let a976 = {
                let a973 = f239(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h962, _, _, _, _) => {
                        h962
                    }
                });
                let a974 = {
                    let a971 = f263(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h963, _, _, _) => {
                            h963
                        }
                    });
                    let a972 = {
                        let a969 = f264(match v0.clone() {
                            Adt4::C0(_, _, _, _, h964, _, _) => {
                                h964
                            }
                        });
                        let a970 = {
                            let a967 = f263(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h965, _) => {
                                    h965
                                }
                            });
                            let a968 = f239(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h966) => {
                                    h966
                                }
                            });
                            append_list(a967, a968)
                        };
                        append_list(a969, a970)
                    };
                    append_list(a971, a972)
                };
                append_list(a973, a974)
            };
            append_list(a975, a976)
        };
        append_list(a977, a978)
    })
}

pub fn f229(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m979 = v1;
    match m979 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f230(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f265(match v0.clone() {
        Adt10::C0(h980, _) => {
            h980
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f266(match v0 {
        Adt10::C0(_, h981) => {
            h981
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f231(v0: List<Adt35>) -> List<Adt6> {
    let m982 = v0;
    match m982.uncons() {
        None => {
            List::<Adt6>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f267(v1), f231(v2))
        }
    }
}

pub fn f232(v0: List<Adt38>) -> Option<List<Adt6>> {
    let m983 = v0;
    match m983.uncons() {
        None => {
            Some(List::<Adt6>::nil())
        }
        Some((v1, v2)) => {
            let m984 = f268(v1);
            match m984 {
                None => {
                    None::<List<Adt6>>
                }
                Some(v3) => {
                    let m985 = f232(v2);
                    match m985 {
                        None => {
                            None::<List<Adt6>>
                        }
                        Some(v4) => {
                            Some(List::cons(v3, v4))
                        }
                    }
                }
            }
        }
    }
}

pub fn f233(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m986 = v1;
    match m986 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f234(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m987 = v1;
    match m987 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f235() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f236(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h988, _, _, _, _, _) => {
            h988
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h989, _, _, _) => {
            h989
        }
    })), {
        let a998 = f229(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h990, _, _, _, _) => {
                h990
            }
        });
        let a999 = {
            let a996 = f229(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h991, _, _) => {
                    h991
                }
            });
            let a997 = {
                let a994 = f229(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h992, _) => {
                        h992
                    }
                });
                let a995 = f269(match v0 {
                    Adt1::C0(_, _, _, _, _, h993) => {
                        h993
                    }
                });
                append_list(a994, a995)
            };
            append_list(a996, a997)
        };
        append_list(a998, a999)
    })))
}

pub fn f237() -> i64 {
    -32021i64
}

pub fn f238() -> i64 {
    -32603i64
}

pub fn f239(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1000 = v1;
    match m1000 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1001) => {
                    h1001
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f240(v0: Adt0, v1: Fn0, v2: Result<Adt14, Adt111>) -> Result<Adt108, Adt110> {
    let m1002 = v2;
    match m1002 {
        Ok(v4) => {
            f270(v0, v1, v4)
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(Adt110::C0(v3))
        }
    }
}

pub fn f241(v0: Adt117, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt111> {
    let m1003 = v0;
    match m1003 {
        Adt117::C0 => {
            f271(v1, v2)
        }
        Adt117::C1 => {
            f272(v1, v2)
        }
        Adt117::C2 => {
            f273(v1, v2)
        }
        Adt117::C3 => {
            f274(v1, v2)
        }
        Adt117::C4 => {
            f275(v1, v2)
        }
        Adt117::C5 => {
            f276(v1, v2)
        }
        Adt117::C6 => {
            f277(v1, v2)
        }
        Adt117::C7 => {
            f278(v1, v2)
        }
        Adt117::C8 => {
            f279(v1, v2)
        }
        Adt117::C9 => {
            f280(v1, v2)
        }
        Adt117::C10 => {
            f281(v1, v2)
        }
        Adt117::C11 => {
            f282(v1, v2)
        }
    }
}

pub fn f242(v0: Adt117) -> Str {
    let m1004 = v0;
    match m1004 {
        Adt117::C0 => {
            Str::lit("server/discover")
        }
        Adt117::C1 => {
            Str::lit("tools/list")
        }
        Adt117::C2 => {
            Str::lit("tools/call")
        }
        Adt117::C3 => {
            Str::lit("resources/list")
        }
        Adt117::C4 => {
            Str::lit("resources/read")
        }
        Adt117::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt117::C6 => {
            Str::lit("prompts/list")
        }
        Adt117::C7 => {
            Str::lit("prompts/get")
        }
        Adt117::C8 => {
            Str::lit("completion/complete")
        }
        Adt117::C9 => {
            Str::lit("tasks/get")
        }
        Adt117::C10 => {
            Str::lit("tasks/update")
        }
        Adt117::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f243(v0: Option<Adt8>) -> bool {
    let m1005 = v0;
    match m1005 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f244(v0: Option<Adt9>) -> bool {
    let m1006 = v0;
    match m1006 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f245(v0: Option<Adt5>) -> bool {
    let m1007 = v0;
    match m1007 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f246(v0: Option<Adt5>, v1: Str) -> bool {
    let m1008 = v0;
    match m1008 {
        None => {
            false
        }
        Some(v2) => {
            f117(v1, v2)
        }
    }
}

pub fn f247() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f248(v0: List<Str>, v1: Adt92, v2: Adt16, v3: Option<Adt94>) -> Adt123 {
    let m1009 = v3;
    match m1009 {
        None => {
            f283(v1, v2.clone(), f284(match v2 {
                Adt16::C0(h1010, _, _, _, _, _) => {
                    h1010
                }
            }, v0))
        }
        Some(v4) => {
            Adt123::C1(Adt110::C2(v4))
        }
    }
}

pub fn f249(v0: Option<Adt93>, v1: Adt125, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt94>> {
    let m1011 = v0;
    match m1011 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v5) => {
            let m1013 = match v5.clone() {
                Adt93::C0(_, _, _, h1012, _) => {
                    h1012
                }
            };
            match m1013 {
                None => {
                    f285(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f250(v0: Option<Adt5>) -> Adt5 {
    let m1014 = v0;
    match m1014 {
        None => {
            f53()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f251(v0: Adt5) -> Result<Adt16, Adt111> {
    let m1015 = f286(v0.clone());
    match m1015 {
        Adt128::C0 => {
            f287(v0.clone(), f86(f288(), v0.clone()), f86(f289(), v0))
        }
        Adt128::C1 => {
            Err::<Adt16, Adt111>(Adt111::C2)
        }
        Adt128::C2 => {
            Err::<Adt16, Adt111>(Adt111::C3)
        }
    }
}

pub fn f252(v0: u64) -> Bytes {
    let m1019 = {
        let a1016 = f290();
        let a1017 = v0;
        let a1018 = 1u64;
        slice_bytes(a1016, a1017, a1018)
    };
    match m1019 {
        None => {
            f202()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f253(v0: u64) -> R<u64> {
    let a1020 = 64u64;
    let a1021 = v0;
    if if nat_lt(a1020, a1021) {
        let a1022 = v0;
        let a1023 = 91u64;
        nat_lt(a1022, a1023)
    } else {
        false
    } {
        let a1024 = v0;
        let a1025 = 32u64;
        nat_add(a1024, a1025)
    } else {
        Ok(v0)
    }
}

pub fn f254(p1028: List<u64>) -> List<u64> {
    let m1027 = p1028;
    {
        let mut m1027 = m1027;
        loop {
            let step = {
                let v0 = m1027;
                let m1026 = v0;
                match m1026.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f291(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f255(v0: u64) -> bool {
    let a1029 = v0;
    let a1030 = 9u64;
    if nat_eq(a1029, a1030) {
        true
    } else {
        let a1031 = 31u64;
        let a1032 = v0;
        if nat_lt(a1031, a1032) {
            let a1033 = v0;
            let a1034 = 127u64;
            nat_lt(a1033, a1034)
        } else {
            false
        }
    }
}

pub fn f256(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1035 = f292(v0)?;
    match m1035 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1036 = f292(v1)?;
            match m1036 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1037 = v2;
                    let a1038 = 61u64;
                    if nat_eq(a1037, a1038) {
                        f293(v5, v6, v4, v3)
                    } else {
                        let m1039 = f292(v2)?;
                        match m1039 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1040 = v3;
                                let a1041 = 61u64;
                                if nat_eq(a1040, a1041) {
                                    f294(v5, v6, v7, v4)
                                } else {
                                    let m1042 = f292(v3)?;
                                    match m1042 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f295(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f257(v0: List<u64>) -> bool {
    let m1043 = v0;
    match m1043.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f258(v0: u64, v1: u64) -> u64 {
    let a1044 = v0;
    let a1045 = v1;
    let a1046 = 0u64;
    nat_rem(a1044, a1045, a1046)
}

pub fn f259(v0: u64, v1: u64) -> u64 {
    let a1047 = v0;
    let a1048 = v1;
    let a1049 = 0u64;
    nat_quot(a1047, a1048, a1049)
}

pub fn f260() -> i64 {
    999999999999999999i64
}

pub fn f261(v0: i64) -> R<List<u64>> {
    let m1052 = {
        let a1050 = v0;
        let a1051 = 0i64;
        compare(a1050, a1051)
    };
    if match m1052 {
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
        Ok(List::cons(45u64, f296(20u64, {
            let a1053 = v0;
            int_neg(a1053)?
        }, List::<u64>::nil())?))
    } else {
        f296(20u64, v0, List::<u64>::nil())
    }
}

pub fn f262(v0: u64) -> R<List<u64>> {
    let a1054 = v0;
    let a1055 = 34u64;
    if nat_eq(a1054, a1055) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1056 = v0;
        let a1057 = 92u64;
        if nat_eq(a1056, a1057) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1058 = v0;
            let a1059 = 8u64;
            if nat_eq(a1058, a1059) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1060 = v0;
                let a1061 = 9u64;
                if nat_eq(a1060, a1061) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1062 = v0;
                    let a1063 = 10u64;
                    if nat_eq(a1062, a1063) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1064 = v0;
                        let a1065 = 12u64;
                        if nat_eq(a1064, a1065) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1066 = v0;
                            let a1067 = 13u64;
                            if nat_eq(a1066, a1067) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1068 = v0;
                                let a1069 = 32u64;
                                if nat_lt(a1068, a1069) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f297(f259(v0, 16u64))?, List::cons(f297(f258(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f263(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1070 = v1;
    match m1070 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f298(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f264(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1071 = v0;
    match m1071 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1074 = f234(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1072, _) => {
                        h1072
                    }
                });
                let a1075 = f234(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1073) => {
                        h1073
                    }
                });
                append_list(a1074, a1075)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f265(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f299(v0, 0i64)?, 0i64)))
}

pub fn f266(v0: Adt11) -> Str {
    let m1076 = v0;
    match m1076 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f267(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h1077, _, _, _, _, _, _, _) => {
            h1077
        }
    })), {
        let a1095 = f229(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h1078, _, _, _, _, _, _) => {
                h1078
            }
        });
        let a1096 = {
            let a1093 = f269(match v0.clone() {
                Adt35::C0(_, _, h1079, _, _, _, _, _) => {
                    h1079
                }
            });
            let a1094 = {
                let a1091 = f229(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h1080, _, _, _, _) => {
                        h1080
                    }
                });
                let a1092 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m1083 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h1082, _, _, _) => {
                            h1082
                        }
                    };
                    match m1083 {
                        Adt5::C0(h1081) => {
                            h1081
                        }
                    }
                })), {
                    let a1089 = f239(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h1084, _, _) => {
                            h1084
                        }
                    });
                    let a1090 = {
                        let a1087 = f300(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h1085, _) => {
                                h1085
                            }
                        });
                        let a1088 = f301(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h1086) => {
                                h1086
                            }
                        });
                        append_list(a1087, a1088)
                    };
                    append_list(a1089, a1090)
                });
                append_list(a1091, a1092)
            };
            append_list(a1093, a1094)
        };
        append_list(a1095, a1096)
    }))
}

pub fn f268(v0: Adt38) -> Option<Adt6> {
    let m1097 = v0;
    match m1097 {
        Adt38::C0(v1) => {
            Some(f302(v1))
        }
        Adt38::C1(_) => {
            None::<Adt6>
        }
        Adt38::C2(_) => {
            None::<Adt6>
        }
        Adt38::C3(_) => {
            None::<Adt6>
        }
        Adt38::C4(_) => {
            None::<Adt6>
        }
    }
}

pub fn f269(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1098 = v0;
    match m1098 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f303(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f270(v0: Adt0, v1: Fn0, v2: Adt14) -> Result<Adt108, Adt110> {
    if f304(f305(v2.clone())) {
        Ok::<Adt108, Adt110>(f306(v0.clone(), Adt32::C0(f307(v0))))
    } else {
        f308(v0, v2.clone(), {
            let c1099: Fn0 = v1;
            c1099.apply(v2)
        })
    }
}

pub fn f271(v0: Adt16, _: Adt5) -> Result<Adt14, Adt111> {
    Ok::<Adt14, Adt111>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f272(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1100 = f309(Str::lit("cursor"), v1);
    match m1100 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f273(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1101 = f310(Str::lit("name"), v1.clone());
    match m1101 {
        Ok(v2) => {
            let m1102 = f311(Str::lit("arguments"), v1.clone());
            match m1102 {
                Ok(v3) => {
                    let m1103 = f311(Str::lit("inputResponses"), v1.clone());
                    match m1103 {
                        Ok(v4) => {
                            let m1104 = f309(Str::lit("requestState"), v1);
                            match m1104 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt111>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt111>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt111>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt111>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt111>(v9)
        }
    }
}

pub fn f274(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1105 = f309(Str::lit("cursor"), v1);
    match m1105 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f275(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1106 = f310(Str::lit("uri"), v1.clone());
    match m1106 {
        Ok(v2) => {
            let m1107 = f311(Str::lit("inputResponses"), v1.clone());
            match m1107 {
                Ok(v3) => {
                    let m1108 = f309(Str::lit("requestState"), v1);
                    match m1108 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt111>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt111>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt111>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt111>(v7)
        }
    }
}

pub fn f276(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1109 = f309(Str::lit("cursor"), v1);
    match m1109 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f277(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1110 = f309(Str::lit("cursor"), v1);
    match m1110 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f278(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1111 = f310(Str::lit("name"), v1.clone());
    match m1111 {
        Ok(v2) => {
            let m1112 = f312(Str::lit("arguments"), v1.clone());
            match m1112 {
                Ok(v3) => {
                    let m1113 = f311(Str::lit("inputResponses"), v1.clone());
                    match m1113 {
                        Ok(v4) => {
                            let m1114 = f309(Str::lit("requestState"), v1);
                            match m1114 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt111>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt111>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt111>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt111>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt111>(v9)
        }
    }
}

pub fn f279(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1115 = f313(Str::lit("ref"), v1.clone());
    match m1115 {
        Ok(v2) => {
            let m1116 = f314(v2);
            match m1116 {
                Ok(v3) => {
                    let m1117 = f313(Str::lit("argument"), v1.clone());
                    match m1117 {
                        Ok(v4) => {
                            let m1118 = f315(v4);
                            match m1118 {
                                Ok(v5) => {
                                    let m1119 = f316(v1);
                                    match m1119 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt111>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt111>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt111>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt111>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt111>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt111>(v11)
        }
    }
}

pub fn f280(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1120 = f310(Str::lit("taskId"), v1);
    match m1120 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f281(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1121 = f310(Str::lit("taskId"), v1.clone());
    match m1121 {
        Ok(v2) => {
            let m1122 = f313(Str::lit("inputResponses"), v1);
            match m1122 {
                Ok(v3) => {
                    Ok::<Adt14, Adt111>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt111>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt111>(v5)
        }
    }
}

pub fn f282(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1123 = f310(Str::lit("taskId"), v1);
    match m1123 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f283(v0: Adt92, v1: Adt16, v2: Option<Adt110>) -> Adt123 {
    let m1124 = v2;
    match m1124 {
        None => {
            Adt123::C0(Adt124::C0(match v0.clone() {
                Adt92::C0(h1125, _, _) => {
                    h1125
                }
            }, match v0.clone() {
                Adt92::C0(_, h1126, _) => {
                    h1126
                }
            }, v1, f250(match v0 {
                Adt92::C0(_, _, h1127) => {
                    h1127
                }
            })))
        }
        Some(v3) => {
            Adt123::C1(v3)
        }
    }
}

pub fn f284(v0: Str, v1: List<Str>) -> Option<Adt110> {
    f317(f318(v0.clone(), v1.clone()), v0, v1)
}

pub fn f285(v0: Adt93, v1: Adt125, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt94>> {
    let m1129 = match v0.clone() {
        Adt93::C0(h1128, _, _, _, _) => {
            h1128
        }
    };
    match m1129 {
        None => {
            Ok(Some(Adt94::C0))
        }
        Some(v5) => {
            let a1130 = v5;
            let a1131 = v3;
            if equal(a1130, a1131) {
                let m1133 = match v0.clone() {
                    Adt93::C0(_, h1132, _, _, _) => {
                        h1132
                    }
                };
                match m1133 {
                    None => {
                        Ok(Some(Adt94::C1))
                    }
                    Some(v6) => {
                        let a1134 = v6;
                        let a1135 = v2;
                        if equal(a1134, a1135) {
                            f319(f320(match v0.clone() {
                                Adt93::C0(_, _, h1136, _, _) => {
                                    h1136
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt93::C0(_, _, _, _, h1137) => {
                                    h1137
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt94::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt94::C3))
            }
        }
    }
}

pub fn f286(v0: Adt5) -> Adt128 {
    f321(f86(f288(), v0.clone()), f86(f289(), v0))
}

pub fn f287(v0: Adt5, v1: Adt114, v2: Adt114) -> Result<Adt16, Adt111> {
    let m1138 = v1;
    match m1138 {
        Adt114::C0 => {
            Err::<Adt16, Adt111>(Adt111::C2)
        }
        Adt114::C1(v3) => {
            let m1139 = v2;
            match m1139 {
                Adt114::C0 => {
                    Err::<Adt16, Adt111>(Adt111::C3)
                }
                Adt114::C1(v4) => {
                    f322(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f288() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f289() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f290() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f291(v0: u64) -> bool {
    let a1140 = v0;
    let a1141 = 9u64;
    if nat_eq(a1140, a1141) {
        true
    } else {
        let a1142 = v0;
        let a1143 = 32u64;
        nat_eq(a1142, a1143)
    }
}

pub fn f292(v0: u64) -> R<Option<u64>> {
    let a1144 = 64u64;
    let a1145 = v0;
    if if nat_lt(a1144, a1145) {
        let a1146 = v0;
        let a1147 = 91u64;
        nat_lt(a1146, a1147)
    } else {
        false
    } {
        Ok(Some({
            let a1148 = v0;
            let a1149 = 65u64;
            nat_sub(a1148, a1149)
        }))
    } else {
        let a1150 = 96u64;
        let a1151 = v0;
        if if nat_lt(a1150, a1151) {
            let a1152 = v0;
            let a1153 = 123u64;
            nat_lt(a1152, a1153)
        } else {
            false
        } {
            Ok(Some({
                let a1154 = v0;
                let a1155 = 71u64;
                nat_sub(a1154, a1155)
            }))
        } else {
            let a1156 = 47u64;
            let a1157 = v0;
            if if nat_lt(a1156, a1157) {
                let a1158 = v0;
                let a1159 = 58u64;
                nat_lt(a1158, a1159)
            } else {
                false
            } {
                Ok(Some({
                    let a1160 = v0;
                    let a1161 = 4u64;
                    nat_add(a1160, a1161)?
                }))
            } else {
                let a1162 = v0;
                let a1163 = 43u64;
                if nat_eq(a1162, a1163) {
                    Ok(Some(62u64))
                } else {
                    let a1164 = v0;
                    let a1165 = 47u64;
                    if nat_eq(a1164, a1165) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f293(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1166 = v3;
        let a1167 = 61u64;
        if nat_eq(a1166, a1167) {
            let a1168 = f323(v1, 16u64);
            let a1169 = 0u64;
            nat_eq(a1168, a1169)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f324(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f294(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1170 = f323(v2, 4u64);
        let a1171 = 0u64;
        nat_eq(a1170, a1171)
    } else {
        false
    } {
        Ok(Some(f325(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f295(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1174 = {
            let a1172 = v0;
            let a1173 = 4u64;
            nat_mul(a1172, a1173)?
        };
        let a1175 = f326(v1, 16u64);
        nat_add(a1174, a1175)?
    }, List::cons({
        let a1178 = {
            let a1176 = f323(v1, 16u64);
            let a1177 = 16u64;
            nat_mul(a1176, a1177)?
        };
        let a1179 = f326(v2, 4u64);
        nat_add(a1178, a1179)?
    }, List::cons({
        let a1182 = {
            let a1180 = f323(v2, 4u64);
            let a1181 = 64u64;
            nat_mul(a1180, a1181)?
        };
        let a1183 = v3;
        nat_add(a1182, a1183)?
    }, List::<u64>::nil()))))
}

pub fn f296(p1195: u64, p1196: i64, p1197: List<u64>) -> R<List<u64>> {
    let m1194 = (p1195, (p1196, p1197));
    {
        let mut m1194 = m1194;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1194;
                let m1184 = v0;
                if m1184 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1184 - 1;
                    let v4: i64 = {
                        let a1185 = v1;
                        let a1186 = 10i64;
                        let a1187 = 0i64;
                        int_quot(a1185, a1186, a1187)?
                    };
                    let v5: i64 = {
                        let a1188 = v1;
                        let a1189 = 10i64;
                        let a1190 = 0i64;
                        int_rem(a1188, a1189, a1190)
                    };
                    let m1193 = {
                        let a1191 = v4;
                        let a1192 = 1i64;
                        compare(a1191, a1192)
                    };
                    if match m1193 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f327(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f327(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1194 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: u64) -> R<u64> {
    let a1198 = v0;
    let a1199 = 10u64;
    if nat_lt(a1198, a1199) {
        let a1200 = 48u64;
        let a1201 = v0;
        nat_add(a1200, a1201)
    } else {
        let a1202 = 87u64;
        let a1203 = v0;
        nat_add(a1202, a1203)
    }
}

pub fn f298(v0: Adt8) -> Adt6 {
    Adt6::C5(f234(Str::lit("listChanged"), match v0 {
        Adt8::C0(h1204) => {
            h1204
        }
    }))
}

pub fn f299(p1209: u64, p1210: i64) -> R<i64> {
    let m1208 = (p1209, p1210);
    {
        let mut m1208 = m1208;
        loop {
            let step = {
                let (v0, v1) = m1208;
                let m1205 = v0;
                if m1205 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m1205 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a1206 = v1;
                        let a1207 = -1i64;
                        int_sub(a1206, a1207)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1208 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m1211 = v0;
    match m1211 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a1223 = f229(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h1212, _, _, _, _) => {
                        h1212
                    }
                });
                let a1224 = {
                    let a1221 = f234(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h1213, _, _, _) => {
                            h1213
                        }
                    });
                    let a1222 = {
                        let a1219 = f234(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h1214, _, _) => {
                                h1214
                            }
                        });
                        let a1220 = {
                            let a1217 = f234(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h1215, _) => {
                                    h1215
                                }
                            });
                            let a1218 = f234(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h1216) => {
                                    h1216
                                }
                            });
                            append_list(a1217, a1218)
                        };
                        append_list(a1219, a1220)
                    };
                    append_list(a1221, a1222)
                };
                append_list(a1223, a1224)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f301(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f239(Str::lit("_meta"), v0)
}

pub fn f302(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h1225, _, _) => {
            h1225
        }
    })), {
        let a1228 = f328(match v0.clone() {
            Adt39::C0(_, h1226, _) => {
                h1226
            }
        });
        let a1229 = f301(match v0 {
            Adt39::C0(_, _, h1227) => {
                h1227
            }
        });
        append_list(a1228, a1229)
    })))
}

pub fn f303(v0: List<Adt2>) -> List<Adt6> {
    let m1230 = v0;
    match m1230.uncons() {
        None => {
            List::<Adt6>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f329(v1), f303(v2))
        }
    }
}

pub fn f304(v0: Adt117) -> bool {
    let m1231 = v0;
    match m1231 {
        Adt117::C0 => {
            true
        }
        Adt117::C1 => {
            false
        }
        Adt117::C2 => {
            false
        }
        Adt117::C3 => {
            false
        }
        Adt117::C4 => {
            false
        }
        Adt117::C5 => {
            false
        }
        Adt117::C6 => {
            false
        }
        Adt117::C7 => {
            false
        }
        Adt117::C8 => {
            false
        }
        Adt117::C9 => {
            false
        }
        Adt117::C10 => {
            false
        }
        Adt117::C11 => {
            false
        }
    }
}

pub fn f305(v0: Adt14) -> Adt117 {
    let m1232 = v0;
    match m1232 {
        Adt14::C0(_) => {
            Adt117::C0
        }
        Adt14::C1(_) => {
            Adt117::C1
        }
        Adt14::C2(_) => {
            Adt117::C2
        }
        Adt14::C3(_) => {
            Adt117::C3
        }
        Adt14::C4(_) => {
            Adt117::C4
        }
        Adt14::C5(_) => {
            Adt117::C5
        }
        Adt14::C6(_) => {
            Adt117::C6
        }
        Adt14::C7(_) => {
            Adt117::C7
        }
        Adt14::C8(_) => {
            Adt117::C8
        }
        Adt14::C9(_) => {
            Adt117::C9
        }
        Adt14::C10(_) => {
            Adt117::C10
        }
        Adt14::C11(_) => {
            Adt117::C11
        }
    }
}

pub fn f306(v0: Adt0, v1: Adt32) -> Adt108 {
    Adt108::C0(v1, Adt109::C0(Some(match v0 {
        Adt0::C0(h1233, _, _, _, _) => {
            h1233
        }
    }), None::<Adt5>))
}

pub fn f307(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h1234, _) => {
            h1234
        }
    }, f84(), match v0.clone() {
        Adt0::C0(_, _, h1235, _, _) => {
            h1235
        }
    }, match v0 {
        Adt0::C0(_, h1236, _, _, _) => {
            h1236
        }
    })
}

pub fn f308(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt82>) -> Result<Adt108, Adt110> {
    let m1237 = v2;
    match m1237 {
        Ok(v4) => {
            if f330(f305(v1.clone()), v4.clone()) {
                f331(v0, v1, v4)
            } else {
                Err::<Adt108, Adt110>(Adt110::C4(f332()))
            }
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(Adt110::C4(v3))
        }
    }
}

pub fn f309(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt111> {
    let m1238 = f86(v0.clone(), v1);
    match m1238 {
        Adt114::C0 => {
            Ok::<Option<Str>, Adt111>(None::<Str>)
        }
        Adt114::C1(v2) => {
            let m1239 = f113(v2);
            match m1239 {
                None => {
                    Err::<Option<Str>, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt111>(Some(v3))
                }
            }
        }
    }
}

pub fn f310(v0: Str, v1: Adt5) -> Result<Str, Adt111> {
    let m1240 = f86(v0.clone(), v1);
    match m1240 {
        Adt114::C0 => {
            Err::<Str, Adt111>(Adt111::C7(v0))
        }
        Adt114::C1(v2) => {
            let m1241 = f113(v2);
            match m1241 {
                None => {
                    Err::<Str, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt111>(v3)
                }
            }
        }
    }
}

pub fn f311(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt111> {
    let m1242 = f86(v0.clone(), v1);
    match m1242 {
        Adt114::C0 => {
            Ok::<Option<Adt5>, Adt111>(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m1243 = f217(v2);
            match m1243 {
                None => {
                    Err::<Option<Adt5>, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt111>(Some(v3))
                }
            }
        }
    }
}

pub fn f312(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt111> {
    let m1244 = f86(v0.clone(), v1);
    match m1244 {
        Adt114::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt111>(None::<List<(Str, Str)>>)
        }
        Adt114::C1(v2) => {
            let m1245 = f217(v2);
            match m1245 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    let m1247 = f333(v0, match v3 {
                        Adt5::C0(h1246) => {
                            h1246
                        }
                    });
                    match m1247 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt111>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt111>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f313(v0: Str, v1: Adt5) -> Result<Adt5, Adt111> {
    let m1248 = f86(v0.clone(), v1);
    match m1248 {
        Adt114::C0 => {
            Err::<Adt5, Adt111>(Adt111::C7(v0))
        }
        Adt114::C1(v2) => {
            let m1249 = f217(v2);
            match m1249 {
                None => {
                    Err::<Adt5, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt111>(v3)
                }
            }
        }
    }
}

pub fn f314(v0: Adt5) -> Result<Adt25, Adt111> {
    let m1250 = f310(Str::lit("type"), v0.clone());
    match m1250 {
        Ok(v1) => {
            let a1251 = v1.clone();
            let a1252 = Str::lit("ref/prompt");
            if equal(a1251, a1252) {
                let m1253 = f310(Str::lit("name"), v0.clone());
                match m1253 {
                    Ok(v2) => {
                        let m1254 = f309(Str::lit("title"), v0);
                        match m1254 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt111>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt111>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt111>(v5)
                    }
                }
            } else {
                let a1255 = v1;
                let a1256 = Str::lit("ref/resource");
                if equal(a1255, a1256) {
                    let m1257 = f310(Str::lit("uri"), v0);
                    match m1257 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt111>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt111>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt111>(Adt111::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt111>(v8)
        }
    }
}

pub fn f315(v0: Adt5) -> Result<Adt27, Adt111> {
    let m1258 = f310(Str::lit("name"), v0.clone());
    match m1258 {
        Ok(v1) => {
            let m1259 = f310(Str::lit("value"), v0);
            match m1259 {
                Ok(v2) => {
                    Ok::<Adt27, Adt111>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt111>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt111>(v4)
        }
    }
}

pub fn f316(v0: Adt5) -> Result<Option<Adt28>, Adt111> {
    let m1260 = f86(Str::lit("context"), v0);
    match m1260 {
        Adt114::C0 => {
            Ok::<Option<Adt28>, Adt111>(None::<Adt28>)
        }
        Adt114::C1(v1) => {
            let m1261 = f217(v1);
            match m1261 {
                None => {
                    Err::<Option<Adt28>, Adt111>(Adt111::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m1262 = f312(Str::lit("arguments"), v2);
                    match m1262 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt111>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt111>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f317(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt110> {
    if v0 {
        None::<Adt110>
    } else {
        Some(Adt110::C1(v1, v2))
    }
}

pub fn f318(v0: Str, p1267: List<Str>) -> bool {
    let m1266 = p1267;
    {
        let mut m1266 = m1266;
        loop {
            let step = {
                let v1 = m1266;
                let m1263 = v1;
                match m1263.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1264 = v2;
                        let a1265 = v0.clone();
                        if equal(a1264, a1265) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1266 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f319(v0: Option<Adt94>, v1: List<Adt96>, v2: Adt5) -> R<Option<Adt94>> {
    let m1268 = v0;
    match m1268 {
        None => {
            f334(v1, f335(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f320(v0: Option<Str>, v1: Adt125, v2: Adt5) -> Option<Adt94> {
    let m1269 = v1.clone();
    match m1269 {
        Adt125::C0 => {
            None::<Adt94>
        }
        Adt125::C1 => {
            f336(v0, f337(v1, v2))
        }
        Adt125::C2 => {
            f336(v0, f337(v1, v2))
        }
        Adt125::C3 => {
            f336(v0, f337(v1, v2))
        }
    }
}

pub fn f321(v0: Adt114, v1: Adt114) -> Adt128 {
    let m1270 = v0;
    match m1270 {
        Adt114::C0 => {
            Adt128::C1
        }
        Adt114::C1(_) => {
            let m1271 = v1;
            match m1271 {
                Adt114::C0 => {
                    Adt128::C2
                }
                Adt114::C1(_) => {
                    Adt128::C0
                }
            }
        }
    }
}

pub fn f322(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt111> {
    let m1272 = f113(v1);
    match m1272 {
        None => {
            Err::<Adt16, Adt111>(Adt111::C5)
        }
        Some(v3) => {
            let m1273 = f338(v2);
            match m1273 {
                None => {
                    Err::<Adt16, Adt111>(Adt111::C6)
                }
                Some(v4) => {
                    let m1274 = f311(f339(), v0.clone());
                    match m1274 {
                        Ok(v6) => {
                            let m1275 = f340(v0.clone());
                            match m1275 {
                                Ok(v8) => {
                                    let m1276 = f341(v0.clone());
                                    match m1276 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt111>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt111>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt111>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt111>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f323(v0: u64, v1: u64) -> u64 {
    let a1277 = v0;
    let a1278 = v1;
    let a1279 = 0u64;
    nat_rem(a1277, a1278, a1279)
}

pub fn f324(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1282 = {
            let a1280 = v0;
            let a1281 = 4u64;
            nat_mul(a1280, a1281)?
        };
        let a1283 = f326(v1, 16u64);
        nat_add(a1282, a1283)?
    }, List::<u64>::nil()))
}

pub fn f325(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1286 = {
            let a1284 = v0;
            let a1285 = 4u64;
            nat_mul(a1284, a1285)?
        };
        let a1287 = f326(v1, 16u64);
        nat_add(a1286, a1287)?
    }, List::cons({
        let a1290 = {
            let a1288 = f323(v1, 16u64);
            let a1289 = 16u64;
            nat_mul(a1288, a1289)?
        };
        let a1291 = f326(v2, 4u64);
        nat_add(a1290, a1291)?
    }, List::<u64>::nil())))
}

pub fn f326(v0: u64, v1: u64) -> u64 {
    let a1292 = v0;
    let a1293 = v1;
    let a1294 = 0u64;
    nat_quot(a1292, a1293, a1294)
}

pub fn f327(v0: i64) -> u64 {
    let m1297 = {
        let a1295 = v0;
        let a1296 = 1i64;
        compare(a1295, a1296)
    };
    if match m1297 {
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
        let m1300 = {
            let a1298 = v0;
            let a1299 = 2i64;
            compare(a1298, a1299)
        };
        if match m1300 {
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
            let m1303 = {
                let a1301 = v0;
                let a1302 = 3i64;
                compare(a1301, a1302)
            };
            if match m1303 {
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
                let m1306 = {
                    let a1304 = v0;
                    let a1305 = 4i64;
                    compare(a1304, a1305)
                };
                if match m1306 {
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
                    let m1309 = {
                        let a1307 = v0;
                        let a1308 = 5i64;
                        compare(a1307, a1308)
                    };
                    if match m1309 {
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
                        let m1312 = {
                            let a1310 = v0;
                            let a1311 = 6i64;
                            compare(a1310, a1311)
                        };
                        if match m1312 {
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
                            let m1315 = {
                                let a1313 = v0;
                                let a1314 = 7i64;
                                compare(a1313, a1314)
                            };
                            if match m1315 {
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
                                let m1318 = {
                                    let a1316 = v0;
                                    let a1317 = 8i64;
                                    compare(a1316, a1317)
                                };
                                if match m1318 {
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
                                    let m1321 = {
                                        let a1319 = v0;
                                        let a1320 = 9i64;
                                        compare(a1319, a1320)
                                    };
                                    if match m1321 {
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

pub fn f328(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m1322 = v0;
    match m1322 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a1328 = f342(match v1.clone() {
                    Adt40::C0(h1323, _, _) => {
                        h1323
                    }
                });
                let a1329 = {
                    let a1326 = f343(match v1.clone() {
                        Adt40::C0(_, h1324, _) => {
                            h1324
                        }
                    });
                    let a1327 = f229(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h1325) => {
                            h1325
                        }
                    });
                    append_list(a1326, a1327)
                };
                append_list(a1328, a1329)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f329(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h1330, _, _, _) => {
            h1330
        }
    })), {
        let a1336 = f229(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h1331, _, _) => {
                h1331
            }
        });
        let a1337 = {
            let a1334 = f344(match v0.clone() {
                Adt2::C0(_, _, h1332, _) => {
                    h1332
                }
            });
            let a1335 = f345(match v0 {
                Adt2::C0(_, _, _, h1333) => {
                    h1333
                }
            });
            append_list(a1334, a1335)
        };
        append_list(a1336, a1337)
    }))
}

pub fn f330(v0: Adt117, v1: Adt32) -> bool {
    let m1338 = v1;
    match m1338 {
        Adt32::C0(_) => {
            f346(v0, Adt117::C0)
        }
        Adt32::C1(_) => {
            f346(v0, Adt117::C1)
        }
        Adt32::C2(_) => {
            f346(v0, Adt117::C2)
        }
        Adt32::C3(_) => {
            f346(v0, Adt117::C3)
        }
        Adt32::C4(_) => {
            f346(v0, Adt117::C4)
        }
        Adt32::C5(_) => {
            f346(v0, Adt117::C5)
        }
        Adt32::C6(_) => {
            f346(v0, Adt117::C6)
        }
        Adt32::C7(_) => {
            f346(v0, Adt117::C7)
        }
        Adt32::C8(_) => {
            f346(v0, Adt117::C8)
        }
        Adt32::C9(_) => {
            f346(v0, Adt117::C9)
        }
        Adt32::C10 => {
            if f346(v0.clone(), Adt117::C10) {
                true
            } else {
                f346(v0, Adt117::C11)
            }
        }
        Adt32::C11(_) => {
            f347(v0)
        }
        Adt32::C12(_) => {
            f348(v0)
        }
    }
}

pub fn f331(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt108, Adt110> {
    let m1339 = v2.clone();
    match m1339 {
        Adt32::C0(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C3(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C4(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C10 => {
            Ok::<Adt108, Adt110>(f306(v0, v2))
        }
        Adt32::C11(v3) => {
            f349(v0, f350(v1), v3)
        }
        Adt32::C12(v4) => {
            f351(v0, f350(v1), v4)
        }
    }
}

pub fn f332() -> Adt82 {
    Adt82::C6(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f333(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt111> {
    let m1340 = v1;
    match m1340.uncons() {
        None => {
            Ok::<List<(Str, Str)>, Adt111>(List::<(Str, Str)>::nil())
        }
        Some((v2, v3)) => {
            let m1342 = f113({
                let (_, h1341) = v2;
                h1341
            });
            match m1342 {
                None => {
                    Err::<List<(Str, Str)>, Adt111>(Adt111::C8(v0))
                }
                Some(v4) => {
                    let m1343 = f333(v0, v3);
                    match m1343 {
                        Ok(v5) => {
                            Ok::<List<(Str, Str)>, Adt111>(List::cons(({
                                let (h1344, _) = v2;
                                h1344
                            }, v4), v5))
                        }
                        Err(v6) => {
                            Err::<List<(Str, Str)>, Adt111>(v6)
                        }
                    }
                }
            }
        }
    }
}

pub fn f334(p1348: List<Adt96>, v1: Adt5) -> R<Option<Adt94>> {
    let m1347 = p1348;
    {
        let mut m1347 = m1347;
        loop {
            let step = {
                let v0 = m1347;
                let m1345 = v0;
                match m1345.uncons() {
                    None => {
                        Err::<List<Adt96>, R<Option<Adt94>>>(Ok(None::<Adt94>))
                    }
                    Some((v2, v3)) => {
                        let m1346 = f352(v2, v1.clone())?;
                        match m1346 {
                            None => {
                                Ok::<List<Adt96>, R<Option<Adt94>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt96>, R<Option<Adt94>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f335(v0: Adt5) -> Adt5 {
    let m1349 = f86(Str::lit("arguments"), v0);
    match m1349 {
        Adt114::C0 => {
            f53()
        }
        Adt114::C1(v1) => {
            let m1350 = f217(v1);
            match m1350 {
                None => {
                    f53()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f336(v0: Option<Str>, v1: Option<Str>) -> Option<Adt94> {
    let m1351 = v0;
    match m1351 {
        None => {
            Some(Adt94::C2)
        }
        Some(v2) => {
            let m1352 = v1;
            match m1352 {
                None => {
                    None::<Adt94>
                }
                Some(v3) => {
                    let a1353 = v2;
                    let a1354 = v3;
                    if equal(a1353, a1354) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C5)
                    }
                }
            }
        }
    }
}

pub fn f337(v0: Adt125, v1: Adt5) -> Option<Str> {
    let m1355 = v0;
    match m1355 {
        Adt125::C0 => {
            None::<Str>
        }
        Adt125::C1 => {
            let m1356 = f86(Str::lit("name"), v1);
            match m1356 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v2) => {
                    f113(v2)
                }
            }
        }
        Adt125::C2 => {
            let m1357 = f86(Str::lit("uri"), v1);
            match m1357 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v3) => {
                    f113(v3)
                }
            }
        }
        Adt125::C3 => {
            let m1358 = f86(Str::lit("taskId"), v1);
            match m1358 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v4) => {
                    f113(v4)
                }
            }
        }
    }
}

pub fn f338(v0: Adt6) -> Option<Adt17> {
    let m1359 = f217(v0);
    match m1359 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m1360 = f353(Str::lit("experimental"), v1.clone());
            match m1360 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m1361 = f353(Str::lit("roots"), v1.clone());
                    match m1361 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m1362 = f353(Str::lit("sampling"), v1.clone());
                            match m1362 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m1363 = f353(Str::lit("elicitation"), v1.clone());
                                    match m1363 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m1364 = f353(Str::lit("extensions"), v1);
                                            match m1364 {
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

pub fn f339() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f340(v0: Adt5) -> Result<Option<Adt18>, Adt111> {
    let m1365 = f309(f354(), v0);
    match m1365 {
        Ok(v2) => {
            let m1366 = v2;
            match m1366 {
                None => {
                    Ok::<Option<Adt18>, Adt111>(None::<Adt18>)
                }
                Some(v3) => {
                    let m1367 = f355(v3);
                    match m1367 {
                        Adt129::C0 => {
                            Err::<Option<Adt18>, Adt111>(Adt111::C8(f354()))
                        }
                        Adt129::C1(v4) => {
                            Ok::<Option<Adt18>, Adt111>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt111>(v1)
        }
    }
}

pub fn f341(v0: Adt5) -> Result<Option<Adt19>, Adt111> {
    let m1368 = f86(f356(), v0);
    match m1368 {
        Adt114::C0 => {
            Ok::<Option<Adt19>, Adt111>(None::<Adt19>)
        }
        Adt114::C1(v1) => {
            let m1369 = f113(v1.clone());
            match m1369 {
                None => {
                    let m1370 = f357(v1);
                    match m1370 {
                        None => {
                            Err::<Option<Adt19>, Adt111>(Adt111::C8(f356()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt111>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt111>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f342(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m1371 = v0;
    match m1371 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f358(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f343(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1372 = v0;
    match m1372 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f344(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m1373 = v0;
    match m1373 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f52(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f345(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m1374 = v0;
    match m1374 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f359(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f346(v0: Adt117, v1: Adt117) -> bool {
    let a1375 = f242(v0);
    let a1376 = f242(v1);
    equal(a1375, a1376)
}

pub fn f347(v0: Adt117) -> bool {
    let m1377 = v0;
    match m1377 {
        Adt117::C0 => {
            false
        }
        Adt117::C1 => {
            false
        }
        Adt117::C2 => {
            true
        }
        Adt117::C3 => {
            false
        }
        Adt117::C4 => {
            true
        }
        Adt117::C5 => {
            false
        }
        Adt117::C6 => {
            false
        }
        Adt117::C7 => {
            true
        }
        Adt117::C8 => {
            false
        }
        Adt117::C9 => {
            false
        }
        Adt117::C10 => {
            false
        }
        Adt117::C11 => {
            false
        }
    }
}

pub fn f348(v0: Adt117) -> bool {
    let m1378 = v0;
    match m1378 {
        Adt117::C0 => {
            false
        }
        Adt117::C1 => {
            false
        }
        Adt117::C2 => {
            true
        }
        Adt117::C3 => {
            false
        }
        Adt117::C4 => {
            false
        }
        Adt117::C5 => {
            false
        }
        Adt117::C6 => {
            false
        }
        Adt117::C7 => {
            false
        }
        Adt117::C8 => {
            false
        }
        Adt117::C9 => {
            false
        }
        Adt117::C10 => {
            false
        }
        Adt117::C11 => {
            false
        }
    }
}

pub fn f349(v0: Adt0, v1: Adt16, v2: Adt79) -> Result<Adt108, Adt110> {
    f360(v0, v2.clone(), f361(match v1 {
        Adt16::C0(_, h1379, _, _, _, _) => {
            h1379
        }
    }, f362(v2)))
}

pub fn f350(v0: Adt14) -> Adt16 {
    let m1380 = v0;
    match m1380 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h1381) => {
                    h1381
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h1382, _) => {
                    h1382
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h1383, _, _, _, _) => {
                    h1383
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h1384, _) => {
                    h1384
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h1385, _, _, _) => {
                    h1385
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h1386, _) => {
                    h1386
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h1387, _) => {
                    h1387
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h1388, _, _, _, _) => {
                    h1388
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h1389, _, _, _) => {
                    h1389
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h1390, _) => {
                    h1390
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h1391, _, _) => {
                    h1391
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h1392, _) => {
                    h1392
                }
            }
        }
    }
}

pub fn f351(v0: Adt0, v1: Adt16, v2: Adt80) -> Result<Adt108, Adt110> {
    f363(v0, v2, f364(match v1 {
        Adt16::C0(_, h1393, _, _, _, _) => {
            h1393
        }
    }))
}

pub fn f352(v0: Adt96, v1: Adt5) -> R<Option<Adt94>> {
    let m1395 = f365(match v0.clone() {
        Adt96::C0(_, h1394, _, _) => {
            h1394
        }
    }, v1);
    match m1395 {
        Adt114::C0 => {
            Ok(f366(v0))
        }
        Adt114::C1(v2) => {
            if f367(v2.clone()) {
                Ok(f366(v0))
            } else {
                f368(v0, v2)
            }
        }
    }
}

pub fn f353(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m1396 = f86(v0, v1);
    match m1396 {
        Adt114::C0 => {
            Some(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m1397 = f217(v2);
            match m1397 {
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

pub fn f354() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f355(v0: Str) -> Adt129 {
    let a1398 = v0.clone();
    let a1399 = Str::lit("debug");
    if equal(a1398, a1399) {
        Adt129::C1(Adt18::C0)
    } else {
        let a1400 = v0.clone();
        let a1401 = Str::lit("info");
        if equal(a1400, a1401) {
            Adt129::C1(Adt18::C1)
        } else {
            let a1402 = v0.clone();
            let a1403 = Str::lit("notice");
            if equal(a1402, a1403) {
                Adt129::C1(Adt18::C2)
            } else {
                let a1404 = v0.clone();
                let a1405 = Str::lit("warning");
                if equal(a1404, a1405) {
                    Adt129::C1(Adt18::C3)
                } else {
                    let a1406 = v0.clone();
                    let a1407 = Str::lit("error");
                    if equal(a1406, a1407) {
                        Adt129::C1(Adt18::C4)
                    } else {
                        let a1408 = v0.clone();
                        let a1409 = Str::lit("critical");
                        if equal(a1408, a1409) {
                            Adt129::C1(Adt18::C5)
                        } else {
                            let a1410 = v0.clone();
                            let a1411 = Str::lit("alert");
                            if equal(a1410, a1411) {
                                Adt129::C1(Adt18::C6)
                            } else {
                                let a1412 = v0;
                                let a1413 = Str::lit("emergency");
                                if equal(a1412, a1413) {
                                    Adt129::C1(Adt18::C7)
                                } else {
                                    Adt129::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f356() -> Str {
    Str::lit("progressToken")
}

pub fn f357(v0: Adt6) -> Option<i64> {
    let m1414 = v0;
    match m1414 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f215(match v2.clone() {
                Adt7::C0(_, h1415) => {
                    h1415
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1416, _) => {
                        h1416
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

pub fn f358(v0: List<Adt41>) -> List<Adt6> {
    let m1417 = v0;
    match m1417.uncons() {
        None => {
            List::<Adt6>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt6::C3(f369(v1)), f358(v2))
        }
    }
}

pub fn f359(v0: Adt3) -> Str {
    let m1418 = v0;
    match m1418 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f360(v0: Adt0, v1: Adt79, v2: bool) -> Result<Adt108, Adt110> {
    if v2 {
        Ok::<Adt108, Adt110>(f306(v0, Adt32::C11(v1)))
    } else {
        Err::<Adt108, Adt110>(Adt110::C4(Adt82::C5(f370(f362(v1)))))
    }
}

pub fn f361(v0: Adt17, v1: List<(Str, Adt62)>) -> bool {
    let a1419 = f371(v1.clone());
    if if if bool_not(a1419) {
        true
    } else {
        f245(match v0.clone() {
            Adt17::C0(_, _, _, h1420, _) => {
                h1420
            }
        })
    } {
        let a1421 = f372(v1.clone());
        if bool_not(a1421) {
            true
        } else {
            f245(match v0.clone() {
                Adt17::C0(_, _, h1422, _, _) => {
                    h1422
                }
            })
        }
    } else {
        false
    } {
        let a1423 = f373(v1);
        if bool_not(a1423) {
            true
        } else {
            f245(match v0 {
                Adt17::C0(_, h1424, _, _, _) => {
                    h1424
                }
            })
        }
    } else {
        false
    }
}

pub fn f362(v0: Adt79) -> List<(Str, Adt62)> {
    let m1425 = v0;
    match m1425 {
        Adt79::C0(v1) => {
            v1
        }
        Adt79::C1(_) => {
            List::<(Str, Adt62)>::nil()
        }
        Adt79::C2(v3, _) => {
            v3
        }
    }
}

pub fn f363(v0: Adt0, v1: Adt80, v2: bool) -> Result<Adt108, Adt110> {
    if v2 {
        Ok::<Adt108, Adt110>(f306(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt108, Adt110>(Adt110::C4(Adt82::C5(f374())))
    }
}

pub fn f364(v0: Adt17) -> bool {
    f246(match v0 {
        Adt17::C0(_, _, _, _, h1426) => {
            h1426
        }
    }, f247())
}

pub fn f365(p1432: List<Str>, p1433: Adt5) -> Adt114 {
    let m1431 = (p1432, p1433);
    {
        let mut m1431 = m1431;
        loop {
            let step = {
                let (v0, v1) = m1431;
                let m1427 = v0;
                match m1427.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m1428 = f86(v2, v1);
                        match m1428 {
                            Adt114::C0 => {
                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                            }
                            Adt114::C1(v4) => {
                                let m1429 = v3.clone();
                                match m1429.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m1430 = f217(v4);
                                        match m1430 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt114>((v3, v7))
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
                Ok(next) => { m1431 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Adt96) -> Option<Adt94> {
    let m1435 = match v0.clone() {
        Adt96::C0(_, _, _, h1434) => {
            h1434
        }
    };
    match m1435 {
        Adt97::C0 => {
            None::<Adt94>
        }
        Adt97::C1 => {
            Some(Adt94::C10(match v0 {
                Adt96::C0(h1436, _, _, _) => {
                    h1436
                }
            }))
        }
        Adt97::C2(_) => {
            None::<Adt94>
        }
    }
}

pub fn f367(v0: Adt6) -> bool {
    let m1437 = v0;
    match m1437 {
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

pub fn f368(v0: Adt96, v1: Adt6) -> R<Option<Adt94>> {
    let m1439 = match v0.clone() {
        Adt96::C0(_, _, _, h1438) => {
            h1438
        }
    };
    match m1439 {
        Adt97::C0 => {
            Ok(Some(Adt94::C8(match v0 {
                Adt96::C0(h1440, _, _, _) => {
                    h1440
                }
            })))
        }
        Adt97::C1 => {
            Ok(Some(Adt94::C10(match v0 {
                Adt96::C0(h1441, _, _, _) => {
                    h1441
                }
            })))
        }
        Adt97::C2(v2) => {
            if f375(match v0.clone() {
                Adt96::C0(_, _, h1442, _) => {
                    h1442
                }
            }, v2, v1)? {
                Ok(None::<Adt94>)
            } else {
                Ok(Some(Adt94::C9(match v0 {
                    Adt96::C0(h1443, _, _, _) => {
                        h1443
                    }
                })))
            }
        }
    }
}

pub fn f369(v0: Adt41) -> Str {
    let m1444 = v0;
    match m1444 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f370(v0: List<(Str, Adt62)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f373(v0.clone()) {
        Some(f53())
    } else {
        None::<Adt5>
    }, if f372(v0.clone()) {
        Some(f53())
    } else {
        None::<Adt5>
    }, if f371(v0) {
        Some(f53())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f371(p1449: List<(Str, Adt62)>) -> bool {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1445 = v0;
                match m1445.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1447 = {
                            let (_, h1446) = v1;
                            h1446
                        };
                        match m1447 {
                            Adt62::C0(_) => {
                                Err::<List<(Str, Adt62)>, bool>(true)
                            }
                            Adt62::C1(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
                            }
                            Adt62::C2(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
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

pub fn f372(p1454: List<(Str, Adt62)>) -> bool {
    let m1453 = p1454;
    {
        let mut m1453 = m1453;
        loop {
            let step = {
                let v0 = m1453;
                let m1450 = v0;
                match m1450.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1452 = {
                            let (_, h1451) = v1;
                            h1451
                        };
                        match m1452 {
                            Adt62::C0(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
                            }
                            Adt62::C1(_) => {
                                Err::<List<(Str, Adt62)>, bool>(true)
                            }
                            Adt62::C2(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(p1459: List<(Str, Adt62)>) -> bool {
    let m1458 = p1459;
    {
        let mut m1458 = m1458;
        loop {
            let step = {
                let v0 = m1458;
                let m1455 = v0;
                match m1455.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1457 = {
                            let (_, h1456) = v1;
                            h1456
                        };
                        match m1457 {
                            Adt62::C0(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
                            }
                            Adt62::C1(_) => {
                                Ok::<List<(Str, Adt62)>, bool>(v2)
                            }
                            Adt62::C2(_) => {
                                Err::<List<(Str, Adt62)>, bool>(true)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f247(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f375(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m1460 = v0;
    match m1460 {
        Adt13::C0 => {
            let m1461 = f113(v2);
            match m1461 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a1462 = v1;
                    let a1463 = v3;
                    Ok(equal(a1462, a1463))
                }
            }
        }
        Adt13::C1 => {
            f376(v1, v2)
        }
        Adt13::C2 => {
            let m1464 = f377(v2);
            match m1464 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a1465 = v1;
                        let a1466 = Str::lit("true");
                        Ok(equal(a1465, a1466))
                    } else {
                        let a1467 = v1;
                        let a1468 = Str::lit("false");
                        Ok(equal(a1467, a1468))
                    }
                }
            }
        }
    }
}

pub fn f376(v0: Str, v1: Adt6) -> R<bool> {
    let m1469 = f378(v1);
    match m1469 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1470 = f115(v2)?;
            match m1470 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1472 = f379(f48({
                        let a1471 = v0;
                        utf8_encode(a1471)
                    })?)?;
                    match m1472 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f215({
                                let a1473 = v3;
                                let a1474 = v4;
                                int_sub(a1473, a1474)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: Adt6) -> Option<bool> {
    let m1475 = v0;
    match m1475 {
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

pub fn f378(v0: Adt6) -> Option<Adt7> {
    let m1476 = v0;
    match m1476 {
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

pub fn f379(v0: List<u64>) -> R<Option<i64>> {
    let m1477 = v0.clone();
    match m1477.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a1478 = v1;
            let a1479 = 45u64;
            if nat_eq(a1478, a1479) {
                let m1480 = v2.clone();
                match m1480.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m1481 = f380(v2, 0i64)?;
                        match m1481 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a1482 = v5;
                                    int_neg(a1482)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f380(v0, 0i64)
            }
        }
    }
}

pub fn f380(p1488: List<u64>, p1489: i64) -> R<Option<i64>> {
    let m1487 = (p1488, p1489);
    {
        let mut m1487 = m1487;
        loop {
            let step = {
                let (v0, v1) = m1487;
                let m1483 = v0;
                match m1483.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1484 = f381(v2);
                        match m1484 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f216({
                                    let a1485 = v1;
                                    let a1486 = 10i64;
                                    int_mul(a1485, a1486)?
                                }, f161(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381(v0: u64) -> Option<u64> {
    let a1490 = v0;
    let a1491 = 48u64;
    if if nat_lt(a1490, a1491) {
        true
    } else {
        let a1492 = 57u64;
        let a1493 = v0;
        nat_lt(a1492, a1493)
    } {
        None::<u64>
    } else {
        Some({
            let a1494 = v0;
            let a1495 = 48u64;
            nat_sub(a1494, a1495)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
