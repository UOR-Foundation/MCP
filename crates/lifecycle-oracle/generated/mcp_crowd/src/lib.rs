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
    C0(List<u64>, List<u64>, List<Adt86>, Bytes, u64, List<List<u64>>),
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
    C0,
    C1(List<Adt99>, Str),
    C2(Str),
    C3(Str),
    C4,
}

#[derive(Clone)]
pub enum Adt99 {
    C0(Str),
    C1(u64),
}

#[derive(Clone)]
pub enum Adt100 {
    C0,
    C1(List<Adt99>, Str),
}

#[derive(Clone)]
pub enum Adt101 {
    C0(Adt6, Adt6, List<Adt99>),
    C1(List<Adt6>, Adt6, List<Adt99>),
    C2(List<Adt6>, Adt6, List<Adt99>, u64),
    C3(List<Adt99>),
    C4(Option<Adt6>, Option<Adt6>, Adt6, List<Adt99>),
    C5(Adt102, Adt6, Adt6, List<Adt99>),
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
    C0(Adt92, Str, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt104 {
    C0(Adt6, bool, List<Str>),
}

#[derive(Clone)]
pub enum Adt105 {
    C0(Adt100, List<Adt101>),
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
    C0(Option<Str>, Option<Str>, Option<Str>, Option<Adt108>, List<Adt110>),
}

#[derive(Clone)]
pub enum Adt108 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt109),
    C7(Adt109),
    C8(Str),
    C9(Str),
    C10(Str),
}

#[derive(Clone)]
pub enum Adt109 {
    C0,
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt110 {
    C0(Str, List<Str>, Adt13, Adt111),
}

#[derive(Clone)]
pub enum Adt111 {
    C0,
    C1,
    C2(Str),
}

#[derive(Clone)]
pub enum Adt112 {
    C0,
    C1(u64),
    C2(u64),
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
    C0(Adt115, List<Adt119>, u64),
}

#[derive(Clone)]
pub enum Adt115 {
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6(Adt116, List<u64>, bool),
    C7(u64, u64),
    C8(Adt117),
    C9(Adt6),
}

#[derive(Clone)]
pub enum Adt116 {
    C0,
    C1,
    C2(u64, u64, u64),
    C3(u64),
    C4(u64),
}

#[derive(Clone)]
pub enum Adt117 {
    C0(bool, Adt118, i64, u64, i64, bool, i64),
}

#[derive(Clone)]
pub enum Adt118 {
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
pub enum Adt119 {
    C0(List<Adt6>),
    C1(List<(Str, Adt6)>, Option<Str>),
}

#[derive(Clone)]
pub enum Adt120 {
    C0(List<Adt12>, List<List<u64>>),
}

#[derive(Clone)]
pub enum Adt121 {
    C0(Str),
    C1(Str),
    C2(Str),
    C3(List<Adt101>),
}

#[derive(Clone)]
pub enum Adt122 {
    C0(Adt92, Result<Adt123, Adt126>),
}

#[derive(Clone)]
pub enum Adt123 {
    C0(Adt32, Adt124, List<Adt125>),
}

#[derive(Clone)]
pub enum Adt124 {
    C0(Option<Adt1>, Option<Adt5>),
}

#[derive(Clone)]
pub enum Adt125 {
    C0(Adt19, Adt50),
}

#[derive(Clone)]
pub enum Adt126 {
    C0(Adt127),
    C1(Str, List<Str>),
    C2(Adt108),
    C3(Str),
    C4(Adt84),
}

#[derive(Clone)]
pub enum Adt127 {
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
pub enum Adt128 {
    C0(Option<Str>, Option<Adt108>),
}

#[derive(Clone)]
pub enum Adt129 {
    C0,
    C1(Adt92),
    C2,
}

#[derive(Clone)]
pub enum Adt130 {
    C0,
    C1(Adt6),
}

#[derive(Clone)]
pub enum Adt131 {
    C0(Adt93),
    C1(Adt114),
}

#[derive(Clone)]
pub enum Adt132 {
    C0(Adt93),
    C1(List<Adt106>, List<u64>),
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
    C1,
    C2,
}

#[derive(Clone)]
pub enum Adt137 {
    C0,
    C1(Adt134),
}

#[derive(Clone)]
pub enum Adt138 {
    C0(Adt139),
    C1(Adt126),
}

#[derive(Clone)]
pub enum Adt139 {
    C0(Adt92, Str, Adt16, Adt5),
}

#[derive(Clone)]
pub enum Adt140 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt141 {
    C0,
    C1,
}

#[derive(Clone)]
pub enum Adt142 {
    C0(Adt91),
    C1(Adt120),
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
    f24(v0, v1, v2.clone(), f25(match v2.clone() {
        Adt85::C0(_, _, h27, _, _, _) => {
            h27
        }
    }, match v2 {
        Adt85::C0(_, _, _, _, _, h28) => {
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
        Adt85::C0(h160, _, _, _, _, _) => {
            h160
        }
    }) {
        f67(v0, v1, match v2.clone() {
            Adt85::C0(_, _, _, _, h161, _) => {
                h161
            }
        }, match v2.clone() {
            Adt85::C0(_, _, h162, _, _, _) => {
                h162
            }
        }, f68(match v2 {
            Adt85::C0(_, _, _, h163, _, _) => {
                h163
            }
        })?)
    } else {
        Ok(f69())
    }
}

pub fn f46(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m164 = f70(v0, f71())?;
    match m164 {
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
            f72(f73(v2), v1, Adt90::C0)
        }
    }
}

pub fn f47(v0: List<Adt86>, v1: List<List<u64>>) -> R<Option<Adt90>> {
    let m165 = f70(v0, f74())?;
    match m165 {
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
            f72(f75(v2), v1, Adt90::C1)
        }
    }
}

pub fn f48(v0: List<Adt35>) -> Option<Adt91> {
    let m166 = f76(v0, List::<Str>::nil());
    match m166 {
        None => {
            None::<Adt91>
        }
        Some(v1) => {
            Some(Adt91::C1(v1))
        }
    }
}

pub fn f49(p170: List<Adt35>, p171: List<Adt12>) -> R<Result<List<Adt12>, Adt91>> {
    let m169 = (p170, p171);
    {
        let mut m169 = m169;
        loop {
            let step = {
                let (v0, v1) = m169;
                let m167 = v0;
                match m167.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Ok::<List<Adt12>, Adt91>(f77(v1, List::<Adt12>::nil()))))
                    }
                    Some((v2, v3)) => {
                        let m168 = f78(v2)?;
                        match m168 {
                            Ok(v5) => {
                                Ok::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>((v3, f77(v5, v1)))
                            }
                            Err(v4) => {
                                Err::<(List<Adt35>, List<Adt12>), R<Result<List<Adt12>, Adt91>>>(Ok(Err::<List<Adt12>, Adt91>(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m169 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f50(v0: Str, v1: Str, v2: Adt5) -> Adt35 {
    Adt35::C0(v0, None::<Str>, None::<List<Adt2>>, Some(v1), v2, None::<Adt5>, None::<Adt36>, None::<Adt5>)
}

pub fn f51(v0: u64) -> R<Str> {
    let a173 = List::cons(Str::lit("crowd_"), List::cons({
        let a172 = f79(v0, 0i64)?;
        format_i64(a172)
    }, List::<Str>::nil()));
    let a174 = Str::lit("");
    Ok(join(a173, a174))
}

pub fn f52() -> Adt5 {
    Adt5::C0(List::cons((Str::lit("type"), Adt6::C3(Str::lit("object"))), List::cons((Str::lit("properties"), Adt6::C5(List::<(Str, Adt6)>::nil())), List::cons((Str::lit("required"), Adt6::C4(List::<Adt6>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f53(v0: Str, p180: List<Adt35>) -> Option<Adt35> {
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

pub fn f54(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m187 = f80(Adt6::C5({
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
    }), f81(v2.clone()))?;
    match m187 {
        Adt98::C0 => {
            f82(v0, v1, v2)
        }
        Adt98::C1(v3, v4) => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(f83(f84(v3, v4)?, true))))
        }
        Adt98::C2(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema is malformed"))))
        }
        Adt98::C3(_) => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema has an unresolved reference"))))
        }
        Adt98::C4 => {
            Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the inputSchema exceeded the validation budget"))))
        }
    }
}

pub fn f55(v0: List<Adt38>) -> Adt49 {
    f85(v0, None::<Adt6>, false, List::<Adt50>::nil())
}

pub fn f56(v0: Str) -> Adt38 {
    Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>))
}

pub fn f57(v0: Bytes, v1: u64) -> R<u64> {
    let m190 = {
        let a188 = v0;
        let a189 = v1;
        index_bytes(a188, a189)
    };
    match m190 {
        None => {
            Ok(0u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a235 = {
                let a211 = {
                    let a199 = {
                        let a193 = {
                            let a191 = v3;
                            let a192 = 128u8;
                            fixed_u8::bit_and(a191, a192)
                        };
                        let a194 = 128u8;
                        if equal(a193, a194) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a200 = {
                        let a197 = {
                            let a195 = v3;
                            let a196 = 64u8;
                            fixed_u8::bit_and(a195, a196)
                        };
                        let a198 = 64u8;
                        if equal(a197, a198) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a199, a200)?
                };
                let a212 = {
                    let a209 = {
                        let a203 = {
                            let a201 = v3;
                            let a202 = 32u8;
                            fixed_u8::bit_and(a201, a202)
                        };
                        let a204 = 32u8;
                        if equal(a203, a204) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a210 = {
                        let a207 = {
                            let a205 = v3;
                            let a206 = 16u8;
                            fixed_u8::bit_and(a205, a206)
                        };
                        let a208 = 16u8;
                        if equal(a207, a208) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a209, a210)?
                };
                nat_add(a211, a212)?
            };
            let a236 = {
                let a233 = {
                    let a221 = {
                        let a215 = {
                            let a213 = v3;
                            let a214 = 8u8;
                            fixed_u8::bit_and(a213, a214)
                        };
                        let a216 = 8u8;
                        if equal(a215, a216) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a222 = {
                        let a219 = {
                            let a217 = v3;
                            let a218 = 4u8;
                            fixed_u8::bit_and(a217, a218)
                        };
                        let a220 = 4u8;
                        if equal(a219, a220) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a221, a222)?
                };
                let a234 = {
                    let a231 = {
                        let a225 = {
                            let a223 = v3;
                            let a224 = 2u8;
                            fixed_u8::bit_and(a223, a224)
                        };
                        let a226 = 2u8;
                        if equal(a225, a226) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a232 = {
                        let a229 = {
                            let a227 = v3;
                            let a228 = 1u8;
                            fixed_u8::bit_and(a227, a228)
                        };
                        let a230 = 1u8;
                        if equal(a229, a230) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a231, a232)?
                };
                nat_add(a233, a234)?
            };
            nat_add(a235, a236)
        }
    }
}

pub fn f58(v0: Bytes, p246: u64, p247: u64, p248: List<Adt86>) -> R<Result<(List<Adt86>, u64), Adt87>> {
    let m245 = (p246, (p247, p248));
    {
        let mut m245 = m245;
        loop {
            let step = {
                let (v1, (v2, v3)) = m245;
                let m237 = v2;
                if m237 == 0 {
                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Ok::<(List<Adt86>, u64), Adt87>((f86(v3, List::<Adt86>::nil()), v1))))
                } else {
                    let v4 = m237 - 1;
                    let m238 = f16(v0.clone(), v1)?;
                    match m238 {
                        None => {
                            Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            let m240 = f16(v0.clone(), match v5.clone() {
                                Adt89::C0(_, h239) => {
                                    h239
                                }
                            })?;
                            match m240 {
                                None => {
                                    Err::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>(Ok(Err::<(List<Adt86>, u64), Adt87>(Adt87::C0(match v5 {
                                        Adt89::C0(_, h241) => {
                                            h241
                                        }
                                    }))))
                                }
                                Some(v6) => {
                                    Ok::<(u64, (u64, List<Adt86>)), R<Result<(List<Adt86>, u64), Adt87>>>((match v6.clone() {
                                        Adt89::C0(_, h242) => {
                                            h242
                                        }
                                    }, (v4, List::cons(Adt86::C0(f87(match v5 {
                                        Adt89::C0(h243, _) => {
                                            h243
                                        }
                                    })?, f87(match v6 {
                                        Adt89::C0(h244, _) => {
                                            h244
                                        }
                                    })?), v3))))
                                }
                            }
                        }
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

pub fn f59(v0: Bytes, v1: Adt89, v2: Adt89, v3: (List<Adt86>, u64)) -> R<Result<Adt85, Adt87>> {
    let m250 = f16(v0.clone(), {
        let (_, h249) = v3.clone();
        h249
    })?;
    match m250 {
        None => {
            Ok(Err::<Adt85, Adt87>(Adt87::C0({
                let (_, h251) = v3;
                h251
            })))
        }
        Some(v4) => {
            let m253 = f33(v0.clone(), match v4.clone() {
                Adt89::C0(_, h252) => {
                    h252
                }
            })?;
            match m253 {
                None => {
                    Ok(Err::<Adt85, Adt87>(Adt87::C0(match v4 {
                        Adt89::C0(_, h254) => {
                            h254
                        }
                    })))
                }
                Some(v5) => {
                    f88(v0, Adt85::C0(f87(match v1 {
                        Adt89::C0(h255, _) => {
                            h255
                        }
                    })?, f87(match v2 {
                        Adt89::C0(h256, _) => {
                            h256
                        }
                    })?, {
                        let (h257, _) = v3;
                        h257
                    }, match v4.clone() {
                        Adt89::C0(h258, _) => {
                            h258
                        }
                    }, v5, List::<List<u64>>::nil()), {
                        let a260 = match v4 {
                            Adt89::C0(_, h259) => {
                                h259
                            }
                        };
                        let a261 = 4u64;
                        nat_add(a260, a261)?
                    })
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
    let m277 = f89(v1)?;
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
            f90()
        }
        Adt90::C1 => {
            f90()
        }
    }
}

pub fn f64(v0: Option<Adt92>, v1: Adt80) -> Adt6 {
    Adt6::C5(f91(v0, v1))
}

pub fn f65(v0: Adt90) -> Adt80 {
    Adt80::C0(f92(), f93(v0), None::<Adt6>)
}

pub fn f66(v0: List<u64>) -> bool {
    f94(v0, f95())
}

pub fn f67(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt6, Adt93>) -> R<Adt88> {
    let m279 = v4;
    match m279 {
        Ok(v6) => {
            f98(v0, v1, v2, v3, f99(v6)?)
        }
        Err(_) => {
            f62(f96(), f64(None::<Adt92>, f97(Adt94::C0)))
        }
    }
}

pub fn f68(v0: Bytes) -> R<Result<Adt6, Adt93>> {
    f100(f87(v0)?)
}

pub fn f69() -> Adt88 {
    Adt88::C0(f101(), List::cons(Adt86::C0(f102(), f103()), List::<Adt86>::nil()), List::<List<u64>>::nil(), List::<u64>::nil())
}

pub fn f70(v0: List<Adt86>, v1: List<u64>) -> R<Adt96> {
    let m280 = f104(v0, v1, Adt97::C0)?;
    match m280 {
        Adt97::C0 => {
            Ok(Adt96::C0)
        }
        Adt97::C1(v2) => {
            let m281 = f105(v2);
            match m281 {
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

pub fn f71() -> List<u64> {
    List::cons(79u64, List::cons(114u64, List::cons(105u64, List::cons(103u64, List::cons(105u64, List::cons(110u64, List::<u64>::nil()))))))
}

pub fn f72(v0: Option<List<u64>>, v1: List<List<u64>>, v2: Adt90) -> R<Option<Adt90>> {
    let m282 = v0;
    match m282 {
        None => {
            Ok(Some(v2))
        }
        Some(v3) => {
            if f106(v3, v1)? {
                Ok(None::<Adt90>)
            } else {
                Ok(Some(v2))
            }
        }
    }
}

pub fn f73(v0: List<u64>) -> Option<List<u64>> {
    let m283 = f107(v0);
    match m283 {
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
    let m284 = v0.clone();
    match m284.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let a285 = v1;
            let a286 = 91u64;
            if nat_eq(a285, a286) {
                f108(f109(v2, List::cons(v1, List::<u64>::nil())))
            } else {
                f108(f110(v0, List::<u64>::nil()))
            }
        }
    }
}

pub fn f76(p292: List<Adt35>, p293: List<Str>) -> Option<Str> {
    let m291 = (p292, p293);
    {
        let mut m291 = m291;
        loop {
            let step = {
                let (v0, v1) = m291;
                let m287 = v0;
                match m287.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Str>), Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        if f111(match v2.clone() {
                            Adt35::C0(h288, _, _, _, _, _, _, _) => {
                                h288
                            }
                        }, v1.clone()) {
                            Err::<(List<Adt35>, List<Str>), Option<Str>>(Some(match v2 {
                                Adt35::C0(h289, _, _, _, _, _, _, _) => {
                                    h289
                                }
                            }))
                        } else {
                            Ok::<(List<Adt35>, List<Str>), Option<Str>>((v3, List::cons(match v2 {
                                Adt35::C0(h290, _, _, _, _, _, _, _) => {
                                    h290
                                }
                            }, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m291 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(p296: List<Adt12>, p297: List<Adt12>) -> List<Adt12> {
    let m295 = (p296, p297);
    {
        let mut m295 = m295;
        loop {
            let step = {
                let (v0, v1) = m295;
                let m294 = v0;
                match m294.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m295 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f78(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m300 = f112(match v0.clone() {
        Adt35::C0(h298, _, _, _, _, _, _, _) => {
            h298
        }
    }, f113(match v0.clone() {
        Adt35::C0(h299, _, _, _, _, _, _, _) => {
            h299
        }
    })?);
    match m300 {
        None => {
            f114(v0)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f79(p303: u64, p304: i64) -> R<i64> {
    let m302 = (p303, p304);
    {
        let mut m302 = m302;
        loop {
            let step = {
                let (v0, v1) = m302;
                let m301 = v0;
                if m301 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m301 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, f115(v1, 1i64)?))
                }
            };
            match step {
                Ok(next) => { m302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f80(v0: Adt6, v1: Adt6, v2: Adt6) -> R<Adt98> {
    f116(v0, f117(), Adt100::C0, List::cons(Adt101::C0(v1, v2, List::<Adt99>::nil()), List::<Adt101>::nil()))
}

pub fn f81(v0: Adt21) -> Adt6 {
    let m306 = match v0 {
        Adt21::C0(_, _, h305, _, _) => {
            h305
        }
    };
    match m306 {
        None => {
            Adt6::C5(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Adt6::C5(match v1 {
                Adt5::C0(h307) => {
                    h307
                }
            })
        }
    }
}

pub fn f82(v0: Adt35, v1: Fn1, v2: Adt21) -> R<Result<Adt32, Adt84>> {
    let m309 = {
        let c308: Fn1 = v1;
        c308.apply(v2)
    };
    match m309 {
        Ok(v4) => {
            f118(v0, v4)
        }
        Err(v3) => {
            Ok(Err::<Adt32, Adt84>(v3))
        }
    }
}

pub fn f83(v0: Str, v1: bool) -> Adt37 {
    Adt37::C0(List::cons(Adt38::C0(Adt39::C0(v0, None::<Adt40>, None::<Adt5>)), List::<Adt38>::nil()), None::<Adt6>, if v1 {
        Some(true)
    } else {
        None::<bool>
    })
}

pub fn f84(v0: List<Adt99>, v1: Str) -> R<Str> {
    let v2: Str = f119(v0)?;
    let a312 = List::cons(Str::lit("Invalid arguments:"), List::cons({
        let a310 = v2.clone();
        let a311 = Str::lit("");
        if equal(a310, a311) {
            Str::lit("the arguments object")
        } else {
            v2
        }
    }, List::cons(Str::lit("fails"), List::cons(v1, List::<Str>::nil()))));
    let a313 = Str::lit(" ");
    Ok(join(a312, a313))
}

pub fn f85(v0: List<Adt38>, v1: Option<Adt6>, v2: bool, v3: List<Adt50>) -> Adt49 {
    Adt49::C0(Adt37::C0(v0, v1, if v2 {
        Some(true)
    } else {
        None::<bool>
    }), v3)
}

pub fn f86(p316: List<Adt86>, p317: List<Adt86>) -> List<Adt86> {
    let m315 = (p316, p317);
    {
        let mut m315 = m315;
        loop {
            let step = {
                let (v0, v1) = m315;
                let m314 = v0;
                match m314.uncons() {
                    None => {
                        Err::<(List<Adt86>, List<Adt86>), List<Adt86>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt86>, List<Adt86>), List<Adt86>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m315 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f87(v0: Bytes) -> R<List<u64>> {
    f120(v0.clone(), {
        let a318 = v0;
        length_bytes(a318)
    }, List::<u64>::nil())
}

pub fn f88(v0: Bytes, v1: Adt85, v2: u64) -> R<Result<Adt85, Adt87>> {
    let a320 = v2;
    let a321 = {
        let a319 = v0.clone();
        length_bytes(a319)
    };
    if nat_eq(a320, a321) {
        Ok(Ok::<Adt85, Adt87>(v1))
    } else {
        let m322 = f33(v0.clone(), v2)?;
        match m322 {
            None => {
                Ok(Err::<Adt85, Adt87>(Adt87::C0(v2)))
            }
            Some(v3) => {
                let m325 = f121(v0.clone(), {
                    let a323 = v2;
                    let a324 = 4u64;
                    nat_add(a323, a324)?
                }, v3, List::<List<u64>>::nil())?;
                match m325 {
                    Ok(v5) => {
                        let a328 = {
                            let (_, h326) = v5.clone();
                            h326
                        };
                        let a329 = {
                            let a327 = v0;
                            length_bytes(a327)
                        };
                        if nat_eq(a328, a329) {
                            Ok(Ok::<Adt85, Adt87>(f122(v1, {
                                let (h330, _) = v5;
                                h330
                            })))
                        } else {
                            Ok(Err::<Adt85, Adt87>(Adt87::C1({
                                let (_, h331) = v5;
                                h331
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

pub fn f89(v0: Adt6) -> R<Result<List<u64>, Adt93>> {
    f123(f124(), v0)
}

pub fn f90() -> u64 {
    403u64
}

pub fn f91(v0: Option<Adt92>, v1: Adt80) -> List<(Str, Adt6)> {
    List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), {
        let a332 = f126(v0);
        let a333 = List::cons((Str::lit("error"), f127(v1)), List::<(Str, Adt6)>::nil());
        append_list(a332, a333)
    })
}

pub fn f92() -> i64 {
    -32000i64
}

pub fn f93(v0: Adt90) -> Str {
    let m334 = v0;
    match m334 {
        Adt90::C0 => {
            Str::lit("Forbidden: invalid Origin")
        }
        Adt90::C1 => {
            Str::lit("Forbidden: invalid Host")
        }
    }
}

pub fn f94(p341: List<u64>, p342: List<u64>) -> bool {
    let m340 = (p341, p342);
    {
        let mut m340 = m340;
        loop {
            let step = {
                let (v0, v1) = m340;
                let m335 = v0;
                match m335.uncons() {
                    None => {
                        let m336 = v1;
                        match m336.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m337 = v1;
                        match m337.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a338 = v4;
                                let a339 = v6;
                                if nat_eq(a338, a339) {
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
                Ok(next) => { m340 = next; }
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

pub fn f97(v0: Adt94) -> Adt80 {
    Adt80::C0(f128(v0.clone()), f129(v0), None::<Adt6>)
}

pub fn f98(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Result<Adt103, Adt94>) -> R<Adt88> {
    let m343 = v4;
    match m343 {
        Ok(v6) => {
            f132(v0, v1, v2, v3, v6)
        }
        Err(v5) => {
            f62(f130(v5.clone()), f64(f131(v5.clone()), f97(v5)))
        }
    }
}

pub fn f99(v0: Adt6) -> R<Result<Adt103, Adt94>> {
    let m344 = v0;
    match m344 {
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

pub fn f100(v0: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m345 = f135(v0.clone())?;
    match m345 {
        Adt112::C0 => {
            f136(f137(), 0u64, v0)
        }
        Adt112::C1(v1) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C0(v1)))
        }
        Adt112::C2(v2) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C1(v2)))
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

pub fn f104(p350: List<Adt86>, v1: List<u64>, p351: Adt97) -> R<Adt97> {
    let m349 = (p350, p351);
    {
        let mut m349 = m349;
        loop {
            let step = {
                let (v0, v2) = m349;
                let m346 = v0;
                match m346.uncons() {
                    None => {
                        Err::<(List<Adt86>, Adt97), R<Adt97>>(Ok(v2))
                    }
                    Some((v3, v4)) => {
                        if f138(match v3.clone() {
                            Adt86::C0(h347, _) => {
                                h347
                            }
                        }, v1.clone())? {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, f139(v2, match v3 {
                                Adt86::C0(_, h348) => {
                                    h348
                                }
                            })))
                        } else {
                            Ok::<(List<Adt86>, Adt97), R<Adt97>>((v4, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m349 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: List<u64>) -> Option<List<u64>> {
    let v1: List<u64> = f140(v0);
    if f141(v1.clone()) {
        Some(v1)
    } else {
        None::<List<u64>>
    }
}

pub fn f106(v0: List<u64>, v1: List<List<u64>>) -> R<bool> {
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

pub fn f107(p356: List<u64>) -> Option<List<u64>> {
    let m355 = p356;
    {
        let mut m355 = m355;
        loop {
            let step = {
                let v0 = m355;
                let m352 = v0;
                match m352.uncons() {
                    None => {
                        Err::<List<u64>, Option<List<u64>>>(None::<List<u64>>)
                    }
                    Some((v1, v2)) => {
                        let a353 = v1;
                        let a354 = 58u64;
                        if nat_eq(a353, a354) {
                            Err::<List<u64>, Option<List<u64>>>(f146(v2))
                        } else {
                            Ok::<List<u64>, Option<List<u64>>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f108(v0: Option<(List<u64>, List<u64>)>) -> Option<List<u64>> {
    let m357 = v0;
    match m357 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            if f147({
                let (_, h358) = v1;
                h358
            }) {
                Some({
                    let (h359, _) = v1;
                    h359
                })
            } else {
                None::<List<u64>>
            }
        }
    }
}

pub fn f109(p364: List<u64>, p365: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m363 = (p364, p365);
    {
        let mut m363 = m363;
        loop {
            let step = {
                let (v0, v1) = m363;
                let m360 = v0;
                match m360.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(None::<(List<u64>, List<u64>)>)
                    }
                    Some((v2, v3)) => {
                        let a361 = v2;
                        let a362 = 93u64;
                        if nat_eq(a361, a362) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(List::cons(v2, v1), List::<u64>::nil()), v3)))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
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

pub fn f110(p370: List<u64>, p371: List<u64>) -> Option<(List<u64>, List<u64>)> {
    let m369 = (p370, p371);
    {
        let mut m369 = m369;
        loop {
            let step = {
                let (v0, v1) = m369;
                let m366 = v0;
                match m366.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(v1, List::<u64>::nil()), List::<u64>::nil())))
                    }
                    Some((v2, v3)) => {
                        let a367 = v2;
                        let a368 = 58u64;
                        if nat_eq(a367, a368) {
                            Err::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>(Some((f148(v1, List::<u64>::nil()), List::cons(v2, v3))))
                        } else {
                            Ok::<(List<u64>, List<u64>), Option<(List<u64>, List<u64>)>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: Str, p376: List<Str>) -> bool {
    let m375 = p376;
    {
        let mut m375 = m375;
        loop {
            let step = {
                let v1 = m375;
                let m372 = v1;
                match m372.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a373 = v0.clone();
                        let a374 = v2;
                        if equal(a373, a374) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f112(v0: Str, v1: List<u64>) -> Option<Adt91> {
    if f149(v1) {
        Some(Adt91::C0(v0))
    } else {
        None::<Adt91>
    }
}

pub fn f113(v0: Str) -> R<List<u64>> {
    f87({
        let a377 = v0;
        utf8_encode(a377)
    })
}

pub fn f114(v0: Adt35) -> R<Result<List<Adt12>, Adt91>> {
    let m378 = f150(v0.clone());
    match m378 {
        None => {
            f151(v0.clone(), f152(f153(), match v0.clone() {
                Adt35::C0(h379, _, _, _, _, _, _, _) => {
                    h379
                }
            }, Adt6::C5({
                let m382 = match v0.clone() {
                    Adt35::C0(_, _, _, _, h381, _, _, _) => {
                        h381
                    }
                };
                match m382 {
                    Adt5::C0(h380) => {
                        h380
                    }
                }
            }), List::cons(Adt104::C0(Adt6::C5({
                let m385 = match v0 {
                    Adt35::C0(_, _, _, _, h384, _, _, _) => {
                        h384
                    }
                };
                match m385 {
                    Adt5::C0(h383) => {
                        h383
                    }
                }
            }), true, List::<Str>::nil()), List::<Adt104>::nil()), f154())?)
        }
        Some(v1) => {
            Ok(Err::<List<Adt12>, Adt91>(v1))
        }
    }
}

pub fn f115(v0: i64, v1: i64) -> R<i64> {
    let a387 = v0;
    let a388 = {
        let a386 = v1;
        int_neg(a386)?
    };
    int_sub(a387, a388)
}

pub fn f116(v0: Adt6, p407: u64, p408: Adt100, p409: List<Adt101>) -> R<Adt98> {
    let m406 = (p407, (p408, p409));
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v1, (v2, v3)) = m406;
                let m389 = v1;
                if m389 == 0 {
                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C4))
                } else {
                    let v4 = m389 - 1;
                    let m390 = v3;
                    match m390.uncons() {
                        None => {
                            let m391 = v2;
                            match m391 {
                                Adt100::C0 => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C0))
                                }
                                Adt100::C1(v5, v6) => {
                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C1(v5, v6)))
                                }
                            }
                        }
                        Some((v7, v8)) => {
                            let m392 = v2.clone();
                            match m392 {
                                Adt100::C0 => {
                                    let m393 = v7.clone();
                                    match m393 {
                                        Adt101::C0(v9, v10, v11) => {
                                            let m394 = f155(v0.clone(), v9, v10, v11.clone())?;
                                            match m394 {
                                                Adt121::C0(v12) => {
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (Adt100::C1(v11, v12), v8)))
                                                }
                                                Adt121::C1(v13) => {
                                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C2(v13)))
                                                }
                                                Adt121::C2(v14) => {
                                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C3(v14)))
                                                }
                                                Adt121::C3(v15) => {
                                                    let v16: List<Adt101> = {
                                                        let a395 = v15;
                                                        let a396 = v8;
                                                        append_list(a395, a396)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v16)))
                                                }
                                            }
                                        }
                                        Adt101::C1(_, _, _) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v8)))
                                        }
                                        Adt101::C2(_, _, _, _) => {
                                            let v24: Adt105 = f156(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v24.clone() {
                                                Adt105::C0(h397, _) => {
                                                    h397
                                                }
                                            }, match v24 {
                                                Adt105::C0(_, h398) => {
                                                    h398
                                                }
                                            })))
                                        }
                                        Adt101::C3(v25) => {
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (Adt100::C1(v25, Str::lit("not")), v8)))
                                        }
                                        Adt101::C4(_, _, _, _) => {
                                            let v39: Adt105 = f156(v7, v8)?;
                                            Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v39.clone() {
                                                Adt105::C0(h402, _) => {
                                                    h402
                                                }
                                            }, match v39 {
                                                Adt105::C0(_, h403) => {
                                                    h403
                                                }
                                            })))
                                        }
                                        Adt101::C5(v26, v27, v28, v29) => {
                                            let m399 = f157(v26, v27, v28, v29.clone());
                                            match m399 {
                                                Adt121::C0(v30) => {
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (Adt100::C1(v29, v30), v8)))
                                                }
                                                Adt121::C1(v31) => {
                                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C2(v31)))
                                                }
                                                Adt121::C2(v32) => {
                                                    Err::<(u64, (Adt100, List<Adt101>)), R<Adt98>>(Ok(Adt98::C3(v32)))
                                                }
                                                Adt121::C3(v33) => {
                                                    let v34: List<Adt101> = {
                                                        let a400 = v33;
                                                        let a401 = v8;
                                                        append_list(a400, a401)
                                                    };
                                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (v2, v34)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt100::C1(v40, v41) => {
                                    let v42: Adt105 = f158(v7, v8, v40, v41);
                                    Ok::<(u64, (Adt100, List<Adt101>)), R<Adt98>>((v4, (match v42.clone() {
                                        Adt105::C0(h404, _) => {
                                            h404
                                        }
                                    }, match v42 {
                                        Adt105::C0(_, h405) => {
                                            h405
                                        }
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117() -> u64 {
    67108864u64
}

pub fn f118(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f159({
        let m412 = match v1.clone() {
            Adt49::C0(h411, _) => {
                h411
            }
        };
        match m412 {
            Adt37::C0(h410, _, _) => {
                h410
            }
        }
    })? {
        f160(v0, v1)
    } else {
        Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("a content payload is not canonical Base64"))))
    }
}

pub fn f119(v0: List<Adt99>) -> R<Str> {
    let a413 = List::cons(Str::lit(""), f161(v0, List::<Str>::nil())?);
    let a414 = Str::lit("/");
    Ok(join(a413, a414))
}

pub fn f120(v0: Bytes, p417: u64, p418: List<u64>) -> R<List<u64>> {
    let m416 = (p417, p418);
    {
        let mut m416 = m416;
        loop {
            let step = {
                let (v1, v2) = m416;
                let m415 = v1;
                if m415 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m415 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, List::cons(f57(v0.clone(), v3)?, v2)))
                }
            };
            match step {
                Ok(next) => { m416 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Bytes, p424: u64, p425: u64, p426: List<List<u64>>) -> R<Result<(List<List<u64>>, u64), Adt87>> {
    let m423 = (p424, (p425, p426));
    {
        let mut m423 = m423;
        loop {
            let step = {
                let (v1, (v2, v3)) = m423;
                let m419 = v2;
                if m419 == 0 {
                    Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Ok::<(List<List<u64>>, u64), Adt87>((f38(v3, List::<List<u64>>::nil()), v1))))
                } else {
                    let v4 = m419 - 1;
                    let m420 = f16(v0.clone(), v1)?;
                    match m420 {
                        None => {
                            Err::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>(Ok(Err::<(List<List<u64>>, u64), Adt87>(Adt87::C0(v1))))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, List<List<u64>>)), R<Result<(List<List<u64>>, u64), Adt87>>>((match v5.clone() {
                                Adt89::C0(_, h421) => {
                                    h421
                                }
                            }, (v4, List::cons(f87(match v5 {
                                Adt89::C0(h422, _) => {
                                    h422
                                }
                            })?, v3))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt85, v1: List<List<u64>>) -> Adt85 {
    Adt85::C0(match v0.clone() {
        Adt85::C0(h427, _, _, _, _, _) => {
            h427
        }
    }, match v0.clone() {
        Adt85::C0(_, h428, _, _, _, _) => {
            h428
        }
    }, match v0.clone() {
        Adt85::C0(_, _, h429, _, _, _) => {
            h429
        }
    }, match v0.clone() {
        Adt85::C0(_, _, _, h430, _, _) => {
            h430
        }
    }, match v0 {
        Adt85::C0(_, _, _, _, h431, _) => {
            h431
        }
    }, v1)
}

pub fn f123(v0: u64, v1: Adt6) -> R<Result<List<u64>, Adt93>> {
    let m432 = f162(v0, List::cons(Adt106::C1(0u64, v1), List::<Adt106>::nil()), List::<u64>::nil())?;
    match m432 {
        Ok(v2) => {
            Ok(Ok::<List<u64>, Adt93>(f163(v2, List::<u64>::nil())))
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
    let m433 = v0;
    match m433 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("id"), f164(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f127(v0: Adt80) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("code"), Adt6::C2(Adt7::C0(match v0.clone() {
        Adt80::C0(h434, _, _) => {
            h434
        }
    }, 0i64))), List::cons((Str::lit("message"), Adt6::C3(match v0.clone() {
        Adt80::C0(_, h435, _) => {
            h435
        }
    })), f165(match v0 {
        Adt80::C0(_, _, h436) => {
            h436
        }
    }))))
}

pub fn f128(v0: Adt94) -> i64 {
    let m437 = v0;
    match m437 {
        Adt94::C0 => {
            f166()
        }
        Adt94::C1(_, _) => {
            f167()
        }
    }
}

pub fn f129(v0: Adt94) -> Str {
    let m438 = v0;
    match m438 {
        Adt94::C0 => {
            Str::lit("Parse error")
        }
        Adt94::C1(_, v2) => {
            f168(v2)
        }
    }
}

pub fn f130(v0: Adt94) -> u64 {
    let m439 = v0;
    match m439 {
        Adt94::C0 => {
            f96()
        }
        Adt94::C1(_, _) => {
            f96()
        }
    }
}

pub fn f131(v0: Adt94) -> Option<Adt92> {
    let m440 = v0;
    match m440 {
        Adt94::C0 => {
            None::<Adt92>
        }
        Adt94::C1(v1, _) => {
            v1
        }
    }
}

pub fn f132(v0: Adt0, v1: Fn0, v2: u64, v3: List<Adt86>, v4: Adt103) -> R<Adt88> {
    f169(f170(v3.clone())?, f171(v0.clone(), v1, v2, Some(f172(v3, match v0 {
        Adt0::C0(_, _, _, _, h441, _) => {
            h441
        }
    }, f173(v4.clone()), match v4.clone() {
        Adt103::C0(_, h442, _) => {
            h442
        }
    })?), f174(), v4)?)
}

pub fn f133(v0: Adt5) -> R<Result<Adt103, Adt94>> {
    let m443 = f175(v0.clone())?;
    match m443 {
        Adt129::C0 => {
            let m444 = f176(Str::lit("method"), v0.clone());
            match m444 {
                Adt130::C0 => {
                    Ok(f134(None::<Adt92>, f177(v0)))
                }
                Adt130::C1(_) => {
                    Ok(f134(None::<Adt92>, if f178(v0) {
                        Adt95::C8
                    } else {
                        Adt95::C7
                    }))
                }
            }
        }
        Adt129::C1(v2) => {
            Ok(f179(v2, v0))
        }
        Adt129::C2 => {
            Ok(f134(None::<Adt92>, Adt95::C5))
        }
    }
}

pub fn f134(v0: Option<Adt92>, v1: Adt95) -> Result<Adt103, Adt94> {
    Err::<Adt103, Adt94>(Adt94::C1(v0, v1))
}

pub fn f135(v0: List<u64>) -> R<Adt112> {
    Ok(f180({
        let a445 = v0.clone();
        length_list(a445)
    }, f181(Adt113::C0, 0u64, 0u64, v0)?))
}

pub fn f136(p452: Adt114, p453: u64, p454: List<u64>) -> R<Result<Adt6, Adt93>> {
    let m451 = (p452, (p453, p454));
    {
        let mut m451 = m451;
        loop {
            let step = {
                let (v0, (v1, v2)) = m451;
                let m446 = v2;
                match m446.uncons() {
                    None => {
                        let m447 = f182(v0, v1)?;
                        match m447 {
                            Ok(v3) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Ok::<Adt6, Adt93>(v3)))
                            }
                            Err(v4) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v4)))
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m448 = f183(v0, v1, v5)?;
                        match m448 {
                            Adt131::C0(v7) => {
                                Err::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>(Ok(Err::<Adt6, Adt93>(v7)))
                            }
                            Adt131::C1(v8) => {
                                Ok::<(Adt114, (u64, List<u64>)), R<Result<Adt6, Adt93>>>((v8, ({
                                    let a449 = v1;
                                    let a450 = 1u64;
                                    nat_add(a449, a450)?
                                }, v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137() -> Adt114 {
    Adt114::C0(Adt115::C0, List::<Adt119>::nil(), 0u64)
}

pub fn f138(p461: List<u64>, p462: List<u64>) -> R<bool> {
    let m460 = (p461, p462);
    {
        let mut m460 = m460;
        loop {
            let step = {
                let (v0, v1) = m460;
                let m455 = v0;
                match m455.uncons() {
                    None => {
                        let m456 = v1;
                        match m456.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(true))
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m457 = v1;
                        match m457.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), R<bool>>(Ok(false))
                            }
                            Some((v6, v7)) => {
                                let a458 = f184(v4)?;
                                let a459 = f184(v6)?;
                                if nat_eq(a458, a459) {
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
                Ok(next) => { m460 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f139(v0: Adt97, v1: List<u64>) -> Adt97 {
    let m463 = v0;
    match m463 {
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
    f148(f185(f148(f185(v0), List::<u64>::nil())), List::<u64>::nil())
}

pub fn f141(p466: List<u64>) -> bool {
    let m465 = p466;
    {
        let mut m465 = m465;
        loop {
            let step = {
                let v0 = m465;
                let m464 = v0;
                match m464.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f186(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
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

pub fn f142() -> List<u64> {
    List::cons(108u64, List::cons(111u64, List::cons(99u64, List::cons(97u64, List::cons(108u64, List::cons(104u64, List::cons(111u64, List::cons(115u64, List::cons(116u64, List::<u64>::nil())))))))))
}

pub fn f143() -> List<u64> {
    List::cons(49u64, List::cons(50u64, List::cons(55u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(48u64, List::cons(46u64, List::cons(49u64, List::<u64>::nil())))))))))
}

pub fn f144() -> List<u64> {
    List::cons(91u64, List::cons(58u64, List::cons(58u64, List::cons(49u64, List::cons(93u64, List::<u64>::nil())))))
}

pub fn f145(v0: List<u64>, p469: List<List<u64>>) -> R<bool> {
    let m468 = p469;
    {
        let mut m468 = m468;
        loop {
            let step = {
                let v1 = m468;
                let m467 = v1;
                match m467.uncons() {
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
                Ok(next) => { m468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: List<u64>) -> Option<List<u64>> {
    let m470 = v0;
    match m470.uncons() {
        None => {
            None::<List<u64>>
        }
        Some((v1, v2)) => {
            let m471 = v2;
            match m471.uncons() {
                None => {
                    None::<List<u64>>
                }
                Some((v3, v4)) => {
                    let a472 = v1;
                    let a473 = 47u64;
                    if if nat_eq(a472, a473) {
                        let a474 = v3;
                        let a475 = 47u64;
                        nat_eq(a474, a475)
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
    let m476 = v0;
    match m476.uncons() {
        None => {
            true
        }
        Some((v1, v2)) => {
            let a477 = v1;
            let a478 = 58u64;
            if nat_eq(a477, a478) {
                let m479 = v2.clone();
                match m479.uncons() {
                    None => {
                        false
                    }
                    Some((_, _)) => {
                        f187(v2)
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn f148(p482: List<u64>, p483: List<u64>) -> List<u64> {
    let m481 = (p482, p483);
    {
        let mut m481 = m481;
        loop {
            let step = {
                let (v0, v1) = m481;
                let m480 = v0;
                match m480.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m481 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: List<u64>) -> bool {
    let m484 = v0.clone();
    match m484.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            let a488 = if f188(v0.clone(), false) {
                let a486 = {
                    let a485 = v0;
                    length_list(a485)
                };
                let a487 = f189();
                nat_le(a486, a487)
            } else {
                false
            };
            bool_not(a488)
        }
    }
}

pub fn f150(v0: Adt35) -> Option<Adt91> {
    let m493 = f190(match v0.clone() {
        Adt35::C0(h489, _, _, _, _, _, _, _) => {
            h489
        }
    }, {
        let m492 = match v0.clone() {
            Adt35::C0(_, _, _, _, h491, _, _, _) => {
                h491
            }
        };
        match m492 {
            Adt5::C0(h490) => {
                h490
            }
        }
    });
    match m493 {
        None => {
            f191(match v0.clone() {
                Adt35::C0(h494, _, _, _, _, _, _, _) => {
                    h494
                }
            }, {
                let m497 = match v0 {
                    Adt35::C0(_, _, _, _, h496, _, _, _) => {
                        h496
                    }
                };
                match m497 {
                    Adt5::C0(h495) => {
                        h495
                    }
                }
            })
        }
        Some(v1) => {
            Some(v1)
        }
    }
}

pub fn f151(v0: Adt35, v1: Result<Adt120, Adt91>) -> R<Result<List<Adt12>, Adt91>> {
    let m498 = v1;
    match m498 {
        Ok(v3) => {
            let m499 = f192(v0)?;
            match m499 {
                None => {
                    Ok(Ok::<List<Adt12>, Adt91>(f77(match v3 {
                        Adt120::C0(h500, _) => {
                            h500
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

pub fn f152(p507: u64, v1: Str, v2: Adt6, p508: List<Adt104>, p509: Adt120) -> R<Result<Adt120, Adt91>> {
    let m506 = (p507, (p508, p509));
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v0, (v3, v4)) = m506;
                let m501 = v0;
                if m501 == 0 {
                    Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Err::<Adt120, Adt91>(Adt91::C5(v1.clone(), Str::lit("budget")))))
                } else {
                    let v5 = m501 - 1;
                    let m502 = v3;
                    match m502.uncons() {
                        None => {
                            Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Ok::<Adt120, Adt91>(v4)))
                        }
                        Some((v6, v7)) => {
                            let m503 = f193(v1.clone(), v2.clone(), v6, v4, v7)?;
                            match m503 {
                                Ok(v9) => {
                                    Ok::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>((v5, ({
                                        let (h504, _) = v9;
                                        h504
                                    }, {
                                        let (_, h505) = v9;
                                        h505
                                    })))
                                }
                                Err(v8) => {
                                    Err::<(u64, (List<Adt104>, Adt120)), R<Result<Adt120, Adt91>>>(Ok(Err::<Adt120, Adt91>(v8)))
                                }
                            }
                        }
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

pub fn f153() -> u64 {
    67108864u64
}

pub fn f154() -> Adt120 {
    Adt120::C0(List::<Adt12>::nil(), List::<List<u64>>::nil())
}

pub fn f155(v0: Adt6, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m510 = f194(v1.clone());
    match m510 {
        None => {
            let m511 = f195(v1);
            match m511 {
                None => {
                    Ok(Adt121::C1(Str::lit("schema")))
                }
                Some(v5) => {
                    f196(v0, match v5 {
                        Adt5::C0(h512) => {
                            h512
                        }
                    }, v2, v3)
                }
            }
        }
        Some(v4) => {
            if v4 {
                Ok(Adt121::C3(List::<Adt101>::nil()))
            } else {
                Ok(Adt121::C0(Str::lit("false")))
            }
        }
    }
}

pub fn f156(v0: Adt101, v1: List<Adt101>) -> R<Adt105> {
    let m513 = v0;
    match m513 {
        Adt101::C0(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C1(_, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
        Adt101::C2(v8, v9, v10, v11) => {
            Ok(f197(v8, v9, v10, {
                let a514 = v11;
                let a515 = 1u64;
                nat_add(a514, a515)?
            }, v1))
        }
        Adt101::C3(v12) => {
            Ok(Adt105::C0(Adt100::C1(v12, Str::lit("not")), v1))
        }
        Adt101::C4(v13, _, v15, v16) => {
            Ok(f198(v13, v15, v16, v1))
        }
        Adt101::C5(_, _, _, _) => {
            Ok(Adt105::C0(Adt100::C0, v1))
        }
    }
}

pub fn f157(v0: Adt102, v1: Adt6, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m516 = f195(v1);
    match m516 {
        None => {
            Adt121::C1(Str::lit("schema"))
        }
        Some(v4) => {
            let m517 = v0;
            match m517 {
                Adt102::C0 => {
                    f199(match v4 {
                        Adt5::C0(h518) => {
                            h518
                        }
                    }, v2, v3)
                }
                Adt102::C1 => {
                    f200(match v4 {
                        Adt5::C0(h519) => {
                            h519
                        }
                    }, v2, v3)
                }
                Adt102::C2 => {
                    f201(match v4 {
                        Adt5::C0(h520) => {
                            h520
                        }
                    }, v2, v3)
                }
                Adt102::C3 => {
                    f202(match v4 {
                        Adt5::C0(h521) => {
                            h521
                        }
                    }, v2, v3)
                }
            }
        }
    }
}

pub fn f158(v0: Adt101, v1: List<Adt101>, v2: List<Adt99>, v3: Str) -> Adt105 {
    let m522 = v0;
    match m522 {
        Adt101::C0(_, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
        Adt101::C1(v7, v8, v9) => {
            f203(v7, v8, v9, v1)
        }
        Adt101::C2(v10, v11, v12, v13) => {
            f197(v10, v11, v12, v13, v1)
        }
        Adt101::C3(_) => {
            Adt105::C0(Adt100::C0, v1)
        }
        Adt101::C4(_, v16, v17, v18) => {
            f198(v16, v17, v18, v1)
        }
        Adt101::C5(_, _, _, _) => {
            Adt105::C0(Adt100::C1(v2, v3), v1)
        }
    }
}

pub fn f159(p531: List<Adt38>) -> R<bool> {
    let m530 = p531;
    {
        let mut m530 = m530;
        loop {
            let step = {
                let v0 = m530;
                let m523 = v0;
                match m523.uncons() {
                    None => {
                        Err::<List<Adt38>, R<bool>>(Ok(true))
                    }
                    Some((v1, v2)) => {
                        let m524 = v1;
                        match m524 {
                            Adt38::C0(_) => {
                                Ok::<List<Adt38>, R<bool>>(v2)
                            }
                            Adt38::C1(v4) => {
                                if f204(match v4 {
                                    Adt42::C0(h525, _, _, _) => {
                                        h525
                                    }
                                })? {
                                    Ok::<List<Adt38>, R<bool>>(v2)
                                } else {
                                    Err::<List<Adt38>, R<bool>>(Ok(false))
                                }
                            }
                            Adt38::C2(v5) => {
                                if f204(match v5 {
                                    Adt43::C0(h526, _, _, _) => {
                                        h526
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
                                let m528 = match v7 {
                                    Adt45::C0(h527, _, _) => {
                                        h527
                                    }
                                };
                                match m528 {
                                    Adt46::C0(_) => {
                                        Ok::<List<Adt38>, R<bool>>(v2)
                                    }
                                    Adt46::C1(v9) => {
                                        if f204(match v9 {
                                            Adt48::C0(_, _, h529, _) => {
                                                h529
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
                Ok(next) => { m530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f160(v0: Adt35, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    if f205(match v1.clone() {
        Adt49::C0(h532, _) => {
            h532
        }
    }) {
        f206(v1)
    } else {
        let m534 = match v0 {
            Adt35::C0(_, _, _, _, _, h533, _, _) => {
                h533
            }
        };
        match m534 {
            None => {
                f206(v1)
            }
            Some(v2) => {
                f207(v2, v1)
            }
        }
    }
}

pub fn f161(p537: List<Adt99>, p538: List<Str>) -> R<List<Str>> {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v0, v1) = m536;
                let m535 = v0;
                match m535.uncons() {
                    None => {
                        Err::<(List<Adt99>, List<Str>), R<List<Str>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt99>, List<Str>), R<List<Str>>>((v3, List::cons(f208(v2)?, v1)))
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

pub fn f162(p543: u64, p544: List<Adt106>, p545: List<u64>) -> R<Result<List<u64>, Adt93>> {
    let m542 = (p543, (p544, p545));
    {
        let mut m542 = m542;
        loop {
            let step = {
                let (v0, (v1, v2)) = m542;
                let m539 = v0;
                if m539 == 0 {
                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(Adt93::C13)))
                } else {
                    let v3 = m539 - 1;
                    let m540 = v1;
                    match m540.uncons() {
                        None => {
                            Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Ok::<List<u64>, Adt93>(v2)))
                        }
                        Some((v4, v5)) => {
                            let m541 = f209(v4, v5, v2)?;
                            match m541 {
                                Adt132::C0(v6) => {
                                    Err::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>(Ok(Err::<List<u64>, Adt93>(v6)))
                                }
                                Adt132::C1(v7, v8) => {
                                    Ok::<(u64, (List<Adt106>, List<u64>)), R<Result<List<u64>, Adt93>>>((v3, (v7, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m542 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(p548: List<u64>, p549: List<u64>) -> List<u64> {
    let m547 = (p548, p549);
    {
        let mut m547 = m547;
        loop {
            let step = {
                let (v0, v1) = m547;
                let m546 = v0;
                match m546.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
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

pub fn f164(v0: Adt92) -> Adt6 {
    let m550 = v0;
    match m550 {
        Adt92::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt92::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f165(v0: Option<Adt6>) -> List<(Str, Adt6)> {
    let m551 = v0;
    match m551 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("data"), v1), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f166() -> i64 {
    -32700i64
}

pub fn f167() -> i64 {
    -32600i64
}

pub fn f168(v0: Adt95) -> Str {
    let m552 = v0;
    match m552 {
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

pub fn f169(v0: bool, v1: Adt122) -> R<Adt88> {
    let m554 = match v1.clone() {
        Adt122::C0(_, h553) => {
            h553
        }
    };
    match m554 {
        Ok(v2) => {
            let m555 = f210(v2.clone())?;
            match m555 {
                None => {
                    Ok(f6())
                }
                Some(v3) => {
                    Ok(f211(v0, f62(f212(), f213(match v1 {
                        Adt122::C0(h556, _) => {
                            h556
                        }
                    }, v3))?, f214(match v2 {
                        Adt123::C0(_, _, h557) => {
                            h557
                        }
                    }, List::<List<u64>>::nil())?))
                }
            }
        }
        Err(v4) => {
            f62(f215(v4.clone()), f64(Some(match v1 {
                Adt122::C0(h558, _) => {
                    h558
                }
            }), f216(v4)?))
        }
    }
}

pub fn f170(p563: List<Adt86>) -> R<bool> {
    let m562 = p563;
    {
        let mut m562 = m562;
        loop {
            let step = {
                let v0 = m562;
                let m559 = v0;
                match m559.uncons() {
                    None => {
                        Err::<List<Adt86>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f138(match v1.clone() {
                            Adt86::C0(h560, _) => {
                                h560
                            }
                        }, f217())? {
                            if f218(f219(44u64, match v1 {
                                Adt86::C0(_, h561) => {
                                    h561
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
                Ok(next) => { m562 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f171(v0: Adt0, v1: Fn0, v2: u64, v3: Option<Adt107>, v4: List<Str>, v5: Adt103) -> R<Adt122> {
    Ok(Adt122::C0(match v5.clone() {
        Adt103::C0(h564, _, _) => {
            h564
        }
    }, f220(v0, v1, v2, f221(v3, v4, v5)?)?))
}

pub fn f172(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>, v3: Str) -> R<Adt107> {
    let v4: Adt128 = f222(Adt109::C0, f70(v0.clone(), f223())?);
    let v5: Adt128 = f222(Adt109::C1, f70(v0.clone(), f224())?);
    let v6: Adt128 = if f225(v3) {
        f222(Adt109::C2, f226(f70(v0.clone(), f227())?)?)
    } else {
        f228()
    };
    Ok(Adt107::C0(match v4.clone() {
        Adt128::C0(h565, _) => {
            h565
        }
    }, match v5.clone() {
        Adt128::C0(h566, _) => {
            h566
        }
    }, match v6.clone() {
        Adt128::C0(h567, _) => {
            h567
        }
    }, f229(match v4 {
        Adt128::C0(_, h568) => {
            h568
        }
    }, f229(match v5 {
        Adt128::C0(_, h569) => {
            h569
        }
    }, match v6 {
        Adt128::C0(_, h570) => {
            h570
        }
    })), f230(v0, v1, v2)?))
}

pub fn f173(v0: Adt103) -> Option<Str> {
    let a572 = match v0.clone() {
        Adt103::C0(_, h571, _) => {
            h571
        }
    };
    let a573 = Str::lit("tools/call");
    if equal(a572, a573) {
        let m575 = match v0 {
            Adt103::C0(_, _, h574) => {
                h574
            }
        };
        match m575 {
            None => {
                None::<Str>
            }
            Some(v1) => {
                let m576 = f176(Str::lit("name"), v1);
                match m576 {
                    Adt130::C0 => {
                        None::<Str>
                    }
                    Adt130::C1(v2) => {
                        f231(v2)
                    }
                }
            }
        }
    } else {
        None::<Str>
    }
}

pub fn f174() -> List<Str> {
    List::cons(f232(), List::<Str>::nil())
}

pub fn f175(v0: Adt5) -> R<Adt129> {
    let m577 = f176(Str::lit("id"), v0);
    match m577 {
        Adt130::C0 => {
            Ok(Adt129::C0)
        }
        Adt130::C1(v1) => {
            let m578 = v1;
            match m578 {
                Adt6::C0 => {
                    Ok(Adt129::C2)
                }
                Adt6::C1(_) => {
                    Ok(Adt129::C2)
                }
                Adt6::C2(v3) => {
                    let m579 = f233(v3)?;
                    match m579 {
                        None => {
                            Ok(Adt129::C2)
                        }
                        Some(v4) => {
                            Ok(Adt129::C1(Adt92::C1(v4)))
                        }
                    }
                }
                Adt6::C3(v2) => {
                    Ok(Adt129::C1(Adt92::C0(v2)))
                }
                Adt6::C4(_) => {
                    Ok(Adt129::C2)
                }
                Adt6::C5(_) => {
                    Ok(Adt129::C2)
                }
            }
        }
    }
}

pub fn f176(v0: Str, v1: Adt5) -> Adt130 {
    f234(v0, match v1 {
        Adt5::C0(h580) => {
            h580
        }
    })
}

pub fn f177(v0: Adt5) -> Adt95 {
    if f178(v0) {
        Adt95::C8
    } else {
        Adt95::C3
    }
}

pub fn f178(v0: Adt5) -> bool {
    if f235(Str::lit("result"), v0.clone()) {
        true
    } else {
        f235(Str::lit("error"), v0)
    }
}

pub fn f179(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m581 = f176(Str::lit("jsonrpc"), v1.clone());
    match m581 {
        Adt130::C0 => {
            f134(Some(v0), Adt95::C2)
        }
        Adt130::C1(v2) => {
            let m582 = f231(v2);
            match m582 {
                None => {
                    f134(Some(v0), Adt95::C2)
                }
                Some(v3) => {
                    let a583 = v3;
                    let a584 = f125();
                    if equal(a583, a584) {
                        f236(v0, v1)
                    } else {
                        f134(Some(v0), Adt95::C2)
                    }
                }
            }
        }
    }
}

pub fn f180(v0: u64, v1: u64) -> Adt112 {
    let a585 = v1;
    let a586 = 0u64;
    if nat_eq(a585, a586) {
        Adt112::C0
    } else {
        let a587 = v1;
        let a588 = 2u64;
        if nat_eq(a587, a588) {
            Adt112::C2(v0)
        } else {
            Adt112::C1({
                let a592 = v0;
                let a593 = {
                    let a589 = v1;
                    let a590 = 3u64;
                    let a591 = 0u64;
                    nat_quot(a589, a590, a591)
                };
                nat_sub(a592, a593)
            })
        }
    }
}

pub fn f181(p621: Adt113, p622: u64, p623: u64, p624: List<u64>) -> R<u64> {
    let m620 = (p621, (p622, (p623, p624)));
    {
        let mut m620 = m620;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m620;
                let m594 = v3;
                match m594.uncons() {
                    None => {
                        let m595 = v0;
                        match m595 {
                            Adt113::C0 => {
                                Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(0u64))
                            }
                            Adt113::C1 => {
                                Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt113::C2 => {
                                Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                            Adt113::C3 => {
                                Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(2u64))
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m596 = v0;
                        match m596 {
                            Adt113::C0 => {
                                let a597 = v4;
                                let a598 = 128u64;
                                if nat_lt(a597, a598) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let a599 = v4;
                                    let a600 = 194u64;
                                    if nat_lt(a599, a600) {
                                        let v6: u64 = f237({
                                            let a601 = List::cons(v4, v5);
                                            length_list(a601)
                                        })?;
                                        Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v6))
                                    } else {
                                        let a602 = v4;
                                        let a603 = 224u64;
                                        if nat_lt(a602, a603) {
                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                        } else {
                                            let a604 = v4;
                                            let a605 = 224u64;
                                            if nat_eq(a604, a605) {
                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (160u64, (191u64, v5))))
                                            } else {
                                                let a606 = v4;
                                                let a607 = 237u64;
                                                if nat_eq(a606, a607) {
                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (159u64, v5))))
                                                } else {
                                                    let a608 = v4;
                                                    let a609 = 240u64;
                                                    if nat_lt(a608, a609) {
                                                        Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                                    } else {
                                                        let a610 = v4;
                                                        let a611 = 240u64;
                                                        if nat_eq(a610, a611) {
                                                            Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (144u64, (191u64, v5))))
                                                        } else {
                                                            let a612 = v4;
                                                            let a613 = 244u64;
                                                            if nat_eq(a612, a613) {
                                                                Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (143u64, v5))))
                                                            } else {
                                                                let a614 = v4;
                                                                let a615 = 244u64;
                                                                if nat_lt(a614, a615) {
                                                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C3, (128u64, (191u64, v5))))
                                                                } else {
                                                                    let v7: u64 = f237({
                                                                        let a616 = List::cons(v4, v5);
                                                                        length_list(a616)
                                                                    })?;
                                                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v7))
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
                            Adt113::C1 => {
                                if f238(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C0, (0u64, (0u64, v5))))
                                } else {
                                    let v8: u64 = f237({
                                        let a617 = List::cons(v4, v5);
                                        length_list(a617)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v8))
                                }
                            }
                            Adt113::C2 => {
                                if f238(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C1, (128u64, (191u64, v5))))
                                } else {
                                    let v9: u64 = f237({
                                        let a618 = List::cons(v4, v5);
                                        length_list(a618)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v9))
                                }
                            }
                            Adt113::C3 => {
                                if f238(v4, v1, v2) {
                                    Ok::<(Adt113, (u64, (u64, List<u64>))), R<u64>>((Adt113::C2, (128u64, (191u64, v5))))
                                } else {
                                    let v10: u64 = f237({
                                        let a619 = List::cons(v4, v5);
                                        length_list(a619)
                                    })?;
                                    Err::<(Adt113, (u64, (u64, List<u64>))), R<u64>>(Ok(v10))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Adt114, v1: u64) -> R<Result<Adt6, Adt93>> {
    let m626 = match v0.clone() {
        Adt114::C0(h625, _, _) => {
            h625
        }
    };
    match m626 {
        Adt115::C0 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C1 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C2 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C3 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C4 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C5 => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C6(_, _, _) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C7(_, _) => {
            Ok(Err::<Adt6, Adt93>(Adt93::C3))
        }
        Adt115::C8(v3) => {
            let m627 = f239(v3)?;
            match m627 {
                Adt135::C0 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C7(v1)))
                }
                Adt135::C1 => {
                    Ok(Err::<Adt6, Adt93>(Adt93::C8(v1)))
                }
                Adt135::C2(v4) => {
                    let m632 = {
                        let m631 = f240(v4, match v0.clone() {
                            Adt114::C0(_, h629, _) => {
                                h629
                            }
                        }, match v0 {
                            Adt114::C0(_, _, h630) => {
                                h630
                            }
                        });
                        match m631 {
                            Adt114::C0(h628, _, _) => {
                                h628
                            }
                        }
                    };
                    match m632 {
                        Adt115::C0 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C1 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C2 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C3 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C4 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C5 => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C6(_, _, _) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C7(_, _) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C8(_) => {
                            Ok(Err::<Adt6, Adt93>(Adt93::C3))
                        }
                        Adt115::C9(v5) => {
                            Ok(Ok::<Adt6, Adt93>(v5))
                        }
                    }
                }
            }
        }
        Adt115::C9(v2) => {
            Ok(Ok::<Adt6, Adt93>(v2))
        }
    }
}

pub fn f183(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let m634 = match v0.clone() {
        Adt114::C0(h633, _, _) => {
            h633
        }
    };
    match m634 {
        Adt115::C0 => {
            if f241(v2) {
                Ok(Adt131::C1(v0))
            } else {
                f242(v0, v1, v2)
            }
        }
        Adt115::C1 => {
            if f241(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a635 = v2;
                let a636 = 93u64;
                if nat_eq(a635, a636) {
                    Ok(f243(v0, v1))
                } else {
                    f242(v0, v1, v2)
                }
            }
        }
        Adt115::C2 => {
            Ok(f244(v0, v1, v2))
        }
        Adt115::C3 => {
            if f241(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a637 = v2;
                let a638 = 125u64;
                if nat_eq(a637, a638) {
                    Ok(f245(v0, v1))
                } else {
                    let a639 = v2;
                    let a640 = 34u64;
                    if nat_eq(a639, a640) {
                        Ok(f246(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                    } else {
                        Ok(f247(Adt93::C2(v1)))
                    }
                }
            }
        }
        Adt115::C4 => {
            if f241(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a641 = v2;
                let a642 = 34u64;
                if nat_eq(a641, a642) {
                    Ok(f246(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), true)))
                } else {
                    Ok(f247(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C5 => {
            if f241(v2) {
                Ok(Adt131::C1(v0))
            } else {
                let a643 = v2;
                let a644 = 58u64;
                if nat_eq(a643, a644) {
                    Ok(f246(v0, Adt115::C0))
                } else {
                    Ok(f247(Adt93::C2(v1)))
                }
            }
        }
        Adt115::C6(v3, v4, v5) => {
            f248(v0, v1, v2, v3, v4, v5)
        }
        Adt115::C7(v6, v7) => {
            f249(v0, v1, v2, v6, v7)
        }
        Adt115::C8(v8) => {
            f250(v0, v1, v2, v8)
        }
        Adt115::C9(_) => {
            Ok(f244(v0, v1, v2))
        }
    }
}

pub fn f184(v0: u64) -> R<u64> {
    let a645 = 64u64;
    let a646 = v0;
    if if nat_lt(a645, a646) {
        let a647 = v0;
        let a648 = 91u64;
        nat_lt(a647, a648)
    } else {
        false
    } {
        let a649 = v0;
        let a650 = 32u64;
        nat_add(a649, a650)
    } else {
        Ok(v0)
    }
}

pub fn f185(p653: List<u64>) -> List<u64> {
    let m652 = p653;
    {
        let mut m652 = m652;
        loop {
            let step = {
                let v0 = m652;
                let m651 = v0;
                match m651.uncons() {
                    None => {
                        Err::<List<u64>, List<u64>>(List::<u64>::nil())
                    }
                    Some((v1, v2)) => {
                        if f251(v1) {
                            Ok::<List<u64>, List<u64>>(v2)
                        } else {
                            Err::<List<u64>, List<u64>>(List::cons(v1, v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: u64) -> bool {
    let a654 = v0;
    let a655 = 9u64;
    if nat_eq(a654, a655) {
        true
    } else {
        let a656 = 31u64;
        let a657 = v0;
        if nat_lt(a656, a657) {
            let a658 = v0;
            let a659 = 127u64;
            nat_lt(a658, a659)
        } else {
            false
        }
    }
}

pub fn f187(p662: List<u64>) -> bool {
    let m661 = p662;
    {
        let mut m661 = m661;
        loop {
            let step = {
                let v0 = m661;
                let m660 = v0;
                match m660.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f252(v1) {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
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

pub fn f188(p665: List<u64>, v1: bool) -> bool {
    let m664 = p665;
    {
        let mut m664 = m664;
        loop {
            let step = {
                let v0 = m664;
                let m663 = v0;
                match m663.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if if v1 {
                            f253(v2)
                        } else {
                            f254(v2)
                        } {
                            Ok::<List<u64>, bool>(v3)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189() -> u64 {
    64u64
}

pub fn f190(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m666 = f234(Str::lit("type"), v1);
    match m666 {
        Adt130::C0 => {
            Some(Adt91::C3(v0))
        }
        Adt130::C1(v2) => {
            let m667 = f231(v2);
            match m667 {
                None => {
                    Some(Adt91::C3(v0))
                }
                Some(v3) => {
                    let a668 = v3;
                    let a669 = Str::lit("object");
                    if equal(a668, a669) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C3(v0))
                    }
                }
            }
        }
    }
}

pub fn f191(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt91> {
    let m670 = f234(Str::lit("$schema"), v1);
    match m670 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v2) => {
            let m671 = f231(v2);
            match m671 {
                None => {
                    Some(Adt91::C2(v0))
                }
                Some(v3) => {
                    let a672 = v3;
                    let a673 = f255();
                    if equal(a672, a673) {
                        None::<Adt91>
                    } else {
                        Some(Adt91::C2(v0))
                    }
                }
            }
        }
    }
}

pub fn f192(v0: Adt35) -> R<Option<Adt91>> {
    let m675 = match v0.clone() {
        Adt35::C0(_, _, _, _, _, h674, _, _) => {
            h674
        }
    };
    match m675 {
        None => {
            Ok(None::<Adt91>)
        }
        Some(v1) => {
            let m678 = f191(match v0.clone() {
                Adt35::C0(h676, _, _, _, _, _, _, _) => {
                    h676
                }
            }, match v1.clone() {
                Adt5::C0(h677) => {
                    h677
                }
            });
            match m678 {
                None => {
                    let m682 = f152(f153(), match v0 {
                        Adt35::C0(h679, _, _, _, _, _, _, _) => {
                            h679
                        }
                    }, Adt6::C5(match v1.clone() {
                        Adt5::C0(h680) => {
                            h680
                        }
                    }), List::cons(Adt104::C0(Adt6::C5(match v1 {
                        Adt5::C0(h681) => {
                            h681
                        }
                    }), false, List::<Str>::nil()), List::<Adt104>::nil()), f154())?;
                    match m682 {
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

pub fn f193(v0: Str, v1: Adt6, v2: Adt104, v3: Adt120, v4: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m683 = v2;
    match m683 {
        Adt104::C0(v5, v6, v7) => {
            let m684 = f194(v5.clone());
            match m684 {
                None => {
                    let m685 = f195(v5);
                    match m685 {
                        None => {
                            Ok(Err::<(List<Adt104>, Adt120), Adt91>(Adt91::C5(v0, Str::lit("schema"))))
                        }
                        Some(v9) => {
                            f256(v0, v1, match v9 {
                                Adt5::C0(h686) => {
                                    h686
                                }
                            }, v6, v7, v3, v4)
                        }
                    }
                }
                Some(_) => {
                    Ok(Ok::<(List<Adt104>, Adt120), Adt91>((v4, v3)))
                }
            }
        }
    }
}

pub fn f194(v0: Adt6) -> Option<bool> {
    let m687 = v0;
    match m687 {
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

pub fn f195(v0: Adt6) -> Option<Adt5> {
    let m688 = v0;
    match m688 {
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

pub fn f196(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    let m689 = f257(v1.clone(), v2.clone())?;
    match m689 {
        Adt121::C0(v4) => {
            Ok(Adt121::C0(v4))
        }
        Adt121::C1(v5) => {
            Ok(Adt121::C1(v5))
        }
        Adt121::C2(v6) => {
            Ok(Adt121::C2(v6))
        }
        Adt121::C3(_) => {
            f258(v0, v1, v2, v3)
        }
    }
}

pub fn f197(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: u64, v4: List<Adt101>) -> Adt105 {
    let m690 = v0;
    match m690.uncons() {
        None => {
            let a691 = v3;
            let a692 = 1u64;
            if nat_eq(a691, a692) {
                Adt105::C0(Adt100::C0, v4)
            } else {
                Adt105::C0(Adt100::C1(v2, Str::lit("oneOf")), v4)
            }
        }
        Some((v5, v6)) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v5, v1.clone(), v2.clone()), List::cons(Adt101::C2(v6, v1, v2, v3), v4)))
        }
    }
}

pub fn f198(v0: Option<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m693 = v0;
    match m693 {
        None => {
            Adt105::C0(Adt100::C0, v3)
        }
        Some(v4) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1, v2), v3))
        }
    }
}

pub fn f199(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m694 = f234(Str::lit("anyOf"), v0);
    match m694 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m695 = f259(v3);
            match m695 {
                None => {
                    Adt121::C1(Str::lit("anyOf"))
                }
                Some(v4) => {
                    let m696 = v4;
                    match m696.uncons() {
                        None => {
                            Adt121::C1(Str::lit("anyOf"))
                        }
                        Some((v5, v6)) => {
                            Adt121::C3(List::cons(Adt101::C0(v5, v1.clone(), v2.clone()), List::cons(Adt101::C1(v6, v1, v2), List::<Adt101>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f200(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m697 = f234(Str::lit("oneOf"), v0);
    match m697 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m698 = f259(v3);
            match m698 {
                None => {
                    Adt121::C1(Str::lit("oneOf"))
                }
                Some(v4) => {
                    let m699 = v4;
                    match m699.uncons() {
                        None => {
                            Adt121::C1(Str::lit("oneOf"))
                        }
                        Some((v5, v6)) => {
                            Adt121::C3(List::cons(Adt101::C0(v5, v1.clone(), v2.clone()), List::cons(Adt101::C2(v6, v1, v2, 0u64), List::<Adt101>::nil())))
                        }
                    }
                }
            }
        }
    }
}

pub fn f201(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m700 = f234(Str::lit("not"), v0);
    match m700 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1, v2.clone()), List::cons(Adt101::C3(v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f202(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m701 = f234(Str::lit("if"), v0.clone());
    match m701 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            Adt121::C3(List::cons(Adt101::C0(v3, v1.clone(), v2.clone()), List::cons(Adt101::C4(f260(Str::lit("then"), v0.clone()), f260(Str::lit("else"), v0), v1, v2), List::<Adt101>::nil())))
        }
    }
}

pub fn f203(v0: List<Adt6>, v1: Adt6, v2: List<Adt99>, v3: List<Adt101>) -> Adt105 {
    let m702 = v0;
    match m702.uncons() {
        None => {
            Adt105::C0(Adt100::C1(v2, Str::lit("anyOf")), v3)
        }
        Some((v4, v5)) => {
            Adt105::C0(Adt100::C0, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), List::cons(Adt101::C1(v5, v1, v2), v3)))
        }
    }
}

pub fn f204(v0: Str) -> R<bool> {
    let m704 = f261(f87({
        let a703 = v0;
        utf8_encode(a703)
    })?)?;
    match m704 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f205(v0: Adt37) -> bool {
    let m706 = match v0 {
        Adt37::C0(_, _, h705) => {
            h705
        }
    };
    match m706 {
        None => {
            false
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f206(v0: Adt49) -> R<Result<Adt32, Adt84>> {
    let m708 = match v0.clone() {
        Adt49::C0(_, h707) => {
            h707
        }
    };
    match m708.uncons() {
        None => {
            Ok(Ok::<Adt32, Adt84>(Adt32::C2(match v0 {
                Adt49::C0(h709, _) => {
                    h709
                }
            })))
        }
        Some((_, _)) => {
            if f262(match v0.clone() {
                Adt49::C0(_, h710) => {
                    h710
                }
            })? {
                Ok(Ok::<Adt32, Adt84>(Adt32::C3(v0)))
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("progress did not increase"))))
            }
        }
    }
}

pub fn f207(v0: Adt5, v1: Adt49) -> R<Result<Adt32, Adt84>> {
    let m714 = {
        let m713 = match v1.clone() {
            Adt49::C0(h712, _) => {
                h712
            }
        };
        match m713 {
            Adt37::C0(_, h711, _) => {
                h711
            }
        }
    };
    match m714 {
        None => {
            Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("the tool declares an outputSchema and returned no structuredContent"))))
        }
        Some(v2) => {
            let m717 = f80(Adt6::C5(match v0.clone() {
                Adt5::C0(h715) => {
                    h715
                }
            }), Adt6::C5(match v0 {
                Adt5::C0(h716) => {
                    h716
                }
            }), v2)?;
            match m717 {
                Adt98::C0 => {
                    f206(v1)
                }
                Adt98::C1(_, _) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C6(Str::lit("structuredContent does not conform to the outputSchema"))))
                }
                Adt98::C2(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema is malformed"))))
                }
                Adt98::C3(_) => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema has an unresolved reference"))))
                }
                Adt98::C4 => {
                    Ok(Err::<Adt32, Adt84>(Adt84::C9(Str::lit("the outputSchema exceeded the validation budget"))))
                }
            }
        }
    }
}

pub fn f208(v0: Adt99) -> R<Str> {
    let m718 = v0;
    match m718 {
        Adt99::C0(v1) => {
            f263(v1)
        }
        Adt99::C1(v2) => {
            let a719 = f79(v2, 0i64)?;
            Ok(format_i64(a719))
        }
    }
}

pub fn f209(v0: Adt106, v1: List<Adt106>, v2: List<u64>) -> R<Adt132> {
    let m720 = v0;
    match m720 {
        Adt106::C0(v3) => {
            Ok(Adt132::C1(v1, f163(v3, v2)))
        }
        Adt106::C1(v4, v5) => {
            f264(v4, v5, v1, v2)
        }
        Adt106::C2(v6, v7) => {
            let m721 = v7;
            match m721.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((v8, v9)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C1(v6, v8), List::cons(Adt106::C3(v6, v9), v1)), v2))
                }
            }
        }
        Adt106::C3(v10, v11) => {
            let m722 = v11.clone();
            match m722.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(93u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C2(v10, v11), v1)), v2))
                }
            }
        }
        Adt106::C4(v14, v15) => {
            let m723 = v15;
            match m723.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((v16, v17)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0({
                        let a725 = f265({
                            let (h724, _) = v16;
                            h724
                        })?;
                        let a726 = List::cons(58u64, List::<u64>::nil());
                        append_list(a725, a726)
                    }), List::cons(Adt106::C1(v14, {
                        let (_, h727) = v16;
                        h727
                    }), List::cons(Adt106::C5(v14, v17), v1))), v2))
                }
            }
        }
        Adt106::C5(v18, v19) => {
            let m728 = v19.clone();
            match m728.uncons() {
                None => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(125u64, List::<u64>::nil())), v1), v2))
                }
                Some((_, _)) => {
                    Ok(Adt132::C1(List::cons(Adt106::C0(List::cons(44u64, List::<u64>::nil())), List::cons(Adt106::C4(v18, v19), v1)), v2))
                }
            }
        }
    }
}

pub fn f210(v0: Adt123) -> R<Option<Adt5>> {
    let m730 = f266(match v0.clone() {
        Adt123::C0(h729, _, _) => {
            h729
        }
    })?;
    match m730 {
        None => {
            Ok(None::<Adt5>)
        }
        Some(v1) => {
            Ok(Some(Adt5::C0({
                let a733 = v1;
                let a734 = {
                    let m732 = f267(v0);
                    match m732 {
                        Adt5::C0(h731) => {
                            h731
                        }
                    }
                };
                append_list(a733, a734)
            })))
        }
    }
}

pub fn f211(v0: bool, v1: Adt88, v2: Option<List<List<u64>>>) -> Adt88 {
    let m735 = v2;
    match m735 {
        None => {
            f6()
        }
        Some(v3) => {
            f268(v0, v1, f38(v3, List::<List<u64>>::nil()))
        }
    }
}

pub fn f212() -> u64 {
    200u64
}

pub fn f213(v0: Adt92, v1: Adt5) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), List::cons((Str::lit("id"), f164(v0)), List::cons((Str::lit("result"), Adt6::C5(match v1 {
        Adt5::C0(h736) => {
            h736
        }
    })), List::<(Str, Adt6)>::nil()))))
}

pub fn f214(p740: List<Adt125>, p741: List<List<u64>>) -> R<Option<List<List<u64>>>> {
    let m739 = (p740, p741);
    {
        let mut m739 = m739;
        loop {
            let step = {
                let (v0, v1) = m739;
                let m737 = v0;
                match m737.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m738 = f269(v2)?;
                        match m738 {
                            None => {
                                Err::<(List<Adt125>, List<List<u64>>), R<Option<List<List<u64>>>>>(Ok(None::<List<List<u64>>>))
                            }
                            Some(v4) => {
                                Ok::<(List<Adt125>, List<List<u64>>), R<Option<List<List<u64>>>>>((v3, List::cons(v4, v1)))
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

pub fn f215(v0: Adt126) -> u64 {
    let m742 = v0;
    match m742 {
        Adt126::C0(_) => {
            f96()
        }
        Adt126::C1(_, _) => {
            f96()
        }
        Adt126::C2(_) => {
            f96()
        }
        Adt126::C3(_) => {
            f270()
        }
        Adt126::C4(v6) => {
            f271(v6)
        }
    }
}

pub fn f216(v0: Adt126) -> R<Adt80> {
    Ok(Adt80::C0(f272(v0.clone()), f273(v0.clone()), f274(v0)?))
}

pub fn f217() -> List<u64> {
    List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(112u64, List::cons(116u64, List::<u64>::nil()))))))
}

pub fn f218(p745: List<List<u64>>) -> R<bool> {
    let m744 = p745;
    {
        let mut m744 = m744;
        loop {
            let step = {
                let v0 = m744;
                let m743 = v0;
                match m743.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f275(v1)? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m744 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f219(v0: u64, p750: List<u64>, p751: List<u64>, p752: List<List<u64>>) -> List<List<u64>> {
    let m749 = (p750, (p751, p752));
    {
        let mut m749 = m749;
        loop {
            let step = {
                let (v1, (v2, v3)) = m749;
                let m746 = v1;
                match m746.uncons() {
                    None => {
                        Err::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>(f38(List::cons(f148(v2, List::<u64>::nil()), v3), List::<List<u64>>::nil()))
                    }
                    Some((v4, v5)) => {
                        let a747 = v4;
                        let a748 = v0;
                        if nat_eq(a747, a748) {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::<u64>::nil(), List::cons(f148(v2, List::<u64>::nil()), v3))))
                        } else {
                            Ok::<(List<u64>, (List<u64>, List<List<u64>>)), List<List<u64>>>((v5, (List::cons(v4, v2), v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m749 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt133, Adt126>) -> R<Result<Adt123, Adt126>> {
    let m753 = v3;
    match m753 {
        Ok(v5) => {
            f276(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(v4))
        }
    }
}

pub fn f221(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Result<Adt133, Adt126>> {
    Ok(f277(f278(match v2.clone() {
        Adt103::C0(_, h754, _) => {
            h754
        }
    }), f279(v0, v1, v2)?))
}

pub fn f222(v0: Adt109, v1: Adt96) -> Adt128 {
    let m755 = v1;
    match m755 {
        Adt96::C0 => {
            f228()
        }
        Adt96::C1 => {
            Adt128::C0(None::<Str>, Some(Adt108::C6(v0)))
        }
        Adt96::C2 => {
            Adt128::C0(None::<Str>, Some(Adt108::C7(v0)))
        }
        Adt96::C3(v2) => {
            let m756 = f280(v2);
            match m756 {
                None => {
                    Adt128::C0(None::<Str>, Some(Adt108::C6(v0)))
                }
                Some(v3) => {
                    Adt128::C0(Some(v3), None::<Adt108>)
                }
            }
        }
    }
}

pub fn f223() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(114u64, List::cons(111u64, List::cons(116u64, List::cons(111u64, List::cons(99u64, List::cons(111u64, List::cons(108u64, List::cons(45u64, List::cons(118u64, List::cons(101u64, List::cons(114u64, List::cons(115u64, List::cons(105u64, List::cons(111u64, List::cons(110u64, List::<u64>::nil()))))))))))))))))))))
}

pub fn f224() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(109u64, List::cons(101u64, List::cons(116u64, List::cons(104u64, List::cons(111u64, List::cons(100u64, List::<u64>::nil()))))))))))
}

pub fn f225(v0: Str) -> bool {
    let m757 = f281(v0);
    match m757 {
        Adt140::C0 => {
            false
        }
        Adt140::C1 => {
            true
        }
        Adt140::C2 => {
            true
        }
        Adt140::C3 => {
            true
        }
    }
}

pub fn f226(v0: Adt96) -> R<Adt96> {
    let m758 = v0.clone();
    match m758 {
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
            let m759 = f282(v1);
            match m759 {
                None => {
                    Ok(v0)
                }
                Some(v2) => {
                    let m760 = f261(v2)?;
                    match m760 {
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

pub fn f227() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(110u64, List::cons(97u64, List::cons(109u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f228() -> Adt128 {
    Adt128::C0(None::<Str>, None::<Adt108>)
}

pub fn f229(v0: Option<Adt108>, v1: Option<Adt108>) -> Option<Adt108> {
    let m761 = v0.clone();
    match m761 {
        None => {
            v1
        }
        Some(_) => {
            v0
        }
    }
}

pub fn f230(v0: List<Adt86>, v1: List<Adt12>, v2: Option<Str>) -> R<List<Adt110>> {
    Ok(f283(f284(v0, v1, v2, List::<Adt110>::nil())?, List::<Adt110>::nil()))
}

pub fn f231(v0: Adt6) -> Option<Str> {
    let m762 = v0;
    match m762 {
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

pub fn f232() -> Str {
    Str::lit("2026-07-28")
}

pub fn f233(v0: Adt7) -> R<Option<i64>> {
    let m765 = f285(40u64, match v0.clone() {
        Adt7::C0(h763, _) => {
            h763
        }
    }, match v0 {
        Adt7::C0(_, h764) => {
            h764
        }
    })?;
    match m765 {
        None => {
            Ok(None::<i64>)
        }
        Some(v1) => {
            Ok(f286(v1))
        }
    }
}

pub fn f234(v0: Str, p771: List<(Str, Adt6)>) -> Adt130 {
    let m770 = p771;
    {
        let mut m770 = m770;
        loop {
            let step = {
                let v1 = m770;
                let m766 = v1;
                match m766.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m768 = f287({
                            let (h767, _) = v2;
                            h767
                        }, v0.clone());
                        match m768 {
                            Adt141::C0 => {
                                Err::<List<(Str, Adt6)>, Adt130>(Adt130::C1({
                                    let (_, h769) = v2;
                                    h769
                                }))
                            }
                            Adt141::C1 => {
                                Ok::<List<(Str, Adt6)>, Adt130>(v3)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: Str, v1: Adt5) -> bool {
    f288(f176(v0, v1))
}

pub fn f236(v0: Adt92, v1: Adt5) -> Result<Adt103, Adt94> {
    let m772 = f176(Str::lit("method"), v1.clone());
    match m772 {
        Adt130::C0 => {
            f134(Some(v0), f177(v1))
        }
        Adt130::C1(v2) => {
            let m773 = f231(v2);
            match m773 {
                None => {
                    f134(Some(v0), Adt95::C4)
                }
                Some(v3) => {
                    f289(v0, v3, v1)
                }
            }
        }
    }
}

pub fn f237(v0: u64) -> R<u64> {
    let a776 = {
        let a774 = v0;
        let a775 = 3u64;
        nat_mul(a774, a775)?
    };
    let a777 = 1u64;
    nat_add(a776, a777)
}

pub fn f238(v0: u64, v1: u64, v2: u64) -> bool {
    let a778 = v1;
    let a779 = v0;
    if nat_le(a778, a779) {
        let a780 = v0;
        let a781 = v2;
        nat_le(a780, a781)
    } else {
        false
    }
}

pub fn f239(v0: Adt117) -> R<Adt135> {
    if f290(match v0.clone() {
        Adt117::C0(_, h782, _, _, _, _, _) => {
            h782
        }
    }) {
        let v1: i64 = {
            let a788 = if match v0.clone() {
                Adt117::C0(_, _, _, _, _, h783, _) => {
                    h783
                }
            } {
                let a785 = match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h784) => {
                        h784
                    }
                };
                int_neg(a785)?
            } else {
                match v0.clone() {
                    Adt117::C0(_, _, _, _, _, _, h786) => {
                        h786
                    }
                }
            };
            let a789 = match v0.clone() {
                Adt117::C0(_, _, _, _, h787, _, _) => {
                    h787
                }
            };
            int_sub(a788, a789)?
        };
        let m792 = {
            let a790 = v1;
            let a791 = -9999i64;
            compare(a790, a791)
        };
        if if match m792 {
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
            let m795 = {
                let a793 = 9999i64;
                let a794 = v1;
                compare(a793, a794)
            };
            match m795 {
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
                Adt117::C0(h796, _, _, _, _, _, _) => {
                    h796
                }
            } {
                let a798 = match v0 {
                    Adt117::C0(_, _, h797, _, _, _, _) => {
                        h797
                    }
                };
                int_neg(a798)?
            } else {
                match v0 {
                    Adt117::C0(_, _, h799, _, _, _, _) => {
                        h799
                    }
                }
            }, v1))))
        }
    } else {
        Ok(Adt135::C0)
    }
}

pub fn f240(v0: Adt6, v1: List<Adt119>, v2: u64) -> Adt114 {
    let m800 = v1.clone();
    match m800.uncons() {
        None => {
            Adt114::C0(Adt115::C9(v0), List::<Adt119>::nil(), 0u64)
        }
        Some((v3, v4)) => {
            let m801 = v3;
            match m801 {
                Adt119::C0(v5) => {
                    Adt114::C0(Adt115::C2, List::cons(Adt119::C0(List::cons(v0, v5)), v4), v2)
                }
                Adt119::C1(v6, v7) => {
                    let m802 = v7;
                    match m802 {
                        None => {
                            Adt114::C0(Adt115::C2, v1, v2)
                        }
                        Some(v8) => {
                            Adt114::C0(Adt115::C2, List::cons(Adt119::C1(List::cons((v8, v0), v6), None::<Str>), v4), v2)
                        }
                    }
                }
            }
        }
    }
}

pub fn f241(v0: u64) -> bool {
    let a803 = v0;
    let a804 = 32u64;
    if nat_eq(a803, a804) {
        true
    } else {
        let a805 = v0;
        let a806 = 10u64;
        if nat_eq(a805, a806) {
            true
        } else {
            let a807 = v0;
            let a808 = 13u64;
            if nat_eq(a807, a808) {
                true
            } else {
                let a809 = v0;
                let a810 = 9u64;
                nat_eq(a809, a810)
            }
        }
    }
}

pub fn f242(v0: Adt114, v1: u64, v2: u64) -> R<Adt131> {
    let a811 = v2;
    let a812 = 34u64;
    if nat_eq(a811, a812) {
        Ok(f246(v0, Adt115::C6(Adt116::C0, List::<u64>::nil(), false)))
    } else {
        let a813 = v2;
        let a814 = 123u64;
        if nat_eq(a813, a814) {
            f291(v0, Adt119::C1(List::<(Str, Adt6)>::nil(), None::<Str>), Adt115::C3, v1)
        } else {
            let a815 = v2;
            let a816 = 91u64;
            if nat_eq(a815, a816) {
                f291(v0, Adt119::C0(List::<Adt6>::nil()), Adt115::C1, v1)
            } else {
                let a817 = v2;
                let a818 = 116u64;
                if nat_eq(a817, a818) {
                    Ok(f246(v0, Adt115::C7(0u64, 1u64)))
                } else {
                    let a819 = v2;
                    let a820 = 102u64;
                    if nat_eq(a819, a820) {
                        Ok(f246(v0, Adt115::C7(1u64, 1u64)))
                    } else {
                        let a821 = v2;
                        let a822 = 110u64;
                        if nat_eq(a821, a822) {
                            Ok(f246(v0, Adt115::C7(2u64, 1u64)))
                        } else {
                            let a823 = v2;
                            let a824 = 45u64;
                            if nat_eq(a823, a824) {
                                Ok(f246(v0, f292(true, Adt118::C0, 0i64, 0u64, 0i64, false, 0i64)))
                            } else {
                                let a825 = v2;
                                let a826 = 48u64;
                                if nat_eq(a825, a826) {
                                    Ok(f246(v0, f292(false, Adt118::C1, 0i64, 0u64, 0i64, false, 0i64)))
                                } else {
                                    if f238(v2, 49u64, 57u64) {
                                        Ok(f246(v0, f292(false, Adt118::C2, f293({
                                            let a827 = v2;
                                            let a828 = 48u64;
                                            nat_sub(a827, a828)
                                        }), 1u64, 0i64, false, 0i64)))
                                    } else {
                                        Ok(f247(Adt93::C2(v1)))
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

pub fn f243(v0: Adt114, v1: u64) -> Adt131 {
    let m830 = match v0.clone() {
        Adt114::C0(_, h829, _) => {
            h829
        }
    };
    match m830.uncons() {
        None => {
            f247(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m831 = v2;
            match m831 {
                Adt119::C0(v4) => {
                    Adt131::C1(f240(Adt6::C4(f294(v4, List::<Adt6>::nil())), v3, {
                        let a833 = match v0 {
                            Adt114::C0(_, _, h832) => {
                                h832
                            }
                        };
                        let a834 = 1u64;
                        nat_sub(a833, a834)
                    }))
                }
                Adt119::C1(_, _) => {
                    f247(Adt93::C2(v1))
                }
            }
        }
    }
}

pub fn f244(v0: Adt114, v1: u64, v2: u64) -> Adt131 {
    let m836 = match v0.clone() {
        Adt114::C0(h835, _, _) => {
            h835
        }
    };
    match m836 {
        Adt115::C0 => {
            f247(Adt93::C2(v1))
        }
        Adt115::C1 => {
            f247(Adt93::C2(v1))
        }
        Adt115::C2 => {
            if f241(v2) {
                Adt131::C1(v0)
            } else {
                let a837 = v2;
                let a838 = 44u64;
                if nat_eq(a837, a838) {
                    f295(v0, v1)
                } else {
                    let a839 = v2;
                    let a840 = 93u64;
                    if nat_eq(a839, a840) {
                        f243(v0, v1)
                    } else {
                        let a841 = v2;
                        let a842 = 125u64;
                        if nat_eq(a841, a842) {
                            f245(v0, v1)
                        } else {
                            f247(Adt93::C2(v1))
                        }
                    }
                }
            }
        }
        Adt115::C3 => {
            f247(Adt93::C2(v1))
        }
        Adt115::C4 => {
            f247(Adt93::C2(v1))
        }
        Adt115::C5 => {
            f247(Adt93::C2(v1))
        }
        Adt115::C6(_, _, _) => {
            f247(Adt93::C2(v1))
        }
        Adt115::C7(_, _) => {
            f247(Adt93::C2(v1))
        }
        Adt115::C8(_) => {
            f247(Adt93::C2(v1))
        }
        Adt115::C9(_) => {
            if f241(v2) {
                Adt131::C1(v0)
            } else {
                f247(Adt93::C10(v1))
            }
        }
    }
}

pub fn f245(v0: Adt114, v1: u64) -> Adt131 {
    let m844 = match v0.clone() {
        Adt114::C0(_, h843, _) => {
            h843
        }
    };
    match m844.uncons() {
        None => {
            f247(Adt93::C2(v1))
        }
        Some((v2, v3)) => {
            let m845 = v2;
            match m845 {
                Adt119::C0(_) => {
                    f247(Adt93::C2(v1))
                }
                Adt119::C1(v5, _) => {
                    Adt131::C1(f240(Adt6::C5(f296(v5, List::<(Str, Adt6)>::nil())), v3, {
                        let a847 = match v0 {
                            Adt114::C0(_, _, h846) => {
                                h846
                            }
                        };
                        let a848 = 1u64;
                        nat_sub(a847, a848)
                    }))
                }
            }
        }
    }
}

pub fn f246(v0: Adt114, v1: Adt115) -> Adt131 {
    Adt131::C1(f297(v0, v1))
}

pub fn f247(v0: Adt93) -> Adt131 {
    Adt131::C0(v0)
}

pub fn f248(v0: Adt114, v1: u64, v2: u64, v3: Adt116, v4: List<u64>, v5: bool) -> R<Adt131> {
    let m849 = v3;
    match m849 {
        Adt116::C0 => {
            let a850 = v2;
            let a851 = 34u64;
            if nat_eq(a850, a851) {
                Ok(f298(v0, v1, v4, v5))
            } else {
                let a852 = v2;
                let a853 = 92u64;
                if nat_eq(a852, a853) {
                    Ok(f246(v0, Adt115::C6(Adt116::C1, v4, v5)))
                } else {
                    let a854 = v2;
                    let a855 = 32u64;
                    if nat_lt(a854, a855) {
                        Ok(f247(Adt93::C4(v1)))
                    } else {
                        Ok(f246(v0, Adt115::C6(Adt116::C0, List::cons(v2, v4), v5)))
                    }
                }
            }
        }
        Adt116::C1 => {
            let a856 = v2;
            let a857 = 117u64;
            if nat_eq(a856, a857) {
                Ok(f246(v0, Adt115::C6(Adt116::C2(0u64, 0u64, 0u64), v4, v5)))
            } else {
                let a858 = f299(v2);
                let a859 = 0u64;
                if nat_eq(a858, a859) {
                    Ok(f247(Adt93::C5(v1)))
                } else {
                    Ok(f246(v0, Adt115::C6(Adt116::C0, List::cons(f299(v2), v4), v5)))
                }
            }
        }
        Adt116::C2(v6, v7, v8) => {
            let a860 = 15u64;
            let a861 = f300(v2);
            if nat_lt(a860, a861) {
                Ok(f247(Adt93::C5(v1)))
            } else {
                let a864 = {
                    let a862 = v6;
                    let a863 = 1u64;
                    nat_add(a862, a863)?
                };
                let a865 = 4u64;
                if nat_eq(a864, a865) {
                    f301(v0, v1, {
                        let a868 = {
                            let a866 = v7;
                            let a867 = 16u64;
                            nat_mul(a866, a867)?
                        };
                        let a869 = f300(v2);
                        nat_add(a868, a869)?
                    }, v8, v4, v5)
                } else {
                    Ok(f246(v0, Adt115::C6(Adt116::C2({
                        let a870 = v6;
                        let a871 = 1u64;
                        nat_add(a870, a871)?
                    }, {
                        let a874 = {
                            let a872 = v7;
                            let a873 = 16u64;
                            nat_mul(a872, a873)?
                        };
                        let a875 = f300(v2);
                        nat_add(a874, a875)?
                    }, v8), v4, v5)))
                }
            }
        }
        Adt116::C3(v9) => {
            let a876 = v2;
            let a877 = 92u64;
            if nat_eq(a876, a877) {
                Ok(f246(v0, Adt115::C6(Adt116::C4(v9), v4, v5)))
            } else {
                Ok(f247(Adt93::C6(v1)))
            }
        }
        Adt116::C4(v10) => {
            let a878 = v2;
            let a879 = 117u64;
            if nat_eq(a878, a879) {
                Ok(f246(v0, Adt115::C6(Adt116::C2(0u64, 0u64, v10), v4, v5)))
            } else {
                Ok(f247(Adt93::C6(v1)))
            }
        }
    }
}

pub fn f249(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt131> {
    let a880 = v2;
    let a881 = f302(v3, v4);
    if nat_eq(a880, a881) {
        let a885 = {
            let a882 = v4;
            let a883 = 1u64;
            nat_add(a882, a883)?
        };
        let a886 = {
            let a884 = f303(v3);
            length_list(a884)
        };
        if nat_eq(a885, a886) {
            Ok(f304(v0, f305(v3)))
        } else {
            Ok(f246(v0, Adt115::C7(v3, {
                let a887 = v4;
                let a888 = 1u64;
                nat_add(a887, a888)?
            })))
        }
    } else {
        Ok(f247(Adt93::C2(v1)))
    }
}

pub fn f250(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m890 = match v3.clone() {
        Adt117::C0(_, h889, _, _, _, _, _) => {
            h889
        }
    };
    match m890 {
        Adt118::C0 => {
            let a891 = v2;
            let a892 = 48u64;
            if nat_eq(a891, a892) {
                Ok(f306(v0, v3, Adt118::C1))
            } else {
                if f238(v2, 49u64, 57u64) {
                    f307(v0, v1, v2, v3, false)
                } else {
                    Ok(f247(Adt93::C7(v1)))
                }
            }
        }
        Adt118::C1 => {
            let a893 = v2;
            let a894 = 46u64;
            if nat_eq(a893, a894) {
                Ok(f306(v0, v3, Adt118::C3))
            } else {
                if f308(v2) {
                    Ok(f306(v0, v3, Adt118::C5))
                } else {
                    if f309(v2) {
                        Ok(f247(Adt93::C7(v1)))
                    } else {
                        f310(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C2 => {
            if f309(v2) {
                f307(v0, v1, v2, v3, false)
            } else {
                let a895 = v2;
                let a896 = 46u64;
                if nat_eq(a895, a896) {
                    Ok(f306(v0, v3, Adt118::C3))
                } else {
                    if f308(v2) {
                        Ok(f306(v0, v3, Adt118::C5))
                    } else {
                        f310(v0, v1, v2, v3)
                    }
                }
            }
        }
        Adt118::C3 => {
            if f309(v2) {
                f307(v0, v1, v2, v3, true)
            } else {
                Ok(f247(Adt93::C7(v1)))
            }
        }
        Adt118::C4 => {
            if f309(v2) {
                f307(v0, v1, v2, v3, true)
            } else {
                if f308(v2) {
                    Ok(f306(v0, v3, Adt118::C5))
                } else {
                    f310(v0, v1, v2, v3)
                }
            }
        }
        Adt118::C5 => {
            let a897 = v2;
            let a898 = 43u64;
            if nat_eq(a897, a898) {
                Ok(f246(v0, f292(match v3.clone() {
                    Adt117::C0(h899, _, _, _, _, _, _) => {
                        h899
                    }
                }, Adt118::C6, match v3.clone() {
                    Adt117::C0(_, _, h900, _, _, _, _) => {
                        h900
                    }
                }, match v3.clone() {
                    Adt117::C0(_, _, _, h901, _, _, _) => {
                        h901
                    }
                }, match v3 {
                    Adt117::C0(_, _, _, _, h902, _, _) => {
                        h902
                    }
                }, false, 0i64)))
            } else {
                let a903 = v2;
                let a904 = 45u64;
                if nat_eq(a903, a904) {
                    Ok(f246(v0, f292(match v3.clone() {
                        Adt117::C0(h905, _, _, _, _, _, _) => {
                            h905
                        }
                    }, Adt118::C6, match v3.clone() {
                        Adt117::C0(_, _, h906, _, _, _, _) => {
                            h906
                        }
                    }, match v3.clone() {
                        Adt117::C0(_, _, _, h907, _, _, _) => {
                            h907
                        }
                    }, match v3 {
                        Adt117::C0(_, _, _, _, h908, _, _) => {
                            h908
                        }
                    }, true, 0i64)))
                } else {
                    if f309(v2) {
                        f311(v0, v1, v2, Adt117::C0(match v3.clone() {
                            Adt117::C0(h909, _, _, _, _, _, _) => {
                                h909
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, h910, _, _, _, _, _) => {
                                h910
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, h911, _, _, _, _) => {
                                h911
                            }
                        }, match v3.clone() {
                            Adt117::C0(_, _, _, h912, _, _, _) => {
                                h912
                            }
                        }, match v3 {
                            Adt117::C0(_, _, _, _, h913, _, _) => {
                                h913
                            }
                        }, false, 0i64), false)
                    } else {
                        Ok(f247(Adt93::C7(v1)))
                    }
                }
            }
        }
        Adt118::C6 => {
            if f309(v2) {
                f311(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h914, _) => {
                        h914
                    }
                })
            } else {
                Ok(f247(Adt93::C7(v1)))
            }
        }
        Adt118::C7 => {
            if f309(v2) {
                f311(v0, v1, v2, v3.clone(), match v3 {
                    Adt117::C0(_, _, _, _, _, h915, _) => {
                        h915
                    }
                })
            } else {
                f310(v0, v1, v2, v3)
            }
        }
    }
}

pub fn f251(v0: u64) -> bool {
    let a916 = v0;
    let a917 = 9u64;
    if nat_eq(a916, a917) {
        true
    } else {
        let a918 = v0;
        let a919 = 32u64;
        nat_eq(a918, a919)
    }
}

pub fn f252(v0: u64) -> bool {
    let a920 = 47u64;
    let a921 = v0;
    if nat_lt(a920, a921) {
        let a922 = v0;
        let a923 = 58u64;
        nat_lt(a922, a923)
    } else {
        false
    }
}

pub fn f253(v0: u64) -> bool {
    let a924 = 48u64;
    let a925 = v0;
    if if nat_le(a924, a925) {
        let a926 = v0;
        let a927 = 57u64;
        nat_le(a926, a927)
    } else {
        false
    } {
        true
    } else {
        let a928 = 65u64;
        let a929 = v0;
        if if nat_le(a928, a929) {
            let a930 = v0;
            let a931 = 90u64;
            nat_le(a930, a931)
        } else {
            false
        } {
            true
        } else {
            let a932 = 97u64;
            let a933 = v0;
            if if nat_le(a932, a933) {
                let a934 = v0;
                let a935 = 122u64;
                nat_le(a934, a935)
            } else {
                false
            } {
                true
            } else {
                let a936 = v0;
                let a937 = 33u64;
                if nat_eq(a936, a937) {
                    true
                } else {
                    let a938 = v0;
                    let a939 = 35u64;
                    if nat_eq(a938, a939) {
                        true
                    } else {
                        let a940 = v0;
                        let a941 = 36u64;
                        if nat_eq(a940, a941) {
                            true
                        } else {
                            let a942 = v0;
                            let a943 = 37u64;
                            if nat_eq(a942, a943) {
                                true
                            } else {
                                let a944 = v0;
                                let a945 = 38u64;
                                if nat_eq(a944, a945) {
                                    true
                                } else {
                                    let a946 = v0;
                                    let a947 = 39u64;
                                    if nat_eq(a946, a947) {
                                        true
                                    } else {
                                        let a948 = v0;
                                        let a949 = 42u64;
                                        if nat_eq(a948, a949) {
                                            true
                                        } else {
                                            let a950 = v0;
                                            let a951 = 43u64;
                                            if nat_eq(a950, a951) {
                                                true
                                            } else {
                                                let a952 = v0;
                                                let a953 = 45u64;
                                                if nat_eq(a952, a953) {
                                                    true
                                                } else {
                                                    let a954 = v0;
                                                    let a955 = 46u64;
                                                    if nat_eq(a954, a955) {
                                                        true
                                                    } else {
                                                        let a956 = v0;
                                                        let a957 = 94u64;
                                                        if nat_eq(a956, a957) {
                                                            true
                                                        } else {
                                                            let a958 = v0;
                                                            let a959 = 95u64;
                                                            if nat_eq(a958, a959) {
                                                                true
                                                            } else {
                                                                let a960 = v0;
                                                                let a961 = 96u64;
                                                                if nat_eq(a960, a961) {
                                                                    true
                                                                } else {
                                                                    let a962 = v0;
                                                                    let a963 = 124u64;
                                                                    if nat_eq(a962, a963) {
                                                                        true
                                                                    } else {
                                                                        let a964 = v0;
                                                                        let a965 = 126u64;
                                                                        nat_eq(a964, a965)
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

pub fn f254(v0: u64) -> bool {
    let a966 = 48u64;
    let a967 = v0;
    if if nat_le(a966, a967) {
        let a968 = v0;
        let a969 = 57u64;
        nat_le(a968, a969)
    } else {
        false
    } {
        true
    } else {
        let a970 = 65u64;
        let a971 = v0;
        if if nat_le(a970, a971) {
            let a972 = v0;
            let a973 = 90u64;
            nat_le(a972, a973)
        } else {
            false
        } {
            true
        } else {
            let a974 = 97u64;
            let a975 = v0;
            if if nat_le(a974, a975) {
                let a976 = v0;
                let a977 = 122u64;
                nat_le(a976, a977)
            } else {
                false
            } {
                true
            } else {
                let a978 = v0;
                let a979 = 95u64;
                if nat_eq(a978, a979) {
                    true
                } else {
                    let a980 = v0;
                    let a981 = 45u64;
                    if nat_eq(a980, a981) {
                        true
                    } else {
                        let a982 = v0;
                        let a983 = 46u64;
                        nat_eq(a982, a983)
                    }
                }
            }
        }
    }
}

pub fn f255() -> Str {
    Str::lit("https://json-schema.org/draft/2020-12/schema")
}

pub fn f256(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>, v3: bool, v4: List<Str>, v5: Adt120, v6: List<Adt104>) -> R<Result<(List<Adt104>, Adt120), Adt91>> {
    let m984 = f312(v0.clone(), v1, v2.clone());
    match m984 {
        None => {
            let m985 = f313(v0, v2.clone(), v3, v4.clone(), v5)?;
            match m985 {
                Adt142::C0(v8) => {
                    Ok(Err::<(List<Adt104>, Adt120), Adt91>(v8))
                }
                Adt142::C1(v9) => {
                    Ok(Ok::<(List<Adt104>, Adt120), Adt91>((f314(v2, v3, v4, v6), v9)))
                }
            }
        }
        Some(v7) => {
            Ok(Err::<(List<Adt104>, Adt120), Adt91>(v7))
        }
    }
}

pub fn f257(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt121> {
    Ok(f315(List::cons((Str::lit("type"), f316(v0.clone(), v1.clone())?), List::cons((Str::lit("enum"), f317(v0.clone(), v1.clone())?), List::cons((Str::lit("const"), f318(v0.clone(), v1.clone())?), List::cons((Str::lit("minimum"), f319(Str::lit("minimum"), 0u64, v0.clone(), v1.clone())?), List::cons((Str::lit("maximum"), f319(Str::lit("maximum"), 1u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMinimum"), f319(Str::lit("exclusiveMinimum"), 2u64, v0.clone(), v1.clone())?), List::cons((Str::lit("exclusiveMaximum"), f319(Str::lit("exclusiveMaximum"), 3u64, v0.clone(), v1.clone())?), {
        let a986 = f320(v0.clone(), v1.clone())?;
        let a987 = List::cons((Str::lit("required"), f321(v0, v1)), List::<(Str, Adt136)>::nil());
        append_list(a986, a987)
    })))))))))
}

pub fn f258(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> R<Adt121> {
    Ok(f322(f323(v0, v1.clone(), v2.clone(), v3.clone()), f322(f324(v1.clone(), v2.clone(), v3.clone()), f322(f325(v1.clone(), v2.clone(), v3.clone()), f322(f326(v1.clone(), v2.clone(), v3.clone())?, f322(f327(v1.clone(), v2.clone(), v3.clone()), f322(f328(Adt102::C0, f199(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f322(f328(Adt102::C1, f200(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f322(f328(Adt102::C2, f201(v1.clone(), v2.clone(), v3.clone()), v1.clone(), v2.clone(), v3.clone()), f328(Adt102::C3, f202(v1.clone(), v2.clone(), v3.clone()), v1, v2, v3))))))))))
}

pub fn f259(v0: Adt6) -> Option<List<Adt6>> {
    let m988 = v0;
    match m988 {
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

pub fn f260(v0: Str, v1: List<(Str, Adt6)>) -> Option<Adt6> {
    let m989 = f234(v0, v1);
    match m989 {
        Adt130::C0 => {
            None::<Adt6>
        }
        Adt130::C1(v2) => {
            Some(v2)
        }
    }
}

pub fn f261(v0: List<u64>) -> R<Option<List<u64>>> {
    let m991 = f329({
        let a990 = v0.clone();
        length_list(a990)
    }, v0, List::<u64>::nil())?;
    match m991 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            Ok(Some(f163(v1, List::<u64>::nil())))
        }
    }
}

pub fn f262(v0: List<Adt50>) -> R<bool> {
    let m992 = v0;
    match m992.uncons() {
        None => {
            Ok(true)
        }
        Some((v1, v2)) => {
            f330(v2, match v1 {
                Adt50::C0(h993, _, _) => {
                    h993
                }
            })
        }
    }
}

pub fn f263(v0: Str) -> R<Str> {
    let m996 = {
        let a995 = f331(f332(), f163(f333(f87({
            let a994 = v0.clone();
            utf8_encode(a994)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a995)
    };
    match m996 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f264(v0: u64, v1: Adt6, v2: List<Adt106>, v3: List<u64>) -> R<Adt132> {
    let m997 = v1;
    match m997 {
        Adt6::C0 => {
            Ok(Adt132::C1(v2, f163(f334(2u64), v3)))
        }
        Adt6::C1(v4) => {
            Ok(Adt132::C1(v2, f163(if v4 {
                f334(0u64)
            } else {
                f334(1u64)
            }, v3)))
        }
        Adt6::C2(v5) => {
            if f335(v5.clone())? {
                Ok(Adt132::C1(v2, f163(f336(v5)?, v3)))
            } else {
                Ok(Adt132::C0(Adt93::C11))
            }
        }
        Adt6::C3(v6) => {
            Ok(Adt132::C1(v2, f163(f265(v6)?, v3)))
        }
        Adt6::C4(v7) => {
            let a998 = v0;
            let a999 = f337();
            if nat_lt(a998, a999) {
                Ok(Adt132::C1(f338(91u64, Adt106::C2({
                    let a1000 = v0;
                    let a1001 = 1u64;
                    nat_add(a1000, a1001)?
                }, v7), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
        Adt6::C5(v8) => {
            let a1002 = v0;
            let a1003 = f337();
            if nat_lt(a1002, a1003) {
                Ok(Adt132::C1(f338(123u64, Adt106::C4({
                    let a1004 = v0;
                    let a1005 = 1u64;
                    nat_add(a1004, a1005)?
                }, v8), v2), v3))
            } else {
                Ok(Adt132::C0(Adt93::C12))
            }
        }
    }
}

pub fn f265(v0: Str) -> R<List<u64>> {
    Ok(List::cons(34u64, {
        let a1007 = f163(f339(f87({
            let a1006 = v0;
            utf8_encode(a1006)
        })?, List::<u64>::nil())?, List::<u64>::nil());
        let a1008 = List::cons(34u64, List::<u64>::nil());
        append_list(a1007, a1008)
    }))
}

pub fn f266(v0: Adt32) -> R<Option<List<(Str, Adt6)>>> {
    let m1009 = v0;
    match m1009 {
        Adt32::C0(v1) => {
            Ok(Some(f340(v1)?))
        }
        Adt32::C1(v2) => {
            Ok(Some(f341(v2)?))
        }
        Adt32::C2(v3) => {
            f342(f343(v3)?)
        }
        Adt32::C3(v4) => {
            f342(f343(match v4 {
                Adt49::C0(h1010, _) => {
                    h1010
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

pub fn f267(v0: Adt123) -> Adt5 {
    Adt5::C0(List::cons((Str::lit("resultType"), Adt6::C3(f344(f345(match v0.clone() {
        Adt123::C0(h1011, _, _) => {
            h1011
        }
    })))), List::cons((Str::lit("_meta"), Adt6::C5(f346({
        let m1014 = match v0 {
            Adt123::C0(_, h1013, _) => {
                h1013
            }
        };
        match m1014 {
            Adt124::C0(h1012, _) => {
                h1012
            }
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f268(v0: bool, v1: Adt88, v2: List<List<u64>>) -> Adt88 {
    if f347(v0, match v1.clone() {
        Adt88::C0(h1015, _, _, _) => {
            h1015
        }
    }, v2.clone()) {
        f348(match v1.clone() {
            Adt88::C0(h1016, _, _, _) => {
                h1016
            }
        }, {
            let a1018 = v2;
            let a1019 = List::cons(match v1 {
                Adt88::C0(_, _, _, h1017) => {
                    h1017
                }
            }, List::<List<u64>>::nil());
            append_list(a1018, a1019)
        })
    } else {
        v1
    }
}

pub fn f269(v0: Adt125) -> R<Option<List<u64>>> {
    let m1020 = f349(f350(v0))?;
    match m1020 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v1) => {
            let m1021 = f89(v1)?;
            match m1021 {
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

pub fn f270() -> u64 {
    404u64
}

pub fn f271(v0: Adt84) -> u64 {
    let m1022 = v0;
    match m1022 {
        Adt84::C0(_) => {
            f96()
        }
        Adt84::C1(_) => {
            f96()
        }
        Adt84::C2(_) => {
            f96()
        }
        Adt84::C3(_) => {
            f96()
        }
        Adt84::C4(_) => {
            f96()
        }
        Adt84::C5(_) => {
            f96()
        }
        Adt84::C6(_) => {
            f21()
        }
        Adt84::C7(_) => {
            f351()
        }
        Adt84::C8(_) => {
            f96()
        }
        Adt84::C9(_) => {
            f21()
        }
    }
}

pub fn f272(v0: Adt126) -> i64 {
    let m1023 = v0;
    match m1023 {
        Adt126::C0(_) => {
            f352()
        }
        Adt126::C1(_, _) => {
            f353()
        }
        Adt126::C2(_) => {
            f354()
        }
        Adt126::C3(_) => {
            f355()
        }
        Adt126::C4(v6) => {
            f356(v6)
        }
    }
}

pub fn f273(v0: Adt126) -> Str {
    let m1024 = v0;
    match m1024 {
        Adt126::C0(v1) => {
            f357(v1)
        }
        Adt126::C1(_, _) => {
            Str::lit("Unsupported protocol version")
        }
        Adt126::C2(v4) => {
            f358(v4)
        }
        Adt126::C3(_) => {
            Str::lit("Method not found")
        }
        Adt126::C4(v6) => {
            let m1025 = v6;
            match m1025 {
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

pub fn f274(v0: Adt126) -> R<Option<Adt6>> {
    let m1026 = v0;
    match m1026 {
        Adt126::C0(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C1(v2, v3) => {
            Ok(Some(Adt6::C5(List::cons((Str::lit("supported"), Adt6::C4(f359(v3))), List::cons((Str::lit("requested"), Adt6::C3(v2)), List::<(Str, Adt6)>::nil())))))
        }
        Adt126::C2(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C3(_) => {
            Ok(None::<Adt6>)
        }
        Adt126::C4(v6) => {
            let m1027 = v6;
            match m1027 {
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
                    Ok(Some(Adt6::C5(List::cons((Str::lit("limit"), Adt6::C2(Adt7::C0(f79(v14, 0i64)?, 0i64))), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C8(v15) => {
                    Ok(Some(Adt6::C5(List::cons((Str::lit("requiredCapabilities"), f360(v15)), List::<(Str, Adt6)>::nil()))))
                }
                Adt84::C9(_) => {
                    Ok(None::<Adt6>)
                }
            }
        }
    }
}

pub fn f275(v0: List<u64>) -> R<bool> {
    let m1028 = f219(59u64, v0, List::<u64>::nil(), List::<List<u64>>::nil());
    match m1028.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            if f361(v2)? {
                Ok(false)
            } else {
                f138(f140(v1), f362())
            }
        }
    }
}

pub fn f276(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133) -> R<Result<Adt123, Adt126>> {
    f363(v0.clone(), v1, v2, v3.clone(), f364(match v0 {
        Adt0::C0(_, _, h1029, _, _, _) => {
            h1029
        }
    }, match v3 {
        Adt133::C0(_, h1030, _, _) => {
            h1030
        }
    }))
}

pub fn f277(v0: Adt137, v1: Adt138) -> Result<Adt133, Adt126> {
    let m1031 = v1;
    match m1031 {
        Adt138::C0(v3) => {
            f365(v3, v0)
        }
        Adt138::C1(v2) => {
            Err::<Adt133, Adt126>(v2)
        }
    }
}

pub fn f278(v0: Str) -> Adt137 {
    let a1032 = v0.clone();
    let a1033 = Str::lit("server/discover");
    if equal(a1032, a1033) {
        Adt137::C1(Adt134::C0)
    } else {
        let a1034 = v0.clone();
        let a1035 = Str::lit("tools/list");
        if equal(a1034, a1035) {
            Adt137::C1(Adt134::C1)
        } else {
            let a1036 = v0.clone();
            let a1037 = Str::lit("tools/call");
            if equal(a1036, a1037) {
                Adt137::C1(Adt134::C2)
            } else {
                let a1038 = v0.clone();
                let a1039 = Str::lit("resources/list");
                if equal(a1038, a1039) {
                    Adt137::C1(Adt134::C3)
                } else {
                    let a1040 = v0.clone();
                    let a1041 = Str::lit("resources/read");
                    if equal(a1040, a1041) {
                        Adt137::C1(Adt134::C4)
                    } else {
                        let a1042 = v0.clone();
                        let a1043 = Str::lit("resources/templates/list");
                        if equal(a1042, a1043) {
                            Adt137::C1(Adt134::C5)
                        } else {
                            let a1044 = v0.clone();
                            let a1045 = Str::lit("prompts/list");
                            if equal(a1044, a1045) {
                                Adt137::C1(Adt134::C6)
                            } else {
                                let a1046 = v0.clone();
                                let a1047 = Str::lit("prompts/get");
                                if equal(a1046, a1047) {
                                    Adt137::C1(Adt134::C7)
                                } else {
                                    let a1048 = v0.clone();
                                    let a1049 = Str::lit("completion/complete");
                                    if equal(a1048, a1049) {
                                        Adt137::C1(Adt134::C8)
                                    } else {
                                        let a1050 = v0.clone();
                                        let a1051 = Str::lit("tasks/get");
                                        if equal(a1050, a1051) {
                                            Adt137::C1(Adt134::C9)
                                        } else {
                                            let a1052 = v0.clone();
                                            let a1053 = Str::lit("tasks/update");
                                            if equal(a1052, a1053) {
                                                Adt137::C1(Adt134::C10)
                                            } else {
                                                let a1054 = v0;
                                                let a1055 = Str::lit("tasks/cancel");
                                                if equal(a1054, a1055) {
                                                    Adt137::C1(Adt134::C11)
                                                } else {
                                                    Adt137::C0
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

pub fn f279(v0: Option<Adt107>, v1: List<Str>, v2: Adt103) -> R<Adt138> {
    f366(v0, v1, v2.clone(), f367(match v2 {
        Adt103::C0(_, _, h1056) => {
            h1056
        }
    }))
}

pub fn f280(v0: List<u64>) -> Option<Str> {
    let a1057 = f331(f332(), v0);
    utf8_decode(a1057)
}

pub fn f281(v0: Str) -> Adt140 {
    let m1058 = f278(v0);
    match m1058 {
        Adt137::C0 => {
            Adt140::C0
        }
        Adt137::C1(v1) => {
            f368(v1)
        }
    }
}

pub fn f282(v0: List<u64>) -> Option<List<u64>> {
    let m1059 = f369(f370(), v0);
    match m1059 {
        None => {
            None::<List<u64>>
        }
        Some(v1) => {
            let m1060 = f369(f371(), f148(v1, List::<u64>::nil()));
            match m1060 {
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

pub fn f283(p1063: List<Adt110>, p1064: List<Adt110>) -> List<Adt110> {
    let m1062 = (p1063, p1064);
    {
        let mut m1062 = m1062;
        loop {
            let step = {
                let (v0, v1) = m1062;
                let m1061 = v0;
                match m1061.uncons() {
                    None => {
                        Err::<(List<Adt110>, List<Adt110>), List<Adt110>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt110>, List<Adt110>), List<Adt110>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: List<Adt86>, p1071: List<Adt12>, v2: Option<Str>, p1072: List<Adt110>) -> R<List<Adt110>> {
    let m1070 = (p1071, p1072);
    {
        let mut m1070 = m1070;
        loop {
            let step = {
                let (v1, v3) = m1070;
                let m1065 = v1;
                match m1065.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>(Ok(v3))
                    }
                    Some((v4, v5)) => {
                        if f372(v4.clone(), v2.clone()) {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, List::cons(Adt110::C0(match v4.clone() {
                                Adt12::C0(_, h1066, _, _) => {
                                    h1066
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, h1067, _) => {
                                    h1067
                                }
                            }, match v4.clone() {
                                Adt12::C0(_, _, _, h1068) => {
                                    h1068
                                }
                            }, f373(v0.clone(), match v4 {
                                Adt12::C0(_, h1069, _, _) => {
                                    h1069
                                }
                            })?), v3)))
                        } else {
                            Ok::<(List<Adt12>, List<Adt110>), R<List<Adt110>>>((v5, v3))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1070 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f285(p1088: u64, p1089: i64, p1090: i64) -> R<Option<i64>> {
    let m1087 = (p1088, (p1089, p1090));
    {
        let mut m1087 = m1087;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1087;
                let m1073 = v0;
                if m1073 == 0 {
                    Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                } else {
                    let v3 = m1073 - 1;
                    if f374(v2) {
                        Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(Some(v1)))
                    } else {
                        let m1076 = {
                            let a1074 = v2;
                            let a1075 = 0i64;
                            compare(a1074, a1075)
                        };
                        if match m1076 {
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
                            if f374({
                                let a1077 = v1;
                                let a1078 = 10i64;
                                let a1079 = 0i64;
                                int_rem(a1077, a1078, a1079)
                            }) {
                                Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                    let a1080 = v1;
                                    let a1081 = 10i64;
                                    let a1082 = 0i64;
                                    int_quot(a1080, a1081, a1082)?
                                }, f115(v2, 1i64)?)))
                            } else {
                                Err::<(u64, (i64, i64)), R<Option<i64>>>(Ok(None::<i64>))
                            }
                        } else {
                            Ok::<(u64, (i64, i64)), R<Option<i64>>>((v3, ({
                                let a1083 = v1;
                                let a1084 = 10i64;
                                int_mul(a1083, a1084)?
                            }, {
                                let a1085 = v2;
                                let a1086 = 1i64;
                                int_sub(a1085, a1086)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1087 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(v0: i64) -> Option<i64> {
    let m1093 = {
        let a1091 = 9007199254740991i64;
        let a1092 = v0;
        compare(a1091, a1092)
    };
    if if match m1093 {
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
        let m1096 = {
            let a1094 = v0;
            let a1095 = -9007199254740991i64;
            compare(a1094, a1095)
        };
        match m1096 {
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

pub fn f287(v0: Str, v1: Str) -> Adt141 {
    let a1097 = v0;
    let a1098 = v1;
    if equal(a1097, a1098) {
        Adt141::C0
    } else {
        Adt141::C1
    }
}

pub fn f288(v0: Adt130) -> bool {
    let m1099 = v0;
    match m1099 {
        Adt130::C0 => {
            false
        }
        Adt130::C1(_) => {
            true
        }
    }
}

pub fn f289(v0: Adt92, v1: Str, v2: Adt5) -> Result<Adt103, Adt94> {
    let m1100 = f176(Str::lit("params"), v2);
    match m1100 {
        Adt130::C0 => {
            Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, None::<Adt5>))
        }
        Adt130::C1(v3) => {
            let m1101 = f195(v3);
            match m1101 {
                None => {
                    f134(Some(v0), Adt95::C6)
                }
                Some(v4) => {
                    Ok::<Adt103, Adt94>(Adt103::C0(v0, v1, Some(v4)))
                }
            }
        }
    }
}

pub fn f290(v0: Adt118) -> bool {
    let m1102 = v0;
    match m1102 {
        Adt118::C0 => {
            false
        }
        Adt118::C1 => {
            true
        }
        Adt118::C2 => {
            true
        }
        Adt118::C3 => {
            false
        }
        Adt118::C4 => {
            true
        }
        Adt118::C5 => {
            false
        }
        Adt118::C6 => {
            false
        }
        Adt118::C7 => {
            true
        }
    }
}

pub fn f291(v0: Adt114, v1: Adt119, v2: Adt115, v3: u64) -> R<Adt131> {
    let a1104 = match v0.clone() {
        Adt114::C0(_, _, h1103) => {
            h1103
        }
    };
    let a1105 = f337();
    if nat_lt(a1104, a1105) {
        Ok(Adt131::C1(Adt114::C0(v2, List::cons(v1, match v0.clone() {
            Adt114::C0(_, h1106, _) => {
                h1106
            }
        }), {
            let a1108 = match v0 {
                Adt114::C0(_, _, h1107) => {
                    h1107
                }
            };
            let a1109 = 1u64;
            nat_add(a1108, a1109)?
        })))
    } else {
        Ok(f247(Adt93::C9(v3)))
    }
}

pub fn f292(v0: bool, v1: Adt118, v2: i64, v3: u64, v4: i64, v5: bool, v6: i64) -> Adt115 {
    Adt115::C8(Adt117::C0(v0, v1, v2, v3, v4, v5, v6))
}

pub fn f293(v0: u64) -> i64 {
    let a1110 = v0;
    let a1111 = 0u64;
    if nat_eq(a1110, a1111) {
        0i64
    } else {
        let a1112 = v0;
        let a1113 = 1u64;
        if nat_eq(a1112, a1113) {
            1i64
        } else {
            let a1114 = v0;
            let a1115 = 2u64;
            if nat_eq(a1114, a1115) {
                2i64
            } else {
                let a1116 = v0;
                let a1117 = 3u64;
                if nat_eq(a1116, a1117) {
                    3i64
                } else {
                    let a1118 = v0;
                    let a1119 = 4u64;
                    if nat_eq(a1118, a1119) {
                        4i64
                    } else {
                        let a1120 = v0;
                        let a1121 = 5u64;
                        if nat_eq(a1120, a1121) {
                            5i64
                        } else {
                            let a1122 = v0;
                            let a1123 = 6u64;
                            if nat_eq(a1122, a1123) {
                                6i64
                            } else {
                                let a1124 = v0;
                                let a1125 = 7u64;
                                if nat_eq(a1124, a1125) {
                                    7i64
                                } else {
                                    let a1126 = v0;
                                    let a1127 = 8u64;
                                    if nat_eq(a1126, a1127) {
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

pub fn f294(p1130: List<Adt6>, p1131: List<Adt6>) -> List<Adt6> {
    let m1129 = (p1130, p1131);
    {
        let mut m1129 = m1129;
        loop {
            let step = {
                let (v0, v1) = m1129;
                let m1128 = v0;
                match m1128.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt6>), List<Adt6>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Adt114, v1: u64) -> Adt131 {
    let m1133 = match v0.clone() {
        Adt114::C0(_, h1132, _) => {
            h1132
        }
    };
    match m1133.uncons() {
        None => {
            f247(Adt93::C2(v1))
        }
        Some((v2, _)) => {
            let m1134 = v2;
            match m1134 {
                Adt119::C0(_) => {
                    f246(v0, Adt115::C0)
                }
                Adt119::C1(_, _) => {
                    f246(v0, Adt115::C4)
                }
            }
        }
    }
}

pub fn f296(p1137: List<(Str, Adt6)>, p1138: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1136 = (p1137, p1138);
    {
        let mut m1136 = m1136;
        loop {
            let step = {
                let (v0, v1) = m1136;
                let m1135 = v0;
                match m1135.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<(Str, Adt6)>), List<(Str, Adt6)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1136 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Adt114, v1: Adt115) -> Adt114 {
    Adt114::C0(v1, match v0.clone() {
        Adt114::C0(_, h1139, _) => {
            h1139
        }
    }, match v0 {
        Adt114::C0(_, _, h1140) => {
            h1140
        }
    })
}

pub fn f298(v0: Adt114, v1: u64, v2: List<u64>, v3: bool) -> Adt131 {
    let m1142 = {
        let a1141 = f331(f332(), f163(v2, List::<u64>::nil()));
        utf8_decode(a1141)
    };
    match m1142 {
        None => {
            f247(Adt93::C14)
        }
        Some(v4) => {
            if v3 {
                f375(v0, v1, v4)
            } else {
                f304(v0, Adt6::C3(v4))
            }
        }
    }
}

pub fn f299(v0: u64) -> u64 {
    let a1143 = v0;
    let a1144 = 34u64;
    if nat_eq(a1143, a1144) {
        34u64
    } else {
        let a1145 = v0;
        let a1146 = 92u64;
        if nat_eq(a1145, a1146) {
            92u64
        } else {
            let a1147 = v0;
            let a1148 = 47u64;
            if nat_eq(a1147, a1148) {
                47u64
            } else {
                let a1149 = v0;
                let a1150 = 98u64;
                if nat_eq(a1149, a1150) {
                    8u64
                } else {
                    let a1151 = v0;
                    let a1152 = 102u64;
                    if nat_eq(a1151, a1152) {
                        12u64
                    } else {
                        let a1153 = v0;
                        let a1154 = 110u64;
                        if nat_eq(a1153, a1154) {
                            10u64
                        } else {
                            let a1155 = v0;
                            let a1156 = 114u64;
                            if nat_eq(a1155, a1156) {
                                13u64
                            } else {
                                let a1157 = v0;
                                let a1158 = 116u64;
                                if nat_eq(a1157, a1158) {
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

pub fn f300(v0: u64) -> u64 {
    let a1159 = v0;
    let a1160 = 48u64;
    if nat_lt(a1159, a1160) {
        16u64
    } else {
        let a1161 = v0;
        let a1162 = 58u64;
        if nat_lt(a1161, a1162) {
            let a1163 = v0;
            let a1164 = 48u64;
            nat_sub(a1163, a1164)
        } else {
            let a1165 = v0;
            let a1166 = 65u64;
            if nat_lt(a1165, a1166) {
                16u64
            } else {
                let a1167 = v0;
                let a1168 = 71u64;
                if nat_lt(a1167, a1168) {
                    let a1169 = v0;
                    let a1170 = 55u64;
                    nat_sub(a1169, a1170)
                } else {
                    let a1171 = v0;
                    let a1172 = 97u64;
                    if nat_lt(a1171, a1172) {
                        16u64
                    } else {
                        let a1173 = v0;
                        let a1174 = 103u64;
                        if nat_lt(a1173, a1174) {
                            let a1175 = v0;
                            let a1176 = 87u64;
                            nat_sub(a1175, a1176)
                        } else {
                            16u64
                        }
                    }
                }
            }
        }
    }
}

pub fn f301(v0: Adt114, v1: u64, v2: u64, v3: u64, v4: List<u64>, v5: bool) -> R<Adt131> {
    let a1177 = v3;
    let a1178 = 0u64;
    if nat_eq(a1177, a1178) {
        if f238(v2, 55296u64, 56319u64) {
            Ok(f246(v0, Adt115::C6(Adt116::C3(v2), v4, v5)))
        } else {
            if f238(v2, 56320u64, 57343u64) {
                Ok(f247(Adt93::C6(v1)))
            } else {
                Ok(f246(v0, Adt115::C6(Adt116::C0, f376(v2, v4)?, v5)))
            }
        }
    } else {
        if f238(v2, 56320u64, 57343u64) {
            Ok(f246(v0, Adt115::C6(Adt116::C0, f376({
                let a1187 = 65536u64;
                let a1188 = {
                    let a1185 = {
                        let a1181 = {
                            let a1179 = v3;
                            let a1180 = 55296u64;
                            nat_sub(a1179, a1180)
                        };
                        let a1182 = 1024u64;
                        nat_mul(a1181, a1182)?
                    };
                    let a1186 = {
                        let a1183 = v2;
                        let a1184 = 56320u64;
                        nat_sub(a1183, a1184)
                    };
                    nat_add(a1185, a1186)?
                };
                nat_add(a1187, a1188)?
            }, v4)?, v5)))
        } else {
            Ok(f247(Adt93::C6(v1)))
        }
    }
}

pub fn f302(v0: u64, v1: u64) -> u64 {
    let m1191 = {
        let a1189 = f303(v0);
        let a1190 = v1;
        index_list(a1189, a1190)
    };
    match m1191 {
        None => {
            0u64
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f303(v0: u64) -> List<u64> {
    let a1192 = v0;
    let a1193 = 0u64;
    if nat_eq(a1192, a1193) {
        List::cons(116u64, List::cons(114u64, List::cons(117u64, List::cons(101u64, List::<u64>::nil()))))
    } else {
        let a1194 = v0;
        let a1195 = 1u64;
        if nat_eq(a1194, a1195) {
            List::cons(102u64, List::cons(97u64, List::cons(108u64, List::cons(115u64, List::cons(101u64, List::<u64>::nil())))))
        } else {
            List::cons(110u64, List::cons(117u64, List::cons(108u64, List::cons(108u64, List::<u64>::nil()))))
        }
    }
}

pub fn f304(v0: Adt114, v1: Adt6) -> Adt131 {
    Adt131::C1(f240(v1, match v0.clone() {
        Adt114::C0(_, h1196, _) => {
            h1196
        }
    }, match v0 {
        Adt114::C0(_, _, h1197) => {
            h1197
        }
    }))
}

pub fn f305(v0: u64) -> Adt6 {
    let a1198 = v0;
    let a1199 = 0u64;
    if nat_eq(a1198, a1199) {
        Adt6::C1(true)
    } else {
        let a1200 = v0;
        let a1201 = 1u64;
        if nat_eq(a1200, a1201) {
            Adt6::C1(false)
        } else {
            Adt6::C0
        }
    }
}

pub fn f306(v0: Adt114, v1: Adt117, v2: Adt118) -> Adt131 {
    f246(v0, Adt115::C8(Adt117::C0(match v1.clone() {
        Adt117::C0(h1202, _, _, _, _, _, _) => {
            h1202
        }
    }, v2, match v1.clone() {
        Adt117::C0(_, _, h1203, _, _, _, _) => {
            h1203
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, h1204, _, _, _) => {
            h1204
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, h1205, _, _) => {
            h1205
        }
    }, match v1.clone() {
        Adt117::C0(_, _, _, _, _, h1206, _) => {
            h1206
        }
    }, match v1 {
        Adt117::C0(_, _, _, _, _, _, h1207) => {
            h1207
        }
    })))
}

pub fn f307(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: u64 = {
        let a1209 = match v3.clone() {
            Adt117::C0(_, _, _, h1208, _, _, _) => {
                h1208
            }
        };
        let a1210 = 0u64;
        if if nat_eq(a1209, a1210) {
            let a1211 = v2;
            let a1212 = 48u64;
            nat_eq(a1211, a1212)
        } else {
            false
        } {
            0u64
        } else {
            let a1214 = match v3.clone() {
                Adt117::C0(_, _, _, h1213, _, _, _) => {
                    h1213
                }
            };
            let a1215 = 1u64;
            nat_add(a1214, a1215)?
        }
    };
    let v6: i64 = if v4 {
        f115(match v3.clone() {
            Adt117::C0(_, _, _, _, h1216, _, _) => {
                h1216
            }
        }, 1i64)?
    } else {
        match v3.clone() {
            Adt117::C0(_, _, _, _, h1217, _, _) => {
                h1217
            }
        }
    };
    let a1218 = f377();
    let a1219 = v5;
    if nat_lt(a1218, a1219) {
        Ok(f247(Adt93::C8(v1)))
    } else {
        let m1222 = {
            let a1220 = 99999i64;
            let a1221 = v6;
            compare(a1220, a1221)
        };
        if match m1222 {
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
            Ok(f247(Adt93::C8(v1)))
        } else {
            Ok(f246(v0, f292(match v3.clone() {
                Adt117::C0(h1223, _, _, _, _, _, _) => {
                    h1223
                }
            }, if v4 {
                Adt118::C4
            } else {
                Adt118::C2
            }, f115({
                let a1225 = match v3.clone() {
                    Adt117::C0(_, _, h1224, _, _, _, _) => {
                        h1224
                    }
                };
                let a1226 = 10i64;
                int_mul(a1225, a1226)?
            }, f293({
                let a1227 = v2;
                let a1228 = 48u64;
                nat_sub(a1227, a1228)
            }))?, v5, v6, match v3.clone() {
                Adt117::C0(_, _, _, _, _, h1229, _) => {
                    h1229
                }
            }, match v3 {
                Adt117::C0(_, _, _, _, _, _, h1230) => {
                    h1230
                }
            })))
        }
    }
}

pub fn f308(v0: u64) -> bool {
    let a1231 = v0;
    let a1232 = 101u64;
    if nat_eq(a1231, a1232) {
        true
    } else {
        let a1233 = v0;
        let a1234 = 69u64;
        nat_eq(a1233, a1234)
    }
}

pub fn f309(v0: u64) -> bool {
    f238(v0, 48u64, 57u64)
}

pub fn f310(v0: Adt114, v1: u64, v2: u64, v3: Adt117) -> R<Adt131> {
    let m1235 = f239(v3)?;
    match m1235 {
        Adt135::C0 => {
            Ok(f247(Adt93::C7(v1)))
        }
        Adt135::C1 => {
            Ok(f247(Adt93::C8(v1)))
        }
        Adt135::C2(v4) => {
            Ok(f244(f240(v4, match v0.clone() {
                Adt114::C0(_, h1236, _) => {
                    h1236
                }
            }, match v0 {
                Adt114::C0(_, _, h1237) => {
                    h1237
                }
            }), v1, v2))
        }
    }
}

pub fn f311(v0: Adt114, v1: u64, v2: u64, v3: Adt117, v4: bool) -> R<Adt131> {
    let v5: i64 = f115({
        let a1239 = match v3.clone() {
            Adt117::C0(_, _, _, _, _, _, h1238) => {
                h1238
            }
        };
        let a1240 = 10i64;
        int_mul(a1239, a1240)?
    }, f293({
        let a1241 = v2;
        let a1242 = 48u64;
        nat_sub(a1241, a1242)
    }))?;
    let m1245 = {
        let a1243 = 99999i64;
        let a1244 = v5;
        compare(a1243, a1244)
    };
    if match m1245 {
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
        Ok(f247(Adt93::C8(v1)))
    } else {
        Ok(f246(v0, f292(match v3.clone() {
            Adt117::C0(h1246, _, _, _, _, _, _) => {
                h1246
            }
        }, Adt118::C7, match v3.clone() {
            Adt117::C0(_, _, h1247, _, _, _, _) => {
                h1247
            }
        }, match v3.clone() {
            Adt117::C0(_, _, _, h1248, _, _, _) => {
                h1248
            }
        }, match v3 {
            Adt117::C0(_, _, _, _, h1249, _, _) => {
                h1249
            }
        }, v4, v5)))
    }
}

pub fn f312(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1250 = f378(f379(), v2.clone());
    match m1250 {
        None => {
            let m1251 = f380(f381(), v2.clone());
            match m1251 {
                None => {
                    f382(v0, v1, v2)
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

pub fn f313(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: Adt120) -> R<Adt142> {
    let m1252 = f234(Str::lit("x-mcp-header"), v1.clone());
    match m1252 {
        Adt130::C0 => {
            Ok(Adt142::C1(v4))
        }
        Adt130::C1(v5) => {
            f383(v0, v1, v5, v2, v3, v4)
        }
    }
}

pub fn f314(v0: List<(Str, Adt6)>, v1: bool, v2: List<Str>, v3: List<Adt104>) -> List<Adt104> {
    f384(Str::lit("properties"), v0.clone(), v1, v2, f384(Str::lit("$defs"), v0.clone(), false, List::<Str>::nil(), f385(Str::lit("items"), v0.clone(), f385(Str::lit("additionalProperties"), v0.clone(), f386(Str::lit("allOf"), v0.clone(), f386(Str::lit("anyOf"), v0.clone(), f386(Str::lit("oneOf"), v0.clone(), f385(Str::lit("not"), v0.clone(), f385(Str::lit("if"), v0.clone(), f385(Str::lit("then"), v0.clone(), f385(Str::lit("else"), v0, v3)))))))))))
}

pub fn f315(p1259: List<(Str, Adt136)>) -> Adt121 {
    let m1258 = p1259;
    {
        let mut m1258 = m1258;
        loop {
            let step = {
                let v0 = m1258;
                let m1253 = v0;
                match m1253.uncons() {
                    None => {
                        Err::<List<(Str, Adt136)>, Adt121>(Adt121::C3(List::<Adt101>::nil()))
                    }
                    Some((v1, v2)) => {
                        let m1255 = {
                            let (_, h1254) = v1;
                            h1254
                        };
                        match m1255 {
                            Adt136::C0 => {
                                Ok::<List<(Str, Adt136)>, Adt121>(v2)
                            }
                            Adt136::C1 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C0({
                                    let (h1256, _) = v1;
                                    h1256
                                }))
                            }
                            Adt136::C2 => {
                                Err::<List<(Str, Adt136)>, Adt121>(Adt121::C1({
                                    let (h1257, _) = v1;
                                    h1257
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1258 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f316(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1260 = f234(Str::lit("type"), v0);
    match m1260 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1261 = f231(v2.clone());
            match m1261 {
                None => {
                    let m1262 = f259(v2);
                    match m1262 {
                        None => {
                            Ok(Adt136::C2)
                        }
                        Some(v4) => {
                            f389(v4, v1, false)
                        }
                    }
                }
                Some(v3) => {
                    if f387(v3.clone()) {
                        if f388(v3, v1)? {
                            Ok(Adt136::C0)
                        } else {
                            Ok(Adt136::C1)
                        }
                    } else {
                        Ok(Adt136::C2)
                    }
                }
            }
        }
    }
}

pub fn f317(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1263 = f234(Str::lit("enum"), v0);
    match m1263 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1264 = f259(v2);
            match m1264 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v3) => {
                    f390(v3, v1)
                }
            }
        }
    }
}

pub fn f318(v0: List<(Str, Adt6)>, v1: Adt6) -> R<Adt136> {
    let m1265 = f234(Str::lit("const"), v0);
    match m1265 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v2) => {
            let m1266 = f391(v2, v1)?;
            match m1266 {
                Adt146::C0 => {
                    Ok(Adt136::C0)
                }
                Adt146::C1 => {
                    Ok(Adt136::C1)
                }
                Adt146::C2 => {
                    Ok(Adt136::C2)
                }
            }
        }
    }
}

pub fn f319(v0: Str, v1: u64, v2: List<(Str, Adt6)>, v3: Adt6) -> R<Adt136> {
    let m1267 = f234(v0, v2);
    match m1267 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1268 = f392(v4);
            match m1268 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1269 = f392(v3);
                    match m1269 {
                        None => {
                            Ok(Adt136::C0)
                        }
                        Some(v6) => {
                            if f393(v1, f394(v6, v5)?) {
                                Ok(Adt136::C0)
                            } else {
                                Ok(Adt136::C1)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f320(v0: List<(Str, Adt6)>, v1: Adt6) -> R<List<(Str, Adt136)>> {
    let m1270 = v1;
    match m1270 {
        Adt6::C0 => {
            Ok(List::<(Str, Adt136)>::nil())
        }
        Adt6::C1(_) => {
            Ok(List::<(Str, Adt136)>::nil())
        }
        Adt6::C2(_) => {
            Ok(List::<(Str, Adt136)>::nil())
        }
        Adt6::C3(v4) => {
            Ok(List::cons((Str::lit("minLength"), f395(Str::lit("minLength"), true, v0.clone(), f396(v4.clone())?)?), List::cons((Str::lit("maxLength"), f395(Str::lit("maxLength"), false, v0, f396(v4)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C4(v5) => {
            Ok(List::cons((Str::lit("minItems"), f395(Str::lit("minItems"), true, v0.clone(), f397(v5.clone(), 0u64)?)?), List::cons((Str::lit("maxItems"), f395(Str::lit("maxItems"), false, v0, f397(v5, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
        Adt6::C5(v6) => {
            Ok(List::cons((Str::lit("minProperties"), f395(Str::lit("minProperties"), true, v0.clone(), f398(v6.clone(), 0u64)?)?), List::cons((Str::lit("maxProperties"), f395(Str::lit("maxProperties"), false, v0, f398(v6, 0u64)?)?), List::<(Str, Adt136)>::nil())))
        }
    }
}

pub fn f321(v0: List<(Str, Adt6)>, v1: Adt6) -> Adt136 {
    let m1271 = f234(Str::lit("required"), v0);
    match m1271 {
        Adt130::C0 => {
            Adt136::C0
        }
        Adt130::C1(v2) => {
            let m1272 = f259(v2);
            match m1272 {
                None => {
                    Adt136::C2
                }
                Some(v3) => {
                    let m1273 = f195(v1);
                    match m1273 {
                        None => {
                            Adt136::C0
                        }
                        Some(v4) => {
                            f399(v3, v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f322(v0: Adt121, v1: Adt121) -> Adt121 {
    let m1274 = v0.clone();
    match m1274 {
        Adt121::C0(_) => {
            v0
        }
        Adt121::C1(_) => {
            v0
        }
        Adt121::C2(_) => {
            v0
        }
        Adt121::C3(v5) => {
            let m1275 = v1.clone();
            match m1275 {
                Adt121::C0(_) => {
                    v1
                }
                Adt121::C1(_) => {
                    v1
                }
                Adt121::C2(_) => {
                    v1
                }
                Adt121::C3(v9) => {
                    Adt121::C3({
                        let a1276 = v5;
                        let a1277 = v9;
                        append_list(a1276, a1277)
                    })
                }
            }
        }
    }
}

pub fn f323(v0: Adt6, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>) -> Adt121 {
    let m1278 = f234(Str::lit("$ref"), v1);
    match m1278 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v4) => {
            let m1279 = f231(v4);
            match m1279 {
                None => {
                    Adt121::C1(Str::lit("$ref"))
                }
                Some(v5) => {
                    let m1280 = f400(v0, v5.clone());
                    match m1280 {
                        None => {
                            Adt121::C2(v5)
                        }
                        Some(v6) => {
                            Adt121::C3(List::cons(Adt101::C0(v6, v2, v3), List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f324(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1281 = f234(Str::lit("properties"), v0);
    match m1281 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1282 = f195(v3);
            match m1282 {
                None => {
                    Adt121::C1(Str::lit("properties"))
                }
                Some(v4) => {
                    let m1283 = f195(v1);
                    match m1283 {
                        None => {
                            Adt121::C3(List::<Adt101>::nil())
                        }
                        Some(v5) => {
                            Adt121::C3(f401(match v4 {
                                Adt5::C0(h1284) => {
                                    h1284
                                }
                            }, match v5 {
                                Adt5::C0(h1285) => {
                                    h1285
                                }
                            }, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f325(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1286 = f234(Str::lit("additionalProperties"), v0.clone());
    match m1286 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1287 = f195(v1);
            match m1287 {
                None => {
                    Adt121::C3(List::<Adt101>::nil())
                }
                Some(v4) => {
                    let m1288 = f194(v3.clone());
                    match m1288 {
                        None => {
                            Adt121::C3(f404(match v4 {
                                Adt5::C0(h1290) => {
                                    h1290
                                }
                            }, f403(v0), v3, v2, List::<Adt101>::nil()))
                        }
                        Some(v5) => {
                            if v5 {
                                Adt121::C3(List::<Adt101>::nil())
                            } else {
                                if f402(match v4 {
                                    Adt5::C0(h1289) => {
                                        h1289
                                    }
                                }, f403(v0)) {
                                    Adt121::C0(Str::lit("additionalProperties"))
                                } else {
                                    Adt121::C3(List::<Adt101>::nil())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f326(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> R<Adt121> {
    let m1291 = f234(Str::lit("items"), v0);
    match m1291 {
        Adt130::C0 => {
            Ok(Adt121::C3(List::<Adt101>::nil()))
        }
        Adt130::C1(v3) => {
            let m1292 = f259(v1);
            match m1292 {
                None => {
                    Ok(Adt121::C3(List::<Adt101>::nil()))
                }
                Some(v4) => {
                    Ok(Adt121::C3(f405(v4, 0u64, v3, v2, List::<Adt101>::nil())?))
                }
            }
        }
    }
}

pub fn f327(v0: List<(Str, Adt6)>, v1: Adt6, v2: List<Adt99>) -> Adt121 {
    let m1293 = f234(Str::lit("allOf"), v0);
    match m1293 {
        Adt130::C0 => {
            Adt121::C3(List::<Adt101>::nil())
        }
        Adt130::C1(v3) => {
            let m1294 = f259(v3);
            match m1294 {
                None => {
                    Adt121::C1(Str::lit("allOf"))
                }
                Some(v4) => {
                    let m1295 = v4.clone();
                    match m1295.uncons() {
                        None => {
                            Adt121::C1(Str::lit("allOf"))
                        }
                        Some((_, _)) => {
                            Adt121::C3(f406(v4, v1, v2, List::<Adt101>::nil()))
                        }
                    }
                }
            }
        }
    }
}

pub fn f328(v0: Adt102, v1: Adt121, v2: List<(Str, Adt6)>, v3: Adt6, v4: List<Adt99>) -> Adt121 {
    let m1296 = v1.clone();
    match m1296 {
        Adt121::C0(_) => {
            v1
        }
        Adt121::C1(_) => {
            v1
        }
        Adt121::C2(_) => {
            v1
        }
        Adt121::C3(v8) => {
            let m1297 = v8;
            match m1297.uncons() {
                None => {
                    v1
                }
                Some((_, _)) => {
                    Adt121::C3(List::cons(Adt101::C5(v0, Adt6::C5(v2), v3, v4), List::<Adt101>::nil()))
                }
            }
        }
    }
}

pub fn f329(p1306: u64, p1307: List<u64>, p1308: List<u64>) -> R<Option<List<u64>>> {
    let m1305 = (p1306, (p1307, p1308));
    {
        let mut m1305 = m1305;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1305;
                let m1298 = v0;
                if m1298 == 0 {
                    let m1299 = v1;
                    match m1299.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                        }
                    }
                } else {
                    let v5 = m1298 - 1;
                    let m1300 = v1;
                    match m1300.uncons() {
                        None => {
                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(Some(v2)))
                        }
                        Some((v6, v7)) => {
                            let m1301 = v7;
                            match m1301.uncons() {
                                None => {
                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                }
                                Some((v8, v9)) => {
                                    let m1302 = v9;
                                    match m1302.uncons() {
                                        None => {
                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                        }
                                        Some((v10, v11)) => {
                                            let m1303 = v11;
                                            match m1303.uncons() {
                                                None => {
                                                    Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                }
                                                Some((v12, v13)) => {
                                                    let m1304 = f407(v6, v8, v10, v12, f408(v13.clone()))?;
                                                    match m1304 {
                                                        None => {
                                                            Err::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>(Ok(None::<List<u64>>))
                                                        }
                                                        Some(v14) => {
                                                            Ok::<(u64, (List<u64>, List<u64>)), R<Option<List<u64>>>>((v5, (v13, f163(v14, v2))))
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
                Ok(next) => { m1305 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(p1313: List<Adt50>, p1314: Adt7) -> R<bool> {
    let m1312 = (p1313, p1314);
    {
        let mut m1312 = m1312;
        loop {
            let step = {
                let (v0, v1) = m1312;
                let m1309 = v0;
                match m1309.uncons() {
                    None => {
                        Err::<(List<Adt50>, Adt7), R<bool>>(Ok(true))
                    }
                    Some((v2, v3)) => {
                        if f409(f394(match v2.clone() {
                            Adt50::C0(h1310, _, _) => {
                                h1310
                            }
                        }, v1)?) {
                            Ok::<(List<Adt50>, Adt7), R<bool>>((v3, match v2 {
                                Adt50::C0(h1311, _, _) => {
                                    h1311
                                }
                            }))
                        } else {
                            Err::<(List<Adt50>, Adt7), R<bool>>(Ok(false))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: Bytes, v1: List<u64>) -> Bytes {
    let a1315 = v0;
    let a1316 = f410(f332(), f411(List::<Bytes>::nil(), v1));
    append_bytes(a1315, a1316)
}

pub fn f332() -> Bytes {
    Bytes::lit(&[])
}

pub fn f333(p1323: List<u64>, p1324: List<u64>) -> List<u64> {
    let m1322 = (p1323, p1324);
    {
        let mut m1322 = m1322;
        loop {
            let step = {
                let (v0, v1) = m1322;
                let m1317 = v0;
                match m1317.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        let a1318 = v2;
                        let a1319 = 126u64;
                        if nat_eq(a1318, a1319) {
                            Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(48u64, List::cons(126u64, v1))))
                        } else {
                            let a1320 = v2;
                            let a1321 = 47u64;
                            if nat_eq(a1320, a1321) {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(49u64, List::cons(126u64, v1))))
                            } else {
                                Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: u64) -> List<u64> {
    f303(v0)
}

pub fn f335(v0: Adt7) -> R<bool> {
    let a1342 = {
        let m1328 = {
            let a1326 = f412();
            let a1327 = match v0.clone() {
                Adt7::C0(h1325, _) => {
                    h1325
                }
            };
            compare(a1326, a1327)
        };
        if match m1328 {
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
            let m1333 = {
                let a1331 = match v0.clone() {
                    Adt7::C0(h1329, _) => {
                        h1329
                    }
                };
                let a1332 = {
                    let a1330 = f412();
                    int_neg(a1330)?
                };
                compare(a1331, a1332)
            };
            if match m1333 {
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
                let m1337 = {
                    let a1335 = 9999i64;
                    let a1336 = match v0.clone() {
                        Adt7::C0(_, h1334) => {
                            h1334
                        }
                    };
                    compare(a1335, a1336)
                };
                if match m1337 {
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
                    let m1341 = {
                        let a1339 = match v0 {
                            Adt7::C0(_, h1338) => {
                                h1338
                            }
                        };
                        let a1340 = -9999i64;
                        compare(a1339, a1340)
                    };
                    match m1341 {
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
    Ok(bool_not(a1342))
}

pub fn f336(v0: Adt7) -> R<List<u64>> {
    let m1346 = {
        let a1344 = match v0.clone() {
            Adt7::C0(_, h1343) => {
                h1343
            }
        };
        let a1345 = 0i64;
        compare(a1344, a1345)
    };
    if if match m1346 {
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
        let m1350 = {
            let a1348 = 0i64;
            let a1349 = match v0.clone() {
                Adt7::C0(_, h1347) => {
                    h1347
                }
            };
            compare(a1348, a1349)
        };
        match m1350 {
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
        let a1353 = f413(match v0.clone() {
            Adt7::C0(h1351, _) => {
                h1351
            }
        })?;
        let a1354 = List::cons(101u64, f413(match v0 {
            Adt7::C0(_, h1352) => {
                h1352
            }
        })?);
        Ok(append_list(a1353, a1354))
    } else {
        f413(match v0 {
            Adt7::C0(h1355, _) => {
                h1355
            }
        })
    }
}

pub fn f337() -> u64 {
    512u64
}

pub fn f338(v0: u64, v1: Adt106, v2: List<Adt106>) -> List<Adt106> {
    List::cons(Adt106::C0(List::cons(v0, List::<u64>::nil())), List::cons(v1, v2))
}

pub fn f339(p1358: List<u64>, p1359: List<u64>) -> R<List<u64>> {
    let m1357 = (p1358, p1359);
    {
        let mut m1357 = m1357;
        loop {
            let step = {
                let (v0, v1) = m1357;
                let m1356 = v0;
                match m1356.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, f163(f414(v2)?, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1357 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(v0: Adt33) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("supportedVersions"), f415(match v0.clone() {
        Adt33::C0(_, h1360, _, _) => {
            h1360
        }
    })), List::cons((Str::lit("capabilities"), f416(match v0.clone() {
        Adt33::C0(_, _, h1361, _) => {
            h1361
        }
    })), {
        let a1364 = f417(Str::lit("instructions"), match v0.clone() {
            Adt33::C0(_, _, _, h1362) => {
                h1362
            }
        });
        let a1365 = f418(match v0 {
            Adt33::C0(h1363, _, _, _) => {
                h1363
            }
        })?;
        append_list(a1364, a1365)
    })))
}

pub fn f341(v0: Adt34) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("tools"), Adt6::C4(f419(match v0.clone() {
        Adt34::C0(_, _, h1366) => {
            h1366
        }
    }))), {
        let a1369 = f417(Str::lit("nextCursor"), match v0.clone() {
            Adt34::C0(_, h1367, _) => {
                h1367
            }
        });
        let a1370 = f418(match v0 {
            Adt34::C0(h1368, _, _) => {
                h1368
            }
        })?;
        append_list(a1369, a1370)
    }))
}

pub fn f342(v0: Option<List<(Str, Adt6)>>) -> R<Option<List<(Str, Adt6)>>> {
    let m1371 = v0;
    match m1371 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            let m1372 = f349(Adt6::C5(v1))?;
            match m1372 {
                None => {
                    Ok(None::<List<(Str, Adt6)>>)
                }
                Some(v2) => {
                    let m1373 = f195(v2);
                    match m1373 {
                        None => {
                            Ok(None::<List<(Str, Adt6)>>)
                        }
                        Some(v3) => {
                            Ok(Some(match v3 {
                                Adt5::C0(h1374) => {
                                    h1374
                                }
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f343(v0: Adt37) -> R<Option<List<(Str, Adt6)>>> {
    let m1376 = f420(match v0.clone() {
        Adt37::C0(h1375, _, _) => {
            h1375
        }
    })?;
    match m1376 {
        None => {
            Ok(None::<List<(Str, Adt6)>>)
        }
        Some(v1) => {
            Ok(Some(List::cons((Str::lit("content"), Adt6::C4(v1)), {
                let a1379 = f421(Str::lit("structuredContent"), match v0.clone() {
                    Adt37::C0(_, h1377, _) => {
                        h1377
                    }
                });
                let a1380 = f422(Str::lit("isError"), match v0 {
                    Adt37::C0(_, _, h1378) => {
                        h1378
                    }
                });
                append_list(a1379, a1380)
            })))
        }
    }
}

pub fn f344(v0: Adt143) -> Str {
    let m1381 = v0;
    match m1381 {
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

pub fn f345(v0: Adt32) -> Adt143 {
    let m1382 = v0;
    match m1382 {
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

pub fn f346(v0: Option<Adt1>) -> List<(Str, Adt6)> {
    let m1383 = v0;
    match m1383 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((f423(), f424(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f347(v0: bool, v1: u64, v2: List<List<u64>>) -> bool {
    if v0 {
        if f425(v2) {
            let a1384 = v1;
            let a1385 = 200u64;
            nat_eq(a1384, a1385)
        } else {
            false
        }
    } else {
        false
    }
}

pub fn f348(v0: u64, v1: List<List<u64>>) -> Adt88 {
    Adt88::C0(v0, f426(), List::<List<u64>>::nil(), f427(v1))
}

pub fn f349(v0: Adt6) -> R<Option<Adt6>> {
    f428(f429(), Adt144::C0(v0), List::<Adt145>::nil())
}

pub fn f350(v0: Adt125) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("jsonrpc"), Adt6::C3(f125())), List::cons((Str::lit("method"), Adt6::C3(Str::lit("notifications/progress"))), List::cons((Str::lit("params"), Adt6::C5(List::cons((Str::lit("progressToken"), f430(match v0.clone() {
        Adt125::C0(h1386, _) => {
            h1386
        }
    })), List::cons((Str::lit("progress"), Adt6::C2({
        let m1389 = match v0.clone() {
            Adt125::C0(_, h1388) => {
                h1388
            }
        };
        match m1389 {
            Adt50::C0(h1387, _, _) => {
                h1387
            }
        }
    })), {
        let a1396 = f431({
            let m1392 = match v0.clone() {
                Adt125::C0(_, h1391) => {
                    h1391
                }
            };
            match m1392 {
                Adt50::C0(_, h1390, _) => {
                    h1390
                }
            }
        });
        let a1397 = f432({
            let m1395 = match v0 {
                Adt125::C0(_, h1394) => {
                    h1394
                }
            };
            match m1395 {
                Adt50::C0(_, _, h1393) => {
                    h1393
                }
            }
        });
        append_list(a1396, a1397)
    })))), List::<(Str, Adt6)>::nil()))))
}

pub fn f351() -> u64 {
    429u64
}

pub fn f352() -> i64 {
    -32602i64
}

pub fn f353() -> i64 {
    -32022i64
}

pub fn f354() -> i64 {
    -32020i64
}

pub fn f355() -> i64 {
    -32601i64
}

pub fn f356(v0: Adt84) -> i64 {
    let m1398 = v0;
    match m1398 {
        Adt84::C0(_) => {
            f352()
        }
        Adt84::C1(_) => {
            f352()
        }
        Adt84::C2(_) => {
            f352()
        }
        Adt84::C3(_) => {
            f352()
        }
        Adt84::C4(_) => {
            f352()
        }
        Adt84::C5(_) => {
            f352()
        }
        Adt84::C6(_) => {
            f433()
        }
        Adt84::C7(_) => {
            f434()
        }
        Adt84::C8(_) => {
            f435()
        }
        Adt84::C9(_) => {
            f433()
        }
    }
}

pub fn f357(v0: Adt127) -> Str {
    let m1399 = v0;
    match m1399 {
        Adt127::C0 => {
            Str::lit("Invalid params: params is required")
        }
        Adt127::C1 => {
            Str::lit("Invalid params: _meta is required")
        }
        Adt127::C2 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/protocolVersion")
        }
        Adt127::C3 => {
            Str::lit("Invalid params: _meta lacks io.modelcontextprotocol/clientCapabilities")
        }
        Adt127::C4 => {
            Str::lit("Invalid params: _meta is not an object")
        }
        Adt127::C5 => {
            Str::lit("Invalid params: io.modelcontextprotocol/protocolVersion is not a string")
        }
        Adt127::C6 => {
            Str::lit("Invalid params: io.modelcontextprotocol/clientCapabilities is not a valid capabilities object")
        }
        Adt127::C7(_) => {
            Str::lit("Invalid params: a required field is missing")
        }
        Adt127::C8(_) => {
            Str::lit("Invalid params: a field has the wrong type")
        }
    }
}

pub fn f358(v0: Adt108) -> Str {
    let m1400 = v0;
    match m1400 {
        Adt108::C0 => {
            Str::lit("Header mismatch: MCP-Protocol-Version header is missing")
        }
        Adt108::C1 => {
            Str::lit("Header mismatch: Mcp-Method header is missing")
        }
        Adt108::C2 => {
            Str::lit("Header mismatch: Mcp-Name header is missing")
        }
        Adt108::C3 => {
            Str::lit("Header mismatch: MCP-Protocol-Version differs from the request body")
        }
        Adt108::C4 => {
            Str::lit("Header mismatch: Mcp-Method differs from the request body")
        }
        Adt108::C5 => {
            Str::lit("Header mismatch: Mcp-Name differs from the request body")
        }
        Adt108::C6(v1) => {
            let m1401 = v1;
            match m1401 {
                Adt109::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version has a malformed value")
                }
                Adt109::C1 => {
                    Str::lit("Header mismatch: Mcp-Method has a malformed value")
                }
                Adt109::C2 => {
                    Str::lit("Header mismatch: Mcp-Name has a malformed value")
                }
            }
        }
        Adt108::C7(v2) => {
            let m1402 = v2;
            match m1402 {
                Adt109::C0 => {
                    Str::lit("Header mismatch: MCP-Protocol-Version is repeated")
                }
                Adt109::C1 => {
                    Str::lit("Header mismatch: Mcp-Method is repeated")
                }
                Adt109::C2 => {
                    Str::lit("Header mismatch: Mcp-Name is repeated")
                }
            }
        }
        Adt108::C8(_) => {
            Str::lit("Header mismatch: a required Mcp-Param header is missing")
        }
        Adt108::C9(_) => {
            Str::lit("Header mismatch: an Mcp-Param header differs from the request body")
        }
        Adt108::C10(_) => {
            Str::lit("Header mismatch: an Mcp-Param header has a malformed value")
        }
    }
}

pub fn f359(v0: List<Str>) -> List<Adt6> {
    f294(f436(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f360(v0: Adt17) -> Adt6 {
    Adt6::C5({
        let a1414 = f437(Str::lit("experimental"), match v0.clone() {
            Adt17::C0(h1403, _, _, _, _) => {
                h1403
            }
        });
        let a1415 = {
            let a1412 = f437(Str::lit("roots"), match v0.clone() {
                Adt17::C0(_, h1404, _, _, _) => {
                    h1404
                }
            });
            let a1413 = {
                let a1410 = f437(Str::lit("sampling"), match v0.clone() {
                    Adt17::C0(_, _, h1405, _, _) => {
                        h1405
                    }
                });
                let a1411 = {
                    let a1408 = f437(Str::lit("elicitation"), match v0.clone() {
                        Adt17::C0(_, _, _, h1406, _) => {
                            h1406
                        }
                    });
                    let a1409 = f437(Str::lit("extensions"), match v0 {
                        Adt17::C0(_, _, _, _, h1407) => {
                            h1407
                        }
                    });
                    append_list(a1408, a1409)
                };
                append_list(a1410, a1411)
            };
            append_list(a1412, a1413)
        };
        append_list(a1414, a1415)
    })
}

pub fn f361(p1418: List<List<u64>>) -> R<bool> {
    let m1417 = p1418;
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let v0 = m1417;
                let m1416 = v0;
                match m1416.uncons() {
                    None => {
                        Err::<List<List<u64>>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        if f438(f140(v1))? {
                            Err::<List<List<u64>>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<List<u64>>, R<bool>>(v2)
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

pub fn f362() -> List<u64> {
    List::cons(116u64, List::cons(101u64, List::cons(120u64, List::cons(116u64, List::cons(47u64, List::cons(101u64, List::cons(118u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(115u64, List::cons(116u64, List::cons(114u64, List::cons(101u64, List::cons(97u64, List::cons(109u64, List::<u64>::nil())))))))))))))))))
}

pub fn f363(v0: Adt0, v1: Fn0, v2: u64, v3: Adt133, v4: bool) -> R<Result<Adt123, Adt126>> {
    if v4 {
        f439(v0, v1, v2, f440(match v3.clone() {
            Adt133::C0(_, h1419, _, _) => {
                h1419
            }
        }, match v3.clone() {
            Adt133::C0(_, _, h1420, _) => {
                h1420
            }
        }, match v3 {
            Adt133::C0(_, _, _, h1421) => {
                h1421
            }
        }))
    } else {
        Ok(Err::<Adt123, Adt126>(Adt126::C3(f441(match v3 {
            Adt133::C0(_, h1422, _, _) => {
                h1422
            }
        }))))
    }
}

pub fn f364(v0: Adt4, v1: Adt134) -> bool {
    let m1423 = v1;
    match m1423 {
        Adt134::C0 => {
            true
        }
        Adt134::C1 => {
            f442(match v0 {
                Adt4::C0(_, _, _, _, _, h1424, _) => {
                    h1424
                }
            })
        }
        Adt134::C2 => {
            f442(match v0 {
                Adt4::C0(_, _, _, _, _, h1425, _) => {
                    h1425
                }
            })
        }
        Adt134::C3 => {
            f443(match v0 {
                Adt4::C0(_, _, _, _, h1426, _, _) => {
                    h1426
                }
            })
        }
        Adt134::C4 => {
            f443(match v0 {
                Adt4::C0(_, _, _, _, h1427, _, _) => {
                    h1427
                }
            })
        }
        Adt134::C5 => {
            f443(match v0 {
                Adt4::C0(_, _, _, _, h1428, _, _) => {
                    h1428
                }
            })
        }
        Adt134::C6 => {
            f442(match v0 {
                Adt4::C0(_, _, _, h1429, _, _, _) => {
                    h1429
                }
            })
        }
        Adt134::C7 => {
            f442(match v0 {
                Adt4::C0(_, _, _, h1430, _, _, _) => {
                    h1430
                }
            })
        }
        Adt134::C8 => {
            f444(match v0 {
                Adt4::C0(_, _, h1431, _, _, _, _) => {
                    h1431
                }
            })
        }
        Adt134::C9 => {
            f445(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1432) => {
                    h1432
                }
            }, f446())
        }
        Adt134::C10 => {
            f445(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1433) => {
                    h1433
                }
            }, f446())
        }
        Adt134::C11 => {
            f445(match v0 {
                Adt4::C0(_, _, _, _, _, _, h1434) => {
                    h1434
                }
            }, f446())
        }
    }
}

pub fn f365(v0: Adt139, v1: Adt137) -> Result<Adt133, Adt126> {
    let m1435 = v1;
    match m1435 {
        Adt137::C0 => {
            Err::<Adt133, Adt126>(Adt126::C3(match v0 {
                Adt139::C0(_, h1436, _, _) => {
                    h1436
                }
            }))
        }
        Adt137::C1(v2) => {
            Ok::<Adt133, Adt126>(Adt133::C0(match v0.clone() {
                Adt139::C0(h1437, _, _, _) => {
                    h1437
                }
            }, v2, match v0.clone() {
                Adt139::C0(_, _, h1438, _) => {
                    h1438
                }
            }, match v0 {
                Adt139::C0(_, _, _, h1439) => {
                    h1439
                }
            }))
        }
    }
}

pub fn f366(v0: Option<Adt107>, v1: List<Str>, v2: Adt103, v3: Result<Adt16, Adt127>) -> R<Adt138> {
    let m1440 = v3;
    match m1440 {
        Ok(v5) => {
            Ok(f447(v1, v2.clone(), v5.clone(), f448(v0, f281(match v2.clone() {
                Adt103::C0(_, h1441, _) => {
                    h1441
                }
            }), match v2.clone() {
                Adt103::C0(_, h1442, _) => {
                    h1442
                }
            }, match v5 {
                Adt16::C0(h1443, _, _, _, _, _) => {
                    h1443
                }
            }, f449(match v2 {
                Adt103::C0(_, _, h1444) => {
                    h1444
                }
            }))?))
        }
        Err(v4) => {
            Ok(Adt138::C1(Adt126::C0(v4)))
        }
    }
}

pub fn f367(v0: Option<Adt5>) -> Result<Adt16, Adt127> {
    let m1445 = v0;
    match m1445 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C0)
        }
        Some(v1) => {
            let m1446 = f176(Str::lit("_meta"), v1);
            match m1446 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C1)
                }
                Adt130::C1(v2) => {
                    let m1447 = f195(v2);
                    match m1447 {
                        None => {
                            Err::<Adt16, Adt127>(Adt127::C4)
                        }
                        Some(v3) => {
                            f450(v3)
                        }
                    }
                }
            }
        }
    }
}

pub fn f368(v0: Adt134) -> Adt140 {
    let m1448 = v0;
    match m1448 {
        Adt134::C0 => {
            Adt140::C0
        }
        Adt134::C1 => {
            Adt140::C0
        }
        Adt134::C2 => {
            Adt140::C1
        }
        Adt134::C3 => {
            Adt140::C0
        }
        Adt134::C4 => {
            Adt140::C2
        }
        Adt134::C5 => {
            Adt140::C0
        }
        Adt134::C6 => {
            Adt140::C0
        }
        Adt134::C7 => {
            Adt140::C1
        }
        Adt134::C8 => {
            Adt140::C0
        }
        Adt134::C9 => {
            Adt140::C3
        }
        Adt134::C10 => {
            Adt140::C3
        }
        Adt134::C11 => {
            Adt140::C3
        }
    }
}

pub fn f369(p1454: List<u64>, p1455: List<u64>) -> Option<List<u64>> {
    let m1453 = (p1454, p1455);
    {
        let mut m1453 = m1453;
        loop {
            let step = {
                let (v0, v1) = m1453;
                let m1449 = v0;
                match m1449.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), Option<List<u64>>>(Some(v1))
                    }
                    Some((v2, v3)) => {
                        let m1450 = v1;
                        match m1450.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), Option<List<u64>>>(None::<List<u64>>)
                            }
                            Some((v4, v5)) => {
                                let a1451 = v4;
                                let a1452 = v2;
                                if nat_eq(a1451, a1452) {
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
                Ok(next) => { m1453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f370() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::cons(98u64, List::cons(97u64, List::cons(115u64, List::cons(101u64, List::cons(54u64, List::cons(52u64, List::cons(63u64, List::<u64>::nil())))))))))
}

pub fn f371() -> List<u64> {
    List::cons(61u64, List::cons(63u64, List::<u64>::nil()))
}

pub fn f372(v0: Adt12, v1: Option<Str>) -> bool {
    let m1456 = v1;
    match m1456 {
        None => {
            false
        }
        Some(v2) => {
            let a1458 = match v0 {
                Adt12::C0(h1457, _, _, _) => {
                    h1457
                }
            };
            let a1459 = v2;
            equal(a1458, a1459)
        }
    }
}

pub fn f373(v0: List<Adt86>, v1: Str) -> R<Adt111> {
    let m1462 = f226(f70(v0, {
        let a1460 = f451();
        let a1461 = f452(v1)?;
        append_list(a1460, a1461)
    })?)?;
    match m1462 {
        Adt96::C0 => {
            Ok(Adt111::C0)
        }
        Adt96::C1 => {
            Ok(Adt111::C1)
        }
        Adt96::C2 => {
            Ok(Adt111::C1)
        }
        Adt96::C3(v2) => {
            let m1463 = f280(v2);
            match m1463 {
                None => {
                    Ok(Adt111::C1)
                }
                Some(v3) => {
                    Ok(Adt111::C2(v3))
                }
            }
        }
    }
}

pub fn f374(v0: i64) -> bool {
    let a1470 = {
        let m1466 = {
            let a1464 = v0;
            let a1465 = 0i64;
            compare(a1464, a1465)
        };
        if match m1466 {
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
            let m1469 = {
                let a1467 = 0i64;
                let a1468 = v0;
                compare(a1467, a1468)
            };
            match m1469 {
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
    bool_not(a1470)
}

pub fn f375(v0: Adt114, v1: u64, v2: Str) -> Adt131 {
    let m1472 = match v0.clone() {
        Adt114::C0(_, h1471, _) => {
            h1471
        }
    };
    match m1472.uncons() {
        None => {
            f247(Adt93::C2(v1))
        }
        Some((v3, v4)) => {
            let m1473 = v3;
            match m1473 {
                Adt119::C0(_) => {
                    f247(Adt93::C2(v1))
                }
                Adt119::C1(v6, _) => {
                    Adt131::C1(Adt114::C0(Adt115::C5, List::cons(Adt119::C1(v6, Some(v2)), v4), match v0 {
                        Adt114::C0(_, _, h1474) => {
                            h1474
                        }
                    }))
                }
            }
        }
    }
}

pub fn f376(v0: u64, v1: List<u64>) -> R<List<u64>> {
    let a1475 = v0;
    let a1476 = 128u64;
    if nat_lt(a1475, a1476) {
        Ok(List::cons(v0, v1))
    } else {
        let a1477 = v0;
        let a1478 = 2048u64;
        if nat_lt(a1477, a1478) {
            Ok(List::cons({
                let a1479 = 128u64;
                let a1480 = f453(v0, 64u64);
                nat_add(a1479, a1480)?
            }, List::cons({
                let a1481 = 192u64;
                let a1482 = f454(v0, 64u64);
                nat_add(a1481, a1482)?
            }, v1)))
        } else {
            let a1483 = v0;
            let a1484 = 65536u64;
            if nat_lt(a1483, a1484) {
                Ok(List::cons({
                    let a1485 = 128u64;
                    let a1486 = f453(v0, 64u64);
                    nat_add(a1485, a1486)?
                }, List::cons({
                    let a1487 = 128u64;
                    let a1488 = f453(f454(v0, 64u64), 64u64);
                    nat_add(a1487, a1488)?
                }, List::cons({
                    let a1489 = 224u64;
                    let a1490 = f454(v0, 4096u64);
                    nat_add(a1489, a1490)?
                }, v1))))
            } else {
                Ok(List::cons({
                    let a1491 = 128u64;
                    let a1492 = f453(v0, 64u64);
                    nat_add(a1491, a1492)?
                }, List::cons({
                    let a1493 = 128u64;
                    let a1494 = f453(f454(v0, 64u64), 64u64);
                    nat_add(a1493, a1494)?
                }, List::cons({
                    let a1495 = 128u64;
                    let a1496 = f453(f454(v0, 4096u64), 64u64);
                    nat_add(a1495, a1496)?
                }, List::cons({
                    let a1497 = 240u64;
                    let a1498 = f454(v0, 262144u64);
                    nat_add(a1497, a1498)?
                }, v1)))))
            }
        }
    }
}

pub fn f377() -> u64 {
    18u64
}

pub fn f378(p1502: List<Str>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1501 = p1502;
    {
        let mut m1501 = m1501;
        loop {
            let step = {
                let v0 = m1501;
                let m1499 = v0;
                match m1499.uncons() {
                    None => {
                        Err::<List<Str>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1500 = f234(v2.clone(), v1.clone());
                        match m1500 {
                            Adt130::C0 => {
                                Ok::<List<Str>, Option<Str>>(v3)
                            }
                            Adt130::C1(_) => {
                                Err::<List<Str>, Option<Str>>(Some(v2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1501 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379() -> List<Str> {
    List::cons(Str::lit("pattern"), List::cons(Str::lit("patternProperties"), List::cons(Str::lit("propertyNames"), List::cons(Str::lit("multipleOf"), List::cons(Str::lit("uniqueItems"), List::cons(Str::lit("contains"), List::cons(Str::lit("minContains"), List::cons(Str::lit("maxContains"), List::cons(Str::lit("prefixItems"), List::cons(Str::lit("dependentRequired"), List::cons(Str::lit("dependentSchemas"), List::cons(Str::lit("dependencies"), List::cons(Str::lit("additionalItems"), List::cons(Str::lit("unevaluatedProperties"), List::cons(Str::lit("unevaluatedItems"), List::cons(Str::lit("$dynamicRef"), List::cons(Str::lit("$dynamicAnchor"), List::cons(Str::lit("$id"), List::cons(Str::lit("$vocabulary"), List::cons(Str::lit("contentEncoding"), List::cons(Str::lit("contentMediaType"), List::cons(Str::lit("contentSchema"), List::<Str>::nil()))))))))))))))))))))))
}

pub fn f380(p1509: List<(Str, u64)>, v1: List<(Str, Adt6)>) -> Option<Str> {
    let m1508 = p1509;
    {
        let mut m1508 = m1508;
        loop {
            let step = {
                let v0 = m1508;
                let m1503 = v0;
                match m1503.uncons() {
                    None => {
                        Err::<List<(Str, u64)>, Option<Str>>(None::<Str>)
                    }
                    Some((v2, v3)) => {
                        let m1505 = f234({
                            let (h1504, _) = v2.clone();
                            h1504
                        }, v1.clone());
                        match m1505 {
                            Adt130::C0 => {
                                Ok::<List<(Str, u64)>, Option<Str>>(v3)
                            }
                            Adt130::C1(v4) => {
                                if f455({
                                    let (_, h1506) = v2;
                                    h1506
                                }, v4) {
                                    Ok::<List<(Str, u64)>, Option<Str>>(v3)
                                } else {
                                    Err::<List<(Str, u64)>, Option<Str>>(Some({
                                        let (h1507, _) = v2;
                                        h1507
                                    }))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1508 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f381() -> List<(Str, u64)> {
    List::cons((Str::lit("$schema"), 0u64), List::cons((Str::lit("$ref"), 0u64), List::cons((Str::lit("$anchor"), 0u64), List::cons((Str::lit("$comment"), 0u64), List::cons((Str::lit("title"), 0u64), List::cons((Str::lit("description"), 0u64), List::cons((Str::lit("format"), 0u64), List::cons((Str::lit("type"), 7u64), List::cons((Str::lit("enum"), 3u64), List::cons((Str::lit("required"), 5u64), List::cons((Str::lit("properties"), 4u64), List::cons((Str::lit("$defs"), 4u64), List::cons((Str::lit("allOf"), 8u64), List::cons((Str::lit("anyOf"), 8u64), List::cons((Str::lit("oneOf"), 8u64), List::cons((Str::lit("minimum"), 1u64), List::cons((Str::lit("maximum"), 1u64), List::cons((Str::lit("exclusiveMinimum"), 1u64), List::cons((Str::lit("exclusiveMaximum"), 1u64), List::cons((Str::lit("minItems"), 2u64), List::cons((Str::lit("maxItems"), 2u64), List::cons((Str::lit("minLength"), 2u64), List::cons((Str::lit("maxLength"), 2u64), List::cons((Str::lit("minProperties"), 2u64), List::cons((Str::lit("maxProperties"), 2u64), List::cons((Str::lit("deprecated"), 6u64), List::cons((Str::lit("readOnly"), 6u64), List::cons((Str::lit("writeOnly"), 6u64), List::<(Str, u64)>::nil()))))))))))))))))))))))))))))
}

pub fn f382(v0: Str, v1: Adt6, v2: List<(Str, Adt6)>) -> Option<Adt91> {
    let m1510 = f234(Str::lit("$ref"), v2);
    match m1510 {
        Adt130::C0 => {
            None::<Adt91>
        }
        Adt130::C1(v3) => {
            let m1511 = f231(v3);
            match m1511 {
                None => {
                    None::<Adt91>
                }
                Some(v4) => {
                    let m1512 = f400(v1, v4.clone());
                    match m1512 {
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

pub fn f383(v0: Str, v1: List<(Str, Adt6)>, v2: Adt6, v3: bool, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    if v3 {
        let m1513 = f231(v2);
        match m1513 {
            None => {
                Ok(Adt142::C0(Adt91::C7(v0)))
            }
            Some(v6) => {
                f456(v0, v1, v6.clone(), f113(v6)?, v4, v5)
            }
        }
    } else {
        Ok(Adt142::C0(Adt91::C10(v0)))
    }
}

pub fn f384(v0: Str, v1: List<(Str, Adt6)>, v2: bool, v3: List<Str>, v4: List<Adt104>) -> List<Adt104> {
    let m1514 = f234(v0.clone(), v1);
    match m1514 {
        Adt130::C0 => {
            v4
        }
        Adt130::C1(v5) => {
            let m1515 = f195(v5);
            match m1515 {
                None => {
                    v4
                }
                Some(v6) => {
                    let a1516 = v0;
                    let a1517 = Str::lit("properties");
                    if equal(a1516, a1517) {
                        f457(match v6 {
                            Adt5::C0(h1518) => {
                                h1518
                            }
                        }, v2, v3, v4)
                    } else {
                        f458(match v6 {
                            Adt5::C0(h1519) => {
                                h1519
                            }
                        }, v4)
                    }
                }
            }
        }
    }
}

pub fn f385(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1520 = f234(v0, v1);
    match m1520 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            List::cons(Adt104::C0(v3, false, List::<Str>::nil()), v2)
        }
    }
}

pub fn f386(v0: Str, v1: List<(Str, Adt6)>, v2: List<Adt104>) -> List<Adt104> {
    let m1521 = f234(v0, v1);
    match m1521 {
        Adt130::C0 => {
            v2
        }
        Adt130::C1(v3) => {
            let m1522 = f259(v3);
            match m1522 {
                None => {
                    v2
                }
                Some(v4) => {
                    f459(v4, v2)
                }
            }
        }
    }
}

pub fn f387(v0: Str) -> bool {
    let a1523 = v0.clone();
    let a1524 = Str::lit("null");
    if equal(a1523, a1524) {
        true
    } else {
        let a1525 = v0.clone();
        let a1526 = Str::lit("boolean");
        if equal(a1525, a1526) {
            true
        } else {
            let a1527 = v0.clone();
            let a1528 = Str::lit("object");
            if equal(a1527, a1528) {
                true
            } else {
                let a1529 = v0.clone();
                let a1530 = Str::lit("array");
                if equal(a1529, a1530) {
                    true
                } else {
                    let a1531 = v0.clone();
                    let a1532 = Str::lit("number");
                    if equal(a1531, a1532) {
                        true
                    } else {
                        let a1533 = v0.clone();
                        let a1534 = Str::lit("string");
                        if equal(a1533, a1534) {
                            true
                        } else {
                            let a1535 = v0;
                            let a1536 = Str::lit("integer");
                            equal(a1535, a1536)
                        }
                    }
                }
            }
        }
    }
}

pub fn f388(v0: Str, v1: Adt6) -> R<bool> {
    let a1537 = v0.clone();
    let a1538 = f460(v1.clone());
    if equal(a1537, a1538) {
        Ok(true)
    } else {
        let a1539 = v0;
        let a1540 = Str::lit("integer");
        if equal(a1539, a1540) {
            let m1541 = f392(v1);
            match m1541 {
                None => {
                    Ok(false)
                }
                Some(v2) => {
                    f461(v2)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f389(p1544: List<Adt6>, v1: Adt6, p1545: bool) -> R<Adt136> {
    let m1543 = (p1544, p1545);
    {
        let mut m1543 = m1543;
        loop {
            let step = {
                let (v0, v2) = m1543;
                let m1542 = v0;
                match m1542.uncons() {
                    None => {
                        if v2 {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C0))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C1))
                        }
                    }
                    Some((v3, v4)) => {
                        if f387(f462(v3.clone())) {
                            Ok::<(List<Adt6>, bool), R<Adt136>>((v4, if v2 {
                                true
                            } else {
                                f388(f462(v3), v1.clone())?
                            }))
                        } else {
                            Err::<(List<Adt6>, bool), R<Adt136>>(Ok(Adt136::C2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1543 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(p1549: List<Adt6>, v1: Adt6) -> R<Adt136> {
    let m1548 = p1549;
    {
        let mut m1548 = m1548;
        loop {
            let step = {
                let v0 = m1548;
                let m1546 = v0;
                match m1546.uncons() {
                    None => {
                        Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C1))
                    }
                    Some((v2, v3)) => {
                        let m1547 = f391(v2, v1.clone())?;
                        match m1547 {
                            Adt146::C0 => {
                                Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C0))
                            }
                            Adt146::C1 => {
                                Ok::<List<Adt6>, R<Adt136>>(v3)
                            }
                            Adt146::C2 => {
                                Err::<List<Adt6>, R<Adt136>>(Ok(Adt136::C2))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1548 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: Adt6, v1: Adt6) -> R<Adt146> {
    f463(f464(), List::cons((v0, v1), List::<(Adt6, Adt6)>::nil()))
}

pub fn f392(v0: Adt6) -> Option<Adt7> {
    let m1550 = v0;
    match m1550 {
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

pub fn f393(v0: u64, v1: Adt147) -> bool {
    let a1551 = v0;
    let a1552 = 0u64;
    if nat_eq(a1551, a1552) {
        let a1553 = f465(v1);
        bool_not(a1553)
    } else {
        let a1554 = v0;
        let a1555 = 1u64;
        if nat_eq(a1554, a1555) {
            let a1556 = f409(v1);
            bool_not(a1556)
        } else {
            let a1557 = v0;
            let a1558 = 2u64;
            if nat_eq(a1557, a1558) {
                f409(v1)
            } else {
                f465(v1)
            }
        }
    }
}

pub fn f394(v0: Adt7, v1: Adt7) -> R<Adt147> {
    f466(v0.clone(), v1.clone(), f467(match v0 {
        Adt7::C0(h1559, _) => {
            h1559
        }
    }), f467(match v1 {
        Adt7::C0(h1560, _) => {
            h1560
        }
    }))
}

pub fn f395(v0: Str, v1: bool, v2: List<(Str, Adt6)>, v3: u64) -> R<Adt136> {
    let m1561 = f234(v0, v2);
    match m1561 {
        Adt130::C0 => {
            Ok(Adt136::C0)
        }
        Adt130::C1(v4) => {
            let m1562 = f468(v4);
            match m1562 {
                None => {
                    Ok(Adt136::C2)
                }
                Some(v5) => {
                    let m1565 = {
                        let a1563 = v5;
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
                        Ok(Adt136::C2)
                    } else {
                        if v1 {
                            if f465(f469(f79(v3, 0i64)?, v5)) {
                                Ok(Adt136::C1)
                            } else {
                                Ok(Adt136::C0)
                            }
                        } else {
                            if f409(f469(f79(v3, 0i64)?, v5)) {
                                Ok(Adt136::C1)
                            } else {
                                Ok(Adt136::C0)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f396(v0: Str) -> R<u64> {
    f470(f87({
        let a1566 = v0;
        utf8_encode(a1566)
    })?, 0u64)
}

pub fn f397(p1571: List<Adt6>, p1572: u64) -> R<u64> {
    let m1570 = (p1571, p1572);
    {
        let mut m1570 = m1570;
        loop {
            let step = {
                let (v0, v1) = m1570;
                let m1567 = v0;
                match m1567.uncons() {
                    None => {
                        Err::<(List<Adt6>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<Adt6>, u64), R<u64>>((v3, {
                            let a1568 = v1;
                            let a1569 = 1u64;
                            nat_add(a1568, a1569)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1570 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(p1577: List<(Str, Adt6)>, p1578: u64) -> R<u64> {
    let m1576 = (p1577, p1578);
    {
        let mut m1576 = m1576;
        loop {
            let step = {
                let (v0, v1) = m1576;
                let m1573 = v0;
                match m1573.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, u64), R<u64>>(Ok(v1))
                    }
                    Some((_, v3)) => {
                        Ok::<(List<(Str, Adt6)>, u64), R<u64>>((v3, {
                            let a1574 = v1;
                            let a1575 = 1u64;
                            nat_add(a1574, a1575)?
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(p1582: List<Adt6>, v1: Adt5) -> Adt136 {
    let m1581 = p1582;
    {
        let mut m1581 = m1581;
        loop {
            let step = {
                let v0 = m1581;
                let m1579 = v0;
                match m1579.uncons() {
                    None => {
                        Err::<List<Adt6>, Adt136>(Adt136::C0)
                    }
                    Some((v2, v3)) => {
                        let m1580 = f231(v2);
                        match m1580 {
                            None => {
                                Err::<List<Adt6>, Adt136>(Adt136::C2)
                            }
                            Some(v4) => {
                                if f235(v4, v1.clone()) {
                                    Ok::<List<Adt6>, Adt136>(v3)
                                } else {
                                    Err::<List<Adt6>, Adt136>(Adt136::C1)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1581 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f400(v0: Adt6, v1: Str) -> Option<Adt6> {
    let a1583 = v1.clone();
    let a1584 = Str::lit("#");
    if equal(a1583, a1584) {
        Some(v0)
    } else {
        let m1585 = f195(v0);
        match m1585 {
            None => {
                None::<Adt6>
            }
            Some(v2) => {
                let m1586 = f176(Str::lit("$defs"), v2);
                match m1586 {
                    Adt130::C0 => {
                        None::<Adt6>
                    }
                    Adt130::C1(v3) => {
                        let m1587 = f195(v3);
                        match m1587 {
                            None => {
                                None::<Adt6>
                            }
                            Some(v4) => {
                                f471(v1, match v4 {
                                    Adt5::C0(h1588) => {
                                        h1588
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

pub fn f401(p1595: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: List<Adt99>, p1596: List<Adt101>) -> List<Adt101> {
    let m1594 = (p1595, p1596);
    {
        let mut m1594 = m1594;
        loop {
            let step = {
                let (v0, v3) = m1594;
                let m1589 = v0;
                match m1589.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        let m1591 = f234({
                            let (h1590, _) = v4.clone();
                            h1590
                        }, v1.clone());
                        match m1591 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, v3))
                            }
                            Adt130::C1(v6) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0({
                                    let (_, h1592) = v4;
                                    h1592
                                }, v6, List::cons(Adt99::C0({
                                    let (h1593, _) = v4;
                                    h1593
                                }), v2.clone())), v3)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1594 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(p1601: List<(Str, Adt6)>, v1: List<(Str, Adt6)>) -> bool {
    let m1600 = p1601;
    {
        let mut m1600 = m1600;
        loop {
            let step = {
                let v0 = m1600;
                let m1597 = v0;
                match m1597.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let m1599 = f234({
                            let (h1598, _) = v2;
                            h1598
                        }, v1.clone());
                        match m1599 {
                            Adt130::C0 => {
                                Err::<List<(Str, Adt6)>, bool>(true)
                            }
                            Adt130::C1(_) => {
                                Ok::<List<(Str, Adt6)>, bool>(v3)
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

pub fn f403(v0: List<(Str, Adt6)>) -> List<(Str, Adt6)> {
    let m1602 = f234(Str::lit("properties"), v0);
    match m1602 {
        Adt130::C0 => {
            List::<(Str, Adt6)>::nil()
        }
        Adt130::C1(v1) => {
            let m1603 = f195(v1);
            match m1603 {
                None => {
                    List::<(Str, Adt6)>::nil()
                }
                Some(v2) => {
                    match v2 {
                        Adt5::C0(h1604) => {
                            h1604
                        }
                    }
                }
            }
        }
    }
}

pub fn f404(p1611: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, v2: Adt6, v3: List<Adt99>, p1612: List<Adt101>) -> List<Adt101> {
    let m1610 = (p1611, p1612);
    {
        let mut m1610 = m1610;
        loop {
            let step = {
                let (v0, v4) = m1610;
                let m1605 = v0;
                match m1605.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>(v4)
                    }
                    Some((v5, v6)) => {
                        let m1607 = f234({
                            let (h1606, _) = v5.clone();
                            h1606
                        }, v1.clone());
                        match m1607 {
                            Adt130::C0 => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v6, List::cons(Adt101::C0(v2.clone(), {
                                    let (_, h1608) = v5;
                                    h1608
                                }, List::cons(Adt99::C0({
                                    let (h1609, _) = v5;
                                    h1609
                                }), v3.clone())), v4)))
                            }
                            Adt130::C1(_) => {
                                Ok::<(List<(Str, Adt6)>, List<Adt101>), List<Adt101>>((v6, v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(p1617: List<Adt6>, p1618: u64, v2: Adt6, v3: List<Adt99>, p1619: List<Adt101>) -> R<List<Adt101>> {
    let m1616 = (p1617, (p1618, p1619));
    {
        let mut m1616 = m1616;
        loop {
            let step = {
                let (v0, (v1, v4)) = m1616;
                let m1613 = v0;
                match m1613.uncons() {
                    None => {
                        Err::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>(Ok(v4))
                    }
                    Some((v5, v6)) => {
                        Ok::<(List<Adt6>, (u64, List<Adt101>)), R<List<Adt101>>>((v6, ({
                            let a1614 = v1;
                            let a1615 = 1u64;
                            nat_add(a1614, a1615)?
                        }, List::cons(Adt101::C0(v2.clone(), v5, List::cons(Adt99::C1(v1), v3.clone())), v4))))
                    }
                }
            };
            match step {
                Ok(next) => { m1616 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p1622: List<Adt6>, v1: Adt6, v2: List<Adt99>, p1623: List<Adt101>) -> List<Adt101> {
    let m1621 = (p1622, p1623);
    {
        let mut m1621 = m1621;
        loop {
            let step = {
                let (v0, v3) = m1621;
                let m1620 = v0;
                match m1620.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt101>), List<Adt101>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<Adt6>, List<Adt101>), List<Adt101>>((v5, List::cons(Adt101::C0(v4, v1.clone(), v2.clone()), v3)))
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

pub fn f407(v0: u64, v1: u64, v2: u64, v3: u64, v4: bool) -> R<Option<List<u64>>> {
    let m1624 = f472(v0)?;
    match m1624 {
        None => {
            Ok(None::<List<u64>>)
        }
        Some(v5) => {
            let m1625 = f472(v1)?;
            match m1625 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v6) => {
                    let a1626 = v2;
                    let a1627 = 61u64;
                    if nat_eq(a1626, a1627) {
                        f473(v5, v6, v4, v3)
                    } else {
                        let m1628 = f472(v2)?;
                        match m1628 {
                            None => {
                                Ok(None::<List<u64>>)
                            }
                            Some(v7) => {
                                let a1629 = v3;
                                let a1630 = 61u64;
                                if nat_eq(a1629, a1630) {
                                    f474(v5, v6, v7, v4)
                                } else {
                                    let m1631 = f472(v3)?;
                                    match m1631 {
                                        None => {
                                            Ok(None::<List<u64>>)
                                        }
                                        Some(v8) => {
                                            Ok(Some(f475(v5, v6, v7, v8)?))
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

pub fn f408(v0: List<u64>) -> bool {
    let m1632 = v0;
    match m1632.uncons() {
        None => {
            true
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f409(v0: Adt147) -> bool {
    let m1633 = v0;
    match m1633 {
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

pub fn f410(p1638: Bytes, p1639: List<Bytes>) -> Bytes {
    let m1637 = (p1638, p1639);
    {
        let mut m1637 = m1637;
        loop {
            let step = {
                let (v0, v1) = m1637;
                let m1634 = v1;
                match m1634.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), Bytes>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(Bytes, List<Bytes>), Bytes>(({
                            let a1635 = v2;
                            let a1636 = v0;
                            append_bytes(a1635, a1636)
                        }, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m1637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(p1642: List<Bytes>, p1643: List<u64>) -> List<Bytes> {
    let m1641 = (p1642, p1643);
    {
        let mut m1641 = m1641;
        loop {
            let step = {
                let (v0, v1) = m1641;
                let m1640 = v1;
                match m1640.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<u64>), List<Bytes>>(v0)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<u64>), List<Bytes>>((f476(f477(v2), v0), v3))
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

pub fn f412() -> i64 {
    999999999999999999i64
}

pub fn f413(v0: i64) -> R<List<u64>> {
    let m1646 = {
        let a1644 = v0;
        let a1645 = 0i64;
        compare(a1644, a1645)
    };
    if match m1646 {
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
        Ok(List::cons(45u64, f478(20u64, {
            let a1647 = v0;
            int_neg(a1647)?
        }, List::<u64>::nil())?))
    } else {
        f478(20u64, v0, List::<u64>::nil())
    }
}

pub fn f414(v0: u64) -> R<List<u64>> {
    let a1648 = v0;
    let a1649 = 34u64;
    if nat_eq(a1648, a1649) {
        Ok(List::cons(92u64, List::cons(34u64, List::<u64>::nil())))
    } else {
        let a1650 = v0;
        let a1651 = 92u64;
        if nat_eq(a1650, a1651) {
            Ok(List::cons(92u64, List::cons(92u64, List::<u64>::nil())))
        } else {
            let a1652 = v0;
            let a1653 = 8u64;
            if nat_eq(a1652, a1653) {
                Ok(List::cons(92u64, List::cons(98u64, List::<u64>::nil())))
            } else {
                let a1654 = v0;
                let a1655 = 9u64;
                if nat_eq(a1654, a1655) {
                    Ok(List::cons(92u64, List::cons(116u64, List::<u64>::nil())))
                } else {
                    let a1656 = v0;
                    let a1657 = 10u64;
                    if nat_eq(a1656, a1657) {
                        Ok(List::cons(92u64, List::cons(110u64, List::<u64>::nil())))
                    } else {
                        let a1658 = v0;
                        let a1659 = 12u64;
                        if nat_eq(a1658, a1659) {
                            Ok(List::cons(92u64, List::cons(102u64, List::<u64>::nil())))
                        } else {
                            let a1660 = v0;
                            let a1661 = 13u64;
                            if nat_eq(a1660, a1661) {
                                Ok(List::cons(92u64, List::cons(114u64, List::<u64>::nil())))
                            } else {
                                let a1662 = v0;
                                let a1663 = 32u64;
                                if nat_lt(a1662, a1663) {
                                    Ok(List::cons(92u64, List::cons(117u64, List::cons(48u64, List::cons(48u64, List::cons(f479(f454(v0, 16u64))?, List::cons(f479(f453(v0, 16u64))?, List::<u64>::nil())))))))
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

pub fn f415(v0: List<Str>) -> Adt6 {
    Adt6::C4(f359(v0))
}

pub fn f416(v0: Adt4) -> Adt6 {
    Adt6::C5({
        let a1681 = f437(Str::lit("experimental"), match v0.clone() {
            Adt4::C0(h1664, _, _, _, _, _, _) => {
                h1664
            }
        });
        let a1682 = {
            let a1679 = f437(Str::lit("logging"), match v0.clone() {
                Adt4::C0(_, h1665, _, _, _, _, _) => {
                    h1665
                }
            });
            let a1680 = {
                let a1677 = f437(Str::lit("completions"), match v0.clone() {
                    Adt4::C0(_, _, h1666, _, _, _, _) => {
                        h1666
                    }
                });
                let a1678 = {
                    let a1675 = f480(Str::lit("prompts"), match v0.clone() {
                        Adt4::C0(_, _, _, h1667, _, _, _) => {
                            h1667
                        }
                    });
                    let a1676 = {
                        let a1673 = f481(match v0.clone() {
                            Adt4::C0(_, _, _, _, h1668, _, _) => {
                                h1668
                            }
                        });
                        let a1674 = {
                            let a1671 = f480(Str::lit("tools"), match v0.clone() {
                                Adt4::C0(_, _, _, _, _, h1669, _) => {
                                    h1669
                                }
                            });
                            let a1672 = f437(Str::lit("extensions"), match v0 {
                                Adt4::C0(_, _, _, _, _, _, h1670) => {
                                    h1670
                                }
                            });
                            append_list(a1671, a1672)
                        };
                        append_list(a1673, a1674)
                    };
                    append_list(a1675, a1676)
                };
                append_list(a1677, a1678)
            };
            append_list(a1679, a1680)
        };
        append_list(a1681, a1682)
    })
}

pub fn f417(v0: Str, v1: Option<Str>) -> List<(Str, Adt6)> {
    let m1683 = v1;
    match m1683 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C3(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f418(v0: Adt10) -> R<List<(Str, Adt6)>> {
    Ok(List::cons((Str::lit("ttlMs"), f482(match v0.clone() {
        Adt10::C0(h1684, _) => {
            h1684
        }
    })?), List::cons((Str::lit("cacheScope"), Adt6::C3(f483(match v0 {
        Adt10::C0(_, h1685) => {
            h1685
        }
    }))), List::<(Str, Adt6)>::nil())))
}

pub fn f419(v0: List<Adt35>) -> List<Adt6> {
    f294(f484(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f420(v0: List<Adt38>) -> R<Option<List<Adt6>>> {
    let m1686 = f485(v0, List::<Adt6>::nil())?;
    match m1686 {
        None => {
            Ok(None::<List<Adt6>>)
        }
        Some(v1) => {
            Ok(Some(f294(v1, List::<Adt6>::nil())))
        }
    }
}

pub fn f421(v0: Str, v1: Option<Adt6>) -> List<(Str, Adt6)> {
    let m1687 = v1;
    match m1687 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, v2), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f422(v0: Str, v1: Option<bool>) -> List<(Str, Adt6)> {
    let m1688 = v1;
    match m1688 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C1(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f423() -> Str {
    Str::lit("io.modelcontextprotocol/serverInfo")
}

pub fn f424(v0: Adt1) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt1::C0(h1689, _, _, _, _, _) => {
            h1689
        }
    })), List::cons((Str::lit("version"), Adt6::C3(match v0.clone() {
        Adt1::C0(_, _, h1690, _, _, _) => {
            h1690
        }
    })), {
        let a1699 = f417(Str::lit("title"), match v0.clone() {
            Adt1::C0(_, h1691, _, _, _, _) => {
                h1691
            }
        });
        let a1700 = {
            let a1697 = f417(Str::lit("description"), match v0.clone() {
                Adt1::C0(_, _, _, h1692, _, _) => {
                    h1692
                }
            });
            let a1698 = {
                let a1695 = f417(Str::lit("websiteUrl"), match v0.clone() {
                    Adt1::C0(_, _, _, _, h1693, _) => {
                        h1693
                    }
                });
                let a1696 = f486(match v0 {
                    Adt1::C0(_, _, _, _, _, h1694) => {
                        h1694
                    }
                });
                append_list(a1695, a1696)
            };
            append_list(a1697, a1698)
        };
        append_list(a1699, a1700)
    })))
}

pub fn f425(v0: List<List<u64>>) -> bool {
    let m1701 = v0;
    match m1701.uncons() {
        None => {
            false
        }
        Some((_, _)) => {
            true
        }
    }
}

pub fn f426() -> List<Adt86> {
    List::cons(Adt86::C0(f487(), f362()), List::cons(Adt86::C0(f488(), f489()), List::cons(Adt86::C0(f490(), f491()), List::<Adt86>::nil())))
}

pub fn f427(v0: List<List<u64>>) -> List<u64> {
    f492(f38(v0, List::<List<u64>>::nil()), List::<u64>::nil())
}

pub fn f428(p1716: u64, p1717: Adt144, p1718: List<Adt145>) -> R<Option<Adt6>> {
    let m1715 = (p1716, (p1717, p1718));
    {
        let mut m1715 = m1715;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1715;
                let m1702 = v0;
                if m1702 == 0 {
                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(None::<Adt6>))
                } else {
                    let v3 = m1702 - 1;
                    let m1703 = v1;
                    match m1703 {
                        Adt144::C0(v4) => {
                            let m1704 = v4.clone();
                            match m1704 {
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
                                    let v8: Str = f493(v7)?;
                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C3(v8)), v2)))
                                }
                                Adt6::C4(v9) => {
                                    let m1705 = v9;
                                    match m1705.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v10, v11)) => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v10), List::cons(Adt145::C0(List::<Adt6>::nil(), v11), v2))))
                                        }
                                    }
                                }
                                Adt6::C5(v12) => {
                                    let m1706 = v12;
                                    match m1706.uncons() {
                                        None => {
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(v4), v2)))
                                        }
                                        Some((v13, v14)) => {
                                            let v15: Str = f493({
                                                let (h1707, _) = v13;
                                                h1707
                                            })?;
                                            Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                let (_, h1708) = v13;
                                                h1708
                                            }), List::cons(Adt145::C1(v15, List::<(Str, Adt6)>::nil(), v14), v2))))
                                        }
                                    }
                                }
                            }
                        }
                        Adt144::C1(v16) => {
                            let m1709 = v2;
                            match m1709.uncons() {
                                None => {
                                    Err::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>(Ok(Some(v16)))
                                }
                                Some((v17, v18)) => {
                                    let m1710 = v17;
                                    match m1710 {
                                        Adt145::C0(v19, v20) => {
                                            let v21: List<Adt6> = List::cons(v16, v19);
                                            let m1711 = v20;
                                            match m1711.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C4(f294(v21, List::<Adt6>::nil()))), v18)))
                                                }
                                                Some((v22, v23)) => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0(v22), List::cons(Adt145::C0(v21, v23), v18))))
                                                }
                                            }
                                        }
                                        Adt145::C1(v24, v25, v26) => {
                                            let v27: List<(Str, Adt6)> = List::cons((v24, v16), v25);
                                            let m1712 = v26;
                                            match m1712.uncons() {
                                                None => {
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C1(Adt6::C5(f296(v27, List::<(Str, Adt6)>::nil()))), v18)))
                                                }
                                                Some((v28, v29)) => {
                                                    let v30: Str = f493({
                                                        let (h1713, _) = v28;
                                                        h1713
                                                    })?;
                                                    Ok::<(u64, (Adt144, List<Adt145>)), R<Option<Adt6>>>((v3, (Adt144::C0({
                                                        let (_, h1714) = v28;
                                                        h1714
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
                Ok(next) => { m1715 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429() -> u64 {
    67108864u64
}

pub fn f430(v0: Adt19) -> Adt6 {
    let m1719 = v0;
    match m1719 {
        Adt19::C0(v1) => {
            Adt6::C3(v1)
        }
        Adt19::C1(v2) => {
            Adt6::C2(Adt7::C0(v2, 0i64))
        }
    }
}

pub fn f431(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m1720 = v0;
    match m1720 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("total"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f432(v0: Option<Str>) -> List<(Str, Adt6)> {
    let m1721 = v0;
    match m1721 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("message"), Adt6::C3(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f433() -> i64 {
    -32603i64
}

pub fn f434() -> i64 {
    429i64
}

pub fn f435() -> i64 {
    -32021i64
}

pub fn f436(p1724: List<Str>, p1725: List<Adt6>) -> List<Adt6> {
    let m1723 = (p1724, p1725);
    {
        let mut m1723 = m1723;
        loop {
            let step = {
                let (v0, v1) = m1723;
                let m1722 = v0;
                match m1722.uncons() {
                    None => {
                        Err::<(List<Str>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(v2), v1)))
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

pub fn f437(v0: Str, v1: Option<Adt5>) -> List<(Str, Adt6)> {
    let m1726 = v1;
    match m1726 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, Adt6::C5(match v2 {
                Adt5::C0(h1727) => {
                    h1727
                }
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f438(v0: List<u64>) -> R<bool> {
    let m1728 = v0;
    match m1728.uncons() {
        None => {
            Ok(false)
        }
        Some((v1, v2)) => {
            let m1729 = v2;
            match m1729.uncons() {
                None => {
                    Ok(false)
                }
                Some((v3, v4)) => {
                    let a1730 = f184(v1)?;
                    let a1731 = 113u64;
                    if if nat_eq(a1730, a1731) {
                        let a1732 = v3;
                        let a1733 = 61u64;
                        nat_eq(a1732, a1733)
                    } else {
                        false
                    } {
                        Ok(f494(v4))
                    } else {
                        Ok(false)
                    }
                }
            }
        }
    }
}

pub fn f439(v0: Adt0, v1: Fn0, v2: u64, v3: Result<Adt14, Adt127>) -> R<Result<Adt123, Adt126>> {
    let m1734 = v3;
    match m1734 {
        Ok(v5) => {
            f495(v0, v1, v2, v5)
        }
        Err(v4) => {
            Ok(Err::<Adt123, Adt126>(Adt126::C0(v4)))
        }
    }
}

pub fn f440(v0: Adt134, v1: Adt16, v2: Adt5) -> Result<Adt14, Adt127> {
    let m1735 = v0;
    match m1735 {
        Adt134::C0 => {
            f496(v1, v2)
        }
        Adt134::C1 => {
            f497(v1, v2)
        }
        Adt134::C2 => {
            f498(v1, v2)
        }
        Adt134::C3 => {
            f499(v1, v2)
        }
        Adt134::C4 => {
            f500(v1, v2)
        }
        Adt134::C5 => {
            f501(v1, v2)
        }
        Adt134::C6 => {
            f502(v1, v2)
        }
        Adt134::C7 => {
            f503(v1, v2)
        }
        Adt134::C8 => {
            f504(v1, v2)
        }
        Adt134::C9 => {
            f505(v1, v2)
        }
        Adt134::C10 => {
            f506(v1, v2)
        }
        Adt134::C11 => {
            f507(v1, v2)
        }
    }
}

pub fn f441(v0: Adt134) -> Str {
    let m1736 = v0;
    match m1736 {
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

pub fn f442(v0: Option<Adt8>) -> bool {
    let m1737 = v0;
    match m1737 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f443(v0: Option<Adt9>) -> bool {
    let m1738 = v0;
    match m1738 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f444(v0: Option<Adt5>) -> bool {
    let m1739 = v0;
    match m1739 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f445(v0: Option<Adt5>, v1: Str) -> bool {
    let m1740 = v0;
    match m1740 {
        None => {
            false
        }
        Some(v2) => {
            f235(v1, v2)
        }
    }
}

pub fn f446() -> Str {
    Str::lit("io.modelcontextprotocol/tasks")
}

pub fn f447(v0: List<Str>, v1: Adt103, v2: Adt16, v3: Option<Adt108>) -> Adt138 {
    let m1741 = v3;
    match m1741 {
        None => {
            f508(v1, v2.clone(), f509(match v2 {
                Adt16::C0(h1742, _, _, _, _, _) => {
                    h1742
                }
            }, v0))
        }
        Some(v4) => {
            Adt138::C1(Adt126::C2(v4))
        }
    }
}

pub fn f448(v0: Option<Adt107>, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m1743 = v0;
    match m1743 {
        None => {
            Ok(None::<Adt108>)
        }
        Some(v5) => {
            let m1745 = match v5.clone() {
                Adt107::C0(_, _, _, h1744, _) => {
                    h1744
                }
            };
            match m1745 {
                None => {
                    f510(v5, v1, v2, v3, v4)
                }
                Some(v6) => {
                    Ok(Some(v6))
                }
            }
        }
    }
}

pub fn f449(v0: Option<Adt5>) -> Adt5 {
    let m1746 = v0;
    match m1746 {
        None => {
            f511()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f450(v0: Adt5) -> Result<Adt16, Adt127> {
    let m1747 = f512(v0.clone());
    match m1747 {
        Adt148::C0 => {
            f513(v0.clone(), f176(f514(), v0.clone()), f176(f515(), v0))
        }
        Adt148::C1 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt148::C2 => {
            Err::<Adt16, Adt127>(Adt127::C3)
        }
    }
}

pub fn f451() -> List<u64> {
    List::cons(109u64, List::cons(99u64, List::cons(112u64, List::cons(45u64, List::cons(112u64, List::cons(97u64, List::cons(114u64, List::cons(97u64, List::cons(109u64, List::cons(45u64, List::<u64>::nil()))))))))))
}

pub fn f452(v0: Str) -> R<List<u64>> {
    f87({
        let a1748 = v0;
        utf8_encode(a1748)
    })
}

pub fn f453(v0: u64, v1: u64) -> u64 {
    let a1749 = v0;
    let a1750 = v1;
    let a1751 = 0u64;
    nat_rem(a1749, a1750, a1751)
}

pub fn f454(v0: u64, v1: u64) -> u64 {
    let a1752 = v0;
    let a1753 = v1;
    let a1754 = 0u64;
    nat_quot(a1752, a1753, a1754)
}

pub fn f455(v0: u64, v1: Adt6) -> bool {
    let a1755 = v0;
    let a1756 = 0u64;
    if nat_eq(a1755, a1756) {
        f516(v1)
    } else {
        let a1757 = v0;
        let a1758 = 1u64;
        if nat_eq(a1757, a1758) {
            f517(v1)
        } else {
            let a1759 = v0;
            let a1760 = 2u64;
            if nat_eq(a1759, a1760) {
                f518(v1)
            } else {
                let a1761 = v0;
                let a1762 = 3u64;
                if nat_eq(a1761, a1762) {
                    f519(v1)
                } else {
                    let a1763 = v0;
                    let a1764 = 4u64;
                    if nat_eq(a1763, a1764) {
                        f520(v1)
                    } else {
                        let a1765 = v0;
                        let a1766 = 5u64;
                        if nat_eq(a1765, a1766) {
                            f521(v1)
                        } else {
                            let a1767 = v0;
                            let a1768 = 6u64;
                            if nat_eq(a1767, a1768) {
                                f522(v1)
                            } else {
                                let a1769 = v0;
                                let a1770 = 7u64;
                                if nat_eq(a1769, a1770) {
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

pub fn f456(v0: Str, v1: List<(Str, Adt6)>, v2: Str, v3: List<u64>, v4: List<Str>, v5: Adt120) -> R<Adt142> {
    let m1771 = v3.clone();
    match m1771.uncons() {
        None => {
            Ok(Adt142::C0(Adt91::C8(v0)))
        }
        Some((_, _)) => {
            if f188(v3.clone(), true) {
                let m1772 = f525(v1);
                match m1772 {
                    None => {
                        Ok(Adt142::C0(Adt91::C11(v0)))
                    }
                    Some(v8) => {
                        let v9: List<u64> = f526(v3)?;
                        if f527(v9.clone(), match v5.clone() {
                            Adt120::C0(_, h1773) => {
                                h1773
                            }
                        }) {
                            Ok(Adt142::C0(Adt91::C12(v0, v2)))
                        } else {
                            Ok(Adt142::C1(Adt120::C0(List::cons(Adt12::C0(v0, v2, f528(v4, List::<Str>::nil()), v8), match v5.clone() {
                                Adt120::C0(h1774, _) => {
                                    h1774
                                }
                            }), List::cons(v9, match v5 {
                                Adt120::C0(_, h1775) => {
                                    h1775
                                }
                            }))))
                        }
                    }
                }
            } else {
                Ok(Adt142::C0(Adt91::C9(v0)))
            }
        }
    }
}

pub fn f457(p1780: List<(Str, Adt6)>, v1: bool, v2: List<Str>, p1781: List<Adt104>) -> List<Adt104> {
    let m1779 = (p1780, p1781);
    {
        let mut m1779 = m1779;
        loop {
            let step = {
                let (v0, v3) = m1779;
                let m1776 = v0;
                match m1776.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v3)
                    }
                    Some((v4, v5)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v5, List::cons(Adt104::C0({
                            let (_, h1777) = v4;
                            h1777
                        }, v1, List::cons({
                            let (h1778, _) = v4;
                            h1778
                        }, v2.clone())), v3)))
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

pub fn f458(p1785: List<(Str, Adt6)>, p1786: List<Adt104>) -> List<Adt104> {
    let m1784 = (p1785, p1786);
    {
        let mut m1784 = m1784;
        loop {
            let step = {
                let (v0, v1) = m1784;
                let m1782 = v0;
                match m1782.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Adt6)>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0({
                            let (_, h1783) = v2;
                            h1783
                        }, false, List::<Str>::nil()), v1)))
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

pub fn f459(p1789: List<Adt6>, p1790: List<Adt104>) -> List<Adt104> {
    let m1788 = (p1789, p1790);
    {
        let mut m1788 = m1788;
        loop {
            let step = {
                let (v0, v1) = m1788;
                let m1787 = v0;
                match m1787.uncons() {
                    None => {
                        Err::<(List<Adt6>, List<Adt104>), List<Adt104>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt6>, List<Adt104>), List<Adt104>>((v3, List::cons(Adt104::C0(v2, false, List::<Str>::nil()), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: Adt6) -> Str {
    let m1791 = v0;
    match m1791 {
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

pub fn f461(v0: Adt7) -> R<bool> {
    if f374(match v0.clone() {
        Adt7::C0(h1792, _) => {
            h1792
        }
    }) {
        Ok(true)
    } else {
        f529(40u64, match v0.clone() {
            Adt7::C0(h1793, _) => {
                h1793
            }
        }, match v0 {
            Adt7::C0(_, h1794) => {
                h1794
            }
        })
    }
}

pub fn f462(v0: Adt6) -> Str {
    let m1795 = f231(v0);
    match m1795 {
        None => {
            Str::lit("")
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f463(p1802: u64, p1803: List<(Adt6, Adt6)>) -> R<Adt146> {
    let m1801 = (p1802, p1803);
    {
        let mut m1801 = m1801;
        loop {
            let step = {
                let (v0, v1) = m1801;
                let m1796 = v0;
                if m1796 == 0 {
                    Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C2))
                } else {
                    let v2 = m1796 - 1;
                    let m1797 = v1;
                    match m1797.uncons() {
                        None => {
                            Err::<(u64, List<(Adt6, Adt6)>), R<Adt146>>(Ok(Adt146::C0))
                        }
                        Some((v3, v4)) => {
                            let m1800 = f530({
                                let (h1798, _) = v3;
                                h1798
                            }, {
                                let (_, h1799) = v3;
                                h1799
                            }, v4)?;
                            match m1800 {
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
                Ok(next) => { m1801 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f464() -> u64 {
    67108864u64
}

pub fn f465(v0: Adt147) -> bool {
    let m1804 = v0;
    match m1804 {
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

pub fn f466(v0: Adt7, v1: Adt7, v2: Adt147, v3: Adt147) -> R<Adt147> {
    let m1805 = v2;
    match m1805 {
        Adt147::C0 => {
            let m1807 = v3;
            match m1807 {
                Adt147::C0 => {
                    Ok(f531(Adt147::C0, f532(f533(match v0.clone() {
                        Adt7::C0(h1808, _) => {
                            h1808
                        }
                    })?, match v0 {
                        Adt7::C0(_, h1809) => {
                            h1809
                        }
                    }, f533(match v1.clone() {
                        Adt7::C0(h1810, _) => {
                            h1810
                        }
                    })?, match v1 {
                        Adt7::C0(_, h1811) => {
                            h1811
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
            let m1806 = v3;
            match m1806 {
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
            let m1812 = v3;
            match m1812 {
                Adt147::C0 => {
                    Ok(Adt147::C2)
                }
                Adt147::C1 => {
                    Ok(Adt147::C2)
                }
                Adt147::C2 => {
                    f532(match v0.clone() {
                        Adt7::C0(h1813, _) => {
                            h1813
                        }
                    }, match v0 {
                        Adt7::C0(_, h1814) => {
                            h1814
                        }
                    }, match v1.clone() {
                        Adt7::C0(h1815, _) => {
                            h1815
                        }
                    }, match v1 {
                        Adt7::C0(_, h1816) => {
                            h1816
                        }
                    })
                }
            }
        }
    }
}

pub fn f467(v0: i64) -> Adt147 {
    f469(v0, 0i64)
}

pub fn f468(v0: Adt6) -> Option<i64> {
    let m1817 = v0;
    match m1817 {
        Adt6::C0 => {
            None::<i64>
        }
        Adt6::C1(_) => {
            None::<i64>
        }
        Adt6::C2(v2) => {
            if f374(match v2.clone() {
                Adt7::C0(_, h1818) => {
                    h1818
                }
            }) {
                Some(match v2 {
                    Adt7::C0(h1819, _) => {
                        h1819
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

pub fn f469(v0: i64, v1: i64) -> Adt147 {
    let m1822 = {
        let a1820 = v0;
        let a1821 = v1;
        compare(a1820, a1821)
    };
    if match m1822 {
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
        let m1825 = {
            let a1823 = v1;
            let a1824 = v0;
            compare(a1823, a1824)
        };
        if match m1825 {
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

pub fn f470(p1834: List<u64>, p1835: u64) -> R<u64> {
    let m1833 = (p1834, p1835);
    {
        let mut m1833 = m1833;
        loop {
            let step = {
                let (v0, v1) = m1833;
                let m1826 = v0;
                match m1826.uncons() {
                    None => {
                        Err::<(List<u64>, u64), R<u64>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a1827 = v2;
                        let a1828 = 128u64;
                        if if nat_lt(a1827, a1828) {
                            true
                        } else {
                            let a1829 = 191u64;
                            let a1830 = v2;
                            nat_lt(a1829, a1830)
                        } {
                            Ok::<(List<u64>, u64), R<u64>>((v3, {
                                let a1831 = v1;
                                let a1832 = 1u64;
                                nat_add(a1831, a1832)?
                            }))
                        } else {
                            Ok::<(List<u64>, u64), R<u64>>((v3, v1))
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

pub fn f471(v0: Str, p1853: List<(Str, Adt6)>) -> Option<Adt6> {
    let m1852 = p1853;
    {
        let mut m1852 = m1852;
        loop {
            let step = {
                let v1 = m1852;
                let m1836 = v1;
                match m1836.uncons() {
                    None => {
                        Err::<List<(Str, Adt6)>, Option<Adt6>>(None::<Adt6>)
                    }
                    Some((v2, v3)) => {
                        let a1840 = v0.clone();
                        let a1841 = {
                            let a1838 = List::cons(Str::lit("#/$defs/"), List::cons({
                                let (h1837, _) = v2.clone();
                                h1837
                            }, List::<Str>::nil()));
                            let a1839 = Str::lit("");
                            join(a1838, a1839)
                        };
                        if equal(a1840, a1841) {
                            Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                let (_, h1842) = v2;
                                h1842
                            }))
                        } else {
                            let m1844 = f195({
                                let (_, h1843) = v2.clone();
                                h1843
                            });
                            match m1844 {
                                None => {
                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                }
                                Some(v4) => {
                                    let m1845 = f176(Str::lit("$anchor"), v4);
                                    match m1845 {
                                        Adt130::C0 => {
                                            Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                        }
                                        Adt130::C1(v5) => {
                                            let m1846 = f231(v5);
                                            match m1846 {
                                                None => {
                                                    Ok::<List<(Str, Adt6)>, Option<Adt6>>(v3)
                                                }
                                                Some(v6) => {
                                                    let a1849 = v0.clone();
                                                    let a1850 = {
                                                        let a1847 = List::cons(Str::lit("#"), List::cons(v6, List::<Str>::nil()));
                                                        let a1848 = Str::lit("");
                                                        join(a1847, a1848)
                                                    };
                                                    if equal(a1849, a1850) {
                                                        Err::<List<(Str, Adt6)>, Option<Adt6>>(Some({
                                                            let (_, h1851) = v2;
                                                            h1851
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
                Ok(next) => { m1852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: u64) -> R<Option<u64>> {
    let a1854 = 64u64;
    let a1855 = v0;
    if if nat_lt(a1854, a1855) {
        let a1856 = v0;
        let a1857 = 91u64;
        nat_lt(a1856, a1857)
    } else {
        false
    } {
        Ok(Some({
            let a1858 = v0;
            let a1859 = 65u64;
            nat_sub(a1858, a1859)
        }))
    } else {
        let a1860 = 96u64;
        let a1861 = v0;
        if if nat_lt(a1860, a1861) {
            let a1862 = v0;
            let a1863 = 123u64;
            nat_lt(a1862, a1863)
        } else {
            false
        } {
            Ok(Some({
                let a1864 = v0;
                let a1865 = 71u64;
                nat_sub(a1864, a1865)
            }))
        } else {
            let a1866 = 47u64;
            let a1867 = v0;
            if if nat_lt(a1866, a1867) {
                let a1868 = v0;
                let a1869 = 58u64;
                nat_lt(a1868, a1869)
            } else {
                false
            } {
                Ok(Some({
                    let a1870 = v0;
                    let a1871 = 4u64;
                    nat_add(a1870, a1871)?
                }))
            } else {
                let a1872 = v0;
                let a1873 = 43u64;
                if nat_eq(a1872, a1873) {
                    Ok(Some(62u64))
                } else {
                    let a1874 = v0;
                    let a1875 = 47u64;
                    if nat_eq(a1874, a1875) {
                        Ok(Some(63u64))
                    } else {
                        Ok(None::<u64>)
                    }
                }
            }
        }
    }
}

pub fn f473(v0: u64, v1: u64, v2: bool, v3: u64) -> R<Option<List<u64>>> {
    if if v2 {
        let a1876 = v3;
        let a1877 = 61u64;
        if nat_eq(a1876, a1877) {
            let a1878 = f534(v1, 16u64);
            let a1879 = 0u64;
            nat_eq(a1878, a1879)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some(f535(v0, v1)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f474(v0: u64, v1: u64, v2: u64, v3: bool) -> R<Option<List<u64>>> {
    if if v3 {
        let a1880 = f534(v2, 4u64);
        let a1881 = 0u64;
        nat_eq(a1880, a1881)
    } else {
        false
    } {
        Ok(Some(f536(v0, v1, v2)?))
    } else {
        Ok(None::<List<u64>>)
    }
}

pub fn f475(v0: u64, v1: u64, v2: u64, v3: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a1884 = {
            let a1882 = v0;
            let a1883 = 4u64;
            nat_mul(a1882, a1883)?
        };
        let a1885 = f537(v1, 16u64);
        nat_add(a1884, a1885)?
    }, List::cons({
        let a1888 = {
            let a1886 = f534(v1, 16u64);
            let a1887 = 16u64;
            nat_mul(a1886, a1887)?
        };
        let a1889 = f537(v2, 4u64);
        nat_add(a1888, a1889)?
    }, List::cons({
        let a1892 = {
            let a1890 = f534(v2, 4u64);
            let a1891 = 64u64;
            nat_mul(a1890, a1891)?
        };
        let a1893 = v3;
        nat_add(a1892, a1893)?
    }, List::<u64>::nil()))))
}

pub fn f476(p1902: Bytes, p1903: List<Bytes>) -> List<Bytes> {
    let m1901 = (p1902, p1903);
    {
        let mut m1901 = m1901;
        loop {
            let step = {
                let (v0, v1) = m1901;
                let m1894 = v1;
                match m1894.uncons() {
                    None => {
                        Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::<Bytes>::nil()))
                    }
                    Some((v2, v3)) => {
                        let a1897 = {
                            let a1895 = v2.clone();
                            length_bytes(a1895)
                        };
                        let a1898 = {
                            let a1896 = v0.clone();
                            length_bytes(a1896)
                        };
                        if nat_eq(a1897, a1898) {
                            Ok::<(Bytes, List<Bytes>), List<Bytes>>(({
                                let a1899 = v2;
                                let a1900 = v0;
                                append_bytes(a1899, a1900)
                            }, v3))
                        } else {
                            Err::<(Bytes, List<Bytes>), List<Bytes>>(List::cons(v0, List::cons(v2, v3)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1901 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: u64) -> Bytes {
    let m1907 = {
        let a1904 = f538();
        let a1905 = v0;
        let a1906 = 1u64;
        slice_bytes(a1904, a1905, a1906)
    };
    match m1907 {
        None => {
            f332()
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f478(p1919: u64, p1920: i64, p1921: List<u64>) -> R<List<u64>> {
    let m1918 = (p1919, (p1920, p1921));
    {
        let mut m1918 = m1918;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1918;
                let m1908 = v0;
                if m1908 == 0 {
                    Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(v2))
                } else {
                    let v3 = m1908 - 1;
                    let v4: i64 = {
                        let a1909 = v1;
                        let a1910 = 10i64;
                        let a1911 = 0i64;
                        int_quot(a1909, a1910, a1911)?
                    };
                    let v5: i64 = {
                        let a1912 = v1;
                        let a1913 = 10i64;
                        let a1914 = 0i64;
                        int_rem(a1912, a1913, a1914)
                    };
                    let m1917 = {
                        let a1915 = v4;
                        let a1916 = 1i64;
                        compare(a1915, a1916)
                    };
                    if match m1917 {
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
                        Err::<(u64, (i64, List<u64>)), R<List<u64>>>(Ok(List::cons(f539(v5), v2)))
                    } else {
                        Ok::<(u64, (i64, List<u64>)), R<List<u64>>>((v3, (v4, List::cons(f539(v5), v2))))
                    }
                }
            };
            match step {
                Ok(next) => { m1918 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: u64) -> R<u64> {
    let a1922 = v0;
    let a1923 = 10u64;
    if nat_lt(a1922, a1923) {
        let a1924 = 48u64;
        let a1925 = v0;
        nat_add(a1924, a1925)
    } else {
        let a1926 = 87u64;
        let a1927 = v0;
        nat_add(a1926, a1927)
    }
}

pub fn f480(v0: Str, v1: Option<Adt8>) -> List<(Str, Adt6)> {
    let m1928 = v1;
    match m1928 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v2) => {
            List::cons((v0, f540(v2)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f481(v0: Option<Adt9>) -> List<(Str, Adt6)> {
    let m1929 = v0;
    match m1929 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("resources"), Adt6::C5({
                let a1932 = f422(Str::lit("subscribe"), match v1.clone() {
                    Adt9::C0(h1930, _) => {
                        h1930
                    }
                });
                let a1933 = f422(Str::lit("listChanged"), match v1 {
                    Adt9::C0(_, h1931) => {
                        h1931
                    }
                });
                append_list(a1932, a1933)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f482(v0: u64) -> R<Adt6> {
    Ok(Adt6::C2(Adt7::C0(f541(v0, 0i64)?, 0i64)))
}

pub fn f483(v0: Adt11) -> Str {
    let m1934 = v0;
    match m1934 {
        Adt11::C0 => {
            Str::lit("public")
        }
        Adt11::C1 => {
            Str::lit("private")
        }
    }
}

pub fn f484(p1937: List<Adt35>, p1938: List<Adt6>) -> List<Adt6> {
    let m1936 = (p1937, p1938);
    {
        let mut m1936 = m1936;
        loop {
            let step = {
                let (v0, v1) = m1936;
                let m1935 = v0;
                match m1935.uncons() {
                    None => {
                        Err::<(List<Adt35>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt35>, List<Adt6>), List<Adt6>>((v3, List::cons(f542(v2), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1936 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(p1942: List<Adt38>, p1943: List<Adt6>) -> R<Option<List<Adt6>>> {
    let m1941 = (p1942, p1943);
    {
        let mut m1941 = m1941;
        loop {
            let step = {
                let (v0, v1) = m1941;
                let m1939 = v0;
                match m1939.uncons() {
                    None => {
                        Err::<(List<Adt38>, List<Adt6>), R<Option<List<Adt6>>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m1940 = f543(v2)?;
                        match m1940 {
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
                Ok(next) => { m1941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486(v0: Option<List<Adt2>>) -> List<(Str, Adt6)> {
    let m1944 = v0;
    match m1944 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("icons"), Adt6::C4(f544(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f487() -> List<u64> {
    List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(101u64, List::cons(110u64, List::cons(116u64, List::cons(45u64, List::cons(84u64, List::cons(121u64, List::cons(112u64, List::cons(101u64, List::<u64>::nil()))))))))))))
}

pub fn f488() -> List<u64> {
    List::cons(67u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::cons(45u64, List::cons(67u64, List::cons(111u64, List::cons(110u64, List::cons(116u64, List::cons(114u64, List::cons(111u64, List::cons(108u64, List::<u64>::nil())))))))))))))
}

pub fn f489() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::cons(45u64, List::cons(99u64, List::cons(97u64, List::cons(99u64, List::cons(104u64, List::cons(101u64, List::<u64>::nil()))))))))
}

pub fn f490() -> List<u64> {
    List::cons(88u64, List::cons(45u64, List::cons(65u64, List::cons(99u64, List::cons(99u64, List::cons(101u64, List::cons(108u64, List::cons(45u64, List::cons(66u64, List::cons(117u64, List::cons(102u64, List::cons(102u64, List::cons(101u64, List::cons(114u64, List::cons(105u64, List::cons(110u64, List::cons(103u64, List::<u64>::nil())))))))))))))))))
}

pub fn f491() -> List<u64> {
    List::cons(110u64, List::cons(111u64, List::<u64>::nil()))
}

pub fn f492(p1949: List<List<u64>>, p1950: List<u64>) -> List<u64> {
    let m1948 = (p1949, p1950);
    {
        let mut m1948 = m1948;
        loop {
            let step = {
                let (v0, v1) = m1948;
                let m1945 = v0;
                match m1945.uncons() {
                    None => {
                        Err::<(List<List<u64>>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<List<u64>>, List<u64>), List<u64>>((v3, {
                            let a1946 = f545(v2);
                            let a1947 = v1;
                            append_list(a1946, a1947)
                        }))
                    }
                }
            };
            match step {
                Ok(next) => { m1948 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f493(v0: Str) -> R<Str> {
    let m1953 = {
        let a1952 = f331(f332(), f163(f546(f87({
            let a1951 = v0;
            utf8_encode(a1951)
        })?, List::<u64>::nil()), List::<u64>::nil()));
        utf8_decode(a1952)
    };
    match m1953 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f494(v0: List<u64>) -> bool {
    let m1954 = v0;
    match m1954.uncons() {
        None => {
            false
        }
        Some((v1, v2)) => {
            let a1955 = v1;
            let a1956 = 48u64;
            if nat_eq(a1955, a1956) {
                f547(v2)
            } else {
                false
            }
        }
    }
}

pub fn f495(v0: Adt0, v1: Fn0, v2: u64, v3: Adt14) -> R<Result<Adt123, Adt126>> {
    if f548(f549(v3.clone())) {
        Ok(Ok::<Adt123, Adt126>(f550(v0.clone(), Adt32::C0(f551(v0)))))
    } else {
        Ok(f552(v0.clone(), v3.clone(), f553(match v0 {
            Adt0::C0(_, _, _, _, _, h1957) => {
                h1957
            }
        }, v2, v1, v3)?))
    }
}

pub fn f496(v0: Adt16, _: Adt5) -> Result<Adt14, Adt127> {
    Ok::<Adt14, Adt127>(Adt14::C0(Adt15::C0(v0)))
}

pub fn f497(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1958 = f554(Str::lit("cursor"), v1);
    match m1958 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C1(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f498(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1959 = f555(Str::lit("name"), v1.clone());
    match m1959 {
        Ok(v2) => {
            let m1960 = f556(Str::lit("arguments"), v1.clone());
            match m1960 {
                Ok(v3) => {
                    let m1961 = f556(Str::lit("inputResponses"), v1.clone());
                    match m1961 {
                        Ok(v4) => {
                            let m1962 = f554(Str::lit("requestState"), v1);
                            match m1962 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt127>(Adt14::C2(Adt21::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt127>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt127>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt127>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt127>(v9)
        }
    }
}

pub fn f499(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1963 = f554(Str::lit("cursor"), v1);
    match m1963 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C3(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f500(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1964 = f555(Str::lit("uri"), v1.clone());
    match m1964 {
        Ok(v2) => {
            let m1965 = f556(Str::lit("inputResponses"), v1.clone());
            match m1965 {
                Ok(v3) => {
                    let m1966 = f554(Str::lit("requestState"), v1);
                    match m1966 {
                        Ok(v4) => {
                            Ok::<Adt14, Adt127>(Adt14::C4(Adt22::C0(v0, v2, v3, v4)))
                        }
                        Err(v5) => {
                            Err::<Adt14, Adt127>(v5)
                        }
                    }
                }
                Err(v6) => {
                    Err::<Adt14, Adt127>(v6)
                }
            }
        }
        Err(v7) => {
            Err::<Adt14, Adt127>(v7)
        }
    }
}

pub fn f501(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1967 = f554(Str::lit("cursor"), v1);
    match m1967 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C5(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f502(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1968 = f554(Str::lit("cursor"), v1);
    match m1968 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C6(Adt20::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f503(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1969 = f555(Str::lit("name"), v1.clone());
    match m1969 {
        Ok(v2) => {
            let m1970 = f557(Str::lit("arguments"), v1.clone());
            match m1970 {
                Ok(v3) => {
                    let m1971 = f556(Str::lit("inputResponses"), v1.clone());
                    match m1971 {
                        Ok(v4) => {
                            let m1972 = f554(Str::lit("requestState"), v1);
                            match m1972 {
                                Ok(v5) => {
                                    Ok::<Adt14, Adt127>(Adt14::C7(Adt23::C0(v0, v2, v3, v4, v5)))
                                }
                                Err(v6) => {
                                    Err::<Adt14, Adt127>(v6)
                                }
                            }
                        }
                        Err(v7) => {
                            Err::<Adt14, Adt127>(v7)
                        }
                    }
                }
                Err(v8) => {
                    Err::<Adt14, Adt127>(v8)
                }
            }
        }
        Err(v9) => {
            Err::<Adt14, Adt127>(v9)
        }
    }
}

pub fn f504(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1973 = f558(Str::lit("ref"), v1.clone());
    match m1973 {
        Ok(v2) => {
            let m1974 = f559(v2);
            match m1974 {
                Ok(v3) => {
                    let m1975 = f558(Str::lit("argument"), v1.clone());
                    match m1975 {
                        Ok(v4) => {
                            let m1976 = f560(v4);
                            match m1976 {
                                Ok(v5) => {
                                    let m1977 = f561(v1);
                                    match m1977 {
                                        Ok(v6) => {
                                            Ok::<Adt14, Adt127>(Adt14::C8(Adt24::C0(v0, v3, v5, v6)))
                                        }
                                        Err(v7) => {
                                            Err::<Adt14, Adt127>(v7)
                                        }
                                    }
                                }
                                Err(v8) => {
                                    Err::<Adt14, Adt127>(v8)
                                }
                            }
                        }
                        Err(v9) => {
                            Err::<Adt14, Adt127>(v9)
                        }
                    }
                }
                Err(v10) => {
                    Err::<Adt14, Adt127>(v10)
                }
            }
        }
        Err(v11) => {
            Err::<Adt14, Adt127>(v11)
        }
    }
}

pub fn f505(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1978 = f555(Str::lit("taskId"), v1);
    match m1978 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C9(Adt29::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f506(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1979 = f555(Str::lit("taskId"), v1.clone());
    match m1979 {
        Ok(v2) => {
            let m1980 = f558(Str::lit("inputResponses"), v1);
            match m1980 {
                Ok(v3) => {
                    Ok::<Adt14, Adt127>(Adt14::C10(Adt30::C0(v0, v2, v3)))
                }
                Err(v4) => {
                    Err::<Adt14, Adt127>(v4)
                }
            }
        }
        Err(v5) => {
            Err::<Adt14, Adt127>(v5)
        }
    }
}

pub fn f507(v0: Adt16, v1: Adt5) -> Result<Adt14, Adt127> {
    let m1981 = f555(Str::lit("taskId"), v1);
    match m1981 {
        Ok(v2) => {
            Ok::<Adt14, Adt127>(Adt14::C11(Adt31::C0(v0, v2)))
        }
        Err(v3) => {
            Err::<Adt14, Adt127>(v3)
        }
    }
}

pub fn f508(v0: Adt103, v1: Adt16, v2: Option<Adt126>) -> Adt138 {
    let m1982 = v2;
    match m1982 {
        None => {
            Adt138::C0(Adt139::C0(match v0.clone() {
                Adt103::C0(h1983, _, _) => {
                    h1983
                }
            }, match v0.clone() {
                Adt103::C0(_, h1984, _) => {
                    h1984
                }
            }, v1, f449(match v0 {
                Adt103::C0(_, _, h1985) => {
                    h1985
                }
            })))
        }
        Some(v3) => {
            Adt138::C1(v3)
        }
    }
}

pub fn f509(v0: Str, v1: List<Str>) -> Option<Adt126> {
    f562(f563(v0.clone(), v1.clone()), v0, v1)
}

pub fn f510(v0: Adt107, v1: Adt140, v2: Str, v3: Str, v4: Adt5) -> R<Option<Adt108>> {
    let m1987 = match v0.clone() {
        Adt107::C0(h1986, _, _, _, _) => {
            h1986
        }
    };
    match m1987 {
        None => {
            Ok(Some(Adt108::C0))
        }
        Some(v5) => {
            let a1988 = v5;
            let a1989 = v3;
            if equal(a1988, a1989) {
                let m1991 = match v0.clone() {
                    Adt107::C0(_, h1990, _, _, _) => {
                        h1990
                    }
                };
                match m1991 {
                    None => {
                        Ok(Some(Adt108::C1))
                    }
                    Some(v6) => {
                        let a1992 = v6;
                        let a1993 = v2;
                        if equal(a1992, a1993) {
                            f564(f565(match v0.clone() {
                                Adt107::C0(_, _, h1994, _, _) => {
                                    h1994
                                }
                            }, v1, v4.clone()), match v0 {
                                Adt107::C0(_, _, _, _, h1995) => {
                                    h1995
                                }
                            }, v4)
                        } else {
                            Ok(Some(Adt108::C4))
                        }
                    }
                }
            } else {
                Ok(Some(Adt108::C3))
            }
        }
    }
}

pub fn f511() -> Adt5 {
    Adt5::C0(List::<(Str, Adt6)>::nil())
}

pub fn f512(v0: Adt5) -> Adt148 {
    f566(f176(f514(), v0.clone()), f176(f515(), v0))
}

pub fn f513(v0: Adt5, v1: Adt130, v2: Adt130) -> Result<Adt16, Adt127> {
    let m1996 = v1;
    match m1996 {
        Adt130::C0 => {
            Err::<Adt16, Adt127>(Adt127::C2)
        }
        Adt130::C1(v3) => {
            let m1997 = v2;
            match m1997 {
                Adt130::C0 => {
                    Err::<Adt16, Adt127>(Adt127::C3)
                }
                Adt130::C1(v4) => {
                    f567(v0, v3, v4)
                }
            }
        }
    }
}

pub fn f514() -> Str {
    Str::lit("io.modelcontextprotocol/protocolVersion")
}

pub fn f515() -> Str {
    Str::lit("io.modelcontextprotocol/clientCapabilities")
}

pub fn f516(v0: Adt6) -> bool {
    let m1998 = f231(v0);
    match m1998 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f517(v0: Adt6) -> bool {
    let m1999 = f392(v0);
    match m1999 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f518(v0: Adt6) -> bool {
    let m2000 = f468(v0);
    match m2000 {
        None => {
            false
        }
        Some(v1) => {
            let a2004 = {
                let m2003 = {
                    let a2001 = v1;
                    let a2002 = 0i64;
                    compare(a2001, a2002)
                };
                match m2003 {
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
            bool_not(a2004)
        }
    }
}

pub fn f519(v0: Adt6) -> bool {
    let m2005 = f259(v0);
    match m2005 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f520(v0: Adt6) -> bool {
    let m2006 = f195(v0);
    match m2006 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f521(v0: Adt6) -> bool {
    let m2007 = f259(v0);
    match m2007 {
        None => {
            false
        }
        Some(v1) => {
            f568(v1)
        }
    }
}

pub fn f522(v0: Adt6) -> bool {
    let m2008 = f194(v0);
    match m2008 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f523(v0: Adt6) -> bool {
    let m2009 = f231(v0.clone());
    match m2009 {
        None => {
            let m2010 = f259(v0);
            match m2010 {
                None => {
                    false
                }
                Some(v2) => {
                    f569(v2)
                }
            }
        }
        Some(v1) => {
            f387(v1)
        }
    }
}

pub fn f524(v0: Adt6) -> bool {
    let m2011 = f259(v0);
    match m2011 {
        None => {
            false
        }
        Some(v1) => {
            let m2012 = v1;
            match m2012.uncons() {
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
    let m2013 = f234(Str::lit("type"), v0);
    match m2013 {
        Adt130::C0 => {
            None::<Adt13>
        }
        Adt130::C1(v1) => {
            let m2014 = f231(v1);
            match m2014 {
                None => {
                    None::<Adt13>
                }
                Some(v2) => {
                    f570(v2)
                }
            }
        }
    }
}

pub fn f526(v0: List<u64>) -> R<List<u64>> {
    Ok(f163(f571(v0, List::<u64>::nil())?, List::<u64>::nil()))
}

pub fn f527(v0: List<u64>, p2017: List<List<u64>>) -> bool {
    let m2016 = p2017;
    {
        let mut m2016 = m2016;
        loop {
            let step = {
                let v1 = m2016;
                let m2015 = v1;
                match m2015.uncons() {
                    None => {
                        Err::<List<List<u64>>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f572(v0.clone(), v2) {
                            Err::<List<List<u64>>, bool>(true)
                        } else {
                            Ok::<List<List<u64>>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f528(p2020: List<Str>, p2021: List<Str>) -> List<Str> {
    let m2019 = (p2020, p2021);
    {
        let mut m2019 = m2019;
        loop {
            let step = {
                let (v0, v1) = m2019;
                let m2018 = v0;
                match m2018.uncons() {
                    None => {
                        Err::<(List<Str>, List<Str>), List<Str>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Str>, List<Str>), List<Str>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2019 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f529(p2033: u64, p2034: i64, p2035: i64) -> R<bool> {
    let m2032 = (p2033, (p2034, p2035));
    {
        let mut m2032 = m2032;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2032;
                let m2022 = v0;
                if m2022 == 0 {
                    Err::<(u64, (i64, i64)), R<bool>>(Ok(false))
                } else {
                    let v3 = m2022 - 1;
                    let m2025 = {
                        let a2023 = v2;
                        let a2024 = 0i64;
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
                        if f374({
                            let a2026 = v1;
                            let a2027 = 10i64;
                            let a2028 = 0i64;
                            int_rem(a2026, a2027, a2028)
                        }) {
                            Ok::<(u64, (i64, i64)), R<bool>>((v3, ({
                                let a2029 = v1;
                                let a2030 = 10i64;
                                let a2031 = 0i64;
                                int_quot(a2029, a2030, a2031)?
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
                Ok(next) => { m2032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(v0: Adt6, v1: Adt6, v2: List<(Adt6, Adt6)>) -> R<Adt149> {
    let a2036 = f573(v0.clone());
    let a2037 = f573(v1.clone());
    if nat_eq(a2036, a2037) {
        let m2038 = v0;
        match m2038 {
            Adt6::C0 => {
                Ok(Adt149::C1(v2))
            }
            Adt6::C1(v3) => {
                let m2039 = f194(v1);
                match m2039 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v4) => {
                        let a2040 = v3;
                        let a2041 = v4;
                        if equal(a2040, a2041) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C2(v5) => {
                let m2042 = f392(v1);
                match m2042 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v6) => {
                        if f574(f394(v5, v6)?) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C3(v7) => {
                let m2043 = f231(v1);
                match m2043 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v8) => {
                        let a2044 = v7;
                        let a2045 = v8;
                        if equal(a2044, a2045) {
                            Ok(Adt149::C1(v2))
                        } else {
                            Ok(Adt149::C0)
                        }
                    }
                }
            }
            Adt6::C4(v9) => {
                let m2046 = f259(v1);
                match m2046 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v10) => {
                        Ok(f575(f576(v9, v10, List::<(Adt6, Adt6)>::nil()), v2))
                    }
                }
            }
            Adt6::C5(v11) => {
                let m2047 = f195(v1);
                match m2047 {
                    None => {
                        Ok(Adt149::C0)
                    }
                    Some(v12) => {
                        let a2049 = f398(v11.clone(), 0u64)?;
                        let a2050 = f398(match v12.clone() {
                            Adt5::C0(h2048) => {
                                h2048
                            }
                        }, 0u64)?;
                        if nat_eq(a2049, a2050) {
                            Ok(f575(f577(v11, match v12 {
                                Adt5::C0(h2051) => {
                                    h2051
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
    let m2052 = v0;
    match m2052 {
        Adt147::C0 => {
            f578(v1)
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
    let m2053 = f469(f579(v0, v1)?, f579(v2, v3)?);
    match m2053 {
        Adt147::C0 => {
            Ok(Adt147::C0)
        }
        Adt147::C1 => {
            f580(v0, f581(v0)?, v2, f581(v2)?)
        }
        Adt147::C2 => {
            Ok(Adt147::C2)
        }
    }
}

pub fn f533(v0: i64) -> R<i64> {
    let m2056 = {
        let a2054 = v0;
        let a2055 = 0i64;
        compare(a2054, a2055)
    };
    if match m2056 {
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
        let a2057 = v0;
        int_neg(a2057)
    } else {
        Ok(v0)
    }
}

pub fn f534(v0: u64, v1: u64) -> u64 {
    let a2058 = v0;
    let a2059 = v1;
    let a2060 = 0u64;
    nat_rem(a2058, a2059, a2060)
}

pub fn f535(v0: u64, v1: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2063 = {
            let a2061 = v0;
            let a2062 = 4u64;
            nat_mul(a2061, a2062)?
        };
        let a2064 = f537(v1, 16u64);
        nat_add(a2063, a2064)?
    }, List::<u64>::nil()))
}

pub fn f536(v0: u64, v1: u64, v2: u64) -> R<List<u64>> {
    Ok(List::cons({
        let a2067 = {
            let a2065 = v0;
            let a2066 = 4u64;
            nat_mul(a2065, a2066)?
        };
        let a2068 = f537(v1, 16u64);
        nat_add(a2067, a2068)?
    }, List::cons({
        let a2071 = {
            let a2069 = f534(v1, 16u64);
            let a2070 = 16u64;
            nat_mul(a2069, a2070)?
        };
        let a2072 = f537(v2, 4u64);
        nat_add(a2071, a2072)?
    }, List::<u64>::nil())))
}

pub fn f537(v0: u64, v1: u64) -> u64 {
    let a2073 = v0;
    let a2074 = v1;
    let a2075 = 0u64;
    nat_quot(a2073, a2074, a2075)
}

pub fn f538() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f539(v0: i64) -> u64 {
    let m2078 = {
        let a2076 = v0;
        let a2077 = 1i64;
        compare(a2076, a2077)
    };
    if match m2078 {
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
        let m2081 = {
            let a2079 = v0;
            let a2080 = 2i64;
            compare(a2079, a2080)
        };
        if match m2081 {
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
            let m2084 = {
                let a2082 = v0;
                let a2083 = 3i64;
                compare(a2082, a2083)
            };
            if match m2084 {
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
                let m2087 = {
                    let a2085 = v0;
                    let a2086 = 4i64;
                    compare(a2085, a2086)
                };
                if match m2087 {
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
                    let m2090 = {
                        let a2088 = v0;
                        let a2089 = 5i64;
                        compare(a2088, a2089)
                    };
                    if match m2090 {
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
                        let m2093 = {
                            let a2091 = v0;
                            let a2092 = 6i64;
                            compare(a2091, a2092)
                        };
                        if match m2093 {
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
                            let m2096 = {
                                let a2094 = v0;
                                let a2095 = 7i64;
                                compare(a2094, a2095)
                            };
                            if match m2096 {
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
                                let m2099 = {
                                    let a2097 = v0;
                                    let a2098 = 8i64;
                                    compare(a2097, a2098)
                                };
                                if match m2099 {
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
                                    let m2102 = {
                                        let a2100 = v0;
                                        let a2101 = 9i64;
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

pub fn f540(v0: Adt8) -> Adt6 {
    Adt6::C5(f422(Str::lit("listChanged"), match v0 {
        Adt8::C0(h2103) => {
            h2103
        }
    }))
}

pub fn f541(p2108: u64, p2109: i64) -> R<i64> {
    let m2107 = (p2108, p2109);
    {
        let mut m2107 = m2107;
        loop {
            let step = {
                let (v0, v1) = m2107;
                let m2104 = v0;
                if m2104 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2104 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2105 = v1;
                        let a2106 = -1i64;
                        int_sub(a2105, a2106)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2107 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f542(v0: Adt35) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
        Adt35::C0(h2110, _, _, _, _, _, _, _) => {
            h2110
        }
    })), {
        let a2128 = f417(Str::lit("title"), match v0.clone() {
            Adt35::C0(_, h2111, _, _, _, _, _, _) => {
                h2111
            }
        });
        let a2129 = {
            let a2126 = f486(match v0.clone() {
                Adt35::C0(_, _, h2112, _, _, _, _, _) => {
                    h2112
                }
            });
            let a2127 = {
                let a2124 = f417(Str::lit("description"), match v0.clone() {
                    Adt35::C0(_, _, _, h2113, _, _, _, _) => {
                        h2113
                    }
                });
                let a2125 = List::cons((Str::lit("inputSchema"), Adt6::C5({
                    let m2116 = match v0.clone() {
                        Adt35::C0(_, _, _, _, h2115, _, _, _) => {
                            h2115
                        }
                    };
                    match m2116 {
                        Adt5::C0(h2114) => {
                            h2114
                        }
                    }
                })), {
                    let a2122 = f437(Str::lit("outputSchema"), match v0.clone() {
                        Adt35::C0(_, _, _, _, _, h2117, _, _) => {
                            h2117
                        }
                    });
                    let a2123 = {
                        let a2120 = f582(match v0.clone() {
                            Adt35::C0(_, _, _, _, _, _, h2118, _) => {
                                h2118
                            }
                        });
                        let a2121 = f583(match v0 {
                            Adt35::C0(_, _, _, _, _, _, _, h2119) => {
                                h2119
                            }
                        });
                        append_list(a2120, a2121)
                    };
                    append_list(a2122, a2123)
                });
                append_list(a2124, a2125)
            };
            append_list(a2126, a2127)
        };
        append_list(a2128, a2129)
    }))
}

pub fn f543(v0: Adt38) -> R<Option<Adt6>> {
    let m2130 = v0;
    match m2130 {
        Adt38::C0(v1) => {
            Ok(Some(f584(v1)))
        }
        Adt38::C1(v2) => {
            Ok(Some(f585(Str::lit("image"), match v2.clone() {
                Adt42::C0(h2131, _, _, _) => {
                    h2131
                }
            }, match v2.clone() {
                Adt42::C0(_, h2132, _, _) => {
                    h2132
                }
            }, match v2.clone() {
                Adt42::C0(_, _, h2133, _) => {
                    h2133
                }
            }, match v2 {
                Adt42::C0(_, _, _, h2134) => {
                    h2134
                }
            })))
        }
        Adt38::C2(v3) => {
            Ok(Some(f585(Str::lit("audio"), match v3.clone() {
                Adt43::C0(h2135, _, _, _) => {
                    h2135
                }
            }, match v3.clone() {
                Adt43::C0(_, h2136, _, _) => {
                    h2136
                }
            }, match v3.clone() {
                Adt43::C0(_, _, h2137, _) => {
                    h2137
                }
            }, match v3 {
                Adt43::C0(_, _, _, h2138) => {
                    h2138
                }
            })))
        }
        Adt38::C3(v4) => {
            Ok(Some(f586(v4)?))
        }
        Adt38::C4(v5) => {
            Ok(Some(f587(v5)))
        }
    }
}

pub fn f544(v0: List<Adt2>) -> List<Adt6> {
    f294(f588(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f545(v0: List<u64>) -> List<u64> {
    let a2141 = f589();
    let a2142 = {
        let a2139 = v0;
        let a2140 = f590();
        append_list(a2139, a2140)
    };
    append_list(a2141, a2142)
}

pub fn f546(v0: List<u64>, v1: List<u64>) -> List<u64> {
    f591(v0, 0u64, v1)
}

pub fn f547(p2149: List<u64>) -> bool {
    let m2148 = p2149;
    {
        let mut m2148 = m2148;
        loop {
            let step = {
                let v0 = m2148;
                let m2143 = v0;
                match m2143.uncons() {
                    None => {
                        Err::<List<u64>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let a2144 = v1;
                        let a2145 = 48u64;
                        if if nat_eq(a2144, a2145) {
                            true
                        } else {
                            let a2146 = v1;
                            let a2147 = 46u64;
                            nat_eq(a2146, a2147)
                        } {
                            Ok::<List<u64>, bool>(v2)
                        } else {
                            Err::<List<u64>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f548(v0: Adt134) -> bool {
    let m2150 = v0;
    match m2150 {
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

pub fn f549(v0: Adt14) -> Adt134 {
    let m2151 = v0;
    match m2151 {
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

pub fn f550(v0: Adt0, v1: Adt32) -> Adt123 {
    Adt123::C0(v1, Adt124::C0(Some(match v0 {
        Adt0::C0(h2152, _, _, _, _, _) => {
            h2152
        }
    }), None::<Adt5>), List::<Adt125>::nil())
}

pub fn f551(v0: Adt0) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt0::C0(_, _, _, h2153, _, _) => {
            h2153
        }
    }, f174(), match v0.clone() {
        Adt0::C0(_, _, h2154, _, _, _) => {
            h2154
        }
    }, match v0 {
        Adt0::C0(_, h2155, _, _, _, _) => {
            h2155
        }
    })
}

pub fn f552(v0: Adt0, v1: Adt14, v2: Result<Adt32, Adt84>) -> Result<Adt123, Adt126> {
    let m2156 = v2;
    match m2156 {
        Ok(v4) => {
            if f592(f549(v1.clone()), v4.clone()) {
                f593(v0, v1, v4)
            } else {
                Err::<Adt123, Adt126>(Adt126::C4(f594()))
            }
        }
        Err(v3) => {
            Err::<Adt123, Adt126>(Adt126::C4(v3))
        }
    }
}

pub fn f553(v0: u64, v1: u64, v2: Fn0, v3: Adt14) -> R<Result<Adt32, Adt84>> {
    let m2157 = v3.clone();
    match m2157 {
        Adt14::C0(_) => {
            let c2159: Fn0 = v2;
            c2159.apply(v3)
        }
        Adt14::C1(_) => {
            let c2160: Fn0 = v2;
            c2160.apply(v3)
        }
        Adt14::C2(_) => {
            if f595(v0, v1) {
                let c2158: Fn0 = v2;
                c2158.apply(v3)
            } else {
                Ok(Err::<Adt32, Adt84>(Adt84::C7(v0)))
            }
        }
        Adt14::C3(_) => {
            let c2161: Fn0 = v2;
            c2161.apply(v3)
        }
        Adt14::C4(_) => {
            let c2162: Fn0 = v2;
            c2162.apply(v3)
        }
        Adt14::C5(_) => {
            let c2163: Fn0 = v2;
            c2163.apply(v3)
        }
        Adt14::C6(_) => {
            let c2164: Fn0 = v2;
            c2164.apply(v3)
        }
        Adt14::C7(_) => {
            let c2165: Fn0 = v2;
            c2165.apply(v3)
        }
        Adt14::C8(_) => {
            let c2166: Fn0 = v2;
            c2166.apply(v3)
        }
        Adt14::C9(_) => {
            let c2167: Fn0 = v2;
            c2167.apply(v3)
        }
        Adt14::C10(_) => {
            let c2168: Fn0 = v2;
            c2168.apply(v3)
        }
        Adt14::C11(_) => {
            let c2169: Fn0 = v2;
            c2169.apply(v3)
        }
    }
}

pub fn f554(v0: Str, v1: Adt5) -> Result<Option<Str>, Adt127> {
    let m2170 = f176(v0.clone(), v1);
    match m2170 {
        Adt130::C0 => {
            Ok::<Option<Str>, Adt127>(None::<Str>)
        }
        Adt130::C1(v2) => {
            let m2171 = f231(v2);
            match m2171 {
                None => {
                    Err::<Option<Str>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Str>, Adt127>(Some(v3))
                }
            }
        }
    }
}

pub fn f555(v0: Str, v1: Adt5) -> Result<Str, Adt127> {
    let m2172 = f176(v0.clone(), v1);
    match m2172 {
        Adt130::C0 => {
            Err::<Str, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2173 = f231(v2);
            match m2173 {
                None => {
                    Err::<Str, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    Ok::<Str, Adt127>(v3)
                }
            }
        }
    }
}

pub fn f556(v0: Str, v1: Adt5) -> Result<Option<Adt5>, Adt127> {
    let m2174 = f176(v0.clone(), v1);
    match m2174 {
        Adt130::C0 => {
            Ok::<Option<Adt5>, Adt127>(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2175 = f195(v2);
            match m2175 {
                None => {
                    Err::<Option<Adt5>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    Ok::<Option<Adt5>, Adt127>(Some(v3))
                }
            }
        }
    }
}

pub fn f557(v0: Str, v1: Adt5) -> Result<Option<List<(Str, Str)>>, Adt127> {
    let m2176 = f176(v0.clone(), v1);
    match m2176 {
        Adt130::C0 => {
            Ok::<Option<List<(Str, Str)>>, Adt127>(None::<List<(Str, Str)>>)
        }
        Adt130::C1(v2) => {
            let m2177 = f195(v2);
            match m2177 {
                None => {
                    Err::<Option<List<(Str, Str)>>, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    let m2179 = f596(v0, match v3 {
                        Adt5::C0(h2178) => {
                            h2178
                        }
                    });
                    match m2179 {
                        Ok(v4) => {
                            Ok::<Option<List<(Str, Str)>>, Adt127>(Some(v4))
                        }
                        Err(v5) => {
                            Err::<Option<List<(Str, Str)>>, Adt127>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f558(v0: Str, v1: Adt5) -> Result<Adt5, Adt127> {
    let m2180 = f176(v0.clone(), v1);
    match m2180 {
        Adt130::C0 => {
            Err::<Adt5, Adt127>(Adt127::C7(v0))
        }
        Adt130::C1(v2) => {
            let m2181 = f195(v2);
            match m2181 {
                None => {
                    Err::<Adt5, Adt127>(Adt127::C8(v0))
                }
                Some(v3) => {
                    Ok::<Adt5, Adt127>(v3)
                }
            }
        }
    }
}

pub fn f559(v0: Adt5) -> Result<Adt25, Adt127> {
    let m2182 = f555(Str::lit("type"), v0.clone());
    match m2182 {
        Ok(v1) => {
            let a2183 = v1.clone();
            let a2184 = Str::lit("ref/prompt");
            if equal(a2183, a2184) {
                let m2185 = f555(Str::lit("name"), v0.clone());
                match m2185 {
                    Ok(v2) => {
                        let m2186 = f554(Str::lit("title"), v0);
                        match m2186 {
                            Ok(v3) => {
                                Ok::<Adt25, Adt127>(Adt25::C0(Adt26::C0(v2, v3)))
                            }
                            Err(v4) => {
                                Err::<Adt25, Adt127>(v4)
                            }
                        }
                    }
                    Err(v5) => {
                        Err::<Adt25, Adt127>(v5)
                    }
                }
            } else {
                let a2187 = v1;
                let a2188 = Str::lit("ref/resource");
                if equal(a2187, a2188) {
                    let m2189 = f555(Str::lit("uri"), v0);
                    match m2189 {
                        Ok(v6) => {
                            Ok::<Adt25, Adt127>(Adt25::C1(v6))
                        }
                        Err(v7) => {
                            Err::<Adt25, Adt127>(v7)
                        }
                    }
                } else {
                    Err::<Adt25, Adt127>(Adt127::C8(Str::lit("type")))
                }
            }
        }
        Err(v8) => {
            Err::<Adt25, Adt127>(v8)
        }
    }
}

pub fn f560(v0: Adt5) -> Result<Adt27, Adt127> {
    let m2190 = f555(Str::lit("name"), v0.clone());
    match m2190 {
        Ok(v1) => {
            let m2191 = f555(Str::lit("value"), v0);
            match m2191 {
                Ok(v2) => {
                    Ok::<Adt27, Adt127>(Adt27::C0(v1, v2))
                }
                Err(v3) => {
                    Err::<Adt27, Adt127>(v3)
                }
            }
        }
        Err(v4) => {
            Err::<Adt27, Adt127>(v4)
        }
    }
}

pub fn f561(v0: Adt5) -> Result<Option<Adt28>, Adt127> {
    let m2192 = f176(Str::lit("context"), v0);
    match m2192 {
        Adt130::C0 => {
            Ok::<Option<Adt28>, Adt127>(None::<Adt28>)
        }
        Adt130::C1(v1) => {
            let m2193 = f195(v1);
            match m2193 {
                None => {
                    Err::<Option<Adt28>, Adt127>(Adt127::C8(Str::lit("context")))
                }
                Some(v2) => {
                    let m2194 = f557(Str::lit("arguments"), v2);
                    match m2194 {
                        Ok(v3) => {
                            Ok::<Option<Adt28>, Adt127>(Some(Adt28::C0(v3)))
                        }
                        Err(v4) => {
                            Err::<Option<Adt28>, Adt127>(v4)
                        }
                    }
                }
            }
        }
    }
}

pub fn f562(v0: bool, v1: Str, v2: List<Str>) -> Option<Adt126> {
    if v0 {
        None::<Adt126>
    } else {
        Some(Adt126::C1(v1, v2))
    }
}

pub fn f563(v0: Str, p2199: List<Str>) -> bool {
    let m2198 = p2199;
    {
        let mut m2198 = m2198;
        loop {
            let step = {
                let v1 = m2198;
                let m2195 = v1;
                match m2195.uncons() {
                    None => {
                        Err::<List<Str>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a2196 = v2;
                        let a2197 = v0.clone();
                        if equal(a2196, a2197) {
                            Err::<List<Str>, bool>(true)
                        } else {
                            Ok::<List<Str>, bool>(v3)
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

pub fn f564(v0: Option<Adt108>, v1: List<Adt110>, v2: Adt5) -> R<Option<Adt108>> {
    let m2200 = v0;
    match m2200 {
        None => {
            f597(v1, f598(v2))
        }
        Some(v3) => {
            Ok(Some(v3))
        }
    }
}

pub fn f565(v0: Option<Str>, v1: Adt140, v2: Adt5) -> Option<Adt108> {
    let m2201 = v1.clone();
    match m2201 {
        Adt140::C0 => {
            None::<Adt108>
        }
        Adt140::C1 => {
            f599(v0, f600(v1, v2))
        }
        Adt140::C2 => {
            f599(v0, f600(v1, v2))
        }
        Adt140::C3 => {
            f599(v0, f600(v1, v2))
        }
    }
}

pub fn f566(v0: Adt130, v1: Adt130) -> Adt148 {
    let m2202 = v0;
    match m2202 {
        Adt130::C0 => {
            Adt148::C1
        }
        Adt130::C1(_) => {
            let m2203 = v1;
            match m2203 {
                Adt130::C0 => {
                    Adt148::C2
                }
                Adt130::C1(_) => {
                    Adt148::C0
                }
            }
        }
    }
}

pub fn f567(v0: Adt5, v1: Adt6, v2: Adt6) -> Result<Adt16, Adt127> {
    let m2204 = f231(v1);
    match m2204 {
        None => {
            Err::<Adt16, Adt127>(Adt127::C5)
        }
        Some(v3) => {
            let m2205 = f601(v2);
            match m2205 {
                None => {
                    Err::<Adt16, Adt127>(Adt127::C6)
                }
                Some(v4) => {
                    let m2206 = f556(f602(), v0.clone());
                    match m2206 {
                        Ok(v6) => {
                            let m2207 = f603(v0.clone());
                            match m2207 {
                                Ok(v8) => {
                                    let m2208 = f604(v0.clone());
                                    match m2208 {
                                        Ok(v10) => {
                                            Ok::<Adt16, Adt127>(Adt16::C0(v3, v4, v6, v8, v10, v0))
                                        }
                                        Err(v9) => {
                                            Err::<Adt16, Adt127>(v9)
                                        }
                                    }
                                }
                                Err(v7) => {
                                    Err::<Adt16, Adt127>(v7)
                                }
                            }
                        }
                        Err(v5) => {
                            Err::<Adt16, Adt127>(v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f568(p2212: List<Adt6>) -> bool {
    let m2211 = p2212;
    {
        let mut m2211 = m2211;
        loop {
            let step = {
                let v0 = m2211;
                let m2209 = v0;
                match m2209.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2210 = f231(v1);
                        match m2210 {
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
                Ok(next) => { m2211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(p2216: List<Adt6>) -> bool {
    let m2215 = p2216;
    {
        let mut m2215 = m2215;
        loop {
            let step = {
                let v0 = m2215;
                let m2213 = v0;
                match m2213.uncons() {
                    None => {
                        Err::<List<Adt6>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        let m2214 = f231(v1);
                        match m2214 {
                            None => {
                                Err::<List<Adt6>, bool>(false)
                            }
                            Some(v3) => {
                                if f387(v3) {
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
                Ok(next) => { m2215 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Str) -> Option<Adt13> {
    let a2217 = v0.clone();
    let a2218 = Str::lit("string");
    if equal(a2217, a2218) {
        Some(Adt13::C0)
    } else {
        let a2219 = v0.clone();
        let a2220 = Str::lit("integer");
        if equal(a2219, a2220) {
            Some(Adt13::C1)
        } else {
            let a2221 = v0;
            let a2222 = Str::lit("boolean");
            if equal(a2221, a2222) {
                Some(Adt13::C2)
            } else {
                None::<Adt13>
            }
        }
    }
}

pub fn f571(p2231: List<u64>, p2232: List<u64>) -> R<List<u64>> {
    let m2230 = (p2231, p2232);
    {
        let mut m2230 = m2230;
        loop {
            let step = {
                let (v0, v1) = m2230;
                let m2223 = v0;
                match m2223.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), R<List<u64>>>(Ok(v1))
                    }
                    Some((v2, v3)) => {
                        let a2224 = 65u64;
                        let a2225 = v2;
                        if if nat_le(a2224, a2225) {
                            let a2226 = v2;
                            let a2227 = 90u64;
                            nat_le(a2226, a2227)
                        } else {
                            false
                        } {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons({
                                let a2228 = v2;
                                let a2229 = 32u64;
                                nat_add(a2228, a2229)?
                            }, v1)))
                        } else {
                            Ok::<(List<u64>, List<u64>), R<List<u64>>>((v3, List::cons(v2, v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2230 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(p2239: List<u64>, p2240: List<u64>) -> bool {
    let m2238 = (p2239, p2240);
    {
        let mut m2238 = m2238;
        loop {
            let step = {
                let (v0, v1) = m2238;
                let m2233 = v0;
                match m2233.uncons() {
                    None => {
                        let m2234 = v1;
                        match m2234.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(true)
                            }
                            Some((_, _)) => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                        }
                    }
                    Some((v4, v5)) => {
                        let m2235 = v1;
                        match m2235.uncons() {
                            None => {
                                Err::<(List<u64>, List<u64>), bool>(false)
                            }
                            Some((v6, v7)) => {
                                let a2236 = v4;
                                let a2237 = v6;
                                if nat_eq(a2236, a2237) {
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
                Ok(next) => { m2238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f573(v0: Adt6) -> u64 {
    let m2241 = v0;
    match m2241 {
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

pub fn f574(v0: Adt147) -> bool {
    let m2242 = v0;
    match m2242 {
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

pub fn f575(v0: Option<List<(Adt6, Adt6)>>, v1: List<(Adt6, Adt6)>) -> Adt149 {
    let m2243 = v0;
    match m2243 {
        None => {
            Adt149::C0
        }
        Some(v2) => {
            Adt149::C1({
                let a2244 = v2;
                let a2245 = v1;
                append_list(a2244, a2245)
            })
        }
    }
}

pub fn f576(p2250: List<Adt6>, p2251: List<Adt6>, p2252: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2249 = (p2250, (p2251, p2252));
    {
        let mut m2249 = m2249;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2249;
                let m2246 = v0;
                match m2246.uncons() {
                    None => {
                        let m2247 = v1;
                        match m2247.uncons() {
                            None => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(Some(v2))
                            }
                            Some((_, _)) => {
                                Err::<(List<Adt6>, (List<Adt6>, List<(Adt6, Adt6)>)), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                        }
                    }
                    Some((v5, v6)) => {
                        let m2248 = v1;
                        match m2248.uncons() {
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
                Ok(next) => { m2249 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(p2258: List<(Str, Adt6)>, v1: List<(Str, Adt6)>, p2259: List<(Adt6, Adt6)>) -> Option<List<(Adt6, Adt6)>> {
    let m2257 = (p2258, p2259);
    {
        let mut m2257 = m2257;
        loop {
            let step = {
                let (v0, v2) = m2257;
                let m2253 = v0;
                match m2253.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(Some(v2))
                    }
                    Some((v3, v4)) => {
                        let m2255 = f234({
                            let (h2254, _) = v3;
                            h2254
                        }, v1.clone());
                        match m2255 {
                            Adt130::C0 => {
                                Err::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>(None::<List<(Adt6, Adt6)>>)
                            }
                            Adt130::C1(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Adt6, Adt6)>), Option<List<(Adt6, Adt6)>>>((v4, List::cons(({
                                    let (_, h2256) = v3;
                                    h2256
                                }, v5), v2)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2257 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: Adt147) -> Adt147 {
    let m2260 = v0;
    match m2260 {
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

pub fn f579(v0: i64, v1: i64) -> R<i64> {
    f115(f79(f581(v0)?, 0i64)?, v1)
}

pub fn f580(v0: i64, v1: u64, v2: i64, v3: u64) -> R<Adt147> {
    let a2261 = v1;
    let a2262 = v3;
    if nat_lt(a2261, a2262) {
        Ok(f469(f605({
            let a2263 = v3;
            let a2264 = v1;
            nat_sub(a2263, a2264)
        }, v0)?, v2))
    } else {
        let a2265 = v3;
        let a2266 = v1;
        if nat_lt(a2265, a2266) {
            Ok(f469(v0, f605({
                let a2267 = v1;
                let a2268 = v3;
                nat_sub(a2267, a2268)
            }, v2)?))
        } else {
            Ok(f469(v0, v2))
        }
    }
}

pub fn f581(v0: i64) -> R<u64> {
    f606(20u64, v0, 0u64)
}

pub fn f582(v0: Option<Adt36>) -> List<(Str, Adt6)> {
    let m2269 = v0;
    match m2269 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2281 = f417(Str::lit("title"), match v1.clone() {
                    Adt36::C0(h2270, _, _, _, _) => {
                        h2270
                    }
                });
                let a2282 = {
                    let a2279 = f422(Str::lit("readOnlyHint"), match v1.clone() {
                        Adt36::C0(_, h2271, _, _, _) => {
                            h2271
                        }
                    });
                    let a2280 = {
                        let a2277 = f422(Str::lit("destructiveHint"), match v1.clone() {
                            Adt36::C0(_, _, h2272, _, _) => {
                                h2272
                            }
                        });
                        let a2278 = {
                            let a2275 = f422(Str::lit("idempotentHint"), match v1.clone() {
                                Adt36::C0(_, _, _, h2273, _) => {
                                    h2273
                                }
                            });
                            let a2276 = f422(Str::lit("openWorldHint"), match v1 {
                                Adt36::C0(_, _, _, _, h2274) => {
                                    h2274
                                }
                            });
                            append_list(a2275, a2276)
                        };
                        append_list(a2277, a2278)
                    };
                    append_list(a2279, a2280)
                };
                append_list(a2281, a2282)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f583(v0: Option<Adt5>) -> List<(Str, Adt6)> {
    f437(Str::lit("_meta"), v0)
}

pub fn f584(v0: Adt39) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("text"))), List::cons((Str::lit("text"), Adt6::C3(match v0.clone() {
        Adt39::C0(h2283, _, _) => {
            h2283
        }
    })), {
        let a2286 = f607(match v0.clone() {
            Adt39::C0(_, h2284, _) => {
                h2284
            }
        });
        let a2287 = f583(match v0 {
            Adt39::C0(_, _, h2285) => {
                h2285
            }
        });
        append_list(a2286, a2287)
    })))
}

pub fn f585(v0: Str, v1: Str, v2: Str, v3: Option<Adt40>, v4: Option<Adt5>) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(v0)), List::cons((Str::lit("data"), Adt6::C3(v1)), List::cons((Str::lit("mimeType"), Adt6::C3(v2)), {
        let a2288 = f607(v3);
        let a2289 = f583(v4);
        append_list(a2288, a2289)
    }))))
}

pub fn f586(v0: Adt44) -> R<Adt6> {
    Ok(Adt6::C5({
        let a2313 = List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource_link"))), List::cons((Str::lit("name"), Adt6::C3(match v0.clone() {
            Adt44::C0(h2290, _, _, _, _, _, _, _, _) => {
                h2290
            }
        })), List::<(Str, Adt6)>::nil()));
        let a2314 = {
            let a2311 = f417(Str::lit("title"), match v0.clone() {
                Adt44::C0(_, h2291, _, _, _, _, _, _, _) => {
                    h2291
                }
            });
            let a2312 = {
                let a2309 = f486(match v0.clone() {
                    Adt44::C0(_, _, h2292, _, _, _, _, _, _) => {
                        h2292
                    }
                });
                let a2310 = {
                    let a2307 = List::cons((Str::lit("uri"), Adt6::C3(match v0.clone() {
                        Adt44::C0(_, _, _, h2293, _, _, _, _, _) => {
                            h2293
                        }
                    })), List::<(Str, Adt6)>::nil());
                    let a2308 = {
                        let a2305 = f417(Str::lit("description"), match v0.clone() {
                            Adt44::C0(_, _, _, _, h2294, _, _, _, _) => {
                                h2294
                            }
                        });
                        let a2306 = {
                            let a2303 = f417(Str::lit("mimeType"), match v0.clone() {
                                Adt44::C0(_, _, _, _, _, h2295, _, _, _) => {
                                    h2295
                                }
                            });
                            let a2304 = {
                                let a2301 = f607(match v0.clone() {
                                    Adt44::C0(_, _, _, _, _, _, h2296, _, _) => {
                                        h2296
                                    }
                                });
                                let a2302 = {
                                    let a2299 = f608(match v0.clone() {
                                        Adt44::C0(_, _, _, _, _, _, _, h2297, _) => {
                                            h2297
                                        }
                                    })?;
                                    let a2300 = f583(match v0 {
                                        Adt44::C0(_, _, _, _, _, _, _, _, h2298) => {
                                            h2298
                                        }
                                    });
                                    append_list(a2299, a2300)
                                };
                                append_list(a2301, a2302)
                            };
                            append_list(a2303, a2304)
                        };
                        append_list(a2305, a2306)
                    };
                    append_list(a2307, a2308)
                };
                append_list(a2309, a2310)
            };
            append_list(a2311, a2312)
        };
        append_list(a2313, a2314)
    }))
}

pub fn f587(v0: Adt45) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("type"), Adt6::C3(Str::lit("resource"))), List::cons((Str::lit("resource"), f609(match v0.clone() {
        Adt45::C0(h2315, _, _) => {
            h2315
        }
    })), {
        let a2318 = f607(match v0.clone() {
            Adt45::C0(_, h2316, _) => {
                h2316
            }
        });
        let a2319 = f583(match v0 {
            Adt45::C0(_, _, h2317) => {
                h2317
            }
        });
        append_list(a2318, a2319)
    })))
}

pub fn f588(p2322: List<Adt2>, p2323: List<Adt6>) -> List<Adt6> {
    let m2321 = (p2322, p2323);
    {
        let mut m2321 = m2321;
        loop {
            let step = {
                let (v0, v1) = m2321;
                let m2320 = v0;
                match m2320.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt2>, List<Adt6>), List<Adt6>>((v3, List::cons(f610(v2), v1)))
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

pub fn f589() -> List<u64> {
    List::cons(100u64, List::cons(97u64, List::cons(116u64, List::cons(97u64, List::cons(58u64, List::cons(32u64, List::<u64>::nil()))))))
}

pub fn f590() -> List<u64> {
    List::cons(10u64, List::cons(10u64, List::<u64>::nil()))
}

pub fn f591(p2328: List<u64>, p2329: u64, p2330: List<u64>) -> List<u64> {
    let m2327 = (p2328, (p2329, p2330));
    {
        let mut m2327 = m2327;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2327;
                let m2324 = v0;
                match m2324.uncons() {
                    None => {
                        Err::<(List<u64>, (u64, List<u64>)), List<u64>>(f611(v1, v2))
                    }
                    Some((v3, v4)) => {
                        let v5: Adt150 = f612(v1, v3, v2);
                        Ok::<(List<u64>, (u64, List<u64>)), List<u64>>((v4, (match v5.clone() {
                            Adt150::C0(h2325, _) => {
                                h2325
                            }
                        }, match v5 {
                            Adt150::C0(_, h2326) => {
                                h2326
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f592(v0: Adt134, v1: Adt32) -> bool {
    let m2331 = v1;
    match m2331 {
        Adt32::C0(_) => {
            f613(v0, Adt134::C0)
        }
        Adt32::C1(_) => {
            f613(v0, Adt134::C1)
        }
        Adt32::C2(_) => {
            f613(v0, Adt134::C2)
        }
        Adt32::C3(_) => {
            f613(v0, Adt134::C2)
        }
        Adt32::C4(_) => {
            f613(v0, Adt134::C3)
        }
        Adt32::C5(_) => {
            f613(v0, Adt134::C4)
        }
        Adt32::C6(_) => {
            f613(v0, Adt134::C5)
        }
        Adt32::C7(_) => {
            f613(v0, Adt134::C6)
        }
        Adt32::C8(_) => {
            f613(v0, Adt134::C7)
        }
        Adt32::C9(_) => {
            f613(v0, Adt134::C8)
        }
        Adt32::C10(_) => {
            f613(v0, Adt134::C9)
        }
        Adt32::C11 => {
            if f613(v0.clone(), Adt134::C10) {
                true
            } else {
                f613(v0, Adt134::C11)
            }
        }
        Adt32::C12(_) => {
            f614(v0)
        }
        Adt32::C13(_) => {
            f615(v0)
        }
    }
}

pub fn f593(v0: Adt0, v1: Adt14, v2: Adt32) -> Result<Adt123, Adt126> {
    let m2332 = v2.clone();
    match m2332 {
        Adt32::C0(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C1(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C2(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C3(v8) => {
            Ok::<Adt123, Adt126>(f619(v0, f617(v1), v8))
        }
        Adt32::C4(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C5(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C6(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C7(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C8(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C9(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C10(_) => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C11 => {
            Ok::<Adt123, Adt126>(f550(v0, v2))
        }
        Adt32::C12(v3) => {
            f616(v0, f617(v1), v3)
        }
        Adt32::C13(v4) => {
            f618(v0, f617(v1), v4)
        }
    }
}

pub fn f594() -> Adt84 {
    Adt84::C9(Str::lit("the provider answered with a result that does not belong to the method"))
}

pub fn f595(v0: u64, v1: u64) -> bool {
    let a2333 = v1;
    let a2334 = v0;
    nat_lt(a2333, a2334)
}

pub fn f596(v0: Str, v1: List<(Str, Adt6)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2335 = f620(v0, v1, List::<(Str, Str)>::nil());
    match m2335 {
        Ok(v2) => {
            Ok::<List<(Str, Str)>, Adt127>(f621(v2, List::<(Str, Str)>::nil()))
        }
        Err(v3) => {
            Err::<List<(Str, Str)>, Adt127>(v3)
        }
    }
}

pub fn f597(p2339: List<Adt110>, v1: Adt5) -> R<Option<Adt108>> {
    let m2338 = p2339;
    {
        let mut m2338 = m2338;
        loop {
            let step = {
                let v0 = m2338;
                let m2336 = v0;
                match m2336.uncons() {
                    None => {
                        Err::<List<Adt110>, R<Option<Adt108>>>(Ok(None::<Adt108>))
                    }
                    Some((v2, v3)) => {
                        let m2337 = f622(v2, v1.clone())?;
                        match m2337 {
                            None => {
                                Ok::<List<Adt110>, R<Option<Adt108>>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt110>, R<Option<Adt108>>>(Ok(Some(v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f598(v0: Adt5) -> Adt5 {
    let m2340 = f176(Str::lit("arguments"), v0);
    match m2340 {
        Adt130::C0 => {
            f511()
        }
        Adt130::C1(v1) => {
            let m2341 = f195(v1);
            match m2341 {
                None => {
                    f511()
                }
                Some(v2) => {
                    v2
                }
            }
        }
    }
}

pub fn f599(v0: Option<Str>, v1: Option<Str>) -> Option<Adt108> {
    let m2342 = v0;
    match m2342 {
        None => {
            Some(Adt108::C2)
        }
        Some(v2) => {
            let m2343 = v1;
            match m2343 {
                None => {
                    None::<Adt108>
                }
                Some(v3) => {
                    let a2344 = v2;
                    let a2345 = v3;
                    if equal(a2344, a2345) {
                        None::<Adt108>
                    } else {
                        Some(Adt108::C5)
                    }
                }
            }
        }
    }
}

pub fn f600(v0: Adt140, v1: Adt5) -> Option<Str> {
    let m2346 = v0;
    match m2346 {
        Adt140::C0 => {
            None::<Str>
        }
        Adt140::C1 => {
            let m2347 = f176(Str::lit("name"), v1);
            match m2347 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v2) => {
                    f231(v2)
                }
            }
        }
        Adt140::C2 => {
            let m2348 = f176(Str::lit("uri"), v1);
            match m2348 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v3) => {
                    f231(v3)
                }
            }
        }
        Adt140::C3 => {
            let m2349 = f176(Str::lit("taskId"), v1);
            match m2349 {
                Adt130::C0 => {
                    None::<Str>
                }
                Adt130::C1(v4) => {
                    f231(v4)
                }
            }
        }
    }
}

pub fn f601(v0: Adt6) -> Option<Adt17> {
    let m2350 = f195(v0);
    match m2350 {
        None => {
            None::<Adt17>
        }
        Some(v1) => {
            let m2351 = f623(Str::lit("experimental"), v1.clone());
            match m2351 {
                None => {
                    None::<Adt17>
                }
                Some(v2) => {
                    let m2352 = f623(Str::lit("roots"), v1.clone());
                    match m2352 {
                        None => {
                            None::<Adt17>
                        }
                        Some(v3) => {
                            let m2353 = f623(Str::lit("sampling"), v1.clone());
                            match m2353 {
                                None => {
                                    None::<Adt17>
                                }
                                Some(v4) => {
                                    let m2354 = f623(Str::lit("elicitation"), v1.clone());
                                    match m2354 {
                                        None => {
                                            None::<Adt17>
                                        }
                                        Some(v5) => {
                                            let m2355 = f623(Str::lit("extensions"), v1);
                                            match m2355 {
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

pub fn f602() -> Str {
    Str::lit("io.modelcontextprotocol/clientInfo")
}

pub fn f603(v0: Adt5) -> Result<Option<Adt18>, Adt127> {
    let m2356 = f554(f624(), v0);
    match m2356 {
        Ok(v2) => {
            let m2357 = v2;
            match m2357 {
                None => {
                    Ok::<Option<Adt18>, Adt127>(None::<Adt18>)
                }
                Some(v3) => {
                    let m2358 = f625(v3);
                    match m2358 {
                        Adt151::C0 => {
                            Err::<Option<Adt18>, Adt127>(Adt127::C8(f624()))
                        }
                        Adt151::C1(v4) => {
                            Ok::<Option<Adt18>, Adt127>(Some(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Err::<Option<Adt18>, Adt127>(v1)
        }
    }
}

pub fn f604(v0: Adt5) -> Result<Option<Adt19>, Adt127> {
    let m2359 = f176(f626(), v0);
    match m2359 {
        Adt130::C0 => {
            Ok::<Option<Adt19>, Adt127>(None::<Adt19>)
        }
        Adt130::C1(v1) => {
            let m2360 = f231(v1.clone());
            match m2360 {
                None => {
                    let m2361 = f468(v1);
                    match m2361 {
                        None => {
                            Err::<Option<Adt19>, Adt127>(Adt127::C8(f626()))
                        }
                        Some(v3) => {
                            Ok::<Option<Adt19>, Adt127>(Some(Adt19::C1(v3)))
                        }
                    }
                }
                Some(v2) => {
                    Ok::<Option<Adt19>, Adt127>(Some(Adt19::C0(v2)))
                }
            }
        }
    }
}

pub fn f605(p2366: u64, p2367: i64) -> R<i64> {
    let m2365 = (p2366, p2367);
    {
        let mut m2365 = m2365;
        loop {
            let step = {
                let (v0, v1) = m2365;
                let m2362 = v0;
                if m2362 == 0 {
                    Err::<(u64, i64), R<i64>>(Ok(v1))
                } else {
                    let v2 = m2362 - 1;
                    Ok::<(u64, i64), R<i64>>((v2, {
                        let a2363 = v1;
                        let a2364 = 10i64;
                        int_mul(a2363, a2364)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2365 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f606(p2378: u64, p2379: i64, p2380: u64) -> R<u64> {
    let m2377 = (p2378, (p2379, p2380));
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2377;
                let m2368 = v0;
                if m2368 == 0 {
                    Err::<(u64, (i64, u64)), R<u64>>(Ok(v2))
                } else {
                    let v3 = m2368 - 1;
                    let v4: u64 = {
                        let a2369 = v2;
                        let a2370 = 1u64;
                        nat_add(a2369, a2370)?
                    };
                    let m2373 = {
                        let a2371 = v1;
                        let a2372 = 10i64;
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
                        Err::<(u64, (i64, u64)), R<u64>>(Ok(v4))
                    } else {
                        Ok::<(u64, (i64, u64)), R<u64>>((v3, ({
                            let a2374 = v1;
                            let a2375 = 10i64;
                            let a2376 = 0i64;
                            int_quot(a2374, a2375, a2376)?
                        }, v4)))
                    }
                }
            };
            match step {
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f607(v0: Option<Adt40>) -> List<(Str, Adt6)> {
    let m2381 = v0;
    match m2381 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("annotations"), Adt6::C5({
                let a2387 = f627(match v1.clone() {
                    Adt40::C0(h2382, _, _) => {
                        h2382
                    }
                });
                let a2388 = {
                    let a2385 = f628(match v1.clone() {
                        Adt40::C0(_, h2383, _) => {
                            h2383
                        }
                    });
                    let a2386 = f417(Str::lit("lastModified"), match v1 {
                        Adt40::C0(_, _, h2384) => {
                            h2384
                        }
                    });
                    append_list(a2385, a2386)
                };
                append_list(a2387, a2388)
            })), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f608(v0: Option<u64>) -> R<List<(Str, Adt6)>> {
    let m2389 = v0;
    match m2389 {
        None => {
            Ok(List::<(Str, Adt6)>::nil())
        }
        Some(v1) => {
            Ok(List::cons((Str::lit("size"), f482(v1)?), List::<(Str, Adt6)>::nil()))
        }
    }
}

pub fn f609(v0: Adt46) -> Adt6 {
    let m2390 = v0;
    match m2390 {
        Adt46::C0(v1) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v1.clone() {
                Adt47::C0(h2391, _, _, _) => {
                    h2391
                }
            })), {
                let a2395 = f417(Str::lit("mimeType"), match v1.clone() {
                    Adt47::C0(_, h2392, _, _) => {
                        h2392
                    }
                });
                let a2396 = List::cons((Str::lit("text"), Adt6::C3(match v1.clone() {
                    Adt47::C0(_, _, h2393, _) => {
                        h2393
                    }
                })), f583(match v1 {
                    Adt47::C0(_, _, _, h2394) => {
                        h2394
                    }
                }));
                append_list(a2395, a2396)
            }))
        }
        Adt46::C1(v2) => {
            Adt6::C5(List::cons((Str::lit("uri"), Adt6::C3(match v2.clone() {
                Adt48::C0(h2397, _, _, _) => {
                    h2397
                }
            })), {
                let a2401 = f417(Str::lit("mimeType"), match v2.clone() {
                    Adt48::C0(_, h2398, _, _) => {
                        h2398
                    }
                });
                let a2402 = List::cons((Str::lit("blob"), Adt6::C3(match v2.clone() {
                    Adt48::C0(_, _, h2399, _) => {
                        h2399
                    }
                })), f583(match v2 {
                    Adt48::C0(_, _, _, h2400) => {
                        h2400
                    }
                }));
                append_list(a2401, a2402)
            }))
        }
    }
}

pub fn f610(v0: Adt2) -> Adt6 {
    Adt6::C5(List::cons((Str::lit("src"), Adt6::C3(match v0.clone() {
        Adt2::C0(h2403, _, _, _) => {
            h2403
        }
    })), {
        let a2409 = f417(Str::lit("mimeType"), match v0.clone() {
            Adt2::C0(_, h2404, _, _) => {
                h2404
            }
        });
        let a2410 = {
            let a2407 = f629(match v0.clone() {
                Adt2::C0(_, _, h2405, _) => {
                    h2405
                }
            });
            let a2408 = f630(match v0 {
                Adt2::C0(_, _, _, h2406) => {
                    h2406
                }
            });
            append_list(a2407, a2408)
        };
        append_list(a2409, a2410)
    }))
}

pub fn f611(v0: u64, v1: List<u64>) -> List<u64> {
    let a2411 = v0;
    let a2412 = 0u64;
    if nat_eq(a2411, a2412) {
        v1
    } else {
        let a2413 = v0;
        let a2414 = 1u64;
        if nat_eq(a2413, a2414) {
            List::cons(194u64, v1)
        } else {
            let a2415 = v0;
            let a2416 = 2u64;
            if nat_eq(a2415, a2416) {
                List::cons(226u64, v1)
            } else {
                let a2417 = v0;
                let a2418 = 3u64;
                if nat_eq(a2417, a2418) {
                    List::cons(128u64, List::cons(226u64, v1))
                } else {
                    List::cons(129u64, List::cons(226u64, v1))
                }
            }
        }
    }
}

pub fn f612(v0: u64, v1: u64, v2: List<u64>) -> Adt150 {
    let a2419 = v0;
    let a2420 = 0u64;
    if nat_eq(a2419, a2420) {
        f631(v1, v2)
    } else {
        let a2421 = v0;
        let a2422 = 1u64;
        if nat_eq(a2421, a2422) {
            let a2423 = 128u64;
            let a2424 = v1;
            if if nat_le(a2423, a2424) {
                let a2425 = v1;
                let a2426 = 159u64;
                nat_le(a2425, a2426)
            } else {
                false
            } {
                Adt150::C0(0u64, f632(v2))
            } else {
                f631(v1, List::cons(194u64, v2))
            }
        } else {
            let a2427 = v0;
            let a2428 = 2u64;
            if nat_eq(a2427, a2428) {
                let a2429 = v1;
                let a2430 = 128u64;
                if nat_eq(a2429, a2430) {
                    Adt150::C0(3u64, v2)
                } else {
                    let a2431 = v1;
                    let a2432 = 129u64;
                    if nat_eq(a2431, a2432) {
                        Adt150::C0(4u64, v2)
                    } else {
                        f631(v1, List::cons(226u64, v2))
                    }
                }
            } else {
                let a2433 = v0;
                let a2434 = 3u64;
                if nat_eq(a2433, a2434) {
                    let a2435 = 170u64;
                    let a2436 = v1;
                    if if nat_le(a2435, a2436) {
                        let a2437 = v1;
                        let a2438 = 174u64;
                        nat_le(a2437, a2438)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f632(v2))
                    } else {
                        f631(v1, List::cons(128u64, List::cons(226u64, v2)))
                    }
                } else {
                    let a2439 = 166u64;
                    let a2440 = v1;
                    if if nat_le(a2439, a2440) {
                        let a2441 = v1;
                        let a2442 = 169u64;
                        nat_le(a2441, a2442)
                    } else {
                        false
                    } {
                        Adt150::C0(0u64, f632(v2))
                    } else {
                        f631(v1, List::cons(129u64, List::cons(226u64, v2)))
                    }
                }
            }
        }
    }
}

pub fn f613(v0: Adt134, v1: Adt134) -> bool {
    let a2443 = f441(v0);
    let a2444 = f441(v1);
    equal(a2443, a2444)
}

pub fn f614(v0: Adt134) -> bool {
    let m2445 = v0;
    match m2445 {
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

pub fn f615(v0: Adt134) -> bool {
    let m2446 = v0;
    match m2446 {
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

pub fn f616(v0: Adt0, v1: Adt16, v2: Adt81) -> Result<Adt123, Adt126> {
    f633(v0, v2.clone(), f634(match v1 {
        Adt16::C0(_, h2447, _, _, _, _) => {
            h2447
        }
    }, f635(v2)))
}

pub fn f617(v0: Adt14) -> Adt16 {
    let m2448 = v0;
    match m2448 {
        Adt14::C0(v1) => {
            match v1 {
                Adt15::C0(h2449) => {
                    h2449
                }
            }
        }
        Adt14::C1(v2) => {
            match v2 {
                Adt20::C0(h2450, _) => {
                    h2450
                }
            }
        }
        Adt14::C2(v3) => {
            match v3 {
                Adt21::C0(h2451, _, _, _, _) => {
                    h2451
                }
            }
        }
        Adt14::C3(v4) => {
            match v4 {
                Adt20::C0(h2452, _) => {
                    h2452
                }
            }
        }
        Adt14::C4(v5) => {
            match v5 {
                Adt22::C0(h2453, _, _, _) => {
                    h2453
                }
            }
        }
        Adt14::C5(v6) => {
            match v6 {
                Adt20::C0(h2454, _) => {
                    h2454
                }
            }
        }
        Adt14::C6(v7) => {
            match v7 {
                Adt20::C0(h2455, _) => {
                    h2455
                }
            }
        }
        Adt14::C7(v8) => {
            match v8 {
                Adt23::C0(h2456, _, _, _, _) => {
                    h2456
                }
            }
        }
        Adt14::C8(v9) => {
            match v9 {
                Adt24::C0(h2457, _, _, _) => {
                    h2457
                }
            }
        }
        Adt14::C9(v10) => {
            match v10 {
                Adt29::C0(h2458, _) => {
                    h2458
                }
            }
        }
        Adt14::C10(v11) => {
            match v11 {
                Adt30::C0(h2459, _, _) => {
                    h2459
                }
            }
        }
        Adt14::C11(v12) => {
            match v12 {
                Adt31::C0(h2460, _) => {
                    h2460
                }
            }
        }
    }
}

pub fn f618(v0: Adt0, v1: Adt16, v2: Adt82) -> Result<Adt123, Adt126> {
    f636(v0, v2, f637(match v1 {
        Adt16::C0(_, h2461, _, _, _, _) => {
            h2461
        }
    }))
}

pub fn f619(v0: Adt0, v1: Adt16, v2: Adt49) -> Adt123 {
    Adt123::C0(Adt32::C2(match v2.clone() {
        Adt49::C0(h2462, _) => {
            h2462
        }
    }), {
        let m2464 = f550(v0, Adt32::C11);
        match m2464 {
            Adt123::C0(_, h2463, _) => {
                h2463
            }
        }
    }, f638(v1, match v2 {
        Adt49::C0(_, h2465) => {
            h2465
        }
    }))
}

pub fn f620(v0: Str, p2471: List<(Str, Adt6)>, p2472: List<(Str, Str)>) -> Result<List<(Str, Str)>, Adt127> {
    let m2470 = (p2471, p2472);
    {
        let mut m2470 = m2470;
        loop {
            let step = {
                let (v1, v2) = m2470;
                let m2466 = v1;
                match m2466.uncons() {
                    None => {
                        Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Ok::<List<(Str, Str)>, Adt127>(v2))
                    }
                    Some((v3, v4)) => {
                        let m2468 = f231({
                            let (_, h2467) = v3;
                            h2467
                        });
                        match m2468 {
                            None => {
                                Err::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>(Err::<List<(Str, Str)>, Adt127>(Adt127::C8(v0.clone())))
                            }
                            Some(v5) => {
                                Ok::<(List<(Str, Adt6)>, List<(Str, Str)>), Result<List<(Str, Str)>, Adt127>>((v4, List::cons(({
                                    let (h2469, _) = v3;
                                    h2469
                                }, v5), v2)))
                            }
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

pub fn f621(p2475: List<(Str, Str)>, p2476: List<(Str, Str)>) -> List<(Str, Str)> {
    let m2474 = (p2475, p2476);
    {
        let mut m2474 = m2474;
        loop {
            let step = {
                let (v0, v1) = m2474;
                let m2473 = v0;
                match m2473.uncons() {
                    None => {
                        Err::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Str, Str)>, List<(Str, Str)>), List<(Str, Str)>>((v3, List::cons(v2, v1)))
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

pub fn f622(v0: Adt110, v1: Adt5) -> R<Option<Adt108>> {
    let m2478 = f639(match v0.clone() {
        Adt110::C0(_, h2477, _, _) => {
            h2477
        }
    }, v1);
    match m2478 {
        Adt130::C0 => {
            Ok(f640(v0))
        }
        Adt130::C1(v2) => {
            if f641(v2.clone()) {
                Ok(f640(v0))
            } else {
                f642(v0, v2)
            }
        }
    }
}

pub fn f623(v0: Str, v1: Adt5) -> Option<Option<Adt5>> {
    let m2479 = f176(v0, v1);
    match m2479 {
        Adt130::C0 => {
            Some(None::<Adt5>)
        }
        Adt130::C1(v2) => {
            let m2480 = f195(v2);
            match m2480 {
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

pub fn f624() -> Str {
    Str::lit("io.modelcontextprotocol/logLevel")
}

pub fn f625(v0: Str) -> Adt151 {
    let a2481 = v0.clone();
    let a2482 = Str::lit("debug");
    if equal(a2481, a2482) {
        Adt151::C1(Adt18::C0)
    } else {
        let a2483 = v0.clone();
        let a2484 = Str::lit("info");
        if equal(a2483, a2484) {
            Adt151::C1(Adt18::C1)
        } else {
            let a2485 = v0.clone();
            let a2486 = Str::lit("notice");
            if equal(a2485, a2486) {
                Adt151::C1(Adt18::C2)
            } else {
                let a2487 = v0.clone();
                let a2488 = Str::lit("warning");
                if equal(a2487, a2488) {
                    Adt151::C1(Adt18::C3)
                } else {
                    let a2489 = v0.clone();
                    let a2490 = Str::lit("error");
                    if equal(a2489, a2490) {
                        Adt151::C1(Adt18::C4)
                    } else {
                        let a2491 = v0.clone();
                        let a2492 = Str::lit("critical");
                        if equal(a2491, a2492) {
                            Adt151::C1(Adt18::C5)
                        } else {
                            let a2493 = v0.clone();
                            let a2494 = Str::lit("alert");
                            if equal(a2493, a2494) {
                                Adt151::C1(Adt18::C6)
                            } else {
                                let a2495 = v0;
                                let a2496 = Str::lit("emergency");
                                if equal(a2495, a2496) {
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

pub fn f626() -> Str {
    Str::lit("progressToken")
}

pub fn f627(v0: Option<List<Adt41>>) -> List<(Str, Adt6)> {
    let m2497 = v0;
    match m2497 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("audience"), Adt6::C4(f643(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f628(v0: Option<Adt7>) -> List<(Str, Adt6)> {
    let m2498 = v0;
    match m2498 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("priority"), Adt6::C2(v1)), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f629(v0: Option<List<Str>>) -> List<(Str, Adt6)> {
    let m2499 = v0;
    match m2499 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("sizes"), Adt6::C4(f359(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f630(v0: Option<Adt3>) -> List<(Str, Adt6)> {
    let m2500 = v0;
    match m2500 {
        None => {
            List::<(Str, Adt6)>::nil()
        }
        Some(v1) => {
            List::cons((Str::lit("theme"), Adt6::C3(f644(v1))), List::<(Str, Adt6)>::nil())
        }
    }
}

pub fn f631(v0: u64, v1: List<u64>) -> Adt150 {
    let a2501 = v0;
    let a2502 = 32u64;
    if nat_lt(a2501, a2502) {
        if f645(v0) {
            Adt150::C0(0u64, List::cons(v0, v1))
        } else {
            Adt150::C0(0u64, f632(v1))
        }
    } else {
        let a2503 = v0;
        let a2504 = 127u64;
        if nat_eq(a2503, a2504) {
            Adt150::C0(0u64, f632(v1))
        } else {
            let a2505 = v0;
            let a2506 = 194u64;
            if nat_eq(a2505, a2506) {
                Adt150::C0(1u64, v1)
            } else {
                let a2507 = v0;
                let a2508 = 226u64;
                if nat_eq(a2507, a2508) {
                    Adt150::C0(2u64, v1)
                } else {
                    Adt150::C0(0u64, List::cons(v0, v1))
                }
            }
        }
    }
}

pub fn f632(v0: List<u64>) -> List<u64> {
    List::cons(189u64, List::cons(191u64, List::cons(239u64, v0)))
}

pub fn f633(v0: Adt0, v1: Adt81, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f550(v0, Adt32::C12(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f646(f635(v1)))))
    }
}

pub fn f634(v0: Adt17, v1: List<(Str, Adt64)>) -> bool {
    let a2509 = f647(v1.clone());
    if if if bool_not(a2509) {
        true
    } else {
        f444(match v0.clone() {
            Adt17::C0(_, _, _, h2510, _) => {
                h2510
            }
        })
    } {
        let a2511 = f648(v1.clone());
        if bool_not(a2511) {
            true
        } else {
            f444(match v0.clone() {
                Adt17::C0(_, _, h2512, _, _) => {
                    h2512
                }
            })
        }
    } else {
        false
    } {
        let a2513 = f649(v1);
        if bool_not(a2513) {
            true
        } else {
            f444(match v0 {
                Adt17::C0(_, h2514, _, _, _) => {
                    h2514
                }
            })
        }
    } else {
        false
    }
}

pub fn f635(v0: Adt81) -> List<(Str, Adt64)> {
    let m2515 = v0;
    match m2515 {
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

pub fn f636(v0: Adt0, v1: Adt82, v2: bool) -> Result<Adt123, Adt126> {
    if v2 {
        Ok::<Adt123, Adt126>(f550(v0, Adt32::C13(v1)))
    } else {
        Err::<Adt123, Adt126>(Adt126::C4(Adt84::C8(f650())))
    }
}

pub fn f637(v0: Adt17) -> bool {
    f445(match v0 {
        Adt17::C0(_, _, _, _, h2516) => {
            h2516
        }
    }, f446())
}

pub fn f638(v0: Adt16, v1: List<Adt50>) -> List<Adt125> {
    let m2518 = match v0 {
        Adt16::C0(_, _, _, _, h2517, _) => {
            h2517
        }
    };
    match m2518 {
        None => {
            List::<Adt125>::nil()
        }
        Some(v2) => {
            f651(f652(v2, v1, List::<Adt125>::nil()), List::<Adt125>::nil())
        }
    }
}

pub fn f639(p2524: List<Str>, p2525: Adt5) -> Adt130 {
    let m2523 = (p2524, p2525);
    {
        let mut m2523 = m2523;
        loop {
            let step = {
                let (v0, v1) = m2523;
                let m2519 = v0;
                match m2519.uncons() {
                    None => {
                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                    }
                    Some((v2, v3)) => {
                        let m2520 = f176(v2, v1);
                        match m2520 {
                            Adt130::C0 => {
                                Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                            }
                            Adt130::C1(v4) => {
                                let m2521 = v3.clone();
                                match m2521.uncons() {
                                    None => {
                                        Err::<(List<Str>, Adt5), Adt130>(Adt130::C1(v4))
                                    }
                                    Some((_, _)) => {
                                        let m2522 = f195(v4);
                                        match m2522 {
                                            None => {
                                                Err::<(List<Str>, Adt5), Adt130>(Adt130::C0)
                                            }
                                            Some(v7) => {
                                                Ok::<(List<Str>, Adt5), Adt130>((v3, v7))
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
                Ok(next) => { m2523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: Adt110) -> Option<Adt108> {
    let m2527 = match v0.clone() {
        Adt110::C0(_, _, _, h2526) => {
            h2526
        }
    };
    match m2527 {
        Adt111::C0 => {
            None::<Adt108>
        }
        Adt111::C1 => {
            Some(Adt108::C10(match v0 {
                Adt110::C0(h2528, _, _, _) => {
                    h2528
                }
            }))
        }
        Adt111::C2(_) => {
            None::<Adt108>
        }
    }
}

pub fn f641(v0: Adt6) -> bool {
    let m2529 = v0;
    match m2529 {
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

pub fn f642(v0: Adt110, v1: Adt6) -> R<Option<Adt108>> {
    let m2531 = match v0.clone() {
        Adt110::C0(_, _, _, h2530) => {
            h2530
        }
    };
    match m2531 {
        Adt111::C0 => {
            Ok(Some(Adt108::C8(match v0 {
                Adt110::C0(h2532, _, _, _) => {
                    h2532
                }
            })))
        }
        Adt111::C1 => {
            Ok(Some(Adt108::C10(match v0 {
                Adt110::C0(h2533, _, _, _) => {
                    h2533
                }
            })))
        }
        Adt111::C2(v2) => {
            if f653(match v0.clone() {
                Adt110::C0(_, _, h2534, _) => {
                    h2534
                }
            }, v2, v1)? {
                Ok(None::<Adt108>)
            } else {
                Ok(Some(Adt108::C9(match v0 {
                    Adt110::C0(h2535, _, _, _) => {
                        h2535
                    }
                })))
            }
        }
    }
}

pub fn f643(v0: List<Adt41>) -> List<Adt6> {
    f294(f654(v0, List::<Adt6>::nil()), List::<Adt6>::nil())
}

pub fn f644(v0: Adt3) -> Str {
    let m2536 = v0;
    match m2536 {
        Adt3::C0 => {
            Str::lit("light")
        }
        Adt3::C1 => {
            Str::lit("dark")
        }
    }
}

pub fn f645(v0: u64) -> bool {
    let a2537 = v0;
    let a2538 = 9u64;
    if nat_eq(a2537, a2538) {
        true
    } else {
        let a2539 = v0;
        let a2540 = 10u64;
        if nat_eq(a2539, a2540) {
            true
        } else {
            let a2541 = v0;
            let a2542 = 13u64;
            nat_eq(a2541, a2542)
        }
    }
}

pub fn f646(v0: List<(Str, Adt64)>) -> Adt17 {
    Adt17::C0(None::<Adt5>, if f649(v0.clone()) {
        Some(f511())
    } else {
        None::<Adt5>
    }, if f648(v0.clone()) {
        Some(f511())
    } else {
        None::<Adt5>
    }, if f647(v0) {
        Some(f511())
    } else {
        None::<Adt5>
    }, None::<Adt5>)
}

pub fn f647(p2547: List<(Str, Adt64)>) -> bool {
    let m2546 = p2547;
    {
        let mut m2546 = m2546;
        loop {
            let step = {
                let v0 = m2546;
                let m2543 = v0;
                match m2543.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2545 = {
                            let (_, h2544) = v1;
                            h2544
                        };
                        match m2545 {
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
                Ok(next) => { m2546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f648(p2552: List<(Str, Adt64)>) -> bool {
    let m2551 = p2552;
    {
        let mut m2551 = m2551;
        loop {
            let step = {
                let v0 = m2551;
                let m2548 = v0;
                match m2548.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2550 = {
                            let (_, h2549) = v1;
                            h2549
                        };
                        match m2550 {
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
                Ok(next) => { m2551 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f649(p2557: List<(Str, Adt64)>) -> bool {
    let m2556 = p2557;
    {
        let mut m2556 = m2556;
        loop {
            let step = {
                let v0 = m2556;
                let m2553 = v0;
                match m2553.uncons() {
                    None => {
                        Err::<List<(Str, Adt64)>, bool>(false)
                    }
                    Some((v1, v2)) => {
                        let m2555 = {
                            let (_, h2554) = v1;
                            h2554
                        };
                        match m2555 {
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
                Ok(next) => { m2556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f650() -> Adt17 {
    Adt17::C0(None::<Adt5>, None::<Adt5>, None::<Adt5>, None::<Adt5>, Some(Adt5::C0(List::cons((f446(), Adt6::C5(List::<(Str, Adt6)>::nil())), List::<(Str, Adt6)>::nil()))))
}

pub fn f651(p2560: List<Adt125>, p2561: List<Adt125>) -> List<Adt125> {
    let m2559 = (p2560, p2561);
    {
        let mut m2559 = m2559;
        loop {
            let step = {
                let (v0, v1) = m2559;
                let m2558 = v0;
                match m2558.uncons() {
                    None => {
                        Err::<(List<Adt125>, List<Adt125>), List<Adt125>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt125>, List<Adt125>), List<Adt125>>((v3, List::cons(v2, v1)))
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

pub fn f652(v0: Adt19, p2564: List<Adt50>, p2565: List<Adt125>) -> List<Adt125> {
    let m2563 = (p2564, p2565);
    {
        let mut m2563 = m2563;
        loop {
            let step = {
                let (v1, v2) = m2563;
                let m2562 = v1;
                match m2562.uncons() {
                    None => {
                        Err::<(List<Adt50>, List<Adt125>), List<Adt125>>(v2)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Adt50>, List<Adt125>), List<Adt125>>((v4, List::cons(Adt125::C0(v0.clone(), v3), v2)))
                    }
                }
            };
            match step {
                Ok(next) => { m2563 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f653(v0: Adt13, v1: Str, v2: Adt6) -> R<bool> {
    let m2566 = v0;
    match m2566 {
        Adt13::C0 => {
            let m2567 = f231(v2);
            match m2567 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let a2568 = v1;
                    let a2569 = v3;
                    Ok(equal(a2568, a2569))
                }
            }
        }
        Adt13::C1 => {
            f655(v1, v2)
        }
        Adt13::C2 => {
            let m2570 = f194(v2);
            match m2570 {
                None => {
                    Ok(false)
                }
                Some(v4) => {
                    if v4 {
                        let a2571 = v1;
                        let a2572 = Str::lit("true");
                        Ok(equal(a2571, a2572))
                    } else {
                        let a2573 = v1;
                        let a2574 = Str::lit("false");
                        Ok(equal(a2573, a2574))
                    }
                }
            }
        }
    }
}

pub fn f654(p2577: List<Adt41>, p2578: List<Adt6>) -> List<Adt6> {
    let m2576 = (p2577, p2578);
    {
        let mut m2576 = m2576;
        loop {
            let step = {
                let (v0, v1) = m2576;
                let m2575 = v0;
                match m2575.uncons() {
                    None => {
                        Err::<(List<Adt41>, List<Adt6>), List<Adt6>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt41>, List<Adt6>), List<Adt6>>((v3, List::cons(Adt6::C3(f656(v2)), v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f655(v0: Str, v1: Adt6) -> R<bool> {
    let m2579 = f392(v1);
    match m2579 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2580 = f233(v2)?;
            match m2580 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    let m2582 = f657(f87({
                        let a2581 = v0;
                        utf8_encode(a2581)
                    })?)?;
                    match m2582 {
                        None => {
                            Ok(false)
                        }
                        Some(v4) => {
                            Ok(f374({
                                let a2583 = v3;
                                let a2584 = v4;
                                int_sub(a2583, a2584)?
                            }))
                        }
                    }
                }
            }
        }
    }
}

pub fn f656(v0: Adt41) -> Str {
    let m2585 = v0;
    match m2585 {
        Adt41::C0 => {
            Str::lit("user")
        }
        Adt41::C1 => {
            Str::lit("assistant")
        }
    }
}

pub fn f657(v0: List<u64>) -> R<Option<i64>> {
    let m2586 = v0.clone();
    match m2586.uncons() {
        None => {
            Ok(None::<i64>)
        }
        Some((v1, v2)) => {
            let a2587 = v1;
            let a2588 = 45u64;
            if nat_eq(a2587, a2588) {
                let m2589 = v2.clone();
                match m2589.uncons() {
                    None => {
                        Ok(None::<i64>)
                    }
                    Some((_, _)) => {
                        let m2590 = f658(v2, 0i64)?;
                        match m2590 {
                            None => {
                                Ok(None::<i64>)
                            }
                            Some(v5) => {
                                Ok(Some({
                                    let a2591 = v5;
                                    int_neg(a2591)?
                                }))
                            }
                        }
                    }
                }
            } else {
                f658(v0, 0i64)
            }
        }
    }
}

pub fn f658(p2597: List<u64>, p2598: i64) -> R<Option<i64>> {
    let m2596 = (p2597, p2598);
    {
        let mut m2596 = m2596;
        loop {
            let step = {
                let (v0, v1) = m2596;
                let m2592 = v0;
                match m2592.uncons() {
                    None => {
                        Err::<(List<u64>, i64), R<Option<i64>>>(Ok(Some(v1)))
                    }
                    Some((v2, v3)) => {
                        let m2593 = f659(v2);
                        match m2593 {
                            None => {
                                Err::<(List<u64>, i64), R<Option<i64>>>(Ok(None::<i64>))
                            }
                            Some(v4) => {
                                Ok::<(List<u64>, i64), R<Option<i64>>>((v3, f115({
                                    let a2594 = v1;
                                    let a2595 = 10i64;
                                    int_mul(a2594, a2595)?
                                }, f293(v4))?))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2596 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f659(v0: u64) -> Option<u64> {
    let a2599 = v0;
    let a2600 = 48u64;
    if if nat_lt(a2599, a2600) {
        true
    } else {
        let a2601 = 57u64;
        let a2602 = v0;
        nat_lt(a2601, a2602)
    } {
        None::<u64>
    } else {
        Some({
            let a2603 = v0;
            let a2604 = 48u64;
            nat_sub(a2603, a2604)
        })
    }
}

pub fn run(p0: Bytes) -> R<List<u64>> {
    f0(p0)
}
