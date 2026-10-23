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
    F41,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt49, Adt84>> {
        match self {
            Fn1::F41 => f41(p0),
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
    Ok(List::cons(f29(Str::lit("echo"), Str::lit("Returns a fixed acknowledgement"), f30()), List::cons(f29(Str::lit("execute_sql"), Str::lit("Runs a statement in a region"), f31()), List::cons(f29(Str::lit("test_missing_capability"), Str::lit("Requires the client to declare sampling"), f30()), List::cons(f29(Str::lit("test_simple_text"), Str::lit("Tests simple text content response"), f32()), List::cons(f29(Str::lit("test_image_content"), Str::lit("Tests image content response"), f32()), List::cons(f29(Str::lit("test_audio_content"), Str::lit("Tests audio content response"), f32()), List::cons(f29(Str::lit("test_embedded_resource"), Str::lit("Tests embedded resource content response"), f32()), List::cons(f29(Str::lit("test_multiple_content_types"), Str::lit("Tests response with multiple content types (text, image, resource)"), f32()), List::cons(f29(Str::lit("test_error_handling"), Str::lit("Tests error response handling"), f32()), List::cons(f29(Str::lit("test_tool_with_progress"), Str::lit("Tests tool that reports progress notifications"), f32()), List::cons(f29(Str::lit("json_schema_2020_12_tool"), Str::lit("Tool with JSON Schema 2020-12 features"), f33()), List::cons(f29(Str::lit("test_arguments"), Str::lit("Accepts arguments that its input schema constrains"), f34()?), List::cons(f29(Str::lit("test_nested_header"), Str::lit("Mirrors a nested parameter into a header"), f35()), List::cons(f36(Str::lit("test_structured_output"), Str::lit("Returns structured content that conforms to its output schema"), f32(), f37()), List::cons(f36(Str::lit("test_bad_structured_output"), Str::lit("Returns structured content that violates its output schema"), f32(), f37()), List::cons(f29(Str::lit("test_bad_payload"), Str::lit("Returns an image whose data is not Base64"), f32()), List::cons(f29(Str::lit("test_unsafe_text"), Str::lit("Returns text with control characters"), f32()), List::cons(f29(Str::lit("test_backward_progress"), Str::lit("Reports progress that does not increase"), f32()), List::<Adt35>::nil())))))))))))))))))))
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
            Ok(Some(Adt84::C9(f38(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m33 = v0;
    match m33 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f39(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f40(f12()?, Fn1::F41, v2)
        }
        Adt14::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C4(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C5(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C6(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C7(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C8(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C9(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C10(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
        Adt14::C11(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the fixture offers tools only"))))
        }
    }
}

pub fn f16(v0: Bytes, v1: u64) -> R<Option<Adt89>> {
    let m34 = f42(v0.clone(), v1)?;
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
            f43(v0.clone(), v1, v3.clone(), f42(v0, match v3 {
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
    f44(f45(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f46(f47(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f48(), f49()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a71 = f50();
    let a72 = {
        let a69 = f51();
        let a70 = f52();
        append_list(a69, a70)
    };
    append_list(a71, a72)
}

pub fn f24(v0: List<u64>) -> bool {
    f53(v0, f54())
}

pub fn f25(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt90>) -> R<Adt88> {
    let m73 = v4;
    match m73 {
        Ok(v6) => {
            f59(v0, v1, v2, v3, f60(v6)?)
        }
        Err(_) => {
            f55(f56(), f57(None::<Adt91>, f58(Adt92::C0)))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Result<Adt6, Adt90>> {
    f61(f62(v0)?)
}

pub fn f27() -> Adt88 {
    Adt88::C0(f63(), List::cons(Adt86::C0(f64(), f65()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f28(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt94>> {
    let m74 = f66(v0.clone());
    match m74 {
        None => {
            f67(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f29(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f30() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f31() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("region"), f68(Str::lit("string"), Str::lit("Region"))), List::cons((Str::lit("query"), f69()), List::cons((Str::lit("limit"), f68(Str::lit("integer"), Str::lit("Limit"))), List::cons((Str::lit("dry_run"), f68(Str::lit("boolean"), Str::lit("Dry-Run"))), List::<(Str, Adt6)>::nil())))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("region")), List::cons(Adt6::C3(Str::lit("query")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f32() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil())))
}

pub fn f33() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("$schema"), Adt6::C3(Str::lit("https://json-schema.org/draft/2020-12/schema"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("$defs"), Adt6::C5(List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$anchor"), Adt6::C3(Str::lit("addressDef"))), List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("street"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("city"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("address"), Adt6::C5(List::cons((Str::lit("$ref"), Adt6::C3(Str::lit("#/$defs/address"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("phone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("email"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))), List::cons((Str::lit("allOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("anyOf"), Adt6::C4(List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::cons(Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil())), List::<Adt6>::nil()))), List::cons((Str::lit("if"), Adt6::C5(List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("contactMethod"), Adt6::C5(List::cons((Str::lit("const"), Adt6::C3(Str::lit("phone"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("contactMethod")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("then"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("phone")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("else"), Adt6::C5(List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("email")), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))))))
}

pub fn f34() -> R<Adt5> {
    Ok(Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("name"), f69()), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("minLength"), f70(1u64)?), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("count"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::cons((Str::lit("minimum"), f70(0u64)?), List::cons((Str::lit("maximum"), f70(10u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("mode"), Adt6::C5(List::cons((Str::lit("enum"), Adt6::C4(List::cons(Adt6::C3(Str::lit("fast")), List::cons(Adt6::C3(Str::lit("slow")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("tags"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("array"))), List::cons((Str::lit("items"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("maxItems"), f70(3u64)?), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("nested"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("flag"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("boolean"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))), List::cons((Str::lit("a/b~c"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil()))))))))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("name")), List::<Adt6>::nil()))), List::cons((Str::lit("additionalProperties"), Adt6::C1(false)), List::<(Str, Adt6)>::nil()))))))
}

pub fn f35() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("target"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("zone"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::cons((Str::lit("x-mcp-header"), Adt6::C3(Str::lit("Zone"))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f36(v0: Str, v1: Str, v2: Adt5, v3: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, Some(v3), None::<Adt36>, None::<Adt5>)
}

pub fn f37() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::cons((Str::lit("sum"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("integer"))), List::<(Str, Adt6)>::nil()))), List::cons((Str::lit("label"), Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())))), List::cons((Str::lit("required"), Adt6::C4(List::cons(Adt6::C3(Str::lit("sum")), List::cons(Adt6::C3(Str::lit("label")), List::<Adt6>::nil())))), List::<(Str, Adt6)>::nil()))))
}

pub fn f38(v0: Adt94) -> Str {
    let m75 = v0;
    match m75 {
        Adt94::C0(v1) => {
            let a76 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a77 = Str::lit(" ");
            join(a76, a77)
        }
        Adt94::C1(v2) => {
            let a78 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a79 = Str::lit(" ");
            join(a78, a79)
        }
        Adt94::C2(v3) => {
            let a80 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a81 = Str::lit(" ");
            join(a80, a81)
        }
        Adt94::C3(v4) => {
            let a82 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a83 = Str::lit(" ");
            join(a82, a83)
        }
        Adt94::C4(v5, v6) => {
            let a84 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a85 = Str::lit(" ");
            join(a84, a85)
        }
        Adt94::C5(v7, v8) => {
            let a86 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a87 = Str::lit(" ");
            join(a86, a87)
        }
        Adt94::C6(v9, v10) => {
            let a88 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a89 = Str::lit(" ");
            join(a88, a89)
        }
        Adt94::C7(v11) => {
            let a90 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a91 = Str::lit(" ");
            join(a90, a91)
        }
        Adt94::C8(v12) => {
            let a92 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a93 = Str::lit(" ");
            join(a92, a93)
        }
        Adt94::C9(v13) => {
            let a94 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a95 = Str::lit(" ");
            join(a94, a95)
        }
        Adt94::C10(v14) => {
            let a96 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a97 = Str::lit(" ");
            join(a96, a97)
        }
        Adt94::C11(v15) => {
            let a98 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a99 = Str::lit(" ");
            join(a98, a99)
        }
        Adt94::C12(v16, v17) => {
            let a100 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a101 = Str::lit(" ");
            join(a100, a101)
        }
    }
}

pub fn f39(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m103 = match v2 {
        Adt20::C0(_, h102) => {
            h102
        }
    };
    match m103 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f40(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m105 = f71(match v2.clone() {
        Adt21::C0(_, h104, _, _, _) => {
            h104
        }
    }, v0);
    match m105 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h106, _, _, _) => {
                    h106
                }
            })))
        }
        Some(v3) => {
            f72(v3, v1, v2)
        }
    }
}

pub fn f41(v0: Adt21) -> R<Result<Adt49, Adt84>> {
    if f73(v0.clone(), Str::lit("test_simple_text")) {
        Ok(Ok::<Adt49, Adt84>(f74(List::cons(f75(Str::lit("This is a simple text response for testing.")), List::<Adt38>::nil()))))
    } else {
        if f73(v0.clone(), Str::lit("test_image_content")) {
            Ok(Ok::<Adt49, Adt84>(f74(List::cons(f76(), List::<Adt38>::nil()))))
        } else {
            if f73(v0.clone(), Str::lit("test_audio_content")) {
                Ok(Ok::<Adt49, Adt84>(f74(List::cons(f77(Str::lit("UklGRiYAAABXQVZFZm10IBAAAAABAAEAQB8AAAB9AAACABAAZGF0YQIAAAA="), Str::lit("audio/wav")), List::<Adt38>::nil()))))
            } else {
                if f73(v0.clone(), Str::lit("test_embedded_resource")) {
                    Ok(Ok::<Adt49, Adt84>(f74(List::cons(f78(Str::lit("test://embedded-resource"), Str::lit("text/plain"), Str::lit("This is an embedded resource content.")), List::<Adt38>::nil()))))
                } else {
                    if f73(v0.clone(), Str::lit("test_multiple_content_types")) {
                        Ok(Ok::<Adt49, Adt84>(f74(List::cons(f75(Str::lit("Multiple content types test:")), List::cons(f76(), List::cons(f78(Str::lit("test://mixed-content-resource"), Str::lit("application/json"), Str::lit("{\"test\":\"data\",\"value\":123}")), List::<Adt38>::nil()))))))
                    } else {
                        if f73(v0.clone(), Str::lit("test_error_handling")) {
                            Ok(Ok::<Adt49, Adt84>(f79(List::cons(f75(Str::lit("This tool intentionally returns an error for testing")), List::<Adt38>::nil()), None::<Adt6>, true, List::<Adt50>::nil())))
                        } else {
                            if f73(v0.clone(), Str::lit("test_tool_with_progress")) {
                                Ok(Ok::<Adt49, Adt84>(f80(v0, f81()?)))
                            } else {
                                if f73(v0.clone(), Str::lit("test_backward_progress")) {
                                    Ok(Ok::<Adt49, Adt84>(f80(v0, f82()?)))
                                } else {
                                    if f73(v0.clone(), Str::lit("test_structured_output")) {
                                        Ok(Ok::<Adt49, Adt84>(f79(List::cons(f75(Str::lit("{\"sum\":3,\"label\":\"three\"}")), List::<Adt38>::nil()), Some(f83()?), false, List::<Adt50>::nil())))
                                    } else {
                                        if f73(v0.clone(), Str::lit("test_bad_structured_output")) {
                                            Ok(Ok::<Adt49, Adt84>(f79(List::cons(f75(Str::lit("{\"sum\":\"three\"}")), List::<Adt38>::nil()), Some(f84()), false, List::<Adt50>::nil())))
                                        } else {
                                            if f73(v0.clone(), Str::lit("test_bad_payload")) {
                                                Ok(Ok::<Adt49, Adt84>(f74(List::cons(f85(Str::lit("not base64!"), Str::lit("image/png")), List::<Adt38>::nil()))))
                                            } else {
                                                if f73(v0.clone(), Str::lit("test_unsafe_text")) {
                                                    Ok(Ok::<Adt49, Adt84>(f74(List::cons(f75(f86()), List::<Adt38>::nil()))))
                                                } else {
                                                    Ok(f87(v0))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a110 = {
        let a107 = v1;
        let a108 = 4u64;
        nat_add(a107, a108)?
    };
    let a111 = {
        let a109 = v0.clone();
        length_bytes(a109)
    };
    if nat_le(a110, a111) {
        Ok(Some({
            let a128 = {
                let a112 = f88(v0.clone(), v1)?;
                let a113 = 16777216u64;
                nat_mul(a112, a113)?
            };
            let a129 = {
                let a126 = {
                    let a116 = f88(v0.clone(), {
                        let a114 = v1;
                        let a115 = 1u64;
                        nat_add(a114, a115)?
                    })?;
                    let a117 = 65536u64;
                    nat_mul(a116, a117)?
                };
                let a127 = {
                    let a124 = {
                        let a120 = f88(v0.clone(), {
                            let a118 = v1;
                            let a119 = 2u64;
                            nat_add(a118, a119)?
                        })?;
                        let a121 = 256u64;
                        nat_mul(a120, a121)?
                    };
                    let a125 = f88(v0, {
                        let a122 = v1;
                        let a123 = 3u64;
                        nat_add(a122, a123)?
                    })?;
                    nat_add(a124, a125)?
                };
                nat_add(a126, a127)?
            };
            nat_add(a128, a129)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f43(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m130 = v3;
    match m130 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h131) => {
                    h131
                }
            })))
        }
        Some(v4) => {
            let m135 = f89(v0.clone(), {
                let a133 = match v2.clone() {
                    Adt89::C0(_, h132) => {
                        h132
                    }
                };
                let a134 = 4u64;
                nat_add(a133, a134)?
            }, v4, List::<Adt86>::nil())?;
            match m135 {
                Ok(v6) => {
                    f90(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f44(p140: List<Adt86>, p141: List<u64>) -> List<u64> {
    let m139 = (p140, p141);
    {
        let mut m139 = m139;
        loop {
            let step = {
                let (v0, v1) = m139;
                let m136 = v0;
                match m136.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a137 = f91(v2);
                            let a138 = v1;
                            append_list(a137, a138)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m139 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f45(p144: List<Adt86>, p145: List<Adt86>) -> List<Adt86> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m142 = v0;
                match m142.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(p150: List<List<u64>>, p151: List<u64>) -> List<u64> {
    let m149 = (p150, p151);
    {
        let mut m149 = m149;
        loop {
            let step = {
                let (v0, v1) = m149;
                let m146 = v0;
                match m146.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a147 = f92(v2);
                            let a148 = v1;
                            append_list(a147, a148)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(p154: List<List<u64>>, p155: List<List<u64>>) -> List<List<u64>> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v0, v1) = m153;
                let m152 = v0;
                match m152.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f49() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f50() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f51() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f52() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f53(p162: List<u64>, p163: List<u64>) -> bool {
    let m161 = (p162, p163);
    {
        let mut m161 = m161;
        loop {
            let step = {
                let (v0, v1) = m161;
                let m156 = v0;
                match m156.uncons() {
                    None => {
                        let m157 = v1;
                        match m157.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m158 = v1;
                        match m158.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a159 = v4;
                                let a160 = v6;
                                if nat_eq(a159, a160) {
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
                Ok(next) => { m161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f55(v0: u64, v1: Adt6) -> R<Adt88> {
    let m164 = f93(v1)?;
    match m164 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f56() -> u64 {
    400u64
}

pub fn f57(v0: Option<Adt91>, v1: Adt80) -> Adt6 {
    Adt6::C5(f94(v0, v1))
}

pub fn f58(v0: Adt92) -> Adt80 {
    Adt80::C0(f95(v0.clone()), f96(v0), None::<Adt6>)
}

pub fn f59(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt95, Adt92>) -> R<Adt88> {
    let m165 = v4;
    match m165 {
        Ok(v6) => {
            f99(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f55(f97(v5.clone()), f57(f98(v5.clone()), f58(v5)))
        }
    }
}

pub fn f60(v0: Adt6) -> R<Result<Adt95, Adt92>> {
    let m166 = v0;
    match m166 {
        Adt6::C0 => {
            Ok(f101(None::<Adt91>, Adt93::C0))
        }
        Adt6::C1(_) => {
            Ok(f101(None::<Adt91>, Adt93::C0))
        }
        Adt6::C2(_) => {
            Ok(f101(None::<Adt91>, Adt93::C0))
        }
        Adt6::C3(_) => {
            Ok(f101(None::<Adt91>, Adt93::C0))
        }
        Adt6::C4(_) => {
            Ok(f101(None::<Adt91>, Adt93::C1))
        }
        Adt6::C5(v1) => {
            f100(Adt5::C0(v1))
        }
    }
}

pub fn f61(v0: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m167 = f102(v0.clone())?;
    match m167 {
        Adt101::C0 => {
            f103(f104(), 0u64, v0)
        }
        Adt101::C1(v1) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C0(v1)))
        }
        Adt101::C2(v2) => {
            Ok(Err::<Adt6, Adt90>(Adt90::C1(v2)))
        }
    }
}

pub fn f62(v0: Bytes) -> R<List<u64>> {
    f105(v0.clone(), {
        let a168 = v0;
        length_bytes(a168)
    }, List::<u64>::nil())
}

pub fn f63() -> u64 {
    405u64
}

pub fn f64() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f65() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f66(v0: List<Adt35>) -> Option<Adt94> {
    let m169 = f106(v0, List::<Str>::nil());
    match m169 {
        None => {
            None::<Adt94>
        }
        Some(v1) => {
            Some(Adt94::C1(v1))
        }
    }
}

pub fn f67(p173: List<Adt35>, p174: List<Adt12>) -> R<Result<List<Adt12>, Adt94>> {
    let m172 = (p173, p174);
    {
        let mut m172 = m172;
        loop {
            let step = {
                let (v0, v1) = m172;
                let m170 = v0;
                match m170.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Ok::<List<Adt12>, Adt94>(f107(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m171 = f108(v2)?;
                        match m171 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>((v3, f107(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt94>>>(Ok(Err::<List<Adt12>, Adt94>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m172 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: Str, v1: Str) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("x-mcp-header"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())))
}

pub fn f69() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("string"))), List::<(Str, Adt6)>::nil()))
}

pub fn f70(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f109(v0, 0i64)?, 0i64)))
}

pub fn f71(v0: Str, p180: List<Adt35>) -> Option<Adt35> {
    let m179 = p180;
    {
        let mut m179 = m179;
        loop {
            let step = {
                let v1 = m179;
                let m175 = v1;
                match m175.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a177 = match v2.clone() {
                            Adt35::C0(h176, _, _, _, _, _, _, _) => {
                                h176
                            }
                        };
                        let a178 = v0.clone();
                        if equal(a177, a178) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f72(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m187 = f110(Adt6::C5({
        let m183 = match v0.clone() {
            Adt35::C0(_, _, _, _, h182, _, _, _) => {
                h182
            }
        };
        match m183 {
            Adt5::C0(h181) => {
                h181
            }
        }
    }), Adt6::C5({
        let m186 = match v0.clone() {
            Adt35::C0(_, _, _, _, h185, _, _, _) => {
                h185
            }
        };
        match m186 {
            Adt5::C0(h184) => {
                h184
            }
        }
    }), f111(v2.clone()))?;
    match m187 {
        Adt109::C0 => {
            f112(v0, v1, v2)
        }
        Adt109::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f113(f114(v3, v4)?, true))))
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

pub fn f73(v0: Adt21, v1: Str) -> bool {
    let a189 = match v0 {
        Adt21::C0(_, h188, _, _, _) => {
            h188
        }
    };
    let a190 = v1;
    equal(a189, a190)
}

pub fn f74(v0: List<Adt38>) -> Adt49 {
    f79(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f75(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f76() -> Adt38 {
    f85(Str::lit("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="), Str::lit("image/png"))
}

pub fn f77(v0: Str, v1: Str) -> Adt38 {
    Adt38::C2(Adt43::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f78(v0: Str, v1: Str, v2: Str) -> Adt38 {
    Adt38::C4(Adt45::C0(Adt46::C0(Adt47::C0(v0, Some(v1), v2, None::<Adt5>)), None::<Adt40>, None::<Adt5>))
}

pub fn f79(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f80(v0: Adt21, v1: List<Adt50>) -> Adt49 {
    f79(List::cons(f75(f115({
        let m193 = match v0 {
            Adt21::C0(h192, _, _, _, _) => {
                h192
            }
        };
        match m193 {
            Adt16::C0(_, _, _, _, h191, _) => {
                h191
            }
        }
    })), List::<Adt38>::nil()), None::<Adt6>, false, v1)
}

pub fn f81() -> R<List<Adt50>> {
    Ok(List::cons(f116(0u64, 100u64)?, List::cons(f116(50u64, 100u64)?, List::cons(f116(100u64, 100u64)?, List::<Adt50>::nil()))))
}

pub fn f82() -> R<List<Adt50>> {
    Ok(List::cons(f116(50u64, 100u64)?, List::cons(f116(10u64, 100u64)?, List::<Adt50>::nil())))
}

pub fn f83() -> R<Adt6> {
    Ok(Adt6::C5(List::cons((Str::lit("sum"), f70(3u64)?), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil()))))
}

pub fn f84() -> Adt6 {
    Adt6::C5(List::cons((Str::lit("sum"), Adt6::C3(Str::lit("three"))), List::cons((Str::lit("label"), Adt6::C3(Str::lit("three"))), List::<(Str, Adt6)>::nil())))
}

pub fn f85(v0: Str, v1: Str) -> Adt38 {
    Adt38::C1(Adt42::C0(v0, v1, None::<Adt40>, None::<Adt5>))
}

pub fn f86() -> Str {
    let m195 = {
        let a194 = Bytes::lit(&[0x62, 0x65, 0x6c, 0x6c, 0x07, 0x20, 0x65, 0x73, 0x63, 0x61, 0x70, 0x65, 0x1b, 0x5b, 0x33, 0x31, 0x6d, 0x20, 0x72, 0x74, 0x6c, 0xe2, 0x80, 0xae]);
        utf8_decode(a194)
    };
    match m195 {
        None => {
            Str::lit("unreachable")
        }
        Some(v0) => {
            v0
        }
    }
}

pub fn f87(v0: Adt21) -> Result<Adt49, Adt84> {
    if f73(v0.clone(), Str::lit("test_missing_capability")) {
        let m201 = {
            let m200 = {
                let m199 = match v0 {
                    Adt21::C0(h198, _, _, _, _) => {
                        h198
                    }
                };
                match m199 {
                    Adt16::C0(_, h197, _, _, _, _) => {
                        h197
                    }
                }
            };
            match m200 {
                Adt17::C0(_, _, h196, _, _) => {
                    h196
                }
            }
        };
        match m201 {
            None => {
                Err::<Adt49, Adt84>(Adt84::C8(f117()))
            }
            Some(_) => {
                Ok::<Adt49, Adt84>(f118())
            }
        }
    } else {
        if f73(v0.clone(), Str::lit("json_schema_2020_12_tool")) {
            Ok::<Adt49, Adt84>(f74(List::cons(f75(Str::lit("JSON Schema 2020-12 tool accepted the arguments")), List::<Adt38>::nil())))
        } else {
            if f73(v0, Str::lit("test_arguments")) {
                Ok::<Adt49, Adt84>(f74(List::cons(f75(Str::lit("arguments accepted")), List::<Adt38>::nil())))
            } else {
                Ok::<Adt49, Adt84>(f118())
            }
        }
    }
}

pub fn f88(v0: Bytes, v1: u64) -> R<u64> {
    let m204 = {
        let a202 = v0;
        let a203 = v1;
        index_bytes(a202, a203)
    };
    match m204 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a249 = {
                let a225 = {
                    let a213 = {
                        let a207 = {
                            let a205 = v3;
                            let a206 = 128u8;
                            fixed_u8::bit_and(a205, a206)
                        };
                        let a208 = 128u8;
                        if equal(a207, a208) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a214 = {
                        let a211 = {
                            let a209 = v3;
                            let a210 = 64u8;
                            fixed_u8::bit_and(a209, a210)
                        };
                        let a212 = 64u8;
                        if equal(a211, a212) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a213, a214)?
                };
                let a226 = {
                    let a223 = {
                        let a217 = {
                            let a215 = v3;
                            let a216 = 32u8;
                            fixed_u8::bit_and(a215, a216)
                        };
                        let a218 = 32u8;
                        if equal(a217, a218) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a224 = {
                        let a221 = {
                            let a219 = v3;
                            let a220 = 16u8;
                            fixed_u8::bit_and(a219, a220)
                        };
                        let a222 = 16u8;
                        if equal(a221, a222) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a223, a224)?
                };
                nat_add(a225, a226)?
            };
            let a250 = {
                let a247 = {
                    let a235 = {
                        let a229 = {
                            let a227 = v3;
                            let a228 = 8u8;
                            fixed_u8::bit_and(a227, a228)
                        };
                        let a230 = 8u8;
                        if equal(a229, a230) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a236 = {
                        let a233 = {
                            let a231 = v3;
                            let a232 = 4u8;
                            fixed_u8::bit_and(a231, a232)
                        };
                        let a234 = 4u8;
                        if equal(a233, a234) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a235, a236)?
                };
                let a248 = {
                    let a245 = {
                        let a239 = {
                            let a237 = v3;
                            let a238 = 2u8;
                            fixed_u8::bit_and(a237, a238)
                        };
                        let a240 = 2u8;
                        if equal(a239, a240) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a246 = {
                        let a243 = {
                            let a241 = v3;
                            let a242 = 1u8;
                            fixed_u8::bit_and(a241, a242)
                        };
                        let a244 = 1u8;
                        if equal(a243, a244) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a245, a246)?
                };
                nat_add(a247, a248)?
            };
            nat_add(a249, a250)
        }
    }
}

pub fn f89(v0: Bytes, p260: u64, p261: u64, p262: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m259 = (p260, (p261, p262));
    {
        let mut m259 = m259;
        loop {
            let step = {
                let (v1, (v2, v3)) = m259;
                let m251 = v2;
                if m251 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f119(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m251 - 1;
                    let m252 = f16(v0.clone(), v1)?;
                    match m252 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m254 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h253) => {
                                    h253
                                }
                            })?;
                            match m254 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h255) => {
                                            h255
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h256) => {
                                            h256
                                        }
                                    }, (v4, List::cons(Adt86::C0(f62(match v5 {
                                        Adt89::C0(h257, _) => {
                                            h257
                                        }
                                    })?, f62(match v6 {
                                        Adt89::C0(h258, _) => {
                                            h258
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m259 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f90(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m264 = f16(v0.clone(), {
        let (_, h263) = v3.clone();
        h263
    })?;
    match m264 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h265) = v3;
                h265
            })))
        }
        Some(v4) => {
            let m267 = f42(v0.clone(), match v4.clone() {
                Adt89::C0(_, h266) => {
                    h266
                }
            })?;
            match m267 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h268) => {
                            h268
                        }
                    })))
                }
                Some(v5) => {
                    let a273 = {
                        let a270 = match v4.clone() {
                            Adt89::C0(_, h269) => {
                                h269
                            }
                        };
                        let a271 = 4u64;
                        nat_add(a270, a271)?
                    };
                    let a274 = {
                        let a272 = v0;
                        length_bytes(a272)
                    };
                    if nat_eq(a273, a274) {
                        Ok(Ok::<Adt85, Adt87>(Adt85::C0(f62(match v1 {
                            Adt89::C0(h275, _) => {
                                h275
                            }
                        })?, f62(match v2 {
                            Adt89::C0(h276, _) => {
                                h276
                            }
                        })?, {
                            let (h277, _) = v3;
                            h277
                        }, match v4 {
                            Adt89::C0(h278, _) => {
                                h278
                            }
                        }, v5)))
                    } else {
                        Ok(Err::<Adt85, Adt87>(Adt87::C1({
                            let a280 = match v4 {
                                Adt89::C0(_, h279) => {
                                    h279
                                }
                            };
                            let a281 = 4u64;
                            nat_add(a280, a281)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f91(v0: Adt86) -> List<u64> {
    let a292 = f18({
        let a283 = match v0.clone() {
            Adt86::C0(h282, _) => {
                h282
            }
        };
        length_list(a283)
    });
    let a293 = {
        let a290 = match v0.clone() {
            Adt86::C0(h284, _) => {
                h284
            }
        };
        let a291 = {
            let a288 = f18({
                let a286 = match v0.clone() {
                    Adt86::C0(_, h285) => {
                        h285
                    }
                };
                length_list(a286)
            });
            let a289 = match v0 {
                Adt86::C0(_, h287) => {
                    h287
                }
            };
            append_list(a288, a289)
        };
        append_list(a290, a291)
    };
    append_list(a292, a293)
}

pub fn f92(v0: List<u64>) -> List<u64> {
    let a295 = f18({
        let a294 = v0.clone();
        length_list(a294)
    });
    let a296 = v0;
    append_list(a295, a296)
}

pub fn f93(v0: Adt6) -> R<Result<List<u64>, Adt90>> {
    f120(f121(), v0)
}

pub fn f94(v0: Option<Adt91>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f122())), {
        let a297 = f123(v0);
        let a298 = List::cons((Str::lit("error"), f124(v1)), List::<(Str, Adt6)>::nil());
        append_list(a297, a298)
    })
}

pub fn f95(v0: Adt92) -> i64 {
    let m299 = v0;
    match m299 {
        Adt92::C0 => {
            f125()
        }
        Adt92::C1(_, _) => {
            f126()
        }
    }
}

pub fn f96(v0: Adt92) -> Str {
    let m300 = v0;
    match m300 {
        Adt92::C0 => {
            Str::lit("Parse error")
        }
        Adt92::C1(_, v2) => {
            f127(v2)
        }
    }
}

pub fn f97(v0: Adt92) -> u64 {
    let m301 = v0;
    match m301 {
        Adt92::C0 => {
            f56()
        }
        Adt92::C1(_, _) => {
            f56()
        }
    }
}

pub fn f98(v0: Adt92) -> Option<Adt91> {
    let m302 = v0;
    match m302 {
        Adt92::C0 => {
            None::<Adt91>
        }
        Adt92::C1(v1, _) => {
            v1
        }
    }
}

pub fn f99(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt95) -> R<Adt88> {
    f128(f129(v0.clone(), v1, v2, Some(f130(v3, match v0 {
        Adt0::C0(_, _, _, _, h303, _) => {
            h303
        }
    }, f131(v4.clone()), match v4.clone() {
        Adt95::C0(_, h304, _) => {
            h304
        }
    })?), f132(), v4)?)
}

pub fn f100(v0: Adt5) -> R<Result<Adt95, Adt92>> {
    let m305 = f133(v0.clone())?;
    match m305 {
        Adt122::C0 => {
            let m306 = f134(Str::lit("method"), v0.clone());
            match m306 {
                Adt123::C0 => {
                    Ok(f101(None::<Adt91>, f135(v0)))
                }
                Adt123::C1(_) => {
                    Ok(f101(None::<Adt91>, if f136(v0) {
                        Adt93::C8
                    } else {
                        Adt93::C7
                    }))
                }
            }
        }
        Adt122::C1(v2) => {
            Ok(f137(v2, v0))
        }
        Adt122::C2 => {
            Ok(f101(None::<Adt91>, Adt93::C5))
        }
    }
}

pub fn f101(v0: Option<Adt91>, v1: Adt93) -> Result<Adt95, Adt92> {
    Err::<Adt95, Adt92>(Adt92::C1(v0, v1))
}

pub fn f102(v0: List<u64>) -> R<Adt101> {
    Ok(f138({
        let a307 = v0.clone();
        length_list(a307)
    }, f139(Adt102::C0, 0u64, 0u64, v0)?))
}

pub fn f103(p314: Adt103, p315: u64, p316: List<u64>) -> R<Result<Adt6, Adt90>> {
    let m313 = (p314, (p315, p316));
    {
        let mut m313 = m313;
        loop {
            let step = {
                let (v0, (v1, v2)) = m313;
                let m308 = v2;
                match m308.uncons() {
                    None => {
                        let m309 = f140(v0, v1)?;
                        match m309 {
                            Ok(v3) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Ok::<Adt6, Adt90>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m310 = f141(v0, v1, v5)?;
                        match m310 {
                            Adt124::C0(v7) => {
                                Err::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>(Ok(Err::<Adt6, Adt90>(v7)))
                            }
                            Adt124::C1(v8) => {
                                Ok::<(Adt103, (u64, List<u64>)), R<Result<Adt6, Adt90>>>((v8, ({
                                    let a311 = v1;
                                    let a312 = 1u64;
                                    nat_add(a311, a312)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m313 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f104() -> Adt103 {
    Adt103::C0(Adt104::C0, List::<Adt108>::nil(), 0u64)
}

pub fn f105(v0: Bytes, p319: u64, p320: List<u64>) -> R<List<u64>> {
    let m318 = (p319, p320);
    {
        let mut m318 = m318;
        loop {
            let step = {
                let (v1, v2) = m318;
                let m317 = v1;
                if m317 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m317 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f88(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f106(p326: List<Adt35>, p327: List<Str>) -> Option<Str> {
    let m325 = (p326, p327);
    {
        let mut m325 = m325;
        loop {
            let step = {
                let (v0, v1) = m325;
                let m321 = v0;
                match m321.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f142(match v2.clone() {
                            Adt35::C0(h322, _, _, _, _, _, _, _) => {
                                h322
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h323, _, _, _, _, _, _, _) => {
                                    h323
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h324, _, _, _, _, _, _, _) => {
                                    h324
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m325 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(p330: List<Adt12>, p331: List<Adt12>) -> List<Adt12> {
    let m329 = (p330, p331);
    {
        let mut m329 = m329;
        loop {
            let step = {
                let (v0, v1) = m329;
                let m328 = v0;
                match m328.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m334 = f143(match v0.clone() {
        Adt35::C0(h332, _, _, _, _, _, _, _) => {
            h332
        }
    }, f144(match v0.clone() {
        Adt35::C0(h333, _, _, _, _, _, _, _) => {
            h333
        }
    })?);
    match m334 {
        None => {
            f145(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f109(p339: u64, p340: i64) -> R<i64> {
    let m338 = (p339, p340);
    {
        let mut m338 = m338;
        loop {
            let step = {
                let (v0, v1) = m338;
                let m335 = v0;
                if m335 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m335 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a336 = v1;
                        let a337 = -1i64;
                        int_sub(a336, a337)?
                    }))
                }
            };
            match step {
                Ok(next) => { m338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f110(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt109> {
    f146(v0, f147(), Adt111::C0, List::cons(Adt112::C0(v1, v2, List::<Adt110>::nil()), List::<Adt112>::nil()))
}

pub fn f111(v0: Adt21) -> Adt6 {
    let m342 = match v0 {
        Adt21::C0(_, _, h341, _, _) => {
            h341
        }
    };
    match m342 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h343) => {
                    h343
                }
            })
        }
    }
}

pub fn f112(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m345 = {
        let c344: Fn1 = v1;
        c344.apply(v2)?
    };
    match m345 {
        Ok(v4) => {
            f148(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f113(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f114(v0: List<Adt110>, v1: Str) -> R<Str> {
    let v2: Str = f149(v0)?;
    let a348 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a346 = v2.clone();
        let a347 = Str::lit("");
        if equal(a346, a347) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a349 = Str::lit(" ");
    Ok(join(a348, a349))
}

pub fn f115(v0: Option<Adt19>) -> Str {
    let m350 = v0;
    match m350 {
        None => {
            Str::lit("0")
        }
        Some(v1) => {
            let m351 = v1;
            match m351 {
                Adt19::C0(v2) => {
                    v2
                }
                Adt19::C1(v3) => {
                    let a352 = v3;
                    format_i64(a352)
                }
            }
        }
    }
}

pub fn f116(v0: u64, v1: u64) -> R<Adt50> {
    Ok(Adt50::C0(f150(v0)?, Some(f150(v1)?), Some({
        let a355 = List::cons(Str::lit("Completed step"), List::cons({
            let a353 = f151(v0, 0i64)?;
            format_i64(a353)
        }, List::cons(Str::lit("of"), List::cons({
            let a354 = f151(v1, 0i64)?;
            format_i64(a354)
        }, List::<Str>::nil()))));
        let a356 = Str::lit(" ");
        join(a355, a356)
    })))
}

pub fn f117() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, Some(f152()), None::<Adt5>, None::<Adt5>)
}

pub fn f118() -> Adt49 {
    f74(List::cons(f75(Str::lit("ok")), List::<Adt38>::nil()))
}

pub fn f119(p359: List<Adt86>, p360: List<Adt86>) -> List<Adt86> {
    let m358 = (p359, p360);
    {
        let mut m358 = m358;
        loop {
            let step = {
                let (v0, v1) = m358;
                let m357 = v0;
                match m357.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m358 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt90>> {
    let m361 = f153(v0, List::cons(Adt114::C1(0u64, v1), List::<Adt114>::nil()), List::<u64>::nil())?;
    match m361 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt90>(f154(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt90>(v3))
        }
    }
}

pub fn f121() -> u64 {
    67108864u64
}

pub fn f122() -> Str {
    Str::lit("2.0")
}

pub fn f123(v0: Option<Adt91>) -> List<(Str, Adt6)> {
    let m362 = v0;
    match m362 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f155(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f124(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h363, _, _) => {
            h363
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h364, _) => {
            h364
        }
    })), f156(match v0 {
        Adt80::C0(_, _, h365) => {
            h365
        }
    }))))
}

pub fn f125() -> i64 {
    -32700i64
}

pub fn f126() -> i64 {
    -32600i64
}

pub fn f127(v0: Adt93) -> Str {
    let m366 = v0;
    match m366 {
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

pub fn f128(v0: Adt115) -> R<Adt88> {
    let m368 = match v0.clone() {
        Adt115::C0(_, h367) => {
            h367
        }
    };
    match m368 {
        Ok(v1) => {
            let m369 = f157(v1.clone())?;
            match m369 {
                None => {
                    Ok(f6())
                }
                Some(v2) => {
                    Ok(f158(f55(f159(), f160(match v0 {
                        Adt115::C0(h370, _) => {
                            h370
                        }
                    }, v2))?, f161(match v1 {
                        Adt116::C0(_, _, h371) => {
                            h371
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v3) => {
            f55(f162(v3.clone()), f57(Some(match v0 {
                Adt115::C0(h372, _) => {
                    h372
                }
            }), f163(v3)?))
        }
    }
}

pub fn f129(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt96>, v4: List<Str>, v5: Adt95) -> R<Adt115> {
    Ok(Adt115::C0(match v5.clone() {
        Adt95::C0(h373, _, _) => {
            h373
        }
    }, f164(v0, v1, v2, f165(v3, v4, v5)?)?))
}

pub fn f130(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt96> {
    let v4: Adt121 = f166(Adt98::C0, f167(v0.clone(), f168())?);
    let v5: Adt121 = f166(Adt98::C1, f167(v0.clone(), f169())?);
    let v6: Adt121 = if f170(v3) {
        f166(Adt98::C2, f171(f167(v0.clone(), f172())?)?)
    } else {
        f173()
    };
    Ok(Adt96::C0(match v4.clone() {
        Adt121::C0(h374, _) => {
            h374
        }
    }, match v5.clone() {
        Adt121::C0(h375, _) => {
            h375
        }
    }, match v6.clone() {
        Adt121::C0(h376, _) => {
            h376
        }
    }, f174(match v4 {
        Adt121::C0(_, h377) => {
            h377
        }
    }, f174(match v5 {
        Adt121::C0(_, h378) => {
            h378
        }
    }, match v6 {
        Adt121::C0(_, h379) => {
            h379
        }
    })), f175(v0, v1, v2)?))
}

pub fn f131(v0: Adt95) -> Option<Str> {
    let a381 = match v0.clone() {
        Adt95::C0(_, h380, _) => {
            h380
        }
    };
    let a382 = Str::lit("tools/call");
    if equal(a381, a382) {
        let m384 = match v0 {
            Adt95::C0(_, _, h383) => {
                h383
            }
        };
        match m384 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m385 = f134(Str::lit("name"), v1);
                match m385 {
                    Adt123::C0 => {
                        None::<Str>
                    }
                    Adt123::C1(v2) => {
                        f176(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f132() -> List<Str> {
    List::cons(f177(), List::<Str>::nil())
}

pub fn f133(v0: Adt5) -> R<Adt122> {
    let m386 = f134(Str::lit("id"), v0);
    match m386 {
        Adt123::C0 => {
            Ok(Adt122::C0)
        }
        Adt123::C1(v1) => {
            let m387 = v1;
            match m387 {
                Adt6::C0 => {
                    Ok(Adt122::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt122::C2)
                }
                Adt6::C2(v3) => {
                    let m388 = f178(v3)?;
                    match m388 {
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

pub fn f134(v0: Str, v1: Adt5) -> Adt123 {
    f179(v0, match v1 {
        Adt5::C0(h389) => {
            h389
        }
    })
}

pub fn f135(v0: Adt5) -> Adt93 {
    if f136(v0) {
        Adt93::C8
    } else {
        Adt93::C3
    }
}

pub fn f136(v0: Adt5) -> bool {
    if f180(Str::lit("result"), v0.clone()) {
        true
    } else {
        f180(Str::lit("error"), v0)
    }
}

pub fn f137(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m390 = f134(Str::lit("jsonrpc"), v1.clone());
    match m390 {
        Adt123::C0 => {
            f101(Some(v0), Adt93::C2)
        }
        Adt123::C1(v2) => {
            let m391 = f176(v2);
            match m391 {
                None => {
                    f101(Some(v0), Adt93::C2)
                }
                Some(v3) => {
                    let a392 = v3;
                    let a393 = f122();
                    if equal(a392, a393) {
                        f181(v0, v1)
                    } else {
                        f101(Some(v0), Adt93::C2)
                    }
                }
            }
        }
    }
}

pub fn f138(v0: u64, v1: u64) -> Adt101 {
    let a394 = v1;
    let a395 = 0u64;
    if nat_eq(a394, a395) {
        Adt101::C0
    } else {
        let a396 = v1;
        let a397 = 2u64;
        if nat_eq(a396, a397) {
            Adt101::C2(v0)
        } else {
            Adt101::C1({
                let a401 = v0;
                let a402 = {
                    let a398 = v1;
                    let a399 = 3u64;
                    let a400 = 0u64;
                    nat_quot(a398, a399, a400)
                };
                nat_sub(a401, a402)
            })
        }
    }
}

pub fn f139(p430: Adt102, p431: u64, p432: u64, p433: List<u64>) -> R<u64> {
    let m429 = (p430, (p431, (p432, p433)));
    {
        let mut m429 = m429;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m429;
                let m403 = v3;
                match m403.uncons() {
                    None => {
                        let m404 = v0;
                        match m404 {
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
                        let m405 = v0;
                        match m405 {
                            Adt102::C0 => {
                                let a406 = v4;
                                let a407 = 128u64;
                                if nat_lt(a406, a407) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a408 = v4;
                                    let a409 = 194u64;
                                    if nat_lt(a408, a409) {
                                        let v6: u64 = f182({
                                            let a410 = List::cons(v4, v5);
                                            length_list(a410)
                                        })?;
                                        Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a411 = v4;
                                        let a412 = 224u64;
                                        if nat_lt(a411, a412) {
                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a413 = v4;
                                            let a414 = 224u64;
                                            if nat_eq(a413, a414) {
                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a415 = v4;
                                                let a416 = 237u64;
                                                if nat_eq(a415, a416) {
                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a417 = v4;
                                                    let a418 = 240u64;
                                                    if nat_lt(a417, a418) {
                                                        Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a419 = v4;
                                                        let a420 = 240u64;
                                                        if nat_eq(a419, a420) {
                                                            Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a421 = v4;
                                                            let a422 = 244u64;
                                                            if nat_eq(a421, a422) {
                                                                Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a423 = v4;
                                                                let a424 = 244u64;
                                                                if nat_lt(a423, a424) {
                                                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f182({
                                                                        let a425 = List::cons(v4, v5);
                                                                        length_list(a425)
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
                                if f183(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f182({
                                        let a426 = List::cons(v4, v5);
                                        length_list(a426)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt102::C2 => {
                                if f183(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f182({
                                        let a427 = List::cons(v4, v5);
                                        length_list(a427)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt102::C3 => {
                                if f183(v4, v1, v2) {
                                    Ok::<(Adt102, (u64, (u64, List<u64>))), R<u64>>((Adt102::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f182({
                                        let a428 = List::cons(v4, v5);
                                        length_list(a428)
                                    })?;
                                    Err::<(Adt102, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m429 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: Adt103, v1: u64) -> R<Result<Adt6, Adt90>> {
    let m435 = match v0.clone() {
        Adt103::C0(h434, _, _) => {
            h434
        }
    };
    match m435 {
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
            let m436 = f184(v3)?;
            match m436 {
                Adt131::C0 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C7(v1)))
                }
                Adt131::C1 => {
                    Ok(Err::<Adt6, Adt90>(Adt90::C8(v1)))
                }
                Adt131::C2(v4) => {
                    let m441 = {
                        let m440 = f185(v4, match v0.clone() {
                            Adt103::C0(_, h438, _) => {
                                h438
                            }
                        }, match v0 {
                            Adt103::C0(_, _, h439) => {
                                h439
                            }
                        });
                        match m440 {
                            Adt103::C0(h437, _, _) => {
                                h437
                            }
                        }
                    };
                    match m441 {
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

pub fn f141(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let m443 = match v0.clone() {
        Adt103::C0(h442, _, _) => {
            h442
        }
    };
    match m443 {
        Adt104::C0 => {
            if f186(v2) {
                Ok(Adt124::C1(v0))
            } else {
                f187(v0, v1, v2)
            }
        }
        Adt104::C1 => {
            if f186(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a444 = v2;
                let a445 = 93u64;
                if nat_eq(a444, a445) {
                    Ok(f188(v0, v1))
                } else {
                    f187(v0, v1, v2)
                }
            }
        }
        Adt104::C2 => {
            Ok(f189(v0, v1, v2))
        }
        Adt104::C3 => {
            if f186(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a446 = v2;
                let a447 = 125u64;
                if nat_eq(a446, a447) {
                    Ok(f190(v0, v1))
                } else {
                    let a448 = v2;
                    let a449 = 34u64;
                    if nat_eq(a448, a449) {
                        Ok(f191(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f192(Adt90::C2(v1)))
                    }
                }
            }
        }
        Adt104::C4 => {
            if f186(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a450 = v2;
                let a451 = 34u64;
                if nat_eq(a450, a451) {
                    Ok(f191(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f192(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C5 => {
            if f186(v2) {
                Ok(Adt124::C1(v0))
            } else {
                let a452 = v2;
                let a453 = 58u64;
                if nat_eq(a452, a453) {
                    Ok(f191(v0, Adt104::C0))
                } else {
                    Ok(f192(Adt90::C2(v1)))
                }
            }
        }
        Adt104::C6(v3, v4, v5) => {
            f193(v0, v1, v2, v3, v4, v5)
        }
        Adt104::C7(v6, v7) => {
            f194(v0, v1, v2, v6, v7)
        }
        Adt104::C8(v8) => {
            f195(v0, v1, v2, v8)
        }
        Adt104::C9(_) => {
            Ok(f189(v0, v1, v2))
        }
    }
}

pub fn f142(v0: Str, p458: List<Str>) -> bool {
    let m457 = p458;
    {
        let mut m457 = m457;
        loop {
            let step = {
                let v1 = m457;
                let m454 = v1;
                match m454.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a455 = v0.clone();
                        let a456 = v2;
                        if equal(a455, a456) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m457 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f143(v0: Str, v1: List<u64>) -> Option<Adt94> {
    if f196(v1) {
        Some(Adt94::C0(v0))
    } else {
        None::<Adt94>
    }
}

pub fn f144(v0: Str) -> R<List<u64>> {
    f62({
        let a459 = v0;
        utf8_encode(a459)
    })
}

pub fn f145(v0: Adt35) -> R<Result<List<Adt12>, Adt94>> {
    let m460 = f197(v0.clone());
    match m460 {
        None => {
            f198(v0.clone(), f199(f200(), match v0.clone() {
                Adt35::C0(h461, _, _, _, _, _, _, _) => {
                    h461
                }
            }, Adt6::C5({
                let m464 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h463, _, _, _) => {
                        h463
                    }
                };
                match m464 {
                    Adt5::C0(h462) => {
                        h462
                    }
                }
            }), List::cons(Adt125::C0(Adt6::C5({
                let m467 = match v0 {
                    Adt35::C0(_, _, _, _, h466, _, _, _) => {
                        h466
                    }
                };
                match m467 {
                    Adt5::C0(h465) => {
                        h465
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt125>::nil()), f201())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt94>(v1))
        }
    }
}

pub fn f146(v0: Adt6, p486: u64, p487: Adt111, p488: List<Adt112>) -> R<Adt109> {
    let m485 = (p486, (p487, p488));
    {
        let mut m485 = m485;
        loop {
            let step = {
                let (v1, (v2, v3)) = m485;
                let m468 = v1;
                if m468 == 0 {
                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C4))
                } else {
                    let v4 = m468 - 1;
                    let m469 = v3;
                    match m469.uncons() {
                        None => {
                            let m470 = v2;
                            match m470 {
                                Adt111::C0 => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C0))
                                }
                                Adt111::C1(v5, v6) => {
                                    Err::<(u64, (Adt111, List<Adt112>)), R<Adt109>>(Ok(Adt109::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m471 = v2.clone();
                            match m471 {
                                Adt111::C0 => {
                                    let m472 = v7.clone();
                                    match m472 {
                                        Adt112::C0(v9, v10, v11) => {
                                            let m473 = f202(v0.clone(), v9, v10, v11.clone())?;
                                            match m473 {
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
                                                        let a474 = v15;
                                                        let a475 = v8;
                                                        append_list(a474, a475)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt112::C1(_, _, _) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v8)))
                                        }
                                        Adt112::C2(_, _, _, _) => {
                                            let v24: Adt126 = f203(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v24.clone() {
                                                Adt126::C0(h476, _) => {
                                                    h476
                                                }
                                            }, match v24 {
                                                Adt126::C0(_, h477) => {
                                                    h477
                                                }
                                            })))
                                        }
                                        Adt112::C3(v25) => {
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (Adt111::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt112::C4(_, _, _, _) => {
                                            let v39: Adt126 = f203(v7, v8)?;
                                            Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v39.clone() {
                                                Adt126::C0(h481, _) => {
                                                    h481
                                                }
                                            }, match v39 {
                                                Adt126::C0(_, h482) => {
                                                    h482
                                                }
                                            })))
                                        }
                                        Adt112::C5(v26, v27, v28, v29) => {
                                            let m478 = f204(v26, v27, v28, v29.clone());
                                            match m478 {
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
                                                        let a479 = v33;
                                                        let a480 = v8;
                                                        append_list(a479, a480)
                                                    };
                                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt111::C1(v40, v41) => {
                                    let v42: Adt126 = f205(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt111, List<Adt112>)), R<Adt109>>((v4, (match v42.clone() {
                                        Adt126::C0(h483, _) => {
                                            h483
                                        }
                                    }, match v42 {
                                        Adt126::C0(_, h484) => {
                                            h484
                                        }
                                    })))
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

pub fn f147() -> u64 {
    67108864u64
}

pub fn f148(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f206({
        let m491 = match v1.clone() {
            Adt49::C0(h490, _) => {
                h490
            }
        };
        match m491 {
            Adt37::C0(h489, _, _) => {
                h489
            }
        }
    })? {
        f207(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f149(v0: List<Adt110>) -> R<Str> {
    let a492 = List::cons(Str::lit(""), f208(v0, List::<Str>::nil())?);
    let a493 = Str::lit("/");
    Ok(join(a492, a493))
}

pub fn f150(v0: u64) -> R<Adt7> {
    Ok(Adt7::C0(f151(v0, 0i64)?, 0i64))
}

pub fn f151(p496: u64, p497: i64) -> R<i64> {
    let m495 = (p496, p497);
    {
        let mut m495 = m495;
        loop {
            let step = {
                let (v0, v1) = m495;
                let m494 = v0;
                if m494 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m494 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f209(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m495 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f153(p502: u64, p503: List<Adt114>, p504: List<u64>) -> R<Result<List<u64>, Adt90>> {
    let m501 = (p502, (p503, p504));
    {
        let mut m501 = m501;
        loop {
            let step = {
                let (v0, (v1, v2)) = m501;
                let m498 = v0;
                if m498 == 0 {
                    Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Err::<List<u64>, Adt90>(Adt90::C13)))
                } else {
                    let v3 = m498 - 1;
                    let m499 = v1;
                    match m499.uncons() {
                        None => {
                            Err::<(u64, (List<Adt114>, List<u64>)), R<Result<List<u64>, Adt90>>>(Ok(Ok::<List<u64>, Adt90>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m500 = f210(v4, v5, v2)?;
                            match m500 {
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
                Ok(next) => { m501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(p507: List<u64>, p508: List<u64>) -> List<u64> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v0, v1) = m506;
                let m505 = v0;
                match m505.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Adt91) -> Adt6 {
    let m509 = v0;
    match m509 {
        Adt91::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt91::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f156(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m510 = v0;
    match m510 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f157(v0: Adt116) -> R<Option<Adt5>> {
    let m512 = f211(match v0.clone() {
        Adt116::C0(h511, _, _) => {
            h511
        }
    })?;
    match m512 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a515 = v1;
                let a516 = {
                    let m514 = f212(v0);
                    match m514 {
                        Adt5::C0(h513) => {
                            h513
                        }
                    }
                };
                append_list(a515, a516)
            })))
        }
    }
}

pub fn f158(v0: Adt88, v1: Option<List<List<u64>>>) -> Adt88 {
    let m517 = v1;
    match m517 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt88::C0(match v0.clone() {
                Adt88::C0(h518, _, _, _) => {
                    h518
                }
            }, match v0.clone() {
                Adt88::C0(_, h519, _, _) => {
                    h519
                }
            }, f47(v2, List::<List<u64>>::nil()), match v0 {
                Adt88::C0(_, _, _, h520) => {
                    h520
                }
            })
        }
    }
}

pub fn f159() -> u64 {
    200u64
}

pub fn f160(v0: Adt91, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f122())), List::cons((Str::lit("id"), f155(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h521) => {
            h521
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f161(p525: List<Adt118>, p526: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m524 = (p525, p526);
    {
        let mut m524 = m524;
        loop {
            let step = {
                let (v0, v1) = m524;
                let m522 = v0;
                match m522.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m523 = f213(v2)?;
                        match m523 {
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
                Ok(next) => { m524 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: Adt119) -> u64 {
    let m527 = v0;
    match m527 {
        Adt119::C0(_) => {
            f56()
        }
        Adt119::C1(_, _) => {
            f56()
        }
        Adt119::C2(_) => {
            f56()
        }
        Adt119::C3(_) => {
            f214()
        }
        Adt119::C4(v6) => {
            f215(v6)
        }
    }
}

pub fn f163(v0: Adt119) -> R<Adt80> {
    Ok(Adt80::C0(f216(v0.clone()), f217(v0.clone()), f218(v0)?))
}

pub fn f164(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt127, Adt119>) -> R<Result<Adt116, Adt119>> {
    let m528 = v3;
    match m528 {
        Ok(v5) => {
            f219(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(v4))
        }
    }
}

pub fn f165(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Result<Adt127, Adt119>> {
    Ok(f220(f221(match v2.clone() {
        Adt95::C0(_, h529, _) => {
            h529
        }
    }), f222(v0, v1, v2)?))
}

pub fn f166(v0: Adt98, v1: Adt129) -> Adt121 {
    let m530 = v1;
    match m530 {
        Adt129::C0 => {
            f173()
        }
        Adt129::C1 => {
            Adt121::C0(None::<Str>, Some(Adt97::C6(v0)))
        }
        Adt129::C2 => {
            Adt121::C0(None::<Str>, Some(Adt97::C7(v0)))
        }
        Adt129::C3(v2) => {
            let m531 = f223(v2);
            match m531 {
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

pub fn f167(v0: List<Adt86>, v1: List<u64>) -> R<Adt129> {
    let m532 = f224(v0, v1, Adt130::C0)?;
    match m532 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v2) => {
            let m533 = f225(v2);
            match m533 {
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

pub fn f168() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f169() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f170(v0: Str) -> bool {
    let m534 = f226(v0);
    match m534 {
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

pub fn f171(v0: Adt129) -> R<Adt129> {
    let m535 = v0.clone();
    match m535 {
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
            let m536 = f227(v1);
            match m536 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m537 = f228(v2)?;
                    match m537 {
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

pub fn f172() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f173() -> Adt121 {
    Adt121::C0(None::<Str>, None::<Adt97>)
}

pub fn f174(v0: Option<Adt97>, v1: Option<Adt97>) -> Option<Adt97> {
    let m538 = v0.clone();
    match m538 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f175(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt99>> {
    Ok(f229(f230(v0, v1, v2, List::<Adt99>::nil())?, List::<Adt99>::nil()))
}

pub fn f176(v0: Adt6) -> Option<Str> {
    let m539 = v0;
    match m539 {
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

pub fn f177() -> Str {
    Str::lit("2026-07-28")
}

pub fn f178(v0: Adt7) -> R<Option<i64>> {
    let m542 = f231(40u64, match v0.clone() {
        Adt7::C0(h540, _) => {
            h540
        }
    }, match v0 {
        Adt7::C0(_, h541) => {
            h541
        }
    })?;
    match m542 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f232(v1))
        }
    }
}

pub fn f179(v0: Str, p548: List<(Str, Adt6)>) -> Adt123 {
    let m547 = p548;
    {
        let mut m547 = m547;
        loop {
            let step = {
                let v1 = m547;
                let m543 = v1;
                match m543.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m545 = f233({
                            let (h544, _) = v2;
                            h544
                        }, v0.clone());
                        match m545 {
                            Adt139::C0 => {
                                Err::<List<(Str, Adt6)>, Adt123>(Adt123::C1({
                                    let (_, h546) = v2;
                                    h546
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
                Ok(next) => { m547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f180(v0: Str, v1: Adt5) -> bool {
    f234(f134(v0, v1))
}

pub fn f181(v0: Adt91, v1: Adt5) -> Result<Adt95, Adt92> {
    let m549 = f134(Str::lit("method"), v1.clone());
    match m549 {
        Adt123::C0 => {
            f101(Some(v0), f135(v1))
        }
        Adt123::C1(v2) => {
            let m550 = f176(v2);
            match m550 {
                None => {
                    f101(Some(v0), Adt93::C4)
                }
                Some(v3) => {
                    f235(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f182(v0: u64) -> R<u64> {
    let a553 = {
        let a551 = v0;
        let a552 = 3u64;
        nat_mul(a551, a552)?
    };
    let a554 = 1u64;
    nat_add(a553, a554)
}

pub fn f183(v0: u64, v1: u64, v2: u64) -> bool {
    let a555 = v1;
    let a556 = v0;
    if nat_le(a555, a556) {
        let a557 = v0;
        let a558 = v2;
        nat_le(a557, a558)
    } else {
        false
    }
}

pub fn f184(v0: Adt106) -> R<Adt131> {
    if f236(match v0.clone() {
        Adt106::C0(_, h559, _, _, _, _, _) => {
            h559
        }
    }) {
        let v1: i64 = {
            let a565 = if match v0.clone() {
                Adt106::C0(_, _, _, _, _, h560, _) => {
                    h560
                }
            } {
                let a562 = match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h561) => {
                        h561
                    }
                };
                int_neg(a562)?
            } else {
                match v0.clone() {
                    Adt106::C0(_, _, _, _, _, _, h563) => {
                        h563
                    }
                }
            };
            let a566 = match v0.clone() {
                Adt106::C0(_, _, _, _, h564, _, _) => {
                    h564
                }
            };
            int_sub(a565, a566)?
        };
        let m569 = {
            let a567 = v1;
            let a568 = -9999i64;
            compare(a567, a568)
        };
        if if match m569 {
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
            let m572 = {
                let a570 = 9999i64;
                let a571 = v1;
                compare(a570, a571)
            };
            match m572 {
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
                Adt106::C0(h573, _, _, _, _, _, _) => {
                    h573
                }
            } {
                let a575 = match v0 {
                    Adt106::C0(_, _, h574, _, _, _, _) => {
                        h574
                    }
                };
                int_neg(a575)?
            } else {
                match v0 {
                    Adt106::C0(_, _, h576, _, _, _, _) => {
                        h576
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt131::C0)
    }
}

pub fn f185(v0: Adt6, v1: List<Adt108>, v2: u64) -> Adt103 {
    let m577 = v1.clone();
    match m577.uncons() {
        None => {
            Adt103::C0(Adt104::C9(v0), List::<Adt108>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m578 = v3;
            match m578 {
                Adt108::C0(v5) => {
                    Adt103::C0(Adt104::C2, List::cons(Adt108::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt108::C1(v6, v7) => {
                    let m579 = v7;
                    match m579 {
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

pub fn f186(v0: u64) -> bool {
    let a580 = v0;
    let a581 = 32u64;
    if nat_eq(a580, a581) {
        true
    } else {
        let a582 = v0;
        let a583 = 10u64;
        if nat_eq(a582, a583) {
            true
        } else {
            let a584 = v0;
            let a585 = 13u64;
            if nat_eq(a584, a585) {
                true
            } else {
                let a586 = v0;
                let a587 = 9u64;
                nat_eq(a586, a587)
            }
        }
    }
}

pub fn f187(v0: Adt103, v1: u64, v2: u64) -> R<Adt124> {
    let a588 = v2;
    let a589 = 34u64;
    if nat_eq(a588, a589) {
        Ok(f191(v0, Adt104::C6(Adt105::C0, List::<u64>::nil(), false)))
    } else {
        let a590 = v2;
        let a591 = 123u64;
        if nat_eq(a590, a591) {
            f237(v0, Adt108::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt104::C3, v1)
        } else {
            let a592 = v2;
            let a593 = 91u64;
            if nat_eq(a592, a593) {
                f237(v0, Adt108::C0(List::<Adt6>::nil()), Adt104::C1, v1)
            } else {
                let a594 = v2;
                let a595 = 116u64;
                if nat_eq(a594, a595) {
                    Ok(f191(v0, Adt104::C7(0u64, 1u64)))
                } else {
                    let a596 = v2;
                    let a597 = 102u64;
                    if nat_eq(a596, a597) {
                        Ok(f191(v0, Adt104::C7(1u64, 1u64)))
                    } else {
                        let a598 = v2;
                        let a599 = 110u64;
                        if nat_eq(a598, a599) {
                            Ok(f191(v0, Adt104::C7(2u64, 1u64)))
                        } else {
                            let a600 = v2;
                            let a601 = 45u64;
                            if nat_eq(a600, a601) {
                                Ok(f191(v0, f238(true, Adt107::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a602 = v2;
                                let a603 = 48u64;
                                if nat_eq(a602, a603) {
                                    Ok(f191(v0, f238(false, Adt107::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f183(v2, 49u64, 57u64) {
                                        Ok(f191(v0, f238(false, Adt107::C2, f239({
                                            let a604 = v2;
                                            let a605 = 48u64;
                                            nat_sub(a604, a605)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f192(Adt90::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f188(v0: Adt103, v1: u64) -> Adt124 {
    let m607 = match v0.clone() {
        Adt103::C0(_, h606, _) => {
            h606
        }
    };
    match m607.uncons() {
        None => {
            f192(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m608 = v2;
            match m608 {
                Adt108::C0(v4) => {
                    Adt124::C1(f185(Adt6::C4(f240(v4, List::<Adt6>::nil())), v3, {
                        let a610 = match v0 {
                            Adt103::C0(_, _, h609) => {
                                h609
                            }
                        };
                        let a611 = 1u64;
                        nat_sub(a610, a611)
                    }))
                }
                Adt108::C1(_, _) => {
                    f192(Adt90::C2(v1))
                }
            }
        }
    }
}

pub fn f189(v0: Adt103, v1: u64, v2: u64) -> Adt124 {
    let m613 = match v0.clone() {
        Adt103::C0(h612, _, _) => {
            h612
        }
    };
    match m613 {
        Adt104::C0 => {
            f192(Adt90::C2(v1))
        }
        Adt104::C1 => {
            f192(Adt90::C2(v1))
        }
        Adt104::C2 => {
            if f186(v2) {
                Adt124::C1(v0)
            } else {
                let a614 = v2;
                let a615 = 44u64;
                if nat_eq(a614, a615) {
                    f241(v0, v1)
                } else {
                    let a616 = v2;
                    let a617 = 93u64;
                    if nat_eq(a616, a617) {
                        f188(v0, v1)
                    } else {
                        let a618 = v2;
                        let a619 = 125u64;
                        if nat_eq(a618, a619) {
                            f190(v0, v1)
                        } else {
                            f192(Adt90::C2(v1))
                        }
                    }
                }
            }
        }
        Adt104::C3 => {
            f192(Adt90::C2(v1))
        }
        Adt104::C4 => {
            f192(Adt90::C2(v1))
        }
        Adt104::C5 => {
            f192(Adt90::C2(v1))
        }
        Adt104::C6(_, _, _) => {
            f192(Adt90::C2(v1))
        }
        Adt104::C7(_, _) => {
            f192(Adt90::C2(v1))
        }
        Adt104::C8(_) => {
            f192(Adt90::C2(v1))
        }
        Adt104::C9(_) => {
            if f186(v2) {
                Adt124::C1(v0)
            } else {
                f192(Adt90::C10(v1))
            }
        }
    }
}

pub fn f190(v0: Adt103, v1: u64) -> Adt124 {
    let m621 = match v0.clone() {
        Adt103::C0(_, h620, _) => {
            h620
        }
    };
    match m621.uncons() {
        None => {
            f192(Adt90::C2(v1))
        }
        Some((v2, v3)) => {
            let m622 = v2;
            match m622 {
                Adt108::C0(_) => {
                    f192(Adt90::C2(v1))
                }
                Adt108::C1(v5, _) => {
                    Adt124::C1(f185(Adt6::C5(f242(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a624 = match v0 {
                            Adt103::C0(_, _, h623) => {
                                h623
                            }
                        };
                        let a625 = 1u64;
                        nat_sub(a624, a625)
                    }))
                }
            }
        }
    }
}

pub fn f191(v0: Adt103, v1: Adt104) -> Adt124 {
    Adt124::C1(f243(v0, v1))
}

pub fn f192(v0: Adt90) -> Adt124 {
    Adt124::C0(v0)
}

pub fn f193(v0: Adt103, v1: u64, v2: u64, v3: Adt105, v4: List<u64>, v5: bool) -> R<Adt124> {
    let m626 = v3;
    match m626 {
        Adt105::C0 => {
            let a627 = v2;
            let a628 = 34u64;
            if nat_eq(a627, a628) {
                Ok(f244(v0, v1, v4, v5))
            } else {
                let a629 = v2;
                let a630 = 92u64;
                if nat_eq(a629, a630) {
                    Ok(f191(v0, Adt104::C6(Adt105::C1, v4, v5)))
                } else {
                    let a631 = v2;
                    let a632 = 32u64;
                    if nat_lt(a631, a632) {
                        Ok(f192(Adt90::C4(v1)))
                    } else {
                        Ok(f191(v0, Adt104::C6(Adt105::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt105::C1 => {
            let a633 = v2;
            let a634 = 117u64;
            if nat_eq(a633, a634) {
                Ok(f191(v0, Adt104::C6(Adt105::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a635 = f245(v2);
                let a636 = 0u64;
                if nat_eq(a635, a636) {
                    Ok(f192(Adt90::C5(v1)))
                } else {
                    Ok(f191(v0, Adt104::C6(Adt105::C0, List::cons(f245(v2), v4), v5)))
                }
            }
        }
        Adt105::C2(v6, v7, v8) => {
            let a637 = 15u64;
            let a638 = f246(v2);
            if nat_lt(a637, a638) {
                Ok(f192(Adt90::C5(v1)))
            } else {
                let a641 = {
                    let a639 = v6;
                    let a640 = 1u64;
                    nat_add(a639, a640)?
                };
                let a642 = 4u64;
                if nat_eq(a641, a642) {
                    f247(v0, v1, {
                        let a645 = {
                            let a643 = v7;
                            let a644 = 16u64;
                            nat_mul(a643, a644)?
                        };
                        let a646 = f246(v2);
                        nat_add(a645, a646)?
                    }, v8, v4, v5)
                } else {
                    Ok(f191(v0, Adt104::C6(Adt105::C2({
                        let a647 = v6;
                        let a648 = 1u64;
                        nat_add(a647, a648)?
                    }, {
                        let a651 = {
                            let a649 = v7;
                            let a650 = 16u64;
                            nat_mul(a649, a650)?
                        };
                        let a652 = f246(v2);
                        nat_add(a651, a652)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt105::C3(v9) => {
            let a653 = v2;
            let a654 = 92u64;
            if nat_eq(a653, a654) {
                Ok(f191(v0, Adt104::C6(Adt105::C4(v9), v4, v5)))
            } else {
                Ok(f192(Adt90::C6(v1)))
            }
        }
        Adt105::C4(v10) => {
            let a655 = v2;
            let a656 = 117u64;
            if nat_eq(a655, a656) {
                Ok(f191(v0, Adt104::C6(Adt105::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f192(Adt90::C6(v1)))
            }
        }
    }
}

pub fn f194(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt124> {
    let a657 = v2;
    let a658 = f248(v3, v4);
    if nat_eq(a657, a658) {
        let a662 = {
            let a659 = v4;
            let a660 = 1u64;
            nat_add(a659, a660)?
        };
        let a663 = {
            let a661 = f249(v3);
            length_list(a661)
        };
        if nat_eq(a662, a663) {
            Ok(f250(v0, f251(v3)))
        } else {
            Ok(f191(v0, Adt104::C7(v3, {
                let a664 = v4;
                let a665 = 1u64;
                nat_add(a664, a665)?
            })))
        }
    } else {
        Ok(f192(Adt90::C2(v1)))
    }
}

pub fn f195(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m667 = match v3.clone() {
        Adt106::C0(_, h666, _, _, _, _, _) => {
            h666
        }
    };
    match m667 {
        Adt107::C0 => {
            let a668 = v2;
            let a669 = 48u64;
            if nat_eq(a668, a669) {
                Ok(f252(v0, v3, Adt107::C1))
            } else {
                if f183(v2, 49u64, 57u64) {
                    f253(v0, v1, v2, v3, false)
                } else {
                    Ok(f192(Adt90::C7(v1)))
                }
            }
        }
        Adt107::C1 => {
            let a670 = v2;
            let a671 = 46u64;
            if nat_eq(a670, a671) {
                Ok(f252(v0, v3, Adt107::C3))
            } else {
                if f254(v2) {
                    Ok(f252(v0, v3, Adt107::C5))
                } else {
                    if f255(v2) {
                        Ok(f192(Adt90::C7(v1)))
                    } else {
                        f256(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C2 => {
            if f255(v2) {
                f253(v0, v1, v2, v3, false)
            } else {
                let a672 = v2;
                let a673 = 46u64;
                if nat_eq(a672, a673) {
                    Ok(f252(v0, v3, Adt107::C3))
                } else {
                    if f254(v2) {
                        Ok(f252(v0, v3, Adt107::C5))
                    } else {
                        f256(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt107::C3 => {
            if f255(v2) {
                f253(v0, v1, v2, v3, true)
            } else {
                Ok(f192(Adt90::C7(v1)))
            }
        }
        Adt107::C4 => {
            if f255(v2) {
                f253(v0, v1, v2, v3, true)
            } else {
                if f254(v2) {
                    Ok(f252(v0, v3, Adt107::C5))
                } else {
                    f256(v0, v1, v2, v3)
                }
            }
        }
        Adt107::C5 => {
            let a674 = v2;
            let a675 = 43u64;
            if nat_eq(a674, a675) {
                Ok(f191(v0, f238(match v3.clone() {
                    Adt106::C0(h676, _, _, _, _, _, _) => {
                        h676
                    }
                }, Adt107::C6, match v3.clone() {
                    Adt106::C0(_, _, h677, _, _, _, _) => {
                        h677
                    }
                }, match v3.clone() {
                    Adt106::C0(_, _, _, h678, _, _, _) => {
                        h678
                    }
                }, match v3 {
                    Adt106::C0(_, _, _, _, h679, _, _) => {
                        h679
                    }
                }, false, 0i64)))
            } else {
                let a680 = v2;
                let a681 = 45u64;
                if nat_eq(a680, a681) {
                    Ok(f191(v0, f238(match v3.clone() {
                        Adt106::C0(h682, _, _, _, _, _, _) => {
                            h682
                        }
                    }, Adt107::C6, match v3.clone() {
                        Adt106::C0(_, _, h683, _, _, _, _) => {
                            h683
                        }
                    }, match v3.clone() {
                        Adt106::C0(_, _, _, h684, _, _, _) => {
                            h684
                        }
                    }, match v3 {
                        Adt106::C0(_, _, _, _, h685, _, _) => {
                            h685
                        }
                    }, true, 0i64)))
                } else {
                    if f255(v2) {
                        f257(v0, v1, v2, Adt106::C0(match v3.clone() {
                            Adt106::C0(h686, _, _, _, _, _, _) => {
                                h686
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, h687, _, _, _, _, _) => {
                                h687
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, h688, _, _, _, _) => {
                                h688
                            }
                        }, match v3.clone() {
                            Adt106::C0(_, _, _, h689, _, _, _) => {
                                h689
                            }
                        }, match v3 {
                            Adt106::C0(_, _, _, _, h690, _, _) => {
                                h690
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f192(Adt90::C7(v1)))
                    }
                }
            }
        }
        Adt107::C6 => {
            if f255(v2) {
                f257(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h691, _) => {
                        h691
                    }
                })
            } else {
                Ok(f192(Adt90::C7(v1)))
            }
        }
        Adt107::C7 => {
            if f255(v2) {
                f257(v0, v1, v2, v3.clone(), match v3 {
                    Adt106::C0(_, _, _, _, _, h692, _) => {
                        h692
                    }
                })
            } else {
                f256(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f196(v0: List<u64>) -> bool {
    let m693 = v0.clone();
    match m693.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a697 = if f258(v0.clone(), false) {
                let a695 = {
                    let a694 = v0;
                    length_list(a694)
                };
                let a696 = f259();
                nat_le(a695, a696)
            } else {
                false
            };
            bool_not(a697)
        }
    }
}

pub fn f197(v0: Adt35) -> Option<Adt94> {
    let m702 = f260(match v0.clone() {
        Adt35::C0(h698, _, _, _, _, _, _, _) => {
            h698
        }
    }, {
        let m701 = match v0.clone() {
            Adt35::C0(_, _, _, _, h700, _, _, _) => {
                h700
            }
        };
        match m701 {
            Adt5::C0(h699) => {
                h699
            }
        }
    });
    match m702 {
        None => {
            f261(match v0.clone() {
                Adt35::C0(h703, _, _, _, _, _, _, _) => {
                    h703
                }
            }, {
                let m706 = match v0 {
                    Adt35::C0(_, _, _, _, h705, _, _, _) => {
                        h705
                    }
                };
                match m706 {
                    Adt5::C0(h704) => {
                        h704
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f198(v0: Adt35, v1: Result<Adt132, Adt94>) -> R<Result<List<Adt12>, Adt94>> {
    let m707 = v1;
    match m707 {
        Ok(v3) => {
            let m708 = f262(v0)?;
            match m708 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt94>(f107(match v3 {
                        Adt132::C0(h709, _) => {
                            h709
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

pub fn f199(p716: u64, v1: Str, v2: Adt6, p717: List<Adt125>, p718: Adt132) -> R<Result<Adt132, Adt94>> {
    let m715 = (p716, (p717, p718));
    {
        let mut m715 = m715;
        loop {
            let step = {
                let (v0, (v3, v4)) = m715;
                let m710 = v0;
                if m710 == 0 {
                    Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Err::<Adt132, Adt94>(Adt94::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m710 - 1;
                    let m711 = v3;
                    match m711.uncons() {
                        None => {
                            Err::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>(Ok(Ok::<Adt132, Adt94>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m712 = f263(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m712 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt125>, Adt132)), R<Result<Adt132, Adt94>>>((v5, ({
                                        let (h713, _) = v9;
                                        h713
                                    }, {
                                        let (_, h714) = v9;
                                        h714
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
                Ok(next) => { m715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200() -> u64 {
    67108864u64
}

pub fn f201() -> Adt132 {
    Adt132::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f202(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m719 = f264(v1.clone());
    match m719 {
        None => {
            let m720 = f265(v1);
            match m720 {
                None => {
                    Ok(Adt133::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f266(v0, match v5 {
                        Adt5::C0(h721) => {
                            h721
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

pub fn f203(v0: Adt112, v1: List<Adt112>) -> R<Adt126> {
    let m722 = v0;
    match m722 {
        Adt112::C0(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C1(_, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
        Adt112::C2(v8, v9, v10, v11) => {
            Ok(f267(v8, v9, v10, {
                let a723 = v11;
                let a724 = 1u64;
                nat_add(a723, a724)?
            }, v1))
        }
        Adt112::C3(v12) => {
            Ok(Adt126::C0(Adt111::C1(v12, Str::lit("not")), v1))
        }
        Adt112::C4(v13, _, v15, v16) => {
            Ok(f268(v13, v15, v16, v1))
        }
        Adt112::C5(_, _, _, _) => {
            Ok(Adt126::C0(Adt111::C0, v1))
        }
    }
}

pub fn f204(v0: Adt113, v1: Adt6, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m725 = f265(v1);
    match m725 {
        None => {
            Adt133::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m726 = v0;
            match m726 {
                Adt113::C0 => {
                    f269(match v4 {
                        Adt5::C0(h727) => {
                            h727
                        }
                    }, v2, v3)
                }
                Adt113::C1 => {
                    f270(match v4 {
                        Adt5::C0(h728) => {
                            h728
                        }
                    }, v2, v3)
                }
                Adt113::C2 => {
                    f271(match v4 {
                        Adt5::C0(h729) => {
                            h729
                        }
                    }, v2, v3)
                }
                Adt113::C3 => {
                    f272(match v4 {
                        Adt5::C0(h730) => {
                            h730
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f205(v0: Adt112, v1: List<Adt112>, v2: List<Adt110>, v3: Str) -> Adt126 {
    let m731 = v0;
    match m731 {
        Adt112::C0(_, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
        Adt112::C1(v7, v8, v9) => {
            f273(v7, v8, v9, v1)
        }
        Adt112::C2(v10, v11, v12, v13) => {
            f267(v10, v11, v12, v13, v1)
        }
        Adt112::C3(_) => {
            Adt126::C0(Adt111::C0, v1)
        }
        Adt112::C4(_, v16, v17, v18) => {
            f268(v16, v17, v18, v1)
        }
        Adt112::C5(_, _, _, _) => {
            Adt126::C0(Adt111::C1(v2, v3), v1)
        }
    }
}

pub fn f206(p740: List<Adt38>) -> R<bool> {
    let m739 = p740;
    {
        let mut m739 = m739;
        loop {
            let step = {
                let v0 = m739;
                let m732 = v0;
                match m732.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m733 = v1;
                        match m733 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f274(match v4 {
                                    Adt42::C0(h734, _, _, _) => {
                                        h734
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f274(match v5 {
                                    Adt43::C0(h735, _, _, _) => {
                                        h735
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
                                let m737 = match v7 {
                                    Adt45::C0(h736, _, _) => {
                                        h736
                                    }
                                };
                                match m737 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f274(match v9 {
                                            Adt48::C0(_, _, h738, _) => {
                                                h738
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
                Ok(next) => { m739 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f275(match v1.clone() {
        Adt49::C0(h741, _) => {
            h741
        }
    }) {
        f276(v1)
    } else {
        let m743 = match v0 {
            Adt35::C0(_, _, _, _, _, h742, _, _) => {
                h742
            }
        };
        match m743 {
            None => {
                f276(v1)
            }
            Some(v2) => {
                f277(v2, v1)
            }
        }
    }
}

pub fn f208(p746: List<Adt110>, p747: List<Str>) -> R<List<Str>> {
    let m745 = (p746, p747);
    {
        let mut m745 = m745;
        loop {
            let step = {
                let (v0, v1) = m745;
                let m744 = v0;
                match m744.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Str>), R<List<Str>>>((v3, List::cons(f278(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: i64, v1: i64) -> R<i64> {
    let a749 = v0;
    let a750 = {
        let a748 = v1;
        int_neg(a748)?
    };
    int_sub(a749, a750)
}

pub fn f210(v0: Adt114, v1: List<Adt114>, v2: List<u64>) -> R<Adt134> {
    let m751 = v0;
    match m751 {
        Adt114::C0(v3) => {
            Ok(Adt134::C1(v1, f154(v3, v2)))
        }
        Adt114::C1(v4, v5) => {
            f279(v4, v5, v1, v2)
        }
        Adt114::C2(v6, v7) => {
            let m752 = v7;
            match m752.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C1(v6, v8), List::cons(Adt114::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt114::C3(v10, v11) => {
            let m753 = v11.clone();
            match m753.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt114::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt114::C4(v14, v15) => {
            let m754 = v15;
            match m754.uncons() {
                None => {
                    Ok(Adt134::C1(List::cons(Adt114::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt134::C1(List::cons(Adt114::C0({
                        let a756 = f280({
                            let (h755, _) = v16;
                            h755
                        })?;
                        let a757 = List::cons(58u64, List::<u64>::nil());
                        append_list(a756, a757)
                    }), List::cons(Adt114::C1(v14, {
                        let (_, h758) = v16;
                        h758
                    }), List::cons(Adt114::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt114::C5(v18, v19) => {
            let m759 = v19.clone();
            match m759.uncons() {
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

pub fn f211(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m760 = v0;
    match m760 {
        Adt32::C0(v1) => {
            Ok(Some(f281(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f282(v2)?))
        }
        Adt32::C2(v3) => {
            f283(f284(v3)?)
        }
        Adt32::C3(v4) => {
            f283(f284(match v4 {
                Adt49::C0(h761, _) => {
                    h761
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

pub fn f212(v0: Adt116) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f285(f286(match v0.clone() {
        Adt116::C0(h762, _, _) => {
            h762
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f287({
        let m765 = match v0 {
            Adt116::C0(_, h764, _) => {
                h764
            }
        };
        match m765 {
            Adt117::C0(h763, _) => {
                h763
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f213(v0: Adt118) -> R<Option<List<u64>>> {
    let m766 = f288(f289(v0))?;
    match m766 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m767 = f93(v1)?;
            match m767 {
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

pub fn f214() -> u64 {
    404u64
}

pub fn f215(v0: Adt84) -> u64 {
    let m768 = v0;
    match m768 {
        Adt84::C0(_) => {
            f56()
        }
        Adt84::C1(_) => {
            f56()
        }
        Adt84::C2(_) => {
            f56()
        }
        Adt84::C3(_) => {
            f56()
        }
        Adt84::C4(_) => {
            f56()
        }
        Adt84::C5(_) => {
            f56()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f290()
        }
        Adt84::C8(_) => {
            f56()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f216(v0: Adt119) -> i64 {
    let m769 = v0;
    match m769 {
        Adt119::C0(_) => {
            f291()
        }
        Adt119::C1(_, _) => {
            f292()
        }
        Adt119::C2(_) => {
            f293()
        }
        Adt119::C3(_) => {
            f294()
        }
        Adt119::C4(v6) => {
            f295(v6)
        }
    }
}

pub fn f217(v0: Adt119) -> Str {
    let m770 = v0;
    match m770 {
        Adt119::C0(v1) => {
            f296(v1)
        }
        Adt119::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt119::C2(v4) => {
            f297(v4)
        }
        Adt119::C3(_) => {
            Str::lit("Method not found")
        }
        Adt119::C4(v6) => {
            let m771 = v6;
            match m771 {
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

pub fn f218(v0: Adt119) -> R<Option<Adt6>> {
    let m772 = v0;
    match m772 {
        Adt119::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f298(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt119::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt119::C4(v6) => {
            let m773 = v6;
            match m773 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f151(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f299(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f219(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127) -> R<Result<Adt116, Adt119>> {
    f300(v0.clone(), v1, v2, v3.clone(), f301(match v0 {
        Adt0::C0(_, _, h774, _, _, _) => {
            h774
        }
    }, match v3 {
        Adt127::C0(_, h775, _, _) => {
            h775
        }
    }))
}

pub fn f220(v0: Adt135, v1: Adt136) -> Result<Adt127, Adt119> {
    let m776 = v1;
    match m776 {
        Adt136::C0(v3) => {
            f302(v3, v0)
        }
        Adt136::C1(v2) => {
            Err::<Adt127, Adt119>(v2)
        }
    }
}

pub fn f221(v0: Str) -> Adt135 {
    let a777 = v0.clone();
    let a778 = Str::lit("server/discover");
    if equal(a777, a778) {
        Adt135::C1(Adt128::C0)
    } else {
        let a779 = v0.clone();
        let a780 = Str::lit("tools/list");
        if equal(a779, a780) {
            Adt135::C1(Adt128::C1)
        } else {
            let a781 = v0.clone();
            let a782 = Str::lit("tools/call");
            if equal(a781, a782) {
                Adt135::C1(Adt128::C2)
            } else {
                let a783 = v0.clone();
                let a784 = Str::lit("resources/list");
                if equal(a783, a784) {
                    Adt135::C1(Adt128::C3)
                } else {
                    let a785 = v0.clone();
                    let a786 = Str::lit("resources/read");
                    if equal(a785, a786) {
                        Adt135::C1(Adt128::C4)
                    } else {
                        let a787 = v0.clone();
                        let a788 = Str::lit("resources/templates/list");
                        if equal(a787, a788) {
                            Adt135::C1(Adt128::C5)
                        } else {
                            let a789 = v0.clone();
                            let a790 = Str::lit("prompts/list");
                            if equal(a789, a790) {
                                Adt135::C1(Adt128::C6)
                            } else {
                                let a791 = v0.clone();
                                let a792 = Str::lit("prompts/get");
                                if equal(a791, a792) {
                                    Adt135::C1(Adt128::C7)
                                } else {
                                    let a793 = v0.clone();
                                    let a794 = Str::lit("completion/complete");
                                    if equal(a793, a794) {
                                        Adt135::C1(Adt128::C8)
                                    } else {
                                        let a795 = v0.clone();
                                        let a796 = Str::lit("tasks/get");
                                        if equal(a795, a796) {
                                            Adt135::C1(Adt128::C9)
                                        } else {
                                            let a797 = v0.clone();
                                            let a798 = Str::lit("tasks/update");
                                            if equal(a797, a798) {
                                                Adt135::C1(Adt128::C10)
                                            } else {
                                                let a799 = v0;
                                                let a800 = Str::lit("tasks/cancel");
                                                if equal(a799, a800) {
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

pub fn f222(v0: Option<Adt96>, v1: List<Str>, v2: Adt95) -> R<Adt136> {
    f303(v0, v1, v2.clone(), f304(match v2 {
        Adt95::C0(_, _, h801) => {
            h801
        }
    }))
}

pub fn f223(v0: List<u64>) -> Option<Str> {
    let a802 = f305(f306(), v0);
    utf8_decode(a802)
}

pub fn f224(p807: List<Adt86>, v1: List<u64>, p808: Adt130) -> R<Adt130> {
    let m806 = (p807, p808);
    {
        let mut m806 = m806;
        loop {
            let step = {
                let (v0, v2) = m806;
                let m803 = v0;
                match m803.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt130), R<Adt130>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f307(match v3.clone() {
                            Adt86::C0(h804, _) => {
                                h804
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, f308(v2, match v3 {
                                Adt86::C0(_, h805) => {
                                    h805
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt130), R<Adt130>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f225(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f309(v0);
    if f310(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f226(v0: Str) -> Adt138 {
    let m809 = f221(v0);
    match m809 {
        Adt135::C0 => {
            Adt138::C0
        }
        Adt135::C1(v1) => {
            f311(v1)
        }
    }
}

pub fn f227(v0: List<u64>) -> Option<List<u64>> {
    let m810 = f312(f313(), v0);
    match m810 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m811 = f312(f314(), f315(v1, List::<u64>::nil()));
            match m811 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f315(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f228(v0: List<u64>) -> R<Option<List<u64>>> {
    let m813 = f316({
        let a812 = v0.clone();
        length_list(a812)
    }, v0, List::<u64>::nil())?;
    match m813 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f154(v1, List::<u64>::nil())))
        }
    }
}

pub fn f229(p816: List<Adt99>, p817: List<Adt99>) -> List<Adt99> {
    let m815 = (p816, p817);
    {
        let mut m815 = m815;
        loop {
            let step = {
                let (v0, v1) = m815;
                let m814 = v0;
                match m814.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Adt99>), List<Adt99>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: List<Adt86>, p824: List<Adt12>, v2: Option<Str>, p825: List<Adt99>) -> R<List<Adt99>> {
    let m823 = (p824, p825);
    {
        let mut m823 = m823;
        loop {
            let step = {
                let (v1, v3) = m823;
                let m818 = v1;
                match m818.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f317(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, List::cons(Adt99::C0(match v4.clone() {
                                Adt12::C0(_, h819, _, _) => {
                                    h819
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h820, _) => {
                                    h820
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h821) => {
                                    h821
                                }
                            }, f318(v0.clone(), match v4 {
                                Adt12::C0(_, h822, _, _) => {
                                    h822
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt99>), R<List<Adt99>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m823 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(p841: u64, p842: i64, p843: i64) -> R<Option<i64>> {
    let m840 = (p841, (p842, p843));
    {
        let mut m840 = m840;
        loop {
            let step = {
                let (v0, (v1, v2)) = m840;
                let m826 = v0;
                if m826 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m826 - 1;
                    if f319(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m829 = {
                            let a827 = v2;
                            let a828 = 0i64;
                            compare(a827, a828)
                        };
                        if match m829 {
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
                            if f319({
                                let a830 = v1;
                                let a831 = 10i64;
                                let a832 = 0i64;
                                int_rem(a830, a831, a832)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a833 = v1;
                                    let a834 = 10i64;
                                    let a835 = 0i64;
                                    int_quot(a833, a834, a835)?
                                }, f209(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a836 = v1;
                                let a837 = 10i64;
                                int_mul(a836, a837)?
                            }, {
                                let a838 = v2;
                                let a839 = 1i64;
                                int_sub(a838, a839)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: i64) -> Option<i64> {
    let m846 = {
        let a844 = 9007199254740991i64;
        let a845 = v0;
        compare(a844, a845)
    };
    if if match m846 {
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
        let m849 = {
            let a847 = v0;
            let a848 = -9007199254740991i64;
            compare(a847, a848)
        };
        match m849 {
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

pub fn f233(v0: Str, v1: Str) -> Adt139 {
    let a850 = v0;
    let a851 = v1;
    if equal(a850, a851) {
        Adt139::C0
    } else {
        Adt139::C1
    }
}

pub fn f234(v0: Adt123) -> bool {
    let m852 = v0;
    match m852 {
        Adt123::C0 => {
            false
        }
        Adt123::C1(_) => {
            true
        }
    }
}

pub fn f235(v0: Adt91, v1: Str, v2: Adt5) -> Result<Adt95, Adt92> {
    let m853 = f134(Str::lit("params"), v2);
    match m853 {
        Adt123::C0 => {
            Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, None::<Adt5>))
        }
        Adt123::C1(v3) => {
            let m854 = f265(v3);
            match m854 {
                None => {
                    f101(Some(v0), Adt93::C6)
                }
                Some(v4) => {
                    Ok::<Adt95, Adt92>(Adt95::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f236(v0: Adt107) -> bool {
    let m855 = v0;
    match m855 {
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

pub fn f237(v0: Adt103, v1: Adt108, v2: Adt104, v3: u64) -> R<Adt124> {
    let a857 = match v0.clone() {
        Adt103::C0(_, _, h856) => {
            h856
        }
    };
    let a858 = f320();
    if nat_lt(a857, a858) {
        Ok(Adt124::C1(Adt103::C0(v2, List::cons(v1, match v0.clone() {
            Adt103::C0(_, h859, _) => {
                h859
            }
        }), {
            let a861 = match v0 {
                Adt103::C0(_, _, h860) => {
                    h860
                }
            };
            let a862 = 1u64;
            nat_add(a861, a862)?
        })))
    } else {
        Ok(f192(Adt90::C9(v3)))
    }
}

pub fn f238(v0: bool, v1: Adt107, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt104 {
    Adt104::C8(Adt106::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f239(v0: u64) -> i64 {
    let a863 = v0;
    let a864 = 0u64;
    if nat_eq(a863, a864) {
        0i64
    } else {
        let a865 = v0;
        let a866 = 1u64;
        if nat_eq(a865, a866) {
            1i64
        } else {
            let a867 = v0;
            let a868 = 2u64;
            if nat_eq(a867, a868) {
                2i64
            } else {
                let a869 = v0;
                let a870 = 3u64;
                if nat_eq(a869, a870) {
                    3i64
                } else {
                    let a871 = v0;
                    let a872 = 4u64;
                    if nat_eq(a871, a872) {
                        4i64
                    } else {
                        let a873 = v0;
                        let a874 = 5u64;
                        if nat_eq(a873, a874) {
                            5i64
                        } else {
                            let a875 = v0;
                            let a876 = 6u64;
                            if nat_eq(a875, a876) {
                                6i64
                            } else {
                                let a877 = v0;
                                let a878 = 7u64;
                                if nat_eq(a877, a878) {
                                    7i64
                                } else {
                                    let a879 = v0;
                                    let a880 = 8u64;
                                    if nat_eq(a879, a880) {
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

pub fn f240(p883: List<Adt6>, p884: List<Adt6>) -> List<Adt6> {
    let m882 = (p883, p884);
    {
        let mut m882 = m882;
        loop {
            let step = {
                let (v0, v1) = m882;
                let m881 = v0;
                match m881.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Adt103, v1: u64) -> Adt124 {
    let m886 = match v0.clone() {
        Adt103::C0(_, h885, _) => {
            h885
        }
    };
    match m886.uncons() {
        None => {
            f192(Adt90::C2(v1))
        }
        Some((v2, _)) => {
            let m887 = v2;
            match m887 {
                Adt108::C0(_) => {
                    f191(v0, Adt104::C0)
                }
                Adt108::C1(_, _) => {
                    f191(v0, Adt104::C4)
                }
            }
        }
    }
}

pub fn f242(p890: List<(Str, Adt6)>, p891: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m889 = (p890, p891);
    {
        let mut m889 = m889;
        loop {
            let step = {
                let (v0, v1) = m889;
                let m888 = v0;
                match m888.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Adt103, v1: Adt104) -> Adt103 {
    Adt103::C0(v1, match v0.clone() {
        Adt103::C0(_, h892, _) => {
            h892
        }
    }, match v0 {
        Adt103::C0(_, _, h893) => {
            h893
        }
    })
}

pub fn f244(v0: Adt103, v1: u64, v2: List<u64>, v3: bool) -> Adt124 {
    let m895 = {
        let a894 = f305(f306(), f154(v2, List::<u64>::nil()));
        utf8_decode(a894)
    };
    match m895 {
        None => {
            f192(Adt90::C14)
        }
        Some(v4) => {
            if v3 {
                f321(v0, v1, v4)
            } else {
                f250(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f245(v0: u64) -> u64 {
    let a896 = v0;
    let a897 = 34u64;
    if nat_eq(a896, a897) {
        34u64
    } else {
        let a898 = v0;
        let a899 = 92u64;
        if nat_eq(a898, a899) {
            92u64
        } else {
            let a900 = v0;
            let a901 = 47u64;
            if nat_eq(a900, a901) {
                47u64
            } else {
                let a902 = v0;
                let a903 = 98u64;
                if nat_eq(a902, a903) {
                    8u64
                } else {
                    let a904 = v0;
                    let a905 = 102u64;
                    if nat_eq(a904, a905) {
                        12u64
                    } else {
                        let a906 = v0;
                        let a907 = 110u64;
                        if nat_eq(a906, a907) {
                            10u64
                        } else {
                            let a908 = v0;
                            let a909 = 114u64;
                            if nat_eq(a908, a909) {
                                13u64
                            } else {
                                let a910 = v0;
                                let a911 = 116u64;
                                if nat_eq(a910, a911) {
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

pub fn f246(v0: u64) -> u64 {
    let a912 = v0;
    let a913 = 48u64;
    if nat_lt(a912, a913) {
        16u64
    } else {
        let a914 = v0;
        let a915 = 58u64;
        if nat_lt(a914, a915) {
            let a916 = v0;
            let a917 = 48u64;
            nat_sub(a916, a917)
        } else {
            let a918 = v0;
            let a919 = 65u64;
            if nat_lt(a918, a919) {
                16u64
            } else {
                let a920 = v0;
                let a921 = 71u64;
                if nat_lt(a920, a921) {
                    let a922 = v0;
                    let a923 = 55u64;
                    nat_sub(a922, a923)
                } else {
                    let a924 = v0;
                    let a925 = 97u64;
                    if nat_lt(a924, a925) {
                        16u64
                    } else {
                        let a926 = v0;
                        let a927 = 103u64;
                        if nat_lt(a926, a927) {
                            let a928 = v0;
                            let a929 = 87u64;
                            nat_sub(a928, a929)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f247(v0: Adt103, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt124> {
    let a930 = v3;
    let a931 = 0u64;
    if nat_eq(a930, a931) {
        if f183(v2, 55296u64, 56319u64) {
            Ok(f191(v0, Adt104::C6(Adt105::C3(v2), v4, v5)))
        } else {
            if f183(v2, 56320u64, 57343u64) {
                Ok(f192(Adt90::C6(v1)))
            } else {
                Ok(f191(v0, Adt104::C6(Adt105::C0, f322(v2, v4)?, v5)))
            }
        }
    } else {
        if f183(v2, 56320u64, 57343u64) {
            Ok(f191(v0, Adt104::C6(Adt105::C0, f322({
                let a940 = 65536u64;
                let a941 = {
                    let a938 = {
                        let a934 = {
                            let a932 = v3;
                            let a933 = 55296u64;
                            nat_sub(a932, a933)
                        };
                        let a935 = 1024u64;
                        nat_mul(a934, a935)?
                    };
                    let a939 = {
                        let a936 = v2;
                        let a937 = 56320u64;
                        nat_sub(a936, a937)
                    };
                    nat_add(a938, a939)?
                };
                nat_add(a940, a941)?
            }, v4)?, v5)))
        } else {
            Ok(f192(Adt90::C6(v1)))
        }
    }
}

pub fn f248(v0: u64, v1: u64) -> u64 {
    let m944 = {
        let a942 = f249(v0);
        let a943 = v1;
        index_list(a942, a943)
    };
    match m944 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f249(v0: u64) -> List<u64> {
    let a945 = v0;
    let a946 = 0u64;
    if nat_eq(a945, a946) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a947 = v0;
        let a948 = 1u64;
        if nat_eq(a947, a948) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f250(v0: Adt103, v1: Adt6) -> Adt124 {
    Adt124::C1(f185(v1, match v0.clone() {
        Adt103::C0(_, h949, _) => {
            h949
        }
    }, match v0 {
        Adt103::C0(_, _, h950) => {
            h950
        }
    }))
}

pub fn f251(v0: u64) -> Adt6 {
    let a951 = v0;
    let a952 = 0u64;
    if nat_eq(a951, a952) {
        Adt6::C1(true)
    } else {
        let a953 = v0;
        let a954 = 1u64;
        if nat_eq(a953, a954) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f252(v0: Adt103, v1: Adt106, v2: Adt107) -> Adt124 {
    f191(v0, Adt104::C8(Adt106::C0(match v1.clone() {
        Adt106::C0(h955, _, _, _, _, _, _) => {
            h955
        }
    }, v2, match v1.clone() {
        Adt106::C0(_, _, h956, _, _, _, _) => {
            h956
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, h957, _, _, _) => {
            h957
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, h958, _, _) => {
            h958
        }
    }, match v1.clone() {
        Adt106::C0(_, _, _, _, _, h959, _) => {
            h959
        }
    }, match v1 {
        Adt106::C0(_, _, _, _, _, _, h960) => {
            h960
        }
    })))
}

pub fn f253(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: u64 = {
        let a962 = match v3.clone() {
            Adt106::C0(_, _, _, h961, _, _, _) => {
                h961
            }
        };
        let a963 = 0u64;
        if if nat_eq(a962, a963) {
            let a964 = v2;
            let a965 = 48u64;
            nat_eq(a964, a965)
        } else {
            false
        } {
            0u64
        } else {
            let a967 = match v3.clone() {
                Adt106::C0(_, _, _, h966, _, _, _) => {
                    h966
                }
            };
            let a968 = 1u64;
            nat_add(a967, a968)?
        }
    };
    let v6: i64 = if v4 {
        f209(match v3.clone() {
            Adt106::C0(_, _, _, _, h969, _, _) => {
                h969
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt106::C0(_, _, _, _, h970, _, _) => {
                h970
            }
        }
    };
    let a971 = f323();
    let a972 = v5;
    if nat_lt(a971, a972) {
        Ok(f192(Adt90::C8(v1)))
    } else {
        let m975 = {
            let a973 = 99999i64;
            let a974 = v6;
            compare(a973, a974)
        };
        if match m975 {
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
            Ok(f192(Adt90::C8(v1)))
        } else {
            Ok(f191(v0, f238(match v3.clone() {
                Adt106::C0(h976, _, _, _, _, _, _) => {
                    h976
                }
            }, if v4 {
                Adt107::C4
            } else {
                Adt107::C2
            }, f209({
                let a978 = match v3.clone() {
                    Adt106::C0(_, _, h977, _, _, _, _) => {
                        h977
                    }
                };
                let a979 = 10i64;
                int_mul(a978, a979)?
            }, f239({
                let a980 = v2;
                let a981 = 48u64;
                nat_sub(a980, a981)
            }))?, v5, v6, match v3.clone() {
                Adt106::C0(_, _, _, _, _, h982, _) => {
                    h982
                }
            }, match v3 {
                Adt106::C0(_, _, _, _, _, _, h983) => {
                    h983
                }
            })))
        }
    }
}

pub fn f254(v0: u64) -> bool {
    let a984 = v0;
    let a985 = 101u64;
    if nat_eq(a984, a985) {
        true
    } else {
        let a986 = v0;
        let a987 = 69u64;
        nat_eq(a986, a987)
    }
}

pub fn f255(v0: u64) -> bool {
    f183(v0, 48u64, 57u64)
}

pub fn f256(v0: Adt103, v1: u64, v2: u64, v3: Adt106) -> R<Adt124> {
    let m988 = f184(v3)?;
    match m988 {
        Adt131::C0 => {
            Ok(f192(Adt90::C7(v1)))
        }
        Adt131::C1 => {
            Ok(f192(Adt90::C8(v1)))
        }
        Adt131::C2(v4) => {
            Ok(f189(f185(v4, match v0.clone() {
                Adt103::C0(_, h989, _) => {
                    h989
                }
            }, match v0 {
                Adt103::C0(_, _, h990) => {
                    h990
                }
            }), v1, v2))
        }
    }
}

pub fn f257(v0: Adt103, v1: u64, v2: u64, v3: Adt106, v4: bool) -> R<Adt124> {
    let v5: i64 = f209({
        let a992 = match v3.clone() {
            Adt106::C0(_, _, _, _, _, _, h991) => {
                h991
            }
        };
        let a993 = 10i64;
        int_mul(a992, a993)?
    }, f239({
        let a994 = v2;
        let a995 = 48u64;
        nat_sub(a994, a995)
    }))?;
    let m998 = {
        let a996 = 99999i64;
        let a997 = v5;
        compare(a996, a997)
    };
    if match m998 {
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
        Ok(f192(Adt90::C8(v1)))
    } else {
        Ok(f191(v0, f238(match v3.clone() {
            Adt106::C0(h999, _, _, _, _, _, _) => {
                h999
            }
        }, Adt107::C7, match v3.clone() {
            Adt106::C0(_, _, h1000, _, _, _, _) => {
                h1000
            }
        }, match v3.clone() {
            Adt106::C0(_, _, _, h1001, _, _, _) => {
                h1001
            }
        }, match v3 {
            Adt106::C0(_, _, _, _, h1002, _, _) => {
                h1002
            }
        }, v4, v5)))
    }
}

pub fn f258(p1005: List<u64>, v1: bool) -> bool {
    let m1004 = p1005;
    {
        let mut m1004 = m1004;
        loop {
            let step = {
                let v0 = m1004;
                let m1003 = v0;
                match m1003.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f324(v2)
                        } else {
                            f325(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259() -> u64 {
    64u64
}

pub fn f260(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1006 = f179(Str::lit("type"), v1);
    match m1006 {
        Adt123::C0 => {
            Some(Adt94::C3(v0))
        }
        Adt123::C1(v2) => {
            let m1007 = f176(v2);
            match m1007 {
                None => {
                    Some(Adt94::C3(v0))
                }
                Some(v3) => {
                    let a1008 = v3;
                    let a1009 = Str::lit("object");
                    if equal(a1008, a1009) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f261(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1010 = f179(Str::lit("$schema"), v1);
    match m1010 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v2) => {
            let m1011 = f176(v2);
            match m1011 {
                None => {
                    Some(Adt94::C2(v0))
                }
                Some(v3) => {
                    let a1012 = v3;
                    let a1013 = f326();
                    if equal(a1012, a1013) {
                        None::<Adt94>
                    } else {
                        Some(Adt94::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f262(v0: Adt35) -> R<Option<Adt94>> {
    let m1015 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h1014, _, _) => {
            h1014
        }
    };
    match m1015 {
        None => {
            Ok(None::<Adt94>)
        }
        Some(v1) => {
            let m1018 = f261(match v0.clone() {
                Adt35::C0(h1016, _, _, _, _, _, _, _) => {
                    h1016
                }
            }, match v1.clone() {
                Adt5::C0(h1017) => {
                    h1017
                }
            });
            match m1018 {
                None => {
                    let m1022 = f199(f200(), match v0 {
                        Adt35::C0(h1019, _, _, _, _, _, _, _) => {
                            h1019
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h1020) => {
                            h1020
                        }
                    }), List::cons(Adt125::C0(Adt6::C5(match v1 {
                        Adt5::C0(h1021) => {
                            h1021
                        }
                    }), false, List::<Str>::nil()), List::<Adt125>::nil()), f201())?;
                    match m1022 {
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

pub fn f263(v0: Str, v1: Adt6, v2: Adt125, v3: Adt132, v4: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1023 = v2;
    match m1023 {
        Adt125::C0(v5, v6, v7) => {
            let m1024 = f264(v5.clone());
            match m1024 {
                None => {
                    let m1025 = f265(v5);
                    match m1025 {
                        None => {
                            Ok(Err::<(List<Adt125>, Adt132), Adt94>(Adt94::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f327(v0, v1, match v9 {
                                Adt5::C0(h1026) => {
                                    h1026
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

pub fn f264(v0: Adt6) -> Option<bool> {
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

pub fn f265(v0: Adt6) -> Option<Adt5> {
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

pub fn f266(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    let m1029 = f328(v1.clone(), v2.clone())?;
    match m1029 {
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
            f329(v0, v1, v2, v3)
        }
    }
}

pub fn f267(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: u64, v4: List<Adt112>) -> Adt126 {
    let m1030 = v0;
    match m1030.uncons() {
        None => {
            let a1031 = v3;
            let a1032 = 1u64;
            if nat_eq(a1031, a1032) {
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

pub fn f268(v0: Option<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1033 = v0;
    match m1033 {
        None => {
            Adt126::C0(Adt111::C0, v3)
        }
        Some(v4) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f269(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1034 = f179(Str::lit("anyOf"), v0);
    match m1034 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1035 = f330(v3);
            match m1035 {
                None => {
                    Adt133::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1036 = v4;
                    match m1036.uncons() {
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

pub fn f270(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1037 = f179(Str::lit("oneOf"), v0);
    match m1037 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1038 = f330(v3);
            match m1038 {
                None => {
                    Adt133::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1039 = v4;
                    match m1039.uncons() {
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

pub fn f271(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1040 = f179(Str::lit("not"), v0);
    match m1040 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1, v2.clone()), List::cons(Adt112::C3(v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f272(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1041 = f179(Str::lit("if"), v0.clone());
    match m1041 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            Adt133::C3(List::cons(Adt112::C0(v3, v1.clone(), v2.clone()), List::cons(Adt112::C4(f331(Str::lit("then"), v0.clone()), f331(Str::lit("else"), v0), v1, v2), List::<Adt112>::nil())))
        }
    }
}

pub fn f273(v0: List<Adt6>, v1: Adt6, v2: List<Adt110>, v3: List<Adt112>) -> Adt126 {
    let m1042 = v0;
    match m1042.uncons() {
        None => {
            Adt126::C0(Adt111::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt126::C0(Adt111::C0, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), List::cons(Adt112::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f274(v0: Str) -> R<bool> {
    let m1044 = f228(f62({
        let a1043 = v0;
        utf8_encode(a1043)
    })?)?;
    match m1044 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f275(v0: Adt37) -> bool {
    let m1046 = match v0 {
        Adt37::C0(_, _, h1045) => {
            h1045
        }
    };
    match m1046 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f276(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1048 = match v0.clone() {
        Adt49::C0(_, h1047) => {
            h1047
        }
    };
    match m1048.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h1049, _) => {
                    h1049
                }
            })))
        }
        Some((_, _)) => {
            if f332(match v0.clone() {
                Adt49::C0(_, h1050) => {
                    h1050
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f277(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1054 = {
        let m1053 = match v1.clone() {
            Adt49::C0(h1052, _) => {
                h1052
            }
        };
        match m1053 {
            Adt37::C0(_, h1051, _) => {
                h1051
            }
        }
    };
    match m1054 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1057 = f110(Adt6::C5(match v0.clone() {
                Adt5::C0(h1055) => {
                    h1055
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1056) => {
                    h1056
                }
            }), v2)?;
            match m1057 {
                Adt109::C0 => {
                    f276(v1)
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

pub fn f278(v0: Adt110) -> R<Str> {
    let m1058 = v0;
    match m1058 {
        Adt110::C0(v1) => {
            f333(v1)
        }
        Adt110::C1(v2) => {
            let a1059 = f151(v2, 0i64)?;
            Ok(format_i64(a1059))
        }
    }
}

pub fn f279(v0: u64, v1: Adt6, v2: List<Adt114>, v3: List<u64>) -> R<Adt134> {
    let m1060 = v1;
    match m1060 {
        Adt6::C0 => {
            Ok(Adt134::C1(v2, f154(f334(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt134::C1(v2, f154(if v4 {
                f334(0u64)
            } else {
                f334(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f335(v5.clone())? {
                Ok(Adt134::C1(v2, f154(f336(v5)?, v3)))
            } else {
                Ok(Adt134::C0(Adt90::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt134::C1(v2, f154(f280(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1061 = v0;
            let a1062 = f320();
            if nat_lt(a1061, a1062) {
                Ok(Adt134::C1(f337(91u64, Adt114::C2({
                    let a1063 = v0;
                    let a1064 = 1u64;
                    nat_add(a1063, a1064)?
                }, v7), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1065 = v0;
            let a1066 = f320();
            if nat_lt(a1065, a1066) {
                Ok(Adt134::C1(f337(123u64, Adt114::C4({
                    let a1067 = v0;
                    let a1068 = 1u64;
                    nat_add(a1067, a1068)?
                }, v8), v2), v3))
            } else {
                Ok(Adt134::C0(Adt90::C12))
            }
        }
    }
}

pub fn f280(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1070 = f154(f338(f62({
            let a1069 = v0;
            utf8_encode(a1069)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1071 = List::cons(34u64, List::<u64>::nil());
        append_list(a1070, a1071)
    }))
}

pub fn f281(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f339(match v0.clone() {
        Adt33::C0(_, h1072, _, _) => {
            h1072
        }
    })), List::cons((Str::lit("capabilities"), f340(match v0.clone() {
        Adt33::C0(_, _, h1073, _) => {
            h1073
        }
    })), {
        let a1076 = f341(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1074) => {
                h1074
            }
        });
        let a1077 = f342(match v0 {
            Adt33::C0(h1075, _, _, _) => {
                h1075
            }
        })?;
        append_list(a1076, a1077)
    })))
}

pub fn f282(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f343(match v0.clone() {
        Adt34::C0(_, _, h1078) => {
            h1078
        }
    }))), {
        let a1081 = f341(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1079, _) => {
                h1079
            }
        });
        let a1082 = f342(match v0 {
            Adt34::C0(h1080, _, _) => {
                h1080
            }
        })?;
        append_list(a1081, a1082)
    }))
}

pub fn f283(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1083 = v0;
    match m1083 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1084 = f288(Adt6::C5(v1))?;
            match m1084 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1085 = f265(v2);
                    match m1085 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1086) => {
                                    h1086
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f284(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1088 = f344(match v0.clone() {
        Adt37::C0(h1087, _, _) => {
            h1087
        }
    })?;
    match m1088 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1091 = f345(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1089, _) => {
                        h1089
                    }
                });
                let a1092 = f346(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1090) => {
                        h1090
                    }
                });
                append_list(a1091, a1092)
            })))
        }
    }
}

pub fn f285(v0: Adt140) -> Str {
    let m1093 = v0;
    match m1093 {
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

pub fn f286(v0: Adt32) -> Adt140 {
    let m1094 = v0;
    match m1094 {
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

pub fn f287(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1095 = v0;
    match m1095 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f347(), f348(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f288(v0: Adt6) -> R<Option<Adt6>> {
    f349(f350(), Adt141::C0(v0), List::<Adt142>::nil())
}

pub fn f289(v0: Adt118) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f122())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f351(match v0.clone() {
        Adt118::C0(h1096, _) => {
            h1096
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1099 = match v0.clone() {
            Adt118::C0(_, h1098) => {
                h1098
            }
        };
        match m1099 {
            Adt50::C0(h1097, _, _) => {
                h1097
            }
        }
    })), {
        let a1106 = f352({
            let m1102 = match v0.clone() {
                Adt118::C0(_, h1101) => {
                    h1101
                }
            };
            match m1102 {
                Adt50::C0(_, h1100, _) => {
                    h1100
                }
            }
        });
        let a1107 = f353({
            let m1105 = match v0 {
                Adt118::C0(_, h1104) => {
                    h1104
                }
            };
            match m1105 {
                Adt50::C0(_, _, h1103) => {
                    h1103
                }
            }
        });
        append_list(a1106, a1107)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f290() -> u64 {
    429u64
}

pub fn f291() -> i64 {
    -32602i64
}

pub fn f292() -> i64 {
    -32022i64
}

pub fn f293() -> i64 {
    -32020i64
}

pub fn f294() -> i64 {
    -32601i64
}

pub fn f295(v0: Adt84) -> i64 {
    let m1108 = v0;
    match m1108 {
        Adt84::C0(_) => {
            f291()
        }
        Adt84::C1(_) => {
            f291()
        }
        Adt84::C2(_) => {
            f291()
        }
        Adt84::C3(_) => {
            f291()
        }
        Adt84::C4(_) => {
            f291()
        }
        Adt84::C5(_) => {
            f291()
        }
        Adt84::C6(_) => {
            f354()
        }
        Adt84::C7(_) => {
            f355()
        }
        Adt84::C8(_) => {
            f356()
        }
        Adt84::C9(_) => {
            f354()
        }
    }
}

pub fn f296(v0: Adt120) -> Str {
    let m1109 = v0;
    match m1109 {
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

pub fn f297(v0: Adt97) -> Str {
    let m1110 = v0;
    match m1110 {
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
            let m1111 = v1;
            match m1111 {
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
            let m1112 = v2;
            match m1112 {
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

pub fn f298(v0: List<Str>) -> List<Adt6> {
    f240(f357(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f299(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1124 = f358(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1113, _, _, _, _) => {
                h1113
            }
        });
        let a1125 = {
            let a1122 = f358(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1114, _, _, _) => {
                    h1114
                }
            });
            let a1123 = {
                let a1120 = f358(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1115, _, _) => {
                        h1115
                    }
                });
                let a1121 = {
                    let a1118 = f358(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1116, _) => {
                            h1116
                        }
                    });
                    let a1119 = f358(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1117) => {
                            h1117
                        }
                    });
                    append_list(a1118, a1119)
                };
                append_list(a1120, a1121)
            };
            append_list(a1122, a1123)
        };
        append_list(a1124, a1125)
    })
}

pub fn f300(v0: Adt0, v1: Fn0, v2: u64, v3: Adt127, v4: bool) -> R<Result<Adt116, Adt119>> {
    if v4 {
        f359(v0, v1, v2, f360(match v3.clone() {
            Adt127::C0(_, h1126, _, _) => {
                h1126
            }
        }, match v3.clone() {
            Adt127::C0(_, _, h1127, _) => {
                h1127
            }
        }, match v3 {
            Adt127::C0(_, _, _, h1128) => {
                h1128
            }
        }))
    } else {
        Ok(Err::<Adt116, Adt119>(Adt119::C3(f361(match v3 {
            Adt127::C0(_, h1129, _, _) => {
                h1129
            }
        }))))
    }
}

pub fn f301(v0: Adt4, v1: Adt128) -> bool {
    let m1130 = v1;
    match m1130 {
        Adt128::C0 => {
            true
        }
        Adt128::C1 => {
            f362(match v0 {
                Adt4::C0(_, _, _, _, _, h1131, _) => {
                    h1131
                }
            })
        }
        Adt128::C2 => {
            f362(match v0 {
                Adt4::C0(_, _, _, _, _, h1132, _) => {
                    h1132
                }
            })
        }
        Adt128::C3 => {
            f363(match v0 {
                Adt4::C0(_, _, _, _, h1133, _, _) => {
                    h1133
                }
            })
        }
        Adt128::C4 => {
            f363(match v0 {
                Adt4::C0(_, _, _, _, h1134, _, _) => {
                    h1134
                }
            })
        }
        Adt128::C5 => {
            f363(match v0 {
                Adt4::C0(_, _, _, _, h1135, _, _) => {
                    h1135
                }
            })
        }
        Adt128::C6 => {
            f362(match v0 {
                Adt4::C0(_, _, _, h1136, _, _, _) => {
                    h1136
                }
            })
        }
        Adt128::C7 => {
            f362(match v0 {
                Adt4::C0(_, _, _, h1137, _, _, _) => {
                    h1137
                }
            })
        }
        Adt128::C8 => {
            f364(match v0 {
                Adt4::C0(_, _, h1138, _, _, _, _) => {
                    h1138
                }
            })
        }
        Adt128::C9 => {
            f365(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1139) => {
                    h1139
                }
            }, f366())
        }
        Adt128::C10 => {
            f365(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1140) => {
                    h1140
                }
            }, f366())
        }
        Adt128::C11 => {
            f365(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1141) => {
                    h1141
                }
            }, f366())
        }
    }
}

pub fn f302(v0: Adt137, v1: Adt135) -> Result<Adt127, Adt119> {
    let m1142 = v1;
    match m1142 {
        Adt135::C0 => {
            Err::<Adt127, Adt119>(Adt119::C3(match v0 {
                Adt137::C0(_, h1143, _, _) => {
                    h1143
                }
            }))
        }
        Adt135::C1(v2) => {
            Ok::<Adt127, Adt119>(Adt127::C0(match v0.clone() {
                Adt137::C0(h1144, _, _, _) => {
                    h1144
                }
            }, v2, match v0.clone() {
                Adt137::C0(_, _, h1145, _) => {
                    h1145
                }
            }, match v0 {
                Adt137::C0(_, _, _, h1146) => {
                    h1146
                }
            }))
        }
    }
}

pub fn f303(v0: Option<Adt96>, v1: List<Str>, v2: Adt95, v3: Result<Adt16, Adt120>) -> R<Adt136> {
    let m1147 = v3;
    match m1147 {
        Ok(v5) => {
            Ok(f367(v1, v2.clone(), v5.clone(), f368(v0, f226(match v2.clone() {
                Adt95::C0(_, h1148, _) => {
                    h1148
                }
            }), match v2.clone() {
                Adt95::C0(_, h1149, _) => {
                    h1149
                }
            }, match v5 {
                Adt16::C0(h1150, _, _, _, _, _) => {
                    h1150
                }
            }, f369(match v2 {
                Adt95::C0(_, _, h1151) => {
                    h1151
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt136::C1(Adt119::C0(v4)))
        }
    }
}

pub fn f304(v0: Option<Adt5>) -> Result<Adt16, Adt120> {
    let m1152 = v0;
    match m1152 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C0)
        }
        Some(v1) => {
            let m1153 = f134(Str::lit("_meta"), v1);
            match m1153 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C1)
                }
                Adt123::C1(v2) => {
                    let m1154 = f265(v2);
                    match m1154 {
                        None => {
                            Err::<Adt16, Adt120>(Adt120::C4)
                        }
                        Some(v3) => {
                            f370(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f305(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1155 = v0;
    let a1156 = f371(f306(), f372(List::<Bytes>::nil(), v1));
    append_bytes(a1155, a1156)
}

pub fn f306() -> Bytes {
    Bytes::lit(&[])
}

pub fn f307(p1163: List<u64>, p1164: List<u64>) -> R<bool> {
    let m1162 = (p1163, p1164);
    {
        let mut m1162 = m1162;
        loop {
            let step = {
                let (v0, v1) = m1162;
                let m1157 = v0;
                match m1157.uncons() {
                    None => {
                        let m1158 = v1;
                        match m1158.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m1159 = v1;
                        match m1159.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a1160 = f373(v4)?;
                                let a1161 = f373(v6)?;
                                if nat_eq(a1160, a1161) {
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
                Ok(next) => { m1162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f308(v0: Adt130, v1: List<u64>) -> Adt130 {
    let m1165 = v0;
    match m1165 {
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

pub fn f309(v0: List<u64>) -> List<u64> {
    f315(f374(f315(f374(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f310(p1168: List<u64>) -> bool {
    let m1167 = p1168;
    {
        let mut m1167 = m1167;
        loop {
            let step = {
                let v0 = m1167;
                let m1166 = v0;
                match m1166.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f375(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1167 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Adt128) -> Adt138 {
    let m1169 = v0;
    match m1169 {
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

pub fn f312(p1175: List<u64>, p1176: List<u64>) -> Option<List<u64>> {
    let m1174 = (p1175, p1176);
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let (v0, v1) = m1174;
                let m1170 = v0;
                match m1170.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1171 = v1;
                        match m1171.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1172 = v4;
                                let a1173 = v2;
                                if nat_eq(a1172, a1173) {
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
                Ok(next) => { m1174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f313() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f314() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f315(p1179: List<u64>, p1180: List<u64>) -> List<u64> {
    let m1178 = (p1179, p1180);
    {
        let mut m1178 = m1178;
        loop {
            let step = {
                let (v0, v1) = m1178;
                let m1177 = v0;
                match m1177.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1178 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(p1189: u64, p1190: List<u64>, p1191: List<u64>) -> R<Option<List<u64>>> {
    let m1188 = (p1189, (p1190, p1191));
    {
        let mut m1188 = m1188;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1188;
                let m1181 = v0;
                if m1181 == 0 {
                    let m1182 = v1;
                    match m1182.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1181 - 1;
                    let m1183 = v1;
                    match m1183.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1184 = v7;
                            match m1184.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1185 = v9;
                                    match m1185.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1186 = v11;
                                            match m1186.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1187 = f376(v6, v8, v10, v12, f377(v13.clone()))?;
                                                    match m1187 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f154(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1188 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f317(v0: Adt12, v1: Option<Str>) -> bool {
    let m1192 = v1;
    match m1192 {
        None => {
            false
        }
        Some(v2) => {
            let a1194 = match v0 {
                Adt12::C0(h1193, _, _, _) => {
                    h1193
                }
            };
            let a1195 = v2;
            equal(a1194, a1195)
        }
    }
}

pub fn f318(v0: List<Adt86>, v1: Str) -> R<Adt100> {
    let m1198 = f171(f167(v0, {
        let a1196 = f378();
        let a1197 = f379(v1)?;
        append_list(a1196, a1197)
    })?)?;
    match m1198 {
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
            let m1199 = f223(v2);
            match m1199 {
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

pub fn f319(v0: i64) -> bool {
    let a1206 = {
        let m1202 = {
            let a1200 = v0;
            let a1201 = 0i64;
            compare(a1200, a1201)
        };
        if match m1202 {
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
            let m1205 = {
                let a1203 = 0i64;
                let a1204 = v0;
                compare(a1203, a1204)
            };
            match m1205 {
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
    bool_not(a1206)
}

pub fn f320() -> u64 {
    512u64
}

pub fn f321(v0: Adt103, v1: u64, v2: Str) -> Adt124 {
    let m1208 = match v0.clone() {
        Adt103::C0(_, h1207, _) => {
            h1207
        }
    };
    match m1208.uncons() {
        None => {
            f192(Adt90::C2(v1))
        }
        Some((v3, v4)) => {
            let m1209 = v3;
            match m1209 {
                Adt108::C0(_) => {
                    f192(Adt90::C2(v1))
                }
                Adt108::C1(v6, _) => {
                    Adt124::C1(Adt103::C0(Adt104::C5, List::cons(Adt108::C1(v6, Some(v2)), v4), match v0 {
                        Adt103::C0(_, _, h1210) => {
                            h1210
                        }
                    }))
                }
            }
        }
    }
}

pub fn f322(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1211 = v0;
    let a1212 = 128u64;
    if nat_lt(a1211, a1212) {
        Ok(List::cons(v0, v1))
    } else {
        let a1213 = v0;
        let a1214 = 2048u64;
        if nat_lt(a1213, a1214) {
            Ok(List::cons({
                let a1215 = 128u64;
                let a1216 = f380(v0, 64u64);
                nat_add(a1215, a1216)?
            }, List::cons({
                let a1217 = 192u64;
                let a1218 = f381(v0, 64u64);
                nat_add(a1217, a1218)?
            }, v1)))
        } else {
            let a1219 = v0;
            let a1220 = 65536u64;
            if nat_lt(a1219, a1220) {
                Ok(List::cons({
                    let a1221 = 128u64;
                    let a1222 = f380(v0, 64u64);
                    nat_add(a1221, a1222)?
                }, List::cons({
                    let a1223 = 128u64;
                    let a1224 = f380(f381(v0, 64u64), 64u64);
                    nat_add(a1223, a1224)?
                }, List::cons({
                    let a1225 = 224u64;
                    let a1226 = f381(v0, 4096u64);
                    nat_add(a1225, a1226)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1227 = 128u64;
                    let a1228 = f380(v0, 64u64);
                    nat_add(a1227, a1228)?
                }, List::cons({
                    let a1229 = 128u64;
                    let a1230 = f380(f381(v0, 64u64), 64u64);
                    nat_add(a1229, a1230)?
                }, List::cons({
                    let a1231 = 128u64;
                    let a1232 = f380(f381(v0, 4096u64), 64u64);
                    nat_add(a1231, a1232)?
                }, List::cons({
                    let a1233 = 240u64;
                    let a1234 = f381(v0, 262144u64);
                    nat_add(a1233, a1234)?
                }, v1)))))
            }
        }
    }
}

pub fn f323() -> u64 {
    18u64
}

pub fn f324(v0: u64) -> bool {
    let a1235 = 48u64;
    let a1236 = v0;
    if if nat_le(a1235, a1236) {
        let a1237 = v0;
        let a1238 = 57u64;
        nat_le(a1237, a1238)
    } else {
        false
    } {
        true
    } else {
        let a1239 = 65u64;
        let a1240 = v0;
        if if nat_le(a1239, a1240) {
            let a1241 = v0;
            let a1242 = 90u64;
            nat_le(a1241, a1242)
        } else {
            false
        } {
            true
        } else {
            let a1243 = 97u64;
            let a1244 = v0;
            if if nat_le(a1243, a1244) {
                let a1245 = v0;
                let a1246 = 122u64;
                nat_le(a1245, a1246)
            } else {
                false
            } {
                true
            } else {
                let a1247 = v0;
                let a1248 = 33u64;
                if nat_eq(a1247, a1248) {
                    true
                } else {
                    let a1249 = v0;
                    let a1250 = 35u64;
                    if nat_eq(a1249, a1250) {
                        true
                    } else {
                        let a1251 = v0;
                        let a1252 = 36u64;
                        if nat_eq(a1251, a1252) {
                            true
                        } else {
                            let a1253 = v0;
                            let a1254 = 37u64;
                            if nat_eq(a1253, a1254) {
                                true
                            } else {
                                let a1255 = v0;
                                let a1256 = 38u64;
                                if nat_eq(a1255, a1256) {
                                    true
                                } else {
                                    let a1257 = v0;
                                    let a1258 = 39u64;
                                    if nat_eq(a1257, a1258) {
                                        true
                                    } else {
                                        let a1259 = v0;
                                        let a1260 = 42u64;
                                        if nat_eq(a1259, a1260) {
                                            true
                                        } else {
                                            let a1261 = v0;
                                            let a1262 = 43u64;
                                            if nat_eq(a1261, a1262) {
                                                true
                                            } else {
                                                let a1263 = v0;
                                                let a1264 = 45u64;
                                                if nat_eq(a1263, a1264) {
                                                    true
                                                } else {
                                                    let a1265 = v0;
                                                    let a1266 = 46u64;
                                                    if nat_eq(a1265, a1266) {
                                                        true
                                                    } else {
                                                        let a1267 = v0;
                                                        let a1268 = 94u64;
                                                        if nat_eq(a1267, a1268) {
                                                            true
                                                        } else {
                                                            let a1269 = v0;
                                                            let a1270 = 95u64;
                                                            if nat_eq(a1269, a1270) {
                                                                true
                                                            } else {
                                                                let a1271 = v0;
                                                                let a1272 = 96u64;
                                                                if nat_eq(a1271, a1272) {
                                                                    true
                                                                } else {
                                                                    let a1273 = v0;
                                                                    let a1274 = 124u64;
                                                                    if nat_eq(a1273, a1274) {
                                                                        true
                                                                    } else {
                                                                        let a1275 = v0;
                                                                        let a1276 = 126u64;
                                                                        nat_eq(a1275, a1276)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f325(v0: u64) -> bool {
    let a1277 = 48u64;
    let a1278 = v0;
    if if nat_le(a1277, a1278) {
        let a1279 = v0;
        let a1280 = 57u64;
        nat_le(a1279, a1280)
    } else {
        false
    } {
        true
    } else {
        let a1281 = 65u64;
        let a1282 = v0;
        if if nat_le(a1281, a1282) {
            let a1283 = v0;
            let a1284 = 90u64;
            nat_le(a1283, a1284)
        } else {
            false
        } {
            true
        } else {
            let a1285 = 97u64;
            let a1286 = v0;
            if if nat_le(a1285, a1286) {
                let a1287 = v0;
                let a1288 = 122u64;
                nat_le(a1287, a1288)
            } else {
                false
            } {
                true
            } else {
                let a1289 = v0;
                let a1290 = 95u64;
                if nat_eq(a1289, a1290) {
                    true
                } else {
                    let a1291 = v0;
                    let a1292 = 45u64;
                    if nat_eq(a1291, a1292) {
                        true
                    } else {
                        let a1293 = v0;
                        let a1294 = 46u64;
                        nat_eq(a1293, a1294)
                    }
                }
            }
        }
    }
}

pub fn f326() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f327(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt132, v6: List<Adt125>) -> R<Result<(List<Adt125>, Adt132), Adt94>> {
    let m1295 = f382(v0.clone(), v1, v2.clone());
    match m1295 {
        None => {
            let m1296 = f383(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1296 {
                Adt144::C0(v8) => {
                    Ok(Err::<(List<Adt125>, Adt132), Adt94>(v8))
                }
                Adt144::C1(v9) => {
                    Ok(Ok::<(List<Adt125>, Adt132), Adt94>((f384(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt125>, Adt132), Adt94>(v7))
        }
    }
}

pub fn f328(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt133> {
    Ok(f385(List::cons((Str::lit("type"), f386(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f387(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f388(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f389(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f389(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f389(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f389(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1297 = f390(v0.clone(), v1.clone())?;
        let a1298 = List::cons((Str::lit("required"), f391(v0, v1)), List::<(Str, Adt143)>::nil());
        append_list(a1297, a1298)
    })))))))))
}

pub fn f329(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> R<Adt133> {
    Ok(f392(f393(v0, v1.clone(), v2.clone(), v3.clone()), f392(f394(v1.clone(), v2.clone(), v3.clone()), f392(f395(v1.clone(), v2.clone(), v3.clone()), f392(f396(v1.clone(), v2.clone(), v3.clone())?, f392(f397(v1.clone(), v2.clone(), v3.clone()), f392(f398(Adt113::C0, f269(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f392(f398(Adt113::C1, f270(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f392(f398(Adt113::C2, f271(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f398(Adt113::C3, f272(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f330(v0: Adt6) -> Option<List<Adt6>> {
    let m1299 = v0;
    match m1299 {
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

pub fn f331(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1300 = f179(v0, v1);
    match m1300 {
        Adt123::C0 => {
            None::<Adt6>
        }
        Adt123::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f332(v0: List<Adt50>) -> R<bool> {
    let m1301 = v0;
    match m1301.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f399(v2, match v1 {
                Adt50::C0(h1302, _, _) => {
                    h1302
                }
            })
        }
    }
}

pub fn f333(v0: Str) -> R<Str> {
    let m1305 = {
        let a1304 = f305(f306(), f154(f400(f62({
            let a1303 = v0.clone();
            utf8_encode(a1303)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1304)
    };
    match m1305 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f334(v0: u64) -> List<u64> {
    f249(v0)
}

pub fn f335(v0: Adt7) -> R<bool> {
    let a1323 = {
        let m1309 = {
            let a1307 = f401();
            let a1308 = match v0.clone() {
                Adt7::C0(h1306, _) => {
                    h1306
                }
            };
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
            true
        } else {
            let m1314 = {
                let a1312 = match v0.clone() {
                    Adt7::C0(h1310, _) => {
                        h1310
                    }
                };
                let a1313 = {
                    let a1311 = f401();
                    int_neg(a1311)?
                };
                compare(a1312, a1313)
            };
            if match m1314 {
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
                let m1318 = {
                    let a1316 = 9999i64;
                    let a1317 = match v0.clone() {
                        Adt7::C0(_, h1315) => {
                            h1315
                        }
                    };
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
                    true
                } else {
                    let m1322 = {
                        let a1320 = match v0 {
                            Adt7::C0(_, h1319) => {
                                h1319
                            }
                        };
                        let a1321 = -9999i64;
                        compare(a1320, a1321)
                    };
                    match m1322 {
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
    Ok(bool_not(a1323))
}

pub fn f336(v0: Adt7) -> R<List<u64>> {
    let m1327 = {
        let a1325 = match v0.clone() {
            Adt7::C0(_, h1324) => {
                h1324
            }
        };
        let a1326 = 0i64;
        compare(a1325, a1326)
    };
    if if match m1327 {
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
        let m1331 = {
            let a1329 = 0i64;
            let a1330 = match v0.clone() {
                Adt7::C0(_, h1328) => {
                    h1328
                }
            };
            compare(a1329, a1330)
        };
        match m1331 {
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
        let a1334 = f402(match v0.clone() {
            Adt7::C0(h1332, _) => {
                h1332
            }
        })?;
        let a1335 = List::cons(101u64, f402(match v0 {
            Adt7::C0(_, h1333) => {
                h1333
            }
        })?);
        Ok(append_list(a1334, a1335))
    } else {
        f402(match v0 {
            Adt7::C0(h1336, _) => {
                h1336
            }
        })
    }
}

pub fn f337(v0: u64, v1: Adt114, v2: List<Adt114>) -> List<Adt114> {
    List::cons(Adt114::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f338(p1339: List<u64>, p1340: List<u64>) -> R<List<u64>> {
    let m1338 = (p1339, p1340);
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let (v0, v1) = m1338;
                let m1337 = v0;
                match m1337.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f154(f403(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f339(v0: List<Str>) -> Adt6 {
    Adt6::C4(f298(v0))
}

pub fn f340(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1358 = f358(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1341, _, _, _, _, _, _) => {
                h1341
            }
        });
        let a1359 = {
            let a1356 = f358(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1342, _, _, _, _, _) => {
                    h1342
                }
            });
            let a1357 = {
                let a1354 = f358(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1343, _, _, _, _) => {
                        h1343
                    }
                });
                let a1355 = {
                    let a1352 = f404(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1344, _, _, _) => {
                            h1344
                        }
                    });
                    let a1353 = {
                        let a1350 = f405(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1345, _, _) => {
                                h1345
                            }
                        });
                        let a1351 = {
                            let a1348 = f404(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1346, _) => {
                                    h1346
                                }
                            });
                            let a1349 = f358(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1347) => {
                                    h1347
                                }
                            });
                            append_list(a1348, a1349)
                        };
                        append_list(a1350, a1351)
                    };
                    append_list(a1352, a1353)
                };
                append_list(a1354, a1355)
            };
            append_list(a1356, a1357)
        };
        append_list(a1358, a1359)
    })
}

pub fn f341(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1360 = v1;
    match m1360 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f342(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f70(match v0.clone() {
        Adt10::C0(h1361, _) => {
            h1361
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f406(match v0 {
        Adt10::C0(_, h1362) => {
            h1362
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f343(v0: List<Adt35>) -> List<Adt6> {
    f240(f407(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f344(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1363 = f408(v0, List::<Adt6>::nil())?;
    match m1363 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f240(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f345(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1364 = v1;
    match m1364 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f346(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1365 = v1;
    match m1365 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f347() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f348(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1366, _, _, _, _, _) => {
            h1366
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1367, _, _, _) => {
            h1367
        }
    })), {
        let a1376 = f341(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1368, _, _, _, _) => {
                h1368
            }
        });
        let a1377 = {
            let a1374 = f341(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1369, _, _) => {
                    h1369
                }
            });
            let a1375 = {
                let a1372 = f341(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1370, _) => {
                        h1370
                    }
                });
                let a1373 = f409(match v0 {
                    Adt1::C0(_, _, _, _, _, h1371) => {
                        h1371
                    }
                });
                append_list(a1372, a1373)
            };
            append_list(a1374, a1375)
        };
        append_list(a1376, a1377)
    })))
}

pub fn f349(p1392: u64, p1393: Adt141, p1394: List<Adt142>) -> R<Option<Adt6>> {
    let m1391 = (p1392, (p1393, p1394));
    {
        let mut m1391 = m1391;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1391;
                let m1378 = v0;
                if m1378 == 0 {
                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1378 - 1;
                    let m1379 = v1;
                    match m1379 {
                        Adt141::C0(v4) => {
                            let m1380 = v4.clone();
                            match m1380 {
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
                                    let v8: Str = f410(v7)?;
                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1381 = v9;
                                    match m1381.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v10), List::cons(Adt142::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1382 = v12;
                                    match m1382.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f410({
                                                let (h1383, _) = v13;
                                                h1383
                                            })?;
                                            Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                let (_, h1384) = v13;
                                                h1384
                                            }), List::cons(Adt142::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt141::C1(v16) => {
                            let m1385 = v2;
                            match m1385.uncons() {
                                None => {
                                    Err::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1386 = v17;
                                    match m1386 {
                                        Adt142::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1387 = v20;
                                            match m1387.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C4(f240(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0(v22), List::cons(Adt142::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt142::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1388 = v26;
                                            match m1388.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C1(Adt6::C5(f242(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f410({
                                                        let (h1389, _) = v28;
                                                        h1389
                                                    })?;
                                                    Ok::<(u64, (Adt141, List<Adt142>)), R<Option<Adt6>>>((v3, (Adt141::C0({
                                                        let (_, h1390) = v28;
                                                        h1390
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
                Ok(next) => { m1391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350() -> u64 {
    67108864u64
}

pub fn f351(v0: Adt19) -> Adt6 {
    let m1395 = v0;
    match m1395 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f352(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1396 = v0;
    match m1396 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f353(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1397 = v0;
    match m1397 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f354() -> i64 {
    -32603i64
}

pub fn f355() -> i64 {
    429i64
}

pub fn f356() -> i64 {
    -32021i64
}

pub fn f357(p1400: List<Str>, p1401: List<Adt6>) -> List<Adt6> {
    let m1399 = (p1400, p1401);
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let (v0, v1) = m1399;
                let m1398 = v0;
                match m1398.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1399 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1402 = v1;
    match m1402 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1403) => {
                    h1403
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f359(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt120>) -> R<Result<Adt116, Adt119>> {
    let m1404 = v3;
    match m1404 {
        Ok(v5) => {
            f411(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt116, Adt119>(Adt119::C0(v4)))
        }
    }
}

pub fn f360(v0: Adt128, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt120> {
    let m1405 = v0;
    match m1405 {
        Adt128::C0 => {
            f412(v1, v2)
        }
        Adt128::C1 => {
            f413(v1, v2)
        }
        Adt128::C2 => {
            f414(v1, v2)
        }
        Adt128::C3 => {
            f415(v1, v2)
        }
        Adt128::C4 => {
            f416(v1, v2)
        }
        Adt128::C5 => {
            f417(v1, v2)
        }
        Adt128::C6 => {
            f418(v1, v2)
        }
        Adt128::C7 => {
            f419(v1, v2)
        }
        Adt128::C8 => {
            f420(v1, v2)
        }
        Adt128::C9 => {
            f421(v1, v2)
        }
        Adt128::C10 => {
            f422(v1, v2)
        }
        Adt128::C11 => {
            f423(v1, v2)
        }
    }
}

pub fn f361(v0: Adt128) -> Str {
    let m1406 = v0;
    match m1406 {
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

pub fn f362(v0: Option<Adt8>) -> bool {
    let m1407 = v0;
    match m1407 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f363(v0: Option<Adt9>) -> bool {
    let m1408 = v0;
    match m1408 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f364(v0: Option<Adt5>) -> bool {
    let m1409 = v0;
    match m1409 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f365(v0: Option<Adt5>, v1: Str) -> bool {
    let m1410 = v0;
    match m1410 {
        None => {
            false
        }
        Some(v2) => {
            f180(v1, v2)
        }
    }
}

pub fn f366() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f367(v0: List<Str>, v1: Adt95, v2: Adt16, v3: Option<Adt97>) -> Adt136 {
    let m1411 = v3;
    match m1411 {
        None => {
            f424(v1, v2.clone(), f425(match v2 {
                Adt16::C0(h1412, _, _, _, _, _) => {
                    h1412
                }
            }, v0))
        }
        Some(v4) => {
            Adt136::C1(Adt119::C2(v4))
        }
    }
}

pub fn f368(v0: Option<Adt96>, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1413 = v0;
    match m1413 {
        None => {
            Ok(None::<Adt97>)
        }
        Some(v5) => {
            let m1415 = match v5.clone() {
                Adt96::C0(_, _, _, h1414, _) => {
                    h1414
                }
            };
            match m1415 {
                None => {
                    f426(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f369(v0: Option<Adt5>) -> Adt5 {
    let m1416 = v0;
    match m1416 {
        None => {
            f152()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f370(v0: Adt5) -> Result<Adt16, Adt120> {
    let m1417 = f427(v0.clone());
    match m1417 {
        Adt145::C0 => {
            f428(v0.clone(), f134(f429(), v0.clone()), f134(f430(), v0))
        }
        Adt145::C1 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt145::C2 => {
            Err::<Adt16, Adt120>(Adt120::C3)
        }
    }
}

pub fn f371(p1422: Bytes, p1423: List<Bytes>) -> Bytes {
    let m1421 = (p1422, p1423);
    {
        let mut m1421 = m1421;
        loop {
            let step = {
                let (v0, v1) = m1421;
                let m1418 = v1;
                match m1418.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1419 = v2;
                            let a1420 = v0;
                            append_bytes(a1419, a1420)
                        }, v3))
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

pub fn f372(p1426: List<Bytes>, p1427: List<u64>) -> List<Bytes> {
    let m1425 = (p1426, p1427);
    {
        let mut m1425 = m1425;
        loop {
            let step = {
                let (v0, v1) = m1425;
                let m1424 = v1;
                match m1424.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f431(f432(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1425 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: u64) -> R<u64> {
    let a1428 = 64u64;
    let a1429 = v0;
    if if nat_lt(a1428, a1429) {
        let a1430 = v0;
        let a1431 = 91u64;
        nat_lt(a1430, a1431)
    } else {
        false
    } {
        let a1432 = v0;
        let a1433 = 32u64;
        nat_add(a1432, a1433)
    } else {
        Ok(v0)
    }
}

pub fn f374(p1436: List<u64>) -> List<u64> {
    let m1435 = p1436;
    {
        let mut m1435 = m1435;
        loop {
            let step = {
                let v0 = m1435;
                let m1434 = v0;
                match m1434.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f433(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1435 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375(v0: u64) -> bool {
    let a1437 = v0;
    let a1438 = 9u64;
    if nat_eq(a1437, a1438) {
        true
    } else {
        let a1439 = 31u64;
        let a1440 = v0;
        if nat_lt(a1439, a1440) {
            let a1441 = v0;
            let a1442 = 127u64;
            nat_lt(a1441, a1442)
        } else {
            false
        }
    }
}

pub fn f376(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1443 = f434(v0)?;
    match m1443 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1444 = f434(v1)?;
            match m1444 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1445 = v2;
                    let a1446 = 61u64;
                    if nat_eq(a1445, a1446) {
                        f435(v5, v6, v4, v3)
                    } else {
                        let m1447 = f434(v2)?;
                        match m1447 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1448 = v3;
                                let a1449 = 61u64;
                                if nat_eq(a1448, a1449) {
                                    f436(v5, v6, v7, v4)
                                } else {
                                    let m1450 = f434(v3)?;
                                    match m1450 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f437(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: List<u64>) -> bool {
    let m1451 = v0;
    match m1451.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f378() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f379(v0: Str) -> R<List<u64>> {
    f62({
        let a1452 = v0;
        utf8_encode(a1452)
    })
}

pub fn f380(v0: u64, v1: u64) -> u64 {
    let a1453 = v0;
    let a1454 = v1;
    let a1455 = 0u64;
    nat_rem(a1453, a1454, a1455)
}

pub fn f381(v0: u64, v1: u64) -> u64 {
    let a1456 = v0;
    let a1457 = v1;
    let a1458 = 0u64;
    nat_quot(a1456, a1457, a1458)
}

pub fn f382(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1459 = f438(f439(), v2.clone());
    match m1459 {
        None => {
            let m1460 = f440(f441(), v2.clone());
            match m1460 {
                None => {
                    f442(v0, v1, v2)
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

pub fn f383(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt132) -> R<Adt144> {
    let m1461 = f179(Str::lit("x-mcp-header"), v1.clone());
    match m1461 {
        Adt123::C0 => {
            Ok(Adt144::C1(v4))
        }
        Adt123::C1(v5) => {
            f443(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f384(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt125>) -> List<Adt125> {
    f444(Str::lit("properties"), v0.clone(), v1, v2, f444(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f445(Str::lit("items"), v0.clone(), f445(Str::lit("additionalProperties"), v0.clone(), f446(Str::lit("allOf"), v0.clone(), f446(Str::lit("anyOf"), v0.clone(), f446(Str::lit("oneOf"), v0.clone(), f445(Str::lit("not"), v0.clone(), f445(Str::lit("if"), v0.clone(), f445(Str::lit("then"), v0.clone(), f445(Str::lit("else"), v0, v3)))))))))))
}

pub fn f385(p1468: List<(Str, Adt143)>) -> Adt133 {
    let m1467 = p1468;
    {
        let mut m1467 = m1467;
        loop {
            let step = {
                let v0 = m1467;
                let m1462 = v0;
                match m1462.uncons() {
                    None => {
                        Err::<List<(Str, Adt143)>, Adt133>(Adt133::C3(List::<Adt112>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1464 = {
                            let (_, h1463) = v1;
                            h1463
                        };
                        match m1464 {
                            Adt143::C0 => {
                                Ok::<List<(Str, Adt143)>, Adt133>(v2)
                            }
                            Adt143::C1 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C0({
                                    let (h1465, _) = v1;
                                    h1465
                                }))
                            }
                            Adt143::C2 => {
                                Err::<List<(Str, Adt143)>, Adt133>(Adt133::C1({
                                    let (h1466, _) = v1;
                                    h1466
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1469 = f179(Str::lit("type"), v0);
    match m1469 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1470 = f176(v2.clone());
            match m1470 {
                None => {
                    let m1471 = f330(v2);
                    match m1471 {
                        None => {
                            Ok(Adt143::C2)
                        }
                        Some(v4) => {
                            f449(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f447(v3.clone()) {
                        if f448(v3, v1)? {
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

pub fn f387(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1472 = f179(Str::lit("enum"), v0);
    match m1472 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1473 = f330(v2);
            match m1473 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v3) => {
                    f450(v3, v1)
                }
            }
        }
    }
}

pub fn f388(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt143> {
    let m1474 = f179(Str::lit("const"), v0);
    match m1474 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v2) => {
            let m1475 = f451(v2, v1)?;
            match m1475 {
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

pub fn f389(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt143> {
    let m1476 = f179(v0, v2);
    match m1476 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1477 = f452(v4);
            match m1477 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1478 = f452(v3);
                    match m1478 {
                        None => {
                            Ok(Adt143::C0)
                        }
                        Some(v6) => {
                            if f453(v1, f454(v6, v5)?) {
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

pub fn f390(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt143)>> {
    let m1479 = v1;
    match m1479 {
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
            Ok(List::cons((Str::lit("minLength"), f455(Str::lit("minLength"), true, v0.clone(), f456(v4.clone())?)?), List::cons((Str::lit("maxLength"), f455(Str::lit("maxLength"), false, v0, f456(v4)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f455(Str::lit("minItems"), true, v0.clone(), f457(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f455(Str::lit("maxItems"), false, v0, f457(v5, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f455(Str::lit("minProperties"), true, v0.clone(), f458(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f455(Str::lit("maxProperties"), false, v0, f458(v6, 0u64)?)?), List::<(Str, Adt143)>::nil())))
        }
    }
}

pub fn f391(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt143 {
    let m1480 = f179(Str::lit("required"), v0);
    match m1480 {
        Adt123::C0 => {
            Adt143::C0
        }
        Adt123::C1(v2) => {
            let m1481 = f330(v2);
            match m1481 {
                None => {
                    Adt143::C2
                }
                Some(v3) => {
                    let m1482 = f265(v1);
                    match m1482 {
                        None => {
                            Adt143::C0
                        }
                        Some(v4) => {
                            f459(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f392(v0: Adt133, v1: Adt133) -> Adt133 {
    let m1483 = v0.clone();
    match m1483 {
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
            let m1484 = v1.clone();
            match m1484 {
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
                        let a1485 = v5;
                        let a1486 = v9;
                        append_list(a1485, a1486)
                    })
                }
            }
        }
    }
}

pub fn f393(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>) -> Adt133 {
    let m1487 = f179(Str::lit("$ref"), v1);
    match m1487 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v4) => {
            let m1488 = f176(v4);
            match m1488 {
                None => {
                    Adt133::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1489 = f460(v0, v5.clone());
                    match m1489 {
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

pub fn f394(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1490 = f179(Str::lit("properties"), v0);
    match m1490 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1491 = f265(v3);
            match m1491 {
                None => {
                    Adt133::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1492 = f265(v1);
                    match m1492 {
                        None => {
                            Adt133::C3(List::<Adt112>::nil())
                        }
                        Some(v5) => {
                            Adt133::C3(f461(match v4 {
                                Adt5::C0(h1493) => {
                                    h1493
                                }
                            }, match v5 {
                                Adt5::C0(h1494) => {
                                    h1494
                                }
                            }, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f395(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1495 = f179(Str::lit("additionalProperties"), v0.clone());
    match m1495 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1496 = f265(v1);
            match m1496 {
                None => {
                    Adt133::C3(List::<Adt112>::nil())
                }
                Some(v4) => {
                    let m1497 = f264(v3.clone());
                    match m1497 {
                        None => {
                            Adt133::C3(f464(match v4 {
                                Adt5::C0(h1499) => {
                                    h1499
                                }
                            }, f463(v0), v3, v2, List::<Adt112>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt133::C3(List::<Adt112>::nil())
                            } else {
                                if f462(match v4 {
                                    Adt5::C0(h1498) => {
                                        h1498
                                    }
                                }, f463(v0)) {
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

pub fn f396(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> R<Adt133> {
    let m1500 = f179(Str::lit("items"), v0);
    match m1500 {
        Adt123::C0 => {
            Ok(Adt133::C3(List::<Adt112>::nil()))
        }
        Adt123::C1(v3) => {
            let m1501 = f330(v1);
            match m1501 {
                None => {
                    Ok(Adt133::C3(List::<Adt112>::nil()))
                }
                Some(v4) => {
                    Ok(Adt133::C3(f465(v4, 0u64, v3, v2, List::<Adt112>::nil())?))
                }
            }
        }
    }
}

pub fn f397(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt110>) -> Adt133 {
    let m1502 = f179(Str::lit("allOf"), v0);
    match m1502 {
        Adt123::C0 => {
            Adt133::C3(List::<Adt112>::nil())
        }
        Adt123::C1(v3) => {
            let m1503 = f330(v3);
            match m1503 {
                None => {
                    Adt133::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1504 = v4.clone();
                    match m1504.uncons() {
                        None => {
                            Adt133::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt133::C3(f466(v4, v1, v2, List::<Adt112>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f398(v0: Adt113, v1: Adt133, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt110>) -> Adt133 {
    let m1505 = v1.clone();
    match m1505 {
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
            let m1506 = v8;
            match m1506.uncons() {
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

pub fn f399(p1511: List<Adt50>, p1512: Adt7) -> R<bool> {
    let m1510 = (p1511, p1512);
    {
        let mut m1510 = m1510;
        loop {
            let step = {
                let (v0, v1) = m1510;
                let m1507 = v0;
                match m1507.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f467(f454(match v2.clone() {
                            Adt50::C0(h1508, _, _) => {
                                h1508
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1509, _, _) => {
                                    h1509
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1510 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(p1519: List<u64>, p1520: List<u64>) -> List<u64> {
    let m1518 = (p1519, p1520);
    {
        let mut m1518 = m1518;
        loop {
            let step = {
                let (v0, v1) = m1518;
                let m1513 = v0;
                match m1513.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1514 = v2;
                        let a1515 = 126u64;
                        if nat_eq(a1514, a1515) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1516 = v2;
                            let a1517 = 47u64;
                            if nat_eq(a1516, a1517) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401() -> i64 {
    999999999999999999i64
}

pub fn f402(v0: i64) -> R<List<u64>> {
    let m1523 = {
        let a1521 = v0;
        let a1522 = 0i64;
        compare(a1521, a1522)
    };
    if match m1523 {
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
        Ok(List::cons(45u64, f468(20u64, {
            let a1524 = v0;
            int_neg(a1524)?
        }, List::<u64>::nil())?))
    } else {
        f468(20u64, v0, List::<u64>::nil())
    }
}

pub fn f403(v0: u64) -> R<List<u64>> {
    let a1525 = v0;
    let a1526 = 34u64;
    if nat_eq(a1525, a1526) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1527 = v0;
        let a1528 = 92u64;
        if nat_eq(a1527, a1528) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1529 = v0;
            let a1530 = 8u64;
            if nat_eq(a1529, a1530) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1531 = v0;
                let a1532 = 9u64;
                if nat_eq(a1531, a1532) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1533 = v0;
                    let a1534 = 10u64;
                    if nat_eq(a1533, a1534) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1535 = v0;
                        let a1536 = 12u64;
                        if nat_eq(a1535, a1536) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1537 = v0;
                            let a1538 = 13u64;
                            if nat_eq(a1537, a1538) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1539 = v0;
                                let a1540 = 32u64;
                                if nat_lt(a1539, a1540) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f469(f381(v0, 16u64))?, List::cons(f469(f380(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f404(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1541 = v1;
    match m1541 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f470(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f405(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1542 = v0;
    match m1542 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1545 = f346(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1543, _) => {
                        h1543
                    }
                });
                let a1546 = f346(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1544) => {
                        h1544
                    }
                });
                append_list(a1545, a1546)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f406(v0: Adt11) -> Str {
    let m1547 = v0;
    match m1547 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f407(p1550: List<Adt35>, p1551: List<Adt6>) -> List<Adt6> {
    let m1549 = (p1550, p1551);
    {
        let mut m1549 = m1549;
        loop {
            let step = {
                let (v0, v1) = m1549;
                let m1548 = v0;
                match m1548.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f471(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1549 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f408(p1555: List<Adt38>, p1556: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1554 = (p1555, p1556);
    {
        let mut m1554 = m1554;
        loop {
            let step = {
                let (v0, v1) = m1554;
                let m1552 = v0;
                match m1552.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1553 = f472(v2)?;
                        match m1553 {
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
                Ok(next) => { m1554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f409(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1557 = v0;
    match m1557 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f473(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f410(v0: Str) -> R<Str> {
    let m1560 = {
        let a1559 = f305(f306(), f154(f474(f62({
            let a1558 = v0;
            utf8_encode(a1558)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1559)
    };
    match m1560 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f411(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt116, Adt119>> {
    if f475(f476(v3.clone())) {
        Ok(Ok::<Adt116, Adt119>(f477(v0.clone(), Adt32::C0(f478(v0)))))
    } else {
        Ok(f479(v0.clone(), v3.clone(), f480(match v0 {
            Adt0::C0(_, _, _, _, _, h1561) => {
                h1561
            }
        }, v2, v1, v3)?))
    }
}

pub fn f412(v0: Adt16, _: Adt5) -> Result<Adt14, Adt120> {
    Ok::<Adt14, Adt120>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f413(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1562 = f481(Str::lit("cursor"), v1);
    match m1562 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f414(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1563 = f482(Str::lit("name"), v1.clone());
    match m1563 {
        Ok(v2) => {
            let m1564 = f483(Str::lit("arguments"), v1.clone());
            match m1564 {
                Ok(v3) => {
                    let m1565 = f483(Str::lit("inputResponses"), v1.clone());
                    match m1565 {
                        Ok(v4) => {
                            let m1566 = f481(Str::lit("requestState"), v1);
                            match m1566 {
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

pub fn f415(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1567 = f481(Str::lit("cursor"), v1);
    match m1567 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f416(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1568 = f482(Str::lit("uri"), v1.clone());
    match m1568 {
        Ok(v2) => {
            let m1569 = f483(Str::lit("inputResponses"), v1.clone());
            match m1569 {
                Ok(v3) => {
                    let m1570 = f481(Str::lit("requestState"), v1);
                    match m1570 {
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

pub fn f417(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1571 = f481(Str::lit("cursor"), v1);
    match m1571 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f418(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1572 = f481(Str::lit("cursor"), v1);
    match m1572 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f419(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1573 = f482(Str::lit("name"), v1.clone());
    match m1573 {
        Ok(v2) => {
            let m1574 = f484(Str::lit("arguments"), v1.clone());
            match m1574 {
                Ok(v3) => {
                    let m1575 = f483(Str::lit("inputResponses"), v1.clone());
                    match m1575 {
                        Ok(v4) => {
                            let m1576 = f481(Str::lit("requestState"), v1);
                            match m1576 {
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

pub fn f420(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1577 = f485(Str::lit("ref"), v1.clone());
    match m1577 {
        Ok(v2) => {
            let m1578 = f486(v2);
            match m1578 {
                Ok(v3) => {
                    let m1579 = f485(Str::lit("argument"), v1.clone());
                    match m1579 {
                        Ok(v4) => {
                            let m1580 = f487(v4);
                            match m1580 {
                                Ok(v5) => {
                                    let m1581 = f488(v1);
                                    match m1581 {
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

pub fn f421(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1582 = f482(Str::lit("taskId"), v1);
    match m1582 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f422(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1583 = f482(Str::lit("taskId"), v1.clone());
    match m1583 {
        Ok(v2) => {
            let m1584 = f485(Str::lit("inputResponses"), v1);
            match m1584 {
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

pub fn f423(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt120> {
    let m1585 = f482(Str::lit("taskId"), v1);
    match m1585 {
        Ok(v2) => {
            Ok::<Adt14, Adt120>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt120>(v3)
        }
    }
}

pub fn f424(v0: Adt95, v1: Adt16, v2: Option<Adt119>) -> Adt136 {
    let m1586 = v2;
    match m1586 {
        None => {
            Adt136::C0(Adt137::C0(match v0.clone() {
                Adt95::C0(h1587, _, _) => {
                    h1587
                }
            }, match v0.clone() {
                Adt95::C0(_, h1588, _) => {
                    h1588
                }
            }, v1, f369(match v0 {
                Adt95::C0(_, _, h1589) => {
                    h1589
                }
            })))
        }
        Some(v3) => {
            Adt136::C1(v3)
        }
    }
}

pub fn f425(v0: Str, v1: List<Str>) -> Option<Adt119> {
    f489(f490(v0.clone(), v1.clone()), v0, v1)
}

pub fn f426(v0: Adt96, v1: Adt138, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt97>> {
    let m1591 = match v0.clone() {
        Adt96::C0(h1590, _, _, _, _) => {
            h1590
        }
    };
    match m1591 {
        None => {
            Ok(Some(Adt97::C0))
        }
        Some(v5) => {
            let a1592 = v5;
            let a1593 = v3;
            if equal(a1592, a1593) {
                let m1595 = match v0.clone() {
                    Adt96::C0(_, h1594, _, _, _) => {
                        h1594
                    }
                };
                match m1595 {
                    None => {
                        Ok(Some(Adt97::C1))
                    }
                    Some(v6) => {
                        let a1596 = v6;
                        let a1597 = v2;
                        if equal(a1596, a1597) {
                            f491(f492(match v0.clone() {
                                Adt96::C0(_, _, h1598, _, _) => {
                                    h1598
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt96::C0(_, _, _, _, h1599) => {
                                    h1599
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

pub fn f427(v0: Adt5) -> Adt145 {
    f493(f134(f429(), v0.clone()), f134(f430(), v0))
}

pub fn f428(v0: Adt5, v1: Adt123, v2: Adt123) -> Result<Adt16, Adt120> {
    let m1600 = v1;
    match m1600 {
        Adt123::C0 => {
            Err::<Adt16, Adt120>(Adt120::C2)
        }
        Adt123::C1(v3) => {
            let m1601 = v2;
            match m1601 {
                Adt123::C0 => {
                    Err::<Adt16, Adt120>(Adt120::C3)
                }
                Adt123::C1(v4) => {
                    f494(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f429() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f430() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f431(p1610: Bytes, p1611: List<Bytes>) -> List<Bytes> {
    let m1609 = (p1610, p1611);
    {
        let mut m1609 = m1609;
        loop {
            let step = {
                let (v0, v1) = m1609;
                let m1602 = v1;
                match m1602.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1605 = {
                            let a1603 = v2.clone();
                            length_bytes(a1603)
                        };
                        let a1606 = {
                            let a1604 = v0.clone();
                            length_bytes(a1604)
                        };
                        if nat_eq(a1605, a1606) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1607 = v2;
                                let a1608 = v0;
                                append_bytes(a1607, a1608)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f432(v0: u64) -> Bytes {
    let m1615 = {
        let a1612 = f495();
        let a1613 = v0;
        let a1614 = 1u64;
        slice_bytes(a1612, a1613, a1614)
    };
    match m1615 {
        None => {
            f306()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f433(v0: u64) -> bool {
    let a1616 = v0;
    let a1617 = 9u64;
    if nat_eq(a1616, a1617) {
        true
    } else {
        let a1618 = v0;
        let a1619 = 32u64;
        nat_eq(a1618, a1619)
    }
}

pub fn f434(v0: u64) -> R<Option<u64>> {
    let a1620 = 64u64;
    let a1621 = v0;
    if if nat_lt(a1620, a1621) {
        let a1622 = v0;
        let a1623 = 91u64;
        nat_lt(a1622, a1623)
    } else {
        false
    } {
        Ok(Some({
            let a1624 = v0;
            let a1625 = 65u64;
            nat_sub(a1624, a1625)
        }))
    } else {
        let a1626 = 96u64;
        let a1627 = v0;
        if if nat_lt(a1626, a1627) {
            let a1628 = v0;
            let a1629 = 123u64;
            nat_lt(a1628, a1629)
        } else {
            false
        } {
            Ok(Some({
                let a1630 = v0;
                let a1631 = 71u64;
                nat_sub(a1630, a1631)
            }))
        } else {
            let a1632 = 47u64;
            let a1633 = v0;
            if if nat_lt(a1632, a1633) {
                let a1634 = v0;
                let a1635 = 58u64;
                nat_lt(a1634, a1635)
            } else {
                false
            } {
                Ok(Some({
                    let a1636 = v0;
                    let a1637 = 4u64;
                    nat_add(a1636, a1637)?
                }))
            } else {
                let a1638 = v0;
                let a1639 = 43u64;
                if nat_eq(a1638, a1639) {
                    Ok(Some(62u64))
                } else {
                    let a1640 = v0;
                    let a1641 = 47u64;
                    if nat_eq(a1640, a1641) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f435(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1642 = v3;
        let a1643 = 61u64;
        if nat_eq(a1642, a1643) {
            let a1644 = f496(v1, 16u64);
            let a1645 = 0u64;
            nat_eq(a1644, a1645)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f497(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f436(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1646 = f496(v2, 4u64);
        let a1647 = 0u64;
        nat_eq(a1646, a1647)
    } else {
        false
    } {
        Ok(Some(f498(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f437(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1650 = {
            let a1648 = v0;
            let a1649 = 4u64;
            nat_mul(a1648, a1649)?
        };
        let a1651 = f499(v1, 16u64);
        nat_add(a1650, a1651)?
    }, List::cons({
        let a1654 = {
            let a1652 = f496(v1, 16u64);
            let a1653 = 16u64;
            nat_mul(a1652, a1653)?
        };
        let a1655 = f499(v2, 4u64);
        nat_add(a1654, a1655)?
    }, List::cons({
        let a1658 = {
            let a1656 = f496(v2, 4u64);
            let a1657 = 64u64;
            nat_mul(a1656, a1657)?
        };
        let a1659 = v3;
        nat_add(a1658, a1659)?
    }, List::<u64>::nil()))))
}

pub fn f438(p1663: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1662 = p1663;
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let v0 = m1662;
                let m1660 = v0;
                match m1660.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1661 = f179(v2.clone(), v1.clone());
                        match m1661 {
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
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f439() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f440(p1670: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1669 = p1670;
    {
        let mut m1669 = m1669;
        loop {
            let step = {
                let v0 = m1669;
                let m1664 = v0;
                match m1664.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1666 = f179({
                            let (h1665, _) = v2.clone();
                            h1665
                        }, v1.clone());
                        match m1666 {
                            Adt123::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt123::C1(v4) => {
                                if f500({
                                    let (_, h1667) = v2;
                                    h1667
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1668, _) = v2;
                                        h1668
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f442(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt94> {
    let m1671 = f179(Str::lit("$ref"), v2);
    match m1671 {
        Adt123::C0 => {
            None::<Adt94>
        }
        Adt123::C1(v3) => {
            let m1672 = f176(v3);
            match m1672 {
                None => {
                    None::<Adt94>
                }
                Some(v4) => {
                    let m1673 = f460(v1, v4.clone());
                    match m1673 {
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

pub fn f443(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    if v3 {
        let m1674 = f176(v2);
        match m1674 {
            None => {
                Ok(Adt144::C0(Adt94::C7(v0)))
            }
            Some(v6) => {
                f501(v0, v1, v6.clone(), f144(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt144::C0(Adt94::C10(v0)))
    }
}

pub fn f444(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt125>) -> List<Adt125> {
    let m1675 = f179(v0.clone(), v1);
    match m1675 {
        Adt123::C0 => {
            v4
        }
        Adt123::C1(v5) => {
            let m1676 = f265(v5);
            match m1676 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1677 = v0;
                    let a1678 = Str::lit("properties");
                    if equal(a1677, a1678) {
                        f502(match v6 {
                            Adt5::C0(h1679) => {
                                h1679
                            }
                        }, v2, v3, v4)
                    } else {
                        f503(match v6 {
                            Adt5::C0(h1680) => {
                                h1680
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f445(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1681 = f179(v0, v1);
    match m1681 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            List::cons(Adt125::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f446(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt125>) -> List<Adt125> {
    let m1682 = f179(v0, v1);
    match m1682 {
        Adt123::C0 => {
            v2
        }
        Adt123::C1(v3) => {
            let m1683 = f330(v3);
            match m1683 {
                None => {
                    v2
                }
                Some(v4) => {
                    f504(v4, v2)
                }
            }
        }
    }
}

pub fn f447(v0: Str) -> bool {
    let a1684 = v0.clone();
    let a1685 = Str::lit("null");
    if equal(a1684, a1685) {
        true
    } else {
        let a1686 = v0.clone();
        let a1687 = Str::lit("boolean");
        if equal(a1686, a1687) {
            true
        } else {
            let a1688 = v0.clone();
            let a1689 = Str::lit("object");
            if equal(a1688, a1689) {
                true
            } else {
                let a1690 = v0.clone();
                let a1691 = Str::lit("array");
                if equal(a1690, a1691) {
                    true
                } else {
                    let a1692 = v0.clone();
                    let a1693 = Str::lit("number");
                    if equal(a1692, a1693) {
                        true
                    } else {
                        let a1694 = v0.clone();
                        let a1695 = Str::lit("string");
                        if equal(a1694, a1695) {
                            true
                        } else {
                            let a1696 = v0;
                            let a1697 = Str::lit("integer");
                            equal(a1696, a1697)
                        }
                    }
                }
            }
        }
    }
}

pub fn f448(v0: Str, v1: Adt6) -> R<bool> {
    let a1698 = v0.clone();
    let a1699 = f505(v1.clone());
    if equal(a1698, a1699) {
        Ok(true)
    } else {
        let a1700 = v0;
        let a1701 = Str::lit("integer");
        if equal(a1700, a1701) {
            let m1702 = f452(v1);
            match m1702 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f506(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f449(p1705: List<Adt6>, v1: Adt6, p1706: bool) -> R<Adt143> {
    let m1704 = (p1705, p1706);
    {
        let mut m1704 = m1704;
        loop {
            let step = {
                let (v0, v2) = m1704;
                let m1703 = v0;
                match m1703.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f447(f507(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt143>>((v4, if v2 {
                                true
                            } else {
                                f448(f507(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt143>>(Ok(Adt143::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(p1710: List<Adt6>, v1: Adt6) -> R<Adt143> {
    let m1709 = p1710;
    {
        let mut m1709 = m1709;
        loop {
            let step = {
                let v0 = m1709;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt143>>(Ok(Adt143::C1))
                    }
                    Some((v2, v3)) => {
                        let m1708 = f451(v2, v1.clone())?;
                        match m1708 {
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
                Ok(next) => { m1709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f451(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f508(f509(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f452(v0: Adt6) -> Option<Adt7> {
    let m1711 = v0;
    match m1711 {
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

pub fn f453(v0: u64, v1: Adt147) -> bool {
    let a1712 = v0;
    let a1713 = 0u64;
    if nat_eq(a1712, a1713) {
        let a1714 = f510(v1);
        bool_not(a1714)
    } else {
        let a1715 = v0;
        let a1716 = 1u64;
        if nat_eq(a1715, a1716) {
            let a1717 = f467(v1);
            bool_not(a1717)
        } else {
            let a1718 = v0;
            let a1719 = 2u64;
            if nat_eq(a1718, a1719) {
                f467(v1)
            } else {
                f510(v1)
            }
        }
    }
}

pub fn f454(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f511(v0.clone(), v1.clone(), f512(match v0 {
        Adt7::C0(h1720, _) => {
            h1720
        }
    }), f512(match v1 {
        Adt7::C0(h1721, _) => {
            h1721
        }
    }))
}

pub fn f455(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt143> {
    let m1722 = f179(v0, v2);
    match m1722 {
        Adt123::C0 => {
            Ok(Adt143::C0)
        }
        Adt123::C1(v4) => {
            let m1723 = f513(v4);
            match m1723 {
                None => {
                    Ok(Adt143::C2)
                }
                Some(v5) => {
                    let m1726 = {
                        let a1724 = v5;
                        let a1725 = 0i64;
                        compare(a1724, a1725)
                    };
                    if match m1726 {
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
                            if f510(f514(f151(v3, 0i64)?, v5)) {
                                Ok(Adt143::C1)
                            } else {
                                Ok(Adt143::C0)
                            }
                        } else {
                            if f467(f514(f151(v3, 0i64)?, v5)) {
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

pub fn f456(v0: Str) -> R<u64> {
    f515(f62({
        let a1727 = v0;
        utf8_encode(a1727)
    })?, 0u64)
}

pub fn f457(p1732: List<Adt6>, p1733: u64) -> R<u64> {
    let m1731 = (p1732, p1733);
    {
        let mut m1731 = m1731;
        loop {
            let step = {
                let (v0, v1) = m1731;
                let m1728 = v0;
                match m1728.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1729 = v1;
                            let a1730 = 1u64;
                            nat_add(a1729, a1730)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p1738: List<(Str, Adt6)>, p1739: u64) -> R<u64> {
    let m1737 = (p1738, p1739);
    {
        let mut m1737 = m1737;
        loop {
            let step = {
                let (v0, v1) = m1737;
                let m1734 = v0;
                match m1734.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1735 = v1;
                            let a1736 = 1u64;
                            nat_add(a1735, a1736)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(p1743: List<Adt6>, v1: Adt5) -> Adt143 {
    let m1742 = p1743;
    {
        let mut m1742 = m1742;
        loop {
            let step = {
                let v0 = m1742;
                let m1740 = v0;
                match m1740.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt143>(Adt143::C0)
                    }
                    Some((v2, v3)) => {
                        let m1741 = f176(v2);
                        match m1741 {
                            None => {
                                Err::<List<Adt6>, Adt143>(Adt143::C2)
                            }
                            Some(v4) => {
                                if f180(v4, v1.clone()) {
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
                Ok(next) => { m1742 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1744 = v1.clone();
    let a1745 = Str::lit("#");
    if equal(a1744, a1745) {
        Some(v0)
    } else {
        let m1746 = f265(v0);
        match m1746 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1747 = f134(Str::lit("$defs"), v2);
                match m1747 {
                    Adt123::C0 => {
                        None::<Adt6>
                    }
                    Adt123::C1(v3) => {
                        let m1748 = f265(v3);
                        match m1748 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f516(v1, match v4 {
                                    Adt5::C0(h1749) => {
                                        h1749
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

pub fn f461(p1756: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt110>, p1757: List<Adt112>) -> List<Adt112> {
    let m1755 = (p1756, p1757);
    {
        let mut m1755 = m1755;
        loop {
            let step = {
                let (v0, v3) = m1755;
                let m1750 = v0;
                match m1750.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1752 = f179({
                            let (h1751, _) = v4.clone();
                            h1751
                        }, v1.clone());
                        match m1752 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, v3))
                            }
                            Adt123::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0({
                                    let (_, h1753) = v4;
                                    h1753
                                }, v6, List::cons(Adt110::C0({
                                    let (h1754, _) = v4;
                                    h1754
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(p1762: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1761 = p1762;
    {
        let mut m1761 = m1761;
        loop {
            let step = {
                let v0 = m1761;
                let m1758 = v0;
                match m1758.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1760 = f179({
                            let (h1759, _) = v2;
                            h1759
                        }, v1.clone());
                        match m1760 {
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
                Ok(next) => { m1761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1763 = f179(Str::lit("properties"), v0);
    match m1763 {
        Adt123::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt123::C1(v1) => {
            let m1764 = f265(v1);
            match m1764 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1765) => {
                            h1765
                        }
                    }
                }
            }
        }
    }
}

pub fn f464(p1772: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt110>, p1773: List<Adt112>) -> List<Adt112> {
    let m1771 = (p1772, p1773);
    {
        let mut m1771 = m1771;
        loop {
            let step = {
                let (v0, v4) = m1771;
                let m1766 = v0;
                match m1766.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1768 = f179({
                            let (h1767, _) = v5.clone();
                            h1767
                        }, v1.clone());
                        match m1768 {
                            Adt123::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt112>), List<Adt112>>((v6, List::cons(Adt112::C0(v2.clone(), {
                                    let (_, h1769) = v5;
                                    h1769
                                }, List::cons(Adt110::C0({
                                    let (h1770, _) = v5;
                                    h1770
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
                Ok(next) => { m1771 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(p1778: List<Adt6>, p1779: u64, v2: Adt6, v3: List<Adt110>, p1780: List<Adt112>) -> R<List<Adt112>> {
    let m1777 = (p1778, (p1779, p1780));
    {
        let mut m1777 = m1777;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1777;
                let m1774 = v0;
                match m1774.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt112>)), R<List<Adt112>>>((v6, ({
                            let a1775 = v1;
                            let a1776 = 1u64;
                            nat_add(a1775, a1776)?
                        }, List::cons(Adt112::C0(v2.clone(), v5, List::cons(Adt110::C1(v1), v3.clone())), v4))))
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

pub fn f466(p1783: List<Adt6>, v1: Adt6, v2: List<Adt110>, p1784: List<Adt112>) -> List<Adt112> {
    let m1782 = (p1783, p1784);
    {
        let mut m1782 = m1782;
        loop {
            let step = {
                let (v0, v3) = m1782;
                let m1781 = v0;
                match m1781.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt112>), List<Adt112>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt112>), List<Adt112>>((v5, List::cons(Adt112::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: Adt147) -> bool {
    let m1785 = v0;
    match m1785 {
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

pub fn f468(p1797: u64, p1798: i64, p1799: List<u64>) -> R<List<u64>> {
    let m1796 = (p1797, (p1798, p1799));
    {
        let mut m1796 = m1796;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1796;
                let m1786 = v0;
                if m1786 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1786 - 1;
                    let v4: i64 = {
                        let a1787 = v1;
                        let a1788 = 10i64;
                        let a1789 = 0i64;
                        int_quot(a1787, a1788, a1789)?
                    };
                    let v5: i64 = {
                        let a1790 = v1;
                        let a1791 = 10i64;
                        let a1792 = 0i64;
                        int_rem(a1790, a1791, a1792)
                    };
                    let m1795 = {
                        let a1793 = v4;
                        let a1794 = 1i64;
                        compare(a1793, a1794)
                    };
                    if match m1795 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f517(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f517(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1796 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: u64) -> R<u64> {
    let a1800 = v0;
    let a1801 = 10u64;
    if nat_lt(a1800, a1801) {
        let a1802 = 48u64;
        let a1803 = v0;
        nat_add(a1802, a1803)
    } else {
        let a1804 = 87u64;
        let a1805 = v0;
        nat_add(a1804, a1805)
    }
}

pub fn f470(v0: Adt8) -> Adt6 {
    Adt6::C5(f346(Str::lit("listChanged"), match v0 {
        Adt8::C0(h1806) => {
            h1806
        }
    }))
}

pub fn f471(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h1807, _, _, _, _, _, _, _) => {
            h1807
        }
    })), {
        let a1825 = f341(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h1808, _, _, _, _, _, _) => {
                h1808
            }
        });
        let a1826 = {
            let a1823 = f409(match v0.clone() {
                Adt35::C0(_, _, h1809, _, _, _, _, _) => {
                    h1809
                }
            });
            let a1824 = {
                let a1821 = f341(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h1810, _, _, _, _) => {
                        h1810
                    }
                });
                let a1822 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m1813 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h1812, _, _, _) => {
                            h1812
                        }
                    };
                    match m1813 {
                        Adt5::C0(h1811) => {
                            h1811
                        }
                    }
                })), {
                    let a1819 = f358(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h1814, _, _) => {
                            h1814
                        }
                    });
                    let a1820 = {
                        let a1817 = f518(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h1815, _) => {
                                h1815
                            }
                        });
                        let a1818 = f519(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h1816) => {
                                h1816
                            }
                        });
                        append_list(a1817, a1818)
                    };
                    append_list(a1819, a1820)
                });
                append_list(a1821, a1822)
            };
            append_list(a1823, a1824)
        };
        append_list(a1825, a1826)
    }))
}

pub fn f472(v0: Adt38) -> R<Option<Adt6>> {
    let m1827 = v0;
    match m1827 {
        Adt38::C0(v1) => {
            Ok(Some(f520(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f521(Str::lit("image"), match v2.clone() {
                Adt42::C0(h1828, _, _, _) => {
                    h1828
                }
            }, match v2.clone() {
                Adt42::C0(_, h1829, _, _) => {
                    h1829
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h1830, _) => {
                    h1830
                }
            }, match v2 {
                Adt42::C0(_, _, _, h1831) => {
                    h1831
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f521(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h1832, _, _, _) => {
                    h1832
                }
            }, match v3.clone() {
                Adt43::C0(_, h1833, _, _) => {
                    h1833
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h1834, _) => {
                    h1834
                }
            }, match v3 {
                Adt43::C0(_, _, _, h1835) => {
                    h1835
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f522(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f523(v5)))
        }
    }
}

pub fn f473(v0: List<Adt2>) -> List<Adt6> {
    f240(f524(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f474(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f525(v0, 0u64, v1)
}

pub fn f475(v0: Adt128) -> bool {
    let m1836 = v0;
    match m1836 {
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

pub fn f476(v0: Adt14) -> Adt128 {
    let m1837 = v0;
    match m1837 {
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

pub fn f477(v0: Adt0, v1: Adt32) -> Adt116 {
    Adt116::C0(v1, Adt117::C0(Some(match v0 {
        Adt0::C0(h1838, _, _, _, _, _) => {
            h1838
        }
    }), None::<Adt5>), List::<Adt118>::nil())
}

pub fn f478(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h1839, _, _) => {
            h1839
        }
    }, f132(), match v0.clone() {
        Adt0::C0(_, _, h1840, _, _, _) => {
            h1840
        }
    }, match v0 {
        Adt0::C0(_, h1841, _, _, _, _) => {
            h1841
        }
    })
}

pub fn f479(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt116, Adt119> {
    let m1842 = v2;
    match m1842 {
        Ok(v4) => {
            if f526(f476(v1.clone()), v4.clone()) {
                f527(v0, v1, v4)
            } else {
                Err::<Adt116, Adt119>(Adt119::C4(f528()))
            }
        }
        Err(v3) => {
            Err::<Adt116, Adt119>(Adt119::C4(v3))
        }
    }
}

pub fn f480(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m1843 = v3.clone();
    match m1843 {
        Adt14::C0(_) => {
            let c1845: Fn0 = v2;
            c1845.apply(v3)
        }
        Adt14::C1(_) => {
            let c1846: Fn0 = v2;
            c1846.apply(v3)
        }
        Adt14::C2(_) => {
            if f529(v0, v1) {
                let c1844: Fn0 = v2;
                c1844.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c1847: Fn0 = v2;
            c1847.apply(v3)
        }
        Adt14::C4(_) => {
            let c1848: Fn0 = v2;
            c1848.apply(v3)
        }
        Adt14::C5(_) => {
            let c1849: Fn0 = v2;
            c1849.apply(v3)
        }
        Adt14::C6(_) => {
            let c1850: Fn0 = v2;
            c1850.apply(v3)
        }
        Adt14::C7(_) => {
            let c1851: Fn0 = v2;
            c1851.apply(v3)
        }
        Adt14::C8(_) => {
            let c1852: Fn0 = v2;
            c1852.apply(v3)
        }
        Adt14::C9(_) => {
            let c1853: Fn0 = v2;
            c1853.apply(v3)
        }
        Adt14::C10(_) => {
            let c1854: Fn0 = v2;
            c1854.apply(v3)
        }
        Adt14::C11(_) => {
            let c1855: Fn0 = v2;
            c1855.apply(v3)
        }
    }
}

pub fn f481(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt120> {
    let m1856 = f134(v0.clone(), v1);
    match m1856 {
        Adt123::C0 => {
            Ok::<Option<Str>, Adt120>(None::<Str>)
        }
        Adt123::C1(v2) => {
            let m1857 = f176(v2);
            match m1857 {
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

pub fn f482(v0: Str, v1: Adt5) -> Result<Str, Adt120> {
    let m1858 = f134(v0.clone(), v1);
    match m1858 {
        Adt123::C0 => {
            Err::<Str, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1859 = f176(v2);
            match m1859 {
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

pub fn f483(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt120> {
    let m1860 = f134(v0.clone(), v1);
    match m1860 {
        Adt123::C0 => {
            Ok::<Option<Adt5>, Adt120>(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m1861 = f265(v2);
            match m1861 {
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

pub fn f484(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt120> {
    let m1862 = f134(v0.clone(), v1);
    match m1862 {
        Adt123::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt120>(None::<List<(Str, Str)>>)
        }
        Adt123::C1(v2) => {
            let m1863 = f265(v2);
            match m1863 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt120>(Adt120::C8(v0))
                }
                Some(v3) => {
                    let m1865 = f530(v0, match v3 {
                        Adt5::C0(h1864) => {
                            h1864
                        }
                    });
                    match m1865 {
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

pub fn f485(v0: Str, v1: Adt5) -> Result<Adt5, Adt120> {
    let m1866 = f134(v0.clone(), v1);
    match m1866 {
        Adt123::C0 => {
            Err::<Adt5, Adt120>(Adt120::C7(v0))
        }
        Adt123::C1(v2) => {
            let m1867 = f265(v2);
            match m1867 {
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

pub fn f486(v0: Adt5) -> Result<Adt25, Adt120> {
    let m1868 = f482(Str::lit("type"), v0.clone());
    match m1868 {
        Ok(v1) => {
            let a1869 = v1.clone();
            let a1870 = Str::lit("ref/prompt");
            if equal(a1869, a1870) {
                let m1871 = f482(Str::lit("name"), v0.clone());
                match m1871 {
                    Ok(v2) => {
                        let m1872 = f481(Str::lit("title"), v0);
                        match m1872 {
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
                let a1873 = v1;
                let a1874 = Str::lit("ref/resource");
                if equal(a1873, a1874) {
                    let m1875 = f482(Str::lit("uri"), v0);
                    match m1875 {
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

pub fn f487(v0: Adt5) -> Result<Adt27, Adt120> {
    let m1876 = f482(Str::lit("name"), v0.clone());
    match m1876 {
        Ok(v1) => {
            let m1877 = f482(Str::lit("value"), v0);
            match m1877 {
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

pub fn f488(v0: Adt5) -> Result<Option<Adt28>, Adt120> {
    let m1878 = f134(Str::lit("context"), v0);
    match m1878 {
        Adt123::C0 => {
            Ok::<Option<Adt28>, Adt120>(None::<Adt28>)
        }
        Adt123::C1(v1) => {
            let m1879 = f265(v1);
            match m1879 {
                None => {
                    Err::<Option<Adt28>, Adt120>(Adt120::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m1880 = f484(Str::lit("arguments"), v2);
                    match m1880 {
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

pub fn f489(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt119> {
    if v0 {
        None::<Adt119>
    } else {
        Some(Adt119::C1(v1, v2))
    }
}

pub fn f490(v0: Str, p1885: List<Str>) -> bool {
    let m1884 = p1885;
    {
        let mut m1884 = m1884;
        loop {
            let step = {
                let v1 = m1884;
                let m1881 = v1;
                match m1881.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a1882 = v2;
                        let a1883 = v0.clone();
                        if equal(a1882, a1883) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
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

pub fn f491(v0: Option<Adt97>, v1: List<Adt99>, v2: Adt5) -> R<Option<Adt97>> {
    let m1886 = v0;
    match m1886 {
        None => {
            f531(v1, f532(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f492(v0: Option<Str>, v1: Adt138, v2: Adt5) -> Option<Adt97> {
    let m1887 = v1.clone();
    match m1887 {
        Adt138::C0 => {
            None::<Adt97>
        }
        Adt138::C1 => {
            f533(v0, f534(v1, v2))
        }
        Adt138::C2 => {
            f533(v0, f534(v1, v2))
        }
        Adt138::C3 => {
            f533(v0, f534(v1, v2))
        }
    }
}

pub fn f493(v0: Adt123, v1: Adt123) -> Adt145 {
    let m1888 = v0;
    match m1888 {
        Adt123::C0 => {
            Adt145::C1
        }
        Adt123::C1(_) => {
            let m1889 = v1;
            match m1889 {
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

pub fn f494(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt120> {
    let m1890 = f176(v1);
    match m1890 {
        None => {
            Err::<Adt16, Adt120>(Adt120::C5)
        }
        Some(v3) => {
            let m1891 = f535(v2);
            match m1891 {
                None => {
                    Err::<Adt16, Adt120>(Adt120::C6)
                }
                Some(v4) => {
                    let m1892 = f483(f536(), v0.clone());
                    match m1892 {
                        Ok(v6) => {
                            let m1893 = f537(v0.clone());
                            match m1893 {
                                Ok(v8) => {
                                    let m1894 = f538(v0.clone());
                                    match m1894 {
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

pub fn f495() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f496(v0: u64, v1: u64) -> u64 {
    let a1895 = v0;
    let a1896 = v1;
    let a1897 = 0u64;
    nat_rem(a1895, a1896, a1897)
}

pub fn f497(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1900 = {
            let a1898 = v0;
            let a1899 = 4u64;
            nat_mul(a1898, a1899)?
        };
        let a1901 = f499(v1, 16u64);
        nat_add(a1900, a1901)?
    }, List::<u64>::nil()))
}

pub fn f498(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1904 = {
            let a1902 = v0;
            let a1903 = 4u64;
            nat_mul(a1902, a1903)?
        };
        let a1905 = f499(v1, 16u64);
        nat_add(a1904, a1905)?
    }, List::cons({
        let a1908 = {
            let a1906 = f496(v1, 16u64);
            let a1907 = 16u64;
            nat_mul(a1906, a1907)?
        };
        let a1909 = f499(v2, 4u64);
        nat_add(a1908, a1909)?
    }, List::<u64>::nil())))
}

pub fn f499(v0: u64, v1: u64) -> u64 {
    let a1910 = v0;
    let a1911 = v1;
    let a1912 = 0u64;
    nat_quot(a1910, a1911, a1912)
}

pub fn f500(v0: u64, v1: Adt6) -> bool {
    let a1913 = v0;
    let a1914 = 0u64;
    if nat_eq(a1913, a1914) {
        f539(v1)
    } else {
        let a1915 = v0;
        let a1916 = 1u64;
        if nat_eq(a1915, a1916) {
            f540(v1)
        } else {
            let a1917 = v0;
            let a1918 = 2u64;
            if nat_eq(a1917, a1918) {
                f541(v1)
            } else {
                let a1919 = v0;
                let a1920 = 3u64;
                if nat_eq(a1919, a1920) {
                    f542(v1)
                } else {
                    let a1921 = v0;
                    let a1922 = 4u64;
                    if nat_eq(a1921, a1922) {
                        f543(v1)
                    } else {
                        let a1923 = v0;
                        let a1924 = 5u64;
                        if nat_eq(a1923, a1924) {
                            f544(v1)
                        } else {
                            let a1925 = v0;
                            let a1926 = 6u64;
                            if nat_eq(a1925, a1926) {
                                f545(v1)
                            } else {
                                let a1927 = v0;
                                let a1928 = 7u64;
                                if nat_eq(a1927, a1928) {
                                    f546(v1)
                                } else {
                                    f547(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f501(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt132) -> R<Adt144> {
    let m1929 = v3.clone();
    match m1929.uncons() {
        None => {
            Ok(Adt144::C0(Adt94::C8(v0)))
        }
        Some((_, _)) => {
            if f258(v3.clone(), true) {
                let m1930 = f548(v1);
                match m1930 {
                    None => {
                        Ok(Adt144::C0(Adt94::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f549(v3)?;
                        if f550(v9.clone(), match v5.clone() {
                            Adt132::C0(_, h1931) => {
                                h1931
                            }
                        }) {
                            Ok(Adt144::C0(Adt94::C12(v0, v2)))
                        } else {
                            Ok(Adt144::C1(Adt132::C0(List::cons(Adt12::C0(v0, v2, f551(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt132::C0(h1932, _) => {
                                    h1932
                                }
                            }), List::cons(v9, match v5 {
                                Adt132::C0(_, h1933) => {
                                    h1933
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

pub fn f502(p1938: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1939: List<Adt125>) -> List<Adt125> {
    let m1937 = (p1938, p1939);
    {
        let mut m1937 = m1937;
        loop {
            let step = {
                let (v0, v3) = m1937;
                let m1934 = v0;
                match m1934.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                            let (_, h1935) = v4;
                            h1935
                        }, v1, List::cons({
                            let (h1936, _) = v4;
                            h1936
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(p1943: List<(Str, Adt6)>, p1944: List<Adt125>) -> List<Adt125> {
    let m1942 = (p1943, p1944);
    {
        let mut m1942 = m1942;
        loop {
            let step = {
                let (v0, v1) = m1942;
                let m1940 = v0;
                match m1940.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0({
                            let (_, h1941) = v2;
                            h1941
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1942 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(p1947: List<Adt6>, p1948: List<Adt125>) -> List<Adt125> {
    let m1946 = (p1947, p1948);
    {
        let mut m1946 = m1946;
        loop {
            let step = {
                let (v0, v1) = m1946;
                let m1945 = v0;
                match m1945.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v3, List::cons(Adt125::C0(v2, false, List::<Str>::nil()), v1)))
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

pub fn f505(v0: Adt6) -> Str {
    let m1949 = v0;
    match m1949 {
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

pub fn f506(v0: Adt7) -> R<bool> {
    if f319(match v0.clone() {
        Adt7::C0(h1950, _) => {
            h1950
        }
    }) {
        Ok(true)
    } else {
        f552(40u64, match v0.clone() {
            Adt7::C0(h1951, _) => {
                h1951
            }
        }, match v0 {
            Adt7::C0(_, h1952) => {
                h1952
            }
        })
    }
}

pub fn f507(v0: Adt6) -> Str {
    let m1953 = f176(v0);
    match m1953 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f508(p1960: u64, p1961: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1959 = (p1960, p1961);
    {
        let mut m1959 = m1959;
        loop {
            let step = {
                let (v0, v1) = m1959;
                let m1954 = v0;
                if m1954 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1954 - 1;
                    let m1955 = v1;
                    match m1955.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1958 = f553({
                                let (h1956, _) = v3;
                                h1956
                            }, {
                                let (_, h1957) = v3;
                                h1957
                            }, v4)?;
                            match m1958 {
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
                Ok(next) => { m1959 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509() -> u64 {
    67108864u64
}

pub fn f510(v0: Adt147) -> bool {
    let m1962 = v0;
    match m1962 {
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

pub fn f511(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1963 = v2;
    match m1963 {
        Adt147::C0 => {
            let m1965 = v3;
            match m1965 {
                Adt147::C0 => {
                    Ok(f554(Adt147::C0, f555(f556(match v0.clone() {
                        Adt7::C0(h1966, _) => {
                            h1966
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1967) => {
                            h1967
                        }
                    }, f556(match v1.clone() {
                        Adt7::C0(h1968, _) => {
                            h1968
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1969) => {
                            h1969
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
            let m1964 = v3;
            match m1964 {
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
            let m1970 = v3;
            match m1970 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f555(match v0.clone() {
                        Adt7::C0(h1971, _) => {
                            h1971
                        }
                    }, match v0 {
                        Adt7::C0(_, h1972) => {
                            h1972
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1973, _) => {
                            h1973
                        }
                    }, match v1 {
                        Adt7::C0(_, h1974) => {
                            h1974
                        }
                    })
                }
            }
        }
    }
}

pub fn f512(v0: i64) -> Adt147 {
    f514(v0, 0i64)
}

pub fn f513(v0: Adt6) -> Option<i64> {
    let m1975 = v0;
    match m1975 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f319(match v2.clone() {
                Adt7::C0(_, h1976) => {
                    h1976
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1977, _) => {
                        h1977
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

pub fn f514(v0: i64, v1: i64) -> Adt147 {
    let m1980 = {
        let a1978 = v0;
        let a1979 = v1;
        compare(a1978, a1979)
    };
    if match m1980 {
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
        let m1983 = {
            let a1981 = v1;
            let a1982 = v0;
            compare(a1981, a1982)
        };
        if match m1983 {
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

pub fn f515(p1992: List<u64>, p1993: u64) -> R<u64> {
    let m1991 = (p1992, p1993);
    {
        let mut m1991 = m1991;
        loop {
            let step = {
                let (v0, v1) = m1991;
                let m1984 = v0;
                match m1984.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1985 = v2;
                        let a1986 = 128u64;
                        if if nat_lt(a1985, a1986) {
                            true
                        } else {
                            let a1987 = 191u64;
                            let a1988 = v2;
                            nat_lt(a1987, a1988)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1989 = v1;
                                let a1990 = 1u64;
                                nat_add(a1989, a1990)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f516(v0: Str, p2011: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2010 = p2011;
    {
        let mut m2010 = m2010;
        loop {
            let step = {
                let v1 = m2010;
                let m1994 = v1;
                match m1994.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1998 = v0.clone();
                        let a1999 = {
                            let a1996 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1995, _) = v2.clone();
                                h1995
                            }, List::<Str>::nil()));
                            let a1997 = Str::lit("");
                            join(a1996, a1997)
                        };
                        if equal(a1998, a1999) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2000) = v2;
                                h2000
                            }))
                        } else {
                            let m2002 = f265({
                                let (_, h2001) = v2.clone();
                                h2001
                            });
                            match m2002 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2003 = f134(Str::lit("$anchor"), v4);
                                    match m2003 {
                                        Adt123::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt123::C1(v5) => {
                                            let m2004 = f176(v5);
                                            match m2004 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2007 = v0.clone();
                                                    let a2008 = {
                                                        let a2005 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2006 = Str::lit("");
                                                        join(a2005, a2006)
                                                    };
                                                    if equal(a2007, a2008) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2009) = v2;
                                                            h2009
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
                Ok(next) => { m2010 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: i64) -> u64 {
    let m2014 = {
        let a2012 = v0;
        let a2013 = 1i64;
        compare(a2012, a2013)
    };
    if match m2014 {
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
        let m2017 = {
            let a2015 = v0;
            let a2016 = 2i64;
            compare(a2015, a2016)
        };
        if match m2017 {
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
            let m2020 = {
                let a2018 = v0;
                let a2019 = 3i64;
                compare(a2018, a2019)
            };
            if match m2020 {
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
                let m2023 = {
                    let a2021 = v0;
                    let a2022 = 4i64;
                    compare(a2021, a2022)
                };
                if match m2023 {
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
                    let m2026 = {
                        let a2024 = v0;
                        let a2025 = 5i64;
                        compare(a2024, a2025)
                    };
                    if match m2026 {
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
                        let m2029 = {
                            let a2027 = v0;
                            let a2028 = 6i64;
                            compare(a2027, a2028)
                        };
                        if match m2029 {
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
                            let m2032 = {
                                let a2030 = v0;
                                let a2031 = 7i64;
                                compare(a2030, a2031)
                            };
                            if match m2032 {
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
                                let m2035 = {
                                    let a2033 = v0;
                                    let a2034 = 8i64;
                                    compare(a2033, a2034)
                                };
                                if match m2035 {
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
                                    let m2038 = {
                                        let a2036 = v0;
                                        let a2037 = 9i64;
                                        compare(a2036, a2037)
                                    };
                                    if match m2038 {
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

pub fn f518(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2039 = v0;
    match m2039 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2051 = f341(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2040, _, _, _, _) => {
                        h2040
                    }
                });
                let a2052 = {
                    let a2049 = f346(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2041, _, _, _) => {
                            h2041
                        }
                    });
                    let a2050 = {
                        let a2047 = f346(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2042, _, _) => {
                                h2042
                            }
                        });
                        let a2048 = {
                            let a2045 = f346(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2043, _) => {
                                    h2043
                                }
                            });
                            let a2046 = f346(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2044) => {
                                    h2044
                                }
                            });
                            append_list(a2045, a2046)
                        };
                        append_list(a2047, a2048)
                    };
                    append_list(a2049, a2050)
                };
                append_list(a2051, a2052)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f519(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f358(Str::lit("_meta"), v0)
}

pub fn f520(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2053, _, _) => {
            h2053
        }
    })), {
        let a2056 = f557(match v0.clone() {
            Adt39::C0(_, h2054, _) => {
                h2054
            }
        });
        let a2057 = f519(match v0 {
            Adt39::C0(_, _, h2055) => {
                h2055
            }
        });
        append_list(a2056, a2057)
    })))
}

pub fn f521(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2058 = f557(v3);
        let a2059 = f519(v4);
        append_list(a2058, a2059)
    }))))
}

pub fn f522(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2083 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2060, _, _, _, _, _, _, _, _) => {
                h2060
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2084 = {
            let a2081 = f341(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2061, _, _, _, _, _, _, _) => {
                    h2061
                }
            });
            let a2082 = {
                let a2079 = f409(match v0.clone() {
                    Adt44::C0(_, _, h2062, _, _, _, _, _, _) => {
                        h2062
                    }
                });
                let a2080 = {
                    let a2077 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2063, _, _, _, _, _) => {
                            h2063
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2078 = {
                        let a2075 = f341(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2064, _, _, _, _) => {
                                h2064
                            }
                        });
                        let a2076 = {
                            let a2073 = f341(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2065, _, _, _) => {
                                    h2065
                                }
                            });
                            let a2074 = {
                                let a2071 = f557(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2066, _, _) => {
                                        h2066
                                    }
                                });
                                let a2072 = {
                                    let a2069 = f558(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2067, _) => {
                                            h2067
                                        }
                                    })?;
                                    let a2070 = f519(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2068) => {
                                            h2068
                                        }
                                    });
                                    append_list(a2069, a2070)
                                };
                                append_list(a2071, a2072)
                            };
                            append_list(a2073, a2074)
                        };
                        append_list(a2075, a2076)
                    };
                    append_list(a2077, a2078)
                };
                append_list(a2079, a2080)
            };
            append_list(a2081, a2082)
        };
        append_list(a2083, a2084)
    }))
}

pub fn f523(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f559(match v0.clone() {
        Adt45::C0(h2085, _, _) => {
            h2085
        }
    })), {
        let a2088 = f557(match v0.clone() {
            Adt45::C0(_, h2086, _) => {
                h2086
            }
        });
        let a2089 = f519(match v0 {
            Adt45::C0(_, _, h2087) => {
                h2087
            }
        });
        append_list(a2088, a2089)
    })))
}

pub fn f524(p2092: List<Adt2>, p2093: List<Adt6>) -> List<Adt6> {
    let m2091 = (p2092, p2093);
    {
        let mut m2091 = m2091;
        loop {
            let step = {
                let (v0, v1) = m2091;
                let m2090 = v0;
                match m2090.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f560(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2091 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(p2098: List<u64>, p2099: u64, p2100: List<u64>) -> List<u64> {
    let m2097 = (p2098, (p2099, p2100));
    {
        let mut m2097 = m2097;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2097;
                let m2094 = v0;
                match m2094.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f561(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt148 = f562(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt148::C0(h2095, _) => {
                                h2095
                            }
                        }, match v5 {
                            Adt148::C0(_, h2096) => {
                                h2096
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2097 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526(v0: Adt128, v1: Adt32) -> bool {
    let m2101 = v1;
    match m2101 {
        Adt32::C0(_) => {
            f563(v0, Adt128::C0)
        }
        Adt32::C1(_) => {
            f563(v0, Adt128::C1)
        }
        Adt32::C2(_) => {
            f563(v0, Adt128::C2)
        }
        Adt32::C3(_) => {
            f563(v0, Adt128::C2)
        }
        Adt32::C4(_) => {
            f563(v0, Adt128::C3)
        }
        Adt32::C5(_) => {
            f563(v0, Adt128::C4)
        }
        Adt32::C6(_) => {
            f563(v0, Adt128::C5)
        }
        Adt32::C7(_) => {
            f563(v0, Adt128::C6)
        }
        Adt32::C8(_) => {
            f563(v0, Adt128::C7)
        }
        Adt32::C9(_) => {
            f563(v0, Adt128::C8)
        }
        Adt32::C10(_) => {
            f563(v0, Adt128::C9)
        }
        Adt32::C11 => {
            if f563(v0.clone(), Adt128::C10) {
                true
            } else {
                f563(v0, Adt128::C11)
            }
        }
        Adt32::C12(_) => {
            f564(v0)
        }
        Adt32::C13(_) => {
            f565(v0)
        }
    }
}

pub fn f527(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt116, Adt119> {
    let m2102 = v2.clone();
    match m2102 {
        Adt32::C0(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt116, Adt119>(f569(v0, f567(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt116, Adt119>(f477(v0, v2))
        }
        Adt32::C12(v3) => {
            f566(v0, f567(v1), v3)
        }
        Adt32::C13(v4) => {
            f568(v0, f567(v1), v4)
        }
    }
}

pub fn f528() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f529(v0: u64, v1: u64) -> bool {
    let a2103 = v1;
    let a2104 = v0;
    nat_lt(a2103, a2104)
}

pub fn f530(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2105 = f570(v0, v1, List::<(Str, Str)>::nil());
    match m2105 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt120>(f571(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt120>(v3)
        }
    }
}

pub fn f531(p2109: List<Adt99>, v1: Adt5) -> R<Option<Adt97>> {
    let m2108 = p2109;
    {
        let mut m2108 = m2108;
        loop {
            let step = {
                let v0 = m2108;
                let m2106 = v0;
                match m2106.uncons() {
                    None => {
                        Err::<List<Adt99>, R<Option<Adt97>>>(Ok(None::<Adt97>))
                    }
                    Some((v2, v3)) => {
                        let m2107 = f572(v2, v1.clone())?;
                        match m2107 {
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
                Ok(next) => { m2108 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Adt5) -> Adt5 {
    let m2110 = f134(Str::lit("arguments"), v0);
    match m2110 {
        Adt123::C0 => {
            f152()
        }
        Adt123::C1(v1) => {
            let m2111 = f265(v1);
            match m2111 {
                None => {
                    f152()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f533(v0: Option<Str>, v1: Option<Str>) -> Option<Adt97> {
    let m2112 = v0;
    match m2112 {
        None => {
            Some(Adt97::C2)
        }
        Some(v2) => {
            let m2113 = v1;
            match m2113 {
                None => {
                    None::<Adt97>
                }
                Some(v3) => {
                    let a2114 = v2;
                    let a2115 = v3;
                    if equal(a2114, a2115) {
                        None::<Adt97>
                    } else {
                        Some(Adt97::C5)
                    }
                }
            }
        }
    }
}

pub fn f534(v0: Adt138, v1: Adt5) -> Option<Str> {
    let m2116 = v0;
    match m2116 {
        Adt138::C0 => {
            None::<Str>
        }
        Adt138::C1 => {
            let m2117 = f134(Str::lit("name"), v1);
            match m2117 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v2) => {
                    f176(v2)
                }
            }
        }
        Adt138::C2 => {
            let m2118 = f134(Str::lit("uri"), v1);
            match m2118 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v3) => {
                    f176(v3)
                }
            }
        }
        Adt138::C3 => {
            let m2119 = f134(Str::lit("taskId"), v1);
            match m2119 {
                Adt123::C0 => {
                    None::<Str>
                }
                Adt123::C1(v4) => {
                    f176(v4)
                }
            }
        }
    }
}

pub fn f535(v0: Adt6) -> Option<Adt17> {
    let m2120 = f265(v0);
    match m2120 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2121 = f573(Str::lit("experimental"), v1.clone());
            match m2121 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2122 = f573(Str::lit("roots"), v1.clone());
                    match m2122 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2123 = f573(Str::lit("sampling"), v1.clone());
                            match m2123 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2124 = f573(Str::lit("elicitation"), v1.clone());
                                    match m2124 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2125 = f573(Str::lit("extensions"), v1);
                                            match m2125 {
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

pub fn f536() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f537(v0: Adt5) -> Result<Option<Adt18>, Adt120> {
    let m2126 = f481(f574(), v0);
    match m2126 {
        Ok(v2) => {
            let m2127 = v2;
            match m2127 {
                None => {
                    Ok::<Option<Adt18>, Adt120>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2128 = f575(v3);
                    match m2128 {
                        Adt150::C0 => {
                            Err::<Option<Adt18>, Adt120>(Adt120::C8(f574()))
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

pub fn f538(v0: Adt5) -> Result<Option<Adt19>, Adt120> {
    let m2129 = f134(f576(), v0);
    match m2129 {
        Adt123::C0 => {
            Ok::<Option<Adt19>, Adt120>(None::<Adt19>)
        }
        Adt123::C1(v1) => {
            let m2130 = f176(v1.clone());
            match m2130 {
                None => {
                    let m2131 = f513(v1);
                    match m2131 {
                        None => {
                            Err::<Option<Adt19>, Adt120>(Adt120::C8(f576()))
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

pub fn f539(v0: Adt6) -> bool {
    let m2132 = f176(v0);
    match m2132 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f540(v0: Adt6) -> bool {
    let m2133 = f452(v0);
    match m2133 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f541(v0: Adt6) -> bool {
    let m2134 = f513(v0);
    match m2134 {
        None => {
            false
        }
        Some(v1) => {
            let a2138 = {
                let m2137 = {
                    let a2135 = v1;
                    let a2136 = 0i64;
                    compare(a2135, a2136)
                };
                match m2137 {
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
            bool_not(a2138)
        }
    }
}

pub fn f542(v0: Adt6) -> bool {
    let m2139 = f330(v0);
    match m2139 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f543(v0: Adt6) -> bool {
    let m2140 = f265(v0);
    match m2140 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f544(v0: Adt6) -> bool {
    let m2141 = f330(v0);
    match m2141 {
        None => {
            false
        }
        Some(v1) => {
            f577(v1)
        }
    }
}

pub fn f545(v0: Adt6) -> bool {
    let m2142 = f264(v0);
    match m2142 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f546(v0: Adt6) -> bool {
    let m2143 = f176(v0.clone());
    match m2143 {
        None => {
            let m2144 = f330(v0);
            match m2144 {
                None => {
                    false
                }
                Some(v2) => {
                    f578(v2)
                }
            }
        }
        Some(v1) => {
            f447(v1)
        }
    }
}

pub fn f547(v0: Adt6) -> bool {
    let m2145 = f330(v0);
    match m2145 {
        None => {
            false
        }
        Some(v1) => {
            let m2146 = v1;
            match m2146.uncons() {
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

pub fn f548(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2147 = f179(Str::lit("type"), v0);
    match m2147 {
        Adt123::C0 => {
            None::<Adt13>
        }
        Adt123::C1(v1) => {
            let m2148 = f176(v1);
            match m2148 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f579(v2)
                }
            }
        }
    }
}

pub fn f549(v0: List<u64>) -> R<List<u64>> {
    Ok(f154(f580(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f550(v0: List<u64>, p2151: List<List<u64>>) -> bool {
    let m2150 = p2151;
    {
        let mut m2150 = m2150;
        loop {
            let step = {
                let v1 = m2150;
                let m2149 = v1;
                match m2149.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f581(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2150 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(p2154: List<Str>, p2155: List<Str>) -> List<Str> {
    let m2153 = (p2154, p2155);
    {
        let mut m2153 = m2153;
        loop {
            let step = {
                let (v0, v1) = m2153;
                let m2152 = v0;
                match m2152.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f552(p2167: u64, p2168: i64, p2169: i64) -> R<bool> {
    let m2166 = (p2167, (p2168, p2169));
    {
        let mut m2166 = m2166;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2166;
                let m2156 = v0;
                if m2156 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2156 - 1;
                    let m2159 = {
                        let a2157 = v2;
                        let a2158 = 0i64;
                        compare(a2157, a2158)
                    };
                    if match m2159 {
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
                        if f319({
                            let a2160 = v1;
                            let a2161 = 10i64;
                            let a2162 = 0i64;
                            int_rem(a2160, a2161, a2162)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2163 = v1;
                                let a2164 = 10i64;
                                let a2165 = 0i64;
                                int_quot(a2163, a2164, a2165)?
                            }, f209(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f553(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2170 = f582(v0.clone());
    let a2171 = f582(v1.clone());
    if nat_eq(a2170, a2171) {
        let m2172 = v0;
        match m2172 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2173 = f264(v1);
                match m2173 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2174 = v3;
                        let a2175 = v4;
                        if equal(a2174, a2175) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2176 = f452(v1);
                match m2176 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f583(f454(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2177 = f176(v1);
                match m2177 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2178 = v7;
                        let a2179 = v8;
                        if equal(a2178, a2179) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2180 = f330(v1);
                match m2180 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f584(f585(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2181 = f265(v1);
                match m2181 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2183 = f458(v11.clone(), 0u64)?;
                        let a2184 = f458(match v12.clone() {
                            Adt5::C0(h2182) => {
                                h2182
                            }
                        }, 0u64)?;
                        if nat_eq(a2183, a2184) {
                            Ok(f584(f586(v11, match v12 {
                                Adt5::C0(h2185) => {
                                    h2185
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

pub fn f554(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2186 = v0;
    match m2186 {
        Adt147::C0 => {
            f587(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f555(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2187 = f514(f588(v0, v1)?, f588(v2, v3)?);
    match m2187 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f589(v0, f590(v0)?, v2, f590(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f556(v0: i64) -> R<i64> {
    let m2190 = {
        let a2188 = v0;
        let a2189 = 0i64;
        compare(a2188, a2189)
    };
    if match m2190 {
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
        let a2191 = v0;
        int_neg(a2191)
    } else {
        Ok(v0)
    }
}

pub fn f557(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2192 = v0;
    match m2192 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2198 = f591(match v1.clone() {
                    Adt40::C0(h2193, _, _) => {
                        h2193
                    }
                });
                let a2199 = {
                    let a2196 = f592(match v1.clone() {
                        Adt40::C0(_, h2194, _) => {
                            h2194
                        }
                    });
                    let a2197 = f341(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2195) => {
                            h2195
                        }
                    });
                    append_list(a2196, a2197)
                };
                append_list(a2198, a2199)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f558(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2200 = v0;
    match m2200 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f70(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f559(v0: Adt46) -> Adt6 {
    let m2201 = v0;
    match m2201 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2202, _, _, _) => {
                    h2202
                }
            })), {
                let a2206 = f341(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2203, _, _) => {
                        h2203
                    }
                });
                let a2207 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2204, _) => {
                        h2204
                    }
                })), f519(match v1 {
                    Adt47::C0(_, _, _, h2205) => {
                        h2205
                    }
                }));
                append_list(a2206, a2207)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2208, _, _, _) => {
                    h2208
                }
            })), {
                let a2212 = f341(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2209, _, _) => {
                        h2209
                    }
                });
                let a2213 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2210, _) => {
                        h2210
                    }
                })), f519(match v2 {
                    Adt48::C0(_, _, _, h2211) => {
                        h2211
                    }
                }));
                append_list(a2212, a2213)
            }))
        }
    }
}

pub fn f560(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2214, _, _, _) => {
            h2214
        }
    })), {
        let a2220 = f341(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2215, _, _) => {
                h2215
            }
        });
        let a2221 = {
            let a2218 = f593(match v0.clone() {
                Adt2::C0(_, _, h2216, _) => {
                    h2216
                }
            });
            let a2219 = f594(match v0 {
                Adt2::C0(_, _, _, h2217) => {
                    h2217
                }
            });
            append_list(a2218, a2219)
        };
        append_list(a2220, a2221)
    }))
}

pub fn f561(v0: u64, v1: List<u64>) -> List<u64> {
    let a2222 = v0;
    let a2223 = 0u64;
    if nat_eq(a2222, a2223) {
        v1
    } else {
        let a2224 = v0;
        let a2225 = 1u64;
        if nat_eq(a2224, a2225) {
            List::cons(194u64, v1)
        } else {
            let a2226 = v0;
            let a2227 = 2u64;
            if nat_eq(a2226, a2227) {
                List::cons(226u64, v1)
            } else {
                let a2228 = v0;
                let a2229 = 3u64;
                if nat_eq(a2228, a2229) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f562(v0: u64, v1: u64, v2: List<u64>) -> Adt148 {
    let a2230 = v0;
    let a2231 = 0u64;
    if nat_eq(a2230, a2231) {
        f595(v1, v2)
    } else {
        let a2232 = v0;
        let a2233 = 1u64;
        if nat_eq(a2232, a2233) {
            let a2234 = 128u64;
            let a2235 = v1;
            if if nat_le(a2234, a2235) {
                let a2236 = v1;
                let a2237 = 159u64;
                nat_le(a2236, a2237)
            } else {
                false
            } {
                Adt148::C0(0u64, f596(v2))
            } else {
                f595(v1, List::cons(194u64, v2))
            }
        } else {
            let a2238 = v0;
            let a2239 = 2u64;
            if nat_eq(a2238, a2239) {
                let a2240 = v1;
                let a2241 = 128u64;
                if nat_eq(a2240, a2241) {
                    Adt148::C0(3u64, v2)
                } else {
                    let a2242 = v1;
                    let a2243 = 129u64;
                    if nat_eq(a2242, a2243) {
                        Adt148::C0(4u64, v2)
                    } else {
                        f595(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2244 = v0;
                let a2245 = 3u64;
                if nat_eq(a2244, a2245) {
                    let a2246 = 170u64;
                    let a2247 = v1;
                    if if nat_le(a2246, a2247) {
                        let a2248 = v1;
                        let a2249 = 174u64;
                        nat_le(a2248, a2249)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f596(v2))
                    } else {
                        f595(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2250 = 166u64;
                    let a2251 = v1;
                    if if nat_le(a2250, a2251) {
                        let a2252 = v1;
                        let a2253 = 169u64;
                        nat_le(a2252, a2253)
                    } else {
                        false
                    } {
                        Adt148::C0(0u64, f596(v2))
                    } else {
                        f595(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f563(v0: Adt128, v1: Adt128) -> bool {
    let a2254 = f361(v0);
    let a2255 = f361(v1);
    equal(a2254, a2255)
}

pub fn f564(v0: Adt128) -> bool {
    let m2256 = v0;
    match m2256 {
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

pub fn f565(v0: Adt128) -> bool {
    let m2257 = v0;
    match m2257 {
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

pub fn f566(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt116, Adt119> {
    f597(v0, v2.clone(), f598(match v1 {
        Adt16::C0(_, h2258, _, _, _, _) => {
            h2258
        }
    }, f599(v2)))
}

pub fn f567(v0: Adt14) -> Adt16 {
    let m2259 = v0;
    match m2259 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2260) => {
                    h2260
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2261, _) => {
                    h2261
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2262, _, _, _, _) => {
                    h2262
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2263, _) => {
                    h2263
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2264, _, _, _) => {
                    h2264
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2265, _) => {
                    h2265
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2266, _) => {
                    h2266
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2267, _, _, _, _) => {
                    h2267
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2268, _, _, _) => {
                    h2268
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2269, _) => {
                    h2269
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2270, _, _) => {
                    h2270
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2271, _) => {
                    h2271
                }
            }
        }
    }
}

pub fn f568(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt116, Adt119> {
    f600(v0, v2, f601(match v1 {
        Adt16::C0(_, h2272, _, _, _, _) => {
            h2272
        }
    }))
}

pub fn f569(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt116 {
    Adt116::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2273, _) => {
            h2273
        }
    }), {
        let m2275 = f477(v0, Adt32::C11);
        match m2275 {
            Adt116::C0(_, h2274, _) => {
                h2274
            }
        }
    }, f602(v1, match v2 {
        Adt49::C0(_, h2276) => {
            h2276
        }
    }))
}

pub fn f570(v0: Str, p2282: List<(Str, Adt6)>, p2283: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt120> {
    let m2281 = (p2282, p2283);
    {
        let mut m2281 = m2281;
        loop {
            let step = {
                let (v1, v2) = m2281;
                let m2277 = v1;
                match m2277.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Ok::<List<(Str, Str)>, Adt120>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2279 = f176({
                            let (_, h2278) = v3;
                            h2278
                        });
                        match m2279 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>(Err::<List<(Str, Str)>, Adt120>(Adt120::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt120>>((v4, List::cons(({
                                    let (h2280, _) = v3;
                                    h2280
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2281 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f571(p2286: List<(Str, Str)>, p2287: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2285 = (p2286, p2287);
    {
        let mut m2285 = m2285;
        loop {
            let step = {
                let (v0, v1) = m2285;
                let m2284 = v0;
                match m2284.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Adt99, v1: Adt5) -> R<Option<Adt97>> {
    let m2289 = f603(match v0.clone() {
        Adt99::C0(_, h2288, _, _) => {
            h2288
        }
    }, v1);
    match m2289 {
        Adt123::C0 => {
            Ok(f604(v0))
        }
        Adt123::C1(v2) => {
            if f605(v2.clone()) {
                Ok(f604(v0))
            } else {
                f606(v0, v2)
            }
        }
    }
}

pub fn f573(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2290 = f134(v0, v1);
    match m2290 {
        Adt123::C0 => {
            Some(None::<Adt5>)
        }
        Adt123::C1(v2) => {
            let m2291 = f265(v2);
            match m2291 {
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

pub fn f574() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f575(v0: Str) -> Adt150 {
    let a2292 = v0.clone();
    let a2293 = Str::lit("debug");
    if equal(a2292, a2293) {
        Adt150::C1(Adt18::C0)
    } else {
        let a2294 = v0.clone();
        let a2295 = Str::lit("info");
        if equal(a2294, a2295) {
            Adt150::C1(Adt18::C1)
        } else {
            let a2296 = v0.clone();
            let a2297 = Str::lit("notice");
            if equal(a2296, a2297) {
                Adt150::C1(Adt18::C2)
            } else {
                let a2298 = v0.clone();
                let a2299 = Str::lit("warning");
                if equal(a2298, a2299) {
                    Adt150::C1(Adt18::C3)
                } else {
                    let a2300 = v0.clone();
                    let a2301 = Str::lit("error");
                    if equal(a2300, a2301) {
                        Adt150::C1(Adt18::C4)
                    } else {
                        let a2302 = v0.clone();
                        let a2303 = Str::lit("critical");
                        if equal(a2302, a2303) {
                            Adt150::C1(Adt18::C5)
                        } else {
                            let a2304 = v0.clone();
                            let a2305 = Str::lit("alert");
                            if equal(a2304, a2305) {
                                Adt150::C1(Adt18::C6)
                            } else {
                                let a2306 = v0;
                                let a2307 = Str::lit("emergency");
                                if equal(a2306, a2307) {
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

pub fn f576() -> Str {
    Str::lit("progressToken")
}

pub fn f577(p2311: List<Adt6>) -> bool {
    let m2310 = p2311;
    {
        let mut m2310 = m2310;
        loop {
            let step = {
                let v0 = m2310;
                let m2308 = v0;
                match m2308.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2309 = f176(v1);
                        match m2309 {
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
                Ok(next) => { m2310 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(p2315: List<Adt6>) -> bool {
    let m2314 = p2315;
    {
        let mut m2314 = m2314;
        loop {
            let step = {
                let v0 = m2314;
                let m2312 = v0;
                match m2312.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2313 = f176(v1);
                        match m2313 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f447(v3) {
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
                Ok(next) => { m2314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f579(v0: Str) -> Option<Adt13> {
    let a2316 = v0.clone();
    let a2317 = Str::lit("string");
    if equal(a2316, a2317) {
        Some(Adt13::C0)
    } else {
        let a2318 = v0.clone();
        let a2319 = Str::lit("integer");
        if equal(a2318, a2319) {
            Some(Adt13::C1)
        } else {
            let a2320 = v0;
            let a2321 = Str::lit("boolean");
            if equal(a2320, a2321) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f580(p2330: List<u64>, p2331: List<u64>) -> R<List<u64>> {
    let m2329 = (p2330, p2331);
    {
        let mut m2329 = m2329;
        loop {
            let step = {
                let (v0, v1) = m2329;
                let m2322 = v0;
                match m2322.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2323 = 65u64;
                        let a2324 = v2;
                        if if nat_le(a2323, a2324) {
                            let a2325 = v2;
                            let a2326 = 90u64;
                            nat_le(a2325, a2326)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2327 = v2;
                                let a2328 = 32u64;
                                nat_add(a2327, a2328)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2329 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p2338: List<u64>, p2339: List<u64>) -> bool {
    let m2337 = (p2338, p2339);
    {
        let mut m2337 = m2337;
        loop {
            let step = {
                let (v0, v1) = m2337;
                let m2332 = v0;
                match m2332.uncons() {
                    None => {
                        let m2333 = v1;
                        match m2333.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2334 = v1;
                        match m2334.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2335 = v4;
                                let a2336 = v6;
                                if nat_eq(a2335, a2336) {
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
                Ok(next) => { m2337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: Adt6) -> u64 {
    let m2340 = v0;
    match m2340 {
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

pub fn f583(v0: Adt147) -> bool {
    let m2341 = v0;
    match m2341 {
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

pub fn f584(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2342 = v0;
    match m2342 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2343 = v2;
                let a2344 = v1;
                append_list(a2343, a2344)
            })
        }
    }
}

pub fn f585(p2349: List<Adt6>, p2350: List<Adt6>, p2351: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2348 = (p2349, (p2350, p2351));
    {
        let mut m2348 = m2348;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2348;
                let m2345 = v0;
                match m2345.uncons() {
                    None => {
                        let m2346 = v1;
                        match m2346.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2347 = v1;
                        match m2347.uncons() {
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
                Ok(next) => { m2348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(p2357: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2358: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2356 = (p2357, p2358);
    {
        let mut m2356 = m2356;
        loop {
            let step = {
                let (v0, v2) = m2356;
                let m2352 = v0;
                match m2352.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2354 = f179({
                            let (h2353, _) = v3;
                            h2353
                        }, v1.clone());
                        match m2354 {
                            Adt123::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt123::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2355) = v3;
                                    h2355
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2356 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(v0: Adt147) -> Adt147 {
    let m2359 = v0;
    match m2359 {
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

pub fn f588(v0: i64, v1: i64) -> R<i64> {
    f209(f151(f590(v0)?, 0i64)?, v1)
}

pub fn f589(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2360 = v1;
    let a2361 = v3;
    if nat_lt(a2360, a2361) {
        Ok(f514(f607({
            let a2362 = v3;
            let a2363 = v1;
            nat_sub(a2362, a2363)
        }, v0)?, v2))
    } else {
        let a2364 = v3;
        let a2365 = v1;
        if nat_lt(a2364, a2365) {
            Ok(f514(v0, f607({
                let a2366 = v1;
                let a2367 = v3;
                nat_sub(a2366, a2367)
            }, v2)?))
        } else {
            Ok(f514(v0, v2))
        }
    }
}

pub fn f590(v0: i64) -> R<u64> {
    f608(20u64, v0, 0u64)
}

pub fn f591(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2368 = v0;
    match m2368 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f609(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f592(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2369 = v0;
    match m2369 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f593(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2370 = v0;
    match m2370 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f298(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f594(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2371 = v0;
    match m2371 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f610(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f595(v0: u64, v1: List<u64>) -> Adt148 {
    let a2372 = v0;
    let a2373 = 32u64;
    if nat_lt(a2372, a2373) {
        if f611(v0) {
            Adt148::C0(0u64, List::cons(v0, v1))
        } else {
            Adt148::C0(0u64, f596(v1))
        }
    } else {
        let a2374 = v0;
        let a2375 = 127u64;
        if nat_eq(a2374, a2375) {
            Adt148::C0(0u64, f596(v1))
        } else {
            let a2376 = v0;
            let a2377 = 194u64;
            if nat_eq(a2376, a2377) {
                Adt148::C0(1u64, v1)
            } else {
                let a2378 = v0;
                let a2379 = 226u64;
                if nat_eq(a2378, a2379) {
                    Adt148::C0(2u64, v1)
                } else {
                    Adt148::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f596(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f597(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f477(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f612(f599(v1)))))
    }
}

pub fn f598(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2380 = f613(v1.clone());
    if if if bool_not(a2380) {
        true
    } else {
        f364(match v0.clone() {
            Adt17::C0(_, _, _, h2381, _) => {
                h2381
            }
        })
    } {
        let a2382 = f614(v1.clone());
        if bool_not(a2382) {
            true
        } else {
            f364(match v0.clone() {
                Adt17::C0(_, _, h2383, _, _) => {
                    h2383
                }
            })
        }
    } else {
        false
    } {
        let a2384 = f615(v1);
        if bool_not(a2384) {
            true
        } else {
            f364(match v0 {
                Adt17::C0(_, h2385, _, _, _) => {
                    h2385
                }
            })
        }
    } else {
        false
    }
}

pub fn f599(v0: Adt81) -> List<(Str, Adt64)> {
    let m2386 = v0;
    match m2386 {
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

pub fn f600(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt116, Adt119> {
    if v2 {
        Ok::<Adt116, Adt119>(f477(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt116, Adt119>(Adt119::C4(Adt84::C8(f616())))
    }
}

pub fn f601(v0: Adt17) -> bool {
    f365(match v0 {
        Adt17::C0(_, _, _, _, h2387) => {
            h2387
        }
    }, f366())
}

pub fn f602(v0: Adt16, v1: List<Adt50>) -> List<Adt118> {
    let m2389 = match v0 {
        Adt16::C0(_, _, _, _, h2388, _) => {
            h2388
        }
    };
    match m2389 {
        None => {
            List::<Adt118>::nil()
        }
        Some(v2) => {
            f617(f618(v2, v1, List::<Adt118>::nil()), List::<Adt118>::nil())
        }
    }
}

pub fn f603(p2395: List<Str>, p2396: Adt5) -> Adt123 {
    let m2394 = (p2395, p2396);
    {
        let mut m2394 = m2394;
        loop {
            let step = {
                let (v0, v1) = m2394;
                let m2390 = v0;
                match m2390.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                    }
                    Some((v2, v3)) => {
                        let m2391 = f134(v2, v1);
                        match m2391 {
                            Adt123::C0 => {
                                Err::<(List<Str>, Adt5), Adt123>(Adt123::C0)
                            }
                            Adt123::C1(v4) => {
                                let m2392 = v3.clone();
                                match m2392.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt123>(Adt123::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2393 = f265(v4);
                                        match m2393 {
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
                Ok(next) => { m2394 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Adt99) -> Option<Adt97> {
    let m2398 = match v0.clone() {
        Adt99::C0(_, _, _, h2397) => {
            h2397
        }
    };
    match m2398 {
        Adt100::C0 => {
            None::<Adt97>
        }
        Adt100::C1 => {
            Some(Adt97::C10(match v0 {
                Adt99::C0(h2399, _, _, _) => {
                    h2399
                }
            }))
        }
        Adt100::C2(_) => {
            None::<Adt97>
        }
    }
}

pub fn f605(v0: Adt6) -> bool {
    let m2400 = v0;
    match m2400 {
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

pub fn f606(v0: Adt99, v1: Adt6) -> R<Option<Adt97>> {
    let m2402 = match v0.clone() {
        Adt99::C0(_, _, _, h2401) => {
            h2401
        }
    };
    match m2402 {
        Adt100::C0 => {
            Ok(Some(Adt97::C8(match v0 {
                Adt99::C0(h2403, _, _, _) => {
                    h2403
                }
            })))
        }
        Adt100::C1 => {
            Ok(Some(Adt97::C10(match v0 {
                Adt99::C0(h2404, _, _, _) => {
                    h2404
                }
            })))
        }
        Adt100::C2(v2) => {
            if f619(match v0.clone() {
                Adt99::C0(_, _, h2405, _) => {
                    h2405
                }
            }, v2, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(Adt97::C9(match v0 {
                    Adt99::C0(h2406, _, _, _) => {
                        h2406
                    }
                })))
            }
        }
    }
}

pub fn f607(p2411: u64, p2412: i64) -> R<i64> {
    let m2410 = (p2411, p2412);
    {
        let mut m2410 = m2410;
        loop {
            let step = {
                let (v0, v1) = m2410;
                let m2407 = v0;
                if m2407 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2407 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2408 = v1;
                        let a2409 = 10i64;
                        int_mul(a2408, a2409)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f608(p2423: u64, p2424: i64, p2425: u64) -> R<u64> {
    let m2422 = (p2423, (p2424, p2425));
    {
        let mut m2422 = m2422;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2422;
                let m2413 = v0;
                if m2413 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2413 - 1;
                    let v4: u64 = {
                        let a2414 = v2;
                        let a2415 = 1u64;
                        nat_add(a2414, a2415)?
                    };
                    let m2418 = {
                        let a2416 = v1;
                        let a2417 = 10i64;
                        compare(a2416, a2417)
                    };
                    if match m2418 {
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
                            let a2419 = v1;
                            let a2420 = 10i64;
                            let a2421 = 0i64;
                            int_quot(a2419, a2420, a2421)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f609(v0: List<Adt41>) -> List<Adt6> {
    f240(f620(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f610(v0: Adt3) -> Str {
    let m2426 = v0;
    match m2426 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f611(v0: u64) -> bool {
    let a2427 = v0;
    let a2428 = 9u64;
    if nat_eq(a2427, a2428) {
        true
    } else {
        let a2429 = v0;
        let a2430 = 10u64;
        if nat_eq(a2429, a2430) {
            true
        } else {
            let a2431 = v0;
            let a2432 = 13u64;
            nat_eq(a2431, a2432)
        }
    }
}

pub fn f612(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f615(v0.clone()) {
        Some(f152())
    } else {
        None::<Adt5>
    }, if f614(v0.clone()) {
        Some(f152())
    } else {
        None::<Adt5>
    }, if f613(v0) {
        Some(f152())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f613(p2437: List<(Str, Adt64)>) -> bool {
    let m2436 = p2437;
    {
        let mut m2436 = m2436;
        loop {
            let step = {
                let v0 = m2436;
                let m2433 = v0;
                match m2433.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2435 = {
                            let (_, h2434) = v1;
                            h2434
                        };
                        match m2435 {
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
                Ok(next) => { m2436 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(p2442: List<(Str, Adt64)>) -> bool {
    let m2441 = p2442;
    {
        let mut m2441 = m2441;
        loop {
            let step = {
                let v0 = m2441;
                let m2438 = v0;
                match m2438.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2440 = {
                            let (_, h2439) = v1;
                            h2439
                        };
                        match m2440 {
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
                Ok(next) => { m2441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f615(p2447: List<(Str, Adt64)>) -> bool {
    let m2446 = p2447;
    {
        let mut m2446 = m2446;
        loop {
            let step = {
                let v0 = m2446;
                let m2443 = v0;
                match m2443.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2445 = {
                            let (_, h2444) = v1;
                            h2444
                        };
                        match m2445 {
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
                Ok(next) => { m2446 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f366(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f617(p2450: List<Adt118>, p2451: List<Adt118>) -> List<Adt118> {
    let m2449 = (p2450, p2451);
    {
        let mut m2449 = m2449;
        loop {
            let step = {
                let (v0, v1) = m2449;
                let m2448 = v0;
                match m2448.uncons() {
                    None => {
                        Err::<(List<Adt118>, List<Adt118>), List<Adt118>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt118>, List<Adt118>), List<Adt118>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: Adt19, p2454: List<Adt50>, p2455: List<Adt118>) -> List<Adt118> {
    let m2453 = (p2454, p2455);
    {
        let mut m2453 = m2453;
        loop {
            let step = {
                let (v1, v2) = m2453;
                let m2452 = v1;
                match m2452.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt118>), List<Adt118>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt118>), List<Adt118>>((v4, List::cons(Adt118::C0(v0.clone(), v3), v2)))
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

pub fn f619(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2456 = v0;
    match m2456 {
        Adt13::C0 => {
            let m2457 = f176(v2);
            match m2457 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2458 = v1;
                    let a2459 = v3;
                    Ok(equal(a2458, a2459))
                }
            }
        }
        Adt13::C1 => {
            f621(v1, v2)
        }
        Adt13::C2 => {
            let m2460 = f264(v2);
            match m2460 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2461 = v1;
                        let a2462 = Str::lit("true");
                        Ok(equal(a2461, a2462))
                    } else {
                        let a2463 = v1;
                        let a2464 = Str::lit("false");
                        Ok(equal(a2463, a2464))
                    }
                }
            }
        }
    }
}

pub fn f620(p2467: List<Adt41>, p2468: List<Adt6>) -> List<Adt6> {
    let m2466 = (p2467, p2468);
    {
        let mut m2466 = m2466;
        loop {
            let step = {
                let (v0, v1) = m2466;
                let m2465 = v0;
                match m2465.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f622(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2466 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f621(v0: Str, v1: Adt6) -> R<bool> {
    let m2469 = f452(v1);
    match m2469 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2470 = f178(v2)?;
            match m2470 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2472 = f623(f62({
                        let a2471 = v0;
                        utf8_encode(a2471)
                    })?)?;
                    match m2472 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f319({
                                let a2473 = v3;
                                let a2474 = v4;
                                int_sub(a2473, a2474)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f622(v0: Adt41) -> Str {
    let m2475 = v0;
    match m2475 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f623(v0: List<u64>) -> R<Option<i64>> {
    let m2476 = v0.clone();
    match m2476.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2477 = v1;
            let a2478 = 45u64;
            if nat_eq(a2477, a2478) {
                let m2479 = v2.clone();
                match m2479.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2480 = f624(v2, 0i64)?;
                        match m2480 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2481 = v5;
                                    int_neg(a2481)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f624(v0, 0i64)
            }
        }
    }
}

pub fn f624(p2487: List<u64>, p2488: i64) -> R<Option<i64>> {
    let m2486 = (p2487, p2488);
    {
        let mut m2486 = m2486;
        loop {
            let step = {
                let (v0, v1) = m2486;
                let m2482 = v0;
                match m2482.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2483 = f625(v2);
                        match m2483 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f209({
                                    let a2484 = v1;
                                    let a2485 = 10i64;
                                    int_mul(a2484, a2485)?
                                }, f239(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2486 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f625(v0: u64) -> Option<u64> {
    let a2489 = v0;
    let a2490 = 48u64;
    if if nat_lt(a2489, a2490) {
        true
    } else {
        let a2491 = 57u64;
        let a2492 = v0;
        nat_lt(a2491, a2492)
    } {
        None::<u64>
    } else {
        Some({
            let a2493 = v0;
            let a2494 = 48u64;
            nat_sub(a2493, a2494)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
