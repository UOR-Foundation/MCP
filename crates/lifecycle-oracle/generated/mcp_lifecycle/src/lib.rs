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
    f33(f34(v0, List::<Adt84>::nil()), List::<u64>::nil())
}

pub fn f18() -> u64 {
    500u64
}

pub fn f19() -> List<Adt84> {
    List::cons(Adt84::C0(f35(), f36()), List::<Adt84>::nil())
}

pub fn f20() -> List<u64> {
    let a76 = f37();
    let a77 = {
        let a74 = f38();
        let a75 = f39();
        append_list(a74, a75)
    };
    append_list(a76, a77)
}

pub fn f21(v0: List<u64>) -> bool {
    f40(v0, f41())
}

pub fn f22(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Result<Adt6, Adt88>) -> R<Adt86> {
    let m78 = v3;
    match m78 {
        Ok(v5) => {
            f46(v0, v1, v2, f47(v5)?)
        }
        Err(_) => {
            f42(f43(), f44(None::<Adt89>, f45(Adt90::C0)))
        }
    }
}

pub fn f23(v0: Bytes) -> R<Result<Adt6, Adt88>> {
    f48(f49(v0)?)
}

pub fn f24() -> Adt86 {
    Adt86::C0(f50(), List::cons(Adt84::C0(f51(), f52()), List::<Adt84>::nil()), List::<u64>::nil())
}

pub fn f25(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f26(v0: List<(Str, Adt6)>, v1: List<Str>) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(v0)), List::cons((Str::lit("required"), Adt6::C4(f53(v1))), List::<(Str, Adt6)>::nil()))))
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
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f54()), None::<Adt5>, None::<Adt5>)
}

pub fn f31(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a82 = {
        let a79 = v1;
        let a80 = 4u64;
        nat_add(a79, a80)?
    };
    let a83 = {
        let a81 = v0.clone();
        length_bytes(a81)
    };
    if nat_le(a82, a83) {
        Ok(Some({
            let a100 = {
                let a84 = f55(v0.clone(), v1)?;
                let a85 = 16777216u64;
                nat_mul(a84, a85)?
            };
            let a101 = {
                let a98 = {
                    let a88 = f55(v0.clone(), {
                        let a86 = v1;
                        let a87 = 1u64;
                        nat_add(a86, a87)?
                    })?;
                    let a89 = 65536u64;
                    nat_mul(a88, a89)?
                };
                let a99 = {
                    let a96 = {
                        let a92 = f55(v0.clone(), {
                            let a90 = v1;
                            let a91 = 2u64;
                            nat_add(a90, a91)?
                        })?;
                        let a93 = 256u64;
                        nat_mul(a92, a93)?
                    };
                    let a97 = f55(v0, {
                        let a94 = v1;
                        let a95 = 3u64;
                        nat_add(a94, a95)?
                    })?;
                    nat_add(a96, a97)?
                };
                nat_add(a98, a99)?
            };
            nat_add(a100, a101)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f32(v0: Bytes, v1: Adt87, v2: Adt87, v3: Option<u64>) -> R<Result<Adt83, Adt85>> {
    let m102 = v3;
    match m102 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0(match v2 {
                Adt87::C0(_, h103) => {
                    h103
                }
            })))
        }
        Some(v4) => {
            let m107 = f56(v0.clone(), {
                let a105 = match v2.clone() {
                    Adt87::C0(_, h104) => {
                        h104
                    }
                };
                let a106 = 4u64;
                nat_add(a105, a106)?
            }, v4, List::<Adt84>::nil())?;
            match m107 {
                Ok(v6) => {
                    f57(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt83, Adt85>(v5))
                }
            }
        }
    }
}

pub fn f33(p112: List<Adt84>, p113: List<u64>) -> List<u64> {
    let m111 = (p112, p113);
    {
        let mut m111 = m111;
        loop {
            let step = {
                let (v0, v1) = m111;
                let m108 = v0;
                match m108.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt84>, List<u64>), List<u64>>((v3, {
                            let a109 = f58(v2);
                            let a110 = v1;
                            append_list(a109, a110)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f34(p116: List<Adt84>, p117: List<Adt84>) -> List<Adt84> {
    let m115 = (p116, p117);
    {
        let mut m115 = m115;
        loop {
            let step = {
                let (v0, v1) = m115;
                let m114 = v0;
                match m114.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt84>), List<Adt84>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt84>, List<Adt84>), List<Adt84>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f36() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f37() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f38() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f39() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f40(p124: List<u64>, p125: List<u64>) -> bool {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v0, v1) = m123;
                let m118 = v0;
                match m118.uncons() {
                    None => {
                        let m119 = v1;
                        match m119.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m120 = v1;
                        match m120.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a121 = v4;
                                let a122 = v6;
                                if nat_eq(a121, a122) {
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
                Ok(next) => { m123 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f41() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f42(v0: u64, v1: Adt6) -> R<Adt86> {
    let m126 = f59(v1)?;
    match m126 {
        Ok(v3) => {
            Ok(Adt86::C0(v0, f19(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f43() -> u64 {
    400u64
}

pub fn f44(v0: Option<Adt89>, v1: Adt78) -> Adt6 {
    Adt6::C5(f60(v0, v1))
}

pub fn f45(v0: Adt90) -> Adt78 {
    Adt78::C0(f61(v0.clone()), f62(v0), None::<Adt6>)
}

pub fn f46(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Result<Adt92, Adt90>) -> R<Adt86> {
    let m127 = v3;
    match m127 {
        Ok(v5) => {
            f65(v0, v1, v2, v5)
        }
        Err(v4) => {
            f42(f63(v4.clone()), f44(f64(v4.clone()), f45(v4)))
        }
    }
}

pub fn f47(v0: Adt6) -> R<Result<Adt92, Adt90>> {
    let m128 = v0;
    match m128 {
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

pub fn f48(v0: List<u64>) -> R<Result<Adt6, Adt88>> {
    let m129 = f68(v0.clone())?;
    match m129 {
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

pub fn f49(v0: Bytes) -> R<List<u64>> {
    f71(v0.clone(), {
        let a130 = v0;
        length_bytes(a130)
    }, List::<u64>::nil())
}

pub fn f50() -> u64 {
    405u64
}

pub fn f51() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f52() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f53(v0: List<Str>) -> List<Adt6> {
    f72(f73(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f54() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f55(v0: Bytes, v1: u64) -> R<u64> {
    let m133 = {
        let a131 = v0;
        let a132 = v1;
        index_bytes(a131, a132)
    };
    match m133 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a178 = {
                let a154 = {
                    let a142 = {
                        let a136 = {
                            let a134 = v3;
                            let a135 = 128u8;
                            fixed_u8::bit_and(a134, a135)
                        };
                        let a137 = 128u8;
                        if equal(a136, a137) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a143 = {
                        let a140 = {
                            let a138 = v3;
                            let a139 = 64u8;
                            fixed_u8::bit_and(a138, a139)
                        };
                        let a141 = 64u8;
                        if equal(a140, a141) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a142, a143)?
                };
                let a155 = {
                    let a152 = {
                        let a146 = {
                            let a144 = v3;
                            let a145 = 32u8;
                            fixed_u8::bit_and(a144, a145)
                        };
                        let a147 = 32u8;
                        if equal(a146, a147) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a153 = {
                        let a150 = {
                            let a148 = v3;
                            let a149 = 16u8;
                            fixed_u8::bit_and(a148, a149)
                        };
                        let a151 = 16u8;
                        if equal(a150, a151) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a152, a153)?
                };
                nat_add(a154, a155)?
            };
            let a179 = {
                let a176 = {
                    let a164 = {
                        let a158 = {
                            let a156 = v3;
                            let a157 = 8u8;
                            fixed_u8::bit_and(a156, a157)
                        };
                        let a159 = 8u8;
                        if equal(a158, a159) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a165 = {
                        let a162 = {
                            let a160 = v3;
                            let a161 = 4u8;
                            fixed_u8::bit_and(a160, a161)
                        };
                        let a163 = 4u8;
                        if equal(a162, a163) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a164, a165)?
                };
                let a177 = {
                    let a174 = {
                        let a168 = {
                            let a166 = v3;
                            let a167 = 2u8;
                            fixed_u8::bit_and(a166, a167)
                        };
                        let a169 = 2u8;
                        if equal(a168, a169) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a175 = {
                        let a172 = {
                            let a170 = v3;
                            let a171 = 1u8;
                            fixed_u8::bit_and(a170, a171)
                        };
                        let a173 = 1u8;
                        if equal(a172, a173) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a174, a175)?
                };
                nat_add(a176, a177)?
            };
            nat_add(a178, a179)
        }
    }
}

pub fn f56(v0: Bytes, p189: u64, p190: u64, p191: List<Adt84>) -> R<Result<(List<Adt84>, u64), Adt85>> {
    let m188 = (p189, (p190, p191));
    {
        let mut m188 = m188;
        loop {
            let step = {
                let (v1, (v2, v3)) = m188;
                let m180 = v2;
                if m180 == 0 {
                    Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Ok::<(List<Adt84>, u64), Adt85>((f74(v3, List::<Adt84>::nil()), v1))))
                } else {
                    let v4 = m180 - 1;
                    let m181 = f14(v0.clone(), v1)?;
                    match m181 {
                        None => {
                            Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Err::<(List<Adt84>, u64), Adt85>(Adt85::C0(v1))))
                        }
                        Some(v5) => {
                            let m183 = f14(v0.clone(), match v5.clone() {
                                Adt87::C0(_, h182) => {
                                    h182
                                }
                            })?;
                            match m183 {
                                None => {
                                    Err::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>(Ok(Err::<(List<Adt84>, u64), Adt85>(Adt85::C0(match v5 {
                                        Adt87::C0(_, h184) => {
                                            h184
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt84>)), R<Result<(List<Adt84>, u64), Adt85>>>((match v6.clone() {
                                        Adt87::C0(_, h185) => {
                                            h185
                                        }
                                    }, (v4, List::cons(Adt84::C0(f49(match v5 {
                                        Adt87::C0(h186, _) => {
                                            h186
                                        }
                                    })?, f49(match v6 {
                                        Adt87::C0(h187, _) => {
                                            h187
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: Bytes, v1: Adt87, v2: Adt87, v3: (List<Adt84>, u64)) -> R<Result<Adt83, Adt85>> {
    let m193 = f14(v0.clone(), {
        let (_, h192) = v3.clone();
        h192
    })?;
    match m193 {
        None => {
            Ok(Err::<Adt83, Adt85>(Adt85::C0({
                let (_, h194) = v3;
                h194
            })))
        }
        Some(v4) => {
            let a197 = match v4.clone() {
                Adt87::C0(_, h195) => {
                    h195
                }
            };
            let a198 = {
                let a196 = v0;
                length_bytes(a196)
            };
            if nat_eq(a197, a198) {
                Ok(Ok::<Adt83, Adt85>(Adt83::C0(f49(match v1 {
                    Adt87::C0(h199, _) => {
                        h199
                    }
                })?, f49(match v2 {
                    Adt87::C0(h200, _) => {
                        h200
                    }
                })?, {
                    let (h201, _) = v3;
                    h201
                }, match v4 {
                    Adt87::C0(h202, _) => {
                        h202
                    }
                })))
            } else {
                Ok(Err::<Adt83, Adt85>(Adt85::C1(match v4 {
                    Adt87::C0(_, h203) => {
                        h203
                    }
                })))
            }
        }
    }
}

pub fn f58(v0: Adt84) -> List<u64> {
    let a214 = f16({
        let a205 = match v0.clone() {
            Adt84::C0(h204, _) => {
                h204
            }
        };
        length_list(a205)
    });
    let a215 = {
        let a212 = match v0.clone() {
            Adt84::C0(h206, _) => {
                h206
            }
        };
        let a213 = {
            let a210 = f16({
                let a208 = match v0.clone() {
                    Adt84::C0(_, h207) => {
                        h207
                    }
                };
                length_list(a208)
            });
            let a211 = match v0 {
                Adt84::C0(_, h209) => {
                    h209
                }
            };
            append_list(a210, a211)
        };
        append_list(a212, a213)
    };
    append_list(a214, a215)
}

pub fn f59(v0: Adt6) -> R<Result<List<u64>, Adt88>> {
    f75(f76(), v0)
}

pub fn f60(v0: Option<Adt89>, v1: Adt78) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f77())), {
        let a216 = f78(v0);
        let a217 = List::cons((Str::lit("error"), f79(v1)), List::<(Str, Adt6)>::nil());
        append_list(a216, a217)
    })
}

pub fn f61(v0: Adt90) -> i64 {
    let m218 = v0;
    match m218 {
        Adt90::C0 => {
            f80()
        }
        Adt90::C1(_, _) => {
            f81()
        }
    }
}

pub fn f62(v0: Adt90) -> Str {
    let m219 = v0;
    match m219 {
        Adt90::C0 => {
            Str::lit("Parse error")
        }
        Adt90::C1(_, v2) => {
            f82(v2)
        }
    }
}

pub fn f63(v0: Adt90) -> u64 {
    let m220 = v0;
    match m220 {
        Adt90::C0 => {
            f43()
        }
        Adt90::C1(_, _) => {
            f43()
        }
    }
}

pub fn f64(v0: Adt90) -> Option<Adt89> {
    let m221 = v0;
    match m221 {
        Adt90::C0 => {
            None::<Adt89>
        }
        Adt90::C1(v1, _) => {
            v1
        }
    }
}

pub fn f65(v0: Adt0, v1: Fn0, v2: List<Adt84>, v3: Adt92) -> R<Adt86> {
    f83(f84(v0.clone(), v1, Some(f85(v2, match v0 {
        Adt0::C0(_, _, _, _, h222) => {
            h222
        }
    }, f86(v3.clone()), match v3.clone() {
        Adt92::C0(_, h223, _) => {
            h223
        }
    })?), f87(), v3)?)
}

pub fn f66(v0: Adt5) -> R<Result<Adt92, Adt90>> {
    let m224 = f88(v0.clone())?;
    match m224 {
        Adt113::C0 => {
            let m225 = f89(Str::lit("method"), v0.clone());
            match m225 {
                Adt114::C0 => {
                    Ok(f67(None::<Adt89>, f90(v0)))
                }
                Adt114::C1(_) => {
                    Ok(f67(None::<Adt89>, if f91(v0) {
                        Adt91::C8
                    } else {
                        Adt91::C7
                    }))
                }
            }
        }
        Adt113::C1(v2) => {
            Ok(f92(v2, v0))
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
    Ok(f93({
        let a226 = v0.clone();
        length_list(a226)
    }, f94(Adt99::C0, 0u64, 0u64, v0)?))
}

pub fn f69(p233: Adt100, p234: u64, p235: List<u64>) -> R<Result<Adt6, Adt88>> {
    let m232 = (p233, (p234, p235));
    {
        let mut m232 = m232;
        loop {
            let step = {
                let (v0, (v1, v2)) = m232;
                let m227 = v2;
                match m227.uncons() {
                    None => {
                        let m228 = f95(v0, v1)?;
                        match m228 {
                            Ok(v3) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Ok::<Adt6, Adt88>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Err::<Adt6, Adt88>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m229 = f96(v0, v1, v5)?;
                        match m229 {
                            Adt115::C0(v7) => {
                                Err::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>(Ok(Err::<Adt6, Adt88>(v7)))
                            }
                            Adt115::C1(v8) => {
                                Ok::<(Adt100, (u64, List<u64>)), R<Result<Adt6, Adt88>>>((v8, ({
                                    let a230 = v1;
                                    let a231 = 1u64;
                                    nat_add(a230, a231)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m232 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70() -> Adt100 {
    Adt100::C0(Adt101::C0, List::<Adt105>::nil(), 0u64)
}

pub fn f71(v0: Bytes, p238: u64, p239: List<u64>) -> R<List<u64>> {
    let m237 = (p238, p239);
    {
        let mut m237 = m237;
        loop {
            let step = {
                let (v1, v2) = m237;
                let m236 = v1;
                if m236 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m236 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f55(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(p242: List<Adt6>, p243: List<Adt6>) -> List<Adt6> {
    let m241 = (p242, p243);
    {
        let mut m241 = m241;
        loop {
            let step = {
                let (v0, v1) = m241;
                let m240 = v0;
                match m240.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(p246: List<Str>, p247: List<Adt6>) -> List<Adt6> {
    let m245 = (p246, p247);
    {
        let mut m245 = m245;
        loop {
            let step = {
                let (v0, v1) = m245;
                let m244 = v0;
                match m244.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f74(p250: List<Adt84>, p251: List<Adt84>) -> List<Adt84> {
    let m249 = (p250, p251);
    {
        let mut m249 = m249;
        loop {
            let step = {
                let (v0, v1) = m249;
                let m248 = v0;
                match m248.uncons() {
                    None => {
                        Err::<(List<Adt84>, List<Adt84>), List<Adt84>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt84>, List<Adt84>), List<Adt84>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt88>> {
    let m252 = f97(v0, List::cons(Adt106::C1(0u64, v1), List::<Adt106>::nil()), List::<u64>::nil())?;
    match m252 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt88>(f98(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt88>(v3))
        }
    }
}

pub fn f76() -> u64 {
    67108864u64
}

pub fn f77() -> Str {
    Str::lit("2.0")
}

pub fn f78(v0: Option<Adt89>) -> List<(Str, Adt6)> {
    let m253 = v0;
    match m253 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f99(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f79(v0: Adt78) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt78::C0(h254, _, _) => {
            h254
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt78::C0(_, h255, _) => {
            h255
        }
    })), f100(match v0 {
        Adt78::C0(_, _, h256) => {
            h256
        }
    }))))
}

pub fn f80() -> i64 {
    -32700i64
}

pub fn f81() -> i64 {
    -32600i64
}

pub fn f82(v0: Adt91) -> Str {
    let m257 = v0;
    match m257 {
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

pub fn f83(v0: Adt107) -> R<Adt86> {
    let m259 = match v0.clone() {
        Adt107::C0(_, h258) => {
            h258
        }
    };
    match m259 {
        Ok(v1) => {
            let m260 = f101(v1)?;
            match m260 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    f42(f102(), f103(match v0 {
                        Adt107::C0(h261, _) => {
                            h261
                        }
                    }, v2))
                }
            }
        }
        Err(v3) => {
            f42(f104(v3.clone()), f44(Some(match v0 {
                Adt107::C0(h262, _) => {
                    h262
                }
            }), f105(v3)))
        }
    }
}

pub fn f84(v0: Adt0, v1: Fn0, v2: Option<Adt93>, v3: List<Str>, v4: Adt92) -> R<Adt107> {
    Ok(Adt107::C0(match v4.clone() {
        Adt92::C0(h263, _, _) => {
            h263
        }
    }, f106(v0, v1, f107(v2, v3, v4)?)))
}

pub fn f85(v0: List<Adt84>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt93> {
    let v4: Adt112 = f108(Adt95::C0, f109(v0.clone(), f110())?);
    let v5: Adt112 = f108(Adt95::C1, f109(v0.clone(), f111())?);
    let v6: Adt112 = if f112(v3) {
        f108(Adt95::C2, f113(f109(v0.clone(), f114())?)?)
    } else {
        f115()
    };
    Ok(Adt93::C0(match v4.clone() {
        Adt112::C0(h264, _) => {
            h264
        }
    }, match v5.clone() {
        Adt112::C0(h265, _) => {
            h265
        }
    }, match v6.clone() {
        Adt112::C0(h266, _) => {
            h266
        }
    }, f116(match v4 {
        Adt112::C0(_, h267) => {
            h267
        }
    }, f116(match v5 {
        Adt112::C0(_, h268) => {
            h268
        }
    }, match v6 {
        Adt112::C0(_, h269) => {
            h269
        }
    })), f117(v0, v1, v2)?))
}

pub fn f86(v0: Adt92) -> Option<Str> {
    let a271 = match v0.clone() {
        Adt92::C0(_, h270, _) => {
            h270
        }
    };
    let a272 = Str::lit("tools/call");
    if equal(a271, a272) {
        let m274 = match v0 {
            Adt92::C0(_, _, h273) => {
                h273
            }
        };
        match m274 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m275 = f89(Str::lit("name"), v1);
                match m275 {
                    Adt114::C0 => {
                        None::<Str>
                    }
                    Adt114::C1(v2) => {
                        f118(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f87() -> List<Str> {
    List::cons(f119(), List::<Str>::nil())
}

pub fn f88(v0: Adt5) -> R<Adt113> {
    let m276 = f89(Str::lit("id"), v0);
    match m276 {
        Adt114::C0 => {
            Ok(Adt113::C0)
        }
        Adt114::C1(v1) => {
            let m277 = v1;
            match m277 {
                Adt6::C0 => {
                    Ok(Adt113::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt113::C2)
                }
                Adt6::C2(v3) => {
                    let m278 = f120(v3)?;
                    match m278 {
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

pub fn f89(v0: Str, v1: Adt5) -> Adt114 {
    f121(v0, match v1 {
        Adt5::C0(h279) => {
            h279
        }
    })
}

pub fn f90(v0: Adt5) -> Adt91 {
    if f91(v0) {
        Adt91::C8
    } else {
        Adt91::C3
    }
}

pub fn f91(v0: Adt5) -> bool {
    if f122(Str::lit("result"), v0.clone()) {
        true
    } else {
        f122(Str::lit("error"), v0)
    }
}

pub fn f92(v0: Adt89, v1: Adt5) -> Result<Adt92, Adt90> {
    let m280 = f89(Str::lit("jsonrpc"), v1.clone());
    match m280 {
        Adt114::C0 => {
            f67(Some(v0), Adt91::C2)
        }
        Adt114::C1(v2) => {
            let m281 = f118(v2);
            match m281 {
                None => {
                    f67(Some(v0), Adt91::C2)
                }
                Some(v3) => {
                    let a282 = v3;
                    let a283 = f77();
                    if equal(a282, a283) {
                        f123(v0, v1)
                    } else {
                        f67(Some(v0), Adt91::C2)
                    }
                }
            }
        }
    }
}

pub fn f93(v0: u64, v1: u64) -> Adt98 {
    let a284 = v1;
    let a285 = 0u64;
    if nat_eq(a284, a285) {
        Adt98::C0
    } else {
        let a286 = v1;
        let a287 = 2u64;
        if nat_eq(a286, a287) {
            Adt98::C2(v0)
        } else {
            Adt98::C1({
                let a291 = v0;
                let a292 = {
                    let a288 = v1;
                    let a289 = 3u64;
                    let a290 = 0u64;
                    nat_quot(a288, a289, a290)
                };
                nat_sub(a291, a292)
            })
        }
    }
}

pub fn f94(p320: Adt99, p321: u64, p322: u64, p323: List<u64>) -> R<u64> {
    let m319 = (p320, (p321, (p322, p323)));
    {
        let mut m319 = m319;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m319;
                let m293 = v3;
                match m293.uncons() {
                    None => {
                        let m294 = v0;
                        match m294 {
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
                        let m295 = v0;
                        match m295 {
                            Adt99::C0 => {
                                let a296 = v4;
                                let a297 = 128u64;
                                if nat_lt(a296, a297) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a298 = v4;
                                    let a299 = 194u64;
                                    if nat_lt(a298, a299) {
                                        let v6: u64 = f124({
                                            let a300 = List::cons(v4, v5);
                                            length_list(a300)
                                        })?;
                                        Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a301 = v4;
                                        let a302 = 224u64;
                                        if nat_lt(a301, a302) {
                                            Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a303 = v4;
                                            let a304 = 224u64;
                                            if nat_eq(a303, a304) {
                                                Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a305 = v4;
                                                let a306 = 237u64;
                                                if nat_eq(a305, a306) {
                                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a307 = v4;
                                                    let a308 = 240u64;
                                                    if nat_lt(a307, a308) {
                                                        Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a309 = v4;
                                                        let a310 = 240u64;
                                                        if nat_eq(a309, a310) {
                                                            Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a311 = v4;
                                                            let a312 = 244u64;
                                                            if nat_eq(a311, a312) {
                                                                Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a313 = v4;
                                                                let a314 = 244u64;
                                                                if nat_lt(a313, a314) {
                                                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f124({
                                                                        let a315 = List::cons(v4, v5);
                                                                        length_list(a315)
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
                                if f125(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f124({
                                        let a316 = List::cons(v4, v5);
                                        length_list(a316)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt99::C2 => {
                                if f125(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f124({
                                        let a317 = List::cons(v4, v5);
                                        length_list(a317)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt99::C3 => {
                                if f125(v4, v1, v2) {
                                    Ok::<(Adt99, (u64, (u64, List<u64>))), R<u64>>((Adt99::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f124({
                                        let a318 = List::cons(v4, v5);
                                        length_list(a318)
                                    })?;
                                    Err::<(Adt99, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
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

pub fn f95(v0: Adt100, v1: u64) -> R<Result<Adt6, Adt88>> {
    let m325 = match v0.clone() {
        Adt100::C0(h324, _, _) => {
            h324
        }
    };
    match m325 {
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
            let m326 = f126(v3)?;
            match m326 {
                Adt120::C0 => {
                    Ok(Err::<Adt6, Adt88>(Adt88::C7(v1)))
                }
                Adt120::C1 => {
                    Ok(Err::<Adt6, Adt88>(Adt88::C8(v1)))
                }
                Adt120::C2(v4) => {
                    let m331 = {
                        let m330 = f127(v4, match v0.clone() {
                            Adt100::C0(_, h328, _) => {
                                h328
                            }
                        }, match v0 {
                            Adt100::C0(_, _, h329) => {
                                h329
                            }
                        });
                        match m330 {
                            Adt100::C0(h327, _, _) => {
                                h327
                            }
                        }
                    };
                    match m331 {
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

pub fn f96(v0: Adt100, v1: u64, v2: u64) -> R<Adt115> {
    let m333 = match v0.clone() {
        Adt100::C0(h332, _, _) => {
            h332
        }
    };
    match m333 {
        Adt101::C0 => {
            if f128(v2) {
                Ok(Adt115::C1(v0))
            } else {
                f129(v0, v1, v2)
            }
        }
        Adt101::C1 => {
            if f128(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a334 = v2;
                let a335 = 93u64;
                if nat_eq(a334, a335) {
                    Ok(f130(v0, v1))
                } else {
                    f129(v0, v1, v2)
                }
            }
        }
        Adt101::C2 => {
            Ok(f131(v0, v1, v2))
        }
        Adt101::C3 => {
            if f128(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a336 = v2;
                let a337 = 125u64;
                if nat_eq(a336, a337) {
                    Ok(f132(v0, v1))
                } else {
                    let a338 = v2;
                    let a339 = 34u64;
                    if nat_eq(a338, a339) {
                        Ok(f133(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f134(Adt88::C2(v1)))
                    }
                }
            }
        }
        Adt101::C4 => {
            if f128(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a340 = v2;
                let a341 = 34u64;
                if nat_eq(a340, a341) {
                    Ok(f133(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f134(Adt88::C2(v1)))
                }
            }
        }
        Adt101::C5 => {
            if f128(v2) {
                Ok(Adt115::C1(v0))
            } else {
                let a342 = v2;
                let a343 = 58u64;
                if nat_eq(a342, a343) {
                    Ok(f133(v0, Adt101::C0))
                } else {
                    Ok(f134(Adt88::C2(v1)))
                }
            }
        }
        Adt101::C6(v3, v4, v5) => {
            f135(v0, v1, v2, v3, v4, v5)
        }
        Adt101::C7(v6, v7) => {
            f136(v0, v1, v2, v6, v7)
        }
        Adt101::C8(v8) => {
            f137(v0, v1, v2, v8)
        }
        Adt101::C9(_) => {
            Ok(f131(v0, v1, v2))
        }
    }
}

pub fn f97(p348: u64, p349: List<Adt106>, p350: List<u64>) -> R<Result<List<u64>, Adt88>> {
    let m347 = (p348, (p349, p350));
    {
        let mut m347 = m347;
        loop {
            let step = {
                let (v0, (v1, v2)) = m347;
                let m344 = v0;
                if m344 == 0 {
                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>(Ok(Err::<List<u64>, Adt88>(Adt88::C13)))
                } else {
                    let v3 = m344 - 1;
                    let m345 = v1;
                    match m345.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt88>>>(Ok(Ok::<List<u64>, Adt88>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m346 = f138(v4, v5, v2)?;
                            match m346 {
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
                Ok(next) => { m347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(p353: List<u64>, p354: List<u64>) -> List<u64> {
    let m352 = (p353, p354);
    {
        let mut m352 = m352;
        loop {
            let step = {
                let (v0, v1) = m352;
                let m351 = v0;
                match m351.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m352 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Adt89) -> Adt6 {
    let m355 = v0;
    match m355 {
        Adt89::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt89::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f100(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m356 = v0;
    match m356 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f101(v0: Adt108) -> R<Option<Adt5>> {
    let m358 = f139(match v0.clone() {
        Adt108::C0(h357, _) => {
            h357
        }
    })?;
    match m358 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a361 = v1;
                let a362 = {
                    let m360 = f140(v0);
                    match m360 {
                        Adt5::C0(h359) => {
                            h359
                        }
                    }
                };
                append_list(a361, a362)
            })))
        }
    }
}

pub fn f102() -> u64 {
    200u64
}

pub fn f103(v0: Adt89, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f77())), List::cons((Str::lit("id"), f99(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h363) => {
            h363
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f104(v0: Adt110) -> u64 {
    let m364 = v0;
    match m364 {
        Adt110::C0(_) => {
            f43()
        }
        Adt110::C1(_, _) => {
            f43()
        }
        Adt110::C2(_) => {
            f43()
        }
        Adt110::C3(_) => {
            f141()
        }
        Adt110::C4(v6) => {
            f142(v6)
        }
    }
}

pub fn f105(v0: Adt110) -> Adt78 {
    Adt78::C0(f143(v0.clone()), f144(v0.clone()), f145(v0))
}

pub fn f106(v0: Adt0, v1: Fn0, v2: Result<Adt116, Adt110>) -> Result<Adt108, Adt110> {
    let m365 = v2;
    match m365 {
        Ok(v4) => {
            f146(v0, v1, v4)
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(v3)
        }
    }
}

pub fn f107(v0: Option<Adt93>, v1: List<Str>, v2: Adt92) -> R<Result<Adt116, Adt110>> {
    Ok(f147(f148(match v2.clone() {
        Adt92::C0(_, h366, _) => {
            h366
        }
    }), f149(v0, v1, v2)?))
}

pub fn f108(v0: Adt95, v1: Adt118) -> Adt112 {
    let m367 = v1;
    match m367 {
        Adt118::C0 => {
            f115()
        }
        Adt118::C1 => {
            Adt112::C0(None::<Str>, Some(Adt94::C6(v0)))
        }
        Adt118::C2 => {
            Adt112::C0(None::<Str>, Some(Adt94::C7(v0)))
        }
        Adt118::C3(v2) => {
            let m368 = f150(v2);
            match m368 {
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

pub fn f109(v0: List<Adt84>, v1: List<u64>) -> R<Adt118> {
    let m369 = f151(v0, v1, Adt119::C0)?;
    match m369 {
        Adt119::C0 => {
            Ok(Adt118::C0)
        }
        Adt119::C1(v2) => {
            let m370 = f152(v2);
            match m370 {
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

pub fn f110() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f111() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f112(v0: Str) -> bool {
    let m371 = f153(v0);
    match m371 {
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

pub fn f113(v0: Adt118) -> R<Adt118> {
    let m372 = v0.clone();
    match m372 {
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
            let m373 = f154(v1);
            match m373 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m374 = f155(v2)?;
                    match m374 {
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

pub fn f114() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f115() -> Adt112 {
    Adt112::C0(None::<Str>, None::<Adt94>)
}

pub fn f116(v0: Option<Adt94>, v1: Option<Adt94>) -> Option<Adt94> {
    let m375 = v0.clone();
    match m375 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f117(v0: List<Adt84>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt96>> {
    Ok(f156(f157(v0, v1, v2, List::<Adt96>::nil())?, List::<Adt96>::nil()))
}

pub fn f118(v0: Adt6) -> Option<Str> {
    let m376 = v0;
    match m376 {
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

pub fn f119() -> Str {
    Str::lit("2026-07-28")
}

pub fn f120(v0: Adt7) -> R<Option<i64>> {
    let m379 = f158(40u64, match v0.clone() {
        Adt7::C0(h377, _) => {
            h377
        }
    }, match v0 {
        Adt7::C0(_, h378) => {
            h378
        }
    })?;
    match m379 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f159(v1))
        }
    }
}

pub fn f121(v0: Str, p385: List<(Str, Adt6)>) -> Adt114 {
    let m384 = p385;
    {
        let mut m384 = m384;
        loop {
            let step = {
                let v1 = m384;
                let m380 = v1;
                match m380.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m382 = f160({
                            let (h381, _) = v2;
                            h381
                        }, v0.clone());
                        match m382 {
                            Adt126::C0 => {
                                Err::<List<(Str, Adt6)>, Adt114>(Adt114::C1({
                                    let (_, h383) = v2;
                                    h383
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
                Ok(next) => { m384 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Str, v1: Adt5) -> bool {
    f161(f89(v0, v1))
}

pub fn f123(v0: Adt89, v1: Adt5) -> Result<Adt92, Adt90> {
    let m386 = f89(Str::lit("method"), v1.clone());
    match m386 {
        Adt114::C0 => {
            f67(Some(v0), f90(v1))
        }
        Adt114::C1(v2) => {
            let m387 = f118(v2);
            match m387 {
                None => {
                    f67(Some(v0), Adt91::C4)
                }
                Some(v3) => {
                    f162(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f124(v0: u64) -> R<u64> {
    let a390 = {
        let a388 = v0;
        let a389 = 3u64;
        nat_mul(a388, a389)?
    };
    let a391 = 1u64;
    nat_add(a390, a391)
}

pub fn f125(v0: u64, v1: u64, v2: u64) -> bool {
    let a392 = v1;
    let a393 = v0;
    if nat_le(a392, a393) {
        let a394 = v0;
        let a395 = v2;
        nat_le(a394, a395)
    } else {
        false
    }
}

pub fn f126(v0: Adt103) -> R<Adt120> {
    if f163(match v0.clone() {
        Adt103::C0(_, h396, _, _, _, _, _) => {
            h396
        }
    }) {
        let v1: i64 = {
            let a402 = if match v0.clone() {
                Adt103::C0(_, _, _, _, _, h397, _) => {
                    h397
                }
            } {
                let a399 = match v0.clone() {
                    Adt103::C0(_, _, _, _, _, _, h398) => {
                        h398
                    }
                };
                int_neg(a399)?
            } else {
                match v0.clone() {
                    Adt103::C0(_, _, _, _, _, _, h400) => {
                        h400
                    }
                }
            };
            let a403 = match v0.clone() {
                Adt103::C0(_, _, _, _, h401, _, _) => {
                    h401
                }
            };
            int_sub(a402, a403)?
        };
        let m406 = {
            let a404 = v1;
            let a405 = -9999i64;
            compare(a404, a405)
        };
        if if match m406 {
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
            let m409 = {
                let a407 = 9999i64;
                let a408 = v1;
                compare(a407, a408)
            };
            match m409 {
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
                Adt103::C0(h410, _, _, _, _, _, _) => {
                    h410
                }
            } {
                let a412 = match v0 {
                    Adt103::C0(_, _, h411, _, _, _, _) => {
                        h411
                    }
                };
                int_neg(a412)?
            } else {
                match v0 {
                    Adt103::C0(_, _, h413, _, _, _, _) => {
                        h413
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt120::C0)
    }
}

pub fn f127(v0: Adt6, v1: List<Adt105>, v2: u64) -> Adt100 {
    let m414 = v1.clone();
    match m414.uncons() {
        None => {
            Adt100::C0(Adt101::C9(v0), List::<Adt105>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m415 = v3;
            match m415 {
                Adt105::C0(v5) => {
                    Adt100::C0(Adt101::C2, List::cons(Adt105::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt105::C1(v6, v7) => {
                    let m416 = v7;
                    match m416 {
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

pub fn f128(v0: u64) -> bool {
    let a417 = v0;
    let a418 = 32u64;
    if nat_eq(a417, a418) {
        true
    } else {
        let a419 = v0;
        let a420 = 10u64;
        if nat_eq(a419, a420) {
            true
        } else {
            let a421 = v0;
            let a422 = 13u64;
            if nat_eq(a421, a422) {
                true
            } else {
                let a423 = v0;
                let a424 = 9u64;
                nat_eq(a423, a424)
            }
        }
    }
}

pub fn f129(v0: Adt100, v1: u64, v2: u64) -> R<Adt115> {
    let a425 = v2;
    let a426 = 34u64;
    if nat_eq(a425, a426) {
        Ok(f133(v0, Adt101::C6(Adt102::C0, List::<u64>::nil(), false)))
    } else {
        let a427 = v2;
        let a428 = 123u64;
        if nat_eq(a427, a428) {
            f164(v0, Adt105::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt101::C3, v1)
        } else {
            let a429 = v2;
            let a430 = 91u64;
            if nat_eq(a429, a430) {
                f164(v0, Adt105::C0(List::<Adt6>::nil()), Adt101::C1, v1)
            } else {
                let a431 = v2;
                let a432 = 116u64;
                if nat_eq(a431, a432) {
                    Ok(f133(v0, Adt101::C7(0u64, 1u64)))
                } else {
                    let a433 = v2;
                    let a434 = 102u64;
                    if nat_eq(a433, a434) {
                        Ok(f133(v0, Adt101::C7(1u64, 1u64)))
                    } else {
                        let a435 = v2;
                        let a436 = 110u64;
                        if nat_eq(a435, a436) {
                            Ok(f133(v0, Adt101::C7(2u64, 1u64)))
                        } else {
                            let a437 = v2;
                            let a438 = 45u64;
                            if nat_eq(a437, a438) {
                                Ok(f133(v0, f165(true, Adt104::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a439 = v2;
                                let a440 = 48u64;
                                if nat_eq(a439, a440) {
                                    Ok(f133(v0, f165(false, Adt104::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f125(v2, 49u64, 57u64) {
                                        Ok(f133(v0, f165(false, Adt104::C2, f166({
                                            let a441 = v2;
                                            let a442 = 48u64;
                                            nat_sub(a441, a442)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f134(Adt88::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f130(v0: Adt100, v1: u64) -> Adt115 {
    let m444 = match v0.clone() {
        Adt100::C0(_, h443, _) => {
            h443
        }
    };
    match m444.uncons() {
        None => {
            f134(Adt88::C2(v1))
        }
        Some((v2, v3)) => {
            let m445 = v2;
            match m445 {
                Adt105::C0(v4) => {
                    Adt115::C1(f127(Adt6::C4(f72(v4, List::<Adt6>::nil())), v3, {
                        let a447 = match v0 {
                            Adt100::C0(_, _, h446) => {
                                h446
                            }
                        };
                        let a448 = 1u64;
                        nat_sub(a447, a448)
                    }))
                }
                Adt105::C1(_, _) => {
                    f134(Adt88::C2(v1))
                }
            }
        }
    }
}

pub fn f131(v0: Adt100, v1: u64, v2: u64) -> Adt115 {
    let m450 = match v0.clone() {
        Adt100::C0(h449, _, _) => {
            h449
        }
    };
    match m450 {
        Adt101::C0 => {
            f134(Adt88::C2(v1))
        }
        Adt101::C1 => {
            f134(Adt88::C2(v1))
        }
        Adt101::C2 => {
            if f128(v2) {
                Adt115::C1(v0)
            } else {
                let a451 = v2;
                let a452 = 44u64;
                if nat_eq(a451, a452) {
                    f167(v0, v1)
                } else {
                    let a453 = v2;
                    let a454 = 93u64;
                    if nat_eq(a453, a454) {
                        f130(v0, v1)
                    } else {
                        let a455 = v2;
                        let a456 = 125u64;
                        if nat_eq(a455, a456) {
                            f132(v0, v1)
                        } else {
                            f134(Adt88::C2(v1))
                        }
                    }
                }
            }
        }
        Adt101::C3 => {
            f134(Adt88::C2(v1))
        }
        Adt101::C4 => {
            f134(Adt88::C2(v1))
        }
        Adt101::C5 => {
            f134(Adt88::C2(v1))
        }
        Adt101::C6(_, _, _) => {
            f134(Adt88::C2(v1))
        }
        Adt101::C7(_, _) => {
            f134(Adt88::C2(v1))
        }
        Adt101::C8(_) => {
            f134(Adt88::C2(v1))
        }
        Adt101::C9(_) => {
            if f128(v2) {
                Adt115::C1(v0)
            } else {
                f134(Adt88::C10(v1))
            }
        }
    }
}

pub fn f132(v0: Adt100, v1: u64) -> Adt115 {
    let m458 = match v0.clone() {
        Adt100::C0(_, h457, _) => {
            h457
        }
    };
    match m458.uncons() {
        None => {
            f134(Adt88::C2(v1))
        }
        Some((v2, v3)) => {
            let m459 = v2;
            match m459 {
                Adt105::C0(_) => {
                    f134(Adt88::C2(v1))
                }
                Adt105::C1(v5, _) => {
                    Adt115::C1(f127(Adt6::C5(f168(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a461 = match v0 {
                            Adt100::C0(_, _, h460) => {
                                h460
                            }
                        };
                        let a462 = 1u64;
                        nat_sub(a461, a462)
                    }))
                }
            }
        }
    }
}

pub fn f133(v0: Adt100, v1: Adt101) -> Adt115 {
    Adt115::C1(f169(v0, v1))
}

pub fn f134(v0: Adt88) -> Adt115 {
    Adt115::C0(v0)
}

pub fn f135(v0: Adt100, v1: u64, v2: u64, v3: Adt102, v4: List<u64>, v5: bool) -> R<Adt115> {
    let m463 = v3;
    match m463 {
        Adt102::C0 => {
            let a464 = v2;
            let a465 = 34u64;
            if nat_eq(a464, a465) {
                Ok(f170(v0, v1, v4, v5))
            } else {
                let a466 = v2;
                let a467 = 92u64;
                if nat_eq(a466, a467) {
                    Ok(f133(v0, Adt101::C6(Adt102::C1, v4, v5)))
                } else {
                    let a468 = v2;
                    let a469 = 32u64;
                    if nat_lt(a468, a469) {
                        Ok(f134(Adt88::C4(v1)))
                    } else {
                        Ok(f133(v0, Adt101::C6(Adt102::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt102::C1 => {
            let a470 = v2;
            let a471 = 117u64;
            if nat_eq(a470, a471) {
                Ok(f133(v0, Adt101::C6(Adt102::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a472 = f171(v2);
                let a473 = 0u64;
                if nat_eq(a472, a473) {
                    Ok(f134(Adt88::C5(v1)))
                } else {
                    Ok(f133(v0, Adt101::C6(Adt102::C0, List::cons(f171(v2), v4), v5)))
                }
            }
        }
        Adt102::C2(v6, v7, v8) => {
            let a474 = 15u64;
            let a475 = f172(v2);
            if nat_lt(a474, a475) {
                Ok(f134(Adt88::C5(v1)))
            } else {
                let a478 = {
                    let a476 = v6;
                    let a477 = 1u64;
                    nat_add(a476, a477)?
                };
                let a479 = 4u64;
                if nat_eq(a478, a479) {
                    f173(v0, v1, {
                        let a482 = {
                            let a480 = v7;
                            let a481 = 16u64;
                            nat_mul(a480, a481)?
                        };
                        let a483 = f172(v2);
                        nat_add(a482, a483)?
                    }, v8, v4, v5)
                } else {
                    Ok(f133(v0, Adt101::C6(Adt102::C2({
                        let a484 = v6;
                        let a485 = 1u64;
                        nat_add(a484, a485)?
                    }, {
                        let a488 = {
                            let a486 = v7;
                            let a487 = 16u64;
                            nat_mul(a486, a487)?
                        };
                        let a489 = f172(v2);
                        nat_add(a488, a489)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt102::C3(v9) => {
            let a490 = v2;
            let a491 = 92u64;
            if nat_eq(a490, a491) {
                Ok(f133(v0, Adt101::C6(Adt102::C4(v9), v4, v5)))
            } else {
                Ok(f134(Adt88::C6(v1)))
            }
        }
        Adt102::C4(v10) => {
            let a492 = v2;
            let a493 = 117u64;
            if nat_eq(a492, a493) {
                Ok(f133(v0, Adt101::C6(Adt102::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f134(Adt88::C6(v1)))
            }
        }
    }
}

pub fn f136(v0: Adt100, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt115> {
    let a494 = v2;
    let a495 = f174(v3, v4);
    if nat_eq(a494, a495) {
        let a499 = {
            let a496 = v4;
            let a497 = 1u64;
            nat_add(a496, a497)?
        };
        let a500 = {
            let a498 = f175(v3);
            length_list(a498)
        };
        if nat_eq(a499, a500) {
            Ok(f176(v0, f177(v3)))
        } else {
            Ok(f133(v0, Adt101::C7(v3, {
                let a501 = v4;
                let a502 = 1u64;
                nat_add(a501, a502)?
            })))
        }
    } else {
        Ok(f134(Adt88::C2(v1)))
    }
}

pub fn f137(v0: Adt100, v1: u64, v2: u64, v3: Adt103) -> R<Adt115> {
    let m504 = match v3.clone() {
        Adt103::C0(_, h503, _, _, _, _, _) => {
            h503
        }
    };
    match m504 {
        Adt104::C0 => {
            let a505 = v2;
            let a506 = 48u64;
            if nat_eq(a505, a506) {
                Ok(f178(v0, v3, Adt104::C1))
            } else {
                if f125(v2, 49u64, 57u64) {
                    f179(v0, v1, v2, v3, false)
                } else {
                    Ok(f134(Adt88::C7(v1)))
                }
            }
        }
        Adt104::C1 => {
            let a507 = v2;
            let a508 = 46u64;
            if nat_eq(a507, a508) {
                Ok(f178(v0, v3, Adt104::C3))
            } else {
                if f180(v2) {
                    Ok(f178(v0, v3, Adt104::C5))
                } else {
                    if f181(v2) {
                        Ok(f134(Adt88::C7(v1)))
                    } else {
                        f182(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt104::C2 => {
            if f181(v2) {
                f179(v0, v1, v2, v3, false)
            } else {
                let a509 = v2;
                let a510 = 46u64;
                if nat_eq(a509, a510) {
                    Ok(f178(v0, v3, Adt104::C3))
                } else {
                    if f180(v2) {
                        Ok(f178(v0, v3, Adt104::C5))
                    } else {
                        f182(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt104::C3 => {
            if f181(v2) {
                f179(v0, v1, v2, v3, true)
            } else {
                Ok(f134(Adt88::C7(v1)))
            }
        }
        Adt104::C4 => {
            if f181(v2) {
                f179(v0, v1, v2, v3, true)
            } else {
                if f180(v2) {
                    Ok(f178(v0, v3, Adt104::C5))
                } else {
                    f182(v0, v1, v2, v3)
                }
            }
        }
        Adt104::C5 => {
            let a511 = v2;
            let a512 = 43u64;
            if nat_eq(a511, a512) {
                Ok(f133(v0, f165(match v3.clone() {
                    Adt103::C0(h513, _, _, _, _, _, _) => {
                        h513
                    }
                }, Adt104::C6, match v3.clone() {
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
                }, false, 0i64)))
            } else {
                let a517 = v2;
                let a518 = 45u64;
                if nat_eq(a517, a518) {
                    Ok(f133(v0, f165(match v3.clone() {
                        Adt103::C0(h519, _, _, _, _, _, _) => {
                            h519
                        }
                    }, Adt104::C6, match v3.clone() {
                        Adt103::C0(_, _, h520, _, _, _, _) => {
                            h520
                        }
                    }, match v3.clone() {
                        Adt103::C0(_, _, _, h521, _, _, _) => {
                            h521
                        }
                    }, match v3 {
                        Adt103::C0(_, _, _, _, h522, _, _) => {
                            h522
                        }
                    }, true, 0i64)))
                } else {
                    if f181(v2) {
                        f183(v0, v1, v2, Adt103::C0(match v3.clone() {
                            Adt103::C0(h523, _, _, _, _, _, _) => {
                                h523
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, h524, _, _, _, _, _) => {
                                h524
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, _, h525, _, _, _, _) => {
                                h525
                            }
                        }, match v3.clone() {
                            Adt103::C0(_, _, _, h526, _, _, _) => {
                                h526
                            }
                        }, match v3 {
                            Adt103::C0(_, _, _, _, h527, _, _) => {
                                h527
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f134(Adt88::C7(v1)))
                    }
                }
            }
        }
        Adt104::C6 => {
            if f181(v2) {
                f183(v0, v1, v2, v3.clone(), match v3 {
                    Adt103::C0(_, _, _, _, _, h528, _) => {
                        h528
                    }
                })
            } else {
                Ok(f134(Adt88::C7(v1)))
            }
        }
        Adt104::C7 => {
            if f181(v2) {
                f183(v0, v1, v2, v3.clone(), match v3 {
                    Adt103::C0(_, _, _, _, _, h529, _) => {
                        h529
                    }
                })
            } else {
                f182(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f138(v0: Adt106, v1: List<Adt106>, v2: List<u64>) -> R<Adt121> {
    let m530 = v0;
    match m530 {
        Adt106::C0(v3) => {
            Ok(Adt121::C1(v1, f98(v3, v2)))
        }
        Adt106::C1(v4, v5) => {
            f184(v4, v5, v1, v2)
        }
        Adt106::C2(v6, v7) => {
            let m531 = v7;
            match m531.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C1(v6, v8), List::cons(Adt106::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt106::C3(v10, v11) => {
            let m532 = v11.clone();
            match m532.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt106::C4(v14, v15) => {
            let m533 = v15;
            match m533.uncons() {
                None => {
                    Ok(Adt121::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt121::C1(List::cons(Adt106::C0({
                        let a535 = f185({
                            let (h534, _) = v16;
                            h534
                        })?;
                        let a536 = List::cons(58u64, List::<u64>::nil());
                        append_list(a535, a536)
                    }), List::cons(Adt106::C1(v14, {
                        let (_, h537) = v16;
                        h537
                    }), List::cons(Adt106::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt106::C5(v18, v19) => {
            let m538 = v19.clone();
            match m538.uncons() {
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

pub fn f139(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m539 = v0;
    match m539 {
        Adt32::C0(v1) => {
            Ok(Some(f186(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f187(v2)?))
        }
        Adt32::C2(v3) => {
            Ok(f188(v3))
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

pub fn f140(v0: Adt108) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f189(f190(match v0.clone() {
        Adt108::C0(h540, _) => {
            h540
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f191({
        let m543 = match v0 {
            Adt108::C0(_, h542) => {
                h542
            }
        };
        match m543 {
            Adt109::C0(h541, _) => {
                h541
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f141() -> u64 {
    404u64
}

pub fn f142(v0: Adt82) -> u64 {
    let m544 = v0;
    match m544 {
        Adt82::C0(_) => {
            f43()
        }
        Adt82::C1(_) => {
            f43()
        }
        Adt82::C2(_) => {
            f43()
        }
        Adt82::C3(_) => {
            f43()
        }
        Adt82::C4(_) => {
            f43()
        }
        Adt82::C5(_) => {
            f43()
        }
        Adt82::C6(_) => {
            f18()
        }
    }
}

pub fn f143(v0: Adt110) -> i64 {
    let m545 = v0;
    match m545 {
        Adt110::C0(_) => {
            f192()
        }
        Adt110::C1(_, _) => {
            f193()
        }
        Adt110::C2(_) => {
            f194()
        }
        Adt110::C3(_) => {
            f195()
        }
        Adt110::C4(v6) => {
            f196(v6)
        }
    }
}

pub fn f144(v0: Adt110) -> Str {
    let m546 = v0;
    match m546 {
        Adt110::C0(v1) => {
            f197(v1)
        }
        Adt110::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt110::C2(v4) => {
            f198(v4)
        }
        Adt110::C3(_) => {
            Str::lit("Method not found")
        }
        Adt110::C4(v6) => {
            let m547 = v6;
            match m547 {
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

pub fn f145(v0: Adt110) -> Option<Adt6> {
    let m548 = v0;
    match m548 {
        Adt110::C0(_) => {
            None::<Adt6>
        }
        Adt110::C1(v2, v3) => {
            Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f53(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil()))))
        }
        Adt110::C2(_) => {
            None::<Adt6>
        }
        Adt110::C3(_) => {
            None::<Adt6>
        }
        Adt110::C4(v6) => {
            let m549 = v6;
            match m549 {
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
                    Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f199(v12)), List::<(Str, Adt6)>::nil())))
                }
                Adt82::C6(_) => {
                    None::<Adt6>
                }
            }
        }
    }
}

pub fn f146(v0: Adt0, v1: Fn0, v2: Adt116) -> Result<Adt108, Adt110> {
    f200(v0.clone(), v1, v2.clone(), f201(match v0 {
        Adt0::C0(_, _, h550, _, _) => {
            h550
        }
    }, match v2 {
        Adt116::C0(_, h551, _, _) => {
            h551
        }
    }))
}

pub fn f147(v0: Adt122, v1: Adt123) -> Result<Adt116, Adt110> {
    let m552 = v1;
    match m552 {
        Adt123::C0(v3) => {
            f202(v3, v0)
        }
        Adt123::C1(v2) => {
            Err::<Adt116, Adt110>(v2)
        }
    }
}

pub fn f148(v0: Str) -> Adt122 {
    let a553 = v0.clone();
    let a554 = Str::lit("server/discover");
    if equal(a553, a554) {
        Adt122::C1(Adt117::C0)
    } else {
        let a555 = v0.clone();
        let a556 = Str::lit("tools/list");
        if equal(a555, a556) {
            Adt122::C1(Adt117::C1)
        } else {
            let a557 = v0.clone();
            let a558 = Str::lit("tools/call");
            if equal(a557, a558) {
                Adt122::C1(Adt117::C2)
            } else {
                let a559 = v0.clone();
                let a560 = Str::lit("resources/list");
                if equal(a559, a560) {
                    Adt122::C1(Adt117::C3)
                } else {
                    let a561 = v0.clone();
                    let a562 = Str::lit("resources/read");
                    if equal(a561, a562) {
                        Adt122::C1(Adt117::C4)
                    } else {
                        let a563 = v0.clone();
                        let a564 = Str::lit("resources/templates/list");
                        if equal(a563, a564) {
                            Adt122::C1(Adt117::C5)
                        } else {
                            let a565 = v0.clone();
                            let a566 = Str::lit("prompts/list");
                            if equal(a565, a566) {
                                Adt122::C1(Adt117::C6)
                            } else {
                                let a567 = v0.clone();
                                let a568 = Str::lit("prompts/get");
                                if equal(a567, a568) {
                                    Adt122::C1(Adt117::C7)
                                } else {
                                    let a569 = v0.clone();
                                    let a570 = Str::lit("completion/complete");
                                    if equal(a569, a570) {
                                        Adt122::C1(Adt117::C8)
                                    } else {
                                        let a571 = v0.clone();
                                        let a572 = Str::lit("tasks/get");
                                        if equal(a571, a572) {
                                            Adt122::C1(Adt117::C9)
                                        } else {
                                            let a573 = v0.clone();
                                            let a574 = Str::lit("tasks/update");
                                            if equal(a573, a574) {
                                                Adt122::C1(Adt117::C10)
                                            } else {
                                                let a575 = v0;
                                                let a576 = Str::lit("tasks/cancel");
                                                if equal(a575, a576) {
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

pub fn f149(v0: Option<Adt93>, v1: List<Str>, v2: Adt92) -> R<Adt123> {
    f203(v0, v1, v2.clone(), f204(match v2 {
        Adt92::C0(_, _, h577) => {
            h577
        }
    }))
}

pub fn f150(v0: List<u64>) -> Option<Str> {
    let a578 = f205(f206(), v0);
    utf8_decode(a578)
}

pub fn f151(p583: List<Adt84>, v1: List<u64>, p584: Adt119) -> R<Adt119> {
    let m582 = (p583, p584);
    {
        let mut m582 = m582;
        loop {
            let step = {
                let (v0, v2) = m582;
                let m579 = v0;
                match m579.uncons() {
                    None => {
                        Err::<(List<Adt84>, Adt119), R<Adt119>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f207(match v3.clone() {
                            Adt84::C0(h580, _) => {
                                h580
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt84>, Adt119), R<Adt119>>((v4, f208(v2, match v3 {
                                Adt84::C0(_, h581) => {
                                    h581
                                }
                            })))
                        } else {
                            Ok::<(List<Adt84>, Adt119), R<Adt119>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f209(v0);
    if f210(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f153(v0: Str) -> Adt125 {
    let m585 = f148(v0);
    match m585 {
        Adt122::C0 => {
            Adt125::C0
        }
        Adt122::C1(v1) => {
            f211(v1)
        }
    }
}

pub fn f154(v0: List<u64>) -> Option<List<u64>> {
    let m586 = f212(f213(), v0);
    match m586 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m587 = f212(f214(), f215(v1, List::<u64>::nil()));
            match m587 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f215(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f155(v0: List<u64>) -> R<Option<List<u64>>> {
    let m589 = f216({
        let a588 = v0.clone();
        length_list(a588)
    }, v0, List::<u64>::nil())?;
    match m589 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f98(v1, List::<u64>::nil())))
        }
    }
}

pub fn f156(p592: List<Adt96>, p593: List<Adt96>) -> List<Adt96> {
    let m591 = (p592, p593);
    {
        let mut m591 = m591;
        loop {
            let step = {
                let (v0, v1) = m591;
                let m590 = v0;
                match m590.uncons() {
                    None => {
                        Err::<(List<Adt96>, List<Adt96>), List<Adt96>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt96>, List<Adt96>), List<Adt96>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f157(v0: List<Adt84>, p600: List<Adt12>, v2: Option<Str>, p601: List<Adt96>) -> R<List<Adt96>> {
    let m599 = (p600, p601);
    {
        let mut m599 = m599;
        loop {
            let step = {
                let (v1, v3) = m599;
                let m594 = v1;
                match m594.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt96>), R<List<Adt96>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f217(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt96>), R<List<Adt96>>>((v5, List::cons(Adt96::C0(match v4.clone() {
                                Adt12::C0(_, h595, _, _) => {
                                    h595
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h596, _) => {
                                    h596
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h597) => {
                                    h597
                                }
                            }, f218(v0.clone(), match v4 {
                                Adt12::C0(_, h598, _, _) => {
                                    h598
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt96>), R<List<Adt96>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m599 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(p617: u64, p618: i64, p619: i64) -> R<Option<i64>> {
    let m616 = (p617, (p618, p619));
    {
        let mut m616 = m616;
        loop {
            let step = {
                let (v0, (v1, v2)) = m616;
                let m602 = v0;
                if m602 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m602 - 1;
                    if f219(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m605 = {
                            let a603 = v2;
                            let a604 = 0i64;
                            compare(a603, a604)
                        };
                        if match m605 {
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
                            if f219({
                                let a606 = v1;
                                let a607 = 10i64;
                                let a608 = 0i64;
                                int_rem(a606, a607, a608)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a609 = v1;
                                    let a610 = 10i64;
                                    let a611 = 0i64;
                                    int_quot(a609, a610, a611)?
                                }, f220(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a612 = v1;
                                let a613 = 10i64;
                                int_mul(a612, a613)?
                            }, {
                                let a614 = v2;
                                let a615 = 1i64;
                                int_sub(a614, a615)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: i64) -> Option<i64> {
    let m622 = {
        let a620 = 9007199254740991i64;
        let a621 = v0;
        compare(a620, a621)
    };
    if if match m622 {
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
        let m625 = {
            let a623 = v0;
            let a624 = -9007199254740991i64;
            compare(a623, a624)
        };
        match m625 {
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

pub fn f160(v0: Str, v1: Str) -> Adt126 {
    let a626 = v0;
    let a627 = v1;
    if equal(a626, a627) {
        Adt126::C0
    } else {
        Adt126::C1
    }
}

pub fn f161(v0: Adt114) -> bool {
    let m628 = v0;
    match m628 {
        Adt114::C0 => {
            false
        }
        Adt114::C1(_) => {
            true
        }
    }
}

pub fn f162(v0: Adt89, v1: Str, v2: Adt5) -> Result<Adt92, Adt90> {
    let m629 = f89(Str::lit("params"), v2);
    match m629 {
        Adt114::C0 => {
            Ok::<Adt92, Adt90>(Adt92::C0(v0, v1, None::<Adt5>))
        }
        Adt114::C1(v3) => {
            let m630 = f221(v3);
            match m630 {
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

pub fn f163(v0: Adt104) -> bool {
    let m631 = v0;
    match m631 {
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

pub fn f164(v0: Adt100, v1: Adt105, v2: Adt101, v3: u64) -> R<Adt115> {
    let a633 = match v0.clone() {
        Adt100::C0(_, _, h632) => {
            h632
        }
    };
    let a634 = f222();
    if nat_lt(a633, a634) {
        Ok(Adt115::C1(Adt100::C0(v2, List::cons(v1, match v0.clone() {
            Adt100::C0(_, h635, _) => {
                h635
            }
        }), {
            let a637 = match v0 {
                Adt100::C0(_, _, h636) => {
                    h636
                }
            };
            let a638 = 1u64;
            nat_add(a637, a638)?
        })))
    } else {
        Ok(f134(Adt88::C9(v3)))
    }
}

pub fn f165(v0: bool, v1: Adt104, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt101 {
    Adt101::C8(Adt103::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f166(v0: u64) -> i64 {
    let a639 = v0;
    let a640 = 0u64;
    if nat_eq(a639, a640) {
        0i64
    } else {
        let a641 = v0;
        let a642 = 1u64;
        if nat_eq(a641, a642) {
            1i64
        } else {
            let a643 = v0;
            let a644 = 2u64;
            if nat_eq(a643, a644) {
                2i64
            } else {
                let a645 = v0;
                let a646 = 3u64;
                if nat_eq(a645, a646) {
                    3i64
                } else {
                    let a647 = v0;
                    let a648 = 4u64;
                    if nat_eq(a647, a648) {
                        4i64
                    } else {
                        let a649 = v0;
                        let a650 = 5u64;
                        if nat_eq(a649, a650) {
                            5i64
                        } else {
                            let a651 = v0;
                            let a652 = 6u64;
                            if nat_eq(a651, a652) {
                                6i64
                            } else {
                                let a653 = v0;
                                let a654 = 7u64;
                                if nat_eq(a653, a654) {
                                    7i64
                                } else {
                                    let a655 = v0;
                                    let a656 = 8u64;
                                    if nat_eq(a655, a656) {
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

pub fn f167(v0: Adt100, v1: u64) -> Adt115 {
    let m658 = match v0.clone() {
        Adt100::C0(_, h657, _) => {
            h657
        }
    };
    match m658.uncons() {
        None => {
            f134(Adt88::C2(v1))
        }
        Some((v2, _)) => {
            let m659 = v2;
            match m659 {
                Adt105::C0(_) => {
                    f133(v0, Adt101::C0)
                }
                Adt105::C1(_, _) => {
                    f133(v0, Adt101::C4)
                }
            }
        }
    }
}

pub fn f168(p662: List<(Str, Adt6)>, p663: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m661 = (p662, p663);
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v0, v1) = m661;
                let m660 = v0;
                match m660.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Adt100, v1: Adt101) -> Adt100 {
    Adt100::C0(v1, match v0.clone() {
        Adt100::C0(_, h664, _) => {
            h664
        }
    }, match v0 {
        Adt100::C0(_, _, h665) => {
            h665
        }
    })
}

pub fn f170(v0: Adt100, v1: u64, v2: List<u64>, v3: bool) -> Adt115 {
    let m667 = {
        let a666 = f205(f206(), f98(v2, List::<u64>::nil()));
        utf8_decode(a666)
    };
    match m667 {
        None => {
            f134(Adt88::C14)
        }
        Some(v4) => {
            if v3 {
                f223(v0, v1, v4)
            } else {
                f176(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f171(v0: u64) -> u64 {
    let a668 = v0;
    let a669 = 34u64;
    if nat_eq(a668, a669) {
        34u64
    } else {
        let a670 = v0;
        let a671 = 92u64;
        if nat_eq(a670, a671) {
            92u64
        } else {
            let a672 = v0;
            let a673 = 47u64;
            if nat_eq(a672, a673) {
                47u64
            } else {
                let a674 = v0;
                let a675 = 98u64;
                if nat_eq(a674, a675) {
                    8u64
                } else {
                    let a676 = v0;
                    let a677 = 102u64;
                    if nat_eq(a676, a677) {
                        12u64
                    } else {
                        let a678 = v0;
                        let a679 = 110u64;
                        if nat_eq(a678, a679) {
                            10u64
                        } else {
                            let a680 = v0;
                            let a681 = 114u64;
                            if nat_eq(a680, a681) {
                                13u64
                            } else {
                                let a682 = v0;
                                let a683 = 116u64;
                                if nat_eq(a682, a683) {
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

pub fn f172(v0: u64) -> u64 {
    let a684 = v0;
    let a685 = 48u64;
    if nat_lt(a684, a685) {
        16u64
    } else {
        let a686 = v0;
        let a687 = 58u64;
        if nat_lt(a686, a687) {
            let a688 = v0;
            let a689 = 48u64;
            nat_sub(a688, a689)
        } else {
            let a690 = v0;
            let a691 = 65u64;
            if nat_lt(a690, a691) {
                16u64
            } else {
                let a692 = v0;
                let a693 = 71u64;
                if nat_lt(a692, a693) {
                    let a694 = v0;
                    let a695 = 55u64;
                    nat_sub(a694, a695)
                } else {
                    let a696 = v0;
                    let a697 = 97u64;
                    if nat_lt(a696, a697) {
                        16u64
                    } else {
                        let a698 = v0;
                        let a699 = 103u64;
                        if nat_lt(a698, a699) {
                            let a700 = v0;
                            let a701 = 87u64;
                            nat_sub(a700, a701)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f173(v0: Adt100, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt115> {
    let a702 = v3;
    let a703 = 0u64;
    if nat_eq(a702, a703) {
        if f125(v2, 55296u64, 56319u64) {
            Ok(f133(v0, Adt101::C6(Adt102::C3(v2), v4, v5)))
        } else {
            if f125(v2, 56320u64, 57343u64) {
                Ok(f134(Adt88::C6(v1)))
            } else {
                Ok(f133(v0, Adt101::C6(Adt102::C0, f224(v2, v4)?, v5)))
            }
        }
    } else {
        if f125(v2, 56320u64, 57343u64) {
            Ok(f133(v0, Adt101::C6(Adt102::C0, f224({
                let a712 = 65536u64;
                let a713 = {
                    let a710 = {
                        let a706 = {
                            let a704 = v3;
                            let a705 = 55296u64;
                            nat_sub(a704, a705)
                        };
                        let a707 = 1024u64;
                        nat_mul(a706, a707)?
                    };
                    let a711 = {
                        let a708 = v2;
                        let a709 = 56320u64;
                        nat_sub(a708, a709)
                    };
                    nat_add(a710, a711)?
                };
                nat_add(a712, a713)?
            }, v4)?, v5)))
        } else {
            Ok(f134(Adt88::C6(v1)))
        }
    }
}

pub fn f174(v0: u64, v1: u64) -> u64 {
    let m716 = {
        let a714 = f175(v0);
        let a715 = v1;
        index_list(a714, a715)
    };
    match m716 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f175(v0: u64) -> List<u64> {
    let a717 = v0;
    let a718 = 0u64;
    if nat_eq(a717, a718) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a719 = v0;
        let a720 = 1u64;
        if nat_eq(a719, a720) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f176(v0: Adt100, v1: Adt6) -> Adt115 {
    Adt115::C1(f127(v1, match v0.clone() {
        Adt100::C0(_, h721, _) => {
            h721
        }
    }, match v0 {
        Adt100::C0(_, _, h722) => {
            h722
        }
    }))
}

pub fn f177(v0: u64) -> Adt6 {
    let a723 = v0;
    let a724 = 0u64;
    if nat_eq(a723, a724) {
        Adt6::C1(true)
    } else {
        let a725 = v0;
        let a726 = 1u64;
        if nat_eq(a725, a726) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f178(v0: Adt100, v1: Adt103, v2: Adt104) -> Adt115 {
    f133(v0, Adt101::C8(Adt103::C0(match v1.clone() {
        Adt103::C0(h727, _, _, _, _, _, _) => {
            h727
        }
    }, v2, match v1.clone() {
        Adt103::C0(_, _, h728, _, _, _, _) => {
            h728
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, h729, _, _, _) => {
            h729
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, _, h730, _, _) => {
            h730
        }
    }, match v1.clone() {
        Adt103::C0(_, _, _, _, _, h731, _) => {
            h731
        }
    }, match v1 {
        Adt103::C0(_, _, _, _, _, _, h732) => {
            h732
        }
    })))
}

pub fn f179(v0: Adt100, v1: u64, v2: u64, v3: Adt103, v4: bool) -> R<Adt115> {
    let v5: u64 = {
        let a734 = match v3.clone() {
            Adt103::C0(_, _, _, h733, _, _, _) => {
                h733
            }
        };
        let a735 = 0u64;
        if if nat_eq(a734, a735) {
            let a736 = v2;
            let a737 = 48u64;
            nat_eq(a736, a737)
        } else {
            false
        } {
            0u64
        } else {
            let a739 = match v3.clone() {
                Adt103::C0(_, _, _, h738, _, _, _) => {
                    h738
                }
            };
            let a740 = 1u64;
            nat_add(a739, a740)?
        }
    };
    let v6: i64 = if v4 {
        f220(match v3.clone() {
            Adt103::C0(_, _, _, _, h741, _, _) => {
                h741
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt103::C0(_, _, _, _, h742, _, _) => {
                h742
            }
        }
    };
    let a743 = f225();
    let a744 = v5;
    if nat_lt(a743, a744) {
        Ok(f134(Adt88::C8(v1)))
    } else {
        let m747 = {
            let a745 = 99999i64;
            let a746 = v6;
            compare(a745, a746)
        };
        if match m747 {
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
            Ok(f134(Adt88::C8(v1)))
        } else {
            Ok(f133(v0, f165(match v3.clone() {
                Adt103::C0(h748, _, _, _, _, _, _) => {
                    h748
                }
            }, if v4 {
                Adt104::C4
            } else {
                Adt104::C2
            }, f220({
                let a750 = match v3.clone() {
                    Adt103::C0(_, _, h749, _, _, _, _) => {
                        h749
                    }
                };
                let a751 = 10i64;
                int_mul(a750, a751)?
            }, f166({
                let a752 = v2;
                let a753 = 48u64;
                nat_sub(a752, a753)
            }))?, v5, v6, match v3.clone() {
                Adt103::C0(_, _, _, _, _, h754, _) => {
                    h754
                }
            }, match v3 {
                Adt103::C0(_, _, _, _, _, _, h755) => {
                    h755
                }
            })))
        }
    }
}

pub fn f180(v0: u64) -> bool {
    let a756 = v0;
    let a757 = 101u64;
    if nat_eq(a756, a757) {
        true
    } else {
        let a758 = v0;
        let a759 = 69u64;
        nat_eq(a758, a759)
    }
}

pub fn f181(v0: u64) -> bool {
    f125(v0, 48u64, 57u64)
}

pub fn f182(v0: Adt100, v1: u64, v2: u64, v3: Adt103) -> R<Adt115> {
    let m760 = f126(v3)?;
    match m760 {
        Adt120::C0 => {
            Ok(f134(Adt88::C7(v1)))
        }
        Adt120::C1 => {
            Ok(f134(Adt88::C8(v1)))
        }
        Adt120::C2(v4) => {
            Ok(f131(f127(v4, match v0.clone() {
                Adt100::C0(_, h761, _) => {
                    h761
                }
            }, match v0 {
                Adt100::C0(_, _, h762) => {
                    h762
                }
            }), v1, v2))
        }
    }
}

pub fn f183(v0: Adt100, v1: u64, v2: u64, v3: Adt103, v4: bool) -> R<Adt115> {
    let v5: i64 = f220({
        let a764 = match v3.clone() {
            Adt103::C0(_, _, _, _, _, _, h763) => {
                h763
            }
        };
        let a765 = 10i64;
        int_mul(a764, a765)?
    }, f166({
        let a766 = v2;
        let a767 = 48u64;
        nat_sub(a766, a767)
    }))?;
    let m770 = {
        let a768 = 99999i64;
        let a769 = v5;
        compare(a768, a769)
    };
    if match m770 {
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
        Ok(f134(Adt88::C8(v1)))
    } else {
        Ok(f133(v0, f165(match v3.clone() {
            Adt103::C0(h771, _, _, _, _, _, _) => {
                h771
            }
        }, Adt104::C7, match v3.clone() {
            Adt103::C0(_, _, h772, _, _, _, _) => {
                h772
            }
        }, match v3.clone() {
            Adt103::C0(_, _, _, h773, _, _, _) => {
                h773
            }
        }, match v3 {
            Adt103::C0(_, _, _, _, h774, _, _) => {
                h774
            }
        }, v4, v5)))
    }
}

pub fn f184(v0: u64, v1: Adt6, v2: List<Adt106>, v3: List<u64>) -> R<Adt121> {
    let m775 = v1;
    match m775 {
        Adt6::C0 => {
            Ok(Adt121::C1(v2, f98(f226(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt121::C1(v2, f98(if v4 {
                f226(0u64)
            } else {
                f226(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f227(v5.clone())? {
                Ok(Adt121::C1(v2, f98(f228(v5)?, v3)))
            } else {
                Ok(Adt121::C0(Adt88::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt121::C1(v2, f98(f185(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a776 = v0;
            let a777 = f222();
            if nat_lt(a776, a777) {
                Ok(Adt121::C1(f229(91u64, Adt106::C2({
                    let a778 = v0;
                    let a779 = 1u64;
                    nat_add(a778, a779)?
                }, v7), v2), v3))
            } else {
                Ok(Adt121::C0(Adt88::C12))
            }
        }
        Adt6::C5(v8) => {
            let a780 = v0;
            let a781 = f222();
            if nat_lt(a780, a781) {
                Ok(Adt121::C1(f229(123u64, Adt106::C4({
                    let a782 = v0;
                    let a783 = 1u64;
                    nat_add(a782, a783)?
                }, v8), v2), v3))
            } else {
                Ok(Adt121::C0(Adt88::C12))
            }
        }
    }
}

pub fn f185(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a785 = f98(f230(f49({
            let a784 = v0;
            utf8_encode(a784)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a786 = List::cons(34u64, List::<u64>::nil());
        append_list(a785, a786)
    }))
}

pub fn f186(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f231(match v0.clone() {
        Adt33::C0(_, h787, _, _) => {
            h787
        }
    })), List::cons((Str::lit("capabilities"), f232(match v0.clone() {
        Adt33::C0(_, _, h788, _) => {
            h788
        }
    })), {
        let a791 = f233(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h789) => {
                h789
            }
        });
        let a792 = f234(match v0 {
            Adt33::C0(h790, _, _, _) => {
                h790
            }
        })?;
        append_list(a791, a792)
    })))
}

pub fn f187(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f235(match v0.clone() {
        Adt34::C0(_, _, h793) => {
            h793
        }
    }))), {
        let a796 = f233(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h794, _) => {
                h794
            }
        });
        let a797 = f234(match v0 {
            Adt34::C0(h795, _, _) => {
                h795
            }
        })?;
        append_list(a796, a797)
    }))
}

pub fn f188(v0: Adt37) -> Option<List<(Str, Adt6)>> {
    let m799 = f236(match v0.clone() {
        Adt37::C0(h798, _, _) => {
            h798
        }
    });
    match m799 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a802 = f237(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h800, _) => {
                        h800
                    }
                });
                let a803 = f238(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h801) => {
                        h801
                    }
                });
                append_list(a802, a803)
            }))
        }
    }
}

pub fn f189(v0: Adt127) -> Str {
    let m804 = v0;
    match m804 {
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

pub fn f190(v0: Adt32) -> Adt127 {
    let m805 = v0;
    match m805 {
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

pub fn f191(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m806 = v0;
    match m806 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f239(), f240(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f192() -> i64 {
    -32602i64
}

pub fn f193() -> i64 {
    -32022i64
}

pub fn f194() -> i64 {
    -32020i64
}

pub fn f195() -> i64 {
    -32601i64
}

pub fn f196(v0: Adt82) -> i64 {
    let m807 = v0;
    match m807 {
        Adt82::C0(_) => {
            f192()
        }
        Adt82::C1(_) => {
            f192()
        }
        Adt82::C2(_) => {
            f192()
        }
        Adt82::C3(_) => {
            f192()
        }
        Adt82::C4(_) => {
            f192()
        }
        Adt82::C5(_) => {
            f241()
        }
        Adt82::C6(_) => {
            f242()
        }
    }
}

pub fn f197(v0: Adt111) -> Str {
    let m808 = v0;
    match m808 {
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

pub fn f198(v0: Adt94) -> Str {
    let m809 = v0;
    match m809 {
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
            let m810 = v1;
            match m810 {
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
            let m811 = v2;
            match m811 {
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

pub fn f199(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a823 = f243(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h812, _, _, _, _) => {
                h812
            }
        });
        let a824 = {
            let a821 = f243(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h813, _, _, _) => {
                    h813
                }
            });
            let a822 = {
                let a819 = f243(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h814, _, _) => {
                        h814
                    }
                });
                let a820 = {
                    let a817 = f243(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h815, _) => {
                            h815
                        }
                    });
                    let a818 = f243(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h816) => {
                            h816
                        }
                    });
                    append_list(a817, a818)
                };
                append_list(a819, a820)
            };
            append_list(a821, a822)
        };
        append_list(a823, a824)
    })
}

pub fn f200(v0: Adt0, v1: Fn0, v2: Adt116, v3: bool) -> Result<Adt108, Adt110> {
    if v3 {
        f244(v0, v1, f245(match v2.clone() {
            Adt116::C0(_, h825, _, _) => {
                h825
            }
        }, match v2.clone() {
            Adt116::C0(_, _, h826, _) => {
                h826
            }
        }, match v2 {
            Adt116::C0(_, _, _, h827) => {
                h827
            }
        }))
    } else {
        Err::<Adt108, Adt110>(Adt110::C3(f246(match v2 {
            Adt116::C0(_, h828, _, _) => {
                h828
            }
        })))
    }
}

pub fn f201(v0: Adt4, v1: Adt117) -> bool {
    let m829 = v1;
    match m829 {
        Adt117::C0 => {
            true
        }
        Adt117::C1 => {
            f247(match v0 {
                Adt4::C0(_, _, _, _, _, h830, _) => {
                    h830
                }
            })
        }
        Adt117::C2 => {
            f247(match v0 {
                Adt4::C0(_, _, _, _, _, h831, _) => {
                    h831
                }
            })
        }
        Adt117::C3 => {
            f248(match v0 {
                Adt4::C0(_, _, _, _, h832, _, _) => {
                    h832
                }
            })
        }
        Adt117::C4 => {
            f248(match v0 {
                Adt4::C0(_, _, _, _, h833, _, _) => {
                    h833
                }
            })
        }
        Adt117::C5 => {
            f248(match v0 {
                Adt4::C0(_, _, _, _, h834, _, _) => {
                    h834
                }
            })
        }
        Adt117::C6 => {
            f247(match v0 {
                Adt4::C0(_, _, _, h835, _, _, _) => {
                    h835
                }
            })
        }
        Adt117::C7 => {
            f247(match v0 {
                Adt4::C0(_, _, _, h836, _, _, _) => {
                    h836
                }
            })
        }
        Adt117::C8 => {
            f249(match v0 {
                Adt4::C0(_, _, h837, _, _, _, _) => {
                    h837
                }
            })
        }
        Adt117::C9 => {
            f250(match v0 {
                Adt4::C0(_, _, _, _, _, _, h838) => {
                    h838
                }
            }, f251())
        }
        Adt117::C10 => {
            f250(match v0 {
                Adt4::C0(_, _, _, _, _, _, h839) => {
                    h839
                }
            }, f251())
        }
        Adt117::C11 => {
            f250(match v0 {
                Adt4::C0(_, _, _, _, _, _, h840) => {
                    h840
                }
            }, f251())
        }
    }
}

pub fn f202(v0: Adt124, v1: Adt122) -> Result<Adt116, Adt110> {
    let m841 = v1;
    match m841 {
        Adt122::C0 => {
            Err::<Adt116, Adt110>(Adt110::C3(match v0 {
                Adt124::C0(_, h842, _, _) => {
                    h842
                }
            }))
        }
        Adt122::C1(v2) => {
            Ok::<Adt116, Adt110>(Adt116::C0(match v0.clone() {
                Adt124::C0(h843, _, _, _) => {
                    h843
                }
            }, v2, match v0.clone() {
                Adt124::C0(_, _, h844, _) => {
                    h844
                }
            }, match v0 {
                Adt124::C0(_, _, _, h845) => {
                    h845
                }
            }))
        }
    }
}

pub fn f203(v0: Option<Adt93>, v1: List<Str>, v2: Adt92, v3: Result<Adt16, Adt111>) -> R<Adt123> {
    let m846 = v3;
    match m846 {
        Ok(v5) => {
            Ok(f252(v1, v2.clone(), v5.clone(), f253(v0, f153(match v2.clone() {
                Adt92::C0(_, h847, _) => {
                    h847
                }
            }), match v2.clone() {
                Adt92::C0(_, h848, _) => {
                    h848
                }
            }, match v5 {
                Adt16::C0(h849, _, _, _, _, _) => {
                    h849
                }
            }, f254(match v2 {
                Adt92::C0(_, _, h850) => {
                    h850
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt123::C1(Adt110::C0(v4)))
        }
    }
}

pub fn f204(v0: Option<Adt5>) -> Result<Adt16, Adt111> {
    let m851 = v0;
    match m851 {
        None => {
            Err::<Adt16, Adt111>(Adt111::C0)
        }
        Some(v1) => {
            let m852 = f89(Str::lit("_meta"), v1);
            match m852 {
                Adt114::C0 => {
                    Err::<Adt16, Adt111>(Adt111::C1)
                }
                Adt114::C1(v2) => {
                    let m853 = f221(v2);
                    match m853 {
                        None => {
                            Err::<Adt16, Adt111>(Adt111::C4)
                        }
                        Some(v3) => {
                            f255(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f205(p858: Bytes, p859: List<u64>) -> Bytes {
    let m857 = (p858, p859);
    {
        let mut m857 = m857;
        loop {
            let step = {
                let (v0, v1) = m857;
                let m854 = v1;
                match m854.uncons() {
                    None => {
                        Err::<(Bytes, List<u64>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<u64>), Bytes>(({
                            let a855 = v0;
                            let a856 = f256(v2);
                            append_bytes(a855, a856)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m857 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206() -> Bytes {
    Bytes::lit(&[])
}

pub fn f207(p866: List<u64>, p867: List<u64>) -> R<bool> {
    let m865 = (p866, p867);
    {
        let mut m865 = m865;
        loop {
            let step = {
                let (v0, v1) = m865;
                let m860 = v0;
                match m860.uncons() {
                    None => {
                        let m861 = v1;
                        match m861.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m862 = v1;
                        match m862.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a863 = f257(v4)?;
                                let a864 = f257(v6)?;
                                if nat_eq(a863, a864) {
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
                Ok(next) => { m865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt119, v1: List<u64>) -> Adt119 {
    let m868 = v0;
    match m868 {
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

pub fn f209(v0: List<u64>) -> List<u64> {
    f215(f258(f215(f258(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f210(p871: List<u64>) -> bool {
    let m870 = p871;
    {
        let mut m870 = m870;
        loop {
            let step = {
                let v0 = m870;
                let m869 = v0;
                match m869.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f259(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f211(v0: Adt117) -> Adt125 {
    let m872 = v0;
    match m872 {
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

pub fn f212(p878: List<u64>, p879: List<u64>) -> Option<List<u64>> {
    let m877 = (p878, p879);
    {
        let mut m877 = m877;
        loop {
            let step = {
                let (v0, v1) = m877;
                let m873 = v0;
                match m873.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m874 = v1;
                        match m874.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a875 = v4;
                                let a876 = v2;
                                if nat_eq(a875, a876) {
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
                Ok(next) => { m877 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f214() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f215(p882: List<u64>, p883: List<u64>) -> List<u64> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v0, v1) = m881;
                let m880 = v0;
                match m880.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f216(p892: u64, p893: List<u64>, p894: List<u64>) -> R<Option<List<u64>>> {
    let m891 = (p892, (p893, p894));
    {
        let mut m891 = m891;
        loop {
            let step = {
                let (v0, (v1, v2)) = m891;
                let m884 = v0;
                if m884 == 0 {
                    let m885 = v1;
                    match m885.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m884 - 1;
                    let m886 = v1;
                    match m886.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m887 = v7;
                            match m887.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m888 = v9;
                                    match m888.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m889 = v11;
                                            match m889.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m890 = f260(v6, v8, v10, v12, f261(v13.clone()))?;
                                                    match m890 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f98(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Adt12, v1: Option<Str>) -> bool {
    let m895 = v1;
    match m895 {
        None => {
            false
        }
        Some(v2) => {
            let a897 = match v0 {
                Adt12::C0(h896, _, _, _) => {
                    h896
                }
            };
            let a898 = v2;
            equal(a897, a898)
        }
    }
}

pub fn f218(v0: List<Adt84>, v1: Str) -> R<Adt97> {
    let m901 = f113(f109(v0, {
        let a899 = f262();
        let a900 = f263(v1)?;
        append_list(a899, a900)
    })?)?;
    match m901 {
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
            let m902 = f150(v2);
            match m902 {
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

pub fn f219(v0: i64) -> bool {
    let a909 = {
        let m905 = {
            let a903 = v0;
            let a904 = 0i64;
            compare(a903, a904)
        };
        if match m905 {
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
            let m908 = {
                let a906 = 0i64;
                let a907 = v0;
                compare(a906, a907)
            };
            match m908 {
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
    bool_not(a909)
}

pub fn f220(v0: i64, v1: i64) -> R<i64> {
    let a911 = v0;
    let a912 = {
        let a910 = v1;
        int_neg(a910)?
    };
    int_sub(a911, a912)
}

pub fn f221(v0: Adt6) -> Option<Adt5> {
    let m913 = v0;
    match m913 {
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

pub fn f222() -> u64 {
    512u64
}

pub fn f223(v0: Adt100, v1: u64, v2: Str) -> Adt115 {
    let m915 = match v0.clone() {
        Adt100::C0(_, h914, _) => {
            h914
        }
    };
    match m915.uncons() {
        None => {
            f134(Adt88::C2(v1))
        }
        Some((v3, v4)) => {
            let m916 = v3;
            match m916 {
                Adt105::C0(_) => {
                    f134(Adt88::C2(v1))
                }
                Adt105::C1(v6, _) => {
                    Adt115::C1(Adt100::C0(Adt101::C5, List::cons(Adt105::C1(v6, Some(v2)), v4), match v0 {
                        Adt100::C0(_, _, h917) => {
                            h917
                        }
                    }))
                }
            }
        }
    }
}

pub fn f224(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a918 = v0;
    let a919 = 128u64;
    if nat_lt(a918, a919) {
        Ok(List::cons(v0, v1))
    } else {
        let a920 = v0;
        let a921 = 2048u64;
        if nat_lt(a920, a921) {
            Ok(List::cons({
                let a922 = 128u64;
                let a923 = f264(v0, 64u64);
                nat_add(a922, a923)?
            }, List::cons({
                let a924 = 192u64;
                let a925 = f265(v0, 64u64);
                nat_add(a924, a925)?
            }, v1)))
        } else {
            let a926 = v0;
            let a927 = 65536u64;
            if nat_lt(a926, a927) {
                Ok(List::cons({
                    let a928 = 128u64;
                    let a929 = f264(v0, 64u64);
                    nat_add(a928, a929)?
                }, List::cons({
                    let a930 = 128u64;
                    let a931 = f264(f265(v0, 64u64), 64u64);
                    nat_add(a930, a931)?
                }, List::cons({
                    let a932 = 224u64;
                    let a933 = f265(v0, 4096u64);
                    nat_add(a932, a933)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a934 = 128u64;
                    let a935 = f264(v0, 64u64);
                    nat_add(a934, a935)?
                }, List::cons({
                    let a936 = 128u64;
                    let a937 = f264(f265(v0, 64u64), 64u64);
                    nat_add(a936, a937)?
                }, List::cons({
                    let a938 = 128u64;
                    let a939 = f264(f265(v0, 4096u64), 64u64);
                    nat_add(a938, a939)?
                }, List::cons({
                    let a940 = 240u64;
                    let a941 = f265(v0, 262144u64);
                    nat_add(a940, a941)?
                }, v1)))))
            }
        }
    }
}

pub fn f225() -> u64 {
    18u64
}

pub fn f226(v0: u64) -> List<u64> {
    f175(v0)
}

pub fn f227(v0: Adt7) -> R<bool> {
    let a959 = {
        let m945 = {
            let a943 = f266();
            let a944 = match v0.clone() {
                Adt7::C0(h942, _) => {
                    h942
                }
            };
            compare(a943, a944)
        };
        if match m945 {
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
                let a948 = match v0.clone() {
                    Adt7::C0(h946, _) => {
                        h946
                    }
                };
                let a949 = {
                    let a947 = f266();
                    int_neg(a947)?
                };
                compare(a948, a949)
            };
            if match m950 {
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
                let m954 = {
                    let a952 = 9999i64;
                    let a953 = match v0.clone() {
                        Adt7::C0(_, h951) => {
                            h951
                        }
                    };
                    compare(a952, a953)
                };
                if match m954 {
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
                    let m958 = {
                        let a956 = match v0 {
                            Adt7::C0(_, h955) => {
                                h955
                            }
                        };
                        let a957 = -9999i64;
                        compare(a956, a957)
                    };
                    match m958 {
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
    Ok(bool_not(a959))
}

pub fn f228(v0: Adt7) -> R<List<u64>> {
    let m963 = {
        let a961 = match v0.clone() {
            Adt7::C0(_, h960) => {
                h960
            }
        };
        let a962 = 0i64;
        compare(a961, a962)
    };
    if if match m963 {
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
        let m967 = {
            let a965 = 0i64;
            let a966 = match v0.clone() {
                Adt7::C0(_, h964) => {
                    h964
                }
            };
            compare(a965, a966)
        };
        match m967 {
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
        let a970 = f267(match v0.clone() {
            Adt7::C0(h968, _) => {
                h968
            }
        })?;
        let a971 = List::cons(101u64, f267(match v0 {
            Adt7::C0(_, h969) => {
                h969
            }
        })?);
        Ok(append_list(a970, a971))
    } else {
        f267(match v0 {
            Adt7::C0(h972, _) => {
                h972
            }
        })
    }
}

pub fn f229(v0: u64, v1: Adt106, v2: List<Adt106>) -> List<Adt106> {
    List::cons(Adt106::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f230(p975: List<u64>, p976: List<u64>) -> R<List<u64>> {
    let m974 = (p975, p976);
    {
        let mut m974 = m974;
        loop {
            let step = {
                let (v0, v1) = m974;
                let m973 = v0;
                match m973.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f98(f268(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m974 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: List<Str>) -> Adt6 {
    Adt6::C4(f53(v0))
}

pub fn f232(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a994 = f243(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h977, _, _, _, _, _, _) => {
                h977
            }
        });
        let a995 = {
            let a992 = f243(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h978, _, _, _, _, _) => {
                    h978
                }
            });
            let a993 = {
                let a990 = f243(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h979, _, _, _, _) => {
                        h979
                    }
                });
                let a991 = {
                    let a988 = f269(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h980, _, _, _) => {
                            h980
                        }
                    });
                    let a989 = {
                        let a986 = f270(match v0.clone() {
                            Adt4::C0(_, _, _, _, h981, _, _) => {
                                h981
                            }
                        });
                        let a987 = {
                            let a984 = f269(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h982, _) => {
                                    h982
                                }
                            });
                            let a985 = f243(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h983) => {
                                    h983
                                }
                            });
                            append_list(a984, a985)
                        };
                        append_list(a986, a987)
                    };
                    append_list(a988, a989)
                };
                append_list(a990, a991)
            };
            append_list(a992, a993)
        };
        append_list(a994, a995)
    })
}

pub fn f233(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m996 = v1;
    match m996 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f234(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f271(match v0.clone() {
        Adt10::C0(h997, _) => {
            h997
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f272(match v0 {
        Adt10::C0(_, h998) => {
            h998
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f235(v0: List<Adt35>) -> List<Adt6> {
    f72(f273(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f236(v0: List<Adt38>) -> Option<List<Adt6>> {
    let m999 = f274(v0, List::<Adt6>::nil());
    match m999 {
        None => {
            None::<List<Adt6>>
        }
        Some(v1) => {
            Some(f72(v1, List::<Adt6>::nil()))
        }
    }
}

pub fn f237(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1000 = v1;
    match m1000 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f238(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1001 = v1;
    match m1001 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f239() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f240(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1002, _, _, _, _, _) => {
            h1002
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1003, _, _, _) => {
            h1003
        }
    })), {
        let a1012 = f233(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1004, _, _, _, _) => {
                h1004
            }
        });
        let a1013 = {
            let a1010 = f233(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1005, _, _) => {
                    h1005
                }
            });
            let a1011 = {
                let a1008 = f233(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1006, _) => {
                        h1006
                    }
                });
                let a1009 = f275(match v0 {
                    Adt1::C0(_, _, _, _, _, h1007) => {
                        h1007
                    }
                });
                append_list(a1008, a1009)
            };
            append_list(a1010, a1011)
        };
        append_list(a1012, a1013)
    })))
}

pub fn f241() -> i64 {
    -32021i64
}

pub fn f242() -> i64 {
    -32603i64
}

pub fn f243(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1014 = v1;
    match m1014 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1015) => {
                    h1015
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f244(v0: Adt0, v1: Fn0, v2: Result<Adt14, Adt111>) -> Result<Adt108, Adt110> {
    let m1016 = v2;
    match m1016 {
        Ok(v4) => {
            f276(v0, v1, v4)
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(Adt110::C0(v3))
        }
    }
}

pub fn f245(v0: Adt117, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt111> {
    let m1017 = v0;
    match m1017 {
        Adt117::C0 => {
            f277(v1, v2)
        }
        Adt117::C1 => {
            f278(v1, v2)
        }
        Adt117::C2 => {
            f279(v1, v2)
        }
        Adt117::C3 => {
            f280(v1, v2)
        }
        Adt117::C4 => {
            f281(v1, v2)
        }
        Adt117::C5 => {
            f282(v1, v2)
        }
        Adt117::C6 => {
            f283(v1, v2)
        }
        Adt117::C7 => {
            f284(v1, v2)
        }
        Adt117::C8 => {
            f285(v1, v2)
        }
        Adt117::C9 => {
            f286(v1, v2)
        }
        Adt117::C10 => {
            f287(v1, v2)
        }
        Adt117::C11 => {
            f288(v1, v2)
        }
    }
}

pub fn f246(v0: Adt117) -> Str {
    let m1018 = v0;
    match m1018 {
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

pub fn f247(v0: Option<Adt8>) -> bool {
    let m1019 = v0;
    match m1019 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f248(v0: Option<Adt9>) -> bool {
    let m1020 = v0;
    match m1020 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f249(v0: Option<Adt5>) -> bool {
    let m1021 = v0;
    match m1021 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f250(v0: Option<Adt5>, v1: Str) -> bool {
    let m1022 = v0;
    match m1022 {
        None => {
            false
        }
        Some(v2) => {
            f122(v1, v2)
        }
    }
}

pub fn f251() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f252(v0: List<Str>, v1: Adt92, v2: Adt16, v3: Option<Adt94>) -> Adt123 {
    let m1023 = v3;
    match m1023 {
        None => {
            f289(v1, v2.clone(), f290(match v2 {
                Adt16::C0(h1024, _, _, _, _, _) => {
                    h1024
                }
            }, v0))
        }
        Some(v4) => {
            Adt123::C1(Adt110::C2(v4))
        }
    }
}

pub fn f253(v0: Option<Adt93>, v1: Adt125, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt94>> {
    let m1025 = v0;
    match m1025 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v5) => {
            let m1027 = match v5.clone() {
                Adt93::C0(_, _, _, h1026, _) => {
                    h1026
                }
            };
            match m1027 {
                None => {
                    f291(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f254(v0: Option<Adt5>) -> Adt5 {
    let m1028 = v0;
    match m1028 {
        None => {
            f54()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f255(v0: Adt5) -> Result<Adt16, Adt111> {
    let m1029 = f292(v0.clone());
    match m1029 {
        Adt128::C0 => {
            f293(v0.clone(), f89(f294(), v0.clone()), f89(f295(), v0))
        }
        Adt128::C1 => {
            Err::<Adt16, Adt111>(Adt111::C2)
        }
        Adt128::C2 => {
            Err::<Adt16, Adt111>(Adt111::C3)
        }
    }
}

pub fn f256(v0: u64) -> Bytes {
    let m1033 = {
        let a1030 = f296();
        let a1031 = v0;
        let a1032 = 1u64;
        slice_bytes(a1030, a1031, a1032)
    };
    match m1033 {
        None => {
            f206()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f257(v0: u64) -> R<u64> {
    let a1034 = 64u64;
    let a1035 = v0;
    if if nat_lt(a1034, a1035) {
        let a1036 = v0;
        let a1037 = 91u64;
        nat_lt(a1036, a1037)
    } else {
        false
    } {
        let a1038 = v0;
        let a1039 = 32u64;
        nat_add(a1038, a1039)
    } else {
        Ok(v0)
    }
}

pub fn f258(p1042: List<u64>) -> List<u64> {
    let m1041 = p1042;
    {
        let mut m1041 = m1041;
        loop {
            let step = {
                let v0 = m1041;
                let m1040 = v0;
                match m1040.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f297(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: u64) -> bool {
    let a1043 = v0;
    let a1044 = 9u64;
    if nat_eq(a1043, a1044) {
        true
    } else {
        let a1045 = 31u64;
        let a1046 = v0;
        if nat_lt(a1045, a1046) {
            let a1047 = v0;
            let a1048 = 127u64;
            nat_lt(a1047, a1048)
        } else {
            false
        }
    }
}

pub fn f260(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1049 = f298(v0)?;
    match m1049 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1050 = f298(v1)?;
            match m1050 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1051 = v2;
                    let a1052 = 61u64;
                    if nat_eq(a1051, a1052) {
                        f299(v5, v6, v4, v3)
                    } else {
                        let m1053 = f298(v2)?;
                        match m1053 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1054 = v3;
                                let a1055 = 61u64;
                                if nat_eq(a1054, a1055) {
                                    f300(v5, v6, v7, v4)
                                } else {
                                    let m1056 = f298(v3)?;
                                    match m1056 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f301(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f261(v0: List<u64>) -> bool {
    let m1057 = v0;
    match m1057.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f262() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f263(v0: Str) -> R<List<u64>> {
    f49({
        let a1058 = v0;
        utf8_encode(a1058)
    })
}

pub fn f264(v0: u64, v1: u64) -> u64 {
    let a1059 = v0;
    let a1060 = v1;
    let a1061 = 0u64;
    nat_rem(a1059, a1060, a1061)
}

pub fn f265(v0: u64, v1: u64) -> u64 {
    let a1062 = v0;
    let a1063 = v1;
    let a1064 = 0u64;
    nat_quot(a1062, a1063, a1064)
}

pub fn f266() -> i64 {
    999999999999999999i64
}

pub fn f267(v0: i64) -> R<List<u64>> {
    let m1067 = {
        let a1065 = v0;
        let a1066 = 0i64;
        compare(a1065, a1066)
    };
    if match m1067 {
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
        Ok(List::cons(45u64, f302(20u64, {
            let a1068 = v0;
            int_neg(a1068)?
        }, List::<u64>::nil())?))
    } else {
        f302(20u64, v0, List::<u64>::nil())
    }
}

pub fn f268(v0: u64) -> R<List<u64>> {
    let a1069 = v0;
    let a1070 = 34u64;
    if nat_eq(a1069, a1070) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1071 = v0;
        let a1072 = 92u64;
        if nat_eq(a1071, a1072) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1073 = v0;
            let a1074 = 8u64;
            if nat_eq(a1073, a1074) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1075 = v0;
                let a1076 = 9u64;
                if nat_eq(a1075, a1076) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1077 = v0;
                    let a1078 = 10u64;
                    if nat_eq(a1077, a1078) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1079 = v0;
                        let a1080 = 12u64;
                        if nat_eq(a1079, a1080) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1081 = v0;
                            let a1082 = 13u64;
                            if nat_eq(a1081, a1082) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1083 = v0;
                                let a1084 = 32u64;
                                if nat_lt(a1083, a1084) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f303(f265(v0, 16u64))?, List::cons(f303(f264(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f269(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1085 = v1;
    match m1085 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f304(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f270(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1086 = v0;
    match m1086 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1089 = f238(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1087, _) => {
                        h1087
                    }
                });
                let a1090 = f238(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1088) => {
                        h1088
                    }
                });
                append_list(a1089, a1090)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f271(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f305(v0, 0i64)?, 0i64)))
}

pub fn f272(v0: Adt11) -> Str {
    let m1091 = v0;
    match m1091 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f273(p1094: List<Adt35>, p1095: List<Adt6>) -> List<Adt6> {
    let m1093 = (p1094, p1095);
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let (v0, v1) = m1093;
                let m1092 = v0;
                match m1092.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f306(v2), v1)))
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

pub fn f274(p1099: List<Adt38>, p1100: List<Adt6>) -> Option<List<Adt6>> {
    let m1098 = (p1099, p1100);
    {
        let mut m1098 = m1098;
        loop {
            let step = {
                let (v0, v1) = m1098;
                let m1096 = v0;
                match m1096.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1097 = f307(v2);
                        match m1097 {
                            None => {
                                Err::<(List<Adt38>, List<Adt6>), Option<List<Adt6>>>(None::<List<Adt6>>)
                            }
                            Some(v4) => {
                                Ok::<(List<Adt38>, List<Adt6>), Option<List<Adt6>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1101 = v0;
    match m1101 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f308(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f276(v0: Adt0, v1: Fn0, v2: Adt14) -> Result<Adt108, Adt110> {
    if f309(f310(v2.clone())) {
        Ok::<Adt108, Adt110>(f311(v0.clone(), Adt32::C0(f312(v0))))
    } else {
        f313(v0, v2.clone(), {
            let c1102: Fn0 = v1;
            c1102.apply(v2)
        })
    }
}

pub fn f277(v0: Adt16, _: Adt5) -> Result<Adt14, Adt111> {
    Ok::<Adt14, Adt111>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f278(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1103 = f314(Str::lit("cursor"), v1);
    match m1103 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f279(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1104 = f315(Str::lit("name"), v1.clone());
    match m1104 {
        Ok(v2) => {
            let m1105 = f316(Str::lit("arguments"), v1.clone());
            match m1105 {
                Ok(v3) => {
                    let m1106 = f316(Str::lit("inputResponses"), v1.clone());
                    match m1106 {
                        Ok(v4) => {
                            let m1107 = f314(Str::lit("requestState"), v1);
                            match m1107 {
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

pub fn f280(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1108 = f314(Str::lit("cursor"), v1);
    match m1108 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f281(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1109 = f315(Str::lit("uri"), v1.clone());
    match m1109 {
        Ok(v2) => {
            let m1110 = f316(Str::lit("inputResponses"), v1.clone());
            match m1110 {
                Ok(v3) => {
                    let m1111 = f314(Str::lit("requestState"), v1);
                    match m1111 {
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

pub fn f282(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1112 = f314(Str::lit("cursor"), v1);
    match m1112 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f283(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1113 = f314(Str::lit("cursor"), v1);
    match m1113 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f284(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1114 = f315(Str::lit("name"), v1.clone());
    match m1114 {
        Ok(v2) => {
            let m1115 = f317(Str::lit("arguments"), v1.clone());
            match m1115 {
                Ok(v3) => {
                    let m1116 = f316(Str::lit("inputResponses"), v1.clone());
                    match m1116 {
                        Ok(v4) => {
                            let m1117 = f314(Str::lit("requestState"), v1);
                            match m1117 {
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

pub fn f285(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1118 = f318(Str::lit("ref"), v1.clone());
    match m1118 {
        Ok(v2) => {
            let m1119 = f319(v2);
            match m1119 {
                Ok(v3) => {
                    let m1120 = f318(Str::lit("argument"), v1.clone());
                    match m1120 {
                        Ok(v4) => {
                            let m1121 = f320(v4);
                            match m1121 {
                                Ok(v5) => {
                                    let m1122 = f321(v1);
                                    match m1122 {
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

pub fn f286(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1123 = f315(Str::lit("taskId"), v1);
    match m1123 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f287(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1124 = f315(Str::lit("taskId"), v1.clone());
    match m1124 {
        Ok(v2) => {
            let m1125 = f318(Str::lit("inputResponses"), v1);
            match m1125 {
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

pub fn f288(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt111> {
    let m1126 = f315(Str::lit("taskId"), v1);
    match m1126 {
        Ok(v2) => {
            Ok::<Adt14, Adt111>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt111>(v3)
        }
    }
}

pub fn f289(v0: Adt92, v1: Adt16, v2: Option<Adt110>) -> Adt123 {
    let m1127 = v2;
    match m1127 {
        None => {
            Adt123::C0(Adt124::C0(match v0.clone() {
                Adt92::C0(h1128, _, _) => {
                    h1128
                }
            }, match v0.clone() {
                Adt92::C0(_, h1129, _) => {
                    h1129
                }
            }, v1, f254(match v0 {
                Adt92::C0(_, _, h1130) => {
                    h1130
                }
            })))
        }
        Some(v3) => {
            Adt123::C1(v3)
        }
    }
}

pub fn f290(v0: Str, v1: List<Str>) -> Option<Adt110> {
    f322(f323(v0.clone(), v1.clone()), v0, v1)
}

pub fn f291(v0: Adt93, v1: Adt125, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt94>> {
    let m1132 = match v0.clone() {
        Adt93::C0(h1131, _, _, _, _) => {
            h1131
        }
    };
    match m1132 {
        None => {
            Ok(Some(Adt94::C0))
        }
        Some(v5) => {
            let a1133 = v5;
            let a1134 = v3;
            if equal(a1133, a1134) {
                let m1136 = match v0.clone() {
                    Adt93::C0(_, h1135, _, _, _) => {
                        h1135
                    }
                };
                match m1136 {
                    None => {
                        Ok(Some(Adt94::C1))
                    }
                    Some(v6) => {
                        let a1137 = v6;
                        let a1138 = v2;
                        if equal(a1137, a1138) {
                            f324(f325(match v0.clone() {
                                Adt93::C0(_, _, h1139, _, _) => {
                                    h1139
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt93::C0(_, _, _, _, h1140) => {
                                    h1140
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

pub fn f292(v0: Adt5) -> Adt128 {
    f326(f89(f294(), v0.clone()), f89(f295(), v0))
}

pub fn f293(v0: Adt5, v1: Adt114, v2: Adt114) -> Result<Adt16, Adt111> {
    let m1141 = v1;
    match m1141 {
        Adt114::C0 => {
            Err::<Adt16, Adt111>(Adt111::C2)
        }
        Adt114::C1(v3) => {
            let m1142 = v2;
            match m1142 {
                Adt114::C0 => {
                    Err::<Adt16, Adt111>(Adt111::C3)
                }
                Adt114::C1(v4) => {
                    f327(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f294() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f295() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f296() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f297(v0: u64) -> bool {
    let a1143 = v0;
    let a1144 = 9u64;
    if nat_eq(a1143, a1144) {
        true
    } else {
        let a1145 = v0;
        let a1146 = 32u64;
        nat_eq(a1145, a1146)
    }
}

pub fn f298(v0: u64) -> R<Option<u64>> {
    let a1147 = 64u64;
    let a1148 = v0;
    if if nat_lt(a1147, a1148) {
        let a1149 = v0;
        let a1150 = 91u64;
        nat_lt(a1149, a1150)
    } else {
        false
    } {
        Ok(Some({
            let a1151 = v0;
            let a1152 = 65u64;
            nat_sub(a1151, a1152)
        }))
    } else {
        let a1153 = 96u64;
        let a1154 = v0;
        if if nat_lt(a1153, a1154) {
            let a1155 = v0;
            let a1156 = 123u64;
            nat_lt(a1155, a1156)
        } else {
            false
        } {
            Ok(Some({
                let a1157 = v0;
                let a1158 = 71u64;
                nat_sub(a1157, a1158)
            }))
        } else {
            let a1159 = 47u64;
            let a1160 = v0;
            if if nat_lt(a1159, a1160) {
                let a1161 = v0;
                let a1162 = 58u64;
                nat_lt(a1161, a1162)
            } else {
                false
            } {
                Ok(Some({
                    let a1163 = v0;
                    let a1164 = 4u64;
                    nat_add(a1163, a1164)?
                }))
            } else {
                let a1165 = v0;
                let a1166 = 43u64;
                if nat_eq(a1165, a1166) {
                    Ok(Some(62u64))
                } else {
                    let a1167 = v0;
                    let a1168 = 47u64;
                    if nat_eq(a1167, a1168) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f299(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1169 = v3;
        let a1170 = 61u64;
        if nat_eq(a1169, a1170) {
            let a1171 = f328(v1, 16u64);
            let a1172 = 0u64;
            nat_eq(a1171, a1172)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f329(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f300(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1173 = f328(v2, 4u64);
        let a1174 = 0u64;
        nat_eq(a1173, a1174)
    } else {
        false
    } {
        Ok(Some(f330(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f301(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1177 = {
            let a1175 = v0;
            let a1176 = 4u64;
            nat_mul(a1175, a1176)?
        };
        let a1178 = f331(v1, 16u64);
        nat_add(a1177, a1178)?
    }, List::cons({
        let a1181 = {
            let a1179 = f328(v1, 16u64);
            let a1180 = 16u64;
            nat_mul(a1179, a1180)?
        };
        let a1182 = f331(v2, 4u64);
        nat_add(a1181, a1182)?
    }, List::cons({
        let a1185 = {
            let a1183 = f328(v2, 4u64);
            let a1184 = 64u64;
            nat_mul(a1183, a1184)?
        };
        let a1186 = v3;
        nat_add(a1185, a1186)?
    }, List::<u64>::nil()))))
}

pub fn f302(p1198: u64, p1199: i64, p1200: List<u64>) -> R<List<u64>> {
    let m1197 = (p1198, (p1199, p1200));
    {
        let mut m1197 = m1197;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1197;
                let m1187 = v0;
                if m1187 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1187 - 1;
                    let v4: i64 = {
                        let a1188 = v1;
                        let a1189 = 10i64;
                        let a1190 = 0i64;
                        int_quot(a1188, a1189, a1190)?
                    };
                    let v5: i64 = {
                        let a1191 = v1;
                        let a1192 = 10i64;
                        let a1193 = 0i64;
                        int_rem(a1191, a1192, a1193)
                    };
                    let m1196 = {
                        let a1194 = v4;
                        let a1195 = 1i64;
                        compare(a1194, a1195)
                    };
                    if match m1196 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f332(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f332(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1197 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f303(v0: u64) -> R<u64> {
    let a1201 = v0;
    let a1202 = 10u64;
    if nat_lt(a1201, a1202) {
        let a1203 = 48u64;
        let a1204 = v0;
        nat_add(a1203, a1204)
    } else {
        let a1205 = 87u64;
        let a1206 = v0;
        nat_add(a1205, a1206)
    }
}

pub fn f304(v0: Adt8) -> Adt6 {
    Adt6::C5(f238(Str::lit("listChanged"), match v0 {
        Adt8::C0(h1207) => {
            h1207
        }
    }))
}

pub fn f305(p1212: u64, p1213: i64) -> R<i64> {
    let m1211 = (p1212, p1213);
    {
        let mut m1211 = m1211;
        loop {
            let step = {
                let (v0, v1) = m1211;
                let m1208 = v0;
                if m1208 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m1208 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a1209 = v1;
                        let a1210 = -1i64;
                        int_sub(a1209, a1210)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h1214, _, _, _, _, _, _, _) => {
            h1214
        }
    })), {
        let a1232 = f233(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h1215, _, _, _, _, _, _) => {
                h1215
            }
        });
        let a1233 = {
            let a1230 = f275(match v0.clone() {
                Adt35::C0(_, _, h1216, _, _, _, _, _) => {
                    h1216
                }
            });
            let a1231 = {
                let a1228 = f233(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h1217, _, _, _, _) => {
                        h1217
                    }
                });
                let a1229 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m1220 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h1219, _, _, _) => {
                            h1219
                        }
                    };
                    match m1220 {
                        Adt5::C0(h1218) => {
                            h1218
                        }
                    }
                })), {
                    let a1226 = f243(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h1221, _, _) => {
                            h1221
                        }
                    });
                    let a1227 = {
                        let a1224 = f333(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h1222, _) => {
                                h1222
                            }
                        });
                        let a1225 = f334(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h1223) => {
                                h1223
                            }
                        });
                        append_list(a1224, a1225)
                    };
                    append_list(a1226, a1227)
                });
                append_list(a1228, a1229)
            };
            append_list(a1230, a1231)
        };
        append_list(a1232, a1233)
    }))
}

pub fn f307(v0: Adt38) -> Option<Adt6> {
    let m1234 = v0;
    match m1234 {
        Adt38::C0(v1) => {
            Some(f335(v1))
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

pub fn f308(v0: List<Adt2>) -> List<Adt6> {
    f72(f336(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f309(v0: Adt117) -> bool {
    let m1235 = v0;
    match m1235 {
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

pub fn f310(v0: Adt14) -> Adt117 {
    let m1236 = v0;
    match m1236 {
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

pub fn f311(v0: Adt0, v1: Adt32) -> Adt108 {
    Adt108::C0(v1, Adt109::C0(Some(match v0 {
        Adt0::C0(h1237, _, _, _, _) => {
            h1237
        }
    }), None::<Adt5>))
}

pub fn f312(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h1238, _) => {
            h1238
        }
    }, f87(), match v0.clone() {
        Adt0::C0(_, _, h1239, _, _) => {
            h1239
        }
    }, match v0 {
        Adt0::C0(_, h1240, _, _, _) => {
            h1240
        }
    })
}

pub fn f313(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt82>) -> Result<Adt108, Adt110> {
    let m1241 = v2;
    match m1241 {
        Ok(v4) => {
            if f337(f310(v1.clone()), v4.clone()) {
                f338(v0, v1, v4)
            } else {
                Err::<Adt108, Adt110>(Adt110::C4(f339()))
            }
        }
        Err(v3) => {
            Err::<Adt108, Adt110>(Adt110::C4(v3))
        }
    }
}

pub fn f314(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt111> {
    let m1242 = f89(v0.clone(), v1);
    match m1242 {
        Adt114::C0 => {
            Ok::<Option<Str>, Adt111>(None::<Str>)
        }
        Adt114::C1(v2) => {
            let m1243 = f118(v2);
            match m1243 {
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

pub fn f315(v0: Str, v1: Adt5) -> Result<Str, Adt111> {
    let m1244 = f89(v0.clone(), v1);
    match m1244 {
        Adt114::C0 => {
            Err::<Str, Adt111>(Adt111::C7(v0))
        }
        Adt114::C1(v2) => {
            let m1245 = f118(v2);
            match m1245 {
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

pub fn f316(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt111> {
    let m1246 = f89(v0.clone(), v1);
    match m1246 {
        Adt114::C0 => {
            Ok::<Option<Adt5>, Adt111>(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m1247 = f221(v2);
            match m1247 {
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

pub fn f317(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt111> {
    let m1248 = f89(v0.clone(), v1);
    match m1248 {
        Adt114::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt111>(None::<List<(Str, Str)>>)
        }
        Adt114::C1(v2) => {
            let m1249 = f221(v2);
            match m1249 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt111>(Adt111::C8(v0))
                }
                Some(v3) => {
                    let m1251 = f340(v0, match v3 {
                        Adt5::C0(h1250) => {
                            h1250
                        }
                    });
                    match m1251 {
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

pub fn f318(v0: Str, v1: Adt5) -> Result<Adt5, Adt111> {
    let m1252 = f89(v0.clone(), v1);
    match m1252 {
        Adt114::C0 => {
            Err::<Adt5, Adt111>(Adt111::C7(v0))
        }
        Adt114::C1(v2) => {
            let m1253 = f221(v2);
            match m1253 {
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

pub fn f319(v0: Adt5) -> Result<Adt25, Adt111> {
    let m1254 = f315(Str::lit("type"), v0.clone());
    match m1254 {
        Ok(v1) => {
            let a1255 = v1.clone();
            let a1256 = Str::lit("ref/prompt");
            if equal(a1255, a1256) {
                let m1257 = f315(Str::lit("name"), v0.clone());
                match m1257 {
                    Ok(v2) => {
                        let m1258 = f314(Str::lit("title"), v0);
                        match m1258 {
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
                let a1259 = v1;
                let a1260 = Str::lit("ref/resource");
                if equal(a1259, a1260) {
                    let m1261 = f315(Str::lit("uri"), v0);
                    match m1261 {
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

pub fn f320(v0: Adt5) -> Result<Adt27, Adt111> {
    let m1262 = f315(Str::lit("name"), v0.clone());
    match m1262 {
        Ok(v1) => {
            let m1263 = f315(Str::lit("value"), v0);
            match m1263 {
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

pub fn f321(v0: Adt5) -> Result<Option<Adt28>, Adt111> {
    let m1264 = f89(Str::lit("context"), v0);
    match m1264 {
        Adt114::C0 => {
            Ok::<Option<Adt28>, Adt111>(None::<Adt28>)
        }
        Adt114::C1(v1) => {
            let m1265 = f221(v1);
            match m1265 {
                None => {
                    Err::<Option<Adt28>, Adt111>(Adt111::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m1266 = f317(Str::lit("arguments"), v2);
                    match m1266 {
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

pub fn f322(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt110> {
    if v0 {
        None::<Adt110>
    } else {
        Some(Adt110::C1(v1, v2))
    }
}

pub fn f323(v0: Str, p1271: List<Str>) -> bool {
    let m1270 = p1271;
    {
        let mut m1270 = m1270;
        loop {
            let step = {
                let v1 = m1270;
                let m1267 = v1;
                match m1267.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1268 = v2;
                        let a1269 = v0.clone();
                        if equal(a1268, a1269) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Option<Adt94>, v1: List<Adt96>, v2: Adt5) -> R<Option<Adt94>> {
    let m1272 = v0;
    match m1272 {
        None => {
            f341(v1, f342(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f325(v0: Option<Str>, v1: Adt125, v2: Adt5) -> Option<Adt94> {
    let m1273 = v1.clone();
    match m1273 {
        Adt125::C0 => {
            None::<Adt94>
        }
        Adt125::C1 => {
            f343(v0, f344(v1, v2))
        }
        Adt125::C2 => {
            f343(v0, f344(v1, v2))
        }
        Adt125::C3 => {
            f343(v0, f344(v1, v2))
        }
    }
}

pub fn f326(v0: Adt114, v1: Adt114) -> Adt128 {
    let m1274 = v0;
    match m1274 {
        Adt114::C0 => {
            Adt128::C1
        }
        Adt114::C1(_) => {
            let m1275 = v1;
            match m1275 {
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

pub fn f327(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt111> {
    let m1276 = f118(v1);
    match m1276 {
        None => {
            Err::<Adt16, Adt111>(Adt111::C5)
        }
        Some(v3) => {
            let m1277 = f345(v2);
            match m1277 {
                None => {
                    Err::<Adt16, Adt111>(Adt111::C6)
                }
                Some(v4) => {
                    let m1278 = f316(f346(), v0.clone());
                    match m1278 {
                        Ok(v6) => {
                            let m1279 = f347(v0.clone());
                            match m1279 {
                                Ok(v8) => {
                                    let m1280 = f348(v0.clone());
                                    match m1280 {
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

pub fn f328(v0: u64, v1: u64) -> u64 {
    let a1281 = v0;
    let a1282 = v1;
    let a1283 = 0u64;
    nat_rem(a1281, a1282, a1283)
}

pub fn f329(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1286 = {
            let a1284 = v0;
            let a1285 = 4u64;
            nat_mul(a1284, a1285)?
        };
        let a1287 = f331(v1, 16u64);
        nat_add(a1286, a1287)?
    }, List::<u64>::nil()))
}

pub fn f330(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1290 = {
            let a1288 = v0;
            let a1289 = 4u64;
            nat_mul(a1288, a1289)?
        };
        let a1291 = f331(v1, 16u64);
        nat_add(a1290, a1291)?
    }, List::cons({
        let a1294 = {
            let a1292 = f328(v1, 16u64);
            let a1293 = 16u64;
            nat_mul(a1292, a1293)?
        };
        let a1295 = f331(v2, 4u64);
        nat_add(a1294, a1295)?
    }, List::<u64>::nil())))
}

pub fn f331(v0: u64, v1: u64) -> u64 {
    let a1296 = v0;
    let a1297 = v1;
    let a1298 = 0u64;
    nat_quot(a1296, a1297, a1298)
}

pub fn f332(v0: i64) -> u64 {
    let m1301 = {
        let a1299 = v0;
        let a1300 = 1i64;
        compare(a1299, a1300)
    };
    if match m1301 {
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
        let m1304 = {
            let a1302 = v0;
            let a1303 = 2i64;
            compare(a1302, a1303)
        };
        if match m1304 {
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
            let m1307 = {
                let a1305 = v0;
                let a1306 = 3i64;
                compare(a1305, a1306)
            };
            if match m1307 {
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
                let m1310 = {
                    let a1308 = v0;
                    let a1309 = 4i64;
                    compare(a1308, a1309)
                };
                if match m1310 {
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
                    let m1313 = {
                        let a1311 = v0;
                        let a1312 = 5i64;
                        compare(a1311, a1312)
                    };
                    if match m1313 {
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
                        let m1316 = {
                            let a1314 = v0;
                            let a1315 = 6i64;
                            compare(a1314, a1315)
                        };
                        if match m1316 {
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
                            let m1319 = {
                                let a1317 = v0;
                                let a1318 = 7i64;
                                compare(a1317, a1318)
                            };
                            if match m1319 {
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
                                let m1322 = {
                                    let a1320 = v0;
                                    let a1321 = 8i64;
                                    compare(a1320, a1321)
                                };
                                if match m1322 {
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
                                    let m1325 = {
                                        let a1323 = v0;
                                        let a1324 = 9i64;
                                        compare(a1323, a1324)
                                    };
                                    if match m1325 {
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

pub fn f333(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m1326 = v0;
    match m1326 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a1338 = f233(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h1327, _, _, _, _) => {
                        h1327
                    }
                });
                let a1339 = {
                    let a1336 = f238(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h1328, _, _, _) => {
                            h1328
                        }
                    });
                    let a1337 = {
                        let a1334 = f238(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h1329, _, _) => {
                                h1329
                            }
                        });
                        let a1335 = {
                            let a1332 = f238(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h1330, _) => {
                                    h1330
                                }
                            });
                            let a1333 = f238(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h1331) => {
                                    h1331
                                }
                            });
                            append_list(a1332, a1333)
                        };
                        append_list(a1334, a1335)
                    };
                    append_list(a1336, a1337)
                };
                append_list(a1338, a1339)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f334(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f243(Str::lit("_meta"), v0)
}

pub fn f335(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h1340, _, _) => {
            h1340
        }
    })), {
        let a1343 = f349(match v0.clone() {
            Adt39::C0(_, h1341, _) => {
                h1341
            }
        });
        let a1344 = f334(match v0 {
            Adt39::C0(_, _, h1342) => {
                h1342
            }
        });
        append_list(a1343, a1344)
    })))
}

pub fn f336(p1347: List<Adt2>, p1348: List<Adt6>) -> List<Adt6> {
    let m1346 = (p1347, p1348);
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let (v0, v1) = m1346;
                let m1345 = v0;
                match m1345.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f350(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Adt117, v1: Adt32) -> bool {
    let m1349 = v1;
    match m1349 {
        Adt32::C0(_) => {
            f351(v0, Adt117::C0)
        }
        Adt32::C1(_) => {
            f351(v0, Adt117::C1)
        }
        Adt32::C2(_) => {
            f351(v0, Adt117::C2)
        }
        Adt32::C3(_) => {
            f351(v0, Adt117::C3)
        }
        Adt32::C4(_) => {
            f351(v0, Adt117::C4)
        }
        Adt32::C5(_) => {
            f351(v0, Adt117::C5)
        }
        Adt32::C6(_) => {
            f351(v0, Adt117::C6)
        }
        Adt32::C7(_) => {
            f351(v0, Adt117::C7)
        }
        Adt32::C8(_) => {
            f351(v0, Adt117::C8)
        }
        Adt32::C9(_) => {
            f351(v0, Adt117::C9)
        }
        Adt32::C10 => {
            if f351(v0.clone(), Adt117::C10) {
                true
            } else {
                f351(v0, Adt117::C11)
            }
        }
        Adt32::C11(_) => {
            f352(v0)
        }
        Adt32::C12(_) => {
            f353(v0)
        }
    }
}

pub fn f338(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt108, Adt110> {
    let m1350 = v2.clone();
    match m1350 {
        Adt32::C0(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C3(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C4(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C10 => {
            Ok::<Adt108, Adt110>(f311(v0, v2))
        }
        Adt32::C11(v3) => {
            f354(v0, f355(v1), v3)
        }
        Adt32::C12(v4) => {
            f356(v0, f355(v1), v4)
        }
    }
}

pub fn f339() -> Adt82 {
    Adt82::C6(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f340(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt111> {
    let m1351 = f357(v0, v1, List::<(Str, Str)>::nil());
    match m1351 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt111>(f358(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt111>(v3)
        }
    }
}

pub fn f341(p1355: List<Adt96>, v1: Adt5) -> R<Option<Adt94>> {
    let m1354 = p1355;
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let v0 = m1354;
                let m1352 = v0;
                match m1352.uncons() {
                    None => {
                        Err::<List<Adt96>, R<Option<Adt94>>>(Ok(None::<Adt94>))
                    }
                    Some((v2, v3)) => {
                        let m1353 = f359(v2, v1.clone())?;
                        match m1353 {
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
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Adt5) -> Adt5 {
    let m1356 = f89(Str::lit("arguments"), v0);
    match m1356 {
        Adt114::C0 => {
            f54()
        }
        Adt114::C1(v1) => {
            let m1357 = f221(v1);
            match m1357 {
                None => {
                    f54()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f343(v0: Option<Str>, v1: Option<Str>) -> Option<Adt94> {
    let m1358 = v0;
    match m1358 {
        None => {
            Some(Adt94::C2)
        }
        Some(v2) => {
            let m1359 = v1;
            match m1359 {
                None => {
                    None::<Adt94>
                }
                Some(v3) => {
                    let a1360 = v2;
                    let a1361 = v3;
                    if equal(a1360, a1361) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C5)
                    }
                }
            }
        }
    }
}

pub fn f344(v0: Adt125, v1: Adt5) -> Option<Str> {
    let m1362 = v0;
    match m1362 {
        Adt125::C0 => {
            None::<Str>
        }
        Adt125::C1 => {
            let m1363 = f89(Str::lit("name"), v1);
            match m1363 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v2) => {
                    f118(v2)
                }
            }
        }
        Adt125::C2 => {
            let m1364 = f89(Str::lit("uri"), v1);
            match m1364 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v3) => {
                    f118(v3)
                }
            }
        }
        Adt125::C3 => {
            let m1365 = f89(Str::lit("taskId"), v1);
            match m1365 {
                Adt114::C0 => {
                    None::<Str>
                }
                Adt114::C1(v4) => {
                    f118(v4)
                }
            }
        }
    }
}

pub fn f345(v0: Adt6) -> Option<Adt17> {
    let m1366 = f221(v0);
    match m1366 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m1367 = f360(Str::lit("experimental"), v1.clone());
            match m1367 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m1368 = f360(Str::lit("roots"), v1.clone());
                    match m1368 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m1369 = f360(Str::lit("sampling"), v1.clone());
                            match m1369 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m1370 = f360(Str::lit("elicitation"), v1.clone());
                                    match m1370 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m1371 = f360(Str::lit("extensions"), v1);
                                            match m1371 {
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

pub fn f346() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f347(v0: Adt5) -> Result<Option<Adt18>, Adt111> {
    let m1372 = f314(f361(), v0);
    match m1372 {
        Ok(v2) => {
            let m1373 = v2;
            match m1373 {
                None => {
                    Ok::<Option<Adt18>, Adt111>(None::<Adt18>)
                }
                Some(v3) => {
                    let m1374 = f362(v3);
                    match m1374 {
                        Adt129::C0 => {
                            Err::<Option<Adt18>, Adt111>(Adt111::C8(f361()))
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

pub fn f348(v0: Adt5) -> Result<Option<Adt19>, Adt111> {
    let m1375 = f89(f363(), v0);
    match m1375 {
        Adt114::C0 => {
            Ok::<Option<Adt19>, Adt111>(None::<Adt19>)
        }
        Adt114::C1(v1) => {
            let m1376 = f118(v1.clone());
            match m1376 {
                None => {
                    let m1377 = f364(v1);
                    match m1377 {
                        None => {
                            Err::<Option<Adt19>, Adt111>(Adt111::C8(f363()))
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

pub fn f349(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m1378 = v0;
    match m1378 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a1384 = f365(match v1.clone() {
                    Adt40::C0(h1379, _, _) => {
                        h1379
                    }
                });
                let a1385 = {
                    let a1382 = f366(match v1.clone() {
                        Adt40::C0(_, h1380, _) => {
                            h1380
                        }
                    });
                    let a1383 = f233(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h1381) => {
                            h1381
                        }
                    });
                    append_list(a1382, a1383)
                };
                append_list(a1384, a1385)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f350(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h1386, _, _, _) => {
            h1386
        }
    })), {
        let a1392 = f233(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h1387, _, _) => {
                h1387
            }
        });
        let a1393 = {
            let a1390 = f367(match v0.clone() {
                Adt2::C0(_, _, h1388, _) => {
                    h1388
                }
            });
            let a1391 = f368(match v0 {
                Adt2::C0(_, _, _, h1389) => {
                    h1389
                }
            });
            append_list(a1390, a1391)
        };
        append_list(a1392, a1393)
    }))
}

pub fn f351(v0: Adt117, v1: Adt117) -> bool {
    let a1394 = f246(v0);
    let a1395 = f246(v1);
    equal(a1394, a1395)
}

pub fn f352(v0: Adt117) -> bool {
    let m1396 = v0;
    match m1396 {
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

pub fn f353(v0: Adt117) -> bool {
    let m1397 = v0;
    match m1397 {
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

pub fn f354(v0: Adt0, v1: Adt16, v2: Adt79) -> Result<Adt108, Adt110> {
    f369(v0, v2.clone(), f370(match v1 {
        Adt16::C0(_, h1398, _, _, _, _) => {
            h1398
        }
    }, f371(v2)))
}

pub fn f355(v0: Adt14) -> Adt16 {
    let m1399 = v0;
    match m1399 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h1400) => {
                    h1400
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h1401, _) => {
                    h1401
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h1402, _, _, _, _) => {
                    h1402
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h1403, _) => {
                    h1403
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h1404, _, _, _) => {
                    h1404
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h1405, _) => {
                    h1405
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h1406, _) => {
                    h1406
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h1407, _, _, _, _) => {
                    h1407
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h1408, _, _, _) => {
                    h1408
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h1409, _) => {
                    h1409
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h1410, _, _) => {
                    h1410
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h1411, _) => {
                    h1411
                }
            }
        }
    }
}

pub fn f356(v0: Adt0, v1: Adt16, v2: Adt80) -> Result<Adt108, Adt110> {
    f372(v0, v2, f373(match v1 {
        Adt16::C0(_, h1412, _, _, _, _) => {
            h1412
        }
    }))
}

pub fn f357(v0: Str, p1418: List<(Str, Adt6)>, p1419: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt111> {
    let m1417 = (p1418, p1419);
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let (v1, v2) = m1417;
                let m1413 = v1;
                match m1413.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt111>>(Ok::<List<(Str, Str)>, Adt111>(v2))
                    }
                    Some((v3, v4)) => {
                        let m1415 = f118({
                            let (_, h1414) = v3;
                            h1414
                        });
                        match m1415 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt111>>(Err::<List<(Str, Str)>, Adt111>(Adt111::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt111>>((v4, List::cons(({
                                    let (h1416, _) = v3;
                                    h1416
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(p1422: List<(Str, Str)>, p1423: List<(Str, Str)>) -> List<(Str, Str)> {
    let m1421 = (p1422, p1423);
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let (v0, v1) = m1421;
                let m1420 = v0;
                match m1420.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
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

pub fn f359(v0: Adt96, v1: Adt5) -> R<Option<Adt94>> {
    let m1425 = f374(match v0.clone() {
        Adt96::C0(_, h1424, _, _) => {
            h1424
        }
    }, v1);
    match m1425 {
        Adt114::C0 => {
            Ok(f375(v0))
        }
        Adt114::C1(v2) => {
            if f376(v2.clone()) {
                Ok(f375(v0))
            } else {
                f377(v0, v2)
            }
        }
    }
}

pub fn f360(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m1426 = f89(v0, v1);
    match m1426 {
        Adt114::C0 => {
            Some(None::<Adt5>)
        }
        Adt114::C1(v2) => {
            let m1427 = f221(v2);
            match m1427 {
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

pub fn f361() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f362(v0: Str) -> Adt129 {
    let a1428 = v0.clone();
    let a1429 = Str::lit("debug");
    if equal(a1428, a1429) {
        Adt129::C1(Adt18::C0)
    } else {
        let a1430 = v0.clone();
        let a1431 = Str::lit("info");
        if equal(a1430, a1431) {
            Adt129::C1(Adt18::C1)
        } else {
            let a1432 = v0.clone();
            let a1433 = Str::lit("notice");
            if equal(a1432, a1433) {
                Adt129::C1(Adt18::C2)
            } else {
                let a1434 = v0.clone();
                let a1435 = Str::lit("warning");
                if equal(a1434, a1435) {
                    Adt129::C1(Adt18::C3)
                } else {
                    let a1436 = v0.clone();
                    let a1437 = Str::lit("error");
                    if equal(a1436, a1437) {
                        Adt129::C1(Adt18::C4)
                    } else {
                        let a1438 = v0.clone();
                        let a1439 = Str::lit("critical");
                        if equal(a1438, a1439) {
                            Adt129::C1(Adt18::C5)
                        } else {
                            let a1440 = v0.clone();
                            let a1441 = Str::lit("alert");
                            if equal(a1440, a1441) {
                                Adt129::C1(Adt18::C6)
                            } else {
                                let a1442 = v0;
                                let a1443 = Str::lit("emergency");
                                if equal(a1442, a1443) {
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

pub fn f363() -> Str {
    Str::lit("progressToken")
}

pub fn f364(v0: Adt6) -> Option<i64> {
    let m1444 = v0;
    match m1444 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f219(match v2.clone() {
                Adt7::C0(_, h1445) => {
                    h1445
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1446, _) => {
                        h1446
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

pub fn f365(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m1447 = v0;
    match m1447 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f378(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f366(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1448 = v0;
    match m1448 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f367(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m1449 = v0;
    match m1449 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f53(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f368(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m1450 = v0;
    match m1450 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f379(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f369(v0: Adt0, v1: Adt79, v2: bool) -> Result<Adt108, Adt110> {
    if v2 {
        Ok::<Adt108, Adt110>(f311(v0, Adt32::C11(v1)))
    } else {
        Err::<Adt108, Adt110>(Adt110::C4(Adt82::C5(f380(f371(v1)))))
    }
}

pub fn f370(v0: Adt17, v1: List<(Str, Adt62)>) -> bool {
    let a1451 = f381(v1.clone());
    if if if bool_not(a1451) {
        true
    } else {
        f249(match v0.clone() {
            Adt17::C0(_, _, _, h1452, _) => {
                h1452
            }
        })
    } {
        let a1453 = f382(v1.clone());
        if bool_not(a1453) {
            true
        } else {
            f249(match v0.clone() {
                Adt17::C0(_, _, h1454, _, _) => {
                    h1454
                }
            })
        }
    } else {
        false
    } {
        let a1455 = f383(v1);
        if bool_not(a1455) {
            true
        } else {
            f249(match v0 {
                Adt17::C0(_, h1456, _, _, _) => {
                    h1456
                }
            })
        }
    } else {
        false
    }
}

pub fn f371(v0: Adt79) -> List<(Str, Adt62)> {
    let m1457 = v0;
    match m1457 {
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

pub fn f372(v0: Adt0, v1: Adt80, v2: bool) -> Result<Adt108, Adt110> {
    if v2 {
        Ok::<Adt108, Adt110>(f311(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt108, Adt110>(Adt110::C4(Adt82::C5(f384())))
    }
}

pub fn f373(v0: Adt17) -> bool {
    f250(match v0 {
        Adt17::C0(_, _, _, _, h1458) => {
            h1458
        }
    }, f251())
}

pub fn f374(p1464: List<Str>, p1465: Adt5) -> Adt114 {
    let m1463 = (p1464, p1465);
    {
        let mut m1463 = m1463;
        loop {
            let step = {
                let (v0, v1) = m1463;
                let m1459 = v0;
                match m1459.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                    }
                    Some((v2, v3)) => {
                        let m1460 = f89(v2, v1);
                        match m1460 {
                            Adt114::C0 => {
                                Err::<(List<Str>, Adt5), Adt114>(Adt114::C0)
                            }
                            Adt114::C1(v4) => {
                                let m1461 = v3.clone();
                                match m1461.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt114>(Adt114::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m1462 = f221(v4);
                                        match m1462 {
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
                Ok(next) => { m1463 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: Adt96) -> Option<Adt94> {
    let m1467 = match v0.clone() {
        Adt96::C0(_, _, _, h1466) => {
            h1466
        }
    };
    match m1467 {
        Adt97::C0 => {
            None::<Adt94>
        }
        Adt97::C1 => {
            Some(Adt94::C10(match v0 {
                Adt96::C0(h1468, _, _, _) => {
                    h1468
                }
            }))
        }
        Adt97::C2(_) => {
            None::<Adt94>
        }
    }
}

pub fn f376(v0: Adt6) -> bool {
    let m1469 = v0;
    match m1469 {
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

pub fn f377(v0: Adt96, v1: Adt6) -> R<Option<Adt94>> {
    let m1471 = match v0.clone() {
        Adt96::C0(_, _, _, h1470) => {
            h1470
        }
    };
    match m1471 {
        Adt97::C0 => {
            Ok(Some(Adt94::C8(match v0 {
                Adt96::C0(h1472, _, _, _) => {
                    h1472
                }
            })))
        }
        Adt97::C1 => {
            Ok(Some(Adt94::C10(match v0 {
                Adt96::C0(h1473, _, _, _) => {
                    h1473
                }
            })))
        }
        Adt97::C2(v2) => {
            if f385(match v0.clone() {
                Adt96::C0(_, _, h1474, _) => {
                    h1474
                }
            }, v2, v1)? {
                Ok(None::<Adt94>)
            } else {
                Ok(Some(Adt94::C9(match v0 {
                    Adt96::C0(h1475, _, _, _) => {
                        h1475
                    }
                })))
            }
        }
    }
}

pub fn f378(v0: List<Adt41>) -> List<Adt6> {
    f72(f386(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f379(v0: Adt3) -> Str {
    let m1476 = v0;
    match m1476 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f380(v0: List<(Str, Adt62)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f383(v0.clone()) {
        Some(f54())
    } else {
        None::<Adt5>
    }, if f382(v0.clone()) {
        Some(f54())
    } else {
        None::<Adt5>
    }, if f381(v0) {
        Some(f54())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f381(p1481: List<(Str, Adt62)>) -> bool {
    let m1480 = p1481;
    {
        let mut m1480 = m1480;
        loop {
            let step = {
                let v0 = m1480;
                let m1477 = v0;
                match m1477.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1479 = {
                            let (_, h1478) = v1;
                            h1478
                        };
                        match m1479 {
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
                Ok(next) => { m1480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(p1486: List<(Str, Adt62)>) -> bool {
    let m1485 = p1486;
    {
        let mut m1485 = m1485;
        loop {
            let step = {
                let v0 = m1485;
                let m1482 = v0;
                match m1482.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1484 = {
                            let (_, h1483) = v1;
                            h1483
                        };
                        match m1484 {
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
                Ok(next) => { m1485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(p1491: List<(Str, Adt62)>) -> bool {
    let m1490 = p1491;
    {
        let mut m1490 = m1490;
        loop {
            let step = {
                let v0 = m1490;
                let m1487 = v0;
                match m1487.uncons() {
                    None => {
                        Err::<List<(Str, Adt62)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m1489 = {
                            let (_, h1488) = v1;
                            h1488
                        };
                        match m1489 {
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
                Ok(next) => { m1490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f384() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f251(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f385(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m1492 = v0;
    match m1492 {
        Adt13::C0 => {
            let m1493 = f118(v2);
            match m1493 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a1494 = v1;
                    let a1495 = v3;
                    Ok(equal(a1494, a1495))
                }
            }
        }
        Adt13::C1 => {
            f387(v1, v2)
        }
        Adt13::C2 => {
            let m1496 = f388(v2);
            match m1496 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a1497 = v1;
                        let a1498 = Str::lit("true");
                        Ok(equal(a1497, a1498))
                    } else {
                        let a1499 = v1;
                        let a1500 = Str::lit("false");
                        Ok(equal(a1499, a1500))
                    }
                }
            }
        }
    }
}

pub fn f386(p1503: List<Adt41>, p1504: List<Adt6>) -> List<Adt6> {
    let m1502 = (p1503, p1504);
    {
        let mut m1502 = m1502;
        loop {
            let step = {
                let (v0, v1) = m1502;
                let m1501 = v0;
                match m1501.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f389(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: Str, v1: Adt6) -> R<bool> {
    let m1505 = f390(v1);
    match m1505 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m1506 = f120(v2)?;
            match m1506 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m1508 = f391(f49({
                        let a1507 = v0;
                        utf8_encode(a1507)
                    })?)?;
                    match m1508 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f219({
                                let a1509 = v3;
                                let a1510 = v4;
                                int_sub(a1509, a1510)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f388(v0: Adt6) -> Option<bool> {
    let m1511 = v0;
    match m1511 {
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

pub fn f389(v0: Adt41) -> Str {
    let m1512 = v0;
    match m1512 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f390(v0: Adt6) -> Option<Adt7> {
    let m1513 = v0;
    match m1513 {
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

pub fn f391(v0: List<u64>) -> R<Option<i64>> {
    let m1514 = v0.clone();
    match m1514.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a1515 = v1;
            let a1516 = 45u64;
            if nat_eq(a1515, a1516) {
                let m1517 = v2.clone();
                match m1517.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m1518 = f392(v2, 0i64)?;
                        match m1518 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a1519 = v5;
                                    int_neg(a1519)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f392(v0, 0i64)
            }
        }
    }
}

pub fn f392(p1525: List<u64>, p1526: i64) -> R<Option<i64>> {
    let m1524 = (p1525, p1526);
    {
        let mut m1524 = m1524;
        loop {
            let step = {
                let (v0, v1) = m1524;
                let m1520 = v0;
                match m1520.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1521 = f393(v2);
                        match m1521 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f220({
                                    let a1522 = v1;
                                    let a1523 = 10i64;
                                    int_mul(a1522, a1523)?
                                }, f166(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(v0: u64) -> Option<u64> {
    let a1527 = v0;
    let a1528 = 48u64;
    if if nat_lt(a1527, a1528) {
        true
    } else {
        let a1529 = 57u64;
        let a1530 = v0;
        nat_lt(a1529, a1530)
    } {
        None::<u64>
    } else {
        Some({
            let a1531 = v0;
            let a1532 = 48u64;
            nat_sub(a1531, a1532)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
