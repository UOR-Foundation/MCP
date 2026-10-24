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
    F742(Bytes),
    F767(Bytes, Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, u64)) -> R<(u64, u64)> {
        match self {
            Fn3::F742(k0) => f742(k0.clone(), p0),
            Fn3::F767(k0, k1) => f767(k0.clone(), k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F747(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<(u64, Bytes)> {
        match self {
            Fn4::F747(k0) => f747(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F752(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
        match self {
            Fn5::F752(k0) => f752(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F775(Bytes, u64, u64, u64),
    F782(Bytes, u64, u64),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, Adt152)) -> R<(u64, Adt152)> {
        match self {
            Fn6::F775(k0, k1, k2, k3) => f775(k0.clone(), *k1, *k2, *k3, p0),
            Fn6::F782(k0, k1, k2) => f782(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F777,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt154) -> R<Adt154> {
        match self {
            Fn7::F777 => f777(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F807,
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn8::F807 => f807(p0),
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
                    let m252 = f86(v0.clone(), {
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
                            f87(v0, Adt85::C0(f85(match v1 {
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
    let m277 = f88(v1)?;
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
            f89()
        }
        Adt90::C1 => {
            f89()
        }
    }
}

pub fn f64(v0: Option<Adt92>, v1: Adt80) -> Adt6 {
    Adt6::C5(f90(v0, v1))
}

pub fn f65(v0: Adt90) -> Adt80 {
    Adt80::C0(f91(), f92(v0), None::<Adt6>)
}

pub fn f66(v0: List<u64>) -> bool {
    f93(v0, f94())
}

pub fn f67(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt93>) -> R<Adt88> {
    let m279 = v4;
    match m279 {
        Ok(v6) => {
            f97(v0, v1, v2, v3, f98(v6)?)
        }
        Err(_) => {
            f62(f95(), f64(None::<Adt92>, f96(Adt94::C0)))
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
    f99(f85(v0)?)
}

pub fn f70() -> Adt88 {
    Adt88::C0(f100(), List::cons(Adt86::C0(f101(), f102()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f71(v0: List<Adt86>, v1: List<u64>) -> R<Adt96> {
    let m286 = f103(v0, v1, Adt97::C0)?;
    match m286 {
        Adt97::C0 => {
            Ok(Adt96::C0)
        }
        Adt97::C1(v2) => {
            let m287 = f104(v2);
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
            if f105(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f74(v0: List<u64>) -> Option<List<u64>> {
    let m289 = f106(v0);
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
                f107(f108(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f107(f109(v0, List::<u64>::nil()))
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
                        if f110(match v2.clone() {
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
    let m306 = f111(match v0.clone() {
        Adt35::C0(h304, _, _, _, _, _, _, _) => {
            h304
        }
    }, f112(match v0.clone() {
        Adt35::C0(h305, _, _, _, _, _, _, _) => {
            h305
        }
    })?);
    match m306 {
        None => {
            f113(v0)
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
                    Ok::<(u64, i64), R<i64>>((v2, f114(v1, 1i64)?))
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
    let m317 = f115(Adt6::C5({
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
    }), f116(v1.clone()))?;
    match m317 {
        Adt100::C0 => {
            let c318: Fn2 = v2;
            c318.apply(v1)
        }
        Adt100::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f117(f118(v3, v4)?, true))))
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
    f119(v1, v0, v2)
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
    f120(v0.clone(), {
        let a323 = v0;
        length_bytes(a323)
    }, List::<u64>::nil())
}

pub fn f86(v0: Bytes, v1: u64) -> R<Option<Adt89>> {
    let a325 = v1;
    let a326 = {
        let a324 = v0.clone();
        length_bytes(a324)
    };
    if nat_eq(a325, a326) {
        Ok(Some(Adt89::C0(Bytes::lit(&[]), v1)))
    } else {
        f16(v0, v1)
    }
}

pub fn f87(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a328 = v2;
    let a329 = {
        let a327 = v0.clone();
        length_bytes(a327)
    };
    if nat_eq(a328, a329) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m330 = f33(v0.clone(), v2)?;
        match m330 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m333 = f121(v0.clone(), {
                    let a331 = v2;
                    let a332 = 4u64;
                    nat_add(a331, a332)?
                }, v3, List::<List<u64>>::nil())?;
                match m333 {
                    Ok(v5) => {
                        let a336 = {
                            let (_, h334) = v5.clone();
                            h334
                        };
                        let a337 = {
                            let a335 = v0;
                            length_bytes(a335)
                        };
                        if nat_eq(a336, a337) {
                            Ok(Ok::<Adt85, Adt87>(f122(v1, {
                                let (h338, _) = v5;
                                h338
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h339) = v5;
                                h339
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

pub fn f88(v0: Adt6) -> R<Result<List<u64>, Adt93>> {
    f123(f124(), v0)
}

pub fn f89() -> u64 {
    403u64
}

pub fn f90(v0: Option<Adt92>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), {
        let a340 = f126(v0);
        let a341 = List::cons((Str::lit("error"), f127(v1)), List::<(Str, Adt6)>::nil());
        append_list(a340, a341)
    })
}

pub fn f91() -> i64 {
    -32000i64
}

pub fn f92(v0: Adt90) -> Str {
    let m342 = v0;
    match m342 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f93(p349: List<u64>, p350: List<u64>) -> bool {
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

pub fn f94() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f95() -> u64 {
    400u64
}

pub fn f96(v0: Adt94) -> Adt80 {
    Adt80::C0(f128(v0.clone()), f129(v0), None::<Adt6>)
}

pub fn f97(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt98, Adt94>) -> R<Adt88> {
    let m351 = v4;
    match m351 {
        Ok(v6) => {
            f132(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f62(f130(v5.clone()), f64(f131(v5.clone()), f96(v5)))
        }
    }
}

pub fn f98(v0: Adt6) -> R<Result<Adt98, Adt94>> {
    let m352 = v0;
    match m352 {
        Adt6::C0 => {
            Ok(f134(None::<Adt92>, Adt95::C0))
        }
        Adt6::C1(_) => {
            Ok(f134(None::<Adt92>, Adt95::C0))
        }
        Adt6::C2(_) => {
            Ok(f134(None::<Adt92>, Adt95::C0))
        }
        Adt6::C3(_) => {
            Ok(f134(None::<Adt92>, Adt95::C0))
        }
        Adt6::C4(_) => {
            Ok(f134(None::<Adt92>, Adt95::C1))
        }
        Adt6::C5(v1) => {
            f133(Adt5::C0(v1))
        }
    }
}

pub fn f99(v0: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m353 = f135(v0.clone())?;
    match m353 {
        Adt111::C0 => {
            f136(f137(), 0u64, v0)
        }
        Adt111::C1(v1) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C0(v1)))
        }
        Adt111::C2(v2) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C1(v2)))
        }
    }
}

pub fn f100() -> u64 {
    405u64
}

pub fn f101() -> List<u64> {
    List::cons(65u64, List::cons(108u64, List::cons(108u64, List::cons(111u64, List::cons(119u64, List::<u64>::nil())))))
}

pub fn f102() -> List<u64> {
    List::cons(80u64, List::cons(79u64, List::cons(83u64, List::cons(84u64, List::<u64>::nil()))))
}

pub fn f103(p358: List<Adt86>, v1: List<u64>, p359: Adt97) -> R<Adt97> {
    let m357 = (p358, p359);
    {
        let mut m357 = m357;
        loop {
            let step = {
                let (v0, v2) = m357;
                let m354 = v0;
                match m354.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt97), R<Adt97>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f138(match v3.clone() {
                            Adt86::C0(h355, _) => {
                                h355
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, f139(v2, match v3 {
                                Adt86::C0(_, h356) => {
                                    h356
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, v2))
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

pub fn f104(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f140(v0);
    if f141(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f105(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
    if f138(v0.clone(), f142())? {
        Ok(true)
    } else {
        if f138(v0.clone(), f143())? {
            Ok(true)
        } else {
            if f138(v0.clone(), f144())? {
                Ok(true)
            } else {
                f145(v0, v1)
            }
        }
    }
}

pub fn f106(p364: List<u64>) -> Option<List<u64>> {
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
                            Err::<List<u64>, Option<List<u64>>>(f146(v2))
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

pub fn f107(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m365 = v0;
    match m365 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f147({
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

pub fn f108(p372: List<u64>, p373: List<u64>) -> Option<(List<u64>, List<u64>)> {
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
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(List::cons(v2, v1), List::<u64>::nil()), v3)))
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

pub fn f109(p378: List<u64>, p379: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m377 = (p378, p379);
    {
        let mut m377 = m377;
        loop {
            let step = {
                let (v0, v1) = m377;
                let m374 = v0;
                match m374.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a375 = v2;
                        let a376 = 58u64;
                        if nat_eq(a375, a376) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(v1, List::<u64>::nil()), List::cons(v2, v3))))
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

pub fn f110(v0: Str, p384: List<Str>) -> bool {
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

pub fn f111(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f149(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f112(v0: Str) -> R<List<u64>> {
    f85({
        let a385 = v0;
        utf8_encode(a385)
    })
}

pub fn f113(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m386 = f150(v0.clone());
    match m386 {
        None => {
            f151(v0.clone(), f152(f153(), match v0.clone() {
                Adt35::C0(h387, _, _, _, _, _, _, _) => {
                    h387
                }
            }, Adt6::C5({
                let m390 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h389, _, _, _) => {
                        h389
                    }
                };
                match m390 {
                    Adt5::C0(h388) => {
                        h388
                    }
                }
            }), List::cons(Adt99::C0(Adt6::C5({
                let m393 = match v0 {
                    Adt35::C0(_, _, _, _, h392, _, _, _) => {
                        h392
                    }
                };
                match m393 {
                    Adt5::C0(h391) => {
                        h391
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt99>::nil()), f154())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f114(v0: i64, v1: i64) -> R<i64> {
    let a395 = v0;
    let a396 = {
        let a394 = v1;
        int_neg(a394)?
    };
    int_sub(a395, a396)
}

pub fn f115(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt100> {
    f155(v0, f156(), Adt102::C0, List::cons(Adt103::C0(v1, v2, List::<Adt101>::nil()), List::<Adt103>::nil()))
}

pub fn f116(v0: Adt21) -> Adt6 {
    let m398 = match v0 {
        Adt21::C0(_, _, h397, _, _) => {
            h397
        }
    };
    match m398 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h399) => {
                    h399
                }
            })
        }
    }
}

pub fn f117(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f118(v0: List<Adt101>, v1: Str) -> R<Str> {
    let v2: Str = f157(v0)?;
    let a402 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a400 = v2.clone();
        let a401 = Str::lit("");
        if equal(a400, a401) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a403 = Str::lit(" ");
    Ok(join(a402, a403))
}

pub fn f119(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m405 = {
        let c404: Fn1 = v1;
        c404.apply(v2)
    };
    match m405 {
        Ok(v4) => {
            f158(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f120(v0: Bytes, p408: u64, p409: List<u64>) -> R<List<u64>> {
    let m407 = (p408, p409);
    {
        let mut m407 = m407;
        loop {
            let step = {
                let (v1, v2) = m407;
                let m406 = v1;
                if m406 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m406 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f57(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m407 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, p415: u64, p416: u64, p417: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m414 = (p415, (p416, p417));
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v1, (v2, v3)) = m414;
                let m410 = v2;
                if m410 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f38(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m410 - 1;
                    let m411 = f16(v0.clone(), v1)?;
                    match m411 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h412) => {
                                    h412
                                }
                            }, (v4, List::cons(f85(match v5 {
                                Adt89::C0(h413, _) => {
                                    h413
                                }
                            })?, v3))))
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

pub fn f122(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h418, _, _, _, _, _, _) => {
            h418
        }
    }, match v0.clone() {
        Adt85::C0(_, h419, _, _, _, _, _) => {
            h419
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h420, _, _, _, _) => {
            h420
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h421, _, _, _) => {
            h421
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, _, h422, _, _) => {
            h422
        }
    }, v1, match v0 {
        Adt85::C0(_, _, _, _, _, _, h423) => {
            h423
        }
    })
}

pub fn f123(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt93>> {
    let m424 = f159(v0, List::cons(Adt105::C1(0u64, v1), List::<Adt105>::nil()), List::<u64>::nil())?;
    match m424 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt93>(f160(v2, List::<u64>::nil())))
        }
        Err(v3) => {
            Ok(Err::<List<u64>, Adt93>(v3))
        }
    }
}

pub fn f124() -> u64 {
    67108864u64
}

pub fn f125() -> Str {
    Str::lit("2.0")
}

pub fn f126(v0: Option<Adt92>) -> List<(Str, Adt6)> {
    let m425 = v0;
    match m425 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f161(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f127(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h426, _, _) => {
            h426
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h427, _) => {
            h427
        }
    })), f162(match v0 {
        Adt80::C0(_, _, h428) => {
            h428
        }
    }))))
}

pub fn f128(v0: Adt94) -> i64 {
    let m429 = v0;
    match m429 {
        Adt94::C0 => {
            f163()
        }
        Adt94::C1(_, _) => {
            f164()
        }
    }
}

pub fn f129(v0: Adt94) -> Str {
    let m430 = v0;
    match m430 {
        Adt94::C0 => {
            Str::lit("Parse error")
        }
        Adt94::C1(_, v2) => {
            f165(v2)
        }
    }
}

pub fn f130(v0: Adt94) -> u64 {
    let m431 = v0;
    match m431 {
        Adt94::C0 => {
            f95()
        }
        Adt94::C1(_, _) => {
            f95()
        }
    }
}

pub fn f131(v0: Adt94) -> Option<Adt92> {
    let m432 = v0;
    match m432 {
        Adt94::C0 => {
            None::<Adt92>
        }
        Adt94::C1(v1, _) => {
            v1
        }
    }
}

pub fn f132(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt98) -> R<Adt88> {
    f166(f167(v3.clone())?, f168(v0.clone(), v1, v2, Some(f169(v3, match v0 {
        Adt0::C0(_, _, _, _, h433, _, _) => {
            h433
        }
    }, f170(v4.clone()), match v4.clone() {
        Adt98::C0(_, h434, _) => {
            h434
        }
    })?), f171(), v4)?)
}

pub fn f133(v0: Adt5) -> R<Result<Adt98, Adt94>> {
    let m435 = f172(v0.clone())?;
    match m435 {
        Adt128::C0 => {
            let m436 = f173(Str::lit("method"), v0.clone());
            match m436 {
                Adt129::C0 => {
                    Ok(f134(None::<Adt92>, f174(v0)))
                }
                Adt129::C1(_) => {
                    Ok(f134(None::<Adt92>, if f175(v0) {
                        Adt95::C8
                    } else {
                        Adt95::C7
                    }))
                }
            }
        }
        Adt128::C1(v2) => {
            Ok(f176(v2, v0))
        }
        Adt128::C2 => {
            Ok(f134(None::<Adt92>, Adt95::C5))
        }
    }
}

pub fn f134(v0: Option<Adt92>, v1: Adt95) -> Result<Adt98, Adt94> {
    Err::<Adt98, Adt94>(Adt94::C1(v0, v1))
}

pub fn f135(v0: List<u64>) -> R<Adt111> {
    Ok(f177({
        let a437 = v0.clone();
        length_list(a437)
    }, f178(Adt112::C0, 0u64, 0u64, v0)?))
}

pub fn f136(p444: Adt113, p445: u64, p446: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m443 = (p444, (p445, p446));
    {
        let mut m443 = m443;
        loop {
            let step = {
                let (v0, (v1, v2)) = m443;
                let m438 = v2;
                match m438.uncons() {
                    None => {
                        let m439 = f179(v0, v1)?;
                        match m439 {
                            Ok(v3) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Ok::<Adt6, Adt93>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m440 = f180(v0, v1, v5)?;
                        match m440 {
                            Adt130::C0(v7) => {
                                Err::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v7)))
                            }
                            Adt130::C1(v8) => {
                                Ok::<(Adt113, (u64, List<u64>)), R<Result<Adt6, Adt93>>>((v8, ({
                                    let a441 = v1;
                                    let a442 = 1u64;
                                    nat_add(a441, a442)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m443 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137() -> Adt113 {
    Adt113::C0(Adt114::C0, List::<Adt118>::nil(), 0u64)
}

pub fn f138(p453: List<u64>, p454: List<u64>) -> R<bool> {
    let m452 = (p453, p454);
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v0, v1) = m452;
                let m447 = v0;
                match m447.uncons() {
                    None => {
                        let m448 = v1;
                        match m448.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m449 = v1;
                        match m449.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a450 = f181(v4)?;
                                let a451 = f181(v6)?;
                                if nat_eq(a450, a451) {
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
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Adt97, v1: List<u64>) -> Adt97 {
    let m455 = v0;
    match m455 {
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

pub fn f140(v0: List<u64>) -> List<u64> {
    f148(f182(f148(f182(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f141(p458: List<u64>) -> bool {
    let m457 = p458;
    {
        let mut m457 = m457;
        loop {
            let step = {
                let v0 = m457;
                let m456 = v0;
                match m456.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f183(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f142() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f143() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f144() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f145(v0: List<u64>, p461: List<List<u64>>) -> R<bool> {
    let m460 = p461;
    {
        let mut m460 = m460;
        loop {
            let step = {
                let v1 = m460;
                let m459 = v1;
                match m459.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f138(v0.clone(), v2)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v3)
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

pub fn f146(v0: List<u64>) -> Option<List<u64>> {
    let m462 = v0;
    match m462.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m463 = v2;
            match m463.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a464 = v1;
                    let a465 = 47u64;
                    if if nat_eq(a464, a465) {
                        let a466 = v3;
                        let a467 = 47u64;
                        nat_eq(a466, a467)
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

pub fn f147(v0: List<u64>) -> bool {
    let m468 = v0;
    match m468.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a469 = v1;
            let a470 = 58u64;
            if nat_eq(a469, a470) {
                let m471 = v2.clone();
                match m471.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f184(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f148(p474: List<u64>, p475: List<u64>) -> List<u64> {
    let m473 = (p474, p475);
    {
        let mut m473 = m473;
        loop {
            let step = {
                let (v0, v1) = m473;
                let m472 = v0;
                match m472.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f149(v0: List<u64>) -> bool {
    let m476 = v0.clone();
    match m476.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a480 = if f185(v0.clone(), false) {
                let a478 = {
                    let a477 = v0;
                    length_list(a477)
                };
                let a479 = f186();
                nat_le(a478, a479)
            } else {
                false
            };
            bool_not(a480)
        }
    }
}

pub fn f150(v0: Adt35) -> Option<Adt91> {
    let m485 = f187(match v0.clone() {
        Adt35::C0(h481, _, _, _, _, _, _, _) => {
            h481
        }
    }, {
        let m484 = match v0.clone() {
            Adt35::C0(_, _, _, _, h483, _, _, _) => {
                h483
            }
        };
        match m484 {
            Adt5::C0(h482) => {
                h482
            }
        }
    });
    match m485 {
        None => {
            f188(match v0.clone() {
                Adt35::C0(h486, _, _, _, _, _, _, _) => {
                    h486
                }
            }, {
                let m489 = match v0 {
                    Adt35::C0(_, _, _, _, h488, _, _, _) => {
                        h488
                    }
                };
                match m489 {
                    Adt5::C0(h487) => {
                        h487
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f151(v0: Adt35, v1: Result<Adt119, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m490 = v1;
    match m490 {
        Ok(v3) => {
            let m491 = f189(v0)?;
            match m491 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f78(match v3 {
                        Adt119::C0(h492, _) => {
                            h492
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

pub fn f152(p499: u64, v1: Str, v2: Adt6, p500: List<Adt99>, p501: Adt119) -> R<Result<Adt119, Adt91>> {
    let m498 = (p499, (p500, p501));
    {
        let mut m498 = m498;
        loop {
            let step = {
                let (v0, (v3, v4)) = m498;
                let m493 = v0;
                if m493 == 0 {
                    Err::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>(Ok(Err::<Adt119, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m493 - 1;
                    let m494 = v3;
                    match m494.uncons() {
                        None => {
                            Err::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>(Ok(Ok::<Adt119, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m495 = f190(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m495 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt99>, Adt119)), R<Result<Adt119, Adt91>>>((v5, ({
                                        let (h496, _) = v9;
                                        h496
                                    }, {
                                        let (_, h497) = v9;
                                        h497
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
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153() -> u64 {
    67108864u64
}

pub fn f154() -> Adt119 {
    Adt119::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f155(v0: Adt6, p520: u64, p521: Adt102, p522: List<Adt103>) -> R<Adt100> {
    let m519 = (p520, (p521, p522));
    {
        let mut m519 = m519;
        loop {
            let step = {
                let (v1, (v2, v3)) = m519;
                let m502 = v1;
                if m502 == 0 {
                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C4))
                } else {
                    let v4 = m502 - 1;
                    let m503 = v3;
                    match m503.uncons() {
                        None => {
                            let m504 = v2;
                            match m504 {
                                Adt102::C0 => {
                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C0))
                                }
                                Adt102::C1(v5, v6) => {
                                    Err::<(u64, (Adt102, List<Adt103>)), R<Adt100>>(Ok(Adt100::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m505 = v2.clone();
                            match m505 {
                                Adt102::C0 => {
                                    let m506 = v7.clone();
                                    match m506 {
                                        Adt103::C0(v9, v10, v11) => {
                                            let m507 = f191(v0.clone(), v9, v10, v11.clone())?;
                                            match m507 {
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
                                                        let a508 = v15;
                                                        let a509 = v8;
                                                        append_list(a508, a509)
                                                    };
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt103::C1(_, _, _) => {
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v8)))
                                        }
                                        Adt103::C2(_, _, _, _) => {
                                            let v24: Adt120 = f192(v7, v8)?;
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v24.clone() {
                                                Adt120::C0(h510, _) => {
                                                    h510
                                                }
                                            }, match v24 {
                                                Adt120::C0(_, h511) => {
                                                    h511
                                                }
                                            })))
                                        }
                                        Adt103::C3(v25) => {
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (Adt102::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt103::C4(_, _, _, _) => {
                                            let v39: Adt120 = f192(v7, v8)?;
                                            Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v39.clone() {
                                                Adt120::C0(h515, _) => {
                                                    h515
                                                }
                                            }, match v39 {
                                                Adt120::C0(_, h516) => {
                                                    h516
                                                }
                                            })))
                                        }
                                        Adt103::C5(v26, v27, v28, v29) => {
                                            let m512 = f193(v26, v27, v28, v29.clone());
                                            match m512 {
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
                                                        let a513 = v33;
                                                        let a514 = v8;
                                                        append_list(a513, a514)
                                                    };
                                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt102::C1(v40, v41) => {
                                    let v42: Adt120 = f194(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt102, List<Adt103>)), R<Adt100>>((v4, (match v42.clone() {
                                        Adt120::C0(h517, _) => {
                                            h517
                                        }
                                    }, match v42 {
                                        Adt120::C0(_, h518) => {
                                            h518
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156() -> u64 {
    67108864u64
}

pub fn f157(v0: List<Adt101>) -> R<Str> {
    let a523 = List::cons(Str::lit(""), f195(v0, List::<Str>::nil())?);
    let a524 = Str::lit("/");
    Ok(join(a523, a524))
}

pub fn f158(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f196({
        let m527 = match v1.clone() {
            Adt49::C0(h526, _) => {
                h526
            }
        };
        match m527 {
            Adt37::C0(h525, _, _) => {
                h525
            }
        }
    })? {
        f197(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f159(p532: u64, p533: List<Adt105>, p534: List<u64>) -> R<Result<List<u64>, Adt93>> {
    let m531 = (p532, (p533, p534));
    {
        let mut m531 = m531;
        loop {
            let step = {
                let (v0, (v1, v2)) = m531;
                let m528 = v0;
                if m528 == 0 {
                    Err::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(Adt93::C13)))
                } else {
                    let v3 = m528 - 1;
                    let m529 = v1;
                    match m529.uncons() {
                        None => {
                            Err::<(u64, (List<Adt105>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Ok::<List<u64>, Adt93>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m530 = f198(v4, v5, v2)?;
                            match m530 {
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
                Ok(next) => { m531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(p537: List<u64>, p538: List<u64>) -> List<u64> {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v0, v1) = m536;
                let m535 = v0;
                match m535.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Adt92) -> Adt6 {
    let m539 = v0;
    match m539 {
        Adt92::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt92::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f162(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m540 = v0;
    match m540 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f163() -> i64 {
    -32700i64
}

pub fn f164() -> i64 {
    -32600i64
}

pub fn f165(v0: Adt95) -> Str {
    let m541 = v0;
    match m541 {
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

pub fn f166(v0: bool, v1: Adt121) -> R<Adt88> {
    let m543 = match v1.clone() {
        Adt121::C0(_, h542) => {
            h542
        }
    };
    match m543 {
        Ok(v2) => {
            let m544 = f199(v2.clone())?;
            match m544 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f200(v0, f62(f201(), f202(match v1 {
                        Adt121::C0(h545, _) => {
                            h545
                        }
                    }, v3))?, f203(match v2 {
                        Adt122::C0(_, _, h546) => {
                            h546
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f62(f204(v4.clone()), f64(Some(match v1 {
                Adt121::C0(h547, _) => {
                    h547
                }
            }), f205(v4)?))
        }
    }
}

pub fn f167(p552: List<Adt86>) -> R<bool> {
    let m551 = p552;
    {
        let mut m551 = m551;
        loop {
            let step = {
                let v0 = m551;
                let m548 = v0;
                match m548.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f138(match v1.clone() {
                            Adt86::C0(h549, _) => {
                                h549
                            }
                        }, f206())? {
                            if f207(f208(44u64, match v1 {
                                Adt86::C0(_, h550) => {
                                    h550
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
                Ok(next) => { m551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt106>, v4: List<Str>, v5: Adt98) -> R<Adt121> {
    Ok(Adt121::C0(match v5.clone() {
        Adt98::C0(h553, _, _) => {
            h553
        }
    }, f209(v0, v1, v2, f210(v3, v4, v5)?)?))
}

pub fn f169(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt106> {
    let v4: Adt127 = f211(Adt108::C0, f71(v0.clone(), f212())?);
    let v5: Adt127 = f211(Adt108::C1, f71(v0.clone(), f213())?);
    let v6: Adt127 = if f214(v3) {
        f211(Adt108::C2, f215(f71(v0.clone(), f216())?)?)
    } else {
        f217()
    };
    Ok(Adt106::C0(match v4.clone() {
        Adt127::C0(h554, _) => {
            h554
        }
    }, match v5.clone() {
        Adt127::C0(h555, _) => {
            h555
        }
    }, match v6.clone() {
        Adt127::C0(h556, _) => {
            h556
        }
    }, f218(match v4 {
        Adt127::C0(_, h557) => {
            h557
        }
    }, f218(match v5 {
        Adt127::C0(_, h558) => {
            h558
        }
    }, match v6 {
        Adt127::C0(_, h559) => {
            h559
        }
    })), f219(v0, v1, v2)?))
}

pub fn f170(v0: Adt98) -> Option<Str> {
    let a561 = match v0.clone() {
        Adt98::C0(_, h560, _) => {
            h560
        }
    };
    let a562 = Str::lit("tools/call");
    if equal(a561, a562) {
        let m564 = match v0 {
            Adt98::C0(_, _, h563) => {
                h563
            }
        };
        match m564 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m565 = f173(Str::lit("name"), v1);
                match m565 {
                    Adt129::C0 => {
                        None::<Str>
                    }
                    Adt129::C1(v2) => {
                        f220(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f171() -> List<Str> {
    List::cons(f221(), List::<Str>::nil())
}

pub fn f172(v0: Adt5) -> R<Adt128> {
    let m566 = f173(Str::lit("id"), v0);
    match m566 {
        Adt129::C0 => {
            Ok(Adt128::C0)
        }
        Adt129::C1(v1) => {
            let m567 = v1;
            match m567 {
                Adt6::C0 => {
                    Ok(Adt128::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt128::C2)
                }
                Adt6::C2(v3) => {
                    let m568 = f222(v3)?;
                    match m568 {
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

pub fn f173(v0: Str, v1: Adt5) -> Adt129 {
    f223(v0, match v1 {
        Adt5::C0(h569) => {
            h569
        }
    })
}

pub fn f174(v0: Adt5) -> Adt95 {
    if f175(v0) {
        Adt95::C8
    } else {
        Adt95::C3
    }
}

pub fn f175(v0: Adt5) -> bool {
    if f224(Str::lit("result"), v0.clone()) {
        true
    } else {
        f224(Str::lit("error"), v0)
    }
}

pub fn f176(v0: Adt92, v1: Adt5) -> Result<Adt98, Adt94> {
    let m570 = f173(Str::lit("jsonrpc"), v1.clone());
    match m570 {
        Adt129::C0 => {
            f134(Some(v0), Adt95::C2)
        }
        Adt129::C1(v2) => {
            let m571 = f220(v2);
            match m571 {
                None => {
                    f134(Some(v0), Adt95::C2)
                }
                Some(v3) => {
                    let a572 = v3;
                    let a573 = f125();
                    if equal(a572, a573) {
                        f225(v0, v1)
                    } else {
                        f134(Some(v0), Adt95::C2)
                    }
                }
            }
        }
    }
}

pub fn f177(v0: u64, v1: u64) -> Adt111 {
    let a574 = v1;
    let a575 = 0u64;
    if nat_eq(a574, a575) {
        Adt111::C0
    } else {
        let a576 = v1;
        let a577 = 2u64;
        if nat_eq(a576, a577) {
            Adt111::C2(v0)
        } else {
            Adt111::C1({
                let a581 = v0;
                let a582 = {
                    let a578 = v1;
                    let a579 = 3u64;
                    let a580 = 0u64;
                    nat_quot(a578, a579, a580)
                };
                nat_sub(a581, a582)
            })
        }
    }
}

pub fn f178(p610: Adt112, p611: u64, p612: u64, p613: List<u64>) -> R<u64> {
    let m609 = (p610, (p611, (p612, p613)));
    {
        let mut m609 = m609;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m609;
                let m583 = v3;
                match m583.uncons() {
                    None => {
                        let m584 = v0;
                        match m584 {
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
                        let m585 = v0;
                        match m585 {
                            Adt112::C0 => {
                                let a586 = v4;
                                let a587 = 128u64;
                                if nat_lt(a586, a587) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a588 = v4;
                                    let a589 = 194u64;
                                    if nat_lt(a588, a589) {
                                        let v6: u64 = f226({
                                            let a590 = List::cons(v4, v5);
                                            length_list(a590)
                                        })?;
                                        Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a591 = v4;
                                        let a592 = 224u64;
                                        if nat_lt(a591, a592) {
                                            Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a593 = v4;
                                            let a594 = 224u64;
                                            if nat_eq(a593, a594) {
                                                Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a595 = v4;
                                                let a596 = 237u64;
                                                if nat_eq(a595, a596) {
                                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a597 = v4;
                                                    let a598 = 240u64;
                                                    if nat_lt(a597, a598) {
                                                        Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a599 = v4;
                                                        let a600 = 240u64;
                                                        if nat_eq(a599, a600) {
                                                            Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a601 = v4;
                                                            let a602 = 244u64;
                                                            if nat_eq(a601, a602) {
                                                                Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a603 = v4;
                                                                let a604 = 244u64;
                                                                if nat_lt(a603, a604) {
                                                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f226({
                                                                        let a605 = List::cons(v4, v5);
                                                                        length_list(a605)
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
                                if f227(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f226({
                                        let a606 = List::cons(v4, v5);
                                        length_list(a606)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt112::C2 => {
                                if f227(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f226({
                                        let a607 = List::cons(v4, v5);
                                        length_list(a607)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt112::C3 => {
                                if f227(v4, v1, v2) {
                                    Ok::<(Adt112, (u64, (u64, List<u64>))), R<u64>>((Adt112::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f226({
                                        let a608 = List::cons(v4, v5);
                                        length_list(a608)
                                    })?;
                                    Err::<(Adt112, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m609 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Adt113, v1: u64) -> R<Result<Adt6, Adt93>> {
    let m615 = match v0.clone() {
        Adt113::C0(h614, _, _) => {
            h614
        }
    };
    match m615 {
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
            let m616 = f228(v3)?;
            match m616 {
                Adt135::C0 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C7(v1)))
                }
                Adt135::C1 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C8(v1)))
                }
                Adt135::C2(v4) => {
                    let m621 = {
                        let m620 = f229(v4, match v0.clone() {
                            Adt113::C0(_, h618, _) => {
                                h618
                            }
                        }, match v0 {
                            Adt113::C0(_, _, h619) => {
                                h619
                            }
                        });
                        match m620 {
                            Adt113::C0(h617, _, _) => {
                                h617
                            }
                        }
                    };
                    match m621 {
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

pub fn f180(v0: Adt113, v1: u64, v2: u64) -> R<Adt130> {
    let m623 = match v0.clone() {
        Adt113::C0(h622, _, _) => {
            h622
        }
    };
    match m623 {
        Adt114::C0 => {
            if f230(v2) {
                Ok(Adt130::C1(v0))
            } else {
                f231(v0, v1, v2)
            }
        }
        Adt114::C1 => {
            if f230(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a624 = v2;
                let a625 = 93u64;
                if nat_eq(a624, a625) {
                    Ok(f232(v0, v1))
                } else {
                    f231(v0, v1, v2)
                }
            }
        }
        Adt114::C2 => {
            Ok(f233(v0, v1, v2))
        }
        Adt114::C3 => {
            if f230(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a626 = v2;
                let a627 = 125u64;
                if nat_eq(a626, a627) {
                    Ok(f234(v0, v1))
                } else {
                    let a628 = v2;
                    let a629 = 34u64;
                    if nat_eq(a628, a629) {
                        Ok(f235(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f236(Adt93::C2(v1)))
                    }
                }
            }
        }
        Adt114::C4 => {
            if f230(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a630 = v2;
                let a631 = 34u64;
                if nat_eq(a630, a631) {
                    Ok(f235(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f236(Adt93::C2(v1)))
                }
            }
        }
        Adt114::C5 => {
            if f230(v2) {
                Ok(Adt130::C1(v0))
            } else {
                let a632 = v2;
                let a633 = 58u64;
                if nat_eq(a632, a633) {
                    Ok(f235(v0, Adt114::C0))
                } else {
                    Ok(f236(Adt93::C2(v1)))
                }
            }
        }
        Adt114::C6(v3, v4, v5) => {
            f237(v0, v1, v2, v3, v4, v5)
        }
        Adt114::C7(v6, v7) => {
            f238(v0, v1, v2, v6, v7)
        }
        Adt114::C8(v8) => {
            f239(v0, v1, v2, v8)
        }
        Adt114::C9(_) => {
            Ok(f233(v0, v1, v2))
        }
    }
}

pub fn f181(v0: u64) -> R<u64> {
    let a634 = 64u64;
    let a635 = v0;
    if if nat_lt(a634, a635) {
        let a636 = v0;
        let a637 = 91u64;
        nat_lt(a636, a637)
    } else {
        false
    } {
        let a638 = v0;
        let a639 = 32u64;
        nat_add(a638, a639)
    } else {
        Ok(v0)
    }
}

pub fn f182(p642: List<u64>) -> List<u64> {
    let m641 = p642;
    {
        let mut m641 = m641;
        loop {
            let step = {
                let v0 = m641;
                let m640 = v0;
                match m640.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f240(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: u64) -> bool {
    let a643 = v0;
    let a644 = 9u64;
    if nat_eq(a643, a644) {
        true
    } else {
        let a645 = 31u64;
        let a646 = v0;
        if nat_lt(a645, a646) {
            let a647 = v0;
            let a648 = 127u64;
            nat_lt(a647, a648)
        } else {
            false
        }
    }
}

pub fn f184(p651: List<u64>) -> bool {
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
                    Some((v1, v2)) => {
                        if f241(v1) {
                            Ok::<List<u64>, bool>(v2)
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

pub fn f185(p654: List<u64>, v1: bool) -> bool {
    let m653 = p654;
    {
        let mut m653 = m653;
        loop {
            let step = {
                let v0 = m653;
                let m652 = v0;
                match m652.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f242(v2)
                        } else {
                            f243(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186() -> u64 {
    64u64
}

pub fn f187(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m655 = f223(Str::lit("type"), v1);
    match m655 {
        Adt129::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt129::C1(v2) => {
            let m656 = f220(v2);
            match m656 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a657 = v3;
                    let a658 = Str::lit("object");
                    if equal(a657, a658) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f188(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m659 = f223(Str::lit("$schema"), v1);
    match m659 {
        Adt129::C0 => {
            None::<Adt91>
        }
        Adt129::C1(v2) => {
            let m660 = f220(v2);
            match m660 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a661 = v3;
                    let a662 = f244();
                    if equal(a661, a662) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f189(v0: Adt35) -> R<Option<Adt91>> {
    let m664 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h663, _, _) => {
            h663
        }
    };
    match m664 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m667 = f188(match v0.clone() {
                Adt35::C0(h665, _, _, _, _, _, _, _) => {
                    h665
                }
            }, match v1.clone() {
                Adt5::C0(h666) => {
                    h666
                }
            });
            match m667 {
                None => {
                    let m671 = f152(f153(), match v0 {
                        Adt35::C0(h668, _, _, _, _, _, _, _) => {
                            h668
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h669) => {
                            h669
                        }
                    }), List::cons(Adt99::C0(Adt6::C5(match v1 {
                        Adt5::C0(h670) => {
                            h670
                        }
                    }), false, List::<Str>::nil()), List::<Adt99>::nil()), f154())?;
                    match m671 {
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

pub fn f190(v0: Str, v1: Adt6, v2: Adt99, v3: Adt119, v4: List<Adt99>) -> R<Result<(List<Adt99>, Adt119), Adt91>> {
    let m672 = v2;
    match m672 {
        Adt99::C0(v5, v6, v7) => {
            let m673 = f245(v5.clone());
            match m673 {
                None => {
                    let m674 = f246(v5);
                    match m674 {
                        None => {
                            Ok(Err::<(List<Adt99>, Adt119), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f247(v0, v1, match v9 {
                                Adt5::C0(h675) => {
                                    h675
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

pub fn f191(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    let m676 = f245(v1.clone());
    match m676 {
        None => {
            let m677 = f246(v1);
            match m677 {
                None => {
                    Ok(Adt131::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f248(v0, match v5 {
                        Adt5::C0(h678) => {
                            h678
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

pub fn f192(v0: Adt103, v1: List<Adt103>) -> R<Adt120> {
    let m679 = v0;
    match m679 {
        Adt103::C0(_, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
        Adt103::C1(_, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
        Adt103::C2(v8, v9, v10, v11) => {
            Ok(f249(v8, v9, v10, {
                let a680 = v11;
                let a681 = 1u64;
                nat_add(a680, a681)?
            }, v1))
        }
        Adt103::C3(v12) => {
            Ok(Adt120::C0(Adt102::C1(v12, Str::lit("not")), v1))
        }
        Adt103::C4(v13, _, v15, v16) => {
            Ok(f250(v13, v15, v16, v1))
        }
        Adt103::C5(_, _, _, _) => {
            Ok(Adt120::C0(Adt102::C0, v1))
        }
    }
}

pub fn f193(v0: Adt104, v1: Adt6, v2: Adt6, v3: List<Adt101>) -> Adt131 {
    let m682 = f246(v1);
    match m682 {
        None => {
            Adt131::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m683 = v0;
            match m683 {
                Adt104::C0 => {
                    f251(match v4 {
                        Adt5::C0(h684) => {
                            h684
                        }
                    }, v2, v3)
                }
                Adt104::C1 => {
                    f252(match v4 {
                        Adt5::C0(h685) => {
                            h685
                        }
                    }, v2, v3)
                }
                Adt104::C2 => {
                    f253(match v4 {
                        Adt5::C0(h686) => {
                            h686
                        }
                    }, v2, v3)
                }
                Adt104::C3 => {
                    f254(match v4 {
                        Adt5::C0(h687) => {
                            h687
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f194(v0: Adt103, v1: List<Adt103>, v2: List<Adt101>, v3: Str) -> Adt120 {
    let m688 = v0;
    match m688 {
        Adt103::C0(_, _, _) => {
            Adt120::C0(Adt102::C1(v2, v3), v1)
        }
        Adt103::C1(v7, v8, v9) => {
            f255(v7, v8, v9, v1)
        }
        Adt103::C2(v10, v11, v12, v13) => {
            f249(v10, v11, v12, v13, v1)
        }
        Adt103::C3(_) => {
            Adt120::C0(Adt102::C0, v1)
        }
        Adt103::C4(_, v16, v17, v18) => {
            f250(v16, v17, v18, v1)
        }
        Adt103::C5(_, _, _, _) => {
            Adt120::C0(Adt102::C1(v2, v3), v1)
        }
    }
}

pub fn f195(p691: List<Adt101>, p692: List<Str>) -> R<List<Str>> {
    let m690 = (p691, p692);
    {
        let mut m690 = m690;
        loop {
            let step = {
                let (v0, v1) = m690;
                let m689 = v0;
                match m689.uncons() {
                    None => {
                        Err::<(List<Adt101>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt101>, List<Str>), R<List<Str>>>((v3, List::cons(f256(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m690 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(p701: List<Adt38>) -> R<bool> {
    let m700 = p701;
    {
        let mut m700 = m700;
        loop {
            let step = {
                let v0 = m700;
                let m693 = v0;
                match m693.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m694 = v1;
                        match m694 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f257(match v4 {
                                    Adt42::C0(h695, _, _, _) => {
                                        h695
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f257(match v5 {
                                    Adt43::C0(h696, _, _, _) => {
                                        h696
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
                                let m698 = match v7 {
                                    Adt45::C0(h697, _, _) => {
                                        h697
                                    }
                                };
                                match m698 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f257(match v9 {
                                            Adt48::C0(_, _, h699, _) => {
                                                h699
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
                Ok(next) => { m700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f197(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f258(match v1.clone() {
        Adt49::C0(h702, _) => {
            h702
        }
    }) {
        f259(v1)
    } else {
        let m704 = match v0 {
            Adt35::C0(_, _, _, _, _, h703, _, _) => {
                h703
            }
        };
        match m704 {
            None => {
                f259(v1)
            }
            Some(v2) => {
                f260(v2, v1)
            }
        }
    }
}

pub fn f198(v0: Adt105, v1: List<Adt105>, v2: List<u64>) -> R<Adt132> {
    let m705 = v0;
    match m705 {
        Adt105::C0(v3) => {
            Ok(Adt132::C1(v1, f160(v3, v2)))
        }
        Adt105::C1(v4, v5) => {
            f261(v4, v5, v1, v2)
        }
        Adt105::C2(v6, v7) => {
            let m706 = v7;
            match m706.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C1(v6, v8), List::cons(Adt105::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt105::C3(v10, v11) => {
            let m707 = v11.clone();
            match m707.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt105::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt105::C4(v14, v15) => {
            let m708 = v15;
            match m708.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt105::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt132::C1(List::cons(Adt105::C0({
                        let a710 = f262({
                            let (h709, _) = v16;
                            h709
                        })?;
                        let a711 = List::cons(58u64, List::<u64>::nil());
                        append_list(a710, a711)
                    }), List::cons(Adt105::C1(v14, {
                        let (_, h712) = v16;
                        h712
                    }), List::cons(Adt105::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt105::C5(v18, v19) => {
            let m713 = v19.clone();
            match m713.uncons() {
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

pub fn f199(v0: Adt122) -> R<Option<Adt5>> {
    let m715 = f263(match v0.clone() {
        Adt122::C0(h714, _, _) => {
            h714
        }
    })?;
    match m715 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a718 = v1;
                let a719 = {
                    let m717 = f264(v0);
                    match m717 {
                        Adt5::C0(h716) => {
                            h716
                        }
                    }
                };
                append_list(a718, a719)
            })))
        }
    }
}

pub fn f200(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m720 = v2;
    match m720 {
        None => {
            f6()
        }
        Some(v3) => {
            f265(v0, v1, f38(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f201() -> u64 {
    200u64
}

pub fn f202(v0: Adt92, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), List::cons((Str::lit("id"), f161(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h721) => {
            h721
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f203(p725: List<Adt124>, p726: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m724 = (p725, p726);
    {
        let mut m724 = m724;
        loop {
            let step = {
                let (v0, v1) = m724;
                let m722 = v0;
                match m722.uncons() {
                    None => {
                        Err::<(List<Adt124>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m723 = f266(v2)?;
                        match m723 {
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
                Ok(next) => { m724 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: Adt125) -> u64 {
    let m727 = v0;
    match m727 {
        Adt125::C0(_) => {
            f95()
        }
        Adt125::C1(_, _) => {
            f95()
        }
        Adt125::C2(_) => {
            f95()
        }
        Adt125::C3(_) => {
            f267()
        }
        Adt125::C4(v6) => {
            f268(v6)
        }
    }
}

pub fn f205(v0: Adt125) -> R<Adt80> {
    Ok(Adt80::C0(f269(v0.clone()), f270(v0.clone()), f271(v0)?))
}

pub fn f206() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f207(p730: List<List<u64>>) -> R<bool> {
    let m729 = p730;
    {
        let mut m729 = m729;
        loop {
            let step = {
                let v0 = m729;
                let m728 = v0;
                match m728.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f272(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f208(v0: u64, p735: List<u64>, p736: List<u64>, p737: List<List<u64>>) -> List<List<u64>> {
    let m734 = (p735, (p736, p737));
    {
        let mut m734 = m734;
        loop {
            let step = {
                let (v1, (v2, v3)) = m734;
                let m731 = v1;
                match m731.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f38(List::cons(f148(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a732 = v4;
                        let a733 = v0;
                        if nat_eq(a732, a733) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f148(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt133, Adt125>) -> R<Result<Adt122, Adt125>> {
    let m738 = v3;
    match m738 {
        Ok(v5) => {
            f273(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt122, Adt125>(v4))
        }
    }
}

pub fn f210(v0: Option<Adt106>, v1: List<Str>, v2: Adt98) -> R<Result<Adt133, Adt125>> {
    Ok(f274(f275(match v2.clone() {
        Adt98::C0(_, h739, _) => {
            h739
        }
    }), f276(v0, v1, v2)?))
}

pub fn f211(v0: Adt108, v1: Adt96) -> Adt127 {
    let m740 = v1;
    match m740 {
        Adt96::C0 => {
            f217()
        }
        Adt96::C1 => {
            Adt127::C0(None::<Str>, Some(Adt107::C6(v0)))
        }
        Adt96::C2 => {
            Adt127::C0(None::<Str>, Some(Adt107::C7(v0)))
        }
        Adt96::C3(v2) => {
            let m741 = f277(v2);
            match m741 {
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

pub fn f212() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f213() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f214(v0: Str) -> bool {
    let m742 = f278(v0);
    match m742 {
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

pub fn f215(v0: Adt96) -> R<Adt96> {
    let m743 = v0.clone();
    match m743 {
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
            let m744 = f279(v1);
            match m744 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m745 = f280(v2)?;
                    match m745 {
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

pub fn f216() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f217() -> Adt127 {
    Adt127::C0(None::<Str>, None::<Adt107>)
}

pub fn f218(v0: Option<Adt107>, v1: Option<Adt107>) -> Option<Adt107> {
    let m746 = v0.clone();
    match m746 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f219(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt109>> {
    Ok(f281(f282(v0, v1, v2, List::<Adt109>::nil())?, List::<Adt109>::nil()))
}

pub fn f220(v0: Adt6) -> Option<Str> {
    let m747 = v0;
    match m747 {
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

pub fn f221() -> Str {
    Str::lit("2026-07-28")
}

pub fn f222(v0: Adt7) -> R<Option<i64>> {
    let m750 = f283(40u64, match v0.clone() {
        Adt7::C0(h748, _) => {
            h748
        }
    }, match v0 {
        Adt7::C0(_, h749) => {
            h749
        }
    })?;
    match m750 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f284(v1))
        }
    }
}

pub fn f223(v0: Str, p756: List<(Str, Adt6)>) -> Adt129 {
    let m755 = p756;
    {
        let mut m755 = m755;
        loop {
            let step = {
                let v1 = m755;
                let m751 = v1;
                match m751.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt129>(Adt129::C0)
                    }
                    Some((v2, v3)) => {
                        let m753 = f285({
                            let (h752, _) = v2;
                            h752
                        }, v0.clone());
                        match m753 {
                            Adt140::C0 => {
                                Err::<List<(Str, Adt6)>, Adt129>(Adt129::C1({
                                    let (_, h754) = v2;
                                    h754
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
                Ok(next) => { m755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f224(v0: Str, v1: Adt5) -> bool {
    f286(f173(v0, v1))
}

pub fn f225(v0: Adt92, v1: Adt5) -> Result<Adt98, Adt94> {
    let m757 = f173(Str::lit("method"), v1.clone());
    match m757 {
        Adt129::C0 => {
            f134(Some(v0), f174(v1))
        }
        Adt129::C1(v2) => {
            let m758 = f220(v2);
            match m758 {
                None => {
                    f134(Some(v0), Adt95::C4)
                }
                Some(v3) => {
                    f287(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f226(v0: u64) -> R<u64> {
    let a761 = {
        let a759 = v0;
        let a760 = 3u64;
        nat_mul(a759, a760)?
    };
    let a762 = 1u64;
    nat_add(a761, a762)
}

pub fn f227(v0: u64, v1: u64, v2: u64) -> bool {
    let a763 = v1;
    let a764 = v0;
    if nat_le(a763, a764) {
        let a765 = v0;
        let a766 = v2;
        nat_le(a765, a766)
    } else {
        false
    }
}

pub fn f228(v0: Adt116) -> R<Adt135> {
    if f288(match v0.clone() {
        Adt116::C0(_, h767, _, _, _, _, _) => {
            h767
        }
    }) {
        let v1: i64 = {
            let a773 = if match v0.clone() {
                Adt116::C0(_, _, _, _, _, h768, _) => {
                    h768
                }
            } {
                let a770 = match v0.clone() {
                    Adt116::C0(_, _, _, _, _, _, h769) => {
                        h769
                    }
                };
                int_neg(a770)?
            } else {
                match v0.clone() {
                    Adt116::C0(_, _, _, _, _, _, h771) => {
                        h771
                    }
                }
            };
            let a774 = match v0.clone() {
                Adt116::C0(_, _, _, _, h772, _, _) => {
                    h772
                }
            };
            int_sub(a773, a774)?
        };
        let m777 = {
            let a775 = v1;
            let a776 = -9999i64;
            compare(a775, a776)
        };
        if if match m777 {
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
            let m780 = {
                let a778 = 9999i64;
                let a779 = v1;
                compare(a778, a779)
            };
            match m780 {
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
                Adt116::C0(h781, _, _, _, _, _, _) => {
                    h781
                }
            } {
                let a783 = match v0 {
                    Adt116::C0(_, _, h782, _, _, _, _) => {
                        h782
                    }
                };
                int_neg(a783)?
            } else {
                match v0 {
                    Adt116::C0(_, _, h784, _, _, _, _) => {
                        h784
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f229(v0: Adt6, v1: List<Adt118>, v2: u64) -> Adt113 {
    let m785 = v1.clone();
    match m785.uncons() {
        None => {
            Adt113::C0(Adt114::C9(v0), List::<Adt118>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m786 = v3;
            match m786 {
                Adt118::C0(v5) => {
                    Adt113::C0(Adt114::C2, List::cons(Adt118::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt118::C1(v6, v7) => {
                    let m787 = v7;
                    match m787 {
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

pub fn f230(v0: u64) -> bool {
    let a788 = v0;
    let a789 = 32u64;
    if nat_eq(a788, a789) {
        true
    } else {
        let a790 = v0;
        let a791 = 10u64;
        if nat_eq(a790, a791) {
            true
        } else {
            let a792 = v0;
            let a793 = 13u64;
            if nat_eq(a792, a793) {
                true
            } else {
                let a794 = v0;
                let a795 = 9u64;
                nat_eq(a794, a795)
            }
        }
    }
}

pub fn f231(v0: Adt113, v1: u64, v2: u64) -> R<Adt130> {
    let a796 = v2;
    let a797 = 34u64;
    if nat_eq(a796, a797) {
        Ok(f235(v0, Adt114::C6(Adt115::C0, List::<u64>::nil(), false)))
    } else {
        let a798 = v2;
        let a799 = 123u64;
        if nat_eq(a798, a799) {
            f289(v0, Adt118::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt114::C3, v1)
        } else {
            let a800 = v2;
            let a801 = 91u64;
            if nat_eq(a800, a801) {
                f289(v0, Adt118::C0(List::<Adt6>::nil()), Adt114::C1, v1)
            } else {
                let a802 = v2;
                let a803 = 116u64;
                if nat_eq(a802, a803) {
                    Ok(f235(v0, Adt114::C7(0u64, 1u64)))
                } else {
                    let a804 = v2;
                    let a805 = 102u64;
                    if nat_eq(a804, a805) {
                        Ok(f235(v0, Adt114::C7(1u64, 1u64)))
                    } else {
                        let a806 = v2;
                        let a807 = 110u64;
                        if nat_eq(a806, a807) {
                            Ok(f235(v0, Adt114::C7(2u64, 1u64)))
                        } else {
                            let a808 = v2;
                            let a809 = 45u64;
                            if nat_eq(a808, a809) {
                                Ok(f235(v0, f290(true, Adt117::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a810 = v2;
                                let a811 = 48u64;
                                if nat_eq(a810, a811) {
                                    Ok(f235(v0, f290(false, Adt117::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f227(v2, 49u64, 57u64) {
                                        Ok(f235(v0, f290(false, Adt117::C2, f291({
                                            let a812 = v2;
                                            let a813 = 48u64;
                                            nat_sub(a812, a813)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f236(Adt93::C2(v1)))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f232(v0: Adt113, v1: u64) -> Adt130 {
    let m815 = match v0.clone() {
        Adt113::C0(_, h814, _) => {
            h814
        }
    };
    match m815.uncons() {
        None => {
            f236(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m816 = v2;
            match m816 {
                Adt118::C0(v4) => {
                    Adt130::C1(f229(Adt6::C4(f292(v4, List::<Adt6>::nil())), v3, {
                        let a818 = match v0 {
                            Adt113::C0(_, _, h817) => {
                                h817
                            }
                        };
                        let a819 = 1u64;
                        nat_sub(a818, a819)
                    }))
                }
                Adt118::C1(_, _) => {
                    f236(Adt93::C2(v1))
                }
            }
        }
    }
}

pub fn f233(v0: Adt113, v1: u64, v2: u64) -> Adt130 {
    let m821 = match v0.clone() {
        Adt113::C0(h820, _, _) => {
            h820
        }
    };
    match m821 {
        Adt114::C0 => {
            f236(Adt93::C2(v1))
        }
        Adt114::C1 => {
            f236(Adt93::C2(v1))
        }
        Adt114::C2 => {
            if f230(v2) {
                Adt130::C1(v0)
            } else {
                let a822 = v2;
                let a823 = 44u64;
                if nat_eq(a822, a823) {
                    f293(v0, v1)
                } else {
                    let a824 = v2;
                    let a825 = 93u64;
                    if nat_eq(a824, a825) {
                        f232(v0, v1)
                    } else {
                        let a826 = v2;
                        let a827 = 125u64;
                        if nat_eq(a826, a827) {
                            f234(v0, v1)
                        } else {
                            f236(Adt93::C2(v1))
                        }
                    }
                }
            }
        }
        Adt114::C3 => {
            f236(Adt93::C2(v1))
        }
        Adt114::C4 => {
            f236(Adt93::C2(v1))
        }
        Adt114::C5 => {
            f236(Adt93::C2(v1))
        }
        Adt114::C6(_, _, _) => {
            f236(Adt93::C2(v1))
        }
        Adt114::C7(_, _) => {
            f236(Adt93::C2(v1))
        }
        Adt114::C8(_) => {
            f236(Adt93::C2(v1))
        }
        Adt114::C9(_) => {
            if f230(v2) {
                Adt130::C1(v0)
            } else {
                f236(Adt93::C10(v1))
            }
        }
    }
}

pub fn f234(v0: Adt113, v1: u64) -> Adt130 {
    let m829 = match v0.clone() {
        Adt113::C0(_, h828, _) => {
            h828
        }
    };
    match m829.uncons() {
        None => {
            f236(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m830 = v2;
            match m830 {
                Adt118::C0(_) => {
                    f236(Adt93::C2(v1))
                }
                Adt118::C1(v5, _) => {
                    Adt130::C1(f229(Adt6::C5(f294(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a832 = match v0 {
                            Adt113::C0(_, _, h831) => {
                                h831
                            }
                        };
                        let a833 = 1u64;
                        nat_sub(a832, a833)
                    }))
                }
            }
        }
    }
}

pub fn f235(v0: Adt113, v1: Adt114) -> Adt130 {
    Adt130::C1(f295(v0, v1))
}

pub fn f236(v0: Adt93) -> Adt130 {
    Adt130::C0(v0)
}

pub fn f237(v0: Adt113, v1: u64, v2: u64, v3: Adt115, v4: List<u64>, v5: bool) -> R<Adt130> {
    let m834 = v3;
    match m834 {
        Adt115::C0 => {
            let a835 = v2;
            let a836 = 34u64;
            if nat_eq(a835, a836) {
                Ok(f296(v0, v1, v4, v5))
            } else {
                let a837 = v2;
                let a838 = 92u64;
                if nat_eq(a837, a838) {
                    Ok(f235(v0, Adt114::C6(Adt115::C1, v4, v5)))
                } else {
                    let a839 = v2;
                    let a840 = 32u64;
                    if nat_lt(a839, a840) {
                        Ok(f236(Adt93::C4(v1)))
                    } else {
                        Ok(f235(v0, Adt114::C6(Adt115::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt115::C1 => {
            let a841 = v2;
            let a842 = 117u64;
            if nat_eq(a841, a842) {
                Ok(f235(v0, Adt114::C6(Adt115::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a843 = f297(v2);
                let a844 = 0u64;
                if nat_eq(a843, a844) {
                    Ok(f236(Adt93::C5(v1)))
                } else {
                    Ok(f235(v0, Adt114::C6(Adt115::C0, List::cons(f297(v2), v4), v5)))
                }
            }
        }
        Adt115::C2(v6, v7, v8) => {
            let a845 = 15u64;
            let a846 = f298(v2);
            if nat_lt(a845, a846) {
                Ok(f236(Adt93::C5(v1)))
            } else {
                let a849 = {
                    let a847 = v6;
                    let a848 = 1u64;
                    nat_add(a847, a848)?
                };
                let a850 = 4u64;
                if nat_eq(a849, a850) {
                    f299(v0, v1, {
                        let a853 = {
                            let a851 = v7;
                            let a852 = 16u64;
                            nat_mul(a851, a852)?
                        };
                        let a854 = f298(v2);
                        nat_add(a853, a854)?
                    }, v8, v4, v5)
                } else {
                    Ok(f235(v0, Adt114::C6(Adt115::C2({
                        let a855 = v6;
                        let a856 = 1u64;
                        nat_add(a855, a856)?
                    }, {
                        let a859 = {
                            let a857 = v7;
                            let a858 = 16u64;
                            nat_mul(a857, a858)?
                        };
                        let a860 = f298(v2);
                        nat_add(a859, a860)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt115::C3(v9) => {
            let a861 = v2;
            let a862 = 92u64;
            if nat_eq(a861, a862) {
                Ok(f235(v0, Adt114::C6(Adt115::C4(v9), v4, v5)))
            } else {
                Ok(f236(Adt93::C6(v1)))
            }
        }
        Adt115::C4(v10) => {
            let a863 = v2;
            let a864 = 117u64;
            if nat_eq(a863, a864) {
                Ok(f235(v0, Adt114::C6(Adt115::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f236(Adt93::C6(v1)))
            }
        }
    }
}

pub fn f238(v0: Adt113, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt130> {
    let a865 = v2;
    let a866 = f300(v3, v4);
    if nat_eq(a865, a866) {
        let a870 = {
            let a867 = v4;
            let a868 = 1u64;
            nat_add(a867, a868)?
        };
        let a871 = {
            let a869 = f301(v3);
            length_list(a869)
        };
        if nat_eq(a870, a871) {
            Ok(f302(v0, f303(v3)))
        } else {
            Ok(f235(v0, Adt114::C7(v3, {
                let a872 = v4;
                let a873 = 1u64;
                nat_add(a872, a873)?
            })))
        }
    } else {
        Ok(f236(Adt93::C2(v1)))
    }
}

pub fn f239(v0: Adt113, v1: u64, v2: u64, v3: Adt116) -> R<Adt130> {
    let m875 = match v3.clone() {
        Adt116::C0(_, h874, _, _, _, _, _) => {
            h874
        }
    };
    match m875 {
        Adt117::C0 => {
            let a876 = v2;
            let a877 = 48u64;
            if nat_eq(a876, a877) {
                Ok(f304(v0, v3, Adt117::C1))
            } else {
                if f227(v2, 49u64, 57u64) {
                    f305(v0, v1, v2, v3, false)
                } else {
                    Ok(f236(Adt93::C7(v1)))
                }
            }
        }
        Adt117::C1 => {
            let a878 = v2;
            let a879 = 46u64;
            if nat_eq(a878, a879) {
                Ok(f304(v0, v3, Adt117::C3))
            } else {
                if f306(v2) {
                    Ok(f304(v0, v3, Adt117::C5))
                } else {
                    if f307(v2) {
                        Ok(f236(Adt93::C7(v1)))
                    } else {
                        f308(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt117::C2 => {
            if f307(v2) {
                f305(v0, v1, v2, v3, false)
            } else {
                let a880 = v2;
                let a881 = 46u64;
                if nat_eq(a880, a881) {
                    Ok(f304(v0, v3, Adt117::C3))
                } else {
                    if f306(v2) {
                        Ok(f304(v0, v3, Adt117::C5))
                    } else {
                        f308(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt117::C3 => {
            if f307(v2) {
                f305(v0, v1, v2, v3, true)
            } else {
                Ok(f236(Adt93::C7(v1)))
            }
        }
        Adt117::C4 => {
            if f307(v2) {
                f305(v0, v1, v2, v3, true)
            } else {
                if f306(v2) {
                    Ok(f304(v0, v3, Adt117::C5))
                } else {
                    f308(v0, v1, v2, v3)
                }
            }
        }
        Adt117::C5 => {
            let a882 = v2;
            let a883 = 43u64;
            if nat_eq(a882, a883) {
                Ok(f235(v0, f290(match v3.clone() {
                    Adt116::C0(h884, _, _, _, _, _, _) => {
                        h884
                    }
                }, Adt117::C6, match v3.clone() {
                    Adt116::C0(_, _, h885, _, _, _, _) => {
                        h885
                    }
                }, match v3.clone() {
                    Adt116::C0(_, _, _, h886, _, _, _) => {
                        h886
                    }
                }, match v3 {
                    Adt116::C0(_, _, _, _, h887, _, _) => {
                        h887
                    }
                }, false, 0i64)))
            } else {
                let a888 = v2;
                let a889 = 45u64;
                if nat_eq(a888, a889) {
                    Ok(f235(v0, f290(match v3.clone() {
                        Adt116::C0(h890, _, _, _, _, _, _) => {
                            h890
                        }
                    }, Adt117::C6, match v3.clone() {
                        Adt116::C0(_, _, h891, _, _, _, _) => {
                            h891
                        }
                    }, match v3.clone() {
                        Adt116::C0(_, _, _, h892, _, _, _) => {
                            h892
                        }
                    }, match v3 {
                        Adt116::C0(_, _, _, _, h893, _, _) => {
                            h893
                        }
                    }, true, 0i64)))
                } else {
                    if f307(v2) {
                        f309(v0, v1, v2, Adt116::C0(match v3.clone() {
                            Adt116::C0(h894, _, _, _, _, _, _) => {
                                h894
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, h895, _, _, _, _, _) => {
                                h895
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, _, h896, _, _, _, _) => {
                                h896
                            }
                        }, match v3.clone() {
                            Adt116::C0(_, _, _, h897, _, _, _) => {
                                h897
                            }
                        }, match v3 {
                            Adt116::C0(_, _, _, _, h898, _, _) => {
                                h898
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f236(Adt93::C7(v1)))
                    }
                }
            }
        }
        Adt117::C6 => {
            if f307(v2) {
                f309(v0, v1, v2, v3.clone(), match v3 {
                    Adt116::C0(_, _, _, _, _, h899, _) => {
                        h899
                    }
                })
            } else {
                Ok(f236(Adt93::C7(v1)))
            }
        }
        Adt117::C7 => {
            if f307(v2) {
                f309(v0, v1, v2, v3.clone(), match v3 {
                    Adt116::C0(_, _, _, _, _, h900, _) => {
                        h900
                    }
                })
            } else {
                f308(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f240(v0: u64) -> bool {
    let a901 = v0;
    let a902 = 9u64;
    if nat_eq(a901, a902) {
        true
    } else {
        let a903 = v0;
        let a904 = 32u64;
        nat_eq(a903, a904)
    }
}

pub fn f241(v0: u64) -> bool {
    let a905 = 47u64;
    let a906 = v0;
    if nat_lt(a905, a906) {
        let a907 = v0;
        let a908 = 58u64;
        nat_lt(a907, a908)
    } else {
        false
    }
}

pub fn f242(v0: u64) -> bool {
    let a909 = 48u64;
    let a910 = v0;
    if if nat_le(a909, a910) {
        let a911 = v0;
        let a912 = 57u64;
        nat_le(a911, a912)
    } else {
        false
    } {
        true
    } else {
        let a913 = 65u64;
        let a914 = v0;
        if if nat_le(a913, a914) {
            let a915 = v0;
            let a916 = 90u64;
            nat_le(a915, a916)
        } else {
            false
        } {
            true
        } else {
            let a917 = 97u64;
            let a918 = v0;
            if if nat_le(a917, a918) {
                let a919 = v0;
                let a920 = 122u64;
                nat_le(a919, a920)
            } else {
                false
            } {
                true
            } else {
                let a921 = v0;
                let a922 = 33u64;
                if nat_eq(a921, a922) {
                    true
                } else {
                    let a923 = v0;
                    let a924 = 35u64;
                    if nat_eq(a923, a924) {
                        true
                    } else {
                        let a925 = v0;
                        let a926 = 36u64;
                        if nat_eq(a925, a926) {
                            true
                        } else {
                            let a927 = v0;
                            let a928 = 37u64;
                            if nat_eq(a927, a928) {
                                true
                            } else {
                                let a929 = v0;
                                let a930 = 38u64;
                                if nat_eq(a929, a930) {
                                    true
                                } else {
                                    let a931 = v0;
                                    let a932 = 39u64;
                                    if nat_eq(a931, a932) {
                                        true
                                    } else {
                                        let a933 = v0;
                                        let a934 = 42u64;
                                        if nat_eq(a933, a934) {
                                            true
                                        } else {
                                            let a935 = v0;
                                            let a936 = 43u64;
                                            if nat_eq(a935, a936) {
                                                true
                                            } else {
                                                let a937 = v0;
                                                let a938 = 45u64;
                                                if nat_eq(a937, a938) {
                                                    true
                                                } else {
                                                    let a939 = v0;
                                                    let a940 = 46u64;
                                                    if nat_eq(a939, a940) {
                                                        true
                                                    } else {
                                                        let a941 = v0;
                                                        let a942 = 94u64;
                                                        if nat_eq(a941, a942) {
                                                            true
                                                        } else {
                                                            let a943 = v0;
                                                            let a944 = 95u64;
                                                            if nat_eq(a943, a944) {
                                                                true
                                                            } else {
                                                                let a945 = v0;
                                                                let a946 = 96u64;
                                                                if nat_eq(a945, a946) {
                                                                    true
                                                                } else {
                                                                    let a947 = v0;
                                                                    let a948 = 124u64;
                                                                    if nat_eq(a947, a948) {
                                                                        true
                                                                    } else {
                                                                        let a949 = v0;
                                                                        let a950 = 126u64;
                                                                        nat_eq(a949, a950)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f243(v0: u64) -> bool {
    let a951 = 48u64;
    let a952 = v0;
    if if nat_le(a951, a952) {
        let a953 = v0;
        let a954 = 57u64;
        nat_le(a953, a954)
    } else {
        false
    } {
        true
    } else {
        let a955 = 65u64;
        let a956 = v0;
        if if nat_le(a955, a956) {
            let a957 = v0;
            let a958 = 90u64;
            nat_le(a957, a958)
        } else {
            false
        } {
            true
        } else {
            let a959 = 97u64;
            let a960 = v0;
            if if nat_le(a959, a960) {
                let a961 = v0;
                let a962 = 122u64;
                nat_le(a961, a962)
            } else {
                false
            } {
                true
            } else {
                let a963 = v0;
                let a964 = 95u64;
                if nat_eq(a963, a964) {
                    true
                } else {
                    let a965 = v0;
                    let a966 = 45u64;
                    if nat_eq(a965, a966) {
                        true
                    } else {
                        let a967 = v0;
                        let a968 = 46u64;
                        nat_eq(a967, a968)
                    }
                }
            }
        }
    }
}

pub fn f244() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f245(v0: Adt6) -> Option<bool> {
    let m969 = v0;
    match m969 {
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

pub fn f246(v0: Adt6) -> Option<Adt5> {
    let m970 = v0;
    match m970 {
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

pub fn f247(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt119, v6: List<Adt99>) -> R<Result<(List<Adt99>, Adt119), Adt91>> {
    let m971 = f310(v0.clone(), v1, v2.clone());
    match m971 {
        None => {
            let m972 = f311(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m972 {
                Adt141::C0(v8) => {
                    Ok(Err::<(List<Adt99>, Adt119), Adt91>(v8))
                }
                Adt141::C1(v9) => {
                    Ok(Ok::<(List<Adt99>, Adt119), Adt91>((f312(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt99>, Adt119), Adt91>(v7))
        }
    }
}

pub fn f248(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    let m973 = f313(v1.clone(), v2.clone())?;
    match m973 {
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
            f314(v0, v1, v2, v3)
        }
    }
}

pub fn f249(v0: List<Adt6>, v1: Adt6, v2: List<Adt101>, v3: u64, v4: List<Adt103>) -> Adt120 {
    let m974 = v0;
    match m974.uncons() {
        None => {
            let a975 = v3;
            let a976 = 1u64;
            if nat_eq(a975, a976) {
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

pub fn f250(v0: Option<Adt6>, v1: Adt6, v2: List<Adt101>, v3: List<Adt103>) -> Adt120 {
    let m977 = v0;
    match m977 {
        None => {
            Adt120::C0(Adt102::C0, v3)
        }
        Some(v4) => {
            Adt120::C0(Adt102::C0, List::cons(Adt103::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f251(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m978 = f223(Str::lit("anyOf"), v0);
    match m978 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m979 = f315(v3);
            match m979 {
                None => {
                    Adt131::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m980 = v4;
                    match m980.uncons() {
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

pub fn f252(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m981 = f223(Str::lit("oneOf"), v0);
    match m981 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m982 = f315(v3);
            match m982 {
                None => {
                    Adt131::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m983 = v4;
                    match m983.uncons() {
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

pub fn f253(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m984 = f223(Str::lit("not"), v0);
    match m984 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            Adt131::C3(List::cons(Adt103::C0(v3, v1, v2.clone()), List::cons(Adt103::C3(v2), List::<Adt103>::nil())))
        }
    }
}

pub fn f254(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m985 = f223(Str::lit("if"), v0.clone());
    match m985 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            Adt131::C3(List::cons(Adt103::C0(v3, v1.clone(), v2.clone()), List::cons(Adt103::C4(f316(Str::lit("then"), v0.clone()), f316(Str::lit("else"), v0), v1, v2), List::<Adt103>::nil())))
        }
    }
}

pub fn f255(v0: List<Adt6>, v1: Adt6, v2: List<Adt101>, v3: List<Adt103>) -> Adt120 {
    let m986 = v0;
    match m986.uncons() {
        None => {
            Adt120::C0(Adt102::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt120::C0(Adt102::C0, List::cons(Adt103::C0(v4, v1.clone(), v2.clone()), List::cons(Adt103::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f256(v0: Adt101) -> R<Str> {
    let m987 = v0;
    match m987 {
        Adt101::C0(v1) => {
            f317(v1)
        }
        Adt101::C1(v2) => {
            let a988 = f80(v2, 0i64)?;
            Ok(format_i64(a988))
        }
    }
}

pub fn f257(v0: Str) -> R<bool> {
    let m990 = f280(f85({
        let a989 = v0;
        utf8_encode(a989)
    })?)?;
    match m990 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f258(v0: Adt37) -> bool {
    let m992 = match v0 {
        Adt37::C0(_, _, h991) => {
            h991
        }
    };
    match m992 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f259(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m994 = match v0.clone() {
        Adt49::C0(_, h993) => {
            h993
        }
    };
    match m994.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h995, _) => {
                    h995
                }
            })))
        }
        Some((_, _)) => {
            if f318(match v0.clone() {
                Adt49::C0(_, h996) => {
                    h996
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f260(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m1000 = {
        let m999 = match v1.clone() {
            Adt49::C0(h998, _) => {
                h998
            }
        };
        match m999 {
            Adt37::C0(_, h997, _) => {
                h997
            }
        }
    };
    match m1000 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m1003 = f115(Adt6::C5(match v0.clone() {
                Adt5::C0(h1001) => {
                    h1001
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h1002) => {
                    h1002
                }
            }), v2)?;
            match m1003 {
                Adt100::C0 => {
                    f259(v1)
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

pub fn f261(v0: u64, v1: Adt6, v2: List<Adt105>, v3: List<u64>) -> R<Adt132> {
    let m1004 = v1;
    match m1004 {
        Adt6::C0 => {
            Ok(Adt132::C1(v2, f160(f319(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt132::C1(v2, f160(if v4 {
                f319(0u64)
            } else {
                f319(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f320(v5.clone())? {
                Ok(Adt132::C1(v2, f160(f321(v5)?, v3)))
            } else {
                Ok(Adt132::C0(Adt93::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt132::C1(v2, f160(f262(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a1005 = v0;
            let a1006 = f322();
            if nat_lt(a1005, a1006) {
                Ok(Adt132::C1(f323(91u64, Adt105::C2({
                    let a1007 = v0;
                    let a1008 = 1u64;
                    nat_add(a1007, a1008)?
                }, v7), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1009 = v0;
            let a1010 = f322();
            if nat_lt(a1009, a1010) {
                Ok(Adt132::C1(f323(123u64, Adt105::C4({
                    let a1011 = v0;
                    let a1012 = 1u64;
                    nat_add(a1011, a1012)?
                }, v8), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
    }
}

pub fn f262(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1014 = f160(f324(f85({
            let a1013 = v0;
            utf8_encode(a1013)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1015 = List::cons(34u64, List::<u64>::nil());
        append_list(a1014, a1015)
    }))
}

pub fn f263(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1016 = v0;
    match m1016 {
        Adt32::C0(v1) => {
            Ok(Some(f325(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f326(v2)?))
        }
        Adt32::C2(v3) => {
            f327(f328(v3)?)
        }
        Adt32::C3(v4) => {
            f327(f328(match v4 {
                Adt49::C0(h1017, _) => {
                    h1017
                }
            })?)
        }
        Adt32::C4(v5) => {
            Ok(Some(f329(v5)?))
        }
        Adt32::C5(v6) => {
            Ok(Some(f330(v6)?))
        }
        Adt32::C6(v7) => {
            Ok(Some(f331(v7)?))
        }
        Adt32::C7(v8) => {
            Ok(Some(f332(v8)?))
        }
        Adt32::C8(v9) => {
            f327(f333(v9)?)
        }
        Adt32::C9(v10) => {
            Ok(Some(f334(v10)?))
        }
        Adt32::C10(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Adt32::C11 => {
            Ok(Some(List::<(Str, Adt6)>::nil()))
        }
        Adt32::C12(v12) => {
            f327(f335(v12)?)
        }
        Adt32::C13(_) => {
            Ok(None::<List<(Str, Adt6)>>)
        }
    }
}

pub fn f264(v0: Adt122) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f336(f337(match v0.clone() {
        Adt122::C0(h1018, _, _) => {
            h1018
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f338({
        let m1021 = match v0 {
            Adt122::C0(_, h1020, _) => {
                h1020
            }
        };
        match m1021 {
            Adt123::C0(h1019, _) => {
                h1019
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f265(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f339(v0, match v1.clone() {
        Adt88::C0(h1022, _, _, _) => {
            h1022
        }
    }, v2.clone()) {
        f340(match v1.clone() {
            Adt88::C0(h1023, _, _, _) => {
                h1023
            }
        }, {
            let a1025 = v2;
            let a1026 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1024) => {
                    h1024
                }
            }, List::<List<u64>>::nil());
            append_list(a1025, a1026)
        })
    } else {
        v1
    }
}

pub fn f266(v0: Adt124) -> R<Option<List<u64>>> {
    let m1027 = f341(f342(v0))?;
    match m1027 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1028 = f88(v1)?;
            match m1028 {
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

pub fn f267() -> u64 {
    404u64
}

pub fn f268(v0: Adt84) -> u64 {
    let m1029 = v0;
    match m1029 {
        Adt84::C0(_) => {
            f95()
        }
        Adt84::C1(_) => {
            f95()
        }
        Adt84::C2(_) => {
            f95()
        }
        Adt84::C3(_) => {
            f95()
        }
        Adt84::C4(_) => {
            f95()
        }
        Adt84::C5(_) => {
            f95()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f343()
        }
        Adt84::C8(_) => {
            f95()
        }
        Adt84::C9(_) => {
            f21()
        }
        Adt84::C10 => {
            f95()
        }
        Adt84::C11 => {
            f95()
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

pub fn f269(v0: Adt125) -> i64 {
    let m1030 = v0;
    match m1030 {
        Adt125::C0(_) => {
            f344()
        }
        Adt125::C1(_, _) => {
            f345()
        }
        Adt125::C2(_) => {
            f346()
        }
        Adt125::C3(_) => {
            f347()
        }
        Adt125::C4(v6) => {
            f348(v6)
        }
    }
}

pub fn f270(v0: Adt125) -> Str {
    let m1031 = v0;
    match m1031 {
        Adt125::C0(v1) => {
            f349(v1)
        }
        Adt125::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt125::C2(v4) => {
            f350(v4)
        }
        Adt125::C3(_) => {
            Str::lit("Method not found")
        }
        Adt125::C4(v6) => {
            let m1032 = v6;
            match m1032 {
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

pub fn f271(v0: Adt125) -> R<Option<Adt6>> {
    let m1033 = v0;
    match m1033 {
        Adt125::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f351(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt125::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt125::C4(v6) => {
            let m1034 = v6;
            match m1034 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f352(v15)), List::<(Str, Adt6)>::nil()))))
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

pub fn f272(v0: List<u64>) -> R<bool> {
    let m1035 = f208(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1035.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f353(v2)? {
                Ok(false)
            } else {
                f138(f140(v1), f354())
            }
        }
    }
}

pub fn f273(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt122, Adt125>> {
    f355(v0.clone(), v1, v2, v3.clone(), f356(match v0 {
        Adt0::C0(_, _, h1036, _, _, _, _) => {
            h1036
        }
    }, match v3 {
        Adt133::C0(_, h1037, _, _) => {
            h1037
        }
    }))
}

pub fn f274(v0: Adt136, v1: Adt137) -> Result<Adt133, Adt125> {
    let m1038 = v1;
    match m1038 {
        Adt137::C0(v3) => {
            f357(v3, v0)
        }
        Adt137::C1(v2) => {
            Err::<Adt133, Adt125>(v2)
        }
    }
}

pub fn f275(v0: Str) -> Adt136 {
    let a1039 = v0.clone();
    let a1040 = Str::lit("server/discover");
    if equal(a1039, a1040) {
        Adt136::C1(Adt134::C0)
    } else {
        let a1041 = v0.clone();
        let a1042 = Str::lit("tools/list");
        if equal(a1041, a1042) {
            Adt136::C1(Adt134::C1)
        } else {
            let a1043 = v0.clone();
            let a1044 = Str::lit("tools/call");
            if equal(a1043, a1044) {
                Adt136::C1(Adt134::C2)
            } else {
                let a1045 = v0.clone();
                let a1046 = Str::lit("resources/list");
                if equal(a1045, a1046) {
                    Adt136::C1(Adt134::C3)
                } else {
                    let a1047 = v0.clone();
                    let a1048 = Str::lit("resources/read");
                    if equal(a1047, a1048) {
                        Adt136::C1(Adt134::C4)
                    } else {
                        let a1049 = v0.clone();
                        let a1050 = Str::lit("resources/templates/list");
                        if equal(a1049, a1050) {
                            Adt136::C1(Adt134::C5)
                        } else {
                            let a1051 = v0.clone();
                            let a1052 = Str::lit("prompts/list");
                            if equal(a1051, a1052) {
                                Adt136::C1(Adt134::C6)
                            } else {
                                let a1053 = v0.clone();
                                let a1054 = Str::lit("prompts/get");
                                if equal(a1053, a1054) {
                                    Adt136::C1(Adt134::C7)
                                } else {
                                    let a1055 = v0.clone();
                                    let a1056 = Str::lit("completion/complete");
                                    if equal(a1055, a1056) {
                                        Adt136::C1(Adt134::C8)
                                    } else {
                                        let a1057 = v0.clone();
                                        let a1058 = Str::lit("tasks/get");
                                        if equal(a1057, a1058) {
                                            Adt136::C1(Adt134::C9)
                                        } else {
                                            let a1059 = v0.clone();
                                            let a1060 = Str::lit("tasks/update");
                                            if equal(a1059, a1060) {
                                                Adt136::C1(Adt134::C10)
                                            } else {
                                                let a1061 = v0;
                                                let a1062 = Str::lit("tasks/cancel");
                                                if equal(a1061, a1062) {
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

pub fn f276(v0: Option<Adt106>, v1: List<Str>, v2: Adt98) -> R<Adt137> {
    f358(v0, v1, v2.clone(), f359(match v2 {
        Adt98::C0(_, _, h1063) => {
            h1063
        }
    }))
}

pub fn f277(v0: List<u64>) -> Option<Str> {
    let a1064 = f360(f361(), v0);
    utf8_decode(a1064)
}

pub fn f278(v0: Str) -> Adt139 {
    let m1065 = f275(v0);
    match m1065 {
        Adt136::C0 => {
            Adt139::C0
        }
        Adt136::C1(v1) => {
            f362(v1)
        }
    }
}

pub fn f279(v0: List<u64>) -> Option<List<u64>> {
    let m1066 = f363(f364(), v0);
    match m1066 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1067 = f363(f365(), f148(v1, List::<u64>::nil()));
            match m1067 {
                None => {
                    None::<List<u64>>
                }
                Some(v2) => {
                    Some(f148(v2, List::<u64>::nil()))
                }
            }
        }
    }
}

pub fn f280(v0: List<u64>) -> R<Option<List<u64>>> {
    let m1069 = f366({
        let a1068 = v0.clone();
        length_list(a1068)
    }, v0, List::<u64>::nil())?;
    match m1069 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f160(v1, List::<u64>::nil())))
        }
    }
}

pub fn f281(p1072: List<Adt109>, p1073: List<Adt109>) -> List<Adt109> {
    let m1071 = (p1072, p1073);
    {
        let mut m1071 = m1071;
        loop {
            let step = {
                let (v0, v1) = m1071;
                let m1070 = v0;
                match m1070.uncons() {
                    None => {
                        Err::<(List<Adt109>, List<Adt109>), List<Adt109>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt109>, List<Adt109>), List<Adt109>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1071 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: List<Adt86>, p1080: List<Adt12>, v2: Option<Str>, p1081: List<Adt109>) -> R<List<Adt109>> {
    let m1079 = (p1080, p1081);
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let (v1, v3) = m1079;
                let m1074 = v1;
                match m1074.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f367(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>((v5, List::cons(Adt109::C0(match v4.clone() {
                                Adt12::C0(_, h1075, _, _) => {
                                    h1075
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1076, _) => {
                                    h1076
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1077) => {
                                    h1077
                                }
                            }, f368(v0.clone(), match v4 {
                                Adt12::C0(_, h1078, _, _) => {
                                    h1078
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt109>), R<List<Adt109>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f283(p1097: u64, p1098: i64, p1099: i64) -> R<Option<i64>> {
    let m1096 = (p1097, (p1098, p1099));
    {
        let mut m1096 = m1096;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1096;
                let m1082 = v0;
                if m1082 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1082 - 1;
                    if f369(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1085 = {
                            let a1083 = v2;
                            let a1084 = 0i64;
                            compare(a1083, a1084)
                        };
                        if match m1085 {
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
                            if f369({
                                let a1086 = v1;
                                let a1087 = 10i64;
                                let a1088 = 0i64;
                                int_rem(a1086, a1087, a1088)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1089 = v1;
                                    let a1090 = 10i64;
                                    let a1091 = 0i64;
                                    int_quot(a1089, a1090, a1091)?
                                }, f114(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1092 = v1;
                                let a1093 = 10i64;
                                int_mul(a1092, a1093)?
                            }, {
                                let a1094 = v2;
                                let a1095 = 1i64;
                                int_sub(a1094, a1095)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1096 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: i64) -> Option<i64> {
    let m1102 = {
        let a1100 = 9007199254740991i64;
        let a1101 = v0;
        compare(a1100, a1101)
    };
    if if match m1102 {
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
        let m1105 = {
            let a1103 = v0;
            let a1104 = -9007199254740991i64;
            compare(a1103, a1104)
        };
        match m1105 {
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

pub fn f285(v0: Str, v1: Str) -> Adt140 {
    let a1106 = v0;
    let a1107 = v1;
    if equal(a1106, a1107) {
        Adt140::C0
    } else {
        Adt140::C1
    }
}

pub fn f286(v0: Adt129) -> bool {
    let m1108 = v0;
    match m1108 {
        Adt129::C0 => {
            false
        }
        Adt129::C1(_) => {
            true
        }
    }
}

pub fn f287(v0: Adt92, v1: Str, v2: Adt5) -> Result<Adt98, Adt94> {
    let m1109 = f173(Str::lit("params"), v2);
    match m1109 {
        Adt129::C0 => {
            Ok::<Adt98, Adt94>(Adt98::C0(v0, v1, None::<Adt5>))
        }
        Adt129::C1(v3) => {
            let m1110 = f246(v3);
            match m1110 {
                None => {
                    f134(Some(v0), Adt95::C6)
                }
                Some(v4) => {
                    Ok::<Adt98, Adt94>(Adt98::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f288(v0: Adt117) -> bool {
    let m1111 = v0;
    match m1111 {
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

pub fn f289(v0: Adt113, v1: Adt118, v2: Adt114, v3: u64) -> R<Adt130> {
    let a1113 = match v0.clone() {
        Adt113::C0(_, _, h1112) => {
            h1112
        }
    };
    let a1114 = f322();
    if nat_lt(a1113, a1114) {
        Ok(Adt130::C1(Adt113::C0(v2, List::cons(v1, match v0.clone() {
            Adt113::C0(_, h1115, _) => {
                h1115
            }
        }), {
            let a1117 = match v0 {
                Adt113::C0(_, _, h1116) => {
                    h1116
                }
            };
            let a1118 = 1u64;
            nat_add(a1117, a1118)?
        })))
    } else {
        Ok(f236(Adt93::C9(v3)))
    }
}

pub fn f290(v0: bool, v1: Adt117, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt114 {
    Adt114::C8(Adt116::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f291(v0: u64) -> i64 {
    let a1119 = v0;
    let a1120 = 0u64;
    if nat_eq(a1119, a1120) {
        0i64
    } else {
        let a1121 = v0;
        let a1122 = 1u64;
        if nat_eq(a1121, a1122) {
            1i64
        } else {
            let a1123 = v0;
            let a1124 = 2u64;
            if nat_eq(a1123, a1124) {
                2i64
            } else {
                let a1125 = v0;
                let a1126 = 3u64;
                if nat_eq(a1125, a1126) {
                    3i64
                } else {
                    let a1127 = v0;
                    let a1128 = 4u64;
                    if nat_eq(a1127, a1128) {
                        4i64
                    } else {
                        let a1129 = v0;
                        let a1130 = 5u64;
                        if nat_eq(a1129, a1130) {
                            5i64
                        } else {
                            let a1131 = v0;
                            let a1132 = 6u64;
                            if nat_eq(a1131, a1132) {
                                6i64
                            } else {
                                let a1133 = v0;
                                let a1134 = 7u64;
                                if nat_eq(a1133, a1134) {
                                    7i64
                                } else {
                                    let a1135 = v0;
                                    let a1136 = 8u64;
                                    if nat_eq(a1135, a1136) {
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

pub fn f292(p1139: List<Adt6>, p1140: List<Adt6>) -> List<Adt6> {
    let m1138 = (p1139, p1140);
    {
        let mut m1138 = m1138;
        loop {
            let step = {
                let (v0, v1) = m1138;
                let m1137 = v0;
                match m1137.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1138 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f293(v0: Adt113, v1: u64) -> Adt130 {
    let m1142 = match v0.clone() {
        Adt113::C0(_, h1141, _) => {
            h1141
        }
    };
    match m1142.uncons() {
        None => {
            f236(Adt93::C2(v1))
        }
        Some((v2, _)) => {
            let m1143 = v2;
            match m1143 {
                Adt118::C0(_) => {
                    f235(v0, Adt114::C0)
                }
                Adt118::C1(_, _) => {
                    f235(v0, Adt114::C4)
                }
            }
        }
    }
}

pub fn f294(p1146: List<(Str, Adt6)>, p1147: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1145 = (p1146, p1147);
    {
        let mut m1145 = m1145;
        loop {
            let step = {
                let (v0, v1) = m1145;
                let m1144 = v0;
                match m1144.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Adt113, v1: Adt114) -> Adt113 {
    Adt113::C0(v1, match v0.clone() {
        Adt113::C0(_, h1148, _) => {
            h1148
        }
    }, match v0 {
        Adt113::C0(_, _, h1149) => {
            h1149
        }
    })
}

pub fn f296(v0: Adt113, v1: u64, v2: List<u64>, v3: bool) -> Adt130 {
    let m1151 = {
        let a1150 = f360(f361(), f160(v2, List::<u64>::nil()));
        utf8_decode(a1150)
    };
    match m1151 {
        None => {
            f236(Adt93::C14)
        }
        Some(v4) => {
            if v3 {
                f370(v0, v1, v4)
            } else {
                f302(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f297(v0: u64) -> u64 {
    let a1152 = v0;
    let a1153 = 34u64;
    if nat_eq(a1152, a1153) {
        34u64
    } else {
        let a1154 = v0;
        let a1155 = 92u64;
        if nat_eq(a1154, a1155) {
            92u64
        } else {
            let a1156 = v0;
            let a1157 = 47u64;
            if nat_eq(a1156, a1157) {
                47u64
            } else {
                let a1158 = v0;
                let a1159 = 98u64;
                if nat_eq(a1158, a1159) {
                    8u64
                } else {
                    let a1160 = v0;
                    let a1161 = 102u64;
                    if nat_eq(a1160, a1161) {
                        12u64
                    } else {
                        let a1162 = v0;
                        let a1163 = 110u64;
                        if nat_eq(a1162, a1163) {
                            10u64
                        } else {
                            let a1164 = v0;
                            let a1165 = 114u64;
                            if nat_eq(a1164, a1165) {
                                13u64
                            } else {
                                let a1166 = v0;
                                let a1167 = 116u64;
                                if nat_eq(a1166, a1167) {
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

pub fn f298(v0: u64) -> u64 {
    let a1168 = v0;
    let a1169 = 48u64;
    if nat_lt(a1168, a1169) {
        16u64
    } else {
        let a1170 = v0;
        let a1171 = 58u64;
        if nat_lt(a1170, a1171) {
            let a1172 = v0;
            let a1173 = 48u64;
            nat_sub(a1172, a1173)
        } else {
            let a1174 = v0;
            let a1175 = 65u64;
            if nat_lt(a1174, a1175) {
                16u64
            } else {
                let a1176 = v0;
                let a1177 = 71u64;
                if nat_lt(a1176, a1177) {
                    let a1178 = v0;
                    let a1179 = 55u64;
                    nat_sub(a1178, a1179)
                } else {
                    let a1180 = v0;
                    let a1181 = 97u64;
                    if nat_lt(a1180, a1181) {
                        16u64
                    } else {
                        let a1182 = v0;
                        let a1183 = 103u64;
                        if nat_lt(a1182, a1183) {
                            let a1184 = v0;
                            let a1185 = 87u64;
                            nat_sub(a1184, a1185)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f299(v0: Adt113, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt130> {
    let a1186 = v3;
    let a1187 = 0u64;
    if nat_eq(a1186, a1187) {
        if f227(v2, 55296u64, 56319u64) {
            Ok(f235(v0, Adt114::C6(Adt115::C3(v2), v4, v5)))
        } else {
            if f227(v2, 56320u64, 57343u64) {
                Ok(f236(Adt93::C6(v1)))
            } else {
                Ok(f235(v0, Adt114::C6(Adt115::C0, f371(v2, v4)?, v5)))
            }
        }
    } else {
        if f227(v2, 56320u64, 57343u64) {
            Ok(f235(v0, Adt114::C6(Adt115::C0, f371({
                let a1196 = 65536u64;
                let a1197 = {
                    let a1194 = {
                        let a1190 = {
                            let a1188 = v3;
                            let a1189 = 55296u64;
                            nat_sub(a1188, a1189)
                        };
                        let a1191 = 1024u64;
                        nat_mul(a1190, a1191)?
                    };
                    let a1195 = {
                        let a1192 = v2;
                        let a1193 = 56320u64;
                        nat_sub(a1192, a1193)
                    };
                    nat_add(a1194, a1195)?
                };
                nat_add(a1196, a1197)?
            }, v4)?, v5)))
        } else {
            Ok(f236(Adt93::C6(v1)))
        }
    }
}

pub fn f300(v0: u64, v1: u64) -> u64 {
    let m1200 = {
        let a1198 = f301(v0);
        let a1199 = v1;
        index_list(a1198, a1199)
    };
    match m1200 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f301(v0: u64) -> List<u64> {
    let a1201 = v0;
    let a1202 = 0u64;
    if nat_eq(a1201, a1202) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1203 = v0;
        let a1204 = 1u64;
        if nat_eq(a1203, a1204) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f302(v0: Adt113, v1: Adt6) -> Adt130 {
    Adt130::C1(f229(v1, match v0.clone() {
        Adt113::C0(_, h1205, _) => {
            h1205
        }
    }, match v0 {
        Adt113::C0(_, _, h1206) => {
            h1206
        }
    }))
}

pub fn f303(v0: u64) -> Adt6 {
    let a1207 = v0;
    let a1208 = 0u64;
    if nat_eq(a1207, a1208) {
        Adt6::C1(true)
    } else {
        let a1209 = v0;
        let a1210 = 1u64;
        if nat_eq(a1209, a1210) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f304(v0: Adt113, v1: Adt116, v2: Adt117) -> Adt130 {
    f235(v0, Adt114::C8(Adt116::C0(match v1.clone() {
        Adt116::C0(h1211, _, _, _, _, _, _) => {
            h1211
        }
    }, v2, match v1.clone() {
        Adt116::C0(_, _, h1212, _, _, _, _) => {
            h1212
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, h1213, _, _, _) => {
            h1213
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, _, h1214, _, _) => {
            h1214
        }
    }, match v1.clone() {
        Adt116::C0(_, _, _, _, _, h1215, _) => {
            h1215
        }
    }, match v1 {
        Adt116::C0(_, _, _, _, _, _, h1216) => {
            h1216
        }
    })))
}

pub fn f305(v0: Adt113, v1: u64, v2: u64, v3: Adt116, v4: bool) -> R<Adt130> {
    let v5: u64 = {
        let a1218 = match v3.clone() {
            Adt116::C0(_, _, _, h1217, _, _, _) => {
                h1217
            }
        };
        let a1219 = 0u64;
        if if nat_eq(a1218, a1219) {
            let a1220 = v2;
            let a1221 = 48u64;
            nat_eq(a1220, a1221)
        } else {
            false
        } {
            0u64
        } else {
            let a1223 = match v3.clone() {
                Adt116::C0(_, _, _, h1222, _, _, _) => {
                    h1222
                }
            };
            let a1224 = 1u64;
            nat_add(a1223, a1224)?
        }
    };
    let v6: i64 = if v4 {
        f114(match v3.clone() {
            Adt116::C0(_, _, _, _, h1225, _, _) => {
                h1225
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt116::C0(_, _, _, _, h1226, _, _) => {
                h1226
            }
        }
    };
    let a1227 = f372();
    let a1228 = v5;
    if nat_lt(a1227, a1228) {
        Ok(f236(Adt93::C8(v1)))
    } else {
        let m1231 = {
            let a1229 = 99999i64;
            let a1230 = v6;
            compare(a1229, a1230)
        };
        if match m1231 {
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
            Ok(f236(Adt93::C8(v1)))
        } else {
            Ok(f235(v0, f290(match v3.clone() {
                Adt116::C0(h1232, _, _, _, _, _, _) => {
                    h1232
                }
            }, if v4 {
                Adt117::C4
            } else {
                Adt117::C2
            }, f114({
                let a1234 = match v3.clone() {
                    Adt116::C0(_, _, h1233, _, _, _, _) => {
                        h1233
                    }
                };
                let a1235 = 10i64;
                int_mul(a1234, a1235)?
            }, f291({
                let a1236 = v2;
                let a1237 = 48u64;
                nat_sub(a1236, a1237)
            }))?, v5, v6, match v3.clone() {
                Adt116::C0(_, _, _, _, _, h1238, _) => {
                    h1238
                }
            }, match v3 {
                Adt116::C0(_, _, _, _, _, _, h1239) => {
                    h1239
                }
            })))
        }
    }
}

pub fn f306(v0: u64) -> bool {
    let a1240 = v0;
    let a1241 = 101u64;
    if nat_eq(a1240, a1241) {
        true
    } else {
        let a1242 = v0;
        let a1243 = 69u64;
        nat_eq(a1242, a1243)
    }
}

pub fn f307(v0: u64) -> bool {
    f227(v0, 48u64, 57u64)
}

pub fn f308(v0: Adt113, v1: u64, v2: u64, v3: Adt116) -> R<Adt130> {
    let m1244 = f228(v3)?;
    match m1244 {
        Adt135::C0 => {
            Ok(f236(Adt93::C7(v1)))
        }
        Adt135::C1 => {
            Ok(f236(Adt93::C8(v1)))
        }
        Adt135::C2(v4) => {
            Ok(f233(f229(v4, match v0.clone() {
                Adt113::C0(_, h1245, _) => {
                    h1245
                }
            }, match v0 {
                Adt113::C0(_, _, h1246) => {
                    h1246
                }
            }), v1, v2))
        }
    }
}

pub fn f309(v0: Adt113, v1: u64, v2: u64, v3: Adt116, v4: bool) -> R<Adt130> {
    let v5: i64 = f114({
        let a1248 = match v3.clone() {
            Adt116::C0(_, _, _, _, _, _, h1247) => {
                h1247
            }
        };
        let a1249 = 10i64;
        int_mul(a1248, a1249)?
    }, f291({
        let a1250 = v2;
        let a1251 = 48u64;
        nat_sub(a1250, a1251)
    }))?;
    let m1254 = {
        let a1252 = 99999i64;
        let a1253 = v5;
        compare(a1252, a1253)
    };
    if match m1254 {
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
        Ok(f236(Adt93::C8(v1)))
    } else {
        Ok(f235(v0, f290(match v3.clone() {
            Adt116::C0(h1255, _, _, _, _, _, _) => {
                h1255
            }
        }, Adt117::C7, match v3.clone() {
            Adt116::C0(_, _, h1256, _, _, _, _) => {
                h1256
            }
        }, match v3.clone() {
            Adt116::C0(_, _, _, h1257, _, _, _) => {
                h1257
            }
        }, match v3 {
            Adt116::C0(_, _, _, _, h1258, _, _) => {
                h1258
            }
        }, v4, v5)))
    }
}

pub fn f310(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1259 = f373(f374(), v2.clone());
    match m1259 {
        None => {
            let m1260 = f375(f376(), v2.clone());
            match m1260 {
                None => {
                    f377(v0, v1, v2)
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

pub fn f311(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt119) -> R<Adt141> {
    let m1261 = f223(Str::lit("x-mcp-header"), v1.clone());
    match m1261 {
        Adt129::C0 => {
            Ok(Adt141::C1(v4))
        }
        Adt129::C1(v5) => {
            f378(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f312(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt99>) -> List<Adt99> {
    f379(Str::lit("properties"), v0.clone(), v1, v2, f379(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f380(Str::lit("items"), v0.clone(), f380(Str::lit("additionalProperties"), v0.clone(), f381(Str::lit("allOf"), v0.clone(), f381(Str::lit("anyOf"), v0.clone(), f381(Str::lit("oneOf"), v0.clone(), f380(Str::lit("not"), v0.clone(), f380(Str::lit("if"), v0.clone(), f380(Str::lit("then"), v0.clone(), f380(Str::lit("else"), v0, v3)))))))))))
}

pub fn f313(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt131> {
    Ok(f382(List::cons((Str::lit("type"), f383(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f384(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f385(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f386(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f386(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f386(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f386(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a1262 = f387(v0.clone(), v1.clone())?;
        let a1263 = List::cons((Str::lit("required"), f388(v0, v1)), List::<(Str, Adt142)>::nil());
        append_list(a1262, a1263)
    })))))))))
}

pub fn f314(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> R<Adt131> {
    Ok(f389(f390(v0, v1.clone(), v2.clone(), v3.clone()), f389(f391(v1.clone(), v2.clone(), v3.clone()), f389(f392(v1.clone(), v2.clone(), v3.clone()), f389(f393(v1.clone(), v2.clone(), v3.clone())?, f389(f394(v1.clone(), v2.clone(), v3.clone()), f389(f395(Adt104::C0, f251(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f389(f395(Adt104::C1, f252(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f389(f395(Adt104::C2, f253(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f395(Adt104::C3, f254(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f315(v0: Adt6) -> Option<List<Adt6>> {
    let m1264 = v0;
    match m1264 {
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

pub fn f316(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1265 = f223(v0, v1);
    match m1265 {
        Adt129::C0 => {
            None::<Adt6>
        }
        Adt129::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f317(v0: Str) -> R<Str> {
    let m1268 = {
        let a1267 = f360(f361(), f160(f396(f85({
            let a1266 = v0.clone();
            utf8_encode(a1266)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1267)
    };
    match m1268 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f318(v0: List<Adt50>) -> R<bool> {
    let m1269 = v0;
    match m1269.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f397(v2, match v1 {
                Adt50::C0(h1270, _, _) => {
                    h1270
                }
            })
        }
    }
}

pub fn f319(v0: u64) -> List<u64> {
    f301(v0)
}

pub fn f320(v0: Adt7) -> R<bool> {
    let a1288 = {
        let m1274 = {
            let a1272 = f398();
            let a1273 = match v0.clone() {
                Adt7::C0(h1271, _) => {
                    h1271
                }
            };
            compare(a1272, a1273)
        };
        if match m1274 {
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
            let m1279 = {
                let a1277 = match v0.clone() {
                    Adt7::C0(h1275, _) => {
                        h1275
                    }
                };
                let a1278 = {
                    let a1276 = f398();
                    int_neg(a1276)?
                };
                compare(a1277, a1278)
            };
            if match m1279 {
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
                let m1283 = {
                    let a1281 = 9999i64;
                    let a1282 = match v0.clone() {
                        Adt7::C0(_, h1280) => {
                            h1280
                        }
                    };
                    compare(a1281, a1282)
                };
                if match m1283 {
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
                    let m1287 = {
                        let a1285 = match v0 {
                            Adt7::C0(_, h1284) => {
                                h1284
                            }
                        };
                        let a1286 = -9999i64;
                        compare(a1285, a1286)
                    };
                    match m1287 {
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
    Ok(bool_not(a1288))
}

pub fn f321(v0: Adt7) -> R<List<u64>> {
    let m1292 = {
        let a1290 = match v0.clone() {
            Adt7::C0(_, h1289) => {
                h1289
            }
        };
        let a1291 = 0i64;
        compare(a1290, a1291)
    };
    if if match m1292 {
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
        let m1296 = {
            let a1294 = 0i64;
            let a1295 = match v0.clone() {
                Adt7::C0(_, h1293) => {
                    h1293
                }
            };
            compare(a1294, a1295)
        };
        match m1296 {
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
        let a1299 = f399(match v0.clone() {
            Adt7::C0(h1297, _) => {
                h1297
            }
        })?;
        let a1300 = List::cons(101u64, f399(match v0 {
            Adt7::C0(_, h1298) => {
                h1298
            }
        })?);
        Ok(append_list(a1299, a1300))
    } else {
        f399(match v0 {
            Adt7::C0(h1301, _) => {
                h1301
            }
        })
    }
}

pub fn f322() -> u64 {
    512u64
}

pub fn f323(v0: u64, v1: Adt105, v2: List<Adt105>) -> List<Adt105> {
    List::cons(Adt105::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f324(p1304: List<u64>, p1305: List<u64>) -> R<List<u64>> {
    let m1303 = (p1304, p1305);
    {
        let mut m1303 = m1303;
        loop {
            let step = {
                let (v0, v1) = m1303;
                let m1302 = v0;
                match m1302.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f160(f400(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f401(match v0.clone() {
        Adt33::C0(_, h1306, _, _) => {
            h1306
        }
    })), List::cons((Str::lit("capabilities"), f402(match v0.clone() {
        Adt33::C0(_, _, h1307, _) => {
            h1307
        }
    })), {
        let a1310 = f403(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1308) => {
                h1308
            }
        });
        let a1311 = f404(match v0 {
            Adt33::C0(h1309, _, _, _) => {
                h1309
            }
        })?;
        append_list(a1310, a1311)
    })))
}

pub fn f326(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f405(match v0.clone() {
        Adt34::C0(_, _, h1312) => {
            h1312
        }
    }))), {
        let a1315 = f403(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1313, _) => {
                h1313
            }
        });
        let a1316 = f404(match v0 {
            Adt34::C0(h1314, _, _) => {
                h1314
            }
        })?;
        append_list(a1315, a1316)
    }))
}

pub fn f327(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1317 = v0;
    match m1317 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1318 = f341(Adt6::C5(v1))?;
            match m1318 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1319 = f246(v2);
                    match m1319 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1320) => {
                                    h1320
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f328(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1322 = f406(match v0.clone() {
        Adt37::C0(h1321, _, _) => {
            h1321
        }
    })?;
    match m1322 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1325 = f407(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1323, _) => {
                        h1323
                    }
                });
                let a1326 = f408(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1324) => {
                        h1324
                    }
                });
                append_list(a1325, a1326)
            })))
        }
    }
}

pub fn f329(v0: Adt51) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resources"), Adt6::C4(f409(match v0.clone() {
        Adt51::C0(_, _, h1327) => {
            h1327
        }
    })?)), {
        let a1330 = f403(Str::lit("nextCursor"), match v0.clone() {
            Adt51::C0(_, h1328, _) => {
                h1328
            }
        });
        let a1331 = f404(match v0 {
            Adt51::C0(h1329, _, _) => {
                h1329
            }
        })?;
        append_list(a1330, a1331)
    }))
}

pub fn f330(v0: Adt52) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("contents"), Adt6::C4(f410(match v0.clone() {
        Adt52::C0(_, h1332) => {
            h1332
        }
    }))), f404(match v0 {
        Adt52::C0(h1333, _) => {
            h1333
        }
    })?))
}

pub fn f331(v0: Adt53) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("resourceTemplates"), Adt6::C4(f411(match v0.clone() {
        Adt53::C0(_, _, h1334) => {
            h1334
        }
    }))), {
        let a1337 = f403(Str::lit("nextCursor"), match v0.clone() {
            Adt53::C0(_, h1335, _) => {
                h1335
            }
        });
        let a1338 = f404(match v0 {
            Adt53::C0(h1336, _, _) => {
                h1336
            }
        })?;
        append_list(a1337, a1338)
    }))
}

pub fn f332(v0: Adt55) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("prompts"), Adt6::C4(f412(match v0.clone() {
        Adt55::C0(_, _, h1339) => {
            h1339
        }
    }))), {
        let a1342 = f403(Str::lit("nextCursor"), match v0.clone() {
            Adt55::C0(_, h1340, _) => {
                h1340
            }
        });
        let a1343 = f404(match v0 {
            Adt55::C0(h1341, _, _) => {
                h1341
            }
        })?;
        append_list(a1342, a1343)
    }))
}

pub fn f333(v0: Adt58) -> R<Option<List<(Str, Adt6)>>> {
    let m1345 = f413(match v0.clone() {
        Adt58::C0(_, h1344) => {
            h1344
        }
    })?;
    match m1345 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1347 = f403(Str::lit("description"), match v0 {
                    Adt58::C0(h1346, _) => {
                        h1346
                    }
                });
                let a1348 = List::cons((Str::lit("messages"), Adt6::C4(v1)), List::<(Str, Adt6)>::nil());
                append_list(a1347, a1348)
            }))
        }
    }
}

pub fn f334(v0: Adt60) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("completion"), Adt6::C5(List::cons((Str::lit("values"), Adt6::C4(f351({
        let m1351 = match v0.clone() {
            Adt60::C0(h1350) => {
                h1350
            }
        };
        match m1351 {
            Adt61::C0(h1349, _, _) => {
                h1349
            }
        }
    }))), {
        let a1358 = f414(Str::lit("total"), {
            let m1354 = match v0.clone() {
                Adt60::C0(h1353) => {
                    h1353
                }
            };
            match m1354 {
                Adt61::C0(_, h1352, _) => {
                    h1352
                }
            }
        })?;
        let a1359 = f408(Str::lit("hasMore"), {
            let m1357 = match v0 {
                Adt60::C0(h1356) => {
                    h1356
                }
            };
            match m1357 {
                Adt61::C0(_, _, h1355) => {
                    h1355
                }
            }
        });
        append_list(a1358, a1359)
    }))), List::<(Str, Adt6)>::nil()))
}

pub fn f335(v0: Adt81) -> R<Option<List<(Str, Adt6)>>> {
    let m1360 = f415(f416(v0.clone()), List::<(Str, Adt6)>::nil())?;
    match m1360 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some({
                let a1361 = f417(v1);
                let a1362 = f418(f419(v0));
                append_list(a1361, a1362)
            }))
        }
    }
}

pub fn f336(v0: Adt143) -> Str {
    let m1363 = v0;
    match m1363 {
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

pub fn f337(v0: Adt32) -> Adt143 {
    let m1364 = v0;
    match m1364 {
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

pub fn f338(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1365 = v0;
    match m1365 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f420(), f421(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f339(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f422(v2) {
            let a1366 = v1;
            let a1367 = 200u64;
            nat_eq(a1366, a1367)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f340(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f423(), List::<List<u64>>::nil(), f424(v1))
}

pub fn f341(v0: Adt6) -> R<Option<Adt6>> {
    f425(f426(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f342(v0: Adt124) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f427(match v0.clone() {
        Adt124::C0(h1368, _) => {
            h1368
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1371 = match v0.clone() {
            Adt124::C0(_, h1370) => {
                h1370
            }
        };
        match m1371 {
            Adt50::C0(h1369, _, _) => {
                h1369
            }
        }
    })), {
        let a1378 = f428({
            let m1374 = match v0.clone() {
                Adt124::C0(_, h1373) => {
                    h1373
                }
            };
            match m1374 {
                Adt50::C0(_, h1372, _) => {
                    h1372
                }
            }
        });
        let a1379 = f429({
            let m1377 = match v0 {
                Adt124::C0(_, h1376) => {
                    h1376
                }
            };
            match m1377 {
                Adt50::C0(_, _, h1375) => {
                    h1375
                }
            }
        });
        append_list(a1378, a1379)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f343() -> u64 {
    429u64
}

pub fn f344() -> i64 {
    -32602i64
}

pub fn f345() -> i64 {
    -32022i64
}

pub fn f346() -> i64 {
    -32020i64
}

pub fn f347() -> i64 {
    -32601i64
}

pub fn f348(v0: Adt84) -> i64 {
    let m1380 = v0;
    match m1380 {
        Adt84::C0(_) => {
            f344()
        }
        Adt84::C1(_) => {
            f344()
        }
        Adt84::C2(_) => {
            f344()
        }
        Adt84::C3(_) => {
            f344()
        }
        Adt84::C4(_) => {
            f344()
        }
        Adt84::C5(_) => {
            f344()
        }
        Adt84::C6(_) => {
            f430()
        }
        Adt84::C7(_) => {
            f431()
        }
        Adt84::C8(_) => {
            f432()
        }
        Adt84::C9(_) => {
            f430()
        }
        Adt84::C10 => {
            f344()
        }
        Adt84::C11 => {
            f344()
        }
        Adt84::C12(_) => {
            f430()
        }
        Adt84::C13 => {
            f430()
        }
        Adt84::C14 => {
            f430()
        }
    }
}

pub fn f349(v0: Adt126) -> Str {
    let m1381 = v0;
    match m1381 {
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

pub fn f350(v0: Adt107) -> Str {
    let m1382 = v0;
    match m1382 {
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
            let m1383 = v1;
            match m1383 {
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
            let m1384 = v2;
            match m1384 {
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

pub fn f351(v0: List<Str>) -> List<Adt6> {
    f292(f433(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f352(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1396 = f434(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1385, _, _, _, _) => {
                h1385
            }
        });
        let a1397 = {
            let a1394 = f434(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1386, _, _, _) => {
                    h1386
                }
            });
            let a1395 = {
                let a1392 = f434(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1387, _, _) => {
                        h1387
                    }
                });
                let a1393 = {
                    let a1390 = f434(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1388, _) => {
                            h1388
                        }
                    });
                    let a1391 = f434(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1389) => {
                            h1389
                        }
                    });
                    append_list(a1390, a1391)
                };
                append_list(a1392, a1393)
            };
            append_list(a1394, a1395)
        };
        append_list(a1396, a1397)
    })
}

pub fn f353(p1400: List<List<u64>>) -> R<bool> {
    let m1399 = p1400;
    {
        let mut m1399 = m1399;
        loop {
            let step = {
                let v0 = m1399;
                let m1398 = v0;
                match m1398.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f435(f140(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
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

pub fn f354() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f355(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt122, Adt125>> {
    if v4 {
        f436(v0, v1, v2, f437(match v3.clone() {
            Adt133::C0(_, h1401, _, _) => {
                h1401
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1402, _) => {
                h1402
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1403) => {
                h1403
            }
        }))
    } else {
        Ok(Err::<Adt122, Adt125>(Adt125::C3(f438(match v3 {
            Adt133::C0(_, h1404, _, _) => {
                h1404
            }
        }))))
    }
}

pub fn f356(v0: Adt4, v1: Adt134) -> bool {
    let m1405 = v1;
    match m1405 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f439(match v0 {
                Adt4::C0(_, _, _, _, _, h1406, _) => {
                    h1406
                }
            })
        }
        Adt134::C2 => {
            f439(match v0 {
                Adt4::C0(_, _, _, _, _, h1407, _) => {
                    h1407
                }
            })
        }
        Adt134::C3 => {
            f440(match v0 {
                Adt4::C0(_, _, _, _, h1408, _, _) => {
                    h1408
                }
            })
        }
        Adt134::C4 => {
            f440(match v0 {
                Adt4::C0(_, _, _, _, h1409, _, _) => {
                    h1409
                }
            })
        }
        Adt134::C5 => {
            f440(match v0 {
                Adt4::C0(_, _, _, _, h1410, _, _) => {
                    h1410
                }
            })
        }
        Adt134::C6 => {
            f439(match v0 {
                Adt4::C0(_, _, _, h1411, _, _, _) => {
                    h1411
                }
            })
        }
        Adt134::C7 => {
            f439(match v0 {
                Adt4::C0(_, _, _, h1412, _, _, _) => {
                    h1412
                }
            })
        }
        Adt134::C8 => {
            f441(match v0 {
                Adt4::C0(_, _, h1413, _, _, _, _) => {
                    h1413
                }
            })
        }
        Adt134::C9 => {
            f442(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1414) => {
                    h1414
                }
            }, f443())
        }
        Adt134::C10 => {
            f442(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1415) => {
                    h1415
                }
            }, f443())
        }
        Adt134::C11 => {
            f442(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1416) => {
                    h1416
                }
            }, f443())
        }
    }
}

pub fn f357(v0: Adt138, v1: Adt136) -> Result<Adt133, Adt125> {
    let m1417 = v1;
    match m1417 {
        Adt136::C0 => {
            Err::<Adt133, Adt125>(Adt125::C3(match v0 {
                Adt138::C0(_, h1418, _, _) => {
                    h1418
                }
            }))
        }
        Adt136::C1(v2) => {
            Ok::<Adt133, Adt125>(Adt133::C0(match v0.clone() {
                Adt138::C0(h1419, _, _, _) => {
                    h1419
                }
            }, v2, match v0.clone() {
                Adt138::C0(_, _, h1420, _) => {
                    h1420
                }
            }, match v0 {
                Adt138::C0(_, _, _, h1421) => {
                    h1421
                }
            }))
        }
    }
}

pub fn f358(v0: Option<Adt106>, v1: List<Str>, v2: Adt98, v3: Result<Adt16, Adt126>) -> R<Adt137> {
    let m1422 = v3;
    match m1422 {
        Ok(v5) => {
            Ok(f444(v1, v2.clone(), v5.clone(), f445(v0, f278(match v2.clone() {
                Adt98::C0(_, h1423, _) => {
                    h1423
                }
            }), match v2.clone() {
                Adt98::C0(_, h1424, _) => {
                    h1424
                }
            }, match v5 {
                Adt16::C0(h1425, _, _, _, _, _) => {
                    h1425
                }
            }, f446(match v2 {
                Adt98::C0(_, _, h1426) => {
                    h1426
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt137::C1(Adt125::C0(v4)))
        }
    }
}

pub fn f359(v0: Option<Adt5>) -> Result<Adt16, Adt126> {
    let m1427 = v0;
    match m1427 {
        None => {
            Err::<Adt16, Adt126>(Adt126::C0)
        }
        Some(v1) => {
            let m1428 = f173(Str::lit("_meta"), v1);
            match m1428 {
                Adt129::C0 => {
                    Err::<Adt16, Adt126>(Adt126::C1)
                }
                Adt129::C1(v2) => {
                    let m1429 = f246(v2);
                    match m1429 {
                        None => {
                            Err::<Adt16, Adt126>(Adt126::C4)
                        }
                        Some(v3) => {
                            f447(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f360(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1430 = v0;
    let a1431 = f448(f361(), f449(List::<Bytes>::nil(), v1));
    append_bytes(a1430, a1431)
}

pub fn f361() -> Bytes {
    Bytes::lit(&[])
}

pub fn f362(v0: Adt134) -> Adt139 {
    let m1432 = v0;
    match m1432 {
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

pub fn f363(p1438: List<u64>, p1439: List<u64>) -> Option<List<u64>> {
    let m1437 = (p1438, p1439);
    {
        let mut m1437 = m1437;
        loop {
            let step = {
                let (v0, v1) = m1437;
                let m1433 = v0;
                match m1433.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1434 = v1;
                        match m1434.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1435 = v4;
                                let a1436 = v2;
                                if nat_eq(a1435, a1436) {
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
                Ok(next) => { m1437 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f364() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f365() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f366(p1448: u64, p1449: List<u64>, p1450: List<u64>) -> R<Option<List<u64>>> {
    let m1447 = (p1448, (p1449, p1450));
    {
        let mut m1447 = m1447;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1447;
                let m1440 = v0;
                if m1440 == 0 {
                    let m1441 = v1;
                    match m1441.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1440 - 1;
                    let m1442 = v1;
                    match m1442.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1443 = v7;
                            match m1443.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1444 = v9;
                                    match m1444.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1445 = v11;
                                            match m1445.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1446 = f450(v6, v8, v10, v12, f451(v13.clone()))?;
                                                    match m1446 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f160(v14, v2))))
                                                        }
                                                    }
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
                Ok(next) => { m1447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Adt12, v1: Option<Str>) -> bool {
    let m1451 = v1;
    match m1451 {
        None => {
            false
        }
        Some(v2) => {
            let a1453 = match v0 {
                Adt12::C0(h1452, _, _, _) => {
                    h1452
                }
            };
            let a1454 = v2;
            equal(a1453, a1454)
        }
    }
}

pub fn f368(v0: List<Adt86>, v1: Str) -> R<Adt110> {
    let m1457 = f215(f71(v0, {
        let a1455 = f452();
        let a1456 = f453(v1)?;
        append_list(a1455, a1456)
    })?)?;
    match m1457 {
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
            let m1458 = f277(v2);
            match m1458 {
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

pub fn f369(v0: i64) -> bool {
    let a1465 = {
        let m1461 = {
            let a1459 = v0;
            let a1460 = 0i64;
            compare(a1459, a1460)
        };
        if match m1461 {
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
            let m1464 = {
                let a1462 = 0i64;
                let a1463 = v0;
                compare(a1462, a1463)
            };
            match m1464 {
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
    bool_not(a1465)
}

pub fn f370(v0: Adt113, v1: u64, v2: Str) -> Adt130 {
    let m1467 = match v0.clone() {
        Adt113::C0(_, h1466, _) => {
            h1466
        }
    };
    match m1467.uncons() {
        None => {
            f236(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1468 = v3;
            match m1468 {
                Adt118::C0(_) => {
                    f236(Adt93::C2(v1))
                }
                Adt118::C1(v6, _) => {
                    Adt130::C1(Adt113::C0(Adt114::C5, List::cons(Adt118::C1(v6, Some(v2)), v4), match v0 {
                        Adt113::C0(_, _, h1469) => {
                            h1469
                        }
                    }))
                }
            }
        }
    }
}

pub fn f371(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1470 = v0;
    let a1471 = 128u64;
    if nat_lt(a1470, a1471) {
        Ok(List::cons(v0, v1))
    } else {
        let a1472 = v0;
        let a1473 = 2048u64;
        if nat_lt(a1472, a1473) {
            Ok(List::cons({
                let a1474 = 128u64;
                let a1475 = f454(v0, 64u64);
                nat_add(a1474, a1475)?
            }, List::cons({
                let a1476 = 192u64;
                let a1477 = f455(v0, 64u64);
                nat_add(a1476, a1477)?
            }, v1)))
        } else {
            let a1478 = v0;
            let a1479 = 65536u64;
            if nat_lt(a1478, a1479) {
                Ok(List::cons({
                    let a1480 = 128u64;
                    let a1481 = f454(v0, 64u64);
                    nat_add(a1480, a1481)?
                }, List::cons({
                    let a1482 = 128u64;
                    let a1483 = f454(f455(v0, 64u64), 64u64);
                    nat_add(a1482, a1483)?
                }, List::cons({
                    let a1484 = 224u64;
                    let a1485 = f455(v0, 4096u64);
                    nat_add(a1484, a1485)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1486 = 128u64;
                    let a1487 = f454(v0, 64u64);
                    nat_add(a1486, a1487)?
                }, List::cons({
                    let a1488 = 128u64;
                    let a1489 = f454(f455(v0, 64u64), 64u64);
                    nat_add(a1488, a1489)?
                }, List::cons({
                    let a1490 = 128u64;
                    let a1491 = f454(f455(v0, 4096u64), 64u64);
                    nat_add(a1490, a1491)?
                }, List::cons({
                    let a1492 = 240u64;
                    let a1493 = f455(v0, 262144u64);
                    nat_add(a1492, a1493)?
                }, v1)))))
            }
        }
    }
}

pub fn f372() -> u64 {
    18u64
}

pub fn f373(p1497: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1496 = p1497;
    {
        let mut m1496 = m1496;
        loop {
            let step = {
                let v0 = m1496;
                let m1494 = v0;
                match m1494.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1495 = f223(v2.clone(), v1.clone());
                        match m1495 {
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
                Ok(next) => { m1496 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f375(p1504: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1503 = p1504;
    {
        let mut m1503 = m1503;
        loop {
            let step = {
                let v0 = m1503;
                let m1498 = v0;
                match m1498.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1500 = f223({
                            let (h1499, _) = v2.clone();
                            h1499
                        }, v1.clone());
                        match m1500 {
                            Adt129::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt129::C1(v4) => {
                                if f456({
                                    let (_, h1501) = v2;
                                    h1501
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1502, _) = v2;
                                        h1502
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1503 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f376() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f377(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1505 = f223(Str::lit("$ref"), v2);
    match m1505 {
        Adt129::C0 => {
            None::<Adt91>
        }
        Adt129::C1(v3) => {
            let m1506 = f220(v3);
            match m1506 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1507 = f457(v1, v4.clone());
                    match m1507 {
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

pub fn f378(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt119) -> R<Adt141> {
    if v3 {
        let m1508 = f220(v2);
        match m1508 {
            None => {
                Ok(Adt141::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f458(v0, v1, v6.clone(), f112(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt141::C0(Adt91::C10(v0)))
    }
}

pub fn f379(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt99>) -> List<Adt99> {
    let m1509 = f223(v0.clone(), v1);
    match m1509 {
        Adt129::C0 => {
            v4
        }
        Adt129::C1(v5) => {
            let m1510 = f246(v5);
            match m1510 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1511 = v0;
                    let a1512 = Str::lit("properties");
                    if equal(a1511, a1512) {
                        f459(match v6 {
                            Adt5::C0(h1513) => {
                                h1513
                            }
                        }, v2, v3, v4)
                    } else {
                        f460(match v6 {
                            Adt5::C0(h1514) => {
                                h1514
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f380(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt99>) -> List<Adt99> {
    let m1515 = f223(v0, v1);
    match m1515 {
        Adt129::C0 => {
            v2
        }
        Adt129::C1(v3) => {
            List::cons(Adt99::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f381(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt99>) -> List<Adt99> {
    let m1516 = f223(v0, v1);
    match m1516 {
        Adt129::C0 => {
            v2
        }
        Adt129::C1(v3) => {
            let m1517 = f315(v3);
            match m1517 {
                None => {
                    v2
                }
                Some(v4) => {
                    f461(v4, v2)
                }
            }
        }
    }
}

pub fn f382(p1524: List<(Str, Adt142)>) -> Adt131 {
    let m1523 = p1524;
    {
        let mut m1523 = m1523;
        loop {
            let step = {
                let v0 = m1523;
                let m1518 = v0;
                match m1518.uncons() {
                    None => {
                        Err::<List<(Str, Adt142)>, Adt131>(Adt131::C3(List::<Adt103>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1520 = {
                            let (_, h1519) = v1;
                            h1519
                        };
                        match m1520 {
                            Adt142::C0 => {
                                Ok::<List<(Str, Adt142)>, Adt131>(v2)
                            }
                            Adt142::C1 => {
                                Err::<List<(Str, Adt142)>, Adt131>(Adt131::C0({
                                    let (h1521, _) = v1;
                                    h1521
                                }))
                            }
                            Adt142::C2 => {
                                Err::<List<(Str, Adt142)>, Adt131>(Adt131::C1({
                                    let (h1522, _) = v1;
                                    h1522
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1525 = f223(Str::lit("type"), v0);
    match m1525 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1526 = f220(v2.clone());
            match m1526 {
                None => {
                    let m1527 = f315(v2);
                    match m1527 {
                        None => {
                            Ok(Adt142::C2)
                        }
                        Some(v4) => {
                            f464(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f462(v3.clone()) {
                        if f463(v3, v1)? {
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

pub fn f384(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1528 = f223(Str::lit("enum"), v0);
    match m1528 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1529 = f315(v2);
            match m1529 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v3) => {
                    f465(v3, v1)
                }
            }
        }
    }
}

pub fn f385(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt142> {
    let m1530 = f223(Str::lit("const"), v0);
    match m1530 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v2) => {
            let m1531 = f466(v2, v1)?;
            match m1531 {
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

pub fn f386(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt142> {
    let m1532 = f223(v0, v2);
    match m1532 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v4) => {
            let m1533 = f467(v4);
            match m1533 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v5) => {
                    let m1534 = f467(v3);
                    match m1534 {
                        None => {
                            Ok(Adt142::C0)
                        }
                        Some(v6) => {
                            if f468(v1, f469(v6, v5)?) {
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

pub fn f387(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt142)>> {
    let m1535 = v1;
    match m1535 {
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
            Ok(List::cons((Str::lit("minLength"), f470(Str::lit("minLength"), true, v0.clone(), f471(v4.clone())?)?), List::cons((Str::lit("maxLength"), f470(Str::lit("maxLength"), false, v0, f471(v4)?)?), List::<(Str, Adt142)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f470(Str::lit("minItems"), true, v0.clone(), f472(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f470(Str::lit("maxItems"), false, v0, f472(v5, 0u64)?)?), List::<(Str, Adt142)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f470(Str::lit("minProperties"), true, v0.clone(), f473(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f470(Str::lit("maxProperties"), false, v0, f473(v6, 0u64)?)?), List::<(Str, Adt142)>::nil())))
        }
    }
}

pub fn f388(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt142 {
    let m1536 = f223(Str::lit("required"), v0);
    match m1536 {
        Adt129::C0 => {
            Adt142::C0
        }
        Adt129::C1(v2) => {
            let m1537 = f315(v2);
            match m1537 {
                None => {
                    Adt142::C2
                }
                Some(v3) => {
                    let m1538 = f246(v1);
                    match m1538 {
                        None => {
                            Adt142::C0
                        }
                        Some(v4) => {
                            f474(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f389(v0: Adt131, v1: Adt131) -> Adt131 {
    let m1539 = v0.clone();
    match m1539 {
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
            let m1540 = v1.clone();
            match m1540 {
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
                        let a1541 = v5;
                        let a1542 = v9;
                        append_list(a1541, a1542)
                    })
                }
            }
        }
    }
}

pub fn f390(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>) -> Adt131 {
    let m1543 = f223(Str::lit("$ref"), v1);
    match m1543 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v4) => {
            let m1544 = f220(v4);
            match m1544 {
                None => {
                    Adt131::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1545 = f457(v0, v5.clone());
                    match m1545 {
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

pub fn f391(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1546 = f223(Str::lit("properties"), v0);
    match m1546 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1547 = f246(v3);
            match m1547 {
                None => {
                    Adt131::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1548 = f246(v1);
                    match m1548 {
                        None => {
                            Adt131::C3(List::<Adt103>::nil())
                        }
                        Some(v5) => {
                            Adt131::C3(f475(match v4 {
                                Adt5::C0(h1549) => {
                                    h1549
                                }
                            }, match v5 {
                                Adt5::C0(h1550) => {
                                    h1550
                                }
                            }, v2, List::<Adt103>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f392(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1551 = f223(Str::lit("additionalProperties"), v0.clone());
    match m1551 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1552 = f246(v1);
            match m1552 {
                None => {
                    Adt131::C3(List::<Adt103>::nil())
                }
                Some(v4) => {
                    let m1553 = f245(v3.clone());
                    match m1553 {
                        None => {
                            Adt131::C3(f478(match v4 {
                                Adt5::C0(h1555) => {
                                    h1555
                                }
                            }, f477(v0), v3, v2, List::<Adt103>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt131::C3(List::<Adt103>::nil())
                            } else {
                                if f476(match v4 {
                                    Adt5::C0(h1554) => {
                                        h1554
                                    }
                                }, f477(v0)) {
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

pub fn f393(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> R<Adt131> {
    let m1556 = f223(Str::lit("items"), v0);
    match m1556 {
        Adt129::C0 => {
            Ok(Adt131::C3(List::<Adt103>::nil()))
        }
        Adt129::C1(v3) => {
            let m1557 = f315(v1);
            match m1557 {
                None => {
                    Ok(Adt131::C3(List::<Adt103>::nil()))
                }
                Some(v4) => {
                    Ok(Adt131::C3(f479(v4, 0u64, v3, v2, List::<Adt103>::nil())?))
                }
            }
        }
    }
}

pub fn f394(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt101>) -> Adt131 {
    let m1558 = f223(Str::lit("allOf"), v0);
    match m1558 {
        Adt129::C0 => {
            Adt131::C3(List::<Adt103>::nil())
        }
        Adt129::C1(v3) => {
            let m1559 = f315(v3);
            match m1559 {
                None => {
                    Adt131::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1560 = v4.clone();
                    match m1560.uncons() {
                        None => {
                            Adt131::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt131::C3(f480(v4, v1, v2, List::<Adt103>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f395(v0: Adt104, v1: Adt131, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt101>) -> Adt131 {
    let m1561 = v1.clone();
    match m1561 {
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
            let m1562 = v8;
            match m1562.uncons() {
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

pub fn f396(p1569: List<u64>, p1570: List<u64>) -> List<u64> {
    let m1568 = (p1569, p1570);
    {
        let mut m1568 = m1568;
        loop {
            let step = {
                let (v0, v1) = m1568;
                let m1563 = v0;
                match m1563.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1564 = v2;
                        let a1565 = 126u64;
                        if nat_eq(a1564, a1565) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1566 = v2;
                            let a1567 = 47u64;
                            if nat_eq(a1566, a1567) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(p1575: List<Adt50>, p1576: Adt7) -> R<bool> {
    let m1574 = (p1575, p1576);
    {
        let mut m1574 = m1574;
        loop {
            let step = {
                let (v0, v1) = m1574;
                let m1571 = v0;
                match m1571.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f481(f469(match v2.clone() {
                            Adt50::C0(h1572, _, _) => {
                                h1572
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1573, _, _) => {
                                    h1573
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398() -> i64 {
    999999999999999999i64
}

pub fn f399(v0: i64) -> R<List<u64>> {
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
        Ok(List::cons(45u64, f482(20u64, {
            let a1580 = v0;
            int_neg(a1580)?
        }, List::<u64>::nil())?))
    } else {
        f482(20u64, v0, List::<u64>::nil())
    }
}

pub fn f400(v0: u64) -> R<List<u64>> {
    let a1581 = v0;
    let a1582 = 34u64;
    if nat_eq(a1581, a1582) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1583 = v0;
        let a1584 = 92u64;
        if nat_eq(a1583, a1584) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1585 = v0;
            let a1586 = 8u64;
            if nat_eq(a1585, a1586) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1587 = v0;
                let a1588 = 9u64;
                if nat_eq(a1587, a1588) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1589 = v0;
                    let a1590 = 10u64;
                    if nat_eq(a1589, a1590) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1591 = v0;
                        let a1592 = 12u64;
                        if nat_eq(a1591, a1592) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1593 = v0;
                            let a1594 = 13u64;
                            if nat_eq(a1593, a1594) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1595 = v0;
                                let a1596 = 32u64;
                                if nat_lt(a1595, a1596) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f483(f455(v0, 16u64))?, List::cons(f483(f454(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f401(v0: List<Str>) -> Adt6 {
    Adt6::C4(f351(v0))
}

pub fn f402(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1614 = f434(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1597, _, _, _, _, _, _) => {
                h1597
            }
        });
        let a1615 = {
            let a1612 = f434(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1598, _, _, _, _, _) => {
                    h1598
                }
            });
            let a1613 = {
                let a1610 = f434(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1599, _, _, _, _) => {
                        h1599
                    }
                });
                let a1611 = {
                    let a1608 = f484(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1600, _, _, _) => {
                            h1600
                        }
                    });
                    let a1609 = {
                        let a1606 = f485(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1601, _, _) => {
                                h1601
                            }
                        });
                        let a1607 = {
                            let a1604 = f484(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1602, _) => {
                                    h1602
                                }
                            });
                            let a1605 = f434(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1603) => {
                                    h1603
                                }
                            });
                            append_list(a1604, a1605)
                        };
                        append_list(a1606, a1607)
                    };
                    append_list(a1608, a1609)
                };
                append_list(a1610, a1611)
            };
            append_list(a1612, a1613)
        };
        append_list(a1614, a1615)
    })
}

pub fn f403(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1616 = v1;
    match m1616 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f404(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f486(match v0.clone() {
        Adt10::C0(h1617, _) => {
            h1617
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f487(match v0 {
        Adt10::C0(_, h1618) => {
            h1618
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f405(v0: List<Adt35>) -> List<Adt6> {
    f292(f488(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f406(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1619 = f489(v0, List::<Adt6>::nil())?;
    match m1619 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f292(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f407(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1620 = v1;
    match m1620 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f408(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1621 = v1;
    match m1621 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f409(v0: List<Adt44>) -> R<List<Adt6>> {
    Ok(f292(f490(v0, List::<Adt6>::nil())?, List::<Adt6>::nil()))
}

pub fn f410(v0: List<Adt46>) -> List<Adt6> {
    f292(f491(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f411(v0: List<Adt54>) -> List<Adt6> {
    f292(f492(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f412(v0: List<Adt56>) -> List<Adt6> {
    f292(f493(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f413(v0: List<Adt59>) -> R<Option<List<Adt6>>> {
    let m1622 = f494(v0, List::<Adt6>::nil())?;
    match m1622 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f292(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f414(v0: Str, v1: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m1623 = v1;
    match m1623 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v2) => {
            Ok(List::cons((v0, f486(v2)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f415(p1629: List<(Str, Adt64)>, p1630: List<(Str, Adt6)>) -> R<Option<List<(Str, Adt6)>>> {
    let m1628 = (p1629, p1630);
    {
        let mut m1628 = m1628;
        loop {
            let step = {
                let (v0, v1) = m1628;
                let m1624 = v0;
                match m1624.uncons() {
                    None => {
                        Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1626 = f495({
                            let (_, h1625) = v2;
                            h1625
                        })?;
                        match m1626 {
                            None => {
                                Err::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>(Ok(None::<List<(Str, Adt6)>>))
                            }
                            Some(v4) => {
                                Ok::<(List<(Str, Adt64)>, List<(Str, Adt6)>), R<Option<List<(Str, Adt6)>>>>((v3, List::cons(({
                                    let (h1627, _) = v2;
                                    h1627
                                }, v4), v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f416(v0: Adt81) -> List<(Str, Adt64)> {
    let m1631 = v0;
    match m1631 {
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

pub fn f417(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1632 = v0.clone();
    match m1632.uncons() {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some((_, _)) => {
            List::cons((Str::lit("inputRequests"), Adt6::C5(f294(v0, List::<(Str, Adt6)>::nil()))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f418(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1633 = v0;
    match m1633 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("requestState"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f419(v0: Adt81) -> Option<Str> {
    let m1634 = v0;
    match m1634 {
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

pub fn f420() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f421(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1635, _, _, _, _, _) => {
            h1635
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1636, _, _, _) => {
            h1636
        }
    })), {
        let a1645 = f403(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1637, _, _, _, _) => {
                h1637
            }
        });
        let a1646 = {
            let a1643 = f403(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1638, _, _) => {
                    h1638
                }
            });
            let a1644 = {
                let a1641 = f403(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1639, _) => {
                        h1639
                    }
                });
                let a1642 = f496(match v0 {
                    Adt1::C0(_, _, _, _, _, h1640) => {
                        h1640
                    }
                });
                append_list(a1641, a1642)
            };
            append_list(a1643, a1644)
        };
        append_list(a1645, a1646)
    })))
}

pub fn f422(v0: List<List<u64>>) -> bool {
    let m1647 = v0;
    match m1647.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f423() -> List<Adt86> {
    List::cons(Adt86::C0(f497(), f354()), List::cons(Adt86::C0(f498(), f499()), List::cons(Adt86::C0(f500(), f501()), List::<Adt86>::nil())))
}

pub fn f424(v0: List<List<u64>>) -> List<u64> {
    f502(f38(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f425(p1662: u64, p1663: Adt144, p1664: List<Adt145>) -> R<Option<Adt6>> {
    let m1661 = (p1662, (p1663, p1664));
    {
        let mut m1661 = m1661;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1661;
                let m1648 = v0;
                if m1648 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1648 - 1;
                    let m1649 = v1;
                    match m1649 {
                        Adt144::C0(v4) => {
                            let m1650 = v4.clone();
                            match m1650 {
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
                                    let v8: Str = f503(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1651 = v9;
                                    match m1651.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1652 = v12;
                                    match m1652.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f503({
                                                let (h1653, _) = v13;
                                                h1653
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1654) = v13;
                                                h1654
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1655 = v2;
                            match m1655.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1656 = v17;
                                    match m1656 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1657 = v20;
                                            match m1657.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C4(f292(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v22), List::cons(Adt145::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt145::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1658 = v26;
                                            match m1658.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f294(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f503({
                                                        let (h1659, _) = v28;
                                                        h1659
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1660) = v28;
                                                        h1660
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
                Ok(next) => { m1661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426() -> u64 {
    67108864u64
}

pub fn f427(v0: Adt19) -> Adt6 {
    let m1665 = v0;
    match m1665 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f428(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1666 = v0;
    match m1666 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f429(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1667 = v0;
    match m1667 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f430() -> i64 {
    -32603i64
}

pub fn f431() -> i64 {
    429i64
}

pub fn f432() -> i64 {
    -32021i64
}

pub fn f433(p1670: List<Str>, p1671: List<Adt6>) -> List<Adt6> {
    let m1669 = (p1670, p1671);
    {
        let mut m1669 = m1669;
        loop {
            let step = {
                let (v0, v1) = m1669;
                let m1668 = v0;
                match m1668.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f434(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1672 = v1;
    match m1672 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1673) => {
                    h1673
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f435(v0: List<u64>) -> R<bool> {
    let m1674 = v0;
    match m1674.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1675 = v2;
            match m1675.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1676 = f181(v1)?;
                    let a1677 = 113u64;
                    if if nat_eq(a1676, a1677) {
                        let a1678 = v3;
                        let a1679 = 61u64;
                        nat_eq(a1678, a1679)
                    } else {
                        false
                    } {
                        Ok(f504(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f436(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt126>) -> R<Result<Adt122, Adt125>> {
    let m1680 = v3;
    match m1680 {
        Ok(v5) => {
            f505(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt122, Adt125>(Adt125::C0(v4)))
        }
    }
}

pub fn f437(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt126> {
    let m1681 = v0;
    match m1681 {
        Adt134::C0 => {
            f506(v1, v2)
        }
        Adt134::C1 => {
            f507(v1, v2)
        }
        Adt134::C2 => {
            f508(v1, v2)
        }
        Adt134::C3 => {
            f509(v1, v2)
        }
        Adt134::C4 => {
            f510(v1, v2)
        }
        Adt134::C5 => {
            f511(v1, v2)
        }
        Adt134::C6 => {
            f512(v1, v2)
        }
        Adt134::C7 => {
            f513(v1, v2)
        }
        Adt134::C8 => {
            f514(v1, v2)
        }
        Adt134::C9 => {
            f515(v1, v2)
        }
        Adt134::C10 => {
            f516(v1, v2)
        }
        Adt134::C11 => {
            f517(v1, v2)
        }
    }
}

pub fn f438(v0: Adt134) -> Str {
    let m1682 = v0;
    match m1682 {
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

pub fn f439(v0: Option<Adt8>) -> bool {
    let m1683 = v0;
    match m1683 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f440(v0: Option<Adt9>) -> bool {
    let m1684 = v0;
    match m1684 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f441(v0: Option<Adt5>) -> bool {
    let m1685 = v0;
    match m1685 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f442(v0: Option<Adt5>, v1: Str) -> bool {
    let m1686 = v0;
    match m1686 {
        None => {
            false
        }
        Some(v2) => {
            f224(v1, v2)
        }
    }
}

pub fn f443() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f444(v0: List<Str>, v1: Adt98, v2: Adt16, v3: Option<Adt107>) -> Adt137 {
    let m1687 = v3;
    match m1687 {
        None => {
            f518(v1, v2.clone(), f519(match v2 {
                Adt16::C0(h1688, _, _, _, _, _) => {
                    h1688
                }
            }, v0))
        }
        Some(v4) => {
            Adt137::C1(Adt125::C2(v4))
        }
    }
}

pub fn f445(v0: Option<Adt106>, v1: Adt139, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt107>> {
    let m1689 = v0;
    match m1689 {
        None => {
            Ok(None::<Adt107>)
        }
        Some(v5) => {
            let m1691 = match v5.clone() {
                Adt106::C0(_, _, _, h1690, _) => {
                    h1690
                }
            };
            match m1691 {
                None => {
                    f520(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f446(v0: Option<Adt5>) -> Adt5 {
    let m1692 = v0;
    match m1692 {
        None => {
            f521()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f447(v0: Adt5) -> Result<Adt16, Adt126> {
    let m1693 = f522(v0.clone());
    match m1693 {
        Adt148::C0 => {
            f523(v0.clone(), f173(f524(), v0.clone()), f173(f525(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt126>(Adt126::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt126>(Adt126::C3)
        }
    }
}

pub fn f448(p1698: Bytes, p1699: List<Bytes>) -> Bytes {
    let m1697 = (p1698, p1699);
    {
        let mut m1697 = m1697;
        loop {
            let step = {
                let (v0, v1) = m1697;
                let m1694 = v1;
                match m1694.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1695 = v2;
                            let a1696 = v0;
                            append_bytes(a1695, a1696)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(p1702: List<Bytes>, p1703: List<u64>) -> List<Bytes> {
    let m1701 = (p1702, p1703);
    {
        let mut m1701 = m1701;
        loop {
            let step = {
                let (v0, v1) = m1701;
                let m1700 = v1;
                match m1700.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f526(f527(v2), v0), v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f450(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1704 = f528(v0)?;
    match m1704 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1705 = f528(v1)?;
            match m1705 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1706 = v2;
                    let a1707 = 61u64;
                    if nat_eq(a1706, a1707) {
                        f529(v5, v6, v4, v3)
                    } else {
                        let m1708 = f528(v2)?;
                        match m1708 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1709 = v3;
                                let a1710 = 61u64;
                                if nat_eq(a1709, a1710) {
                                    f530(v5, v6, v7, v4)
                                } else {
                                    let m1711 = f528(v3)?;
                                    match m1711 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f531(v5, v6, v7, v8)?))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f451(v0: List<u64>) -> bool {
    let m1712 = v0;
    match m1712.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f452() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f453(v0: Str) -> R<List<u64>> {
    f85({
        let a1713 = v0;
        utf8_encode(a1713)
    })
}

pub fn f454(v0: u64, v1: u64) -> u64 {
    let a1714 = v0;
    let a1715 = v1;
    let a1716 = 0u64;
    nat_rem(a1714, a1715, a1716)
}

pub fn f455(v0: u64, v1: u64) -> u64 {
    let a1717 = v0;
    let a1718 = v1;
    let a1719 = 0u64;
    nat_quot(a1717, a1718, a1719)
}

pub fn f456(v0: u64, v1: Adt6) -> bool {
    let a1720 = v0;
    let a1721 = 0u64;
    if nat_eq(a1720, a1721) {
        f532(v1)
    } else {
        let a1722 = v0;
        let a1723 = 1u64;
        if nat_eq(a1722, a1723) {
            f533(v1)
        } else {
            let a1724 = v0;
            let a1725 = 2u64;
            if nat_eq(a1724, a1725) {
                f534(v1)
            } else {
                let a1726 = v0;
                let a1727 = 3u64;
                if nat_eq(a1726, a1727) {
                    f535(v1)
                } else {
                    let a1728 = v0;
                    let a1729 = 4u64;
                    if nat_eq(a1728, a1729) {
                        f536(v1)
                    } else {
                        let a1730 = v0;
                        let a1731 = 5u64;
                        if nat_eq(a1730, a1731) {
                            f537(v1)
                        } else {
                            let a1732 = v0;
                            let a1733 = 6u64;
                            if nat_eq(a1732, a1733) {
                                f538(v1)
                            } else {
                                let a1734 = v0;
                                let a1735 = 7u64;
                                if nat_eq(a1734, a1735) {
                                    f539(v1)
                                } else {
                                    f540(v1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f457(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1736 = v1.clone();
    let a1737 = Str::lit("#");
    if equal(a1736, a1737) {
        Some(v0)
    } else {
        let m1738 = f246(v0);
        match m1738 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1739 = f173(Str::lit("$defs"), v2);
                match m1739 {
                    Adt129::C0 => {
                        None::<Adt6>
                    }
                    Adt129::C1(v3) => {
                        let m1740 = f246(v3);
                        match m1740 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f541(v1, match v4 {
                                    Adt5::C0(h1741) => {
                                        h1741
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

pub fn f458(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt119) -> R<Adt141> {
    let m1742 = v3.clone();
    match m1742.uncons() {
        None => {
            Ok(Adt141::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f185(v3.clone(), true) {
                let m1743 = f542(v1);
                match m1743 {
                    None => {
                        Ok(Adt141::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f543(v3)?;
                        if f544(v9.clone(), match v5.clone() {
                            Adt119::C0(_, h1744) => {
                                h1744
                            }
                        }) {
                            Ok(Adt141::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt141::C1(Adt119::C0(List::cons(Adt12::C0(v0, v2, f545(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt119::C0(h1745, _) => {
                                    h1745
                                }
                            }), List::cons(v9, match v5 {
                                Adt119::C0(_, h1746) => {
                                    h1746
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

pub fn f459(p1751: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1752: List<Adt99>) -> List<Adt99> {
    let m1750 = (p1751, p1752);
    {
        let mut m1750 = m1750;
        loop {
            let step = {
                let (v0, v3) = m1750;
                let m1747 = v0;
                match m1747.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>((v5, List::cons(Adt99::C0({
                            let (_, h1748) = v4;
                            h1748
                        }, v1, List::cons({
                            let (h1749, _) = v4;
                            h1749
                        }, v2.clone())), v3)))
                    }
                }
            };
            match step {
                Ok(next) => { m1750 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(p1756: List<(Str, Adt6)>, p1757: List<Adt99>) -> List<Adt99> {
    let m1755 = (p1756, p1757);
    {
        let mut m1755 = m1755;
        loop {
            let step = {
                let (v0, v1) = m1755;
                let m1753 = v0;
                match m1753.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt99>), List<Adt99>>((v3, List::cons(Adt99::C0({
                            let (_, h1754) = v2;
                            h1754
                        }, false, List::<Str>::nil()), v1)))
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

pub fn f461(p1760: List<Adt6>, p1761: List<Adt99>) -> List<Adt99> {
    let m1759 = (p1760, p1761);
    {
        let mut m1759 = m1759;
        loop {
            let step = {
                let (v0, v1) = m1759;
                let m1758 = v0;
                match m1758.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt99>), List<Adt99>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt99>), List<Adt99>>((v3, List::cons(Adt99::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Str) -> bool {
    let a1762 = v0.clone();
    let a1763 = Str::lit("null");
    if equal(a1762, a1763) {
        true
    } else {
        let a1764 = v0.clone();
        let a1765 = Str::lit("boolean");
        if equal(a1764, a1765) {
            true
        } else {
            let a1766 = v0.clone();
            let a1767 = Str::lit("object");
            if equal(a1766, a1767) {
                true
            } else {
                let a1768 = v0.clone();
                let a1769 = Str::lit("array");
                if equal(a1768, a1769) {
                    true
                } else {
                    let a1770 = v0.clone();
                    let a1771 = Str::lit("number");
                    if equal(a1770, a1771) {
                        true
                    } else {
                        let a1772 = v0.clone();
                        let a1773 = Str::lit("string");
                        if equal(a1772, a1773) {
                            true
                        } else {
                            let a1774 = v0;
                            let a1775 = Str::lit("integer");
                            equal(a1774, a1775)
                        }
                    }
                }
            }
        }
    }
}

pub fn f463(v0: Str, v1: Adt6) -> R<bool> {
    let a1776 = v0.clone();
    let a1777 = f546(v1.clone());
    if equal(a1776, a1777) {
        Ok(true)
    } else {
        let a1778 = v0;
        let a1779 = Str::lit("integer");
        if equal(a1778, a1779) {
            let m1780 = f467(v1);
            match m1780 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f547(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f464(p1783: List<Adt6>, v1: Adt6, p1784: bool) -> R<Adt142> {
    let m1782 = (p1783, p1784);
    {
        let mut m1782 = m1782;
        loop {
            let step = {
                let (v0, v2) = m1782;
                let m1781 = v0;
                match m1781.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f462(f548(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt142>>((v4, if v2 {
                                true
                            } else {
                                f463(f548(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt142>>(Ok(Adt142::C2))
                        }
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

pub fn f465(p1788: List<Adt6>, v1: Adt6) -> R<Adt142> {
    let m1787 = p1788;
    {
        let mut m1787 = m1787;
        loop {
            let step = {
                let v0 = m1787;
                let m1785 = v0;
                match m1785.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt142>>(Ok(Adt142::C1))
                    }
                    Some((v2, v3)) => {
                        let m1786 = f466(v2, v1.clone())?;
                        match m1786 {
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
                Ok(next) => { m1787 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f549(f550(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f467(v0: Adt6) -> Option<Adt7> {
    let m1789 = v0;
    match m1789 {
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

pub fn f468(v0: u64, v1: Adt147) -> bool {
    let a1790 = v0;
    let a1791 = 0u64;
    if nat_eq(a1790, a1791) {
        let a1792 = f551(v1);
        bool_not(a1792)
    } else {
        let a1793 = v0;
        let a1794 = 1u64;
        if nat_eq(a1793, a1794) {
            let a1795 = f481(v1);
            bool_not(a1795)
        } else {
            let a1796 = v0;
            let a1797 = 2u64;
            if nat_eq(a1796, a1797) {
                f481(v1)
            } else {
                f551(v1)
            }
        }
    }
}

pub fn f469(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f552(v0.clone(), v1.clone(), f553(match v0 {
        Adt7::C0(h1798, _) => {
            h1798
        }
    }), f553(match v1 {
        Adt7::C0(h1799, _) => {
            h1799
        }
    }))
}

pub fn f470(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt142> {
    let m1800 = f223(v0, v2);
    match m1800 {
        Adt129::C0 => {
            Ok(Adt142::C0)
        }
        Adt129::C1(v4) => {
            let m1801 = f554(v4);
            match m1801 {
                None => {
                    Ok(Adt142::C2)
                }
                Some(v5) => {
                    let m1804 = {
                        let a1802 = v5;
                        let a1803 = 0i64;
                        compare(a1802, a1803)
                    };
                    if match m1804 {
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
                            if f551(f555(f80(v3, 0i64)?, v5)) {
                                Ok(Adt142::C1)
                            } else {
                                Ok(Adt142::C0)
                            }
                        } else {
                            if f481(f555(f80(v3, 0i64)?, v5)) {
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

pub fn f471(v0: Str) -> R<u64> {
    f556(f85({
        let a1805 = v0;
        utf8_encode(a1805)
    })?, 0u64)
}

pub fn f472(p1810: List<Adt6>, p1811: u64) -> R<u64> {
    let m1809 = (p1810, p1811);
    {
        let mut m1809 = m1809;
        loop {
            let step = {
                let (v0, v1) = m1809;
                let m1806 = v0;
                match m1806.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1807 = v1;
                            let a1808 = 1u64;
                            nat_add(a1807, a1808)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f473(p1816: List<(Str, Adt6)>, p1817: u64) -> R<u64> {
    let m1815 = (p1816, p1817);
    {
        let mut m1815 = m1815;
        loop {
            let step = {
                let (v0, v1) = m1815;
                let m1812 = v0;
                match m1812.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1813 = v1;
                            let a1814 = 1u64;
                            nat_add(a1813, a1814)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(p1821: List<Adt6>, v1: Adt5) -> Adt142 {
    let m1820 = p1821;
    {
        let mut m1820 = m1820;
        loop {
            let step = {
                let v0 = m1820;
                let m1818 = v0;
                match m1818.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt142>(Adt142::C0)
                    }
                    Some((v2, v3)) => {
                        let m1819 = f220(v2);
                        match m1819 {
                            None => {
                                Err::<List<Adt6>, Adt142>(Adt142::C2)
                            }
                            Some(v4) => {
                                if f224(v4, v1.clone()) {
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
                Ok(next) => { m1820 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475(p1828: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt101>, p1829: List<Adt103>) -> List<Adt103> {
    let m1827 = (p1828, p1829);
    {
        let mut m1827 = m1827;
        loop {
            let step = {
                let (v0, v3) = m1827;
                let m1822 = v0;
                match m1822.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1824 = f223({
                            let (h1823, _) = v4.clone();
                            h1823
                        }, v1.clone());
                        match m1824 {
                            Adt129::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v5, v3))
                            }
                            Adt129::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v5, List::cons(Adt103::C0({
                                    let (_, h1825) = v4;
                                    h1825
                                }, v6, List::cons(Adt101::C0({
                                    let (h1826, _) = v4;
                                    h1826
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f476(p1834: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1833 = p1834;
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let v0 = m1833;
                let m1830 = v0;
                match m1830.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1832 = f223({
                            let (h1831, _) = v2;
                            h1831
                        }, v1.clone());
                        match m1832 {
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
                Ok(next) => { m1833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1835 = f223(Str::lit("properties"), v0);
    match m1835 {
        Adt129::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt129::C1(v1) => {
            let m1836 = f246(v1);
            match m1836 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1837) => {
                            h1837
                        }
                    }
                }
            }
        }
    }
}

pub fn f478(p1844: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt101>, p1845: List<Adt103>) -> List<Adt103> {
    let m1843 = (p1844, p1845);
    {
        let mut m1843 = m1843;
        loop {
            let step = {
                let (v0, v4) = m1843;
                let m1838 = v0;
                match m1838.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1840 = f223({
                            let (h1839, _) = v5.clone();
                            h1839
                        }, v1.clone());
                        match m1840 {
                            Adt129::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt103>), List<Adt103>>((v6, List::cons(Adt103::C0(v2.clone(), {
                                    let (_, h1841) = v5;
                                    h1841
                                }, List::cons(Adt101::C0({
                                    let (h1842, _) = v5;
                                    h1842
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
                Ok(next) => { m1843 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(p1850: List<Adt6>, p1851: u64, v2: Adt6, v3: List<Adt101>, p1852: List<Adt103>) -> R<List<Adt103>> {
    let m1849 = (p1850, (p1851, p1852));
    {
        let mut m1849 = m1849;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1849;
                let m1846 = v0;
                match m1846.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt103>)), R<List<Adt103>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt103>)), R<List<Adt103>>>((v6, ({
                            let a1847 = v1;
                            let a1848 = 1u64;
                            nat_add(a1847, a1848)?
                        }, List::cons(Adt103::C0(v2.clone(), v5, List::cons(Adt101::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(p1855: List<Adt6>, v1: Adt6, v2: List<Adt101>, p1856: List<Adt103>) -> List<Adt103> {
    let m1854 = (p1855, p1856);
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let (v0, v3) = m1854;
                let m1853 = v0;
                match m1853.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt103>), List<Adt103>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt103>), List<Adt103>>((v5, List::cons(Adt103::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f481(v0: Adt147) -> bool {
    let m1857 = v0;
    match m1857 {
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

pub fn f482(p1869: u64, p1870: i64, p1871: List<u64>) -> R<List<u64>> {
    let m1868 = (p1869, (p1870, p1871));
    {
        let mut m1868 = m1868;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1868;
                let m1858 = v0;
                if m1858 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1858 - 1;
                    let v4: i64 = {
                        let a1859 = v1;
                        let a1860 = 10i64;
                        let a1861 = 0i64;
                        int_quot(a1859, a1860, a1861)?
                    };
                    let v5: i64 = {
                        let a1862 = v1;
                        let a1863 = 10i64;
                        let a1864 = 0i64;
                        int_rem(a1862, a1863, a1864)
                    };
                    let m1867 = {
                        let a1865 = v4;
                        let a1866 = 1i64;
                        compare(a1865, a1866)
                    };
                    if match m1867 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f557(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f557(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1868 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f483(v0: u64) -> R<u64> {
    let a1872 = v0;
    let a1873 = 10u64;
    if nat_lt(a1872, a1873) {
        let a1874 = 48u64;
        let a1875 = v0;
        nat_add(a1874, a1875)
    } else {
        let a1876 = 87u64;
        let a1877 = v0;
        nat_add(a1876, a1877)
    }
}

pub fn f484(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1878 = v1;
    match m1878 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f558(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f485(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1879 = v0;
    match m1879 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1882 = f408(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1880, _) => {
                        h1880
                    }
                });
                let a1883 = f408(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1881) => {
                        h1881
                    }
                });
                append_list(a1882, a1883)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f486(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f559(v0, 0i64)?, 0i64)))
}

pub fn f487(v0: Adt11) -> Str {
    let m1884 = v0;
    match m1884 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f488(p1887: List<Adt35>, p1888: List<Adt6>) -> List<Adt6> {
    let m1886 = (p1887, p1888);
    {
        let mut m1886 = m1886;
        loop {
            let step = {
                let (v0, v1) = m1886;
                let m1885 = v0;
                match m1885.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f560(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1886 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(p1892: List<Adt38>, p1893: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1891 = (p1892, p1893);
    {
        let mut m1891 = m1891;
        loop {
            let step = {
                let (v0, v1) = m1891;
                let m1889 = v0;
                match m1889.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1890 = f561(v2)?;
                        match m1890 {
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
                Ok(next) => { m1891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(p1896: List<Adt44>, p1897: List<Adt6>) -> R<List<Adt6>> {
    let m1895 = (p1896, p1897);
    {
        let mut m1895 = m1895;
        loop {
            let step = {
                let (v0, v1) = m1895;
                let m1894 = v0;
                match m1894.uncons() {
                    None => {
                        Err::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt44>, List<Adt6>), R<List<Adt6>>>((v3, List::cons(f562(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1895 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f491(p1900: List<Adt46>, p1901: List<Adt6>) -> List<Adt6> {
    let m1899 = (p1900, p1901);
    {
        let mut m1899 = m1899;
        loop {
            let step = {
                let (v0, v1) = m1899;
                let m1898 = v0;
                match m1898.uncons() {
                    None => {
                        Err::<(List<Adt46>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt46>, List<Adt6>), List<Adt6>>((v3, List::cons(f563(v2), v1)))
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

pub fn f492(p1904: List<Adt54>, p1905: List<Adt6>) -> List<Adt6> {
    let m1903 = (p1904, p1905);
    {
        let mut m1903 = m1903;
        loop {
            let step = {
                let (v0, v1) = m1903;
                let m1902 = v0;
                match m1902.uncons() {
                    None => {
                        Err::<(List<Adt54>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt54>, List<Adt6>), List<Adt6>>((v3, List::cons(f564(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1903 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(p1908: List<Adt56>, p1909: List<Adt6>) -> List<Adt6> {
    let m1907 = (p1908, p1909);
    {
        let mut m1907 = m1907;
        loop {
            let step = {
                let (v0, v1) = m1907;
                let m1906 = v0;
                match m1906.uncons() {
                    None => {
                        Err::<(List<Adt56>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt56>, List<Adt6>), List<Adt6>>((v3, List::cons(f565(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1907 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494(p1913: List<Adt59>, p1914: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1912 = (p1913, p1914);
    {
        let mut m1912 = m1912;
        loop {
            let step = {
                let (v0, v1) = m1912;
                let m1910 = v0;
                match m1910.uncons() {
                    None => {
                        Err::<(List<Adt59>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1911 = f566(v2)?;
                        match m1911 {
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
                Ok(next) => { m1912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f495(v0: Adt64) -> R<Option<Adt6>> {
    let m1915 = v0;
    match m1915 {
        Adt64::C0(v1) => {
            Ok(Some(f567(Str::lit("elicitation/create"), Some(f568(v1)))))
        }
        Adt64::C1(v2) => {
            let m1916 = f569(v2)?;
            match m1916 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(f567(Str::lit("sampling/createMessage"), Some(v3))))
                }
            }
        }
        Adt64::C2(v4) => {
            Ok(Some(f567(Str::lit("roots/list"), {
                let m1917 = v4;
                match m1917 {
                    None => {
                        None::<Adt6>
                    }
                    Some(v5) => {
                        Some(Adt6::C5(match v5 {
                            Adt5::C0(h1918) => {
                                h1918
                            }
                        }))
                    }
                }
            })))
        }
    }
}

pub fn f496(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1919 = v0;
    match m1919 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f570(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f497() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f498() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f499() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f500() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f501() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f502(p1924: List<List<u64>>, p1925: List<u64>) -> List<u64> {
    let m1923 = (p1924, p1925);
    {
        let mut m1923 = m1923;
        loop {
            let step = {
                let (v0, v1) = m1923;
                let m1920 = v0;
                match m1920.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a1921 = f571(v2);
                            let a1922 = v1;
                            append_list(a1921, a1922)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1923 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f503(v0: Str) -> R<Str> {
    let m1928 = {
        let a1927 = f360(f361(), f160(f572(f85({
            let a1926 = v0;
            utf8_encode(a1926)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1927)
    };
    match m1928 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f504(v0: List<u64>) -> bool {
    let m1929 = v0;
    match m1929.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a1930 = v1;
            let a1931 = 48u64;
            if nat_eq(a1930, a1931) {
                f573(v2)
            } else {
                false
            }
        }
    }
}

pub fn f505(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt122, Adt125>> {
    if f574(f575(v3.clone())) {
        Ok(Ok::<Adt122, Adt125>(f576(v0.clone(), Adt32::C0(f577(v0)))))
    } else {
        Ok(f578(v0.clone(), v3.clone(), f579(match v0.clone() {
            Adt0::C0(_, _, _, _, _, _, h1932) => {
                h1932
            }
        }, match v0 {
            Adt0::C0(_, _, _, _, _, h1933, _) => {
                h1933
            }
        }, v2, v1, v3)?))
    }
}

pub fn f506(v0: Adt16, _: Adt5) -> Result<Adt14, Adt126> {
    Ok::<Adt14, Adt126>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f507(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1934 = f580(Str::lit("cursor"), v1);
    match m1934 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f508(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1935 = f581(Str::lit("name"), v1.clone());
    match m1935 {
        Ok(v2) => {
            let m1936 = f582(Str::lit("arguments"), v1.clone());
            match m1936 {
                Ok(v3) => {
                    let m1937 = f582(Str::lit("inputResponses"), v1.clone());
                    match m1937 {
                        Ok(v4) => {
                            let m1938 = f580(Str::lit("requestState"), v1);
                            match m1938 {
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

pub fn f509(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1939 = f580(Str::lit("cursor"), v1);
    match m1939 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f510(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1940 = f581(Str::lit("uri"), v1.clone());
    match m1940 {
        Ok(v2) => {
            let m1941 = f582(Str::lit("inputResponses"), v1.clone());
            match m1941 {
                Ok(v3) => {
                    let m1942 = f580(Str::lit("requestState"), v1);
                    match m1942 {
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

pub fn f511(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1943 = f580(Str::lit("cursor"), v1);
    match m1943 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f512(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1944 = f580(Str::lit("cursor"), v1);
    match m1944 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f513(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1945 = f581(Str::lit("name"), v1.clone());
    match m1945 {
        Ok(v2) => {
            let m1946 = f583(Str::lit("arguments"), v1.clone());
            match m1946 {
                Ok(v3) => {
                    let m1947 = f582(Str::lit("inputResponses"), v1.clone());
                    match m1947 {
                        Ok(v4) => {
                            let m1948 = f580(Str::lit("requestState"), v1);
                            match m1948 {
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

pub fn f514(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1949 = f584(Str::lit("ref"), v1.clone());
    match m1949 {
        Ok(v2) => {
            let m1950 = f585(v2);
            match m1950 {
                Ok(v3) => {
                    let m1951 = f584(Str::lit("argument"), v1.clone());
                    match m1951 {
                        Ok(v4) => {
                            let m1952 = f586(v4);
                            match m1952 {
                                Ok(v5) => {
                                    let m1953 = f587(v1);
                                    match m1953 {
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

pub fn f515(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1954 = f581(Str::lit("taskId"), v1);
    match m1954 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f516(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1955 = f581(Str::lit("taskId"), v1.clone());
    match m1955 {
        Ok(v2) => {
            let m1956 = f584(Str::lit("inputResponses"), v1);
            match m1956 {
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

pub fn f517(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt126> {
    let m1957 = f581(Str::lit("taskId"), v1);
    match m1957 {
        Ok(v2) => {
            Ok::<Adt14, Adt126>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt126>(v3)
        }
    }
}

pub fn f518(v0: Adt98, v1: Adt16, v2: Option<Adt125>) -> Adt137 {
    let m1958 = v2;
    match m1958 {
        None => {
            Adt137::C0(Adt138::C0(match v0.clone() {
                Adt98::C0(h1959, _, _) => {
                    h1959
                }
            }, match v0.clone() {
                Adt98::C0(_, h1960, _) => {
                    h1960
                }
            }, v1, f446(match v0 {
                Adt98::C0(_, _, h1961) => {
                    h1961
                }
            })))
        }
        Some(v3) => {
            Adt137::C1(v3)
        }
    }
}

pub fn f519(v0: Str, v1: List<Str>) -> Option<Adt125> {
    f588(f589(v0.clone(), v1.clone()), v0, v1)
}

pub fn f520(v0: Adt106, v1: Adt139, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt107>> {
    let m1963 = match v0.clone() {
        Adt106::C0(h1962, _, _, _, _) => {
            h1962
        }
    };
    match m1963 {
        None => {
            Ok(Some(Adt107::C0))
        }
        Some(v5) => {
            let a1964 = v5;
            let a1965 = v3;
            if equal(a1964, a1965) {
                let m1967 = match v0.clone() {
                    Adt106::C0(_, h1966, _, _, _) => {
                        h1966
                    }
                };
                match m1967 {
                    None => {
                        Ok(Some(Adt107::C1))
                    }
                    Some(v6) => {
                        let a1968 = v6;
                        let a1969 = v2;
                        if equal(a1968, a1969) {
                            f590(f591(match v0.clone() {
                                Adt106::C0(_, _, h1970, _, _) => {
                                    h1970
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt106::C0(_, _, _, _, h1971) => {
                                    h1971
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

pub fn f521() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f522(v0: Adt5) -> Adt148 {
    f592(f173(f524(), v0.clone()), f173(f525(), v0))
}

pub fn f523(v0: Adt5, v1: Adt129, v2: Adt129) -> Result<Adt16, Adt126> {
    let m1972 = v1;
    match m1972 {
        Adt129::C0 => {
            Err::<Adt16, Adt126>(Adt126::C2)
        }
        Adt129::C1(v3) => {
            let m1973 = v2;
            match m1973 {
                Adt129::C0 => {
                    Err::<Adt16, Adt126>(Adt126::C3)
                }
                Adt129::C1(v4) => {
                    f593(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f524() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f525() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f526(p1982: Bytes, p1983: List<Bytes>) -> List<Bytes> {
    let m1981 = (p1982, p1983);
    {
        let mut m1981 = m1981;
        loop {
            let step = {
                let (v0, v1) = m1981;
                let m1974 = v1;
                match m1974.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1977 = {
                            let a1975 = v2.clone();
                            length_bytes(a1975)
                        };
                        let a1978 = {
                            let a1976 = v0.clone();
                            length_bytes(a1976)
                        };
                        if nat_eq(a1977, a1978) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1979 = v2;
                                let a1980 = v0;
                                append_bytes(a1979, a1980)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1981 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f527(v0: u64) -> Bytes {
    let m1987 = {
        let a1984 = f594();
        let a1985 = v0;
        let a1986 = 1u64;
        slice_bytes(a1984, a1985, a1986)
    };
    match m1987 {
        None => {
            f361()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f528(v0: u64) -> R<Option<u64>> {
    let a1988 = 64u64;
    let a1989 = v0;
    if if nat_lt(a1988, a1989) {
        let a1990 = v0;
        let a1991 = 91u64;
        nat_lt(a1990, a1991)
    } else {
        false
    } {
        Ok(Some({
            let a1992 = v0;
            let a1993 = 65u64;
            nat_sub(a1992, a1993)
        }))
    } else {
        let a1994 = 96u64;
        let a1995 = v0;
        if if nat_lt(a1994, a1995) {
            let a1996 = v0;
            let a1997 = 123u64;
            nat_lt(a1996, a1997)
        } else {
            false
        } {
            Ok(Some({
                let a1998 = v0;
                let a1999 = 71u64;
                nat_sub(a1998, a1999)
            }))
        } else {
            let a2000 = 47u64;
            let a2001 = v0;
            if if nat_lt(a2000, a2001) {
                let a2002 = v0;
                let a2003 = 58u64;
                nat_lt(a2002, a2003)
            } else {
                false
            } {
                Ok(Some({
                    let a2004 = v0;
                    let a2005 = 4u64;
                    nat_add(a2004, a2005)?
                }))
            } else {
                let a2006 = v0;
                let a2007 = 43u64;
                if nat_eq(a2006, a2007) {
                    Ok(Some(62u64))
                } else {
                    let a2008 = v0;
                    let a2009 = 47u64;
                    if nat_eq(a2008, a2009) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f529(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a2010 = v3;
        let a2011 = 61u64;
        if nat_eq(a2010, a2011) {
            let a2012 = f595(v1, 16u64);
            let a2013 = 0u64;
            nat_eq(a2012, a2013)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f596(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f530(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a2014 = f595(v2, 4u64);
        let a2015 = 0u64;
        nat_eq(a2014, a2015)
    } else {
        false
    } {
        Ok(Some(f597(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f531(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2018 = {
            let a2016 = v0;
            let a2017 = 4u64;
            nat_mul(a2016, a2017)?
        };
        let a2019 = f598(v1, 16u64);
        nat_add(a2018, a2019)?
    }, List::cons({
        let a2022 = {
            let a2020 = f595(v1, 16u64);
            let a2021 = 16u64;
            nat_mul(a2020, a2021)?
        };
        let a2023 = f598(v2, 4u64);
        nat_add(a2022, a2023)?
    }, List::cons({
        let a2026 = {
            let a2024 = f595(v2, 4u64);
            let a2025 = 64u64;
            nat_mul(a2024, a2025)?
        };
        let a2027 = v3;
        nat_add(a2026, a2027)?
    }, List::<u64>::nil()))))
}

pub fn f532(v0: Adt6) -> bool {
    let m2028 = f220(v0);
    match m2028 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f533(v0: Adt6) -> bool {
    let m2029 = f467(v0);
    match m2029 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f534(v0: Adt6) -> bool {
    let m2030 = f554(v0);
    match m2030 {
        None => {
            false
        }
        Some(v1) => {
            let a2034 = {
                let m2033 = {
                    let a2031 = v1;
                    let a2032 = 0i64;
                    compare(a2031, a2032)
                };
                match m2033 {
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
            bool_not(a2034)
        }
    }
}

pub fn f535(v0: Adt6) -> bool {
    let m2035 = f315(v0);
    match m2035 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f536(v0: Adt6) -> bool {
    let m2036 = f246(v0);
    match m2036 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f537(v0: Adt6) -> bool {
    let m2037 = f315(v0);
    match m2037 {
        None => {
            false
        }
        Some(v1) => {
            f599(v1)
        }
    }
}

pub fn f538(v0: Adt6) -> bool {
    let m2038 = f245(v0);
    match m2038 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f539(v0: Adt6) -> bool {
    let m2039 = f220(v0.clone());
    match m2039 {
        None => {
            let m2040 = f315(v0);
            match m2040 {
                None => {
                    false
                }
                Some(v2) => {
                    f600(v2)
                }
            }
        }
        Some(v1) => {
            f462(v1)
        }
    }
}

pub fn f540(v0: Adt6) -> bool {
    let m2041 = f315(v0);
    match m2041 {
        None => {
            false
        }
        Some(v1) => {
            let m2042 = v1;
            match m2042.uncons() {
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

pub fn f541(v0: Str, p2060: List<(Str, Adt6)>) -> Option<Adt6> {
    let m2059 = p2060;
    {
        let mut m2059 = m2059;
        loop {
            let step = {
                let v1 = m2059;
                let m2043 = v1;
                match m2043.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a2047 = v0.clone();
                        let a2048 = {
                            let a2045 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h2044, _) = v2.clone();
                                h2044
                            }, List::<Str>::nil()));
                            let a2046 = Str::lit("");
                            join(a2045, a2046)
                        };
                        if equal(a2047, a2048) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h2049) = v2;
                                h2049
                            }))
                        } else {
                            let m2051 = f246({
                                let (_, h2050) = v2.clone();
                                h2050
                            });
                            match m2051 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m2052 = f173(Str::lit("$anchor"), v4);
                                    match m2052 {
                                        Adt129::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt129::C1(v5) => {
                                            let m2053 = f220(v5);
                                            match m2053 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a2056 = v0.clone();
                                                    let a2057 = {
                                                        let a2054 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a2055 = Str::lit("");
                                                        join(a2054, a2055)
                                                    };
                                                    if equal(a2056, a2057) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h2058) = v2;
                                                            h2058
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
                Ok(next) => { m2059 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: List<(Str, Adt6)>) -> Option<Adt13> {
    let m2061 = f223(Str::lit("type"), v0);
    match m2061 {
        Adt129::C0 => {
            None::<Adt13>
        }
        Adt129::C1(v1) => {
            let m2062 = f220(v1);
            match m2062 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f601(v2)
                }
            }
        }
    }
}

pub fn f543(v0: List<u64>) -> R<List<u64>> {
    Ok(f160(f602(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f544(v0: List<u64>, p2065: List<List<u64>>) -> bool {
    let m2064 = p2065;
    {
        let mut m2064 = m2064;
        loop {
            let step = {
                let v1 = m2064;
                let m2063 = v1;
                match m2063.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f603(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
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

pub fn f545(p2068: List<Str>, p2069: List<Str>) -> List<Str> {
    let m2067 = (p2068, p2069);
    {
        let mut m2067 = m2067;
        loop {
            let step = {
                let (v0, v1) = m2067;
                let m2066 = v0;
                match m2066.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
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

pub fn f546(v0: Adt6) -> Str {
    let m2070 = v0;
    match m2070 {
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

pub fn f547(v0: Adt7) -> R<bool> {
    if f369(match v0.clone() {
        Adt7::C0(h2071, _) => {
            h2071
        }
    }) {
        Ok(true)
    } else {
        f604(40u64, match v0.clone() {
            Adt7::C0(h2072, _) => {
                h2072
            }
        }, match v0 {
            Adt7::C0(_, h2073) => {
                h2073
            }
        })
    }
}

pub fn f548(v0: Adt6) -> Str {
    let m2074 = f220(v0);
    match m2074 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f549(p2081: u64, p2082: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m2080 = (p2081, p2082);
    {
        let mut m2080 = m2080;
        loop {
            let step = {
                let (v0, v1) = m2080;
                let m2075 = v0;
                if m2075 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m2075 - 1;
                    let m2076 = v1;
                    match m2076.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m2079 = f605({
                                let (h2077, _) = v3;
                                h2077
                            }, {
                                let (_, h2078) = v3;
                                h2078
                            }, v4)?;
                            match m2079 {
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
                Ok(next) => { m2080 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f550() -> u64 {
    67108864u64
}

pub fn f551(v0: Adt147) -> bool {
    let m2083 = v0;
    match m2083 {
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

pub fn f552(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m2084 = v2;
    match m2084 {
        Adt147::C0 => {
            let m2086 = v3;
            match m2086 {
                Adt147::C0 => {
                    Ok(f606(Adt147::C0, f607(f608(match v0.clone() {
                        Adt7::C0(h2087, _) => {
                            h2087
                        }
                    })?, match v0 {
                        Adt7::C0(_, h2088) => {
                            h2088
                        }
                    }, f608(match v1.clone() {
                        Adt7::C0(h2089, _) => {
                            h2089
                        }
                    })?, match v1 {
                        Adt7::C0(_, h2090) => {
                            h2090
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
            let m2085 = v3;
            match m2085 {
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
            let m2091 = v3;
            match m2091 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f607(match v0.clone() {
                        Adt7::C0(h2092, _) => {
                            h2092
                        }
                    }, match v0 {
                        Adt7::C0(_, h2093) => {
                            h2093
                        }
                    }, match v1.clone() {
                        Adt7::C0(h2094, _) => {
                            h2094
                        }
                    }, match v1 {
                        Adt7::C0(_, h2095) => {
                            h2095
                        }
                    })
                }
            }
        }
    }
}

pub fn f553(v0: i64) -> Adt147 {
    f555(v0, 0i64)
}

pub fn f554(v0: Adt6) -> Option<i64> {
    let m2096 = v0;
    match m2096 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f369(match v2.clone() {
                Adt7::C0(_, h2097) => {
                    h2097
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h2098, _) => {
                        h2098
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

pub fn f555(v0: i64, v1: i64) -> Adt147 {
    let m2101 = {
        let a2099 = v0;
        let a2100 = v1;
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
        Adt147::C0
    } else {
        let m2104 = {
            let a2102 = v1;
            let a2103 = v0;
            compare(a2102, a2103)
        };
        if match m2104 {
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

pub fn f556(p2113: List<u64>, p2114: u64) -> R<u64> {
    let m2112 = (p2113, p2114);
    {
        let mut m2112 = m2112;
        loop {
            let step = {
                let (v0, v1) = m2112;
                let m2105 = v0;
                match m2105.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2106 = v2;
                        let a2107 = 128u64;
                        if if nat_lt(a2106, a2107) {
                            true
                        } else {
                            let a2108 = 191u64;
                            let a2109 = v2;
                            nat_lt(a2108, a2109)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a2110 = v1;
                                let a2111 = 1u64;
                                nat_add(a2110, a2111)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f557(v0: i64) -> u64 {
    let m2117 = {
        let a2115 = v0;
        let a2116 = 1i64;
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
        48u64
    } else {
        let m2120 = {
            let a2118 = v0;
            let a2119 = 2i64;
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
            49u64
        } else {
            let m2123 = {
                let a2121 = v0;
                let a2122 = 3i64;
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
                50u64
            } else {
                let m2126 = {
                    let a2124 = v0;
                    let a2125 = 4i64;
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
                    51u64
                } else {
                    let m2129 = {
                        let a2127 = v0;
                        let a2128 = 5i64;
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
                        52u64
                    } else {
                        let m2132 = {
                            let a2130 = v0;
                            let a2131 = 6i64;
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
                            53u64
                        } else {
                            let m2135 = {
                                let a2133 = v0;
                                let a2134 = 7i64;
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
                                54u64
                            } else {
                                let m2138 = {
                                    let a2136 = v0;
                                    let a2137 = 8i64;
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
                                    55u64
                                } else {
                                    let m2141 = {
                                        let a2139 = v0;
                                        let a2140 = 9i64;
                                        compare(a2139, a2140)
                                    };
                                    if match m2141 {
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

pub fn f558(v0: Adt8) -> Adt6 {
    Adt6::C5(f408(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2142) => {
            h2142
        }
    }))
}

pub fn f559(p2147: u64, p2148: i64) -> R<i64> {
    let m2146 = (p2147, p2148);
    {
        let mut m2146 = m2146;
        loop {
            let step = {
                let (v0, v1) = m2146;
                let m2143 = v0;
                if m2143 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2143 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2144 = v1;
                        let a2145 = -1i64;
                        int_sub(a2144, a2145)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2146 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f560(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2149, _, _, _, _, _, _, _) => {
            h2149
        }
    })), {
        let a2167 = f403(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2150, _, _, _, _, _, _) => {
                h2150
            }
        });
        let a2168 = {
            let a2165 = f496(match v0.clone() {
                Adt35::C0(_, _, h2151, _, _, _, _, _) => {
                    h2151
                }
            });
            let a2166 = {
                let a2163 = f403(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2152, _, _, _, _) => {
                        h2152
                    }
                });
                let a2164 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2155 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2154, _, _, _) => {
                            h2154
                        }
                    };
                    match m2155 {
                        Adt5::C0(h2153) => {
                            h2153
                        }
                    }
                })), {
                    let a2161 = f434(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2156, _, _) => {
                            h2156
                        }
                    });
                    let a2162 = {
                        let a2159 = f609(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2157, _) => {
                                h2157
                            }
                        });
                        let a2160 = f610(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2158) => {
                                h2158
                            }
                        });
                        append_list(a2159, a2160)
                    };
                    append_list(a2161, a2162)
                });
                append_list(a2163, a2164)
            };
            append_list(a2165, a2166)
        };
        append_list(a2167, a2168)
    }))
}

pub fn f561(v0: Adt38) -> R<Option<Adt6>> {
    let m2169 = v0;
    match m2169 {
        Adt38::C0(v1) => {
            Ok(Some(f611(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f612(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2170, _, _, _) => {
                    h2170
                }
            }, match v2.clone() {
                Adt42::C0(_, h2171, _, _) => {
                    h2171
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2172, _) => {
                    h2172
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2173) => {
                    h2173
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f612(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2174, _, _, _) => {
                    h2174
                }
            }, match v3.clone() {
                Adt43::C0(_, h2175, _, _) => {
                    h2175
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2176, _) => {
                    h2176
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2177) => {
                    h2177
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f613(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f614(v5)))
        }
    }
}

pub fn f562(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2201 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2178, _, _, _, _, _, _, _, _) => {
                h2178
            }
        })), List::<(Str, Adt6)>::nil());
        let a2202 = {
            let a2199 = f403(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2179, _, _, _, _, _, _, _) => {
                    h2179
                }
            });
            let a2200 = {
                let a2197 = f496(match v0.clone() {
                    Adt44::C0(_, _, h2180, _, _, _, _, _, _) => {
                        h2180
                    }
                });
                let a2198 = {
                    let a2195 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2181, _, _, _, _, _) => {
                            h2181
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2196 = {
                        let a2193 = f403(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2182, _, _, _, _) => {
                                h2182
                            }
                        });
                        let a2194 = {
                            let a2191 = f403(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2183, _, _, _) => {
                                    h2183
                                }
                            });
                            let a2192 = {
                                let a2189 = f615(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2184, _, _) => {
                                        h2184
                                    }
                                });
                                let a2190 = {
                                    let a2187 = f616(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2185, _) => {
                                            h2185
                                        }
                                    })?;
                                    let a2188 = f610(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2186) => {
                                            h2186
                                        }
                                    });
                                    append_list(a2187, a2188)
                                };
                                append_list(a2189, a2190)
                            };
                            append_list(a2191, a2192)
                        };
                        append_list(a2193, a2194)
                    };
                    append_list(a2195, a2196)
                };
                append_list(a2197, a2198)
            };
            append_list(a2199, a2200)
        };
        append_list(a2201, a2202)
    }))
}

pub fn f563(v0: Adt46) -> Adt6 {
    let m2203 = v0;
    match m2203 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2204, _, _, _) => {
                    h2204
                }
            })), {
                let a2208 = f403(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2205, _, _) => {
                        h2205
                    }
                });
                let a2209 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2206, _) => {
                        h2206
                    }
                })), f610(match v1 {
                    Adt47::C0(_, _, _, h2207) => {
                        h2207
                    }
                }));
                append_list(a2208, a2209)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2210, _, _, _) => {
                    h2210
                }
            })), {
                let a2214 = f403(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2211, _, _) => {
                        h2211
                    }
                });
                let a2215 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2212, _) => {
                        h2212
                    }
                })), f610(match v2 {
                    Adt48::C0(_, _, _, h2213) => {
                        h2213
                    }
                }));
                append_list(a2214, a2215)
            }))
        }
    }
}

pub fn f564(v0: Adt54) -> Adt6 {
    Adt6::C5({
        let a2236 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt54::C0(h2216, _, _, _, _, _, _, _) => {
                h2216
            }
        })), List::<(Str, Adt6)>::nil());
        let a2237 = {
            let a2234 = f403(Str::lit("title"), match v0.clone() {
                Adt54::C0(_, h2217, _, _, _, _, _, _) => {
                    h2217
                }
            });
            let a2235 = {
                let a2232 = f496(match v0.clone() {
                    Adt54::C0(_, _, h2218, _, _, _, _, _) => {
                        h2218
                    }
                });
                let a2233 = {
                    let a2230 = List::cons((Str::lit("uriTemplate"), Adt6::C3(match v0.clone() {
                        Adt54::C0(_, _, _, h2219, _, _, _, _) => {
                            h2219
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2231 = {
                        let a2228 = f403(Str::lit("description"), match v0.clone() {
                            Adt54::C0(_, _, _, _, h2220, _, _, _) => {
                                h2220
                            }
                        });
                        let a2229 = {
                            let a2226 = f403(Str::lit("mimeType"), match v0.clone() {
                                Adt54::C0(_, _, _, _, _, h2221, _, _) => {
                                    h2221
                                }
                            });
                            let a2227 = {
                                let a2224 = f615(match v0.clone() {
                                    Adt54::C0(_, _, _, _, _, _, h2222, _) => {
                                        h2222
                                    }
                                });
                                let a2225 = f610(match v0 {
                                    Adt54::C0(_, _, _, _, _, _, _, h2223) => {
                                        h2223
                                    }
                                });
                                append_list(a2224, a2225)
                            };
                            append_list(a2226, a2227)
                        };
                        append_list(a2228, a2229)
                    };
                    append_list(a2230, a2231)
                };
                append_list(a2232, a2233)
            };
            append_list(a2234, a2235)
        };
        append_list(a2236, a2237)
    })
}

pub fn f565(v0: Adt56) -> Adt6 {
    Adt6::C5({
        let a2253 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt56::C0(h2238, _, _, _, _, _) => {
                h2238
            }
        })), List::<(Str, Adt6)>::nil());
        let a2254 = {
            let a2251 = f403(Str::lit("title"), match v0.clone() {
                Adt56::C0(_, h2239, _, _, _, _) => {
                    h2239
                }
            });
            let a2252 = {
                let a2249 = f496(match v0.clone() {
                    Adt56::C0(_, _, h2240, _, _, _) => {
                        h2240
                    }
                });
                let a2250 = {
                    let a2247 = f403(Str::lit("description"), match v0.clone() {
                        Adt56::C0(_, _, _, h2241, _, _) => {
                            h2241
                        }
                    });
                    let a2248 = {
                        let a2245 = {
                            let m2243 = match v0.clone() {
                                Adt56::C0(_, _, _, _, h2242, _) => {
                                    h2242
                                }
                            };
                            match m2243 {
                                None => {
                                    List::<(Str, Adt6)>::nil()
                                }
                                Some(v1) => {
                                    List::cons((Str::lit("arguments"), Adt6::C4(f617(v1))), List::<(Str, Adt6)>::nil())
                                }
                            }
                        };
                        let a2246 = f610(match v0 {
                            Adt56::C0(_, _, _, _, _, h2244) => {
                                h2244
                            }
                        });
                        append_list(a2245, a2246)
                    };
                    append_list(a2247, a2248)
                };
                append_list(a2249, a2250)
            };
            append_list(a2251, a2252)
        };
        append_list(a2253, a2254)
    })
}

pub fn f566(v0: Adt59) -> R<Option<Adt6>> {
    let m2256 = f561(match v0.clone() {
        Adt59::C0(_, h2255) => {
            h2255
        }
    })?;
    match m2256 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f618(match v0 {
                Adt59::C0(h2257, _) => {
                    h2257
                }
            }))), List::cons((Str::lit("content"), v1), List::<(Str, Adt6)>::nil())))))
        }
    }
}

pub fn f567(v0: Str, v1: Option<Adt6>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("method"), Adt6::C3(v0)), f407(Str::lit("params"), v1)))
}

pub fn f568(v0: Adt65) -> Adt6 {
    let m2258 = v0;
    match m2258 {
        Adt65::C0(v1) => {
            f619(v1)
        }
        Adt65::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("mode"), Adt6::C3(Str::lit("url"))), List::cons((Str::lit("message"), Adt6::C3(match v2.clone() {
                Adt68::C0(h2259, _) => {
                    h2259
                }
            })), List::cons((Str::lit("url"), Adt6::C3(match v2 {
                Adt68::C0(_, h2260) => {
                    h2260
                }
            })), List::<(Str, Adt6)>::nil()))))
        }
    }
}

pub fn f569(v0: Adt69) -> R<Option<Adt6>> {
    let m2262 = f620(match v0.clone() {
        Adt69::C0(h2261, _, _, _, _, _, _, _, _, _) => {
            h2261
        }
    })?;
    match m2262 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("messages"), Adt6::C4(v1)), {
                let a2264 = f621(v0.clone());
                let a2265 = List::cons((Str::lit("maxTokens"), f486(match v0.clone() {
                    Adt69::C0(_, _, _, _, _, h2263, _, _, _, _) => {
                        h2263
                    }
                })?), f622(v0));
                append_list(a2264, a2265)
            }))))
        }
    }
}

pub fn f570(v0: List<Adt2>) -> List<Adt6> {
    f292(f623(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f571(v0: List<u64>) -> List<u64> {
    let a2268 = f624();
    let a2269 = {
        let a2266 = v0;
        let a2267 = f625();
        append_list(a2266, a2267)
    };
    append_list(a2268, a2269)
}

pub fn f572(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f626(v0, 0u64, v1)
}

pub fn f573(p2276: List<u64>) -> bool {
    let m2275 = p2276;
    {
        let mut m2275 = m2275;
        loop {
            let step = {
                let v0 = m2275;
                let m2270 = v0;
                match m2270.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2271 = v1;
                        let a2272 = 48u64;
                        if if nat_eq(a2271, a2272) {
                            true
                        } else {
                            let a2273 = v1;
                            let a2274 = 46u64;
                            nat_eq(a2273, a2274)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2275 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: Adt134) -> bool {
    let m2277 = v0;
    match m2277 {
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

pub fn f575(v0: Adt14) -> Adt134 {
    let m2278 = v0;
    match m2278 {
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

pub fn f576(v0: Adt0, v1: Adt32) -> Adt122 {
    Adt122::C0(v1, Adt123::C0(Some(match v0 {
        Adt0::C0(h2279, _, _, _, _, _, _) => {
            h2279
        }
    }), None::<Adt5>), List::<Adt124>::nil())
}

pub fn f577(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2280, _, _, _) => {
            h2280
        }
    }, f171(), match v0.clone() {
        Adt0::C0(_, _, h2281, _, _, _, _) => {
            h2281
        }
    }, match v0 {
        Adt0::C0(_, h2282, _, _, _, _, _) => {
            h2282
        }
    })
}

pub fn f578(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt122, Adt125> {
    let m2283 = v2;
    match m2283 {
        Ok(v4) => {
            if f627(f575(v1.clone()), v4.clone()) {
                f628(v0, v1, v4)
            } else {
                Err::<Adt122, Adt125>(Adt125::C4(f629()))
            }
        }
        Err(v3) => {
            Err::<Adt122, Adt125>(Adt125::C4(v3))
        }
    }
}

pub fn f579(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14) -> R<Result<Adt32, Adt84>> {
    f630(v0.clone(), v1, v2, v3, v4.clone(), f631(v0, v4)?)
}

pub fn f580(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt126> {
    let m2284 = f173(v0.clone(), v1);
    match m2284 {
        Adt129::C0 => {
            Ok::<Option<Str>, Adt126>(None::<Str>)
        }
        Adt129::C1(v2) => {
            let m2285 = f220(v2);
            match m2285 {
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

pub fn f581(v0: Str, v1: Adt5) -> Result<Str, Adt126> {
    let m2286 = f173(v0.clone(), v1);
    match m2286 {
        Adt129::C0 => {
            Err::<Str, Adt126>(Adt126::C7(v0))
        }
        Adt129::C1(v2) => {
            let m2287 = f220(v2);
            match m2287 {
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

pub fn f582(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt126> {
    let m2288 = f173(v0.clone(), v1);
    match m2288 {
        Adt129::C0 => {
            Ok::<Option<Adt5>, Adt126>(None::<Adt5>)
        }
        Adt129::C1(v2) => {
            let m2289 = f246(v2);
            match m2289 {
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

pub fn f583(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt126> {
    let m2290 = f173(v0.clone(), v1);
    match m2290 {
        Adt129::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt126>(None::<List<(Str, Str)>>)
        }
        Adt129::C1(v2) => {
            let m2291 = f246(v2);
            match m2291 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt126>(Adt126::C8(v0))
                }
                Some(v3) => {
                    let m2293 = f632(v0, match v3 {
                        Adt5::C0(h2292) => {
                            h2292
                        }
                    });
                    match m2293 {
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

pub fn f584(v0: Str, v1: Adt5) -> Result<Adt5, Adt126> {
    let m2294 = f173(v0.clone(), v1);
    match m2294 {
        Adt129::C0 => {
            Err::<Adt5, Adt126>(Adt126::C7(v0))
        }
        Adt129::C1(v2) => {
            let m2295 = f246(v2);
            match m2295 {
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

pub fn f585(v0: Adt5) -> Result<Adt25, Adt126> {
    let m2296 = f581(Str::lit("type"), v0.clone());
    match m2296 {
        Ok(v1) => {
            let a2297 = v1.clone();
            let a2298 = Str::lit("ref/prompt");
            if equal(a2297, a2298) {
                let m2299 = f581(Str::lit("name"), v0.clone());
                match m2299 {
                    Ok(v2) => {
                        let m2300 = f580(Str::lit("title"), v0);
                        match m2300 {
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
                let a2301 = v1;
                let a2302 = Str::lit("ref/resource");
                if equal(a2301, a2302) {
                    let m2303 = f581(Str::lit("uri"), v0);
                    match m2303 {
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

pub fn f586(v0: Adt5) -> Result<Adt27, Adt126> {
    let m2304 = f581(Str::lit("name"), v0.clone());
    match m2304 {
        Ok(v1) => {
            let m2305 = f581(Str::lit("value"), v0);
            match m2305 {
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

pub fn f587(v0: Adt5) -> Result<Option<Adt28>, Adt126> {
    let m2306 = f173(Str::lit("context"), v0);
    match m2306 {
        Adt129::C0 => {
            Ok::<Option<Adt28>, Adt126>(None::<Adt28>)
        }
        Adt129::C1(v1) => {
            let m2307 = f246(v1);
            match m2307 {
                None => {
                    Err::<Option<Adt28>, Adt126>(Adt126::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2308 = f583(Str::lit("arguments"), v2);
                    match m2308 {
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

pub fn f588(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt125> {
    if v0 {
        None::<Adt125>
    } else {
        Some(Adt125::C1(v1, v2))
    }
}

pub fn f589(v0: Str, p2313: List<Str>) -> bool {
    let m2312 = p2313;
    {
        let mut m2312 = m2312;
        loop {
            let step = {
                let v1 = m2312;
                let m2309 = v1;
                match m2309.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2310 = v2;
                        let a2311 = v0.clone();
                        if equal(a2310, a2311) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Option<Adt107>, v1: List<Adt109>, v2: Adt5) -> R<Option<Adt107>> {
    let m2314 = v0;
    match m2314 {
        None => {
            f633(v1, f634(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f591(v0: Option<Str>, v1: Adt139, v2: Adt5) -> Option<Adt107> {
    let m2315 = v1.clone();
    match m2315 {
        Adt139::C0 => {
            None::<Adt107>
        }
        Adt139::C1 => {
            f635(v0, f636(v1, v2))
        }
        Adt139::C2 => {
            f635(v0, f636(v1, v2))
        }
        Adt139::C3 => {
            f635(v0, f636(v1, v2))
        }
    }
}

pub fn f592(v0: Adt129, v1: Adt129) -> Adt148 {
    let m2316 = v0;
    match m2316 {
        Adt129::C0 => {
            Adt148::C1
        }
        Adt129::C1(_) => {
            let m2317 = v1;
            match m2317 {
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

pub fn f593(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt126> {
    let m2318 = f220(v1);
    match m2318 {
        None => {
            Err::<Adt16, Adt126>(Adt126::C5)
        }
        Some(v3) => {
            let m2319 = f637(v2);
            match m2319 {
                None => {
                    Err::<Adt16, Adt126>(Adt126::C6)
                }
                Some(v4) => {
                    let m2320 = f582(f638(), v0.clone());
                    match m2320 {
                        Ok(v6) => {
                            let m2321 = f639(v0.clone());
                            match m2321 {
                                Ok(v8) => {
                                    let m2322 = f640(v0.clone());
                                    match m2322 {
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

pub fn f594() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f595(v0: u64, v1: u64) -> u64 {
    let a2323 = v0;
    let a2324 = v1;
    let a2325 = 0u64;
    nat_rem(a2323, a2324, a2325)
}

pub fn f596(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2328 = {
            let a2326 = v0;
            let a2327 = 4u64;
            nat_mul(a2326, a2327)?
        };
        let a2329 = f598(v1, 16u64);
        nat_add(a2328, a2329)?
    }, List::<u64>::nil()))
}

pub fn f597(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2332 = {
            let a2330 = v0;
            let a2331 = 4u64;
            nat_mul(a2330, a2331)?
        };
        let a2333 = f598(v1, 16u64);
        nat_add(a2332, a2333)?
    }, List::cons({
        let a2336 = {
            let a2334 = f595(v1, 16u64);
            let a2335 = 16u64;
            nat_mul(a2334, a2335)?
        };
        let a2337 = f598(v2, 4u64);
        nat_add(a2336, a2337)?
    }, List::<u64>::nil())))
}

pub fn f598(v0: u64, v1: u64) -> u64 {
    let a2338 = v0;
    let a2339 = v1;
    let a2340 = 0u64;
    nat_quot(a2338, a2339, a2340)
}

pub fn f599(p2344: List<Adt6>) -> bool {
    let m2343 = p2344;
    {
        let mut m2343 = m2343;
        loop {
            let step = {
                let v0 = m2343;
                let m2341 = v0;
                match m2341.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2342 = f220(v1);
                        match m2342 {
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
                Ok(next) => { m2343 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f600(p2348: List<Adt6>) -> bool {
    let m2347 = p2348;
    {
        let mut m2347 = m2347;
        loop {
            let step = {
                let v0 = m2347;
                let m2345 = v0;
                match m2345.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2346 = f220(v1);
                        match m2346 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f462(v3) {
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
                Ok(next) => { m2347 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f601(v0: Str) -> Option<Adt13> {
    let a2349 = v0.clone();
    let a2350 = Str::lit("string");
    if equal(a2349, a2350) {
        Some(Adt13::C0)
    } else {
        let a2351 = v0.clone();
        let a2352 = Str::lit("integer");
        if equal(a2351, a2352) {
            Some(Adt13::C1)
        } else {
            let a2353 = v0;
            let a2354 = Str::lit("boolean");
            if equal(a2353, a2354) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f602(p2363: List<u64>, p2364: List<u64>) -> R<List<u64>> {
    let m2362 = (p2363, p2364);
    {
        let mut m2362 = m2362;
        loop {
            let step = {
                let (v0, v1) = m2362;
                let m2355 = v0;
                match m2355.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2356 = 65u64;
                        let a2357 = v2;
                        if if nat_le(a2356, a2357) {
                            let a2358 = v2;
                            let a2359 = 90u64;
                            nat_le(a2358, a2359)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2360 = v2;
                                let a2361 = 32u64;
                                nat_add(a2360, a2361)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2362 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f603(p2371: List<u64>, p2372: List<u64>) -> bool {
    let m2370 = (p2371, p2372);
    {
        let mut m2370 = m2370;
        loop {
            let step = {
                let (v0, v1) = m2370;
                let m2365 = v0;
                match m2365.uncons() {
                    None => {
                        let m2366 = v1;
                        match m2366.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2367 = v1;
                        match m2367.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2368 = v4;
                                let a2369 = v6;
                                if nat_eq(a2368, a2369) {
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
                Ok(next) => { m2370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f604(p2384: u64, p2385: i64, p2386: i64) -> R<bool> {
    let m2383 = (p2384, (p2385, p2386));
    {
        let mut m2383 = m2383;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2383;
                let m2373 = v0;
                if m2373 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2373 - 1;
                    let m2376 = {
                        let a2374 = v2;
                        let a2375 = 0i64;
                        compare(a2374, a2375)
                    };
                    if match m2376 {
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
                        if f369({
                            let a2377 = v1;
                            let a2378 = 10i64;
                            let a2379 = 0i64;
                            int_rem(a2377, a2378, a2379)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2380 = v1;
                                let a2381 = 10i64;
                                let a2382 = 0i64;
                                int_quot(a2380, a2381, a2382)?
                            }, f114(v2, 1i64)?)))
                        } else {
                            Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                        }
                    } else {
                        Err::<(u64, (i64, i64)), R<bool>>(Ok(true))
                    }
                }
            };
            match step {
                Ok(next) => { m2383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2387 = f641(v0.clone());
    let a2388 = f641(v1.clone());
    if nat_eq(a2387, a2388) {
        let m2389 = v0;
        match m2389 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2390 = f245(v1);
                match m2390 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2391 = v3;
                        let a2392 = v4;
                        if equal(a2391, a2392) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2393 = f467(v1);
                match m2393 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f642(f469(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2394 = f220(v1);
                match m2394 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2395 = v7;
                        let a2396 = v8;
                        if equal(a2395, a2396) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2397 = f315(v1);
                match m2397 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f643(f644(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2398 = f246(v1);
                match m2398 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2400 = f473(v11.clone(), 0u64)?;
                        let a2401 = f473(match v12.clone() {
                            Adt5::C0(h2399) => {
                                h2399
                            }
                        }, 0u64)?;
                        if nat_eq(a2400, a2401) {
                            Ok(f643(f645(v11, match v12 {
                                Adt5::C0(h2402) => {
                                    h2402
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

pub fn f606(v0: Adt147, v1: Adt147) -> Adt147 {
    let m2403 = v0;
    match m2403 {
        Adt147::C0 => {
            f646(v1)
        }
        Adt147::C1 => {
            Adt147::C1
        }
        Adt147::C2 => {
            v1
        }
    }
}

pub fn f607(v0: i64, v1: i64, v2: i64, v3: i64) -> R<Adt147> {
    let m2404 = f555(f647(v0, v1)?, f647(v2, v3)?);
    match m2404 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f648(v0, f649(v0)?, v2, f649(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f608(v0: i64) -> R<i64> {
    let m2407 = {
        let a2405 = v0;
        let a2406 = 0i64;
        compare(a2405, a2406)
    };
    if match m2407 {
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
        let a2408 = v0;
        int_neg(a2408)
    } else {
        Ok(v0)
    }
}

pub fn f609(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2409 = v0;
    match m2409 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2421 = f403(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2410, _, _, _, _) => {
                        h2410
                    }
                });
                let a2422 = {
                    let a2419 = f408(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2411, _, _, _) => {
                            h2411
                        }
                    });
                    let a2420 = {
                        let a2417 = f408(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2412, _, _) => {
                                h2412
                            }
                        });
                        let a2418 = {
                            let a2415 = f408(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2413, _) => {
                                    h2413
                                }
                            });
                            let a2416 = f408(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2414) => {
                                    h2414
                                }
                            });
                            append_list(a2415, a2416)
                        };
                        append_list(a2417, a2418)
                    };
                    append_list(a2419, a2420)
                };
                append_list(a2421, a2422)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f610(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f434(Str::lit("_meta"), v0)
}

pub fn f611(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2423, _, _) => {
            h2423
        }
    })), {
        let a2426 = f615(match v0.clone() {
            Adt39::C0(_, h2424, _) => {
                h2424
            }
        });
        let a2427 = f610(match v0 {
            Adt39::C0(_, _, h2425) => {
                h2425
            }
        });
        append_list(a2426, a2427)
    })))
}

pub fn f612(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2428 = f615(v3);
        let a2429 = f610(v4);
        append_list(a2428, a2429)
    }))))
}

pub fn f613(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2453 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2430, _, _, _, _, _, _, _, _) => {
                h2430
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2454 = {
            let a2451 = f403(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2431, _, _, _, _, _, _, _) => {
                    h2431
                }
            });
            let a2452 = {
                let a2449 = f496(match v0.clone() {
                    Adt44::C0(_, _, h2432, _, _, _, _, _, _) => {
                        h2432
                    }
                });
                let a2450 = {
                    let a2447 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2433, _, _, _, _, _) => {
                            h2433
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2448 = {
                        let a2445 = f403(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2434, _, _, _, _) => {
                                h2434
                            }
                        });
                        let a2446 = {
                            let a2443 = f403(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2435, _, _, _) => {
                                    h2435
                                }
                            });
                            let a2444 = {
                                let a2441 = f615(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2436, _, _) => {
                                        h2436
                                    }
                                });
                                let a2442 = {
                                    let a2439 = f616(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2437, _) => {
                                            h2437
                                        }
                                    })?;
                                    let a2440 = f610(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2438) => {
                                            h2438
                                        }
                                    });
                                    append_list(a2439, a2440)
                                };
                                append_list(a2441, a2442)
                            };
                            append_list(a2443, a2444)
                        };
                        append_list(a2445, a2446)
                    };
                    append_list(a2447, a2448)
                };
                append_list(a2449, a2450)
            };
            append_list(a2451, a2452)
        };
        append_list(a2453, a2454)
    }))
}

pub fn f614(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f563(match v0.clone() {
        Adt45::C0(h2455, _, _) => {
            h2455
        }
    })), {
        let a2458 = f615(match v0.clone() {
            Adt45::C0(_, h2456, _) => {
                h2456
            }
        });
        let a2459 = f610(match v0 {
            Adt45::C0(_, _, h2457) => {
                h2457
            }
        });
        append_list(a2458, a2459)
    })))
}

pub fn f615(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2460 = v0;
    match m2460 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2466 = f650(match v1.clone() {
                    Adt40::C0(h2461, _, _) => {
                        h2461
                    }
                });
                let a2467 = {
                    let a2464 = f651(match v1.clone() {
                        Adt40::C0(_, h2462, _) => {
                            h2462
                        }
                    });
                    let a2465 = f403(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2463) => {
                            h2463
                        }
                    });
                    append_list(a2464, a2465)
                };
                append_list(a2466, a2467)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f616(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2468 = v0;
    match m2468 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f486(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f617(v0: List<Adt57>) -> List<Adt6> {
    f292(f652(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f618(v0: Adt41) -> Str {
    let m2469 = v0;
    match m2469 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f619(v0: Adt66) -> Adt6 {
    Adt6::C5({
        let a2476 = {
            let m2471 = match v0.clone() {
                Adt66::C0(h2470, _, _) => {
                    h2470
                }
            };
            match m2471 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(_) => {
                    List::cons((Str::lit("mode"), Adt6::C3(Str::lit("form"))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2477 = List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
            Adt66::C0(_, h2472, _) => {
                h2472
            }
        })), List::cons((Str::lit("requestedSchema"), Adt6::C5({
            let m2475 = match v0 {
                Adt66::C0(_, _, h2474) => {
                    h2474
                }
            };
            match m2475 {
                Adt5::C0(h2473) => {
                    h2473
                }
            }
        })), List::<(Str, Adt6)>::nil()));
        append_list(a2476, a2477)
    })
}

pub fn f620(v0: List<Adt70>) -> R<Option<List<Adt6>>> {
    let m2478 = f653(v0, List::<Adt6>::nil())?;
    match m2478 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f292(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f621(v0: Adt69) -> List<(Str, Adt6)> {
    let a2489 = {
        let m2480 = match v0.clone() {
            Adt69::C0(_, h2479, _, _, _, _, _, _, _, _) => {
                h2479
            }
        };
        match m2480 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("modelPreferences"), f654(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2490 = {
        let a2487 = f403(Str::lit("systemPrompt"), match v0.clone() {
            Adt69::C0(_, _, h2481, _, _, _, _, _, _, _) => {
                h2481
            }
        });
        let a2488 = {
            let a2485 = {
                let m2483 = match v0.clone() {
                    Adt69::C0(_, _, _, h2482, _, _, _, _, _, _) => {
                        h2482
                    }
                };
                match m2483 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("includeContext"), Adt6::C3(f655(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2486 = f656(Str::lit("temperature"), match v0 {
                Adt69::C0(_, _, _, _, h2484, _, _, _, _, _) => {
                    h2484
                }
            });
            append_list(a2485, a2486)
        };
        append_list(a2487, a2488)
    };
    append_list(a2489, a2490)
}

pub fn f622(v0: Adt69) -> List<(Str, Adt6)> {
    let a2502 = {
        let m2492 = match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, h2491, _, _, _) => {
                h2491
            }
        };
        match m2492 {
            None => {
                List::<(Str, Adt6)>::nil()
            }
            Some(v1) => {
                List::cons((Str::lit("stopSequences"), f401(v1)), List::<(Str, Adt6)>::nil())
            }
        }
    };
    let a2503 = {
        let a2500 = f434(Str::lit("metadata"), match v0.clone() {
            Adt69::C0(_, _, _, _, _, _, _, h2493, _, _) => {
                h2493
            }
        });
        let a2501 = {
            let a2498 = {
                let m2495 = match v0.clone() {
                    Adt69::C0(_, _, _, _, _, _, _, _, h2494, _) => {
                        h2494
                    }
                };
                match m2495 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v2) => {
                        List::cons((Str::lit("tools"), Adt6::C4(f405(v2))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            let a2499 = {
                let m2497 = match v0 {
                    Adt69::C0(_, _, _, _, _, _, _, _, _, h2496) => {
                        h2496
                    }
                };
                match m2497 {
                    None => {
                        List::<(Str, Adt6)>::nil()
                    }
                    Some(v3) => {
                        List::cons((Str::lit("toolChoice"), Adt6::C5(f657(v3))), List::<(Str, Adt6)>::nil())
                    }
                }
            };
            append_list(a2498, a2499)
        };
        append_list(a2500, a2501)
    };
    append_list(a2502, a2503)
}

pub fn f623(p2506: List<Adt2>, p2507: List<Adt6>) -> List<Adt6> {
    let m2505 = (p2506, p2507);
    {
        let mut m2505 = m2505;
        loop {
            let step = {
                let (v0, v1) = m2505;
                let m2504 = v0;
                match m2504.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f658(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2505 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f624() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f625() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f626(p2512: List<u64>, p2513: u64, p2514: List<u64>) -> List<u64> {
    let m2511 = (p2512, (p2513, p2514));
    {
        let mut m2511 = m2511;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2511;
                let m2508 = v0;
                match m2508.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f659(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f660(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2509, _) => {
                                h2509
                            }
                        }, match v5 {
                            Adt150::C0(_, h2510) => {
                                h2510
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2511 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627(v0: Adt134, v1: Adt32) -> bool {
    let m2515 = v1;
    match m2515 {
        Adt32::C0(_) => {
            f661(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f661(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f661(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f661(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f661(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f661(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f661(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f661(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f661(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f661(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f661(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f661(v0.clone(), Adt134::C10) {
                true
            } else {
                f661(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f662(v0)
        }
        Adt32::C13(_) => {
            f663(v0)
        }
    }
}

pub fn f628(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt122, Adt125> {
    let m2516 = v2.clone();
    match m2516 {
        Adt32::C0(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt122, Adt125>(f667(v0, f665(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt122, Adt125>(f576(v0, v2))
        }
        Adt32::C12(v3) => {
            f664(v0, f665(v1), v3)
        }
        Adt32::C13(v4) => {
            f666(v0, f665(v1), v4)
        }
    }
}

pub fn f629() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f630(v0: Bytes, v1: u64, v2: u64, v3: Fn0, v4: Adt14, v5: Result<Adt14, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2517 = v5;
    match m2517 {
        Ok(v7) => {
            f668(v0, v4, f669(v1, v2, v3, v7)?)
        }
        Err(v6) => {
            Ok(Err::<Adt32, Adt84>(v6))
        }
    }
}

pub fn f631(v0: Bytes, v1: Adt14) -> R<Result<Adt14, Adt84>> {
    let m2518 = v1.clone();
    match m2518 {
        Adt14::C0(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C1(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C2(v2) => {
            f670(v0, v2)
        }
        Adt14::C3(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C4(v3) => {
            f671(v0, v3)
        }
        Adt14::C5(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C6(_) => {
            Ok(Ok::<Adt14, Adt84>(v1))
        }
        Adt14::C7(v4) => {
            f672(v0, v4)
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

pub fn f632(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt126> {
    let m2519 = f673(v0, v1, List::<(Str, Str)>::nil());
    match m2519 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt126>(f674(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt126>(v3)
        }
    }
}

pub fn f633(p2523: List<Adt109>, v1: Adt5) -> R<Option<Adt107>> {
    let m2522 = p2523;
    {
        let mut m2522 = m2522;
        loop {
            let step = {
                let v0 = m2522;
                let m2520 = v0;
                match m2520.uncons() {
                    None => {
                        Err::<List<Adt109>, R<Option<Adt107>>>(Ok(None::<Adt107>))
                    }
                    Some((v2, v3)) => {
                        let m2521 = f675(v2, v1.clone())?;
                        match m2521 {
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
                Ok(next) => { m2522 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f634(v0: Adt5) -> Adt5 {
    let m2524 = f173(Str::lit("arguments"), v0);
    match m2524 {
        Adt129::C0 => {
            f521()
        }
        Adt129::C1(v1) => {
            let m2525 = f246(v1);
            match m2525 {
                None => {
                    f521()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f635(v0: Option<Str>, v1: Option<Str>) -> Option<Adt107> {
    let m2526 = v0;
    match m2526 {
        None => {
            Some(Adt107::C2)
        }
        Some(v2) => {
            let m2527 = v1;
            match m2527 {
                None => {
                    None::<Adt107>
                }
                Some(v3) => {
                    let a2528 = v2;
                    let a2529 = v3;
                    if equal(a2528, a2529) {
                        None::<Adt107>
                    } else {
                        Some(Adt107::C5)
                    }
                }
            }
        }
    }
}

pub fn f636(v0: Adt139, v1: Adt5) -> Option<Str> {
    let m2530 = v0;
    match m2530 {
        Adt139::C0 => {
            None::<Str>
        }
        Adt139::C1 => {
            let m2531 = f173(Str::lit("name"), v1);
            match m2531 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v2) => {
                    f220(v2)
                }
            }
        }
        Adt139::C2 => {
            let m2532 = f173(Str::lit("uri"), v1);
            match m2532 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v3) => {
                    f220(v3)
                }
            }
        }
        Adt139::C3 => {
            let m2533 = f173(Str::lit("taskId"), v1);
            match m2533 {
                Adt129::C0 => {
                    None::<Str>
                }
                Adt129::C1(v4) => {
                    f220(v4)
                }
            }
        }
    }
}

pub fn f637(v0: Adt6) -> Option<Adt17> {
    let m2534 = f246(v0);
    match m2534 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2535 = f676(Str::lit("experimental"), v1.clone());
            match m2535 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2536 = f676(Str::lit("roots"), v1.clone());
                    match m2536 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2537 = f676(Str::lit("sampling"), v1.clone());
                            match m2537 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2538 = f676(Str::lit("elicitation"), v1.clone());
                                    match m2538 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2539 = f676(Str::lit("extensions"), v1);
                                            match m2539 {
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

pub fn f638() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f639(v0: Adt5) -> Result<Option<Adt18>, Adt126> {
    let m2540 = f580(f677(), v0);
    match m2540 {
        Ok(v2) => {
            let m2541 = v2;
            match m2541 {
                None => {
                    Ok::<Option<Adt18>, Adt126>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2542 = f678(v3);
                    match m2542 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt126>(Adt126::C8(f677()))
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

pub fn f640(v0: Adt5) -> Result<Option<Adt19>, Adt126> {
    let m2543 = f173(f679(), v0);
    match m2543 {
        Adt129::C0 => {
            Ok::<Option<Adt19>, Adt126>(None::<Adt19>)
        }
        Adt129::C1(v1) => {
            let m2544 = f220(v1.clone());
            match m2544 {
                None => {
                    let m2545 = f554(v1);
                    match m2545 {
                        None => {
                            Err::<Option<Adt19>, Adt126>(Adt126::C8(f679()))
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

pub fn f641(v0: Adt6) -> u64 {
    let m2546 = v0;
    match m2546 {
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

pub fn f642(v0: Adt147) -> bool {
    let m2547 = v0;
    match m2547 {
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

pub fn f643(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2548 = v0;
    match m2548 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2549 = v2;
                let a2550 = v1;
                append_list(a2549, a2550)
            })
        }
    }
}

pub fn f644(p2555: List<Adt6>, p2556: List<Adt6>, p2557: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2554 = (p2555, (p2556, p2557));
    {
        let mut m2554 = m2554;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2554;
                let m2551 = v0;
                match m2551.uncons() {
                    None => {
                        let m2552 = v1;
                        match m2552.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2553 = v1;
                        match m2553.uncons() {
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
                Ok(next) => { m2554 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f645(p2563: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2564: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2562 = (p2563, p2564);
    {
        let mut m2562 = m2562;
        loop {
            let step = {
                let (v0, v2) = m2562;
                let m2558 = v0;
                match m2558.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2560 = f223({
                            let (h2559, _) = v3;
                            h2559
                        }, v1.clone());
                        match m2560 {
                            Adt129::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt129::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2561) = v3;
                                    h2561
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f646(v0: Adt147) -> Adt147 {
    let m2565 = v0;
    match m2565 {
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

pub fn f647(v0: i64, v1: i64) -> R<i64> {
    f114(f80(f649(v0)?, 0i64)?, v1)
}

pub fn f648(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2566 = v1;
    let a2567 = v3;
    if nat_lt(a2566, a2567) {
        Ok(f555(f680({
            let a2568 = v3;
            let a2569 = v1;
            nat_sub(a2568, a2569)
        }, v0)?, v2))
    } else {
        let a2570 = v3;
        let a2571 = v1;
        if nat_lt(a2570, a2571) {
            Ok(f555(v0, f680({
                let a2572 = v1;
                let a2573 = v3;
                nat_sub(a2572, a2573)
            }, v2)?))
        } else {
            Ok(f555(v0, v2))
        }
    }
}

pub fn f649(v0: i64) -> R<u64> {
    f681(20u64, v0, 0u64)
}

pub fn f650(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2574 = v0;
    match m2574 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f682(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f651(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2575 = v0;
    match m2575 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f652(p2578: List<Adt57>, p2579: List<Adt6>) -> List<Adt6> {
    let m2577 = (p2578, p2579);
    {
        let mut m2577 = m2577;
        loop {
            let step = {
                let (v0, v1) = m2577;
                let m2576 = v0;
                match m2576.uncons() {
                    None => {
                        Err::<(List<Adt57>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt57>, List<Adt6>), List<Adt6>>((v3, List::cons(f683(v2), v1)))
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

pub fn f653(p2583: List<Adt70>, p2584: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2582 = (p2583, p2584);
    {
        let mut m2582 = m2582;
        loop {
            let step = {
                let (v0, v1) = m2582;
                let m2580 = v0;
                match m2580.uncons() {
                    None => {
                        Err::<(List<Adt70>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2581 = f684(v2)?;
                        match m2581 {
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
                Ok(next) => { m2582 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f654(v0: Adt75) -> Adt6 {
    Adt6::C5({
        let a2594 = {
            let m2586 = match v0.clone() {
                Adt75::C0(h2585, _, _, _) => {
                    h2585
                }
            };
            match m2586 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v1) => {
                    List::cons((Str::lit("hints"), Adt6::C4(f292(f685(v1, List::<Adt6>::nil()), List::<Adt6>::nil()))), List::<(Str, Adt6)>::nil())
                }
            }
        };
        let a2595 = {
            let a2592 = f656(Str::lit("costPriority"), match v0.clone() {
                Adt75::C0(_, h2587, _, _) => {
                    h2587
                }
            });
            let a2593 = {
                let a2590 = f656(Str::lit("speedPriority"), match v0.clone() {
                    Adt75::C0(_, _, h2588, _) => {
                        h2588
                    }
                });
                let a2591 = f656(Str::lit("intelligencePriority"), match v0 {
                    Adt75::C0(_, _, _, h2589) => {
                        h2589
                    }
                });
                append_list(a2590, a2591)
            };
            append_list(a2592, a2593)
        };
        append_list(a2594, a2595)
    })
}

pub fn f655(v0: Adt77) -> Str {
    let m2596 = v0;
    match m2596 {
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

pub fn f656(v0: Str, v1: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2597 = v1;
    match m2597 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C2(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f657(v0: Adt78) -> List<(Str, Adt6)> {
    let m2599 = match v0 {
        Adt78::C0(h2598) => {
            h2598
        }
    };
    match m2599 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("mode"), Adt6::C3(f686(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f658(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2600, _, _, _) => {
            h2600
        }
    })), {
        let a2606 = f403(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2601, _, _) => {
                h2601
            }
        });
        let a2607 = {
            let a2604 = f687(match v0.clone() {
                Adt2::C0(_, _, h2602, _) => {
                    h2602
                }
            });
            let a2605 = f688(match v0 {
                Adt2::C0(_, _, _, h2603) => {
                    h2603
                }
            });
            append_list(a2604, a2605)
        };
        append_list(a2606, a2607)
    }))
}

pub fn f659(v0: u64, v1: List<u64>) -> List<u64> {
    let a2608 = v0;
    let a2609 = 0u64;
    if nat_eq(a2608, a2609) {
        v1
    } else {
        let a2610 = v0;
        let a2611 = 1u64;
        if nat_eq(a2610, a2611) {
            List::cons(194u64, v1)
        } else {
            let a2612 = v0;
            let a2613 = 2u64;
            if nat_eq(a2612, a2613) {
                List::cons(226u64, v1)
            } else {
                let a2614 = v0;
                let a2615 = 3u64;
                if nat_eq(a2614, a2615) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f660(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2616 = v0;
    let a2617 = 0u64;
    if nat_eq(a2616, a2617) {
        f689(v1, v2)
    } else {
        let a2618 = v0;
        let a2619 = 1u64;
        if nat_eq(a2618, a2619) {
            let a2620 = 128u64;
            let a2621 = v1;
            if if nat_le(a2620, a2621) {
                let a2622 = v1;
                let a2623 = 159u64;
                nat_le(a2622, a2623)
            } else {
                false
            } {
                Adt150::C0(0u64, f690(v2))
            } else {
                f689(v1, List::cons(194u64, v2))
            }
        } else {
            let a2624 = v0;
            let a2625 = 2u64;
            if nat_eq(a2624, a2625) {
                let a2626 = v1;
                let a2627 = 128u64;
                if nat_eq(a2626, a2627) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2628 = v1;
                    let a2629 = 129u64;
                    if nat_eq(a2628, a2629) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f689(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2630 = v0;
                let a2631 = 3u64;
                if nat_eq(a2630, a2631) {
                    let a2632 = 170u64;
                    let a2633 = v1;
                    if if nat_le(a2632, a2633) {
                        let a2634 = v1;
                        let a2635 = 174u64;
                        nat_le(a2634, a2635)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f690(v2))
                    } else {
                        f689(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2636 = 166u64;
                    let a2637 = v1;
                    if if nat_le(a2636, a2637) {
                        let a2638 = v1;
                        let a2639 = 169u64;
                        nat_le(a2638, a2639)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f690(v2))
                    } else {
                        f689(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f661(v0: Adt134, v1: Adt134) -> bool {
    let a2640 = f438(v0);
    let a2641 = f438(v1);
    equal(a2640, a2641)
}

pub fn f662(v0: Adt134) -> bool {
    let m2642 = v0;
    match m2642 {
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

pub fn f663(v0: Adt134) -> bool {
    let m2643 = v0;
    match m2643 {
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

pub fn f664(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt122, Adt125> {
    f691(v0, v2.clone(), f692(match v1 {
        Adt16::C0(_, h2644, _, _, _, _) => {
            h2644
        }
    }, f416(v2)))
}

pub fn f665(v0: Adt14) -> Adt16 {
    let m2645 = v0;
    match m2645 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2646) => {
                    h2646
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2647, _) => {
                    h2647
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2648, _, _, _, _) => {
                    h2648
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2649, _) => {
                    h2649
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2650, _, _, _) => {
                    h2650
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2651, _) => {
                    h2651
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2652, _) => {
                    h2652
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2653, _, _, _, _) => {
                    h2653
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2654, _, _, _) => {
                    h2654
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2655, _) => {
                    h2655
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2656, _, _) => {
                    h2656
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2657, _) => {
                    h2657
                }
            }
        }
    }
}

pub fn f666(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt122, Adt125> {
    f693(v0, v2, f694(match v1 {
        Adt16::C0(_, h2658, _, _, _, _) => {
            h2658
        }
    }))
}

pub fn f667(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt122 {
    Adt122::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2659, _) => {
            h2659
        }
    }), {
        let m2661 = f576(v0, Adt32::C11);
        match m2661 {
            Adt122::C0(_, h2660, _) => {
                h2660
            }
        }
    }, f695(v1, match v2 {
        Adt49::C0(_, h2662) => {
            h2662
        }
    }))
}

pub fn f668(v0: Bytes, v1: Adt14, v2: Result<Adt32, Adt84>) -> R<Result<Adt32, Adt84>> {
    let m2663 = v2;
    match m2663 {
        Ok(v4) => {
            f696(v0, f575(v1.clone()), f697(v1), v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f669(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2664 = v3.clone();
    match m2664 {
        Adt14::C0(_) => {
            let c2666: Fn0 = v2;
            c2666.apply(v3)
        }
        Adt14::C1(_) => {
            let c2667: Fn0 = v2;
            c2667.apply(v3)
        }
        Adt14::C2(_) => {
            if f698(v0, v1) {
                let c2665: Fn0 = v2;
                c2665.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2668: Fn0 = v2;
            c2668.apply(v3)
        }
        Adt14::C4(_) => {
            let c2669: Fn0 = v2;
            c2669.apply(v3)
        }
        Adt14::C5(_) => {
            let c2670: Fn0 = v2;
            c2670.apply(v3)
        }
        Adt14::C6(_) => {
            let c2671: Fn0 = v2;
            c2671.apply(v3)
        }
        Adt14::C7(_) => {
            let c2672: Fn0 = v2;
            c2672.apply(v3)
        }
        Adt14::C8(_) => {
            let c2673: Fn0 = v2;
            c2673.apply(v3)
        }
        Adt14::C9(_) => {
            let c2674: Fn0 = v2;
            c2674.apply(v3)
        }
        Adt14::C10(_) => {
            let c2675: Fn0 = v2;
            c2675.apply(v3)
        }
        Adt14::C11(_) => {
            let c2676: Fn0 = v2;
            c2676.apply(v3)
        }
    }
}

pub fn f670(v0: Bytes, v1: Adt21) -> R<Result<Adt14, Adt84>> {
    let m2680 = f699(v0, Str::lit("tools/call"), match v1.clone() {
        Adt21::C0(_, h2677, _, _, _) => {
            h2677
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, _, h2678) => {
            h2678
        }
    }, match v1.clone() {
        Adt21::C0(_, _, _, h2679, _) => {
            h2679
        }
    })?;
    match m2680 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C2(Adt21::C0(match v1.clone() {
                Adt21::C0(h2681, _, _, _, _) => {
                    h2681
                }
            }, match v1.clone() {
                Adt21::C0(_, h2682, _, _, _) => {
                    h2682
                }
            }, match v1.clone() {
                Adt21::C0(_, _, h2683, _, _) => {
                    h2683
                }
            }, match v1 {
                Adt21::C0(_, _, _, h2684, _) => {
                    h2684
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f671(v0: Bytes, v1: Adt22) -> R<Result<Adt14, Adt84>> {
    let m2688 = f699(v0, Str::lit("resources/read"), match v1.clone() {
        Adt22::C0(_, h2685, _, _) => {
            h2685
        }
    }, match v1.clone() {
        Adt22::C0(_, _, _, h2686) => {
            h2686
        }
    }, match v1.clone() {
        Adt22::C0(_, _, h2687, _) => {
            h2687
        }
    })?;
    match m2688 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C4(Adt22::C0(match v1.clone() {
                Adt22::C0(h2689, _, _, _) => {
                    h2689
                }
            }, match v1.clone() {
                Adt22::C0(_, h2690, _, _) => {
                    h2690
                }
            }, match v1 {
                Adt22::C0(_, _, h2691, _) => {
                    h2691
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f672(v0: Bytes, v1: Adt23) -> R<Result<Adt14, Adt84>> {
    let m2695 = f699(v0, Str::lit("prompts/get"), match v1.clone() {
        Adt23::C0(_, h2692, _, _, _) => {
            h2692
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, _, h2693) => {
            h2693
        }
    }, match v1.clone() {
        Adt23::C0(_, _, _, h2694, _) => {
            h2694
        }
    })?;
    match m2695 {
        Ok(v3) => {
            Ok(Ok::<Adt14, Adt84>(Adt14::C7(Adt23::C0(match v1.clone() {
                Adt23::C0(h2696, _, _, _, _) => {
                    h2696
                }
            }, match v1.clone() {
                Adt23::C0(_, h2697, _, _, _) => {
                    h2697
                }
            }, match v1.clone() {
                Adt23::C0(_, _, h2698, _, _) => {
                    h2698
                }
            }, match v1 {
                Adt23::C0(_, _, _, h2699, _) => {
                    h2699
                }
            }, v3))))
        }
        Err(v2) => {
            Ok(Err::<Adt14, Adt84>(v2))
        }
    }
}

pub fn f673(v0: Str, p2705: List<(Str, Adt6)>, p2706: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt126> {
    let m2704 = (p2705, p2706);
    {
        let mut m2704 = m2704;
        loop {
            let step = {
                let (v1, v2) = m2704;
                let m2700 = v1;
                match m2700.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>(Ok::<List<(Str, Str)>, Adt126>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2702 = f220({
                            let (_, h2701) = v3;
                            h2701
                        });
                        match m2702 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>(Err::<List<(Str, Str)>, Adt126>(Adt126::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt126>>((v4, List::cons(({
                                    let (h2703, _) = v3;
                                    h2703
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f674(p2709: List<(Str, Str)>, p2710: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2708 = (p2709, p2710);
    {
        let mut m2708 = m2708;
        loop {
            let step = {
                let (v0, v1) = m2708;
                let m2707 = v0;
                match m2707.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(v0: Adt109, v1: Adt5) -> R<Option<Adt107>> {
    let m2712 = f700(match v0.clone() {
        Adt109::C0(_, h2711, _, _) => {
            h2711
        }
    }, v1);
    match m2712 {
        Adt129::C0 => {
            Ok(f701(v0))
        }
        Adt129::C1(v2) => {
            if f702(v2.clone()) {
                Ok(f701(v0))
            } else {
                f703(v0, v2)
            }
        }
    }
}

pub fn f676(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2713 = f173(v0, v1);
    match m2713 {
        Adt129::C0 => {
            Some(None::<Adt5>)
        }
        Adt129::C1(v2) => {
            let m2714 = f246(v2);
            match m2714 {
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

pub fn f677() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f678(v0: Str) -> Adt151 {
    let a2715 = v0.clone();
    let a2716 = Str::lit("debug");
    if equal(a2715, a2716) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2717 = v0.clone();
        let a2718 = Str::lit("info");
        if equal(a2717, a2718) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2719 = v0.clone();
            let a2720 = Str::lit("notice");
            if equal(a2719, a2720) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2721 = v0.clone();
                let a2722 = Str::lit("warning");
                if equal(a2721, a2722) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2723 = v0.clone();
                    let a2724 = Str::lit("error");
                    if equal(a2723, a2724) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2725 = v0.clone();
                        let a2726 = Str::lit("critical");
                        if equal(a2725, a2726) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2727 = v0.clone();
                            let a2728 = Str::lit("alert");
                            if equal(a2727, a2728) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2729 = v0;
                                let a2730 = Str::lit("emergency");
                                if equal(a2729, a2730) {
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

pub fn f679() -> Str {
    Str::lit("progressToken")
}

pub fn f680(p2735: u64, p2736: i64) -> R<i64> {
    let m2734 = (p2735, p2736);
    {
        let mut m2734 = m2734;
        loop {
            let step = {
                let (v0, v1) = m2734;
                let m2731 = v0;
                if m2731 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2731 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2732 = v1;
                        let a2733 = 10i64;
                        int_mul(a2732, a2733)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2734 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f681(p2747: u64, p2748: i64, p2749: u64) -> R<u64> {
    let m2746 = (p2747, (p2748, p2749));
    {
        let mut m2746 = m2746;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2746;
                let m2737 = v0;
                if m2737 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2737 - 1;
                    let v4: u64 = {
                        let a2738 = v2;
                        let a2739 = 1u64;
                        nat_add(a2738, a2739)?
                    };
                    let m2742 = {
                        let a2740 = v1;
                        let a2741 = 10i64;
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
                        Err::<(u64, (i64, u64)), R<u64>>(Ok(v4))
                    } else {
                        Ok::<(u64, (i64, u64)), R<u64>>((v3, ({
                            let a2743 = v1;
                            let a2744 = 10i64;
                            let a2745 = 0i64;
                            int_quot(a2743, a2744, a2745)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2746 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f682(v0: List<Adt41>) -> List<Adt6> {
    f292(f704(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f683(v0: Adt57) -> Adt6 {
    Adt6::C5({
        let a2758 = List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt57::C0(h2750, _, _, _) => {
                h2750
            }
        })), List::<(Str, Adt6)>::nil());
        let a2759 = {
            let a2756 = f403(Str::lit("title"), match v0.clone() {
                Adt57::C0(_, h2751, _, _) => {
                    h2751
                }
            });
            let a2757 = {
                let a2754 = f403(Str::lit("description"), match v0.clone() {
                    Adt57::C0(_, _, h2752, _) => {
                        h2752
                    }
                });
                let a2755 = f408(Str::lit("required"), match v0 {
                    Adt57::C0(_, _, _, h2753) => {
                        h2753
                    }
                });
                append_list(a2754, a2755)
            };
            append_list(a2756, a2757)
        };
        append_list(a2758, a2759)
    })
}

pub fn f684(v0: Adt70) -> R<Option<Adt6>> {
    let m2761 = f705(match v0.clone() {
        Adt70::C0(_, h2760, _) => {
            h2760
        }
    })?;
    match m2761 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("role"), Adt6::C3(f618(match v0.clone() {
                Adt70::C0(h2762, _, _) => {
                    h2762
                }
            }))), List::cons((Str::lit("content"), v1), f610(match v0 {
                Adt70::C0(_, _, h2763) => {
                    h2763
                }
            }))))))
        }
    }
}

pub fn f685(p2767: List<Adt76>, p2768: List<Adt6>) -> List<Adt6> {
    let m2766 = (p2767, p2768);
    {
        let mut m2766 = m2766;
        loop {
            let step = {
                let (v0, v1) = m2766;
                let m2764 = v0;
                match m2764.uncons() {
                    None => {
                        Err::<(List<Adt76>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt76>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C5(f403(Str::lit("name"), match v2 {
                            Adt76::C0(h2765) => {
                                h2765
                            }
                        })), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f686(v0: Adt79) -> Str {
    let m2769 = v0;
    match m2769 {
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

pub fn f687(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2770 = v0;
    match m2770 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f351(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f688(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2771 = v0;
    match m2771 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f706(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f689(v0: u64, v1: List<u64>) -> Adt150 {
    let a2772 = v0;
    let a2773 = 32u64;
    if nat_lt(a2772, a2773) {
        if f707(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f690(v1))
        }
    } else {
        let a2774 = v0;
        let a2775 = 127u64;
        if nat_eq(a2774, a2775) {
            Adt150::C0(0u64, f690(v1))
        } else {
            let a2776 = v0;
            let a2777 = 194u64;
            if nat_eq(a2776, a2777) {
                Adt150::C0(1u64, v1)
            } else {
                let a2778 = v0;
                let a2779 = 226u64;
                if nat_eq(a2778, a2779) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f690(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f691(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt122, Adt125> {
    if v2 {
        Ok::<Adt122, Adt125>(f576(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt122, Adt125>(Adt125::C4(Adt84::C8(f708(f416(v1)))))
    }
}

pub fn f692(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2780 = f709(v1.clone());
    if if if bool_not(a2780) {
        true
    } else {
        f441(match v0.clone() {
            Adt17::C0(_, _, _, h2781, _) => {
                h2781
            }
        })
    } {
        let a2782 = f710(v1.clone());
        if bool_not(a2782) {
            true
        } else {
            f441(match v0.clone() {
                Adt17::C0(_, _, h2783, _, _) => {
                    h2783
                }
            })
        }
    } else {
        false
    } {
        let a2784 = f711(v1);
        if bool_not(a2784) {
            true
        } else {
            f441(match v0 {
                Adt17::C0(_, h2785, _, _, _) => {
                    h2785
                }
            })
        }
    } else {
        false
    }
}

pub fn f693(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt122, Adt125> {
    if v2 {
        Ok::<Adt122, Adt125>(f576(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt122, Adt125>(Adt125::C4(Adt84::C8(f712())))
    }
}

pub fn f694(v0: Adt17) -> bool {
    f442(match v0 {
        Adt17::C0(_, _, _, _, h2786) => {
            h2786
        }
    }, f443())
}

pub fn f695(v0: Adt16, v1: List<Adt50>) -> List<Adt124> {
    let m2788 = match v0 {
        Adt16::C0(_, _, _, _, h2787, _) => {
            h2787
        }
    };
    match m2788 {
        None => {
            List::<Adt124>::nil()
        }
        Some(v2) => {
            f713(f714(v2, v1, List::<Adt124>::nil()), List::<Adt124>::nil())
        }
    }
}

pub fn f696(v0: Bytes, v1: Adt134, v2: Str, v3: Adt32) -> R<Result<Adt32, Adt84>> {
    let m2789 = v3.clone();
    match m2789 {
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
            f715(v0, v1, v2, v4)
        }
        Adt32::C13(_) => {
            Ok(Ok::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f697(v0: Adt14) -> Str {
    let m2790 = v0;
    match m2790 {
        Adt14::C0(_) => {
            Str::lit("")
        }
        Adt14::C1(_) => {
            Str::lit("")
        }
        Adt14::C2(v1) => {
            match v1 {
                Adt21::C0(_, h2791, _, _, _) => {
                    h2791
                }
            }
        }
        Adt14::C3(_) => {
            Str::lit("")
        }
        Adt14::C4(v2) => {
            match v2 {
                Adt22::C0(_, h2792, _, _) => {
                    h2792
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
                Adt23::C0(_, h2793, _, _, _) => {
                    h2793
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

pub fn f698(v0: u64, v1: u64) -> bool {
    let a2794 = v1;
    let a2795 = v0;
    nat_lt(a2794, a2795)
}

pub fn f699(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>, v4: Option<Adt5>) -> R<Result<Option<Str>, Adt84>> {
    Ok(f716(f717(v0, v1, v2, v3)?, f718(v4)))
}

pub fn f700(p2801: List<Str>, p2802: Adt5) -> Adt129 {
    let m2800 = (p2801, p2802);
    {
        let mut m2800 = m2800;
        loop {
            let step = {
                let (v0, v1) = m2800;
                let m2796 = v0;
                match m2796.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt129>(Adt129::C0)
                    }
                    Some((v2, v3)) => {
                        let m2797 = f173(v2, v1);
                        match m2797 {
                            Adt129::C0 => {
                                Err::<(List<Str>, Adt5), Adt129>(Adt129::C0)
                            }
                            Adt129::C1(v4) => {
                                let m2798 = v3.clone();
                                match m2798.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt129>(Adt129::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2799 = f246(v4);
                                        match m2799 {
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
                Ok(next) => { m2800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f701(v0: Adt109) -> Option<Adt107> {
    let m2804 = match v0.clone() {
        Adt109::C0(_, _, _, h2803) => {
            h2803
        }
    };
    match m2804 {
        Adt110::C0 => {
            None::<Adt107>
        }
        Adt110::C1 => {
            Some(Adt107::C10(match v0 {
                Adt109::C0(h2805, _, _, _) => {
                    h2805
                }
            }))
        }
        Adt110::C2(_) => {
            None::<Adt107>
        }
    }
}

pub fn f702(v0: Adt6) -> bool {
    let m2806 = v0;
    match m2806 {
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

pub fn f703(v0: Adt109, v1: Adt6) -> R<Option<Adt107>> {
    let m2808 = match v0.clone() {
        Adt109::C0(_, _, _, h2807) => {
            h2807
        }
    };
    match m2808 {
        Adt110::C0 => {
            Ok(Some(Adt107::C8(match v0 {
                Adt109::C0(h2809, _, _, _) => {
                    h2809
                }
            })))
        }
        Adt110::C1 => {
            Ok(Some(Adt107::C10(match v0 {
                Adt109::C0(h2810, _, _, _) => {
                    h2810
                }
            })))
        }
        Adt110::C2(v2) => {
            if f719(match v0.clone() {
                Adt109::C0(_, _, h2811, _) => {
                    h2811
                }
            }, v2, v1)? {
                Ok(None::<Adt107>)
            } else {
                Ok(Some(Adt107::C9(match v0 {
                    Adt109::C0(h2812, _, _, _) => {
                        h2812
                    }
                })))
            }
        }
    }
}

pub fn f704(p2815: List<Adt41>, p2816: List<Adt6>) -> List<Adt6> {
    let m2814 = (p2815, p2816);
    {
        let mut m2814 = m2814;
        loop {
            let step = {
                let (v0, v1) = m2814;
                let m2813 = v0;
                match m2813.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f618(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f705(v0: Adt71) -> R<Option<Adt6>> {
    let m2817 = v0;
    match m2817 {
        Adt71::C0(v1) => {
            f720(v1)
        }
        Adt71::C1(v2) => {
            let m2818 = f721(v2, List::<Adt6>::nil())?;
            match m2818 {
                None => {
                    Ok(None::<Adt6>)
                }
                Some(v3) => {
                    Ok(Some(Adt6::C4(f292(v3, List::<Adt6>::nil()))))
                }
            }
        }
    }
}

pub fn f706(v0: Adt3) -> Str {
    let m2819 = v0;
    match m2819 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f707(v0: u64) -> bool {
    let a2820 = v0;
    let a2821 = 9u64;
    if nat_eq(a2820, a2821) {
        true
    } else {
        let a2822 = v0;
        let a2823 = 10u64;
        if nat_eq(a2822, a2823) {
            true
        } else {
            let a2824 = v0;
            let a2825 = 13u64;
            nat_eq(a2824, a2825)
        }
    }
}

pub fn f708(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f711(v0.clone()) {
        Some(f521())
    } else {
        None::<Adt5>
    }, if f710(v0.clone()) {
        Some(f521())
    } else {
        None::<Adt5>
    }, if f709(v0) {
        Some(f521())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f709(p2830: List<(Str, Adt64)>) -> bool {
    let m2829 = p2830;
    {
        let mut m2829 = m2829;
        loop {
            let step = {
                let v0 = m2829;
                let m2826 = v0;
                match m2826.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2828 = {
                            let (_, h2827) = v1;
                            h2827
                        };
                        match m2828 {
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
                Ok(next) => { m2829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f710(p2835: List<(Str, Adt64)>) -> bool {
    let m2834 = p2835;
    {
        let mut m2834 = m2834;
        loop {
            let step = {
                let v0 = m2834;
                let m2831 = v0;
                match m2831.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2833 = {
                            let (_, h2832) = v1;
                            h2832
                        };
                        match m2833 {
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
                Ok(next) => { m2834 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f711(p2840: List<(Str, Adt64)>) -> bool {
    let m2839 = p2840;
    {
        let mut m2839 = m2839;
        loop {
            let step = {
                let v0 = m2839;
                let m2836 = v0;
                match m2836.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2838 = {
                            let (_, h2837) = v1;
                            h2837
                        };
                        match m2838 {
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
                Ok(next) => { m2839 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f712() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f443(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f713(p2843: List<Adt124>, p2844: List<Adt124>) -> List<Adt124> {
    let m2842 = (p2843, p2844);
    {
        let mut m2842 = m2842;
        loop {
            let step = {
                let (v0, v1) = m2842;
                let m2841 = v0;
                match m2841.uncons() {
                    None => {
                        Err::<(List<Adt124>, List<Adt124>), List<Adt124>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt124>, List<Adt124>), List<Adt124>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f714(v0: Adt19, p2847: List<Adt50>, p2848: List<Adt124>) -> List<Adt124> {
    let m2846 = (p2847, p2848);
    {
        let mut m2846 = m2846;
        loop {
            let step = {
                let (v1, v2) = m2846;
                let m2845 = v1;
                match m2845.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt124>), List<Adt124>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt124>), List<Adt124>>((v4, List::cons(Adt124::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2846 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f715(v0: Bytes, v1: Adt134, v2: Str, v3: Adt81) -> R<Result<Adt32, Adt84>> {
    if f662(v1.clone()) {
        if f722(v3.clone()) {
            let m2849 = f723(v0, f438(v1), v2, v3)?;
            match m2849 {
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
        Ok(Err::<Adt32, Adt84>(Adt84::C12(f438(v1))))
    }
}

pub fn f716(v0: Result<Option<Str>, Adt84>, v1: bool) -> Result<Option<Str>, Adt84> {
    let m2850 = v0;
    match m2850 {
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

pub fn f717(v0: Bytes, v1: Str, v2: Str, v3: Option<Str>) -> R<Result<Option<Str>, Adt84>> {
    let m2851 = v3;
    match m2851 {
        None => {
            Ok(Ok::<Option<Str>, Adt84>(None::<Str>))
        }
        Some(v4) => {
            Ok(f724(f725(v0, v1, v2, v4)?))
        }
    }
}

pub fn f718(v0: Option<Adt5>) -> bool {
    let m2852 = v0;
    match m2852 {
        None => {
            true
        }
        Some(v1) => {
            f726(match v1 {
                Adt5::C0(h2853) => {
                    h2853
                }
            })
        }
    }
}

pub fn f719(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2854 = v0;
    match m2854 {
        Adt13::C0 => {
            let m2855 = f220(v2);
            match m2855 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2856 = v1;
                    let a2857 = v3;
                    Ok(equal(a2856, a2857))
                }
            }
        }
        Adt13::C1 => {
            f727(v1, v2)
        }
        Adt13::C2 => {
            let m2858 = f245(v2);
            match m2858 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2859 = v1;
                        let a2860 = Str::lit("true");
                        Ok(equal(a2859, a2860))
                    } else {
                        let a2861 = v1;
                        let a2862 = Str::lit("false");
                        Ok(equal(a2861, a2862))
                    }
                }
            }
        }
    }
}

pub fn f720(v0: Adt72) -> R<Option<Adt6>> {
    let m2863 = v0;
    match m2863 {
        Adt72::C0(v1) => {
            Ok(Some(f611(v1)))
        }
        Adt72::C1(v2) => {
            Ok(Some(f612(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2864, _, _, _) => {
                    h2864
                }
            }, match v2.clone() {
                Adt42::C0(_, h2865, _, _) => {
                    h2865
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2866, _) => {
                    h2866
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2867) => {
                    h2867
                }
            })))
        }
        Adt72::C2(v3) => {
            Ok(Some(f612(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2868, _, _, _) => {
                    h2868
                }
            }, match v3.clone() {
                Adt43::C0(_, h2869, _, _) => {
                    h2869
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2870, _) => {
                    h2870
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2871) => {
                    h2871
                }
            })))
        }
        Adt72::C3(v4) => {
            Ok(Some(f728(v4)))
        }
        Adt72::C4(v5) => {
            f729(v5)
        }
    }
}

pub fn f721(p2875: List<Adt72>, p2876: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m2874 = (p2875, p2876);
    {
        let mut m2874 = m2874;
        loop {
            let step = {
                let (v0, v1) = m2874;
                let m2872 = v0;
                match m2872.uncons() {
                    None => {
                        Err::<(List<Adt72>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2873 = f720(v2)?;
                        match m2873 {
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
                Ok(next) => { m2874 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f722(v0: Adt81) -> bool {
    let m2877 = v0;
    match m2877 {
        Adt81::C0(v1) => {
            let m2878 = v1;
            match m2878.uncons() {
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

pub fn f723(v0: Bytes, v1: Str, v2: Str, v3: Adt81) -> R<Result<Adt81, Adt84>> {
    let m2879 = v3.clone();
    match m2879 {
        Adt81::C0(_) => {
            Ok(Ok::<Adt81, Adt84>(v3))
        }
        Adt81::C1(v5) => {
            Ok(f730(f731(v0, v1, v2, v5)?, List::<(Str, Adt64)>::nil()))
        }
        Adt81::C2(v6, v7) => {
            Ok(f730(f731(v0, v1, v2, v7)?, v6))
        }
    }
}

pub fn f724(v0: Option<Str>) -> Result<Option<Str>, Adt84> {
    let m2880 = v0;
    match m2880 {
        None => {
            Err::<Option<Str>, Adt84>(Adt84::C10)
        }
        Some(v1) => {
            Ok::<Option<Str>, Adt84>(Some(v1))
        }
    }
}

pub fn f725(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m2884 = f732(v0, {
        let a2881 = v1;
        utf8_encode(a2881)
    }, {
        let a2882 = v2;
        utf8_encode(a2882)
    }, {
        let a2883 = v3;
        utf8_encode(a2883)
    })?;
    match m2884 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a2885 = v4;
            Ok(utf8_decode(a2885))
        }
    }
}

pub fn f726(p2890: List<(Str, Adt6)>) -> bool {
    let m2889 = p2890;
    {
        let mut m2889 = m2889;
        loop {
            let step = {
                let v0 = m2889;
                let m2886 = v0;
                match m2886.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2888 = f246({
                            let (_, h2887) = v1;
                            h2887
                        });
                        match m2888 {
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
                Ok(next) => { m2889 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f727(v0: Str, v1: Adt6) -> R<bool> {
    let m2891 = f467(v1);
    match m2891 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2892 = f222(v2)?;
            match m2892 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2894 = f733(f85({
                        let a2893 = v0;
                        utf8_encode(a2893)
                    })?)?;
                    match m2894 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f369({
                                let a2895 = v3;
                                let a2896 = v4;
                                int_sub(a2895, a2896)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f728(v0: Adt73) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_use"))), List::cons((Str::lit("id"), Adt6::C3(match v0.clone() {
        Adt73::C0(h2897, _, _, _) => {
            h2897
        }
    })), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt73::C0(_, h2898, _, _) => {
            h2898
        }
    })), List::cons((Str::lit("input"), Adt6::C5({
        let m2901 = match v0.clone() {
            Adt73::C0(_, _, h2900, _) => {
                h2900
            }
        };
        match m2901 {
            Adt5::C0(h2899) => {
                h2899
            }
        }
    })), f610(match v0 {
        Adt73::C0(_, _, _, h2902) => {
            h2902
        }
    }))))))
}

pub fn f729(v0: Adt74) -> R<Option<Adt6>> {
    let m2904 = f406(match v0.clone() {
        Adt74::C0(_, h2903, _, _, _) => {
            h2903
        }
    })?;
    match m2904 {
        None => {
            Ok(None::<Adt6>)
        }
        Some(v1) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("tool_result"))), List::cons((Str::lit("toolUseId"), Adt6::C3(match v0.clone() {
                Adt74::C0(h2905, _, _, _, _) => {
                    h2905
                }
            })), List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a2911 = f407(Str::lit("structuredContent"), match v0.clone() {
                    Adt74::C0(_, _, h2906, _, _) => {
                        h2906
                    }
                });
                let a2912 = {
                    let a2909 = f408(Str::lit("isError"), match v0.clone() {
                        Adt74::C0(_, _, _, h2907, _) => {
                            h2907
                        }
                    });
                    let a2910 = f610(match v0 {
                        Adt74::C0(_, _, _, _, h2908) => {
                            h2908
                        }
                    });
                    append_list(a2909, a2910)
                };
                append_list(a2911, a2912)
            }))))))
        }
    }
}

pub fn f730(v0: Option<Str>, v1: List<(Str, Adt64)>) -> Result<Adt81, Adt84> {
    let m2913 = v0;
    match m2913 {
        None => {
            Err::<Adt81, Adt84>(Adt84::C14)
        }
        Some(v2) => {
            let m2914 = v1.clone();
            match m2914.uncons() {
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

pub fn f731(v0: Bytes, v1: Str, v2: Str, v3: Str) -> R<Option<Str>> {
    let m2918 = f734(v0, {
        let a2915 = v1;
        utf8_encode(a2915)
    }, {
        let a2916 = v2;
        utf8_encode(a2916)
    }, {
        let a2917 = v3;
        utf8_encode(a2917)
    })?;
    match m2918 {
        None => {
            Ok(None::<Str>)
        }
        Some(v4) => {
            let a2919 = v4;
            Ok(utf8_decode(a2919))
        }
    }
}

pub fn f732(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a2921 = f735();
    let a2922 = {
        let a2920 = v0.clone();
        length_bytes(a2920)
    };
    if nat_le(a2921, a2922) {
        let a2924 = f736(v3.clone())?;
        let a2925 = {
            let a2923 = v3.clone();
            length_bytes(a2923)
        };
        if nat_eq(a2924, a2925) {
            Ok(None::<Bytes>)
        } else {
            f737(v0, v1, v2, v3.clone(), f736(v3)?)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f733(v0: List<u64>) -> R<Option<i64>> {
    let m2926 = v0.clone();
    match m2926.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2927 = v1;
            let a2928 = 45u64;
            if nat_eq(a2927, a2928) {
                let m2929 = v2.clone();
                match m2929.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2930 = f738(v2, 0i64)?;
                        match m2930 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2931 = v5;
                                    int_neg(a2931)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f738(v0, 0i64)
            }
        }
    }
}

pub fn f734(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Option<Bytes>> {
    let a2933 = f735();
    let a2934 = {
        let a2932 = v0.clone();
        length_bytes(a2932)
    };
    if nat_le(a2933, a2934) {
        Ok(Some({
            let a2937 = f739(v3.clone())?;
            let a2938 = {
                let a2935 = Bytes::lit(&[0x2e]);
                let a2936 = f739(f740(v0, v1, v2, v3)?)?;
                append_bytes(a2935, a2936)
            };
            append_bytes(a2937, a2938)
        }))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f735() -> u64 {
    32u64
}

pub fn f736(v0: Bytes) -> R<u64> {
    let (_, h2939) = f741(Fn3::F742(v0.clone()), {
        let a2940 = v0.clone();
        length_bytes(a2940)
    }, (0u64, {
        let a2941 = v0;
        length_bytes(a2941)
    }))?;
    Ok(h2939)
}

pub fn f737(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes, v4: u64) -> R<Option<Bytes>> {
    let m2945 = {
        let a2942 = v3.clone();
        let a2943 = 0u64;
        let a2944 = v4;
        slice_bytes(a2942, a2943, a2944)
    };
    match m2945 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m2956 = {
                let a2953 = v3.clone();
                let a2954 = {
                    let a2946 = v4;
                    let a2947 = 1u64;
                    nat_add(a2946, a2947)?
                };
                let a2955 = {
                    let a2951 = {
                        let a2948 = v3;
                        length_bytes(a2948)
                    };
                    let a2952 = {
                        let a2949 = v4;
                        let a2950 = 1u64;
                        nat_add(a2949, a2950)?
                    };
                    nat_sub(a2951, a2952)
                };
                slice_bytes(a2953, a2954, a2955)
            };
            match m2956 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    f743(v0, v1, v2, f744(v5)?, f744(v6)?)
                }
            }
        }
    }
}

pub fn f738(p2962: List<u64>, p2963: i64) -> R<Option<i64>> {
    let m2961 = (p2962, p2963);
    {
        let mut m2961 = m2961;
        loop {
            let step = {
                let (v0, v1) = m2961;
                let m2957 = v0;
                match m2957.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2958 = f745(v2);
                        match m2958 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f114({
                                    let a2959 = v1;
                                    let a2960 = 10i64;
                                    int_mul(a2959, a2960)?
                                }, f291(v4))?))
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

pub fn f739(v0: Bytes) -> R<Bytes> {
    let (_, h2964) = f746(Fn4::F747(v0.clone()), {
        let a2965 = v0;
        length_bytes(a2965)
    }, (0u64, f361()))?;
    Ok(h2964)
}

pub fn f740(v0: Bytes, v1: Bytes, v2: Bytes, v3: Bytes) -> R<Bytes> {
    f748(v0, {
        let a2966 = f749(v1, v2);
        let a2967 = v3;
        append_bytes(a2966, a2967)
    })
}

pub fn f741(v0: Fn3, p2971: u64, p2972: (u64, u64)) -> R<(u64, u64)> {
    let m2970 = (p2971, p2972);
    {
        let mut m2970 = m2970;
        loop {
            let step = {
                let (v1, v2) = m2970;
                let m2968 = v1;
                if m2968 == 0 {
                    Err::<(u64, (u64, u64)), R<(u64, u64)>>(Ok(v2))
                } else {
                    let v3 = m2968 - 1;
                    Ok::<(u64, (u64, u64)), R<(u64, u64)>>((v3, {
                        let c2969: Fn3 = v0.clone();
                        c2969.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2970 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f742(v0: Bytes, v1: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a2974 = {
            let (h2973, _) = v1;
            h2973
        };
        let a2975 = 1u64;
        nat_add(a2974, a2975)?
    }, {
        let a2978 = {
            let (_, h2976) = v1;
            h2976
        };
        let a2979 = {
            let a2977 = v0.clone();
            length_bytes(a2977)
        };
        if nat_eq(a2978, a2979) {
            let a2981 = f57(v0, {
                let (h2980, _) = v1;
                h2980
            })?;
            let a2982 = 46u64;
            if nat_eq(a2981, a2982) {
                let (h2983, _) = v1;
                h2983
            } else {
                let (_, h2984) = v1;
                h2984
            }
        } else {
            let (_, h2985) = v1;
            h2985
        }
    }))
}

pub fn f743(v0: Bytes, v1: Bytes, v2: Bytes, v3: Option<Bytes>, v4: Option<Bytes>) -> R<Option<Bytes>> {
    let m2986 = v3;
    match m2986 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v5) => {
            let m2987 = v4;
            match m2987 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    if f750(f740(v0, v1, v2, v5.clone())?, v6)? {
                        Ok(Some(v5))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f744(v0: Bytes) -> R<Option<Bytes>> {
    let a2992 = {
        let a2989 = {
            let a2988 = v0.clone();
            length_bytes(a2988)
        };
        let a2990 = 2u64;
        let a2991 = 0u64;
        nat_rem(a2989, a2990, a2991)
    };
    let a2993 = 0u64;
    if nat_eq(a2992, a2993) {
        let v1: (u64, (Bytes, u64)) = f751(Fn5::F752(v0.clone()), {
            let a2995 = {
                let a2994 = v0;
                length_bytes(a2994)
            };
            let a2996 = 2u64;
            let a2997 = 0u64;
            nat_quot(a2995, a2996, a2997)
        }, (0u64, (f361(), 0u64)))?;
        let a3000 = {
            let (_, h2998) = {
                let (_, h2999) = v1.clone();
                h2999
            };
            h2998
        };
        let a3001 = 0u64;
        if nat_eq(a3000, a3001) {
            Ok(Some({
                let (h3002, _) = {
                    let (_, h3003) = v1;
                    h3003
                };
                h3002
            }))
        } else {
            Ok(None::<Bytes>)
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f745(v0: u64) -> Option<u64> {
    let a3004 = v0;
    let a3005 = 48u64;
    if if nat_lt(a3004, a3005) {
        true
    } else {
        let a3006 = 57u64;
        let a3007 = v0;
        nat_lt(a3006, a3007)
    } {
        None::<u64>
    } else {
        Some({
            let a3008 = v0;
            let a3009 = 48u64;
            nat_sub(a3008, a3009)
        })
    }
}

pub fn f746(v0: Fn4, p3013: u64, p3014: (u64, Bytes)) -> R<(u64, Bytes)> {
    let m3012 = (p3013, p3014);
    {
        let mut m3012 = m3012;
        loop {
            let step = {
                let (v1, v2) = m3012;
                let m3010 = v1;
                if m3010 == 0 {
                    Err::<(u64, (u64, Bytes)), R<(u64, Bytes)>>(Ok(v2))
                } else {
                    let v3 = m3010 - 1;
                    Ok::<(u64, (u64, Bytes)), R<(u64, Bytes)>>((v3, {
                        let c3011: Fn4 = v0.clone();
                        c3011.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3012 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f747(v0: Bytes, v1: (u64, Bytes)) -> R<(u64, Bytes)> {
    Ok(({
        let a3016 = {
            let (h3015, _) = v1.clone();
            h3015
        };
        let a3017 = 1u64;
        nat_add(a3016, a3017)?
    }, {
        let a3020 = {
            let (_, h3018) = v1;
            h3018
        };
        let a3021 = f753(f57(v0, {
            let (h3019, _) = v1;
            h3019
        })?)?;
        append_bytes(a3020, a3021)
    }))
}

pub fn f748(v0: Bytes, v1: Bytes) -> R<Bytes> {
    f754(f755(v0, v1)?)
}

pub fn f749(v0: Bytes, v1: Bytes) -> Bytes {
    let a3028 = f756({
        let a3022 = v0.clone();
        length_bytes(a3022)
    });
    let a3029 = {
        let a3026 = v0;
        let a3027 = {
            let a3024 = f756({
                let a3023 = v1.clone();
                length_bytes(a3023)
            });
            let a3025 = v1;
            append_bytes(a3024, a3025)
        };
        append_bytes(a3026, a3027)
    };
    append_bytes(a3028, a3029)
}

pub fn f750(v0: Bytes, v1: Bytes) -> R<bool> {
    let a3032 = {
        let a3030 = v0.clone();
        length_bytes(a3030)
    };
    let a3033 = {
        let a3031 = v1.clone();
        length_bytes(a3031)
    };
    if nat_eq(a3032, a3033) {
        let a3034 = f757(v0, v1)?;
        let a3035 = 0u64;
        Ok(nat_eq(a3034, a3035))
    } else {
        Ok(false)
    }
}

pub fn f751(v0: Fn5, p3039: u64, p3040: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let m3038 = (p3039, p3040);
    {
        let mut m3038 = m3038;
        loop {
            let step = {
                let (v1, v2) = m3038;
                let m3036 = v1;
                if m3036 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>(Ok(v2))
                } else {
                    let v3 = m3036 - 1;
                    Ok::<(u64, (u64, (Bytes, u64))), R<(u64, (Bytes, u64))>>((v3, {
                        let c3037: Fn5 = v0.clone();
                        c3037.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3038 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f752(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    f758(v0, v1)
}

pub fn f753(v0: u64) -> R<Bytes> {
    let a3047 = f759({
        let a3041 = v0;
        let a3042 = 16u64;
        let a3043 = 0u64;
        nat_quot(a3041, a3042, a3043)
    })?;
    let a3048 = f759({
        let a3044 = v0;
        let a3045 = 16u64;
        let a3046 = 0u64;
        nat_rem(a3044, a3045, a3046)
    })?;
    Ok(append_bytes(a3047, a3048))
}

pub fn f754(v0: Adt152) -> R<Bytes> {
    let a3069 = f760(match v0.clone() {
        Adt152::C0(h3049, _, _, _, _, _, _, _) => {
            h3049
        }
    })?;
    let a3070 = {
        let a3067 = f760(match v0.clone() {
            Adt152::C0(_, h3050, _, _, _, _, _, _) => {
                h3050
            }
        })?;
        let a3068 = {
            let a3065 = f760(match v0.clone() {
                Adt152::C0(_, _, h3051, _, _, _, _, _) => {
                    h3051
                }
            })?;
            let a3066 = {
                let a3063 = f760(match v0.clone() {
                    Adt152::C0(_, _, _, h3052, _, _, _, _) => {
                        h3052
                    }
                })?;
                let a3064 = {
                    let a3061 = f760(match v0.clone() {
                        Adt152::C0(_, _, _, _, h3053, _, _, _) => {
                            h3053
                        }
                    })?;
                    let a3062 = {
                        let a3059 = f760(match v0.clone() {
                            Adt152::C0(_, _, _, _, _, h3054, _, _) => {
                                h3054
                            }
                        })?;
                        let a3060 = {
                            let a3057 = f760(match v0.clone() {
                                Adt152::C0(_, _, _, _, _, _, h3055, _) => {
                                    h3055
                                }
                            })?;
                            let a3058 = f760(match v0 {
                                Adt152::C0(_, _, _, _, _, _, _, h3056) => {
                                    h3056
                                }
                            })?;
                            append_bytes(a3057, a3058)
                        };
                        append_bytes(a3059, a3060)
                    };
                    append_bytes(a3061, a3062)
                };
                append_bytes(a3063, a3064)
            };
            append_bytes(a3065, a3066)
        };
        append_bytes(a3067, a3068)
    };
    Ok(append_bytes(a3069, a3070))
}

pub fn f755(v0: Bytes, v1: Bytes) -> R<Adt152> {
    let v2: Adt153 = f761(v0)?;
    let v3: Adt152 = f762(f763(f764(), f765(v2.clone(), 909522486u32))?, 64u64, v1)?;
    f763(f763(f764(), f765(v2, 1549556828u32))?, f766(v3))
}

pub fn f756(v0: u64) -> Bytes {
    let a3096 = f527({
        let a3074 = {
            let a3071 = v0;
            let a3072 = 16777216u64;
            let a3073 = 0u64;
            nat_quot(a3071, a3072, a3073)
        };
        let a3075 = 256u64;
        let a3076 = 0u64;
        nat_rem(a3074, a3075, a3076)
    });
    let a3097 = {
        let a3094 = f527({
            let a3080 = {
                let a3077 = v0;
                let a3078 = 65536u64;
                let a3079 = 0u64;
                nat_quot(a3077, a3078, a3079)
            };
            let a3081 = 256u64;
            let a3082 = 0u64;
            nat_rem(a3080, a3081, a3082)
        });
        let a3095 = {
            let a3092 = f527({
                let a3086 = {
                    let a3083 = v0;
                    let a3084 = 256u64;
                    let a3085 = 0u64;
                    nat_quot(a3083, a3084, a3085)
                };
                let a3087 = 256u64;
                let a3088 = 0u64;
                nat_rem(a3086, a3087, a3088)
            });
            let a3093 = f527({
                let a3089 = v0;
                let a3090 = 256u64;
                let a3091 = 0u64;
                nat_rem(a3089, a3090, a3091)
            });
            append_bytes(a3092, a3093)
        };
        append_bytes(a3094, a3095)
    };
    append_bytes(a3096, a3097)
}

pub fn f757(v0: Bytes, v1: Bytes) -> R<u64> {
    let (_, h3098) = f741(Fn3::F767(v0.clone(), v1), {
        let a3099 = v0;
        length_bytes(a3099)
    }, (0u64, 0u64))?;
    Ok(h3098)
}

pub fn f758(v0: Bytes, v1: (u64, (Bytes, u64))) -> R<(u64, (Bytes, u64))> {
    let v2: u64 = f768(f57(v0.clone(), {
        let (h3100, _) = v1.clone();
        h3100
    })?);
    let v3: u64 = f768(f57(v0, {
        let a3102 = {
            let (h3101, _) = v1.clone();
            h3101
        };
        let a3103 = 1u64;
        nat_add(a3102, a3103)?
    })?);
    let a3104 = v2;
    let a3105 = 16u64;
    if if nat_lt(a3104, a3105) {
        let a3106 = v3;
        let a3107 = 16u64;
        nat_lt(a3106, a3107)
    } else {
        false
    } {
        Ok(({
            let a3109 = {
                let (h3108, _) = v1.clone();
                h3108
            };
            let a3110 = 2u64;
            nat_add(a3109, a3110)?
        }, ({
            let a3117 = {
                let (h3111, _) = {
                    let (_, h3112) = v1.clone();
                    h3112
                };
                h3111
            };
            let a3118 = f527({
                let a3115 = {
                    let a3113 = v2;
                    let a3114 = 16u64;
                    nat_mul(a3113, a3114)?
                };
                let a3116 = v3;
                nat_add(a3115, a3116)?
            });
            append_bytes(a3117, a3118)
        }, {
            let (_, h3119) = {
                let (_, h3120) = v1;
                h3120
            };
            h3119
        })))
    } else {
        Ok(({
            let a3122 = {
                let (h3121, _) = v1.clone();
                h3121
            };
            let a3123 = 2u64;
            nat_add(a3122, a3123)?
        }, ({
            let (h3124, _) = {
                let (_, h3125) = v1.clone();
                h3125
            };
            h3124
        }, {
            let a3128 = {
                let (_, h3126) = {
                    let (_, h3127) = v1;
                    h3127
                };
                h3126
            };
            let a3129 = 1u64;
            nat_add(a3128, a3129)?
        })))
    }
}

pub fn f759(v0: u64) -> R<Bytes> {
    Ok(f527({
        let a3130 = v0;
        let a3131 = 10u64;
        if nat_lt(a3130, a3131) {
            let a3132 = 48u64;
            let a3133 = v0;
            nat_add(a3132, a3133)?
        } else {
            let a3134 = 87u64;
            let a3135 = v0;
            nat_add(a3134, a3135)?
        }
    }))
}

pub fn f760(v0: u32) -> R<Bytes> {
    let a3140 = f769(v0, 24u32)?;
    let a3141 = {
        let a3138 = f769(v0, 16u32)?;
        let a3139 = {
            let a3136 = f769(v0, 8u32)?;
            let a3137 = f769(v0, 0u32)?;
            append_bytes(a3136, a3137)
        };
        append_bytes(a3138, a3139)
    };
    Ok(append_bytes(a3140, a3141))
}

pub fn f761(v0: Bytes) -> R<Adt153> {
    let a3143 = f770();
    let a3144 = {
        let a3142 = v0.clone();
        length_bytes(a3142)
    };
    if nat_lt(a3143, a3144) {
        Ok(f771(f772(v0)?))
    } else {
        f773(v0)
    }
}

pub fn f762(v0: Adt152, v1: u64, v2: Bytes) -> R<Adt152> {
    let v3: u64 = {
        let a3145 = v2.clone();
        length_bytes(a3145)
    };
    let v4: u64 = {
        let a3151 = {
            let a3148 = {
                let a3146 = v3;
                let a3147 = 72u64;
                nat_add(a3146, a3147)?
            };
            let a3149 = 64u64;
            let a3150 = 0u64;
            nat_quot(a3148, a3149, a3150)
        };
        let a3152 = 64u64;
        nat_mul(a3151, a3152)?
    };
    let (_, h3153) = f774(Fn6::F775(v2, v1, v3, v4), {
        let a3154 = v4;
        let a3155 = 64u64;
        let a3156 = 0u64;
        nat_quot(a3154, a3155, a3156)
    }, (0u64, v0))?;
    Ok(h3153)
}

pub fn f763(v0: Adt152, v1: Adt153) -> R<Adt152> {
    let v2: Adt154 = f776(Fn7::F777, 64u64, Adt154::C0(match v0.clone() {
        Adt152::C0(h3157, _, _, _, _, _, _, _) => {
            h3157
        }
    }, match v0.clone() {
        Adt152::C0(_, h3158, _, _, _, _, _, _) => {
            h3158
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3159, _, _, _, _, _) => {
            h3159
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3160, _, _, _, _) => {
            h3160
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3161, _, _, _) => {
            h3161
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3162, _, _) => {
            h3162
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3163, _) => {
            h3163
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, _, h3164) => {
            h3164
        }
    }, v1, 0u64))?;
    Ok(Adt152::C0(f778(match v0.clone() {
        Adt152::C0(h3165, _, _, _, _, _, _, _) => {
            h3165
        }
    }, match v2.clone() {
        Adt154::C0(h3166, _, _, _, _, _, _, _, _, _) => {
            h3166
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, h3167, _, _, _, _, _, _) => {
            h3167
        }
    }, match v2.clone() {
        Adt154::C0(_, h3168, _, _, _, _, _, _, _, _) => {
            h3168
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, _, h3169, _, _, _, _, _) => {
            h3169
        }
    }, match v2.clone() {
        Adt154::C0(_, _, h3170, _, _, _, _, _, _, _) => {
            h3170
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, _, _, h3171, _, _, _, _) => {
            h3171
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, h3172, _, _, _, _, _, _) => {
            h3172
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, _, _, _, h3173, _, _, _) => {
            h3173
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, h3174, _, _, _, _, _) => {
            h3174
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3175, _, _) => {
            h3175
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, _, h3176, _, _, _, _) => {
            h3176
        }
    }), f778(match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3177, _) => {
            h3177
        }
    }, match v2.clone() {
        Adt154::C0(_, _, _, _, _, _, h3178, _, _, _) => {
            h3178
        }
    }), f778(match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3179) => {
            h3179
        }
    }, match v2 {
        Adt154::C0(_, _, _, _, _, _, _, h3180, _, _) => {
            h3180
        }
    })))
}

pub fn f764() -> Adt152 {
    Adt152::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f765(v0: Adt153, v1: u32) -> Adt153 {
    Adt153::C0({
        let a3182 = match v0.clone() {
            Adt153::C0(h3181, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3181
            }
        };
        let a3183 = v1;
        fixed_u32::bit_xor(a3182, a3183)
    }, {
        let a3185 = match v0.clone() {
            Adt153::C0(_, h3184, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3184
            }
        };
        let a3186 = v1;
        fixed_u32::bit_xor(a3185, a3186)
    }, {
        let a3188 = match v0.clone() {
            Adt153::C0(_, _, h3187, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3187
            }
        };
        let a3189 = v1;
        fixed_u32::bit_xor(a3188, a3189)
    }, {
        let a3191 = match v0.clone() {
            Adt153::C0(_, _, _, h3190, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3190
            }
        };
        let a3192 = v1;
        fixed_u32::bit_xor(a3191, a3192)
    }, {
        let a3194 = match v0.clone() {
            Adt153::C0(_, _, _, _, h3193, _, _, _, _, _, _, _, _, _, _, _) => {
                h3193
            }
        };
        let a3195 = v1;
        fixed_u32::bit_xor(a3194, a3195)
    }, {
        let a3197 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, h3196, _, _, _, _, _, _, _, _, _, _) => {
                h3196
            }
        };
        let a3198 = v1;
        fixed_u32::bit_xor(a3197, a3198)
    }, {
        let a3200 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, h3199, _, _, _, _, _, _, _, _, _) => {
                h3199
            }
        };
        let a3201 = v1;
        fixed_u32::bit_xor(a3200, a3201)
    }, {
        let a3203 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, h3202, _, _, _, _, _, _, _, _) => {
                h3202
            }
        };
        let a3204 = v1;
        fixed_u32::bit_xor(a3203, a3204)
    }, {
        let a3206 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, h3205, _, _, _, _, _, _, _) => {
                h3205
            }
        };
        let a3207 = v1;
        fixed_u32::bit_xor(a3206, a3207)
    }, {
        let a3209 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, h3208, _, _, _, _, _, _) => {
                h3208
            }
        };
        let a3210 = v1;
        fixed_u32::bit_xor(a3209, a3210)
    }, {
        let a3212 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, h3211, _, _, _, _, _) => {
                h3211
            }
        };
        let a3213 = v1;
        fixed_u32::bit_xor(a3212, a3213)
    }, {
        let a3215 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, h3214, _, _, _, _) => {
                h3214
            }
        };
        let a3216 = v1;
        fixed_u32::bit_xor(a3215, a3216)
    }, {
        let a3218 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3217, _, _, _) => {
                h3217
            }
        };
        let a3219 = v1;
        fixed_u32::bit_xor(a3218, a3219)
    }, {
        let a3221 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3220, _, _) => {
                h3220
            }
        };
        let a3222 = v1;
        fixed_u32::bit_xor(a3221, a3222)
    }, {
        let a3224 = match v0.clone() {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3223, _) => {
                h3223
            }
        };
        let a3225 = v1;
        fixed_u32::bit_xor(a3224, a3225)
    }, {
        let a3227 = match v0 {
            Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3226) => {
                h3226
            }
        };
        let a3228 = v1;
        fixed_u32::bit_xor(a3227, a3228)
    })
}

pub fn f766(v0: Adt152) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt152::C0(h3229, _, _, _, _, _, _, _) => {
            h3229
        }
    }, match v0.clone() {
        Adt152::C0(_, h3230, _, _, _, _, _, _) => {
            h3230
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3231, _, _, _, _, _) => {
            h3231
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3232, _, _, _, _) => {
            h3232
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3233, _, _, _) => {
            h3233
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3234, _, _) => {
            h3234
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3235, _) => {
            h3235
        }
    }, match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3236) => {
            h3236
        }
    }, 2147483648u32, f779(), f779(), f779(), f779(), f779(), f779(), 768u32)
}

pub fn f767(v0: Bytes, v1: Bytes, v2: (u64, u64)) -> R<(u64, u64)> {
    Ok(({
        let a3238 = {
            let (h3237, _) = v2;
            h3237
        };
        let a3239 = 1u64;
        nat_add(a3238, a3239)?
    }, {
        let a3291 = {
            let (_, h3240) = v2;
            h3240
        };
        let a3292 = {
            let v3: u8 = {
                let a3243 = f780(v0, {
                    let (h3241, _) = v2;
                    h3241
                });
                let a3244 = f780(v1, {
                    let (h3242, _) = v2;
                    h3242
                });
                fixed_u8::bit_xor(a3243, a3244)
            };
            let a3289 = {
                let a3265 = {
                    let a3253 = {
                        let a3247 = {
                            let a3245 = v3;
                            let a3246 = 128u8;
                            fixed_u8::bit_and(a3245, a3246)
                        };
                        let a3248 = 128u8;
                        if equal(a3247, a3248) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a3254 = {
                        let a3251 = {
                            let a3249 = v3;
                            let a3250 = 64u8;
                            fixed_u8::bit_and(a3249, a3250)
                        };
                        let a3252 = 64u8;
                        if equal(a3251, a3252) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3253, a3254)?
                };
                let a3266 = {
                    let a3263 = {
                        let a3257 = {
                            let a3255 = v3;
                            let a3256 = 32u8;
                            fixed_u8::bit_and(a3255, a3256)
                        };
                        let a3258 = 32u8;
                        if equal(a3257, a3258) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a3264 = {
                        let a3261 = {
                            let a3259 = v3;
                            let a3260 = 16u8;
                            fixed_u8::bit_and(a3259, a3260)
                        };
                        let a3262 = 16u8;
                        if equal(a3261, a3262) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3263, a3264)?
                };
                nat_add(a3265, a3266)?
            };
            let a3290 = {
                let a3287 = {
                    let a3275 = {
                        let a3269 = {
                            let a3267 = v3;
                            let a3268 = 8u8;
                            fixed_u8::bit_and(a3267, a3268)
                        };
                        let a3270 = 8u8;
                        if equal(a3269, a3270) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a3276 = {
                        let a3273 = {
                            let a3271 = v3;
                            let a3272 = 4u8;
                            fixed_u8::bit_and(a3271, a3272)
                        };
                        let a3274 = 4u8;
                        if equal(a3273, a3274) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3275, a3276)?
                };
                let a3288 = {
                    let a3285 = {
                        let a3279 = {
                            let a3277 = v3;
                            let a3278 = 2u8;
                            fixed_u8::bit_and(a3277, a3278)
                        };
                        let a3280 = 2u8;
                        if equal(a3279, a3280) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a3286 = {
                        let a3283 = {
                            let a3281 = v3;
                            let a3282 = 1u8;
                            fixed_u8::bit_and(a3281, a3282)
                        };
                        let a3284 = 1u8;
                        if equal(a3283, a3284) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a3285, a3286)?
                };
                nat_add(a3287, a3288)?
            };
            nat_add(a3289, a3290)?
        };
        nat_add(a3291, a3292)?
    }))
}

pub fn f768(v0: u64) -> u64 {
    let a3293 = v0;
    let a3294 = 48u64;
    if nat_lt(a3293, a3294) {
        16u64
    } else {
        let a3295 = v0;
        let a3296 = 58u64;
        if nat_lt(a3295, a3296) {
            let a3297 = v0;
            let a3298 = 48u64;
            nat_sub(a3297, a3298)
        } else {
            let a3299 = v0;
            let a3300 = 97u64;
            if nat_lt(a3299, a3300) {
                16u64
            } else {
                let a3301 = v0;
                let a3302 = 103u64;
                if nat_lt(a3301, a3302) {
                    let a3303 = v0;
                    let a3304 = 87u64;
                    nat_sub(a3303, a3304)
                } else {
                    16u64
                }
            }
        }
    }
}

pub fn f769(v0: u32, v1: u32) -> R<Bytes> {
    Ok(f527({
        let v2: u32 = {
            let a3305 = f781(v0, v1);
            let a3306 = 255u32;
            fixed_u32::bit_and(a3305, a3306)
        };
        let a3495 = {
            let a3399 = {
                let a3351 = {
                    let a3327 = {
                        let a3315 = {
                            let a3309 = {
                                let a3307 = v2;
                                let a3308 = 2147483648u32;
                                fixed_u32::bit_and(a3307, a3308)
                            };
                            let a3310 = 2147483648u32;
                            if equal(a3309, a3310) {
                                2147483648u64
                            } else {
                                0u64
                            }
                        };
                        let a3316 = {
                            let a3313 = {
                                let a3311 = v2;
                                let a3312 = 1073741824u32;
                                fixed_u32::bit_and(a3311, a3312)
                            };
                            let a3314 = 1073741824u32;
                            if equal(a3313, a3314) {
                                1073741824u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3315, a3316)?
                    };
                    let a3328 = {
                        let a3325 = {
                            let a3319 = {
                                let a3317 = v2;
                                let a3318 = 536870912u32;
                                fixed_u32::bit_and(a3317, a3318)
                            };
                            let a3320 = 536870912u32;
                            if equal(a3319, a3320) {
                                536870912u64
                            } else {
                                0u64
                            }
                        };
                        let a3326 = {
                            let a3323 = {
                                let a3321 = v2;
                                let a3322 = 268435456u32;
                                fixed_u32::bit_and(a3321, a3322)
                            };
                            let a3324 = 268435456u32;
                            if equal(a3323, a3324) {
                                268435456u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3325, a3326)?
                    };
                    nat_add(a3327, a3328)?
                };
                let a3352 = {
                    let a3349 = {
                        let a3337 = {
                            let a3331 = {
                                let a3329 = v2;
                                let a3330 = 134217728u32;
                                fixed_u32::bit_and(a3329, a3330)
                            };
                            let a3332 = 134217728u32;
                            if equal(a3331, a3332) {
                                134217728u64
                            } else {
                                0u64
                            }
                        };
                        let a3338 = {
                            let a3335 = {
                                let a3333 = v2;
                                let a3334 = 67108864u32;
                                fixed_u32::bit_and(a3333, a3334)
                            };
                            let a3336 = 67108864u32;
                            if equal(a3335, a3336) {
                                67108864u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3337, a3338)?
                    };
                    let a3350 = {
                        let a3347 = {
                            let a3341 = {
                                let a3339 = v2;
                                let a3340 = 33554432u32;
                                fixed_u32::bit_and(a3339, a3340)
                            };
                            let a3342 = 33554432u32;
                            if equal(a3341, a3342) {
                                33554432u64
                            } else {
                                0u64
                            }
                        };
                        let a3348 = {
                            let a3345 = {
                                let a3343 = v2;
                                let a3344 = 16777216u32;
                                fixed_u32::bit_and(a3343, a3344)
                            };
                            let a3346 = 16777216u32;
                            if equal(a3345, a3346) {
                                16777216u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3347, a3348)?
                    };
                    nat_add(a3349, a3350)?
                };
                nat_add(a3351, a3352)?
            };
            let a3400 = {
                let a3397 = {
                    let a3373 = {
                        let a3361 = {
                            let a3355 = {
                                let a3353 = v2;
                                let a3354 = 8388608u32;
                                fixed_u32::bit_and(a3353, a3354)
                            };
                            let a3356 = 8388608u32;
                            if equal(a3355, a3356) {
                                8388608u64
                            } else {
                                0u64
                            }
                        };
                        let a3362 = {
                            let a3359 = {
                                let a3357 = v2;
                                let a3358 = 4194304u32;
                                fixed_u32::bit_and(a3357, a3358)
                            };
                            let a3360 = 4194304u32;
                            if equal(a3359, a3360) {
                                4194304u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3361, a3362)?
                    };
                    let a3374 = {
                        let a3371 = {
                            let a3365 = {
                                let a3363 = v2;
                                let a3364 = 2097152u32;
                                fixed_u32::bit_and(a3363, a3364)
                            };
                            let a3366 = 2097152u32;
                            if equal(a3365, a3366) {
                                2097152u64
                            } else {
                                0u64
                            }
                        };
                        let a3372 = {
                            let a3369 = {
                                let a3367 = v2;
                                let a3368 = 1048576u32;
                                fixed_u32::bit_and(a3367, a3368)
                            };
                            let a3370 = 1048576u32;
                            if equal(a3369, a3370) {
                                1048576u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3371, a3372)?
                    };
                    nat_add(a3373, a3374)?
                };
                let a3398 = {
                    let a3395 = {
                        let a3383 = {
                            let a3377 = {
                                let a3375 = v2;
                                let a3376 = 524288u32;
                                fixed_u32::bit_and(a3375, a3376)
                            };
                            let a3378 = 524288u32;
                            if equal(a3377, a3378) {
                                524288u64
                            } else {
                                0u64
                            }
                        };
                        let a3384 = {
                            let a3381 = {
                                let a3379 = v2;
                                let a3380 = 262144u32;
                                fixed_u32::bit_and(a3379, a3380)
                            };
                            let a3382 = 262144u32;
                            if equal(a3381, a3382) {
                                262144u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3383, a3384)?
                    };
                    let a3396 = {
                        let a3393 = {
                            let a3387 = {
                                let a3385 = v2;
                                let a3386 = 131072u32;
                                fixed_u32::bit_and(a3385, a3386)
                            };
                            let a3388 = 131072u32;
                            if equal(a3387, a3388) {
                                131072u64
                            } else {
                                0u64
                            }
                        };
                        let a3394 = {
                            let a3391 = {
                                let a3389 = v2;
                                let a3390 = 65536u32;
                                fixed_u32::bit_and(a3389, a3390)
                            };
                            let a3392 = 65536u32;
                            if equal(a3391, a3392) {
                                65536u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3393, a3394)?
                    };
                    nat_add(a3395, a3396)?
                };
                nat_add(a3397, a3398)?
            };
            nat_add(a3399, a3400)?
        };
        let a3496 = {
            let a3493 = {
                let a3445 = {
                    let a3421 = {
                        let a3409 = {
                            let a3403 = {
                                let a3401 = v2;
                                let a3402 = 32768u32;
                                fixed_u32::bit_and(a3401, a3402)
                            };
                            let a3404 = 32768u32;
                            if equal(a3403, a3404) {
                                32768u64
                            } else {
                                0u64
                            }
                        };
                        let a3410 = {
                            let a3407 = {
                                let a3405 = v2;
                                let a3406 = 16384u32;
                                fixed_u32::bit_and(a3405, a3406)
                            };
                            let a3408 = 16384u32;
                            if equal(a3407, a3408) {
                                16384u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3409, a3410)?
                    };
                    let a3422 = {
                        let a3419 = {
                            let a3413 = {
                                let a3411 = v2;
                                let a3412 = 8192u32;
                                fixed_u32::bit_and(a3411, a3412)
                            };
                            let a3414 = 8192u32;
                            if equal(a3413, a3414) {
                                8192u64
                            } else {
                                0u64
                            }
                        };
                        let a3420 = {
                            let a3417 = {
                                let a3415 = v2;
                                let a3416 = 4096u32;
                                fixed_u32::bit_and(a3415, a3416)
                            };
                            let a3418 = 4096u32;
                            if equal(a3417, a3418) {
                                4096u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3419, a3420)?
                    };
                    nat_add(a3421, a3422)?
                };
                let a3446 = {
                    let a3443 = {
                        let a3431 = {
                            let a3425 = {
                                let a3423 = v2;
                                let a3424 = 2048u32;
                                fixed_u32::bit_and(a3423, a3424)
                            };
                            let a3426 = 2048u32;
                            if equal(a3425, a3426) {
                                2048u64
                            } else {
                                0u64
                            }
                        };
                        let a3432 = {
                            let a3429 = {
                                let a3427 = v2;
                                let a3428 = 1024u32;
                                fixed_u32::bit_and(a3427, a3428)
                            };
                            let a3430 = 1024u32;
                            if equal(a3429, a3430) {
                                1024u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3431, a3432)?
                    };
                    let a3444 = {
                        let a3441 = {
                            let a3435 = {
                                let a3433 = v2;
                                let a3434 = 512u32;
                                fixed_u32::bit_and(a3433, a3434)
                            };
                            let a3436 = 512u32;
                            if equal(a3435, a3436) {
                                512u64
                            } else {
                                0u64
                            }
                        };
                        let a3442 = {
                            let a3439 = {
                                let a3437 = v2;
                                let a3438 = 256u32;
                                fixed_u32::bit_and(a3437, a3438)
                            };
                            let a3440 = 256u32;
                            if equal(a3439, a3440) {
                                256u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3441, a3442)?
                    };
                    nat_add(a3443, a3444)?
                };
                nat_add(a3445, a3446)?
            };
            let a3494 = {
                let a3491 = {
                    let a3467 = {
                        let a3455 = {
                            let a3449 = {
                                let a3447 = v2;
                                let a3448 = 128u32;
                                fixed_u32::bit_and(a3447, a3448)
                            };
                            let a3450 = 128u32;
                            if equal(a3449, a3450) {
                                128u64
                            } else {
                                0u64
                            }
                        };
                        let a3456 = {
                            let a3453 = {
                                let a3451 = v2;
                                let a3452 = 64u32;
                                fixed_u32::bit_and(a3451, a3452)
                            };
                            let a3454 = 64u32;
                            if equal(a3453, a3454) {
                                64u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3455, a3456)?
                    };
                    let a3468 = {
                        let a3465 = {
                            let a3459 = {
                                let a3457 = v2;
                                let a3458 = 32u32;
                                fixed_u32::bit_and(a3457, a3458)
                            };
                            let a3460 = 32u32;
                            if equal(a3459, a3460) {
                                32u64
                            } else {
                                0u64
                            }
                        };
                        let a3466 = {
                            let a3463 = {
                                let a3461 = v2;
                                let a3462 = 16u32;
                                fixed_u32::bit_and(a3461, a3462)
                            };
                            let a3464 = 16u32;
                            if equal(a3463, a3464) {
                                16u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3465, a3466)?
                    };
                    nat_add(a3467, a3468)?
                };
                let a3492 = {
                    let a3489 = {
                        let a3477 = {
                            let a3471 = {
                                let a3469 = v2;
                                let a3470 = 8u32;
                                fixed_u32::bit_and(a3469, a3470)
                            };
                            let a3472 = 8u32;
                            if equal(a3471, a3472) {
                                8u64
                            } else {
                                0u64
                            }
                        };
                        let a3478 = {
                            let a3475 = {
                                let a3473 = v2;
                                let a3474 = 4u32;
                                fixed_u32::bit_and(a3473, a3474)
                            };
                            let a3476 = 4u32;
                            if equal(a3475, a3476) {
                                4u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3477, a3478)?
                    };
                    let a3490 = {
                        let a3487 = {
                            let a3481 = {
                                let a3479 = v2;
                                let a3480 = 2u32;
                                fixed_u32::bit_and(a3479, a3480)
                            };
                            let a3482 = 2u32;
                            if equal(a3481, a3482) {
                                2u64
                            } else {
                                0u64
                            }
                        };
                        let a3488 = {
                            let a3485 = {
                                let a3483 = v2;
                                let a3484 = 1u32;
                                fixed_u32::bit_and(a3483, a3484)
                            };
                            let a3486 = 1u32;
                            if equal(a3485, a3486) {
                                1u64
                            } else {
                                0u64
                            }
                        };
                        nat_add(a3487, a3488)?
                    };
                    nat_add(a3489, a3490)?
                };
                nat_add(a3491, a3492)?
            };
            nat_add(a3493, a3494)?
        };
        nat_add(a3495, a3496)?
    }))
}

pub fn f770() -> u64 {
    64u64
}

pub fn f771(v0: Adt152) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt152::C0(h3497, _, _, _, _, _, _, _) => {
            h3497
        }
    }, match v0.clone() {
        Adt152::C0(_, h3498, _, _, _, _, _, _) => {
            h3498
        }
    }, match v0.clone() {
        Adt152::C0(_, _, h3499, _, _, _, _, _) => {
            h3499
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, h3500, _, _, _, _) => {
            h3500
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, h3501, _, _, _) => {
            h3501
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, h3502, _, _) => {
            h3502
        }
    }, match v0.clone() {
        Adt152::C0(_, _, _, _, _, _, h3503, _) => {
            h3503
        }
    }, match v0 {
        Adt152::C0(_, _, _, _, _, _, _, h3504) => {
            h3504
        }
    }, f779(), f779(), f779(), f779(), f779(), f779(), f779(), f779())
}

pub fn f772(v0: Bytes) -> R<Adt152> {
    let v1: u64 = {
        let a3505 = v0.clone();
        length_bytes(a3505)
    };
    let v2: u64 = {
        let a3511 = {
            let a3508 = {
                let a3506 = v1;
                let a3507 = 72u64;
                nat_add(a3506, a3507)?
            };
            let a3509 = 64u64;
            let a3510 = 0u64;
            nat_quot(a3508, a3509, a3510)
        };
        let a3512 = 64u64;
        nat_mul(a3511, a3512)?
    };
    let (_, h3513) = f774(Fn6::F782(v0, v1, v2), {
        let a3514 = v2;
        let a3515 = 64u64;
        let a3516 = 0u64;
        nat_quot(a3514, a3515, a3516)
    }, (0u64, f764()))?;
    Ok(h3513)
}

pub fn f773(v0: Bytes) -> R<Adt153> {
    Ok(Adt153::C0(f783(v0.clone(), 0u64)?, f783(v0.clone(), 4u64)?, f783(v0.clone(), 8u64)?, f783(v0.clone(), 12u64)?, f783(v0.clone(), 16u64)?, f783(v0.clone(), 20u64)?, f783(v0.clone(), 24u64)?, f783(v0.clone(), 28u64)?, f783(v0.clone(), 32u64)?, f783(v0.clone(), 36u64)?, f783(v0.clone(), 40u64)?, f783(v0.clone(), 44u64)?, f783(v0.clone(), 48u64)?, f783(v0.clone(), 52u64)?, f783(v0.clone(), 56u64)?, f783(v0, 60u64)?))
}

pub fn f774(v0: Fn6, p3520: u64, p3521: (u64, Adt152)) -> R<(u64, Adt152)> {
    let m3519 = (p3520, p3521);
    {
        let mut m3519 = m3519;
        loop {
            let step = {
                let (v1, v2) = m3519;
                let m3517 = v1;
                if m3517 == 0 {
                    Err::<(u64, (u64, Adt152)), R<(u64, Adt152)>>(Ok(v2))
                } else {
                    let v3 = m3517 - 1;
                    Ok::<(u64, (u64, Adt152)), R<(u64, Adt152)>>((v3, {
                        let c3518: Fn6 = v0.clone();
                        c3518.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3519 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f775(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: (u64, Adt152)) -> R<(u64, Adt152)> {
    Ok(({
        let a3523 = {
            let (h3522, _) = v4.clone();
            h3522
        };
        let a3524 = 64u64;
        nat_add(a3523, a3524)?
    }, f763({
        let (_, h3525) = v4;
        h3525
    }, f784(v0, v2, v3, {
        let a3528 = {
            let a3526 = v1;
            let a3527 = v2;
            nat_add(a3526, a3527)?
        };
        let a3529 = 8u64;
        nat_mul(a3528, a3529)?
    }, {
        let (h3530, _) = v4;
        h3530
    })?)?))
}

pub fn f776(v0: Fn7, p3534: u64, p3535: Adt154) -> R<Adt154> {
    let m3533 = (p3534, p3535);
    {
        let mut m3533 = m3533;
        loop {
            let step = {
                let (v1, v2) = m3533;
                let m3531 = v1;
                if m3531 == 0 {
                    Err::<(u64, Adt154), R<Adt154>>(Ok(v2))
                } else {
                    let v3 = m3531 - 1;
                    Ok::<(u64, Adt154), R<Adt154>>((v3, {
                        let c3532: Fn7 = v0.clone();
                        c3532.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m3533 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f777(v0: Adt154) -> R<Adt154> {
    f785(v0)
}

pub fn f778(v0: u32, v1: u32) -> u32 {
    let m3538 = {
        let a3536 = f786(v0);
        let a3537 = f786(v1);
        fixed_u64::checked_add(a3536, a3537)
    };
    match m3538 {
        None => {
            0u32
        }
        Some(v2) => {
            f787(v2)
        }
    }
}

pub fn f779() -> u32 {
    0u32
}

pub fn f780(v0: Bytes, v1: u64) -> u8 {
    let m3541 = {
        let a3539 = v0;
        let a3540 = v1;
        index_bytes(a3539, a3540)
    };
    match m3541 {
        None => {
            0u8
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f781(v0: u32, v1: u32) -> u32 {
    let m3544 = {
        let a3542 = v0;
        let a3543 = v1;
        fixed_u32::shift_right(a3542, a3543)
    };
    match m3544 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f782(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt152)) -> R<(u64, Adt152)> {
    Ok(({
        let a3546 = {
            let (h3545, _) = v3.clone();
            h3545
        };
        let a3547 = 64u64;
        nat_add(a3546, a3547)?
    }, f763({
        let (_, h3548) = v3;
        h3548
    }, f788(v0, v1, v2, {
        let (h3549, _) = v3;
        h3549
    })?)?))
}

pub fn f783(v0: Bytes, v1: u64) -> R<u32> {
    Ok(f789(f790(v0.clone(), v1), f790(v0.clone(), {
        let a3550 = v1;
        let a3551 = 1u64;
        nat_add(a3550, a3551)?
    }), f790(v0.clone(), {
        let a3552 = v1;
        let a3553 = 2u64;
        nat_add(a3552, a3553)?
    }), f790(v0, {
        let a3554 = v1;
        let a3555 = 3u64;
        nat_add(a3554, a3555)?
    })))
}

pub fn f784(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt153> {
    Ok(Adt153::C0(f791(v0.clone(), v1, v2, v3, v4)?, f791(v0.clone(), v1, v2, v3, {
        let a3556 = v4;
        let a3557 = 4u64;
        nat_add(a3556, a3557)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3558 = v4;
        let a3559 = 8u64;
        nat_add(a3558, a3559)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3560 = v4;
        let a3561 = 12u64;
        nat_add(a3560, a3561)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3562 = v4;
        let a3563 = 16u64;
        nat_add(a3562, a3563)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3564 = v4;
        let a3565 = 20u64;
        nat_add(a3564, a3565)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3566 = v4;
        let a3567 = 24u64;
        nat_add(a3566, a3567)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3568 = v4;
        let a3569 = 28u64;
        nat_add(a3568, a3569)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3570 = v4;
        let a3571 = 32u64;
        nat_add(a3570, a3571)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3572 = v4;
        let a3573 = 36u64;
        nat_add(a3572, a3573)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3574 = v4;
        let a3575 = 40u64;
        nat_add(a3574, a3575)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3576 = v4;
        let a3577 = 44u64;
        nat_add(a3576, a3577)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3578 = v4;
        let a3579 = 48u64;
        nat_add(a3578, a3579)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3580 = v4;
        let a3581 = 52u64;
        nat_add(a3580, a3581)?
    })?, f791(v0.clone(), v1, v2, v3, {
        let a3582 = v4;
        let a3583 = 56u64;
        nat_add(a3582, a3583)?
    })?, f791(v0, v1, v2, v3, {
        let a3584 = v4;
        let a3585 = 60u64;
        nat_add(a3584, a3585)?
    })?))
}

pub fn f785(v0: Adt154) -> R<Adt154> {
    let v1: u32 = f792(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, _, _, h3586) => {
            h3586
        }
    });
    let v2: u32 = f778(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, h3587, _, _) => {
            h3587
        }
    }, f778(f793(match v0.clone() {
        Adt154::C0(_, _, _, _, h3588, _, _, _, _, _) => {
            h3588
        }
    }), f778(f794(match v0.clone() {
        Adt154::C0(_, _, _, _, h3589, _, _, _, _, _) => {
            h3589
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, h3590, _, _, _, _) => {
            h3590
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, h3591, _, _, _) => {
            h3591
        }
    }), f778(v1, {
        let m3594 = match v0.clone() {
            Adt154::C0(_, _, _, _, _, _, _, _, h3593, _) => {
                h3593
            }
        };
        match m3594 {
            Adt153::C0(h3592, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3592
            }
        }
    }))));
    let v3: u32 = f778(f795(match v0.clone() {
        Adt154::C0(h3595, _, _, _, _, _, _, _, _, _) => {
            h3595
        }
    }), f796(match v0.clone() {
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
    }));
    Ok(Adt154::C0(f778(v2, v3), match v0.clone() {
        Adt154::C0(h3599, _, _, _, _, _, _, _, _, _) => {
            h3599
        }
    }, match v0.clone() {
        Adt154::C0(_, h3600, _, _, _, _, _, _, _, _) => {
            h3600
        }
    }, match v0.clone() {
        Adt154::C0(_, _, h3601, _, _, _, _, _, _, _) => {
            h3601
        }
    }, f778(match v0.clone() {
        Adt154::C0(_, _, _, h3602, _, _, _, _, _, _) => {
            h3602
        }
    }, v2), match v0.clone() {
        Adt154::C0(_, _, _, _, h3603, _, _, _, _, _) => {
            h3603
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, h3604, _, _, _, _) => {
            h3604
        }
    }, match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, h3605, _, _, _) => {
            h3605
        }
    }, f797(match v0.clone() {
        Adt154::C0(_, _, _, _, _, _, _, _, h3606, _) => {
            h3606
        }
    }), {
        let a3608 = match v0 {
            Adt154::C0(_, _, _, _, _, _, _, _, _, h3607) => {
                h3607
            }
        };
        let a3609 = 1u64;
        nat_add(a3608, a3609)?
    }))
}

pub fn f786(v0: u32) -> u64 {
    let m3611 = {
        let a3610 = v0;
        fixed_u64::convert(i128::from(a3610))
    };
    match m3611 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f787(v0: u64) -> u32 {
    let m3615 = {
        let a3614 = {
            let a3612 = v0;
            let a3613 = 4294967295u64;
            fixed_u64::bit_and(a3612, a3613)
        };
        fixed_u32::convert(i128::from(a3614))
    };
    match m3615 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f788(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt153> {
    Ok(Adt153::C0(f798(v0.clone(), v1, v2, v3)?, f798(v0.clone(), v1, v2, {
        let a3616 = v3;
        let a3617 = 4u64;
        nat_add(a3616, a3617)?
    })?, f798(v0.clone(), v1, v2, {
        let a3618 = v3;
        let a3619 = 8u64;
        nat_add(a3618, a3619)?
    })?, f798(v0.clone(), v1, v2, {
        let a3620 = v3;
        let a3621 = 12u64;
        nat_add(a3620, a3621)?
    })?, f798(v0.clone(), v1, v2, {
        let a3622 = v3;
        let a3623 = 16u64;
        nat_add(a3622, a3623)?
    })?, f798(v0.clone(), v1, v2, {
        let a3624 = v3;
        let a3625 = 20u64;
        nat_add(a3624, a3625)?
    })?, f798(v0.clone(), v1, v2, {
        let a3626 = v3;
        let a3627 = 24u64;
        nat_add(a3626, a3627)?
    })?, f798(v0.clone(), v1, v2, {
        let a3628 = v3;
        let a3629 = 28u64;
        nat_add(a3628, a3629)?
    })?, f798(v0.clone(), v1, v2, {
        let a3630 = v3;
        let a3631 = 32u64;
        nat_add(a3630, a3631)?
    })?, f798(v0.clone(), v1, v2, {
        let a3632 = v3;
        let a3633 = 36u64;
        nat_add(a3632, a3633)?
    })?, f798(v0.clone(), v1, v2, {
        let a3634 = v3;
        let a3635 = 40u64;
        nat_add(a3634, a3635)?
    })?, f798(v0.clone(), v1, v2, {
        let a3636 = v3;
        let a3637 = 44u64;
        nat_add(a3636, a3637)?
    })?, f798(v0.clone(), v1, v2, {
        let a3638 = v3;
        let a3639 = 48u64;
        nat_add(a3638, a3639)?
    })?, f798(v0.clone(), v1, v2, {
        let a3640 = v3;
        let a3641 = 52u64;
        nat_add(a3640, a3641)?
    })?, f798(v0.clone(), v1, v2, {
        let a3642 = v3;
        let a3643 = 56u64;
        nat_add(a3642, a3643)?
    })?, f798(v0, v1, v2, {
        let a3644 = v3;
        let a3645 = 60u64;
        nat_add(a3644, a3645)?
    })?))
}

pub fn f789(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3650 = f799(v0, 24u32);
    let a3651 = {
        let a3648 = f799(v1, 16u32);
        let a3649 = {
            let a3646 = f799(v2, 8u32);
            let a3647 = v3;
            fixed_u32::bit_or(a3646, a3647)
        };
        fixed_u32::bit_or(a3648, a3649)
    };
    fixed_u32::bit_or(a3650, a3651)
}

pub fn f790(v0: Bytes, v1: u64) -> u32 {
    let m3654 = {
        let a3652 = v0;
        let a3653 = v1;
        index_bytes(a3652, a3653)
    };
    match m3654 {
        None => {
            0u32
        }
        Some(v2) => {
            let m3656 = {
                let a3655 = v2;
                fixed_u32::convert(i128::from(a3655))
            };
            match m3656 {
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

pub fn f791(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u32> {
    let a3659 = {
        let a3657 = v4;
        let a3658 = 8u64;
        nat_add(a3657, a3658)?
    };
    let a3660 = v2;
    if nat_eq(a3659, a3660) {
        Ok(f800(v3, true))
    } else {
        let a3663 = {
            let a3661 = v4;
            let a3662 = 4u64;
            nat_add(a3661, a3662)?
        };
        let a3664 = v2;
        if nat_eq(a3663, a3664) {
            Ok(f800(v3, false))
        } else {
            Ok(f789(f801(v0.clone(), v1, v4), f801(v0.clone(), v1, {
                let a3665 = v4;
                let a3666 = 1u64;
                nat_add(a3665, a3666)?
            }), f801(v0.clone(), v1, {
                let a3667 = v4;
                let a3668 = 2u64;
                nat_add(a3667, a3668)?
            }), f801(v0, v1, {
                let a3669 = v4;
                let a3670 = 3u64;
                nat_add(a3669, a3670)?
            })))
        }
    }
}

pub fn f792(v0: u64) -> u32 {
    let a3671 = v0;
    let a3672 = 32u64;
    if nat_lt(a3671, a3672) {
        let a3673 = v0;
        let a3674 = 16u64;
        if nat_lt(a3673, a3674) {
            let a3675 = v0;
            let a3676 = 8u64;
            if nat_lt(a3675, a3676) {
                let a3677 = v0;
                let a3678 = 4u64;
                if nat_lt(a3677, a3678) {
                    let a3679 = v0;
                    let a3680 = 2u64;
                    if nat_lt(a3679, a3680) {
                        let a3681 = v0;
                        let a3682 = 1u64;
                        if nat_lt(a3681, a3682) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a3683 = v0;
                        let a3684 = 3u64;
                        if nat_lt(a3683, a3684) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a3685 = v0;
                    let a3686 = 6u64;
                    if nat_lt(a3685, a3686) {
                        let a3687 = v0;
                        let a3688 = 5u64;
                        if nat_lt(a3687, a3688) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a3689 = v0;
                        let a3690 = 7u64;
                        if nat_lt(a3689, a3690) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a3691 = v0;
                let a3692 = 12u64;
                if nat_lt(a3691, a3692) {
                    let a3693 = v0;
                    let a3694 = 10u64;
                    if nat_lt(a3693, a3694) {
                        let a3695 = v0;
                        let a3696 = 9u64;
                        if nat_lt(a3695, a3696) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a3697 = v0;
                        let a3698 = 11u64;
                        if nat_lt(a3697, a3698) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a3699 = v0;
                    let a3700 = 14u64;
                    if nat_lt(a3699, a3700) {
                        let a3701 = v0;
                        let a3702 = 13u64;
                        if nat_lt(a3701, a3702) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a3703 = v0;
                        let a3704 = 15u64;
                        if nat_lt(a3703, a3704) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a3705 = v0;
            let a3706 = 24u64;
            if nat_lt(a3705, a3706) {
                let a3707 = v0;
                let a3708 = 20u64;
                if nat_lt(a3707, a3708) {
                    let a3709 = v0;
                    let a3710 = 18u64;
                    if nat_lt(a3709, a3710) {
                        let a3711 = v0;
                        let a3712 = 17u64;
                        if nat_lt(a3711, a3712) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a3713 = v0;
                        let a3714 = 19u64;
                        if nat_lt(a3713, a3714) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a3715 = v0;
                    let a3716 = 22u64;
                    if nat_lt(a3715, a3716) {
                        let a3717 = v0;
                        let a3718 = 21u64;
                        if nat_lt(a3717, a3718) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a3719 = v0;
                        let a3720 = 23u64;
                        if nat_lt(a3719, a3720) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a3721 = v0;
                let a3722 = 28u64;
                if nat_lt(a3721, a3722) {
                    let a3723 = v0;
                    let a3724 = 26u64;
                    if nat_lt(a3723, a3724) {
                        let a3725 = v0;
                        let a3726 = 25u64;
                        if nat_lt(a3725, a3726) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a3727 = v0;
                        let a3728 = 27u64;
                        if nat_lt(a3727, a3728) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a3729 = v0;
                    let a3730 = 30u64;
                    if nat_lt(a3729, a3730) {
                        let a3731 = v0;
                        let a3732 = 29u64;
                        if nat_lt(a3731, a3732) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a3733 = v0;
                        let a3734 = 31u64;
                        if nat_lt(a3733, a3734) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a3735 = v0;
        let a3736 = 48u64;
        if nat_lt(a3735, a3736) {
            let a3737 = v0;
            let a3738 = 40u64;
            if nat_lt(a3737, a3738) {
                let a3739 = v0;
                let a3740 = 36u64;
                if nat_lt(a3739, a3740) {
                    let a3741 = v0;
                    let a3742 = 34u64;
                    if nat_lt(a3741, a3742) {
                        let a3743 = v0;
                        let a3744 = 33u64;
                        if nat_lt(a3743, a3744) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a3745 = v0;
                        let a3746 = 35u64;
                        if nat_lt(a3745, a3746) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a3747 = v0;
                    let a3748 = 38u64;
                    if nat_lt(a3747, a3748) {
                        let a3749 = v0;
                        let a3750 = 37u64;
                        if nat_lt(a3749, a3750) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a3751 = v0;
                        let a3752 = 39u64;
                        if nat_lt(a3751, a3752) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a3753 = v0;
                let a3754 = 44u64;
                if nat_lt(a3753, a3754) {
                    let a3755 = v0;
                    let a3756 = 42u64;
                    if nat_lt(a3755, a3756) {
                        let a3757 = v0;
                        let a3758 = 41u64;
                        if nat_lt(a3757, a3758) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a3759 = v0;
                        let a3760 = 43u64;
                        if nat_lt(a3759, a3760) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a3761 = v0;
                    let a3762 = 46u64;
                    if nat_lt(a3761, a3762) {
                        let a3763 = v0;
                        let a3764 = 45u64;
                        if nat_lt(a3763, a3764) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a3765 = v0;
                        let a3766 = 47u64;
                        if nat_lt(a3765, a3766) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a3767 = v0;
            let a3768 = 56u64;
            if nat_lt(a3767, a3768) {
                let a3769 = v0;
                let a3770 = 52u64;
                if nat_lt(a3769, a3770) {
                    let a3771 = v0;
                    let a3772 = 50u64;
                    if nat_lt(a3771, a3772) {
                        let a3773 = v0;
                        let a3774 = 49u64;
                        if nat_lt(a3773, a3774) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a3775 = v0;
                        let a3776 = 51u64;
                        if nat_lt(a3775, a3776) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a3777 = v0;
                    let a3778 = 54u64;
                    if nat_lt(a3777, a3778) {
                        let a3779 = v0;
                        let a3780 = 53u64;
                        if nat_lt(a3779, a3780) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a3781 = v0;
                        let a3782 = 55u64;
                        if nat_lt(a3781, a3782) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a3783 = v0;
                let a3784 = 60u64;
                if nat_lt(a3783, a3784) {
                    let a3785 = v0;
                    let a3786 = 58u64;
                    if nat_lt(a3785, a3786) {
                        let a3787 = v0;
                        let a3788 = 57u64;
                        if nat_lt(a3787, a3788) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a3789 = v0;
                        let a3790 = 59u64;
                        if nat_lt(a3789, a3790) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a3791 = v0;
                    let a3792 = 62u64;
                    if nat_lt(a3791, a3792) {
                        let a3793 = v0;
                        let a3794 = 61u64;
                        if nat_lt(a3793, a3794) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a3795 = v0;
                        let a3796 = 63u64;
                        if nat_lt(a3795, a3796) {
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

pub fn f793(v0: u32) -> u32 {
    let a3799 = {
        let a3797 = f802(v0, 6u32, 26u32);
        let a3798 = f802(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a3797, a3798)
    };
    let a3800 = f802(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a3799, a3800)
}

pub fn f794(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3806 = {
        let a3801 = v0;
        let a3802 = v1;
        fixed_u32::bit_and(a3801, a3802)
    };
    let a3807 = {
        let a3804 = {
            let a3803 = v0;
            fixed_u32::bit_not(a3803)
        };
        let a3805 = v2;
        fixed_u32::bit_and(a3804, a3805)
    };
    fixed_u32::bit_xor(a3806, a3807)
}

pub fn f795(v0: u32) -> u32 {
    let a3810 = {
        let a3808 = f802(v0, 2u32, 30u32);
        let a3809 = f802(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a3808, a3809)
    };
    let a3811 = f802(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a3810, a3811)
}

pub fn f796(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3820 = {
        let a3816 = {
            let a3812 = v0;
            let a3813 = v1;
            fixed_u32::bit_and(a3812, a3813)
        };
        let a3817 = {
            let a3814 = v0;
            let a3815 = v2;
            fixed_u32::bit_and(a3814, a3815)
        };
        fixed_u32::bit_xor(a3816, a3817)
    };
    let a3821 = {
        let a3818 = v1;
        let a3819 = v2;
        fixed_u32::bit_and(a3818, a3819)
    };
    fixed_u32::bit_xor(a3820, a3821)
}

pub fn f797(v0: Adt153) -> Adt153 {
    Adt153::C0(match v0.clone() {
        Adt153::C0(_, h3822, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3822
        }
    }, match v0.clone() {
        Adt153::C0(_, _, h3823, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3823
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, h3824, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3824
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, h3825, _, _, _, _, _, _, _, _, _, _, _) => {
            h3825
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, h3826, _, _, _, _, _, _, _, _, _, _) => {
            h3826
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, h3827, _, _, _, _, _, _, _, _, _) => {
            h3827
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, h3828, _, _, _, _, _, _, _, _) => {
            h3828
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, h3829, _, _, _, _, _, _, _) => {
            h3829
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, h3830, _, _, _, _, _, _) => {
            h3830
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, h3831, _, _, _, _, _) => {
            h3831
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, h3832, _, _, _, _) => {
            h3832
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, h3833, _, _, _) => {
            h3833
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h3834, _, _) => {
            h3834
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3835, _) => {
            h3835
        }
    }, match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h3836) => {
            h3836
        }
    }, f778(f803(match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h3837, _) => {
            h3837
        }
    }), f778(match v0.clone() {
        Adt153::C0(_, _, _, _, _, _, _, _, _, h3838, _, _, _, _, _, _) => {
            h3838
        }
    }, f778(f804(match v0.clone() {
        Adt153::C0(_, h3839, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3839
        }
    }), match v0 {
        Adt153::C0(h3840, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h3840
        }
    }))))
}

pub fn f798(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a3843 = {
        let a3841 = v3;
        let a3842 = 8u64;
        nat_add(a3841, a3842)?
    };
    let a3844 = v2;
    if nat_eq(a3843, a3844) {
        Ok(f805({
            let a3847 = {
                let a3845 = v1;
                let a3846 = 8u64;
                nat_mul(a3845, a3846)?
            };
            let a3848 = 4294967296u64;
            let a3849 = 0u64;
            nat_quot(a3847, a3848, a3849)
        }))
    } else {
        let a3852 = {
            let a3850 = v3;
            let a3851 = 4u64;
            nat_add(a3850, a3851)?
        };
        let a3853 = v2;
        if nat_eq(a3852, a3853) {
            Ok(f805({
                let a3856 = {
                    let a3854 = v1;
                    let a3855 = 8u64;
                    nat_mul(a3854, a3855)?
                };
                let a3857 = 4294967296u64;
                let a3858 = 0u64;
                nat_rem(a3856, a3857, a3858)
            }))
        } else {
            Ok(f789(f801(v0.clone(), v1, v3), f801(v0.clone(), v1, {
                let a3859 = v3;
                let a3860 = 1u64;
                nat_add(a3859, a3860)?
            }), f801(v0.clone(), v1, {
                let a3861 = v3;
                let a3862 = 2u64;
                nat_add(a3861, a3862)?
            }), f801(v0, v1, {
                let a3863 = v3;
                let a3864 = 3u64;
                nat_add(a3863, a3864)?
            })))
        }
    }
}

pub fn f799(v0: u32, v1: u32) -> u32 {
    let m3867 = {
        let a3865 = v0;
        let a3866 = v1;
        fixed_u32::shift_left(a3865, a3866)
    };
    match m3867 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f800(v0: u64, v1: bool) -> u32 {
    if v1 {
        f805({
            let a3868 = v0;
            let a3869 = 4294967296u64;
            let a3870 = 0u64;
            nat_quot(a3868, a3869, a3870)
        })
    } else {
        f805({
            let a3871 = v0;
            let a3872 = 4294967296u64;
            let a3873 = 0u64;
            nat_rem(a3871, a3872, a3873)
        })
    }
}

pub fn f801(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3874 = v2;
    let a3875 = v1;
    if nat_lt(a3874, a3875) {
        let m3878 = {
            let a3876 = v0;
            let a3877 = v2;
            index_bytes(a3876, a3877)
        };
        match m3878 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3880 = {
                    let a3879 = v3;
                    fixed_u32::convert(i128::from(a3879))
                };
                match m3880 {
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
        let a3881 = v2;
        let a3882 = v1;
        if nat_eq(a3881, a3882) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f802(v0: u32, v1: u32, v2: u32) -> u32 {
    let a3883 = f781(v0, v1);
    let a3884 = f799(v0, v2);
    fixed_u32::bit_or(a3883, a3884)
}

pub fn f803(v0: u32) -> u32 {
    let a3887 = {
        let a3885 = f802(v0, 17u32, 15u32);
        let a3886 = f802(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a3885, a3886)
    };
    let a3888 = f781(v0, 10u32);
    fixed_u32::bit_xor(a3887, a3888)
}

pub fn f804(v0: u32) -> u32 {
    let a3891 = {
        let a3889 = f802(v0, 7u32, 25u32);
        let a3890 = f802(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a3889, a3890)
    };
    let a3892 = f781(v0, 3u32);
    fixed_u32::bit_xor(a3891, a3892)
}

pub fn f805(v0: u64) -> u32 {
    let (_, h3893) = {
        let (_, h3894) = f806(Fn8::F807, 32u64, (v0, (1u32, 0u32)));
        h3894
    };
    h3893
}

pub fn f806(v0: Fn8, p3898: u64, p3899: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m3897 = (p3898, p3899);
    {
        let mut m3897 = m3897;
        loop {
            let step = {
                let (v1, v2) = m3897;
                let m3895 = v1;
                if m3895 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m3895 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c3896: Fn8 = v0.clone();
                        c3896.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m3897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f807(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h3900, _) = v0;
        h3900
    };
    let v2: u32 = {
        let (h3901, _) = {
            let (_, h3902) = v0;
            h3902
        };
        h3901
    };
    let v3: u32 = {
        let (_, h3903) = {
            let (_, h3904) = v0;
            h3904
        };
        h3903
    };
    ({
        let a3905 = v1;
        let a3906 = 2u64;
        let a3907 = 0u64;
        nat_quot(a3905, a3906, a3907)
    }, (f799(v2, 1u32), {
        let a3911 = {
            let a3908 = v1;
            let a3909 = 2u64;
            let a3910 = 0u64;
            nat_rem(a3908, a3909, a3910)
        };
        let a3912 = 1u64;
        if nat_eq(a3911, a3912) {
            let a3913 = v3;
            let a3914 = v2;
            fixed_u32::bit_or(a3913, a3914)
        } else {
            v3
        }
    }))
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
