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
    C0(Adt1, Option<Str>, Adt4, Adt10, List<Adt12>, u64, Bytes),
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
    C10,
    C11,
    C12(Str),
    C13,
    C14,
    C15(Adt91),
}

#[derive(Clone)]
pub enum Adt91 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt92 {
    C0(List<u64>, List<u64>, List<Adt93>, Bytes, u64, List<List<u64>>, Bytes, Adt14),
}

#[derive(Clone)]
pub enum Adt93 {
    C0(List<u64>, List<u64>),
}

#[derive(Clone)]
pub enum Adt94 {
    C0(u64),
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt95 {
    C0(u64, List<Adt93>, List<List<u64>>, List<u64>, Option<List<u64>>),
}

#[derive(Clone)]
pub enum Adt96 {
    C0(Bytes, u64),
}

#[derive(Clone)]
pub enum Adt97 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt98 {
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
pub enum Adt99 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt100 {
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
pub enum Adt101 {
    C0,
    C1(Option<Adt99>, Adt102),
}

#[derive(Clone)]
pub enum Adt102 {
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
pub enum Adt103 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt99, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt107 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt108 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt109>, List<Adt111>),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt110),
    C7(Adt110),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt111 {
    C0(Str, List<Str>, Adt13, Adt112),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt113 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt115 {
    C0(Adt116, List<Adt120>, u64),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt117, List<u64>, bool),
    C7(u64, u64),
    C8(Adt118),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt117 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt118 {
    C0(bool, Adt119, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt119 {
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
pub enum Adt120 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0,
    C1(List<Adt123>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt124 {
    C0,
    C1(List<Adt123>, Str),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Adt6, Adt6, List<Adt123>),
    C1(List<Adt6>, Adt6, List<Adt123>),
    C2(List<Adt6>, Adt6, List<Adt123>, u64),
    C3(List<Adt123>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt123>),
    C5(Adt126, Adt6, Adt6, List<Adt123>),
}

#[derive(Clone)]
pub enum Adt126 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt127 {
    C0(Adt99, Result<Adt128, Adt131>),
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
    C2(Adt109),
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
    C0(Option<Str>, Option<Adt109>),
}

#[derive(Clone)]
pub enum Adt134 {
    C0,
    C1(Adt99),
    C2,
}

#[derive(Clone)]
pub enum Adt135 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt136 {
    C0(Adt100),
    C1(Adt115),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt124, List<Adt125>),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt100),
    C1(List<Adt107>, List<u64>),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt99, Adt140, Adt17, Adt5),
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
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt125>),
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1(Adt140),
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt145),
    C1(Adt131),
}

#[derive(Clone)]
pub enum Adt145 {
    C0(Adt99, Str, Adt17, Adt5),
}

#[derive(Clone)]
pub enum Adt146 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt147 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt148 {
    C0(Adt98),
    C1(Adt121),
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
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt156 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt157 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt158 {
    C0,
    C1(Adt19),
}

#[derive(Clone)]
pub enum Adt159 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt160 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt161 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt160, u64),
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
    F68(Fn0, Adt92),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt15) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn2::F68(k0, k1) => f68(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F118(Fn1, Adt36),
}

impl Fn3 {
    pub fn apply(&self, p0: Adt22) -> R<Result<Adt33, Adt90>> {
        match self {
            Fn3::F118(k0, k1) => f118(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F808(Bytes),
    F833(Bytes, Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn4::F808(k0) => f808(k0.clone(), p0),
            Fn4::F833(k0, k1) => f833(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F813(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn5::F813(k0) => f813(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F818(Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn6::F818(k0) => f818(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F841(Bytes, u64, u64, u64),
    F848(Bytes, u64, u64),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, Adt159)) -> R<(u64, Adt159)> {
        match self {
            Fn7::F841(k0, k1, k2, k3) => f841(k0.clone(), *k1, *k2, *k3, p0),
            Fn7::F848(k0, k1, k2) => f848(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F843,
}

impl Fn8 {
    pub fn apply(&self, p0: Adt161) -> R<Adt161> {
        match self {
            Fn8::F843 => f843(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F873,
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn9::F873 => f873(p0),
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
    Ok(Adt0::C0(f8(), None::<Str>, f9(), f10(), f11(f12()?)?, f13(), Bytes::lit(&[])))
}

pub fn f3(_: Adt14, v1: Adt15) -> R<Result<Adt33, Adt90>> {
    f14(v1)
}

pub fn f4(v0: Bytes) -> R<Result<Adt92, Adt94>> {
    let m2 = f15(v0.clone(), 0u64)?;
    match m2 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(0u64)))
        }
        Some(v1) => {
            f16(v0.clone(), v1.clone(), f15(v0, match v1 {
                Adt96::C0(_, h3) => {
                    h3
                }
            })?)
        }
    }
}

pub fn f5(v0: Adt95) -> List<u64> {
    let a27 = f17(match v0.clone() {
        Adt95::C0(h4, _, _, _, _) => {
            h4
        }
    });
    let a28 = {
        let a25 = f17({
            let a6 = match v0.clone() {
                Adt95::C0(_, h5, _, _, _) => {
                    h5
                }
            };
            length_list(a6)
        });
        let a26 = {
            let a23 = f18(match v0.clone() {
                Adt95::C0(_, h7, _, _, _) => {
                    h7
                }
            });
            let a24 = {
                let a21 = f17({
                    let a9 = match v0.clone() {
                        Adt95::C0(_, _, h8, _, _) => {
                            h8
                        }
                    };
                    length_list(a9)
                });
                let a22 = {
                    let a19 = f19(match v0.clone() {
                        Adt95::C0(_, _, h10, _, _) => {
                            h10
                        }
                    });
                    let a20 = {
                        let a17 = f17({
                            let a12 = match v0.clone() {
                                Adt95::C0(_, _, _, h11, _) => {
                                    h11
                                }
                            };
                            length_list(a12)
                        });
                        let a18 = {
                            let a15 = match v0.clone() {
                                Adt95::C0(_, _, _, h13, _) => {
                                    h13
                                }
                            };
                            let a16 = f20(match v0 {
                                Adt95::C0(_, _, _, _, h14) => {
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

pub fn f6() -> Adt95 {
    Adt95::C0(f21(), f22(), List::<List<u64>>::nil(), f23(), None::<List<u64>>)
}

pub fn f7(v0: Adt0, v1: Fn0, v2: Adt92) -> R<Adt95> {
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt92::C0(_, _, h29, _, _, _, _, _) => {
            h29
        }
    }, match v2 {
        Adt92::C0(_, _, _, _, _, h30, _, _) => {
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

pub fn f15(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let m33 = f32(v0.clone(), v1)?;
    match m33 {
        None => {
            Ok(None::<Adt96>)
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
                    Ok(None::<Adt96>)
                }
                Some(v3) => {
                    Ok(Some(Adt96::C0(v3, {
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

pub fn f16(v0: Bytes, v1: Adt96, v2: Option<Adt96>) -> R<Result<Adt92, Adt94>> {
    let m44 = v2;
    match m44 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v1 {
                Adt96::C0(_, h45) => {
                    h45
                }
            })))
        }
        Some(v3) => {
            f33(v0.clone(), v1, v3.clone(), f32(v0, match v3 {
                Adt96::C0(_, h46) => {
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

pub fn f18(v0: List<Adt93>) -> List<u64> {
    f34(f35(v0, List::<Adt93>::nil()), List::<u64>::nil())
}

pub fn f19(v0: List<List<u64>>) -> List<u64> {
    f36(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f20(v0: Option<List<u64>>) -> List<u64> {
    let m68 = v0;
    match m68 {
        None => {
            List::<u64>::nil()
        }
        Some(v1) => {
            let a72 = f17(1u64);
            let a73 = {
                let a70 = f17({
                    let a69 = v1.clone();
                    length_list(a69)
                });
                let a71 = v1;
                append_list(a70, a71)
            };
            append_list(a72, a73)
        }
    }
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt93> {
    List::cons(Adt93::C0(f38(), f39()), List::<Adt93>::nil())
}

pub fn f23() -> List<u64> {
    let a76 = f40();
    let a77 = {
        let a74 = f41();
        let a75 = f42();
        append_list(a74, a75)
    };
    append_list(a76, a77)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt92, v3: Option<Adt97>) -> R<Adt95> {
    let m78 = v3;
    match m78 {
        None => {
            f44(v0, v1, v2)
        }
        Some(v4) => {
            f43(v4)
        }
    }
}

pub fn f25(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m79 = f45(v0.clone(), v1.clone())?;
    match m79 {
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

pub fn f27(v0: List<Adt36>) -> R<Result<List<Adt12>, Adt98>> {
    let m80 = f47(v0.clone());
    match m80 {
        None => {
            f48(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f28(p83: u64, p84: List<Adt36>) -> R<List<Adt36>> {
    let m82 = (p83, p84);
    {
        let mut m82 = m82;
        loop {
            let step = {
                let (v0, v1) = m82;
                let m81 = v0;
                if m81 == 0 {
                    Err::<(u64, List<Adt36>), R<List<Adt36>>>(Ok(v1))
                } else {
                    let v2 = m81 - 1;
                    let v3: Adt36 = f49(f50(v2)?, Str::lit("Answers with its own name"), f51());
                    Ok::<(u64, List<Adt36>), R<List<Adt36>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m82 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f29() -> u64 {
    60u64
}

pub fn f30(v0: List<Adt36>) -> R<Option<Adt90>> {
    let m85 = f27(v0)?;
    match m85 {
        Ok(_) => {
            Ok(None::<Adt90>)
        }
        Err(v1) => {
            Ok(Some(Adt90::C9(f52(v1))))
        }
    }
}

pub fn f31(v0: Adt15) -> R<Result<Adt33, Adt90>> {
    let m86 = v0;
    match m86 {
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
    let a90 = {
        let a87 = v1;
        let a88 = 4u64;
        nat_add(a87, a88)?
    };
    let a91 = {
        let a89 = v0.clone();
        length_bytes(a89)
    };
    if nat_le(a90, a91) {
        Ok(Some({
            let a108 = {
                let a92 = f56(v0.clone(), v1)?;
                let a93 = 16777216u64;
                nat_mul(a92, a93)?
            };
            let a109 = {
                let a106 = {
                    let a96 = f56(v0.clone(), {
                        let a94 = v1;
                        let a95 = 1u64;
                        nat_add(a94, a95)?
                    })?;
                    let a97 = 65536u64;
                    nat_mul(a96, a97)?
                };
                let a107 = {
                    let a104 = {
                        let a100 = f56(v0.clone(), {
                            let a98 = v1;
                            let a99 = 2u64;
                            nat_add(a98, a99)?
                        })?;
                        let a101 = 256u64;
                        nat_mul(a100, a101)?
                    };
                    let a105 = f56(v0, {
                        let a102 = v1;
                        let a103 = 3u64;
                        nat_add(a102, a103)?
                    })?;
                    nat_add(a104, a105)?
                };
                nat_add(a106, a107)?
            };
            nat_add(a108, a109)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f33(v0: Bytes, v1: Adt96, v2: Adt96, v3: Option<u64>) -> R<Result<Adt92, Adt94>> {
    let m110 = v3;
    match m110 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0(match v2 {
                Adt96::C0(_, h111) => {
                    h111
                }
            })))
        }
        Some(v4) => {
            let m115 = f57(v0.clone(), {
                let a113 = match v2.clone() {
                    Adt96::C0(_, h112) => {
                        h112
                    }
                };
                let a114 = 4u64;
                nat_add(a113, a114)?
            }, v4, List::<Adt93>::nil())?;
            match m115 {
                Ok(v6) => {
                    f58(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt92, Adt94>(v5))
                }
            }
        }
    }
}

pub fn f34(p120: List<Adt93>, p121: List<u64>) -> List<u64> {
    let m119 = (p120, p121);
    {
        let mut m119 = m119;
        loop {
            let step = {
                let (v0, v1) = m119;
                let m116 = v0;
                match m116.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<u64>), List<u64>>((v3, {
                            let a117 = f59(v2);
                            let a118 = v1;
                            append_list(a117, a118)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f35(p124: List<Adt93>, p125: List<Adt93>) -> List<Adt93> {
    let m123 = (p124, p125);
    {
        let mut m123 = m123;
        loop {
            let step = {
                let (v0, v1) = m123;
                let m122 = v0;
                match m122.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
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

pub fn f36(p130: List<List<u64>>, p131: List<u64>) -> List<u64> {
    let m129 = (p130, p131);
    {
        let mut m129 = m129;
        loop {
            let step = {
                let (v0, v1) = m129;
                let m126 = v0;
                match m126.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a127 = f60(v2);
                            let a128 = v1;
                            append_list(a127, a128)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f37(p134: List<List<u64>>, p135: List<List<u64>>) -> List<List<u64>> {
    let m133 = (p134, p135);
    {
        let mut m133 = m133;
        loop {
            let step = {
                let (v0, v1) = m133;
                let m132 = v0;
                match m132.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m133 = next; }
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

pub fn f43(v0: Adt97) -> R<Adt95> {
    f61(f62(v0.clone()), f63(None::<Adt99>, f64(v0)))
}

pub fn f44(v0: Adt0, v1: Fn0, v2: Adt92) -> R<Adt95> {
    if f65(match v2.clone() {
        Adt92::C0(h136, _, _, _, _, _, _, _) => {
            h136
        }
    }) {
        f66(match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, _, h137) => {
                h137
            }
        }, f67(v0, match v2.clone() {
            Adt92::C0(_, _, _, _, _, _, h138, _) => {
                h138
            }
        }), Fn2::F68(v1, v2.clone()), match v2.clone() {
            Adt92::C0(_, _, _, _, h139, _, _, _) => {
                h139
            }
        }, match v2.clone() {
            Adt92::C0(_, _, h140, _, _, _, _, _) => {
                h140
            }
        }, f69(match v2 {
            Adt92::C0(_, _, _, h141, _, _, _, _) => {
                h141
            }
        })?)
    } else {
        Ok(f70())
    }
}

pub fn f45(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m142 = f71(v0, f72())?;
    match m142 {
        Adt103::C0 => {
            Ok(None::<Adt97>)
        }
        Adt103::C1 => {
            Ok(Some(Adt97::C0))
        }
        Adt103::C2 => {
            Ok(Some(Adt97::C0))
        }
        Adt103::C3(v2) => {
            f73(f74(v2), v1, Adt97::C0)
        }
    }
}

pub fn f46(v0: List<Adt93>, v1: List<List<u64>>) -> R<Option<Adt97>> {
    let m143 = f71(v0, f75())?;
    match m143 {
        Adt103::C0 => {
            Ok(None::<Adt97>)
        }
        Adt103::C1 => {
            Ok(Some(Adt97::C1))
        }
        Adt103::C2 => {
            Ok(Some(Adt97::C1))
        }
        Adt103::C3(v2) => {
            f73(f76(v2), v1, Adt97::C1)
        }
    }
}

pub fn f47(v0: List<Adt36>) -> Option<Adt98> {
    let m144 = f77(v0, List::<Str>::nil());
    match m144 {
        None => {
            None::<Adt98>
        }
        Some(v1) => {
            Some(Adt98::C1(v1))
        }
    }
}

pub fn f48(p148: List<Adt36>, p149: List<Adt12>) -> R<Result<List<Adt12>, Adt98>> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v0, v1) = m147;
                let m145 = v0;
                match m145.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Ok::<List<Adt12>, Adt98>(f78(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m146 = f79(v2)?;
                        match m146 {
                            Ok(v5) => {
                                Ok::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>((v3, f78(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt36>, List<Adt12>), R<Result<List<Adt12>, Adt98>>>(Ok(Err::<List<Adt12>, Adt98>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f49(v0: Str, v1: Str, v2: Adt5) -> Adt36 {
    Adt36::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt37>, None::<Adt5>)
}

pub fn f50(v0: u64) -> R<Str> {
    let a151 = List::cons(Str::lit("crowd_"), List::cons({
        let a150 = f80(v0, 0i64)?;
        format_i64(a150)
    }, List::<Str>::nil()));
    let a152 = Str::lit("");
    Ok(join(a151, a152))
}

pub fn f51() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f52(v0: Adt98) -> Str {
    let m153 = v0;
    match m153 {
        Adt98::C0(v1) => {
            let a154 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a155 = Str::lit(" ");
            join(a154, a155)
        }
        Adt98::C1(v2) => {
            let a156 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a157 = Str::lit(" ");
            join(a156, a157)
        }
        Adt98::C2(v3) => {
            let a158 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a159 = Str::lit(" ");
            join(a158, a159)
        }
        Adt98::C3(v4) => {
            let a160 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a161 = Str::lit(" ");
            join(a160, a161)
        }
        Adt98::C4(v5, v6) => {
            let a162 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a163 = Str::lit(" ");
            join(a162, a163)
        }
        Adt98::C5(v7, v8) => {
            let a164 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a165 = Str::lit(" ");
            join(a164, a165)
        }
        Adt98::C6(v9, v10) => {
            let a166 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a167 = Str::lit(" ");
            join(a166, a167)
        }
        Adt98::C7(v11) => {
            let a168 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a169 = Str::lit(" ");
            join(a168, a169)
        }
        Adt98::C8(v12) => {
            let a170 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a171 = Str::lit(" ");
            join(a170, a171)
        }
        Adt98::C9(v13) => {
            let a172 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a173 = Str::lit(" ");
            join(a172, a173)
        }
        Adt98::C10(v14) => {
            let a174 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a175 = Str::lit(" ");
            join(a174, a175)
        }
        Adt98::C11(v15) => {
            let a176 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a177 = Str::lit(" ");
            join(a176, a177)
        }
        Adt98::C12(v16, v17) => {
            let a178 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a179 = Str::lit(" ");
            join(a178, a179)
        }
    }
}

pub fn f53(v0: Adt10, v1: List<Adt36>, v2: Adt21) -> Result<Adt33, Adt90> {
    let m181 = match v2 {
        Adt21::C0(_, h180) => {
            h180
        }
    };
    match m181 {
        None => {
            Ok::<Adt33, Adt90>(Adt33::C1(Adt35::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt33, Adt90>(Adt90::C5(v3))
        }
    }
}

pub fn f54(v0: List<Adt36>, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m183 = f81(match v2.clone() {
        Adt22::C0(_, h182, _, _, _) => {
            h182
        }
    }, v0);
    match m183 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C0(match v2 {
                Adt22::C0(_, h184, _, _, _) => {
                    h184
                }
            })))
        }
        Some(v3) => {
            f82(v3, v1, v2)
        }
    }
}

pub fn f55(v0: Adt22) -> Result<Adt50, Adt90> {
    Ok::<Adt50, Adt90>(f83(List::cons(f84(match v0 {
        Adt22::C0(_, h185, _, _, _) => {
            h185
        }
    }), List::<Adt39>::nil())))
}

pub fn f56(v0: Bytes, v1: u64) -> R<u64> {
    let m188 = {
        let a186 = v0;
        let a187 = v1;
        index_bytes(a186, a187)
    };
    match m188 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a233 = {
                let a209 = {
                    let a197 = {
                        let a191 = {
                            let a189 = v3;
                            let a190 = 128u8;
                            fixed_u8::bit_and(a189, a190)
                        };
                        let a192 = 128u8;
                        if equal(a191, a192) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a198 = {
                        let a195 = {
                            let a193 = v3;
                            let a194 = 64u8;
                            fixed_u8::bit_and(a193, a194)
                        };
                        let a196 = 64u8;
                        if equal(a195, a196) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a197, a198)?
                };
                let a210 = {
                    let a207 = {
                        let a201 = {
                            let a199 = v3;
                            let a200 = 32u8;
                            fixed_u8::bit_and(a199, a200)
                        };
                        let a202 = 32u8;
                        if equal(a201, a202) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a208 = {
                        let a205 = {
                            let a203 = v3;
                            let a204 = 16u8;
                            fixed_u8::bit_and(a203, a204)
                        };
                        let a206 = 16u8;
                        if equal(a205, a206) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a207, a208)?
                };
                nat_add(a209, a210)?
            };
            let a234 = {
                let a231 = {
                    let a219 = {
                        let a213 = {
                            let a211 = v3;
                            let a212 = 8u8;
                            fixed_u8::bit_and(a211, a212)
                        };
                        let a214 = 8u8;
                        if equal(a213, a214) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a220 = {
                        let a217 = {
                            let a215 = v3;
                            let a216 = 4u8;
                            fixed_u8::bit_and(a215, a216)
                        };
                        let a218 = 4u8;
                        if equal(a217, a218) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a219, a220)?
                };
                let a232 = {
                    let a229 = {
                        let a223 = {
                            let a221 = v3;
                            let a222 = 2u8;
                            fixed_u8::bit_and(a221, a222)
                        };
                        let a224 = 2u8;
                        if equal(a223, a224) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a230 = {
                        let a227 = {
                            let a225 = v3;
                            let a226 = 1u8;
                            fixed_u8::bit_and(a225, a226)
                        };
                        let a228 = 1u8;
                        if equal(a227, a228) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a229, a230)?
                };
                nat_add(a231, a232)?
            };
            nat_add(a233, a234)
        }
    }
}

pub fn f57(v0: Bytes, p244: u64, p245: u64, p246: List<Adt93>) -> R<Result<(List<Adt93>, u64), Adt94>> {
    let m243 = (p244, (p245, p246));
    {
        let mut m243 = m243;
        loop {
            let step = {
                let (v1, (v2, v3)) = m243;
                let m235 = v2;
                if m235 == 0 {
                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Ok::<(List<Adt93>, u64), Adt94>((f85(v3, List::<Adt93>::nil()), v1))))
                } else {
                    let v4 = m235 - 1;
                    let m236 = f15(v0.clone(), v1)?;
                    match m236 {
                        None => {
                            Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            let m238 = f15(v0.clone(), match v5.clone() {
                                Adt96::C0(_, h237) => {
                                    h237
                                }
                            })?;
                            match m238 {
                                None => {
                                    Err::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>(Ok(Err::<(List<Adt93>, u64), Adt94>(Adt94::C0(match v5 {
                                        Adt96::C0(_, h239) => {
                                            h239
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt93>)), R<Result<(List<Adt93>, u64), Adt94>>>((match v6.clone() {
                                        Adt96::C0(_, h240) => {
                                            h240
                                        }
                                    }, (v4, List::cons(Adt93::C0(f86(match v5 {
                                        Adt96::C0(h241, _) => {
                                            h241
                                        }
                                    })?, f86(match v6 {
                                        Adt96::C0(h242, _) => {
                                            h242
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f58(v0: Bytes, v1: Adt96, v2: Adt96, v3: (List<Adt93>, u64)) -> R<Result<Adt92, Adt94>> {
    let m248 = f15(v0.clone(), {
        let (_, h247) = v3.clone();
        h247
    })?;
    match m248 {
        None => {
            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                let (_, h249) = v3;
                h249
            })))
        }
        Some(v4) => {
            let m251 = f32(v0.clone(), match v4.clone() {
                Adt96::C0(_, h250) => {
                    h250
                }
            })?;
            match m251 {
                None => {
                    Ok(Err::<Adt92, Adt94>(Adt94::C0(match v4 {
                        Adt96::C0(_, h252) => {
                            h252
                        }
                    })))
                }
                Some(v5) => {
                    let m256 = f87(v0.clone(), {
                        let a254 = match v4.clone() {
                            Adt96::C0(_, h253) => {
                                h253
                            }
                        };
                        let a255 = 4u64;
                        nat_add(a254, a255)?
                    })?;
                    match m256 {
                        None => {
                            Ok(Err::<Adt92, Adt94>(Adt94::C0({
                                let a258 = match v4 {
                                    Adt96::C0(_, h257) => {
                                        h257
                                    }
                                };
                                let a259 = 4u64;
                                nat_add(a258, a259)?
                            })))
                        }
                        Some(v6) => {
                            f88(v0, Adt92::C0(f86(match v1 {
                                Adt96::C0(h260, _) => {
                                    h260
                                }
                            })?, f86(match v2 {
                                Adt96::C0(h261, _) => {
                                    h261
                                }
                            })?, {
                                let (h262, _) = v3;
                                h262
                            }, match v4 {
                                Adt96::C0(h263, _) => {
                                    h263
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt96::C0(h264, _) => {
                                    h264
                                }
                            }, Adt14::C0(0i64, Str::lit(""), Str::lit(""), f89())), match v6 {
                                Adt96::C0(_, h265) => {
                                    h265
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f59(v0: Adt93) -> List<u64> {
    let a276 = f17({
        let a267 = match v0.clone() {
            Adt93::C0(h266, _) => {
                h266
            }
        };
        length_list(a267)
    });
    let a277 = {
        let a274 = match v0.clone() {
            Adt93::C0(h268, _) => {
                h268
            }
        };
        let a275 = {
            let a272 = f17({
                let a270 = match v0.clone() {
                    Adt93::C0(_, h269) => {
                        h269
                    }
                };
                length_list(a270)
            });
            let a273 = match v0 {
                Adt93::C0(_, h271) => {
                    h271
                }
            };
            append_list(a272, a273)
        };
        append_list(a274, a275)
    };
    append_list(a276, a277)
}

pub fn f60(v0: List<u64>) -> List<u64> {
    let a279 = f17({
        let a278 = v0.clone();
        length_list(a278)
    });
    let a280 = v0;
    append_list(a279, a280)
}

pub fn f61(v0: u64, v1: Adt6) -> R<Adt95> {
    let m281 = f90(v1)?;
    match m281 {
        Ok(v3) => {
            Ok(Adt95::C0(v0, f22(), List::<List<u64>>::nil(), v3, None::<List<u64>>))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f62(v0: Adt97) -> u64 {
    let m282 = v0;
    match m282 {
        Adt97::C0 => {
            f91()
        }
        Adt97::C1 => {
            f91()
        }
    }
}

pub fn f63(v0: Option<Adt99>, v1: Adt81) -> Adt6 {
    Adt6::C5(f92(v0, v1))
}

pub fn f64(v0: Adt97) -> Adt81 {
    Adt81::C0(f93(), f94(v0), None::<Adt6>)
}

pub fn f65(v0: List<u64>) -> bool {
    f95(v0, f96())
}

pub fn f66(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt93>, v5: Result<Adt6, Adt100>) -> R<Adt95> {
    let m283 = v5;
    match m283 {
        Ok(v7) => {
            f99(v0, v1, v2, v3, v4, f100(v7)?)
        }
        Err(_) => {
            f61(f97(), f63(None::<Adt99>, f98(Adt101::C0)))
        }
    }
}

pub fn f67(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h284, _, _, _, _, _, _) => {
            h284
        }
    }, match v0.clone() {
        Adt0::C0(_, h285, _, _, _, _, _) => {
            h285
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h286, _, _, _, _) => {
            h286
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h287, _, _, _) => {
            h287
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h288, _, _) => {
            h288
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h289, _) => {
            h289
        }
    }, v1)
}

pub fn f68(v0: Fn0, v1: Adt92, v2: Adt15) -> R<Result<Adt33, Adt90>> {
    let c290: Fn0 = v0;
    c290.apply(match v1 {
        Adt92::C0(_, _, _, _, _, _, _, h291) => {
            h291
        }
    }, v2)
}

pub fn f69(v0: Bytes) -> R<Result<Adt6, Adt100>> {
    f101(f86(v0)?)
}

pub fn f70() -> Adt95 {
    Adt95::C0(f102(), List::cons(Adt93::C0(f103(), f104()), List::<Adt93>::nil()), List::<List<u64>>::nil(), List::<u64>::nil(), None::<List<u64>>)
}

pub fn f71(v0: List<Adt93>, v1: List<u64>) -> R<Adt103> {
    let m292 = f105(v0, v1, Adt104::C0)?;
    match m292 {
        Adt104::C0 => {
            Ok(Adt103::C0)
        }
        Adt104::C1(v2) => {
            let m293 = f106(v2);
            match m293 {
                None => {
                    Ok(Adt103::C1)
                }
                Some(v3) => {
                    Ok(Adt103::C3(v3))
                }
            }
        }
        Adt104::C2 => {
            Ok(Adt103::C2)
        }
    }
}

pub fn f72() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f73(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt97) -> R<Option<Adt97>> {
    let m294 = v0;
    match m294 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f107(v3, v1)? {
                Ok(None::<Adt97>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f74(v0: List<u64>) -> Option<List<u64>> {
    let m295 = f108(v0);
    match m295 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            f76(v1)
        }
    }
}

pub fn f75() -> List<u64> {
    List::cons(72u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil()))))
}

pub fn f76(v0: List<u64>) -> Option<List<u64>> {
    let m296 = v0.clone();
    match m296.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a297 = v1;
            let a298 = 91u64;
            if nat_eq(a297, a298) {
                f109(f110(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f109(f111(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f77(p304: List<Adt36>, p305: List<Str>) -> Option<Str> {
    let m303 = (p304, p305);
    {
        let mut m303 = m303;
        loop {
            let step = {
                let (v0, v1) = m303;
                let m299 = v0;
                match m299.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f112(match v2.clone() {
                            Adt36::C0(h300, _, _, _, _, _, _, _) => {
                                h300
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt36>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt36::C0(h301, _, _, _, _, _, _, _) => {
                                    h301
                                }
                            }))
                        } else {
                            Ok::<(List<Adt36>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt36::C0(h302, _, _, _, _, _, _, _) => {
                                    h302
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(p308: List<Adt12>, p309: List<Adt12>) -> List<Adt12> {
    let m307 = (p308, p309);
    {
        let mut m307 = m307;
        loop {
            let step = {
                let (v0, v1) = m307;
                let m306 = v0;
                match m306.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
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

pub fn f79(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m312 = f113(match v0.clone() {
        Adt36::C0(h310, _, _, _, _, _, _, _) => {
            h310
        }
    }, f114(match v0.clone() {
        Adt36::C0(h311, _, _, _, _, _, _, _) => {
            h311
        }
    })?);
    match m312 {
        None => {
            f115(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f80(p315: u64, p316: i64) -> R<i64> {
    let m314 = (p315, p316);
    {
        let mut m314 = m314;
        loop {
            let step = {
                let (v0, v1) = m314;
                let m313 = v0;
                if m313 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m313 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f116(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Str, p322: List<Adt36>) -> Option<Adt36> {
    let m321 = p322;
    {
        let mut m321 = m321;
        loop {
            let step = {
                let v1 = m321;
                let m317 = v1;
                match m317.uncons() {
                    None => {
                        Err::<List<Adt36>, Option<Adt36>>(None::<Adt36>)
                    }
                    Some((v2, v3)) => {
                        let a319 = match v2.clone() {
                            Adt36::C0(h318, _, _, _, _, _, _, _) => {
                                h318
                            }
                        };
                        let a320 = v0.clone();
                        if equal(a319, a320) {
                            Err::<List<Adt36>, Option<Adt36>>(Some(v2))
                        } else {
                            Ok::<List<Adt36>, Option<Adt36>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m321 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f82(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f117(v0.clone(), v2, Fn3::F118(v1, v0))
}

pub fn f83(v0: List<Adt39>) -> Adt50 {
    f119(v0, None::<Adt6>, false, List::<Adt51>::nil())
}

pub fn f84(v0: Str) -> Adt39 {
    Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>))
}

pub fn f85(p325: List<Adt93>, p326: List<Adt93>) -> List<Adt93> {
    let m324 = (p325, p326);
    {
        let mut m324 = m324;
        loop {
            let step = {
                let (v0, v1) = m324;
                let m323 = v0;
                match m323.uncons() {
                    None => {
                        Err::<(List<Adt93>, List<Adt93>), List<Adt93>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt93>, List<Adt93>), List<Adt93>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m324 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes) -> R<List<u64>> {
    f120(v0.clone(), {
        let a327 = v0;
        length_bytes(a327)
    }, List::<u64>::nil())
}

pub fn f87(v0: Bytes, v1: u64) -> R<Option<Adt96>> {
    let a329 = v1;
    let a330 = {
        let a328 = v0.clone();
        length_bytes(a328)
    };
    if nat_eq(a329, a330) {
        Ok(Some(Adt96::C0(Bytes::lit(&[]), v1)))
    } else {
        f15(v0, v1)
    }
}

pub fn f88(v0: Bytes, v1: Adt92, v2: u64) -> R<Result<Adt92, Adt94>> {
    let a332 = v2;
    let a333 = {
        let a331 = v0.clone();
        length_bytes(a331)
    };
    if nat_eq(a332, a333) {
        Ok(Ok::<Adt92, Adt94>(v1))
    } else {
        let m334 = f32(v0.clone(), v2)?;
        match m334 {
            None => {
                Ok(Err::<Adt92, Adt94>(Adt94::C0(v2)))
            }
            Some(v3) => {
                let m337 = f121(v0.clone(), {
                    let a335 = v2;
                    let a336 = 4u64;
                    nat_add(a335, a336)?
                }, v3, List::<List<u64>>::nil())?;
                match m337 {
                    Ok(v5) => {
                        Ok(f122(v0.clone(), f123(v1, {
                            let (h338, _) = v5;
                            h338
                        }), f124(v0, {
                            let (_, h339) = v5;
                            h339
                        })?))
                    }
                    Err(v4) => {
                        Ok(Err::<Adt92, Adt94>(v4))
                    }
                }
            }
        }
    }
}

pub fn f89() -> Bytes {
    Bytes::lit(&[])
}

pub fn f90(v0: Adt6) -> R<Result<List<u64>, Adt100>> {
    f125(f126(), v0)
}

pub fn f91() -> u64 {
    403u64
}

pub fn f92(v0: Option<Adt99>, v1: Adt81) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), {
        let a340 = f128(v0);
        let a341 = List::cons((Str::lit("error"), f129(v1)), List::<(Str, Adt6)>::nil());
        append_list(a340, a341)
    })
}

pub fn f93() -> i64 {
    -32000i64
}

pub fn f94(v0: Adt97) -> Str {
    let m342 = v0;
    match m342 {
        Adt97::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt97::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f95(p349: List<u64>, p350: List<u64>) -> bool {
    let m348 = (p349, p350);
    {
        let mut m348 = m348;
        loop {
            let step = {
                let (v0, v1) = m348;
                let m343 = v0;
                match m343.uncons() {
                    None => {
                        let m344 = v1;
                        match m344.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m345 = v1;
                        match m345.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a346 = v4;
                                let a347 = v6;
                                if nat_eq(a346, a347) {
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
                Ok(next) => { m348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f97() -> u64 {
    400u64
}

pub fn f98(v0: Adt101) -> Adt81 {
    Adt81::C0(f130(v0.clone()), f131(v0), None::<Adt6>)
}

pub fn f99(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt93>, v5: Result<Adt105, Adt101>) -> R<Adt95> {
    let m351 = v5;
    match m351 {
        Ok(v7) => {
            f134(v0, v1, v2, v3, v4, v7)
        }
        Err(v6) => {
            f61(f132(v6.clone()), f63(f133(v6.clone()), f98(v6)))
        }
    }
}

pub fn f100(v0: Adt6) -> R<Result<Adt105, Adt101>> {
    let m352 = v0;
    match m352 {
        Adt6::C0 => {
            Ok(f136(None::<Adt99>, Adt102::C0))
        }
        Adt6::C1(_) => {
            Ok(f136(None::<Adt99>, Adt102::C0))
        }
        Adt6::C2(_) => {
            Ok(f136(None::<Adt99>, Adt102::C0))
        }
        Adt6::C3(_) => {
            Ok(f136(None::<Adt99>, Adt102::C0))
        }
        Adt6::C4(_) => {
            Ok(f136(None::<Adt99>, Adt102::C1))
        }
        Adt6::C5(v1) => {
            f135(Adt5::C0(v1))
        }
    }
}

pub fn f101(v0: List<u64>) -> R<Result<Adt6, Adt100>> {
    let m353 = f137(v0.clone())?;
    match m353 {
        Adt113::C0 => {
            f138(f139(), 0u64, v0)
        }
        Adt113::C1(v1) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C0(v1)))
        }
        Adt113::C2(v2) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C1(v2)))
        }
    }
}

pub fn f102() -> u64 {
    405u64
}

pub fn f103() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f104() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f105(p358: List<Adt93>, v1: List<u64>, p359: Adt104) -> R<Adt104> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v0, v2) = m357;
                let m354 = v0;
                match m354.uncons() {
                    None => {
                        Err::<(List<Adt93>, Adt104), R<Adt104>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f140(match v3.clone() {
                            Adt93::C0(h355, _) => {
                                h355
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, f141(v2, match v3 {
                                Adt93::C0(_, h356) => {
                                    h356
                                }
                            })))
                        } else {
                            Ok::<(List<Adt93>, Adt104), R<Adt104>>((v4, v2))
                        }
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

pub fn f106(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f142(v0);
    if f143(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f107(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f140(v0.clone(), f144())? {
        Ok(true)
    } else {
        if f140(v0.clone(), f145())? {
            Ok(true)
        } else {
            if f140(v0.clone(), f146())? {
                Ok(true)
            } else {
                f147(v0, v1)
            }
        }
    }
}

pub fn f108(p364: List<u64>) -> Option<List<u64>> {
    let m363 = p364;
    {
        let mut m363 = m363;
        loop {
            let step = {
                let v0 = m363;
                let m360 = v0;
                match m360.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a361 = v1;
                        let a362 = 58u64;
                        if nat_eq(a361, a362) {
                            Err::<List<u64>, Option<List<u64>>>(f148(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m363 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m365 = v0;
    match m365 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f149({
                let (_, h366) = v1;
                h366
            }) {
                Some({
                    let (h367, _) = v1;
                    h367
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f110(p372: List<u64>, p373: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m371 = (p372, p373);
    {
        let mut m371 = m371;
        loop {
            let step = {
                let (v0, v1) = m371;
                let m368 = v0;
                match m368.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a369 = v2;
                        let a370 = 93u64;
                        if nat_eq(a369, a370) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
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

pub fn f111(p378: List<u64>, p379: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m377 = (p378, p379);
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v0, v1) = m377;
                let m374 = v0;
                match m374.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a375 = v2;
                        let a376 = 58u64;
                        if nat_eq(a375, a376) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f150(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Str, p384: List<Str>) -> bool {
    let m383 = p384;
    {
        let mut m383 = m383;
        loop {
            let step = {
                let v1 = m383;
                let m380 = v1;
                match m380.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a381 = v0.clone();
                        let a382 = v2;
                        if equal(a381, a382) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f113(v0: Str, v1: List<u64>) -> Option<Adt98> {
    if f151(v1) {
        Some(Adt98::C0(v0))
    } else {
        None::<Adt98>
    }
}

pub fn f114(v0: Str) -> R<List<u64>> {
    f86({
        let a385 = v0;
        utf8_encode(a385)
    })
}

pub fn f115(v0: Adt36) -> R<Result<List<Adt12>, Adt98>> {
    let m386 = f152(v0.clone());
    match m386 {
        None => {
            f153(v0.clone(), f154(f155(), match v0.clone() {
                Adt36::C0(h387, _, _, _, _, _, _, _) => {
                    h387
                }
            }, Adt6::C5({
                let m390 = match v0.clone() {
                    Adt36::C0(_, _, _, _, h389, _, _, _) => {
                        h389
                    }
                };
                match m390 {
                    Adt5::C0(h388) => {
                        h388
                    }
                }
            }), List::cons(Adt106::C0(Adt6::C5({
                let m393 = match v0 {
                    Adt36::C0(_, _, _, _, h392, _, _, _) => {
                        h392
                    }
                };
                match m393 {
                    Adt5::C0(h391) => {
                        h391
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt106>::nil()), f156())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt98>(v1))
        }
    }
}

pub fn f116(v0: i64, v1: i64) -> R<i64> {
    let a395 = v0;
    let a396 = {
        let a394 = v1;
        int_neg(a394)?
    };
    int_sub(a395, a396)
}

pub fn f117(v0: Adt36, v1: Adt22, v2: Fn3) -> R<Result<Adt33, Adt90>> {
    let m403 = f157(Adt6::C5({
        let m399 = match v0.clone() {
            Adt36::C0(_, _, _, _, h398, _, _, _) => {
                h398
            }
        };
        match m399 {
            Adt5::C0(h397) => {
                h397
            }
        }
    }), Adt6::C5({
        let m402 = match v0 {
            Adt36::C0(_, _, _, _, h401, _, _, _) => {
                h401
            }
        };
        match m402 {
            Adt5::C0(h400) => {
                h400
            }
        }
    }), f158(v1.clone()))?;
    match m403 {
        Adt122::C0 => {
            let c404: Fn3 = v2;
            c404.apply(v1)
        }
        Adt122::C1(v3, v4) => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(f159(f160(v3, v4)?, true))))
        }
        Adt122::C2(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt122::C3(_) => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt122::C4 => {
            Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f118(v0: Fn1, v1: Adt36, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    f161(v1, v0, v2)
}

pub fn f119(v0: List<Adt39>, v1: Option<Adt6>, v2: bool, v3: List<Adt51>) -> Adt50 {
    Adt50::C0(Adt38::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f120(v0: Bytes, p407: u64, p408: List<u64>) -> R<List<u64>> {
    let m406 = (p407, p408);
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v1, v2) = m406;
                let m405 = v1;
                if m405 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m405 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f56(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, p414: u64, p415: u64, p416: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt94>> {
    let m413 = (p414, (p415, p416));
    {
        let mut m413 = m413;
        loop {
            let step = {
                let (v1, (v2, v3)) = m413;
                let m409 = v2;
                if m409 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Ok::<(List<List<u64>>, u64), Adt94>((f37(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m409 - 1;
                    let m410 = f15(v0.clone(), v1)?;
                    match m410 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>(Ok(Err::<(List<List<u64>>, u64), Adt94>(Adt94::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt94>>>((match v5.clone() {
                                Adt96::C0(_, h411) => {
                                    h411
                                }
                            }, (v4, List::cons(f86(match v5 {
                                Adt96::C0(h412, _) => {
                                    h412
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Bytes, v1: Adt92, v2: Result<(Adt14, u64), Adt94>) -> Result<Adt92, Adt94> {
    let m417 = v2;
    match m417 {
        Ok(v4) => {
            let a420 = {
                let (_, h418) = v4.clone();
                h418
            };
            let a421 = {
                let a419 = v0;
                length_bytes(a419)
            };
            if nat_eq(a420, a421) {
                Ok::<Adt92, Adt94>(Adt92::C0(match v1.clone() {
                    Adt92::C0(h422, _, _, _, _, _, _, _) => {
                        h422
                    }
                }, match v1.clone() {
                    Adt92::C0(_, h423, _, _, _, _, _, _) => {
                        h423
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, h424, _, _, _, _, _) => {
                        h424
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, h425, _, _, _, _) => {
                        h425
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, _, h426, _, _, _) => {
                        h426
                    }
                }, match v1.clone() {
                    Adt92::C0(_, _, _, _, _, h427, _, _) => {
                        h427
                    }
                }, match v1 {
                    Adt92::C0(_, _, _, _, _, _, h428, _) => {
                        h428
                    }
                }, {
                    let (h429, _) = v4;
                    h429
                }))
            } else {
                Err::<Adt92, Adt94>(Adt94::C1({
                    let (_, h430) = v4;
                    h430
                }))
            }
        }
        Err(v3) => {
            Err::<Adt92, Adt94>(v3)
        }
    }
}

pub fn f123(v0: Adt92, v1: List<List<u64>>) -> Adt92 {
    Adt92::C0(match v0.clone() {
        Adt92::C0(h431, _, _, _, _, _, _, _) => {
            h431
        }
    }, match v0.clone() {
        Adt92::C0(_, h432, _, _, _, _, _, _) => {
            h432
        }
    }, match v0.clone() {
        Adt92::C0(_, _, h433, _, _, _, _, _) => {
            h433
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, h434, _, _, _, _) => {
            h434
        }
    }, match v0.clone() {
        Adt92::C0(_, _, _, _, h435, _, _, _) => {
            h435
        }
    }, v1, match v0.clone() {
        Adt92::C0(_, _, _, _, _, _, h436, _) => {
            h436
        }
    }, match v0 {
        Adt92::C0(_, _, _, _, _, _, _, h437) => {
            h437
        }
    })
}

pub fn f124(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let a439 = v1;
    let a440 = {
        let a438 = v0.clone();
        length_bytes(a438)
    };
    if nat_eq(a439, a440) {
        Ok(Ok::<(Adt14, u64), Adt94>((Adt14::C0(0i64, Str::lit(""), Str::lit(""), f89()), v1)))
    } else {
        f162(v0, v1)
    }
}

pub fn f125(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt100>> {
    let m441 = f163(v0, List::cons(Adt107::C1(0u64, v1), List::<Adt107>::nil()), List::<u64>::nil())?;
    match m441 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt100>(f164(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt100>(v3))
        }
    }
}

pub fn f126() -> u64 {
    67108864u64
}

pub fn f127() -> Str {
    Str::lit("2.0")
}

pub fn f128(v0: Option<Adt99>) -> List<(Str, Adt6)> {
    let m442 = v0;
    match m442 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f165(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f129(v0: Adt81) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt81::C0(h443, _, _) => {
            h443
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt81::C0(_, h444, _) => {
            h444
        }
    })), f166(match v0 {
        Adt81::C0(_, _, h445) => {
            h445
        }
    }))))
}

pub fn f130(v0: Adt101) -> i64 {
    let m446 = v0;
    match m446 {
        Adt101::C0 => {
            f167()
        }
        Adt101::C1(_, _) => {
            f168()
        }
    }
}

pub fn f131(v0: Adt101) -> Str {
    let m447 = v0;
    match m447 {
        Adt101::C0 => {
            Str::lit("Parse error")
        }
        Adt101::C1(_, v2) => {
            f169(v2)
        }
    }
}

pub fn f132(v0: Adt101) -> u64 {
    let m448 = v0;
    match m448 {
        Adt101::C0 => {
            f97()
        }
        Adt101::C1(_, _) => {
            f97()
        }
    }
}

pub fn f133(v0: Adt101) -> Option<Adt99> {
    let m449 = v0;
    match m449 {
        Adt101::C0 => {
            None::<Adt99>
        }
        Adt101::C1(v1, _) => {
            v1
        }
    }
}

pub fn f134(v0: Adt14, v1: Adt0, v2: Fn2, v3: u64, v4: List<Adt93>, v5: Adt105) -> R<Adt95> {
    f170(v0, f171(v4.clone())?, f172(v1.clone(), v2, v3, Some(f173(v4, match v1 {
        Adt0::C0(_, _, _, _, h450, _, _) => {
            h450
        }
    }, f174(v5.clone()), match v5.clone() {
        Adt105::C0(_, h451, _) => {
            h451
        }
    })?), f175(), v5)?)
}

pub fn f135(v0: Adt5) -> R<Result<Adt105, Adt101>> {
    let m452 = f176(v0.clone())?;
    match m452 {
        Adt134::C0 => {
            let m453 = f177(Str::lit("method"), v0.clone());
            match m453 {
                Adt135::C0 => {
                    Ok(f136(None::<Adt99>, f178(v0)))
                }
                Adt135::C1(_) => {
                    Ok(f136(None::<Adt99>, if f179(v0) {
                        Adt102::C8
                    } else {
                        Adt102::C7
                    }))
                }
            }
        }
        Adt134::C1(v2) => {
            Ok(f180(v2, v0))
        }
        Adt134::C2 => {
            Ok(f136(None::<Adt99>, Adt102::C5))
        }
    }
}

pub fn f136(v0: Option<Adt99>, v1: Adt102) -> Result<Adt105, Adt101> {
    Err::<Adt105, Adt101>(Adt101::C1(v0, v1))
}

pub fn f137(v0: List<u64>) -> R<Adt113> {
    Ok(f181({
        let a454 = v0.clone();
        length_list(a454)
    }, f182(Adt114::C0, 0u64, 0u64, v0)?))
}

pub fn f138(p461: Adt115, p462: u64, p463: List<u64>) -> R<Result<Adt6, Adt100>> {
    let m460 = (p461, (p462, p463));
    {
        let mut m460 = m460;
        loop {
            let step = {
                let (v0, (v1, v2)) = m460;
                let m455 = v2;
                match m455.uncons() {
                    None => {
                        let m456 = f183(v0, v1)?;
                        match m456 {
                            Ok(v3) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Ok::<Adt6, Adt100>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m457 = f184(v0, v1, v5)?;
                        match m457 {
                            Adt136::C0(v7) => {
                                Err::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>(Ok(Err::<Adt6, Adt100>(v7)))
                            }
                            Adt136::C1(v8) => {
                                Ok::<(Adt115, (u64, List<u64>)), R<Result<Adt6, Adt100>>>((v8, ({
                                    let a458 = v1;
                                    let a459 = 1u64;
                                    nat_add(a458, a459)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139() -> Adt115 {
    Adt115::C0(Adt116::C0, List::<Adt120>::nil(), 0u64)
}

pub fn f140(p470: List<u64>, p471: List<u64>) -> R<bool> {
    let m469 = (p470, p471);
    {
        let mut m469 = m469;
        loop {
            let step = {
                let (v0, v1) = m469;
                let m464 = v0;
                match m464.uncons() {
                    None => {
                        let m465 = v1;
                        match m465.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m466 = v1;
                        match m466.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a467 = f185(v4)?;
                                let a468 = f185(v6)?;
                                if nat_eq(a467, a468) {
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
                Ok(next) => { m469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141(v0: Adt104, v1: List<u64>) -> Adt104 {
    let m472 = v0;
    match m472 {
        Adt104::C0 => {
            Adt104::C1(v1)
        }
        Adt104::C1(_) => {
            Adt104::C2
        }
        Adt104::C2 => {
            Adt104::C2
        }
    }
}

pub fn f142(v0: List<u64>) -> List<u64> {
    f150(f186(f150(f186(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f143(p475: List<u64>) -> bool {
    let m474 = p475;
    {
        let mut m474 = m474;
        loop {
            let step = {
                let v0 = m474;
                let m473 = v0;
                match m473.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f187(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f144() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f145() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f146() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f147(v0: List<u64>, p478: List<List<u64>>) -> R<bool> {
    let m477 = p478;
    {
        let mut m477 = m477;
        loop {
            let step = {
                let v1 = m477;
                let m476 = v1;
                match m476.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f140(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<u64>) -> Option<List<u64>> {
    let m479 = v0;
    match m479.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m480 = v2;
            match m480.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a481 = v1;
                    let a482 = 47u64;
                    if if nat_eq(a481, a482) {
                        let a483 = v3;
                        let a484 = 47u64;
                        nat_eq(a483, a484)
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

pub fn f149(v0: List<u64>) -> bool {
    let m485 = v0;
    match m485.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a486 = v1;
            let a487 = 58u64;
            if nat_eq(a486, a487) {
                let m488 = v2.clone();
                match m488.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f188(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f150(p491: List<u64>, p492: List<u64>) -> List<u64> {
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

pub fn f151(v0: List<u64>) -> bool {
    let m493 = v0.clone();
    match m493.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a497 = if f189(v0.clone(), false) {
                let a495 = {
                    let a494 = v0;
                    length_list(a494)
                };
                let a496 = f190();
                nat_le(a495, a496)
            } else {
                false
            };
            bool_not(a497)
        }
    }
}

pub fn f152(v0: Adt36) -> Option<Adt98> {
    let m502 = f191(match v0.clone() {
        Adt36::C0(h498, _, _, _, _, _, _, _) => {
            h498
        }
    }, {
        let m501 = match v0.clone() {
            Adt36::C0(_, _, _, _, h500, _, _, _) => {
                h500
            }
        };
        match m501 {
            Adt5::C0(h499) => {
                h499
            }
        }
    });
    match m502 {
        None => {
            f192(match v0.clone() {
                Adt36::C0(h503, _, _, _, _, _, _, _) => {
                    h503
                }
            }, {
                let m506 = match v0 {
                    Adt36::C0(_, _, _, _, h505, _, _, _) => {
                        h505
                    }
                };
                match m506 {
                    Adt5::C0(h504) => {
                        h504
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f153(v0: Adt36, v1: Result<Adt121, Adt98>) -> R<Result<List<Adt12>, Adt98>> {
    let m507 = v1;
    match m507 {
        Ok(v3) => {
            let m508 = f193(v0)?;
            match m508 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt98>(f78(match v3 {
                        Adt121::C0(h509, _) => {
                            h509
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt98>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt98>(v2))
        }
    }
}

pub fn f154(p516: u64, v1: Str, v2: Adt6, p517: List<Adt106>, p518: Adt121) -> R<Result<Adt121, Adt98>> {
    let m515 = (p516, (p517, p518));
    {
        let mut m515 = m515;
        loop {
            let step = {
                let (v0, (v3, v4)) = m515;
                let m510 = v0;
                if m510 == 0 {
                    Err::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>(Ok(Err::<Adt121, Adt98>(Adt98::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m510 - 1;
                    let m511 = v3;
                    match m511.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>(Ok(Ok::<Adt121, Adt98>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m512 = f194(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m512 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>((v5, ({
                                        let (h513, _) = v9;
                                        h513
                                    }, {
                                        let (_, h514) = v9;
                                        h514
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt106>, Adt121)), R<Result<Adt121, Adt98>>>(Ok(Err::<Adt121, Adt98>(v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155() -> u64 {
    67108864u64
}

pub fn f156() -> Adt121 {
    Adt121::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f157(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt122> {
    f195(v0, f196(), Adt124::C0, List::cons(Adt125::C0(v1, v2, List::<Adt123>::nil()), List::<Adt125>::nil()))
}

pub fn f158(v0: Adt22) -> Adt6 {
    let m520 = match v0 {
        Adt22::C0(_, _, h519, _, _) => {
            h519
        }
    };
    match m520 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h521) => {
                    h521
                }
            })
        }
    }
}

pub fn f159(v0: Str, v1: bool) -> Adt38 {
    Adt38::C0(List::cons(Adt39::C0(Adt40::C0(v0, None::<Adt41>, None::<Adt5>)), List::<Adt39>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f160(v0: List<Adt123>, v1: Str) -> R<Str> {
    let v2: Str = f197(v0)?;
    let a524 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a522 = v2.clone();
        let a523 = Str::lit("");
        if equal(a522, a523) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a525 = Str::lit(" ");
    Ok(join(a524, a525))
}

pub fn f161(v0: Adt36, v1: Fn1, v2: Adt22) -> R<Result<Adt33, Adt90>> {
    let m527 = {
        let c526: Fn1 = v1;
        c526.apply(v2)
    };
    match m527 {
        Ok(v4) => {
            f198(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f162(v0: Bytes, v1: u64) -> R<Result<(Adt14, u64), Adt94>> {
    let m528 = f199(v0.clone(), v1)?;
    match m528 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m531 = f200(v0.clone(), {
                let a529 = v1;
                let a530 = 8u64;
                nat_add(a529, a530)?
            })?;
            match m531 {
                Ok(v4) => {
                    f201(v0, v2, v4)
                }
                Err(v3) => {
                    Ok(Err::<(Adt14, u64), Adt94>(v3))
                }
            }
        }
    }
}

pub fn f163(p536: u64, p537: List<Adt107>, p538: List<u64>) -> R<Result<List<u64>, Adt100>> {
    let m535 = (p536, (p537, p538));
    {
        let mut m535 = m535;
        loop {
            let step = {
                let (v0, (v1, v2)) = m535;
                let m532 = v0;
                if m532 == 0 {
                    Err::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Err::<List<u64>, Adt100>(Adt100::C13)))
                } else {
                    let v3 = m532 - 1;
                    let m533 = v1;
                    match m533.uncons() {
                        None => {
                            Err::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Ok::<List<u64>, Adt100>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m534 = f202(v4, v5, v2)?;
                            match m534 {
                                Adt138::C0(v6) => {
                                    Err::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>(Ok(Err::<List<u64>, Adt100>(v6)))
                                }
                                Adt138::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt107>, List<u64>)), R<Result<List<u64>, Adt100>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(p541: List<u64>, p542: List<u64>) -> List<u64> {
    let m540 = (p541, p542);
    {
        let mut m540 = m540;
        loop {
            let step = {
                let (v0, v1) = m540;
                let m539 = v0;
                match m539.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m540 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Adt99) -> Adt6 {
    let m543 = v0;
    match m543 {
        Adt99::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt99::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f166(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m544 = v0;
    match m544 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f167() -> i64 {
    -32700i64
}

pub fn f168() -> i64 {
    -32600i64
}

pub fn f169(v0: Adt102) -> Str {
    let m545 = v0;
    match m545 {
        Adt102::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt102::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt102::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt102::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt102::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt102::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt102::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt102::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt102::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f170(v0: Adt14, v1: bool, v2: Adt127) -> R<Adt95> {
    let m547 = match v2.clone() {
        Adt127::C0(_, h546) => {
            h546
        }
    };
    match m547 {
        Ok(v3) => {
            let m548 = f203(v3.clone())?;
            match m548 {
                None => {
                    Ok(f6())
                }
                Some(v4) => {
                    f204(v0, f205(match v3.clone() {
                        Adt128::C0(h549, _, _) => {
                            h549
                        }
                    }), f206(v1, f61(f207(), f208(match v2 {
                        Adt127::C0(h550, _) => {
                            h550
                        }
                    }, v4))?, f209(f210(match v3.clone() {
                        Adt128::C0(_, _, h551) => {
                            h551
                        }
                    }, List::<List<u64>>::nil())?, f211(f205(match v3 {
                        Adt128::C0(h552, _, _) => {
                            h552
                        }
                    }))?)))
                }
            }
        }
        Err(v5) => {
            f61(f212(v5.clone()), f63(Some(match v2 {
                Adt127::C0(h553, _) => {
                    h553
                }
            }), f213(v5)?))
        }
    }
}

pub fn f171(p558: List<Adt93>) -> R<bool> {
    let m557 = p558;
    {
        let mut m557 = m557;
        loop {
            let step = {
                let v0 = m557;
                let m554 = v0;
                match m554.uncons() {
                    None => {
                        Err::<List<Adt93>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f140(match v1.clone() {
                            Adt93::C0(h555, _) => {
                                h555
                            }
                        }, f214())? {
                            if f215(f216(44u64, match v1 {
                                Adt93::C0(_, h556) => {
                                    h556
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt93>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt93>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt93>, R<bool>>(v2)
                        }
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

pub fn f172(v0: Adt0, v1: Fn2, v2: u64, v3: Option<Adt108>, v4: List<Str>, v5: Adt105) -> R<Adt127> {
    Ok(Adt127::C0(match v5.clone() {
        Adt105::C0(h559, _, _) => {
            h559
        }
    }, f217(v0, v1, v2, f218(v3, v4, v5)?)?))
}

pub fn f173(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt108> {
    let v4: Adt133 = f219(Adt110::C0, f71(v0.clone(), f220())?);
    let v5: Adt133 = f219(Adt110::C1, f71(v0.clone(), f221())?);
    let v6: Adt133 = if f222(v3) {
        f219(Adt110::C2, f223(f71(v0.clone(), f224())?)?)
    } else {
        f225()
    };
    Ok(Adt108::C0(match v4.clone() {
        Adt133::C0(h560, _) => {
            h560
        }
    }, match v5.clone() {
        Adt133::C0(h561, _) => {
            h561
        }
    }, match v6.clone() {
        Adt133::C0(h562, _) => {
            h562
        }
    }, f226(match v4 {
        Adt133::C0(_, h563) => {
            h563
        }
    }, f226(match v5 {
        Adt133::C0(_, h564) => {
            h564
        }
    }, match v6 {
        Adt133::C0(_, h565) => {
            h565
        }
    })), f227(v0, v1, v2)?))
}

pub fn f174(v0: Adt105) -> Option<Str> {
    let a567 = match v0.clone() {
        Adt105::C0(_, h566, _) => {
            h566
        }
    };
    let a568 = Str::lit("tools/call");
    if equal(a567, a568) {
        let m570 = match v0 {
            Adt105::C0(_, _, h569) => {
                h569
            }
        };
        match m570 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m571 = f177(Str::lit("name"), v1);
                match m571 {
                    Adt135::C0 => {
                        None::<Str>
                    }
                    Adt135::C1(v2) => {
                        f228(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f175() -> List<Str> {
    List::cons(f229(), List::<Str>::nil())
}

pub fn f176(v0: Adt5) -> R<Adt134> {
    let m572 = f177(Str::lit("id"), v0);
    match m572 {
        Adt135::C0 => {
            Ok(Adt134::C0)
        }
        Adt135::C1(v1) => {
            let m573 = v1;
            match m573 {
                Adt6::C0 => {
                    Ok(Adt134::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt134::C2)
                }
                Adt6::C2(v3) => {
                    let m574 = f230(v3)?;
                    match m574 {
                        None => {
                            Ok(Adt134::C2)
                        }
                        Some(v4) => {
                            Ok(Adt134::C1(Adt99::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt134::C1(Adt99::C0(v2)))
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

pub fn f177(v0: Str, v1: Adt5) -> Adt135 {
    f231(v0, match v1 {
        Adt5::C0(h575) => {
            h575
        }
    })
}

pub fn f178(v0: Adt5) -> Adt102 {
    if f179(v0) {
        Adt102::C8
    } else {
        Adt102::C3
    }
}

pub fn f179(v0: Adt5) -> bool {
    if f232(Str::lit("result"), v0.clone()) {
        true
    } else {
        f232(Str::lit("error"), v0)
    }
}

pub fn f180(v0: Adt99, v1: Adt5) -> Result<Adt105, Adt101> {
    let m576 = f177(Str::lit("jsonrpc"), v1.clone());
    match m576 {
        Adt135::C0 => {
            f136(Some(v0), Adt102::C2)
        }
        Adt135::C1(v2) => {
            let m577 = f228(v2);
            match m577 {
                None => {
                    f136(Some(v0), Adt102::C2)
                }
                Some(v3) => {
                    let a578 = v3;
                    let a579 = f127();
                    if equal(a578, a579) {
                        f233(v0, v1)
                    } else {
                        f136(Some(v0), Adt102::C2)
                    }
                }
            }
        }
    }
}

pub fn f181(v0: u64, v1: u64) -> Adt113 {
    let a580 = v1;
    let a581 = 0u64;
    if nat_eq(a580, a581) {
        Adt113::C0
    } else {
        let a582 = v1;
        let a583 = 2u64;
        if nat_eq(a582, a583) {
            Adt113::C2(v0)
        } else {
            Adt113::C1({
                let a587 = v0;
                let a588 = {
                    let a584 = v1;
                    let a585 = 3u64;
                    let a586 = 0u64;
                    nat_quot(a584, a585, a586)
                };
                nat_sub(a587, a588)
            })
        }
    }
}

pub fn f182(p616: Adt114, p617: u64, p618: u64, p619: List<u64>) -> R<u64> {
    let m615 = (p616, (p617, (p618, p619)));
    {
        let mut m615 = m615;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m615;
                let m589 = v3;
                match m589.uncons() {
                    None => {
                        let m590 = v0;
                        match m590 {
                            Adt114::C0 => {
                                Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt114::C1 => {
                                Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt114::C2 => {
                                Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt114::C3 => {
                                Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m591 = v0;
                        match m591 {
                            Adt114::C0 => {
                                let a592 = v4;
                                let a593 = 128u64;
                                if nat_lt(a592, a593) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a594 = v4;
                                    let a595 = 194u64;
                                    if nat_lt(a594, a595) {
                                        let v6: u64 = f234({
                                            let a596 = List::cons(v4, v5);
                                            length_list(a596)
                                        })?;
                                        Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a597 = v4;
                                        let a598 = 224u64;
                                        if nat_lt(a597, a598) {
                                            Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a599 = v4;
                                            let a600 = 224u64;
                                            if nat_eq(a599, a600) {
                                                Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a601 = v4;
                                                let a602 = 237u64;
                                                if nat_eq(a601, a602) {
                                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a603 = v4;
                                                    let a604 = 240u64;
                                                    if nat_lt(a603, a604) {
                                                        Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a605 = v4;
                                                        let a606 = 240u64;
                                                        if nat_eq(a605, a606) {
                                                            Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a607 = v4;
                                                            let a608 = 244u64;
                                                            if nat_eq(a607, a608) {
                                                                Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a609 = v4;
                                                                let a610 = 244u64;
                                                                if nat_lt(a609, a610) {
                                                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f234({
                                                                        let a611 = List::cons(v4, v5);
                                                                        length_list(a611)
                                                                    })?;
                                                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt114::C1 => {
                                if f235(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f234({
                                        let a612 = List::cons(v4, v5);
                                        length_list(a612)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt114::C2 => {
                                if f235(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f234({
                                        let a613 = List::cons(v4, v5);
                                        length_list(a613)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt114::C3 => {
                                if f235(v4, v1, v2) {
                                    Ok::<(Adt114, (u64, (u64, List<u64>))), R<u64>>((Adt114::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f234({
                                        let a614 = List::cons(v4, v5);
                                        length_list(a614)
                                    })?;
                                    Err::<(Adt114, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
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

pub fn f183(v0: Adt115, v1: u64) -> R<Result<Adt6, Adt100>> {
    let m621 = match v0.clone() {
        Adt115::C0(h620, _, _) => {
            h620
        }
    };
    match m621 {
        Adt116::C0 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C1 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C2 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C3 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C4 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C5 => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C7(_, _) => {
            Ok(Err::<Adt6, Adt100>(Adt100::C3))
        }
        Adt116::C8(v3) => {
            let m622 = f236(v3)?;
            match m622 {
                Adt141::C0 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C7(v1)))
                }
                Adt141::C1 => {
                    Ok(Err::<Adt6, Adt100>(Adt100::C8(v1)))
                }
                Adt141::C2(v4) => {
                    let m627 = {
                        let m626 = f237(v4, match v0.clone() {
                            Adt115::C0(_, h624, _) => {
                                h624
                            }
                        }, match v0 {
                            Adt115::C0(_, _, h625) => {
                                h625
                            }
                        });
                        match m626 {
                            Adt115::C0(h623, _, _) => {
                                h623
                            }
                        }
                    };
                    match m627 {
                        Adt116::C0 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C1 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C2 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C3 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C4 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C5 => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C7(_, _) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C8(_) => {
                            Ok(Err::<Adt6, Adt100>(Adt100::C3))
                        }
                        Adt116::C9(v5) => {
                            Ok(Ok::<Adt6, Adt100>(v5))
                        }
                    }
                }
            }
        }
        Adt116::C9(v2) => {
            Ok(Ok::<Adt6, Adt100>(v2))
        }
    }
}

pub fn f184(v0: Adt115, v1: u64, v2: u64) -> R<Adt136> {
    let m629 = match v0.clone() {
        Adt115::C0(h628, _, _) => {
            h628
        }
    };
    match m629 {
        Adt116::C0 => {
            if f238(v2) {
                Ok(Adt136::C1(v0))
            } else {
                f239(v0, v1, v2)
            }
        }
        Adt116::C1 => {
            if f238(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a630 = v2;
                let a631 = 93u64;
                if nat_eq(a630, a631) {
                    Ok(f240(v0, v1))
                } else {
                    f239(v0, v1, v2)
                }
            }
        }
        Adt116::C2 => {
            Ok(f241(v0, v1, v2))
        }
        Adt116::C3 => {
            if f238(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a632 = v2;
                let a633 = 125u64;
                if nat_eq(a632, a633) {
                    Ok(f242(v0, v1))
                } else {
                    let a634 = v2;
                    let a635 = 34u64;
                    if nat_eq(a634, a635) {
                        Ok(f243(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f244(Adt100::C2(v1)))
                    }
                }
            }
        }
        Adt116::C4 => {
            if f238(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a636 = v2;
                let a637 = 34u64;
                if nat_eq(a636, a637) {
                    Ok(f243(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f244(Adt100::C2(v1)))
                }
            }
        }
        Adt116::C5 => {
            if f238(v2) {
                Ok(Adt136::C1(v0))
            } else {
                let a638 = v2;
                let a639 = 58u64;
                if nat_eq(a638, a639) {
                    Ok(f243(v0, Adt116::C0))
                } else {
                    Ok(f244(Adt100::C2(v1)))
                }
            }
        }
        Adt116::C6(v3, v4, v5) => {
            f245(v0, v1, v2, v3, v4, v5)
        }
        Adt116::C7(v6, v7) => {
            f246(v0, v1, v2, v6, v7)
        }
        Adt116::C8(v8) => {
            f247(v0, v1, v2, v8)
        }
        Adt116::C9(_) => {
            Ok(f241(v0, v1, v2))
        }
    }
}

pub fn f185(v0: u64) -> R<u64> {
    let a640 = 64u64;
    let a641 = v0;
    if if nat_lt(a640, a641) {
        let a642 = v0;
        let a643 = 91u64;
        nat_lt(a642, a643)
    } else {
        false
    } {
        let a644 = v0;
        let a645 = 32u64;
        nat_add(a644, a645)
    } else {
        Ok(v0)
    }
}

pub fn f186(p648: List<u64>) -> List<u64> {
    let m647 = p648;
    {
        let mut m647 = m647;
        loop {
            let step = {
                let v0 = m647;
                let m646 = v0;
                match m646.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f248(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: u64) -> bool {
    let a649 = v0;
    let a650 = 9u64;
    if nat_eq(a649, a650) {
        true
    } else {
        let a651 = 31u64;
        let a652 = v0;
        if nat_lt(a651, a652) {
            let a653 = v0;
            let a654 = 127u64;
            nat_lt(a653, a654)
        } else {
            false
        }
    }
}

pub fn f188(p657: List<u64>) -> bool {
    let m656 = p657;
    {
        let mut m656 = m656;
        loop {
            let step = {
                let v0 = m656;
                let m655 = v0;
                match m655.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f249(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(p660: List<u64>, v1: bool) -> bool {
    let m659 = p660;
    {
        let mut m659 = m659;
        loop {
            let step = {
                let v0 = m659;
                let m658 = v0;
                match m658.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f250(v2)
                        } else {
                            f251(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m659 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190() -> u64 {
    64u64
}

pub fn f191(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m661 = f231(Str::lit("type"), v1);
    match m661 {
        Adt135::C0 => {
            Some(Adt98::C3(v0))
        }
        Adt135::C1(v2) => {
            let m662 = f228(v2);
            match m662 {
                None => {
                    Some(Adt98::C3(v0))
                }
                Some(v3) => {
                    let a663 = v3;
                    let a664 = Str::lit("object");
                    if equal(a663, a664) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f192(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt98> {
    let m665 = f231(Str::lit("$schema"), v1);
    match m665 {
        Adt135::C0 => {
            None::<Adt98>
        }
        Adt135::C1(v2) => {
            let m666 = f228(v2);
            match m666 {
                None => {
                    Some(Adt98::C2(v0))
                }
                Some(v3) => {
                    let a667 = v3;
                    let a668 = f252();
                    if equal(a667, a668) {
                        None::<Adt98>
                    } else {
                        Some(Adt98::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f193(v0: Adt36) -> R<Option<Adt98>> {
    let m670 = match v0.clone() {
        Adt36::C0(_, _, _, _, _, h669, _, _) => {
            h669
        }
    };
    match m670 {
        None => {
            Ok(None::<Adt98>)
        }
        Some(v1) => {
            let m673 = f192(match v0.clone() {
                Adt36::C0(h671, _, _, _, _, _, _, _) => {
                    h671
                }
            }, match v1.clone() {
                Adt5::C0(h672) => {
                    h672
                }
            });
            match m673 {
                None => {
                    let m677 = f154(f155(), match v0 {
                        Adt36::C0(h674, _, _, _, _, _, _, _) => {
                            h674
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h675) => {
                            h675
                        }
                    }), List::cons(Adt106::C0(Adt6::C5(match v1 {
                        Adt5::C0(h676) => {
                            h676
                        }
                    }), false, List::<Str>::nil()), List::<Adt106>::nil()), f156())?;
                    match m677 {
                        Ok(_) => {
                            Ok(None::<Adt98>)
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

pub fn f194(v0: Str, v1: Adt6, v2: Adt106, v3: Adt121, v4: List<Adt106>) -> R<Result<(List<Adt106>, Adt121), Adt98>> {
    let m678 = v2;
    match m678 {
        Adt106::C0(v5, v6, v7) => {
            let m679 = f253(v5.clone());
            match m679 {
                None => {
                    let m680 = f254(v5);
                    match m680 {
                        None => {
                            Ok(Err::<(List<Adt106>, Adt121), Adt98>(Adt98::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f255(v0, v1, match v9 {
                                Adt5::C0(h681) => {
                                    h681
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt106>, Adt121), Adt98>((v4, v3)))
                }
            }
        }
    }
}

pub fn f195(v0: Adt6, p700: u64, p701: Adt124, p702: List<Adt125>) -> R<Adt122> {
    let m699 = (p700, (p701, p702));
    {
        let mut m699 = m699;
        loop {
            let step = {
                let (v1, (v2, v3)) = m699;
                let m682 = v1;
                if m682 == 0 {
                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C4))
                } else {
                    let v4 = m682 - 1;
                    let m683 = v3;
                    match m683.uncons() {
                        None => {
                            let m684 = v2;
                            match m684 {
                                Adt124::C0 => {
                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C0))
                                }
                                Adt124::C1(v5, v6) => {
                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m685 = v2.clone();
                            match m685 {
                                Adt124::C0 => {
                                    let m686 = v7.clone();
                                    match m686 {
                                        Adt125::C0(v9, v10, v11) => {
                                            let m687 = f256(v0.clone(), v9, v10, v11.clone())?;
                                            match m687 {
                                                Adt142::C0(v12) => {
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (Adt124::C1(v11, v12), v8)))
                                                }
                                                Adt142::C1(v13) => {
                                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C2(v13)))
                                                }
                                                Adt142::C2(v14) => {
                                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C3(v14)))
                                                }
                                                Adt142::C3(v15) => {
                                                    let v16: List<Adt125> = {
                                                        let a688 = v15;
                                                        let a689 = v8;
                                                        append_list(a688, a689)
                                                    };
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt125::C1(_, _, _) => {
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v8)))
                                        }
                                        Adt125::C2(_, _, _, _) => {
                                            let v24: Adt137 = f257(v7, v8)?;
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v24.clone() {
                                                Adt137::C0(h690, _) => {
                                                    h690
                                                }
                                            }, match v24 {
                                                Adt137::C0(_, h691) => {
                                                    h691
                                                }
                                            })))
                                        }
                                        Adt125::C3(v25) => {
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (Adt124::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt125::C4(_, _, _, _) => {
                                            let v39: Adt137 = f257(v7, v8)?;
                                            Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v39.clone() {
                                                Adt137::C0(h695, _) => {
                                                    h695
                                                }
                                            }, match v39 {
                                                Adt137::C0(_, h696) => {
                                                    h696
                                                }
                                            })))
                                        }
                                        Adt125::C5(v26, v27, v28, v29) => {
                                            let m692 = f258(v26, v27, v28, v29.clone());
                                            match m692 {
                                                Adt142::C0(v30) => {
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (Adt124::C1(v29, v30), v8)))
                                                }
                                                Adt142::C1(v31) => {
                                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C2(v31)))
                                                }
                                                Adt142::C2(v32) => {
                                                    Err::<(u64, (Adt124, List<Adt125>)), R<Adt122>>(Ok(Adt122::C3(v32)))
                                                }
                                                Adt142::C3(v33) => {
                                                    let v34: List<Adt125> = {
                                                        let a693 = v33;
                                                        let a694 = v8;
                                                        append_list(a693, a694)
                                                    };
                                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt124::C1(v40, v41) => {
                                    let v42: Adt137 = f259(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt124, List<Adt125>)), R<Adt122>>((v4, (match v42.clone() {
                                        Adt137::C0(h697, _) => {
                                            h697
                                        }
                                    }, match v42 {
                                        Adt137::C0(_, h698) => {
                                            h698
                                        }
                                    })))
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

pub fn f196() -> u64 {
    67108864u64
}

pub fn f197(v0: List<Adt123>) -> R<Str> {
    let a703 = List::cons(Str::lit(""), f260(v0, List::<Str>::nil())?);
    let a704 = Str::lit("/");
    Ok(join(a703, a704))
}

pub fn f198(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f261({
        let m707 = match v1.clone() {
            Adt50::C0(h706, _) => {
                h706
            }
        };
        match m707 {
            Adt38::C0(h705, _, _) => {
                h705
            }
        }
    })? {
        f262(v0, v1)
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f199(v0: Bytes, v1: u64) -> R<Option<i64>> {
    let a711 = {
        let a708 = v1;
        let a709 = 8u64;
        nat_add(a708, a709)?
    };
    let a712 = {
        let a710 = v0.clone();
        length_bytes(a710)
    };
    if nat_le(a711, a712) {
        Ok(Some(f263(f263(f263(f263(f263(f263(f263(f263(0i64, f56(v0.clone(), v1)?)?, f56(v0.clone(), {
            let a713 = v1;
            let a714 = 1u64;
            nat_add(a713, a714)?
        })?)?, f56(v0.clone(), {
            let a715 = v1;
            let a716 = 2u64;
            nat_add(a715, a716)?
        })?)?, f56(v0.clone(), {
            let a717 = v1;
            let a718 = 3u64;
            nat_add(a717, a718)?
        })?)?, f56(v0.clone(), {
            let a719 = v1;
            let a720 = 4u64;
            nat_add(a719, a720)?
        })?)?, f56(v0.clone(), {
            let a721 = v1;
            let a722 = 5u64;
            nat_add(a721, a722)?
        })?)?, f56(v0.clone(), {
            let a723 = v1;
            let a724 = 6u64;
            nat_add(a723, a724)?
        })?)?, f56(v0, {
            let a725 = v1;
            let a726 = 7u64;
            nat_add(a725, a726)?
        })?)?))
    } else {
        Ok(None::<i64>)
    }
}

pub fn f200(v0: Bytes, v1: u64) -> R<Result<(Str, u64), Adt94>> {
    let m727 = f15(v0, v1)?;
    match m727 {
        None => {
            Ok(Err::<(Str, u64), Adt94>(Adt94::C0(v1)))
        }
        Some(v2) => {
            let m729 = f264(f86(match v2.clone() {
                Adt96::C0(h728, _) => {
                    h728
                }
            })?);
            match m729 {
                None => {
                    Ok(Err::<(Str, u64), Adt94>(Adt94::C2(v1)))
                }
                Some(v3) => {
                    Ok(Ok::<(Str, u64), Adt94>((v3, match v2 {
                        Adt96::C0(_, h730) => {
                            h730
                        }
                    })))
                }
            }
        }
    }
}

pub fn f201(v0: Bytes, v1: i64, v2: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m732 = f200(v0.clone(), {
        let (_, h731) = v2;
        h731
    })?;
    match m732 {
        Ok(v4) => {
            f265(v0, v1, {
                let (h733, _) = v2;
                h733
            }, v4)
        }
        Err(v3) => {
            Ok(Err::<(Adt14, u64), Adt94>(v3))
        }
    }
}

pub fn f202(v0: Adt107, v1: List<Adt107>, v2: List<u64>) -> R<Adt138> {
    let m734 = v0;
    match m734 {
        Adt107::C0(v3) => {
            Ok(Adt138::C1(v1, f164(v3, v2)))
        }
        Adt107::C1(v4, v5) => {
            f266(v4, v5, v1, v2)
        }
        Adt107::C2(v6, v7) => {
            let m735 = v7;
            match m735.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C1(v6, v8), List::cons(Adt107::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt107::C3(v10, v11) => {
            let m736 = v11.clone();
            match m736.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt107::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt107::C4(v14, v15) => {
            let m737 = v15;
            match m737.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C0({
                        let a739 = f267({
                            let (h738, _) = v16;
                            h738
                        })?;
                        let a740 = List::cons(58u64, List::<u64>::nil());
                        append_list(a739, a740)
                    }), List::cons(Adt107::C1(v14, {
                        let (_, h741) = v16;
                        h741
                    }), List::cons(Adt107::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt107::C5(v18, v19) => {
            let m742 = v19.clone();
            match m742.uncons() {
                None => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt138::C1(List::cons(Adt107::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt107::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f203(v0: Adt128) -> R<Option<Adt5>> {
    let m744 = f268(match v0.clone() {
        Adt128::C0(h743, _, _) => {
            h743
        }
    })?;
    match m744 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a747 = v1;
                let a748 = {
                    let m746 = f269(v0);
                    match m746 {
                        Adt5::C0(h745) => {
                            h745
                        }
                    }
                };
                append_list(a747, a748)
            })))
        }
    }
}

pub fn f204(v0: Adt14, v1: Option<Adt88>, v2: Adt95) -> R<Adt95> {
    let m749 = v1;
    match m749 {
        None => {
            Ok(v2)
        }
        Some(v3) => {
            let a751 = match v2.clone() {
                Adt95::C0(h750, _, _, _, _) => {
                    h750
                }
            };
            let a752 = f207();
            if nat_eq(a751, a752) {
                Ok(f270(f271(v0, v3)?, v2))
            } else {
                Ok(v2)
            }
        }
    }
}

pub fn f205(v0: Adt33) -> Option<Adt88> {
    let m753 = v0;
    match m753 {
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
                Adt83::C0(_, _, h754) => {
                    h754
                }
            }, Adt89::C0, false))
        }
        Adt33::C14(v14) => {
            Some(v14)
        }
    }
}

pub fn f206(v0: bool, v1: Adt95, v2: Option<List<List<u64>>>) -> Adt95 {
    let m755 = v2;
    match m755 {
        None => {
            f6()
        }
        Some(v3) => {
            f272(v0, v1, f37(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f207() -> u64 {
    200u64
}

pub fn f208(v0: Adt99, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("id"), f165(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h756) => {
            h756
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f209(v0: Option<List<List<u64>>>, v1: Option<List<List<u64>>>) -> Option<List<List<u64>>> {
    let m757 = v0;
    match m757 {
        None => {
            None::<List<List<u64>>>
        }
        Some(v2) => {
            let m758 = v1;
            match m758 {
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

pub fn f210(p762: List<Adt130>, p763: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m761 = (p762, p763);
    {
        let mut m761 = m761;
        loop {
            let step = {
                let (v0, v1) = m761;
                let m759 = v0;
                match m759.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m760 = f273(v2)?;
                        match m760 {
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
                Ok(next) => { m761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Option<Adt88>) -> R<Option<List<List<u64>>>> {
    let m764 = v0;
    match m764 {
        None => {
            Ok(Some(List::<List<u64>>::nil()))
        }
        Some(v1) => {
            f274(v1)
        }
    }
}

pub fn f212(v0: Adt131) -> u64 {
    let m765 = v0;
    match m765 {
        Adt131::C0(_) => {
            f97()
        }
        Adt131::C1(_, _) => {
            f97()
        }
        Adt131::C2(_) => {
            f97()
        }
        Adt131::C3(_) => {
            f275()
        }
        Adt131::C4(v6) => {
            f276(v6)
        }
    }
}

pub fn f213(v0: Adt131) -> R<Adt81> {
    Ok(Adt81::C0(f277(v0.clone()), f278(v0.clone()), f279(v0)?))
}

pub fn f214() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f215(p768: List<List<u64>>) -> R<bool> {
    let m767 = p768;
    {
        let mut m767 = m767;
        loop {
            let step = {
                let v0 = m767;
                let m766 = v0;
                match m766.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f280(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m767 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: u64, p773: List<u64>, p774: List<u64>, p775: List<List<u64>>) -> List<List<u64>> {
    let m772 = (p773, (p774, p775));
    {
        let mut m772 = m772;
        loop {
            let step = {
                let (v1, (v2, v3)) = m772;
                let m769 = v1;
                match m769.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f37(List::cons(f150(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a770 = v4;
                        let a771 = v0;
                        if nat_eq(a770, a771) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f150(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m772 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt139, Adt131>) -> R<Result<Adt128, Adt131>> {
    let m776 = v3;
    match m776 {
        Ok(v5) => {
            f281(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(v4))
        }
    }
}

pub fn f218(v0: Option<Adt108>, v1: List<Str>, v2: Adt105) -> R<Result<Adt139, Adt131>> {
    Ok(f282(f283(match v2.clone() {
        Adt105::C0(_, h777, _) => {
            h777
        }
    }), f284(v0, v1, v2)?))
}

pub fn f219(v0: Adt110, v1: Adt103) -> Adt133 {
    let m778 = v1;
    match m778 {
        Adt103::C0 => {
            f225()
        }
        Adt103::C1 => {
            Adt133::C0(None::<Str>, Some(Adt109::C6(v0)))
        }
        Adt103::C2 => {
            Adt133::C0(None::<Str>, Some(Adt109::C7(v0)))
        }
        Adt103::C3(v2) => {
            let m779 = f264(v2);
            match m779 {
                None => {
                    Adt133::C0(None::<Str>, Some(Adt109::C6(v0)))
                }
                Some(v3) => {
                    Adt133::C0(Some(v3), None::<Adt109>)
                }
            }
        }
    }
}

pub fn f220() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f221() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f222(v0: Str) -> bool {
    let m780 = f285(v0);
    match m780 {
        Adt146::C0 => {
            false
        }
        Adt146::C1 => {
            true
        }
        Adt146::C2 => {
            true
        }
        Adt146::C3 => {
            true
        }
    }
}

pub fn f223(v0: Adt103) -> R<Adt103> {
    let m781 = v0.clone();
    match m781 {
        Adt103::C0 => {
            Ok(v0)
        }
        Adt103::C1 => {
            Ok(v0)
        }
        Adt103::C2 => {
            Ok(v0)
        }
        Adt103::C3(v1) => {
            let m782 = f286(v1);
            match m782 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m783 = f287(v2)?;
                    match m783 {
                        None => {
                            Ok(Adt103::C1)
                        }
                        Some(v3) => {
                            Ok(Adt103::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f224() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f225() -> Adt133 {
    Adt133::C0(None::<Str>, None::<Adt109>)
}

pub fn f226(v0: Option<Adt109>, v1: Option<Adt109>) -> Option<Adt109> {
    let m784 = v0.clone();
    match m784 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f227(v0: List<Adt93>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt111>> {
    Ok(f288(f289(v0, v1, v2, List::<Adt111>::nil())?, List::<Adt111>::nil()))
}

pub fn f228(v0: Adt6) -> Option<Str> {
    let m785 = v0;
    match m785 {
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

pub fn f229() -> Str {
    Str::lit("2026-07-28")
}

pub fn f230(v0: Adt7) -> R<Option<i64>> {
    let m788 = f290(40u64, match v0.clone() {
        Adt7::C0(h786, _) => {
            h786
        }
    }, match v0 {
        Adt7::C0(_, h787) => {
            h787
        }
    })?;
    match m788 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f291(v1))
        }
    }
}

pub fn f231(v0: Str, p794: List<(Str, Adt6)>) -> Adt135 {
    let m793 = p794;
    {
        let mut m793 = m793;
        loop {
            let step = {
                let v1 = m793;
                let m789 = v1;
                match m789.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m791 = f292({
                            let (h790, _) = v2;
                            h790
                        }, v0.clone());
                        match m791 {
                            Adt147::C0 => {
                                Err::<List<(Str, Adt6)>, Adt135>(Adt135::C1({
                                    let (_, h792) = v2;
                                    h792
                                }))
                            }
                            Adt147::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt135>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m793 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Str, v1: Adt5) -> bool {
    f293(f177(v0, v1))
}

pub fn f233(v0: Adt99, v1: Adt5) -> Result<Adt105, Adt101> {
    let m795 = f177(Str::lit("method"), v1.clone());
    match m795 {
        Adt135::C0 => {
            f136(Some(v0), f178(v1))
        }
        Adt135::C1(v2) => {
            let m796 = f228(v2);
            match m796 {
                None => {
                    f136(Some(v0), Adt102::C4)
                }
                Some(v3) => {
                    f294(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f234(v0: u64) -> R<u64> {
    let a799 = {
        let a797 = v0;
        let a798 = 3u64;
        nat_mul(a797, a798)?
    };
    let a800 = 1u64;
    nat_add(a799, a800)
}

pub fn f235(v0: u64, v1: u64, v2: u64) -> bool {
    let a801 = v1;
    let a802 = v0;
    if nat_le(a801, a802) {
        let a803 = v0;
        let a804 = v2;
        nat_le(a803, a804)
    } else {
        false
    }
}

pub fn f236(v0: Adt118) -> R<Adt141> {
    if f295(match v0.clone() {
        Adt118::C0(_, h805, _, _, _, _, _) => {
            h805
        }
    }) {
        let v1: i64 = {
            let a811 = if match v0.clone() {
                Adt118::C0(_, _, _, _, _, h806, _) => {
                    h806
                }
            } {
                let a808 = match v0.clone() {
                    Adt118::C0(_, _, _, _, _, _, h807) => {
                        h807
                    }
                };
                int_neg(a808)?
            } else {
                match v0.clone() {
                    Adt118::C0(_, _, _, _, _, _, h809) => {
                        h809
                    }
                }
            };
            let a812 = match v0.clone() {
                Adt118::C0(_, _, _, _, h810, _, _) => {
                    h810
                }
            };
            int_sub(a811, a812)?
        };
        let m815 = {
            let a813 = v1;
            let a814 = -9999i64;
            compare(a813, a814)
        };
        if if match m815 {
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
            let m818 = {
                let a816 = 9999i64;
                let a817 = v1;
                compare(a816, a817)
            };
            match m818 {
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
                Adt118::C0(h819, _, _, _, _, _, _) => {
                    h819
                }
            } {
                let a821 = match v0 {
                    Adt118::C0(_, _, h820, _, _, _, _) => {
                        h820
                    }
                };
                int_neg(a821)?
            } else {
                match v0 {
                    Adt118::C0(_, _, h822, _, _, _, _) => {
                        h822
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt141::C0)
    }
}

pub fn f237(v0: Adt6, v1: List<Adt120>, v2: u64) -> Adt115 {
    let m823 = v1.clone();
    match m823.uncons() {
        None => {
            Adt115::C0(Adt116::C9(v0), List::<Adt120>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m824 = v3;
            match m824 {
                Adt120::C0(v5) => {
                    Adt115::C0(Adt116::C2, List::cons(Adt120::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt120::C1(v6, v7) => {
                    let m825 = v7;
                    match m825 {
                        None => {
                            Adt115::C0(Adt116::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt115::C0(Adt116::C2, List::cons(Adt120::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f238(v0: u64) -> bool {
    let a826 = v0;
    let a827 = 32u64;
    if nat_eq(a826, a827) {
        true
    } else {
        let a828 = v0;
        let a829 = 10u64;
        if nat_eq(a828, a829) {
            true
        } else {
            let a830 = v0;
            let a831 = 13u64;
            if nat_eq(a830, a831) {
                true
            } else {
                let a832 = v0;
                let a833 = 9u64;
                nat_eq(a832, a833)
            }
        }
    }
}

pub fn f239(v0: Adt115, v1: u64, v2: u64) -> R<Adt136> {
    let a834 = v2;
    let a835 = 34u64;
    if nat_eq(a834, a835) {
        Ok(f243(v0, Adt116::C6(Adt117::C0, List::<u64>::nil(), false)))
    } else {
        let a836 = v2;
        let a837 = 123u64;
        if nat_eq(a836, a837) {
            f296(v0, Adt120::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt116::C3, v1)
        } else {
            let a838 = v2;
            let a839 = 91u64;
            if nat_eq(a838, a839) {
                f296(v0, Adt120::C0(List::<Adt6>::nil()), Adt116::C1, v1)
            } else {
                let a840 = v2;
                let a841 = 116u64;
                if nat_eq(a840, a841) {
                    Ok(f243(v0, Adt116::C7(0u64, 1u64)))
                } else {
                    let a842 = v2;
                    let a843 = 102u64;
                    if nat_eq(a842, a843) {
                        Ok(f243(v0, Adt116::C7(1u64, 1u64)))
                    } else {
                        let a844 = v2;
                        let a845 = 110u64;
                        if nat_eq(a844, a845) {
                            Ok(f243(v0, Adt116::C7(2u64, 1u64)))
                        } else {
                            let a846 = v2;
                            let a847 = 45u64;
                            if nat_eq(a846, a847) {
                                Ok(f243(v0, f297(true, Adt119::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a848 = v2;
                                let a849 = 48u64;
                                if nat_eq(a848, a849) {
                                    Ok(f243(v0, f297(false, Adt119::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f235(v2, 49u64, 57u64) {
                                        Ok(f243(v0, f297(false, Adt119::C2, f298({
                                            let a850 = v2;
                                            let a851 = 48u64;
                                            nat_sub(a850, a851)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f244(Adt100::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f240(v0: Adt115, v1: u64) -> Adt136 {
    let m853 = match v0.clone() {
        Adt115::C0(_, h852, _) => {
            h852
        }
    };
    match m853.uncons() {
        None => {
            f244(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m854 = v2;
            match m854 {
                Adt120::C0(v4) => {
                    Adt136::C1(f237(Adt6::C4(f299(v4, List::<Adt6>::nil())), v3, {
                        let a856 = match v0 {
                            Adt115::C0(_, _, h855) => {
                                h855
                            }
                        };
                        let a857 = 1u64;
                        nat_sub(a856, a857)
                    }))
                }
                Adt120::C1(_, _) => {
                    f244(Adt100::C2(v1))
                }
            }
        }
    }
}

pub fn f241(v0: Adt115, v1: u64, v2: u64) -> Adt136 {
    let m859 = match v0.clone() {
        Adt115::C0(h858, _, _) => {
            h858
        }
    };
    match m859 {
        Adt116::C0 => {
            f244(Adt100::C2(v1))
        }
        Adt116::C1 => {
            f244(Adt100::C2(v1))
        }
        Adt116::C2 => {
            if f238(v2) {
                Adt136::C1(v0)
            } else {
                let a860 = v2;
                let a861 = 44u64;
                if nat_eq(a860, a861) {
                    f300(v0, v1)
                } else {
                    let a862 = v2;
                    let a863 = 93u64;
                    if nat_eq(a862, a863) {
                        f240(v0, v1)
                    } else {
                        let a864 = v2;
                        let a865 = 125u64;
                        if nat_eq(a864, a865) {
                            f242(v0, v1)
                        } else {
                            f244(Adt100::C2(v1))
                        }
                    }
                }
            }
        }
        Adt116::C3 => {
            f244(Adt100::C2(v1))
        }
        Adt116::C4 => {
            f244(Adt100::C2(v1))
        }
        Adt116::C5 => {
            f244(Adt100::C2(v1))
        }
        Adt116::C6(_, _, _) => {
            f244(Adt100::C2(v1))
        }
        Adt116::C7(_, _) => {
            f244(Adt100::C2(v1))
        }
        Adt116::C8(_) => {
            f244(Adt100::C2(v1))
        }
        Adt116::C9(_) => {
            if f238(v2) {
                Adt136::C1(v0)
            } else {
                f244(Adt100::C10(v1))
            }
        }
    }
}

pub fn f242(v0: Adt115, v1: u64) -> Adt136 {
    let m867 = match v0.clone() {
        Adt115::C0(_, h866, _) => {
            h866
        }
    };
    match m867.uncons() {
        None => {
            f244(Adt100::C2(v1))
        }
        Some((v2, v3)) => {
            let m868 = v2;
            match m868 {
                Adt120::C0(_) => {
                    f244(Adt100::C2(v1))
                }
                Adt120::C1(v5, _) => {
                    Adt136::C1(f237(Adt6::C5(f301(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a870 = match v0 {
                            Adt115::C0(_, _, h869) => {
                                h869
                            }
                        };
                        let a871 = 1u64;
                        nat_sub(a870, a871)
                    }))
                }
            }
        }
    }
}

pub fn f243(v0: Adt115, v1: Adt116) -> Adt136 {
    Adt136::C1(f302(v0, v1))
}

pub fn f244(v0: Adt100) -> Adt136 {
    Adt136::C0(v0)
}

pub fn f245(v0: Adt115, v1: u64, v2: u64, v3: Adt117, v4: List<u64>, v5: bool) -> R<Adt136> {
    let m872 = v3;
    match m872 {
        Adt117::C0 => {
            let a873 = v2;
            let a874 = 34u64;
            if nat_eq(a873, a874) {
                Ok(f303(v0, v1, v4, v5))
            } else {
                let a875 = v2;
                let a876 = 92u64;
                if nat_eq(a875, a876) {
                    Ok(f243(v0, Adt116::C6(Adt117::C1, v4, v5)))
                } else {
                    let a877 = v2;
                    let a878 = 32u64;
                    if nat_lt(a877, a878) {
                        Ok(f244(Adt100::C4(v1)))
                    } else {
                        Ok(f243(v0, Adt116::C6(Adt117::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt117::C1 => {
            let a879 = v2;
            let a880 = 117u64;
            if nat_eq(a879, a880) {
                Ok(f243(v0, Adt116::C6(Adt117::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a881 = f304(v2);
                let a882 = 0u64;
                if nat_eq(a881, a882) {
                    Ok(f244(Adt100::C5(v1)))
                } else {
                    Ok(f243(v0, Adt116::C6(Adt117::C0, List::cons(f304(v2), v4), v5)))
                }
            }
        }
        Adt117::C2(v6, v7, v8) => {
            let a883 = 15u64;
            let a884 = f305(v2);
            if nat_lt(a883, a884) {
                Ok(f244(Adt100::C5(v1)))
            } else {
                let a887 = {
                    let a885 = v6;
                    let a886 = 1u64;
                    nat_add(a885, a886)?
                };
                let a888 = 4u64;
                if nat_eq(a887, a888) {
                    f306(v0, v1, {
                        let a891 = {
                            let a889 = v7;
                            let a890 = 16u64;
                            nat_mul(a889, a890)?
                        };
                        let a892 = f305(v2);
                        nat_add(a891, a892)?
                    }, v8, v4, v5)
                } else {
                    Ok(f243(v0, Adt116::C6(Adt117::C2({
                        let a893 = v6;
                        let a894 = 1u64;
                        nat_add(a893, a894)?
                    }, {
                        let a897 = {
                            let a895 = v7;
                            let a896 = 16u64;
                            nat_mul(a895, a896)?
                        };
                        let a898 = f305(v2);
                        nat_add(a897, a898)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt117::C3(v9) => {
            let a899 = v2;
            let a900 = 92u64;
            if nat_eq(a899, a900) {
                Ok(f243(v0, Adt116::C6(Adt117::C4(v9), v4, v5)))
            } else {
                Ok(f244(Adt100::C6(v1)))
            }
        }
        Adt117::C4(v10) => {
            let a901 = v2;
            let a902 = 117u64;
            if nat_eq(a901, a902) {
                Ok(f243(v0, Adt116::C6(Adt117::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f244(Adt100::C6(v1)))
            }
        }
    }
}

pub fn f246(v0: Adt115, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt136> {
    let a903 = v2;
    let a904 = f307(v3, v4);
    if nat_eq(a903, a904) {
        let a908 = {
            let a905 = v4;
            let a906 = 1u64;
            nat_add(a905, a906)?
        };
        let a909 = {
            let a907 = f308(v3);
            length_list(a907)
        };
        if nat_eq(a908, a909) {
            Ok(f309(v0, f310(v3)))
        } else {
            Ok(f243(v0, Adt116::C7(v3, {
                let a910 = v4;
                let a911 = 1u64;
                nat_add(a910, a911)?
            })))
        }
    } else {
        Ok(f244(Adt100::C2(v1)))
    }
}

pub fn f247(v0: Adt115, v1: u64, v2: u64, v3: Adt118) -> R<Adt136> {
    let m913 = match v3.clone() {
        Adt118::C0(_, h912, _, _, _, _, _) => {
            h912
        }
    };
    match m913 {
        Adt119::C0 => {
            let a914 = v2;
            let a915 = 48u64;
            if nat_eq(a914, a915) {
                Ok(f311(v0, v3, Adt119::C1))
            } else {
                if f235(v2, 49u64, 57u64) {
                    f312(v0, v1, v2, v3, false)
                } else {
                    Ok(f244(Adt100::C7(v1)))
                }
            }
        }
        Adt119::C1 => {
            let a916 = v2;
            let a917 = 46u64;
            if nat_eq(a916, a917) {
                Ok(f311(v0, v3, Adt119::C3))
            } else {
                if f313(v2) {
                    Ok(f311(v0, v3, Adt119::C5))
                } else {
                    if f314(v2) {
                        Ok(f244(Adt100::C7(v1)))
                    } else {
                        f315(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt119::C2 => {
            if f314(v2) {
                f312(v0, v1, v2, v3, false)
            } else {
                let a918 = v2;
                let a919 = 46u64;
                if nat_eq(a918, a919) {
                    Ok(f311(v0, v3, Adt119::C3))
                } else {
                    if f313(v2) {
                        Ok(f311(v0, v3, Adt119::C5))
                    } else {
                        f315(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt119::C3 => {
            if f314(v2) {
                f312(v0, v1, v2, v3, true)
            } else {
                Ok(f244(Adt100::C7(v1)))
            }
        }
        Adt119::C4 => {
            if f314(v2) {
                f312(v0, v1, v2, v3, true)
            } else {
                if f313(v2) {
                    Ok(f311(v0, v3, Adt119::C5))
                } else {
                    f315(v0, v1, v2, v3)
                }
            }
        }
        Adt119::C5 => {
            let a920 = v2;
            let a921 = 43u64;
            if nat_eq(a920, a921) {
                Ok(f243(v0, f297(match v3.clone() {
                    Adt118::C0(h922, _, _, _, _, _, _) => {
                        h922
                    }
                }, Adt119::C6, match v3.clone() {
                    Adt118::C0(_, _, h923, _, _, _, _) => {
                        h923
                    }
                }, match v3.clone() {
                    Adt118::C0(_, _, _, h924, _, _, _) => {
                        h924
                    }
                }, match v3 {
                    Adt118::C0(_, _, _, _, h925, _, _) => {
                        h925
                    }
                }, false, 0i64)))
            } else {
                let a926 = v2;
                let a927 = 45u64;
                if nat_eq(a926, a927) {
                    Ok(f243(v0, f297(match v3.clone() {
                        Adt118::C0(h928, _, _, _, _, _, _) => {
                            h928
                        }
                    }, Adt119::C6, match v3.clone() {
                        Adt118::C0(_, _, h929, _, _, _, _) => {
                            h929
                        }
                    }, match v3.clone() {
                        Adt118::C0(_, _, _, h930, _, _, _) => {
                            h930
                        }
                    }, match v3 {
                        Adt118::C0(_, _, _, _, h931, _, _) => {
                            h931
                        }
                    }, true, 0i64)))
                } else {
                    if f314(v2) {
                        f316(v0, v1, v2, Adt118::C0(match v3.clone() {
                            Adt118::C0(h932, _, _, _, _, _, _) => {
                                h932
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, h933, _, _, _, _, _) => {
                                h933
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, _, h934, _, _, _, _) => {
                                h934
                            }
                        }, match v3.clone() {
                            Adt118::C0(_, _, _, h935, _, _, _) => {
                                h935
                            }
                        }, match v3 {
                            Adt118::C0(_, _, _, _, h936, _, _) => {
                                h936
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f244(Adt100::C7(v1)))
                    }
                }
            }
        }
        Adt119::C6 => {
            if f314(v2) {
                f316(v0, v1, v2, v3.clone(), match v3 {
                    Adt118::C0(_, _, _, _, _, h937, _) => {
                        h937
                    }
                })
            } else {
                Ok(f244(Adt100::C7(v1)))
            }
        }
        Adt119::C7 => {
            if f314(v2) {
                f316(v0, v1, v2, v3.clone(), match v3 {
                    Adt118::C0(_, _, _, _, _, h938, _) => {
                        h938
                    }
                })
            } else {
                f315(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f248(v0: u64) -> bool {
    let a939 = v0;
    let a940 = 9u64;
    if nat_eq(a939, a940) {
        true
    } else {
        let a941 = v0;
        let a942 = 32u64;
        nat_eq(a941, a942)
    }
}

pub fn f249(v0: u64) -> bool {
    let a943 = 47u64;
    let a944 = v0;
    if nat_lt(a943, a944) {
        let a945 = v0;
        let a946 = 58u64;
        nat_lt(a945, a946)
    } else {
        false
    }
}

pub fn f250(v0: u64) -> bool {
    let a947 = 48u64;
    let a948 = v0;
    if if nat_le(a947, a948) {
        let a949 = v0;
        let a950 = 57u64;
        nat_le(a949, a950)
    } else {
        false
    } {
        true
    } else {
        let a951 = 65u64;
        let a952 = v0;
        if if nat_le(a951, a952) {
            let a953 = v0;
            let a954 = 90u64;
            nat_le(a953, a954)
        } else {
            false
        } {
            true
        } else {
            let a955 = 97u64;
            let a956 = v0;
            if if nat_le(a955, a956) {
                let a957 = v0;
                let a958 = 122u64;
                nat_le(a957, a958)
            } else {
                false
            } {
                true
            } else {
                let a959 = v0;
                let a960 = 33u64;
                if nat_eq(a959, a960) {
                    true
                } else {
                    let a961 = v0;
                    let a962 = 35u64;
                    if nat_eq(a961, a962) {
                        true
                    } else {
                        let a963 = v0;
                        let a964 = 36u64;
                        if nat_eq(a963, a964) {
                            true
                        } else {
                            let a965 = v0;
                            let a966 = 37u64;
                            if nat_eq(a965, a966) {
                                true
                            } else {
                                let a967 = v0;
                                let a968 = 38u64;
                                if nat_eq(a967, a968) {
                                    true
                                } else {
                                    let a969 = v0;
                                    let a970 = 39u64;
                                    if nat_eq(a969, a970) {
                                        true
                                    } else {
                                        let a971 = v0;
                                        let a972 = 42u64;
                                        if nat_eq(a971, a972) {
                                            true
                                        } else {
                                            let a973 = v0;
                                            let a974 = 43u64;
                                            if nat_eq(a973, a974) {
                                                true
                                            } else {
                                                let a975 = v0;
                                                let a976 = 45u64;
                                                if nat_eq(a975, a976) {
                                                    true
                                                } else {
                                                    let a977 = v0;
                                                    let a978 = 46u64;
                                                    if nat_eq(a977, a978) {
                                                        true
                                                    } else {
                                                        let a979 = v0;
                                                        let a980 = 94u64;
                                                        if nat_eq(a979, a980) {
                                                            true
                                                        } else {
                                                            let a981 = v0;
                                                            let a982 = 95u64;
                                                            if nat_eq(a981, a982) {
                                                                true
                                                            } else {
                                                                let a983 = v0;
                                                                let a984 = 96u64;
                                                                if nat_eq(a983, a984) {
                                                                    true
                                                                } else {
                                                                    let a985 = v0;
                                                                    let a986 = 124u64;
                                                                    if nat_eq(a985, a986) {
                                                                        true
                                                                    } else {
                                                                        let a987 = v0;
                                                                        let a988 = 126u64;
                                                                        nat_eq(a987, a988)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f251(v0: u64) -> bool {
    let a989 = 48u64;
    let a990 = v0;
    if if nat_le(a989, a990) {
        let a991 = v0;
        let a992 = 57u64;
        nat_le(a991, a992)
    } else {
        false
    } {
        true
    } else {
        let a993 = 65u64;
        let a994 = v0;
        if if nat_le(a993, a994) {
            let a995 = v0;
            let a996 = 90u64;
            nat_le(a995, a996)
        } else {
            false
        } {
            true
        } else {
            let a997 = 97u64;
            let a998 = v0;
            if if nat_le(a997, a998) {
                let a999 = v0;
                let a1000 = 122u64;
                nat_le(a999, a1000)
            } else {
                false
            } {
                true
            } else {
                let a1001 = v0;
                let a1002 = 95u64;
                if nat_eq(a1001, a1002) {
                    true
                } else {
                    let a1003 = v0;
                    let a1004 = 45u64;
                    if nat_eq(a1003, a1004) {
                        true
                    } else {
                        let a1005 = v0;
                        let a1006 = 46u64;
                        nat_eq(a1005, a1006)
                    }
                }
            }
        }
    }
}

pub fn f252() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f253(v0: Adt6) -> Option<bool> {
    let m1007 = v0;
    match m1007 {
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

pub fn f254(v0: Adt6) -> Option<Adt5> {
    let m1008 = v0;
    match m1008 {
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

pub fn f255(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt121, v6: List<Adt106>) -> R<Result<(List<Adt106>, Adt121), Adt98>> {
    let m1009 = f317(v0.clone(), v1, v2.clone());
    match m1009 {
        None => {
            let m1010 = f318(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m1010 {
                Adt148::C0(v8) => {
                    Ok(Err::<(List<Adt106>, Adt121), Adt98>(v8))
                }
                Adt148::C1(v9) => {
                    Ok(Ok::<(List<Adt106>, Adt121), Adt98>((f319(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt106>, Adt121), Adt98>(v7))
        }
    }
}

pub fn f256(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    let m1011 = f253(v1.clone());
    match m1011 {
        None => {
            let m1012 = f254(v1);
            match m1012 {
                None => {
                    Ok(Adt142::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f320(v0, match v5 {
                        Adt5::C0(h1013) => {
                            h1013
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt142::C3(List::<Adt125>::nil()))
            } else {
                Ok(Adt142::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f257(v0: Adt125, v1: List<Adt125>) -> R<Adt137> {
    let m1014 = v0;
    match m1014 {
        Adt125::C0(_, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
        Adt125::C1(_, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
        Adt125::C2(v8, v9, v10, v11) => {
            Ok(f321(v8, v9, v10, {
                let a1015 = v11;
                let a1016 = 1u64;
                nat_add(a1015, a1016)?
            }, v1))
        }
        Adt125::C3(v12) => {
            Ok(Adt137::C0(Adt124::C1(v12, Str::lit("not")), v1))
        }
        Adt125::C4(v13, _, v15, v16) => {
            Ok(f322(v13, v15, v16, v1))
        }
        Adt125::C5(_, _, _, _) => {
            Ok(Adt137::C0(Adt124::C0, v1))
        }
    }
}

pub fn f258(v0: Adt126, v1: Adt6, v2: Adt6, v3: List<Adt123>) -> Adt142 {
    let m1017 = f254(v1);
    match m1017 {
        None => {
            Adt142::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m1018 = v0;
            match m1018 {
                Adt126::C0 => {
                    f323(match v4 {
                        Adt5::C0(h1019) => {
                            h1019
                        }
                    }, v2, v3)
                }
                Adt126::C1 => {
                    f324(match v4 {
                        Adt5::C0(h1020) => {
                            h1020
                        }
                    }, v2, v3)
                }
                Adt126::C2 => {
                    f325(match v4 {
                        Adt5::C0(h1021) => {
                            h1021
                        }
                    }, v2, v3)
                }
                Adt126::C3 => {
                    f326(match v4 {
                        Adt5::C0(h1022) => {
                            h1022
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f259(v0: Adt125, v1: List<Adt125>, v2: List<Adt123>, v3: Str) -> Adt137 {
    let m1023 = v0;
    match m1023 {
        Adt125::C0(_, _, _) => {
            Adt137::C0(Adt124::C1(v2, v3), v1)
        }
        Adt125::C1(v7, v8, v9) => {
            f327(v7, v8, v9, v1)
        }
        Adt125::C2(v10, v11, v12, v13) => {
            f321(v10, v11, v12, v13, v1)
        }
        Adt125::C3(_) => {
            Adt137::C0(Adt124::C0, v1)
        }
        Adt125::C4(_, v16, v17, v18) => {
            f322(v16, v17, v18, v1)
        }
        Adt125::C5(_, _, _, _) => {
            Adt137::C0(Adt124::C1(v2, v3), v1)
        }
    }
}

pub fn f260(p1026: List<Adt123>, p1027: List<Str>) -> R<List<Str>> {
    let m1025 = (p1026, p1027);
    {
        let mut m1025 = m1025;
        loop {
            let step = {
                let (v0, v1) = m1025;
                let m1024 = v0;
                match m1024.uncons() {
                    None => {
                        Err::<(List<Adt123>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt123>, List<Str>), R<List<Str>>>((v3, List::cons(f328(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1025 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(p1036: List<Adt39>) -> R<bool> {
    let m1035 = p1036;
    {
        let mut m1035 = m1035;
        loop {
            let step = {
                let v0 = m1035;
                let m1028 = v0;
                match m1028.uncons() {
                    None => {
                        Err::<List<Adt39>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m1029 = v1;
                        match m1029 {
                            Adt39::C0(_) => {
                                Ok::<List<Adt39>, R<bool>>(v2)
                            }
                            Adt39::C1(v4) => {
                                if f329(match v4 {
                                    Adt43::C0(h1030, _, _, _) => {
                                        h1030
                                    }
                                })? {
                                    Ok::<List<Adt39>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt39>, R<bool>>(Ok(false))
                                }
                            }
                            Adt39::C2(v5) => {
                                if f329(match v5 {
                                    Adt44::C0(h1031, _, _, _) => {
                                        h1031
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
                                let m1033 = match v7 {
                                    Adt46::C0(h1032, _, _) => {
                                        h1032
                                    }
                                };
                                match m1033 {
                                    Adt47::C0(_) => {
                                        Ok::<List<Adt39>, R<bool>>(v2)
                                    }
                                    Adt47::C1(v9) => {
                                        if f329(match v9 {
                                            Adt49::C0(_, _, h1034, _) => {
                                                h1034
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
                Ok(next) => { m1035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f262(v0: Adt36, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    if f330(match v1.clone() {
        Adt50::C0(h1037, _) => {
            h1037
        }
    }) {
        f331(v1)
    } else {
        let m1039 = match v0 {
            Adt36::C0(_, _, _, _, _, h1038, _, _) => {
                h1038
            }
        };
        match m1039 {
            None => {
                f331(v1)
            }
            Some(v2) => {
                f332(v2, v1)
            }
        }
    }
}

pub fn f263(v0: i64, v1: u64) -> R<i64> {
    f116({
        let a1040 = v0;
        let a1041 = 256i64;
        int_mul(a1040, a1041)?
    }, f80(v1, 0i64)?)
}

pub fn f264(v0: List<u64>) -> Option<Str> {
    let a1042 = f333(f89(), v0);
    utf8_decode(a1042)
}

pub fn f265(v0: Bytes, v1: i64, v2: Str, v3: (Str, u64)) -> R<Result<(Adt14, u64), Adt94>> {
    let m1044 = f15(v0, {
        let (_, h1043) = v3.clone();
        h1043
    })?;
    match m1044 {
        None => {
            Ok(Err::<(Adt14, u64), Adt94>(Adt94::C0({
                let (_, h1045) = v3;
                h1045
            })))
        }
        Some(v4) => {
            Ok(f334(v1, v2, {
                let (h1046, _) = v3;
                h1046
            }, v4))
        }
    }
}

pub fn f266(v0: u64, v1: Adt6, v2: List<Adt107>, v3: List<u64>) -> R<Adt138> {
    let m1047 = v1;
    match m1047 {
        Adt6::C0 => {
            Ok(Adt138::C1(v2, f164(f335(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt138::C1(v2, f164(if v4 {
                f335(0u64)
            } else {
                f335(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f336(v5.clone())? {
                Ok(Adt138::C1(v2, f164(f337(v5)?, v3)))
            } else {
                Ok(Adt138::C0(Adt100::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt138::C1(v2, f164(f267(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1048 = v0;
            let a1049 = f338();
            if nat_lt(a1048, a1049) {
                Ok(Adt138::C1(f339(91u64, Adt107::C2({
                    let a1050 = v0;
                    let a1051 = 1u64;
                    nat_add(a1050, a1051)?
                }, v7), v2), v3))
            } else {
                Ok(Adt138::C0(Adt100::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1052 = v0;
            let a1053 = f338();
            if nat_lt(a1052, a1053) {
                Ok(Adt138::C1(f339(123u64, Adt107::C4({
                    let a1054 = v0;
                    let a1055 = 1u64;
                    nat_add(a1054, a1055)?
                }, v8), v2), v3))
            } else {
                Ok(Adt138::C0(Adt100::C12))
            }
        }
    }
}

pub fn f267(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1057 = f164(f340(f86({
            let a1056 = v0;
            utf8_encode(a1056)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1058 = List::cons(34u64, List::<u64>::nil());
        append_list(a1057, a1058)
    }))
}

pub fn f268(v0: Adt33) -> R<Option<List<(Str, Adt6)>>> {
    let m1059 = v0;
    match m1059 {
        Adt33::C0(v1) => {
            Ok(Some(f341(v1)?))
        }
        Adt33::C1(v2) => {
            Ok(Some(f342(v2)?))
        }
        Adt33::C2(v3) => {
            f343(f344(v3)?)
        }
        Adt33::C3(v4) => {
            f343(f344(match v4 {
                Adt50::C0(h1060, _) => {
                    h1060
                }
            })?)
        }
        Adt33::C4(v5) => {
            Ok(Some(f345(v5)?))
        }
        Adt33::C5(v6) => {
            Ok(Some(f346(v6)?))
        }
        Adt33::C6(v7) => {
            Ok(Some(f347(v7)?))
        }
        Adt33::C7(v8) => {
            Ok(Some(f348(v8)?))
        }
        Adt33::C8(v9) => {
            f343(f349(v9)?)
        }
        Adt33::C9(v10) => {
            Ok(Some(f350(v10)?))
        }
        Adt33::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt33::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt33::C12(v12) => {
            f343(f351(v12)?)
        }
        Adt33::C13(v13) => {
            f343(f352({
                let m1063 = match v13 {
                    Adt83::C0(_, _, h1062) => {
                        h1062
                    }
                };
                match m1063 {
                    Adt85::C0(h1061, _, _) => {
                        h1061
                    }
                }
            }))
        }
        Adt33::C14(v14) => {
            Ok(f353(v14))
        }
    }
}

pub fn f269(v0: Adt128) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f354(f355(match v0.clone() {
        Adt128::C0(h1064, _, _) => {
            h1064
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f356({
        let m1067 = match v0 {
            Adt128::C0(_, h1066, _) => {
                h1066
            }
        };
        match m1067 {
            Adt129::C0(h1065, _) => {
                h1065
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f270(v0: Option<Option<List<u64>>>, v1: Adt95) -> Adt95 {
    let m1068 = v0;
    match m1068 {
        None => {
            f6()
        }
        Some(v2) => {
            Adt95::C0(match v1.clone() {
                Adt95::C0(h1069, _, _, _, _) => {
                    h1069
                }
            }, match v1.clone() {
                Adt95::C0(_, h1070, _, _, _) => {
                    h1070
                }
            }, match v1.clone() {
                Adt95::C0(_, _, h1071, _, _) => {
                    h1071
                }
            }, match v1 {
                Adt95::C0(_, _, _, h1072, _) => {
                    h1072
                }
            }, v2)
        }
    }
}

pub fn f271(v0: Adt14, v1: Adt88) -> R<Option<Option<List<u64>>>> {
    let m1074 = f357(match v0.clone() {
        Adt14::C0(_, _, _, h1073) => {
            h1073
        }
    })?;
    match m1074 {
        None => {
            Ok(Some(None::<List<u64>>))
        }
        Some(v2) => {
            let m1077 = f358(f359(match v1 {
                Adt88::C0(h1075, _, _) => {
                    h1075
                }
            }, v2, match v0 {
                Adt14::C0(h1076, _, _, _) => {
                    h1076
                }
            })?)?;
            match m1077 {
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

pub fn f272(v0: bool, v1: Adt95, v2: List<List<u64>>) -> Adt95 {
    if f360(v0, match v1.clone() {
        Adt95::C0(h1078, _, _, _, _) => {
            h1078
        }
    }, v2.clone()) {
        f361(match v1.clone() {
            Adt95::C0(h1079, _, _, _, _) => {
                h1079
            }
        }, {
            let a1081 = v2;
            let a1082 = List::cons(match v1 {
                Adt95::C0(_, _, _, h1080, _) => {
                    h1080
                }
            }, List::<List<u64>>::nil());
            append_list(a1081, a1082)
        })
    } else {
        v1
    }
}

pub fn f273(v0: Adt130) -> R<Option<List<u64>>> {
    let m1083 = f362(f363(v0))?;
    match m1083 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1084 = f90(v1)?;
            match m1084 {
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

pub fn f274(v0: Adt88) -> R<Option<List<List<u64>>>> {
    if f364(v0.clone()) {
        let m1088 = f365({
            let m1087 = match v0 {
                Adt88::C0(h1086, _, _) => {
                    h1086
                }
            };
            match m1087 {
                Adt85::C0(h1085, _, _) => {
                    h1085
                }
            }
        });
        match m1088 {
            None => {
                Ok(None::<List<List<u64>>>)
            }
            Some(v1) => {
                let m1089 = f362(v1)?;
                match m1089 {
                    None => {
                        Ok(None::<List<List<u64>>>)
                    }
                    Some(v2) => {
                        let m1090 = f90(v2)?;
                        match m1090 {
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

pub fn f275() -> u64 {
    404u64
}

pub fn f276(v0: Adt90) -> u64 {
    let m1091 = v0;
    match m1091 {
        Adt90::C0(_) => {
            f97()
        }
        Adt90::C1(_) => {
            f97()
        }
        Adt90::C2(_) => {
            f97()
        }
        Adt90::C3(_) => {
            f97()
        }
        Adt90::C4(_) => {
            f97()
        }
        Adt90::C5(_) => {
            f97()
        }
        Adt90::C6(_) => {
            f21()
        }
        Adt90::C7(_) => {
            f366()
        }
        Adt90::C8(_) => {
            f97()
        }
        Adt90::C9(_) => {
            f21()
        }
        Adt90::C10 => {
            f97()
        }
        Adt90::C11 => {
            f97()
        }
        Adt90::C12(_) => {
            f21()
        }
        Adt90::C13 => {
            f21()
        }
        Adt90::C14 => {
            f21()
        }
        Adt90::C15(_) => {
            f21()
        }
    }
}

pub fn f277(v0: Adt131) -> i64 {
    let m1092 = v0;
    match m1092 {
        Adt131::C0(_) => {
            f367()
        }
        Adt131::C1(_, _) => {
            f368()
        }
        Adt131::C2(_) => {
            f369()
        }
        Adt131::C3(_) => {
            f370()
        }
        Adt131::C4(v6) => {
            f371(v6)
        }
    }
}

pub fn f278(v0: Adt131) -> Str {
    let m1093 = v0;
    match m1093 {
        Adt131::C0(v1) => {
            f372(v1)
        }
        Adt131::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt131::C2(v4) => {
            f373(v4)
        }
        Adt131::C3(_) => {
            Str::lit("Method not found")
        }
        Adt131::C4(v6) => {
            let m1094 = v6;
            match m1094 {
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
                Adt90::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt90::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt90::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt90::C13 => {
                    Str::lit("Internal error")
                }
                Adt90::C14 => {
                    Str::lit("Internal error")
                }
                Adt90::C15(_) => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f279(v0: Adt131) -> R<Option<Adt6>> {
    let m1095 = v0;
    match m1095 {
        Adt131::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f374(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt131::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt131::C4(v6) => {
            let m1096 = v6;
            match m1096 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f80(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f375(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt90::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt90::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt90::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt90::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt90::C14 => {
                    Ok(None::<Adt6>)
                }
                Adt90::C15(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f280(v0: List<u64>) -> R<bool> {
    let m1097 = f216(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1097.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f376(v2)? {
                Ok(false)
            } else {
                f140(f142(v1), f377())
            }
        }
    }
}

pub fn f281(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139) -> R<Result<Adt128, Adt131>> {
    f378(v0.clone(), v1, v2, v3.clone(), f379(match v0 {
        Adt0::C0(_, _, h1098, _, _, _, _) => {
            h1098
        }
    }, match v3 {
        Adt139::C0(_, h1099, _, _) => {
            h1099
        }
    }))
}

pub fn f282(v0: Adt143, v1: Adt144) -> Result<Adt139, Adt131> {
    let m1100 = v1;
    match m1100 {
        Adt144::C0(v3) => {
            f380(v3, v0)
        }
        Adt144::C1(v2) => {
            Err::<Adt139, Adt131>(v2)
        }
    }
}

pub fn f283(v0: Str) -> Adt143 {
    let a1101 = v0.clone();
    let a1102 = Str::lit("server/discover");
    if equal(a1101, a1102) {
        Adt143::C1(Adt140::C0)
    } else {
        let a1103 = v0.clone();
        let a1104 = Str::lit("tools/list");
        if equal(a1103, a1104) {
            Adt143::C1(Adt140::C1)
        } else {
            let a1105 = v0.clone();
            let a1106 = Str::lit("tools/call");
            if equal(a1105, a1106) {
                Adt143::C1(Adt140::C2)
            } else {
                let a1107 = v0.clone();
                let a1108 = Str::lit("resources/list");
                if equal(a1107, a1108) {
                    Adt143::C1(Adt140::C3)
                } else {
                    let a1109 = v0.clone();
                    let a1110 = Str::lit("resources/read");
                    if equal(a1109, a1110) {
                        Adt143::C1(Adt140::C4)
                    } else {
                        let a1111 = v0.clone();
                        let a1112 = Str::lit("resources/templates/list");
                        if equal(a1111, a1112) {
                            Adt143::C1(Adt140::C5)
                        } else {
                            let a1113 = v0.clone();
                            let a1114 = Str::lit("prompts/list");
                            if equal(a1113, a1114) {
                                Adt143::C1(Adt140::C6)
                            } else {
                                let a1115 = v0.clone();
                                let a1116 = Str::lit("prompts/get");
                                if equal(a1115, a1116) {
                                    Adt143::C1(Adt140::C7)
                                } else {
                                    let a1117 = v0.clone();
                                    let a1118 = Str::lit("completion/complete");
                                    if equal(a1117, a1118) {
                                        Adt143::C1(Adt140::C8)
                                    } else {
                                        let a1119 = v0.clone();
                                        let a1120 = Str::lit("tasks/get");
                                        if equal(a1119, a1120) {
                                            Adt143::C1(Adt140::C9)
                                        } else {
                                            let a1121 = v0.clone();
                                            let a1122 = Str::lit("tasks/update");
                                            if equal(a1121, a1122) {
                                                Adt143::C1(Adt140::C10)
                                            } else {
                                                let a1123 = v0;
                                                let a1124 = Str::lit("tasks/cancel");
                                                if equal(a1123, a1124) {
                                                    Adt143::C1(Adt140::C11)
                                                } else {
                                                    Adt143::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f284(v0: Option<Adt108>, v1: List<Str>, v2: Adt105) -> R<Adt144> {
    f381(v0, v1, v2.clone(), f382(match v2 {
        Adt105::C0(_, _, h1125) => {
            h1125
        }
    }))
}

pub fn f285(v0: Str) -> Adt146 {
    let m1126 = f283(v0);
    match m1126 {
        Adt143::C0 => {
            Adt146::C0
        }
        Adt143::C1(v1) => {
            f383(v1)
        }
    }
}

pub fn f286(v0: List<u64>) -> Option<List<u64>> {
    let m1127 = f384(f385(), v0);
    match m1127 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1128 = f384(f386(), f150(v1, List::<u64>::nil()));
            match m1128 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f150(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f287(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1130 = f387({
        let a1129 = v0.clone();
        length_list(a1129)
    }, v0, List::<u64>::nil())?;
    match m1130 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f164(v1, List::<u64>::nil())))
        }
    }
}

pub fn f288(p1133: List<Adt111>, p1134: List<Adt111>) -> List<Adt111> {
    let m1132 = (p1133, p1134);
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v0, v1) = m1132;
                let m1131 = v0;
                match m1131.uncons() {
                    None => {
                        Err::<(List<Adt111>, List<Adt111>), List<Adt111>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt111>, List<Adt111>), List<Adt111>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f289(v0: List<Adt93>, p1141: List<Adt12>, v2: Option<Str>, p1142: List<Adt111>) -> R<List<Adt111>> {
    let m1140 = (p1141, p1142);
    {
        let mut m1140 = m1140;
        loop {
            let step = {
                let (v1, v3) = m1140;
                let m1135 = v1;
                match m1135.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f388(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>((v5, List::cons(Adt111::C0(match v4.clone() {
                                Adt12::C0(_, h1136, _, _) => {
                                    h1136
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1137, _) => {
                                    h1137
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1138) => {
                                    h1138
                                }
                            }, f389(v0.clone(), match v4 {
                                Adt12::C0(_, h1139, _, _) => {
                                    h1139
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt111>), R<List<Adt111>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1140 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(p1158: u64, p1159: i64, p1160: i64) -> R<Option<i64>> {
    let m1157 = (p1158, (p1159, p1160));
    {
        let mut m1157 = m1157;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1157;
                let m1143 = v0;
                if m1143 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1143 - 1;
                    if f390(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1146 = {
                            let a1144 = v2;
                            let a1145 = 0i64;
                            compare(a1144, a1145)
                        };
                        if match m1146 {
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
                            if f390({
                                let a1147 = v1;
                                let a1148 = 10i64;
                                let a1149 = 0i64;
                                int_rem(a1147, a1148, a1149)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1150 = v1;
                                    let a1151 = 10i64;
                                    let a1152 = 0i64;
                                    int_quot(a1150, a1151, a1152)?
                                }, f116(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1153 = v1;
                                let a1154 = 10i64;
                                int_mul(a1153, a1154)?
                            }, {
                                let a1155 = v2;
                                let a1156 = 1i64;
                                int_sub(a1155, a1156)?
                            })))
                        }
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

pub fn f291(v0: i64) -> Option<i64> {
    let m1163 = {
        let a1161 = 9007199254740991i64;
        let a1162 = v0;
        compare(a1161, a1162)
    };
    if if match m1163 {
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
        let m1166 = {
            let a1164 = v0;
            let a1165 = -9007199254740991i64;
            compare(a1164, a1165)
        };
        match m1166 {
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

pub fn f292(v0: Str, v1: Str) -> Adt147 {
    let a1167 = v0;
    let a1168 = v1;
    if equal(a1167, a1168) {
        Adt147::C0
    } else {
        Adt147::C1
    }
}

pub fn f293(v0: Adt135) -> bool {
    let m1169 = v0;
    match m1169 {
        Adt135::C0 => {
            false
        }
        Adt135::C1(_) => {
            true
        }
    }
}

pub fn f294(v0: Adt99, v1: Str, v2: Adt5) -> Result<Adt105, Adt101> {
    let m1170 = f177(Str::lit("params"), v2);
    match m1170 {
        Adt135::C0 => {
            Ok::<Adt105, Adt101>(Adt105::C0(v0, v1, None::<Adt5>))
        }
        Adt135::C1(v3) => {
            let m1171 = f254(v3);
            match m1171 {
                None => {
                    f136(Some(v0), Adt102::C6)
                }
                Some(v4) => {
                    Ok::<Adt105, Adt101>(Adt105::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f295(v0: Adt119) -> bool {
    let m1172 = v0;
    match m1172 {
        Adt119::C0 => {
            false
        }
        Adt119::C1 => {
            true
        }
        Adt119::C2 => {
            true
        }
        Adt119::C3 => {
            false
        }
        Adt119::C4 => {
            true
        }
        Adt119::C5 => {
            false
        }
        Adt119::C6 => {
            false
        }
        Adt119::C7 => {
            true
        }
    }
}

pub fn f296(v0: Adt115, v1: Adt120, v2: Adt116, v3: u64) -> R<Adt136> {
    let a1174 = match v0.clone() {
        Adt115::C0(_, _, h1173) => {
            h1173
        }
    };
    let a1175 = f338();
    if nat_lt(a1174, a1175) {
        Ok(Adt136::C1(Adt115::C0(v2, List::cons(v1, match v0.clone() {
            Adt115::C0(_, h1176, _) => {
                h1176
            }
        }), {
            let a1178 = match v0 {
                Adt115::C0(_, _, h1177) => {
                    h1177
                }
            };
            let a1179 = 1u64;
            nat_add(a1178, a1179)?
        })))
    } else {
        Ok(f244(Adt100::C9(v3)))
    }
}

pub fn f297(v0: bool, v1: Adt119, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt116 {
    Adt116::C8(Adt118::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f298(v0: u64) -> i64 {
    let a1180 = v0;
    let a1181 = 0u64;
    if nat_eq(a1180, a1181) {
        0i64
    } else {
        let a1182 = v0;
        let a1183 = 1u64;
        if nat_eq(a1182, a1183) {
            1i64
        } else {
            let a1184 = v0;
            let a1185 = 2u64;
            if nat_eq(a1184, a1185) {
                2i64
            } else {
                let a1186 = v0;
                let a1187 = 3u64;
                if nat_eq(a1186, a1187) {
                    3i64
                } else {
                    let a1188 = v0;
                    let a1189 = 4u64;
                    if nat_eq(a1188, a1189) {
                        4i64
                    } else {
                        let a1190 = v0;
                        let a1191 = 5u64;
                        if nat_eq(a1190, a1191) {
                            5i64
                        } else {
                            let a1192 = v0;
                            let a1193 = 6u64;
                            if nat_eq(a1192, a1193) {
                                6i64
                            } else {
                                let a1194 = v0;
                                let a1195 = 7u64;
                                if nat_eq(a1194, a1195) {
                                    7i64
                                } else {
                                    let a1196 = v0;
                                    let a1197 = 8u64;
                                    if nat_eq(a1196, a1197) {
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

pub fn f299(p1200: List<Adt6>, p1201: List<Adt6>) -> List<Adt6> {
    let m1199 = (p1200, p1201);
    {
        let mut m1199 = m1199;
        loop {
            let step = {
                let (v0, v1) = m1199;
                let m1198 = v0;
                match m1198.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f300(v0: Adt115, v1: u64) -> Adt136 {
    let m1203 = match v0.clone() {
        Adt115::C0(_, h1202, _) => {
            h1202
        }
    };
    match m1203.uncons() {
        None => {
            f244(Adt100::C2(v1))
        }
        Some((v2, _)) => {
            let m1204 = v2;
            match m1204 {
                Adt120::C0(_) => {
                    f243(v0, Adt116::C0)
                }
                Adt120::C1(_, _) => {
                    f243(v0, Adt116::C4)
                }
            }
        }
    }
}

pub fn f301(p1207: List<(Str, Adt6)>, p1208: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1206 = (p1207, p1208);
    {
        let mut m1206 = m1206;
        loop {
            let step = {
                let (v0, v1) = m1206;
                let m1205 = v0;
                match m1205.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1206 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f302(v0: Adt115, v1: Adt116) -> Adt115 {
    Adt115::C0(v1, match v0.clone() {
        Adt115::C0(_, h1209, _) => {
            h1209
        }
    }, match v0 {
        Adt115::C0(_, _, h1210) => {
            h1210
        }
    })
}

pub fn f303(v0: Adt115, v1: u64, v2: List<u64>, v3: bool) -> Adt136 {
    let m1212 = {
        let a1211 = f333(f89(), f164(v2, List::<u64>::nil()));
        utf8_decode(a1211)
    };
    match m1212 {
        None => {
            f244(Adt100::C14)
        }
        Some(v4) => {
            if v3 {
                f391(v0, v1, v4)
            } else {
                f309(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f304(v0: u64) -> u64 {
    let a1213 = v0;
    let a1214 = 34u64;
    if nat_eq(a1213, a1214) {
        34u64
    } else {
        let a1215 = v0;
        let a1216 = 92u64;
        if nat_eq(a1215, a1216) {
            92u64
        } else {
            let a1217 = v0;
            let a1218 = 47u64;
            if nat_eq(a1217, a1218) {
                47u64
            } else {
                let a1219 = v0;
                let a1220 = 98u64;
                if nat_eq(a1219, a1220) {
                    8u64
                } else {
                    let a1221 = v0;
                    let a1222 = 102u64;
                    if nat_eq(a1221, a1222) {
                        12u64
                    } else {
                        let a1223 = v0;
                        let a1224 = 110u64;
                        if nat_eq(a1223, a1224) {
                            10u64
                        } else {
                            let a1225 = v0;
                            let a1226 = 114u64;
                            if nat_eq(a1225, a1226) {
                                13u64
                            } else {
                                let a1227 = v0;
                                let a1228 = 116u64;
                                if nat_eq(a1227, a1228) {
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

pub fn f305(v0: u64) -> u64 {
    let a1229 = v0;
    let a1230 = 48u64;
    if nat_lt(a1229, a1230) {
        16u64
    } else {
        let a1231 = v0;
        let a1232 = 58u64;
        if nat_lt(a1231, a1232) {
            let a1233 = v0;
            let a1234 = 48u64;
            nat_sub(a1233, a1234)
        } else {
            let a1235 = v0;
            let a1236 = 65u64;
            if nat_lt(a1235, a1236) {
                16u64
            } else {
                let a1237 = v0;
                let a1238 = 71u64;
                if nat_lt(a1237, a1238) {
                    let a1239 = v0;
                    let a1240 = 55u64;
                    nat_sub(a1239, a1240)
                } else {
                    let a1241 = v0;
                    let a1242 = 97u64;
                    if nat_lt(a1241, a1242) {
                        16u64
                    } else {
                        let a1243 = v0;
                        let a1244 = 103u64;
                        if nat_lt(a1243, a1244) {
                            let a1245 = v0;
                            let a1246 = 87u64;
                            nat_sub(a1245, a1246)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f306(v0: Adt115, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt136> {
    let a1247 = v3;
    let a1248 = 0u64;
    if nat_eq(a1247, a1248) {
        if f235(v2, 55296u64, 56319u64) {
            Ok(f243(v0, Adt116::C6(Adt117::C3(v2), v4, v5)))
        } else {
            if f235(v2, 56320u64, 57343u64) {
                Ok(f244(Adt100::C6(v1)))
            } else {
                Ok(f243(v0, Adt116::C6(Adt117::C0, f392(v2, v4)?, v5)))
            }
        }
    } else {
        if f235(v2, 56320u64, 57343u64) {
            Ok(f243(v0, Adt116::C6(Adt117::C0, f392({
                let a1257 = 65536u64;
                let a1258 = {
                    let a1255 = {
                        let a1251 = {
                            let a1249 = v3;
                            let a1250 = 55296u64;
                            nat_sub(a1249, a1250)
                        };
                        let a1252 = 1024u64;
                        nat_mul(a1251, a1252)?
                    };
                    let a1256 = {
                        let a1253 = v2;
                        let a1254 = 56320u64;
                        nat_sub(a1253, a1254)
                    };
                    nat_add(a1255, a1256)?
                };
                nat_add(a1257, a1258)?
            }, v4)?, v5)))
        } else {
            Ok(f244(Adt100::C6(v1)))
        }
    }
}

pub fn f307(v0: u64, v1: u64) -> u64 {
    let m1261 = {
        let a1259 = f308(v0);
        let a1260 = v1;
        index_list(a1259, a1260)
    };
    match m1261 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f308(v0: u64) -> List<u64> {
    let a1262 = v0;
    let a1263 = 0u64;
    if nat_eq(a1262, a1263) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1264 = v0;
        let a1265 = 1u64;
        if nat_eq(a1264, a1265) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f309(v0: Adt115, v1: Adt6) -> Adt136 {
    Adt136::C1(f237(v1, match v0.clone() {
        Adt115::C0(_, h1266, _) => {
            h1266
        }
    }, match v0 {
        Adt115::C0(_, _, h1267) => {
            h1267
        }
    }))
}

pub fn f310(v0: u64) -> Adt6 {
    let a1268 = v0;
    let a1269 = 0u64;
    if nat_eq(a1268, a1269) {
        Adt6::C1(true)
    } else {
        let a1270 = v0;
        let a1271 = 1u64;
        if nat_eq(a1270, a1271) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f311(v0: Adt115, v1: Adt118, v2: Adt119) -> Adt136 {
    f243(v0, Adt116::C8(Adt118::C0(match v1.clone() {
        Adt118::C0(h1272, _, _, _, _, _, _) => {
            h1272
        }
    }, v2, match v1.clone() {
        Adt118::C0(_, _, h1273, _, _, _, _) => {
            h1273
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, h1274, _, _, _) => {
            h1274
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, _, h1275, _, _) => {
            h1275
        }
    }, match v1.clone() {
        Adt118::C0(_, _, _, _, _, h1276, _) => {
            h1276
        }
    }, match v1 {
        Adt118::C0(_, _, _, _, _, _, h1277) => {
            h1277
        }
    })))
}

pub fn f312(v0: Adt115, v1: u64, v2: u64, v3: Adt118, v4: bool) -> R<Adt136> {
    let v5: u64 = {
        let a1279 = match v3.clone() {
            Adt118::C0(_, _, _, h1278, _, _, _) => {
                h1278
            }
        };
        let a1280 = 0u64;
        if if nat_eq(a1279, a1280) {
            let a1281 = v2;
            let a1282 = 48u64;
            nat_eq(a1281, a1282)
        } else {
            false
        } {
            0u64
        } else {
            let a1284 = match v3.clone() {
                Adt118::C0(_, _, _, h1283, _, _, _) => {
                    h1283
                }
            };
            let a1285 = 1u64;
            nat_add(a1284, a1285)?
        }
    };
    let v6: i64 = if v4 {
        f116(match v3.clone() {
            Adt118::C0(_, _, _, _, h1286, _, _) => {
                h1286
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt118::C0(_, _, _, _, h1287, _, _) => {
                h1287
            }
        }
    };
    let a1288 = f393();
    let a1289 = v5;
    if nat_lt(a1288, a1289) {
        Ok(f244(Adt100::C8(v1)))
    } else {
        let m1292 = {
            let a1290 = 99999i64;
            let a1291 = v6;
            compare(a1290, a1291)
        };
        if match m1292 {
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
            Ok(f244(Adt100::C8(v1)))
        } else {
            Ok(f243(v0, f297(match v3.clone() {
                Adt118::C0(h1293, _, _, _, _, _, _) => {
                    h1293
                }
            }, if v4 {
                Adt119::C4
            } else {
                Adt119::C2
            }, f116({
                let a1295 = match v3.clone() {
                    Adt118::C0(_, _, h1294, _, _, _, _) => {
                        h1294
                    }
                };
                let a1296 = 10i64;
                int_mul(a1295, a1296)?
            }, f298({
                let a1297 = v2;
                let a1298 = 48u64;
                nat_sub(a1297, a1298)
            }))?, v5, v6, match v3.clone() {
                Adt118::C0(_, _, _, _, _, h1299, _) => {
                    h1299
                }
            }, match v3 {
                Adt118::C0(_, _, _, _, _, _, h1300) => {
                    h1300
                }
            })))
        }
    }
}

pub fn f313(v0: u64) -> bool {
    let a1301 = v0;
    let a1302 = 101u64;
    if nat_eq(a1301, a1302) {
        true
    } else {
        let a1303 = v0;
        let a1304 = 69u64;
        nat_eq(a1303, a1304)
    }
}

pub fn f314(v0: u64) -> bool {
    f235(v0, 48u64, 57u64)
}

pub fn f315(v0: Adt115, v1: u64, v2: u64, v3: Adt118) -> R<Adt136> {
    let m1305 = f236(v3)?;
    match m1305 {
        Adt141::C0 => {
            Ok(f244(Adt100::C7(v1)))
        }
        Adt141::C1 => {
            Ok(f244(Adt100::C8(v1)))
        }
        Adt141::C2(v4) => {
            Ok(f241(f237(v4, match v0.clone() {
                Adt115::C0(_, h1306, _) => {
                    h1306
                }
            }, match v0 {
                Adt115::C0(_, _, h1307) => {
                    h1307
                }
            }), v1, v2))
        }
    }
}

pub fn f316(v0: Adt115, v1: u64, v2: u64, v3: Adt118, v4: bool) -> R<Adt136> {
    let v5: i64 = f116({
        let a1309 = match v3.clone() {
            Adt118::C0(_, _, _, _, _, _, h1308) => {
                h1308
            }
        };
        let a1310 = 10i64;
        int_mul(a1309, a1310)?
    }, f298({
        let a1311 = v2;
        let a1312 = 48u64;
        nat_sub(a1311, a1312)
    }))?;
    let m1315 = {
        let a1313 = 99999i64;
        let a1314 = v5;
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
        Ok(f244(Adt100::C8(v1)))
    } else {
        Ok(f243(v0, f297(match v3.clone() {
            Adt118::C0(h1316, _, _, _, _, _, _) => {
                h1316
            }
        }, Adt119::C7, match v3.clone() {
            Adt118::C0(_, _, h1317, _, _, _, _) => {
                h1317
            }
        }, match v3.clone() {
            Adt118::C0(_, _, _, h1318, _, _, _) => {
                h1318
            }
        }, match v3 {
            Adt118::C0(_, _, _, _, h1319, _, _) => {
                h1319
            }
        }, v4, v5)))
    }
}

pub fn f317(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1320 = f394(f395(), v2.clone());
    match m1320 {
        None => {
            let m1321 = f396(f397(), v2.clone());
            match m1321 {
                None => {
                    f398(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt98::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt98::C4(v0, v3))
        }
    }
}

pub fn f318(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt121) -> R<Adt148> {
    let m1322 = f231(Str::lit("x-mcp-header"), v1.clone());
    match m1322 {
        Adt135::C0 => {
            Ok(Adt148::C1(v4))
        }
        Adt135::C1(v5) => {
            f399(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f319(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt106>) -> List<Adt106> {
    f400(Str::lit("properties"), v0.clone(), v1, v2, f400(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f401(Str::lit("items"), v0.clone(), f401(Str::lit("additionalProperties"), v0.clone(), f402(Str::lit("allOf"), v0.clone(), f402(Str::lit("anyOf"), v0.clone(), f402(Str::lit("oneOf"), v0.clone(), f401(Str::lit("not"), v0.clone(), f401(Str::lit("if"), v0.clone(), f401(Str::lit("then"), v0.clone(), f401(Str::lit("else"), v0, v3)))))))))))
}

pub fn f320(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    let m1323 = f403(v1.clone(), v2.clone())?;
    match m1323 {
        Adt142::C0(v4) => {
            Ok(Adt142::C0(v4))
        }
        Adt142::C1(v5) => {
            Ok(Adt142::C1(v5))
        }
        Adt142::C2(v6) => {
            Ok(Adt142::C2(v6))
        }
        Adt142::C3(_) => {
            f404(v0, v1, v2, v3)
        }
    }
}

pub fn f321(v0: List<Adt6>, v1: Adt6, v2: List<Adt123>, v3: u64, v4: List<Adt125>) -> Adt137 {
    let m1324 = v0;
    match m1324.uncons() {
        None => {
            let a1325 = v3;
            let a1326 = 1u64;
            if nat_eq(a1325, a1326) {
                Adt137::C0(Adt124::C0, v4)
            } else {
                Adt137::C0(Adt124::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt137::C0(Adt124::C0, List::cons(Adt125::C0(v5, v1.clone(), v2.clone()), List::cons(Adt125::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f322(v0: Option<Adt6>, v1: Adt6, v2: List<Adt123>, v3: List<Adt125>) -> Adt137 {
    let m1327 = v0;
    match m1327 {
        None => {
            Adt137::C0(Adt124::C0, v3)
        }
        Some(v4) => {
            Adt137::C0(Adt124::C0, List::cons(Adt125::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f323(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1328 = f231(Str::lit("anyOf"), v0);
    match m1328 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1329 = f405(v3);
            match m1329 {
                None => {
                    Adt142::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m1330 = v4;
                    match m1330.uncons() {
                        None => {
                            Adt142::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt142::C3(List::cons(Adt125::C0(v5, v1.clone(), v2.clone()), List::cons(Adt125::C1(v6, v1, v2), List::<Adt125>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f324(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1331 = f231(Str::lit("oneOf"), v0);
    match m1331 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1332 = f405(v3);
            match m1332 {
                None => {
                    Adt142::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m1333 = v4;
                    match m1333.uncons() {
                        None => {
                            Adt142::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt142::C3(List::cons(Adt125::C0(v5, v1.clone(), v2.clone()), List::cons(Adt125::C2(v6, v1, v2, 0u64), List::<Adt125>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f325(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1334 = f231(Str::lit("not"), v0);
    match m1334 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            Adt142::C3(List::cons(Adt125::C0(v3, v1, v2.clone()), List::cons(Adt125::C3(v2), List::<Adt125>::nil())))
        }
    }
}

pub fn f326(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1335 = f231(Str::lit("if"), v0.clone());
    match m1335 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            Adt142::C3(List::cons(Adt125::C0(v3, v1.clone(), v2.clone()), List::cons(Adt125::C4(f406(Str::lit("then"), v0.clone()), f406(Str::lit("else"), v0), v1, v2), List::<Adt125>::nil())))
        }
    }
}

pub fn f327(v0: List<Adt6>, v1: Adt6, v2: List<Adt123>, v3: List<Adt125>) -> Adt137 {
    let m1336 = v0;
    match m1336.uncons() {
        None => {
            Adt137::C0(Adt124::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt137::C0(Adt124::C0, List::cons(Adt125::C0(v4, v1.clone(), v2.clone()), List::cons(Adt125::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f328(v0: Adt123) -> R<Str> {
    let m1337 = v0;
    match m1337 {
        Adt123::C0(v1) => {
            f407(v1)
        }
        Adt123::C1(v2) => {
            let a1338 = f80(v2, 0i64)?;
            Ok(format_i64(a1338))
        }
    }
}

pub fn f329(v0: Str) -> R<bool> {
    let m1340 = f287(f86({
        let a1339 = v0;
        utf8_encode(a1339)
    })?)?;
    match m1340 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f330(v0: Adt38) -> bool {
    let m1342 = match v0 {
        Adt38::C0(_, _, h1341) => {
            h1341
        }
    };
    match m1342 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f331(v0: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1344 = match v0.clone() {
        Adt50::C0(_, h1343) => {
            h1343
        }
    };
    match m1344.uncons() {
        None => {
            Ok(Ok::<Adt33, Adt90>(Adt33::C2(match v0 {
                Adt50::C0(h1345, _) => {
                    h1345
                }
            })))
        }
        Some((_, _)) => {
            if f408(match v0.clone() {
                Adt50::C0(_, h1346) => {
                    h1346
                }
            })? {
                Ok(Ok::<Adt33, Adt90>(Adt33::C3(v0)))
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f332(v0: Adt5, v1: Adt50) -> R<Result<Adt33, Adt90>> {
    let m1350 = {
        let m1349 = match v1.clone() {
            Adt50::C0(h1348, _) => {
                h1348
            }
        };
        match m1349 {
            Adt38::C0(_, h1347, _) => {
                h1347
            }
        }
    };
    match m1350 {
        None => {
            Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1353 = f157(Adt6::C5(match v0.clone() {
                Adt5::C0(h1351) => {
                    h1351
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1352) => {
                    h1352
                }
            }), v2)?;
            match m1353 {
                Adt122::C0 => {
                    f331(v1)
                }
                Adt122::C1(_, _) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt122::C2(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt122::C3(_) => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt122::C4 => {
                    Ok(Err::<Adt33, Adt90>(Adt90::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f333(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1354 = v0;
    let a1355 = f409(f89(), f410(List::<Bytes>::nil(), v1));
    append_bytes(a1354, a1355)
}

pub fn f334(v0: i64, v1: Str, v2: Str, v3: Adt96) -> Result<(Adt14, u64), Adt94> {
    Ok::<(Adt14, u64), Adt94>((Adt14::C0(v0, v1, v2, match v3.clone() {
        Adt96::C0(h1356, _) => {
            h1356
        }
    }), match v3 {
        Adt96::C0(_, h1357) => {
            h1357
        }
    }))
}

pub fn f335(v0: u64) -> List<u64> {
    f308(v0)
}

pub fn f336(v0: Adt7) -> R<bool> {
    let a1375 = {
        let m1361 = {
            let a1359 = f411();
            let a1360 = match v0.clone() {
                Adt7::C0(h1358, _) => {
                    h1358
                }
            };
            compare(a1359, a1360)
        };
        if match m1361 {
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
            let m1366 = {
                let a1364 = match v0.clone() {
                    Adt7::C0(h1362, _) => {
                        h1362
                    }
                };
                let a1365 = {
                    let a1363 = f411();
                    int_neg(a1363)?
                };
                compare(a1364, a1365)
            };
            if match m1366 {
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
                let m1370 = {
                    let a1368 = 9999i64;
                    let a1369 = match v0.clone() {
                        Adt7::C0(_, h1367) => {
                            h1367
                        }
                    };
                    compare(a1368, a1369)
                };
                if match m1370 {
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
                    let m1374 = {
                        let a1372 = match v0 {
                            Adt7::C0(_, h1371) => {
                                h1371
                            }
                        };
                        let a1373 = -9999i64;
                        compare(a1372, a1373)
                    };
                    match m1374 {
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
    Ok(bool_not(a1375))
}

pub fn f337(v0: Adt7) -> R<List<u64>> {
    let m1379 = {
        let a1377 = match v0.clone() {
            Adt7::C0(_, h1376) => {
                h1376
            }
        };
        let a1378 = 0i64;
        compare(a1377, a1378)
    };
    if if match m1379 {
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
            let a1381 = 0i64;
            let a1382 = match v0.clone() {
                Adt7::C0(_, h1380) => {
                    h1380
                }
            };
            compare(a1381, a1382)
        };
        match m1383 {
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
        let a1386 = f412(match v0.clone() {
            Adt7::C0(h1384, _) => {
                h1384
            }
        })?;
        let a1387 = List::cons(101u64, f412(match v0 {
            Adt7::C0(_, h1385) => {
                h1385
            }
        })?);
        Ok(append_list(a1386, a1387))
    } else {
        f412(match v0 {
            Adt7::C0(h1388, _) => {
                h1388
            }
        })
    }
}

pub fn f338() -> u64 {
    512u64
}

pub fn f339(v0: u64, v1: Adt107, v2: List<Adt107>) -> List<Adt107> {
    List::cons(Adt107::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f340(p1391: List<u64>, p1392: List<u64>) -> R<List<u64>> {
    let m1390 = (p1391, p1392);
    {
        let mut m1390 = m1390;
        loop {
            let step = {
                let (v0, v1) = m1390;
                let m1389 = v0;
                match m1389.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f164(f413(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1390 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f414(match v0.clone() {
        Adt34::C0(_, h1393, _, _) => {
            h1393
        }
    })), List::cons((Str::lit("capabilities"), f415(match v0.clone() {
        Adt34::C0(_, _, h1394, _) => {
            h1394
        }
    })), {
        let a1397 = f416(Str::lit("instructions"), match v0.clone() {
            Adt34::C0(_, _, _, h1395) => {
                h1395
            }
        });
        let a1398 = f417(match v0 {
            Adt34::C0(h1396, _, _, _) => {
                h1396
            }
        })?;
        append_list(a1397, a1398)
    })))
}

pub fn f342(v0: Adt35) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f418(match v0.clone() {
        Adt35::C0(_, _, h1399) => {
            h1399
        }
    }))), {
        let a1402 = f416(Str::lit("nextCursor"), match v0.clone() {
            Adt35::C0(_, h1400, _) => {
                h1400
            }
        });
        let a1403 = f417(match v0 {
            Adt35::C0(h1401, _, _) => {
                h1401
            }
        })?;
        append_list(a1402, a1403)
    }))
}

pub fn f343(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1404 = v0;
    match m1404 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1405 = f362(Adt6::C5(v1))?;
            match m1405 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1406 = f254(v2);
                    match m1406 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1407) => {
                                    h1407
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f344(v0: Adt38) -> R<Option<List<(Str, Adt6)>>> {
    let m1409 = f419(match v0.clone() {
        Adt38::C0(h1408, _, _) => {
            h1408
        }
    })?;
    match m1409 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1412 = f420(Str::lit("structuredContent"), match v0.clone() {
                    Adt38::C0(_, h1410, _) => {
                        h1410
                    }
                });
                let a1413 = f421(Str::lit("isError"), match v0 {
                    Adt38::C0(_, _, h1411) => {
                        h1411
                    }
                });
                append_list(a1412, a1413)
            })))
        }
    }
}

pub fn f345(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f422(match v0.clone() {
        Adt52::C0(_, _, h1414) => {
            h1414
        }
    })?)), {
        let a1417 = f416(Str::lit("nextCursor"), match v0.clone() {
            Adt52::C0(_, h1415, _) => {
                h1415
            }
        });
        let a1418 = f417(match v0 {
            Adt52::C0(h1416, _, _) => {
                h1416
            }
        })?;
        append_list(a1417, a1418)
    }))
}

pub fn f346(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f423(match v0.clone() {
        Adt53::C0(_, h1419) => {
            h1419
        }
    }))), f417(match v0 {
        Adt53::C0(h1420, _) => {
            h1420
        }
    })?))
}

pub fn f347(v0: Adt54) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f424(match v0.clone() {
        Adt54::C0(_, _, h1421) => {
            h1421
        }
    }))), {
        let a1424 = f416(Str::lit("nextCursor"), match v0.clone() {
            Adt54::C0(_, h1422, _) => {
                h1422
            }
        });
        let a1425 = f417(match v0 {
            Adt54::C0(h1423, _, _) => {
                h1423
            }
        })?;
        append_list(a1424, a1425)
    }))
}

pub fn f348(v0: Adt56) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f425(match v0.clone() {
        Adt56::C0(_, _, h1426) => {
            h1426
        }
    }))), {
        let a1429 = f416(Str::lit("nextCursor"), match v0.clone() {
            Adt56::C0(_, h1427, _) => {
                h1427
            }
        });
        let a1430 = f417(match v0 {
            Adt56::C0(h1428, _, _) => {
                h1428
            }
        })?;
        append_list(a1429, a1430)
    }))
}

pub fn f349(v0: Adt59) -> R<Option<List<(Str, Adt6)>>> {
    let m1432 = f426(match v0.clone() {
        Adt59::C0(_, h1431) => {
            h1431
        }
    })?;
    match m1432 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1434 = f416(Str::lit("description"), match v0 {
                    Adt59::C0(h1433, _) => {
                        h1433
                    }
                });
                let a1435 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1434, a1435)
            }))
        }
    }
}

pub fn f350(v0: Adt61) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f374({
        let m1438 = match v0.clone() {
            Adt61::C0(h1437) => {
                h1437
            }
        };
        match m1438 {
            Adt62::C0(h1436, _, _) => {
                h1436
            }
        }
    }))), {
        let a1445 = f427(Str::lit("total"), {
            let m1441 = match v0.clone() {
                Adt61::C0(h1440) => {
                    h1440
                }
            };
            match m1441 {
                Adt62::C0(_, h1439, _) => {
                    h1439
                }
            }
        })?;
        let a1446 = f421(Str::lit("hasMore"), {
            let m1444 = match v0 {
                Adt61::C0(h1443) => {
                    h1443
                }
            };
            match m1444 {
                Adt62::C0(_, _, h1442) => {
                    h1442
                }
            }
        });
        append_list(a1445, a1446)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f351(v0: Adt82) -> R<Option<List<(Str, Adt6)>>> {
    let m1447 = f428(f429(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1447 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1448 = f430(v1);
                let a1449 = f431(f432(v0));
                append_list(a1448, a1449)
            }))
        }
    }
}

pub fn f352(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1450 = f433(v0.clone());
    match m1450 {
        None => {
            None::<List<(Str, Adt6)>>
        }
        Some(v1) => {
            Some({
                let a1451 = f434(f435(v0.clone()), f436(f437(v0)));
                let a1452 = v1;
                append_list(a1451, a1452)
            })
        }
    }
}

pub fn f353(v0: Adt88) -> Option<List<(Str, Adt6)>> {
    let m1454 = match v0.clone() {
        Adt88::C0(_, h1453, _) => {
            h1453
        }
    };
    match m1454 {
        Adt89::C0 => {
            f352({
                let m1457 = match v0 {
                    Adt88::C0(h1456, _, _) => {
                        h1456
                    }
                };
                match m1457 {
                    Adt85::C0(h1455, _, _) => {
                        h1455
                    }
                }
            })
        }
        Adt89::C1 => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f354(v0: Adt149) -> Str {
    let m1458 = v0;
    match m1458 {
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

pub fn f355(v0: Adt33) -> Adt149 {
    let m1459 = v0;
    match m1459 {
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

pub fn f356(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1460 = v0;
    match m1460 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f438(), f439(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f357(v0: Bytes) -> R<Option<List<Adt85>>> {
    let a1462 = {
        let a1461 = v0.clone();
        length_bytes(a1461)
    };
    let a1463 = 0u64;
    if nat_eq(a1462, a1463) {
        Ok(Some(List::<Adt85>::nil()))
    } else {
        let m1464 = f69(v0)?;
        match m1464 {
            Ok(v2) => {
                let m1465 = f405(v2);
                match m1465 {
                    None => {
                        Ok(None::<List<Adt85>>)
                    }
                    Some(v3) => {
                        Ok(f440(v3, List::<Adt85>::nil()))
                    }
                }
            }
            Err(_) => {
                Ok(None::<List<Adt85>>)
            }
        }
    }
}

pub fn f358(v0: List<Adt85>) -> R<Option<List<u64>>> {
    let m1466 = f441(v0);
    match m1466 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1467 = f90(v1)?;
            match m1467 {
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

pub fn f359(v0: Adt85, v1: List<Adt85>, v2: i64) -> R<List<Adt85>> {
    Ok(List::cons(v0.clone(), f442(f443(f444(v0), v1, v2, List::<Adt85>::nil())?, List::<Adt85>::nil())))
}

pub fn f360(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f445(v2) {
            let a1468 = v1;
            let a1469 = 200u64;
            nat_eq(a1468, a1469)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f361(v0: u64, v1: List<List<u64>>) -> Adt95 {
    Adt95::C0(v0, f446(), List::<List<u64>>::nil(), f447(v1), None::<List<u64>>)
}

pub fn f362(v0: Adt6) -> R<Option<Adt6>> {
    f448(f449(), Adt150::C0(v0), List::<Adt151>::nil())
}

pub fn f363(v0: Adt130) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f127())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f450(match v0.clone() {
        Adt130::C0(h1470, _) => {
            h1470
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1473 = match v0.clone() {
            Adt130::C0(_, h1472) => {
                h1472
            }
        };
        match m1473 {
            Adt51::C0(h1471, _, _) => {
                h1471
            }
        }
    })), {
        let a1480 = f451({
            let m1476 = match v0.clone() {
                Adt130::C0(_, h1475) => {
                    h1475
                }
            };
            match m1476 {
                Adt51::C0(_, h1474, _) => {
                    h1474
                }
            }
        });
        let a1481 = f452({
            let m1479 = match v0 {
                Adt130::C0(_, h1478) => {
                    h1478
                }
            };
            match m1479 {
                Adt51::C0(_, _, h1477) => {
                    h1477
                }
            }
        });
        append_list(a1480, a1481)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f364(v0: Adt88) -> bool {
    if match v0.clone() {
        Adt88::C0(_, _, h1482) => {
            h1482
        }
    } {
        let m1484 = match v0 {
            Adt88::C0(_, h1483, _) => {
                h1483
            }
        };
        match m1484 {
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

pub fn f365(v0: Adt63) -> Option<Adt6> {
    let m1485 = f352(v0);
    match m1485 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(Str::lit("2.0"))), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/tasks"))), List::cons((Str::lit("params"), Adt6::C5(v1)), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f366() -> u64 {
    429u64
}

pub fn f367() -> i64 {
    -32602i64
}

pub fn f368() -> i64 {
    -32022i64
}

pub fn f369() -> i64 {
    -32020i64
}

pub fn f370() -> i64 {
    -32601i64
}

pub fn f371(v0: Adt90) -> i64 {
    let m1486 = v0;
    match m1486 {
        Adt90::C0(_) => {
            f367()
        }
        Adt90::C1(_) => {
            f367()
        }
        Adt90::C2(_) => {
            f367()
        }
        Adt90::C3(_) => {
            f367()
        }
        Adt90::C4(_) => {
            f367()
        }
        Adt90::C5(_) => {
            f367()
        }
        Adt90::C6(_) => {
            f453()
        }
        Adt90::C7(_) => {
            f454()
        }
        Adt90::C8(_) => {
            f455()
        }
        Adt90::C9(_) => {
            f453()
        }
        Adt90::C10 => {
            f367()
        }
        Adt90::C11 => {
            f367()
        }
        Adt90::C12(_) => {
            f453()
        }
        Adt90::C13 => {
            f453()
        }
        Adt90::C14 => {
            f453()
        }
        Adt90::C15(_) => {
            f453()
        }
    }
}

pub fn f372(v0: Adt132) -> Str {
    let m1487 = v0;
    match m1487 {
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

pub fn f373(v0: Adt109) -> Str {
    let m1488 = v0;
    match m1488 {
        Adt109::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt109::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt109::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt109::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt109::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt109::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt109::C6(v1) => {
            let m1489 = v1;
            match m1489 {
                Adt110::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt110::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt110::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt109::C7(v2) => {
            let m1490 = v2;
            match m1490 {
                Adt110::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt110::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt110::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt109::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt109::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt109::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f374(v0: List<Str>) -> List<Adt6> {
    f299(f456(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f375(v0: Adt18) -> Adt6 {
    Adt6::C5({
        let a1502 = f457(Str::lit("experimental"), match v0.clone() {
            Adt18::C0(h1491, _, _, _, _) => {
                h1491
            }
        });
        let a1503 = {
            let a1500 = f457(Str::lit("roots"), match v0.clone() {
                Adt18::C0(_, h1492, _, _, _) => {
                    h1492
                }
            });
            let a1501 = {
                let a1498 = f457(Str::lit("sampling"), match v0.clone() {
                    Adt18::C0(_, _, h1493, _, _) => {
                        h1493
                    }
                });
                let a1499 = {
                    let a1496 = f457(Str::lit("elicitation"), match v0.clone() {
                        Adt18::C0(_, _, _, h1494, _) => {
                            h1494
                        }
                    });
                    let a1497 = f457(Str::lit("extensions"), match v0 {
                        Adt18::C0(_, _, _, _, h1495) => {
                            h1495
                        }
                    });
                    append_list(a1496, a1497)
                };
                append_list(a1498, a1499)
            };
            append_list(a1500, a1501)
        };
        append_list(a1502, a1503)
    })
}

pub fn f376(p1506: List<List<u64>>) -> R<bool> {
    let m1505 = p1506;
    {
        let mut m1505 = m1505;
        loop {
            let step = {
                let v0 = m1505;
                let m1504 = v0;
                match m1504.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f458(f142(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f377() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f378(v0: Adt0, v1: Fn2, v2: u64, v3: Adt139, v4: bool) -> R<Result<Adt128, Adt131>> {
    if v4 {
        f459(v0, v1, v2, f460(match v3.clone() {
            Adt139::C0(_, h1507, _, _) => {
                h1507
            }
        }, match v3.clone() {
            Adt139::C0(_, _, h1508, _) => {
                h1508
            }
        }, match v3 {
            Adt139::C0(_, _, _, h1509) => {
                h1509
            }
        }))
    } else {
        Ok(Err::<Adt128, Adt131>(Adt131::C3(f461(match v3 {
            Adt139::C0(_, h1510, _, _) => {
                h1510
            }
        }))))
    }
}

pub fn f379(v0: Adt4, v1: Adt140) -> bool {
    let m1511 = v1;
    match m1511 {
        Adt140::C0 => {
            true
        }
        Adt140::C1 => {
            f462(match v0 {
                Adt4::C0(_, _, _, _, _, h1512, _) => {
                    h1512
                }
            })
        }
        Adt140::C2 => {
            f462(match v0 {
                Adt4::C0(_, _, _, _, _, h1513, _) => {
                    h1513
                }
            })
        }
        Adt140::C3 => {
            f463(match v0 {
                Adt4::C0(_, _, _, _, h1514, _, _) => {
                    h1514
                }
            })
        }
        Adt140::C4 => {
            f463(match v0 {
                Adt4::C0(_, _, _, _, h1515, _, _) => {
                    h1515
                }
            })
        }
        Adt140::C5 => {
            f463(match v0 {
                Adt4::C0(_, _, _, _, h1516, _, _) => {
                    h1516
                }
            })
        }
        Adt140::C6 => {
            f462(match v0 {
                Adt4::C0(_, _, _, h1517, _, _, _) => {
                    h1517
                }
            })
        }
        Adt140::C7 => {
            f462(match v0 {
                Adt4::C0(_, _, _, h1518, _, _, _) => {
                    h1518
                }
            })
        }
        Adt140::C8 => {
            f464(match v0 {
                Adt4::C0(_, _, h1519, _, _, _, _) => {
                    h1519
                }
            })
        }
        Adt140::C9 => {
            f465(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1520) => {
                    h1520
                }
            }, f26())
        }
        Adt140::C10 => {
            f465(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1521) => {
                    h1521
                }
            }, f26())
        }
        Adt140::C11 => {
            f465(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1522) => {
                    h1522
                }
            }, f26())
        }
    }
}

pub fn f380(v0: Adt145, v1: Adt143) -> Result<Adt139, Adt131> {
    let m1523 = v1;
    match m1523 {
        Adt143::C0 => {
            Err::<Adt139, Adt131>(Adt131::C3(match v0 {
                Adt145::C0(_, h1524, _, _) => {
                    h1524
                }
            }))
        }
        Adt143::C1(v2) => {
            Ok::<Adt139, Adt131>(Adt139::C0(match v0.clone() {
                Adt145::C0(h1525, _, _, _) => {
                    h1525
                }
            }, v2, match v0.clone() {
                Adt145::C0(_, _, h1526, _) => {
                    h1526
                }
            }, match v0 {
                Adt145::C0(_, _, _, h1527) => {
                    h1527
                }
            }))
        }
    }
}

pub fn f381(v0: Option<Adt108>, v1: List<Str>, v2: Adt105, v3: Result<Adt17, Adt132>) -> R<Adt144> {
    let m1528 = v3;
    match m1528 {
        Ok(v5) => {
            Ok(f466(v1, v2.clone(), v5.clone(), f467(v0, f285(match v2.clone() {
                Adt105::C0(_, h1529, _) => {
                    h1529
                }
            }), match v2.clone() {
                Adt105::C0(_, h1530, _) => {
                    h1530
                }
            }, match v5 {
                Adt17::C0(h1531, _, _, _, _, _) => {
                    h1531
                }
            }, f468(match v2 {
                Adt105::C0(_, _, h1532) => {
                    h1532
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt144::C1(Adt131::C0(v4)))
        }
    }
}

pub fn f382(v0: Option<Adt5>) -> Result<Adt17, Adt132> {
    let m1533 = v0;
    match m1533 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C0)
        }
        Some(v1) => {
            let m1534 = f177(Str::lit("_meta"), v1);
            match m1534 {
                Adt135::C0 => {
                    Err::<Adt17, Adt132>(Adt132::C1)
                }
                Adt135::C1(v2) => {
                    let m1535 = f254(v2);
                    match m1535 {
                        None => {
                            Err::<Adt17, Adt132>(Adt132::C4)
                        }
                        Some(v3) => {
                            f469(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f383(v0: Adt140) -> Adt146 {
    let m1536 = v0;
    match m1536 {
        Adt140::C0 => {
            Adt146::C0
        }
        Adt140::C1 => {
            Adt146::C0
        }
        Adt140::C2 => {
            Adt146::C1
        }
        Adt140::C3 => {
            Adt146::C0
        }
        Adt140::C4 => {
            Adt146::C2
        }
        Adt140::C5 => {
            Adt146::C0
        }
        Adt140::C6 => {
            Adt146::C0
        }
        Adt140::C7 => {
            Adt146::C1
        }
        Adt140::C8 => {
            Adt146::C0
        }
        Adt140::C9 => {
            Adt146::C3
        }
        Adt140::C10 => {
            Adt146::C3
        }
        Adt140::C11 => {
            Adt146::C3
        }
    }
}

pub fn f384(p1542: List<u64>, p1543: List<u64>) -> Option<List<u64>> {
    let m1541 = (p1542, p1543);
    {
        let mut m1541 = m1541;
        loop {
            let step = {
                let (v0, v1) = m1541;
                let m1537 = v0;
                match m1537.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1538 = v1;
                        match m1538.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1539 = v4;
                                let a1540 = v2;
                                if nat_eq(a1539, a1540) {
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
                Ok(next) => { m1541 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f386() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f387(p1552: u64, p1553: List<u64>, p1554: List<u64>) -> R<Option<List<u64>>> {
    let m1551 = (p1552, (p1553, p1554));
    {
        let mut m1551 = m1551;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1551;
                let m1544 = v0;
                if m1544 == 0 {
                    let m1545 = v1;
                    match m1545.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1544 - 1;
                    let m1546 = v1;
                    match m1546.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1547 = v7;
                            match m1547.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1548 = v9;
                                    match m1548.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1549 = v11;
                                            match m1549.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1550 = f470(v6, v8, v10, v12, f471(v13.clone()))?;
                                                    match m1550 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f164(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f388(v0: Adt12, v1: Option<Str>) -> bool {
    let m1555 = v1;
    match m1555 {
        None => {
            false
        }
        Some(v2) => {
            let a1557 = match v0 {
                Adt12::C0(h1556, _, _, _) => {
                    h1556
                }
            };
            let a1558 = v2;
            equal(a1557, a1558)
        }
    }
}

pub fn f389(v0: List<Adt93>, v1: Str) -> R<Adt112> {
    let m1561 = f223(f71(v0, {
        let a1559 = f472();
        let a1560 = f473(v1)?;
        append_list(a1559, a1560)
    })?)?;
    match m1561 {
        Adt103::C0 => {
            Ok(Adt112::C0)
        }
        Adt103::C1 => {
            Ok(Adt112::C1)
        }
        Adt103::C2 => {
            Ok(Adt112::C1)
        }
        Adt103::C3(v2) => {
            let m1562 = f264(v2);
            match m1562 {
                None => {
                    Ok(Adt112::C1)
                }
                Some(v3) => {
                    Ok(Adt112::C2(v3))
                }
            }
        }
    }
}

pub fn f390(v0: i64) -> bool {
    let a1569 = {
        let m1565 = {
            let a1563 = v0;
            let a1564 = 0i64;
            compare(a1563, a1564)
        };
        if match m1565 {
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
            let m1568 = {
                let a1566 = 0i64;
                let a1567 = v0;
                compare(a1566, a1567)
            };
            match m1568 {
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
    bool_not(a1569)
}

pub fn f391(v0: Adt115, v1: u64, v2: Str) -> Adt136 {
    let m1571 = match v0.clone() {
        Adt115::C0(_, h1570, _) => {
            h1570
        }
    };
    match m1571.uncons() {
        None => {
            f244(Adt100::C2(v1))
        }
        Some((v3, v4)) => {
            let m1572 = v3;
            match m1572 {
                Adt120::C0(_) => {
                    f244(Adt100::C2(v1))
                }
                Adt120::C1(v6, _) => {
                    Adt136::C1(Adt115::C0(Adt116::C5, List::cons(Adt120::C1(v6, Some(v2)), v4), match v0 {
                        Adt115::C0(_, _, h1573) => {
                            h1573
                        }
                    }))
                }
            }
        }
    }
}

pub fn f392(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1574 = v0;
    let a1575 = 128u64;
    if nat_lt(a1574, a1575) {
        Ok(List::cons(v0, v1))
    } else {
        let a1576 = v0;
        let a1577 = 2048u64;
        if nat_lt(a1576, a1577) {
            Ok(List::cons({
                let a1578 = 128u64;
                let a1579 = f474(v0, 64u64);
                nat_add(a1578, a1579)?
            }, List::cons({
                let a1580 = 192u64;
                let a1581 = f475(v0, 64u64);
                nat_add(a1580, a1581)?
            }, v1)))
        } else {
            let a1582 = v0;
            let a1583 = 65536u64;
            if nat_lt(a1582, a1583) {
                Ok(List::cons({
                    let a1584 = 128u64;
                    let a1585 = f474(v0, 64u64);
                    nat_add(a1584, a1585)?
                }, List::cons({
                    let a1586 = 128u64;
                    let a1587 = f474(f475(v0, 64u64), 64u64);
                    nat_add(a1586, a1587)?
                }, List::cons({
                    let a1588 = 224u64;
                    let a1589 = f475(v0, 4096u64);
                    nat_add(a1588, a1589)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1590 = 128u64;
                    let a1591 = f474(v0, 64u64);
                    nat_add(a1590, a1591)?
                }, List::cons({
                    let a1592 = 128u64;
                    let a1593 = f474(f475(v0, 64u64), 64u64);
                    nat_add(a1592, a1593)?
                }, List::cons({
                    let a1594 = 128u64;
                    let a1595 = f474(f475(v0, 4096u64), 64u64);
                    nat_add(a1594, a1595)?
                }, List::cons({
                    let a1596 = 240u64;
                    let a1597 = f475(v0, 262144u64);
                    nat_add(a1596, a1597)?
                }, v1)))))
            }
        }
    }
}

pub fn f393() -> u64 {
    18u64
}

pub fn f394(p1601: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1600 = p1601;
    {
        let mut m1600 = m1600;
        loop {
            let step = {
                let v0 = m1600;
                let m1598 = v0;
                match m1598.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1599 = f231(v2.clone(), v1.clone());
                        match m1599 {
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
                Ok(next) => { m1600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f396(p1608: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1607 = p1608;
    {
        let mut m1607 = m1607;
        loop {
            let step = {
                let v0 = m1607;
                let m1602 = v0;
                match m1602.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1604 = f231({
                            let (h1603, _) = v2.clone();
                            h1603
                        }, v1.clone());
                        match m1604 {
                            Adt135::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt135::C1(v4) => {
                                if f476({
                                    let (_, h1605) = v2;
                                    h1605
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1606, _) = v2;
                                        h1606
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1607 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f398(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt98> {
    let m1609 = f231(Str::lit("$ref"), v2);
    match m1609 {
        Adt135::C0 => {
            None::<Adt98>
        }
        Adt135::C1(v3) => {
            let m1610 = f228(v3);
            match m1610 {
                None => {
                    None::<Adt98>
                }
                Some(v4) => {
                    let m1611 = f477(v1, v4.clone());
                    match m1611 {
                        None => {
                            Some(Adt98::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt98>
                        }
                    }
                }
            }
        }
    }
}

pub fn f399(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt121) -> R<Adt148> {
    if v3 {
        let m1612 = f228(v2);
        match m1612 {
            None => {
                Ok(Adt148::C0(Adt98::C7(v0)))
            }
            Some(v6) => {
                f478(v0, v1, v6.clone(), f114(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt148::C0(Adt98::C10(v0)))
    }
}

pub fn f400(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt106>) -> List<Adt106> {
    let m1613 = f231(v0.clone(), v1);
    match m1613 {
        Adt135::C0 => {
            v4
        }
        Adt135::C1(v5) => {
            let m1614 = f254(v5);
            match m1614 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1615 = v0;
                    let a1616 = Str::lit("properties");
                    if equal(a1615, a1616) {
                        f479(match v6 {
                            Adt5::C0(h1617) => {
                                h1617
                            }
                        }, v2, v3, v4)
                    } else {
                        f480(match v6 {
                            Adt5::C0(h1618) => {
                                h1618
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f401(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1619 = f231(v0, v1);
    match m1619 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            List::cons(Adt106::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f402(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt106>) -> List<Adt106> {
    let m1620 = f231(v0, v1);
    match m1620 {
        Adt135::C0 => {
            v2
        }
        Adt135::C1(v3) => {
            let m1621 = f405(v3);
            match m1621 {
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

pub fn f403(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    Ok(f482(List::cons((Str::lit("type"), f483(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f484(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f485(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f486(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f486(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f486(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f486(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1622 = f487(v0.clone(), v1.clone())?;
        let a1623 = List::cons((Str::lit("required"), f488(v0, v1)), List::<(Str, Adt152)>::nil());
        append_list(a1622, a1623)
    })))))))))
}

pub fn f404(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> R<Adt142> {
    Ok(f489(f490(v0, v1.clone(), v2.clone(), v3.clone()), f489(f491(v1.clone(), v2.clone(), v3.clone()), f489(f492(v1.clone(), v2.clone(), v3.clone()), f489(f493(v1.clone(), v2.clone(), v3.clone())?, f489(f494(v1.clone(), v2.clone(), v3.clone()), f489(f495(Adt126::C0, f323(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f489(f495(Adt126::C1, f324(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f489(f495(Adt126::C2, f325(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f495(Adt126::C3, f326(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f405(v0: Adt6) -> Option<List<Adt6>> {
    let m1624 = v0;
    match m1624 {
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

pub fn f406(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1625 = f231(v0, v1);
    match m1625 {
        Adt135::C0 => {
            None::<Adt6>
        }
        Adt135::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f407(v0: Str) -> R<Str> {
    let m1628 = {
        let a1627 = f333(f89(), f164(f496(f86({
            let a1626 = v0.clone();
            utf8_encode(a1626)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1627)
    };
    match m1628 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f408(v0: List<Adt51>) -> R<bool> {
    let m1629 = v0;
    match m1629.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f497(v2, match v1 {
                Adt51::C0(h1630, _, _) => {
                    h1630
                }
            })
        }
    }
}

pub fn f409(p1635: Bytes, p1636: List<Bytes>) -> Bytes {
    let m1634 = (p1635, p1636);
    {
        let mut m1634 = m1634;
        loop {
            let step = {
                let (v0, v1) = m1634;
                let m1631 = v1;
                match m1631.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1632 = v2;
                            let a1633 = v0;
                            append_bytes(a1632, a1633)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1634 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(p1639: List<Bytes>, p1640: List<u64>) -> List<Bytes> {
    let m1638 = (p1639, p1640);
    {
        let mut m1638 = m1638;
        loop {
            let step = {
                let (v0, v1) = m1638;
                let m1637 = v1;
                match m1637.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f498(f499(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411() -> i64 {
    999999999999999999i64
}

pub fn f412(v0: i64) -> R<List<u64>> {
    let m1643 = {
        let a1641 = v0;
        let a1642 = 0i64;
        compare(a1641, a1642)
    };
    if match m1643 {
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
        Ok(List::cons(45u64, f500(20u64, {
            let a1644 = v0;
            int_neg(a1644)?
        }, List::<u64>::nil())?))
    } else {
        f500(20u64, v0, List::<u64>::nil())
    }
}

pub fn f413(v0: u64) -> R<List<u64>> {
    let a1645 = v0;
    let a1646 = 34u64;
    if nat_eq(a1645, a1646) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1647 = v0;
        let a1648 = 92u64;
        if nat_eq(a1647, a1648) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1649 = v0;
            let a1650 = 8u64;
            if nat_eq(a1649, a1650) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1651 = v0;
                let a1652 = 9u64;
                if nat_eq(a1651, a1652) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1653 = v0;
                    let a1654 = 10u64;
                    if nat_eq(a1653, a1654) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1655 = v0;
                        let a1656 = 12u64;
                        if nat_eq(a1655, a1656) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1657 = v0;
                            let a1658 = 13u64;
                            if nat_eq(a1657, a1658) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1659 = v0;
                                let a1660 = 32u64;
                                if nat_lt(a1659, a1660) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f501(f475(v0, 16u64))?, List::cons(f501(f474(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f414(v0: List<Str>) -> Adt6 {
    Adt6::C4(f374(v0))
}

pub fn f415(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1678 = f457(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1661, _, _, _, _, _, _) => {
                h1661
            }
        });
        let a1679 = {
            let a1676 = f457(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1662, _, _, _, _, _) => {
                    h1662
                }
            });
            let a1677 = {
                let a1674 = f457(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1663, _, _, _, _) => {
                        h1663
                    }
                });
                let a1675 = {
                    let a1672 = f502(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1664, _, _, _) => {
                            h1664
                        }
                    });
                    let a1673 = {
                        let a1670 = f503(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1665, _, _) => {
                                h1665
                            }
                        });
                        let a1671 = {
                            let a1668 = f502(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1666, _) => {
                                    h1666
                                }
                            });
                            let a1669 = f457(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1667) => {
                                    h1667
                                }
                            });
                            append_list(a1668, a1669)
                        };
                        append_list(a1670, a1671)
                    };
                    append_list(a1672, a1673)
                };
                append_list(a1674, a1675)
            };
            append_list(a1676, a1677)
        };
        append_list(a1678, a1679)
    })
}

pub fn f416(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1680 = v1;
    match m1680 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f417(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f504(match v0.clone() {
        Adt10::C0(h1681, _) => {
            h1681
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f505(match v0 {
        Adt10::C0(_, h1682) => {
            h1682
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f418(v0: List<Adt36>) -> List<Adt6> {
    f299(f506(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f419(v0: List<Adt39>) -> R<Option<List<Adt6>>> {
    let m1683 = f507(v0, List::<Adt6>::nil())?;
    match m1683 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f299(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f420(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1684 = v1;
    match m1684 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f421(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1685 = v1;
    match m1685 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f422(v0: List<Adt45>) -> R<List<Adt6>> {
    Ok(f299(f508(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f423(v0: List<Adt47>) -> List<Adt6> {
    f299(f509(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f424(v0: List<Adt55>) -> List<Adt6> {
    f299(f510(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f425(v0: List<Adt57>) -> List<Adt6> {
    f299(f511(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f426(v0: List<Adt60>) -> R<Option<List<Adt6>>> {
    let m1686 = f512(v0, List::<Adt6>::nil())?;
    match m1686 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f299(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f427(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1687 = v1;
    match m1687 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f504(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f428(p1693: List<(Str, Adt65)>, p1694: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m1692 = (p1693, p1694);
    {
        let mut m1692 = m1692;
        loop {
            let step = {
                let (v0, v1) = m1692;
                let m1688 = v0;
                match m1688.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1690 = f513({
                            let (_, h1689) = v2;
                            h1689
                        })?;
                        match m1690 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h1691, _) = v2;
                                    h1691
                                }, v4), v1)))
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

pub fn f429(v0: Adt82) -> List<(Str, Adt65)> {
    let m1695 = v0;
    match m1695 {
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

pub fn f430(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1696 = v0.clone();
    match m1696.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f301(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f431(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1697 = v0;
    match m1697 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f432(v0: Adt82) -> Option<Str> {
    let m1698 = v0;
    match m1698 {
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

pub fn f433(v0: Adt63) -> Option<List<(Str, Adt6)>> {
    let m1699 = v0;
    match m1699 {
        Adt63::C0(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
        Adt63::C1(_, v3) => {
            let m1700 = f514(v3);
            match m1700 {
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
                Adt5::C0(h1701) => {
                    h1701
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C3(_, v8) => {
            Some(List::cons((Str::lit("error"), f129(v8)), List::<(Str, Adt6)>::nil()))
        }
        Adt63::C4(_) => {
            Some(List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f434(v0: Adt64, v1: Str) -> List<(Str, Adt6)> {
    List::cons((Str::lit("taskId"), Adt6::C3(match v0.clone() {
        Adt64::C0(h1702, _, _, _, _, _) => {
            h1702
        }
    })), List::cons((Str::lit("status"), Adt6::C3(v1)), {
        let a1708 = f515(Str::lit("statusMessage"), match v0.clone() {
            Adt64::C0(_, h1703, _, _, _, _) => {
                h1703
            }
        });
        let a1709 = List::cons((Str::lit("createdAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, h1704, _, _, _) => {
                h1704
            }
        })), List::cons((Str::lit("lastUpdatedAt"), Adt6::C3(match v0.clone() {
            Adt64::C0(_, _, _, h1705, _, _) => {
                h1705
            }
        })), List::cons((Str::lit("ttlMs"), f516(match v0.clone() {
            Adt64::C0(_, _, _, _, h1706, _) => {
                h1706
            }
        })), f517(Str::lit("pollIntervalMs"), match v0 {
            Adt64::C0(_, _, _, _, _, h1707) => {
                h1707
            }
        }))));
        append_list(a1708, a1709)
    }))
}

pub fn f435(v0: Adt63) -> Adt64 {
    let m1710 = v0;
    match m1710 {
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

pub fn f436(v0: Adt84) -> Str {
    let m1711 = v0;
    match m1711 {
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

pub fn f437(v0: Adt63) -> Adt84 {
    let m1712 = v0;
    match m1712 {
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

pub fn f438() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f439(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1713, _, _, _, _, _) => {
            h1713
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1714, _, _, _) => {
            h1714
        }
    })), {
        let a1723 = f416(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1715, _, _, _, _) => {
                h1715
            }
        });
        let a1724 = {
            let a1721 = f416(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1716, _, _) => {
                    h1716
                }
            });
            let a1722 = {
                let a1719 = f416(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1717, _) => {
                        h1717
                    }
                });
                let a1720 = f518(match v0 {
                    Adt1::C0(_, _, _, _, _, h1718) => {
                        h1718
                    }
                });
                append_list(a1719, a1720)
            };
            append_list(a1721, a1722)
        };
        append_list(a1723, a1724)
    })))
}

pub fn f440(p1728: List<Adt6>, p1729: List<Adt85>) -> Option<List<Adt85>> {
    let m1727 = (p1728, p1729);
    {
        let mut m1727 = m1727;
        loop {
            let step = {
                let (v0, v1) = m1727;
                let m1725 = v0;
                match m1725.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt85>), Option<List<Adt85>>>(Some(f442(v1, List::<Adt85>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1726 = f519(v2);
                        match m1726 {
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
                Ok(next) => { m1727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: List<Adt85>) -> Option<Adt6> {
    let m1730 = f520(v0, List::<Adt6>::nil());
    match m1730 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C4(f299(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f442(p1733: List<Adt85>, p1734: List<Adt85>) -> List<Adt85> {
    let m1732 = (p1733, p1734);
    {
        let mut m1732 = m1732;
        loop {
            let step = {
                let (v0, v1) = m1732;
                let m1731 = v0;
                match m1731.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), List<Adt85>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt85>, List<Adt85>), List<Adt85>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1732 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f443(v0: Str, p1739: List<Adt85>, v2: i64, p1740: List<Adt85>) -> R<List<Adt85>> {
    let m1738 = (p1739, p1740);
    {
        let mut m1738 = m1738;
        loop {
            let step = {
                let (v1, v3) = m1738;
                let m1735 = v1;
                match m1735.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        let a1736 = f444(v4.clone());
                        let a1737 = v0.clone();
                        if if equal(a1736, a1737) {
                            true
                        } else {
                            f521(v4.clone(), v2)?
                        } {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, v3))
                        } else {
                            Ok::<(List<Adt85>, List<Adt85>), R<List<Adt85>>>((v5, List::cons(v4, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1738 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: Adt85) -> Str {
    let m1743 = f435(match v0 {
        Adt85::C0(h1742, _, _) => {
            h1742
        }
    });
    match m1743 {
        Adt64::C0(h1741, _, _, _, _, _) => {
            h1741
        }
    }
}

pub fn f445(v0: List<List<u64>>) -> bool {
    let m1744 = v0;
    match m1744.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f446() -> List<Adt93> {
    List::cons(Adt93::C0(f522(), f377()), List::cons(Adt93::C0(f523(), f524()), List::cons(Adt93::C0(f525(), f526()), List::<Adt93>::nil())))
}

pub fn f447(v0: List<List<u64>>) -> List<u64> {
    f527(f37(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f448(p1759: u64, p1760: Adt150, p1761: List<Adt151>) -> R<Option<Adt6>> {
    let m1758 = (p1759, (p1760, p1761));
    {
        let mut m1758 = m1758;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1758;
                let m1745 = v0;
                if m1745 == 0 {
                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1745 - 1;
                    let m1746 = v1;
                    match m1746 {
                        Adt150::C0(v4) => {
                            let m1747 = v4.clone();
                            match m1747 {
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
                                    let v8: Str = f528(v7)?;
                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1748 = v9;
                                    match m1748.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v10), List::cons(Adt151::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1749 = v12;
                                    match m1749.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f528({
                                                let (h1750, _) = v13;
                                                h1750
                                            })?;
                                            Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                let (_, h1751) = v13;
                                                h1751
                                            }), List::cons(Adt151::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt150::C1(v16) => {
                            let m1752 = v2;
                            match m1752.uncons() {
                                None => {
                                    Err::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1753 = v17;
                                    match m1753 {
                                        Adt151::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1754 = v20;
                                            match m1754.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C4(f299(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0(v22), List::cons(Adt151::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt151::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1755 = v26;
                                            match m1755.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C1(Adt6::C5(f301(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f528({
                                                        let (h1756, _) = v28;
                                                        h1756
                                                    })?;
                                                    Ok::<(u64, (Adt150, List<Adt151>)), R<Option<Adt6>>>((v3, (Adt150::C0({
                                                        let (_, h1757) = v28;
                                                        h1757
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
                Ok(next) => { m1758 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449() -> u64 {
    67108864u64
}

pub fn f450(v0: Adt20) -> Adt6 {
    let m1762 = v0;
    match m1762 {
        Adt20::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt20::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f451(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1763 = v0;
    match m1763 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f452(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1764 = v0;
    match m1764 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f453() -> i64 {
    -32603i64
}

pub fn f454() -> i64 {
    429i64
}

pub fn f455() -> i64 {
    -32021i64
}

pub fn f456(p1767: List<Str>, p1768: List<Adt6>) -> List<Adt6> {
    let m1766 = (p1767, p1768);
    {
        let mut m1766 = m1766;
        loop {
            let step = {
                let (v0, v1) = m1766;
                let m1765 = v0;
                match m1765.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1769 = v1;
    match m1769 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1770) => {
                    h1770
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f458(v0: List<u64>) -> R<bool> {
    let m1771 = v0;
    match m1771.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1772 = v2;
            match m1772.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1773 = f185(v1)?;
                    let a1774 = 113u64;
                    if if nat_eq(a1773, a1774) {
                        let a1775 = v3;
                        let a1776 = 61u64;
                        nat_eq(a1775, a1776)
                    } else {
                        false
                    } {
                        Ok(f529(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f459(v0: Adt0, v1: Fn2, v2: u64, v3: Result<Adt15, Adt132>) -> R<Result<Adt128, Adt131>> {
    let m1777 = v3;
    match m1777 {
        Ok(v5) => {
            f530(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt128, Adt131>(Adt131::C0(v4)))
        }
    }
}

pub fn f460(v0: Adt140, v1: Adt17, v2: Adt5) -> Result<Adt15, Adt132> {
    let m1778 = v0;
    match m1778 {
        Adt140::C0 => {
            f531(v1, v2)
        }
        Adt140::C1 => {
            f532(v1, v2)
        }
        Adt140::C2 => {
            f533(v1, v2)
        }
        Adt140::C3 => {
            f534(v1, v2)
        }
        Adt140::C4 => {
            f535(v1, v2)
        }
        Adt140::C5 => {
            f536(v1, v2)
        }
        Adt140::C6 => {
            f537(v1, v2)
        }
        Adt140::C7 => {
            f538(v1, v2)
        }
        Adt140::C8 => {
            f539(v1, v2)
        }
        Adt140::C9 => {
            f540(v1, v2)
        }
        Adt140::C10 => {
            f541(v1, v2)
        }
        Adt140::C11 => {
            f542(v1, v2)
        }
    }
}

pub fn f461(v0: Adt140) -> Str {
    let m1779 = v0;
    match m1779 {
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

pub fn f462(v0: Option<Adt8>) -> bool {
    let m1780 = v0;
    match m1780 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f463(v0: Option<Adt9>) -> bool {
    let m1781 = v0;
    match m1781 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f464(v0: Option<Adt5>) -> bool {
    let m1782 = v0;
    match m1782 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f465(v0: Option<Adt5>, v1: Str) -> bool {
    let m1783 = v0;
    match m1783 {
        None => {
            false
        }
        Some(v2) => {
            f232(v1, v2)
        }
    }
}

pub fn f466(v0: List<Str>, v1: Adt105, v2: Adt17, v3: Option<Adt109>) -> Adt144 {
    let m1784 = v3;
    match m1784 {
        None => {
            f543(v1, v2.clone(), f544(match v2 {
                Adt17::C0(h1785, _, _, _, _, _) => {
                    h1785
                }
            }, v0))
        }
        Some(v4) => {
            Adt144::C1(Adt131::C2(v4))
        }
    }
}

pub fn f467(v0: Option<Adt108>, v1: Adt146, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt109>> {
    let m1786 = v0;
    match m1786 {
        None => {
            Ok(None::<Adt109>)
        }
        Some(v5) => {
            let m1788 = match v5.clone() {
                Adt108::C0(_, _, _, h1787, _) => {
                    h1787
                }
            };
            match m1788 {
                None => {
                    f545(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f468(v0: Option<Adt5>) -> Adt5 {
    let m1789 = v0;
    match m1789 {
        None => {
            f546()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f469(v0: Adt5) -> Result<Adt17, Adt132> {
    let m1790 = f547(v0.clone());
    match m1790 {
        Adt153::C0 => {
            f548(v0.clone(), f177(f549(), v0.clone()), f177(f550(), v0))
        }
        Adt153::C1 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt153::C2 => {
            Err::<Adt17, Adt132>(Adt132::C3)
        }
    }
}

pub fn f470(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1791 = f551(v0)?;
    match m1791 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1792 = f551(v1)?;
            match m1792 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1793 = v2;
                    let a1794 = 61u64;
                    if nat_eq(a1793, a1794) {
                        f552(v5, v6, v4, v3)
                    } else {
                        let m1795 = f551(v2)?;
                        match m1795 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1796 = v3;
                                let a1797 = 61u64;
                                if nat_eq(a1796, a1797) {
                                    f553(v5, v6, v7, v4)
                                } else {
                                    let m1798 = f551(v3)?;
                                    match m1798 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f554(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f471(v0: List<u64>) -> bool {
    let m1799 = v0;
    match m1799.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f472() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f473(v0: Str) -> R<List<u64>> {
    f86({
        let a1800 = v0;
        utf8_encode(a1800)
    })
}

pub fn f474(v0: u64, v1: u64) -> u64 {
    let a1801 = v0;
    let a1802 = v1;
    let a1803 = 0u64;
    nat_rem(a1801, a1802, a1803)
}

pub fn f475(v0: u64, v1: u64) -> u64 {
    let a1804 = v0;
    let a1805 = v1;
    let a1806 = 0u64;
    nat_quot(a1804, a1805, a1806)
}

pub fn f476(v0: u64, v1: Adt6) -> bool {
    let a1807 = v0;
    let a1808 = 0u64;
    if nat_eq(a1807, a1808) {
        f555(v1)
    } else {
        let a1809 = v0;
        let a1810 = 1u64;
        if nat_eq(a1809, a1810) {
            f556(v1)
        } else {
            let a1811 = v0;
            let a1812 = 2u64;
            if nat_eq(a1811, a1812) {
                f557(v1)
            } else {
                let a1813 = v0;
                let a1814 = 3u64;
                if nat_eq(a1813, a1814) {
                    f558(v1)
                } else {
                    let a1815 = v0;
                    let a1816 = 4u64;
                    if nat_eq(a1815, a1816) {
                        f559(v1)
                    } else {
                        let a1817 = v0;
                        let a1818 = 5u64;
                        if nat_eq(a1817, a1818) {
                            f560(v1)
                        } else {
                            let a1819 = v0;
                            let a1820 = 6u64;
                            if nat_eq(a1819, a1820) {
                                f561(v1)
                            } else {
                                let a1821 = v0;
                                let a1822 = 7u64;
                                if nat_eq(a1821, a1822) {
                                    f562(v1)
                                } else {
                                    f563(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f477(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1823 = v1.clone();
    let a1824 = Str::lit("#");
    if equal(a1823, a1824) {
        Some(v0)
    } else {
        let m1825 = f254(v0);
        match m1825 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1826 = f177(Str::lit("$defs"), v2);
                match m1826 {
                    Adt135::C0 => {
                        None::<Adt6>
                    }
                    Adt135::C1(v3) => {
                        let m1827 = f254(v3);
                        match m1827 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f564(v1, match v4 {
                                    Adt5::C0(h1828) => {
                                        h1828
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

pub fn f478(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt121) -> R<Adt148> {
    let m1829 = v3.clone();
    match m1829.uncons() {
        None => {
            Ok(Adt148::C0(Adt98::C8(v0)))
        }
        Some((_, _)) => {
            if f189(v3.clone(), true) {
                let m1830 = f565(v1);
                match m1830 {
                    None => {
                        Ok(Adt148::C0(Adt98::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f566(v3)?;
                        if f567(v9.clone(), match v5.clone() {
                            Adt121::C0(_, h1831) => {
                                h1831
                            }
                        }) {
                            Ok(Adt148::C0(Adt98::C12(v0, v2)))
                        } else {
                            Ok(Adt148::C1(Adt121::C0(List::cons(Adt12::C0(v0, v2, f568(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt121::C0(h1832, _) => {
                                    h1832
                                }
                            }), List::cons(v9, match v5 {
                                Adt121::C0(_, h1833) => {
                                    h1833
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt148::C0(Adt98::C9(v0)))
            }
        }
    }
}

pub fn f479(p1838: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1839: List<Adt106>) -> List<Adt106> {
    let m1837 = (p1838, p1839);
    {
        let mut m1837 = m1837;
        loop {
            let step = {
                let (v0, v3) = m1837;
                let m1834 = v0;
                match m1834.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v5, List::cons(Adt106::C0({
                            let (_, h1835) = v4;
                            h1835
                        }, v1, List::cons({
                            let (h1836, _) = v4;
                            h1836
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(p1843: List<(Str, Adt6)>, p1844: List<Adt106>) -> List<Adt106> {
    let m1842 = (p1843, p1844);
    {
        let mut m1842 = m1842;
        loop {
            let step = {
                let (v0, v1) = m1842;
                let m1840 = v0;
                match m1840.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0({
                            let (_, h1841) = v2;
                            h1841
                        }, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(p1847: List<Adt6>, p1848: List<Adt106>) -> List<Adt106> {
    let m1846 = (p1847, p1848);
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let (v0, v1) = m1846;
                let m1845 = v0;
                match m1845.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt106>), List<Adt106>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt106>), List<Adt106>>((v3, List::cons(Adt106::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(p1855: List<(Str, Adt152)>) -> Adt142 {
    let m1854 = p1855;
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let v0 = m1854;
                let m1849 = v0;
                match m1849.uncons() {
                    None => {
                        Err::<List<(Str, Adt152)>, Adt142>(Adt142::C3(List::<Adt125>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1851 = {
                            let (_, h1850) = v1;
                            h1850
                        };
                        match m1851 {
                            Adt152::C0 => {
                                Ok::<List<(Str, Adt152)>, Adt142>(v2)
                            }
                            Adt152::C1 => {
                                Err::<List<(Str, Adt152)>, Adt142>(Adt142::C0({
                                    let (h1852, _) = v1;
                                    h1852
                                }))
                            }
                            Adt152::C2 => {
                                Err::<List<(Str, Adt152)>, Adt142>(Adt142::C1({
                                    let (h1853, _) = v1;
                                    h1853
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f483(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1856 = f231(Str::lit("type"), v0);
    match m1856 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1857 = f228(v2.clone());
            match m1857 {
                None => {
                    let m1858 = f405(v2);
                    match m1858 {
                        None => {
                            Ok(Adt152::C2)
                        }
                        Some(v4) => {
                            f571(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f569(v3.clone()) {
                        if f570(v3, v1)? {
                            Ok(Adt152::C0)
                        } else {
                            Ok(Adt152::C1)
                        }
                    } else {
                        Ok(Adt152::C2)
                    }
                }
            }
        }
    }
}

pub fn f484(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1859 = f231(Str::lit("enum"), v0);
    match m1859 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1860 = f405(v2);
            match m1860 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v3) => {
                    f572(v3, v1)
                }
            }
        }
    }
}

pub fn f485(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt152> {
    let m1861 = f231(Str::lit("const"), v0);
    match m1861 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v2) => {
            let m1862 = f573(v2, v1)?;
            match m1862 {
                Adt154::C0 => {
                    Ok(Adt152::C0)
                }
                Adt154::C1 => {
                    Ok(Adt152::C1)
                }
                Adt154::C2 => {
                    Ok(Adt152::C2)
                }
            }
        }
    }
}

pub fn f486(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt152> {
    let m1863 = f231(v0, v2);
    match m1863 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v4) => {
            let m1864 = f574(v4);
            match m1864 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m1865 = f574(v3);
                    match m1865 {
                        None => {
                            Ok(Adt152::C0)
                        }
                        Some(v6) => {
                            if f575(v1, f576(v6, v5)?) {
                                Ok(Adt152::C0)
                            } else {
                                Ok(Adt152::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f487(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt152)>> {
    let m1866 = v1;
    match m1866 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt152)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt152)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt152)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f577(Str::lit("minLength"), true, v0.clone(), f578(v4.clone())?)?), List::cons((Str::lit("maxLength"), f577(Str::lit("maxLength"), false, v0, f578(v4)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f577(Str::lit("minItems"), true, v0.clone(), f579(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f577(Str::lit("maxItems"), false, v0, f579(v5, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f577(Str::lit("minProperties"), true, v0.clone(), f580(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f577(Str::lit("maxProperties"), false, v0, f580(v6, 0u64)?)?), List::<(Str, Adt152)>::nil())))
        }
    }
}

pub fn f488(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt152 {
    let m1867 = f231(Str::lit("required"), v0);
    match m1867 {
        Adt135::C0 => {
            Adt152::C0
        }
        Adt135::C1(v2) => {
            let m1868 = f405(v2);
            match m1868 {
                None => {
                    Adt152::C2
                }
                Some(v3) => {
                    let m1869 = f254(v1);
                    match m1869 {
                        None => {
                            Adt152::C0
                        }
                        Some(v4) => {
                            f581(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f489(v0: Adt142, v1: Adt142) -> Adt142 {
    let m1870 = v0.clone();
    match m1870 {
        Adt142::C0(_) => {
            v0
        }
        Adt142::C1(_) => {
            v0
        }
        Adt142::C2(_) => {
            v0
        }
        Adt142::C3(v5) => {
            let m1871 = v1.clone();
            match m1871 {
                Adt142::C0(_) => {
                    v1
                }
                Adt142::C1(_) => {
                    v1
                }
                Adt142::C2(_) => {
                    v1
                }
                Adt142::C3(v9) => {
                    Adt142::C3({
                        let a1872 = v5;
                        let a1873 = v9;
                        append_list(a1872, a1873)
                    })
                }
            }
        }
    }
}

pub fn f490(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>) -> Adt142 {
    let m1874 = f231(Str::lit("$ref"), v1);
    match m1874 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v4) => {
            let m1875 = f228(v4);
            match m1875 {
                None => {
                    Adt142::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1876 = f477(v0, v5.clone());
                    match m1876 {
                        None => {
                            Adt142::C2(v5)
                        }
                        Some(v6) => {
                            Adt142::C3(List::cons(Adt125::C0(v6, v2, v3), List::<Adt125>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f491(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1877 = f231(Str::lit("properties"), v0);
    match m1877 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1878 = f254(v3);
            match m1878 {
                None => {
                    Adt142::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1879 = f254(v1);
                    match m1879 {
                        None => {
                            Adt142::C3(List::<Adt125>::nil())
                        }
                        Some(v5) => {
                            Adt142::C3(f582(match v4 {
                                Adt5::C0(h1880) => {
                                    h1880
                                }
                            }, match v5 {
                                Adt5::C0(h1881) => {
                                    h1881
                                }
                            }, v2, List::<Adt125>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f492(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1882 = f231(Str::lit("additionalProperties"), v0.clone());
    match m1882 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1883 = f254(v1);
            match m1883 {
                None => {
                    Adt142::C3(List::<Adt125>::nil())
                }
                Some(v4) => {
                    let m1884 = f253(v3.clone());
                    match m1884 {
                        None => {
                            Adt142::C3(f585(match v4 {
                                Adt5::C0(h1886) => {
                                    h1886
                                }
                            }, f584(v0), v3, v2, List::<Adt125>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt142::C3(List::<Adt125>::nil())
                            } else {
                                if f583(match v4 {
                                    Adt5::C0(h1885) => {
                                        h1885
                                    }
                                }, f584(v0)) {
                                    Adt142::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt142::C3(List::<Adt125>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f493(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> R<Adt142> {
    let m1887 = f231(Str::lit("items"), v0);
    match m1887 {
        Adt135::C0 => {
            Ok(Adt142::C3(List::<Adt125>::nil()))
        }
        Adt135::C1(v3) => {
            let m1888 = f405(v1);
            match m1888 {
                None => {
                    Ok(Adt142::C3(List::<Adt125>::nil()))
                }
                Some(v4) => {
                    Ok(Adt142::C3(f586(v4, 0u64, v3, v2, List::<Adt125>::nil())?))
                }
            }
        }
    }
}

pub fn f494(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt123>) -> Adt142 {
    let m1889 = f231(Str::lit("allOf"), v0);
    match m1889 {
        Adt135::C0 => {
            Adt142::C3(List::<Adt125>::nil())
        }
        Adt135::C1(v3) => {
            let m1890 = f405(v3);
            match m1890 {
                None => {
                    Adt142::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1891 = v4.clone();
                    match m1891.uncons() {
                        None => {
                            Adt142::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt142::C3(f587(v4, v1, v2, List::<Adt125>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f495(v0: Adt126, v1: Adt142, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt123>) -> Adt142 {
    let m1892 = v1.clone();
    match m1892 {
        Adt142::C0(_) => {
            v1
        }
        Adt142::C1(_) => {
            v1
        }
        Adt142::C2(_) => {
            v1
        }
        Adt142::C3(v8) => {
            let m1893 = v8;
            match m1893.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt142::C3(List::cons(Adt125::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt125>::nil()))
                }
            }
        }
    }
}

pub fn f496(p1900: List<u64>, p1901: List<u64>) -> List<u64> {
    let m1899 = (p1900, p1901);
    {
        let mut m1899 = m1899;
        loop {
            let step = {
                let (v0, v1) = m1899;
                let m1894 = v0;
                match m1894.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1895 = v2;
                        let a1896 = 126u64;
                        if nat_eq(a1895, a1896) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1897 = v2;
                            let a1898 = 47u64;
                            if nat_eq(a1897, a1898) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(p1906: List<Adt51>, p1907: Adt7) -> R<bool> {
    let m1905 = (p1906, p1907);
    {
        let mut m1905 = m1905;
        loop {
            let step = {
                let (v0, v1) = m1905;
                let m1902 = v0;
                match m1902.uncons() {
                    None => {
                        Err::<(List<Adt51>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f588(f576(match v2.clone() {
                            Adt51::C0(h1903, _, _) => {
                                h1903
                            }
                        }, v1)?) {
                            Ok::<(List<Adt51>, Adt7), R<bool>>((v3, match v2 {
                                Adt51::C0(h1904, _, _) => {
                                    h1904
                                }
                            }))
                        } else {
                            Err::<(List<Adt51>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1905 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(p1916: Bytes, p1917: List<Bytes>) -> List<Bytes> {
    let m1915 = (p1916, p1917);
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v0, v1) = m1915;
                let m1908 = v1;
                match m1908.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1911 = {
                            let a1909 = v2.clone();
                            length_bytes(a1909)
                        };
                        let a1912 = {
                            let a1910 = v0.clone();
                            length_bytes(a1910)
                        };
                        if nat_eq(a1911, a1912) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1913 = v2;
                                let a1914 = v0;
                                append_bytes(a1913, a1914)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f499(v0: u64) -> Bytes {
    let m1921 = {
        let a1918 = f589();
        let a1919 = v0;
        let a1920 = 1u64;
        slice_bytes(a1918, a1919, a1920)
    };
    match m1921 {
        None => {
            f89()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f500(p1933: u64, p1934: i64, p1935: List<u64>) -> R<List<u64>> {
    let m1932 = (p1933, (p1934, p1935));
    {
        let mut m1932 = m1932;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1932;
                let m1922 = v0;
                if m1922 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1922 - 1;
                    let v4: i64 = {
                        let a1923 = v1;
                        let a1924 = 10i64;
                        let a1925 = 0i64;
                        int_quot(a1923, a1924, a1925)?
                    };
                    let v5: i64 = {
                        let a1926 = v1;
                        let a1927 = 10i64;
                        let a1928 = 0i64;
                        int_rem(a1926, a1927, a1928)
                    };
                    let m1931 = {
                        let a1929 = v4;
                        let a1930 = 1i64;
                        compare(a1929, a1930)
                    };
                    if match m1931 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f590(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f590(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1932 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: u64) -> R<u64> {
    let a1936 = v0;
    let a1937 = 10u64;
    if nat_lt(a1936, a1937) {
        let a1938 = 48u64;
        let a1939 = v0;
        nat_add(a1938, a1939)
    } else {
        let a1940 = 87u64;
        let a1941 = v0;
        nat_add(a1940, a1941)
    }
}

pub fn f502(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1942 = v1;
    match m1942 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f591(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f503(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1943 = v0;
    match m1943 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1946 = f421(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1944, _) => {
                        h1944
                    }
                });
                let a1947 = f421(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1945) => {
                        h1945
                    }
                });
                append_list(a1946, a1947)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f504(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f592(v0, 0i64)?, 0i64)))
}

pub fn f505(v0: Adt11) -> Str {
    let m1948 = v0;
    match m1948 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f506(p1951: List<Adt36>, p1952: List<Adt6>) -> List<Adt6> {
    let m1950 = (p1951, p1952);
    {
        let mut m1950 = m1950;
        loop {
            let step = {
                let (v0, v1) = m1950;
                let m1949 = v0;
                match m1949.uncons() {
                    None => {
                        Err::<(List<Adt36>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt36>, List<Adt6>), List<Adt6>>((v3, List::cons(f593(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1950 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(p1956: List<Adt39>, p1957: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1955 = (p1956, p1957);
    {
        let mut m1955 = m1955;
        loop {
            let step = {
                let (v0, v1) = m1955;
                let m1953 = v0;
                match m1953.uncons() {
                    None => {
                        Err::<(List<Adt39>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1954 = f594(v2)?;
                        match m1954 {
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
                Ok(next) => { m1955 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f508(p1960: List<Adt45>, p1961: List<Adt6>) -> R<List<Adt6>> {
    let m1959 = (p1960, p1961);
    {
        let mut m1959 = m1959;
        loop {
            let step = {
                let (v0, v1) = m1959;
                let m1958 = v0;
                match m1958.uncons() {
                    None => {
                        Err::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt45>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f595(v2)?, v1)))
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

pub fn f509(p1964: List<Adt47>, p1965: List<Adt6>) -> List<Adt6> {
    let m1963 = (p1964, p1965);
    {
        let mut m1963 = m1963;
        loop {
            let step = {
                let (v0, v1) = m1963;
                let m1962 = v0;
                match m1962.uncons() {
                    None => {
                        Err::<(List<Adt47>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt47>, List<Adt6>), List<Adt6>>((v3, List::cons(f596(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1963 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f510(p1968: List<Adt55>, p1969: List<Adt6>) -> List<Adt6> {
    let m1967 = (p1968, p1969);
    {
        let mut m1967 = m1967;
        loop {
            let step = {
                let (v0, v1) = m1967;
                let m1966 = v0;
                match m1966.uncons() {
                    None => {
                        Err::<(List<Adt55>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt55>, List<Adt6>), List<Adt6>>((v3, List::cons(f597(v2), v1)))
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

pub fn f511(p1972: List<Adt57>, p1973: List<Adt6>) -> List<Adt6> {
    let m1971 = (p1972, p1973);
    {
        let mut m1971 = m1971;
        loop {
            let step = {
                let (v0, v1) = m1971;
                let m1970 = v0;
                match m1970.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f598(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1971 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(p1977: List<Adt60>, p1978: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1976 = (p1977, p1978);
    {
        let mut m1976 = m1976;
        loop {
            let step = {
                let (v0, v1) = m1976;
                let m1974 = v0;
                match m1974.uncons() {
                    None => {
                        Err::<(List<Adt60>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1975 = f599(v2)?;
                        match m1975 {
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
                Ok(next) => { m1976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f513(v0: Adt65) -> R<Option<Adt6>> {
    let m1979 = v0;
    match m1979 {
        Adt65::C0(v1) => {
            Ok(Some(f600(Str::lit("elicitation/create"), Some(f601(v1)))))
        }
        Adt65::C1(v2) => {
            let m1980 = f602(v2)?;
            match m1980 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f600(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt65::C2(v4) => {
            Ok(Some(f600(Str::lit("roots/list"), {
                let m1981 = v4;
                match m1981 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h1982) => {
                                h1982
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f514(v0: List<(Str, Adt65)>) -> Option<Adt6> {
    let m1983 = f603(v0, List::<(Str, Adt6)>::nil());
    match m1983 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(f301(v1, List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f515(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1984 = v1;
    match m1984 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f516(v0: Option<i64>) -> Adt6 {
    let m1985 = v0;
    match m1985 {
        None => {
            Adt6::C0
        }
        Some(v1) => {
            f604(v1)
        }
    }
}

pub fn f517(v0: Str, v1: Option<i64>) -> List<(Str, Adt6)> {
    let m1986 = v1;
    match m1986 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f604(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f518(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1987 = v0;
    match m1987 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f605(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f519(v0: Adt6) -> Option<Adt85> {
    let m1988 = f254(v0);
    match m1988 {
        None => {
            None::<Adt85>
        }
        Some(v1) => {
            let m1989 = f606(Str::lit("task"), v1.clone());
            match m1989 {
                None => {
                    None::<Adt85>
                }
                Some(v2) => {
                    let m1990 = f607(v2);
                    match m1990 {
                        None => {
                            None::<Adt85>
                        }
                        Some(v3) => {
                            let m1991 = f608(Str::lit("createdMs"), v1.clone());
                            match m1991 {
                                None => {
                                    None::<Adt85>
                                }
                                Some(v4) => {
                                    let m1992 = f606(Str::lit("plan"), v1);
                                    match m1992 {
                                        None => {
                                            None::<Adt85>
                                        }
                                        Some(v5) => {
                                            let m1993 = f609(v5);
                                            match m1993 {
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

pub fn f520(p1997: List<Adt85>, p1998: List<Adt6>) -> Option<List<Adt6>> {
    let m1996 = (p1997, p1998);
    {
        let mut m1996 = m1996;
        loop {
            let step = {
                let (v0, v1) = m1996;
                let m1994 = v0;
                match m1994.uncons() {
                    None => {
                        Err::<(List<Adt85>, List<Adt6>), Option<List<Adt6>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1995 = f610(v2);
                        match m1995 {
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
                Ok(next) => { m1996 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Adt85, v1: i64) -> R<bool> {
    let m2002 = {
        let m2001 = f435(match v0.clone() {
            Adt85::C0(h2000, _, _) => {
                h2000
            }
        });
        match m2001 {
            Adt64::C0(_, _, _, _, h1999, _) => {
                h1999
            }
        }
    };
    match m2002 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let a2009 = {
                let m2008 = {
                    let a2006 = {
                        let a2004 = v1;
                        let a2005 = match v0 {
                            Adt85::C0(_, h2003, _) => {
                                h2003
                            }
                        };
                        int_sub(a2004, a2005)?
                    };
                    let a2007 = v2;
                    compare(a2006, a2007)
                };
                match m2008 {
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
            Ok(bool_not(a2009))
        }
    }
}

pub fn f522() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f523() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f524() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f525() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f526() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f527(p2014: List<List<u64>>, p2015: List<u64>) -> List<u64> {
    let m2013 = (p2014, p2015);
    {
        let mut m2013 = m2013;
        loop {
            let step = {
                let (v0, v1) = m2013;
                let m2010 = v0;
                match m2010.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a2011 = f611(v2);
                            let a2012 = v1;
                            append_list(a2011, a2012)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2013 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(v0: Str) -> R<Str> {
    let m2018 = {
        let a2017 = f333(f89(), f164(f612(f86({
            let a2016 = v0;
            utf8_encode(a2016)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a2017)
    };
    match m2018 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f529(v0: List<u64>) -> bool {
    let m2019 = v0;
    match m2019.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a2020 = v1;
            let a2021 = 48u64;
            if nat_eq(a2020, a2021) {
                f613(v2)
            } else {
                false
            }
        }
    }
}

pub fn f530(v0: Adt0, v1: Fn2, v2: u64, v3: Adt15) -> R<Result<Adt128, Adt131>> {
    if f614(f615(v3.clone())) {
        Ok(Ok::<Adt128, Adt131>(f616(v0.clone(), Adt33::C0(f617(v0)))))
    } else {
        Ok(f618(v0.clone(), v3.clone(), f619(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h2022) => {
                h2022
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h2023, _) => {
                h2023
            }
        }, v2, v1, v3)?))
    }
}

pub fn f531(v0: Adt17, _: Adt5) -> Result<Adt15, Adt132> {
    Ok::<Adt15, Adt132>(Adt15::C0(Adt16::C0(v0)))
}

pub fn f532(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2024 = f620(Str::lit("cursor"), v1);
    match m2024 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C1(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f533(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2025 = f621(Str::lit("name"), v1.clone());
    match m2025 {
        Ok(v2) => {
            let m2026 = f622(Str::lit("arguments"), v1.clone());
            match m2026 {
                Ok(v3) => {
                    let m2027 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2027 {
                        Ok(v4) => {
                            let m2028 = f620(Str::lit("requestState"), v1);
                            match m2028 {
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

pub fn f534(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2029 = f620(Str::lit("cursor"), v1);
    match m2029 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C3(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f535(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2030 = f621(Str::lit("uri"), v1.clone());
    match m2030 {
        Ok(v2) => {
            let m2031 = f622(Str::lit("inputResponses"), v1.clone());
            match m2031 {
                Ok(v3) => {
                    let m2032 = f620(Str::lit("requestState"), v1);
                    match m2032 {
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

pub fn f536(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2033 = f620(Str::lit("cursor"), v1);
    match m2033 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C5(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f537(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2034 = f620(Str::lit("cursor"), v1);
    match m2034 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C6(Adt21::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f538(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2035 = f621(Str::lit("name"), v1.clone());
    match m2035 {
        Ok(v2) => {
            let m2036 = f623(Str::lit("arguments"), v1.clone());
            match m2036 {
                Ok(v3) => {
                    let m2037 = f622(Str::lit("inputResponses"), v1.clone());
                    match m2037 {
                        Ok(v4) => {
                            let m2038 = f620(Str::lit("requestState"), v1);
                            match m2038 {
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

pub fn f539(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2039 = f624(Str::lit("ref"), v1.clone());
    match m2039 {
        Ok(v2) => {
            let m2040 = f625(v2);
            match m2040 {
                Ok(v3) => {
                    let m2041 = f624(Str::lit("argument"), v1.clone());
                    match m2041 {
                        Ok(v4) => {
                            let m2042 = f626(v4);
                            match m2042 {
                                Ok(v5) => {
                                    let m2043 = f627(v1);
                                    match m2043 {
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

pub fn f540(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2044 = f621(Str::lit("taskId"), v1);
    match m2044 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C9(Adt30::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f541(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2045 = f621(Str::lit("taskId"), v1.clone());
    match m2045 {
        Ok(v2) => {
            let m2046 = f624(Str::lit("inputResponses"), v1);
            match m2046 {
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

pub fn f542(v0: Adt17, v1: Adt5) -> Result<Adt15, Adt132> {
    let m2047 = f621(Str::lit("taskId"), v1);
    match m2047 {
        Ok(v2) => {
            Ok::<Adt15, Adt132>(Adt15::C11(Adt32::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt15, Adt132>(v3)
        }
    }
}

pub fn f543(v0: Adt105, v1: Adt17, v2: Option<Adt131>) -> Adt144 {
    let m2048 = v2;
    match m2048 {
        None => {
            Adt144::C0(Adt145::C0(match v0.clone() {
                Adt105::C0(h2049, _, _) => {
                    h2049
                }
            }, match v0.clone() {
                Adt105::C0(_, h2050, _) => {
                    h2050
                }
            }, v1, f468(match v0 {
                Adt105::C0(_, _, h2051) => {
                    h2051
                }
            })))
        }
        Some(v3) => {
            Adt144::C1(v3)
        }
    }
}

pub fn f544(v0: Str, v1: List<Str>) -> Option<Adt131> {
    f628(f629(v0.clone(), v1.clone()), v0, v1)
}

pub fn f545(v0: Adt108, v1: Adt146, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt109>> {
    let m2053 = match v0.clone() {
        Adt108::C0(h2052, _, _, _, _) => {
            h2052
        }
    };
    match m2053 {
        None => {
            Ok(Some(Adt109::C0))
        }
        Some(v5) => {
            let a2054 = v5;
            let a2055 = v3;
            if equal(a2054, a2055) {
                let m2057 = match v0.clone() {
                    Adt108::C0(_, h2056, _, _, _) => {
                        h2056
                    }
                };
                match m2057 {
                    None => {
                        Ok(Some(Adt109::C1))
                    }
                    Some(v6) => {
                        let a2058 = v6;
                        let a2059 = v2;
                        if equal(a2058, a2059) {
                            f630(f631(match v0.clone() {
                                Adt108::C0(_, _, h2060, _, _) => {
                                    h2060
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt108::C0(_, _, _, _, h2061) => {
                                    h2061
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt109::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt109::C3))
            }
        }
    }
}

pub fn f546() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f547(v0: Adt5) -> Adt153 {
    f632(f177(f549(), v0.clone()), f177(f550(), v0))
}

pub fn f548(v0: Adt5, v1: Adt135, v2: Adt135) -> Result<Adt17, Adt132> {
    let m2062 = v1;
    match m2062 {
        Adt135::C0 => {
            Err::<Adt17, Adt132>(Adt132::C2)
        }
        Adt135::C1(v3) => {
            let m2063 = v2;
            match m2063 {
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

pub fn f549() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f550() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f551(v0: u64) -> R<Option<u64>> {
    let a2064 = 64u64;
    let a2065 = v0;
    if if nat_lt(a2064, a2065) {
        let a2066 = v0;
        let a2067 = 91u64;
        nat_lt(a2066, a2067)
    } else {
        false
    } {
        Ok(Some({
            let a2068 = v0;
            let a2069 = 65u64;
            nat_sub(a2068, a2069)
        }))
    } else {
        let a2070 = 96u64;
        let a2071 = v0;
        if if nat_lt(a2070, a2071) {
            let a2072 = v0;
            let a2073 = 123u64;
            nat_lt(a2072, a2073)
        } else {
            false
        } {
            Ok(Some({
                let a2074 = v0;
                let a2075 = 71u64;
                nat_sub(a2074, a2075)
            }))
        } else {
            let a2076 = 47u64;
            let a2077 = v0;
            if if nat_lt(a2076, a2077) {
                let a2078 = v0;
                let a2079 = 58u64;
                nat_lt(a2078, a2079)
            } else {
                false
            } {
                Ok(Some({
                    let a2080 = v0;
                    let a2081 = 4u64;
                    nat_add(a2080, a2081)?
                }))
            } else {
                let a2082 = v0;
                let a2083 = 43u64;
                if nat_eq(a2082, a2083) {
                    Ok(Some(62u64))
                } else {
                    let a2084 = v0;
                    let a2085 = 47u64;
                    if nat_eq(a2084, a2085) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f552(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2086 = v3;
        let a2087 = 61u64;
        if nat_eq(a2086, a2087) {
            let a2088 = f634(v1, 16u64);
            let a2089 = 0u64;
            nat_eq(a2088, a2089)
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

pub fn f553(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2090 = f634(v2, 4u64);
        let a2091 = 0u64;
        nat_eq(a2090, a2091)
    } else {
        false
    } {
        Ok(Some(f636(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f554(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2094 = {
            let a2092 = v0;
            let a2093 = 4u64;
            nat_mul(a2092, a2093)?
        };
        let a2095 = f637(v1, 16u64);
        nat_add(a2094, a2095)?
    }, List::cons({
        let a2098 = {
            let a2096 = f634(v1, 16u64);
            let a2097 = 16u64;
            nat_mul(a2096, a2097)?
        };
        let a2099 = f637(v2, 4u64);
        nat_add(a2098, a2099)?
    }, List::cons({
        let a2102 = {
            let a2100 = f634(v2, 4u64);
            let a2101 = 64u64;
            nat_mul(a2100, a2101)?
        };
        let a2103 = v3;
        nat_add(a2102, a2103)?
    }, List::<u64>::nil()))))
}

pub fn f555(v0: Adt6) -> bool {
    let m2104 = f228(v0);
    match m2104 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f556(v0: Adt6) -> bool {
    let m2105 = f574(v0);
    match m2105 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f557(v0: Adt6) -> bool {
    let m2106 = f638(v0);
    match m2106 {
        None => {
            false
        }
        Some(v1) => {
            let a2110 = {
                let m2109 = {
                    let a2107 = v1;
                    let a2108 = 0i64;
                    compare(a2107, a2108)
                };
                match m2109 {
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
            bool_not(a2110)
        }
    }
}

pub fn f558(v0: Adt6) -> bool {
    let m2111 = f405(v0);
    match m2111 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f559(v0: Adt6) -> bool {
    let m2112 = f254(v0);
    match m2112 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f560(v0: Adt6) -> bool {
    let m2113 = f405(v0);
    match m2113 {
        None => {
            false
        }
        Some(v1) => {
            f639(v1)
        }
    }
}

pub fn f561(v0: Adt6) -> bool {
    let m2114 = f253(v0);
    match m2114 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f562(v0: Adt6) -> bool {
    let m2115 = f228(v0.clone());
    match m2115 {
        None => {
            let m2116 = f405(v0);
            match m2116 {
                None => {
                    false
                }
                Some(v2) => {
                    f640(v2)
                }
            }
        }
        Some(v1) => {
            f569(v1)
        }
    }
}

pub fn f563(v0: Adt6) -> bool {
    let m2117 = f405(v0);
    match m2117 {
        None => {
            false
        }
        Some(v1) => {
            let m2118 = v1;
            match m2118.uncons() {
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

pub fn f564(v0: Str, p2136: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2135 = p2136;
    {
        let mut m2135 = m2135;
        loop {
            let step = {
                let v1 = m2135;
                let m2119 = v1;
                match m2119.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2123 = v0.clone();
                        let a2124 = {
                            let a2121 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2120, _) = v2.clone();
                                h2120
                            }, List::<Str>::nil()));
                            let a2122 = Str::lit("");
                            join(a2121, a2122)
                        };
                        if equal(a2123, a2124) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2125) = v2;
                                h2125
                            }))
                        } else {
                            let m2127 = f254({
                                let (_, h2126) = v2.clone();
                                h2126
                            });
                            match m2127 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2128 = f177(Str::lit("$anchor"), v4);
                                    match m2128 {
                                        Adt135::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt135::C1(v5) => {
                                            let m2129 = f228(v5);
                                            match m2129 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2132 = v0.clone();
                                                    let a2133 = {
                                                        let a2130 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2131 = Str::lit("");
                                                        join(a2130, a2131)
                                                    };
                                                    if equal(a2132, a2133) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2134) = v2;
                                                            h2134
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
                Ok(next) => { m2135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f565(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2137 = f231(Str::lit("type"), v0);
    match m2137 {
        Adt135::C0 => {
            None::<Adt13>
        }
        Adt135::C1(v1) => {
            let m2138 = f228(v1);
            match m2138 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f641(v2)
                }
            }
        }
    }
}

pub fn f566(v0: List<u64>) -> R<List<u64>> {
    Ok(f164(f642(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f567(v0: List<u64>, p2141: List<List<u64>>) -> bool {
    let m2140 = p2141;
    {
        let mut m2140 = m2140;
        loop {
            let step = {
                let v1 = m2140;
                let m2139 = v1;
                match m2139.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f643(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
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

pub fn f568(p2144: List<Str>, p2145: List<Str>) -> List<Str> {
    let m2143 = (p2144, p2145);
    {
        let mut m2143 = m2143;
        loop {
            let step = {
                let (v0, v1) = m2143;
                let m2142 = v0;
                match m2142.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: Str) -> bool {
    let a2146 = v0.clone();
    let a2147 = Str::lit("null");
    if equal(a2146, a2147) {
        true
    } else {
        let a2148 = v0.clone();
        let a2149 = Str::lit("boolean");
        if equal(a2148, a2149) {
            true
        } else {
            let a2150 = v0.clone();
            let a2151 = Str::lit("object");
            if equal(a2150, a2151) {
                true
            } else {
                let a2152 = v0.clone();
                let a2153 = Str::lit("array");
                if equal(a2152, a2153) {
                    true
                } else {
                    let a2154 = v0.clone();
                    let a2155 = Str::lit("number");
                    if equal(a2154, a2155) {
                        true
                    } else {
                        let a2156 = v0.clone();
                        let a2157 = Str::lit("string");
                        if equal(a2156, a2157) {
                            true
                        } else {
                            let a2158 = v0;
                            let a2159 = Str::lit("integer");
                            equal(a2158, a2159)
                        }
                    }
                }
            }
        }
    }
}

pub fn f570(v0: Str, v1: Adt6) -> R<bool> {
    let a2160 = v0.clone();
    let a2161 = f644(v1.clone());
    if equal(a2160, a2161) {
        Ok(true)
    } else {
        let a2162 = v0;
        let a2163 = Str::lit("integer");
        if equal(a2162, a2163) {
            let m2164 = f574(v1);
            match m2164 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f645(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f571(p2167: List<Adt6>, v1: Adt6, p2168: bool) -> R<Adt152> {
    let m2166 = (p2167, p2168);
    {
        let mut m2166 = m2166;
        loop {
            let step = {
                let (v0, v2) = m2166;
                let m2165 = v0;
                match m2165.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f569(f646(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt152>>((v4, if v2 {
                                true
                            } else {
                                f570(f646(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt152>>(Ok(Adt152::C2))
                        }
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

pub fn f572(p2172: List<Adt6>, v1: Adt6) -> R<Adt152> {
    let m2171 = p2172;
    {
        let mut m2171 = m2171;
        loop {
            let step = {
                let v0 = m2171;
                let m2169 = v0;
                match m2169.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C1))
                    }
                    Some((v2, v3)) => {
                        let m2170 = f573(v2, v1.clone())?;
                        match m2170 {
                            Adt154::C0 => {
                                Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C0))
                            }
                            Adt154::C1 => {
                                Ok::<List<Adt6>, R<Adt152>>(v3)
                            }
                            Adt154::C2 => {
                                Err::<List<Adt6>, R<Adt152>>(Ok(Adt152::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2171 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Adt6, v1: Adt6) -> R<Adt154> {
    f647(f648(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f574(v0: Adt6) -> Option<Adt7> {
    let m2173 = v0;
    match m2173 {
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

pub fn f575(v0: u64, v1: Adt155) -> bool {
    let a2174 = v0;
    let a2175 = 0u64;
    if nat_eq(a2174, a2175) {
        let a2176 = f649(v1);
        bool_not(a2176)
    } else {
        let a2177 = v0;
        let a2178 = 1u64;
        if nat_eq(a2177, a2178) {
            let a2179 = f588(v1);
            bool_not(a2179)
        } else {
            let a2180 = v0;
            let a2181 = 2u64;
            if nat_eq(a2180, a2181) {
                f588(v1)
            } else {
                f649(v1)
            }
        }
    }
}

pub fn f576(v0: Adt7, v1: Adt7) -> R<Adt155> {
    f650(v0.clone(), v1.clone(), f651(match v0 {
        Adt7::C0(h2182, _) => {
            h2182
        }
    }), f651(match v1 {
        Adt7::C0(h2183, _) => {
            h2183
        }
    }))
}

pub fn f577(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt152> {
    let m2184 = f231(v0, v2);
    match m2184 {
        Adt135::C0 => {
            Ok(Adt152::C0)
        }
        Adt135::C1(v4) => {
            let m2185 = f638(v4);
            match m2185 {
                None => {
                    Ok(Adt152::C2)
                }
                Some(v5) => {
                    let m2188 = {
                        let a2186 = v5;
                        let a2187 = 0i64;
                        compare(a2186, a2187)
                    };
                    if match m2188 {
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
                        Ok(Adt152::C2)
                    } else {
                        if v1 {
                            if f649(f652(f80(v3, 0i64)?, v5)) {
                                Ok(Adt152::C1)
                            } else {
                                Ok(Adt152::C0)
                            }
                        } else {
                            if f588(f652(f80(v3, 0i64)?, v5)) {
                                Ok(Adt152::C1)
                            } else {
                                Ok(Adt152::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f578(v0: Str) -> R<u64> {
    f653(f86({
        let a2189 = v0;
        utf8_encode(a2189)
    })?, 0u64)
}

pub fn f579(p2194: List<Adt6>, p2195: u64) -> R<u64> {
    let m2193 = (p2194, p2195);
    {
        let mut m2193 = m2193;
        loop {
            let step = {
                let (v0, v1) = m2193;
                let m2190 = v0;
                match m2190.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a2191 = v1;
                            let a2192 = 1u64;
                            nat_add(a2191, a2192)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2193 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f580(p2200: List<(Str, Adt6)>, p2201: u64) -> R<u64> {
    let m2199 = (p2200, p2201);
    {
        let mut m2199 = m2199;
        loop {
            let step = {
                let (v0, v1) = m2199;
                let m2196 = v0;
                match m2196.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a2197 = v1;
                            let a2198 = 1u64;
                            nat_add(a2197, a2198)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m2199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f581(p2205: List<Adt6>, v1: Adt5) -> Adt152 {
    let m2204 = p2205;
    {
        let mut m2204 = m2204;
        loop {
            let step = {
                let v0 = m2204;
                let m2202 = v0;
                match m2202.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt152>(Adt152::C0)
                    }
                    Some((v2, v3)) => {
                        let m2203 = f228(v2);
                        match m2203 {
                            None => {
                                Err::<List<Adt6>, Adt152>(Adt152::C2)
                            }
                            Some(v4) => {
                                if f232(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt152>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt152>(Adt152::C1)
                                }
                            }
                        }
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

pub fn f582(p2212: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt123>, p2213: List<Adt125>) -> List<Adt125> {
    let m2211 = (p2212, p2213);
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let (v0, v3) = m2211;
                let m2206 = v0;
                match m2206.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m2208 = f231({
                            let (h2207, _) = v4.clone();
                            h2207
                        }, v1.clone());
                        match m2208 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, v3))
                            }
                            Adt135::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0({
                                    let (_, h2209) = v4;
                                    h2209
                                }, v6, List::cons(Adt123::C0({
                                    let (h2210, _) = v4;
                                    h2210
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f583(p2218: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m2217 = p2218;
    {
        let mut m2217 = m2217;
        loop {
            let step = {
                let v0 = m2217;
                let m2214 = v0;
                match m2214.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m2216 = f231({
                            let (h2215, _) = v2;
                            h2215
                        }, v1.clone());
                        match m2216 {
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
                Ok(next) => { m2217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m2219 = f231(Str::lit("properties"), v0);
    match m2219 {
        Adt135::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt135::C1(v1) => {
            let m2220 = f254(v1);
            match m2220 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h2221) => {
                            h2221
                        }
                    }
                }
            }
        }
    }
}

pub fn f585(p2228: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt123>, p2229: List<Adt125>) -> List<Adt125> {
    let m2227 = (p2228, p2229);
    {
        let mut m2227 = m2227;
        loop {
            let step = {
                let (v0, v4) = m2227;
                let m2222 = v0;
                match m2222.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m2224 = f231({
                            let (h2223, _) = v5.clone();
                            h2223
                        }, v1.clone());
                        match m2224 {
                            Adt135::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v6, List::cons(Adt125::C0(v2.clone(), {
                                    let (_, h2225) = v5;
                                    h2225
                                }, List::cons(Adt123::C0({
                                    let (h2226, _) = v5;
                                    h2226
                                }), v3.clone())), v4)))
                            }
                            Adt135::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt125>), List<Adt125>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(p2234: List<Adt6>, p2235: u64, v2: Adt6, v3: List<Adt123>, p2236: List<Adt125>) -> R<List<Adt125>> {
    let m2233 = (p2234, (p2235, p2236));
    {
        let mut m2233 = m2233;
        loop {
            let step = {
                let (v0, (v1, v4)) = m2233;
                let m2230 = v0;
                match m2230.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt125>)), R<List<Adt125>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt125>)), R<List<Adt125>>>((v6, ({
                            let a2231 = v1;
                            let a2232 = 1u64;
                            nat_add(a2231, a2232)?
                        }, List::cons(Adt125::C0(v2.clone(), v5, List::cons(Adt123::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m2233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f587(p2239: List<Adt6>, v1: Adt6, v2: List<Adt123>, p2240: List<Adt125>) -> List<Adt125> {
    let m2238 = (p2239, p2240);
    {
        let mut m2238 = m2238;
        loop {
            let step = {
                let (v0, v3) = m2238;
                let m2237 = v0;
                match m2237.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt125>), List<Adt125>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt125>), List<Adt125>>((v5, List::cons(Adt125::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m2238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: Adt155) -> bool {
    let m2241 = v0;
    match m2241 {
        Adt155::C0 => {
            false
        }
        Adt155::C1 => {
            false
        }
        Adt155::C2 => {
            true
        }
    }
}

pub fn f589() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f590(v0: i64) -> u64 {
    let m2244 = {
        let a2242 = v0;
        let a2243 = 1i64;
        compare(a2242, a2243)
    };
    if match m2244 {
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
        let m2247 = {
            let a2245 = v0;
            let a2246 = 2i64;
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
            49u64
        } else {
            let m2250 = {
                let a2248 = v0;
                let a2249 = 3i64;
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
                50u64
            } else {
                let m2253 = {
                    let a2251 = v0;
                    let a2252 = 4i64;
                    compare(a2251, a2252)
                };
                if match m2253 {
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
                    let m2256 = {
                        let a2254 = v0;
                        let a2255 = 5i64;
                        compare(a2254, a2255)
                    };
                    if match m2256 {
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
                        let m2259 = {
                            let a2257 = v0;
                            let a2258 = 6i64;
                            compare(a2257, a2258)
                        };
                        if match m2259 {
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
                            let m2262 = {
                                let a2260 = v0;
                                let a2261 = 7i64;
                                compare(a2260, a2261)
                            };
                            if match m2262 {
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
                                let m2265 = {
                                    let a2263 = v0;
                                    let a2264 = 8i64;
                                    compare(a2263, a2264)
                                };
                                if match m2265 {
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
                                    let m2268 = {
                                        let a2266 = v0;
                                        let a2267 = 9i64;
                                        compare(a2266, a2267)
                                    };
                                    if match m2268 {
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

pub fn f591(v0: Adt8) -> Adt6 {
    Adt6::C5(f421(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2269) => {
            h2269
        }
    }))
}

pub fn f592(p2274: u64, p2275: i64) -> R<i64> {
    let m2273 = (p2274, p2275);
    {
        let mut m2273 = m2273;
        loop {
            let step = {
                let (v0, v1) = m2273;
                let m2270 = v0;
                if m2270 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2270 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2271 = v1;
                        let a2272 = -1i64;
                        int_sub(a2271, a2272)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2273 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f593(v0: Adt36) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt36::C0(h2276, _, _, _, _, _, _, _) => {
            h2276
        }
    })), {
        let a2294 = f416(Str::lit("title"), match v0.clone() {
            Adt36::C0(_, h2277, _, _, _, _, _, _) => {
                h2277
            }
        });
        let a2295 = {
            let a2292 = f518(match v0.clone() {
                Adt36::C0(_, _, h2278, _, _, _, _, _) => {
                    h2278
                }
            });
            let a2293 = {
                let a2290 = f416(Str::lit("description"), match v0.clone() {
                    Adt36::C0(_, _, _, h2279, _, _, _, _) => {
                        h2279
                    }
                });
                let a2291 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2282 = match v0.clone() {
                        Adt36::C0(_, _, _, _, h2281, _, _, _) => {
                            h2281
                        }
                    };
                    match m2282 {
                        Adt5::C0(h2280) => {
                            h2280
                        }
                    }
                })), {
                    let a2288 = f457(Str::lit("outputSchema"), match v0.clone() {
                        Adt36::C0(_, _, _, _, _, h2283, _, _) => {
                            h2283
                        }
                    });
                    let a2289 = {
                        let a2286 = f654(match v0.clone() {
                            Adt36::C0(_, _, _, _, _, _, h2284, _) => {
                                h2284
                            }
                        });
                        let a2287 = f655(match v0 {
                            Adt36::C0(_, _, _, _, _, _, _, h2285) => {
                                h2285
                            }
                        });
                        append_list(a2286, a2287)
                    };
                    append_list(a2288, a2289)
                });
                append_list(a2290, a2291)
            };
            append_list(a2292, a2293)
        };
        append_list(a2294, a2295)
    }))
}

pub fn f594(v0: Adt39) -> R<Option<Adt6>> {
    let m2296 = v0;
    match m2296 {
        Adt39::C0(v1) => {
            Ok(Some(f656(v1)))
        }
        Adt39::C1(v2) => {
            Ok(Some(f657(Str::lit("image"), match v2.clone() {
                Adt43::C0(h2297, _, _, _) => {
                    h2297
                }
            }, match v2.clone() {
                Adt43::C0(_, h2298, _, _) => {
                    h2298
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h2299, _) => {
                    h2299
                }
            }, match v2 {
                Adt43::C0(_, _, _, h2300) => {
                    h2300
                }
            })))
        }
        Adt39::C2(v3) => {
            Ok(Some(f657(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h2301, _, _, _) => {
                    h2301
                }
            }, match v3.clone() {
                Adt44::C0(_, h2302, _, _) => {
                    h2302
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h2303, _) => {
                    h2303
                }
            }, match v3 {
                Adt44::C0(_, _, _, h2304) => {
                    h2304
                }
            })))
        }
        Adt39::C3(v4) => {
            Ok(Some(f658(v4)?))
        }
        Adt39::C4(v5) => {
            Ok(Some(f659(v5)))
        }
    }
}

pub fn f595(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2328 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2305, _, _, _, _, _, _, _, _) => {
                h2305
            }
        })), List::<(Str, Adt6)>::nil());
        let a2329 = {
            let a2326 = f416(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2306, _, _, _, _, _, _, _) => {
                    h2306
                }
            });
            let a2327 = {
                let a2324 = f518(match v0.clone() {
                    Adt45::C0(_, _, h2307, _, _, _, _, _, _) => {
                        h2307
                    }
                });
                let a2325 = {
                    let a2322 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2308, _, _, _, _, _) => {
                            h2308
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2323 = {
                        let a2320 = f416(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2309, _, _, _, _) => {
                                h2309
                            }
                        });
                        let a2321 = {
                            let a2318 = f416(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2310, _, _, _) => {
                                    h2310
                                }
                            });
                            let a2319 = {
                                let a2316 = f660(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2311, _, _) => {
                                        h2311
                                    }
                                });
                                let a2317 = {
                                    let a2314 = f661(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2312, _) => {
                                            h2312
                                        }
                                    })?;
                                    let a2315 = f655(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2313) => {
                                            h2313
                                        }
                                    });
                                    append_list(a2314, a2315)
                                };
                                append_list(a2316, a2317)
                            };
                            append_list(a2318, a2319)
                        };
                        append_list(a2320, a2321)
                    };
                    append_list(a2322, a2323)
                };
                append_list(a2324, a2325)
            };
            append_list(a2326, a2327)
        };
        append_list(a2328, a2329)
    }))
}

pub fn f596(v0: Adt47) -> Adt6 {
    let m2330 = v0;
    match m2330 {
        Adt47::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt48::C0(h2331, _, _, _) => {
                    h2331
                }
            })), {
                let a2335 = f416(Str::lit("mimeType"), match v1.clone() {
                    Adt48::C0(_, h2332, _, _) => {
                        h2332
                    }
                });
                let a2336 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt48::C0(_, _, h2333, _) => {
                        h2333
                    }
                })), f655(match v1 {
                    Adt48::C0(_, _, _, h2334) => {
                        h2334
                    }
                }));
                append_list(a2335, a2336)
            }))
        }
        Adt47::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt49::C0(h2337, _, _, _) => {
                    h2337
                }
            })), {
                let a2341 = f416(Str::lit("mimeType"), match v2.clone() {
                    Adt49::C0(_, h2338, _, _) => {
                        h2338
                    }
                });
                let a2342 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt49::C0(_, _, h2339, _) => {
                        h2339
                    }
                })), f655(match v2 {
                    Adt49::C0(_, _, _, h2340) => {
                        h2340
                    }
                }));
                append_list(a2341, a2342)
            }))
        }
    }
}

pub fn f597(v0: Adt55) -> Adt6 {
    Adt6::C5({
        let a2363 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt55::C0(h2343, _, _, _, _, _, _, _) => {
                h2343
            }
        })), List::<(Str, Adt6)>::nil());
        let a2364 = {
            let a2361 = f416(Str::lit("title"), match v0.clone() {
                Adt55::C0(_, h2344, _, _, _, _, _, _) => {
                    h2344
                }
            });
            let a2362 = {
                let a2359 = f518(match v0.clone() {
                    Adt55::C0(_, _, h2345, _, _, _, _, _) => {
                        h2345
                    }
                });
                let a2360 = {
                    let a2357 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt55::C0(_, _, _, h2346, _, _, _, _) => {
                            h2346
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2358 = {
                        let a2355 = f416(Str::lit("description"), match v0.clone() {
                            Adt55::C0(_, _, _, _, h2347, _, _, _) => {
                                h2347
                            }
                        });
                        let a2356 = {
                            let a2353 = f416(Str::lit("mimeType"), match v0.clone() {
                                Adt55::C0(_, _, _, _, _, h2348, _, _) => {
                                    h2348
                                }
                            });
                            let a2354 = {
                                let a2351 = f660(match v0.clone() {
                                    Adt55::C0(_, _, _, _, _, _, h2349, _) => {
                                        h2349
                                    }
                                });
                                let a2352 = f655(match v0 {
                                    Adt55::C0(_, _, _, _, _, _, _, h2350) => {
                                        h2350
                                    }
                                });
                                append_list(a2351, a2352)
                            };
                            append_list(a2353, a2354)
                        };
                        append_list(a2355, a2356)
                    };
                    append_list(a2357, a2358)
                };
                append_list(a2359, a2360)
            };
            append_list(a2361, a2362)
        };
        append_list(a2363, a2364)
    })
}

pub fn f598(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2380 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2365, _, _, _, _, _) => {
                h2365
            }
        })), List::<(Str, Adt6)>::nil());
        let a2381 = {
            let a2378 = f416(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2366, _, _, _, _) => {
                    h2366
                }
            });
            let a2379 = {
                let a2376 = f518(match v0.clone() {
                    Adt57::C0(_, _, h2367, _, _, _) => {
                        h2367
                    }
                });
                let a2377 = {
                    let a2374 = f416(Str::lit("description"), match v0.clone() {
                        Adt57::C0(_, _, _, h2368, _, _) => {
                            h2368
                        }
                    });
                    let a2375 = {
                        let a2372 = {
                            let m2370 = match v0.clone() {
                                Adt57::C0(_, _, _, _, h2369, _) => {
                                    h2369
                                }
                            };
                            match m2370 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f662(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2373 = f655(match v0 {
                            Adt57::C0(_, _, _, _, _, h2371) => {
                                h2371
                            }
                        });
                        append_list(a2372, a2373)
                    };
                    append_list(a2374, a2375)
                };
                append_list(a2376, a2377)
            };
            append_list(a2378, a2379)
        };
        append_list(a2380, a2381)
    })
}

pub fn f599(v0: Adt60) -> R<Option<Adt6>> {
    let m2383 = f594(match v0.clone() {
        Adt60::C0(_, h2382) => {
            h2382
        }
    })?;
    match m2383 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f663(match v0 {
                Adt60::C0(h2384, _) => {
                    h2384
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f600(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f420(Str::lit("params"), v1)))
}

pub fn f601(v0: Adt66) -> Adt6 {
    let m2385 = v0;
    match m2385 {
        Adt66::C0(v1) => {
            f664(v1)
        }
        Adt66::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt69::C0(h2386, _) => {
                    h2386
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt69::C0(_, h2387) => {
                    h2387
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f602(v0: Adt70) -> R<Option<Adt6>> {
    let m2389 = f665(match v0.clone() {
        Adt70::C0(h2388, _, _, _, _, _, _, _, _, _) => {
            h2388
        }
    })?;
    match m2389 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2391 = f666(v0.clone());
                let a2392 = List::cons((Str::lit("maxTokens"), f504(match v0.clone() {
                    Adt70::C0(_, _, _, _, _, h2390, _, _, _, _) => {
                        h2390
                    }
                })?), f667(v0));
                append_list(a2391, a2392)
            }))))
        }
    }
}

pub fn f603(p2398: List<(Str, Adt65)>, p2399: List<(Str, Adt6)>) -> Option<List<(Str, Adt6)>> {
    let m2397 = (p2398, p2399);
    {
        let mut m2397 = m2397;
        loop {
            let step = {
                let (v0, v1) = m2397;
                let m2393 = v0;
                match m2393.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m2395 = f668({
                            let (_, h2394) = v2;
                            h2394
                        });
                        match m2395 {
                            None => {
                                Err::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>(None::<List<(Str, Adt6)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt65)>, List<(Str, Adt6)>), Option<List<(Str, Adt6)>>>((v3, List::cons(({
                                    let (h2396, _) = v2;
                                    h2396
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2397 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: i64) -> Adt6 {
    Adt6::C2(Adt7::C0(v0, 0i64))
}

pub fn f605(v0: List<Adt2>) -> List<Adt6> {
    f299(f669(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f606(v0: Str, v1: Adt5) -> Option<Adt5> {
    let m2400 = f177(v0, v1);
    match m2400 {
        Adt135::C0 => {
            None::<Adt5>
        }
        Adt135::C1(v2) => {
            f254(v2)
        }
    }
}

pub fn f607(v0: Adt5) -> Option<Adt63> {
    let m2401 = f670(v0.clone());
    match m2401 {
        None => {
            None::<Adt63>
        }
        Some(v1) => {
            let m2402 = f671(Str::lit("status"), v0.clone());
            match m2402 {
                None => {
                    None::<Adt63>
                }
                Some(v2) => {
                    let a2403 = v2.clone();
                    let a2404 = Str::lit("working");
                    if equal(a2403, a2404) {
                        Some(Adt63::C0(v1))
                    } else {
                        let a2405 = v2.clone();
                        let a2406 = Str::lit("cancelled");
                        if equal(a2405, a2406) {
                            Some(Adt63::C4(v1))
                        } else {
                            let a2407 = v2.clone();
                            let a2408 = Str::lit("input_required");
                            if equal(a2407, a2408) {
                                let m2409 = f672(Str::lit("inputRequests"), v0);
                                match m2409 {
                                    None => {
                                        None::<Adt63>
                                    }
                                    Some(v3) => {
                                        Some(Adt63::C1(v1, v3))
                                    }
                                }
                            } else {
                                let a2410 = v2.clone();
                                let a2411 = Str::lit("completed");
                                if equal(a2410, a2411) {
                                    let m2412 = f606(Str::lit("result"), v0);
                                    match m2412 {
                                        None => {
                                            None::<Adt63>
                                        }
                                        Some(v4) => {
                                            Some(Adt63::C2(v1, v4))
                                        }
                                    }
                                } else {
                                    let a2413 = v2;
                                    let a2414 = Str::lit("failed");
                                    if equal(a2413, a2414) {
                                        let m2415 = f606(Str::lit("error"), v0);
                                        match m2415 {
                                            None => {
                                                None::<Adt63>
                                            }
                                            Some(v5) => {
                                                let m2416 = f673(v5);
                                                match m2416 {
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
    let m2417 = f177(v0, v1);
    match m2417 {
        Adt135::C0 => {
            None::<i64>
        }
        Adt135::C1(v2) => {
            f638(v2)
        }
    }
}

pub fn f609(v0: Adt5) -> Option<Adt86> {
    let m2418 = f608(Str::lit("parkAfterMs"), v0.clone());
    match m2418 {
        None => {
            None::<Adt86>
        }
        Some(v1) => {
            let m2419 = f672(Str::lit("parked"), v0.clone());
            match m2419 {
                None => {
                    None::<Adt86>
                }
                Some(v2) => {
                    let m2420 = f608(Str::lit("finishAfterMs"), v0.clone());
                    match m2420 {
                        None => {
                            None::<Adt86>
                        }
                        Some(v3) => {
                            let m2421 = f606(Str::lit("outcome"), v0);
                            match m2421 {
                                None => {
                                    None::<Adt86>
                                }
                                Some(v4) => {
                                    let m2422 = f674(v4);
                                    match m2422 {
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
    let m2424 = f352(match v0.clone() {
        Adt85::C0(h2423, _, _) => {
            h2423
        }
    });
    match m2424 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            let m2426 = f675(match v0.clone() {
                Adt85::C0(_, _, h2425) => {
                    h2425
                }
            });
            match m2426 {
                None => {
                    None::<Adt6>
                }
                Some(v2) => {
                    Some(Adt6::C5(List::cons((Str::lit("task"), Adt6::C5(v1)), List::cons((Str::lit("createdMs"), f604(match v0 {
                        Adt85::C0(_, h2427, _) => {
                            h2427
                        }
                    })), List::cons((Str::lit("plan"), v2), List::<(Str, Adt6)>::nil())))))
                }
            }
        }
    }
}

pub fn f611(v0: List<u64>) -> List<u64> {
    let a2430 = f676();
    let a2431 = {
        let a2428 = v0;
        let a2429 = f677();
        append_list(a2428, a2429)
    };
    append_list(a2430, a2431)
}

pub fn f612(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f678(v0, 0u64, v1)
}

pub fn f613(p2438: List<u64>) -> bool {
    let m2437 = p2438;
    {
        let mut m2437 = m2437;
        loop {
            let step = {
                let v0 = m2437;
                let m2432 = v0;
                match m2432.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2433 = v1;
                        let a2434 = 48u64;
                        if if nat_eq(a2433, a2434) {
                            true
                        } else {
                            let a2435 = v1;
                            let a2436 = 46u64;
                            nat_eq(a2435, a2436)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f614(v0: Adt140) -> bool {
    let m2439 = v0;
    match m2439 {
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
    let m2440 = v0;
    match m2440 {
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
        Adt0::C0(h2441, _, _, _, _, _, _) => {
            h2441
        }
    }), None::<Adt5>), List::<Adt130>::nil())
}

pub fn f617(v0: Adt0) -> Adt34 {
    Adt34::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2442, _, _, _) => {
            h2442
        }
    }, f175(), match v0.clone() {
        Adt0::C0(_, _, h2443, _, _, _, _) => {
            h2443
        }
    }, match v0 {
        Adt0::C0(_, h2444, _, _, _, _, _) => {
            h2444
        }
    })
}

pub fn f618(v0: Adt0, v1: Adt15, v2: Result<Adt33, Adt90>) -> Result<Adt128, Adt131> {
    let m2445 = v2;
    match m2445 {
        Ok(v4) => {
            if f679(f615(v1.clone()), v4.clone()) {
                f680(v0, v1, v4)
            } else {
                Err::<Adt128, Adt131>(Adt131::C4(f681()))
            }
        }
        Err(v3) => {
            Err::<Adt128, Adt131>(Adt131::C4(v3))
        }
    }
}

pub fn f619(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt15) -> R<Result<Adt33, Adt90>> {
    f682(v0.clone(), v1, v2, v3, v4.clone(), f683(v0, v4)?)
}

pub fn f620(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt132> {
    let m2446 = f177(v0.clone(), v1);
    match m2446 {
        Adt135::C0 => {
            Ok::<Option<Str>, Adt132>(None::<Str>)
        }
        Adt135::C1(v2) => {
            let m2447 = f228(v2);
            match m2447 {
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
    let m2448 = f177(v0.clone(), v1);
    match m2448 {
        Adt135::C0 => {
            Err::<Str, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2449 = f228(v2);
            match m2449 {
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
    let m2450 = f177(v0.clone(), v1);
    match m2450 {
        Adt135::C0 => {
            Ok::<Option<Adt5>, Adt132>(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2451 = f254(v2);
            match m2451 {
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
    let m2452 = f177(v0.clone(), v1);
    match m2452 {
        Adt135::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt132>(None::<List<(Str, Str)>>)
        }
        Adt135::C1(v2) => {
            let m2453 = f254(v2);
            match m2453 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt132>(Adt132::C8(v0))
                }
                Some(v3) => {
                    let m2455 = f684(v0, match v3 {
                        Adt5::C0(h2454) => {
                            h2454
                        }
                    });
                    match m2455 {
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
    let m2456 = f177(v0.clone(), v1);
    match m2456 {
        Adt135::C0 => {
            Err::<Adt5, Adt132>(Adt132::C7(v0))
        }
        Adt135::C1(v2) => {
            let m2457 = f254(v2);
            match m2457 {
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
    let m2458 = f621(Str::lit("type"), v0.clone());
    match m2458 {
        Ok(v1) => {
            let a2459 = v1.clone();
            let a2460 = Str::lit("ref/prompt");
            if equal(a2459, a2460) {
                let m2461 = f621(Str::lit("name"), v0.clone());
                match m2461 {
                    Ok(v2) => {
                        let m2462 = f620(Str::lit("title"), v0);
                        match m2462 {
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
                let a2463 = v1;
                let a2464 = Str::lit("ref/resource");
                if equal(a2463, a2464) {
                    let m2465 = f621(Str::lit("uri"), v0);
                    match m2465 {
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
    let m2466 = f621(Str::lit("name"), v0.clone());
    match m2466 {
        Ok(v1) => {
            let m2467 = f621(Str::lit("value"), v0);
            match m2467 {
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
    let m2468 = f177(Str::lit("context"), v0);
    match m2468 {
        Adt135::C0 => {
            Ok::<Option<Adt29>, Adt132>(None::<Adt29>)
        }
        Adt135::C1(v1) => {
            let m2469 = f254(v1);
            match m2469 {
                None => {
                    Err::<Option<Adt29>, Adt132>(Adt132::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2470 = f623(Str::lit("arguments"), v2);
                    match m2470 {
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

pub fn f629(v0: Str, p2475: List<Str>) -> bool {
    let m2474 = p2475;
    {
        let mut m2474 = m2474;
        loop {
            let step = {
                let v1 = m2474;
                let m2471 = v1;
                match m2471.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2472 = v2;
                        let a2473 = v0.clone();
                        if equal(a2472, a2473) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f630(v0: Option<Adt109>, v1: List<Adt111>, v2: Adt5) -> R<Option<Adt109>> {
    let m2476 = v0;
    match m2476 {
        None => {
            f685(v1, f686(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f631(v0: Option<Str>, v1: Adt146, v2: Adt5) -> Option<Adt109> {
    let m2477 = v1.clone();
    match m2477 {
        Adt146::C0 => {
            None::<Adt109>
        }
        Adt146::C1 => {
            f687(v0, f688(v1, v2))
        }
        Adt146::C2 => {
            f687(v0, f688(v1, v2))
        }
        Adt146::C3 => {
            f687(v0, f688(v1, v2))
        }
    }
}

pub fn f632(v0: Adt135, v1: Adt135) -> Adt153 {
    let m2478 = v0;
    match m2478 {
        Adt135::C0 => {
            Adt153::C1
        }
        Adt135::C1(_) => {
            let m2479 = v1;
            match m2479 {
                Adt135::C0 => {
                    Adt153::C2
                }
                Adt135::C1(_) => {
                    Adt153::C0
                }
            }
        }
    }
}

pub fn f633(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt17, Adt132> {
    let m2480 = f228(v1);
    match m2480 {
        None => {
            Err::<Adt17, Adt132>(Adt132::C5)
        }
        Some(v3) => {
            let m2481 = f689(v2);
            match m2481 {
                None => {
                    Err::<Adt17, Adt132>(Adt132::C6)
                }
                Some(v4) => {
                    let m2482 = f622(f690(), v0.clone());
                    match m2482 {
                        Ok(v6) => {
                            let m2483 = f691(v0.clone());
                            match m2483 {
                                Ok(v8) => {
                                    let m2484 = f692(v0.clone());
                                    match m2484 {
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
    let a2485 = v0;
    let a2486 = v1;
    let a2487 = 0u64;
    nat_rem(a2485, a2486, a2487)
}

pub fn f635(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2490 = {
            let a2488 = v0;
            let a2489 = 4u64;
            nat_mul(a2488, a2489)?
        };
        let a2491 = f637(v1, 16u64);
        nat_add(a2490, a2491)?
    }, List::<u64>::nil()))
}

pub fn f636(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2494 = {
            let a2492 = v0;
            let a2493 = 4u64;
            nat_mul(a2492, a2493)?
        };
        let a2495 = f637(v1, 16u64);
        nat_add(a2494, a2495)?
    }, List::cons({
        let a2498 = {
            let a2496 = f634(v1, 16u64);
            let a2497 = 16u64;
            nat_mul(a2496, a2497)?
        };
        let a2499 = f637(v2, 4u64);
        nat_add(a2498, a2499)?
    }, List::<u64>::nil())))
}

pub fn f637(v0: u64, v1: u64) -> u64 {
    let a2500 = v0;
    let a2501 = v1;
    let a2502 = 0u64;
    nat_quot(a2500, a2501, a2502)
}

pub fn f638(v0: Adt6) -> Option<i64> {
    let m2503 = v0;
    match m2503 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f390(match v2.clone() {
                Adt7::C0(_, h2504) => {
                    h2504
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2505, _) => {
                        h2505
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

pub fn f639(p2509: List<Adt6>) -> bool {
    let m2508 = p2509;
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let v0 = m2508;
                let m2506 = v0;
                match m2506.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2507 = f228(v1);
                        match m2507 {
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
                Ok(next) => { m2508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(p2513: List<Adt6>) -> bool {
    let m2512 = p2513;
    {
        let mut m2512 = m2512;
        loop {
            let step = {
                let v0 = m2512;
                let m2510 = v0;
                match m2510.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2511 = f228(v1);
                        match m2511 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f569(v3) {
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
                Ok(next) => { m2512 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f641(v0: Str) -> Option<Adt13> {
    let a2514 = v0.clone();
    let a2515 = Str::lit("string");
    if equal(a2514, a2515) {
        Some(Adt13::C0)
    } else {
        let a2516 = v0.clone();
        let a2517 = Str::lit("integer");
        if equal(a2516, a2517) {
            Some(Adt13::C1)
        } else {
            let a2518 = v0;
            let a2519 = Str::lit("boolean");
            if equal(a2518, a2519) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f642(p2528: List<u64>, p2529: List<u64>) -> R<List<u64>> {
    let m2527 = (p2528, p2529);
    {
        let mut m2527 = m2527;
        loop {
            let step = {
                let (v0, v1) = m2527;
                let m2520 = v0;
                match m2520.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2521 = 65u64;
                        let a2522 = v2;
                        if if nat_le(a2521, a2522) {
                            let a2523 = v2;
                            let a2524 = 90u64;
                            nat_le(a2523, a2524)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2525 = v2;
                                let a2526 = 32u64;
                                nat_add(a2525, a2526)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f643(p2536: List<u64>, p2537: List<u64>) -> bool {
    let m2535 = (p2536, p2537);
    {
        let mut m2535 = m2535;
        loop {
            let step = {
                let (v0, v1) = m2535;
                let m2530 = v0;
                match m2530.uncons() {
                    None => {
                        let m2531 = v1;
                        match m2531.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2532 = v1;
                        match m2532.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2533 = v4;
                                let a2534 = v6;
                                if nat_eq(a2533, a2534) {
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
                Ok(next) => { m2535 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(v0: Adt6) -> Str {
    let m2538 = v0;
    match m2538 {
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

pub fn f645(v0: Adt7) -> R<bool> {
    if f390(match v0.clone() {
        Adt7::C0(h2539, _) => {
            h2539
        }
    }) {
        Ok(true)
    } else {
        f693(40u64, match v0.clone() {
            Adt7::C0(h2540, _) => {
                h2540
            }
        }, match v0 {
            Adt7::C0(_, h2541) => {
                h2541
            }
        })
    }
}

pub fn f646(v0: Adt6) -> Str {
    let m2542 = f228(v0);
    match m2542 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f647(p2549: u64, p2550: List<(Adt6, Adt6)>) -> R<Adt154> {
    let m2548 = (p2549, p2550);
    {
        let mut m2548 = m2548;
        loop {
            let step = {
                let (v0, v1) = m2548;
                let m2543 = v0;
                if m2543 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt154>>(Ok(Adt154::C2))
                } else {
                    let v2 = m2543 - 1;
                    let m2544 = v1;
                    match m2544.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt154>>(Ok(Adt154::C0))
                        }
                        Some((v3, v4)) => {
                            let m2547 = f694({
                                let (h2545, _) = v3;
                                h2545
                            }, {
                                let (_, h2546) = v3;
                                h2546
                            }, v4)?;
                            match m2547 {
                                Adt157::C0 => {
                                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt154>>(Ok(Adt154::C1))
                                }
                                Adt157::C1(v5) => {
                                    Ok::<(u64, List<(Adt6, Adt6)>), R<Adt154>>((v2, v5))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648() -> u64 {
    67108864u64
}

pub fn f649(v0: Adt155) -> bool {
    let m2551 = v0;
    match m2551 {
        Adt155::C0 => {
            true
        }
        Adt155::C1 => {
            false
        }
        Adt155::C2 => {
            false
        }
    }
}

pub fn f650(v0: Adt7, v1: Adt7, v2: Adt155, v3: Adt155) -> R<Adt155> {
    let m2552 = v2;
    match m2552 {
        Adt155::C0 => {
            let m2554 = v3;
            match m2554 {
                Adt155::C0 => {
                    Ok(f695(Adt155::C0, f696(f697(match v0.clone() {
                        Adt7::C0(h2555, _) => {
                            h2555
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2556) => {
                            h2556
                        }
                    }, f697(match v1.clone() {
                        Adt7::C0(h2557, _) => {
                            h2557
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2558) => {
                            h2558
                        }
                    })?))
                }
                Adt155::C1 => {
                    Ok(Adt155::C0)
                }
                Adt155::C2 => {
                    Ok(Adt155::C0)
                }
            }
        }
        Adt155::C1 => {
            let m2553 = v3;
            match m2553 {
                Adt155::C0 => {
                    Ok(Adt155::C2)
                }
                Adt155::C1 => {
                    Ok(Adt155::C1)
                }
                Adt155::C2 => {
                    Ok(Adt155::C0)
                }
            }
        }
        Adt155::C2 => {
            let m2559 = v3;
            match m2559 {
                Adt155::C0 => {
                    Ok(Adt155::C2)
                }
                Adt155::C1 => {
                    Ok(Adt155::C2)
                }
                Adt155::C2 => {
                    f696(match v0.clone() {
                        Adt7::C0(h2560, _) => {
                            h2560
                        }
                    }, match v0 {
                        Adt7::C0(_, h2561) => {
                            h2561
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2562, _) => {
                            h2562
                        }
                    }, match v1 {
                        Adt7::C0(_, h2563) => {
                            h2563
                        }
                    })
                }
            }
        }
    }
}

pub fn f651(v0: i64) -> Adt155 {
    f652(v0, 0i64)
}

pub fn f652(v0: i64, v1: i64) -> Adt155 {
    let m2566 = {
        let a2564 = v0;
        let a2565 = v1;
        compare(a2564, a2565)
    };
    if match m2566 {
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
        Adt155::C0
    } else {
        let m2569 = {
            let a2567 = v1;
            let a2568 = v0;
            compare(a2567, a2568)
        };
        if match m2569 {
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
            Adt155::C2
        } else {
            Adt155::C1
        }
    }
}

pub fn f653(p2578: List<u64>, p2579: u64) -> R<u64> {
    let m2577 = (p2578, p2579);
    {
        let mut m2577 = m2577;
        loop {
            let step = {
                let (v0, v1) = m2577;
                let m2570 = v0;
                match m2570.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2571 = v2;
                        let a2572 = 128u64;
                        if if nat_lt(a2571, a2572) {
                            true
                        } else {
                            let a2573 = 191u64;
                            let a2574 = v2;
                            nat_lt(a2573, a2574)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2575 = v1;
                                let a2576 = 1u64;
                                nat_add(a2575, a2576)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2577 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654(v0: Option<Adt37>) -> List<(Str, Adt6)> {
    let m2580 = v0;
    match m2580 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2592 = f416(Str::lit("title"), match v1.clone() {
                    Adt37::C0(h2581, _, _, _, _) => {
                        h2581
                    }
                });
                let a2593 = {
                    let a2590 = f421(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt37::C0(_, h2582, _, _, _) => {
                            h2582
                        }
                    });
                    let a2591 = {
                        let a2588 = f421(Str::lit("destructiveHint"), match v1.clone() {
                            Adt37::C0(_, _, h2583, _, _) => {
                                h2583
                            }
                        });
                        let a2589 = {
                            let a2586 = f421(Str::lit("idempotentHint"), match v1.clone() {
                                Adt37::C0(_, _, _, h2584, _) => {
                                    h2584
                                }
                            });
                            let a2587 = f421(Str::lit("openWorldHint"), match v1 {
                                Adt37::C0(_, _, _, _, h2585) => {
                                    h2585
                                }
                            });
                            append_list(a2586, a2587)
                        };
                        append_list(a2588, a2589)
                    };
                    append_list(a2590, a2591)
                };
                append_list(a2592, a2593)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f655(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f457(Str::lit("_meta"), v0)
}

pub fn f656(v0: Adt40) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt40::C0(h2594, _, _) => {
            h2594
        }
    })), {
        let a2597 = f660(match v0.clone() {
            Adt40::C0(_, h2595, _) => {
                h2595
            }
        });
        let a2598 = f655(match v0 {
            Adt40::C0(_, _, h2596) => {
                h2596
            }
        });
        append_list(a2597, a2598)
    })))
}

pub fn f657(v0: Str, v1: Str, v2: Str, v3: Option<Adt41>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2599 = f660(v3);
        let a2600 = f655(v4);
        append_list(a2599, a2600)
    }))))
}

pub fn f658(v0: Adt45) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2624 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt45::C0(h2601, _, _, _, _, _, _, _, _) => {
                h2601
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2625 = {
            let a2622 = f416(Str::lit("title"), match v0.clone() {
                Adt45::C0(_, h2602, _, _, _, _, _, _, _) => {
                    h2602
                }
            });
            let a2623 = {
                let a2620 = f518(match v0.clone() {
                    Adt45::C0(_, _, h2603, _, _, _, _, _, _) => {
                        h2603
                    }
                });
                let a2621 = {
                    let a2618 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt45::C0(_, _, _, h2604, _, _, _, _, _) => {
                            h2604
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2619 = {
                        let a2616 = f416(Str::lit("description"), match v0.clone() {
                            Adt45::C0(_, _, _, _, h2605, _, _, _, _) => {
                                h2605
                            }
                        });
                        let a2617 = {
                            let a2614 = f416(Str::lit("mimeType"), match v0.clone() {
                                Adt45::C0(_, _, _, _, _, h2606, _, _, _) => {
                                    h2606
                                }
                            });
                            let a2615 = {
                                let a2612 = f660(match v0.clone() {
                                    Adt45::C0(_, _, _, _, _, _, h2607, _, _) => {
                                        h2607
                                    }
                                });
                                let a2613 = {
                                    let a2610 = f661(match v0.clone() {
                                        Adt45::C0(_, _, _, _, _, _, _, h2608, _) => {
                                            h2608
                                        }
                                    })?;
                                    let a2611 = f655(match v0 {
                                        Adt45::C0(_, _, _, _, _, _, _, _, h2609) => {
                                            h2609
                                        }
                                    });
                                    append_list(a2610, a2611)
                                };
                                append_list(a2612, a2613)
                            };
                            append_list(a2614, a2615)
                        };
                        append_list(a2616, a2617)
                    };
                    append_list(a2618, a2619)
                };
                append_list(a2620, a2621)
            };
            append_list(a2622, a2623)
        };
        append_list(a2624, a2625)
    }))
}

pub fn f659(v0: Adt46) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f596(match v0.clone() {
        Adt46::C0(h2626, _, _) => {
            h2626
        }
    })), {
        let a2629 = f660(match v0.clone() {
            Adt46::C0(_, h2627, _) => {
                h2627
            }
        });
        let a2630 = f655(match v0 {
            Adt46::C0(_, _, h2628) => {
                h2628
            }
        });
        append_list(a2629, a2630)
    })))
}

pub fn f660(v0: Option<Adt41>) -> List<(Str, Adt6)> {
    let m2631 = v0;
    match m2631 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2637 = f698(match v1.clone() {
                    Adt41::C0(h2632, _, _) => {
                        h2632
                    }
                });
                let a2638 = {
                    let a2635 = f699(match v1.clone() {
                        Adt41::C0(_, h2633, _) => {
                            h2633
                        }
                    });
                    let a2636 = f416(Str::lit("lastModified"), match v1 {
                        Adt41::C0(_, _, h2634) => {
                            h2634
                        }
                    });
                    append_list(a2635, a2636)
                };
                append_list(a2637, a2638)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f661(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2639 = v0;
    match m2639 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f504(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f662(v0: List<Adt58>) -> List<Adt6> {
    f299(f700(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f663(v0: Adt42) -> Str {
    let m2640 = v0;
    match m2640 {
        Adt42::C0 => {
            Str::lit("user")
        }
        Adt42::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f664(v0: Adt67) -> Adt6 {
    Adt6::C5({
        let a2647 = {
            let m2642 = match v0.clone() {
                Adt67::C0(h2641, _, _) => {
                    h2641
                }
            };
            match m2642 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2648 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt67::C0(_, h2643, _) => {
                h2643
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2646 = match v0 {
                Adt67::C0(_, _, h2645) => {
                    h2645
                }
            };
            match m2646 {
                Adt5::C0(h2644) => {
                    h2644
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2647, a2648)
    })
}

pub fn f665(v0: List<Adt71>) -> R<Option<List<Adt6>>> {
    let m2649 = f701(v0, List::<Adt6>::nil())?;
    match m2649 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f299(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f666(v0: Adt70) -> List<(Str, Adt6)> {
    let a2660 = {
        let m2651 = match v0.clone() {
            Adt70::C0(_, h2650, _, _, _, _, _, _, _, _) => {
                h2650
            }
        };
        match m2651 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f702(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2661 = {
        let a2658 = f416(Str::lit("systemPrompt"), match v0.clone() {
            Adt70::C0(_, _, h2652, _, _, _, _, _, _, _) => {
                h2652
            }
        });
        let a2659 = {
            let a2656 = {
                let m2654 = match v0.clone() {
                    Adt70::C0(_, _, _, h2653, _, _, _, _, _, _) => {
                        h2653
                    }
                };
                match m2654 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f703(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2657 = f704(Str::lit("temperature"), match v0 {
                Adt70::C0(_, _, _, _, h2655, _, _, _, _, _) => {
                    h2655
                }
            });
            append_list(a2656, a2657)
        };
        append_list(a2658, a2659)
    };
    append_list(a2660, a2661)
}

pub fn f667(v0: Adt70) -> List<(Str, Adt6)> {
    let a2673 = {
        let m2663 = match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, h2662, _, _, _) => {
                h2662
            }
        };
        match m2663 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f414(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2674 = {
        let a2671 = f457(Str::lit("metadata"), match v0.clone() {
            Adt70::C0(_, _, _, _, _, _, _, h2664, _, _) => {
                h2664
            }
        });
        let a2672 = {
            let a2669 = {
                let m2666 = match v0.clone() {
                    Adt70::C0(_, _, _, _, _, _, _, _, h2665, _) => {
                        h2665
                    }
                };
                match m2666 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f418(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2670 = {
                let m2668 = match v0 {
                    Adt70::C0(_, _, _, _, _, _, _, _, _, h2667) => {
                        h2667
                    }
                };
                match m2668 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f705(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2669, a2670)
        };
        append_list(a2671, a2672)
    };
    append_list(a2673, a2674)
}

pub fn f668(v0: Adt65) -> Option<Adt6> {
    let m2675 = v0;
    match m2675 {
        Adt65::C0(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("elicitation/create"))), List::cons((Str::lit("params"), Adt6::C5(f706(v1))), List::<(Str, Adt6)>::nil()))))
        }
        Adt65::C1(_) => {
            None::<Adt6>
        }
        Adt65::C2(v3) => {
            Some(Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(Str::lit("roots/list"))), {
                let m2676 = v3;
                match m2676 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v4) => {
                        List::cons((Str::lit("params"), Adt6::C5(match v4 {
                            Adt5::C0(h2677) => {
                                h2677
                            }
                        })), List::<(Str, Adt6)>::nil())
                    }
                }
            })))
        }
    }
}

pub fn f669(p2680: List<Adt2>, p2681: List<Adt6>) -> List<Adt6> {
    let m2679 = (p2680, p2681);
    {
        let mut m2679 = m2679;
        loop {
            let step = {
                let (v0, v1) = m2679;
                let m2678 = v0;
                match m2678.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f707(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2679 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f670(v0: Adt5) -> Option<Adt64> {
    let m2682 = f671(Str::lit("taskId"), v0.clone());
    match m2682 {
        None => {
            None::<Adt64>
        }
        Some(v1) => {
            let m2683 = f671(Str::lit("createdAt"), v0.clone());
            match m2683 {
                None => {
                    None::<Adt64>
                }
                Some(v2) => {
                    let m2684 = f671(Str::lit("lastUpdatedAt"), v0.clone());
                    match m2684 {
                        None => {
                            None::<Adt64>
                        }
                        Some(v3) => {
                            Some(Adt64::C0(v1, f671(Str::lit("statusMessage"), v0.clone()), v2, v3, f608(Str::lit("ttlMs"), v0.clone()), f608(Str::lit("pollIntervalMs"), v0)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f671(v0: Str, v1: Adt5) -> Option<Str> {
    let m2685 = f177(v0, v1);
    match m2685 {
        Adt135::C0 => {
            None::<Str>
        }
        Adt135::C1(v2) => {
            f228(v2)
        }
    }
}

pub fn f672(v0: Str, v1: Adt5) -> Option<List<(Str, Adt65)>> {
    let m2686 = f606(v0, v1);
    match m2686 {
        None => {
            None::<List<(Str, Adt65)>>
        }
        Some(v2) => {
            f708(match v2 {
                Adt5::C0(h2687) => {
                    h2687
                }
            }, List::<(Str, Adt65)>::nil())
        }
    }
}

pub fn f673(v0: Adt5) -> Option<Adt81> {
    let m2688 = f608(Str::lit("code"), v0.clone());
    match m2688 {
        None => {
            None::<Adt81>
        }
        Some(v1) => {
            let m2689 = f671(Str::lit("message"), v0.clone());
            match m2689 {
                None => {
                    None::<Adt81>
                }
                Some(v2) => {
                    Some(Adt81::C0(v1, v2, {
                        let m2690 = f177(Str::lit("data"), v0);
                        match m2690 {
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

pub fn f674(v0: Adt5) -> Option<Adt87> {
    let m2691 = f606(Str::lit("completes"), v0.clone());
    match m2691 {
        None => {
            let m2692 = f606(Str::lit("fails"), v0);
            match m2692 {
                None => {
                    None::<Adt87>
                }
                Some(v2) => {
                    let m2693 = f673(v2);
                    match m2693 {
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

pub fn f675(v0: Adt86) -> Option<Adt6> {
    let m2695 = f514(match v0.clone() {
        Adt86::C0(_, h2694, _, _) => {
            h2694
        }
    });
    match m2695 {
        None => {
            None::<Adt6>
        }
        Some(v1) => {
            Some(Adt6::C5(List::cons((Str::lit("parkAfterMs"), f604(match v0.clone() {
                Adt86::C0(h2696, _, _, _) => {
                    h2696
                }
            })), List::cons((Str::lit("parked"), v1), List::cons((Str::lit("finishAfterMs"), f604(match v0.clone() {
                Adt86::C0(_, _, h2697, _) => {
                    h2697
                }
            })), List::cons((Str::lit("outcome"), f709(match v0 {
                Adt86::C0(_, _, _, h2698) => {
                    h2698
                }
            })), List::<(Str, Adt6)>::nil()))))))
        }
    }
}

pub fn f676() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f677() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f678(p2703: List<u64>, p2704: u64, p2705: List<u64>) -> List<u64> {
    let m2702 = (p2703, (p2704, p2705));
    {
        let mut m2702 = m2702;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2702;
                let m2699 = v0;
                match m2699.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f710(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt156 = f711(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt156::C0(h2700, _) => {
                                h2700
                            }
                        }, match v5 {
                            Adt156::C0(_, h2701) => {
                                h2701
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f679(v0: Adt140, v1: Adt33) -> bool {
    let m2706 = v1;
    match m2706 {
        Adt33::C0(_) => {
            f712(v0, Adt140::C0)
        }
        Adt33::C1(_) => {
            f712(v0, Adt140::C1)
        }
        Adt33::C2(_) => {
            f712(v0, Adt140::C2)
        }
        Adt33::C3(_) => {
            f712(v0, Adt140::C2)
        }
        Adt33::C4(_) => {
            f712(v0, Adt140::C3)
        }
        Adt33::C5(_) => {
            f712(v0, Adt140::C4)
        }
        Adt33::C6(_) => {
            f712(v0, Adt140::C5)
        }
        Adt33::C7(_) => {
            f712(v0, Adt140::C6)
        }
        Adt33::C8(_) => {
            f712(v0, Adt140::C7)
        }
        Adt33::C9(_) => {
            f712(v0, Adt140::C8)
        }
        Adt33::C10(_) => {
            f712(v0, Adt140::C9)
        }
        Adt33::C11 => {
            if f712(v0.clone(), Adt140::C10) {
                true
            } else {
                f712(v0, Adt140::C11)
            }
        }
        Adt33::C12(_) => {
            f713(v0)
        }
        Adt33::C13(_) => {
            f714(v0)
        }
        Adt33::C14(v15) => {
            let m2708 = match v15 {
                Adt88::C0(_, h2707, _) => {
                    h2707
                }
            };
            match m2708 {
                Adt89::C0 => {
                    f712(v0, Adt140::C9)
                }
                Adt89::C1 => {
                    if f712(v0.clone(), Adt140::C10) {
                        true
                    } else {
                        f712(v0, Adt140::C11)
                    }
                }
            }
        }
    }
}

pub fn f680(v0: Adt0, v1: Adt15, v2: Adt33) -> Result<Adt128, Adt131> {
    let m2709 = v2.clone();
    match m2709 {
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
            Ok::<Adt128, Adt131>(f719(v0, f716(v1), v9))
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
            f715(v0, f716(v1), v3)
        }
        Adt33::C13(v4) => {
            f717(v0, f716(v1), v4)
        }
        Adt33::C14(v5) => {
            f718(v0, f716(v1), v5)
        }
    }
}

pub fn f681() -> Adt90 {
    Adt90::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f682(v0: Bytes, v1: u64, v2: u64, v3: Fn2, v4: Adt15, v5: Result<Adt15, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m2710 = v5;
    match m2710 {
        Ok(v7) => {
            f720(v0, v4, f721(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt33, Adt90>(v6))
        }
    }
}

pub fn f683(v0: Bytes, v1: Adt15) -> R<Result<Adt15, Adt90>> {
    let m2711 = v1.clone();
    match m2711 {
        Adt15::C0(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C1(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C2(v2) => {
            f722(v0, v2)
        }
        Adt15::C3(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C4(v3) => {
            f723(v0, v3)
        }
        Adt15::C5(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C6(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C7(v4) => {
            f724(v0, v4)
        }
        Adt15::C8(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C9(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C10(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
        Adt15::C11(_) => {
            Ok(Ok::<Adt15, Adt90>(v1))
        }
    }
}

pub fn f684(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2712 = f725(v0, v1, List::<(Str, Str)>::nil());
    match m2712 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt132>(f726(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt132>(v3)
        }
    }
}

pub fn f685(p2716: List<Adt111>, v1: Adt5) -> R<Option<Adt109>> {
    let m2715 = p2716;
    {
        let mut m2715 = m2715;
        loop {
            let step = {
                let v0 = m2715;
                let m2713 = v0;
                match m2713.uncons() {
                    None => {
                        Err::<List<Adt111>, R<Option<Adt109>>>(Ok(None::<Adt109>))
                    }
                    Some((v2, v3)) => {
                        let m2714 = f727(v2, v1.clone())?;
                        match m2714 {
                            None => {
                                Ok::<List<Adt111>, R<Option<Adt109>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt111>, R<Option<Adt109>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt5) -> Adt5 {
    let m2717 = f177(Str::lit("arguments"), v0);
    match m2717 {
        Adt135::C0 => {
            f546()
        }
        Adt135::C1(v1) => {
            let m2718 = f254(v1);
            match m2718 {
                None => {
                    f546()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f687(v0: Option<Str>, v1: Option<Str>) -> Option<Adt109> {
    let m2719 = v0;
    match m2719 {
        None => {
            Some(Adt109::C2)
        }
        Some(v2) => {
            let m2720 = v1;
            match m2720 {
                None => {
                    None::<Adt109>
                }
                Some(v3) => {
                    let a2721 = v2;
                    let a2722 = v3;
                    if equal(a2721, a2722) {
                        None::<Adt109>
                    } else {
                        Some(Adt109::C5)
                    }
                }
            }
        }
    }
}

pub fn f688(v0: Adt146, v1: Adt5) -> Option<Str> {
    let m2723 = v0;
    match m2723 {
        Adt146::C0 => {
            None::<Str>
        }
        Adt146::C1 => {
            let m2724 = f177(Str::lit("name"), v1);
            match m2724 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v2) => {
                    f228(v2)
                }
            }
        }
        Adt146::C2 => {
            let m2725 = f177(Str::lit("uri"), v1);
            match m2725 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v3) => {
                    f228(v3)
                }
            }
        }
        Adt146::C3 => {
            let m2726 = f177(Str::lit("taskId"), v1);
            match m2726 {
                Adt135::C0 => {
                    None::<Str>
                }
                Adt135::C1(v4) => {
                    f228(v4)
                }
            }
        }
    }
}

pub fn f689(v0: Adt6) -> Option<Adt18> {
    let m2727 = f254(v0);
    match m2727 {
        None => {
            None::<Adt18>
        }
        Some(v1) => {
            let m2728 = f728(Str::lit("experimental"), v1.clone());
            match m2728 {
                None => {
                    None::<Adt18>
                }
                Some(v2) => {
                    let m2729 = f728(Str::lit("roots"), v1.clone());
                    match m2729 {
                        None => {
                            None::<Adt18>
                        }
                        Some(v3) => {
                            let m2730 = f728(Str::lit("sampling"), v1.clone());
                            match m2730 {
                                None => {
                                    None::<Adt18>
                                }
                                Some(v4) => {
                                    let m2731 = f728(Str::lit("elicitation"), v1.clone());
                                    match m2731 {
                                        None => {
                                            None::<Adt18>
                                        }
                                        Some(v5) => {
                                            let m2732 = f728(Str::lit("extensions"), v1);
                                            match m2732 {
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

pub fn f690() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f691(v0: Adt5) -> Result<Option<Adt19>, Adt132> {
    let m2733 = f620(f729(), v0);
    match m2733 {
        Ok(v2) => {
            let m2734 = v2;
            match m2734 {
                None => {
                    Ok::<Option<Adt19>, Adt132>(None::<Adt19>)
                }
                Some(v3) => {
                    let m2735 = f730(v3);
                    match m2735 {
                        Adt158::C0 => {
                            Err::<Option<Adt19>, Adt132>(Adt132::C8(f729()))
                        }
                        Adt158::C1(v4) => {
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

pub fn f692(v0: Adt5) -> Result<Option<Adt20>, Adt132> {
    let m2736 = f177(f731(), v0);
    match m2736 {
        Adt135::C0 => {
            Ok::<Option<Adt20>, Adt132>(None::<Adt20>)
        }
        Adt135::C1(v1) => {
            let m2737 = f228(v1.clone());
            match m2737 {
                None => {
                    let m2738 = f638(v1);
                    match m2738 {
                        None => {
                            Err::<Option<Adt20>, Adt132>(Adt132::C8(f731()))
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

pub fn f693(p2750: u64, p2751: i64, p2752: i64) -> R<bool> {
    let m2749 = (p2750, (p2751, p2752));
    {
        let mut m2749 = m2749;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2749;
                let m2739 = v0;
                if m2739 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2739 - 1;
                    let m2742 = {
                        let a2740 = v2;
                        let a2741 = 0i64;
                        compare(a2740, a2741)
                    };
                    if match m2742 {
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
                        if f390({
                            let a2743 = v1;
                            let a2744 = 10i64;
                            let a2745 = 0i64;
                            int_rem(a2743, a2744, a2745)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2746 = v1;
                                let a2747 = 10i64;
                                let a2748 = 0i64;
                                int_quot(a2746, a2747, a2748)?
                            }, f116(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f694(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt157> {
    let a2753 = f732(v0.clone());
    let a2754 = f732(v1.clone());
    if nat_eq(a2753, a2754) {
        let m2755 = v0;
        match m2755 {
            Adt6::C0 => {
                Ok(Adt157::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2756 = f253(v1);
                match m2756 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v4) => {
                        let a2757 = v3;
                        let a2758 = v4;
                        if equal(a2757, a2758) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2759 = f574(v1);
                match m2759 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v6) => {
                        if f733(f576(v5, v6)?) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2760 = f228(v1);
                match m2760 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v8) => {
                        let a2761 = v7;
                        let a2762 = v8;
                        if equal(a2761, a2762) {
                            Ok(Adt157::C1(v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2763 = f405(v1);
                match m2763 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v10) => {
                        Ok(f734(f735(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2764 = f254(v1);
                match m2764 {
                    None => {
                        Ok(Adt157::C0)
                    }
                    Some(v12) => {
                        let a2766 = f580(v11.clone(), 0u64)?;
                        let a2767 = f580(match v12.clone() {
                            Adt5::C0(h2765) => {
                                h2765
                            }
                        }, 0u64)?;
                        if nat_eq(a2766, a2767) {
                            Ok(f734(f736(v11, match v12 {
                                Adt5::C0(h2768) => {
                                    h2768
                                }
                            }, List::<(Adt6, Adt6)>::nil()), v2))
                        } else {
                            Ok(Adt157::C0)
                        }
                    }
                }
            }
        }
    } else {
        Ok(Adt157::C0)
    }
}

pub fn f695(v0: Adt155, v1: Adt155) -> Adt155 {
    let m2769 = v0;
    match m2769 {
        Adt155::C0 => {
            f737(v1)
        }
        Adt155::C1 => {
            Adt155::C1
        }
        Adt155::C2 => {
            v1
        }
    }
}

pub fn f696(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt155> {
    let m2770 = f652(f738(v0, v1)?, f738(v2, v3)?);
    match m2770 {
        Adt155::C0 => {
            Ok(Adt155::C0)
        }
        Adt155::C1 => {
            f739(v0, f740(v0)?, v2, f740(v2)?)
        }
        Adt155::C2 => {
            Ok(Adt155::C2)
        }
    }
}

pub fn f697(v0: i64) -> R<i64> {
    let m2773 = {
        let a2771 = v0;
        let a2772 = 0i64;
        compare(a2771, a2772)
    };
    if match m2773 {
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
        let a2774 = v0;
        int_neg(a2774)
    } else {
        Ok(v0)
    }
}

pub fn f698(v0: Option<List<Adt42>>) -> List<(Str, Adt6)> {
    let m2775 = v0;
    match m2775 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f741(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f699(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2776 = v0;
    match m2776 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f700(p2779: List<Adt58>, p2780: List<Adt6>) -> List<Adt6> {
    let m2778 = (p2779, p2780);
    {
        let mut m2778 = m2778;
        loop {
            let step = {
                let (v0, v1) = m2778;
                let m2777 = v0;
                match m2777.uncons() {
                    None => {
                        Err::<(List<Adt58>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt58>, List<Adt6>), List<Adt6>>((v3, List::cons(f742(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2778 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(p2784: List<Adt71>, p2785: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2783 = (p2784, p2785);
    {
        let mut m2783 = m2783;
        loop {
            let step = {
                let (v0, v1) = m2783;
                let m2781 = v0;
                match m2781.uncons() {
                    None => {
                        Err::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2782 = f743(v2)?;
                        match m2782 {
                            None => {
                                Err::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt71>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2783 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f702(v0: Adt76) -> Adt6 {
    Adt6::C5({
        let a2795 = {
            let m2787 = match v0.clone() {
                Adt76::C0(h2786, _, _, _) => {
                    h2786
                }
            };
            match m2787 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f299(f744(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2796 = {
            let a2793 = f704(Str::lit("costPriority"), match v0.clone() {
                Adt76::C0(_, h2788, _, _) => {
                    h2788
                }
            });
            let a2794 = {
                let a2791 = f704(Str::lit("speedPriority"), match v0.clone() {
                    Adt76::C0(_, _, h2789, _) => {
                        h2789
                    }
                });
                let a2792 = f704(Str::lit("intelligencePriority"), match v0 {
                    Adt76::C0(_, _, _, h2790) => {
                        h2790
                    }
                });
                append_list(a2791, a2792)
            };
            append_list(a2793, a2794)
        };
        append_list(a2795, a2796)
    })
}

pub fn f703(v0: Adt78) -> Str {
    let m2797 = v0;
    match m2797 {
        Adt78::C0 => {
            Str::lit("none")
        }
        Adt78::C1 => {
            Str::lit("thisServer")
        }
        Adt78::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f704(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2798 = v1;
    match m2798 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f705(v0: Adt79) -> List<(Str, Adt6)> {
    let m2800 = match v0 {
        Adt79::C0(h2799) => {
            h2799
        }
    };
    match m2800 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f745(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f706(v0: Adt66) -> List<(Str, Adt6)> {
    let m2801 = v0;
    match m2801 {
        Adt66::C0(v1) => {
            let a2808 = {
                let m2803 = match v1.clone() {
                    Adt67::C0(h2802, _, _) => {
                        h2802
                    }
                };
                match m2803 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(_) => {
                        List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2809 = List::cons((Str::lit("message"), Adt6::C3(match v1.clone() {
                Adt67::C0(_, h2804, _) => {
                    h2804
                }
            })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
                let m2807 = match v1 {
                    Adt67::C0(_, _, h2806) => {
                        h2806
                    }
                };
                match m2807 {
                    Adt5::C0(h2805) => {
                        h2805
                    }
                }
            })), List::<(Str, Adt6)>::nil()));
            append_list(a2808, a2809)
        }
        Adt66::C1(v3) => {
            List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v3.clone() {
                Adt69::C0(h2810, _) => {
                    h2810
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v3 {
                Adt69::C0(_, h2811) => {
                    h2811
                }
            })), List::<(Str, Adt6)>::nil())))
        }
    }
}

pub fn f707(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2812, _, _, _) => {
            h2812
        }
    })), {
        let a2818 = f416(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2813, _, _) => {
                h2813
            }
        });
        let a2819 = {
            let a2816 = f746(match v0.clone() {
                Adt2::C0(_, _, h2814, _) => {
                    h2814
                }
            });
            let a2817 = f747(match v0 {
                Adt2::C0(_, _, _, h2815) => {
                    h2815
                }
            });
            append_list(a2816, a2817)
        };
        append_list(a2818, a2819)
    }))
}

pub fn f708(p2825: List<(Str, Adt6)>, p2826: List<(Str, Adt65)>) -> Option<List<(Str, Adt65)>> {
    let m2824 = (p2825, p2826);
    {
        let mut m2824 = m2824;
        loop {
            let step = {
                let (v0, v1) = m2824;
                let m2820 = v0;
                match m2820.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(Some(f748(v1, List::<(Str, Adt65)>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2822 = f749({
                            let (_, h2821) = v2;
                            h2821
                        });
                        match m2822 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>(None::<List<(Str, Adt65)>>)
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Adt65)>), Option<List<(Str, Adt65)>>>((v3, List::cons(({
                                    let (h2823, _) = v2;
                                    h2823
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(v0: Adt87) -> Adt6 {
    let m2827 = v0;
    match m2827 {
        Adt87::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("completes"), Adt6::C5(match v1 {
                Adt5::C0(h2828) => {
                    h2828
                }
            })), List::<(Str, Adt6)>::nil()))
        }
        Adt87::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("fails"), f129(v2)), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f710(v0: u64, v1: List<u64>) -> List<u64> {
    let a2829 = v0;
    let a2830 = 0u64;
    if nat_eq(a2829, a2830) {
        v1
    } else {
        let a2831 = v0;
        let a2832 = 1u64;
        if nat_eq(a2831, a2832) {
            List::cons(194u64, v1)
        } else {
            let a2833 = v0;
            let a2834 = 2u64;
            if nat_eq(a2833, a2834) {
                List::cons(226u64, v1)
            } else {
                let a2835 = v0;
                let a2836 = 3u64;
                if nat_eq(a2835, a2836) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f711(v0: u64, v1: u64, v2: List<u64>) -> Adt156 {
    let a2837 = v0;
    let a2838 = 0u64;
    if nat_eq(a2837, a2838) {
        f750(v1, v2)
    } else {
        let a2839 = v0;
        let a2840 = 1u64;
        if nat_eq(a2839, a2840) {
            let a2841 = 128u64;
            let a2842 = v1;
            if if nat_le(a2841, a2842) {
                let a2843 = v1;
                let a2844 = 159u64;
                nat_le(a2843, a2844)
            } else {
                false
            } {
                Adt156::C0(0u64, f751(v2))
            } else {
                f750(v1, List::cons(194u64, v2))
            }
        } else {
            let a2845 = v0;
            let a2846 = 2u64;
            if nat_eq(a2845, a2846) {
                let a2847 = v1;
                let a2848 = 128u64;
                if nat_eq(a2847, a2848) {
                    Adt156::C0(3u64, v2)
                } else {
                    let a2849 = v1;
                    let a2850 = 129u64;
                    if nat_eq(a2849, a2850) {
                        Adt156::C0(4u64, v2)
                    } else {
                        f750(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2851 = v0;
                let a2852 = 3u64;
                if nat_eq(a2851, a2852) {
                    let a2853 = 170u64;
                    let a2854 = v1;
                    if if nat_le(a2853, a2854) {
                        let a2855 = v1;
                        let a2856 = 174u64;
                        nat_le(a2855, a2856)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f751(v2))
                    } else {
                        f750(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2857 = 166u64;
                    let a2858 = v1;
                    if if nat_le(a2857, a2858) {
                        let a2859 = v1;
                        let a2860 = 169u64;
                        nat_le(a2859, a2860)
                    } else {
                        false
                    } {
                        Adt156::C0(0u64, f751(v2))
                    } else {
                        f750(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f712(v0: Adt140, v1: Adt140) -> bool {
    let a2861 = f461(v0);
    let a2862 = f461(v1);
    equal(a2861, a2862)
}

pub fn f713(v0: Adt140) -> bool {
    let m2863 = v0;
    match m2863 {
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

pub fn f714(v0: Adt140) -> bool {
    let m2864 = v0;
    match m2864 {
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

pub fn f715(v0: Adt0, v1: Adt17, v2: Adt82) -> Result<Adt128, Adt131> {
    f752(v0, v2.clone(), f753(match v1 {
        Adt17::C0(_, h2865, _, _, _, _) => {
            h2865
        }
    }, f429(v2)))
}

pub fn f716(v0: Adt15) -> Adt17 {
    let m2866 = v0;
    match m2866 {
        Adt15::C0(v1) => {
            match v1 {
                Adt16::C0(h2867) => {
                    h2867
                }
            }
        }
        Adt15::C1(v2) => {
            match v2 {
                Adt21::C0(h2868, _) => {
                    h2868
                }
            }
        }
        Adt15::C2(v3) => {
            match v3 {
                Adt22::C0(h2869, _, _, _, _) => {
                    h2869
                }
            }
        }
        Adt15::C3(v4) => {
            match v4 {
                Adt21::C0(h2870, _) => {
                    h2870
                }
            }
        }
        Adt15::C4(v5) => {
            match v5 {
                Adt23::C0(h2871, _, _, _) => {
                    h2871
                }
            }
        }
        Adt15::C5(v6) => {
            match v6 {
                Adt21::C0(h2872, _) => {
                    h2872
                }
            }
        }
        Adt15::C6(v7) => {
            match v7 {
                Adt21::C0(h2873, _) => {
                    h2873
                }
            }
        }
        Adt15::C7(v8) => {
            match v8 {
                Adt24::C0(h2874, _, _, _, _) => {
                    h2874
                }
            }
        }
        Adt15::C8(v9) => {
            match v9 {
                Adt25::C0(h2875, _, _, _) => {
                    h2875
                }
            }
        }
        Adt15::C9(v10) => {
            match v10 {
                Adt30::C0(h2876, _) => {
                    h2876
                }
            }
        }
        Adt15::C10(v11) => {
            match v11 {
                Adt31::C0(h2877, _, _) => {
                    h2877
                }
            }
        }
        Adt15::C11(v12) => {
            match v12 {
                Adt32::C0(h2878, _) => {
                    h2878
                }
            }
        }
    }
}

pub fn f717(v0: Adt0, v1: Adt17, v2: Adt83) -> Result<Adt128, Adt131> {
    f754(v0, v2, f755(match v1 {
        Adt17::C0(_, h2879, _, _, _, _) => {
            h2879
        }
    }))
}

pub fn f718(v0: Adt0, v1: Adt17, v2: Adt88) -> Result<Adt128, Adt131> {
    f756(v0, v2, f755(match v1 {
        Adt17::C0(_, h2880, _, _, _, _) => {
            h2880
        }
    }))
}

pub fn f719(v0: Adt0, v1: Adt17, v2: Adt50) -> Adt128 {
    Adt128::C0(Adt33::C2(match v2.clone() {
        Adt50::C0(h2881, _) => {
            h2881
        }
    }), {
        let m2883 = f616(v0, Adt33::C11);
        match m2883 {
            Adt128::C0(_, h2882, _) => {
                h2882
            }
        }
    }, f757(v1, match v2 {
        Adt50::C0(_, h2884) => {
            h2884
        }
    }))
}

pub fn f720(v0: Bytes, v1: Adt15, v2: Result<Adt33, Adt90>) -> R<Result<Adt33, Adt90>> {
    let m2885 = v2;
    match m2885 {
        Ok(v4) => {
            f758(v0, f615(v1.clone()), f759(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f721(v0: u64, v1: u64, v2: Fn2, v3: Adt15) -> R<Result<Adt33, Adt90>> {
    let m2886 = v3.clone();
    match m2886 {
        Adt15::C0(_) => {
            let c2888: Fn2 = v2;
            c2888.apply(v3)
        }
        Adt15::C1(_) => {
            let c2889: Fn2 = v2;
            c2889.apply(v3)
        }
        Adt15::C2(_) => {
            if f760(v0, v1) {
                let c2887: Fn2 = v2;
                c2887.apply(v3)
            } else {
                Ok(Err::<Adt33, Adt90>(Adt90::C7(v0)))
            }
        }
        Adt15::C3(_) => {
            let c2890: Fn2 = v2;
            c2890.apply(v3)
        }
        Adt15::C4(_) => {
            let c2891: Fn2 = v2;
            c2891.apply(v3)
        }
        Adt15::C5(_) => {
            let c2892: Fn2 = v2;
            c2892.apply(v3)
        }
        Adt15::C6(_) => {
            let c2893: Fn2 = v2;
            c2893.apply(v3)
        }
        Adt15::C7(_) => {
            let c2894: Fn2 = v2;
            c2894.apply(v3)
        }
        Adt15::C8(_) => {
            let c2895: Fn2 = v2;
            c2895.apply(v3)
        }
        Adt15::C9(_) => {
            let c2896: Fn2 = v2;
            c2896.apply(v3)
        }
        Adt15::C10(_) => {
            let c2897: Fn2 = v2;
            c2897.apply(v3)
        }
        Adt15::C11(_) => {
            let c2898: Fn2 = v2;
            c2898.apply(v3)
        }
    }
}

pub fn f722(v0: Bytes, v1: Adt22) -> R<Result<Adt15, Adt90>> {
    let m2902 = f761(v0, Str::lit("tools/call"), match v1.clone() {
        Adt22::C0(_, h2899, _, _, _) => {
            h2899
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, _, h2900) => {
            h2900
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h2901, _) => {
            h2901
        }
    })?;
    match m2902 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C2(Adt22::C0(match v1.clone() {
                Adt22::C0(h2903, _, _, _, _) => {
                    h2903
                }
            }, match v1.clone() {
                Adt22::C0(_, h2904, _, _, _) => {
                    h2904
                }
            }, match v1.clone() {
                Adt22::C0(_, _, h2905, _, _) => {
                    h2905
                }
            }, match v1 {
                Adt22::C0(_, _, _, h2906, _) => {
                    h2906
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f723(v0: Bytes, v1: Adt23) -> R<Result<Adt15, Adt90>> {
    let m2910 = f761(v0, Str::lit("resources/read"), match v1.clone() {
        Adt23::C0(_, h2907, _, _) => {
            h2907
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2908) => {
            h2908
        }
    }, match v1.clone() {
        Adt23::C0(_, _, h2909, _) => {
            h2909
        }
    })?;
    match m2910 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C4(Adt23::C0(match v1.clone() {
                Adt23::C0(h2911, _, _, _) => {
                    h2911
                }
            }, match v1.clone() {
                Adt23::C0(_, h2912, _, _) => {
                    h2912
                }
            }, match v1 {
                Adt23::C0(_, _, h2913, _) => {
                    h2913
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f724(v0: Bytes, v1: Adt24) -> R<Result<Adt15, Adt90>> {
    let m2917 = f761(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt24::C0(_, h2914, _, _, _) => {
            h2914
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, _, h2915) => {
            h2915
        }
    }, match v1.clone() {
        Adt24::C0(_, _, _, h2916, _) => {
            h2916
        }
    })?;
    match m2917 {
        Ok(v3) => {
            Ok(Ok::<Adt15, Adt90>(Adt15::C7(Adt24::C0(match v1.clone() {
                Adt24::C0(h2918, _, _, _, _) => {
                    h2918
                }
            }, match v1.clone() {
                Adt24::C0(_, h2919, _, _, _) => {
                    h2919
                }
            }, match v1.clone() {
                Adt24::C0(_, _, h2920, _, _) => {
                    h2920
                }
            }, match v1 {
                Adt24::C0(_, _, _, h2921, _) => {
                    h2921
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt15, Adt90>(v2))
        }
    }
}

pub fn f725(v0: Str, p2927: List<(Str, Adt6)>, p2928: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt132> {
    let m2926 = (p2927, p2928);
    {
        let mut m2926 = m2926;
        loop {
            let step = {
                let (v1, v2) = m2926;
                let m2922 = v1;
                match m2922.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Ok::<List<(Str, Str)>, Adt132>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2924 = f228({
                            let (_, h2923) = v3;
                            h2923
                        });
                        match m2924 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>(Err::<List<(Str, Str)>, Adt132>(Adt132::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt132>>((v4, List::cons(({
                                    let (h2925, _) = v3;
                                    h2925
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2926 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(p2931: List<(Str, Str)>, p2932: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2930 = (p2931, p2932);
    {
        let mut m2930 = m2930;
        loop {
            let step = {
                let (v0, v1) = m2930;
                let m2929 = v0;
                match m2929.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2930 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(v0: Adt111, v1: Adt5) -> R<Option<Adt109>> {
    let m2934 = f762(match v0.clone() {
        Adt111::C0(_, h2933, _, _) => {
            h2933
        }
    }, v1);
    match m2934 {
        Adt135::C0 => {
            Ok(f763(v0))
        }
        Adt135::C1(v2) => {
            if f764(v2.clone()) {
                Ok(f763(v0))
            } else {
                f765(v0, v2)
            }
        }
    }
}

pub fn f728(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2935 = f177(v0, v1);
    match m2935 {
        Adt135::C0 => {
            Some(None::<Adt5>)
        }
        Adt135::C1(v2) => {
            let m2936 = f254(v2);
            match m2936 {
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

pub fn f729() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f730(v0: Str) -> Adt158 {
    let a2937 = v0.clone();
    let a2938 = Str::lit("debug");
    if equal(a2937, a2938) {
        Adt158::C1(Adt19::C0)
    } else {
        let a2939 = v0.clone();
        let a2940 = Str::lit("info");
        if equal(a2939, a2940) {
            Adt158::C1(Adt19::C1)
        } else {
            let a2941 = v0.clone();
            let a2942 = Str::lit("notice");
            if equal(a2941, a2942) {
                Adt158::C1(Adt19::C2)
            } else {
                let a2943 = v0.clone();
                let a2944 = Str::lit("warning");
                if equal(a2943, a2944) {
                    Adt158::C1(Adt19::C3)
                } else {
                    let a2945 = v0.clone();
                    let a2946 = Str::lit("error");
                    if equal(a2945, a2946) {
                        Adt158::C1(Adt19::C4)
                    } else {
                        let a2947 = v0.clone();
                        let a2948 = Str::lit("critical");
                        if equal(a2947, a2948) {
                            Adt158::C1(Adt19::C5)
                        } else {
                            let a2949 = v0.clone();
                            let a2950 = Str::lit("alert");
                            if equal(a2949, a2950) {
                                Adt158::C1(Adt19::C6)
                            } else {
                                let a2951 = v0;
                                let a2952 = Str::lit("emergency");
                                if equal(a2951, a2952) {
                                    Adt158::C1(Adt19::C7)
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

pub fn f731() -> Str {
    Str::lit("progressToken")
}

pub fn f732(v0: Adt6) -> u64 {
    let m2953 = v0;
    match m2953 {
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

pub fn f733(v0: Adt155) -> bool {
    let m2954 = v0;
    match m2954 {
        Adt155::C0 => {
            false
        }
        Adt155::C1 => {
            true
        }
        Adt155::C2 => {
            false
        }
    }
}

pub fn f734(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt157 {
    let m2955 = v0;
    match m2955 {
        None => {
            Adt157::C0
        }
        Some(v2) => {
            Adt157::C1({
                let a2956 = v2;
                let a2957 = v1;
                append_list(a2956, a2957)
            })
        }
    }
}

pub fn f735(p2962: List<Adt6>, p2963: List<Adt6>, p2964: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2961 = (p2962, (p2963, p2964));
    {
        let mut m2961 = m2961;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2961;
                let m2958 = v0;
                match m2958.uncons() {
                    None => {
                        let m2959 = v1;
                        match m2959.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2960 = v1;
                        match m2960.uncons() {
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
                Ok(next) => { m2961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f736(p2970: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2971: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2969 = (p2970, p2971);
    {
        let mut m2969 = m2969;
        loop {
            let step = {
                let (v0, v2) = m2969;
                let m2965 = v0;
                match m2965.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2967 = f231({
                            let (h2966, _) = v3;
                            h2966
                        }, v1.clone());
                        match m2967 {
                            Adt135::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt135::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2968) = v3;
                                    h2968
                                }, v5), v2)))
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

pub fn f737(v0: Adt155) -> Adt155 {
    let m2972 = v0;
    match m2972 {
        Adt155::C0 => {
            Adt155::C2
        }
        Adt155::C1 => {
            Adt155::C1
        }
        Adt155::C2 => {
            Adt155::C0
        }
    }
}

pub fn f738(v0: i64, v1: i64) -> R<i64> {
    f116(f80(f740(v0)?, 0i64)?, v1)
}

pub fn f739(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt155> {
    let a2973 = v1;
    let a2974 = v3;
    if nat_lt(a2973, a2974) {
        Ok(f652(f766({
            let a2975 = v3;
            let a2976 = v1;
            nat_sub(a2975, a2976)
        }, v0)?, v2))
    } else {
        let a2977 = v3;
        let a2978 = v1;
        if nat_lt(a2977, a2978) {
            Ok(f652(v0, f766({
                let a2979 = v1;
                let a2980 = v3;
                nat_sub(a2979, a2980)
            }, v2)?))
        } else {
            Ok(f652(v0, v2))
        }
    }
}

pub fn f740(v0: i64) -> R<u64> {
    f767(20u64, v0, 0u64)
}

pub fn f741(v0: List<Adt42>) -> List<Adt6> {
    f299(f768(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f742(v0: Adt58) -> Adt6 {
    Adt6::C5({
        let a2989 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt58::C0(h2981, _, _, _) => {
                h2981
            }
        })), List::<(Str, Adt6)>::nil());
        let a2990 = {
            let a2987 = f416(Str::lit("title"), match v0.clone() {
                Adt58::C0(_, h2982, _, _) => {
                    h2982
                }
            });
            let a2988 = {
                let a2985 = f416(Str::lit("description"), match v0.clone() {
                    Adt58::C0(_, _, h2983, _) => {
                        h2983
                    }
                });
                let a2986 = f421(Str::lit("required"), match v0 {
                    Adt58::C0(_, _, _, h2984) => {
                        h2984
                    }
                });
                append_list(a2985, a2986)
            };
            append_list(a2987, a2988)
        };
        append_list(a2989, a2990)
    })
}

pub fn f743(v0: Adt71) -> R<Option<Adt6>> {
    let m2992 = f769(match v0.clone() {
        Adt71::C0(_, h2991, _) => {
            h2991
        }
    })?;
    match m2992 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f663(match v0.clone() {
                Adt71::C0(h2993, _, _) => {
                    h2993
                }
            }))), List::cons((Str::lit("content"), v1), f655(match v0 {
                Adt71::C0(_, _, h2994) => {
                    h2994
                }
            }))))))
        }
    }
}

pub fn f744(p2998: List<Adt77>, p2999: List<Adt6>) -> List<Adt6> {
    let m2997 = (p2998, p2999);
    {
        let mut m2997 = m2997;
        loop {
            let step = {
                let (v0, v1) = m2997;
                let m2995 = v0;
                match m2995.uncons() {
                    None => {
                        Err::<(List<Adt77>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt77>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f416(Str::lit("name"), match v2 {
                            Adt77::C0(h2996) => {
                                h2996
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f745(v0: Adt80) -> Str {
    let m3000 = v0;
    match m3000 {
        Adt80::C0 => {
            Str::lit("auto")
        }
        Adt80::C1 => {
            Str::lit("required")
        }
        Adt80::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f746(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m3001 = v0;
    match m3001 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f374(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f747(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m3002 = v0;
    match m3002 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f770(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f748(p3005: List<(Str, Adt65)>, p3006: List<(Str, Adt65)>) -> List<(Str, Adt65)> {
    let m3004 = (p3005, p3006);
    {
        let mut m3004 = m3004;
        loop {
            let step = {
                let (v0, v1) = m3004;
                let m3003 = v0;
                match m3003.uncons() {
                    None => {
                        Err::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt65)>, List<(Str, Adt65)>), List<(Str, Adt65)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3004 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f749(v0: Adt6) -> Option<Adt65> {
    let m3007 = f254(v0);
    match m3007 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m3008 = f671(Str::lit("method"), v1.clone());
            match m3008 {
                None => {
                    None::<Adt65>
                }
                Some(v2) => {
                    let a3009 = v2.clone();
                    let a3010 = Str::lit("elicitation/create");
                    if equal(a3009, a3010) {
                        let m3011 = f606(Str::lit("params"), v1);
                        match m3011 {
                            None => {
                                None::<Adt65>
                            }
                            Some(v3) => {
                                f771(v3)
                            }
                        }
                    } else {
                        let a3012 = v2;
                        let a3013 = Str::lit("roots/list");
                        if equal(a3012, a3013) {
                            f772(v1)
                        } else {
                            None::<Adt65>
                        }
                    }
                }
            }
        }
    }
}

pub fn f750(v0: u64, v1: List<u64>) -> Adt156 {
    let a3014 = v0;
    let a3015 = 32u64;
    if nat_lt(a3014, a3015) {
        if f773(v0) {
            Adt156::C0(0u64, List::cons(v0, v1))
        } else {
            Adt156::C0(0u64, f751(v1))
        }
    } else {
        let a3016 = v0;
        let a3017 = 127u64;
        if nat_eq(a3016, a3017) {
            Adt156::C0(0u64, f751(v1))
        } else {
            let a3018 = v0;
            let a3019 = 194u64;
            if nat_eq(a3018, a3019) {
                Adt156::C0(1u64, v1)
            } else {
                let a3020 = v0;
                let a3021 = 226u64;
                if nat_eq(a3020, a3021) {
                    Adt156::C0(2u64, v1)
                } else {
                    Adt156::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f751(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f752(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C12(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f774(f429(v1)))))
    }
}

pub fn f753(v0: Adt18, v1: List<(Str, Adt65)>) -> bool {
    let a3022 = f775(v1.clone());
    if if if bool_not(a3022) {
        true
    } else {
        f464(match v0.clone() {
            Adt18::C0(_, _, _, h3023, _) => {
                h3023
            }
        })
    } {
        let a3024 = f776(v1.clone());
        if bool_not(a3024) {
            true
        } else {
            f464(match v0.clone() {
                Adt18::C0(_, _, h3025, _, _) => {
                    h3025
                }
            })
        }
    } else {
        false
    } {
        let a3026 = f777(v1);
        if bool_not(a3026) {
            true
        } else {
            f464(match v0 {
                Adt18::C0(_, h3027, _, _, _) => {
                    h3027
                }
            })
        }
    } else {
        false
    }
}

pub fn f754(v0: Adt0, v1: Adt83, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C13(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f778())))
    }
}

pub fn f755(v0: Adt18) -> bool {
    f465(match v0 {
        Adt18::C0(_, _, _, _, h3028) => {
            h3028
        }
    }, f26())
}

pub fn f756(v0: Adt0, v1: Adt88, v2: bool) -> Result<Adt128, Adt131> {
    if v2 {
        Ok::<Adt128, Adt131>(f616(v0, Adt33::C14(v1)))
    } else {
        Err::<Adt128, Adt131>(Adt131::C4(Adt90::C8(f778())))
    }
}

pub fn f757(v0: Adt17, v1: List<Adt51>) -> List<Adt130> {
    let m3030 = match v0 {
        Adt17::C0(_, _, _, _, h3029, _) => {
            h3029
        }
    };
    match m3030 {
        None => {
            List::<Adt130>::nil()
        }
        Some(v2) => {
            f779(f780(v2, v1, List::<Adt130>::nil()), List::<Adt130>::nil())
        }
    }
}

pub fn f758(v0: Bytes, v1: Adt140, v2: Str, v3: Adt33) -> R<Result<Adt33, Adt90>> {
    let m3031 = v3.clone();
    match m3031 {
        Adt33::C0(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C1(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C2(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C3(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C4(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C5(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C6(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C7(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C8(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C9(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C10(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C11 => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C12(v4) => {
            f781(v0, v1, v2, v4)
        }
        Adt33::C13(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
        Adt33::C14(_) => {
            Ok(Ok::<Adt33, Adt90>(v3))
        }
    }
}

pub fn f759(v0: Adt15) -> Str {
    let m3032 = v0;
    match m3032 {
        Adt15::C0(_) => {
            Str::lit("")
        }
        Adt15::C1(_) => {
            Str::lit("")
        }
        Adt15::C2(v1) => {
            match v1 {
                Adt22::C0(_, h3033, _, _, _) => {
                    h3033
                }
            }
        }
        Adt15::C3(_) => {
            Str::lit("")
        }
        Adt15::C4(v2) => {
            match v2 {
                Adt23::C0(_, h3034, _, _) => {
                    h3034
                }
            }
        }
        Adt15::C5(_) => {
            Str::lit("")
        }
        Adt15::C6(_) => {
            Str::lit("")
        }
        Adt15::C7(v3) => {
            match v3 {
                Adt24::C0(_, h3035, _, _, _) => {
                    h3035
                }
            }
        }
        Adt15::C8(_) => {
            Str::lit("")
        }
        Adt15::C9(_) => {
            Str::lit("")
        }
        Adt15::C10(_) => {
            Str::lit("")
        }
        Adt15::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f760(v0: u64, v1: u64) -> bool {
    let a3036 = v1;
    let a3037 = v0;
    nat_lt(a3036, a3037)
}

pub fn f761(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt90>> {
    Ok(f782(f783(v0, v1, v2, v3)?, f784(v4)))
}

pub fn f762(p3043: List<Str>, p3044: Adt5) -> Adt135 {
    let m3042 = (p3043, p3044);
    {
        let mut m3042 = m3042;
        loop {
            let step = {
                let (v0, v1) = m3042;
                let m3038 = v0;
                match m3038.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                    }
                    Some((v2, v3)) => {
                        let m3039 = f177(v2, v1);
                        match m3039 {
                            Adt135::C0 => {
                                Err::<(List<Str>, Adt5), Adt135>(Adt135::C0)
                            }
                            Adt135::C1(v4) => {
                                let m3040 = v3.clone();
                                match m3040.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt135>(Adt135::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m3041 = f254(v4);
                                        match m3041 {
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
                Ok(next) => { m3042 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f763(v0: Adt111) -> Option<Adt109> {
    let m3046 = match v0.clone() {
        Adt111::C0(_, _, _, h3045) => {
            h3045
        }
    };
    match m3046 {
        Adt112::C0 => {
            None::<Adt109>
        }
        Adt112::C1 => {
            Some(Adt109::C10(match v0 {
                Adt111::C0(h3047, _, _, _) => {
                    h3047
                }
            }))
        }
        Adt112::C2(_) => {
            None::<Adt109>
        }
    }
}

pub fn f764(v0: Adt6) -> bool {
    let m3048 = v0;
    match m3048 {
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

pub fn f765(v0: Adt111, v1: Adt6) -> R<Option<Adt109>> {
    let m3050 = match v0.clone() {
        Adt111::C0(_, _, _, h3049) => {
            h3049
        }
    };
    match m3050 {
        Adt112::C0 => {
            Ok(Some(Adt109::C8(match v0 {
                Adt111::C0(h3051, _, _, _) => {
                    h3051
                }
            })))
        }
        Adt112::C1 => {
            Ok(Some(Adt109::C10(match v0 {
                Adt111::C0(h3052, _, _, _) => {
                    h3052
                }
            })))
        }
        Adt112::C2(v2) => {
            if f785(match v0.clone() {
                Adt111::C0(_, _, h3053, _) => {
                    h3053
                }
            }, v2, v1)? {
                Ok(None::<Adt109>)
            } else {
                Ok(Some(Adt109::C9(match v0 {
                    Adt111::C0(h3054, _, _, _) => {
                        h3054
                    }
                })))
            }
        }
    }
}

pub fn f766(p3059: u64, p3060: i64) -> R<i64> {
    let m3058 = (p3059, p3060);
    {
        let mut m3058 = m3058;
        loop {
            let step = {
                let (v0, v1) = m3058;
                let m3055 = v0;
                if m3055 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m3055 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a3056 = v1;
                        let a3057 = 10i64;
                        int_mul(a3056, a3057)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f767(p3071: u64, p3072: i64, p3073: u64) -> R<u64> {
    let m3070 = (p3071, (p3072, p3073));
    {
        let mut m3070 = m3070;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3070;
                let m3061 = v0;
                if m3061 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m3061 - 1;
                    let v4: u64 = {
                        let a3062 = v2;
                        let a3063 = 1u64;
                        nat_add(a3062, a3063)?
                    };
                    let m3066 = {
                        let a3064 = v1;
                        let a3065 = 10i64;
                        compare(a3064, a3065)
                    };
                    if match m3066 {
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
                            let a3067 = v1;
                            let a3068 = 10i64;
                            let a3069 = 0i64;
                            int_quot(a3067, a3068, a3069)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m3070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f768(p3076: List<Adt42>, p3077: List<Adt6>) -> List<Adt6> {
    let m3075 = (p3076, p3077);
    {
        let mut m3075 = m3075;
        loop {
            let step = {
                let (v0, v1) = m3075;
                let m3074 = v0;
                match m3074.uncons() {
                    None => {
                        Err::<(List<Adt42>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt42>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f663(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f769(v0: Adt72) -> R<Option<Adt6>> {
    let m3078 = v0;
    match m3078 {
        Adt72::C0(v1) => {
            f786(v1)
        }
        Adt72::C1(v2) => {
            let m3079 = f787(v2, List::<Adt6>::nil())?;
            match m3079 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f299(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f770(v0: Adt3) -> Str {
    let m3080 = v0;
    match m3080 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f771(v0: Adt5) -> Option<Adt65> {
    let m3081 = f671(Str::lit("message"), v0.clone());
    match m3081 {
        None => {
            None::<Adt65>
        }
        Some(v1) => {
            let m3082 = f177(Str::lit("mode"), v0.clone());
            match m3082 {
                Adt135::C0 => {
                    let m3083 = f606(Str::lit("requestedSchema"), v0);
                    match m3083 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v2) => {
                            Some(Adt65::C0(Adt66::C0(Adt67::C0(None::<Adt68>, v1, v2))))
                        }
                    }
                }
                Adt135::C1(v3) => {
                    let m3084 = f228(v3);
                    match m3084 {
                        None => {
                            None::<Adt65>
                        }
                        Some(v4) => {
                            let a3085 = v4.clone();
                            let a3086 = Str::lit("url");
                            if equal(a3085, a3086) {
                                let m3087 = f671(Str::lit("url"), v0);
                                match m3087 {
                                    None => {
                                        None::<Adt65>
                                    }
                                    Some(v5) => {
                                        Some(Adt65::C0(Adt66::C1(Adt69::C0(v1, v5))))
                                    }
                                }
                            } else {
                                let a3088 = v4;
                                let a3089 = Str::lit("form");
                                if equal(a3088, a3089) {
                                    let m3090 = f606(Str::lit("requestedSchema"), v0);
                                    match m3090 {
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

pub fn f772(v0: Adt5) -> Option<Adt65> {
    let m3091 = f177(Str::lit("params"), v0);
    match m3091 {
        Adt135::C0 => {
            Some(Adt65::C2(None::<Adt5>))
        }
        Adt135::C1(v1) => {
            let m3092 = f254(v1);
            match m3092 {
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

pub fn f773(v0: u64) -> bool {
    let a3093 = v0;
    let a3094 = 9u64;
    if nat_eq(a3093, a3094) {
        true
    } else {
        let a3095 = v0;
        let a3096 = 10u64;
        if nat_eq(a3095, a3096) {
            true
        } else {
            let a3097 = v0;
            let a3098 = 13u64;
            nat_eq(a3097, a3098)
        }
    }
}

pub fn f774(v0: List<(Str, Adt65)>) -> Adt18 {
    Adt18::C0(None::<Adt5>, if f777(v0.clone()) {
        Some(f546())
    } else {
        None::<Adt5>
    }, if f776(v0.clone()) {
        Some(f546())
    } else {
        None::<Adt5>
    }, if f775(v0) {
        Some(f546())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f775(p3103: List<(Str, Adt65)>) -> bool {
    let m3102 = p3103;
    {
        let mut m3102 = m3102;
        loop {
            let step = {
                let v0 = m3102;
                let m3099 = v0;
                match m3099.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3101 = {
                            let (_, h3100) = v1;
                            h3100
                        };
                        match m3101 {
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
                Ok(next) => { m3102 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f776(p3108: List<(Str, Adt65)>) -> bool {
    let m3107 = p3108;
    {
        let mut m3107 = m3107;
        loop {
            let step = {
                let v0 = m3107;
                let m3104 = v0;
                match m3104.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3106 = {
                            let (_, h3105) = v1;
                            h3105
                        };
                        match m3106 {
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
                Ok(next) => { m3107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f777(p3113: List<(Str, Adt65)>) -> bool {
    let m3112 = p3113;
    {
        let mut m3112 = m3112;
        loop {
            let step = {
                let v0 = m3112;
                let m3109 = v0;
                match m3109.uncons() {
                    None => {
                        Err::<List<(Str, Adt65)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m3111 = {
                            let (_, h3110) = v1;
                            h3110
                        };
                        match m3111 {
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
                Ok(next) => { m3112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f778() -> Adt18 {
    Adt18::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f26(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f779(p3116: List<Adt130>, p3117: List<Adt130>) -> List<Adt130> {
    let m3115 = (p3116, p3117);
    {
        let mut m3115 = m3115;
        loop {
            let step = {
                let (v0, v1) = m3115;
                let m3114 = v0;
                match m3114.uncons() {
                    None => {
                        Err::<(List<Adt130>, List<Adt130>), List<Adt130>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt130>, List<Adt130>), List<Adt130>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3115 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f780(v0: Adt20, p3120: List<Adt51>, p3121: List<Adt130>) -> List<Adt130> {
    let m3119 = (p3120, p3121);
    {
        let mut m3119 = m3119;
        loop {
            let step = {
                let (v1, v2) = m3119;
                let m3118 = v1;
                match m3118.uncons() {
                    None => {
                        Err::<(List<Adt51>, List<Adt130>), List<Adt130>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt51>, List<Adt130>), List<Adt130>>((v4, List::cons(Adt130::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m3119 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f781(v0: Bytes, v1: Adt140, v2: Str, v3: Adt82) -> R<Result<Adt33, Adt90>> {
    if f713(v1.clone()) {
        if f788(v3.clone()) {
            let m3122 = f789(v0, f461(v1), v2, v3)?;
            match m3122 {
                Ok(v5) => {
                    Ok(Ok::<Adt33, Adt90>(Adt33::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt33, Adt90>(v4))
                }
            }
        } else {
            Ok(Err::<Adt33, Adt90>(Adt90::C13))
        }
    } else {
        Ok(Err::<Adt33, Adt90>(Adt90::C12(f461(v1))))
    }
}

pub fn f782(v0: Result<Option<Str>, Adt90>, v1: bool) -> Result<Option<Str>, Adt90> {
    let m3123 = v0;
    match m3123 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt90>(v3)
            } else {
                Err::<Option<Str>, Adt90>(Adt90::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt90>(v2)
        }
    }
}

pub fn f783(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt90>> {
    let m3124 = v3;
    match m3124 {
        None => {
            Ok(Ok::<Option<Str>, Adt90>(None::<Str>))
        }
        Some(v4) => {
            Ok(f790(f791(v0, v1, v2, v4)?))
        }
    }
}

pub fn f784(v0: Option<Adt5>) -> bool {
    let m3125 = v0;
    match m3125 {
        None => {
            true
        }
        Some(v1) => {
            f792(match v1 {
                Adt5::C0(h3126) => {
                    h3126
                }
            })
        }
    }
}

pub fn f785(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m3127 = v0;
    match m3127 {
        Adt13::C0 => {
            let m3128 = f228(v2);
            match m3128 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a3129 = v1;
                    let a3130 = v3;
                    Ok(equal(a3129, a3130))
                }
            }
        }
        Adt13::C1 => {
            f793(v1, v2)
        }
        Adt13::C2 => {
            let m3131 = f253(v2);
            match m3131 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a3132 = v1;
                        let a3133 = Str::lit("true");
                        Ok(equal(a3132, a3133))
                    } else {
                        let a3134 = v1;
                        let a3135 = Str::lit("false");
                        Ok(equal(a3134, a3135))
                    }
                }
            }
        }
    }
}

pub fn f786(v0: Adt73) -> R<Option<Adt6>> {
    let m3136 = v0;
    match m3136 {
        Adt73::C0(v1) => {
            Ok(Some(f656(v1)))
        }
        Adt73::C1(v2) => {
            Ok(Some(f657(Str::lit("image"), match v2.clone() {
                Adt43::C0(h3137, _, _, _) => {
                    h3137
                }
            }, match v2.clone() {
                Adt43::C0(_, h3138, _, _) => {
                    h3138
                }
            }, match v2.clone() {
                Adt43::C0(_, _, h3139, _) => {
                    h3139
                }
            }, match v2 {
                Adt43::C0(_, _, _, h3140) => {
                    h3140
                }
            })))
        }
        Adt73::C2(v3) => {
            Ok(Some(f657(Str::lit("audio"), match v3.clone() {
                Adt44::C0(h3141, _, _, _) => {
                    h3141
                }
            }, match v3.clone() {
                Adt44::C0(_, h3142, _, _) => {
                    h3142
                }
            }, match v3.clone() {
                Adt44::C0(_, _, h3143, _) => {
                    h3143
                }
            }, match v3 {
                Adt44::C0(_, _, _, h3144) => {
                    h3144
                }
            })))
        }
        Adt73::C3(v4) => {
            Ok(Some(f794(v4)))
        }
        Adt73::C4(v5) => {
            f795(v5)
        }
    }
}

pub fn f787(p3148: List<Adt73>, p3149: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m3147 = (p3148, p3149);
    {
        let mut m3147 = m3147;
        loop {
            let step = {
                let (v0, v1) = m3147;
                let m3145 = v0;
                match m3145.uncons() {
                    None => {
                        Err::<(List<Adt73>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3146 = f786(v2)?;
                        match m3146 {
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
                Ok(next) => { m3147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f788(v0: Adt82) -> bool {
    let m3150 = v0;
    match m3150 {
        Adt82::C0(v1) => {
            let m3151 = v1;
            match m3151.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt82::C1(_) => {
            true
        }
        Adt82::C2(_, _) => {
            true
        }
    }
}

pub fn f789(v0: Bytes, v1: Str, v2: Str, v3: Adt82) -> R<Result<Adt82, Adt90>> {
    let m3152 = v3.clone();
    match m3152 {
        Adt82::C0(_) => {
            Ok(Ok::<Adt82, Adt90>(v3))
        }
        Adt82::C1(v5) => {
            Ok(f796(f797(v0, v1, v2, v5)?, List::<(Str, Adt65)>::nil()))
        }
        Adt82::C2(v6, v7) => {
            Ok(f796(f797(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f790(v0: Option<Str>) -> Result<Option<Str>, Adt90> {
    let m3153 = v0;
    match m3153 {
        None => {
            Err::<Option<Str>, Adt90>(Adt90::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt90>(Some(v1))
        }
    }
}

pub fn f791(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3157 = f798(v0, {
        let a3154 = v1;
        utf8_encode(a3154)
    }, {
        let a3155 = v2;
        utf8_encode(a3155)
    }, {
        let a3156 = v3;
        utf8_encode(a3156)
    })?;
    match m3157 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3158 = v4;
            Ok(utf8_decode(a3158))
        }
    }
}

pub fn f792(p3163: List<(Str, Adt6)>) -> bool {
    let m3162 = p3163;
    {
        let mut m3162 = m3162;
        loop {
            let step = {
                let v0 = m3162;
                let m3159 = v0;
                match m3159.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m3161 = f254({
                            let (_, h3160) = v1;
                            h3160
                        });
                        match m3161 {
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
                Ok(next) => { m3162 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f793(v0: Str, v1: Adt6) -> R<bool> {
    let m3164 = f574(v1);
    match m3164 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m3165 = f230(v2)?;
            match m3165 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m3167 = f799(f86({
                        let a3166 = v0;
                        utf8_encode(a3166)
                    })?)?;
                    match m3167 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f390({
                                let a3168 = v3;
                                let a3169 = v4;
                                int_sub(a3168, a3169)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f794(v0: Adt74) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt74::C0(h3170, _, _, _) => {
            h3170
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt74::C0(_, h3171, _, _) => {
            h3171
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m3174 = match v0.clone() {
            Adt74::C0(_, _, h3173, _) => {
                h3173
            }
        };
        match m3174 {
            Adt5::C0(h3172) => {
                h3172
            }
        }
    })), f655(match v0 {
        Adt74::C0(_, _, _, h3175) => {
            h3175
        }
    }))))))
}

pub fn f795(v0: Adt75) -> R<Option<Adt6>> {
    let m3177 = f419(match v0.clone() {
        Adt75::C0(_, h3176, _, _, _) => {
            h3176
        }
    })?;
    match m3177 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt75::C0(h3178, _, _, _, _) => {
                    h3178
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a3184 = f420(Str::lit("structuredContent"), match v0.clone() {
                    Adt75::C0(_, _, h3179, _, _) => {
                        h3179
                    }
                });
                let a3185 = {
                    let a3182 = f421(Str::lit("isError"), match v0.clone() {
                        Adt75::C0(_, _, _, h3180, _) => {
                            h3180
                        }
                    });
                    let a3183 = f655(match v0 {
                        Adt75::C0(_, _, _, _, h3181) => {
                            h3181
                        }
                    });
                    append_list(a3182, a3183)
                };
                append_list(a3184, a3185)
            }))))))
        }
    }
}

pub fn f796(v0: Option<Str>, v1: List<(Str, Adt65)>) -> Result<Adt82, Adt90> {
    let m3186 = v0;
    match m3186 {
        None => {
            Err::<Adt82, Adt90>(Adt90::C14)
        }
        Some(v2) => {
            let m3187 = v1.clone();
            match m3187.uncons() {
                None => {
                    Ok::<Adt82, Adt90>(Adt82::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt82, Adt90>(Adt82::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f797(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m3191 = f800(v0, {
        let a3188 = v1;
        utf8_encode(a3188)
    }, {
        let a3189 = v2;
        utf8_encode(a3189)
    }, {
        let a3190 = v3;
        utf8_encode(a3190)
    })?;
    match m3191 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a3192 = v4;
            Ok(utf8_decode(a3192))
        }
    }
}

pub fn f798(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3194 = f801();
    let a3195 = {
        let a3193 = v0.clone();
        length_bytes(a3193)
    };
    if nat_le(a3194, a3195) {
        let a3197 = f802(v3.clone())?;
        let a3198 = {
            let a3196 = v3.clone();
            length_bytes(a3196)
        };
        if nat_eq(a3197, a3198) {
            Ok(None::<Bytes>)
        } else {
            f803(v0, v1, v2, v3.clone(), f802(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f799(v0: List<u64>) -> R<Option<i64>> {
    let m3199 = v0.clone();
    match m3199.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a3200 = v1;
            let a3201 = 45u64;
            if nat_eq(a3200, a3201) {
                let m3202 = v2.clone();
                match m3202.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m3203 = f804(v2, 0i64)?;
                        match m3203 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a3204 = v5;
                                    int_neg(a3204)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f804(v0, 0i64)
            }
        }
    }
}

pub fn f800(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a3206 = f801();
    let a3207 = {
        let a3205 = v0.clone();
        length_bytes(a3205)
    };
    if nat_le(a3206, a3207) {
        Ok(Some({
            let a3210 = f805(v3.clone())?;
            let a3211 = {
                let a3208 = Bytes::lit(&[0x2e]);
                let a3209 = f805(f806(v0, v1, v2, v3)?)?;
                append_bytes(a3208, a3209)
            };
            append_bytes(a3210, a3211)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f801() -> u64 {
    32u64
}

pub fn f802(v0: Bytes) -> R<u64> {
    let (_, h3212) = f807(Fn4::F808(v0.clone()), {
        let a3213 = v0.clone();
        length_bytes(a3213)
    }, (0u64, {
        let a3214 = v0;
        length_bytes(a3214)
    }))?;
    Ok(h3212)
}

pub fn f803(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m3218 = {
        let a3215 = v3.clone();
        let a3216 = 0u64;
        let a3217 = v4;
        slice_bytes(a3215, a3216, a3217)
    };
    match m3218 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3229 = {
                let a3226 = v3.clone();
                let a3227 = {
                    let a3219 = v4;
                    let a3220 = 1u64;
                    nat_add(a3219, a3220)?
                };
                let a3228 = {
                    let a3224 = {
                        let a3221 = v3;
                        length_bytes(a3221)
                    };
                    let a3225 = {
                        let a3222 = v4;
                        let a3223 = 1u64;
                        nat_add(a3222, a3223)?
                    };
                    nat_sub(a3224, a3225)
                };
                slice_bytes(a3226, a3227, a3228)
            };
            match m3229 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f809(v0, v1, v2, f810(v5)?, f810(v6)?)
                }
            }
        }
    }
}

pub fn f804(p3235: List<u64>, p3236: i64) -> R<Option<i64>> {
    let m3234 = (p3235, p3236);
    {
        let mut m3234 = m3234;
        loop {
            let step = {
                let (v0, v1) = m3234;
                let m3230 = v0;
                match m3230.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m3231 = f811(v2);
                        match m3231 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f116({
                                    let a3232 = v1;
                                    let a3233 = 10i64;
                                    int_mul(a3232, a3233)?
                                }, f298(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f805(v0: Bytes) -> R<Bytes> {
    let (_, h3237) = f812(Fn5::F813(v0.clone()), {
        let a3238 = v0;
        length_bytes(a3238)
    }, (0u64, f89()))?;
    Ok(h3237)
}

pub fn f806(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f814(v0, {
        let a3239 = f815(v1, v2);
        let a3240 = v3;
        append_bytes(a3239, a3240)
    })
}

pub fn f807(v0: Fn4, p3244: u64, p3245: (u64, u64)) -> R<(u64, u64)> {
    let m3243 = (p3244, p3245);
    {
        let mut m3243 = m3243;
        loop {
            let step = {
                let (v1, v2) = m3243;
                let m3241 = v1;
                if m3241 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m3241 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c3242: Fn4 = v0.clone();
                        c3242.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3243 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f808(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3247 = {
            let (h3246, _) = v1;
            h3246
        };
        let a3248 = 1u64;
        nat_add(a3247, a3248)?
    }, {
        let a3251 = {
            let (_, h3249) = v1;
            h3249
        };
        let a3252 = {
            let a3250 = v0.clone();
            length_bytes(a3250)
        };
        if nat_eq(a3251, a3252) {
            let a3254 = f56(v0, {
                let (h3253, _) = v1;
                h3253
            })?;
            let a3255 = 46u64;
            if nat_eq(a3254, a3255) {
                let (h3256, _) = v1;
                h3256
            } else {
                let (_, h3257) = v1;
                h3257
            }
        } else {
            let (_, h3258) = v1;
            h3258
        }
    }))
}

pub fn f809(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m3259 = v3;
    match m3259 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m3260 = v4;
            match m3260 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f816(f806(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f810(v0: Bytes) -> R<Option<Bytes>> {
    let a3265 = {
        let a3262 = {
            let a3261 = v0.clone();
            length_bytes(a3261)
        };
        let a3263 = 2u64;
        let a3264 = 0u64;
        nat_rem(a3262, a3263, a3264)
    };
    let a3266 = 0u64;
    if nat_eq(a3265, a3266) {
        let v1: (u64, (Bytes, u64)) = f817(Fn6::F818(v0.clone()), {
            let a3268 = {
                let a3267 = v0;
                length_bytes(a3267)
            };
            let a3269 = 2u64;
            let a3270 = 0u64;
            nat_quot(a3268, a3269, a3270)
        }, (0u64, (f89(), 0u64)))?;
        let a3273 = {
            let (_, h3271) = {
                let (_, h3272) = v1.clone();
                h3272
            };
            h3271
        };
        let a3274 = 0u64;
        if nat_eq(a3273, a3274) {
            Ok(Some({
                let (h3275, _) = {
                    let (_, h3276) = v1;
                    h3276
                };
                h3275
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f811(v0: u64) -> Option<u64> {
    let a3277 = v0;
    let a3278 = 48u64;
    if if nat_lt(a3277, a3278) {
        true
    } else {
        let a3279 = 57u64;
        let a3280 = v0;
        nat_lt(a3279, a3280)
    } {
        None::<u64>
    } else {
        Some({
            let a3281 = v0;
            let a3282 = 48u64;
            nat_sub(a3281, a3282)
        })
    }
}

pub fn f812(v0: Fn5, p3286: u64, p3287: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3285 = (p3286, p3287);
    {
        let mut m3285 = m3285;
        loop {
            let step = {
                let (v1, v2) = m3285;
                let m3283 = v1;
                if m3283 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3283 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3284: Fn5 = v0.clone();
                        c3284.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f813(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3289 = {
            let (h3288, _) = v1.clone();
            h3288
        };
        let a3290 = 1u64;
        nat_add(a3289, a3290)?
    }, {
        let a3293 = {
            let (_, h3291) = v1;
            h3291
        };
        let a3294 = f819(f56(v0, {
            let (h3292, _) = v1;
            h3292
        })?)?;
        append_bytes(a3293, a3294)
    }))
}

pub fn f814(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f820(f821(v0, v1)?)
}

pub fn f815(v0: Bytes, v1: Bytes) -> Bytes {
    let a3301 = f822({
        let a3295 = v0.clone();
        length_bytes(a3295)
    });
    let a3302 = {
        let a3299 = v0;
        let a3300 = {
            let a3297 = f822({
                let a3296 = v1.clone();
                length_bytes(a3296)
            });
            let a3298 = v1;
            append_bytes(a3297, a3298)
        };
        append_bytes(a3299, a3300)
    };
    append_bytes(a3301, a3302)
}

pub fn f816(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3305 = {
        let a3303 = v0.clone();
        length_bytes(a3303)
    };
    let a3306 = {
        let a3304 = v1.clone();
        length_bytes(a3304)
    };
    if nat_eq(a3305, a3306) {
        let a3307 = f823(v0, v1)?;
        let a3308 = 0u64;
        Ok(nat_eq(a3307, a3308))
    } else {
        Ok(false)
    }
}

pub fn f817(v0: Fn6, p3312: u64, p3313: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3311 = (p3312, p3313);
    {
        let mut m3311 = m3311;
        loop {
            let step = {
                let (v1, v2) = m3311;
                let m3309 = v1;
                if m3309 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3309 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3310: Fn6 = v0.clone();
                        c3310.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f818(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f824(v0, v1)
}

pub fn f819(v0: u64) -> R<Bytes> {
    let a3320 = f825({
        let a3314 = v0;
        let a3315 = 16u64;
        let a3316 = 0u64;
        nat_quot(a3314, a3315, a3316)
    })?;
    let a3321 = f825({
        let a3317 = v0;
        let a3318 = 16u64;
        let a3319 = 0u64;
        nat_rem(a3317, a3318, a3319)
    })?;
    Ok(append_bytes(a3320, a3321))
}

pub fn f820(v0: Adt159) -> R<Bytes> {
    let a3342 = f826(match v0.clone() {
        Adt159::C0(h3322, _, _, _, _, _, _, _) => {
            h3322
        }
    })?;
    let a3343 = {
        let a3340 = f826(match v0.clone() {
            Adt159::C0(_, h3323, _, _, _, _, _, _) => {
                h3323
            }
        })?;
        let a3341 = {
            let a3338 = f826(match v0.clone() {
                Adt159::C0(_, _, h3324, _, _, _, _, _) => {
                    h3324
                }
            })?;
            let a3339 = {
                let a3336 = f826(match v0.clone() {
                    Adt159::C0(_, _, _, h3325, _, _, _, _) => {
                        h3325
                    }
                })?;
                let a3337 = {
                    let a3334 = f826(match v0.clone() {
                        Adt159::C0(_, _, _, _, h3326, _, _, _) => {
                            h3326
                        }
                    })?;
                    let a3335 = {
                        let a3332 = f826(match v0.clone() {
                            Adt159::C0(_, _, _, _, _, h3327, _, _) => {
                                h3327
                            }
                        })?;
                        let a3333 = {
                            let a3330 = f826(match v0.clone() {
                                Adt159::C0(_, _, _, _, _, _, h3328, _) => {
                                    h3328
                                }
                            })?;
                            let a3331 = f826(match v0 {
                                Adt159::C0(_, _, _, _, _, _, _, h3329) => {
                                    h3329
                                }
                            })?;
                            append_bytes(a3330, a3331)
                        };
                        append_bytes(a3332, a3333)
                    };
                    append_bytes(a3334, a3335)
                };
                append_bytes(a3336, a3337)
            };
            append_bytes(a3338, a3339)
        };
        append_bytes(a3340, a3341)
    };
    Ok(append_bytes(a3342, a3343))
}

pub fn f821(v0: Bytes, v1: Bytes) -> R<Adt159> {
    let v2: Adt160 = f827(v0)?;
    let v3: Adt159 = f828(f829(f830(), f831(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f829(f829(f830(), f831(v2, 1549556828u32))?, f832(v3))
}

pub fn f822(v0: u64) -> Bytes {
    let a3369 = f499({
        let a3347 = {
            let a3344 = v0;
            let a3345 = 16777216u64;
            let a3346 = 0u64;
            nat_quot(a3344, a3345, a3346)
        };
        let a3348 = 256u64;
        let a3349 = 0u64;
        nat_rem(a3347, a3348, a3349)
    });
    let a3370 = {
        let a3367 = f499({
            let a3353 = {
                let a3350 = v0;
                let a3351 = 65536u64;
                let a3352 = 0u64;
                nat_quot(a3350, a3351, a3352)
            };
            let a3354 = 256u64;
            let a3355 = 0u64;
            nat_rem(a3353, a3354, a3355)
        });
        let a3368 = {
            let a3365 = f499({
                let a3359 = {
                    let a3356 = v0;
                    let a3357 = 256u64;
                    let a3358 = 0u64;
                    nat_quot(a3356, a3357, a3358)
                };
                let a3360 = 256u64;
                let a3361 = 0u64;
                nat_rem(a3359, a3360, a3361)
            });
            let a3366 = f499({
                let a3362 = v0;
                let a3363 = 256u64;
                let a3364 = 0u64;
                nat_rem(a3362, a3363, a3364)
            });
            append_bytes(a3365, a3366)
        };
        append_bytes(a3367, a3368)
    };
    append_bytes(a3369, a3370)
}

pub fn f823(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3371) = f807(Fn4::F833(v0.clone(), v1), {
        let a3372 = v0;
        length_bytes(a3372)
    }, (0u64, 0u64))?;
    Ok(h3371)
}

pub fn f824(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f834(f56(v0.clone(), {
        let (h3373, _) = v1.clone();
        h3373
    })?);
    let v3: u64 = f834(f56(v0, {
        let a3375 = {
            let (h3374, _) = v1.clone();
            h3374
        };
        let a3376 = 1u64;
        nat_add(a3375, a3376)?
    })?);
    let a3377 = v2;
    let a3378 = 16u64;
    if if nat_lt(a3377, a3378) {
        let a3379 = v3;
        let a3380 = 16u64;
        nat_lt(a3379, a3380)
    } else {
        false
    } {
        Ok(({
            let a3382 = {
                let (h3381, _) = v1.clone();
                h3381
            };
            let a3383 = 2u64;
            nat_add(a3382, a3383)?
        }, ({
            let a3390 = {
                let (h3384, _) = {
                    let (_, h3385) = v1.clone();
                    h3385
                };
                h3384
            };
            let a3391 = f499({
                let a3388 = {
                    let a3386 = v2;
                    let a3387 = 16u64;
                    nat_mul(a3386, a3387)?
                };
                let a3389 = v3;
                nat_add(a3388, a3389)?
            });
            append_bytes(a3390, a3391)
        }, {
            let (_, h3392) = {
                let (_, h3393) = v1;
                h3393
            };
            h3392
        })))
    } else {
        Ok(({
            let a3395 = {
                let (h3394, _) = v1.clone();
                h3394
            };
            let a3396 = 2u64;
            nat_add(a3395, a3396)?
        }, ({
            let (h3397, _) = {
                let (_, h3398) = v1.clone();
                h3398
            };
            h3397
        }, {
            let a3401 = {
                let (_, h3399) = {
                    let (_, h3400) = v1;
                    h3400
                };
                h3399
            };
            let a3402 = 1u64;
            nat_add(a3401, a3402)?
        })))
    }
}

pub fn f825(v0: u64) -> R<Bytes> {
    Ok(f499({
        let a3403 = v0;
        let a3404 = 10u64;
        if nat_lt(a3403, a3404) {
            let a3405 = 48u64;
            let a3406 = v0;
            nat_add(a3405, a3406)?
        } else {
            let a3407 = 87u64;
            let a3408 = v0;
            nat_add(a3407, a3408)?
        }
    }))
}

pub fn f826(v0: u32) -> R<Bytes> {
    let a3413 = f835(v0, 24u32)?;
    let a3414 = {
        let a3411 = f835(v0, 16u32)?;
        let a3412 = {
            let a3409 = f835(v0, 8u32)?;
            let a3410 = f835(v0, 0u32)?;
            append_bytes(a3409, a3410)
        };
        append_bytes(a3411, a3412)
    };
    Ok(append_bytes(a3413, a3414))
}

pub fn f827(v0: Bytes) -> R<Adt160> {
    let a3416 = f836();
    let a3417 = {
        let a3415 = v0.clone();
        length_bytes(a3415)
    };
    if nat_lt(a3416, a3417) {
        Ok(f837(f838(v0)?))
    } else {
        f839(v0)
    }
}

pub fn f828(v0: Adt159, v1: u64, v2: Bytes) -> R<Adt159> {
    let v3: u64 = {
        let a3418 = v2.clone();
        length_bytes(a3418)
    };
    let v4: u64 = {
        let a3424 = {
            let a3421 = {
                let a3419 = v3;
                let a3420 = 72u64;
                nat_add(a3419, a3420)?
            };
            let a3422 = 64u64;
            let a3423 = 0u64;
            nat_quot(a3421, a3422, a3423)
        };
        let a3425 = 64u64;
        nat_mul(a3424, a3425)?
    };
    let (_, h3426) = f840(Fn7::F841(v2, v1, v3, v4), {
        let a3427 = v4;
        let a3428 = 64u64;
        let a3429 = 0u64;
        nat_quot(a3427, a3428, a3429)
    }, (0u64, v0))?;
    Ok(h3426)
}

pub fn f829(v0: Adt159, v1: Adt160) -> R<Adt159> {
    let v2: Adt161 = f842(Fn8::F843, 64u64, Adt161::C0(match v0.clone() {
        Adt159::C0(h3430, _, _, _, _, _, _, _) => {
            h3430
        }
    }, match v0.clone() {
        Adt159::C0(_, h3431, _, _, _, _, _, _) => {
            h3431
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3432, _, _, _, _, _) => {
            h3432
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3433, _, _, _, _) => {
            h3433
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3434, _, _, _) => {
            h3434
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3435, _, _) => {
            h3435
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3436, _) => {
            h3436
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, _, h3437) => {
            h3437
        }
    }, v1, 0u64))?;
    Ok(Adt159::C0(f844(match v0.clone() {
        Adt159::C0(h3438, _, _, _, _, _, _, _) => {
            h3438
        }
    }, match v2.clone() {
        Adt161::C0(h3439, _, _, _, _, _, _, _, _, _) => {
            h3439
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, h3440, _, _, _, _, _, _) => {
            h3440
        }
    }, match v2.clone() {
        Adt161::C0(_, h3441, _, _, _, _, _, _, _, _) => {
            h3441
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, h3442, _, _, _, _, _) => {
            h3442
        }
    }, match v2.clone() {
        Adt161::C0(_, _, h3443, _, _, _, _, _, _, _) => {
            h3443
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, h3444, _, _, _, _) => {
            h3444
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, h3445, _, _, _, _, _, _) => {
            h3445
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, h3446, _, _, _) => {
            h3446
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, h3447, _, _, _, _, _) => {
            h3447
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3448, _, _) => {
            h3448
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, h3449, _, _, _, _) => {
            h3449
        }
    }), f844(match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3450, _) => {
            h3450
        }
    }, match v2.clone() {
        Adt161::C0(_, _, _, _, _, _, h3451, _, _, _) => {
            h3451
        }
    }), f844(match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3452) => {
            h3452
        }
    }, match v2 {
        Adt161::C0(_, _, _, _, _, _, _, h3453, _, _) => {
            h3453
        }
    })))
}

pub fn f830() -> Adt159 {
    Adt159::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f831(v0: Adt160, v1: u32) -> Adt160 {
    Adt160::C0({
        let a3455 = match v0.clone() {
            Adt160::C0(h3454, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3454
            }
        };
        let a3456 = v1;
        fixed_u32::bit_xor(a3455, a3456)
    }, {
        let a3458 = match v0.clone() {
            Adt160::C0(_, h3457, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3457
            }
        };
        let a3459 = v1;
        fixed_u32::bit_xor(a3458, a3459)
    }, {
        let a3461 = match v0.clone() {
            Adt160::C0(_, _, h3460, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3460
            }
        };
        let a3462 = v1;
        fixed_u32::bit_xor(a3461, a3462)
    }, {
        let a3464 = match v0.clone() {
            Adt160::C0(_, _, _, h3463, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3463
            }
        };
        let a3465 = v1;
        fixed_u32::bit_xor(a3464, a3465)
    }, {
        let a3467 = match v0.clone() {
            Adt160::C0(_, _, _, _, h3466, _, _, _, _, _, _, _, _, _, _, _) => {
                h3466
            }
        };
        let a3468 = v1;
        fixed_u32::bit_xor(a3467, a3468)
    }, {
        let a3470 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, h3469, _, _, _, _, _, _, _, _, _, _) => {
                h3469
            }
        };
        let a3471 = v1;
        fixed_u32::bit_xor(a3470, a3471)
    }, {
        let a3473 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, h3472, _, _, _, _, _, _, _, _, _) => {
                h3472
            }
        };
        let a3474 = v1;
        fixed_u32::bit_xor(a3473, a3474)
    }, {
        let a3476 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, h3475, _, _, _, _, _, _, _, _) => {
                h3475
            }
        };
        let a3477 = v1;
        fixed_u32::bit_xor(a3476, a3477)
    }, {
        let a3479 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, h3478, _, _, _, _, _, _, _) => {
                h3478
            }
        };
        let a3480 = v1;
        fixed_u32::bit_xor(a3479, a3480)
    }, {
        let a3482 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, h3481, _, _, _, _, _, _) => {
                h3481
            }
        };
        let a3483 = v1;
        fixed_u32::bit_xor(a3482, a3483)
    }, {
        let a3485 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, h3484, _, _, _, _, _) => {
                h3484
            }
        };
        let a3486 = v1;
        fixed_u32::bit_xor(a3485, a3486)
    }, {
        let a3488 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h3487, _, _, _, _) => {
                h3487
            }
        };
        let a3489 = v1;
        fixed_u32::bit_xor(a3488, a3489)
    }, {
        let a3491 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3490, _, _, _) => {
                h3490
            }
        };
        let a3492 = v1;
        fixed_u32::bit_xor(a3491, a3492)
    }, {
        let a3494 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3493, _, _) => {
                h3493
            }
        };
        let a3495 = v1;
        fixed_u32::bit_xor(a3494, a3495)
    }, {
        let a3497 = match v0.clone() {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3496, _) => {
                h3496
            }
        };
        let a3498 = v1;
        fixed_u32::bit_xor(a3497, a3498)
    }, {
        let a3500 = match v0 {
            Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3499) => {
                h3499
            }
        };
        let a3501 = v1;
        fixed_u32::bit_xor(a3500, a3501)
    })
}

pub fn f832(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3502, _, _, _, _, _, _, _) => {
            h3502
        }
    }, match v0.clone() {
        Adt159::C0(_, h3503, _, _, _, _, _, _) => {
            h3503
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3504, _, _, _, _, _) => {
            h3504
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3505, _, _, _, _) => {
            h3505
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3506, _, _, _) => {
            h3506
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3507, _, _) => {
            h3507
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3508, _) => {
            h3508
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3509) => {
            h3509
        }
    }, 2147483648u32, f845(), f845(), f845(), f845(), f845(), f845(), 768u32)
}

pub fn f833(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3511 = {
            let (h3510, _) = v2;
            h3510
        };
        let a3512 = 1u64;
        nat_add(a3511, a3512)?
    }, {
        let a3564 = {
            let (_, h3513) = v2;
            h3513
        };
        let a3565 = {
            let v3: u8 = {
                let a3516 = f846(v0, {
                    let (h3514, _) = v2;
                    h3514
                });
                let a3517 = f846(v1, {
                    let (h3515, _) = v2;
                    h3515
                });
                fixed_u8::bit_xor(a3516, a3517)
            };
            let a3562 = {
                let a3538 = {
                    let a3526 = {
                        let a3520 = {
                            let a3518 = v3;
                            let a3519 = 128u8;
                            fixed_u8::bit_and(a3518, a3519)
                        };
                        let a3521 = 128u8;
                        if equal(a3520, a3521) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3527 = {
                        let a3524 = {
                            let a3522 = v3;
                            let a3523 = 64u8;
                            fixed_u8::bit_and(a3522, a3523)
                        };
                        let a3525 = 64u8;
                        if equal(a3524, a3525) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3526, a3527)?
                };
                let a3539 = {
                    let a3536 = {
                        let a3530 = {
                            let a3528 = v3;
                            let a3529 = 32u8;
                            fixed_u8::bit_and(a3528, a3529)
                        };
                        let a3531 = 32u8;
                        if equal(a3530, a3531) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3537 = {
                        let a3534 = {
                            let a3532 = v3;
                            let a3533 = 16u8;
                            fixed_u8::bit_and(a3532, a3533)
                        };
                        let a3535 = 16u8;
                        if equal(a3534, a3535) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3536, a3537)?
                };
                nat_add(a3538, a3539)?
            };
            let a3563 = {
                let a3560 = {
                    let a3548 = {
                        let a3542 = {
                            let a3540 = v3;
                            let a3541 = 8u8;
                            fixed_u8::bit_and(a3540, a3541)
                        };
                        let a3543 = 8u8;
                        if equal(a3542, a3543) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3549 = {
                        let a3546 = {
                            let a3544 = v3;
                            let a3545 = 4u8;
                            fixed_u8::bit_and(a3544, a3545)
                        };
                        let a3547 = 4u8;
                        if equal(a3546, a3547) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3548, a3549)?
                };
                let a3561 = {
                    let a3558 = {
                        let a3552 = {
                            let a3550 = v3;
                            let a3551 = 2u8;
                            fixed_u8::bit_and(a3550, a3551)
                        };
                        let a3553 = 2u8;
                        if equal(a3552, a3553) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3559 = {
                        let a3556 = {
                            let a3554 = v3;
                            let a3555 = 1u8;
                            fixed_u8::bit_and(a3554, a3555)
                        };
                        let a3557 = 1u8;
                        if equal(a3556, a3557) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3558, a3559)?
                };
                nat_add(a3560, a3561)?
            };
            nat_add(a3562, a3563)?
        };
        nat_add(a3564, a3565)?
    }))
}

pub fn f834(v0: u64) -> u64 {
    let a3566 = v0;
    let a3567 = 48u64;
    if nat_lt(a3566, a3567) {
        16u64
    } else {
        let a3568 = v0;
        let a3569 = 58u64;
        if nat_lt(a3568, a3569) {
            let a3570 = v0;
            let a3571 = 48u64;
            nat_sub(a3570, a3571)
        } else {
            let a3572 = v0;
            let a3573 = 97u64;
            if nat_lt(a3572, a3573) {
                16u64
            } else {
                let a3574 = v0;
                let a3575 = 103u64;
                if nat_lt(a3574, a3575) {
                    let a3576 = v0;
                    let a3577 = 87u64;
                    nat_sub(a3576, a3577)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f835(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f499({
        let v2: u32 = {
            let a3578 = f847(v0, v1);
            let a3579 = 255u32;
            fixed_u32::bit_and(a3578, a3579)
        };
        let a3768 = {
            let a3672 = {
                let a3624 = {
                    let a3600 = {
                        let a3588 = {
                            let a3582 = {
                                let a3580 = v2;
                                let a3581 = 2147483648u32;
                                fixed_u32::bit_and(a3580, a3581)
                            };
                            let a3583 = 2147483648u32;
                            if equal(a3582, a3583) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3589 = {
                            let a3586 = {
                                let a3584 = v2;
                                let a3585 = 1073741824u32;
                                fixed_u32::bit_and(a3584, a3585)
                            };
                            let a3587 = 1073741824u32;
                            if equal(a3586, a3587) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3588, a3589)?
                    };
                    let a3601 = {
                        let a3598 = {
                            let a3592 = {
                                let a3590 = v2;
                                let a3591 = 536870912u32;
                                fixed_u32::bit_and(a3590, a3591)
                            };
                            let a3593 = 536870912u32;
                            if equal(a3592, a3593) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3599 = {
                            let a3596 = {
                                let a3594 = v2;
                                let a3595 = 268435456u32;
                                fixed_u32::bit_and(a3594, a3595)
                            };
                            let a3597 = 268435456u32;
                            if equal(a3596, a3597) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3598, a3599)?
                    };
                    nat_add(a3600, a3601)?
                };
                let a3625 = {
                    let a3622 = {
                        let a3610 = {
                            let a3604 = {
                                let a3602 = v2;
                                let a3603 = 134217728u32;
                                fixed_u32::bit_and(a3602, a3603)
                            };
                            let a3605 = 134217728u32;
                            if equal(a3604, a3605) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3611 = {
                            let a3608 = {
                                let a3606 = v2;
                                let a3607 = 67108864u32;
                                fixed_u32::bit_and(a3606, a3607)
                            };
                            let a3609 = 67108864u32;
                            if equal(a3608, a3609) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3610, a3611)?
                    };
                    let a3623 = {
                        let a3620 = {
                            let a3614 = {
                                let a3612 = v2;
                                let a3613 = 33554432u32;
                                fixed_u32::bit_and(a3612, a3613)
                            };
                            let a3615 = 33554432u32;
                            if equal(a3614, a3615) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3621 = {
                            let a3618 = {
                                let a3616 = v2;
                                let a3617 = 16777216u32;
                                fixed_u32::bit_and(a3616, a3617)
                            };
                            let a3619 = 16777216u32;
                            if equal(a3618, a3619) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3620, a3621)?
                    };
                    nat_add(a3622, a3623)?
                };
                nat_add(a3624, a3625)?
            };
            let a3673 = {
                let a3670 = {
                    let a3646 = {
                        let a3634 = {
                            let a3628 = {
                                let a3626 = v2;
                                let a3627 = 8388608u32;
                                fixed_u32::bit_and(a3626, a3627)
                            };
                            let a3629 = 8388608u32;
                            if equal(a3628, a3629) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3635 = {
                            let a3632 = {
                                let a3630 = v2;
                                let a3631 = 4194304u32;
                                fixed_u32::bit_and(a3630, a3631)
                            };
                            let a3633 = 4194304u32;
                            if equal(a3632, a3633) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3634, a3635)?
                    };
                    let a3647 = {
                        let a3644 = {
                            let a3638 = {
                                let a3636 = v2;
                                let a3637 = 2097152u32;
                                fixed_u32::bit_and(a3636, a3637)
                            };
                            let a3639 = 2097152u32;
                            if equal(a3638, a3639) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3645 = {
                            let a3642 = {
                                let a3640 = v2;
                                let a3641 = 1048576u32;
                                fixed_u32::bit_and(a3640, a3641)
                            };
                            let a3643 = 1048576u32;
                            if equal(a3642, a3643) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3644, a3645)?
                    };
                    nat_add(a3646, a3647)?
                };
                let a3671 = {
                    let a3668 = {
                        let a3656 = {
                            let a3650 = {
                                let a3648 = v2;
                                let a3649 = 524288u32;
                                fixed_u32::bit_and(a3648, a3649)
                            };
                            let a3651 = 524288u32;
                            if equal(a3650, a3651) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3657 = {
                            let a3654 = {
                                let a3652 = v2;
                                let a3653 = 262144u32;
                                fixed_u32::bit_and(a3652, a3653)
                            };
                            let a3655 = 262144u32;
                            if equal(a3654, a3655) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3656, a3657)?
                    };
                    let a3669 = {
                        let a3666 = {
                            let a3660 = {
                                let a3658 = v2;
                                let a3659 = 131072u32;
                                fixed_u32::bit_and(a3658, a3659)
                            };
                            let a3661 = 131072u32;
                            if equal(a3660, a3661) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3667 = {
                            let a3664 = {
                                let a3662 = v2;
                                let a3663 = 65536u32;
                                fixed_u32::bit_and(a3662, a3663)
                            };
                            let a3665 = 65536u32;
                            if equal(a3664, a3665) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3666, a3667)?
                    };
                    nat_add(a3668, a3669)?
                };
                nat_add(a3670, a3671)?
            };
            nat_add(a3672, a3673)?
        };
        let a3769 = {
            let a3766 = {
                let a3718 = {
                    let a3694 = {
                        let a3682 = {
                            let a3676 = {
                                let a3674 = v2;
                                let a3675 = 32768u32;
                                fixed_u32::bit_and(a3674, a3675)
                            };
                            let a3677 = 32768u32;
                            if equal(a3676, a3677) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3683 = {
                            let a3680 = {
                                let a3678 = v2;
                                let a3679 = 16384u32;
                                fixed_u32::bit_and(a3678, a3679)
                            };
                            let a3681 = 16384u32;
                            if equal(a3680, a3681) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3682, a3683)?
                    };
                    let a3695 = {
                        let a3692 = {
                            let a3686 = {
                                let a3684 = v2;
                                let a3685 = 8192u32;
                                fixed_u32::bit_and(a3684, a3685)
                            };
                            let a3687 = 8192u32;
                            if equal(a3686, a3687) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3693 = {
                            let a3690 = {
                                let a3688 = v2;
                                let a3689 = 4096u32;
                                fixed_u32::bit_and(a3688, a3689)
                            };
                            let a3691 = 4096u32;
                            if equal(a3690, a3691) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3692, a3693)?
                    };
                    nat_add(a3694, a3695)?
                };
                let a3719 = {
                    let a3716 = {
                        let a3704 = {
                            let a3698 = {
                                let a3696 = v2;
                                let a3697 = 2048u32;
                                fixed_u32::bit_and(a3696, a3697)
                            };
                            let a3699 = 2048u32;
                            if equal(a3698, a3699) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3705 = {
                            let a3702 = {
                                let a3700 = v2;
                                let a3701 = 1024u32;
                                fixed_u32::bit_and(a3700, a3701)
                            };
                            let a3703 = 1024u32;
                            if equal(a3702, a3703) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3704, a3705)?
                    };
                    let a3717 = {
                        let a3714 = {
                            let a3708 = {
                                let a3706 = v2;
                                let a3707 = 512u32;
                                fixed_u32::bit_and(a3706, a3707)
                            };
                            let a3709 = 512u32;
                            if equal(a3708, a3709) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3715 = {
                            let a3712 = {
                                let a3710 = v2;
                                let a3711 = 256u32;
                                fixed_u32::bit_and(a3710, a3711)
                            };
                            let a3713 = 256u32;
                            if equal(a3712, a3713) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3714, a3715)?
                    };
                    nat_add(a3716, a3717)?
                };
                nat_add(a3718, a3719)?
            };
            let a3767 = {
                let a3764 = {
                    let a3740 = {
                        let a3728 = {
                            let a3722 = {
                                let a3720 = v2;
                                let a3721 = 128u32;
                                fixed_u32::bit_and(a3720, a3721)
                            };
                            let a3723 = 128u32;
                            if equal(a3722, a3723) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3729 = {
                            let a3726 = {
                                let a3724 = v2;
                                let a3725 = 64u32;
                                fixed_u32::bit_and(a3724, a3725)
                            };
                            let a3727 = 64u32;
                            if equal(a3726, a3727) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3728, a3729)?
                    };
                    let a3741 = {
                        let a3738 = {
                            let a3732 = {
                                let a3730 = v2;
                                let a3731 = 32u32;
                                fixed_u32::bit_and(a3730, a3731)
                            };
                            let a3733 = 32u32;
                            if equal(a3732, a3733) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3739 = {
                            let a3736 = {
                                let a3734 = v2;
                                let a3735 = 16u32;
                                fixed_u32::bit_and(a3734, a3735)
                            };
                            let a3737 = 16u32;
                            if equal(a3736, a3737) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3738, a3739)?
                    };
                    nat_add(a3740, a3741)?
                };
                let a3765 = {
                    let a3762 = {
                        let a3750 = {
                            let a3744 = {
                                let a3742 = v2;
                                let a3743 = 8u32;
                                fixed_u32::bit_and(a3742, a3743)
                            };
                            let a3745 = 8u32;
                            if equal(a3744, a3745) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3751 = {
                            let a3748 = {
                                let a3746 = v2;
                                let a3747 = 4u32;
                                fixed_u32::bit_and(a3746, a3747)
                            };
                            let a3749 = 4u32;
                            if equal(a3748, a3749) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3750, a3751)?
                    };
                    let a3763 = {
                        let a3760 = {
                            let a3754 = {
                                let a3752 = v2;
                                let a3753 = 2u32;
                                fixed_u32::bit_and(a3752, a3753)
                            };
                            let a3755 = 2u32;
                            if equal(a3754, a3755) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3761 = {
                            let a3758 = {
                                let a3756 = v2;
                                let a3757 = 1u32;
                                fixed_u32::bit_and(a3756, a3757)
                            };
                            let a3759 = 1u32;
                            if equal(a3758, a3759) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3760, a3761)?
                    };
                    nat_add(a3762, a3763)?
                };
                nat_add(a3764, a3765)?
            };
            nat_add(a3766, a3767)?
        };
        nat_add(a3768, a3769)?
    }))
}

pub fn f836() -> u64 {
    64u64
}

pub fn f837(v0: Adt159) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt159::C0(h3770, _, _, _, _, _, _, _) => {
            h3770
        }
    }, match v0.clone() {
        Adt159::C0(_, h3771, _, _, _, _, _, _) => {
            h3771
        }
    }, match v0.clone() {
        Adt159::C0(_, _, h3772, _, _, _, _, _) => {
            h3772
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, h3773, _, _, _, _) => {
            h3773
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, h3774, _, _, _) => {
            h3774
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, h3775, _, _) => {
            h3775
        }
    }, match v0.clone() {
        Adt159::C0(_, _, _, _, _, _, h3776, _) => {
            h3776
        }
    }, match v0 {
        Adt159::C0(_, _, _, _, _, _, _, h3777) => {
            h3777
        }
    }, f845(), f845(), f845(), f845(), f845(), f845(), f845(), f845())
}

pub fn f838(v0: Bytes) -> R<Adt159> {
    let v1: u64 = {
        let a3778 = v0.clone();
        length_bytes(a3778)
    };
    let v2: u64 = {
        let a3784 = {
            let a3781 = {
                let a3779 = v1;
                let a3780 = 72u64;
                nat_add(a3779, a3780)?
            };
            let a3782 = 64u64;
            let a3783 = 0u64;
            nat_quot(a3781, a3782, a3783)
        };
        let a3785 = 64u64;
        nat_mul(a3784, a3785)?
    };
    let (_, h3786) = f840(Fn7::F848(v0, v1, v2), {
        let a3787 = v2;
        let a3788 = 64u64;
        let a3789 = 0u64;
        nat_quot(a3787, a3788, a3789)
    }, (0u64, f830()))?;
    Ok(h3786)
}

pub fn f839(v0: Bytes) -> R<Adt160> {
    Ok(Adt160::C0(f849(v0.clone(), 0u64)?, f849(v0.clone(), 4u64)?, f849(v0.clone(), 8u64)?, f849(v0.clone(), 12u64)?, f849(v0.clone(), 16u64)?, f849(v0.clone(), 20u64)?, f849(v0.clone(), 24u64)?, f849(v0.clone(), 28u64)?, f849(v0.clone(), 32u64)?, f849(v0.clone(), 36u64)?, f849(v0.clone(), 40u64)?, f849(v0.clone(), 44u64)?, f849(v0.clone(), 48u64)?, f849(v0.clone(), 52u64)?, f849(v0.clone(), 56u64)?, f849(v0, 60u64)?))
}

pub fn f840(v0: Fn7, p3793: u64, p3794: (u64, Adt159)) -> R<(u64, Adt159)> {
    let m3792 = (p3793, p3794);
    {
        let mut m3792 = m3792;
        loop {
            let step = {
                let (v1, v2) = m3792;
                let m3790 = v1;
                if m3790 == 0 {
                    Err::<(u64, (u64, Adt159)), R<(u64, Adt159)>>(Ok(v2))
                } else {
                    let v3 = m3790 - 1;
                    Ok::<(u64, (u64, Adt159)), R<(u64, Adt159)>>((v3, {
                        let c3791: Fn7 = v0.clone();
                        c3791.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f841(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3796 = {
            let (h3795, _) = v4.clone();
            h3795
        };
        let a3797 = 64u64;
        nat_add(a3796, a3797)?
    }, f829({
        let (_, h3798) = v4;
        h3798
    }, f850(v0, v2, v3, {
        let a3801 = {
            let a3799 = v1;
            let a3800 = v2;
            nat_add(a3799, a3800)?
        };
        let a3802 = 8u64;
        nat_mul(a3801, a3802)?
    }, {
        let (h3803, _) = v4;
        h3803
    })?)?))
}

pub fn f842(v0: Fn8, p3807: u64, p3808: Adt161) -> R<Adt161> {
    let m3806 = (p3807, p3808);
    {
        let mut m3806 = m3806;
        loop {
            let step = {
                let (v1, v2) = m3806;
                let m3804 = v1;
                if m3804 == 0 {
                    Err::<(u64, Adt161), R<Adt161>>(Ok(v2))
                } else {
                    let v3 = m3804 - 1;
                    Ok::<(u64, Adt161), R<Adt161>>((v3, {
                        let c3805: Fn8 = v0.clone();
                        c3805.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f843(v0: Adt161) -> R<Adt161> {
    f851(v0)
}

pub fn f844(v0: u32, v1: u32) -> u32 {
    let m3811 = {
        let a3809 = f852(v0);
        let a3810 = f852(v1);
        fixed_u64::checked_add(a3809, a3810)
    };
    match m3811 {
        None => {
            0u32
        }
        Some(v2) => {
            f853(v2)
        }
    }
}

pub fn f845() -> u32 {
    0u32
}

pub fn f846(v0: Bytes, v1: u64) -> u8 {
    let m3814 = {
        let a3812 = v0;
        let a3813 = v1;
        index_bytes(a3812, a3813)
    };
    match m3814 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f847(v0: u32, v1: u32) -> u32 {
    let m3817 = {
        let a3815 = v0;
        let a3816 = v1;
        fixed_u32::shift_right(a3815, a3816)
    };
    match m3817 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f848(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt159)) -> R<(u64, Adt159)> {
    Ok(({
        let a3819 = {
            let (h3818, _) = v3.clone();
            h3818
        };
        let a3820 = 64u64;
        nat_add(a3819, a3820)?
    }, f829({
        let (_, h3821) = v3;
        h3821
    }, f854(v0, v1, v2, {
        let (h3822, _) = v3;
        h3822
    })?)?))
}

pub fn f849(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f855(f856(v0.clone(), v1), f856(v0.clone(), {
        let a3823 = v1;
        let a3824 = 1u64;
        nat_add(a3823, a3824)?
    }), f856(v0.clone(), {
        let a3825 = v1;
        let a3826 = 2u64;
        nat_add(a3825, a3826)?
    }), f856(v0, {
        let a3827 = v1;
        let a3828 = 3u64;
        nat_add(a3827, a3828)?
    })))
}

pub fn f850(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt160> {
    Ok(Adt160::C0(f857(v0.clone(), v1, v2, v3, v4)?, f857(v0.clone(), v1, v2, v3, {
        let a3829 = v4;
        let a3830 = 4u64;
        nat_add(a3829, a3830)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3831 = v4;
        let a3832 = 8u64;
        nat_add(a3831, a3832)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3833 = v4;
        let a3834 = 12u64;
        nat_add(a3833, a3834)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3835 = v4;
        let a3836 = 16u64;
        nat_add(a3835, a3836)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3837 = v4;
        let a3838 = 20u64;
        nat_add(a3837, a3838)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3839 = v4;
        let a3840 = 24u64;
        nat_add(a3839, a3840)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3841 = v4;
        let a3842 = 28u64;
        nat_add(a3841, a3842)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3843 = v4;
        let a3844 = 32u64;
        nat_add(a3843, a3844)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3845 = v4;
        let a3846 = 36u64;
        nat_add(a3845, a3846)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3847 = v4;
        let a3848 = 40u64;
        nat_add(a3847, a3848)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3849 = v4;
        let a3850 = 44u64;
        nat_add(a3849, a3850)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3851 = v4;
        let a3852 = 48u64;
        nat_add(a3851, a3852)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3853 = v4;
        let a3854 = 52u64;
        nat_add(a3853, a3854)?
    })?, f857(v0.clone(), v1, v2, v3, {
        let a3855 = v4;
        let a3856 = 56u64;
        nat_add(a3855, a3856)?
    })?, f857(v0, v1, v2, v3, {
        let a3857 = v4;
        let a3858 = 60u64;
        nat_add(a3857, a3858)?
    })?))
}

pub fn f851(v0: Adt161) -> R<Adt161> {
    let v1: u32 = f858(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, _, h3859) => {
            h3859
        }
    });
    let v2: u32 = f844(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, h3860, _, _) => {
            h3860
        }
    }, f844(f859(match v0.clone() {
        Adt161::C0(_, _, _, _, h3861, _, _, _, _, _) => {
            h3861
        }
    }), f844(f860(match v0.clone() {
        Adt161::C0(_, _, _, _, h3862, _, _, _, _, _) => {
            h3862
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3863, _, _, _, _) => {
            h3863
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3864, _, _, _) => {
            h3864
        }
    }), f844(v1, {
        let m3867 = match v0.clone() {
            Adt161::C0(_, _, _, _, _, _, _, _, h3866, _) => {
                h3866
            }
        };
        match m3867 {
            Adt160::C0(h3865, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3865
            }
        }
    }))));
    let v3: u32 = f844(f861(match v0.clone() {
        Adt161::C0(h3868, _, _, _, _, _, _, _, _, _) => {
            h3868
        }
    }), f862(match v0.clone() {
        Adt161::C0(h3869, _, _, _, _, _, _, _, _, _) => {
            h3869
        }
    }, match v0.clone() {
        Adt161::C0(_, h3870, _, _, _, _, _, _, _, _) => {
            h3870
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3871, _, _, _, _, _, _, _) => {
            h3871
        }
    }));
    Ok(Adt161::C0(f844(v2, v3), match v0.clone() {
        Adt161::C0(h3872, _, _, _, _, _, _, _, _, _) => {
            h3872
        }
    }, match v0.clone() {
        Adt161::C0(_, h3873, _, _, _, _, _, _, _, _) => {
            h3873
        }
    }, match v0.clone() {
        Adt161::C0(_, _, h3874, _, _, _, _, _, _, _) => {
            h3874
        }
    }, f844(match v0.clone() {
        Adt161::C0(_, _, _, h3875, _, _, _, _, _, _) => {
            h3875
        }
    }, v2), match v0.clone() {
        Adt161::C0(_, _, _, _, h3876, _, _, _, _, _) => {
            h3876
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, h3877, _, _, _, _) => {
            h3877
        }
    }, match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, h3878, _, _, _) => {
            h3878
        }
    }, f863(match v0.clone() {
        Adt161::C0(_, _, _, _, _, _, _, _, h3879, _) => {
            h3879
        }
    }), {
        let a3881 = match v0 {
            Adt161::C0(_, _, _, _, _, _, _, _, _, h3880) => {
                h3880
            }
        };
        let a3882 = 1u64;
        nat_add(a3881, a3882)?
    }))
}

pub fn f852(v0: u32) -> u64 {
    let m3884 = {
        let a3883 = v0;
        fixed_u64::convert(i128::from(a3883))
    };
    match m3884 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f853(v0: u64) -> u32 {
    let m3888 = {
        let a3887 = {
            let a3885 = v0;
            let a3886 = 4294967295u64;
            fixed_u64::bit_and(a3885, a3886)
        };
        fixed_u32::convert(i128::from(a3887))
    };
    match m3888 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f854(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt160> {
    Ok(Adt160::C0(f864(v0.clone(), v1, v2, v3)?, f864(v0.clone(), v1, v2, {
        let a3889 = v3;
        let a3890 = 4u64;
        nat_add(a3889, a3890)?
    })?, f864(v0.clone(), v1, v2, {
        let a3891 = v3;
        let a3892 = 8u64;
        nat_add(a3891, a3892)?
    })?, f864(v0.clone(), v1, v2, {
        let a3893 = v3;
        let a3894 = 12u64;
        nat_add(a3893, a3894)?
    })?, f864(v0.clone(), v1, v2, {
        let a3895 = v3;
        let a3896 = 16u64;
        nat_add(a3895, a3896)?
    })?, f864(v0.clone(), v1, v2, {
        let a3897 = v3;
        let a3898 = 20u64;
        nat_add(a3897, a3898)?
    })?, f864(v0.clone(), v1, v2, {
        let a3899 = v3;
        let a3900 = 24u64;
        nat_add(a3899, a3900)?
    })?, f864(v0.clone(), v1, v2, {
        let a3901 = v3;
        let a3902 = 28u64;
        nat_add(a3901, a3902)?
    })?, f864(v0.clone(), v1, v2, {
        let a3903 = v3;
        let a3904 = 32u64;
        nat_add(a3903, a3904)?
    })?, f864(v0.clone(), v1, v2, {
        let a3905 = v3;
        let a3906 = 36u64;
        nat_add(a3905, a3906)?
    })?, f864(v0.clone(), v1, v2, {
        let a3907 = v3;
        let a3908 = 40u64;
        nat_add(a3907, a3908)?
    })?, f864(v0.clone(), v1, v2, {
        let a3909 = v3;
        let a3910 = 44u64;
        nat_add(a3909, a3910)?
    })?, f864(v0.clone(), v1, v2, {
        let a3911 = v3;
        let a3912 = 48u64;
        nat_add(a3911, a3912)?
    })?, f864(v0.clone(), v1, v2, {
        let a3913 = v3;
        let a3914 = 52u64;
        nat_add(a3913, a3914)?
    })?, f864(v0.clone(), v1, v2, {
        let a3915 = v3;
        let a3916 = 56u64;
        nat_add(a3915, a3916)?
    })?, f864(v0, v1, v2, {
        let a3917 = v3;
        let a3918 = 60u64;
        nat_add(a3917, a3918)?
    })?))
}

pub fn f855(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3923 = f865(v0, 24u32);
    let a3924 = {
        let a3921 = f865(v1, 16u32);
        let a3922 = {
            let a3919 = f865(v2, 8u32);
            let a3920 = v3;
            fixed_u32::bit_or(a3919, a3920)
        };
        fixed_u32::bit_or(a3921, a3922)
    };
    fixed_u32::bit_or(a3923, a3924)
}

pub fn f856(v0: Bytes, v1: u64) -> u32 {
    let m3927 = {
        let a3925 = v0;
        let a3926 = v1;
        index_bytes(a3925, a3926)
    };
    match m3927 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3929 = {
                let a3928 = v2;
                fixed_u32::convert(i128::from(a3928))
            };
            match m3929 {
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

pub fn f857(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3932 = {
        let a3930 = v4;
        let a3931 = 8u64;
        nat_add(a3930, a3931)?
    };
    let a3933 = v2;
    if nat_eq(a3932, a3933) {
        Ok(f866(v3, true))
    } else {
        let a3936 = {
            let a3934 = v4;
            let a3935 = 4u64;
            nat_add(a3934, a3935)?
        };
        let a3937 = v2;
        if nat_eq(a3936, a3937) {
            Ok(f866(v3, false))
        } else {
            Ok(f855(f867(v0.clone(), v1, v4), f867(v0.clone(), v1, {
                let a3938 = v4;
                let a3939 = 1u64;
                nat_add(a3938, a3939)?
            }), f867(v0.clone(), v1, {
                let a3940 = v4;
                let a3941 = 2u64;
                nat_add(a3940, a3941)?
            }), f867(v0, v1, {
                let a3942 = v4;
                let a3943 = 3u64;
                nat_add(a3942, a3943)?
            })))
        }
    }
}

pub fn f858(v0: u64) -> u32 {
    let a3944 = v0;
    let a3945 = 32u64;
    if nat_lt(a3944, a3945) {
        let a3946 = v0;
        let a3947 = 16u64;
        if nat_lt(a3946, a3947) {
            let a3948 = v0;
            let a3949 = 8u64;
            if nat_lt(a3948, a3949) {
                let a3950 = v0;
                let a3951 = 4u64;
                if nat_lt(a3950, a3951) {
                    let a3952 = v0;
                    let a3953 = 2u64;
                    if nat_lt(a3952, a3953) {
                        let a3954 = v0;
                        let a3955 = 1u64;
                        if nat_lt(a3954, a3955) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3956 = v0;
                        let a3957 = 3u64;
                        if nat_lt(a3956, a3957) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3958 = v0;
                    let a3959 = 6u64;
                    if nat_lt(a3958, a3959) {
                        let a3960 = v0;
                        let a3961 = 5u64;
                        if nat_lt(a3960, a3961) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3962 = v0;
                        let a3963 = 7u64;
                        if nat_lt(a3962, a3963) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3964 = v0;
                let a3965 = 12u64;
                if nat_lt(a3964, a3965) {
                    let a3966 = v0;
                    let a3967 = 10u64;
                    if nat_lt(a3966, a3967) {
                        let a3968 = v0;
                        let a3969 = 9u64;
                        if nat_lt(a3968, a3969) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3970 = v0;
                        let a3971 = 11u64;
                        if nat_lt(a3970, a3971) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3972 = v0;
                    let a3973 = 14u64;
                    if nat_lt(a3972, a3973) {
                        let a3974 = v0;
                        let a3975 = 13u64;
                        if nat_lt(a3974, a3975) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3976 = v0;
                        let a3977 = 15u64;
                        if nat_lt(a3976, a3977) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3978 = v0;
            let a3979 = 24u64;
            if nat_lt(a3978, a3979) {
                let a3980 = v0;
                let a3981 = 20u64;
                if nat_lt(a3980, a3981) {
                    let a3982 = v0;
                    let a3983 = 18u64;
                    if nat_lt(a3982, a3983) {
                        let a3984 = v0;
                        let a3985 = 17u64;
                        if nat_lt(a3984, a3985) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3986 = v0;
                        let a3987 = 19u64;
                        if nat_lt(a3986, a3987) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3988 = v0;
                    let a3989 = 22u64;
                    if nat_lt(a3988, a3989) {
                        let a3990 = v0;
                        let a3991 = 21u64;
                        if nat_lt(a3990, a3991) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3992 = v0;
                        let a3993 = 23u64;
                        if nat_lt(a3992, a3993) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3994 = v0;
                let a3995 = 28u64;
                if nat_lt(a3994, a3995) {
                    let a3996 = v0;
                    let a3997 = 26u64;
                    if nat_lt(a3996, a3997) {
                        let a3998 = v0;
                        let a3999 = 25u64;
                        if nat_lt(a3998, a3999) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4000 = v0;
                        let a4001 = 27u64;
                        if nat_lt(a4000, a4001) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4002 = v0;
                    let a4003 = 30u64;
                    if nat_lt(a4002, a4003) {
                        let a4004 = v0;
                        let a4005 = 29u64;
                        if nat_lt(a4004, a4005) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4006 = v0;
                        let a4007 = 31u64;
                        if nat_lt(a4006, a4007) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4008 = v0;
        let a4009 = 48u64;
        if nat_lt(a4008, a4009) {
            let a4010 = v0;
            let a4011 = 40u64;
            if nat_lt(a4010, a4011) {
                let a4012 = v0;
                let a4013 = 36u64;
                if nat_lt(a4012, a4013) {
                    let a4014 = v0;
                    let a4015 = 34u64;
                    if nat_lt(a4014, a4015) {
                        let a4016 = v0;
                        let a4017 = 33u64;
                        if nat_lt(a4016, a4017) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4018 = v0;
                        let a4019 = 35u64;
                        if nat_lt(a4018, a4019) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4020 = v0;
                    let a4021 = 38u64;
                    if nat_lt(a4020, a4021) {
                        let a4022 = v0;
                        let a4023 = 37u64;
                        if nat_lt(a4022, a4023) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4024 = v0;
                        let a4025 = 39u64;
                        if nat_lt(a4024, a4025) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4026 = v0;
                let a4027 = 44u64;
                if nat_lt(a4026, a4027) {
                    let a4028 = v0;
                    let a4029 = 42u64;
                    if nat_lt(a4028, a4029) {
                        let a4030 = v0;
                        let a4031 = 41u64;
                        if nat_lt(a4030, a4031) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4032 = v0;
                        let a4033 = 43u64;
                        if nat_lt(a4032, a4033) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4034 = v0;
                    let a4035 = 46u64;
                    if nat_lt(a4034, a4035) {
                        let a4036 = v0;
                        let a4037 = 45u64;
                        if nat_lt(a4036, a4037) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4038 = v0;
                        let a4039 = 47u64;
                        if nat_lt(a4038, a4039) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4040 = v0;
            let a4041 = 56u64;
            if nat_lt(a4040, a4041) {
                let a4042 = v0;
                let a4043 = 52u64;
                if nat_lt(a4042, a4043) {
                    let a4044 = v0;
                    let a4045 = 50u64;
                    if nat_lt(a4044, a4045) {
                        let a4046 = v0;
                        let a4047 = 49u64;
                        if nat_lt(a4046, a4047) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4048 = v0;
                        let a4049 = 51u64;
                        if nat_lt(a4048, a4049) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4050 = v0;
                    let a4051 = 54u64;
                    if nat_lt(a4050, a4051) {
                        let a4052 = v0;
                        let a4053 = 53u64;
                        if nat_lt(a4052, a4053) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4054 = v0;
                        let a4055 = 55u64;
                        if nat_lt(a4054, a4055) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4056 = v0;
                let a4057 = 60u64;
                if nat_lt(a4056, a4057) {
                    let a4058 = v0;
                    let a4059 = 58u64;
                    if nat_lt(a4058, a4059) {
                        let a4060 = v0;
                        let a4061 = 57u64;
                        if nat_lt(a4060, a4061) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4062 = v0;
                        let a4063 = 59u64;
                        if nat_lt(a4062, a4063) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4064 = v0;
                    let a4065 = 62u64;
                    if nat_lt(a4064, a4065) {
                        let a4066 = v0;
                        let a4067 = 61u64;
                        if nat_lt(a4066, a4067) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4068 = v0;
                        let a4069 = 63u64;
                        if nat_lt(a4068, a4069) {
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

pub fn f859(v0: u32) -> u32 {
    let a4072 = {
        let a4070 = f868(v0, 6u32, 26u32);
        let a4071 = f868(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4070, a4071)
    };
    let a4073 = f868(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4072, a4073)
}

pub fn f860(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4079 = {
        let a4074 = v0;
        let a4075 = v1;
        fixed_u32::bit_and(a4074, a4075)
    };
    let a4080 = {
        let a4077 = {
            let a4076 = v0;
            fixed_u32::bit_not(a4076)
        };
        let a4078 = v2;
        fixed_u32::bit_and(a4077, a4078)
    };
    fixed_u32::bit_xor(a4079, a4080)
}

pub fn f861(v0: u32) -> u32 {
    let a4083 = {
        let a4081 = f868(v0, 2u32, 30u32);
        let a4082 = f868(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4081, a4082)
    };
    let a4084 = f868(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4083, a4084)
}

pub fn f862(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4093 = {
        let a4089 = {
            let a4085 = v0;
            let a4086 = v1;
            fixed_u32::bit_and(a4085, a4086)
        };
        let a4090 = {
            let a4087 = v0;
            let a4088 = v2;
            fixed_u32::bit_and(a4087, a4088)
        };
        fixed_u32::bit_xor(a4089, a4090)
    };
    let a4094 = {
        let a4091 = v1;
        let a4092 = v2;
        fixed_u32::bit_and(a4091, a4092)
    };
    fixed_u32::bit_xor(a4093, a4094)
}

pub fn f863(v0: Adt160) -> Adt160 {
    Adt160::C0(match v0.clone() {
        Adt160::C0(_, h4095, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4095
        }
    }, match v0.clone() {
        Adt160::C0(_, _, h4096, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4096
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, h4097, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4097
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, h4098, _, _, _, _, _, _, _, _, _, _, _) => {
            h4098
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, h4099, _, _, _, _, _, _, _, _, _, _) => {
            h4099
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, h4100, _, _, _, _, _, _, _, _, _) => {
            h4100
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, h4101, _, _, _, _, _, _, _, _) => {
            h4101
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, h4102, _, _, _, _, _, _, _) => {
            h4102
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4103, _, _, _, _, _, _) => {
            h4103
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, h4104, _, _, _, _, _) => {
            h4104
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, h4105, _, _, _, _) => {
            h4105
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4106, _, _, _) => {
            h4106
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4107, _, _) => {
            h4107
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4108, _) => {
            h4108
        }
    }, match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4109) => {
            h4109
        }
    }, f844(f869(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4110, _) => {
            h4110
        }
    }), f844(match v0.clone() {
        Adt160::C0(_, _, _, _, _, _, _, _, _, h4111, _, _, _, _, _, _) => {
            h4111
        }
    }, f844(f870(match v0.clone() {
        Adt160::C0(_, h4112, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4112
        }
    }), match v0 {
        Adt160::C0(h4113, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4113
        }
    }))))
}

pub fn f864(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a4116 = {
        let a4114 = v3;
        let a4115 = 8u64;
        nat_add(a4114, a4115)?
    };
    let a4117 = v2;
    if nat_eq(a4116, a4117) {
        Ok(f871({
            let a4120 = {
                let a4118 = v1;
                let a4119 = 8u64;
                nat_mul(a4118, a4119)?
            };
            let a4121 = 4294967296u64;
            let a4122 = 0u64;
            nat_quot(a4120, a4121, a4122)
        }))
    } else {
        let a4125 = {
            let a4123 = v3;
            let a4124 = 4u64;
            nat_add(a4123, a4124)?
        };
        let a4126 = v2;
        if nat_eq(a4125, a4126) {
            Ok(f871({
                let a4129 = {
                    let a4127 = v1;
                    let a4128 = 8u64;
                    nat_mul(a4127, a4128)?
                };
                let a4130 = 4294967296u64;
                let a4131 = 0u64;
                nat_rem(a4129, a4130, a4131)
            }))
        } else {
            Ok(f855(f867(v0.clone(), v1, v3), f867(v0.clone(), v1, {
                let a4132 = v3;
                let a4133 = 1u64;
                nat_add(a4132, a4133)?
            }), f867(v0.clone(), v1, {
                let a4134 = v3;
                let a4135 = 2u64;
                nat_add(a4134, a4135)?
            }), f867(v0, v1, {
                let a4136 = v3;
                let a4137 = 3u64;
                nat_add(a4136, a4137)?
            })))
        }
    }
}

pub fn f865(v0: u32, v1: u32) -> u32 {
    let m4140 = {
        let a4138 = v0;
        let a4139 = v1;
        fixed_u32::shift_left(a4138, a4139)
    };
    match m4140 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f866(v0: u64, v1: bool) -> u32 {
    if v1 {
        f871({
            let a4141 = v0;
            let a4142 = 4294967296u64;
            let a4143 = 0u64;
            nat_quot(a4141, a4142, a4143)
        })
    } else {
        f871({
            let a4144 = v0;
            let a4145 = 4294967296u64;
            let a4146 = 0u64;
            nat_rem(a4144, a4145, a4146)
        })
    }
}

pub fn f867(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a4147 = v2;
    let a4148 = v1;
    if nat_lt(a4147, a4148) {
        let m4151 = {
            let a4149 = v0;
            let a4150 = v2;
            index_bytes(a4149, a4150)
        };
        match m4151 {
            None => {
                0u32
            }
            Some(v3) => {
                let m4153 = {
                    let a4152 = v3;
                    fixed_u32::convert(i128::from(a4152))
                };
                match m4153 {
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
        let a4154 = v2;
        let a4155 = v1;
        if nat_eq(a4154, a4155) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f868(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4156 = f847(v0, v1);
    let a4157 = f865(v0, v2);
    fixed_u32::bit_or(a4156, a4157)
}

pub fn f869(v0: u32) -> u32 {
    let a4160 = {
        let a4158 = f868(v0, 17u32, 15u32);
        let a4159 = f868(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4158, a4159)
    };
    let a4161 = f847(v0, 10u32);
    fixed_u32::bit_xor(a4160, a4161)
}

pub fn f870(v0: u32) -> u32 {
    let a4164 = {
        let a4162 = f868(v0, 7u32, 25u32);
        let a4163 = f868(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4162, a4163)
    };
    let a4165 = f847(v0, 3u32);
    fixed_u32::bit_xor(a4164, a4165)
}

pub fn f871(v0: u64) -> u32 {
    let (_, h4166) = {
        let (_, h4167) = f872(Fn9::F873, 32u64, (v0, (1u32, 0u32)));
        h4167
    };
    h4166
}

pub fn f872(v0: Fn9, p4171: u64, p4172: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4170 = (p4171, p4172);
    {
        let mut m4170 = m4170;
        loop {
            let step = {
                let (v1, v2) = m4170;
                let m4168 = v1;
                if m4168 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4168 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4169: Fn9 = v0.clone();
                        c4169.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f873(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4173, _) = v0;
        h4173
    };
    let v2: u32 = {
        let (h4174, _) = {
            let (_, h4175) = v0;
            h4175
        };
        h4174
    };
    let v3: u32 = {
        let (_, h4176) = {
            let (_, h4177) = v0;
            h4177
        };
        h4176
    };
    ({
        let a4178 = v1;
        let a4179 = 2u64;
        let a4180 = 0u64;
        nat_quot(a4178, a4179, a4180)
    }, (f865(v2, 1u32), {
        let a4184 = {
            let a4181 = v1;
            let a4182 = 2u64;
            let a4183 = 0u64;
            nat_rem(a4181, a4182, a4183)
        };
        let a4185 = 1u64;
        if nat_eq(a4184, a4185) {
            let a4186 = v3;
            let a4187 = v2;
            fixed_u32::bit_or(a4186, a4187)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
