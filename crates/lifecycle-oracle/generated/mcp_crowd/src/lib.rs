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
    C10,
    C11,
    C12(Str),
    C13,
    C14,
}

#[derive(Clone)]
pub enum Adt85 {
    C0(List<u64>, List<u64>, List<Adt86>, Bytes, u64, List<List<u64>>, Bytes),
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
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt91 {
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
pub enum Adt92 {
    C0(Str),
    C1(i64),
}

#[derive(Clone)]
pub enum Adt93 {
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
pub enum Adt94 {
    C0,
    C1(Option<Adt92>, Adt95),
}

#[derive(Clone)]
pub enum Adt95 {
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
pub enum Adt96 {
    C0,
    C1,
    C2,
    C3(List<u64>),
}

#[derive(Clone)]
pub enum Adt97 {
    C0,
    C1(List<u64>),
    C2,
}

#[derive(Clone)]
pub enum Adt98 {
    C0(Adt92, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1(List<Adt101>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt101 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt102 {
    C0,
    C1(List<Adt101>, Str),
}

#[derive(Clone)]
pub enum Adt103 {
    C0(Adt6, Adt6, List<Adt101>),
    C1(List<Adt6>, Adt6, List<Adt101>),
    C2(List<Adt6>, Adt6, List<Adt101>, u64),
    C3(List<Adt101>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt101>),
    C5(Adt104, Adt6, Adt6, List<Adt101>),
}

#[derive(Clone)]
pub enum Adt104 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt105 {
    C0(List<u64>),
    C1(u64, Adt6),
    C2(u64, List<Adt6>),
    C3(u64, List<Adt6>),
    C4(u64, List<(Str, Adt6)>),
    C5(u64, List<(Str, Adt6)>),
}

#[derive(Clone)]
pub enum Adt106 {
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt107>, List<Adt109>),
}

#[derive(Clone)]
pub enum Adt107 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt108),
    C7(Adt108),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt109 {
    C0(Str, List<Str>, Adt13, Adt110),
}

#[derive(Clone)]
pub enum Adt110 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1(u64),
    C2(u64),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt113 {
    C0(Adt114, List<Adt118>, u64),
}

#[derive(Clone)]
pub enum Adt114 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt115, List<u64>, bool),
    C7(u64, u64),
    C8(Adt116),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt116 {
    C0(bool, Adt117, i64, u64, i64, bool, i64),
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
}

#[derive(Clone)]
pub enum Adt118 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt119 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0(Adt102, List<Adt103>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Adt92, Result<Adt122, Adt125>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Adt32, Adt123, List<Adt124>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Adt126),
    C1(Str, List<Str>),
    C2(Adt107),
    C3(Str),
    C4(Adt84),
}

#[derive(Clone)]
pub enum Adt126 {
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
pub enum Adt127 {
    C0(Option<Str>, Option<Adt107>),
}

#[derive(Clone)]
pub enum Adt128 {
    C0,
    C1(Adt92),
    C2,
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt130 {
    C0(Adt93),
    C1(Adt113),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt103>),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Adt93),
    C1(List<Adt105>, List<u64>),
}

#[derive(Clone)]
pub enum Adt133 {
    C0(Adt92, Adt134, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt134 {
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
pub enum Adt135 {
    C0,
    C1,
    C2(Adt6),
}

#[derive(Clone)]
pub enum Adt136 {
    C0,
    C1(Adt134),
}

#[derive(Clone)]
pub enum Adt137 {
    C0(Adt138),
    C1(Adt125),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt92, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt139 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt141 {
    C0(Adt91),
    C1(Adt119),
}

#[derive(Clone)]
pub enum Adt142 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt143 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt144 {
    C0(Adt6),
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt145 {
    C0(List<Adt6>, List<Adt6>),
    C1(Str, List<(Str, Adt6)>, List<(Str, Adt6)>),
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
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt149 {
    C0,
    C1(List<(Adt6, Adt6)>),
}

#[derive(Clone)]
pub enum Adt150 {
    C0(u64, List<u64>),
}

#[derive(Clone)]
pub enum Adt151 {
    C0,
    C1(Adt18),
}

#[derive(Clone)]
pub enum Adt152 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt153 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt154 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt153, u64),
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
    F32,
}

impl Fn1 {
    pub fn apply(&self, p0: Adt21) -> Result<Adt49, Adt84> {
        match self {
            Fn1::F32 => f32(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F82(Fn1, Adt35),
}

impl Fn2 {
    pub fn apply(&self, p0: Adt21) -> R<Result<Adt32, Adt84>> {
        match self {
            Fn2::F82(k0, k1) => f82(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F741(Bytes),
    F766(Bytes, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn3::F741(k0) => f741(k0.clone(), p0),
            Fn3::F766(k0, k1) => f766(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F746(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn4::F746(k0) => f746(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F751(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn5::F751(k0) => f751(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F774(Bytes, u64, u64, u64),
    F781(Bytes, u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Adt152)) -> R<(u64, Adt152)> {
        match self {
            Fn6::F774(k0, k1, k2, k3) => f774(k0.clone(), *k1, *k2, *k3, p0),
            Fn6::F781(k0, k1, k2) => f781(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F776,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt154) -> R<Adt154> {
        match self {
            Fn7::F776 => f776(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F806,
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn8::F806 => f806(p0),
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
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt85::C0(_, _, h27, _, _, _, _) => {
            h27
        }
    }, match v2 {
        Adt85::C0(_, _, _, _, _, h28, _) => {
            h28
        }
    })?)
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
    let m29 = f26(v0)?;
    match m29 {
        Ok(v1) => {
            Ok(v1)
        }
        Err(_) => {
            Ok(List::<Adt12>::nil())
        }
    }
}

pub fn f12() -> R<List<Adt35>> {
    f27(f28(), List::<Adt35>::nil())
}

pub fn f13() -> u64 {
    100u64
}

pub fn f14(v0: List<Adt35>) -> R<Option<Adt84>> {
    let m30 = f26(v0)?;
    match m30 {
        Ok(_) => {
            Ok(None::<Adt84>)
        }
        Err(v1) => {
            Ok(Some(Adt84::C9(f29(v1))))
        }
    }
}

pub fn f15(v0: Adt14) -> R<Result<Adt32, Adt84>> {
    let m31 = v0;
    match m31 {
        Adt14::C0(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the crowd offers tools only"))))
        }
        Adt14::C1(v1) => {
            Ok(f30(f10(), f12()?, v1))
        }
        Adt14::C2(v2) => {
            f31(f12()?, Fn1::F32, v2)
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
    let m32 = f33(v0.clone(), v1)?;
    match m32 {
        None => {
            Ok(None::<Adt89>)
        }
        Some(v2) => {
            let m38 = {
                let a35 = v0;
                let a36 = {
                    let a33 = v1;
                    let a34 = 4u64;
                    nat_add(a33, a34)?
                };
                let a37 = v2;
                slice_bytes(a35, a36, a37)
            };
            match m38 {
                None => {
                    Ok(None::<Adt89>)
                }
                Some(v3) => {
                    Ok(Some(Adt89::C0(v3, {
                        let a41 = {
                            let a39 = v1;
                            let a40 = 4u64;
                            nat_add(a39, a40)?
                        };
                        let a42 = v2;
                        nat_add(a41, a42)?
                    })))
                }
            }
        }
    }
}

pub fn f17(v0: Bytes, v1: Adt89, v2: Option<Adt89>) -> R<Result<Adt85, Adt87>> {
    let m43 = v2;
    match m43 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v1 {
                Adt89::C0(_, h44) => {
                    h44
                }
            })))
        }
        Some(v3) => {
            f34(v0.clone(), v1, v3.clone(), f33(v0, match v3 {
                Adt89::C0(_, h45) => {
                    h45
                }
            })?)
        }
    }
}

pub fn f18(v0: u64) -> List<u64> {
    List::cons({
        let a49 = {
            let a46 = v0;
            let a47 = 16777216u64;
            let a48 = 0u64;
            nat_quot(a46, a47, a48)
        };
        let a50 = 256u64;
        let a51 = 0u64;
        nat_rem(a49, a50, a51)
    }, List::cons({
        let a55 = {
            let a52 = v0;
            let a53 = 65536u64;
            let a54 = 0u64;
            nat_quot(a52, a53, a54)
        };
        let a56 = 256u64;
        let a57 = 0u64;
        nat_rem(a55, a56, a57)
    }, List::cons({
        let a61 = {
            let a58 = v0;
            let a59 = 256u64;
            let a60 = 0u64;
            nat_quot(a58, a59, a60)
        };
        let a62 = 256u64;
        let a63 = 0u64;
        nat_rem(a61, a62, a63)
    }, List::cons({
        let a64 = v0;
        let a65 = 256u64;
        let a66 = 0u64;
        nat_rem(a64, a65, a66)
    }, List::<u64>::nil()))))
}

pub fn f19(v0: List<Adt86>) -> List<u64> {
    f35(f36(v0, List::<Adt86>::nil()), List::<u64>::nil())
}

pub fn f20(v0: List<List<u64>>) -> List<u64> {
    f37(f38(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f21() -> u64 {
    500u64
}

pub fn f22() -> List<Adt86> {
    List::cons(Adt86::C0(f39(), f40()), List::<Adt86>::nil())
}

pub fn f23() -> List<u64> {
    let a69 = f41();
    let a70 = {
        let a67 = f42();
        let a68 = f43();
        append_list(a67, a68)
    };
    append_list(a69, a70)
}

pub fn f24(v0: Adt0, v1: Fn0, v2: Adt85, v3: Option<Adt90>) -> R<Adt88> {
    let m71 = v3;
    match m71 {
        None => {
            f45(v0, v1, v2)
        }
        Some(v4) => {
            f44(v4)
        }
    }
}

pub fn f25(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m72 = f46(v0.clone(), v1.clone())?;
    match m72 {
        None => {
            f47(v0, v1)
        }
        Some(v2) => {
            Ok(Some(v2))
        }
    }
}

pub fn f26(v0: List<Adt35>) -> R<Result<List<Adt12>, Adt91>> {
    let m73 = f48(v0.clone());
    match m73 {
        None => {
            f49(v0, List::<Adt12>::nil())
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f27(p76: u64, p77: List<Adt35>) -> R<List<Adt35>> {
    let m75 = (p76, p77);
    {
        let mut m75 = m75;
        loop {
            let step = {
                let (v0, v1) = m75;
                let m74 = v0;
                if m74 == 0 {
                    Err::<(u64, List<Adt35>), R<List<Adt35>>>(Ok(v1))
                } else {
                    let v2 = m74 - 1;
                    let v3: Adt35 = f50(f51(v2)?, Str::lit("Answers with its own name"), f52());
                    Ok::<(u64, List<Adt35>), R<List<Adt35>>>((v2, List::cons(v3, v1)))
                }
            };
            match step {
                Ok(next) => { m75 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f28() -> u64 {
    60u64
}

pub fn f29(v0: Adt91) -> Str {
    let m78 = v0;
    match m78 {
        Adt91::C0(v1) => {
            let a79 = List::cons(Str::lit("tool name is not 1 to 64 characters of letters, digits, underscore, hyphen and dot:"), List::cons(v1, List::<Str>::nil()));
            let a80 = Str::lit(" ");
            join(a79, a80)
        }
        Adt91::C1(v2) => {
            let a81 = List::cons(Str::lit("tool name is used twice:"), List::cons(v2, List::<Str>::nil()));
            let a82 = Str::lit(" ");
            join(a81, a82)
        }
        Adt91::C2(v3) => {
            let a83 = List::cons(Str::lit("the schema dialect of tool"), List::cons(v3, List::cons(Str::lit("is not JSON Schema 2020-12"), List::<Str>::nil())));
            let a84 = Str::lit(" ");
            join(a83, a84)
        }
        Adt91::C3(v4) => {
            let a85 = List::cons(Str::lit("the input schema of tool"), List::cons(v4, List::cons(Str::lit("does not have type object"), List::<Str>::nil())));
            let a86 = Str::lit(" ");
            join(a85, a86)
        }
        Adt91::C4(v5, v6) => {
            let a87 = List::cons(Str::lit("tool"), List::cons(v5, List::cons(Str::lit("uses the unsupported schema keyword"), List::cons(v6, List::<Str>::nil()))));
            let a88 = Str::lit(" ");
            join(a87, a88)
        }
        Adt91::C5(v7, v8) => {
            let a89 = List::cons(Str::lit("tool"), List::cons(v7, List::cons(Str::lit("has a malformed schema keyword"), List::cons(v8, List::<Str>::nil()))));
            let a90 = Str::lit(" ");
            join(a89, a90)
        }
        Adt91::C6(v9, v10) => {
            let a91 = List::cons(Str::lit("tool"), List::cons(v9, List::cons(Str::lit("has an unresolved reference"), List::cons(v10, List::<Str>::nil()))));
            let a92 = Str::lit(" ");
            join(a91, a92)
        }
        Adt91::C7(v11) => {
            let a93 = List::cons(Str::lit("tool"), List::cons(v11, List::cons(Str::lit("has an x-mcp-header that is not text"), List::<Str>::nil())));
            let a94 = Str::lit(" ");
            join(a93, a94)
        }
        Adt91::C8(v12) => {
            let a95 = List::cons(Str::lit("tool"), List::cons(v12, List::cons(Str::lit("has an empty x-mcp-header"), List::<Str>::nil())));
            let a96 = Str::lit(" ");
            join(a95, a96)
        }
        Adt91::C9(v13) => {
            let a97 = List::cons(Str::lit("tool"), List::cons(v13, List::cons(Str::lit("has an x-mcp-header that is not an HTTP field name"), List::<Str>::nil())));
            let a98 = Str::lit(" ");
            join(a97, a98)
        }
        Adt91::C10(v14) => {
            let a99 = List::cons(Str::lit("tool"), List::cons(v14, List::cons(Str::lit("has an x-mcp-header that is not reached through properties alone"), List::<Str>::nil())));
            let a100 = Str::lit(" ");
            join(a99, a100)
        }
        Adt91::C11(v15) => {
            let a101 = List::cons(Str::lit("tool"), List::cons(v15, List::cons(Str::lit("has an x-mcp-header on a parameter that is not a string, integer or boolean"), List::<Str>::nil())));
            let a102 = Str::lit(" ");
            join(a101, a102)
        }
        Adt91::C12(v16, v17) => {
            let a103 = List::cons(Str::lit("tool"), List::cons(v16, List::cons(Str::lit("repeats the x-mcp-header"), List::cons(v17, List::<Str>::nil()))));
            let a104 = Str::lit(" ");
            join(a103, a104)
        }
    }
}

pub fn f30(v0: Adt10, v1: List<Adt35>, v2: Adt20) -> Result<Adt32, Adt84> {
    let m106 = match v2 {
        Adt20::C0(_, h105) => {
            h105
        }
    };
    match m106 {
        None => {
            Ok::<Adt32, Adt84>(Adt32::C1(Adt34::C0(v0, None::<Str>, v1)))
        }
        Some(v3) => {
            Err::<Adt32, Adt84>(Adt84::C5(v3))
        }
    }
}

pub fn f31(v0: List<Adt35>, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m108 = f53(match v2.clone() {
        Adt21::C0(_, h107, _, _, _) => {
            h107
        }
    }, v0);
    match m108 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C0(match v2 {
                Adt21::C0(_, h109, _, _, _) => {
                    h109
                }
            })))
        }
        Some(v3) => {
            f54(v3, v1, v2)
        }
    }
}

pub fn f32(v0: Adt21) -> Result<Adt49, Adt84> {
    Ok::<Adt49, Adt84>(f55(List::cons(f56(match v0 {
        Adt21::C0(_, h110, _, _, _) => {
            h110
        }
    }), List::<Adt38>::nil())))
}

pub fn f33(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a114 = {
        let a111 = v1;
        let a112 = 4u64;
        nat_add(a111, a112)?
    };
    let a115 = {
        let a113 = v0.clone();
        length_bytes(a113)
    };
    if nat_le(a114, a115) {
        Ok(Some({
            let a132 = {
                let a116 = f57(v0.clone(), v1)?;
                let a117 = 16777216u64;
                nat_mul(a116, a117)?
            };
            let a133 = {
                let a130 = {
                    let a120 = f57(v0.clone(), {
                        let a118 = v1;
                        let a119 = 1u64;
                        nat_add(a118, a119)?
                    })?;
                    let a121 = 65536u64;
                    nat_mul(a120, a121)?
                };
                let a131 = {
                    let a128 = {
                        let a124 = f57(v0.clone(), {
                            let a122 = v1;
                            let a123 = 2u64;
                            nat_add(a122, a123)?
                        })?;
                        let a125 = 256u64;
                        nat_mul(a124, a125)?
                    };
                    let a129 = f57(v0, {
                        let a126 = v1;
                        let a127 = 3u64;
                        nat_add(a126, a127)?
                    })?;
                    nat_add(a128, a129)?
                };
                nat_add(a130, a131)?
            };
            nat_add(a132, a133)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f34(v0: Bytes, v1: Adt89, v2: Adt89, v3: Option<u64>) -> R<Result<Adt85, Adt87>> {
    let m134 = v3;
    match m134 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0(match v2 {
                Adt89::C0(_, h135) => {
                    h135
                }
            })))
        }
        Some(v4) => {
            let m139 = f58(v0.clone(), {
                let a137 = match v2.clone() {
                    Adt89::C0(_, h136) => {
                        h136
                    }
                };
                let a138 = 4u64;
                nat_add(a137, a138)?
            }, v4, List::<Adt86>::nil())?;
            match m139 {
                Ok(v6) => {
                    f59(v0, v1, v2, v6)
                }
                Err(v5) => {
                    Ok(Err::<Adt85, Adt87>(v5))
                }
            }
        }
    }
}

pub fn f35(p144: List<Adt86>, p145: List<u64>) -> List<u64> {
    let m143 = (p144, p145);
    {
        let mut m143 = m143;
        loop {
            let step = {
                let (v0, v1) = m143;
                let m140 = v0;
                match m140.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<u64>), List<u64>>((v3, {
                            let a141 = f60(v2);
                            let a142 = v1;
                            append_list(a141, a142)
                        }))
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

pub fn f36(p148: List<Adt86>, p149: List<Adt86>) -> List<Adt86> {
    let m147 = (p148, p149);
    {
        let mut m147 = m147;
        loop {
            let step = {
                let (v0, v1) = m147;
                let m146 = v0;
                match m146.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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

pub fn f37(p154: List<List<u64>>, p155: List<u64>) -> List<u64> {
    let m153 = (p154, p155);
    {
        let mut m153 = m153;
        loop {
            let step = {
                let (v0, v1) = m153;
                let m150 = v0;
                match m150.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a151 = f61(v2);
                            let a152 = v1;
                            append_list(a151, a152)
                        }))
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

pub fn f38(p158: List<List<u64>>, p159: List<List<u64>>) -> List<List<u64>> {
    let m157 = (p158, p159);
    {
        let mut m157 = m157;
        loop {
            let step = {
                let (v0, v1) = m157;
                let m156 = v0;
                match m156.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<List<u64>>), List<List<u64>>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m157 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f39() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f40() -> List<u64> {
    List::cons(97u64, List::cons(112u64, List::cons(112u64, List::cons(108u64, List::cons(105u64, List::cons(99u64, List::cons(97u64, List::cons(116u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::cons(47u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))
}

pub fn f41() -> List<u64> {
    List::cons(123u64, List::cons(34u64, List::cons(106u64, List::cons(115u64, List::cons(111u64, List::cons(110u64, List::cons(114u64, List::cons(112u64, List::cons(99u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(50u64, List::cons(46u64, List::cons(48u64, List::cons(34u64, List::<u64>::nil()))))))))))))))))
}

pub fn f42() -> List<u64> {
    List::cons(44u64, List::cons(34u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(58u64, List::cons(123u64, List::cons(34u64, List::cons(99u64, List::cons(111u64, List::cons(100u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(45u64, List::cons(51u64, List::cons(50u64, List::cons(54u64, List::cons(48u64, List::cons(51u64, List::cons(44u64, List::cons(34u64, List::cons(109u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f43() -> List<u64> {
    List::cons(101u64, List::cons(115u64, List::cons(115u64, List::cons(97u64, List::cons(103u64, List::cons(101u64, List::cons(34u64, List::cons(58u64, List::cons(34u64, List::cons(73u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(114u64, List::cons(110u64, List::cons(97u64, List::cons(108u64, List::cons(32u64, List::cons(101u64, List::cons(114u64, List::cons(114u64, List::cons(111u64, List::cons(114u64, List::cons(34u64, List::cons(125u64, List::cons(125u64, List::<u64>::nil()))))))))))))))))))))))))))
}

pub fn f44(v0: Adt90) -> R<Adt88> {
    f62(f63(v0.clone()), f64(None::<Adt92>, f65(v0)))
}

pub fn f45(v0: Adt0, v1: Fn0, v2: Adt85) -> R<Adt88> {
    if f66(match v2.clone() {
        Adt85::C0(h160, _, _, _, _, _, _) => {
            h160
        }
    }) {
        f67(f68(v0, match v2.clone() {
            Adt85::C0(_, _, _, _, _, _, h161) => {
                h161
            }
        }), v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h162, _, _) => {
                h162
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h163, _, _, _, _) => {
                h163
            }
        }, f69(match v2 {
            Adt85::C0(_, _, _, h164, _, _, _) => {
                h164
            }
        })?)
    } else {
        Ok(f70())
    }
}

pub fn f46(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m165 = f71(v0, f72())?;
    match m165 {
        Adt96::C0 => {
            Ok(None::<Adt90>)
        }
        Adt96::C1 => {
            Ok(Some(Adt90::C0))
        }
        Adt96::C2 => {
            Ok(Some(Adt90::C0))
        }
        Adt96::C3(v2) => {
            f73(f74(v2), v1, Adt90::C0)
        }
    }
}

pub fn f47(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m166 = f71(v0, f75())?;
    match m166 {
        Adt96::C0 => {
            Ok(None::<Adt90>)
        }
        Adt96::C1 => {
            Ok(Some(Adt90::C1))
        }
        Adt96::C2 => {
            Ok(Some(Adt90::C1))
        }
        Adt96::C3(v2) => {
            f73(f76(v2), v1, Adt90::C1)
        }
    }
}

pub fn f48(v0: List<Adt35>) -> Option<Adt91> {
    let m167 = f77(v0, List::<Str>::nil());
    match m167 {
        None => {
            None::<Adt91>
        }
        Some(v1) => {
            Some(Adt91::C1(v1))
        }
    }
}

pub fn f49(p171: List<Adt35>, p172: List<Adt12>) -> R<Result<List<Adt12>, Adt91>> {
    let m170 = (p171, p172);
    {
        let mut m170 = m170;
        loop {
            let step = {
                let (v0, v1) = m170;
                let m168 = v0;
                match m168.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Ok::<List<Adt12>, Adt91>(f78(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m169 = f79(v2)?;
                        match m169 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>((v3, f78(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Err::<List<Adt12>, Adt91>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m170 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f51(v0: u64) -> R<Str> {
    let a174 = List::cons(Str::lit("crowd_"), List::cons({
        let a173 = f80(v0, 0i64)?;
        format_i64(a173)
    }, List::<Str>::nil()));
    let a175 = Str::lit("");
    Ok(join(a174, a175))
}

pub fn f52() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f53(v0: Str, p181: List<Adt35>) -> Option<Adt35> {
    let m180 = p181;
    {
        let mut m180 = m180;
        loop {
            let step = {
                let v1 = m180;
                let m176 = v1;
                match m176.uncons() {
                    None => {
                        Err::<List<Adt35>, Option<Adt35>>(None::<Adt35>)
                    }
                    Some((v2, v3)) => {
                        let a178 = match v2.clone() {
                            Adt35::C0(h177, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        let a179 = v0.clone();
                        if equal(a178, a179) {
                            Err::<List<Adt35>, Option<Adt35>>(Some(v2))
                        } else {
                            Ok::<List<Adt35>, Option<Adt35>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m180 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f54(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f81(v0.clone(), v2, Fn2::F82(v1, v0))
}

pub fn f55(v0: List<Adt38>) -> Adt49 {
    f83(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f56(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f57(v0: Bytes, v1: u64) -> R<u64> {
    let m184 = {
        let a182 = v0;
        let a183 = v1;
        index_bytes(a182, a183)
    };
    match m184 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a229 = {
                let a205 = {
                    let a193 = {
                        let a187 = {
                            let a185 = v3;
                            let a186 = 128u8;
                            fixed_u8::bit_and(a185, a186)
                        };
                        let a188 = 128u8;
                        if equal(a187, a188) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a194 = {
                        let a191 = {
                            let a189 = v3;
                            let a190 = 64u8;
                            fixed_u8::bit_and(a189, a190)
                        };
                        let a192 = 64u8;
                        if equal(a191, a192) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a193, a194)?
                };
                let a206 = {
                    let a203 = {
                        let a197 = {
                            let a195 = v3;
                            let a196 = 32u8;
                            fixed_u8::bit_and(a195, a196)
                        };
                        let a198 = 32u8;
                        if equal(a197, a198) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a204 = {
                        let a201 = {
                            let a199 = v3;
                            let a200 = 16u8;
                            fixed_u8::bit_and(a199, a200)
                        };
                        let a202 = 16u8;
                        if equal(a201, a202) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a203, a204)?
                };
                nat_add(a205, a206)?
            };
            let a230 = {
                let a227 = {
                    let a215 = {
                        let a209 = {
                            let a207 = v3;
                            let a208 = 8u8;
                            fixed_u8::bit_and(a207, a208)
                        };
                        let a210 = 8u8;
                        if equal(a209, a210) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a216 = {
                        let a213 = {
                            let a211 = v3;
                            let a212 = 4u8;
                            fixed_u8::bit_and(a211, a212)
                        };
                        let a214 = 4u8;
                        if equal(a213, a214) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a215, a216)?
                };
                let a228 = {
                    let a225 = {
                        let a219 = {
                            let a217 = v3;
                            let a218 = 2u8;
                            fixed_u8::bit_and(a217, a218)
                        };
                        let a220 = 2u8;
                        if equal(a219, a220) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a226 = {
                        let a223 = {
                            let a221 = v3;
                            let a222 = 1u8;
                            fixed_u8::bit_and(a221, a222)
                        };
                        let a224 = 1u8;
                        if equal(a223, a224) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a225, a226)?
                };
                nat_add(a227, a228)?
            };
            nat_add(a229, a230)
        }
    }
}

pub fn f58(v0: Bytes, p240: u64, p241: u64, p242: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m239 = (p240, (p241, p242));
    {
        let mut m239 = m239;
        loop {
            let step = {
                let (v1, (v2, v3)) = m239;
                let m231 = v2;
                if m231 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f84(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m231 - 1;
                    let m232 = f16(v0.clone(), v1)?;
                    match m232 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m234 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h233) => {
                                    h233
                                }
                            })?;
                            match m234 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h235) => {
                                            h235
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h236) => {
                                            h236
                                        }
                                    }, (v4, List::cons(Adt86::C0(f85(match v5 {
                                        Adt89::C0(h237, _) => {
                                            h237
                                        }
                                    })?, f85(match v6 {
                                        Adt89::C0(h238, _) => {
                                            h238
                                        }
                                    })?), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f59(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m244 = f16(v0.clone(), {
        let (_, h243) = v3.clone();
        h243
    })?;
    match m244 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h245) = v3;
                h245
            })))
        }
        Some(v4) => {
            let m247 = f33(v0.clone(), match v4.clone() {
                Adt89::C0(_, h246) => {
                    h246
                }
            })?;
            match m247 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h248) => {
                            h248
                        }
                    })))
                }
                Some(v5) => {
                    let m252 = f16(v0.clone(), {
                        let a250 = match v4.clone() {
                            Adt89::C0(_, h249) => {
                                h249
                            }
                        };
                        let a251 = 4u64;
                        nat_add(a250, a251)?
                    })?;
                    match m252 {
                        None => {
                            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                                let a254 = match v4 {
                                    Adt89::C0(_, h253) => {
                                        h253
                                    }
                                };
                                let a255 = 4u64;
                                nat_add(a254, a255)?
                            })))
                        }
                        Some(v6) => {
                            f86(v0, Adt85::C0(f85(match v1 {
                                Adt89::C0(h256, _) => {
                                    h256
                                }
                            })?, f85(match v2 {
                                Adt89::C0(h257, _) => {
                                    h257
                                }
                            })?, {
                                let (h258, _) = v3;
                                h258
                            }, match v4 {
                                Adt89::C0(h259, _) => {
                                    h259
                                }
                            }, v5, List::<List<u64>>::nil(), match v6.clone() {
                                Adt89::C0(h260, _) => {
                                    h260
                                }
                            }), match v6 {
                                Adt89::C0(_, h261) => {
                                    h261
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}

pub fn f60(v0: Adt86) -> List<u64> {
    let a272 = f18({
        let a263 = match v0.clone() {
            Adt86::C0(h262, _) => {
                h262
            }
        };
        length_list(a263)
    });
    let a273 = {
        let a270 = match v0.clone() {
            Adt86::C0(h264, _) => {
                h264
            }
        };
        let a271 = {
            let a268 = f18({
                let a266 = match v0.clone() {
                    Adt86::C0(_, h265) => {
                        h265
                    }
                };
                length_list(a266)
            });
            let a269 = match v0 {
                Adt86::C0(_, h267) => {
                    h267
                }
            };
            append_list(a268, a269)
        };
        append_list(a270, a271)
    };
    append_list(a272, a273)
}

pub fn f61(v0: List<u64>) -> List<u64> {
    let a275 = f18({
        let a274 = v0.clone();
        length_list(a274)
    });
    let a276 = v0;
    append_list(a275, a276)
}

pub fn f62(v0: u64, v1: Adt6) -> R<Adt88> {
    let m277 = f87(v1)?;
    match m277 {
        Ok(v3) => {
            Ok(Adt88::C0(v0, f22(), List::<List<u64>>::nil(), v3))
        }
        Err(_) => {
            Ok(f6())
        }
    }
}

pub fn f63(v0: Adt90) -> u64 {
    let m278 = v0;
    match m278 {
        Adt90::C0 => {
            f88()
        }
        Adt90::C1 => {
            f88()
        }
    }
}

pub fn f64(v0: Option<Adt92>, v1: Adt80) -> Adt6 {
    Adt6::C5(f89(v0, v1))
}

pub fn f65(v0: Adt90) -> Adt80 {
    Adt80::C0(f90(), f91(v0), None::<Adt6>)
}

pub fn f66(v0: List<u64>) -> bool {
    f92(v0, f93())
}

pub fn f67(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt93>) -> R<Adt88> {
    let m279 = v4;
    match m279 {
        Ok(v6) => {
            f96(v0, v1, v2, v3, f97(v6)?)
        }
        Err(_) => {
            f62(f94(), f64(None::<Adt92>, f95(Adt94::C0)))
        }
    }
}

pub fn f68(v0: Adt0, v1: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h280, _, _, _, _, _, _) => {
            h280
        }
    }, match v0.clone() {
        Adt0::C0(_, h281, _, _, _, _, _) => {
            h281
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h282, _, _, _, _) => {
            h282
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h283, _, _, _) => {
            h283
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h284, _, _) => {
            h284
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, h285, _) => {
            h285
        }
    }, v1)
}

pub fn f69(v0: Bytes) -> R<Result<Adt6, Adt93>> {
    f98(f85(v0)?)
}

pub fn f70() -> Adt88 {
    Adt88::C0(f99(), List::cons(Adt86::C0(f100(), f101()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f71(v0: List<Adt86>, v1: List<u64>) -> R<Adt96> {
    let m286 = f102(v0, v1, Adt97::C0)?;
    match m286 {
        Adt97::C0 => {
            Ok(Adt96::C0)
        }
        Adt97::C1(v2) => {
            let m287 = f103(v2);
            match m287 {
                None => {
                    Ok(Adt96::C1)
                }
                Some(v3) => {
                    Ok(Adt96::C3(v3))
                }
            }
        }
        Adt97::C2 => {
            Ok(Adt96::C2)
        }
    }
}

pub fn f72() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f73(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt90) -> R<Option<Adt90>> {
    let m288 = v0;
    match m288 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f104(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f74(v0: List<u64>) -> Option<List<u64>> {
    let m289 = f105(v0);
    match m289 {
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
    let m290 = v0.clone();
    match m290.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a291 = v1;
            let a292 = 91u64;
            if nat_eq(a291, a292) {
                f106(f107(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f106(f108(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f77(p298: List<Adt35>, p299: List<Str>) -> Option<Str> {
    let m297 = (p298, p299);
    {
        let mut m297 = m297;
        loop {
            let step = {
                let (v0, v1) = m297;
                let m293 = v0;
                match m293.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f109(match v2.clone() {
                            Adt35::C0(h294, _, _, _, _, _, _, _) => {
                                h294
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h295, _, _, _, _, _, _, _) => {
                                    h295
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h296, _, _, _, _, _, _, _) => {
                                    h296
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(p302: List<Adt12>, p303: List<Adt12>) -> List<Adt12> {
    let m301 = (p302, p303);
    {
        let mut m301 = m301;
        loop {
            let step = {
                let (v0, v1) = m301;
                let m300 = v0;
                match m300.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m301 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f79(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m306 = f110(match v0.clone() {
        Adt35::C0(h304, _, _, _, _, _, _, _) => {
            h304
        }
    }, f111(match v0.clone() {
        Adt35::C0(h305, _, _, _, _, _, _, _) => {
            h305
        }
    })?);
    match m306 {
        None => {
            f112(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f80(p309: u64, p310: i64) -> R<i64> {
    let m308 = (p309, p310);
    {
        let mut m308 = m308;
        loop {
            let step = {
                let (v0, v1) = m308;
                let m307 = v0;
                if m307 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m307 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f113(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: Adt35, v1: Adt21, v2: Fn2) -> R<Result<Adt32, Adt84>> {
    let m317 = f114(Adt6::C5({
        let m313 = match v0.clone() {
            Adt35::C0(_, _, _, _, h312, _, _, _) => {
                h312
            }
        };
        match m313 {
            Adt5::C0(h311) => {
                h311
            }
        }
    }), Adt6::C5({
        let m316 = match v0 {
            Adt35::C0(_, _, _, _, h315, _, _, _) => {
                h315
            }
        };
        match m316 {
            Adt5::C0(h314) => {
                h314
            }
        }
    }), f115(v1.clone()))?;
    match m317 {
        Adt100::C0 => {
            let c318: Fn2 = v2;
            c318.apply(v1)
        }
        Adt100::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f116(f117(v3, v4)?, true))))
        }
        Adt100::C2(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt100::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt100::C4 => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f82(v0: Fn1, v1: Adt35, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    f118(v1, v0, v2)
}

pub fn f83(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f84(p321: List<Adt86>, p322: List<Adt86>) -> List<Adt86> {
    let m320 = (p321, p322);
    {
        let mut m320 = m320;
        loop {
            let step = {
                let (v0, v1) = m320;
                let m319 = v0;
                match m319.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
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
    f119(v0.clone(), {
        let a323 = v0;
        length_bytes(a323)
    }, List::<u64>::nil())
}

pub fn f86(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a325 = v2;
    let a326 = {
        let a324 = v0.clone();
        length_bytes(a324)
    };
    if nat_eq(a325, a326) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m327 = f33(v0.clone(), v2)?;
        match m327 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m330 = f120(v0.clone(), {
                    let a328 = v2;
                    let a329 = 4u64;
                    nat_add(a328, a329)?
                }, v3, List::<List<u64>>::nil())?;
                match m330 {
                    Ok(v5) => {
                        let a333 = {
                            let (_, h331) = v5.clone();
                            h331
                        };
                        let a334 = {
                            let a332 = v0;
                            length_bytes(a332)
                        };
                        if nat_eq(a333, a334) {
                            Ok(Ok::<Adt85, Adt87>(f121(v1, {
                                let (h335, _) = v5;
                                h335
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h336) = v5;
                                h336
                            })))
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<Adt85, Adt87>(v4))
                    }
                }
            }
        }
    }
}

pub fn f87(v0: Adt6) -> R<Result<List<u64>, Adt93>> {
    f122(f123(), v0)
}

pub fn f88() -> u64 {
    403u64
}

pub fn f89(v0: Option<Adt92>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f124())), {
        let a337 = f125(v0);
        let a338 = List::cons((Str::lit("error"), f126(v1)), List::<(Str, Adt6)>::nil());
        append_list(a337, a338)
    })
}

pub fn f90() -> i64 {
    -32000i64
}

pub fn f91(v0: Adt90) -> Str {
    let m339 = v0;
    match m339 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f92(p346: List<u64>, p347: List<u64>) -> bool {
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

pub fn f93() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f94() -> u64 {
    400u64
}

pub fn f95(v0: Adt94) -> Adt80 {
    Adt80::C0(f127(v0.clone()), f128(v0), None::<Adt6>)
}

pub fn f96(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt98, Adt94>) -> R<Adt88> {
    let m348 = v4;
    match m348 {
        Ok(v6) => {
            f131(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f62(f129(v5.clone()), f64(f130(v5.clone()), f95(v5)))
        }
    }
}

pub fn f97(v0: Adt6) -> R<Result<Adt98, Adt94>> {
    let m349 = v0;
    match m349 {
        Adt6::C0 => {
            Ok(f133(None::<Adt92>, Adt95::C0))
        }
        Adt6::C1(_) => {
            Ok(f133(None::<Adt92>, Adt95::C0))
        }
        Adt6::C2(_) => {
            Ok(f133(None::<Adt92>, Adt95::C0))
        }
        Adt6::C3(_) => {
            Ok(f133(None::<Adt92>, Adt95::C0))
        }
        Adt6::C4(_) => {
            Ok(f133(None::<Adt92>, Adt95::C1))
        }
        Adt6::C5(v1) => {
            f132(Adt5::C0(v1))
        }
    }
}

pub fn f98(v0: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m350 = f134(v0.clone())?;
    match m350 {
        Adt111::C0 => {
            f135(f136(), 0u64, v0)
        }
        Adt111::C1(v1) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C0(v1)))
        }
        Adt111::C2(v2) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C1(v2)))
        }
    }
}

pub fn f99() -> u64 {
    405u64
}

pub fn f100() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f101() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f102(p355: List<Adt86>, v1: List<u64>, p356: Adt97) -> R<Adt97> {
    let m354 = (p355, p356);
    {
        let mut m354 = m354;
        loop {
            let step = {
                let (v0, v2) = m354;
                let m351 = v0;
                match m351.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt97), R<Adt97>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f137(match v3.clone() {
                            Adt86::C0(h352, _) => {
                                h352
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, f138(v2, match v3 {
                                Adt86::C0(_, h353) => {
                                    h353
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, v2))
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

pub fn f103(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f139(v0);
    if f140(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f104(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f137(v0.clone(), f141())? {
        Ok(true)
    } else {
        if f137(v0.clone(), f142())? {
            Ok(true)
        } else {
            if f137(v0.clone(), f143())? {
                Ok(true)
            } else {
                f144(v0, v1)
            }
        }
    }
}

pub fn f105(p361: List<u64>) -> Option<List<u64>> {
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
                            Err::<List<u64>, Option<List<u64>>>(f145(v2))
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

pub fn f106(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m362 = v0;
    match m362 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f146({
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

pub fn f107(p369: List<u64>, p370: List<u64>) -> Option<(List<u64>, List<u64>)> {
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
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f147(List::cons(v2, v1), List::<u64>::nil()), v3)))
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

pub fn f108(p375: List<u64>, p376: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m374 = (p375, p376);
    {
        let mut m374 = m374;
        loop {
            let step = {
                let (v0, v1) = m374;
                let m371 = v0;
                match m371.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f147(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a372 = v2;
                        let a373 = 58u64;
                        if nat_eq(a372, a373) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f147(v1, List::<u64>::nil()), List::cons(v2, v3))))
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

pub fn f109(v0: Str, p381: List<Str>) -> bool {
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

pub fn f110(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f148(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f111(v0: Str) -> R<List<u64>> {
    f85({
        let a382 = v0;
        utf8_encode(a382)
    })
}

pub fn f112(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m383 = f149(v0.clone());
    match m383 {
        None => {
            f150(v0.clone(), f151(f152(), match v0.clone() {
                Adt35::C0(h384, _, _, _, _, _, _, _) => {
                    h384
                }
            }, Adt6::C5({
                let m387 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h386, _, _, _) => {
                        h386
                    }
                };
                match m387 {
                    Adt5::C0(h385) => {
                        h385
                    }
                }
            }), List::cons(Adt99::C0(Adt6::C5({
                let m390 = match v0 {
                    Adt35::C0(_, _, _, _, h389, _, _, _) => {
                        h389
                    }
                };
                match m390 {
                    Adt5::C0(h388) => {
                        h388
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt99>::nil()), f153())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f113(v0: i64, v1: i64) -> R<i64> {
    let a392 = v0;
    let a393 = {
        let a391 = v1;
        int_neg(a391)?
    };
    int_sub(a392, a393)
}

pub fn f114(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt100> {
    f154(v0, f155(), Adt102::C0, List::cons(Adt103::C0(v1, v2, List::<Adt101>::nil()), List::<Adt103>::nil()))
}

pub fn f115(v0: Adt21) -> Adt6 {
    let m395 = match v0 {
        Adt21::C0(_, _, h394, _, _) => {
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

pub fn f116(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f117(v0: List<Adt101>, v1: Str) -> R<Str> {
    let v2: Str = f156(v0)?;
    let a399 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a397 = v2.clone();
        let a398 = Str::lit("");
        if equal(a397, a398) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a400 = Str::lit(" ");
    Ok(join(a399, a400))
}

pub fn f118(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m402 = {
        let c401: Fn1 = v1;
        c401.apply(v2)
    };
    match m402 {
        Ok(v4) => {
            f157(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f119(v0: Bytes, p405: u64, p406: List<u64>) -> R<List<u64>> {
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
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f57(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f120(v0: Bytes, p412: u64, p413: u64, p414: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m411 = (p412, (p413, p414));
    {
        let mut m411 = m411;
        loop {
            let step = {
                let (v1, (v2, v3)) = m411;
                let m407 = v2;
                if m407 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f38(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m407 - 1;
                    let m408 = f16(v0.clone(), v1)?;
                    match m408 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h409) => {
                                    h409
                                }
                            }, (v4, List::cons(f85(match v5 {
                                Adt89::C0(h410, _) => {
                                    h410
                                }
                            })?, v3))))
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

pub fn f121(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h415, _, _, _, _, _, _) => {
            h415
        }
    }, match v0.clone() {
        Adt85::C0(_, h416, _, _, _, _, _) => {
            h416
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h417, _, _, _, _) => {
            h417
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h418, _, _, _) => {
            h418
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, _, h419, _, _) => {
            h419
        }
    }, v1, match v0 {
        Adt85::C0(_, _, _, _, _, _, h420) => {
            h420
        }
    })
}

pub fn f122(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt93>> {
    let m421 = f158(v0, List::cons(Adt105::C1(0u64, v1), List::<Adt105>::nil()), List::<u64>::nil())?;
    match m421 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt93>(f159(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt93>(v3))
        }
    }
}

pub fn f123() -> u64 {
    67108864u64
}

pub fn f124() -> Str {
    Str::lit("2.0")
}

pub fn f125(v0: Option<Adt92>) -> List<(Str, Adt6)> {
    let m422 = v0;
    match m422 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f160(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f126(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h423, _, _) => {
            h423
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h424, _) => {
            h424
        }
    })), f161(match v0 {
        Adt80::C0(_, _, h425) => {
            h425
        }
    }))))
}

pub fn f127(v0: Adt94) -> i64 {
    let m426 = v0;
    match m426 {
        Adt94::C0 => {
            f162()
        }
        Adt94::C1(_, _) => {
            f163()
        }
    }
}

pub fn f128(v0: Adt94) -> Str {
    let m427 = v0;
    match m427 {
        Adt94::C0 => {
            Str::lit("Parse error")
        }
        Adt94::C1(_, v2) => {
            f164(v2)
        }
    }
}

pub fn f129(v0: Adt94) -> u64 {
    let m428 = v0;
    match m428 {
        Adt94::C0 => {
            f94()
        }
        Adt94::C1(_, _) => {
            f94()
        }
    }
}

pub fn f130(v0: Adt94) -> Option<Adt92> {
    let m429 = v0;
    match m429 {
        Adt94::C0 => {
            None::<Adt92>
        }
        Adt94::C1(v1, _) => {
            v1
        }
    }
}

pub fn f131(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt98) -> R<Adt88> {
    f165(f166(v3.clone())?, f167(v0.clone(), v1, v2, Some(f168(v3, match v0 {
        Adt0::C0(_, _, _, _, h430, _, _) => {
            h430
        }
    }, f169(v4.clone()), match v4.clone() {
        Adt98::C0(_, h431, _) => {
            h431
        }
    })?), f170(), v4)?)
}

pub fn f132(v0: Adt5) -> R<Result<Adt98, Adt94>> {
    let m432 = f171(v0.clone())?;
    match m432 {
        Adt128::C0 => {
            let m433 = f172(Str::lit("method"), v0.clone());
            match m433 {
                Adt129::C0 => {
                    Ok(f133(None::<Adt92>, f173(v0)))
                }
                Adt129::C1(_) => {
                    Ok(f133(None::<Adt92>, if f174(v0) {
                        Adt95::C8
                    } else {
                        Adt95::C7
                    }))
                }
            }
        }
        Adt128::C1(v2) => {
            Ok(f175(v2, v0))
        }
        Adt128::C2 => {
            Ok(f133(None::<Adt92>, Adt95::C5))
        }
    }
}

pub fn f133(v0: Option<Adt92>, v1: Adt95) -> Result<Adt98, Adt94> {
    Err::<Adt98, Adt94>(Adt94::C1(v0, v1))
}

pub fn f134(v0: List<u64>) -> R<Adt111> {
    Ok(f176({
        let a434 = v0.clone();
        length_list(a434)
    }, f177(Adt112::C0, 0u64, 0u64, v0)?))
}

pub fn f135(p441: Adt113, p442: u64, p443: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m440 = (p441, (p442, p443));
    {
        let mut m440 = m440;
        loop {
            let step = {
                let (v0, (v1, v2)) = m440;
                let m435 = v2;
                match m435.uncons() {
                    None => {
                        let m436 = f178(v0, v1)?;
                        match m436 {
                            Ok(v3) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Ok::<Adt6, Adt93>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m437 = f179(v0, v1, v5)?;
                        match m437 {
                            Adt130::C0(v7) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v7)))
                            }
                            Adt130::C1(v8) => {
                                Ok::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>((v8, ({
                                    let a438 = v1;
                                    let a439 = 1u64;
                                    nat_add(a438, a439)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m440 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f136() -> Adt113 {
    Adt113::C0(Adt114::C0, List::<Adt118>::nil(), 0u64)
}

pub fn f137(p450: List<u64>, p451: List<u64>) -> R<bool> {
    let m449 = (p450, p451);
    {
        let mut m449 = m449;
        loop {
            let step = {
                let (v0, v1) = m449;
                let m444 = v0;
                match m444.uncons() {
                    None => {
                        let m445 = v1;
                        match m445.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m446 = v1;
                        match m446.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a447 = f180(v4)?;
                                let a448 = f180(v6)?;
                                if nat_eq(a447, a448) {
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
                Ok(next) => { m449 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Adt97, v1: List<u64>) -> Adt97 {
    let m452 = v0;
    match m452 {
        Adt97::C0 => {
            Adt97::C1(v1)
        }
        Adt97::C1(_) => {
            Adt97::C2
        }
        Adt97::C2 => {
            Adt97::C2
        }
    }
}

pub fn f139(v0: List<u64>) -> List<u64> {
    f147(f181(f147(f181(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f140(p455: List<u64>) -> bool {
    let m454 = p455;
    {
        let mut m454 = m454;
        loop {
            let step = {
                let v0 = m454;
                let m453 = v0;
                match m453.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f182(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f141() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f142() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f143() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f144(v0: List<u64>, p458: List<List<u64>>) -> R<bool> {
    let m457 = p458;
    {
        let mut m457 = m457;
        loop {
            let step = {
                let v1 = m457;
                let m456 = v1;
                match m456.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f137(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
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

pub fn f145(v0: List<u64>) -> Option<List<u64>> {
    let m459 = v0;
    match m459.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m460 = v2;
            match m460.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a461 = v1;
                    let a462 = 47u64;
                    if if nat_eq(a461, a462) {
                        let a463 = v3;
                        let a464 = 47u64;
                        nat_eq(a463, a464)
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

pub fn f146(v0: List<u64>) -> bool {
    let m465 = v0;
    match m465.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a466 = v1;
            let a467 = 58u64;
            if nat_eq(a466, a467) {
                let m468 = v2.clone();
                match m468.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f183(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f147(p471: List<u64>, p472: List<u64>) -> List<u64> {
    let m470 = (p471, p472);
    {
        let mut m470 = m470;
        loop {
            let step = {
                let (v0, v1) = m470;
                let m469 = v0;
                match m469.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m470 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: List<u64>) -> bool {
    let m473 = v0.clone();
    match m473.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a477 = if f184(v0.clone(), false) {
                let a475 = {
                    let a474 = v0;
                    length_list(a474)
                };
                let a476 = f185();
                nat_le(a475, a476)
            } else {
                false
            };
            bool_not(a477)
        }
    }
}

pub fn f149(v0: Adt35) -> Option<Adt91> {
    let m482 = f186(match v0.clone() {
        Adt35::C0(h478, _, _, _, _, _, _, _) => {
            h478
        }
    }, {
        let m481 = match v0.clone() {
            Adt35::C0(_, _, _, _, h480, _, _, _) => {
                h480
            }
        };
        match m481 {
            Adt5::C0(h479) => {
                h479
            }
        }
    });
    match m482 {
        None => {
            f187(match v0.clone() {
                Adt35::C0(h483, _, _, _, _, _, _, _) => {
                    h483
                }
            }, {
                let m486 = match v0 {
                    Adt35::C0(_, _, _, _, h485, _, _, _) => {
                        h485
                    }
                };
                match m486 {
                    Adt5::C0(h484) => {
                        h484
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f150(v0: Adt35, v1: Result<Adt119, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m487 = v1;
    match m487 {
        Ok(v3) => {
            let m488 = f188(v0)?;
            match m488 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f78(match v3 {
                        Adt119::C0(h489, _) => {
                            h489
                        }
                    }, List::<Adt12>::nil())))
                }
                Some(v4) => {
                    Ok(Err::<List<Adt12>, Adt91>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<List<Adt12>, Adt91>(v2))
        }
    }
}

pub fn f151(p496: u64, v1: Str, v2: Adt6, p497: List<Adt99>, p498: Adt119) -> R<Result<Adt119, Adt91>> {
    let m495 = (p496, (p497, p498));
    {
        let mut m495 = m495;
        loop {
            let step = {
                let (v0, (v3, v4)) = m495;
                let m490 = v0;
                if m490 == 0 {
                    Err::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>(Ok(Err::<Adt119, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m490 - 1;
                    let m491 = v3;
                    match m491.uncons() {
                        None => {
                            Err::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>(Ok(Ok::<Adt119, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m492 = f189(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m492 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>((v5, ({
                                        let (h493, _) = v9;
                                        h493
                                    }, {
                                        let (_, h494) = v9;
                                        h494
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>(Ok(Err::<Adt119, Adt91>(v8)))
                                }
                            }
                        }
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

pub fn f152() -> u64 {
    67108864u64
}

pub fn f153() -> Adt119 {
    Adt119::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f154(v0: Adt6, p517: u64, p518: Adt102, p519: List<Adt103>) -> R<Adt100> {
    let m516 = (p517, (p518, p519));
    {
        let mut m516 = m516;
        loop {
            let step = {
                let (v1, (v2, v3)) = m516;
                let m499 = v1;
                if m499 == 0 {
                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C4))
                } else {
                    let v4 = m499 - 1;
                    let m500 = v3;
                    match m500.uncons() {
                        None => {
                            let m501 = v2;
                            match m501 {
                                Adt102::C0 => {
                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C0))
                                }
                                Adt102::C1(v5, v6) => {
                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m502 = v2.clone();
                            match m502 {
                                Adt102::C0 => {
                                    let m503 = v7.clone();
                                    match m503 {
                                        Adt103::C0(v9, v10, v11) => {
                                            let m504 = f190(v0.clone(), v9, v10, v11.clone())?;
                                            match m504 {
                                                Adt131::C0(v12) => {
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (Adt102::C1(v11, v12), v8)))
                                                }
                                                Adt131::C1(v13) => {
                                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C2(v13)))
                                                }
                                                Adt131::C2(v14) => {
                                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C3(v14)))
                                                }
                                                Adt131::C3(v15) => {
                                                    let v16: List<Adt103> = {
                                                        let a505 = v15;
                                                        let a506 = v8;
                                                        append_list(a505, a506)
                                                    };
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt103::C1(_, _, _) => {
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v8)))
                                        }
                                        Adt103::C2(_, _, _, _) => {
                                            let v24: Adt120 = f191(v7, v8)?;
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v24.clone() {
                                                Adt120::C0(h507, _) => {
                                                    h507
                                                }
                                            }, match v24 {
                                                Adt120::C0(_, h508) => {
                                                    h508
                                                }
                                            })))
                                        }
                                        Adt103::C3(v25) => {
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (Adt102::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt103::C4(_, _, _, _) => {
                                            let v39: Adt120 = f191(v7, v8)?;
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v39.clone() {
                                                Adt120::C0(h512, _) => {
                                                    h512
                                                }
                                            }, match v39 {
                                                Adt120::C0(_, h513) => {
                                                    h513
                                                }
                                            })))
                                        }
                                        Adt103::C5(v26, v27, v28, v29) => {
                                            let m509 = f192(v26, v27, v28, v29.clone());
                                            match m509 {
                                                Adt131::C0(v30) => {
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (Adt102::C1(v29, v30), v8)))
                                                }
                                                Adt131::C1(v31) => {
                                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C2(v31)))
                                                }
                                                Adt131::C2(v32) => {
                                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C3(v32)))
                                                }
                                                Adt131::C3(v33) => {
                                                    let v34: List<Adt103> = {
                                                        let a510 = v33;
                                                        let a511 = v8;
                                                        append_list(a510, a511)
                                                    };
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt102::C1(v40, v41) => {
                                    let v42: Adt120 = f193(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v42.clone() {
                                        Adt120::C0(h514, _) => {
                                            h514
                                        }
                                    }, match v42 {
                                        Adt120::C0(_, h515) => {
                                            h515
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155() -> u64 {
    67108864u64
}

pub fn f156(v0: List<Adt101>) -> R<Str> {
    let a520 = List::cons(Str::lit(""), f194(v0, List::<Str>::nil())?);
    let a521 = Str::lit("/");
    Ok(join(a520, a521))
}

pub fn f157(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f195({
        let m524 = match v1.clone() {
            Adt49::C0(h523, _) => {
                h523
            }
        };
        match m524 {
            Adt37::C0(h522, _, _) => {
                h522
            }
        }
    })? {
        f196(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f158(p529: u64, p530: List<Adt105>, p531: List<u64>) -> R<Result<List<u64>, Adt93>> {
    let m528 = (p529, (p530, p531));
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v0, (v1, v2)) = m528;
                let m525 = v0;
                if m525 == 0 {
                    Err::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(Adt93::C13)))
                } else {
                    let v3 = m525 - 1;
                    let m526 = v1;
                    match m526.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Ok::<List<u64>, Adt93>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m527 = f197(v4, v5, v2)?;
                            match m527 {
                                Adt132::C0(v6) => {
                                    Err::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(v6)))
                                }
                                Adt132::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(p534: List<u64>, p535: List<u64>) -> List<u64> {
    let m533 = (p534, p535);
    {
        let mut m533 = m533;
        loop {
            let step = {
                let (v0, v1) = m533;
                let m532 = v0;
                match m532.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Adt92) -> Adt6 {
    let m536 = v0;
    match m536 {
        Adt92::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt92::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f161(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m537 = v0;
    match m537 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f162() -> i64 {
    -32700i64
}

pub fn f163() -> i64 {
    -32600i64
}

pub fn f164(v0: Adt95) -> Str {
    let m538 = v0;
    match m538 {
        Adt95::C0 => {
            Str::lit("Invalid Request: the body is not a JSON object")
        }
        Adt95::C1 => {
            Str::lit("Invalid Request: batches are not supported")
        }
        Adt95::C2 => {
            Str::lit("Invalid Request: jsonrpc must be 2.0")
        }
        Adt95::C3 => {
            Str::lit("Invalid Request: method is required")
        }
        Adt95::C4 => {
            Str::lit("Invalid Request: method must be a string")
        }
        Adt95::C5 => {
            Str::lit("Invalid Request: ident must be a string or an integer")
        }
        Adt95::C6 => {
            Str::lit("Invalid Request: params must be an object")
        }
        Adt95::C7 => {
            Str::lit("Invalid Request: notifications are not accepted")
        }
        Adt95::C8 => {
            Str::lit("Invalid Request: the client must not send responses")
        }
    }
}

pub fn f165(v0: bool, v1: Adt121) -> R<Adt88> {
    let m540 = match v1.clone() {
        Adt121::C0(_, h539) => {
            h539
        }
    };
    match m540 {
        Ok(v2) => {
            let m541 = f198(v2.clone())?;
            match m541 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f199(v0, f62(f200(), f201(match v1 {
                        Adt121::C0(h542, _) => {
                            h542
                        }
                    }, v3))?, f202(match v2 {
                        Adt122::C0(_, _, h543) => {
                            h543
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f62(f203(v4.clone()), f64(Some(match v1 {
                Adt121::C0(h544, _) => {
                    h544
                }
            }), f204(v4)?))
        }
    }
}

pub fn f166(p549: List<Adt86>) -> R<bool> {
    let m548 = p549;
    {
        let mut m548 = m548;
        loop {
            let step = {
                let v0 = m548;
                let m545 = v0;
                match m545.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f137(match v1.clone() {
                            Adt86::C0(h546, _) => {
                                h546
                            }
                        }, f205())? {
                            if f206(f207(44u64, match v1 {
                                Adt86::C0(_, h547) => {
                                    h547
                                }
                            }, List::<u64>::nil(), List::<List<u64>>::nil()))? {
                                Err::<List<Adt86>, R<bool>>(Ok(true))
                            } else {
                                Ok::<List<Adt86>, R<bool>>(v2)
                            }
                        } else {
                            Ok::<List<Adt86>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f167(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt106>, v4: List<Str>, v5: Adt98) -> R<Adt121> {
    Ok(Adt121::C0(match v5.clone() {
        Adt98::C0(h550, _, _) => {
            h550
        }
    }, f208(v0, v1, v2, f209(v3, v4, v5)?)?))
}

pub fn f168(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt106> {
    let v4: Adt127 = f210(Adt108::C0, f71(v0.clone(), f211())?);
    let v5: Adt127 = f210(Adt108::C1, f71(v0.clone(), f212())?);
    let v6: Adt127 = if f213(v3) {
        f210(Adt108::C2, f214(f71(v0.clone(), f215())?)?)
    } else {
        f216()
    };
    Ok(Adt106::C0(match v4.clone() {
        Adt127::C0(h551, _) => {
            h551
        }
    }, match v5.clone() {
        Adt127::C0(h552, _) => {
            h552
        }
    }, match v6.clone() {
        Adt127::C0(h553, _) => {
            h553
        }
    }, f217(match v4 {
        Adt127::C0(_, h554) => {
            h554
        }
    }, f217(match v5 {
        Adt127::C0(_, h555) => {
            h555
        }
    }, match v6 {
        Adt127::C0(_, h556) => {
            h556
        }
    })), f218(v0, v1, v2)?))
}

pub fn f169(v0: Adt98) -> Option<Str> {
    let a558 = match v0.clone() {
        Adt98::C0(_, h557, _) => {
            h557
        }
    };
    let a559 = Str::lit("tools/call");
    if equal(a558, a559) {
        let m561 = match v0 {
            Adt98::C0(_, _, h560) => {
                h560
            }
        };
        match m561 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m562 = f172(Str::lit("name"), v1);
                match m562 {
                    Adt129::C0 => {
                        None::<Str>
                    }
                    Adt129::C1(v2) => {
                        f219(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f170() -> List<Str> {
    List::cons(f220(), List::<Str>::nil())
}

pub fn f171(v0: Adt5) -> R<Adt128> {
    let m563 = f172(Str::lit("id"), v0);
    match m563 {
        Adt129::C0 => {
            Ok(Adt128::C0)
        }
        Adt129::C1(v1) => {
            let m564 = v1;
            match m564 {
                Adt6::C0 => {
                    Ok(Adt128::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt128::C2)
                }
                Adt6::C2(v3) => {
                    let m565 = f221(v3)?;
                    match m565 {
                        None => {
                            Ok(Adt128::C2)
                        }
                        Some(v4) => {
                            Ok(Adt128::C1(Adt92::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt128::C1(Adt92::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt128::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt128::C2)
                }
            }
        }
    }
}

pub fn f172(v0: Str, v1: Adt5) -> Adt129 {
    f222(v0, match v1 {
        Adt5::C0(h566) => {
            h566
        }
    })
}

pub fn f173(v0: Adt5) -> Adt95 {
    if f174(v0) {
        Adt95::C8
    } else {
        Adt95::C3
    }
}

pub fn f174(v0: Adt5) -> bool {
    if f223(Str::lit("result"), v0.clone()) {
        true
    } else {
        f223(Str::lit("error"), v0)
    }
}

pub fn f175(v0: Adt92, v1: Adt5) -> Result<Adt98, Adt94> {
    let m567 = f172(Str::lit("jsonrpc"), v1.clone());
    match m567 {
        Adt129::C0 => {
            f133(Some(v0), Adt95::C2)
        }
        Adt129::C1(v2) => {
            let m568 = f219(v2);
            match m568 {
                None => {
                    f133(Some(v0), Adt95::C2)
                }
                Some(v3) => {
                    let a569 = v3;
                    let a570 = f124();
                    if equal(a569, a570) {
                        f224(v0, v1)
                    } else {
                        f133(Some(v0), Adt95::C2)
                    }
                }
            }
        }
    }
}

pub fn f176(v0: u64, v1: u64) -> Adt111 {
    let a571 = v1;
    let a572 = 0u64;
    if nat_eq(a571, a572) {
        Adt111::C0
    } else {
        let a573 = v1;
        let a574 = 2u64;
        if nat_eq(a573, a574) {
            Adt111::C2(v0)
        } else {
            Adt111::C1({
                let a578 = v0;
                let a579 = {
                    let a575 = v1;
                    let a576 = 3u64;
                    let a577 = 0u64;
                    nat_quot(a575, a576, a577)
                };
                nat_sub(a578, a579)
            })
        }
    }
}

pub fn f177(p607: Adt112, p608: u64, p609: u64, p610: List<u64>) -> R<u64> {
    let m606 = (p607, (p608, (p609, p610)));
    {
        let mut m606 = m606;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m606;
                let m580 = v3;
                match m580.uncons() {
                    None => {
                        let m581 = v0;
                        match m581 {
                            Adt112::C0 => {
                                Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt112::C1 => {
                                Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt112::C2 => {
                                Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt112::C3 => {
                                Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m582 = v0;
                        match m582 {
                            Adt112::C0 => {
                                let a583 = v4;
                                let a584 = 128u64;
                                if nat_lt(a583, a584) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a585 = v4;
                                    let a586 = 194u64;
                                    if nat_lt(a585, a586) {
                                        let v6: u64 = f225({
                                            let a587 = List::cons(v4, v5);
                                            length_list(a587)
                                        })?;
                                        Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a588 = v4;
                                        let a589 = 224u64;
                                        if nat_lt(a588, a589) {
                                            Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a590 = v4;
                                            let a591 = 224u64;
                                            if nat_eq(a590, a591) {
                                                Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a592 = v4;
                                                let a593 = 237u64;
                                                if nat_eq(a592, a593) {
                                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a594 = v4;
                                                    let a595 = 240u64;
                                                    if nat_lt(a594, a595) {
                                                        Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a596 = v4;
                                                        let a597 = 240u64;
                                                        if nat_eq(a596, a597) {
                                                            Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a598 = v4;
                                                            let a599 = 244u64;
                                                            if nat_eq(a598, a599) {
                                                                Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a600 = v4;
                                                                let a601 = 244u64;
                                                                if nat_lt(a600, a601) {
                                                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f225({
                                                                        let a602 = List::cons(v4, v5);
                                                                        length_list(a602)
                                                                    })?;
                                                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Adt112::C1 => {
                                if f226(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f225({
                                        let a603 = List::cons(v4, v5);
                                        length_list(a603)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt112::C2 => {
                                if f226(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f225({
                                        let a604 = List::cons(v4, v5);
                                        length_list(a604)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt112::C3 => {
                                if f226(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f225({
                                        let a605 = List::cons(v4, v5);
                                        length_list(a605)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m606 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: Adt113, v1: u64) -> R<Result<Adt6, Adt93>> {
    let m612 = match v0.clone() {
        Adt113::C0(h611, _, _) => {
            h611
        }
    };
    match m612 {
        Adt114::C0 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C1 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C2 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C3 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C4 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C5 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C7(_, _) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt114::C8(v3) => {
            let m613 = f227(v3)?;
            match m613 {
                Adt135::C0 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C7(v1)))
                }
                Adt135::C1 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C8(v1)))
                }
                Adt135::C2(v4) => {
                    let m618 = {
                        let m617 = f228(v4, match v0.clone() {
                            Adt113::C0(_, h615, _) => {
                                h615
                            }
                        }, match v0 {
                            Adt113::C0(_, _, h616) => {
                                h616
                            }
                        });
                        match m617 {
                            Adt113::C0(h614, _, _) => {
                                h614
                            }
                        }
                    };
                    match m618 {
                        Adt114::C0 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C1 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C2 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C3 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C4 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C5 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C7(_, _) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C8(_) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt114::C9(v5) => {
                            Ok(Ok::<Adt6, Adt93>(v5))
                        }
                    }
                }
            }
        }
        Adt114::C9(v2) => {
            Ok(Ok::<Adt6, Adt93>(v2))
        }
    }
}

pub fn f179(v0: Adt113, v1: u64, v2: u64) -> R<Adt130> {
    let m620 = match v0.clone() {
        Adt113::C0(h619, _, _) => {
            h619
        }
    };
    match m620 {
        Adt114::C0 => {
            if f229(v2) {
                Ok(Adt130::C1(v0))
            } else {
                f230(v0, v1, v2)
            }
        }
        Adt114::C1 => {
            if f229(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a621 = v2;
                let a622 = 93u64;
                if nat_eq(a621, a622) {
                    Ok(f231(v0, v1))
                } else {
                    f230(v0, v1, v2)
                }
            }
        }
        Adt114::C2 => {
            Ok(f232(v0, v1, v2))
        }
        Adt114::C3 => {
            if f229(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a623 = v2;
                let a624 = 125u64;
                if nat_eq(a623, a624) {
                    Ok(f233(v0, v1))
                } else {
                    let a625 = v2;
                    let a626 = 34u64;
                    if nat_eq(a625, a626) {
                        Ok(f234(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f235(Adt93::C2(v1)))
                    }
                }
            }
        }
        Adt114::C4 => {
            if f229(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a627 = v2;
                let a628 = 34u64;
                if nat_eq(a627, a628) {
                    Ok(f234(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f235(Adt93::C2(v1)))
                }
            }
        }
        Adt114::C5 => {
            if f229(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a629 = v2;
                let a630 = 58u64;
                if nat_eq(a629, a630) {
                    Ok(f234(v0, Adt114::C0))
                } else {
                    Ok(f235(Adt93::C2(v1)))
                }
            }
        }
        Adt114::C6(v3, v4, v5) => {
            f236(v0, v1, v2, v3, v4, v5)
        }
        Adt114::C7(v6, v7) => {
            f237(v0, v1, v2, v6, v7)
        }
        Adt114::C8(v8) => {
            f238(v0, v1, v2, v8)
        }
        Adt114::C9(_) => {
            Ok(f232(v0, v1, v2))
        }
    }
}

pub fn f180(v0: u64) -> R<u64> {
    let a631 = 64u64;
    let a632 = v0;
    if if nat_lt(a631, a632) {
        let a633 = v0;
        let a634 = 91u64;
        nat_lt(a633, a634)
    } else {
        false
    } {
        let a635 = v0;
        let a636 = 32u64;
        nat_add(a635, a636)
    } else {
        Ok(v0)
    }
}

pub fn f181(p639: List<u64>) -> List<u64> {
    let m638 = p639;
    {
        let mut m638 = m638;
        loop {
            let step = {
                let v0 = m638;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f239(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m638 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: u64) -> bool {
    let a640 = v0;
    let a641 = 9u64;
    if nat_eq(a640, a641) {
        true
    } else {
        let a642 = 31u64;
        let a643 = v0;
        if nat_lt(a642, a643) {
            let a644 = v0;
            let a645 = 127u64;
            nat_lt(a644, a645)
        } else {
            false
        }
    }
}

pub fn f183(p648: List<u64>) -> bool {
    let m647 = p648;
    {
        let mut m647 = m647;
        loop {
            let step = {
                let v0 = m647;
                let m646 = v0;
                match m646.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f240(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f184(p651: List<u64>, v1: bool) -> bool {
    let m650 = p651;
    {
        let mut m650 = m650;
        loop {
            let step = {
                let v0 = m650;
                let m649 = v0;
                match m649.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f241(v2)
                        } else {
                            f242(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185() -> u64 {
    64u64
}

pub fn f186(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m652 = f222(Str::lit("type"), v1);
    match m652 {
        Adt129::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt129::C1(v2) => {
            let m653 = f219(v2);
            match m653 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a654 = v3;
                    let a655 = Str::lit("object");
                    if equal(a654, a655) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f187(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m656 = f222(Str::lit("$schema"), v1);
    match m656 {
        Adt129::C0 => {
            None::<Adt91>
        }
        Adt129::C1(v2) => {
            let m657 = f219(v2);
            match m657 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a658 = v3;
                    let a659 = f243();
                    if equal(a658, a659) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f188(v0: Adt35) -> R<Option<Adt91>> {
    let m661 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h660, _, _) => {
            h660
        }
    };
    match m661 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m664 = f187(match v0.clone() {
                Adt35::C0(h662, _, _, _, _, _, _, _) => {
                    h662
                }
            }, match v1.clone() {
                Adt5::C0(h663) => {
                    h663
                }
            });
            match m664 {
                None => {
                    let m668 = f151(f152(), match v0 {
                        Adt35::C0(h665, _, _, _, _, _, _, _) => {
                            h665
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h666) => {
                            h666
                        }
                    }), List::cons(Adt99::C0(Adt6::C5(match v1 {
                        Adt5::C0(h667) => {
                            h667
                        }
                    }), false, List::<Str>::nil()), List::<Adt99>::nil()), f153())?;
                    match m668 {
                        Ok(_) => {
                            Ok(None::<Adt91>)
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

pub fn f189(v0: Str, v1: Adt6, v2: Adt99, v3: Adt119, v4: List<Adt99>) -> R<Result<(List<Adt99>, Adt119), Adt91>> {
    let m669 = v2;
    match m669 {
        Adt99::C0(v5, v6, v7) => {
            let m670 = f244(v5.clone());
            match m670 {
                None => {
                    let m671 = f245(v5);
                    match m671 {
                        None => {
                            Ok(Err::<(List<Adt99>, Adt119), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f246(v0, v1, match v9 {
                                Adt5::C0(h672) => {
                                    h672
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt99>, Adt119), Adt91>((v4, v3)))
                }
            }
        }
    }
}

pub fn f190(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    let m673 = f244(v1.clone());
    match m673 {
        None => {
            let m674 = f245(v1);
            match m674 {
                None => {
                    Ok(Adt131::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f247(v0, match v5 {
                        Adt5::C0(h675) => {
                            h675
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt131::C3(List::<Adt103>::nil()))
            } else {
                Ok(Adt131::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f191(v0: Adt103, v1: List<Adt103>) -> R<Adt120> {
    let m676 = v0;
    match m676 {
        Adt103::C0(_, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
        Adt103::C1(_, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
        Adt103::C2(v8, v9, v10, v11) => {
            Ok(f248(v8, v9, v10, {
                let a677 = v11;
                let a678 = 1u64;
                nat_add(a677, a678)?
            }, v1))
        }
        Adt103::C3(v12) => {
            Ok(Adt120::C0(Adt102::C1(v12, Str::lit("not")), v1))
        }
        Adt103::C4(v13, _, v15, v16) => {
            Ok(f249(v13, v15, v16, v1))
        }
        Adt103::C5(_, _, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
    }
}

pub fn f192(v0: Adt104, v1: Adt6, v2: Adt6, v3: List<Adt101>) -> Adt131 {
    let m679 = f245(v1);
    match m679 {
        None => {
            Adt131::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m680 = v0;
            match m680 {
                Adt104::C0 => {
                    f250(match v4 {
                        Adt5::C0(h681) => {
                            h681
                        }
                    }, v2, v3)
                }
                Adt104::C1 => {
                    f251(match v4 {
                        Adt5::C0(h682) => {
                            h682
                        }
                    }, v2, v3)
                }
                Adt104::C2 => {
                    f252(match v4 {
                        Adt5::C0(h683) => {
                            h683
                        }
                    }, v2, v3)
                }
                Adt104::C3 => {
                    f253(match v4 {
                        Adt5::C0(h684) => {
                            h684
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f193(v0: Adt103, v1: List<Adt103>, v2: List<Adt101>, v3: Str) -> Adt120 {
    let m685 = v0;
    match m685 {
        Adt103::C0(_, _, _) => {
            Adt120::C0(Adt102::C1(v2, v3), v1)
        }
        Adt103::C1(v7, v8, v9) => {
            f254(v7, v8, v9, v1)
        }
        Adt103::C2(v10, v11, v12, v13) => {
            f248(v10, v11, v12, v13, v1)
        }
        Adt103::C3(_) => {
            Adt120::C0(Adt102::C0, v1)
        }
        Adt103::C4(_, v16, v17, v18) => {
            f249(v16, v17, v18, v1)
        }
        Adt103::C5(_, _, _, _) => {
            Adt120::C0(Adt102::C1(v2, v3), v1)
        }
    }
}

pub fn f194(p688: List<Adt101>, p689: List<Str>) -> R<List<Str>> {
    let m687 = (p688, p689);
    {
        let mut m687 = m687;
        loop {
            let step = {
                let (v0, v1) = m687;
                let m686 = v0;
                match m686.uncons() {
                    None => {
                        Err::<(List<Adt101>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt101>, List<Str>), R<List<Str>>>((v3, List::cons(f255(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m687 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(p698: List<Adt38>) -> R<bool> {
    let m697 = p698;
    {
        let mut m697 = m697;
        loop {
            let step = {
                let v0 = m697;
                let m690 = v0;
                match m690.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m691 = v1;
                        match m691 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f256(match v4 {
                                    Adt42::C0(h692, _, _, _) => {
                                        h692
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f256(match v5 {
                                    Adt43::C0(h693, _, _, _) => {
                                        h693
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
                                let m695 = match v7 {
                                    Adt45::C0(h694, _, _) => {
                                        h694
                                    }
                                };
                                match m695 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f256(match v9 {
                                            Adt48::C0(_, _, h696, _) => {
                                                h696
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
                Ok(next) => { m697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f257(match v1.clone() {
        Adt49::C0(h699, _) => {
            h699
        }
    }) {
        f258(v1)
    } else {
        let m701 = match v0 {
            Adt35::C0(_, _, _, _, _, h700, _, _) => {
                h700
            }
        };
        match m701 {
            None => {
                f258(v1)
            }
            Some(v2) => {
                f259(v2, v1)
            }
        }
    }
}

pub fn f197(v0: Adt105, v1: List<Adt105>, v2: List<u64>) -> R<Adt132> {
    let m702 = v0;
    match m702 {
        Adt105::C0(v3) => {
            Ok(Adt132::C1(v1, f159(v3, v2)))
        }
        Adt105::C1(v4, v5) => {
            f260(v4, v5, v1, v2)
        }
        Adt105::C2(v6, v7) => {
            let m703 = v7;
            match m703.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C1(v6, v8), List::cons(Adt105::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt105::C3(v10, v11) => {
            let m704 = v11.clone();
            match m704.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt105::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt105::C4(v14, v15) => {
            let m705 = v15;
            match m705.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C0({
                        let a707 = f261({
                            let (h706, _) = v16;
                            h706
                        })?;
                        let a708 = List::cons(58u64, List::<u64>::nil());
                        append_list(a707, a708)
                    }), List::cons(Adt105::C1(v14, {
                        let (_, h709) = v16;
                        h709
                    }), List::cons(Adt105::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt105::C5(v18, v19) => {
            let m710 = v19.clone();
            match m710.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt105::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f198(v0: Adt122) -> R<Option<Adt5>> {
    let m712 = f262(match v0.clone() {
        Adt122::C0(h711, _, _) => {
            h711
        }
    })?;
    match m712 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a715 = v1;
                let a716 = {
                    let m714 = f263(v0);
                    match m714 {
                        Adt5::C0(h713) => {
                            h713
                        }
                    }
                };
                append_list(a715, a716)
            })))
        }
    }
}

pub fn f199(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m717 = v2;
    match m717 {
        None => {
            f6()
        }
        Some(v3) => {
            f264(v0, v1, f38(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f200() -> u64 {
    200u64
}

pub fn f201(v0: Adt92, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f124())), List::cons((Str::lit("id"), f160(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h718) => {
            h718
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f202(p722: List<Adt124>, p723: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m721 = (p722, p723);
    {
        let mut m721 = m721;
        loop {
            let step = {
                let (v0, v1) = m721;
                let m719 = v0;
                match m719.uncons() {
                    None => {
                        Err::<(List<Adt124>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m720 = f265(v2)?;
                        match m720 {
                            None => {
                                Err::<(List<Adt124>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt124>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m721 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Adt125) -> u64 {
    let m724 = v0;
    match m724 {
        Adt125::C0(_) => {
            f94()
        }
        Adt125::C1(_, _) => {
            f94()
        }
        Adt125::C2(_) => {
            f94()
        }
        Adt125::C3(_) => {
            f266()
        }
        Adt125::C4(v6) => {
            f267(v6)
        }
    }
}

pub fn f204(v0: Adt125) -> R<Adt80> {
    Ok(Adt80::C0(f268(v0.clone()), f269(v0.clone()), f270(v0)?))
}

pub fn f205() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f206(p727: List<List<u64>>) -> R<bool> {
    let m726 = p727;
    {
        let mut m726 = m726;
        loop {
            let step = {
                let v0 = m726;
                let m725 = v0;
                match m725.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f271(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f207(v0: u64, p732: List<u64>, p733: List<u64>, p734: List<List<u64>>) -> List<List<u64>> {
    let m731 = (p732, (p733, p734));
    {
        let mut m731 = m731;
        loop {
            let step = {
                let (v1, (v2, v3)) = m731;
                let m728 = v1;
                match m728.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f38(List::cons(f147(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a729 = v4;
                        let a730 = v0;
                        if nat_eq(a729, a730) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f147(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f208(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt133, Adt125>) -> R<Result<Adt122, Adt125>> {
    let m735 = v3;
    match m735 {
        Ok(v5) => {
            f272(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt122, Adt125>(v4))
        }
    }
}

pub fn f209(v0: Option<Adt106>, v1: List<Str>, v2: Adt98) -> R<Result<Adt133, Adt125>> {
    Ok(f273(f274(match v2.clone() {
        Adt98::C0(_, h736, _) => {
            h736
        }
    }), f275(v0, v1, v2)?))
}

pub fn f210(v0: Adt108, v1: Adt96) -> Adt127 {
    let m737 = v1;
    match m737 {
        Adt96::C0 => {
            f216()
        }
        Adt96::C1 => {
            Adt127::C0(None::<Str>, Some(Adt107::C6(v0)))
        }
        Adt96::C2 => {
            Adt127::C0(None::<Str>, Some(Adt107::C7(v0)))
        }
        Adt96::C3(v2) => {
            let m738 = f276(v2);
            match m738 {
                None => {
                    Adt127::C0(None::<Str>, Some(Adt107::C6(v0)))
                }
                Some(v3) => {
                    Adt127::C0(Some(v3), None::<Adt107>)
                }
            }
        }
    }
}

pub fn f211() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f212() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f213(v0: Str) -> bool {
    let m739 = f277(v0);
    match m739 {
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
            true
        }
    }
}

pub fn f214(v0: Adt96) -> R<Adt96> {
    let m740 = v0.clone();
    match m740 {
        Adt96::C0 => {
            Ok(v0)
        }
        Adt96::C1 => {
            Ok(v0)
        }
        Adt96::C2 => {
            Ok(v0)
        }
        Adt96::C3(v1) => {
            let m741 = f278(v1);
            match m741 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m742 = f279(v2)?;
                    match m742 {
                        None => {
                            Ok(Adt96::C1)
                        }
                        Some(v3) => {
                            Ok(Adt96::C3(v3))
                        }
                    }
                }
            }
        }
    }
}

pub fn f215() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f216() -> Adt127 {
    Adt127::C0(None::<Str>, None::<Adt107>)
}

pub fn f217(v0: Option<Adt107>, v1: Option<Adt107>) -> Option<Adt107> {
    let m743 = v0.clone();
    match m743 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f218(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt109>> {
    Ok(f280(f281(v0, v1, v2, List::<Adt109>::nil())?, List::<Adt109>::nil()))
}

pub fn f219(v0: Adt6) -> Option<Str> {
    let m744 = v0;
    match m744 {
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

pub fn f220() -> Str {
    Str::lit("2026-07-28")
}

pub fn f221(v0: Adt7) -> R<Option<i64>> {
    let m747 = f282(40u64, match v0.clone() {
        Adt7::C0(h745, _) => {
            h745
        }
    }, match v0 {
        Adt7::C0(_, h746) => {
            h746
        }
    })?;
    match m747 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f283(v1))
        }
    }
}

pub fn f222(v0: Str, p753: List<(Str, Adt6)>) -> Adt129 {
    let m752 = p753;
    {
        let mut m752 = m752;
        loop {
            let step = {
                let v1 = m752;
                let m748 = v1;
                match m748.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt129>(Adt129::C0)
                    }
                    Some((v2, v3)) => {
                        let m750 = f284({
                            let (h749, _) = v2;
                            h749
                        }, v0.clone());
                        match m750 {
                            Adt140::C0 => {
                                Err::<List<(Str, Adt6)>, Adt129>(Adt129::C1({
                                    let (_, h751) = v2;
                                    h751
                                }))
                            }
                            Adt140::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt129>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m752 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Str, v1: Adt5) -> bool {
    f285(f172(v0, v1))
}

pub fn f224(v0: Adt92, v1: Adt5) -> Result<Adt98, Adt94> {
    let m754 = f172(Str::lit("method"), v1.clone());
    match m754 {
        Adt129::C0 => {
            f133(Some(v0), f173(v1))
        }
        Adt129::C1(v2) => {
            let m755 = f219(v2);
            match m755 {
                None => {
                    f133(Some(v0), Adt95::C4)
                }
                Some(v3) => {
                    f286(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f225(v0: u64) -> R<u64> {
    let a758 = {
        let a756 = v0;
        let a757 = 3u64;
        nat_mul(a756, a757)?
    };
    let a759 = 1u64;
    nat_add(a758, a759)
}

pub fn f226(v0: u64, v1: u64, v2: u64) -> bool {
    let a760 = v1;
    let a761 = v0;
    if nat_le(a760, a761) {
        let a762 = v0;
        let a763 = v2;
        nat_le(a762, a763)
    } else {
        false
    }
}

pub fn f227(v0: Adt116) -> R<Adt135> {
    if f287(match v0.clone() {
        Adt116::C0(_, h764, _, _, _, _, _) => {
            h764
        }
    }) {
        let v1: i64 = {
            let a770 = if match v0.clone() {
                Adt116::C0(_, _, _, _, _, h765, _) => {
                    h765
                }
            } {
                let a767 = match v0.clone() {
                    Adt116::C0(_, _, _, _, _, _, h766) => {
                        h766
                    }
                };
                int_neg(a767)?
            } else {
                match v0.clone() {
                    Adt116::C0(_, _, _, _, _, _, h768) => {
                        h768
                    }
                }
            };
            let a771 = match v0.clone() {
                Adt116::C0(_, _, _, _, h769, _, _) => {
                    h769
                }
            };
            int_sub(a770, a771)?
        };
        let m774 = {
            let a772 = v1;
            let a773 = -9999i64;
            compare(a772, a773)
        };
        if if match m774 {
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
            let m777 = {
                let a775 = 9999i64;
                let a776 = v1;
                compare(a775, a776)
            };
            match m777 {
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
            Ok(Adt135::C1)
        } else {
            Ok(Adt135::C2(Adt6::C2(Adt7::C0(if match v0.clone() {
                Adt116::C0(h778, _, _, _, _, _, _) => {
                    h778
                }
            } {
                let a780 = match v0 {
                    Adt116::C0(_, _, h779, _, _, _, _) => {
                        h779
                    }
                };
                int_neg(a780)?
            } else {
                match v0 {
                    Adt116::C0(_, _, h781, _, _, _, _) => {
                        h781
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f228(v0: Adt6, v1: List<Adt118>, v2: u64) -> Adt113 {
    let m782 = v1.clone();
    match m782.uncons() {
        None => {
            Adt113::C0(Adt114::C9(v0), List::<Adt118>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m783 = v3;
            match m783 {
                Adt118::C0(v5) => {
                    Adt113::C0(Adt114::C2, List::cons(Adt118::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt118::C1(v6, v7) => {
                    let m784 = v7;
                    match m784 {
                        None => {
                            Adt113::C0(Adt114::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt113::C0(Adt114::C2, List::cons(Adt118::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f229(v0: u64) -> bool {
    let a785 = v0;
    let a786 = 32u64;
    if nat_eq(a785, a786) {
        true
    } else {
        let a787 = v0;
        let a788 = 10u64;
        if nat_eq(a787, a788) {
            true
        } else {
            let a789 = v0;
            let a790 = 13u64;
            if nat_eq(a789, a790) {
                true
            } else {
                let a791 = v0;
                let a792 = 9u64;
                nat_eq(a791, a792)
            }
        }
    }
}

pub fn f230(v0: Adt113, v1: u64, v2: u64) -> R<Adt130> {
    let a793 = v2;
    let a794 = 34u64;
    if nat_eq(a793, a794) {
        Ok(f234(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), false)))
    } else {
        let a795 = v2;
        let a796 = 123u64;
        if nat_eq(a795, a796) {
            f288(v0, Adt118::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt114::C3, v1)
        } else {
            let a797 = v2;
            let a798 = 91u64;
            if nat_eq(a797, a798) {
                f288(v0, Adt118::C0(List::<Adt6>::nil()), Adt114::C1, v1)
            } else {
                let a799 = v2;
                let a800 = 116u64;
                if nat_eq(a799, a800) {
                    Ok(f234(v0, Adt114::C7(0u64, 1u64)))
                } else {
                    let a801 = v2;
                    let a802 = 102u64;
                    if nat_eq(a801, a802) {
                        Ok(f234(v0, Adt114::C7(1u64, 1u64)))
                    } else {
                        let a803 = v2;
                        let a804 = 110u64;
                        if nat_eq(a803, a804) {
                            Ok(f234(v0, Adt114::C7(2u64, 1u64)))
                        } else {
                            let a805 = v2;
                            let a806 = 45u64;
                            if nat_eq(a805, a806) {
                                Ok(f234(v0, f289(true, Adt117::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a807 = v2;
                                let a808 = 48u64;
                                if nat_eq(a807, a808) {
                                    Ok(f234(v0, f289(false, Adt117::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f226(v2, 49u64, 57u64) {
                                        Ok(f234(v0, f289(false, Adt117::C2, f290({
                                            let a809 = v2;
                                            let a810 = 48u64;
                                            nat_sub(a809, a810)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f235(Adt93::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Adt113, v1: u64) -> Adt130 {
    let m812 = match v0.clone() {
        Adt113::C0(_, h811, _) => {
            h811
        }
    };
    match m812.uncons() {
        None => {
            f235(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m813 = v2;
            match m813 {
                Adt118::C0(v4) => {
                    Adt130::C1(f228(Adt6::C4(f291(v4, List::<Adt6>::nil())), v3, {
                        let a815 = match v0 {
                            Adt113::C0(_, _, h814) => {
                                h814
                            }
                        };
                        let a816 = 1u64;
                        nat_sub(a815, a816)
                    }))
                }
                Adt118::C1(_, _) => {
                    f235(Adt93::C2(v1))
                }
            }
        }
    }
}

pub fn f232(v0: Adt113, v1: u64, v2: u64) -> Adt130 {
    let m818 = match v0.clone() {
        Adt113::C0(h817, _, _) => {
            h817
        }
    };
    match m818 {
        Adt114::C0 => {
            f235(Adt93::C2(v1))
        }
        Adt114::C1 => {
            f235(Adt93::C2(v1))
        }
        Adt114::C2 => {
            if f229(v2) {
                Adt130::C1(v0)
            } else {
                let a819 = v2;
                let a820 = 44u64;
                if nat_eq(a819, a820) {
                    f292(v0, v1)
                } else {
                    let a821 = v2;
                    let a822 = 93u64;
                    if nat_eq(a821, a822) {
                        f231(v0, v1)
                    } else {
                        let a823 = v2;
                        let a824 = 125u64;
                        if nat_eq(a823, a824) {
                            f233(v0, v1)
                        } else {
                            f235(Adt93::C2(v1))
                        }
                    }
                }
            }
        }
        Adt114::C3 => {
            f235(Adt93::C2(v1))
        }
        Adt114::C4 => {
            f235(Adt93::C2(v1))
        }
        Adt114::C5 => {
            f235(Adt93::C2(v1))
        }
        Adt114::C6(_, _, _) => {
            f235(Adt93::C2(v1))
        }
        Adt114::C7(_, _) => {
            f235(Adt93::C2(v1))
        }
        Adt114::C8(_) => {
            f235(Adt93::C2(v1))
        }
        Adt114::C9(_) => {
            if f229(v2) {
                Adt130::C1(v0)
            } else {
                f235(Adt93::C10(v1))
            }
        }
    }
}

pub fn f233(v0: Adt113, v1: u64) -> Adt130 {
    let m826 = match v0.clone() {
        Adt113::C0(_, h825, _) => {
            h825
        }
    };
    match m826.uncons() {
        None => {
            f235(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m827 = v2;
            match m827 {
                Adt118::C0(_) => {
                    f235(Adt93::C2(v1))
                }
                Adt118::C1(v5, _) => {
                    Adt130::C1(f228(Adt6::C5(f293(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a829 = match v0 {
                            Adt113::C0(_, _, h828) => {
                                h828
                            }
                        };
                        let a830 = 1u64;
                        nat_sub(a829, a830)
                    }))
                }
            }
        }
    }
}

pub fn f234(v0: Adt113, v1: Adt114) -> Adt130 {
    Adt130::C1(f294(v0, v1))
}

pub fn f235(v0: Adt93) -> Adt130 {
    Adt130::C0(v0)
}

pub fn f236(v0: Adt113, v1: u64, v2: u64, v3: Adt115, v4: List<u64>, v5: bool) -> R<Adt130> {
    let m831 = v3;
    match m831 {
        Adt115::C0 => {
            let a832 = v2;
            let a833 = 34u64;
            if nat_eq(a832, a833) {
                Ok(f295(v0, v1, v4, v5))
            } else {
                let a834 = v2;
                let a835 = 92u64;
                if nat_eq(a834, a835) {
                    Ok(f234(v0, Adt114::C6(Adt115::C1, v4, v5)))
                } else {
                    let a836 = v2;
                    let a837 = 32u64;
                    if nat_lt(a836, a837) {
                        Ok(f235(Adt93::C4(v1)))
                    } else {
                        Ok(f234(v0, Adt114::C6(Adt115::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt115::C1 => {
            let a838 = v2;
            let a839 = 117u64;
            if nat_eq(a838, a839) {
                Ok(f234(v0, Adt114::C6(Adt115::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a840 = f296(v2);
                let a841 = 0u64;
                if nat_eq(a840, a841) {
                    Ok(f235(Adt93::C5(v1)))
                } else {
                    Ok(f234(v0, Adt114::C6(Adt115::C0, List::cons(f296(v2), v4), v5)))
                }
            }
        }
        Adt115::C2(v6, v7, v8) => {
            let a842 = 15u64;
            let a843 = f297(v2);
            if nat_lt(a842, a843) {
                Ok(f235(Adt93::C5(v1)))
            } else {
                let a846 = {
                    let a844 = v6;
                    let a845 = 1u64;
                    nat_add(a844, a845)?
                };
                let a847 = 4u64;
                if nat_eq(a846, a847) {
                    f298(v0, v1, {
                        let a850 = {
                            let a848 = v7;
                            let a849 = 16u64;
                            nat_mul(a848, a849)?
                        };
                        let a851 = f297(v2);
                        nat_add(a850, a851)?
                    }, v8, v4, v5)
                } else {
                    Ok(f234(v0, Adt114::C6(Adt115::C2({
                        let a852 = v6;
                        let a853 = 1u64;
                        nat_add(a852, a853)?
                    }, {
                        let a856 = {
                            let a854 = v7;
                            let a855 = 16u64;
                            nat_mul(a854, a855)?
                        };
                        let a857 = f297(v2);
                        nat_add(a856, a857)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt115::C3(v9) => {
            let a858 = v2;
            let a859 = 92u64;
            if nat_eq(a858, a859) {
                Ok(f234(v0, Adt114::C6(Adt115::C4(v9), v4, v5)))
            } else {
                Ok(f235(Adt93::C6(v1)))
            }
        }
        Adt115::C4(v10) => {
            let a860 = v2;
            let a861 = 117u64;
            if nat_eq(a860, a861) {
                Ok(f234(v0, Adt114::C6(Adt115::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f235(Adt93::C6(v1)))
            }
        }
    }
}

pub fn f237(v0: Adt113, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt130> {
    let a862 = v2;
    let a863 = f299(v3, v4);
    if nat_eq(a862, a863) {
        let a867 = {
            let a864 = v4;
            let a865 = 1u64;
            nat_add(a864, a865)?
        };
        let a868 = {
            let a866 = f300(v3);
            length_list(a866)
        };
        if nat_eq(a867, a868) {
            Ok(f301(v0, f302(v3)))
        } else {
            Ok(f234(v0, Adt114::C7(v3, {
                let a869 = v4;
                let a870 = 1u64;
                nat_add(a869, a870)?
            })))
        }
    } else {
        Ok(f235(Adt93::C2(v1)))
    }
}

pub fn f238(v0: Adt113, v1: u64, v2: u64, v3: Adt116) -> R<Adt130> {
    let m872 = match v3.clone() {
        Adt116::C0(_, h871, _, _, _, _, _) => {
            h871
        }
    };
    match m872 {
        Adt117::C0 => {
            let a873 = v2;
            let a874 = 48u64;
            if nat_eq(a873, a874) {
                Ok(f303(v0, v3, Adt117::C1))
            } else {
                if f226(v2, 49u64, 57u64) {
                    f304(v0, v1, v2, v3, false)
                } else {
                    Ok(f235(Adt93::C7(v1)))
                }
            }
        }
        Adt117::C1 => {
            let a875 = v2;
            let a876 = 46u64;
            if nat_eq(a875, a876) {
                Ok(f303(v0, v3, Adt117::C3))
            } else {
                if f305(v2) {
                    Ok(f303(v0, v3, Adt117::C5))
                } else {
                    if f306(v2) {
                        Ok(f235(Adt93::C7(v1)))
                    } else {
                        f307(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt117::C2 => {
            if f306(v2) {
                f304(v0, v1, v2, v3, false)
            } else {
                let a877 = v2;
                let a878 = 46u64;
                if nat_eq(a877, a878) {
                    Ok(f303(v0, v3, Adt117::C3))
                } else {
                    if f305(v2) {
                        Ok(f303(v0, v3, Adt117::C5))
                    } else {
                        f307(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt117::C3 => {
            if f306(v2) {
                f304(v0, v1, v2, v3, true)
            } else {
                Ok(f235(Adt93::C7(v1)))
            }
        }
        Adt117::C4 => {
            if f306(v2) {
                f304(v0, v1, v2, v3, true)
            } else {
                if f305(v2) {
                    Ok(f303(v0, v3, Adt117::C5))
                } else {
                    f307(v0, v1, v2, v3)
                }
            }
        }
        Adt117::C5 => {
            let a879 = v2;
            let a880 = 43u64;
            if nat_eq(a879, a880) {
                Ok(f234(v0, f289(match v3.clone() {
                    Adt116::C0(h881, _, _, _, _, _, _) => {
                        h881
                    }
                }, Adt117::C6, match v3.clone() {
                    Adt116::C0(_, _, h882, _, _, _, _) => {
                        h882
                    }
                }, match v3.clone() {
                    Adt116::C0(_, _, _, h883, _, _, _) => {
                        h883
                    }
                }, match v3 {
                    Adt116::C0(_, _, _, _, h884, _, _) => {
                        h884
                    }
                }, false, 0i64)))
            } else {
                let a885 = v2;
                let a886 = 45u64;
                if nat_eq(a885, a886) {
                    Ok(f234(v0, f289(match v3.clone() {
                        Adt116::C0(h887, _, _, _, _, _, _) => {
                            h887
                        }
                    }, Adt117::C6, match v3.clone() {
                        Adt116::C0(_, _, h888, _, _, _, _) => {
                            h888
                        }
                    }, match v3.clone() {
                        Adt116::C0(_, _, _, h889, _, _, _) => {
                            h889
                        }
                    }, match v3 {
                        Adt116::C0(_, _, _, _, h890, _, _) => {
                            h890
                        }
                    }, true, 0i64)))
                } else {
                    if f306(v2) {
                        f308(v0, v1, v2, Adt116::C0(match v3.clone() {
                            Adt116::C0(h891, _, _, _, _, _, _) => {
                                h891
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, h892, _, _, _, _, _) => {
                                h892
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, _, h893, _, _, _, _) => {
                                h893
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, _, _, h894, _, _, _) => {
                                h894
                            }
                        }, match v3 {
                            Adt116::C0(_, _, _, _, h895, _, _) => {
                                h895
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f235(Adt93::C7(v1)))
                    }
                }
            }
        }
        Adt117::C6 => {
            if f306(v2) {
                f308(v0, v1, v2, v3.clone(), match v3 {
                    Adt116::C0(_, _, _, _, _, h896, _) => {
                        h896
                    }
                })
            } else {
                Ok(f235(Adt93::C7(v1)))
            }
        }
        Adt117::C7 => {
            if f306(v2) {
                f308(v0, v1, v2, v3.clone(), match v3 {
                    Adt116::C0(_, _, _, _, _, h897, _) => {
                        h897
                    }
                })
            } else {
                f307(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f239(v0: u64) -> bool {
    let a898 = v0;
    let a899 = 9u64;
    if nat_eq(a898, a899) {
        true
    } else {
        let a900 = v0;
        let a901 = 32u64;
        nat_eq(a900, a901)
    }
}

pub fn f240(v0: u64) -> bool {
    let a902 = 47u64;
    let a903 = v0;
    if nat_lt(a902, a903) {
        let a904 = v0;
        let a905 = 58u64;
        nat_lt(a904, a905)
    } else {
        false
    }
}

pub fn f241(v0: u64) -> bool {
    let a906 = 48u64;
    let a907 = v0;
    if if nat_le(a906, a907) {
        let a908 = v0;
        let a909 = 57u64;
        nat_le(a908, a909)
    } else {
        false
    } {
        true
    } else {
        let a910 = 65u64;
        let a911 = v0;
        if if nat_le(a910, a911) {
            let a912 = v0;
            let a913 = 90u64;
            nat_le(a912, a913)
        } else {
            false
        } {
            true
        } else {
            let a914 = 97u64;
            let a915 = v0;
            if if nat_le(a914, a915) {
                let a916 = v0;
                let a917 = 122u64;
                nat_le(a916, a917)
            } else {
                false
            } {
                true
            } else {
                let a918 = v0;
                let a919 = 33u64;
                if nat_eq(a918, a919) {
                    true
                } else {
                    let a920 = v0;
                    let a921 = 35u64;
                    if nat_eq(a920, a921) {
                        true
                    } else {
                        let a922 = v0;
                        let a923 = 36u64;
                        if nat_eq(a922, a923) {
                            true
                        } else {
                            let a924 = v0;
                            let a925 = 37u64;
                            if nat_eq(a924, a925) {
                                true
                            } else {
                                let a926 = v0;
                                let a927 = 38u64;
                                if nat_eq(a926, a927) {
                                    true
                                } else {
                                    let a928 = v0;
                                    let a929 = 39u64;
                                    if nat_eq(a928, a929) {
                                        true
                                    } else {
                                        let a930 = v0;
                                        let a931 = 42u64;
                                        if nat_eq(a930, a931) {
                                            true
                                        } else {
                                            let a932 = v0;
                                            let a933 = 43u64;
                                            if nat_eq(a932, a933) {
                                                true
                                            } else {
                                                let a934 = v0;
                                                let a935 = 45u64;
                                                if nat_eq(a934, a935) {
                                                    true
                                                } else {
                                                    let a936 = v0;
                                                    let a937 = 46u64;
                                                    if nat_eq(a936, a937) {
                                                        true
                                                    } else {
                                                        let a938 = v0;
                                                        let a939 = 94u64;
                                                        if nat_eq(a938, a939) {
                                                            true
                                                        } else {
                                                            let a940 = v0;
                                                            let a941 = 95u64;
                                                            if nat_eq(a940, a941) {
                                                                true
                                                            } else {
                                                                let a942 = v0;
                                                                let a943 = 96u64;
                                                                if nat_eq(a942, a943) {
                                                                    true
                                                                } else {
                                                                    let a944 = v0;
                                                                    let a945 = 124u64;
                                                                    if nat_eq(a944, a945) {
                                                                        true
                                                                    } else {
                                                                        let a946 = v0;
                                                                        let a947 = 126u64;
                                                                        nat_eq(a946, a947)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f242(v0: u64) -> bool {
    let a948 = 48u64;
    let a949 = v0;
    if if nat_le(a948, a949) {
        let a950 = v0;
        let a951 = 57u64;
        nat_le(a950, a951)
    } else {
        false
    } {
        true
    } else {
        let a952 = 65u64;
        let a953 = v0;
        if if nat_le(a952, a953) {
            let a954 = v0;
            let a955 = 90u64;
            nat_le(a954, a955)
        } else {
            false
        } {
            true
        } else {
            let a956 = 97u64;
            let a957 = v0;
            if if nat_le(a956, a957) {
                let a958 = v0;
                let a959 = 122u64;
                nat_le(a958, a959)
            } else {
                false
            } {
                true
            } else {
                let a960 = v0;
                let a961 = 95u64;
                if nat_eq(a960, a961) {
                    true
                } else {
                    let a962 = v0;
                    let a963 = 45u64;
                    if nat_eq(a962, a963) {
                        true
                    } else {
                        let a964 = v0;
                        let a965 = 46u64;
                        nat_eq(a964, a965)
                    }
                }
            }
        }
    }
}

pub fn f243() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f244(v0: Adt6) -> Option<bool> {
    let m966 = v0;
    match m966 {
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

pub fn f245(v0: Adt6) -> Option<Adt5> {
    let m967 = v0;
    match m967 {
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

pub fn f246(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt119, v6: List<Adt99>) -> R<Result<(List<Adt99>, Adt119), Adt91>> {
    let m968 = f309(v0.clone(), v1, v2.clone());
    match m968 {
        None => {
            let m969 = f310(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m969 {
                Adt141::C0(v8) => {
                    Ok(Err::<(List<Adt99>, Adt119), Adt91>(v8))
                }
                Adt141::C1(v9) => {
                    Ok(Ok::<(List<Adt99>, Adt119), Adt91>((f311(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt99>, Adt119), Adt91>(v7))
        }
    }
}

pub fn f247(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    let m970 = f312(v1.clone(), v2.clone())?;
    match m970 {
        Adt131::C0(v4) => {
            Ok(Adt131::C0(v4))
        }
        Adt131::C1(v5) => {
            Ok(Adt131::C1(v5))
        }
        Adt131::C2(v6) => {
            Ok(Adt131::C2(v6))
        }
        Adt131::C3(_) => {
            f313(v0, v1, v2, v3)
        }
    }
}

pub fn f248(v0: List<Adt6>, v1: Adt6, v2: List<Adt101>, v3: u64, v4: List<Adt103>) -> Adt120 {
    let m971 = v0;
    match m971.uncons() {
        None => {
            let a972 = v3;
            let a973 = 1u64;
            if nat_eq(a972, a973) {
                Adt120::C0(Adt102::C0, v4)
            } else {
                Adt120::C0(Adt102::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt120::C0(Adt102::C0, List::cons(Adt103::C0(v5, v1.clone(), v2.clone()), List::cons(Adt103::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f249(v0: Option<Adt6>, v1: Adt6, v2: List<Adt101>, v3: List<Adt103>) -> Adt120 {
    let m974 = v0;
    match m974 {
        None => {
            Adt120::C0(Adt102::C0, v3)
        }
        Some(v4) => {
            Adt120::C0(Adt102::C0, List::cons(Adt103::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f250(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m975 = f222(Str::lit("anyOf"), v0);
    match m975 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m976 = f314(v3);
            match m976 {
                None => {
                    Adt131::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m977 = v4;
                    match m977.uncons() {
                        None => {
                            Adt131::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt131::C3(List::cons(Adt103::C0(v5, v1.clone(), v2.clone()), List::cons(Adt103::C1(v6, v1, v2), List::<Adt103>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f251(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m978 = f222(Str::lit("oneOf"), v0);
    match m978 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m979 = f314(v3);
            match m979 {
                None => {
                    Adt131::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m980 = v4;
                    match m980.uncons() {
                        None => {
                            Adt131::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt131::C3(List::cons(Adt103::C0(v5, v1.clone(), v2.clone()), List::cons(Adt103::C2(v6, v1, v2, 0u64), List::<Adt103>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f252(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m981 = f222(Str::lit("not"), v0);
    match m981 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            Adt131::C3(List::cons(Adt103::C0(v3, v1, v2.clone()), List::cons(Adt103::C3(v2), List::<Adt103>::nil())))
        }
    }
}

pub fn f253(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m982 = f222(Str::lit("if"), v0.clone());
    match m982 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            Adt131::C3(List::cons(Adt103::C0(v3, v1.clone(), v2.clone()), List::cons(Adt103::C4(f315(Str::lit("then"), v0.clone()), f315(Str::lit("else"), v0), v1, v2), List::<Adt103>::nil())))
        }
    }
}

pub fn f254(v0: List<Adt6>, v1: Adt6, v2: List<Adt101>, v3: List<Adt103>) -> Adt120 {
    let m983 = v0;
    match m983.uncons() {
        None => {
            Adt120::C0(Adt102::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt120::C0(Adt102::C0, List::cons(Adt103::C0(v4, v1.clone(), v2.clone()), List::cons(Adt103::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f255(v0: Adt101) -> R<Str> {
    let m984 = v0;
    match m984 {
        Adt101::C0(v1) => {
            f316(v1)
        }
        Adt101::C1(v2) => {
            let a985 = f80(v2, 0i64)?;
            Ok(format_i64(a985))
        }
    }
}

pub fn f256(v0: Str) -> R<bool> {
    let m987 = f279(f85({
        let a986 = v0;
        utf8_encode(a986)
    })?)?;
    match m987 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f257(v0: Adt37) -> bool {
    let m989 = match v0 {
        Adt37::C0(_, _, h988) => {
            h988
        }
    };
    match m989 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f258(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m991 = match v0.clone() {
        Adt49::C0(_, h990) => {
            h990
        }
    };
    match m991.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h992, _) => {
                    h992
                }
            })))
        }
        Some((_, _)) => {
            if f317(match v0.clone() {
                Adt49::C0(_, h993) => {
                    h993
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f259(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m997 = {
        let m996 = match v1.clone() {
            Adt49::C0(h995, _) => {
                h995
            }
        };
        match m996 {
            Adt37::C0(_, h994, _) => {
                h994
            }
        }
    };
    match m997 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1000 = f114(Adt6::C5(match v0.clone() {
                Adt5::C0(h998) => {
                    h998
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h999) => {
                    h999
                }
            }), v2)?;
            match m1000 {
                Adt100::C0 => {
                    f258(v1)
                }
                Adt100::C1(_, _) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt100::C2(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt100::C3(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt100::C4 => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f260(v0: u64, v1: Adt6, v2: List<Adt105>, v3: List<u64>) -> R<Adt132> {
    let m1001 = v1;
    match m1001 {
        Adt6::C0 => {
            Ok(Adt132::C1(v2, f159(f318(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt132::C1(v2, f159(if v4 {
                f318(0u64)
            } else {
                f318(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f319(v5.clone())? {
                Ok(Adt132::C1(v2, f159(f320(v5)?, v3)))
            } else {
                Ok(Adt132::C0(Adt93::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt132::C1(v2, f159(f261(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1002 = v0;
            let a1003 = f321();
            if nat_lt(a1002, a1003) {
                Ok(Adt132::C1(f322(91u64, Adt105::C2({
                    let a1004 = v0;
                    let a1005 = 1u64;
                    nat_add(a1004, a1005)?
                }, v7), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1006 = v0;
            let a1007 = f321();
            if nat_lt(a1006, a1007) {
                Ok(Adt132::C1(f322(123u64, Adt105::C4({
                    let a1008 = v0;
                    let a1009 = 1u64;
                    nat_add(a1008, a1009)?
                }, v8), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
    }
}

pub fn f261(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1011 = f159(f323(f85({
            let a1010 = v0;
            utf8_encode(a1010)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1012 = List::cons(34u64, List::<u64>::nil());
        append_list(a1011, a1012)
    }))
}

pub fn f262(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1013 = v0;
    match m1013 {
        Adt32::C0(v1) => {
            Ok(Some(f324(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f325(v2)?))
        }
        Adt32::C2(v3) => {
            f326(f327(v3)?)
        }
        Adt32::C3(v4) => {
            f326(f327(match v4 {
                Adt49::C0(h1014, _) => {
                    h1014
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f328(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f329(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f330(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f331(v8)?))
        }
        Adt32::C8(v9) => {
            f326(f332(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f333(v10)?))
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(v12) => {
            f326(f334(v12)?)
        }
        Adt32::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f263(v0: Adt122) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f335(f336(match v0.clone() {
        Adt122::C0(h1015, _, _) => {
            h1015
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f337({
        let m1018 = match v0 {
            Adt122::C0(_, h1017, _) => {
                h1017
            }
        };
        match m1018 {
            Adt123::C0(h1016, _) => {
                h1016
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f264(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f338(v0, match v1.clone() {
        Adt88::C0(h1019, _, _, _) => {
            h1019
        }
    }, v2.clone()) {
        f339(match v1.clone() {
            Adt88::C0(h1020, _, _, _) => {
                h1020
            }
        }, {
            let a1022 = v2;
            let a1023 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1021) => {
                    h1021
                }
            }, List::<List<u64>>::nil());
            append_list(a1022, a1023)
        })
    } else {
        v1
    }
}

pub fn f265(v0: Adt124) -> R<Option<List<u64>>> {
    let m1024 = f340(f341(v0))?;
    match m1024 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1025 = f87(v1)?;
            match m1025 {
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

pub fn f266() -> u64 {
    404u64
}

pub fn f267(v0: Adt84) -> u64 {
    let m1026 = v0;
    match m1026 {
        Adt84::C0(_) => {
            f94()
        }
        Adt84::C1(_) => {
            f94()
        }
        Adt84::C2(_) => {
            f94()
        }
        Adt84::C3(_) => {
            f94()
        }
        Adt84::C4(_) => {
            f94()
        }
        Adt84::C5(_) => {
            f94()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f342()
        }
        Adt84::C8(_) => {
            f94()
        }
        Adt84::C9(_) => {
            f21()
        }
        Adt84::C10 => {
            f94()
        }
        Adt84::C11 => {
            f94()
        }
        Adt84::C12(_) => {
            f21()
        }
        Adt84::C13 => {
            f21()
        }
        Adt84::C14 => {
            f21()
        }
    }
}

pub fn f268(v0: Adt125) -> i64 {
    let m1027 = v0;
    match m1027 {
        Adt125::C0(_) => {
            f343()
        }
        Adt125::C1(_, _) => {
            f344()
        }
        Adt125::C2(_) => {
            f345()
        }
        Adt125::C3(_) => {
            f346()
        }
        Adt125::C4(v6) => {
            f347(v6)
        }
    }
}

pub fn f269(v0: Adt125) -> Str {
    let m1028 = v0;
    match m1028 {
        Adt125::C0(v1) => {
            f348(v1)
        }
        Adt125::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt125::C2(v4) => {
            f349(v4)
        }
        Adt125::C3(_) => {
            Str::lit("Method not found")
        }
        Adt125::C4(v6) => {
            let m1029 = v6;
            match m1029 {
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
                Adt84::C10 => {
                    Str::lit("Invalid params: requestState failed verification")
                }
                Adt84::C11 => {
                    Str::lit("Invalid params: inputResponses is not a valid InputResponses object")
                }
                Adt84::C12(_) => {
                    Str::lit("Internal error")
                }
                Adt84::C13 => {
                    Str::lit("Internal error")
                }
                Adt84::C14 => {
                    Str::lit("Internal error")
                }
            }
        }
    }
}

pub fn f270(v0: Adt125) -> R<Option<Adt6>> {
    let m1030 = v0;
    match m1030 {
        Adt125::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f350(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt125::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C4(v6) => {
            let m1031 = v6;
            match m1031 {
                Adt84::C0(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C1(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C2(v9) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(v9)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C3(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C4(v11) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("reason"), Adt6::C3(v11)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C5(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C6(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C7(v14) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f80(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f351(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C10 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C11 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C12(_) => {
                    Ok(None::<Adt6>)
                }
                Adt84::C13 => {
                    Ok(None::<Adt6>)
                }
                Adt84::C14 => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f271(v0: List<u64>) -> R<bool> {
    let m1032 = f207(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1032.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f352(v2)? {
                Ok(false)
            } else {
                f137(f139(v1), f353())
            }
        }
    }
}

pub fn f272(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt122, Adt125>> {
    f354(v0.clone(), v1, v2, v3.clone(), f355(match v0 {
        Adt0::C0(_, _, h1033, _, _, _, _) => {
            h1033
        }
    }, match v3 {
        Adt133::C0(_, h1034, _, _) => {
            h1034
        }
    }))
}

pub fn f273(v0: Adt136, v1: Adt137) -> Result<Adt133, Adt125> {
    let m1035 = v1;
    match m1035 {
        Adt137::C0(v3) => {
            f356(v3, v0)
        }
        Adt137::C1(v2) => {
            Err::<Adt133, Adt125>(v2)
        }
    }
}

pub fn f274(v0: Str) -> Adt136 {
    let a1036 = v0.clone();
    let a1037 = Str::lit("server/discover");
    if equal(a1036, a1037) {
        Adt136::C1(Adt134::C0)
    } else {
        let a1038 = v0.clone();
        let a1039 = Str::lit("tools/list");
        if equal(a1038, a1039) {
            Adt136::C1(Adt134::C1)
        } else {
            let a1040 = v0.clone();
            let a1041 = Str::lit("tools/call");
            if equal(a1040, a1041) {
                Adt136::C1(Adt134::C2)
            } else {
                let a1042 = v0.clone();
                let a1043 = Str::lit("resources/list");
                if equal(a1042, a1043) {
                    Adt136::C1(Adt134::C3)
                } else {
                    let a1044 = v0.clone();
                    let a1045 = Str::lit("resources/read");
                    if equal(a1044, a1045) {
                        Adt136::C1(Adt134::C4)
                    } else {
                        let a1046 = v0.clone();
                        let a1047 = Str::lit("resources/templates/list");
                        if equal(a1046, a1047) {
                            Adt136::C1(Adt134::C5)
                        } else {
                            let a1048 = v0.clone();
                            let a1049 = Str::lit("prompts/list");
                            if equal(a1048, a1049) {
                                Adt136::C1(Adt134::C6)
                            } else {
                                let a1050 = v0.clone();
                                let a1051 = Str::lit("prompts/get");
                                if equal(a1050, a1051) {
                                    Adt136::C1(Adt134::C7)
                                } else {
                                    let a1052 = v0.clone();
                                    let a1053 = Str::lit("completion/complete");
                                    if equal(a1052, a1053) {
                                        Adt136::C1(Adt134::C8)
                                    } else {
                                        let a1054 = v0.clone();
                                        let a1055 = Str::lit("tasks/get");
                                        if equal(a1054, a1055) {
                                            Adt136::C1(Adt134::C9)
                                        } else {
                                            let a1056 = v0.clone();
                                            let a1057 = Str::lit("tasks/update");
                                            if equal(a1056, a1057) {
                                                Adt136::C1(Adt134::C10)
                                            } else {
                                                let a1058 = v0;
                                                let a1059 = Str::lit("tasks/cancel");
                                                if equal(a1058, a1059) {
                                                    Adt136::C1(Adt134::C11)
                                                } else {
                                                    Adt136::C0
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f275(v0: Option<Adt106>, v1: List<Str>, v2: Adt98) -> R<Adt137> {
    f357(v0, v1, v2.clone(), f358(match v2 {
        Adt98::C0(_, _, h1060) => {
            h1060
        }
    }))
}

pub fn f276(v0: List<u64>) -> Option<Str> {
    let a1061 = f359(f360(), v0);
    utf8_decode(a1061)
}

pub fn f277(v0: Str) -> Adt139 {
    let m1062 = f274(v0);
    match m1062 {
        Adt136::C0 => {
            Adt139::C0
        }
        Adt136::C1(v1) => {
            f361(v1)
        }
    }
}

pub fn f278(v0: List<u64>) -> Option<List<u64>> {
    let m1063 = f362(f363(), v0);
    match m1063 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1064 = f362(f364(), f147(v1, List::<u64>::nil()));
            match m1064 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f147(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f279(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1066 = f365({
        let a1065 = v0.clone();
        length_list(a1065)
    }, v0, List::<u64>::nil())?;
    match m1066 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f159(v1, List::<u64>::nil())))
        }
    }
}

pub fn f280(p1069: List<Adt109>, p1070: List<Adt109>) -> List<Adt109> {
    let m1068 = (p1069, p1070);
    {
        let mut m1068 = m1068;
        loop {
            let step = {
                let (v0, v1) = m1068;
                let m1067 = v0;
                match m1067.uncons() {
                    None => {
                        Err::<(List<Adt109>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt109>, List<Adt109>), List<Adt109>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: List<Adt86>, p1077: List<Adt12>, v2: Option<Str>, p1078: List<Adt109>) -> R<List<Adt109>> {
    let m1076 = (p1077, p1078);
    {
        let mut m1076 = m1076;
        loop {
            let step = {
                let (v1, v3) = m1076;
                let m1071 = v1;
                match m1071.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f366(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>((v5, List::cons(Adt109::C0(match v4.clone() {
                                Adt12::C0(_, h1072, _, _) => {
                                    h1072
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1073, _) => {
                                    h1073
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1074) => {
                                    h1074
                                }
                            }, f367(v0.clone(), match v4 {
                                Adt12::C0(_, h1075, _, _) => {
                                    h1075
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1076 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(p1094: u64, p1095: i64, p1096: i64) -> R<Option<i64>> {
    let m1093 = (p1094, (p1095, p1096));
    {
        let mut m1093 = m1093;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1093;
                let m1079 = v0;
                if m1079 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1079 - 1;
                    if f368(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1082 = {
                            let a1080 = v2;
                            let a1081 = 0i64;
                            compare(a1080, a1081)
                        };
                        if match m1082 {
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
                            if f368({
                                let a1083 = v1;
                                let a1084 = 10i64;
                                let a1085 = 0i64;
                                int_rem(a1083, a1084, a1085)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1086 = v1;
                                    let a1087 = 10i64;
                                    let a1088 = 0i64;
                                    int_quot(a1086, a1087, a1088)?
                                }, f113(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1089 = v1;
                                let a1090 = 10i64;
                                int_mul(a1089, a1090)?
                            }, {
                                let a1091 = v2;
                                let a1092 = 1i64;
                                int_sub(a1091, a1092)?
                            })))
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

pub fn f283(v0: i64) -> Option<i64> {
    let m1099 = {
        let a1097 = 9007199254740991i64;
        let a1098 = v0;
        compare(a1097, a1098)
    };
    if if match m1099 {
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
        let m1102 = {
            let a1100 = v0;
            let a1101 = -9007199254740991i64;
            compare(a1100, a1101)
        };
        match m1102 {
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

pub fn f284(v0: Str, v1: Str) -> Adt140 {
    let a1103 = v0;
    let a1104 = v1;
    if equal(a1103, a1104) {
        Adt140::C0
    } else {
        Adt140::C1
    }
}

pub fn f285(v0: Adt129) -> bool {
    let m1105 = v0;
    match m1105 {
        Adt129::C0 => {
            false
        }
        Adt129::C1(_) => {
            true
        }
    }
}

pub fn f286(v0: Adt92, v1: Str, v2: Adt5) -> Result<Adt98, Adt94> {
    let m1106 = f172(Str::lit("params"), v2);
    match m1106 {
        Adt129::C0 => {
            Ok::<Adt98, Adt94>(Adt98::C0(v0, v1, None::<Adt5>))
        }
        Adt129::C1(v3) => {
            let m1107 = f245(v3);
            match m1107 {
                None => {
                    f133(Some(v0), Adt95::C6)
                }
                Some(v4) => {
                    Ok::<Adt98, Adt94>(Adt98::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f287(v0: Adt117) -> bool {
    let m1108 = v0;
    match m1108 {
        Adt117::C0 => {
            false
        }
        Adt117::C1 => {
            true
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
    }
}

pub fn f288(v0: Adt113, v1: Adt118, v2: Adt114, v3: u64) -> R<Adt130> {
    let a1110 = match v0.clone() {
        Adt113::C0(_, _, h1109) => {
            h1109
        }
    };
    let a1111 = f321();
    if nat_lt(a1110, a1111) {
        Ok(Adt130::C1(Adt113::C0(v2, List::cons(v1, match v0.clone() {
            Adt113::C0(_, h1112, _) => {
                h1112
            }
        }), {
            let a1114 = match v0 {
                Adt113::C0(_, _, h1113) => {
                    h1113
                }
            };
            let a1115 = 1u64;
            nat_add(a1114, a1115)?
        })))
    } else {
        Ok(f235(Adt93::C9(v3)))
    }
}

pub fn f289(v0: bool, v1: Adt117, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt114 {
    Adt114::C8(Adt116::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f290(v0: u64) -> i64 {
    let a1116 = v0;
    let a1117 = 0u64;
    if nat_eq(a1116, a1117) {
        0i64
    } else {
        let a1118 = v0;
        let a1119 = 1u64;
        if nat_eq(a1118, a1119) {
            1i64
        } else {
            let a1120 = v0;
            let a1121 = 2u64;
            if nat_eq(a1120, a1121) {
                2i64
            } else {
                let a1122 = v0;
                let a1123 = 3u64;
                if nat_eq(a1122, a1123) {
                    3i64
                } else {
                    let a1124 = v0;
                    let a1125 = 4u64;
                    if nat_eq(a1124, a1125) {
                        4i64
                    } else {
                        let a1126 = v0;
                        let a1127 = 5u64;
                        if nat_eq(a1126, a1127) {
                            5i64
                        } else {
                            let a1128 = v0;
                            let a1129 = 6u64;
                            if nat_eq(a1128, a1129) {
                                6i64
                            } else {
                                let a1130 = v0;
                                let a1131 = 7u64;
                                if nat_eq(a1130, a1131) {
                                    7i64
                                } else {
                                    let a1132 = v0;
                                    let a1133 = 8u64;
                                    if nat_eq(a1132, a1133) {
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

pub fn f291(p1136: List<Adt6>, p1137: List<Adt6>) -> List<Adt6> {
    let m1135 = (p1136, p1137);
    {
        let mut m1135 = m1135;
        loop {
            let step = {
                let (v0, v1) = m1135;
                let m1134 = v0;
                match m1134.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1135 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f292(v0: Adt113, v1: u64) -> Adt130 {
    let m1139 = match v0.clone() {
        Adt113::C0(_, h1138, _) => {
            h1138
        }
    };
    match m1139.uncons() {
        None => {
            f235(Adt93::C2(v1))
        }
        Some((v2, _)) => {
            let m1140 = v2;
            match m1140 {
                Adt118::C0(_) => {
                    f234(v0, Adt114::C0)
                }
                Adt118::C1(_, _) => {
                    f234(v0, Adt114::C4)
                }
            }
        }
    }
}

pub fn f293(p1143: List<(Str, Adt6)>, p1144: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1142 = (p1143, p1144);
    {
        let mut m1142 = m1142;
        loop {
            let step = {
                let (v0, v1) = m1142;
                let m1141 = v0;
                match m1141.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1142 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: Adt113, v1: Adt114) -> Adt113 {
    Adt113::C0(v1, match v0.clone() {
        Adt113::C0(_, h1145, _) => {
            h1145
        }
    }, match v0 {
        Adt113::C0(_, _, h1146) => {
            h1146
        }
    })
}

pub fn f295(v0: Adt113, v1: u64, v2: List<u64>, v3: bool) -> Adt130 {
    let m1148 = {
        let a1147 = f359(f360(), f159(v2, List::<u64>::nil()));
        utf8_decode(a1147)
    };
    match m1148 {
        None => {
            f235(Adt93::C14)
        }
        Some(v4) => {
            if v3 {
                f369(v0, v1, v4)
            } else {
                f301(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f296(v0: u64) -> u64 {
    let a1149 = v0;
    let a1150 = 34u64;
    if nat_eq(a1149, a1150) {
        34u64
    } else {
        let a1151 = v0;
        let a1152 = 92u64;
        if nat_eq(a1151, a1152) {
            92u64
        } else {
            let a1153 = v0;
            let a1154 = 47u64;
            if nat_eq(a1153, a1154) {
                47u64
            } else {
                let a1155 = v0;
                let a1156 = 98u64;
                if nat_eq(a1155, a1156) {
                    8u64
                } else {
                    let a1157 = v0;
                    let a1158 = 102u64;
                    if nat_eq(a1157, a1158) {
                        12u64
                    } else {
                        let a1159 = v0;
                        let a1160 = 110u64;
                        if nat_eq(a1159, a1160) {
                            10u64
                        } else {
                            let a1161 = v0;
                            let a1162 = 114u64;
                            if nat_eq(a1161, a1162) {
                                13u64
                            } else {
                                let a1163 = v0;
                                let a1164 = 116u64;
                                if nat_eq(a1163, a1164) {
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

pub fn f297(v0: u64) -> u64 {
    let a1165 = v0;
    let a1166 = 48u64;
    if nat_lt(a1165, a1166) {
        16u64
    } else {
        let a1167 = v0;
        let a1168 = 58u64;
        if nat_lt(a1167, a1168) {
            let a1169 = v0;
            let a1170 = 48u64;
            nat_sub(a1169, a1170)
        } else {
            let a1171 = v0;
            let a1172 = 65u64;
            if nat_lt(a1171, a1172) {
                16u64
            } else {
                let a1173 = v0;
                let a1174 = 71u64;
                if nat_lt(a1173, a1174) {
                    let a1175 = v0;
                    let a1176 = 55u64;
                    nat_sub(a1175, a1176)
                } else {
                    let a1177 = v0;
                    let a1178 = 97u64;
                    if nat_lt(a1177, a1178) {
                        16u64
                    } else {
                        let a1179 = v0;
                        let a1180 = 103u64;
                        if nat_lt(a1179, a1180) {
                            let a1181 = v0;
                            let a1182 = 87u64;
                            nat_sub(a1181, a1182)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f298(v0: Adt113, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt130> {
    let a1183 = v3;
    let a1184 = 0u64;
    if nat_eq(a1183, a1184) {
        if f226(v2, 55296u64, 56319u64) {
            Ok(f234(v0, Adt114::C6(Adt115::C3(v2), v4, v5)))
        } else {
            if f226(v2, 56320u64, 57343u64) {
                Ok(f235(Adt93::C6(v1)))
            } else {
                Ok(f234(v0, Adt114::C6(Adt115::C0, f370(v2, v4)?, v5)))
            }
        }
    } else {
        if f226(v2, 56320u64, 57343u64) {
            Ok(f234(v0, Adt114::C6(Adt115::C0, f370({
                let a1193 = 65536u64;
                let a1194 = {
                    let a1191 = {
                        let a1187 = {
                            let a1185 = v3;
                            let a1186 = 55296u64;
                            nat_sub(a1185, a1186)
                        };
                        let a1188 = 1024u64;
                        nat_mul(a1187, a1188)?
                    };
                    let a1192 = {
                        let a1189 = v2;
                        let a1190 = 56320u64;
                        nat_sub(a1189, a1190)
                    };
                    nat_add(a1191, a1192)?
                };
                nat_add(a1193, a1194)?
            }, v4)?, v5)))
        } else {
            Ok(f235(Adt93::C6(v1)))
        }
    }
}

pub fn f299(v0: u64, v1: u64) -> u64 {
    let m1197 = {
        let a1195 = f300(v0);
        let a1196 = v1;
        index_list(a1195, a1196)
    };
    match m1197 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f300(v0: u64) -> List<u64> {
    let a1198 = v0;
    let a1199 = 0u64;
    if nat_eq(a1198, a1199) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1200 = v0;
        let a1201 = 1u64;
        if nat_eq(a1200, a1201) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f301(v0: Adt113, v1: Adt6) -> Adt130 {
    Adt130::C1(f228(v1, match v0.clone() {
        Adt113::C0(_, h1202, _) => {
            h1202
        }
    }, match v0 {
        Adt113::C0(_, _, h1203) => {
            h1203
        }
    }))
}

pub fn f302(v0: u64) -> Adt6 {
    let a1204 = v0;
    let a1205 = 0u64;
    if nat_eq(a1204, a1205) {
        Adt6::C1(true)
    } else {
        let a1206 = v0;
        let a1207 = 1u64;
        if nat_eq(a1206, a1207) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f303(v0: Adt113, v1: Adt116, v2: Adt117) -> Adt130 {
    f234(v0, Adt114::C8(Adt116::C0(match v1.clone() {
        Adt116::C0(h1208, _, _, _, _, _, _) => {
            h1208
        }
    }, v2, match v1.clone() {
        Adt116::C0(_, _, h1209, _, _, _, _) => {
            h1209
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, h1210, _, _, _) => {
            h1210
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, _, h1211, _, _) => {
            h1211
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, _, _, h1212, _) => {
            h1212
        }
    }, match v1 {
        Adt116::C0(_, _, _, _, _, _, h1213) => {
            h1213
        }
    })))
}

pub fn f304(v0: Adt113, v1: u64, v2: u64, v3: Adt116, v4: bool) -> R<Adt130> {
    let v5: u64 = {
        let a1215 = match v3.clone() {
            Adt116::C0(_, _, _, h1214, _, _, _) => {
                h1214
            }
        };
        let a1216 = 0u64;
        if if nat_eq(a1215, a1216) {
            let a1217 = v2;
            let a1218 = 48u64;
            nat_eq(a1217, a1218)
        } else {
            false
        } {
            0u64
        } else {
            let a1220 = match v3.clone() {
                Adt116::C0(_, _, _, h1219, _, _, _) => {
                    h1219
                }
            };
            let a1221 = 1u64;
            nat_add(a1220, a1221)?
        }
    };
    let v6: i64 = if v4 {
        f113(match v3.clone() {
            Adt116::C0(_, _, _, _, h1222, _, _) => {
                h1222
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt116::C0(_, _, _, _, h1223, _, _) => {
                h1223
            }
        }
    };
    let a1224 = f371();
    let a1225 = v5;
    if nat_lt(a1224, a1225) {
        Ok(f235(Adt93::C8(v1)))
    } else {
        let m1228 = {
            let a1226 = 99999i64;
            let a1227 = v6;
            compare(a1226, a1227)
        };
        if match m1228 {
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
            Ok(f235(Adt93::C8(v1)))
        } else {
            Ok(f234(v0, f289(match v3.clone() {
                Adt116::C0(h1229, _, _, _, _, _, _) => {
                    h1229
                }
            }, if v4 {
                Adt117::C4
            } else {
                Adt117::C2
            }, f113({
                let a1231 = match v3.clone() {
                    Adt116::C0(_, _, h1230, _, _, _, _) => {
                        h1230
                    }
                };
                let a1232 = 10i64;
                int_mul(a1231, a1232)?
            }, f290({
                let a1233 = v2;
                let a1234 = 48u64;
                nat_sub(a1233, a1234)
            }))?, v5, v6, match v3.clone() {
                Adt116::C0(_, _, _, _, _, h1235, _) => {
                    h1235
                }
            }, match v3 {
                Adt116::C0(_, _, _, _, _, _, h1236) => {
                    h1236
                }
            })))
        }
    }
}

pub fn f305(v0: u64) -> bool {
    let a1237 = v0;
    let a1238 = 101u64;
    if nat_eq(a1237, a1238) {
        true
    } else {
        let a1239 = v0;
        let a1240 = 69u64;
        nat_eq(a1239, a1240)
    }
}

pub fn f306(v0: u64) -> bool {
    f226(v0, 48u64, 57u64)
}

pub fn f307(v0: Adt113, v1: u64, v2: u64, v3: Adt116) -> R<Adt130> {
    let m1241 = f227(v3)?;
    match m1241 {
        Adt135::C0 => {
            Ok(f235(Adt93::C7(v1)))
        }
        Adt135::C1 => {
            Ok(f235(Adt93::C8(v1)))
        }
        Adt135::C2(v4) => {
            Ok(f232(f228(v4, match v0.clone() {
                Adt113::C0(_, h1242, _) => {
                    h1242
                }
            }, match v0 {
                Adt113::C0(_, _, h1243) => {
                    h1243
                }
            }), v1, v2))
        }
    }
}

pub fn f308(v0: Adt113, v1: u64, v2: u64, v3: Adt116, v4: bool) -> R<Adt130> {
    let v5: i64 = f113({
        let a1245 = match v3.clone() {
            Adt116::C0(_, _, _, _, _, _, h1244) => {
                h1244
            }
        };
        let a1246 = 10i64;
        int_mul(a1245, a1246)?
    }, f290({
        let a1247 = v2;
        let a1248 = 48u64;
        nat_sub(a1247, a1248)
    }))?;
    let m1251 = {
        let a1249 = 99999i64;
        let a1250 = v5;
        compare(a1249, a1250)
    };
    if match m1251 {
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
        Ok(f235(Adt93::C8(v1)))
    } else {
        Ok(f234(v0, f289(match v3.clone() {
            Adt116::C0(h1252, _, _, _, _, _, _) => {
                h1252
            }
        }, Adt117::C7, match v3.clone() {
            Adt116::C0(_, _, h1253, _, _, _, _) => {
                h1253
            }
        }, match v3.clone() {
            Adt116::C0(_, _, _, h1254, _, _, _) => {
                h1254
            }
        }, match v3 {
            Adt116::C0(_, _, _, _, h1255, _, _) => {
                h1255
            }
        }, v4, v5)))
    }
}

pub fn f309(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1256 = f372(f373(), v2.clone());
    match m1256 {
        None => {
            let m1257 = f374(f375(), v2.clone());
            match m1257 {
                None => {
                    f376(v0, v1, v2)
                }
                Some(v4) => {
                    Some(Adt91::C5(v0, v4))
                }
            }
        }
        Some(v3) => {
            Some(Adt91::C4(v0, v3))
        }
    }
}

pub fn f310(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt119) -> R<Adt141> {
    let m1258 = f222(Str::lit("x-mcp-header"), v1.clone());
    match m1258 {
        Adt129::C0 => {
            Ok(Adt141::C1(v4))
        }
        Adt129::C1(v5) => {
            f377(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f311(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt99>) -> List<Adt99> {
    f378(Str::lit("properties"), v0.clone(), v1, v2, f378(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f379(Str::lit("items"), v0.clone(), f379(Str::lit("additionalProperties"), v0.clone(), f380(Str::lit("allOf"), v0.clone(), f380(Str::lit("anyOf"), v0.clone(), f380(Str::lit("oneOf"), v0.clone(), f379(Str::lit("not"), v0.clone(), f379(Str::lit("if"), v0.clone(), f379(Str::lit("then"), v0.clone(), f379(Str::lit("else"), v0, v3)))))))))))
}

pub fn f312(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt131> {
    Ok(f381(List::cons((Str::lit("type"), f382(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f383(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f384(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f385(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f385(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f385(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f385(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1259 = f386(v0.clone(), v1.clone())?;
        let a1260 = List::cons((Str::lit("required"), f387(v0, v1)), List::<(Str, Adt142)>::nil());
        append_list(a1259, a1260)
    })))))))))
}

pub fn f313(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    Ok(f388(f389(v0, v1.clone(), v2.clone(), v3.clone()), f388(f390(v1.clone(), v2.clone(), v3.clone()), f388(f391(v1.clone(), v2.clone(), v3.clone()), f388(f392(v1.clone(), v2.clone(), v3.clone())?, f388(f393(v1.clone(), v2.clone(), v3.clone()), f388(f394(Adt104::C0, f250(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f388(f394(Adt104::C1, f251(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f388(f394(Adt104::C2, f252(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f394(Adt104::C3, f253(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f314(v0: Adt6) -> Option<List<Adt6>> {
    let m1261 = v0;
    match m1261 {
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

pub fn f315(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1262 = f222(v0, v1);
    match m1262 {
        Adt129::C0 => {
            None::<Adt6>
        }
        Adt129::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f316(v0: Str) -> R<Str> {
    let m1265 = {
        let a1264 = f359(f360(), f159(f395(f85({
            let a1263 = v0.clone();
            utf8_encode(a1263)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1264)
    };
    match m1265 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f317(v0: List<Adt50>) -> R<bool> {
    let m1266 = v0;
    match m1266.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f396(v2, match v1 {
                Adt50::C0(h1267, _, _) => {
                    h1267
                }
            })
        }
    }
}

pub fn f318(v0: u64) -> List<u64> {
    f300(v0)
}

pub fn f319(v0: Adt7) -> R<bool> {
    let a1285 = {
        let m1271 = {
            let a1269 = f397();
            let a1270 = match v0.clone() {
                Adt7::C0(h1268, _) => {
                    h1268
                }
            };
            compare(a1269, a1270)
        };
        if match m1271 {
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
            let m1276 = {
                let a1274 = match v0.clone() {
                    Adt7::C0(h1272, _) => {
                        h1272
                    }
                };
                let a1275 = {
                    let a1273 = f397();
                    int_neg(a1273)?
                };
                compare(a1274, a1275)
            };
            if match m1276 {
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
                let m1280 = {
                    let a1278 = 9999i64;
                    let a1279 = match v0.clone() {
                        Adt7::C0(_, h1277) => {
                            h1277
                        }
                    };
                    compare(a1278, a1279)
                };
                if match m1280 {
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
                    let m1284 = {
                        let a1282 = match v0 {
                            Adt7::C0(_, h1281) => {
                                h1281
                            }
                        };
                        let a1283 = -9999i64;
                        compare(a1282, a1283)
                    };
                    match m1284 {
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
    Ok(bool_not(a1285))
}

pub fn f320(v0: Adt7) -> R<List<u64>> {
    let m1289 = {
        let a1287 = match v0.clone() {
            Adt7::C0(_, h1286) => {
                h1286
            }
        };
        let a1288 = 0i64;
        compare(a1287, a1288)
    };
    if if match m1289 {
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
        let m1293 = {
            let a1291 = 0i64;
            let a1292 = match v0.clone() {
                Adt7::C0(_, h1290) => {
                    h1290
                }
            };
            compare(a1291, a1292)
        };
        match m1293 {
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
        let a1296 = f398(match v0.clone() {
            Adt7::C0(h1294, _) => {
                h1294
            }
        })?;
        let a1297 = List::cons(101u64, f398(match v0 {
            Adt7::C0(_, h1295) => {
                h1295
            }
        })?);
        Ok(append_list(a1296, a1297))
    } else {
        f398(match v0 {
            Adt7::C0(h1298, _) => {
                h1298
            }
        })
    }
}

pub fn f321() -> u64 {
    512u64
}

pub fn f322(v0: u64, v1: Adt105, v2: List<Adt105>) -> List<Adt105> {
    List::cons(Adt105::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f323(p1301: List<u64>, p1302: List<u64>) -> R<List<u64>> {
    let m1300 = (p1301, p1302);
    {
        let mut m1300 = m1300;
        loop {
            let step = {
                let (v0, v1) = m1300;
                let m1299 = v0;
                match m1299.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f159(f399(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f400(match v0.clone() {
        Adt33::C0(_, h1303, _, _) => {
            h1303
        }
    })), List::cons((Str::lit("capabilities"), f401(match v0.clone() {
        Adt33::C0(_, _, h1304, _) => {
            h1304
        }
    })), {
        let a1307 = f402(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1305) => {
                h1305
            }
        });
        let a1308 = f403(match v0 {
            Adt33::C0(h1306, _, _, _) => {
                h1306
            }
        })?;
        append_list(a1307, a1308)
    })))
}

pub fn f325(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f404(match v0.clone() {
        Adt34::C0(_, _, h1309) => {
            h1309
        }
    }))), {
        let a1312 = f402(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1310, _) => {
                h1310
            }
        });
        let a1313 = f403(match v0 {
            Adt34::C0(h1311, _, _) => {
                h1311
            }
        })?;
        append_list(a1312, a1313)
    }))
}

pub fn f326(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1314 = v0;
    match m1314 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1315 = f340(Adt6::C5(v1))?;
            match m1315 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1316 = f245(v2);
                    match m1316 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1317) => {
                                    h1317
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f327(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1319 = f405(match v0.clone() {
        Adt37::C0(h1318, _, _) => {
            h1318
        }
    })?;
    match m1319 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1322 = f406(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1320, _) => {
                        h1320
                    }
                });
                let a1323 = f407(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1321) => {
                        h1321
                    }
                });
                append_list(a1322, a1323)
            })))
        }
    }
}

pub fn f328(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f408(match v0.clone() {
        Adt51::C0(_, _, h1324) => {
            h1324
        }
    })?)), {
        let a1327 = f402(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1325, _) => {
                h1325
            }
        });
        let a1328 = f403(match v0 {
            Adt51::C0(h1326, _, _) => {
                h1326
            }
        })?;
        append_list(a1327, a1328)
    }))
}

pub fn f329(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f409(match v0.clone() {
        Adt52::C0(_, h1329) => {
            h1329
        }
    }))), f403(match v0 {
        Adt52::C0(h1330, _) => {
            h1330
        }
    })?))
}

pub fn f330(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f410(match v0.clone() {
        Adt53::C0(_, _, h1331) => {
            h1331
        }
    }))), {
        let a1334 = f402(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1332, _) => {
                h1332
            }
        });
        let a1335 = f403(match v0 {
            Adt53::C0(h1333, _, _) => {
                h1333
            }
        })?;
        append_list(a1334, a1335)
    }))
}

pub fn f331(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f411(match v0.clone() {
        Adt55::C0(_, _, h1336) => {
            h1336
        }
    }))), {
        let a1339 = f402(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1337, _) => {
                h1337
            }
        });
        let a1340 = f403(match v0 {
            Adt55::C0(h1338, _, _) => {
                h1338
            }
        })?;
        append_list(a1339, a1340)
    }))
}

pub fn f332(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1342 = f412(match v0.clone() {
        Adt58::C0(_, h1341) => {
            h1341
        }
    })?;
    match m1342 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1344 = f402(Str::lit("description"), match v0 {
                    Adt58::C0(h1343, _) => {
                        h1343
                    }
                });
                let a1345 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1344, a1345)
            }))
        }
    }
}

pub fn f333(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f350({
        let m1348 = match v0.clone() {
            Adt60::C0(h1347) => {
                h1347
            }
        };
        match m1348 {
            Adt61::C0(h1346, _, _) => {
                h1346
            }
        }
    }))), {
        let a1355 = f413(Str::lit("total"), {
            let m1351 = match v0.clone() {
                Adt60::C0(h1350) => {
                    h1350
                }
            };
            match m1351 {
                Adt61::C0(_, h1349, _) => {
                    h1349
                }
            }
        })?;
        let a1356 = f407(Str::lit("hasMore"), {
            let m1354 = match v0 {
                Adt60::C0(h1353) => {
                    h1353
                }
            };
            match m1354 {
                Adt61::C0(_, _, h1352) => {
                    h1352
                }
            }
        });
        append_list(a1355, a1356)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f334(v0: Adt81) -> R<Option<List<(Str, Adt6)>>> {
    let m1357 = f414(f415(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1357 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1358 = f416(v1);
                let a1359 = f417(f418(v0));
                append_list(a1358, a1359)
            }))
        }
    }
}

pub fn f335(v0: Adt143) -> Str {
    let m1360 = v0;
    match m1360 {
        Adt143::C0 => {
            Str::lit("complete")
        }
        Adt143::C1 => {
            Str::lit("input_required")
        }
        Adt143::C2 => {
            Str::lit("task")
        }
    }
}

pub fn f336(v0: Adt32) -> Adt143 {
    let m1361 = v0;
    match m1361 {
        Adt32::C0(_) => {
            Adt143::C0
        }
        Adt32::C1(_) => {
            Adt143::C0
        }
        Adt32::C2(_) => {
            Adt143::C0
        }
        Adt32::C3(_) => {
            Adt143::C0
        }
        Adt32::C4(_) => {
            Adt143::C0
        }
        Adt32::C5(_) => {
            Adt143::C0
        }
        Adt32::C6(_) => {
            Adt143::C0
        }
        Adt32::C7(_) => {
            Adt143::C0
        }
        Adt32::C8(_) => {
            Adt143::C0
        }
        Adt32::C9(_) => {
            Adt143::C0
        }
        Adt32::C10(_) => {
            Adt143::C0
        }
        Adt32::C11 => {
            Adt143::C0
        }
        Adt32::C12(_) => {
            Adt143::C1
        }
        Adt32::C13(_) => {
            Adt143::C2
        }
    }
}

pub fn f337(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1362 = v0;
    match m1362 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f419(), f420(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f338(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f421(v2) {
            let a1363 = v1;
            let a1364 = 200u64;
            nat_eq(a1363, a1364)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f339(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f422(), List::<List<u64>>::nil(), f423(v1))
}

pub fn f340(v0: Adt6) -> R<Option<Adt6>> {
    f424(f425(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f341(v0: Adt124) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f124())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f426(match v0.clone() {
        Adt124::C0(h1365, _) => {
            h1365
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1368 = match v0.clone() {
            Adt124::C0(_, h1367) => {
                h1367
            }
        };
        match m1368 {
            Adt50::C0(h1366, _, _) => {
                h1366
            }
        }
    })), {
        let a1375 = f427({
            let m1371 = match v0.clone() {
                Adt124::C0(_, h1370) => {
                    h1370
                }
            };
            match m1371 {
                Adt50::C0(_, h1369, _) => {
                    h1369
                }
            }
        });
        let a1376 = f428({
            let m1374 = match v0 {
                Adt124::C0(_, h1373) => {
                    h1373
                }
            };
            match m1374 {
                Adt50::C0(_, _, h1372) => {
                    h1372
                }
            }
        });
        append_list(a1375, a1376)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f342() -> u64 {
    429u64
}

pub fn f343() -> i64 {
    -32602i64
}

pub fn f344() -> i64 {
    -32022i64
}

pub fn f345() -> i64 {
    -32020i64
}

pub fn f346() -> i64 {
    -32601i64
}

pub fn f347(v0: Adt84) -> i64 {
    let m1377 = v0;
    match m1377 {
        Adt84::C0(_) => {
            f343()
        }
        Adt84::C1(_) => {
            f343()
        }
        Adt84::C2(_) => {
            f343()
        }
        Adt84::C3(_) => {
            f343()
        }
        Adt84::C4(_) => {
            f343()
        }
        Adt84::C5(_) => {
            f343()
        }
        Adt84::C6(_) => {
            f429()
        }
        Adt84::C7(_) => {
            f430()
        }
        Adt84::C8(_) => {
            f431()
        }
        Adt84::C9(_) => {
            f429()
        }
        Adt84::C10 => {
            f343()
        }
        Adt84::C11 => {
            f343()
        }
        Adt84::C12(_) => {
            f429()
        }
        Adt84::C13 => {
            f429()
        }
        Adt84::C14 => {
            f429()
        }
    }
}

pub fn f348(v0: Adt126) -> Str {
    let m1378 = v0;
    match m1378 {
        Adt126::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt126::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt126::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt126::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt126::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt126::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt126::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt126::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt126::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f349(v0: Adt107) -> Str {
    let m1379 = v0;
    match m1379 {
        Adt107::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt107::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt107::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt107::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt107::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt107::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt107::C6(v1) => {
            let m1380 = v1;
            match m1380 {
                Adt108::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt108::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt108::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt107::C7(v2) => {
            let m1381 = v2;
            match m1381 {
                Adt108::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt108::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt108::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt107::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt107::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt107::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f350(v0: List<Str>) -> List<Adt6> {
    f291(f432(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f351(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1393 = f433(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1382, _, _, _, _) => {
                h1382
            }
        });
        let a1394 = {
            let a1391 = f433(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1383, _, _, _) => {
                    h1383
                }
            });
            let a1392 = {
                let a1389 = f433(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1384, _, _) => {
                        h1384
                    }
                });
                let a1390 = {
                    let a1387 = f433(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1385, _) => {
                            h1385
                        }
                    });
                    let a1388 = f433(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1386) => {
                            h1386
                        }
                    });
                    append_list(a1387, a1388)
                };
                append_list(a1389, a1390)
            };
            append_list(a1391, a1392)
        };
        append_list(a1393, a1394)
    })
}

pub fn f352(p1397: List<List<u64>>) -> R<bool> {
    let m1396 = p1397;
    {
        let mut m1396 = m1396;
        loop {
            let step = {
                let v0 = m1396;
                let m1395 = v0;
                match m1395.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f434(f139(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f354(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt122, Adt125>> {
    if v4 {
        f435(v0, v1, v2, f436(match v3.clone() {
            Adt133::C0(_, h1398, _, _) => {
                h1398
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1399, _) => {
                h1399
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1400) => {
                h1400
            }
        }))
    } else {
        Ok(Err::<Adt122, Adt125>(Adt125::C3(f437(match v3 {
            Adt133::C0(_, h1401, _, _) => {
                h1401
            }
        }))))
    }
}

pub fn f355(v0: Adt4, v1: Adt134) -> bool {
    let m1402 = v1;
    match m1402 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f438(match v0 {
                Adt4::C0(_, _, _, _, _, h1403, _) => {
                    h1403
                }
            })
        }
        Adt134::C2 => {
            f438(match v0 {
                Adt4::C0(_, _, _, _, _, h1404, _) => {
                    h1404
                }
            })
        }
        Adt134::C3 => {
            f439(match v0 {
                Adt4::C0(_, _, _, _, h1405, _, _) => {
                    h1405
                }
            })
        }
        Adt134::C4 => {
            f439(match v0 {
                Adt4::C0(_, _, _, _, h1406, _, _) => {
                    h1406
                }
            })
        }
        Adt134::C5 => {
            f439(match v0 {
                Adt4::C0(_, _, _, _, h1407, _, _) => {
                    h1407
                }
            })
        }
        Adt134::C6 => {
            f438(match v0 {
                Adt4::C0(_, _, _, h1408, _, _, _) => {
                    h1408
                }
            })
        }
        Adt134::C7 => {
            f438(match v0 {
                Adt4::C0(_, _, _, h1409, _, _, _) => {
                    h1409
                }
            })
        }
        Adt134::C8 => {
            f440(match v0 {
                Adt4::C0(_, _, h1410, _, _, _, _) => {
                    h1410
                }
            })
        }
        Adt134::C9 => {
            f441(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1411) => {
                    h1411
                }
            }, f442())
        }
        Adt134::C10 => {
            f441(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1412) => {
                    h1412
                }
            }, f442())
        }
        Adt134::C11 => {
            f441(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1413) => {
                    h1413
                }
            }, f442())
        }
    }
}

pub fn f356(v0: Adt138, v1: Adt136) -> Result<Adt133, Adt125> {
    let m1414 = v1;
    match m1414 {
        Adt136::C0 => {
            Err::<Adt133, Adt125>(Adt125::C3(match v0 {
                Adt138::C0(_, h1415, _, _) => {
                    h1415
                }
            }))
        }
        Adt136::C1(v2) => {
            Ok::<Adt133, Adt125>(Adt133::C0(match v0.clone() {
                Adt138::C0(h1416, _, _, _) => {
                    h1416
                }
            }, v2, match v0.clone() {
                Adt138::C0(_, _, h1417, _) => {
                    h1417
                }
            }, match v0 {
                Adt138::C0(_, _, _, h1418) => {
                    h1418
                }
            }))
        }
    }
}

pub fn f357(v0: Option<Adt106>, v1: List<Str>, v2: Adt98, v3: Result<Adt16, Adt126>) -> R<Adt137> {
    let m1419 = v3;
    match m1419 {
        Ok(v5) => {
            Ok(f443(v1, v2.clone(), v5.clone(), f444(v0, f277(match v2.clone() {
                Adt98::C0(_, h1420, _) => {
                    h1420
                }
            }), match v2.clone() {
                Adt98::C0(_, h1421, _) => {
                    h1421
                }
            }, match v5 {
                Adt16::C0(h1422, _, _, _, _, _) => {
                    h1422
                }
            }, f445(match v2 {
                Adt98::C0(_, _, h1423) => {
                    h1423
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt137::C1(Adt125::C0(v4)))
        }
    }
}

pub fn f358(v0: Option<Adt5>) -> Result<Adt16, Adt126> {
    let m1424 = v0;
    match m1424 {
        None => {
            Err::<Adt16, Adt126>(Adt126::C0)
        }
        Some(v1) => {
            let m1425 = f172(Str::lit("_meta"), v1);
            match m1425 {
                Adt129::C0 => {
                    Err::<Adt16, Adt126>(Adt126::C1)
                }
                Adt129::C1(v2) => {
                    let m1426 = f245(v2);
                    match m1426 {
                        None => {
                            Err::<Adt16, Adt126>(Adt126::C4)
                        }
                        Some(v3) => {
                            f446(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f359(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1427 = v0;
    let a1428 = f447(f360(), f448(List::<Bytes>::nil(), v1));
    append_bytes(a1427, a1428)
}

pub fn f360() -> Bytes {
    Bytes::lit(&[])
}

pub fn f361(v0: Adt134) -> Adt139 {
    let m1429 = v0;
    match m1429 {
        Adt134::C0 => {
            Adt139::C0
        }
        Adt134::C1 => {
            Adt139::C0
        }
        Adt134::C2 => {
            Adt139::C1
        }
        Adt134::C3 => {
            Adt139::C0
        }
        Adt134::C4 => {
            Adt139::C2
        }
        Adt134::C5 => {
            Adt139::C0
        }
        Adt134::C6 => {
            Adt139::C0
        }
        Adt134::C7 => {
            Adt139::C1
        }
        Adt134::C8 => {
            Adt139::C0
        }
        Adt134::C9 => {
            Adt139::C3
        }
        Adt134::C10 => {
            Adt139::C3
        }
        Adt134::C11 => {
            Adt139::C3
        }
    }
}

pub fn f362(p1435: List<u64>, p1436: List<u64>) -> Option<List<u64>> {
    let m1434 = (p1435, p1436);
    {
        let mut m1434 = m1434;
        loop {
            let step = {
                let (v0, v1) = m1434;
                let m1430 = v0;
                match m1430.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1431 = v1;
                        match m1431.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1432 = v4;
                                let a1433 = v2;
                                if nat_eq(a1432, a1433) {
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
                Ok(next) => { m1434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f364() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f365(p1445: u64, p1446: List<u64>, p1447: List<u64>) -> R<Option<List<u64>>> {
    let m1444 = (p1445, (p1446, p1447));
    {
        let mut m1444 = m1444;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1444;
                let m1437 = v0;
                if m1437 == 0 {
                    let m1438 = v1;
                    match m1438.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1437 - 1;
                    let m1439 = v1;
                    match m1439.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1440 = v7;
                            match m1440.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1441 = v9;
                                    match m1441.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1442 = v11;
                                            match m1442.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1443 = f449(v6, v8, v10, v12, f450(v13.clone()))?;
                                                    match m1443 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f159(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f366(v0: Adt12, v1: Option<Str>) -> bool {
    let m1448 = v1;
    match m1448 {
        None => {
            false
        }
        Some(v2) => {
            let a1450 = match v0 {
                Adt12::C0(h1449, _, _, _) => {
                    h1449
                }
            };
            let a1451 = v2;
            equal(a1450, a1451)
        }
    }
}

pub fn f367(v0: List<Adt86>, v1: Str) -> R<Adt110> {
    let m1454 = f214(f71(v0, {
        let a1452 = f451();
        let a1453 = f452(v1)?;
        append_list(a1452, a1453)
    })?)?;
    match m1454 {
        Adt96::C0 => {
            Ok(Adt110::C0)
        }
        Adt96::C1 => {
            Ok(Adt110::C1)
        }
        Adt96::C2 => {
            Ok(Adt110::C1)
        }
        Adt96::C3(v2) => {
            let m1455 = f276(v2);
            match m1455 {
                None => {
                    Ok(Adt110::C1)
                }
                Some(v3) => {
                    Ok(Adt110::C2(v3))
                }
            }
        }
    }
}

pub fn f368(v0: i64) -> bool {
    let a1462 = {
        let m1458 = {
            let a1456 = v0;
            let a1457 = 0i64;
            compare(a1456, a1457)
        };
        if match m1458 {
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
            let m1461 = {
                let a1459 = 0i64;
                let a1460 = v0;
                compare(a1459, a1460)
            };
            match m1461 {
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
    bool_not(a1462)
}

pub fn f369(v0: Adt113, v1: u64, v2: Str) -> Adt130 {
    let m1464 = match v0.clone() {
        Adt113::C0(_, h1463, _) => {
            h1463
        }
    };
    match m1464.uncons() {
        None => {
            f235(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1465 = v3;
            match m1465 {
                Adt118::C0(_) => {
                    f235(Adt93::C2(v1))
                }
                Adt118::C1(v6, _) => {
                    Adt130::C1(Adt113::C0(Adt114::C5, List::cons(Adt118::C1(v6, Some(v2)), v4), match v0 {
                        Adt113::C0(_, _, h1466) => {
                            h1466
                        }
                    }))
                }
            }
        }
    }
}

pub fn f370(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1467 = v0;
    let a1468 = 128u64;
    if nat_lt(a1467, a1468) {
        Ok(List::cons(v0, v1))
    } else {
        let a1469 = v0;
        let a1470 = 2048u64;
        if nat_lt(a1469, a1470) {
            Ok(List::cons({
                let a1471 = 128u64;
                let a1472 = f453(v0, 64u64);
                nat_add(a1471, a1472)?
            }, List::cons({
                let a1473 = 192u64;
                let a1474 = f454(v0, 64u64);
                nat_add(a1473, a1474)?
            }, v1)))
        } else {
            let a1475 = v0;
            let a1476 = 65536u64;
            if nat_lt(a1475, a1476) {
                Ok(List::cons({
                    let a1477 = 128u64;
                    let a1478 = f453(v0, 64u64);
                    nat_add(a1477, a1478)?
                }, List::cons({
                    let a1479 = 128u64;
                    let a1480 = f453(f454(v0, 64u64), 64u64);
                    nat_add(a1479, a1480)?
                }, List::cons({
                    let a1481 = 224u64;
                    let a1482 = f454(v0, 4096u64);
                    nat_add(a1481, a1482)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1483 = 128u64;
                    let a1484 = f453(v0, 64u64);
                    nat_add(a1483, a1484)?
                }, List::cons({
                    let a1485 = 128u64;
                    let a1486 = f453(f454(v0, 64u64), 64u64);
                    nat_add(a1485, a1486)?
                }, List::cons({
                    let a1487 = 128u64;
                    let a1488 = f453(f454(v0, 4096u64), 64u64);
                    nat_add(a1487, a1488)?
                }, List::cons({
                    let a1489 = 240u64;
                    let a1490 = f454(v0, 262144u64);
                    nat_add(a1489, a1490)?
                }, v1)))))
            }
        }
    }
}

pub fn f371() -> u64 {
    18u64
}

pub fn f372(p1494: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1493 = p1494;
    {
        let mut m1493 = m1493;
        loop {
            let step = {
                let v0 = m1493;
                let m1491 = v0;
                match m1491.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1492 = f222(v2.clone(), v1.clone());
                        match m1492 {
                            Adt129::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt129::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1493 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f374(p1501: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1500 = p1501;
    {
        let mut m1500 = m1500;
        loop {
            let step = {
                let v0 = m1500;
                let m1495 = v0;
                match m1495.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1497 = f222({
                            let (h1496, _) = v2.clone();
                            h1496
                        }, v1.clone());
                        match m1497 {
                            Adt129::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt129::C1(v4) => {
                                if f455({
                                    let (_, h1498) = v2;
                                    h1498
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1499, _) = v2;
                                        h1499
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1500 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f375() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f376(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1502 = f222(Str::lit("$ref"), v2);
    match m1502 {
        Adt129::C0 => {
            None::<Adt91>
        }
        Adt129::C1(v3) => {
            let m1503 = f219(v3);
            match m1503 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1504 = f456(v1, v4.clone());
                    match m1504 {
                        None => {
                            Some(Adt91::C6(v0, v4))
                        }
                        Some(_) => {
                            None::<Adt91>
                        }
                    }
                }
            }
        }
    }
}

pub fn f377(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt119) -> R<Adt141> {
    if v3 {
        let m1505 = f219(v2);
        match m1505 {
            None => {
                Ok(Adt141::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f457(v0, v1, v6.clone(), f111(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt141::C0(Adt91::C10(v0)))
    }
}

pub fn f378(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt99>) -> List<Adt99> {
    let m1506 = f222(v0.clone(), v1);
    match m1506 {
        Adt129::C0 => {
            v4
        }
        Adt129::C1(v5) => {
            let m1507 = f245(v5);
            match m1507 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1508 = v0;
                    let a1509 = Str::lit("properties");
                    if equal(a1508, a1509) {
                        f458(match v6 {
                            Adt5::C0(h1510) => {
                                h1510
                            }
                        }, v2, v3, v4)
                    } else {
                        f459(match v6 {
                            Adt5::C0(h1511) => {
                                h1511
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f379(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt99>) -> List<Adt99> {
    let m1512 = f222(v0, v1);
    match m1512 {
        Adt129::C0 => {
            v2
        }
        Adt129::C1(v3) => {
            List::cons(Adt99::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f380(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt99>) -> List<Adt99> {
    let m1513 = f222(v0, v1);
    match m1513 {
        Adt129::C0 => {
            v2
        }
        Adt129::C1(v3) => {
            let m1514 = f314(v3);
            match m1514 {
                None => {
                    v2
                }
                Some(v4) => {
                    f460(v4, v2)
                }
            }
        }
    }
}

pub fn f381(p1521: List<(Str, Adt142)>) -> Adt131 {
    let m1520 = p1521;
    {
        let mut m1520 = m1520;
        loop {
            let step = {
                let v0 = m1520;
                let m1515 = v0;
                match m1515.uncons() {
                    None => {
                        Err::<List<(Str, Adt142)>, Adt131>(Adt131::C3(List::<Adt103>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1517 = {
                            let (_, h1516) = v1;
                            h1516
                        };
                        match m1517 {
                            Adt142::C0 => {
                                Ok::<List<(Str, Adt142)>, Adt131>(v2)
                            }
                            Adt142::C1 => {
                                Err::<List<(Str, Adt142)>, Adt131>(Adt131::C0({
                                    let (h1518, _) = v1;
                                    h1518
                                }))
                            }
                            Adt142::C2 => {
                                Err::<List<(Str, Adt142)>, Adt131>(Adt131::C1({
                                    let (h1519, _) = v1;
                                    h1519
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1520 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f382(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1522 = f222(Str::lit("type"), v0);
    match m1522 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1523 = f219(v2.clone());
            match m1523 {
                None => {
                    let m1524 = f314(v2);
                    match m1524 {
                        None => {
                            Ok(Adt142::C2)
                        }
                        Some(v4) => {
                            f463(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f461(v3.clone()) {
                        if f462(v3, v1)? {
                            Ok(Adt142::C0)
                        } else {
                            Ok(Adt142::C1)
                        }
                    } else {
                        Ok(Adt142::C2)
                    }
                }
            }
        }
    }
}

pub fn f383(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1525 = f222(Str::lit("enum"), v0);
    match m1525 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1526 = f314(v2);
            match m1526 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v3) => {
                    f464(v3, v1)
                }
            }
        }
    }
}

pub fn f384(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1527 = f222(Str::lit("const"), v0);
    match m1527 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1528 = f465(v2, v1)?;
            match m1528 {
                Adt146::C0 => {
                    Ok(Adt142::C0)
                }
                Adt146::C1 => {
                    Ok(Adt142::C1)
                }
                Adt146::C2 => {
                    Ok(Adt142::C2)
                }
            }
        }
    }
}

pub fn f385(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt142> {
    let m1529 = f222(v0, v2);
    match m1529 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v4) => {
            let m1530 = f466(v4);
            match m1530 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v5) => {
                    let m1531 = f466(v3);
                    match m1531 {
                        None => {
                            Ok(Adt142::C0)
                        }
                        Some(v6) => {
                            if f467(v1, f468(v6, v5)?) {
                                Ok(Adt142::C0)
                            } else {
                                Ok(Adt142::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f386(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt142)>> {
    let m1532 = v1;
    match m1532 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt142)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt142)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt142)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f469(Str::lit("minLength"), true, v0.clone(), f470(v4.clone())?)?), List::cons((Str::lit("maxLength"), f469(Str::lit("maxLength"), false, v0, f470(v4)?)?), List::<(Str, Adt142)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f469(Str::lit("minItems"), true, v0.clone(), f471(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f469(Str::lit("maxItems"), false, v0, f471(v5, 0u64)?)?), List::<(Str, Adt142)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f469(Str::lit("minProperties"), true, v0.clone(), f472(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f469(Str::lit("maxProperties"), false, v0, f472(v6, 0u64)?)?), List::<(Str, Adt142)>::nil())))
        }
    }
}

pub fn f387(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt142 {
    let m1533 = f222(Str::lit("required"), v0);
    match m1533 {
        Adt129::C0 => {
            Adt142::C0
        }
        Adt129::C1(v2) => {
            let m1534 = f314(v2);
            match m1534 {
                None => {
                    Adt142::C2
                }
                Some(v3) => {
                    let m1535 = f245(v1);
                    match m1535 {
                        None => {
                            Adt142::C0
                        }
                        Some(v4) => {
                            f473(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f388(v0: Adt131, v1: Adt131) -> Adt131 {
    let m1536 = v0.clone();
    match m1536 {
        Adt131::C0(_) => {
            v0
        }
        Adt131::C1(_) => {
            v0
        }
        Adt131::C2(_) => {
            v0
        }
        Adt131::C3(v5) => {
            let m1537 = v1.clone();
            match m1537 {
                Adt131::C0(_) => {
                    v1
                }
                Adt131::C1(_) => {
                    v1
                }
                Adt131::C2(_) => {
                    v1
                }
                Adt131::C3(v9) => {
                    Adt131::C3({
                        let a1538 = v5;
                        let a1539 = v9;
                        append_list(a1538, a1539)
                    })
                }
            }
        }
    }
}

pub fn f389(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> Adt131 {
    let m1540 = f222(Str::lit("$ref"), v1);
    match m1540 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v4) => {
            let m1541 = f219(v4);
            match m1541 {
                None => {
                    Adt131::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1542 = f456(v0, v5.clone());
                    match m1542 {
                        None => {
                            Adt131::C2(v5)
                        }
                        Some(v6) => {
                            Adt131::C3(List::cons(Adt103::C0(v6, v2, v3), List::<Adt103>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f390(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1543 = f222(Str::lit("properties"), v0);
    match m1543 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1544 = f245(v3);
            match m1544 {
                None => {
                    Adt131::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1545 = f245(v1);
                    match m1545 {
                        None => {
                            Adt131::C3(List::<Adt103>::nil())
                        }
                        Some(v5) => {
                            Adt131::C3(f474(match v4 {
                                Adt5::C0(h1546) => {
                                    h1546
                                }
                            }, match v5 {
                                Adt5::C0(h1547) => {
                                    h1547
                                }
                            }, v2, List::<Adt103>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f391(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1548 = f222(Str::lit("additionalProperties"), v0.clone());
    match m1548 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1549 = f245(v1);
            match m1549 {
                None => {
                    Adt131::C3(List::<Adt103>::nil())
                }
                Some(v4) => {
                    let m1550 = f244(v3.clone());
                    match m1550 {
                        None => {
                            Adt131::C3(f477(match v4 {
                                Adt5::C0(h1552) => {
                                    h1552
                                }
                            }, f476(v0), v3, v2, List::<Adt103>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt131::C3(List::<Adt103>::nil())
                            } else {
                                if f475(match v4 {
                                    Adt5::C0(h1551) => {
                                        h1551
                                    }
                                }, f476(v0)) {
                                    Adt131::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt131::C3(List::<Adt103>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f392(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> R<Adt131> {
    let m1553 = f222(Str::lit("items"), v0);
    match m1553 {
        Adt129::C0 => {
            Ok(Adt131::C3(List::<Adt103>::nil()))
        }
        Adt129::C1(v3) => {
            let m1554 = f314(v1);
            match m1554 {
                None => {
                    Ok(Adt131::C3(List::<Adt103>::nil()))
                }
                Some(v4) => {
                    Ok(Adt131::C3(f478(v4, 0u64, v3, v2, List::<Adt103>::nil())?))
                }
            }
        }
    }
}

pub fn f393(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1555 = f222(Str::lit("allOf"), v0);
    match m1555 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1556 = f314(v3);
            match m1556 {
                None => {
                    Adt131::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1557 = v4.clone();
                    match m1557.uncons() {
                        None => {
                            Adt131::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt131::C3(f479(v4, v1, v2, List::<Adt103>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f394(v0: Adt104, v1: Adt131, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt101>) -> Adt131 {
    let m1558 = v1.clone();
    match m1558 {
        Adt131::C0(_) => {
            v1
        }
        Adt131::C1(_) => {
            v1
        }
        Adt131::C2(_) => {
            v1
        }
        Adt131::C3(v8) => {
            let m1559 = v8;
            match m1559.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt131::C3(List::cons(Adt103::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt103>::nil()))
                }
            }
        }
    }
}

pub fn f395(p1566: List<u64>, p1567: List<u64>) -> List<u64> {
    let m1565 = (p1566, p1567);
    {
        let mut m1565 = m1565;
        loop {
            let step = {
                let (v0, v1) = m1565;
                let m1560 = v0;
                match m1560.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1561 = v2;
                        let a1562 = 126u64;
                        if nat_eq(a1561, a1562) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1563 = v2;
                            let a1564 = 47u64;
                            if nat_eq(a1563, a1564) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f396(p1572: List<Adt50>, p1573: Adt7) -> R<bool> {
    let m1571 = (p1572, p1573);
    {
        let mut m1571 = m1571;
        loop {
            let step = {
                let (v0, v1) = m1571;
                let m1568 = v0;
                match m1568.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f480(f468(match v2.clone() {
                            Adt50::C0(h1569, _, _) => {
                                h1569
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1570, _, _) => {
                                    h1570
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1571 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397() -> i64 {
    999999999999999999i64
}

pub fn f398(v0: i64) -> R<List<u64>> {
    let m1576 = {
        let a1574 = v0;
        let a1575 = 0i64;
        compare(a1574, a1575)
    };
    if match m1576 {
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
        Ok(List::cons(45u64, f481(20u64, {
            let a1577 = v0;
            int_neg(a1577)?
        }, List::<u64>::nil())?))
    } else {
        f481(20u64, v0, List::<u64>::nil())
    }
}

pub fn f399(v0: u64) -> R<List<u64>> {
    let a1578 = v0;
    let a1579 = 34u64;
    if nat_eq(a1578, a1579) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1580 = v0;
        let a1581 = 92u64;
        if nat_eq(a1580, a1581) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1582 = v0;
            let a1583 = 8u64;
            if nat_eq(a1582, a1583) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1584 = v0;
                let a1585 = 9u64;
                if nat_eq(a1584, a1585) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1586 = v0;
                    let a1587 = 10u64;
                    if nat_eq(a1586, a1587) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1588 = v0;
                        let a1589 = 12u64;
                        if nat_eq(a1588, a1589) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1590 = v0;
                            let a1591 = 13u64;
                            if nat_eq(a1590, a1591) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1592 = v0;
                                let a1593 = 32u64;
                                if nat_lt(a1592, a1593) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f482(f454(v0, 16u64))?, List::cons(f482(f453(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f400(v0: List<Str>) -> Adt6 {
    Adt6::C4(f350(v0))
}

pub fn f401(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1611 = f433(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1594, _, _, _, _, _, _) => {
                h1594
            }
        });
        let a1612 = {
            let a1609 = f433(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1595, _, _, _, _, _) => {
                    h1595
                }
            });
            let a1610 = {
                let a1607 = f433(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1596, _, _, _, _) => {
                        h1596
                    }
                });
                let a1608 = {
                    let a1605 = f483(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1597, _, _, _) => {
                            h1597
                        }
                    });
                    let a1606 = {
                        let a1603 = f484(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1598, _, _) => {
                                h1598
                            }
                        });
                        let a1604 = {
                            let a1601 = f483(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1599, _) => {
                                    h1599
                                }
                            });
                            let a1602 = f433(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1600) => {
                                    h1600
                                }
                            });
                            append_list(a1601, a1602)
                        };
                        append_list(a1603, a1604)
                    };
                    append_list(a1605, a1606)
                };
                append_list(a1607, a1608)
            };
            append_list(a1609, a1610)
        };
        append_list(a1611, a1612)
    })
}

pub fn f402(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1613 = v1;
    match m1613 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f403(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f485(match v0.clone() {
        Adt10::C0(h1614, _) => {
            h1614
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f486(match v0 {
        Adt10::C0(_, h1615) => {
            h1615
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f404(v0: List<Adt35>) -> List<Adt6> {
    f291(f487(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f405(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1616 = f488(v0, List::<Adt6>::nil())?;
    match m1616 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f291(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f406(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1617 = v1;
    match m1617 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f407(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1618 = v1;
    match m1618 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f408(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f291(f489(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f409(v0: List<Adt46>) -> List<Adt6> {
    f291(f490(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f410(v0: List<Adt54>) -> List<Adt6> {
    f291(f491(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f411(v0: List<Adt56>) -> List<Adt6> {
    f291(f492(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f412(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1619 = f493(v0, List::<Adt6>::nil())?;
    match m1619 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f291(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f413(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1620 = v1;
    match m1620 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f485(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f414(p1626: List<(Str, Adt64)>, p1627: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m1625 = (p1626, p1627);
    {
        let mut m1625 = m1625;
        loop {
            let step = {
                let (v0, v1) = m1625;
                let m1621 = v0;
                match m1621.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1623 = f494({
                            let (_, h1622) = v2;
                            h1622
                        })?;
                        match m1623 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h1624, _) = v2;
                                    h1624
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1625 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f415(v0: Adt81) -> List<(Str, Adt64)> {
    let m1628 = v0;
    match m1628 {
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

pub fn f416(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1629 = v0.clone();
    match m1629.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f293(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f417(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1630 = v0;
    match m1630 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f418(v0: Adt81) -> Option<Str> {
    let m1631 = v0;
    match m1631 {
        Adt81::C0(_) => {
            None::<Str>
        }
        Adt81::C1(v2) => {
            Some(v2)
        }
        Adt81::C2(_, v4) => {
            Some(v4)
        }
    }
}

pub fn f419() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f420(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1632, _, _, _, _, _) => {
            h1632
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1633, _, _, _) => {
            h1633
        }
    })), {
        let a1642 = f402(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1634, _, _, _, _) => {
                h1634
            }
        });
        let a1643 = {
            let a1640 = f402(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1635, _, _) => {
                    h1635
                }
            });
            let a1641 = {
                let a1638 = f402(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1636, _) => {
                        h1636
                    }
                });
                let a1639 = f495(match v0 {
                    Adt1::C0(_, _, _, _, _, h1637) => {
                        h1637
                    }
                });
                append_list(a1638, a1639)
            };
            append_list(a1640, a1641)
        };
        append_list(a1642, a1643)
    })))
}

pub fn f421(v0: List<List<u64>>) -> bool {
    let m1644 = v0;
    match m1644.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f422() -> List<Adt86> {
    List::cons(Adt86::C0(f496(), f353()), List::cons(Adt86::C0(f497(), f498()), List::cons(Adt86::C0(f499(), f500()), List::<Adt86>::nil())))
}

pub fn f423(v0: List<List<u64>>) -> List<u64> {
    f501(f38(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f424(p1659: u64, p1660: Adt144, p1661: List<Adt145>) -> R<Option<Adt6>> {
    let m1658 = (p1659, (p1660, p1661));
    {
        let mut m1658 = m1658;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1658;
                let m1645 = v0;
                if m1645 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1645 - 1;
                    let m1646 = v1;
                    match m1646 {
                        Adt144::C0(v4) => {
                            let m1647 = v4.clone();
                            match m1647 {
                                Adt6::C0 => {
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                }
                                Adt6::C1(_) => {
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                }
                                Adt6::C2(_) => {
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                }
                                Adt6::C3(v7) => {
                                    let v8: Str = f502(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1648 = v9;
                                    match m1648.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1649 = v12;
                                    match m1649.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f502({
                                                let (h1650, _) = v13;
                                                h1650
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1651) = v13;
                                                h1651
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1652 = v2;
                            match m1652.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1653 = v17;
                                    match m1653 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1654 = v20;
                                            match m1654.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C4(f291(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v22), List::cons(Adt145::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt145::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1655 = v26;
                                            match m1655.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f293(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f502({
                                                        let (h1656, _) = v28;
                                                        h1656
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1657) = v28;
                                                        h1657
                                                    }), List::cons(Adt145::C1(v30, v27, v29), v18))))
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
                Ok(next) => { m1658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425() -> u64 {
    67108864u64
}

pub fn f426(v0: Adt19) -> Adt6 {
    let m1662 = v0;
    match m1662 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f427(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1663 = v0;
    match m1663 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f428(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1664 = v0;
    match m1664 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f429() -> i64 {
    -32603i64
}

pub fn f430() -> i64 {
    429i64
}

pub fn f431() -> i64 {
    -32021i64
}

pub fn f432(p1667: List<Str>, p1668: List<Adt6>) -> List<Adt6> {
    let m1666 = (p1667, p1668);
    {
        let mut m1666 = m1666;
        loop {
            let step = {
                let (v0, v1) = m1666;
                let m1665 = v0;
                match m1665.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1666 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1669 = v1;
    match m1669 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1670) => {
                    h1670
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f434(v0: List<u64>) -> R<bool> {
    let m1671 = v0;
    match m1671.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1672 = v2;
            match m1672.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1673 = f180(v1)?;
                    let a1674 = 113u64;
                    if if nat_eq(a1673, a1674) {
                        let a1675 = v3;
                        let a1676 = 61u64;
                        nat_eq(a1675, a1676)
                    } else {
                        false
                    } {
                        Ok(f503(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f435(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt126>) -> R<Result<Adt122, Adt125>> {
    let m1677 = v3;
    match m1677 {
        Ok(v5) => {
            f504(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt122, Adt125>(Adt125::C0(v4)))
        }
    }
}

pub fn f436(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt126> {
    let m1678 = v0;
    match m1678 {
        Adt134::C0 => {
            f505(v1, v2)
        }
        Adt134::C1 => {
            f506(v1, v2)
        }
        Adt134::C2 => {
            f507(v1, v2)
        }
        Adt134::C3 => {
            f508(v1, v2)
        }
        Adt134::C4 => {
            f509(v1, v2)
        }
        Adt134::C5 => {
            f510(v1, v2)
        }
        Adt134::C6 => {
            f511(v1, v2)
        }
        Adt134::C7 => {
            f512(v1, v2)
        }
        Adt134::C8 => {
            f513(v1, v2)
        }
        Adt134::C9 => {
            f514(v1, v2)
        }
        Adt134::C10 => {
            f515(v1, v2)
        }
        Adt134::C11 => {
            f516(v1, v2)
        }
    }
}

pub fn f437(v0: Adt134) -> Str {
    let m1679 = v0;
    match m1679 {
        Adt134::C0 => {
            Str::lit("server/discover")
        }
        Adt134::C1 => {
            Str::lit("tools/list")
        }
        Adt134::C2 => {
            Str::lit("tools/call")
        }
        Adt134::C3 => {
            Str::lit("resources/list")
        }
        Adt134::C4 => {
            Str::lit("resources/read")
        }
        Adt134::C5 => {
            Str::lit("resources/templates/list")
        }
        Adt134::C6 => {
            Str::lit("prompts/list")
        }
        Adt134::C7 => {
            Str::lit("prompts/get")
        }
        Adt134::C8 => {
            Str::lit("completion/complete")
        }
        Adt134::C9 => {
            Str::lit("tasks/get")
        }
        Adt134::C10 => {
            Str::lit("tasks/update")
        }
        Adt134::C11 => {
            Str::lit("tasks/cancel")
        }
    }
}

pub fn f438(v0: Option<Adt8>) -> bool {
    let m1680 = v0;
    match m1680 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f439(v0: Option<Adt9>) -> bool {
    let m1681 = v0;
    match m1681 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f440(v0: Option<Adt5>) -> bool {
    let m1682 = v0;
    match m1682 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f441(v0: Option<Adt5>, v1: Str) -> bool {
    let m1683 = v0;
    match m1683 {
        None => {
            false
        }
        Some(v2) => {
            f223(v1, v2)
        }
    }
}

pub fn f442() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f443(v0: List<Str>, v1: Adt98, v2: Adt16, v3: Option<Adt107>) -> Adt137 {
    let m1684 = v3;
    match m1684 {
        None => {
            f517(v1, v2.clone(), f518(match v2 {
                Adt16::C0(h1685, _, _, _, _, _) => {
                    h1685
                }
            }, v0))
        }
        Some(v4) => {
            Adt137::C1(Adt125::C2(v4))
        }
    }
}

pub fn f444(v0: Option<Adt106>, v1: Adt139, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt107>> {
    let m1686 = v0;
    match m1686 {
        None => {
            Ok(None::<Adt107>)
        }
        Some(v5) => {
            let m1688 = match v5.clone() {
                Adt106::C0(_, _, _, h1687, _) => {
                    h1687
                }
            };
            match m1688 {
                None => {
                    f519(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f445(v0: Option<Adt5>) -> Adt5 {
    let m1689 = v0;
    match m1689 {
        None => {
            f520()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f446(v0: Adt5) -> Result<Adt16, Adt126> {
    let m1690 = f521(v0.clone());
    match m1690 {
        Adt148::C0 => {
            f522(v0.clone(), f172(f523(), v0.clone()), f172(f524(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt126>(Adt126::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt126>(Adt126::C3)
        }
    }
}

pub fn f447(p1695: Bytes, p1696: List<Bytes>) -> Bytes {
    let m1694 = (p1695, p1696);
    {
        let mut m1694 = m1694;
        loop {
            let step = {
                let (v0, v1) = m1694;
                let m1691 = v1;
                match m1691.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1692 = v2;
                            let a1693 = v0;
                            append_bytes(a1692, a1693)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1694 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(p1699: List<Bytes>, p1700: List<u64>) -> List<Bytes> {
    let m1698 = (p1699, p1700);
    {
        let mut m1698 = m1698;
        loop {
            let step = {
                let (v0, v1) = m1698;
                let m1697 = v1;
                match m1697.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f525(f526(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1701 = f527(v0)?;
    match m1701 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1702 = f527(v1)?;
            match m1702 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1703 = v2;
                    let a1704 = 61u64;
                    if nat_eq(a1703, a1704) {
                        f528(v5, v6, v4, v3)
                    } else {
                        let m1705 = f527(v2)?;
                        match m1705 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1706 = v3;
                                let a1707 = 61u64;
                                if nat_eq(a1706, a1707) {
                                    f529(v5, v6, v7, v4)
                                } else {
                                    let m1708 = f527(v3)?;
                                    match m1708 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f530(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f450(v0: List<u64>) -> bool {
    let m1709 = v0;
    match m1709.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f451() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f452(v0: Str) -> R<List<u64>> {
    f85({
        let a1710 = v0;
        utf8_encode(a1710)
    })
}

pub fn f453(v0: u64, v1: u64) -> u64 {
    let a1711 = v0;
    let a1712 = v1;
    let a1713 = 0u64;
    nat_rem(a1711, a1712, a1713)
}

pub fn f454(v0: u64, v1: u64) -> u64 {
    let a1714 = v0;
    let a1715 = v1;
    let a1716 = 0u64;
    nat_quot(a1714, a1715, a1716)
}

pub fn f455(v0: u64, v1: Adt6) -> bool {
    let a1717 = v0;
    let a1718 = 0u64;
    if nat_eq(a1717, a1718) {
        f531(v1)
    } else {
        let a1719 = v0;
        let a1720 = 1u64;
        if nat_eq(a1719, a1720) {
            f532(v1)
        } else {
            let a1721 = v0;
            let a1722 = 2u64;
            if nat_eq(a1721, a1722) {
                f533(v1)
            } else {
                let a1723 = v0;
                let a1724 = 3u64;
                if nat_eq(a1723, a1724) {
                    f534(v1)
                } else {
                    let a1725 = v0;
                    let a1726 = 4u64;
                    if nat_eq(a1725, a1726) {
                        f535(v1)
                    } else {
                        let a1727 = v0;
                        let a1728 = 5u64;
                        if nat_eq(a1727, a1728) {
                            f536(v1)
                        } else {
                            let a1729 = v0;
                            let a1730 = 6u64;
                            if nat_eq(a1729, a1730) {
                                f537(v1)
                            } else {
                                let a1731 = v0;
                                let a1732 = 7u64;
                                if nat_eq(a1731, a1732) {
                                    f538(v1)
                                } else {
                                    f539(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f456(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1733 = v1.clone();
    let a1734 = Str::lit("#");
    if equal(a1733, a1734) {
        Some(v0)
    } else {
        let m1735 = f245(v0);
        match m1735 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1736 = f172(Str::lit("$defs"), v2);
                match m1736 {
                    Adt129::C0 => {
                        None::<Adt6>
                    }
                    Adt129::C1(v3) => {
                        let m1737 = f245(v3);
                        match m1737 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f540(v1, match v4 {
                                    Adt5::C0(h1738) => {
                                        h1738
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

pub fn f457(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt119) -> R<Adt141> {
    let m1739 = v3.clone();
    match m1739.uncons() {
        None => {
            Ok(Adt141::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f184(v3.clone(), true) {
                let m1740 = f541(v1);
                match m1740 {
                    None => {
                        Ok(Adt141::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f542(v3)?;
                        if f543(v9.clone(), match v5.clone() {
                            Adt119::C0(_, h1741) => {
                                h1741
                            }
                        }) {
                            Ok(Adt141::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt141::C1(Adt119::C0(List::cons(Adt12::C0(v0, v2, f544(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt119::C0(h1742, _) => {
                                    h1742
                                }
                            }), List::cons(v9, match v5 {
                                Adt119::C0(_, h1743) => {
                                    h1743
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt141::C0(Adt91::C9(v0)))
            }
        }
    }
}

pub fn f458(p1748: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1749: List<Adt99>) -> List<Adt99> {
    let m1747 = (p1748, p1749);
    {
        let mut m1747 = m1747;
        loop {
            let step = {
                let (v0, v3) = m1747;
                let m1744 = v0;
                match m1744.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>((v5, List::cons(Adt99::C0({
                            let (_, h1745) = v4;
                            h1745
                        }, v1, List::cons({
                            let (h1746, _) = v4;
                            h1746
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1747 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(p1753: List<(Str, Adt6)>, p1754: List<Adt99>) -> List<Adt99> {
    let m1752 = (p1753, p1754);
    {
        let mut m1752 = m1752;
        loop {
            let step = {
                let (v0, v1) = m1752;
                let m1750 = v0;
                match m1750.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>((v3, List::cons(Adt99::C0({
                            let (_, h1751) = v2;
                            h1751
                        }, false, List::<Str>::nil()), v1)))
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

pub fn f460(p1757: List<Adt6>, p1758: List<Adt99>) -> List<Adt99> {
    let m1756 = (p1757, p1758);
    {
        let mut m1756 = m1756;
        loop {
            let step = {
                let (v0, v1) = m1756;
                let m1755 = v0;
                match m1755.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt99>), List<Adt99>>((v3, List::cons(Adt99::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: Str) -> bool {
    let a1759 = v0.clone();
    let a1760 = Str::lit("null");
    if equal(a1759, a1760) {
        true
    } else {
        let a1761 = v0.clone();
        let a1762 = Str::lit("boolean");
        if equal(a1761, a1762) {
            true
        } else {
            let a1763 = v0.clone();
            let a1764 = Str::lit("object");
            if equal(a1763, a1764) {
                true
            } else {
                let a1765 = v0.clone();
                let a1766 = Str::lit("array");
                if equal(a1765, a1766) {
                    true
                } else {
                    let a1767 = v0.clone();
                    let a1768 = Str::lit("number");
                    if equal(a1767, a1768) {
                        true
                    } else {
                        let a1769 = v0.clone();
                        let a1770 = Str::lit("string");
                        if equal(a1769, a1770) {
                            true
                        } else {
                            let a1771 = v0;
                            let a1772 = Str::lit("integer");
                            equal(a1771, a1772)
                        }
                    }
                }
            }
        }
    }
}

pub fn f462(v0: Str, v1: Adt6) -> R<bool> {
    let a1773 = v0.clone();
    let a1774 = f545(v1.clone());
    if equal(a1773, a1774) {
        Ok(true)
    } else {
        let a1775 = v0;
        let a1776 = Str::lit("integer");
        if equal(a1775, a1776) {
            let m1777 = f466(v1);
            match m1777 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f546(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f463(p1780: List<Adt6>, v1: Adt6, p1781: bool) -> R<Adt142> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v0, v2) = m1779;
                let m1778 = v0;
                match m1778.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f461(f547(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt142>>((v4, if v2 {
                                true
                            } else {
                                f462(f547(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C2))
                        }
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

pub fn f464(p1785: List<Adt6>, v1: Adt6) -> R<Adt142> {
    let m1784 = p1785;
    {
        let mut m1784 = m1784;
        loop {
            let step = {
                let v0 = m1784;
                let m1782 = v0;
                match m1782.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt142>>(Ok(Adt142::C1))
                    }
                    Some((v2, v3)) => {
                        let m1783 = f465(v2, v1.clone())?;
                        match m1783 {
                            Adt146::C0 => {
                                Err::<List<Adt6>, R<Adt142>>(Ok(Adt142::C0))
                            }
                            Adt146::C1 => {
                                Ok::<List<Adt6>, R<Adt142>>(v3)
                            }
                            Adt146::C2 => {
                                Err::<List<Adt6>, R<Adt142>>(Ok(Adt142::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f465(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f548(f549(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f466(v0: Adt6) -> Option<Adt7> {
    let m1786 = v0;
    match m1786 {
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

pub fn f467(v0: u64, v1: Adt147) -> bool {
    let a1787 = v0;
    let a1788 = 0u64;
    if nat_eq(a1787, a1788) {
        let a1789 = f550(v1);
        bool_not(a1789)
    } else {
        let a1790 = v0;
        let a1791 = 1u64;
        if nat_eq(a1790, a1791) {
            let a1792 = f480(v1);
            bool_not(a1792)
        } else {
            let a1793 = v0;
            let a1794 = 2u64;
            if nat_eq(a1793, a1794) {
                f480(v1)
            } else {
                f550(v1)
            }
        }
    }
}

pub fn f468(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f551(v0.clone(), v1.clone(), f552(match v0 {
        Adt7::C0(h1795, _) => {
            h1795
        }
    }), f552(match v1 {
        Adt7::C0(h1796, _) => {
            h1796
        }
    }))
}

pub fn f469(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt142> {
    let m1797 = f222(v0, v2);
    match m1797 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v4) => {
            let m1798 = f553(v4);
            match m1798 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v5) => {
                    let m1801 = {
                        let a1799 = v5;
                        let a1800 = 0i64;
                        compare(a1799, a1800)
                    };
                    if match m1801 {
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
                        Ok(Adt142::C2)
                    } else {
                        if v1 {
                            if f550(f554(f80(v3, 0i64)?, v5)) {
                                Ok(Adt142::C1)
                            } else {
                                Ok(Adt142::C0)
                            }
                        } else {
                            if f480(f554(f80(v3, 0i64)?, v5)) {
                                Ok(Adt142::C1)
                            } else {
                                Ok(Adt142::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f470(v0: Str) -> R<u64> {
    f555(f85({
        let a1802 = v0;
        utf8_encode(a1802)
    })?, 0u64)
}

pub fn f471(p1807: List<Adt6>, p1808: u64) -> R<u64> {
    let m1806 = (p1807, p1808);
    {
        let mut m1806 = m1806;
        loop {
            let step = {
                let (v0, v1) = m1806;
                let m1803 = v0;
                match m1803.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1804 = v1;
                            let a1805 = 1u64;
                            nat_add(a1804, a1805)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1806 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(p1813: List<(Str, Adt6)>, p1814: u64) -> R<u64> {
    let m1812 = (p1813, p1814);
    {
        let mut m1812 = m1812;
        loop {
            let step = {
                let (v0, v1) = m1812;
                let m1809 = v0;
                match m1809.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1810 = v1;
                            let a1811 = 1u64;
                            nat_add(a1810, a1811)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(p1818: List<Adt6>, v1: Adt5) -> Adt142 {
    let m1817 = p1818;
    {
        let mut m1817 = m1817;
        loop {
            let step = {
                let v0 = m1817;
                let m1815 = v0;
                match m1815.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt142>(Adt142::C0)
                    }
                    Some((v2, v3)) => {
                        let m1816 = f219(v2);
                        match m1816 {
                            None => {
                                Err::<List<Adt6>, Adt142>(Adt142::C2)
                            }
                            Some(v4) => {
                                if f223(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt142>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt142>(Adt142::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1817 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(p1825: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt101>, p1826: List<Adt103>) -> List<Adt103> {
    let m1824 = (p1825, p1826);
    {
        let mut m1824 = m1824;
        loop {
            let step = {
                let (v0, v3) = m1824;
                let m1819 = v0;
                match m1819.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1821 = f222({
                            let (h1820, _) = v4.clone();
                            h1820
                        }, v1.clone());
                        match m1821 {
                            Adt129::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v5, v3))
                            }
                            Adt129::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v5, List::cons(Adt103::C0({
                                    let (_, h1822) = v4;
                                    h1822
                                }, v6, List::cons(Adt101::C0({
                                    let (h1823, _) = v4;
                                    h1823
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(p1831: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1830 = p1831;
    {
        let mut m1830 = m1830;
        loop {
            let step = {
                let v0 = m1830;
                let m1827 = v0;
                match m1827.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1829 = f222({
                            let (h1828, _) = v2;
                            h1828
                        }, v1.clone());
                        match m1829 {
                            Adt129::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt129::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1832 = f222(Str::lit("properties"), v0);
    match m1832 {
        Adt129::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt129::C1(v1) => {
            let m1833 = f245(v1);
            match m1833 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1834) => {
                            h1834
                        }
                    }
                }
            }
        }
    }
}

pub fn f477(p1841: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>, p1842: List<Adt103>) -> List<Adt103> {
    let m1840 = (p1841, p1842);
    {
        let mut m1840 = m1840;
        loop {
            let step = {
                let (v0, v4) = m1840;
                let m1835 = v0;
                match m1835.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1837 = f222({
                            let (h1836, _) = v5.clone();
                            h1836
                        }, v1.clone());
                        match m1837 {
                            Adt129::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v6, List::cons(Adt103::C0(v2.clone(), {
                                    let (_, h1838) = v5;
                                    h1838
                                }, List::cons(Adt101::C0({
                                    let (h1839, _) = v5;
                                    h1839
                                }), v3.clone())), v4)))
                            }
                            Adt129::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1840 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(p1847: List<Adt6>, p1848: u64, v2: Adt6, v3: List<Adt101>, p1849: List<Adt103>) -> R<List<Adt103>> {
    let m1846 = (p1847, (p1848, p1849));
    {
        let mut m1846 = m1846;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1846;
                let m1843 = v0;
                match m1843.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt103>)), R<List<Adt103>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt103>)), R<List<Adt103>>>((v6, ({
                            let a1844 = v1;
                            let a1845 = 1u64;
                            nat_add(a1844, a1845)?
                        }, List::cons(Adt103::C0(v2.clone(), v5, List::cons(Adt101::C1(v1), v3.clone())), v4))))
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

pub fn f479(p1852: List<Adt6>, v1: Adt6, v2: List<Adt101>, p1853: List<Adt103>) -> List<Adt103> {
    let m1851 = (p1852, p1853);
    {
        let mut m1851 = m1851;
        loop {
            let step = {
                let (v0, v3) = m1851;
                let m1850 = v0;
                match m1850.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt103>), List<Adt103>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt103>), List<Adt103>>((v5, List::cons(Adt103::C0(v4, v1.clone(), v2.clone()), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1851 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: Adt147) -> bool {
    let m1854 = v0;
    match m1854 {
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

pub fn f481(p1866: u64, p1867: i64, p1868: List<u64>) -> R<List<u64>> {
    let m1865 = (p1866, (p1867, p1868));
    {
        let mut m1865 = m1865;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1865;
                let m1855 = v0;
                if m1855 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1855 - 1;
                    let v4: i64 = {
                        let a1856 = v1;
                        let a1857 = 10i64;
                        let a1858 = 0i64;
                        int_quot(a1856, a1857, a1858)?
                    };
                    let v5: i64 = {
                        let a1859 = v1;
                        let a1860 = 10i64;
                        let a1861 = 0i64;
                        int_rem(a1859, a1860, a1861)
                    };
                    let m1864 = {
                        let a1862 = v4;
                        let a1863 = 1i64;
                        compare(a1862, a1863)
                    };
                    if match m1864 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f556(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f556(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1865 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: u64) -> R<u64> {
    let a1869 = v0;
    let a1870 = 10u64;
    if nat_lt(a1869, a1870) {
        let a1871 = 48u64;
        let a1872 = v0;
        nat_add(a1871, a1872)
    } else {
        let a1873 = 87u64;
        let a1874 = v0;
        nat_add(a1873, a1874)
    }
}

pub fn f483(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1875 = v1;
    match m1875 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f557(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f484(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1876 = v0;
    match m1876 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1879 = f407(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1877, _) => {
                        h1877
                    }
                });
                let a1880 = f407(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1878) => {
                        h1878
                    }
                });
                append_list(a1879, a1880)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f485(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f558(v0, 0i64)?, 0i64)))
}

pub fn f486(v0: Adt11) -> Str {
    let m1881 = v0;
    match m1881 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f487(p1884: List<Adt35>, p1885: List<Adt6>) -> List<Adt6> {
    let m1883 = (p1884, p1885);
    {
        let mut m1883 = m1883;
        loop {
            let step = {
                let (v0, v1) = m1883;
                let m1882 = v0;
                match m1882.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f559(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1883 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f488(p1889: List<Adt38>, p1890: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1888 = (p1889, p1890);
    {
        let mut m1888 = m1888;
        loop {
            let step = {
                let (v0, v1) = m1888;
                let m1886 = v0;
                match m1886.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1887 = f560(v2)?;
                        match m1887 {
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
                Ok(next) => { m1888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(p1893: List<Adt44>, p1894: List<Adt6>) -> R<List<Adt6>> {
    let m1892 = (p1893, p1894);
    {
        let mut m1892 = m1892;
        loop {
            let step = {
                let (v0, v1) = m1892;
                let m1891 = v0;
                match m1891.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f561(v2)?, v1)))
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

pub fn f490(p1897: List<Adt46>, p1898: List<Adt6>) -> List<Adt6> {
    let m1896 = (p1897, p1898);
    {
        let mut m1896 = m1896;
        loop {
            let step = {
                let (v0, v1) = m1896;
                let m1895 = v0;
                match m1895.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f562(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1896 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(p1901: List<Adt54>, p1902: List<Adt6>) -> List<Adt6> {
    let m1900 = (p1901, p1902);
    {
        let mut m1900 = m1900;
        loop {
            let step = {
                let (v0, v1) = m1900;
                let m1899 = v0;
                match m1899.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f563(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f492(p1905: List<Adt56>, p1906: List<Adt6>) -> List<Adt6> {
    let m1904 = (p1905, p1906);
    {
        let mut m1904 = m1904;
        loop {
            let step = {
                let (v0, v1) = m1904;
                let m1903 = v0;
                match m1903.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f564(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1904 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(p1910: List<Adt59>, p1911: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1909 = (p1910, p1911);
    {
        let mut m1909 = m1909;
        loop {
            let step = {
                let (v0, v1) = m1909;
                let m1907 = v0;
                match m1907.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1908 = f565(v2)?;
                        match m1908 {
                            None => {
                                Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1909 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(v0: Adt64) -> R<Option<Adt6>> {
    let m1912 = v0;
    match m1912 {
        Adt64::C0(v1) => {
            Ok(Some(f566(Str::lit("elicitation/create"), Some(f567(v1)))))
        }
        Adt64::C1(v2) => {
            let m1913 = f568(v2)?;
            match m1913 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f566(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt64::C2(v4) => {
            Ok(Some(f566(Str::lit("roots/list"), {
                let m1914 = v4;
                match m1914 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h1915) => {
                                h1915
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f495(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1916 = v0;
    match m1916 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f569(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f496() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f497() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f498() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f499() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f500() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f501(p1921: List<List<u64>>, p1922: List<u64>) -> List<u64> {
    let m1920 = (p1921, p1922);
    {
        let mut m1920 = m1920;
        loop {
            let step = {
                let (v0, v1) = m1920;
                let m1917 = v0;
                match m1917.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a1918 = f570(v2);
                            let a1919 = v1;
                            append_list(a1918, a1919)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1920 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Str) -> R<Str> {
    let m1925 = {
        let a1924 = f359(f360(), f159(f571(f85({
            let a1923 = v0;
            utf8_encode(a1923)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1924)
    };
    match m1925 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f503(v0: List<u64>) -> bool {
    let m1926 = v0;
    match m1926.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a1927 = v1;
            let a1928 = 48u64;
            if nat_eq(a1927, a1928) {
                f572(v2)
            } else {
                false
            }
        }
    }
}

pub fn f504(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt122, Adt125>> {
    if f573(f574(v3.clone())) {
        Ok(Ok::<Adt122, Adt125>(f575(v0.clone(), Adt32::C0(f576(v0)))))
    } else {
        Ok(f577(v0.clone(), v3.clone(), f578(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h1929) => {
                h1929
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h1930, _) => {
                h1930
            }
        }, v2, v1, v3)?))
    }
}

pub fn f505(v0: Adt16, _: Adt5) -> Result<Adt14, Adt126> {
    Ok::<Adt14, Adt126>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f506(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1931 = f579(Str::lit("cursor"), v1);
    match m1931 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f507(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1932 = f580(Str::lit("name"), v1.clone());
    match m1932 {
        Ok(v2) => {
            let m1933 = f581(Str::lit("arguments"), v1.clone());
            match m1933 {
                Ok(v3) => {
                    let m1934 = f581(Str::lit("inputResponses"), v1.clone());
                    match m1934 {
                        Ok(v4) => {
                            let m1935 = f579(Str::lit("requestState"), v1);
                            match m1935 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt126>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt126>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt126>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt126>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt126>(v9)
        }
    }
}

pub fn f508(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1936 = f579(Str::lit("cursor"), v1);
    match m1936 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f509(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1937 = f580(Str::lit("uri"), v1.clone());
    match m1937 {
        Ok(v2) => {
            let m1938 = f581(Str::lit("inputResponses"), v1.clone());
            match m1938 {
                Ok(v3) => {
                    let m1939 = f579(Str::lit("requestState"), v1);
                    match m1939 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt126>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt126>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt126>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt126>(v7)
        }
    }
}

pub fn f510(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1940 = f579(Str::lit("cursor"), v1);
    match m1940 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f511(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1941 = f579(Str::lit("cursor"), v1);
    match m1941 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f512(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1942 = f580(Str::lit("name"), v1.clone());
    match m1942 {
        Ok(v2) => {
            let m1943 = f582(Str::lit("arguments"), v1.clone());
            match m1943 {
                Ok(v3) => {
                    let m1944 = f581(Str::lit("inputResponses"), v1.clone());
                    match m1944 {
                        Ok(v4) => {
                            let m1945 = f579(Str::lit("requestState"), v1);
                            match m1945 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt126>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt126>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt126>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt126>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt126>(v9)
        }
    }
}

pub fn f513(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1946 = f583(Str::lit("ref"), v1.clone());
    match m1946 {
        Ok(v2) => {
            let m1947 = f584(v2);
            match m1947 {
                Ok(v3) => {
                    let m1948 = f583(Str::lit("argument"), v1.clone());
                    match m1948 {
                        Ok(v4) => {
                            let m1949 = f585(v4);
                            match m1949 {
                                Ok(v5) => {
                                    let m1950 = f586(v1);
                                    match m1950 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt126>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt126>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt126>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt126>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt126>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt126>(v11)
        }
    }
}

pub fn f514(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1951 = f580(Str::lit("taskId"), v1);
    match m1951 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f515(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1952 = f580(Str::lit("taskId"), v1.clone());
    match m1952 {
        Ok(v2) => {
            let m1953 = f583(Str::lit("inputResponses"), v1);
            match m1953 {
                Ok(v3) => {
                    Ok::<Adt14, Adt126>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt126>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt126>(v5)
        }
    }
}

pub fn f516(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1954 = f580(Str::lit("taskId"), v1);
    match m1954 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f517(v0: Adt98, v1: Adt16, v2: Option<Adt125>) -> Adt137 {
    let m1955 = v2;
    match m1955 {
        None => {
            Adt137::C0(Adt138::C0(match v0.clone() {
                Adt98::C0(h1956, _, _) => {
                    h1956
                }
            }, match v0.clone() {
                Adt98::C0(_, h1957, _) => {
                    h1957
                }
            }, v1, f445(match v0 {
                Adt98::C0(_, _, h1958) => {
                    h1958
                }
            })))
        }
        Some(v3) => {
            Adt137::C1(v3)
        }
    }
}

pub fn f518(v0: Str, v1: List<Str>) -> Option<Adt125> {
    f587(f588(v0.clone(), v1.clone()), v0, v1)
}

pub fn f519(v0: Adt106, v1: Adt139, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt107>> {
    let m1960 = match v0.clone() {
        Adt106::C0(h1959, _, _, _, _) => {
            h1959
        }
    };
    match m1960 {
        None => {
            Ok(Some(Adt107::C0))
        }
        Some(v5) => {
            let a1961 = v5;
            let a1962 = v3;
            if equal(a1961, a1962) {
                let m1964 = match v0.clone() {
                    Adt106::C0(_, h1963, _, _, _) => {
                        h1963
                    }
                };
                match m1964 {
                    None => {
                        Ok(Some(Adt107::C1))
                    }
                    Some(v6) => {
                        let a1965 = v6;
                        let a1966 = v2;
                        if equal(a1965, a1966) {
                            f589(f590(match v0.clone() {
                                Adt106::C0(_, _, h1967, _, _) => {
                                    h1967
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt106::C0(_, _, _, _, h1968) => {
                                    h1968
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt107::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt107::C3))
            }
        }
    }
}

pub fn f520() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f521(v0: Adt5) -> Adt148 {
    f591(f172(f523(), v0.clone()), f172(f524(), v0))
}

pub fn f522(v0: Adt5, v1: Adt129, v2: Adt129) -> Result<Adt16, Adt126> {
    let m1969 = v1;
    match m1969 {
        Adt129::C0 => {
            Err::<Adt16, Adt126>(Adt126::C2)
        }
        Adt129::C1(v3) => {
            let m1970 = v2;
            match m1970 {
                Adt129::C0 => {
                    Err::<Adt16, Adt126>(Adt126::C3)
                }
                Adt129::C1(v4) => {
                    f592(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f523() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f524() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f525(p1979: Bytes, p1980: List<Bytes>) -> List<Bytes> {
    let m1978 = (p1979, p1980);
    {
        let mut m1978 = m1978;
        loop {
            let step = {
                let (v0, v1) = m1978;
                let m1971 = v1;
                match m1971.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1974 = {
                            let a1972 = v2.clone();
                            length_bytes(a1972)
                        };
                        let a1975 = {
                            let a1973 = v0.clone();
                            length_bytes(a1973)
                        };
                        if nat_eq(a1974, a1975) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1976 = v2;
                                let a1977 = v0;
                                append_bytes(a1976, a1977)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
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

pub fn f526(v0: u64) -> Bytes {
    let m1984 = {
        let a1981 = f593();
        let a1982 = v0;
        let a1983 = 1u64;
        slice_bytes(a1981, a1982, a1983)
    };
    match m1984 {
        None => {
            f360()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f527(v0: u64) -> R<Option<u64>> {
    let a1985 = 64u64;
    let a1986 = v0;
    if if nat_lt(a1985, a1986) {
        let a1987 = v0;
        let a1988 = 91u64;
        nat_lt(a1987, a1988)
    } else {
        false
    } {
        Ok(Some({
            let a1989 = v0;
            let a1990 = 65u64;
            nat_sub(a1989, a1990)
        }))
    } else {
        let a1991 = 96u64;
        let a1992 = v0;
        if if nat_lt(a1991, a1992) {
            let a1993 = v0;
            let a1994 = 123u64;
            nat_lt(a1993, a1994)
        } else {
            false
        } {
            Ok(Some({
                let a1995 = v0;
                let a1996 = 71u64;
                nat_sub(a1995, a1996)
            }))
        } else {
            let a1997 = 47u64;
            let a1998 = v0;
            if if nat_lt(a1997, a1998) {
                let a1999 = v0;
                let a2000 = 58u64;
                nat_lt(a1999, a2000)
            } else {
                false
            } {
                Ok(Some({
                    let a2001 = v0;
                    let a2002 = 4u64;
                    nat_add(a2001, a2002)?
                }))
            } else {
                let a2003 = v0;
                let a2004 = 43u64;
                if nat_eq(a2003, a2004) {
                    Ok(Some(62u64))
                } else {
                    let a2005 = v0;
                    let a2006 = 47u64;
                    if nat_eq(a2005, a2006) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f528(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2007 = v3;
        let a2008 = 61u64;
        if nat_eq(a2007, a2008) {
            let a2009 = f594(v1, 16u64);
            let a2010 = 0u64;
            nat_eq(a2009, a2010)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f595(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f529(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2011 = f594(v2, 4u64);
        let a2012 = 0u64;
        nat_eq(a2011, a2012)
    } else {
        false
    } {
        Ok(Some(f596(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f530(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2015 = {
            let a2013 = v0;
            let a2014 = 4u64;
            nat_mul(a2013, a2014)?
        };
        let a2016 = f597(v1, 16u64);
        nat_add(a2015, a2016)?
    }, List::cons({
        let a2019 = {
            let a2017 = f594(v1, 16u64);
            let a2018 = 16u64;
            nat_mul(a2017, a2018)?
        };
        let a2020 = f597(v2, 4u64);
        nat_add(a2019, a2020)?
    }, List::cons({
        let a2023 = {
            let a2021 = f594(v2, 4u64);
            let a2022 = 64u64;
            nat_mul(a2021, a2022)?
        };
        let a2024 = v3;
        nat_add(a2023, a2024)?
    }, List::<u64>::nil()))))
}

pub fn f531(v0: Adt6) -> bool {
    let m2025 = f219(v0);
    match m2025 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f532(v0: Adt6) -> bool {
    let m2026 = f466(v0);
    match m2026 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f533(v0: Adt6) -> bool {
    let m2027 = f553(v0);
    match m2027 {
        None => {
            false
        }
        Some(v1) => {
            let a2031 = {
                let m2030 = {
                    let a2028 = v1;
                    let a2029 = 0i64;
                    compare(a2028, a2029)
                };
                match m2030 {
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
            bool_not(a2031)
        }
    }
}

pub fn f534(v0: Adt6) -> bool {
    let m2032 = f314(v0);
    match m2032 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f535(v0: Adt6) -> bool {
    let m2033 = f245(v0);
    match m2033 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f536(v0: Adt6) -> bool {
    let m2034 = f314(v0);
    match m2034 {
        None => {
            false
        }
        Some(v1) => {
            f598(v1)
        }
    }
}

pub fn f537(v0: Adt6) -> bool {
    let m2035 = f244(v0);
    match m2035 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f538(v0: Adt6) -> bool {
    let m2036 = f219(v0.clone());
    match m2036 {
        None => {
            let m2037 = f314(v0);
            match m2037 {
                None => {
                    false
                }
                Some(v2) => {
                    f599(v2)
                }
            }
        }
        Some(v1) => {
            f461(v1)
        }
    }
}

pub fn f539(v0: Adt6) -> bool {
    let m2038 = f314(v0);
    match m2038 {
        None => {
            false
        }
        Some(v1) => {
            let m2039 = v1;
            match m2039.uncons() {
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

pub fn f540(v0: Str, p2057: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2056 = p2057;
    {
        let mut m2056 = m2056;
        loop {
            let step = {
                let v1 = m2056;
                let m2040 = v1;
                match m2040.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2044 = v0.clone();
                        let a2045 = {
                            let a2042 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2041, _) = v2.clone();
                                h2041
                            }, List::<Str>::nil()));
                            let a2043 = Str::lit("");
                            join(a2042, a2043)
                        };
                        if equal(a2044, a2045) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2046) = v2;
                                h2046
                            }))
                        } else {
                            let m2048 = f245({
                                let (_, h2047) = v2.clone();
                                h2047
                            });
                            match m2048 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2049 = f172(Str::lit("$anchor"), v4);
                                    match m2049 {
                                        Adt129::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt129::C1(v5) => {
                                            let m2050 = f219(v5);
                                            match m2050 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2053 = v0.clone();
                                                    let a2054 = {
                                                        let a2051 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2052 = Str::lit("");
                                                        join(a2051, a2052)
                                                    };
                                                    if equal(a2053, a2054) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2055) = v2;
                                                            h2055
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
                Ok(next) => { m2056 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f541(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2058 = f222(Str::lit("type"), v0);
    match m2058 {
        Adt129::C0 => {
            None::<Adt13>
        }
        Adt129::C1(v1) => {
            let m2059 = f219(v1);
            match m2059 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f600(v2)
                }
            }
        }
    }
}

pub fn f542(v0: List<u64>) -> R<List<u64>> {
    Ok(f159(f601(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f543(v0: List<u64>, p2062: List<List<u64>>) -> bool {
    let m2061 = p2062;
    {
        let mut m2061 = m2061;
        loop {
            let step = {
                let v1 = m2061;
                let m2060 = v1;
                match m2060.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f602(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2061 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f544(p2065: List<Str>, p2066: List<Str>) -> List<Str> {
    let m2064 = (p2065, p2066);
    {
        let mut m2064 = m2064;
        loop {
            let step = {
                let (v0, v1) = m2064;
                let m2063 = v0;
                match m2063.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(v0: Adt6) -> Str {
    let m2067 = v0;
    match m2067 {
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

pub fn f546(v0: Adt7) -> R<bool> {
    if f368(match v0.clone() {
        Adt7::C0(h2068, _) => {
            h2068
        }
    }) {
        Ok(true)
    } else {
        f603(40u64, match v0.clone() {
            Adt7::C0(h2069, _) => {
                h2069
            }
        }, match v0 {
            Adt7::C0(_, h2070) => {
                h2070
            }
        })
    }
}

pub fn f547(v0: Adt6) -> Str {
    let m2071 = f219(v0);
    match m2071 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f548(p2078: u64, p2079: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m2077 = (p2078, p2079);
    {
        let mut m2077 = m2077;
        loop {
            let step = {
                let (v0, v1) = m2077;
                let m2072 = v0;
                if m2072 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m2072 - 1;
                    let m2073 = v1;
                    match m2073.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m2076 = f604({
                                let (h2074, _) = v3;
                                h2074
                            }, {
                                let (_, h2075) = v3;
                                h2075
                            }, v4)?;
                            match m2076 {
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
                Ok(next) => { m2077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f549() -> u64 {
    67108864u64
}

pub fn f550(v0: Adt147) -> bool {
    let m2080 = v0;
    match m2080 {
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

pub fn f551(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m2081 = v2;
    match m2081 {
        Adt147::C0 => {
            let m2083 = v3;
            match m2083 {
                Adt147::C0 => {
                    Ok(f605(Adt147::C0, f606(f607(match v0.clone() {
                        Adt7::C0(h2084, _) => {
                            h2084
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2085) => {
                            h2085
                        }
                    }, f607(match v1.clone() {
                        Adt7::C0(h2086, _) => {
                            h2086
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2087) => {
                            h2087
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
            let m2082 = v3;
            match m2082 {
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
            let m2088 = v3;
            match m2088 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f606(match v0.clone() {
                        Adt7::C0(h2089, _) => {
                            h2089
                        }
                    }, match v0 {
                        Adt7::C0(_, h2090) => {
                            h2090
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2091, _) => {
                            h2091
                        }
                    }, match v1 {
                        Adt7::C0(_, h2092) => {
                            h2092
                        }
                    })
                }
            }
        }
    }
}

pub fn f552(v0: i64) -> Adt147 {
    f554(v0, 0i64)
}

pub fn f553(v0: Adt6) -> Option<i64> {
    let m2093 = v0;
    match m2093 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f368(match v2.clone() {
                Adt7::C0(_, h2094) => {
                    h2094
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2095, _) => {
                        h2095
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

pub fn f554(v0: i64, v1: i64) -> Adt147 {
    let m2098 = {
        let a2096 = v0;
        let a2097 = v1;
        compare(a2096, a2097)
    };
    if match m2098 {
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
        let m2101 = {
            let a2099 = v1;
            let a2100 = v0;
            compare(a2099, a2100)
        };
        if match m2101 {
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

pub fn f555(p2110: List<u64>, p2111: u64) -> R<u64> {
    let m2109 = (p2110, p2111);
    {
        let mut m2109 = m2109;
        loop {
            let step = {
                let (v0, v1) = m2109;
                let m2102 = v0;
                match m2102.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2103 = v2;
                        let a2104 = 128u64;
                        if if nat_lt(a2103, a2104) {
                            true
                        } else {
                            let a2105 = 191u64;
                            let a2106 = v2;
                            nat_lt(a2105, a2106)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2107 = v1;
                                let a2108 = 1u64;
                                nat_add(a2107, a2108)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f556(v0: i64) -> u64 {
    let m2114 = {
        let a2112 = v0;
        let a2113 = 1i64;
        compare(a2112, a2113)
    };
    if match m2114 {
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
        let m2117 = {
            let a2115 = v0;
            let a2116 = 2i64;
            compare(a2115, a2116)
        };
        if match m2117 {
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
            let m2120 = {
                let a2118 = v0;
                let a2119 = 3i64;
                compare(a2118, a2119)
            };
            if match m2120 {
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
                let m2123 = {
                    let a2121 = v0;
                    let a2122 = 4i64;
                    compare(a2121, a2122)
                };
                if match m2123 {
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
                    let m2126 = {
                        let a2124 = v0;
                        let a2125 = 5i64;
                        compare(a2124, a2125)
                    };
                    if match m2126 {
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
                        let m2129 = {
                            let a2127 = v0;
                            let a2128 = 6i64;
                            compare(a2127, a2128)
                        };
                        if match m2129 {
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
                            let m2132 = {
                                let a2130 = v0;
                                let a2131 = 7i64;
                                compare(a2130, a2131)
                            };
                            if match m2132 {
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
                                let m2135 = {
                                    let a2133 = v0;
                                    let a2134 = 8i64;
                                    compare(a2133, a2134)
                                };
                                if match m2135 {
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
                                    let m2138 = {
                                        let a2136 = v0;
                                        let a2137 = 9i64;
                                        compare(a2136, a2137)
                                    };
                                    if match m2138 {
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

pub fn f557(v0: Adt8) -> Adt6 {
    Adt6::C5(f407(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2139) => {
            h2139
        }
    }))
}

pub fn f558(p2144: u64, p2145: i64) -> R<i64> {
    let m2143 = (p2144, p2145);
    {
        let mut m2143 = m2143;
        loop {
            let step = {
                let (v0, v1) = m2143;
                let m2140 = v0;
                if m2140 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2140 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2141 = v1;
                        let a2142 = -1i64;
                        int_sub(a2141, a2142)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2143 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f559(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2146, _, _, _, _, _, _, _) => {
            h2146
        }
    })), {
        let a2164 = f402(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2147, _, _, _, _, _, _) => {
                h2147
            }
        });
        let a2165 = {
            let a2162 = f495(match v0.clone() {
                Adt35::C0(_, _, h2148, _, _, _, _, _) => {
                    h2148
                }
            });
            let a2163 = {
                let a2160 = f402(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2149, _, _, _, _) => {
                        h2149
                    }
                });
                let a2161 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2152 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2151, _, _, _) => {
                            h2151
                        }
                    };
                    match m2152 {
                        Adt5::C0(h2150) => {
                            h2150
                        }
                    }
                })), {
                    let a2158 = f433(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2153, _, _) => {
                            h2153
                        }
                    });
                    let a2159 = {
                        let a2156 = f608(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2154, _) => {
                                h2154
                            }
                        });
                        let a2157 = f609(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2155) => {
                                h2155
                            }
                        });
                        append_list(a2156, a2157)
                    };
                    append_list(a2158, a2159)
                });
                append_list(a2160, a2161)
            };
            append_list(a2162, a2163)
        };
        append_list(a2164, a2165)
    }))
}

pub fn f560(v0: Adt38) -> R<Option<Adt6>> {
    let m2166 = v0;
    match m2166 {
        Adt38::C0(v1) => {
            Ok(Some(f610(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f611(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2167, _, _, _) => {
                    h2167
                }
            }, match v2.clone() {
                Adt42::C0(_, h2168, _, _) => {
                    h2168
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2169, _) => {
                    h2169
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2170) => {
                    h2170
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f611(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2171, _, _, _) => {
                    h2171
                }
            }, match v3.clone() {
                Adt43::C0(_, h2172, _, _) => {
                    h2172
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2173, _) => {
                    h2173
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2174) => {
                    h2174
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f612(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f613(v5)))
        }
    }
}

pub fn f561(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2198 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2175, _, _, _, _, _, _, _, _) => {
                h2175
            }
        })), List::<(Str, Adt6)>::nil());
        let a2199 = {
            let a2196 = f402(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2176, _, _, _, _, _, _, _) => {
                    h2176
                }
            });
            let a2197 = {
                let a2194 = f495(match v0.clone() {
                    Adt44::C0(_, _, h2177, _, _, _, _, _, _) => {
                        h2177
                    }
                });
                let a2195 = {
                    let a2192 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2178, _, _, _, _, _) => {
                            h2178
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2193 = {
                        let a2190 = f402(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2179, _, _, _, _) => {
                                h2179
                            }
                        });
                        let a2191 = {
                            let a2188 = f402(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2180, _, _, _) => {
                                    h2180
                                }
                            });
                            let a2189 = {
                                let a2186 = f614(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2181, _, _) => {
                                        h2181
                                    }
                                });
                                let a2187 = {
                                    let a2184 = f615(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2182, _) => {
                                            h2182
                                        }
                                    })?;
                                    let a2185 = f609(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2183) => {
                                            h2183
                                        }
                                    });
                                    append_list(a2184, a2185)
                                };
                                append_list(a2186, a2187)
                            };
                            append_list(a2188, a2189)
                        };
                        append_list(a2190, a2191)
                    };
                    append_list(a2192, a2193)
                };
                append_list(a2194, a2195)
            };
            append_list(a2196, a2197)
        };
        append_list(a2198, a2199)
    }))
}

pub fn f562(v0: Adt46) -> Adt6 {
    let m2200 = v0;
    match m2200 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2201, _, _, _) => {
                    h2201
                }
            })), {
                let a2205 = f402(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2202, _, _) => {
                        h2202
                    }
                });
                let a2206 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2203, _) => {
                        h2203
                    }
                })), f609(match v1 {
                    Adt47::C0(_, _, _, h2204) => {
                        h2204
                    }
                }));
                append_list(a2205, a2206)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2207, _, _, _) => {
                    h2207
                }
            })), {
                let a2211 = f402(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2208, _, _) => {
                        h2208
                    }
                });
                let a2212 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2209, _) => {
                        h2209
                    }
                })), f609(match v2 {
                    Adt48::C0(_, _, _, h2210) => {
                        h2210
                    }
                }));
                append_list(a2211, a2212)
            }))
        }
    }
}

pub fn f563(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2233 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2213, _, _, _, _, _, _, _) => {
                h2213
            }
        })), List::<(Str, Adt6)>::nil());
        let a2234 = {
            let a2231 = f402(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2214, _, _, _, _, _, _) => {
                    h2214
                }
            });
            let a2232 = {
                let a2229 = f495(match v0.clone() {
                    Adt54::C0(_, _, h2215, _, _, _, _, _) => {
                        h2215
                    }
                });
                let a2230 = {
                    let a2227 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2216, _, _, _, _) => {
                            h2216
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2228 = {
                        let a2225 = f402(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2217, _, _, _) => {
                                h2217
                            }
                        });
                        let a2226 = {
                            let a2223 = f402(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2218, _, _) => {
                                    h2218
                                }
                            });
                            let a2224 = {
                                let a2221 = f614(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2219, _) => {
                                        h2219
                                    }
                                });
                                let a2222 = f609(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2220) => {
                                        h2220
                                    }
                                });
                                append_list(a2221, a2222)
                            };
                            append_list(a2223, a2224)
                        };
                        append_list(a2225, a2226)
                    };
                    append_list(a2227, a2228)
                };
                append_list(a2229, a2230)
            };
            append_list(a2231, a2232)
        };
        append_list(a2233, a2234)
    })
}

pub fn f564(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2250 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2235, _, _, _, _, _) => {
                h2235
            }
        })), List::<(Str, Adt6)>::nil());
        let a2251 = {
            let a2248 = f402(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2236, _, _, _, _) => {
                    h2236
                }
            });
            let a2249 = {
                let a2246 = f495(match v0.clone() {
                    Adt56::C0(_, _, h2237, _, _, _) => {
                        h2237
                    }
                });
                let a2247 = {
                    let a2244 = f402(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2238, _, _) => {
                            h2238
                        }
                    });
                    let a2245 = {
                        let a2242 = {
                            let m2240 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2239, _) => {
                                    h2239
                                }
                            };
                            match m2240 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f616(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2243 = f609(match v0 {
                            Adt56::C0(_, _, _, _, _, h2241) => {
                                h2241
                            }
                        });
                        append_list(a2242, a2243)
                    };
                    append_list(a2244, a2245)
                };
                append_list(a2246, a2247)
            };
            append_list(a2248, a2249)
        };
        append_list(a2250, a2251)
    })
}

pub fn f565(v0: Adt59) -> R<Option<Adt6>> {
    let m2253 = f560(match v0.clone() {
        Adt59::C0(_, h2252) => {
            h2252
        }
    })?;
    match m2253 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f617(match v0 {
                Adt59::C0(h2254, _) => {
                    h2254
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f566(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f406(Str::lit("params"), v1)))
}

pub fn f567(v0: Adt65) -> Adt6 {
    let m2255 = v0;
    match m2255 {
        Adt65::C0(v1) => {
            f618(v1)
        }
        Adt65::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt68::C0(h2256, _) => {
                    h2256
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt68::C0(_, h2257) => {
                    h2257
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f568(v0: Adt69) -> R<Option<Adt6>> {
    let m2259 = f619(match v0.clone() {
        Adt69::C0(h2258, _, _, _, _, _, _, _, _, _) => {
            h2258
        }
    })?;
    match m2259 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2261 = f620(v0.clone());
                let a2262 = List::cons((Str::lit("maxTokens"), f485(match v0.clone() {
                    Adt69::C0(_, _, _, _, _, h2260, _, _, _, _) => {
                        h2260
                    }
                })?), f621(v0));
                append_list(a2261, a2262)
            }))))
        }
    }
}

pub fn f569(v0: List<Adt2>) -> List<Adt6> {
    f291(f622(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f570(v0: List<u64>) -> List<u64> {
    let a2265 = f623();
    let a2266 = {
        let a2263 = v0;
        let a2264 = f624();
        append_list(a2263, a2264)
    };
    append_list(a2265, a2266)
}

pub fn f571(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f625(v0, 0u64, v1)
}

pub fn f572(p2273: List<u64>) -> bool {
    let m2272 = p2273;
    {
        let mut m2272 = m2272;
        loop {
            let step = {
                let v0 = m2272;
                let m2267 = v0;
                match m2267.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2268 = v1;
                        let a2269 = 48u64;
                        if if nat_eq(a2268, a2269) {
                            true
                        } else {
                            let a2270 = v1;
                            let a2271 = 46u64;
                            nat_eq(a2270, a2271)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f573(v0: Adt134) -> bool {
    let m2274 = v0;
    match m2274 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            false
        }
        Adt134::C2 => {
            false
        }
        Adt134::C3 => {
            false
        }
        Adt134::C4 => {
            false
        }
        Adt134::C5 => {
            false
        }
        Adt134::C6 => {
            false
        }
        Adt134::C7 => {
            false
        }
        Adt134::C8 => {
            false
        }
        Adt134::C9 => {
            false
        }
        Adt134::C10 => {
            false
        }
        Adt134::C11 => {
            false
        }
    }
}

pub fn f574(v0: Adt14) -> Adt134 {
    let m2275 = v0;
    match m2275 {
        Adt14::C0(_) => {
            Adt134::C0
        }
        Adt14::C1(_) => {
            Adt134::C1
        }
        Adt14::C2(_) => {
            Adt134::C2
        }
        Adt14::C3(_) => {
            Adt134::C3
        }
        Adt14::C4(_) => {
            Adt134::C4
        }
        Adt14::C5(_) => {
            Adt134::C5
        }
        Adt14::C6(_) => {
            Adt134::C6
        }
        Adt14::C7(_) => {
            Adt134::C7
        }
        Adt14::C8(_) => {
            Adt134::C8
        }
        Adt14::C9(_) => {
            Adt134::C9
        }
        Adt14::C10(_) => {
            Adt134::C10
        }
        Adt14::C11(_) => {
            Adt134::C11
        }
    }
}

pub fn f575(v0: Adt0, v1: Adt32) -> Adt122 {
    Adt122::C0(v1, Adt123::C0(Some(match v0 {
        Adt0::C0(h2276, _, _, _, _, _, _) => {
            h2276
        }
    }), None::<Adt5>), List::<Adt124>::nil())
}

pub fn f576(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2277, _, _, _) => {
            h2277
        }
    }, f170(), match v0.clone() {
        Adt0::C0(_, _, h2278, _, _, _, _) => {
            h2278
        }
    }, match v0 {
        Adt0::C0(_, h2279, _, _, _, _, _) => {
            h2279
        }
    })
}

pub fn f577(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt122, Adt125> {
    let m2280 = v2;
    match m2280 {
        Ok(v4) => {
            if f626(f574(v1.clone()), v4.clone()) {
                f627(v0, v1, v4)
            } else {
                Err::<Adt122, Adt125>(Adt125::C4(f628()))
            }
        }
        Err(v3) => {
            Err::<Adt122, Adt125>(Adt125::C4(v3))
        }
    }
}

pub fn f578(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14) -> R<Result<Adt32, Adt84>> {
    f629(v0.clone(), v1, v2, v3, v4.clone(), f630(v0, v4)?)
}

pub fn f579(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt126> {
    let m2281 = f172(v0.clone(), v1);
    match m2281 {
        Adt129::C0 => {
            Ok::<Option<Str>, Adt126>(None::<Str>)
        }
        Adt129::C1(v2) => {
            let m2282 = f219(v2);
            match m2282 {
                None => {
                    Err::<Option<Str>, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt126>(Some(v3))
                }
            }
        }
    }
}

pub fn f580(v0: Str, v1: Adt5) -> Result<Str, Adt126> {
    let m2283 = f172(v0.clone(), v1);
    match m2283 {
        Adt129::C0 => {
            Err::<Str, Adt126>(Adt126::C7(v0))
        }
        Adt129::C1(v2) => {
            let m2284 = f219(v2);
            match m2284 {
                None => {
                    Err::<Str, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt126>(v3)
                }
            }
        }
    }
}

pub fn f581(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt126> {
    let m2285 = f172(v0.clone(), v1);
    match m2285 {
        Adt129::C0 => {
            Ok::<Option<Adt5>, Adt126>(None::<Adt5>)
        }
        Adt129::C1(v2) => {
            let m2286 = f245(v2);
            match m2286 {
                None => {
                    Err::<Option<Adt5>, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt126>(Some(v3))
                }
            }
        }
    }
}

pub fn f582(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt126> {
    let m2287 = f172(v0.clone(), v1);
    match m2287 {
        Adt129::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt126>(None::<List<(Str, Str)>>)
        }
        Adt129::C1(v2) => {
            let m2288 = f245(v2);
            match m2288 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    let m2290 = f631(v0, match v3 {
                        Adt5::C0(h2289) => {
                            h2289
                        }
                    });
                    match m2290 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt126>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt126>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f583(v0: Str, v1: Adt5) -> Result<Adt5, Adt126> {
    let m2291 = f172(v0.clone(), v1);
    match m2291 {
        Adt129::C0 => {
            Err::<Adt5, Adt126>(Adt126::C7(v0))
        }
        Adt129::C1(v2) => {
            let m2292 = f245(v2);
            match m2292 {
                None => {
                    Err::<Adt5, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt126>(v3)
                }
            }
        }
    }
}

pub fn f584(v0: Adt5) -> Result<Adt25, Adt126> {
    let m2293 = f580(Str::lit("type"), v0.clone());
    match m2293 {
        Ok(v1) => {
            let a2294 = v1.clone();
            let a2295 = Str::lit("ref/prompt");
            if equal(a2294, a2295) {
                let m2296 = f580(Str::lit("name"), v0.clone());
                match m2296 {
                    Ok(v2) => {
                        let m2297 = f579(Str::lit("title"), v0);
                        match m2297 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt126>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt126>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt126>(v5)
                    }
                }
            } else {
                let a2298 = v1;
                let a2299 = Str::lit("ref/resource");
                if equal(a2298, a2299) {
                    let m2300 = f580(Str::lit("uri"), v0);
                    match m2300 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt126>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt126>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt126>(Adt126::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt126>(v8)
        }
    }
}

pub fn f585(v0: Adt5) -> Result<Adt27, Adt126> {
    let m2301 = f580(Str::lit("name"), v0.clone());
    match m2301 {
        Ok(v1) => {
            let m2302 = f580(Str::lit("value"), v0);
            match m2302 {
                Ok(v2) => {
                    Ok::<Adt27, Adt126>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt126>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt126>(v4)
        }
    }
}

pub fn f586(v0: Adt5) -> Result<Option<Adt28>, Adt126> {
    let m2303 = f172(Str::lit("context"), v0);
    match m2303 {
        Adt129::C0 => {
            Ok::<Option<Adt28>, Adt126>(None::<Adt28>)
        }
        Adt129::C1(v1) => {
            let m2304 = f245(v1);
            match m2304 {
                None => {
                    Err::<Option<Adt28>, Adt126>(Adt126::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2305 = f582(Str::lit("arguments"), v2);
                    match m2305 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt126>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt126>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f587(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt125> {
    if v0 {
        None::<Adt125>
    } else {
        Some(Adt125::C1(v1, v2))
    }
}

pub fn f588(v0: Str, p2310: List<Str>) -> bool {
    let m2309 = p2310;
    {
        let mut m2309 = m2309;
        loop {
            let step = {
                let v1 = m2309;
                let m2306 = v1;
                match m2306.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2307 = v2;
                        let a2308 = v0.clone();
                        if equal(a2307, a2308) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2309 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f589(v0: Option<Adt107>, v1: List<Adt109>, v2: Adt5) -> R<Option<Adt107>> {
    let m2311 = v0;
    match m2311 {
        None => {
            f632(v1, f633(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f590(v0: Option<Str>, v1: Adt139, v2: Adt5) -> Option<Adt107> {
    let m2312 = v1.clone();
    match m2312 {
        Adt139::C0 => {
            None::<Adt107>
        }
        Adt139::C1 => {
            f634(v0, f635(v1, v2))
        }
        Adt139::C2 => {
            f634(v0, f635(v1, v2))
        }
        Adt139::C3 => {
            f634(v0, f635(v1, v2))
        }
    }
}

pub fn f591(v0: Adt129, v1: Adt129) -> Adt148 {
    let m2313 = v0;
    match m2313 {
        Adt129::C0 => {
            Adt148::C1
        }
        Adt129::C1(_) => {
            let m2314 = v1;
            match m2314 {
                Adt129::C0 => {
                    Adt148::C2
                }
                Adt129::C1(_) => {
                    Adt148::C0
                }
            }
        }
    }
}

pub fn f592(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt126> {
    let m2315 = f219(v1);
    match m2315 {
        None => {
            Err::<Adt16, Adt126>(Adt126::C5)
        }
        Some(v3) => {
            let m2316 = f636(v2);
            match m2316 {
                None => {
                    Err::<Adt16, Adt126>(Adt126::C6)
                }
                Some(v4) => {
                    let m2317 = f581(f637(), v0.clone());
                    match m2317 {
                        Ok(v6) => {
                            let m2318 = f638(v0.clone());
                            match m2318 {
                                Ok(v8) => {
                                    let m2319 = f639(v0.clone());
                                    match m2319 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt126>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt126>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt126>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt126>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f593() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f594(v0: u64, v1: u64) -> u64 {
    let a2320 = v0;
    let a2321 = v1;
    let a2322 = 0u64;
    nat_rem(a2320, a2321, a2322)
}

pub fn f595(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2325 = {
            let a2323 = v0;
            let a2324 = 4u64;
            nat_mul(a2323, a2324)?
        };
        let a2326 = f597(v1, 16u64);
        nat_add(a2325, a2326)?
    }, List::<u64>::nil()))
}

pub fn f596(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2329 = {
            let a2327 = v0;
            let a2328 = 4u64;
            nat_mul(a2327, a2328)?
        };
        let a2330 = f597(v1, 16u64);
        nat_add(a2329, a2330)?
    }, List::cons({
        let a2333 = {
            let a2331 = f594(v1, 16u64);
            let a2332 = 16u64;
            nat_mul(a2331, a2332)?
        };
        let a2334 = f597(v2, 4u64);
        nat_add(a2333, a2334)?
    }, List::<u64>::nil())))
}

pub fn f597(v0: u64, v1: u64) -> u64 {
    let a2335 = v0;
    let a2336 = v1;
    let a2337 = 0u64;
    nat_quot(a2335, a2336, a2337)
}

pub fn f598(p2341: List<Adt6>) -> bool {
    let m2340 = p2341;
    {
        let mut m2340 = m2340;
        loop {
            let step = {
                let v0 = m2340;
                let m2338 = v0;
                match m2338.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2339 = f219(v1);
                        match m2339 {
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
                Ok(next) => { m2340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f599(p2345: List<Adt6>) -> bool {
    let m2344 = p2345;
    {
        let mut m2344 = m2344;
        loop {
            let step = {
                let v0 = m2344;
                let m2342 = v0;
                match m2342.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2343 = f219(v1);
                        match m2343 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f461(v3) {
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
                Ok(next) => { m2344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(v0: Str) -> Option<Adt13> {
    let a2346 = v0.clone();
    let a2347 = Str::lit("string");
    if equal(a2346, a2347) {
        Some(Adt13::C0)
    } else {
        let a2348 = v0.clone();
        let a2349 = Str::lit("integer");
        if equal(a2348, a2349) {
            Some(Adt13::C1)
        } else {
            let a2350 = v0;
            let a2351 = Str::lit("boolean");
            if equal(a2350, a2351) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f601(p2360: List<u64>, p2361: List<u64>) -> R<List<u64>> {
    let m2359 = (p2360, p2361);
    {
        let mut m2359 = m2359;
        loop {
            let step = {
                let (v0, v1) = m2359;
                let m2352 = v0;
                match m2352.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2353 = 65u64;
                        let a2354 = v2;
                        if if nat_le(a2353, a2354) {
                            let a2355 = v2;
                            let a2356 = 90u64;
                            nat_le(a2355, a2356)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2357 = v2;
                                let a2358 = 32u64;
                                nat_add(a2357, a2358)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2359 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f602(p2368: List<u64>, p2369: List<u64>) -> bool {
    let m2367 = (p2368, p2369);
    {
        let mut m2367 = m2367;
        loop {
            let step = {
                let (v0, v1) = m2367;
                let m2362 = v0;
                match m2362.uncons() {
                    None => {
                        let m2363 = v1;
                        match m2363.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2364 = v1;
                        match m2364.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2365 = v4;
                                let a2366 = v6;
                                if nat_eq(a2365, a2366) {
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
                Ok(next) => { m2367 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f603(p2381: u64, p2382: i64, p2383: i64) -> R<bool> {
    let m2380 = (p2381, (p2382, p2383));
    {
        let mut m2380 = m2380;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2380;
                let m2370 = v0;
                if m2370 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2370 - 1;
                    let m2373 = {
                        let a2371 = v2;
                        let a2372 = 0i64;
                        compare(a2371, a2372)
                    };
                    if match m2373 {
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
                        if f368({
                            let a2374 = v1;
                            let a2375 = 10i64;
                            let a2376 = 0i64;
                            int_rem(a2374, a2375, a2376)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2377 = v1;
                                let a2378 = 10i64;
                                let a2379 = 0i64;
                                int_quot(a2377, a2378, a2379)?
                            }, f113(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2380 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2384 = f640(v0.clone());
    let a2385 = f640(v1.clone());
    if nat_eq(a2384, a2385) {
        let m2386 = v0;
        match m2386 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2387 = f244(v1);
                match m2387 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2388 = v3;
                        let a2389 = v4;
                        if equal(a2388, a2389) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2390 = f466(v1);
                match m2390 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f641(f468(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2391 = f219(v1);
                match m2391 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2392 = v7;
                        let a2393 = v8;
                        if equal(a2392, a2393) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2394 = f314(v1);
                match m2394 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f642(f643(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2395 = f245(v1);
                match m2395 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2397 = f472(v11.clone(), 0u64)?;
                        let a2398 = f472(match v12.clone() {
                            Adt5::C0(h2396) => {
                                h2396
                            }
                        }, 0u64)?;
                        if nat_eq(a2397, a2398) {
                            Ok(f642(f644(v11, match v12 {
                                Adt5::C0(h2399) => {
                                    h2399
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

pub fn f605(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2400 = v0;
    match m2400 {
        Adt147::C0 => {
            f645(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f606(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2401 = f554(f646(v0, v1)?, f646(v2, v3)?);
    match m2401 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f647(v0, f648(v0)?, v2, f648(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f607(v0: i64) -> R<i64> {
    let m2404 = {
        let a2402 = v0;
        let a2403 = 0i64;
        compare(a2402, a2403)
    };
    if match m2404 {
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
        let a2405 = v0;
        int_neg(a2405)
    } else {
        Ok(v0)
    }
}

pub fn f608(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2406 = v0;
    match m2406 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2418 = f402(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2407, _, _, _, _) => {
                        h2407
                    }
                });
                let a2419 = {
                    let a2416 = f407(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2408, _, _, _) => {
                            h2408
                        }
                    });
                    let a2417 = {
                        let a2414 = f407(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2409, _, _) => {
                                h2409
                            }
                        });
                        let a2415 = {
                            let a2412 = f407(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2410, _) => {
                                    h2410
                                }
                            });
                            let a2413 = f407(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2411) => {
                                    h2411
                                }
                            });
                            append_list(a2412, a2413)
                        };
                        append_list(a2414, a2415)
                    };
                    append_list(a2416, a2417)
                };
                append_list(a2418, a2419)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f609(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f433(Str::lit("_meta"), v0)
}

pub fn f610(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2420, _, _) => {
            h2420
        }
    })), {
        let a2423 = f614(match v0.clone() {
            Adt39::C0(_, h2421, _) => {
                h2421
            }
        });
        let a2424 = f609(match v0 {
            Adt39::C0(_, _, h2422) => {
                h2422
            }
        });
        append_list(a2423, a2424)
    })))
}

pub fn f611(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2425 = f614(v3);
        let a2426 = f609(v4);
        append_list(a2425, a2426)
    }))))
}

pub fn f612(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2450 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2427, _, _, _, _, _, _, _, _) => {
                h2427
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2451 = {
            let a2448 = f402(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2428, _, _, _, _, _, _, _) => {
                    h2428
                }
            });
            let a2449 = {
                let a2446 = f495(match v0.clone() {
                    Adt44::C0(_, _, h2429, _, _, _, _, _, _) => {
                        h2429
                    }
                });
                let a2447 = {
                    let a2444 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2430, _, _, _, _, _) => {
                            h2430
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2445 = {
                        let a2442 = f402(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2431, _, _, _, _) => {
                                h2431
                            }
                        });
                        let a2443 = {
                            let a2440 = f402(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2432, _, _, _) => {
                                    h2432
                                }
                            });
                            let a2441 = {
                                let a2438 = f614(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2433, _, _) => {
                                        h2433
                                    }
                                });
                                let a2439 = {
                                    let a2436 = f615(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2434, _) => {
                                            h2434
                                        }
                                    })?;
                                    let a2437 = f609(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2435) => {
                                            h2435
                                        }
                                    });
                                    append_list(a2436, a2437)
                                };
                                append_list(a2438, a2439)
                            };
                            append_list(a2440, a2441)
                        };
                        append_list(a2442, a2443)
                    };
                    append_list(a2444, a2445)
                };
                append_list(a2446, a2447)
            };
            append_list(a2448, a2449)
        };
        append_list(a2450, a2451)
    }))
}

pub fn f613(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f562(match v0.clone() {
        Adt45::C0(h2452, _, _) => {
            h2452
        }
    })), {
        let a2455 = f614(match v0.clone() {
            Adt45::C0(_, h2453, _) => {
                h2453
            }
        });
        let a2456 = f609(match v0 {
            Adt45::C0(_, _, h2454) => {
                h2454
            }
        });
        append_list(a2455, a2456)
    })))
}

pub fn f614(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2457 = v0;
    match m2457 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2463 = f649(match v1.clone() {
                    Adt40::C0(h2458, _, _) => {
                        h2458
                    }
                });
                let a2464 = {
                    let a2461 = f650(match v1.clone() {
                        Adt40::C0(_, h2459, _) => {
                            h2459
                        }
                    });
                    let a2462 = f402(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2460) => {
                            h2460
                        }
                    });
                    append_list(a2461, a2462)
                };
                append_list(a2463, a2464)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f615(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2465 = v0;
    match m2465 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f485(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f616(v0: List<Adt57>) -> List<Adt6> {
    f291(f651(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f617(v0: Adt41) -> Str {
    let m2466 = v0;
    match m2466 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f618(v0: Adt66) -> Adt6 {
    Adt6::C5({
        let a2473 = {
            let m2468 = match v0.clone() {
                Adt66::C0(h2467, _, _) => {
                    h2467
                }
            };
            match m2468 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2474 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, h2469, _) => {
                h2469
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2472 = match v0 {
                Adt66::C0(_, _, h2471) => {
                    h2471
                }
            };
            match m2472 {
                Adt5::C0(h2470) => {
                    h2470
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2473, a2474)
    })
}

pub fn f619(v0: List<Adt70>) -> R<Option<List<Adt6>>> {
    let m2475 = f652(v0, List::<Adt6>::nil())?;
    match m2475 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f291(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f620(v0: Adt69) -> List<(Str, Adt6)> {
    let a2486 = {
        let m2477 = match v0.clone() {
            Adt69::C0(_, h2476, _, _, _, _, _, _, _, _) => {
                h2476
            }
        };
        match m2477 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f653(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2487 = {
        let a2484 = f402(Str::lit("systemPrompt"), match v0.clone() {
            Adt69::C0(_, _, h2478, _, _, _, _, _, _, _) => {
                h2478
            }
        });
        let a2485 = {
            let a2482 = {
                let m2480 = match v0.clone() {
                    Adt69::C0(_, _, _, h2479, _, _, _, _, _, _) => {
                        h2479
                    }
                };
                match m2480 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f654(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2483 = f655(Str::lit("temperature"), match v0 {
                Adt69::C0(_, _, _, _, h2481, _, _, _, _, _) => {
                    h2481
                }
            });
            append_list(a2482, a2483)
        };
        append_list(a2484, a2485)
    };
    append_list(a2486, a2487)
}

pub fn f621(v0: Adt69) -> List<(Str, Adt6)> {
    let a2499 = {
        let m2489 = match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, h2488, _, _, _) => {
                h2488
            }
        };
        match m2489 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f400(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2500 = {
        let a2497 = f433(Str::lit("metadata"), match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, _, h2490, _, _) => {
                h2490
            }
        });
        let a2498 = {
            let a2495 = {
                let m2492 = match v0.clone() {
                    Adt69::C0(_, _, _, _, _, _, _, _, h2491, _) => {
                        h2491
                    }
                };
                match m2492 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f404(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2496 = {
                let m2494 = match v0 {
                    Adt69::C0(_, _, _, _, _, _, _, _, _, h2493) => {
                        h2493
                    }
                };
                match m2494 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f656(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2495, a2496)
        };
        append_list(a2497, a2498)
    };
    append_list(a2499, a2500)
}

pub fn f622(p2503: List<Adt2>, p2504: List<Adt6>) -> List<Adt6> {
    let m2502 = (p2503, p2504);
    {
        let mut m2502 = m2502;
        loop {
            let step = {
                let (v0, v1) = m2502;
                let m2501 = v0;
                match m2501.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f657(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2502 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f624() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f625(p2509: List<u64>, p2510: u64, p2511: List<u64>) -> List<u64> {
    let m2508 = (p2509, (p2510, p2511));
    {
        let mut m2508 = m2508;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2508;
                let m2505 = v0;
                match m2505.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f658(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f659(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2506, _) => {
                                h2506
                            }
                        }, match v5 {
                            Adt150::C0(_, h2507) => {
                                h2507
                            }
                        })))
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

pub fn f626(v0: Adt134, v1: Adt32) -> bool {
    let m2512 = v1;
    match m2512 {
        Adt32::C0(_) => {
            f660(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f660(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f660(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f660(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f660(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f660(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f660(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f660(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f660(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f660(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f660(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f660(v0.clone(), Adt134::C10) {
                true
            } else {
                f660(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f661(v0)
        }
        Adt32::C13(_) => {
            f662(v0)
        }
    }
}

pub fn f627(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt122, Adt125> {
    let m2513 = v2.clone();
    match m2513 {
        Adt32::C0(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt122, Adt125>(f666(v0, f664(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt122, Adt125>(f575(v0, v2))
        }
        Adt32::C12(v3) => {
            f663(v0, f664(v1), v3)
        }
        Adt32::C13(v4) => {
            f665(v0, f664(v1), v4)
        }
    }
}

pub fn f628() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f629(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14, v5: Result<Adt14, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2514 = v5;
    match m2514 {
        Ok(v7) => {
            f667(v0, v4, f668(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt32, Adt84>(v6))
        }
    }
}

pub fn f630(v0: Bytes, v1: Adt14) -> R<Result<Adt14, Adt84>> {
    let m2515 = v1.clone();
    match m2515 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C1(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C2(v2) => {
            f669(v0, v2)
        }
        Adt14::C3(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C4(v3) => {
            f670(v0, v3)
        }
        Adt14::C5(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C6(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C7(v4) => {
            f671(v0, v4)
        }
        Adt14::C8(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C9(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C10(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C11(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
    }
}

pub fn f631(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt126> {
    let m2516 = f672(v0, v1, List::<(Str, Str)>::nil());
    match m2516 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt126>(f673(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt126>(v3)
        }
    }
}

pub fn f632(p2520: List<Adt109>, v1: Adt5) -> R<Option<Adt107>> {
    let m2519 = p2520;
    {
        let mut m2519 = m2519;
        loop {
            let step = {
                let v0 = m2519;
                let m2517 = v0;
                match m2517.uncons() {
                    None => {
                        Err::<List<Adt109>, R<Option<Adt107>>>(Ok(None::<Adt107>))
                    }
                    Some((v2, v3)) => {
                        let m2518 = f674(v2, v1.clone())?;
                        match m2518 {
                            None => {
                                Ok::<List<Adt109>, R<Option<Adt107>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt109>, R<Option<Adt107>>>(Ok(Some(v4)))
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

pub fn f633(v0: Adt5) -> Adt5 {
    let m2521 = f172(Str::lit("arguments"), v0);
    match m2521 {
        Adt129::C0 => {
            f520()
        }
        Adt129::C1(v1) => {
            let m2522 = f245(v1);
            match m2522 {
                None => {
                    f520()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f634(v0: Option<Str>, v1: Option<Str>) -> Option<Adt107> {
    let m2523 = v0;
    match m2523 {
        None => {
            Some(Adt107::C2)
        }
        Some(v2) => {
            let m2524 = v1;
            match m2524 {
                None => {
                    None::<Adt107>
                }
                Some(v3) => {
                    let a2525 = v2;
                    let a2526 = v3;
                    if equal(a2525, a2526) {
                        None::<Adt107>
                    } else {
                        Some(Adt107::C5)
                    }
                }
            }
        }
    }
}

pub fn f635(v0: Adt139, v1: Adt5) -> Option<Str> {
    let m2527 = v0;
    match m2527 {
        Adt139::C0 => {
            None::<Str>
        }
        Adt139::C1 => {
            let m2528 = f172(Str::lit("name"), v1);
            match m2528 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v2) => {
                    f219(v2)
                }
            }
        }
        Adt139::C2 => {
            let m2529 = f172(Str::lit("uri"), v1);
            match m2529 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v3) => {
                    f219(v3)
                }
            }
        }
        Adt139::C3 => {
            let m2530 = f172(Str::lit("taskId"), v1);
            match m2530 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v4) => {
                    f219(v4)
                }
            }
        }
    }
}

pub fn f636(v0: Adt6) -> Option<Adt17> {
    let m2531 = f245(v0);
    match m2531 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2532 = f675(Str::lit("experimental"), v1.clone());
            match m2532 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2533 = f675(Str::lit("roots"), v1.clone());
                    match m2533 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2534 = f675(Str::lit("sampling"), v1.clone());
                            match m2534 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2535 = f675(Str::lit("elicitation"), v1.clone());
                                    match m2535 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2536 = f675(Str::lit("extensions"), v1);
                                            match m2536 {
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

pub fn f637() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f638(v0: Adt5) -> Result<Option<Adt18>, Adt126> {
    let m2537 = f579(f676(), v0);
    match m2537 {
        Ok(v2) => {
            let m2538 = v2;
            match m2538 {
                None => {
                    Ok::<Option<Adt18>, Adt126>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2539 = f677(v3);
                    match m2539 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt126>(Adt126::C8(f676()))
                        }
                        Adt151::C1(v4) => {
                            Ok::<Option<Adt18>, Adt126>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt126>(v1)
        }
    }
}

pub fn f639(v0: Adt5) -> Result<Option<Adt19>, Adt126> {
    let m2540 = f172(f678(), v0);
    match m2540 {
        Adt129::C0 => {
            Ok::<Option<Adt19>, Adt126>(None::<Adt19>)
        }
        Adt129::C1(v1) => {
            let m2541 = f219(v1.clone());
            match m2541 {
                None => {
                    let m2542 = f553(v1);
                    match m2542 {
                        None => {
                            Err::<Option<Adt19>, Adt126>(Adt126::C8(f678()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt126>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt126>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f640(v0: Adt6) -> u64 {
    let m2543 = v0;
    match m2543 {
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

pub fn f641(v0: Adt147) -> bool {
    let m2544 = v0;
    match m2544 {
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

pub fn f642(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2545 = v0;
    match m2545 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2546 = v2;
                let a2547 = v1;
                append_list(a2546, a2547)
            })
        }
    }
}

pub fn f643(p2552: List<Adt6>, p2553: List<Adt6>, p2554: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2551 = (p2552, (p2553, p2554));
    {
        let mut m2551 = m2551;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2551;
                let m2548 = v0;
                match m2548.uncons() {
                    None => {
                        let m2549 = v1;
                        match m2549.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2550 = v1;
                        match m2550.uncons() {
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
                Ok(next) => { m2551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f644(p2560: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2561: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2559 = (p2560, p2561);
    {
        let mut m2559 = m2559;
        loop {
            let step = {
                let (v0, v2) = m2559;
                let m2555 = v0;
                match m2555.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2557 = f222({
                            let (h2556, _) = v3;
                            h2556
                        }, v1.clone());
                        match m2557 {
                            Adt129::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt129::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2558) = v3;
                                    h2558
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2559 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(v0: Adt147) -> Adt147 {
    let m2562 = v0;
    match m2562 {
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

pub fn f646(v0: i64, v1: i64) -> R<i64> {
    f113(f80(f648(v0)?, 0i64)?, v1)
}

pub fn f647(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2563 = v1;
    let a2564 = v3;
    if nat_lt(a2563, a2564) {
        Ok(f554(f679({
            let a2565 = v3;
            let a2566 = v1;
            nat_sub(a2565, a2566)
        }, v0)?, v2))
    } else {
        let a2567 = v3;
        let a2568 = v1;
        if nat_lt(a2567, a2568) {
            Ok(f554(v0, f679({
                let a2569 = v1;
                let a2570 = v3;
                nat_sub(a2569, a2570)
            }, v2)?))
        } else {
            Ok(f554(v0, v2))
        }
    }
}

pub fn f648(v0: i64) -> R<u64> {
    f680(20u64, v0, 0u64)
}

pub fn f649(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2571 = v0;
    match m2571 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f681(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f650(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2572 = v0;
    match m2572 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f651(p2575: List<Adt57>, p2576: List<Adt6>) -> List<Adt6> {
    let m2574 = (p2575, p2576);
    {
        let mut m2574 = m2574;
        loop {
            let step = {
                let (v0, v1) = m2574;
                let m2573 = v0;
                match m2573.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f682(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f652(p2580: List<Adt70>, p2581: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2579 = (p2580, p2581);
    {
        let mut m2579 = m2579;
        loop {
            let step = {
                let (v0, v1) = m2579;
                let m2577 = v0;
                match m2577.uncons() {
                    None => {
                        Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2578 = f683(v2)?;
                        match m2578 {
                            None => {
                                Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f653(v0: Adt75) -> Adt6 {
    Adt6::C5({
        let a2591 = {
            let m2583 = match v0.clone() {
                Adt75::C0(h2582, _, _, _) => {
                    h2582
                }
            };
            match m2583 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f291(f684(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2592 = {
            let a2589 = f655(Str::lit("costPriority"), match v0.clone() {
                Adt75::C0(_, h2584, _, _) => {
                    h2584
                }
            });
            let a2590 = {
                let a2587 = f655(Str::lit("speedPriority"), match v0.clone() {
                    Adt75::C0(_, _, h2585, _) => {
                        h2585
                    }
                });
                let a2588 = f655(Str::lit("intelligencePriority"), match v0 {
                    Adt75::C0(_, _, _, h2586) => {
                        h2586
                    }
                });
                append_list(a2587, a2588)
            };
            append_list(a2589, a2590)
        };
        append_list(a2591, a2592)
    })
}

pub fn f654(v0: Adt77) -> Str {
    let m2593 = v0;
    match m2593 {
        Adt77::C0 => {
            Str::lit("none")
        }
        Adt77::C1 => {
            Str::lit("thisServer")
        }
        Adt77::C2 => {
            Str::lit("allServers")
        }
    }
}

pub fn f655(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2594 = v1;
    match m2594 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f656(v0: Adt78) -> List<(Str, Adt6)> {
    let m2596 = match v0 {
        Adt78::C0(h2595) => {
            h2595
        }
    };
    match m2596 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f685(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f657(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2597, _, _, _) => {
            h2597
        }
    })), {
        let a2603 = f402(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2598, _, _) => {
                h2598
            }
        });
        let a2604 = {
            let a2601 = f686(match v0.clone() {
                Adt2::C0(_, _, h2599, _) => {
                    h2599
                }
            });
            let a2602 = f687(match v0 {
                Adt2::C0(_, _, _, h2600) => {
                    h2600
                }
            });
            append_list(a2601, a2602)
        };
        append_list(a2603, a2604)
    }))
}

pub fn f658(v0: u64, v1: List<u64>) -> List<u64> {
    let a2605 = v0;
    let a2606 = 0u64;
    if nat_eq(a2605, a2606) {
        v1
    } else {
        let a2607 = v0;
        let a2608 = 1u64;
        if nat_eq(a2607, a2608) {
            List::cons(194u64, v1)
        } else {
            let a2609 = v0;
            let a2610 = 2u64;
            if nat_eq(a2609, a2610) {
                List::cons(226u64, v1)
            } else {
                let a2611 = v0;
                let a2612 = 3u64;
                if nat_eq(a2611, a2612) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f659(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2613 = v0;
    let a2614 = 0u64;
    if nat_eq(a2613, a2614) {
        f688(v1, v2)
    } else {
        let a2615 = v0;
        let a2616 = 1u64;
        if nat_eq(a2615, a2616) {
            let a2617 = 128u64;
            let a2618 = v1;
            if if nat_le(a2617, a2618) {
                let a2619 = v1;
                let a2620 = 159u64;
                nat_le(a2619, a2620)
            } else {
                false
            } {
                Adt150::C0(0u64, f689(v2))
            } else {
                f688(v1, List::cons(194u64, v2))
            }
        } else {
            let a2621 = v0;
            let a2622 = 2u64;
            if nat_eq(a2621, a2622) {
                let a2623 = v1;
                let a2624 = 128u64;
                if nat_eq(a2623, a2624) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2625 = v1;
                    let a2626 = 129u64;
                    if nat_eq(a2625, a2626) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f688(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2627 = v0;
                let a2628 = 3u64;
                if nat_eq(a2627, a2628) {
                    let a2629 = 170u64;
                    let a2630 = v1;
                    if if nat_le(a2629, a2630) {
                        let a2631 = v1;
                        let a2632 = 174u64;
                        nat_le(a2631, a2632)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f689(v2))
                    } else {
                        f688(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2633 = 166u64;
                    let a2634 = v1;
                    if if nat_le(a2633, a2634) {
                        let a2635 = v1;
                        let a2636 = 169u64;
                        nat_le(a2635, a2636)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f689(v2))
                    } else {
                        f688(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f660(v0: Adt134, v1: Adt134) -> bool {
    let a2637 = f437(v0);
    let a2638 = f437(v1);
    equal(a2637, a2638)
}

pub fn f661(v0: Adt134) -> bool {
    let m2639 = v0;
    match m2639 {
        Adt134::C0 => {
            false
        }
        Adt134::C1 => {
            false
        }
        Adt134::C2 => {
            true
        }
        Adt134::C3 => {
            false
        }
        Adt134::C4 => {
            true
        }
        Adt134::C5 => {
            false
        }
        Adt134::C6 => {
            false
        }
        Adt134::C7 => {
            true
        }
        Adt134::C8 => {
            false
        }
        Adt134::C9 => {
            false
        }
        Adt134::C10 => {
            false
        }
        Adt134::C11 => {
            false
        }
    }
}

pub fn f662(v0: Adt134) -> bool {
    let m2640 = v0;
    match m2640 {
        Adt134::C0 => {
            false
        }
        Adt134::C1 => {
            false
        }
        Adt134::C2 => {
            true
        }
        Adt134::C3 => {
            false
        }
        Adt134::C4 => {
            false
        }
        Adt134::C5 => {
            false
        }
        Adt134::C6 => {
            false
        }
        Adt134::C7 => {
            false
        }
        Adt134::C8 => {
            false
        }
        Adt134::C9 => {
            false
        }
        Adt134::C10 => {
            false
        }
        Adt134::C11 => {
            false
        }
    }
}

pub fn f663(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt122, Adt125> {
    f690(v0, v2.clone(), f691(match v1 {
        Adt16::C0(_, h2641, _, _, _, _) => {
            h2641
        }
    }, f415(v2)))
}

pub fn f664(v0: Adt14) -> Adt16 {
    let m2642 = v0;
    match m2642 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2643) => {
                    h2643
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2644, _) => {
                    h2644
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2645, _, _, _, _) => {
                    h2645
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2646, _) => {
                    h2646
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2647, _, _, _) => {
                    h2647
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2648, _) => {
                    h2648
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2649, _) => {
                    h2649
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2650, _, _, _, _) => {
                    h2650
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2651, _, _, _) => {
                    h2651
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2652, _) => {
                    h2652
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2653, _, _) => {
                    h2653
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2654, _) => {
                    h2654
                }
            }
        }
    }
}

pub fn f665(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt122, Adt125> {
    f692(v0, v2, f693(match v1 {
        Adt16::C0(_, h2655, _, _, _, _) => {
            h2655
        }
    }))
}

pub fn f666(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt122 {
    Adt122::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2656, _) => {
            h2656
        }
    }), {
        let m2658 = f575(v0, Adt32::C11);
        match m2658 {
            Adt122::C0(_, h2657, _) => {
                h2657
            }
        }
    }, f694(v1, match v2 {
        Adt49::C0(_, h2659) => {
            h2659
        }
    }))
}

pub fn f667(v0: Bytes, v1: Adt14, v2: Result<Adt32, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2660 = v2;
    match m2660 {
        Ok(v4) => {
            f695(v0, f574(v1.clone()), f696(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f668(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2661 = v3.clone();
    match m2661 {
        Adt14::C0(_) => {
            let c2663: Fn0 = v2;
            c2663.apply(v3)
        }
        Adt14::C1(_) => {
            let c2664: Fn0 = v2;
            c2664.apply(v3)
        }
        Adt14::C2(_) => {
            if f697(v0, v1) {
                let c2662: Fn0 = v2;
                c2662.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2665: Fn0 = v2;
            c2665.apply(v3)
        }
        Adt14::C4(_) => {
            let c2666: Fn0 = v2;
            c2666.apply(v3)
        }
        Adt14::C5(_) => {
            let c2667: Fn0 = v2;
            c2667.apply(v3)
        }
        Adt14::C6(_) => {
            let c2668: Fn0 = v2;
            c2668.apply(v3)
        }
        Adt14::C7(_) => {
            let c2669: Fn0 = v2;
            c2669.apply(v3)
        }
        Adt14::C8(_) => {
            let c2670: Fn0 = v2;
            c2670.apply(v3)
        }
        Adt14::C9(_) => {
            let c2671: Fn0 = v2;
            c2671.apply(v3)
        }
        Adt14::C10(_) => {
            let c2672: Fn0 = v2;
            c2672.apply(v3)
        }
        Adt14::C11(_) => {
            let c2673: Fn0 = v2;
            c2673.apply(v3)
        }
    }
}

pub fn f669(v0: Bytes, v1: Adt21) -> R<Result<Adt14, Adt84>> {
    let m2677 = f698(v0, Str::lit("tools/call"), match v1.clone() {
        Adt21::C0(_, h2674, _, _, _) => {
            h2674
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, _, h2675) => {
            h2675
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, h2676, _) => {
            h2676
        }
    })?;
    match m2677 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C2(Adt21::C0(match v1.clone() {
                Adt21::C0(h2678, _, _, _, _) => {
                    h2678
                }
            }, match v1.clone() {
                Adt21::C0(_, h2679, _, _, _) => {
                    h2679
                }
            }, match v1.clone() {
                Adt21::C0(_, _, h2680, _, _) => {
                    h2680
                }
            }, match v1 {
                Adt21::C0(_, _, _, h2681, _) => {
                    h2681
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f670(v0: Bytes, v1: Adt22) -> R<Result<Adt14, Adt84>> {
    let m2685 = f698(v0, Str::lit("resources/read"), match v1.clone() {
        Adt22::C0(_, h2682, _, _) => {
            h2682
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h2683) => {
            h2683
        }
    }, match v1.clone() {
        Adt22::C0(_, _, h2684, _) => {
            h2684
        }
    })?;
    match m2685 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C4(Adt22::C0(match v1.clone() {
                Adt22::C0(h2686, _, _, _) => {
                    h2686
                }
            }, match v1.clone() {
                Adt22::C0(_, h2687, _, _) => {
                    h2687
                }
            }, match v1 {
                Adt22::C0(_, _, h2688, _) => {
                    h2688
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f671(v0: Bytes, v1: Adt23) -> R<Result<Adt14, Adt84>> {
    let m2692 = f698(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt23::C0(_, h2689, _, _, _) => {
            h2689
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2690) => {
            h2690
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2691, _) => {
            h2691
        }
    })?;
    match m2692 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C7(Adt23::C0(match v1.clone() {
                Adt23::C0(h2693, _, _, _, _) => {
                    h2693
                }
            }, match v1.clone() {
                Adt23::C0(_, h2694, _, _, _) => {
                    h2694
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2695, _, _) => {
                    h2695
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2696, _) => {
                    h2696
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f672(v0: Str, p2702: List<(Str, Adt6)>, p2703: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt126> {
    let m2701 = (p2702, p2703);
    {
        let mut m2701 = m2701;
        loop {
            let step = {
                let (v1, v2) = m2701;
                let m2697 = v1;
                match m2697.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>(Ok::<List<(Str, Str)>, Adt126>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2699 = f219({
                            let (_, h2698) = v3;
                            h2698
                        });
                        match m2699 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>(Err::<List<(Str, Str)>, Adt126>(Adt126::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>((v4, List::cons(({
                                    let (h2700, _) = v3;
                                    h2700
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f673(p2706: List<(Str, Str)>, p2707: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2705 = (p2706, p2707);
    {
        let mut m2705 = m2705;
        loop {
            let step = {
                let (v0, v1) = m2705;
                let m2704 = v0;
                match m2704.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2705 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(v0: Adt109, v1: Adt5) -> R<Option<Adt107>> {
    let m2709 = f699(match v0.clone() {
        Adt109::C0(_, h2708, _, _) => {
            h2708
        }
    }, v1);
    match m2709 {
        Adt129::C0 => {
            Ok(f700(v0))
        }
        Adt129::C1(v2) => {
            if f701(v2.clone()) {
                Ok(f700(v0))
            } else {
                f702(v0, v2)
            }
        }
    }
}

pub fn f675(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2710 = f172(v0, v1);
    match m2710 {
        Adt129::C0 => {
            Some(None::<Adt5>)
        }
        Adt129::C1(v2) => {
            let m2711 = f245(v2);
            match m2711 {
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

pub fn f676() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f677(v0: Str) -> Adt151 {
    let a2712 = v0.clone();
    let a2713 = Str::lit("debug");
    if equal(a2712, a2713) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2714 = v0.clone();
        let a2715 = Str::lit("info");
        if equal(a2714, a2715) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2716 = v0.clone();
            let a2717 = Str::lit("notice");
            if equal(a2716, a2717) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2718 = v0.clone();
                let a2719 = Str::lit("warning");
                if equal(a2718, a2719) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2720 = v0.clone();
                    let a2721 = Str::lit("error");
                    if equal(a2720, a2721) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2722 = v0.clone();
                        let a2723 = Str::lit("critical");
                        if equal(a2722, a2723) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2724 = v0.clone();
                            let a2725 = Str::lit("alert");
                            if equal(a2724, a2725) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2726 = v0;
                                let a2727 = Str::lit("emergency");
                                if equal(a2726, a2727) {
                                    Adt151::C1(Adt18::C7)
                                } else {
                                    Adt151::C0
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f678() -> Str {
    Str::lit("progressToken")
}

pub fn f679(p2732: u64, p2733: i64) -> R<i64> {
    let m2731 = (p2732, p2733);
    {
        let mut m2731 = m2731;
        loop {
            let step = {
                let (v0, v1) = m2731;
                let m2728 = v0;
                if m2728 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2728 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2729 = v1;
                        let a2730 = 10i64;
                        int_mul(a2729, a2730)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2731 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f680(p2744: u64, p2745: i64, p2746: u64) -> R<u64> {
    let m2743 = (p2744, (p2745, p2746));
    {
        let mut m2743 = m2743;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2743;
                let m2734 = v0;
                if m2734 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2734 - 1;
                    let v4: u64 = {
                        let a2735 = v2;
                        let a2736 = 1u64;
                        nat_add(a2735, a2736)?
                    };
                    let m2739 = {
                        let a2737 = v1;
                        let a2738 = 10i64;
                        compare(a2737, a2738)
                    };
                    if match m2739 {
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
                            let a2740 = v1;
                            let a2741 = 10i64;
                            let a2742 = 0i64;
                            int_quot(a2740, a2741, a2742)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2743 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(v0: List<Adt41>) -> List<Adt6> {
    f291(f703(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f682(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2755 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2747, _, _, _) => {
                h2747
            }
        })), List::<(Str, Adt6)>::nil());
        let a2756 = {
            let a2753 = f402(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2748, _, _) => {
                    h2748
                }
            });
            let a2754 = {
                let a2751 = f402(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2749, _) => {
                        h2749
                    }
                });
                let a2752 = f407(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2750) => {
                        h2750
                    }
                });
                append_list(a2751, a2752)
            };
            append_list(a2753, a2754)
        };
        append_list(a2755, a2756)
    })
}

pub fn f683(v0: Adt70) -> R<Option<Adt6>> {
    let m2758 = f704(match v0.clone() {
        Adt70::C0(_, h2757, _) => {
            h2757
        }
    })?;
    match m2758 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f617(match v0.clone() {
                Adt70::C0(h2759, _, _) => {
                    h2759
                }
            }))), List::cons((Str::lit("content"), v1), f609(match v0 {
                Adt70::C0(_, _, h2760) => {
                    h2760
                }
            }))))))
        }
    }
}

pub fn f684(p2764: List<Adt76>, p2765: List<Adt6>) -> List<Adt6> {
    let m2763 = (p2764, p2765);
    {
        let mut m2763 = m2763;
        loop {
            let step = {
                let (v0, v1) = m2763;
                let m2761 = v0;
                match m2761.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt76>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f402(Str::lit("name"), match v2 {
                            Adt76::C0(h2762) => {
                                h2762
                            }
                        })), v1)))
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

pub fn f685(v0: Adt79) -> Str {
    let m2766 = v0;
    match m2766 {
        Adt79::C0 => {
            Str::lit("auto")
        }
        Adt79::C1 => {
            Str::lit("required")
        }
        Adt79::C2 => {
            Str::lit("none")
        }
    }
}

pub fn f686(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2767 = v0;
    match m2767 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f350(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f687(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2768 = v0;
    match m2768 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f705(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f688(v0: u64, v1: List<u64>) -> Adt150 {
    let a2769 = v0;
    let a2770 = 32u64;
    if nat_lt(a2769, a2770) {
        if f706(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f689(v1))
        }
    } else {
        let a2771 = v0;
        let a2772 = 127u64;
        if nat_eq(a2771, a2772) {
            Adt150::C0(0u64, f689(v1))
        } else {
            let a2773 = v0;
            let a2774 = 194u64;
            if nat_eq(a2773, a2774) {
                Adt150::C0(1u64, v1)
            } else {
                let a2775 = v0;
                let a2776 = 226u64;
                if nat_eq(a2775, a2776) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f689(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f690(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt122, Adt125> {
    if v2 {
        Ok::<Adt122, Adt125>(f575(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt122, Adt125>(Adt125::C4(Adt84::C8(f707(f415(v1)))))
    }
}

pub fn f691(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2777 = f708(v1.clone());
    if if if bool_not(a2777) {
        true
    } else {
        f440(match v0.clone() {
            Adt17::C0(_, _, _, h2778, _) => {
                h2778
            }
        })
    } {
        let a2779 = f709(v1.clone());
        if bool_not(a2779) {
            true
        } else {
            f440(match v0.clone() {
                Adt17::C0(_, _, h2780, _, _) => {
                    h2780
                }
            })
        }
    } else {
        false
    } {
        let a2781 = f710(v1);
        if bool_not(a2781) {
            true
        } else {
            f440(match v0 {
                Adt17::C0(_, h2782, _, _, _) => {
                    h2782
                }
            })
        }
    } else {
        false
    }
}

pub fn f692(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt122, Adt125> {
    if v2 {
        Ok::<Adt122, Adt125>(f575(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt122, Adt125>(Adt125::C4(Adt84::C8(f711())))
    }
}

pub fn f693(v0: Adt17) -> bool {
    f441(match v0 {
        Adt17::C0(_, _, _, _, h2783) => {
            h2783
        }
    }, f442())
}

pub fn f694(v0: Adt16, v1: List<Adt50>) -> List<Adt124> {
    let m2785 = match v0 {
        Adt16::C0(_, _, _, _, h2784, _) => {
            h2784
        }
    };
    match m2785 {
        None => {
            List::<Adt124>::nil()
        }
        Some(v2) => {
            f712(f713(v2, v1, List::<Adt124>::nil()), List::<Adt124>::nil())
        }
    }
}

pub fn f695(v0: Bytes, v1: Adt134, v2: Str, v3: Adt32) -> R<Result<Adt32, Adt84>> {
    let m2786 = v3.clone();
    match m2786 {
        Adt32::C0(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C1(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C2(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C3(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C4(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C5(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C6(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C7(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C8(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C9(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C10(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C11 => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
        Adt32::C12(v4) => {
            f714(v0, v1, v2, v4)
        }
        Adt32::C13(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f696(v0: Adt14) -> Str {
    let m2787 = v0;
    match m2787 {
        Adt14::C0(_) => {
            Str::lit("")
        }
        Adt14::C1(_) => {
            Str::lit("")
        }
        Adt14::C2(v1) => {
            match v1 {
                Adt21::C0(_, h2788, _, _, _) => {
                    h2788
                }
            }
        }
        Adt14::C3(_) => {
            Str::lit("")
        }
        Adt14::C4(v2) => {
            match v2 {
                Adt22::C0(_, h2789, _, _) => {
                    h2789
                }
            }
        }
        Adt14::C5(_) => {
            Str::lit("")
        }
        Adt14::C6(_) => {
            Str::lit("")
        }
        Adt14::C7(v3) => {
            match v3 {
                Adt23::C0(_, h2790, _, _, _) => {
                    h2790
                }
            }
        }
        Adt14::C8(_) => {
            Str::lit("")
        }
        Adt14::C9(_) => {
            Str::lit("")
        }
        Adt14::C10(_) => {
            Str::lit("")
        }
        Adt14::C11(_) => {
            Str::lit("")
        }
    }
}

pub fn f697(v0: u64, v1: u64) -> bool {
    let a2791 = v1;
    let a2792 = v0;
    nat_lt(a2791, a2792)
}

pub fn f698(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt84>> {
    Ok(f715(f716(v0, v1, v2, v3)?, f717(v4)))
}

pub fn f699(p2798: List<Str>, p2799: Adt5) -> Adt129 {
    let m2797 = (p2798, p2799);
    {
        let mut m2797 = m2797;
        loop {
            let step = {
                let (v0, v1) = m2797;
                let m2793 = v0;
                match m2793.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt129>(Adt129::C0)
                    }
                    Some((v2, v3)) => {
                        let m2794 = f172(v2, v1);
                        match m2794 {
                            Adt129::C0 => {
                                Err::<(List<Str>, Adt5), Adt129>(Adt129::C0)
                            }
                            Adt129::C1(v4) => {
                                let m2795 = v3.clone();
                                match m2795.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt129>(Adt129::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2796 = f245(v4);
                                        match m2796 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt129>(Adt129::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt129>((v3, v7))
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
                Ok(next) => { m2797 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f700(v0: Adt109) -> Option<Adt107> {
    let m2801 = match v0.clone() {
        Adt109::C0(_, _, _, h2800) => {
            h2800
        }
    };
    match m2801 {
        Adt110::C0 => {
            None::<Adt107>
        }
        Adt110::C1 => {
            Some(Adt107::C10(match v0 {
                Adt109::C0(h2802, _, _, _) => {
                    h2802
                }
            }))
        }
        Adt110::C2(_) => {
            None::<Adt107>
        }
    }
}

pub fn f701(v0: Adt6) -> bool {
    let m2803 = v0;
    match m2803 {
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

pub fn f702(v0: Adt109, v1: Adt6) -> R<Option<Adt107>> {
    let m2805 = match v0.clone() {
        Adt109::C0(_, _, _, h2804) => {
            h2804
        }
    };
    match m2805 {
        Adt110::C0 => {
            Ok(Some(Adt107::C8(match v0 {
                Adt109::C0(h2806, _, _, _) => {
                    h2806
                }
            })))
        }
        Adt110::C1 => {
            Ok(Some(Adt107::C10(match v0 {
                Adt109::C0(h2807, _, _, _) => {
                    h2807
                }
            })))
        }
        Adt110::C2(v2) => {
            if f718(match v0.clone() {
                Adt109::C0(_, _, h2808, _) => {
                    h2808
                }
            }, v2, v1)? {
                Ok(None::<Adt107>)
            } else {
                Ok(Some(Adt107::C9(match v0 {
                    Adt109::C0(h2809, _, _, _) => {
                        h2809
                    }
                })))
            }
        }
    }
}

pub fn f703(p2812: List<Adt41>, p2813: List<Adt6>) -> List<Adt6> {
    let m2811 = (p2812, p2813);
    {
        let mut m2811 = m2811;
        loop {
            let step = {
                let (v0, v1) = m2811;
                let m2810 = v0;
                match m2810.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f617(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2811 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f704(v0: Adt71) -> R<Option<Adt6>> {
    let m2814 = v0;
    match m2814 {
        Adt71::C0(v1) => {
            f719(v1)
        }
        Adt71::C1(v2) => {
            let m2815 = f720(v2, List::<Adt6>::nil())?;
            match m2815 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f291(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f705(v0: Adt3) -> Str {
    let m2816 = v0;
    match m2816 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f706(v0: u64) -> bool {
    let a2817 = v0;
    let a2818 = 9u64;
    if nat_eq(a2817, a2818) {
        true
    } else {
        let a2819 = v0;
        let a2820 = 10u64;
        if nat_eq(a2819, a2820) {
            true
        } else {
            let a2821 = v0;
            let a2822 = 13u64;
            nat_eq(a2821, a2822)
        }
    }
}

pub fn f707(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f710(v0.clone()) {
        Some(f520())
    } else {
        None::<Adt5>
    }, if f709(v0.clone()) {
        Some(f520())
    } else {
        None::<Adt5>
    }, if f708(v0) {
        Some(f520())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f708(p2827: List<(Str, Adt64)>) -> bool {
    let m2826 = p2827;
    {
        let mut m2826 = m2826;
        loop {
            let step = {
                let v0 = m2826;
                let m2823 = v0;
                match m2823.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2825 = {
                            let (_, h2824) = v1;
                            h2824
                        };
                        match m2825 {
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
                Ok(next) => { m2826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f709(p2832: List<(Str, Adt64)>) -> bool {
    let m2831 = p2832;
    {
        let mut m2831 = m2831;
        loop {
            let step = {
                let v0 = m2831;
                let m2828 = v0;
                match m2828.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2830 = {
                            let (_, h2829) = v1;
                            h2829
                        };
                        match m2830 {
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
                Ok(next) => { m2831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710(p2837: List<(Str, Adt64)>) -> bool {
    let m2836 = p2837;
    {
        let mut m2836 = m2836;
        loop {
            let step = {
                let v0 = m2836;
                let m2833 = v0;
                match m2833.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2835 = {
                            let (_, h2834) = v1;
                            h2834
                        };
                        match m2835 {
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
                Ok(next) => { m2836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f711() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f442(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f712(p2840: List<Adt124>, p2841: List<Adt124>) -> List<Adt124> {
    let m2839 = (p2840, p2841);
    {
        let mut m2839 = m2839;
        loop {
            let step = {
                let (v0, v1) = m2839;
                let m2838 = v0;
                match m2838.uncons() {
                    None => {
                        Err::<(List<Adt124>, List<Adt124>), List<Adt124>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt124>, List<Adt124>), List<Adt124>>((v3, List::cons(v2, v1)))
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

pub fn f713(v0: Adt19, p2844: List<Adt50>, p2845: List<Adt124>) -> List<Adt124> {
    let m2843 = (p2844, p2845);
    {
        let mut m2843 = m2843;
        loop {
            let step = {
                let (v1, v2) = m2843;
                let m2842 = v1;
                match m2842.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt124>), List<Adt124>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt124>), List<Adt124>>((v4, List::cons(Adt124::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(v0: Bytes, v1: Adt134, v2: Str, v3: Adt81) -> R<Result<Adt32, Adt84>> {
    if f661(v1.clone()) {
        if f721(v3.clone()) {
            let m2846 = f722(v0, f437(v1), v2, v3)?;
            match m2846 {
                Ok(v5) => {
                    Ok(Ok::<Adt32, Adt84>(Adt32::C12(v5)))
                }
                Err(v4) => {
                    Ok(Err::<Adt32, Adt84>(v4))
                }
            }
        } else {
            Ok(Err::<Adt32, Adt84>(Adt84::C13))
        }
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C12(f437(v1))))
    }
}

pub fn f715(v0: Result<Option<Str>, Adt84>, v1: bool) -> Result<Option<Str>, Adt84> {
    let m2847 = v0;
    match m2847 {
        Ok(v3) => {
            if v1 {
                Ok::<Option<Str>, Adt84>(v3)
            } else {
                Err::<Option<Str>, Adt84>(Adt84::C11)
            }
        }
        Err(v2) => {
            Err::<Option<Str>, Adt84>(v2)
        }
    }
}

pub fn f716(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt84>> {
    let m2848 = v3;
    match m2848 {
        None => {
            Ok(Ok::<Option<Str>, Adt84>(None::<Str>))
        }
        Some(v4) => {
            Ok(f723(f724(v0, v1, v2, v4)?))
        }
    }
}

pub fn f717(v0: Option<Adt5>) -> bool {
    let m2849 = v0;
    match m2849 {
        None => {
            true
        }
        Some(v1) => {
            f725(match v1 {
                Adt5::C0(h2850) => {
                    h2850
                }
            })
        }
    }
}

pub fn f718(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2851 = v0;
    match m2851 {
        Adt13::C0 => {
            let m2852 = f219(v2);
            match m2852 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2853 = v1;
                    let a2854 = v3;
                    Ok(equal(a2853, a2854))
                }
            }
        }
        Adt13::C1 => {
            f726(v1, v2)
        }
        Adt13::C2 => {
            let m2855 = f244(v2);
            match m2855 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2856 = v1;
                        let a2857 = Str::lit("true");
                        Ok(equal(a2856, a2857))
                    } else {
                        let a2858 = v1;
                        let a2859 = Str::lit("false");
                        Ok(equal(a2858, a2859))
                    }
                }
            }
        }
    }
}

pub fn f719(v0: Adt72) -> R<Option<Adt6>> {
    let m2860 = v0;
    match m2860 {
        Adt72::C0(v1) => {
            Ok(Some(f610(v1)))
        }
        Adt72::C1(v2) => {
            Ok(Some(f611(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2861, _, _, _) => {
                    h2861
                }
            }, match v2.clone() {
                Adt42::C0(_, h2862, _, _) => {
                    h2862
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2863, _) => {
                    h2863
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2864) => {
                    h2864
                }
            })))
        }
        Adt72::C2(v3) => {
            Ok(Some(f611(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2865, _, _, _) => {
                    h2865
                }
            }, match v3.clone() {
                Adt43::C0(_, h2866, _, _) => {
                    h2866
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2867, _) => {
                    h2867
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2868) => {
                    h2868
                }
            })))
        }
        Adt72::C3(v4) => {
            Ok(Some(f727(v4)))
        }
        Adt72::C4(v5) => {
            f728(v5)
        }
    }
}

pub fn f720(p2872: List<Adt72>, p2873: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2871 = (p2872, p2873);
    {
        let mut m2871 = m2871;
        loop {
            let step = {
                let (v0, v1) = m2871;
                let m2869 = v0;
                match m2869.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2870 = f719(v2)?;
                        match m2870 {
                            None => {
                                Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(None::<List<Adt6>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>((v3, List::cons(v4, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f721(v0: Adt81) -> bool {
    let m2874 = v0;
    match m2874 {
        Adt81::C0(v1) => {
            let m2875 = v1;
            match m2875.uncons() {
                None => {
                    false
                }
                Some((_, _)) => {
                    true
                }
            }
        }
        Adt81::C1(_) => {
            true
        }
        Adt81::C2(_, _) => {
            true
        }
    }
}

pub fn f722(v0: Bytes, v1: Str, v2: Str, v3: Adt81) -> R<Result<Adt81, Adt84>> {
    let m2876 = v3.clone();
    match m2876 {
        Adt81::C0(_) => {
            Ok(Ok::<Adt81, Adt84>(v3))
        }
        Adt81::C1(v5) => {
            Ok(f729(f730(v0, v1, v2, v5)?, List::<(Str, Adt64)>::nil()))
        }
        Adt81::C2(v6, v7) => {
            Ok(f729(f730(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f723(v0: Option<Str>) -> Result<Option<Str>, Adt84> {
    let m2877 = v0;
    match m2877 {
        None => {
            Err::<Option<Str>, Adt84>(Adt84::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt84>(Some(v1))
        }
    }
}

pub fn f724(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m2881 = f731(v0, {
        let a2878 = v1;
        utf8_encode(a2878)
    }, {
        let a2879 = v2;
        utf8_encode(a2879)
    }, {
        let a2880 = v3;
        utf8_encode(a2880)
    })?;
    match m2881 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a2882 = v4;
            Ok(utf8_decode(a2882))
        }
    }
}

pub fn f725(p2887: List<(Str, Adt6)>) -> bool {
    let m2886 = p2887;
    {
        let mut m2886 = m2886;
        loop {
            let step = {
                let v0 = m2886;
                let m2883 = v0;
                match m2883.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2885 = f245({
                            let (_, h2884) = v1;
                            h2884
                        });
                        match m2885 {
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
                Ok(next) => { m2886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f726(v0: Str, v1: Adt6) -> R<bool> {
    let m2888 = f466(v1);
    match m2888 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2889 = f221(v2)?;
            match m2889 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2891 = f732(f85({
                        let a2890 = v0;
                        utf8_encode(a2890)
                    })?)?;
                    match m2891 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f368({
                                let a2892 = v3;
                                let a2893 = v4;
                                int_sub(a2892, a2893)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f727(v0: Adt73) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt73::C0(h2894, _, _, _) => {
            h2894
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt73::C0(_, h2895, _, _) => {
            h2895
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m2898 = match v0.clone() {
            Adt73::C0(_, _, h2897, _) => {
                h2897
            }
        };
        match m2898 {
            Adt5::C0(h2896) => {
                h2896
            }
        }
    })), f609(match v0 {
        Adt73::C0(_, _, _, h2899) => {
            h2899
        }
    }))))))
}

pub fn f728(v0: Adt74) -> R<Option<Adt6>> {
    let m2901 = f405(match v0.clone() {
        Adt74::C0(_, h2900, _, _, _) => {
            h2900
        }
    })?;
    match m2901 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt74::C0(h2902, _, _, _, _) => {
                    h2902
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a2908 = f406(Str::lit("structuredContent"), match v0.clone() {
                    Adt74::C0(_, _, h2903, _, _) => {
                        h2903
                    }
                });
                let a2909 = {
                    let a2906 = f407(Str::lit("isError"), match v0.clone() {
                        Adt74::C0(_, _, _, h2904, _) => {
                            h2904
                        }
                    });
                    let a2907 = f609(match v0 {
                        Adt74::C0(_, _, _, _, h2905) => {
                            h2905
                        }
                    });
                    append_list(a2906, a2907)
                };
                append_list(a2908, a2909)
            }))))))
        }
    }
}

pub fn f729(v0: Option<Str>, v1: List<(Str, Adt64)>) -> Result<Adt81, Adt84> {
    let m2910 = v0;
    match m2910 {
        None => {
            Err::<Adt81, Adt84>(Adt84::C14)
        }
        Some(v2) => {
            let m2911 = v1.clone();
            match m2911.uncons() {
                None => {
                    Ok::<Adt81, Adt84>(Adt81::C1(v2))
                }
                Some((_, _)) => {
                    Ok::<Adt81, Adt84>(Adt81::C2(v1, v2))
                }
            }
        }
    }
}

pub fn f730(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m2915 = f733(v0, {
        let a2912 = v1;
        utf8_encode(a2912)
    }, {
        let a2913 = v2;
        utf8_encode(a2913)
    }, {
        let a2914 = v3;
        utf8_encode(a2914)
    })?;
    match m2915 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a2916 = v4;
            Ok(utf8_decode(a2916))
        }
    }
}

pub fn f731(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a2918 = f734();
    let a2919 = {
        let a2917 = v0.clone();
        length_bytes(a2917)
    };
    if nat_le(a2918, a2919) {
        let a2921 = f735(v3.clone())?;
        let a2922 = {
            let a2920 = v3.clone();
            length_bytes(a2920)
        };
        if nat_eq(a2921, a2922) {
            Ok(None::<Bytes>)
        } else {
            f736(v0, v1, v2, v3.clone(), f735(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f732(v0: List<u64>) -> R<Option<i64>> {
    let m2923 = v0.clone();
    match m2923.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2924 = v1;
            let a2925 = 45u64;
            if nat_eq(a2924, a2925) {
                let m2926 = v2.clone();
                match m2926.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2927 = f737(v2, 0i64)?;
                        match m2927 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2928 = v5;
                                    int_neg(a2928)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f737(v0, 0i64)
            }
        }
    }
}

pub fn f733(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a2930 = f734();
    let a2931 = {
        let a2929 = v0.clone();
        length_bytes(a2929)
    };
    if nat_le(a2930, a2931) {
        Ok(Some({
            let a2934 = f738(v3.clone())?;
            let a2935 = {
                let a2932 = Bytes::lit(&[0x2e]);
                let a2933 = f738(f739(v0, v1, v2, v3)?)?;
                append_bytes(a2932, a2933)
            };
            append_bytes(a2934, a2935)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f734() -> u64 {
    32u64
}

pub fn f735(v0: Bytes) -> R<u64> {
    let (_, h2936) = f740(Fn3::F741(v0.clone()), {
        let a2937 = v0.clone();
        length_bytes(a2937)
    }, (0u64, {
        let a2938 = v0;
        length_bytes(a2938)
    }))?;
    Ok(h2936)
}

pub fn f736(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m2942 = {
        let a2939 = v3.clone();
        let a2940 = 0u64;
        let a2941 = v4;
        slice_bytes(a2939, a2940, a2941)
    };
    match m2942 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m2953 = {
                let a2950 = v3.clone();
                let a2951 = {
                    let a2943 = v4;
                    let a2944 = 1u64;
                    nat_add(a2943, a2944)?
                };
                let a2952 = {
                    let a2948 = {
                        let a2945 = v3;
                        length_bytes(a2945)
                    };
                    let a2949 = {
                        let a2946 = v4;
                        let a2947 = 1u64;
                        nat_add(a2946, a2947)?
                    };
                    nat_sub(a2948, a2949)
                };
                slice_bytes(a2950, a2951, a2952)
            };
            match m2953 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f742(v0, v1, v2, f743(v5)?, f743(v6)?)
                }
            }
        }
    }
}

pub fn f737(p2959: List<u64>, p2960: i64) -> R<Option<i64>> {
    let m2958 = (p2959, p2960);
    {
        let mut m2958 = m2958;
        loop {
            let step = {
                let (v0, v1) = m2958;
                let m2954 = v0;
                match m2954.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2955 = f744(v2);
                        match m2955 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f113({
                                    let a2956 = v1;
                                    let a2957 = 10i64;
                                    int_mul(a2956, a2957)?
                                }, f290(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f738(v0: Bytes) -> R<Bytes> {
    let (_, h2961) = f745(Fn4::F746(v0.clone()), {
        let a2962 = v0;
        length_bytes(a2962)
    }, (0u64, f360()))?;
    Ok(h2961)
}

pub fn f739(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f747(v0, {
        let a2963 = f748(v1, v2);
        let a2964 = v3;
        append_bytes(a2963, a2964)
    })
}

pub fn f740(v0: Fn3, p2968: u64, p2969: (u64, u64)) -> R<(u64, u64)> {
    let m2967 = (p2968, p2969);
    {
        let mut m2967 = m2967;
        loop {
            let step = {
                let (v1, v2) = m2967;
                let m2965 = v1;
                if m2965 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m2965 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c2966: Fn3 = v0.clone();
                        c2966.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2967 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f741(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a2971 = {
            let (h2970, _) = v1;
            h2970
        };
        let a2972 = 1u64;
        nat_add(a2971, a2972)?
    }, {
        let a2975 = {
            let (_, h2973) = v1;
            h2973
        };
        let a2976 = {
            let a2974 = v0.clone();
            length_bytes(a2974)
        };
        if nat_eq(a2975, a2976) {
            let a2978 = f57(v0, {
                let (h2977, _) = v1;
                h2977
            })?;
            let a2979 = 46u64;
            if nat_eq(a2978, a2979) {
                let (h2980, _) = v1;
                h2980
            } else {
                let (_, h2981) = v1;
                h2981
            }
        } else {
            let (_, h2982) = v1;
            h2982
        }
    }))
}

pub fn f742(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m2983 = v3;
    match m2983 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m2984 = v4;
            match m2984 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f749(f739(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f743(v0: Bytes) -> R<Option<Bytes>> {
    let a2989 = {
        let a2986 = {
            let a2985 = v0.clone();
            length_bytes(a2985)
        };
        let a2987 = 2u64;
        let a2988 = 0u64;
        nat_rem(a2986, a2987, a2988)
    };
    let a2990 = 0u64;
    if nat_eq(a2989, a2990) {
        let v1: (u64, (Bytes, u64)) = f750(Fn5::F751(v0.clone()), {
            let a2992 = {
                let a2991 = v0;
                length_bytes(a2991)
            };
            let a2993 = 2u64;
            let a2994 = 0u64;
            nat_quot(a2992, a2993, a2994)
        }, (0u64, (f360(), 0u64)))?;
        let a2997 = {
            let (_, h2995) = {
                let (_, h2996) = v1.clone();
                h2996
            };
            h2995
        };
        let a2998 = 0u64;
        if nat_eq(a2997, a2998) {
            Ok(Some({
                let (h2999, _) = {
                    let (_, h3000) = v1;
                    h3000
                };
                h2999
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f744(v0: u64) -> Option<u64> {
    let a3001 = v0;
    let a3002 = 48u64;
    if if nat_lt(a3001, a3002) {
        true
    } else {
        let a3003 = 57u64;
        let a3004 = v0;
        nat_lt(a3003, a3004)
    } {
        None::<u64>
    } else {
        Some({
            let a3005 = v0;
            let a3006 = 48u64;
            nat_sub(a3005, a3006)
        })
    }
}

pub fn f745(v0: Fn4, p3010: u64, p3011: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3009 = (p3010, p3011);
    {
        let mut m3009 = m3009;
        loop {
            let step = {
                let (v1, v2) = m3009;
                let m3007 = v1;
                if m3007 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3007 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3008: Fn4 = v0.clone();
                        c3008.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3013 = {
            let (h3012, _) = v1.clone();
            h3012
        };
        let a3014 = 1u64;
        nat_add(a3013, a3014)?
    }, {
        let a3017 = {
            let (_, h3015) = v1;
            h3015
        };
        let a3018 = f752(f57(v0, {
            let (h3016, _) = v1;
            h3016
        })?)?;
        append_bytes(a3017, a3018)
    }))
}

pub fn f747(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f753(f754(v0, v1)?)
}

pub fn f748(v0: Bytes, v1: Bytes) -> Bytes {
    let a3025 = f755({
        let a3019 = v0.clone();
        length_bytes(a3019)
    });
    let a3026 = {
        let a3023 = v0;
        let a3024 = {
            let a3021 = f755({
                let a3020 = v1.clone();
                length_bytes(a3020)
            });
            let a3022 = v1;
            append_bytes(a3021, a3022)
        };
        append_bytes(a3023, a3024)
    };
    append_bytes(a3025, a3026)
}

pub fn f749(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3029 = {
        let a3027 = v0.clone();
        length_bytes(a3027)
    };
    let a3030 = {
        let a3028 = v1.clone();
        length_bytes(a3028)
    };
    if nat_eq(a3029, a3030) {
        let a3031 = f756(v0, v1)?;
        let a3032 = 0u64;
        Ok(nat_eq(a3031, a3032))
    } else {
        Ok(false)
    }
}

pub fn f750(v0: Fn5, p3036: u64, p3037: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3035 = (p3036, p3037);
    {
        let mut m3035 = m3035;
        loop {
            let step = {
                let (v1, v2) = m3035;
                let m3033 = v1;
                if m3033 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3033 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3034: Fn5 = v0.clone();
                        c3034.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f751(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f757(v0, v1)
}

pub fn f752(v0: u64) -> R<Bytes> {
    let a3044 = f758({
        let a3038 = v0;
        let a3039 = 16u64;
        let a3040 = 0u64;
        nat_quot(a3038, a3039, a3040)
    })?;
    let a3045 = f758({
        let a3041 = v0;
        let a3042 = 16u64;
        let a3043 = 0u64;
        nat_rem(a3041, a3042, a3043)
    })?;
    Ok(append_bytes(a3044, a3045))
}

pub fn f753(v0: Adt152) -> R<Bytes> {
    let a3066 = f759(match v0.clone() {
        Adt152::C0(h3046, _, _, _, _, _, _, _) => {
            h3046
        }
    })?;
    let a3067 = {
        let a3064 = f759(match v0.clone() {
            Adt152::C0(_, h3047, _, _, _, _, _, _) => {
                h3047
            }
        })?;
        let a3065 = {
            let a3062 = f759(match v0.clone() {
                Adt152::C0(_, _, h3048, _, _, _, _, _) => {
                    h3048
                }
            })?;
            let a3063 = {
                let a3060 = f759(match v0.clone() {
                    Adt152::C0(_, _, _, h3049, _, _, _, _) => {
                        h3049
                    }
                })?;
                let a3061 = {
                    let a3058 = f759(match v0.clone() {
                        Adt152::C0(_, _, _, _, h3050, _, _, _) => {
                            h3050
                        }
                    })?;
                    let a3059 = {
                        let a3056 = f759(match v0.clone() {
                            Adt152::C0(_, _, _, _, _, h3051, _, _) => {
                                h3051
                            }
                        })?;
                        let a3057 = {
                            let a3054 = f759(match v0.clone() {
                                Adt152::C0(_, _, _, _, _, _, h3052, _) => {
                                    h3052
                                }
                            })?;
                            let a3055 = f759(match v0 {
                                Adt152::C0(_, _, _, _, _, _, _, h3053) => {
                                    h3053
                                }
                            })?;
                            append_bytes(a3054, a3055)
                        };
                        append_bytes(a3056, a3057)
                    };
                    append_bytes(a3058, a3059)
                };
                append_bytes(a3060, a3061)
            };
            append_bytes(a3062, a3063)
        };
        append_bytes(a3064, a3065)
    };
    Ok(append_bytes(a3066, a3067))
}

pub fn f754(v0: Bytes, v1: Bytes) -> R<Adt152> {
    let v2: Adt153 = f760(v0)?;
    let v3: Adt152 = f761(f762(f763(), f764(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f762(f762(f763(), f764(v2, 1549556828u32))?, f765(v3))
}

pub fn f755(v0: u64) -> Bytes {
    let a3093 = f526({
        let a3071 = {
            let a3068 = v0;
            let a3069 = 16777216u64;
            let a3070 = 0u64;
            nat_quot(a3068, a3069, a3070)
        };
        let a3072 = 256u64;
        let a3073 = 0u64;
        nat_rem(a3071, a3072, a3073)
    });
    let a3094 = {
        let a3091 = f526({
            let a3077 = {
                let a3074 = v0;
                let a3075 = 65536u64;
                let a3076 = 0u64;
                nat_quot(a3074, a3075, a3076)
            };
            let a3078 = 256u64;
            let a3079 = 0u64;
            nat_rem(a3077, a3078, a3079)
        });
        let a3092 = {
            let a3089 = f526({
                let a3083 = {
                    let a3080 = v0;
                    let a3081 = 256u64;
                    let a3082 = 0u64;
                    nat_quot(a3080, a3081, a3082)
                };
                let a3084 = 256u64;
                let a3085 = 0u64;
                nat_rem(a3083, a3084, a3085)
            });
            let a3090 = f526({
                let a3086 = v0;
                let a3087 = 256u64;
                let a3088 = 0u64;
                nat_rem(a3086, a3087, a3088)
            });
            append_bytes(a3089, a3090)
        };
        append_bytes(a3091, a3092)
    };
    append_bytes(a3093, a3094)
}

pub fn f756(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3095) = f740(Fn3::F766(v0.clone(), v1), {
        let a3096 = v0;
        length_bytes(a3096)
    }, (0u64, 0u64))?;
    Ok(h3095)
}

pub fn f757(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f767(f57(v0.clone(), {
        let (h3097, _) = v1.clone();
        h3097
    })?);
    let v3: u64 = f767(f57(v0, {
        let a3099 = {
            let (h3098, _) = v1.clone();
            h3098
        };
        let a3100 = 1u64;
        nat_add(a3099, a3100)?
    })?);
    let a3101 = v2;
    let a3102 = 16u64;
    if if nat_lt(a3101, a3102) {
        let a3103 = v3;
        let a3104 = 16u64;
        nat_lt(a3103, a3104)
    } else {
        false
    } {
        Ok(({
            let a3106 = {
                let (h3105, _) = v1.clone();
                h3105
            };
            let a3107 = 2u64;
            nat_add(a3106, a3107)?
        }, ({
            let a3114 = {
                let (h3108, _) = {
                    let (_, h3109) = v1.clone();
                    h3109
                };
                h3108
            };
            let a3115 = f526({
                let a3112 = {
                    let a3110 = v2;
                    let a3111 = 16u64;
                    nat_mul(a3110, a3111)?
                };
                let a3113 = v3;
                nat_add(a3112, a3113)?
            });
            append_bytes(a3114, a3115)
        }, {
            let (_, h3116) = {
                let (_, h3117) = v1;
                h3117
            };
            h3116
        })))
    } else {
        Ok(({
            let a3119 = {
                let (h3118, _) = v1.clone();
                h3118
            };
            let a3120 = 2u64;
            nat_add(a3119, a3120)?
        }, ({
            let (h3121, _) = {
                let (_, h3122) = v1.clone();
                h3122
            };
            h3121
        }, {
            let a3125 = {
                let (_, h3123) = {
                    let (_, h3124) = v1;
                    h3124
                };
                h3123
            };
            let a3126 = 1u64;
            nat_add(a3125, a3126)?
        })))
    }
}

pub fn f758(v0: u64) -> R<Bytes> {
    Ok(f526({
        let a3127 = v0;
        let a3128 = 10u64;
        if nat_lt(a3127, a3128) {
            let a3129 = 48u64;
            let a3130 = v0;
            nat_add(a3129, a3130)?
        } else {
            let a3131 = 87u64;
            let a3132 = v0;
            nat_add(a3131, a3132)?
        }
    }))
}

pub fn f759(v0: u32) -> R<Bytes> {
    let a3137 = f768(v0, 24u32)?;
    let a3138 = {
        let a3135 = f768(v0, 16u32)?;
        let a3136 = {
            let a3133 = f768(v0, 8u32)?;
            let a3134 = f768(v0, 0u32)?;
            append_bytes(a3133, a3134)
        };
        append_bytes(a3135, a3136)
    };
    Ok(append_bytes(a3137, a3138))
}

pub fn f760(v0: Bytes) -> R<Adt153> {
    let a3140 = f769();
    let a3141 = {
        let a3139 = v0.clone();
        length_bytes(a3139)
    };
    if nat_lt(a3140, a3141) {
        Ok(f770(f771(v0)?))
    } else {
        f772(v0)
    }
}

pub fn f761(v0: Adt152, v1: u64, v2: Bytes) -> R<Adt152> {
    let v3: u64 = {
        let a3142 = v2.clone();
        length_bytes(a3142)
    };
    let v4: u64 = {
        let a3148 = {
            let a3145 = {
                let a3143 = v3;
                let a3144 = 72u64;
                nat_add(a3143, a3144)?
            };
            let a3146 = 64u64;
            let a3147 = 0u64;
            nat_quot(a3145, a3146, a3147)
        };
        let a3149 = 64u64;
        nat_mul(a3148, a3149)?
    };
    let (_, h3150) = f773(Fn6::F774(v2, v1, v3, v4), {
        let a3151 = v4;
        let a3152 = 64u64;
        let a3153 = 0u64;
        nat_quot(a3151, a3152, a3153)
    }, (0u64, v0))?;
    Ok(h3150)
}

pub fn f762(v0: Adt152, v1: Adt153) -> R<Adt152> {
    let v2: Adt154 = f775(Fn7::F776, 64u64, Adt154::C0(match v0.clone() {
        Adt152::C0(h3154, _, _, _, _, _, _, _) => {
            h3154
        }
    }, match v0.clone() {
        Adt152::C0(_, h3155, _, _, _, _, _, _) => {
            h3155
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3156, _, _, _, _, _) => {
            h3156
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3157, _, _, _, _) => {
            h3157
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3158, _, _, _) => {
            h3158
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3159, _, _) => {
            h3159
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3160, _) => {
            h3160
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, _, h3161) => {
            h3161
        }
    }, v1, 0u64))?;
    Ok(Adt152::C0(f777(match v0.clone() {
        Adt152::C0(h3162, _, _, _, _, _, _, _) => {
            h3162
        }
    }, match v2.clone() {
        Adt154::C0(h3163, _, _, _, _, _, _, _, _, _) => {
            h3163
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, h3164, _, _, _, _, _, _) => {
            h3164
        }
    }, match v2.clone() {
        Adt154::C0(_, h3165, _, _, _, _, _, _, _, _) => {
            h3165
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, _, h3166, _, _, _, _, _) => {
            h3166
        }
    }, match v2.clone() {
        Adt154::C0(_, _, h3167, _, _, _, _, _, _, _) => {
            h3167
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, _, _, h3168, _, _, _, _) => {
            h3168
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, h3169, _, _, _, _, _, _) => {
            h3169
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, _, _, _, h3170, _, _, _) => {
            h3170
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, h3171, _, _, _, _, _) => {
            h3171
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3172, _, _) => {
            h3172
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, _, h3173, _, _, _, _) => {
            h3173
        }
    }), f777(match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3174, _) => {
            h3174
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, _, _, h3175, _, _, _) => {
            h3175
        }
    }), f777(match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3176) => {
            h3176
        }
    }, match v2 {
        Adt154::C0(_, _, _, _, _, _, _, h3177, _, _) => {
            h3177
        }
    })))
}

pub fn f763() -> Adt152 {
    Adt152::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f764(v0: Adt153, v1: u32) -> Adt153 {
    Adt153::C0({
        let a3179 = match v0.clone() {
            Adt153::C0(h3178, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3178
            }
        };
        let a3180 = v1;
        fixed_u32::bit_xor(a3179, a3180)
    }, {
        let a3182 = match v0.clone() {
            Adt153::C0(_, h3181, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3181
            }
        };
        let a3183 = v1;
        fixed_u32::bit_xor(a3182, a3183)
    }, {
        let a3185 = match v0.clone() {
            Adt153::C0(_, _, h3184, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3184
            }
        };
        let a3186 = v1;
        fixed_u32::bit_xor(a3185, a3186)
    }, {
        let a3188 = match v0.clone() {
            Adt153::C0(_, _, _, h3187, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3187
            }
        };
        let a3189 = v1;
        fixed_u32::bit_xor(a3188, a3189)
    }, {
        let a3191 = match v0.clone() {
            Adt153::C0(_, _, _, _, h3190, _, _, _, _, _, _, _, _, _, _, _) => {
                h3190
            }
        };
        let a3192 = v1;
        fixed_u32::bit_xor(a3191, a3192)
    }, {
        let a3194 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, h3193, _, _, _, _, _, _, _, _, _, _) => {
                h3193
            }
        };
        let a3195 = v1;
        fixed_u32::bit_xor(a3194, a3195)
    }, {
        let a3197 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, h3196, _, _, _, _, _, _, _, _, _) => {
                h3196
            }
        };
        let a3198 = v1;
        fixed_u32::bit_xor(a3197, a3198)
    }, {
        let a3200 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, h3199, _, _, _, _, _, _, _, _) => {
                h3199
            }
        };
        let a3201 = v1;
        fixed_u32::bit_xor(a3200, a3201)
    }, {
        let a3203 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, h3202, _, _, _, _, _, _, _) => {
                h3202
            }
        };
        let a3204 = v1;
        fixed_u32::bit_xor(a3203, a3204)
    }, {
        let a3206 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, h3205, _, _, _, _, _, _) => {
                h3205
            }
        };
        let a3207 = v1;
        fixed_u32::bit_xor(a3206, a3207)
    }, {
        let a3209 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, h3208, _, _, _, _, _) => {
                h3208
            }
        };
        let a3210 = v1;
        fixed_u32::bit_xor(a3209, a3210)
    }, {
        let a3212 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, h3211, _, _, _, _) => {
                h3211
            }
        };
        let a3213 = v1;
        fixed_u32::bit_xor(a3212, a3213)
    }, {
        let a3215 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3214, _, _, _) => {
                h3214
            }
        };
        let a3216 = v1;
        fixed_u32::bit_xor(a3215, a3216)
    }, {
        let a3218 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3217, _, _) => {
                h3217
            }
        };
        let a3219 = v1;
        fixed_u32::bit_xor(a3218, a3219)
    }, {
        let a3221 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3220, _) => {
                h3220
            }
        };
        let a3222 = v1;
        fixed_u32::bit_xor(a3221, a3222)
    }, {
        let a3224 = match v0 {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3223) => {
                h3223
            }
        };
        let a3225 = v1;
        fixed_u32::bit_xor(a3224, a3225)
    })
}

pub fn f765(v0: Adt152) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt152::C0(h3226, _, _, _, _, _, _, _) => {
            h3226
        }
    }, match v0.clone() {
        Adt152::C0(_, h3227, _, _, _, _, _, _) => {
            h3227
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3228, _, _, _, _, _) => {
            h3228
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3229, _, _, _, _) => {
            h3229
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3230, _, _, _) => {
            h3230
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3231, _, _) => {
            h3231
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3232, _) => {
            h3232
        }
    }, match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3233) => {
            h3233
        }
    }, 2147483648u32, f778(), f778(), f778(), f778(), f778(), f778(), 768u32)
}

pub fn f766(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3235 = {
            let (h3234, _) = v2;
            h3234
        };
        let a3236 = 1u64;
        nat_add(a3235, a3236)?
    }, {
        let a3288 = {
            let (_, h3237) = v2;
            h3237
        };
        let a3289 = {
            let v3: u8 = {
                let a3240 = f779(v0, {
                    let (h3238, _) = v2;
                    h3238
                });
                let a3241 = f779(v1, {
                    let (h3239, _) = v2;
                    h3239
                });
                fixed_u8::bit_xor(a3240, a3241)
            };
            let a3286 = {
                let a3262 = {
                    let a3250 = {
                        let a3244 = {
                            let a3242 = v3;
                            let a3243 = 128u8;
                            fixed_u8::bit_and(a3242, a3243)
                        };
                        let a3245 = 128u8;
                        if equal(a3244, a3245) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3251 = {
                        let a3248 = {
                            let a3246 = v3;
                            let a3247 = 64u8;
                            fixed_u8::bit_and(a3246, a3247)
                        };
                        let a3249 = 64u8;
                        if equal(a3248, a3249) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3250, a3251)?
                };
                let a3263 = {
                    let a3260 = {
                        let a3254 = {
                            let a3252 = v3;
                            let a3253 = 32u8;
                            fixed_u8::bit_and(a3252, a3253)
                        };
                        let a3255 = 32u8;
                        if equal(a3254, a3255) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3261 = {
                        let a3258 = {
                            let a3256 = v3;
                            let a3257 = 16u8;
                            fixed_u8::bit_and(a3256, a3257)
                        };
                        let a3259 = 16u8;
                        if equal(a3258, a3259) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3260, a3261)?
                };
                nat_add(a3262, a3263)?
            };
            let a3287 = {
                let a3284 = {
                    let a3272 = {
                        let a3266 = {
                            let a3264 = v3;
                            let a3265 = 8u8;
                            fixed_u8::bit_and(a3264, a3265)
                        };
                        let a3267 = 8u8;
                        if equal(a3266, a3267) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3273 = {
                        let a3270 = {
                            let a3268 = v3;
                            let a3269 = 4u8;
                            fixed_u8::bit_and(a3268, a3269)
                        };
                        let a3271 = 4u8;
                        if equal(a3270, a3271) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3272, a3273)?
                };
                let a3285 = {
                    let a3282 = {
                        let a3276 = {
                            let a3274 = v3;
                            let a3275 = 2u8;
                            fixed_u8::bit_and(a3274, a3275)
                        };
                        let a3277 = 2u8;
                        if equal(a3276, a3277) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3283 = {
                        let a3280 = {
                            let a3278 = v3;
                            let a3279 = 1u8;
                            fixed_u8::bit_and(a3278, a3279)
                        };
                        let a3281 = 1u8;
                        if equal(a3280, a3281) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3282, a3283)?
                };
                nat_add(a3284, a3285)?
            };
            nat_add(a3286, a3287)?
        };
        nat_add(a3288, a3289)?
    }))
}

pub fn f767(v0: u64) -> u64 {
    let a3290 = v0;
    let a3291 = 48u64;
    if nat_lt(a3290, a3291) {
        16u64
    } else {
        let a3292 = v0;
        let a3293 = 58u64;
        if nat_lt(a3292, a3293) {
            let a3294 = v0;
            let a3295 = 48u64;
            nat_sub(a3294, a3295)
        } else {
            let a3296 = v0;
            let a3297 = 97u64;
            if nat_lt(a3296, a3297) {
                16u64
            } else {
                let a3298 = v0;
                let a3299 = 103u64;
                if nat_lt(a3298, a3299) {
                    let a3300 = v0;
                    let a3301 = 87u64;
                    nat_sub(a3300, a3301)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f768(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f526({
        let v2: u32 = {
            let a3302 = f780(v0, v1);
            let a3303 = 255u32;
            fixed_u32::bit_and(a3302, a3303)
        };
        let a3492 = {
            let a3396 = {
                let a3348 = {
                    let a3324 = {
                        let a3312 = {
                            let a3306 = {
                                let a3304 = v2;
                                let a3305 = 2147483648u32;
                                fixed_u32::bit_and(a3304, a3305)
                            };
                            let a3307 = 2147483648u32;
                            if equal(a3306, a3307) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3313 = {
                            let a3310 = {
                                let a3308 = v2;
                                let a3309 = 1073741824u32;
                                fixed_u32::bit_and(a3308, a3309)
                            };
                            let a3311 = 1073741824u32;
                            if equal(a3310, a3311) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3312, a3313)?
                    };
                    let a3325 = {
                        let a3322 = {
                            let a3316 = {
                                let a3314 = v2;
                                let a3315 = 536870912u32;
                                fixed_u32::bit_and(a3314, a3315)
                            };
                            let a3317 = 536870912u32;
                            if equal(a3316, a3317) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3323 = {
                            let a3320 = {
                                let a3318 = v2;
                                let a3319 = 268435456u32;
                                fixed_u32::bit_and(a3318, a3319)
                            };
                            let a3321 = 268435456u32;
                            if equal(a3320, a3321) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3322, a3323)?
                    };
                    nat_add(a3324, a3325)?
                };
                let a3349 = {
                    let a3346 = {
                        let a3334 = {
                            let a3328 = {
                                let a3326 = v2;
                                let a3327 = 134217728u32;
                                fixed_u32::bit_and(a3326, a3327)
                            };
                            let a3329 = 134217728u32;
                            if equal(a3328, a3329) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3335 = {
                            let a3332 = {
                                let a3330 = v2;
                                let a3331 = 67108864u32;
                                fixed_u32::bit_and(a3330, a3331)
                            };
                            let a3333 = 67108864u32;
                            if equal(a3332, a3333) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3334, a3335)?
                    };
                    let a3347 = {
                        let a3344 = {
                            let a3338 = {
                                let a3336 = v2;
                                let a3337 = 33554432u32;
                                fixed_u32::bit_and(a3336, a3337)
                            };
                            let a3339 = 33554432u32;
                            if equal(a3338, a3339) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3345 = {
                            let a3342 = {
                                let a3340 = v2;
                                let a3341 = 16777216u32;
                                fixed_u32::bit_and(a3340, a3341)
                            };
                            let a3343 = 16777216u32;
                            if equal(a3342, a3343) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3344, a3345)?
                    };
                    nat_add(a3346, a3347)?
                };
                nat_add(a3348, a3349)?
            };
            let a3397 = {
                let a3394 = {
                    let a3370 = {
                        let a3358 = {
                            let a3352 = {
                                let a3350 = v2;
                                let a3351 = 8388608u32;
                                fixed_u32::bit_and(a3350, a3351)
                            };
                            let a3353 = 8388608u32;
                            if equal(a3352, a3353) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3359 = {
                            let a3356 = {
                                let a3354 = v2;
                                let a3355 = 4194304u32;
                                fixed_u32::bit_and(a3354, a3355)
                            };
                            let a3357 = 4194304u32;
                            if equal(a3356, a3357) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3358, a3359)?
                    };
                    let a3371 = {
                        let a3368 = {
                            let a3362 = {
                                let a3360 = v2;
                                let a3361 = 2097152u32;
                                fixed_u32::bit_and(a3360, a3361)
                            };
                            let a3363 = 2097152u32;
                            if equal(a3362, a3363) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3369 = {
                            let a3366 = {
                                let a3364 = v2;
                                let a3365 = 1048576u32;
                                fixed_u32::bit_and(a3364, a3365)
                            };
                            let a3367 = 1048576u32;
                            if equal(a3366, a3367) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3368, a3369)?
                    };
                    nat_add(a3370, a3371)?
                };
                let a3395 = {
                    let a3392 = {
                        let a3380 = {
                            let a3374 = {
                                let a3372 = v2;
                                let a3373 = 524288u32;
                                fixed_u32::bit_and(a3372, a3373)
                            };
                            let a3375 = 524288u32;
                            if equal(a3374, a3375) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3381 = {
                            let a3378 = {
                                let a3376 = v2;
                                let a3377 = 262144u32;
                                fixed_u32::bit_and(a3376, a3377)
                            };
                            let a3379 = 262144u32;
                            if equal(a3378, a3379) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3380, a3381)?
                    };
                    let a3393 = {
                        let a3390 = {
                            let a3384 = {
                                let a3382 = v2;
                                let a3383 = 131072u32;
                                fixed_u32::bit_and(a3382, a3383)
                            };
                            let a3385 = 131072u32;
                            if equal(a3384, a3385) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3391 = {
                            let a3388 = {
                                let a3386 = v2;
                                let a3387 = 65536u32;
                                fixed_u32::bit_and(a3386, a3387)
                            };
                            let a3389 = 65536u32;
                            if equal(a3388, a3389) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3390, a3391)?
                    };
                    nat_add(a3392, a3393)?
                };
                nat_add(a3394, a3395)?
            };
            nat_add(a3396, a3397)?
        };
        let a3493 = {
            let a3490 = {
                let a3442 = {
                    let a3418 = {
                        let a3406 = {
                            let a3400 = {
                                let a3398 = v2;
                                let a3399 = 32768u32;
                                fixed_u32::bit_and(a3398, a3399)
                            };
                            let a3401 = 32768u32;
                            if equal(a3400, a3401) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3407 = {
                            let a3404 = {
                                let a3402 = v2;
                                let a3403 = 16384u32;
                                fixed_u32::bit_and(a3402, a3403)
                            };
                            let a3405 = 16384u32;
                            if equal(a3404, a3405) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3406, a3407)?
                    };
                    let a3419 = {
                        let a3416 = {
                            let a3410 = {
                                let a3408 = v2;
                                let a3409 = 8192u32;
                                fixed_u32::bit_and(a3408, a3409)
                            };
                            let a3411 = 8192u32;
                            if equal(a3410, a3411) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3417 = {
                            let a3414 = {
                                let a3412 = v2;
                                let a3413 = 4096u32;
                                fixed_u32::bit_and(a3412, a3413)
                            };
                            let a3415 = 4096u32;
                            if equal(a3414, a3415) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3416, a3417)?
                    };
                    nat_add(a3418, a3419)?
                };
                let a3443 = {
                    let a3440 = {
                        let a3428 = {
                            let a3422 = {
                                let a3420 = v2;
                                let a3421 = 2048u32;
                                fixed_u32::bit_and(a3420, a3421)
                            };
                            let a3423 = 2048u32;
                            if equal(a3422, a3423) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3429 = {
                            let a3426 = {
                                let a3424 = v2;
                                let a3425 = 1024u32;
                                fixed_u32::bit_and(a3424, a3425)
                            };
                            let a3427 = 1024u32;
                            if equal(a3426, a3427) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3428, a3429)?
                    };
                    let a3441 = {
                        let a3438 = {
                            let a3432 = {
                                let a3430 = v2;
                                let a3431 = 512u32;
                                fixed_u32::bit_and(a3430, a3431)
                            };
                            let a3433 = 512u32;
                            if equal(a3432, a3433) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3439 = {
                            let a3436 = {
                                let a3434 = v2;
                                let a3435 = 256u32;
                                fixed_u32::bit_and(a3434, a3435)
                            };
                            let a3437 = 256u32;
                            if equal(a3436, a3437) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3438, a3439)?
                    };
                    nat_add(a3440, a3441)?
                };
                nat_add(a3442, a3443)?
            };
            let a3491 = {
                let a3488 = {
                    let a3464 = {
                        let a3452 = {
                            let a3446 = {
                                let a3444 = v2;
                                let a3445 = 128u32;
                                fixed_u32::bit_and(a3444, a3445)
                            };
                            let a3447 = 128u32;
                            if equal(a3446, a3447) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3453 = {
                            let a3450 = {
                                let a3448 = v2;
                                let a3449 = 64u32;
                                fixed_u32::bit_and(a3448, a3449)
                            };
                            let a3451 = 64u32;
                            if equal(a3450, a3451) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3452, a3453)?
                    };
                    let a3465 = {
                        let a3462 = {
                            let a3456 = {
                                let a3454 = v2;
                                let a3455 = 32u32;
                                fixed_u32::bit_and(a3454, a3455)
                            };
                            let a3457 = 32u32;
                            if equal(a3456, a3457) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3463 = {
                            let a3460 = {
                                let a3458 = v2;
                                let a3459 = 16u32;
                                fixed_u32::bit_and(a3458, a3459)
                            };
                            let a3461 = 16u32;
                            if equal(a3460, a3461) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3462, a3463)?
                    };
                    nat_add(a3464, a3465)?
                };
                let a3489 = {
                    let a3486 = {
                        let a3474 = {
                            let a3468 = {
                                let a3466 = v2;
                                let a3467 = 8u32;
                                fixed_u32::bit_and(a3466, a3467)
                            };
                            let a3469 = 8u32;
                            if equal(a3468, a3469) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3475 = {
                            let a3472 = {
                                let a3470 = v2;
                                let a3471 = 4u32;
                                fixed_u32::bit_and(a3470, a3471)
                            };
                            let a3473 = 4u32;
                            if equal(a3472, a3473) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3474, a3475)?
                    };
                    let a3487 = {
                        let a3484 = {
                            let a3478 = {
                                let a3476 = v2;
                                let a3477 = 2u32;
                                fixed_u32::bit_and(a3476, a3477)
                            };
                            let a3479 = 2u32;
                            if equal(a3478, a3479) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3485 = {
                            let a3482 = {
                                let a3480 = v2;
                                let a3481 = 1u32;
                                fixed_u32::bit_and(a3480, a3481)
                            };
                            let a3483 = 1u32;
                            if equal(a3482, a3483) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3484, a3485)?
                    };
                    nat_add(a3486, a3487)?
                };
                nat_add(a3488, a3489)?
            };
            nat_add(a3490, a3491)?
        };
        nat_add(a3492, a3493)?
    }))
}

pub fn f769() -> u64 {
    64u64
}

pub fn f770(v0: Adt152) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt152::C0(h3494, _, _, _, _, _, _, _) => {
            h3494
        }
    }, match v0.clone() {
        Adt152::C0(_, h3495, _, _, _, _, _, _) => {
            h3495
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3496, _, _, _, _, _) => {
            h3496
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3497, _, _, _, _) => {
            h3497
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3498, _, _, _) => {
            h3498
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3499, _, _) => {
            h3499
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3500, _) => {
            h3500
        }
    }, match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3501) => {
            h3501
        }
    }, f778(), f778(), f778(), f778(), f778(), f778(), f778(), f778())
}

pub fn f771(v0: Bytes) -> R<Adt152> {
    let v1: u64 = {
        let a3502 = v0.clone();
        length_bytes(a3502)
    };
    let v2: u64 = {
        let a3508 = {
            let a3505 = {
                let a3503 = v1;
                let a3504 = 72u64;
                nat_add(a3503, a3504)?
            };
            let a3506 = 64u64;
            let a3507 = 0u64;
            nat_quot(a3505, a3506, a3507)
        };
        let a3509 = 64u64;
        nat_mul(a3508, a3509)?
    };
    let (_, h3510) = f773(Fn6::F781(v0, v1, v2), {
        let a3511 = v2;
        let a3512 = 64u64;
        let a3513 = 0u64;
        nat_quot(a3511, a3512, a3513)
    }, (0u64, f763()))?;
    Ok(h3510)
}

pub fn f772(v0: Bytes) -> R<Adt153> {
    Ok(Adt153::C0(f782(v0.clone(), 0u64)?, f782(v0.clone(), 4u64)?, f782(v0.clone(), 8u64)?, f782(v0.clone(), 12u64)?, f782(v0.clone(), 16u64)?, f782(v0.clone(), 20u64)?, f782(v0.clone(), 24u64)?, f782(v0.clone(), 28u64)?, f782(v0.clone(), 32u64)?, f782(v0.clone(), 36u64)?, f782(v0.clone(), 40u64)?, f782(v0.clone(), 44u64)?, f782(v0.clone(), 48u64)?, f782(v0.clone(), 52u64)?, f782(v0.clone(), 56u64)?, f782(v0, 60u64)?))
}

pub fn f773(v0: Fn6, p3517: u64, p3518: (u64, Adt152)) -> R<(u64, Adt152)> {
    let m3516 = (p3517, p3518);
    {
        let mut m3516 = m3516;
        loop {
            let step = {
                let (v1, v2) = m3516;
                let m3514 = v1;
                if m3514 == 0 {
                    Err::<(u64, (u64, Adt152)), R<(u64, Adt152)>>(Ok(v2))
                } else {
                    let v3 = m3514 - 1;
                    Ok::<(u64, (u64, Adt152)), R<(u64, Adt152)>>((v3, {
                        let c3515: Fn6 = v0.clone();
                        c3515.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3516 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f774(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt152)) -> R<(u64, Adt152)> {
    Ok(({
        let a3520 = {
            let (h3519, _) = v4.clone();
            h3519
        };
        let a3521 = 64u64;
        nat_add(a3520, a3521)?
    }, f762({
        let (_, h3522) = v4;
        h3522
    }, f783(v0, v2, v3, {
        let a3525 = {
            let a3523 = v1;
            let a3524 = v2;
            nat_add(a3523, a3524)?
        };
        let a3526 = 8u64;
        nat_mul(a3525, a3526)?
    }, {
        let (h3527, _) = v4;
        h3527
    })?)?))
}

pub fn f775(v0: Fn7, p3531: u64, p3532: Adt154) -> R<Adt154> {
    let m3530 = (p3531, p3532);
    {
        let mut m3530 = m3530;
        loop {
            let step = {
                let (v1, v2) = m3530;
                let m3528 = v1;
                if m3528 == 0 {
                    Err::<(u64, Adt154), R<Adt154>>(Ok(v2))
                } else {
                    let v3 = m3528 - 1;
                    Ok::<(u64, Adt154), R<Adt154>>((v3, {
                        let c3529: Fn7 = v0.clone();
                        c3529.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f776(v0: Adt154) -> R<Adt154> {
    f784(v0)
}

pub fn f777(v0: u32, v1: u32) -> u32 {
    let m3535 = {
        let a3533 = f785(v0);
        let a3534 = f785(v1);
        fixed_u64::checked_add(a3533, a3534)
    };
    match m3535 {
        None => {
            0u32
        }
        Some(v2) => {
            f786(v2)
        }
    }
}

pub fn f778() -> u32 {
    0u32
}

pub fn f779(v0: Bytes, v1: u64) -> u8 {
    let m3538 = {
        let a3536 = v0;
        let a3537 = v1;
        index_bytes(a3536, a3537)
    };
    match m3538 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f780(v0: u32, v1: u32) -> u32 {
    let m3541 = {
        let a3539 = v0;
        let a3540 = v1;
        fixed_u32::shift_right(a3539, a3540)
    };
    match m3541 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f781(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt152)) -> R<(u64, Adt152)> {
    Ok(({
        let a3543 = {
            let (h3542, _) = v3.clone();
            h3542
        };
        let a3544 = 64u64;
        nat_add(a3543, a3544)?
    }, f762({
        let (_, h3545) = v3;
        h3545
    }, f787(v0, v1, v2, {
        let (h3546, _) = v3;
        h3546
    })?)?))
}

pub fn f782(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f788(f789(v0.clone(), v1), f789(v0.clone(), {
        let a3547 = v1;
        let a3548 = 1u64;
        nat_add(a3547, a3548)?
    }), f789(v0.clone(), {
        let a3549 = v1;
        let a3550 = 2u64;
        nat_add(a3549, a3550)?
    }), f789(v0, {
        let a3551 = v1;
        let a3552 = 3u64;
        nat_add(a3551, a3552)?
    })))
}

pub fn f783(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt153> {
    Ok(Adt153::C0(f790(v0.clone(), v1, v2, v3, v4)?, f790(v0.clone(), v1, v2, v3, {
        let a3553 = v4;
        let a3554 = 4u64;
        nat_add(a3553, a3554)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3555 = v4;
        let a3556 = 8u64;
        nat_add(a3555, a3556)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3557 = v4;
        let a3558 = 12u64;
        nat_add(a3557, a3558)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3559 = v4;
        let a3560 = 16u64;
        nat_add(a3559, a3560)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3561 = v4;
        let a3562 = 20u64;
        nat_add(a3561, a3562)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3563 = v4;
        let a3564 = 24u64;
        nat_add(a3563, a3564)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3565 = v4;
        let a3566 = 28u64;
        nat_add(a3565, a3566)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3567 = v4;
        let a3568 = 32u64;
        nat_add(a3567, a3568)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3569 = v4;
        let a3570 = 36u64;
        nat_add(a3569, a3570)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3571 = v4;
        let a3572 = 40u64;
        nat_add(a3571, a3572)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3573 = v4;
        let a3574 = 44u64;
        nat_add(a3573, a3574)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3575 = v4;
        let a3576 = 48u64;
        nat_add(a3575, a3576)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3577 = v4;
        let a3578 = 52u64;
        nat_add(a3577, a3578)?
    })?, f790(v0.clone(), v1, v2, v3, {
        let a3579 = v4;
        let a3580 = 56u64;
        nat_add(a3579, a3580)?
    })?, f790(v0, v1, v2, v3, {
        let a3581 = v4;
        let a3582 = 60u64;
        nat_add(a3581, a3582)?
    })?))
}

pub fn f784(v0: Adt154) -> R<Adt154> {
    let v1: u32 = f791(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, _, _, h3583) => {
            h3583
        }
    });
    let v2: u32 = f777(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, h3584, _, _) => {
            h3584
        }
    }, f777(f792(match v0.clone() {
        Adt154::C0(_, _, _, _, h3585, _, _, _, _, _) => {
            h3585
        }
    }), f777(f793(match v0.clone() {
        Adt154::C0(_, _, _, _, h3586, _, _, _, _, _) => {
            h3586
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, h3587, _, _, _, _) => {
            h3587
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, h3588, _, _, _) => {
            h3588
        }
    }), f777(v1, {
        let m3591 = match v0.clone() {
            Adt154::C0(_, _, _, _, _, _, _, _, h3590, _) => {
                h3590
            }
        };
        match m3591 {
            Adt153::C0(h3589, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3589
            }
        }
    }))));
    let v3: u32 = f777(f794(match v0.clone() {
        Adt154::C0(h3592, _, _, _, _, _, _, _, _, _) => {
            h3592
        }
    }), f795(match v0.clone() {
        Adt154::C0(h3593, _, _, _, _, _, _, _, _, _) => {
            h3593
        }
    }, match v0.clone() {
        Adt154::C0(_, h3594, _, _, _, _, _, _, _, _) => {
            h3594
        }
    }, match v0.clone() {
        Adt154::C0(_, _, h3595, _, _, _, _, _, _, _) => {
            h3595
        }
    }));
    Ok(Adt154::C0(f777(v2, v3), match v0.clone() {
        Adt154::C0(h3596, _, _, _, _, _, _, _, _, _) => {
            h3596
        }
    }, match v0.clone() {
        Adt154::C0(_, h3597, _, _, _, _, _, _, _, _) => {
            h3597
        }
    }, match v0.clone() {
        Adt154::C0(_, _, h3598, _, _, _, _, _, _, _) => {
            h3598
        }
    }, f777(match v0.clone() {
        Adt154::C0(_, _, _, h3599, _, _, _, _, _, _) => {
            h3599
        }
    }, v2), match v0.clone() {
        Adt154::C0(_, _, _, _, h3600, _, _, _, _, _) => {
            h3600
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, h3601, _, _, _, _) => {
            h3601
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, h3602, _, _, _) => {
            h3602
        }
    }, f796(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, _, h3603, _) => {
            h3603
        }
    }), {
        let a3605 = match v0 {
            Adt154::C0(_, _, _, _, _, _, _, _, _, h3604) => {
                h3604
            }
        };
        let a3606 = 1u64;
        nat_add(a3605, a3606)?
    }))
}

pub fn f785(v0: u32) -> u64 {
    let m3608 = {
        let a3607 = v0;
        fixed_u64::convert(i128::from(a3607))
    };
    match m3608 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f786(v0: u64) -> u32 {
    let m3612 = {
        let a3611 = {
            let a3609 = v0;
            let a3610 = 4294967295u64;
            fixed_u64::bit_and(a3609, a3610)
        };
        fixed_u32::convert(i128::from(a3611))
    };
    match m3612 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f787(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt153> {
    Ok(Adt153::C0(f797(v0.clone(), v1, v2, v3)?, f797(v0.clone(), v1, v2, {
        let a3613 = v3;
        let a3614 = 4u64;
        nat_add(a3613, a3614)?
    })?, f797(v0.clone(), v1, v2, {
        let a3615 = v3;
        let a3616 = 8u64;
        nat_add(a3615, a3616)?
    })?, f797(v0.clone(), v1, v2, {
        let a3617 = v3;
        let a3618 = 12u64;
        nat_add(a3617, a3618)?
    })?, f797(v0.clone(), v1, v2, {
        let a3619 = v3;
        let a3620 = 16u64;
        nat_add(a3619, a3620)?
    })?, f797(v0.clone(), v1, v2, {
        let a3621 = v3;
        let a3622 = 20u64;
        nat_add(a3621, a3622)?
    })?, f797(v0.clone(), v1, v2, {
        let a3623 = v3;
        let a3624 = 24u64;
        nat_add(a3623, a3624)?
    })?, f797(v0.clone(), v1, v2, {
        let a3625 = v3;
        let a3626 = 28u64;
        nat_add(a3625, a3626)?
    })?, f797(v0.clone(), v1, v2, {
        let a3627 = v3;
        let a3628 = 32u64;
        nat_add(a3627, a3628)?
    })?, f797(v0.clone(), v1, v2, {
        let a3629 = v3;
        let a3630 = 36u64;
        nat_add(a3629, a3630)?
    })?, f797(v0.clone(), v1, v2, {
        let a3631 = v3;
        let a3632 = 40u64;
        nat_add(a3631, a3632)?
    })?, f797(v0.clone(), v1, v2, {
        let a3633 = v3;
        let a3634 = 44u64;
        nat_add(a3633, a3634)?
    })?, f797(v0.clone(), v1, v2, {
        let a3635 = v3;
        let a3636 = 48u64;
        nat_add(a3635, a3636)?
    })?, f797(v0.clone(), v1, v2, {
        let a3637 = v3;
        let a3638 = 52u64;
        nat_add(a3637, a3638)?
    })?, f797(v0.clone(), v1, v2, {
        let a3639 = v3;
        let a3640 = 56u64;
        nat_add(a3639, a3640)?
    })?, f797(v0, v1, v2, {
        let a3641 = v3;
        let a3642 = 60u64;
        nat_add(a3641, a3642)?
    })?))
}

pub fn f788(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3647 = f798(v0, 24u32);
    let a3648 = {
        let a3645 = f798(v1, 16u32);
        let a3646 = {
            let a3643 = f798(v2, 8u32);
            let a3644 = v3;
            fixed_u32::bit_or(a3643, a3644)
        };
        fixed_u32::bit_or(a3645, a3646)
    };
    fixed_u32::bit_or(a3647, a3648)
}

pub fn f789(v0: Bytes, v1: u64) -> u32 {
    let m3651 = {
        let a3649 = v0;
        let a3650 = v1;
        index_bytes(a3649, a3650)
    };
    match m3651 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3653 = {
                let a3652 = v2;
                fixed_u32::convert(i128::from(a3652))
            };
            match m3653 {
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

pub fn f790(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3656 = {
        let a3654 = v4;
        let a3655 = 8u64;
        nat_add(a3654, a3655)?
    };
    let a3657 = v2;
    if nat_eq(a3656, a3657) {
        Ok(f799(v3, true))
    } else {
        let a3660 = {
            let a3658 = v4;
            let a3659 = 4u64;
            nat_add(a3658, a3659)?
        };
        let a3661 = v2;
        if nat_eq(a3660, a3661) {
            Ok(f799(v3, false))
        } else {
            Ok(f788(f800(v0.clone(), v1, v4), f800(v0.clone(), v1, {
                let a3662 = v4;
                let a3663 = 1u64;
                nat_add(a3662, a3663)?
            }), f800(v0.clone(), v1, {
                let a3664 = v4;
                let a3665 = 2u64;
                nat_add(a3664, a3665)?
            }), f800(v0, v1, {
                let a3666 = v4;
                let a3667 = 3u64;
                nat_add(a3666, a3667)?
            })))
        }
    }
}

pub fn f791(v0: u64) -> u32 {
    let a3668 = v0;
    let a3669 = 32u64;
    if nat_lt(a3668, a3669) {
        let a3670 = v0;
        let a3671 = 16u64;
        if nat_lt(a3670, a3671) {
            let a3672 = v0;
            let a3673 = 8u64;
            if nat_lt(a3672, a3673) {
                let a3674 = v0;
                let a3675 = 4u64;
                if nat_lt(a3674, a3675) {
                    let a3676 = v0;
                    let a3677 = 2u64;
                    if nat_lt(a3676, a3677) {
                        let a3678 = v0;
                        let a3679 = 1u64;
                        if nat_lt(a3678, a3679) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3680 = v0;
                        let a3681 = 3u64;
                        if nat_lt(a3680, a3681) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3682 = v0;
                    let a3683 = 6u64;
                    if nat_lt(a3682, a3683) {
                        let a3684 = v0;
                        let a3685 = 5u64;
                        if nat_lt(a3684, a3685) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3686 = v0;
                        let a3687 = 7u64;
                        if nat_lt(a3686, a3687) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3688 = v0;
                let a3689 = 12u64;
                if nat_lt(a3688, a3689) {
                    let a3690 = v0;
                    let a3691 = 10u64;
                    if nat_lt(a3690, a3691) {
                        let a3692 = v0;
                        let a3693 = 9u64;
                        if nat_lt(a3692, a3693) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3694 = v0;
                        let a3695 = 11u64;
                        if nat_lt(a3694, a3695) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3696 = v0;
                    let a3697 = 14u64;
                    if nat_lt(a3696, a3697) {
                        let a3698 = v0;
                        let a3699 = 13u64;
                        if nat_lt(a3698, a3699) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3700 = v0;
                        let a3701 = 15u64;
                        if nat_lt(a3700, a3701) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3702 = v0;
            let a3703 = 24u64;
            if nat_lt(a3702, a3703) {
                let a3704 = v0;
                let a3705 = 20u64;
                if nat_lt(a3704, a3705) {
                    let a3706 = v0;
                    let a3707 = 18u64;
                    if nat_lt(a3706, a3707) {
                        let a3708 = v0;
                        let a3709 = 17u64;
                        if nat_lt(a3708, a3709) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3710 = v0;
                        let a3711 = 19u64;
                        if nat_lt(a3710, a3711) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3712 = v0;
                    let a3713 = 22u64;
                    if nat_lt(a3712, a3713) {
                        let a3714 = v0;
                        let a3715 = 21u64;
                        if nat_lt(a3714, a3715) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3716 = v0;
                        let a3717 = 23u64;
                        if nat_lt(a3716, a3717) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3718 = v0;
                let a3719 = 28u64;
                if nat_lt(a3718, a3719) {
                    let a3720 = v0;
                    let a3721 = 26u64;
                    if nat_lt(a3720, a3721) {
                        let a3722 = v0;
                        let a3723 = 25u64;
                        if nat_lt(a3722, a3723) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3724 = v0;
                        let a3725 = 27u64;
                        if nat_lt(a3724, a3725) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3726 = v0;
                    let a3727 = 30u64;
                    if nat_lt(a3726, a3727) {
                        let a3728 = v0;
                        let a3729 = 29u64;
                        if nat_lt(a3728, a3729) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3730 = v0;
                        let a3731 = 31u64;
                        if nat_lt(a3730, a3731) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3732 = v0;
        let a3733 = 48u64;
        if nat_lt(a3732, a3733) {
            let a3734 = v0;
            let a3735 = 40u64;
            if nat_lt(a3734, a3735) {
                let a3736 = v0;
                let a3737 = 36u64;
                if nat_lt(a3736, a3737) {
                    let a3738 = v0;
                    let a3739 = 34u64;
                    if nat_lt(a3738, a3739) {
                        let a3740 = v0;
                        let a3741 = 33u64;
                        if nat_lt(a3740, a3741) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3742 = v0;
                        let a3743 = 35u64;
                        if nat_lt(a3742, a3743) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3744 = v0;
                    let a3745 = 38u64;
                    if nat_lt(a3744, a3745) {
                        let a3746 = v0;
                        let a3747 = 37u64;
                        if nat_lt(a3746, a3747) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3748 = v0;
                        let a3749 = 39u64;
                        if nat_lt(a3748, a3749) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3750 = v0;
                let a3751 = 44u64;
                if nat_lt(a3750, a3751) {
                    let a3752 = v0;
                    let a3753 = 42u64;
                    if nat_lt(a3752, a3753) {
                        let a3754 = v0;
                        let a3755 = 41u64;
                        if nat_lt(a3754, a3755) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3756 = v0;
                        let a3757 = 43u64;
                        if nat_lt(a3756, a3757) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3758 = v0;
                    let a3759 = 46u64;
                    if nat_lt(a3758, a3759) {
                        let a3760 = v0;
                        let a3761 = 45u64;
                        if nat_lt(a3760, a3761) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3762 = v0;
                        let a3763 = 47u64;
                        if nat_lt(a3762, a3763) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3764 = v0;
            let a3765 = 56u64;
            if nat_lt(a3764, a3765) {
                let a3766 = v0;
                let a3767 = 52u64;
                if nat_lt(a3766, a3767) {
                    let a3768 = v0;
                    let a3769 = 50u64;
                    if nat_lt(a3768, a3769) {
                        let a3770 = v0;
                        let a3771 = 49u64;
                        if nat_lt(a3770, a3771) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3772 = v0;
                        let a3773 = 51u64;
                        if nat_lt(a3772, a3773) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3774 = v0;
                    let a3775 = 54u64;
                    if nat_lt(a3774, a3775) {
                        let a3776 = v0;
                        let a3777 = 53u64;
                        if nat_lt(a3776, a3777) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3778 = v0;
                        let a3779 = 55u64;
                        if nat_lt(a3778, a3779) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3780 = v0;
                let a3781 = 60u64;
                if nat_lt(a3780, a3781) {
                    let a3782 = v0;
                    let a3783 = 58u64;
                    if nat_lt(a3782, a3783) {
                        let a3784 = v0;
                        let a3785 = 57u64;
                        if nat_lt(a3784, a3785) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3786 = v0;
                        let a3787 = 59u64;
                        if nat_lt(a3786, a3787) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3788 = v0;
                    let a3789 = 62u64;
                    if nat_lt(a3788, a3789) {
                        let a3790 = v0;
                        let a3791 = 61u64;
                        if nat_lt(a3790, a3791) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3792 = v0;
                        let a3793 = 63u64;
                        if nat_lt(a3792, a3793) {
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

pub fn f792(v0: u32) -> u32 {
    let a3796 = {
        let a3794 = f801(v0, 6u32, 26u32);
        let a3795 = f801(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3794, a3795)
    };
    let a3797 = f801(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3796, a3797)
}

pub fn f793(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3803 = {
        let a3798 = v0;
        let a3799 = v1;
        fixed_u32::bit_and(a3798, a3799)
    };
    let a3804 = {
        let a3801 = {
            let a3800 = v0;
            fixed_u32::bit_not(a3800)
        };
        let a3802 = v2;
        fixed_u32::bit_and(a3801, a3802)
    };
    fixed_u32::bit_xor(a3803, a3804)
}

pub fn f794(v0: u32) -> u32 {
    let a3807 = {
        let a3805 = f801(v0, 2u32, 30u32);
        let a3806 = f801(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3805, a3806)
    };
    let a3808 = f801(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3807, a3808)
}

pub fn f795(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3817 = {
        let a3813 = {
            let a3809 = v0;
            let a3810 = v1;
            fixed_u32::bit_and(a3809, a3810)
        };
        let a3814 = {
            let a3811 = v0;
            let a3812 = v2;
            fixed_u32::bit_and(a3811, a3812)
        };
        fixed_u32::bit_xor(a3813, a3814)
    };
    let a3818 = {
        let a3815 = v1;
        let a3816 = v2;
        fixed_u32::bit_and(a3815, a3816)
    };
    fixed_u32::bit_xor(a3817, a3818)
}

pub fn f796(v0: Adt153) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt153::C0(_, h3819, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3819
        }
    }, match v0.clone() {
        Adt153::C0(_, _, h3820, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3820
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, h3821, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3821
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, h3822, _, _, _, _, _, _, _, _, _, _, _) => {
            h3822
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, h3823, _, _, _, _, _, _, _, _, _, _) => {
            h3823
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, h3824, _, _, _, _, _, _, _, _, _) => {
            h3824
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, h3825, _, _, _, _, _, _, _, _) => {
            h3825
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, h3826, _, _, _, _, _, _, _) => {
            h3826
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, h3827, _, _, _, _, _, _) => {
            h3827
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, h3828, _, _, _, _, _) => {
            h3828
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, h3829, _, _, _, _) => {
            h3829
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3830, _, _, _) => {
            h3830
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3831, _, _) => {
            h3831
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3832, _) => {
            h3832
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3833) => {
            h3833
        }
    }, f777(f802(match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3834, _) => {
            h3834
        }
    }), f777(match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, h3835, _, _, _, _, _, _) => {
            h3835
        }
    }, f777(f803(match v0.clone() {
        Adt153::C0(_, h3836, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3836
        }
    }), match v0 {
        Adt153::C0(h3837, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3837
        }
    }))))
}

pub fn f797(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a3840 = {
        let a3838 = v3;
        let a3839 = 8u64;
        nat_add(a3838, a3839)?
    };
    let a3841 = v2;
    if nat_eq(a3840, a3841) {
        Ok(f804({
            let a3844 = {
                let a3842 = v1;
                let a3843 = 8u64;
                nat_mul(a3842, a3843)?
            };
            let a3845 = 4294967296u64;
            let a3846 = 0u64;
            nat_quot(a3844, a3845, a3846)
        }))
    } else {
        let a3849 = {
            let a3847 = v3;
            let a3848 = 4u64;
            nat_add(a3847, a3848)?
        };
        let a3850 = v2;
        if nat_eq(a3849, a3850) {
            Ok(f804({
                let a3853 = {
                    let a3851 = v1;
                    let a3852 = 8u64;
                    nat_mul(a3851, a3852)?
                };
                let a3854 = 4294967296u64;
                let a3855 = 0u64;
                nat_rem(a3853, a3854, a3855)
            }))
        } else {
            Ok(f788(f800(v0.clone(), v1, v3), f800(v0.clone(), v1, {
                let a3856 = v3;
                let a3857 = 1u64;
                nat_add(a3856, a3857)?
            }), f800(v0.clone(), v1, {
                let a3858 = v3;
                let a3859 = 2u64;
                nat_add(a3858, a3859)?
            }), f800(v0, v1, {
                let a3860 = v3;
                let a3861 = 3u64;
                nat_add(a3860, a3861)?
            })))
        }
    }
}

pub fn f798(v0: u32, v1: u32) -> u32 {
    let m3864 = {
        let a3862 = v0;
        let a3863 = v1;
        fixed_u32::shift_left(a3862, a3863)
    };
    match m3864 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f799(v0: u64, v1: bool) -> u32 {
    if v1 {
        f804({
            let a3865 = v0;
            let a3866 = 4294967296u64;
            let a3867 = 0u64;
            nat_quot(a3865, a3866, a3867)
        })
    } else {
        f804({
            let a3868 = v0;
            let a3869 = 4294967296u64;
            let a3870 = 0u64;
            nat_rem(a3868, a3869, a3870)
        })
    }
}

pub fn f800(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3871 = v2;
    let a3872 = v1;
    if nat_lt(a3871, a3872) {
        let m3875 = {
            let a3873 = v0;
            let a3874 = v2;
            index_bytes(a3873, a3874)
        };
        match m3875 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3877 = {
                    let a3876 = v3;
                    fixed_u32::convert(i128::from(a3876))
                };
                match m3877 {
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
        let a3878 = v2;
        let a3879 = v1;
        if nat_eq(a3878, a3879) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f801(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3880 = f780(v0, v1);
    let a3881 = f798(v0, v2);
    fixed_u32::bit_or(a3880, a3881)
}

pub fn f802(v0: u32) -> u32 {
    let a3884 = {
        let a3882 = f801(v0, 17u32, 15u32);
        let a3883 = f801(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3882, a3883)
    };
    let a3885 = f780(v0, 10u32);
    fixed_u32::bit_xor(a3884, a3885)
}

pub fn f803(v0: u32) -> u32 {
    let a3888 = {
        let a3886 = f801(v0, 7u32, 25u32);
        let a3887 = f801(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3886, a3887)
    };
    let a3889 = f780(v0, 3u32);
    fixed_u32::bit_xor(a3888, a3889)
}

pub fn f804(v0: u64) -> u32 {
    let (_, h3890) = {
        let (_, h3891) = f805(Fn8::F806, 32u64, (v0, (1u32, 0u32)));
        h3891
    };
    h3890
}

pub fn f805(v0: Fn8, p3895: u64, p3896: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3894 = (p3895, p3896);
    {
        let mut m3894 = m3894;
        loop {
            let step = {
                let (v1, v2) = m3894;
                let m3892 = v1;
                if m3892 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3892 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3893: Fn8 = v0.clone();
                        c3893.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3894 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f806(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3897, _) = v0;
        h3897
    };
    let v2: u32 = {
        let (h3898, _) = {
            let (_, h3899) = v0;
            h3899
        };
        h3898
    };
    let v3: u32 = {
        let (_, h3900) = {
            let (_, h3901) = v0;
            h3901
        };
        h3900
    };
    ({
        let a3902 = v1;
        let a3903 = 2u64;
        let a3904 = 0u64;
        nat_quot(a3902, a3903, a3904)
    }, (f798(v2, 1u32), {
        let a3908 = {
            let a3905 = v1;
            let a3906 = 2u64;
            let a3907 = 0u64;
            nat_rem(a3905, a3906, a3907)
        };
        let a3909 = 1u64;
        if nat_eq(a3908, a3909) {
            let a3910 = v3;
            let a3911 = v2;
            fixed_u32::bit_or(a3910, a3911)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
